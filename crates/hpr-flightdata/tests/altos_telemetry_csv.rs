//! AltOS's radio telemetry decoded to a table, read from an invented TeleMega's packets by the
//! importer and pinned whole by a snapshot (M7.1c21, ADR-274).

use hpr_flightdata::import;
use hpr_flightdata::log::{LogFormat, Quantity};

#[test]
fn each_column_reads_by_its_name() {
    let log = import::read(include_bytes!("samples/altos-telemetry-table.csv")).unwrap();
    assert_eq!(log.format, LogFormat::AltosTelemetryCsv);
    assert_eq!(log.logger, "AltOS");
    assert_eq!(log.serial_number.as_deref(), Some("2047"));
    // 24 packets, one failing its CRC.
    assert_eq!(log.time_s.len(), 23);
    assert!(
        log.notes
            .iter()
            .any(|n| n.starts_with("1 row whose `crc` is FALSE")),
        "{:#?}",
        log.notes
    );
    assert_eq!(log.time_s[0], 402.0);
    assert_eq!(log.altitude_m[22], 118.0);
    let column = |name: &str| log.channels.iter().find(|c| c.column == name).unwrap();
    assert_eq!(column("height").quantity, Quantity::AltitudeAboveGround);
    assert_eq!(column("speed").values[4], 8.0);
    assert_eq!(column("ground_pres").quantity, Quantity::Pressure);
    // `sense` is four voltages, one channel each.
    assert_eq!(column("sense 3").quantity, Quantity::Voltage);
    assert_eq!(column("sense 4").values[0], 3.90);
    assert_eq!(log.battery_v.as_ref().unwrap()[0], 4.08);
    let names: Vec<&str> = log.events.iter().map(|e| e.name.as_str()).collect();
    assert_eq!(names, ["pad", "boost", "coast"]);
    insta::assert_json_snapshot!(log);
}
