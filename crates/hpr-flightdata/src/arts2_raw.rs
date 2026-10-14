//! Ozark Aerospace ARTS2 flight computer's raw data: the `.odf` file its PC software,
//! DataAnalyzer, saves a downloaded flight to, and the sensors' counts as the software exports
//! them to CSV.
//!
//! The ARTS2 reads an Analog Devices ADXL78 accelerometer, set for ±50 g, and a Motorola
//! MPXA4100A pressure sensor with its processor's 10-bit converter, and records both at a rate
//! its owner sets (*ARTS2 User Manual*, V1.0.0, Ozark Aerospace, 2007, Appendix A, p. 16; p. 7).
//! *Flight → Save As* keeps a download as an `.odf` file (p. 14). The manual describes neither
//! the `.odf`'s bytes nor a raw export; what follows was worked out from the three flights among
//! the project's sample logs that hold both, each beside the software's interpreted export of the
//! same flight ([`crate::arts2`]), with no other program's source read.
//!
//! # The `.odf`
//!
//! Little-endian throughout:
//!
//! | Offset | Bytes | What |
//! |---|---|---|
//! | 0 | 4 + 9 | the length 9, then `Ozark.odf` |
//! | 13 | 20 | five 32-bit words, 3, 3, 1, 61 and 2 in every sample file; not read |
//! | 33 | 256 | the board's settings and calibration (below) |
//! | 289 | 4 | the number of records, `n` |
//! | 293 | 12 | three 32-bit words, each the memory's size in records; equal, and at least `n` |
//! | 305 | 8 `n` | the records |
//! | 305 + 8 `n` | 124 | the software's own settings; not read |
//!
//! A file whose length isn't `305 + 8n + 124` is refused, as are three sizes that disagree or `n`
//! over them. Of the settings block, the reader takes three things, at these offsets within it:
//!
//! - 10, a 16-bit word: the barometer's count on the pad, the "Zero Altitude baseline" the board
//!   averages from 16 samples on entering flight mode (p. 17);
//! - 12, a 16-bit word: eight times the accelerometer's count on the pad, its "Zero G baseline",
//!   likewise averaged (p. 16), so that it keeps an eighth of a count;
//! - 20, 22 and 24, three 16-bit words: the sums of 64 accelerometer readings the board takes
//!   when calibrated on its side and on either end, "at +1g, −1g, and 0", which "are sent along
//!   with flight data" (p. 16). In every sample file the middle one is the reading on its side.
//!
//! And at 8, a 16-bit word that is 2 in every sample file, all recorded at 50 a second; a file
//! whose word isn't 2 is refused (the clock, below). Its other bytes, among them the two flight
//! profiles' deployment settings, aren't decoded. Each record is four
//! 16-bit words, in the order of the raw export's columns (below): the accelerometer's count, a
//! byte of flags and low bits, the barometer's count and the board's state.
//!
//! **The first record** is the baselines, not a sample: its barometer count is the pad's, and its
//! accelerometer count is the pad's eighth-count baseline cut to a whole count. A file whose first
//! record isn't so is refused: its bytes are laid out otherwise than the sample files'. The
//! software exports it as the flight's first row, at time 0, and so does the reader.
//!
//! **The clock.** The records are 0.02 s apart (50 a second), from the first: the rate DataAnalyzer
//! gives every sample file's records in both its exports. The owner sets the rate, in each of the
//! board's two flight profiles (pp. 6–7), and where the file keeps it isn't known: the sample files
//! are all at one rate, so no byte can be shown to change with it. The reader fails closed on the
//! one candidate, the settings word at 8, small and the same in every sample file: a file whose
//! word isn't 2 is refused. If the rate is kept elsewhere, a file recorded at another rate is
//! caught only by the climb check (below), loosely: a clock whose step is `k` times the true one
//! makes the climb by the acceleration `k²` times larger over the same records, so the check
//! passes a `k` from `√(r / (5/3))` to `√(r / 0.6)`, `r` the flight's ratio on its true clock: 0.67
//! to 1.12 at the sample files' least ratio, 0.754, and 0.83 to 1.38 at their greatest, 1.145.
//! Every record says so in a note.
//!
//! **Units.**
//!
//! | Record word | Column | Quantity | As read |
//! |---|---|---|---|
//! | 0 | `Acc(HI)` | [`Quantity::Acceleration`], along the board's long axis, toward its forward end, with the pad's reading taken out | from the calibration (below) |
//! | 1 | `Acc/Baro(LO)` | [`Quantity::Unrecognized`] | the byte as written |
//! | 2 | `Baro(HI)` | [`Quantity::Pressure`] | by the sensor's transfer function (below) |
//! | 3 | `State` | [`Quantity::Number`] | the byte as written |
//!
//! The columns' names are the raw export's; the `.odf` names none.
//!
//! *The acceleration.* The ±1 g sums are 2 g apart, so a g is a 128th of their difference, in
//! counts; the reading on the pad, nose up, lies near the end sum on the forward end's side, which
//! sets the sign: `a = ±(count − pad) / per_g × g₀`. As the software's interpreted export does,
//! the pad's reading is taken out, so the acceleration on the pad reads zero, gravity with it.
//! That takes out gravity exactly only while the board points as it did on the pad: once the
//! rocket tips over in the coast, the reading runs below the acceleration along the axis by
//! `g₀ (cos θ_pad − cos θ)`, `θ` the board's angle from the vertical, up to a g. The first
//! record, the baselines', reads the pad's whole count less its eighth-count baseline, up to
//! 7/8 of a count below zero, not zero. The
//! calibration is refused unless the ADXL78's data sheet bears it out (*ADXL78*, Rev. B, Analog
//! Devices, 2010, Specifications, AD22280 for ±50 g, p. 3; the output is ratiometric to its
//! 5 V supply, which the converter reads against, so a count is 5 V / 1,024): a g from 36.1 to
//! 39.9 mV, 7.39 to 8.17 counts, and the side's reading within ±150 mV of half the supply,
//! 512 ± 30.72 counts, between the two end sums; and the pad's reading nearer the end sum on its
//! side than the side's, a tilt under 60°.
//!
//! *The pressure.* `P = (count / 1,024 + 0.1518) / 0.010 59` kPa, the sensor's nominal transfer
//! function, `V_out = V_S (0.010 59 P − 0.1518)` (*MPX4100A*, Rev 9, Freescale, 2009, p. 5,
//! p. 6), read against the supply as the converter's full scale. The pad's count must give a
//! pressure in the sensor's rated 20 to 105 kPa (p. 3). The sensor's error band is ±1.5 kPa over
//! that range from 0 to 85 °C (p. 6).
//!
//! **The climb check** is each unit's second check, and the clock's: over the first 2 s from the
//! first record, the climb by the pressure, its pressure altitude in the U.S. Standard
//! Atmosphere, 1976, at the window's end (the median of the 5 records within 0.04 s of it) less
//! the pad's, over the climb by the acceleration integrated twice by the trapezoid rule from rest
//! at the first record, must lie from 0.6 to 5/3, as for an AltOS raw download
//! ([`crate::altos_eeprom`]), and the rocket must climb by both. The window is twice AltOS's 1 s:
//! one count of this barometer is 92 Pa, 7.7 m of height near sea level and more at a higher
//! site, so a 1 s window held two counts of one sample file's climb; 2 s holds at least 10 in
//! each. A file whose climb
//! can't be measured, or falls outside the band, is refused: its clock rests on that check as
//! much as its units do, and a count kept as written can't be put on a clock nothing bears out.
//!
//! **The flags byte.** Its bits 3 and 2 are the barometer count's low two bits, in every record of
//! every sample file; a record whose aren't is refused, a check that the record's words are where
//! the reader takes them. Its other bits follow the flight (bits 5 and 4 set about the apogee,
//! bits 1 and 0 before the main's), and bits 7 and 6 match the accelerometer count's low two
//! bits in one sample file and in 57 to 62% of the others' records, so the byte is kept as
//! written. So is `State`, which steps through 0, 1, 2, 3, 5, 6 and 7 in the flights in order;
//! no document names them, so no event is taken from it.
//!
//! **The primary track.** The file holds no altitude, so [`FlightLog::time_s`] and
//! [`FlightLog::altitude_m`] are empty, as for the Raven's raw `.FIPa`, and every word is a
//! channel. [`Stated::figures`] holds the two baselines and the three calibration sums.
//!
//! # The raw CSV
//!
//! The software's raw export of the same records: a `MET,Acc(HI),Acc/Baro(LO),Baro(HI),State`
//! header, then a row for each record, its time and its four words as whole numbers:
//!
//! ```text
//! MET,Acc(HI),Acc/Baro(LO),Baro(HI),State
//! 0.000,512,1,780,64
//! 0.020,575,1,780,0
//! ```
//!
//! In every sample file its rows are its `.odf`'s records, one for one. **Recognition**: the first
//! line that isn't blank is that header, its names compared as written. The file holds no
//! calibration, only the first record's whole-count baselines, so no column is converted, and
//! `MET`, DataAnalyzer's clock, is kept as the time:
//!
//! | Column | Quantity | As read |
//! |---|---|---|
//! | `MET` | the clock, s | as written; each greater than the row before's |
//! | `Acc(HI)`, `Baro(HI)` | [`Quantity::Unrecognized`] | the converter's counts, whole numbers 0 to 1,023, as written |
//! | `Acc/Baro(LO)`, `State` | [`Quantity::Unrecognized`], [`Quantity::Number`] | whole numbers 0 to 255, as written |
//!
//! Every row must hold five numbers, and its flags byte the barometer's low bits as above, or the
//! file is refused, naming the line. Read the flight's `.odf` for its values in units.

use crate::error::LogError;
use crate::log::{Channel, Figure, FlightLog, LogFormat, Quantity, Stated};

/// The `.odf` format's name in errors.
const ODF: &str = "ARTS2 .odf";

/// The raw CSV format's name in errors.
const CSV: &str = "ARTS2 raw CSV";

/// The logger, as the record names it.
const LOGGER: &str = "Ozark Aerospace ARTS2";

/// The `.odf`'s first 13 bytes: the length 9, then the name.
const MAGIC: &[u8; 13] = b"\x09\x00\x00\x00Ozark.odf";

/// Where the settings block starts, and its length.
const BLOCK_AT: usize = 33;
const BLOCK_LEN: usize = 256;

/// Where the record count is, then the three memory sizes, then the records.
const COUNT_AT: usize = BLOCK_AT + BLOCK_LEN;
const SIZES_AT: usize = COUNT_AT + 4;
const RECORDS_AT: usize = SIZES_AT + 12;

/// A record's length, bytes, and the software's settings' after the records.
const RECORD_LEN: usize = 8;
const TRAILER_LEN: usize = 124;

/// Offsets within the settings block: the pad's barometer count, eight times its accelerometer
/// count, and the three calibration sums.
const PAD_BARO: usize = 10;
const PAD_ACC_EIGHTHS: usize = 12;
const CALIBRATION_SUMS: usize = 20;

/// The settings word that may hold the rate, and its value in every sample file (module docs,
/// *The clock*).
const RATE_WORD: usize = 8;
const RATE_CODE: u16 = 2;

/// How many readings each calibration sum adds up (Appendix A, p. 16).
const SUMMED: f64 = 64.0;

/// The records' rate, a second: DataAnalyzer's exports put every sample file's 0.02 s apart.
const RATE_HZ: f64 = 50.0;

/// The raw CSV's header.
const HEADER: [&str; 5] = ["MET", "Acc(HI)", "Acc/Baro(LO)", "Baro(HI)", "State"];

/// The largest count of the 10-bit converter, and of a byte.
const MAX_COUNT: u16 = 1_023;
const MAX_BYTE: u16 = 255;

/// Standard gravity, m/s².
const STANDARD_GRAVITY: f64 = 9.806_65;

/// One count of the converter, mV: 5 V over 1,024.
const COUNT_MV: f64 = 5_000.0 / 1_024.0;

/// The ADXL78's sensitivity for ±50 g (AD22280) at a 5 V supply, mV/g, least and most (Rev. B,
/// p. 3).
const ADXL78_MV_PER_G: std::ops::RangeInclusive<f64> = 36.1..=39.9;

/// How far the ADXL78's output at zero g may lie from half its supply, mV (Rev. B, p. 3).
const ADXL78_ZERO_G_MV: f64 = 150.0;

/// The MPX4100A's rated pressures, Pa (Rev 9, p. 3).
const MPX4100A_PA: std::ops::RangeInclusive<f64> = 20_000.0..=105_000.0;

/// The climb check's window, records: 2 s at 50 a second.
const CLIMB_WINDOW: usize = 100;

/// Half the span, records, of those whose median pressure ends the window: 0.04 s.
const CLIMB_END: usize = 2;

/// The barometer's step, as the format's resolution ([`LogFormat::altitude_resolution_m`]): the
/// files write no altitude, but the pressure moves by one count of the converter, 92.2 Pa (the
/// transfer function's slope, module docs), 7.68 m near sea level (`Δh = Δp / (ρ₀ g₀)`). Aloft
/// it is larger.
pub const ALTITUDE_STEP_M: f64 = 1_000.0 / (1_024.0 * 0.010_59) / (1.225 * STANDARD_GRAVITY);

/// Whether a file's bytes are an `.odf`: they open with the length 9 and `Ozark.odf`.
pub fn recognizes_odf(bytes: &[u8]) -> bool {
    bytes.starts_with(MAGIC)
}

/// Whether a file's text is the raw CSV: its first line that isn't blank is the header, its lines
/// ending in CR, LF or both.
pub fn recognizes_csv(text: &str) -> bool {
    text.strip_prefix('\u{feff}')
        .unwrap_or(text)
        .split(['\r', '\n'])
        .find(|line| !line.trim().is_empty())
        .is_some_and(is_header)
}

/// Whether a line is the raw CSV's header.
fn is_header(line: &str) -> bool {
    line.trim().split(',').map(str::trim).eq(HEADER)
}

/// One record: the accelerometer's count, the flags byte, the barometer's count and the state.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Record {
    acc: u16,
    flags: u16,
    baro: u16,
    state: u16,
}

impl Record {
    /// Why the record's words can't be the board's, if they can't: a count over the converter's,
    /// a byte over 255, or a flags byte whose bits 3 and 2 aren't the barometer count's low bits.
    fn fault(&self) -> Option<String> {
        if self.acc > MAX_COUNT || self.baro > MAX_COUNT {
            return Some(format!(
                "the counts {} and {} aren't both a 10-bit converter's",
                self.acc, self.baro
            ));
        }
        if self.flags > MAX_BYTE || self.state > MAX_BYTE {
            return Some(format!(
                "the flags {} and state {} aren't both a byte",
                self.flags, self.state
            ));
        }
        if (self.flags >> 2) & 3 != self.baro & 3 {
            return Some(format!(
                "the flags byte {} doesn't hold the barometer count {}'s low bits",
                self.flags, self.baro
            ));
        }
        None
    }
}

/// The settings block's baselines and calibration.
#[derive(Debug, Clone, Copy)]
struct Calibration {
    pad_baro: u16,
    pad_acc_eighths: u16,
    sums: [u16; 3],
}

/// The acceleration's conversion: the pad's count, the counts a g, and the sign.
#[derive(Debug, Clone, Copy)]
struct Axis {
    pad: f64,
    per_g: f64,
    sign: f64,
}

impl Axis {
    /// A count's acceleration, m/s².
    fn acceleration(&self, count: u16) -> f64 {
        self.sign * (f64::from(count) - self.pad) / self.per_g * STANDARD_GRAVITY
    }
}

impl Calibration {
    /// The acceleration's conversion, or why the data sheet doesn't bear the calibration out.
    fn axis(&self) -> Result<Axis, String> {
        let [a, side, b] = self.sums.map(|sum| f64::from(sum) / SUMMED);
        let per_g = (b - a).abs() / 2.0;
        let mv_per_g = per_g * COUNT_MV;
        if !ADXL78_MV_PER_G.contains(&mv_per_g) {
            return Err(format!(
                "the calibration's ends, {a:.2} and {b:.2} counts, make a g {mv_per_g:.2} mV, \
                 outside the ADXL78's 36.1 to 39.9"
            ));
        }
        let off_mv = (side - 512.0) * COUNT_MV;
        if off_mv.abs() > ADXL78_ZERO_G_MV || side <= a.min(b) || side >= a.max(b) {
            return Err(format!(
                "the calibration's side reading, {side:.2} counts, isn't zero g: it must lie \
                 between its ends, {a:.2} and {b:.2}, and within the ADXL78's 150 mV of half \
                 its supply (it is {off_mv:.1} mV)"
            ));
        }
        let pad = f64::from(self.pad_acc_eighths) / 8.0;
        let near = if (pad - a).abs() < (pad - b).abs() {
            a
        } else {
            b
        };
        if (pad - near).abs() >= (pad - side).abs() {
            return Err(format!(
                "the pad's reading, {pad:.3} counts, lies nearer the side's, {side:.2}, than \
                 either end's: the board wasn't upright on the pad"
            ));
        }
        Ok(Axis {
            pad,
            per_g,
            sign: if near > side { 1.0 } else { -1.0 },
        })
    }
}

/// A barometer count's pressure, Pa, by the MPX4100A's nominal transfer function.
fn pressure_pa(count: u16) -> f64 {
    (f64::from(count) / 1_024.0 + 0.1518) / 0.010_59 * 1_000.0
}

/// A little-endian 16-bit word at an offset; the caller has checked the length.
fn word(bytes: &[u8], at: usize) -> u16 {
    u16::from_le_bytes([bytes[at], bytes[at + 1]])
}

/// A little-endian 32-bit word at an offset; the caller has checked the length.
fn long(bytes: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]])
}

/// A fault in an `.odf`'s bytes.
fn fault(offset: usize, message: impl Into<String>) -> LogError {
    LogError::Binary {
        format: ODF,
        offset,
        message: message.into(),
    }
}

/// Reads an ARTS2 `.odf` file.
///
/// # Errors
///
/// [`LogError::NotThisFormat`] if the bytes don't open as an `.odf`'s; [`LogError::Binary`] if
/// they aren't laid out as the module docs say, the settings word that may hold the rate isn't
/// the sample files', the calibration or the pad's pressure isn't borne out by the sensors' data
/// sheets, or the climb check fails or can't be measured;
/// [`LogError::NoData`] if the file holds no record after the baselines.
pub fn read_odf(bytes: &[u8]) -> Result<FlightLog, LogError> {
    if !recognizes_odf(bytes) {
        return Err(LogError::NotThisFormat {
            format: ODF,
            message: "the file doesn't open with the length 9 and `Ozark.odf`".to_owned(),
        });
    }
    if bytes.len() < RECORDS_AT + TRAILER_LEN {
        return Err(fault(
            bytes.len(),
            format!(
                "the file ends at {} bytes, before its {} of header and settings",
                bytes.len(),
                RECORDS_AT + TRAILER_LEN
            ),
        ));
    }
    let n = long(bytes, COUNT_AT) as usize;
    let sizes = [0, 4, 8].map(|k| long(bytes, SIZES_AT + k) as usize);
    if sizes.iter().any(|&size| size != sizes[0]) || n > sizes[0] {
        return Err(fault(
            COUNT_AT,
            format!(
                "{n} records and memory sizes {}, {} and {}: the sizes must agree and hold them",
                sizes[0], sizes[1], sizes[2]
            ),
        ));
    }
    let expected = n
        .checked_mul(RECORD_LEN)
        .and_then(|records| records.checked_add(RECORDS_AT + TRAILER_LEN));
    if expected != Some(bytes.len()) {
        return Err(fault(
            COUNT_AT,
            format!(
                "{n} records make a file of {} bytes, not {}",
                expected.map_or_else(|| "too many".to_owned(), |size| size.to_string()),
                bytes.len()
            ),
        ));
    }
    let block = |at: usize| word(bytes, BLOCK_AT + at);
    let calibration = Calibration {
        pad_baro: block(PAD_BARO),
        pad_acc_eighths: block(PAD_ACC_EIGHTHS),
        sums: [0, 2, 4].map(|k| block(CALIBRATION_SUMS + k)),
    };
    let mut records = Vec::with_capacity(n);
    for k in 0..n {
        let at = RECORDS_AT + k * RECORD_LEN;
        let record = Record {
            acc: word(bytes, at),
            flags: word(bytes, at + 2),
            baro: word(bytes, at + 4),
            state: word(bytes, at + 6),
        };
        if let Some(why) = record.fault() {
            return Err(fault(at, format!("record {k}: {why}")));
        }
        records.push(record);
    }
    let Some(first) = records.first() else {
        return Err(LogError::NoData { format: ODF });
    };
    let pad_acc = f64::from(calibration.pad_acc_eighths) / 8.0;
    if first.baro != calibration.pad_baro || f64::from(first.acc) != pad_acc.floor() {
        return Err(fault(
            RECORDS_AT,
            format!(
                "the first record's counts, {} and {}, aren't the pad's baselines, {} and {pad_acc}",
                first.acc, first.baro, calibration.pad_baro
            ),
        ));
    }
    if records.len() < 2 {
        return Err(LogError::NoData { format: ODF });
    }
    let rate_code = block(RATE_WORD);
    if rate_code != RATE_CODE {
        return Err(fault(
            BLOCK_AT + RATE_WORD,
            format!(
                "the settings word {rate_code} at the rate's likely place isn't the {RATE_CODE} of \
                 every sample file, all at 50 records a second: another rate may be set, which the \
                 reader can't yet read"
            ),
        ));
    }
    let pad_pa = pressure_pa(calibration.pad_baro);
    if !MPX4100A_PA.contains(&pad_pa) {
        return Err(fault(
            BLOCK_AT + PAD_BARO,
            format!(
                "the pad's barometer count {} is {:.0} Pa, outside the MPX4100A's 20 to 105 kPa",
                calibration.pad_baro, pad_pa
            ),
        ));
    }
    let axis = calibration
        .axis()
        .map_err(|why| fault(BLOCK_AT + CALIBRATION_SUMS, why))?;

    let time_s: Vec<f64> = (0..records.len()).map(|k| k as f64 / RATE_HZ).collect();
    let acceleration: Vec<f64> = records.iter().map(|r| axis.acceleration(r.acc)).collect();
    let pressure: Vec<f64> = records.iter().map(|r| pressure_pa(r.baro)).collect();
    let climb = climb(pad_pa, &pressure, &acceleration).map_err(|why| fault(RECORDS_AT, why))?;

    let channel = |column: &str, quantity, values: Vec<f64>| Channel {
        column: column.to_owned(),
        quantity,
        time_s: time_s.clone(),
        values,
    };
    let channels = vec![
        channel(HEADER[1], Quantity::Acceleration, acceleration),
        channel(
            HEADER[2],
            Quantity::Unrecognized,
            records.iter().map(|r| f64::from(r.flags)).collect(),
        ),
        channel(HEADER[3], Quantity::Pressure, pressure),
        channel(
            HEADER[4],
            Quantity::Number,
            records.iter().map(|r| f64::from(r.state)).collect(),
        ),
    ];
    let sums = calibration.sums.map(f64::from);
    let figures = vec![
        Figure::new(
            "Zero Altitude baseline",
            calibration.pad_baro.to_string(),
            Some(Quantity::Pressure),
            vec![pad_pa],
        ),
        Figure::new(
            "Zero G baseline",
            calibration.pad_acc_eighths.to_string(),
            Some(Quantity::Unrecognized),
            vec![pad_acc],
        ),
        Figure::new(
            "calibration sums",
            format!("{},{},{}", sums[0], sums[1], sums[2]),
            Some(Quantity::Unrecognized),
            sums.to_vec(),
        ),
    ];
    let notes = vec![
        format!(
            "`Acc(HI)` read in m/s², along the board's axis with the pad's reading taken out: \
             the calibration's ends, {:.2} and {:.2} counts, make a g {:.4} counts ({:.2} mV, \
             within the ADXL78's 36.1 to 39.9), its side {:.2} counts, and the pad's baseline \
             {:.3} counts",
            sums[0] / SUMMED,
            sums[2] / SUMMED,
            axis.per_g,
            axis.per_g * COUNT_MV,
            sums[1] / SUMMED,
            axis.pad
        ),
        format!(
            "`Baro(HI)` read in pascals, by the MPX4100A's nominal transfer function: the pad's \
             count {} is {:.0} Pa",
            calibration.pad_baro, pad_pa
        ),
        format!(
            "`Acc(HI)` and `Baro(HI)` held to each other, and the clock at 50 records a second: \
             over the first 2 s the rocket climbs {:.1} m by the pressure in the standard \
             atmosphere and {:.1} m by the acceleration integrated twice from rest, a ratio of \
             {:.3}, within 0.6 to 5/3",
            climb.0,
            climb.1,
            climb.0 / climb.1
        ),
        "`Acc/Baro(LO)` and `State` kept as written: no document names their bits or codes"
            .to_owned(),
        "the clock is 50 records a second, the rate of every sample file: the file states none \
         the reader can read, and the climb check bounds it only loosely"
            .to_owned(),
        "the first record is the pad's baselines, as the software's exports place it at time 0"
            .to_owned(),
        format!(
            "not read: the settings block's {} other bytes, and the software's {TRAILER_LEN} \
             after the records",
            BLOCK_LEN - 12
        ),
    ];
    Ok(FlightLog {
        format: LogFormat::Arts2Odf,
        logger: LOGGER.to_owned(),
        serial_number: None,
        firmware: None,
        flight_number: None,
        stated: Stated {
            samples: Some(n),
            figures,
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

/// The climb check's two climbs, m: by the pressure, and by the acceleration (module docs).
fn climb(pad_pa: f64, pressure: &[f64], acceleration: &[f64]) -> Result<(f64, f64), String> {
    if pressure.len() <= CLIMB_WINDOW + CLIMB_END {
        return Err(format!(
            "the climb check can't be measured: {} records end before its 2 s window, so \
             nothing bears out the clock or the units",
            pressure.len()
        ));
    }
    let dt = 1.0 / RATE_HZ;
    let (mut speed, mut by_acceleration) = (0.0, 0.0);
    for pair in acceleration[..=CLIMB_WINDOW].windows(2) {
        let next = speed + (pair[0] + pair[1]) / 2.0 * dt;
        by_acceleration += (speed + next) / 2.0 * dt;
        speed = next;
    }
    let mut near = pressure[CLIMB_WINDOW - CLIMB_END..=CLIMB_WINDOW + CLIMB_END].to_vec();
    let end_pa = crate::filter::median(&mut near).unwrap_or(f64::NAN);
    let atmosphere = hpr_atmos::ussa76::Ussa76::standard();
    let height = |pa: f64| atmosphere.pressure_altitude_m(pa).unwrap_or(f64::NAN);
    let by_pressure = height(end_pa) - height(pad_pa);
    let ratio = by_pressure / by_acceleration;
    if by_pressure > 0.0
        && by_acceleration > 0.0
        && crate::altos_eeprom::CLIMB_RATIO.contains(&ratio)
    {
        Ok((by_pressure, by_acceleration))
    } else {
        Err(format!(
            "over the first 2 s the rocket climbs {by_pressure:.1} m by the pressure and \
             {by_acceleration:.1} m by the acceleration, a ratio of {ratio:.3}, not both upward \
             and within 0.6 to 5/3: nothing bears out the clock or the units"
        ))
    }
}

/// Reads an ARTS2 raw CSV export.
///
/// # Errors
///
/// [`LogError::NotThisFormat`] if the first line that isn't blank isn't the header;
/// [`LogError::Syntax`] for a row that doesn't hold five numbers, a count out of its range, a
/// flags byte without the barometer's low bits, or a time not after the row before's;
/// [`LogError::NoData`] if no row follows the header.
pub fn read_csv(text: &str) -> Result<FlightLog, LogError> {
    let mut lines =
        crate::table::numbered_lines(text).skip_while(|(_, line)| line.trim().is_empty());
    if !lines.next().is_some_and(|(_, line)| is_header(line)) {
        return Err(LogError::NotThisFormat {
            format: CSV,
            message: format!(
                "the first line that isn't blank isn't `{}`",
                HEADER.join(",")
            ),
        });
    }
    let mut time_s = Vec::new();
    let mut records = Vec::new();
    for (line_number, line) in lines {
        if line.trim().is_empty() {
            continue;
        }
        let syntax = |message: String| LogError::Syntax {
            format: CSV,
            line: line_number,
            message,
        };
        let cells: Vec<&str> = line.split(',').map(str::trim).collect();
        let [met, acc, flags, baro, state] = cells[..] else {
            return Err(syntax(format!("{} cells, not 5", cells.len())));
        };
        let time: f64 = met
            .parse()
            .ok()
            .filter(|t: &f64| t.is_finite())
            .ok_or_else(|| syntax(format!("`MET` {} isn't a number", crate::table::short(met))))?;
        if time_s.last().is_some_and(|&last| time <= last) {
            return Err(syntax(format!("`MET` {time} isn't after the row before's")));
        }
        let whole = |cell: &str, name: &str| {
            cell.parse::<u16>().map_err(|_| {
                syntax(format!(
                    "`{name}` {} isn't a count",
                    crate::table::short(cell)
                ))
            })
        };
        let record = Record {
            acc: whole(acc, HEADER[1])?,
            flags: whole(flags, HEADER[2])?,
            baro: whole(baro, HEADER[3])?,
            state: whole(state, HEADER[4])?,
        };
        if let Some(why) = record.fault() {
            return Err(syntax(why));
        }
        time_s.push(time);
        records.push(record);
    }
    if records.is_empty() {
        return Err(LogError::NoData { format: CSV });
    }
    let channel = |k: usize, quantity, word: fn(&Record) -> u16| Channel {
        column: HEADER[k].to_owned(),
        quantity,
        time_s: time_s.clone(),
        values: records.iter().map(|r| f64::from(word(r))).collect(),
    };
    let channels = vec![
        channel(1, Quantity::Unrecognized, |r| r.acc),
        channel(2, Quantity::Unrecognized, |r| r.flags),
        channel(3, Quantity::Unrecognized, |r| r.baro),
        channel(4, Quantity::Number, |r| r.state),
    ];
    let notes = vec![
        "`Acc(HI)` and `Baro(HI)` kept as written: the export holds the converter's counts and \
         no calibration; the flight's `.odf` holds the same records with the board's, and reads \
         in units"
            .to_owned(),
        "`Acc/Baro(LO)` and `State` kept as written: no document names their bits or codes"
            .to_owned(),
        "the first row is the pad's baselines, as whole counts, not a sample".to_owned(),
    ];
    Ok(FlightLog {
        format: LogFormat::Arts2RawCsv,
        logger: LOGGER.to_owned(),
        serial_number: None,
        firmware: None,
        flight_number: None,
        stated: Stated::default(),
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

#[cfg(test)]
mod tests {
    use super::*;

    /// The pressure's transfer function against a hand computation: count 780 is
    /// (780 / 1,024 + 0.1518) / 0.010 59 = 86.2624 kPa.
    #[test]
    fn a_count_is_the_sensors_nominal_pressure() {
        assert!(
            (pressure_pa(780) - 86_262.39).abs() < 0.01,
            "{}",
            pressure_pa(780)
        );
    }

    fn calibration(sums: [u16; 3], pad_eighths: u16) -> Calibration {
        Calibration {
            pad_baro: 780,
            pad_acc_eighths: pad_eighths,
            sums,
        }
    }

    /// Ends 64 × 496 and 64 × 512 make a g 8 counts, 39.06 mV; the pad at 512 is on the high
    /// end's side, so a count above it is upward: 520 is one g.
    #[test]
    fn the_calibration_gives_the_scale_and_the_sign() {
        let axis = calibration([31_744, 32_256, 32_768], 4_096).axis().unwrap();
        assert_eq!(axis.per_g, 8.0);
        assert!((axis.acceleration(520) - STANDARD_GRAVITY).abs() < 1e-12);
        // The board turned round: the pad on the low end's side, so counts fall upward.
        let axis = calibration([31_744, 32_256, 32_768], 3_968).axis().unwrap();
        assert!((axis.acceleration(488) - STANDARD_GRAVITY).abs() < 1e-12);
    }

    /// Each data-sheet band refuses: a g of 7 counts (34.2 mV), a side 40 counts off (195 mV),
    /// a side outside its ends, and a pad nearer the side than either end.
    #[test]
    fn a_calibration_the_data_sheet_doesnt_bear_out_is_refused() {
        let cases = [
            ([31_808, 32_256, 32_704], 4_096, "outside the ADXL78"),
            ([34_304, 34_816, 35_328], 4_416, "isn't zero g"),
            ([31_744, 33_000, 32_768], 4_096, "isn't zero g"),
            ([31_744, 32_256, 32_768], 4_064, "wasn't upright"),
        ];
        for (sums, pad, why) in cases {
            let error = calibration(sums, pad).axis().unwrap_err();
            assert!(error.contains(why), "{sums:?} {pad}: {error}");
        }
    }

    /// At each band's edge: ends 947 apart make a g 7.398 counts, 36.12 mV, and read; 946,
    /// 36.09 mV, are refused; 1,045, 39.86 mV, read; 1,046, 39.90 mV, are refused.
    #[test]
    fn a_calibration_at_the_bands_edges() {
        let cases = [
            (473, 474, true),
            (473, 473, false),
            (522, 523, true),
            (523, 523, false),
        ];
        for (below, above, reads) in cases {
            let sums = [32_256 - below, 32_256, 32_256 + above];
            assert_eq!(calibration(sums, 4_100).axis().is_ok(), reads, "{sums:?}");
        }
    }

    /// The flags byte must carry the barometer's low bits; a count over 1,023 or a byte over 255
    /// is refused.
    #[test]
    fn a_records_words_are_checked() {
        let good = Record {
            acc: 512,
            flags: 0b1100_0100,
            baro: 777,
            state: 0,
        };
        assert_eq!(good.fault(), None);
        assert!(
            Record { flags: 0, ..good }
                .fault()
                .unwrap()
                .contains("low bits")
        );
        assert!(
            Record { acc: 1_024, ..good }
                .fault()
                .unwrap()
                .contains("10-bit")
        );
        assert!(
            Record { state: 256, ..good }
                .fault()
                .unwrap()
                .contains("byte")
        );
    }
}
