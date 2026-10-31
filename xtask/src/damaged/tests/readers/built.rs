//! Samples built where no committed file serves: each is the smallest writer or literal of its
//! format's own tests, every value invented there, or a committed file cut or unpacked.

use std::io::{Cursor, Read, Write};

use hpr_format::container::{self, Entry, Hprz};
use hpr_net::Cache;
use serde_json::Value;

use super::super::root;
use super::CACHED_URL;

fn committed(path: &str) -> Vec<u8> {
    std::fs::read(root().join(path)).unwrap_or_else(|error| panic!("{path}: {error}"))
}

/// A `.FIPa` of a flight, as `tests/raven_fipa.rs` lays it out: five pad blocks, the settings, and
/// three blocks of flight, in frames of four with their checksums.
pub(super) fn raven_fipa() -> Vec<u8> {
    let mut settings = [0_u16; 32];
    for (at, word) in [
        (0, 5),
        (1, 1000),
        (24, 8200),
        (25, 1000),
        (27, 2000),
        (28, 512),
    ] {
        settings[at] = word;
    }
    let block = |axial: (u16, u16), lateral: u16, velocity: i16, last: [u16; 2]| {
        let slow = [
            0,
            5000,
            5100,
            0,
            (-20_i16).cast_unsigned(),
            0x1234,
            8000,
            7000,
            23_500,
            velocity.cast_unsigned(),
        ];
        let mut words = [0_u16; 32];
        for (k, word) in slow.iter().enumerate() {
            words[3 * k] = (axial.0 << 5) | (lateral >> 5);
            words[3 * k + 1] = ((lateral & 0x1f) << 11) | axial.1;
            words[3 * k + 2] = *word;
        }
        words[30] = last[0];
        words[31] = last[1];
        words
    };
    let mut blocks = vec![block((1025, 1000), 612, 0, [0xabcd, 7]); 5];
    blocks.push(settings);
    blocks.push(block((1100, 1025), 612, 1000, [1, 2]));
    blocks.push(block((900, 1000), 512, -10, [3, 4]));
    blocks.push(block((1000, 1000), 412, -10, [5, 6]));
    while !blocks.len().is_multiple_of(4) {
        blocks.push([0xffff; 32]);
    }
    let listed: Vec<String> = settings[..29].iter().map(ToString::to_string).collect();
    let mut bytes = format!(
        "#Component=Raven\r\n#Style=4-Output\r\n#DateTime=1/2/2020 3:04:05 PM\r\n#FlightNum=7\r\n\
         #FlightState=#7\tO\t{}\r\n#BinaryFormat\t1.0\r\n#R 3.0\r\n#D Jan 1 2015 00:00:00\r\n\
         #Axis\t2\r\n#Num\t25\r\n#den\t20\r\n",
        listed.join("\t")
    )
    .into_bytes();
    for frame in blocks.chunks(4) {
        let data: Vec<u8> = frame
            .iter()
            .flat_map(|block| block.iter().flat_map(|word| word.to_be_bytes()))
            .collect();
        bytes.push(b':');
        bytes.extend(&data);
        bytes.push(data.iter().fold(0_u8, |sum, b| sum.wrapping_add(*b)));
    }
    bytes.extend(b"#DONE\r\n");
    bytes
}

/// An `.rff` of three seconds, as `tests/rrc3_rff.rs` lays it out.
pub(super) fn rrc3_rff() -> Vec<u8> {
    const CAPACITY: usize = 256;
    const ID: i32 = 2;
    let mut words = vec![0x6f00_u16, 0x7f00, 0x0400];
    for n in 0..60_u16 {
        words.push(10_000 - n);
        if n == 29 {
            words.push(0x7918);
        }
        if n % 20 == 19 {
            words.extend([0x6e00 + n, 0x7e00]);
        }
    }
    words.push(0xffff);
    let mut bytes = vec![0_u8];
    for value in [1_i32, -1, 1, 0] {
        bytes.extend(value.to_le_bytes());
    }
    bytes.extend(b"\x0c\x02\x00\x00\x00mDACS.AnalysisSupport+FlightRecord");
    let size = i32::try_from(words.len()).expect("a few words");
    bytes.push(0x09);
    bytes.extend(ID.to_le_bytes());
    bytes.extend(size.to_le_bytes());
    bytes.extend(size.to_le_bytes());
    bytes.push(0x0f);
    bytes.extend(ID.to_le_bytes());
    bytes.extend(
        i32::try_from(CAPACITY)
            .expect("a small capacity")
            .to_le_bytes(),
    );
    bytes.push(7);
    for slot in 0..CAPACITY {
        bytes.extend(words.get(slot).copied().unwrap_or(0).to_le_bytes());
    }
    bytes.push(0x0b);
    bytes
}

/// An `.ALS` of twelve records, as `tests/altimax_als.rs` lays it out.
pub(super) fn altimax_als() -> Vec<u8> {
    const RECORDS: [(u8, u32, i16, bool); 12] = [
        (0, 98_000, -100, false),
        (0, 98_000, -100, false),
        (3, 97_990, -800, false),
        (0, 97_950, -790, false),
        (0, 97_880, -300, false),
        (0, 97_800, 20, false),
        (0, 97_740, 5, false),
        (0, 97_700, 2, false),
        (9, 97_690, 0, true),
        (42, 97_700, -100, true),
        (0, 97_720, -100, true),
        (14, 97_740, -100, true),
    ];
    let mut bytes = vec![0_u8; 32];
    bytes[4..8].copy_from_slice(&98_000_u32.to_le_bytes());
    bytes[8..12].copy_from_slice(&97_690_u32.to_le_bytes());
    bytes[26..28].copy_from_slice(&11_u16.to_le_bytes());
    for (index, (event, raw_pa, counts, slow)) in (0_u8..).zip(RECORDS) {
        bytes.push((index % 5 + 1) << 3 | if slow { 0b101 } else { 0b001 });
        bytes.push(event);
        bytes.extend((f64::from(raw_pa) as f32 + 0.5).to_le_bytes());
        bytes.extend(raw_pa.to_le_bytes());
        bytes.extend(counts.to_le_bytes());
        bytes.extend((500 + u16::from(index)).to_le_bytes());
        bytes.extend([0; 8]);
    }
    bytes
}

/// The `.xtra` of the flight of `tests/entacore_raw.rs`: each record's ticks, class and
/// little-endian payload.
pub(super) fn entacore_xtra() -> Vec<u8> {
    let f32s =
        |values: &[f32]| -> Vec<u8> { values.iter().flat_map(|v| v.to_le_bytes()).collect() };
    let status = |state: u32, on: [u8; 4], continuity: [u8; 4]| {
        let mut bytes = state.to_le_bytes().to_vec();
        bytes.extend(4_u32.to_le_bytes());
        bytes.extend(on);
        bytes.extend(4_u32.to_le_bytes());
        bytes.extend(continuity);
        bytes.extend(4_u32.to_le_bytes());
        bytes.extend([0; 4]);
        bytes
    };
    let degrees = f64::from(1e-7_f32);
    let mut gps = (297_000_000.0 * degrees).to_le_bytes().to_vec();
    gps.extend((-950_000_000.0 * degrees).to_le_bytes());
    gps.extend((150_000.0_f32 * 0.001).to_le_bytes());
    gps.extend(9_u32.to_le_bytes());
    gps.push(1);
    let mut gps_time = 123_456_000_u32.to_le_bytes().to_vec();
    gps_time.extend(2_400_u16.to_le_bytes());
    gps_time.extend([1, 0]);
    gps_time.extend(18_u32.to_le_bytes());
    let battery = 13.299_999_237_060_547_f64 / 4_096.0;
    let line = 3.3_f64 / 16_384.0;
    let records: Vec<(u32, i16, Vec<u8>)> = vec![
        (10, 4, f32s(&[97_700.0])),
        (5, 2, f32s(&[1.0])),
        (5, 5, f32s(&[(2_048.0 * battery) as f32])),
        (0, 7, f32s(&[2_150.0_f32 * 0.01])),
        (10, 13, status(0, [0; 4], [1; 4])),
        (300, 2, f32s(&[7.0])),
        (0, 3, f32s(&[3.0, -2.0])),
        (0, 13, status(1, [0; 4], [1; 4])),
        (
            130,
            17,
            [vec![0, 0], f32s(&[4_096.0_f32 * line as f32])].concat(),
        ),
        (0, 13, status(3, [0, 0, 1, 0], [1; 4])),
        (1, 10, gps),
        (1, 14, gps_time),
        (1, 4, f32s(&[97_500.0])),
        (1, 15, f32s(&[100.0 * 0.07, -200.0 * 0.07, 300.0 * 0.07])),
        (1, 16, f32s(&[500.0 * 1e-6, -250.0 * 1e-6, 1_000.0 * 1e-6])),
        (1, 21, f32s(&[0.5, -0.25, 0.0, 32_767.0 / 32_768.0])),
        (1, 6, f32s(&[(2_047.0 * battery) as f32])),
        (
            1,
            18,
            [vec![0, 0], f32s(&[2_048.0_f32 * line as f32])].concat(),
        ),
    ];
    let mut bytes = b"\x16\0\0\0serialization::archive\x10\0\x04\x04\x04\x08\x01\0\0\0".to_vec();
    bytes.extend([0, 3, 0, 0, 0, 0, 0, 0, 0, 0]);
    bytes.extend((records.len() as u64).to_le_bytes());
    let mut seen = Vec::new();
    let mut time = 0.0_f32;
    for (index, (mut ticks, class, payload)) in (0_u32..).zip(records) {
        while ticks > 0 {
            let step = ticks.min(126);
            time += step as f32 * 0.001;
            ticks -= step;
        }
        bytes.extend(class.to_le_bytes());
        if !seen.contains(&class) {
            bytes.push(1);
            let version: u32 = match class {
                2 | 3 => 3,
                10 => 4,
                14 => 2,
                4..=7 | 13 => 1,
                _ => 0,
            };
            bytes.extend(version.to_le_bytes());
            seen.push(class);
        }
        bytes.extend((2 * index).to_le_bytes());
        if index == 0 {
            bytes.extend([1, 1, 0, 0, 0]);
        }
        bytes.extend((2 * index + 1).to_le_bytes());
        bytes.extend(3.3_f32.to_le_bytes());
        bytes.push(0);
        bytes.extend(time.to_le_bytes());
        bytes.extend(payload);
    }
    bytes
}

/// A worksheet stream of `tests/altimax_xls.rs`: BIFF8's BOF id over BIFF2's cell records.
pub(super) fn xls() -> Vec<u8> {
    fn record(id: u16, data: &[u8]) -> Vec<u8> {
        let mut out = id.to_le_bytes().to_vec();
        out.extend(
            u16::try_from(data.len())
                .expect("a short record")
                .to_le_bytes(),
        );
        out.extend(data);
        out
    }
    fn place(row: u16, column: u16) -> Vec<u8> {
        let mut out = row.to_le_bytes().to_vec();
        out.extend(column.to_le_bytes());
        out.extend([0, 0, 0]);
        out
    }
    fn label(row: u16, column: u16, text: &str) -> Vec<u8> {
        let mut data = place(row, column);
        data.push(u8::try_from(text.len()).expect("a short label"));
        data.extend(text.as_bytes());
        record(0x0004, &data)
    }
    let mut bytes = record(0x0809, &[0x00, 0x00, 0x10, 0x00, 0xbb, 0x0d]);
    bytes.extend(label(0, 0, "Time (s)"));
    bytes.extend(label(0, 1, "Height (m)"));
    for (row, (t, h)) in (1_u16..).zip([(0.0_f64, 0.0_f64), (0.5, 4.0), (1.0, 15.0), (1.5, 30.0)]) {
        for (column, value) in (0_u16..).zip([t, h]) {
            let mut data = place(row, column);
            data.extend(value.to_le_bytes());
            bytes.extend(record(0x0003, &data));
        }
    }
    let mut data = place(5, 0);
    data.extend(2_u16.to_le_bytes());
    bytes.extend(record(0x0002, &data));
    let mut data = place(5, 1);
    data.extend(45_u16.to_le_bytes());
    bytes.extend(record(0x0002, &data));
    bytes.extend(record(0x000a, &[]));
    bytes
}

/// An AltiMAX export written out as text, as `altimax.rs`'s tests write it.
pub(super) fn altimax_text() -> Vec<u8> {
    b"TIME,PRESS FILTER [hpa],HEIGHT FILTER [m],PRESS RAW [hpa],HEIGHT RAW [m],\
      ACCEL [m/s2],SPEED [m/s],EVENT,EVENT,VOLT. CHANNEL,VOLTAGE [V]\n\
      0.005,98000,0,98000,0,-9.8,0,0,PAD,1,7.4\n\
      0.010,97990,0.9,97990,0.9,-9.8,0,3,BARO-LIFTOFF,2,7.3\n\
      0.015,97975,2.2,97975,2.2,-9.8,0,0,PAD,1,7.39\n\
      0.040,97950,4.3,97950,4.3,-9.8,0,9,APOGEE PRESSURE,2,7.29\n"
        .to_vec()
}

/// A GPS tracker's summary, the 15 invented rows of `tests/featherweight_summary.rs`.
pub(super) fn featherweight_summary() -> Vec<u8> {
    b"\r\nRocket Name,Demo tracker\r\nLaunch time UTC,2026-06-01 15:00:00 UTC\r\n\
      Launch time local time zone,2026-06-01 10:00:00\r\nMax Altitude,2000.00 feet\r\n\
      Pad altitude,1500.0 ft\r\nPad Latitude,35.12345 deg\r\nPad Longtitude,-100.54321 deg\r\n\
      Distance at launch,10 ft\r\nDistance at apogee,900 ft\r\nDistance at landing,2500 ft\r\n\
      Max vertical velocity,450 ft/sec\r\nMax horizontal velocity,120 ft/sec\r\n\
      Horizontal velocity at apogee,60 ft/sec\r\nHorizontal velocity at laniding,5 ft/sec\r\n\
      Vertical velocity at landing,-20 ft/sec\r\n"
        .to_vec()
}

/// The rows of the committed FIP export, whose header the row states.
pub(super) fn fip_rows() -> Vec<u8> {
    let whole = committed("crates/hpr-flightdata/tests/samples/raven-fip-plain.csv");
    let start = whole
        .iter()
        .position(|b| *b == b'\n')
        .map_or(0, |at| at + 1);
    whole[start..].to_vec()
}

/// The design inside a committed `.ork`, unpacked.
pub(super) fn ork_design() -> Vec<u8> {
    let bytes = committed("validation/fixtures/ork/loft-demo/demo-dual-deploy.ork");
    hpr_io::ork::container::unpack(&bytes)
        .expect("a committed design")
        .value
        .design
        .into_bytes()
}

/// The first sheet's XML of the committed workbook.
pub(super) fn workbook_sheet() -> Vec<u8> {
    let mut zip = zip::ZipArchive::new(Cursor::new(super::TWO_FLIGHTS)).expect("a committed zip");
    let mut sheet = Vec::new();
    zip.by_name(super::TWO_FLIGHTS_SHEET)
        .expect("a committed sheet")
        .read_to_end(&mut sheet)
        .expect("a stored sheet");
    sheet
}

/// The committed workbook with `sheet` for its first sheet, its zip's checksums good.
pub(super) fn workbook_with(sheet: &[u8]) -> Vec<u8> {
    let mut zip = zip::ZipArchive::new(Cursor::new(super::TWO_FLIGHTS)).expect("a committed zip");
    let mut out = zip::ZipWriter::new(Cursor::new(Vec::new()));
    let options =
        zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
    for index in 0..zip.len() {
        let mut entry = zip.by_index(index).expect("a committed entry");
        let name = entry.name().expect("a committed name").into_owned();
        let mut bytes = Vec::new();
        entry.read_to_end(&mut bytes).expect("a stored entry");
        out.start_file(name.as_str(), options).expect("an entry");
        out.write_all(if name == super::TWO_FLIGHTS_SHEET {
            sheet
        } else {
            &bytes
        })
        .expect("written to memory");
    }
    out.finish().expect("written to memory").into_inner()
}

/// The words of a `.ork`'s elements, with the blank space a file may put around them.
pub(super) fn deploy_event() -> Vec<u8> {
    format!("{:64}altitude{:8}", "", "").into_bytes()
}

pub(super) fn separation_event() -> Vec<u8> {
    format!("{:64}upperignition{:8}", "", "").into_bytes()
}

pub(super) fn ignition_event() -> Vec<u8> {
    format!("{:64}ejectioncharge{:8}", "", "").into_bytes()
}

pub(super) fn dimension() -> Vec<u8> {
    format!("{:64}auto 0.025{:8}", "", "").into_bytes()
}

pub(super) fn schema_version() -> Vec<u8> {
    format!("1.{}9", "0".repeat(70)).into_bytes()
}

pub(super) fn delays() -> Vec<u8> {
    b"3-4-5-6-7-8-9-10-11-12-13-14-15-16-17-18-19-20-21-22-23-24-25-26-27-28-29-30-P".to_vec()
}

/// A `.hprz` of the committed `.hpr` and one attachment.
pub(super) fn hprz() -> Vec<u8> {
    let text = committed("crates/hpr-format/fixtures/embedded-curve-0.1.hpr");
    let text = String::from_utf8(text).expect("a committed document");
    let design = hpr_format::read_json(&text)
        .expect("a committed document")
        .value;
    let attachment = Entry::new("notes.txt", b"Flown on an invented day\n".to_vec());
    container::write(&Hprz::new(design, vec![attachment])).expect("a container of those")
}

/// Open-Meteo's answer for twelve places; the recorded answers, of one and three, are shorter than
/// the places a sample is damaged at.
pub(super) fn elevation() -> Vec<u8> {
    let heights = ["1400.0", "-427.0", "0.0"].repeat(4).join(",");
    format!("{{\"elevation\":[{heights}]}}").into_bytes()
}

/// A committed list of motors with its first three.
fn few_motors(path: &str) -> Vec<u8> {
    let mut list: Value = serde_json::from_slice(&committed(path)).expect("a committed answer");
    if let Some(motors) = list["motors"].as_array_mut() {
        motors.truncate(3);
    }
    list["count"] = 3.into();
    serde_json::to_vec(&list).expect("a value written")
}

pub(super) fn motor_finder_motors() -> Vec<u8> {
    few_motors("crates/hpr-net/tests/fixtures/replay/motor-finder-motors.json")
}

pub(super) fn motor_finder_in_stock() -> Vec<u8> {
    few_motors("crates/hpr-net/tests/fixtures/replay/motor-finder-in-stock.json")
}

/// The metadata file [`Cache::put`] writes.
pub(super) fn cache_metadata() -> Vec<u8> {
    let dir = tempfile::tempdir().expect("a temporary folder");
    let cache = Cache::new(dir.path());
    cache
        .put(CACHED_URL, b"body", 1_750_000_000)
        .expect("a write");
    super::metadata_file(dir.path())
        .and_then(|path| std::fs::read(path).ok())
        .expect("the metadata written")
}
