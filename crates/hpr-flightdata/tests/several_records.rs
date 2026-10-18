//! Files that hold several records, read one at a time (M7.2b): a workbook's sheets by
//! `import::read_one`, and a Raven FIP export that lost its header line, pasted into a workbook
//! below none, by `custom::read_by_fip_header`. Each file is built here, and its committed copy
//! under `validation/fixtures/logs/several/`, which the command line's and Python's tests read,
//! is held to it byte for byte; `HPR_WRITE_SEVERAL_RECORDS=1` writes them again. An Entacore AIM
//! dump of two recordings is built and held the same way in `tests/entacore_raw.rs`.

use std::io::{Cursor, Write};
use std::path::PathBuf;

use hpr_flightdata::custom::{self, FipHeader};
use hpr_flightdata::error::LogError;
use hpr_flightdata::import;
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipWriter};

const MAIN: &str = "http://schemas.openxmlformats.org/spreadsheetml/2006/main";
const R: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
const PACKAGE: &str = "http://schemas.openxmlformats.org/package/2006/relationships";

/// Holds the committed copy `committed`, of the file `name`, to `bytes`, or writes it with
/// `HPR_WRITE_SEVERAL_RECORDS=1`; the test then runs again to compare the new copy.
#[cfg(test)]
#[allow(
    clippy::disallowed_methods,
    reason = "a test that writes its own fixture when asked, as `cargo xtask` outputs are"
)]
fn held(name: &str, committed: &[u8], bytes: &[u8]) {
    if std::env::var_os("HPR_WRITE_SEVERAL_RECORDS").is_some() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../validation/fixtures/logs/several")
            .join(name);
        std::fs::write(path, bytes).unwrap();
    }
    assert!(
        committed == bytes,
        "{name} is stale: run this test with `HPR_WRITE_SEVERAL_RECORDS=1`, then again"
    );
}

/// The committed copies.
const TWO_FLIGHTS: &[u8] =
    include_bytes!("../../../validation/fixtures/logs/several/two-flights.xlsx");
const FIP_SHEET: &[u8] =
    include_bytes!("../../../validation/fixtures/logs/several/fip-below-no-header.xlsx");
const FIP_HEADER_FILE: &[u8] =
    include_bytes!("../../../validation/fixtures/logs/several/fip-below-no-header.header.json");

/// A column's letters, from 0.
#[cfg(test)]
fn column(mut index: usize) -> String {
    let mut letters = Vec::new();
    loop {
        letters.push(b'A' + (index % 26) as u8);
        if index < 26 {
            break;
        }
        index = index / 26 - 1;
    }
    letters.iter().rev().map(|&b| char::from(b)).collect()
}

/// A worksheet's `sheetData` of rows of cells: a number as a value, any other text as an inline
/// string.
#[cfg(test)]
fn sheet_data(rows: &[Vec<String>]) -> String {
    let mut data = String::new();
    for (r, row) in rows.iter().enumerate() {
        data += &format!(r#"<row r="{}">"#, r + 1);
        for (c, cell) in row.iter().enumerate() {
            let at = format!("{}{}", column(c), r + 1);
            data += &if cell.parse::<f64>().is_ok() {
                format!(r#"<c r="{at}"><v>{cell}</v></c>"#)
            } else {
                format!(r#"<c r="{at}" t="inlineStr"><is><t>{cell}</t></is></c>"#)
            };
        }
        data += "</row>";
    }
    data
}

/// An `.xlsx` workbook of the named sheets, its parts stored uncompressed, at one date, with one
/// set of permissions and one host system, so that its bytes are the same on every run and every
/// system.
#[cfg(test)]
fn workbook(sheets: &[(&str, String)]) -> Vec<u8> {
    let mut list = String::new();
    let mut rels = String::new();
    let mut parts: Vec<(String, String)> = Vec::new();
    for (n, (name, data)) in sheets.iter().enumerate() {
        list += &format!(r#"<sheet name="{name}" sheetId="{}" r:id="s{n}"/>"#, n + 1);
        rels += &format!(
            r#"<Relationship Id="s{n}" Type="{R}/worksheet" Target="worksheets/sheet{n}.xml"/>"#
        );
        parts.push((
            format!("xl/worksheets/sheet{n}.xml"),
            format!(r#"<worksheet xmlns="{MAIN}"><sheetData>{data}</sheetData></worksheet>"#),
        ));
    }
    parts.push((
        "xl/workbook.xml".to_owned(),
        format!(r#"<workbook xmlns="{MAIN}" xmlns:r="{R}"><sheets>{list}</sheets></workbook>"#),
    ));
    parts.push((
        "xl/_rels/workbook.xml.rels".to_owned(),
        format!(r#"<Relationships xmlns="{PACKAGE}">{rels}</Relationships>"#),
    ));
    parts.push((
        "_rels/.rels".to_owned(),
        format!(
            r#"<Relationships xmlns="{PACKAGE}"><Relationship Id="r1" Type="{R}/officeDocument" Target="xl/workbook.xml"/></Relationships>"#
        ),
    ));
    let mut zip = ZipWriter::new(Cursor::new(Vec::new()));
    // Dated 1980-01-01, zip's earliest, whether or not another crate turns on its clock, and made
    // on Unix, which zip would otherwise write as DOS when the test runs on Windows.
    let options = SimpleFileOptions::default()
        .compression_method(CompressionMethod::Stored)
        .last_modified_time(zip::DateTime::default())
        .unix_permissions(0o644)
        .system(zip::System::Unix);
    for (name, text) in &parts {
        zip.start_file(name.as_str(), options).unwrap();
        zip.write_all(text.as_bytes()).unwrap();
    }
    zip.finish().unwrap().into_inner()
}

/// An invented flight's height above the pad, m, at `t` s: on the pad for 1 s, 3 s at
/// `boost` m/s², a vacuum coast, then down at 10 m/s.
#[cfg(test)]
fn height(boost: f64, t: f64) -> f64 {
    const G: f64 = 9.806_65;
    let s = t - 1.0;
    if s <= 0.0 {
        return 0.0;
    }
    if s <= 3.0 {
        return 0.5 * boost * s * s;
    }
    let (burnout_m, burnout_m_s) = (4.5 * boost, 3.0 * boost);
    let coast_s = burnout_m_s / G;
    let c = s - 3.0;
    if c <= coast_s {
        return burnout_m + burnout_m_s * c - 0.5 * G * c * c;
    }
    let apogee_m = burnout_m + burnout_m_s * coast_s - 0.5 * G * coast_s * coast_s;
    (apogee_m - 10.0 * (c - coast_s)).max(0.0)
}

/// The flight's samples, every 0.2 s for `end_s`.
#[cfg(test)]
fn samples(boost: f64, end_s: f64) -> Vec<(f64, f64)> {
    (0..=(end_s * 5.0).round() as u32)
        .map(|k| {
            let t = f64::from(k) / 5.0;
            (t, (height(boost, t) * 100.0).round() / 100.0)
        })
        .collect()
}

/// A workbook of two flights' tables, each read by its header, and a sheet of notes that reads
/// as no log.
#[cfg(test)]
fn two_flights() -> Vec<u8> {
    let table = |boost: f64| {
        let mut rows = vec![vec!["Time (s)".to_owned(), "Baro Alt AGL (m)".to_owned()]];
        rows.extend(
            samples(boost, 40.0)
                .iter()
                .map(|(t, h)| vec![format!("{t:.1}"), format!("{h:.2}")]),
        );
        sheet_data(&rows)
    };
    let notes = sheet_data(&[vec!["Flown on an invented day".to_owned()]]);
    workbook(&[
        ("First", table(20.0)),
        ("Notes", notes),
        ("Second", table(30.0)),
    ])
}

/// The header a Raven FIP export of three channels writes.
const FIP_HEADER: &str = "Time@Axial Accel (Gs),Axial Accel (Gs),bILBA,Time@Baro (Atm),Baro (Atm),\
                          bILBA,Time@[Altitude (Baro-Ft-AGL)],[Altitude (Baro-Ft-AGL)],bILBA";

/// A FIP export of the flight pasted into a workbook below no header: each channel's clock,
/// value and status, the altitude in feet.
#[cfg(test)]
fn fip_sheet() -> Vec<u8> {
    let rows: Vec<Vec<String>> = samples(20.0, 40.0)
        .iter()
        .map(|(t, h)| {
            let feet = h / 0.3048;
            let atm = (1.0 - 2.255_77e-5 * h).powf(5.255_88);
            vec![
                format!("{t:.2}"),
                "1.00".to_owned(),
                "b0000".to_owned(),
                format!("{t:.2}"),
                format!("{atm:.5}"),
                "b0000".to_owned(),
                format!("{t:.2}"),
                format!("{feet:.1}"),
                "b0000".to_owned(),
            ]
        })
        .collect();
    workbook(&[("Raven", sheet_data(&rows))])
}

/// The FIP header file for [`fip_sheet`].
#[cfg(test)]
fn fip_header() -> String {
    format!("{{\n  \"header\": \"{FIP_HEADER}\"\n}}\n")
}

#[cfg(test)]
fn choice(error: LogError) -> String {
    match error {
        LogError::Choice { message } => message,
        other => panic!("not a choice: {other}"),
    }
}

/// A workbook of two flights and a sheet of notes: with no sheet chosen it is refused, the two
/// that read named; each chosen sheet is read as its own file; a sheet past the last, or a
/// recording, is refused; the notes are refused as their own reader refuses them.
#[test]
fn a_workbooks_sheet_is_read_as_chosen() {
    let bytes = two_flights();
    held("two-flights.xlsx", TWO_FLIGHTS, &bytes);

    let message = choice(import::read_one(&bytes, None, None).unwrap_err());
    assert!(
        message.contains("sheets 0 (First), 2 (Second)"),
        "{message}"
    );
    let top = |sheet: usize| {
        let log = import::read_one(&bytes, Some(sheet), None).unwrap();
        let track = log
            .channels
            .iter()
            .find(|c| c.column == "Baro Alt AGL (m)")
            .unwrap();
        track
            .values
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max)
    };
    // Each sheet holds its own flight: 3 s at 20 m/s² reaches 90 m at 60 m/s and coasts 183.5 m
    // more; at 30 m/s², 135 m and 90 m/s, then 413.0 m; each sampled within 0.2 m of its top.
    assert!((top(0) - 273.5).abs() < 0.2, "{}", top(0));
    assert!((top(2) - 548.0).abs() < 0.2, "{}", top(2));
    assert!(!matches!(
        import::read_one(&bytes, Some(1), None),
        Ok(_) | Err(LogError::Choice { .. })
    ));
    let past = choice(import::read_one(&bytes, Some(3), None).unwrap_err());
    assert!(past.contains("has 3 sheets"), "{past}");
    choice(import::read_one(&bytes, None, Some(0)).unwrap_err());

    // A workbook of one sheet that reads is read with no choice.
    let one = workbook(&[(
        "Only",
        sheet_data(&[
            vec!["Time (s)".to_owned(), "Baro Alt AGL (m)".to_owned()],
            vec!["0".to_owned(), "0".to_owned()],
            vec!["0.1".to_owned(), "0.5".to_owned()],
        ]),
    )]);
    let only = import::read_one(&one, None, None).unwrap();
    assert_eq!(only.channels[0].values, [0.0, 0.5]);

    // By a column mapping, a workbook's one sheet is read with none named; of several, one must
    // be named.
    let (mapping, _) = custom::header_mapping(&one, None).unwrap();
    assert_eq!(
        custom::read_by_mapping(&one, &mapping, None).unwrap(),
        custom::read_by_mapping(&one, &mapping, Some(0)).unwrap()
    );
    let several = custom::read_by_mapping(&bytes, &mapping, None).unwrap_err();
    assert!(
        several.to_string().contains(
            r#"its sheets 0 "First", 1 "Notes", 2 "Second": name the sheet the table is on"#
        ),
        "{several}"
    );
}

/// A workbook of many sheets with long names: the refusal lists the first 20, each name cut to
/// 60 characters, and counts the rest, whether its sheets read by their own reader or by a
/// mapping.
#[test]
fn a_refusal_lists_a_few_sheets_shortly() {
    let table = sheet_data(&[
        vec!["Time (s)".to_owned(), "Baro Alt AGL (m)".to_owned()],
        vec!["0".to_owned(), "0".to_owned()],
        vec!["0.1".to_owned(), "0.5".to_owned()],
    ]);
    let long = "🚀".repeat(70);
    let names: Vec<String> = (0..25).map(|n| format!("{n:02}{long}")).collect();
    let sheets: Vec<(&str, String)> = names.iter().map(|n| (n.as_str(), table.clone())).collect();
    let bytes = workbook(&sheets);
    let by_reader = choice(import::read_one(&bytes, None, None).unwrap_err());
    let (mapping, _) = custom::header_mapping(&bytes, Some(0)).unwrap();
    let by_mapping = custom::read_by_mapping(&bytes, &mapping, None)
        .unwrap_err()
        .to_string();
    for message in [by_reader, by_mapping] {
        assert!(message.contains("and 5 more"), "{message}");
        assert!(
            message.contains(&format!("19{}…", "🚀".repeat(58))),
            "{message}"
        );
        assert!(
            !message.contains("20🚀") && !message.contains(&"🚀".repeat(59)),
            "{message}"
        );
    }
}

/// A file of one record takes no choice of sheet or recording.
#[test]
fn a_file_of_one_record_takes_no_choice() {
    let csv = b"Time (s),Baro Alt AGL (m)\n0,0\n0.1,0.5\n";
    assert_eq!(
        import::read_one(csv, None, None).unwrap(),
        import::read(csv).unwrap()
    );
    choice(import::read_one(csv, Some(0), None).unwrap_err());
    choice(import::read_one(csv, None, Some(0)).unwrap_err());
}

/// A FIP export below no header reads under the header stated for it, from its workbook's one
/// sheet, its altitude in feet as the header says; with no header, the workbook's sheet reads as
/// no log.
#[test]
fn a_fip_export_reads_under_its_stated_header() {
    let bytes = fip_sheet();
    held("fip-below-no-header.xlsx", FIP_SHEET, &bytes);
    held(
        "fip-below-no-header.header.json",
        FIP_HEADER_FILE,
        fip_header().as_bytes(),
    );

    let stated: FipHeader = custom::fip_header_from_json(&fip_header()).unwrap();
    assert!(stated.pasted.is_empty());
    let log = custom::read_by_fip_header(&bytes, &stated, None).unwrap();
    let top = log
        .altitude_m
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max);
    assert!((top - 273.5).abs() < 0.2, "{top}");
    assert!(import::read_one(&bytes, None, None).is_err());
    choice(custom::read_by_fip_header(&bytes, &stated, Some(1)).unwrap_err());
    choice(custom::read_by_fip_header(b"0,1,b0000\n", &stated, Some(0)).unwrap_err());
    // A key the file's schema doesn't name is refused, naming it.
    let typo = custom::fip_header_from_json(r#"{"header": "a", "pastd": [1]}"#).unwrap_err();
    assert!(typo.to_string().contains("pastd"), "{typo}");
    // A header line or pasted columns the rows refuse are the FIP header's fault, said so.
    for (header, pasted, said) in [
        ("nonsense", vec![], "FIP header: the header stated"),
        (
            FIP_HEADER,
            vec![0],
            "FIP header: the pasted columns are numbered from 1",
        ),
    ] {
        let stated = FipHeader {
            header: header.to_owned(),
            pasted,
        };
        match custom::read_by_fip_header(&bytes, &stated, None) {
            Err(error @ LogError::FipHeader { .. }) => {
                assert!(error.to_string().starts_with(said), "{error}");
            }
            other => panic!("{other:?}"),
        }
    }
}

/// A workbook that fails is refused for its own fault, by the importer and by a mapping alike,
/// and a workbook of one sheet that doesn't read is refused as that sheet is; a zip archive that
/// isn't a workbook is refused as an archive.
#[test]
fn a_workbooks_own_refusal_is_kept() {
    let broken = workbook(&[("Broken", "<row r=\"1\">".to_owned())]);
    assert!(matches!(
        import::read_one(&broken, None, None),
        Err(LogError::Workbook { .. })
    ));
    let (mapping, _) = custom::header_mapping(&two_flights(), Some(0)).unwrap();
    assert!(matches!(
        custom::read_by_mapping(&broken, &mapping, None),
        Err(LogError::Workbook { .. })
    ));

    let notes = workbook(&[("Notes", sheet_data(&[vec!["An invented day".to_owned()]]))]);
    let refused = import::read_one(&notes, None, None).unwrap_err();
    assert!(!matches!(refused, LogError::Choice { .. }), "{refused}");
    assert_eq!(
        refused.to_string(),
        import::read_one(&notes, Some(0), None)
            .unwrap_err()
            .to_string()
    );

    let mut zip = ZipWriter::new(Cursor::new(Vec::new()));
    let options = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);
    zip.start_file("flight.csv", options).unwrap();
    zip.write_all(b"Time (s),Baro Alt AGL (m)\n0,0\n").unwrap();
    let archive = zip.finish().unwrap().into_inner();
    assert!(matches!(
        import::read_one(&archive, None, None),
        Err(LogError::Archive { .. })
    ));
}
