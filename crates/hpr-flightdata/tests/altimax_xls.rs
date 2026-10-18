//! An AltiMAX altimeter's Excel export, read through the `.xls` reader from an invented stream
//! (M7.1c20): a bare worksheet stream that opens with BIFF8's BOF id and holds BIFF2's cell
//! records, as the one sample export does, its table headed by the export's eleven columns.

use hpr_flightdata::error::LogError;
use hpr_flightdata::log::{FlightLog, LogFormat, Quantity};
use hpr_flightdata::{custom, import, xls};

/// A record: its id, its data's length and its data.
fn record(id: u16, data: &[u8]) -> Vec<u8> {
    let mut out = id.to_le_bytes().to_vec();
    out.extend((data.len() as u16).to_le_bytes());
    out.extend(data);
    out
}

/// A cell's row, column and three bytes of attributes, all zero.
fn place(row: u16, column: u16) -> Vec<u8> {
    let mut out = row.to_le_bytes().to_vec();
    out.extend(column.to_le_bytes());
    out.extend([0, 0, 0]);
    out
}

fn number(row: u16, column: u16, value: f64) -> Vec<u8> {
    let mut data = place(row, column);
    data.extend(value.to_le_bytes());
    record(0x0003, &data)
}

fn integer(row: u16, column: u16, value: u16) -> Vec<u8> {
    let mut data = place(row, column);
    data.extend(value.to_le_bytes());
    record(0x0002, &data)
}

fn label(row: u16, column: u16, text: &str) -> Vec<u8> {
    let mut data = place(row, column);
    data.push(text.len() as u8);
    data.extend(text.as_bytes());
    record(0x0004, &data)
}

const HEADER: [&str; 11] = [
    "TIME",
    "PRESS FILTER [hpa]",
    "HEIGHT FILTER [m]",
    "PRESS RAW [hpa]",
    "HEIGHT RAW [m]",
    "ACCEL [m/s2]",
    "SPEED [m/s]",
    "EVENT",
    "EVENT",
    "VOLT. CHANNEL",
    "VOLTAGE [V]",
];

/// A row of the export: the clock, the pressures and heights, `ACCEL`, `SPEED`, the event's code
/// and name, and the voltage's channel and value.
type Row = (
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    u16,
    &'static str,
    u16,
    f64,
);

/// The invented export: four samples 5 ms apart, the pressures in pascals under `[hpa]`, as the
/// sample's are, liftoff the second row's event, and the voltage channels cycling.
fn export() -> Vec<u8> {
    // BOF: BIFF8's id, a 6-byte body, version 0, a worksheet (0x0010), a build number.
    let mut bytes = record(0x0809, &[0x00, 0x00, 0x10, 0x00, 0xbb, 0x0d]);
    for (column, name) in HEADER.iter().enumerate() {
        bytes.extend(label(0, column as u16, name));
    }
    let rows: [Row; 4] = [
        (
            0.005, 98_000.5, 0.0, 98_000.0, 0.0, -9.22, 0.0, 0, "PAD", 1, 7.4,
        ),
        (
            0.01,
            97_990.25,
            0.85,
            97_990.0,
            0.86,
            -40.0,
            -0.2,
            3,
            "BARO-LIFTOFF",
            2,
            7.3,
        ),
        (
            0.015, 97_975.75, 2.12, 97_975.0, 2.13, -80.0, -0.6, 0, "PAD", 3, 0.1,
        ),
        (
            0.02, 97_955.0, 3.87, 97_954.0, 3.89, -80.5, -1.0, 0, "PAD", 1, 7.35,
        ),
    ];
    for (row, values) in rows.iter().enumerate() {
        let row = row as u16 + 1;
        let (t, pf, hf, pr, hr, a, v, event, name, channel, volts) = *values;
        for (column, value) in [t, pf, hf, pr, hr, a, v].into_iter().enumerate() {
            bytes.extend(number(row, column as u16, value));
        }
        bytes.extend(integer(row, 7, event));
        bytes.extend(label(row, 8, name));
        bytes.extend(integer(row, 9, channel));
        bytes.extend(number(row, 10, volts));
    }
    bytes.extend(record(0x000a, &[]));
    bytes
}

/// The export's one sheet, read.
#[cfg(test)]
fn read() -> FlightLog {
    let sheets = xls::read(&export()).unwrap();
    assert_eq!(sheets.len(), 1);
    sheets.into_iter().next().unwrap().log.unwrap()
}

#[test]
fn the_importer_names_the_reader_for_an_xls_stream() {
    let bytes = export();
    assert!(xls::recognizes(&bytes));
    match import::read(&bytes) {
        Err(LogError::Workbook { message }) => assert!(message.contains("xls::read"), "{message}"),
        other => panic!("expected a workbook refusal, got {other:?}"),
    }
}

#[test]
fn the_export_reads_by_its_columns_its_pressures_in_pascals() {
    let log = read();
    assert_eq!(log.format, LogFormat::AltimaxXls);
    assert_eq!(log.time_s, [0.005, 0.01, 0.015, 0.02]);
    // The primary track is the raw height.
    assert_eq!(log.altitude_m, [0.0, 0.86, 2.13, 3.89]);
    let quantity = |name: &str| {
        log.channels
            .iter()
            .find(|c| c.column == name)
            .map(|c| (c.quantity, c.values.clone()))
            .unwrap_or_else(|| panic!("no {name}"))
    };
    // Read in pascals, as the values are, though the column says hPa.
    assert_eq!(
        quantity("PRESS RAW [hpa]"),
        (
            Quantity::Pressure,
            vec![98_000.0, 97_990.0, 97_975.0, 97_954.0]
        )
    );
    assert_eq!(quantity("ACCEL [m/s2]").0, Quantity::Acceleration);
    assert_eq!(quantity("SPEED [m/s]").0, Quantity::Speed);
    assert_eq!(
        quantity("VOLTAGE [V] (channel 1)"),
        (Quantity::Voltage, vec![7.4, 7.35])
    );
    assert_eq!(log.vertical_speed_m_s, None);
    assert_eq!(log.events.len(), 1);
    assert_eq!(
        (log.events[0].name.as_str(), log.events[0].time_s),
        ("BARO-LIFTOFF", 0.01)
    );
    assert!(
        log.notes
            .iter()
            .any(|n| n.contains("read from an Excel `.xls` worksheet"))
    );
    assert!(
        log.notes
            .iter()
            .any(|n| n.contains("says hPa, but its values are pascals"))
    );
    insta::assert_json_snapshot!(log);
}

#[test]
fn a_later_versions_cell_is_refused_by_name() {
    // An `RK` cell, BIFF3's and later's compressed number, which the reader doesn't read.
    let mut bytes = export();
    let eof = bytes.len() - 4;
    let mut rk = place(5, 0);
    rk.extend([0, 0, 0, 0]);
    bytes.splice(eof..eof, record(0x027e, &rk));
    match xls::read(&bytes) {
        Err(LogError::Workbook { message }) => assert!(message.contains("RK"), "{message}"),
        other => panic!("expected a workbook refusal, got {other:?}"),
    }
}

/// The importer reads the export's one sheet with no sheet chosen, and the sheet chosen, and
/// refuses a sheet past the last, naming the choice (M7.2b); a table on an `.xls` sheet that no
/// reader takes reads by a column mapping, its one sheet with none named or named.
#[test]
fn a_sheet_is_read_as_chosen_and_a_table_by_a_mapping() {
    let bytes = export();
    assert_eq!(import::read_one(&bytes, None, None).unwrap(), read());
    assert_eq!(import::read_one(&bytes, Some(0), None).unwrap(), read());
    match import::read_one(&bytes, Some(1), None) {
        Err(LogError::Choice { message }) => {
            assert!(
                message.contains("has 1 sheets, numbered from 0"),
                "{message}"
            );
        }
        other => panic!("expected a choice, got {other:?}"),
    }

    let mut table = record(0x0809, &[0x00, 0x00, 0x10, 0x00, 0xbb, 0x0d]);
    table.extend(label(0, 0, "Time (s)"));
    table.extend(label(0, 1, "Height (m)"));
    for (row, (t, h)) in [(0.0, 0.0), (0.5, 4.0), (1.0, 15.0)]
        .into_iter()
        .enumerate()
    {
        table.extend(number(row as u16 + 1, 0, t));
        table.extend(number(row as u16 + 1, 1, h));
    }
    table.extend(record(0x000a, &[]));
    let (mapping, _) = custom::header_mapping(&table, None).unwrap();
    assert_eq!(mapping, custom::header_mapping(&table, Some(0)).unwrap().0);
    let log = custom::read_by_mapping(&table, &mapping, None).unwrap();
    assert_eq!(
        log,
        custom::read_by_mapping(&table, &mapping, Some(0)).unwrap()
    );
    assert_eq!(log.channels[0].time_s, [0.0, 0.5, 1.0]);
    assert_eq!(log.channels[0].values, [0.0, 4.0, 15.0]);
}
