//! One team's radio telemetry packets, read through the importer from invented files (M7.1c19):
//! the flight computer's own log, the text a receiver wrote of what it heard, with broken and
//! repeated packets, and the same packets in an RTF document a spreadsheet saved as CSV.

use hpr_flightdata::error::LogError;
use hpr_flightdata::import;
use hpr_flightdata::log::{Channel, FlightLog, LogFormat, Quantity};
use hpr_flightdata::packets;

/// The channel of a column.
#[cfg(test)]
fn channel<'a>(log: &'a FlightLog, column: &str) -> &'a Channel {
    log.channels
        .iter()
        .find(|c| c.column == column)
        .unwrap_or_else(|| panic!("no {column}"))
}

/// The notes that open as given, one each.
fn note<'a>(log: &'a FlightLog, start: &str) -> &'a str {
    let matching: Vec<&String> = log.notes.iter().filter(|n| n.starts_with(start)).collect();
    assert_eq!(matching.len(), 1, "{start}: {:#?}", log.notes);
    matching[0]
}

/// Each kind on its own clock in seconds, its stage a channel, `PP 1` in pascals, a repeated
/// packet dropped, and the `Stage` line an event at the next packet's clock.
#[test]
fn the_onboard_log_reads_each_kind_on_its_own_clock() {
    let bytes = include_bytes!("samples/radio-packets-onboard.txt");
    assert_eq!(import::detect(bytes), Some(LogFormat::RadioPacketText));
    let log = import::read(bytes).unwrap();
    assert_eq!(log.logger, "radio telemetry packets");
    assert!(log.time_s.is_empty() && log.altitude_m.is_empty());
    let columns: Vec<(&str, Quantity)> = log
        .channels
        .iter()
        .map(|c| (c.column.as_str(), c.quantity))
        .collect();
    let kept = Quantity::Unrecognized;
    assert_eq!(
        columns,
        [
            ("PP stage", kept),
            ("PP 1", Quantity::Pressure),
            ("PP 2", kept),
            ("PP 3", kept),
            ("CC stage", kept),
            ("CC 1", kept),
            ("CC 2", kept),
            ("CC 3", kept),
            ("CC 4", kept),
            ("AA stage", kept),
            ("AA 1", kept),
            ("AA 2", kept),
            ("AA 3", kept),
        ]
    );
    let pressure = channel(&log, "PP 1");
    assert_eq!(pressure.time_s, [100.0, 100.15, 100.3]);
    assert_eq!(pressure.values, [95_012.0, 95_010.0, 94_800.0]);
    assert_eq!(channel(&log, "PP stage").values, [0.0, 0.0, 22.0]);
    assert_eq!(channel(&log, "AA 1").values, [9.70, 55.20]);
    assert_eq!(channel(&log, "CC 4").time_s, [100.0, 100.31]);
    assert_eq!(log.events.len(), 1);
    assert_eq!(log.events[0].name, "Stage Two");
    assert_eq!(log.events[0].time_s, 100.3);
    assert_eq!(
        note(&log, "2 `Gain"),
        "2 `Gain[…]` lines, the flight computer's filter gains, kept out"
    );
    assert_eq!(
        note(&log, "1 packets heard"),
        "1 packets heard again dropped"
    );
    assert!(!log.notes.iter().any(|n| n.contains("broken")));
}

/// A packet heard by both paths is read once; a cut packet, a packet run into the next, a packet
/// short of its kind's values and a garbled mark are broken; the GPS packet is its own kind.
#[test]
fn the_received_text_drops_broken_and_repeated_packets() {
    let bytes = include_bytes!("samples/radio-packets-received.txt");
    assert_eq!(import::detect(bytes), Some(LogFormat::RadioPacketText));
    let log = import::read(bytes).unwrap();
    assert_eq!(channel(&log, "GG 3").values, [1234.56]);
    assert_eq!(channel(&log, "GG stage").values, [88.0]);
    // The relaying transmitter's own clock: kept as written, no time on the log's axis.
    assert_eq!(channel(&log, "GG clock").values, [100_050.0]);
    assert!(channel(&log, "GG 3").time_s[0].is_nan());
    assert_eq!(channel(&log, "PP 1").time_s, [100.0, 100.3]);
    assert_eq!(channel(&log, "AA 1").values, [9.70]);
    assert_eq!(
        note(&log, "4 broken"),
        "4 broken lines dropped, packets the radio cut or ran into the next, or with another \
         count of values than their kind's first, the first on line 4"
    );
    assert_eq!(
        note(&log, "2 packets heard"),
        "2 packets heard again dropped"
    );
    // A garbled mark is broken; a mark heard again is dropped.
    assert_eq!(log.events.len(), 1);
    assert_eq!(
        (log.events[0].name.as_str(), log.events[0].time_s),
        ("Stage Two", 100.3)
    );
    assert_eq!(
        note(&log, "1 `Stage`"),
        "1 `Stage` lines heard again dropped"
    );
}

/// The spreadsheet's commas, the markup lines, the control words before the first packet, the
/// `\` ending a line and the closing `}` taken off; a packet with its end cut is broken.
#[test]
fn the_rtf_export_reads_the_packets_inside_the_markup() {
    let bytes = include_bytes!("samples/radio-packets.rtf");
    assert_eq!(import::detect(bytes), Some(LogFormat::RadioPacketRtf));
    let log = import::read(bytes).unwrap();
    assert_eq!(log.format, LogFormat::RadioPacketRtf);
    let pressure = channel(&log, "PP 1");
    assert_eq!(pressure.time_s, [100.0, 100.3]);
    assert_eq!(pressure.values, [95_012.0, 94_800.0]);
    assert_eq!(channel(&log, "CC 4").values, [0.0, 18.5]);
    assert_eq!(log.events.len(), 1);
    assert_eq!(log.events[0].time_s, 100.3);
    assert_eq!(
        note(&log, "1 broken"),
        "1 broken lines dropped, packets the radio cut or ran into the next, or with another \
         count of values than their kind's first, the first on line 9"
    );
}

/// The refusals the module names, each by its message.
#[test]
fn files_past_the_limits_are_refused_by_name() {
    let refused = |text: &str, expected: &str| match packets::read(text) {
        Err(LogError::Syntax { message, .. }) => {
            assert!(message.contains(expected), "{message}");
        }
        other => panic!("{other:?}"),
    };
    refused("7\r\n25:T:00\r\n", "no whole packet");
    refused("255:T:00:PP:4294967296:950.00:254\r\n", "2³² ms or more");
    let values = vec!["1.0"; packets::MAX_VALUES + 1].join(":");
    refused(
        &format!("255:T:00:PP:1:{values}:254\n"),
        "more than 32 values",
    );
    let kinds: String = (0..=packets::MAX_KINDS)
        .map(|k| {
            let a = char::from(b'A' + u8::try_from(k / 26).unwrap());
            let b = char::from(b'A' + u8::try_from(k % 26).unwrap());
            format!("255:T:00:{a}{b}:1:1.0:254\n")
        })
        .collect();
    refused(&kinds, "more than 32 kinds");
    // The largest clock is read.
    assert!(packets::read("255:T:00:PP:4294967295:950.00:254\n").is_ok());
}

/// Text that only looks like packets isn't recognized: a lone number with no packet after it, or
/// a packet whose numbers aren't written as the packets write them.
#[test]
fn text_without_a_whole_packet_is_not_recognized() {
    assert_ne!(
        import::detect(b"7\r\nhello\r\n"),
        Some(LogFormat::RadioPacketText)
    );
    assert_ne!(
        import::detect(b"255:T:00:PP:100:1e3:254\n"),
        Some(LogFormat::RadioPacketText)
    );
    assert_ne!(
        import::detect(b"255:T:0:PP:100:950.00:254\n"),
        Some(LogFormat::RadioPacketText)
    );
    // A number too long to be finite makes its line broken, not an infinite value.
    let long = "9".repeat(400);
    let text = format!("255:T:00:PP:1:950.00:254\n255:T:00:PP:2:{long}:254\n");
    let log = packets::read(&text).unwrap();
    assert_eq!(log.channels[1].values, [95_000.0]);
    assert!(log.notes.iter().any(|n| n.starts_with("1 broken")));
}
