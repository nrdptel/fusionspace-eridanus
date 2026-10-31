//! The list of readers of outside bytes, each with its sample and how it is read (M0.15).

use std::path::{Path, PathBuf};

use hpr_atmos::WindInterpolation;
use hpr_flightdata::error::LogError;
use hpr_flightdata::log::{FlightLog, LogFormat};
use hpr_flightdata::workbook::SheetLog;
use hpr_flightdata::{
    aerospacenu_fcb, aerospacenu_gs, altimax, altimax_als, altimetercloud, altos, altos_eeprom,
    altos_telem, altos_telem_csv, archive, arts2, arts2_raw, blueraven, blueraven_usb, capture,
    custom, eggtimer, entacore, entacore_raw, fluctus, fwgps, fwsummary, generic, gpx, import, kml,
    marsa, nmea, packets, perfectflite, phone_launch, raven, raven_fipa, rdas, readings, records,
    rrc3, rrc3_rff, rtx, sillygoose, teensy, workbook, xls,
};
use hpr_io::era5::{Era5Profile, Era5Request, UtcTime};
use hpr_io::geotiff::ElevationRaster;
use hpr_io::netcdf::NetCdf;
use hpr_io::{grib2, orc, ork};
use hpr_motor::{DelayList, eng, rse};
use hpr_net::elevation::{self, Place};
use hpr_net::nomads::NomadsProfile;
use hpr_net::open_meteo::OpenMeteoProfile;
use hpr_net::thrustcurve::{self, DataFile};
use hpr_net::wyoming::WyomingSounding;
use hpr_net::{Cache, motor_finder, on_demand};

use super::{NoOutsideBytes, Reader, Sample, text};

mod built;

const FLIGHT_LAYOUT: &str =
    include_str!("../../../../validation/fixtures/logs/own-logger/flight.layout.json");
const CAPTURE_LAYOUT: &str =
    include_str!("../../../../validation/fixtures/logs/own-logger/flight-capture.layout.json");
const GPS_MAPPING: &str =
    include_str!("../../../../validation/fixtures/logs/own-logger/gps-track.mapping.json");
const TABLE_UNITS: &str =
    include_str!("../../../../crates/hpr-flightdata/tests/samples/table-units.csv");
const FIP_PLAIN: &str =
    include_str!("../../../../crates/hpr-flightdata/tests/samples/raven-fip-plain.csv");
const TWO_FLIGHTS: &[u8] =
    include_bytes!("../../../../validation/fixtures/logs/several/two-flights.xlsx");
const TWO_FLIGHTS_SHEET: &str = "xl/worksheets/sheet0.xml";
const THRUSTCURVE_DOWNLOAD: &[u8] = include_bytes!(
    "../../../../crates/hpr-net/tests/fixtures/replay/thrustcurve-download-H128W.json"
);
/// A table with the columns of the committed `two-flights.xlsx`.
const TWO_FLIGHTS_TABLE: &str = "Time (s),Baro Alt AGL (m)\n0,0\n0.05,0.2\n";
/// The URL a cached entry is stored under; long enough for a metadata file over 64 bytes.
const CACHED_URL: &str =
    "https://example.invalid/api/v1/an-answer-kept-in-the-cache.json?for=damage";

/// A file of the flight logs' samples.
macro_rules! logs {
    ($name:literal) => {
        Sample::File(concat!("crates/hpr-flightdata/tests/samples/", $name))
    };
}

/// A committed file of the own-logger worked example.
macro_rules! own {
    ($name:literal) => {
        Sample::File(concat!("validation/fixtures/logs/own-logger/", $name))
    };
}

/// A committed answer of a server.
macro_rules! replay {
    ($name:literal) => {
        Sample::File(concat!("crates/hpr-net/tests/fixtures/replay/", $name))
    };
}

/// A flight log's reader of text.
macro_rules! log_text {
    ($file:literal, $function:literal, $formats:expr, $sample:expr, $read:path) => {
        Reader {
            krate: "hpr-flightdata",
            file: $file,
            function: $function,
            formats: $formats,
            sample: $sample,
            read: |bytes| log($read(&text(bytes))),
        }
    };
}

/// A flight log's reader of bytes.
macro_rules! log_bytes {
    ($file:literal, $function:literal, $formats:expr, $sample:expr, $read:path) => {
        Reader {
            krate: "hpr-flightdata",
            file: $file,
            function: $function,
            formats: $formats,
            sample: $sample,
            read: |bytes| log($read(bytes)),
        }
    };
}

/// Any other row, its crate first.
macro_rules! row {
    ($krate:literal, $file:literal, $function:literal, $sample:expr, $read:expr) => {
        Reader {
            krate: $krate,
            file: $file,
            function: $function,
            formats: &[],
            sample: $sample,
            read: $read,
        }
    };
}

/// A log, then what every caller reads of it next.
fn log(read: Result<FlightLog, LogError>) -> bool {
    read.map(|log| drop(readings::read(&log))).is_ok()
}

/// A workbook's sheets, each read on; whether any gave a log.
fn sheets(read: Result<Vec<SheetLog>, LogError>) -> bool {
    read.is_ok_and(|sheets| {
        let mut any = false;
        for sheet in sheets {
            any |= log(sheet.log);
        }
        any
    })
}

fn layout(json: &str) -> custom::Layout {
    custom::layout_from_json(json).expect("a committed layout")
}

fn grib(bytes: &[u8]) -> bool {
    let Ok(fields) = grib2::parse(bytes) else {
        return false;
    };
    !fields.is_empty()
        && fields.iter().all(|field| {
            let _ = field.product.forecast_time_s();
            let _ = field.grid.point_deg(0);
            let _ = field.grid.index_at(35.0, 250.0);
            let _ = field.grid.north_to_grid_y_rad(250.0);
            let _ = field.value(0);
            let _ = field.values_at(&[0, field.points().saturating_sub(1)]);
            field.values().is_ok()
        })
}

fn raster(bytes: &[u8]) -> bool {
    let Ok(raster) = ElevationRaster::parse(bytes) else {
        return false;
    };
    let edges = raster.info().bounds();
    let latitude_deg = f64::midpoint(edges.south_deg, edges.north_deg);
    let longitude_deg = f64::midpoint(edges.west_deg, edges.east_deg);
    let _ = raster.height_at(latitude_deg, longitude_deg);
    let _ = raster.value_at(latitude_deg, longitude_deg);
    let _ = raster.values_at(&[(latitude_deg, longitude_deg)]);
    if let Some(pixel) = raster.info().pixel_of(latitude_deg, longitude_deg) {
        let _ = raster.pixel(pixel);
    }
    raster.values().is_ok()
}

fn netcdf(bytes: &[u8]) -> bool {
    let Ok(file) = NetCdf::parse(bytes) else {
        return false;
    };
    for variable in &file.variables {
        let _ = variable.packing();
        let _ = variable.values.text();
        let first = vec![0; variable.shape.len()];
        let last: Vec<u64> = variable.shape.iter().map(|n| n.saturating_sub(1)).collect();
        let _ = variable.unpacked(&first);
        let _ = variable.unpacked(&last);
    }
    true
}

fn era5(bytes: &[u8], latitude_deg: f64, longitude_deg: f64, unix_s: f64) -> bool {
    let (Ok(file), Ok(time)) = (NetCdf::parse(bytes), UtcTime::from_unix_seconds(unix_s)) else {
        return false;
    };
    let request = Era5Request {
        latitude_deg,
        longitude_deg,
        time,
    };
    Era5Profile::read(&file, request)
        .is_ok_and(|profile| profile.sounding(WindInterpolation::SpeedDirection).is_ok())
}

/// A `.ork`, then the design a caller walks it to.
fn ork_file(bytes: &[u8]) -> bool {
    ork::read(bytes).is_ok_and(|read| {
        let _ = ork::airframe_not_as_written(&read.value);
        drop(ork::design(&read.value));
        true
    })
}

fn opened_design(design: &hpr_format::DesignFile) -> bool {
    drop(design.design());
    design.source_files().is_ok()
}

fn opened_hprz(
    read: Result<hpr_format::Opened<hpr_format::container::Hprz>, hpr_format::FormatError>,
) -> bool {
    read.is_ok_and(|opened| opened_design(&opened.value.design))
}

/// A download's files, each read by `hpr_motor`'s reader to its curve.
fn download(bytes: &[u8], read: fn(&DataFile) -> bool) -> bool {
    thrustcurve::parse_download(bytes)
        .is_ok_and(|answer| !answer.results.is_empty() && answer.results.iter().all(read))
}

/// A committed answer's data file holding `bytes` for its text.
fn data_file(bytes: &[u8]) -> DataFile {
    let mut file = thrustcurve::parse_download(THRUSTCURVE_DOWNLOAD)
        .expect("a committed answer")
        .results
        .swap_remove(0);
    file.data = text(bytes).into_owned();
    file
}

/// The metadata file in a cache's folder.
fn metadata_file(dir: &Path) -> Option<PathBuf> {
    std::fs::read_dir(dir)
        .ok()?
        .flatten()
        .map(|entry| entry.path())
        .find(|path| path.extension().is_some_and(|e| e == "json"))
}

/// One row for each public function named `read…` or `parse…` in [`CRATES`](super::CRATES), exactly.
pub(super) const READERS: &[Reader] = &[
    // The importer, and each format through what it dispatches to.
    log_bytes!(
        "import.rs",
        "read",
        &[],
        logs!("altos-table.csv"),
        import::read
    ),
    row!(
        "hpr-flightdata",
        "import.rs",
        "read_one",
        logs!("altos-table.csv"),
        |bytes| { log(import::read_one(bytes, None, None)) }
    ),
    log_bytes!(
        "raven_fipa.rs",
        "read",
        &[LogFormat::RavenFipa],
        Sample::Built(
            built::raven_fipa,
            "no .FIPa is committed: the format's own tests write one"
        ),
        raven_fipa::read
    ),
    log_bytes!(
        "rrc3_rff.rs",
        "read",
        &[LogFormat::Rrc3Rff],
        Sample::Built(
            built::rrc3_rff,
            "no .rff is committed: the format's own tests write one"
        ),
        rrc3_rff::read
    ),
    log_bytes!(
        "arts2_raw.rs",
        "read_odf",
        &[LogFormat::Arts2Odf],
        logs!("arts2-raw.odf"),
        arts2_raw::read_odf
    ),
    log_bytes!(
        "rtx.rs",
        "read_raw",
        &[LogFormat::RtxRaw],
        logs!("rtx.rtx"),
        rtx::read_raw
    ),
    log_bytes!(
        "altimax_als.rs",
        "read",
        &[LogFormat::AltimaxAls],
        Sample::Built(
            built::altimax_als,
            "no .ALS is committed: the format's own tests write one"
        ),
        altimax_als::read
    ),
    log_bytes!(
        "entacore_raw.rs",
        "read_xtra",
        &[LogFormat::EntacoreXtra],
        Sample::Built(
            built::entacore_xtra,
            "no .xtra is committed: the format's own tests write one"
        ),
        entacore_raw::read_xtra
    ),
    Reader {
        krate: "hpr-flightdata",
        file: "entacore_raw.rs",
        function: "read_dump",
        formats: &[LogFormat::EntacoreDump],
        sample: Sample::File("validation/fixtures/logs/several/two-recordings.dump"),
        read: |bytes| {
            entacore_raw::read_dump(bytes).is_ok_and(|logs| {
                for log in &logs {
                    drop(readings::read(log));
                }
                !logs.is_empty()
            })
        },
    },
    log_text!(
        "packets.rs",
        "read_rtf",
        &[LogFormat::RadioPacketRtf],
        logs!("radio-packets.rtf"),
        packets::read_rtf
    ),
    log_text!(
        "eggtimer.rs",
        "read_rtf",
        &[LogFormat::EggtimerRtf],
        logs!("eggtimer-rtf-two-downloads.rtf"),
        eggtimer::read_rtf
    ),
    log_text!(
        "aerospacenu_gs.rs",
        "read",
        &[LogFormat::AerospaceNuGroundStationText],
        logs!("aerospacenu-gs-kept.txt"),
        aerospacenu_gs::read
    ),
    log_text!(
        "blueraven_usb.rs",
        "read",
        &[LogFormat::BlueRavenUsb],
        logs!("blueraven-usb-high-rate.txt"),
        blueraven_usb::read
    ),
    log_text!(
        "teensy.rs",
        "read",
        &[LogFormat::TeensyTabs],
        logs!("teensy-tabs.txt"),
        teensy::read
    ),
    log_text!(
        "teensy.rs",
        "read_named",
        &[LogFormat::TeensyNamed],
        logs!("teensy-named.txt"),
        teensy::read_named
    ),
    log_text!(
        "packets.rs",
        "read",
        &[LogFormat::RadioPacketText],
        logs!("radio-packets-received.txt"),
        packets::read
    ),
    log_text!(
        "altimax.rs",
        "read",
        &[LogFormat::AltimaxXls],
        Sample::Built(
            built::altimax_text,
            "no AltiMAX export is committed: its reader's tests write one"
        ),
        altimax::read
    ),
    log_text!(
        "marsa.rs",
        "read",
        &[LogFormat::Marsa],
        logs!("marsa.dat"),
        marsa::read
    ),
    Reader {
        krate: "hpr-flightdata",
        file: "capture.rs",
        function: "read",
        formats: &[LogFormat::ConsoleCapture],
        sample: own!("flight-capture.txt"),
        read: |bytes| {
            log(capture::read(
                &text(bytes),
                "flight",
                &layout(CAPTURE_LAYOUT).record_layout(),
            ))
        },
    },
    log_text!(
        "kml.rs",
        "read",
        &[LogFormat::Kml],
        logs!("kml-fw2kml-track.kml"),
        kml::read
    ),
    log_text!(
        "gpx.rs",
        "read",
        &[LogFormat::Gpx],
        logs!("gpx-1.1-two-flights.gpx"),
        gpx::read
    ),
    log_text!(
        "nmea.rs",
        "read",
        &[LogFormat::Nmea0183],
        logs!("nmea-noise-and-midnight.txt"),
        nmea::read
    ),
    log_text!(
        "fwgps.rs",
        "read",
        &[LogFormat::FeatherweightGpsCsv],
        logs!("fwgps-tracker-older.csv"),
        fwgps::read
    ),
    log_text!(
        "fwsummary.rs",
        "read",
        &[LogFormat::FeatherweightSummaryCsv],
        Sample::Built(
            built::featherweight_summary,
            "no app summary is committed: the format's own tests hold one"
        ),
        fwsummary::read
    ),
    log_text!(
        "rdas.rs",
        "read",
        &[LogFormat::RdasText],
        logs!("rdas-raw.txt"),
        rdas::read
    ),
    log_text!(
        "altos_eeprom.rs",
        "read",
        &[LogFormat::AltosEeprom],
        logs!("altos-eeprom-telemetrum-v1.eeprom"),
        altos_eeprom::read
    ),
    log_text!(
        "altos_telem.rs",
        "read",
        &[LogFormat::AltosTelemetry],
        logs!("altos-telemetry.telem"),
        altos_telem::read
    ),
    log_text!(
        "altos_telem_csv.rs",
        "read",
        &[LogFormat::AltosTelemetryCsv],
        logs!("altos-telemetry-table.csv"),
        altos_telem_csv::read
    ),
    log_text!(
        "arts2.rs",
        "read",
        &[LogFormat::Arts2Csv],
        logs!("arts2-metric.csv"),
        arts2::read
    ),
    log_text!(
        "arts2_raw.rs",
        "read_csv",
        &[LogFormat::Arts2RawCsv],
        logs!("arts2-raw.csv"),
        arts2_raw::read_csv
    ),
    log_text!(
        "rtx.rs",
        "read_export",
        &[LogFormat::RtxCsv],
        logs!("rtx.csv"),
        rtx::read_export
    ),
    log_text!(
        "perfectflite.rs",
        "read",
        &[LogFormat::PerfectFlitePf2],
        Sample::File("validation/fixtures/logs/synthetic-pnut.pf2"),
        perfectflite::read
    ),
    log_text!(
        "perfectflite.rs",
        "read_pfa",
        &[LogFormat::PerfectFlitePfa],
        logs!("perfectflite-alt15k.pfa"),
        perfectflite::read_pfa
    ),
    log_text!(
        "perfectflite.rs",
        "read_export",
        &[LogFormat::PerfectFliteCsv],
        logs!("perfectflite-export.csv"),
        perfectflite::read_export
    ),
    log_text!(
        "altos.rs",
        "read",
        &[LogFormat::AltosCsv],
        logs!("altos-table.csv"),
        altos::read
    ),
    log_text!(
        "raven.rs",
        "read",
        &[LogFormat::RavenFipCsv],
        logs!("raven-fip-plain.csv"),
        raven::read
    ),
    log_text!(
        "blueraven.rs",
        "read",
        &[LogFormat::BlueRavenCsv],
        logs!("blueraven-high-rate.csv"),
        blueraven::read
    ),
    log_text!(
        "rrc3.rs",
        "read",
        &[LogFormat::Rrc3Csv],
        logs!("rrc3-mph.csv"),
        rrc3::read
    ),
    log_text!(
        "altimetercloud.rs",
        "read",
        &[LogFormat::AltimeterCloudCsv],
        logs!("altimetercloud-mercury.csv"),
        altimetercloud::read
    ),
    log_text!(
        "entacore.rs",
        "read",
        &[LogFormat::EntacoreAimCsv],
        logs!("entacore-feet.csv"),
        entacore::read
    ),
    log_text!(
        "sillygoose.rs",
        "read",
        &[LogFormat::SillyGooseText],
        logs!("sillygoose-config.txt"),
        sillygoose::read
    ),
    log_text!(
        "fluctus.rs",
        "read",
        &[LogFormat::FluctusCsv],
        logs!("fluctus-other-units.csv"),
        fluctus::read
    ),
    log_text!(
        "aerospacenu_fcb.rs",
        "read",
        &[LogFormat::AerospaceNuFcbCsv],
        logs!("aerospacenu-fcb-seconds.csv"),
        aerospacenu_fcb::read
    ),
    log_text!(
        "phone_launch.rs",
        "read",
        &[LogFormat::PhoneLaunchCsv],
        logs!("phone-launch-older.csv"),
        phone_launch::read
    ),
    log_text!(
        "eggtimer.rs",
        "read_text",
        &[LogFormat::EggtimerText],
        logs!("eggtimer-text-trs-2-07.txt"),
        eggtimer::read_text
    ),
    log_text!(
        "eggtimer.rs",
        "read",
        &[LogFormat::EggtimerCsv],
        logs!("eggtimer-trs-seconds.csv"),
        eggtimer::read
    ),
    log_text!(
        "generic.rs",
        "read",
        &[LogFormat::GenericTable],
        logs!("table-units.csv"),
        generic::read
    ),
    // A caller's mapping, layout or header, the file's bytes damaged.
    row!(
        "hpr-flightdata",
        "generic.rs",
        "read_with",
        logs!("table-units.csv"),
        |bytes| {
            let mapping = generic::mapping(TABLE_UNITS).expect("a committed table");
            log(generic::read_with(&text(bytes), &mapping))
        }
    ),
    row!(
        "hpr-flightdata",
        "raven.rs",
        "read_rows",
        logs!("raven-fip-plain.csv"),
        |bytes| {
            let whole = text(bytes);
            whole
                .split_once('\n')
                .is_some_and(|(header, rows)| log(raven::read_rows(header, rows)))
        }
    ),
    row!(
        "hpr-flightdata",
        "raven.rs",
        "read_rows_around",
        logs!("raven-fip-plain.csv"),
        |bytes| {
            let whole = text(bytes);
            whole
                .split_once('\n')
                .is_some_and(|(header, rows)| log(raven::read_rows_around(header, rows, &[2])))
        }
    ),
    row!(
        "hpr-flightdata",
        "records.rs",
        "read",
        own!("flight.bin"),
        |bytes| { log(records::read(bytes, &layout(FLIGHT_LAYOUT).record_layout())) }
    ),
    row!(
        "hpr-flightdata",
        "custom.rs",
        "read_by_fip_header",
        Sample::Built(
            built::fip_rows,
            "the committed sample's rows: its first line is the header the caller states"
        ),
        |bytes| {
            let header = FIP_PLAIN.lines().next().unwrap_or_default().to_owned();
            let stated = custom::FipHeader {
                header,
                pasted: Vec::new(),
            };
            log(custom::read_by_fip_header(bytes, &stated, None))
        }
    ),
    row!(
        "hpr-flightdata",
        "custom.rs",
        "read_by_mapping",
        own!("gps-track.csv"),
        |bytes| {
            let mapping = custom::mapping_from_json(GPS_MAPPING).expect("a committed mapping");
            log(custom::read_by_mapping(bytes, &mapping, None))
        }
    ),
    row!(
        "hpr-flightdata",
        "custom.rs",
        "read_by_layout",
        own!("flight.bin"),
        |bytes| { log(custom::read_by_layout(bytes, &layout(FLIGHT_LAYOUT))) }
    ),
    row!(
        "hpr-flightdata",
        "workbook.rs",
        "read",
        Sample::File("validation/fixtures/logs/several/two-flights.xlsx"),
        |bytes| sheets(workbook::read(bytes))
    ),
    row!(
        "hpr-flightdata",
        "xls.rs",
        "read",
        Sample::Built(
            built::xls,
            "no .xls is committed: the reader's tests write a worksheet stream"
        ),
        |bytes| sheets(xls::read(bytes))
    ),
    // hpr-io, each followed to the values a caller takes from it.
    row!(
        "hpr-io",
        "grib2/mod.rs",
        "parse",
        Sample::File("crates/hpr-net/tests/fixtures/replay/nomads-gfs.grib2"),
        grib
    ),
    row!(
        "hpr-io",
        "geotiff/mod.rs",
        "parse",
        Sample::File("crates/hpr-io/tests/fixtures/geotiff/usgs-i16-deflate-strips-be.tif"),
        raster
    ),
    row!(
        "hpr-io",
        "netcdf/mod.rs",
        "parse",
        Sample::File("validation/fixtures/weather/netcdf/types-classic.nc"),
        netcdf
    ),
    row!(
        "hpr-io",
        "era5/mod.rs",
        "read",
        Sample::File("validation/fixtures/weather/era5/bella-lui.nc"),
        |bytes| era5(bytes, 47.213476, 9.003336, 1_582_376_400.0)
    ),
    row!(
        "hpr-io",
        "orc/mod.rs",
        "read",
        Sample::File("crates/hpr-io/data/openrocket-database/apogee.orc"),
        |bytes| {
            orc::read(&text(bytes), "apogee.orc").is_ok_and(|file| {
                drop(file.catalog.search(None, "a"));
                true
            })
        }
    ),
    row!(
        "hpr-io",
        "ork/mod.rs",
        "read",
        Sample::File("validation/fixtures/ork/loft-demo/demo-dual-deploy.ork"),
        ork_file
    ),
    row!(
        "hpr-io",
        "ork/document.rs",
        "parse",
        Sample::Built(
            built::ork_design,
            "the committed file's design, unpacked: this reader takes the XML, not the zip"
        ),
        |bytes| ork::Document::parse(&text(bytes)).is_ok()
    ),
    row!(
        "hpr-io",
        "ork/document.rs",
        "parse",
        Sample::Built(
            built::schema_version,
            "an attribute's value, a fragment of a file, padded past the 64 places"
        ),
        |bytes| ork::SchemaVersion::parse(&text(bytes)).is_ok()
    ),
    row!(
        "hpr-io",
        "ork/recovery.rs",
        "parse",
        Sample::Built(
            built::deploy_event,
            "a deployment event's text, a fragment of a file, padded past the 64 places"
        ),
        |bytes| {
            drop(ork::DeployEvent::parse(&text(bytes)));
            true
        }
    ),
    row!(
        "hpr-io",
        "ork/recovery.rs",
        "parse",
        Sample::Built(
            built::separation_event,
            "a separation event's text, a fragment of a file, padded past the 64 places"
        ),
        |bytes| {
            drop(ork::SeparationEvent::parse(&text(bytes)));
            true
        }
    ),
    row!(
        "hpr-io",
        "ork/motors.rs",
        "parse",
        Sample::Built(
            built::ignition_event,
            "an element's text, a fragment of a file, padded past the 64 places"
        ),
        |bytes| {
            drop(ork::IgnitionEvent::parse(&text(bytes)));
            true
        }
    ),
    row!(
        "hpr-io",
        "ork/value.rs",
        "parse",
        Sample::Built(
            built::dimension,
            "an element's text, a fragment of a file, padded past the 64 places"
        ),
        |bytes| ork::Dimension::parse(&text(bytes)).is_some()
    ),
    // hpr-motor.
    row!(
        "hpr-motor",
        "eng.rs",
        "parse",
        Sample::File("crates/hpr-motor/data/thrustcurve/curves/5f4294d20002e90000000895.eng"),
        |bytes| {
            eng::parse(&text(bytes)).is_ok_and(|parsed| {
                parsed.value.entries.iter().all(|entry| {
                    drop(entry.delays());
                    entry.thrust_curve().is_ok()
                })
            })
        }
    ),
    row!(
        "hpr-motor",
        "rse.rs",
        "parse",
        Sample::File("crates/hpr-motor/data/thrustcurve/curves/5f4294d20002e90000000390.rse"),
        |bytes| {
            rse::parse(&text(bytes)).is_ok_and(|parsed| {
                parsed.value.engines.iter().all(|engine| {
                    drop(engine.delays());
                    engine.thrust_curve().is_ok()
                })
            })
        }
    ),
    row!(
        "hpr-motor",
        "delay.rs",
        "parse",
        Sample::Built(
            built::delays,
            "a motor's delay field, a fragment of a file, long enough for the 64 places"
        ),
        |bytes| {
            let _ = DelayList::parse(&text(bytes)).has_plugged();
            true
        }
    ),
    // hpr-format.
    row!(
        "hpr-format",
        "lib.rs",
        "read_json",
        Sample::File("crates/hpr-format/fixtures/embedded-curve-0.1.hpr"),
        |bytes| hpr_format::read_json(&text(bytes))
            .is_ok_and(|opened| opened_design(&opened.value))
    ),
    row!(
        "hpr-format",
        "container.rs",
        "read",
        Sample::Built(
            built::hprz,
            "no .hprz is committed: this writes the committed .hpr and a file into one"
        ),
        |bytes| opened_hprz(hpr_format::container::read(bytes))
    ),
    row!(
        "hpr-format",
        "container.rs",
        "read_within",
        Sample::Built(
            built::hprz,
            "no .hprz is committed: this writes the committed .hpr and a file into one"
        ),
        |bytes| opened_hprz(hpr_format::container::read_within(bytes, 1 << 20))
    ),
    // hpr-net: the servers' recorded answers.
    row!(
        "hpr-net",
        "elevation.rs",
        "parse",
        Sample::Built(
            built::elevation,
            "the recorded answers are 22 and 35 bytes, short of the 64 places"
        ),
        |bytes| {
            let places = vec![Place::new(32.99, -106.97); 12];
            elevation::parse(bytes, &places).is_ok()
        }
    ),
    row!(
        "hpr-net",
        "wyoming.rs",
        "parse",
        replay!("wyoming-72364-fm35.csv"),
        |bytes| {
            WyomingSounding::parse(bytes)
                .is_ok_and(|sounding| sounding.sounding(WindInterpolation::SpeedDirection).is_ok())
        }
    ),
    row!(
        "hpr-net",
        "open_meteo.rs",
        "parse",
        replay!("open-meteo-forecast.json"),
        |bytes| {
            OpenMeteoProfile::parse(bytes, 1_790_964_000)
                .is_ok_and(|profile| profile.sounding(WindInterpolation::SpeedDirection).is_ok())
        }
    ),
    row!(
        "hpr-net",
        "nomads.rs",
        "parse",
        replay!("nomads-gfs.grib2"),
        |bytes| {
            NomadsProfile::parse(bytes, 32.99, -106.97)
                .is_ok_and(|profile| profile.sounding(WindInterpolation::SpeedDirection).is_ok())
        }
    ),
    row!(
        "hpr-net",
        "motor_finder.rs",
        "parse_meta",
        replay!("motor-finder-meta.json"),
        |bytes| { motor_finder::parse_meta(bytes).is_ok() }
    ),
    row!(
        "hpr-net",
        "motor_finder.rs",
        "parse_motors",
        Sample::Built(
            built::motor_finder_motors,
            "the recorded answer is 1.6 MB: its first three motors, read some 400 times"
        ),
        |bytes| motor_finder::parse_motors(bytes).is_ok()
    ),
    row!(
        "hpr-net",
        "motor_finder.rs",
        "parse_in_stock",
        Sample::Built(
            built::motor_finder_in_stock,
            "the recorded answer is 960 KB: its first three motors, read some 400 times"
        ),
        |bytes| motor_finder::parse_in_stock(bytes).is_ok()
    ),
    row!(
        "hpr-net",
        "motor_finder.rs",
        "parse_vendors",
        replay!("motor-finder-vendors.json"),
        |bytes| { motor_finder::parse_vendors(bytes).is_ok() }
    ),
    row!(
        "hpr-net",
        "motor_finder.rs",
        "parse_motor",
        replay!("motor-finder-loki-J-326-LR.json"),
        |bytes| { motor_finder::parse_motor(bytes).is_ok() }
    ),
    row!(
        "hpr-net",
        "thrustcurve.rs",
        "parse_search",
        replay!("thrustcurve-search-aerotech-H128W.json"),
        |bytes| { thrustcurve::parse_search(bytes).is_ok() }
    ),
    row!(
        "hpr-net",
        "thrustcurve.rs",
        "parse_download",
        replay!("thrustcurve-download-H128W.json"),
        |bytes| { thrustcurve::parse_download(bytes).is_ok() }
    ),
    row!(
        "hpr-net",
        "thrustcurve.rs",
        "read",
        Sample::Built(built::thrustcurve_data, built::THRUSTCURVE_DATA),
        |bytes| {
            data_file(bytes)
                .read()
                .is_ok_and(|curve| curve.thrust_curve().is_ok())
        }
    ),
    row!(
        "hpr-net",
        "on_demand.rs",
        "reads_as_one_motor",
        Sample::Built(built::thrustcurve_data, built::THRUSTCURVE_DATA),
        |bytes| { on_demand::reads_as_one_motor(&data_file(bytes)).is_ok() }
    ),
];

/// More rows, which the name check doesn't count: other samples of a reader above, for another
/// format or layout, and readers of outside bytes not named `read…` or `parse…`.
///
/// `Catalog::from_json` isn't here: it reads the catalog compiled into `hpr-motor`, nothing from
/// outside.
pub(super) const EXTRA_READERS: &[Reader] = &[
    row!(
        "hpr-flightdata",
        "custom.rs",
        "json_text",
        own!("gps-track.mapping.json"),
        |bytes| custom::json_text(bytes).is_some()
    ),
    row!(
        "hpr-flightdata",
        "custom.rs",
        "header_mapping",
        logs!("table-units.csv"),
        |bytes| custom::header_mapping(bytes, None).is_ok()
    ),
    row!(
        "hpr-flightdata",
        "generic.rs",
        "mapping",
        logs!("table-units.csv"),
        |bytes| generic::mapping(&text(bytes)).is_ok()
    ),
    row!(
        "hpr-flightdata",
        "generic.rs",
        "header_reading",
        logs!("table-units.csv"),
        |bytes| generic::header_reading(&text(bytes)).is_ok()
    ),
    row!(
        "hpr-flightdata",
        "workbook.rs",
        "sheets",
        Sample::Built(
            built::workbook_sheet,
            "the committed workbook's sheet, zipped again at each read: a changed byte of the zip \
             only fails its checksum"
        ),
        |bytes| workbook::sheets(&built::workbook_with(bytes)).is_ok()
    ),
    row!(
        "hpr-flightdata",
        "import.rs",
        "read_one",
        Sample::Built(
            built::workbook_sheet,
            "the same sheet through the importer: a sheet's damage past the zip's checksum"
        ),
        |bytes| log(import::read_one(
            &built::workbook_with(bytes),
            Some(0),
            None
        ))
    ),
    row!(
        "hpr-flightdata",
        "xls.rs",
        "sheets",
        Sample::Built(
            built::xls,
            "no .xls is committed: the sheets of a worksheet stream the reader's tests write"
        ),
        |bytes| xls::sheets(bytes).is_ok()
    ),
    row!(
        "hpr-flightdata",
        "records.rs",
        "table",
        own!("flight.bin"),
        |bytes| records::table(bytes, &layout(FLIGHT_LAYOUT).record_layout()).is_ok()
    ),
    row!(
        "hpr-flightdata",
        "capture.rs",
        "blocks",
        own!("flight-capture.txt"),
        |bytes| capture::blocks(&text(bytes)).is_ok()
    ),
    row!(
        "hpr-flightdata",
        "import.rs",
        "detect",
        logs!("altos-table.csv"),
        |bytes| import::detect(bytes).is_some()
    ),
    row!(
        "hpr-flightdata",
        "import.rs",
        "decode",
        logs!("altos-table.csv"),
        |bytes| {
            drop(import::decode(bytes));
            true
        }
    ),
    row!(
        "hpr-format",
        "lib.rs",
        "from_ork",
        Sample::Built(
            built::ork_design,
            "the committed file's design, unpacked, for the conversion: a changed byte of a zip \
             only fails its checksum"
        ),
        |bytes| hpr_format::DesignFile::from_ork(bytes).is_ok()
    ),
    row!(
        "hpr-flightdata",
        "import.rs",
        "read_one",
        Sample::File("validation/fixtures/logs/several/two-flights.xlsx"),
        |bytes| log(import::read_one(bytes, Some(0), None))
    ),
    row!(
        "hpr-flightdata",
        "import.rs",
        "read_one",
        Sample::File("validation/fixtures/logs/several/two-recordings.dump"),
        |bytes| log(import::read_one(bytes, None, Some(1)))
    ),
    log_text!(
        "rtx.rs",
        "read_export",
        &[LogFormat::RtxText],
        logs!("rtx.txt"),
        rtx::read_export
    ),
    log_text!(
        "perfectflite.rs",
        "read_export",
        &[LogFormat::PerfectFliteText],
        logs!("perfectflite-export.txt"),
        perfectflite::read_export
    ),
    row!(
        "hpr-flightdata",
        "custom.rs",
        "read_by_mapping",
        Sample::File("validation/fixtures/logs/several/two-flights.xlsx"),
        |bytes| {
            let mapping = generic::mapping(TWO_FLIGHTS_TABLE).expect("a table's header");
            log(custom::read_by_mapping(bytes, &mapping, Some(0)))
        }
    ),
    row!(
        "hpr-flightdata",
        "custom.rs",
        "read_by_layout",
        own!("flight-capture.txt"),
        |bytes| { log(custom::read_by_layout(bytes, &layout(CAPTURE_LAYOUT))) }
    ),
    row!(
        "hpr-flightdata",
        "workbook.rs",
        "read",
        Sample::Built(
            built::workbook_sheet,
            "the committed workbook's sheet, zipped again at each read: a changed byte of the zip \
             only fails its checksum"
        ),
        |bytes| sheets(workbook::read(&built::workbook_with(bytes)))
    ),
    row!(
        "hpr-io",
        "grib2/mod.rs",
        "parse",
        Sample::File("crates/hpr-net/tests/fixtures/replay/nomads-rap.grib2"),
        grib
    ),
    row!(
        "hpr-io",
        "geotiff/mod.rs",
        "parse",
        Sample::File("crates/hpr-io/tests/fixtures/geotiff/usgs-i32-lzw-tiles-lon360.tif"),
        raster
    ),
    row!(
        "hpr-io",
        "geotiff/mod.rs",
        "parse",
        Sample::File("crates/hpr-io/tests/fixtures/geotiff/usgs-u16-packbits-ftus-nodata.tif"),
        raster
    ),
    row!(
        "hpr-io",
        "geotiff/mod.rs",
        "parse",
        Sample::File("crates/hpr-io/tests/fixtures/geotiff/usgs-f32-lzw-fp-tiles.tif"),
        raster
    ),
    row!(
        "hpr-io",
        "geotiff/mod.rs",
        "parse",
        Sample::File("crates/hpr-io/tests/fixtures/geotiff/usgs-f64-raw-bigtiff-point.tif"),
        raster
    ),
    row!(
        "hpr-io",
        "netcdf/mod.rs",
        "parse",
        Sample::File("validation/fixtures/weather/netcdf/records-offset64.nc"),
        netcdf
    ),
    row!(
        "hpr-io",
        "netcdf/mod.rs",
        "parse",
        Sample::File("validation/fixtures/weather/netcdf/packing-classic.nc"),
        netcdf
    ),
    row!(
        "hpr-io",
        "era5/mod.rs",
        "read",
        Sample::File("validation/fixtures/weather/era5/ndrt-2020-cds.nc"),
        |bytes| era5(bytes, 41.775447, -86.572467, 1_582_473_600.0)
    ),
    row!(
        "hpr-io",
        "ork/mod.rs",
        "read",
        Sample::Built(
            built::ork_design,
            "the committed file's design, unpacked: a changed byte of a zip only fails its checksum"
        ),
        ork_file
    ),
    row!(
        "hpr-net",
        "thrustcurve.rs",
        "read",
        replay!("thrustcurve-download-H128W-rocksim.json"),
        |bytes| {
            download(bytes, |file| {
                file.read().is_ok_and(|curve| curve.thrust_curve().is_ok())
            })
        }
    ),
    row!(
        "hpr-format",
        "lib.rs",
        "from_json",
        Sample::File("crates/hpr-format/fixtures/embedded-curve-0.1.hpr"),
        |bytes| hpr_format::from_json(&text(bytes)).is_ok()
    ),
    row!(
        "hpr-flightdata",
        "archive.rs",
        "unpack",
        Sample::File("validation/fixtures/ork/loft-demo/demo-quirks.ork"),
        |bytes| {
            archive::unpack(bytes).is_ok_and(|members| {
                for member in &members {
                    let _ = import::read(&member.bytes);
                }
                !members.is_empty()
            })
        }
    ),
    row!(
        "hpr-flightdata",
        "custom.rs",
        "mapping_from_json",
        own!("gps-track.mapping.json"),
        |bytes| { custom::mapping_from_json(&text(bytes)).is_ok() }
    ),
    row!(
        "hpr-flightdata",
        "custom.rs",
        "layout_from_json",
        own!("flight.layout.json"),
        |bytes| { custom::layout_from_json(&text(bytes)).is_ok() }
    ),
    row!(
        "hpr-flightdata",
        "custom.rs",
        "fip_header_from_json",
        Sample::File("validation/fixtures/logs/several/fip-below-no-header.header.json"),
        |bytes| custom::fip_header_from_json(&text(bytes)).is_ok()
    ),
    row!(
        "hpr-format",
        "lib.rs",
        "from_ork",
        Sample::File("validation/fixtures/ork/loft-demo/demo-dual-deploy.ork"),
        |bytes| hpr_format::DesignFile::from_ork(bytes).is_ok()
    ),
    row!(
        "hpr-net",
        "cache.rs",
        "get",
        Sample::Built(
            built::cache_metadata,
            "the cache writes its metadata when it stores an answer; no copy is committed"
        ),
        |bytes| {
            let Ok(dir) = tempfile::tempdir() else {
                return false;
            };
            let cache = Cache::new(dir.path());
            if cache.put(CACHED_URL, b"body", 1_750_000_000).is_err() {
                return false;
            }
            let Some(path) = metadata_file(dir.path()) else {
                return false;
            };
            std::fs::write(path, bytes).is_ok()
                && cache.get(CACHED_URL).is_ok_and(|entry| entry.is_some())
        }
    ),
];

/// What is named as a reader and reads no outside bytes.
pub(super) const NO_OUTSIDE_BYTES: &[NoOutsideBytes] = &[NoOutsideBytes {
    krate: "hpr-flightdata",
    file: "readings.rs",
    function: "read",
    why: "takes a FlightLog already read, which every log row above passes it",
}];
