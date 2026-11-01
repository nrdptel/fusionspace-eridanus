//! Flight-log importers, the canonical flight record, smoothing and time alignment, and the
//! readings taken from a flight with the provenance of each.
//!
//! [Start here][guide-start] says what works today and what is planned.
//!
//! [guide-start]: https://hpr.fusionspace.co/start-here.html
//! [roadmap]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/ROADMAP.md
//! [adr-046]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/DECISIONS.md#adr-046-debrief-folded-in-and-flight-log-analysis-that-stands-without-the-simulator-2026-09-20
//!
//! This crate doesn't depend on the simulator, and [must not][adr-046]. Reading a flight log and
//! working out what it says is a use of this project in its own right. It serves someone who has
//! a log and neither a design file nor any wish to simulate anything. Comparing a flight with a
//! simulation of it is `hpr-forensics`, which depends on both.
//!
//! Every reading carries where it came from: measured by an instrument, derived from what was
//! measured, or clipped because the sensor saturated. A reading the log can't support is withheld
//! with a reason rather than printed.
//!
//! Status: pre-alpha. [`import::read`] recognizes a log's layout from its content. It reads the
//! log into the record every reader produces ([`log::FlightLog`]). The readers for altimeters and
//! flight computers:
//!
//! - PerfectFlite's `.pf2`, its CSV and text exports, and its older `.pfa` ([`perfectflite`]).
//! - AltOS's CSV, and a table of AltOS's own columns with no `version` header ([`altos`]). It also
//!   reads an AltOS export a spreadsheet re-saved with formula cells, each formula kept as text.
//! - AltOS's radio telemetry as AltosUI saves it ([`altos_telem`]).
//! - AltOS's radio telemetry decoded to a table ([`altos_telem_csv`]).
//! - AltOS's raw downloads: the board's own log as AltosUI saves it to an `.eeprom` file
//!   ([`altos_eeprom`]). It's checked against AltosUI's export of the same flight in all 9 samples
//!   that have one.
//! - The Featherweight Raven's FIP CSV ([`raven`]). A FIP export that lost its header line is read
//!   under the header the caller states ([`raven::read_rows`]).
//! - The Blue Raven's CSV ([`blueraven`]).
//! - The Blue Raven's USB download to a serial terminal, each line held to its CRC
//!   ([`blueraven_usb`]).
//! - The Featherweight app's summaries of a Blue Raven's or a GPS tracker's flight
//!   ([`fwsummary`]).
//! - Missile Works RRC3's CSV ([`rrc3`]).
//! - Two raw downloads, the Raven's `.FIPa` ([`raven_fipa`]) and the RRC3's `.rff`
//!   ([`rrc3_rff`]). Each is checked against its maker's export of the same flight where the
//!   sample logs hold one. That's 55 of the 57 `.FIPa` files and 1 of the 6 `.rff`.
//! - Eggtimer's CSV, an Eggtimer TRS's terminal text, and an Eggtimer's download saved as RTF
//!   ([`eggtimer`]).
//! - AltimeterCloud's CSV ([`altimetercloud`]).
//! - Entacore AIM's CSV ([`entacore`]).
//! - AerospaceNU's SillyGoose logs ([`sillygoose`]).
//! - AerospaceNU's flight computer board's CSV ([`aerospacenu_fcb`]) and its ground station's
//!   text ([`aerospacenu_gs`]).
//! - Silicdyne Fluctus's CSV ([`fluctus`]).
//! - The ARTS2's interpreted data ([`arts2`]).
//! - The ARTS2's `.odf` and its raw CSV export ([`arts2_raw`]). The `.odf` is read in units held
//!   to the interpreted export of the same flight.
//! - The R-DAS's raw text, its counts kept as written ([`rdas`]).
//! - A phone app's `launch` CSV ([`phone_launch`]).
//! - A course's Teensy data logger, one sensor's reading a line ([`teensy`]).
//! - One team's radio telemetry packets, `255:…:254`, as its flight computer logged them, as a
//!   receiver heard them, and inside an RTF document ([`packets`]).
//! - A MARSA altimeter's download and a spreadsheet's re-save of it ([`marsa`]).
//!
//! An AltiMAX's own `.ALS` file is read too ([`altimax_als`]), as are an Entacore AIM's `.xtra`
//! files and raw dumps of its memory ([`entacore_raw`]). Each of these layouts was worked out from
//! one sample flight and held to the maker's conversion of it.
//!
//! Five GPS logs are read, every column kept as a channel on its own clock:
//!
//! - Featherweight's tracker and ground-station CSV ([`fwgps`]).
//! - KML ([`kml`]).
//! - NMEA 0183 ([`nmea`]).
//! - Missile Works' RTx tracker's `.rtx` file and its CSV and text exports ([`rtx`]).
//! - GPX 1.0 and 1.1, checked on invented files only ([`gpx`]).
//!
//! Files from other loggers, and files that hold other files:
//!
//! - A table from a logger no other reader knows is read by the units its header states
//!   ([`generic`]). Each unit is held to the column's name and values. The caller can give a
//!   column mapping instead.
//! - A team flight computer's binary records are read by a layout the caller states. They come
//!   from a raw file ([`records`]) or a serial console's capture of them as base64 ([`capture`]).
//! - A mapping or a layout can come from the JSON file a person writes for their own logger
//!   ([`custom`]). That's how the command line and Python take them.
//! - A zip archive of a flight's files is unpacked in memory, within a bound, and each file is
//!   read on its own ([`archive`]).
//! - An Excel workbook's sheets are written out as text, each cell as the workbook stores it, and
//!   each sheet is read on its own ([`workbook`]).
//! - An older Excel `.xls` worksheet's cells are written out as text ([`xls`]). An AltiMAX
//!   altimeter's export is one such worksheet ([`altimax`]).
//!
//! [`readings`] takes a first set of readings from the record. They're liftoff, apogee, the top
//! speed and acceleration, burnout, landing, the flight time, and the descents under the drogue
//! and the main. Each states the bound its error stays within. On the synthetic flights' tuning
//! set, logged from 500 to half a sample a second and in the other conditions [`readings`] lists,
//! every reading given stayed within its bound. The guide's [reading a flight log][guide-log]
//! page shows them on a log, and [the layouts read][guide-layouts] lists what each
//! reader keeps. The rest of the readings, smoothing and reconstruction are planned ([the
//! roadmap][roadmap]).
//!
//! [guide-layouts]: https://hpr.fusionspace.co/format/flight-logs.html
//!
//! [guide-log]: https://hpr.fusionspace.co/reading-a-flight-log.html
//!
//! ```
//! use hpr_flightdata::{perfectflite, readings};
//!
//! // A few rows of an invented PerfectFlite log: feet and seconds.
//! let mut text = String::from("PerfectFlite Pnut\nData: (Time, Altitude, Velocity)\n");
//! for (i, feet) in [0, 0, 0, 40, 90, 120, 130, 130, 120, 90, 40, 5, 0, 0, 0, 0].iter().enumerate() {
//!     text.push_str(&format!("{}, {feet}, 0\n", i as f64 * 0.5));
//! }
//! let log = perfectflite::read(&text)?;
//! let read = readings::read(&log);
//! let apogee = read.apogee.value().expect("the log climbs");
//! assert_eq!(apogee.altitude_m, 130.0 * 0.3048);
//! assert_eq!(apogee.time_s, 3.25); // midway between the two samples at the top
//! # Ok::<(), hpr_flightdata::error::LogError>(())
//! ```

// Outside bytes reach casts here. An `as` cast of a length or count truncates silently where
// `usize` is 32 bits (wasm32), and one from a float saturates: each cast is checked with
// `try_from`, or its `allow` says why it is exact or meant.
#![warn(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
#![cfg_attr(
    test,
    allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "a test builds its own inputs, so it sets its casts' ranges"
    )
)]
pub mod aerospacenu_fcb;
pub mod aerospacenu_gs;
pub mod altimax;
pub mod altimax_als;
pub mod altimetercloud;
pub mod altos;
pub mod altos_eeprom;
pub mod altos_telem;
pub mod altos_telem_csv;
pub mod archive;
pub mod arts2;
pub mod arts2_raw;
pub mod blueraven;
pub mod blueraven_usb;
pub mod capture;
pub mod custom;
pub mod eggtimer;
pub mod entacore;
pub mod entacore_raw;
pub mod error;
pub mod filter;
pub mod fluctus;
pub mod fwgps;
pub mod fwsummary;
pub mod generic;
pub mod ghost;
pub mod gpx;
pub mod import;
pub mod kml;
pub mod log;
pub mod marsa;
pub mod nmea;
pub mod packets;
pub mod perfectflite;
pub mod phone_launch;
pub mod raven;
pub mod raven_fipa;
pub mod rdas;
pub mod readings;
pub mod records;
pub mod rrc3;
pub mod rrc3_rff;
pub mod rtx;
pub mod sillygoose;
pub mod smoother;
mod table;
pub mod teensy;
pub mod timeline;
pub mod workbook;
pub mod xls;

#[cfg(test)]
mod synthetic;
