//! The Featherweight Raven's raw `.FIPa` downloads, read through the importer from invented files
//! that a small writer below lays out (M7.1c5): a text header, then frames of four blocks of 32
//! big-endian words with their checksums, then `#DONE`. The values each test expects are worked by
//! hand from the words it writes.

use hpr_core::gravity::STANDARD_GRAVITY_MPS2;
use hpr_flightdata::error::LogError;
use hpr_flightdata::import;
use hpr_flightdata::log::{Channel, FlightLog, LogFormat, Quantity};

/// The invented calibration: axial offset 0.05 g, ground 1,000 ft, axial gain 8,040 (thousandths
/// of a hundredth of a g a count) about 1,000 counts, lateral gain 2,000 about 512 counts.
fn settings() -> [u16; 32] {
    let mut words = [0u16; 32];
    words[0] = 5;
    words[1] = 1000;
    words[23] = 0;
    words[24] = 8040;
    words[25] = 1000;
    words[26] = 0;
    words[27] = 2000;
    words[28] = 512;
    words
}

/// A data block: ten triplets, each with axial counts `axial`, lateral counts `lateral` and its
/// slow channel's word from `slow`, then the two last words.
fn block(axial: (u16, u16), lateral: u16, slow: [u16; 10], last: [u16; 2]) -> [u16; 32] {
    let mut words = [0u16; 32];
    for (k, word) in slow.iter().enumerate() {
        words[3 * k] = (axial.0 << 5) | (lateral >> 5);
        words[3 * k + 1] = ((lateral & 0x1f) << 11) | axial.1;
        words[3 * k + 2] = *word;
    }
    words[30] = last[0];
    words[31] = last[1];
    words
}

/// The slow words of a block on the pad: no current, pyro channels at 5, 5.1, 0 and −0.02 V, slow
/// channel 5's word 0x1234, the battery at 8 V, 70 °F, 0.94 atm, and a velocity of `velocity`.
fn slow(velocity: i16) -> [u16; 10] {
    [
        0,
        5000,
        5100,
        0,
        (-20i16) as u16,
        0x1234,
        8000,
        7000,
        23_500,
        velocity as u16,
    ]
}

/// A header, its `#FlightState` listing the first 29 settings words.
fn header(settings: &[u16; 32], axis: &str) -> String {
    let listed: Vec<String> = settings[..29].iter().map(|w| w.to_string()).collect();
    format!(
        "#Component=Raven\r\n#Style=4-Output\r\n#DateTime=1/2/2020 3:04:05 PM\r\n#FlightNum=7\r\n\
         #FlightState=#7\tO\t{}\r\n#BinaryFormat\t1.0\r\n#R 3.0\r\n#D Jan 1 2015 00:00:00\r\n\
         #Axis\t{axis}\r\n#Num\t25\r\n#den\t20\r\n",
        listed.join("\t")
    )
}

/// The file: the header, the blocks in frames of four (the last filled with unused blocks), each
/// frame's checksum the sum of its bytes, then `#DONE`.
fn file(header: &str, blocks: &[[u16; 32]]) -> Vec<u8> {
    let mut bytes = header.as_bytes().to_vec();
    let mut blocks = blocks.to_vec();
    while !blocks.len().is_multiple_of(4) {
        blocks.push([0xffff; 32]);
    }
    for frame in blocks.chunks(4) {
        let data: Vec<u8> = frame
            .iter()
            .flat_map(|block| block.iter().flat_map(|word| word.to_be_bytes()))
            .collect();
        bytes.push(b':');
        bytes.extend(&data);
        bytes.push(data.iter().fold(0u8, |sum, b| sum.wrapping_add(*b)));
    }
    bytes.extend(b"#DONE\r\n");
    bytes
}

/// A flight: five blocks before the launch, the settings block, then three blocks of flight
/// whose velocity is 100 ft/s, then −1 ft/s, then −1 ft/s.
fn flight(axis: &str) -> Vec<u8> {
    let settings = settings();
    let pad = block((1025, 1000), 612, slow(0), [0xabcd, 7]);
    let mut blocks = vec![pad; 5];
    blocks.push(settings);
    blocks.push(block((1100, 1025), 612, slow(1000), [1, 2]));
    blocks.push(block((900, 1000), 512, slow(-10), [3, 4]));
    blocks.push(block((1000, 1000), 412, slow(-10), [5, 6]));
    file(&header(&settings, axis), &blocks)
}

/// The channel named `column`.
#[cfg(test)]
fn channel<'a>(log: &'a FlightLog, column: &str) -> &'a Channel {
    log.channels
        .iter()
        .find(|c| c.column == column)
        .unwrap_or_else(|| panic!("no channel {column}: {:#?}", log.channels))
}

/// The tick, s: 25/20 ms.
const TICK_S: f64 = 25.0 / 20.0 / 1000.0;

#[test]
fn a_flight_reads_every_channel_on_its_own_clock() {
    let bytes = flight("2");
    assert_eq!(import::detect(&bytes), Some(LogFormat::RavenFipa));
    let log = import::read(&bytes).unwrap();
    assert_eq!(log.format, LogFormat::RavenFipa);
    assert_eq!(log.logger, "Featherweight Raven");
    assert_eq!(log.firmware.as_deref(), Some("3.0"));
    assert_eq!(log.flight_number, Some(7));
    assert_eq!(log.stated.ground_elevation_msl_m, Some(1000.0 * 0.3048));
    // No altitude in the file: the primary track is empty.
    assert!(log.time_s.is_empty() && log.altitude_m.is_empty());

    // Eight data blocks, ten triplets each: two axial samples a triplet, one lateral, one slow.
    let axial = channel(&log, "Axial Accel (Gs)");
    assert_eq!(axial.quantity, Quantity::Acceleration);
    assert_eq!(axial.values.len(), 8 * 20);
    // Before the launch, the first triplet's samples at ticks 0 and 2.
    assert_eq!(axial.time_s[..2], [0.0, 2.0 * TICK_S]);
    // After the settings block, one tick later: the sixth data block starts at tick 200 + 1.
    assert_eq!(axial.time_s[100], 201.0 * TICK_S);
    // 1,025 counts: 8.04 × 25 is 200.999… in double precision, so trunc gives 200, + 5: 2.05 g
    // (exact integers would give 2.06); 1,000 counts: 0 + 5, 0.05 g.
    assert_eq!(axial.values[0], 2.05 * STANDARD_GRAVITY_MPS2);
    assert_eq!(axial.values[1], 0.05 * STANDARD_GRAVITY_MPS2);
    // 1,100 counts: 8.04 × 100 is 803.999… in double precision, so trunc gives 803, + 5: 8.08
    // g, where exact integers would give 8.09; 1,025: 2.05 g.
    assert_eq!(axial.values[100], 8.08 * STANDARD_GRAVITY_MPS2);
    assert_eq!(axial.values[101], 2.05 * STANDARD_GRAVITY_MPS2);
    // 900 counts: trunc(8.04 × −100) = −803, toward zero, + 5: −7.98 g.
    assert_eq!(axial.values[120], -7.98 * STANDARD_GRAVITY_MPS2);

    let lateral = channel(&log, "Lateral Accel (Gs)");
    assert_eq!(lateral.values.len(), 8 * 10);
    assert_eq!(lateral.time_s[0], TICK_S);
    // 612 counts: trunc(2 × 100) = 200, 2.00 g; 412: −2.00 g.
    assert_eq!(lateral.values[0], 2.0 * STANDARD_GRAVITY_MPS2);
    assert_eq!(lateral.values[79], -2.0 * STANDARD_GRAVITY_MPS2);

    for (column, quantity, value) in [
        ("Current Draw (A)", Quantity::Current, 0.0),
        ("Volts Pyro Apogee (V)", Quantity::Voltage, 5.0),
        ("Volts Pyro Main (V)", Quantity::Voltage, 5.1),
        ("Volts Pyro 3rd (V)", Quantity::Voltage, 0.0),
        ("Volts Pyro 4th (V)", Quantity::Voltage, -0.02),
        ("Volts Battery (V)", Quantity::Voltage, 8.0),
        (
            "Temperature (F)",
            Quantity::Temperature,
            (70.0 - 32.0) * 5.0 / 9.0 + 273.15,
        ),
        ("Baro (Atm)", Quantity::Pressure, 0.94 * 101_325.0),
        (
            "slow channel 5",
            Quantity::Unrecognized,
            f64::from(0x1234u16),
        ),
    ] {
        let slow = channel(&log, column);
        assert_eq!(slow.quantity, quantity, "{column}");
        assert_eq!(slow.values.len(), 8, "{column}");
        assert_eq!(slow.values[0], value, "{column}");
    }
    // The slow channel k at tick 4k + 3 of its block: the battery, k = 6, at 27, then at 200 + 28.
    let battery = channel(&log, "Volts Battery (V)");
    assert_eq!(battery.time_s[0], 27.0 * TICK_S);
    assert_eq!(battery.time_s[5], 228.0 * TICK_S);

    // Every velocity the Raven wrote, past the first at or below zero after it went positive.
    let velocity = channel(&log, "Velocity (Accel-Ft/Sec)");
    assert_eq!(velocity.quantity, Quantity::Speed);
    assert_eq!(
        velocity.values[4..],
        [0.0, 100.0 * 0.3048, -0.3048, -0.3048]
    );

    let last = channel(&log, "block word 30");
    assert_eq!(last.quantity, Quantity::Unrecognized);
    assert_eq!(
        last.values[..6],
        [43981.0, 43981.0, 43981.0, 43981.0, 43981.0, 1.0]
    );
    assert_eq!(channel(&log, "block word 31").values[7], 6.0);
    // Every channel's clock rises.
    for channel in &log.channels {
        assert!(
            channel.time_s.windows(2).all(|pair| pair[0] < pair[1]),
            "{}",
            channel.column
        );
    }
    insta::assert_json_snapshot!(log.notes);
}

#[test]
fn a_one_axis_header_keeps_the_lateral_bits_as_counts() {
    let log = import::read(&flight("1")).unwrap();
    assert!(
        !log.channels
            .iter()
            .any(|c| c.column == "Lateral Accel (Gs)")
    );
    let counts = channel(&log, "lateral counts");
    assert_eq!(counts.quantity, Quantity::Unrecognized);
    assert_eq!(counts.values[0], 612.0);
    assert!(log.notes.iter().any(|n| n.contains("`#Axis` is 1")));
}

/// The byte a refusal names.
#[cfg(test)]
fn refused_at(bytes: &[u8]) -> (usize, String) {
    match import::read(bytes) {
        Err(LogError::Binary {
            format: "Raven .FIPa",
            offset,
            message,
        }) => (offset, message),
        other => panic!("expected a refusal at a byte, got {other:?}"),
    }
}

#[test]
fn a_frame_whose_checksum_fails_is_refused_at_the_frame() {
    let mut bytes = flight("2");
    let body = header(&settings(), "2").len();
    assert_eq!(bytes[body], b':');
    // A byte in the second frame.
    bytes[body + 258 + 40] ^= 0x01;
    let (offset, message) = refused_at(&bytes);
    assert_eq!(offset, body + 258);
    assert!(message.contains("checksum"), "{message}");
}

#[test]
fn what_follows_the_frames_must_be_done() {
    let mut bytes = flight("2");
    let done = bytes.len() - b"#DONE\r\n".len();
    bytes.truncate(done);
    bytes.extend(b"#DONE\r\nmore");
    let (offset, message) = refused_at(&bytes);
    assert_eq!(offset, done);
    assert!(message.contains("#DONE"), "{message}");
    // A frame cut short is refused at that frame.
    let mut cut = flight("2");
    cut.truncate(done - 10);
    let (offset, message) = refused_at(&cut);
    assert_eq!(offset, done - 258);
    assert!(message.contains("cut short"), "{message}");
}

#[test]
fn a_settings_block_other_than_the_headers_is_refused() {
    let settings = settings();
    let mut other = settings;
    other[1] = 999;
    let pad = block((1025, 1000), 612, slow(0), [0, 0]);
    let mut blocks = vec![pad; 5];
    blocks.push(other);
    blocks.push(pad);
    let (_, message) = refused_at(&file(&header(&settings, "2"), &blocks));
    assert!(message.contains("#FlightState"), "{message}");
}

#[test]
fn a_used_block_after_an_unused_one_is_refused() {
    let settings = settings();
    let pad = block((1025, 1000), 612, slow(0), [0, 0]);
    let mut blocks = vec![pad; 5];
    blocks.push(settings);
    blocks.push(pad);
    blocks.push([0xffff; 32]);
    blocks.push(pad);
    let (_, message) = refused_at(&file(&header(&settings, "2"), &blocks));
    assert!(
        message.contains("block 8 holds data after block 7"),
        "{message}"
    );
}

#[test]
fn an_unknown_header_value_is_refused() {
    let settings = settings();
    let blocks = [
        vec![block((1025, 1000), 612, slow(0), [0, 0]); 5],
        vec![settings],
    ]
    .concat();
    for (from, to, says) in [
        ("#BinaryFormat\t1.0", "#BinaryFormat\t2.0", "#BinaryFormat"),
        ("#Axis\t2", "#Axis\t3", "#Axis"),
        ("#Num\t25", "#Num\t0", "#Num"),
    ] {
        let header = header(&settings, "2").replace(from, to);
        let (offset, message) = refused_at(&file(&header, &blocks));
        assert_eq!(offset, 0);
        assert!(message.contains(says), "{message}");
    }
}

#[test]
fn a_file_with_no_flight_after_the_settings_block_has_no_data() {
    let settings = settings();
    let blocks = [
        vec![block((1025, 1000), 612, slow(0), [0, 0]); 5],
        vec![settings],
    ]
    .concat();
    assert_eq!(
        import::read(&file(&header(&settings, "2"), &blocks)),
        Err(LogError::NoData {
            format: "Raven .FIPa"
        })
    );
}

#[test]
fn a_header_without_frames_is_not_this_format() {
    // A Raven export's text that opens like the header is read as text, never as a `.FIPa`.
    assert_eq!(import::detect(b"#Component=Raven\n"), None);
    let header = header(&settings(), "2");
    assert_eq!(import::detect(header.as_bytes()), None);
    assert_eq!(
        import::detect(format!("{header}#DONE\r\n").as_bytes()),
        None
    );
}

proptest::proptest! {
    /// Any words in the blocks, and any bytes changed in a whole file, are read or refused, never a
    /// panic; what is read keeps each channel's times rising, one a value.
    #[test]
    fn any_file_is_read_or_refused(
        words in proptest::collection::vec(proptest::prelude::any::<u16>(), 0..400),
        changes in proptest::collection::vec((0usize..4000, proptest::prelude::any::<u8>()), 0..6),
        cut in 0usize..5000,
    ) {
        let settings = settings();
        let mut blocks = vec![block((1025, 1000), 612, slow(0), [0, 0]); 5];
        blocks.push(settings);
        for chunk in words.chunks(32) {
            let mut block = [0xffffu16; 32];
            block[..chunk.len()].copy_from_slice(chunk);
            blocks.push(block);
        }
        let mut bytes = file(&header(&settings, "2"), &blocks);
        for (at, byte) in changes {
            if let Some(slot) = bytes.get_mut(at) {
                *slot = byte;
            }
        }
        bytes.truncate(bytes.len().saturating_sub(cut % 600));
        if let Ok(log) = import::read(&bytes) {
            for channel in &log.channels {
                proptest::prop_assert_eq!(channel.time_s.len(), channel.values.len());
                proptest::prop_assert!(channel.time_s.windows(2).all(|pair| pair[0] < pair[1]));
            }
        }
    }
}
