//! The Featherweight Raven's raw downloads, `.FIPa`: the altimeter's flash memory as the
//! Featherweight Interface Program (FIP) saves it, before any export.
//!
//! The file opens with text lines, each starting `#`, up to a `#den` line; then the memory, in
//! frames of a `:`, 256 data bytes and a checksum byte, the sum of the 256 modulo 256; then
//! `#DONE` and a line end:
//!
//! ```text
//! #Component=Raven
//! #DateTime=4/15/2017 2:38:06 PM
//! #FlightNum=2
//! #FlightState=#2  O  88  1882  …      (tab-separated: the settings block's 32 words)
//! #BinaryFormat 1.0
//! #R 3.0                                (the firmware's version)
//! #Num 25
//! #den 20                               (a tick is Num/den ms: 1.25 ms)
//! :<256 bytes><checksum>:<256 bytes><checksum>…#DONE
//! ```
//!
//! **Blocks.** Each frame holds four blocks of 32 big-endian 16-bit words. Unused memory reads
//! `0xFFFF`; the flight ends at the first block of nothing else, and every block after it must be
//! unused too. Block 5 is the settings block, the words `#FlightState` lists after its first two
//! fields. Each other block covers 40 ticks, 0.05 s: blocks 0 to 4 are the 0.25 s before the Raven
//! detected the launch, and the blocks after the settings block the flight, one tick later in each
//! block's pattern. A block is ten triplets `(A, B, S)`, then two words, at ticks `4k` to `4k + 3`
//! within it (`4k + 1` to `4k + 4` after the settings block):
//!
//! | Bits | Sample | Tick |
//! |---|---|---|
//! | `A >> 5`, 11 bits | the axial accelerometer | `4k` |
//! | `(A & 0x1f) << 5 \| B >> 11`, 10 bits | the lateral accelerometer | `4k + 1` |
//! | `B & 0x7ff`, 11 bits | the axial accelerometer | `4k + 2` |
//! | `S` | the `k`th slow channel | `4k + 3` |
//!
//! so the axial accelerometer runs at 400 Hz, the lateral at 200 Hz and the slow channels at 20
//! Hz, the rates Featherweight's manual gives (*Raven User's Manual, updated for the Raven4*,
//! 2020-08-15). Time zero is the first sample before the launch, as in the FIP's export.
//!
//! **Values.** Each into the FIP's export's column, by the export's own arithmetic, then SI as
//! [`crate::raven`] reads that column:
//!
//! | Column | From the word | Raven's unit |
//! |---|---|---|
//! | `Axial Accel (Gs)` | `(trunc(X1 (a − Z1) / 1000) + F3) / 100` | g |
//! | `Lateral Accel (Gs)` | `trunc(X2 (l − Z2) / 1000) / 100` | g |
//! | `Current Draw (A)` | slow 0, signed, / 1000 | A |
//! | `Volts Pyro Apogee (V)`, `… Main`, `… 3rd`, `… 4th` | slow 1 to 4, signed, / 1000 | V |
//! | `Volts Battery (V)` | slow 6, signed, / 1000 | V |
//! | `Temperature (F)` | slow 7, signed, / 100 | °F |
//! | `Baro (Atm)` | slow 8, unsigned, / 25,000 | atm |
//! | `Velocity (Accel-Ft/Sec)` | slow 9, signed, / 10 | ft/s |
//!
//! `trunc` rounds toward zero, and `X1 / 1000` is worked in double precision, as the FIP does:
//! exact integers differ from its export by a hundredth of a g where the product is a whole number
//! that double precision leaves just under it, as on the 11 sample files whose gain is ±8,040.
//! `X1`, `Z1`, `F3`, `X2` and `Z2` are the settings block's: word 0 the
//! axial offset `F3`, words 23–24 the signed 32-bit axial gain `X1`, word 25 the axial zero `Z1`,
//! words 26–27 the lateral gain `X2`, word 28 the lateral zero `Z2`. A header whose `#Axis` is 1
//! (a single-axis accelerometer; 2 otherwise) keeps the lateral bits as written, as `lateral
//! counts`, since the Raven calibrates no lateral axis. Slow channel 5, which the FIP
//! doesn't export, and the two words after each block's triplets are kept as written, numbered by
//! their place, as [`Quantity::Unrecognized`]. The export stops `Velocity (Accel-Ft/Sec)` at its
//! first sample at or below zero after it has gone positive, its apogee; the record keeps every
//! sample the Raven wrote.
//!
//! **The record.** The primary track is empty, with a note: the Raven records a pressure, not an
//! altitude; the export's `[Altitude (Baro-Ft-AGL)]` and `[Altitude (Baro-Ft-ASL)]` are the FIP's
//! conversions, and working an altitude out of the pressure is a reading, not this reader's to make
//! (M7.2's). The settings block's word 1, the ground's elevation in feet, which the FIP's export
//! takes from the altitude above sea level to give it above the pad, is stated as the launch
//! site's elevation; `#FlightNum` is the flight's number and `#R` the firmware.
//!
//! **Checked against the FIP.** The layout and the arithmetic were worked out from raw files and the
//! FIP's exports of the same flights; wherever the project's sample logs hold both, `cargo xtask
//! logs` compares every sample of every channel the export holds with the raw file's.
//!
//! **Refused**, naming the byte: a header without its `#Num` and `#den` ticks, or a
//! `#BinaryFormat` other than 1.0; a frame whose checksum fails; anything after the frames but
//! `#DONE` and a line end; a settings block other than `#FlightState`'s words; an `#Axis` other
//! than 1 or 2; fewer than the five
//! blocks before the launch and the settings block; and a used block after an unused one.

use hpr_core::gravity::STANDARD_GRAVITY_MPS2;

use crate::error::LogError;
use crate::log::{Channel, FlightLog, LogFormat, Quantity, Stated};
use crate::perfectflite::FOOT_M;

/// The format's name in errors.
const FORMAT: &str = "Raven .FIPa";

/// How the file opens.
const OPENING: &[u8] = b"#Component=Raven";

/// The most bytes the text header may run to.
const MAX_HEADER: usize = 4096;

/// A frame: the `:`, 256 data bytes and the checksum byte.
const FRAME: usize = 258;

/// The words in a block.
const WORDS: usize = 32;

/// The settings block's place among the blocks.
const SETTINGS: usize = 5;

/// Ticks in a block.
const BLOCK_TICKS: u64 = 40;

/// The slow channels, by their place in a block's triplets: the FIP's export's names, or `None`
/// for the one it doesn't export.
const SLOW: [Option<&str>; 10] = [
    Some("Current Draw (A)"),
    Some("Volts Pyro Apogee (V)"),
    Some("Volts Pyro Main (V)"),
    Some("Volts Pyro 3rd (V)"),
    Some("Volts Pyro 4th (V)"),
    None,
    Some("Volts Battery (V)"),
    Some("Temperature (F)"),
    Some("Baro (Atm)"),
    Some("Velocity (Accel-Ft/Sec)"),
];

/// Pascals in a standard atmosphere, exactly (ISO 2533).
const ATM_PA: f64 = 101_325.0;

/// The Raven's pressure word's count in an atmosphere.
const BARO_COUNTS_PER_ATM: f64 = 25_000.0;

/// Whether the bytes are a `.FIPa`: they open `#Component=Raven`, and a line starting `#den`,
/// within the first [`MAX_HEADER`] bytes, is followed by a frame's `:`.
pub(crate) fn recognizes(bytes: &[u8]) -> bool {
    bytes.starts_with(OPENING) && header_end(bytes).is_some()
}

/// Where the text header ends: the byte after the `#den` line's end, which must be a frame's `:`.
fn header_end(bytes: &[u8]) -> Option<usize> {
    let head = &bytes[..bytes.len().min(MAX_HEADER)];
    let mut start = 0;
    while start < head.len() {
        let end = head[start..]
            .iter()
            .position(|&b| b == b'\n')
            .map(|at| start + at + 1)?;
        if head[start..].starts_with(b"#den") {
            return (bytes.get(end) == Some(&b':')).then_some(end);
        }
        if head[start] != b'#' {
            return None;
        }
        start = end;
    }
    None
}

/// The header's lines, `#Key=Value`, `#Key<tab>Value` or `#Key Value`, as key and value.
fn header_fields(header: &str) -> Vec<(&str, &str)> {
    header
        .lines()
        .filter_map(|line| {
            let line = line.trim_end_matches('\r').strip_prefix('#')?;
            let split = line.find(['=', '\t', ' '])?;
            Some((line[..split].trim(), line[split + 1..].trim()))
        })
        .collect()
}

/// A word read as a signed 16-bit integer.
fn signed(word: u16) -> i64 {
    i64::from(word as i16)
}

/// The settings block's calibration.
struct Calibration {
    /// The axial accelerometer's offset, hundredths of a g.
    f3: i64,
    /// The ground's elevation, ft above sea level.
    f4: i64,
    /// The axial accelerometer's gain, thousandths of a hundredth of a g a count.
    x1: i64,
    /// The axial accelerometer's zero, counts.
    z1: i64,
    /// The lateral accelerometer's gain.
    x2: i64,
    /// The lateral accelerometer's zero, counts.
    z2: i64,
}

impl Calibration {
    fn from_block(block: &[u16; WORDS]) -> Self {
        let pair = |hi: u16, lo: u16| i64::from(((u32::from(hi) << 16) | u32::from(lo)) as i32);
        Self {
            f3: signed(block[0]),
            f4: signed(block[1]),
            x1: pair(block[23], block[24]),
            z1: i64::from(block[25]),
            x2: pair(block[26], block[27]),
            z2: i64::from(block[28]),
        }
    }

    /// The axial acceleration, g, from an 11-bit count.
    fn axial_g(&self, count: u16) -> f64 {
        (scaled(self.x1, i64::from(count) - self.z1) + self.f3) as f64 / 100.0
    }

    /// The lateral acceleration, g, from a 10-bit count.
    fn lateral_g(&self, count: u16) -> f64 {
        scaled(self.x2, i64::from(count) - self.z2) as f64 / 100.0
    }
}

/// A count times a gain in thousandths, hundredths of a g: `trunc((gain / 1000) × counts)`, in
/// double precision, as the FIP's export works it. Exact integer arithmetic, `trunc(gain × counts /
/// 1000)`, differs from the export by a hundredth of a g where the product is a whole number that
/// rounding in double precision leaves just under it: on 11 of the 55 sample files with the
/// FIP's export beside them, those whose gain is ±8,040. Both operands are below 2³², so the
/// conversions are exact.
fn scaled(gain: i64, counts: i64) -> i64 {
    ((gain as f64 / 1000.0) * counts as f64).trunc() as i64
}

/// One channel as it fills: its sample ticks and values in SI.
struct Filling {
    column: String,
    quantity: Quantity,
    ticks: Vec<u64>,
    values: Vec<f64>,
}

impl Filling {
    fn new(column: &str, quantity: Quantity) -> Self {
        Self {
            column: column.to_owned(),
            quantity,
            ticks: Vec::new(),
            values: Vec::new(),
        }
    }

    fn push(&mut self, tick: u64, value: f64) {
        self.ticks.push(tick);
        self.values.push(value);
    }
}

/// Reads a Raven `.FIPa` file.
pub fn read(bytes: &[u8]) -> Result<FlightLog, LogError> {
    if !bytes.starts_with(OPENING) {
        return Err(LogError::NotThisFormat {
            format: FORMAT,
            message: "the file doesn't open `#Component=Raven`".to_owned(),
        });
    }
    let Some(body) = header_end(bytes) else {
        return Err(binary(
            0,
            format!(
                "the text header has no `#den` line followed by a frame within its first \
                 {MAX_HEADER} bytes"
            ),
        ));
    };
    let header = String::from_utf8_lossy(&bytes[..body]);
    let fields = header_fields(&header);
    let field = |key: &str| {
        fields
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(key))
            .map(|(_, v)| *v)
    };
    if let Some(version) = field("BinaryFormat")
        && version != "1.0"
    {
        return Err(binary(
            0,
            format!("the header's `#BinaryFormat` is {version:?}; this reader knows 1.0"),
        ));
    }
    let tick_part = |key: &str| {
        field(key)
            .and_then(|value| value.parse::<u32>().ok())
            .filter(|&value| value > 0)
            .ok_or_else(|| binary(0, format!("the header has no positive whole `#{key}`")))
    };
    let (num, den) = (tick_part("Num")?, tick_part("den")?);
    // A tick, s: `#Num` / `#den` milliseconds.
    let tick_s = f64::from(num) / f64::from(den) / 1000.0;

    // The frames, then `#DONE`.
    let mut blocks: Vec<[u16; WORDS]> = Vec::new();
    let mut at = body;
    while bytes.get(at) == Some(&b':') {
        let Some(frame) = bytes.get(at + 1..at + FRAME) else {
            return Err(binary(at, "the last frame is cut short".to_owned()));
        };
        let (data, checksum) = frame.split_at(256);
        let sum = data.iter().fold(0u8, |sum, &b| sum.wrapping_add(b));
        if sum != checksum[0] {
            return Err(binary(
                at,
                format!(
                    "the frame's checksum is {:#04x}, its bytes sum to {sum:#04x}",
                    checksum[0]
                ),
            ));
        }
        for block in data.chunks_exact(2 * WORDS) {
            let mut words = [0u16; WORDS];
            for (word, pair) in words.iter_mut().zip(block.chunks_exact(2)) {
                *word = u16::from_be_bytes([pair[0], pair[1]]);
            }
            blocks.push(words);
        }
        at += FRAME;
    }
    let trailer = &bytes[at..];
    let trailer_ok = trailer
        .strip_prefix(b"#DONE")
        .is_some_and(|rest| matches!(rest, b"" | b"\n" | b"\r\n"));
    if !trailer_ok {
        return Err(binary(
            at,
            "the frames are followed by something other than `#DONE` and a line end".to_owned(),
        ));
    }
    if blocks.len() <= SETTINGS {
        return Err(LogError::NoData { format: FORMAT });
    }

    let settings = blocks[SETTINGS];
    if let Some(state) = field("FlightState") {
        let listed: Vec<&str> = state.split('\t').skip(2).collect();
        let agrees = listed.len() <= WORDS
            && listed.iter().zip(settings).all(|(listed, word)| {
                listed
                    .trim()
                    .parse::<i64>()
                    .is_ok_and(|value| value as u16 == word)
            });
        if !agrees {
            return Err(binary(
                body + FRAME * (SETTINGS / 4) + 1 + 2 * WORDS * (SETTINGS % 4),
                "the settings block's words aren't the ones `#FlightState` lists".to_owned(),
            ));
        }
    }
    let calibration = Calibration::from_block(&settings);
    // `#Axis` is the number of the accelerometer's axes the Raven records: 2 in all but one of the
    // sample files, 1 in that one, whose lateral zero, 4 counts, is far from the two-axis files'
    // (near 510 counts, the middle of a 10-bit count).
    let two_axes = match field("Axis") {
        None | Some("2") => true,
        Some("1") => false,
        Some(other) => {
            return Err(binary(
                0,
                format!("the header's `#Axis` is {other:?}; this reader knows 1 and 2"),
            ));
        }
    };

    let mut axial = Filling::new("Axial Accel (Gs)", Quantity::Acceleration);
    let mut lateral = if two_axes {
        Filling::new("Lateral Accel (Gs)", Quantity::Acceleration)
    } else {
        Filling::new("lateral counts", Quantity::Unrecognized)
    };
    let mut slow: Vec<Filling> = SLOW
        .iter()
        .enumerate()
        .map(|(k, name)| match name {
            Some(name) => Filling::new(name, quantity_of(name)),
            None => Filling::new(&format!("slow channel {k}"), Quantity::Unrecognized),
        })
        .collect();
    let mut tails = [
        Filling::new("block word 30", Quantity::Unrecognized),
        Filling::new("block word 31", Quantity::Unrecognized),
    ];
    let mut data_blocks = 0u64;
    let mut ended = None;
    for (index, block) in blocks.iter().enumerate() {
        if index == SETTINGS {
            continue;
        }
        if block.iter().all(|&word| word == 0xffff) {
            ended = Some(index);
            break;
        }
        let shift = u64::from(index > SETTINGS);
        let first = data_blocks * BLOCK_TICKS;
        data_blocks += 1;
        for (k, triplet) in block[..30].chunks_exact(3).enumerate() {
            let (a, b, s) = (triplet[0], triplet[1], triplet[2]);
            let base = first + 4 * k as u64 + shift;
            axial.push(base, calibration.axial_g(a >> 5) * STANDARD_GRAVITY_MPS2);
            let count = ((a & 0x1f) << 5) | (b >> 11);
            lateral.push(
                base + 1,
                if two_axes {
                    calibration.lateral_g(count) * STANDARD_GRAVITY_MPS2
                } else {
                    f64::from(count)
                },
            );
            axial.push(
                base + 2,
                calibration.axial_g(b & 0x7ff) * STANDARD_GRAVITY_MPS2,
            );
            slow[k].push(base + 3, slow_si(k, s));
        }
        let last = first + BLOCK_TICKS - 1 + shift;
        tails[0].push(last, f64::from(block[30]));
        tails[1].push(last, f64::from(block[31]));
    }
    if let Some(stop) = ended
        && let Some(used) = (stop + 1..blocks.len())
            .filter(|&index| index != SETTINGS)
            .find(|&index| blocks[index].iter().any(|&word| word != 0xffff))
    {
        return Err(binary(
            body + FRAME * (used / 4),
            format!("block {used} holds data after block {stop}, which is unused"),
        ));
    }
    if data_blocks <= SETTINGS as u64 {
        return Err(LogError::NoData { format: FORMAT });
    }

    let mut notes = vec![
        "the record's primary track is empty: the Raven records a pressure, not an altitude; the \
         FIP's export works its altitudes out of the pressure"
            .to_owned(),
        format!(
            "read from the Raven's memory: {data_blocks} blocks of 0.05 s, the first {SETTINGS} \
             before the launch; a tick is {num}/{den} ms"
        ),
        "`slow channel 5` and `block word 30` and `31` are kept as written: the FIP's export \
         holds neither"
            .to_owned(),
    ];
    if !two_axes {
        notes.push(
            "the header's `#Axis` is 1: the Raven records no lateral accelerometer it calibrates, \
             so the lateral bits are kept as written, as `lateral counts`"
                .to_owned(),
        );
    }
    if let Some(stop) = ended {
        notes.push(format!(
            "the memory is unused from block {stop} of {}",
            blocks.len()
        ));
    }
    let date = field("DateTime").map(str::to_owned);
    if let Some(date) = date {
        notes.push(format!(
            "the header's `#DateTime` is {date:?}, as written: the file doesn't say which clock \
             wrote it"
        ));
    }

    let mut channels = Vec::new();
    for filling in [axial, lateral].into_iter().chain(slow).chain(tails) {
        channels.push(Channel {
            column: filling.column,
            quantity: filling.quantity,
            time_s: filling
                .ticks
                .iter()
                .map(|&tick| tick as f64 * tick_s)
                .collect(),
            values: filling.values,
        });
    }

    Ok(FlightLog {
        format: LogFormat::RavenFipa,
        logger: "Featherweight Raven".to_owned(),
        serial_number: None,
        firmware: field("R").map(str::to_owned),
        flight_number: field("FlightNum").and_then(|n| n.parse().ok()),
        stated: Stated {
            ground_elevation_msl_m: Some(calibration.f4 as f64 * FOOT_M),
            ..Stated::default()
        },
        time_s: Vec::new(),
        altitude_m: Vec::new(),
        vertical_speed_m_s: None,
        temperature_k: None,
        battery_v: None,
        channels,
        events: Vec::new(),
        notes,
    })
}

/// What a slow channel measures, by its export name.
fn quantity_of(name: &str) -> Quantity {
    match name {
        "Current Draw (A)" => Quantity::Current,
        "Temperature (F)" => Quantity::Temperature,
        "Baro (Atm)" => Quantity::Pressure,
        "Velocity (Accel-Ft/Sec)" => Quantity::Speed,
        _ => Quantity::Voltage,
    }
}

/// The `k`th slow channel's word in SI.
fn slow_si(k: usize, word: u16) -> f64 {
    match k {
        0..=4 | 6 => signed(word) as f64 / 1000.0,
        // As `crate::raven` converts the export's column, so that the two agree to the bit.
        7 => (signed(word) as f64 / 100.0 - 32.0) * 5.0 / 9.0 + 273.15,
        8 => f64::from(word) / BARO_COUNTS_PER_ATM * ATM_PA,
        9 => signed(word) as f64 / 10.0 * FOOT_M,
        _ => f64::from(word),
    }
}

/// A refusal at a byte of the file.
fn binary(offset: usize, message: String) -> LogError {
    LogError::Binary {
        format: FORMAT,
        offset,
        message,
    }
}
