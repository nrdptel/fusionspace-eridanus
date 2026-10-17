//! Flight-log importers, the canonical flight record, smoothing and time alignment, and the
//! readings taken from a flight with the provenance of each.
//!
//! **Guide:** [Start here][guide-start] says what works today and what is planned.
//!
//! [guide-start]: https://hpr.fusionspace.co/start-here.html
//! [roadmap]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/ROADMAP.md
//! [adr-046]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/DECISIONS.md#adr-046-debrief-folded-in-and-flight-log-analysis-that-stands-without-the-simulator-2026-09-20
//!
//! This crate does not depend on the simulator, and [must not][adr-046]: reading a flight log and
//! working out what it says is a use of this project in its own right, for someone who has a log
//! and neither a design file nor any wish to simulate anything. Comparing a flight with a
//! simulation of it is `hpr-forensics`, which depends on both.
//!
//! Every reading carries where it came from (measured by an instrument, derived from what was
//! measured, or clipped because the sensor saturated), and a reading the log cannot support is
//! withheld with a reason rather than printed.
//!
//! Status: pre-alpha. [`import::read`] recognizes a log's layout from its content and reads it into
//! the record every reader produces ([`log::FlightLog`]): PerfectFlite's `.pf2` and its CSV and
//! text exports ([`perfectflite`]), AltOS's CSV ([`altos`]), the Featherweight Raven's FIP CSV
//! ([`raven`]), the Blue Raven's CSV ([`blueraven`]), Missile Works RRC3's ([`rrc3`]),
//! Eggtimer's CSV and an Eggtimer TRS's terminal text ([`eggtimer`]), PerfectFlite's older `.pfa`
//! ([`perfectflite`]), AltimeterCloud's CSV ([`altimetercloud`]) and Entacore AIM's
//! ([`entacore`]), AerospaceNU's SillyGoose logs ([`sillygoose`]), Silicdyne Fluctus's CSV
//! ([`fluctus`]), the ARTS2's interpreted data ([`arts2`]), AerospaceNU's flight computer board's
//! CSV ([`aerospacenu_fcb`]) and its ground station's text ([`aerospacenu_gs`]), a phone app's
//! `launch` CSV ([`phone_launch`]), an Eggtimer's download saved as RTF ([`eggtimer`]), the
//! Featherweight app's summaries of a Blue Raven's or a GPS tracker's flight ([`fwsummary`]), two raw
//! downloads, the Raven's `.FIPa` ([`raven_fipa`]) and the RRC3's `.rff` ([`rrc3_rff`]), each
//! checked against its maker's export of the same flight where the sample logs hold one (55 of the
//! 57 `.FIPa` files, 1 of the 6 `.rff`), the R-DAS's raw text, its counts kept as written
//! ([`rdas`]), AltOS's radio telemetry as AltosUI saves it ([`altos_telem`]), AltOS's raw
//! downloads, the board's own log as AltosUI saves it to an `.eeprom` file, checked against
//! AltosUI's export of the same flight in all 9 samples that have one ([`altos_eeprom`]), the
//! ARTS2's `.odf` and its raw CSV export, the `.odf` in units held to the interpreted export of the
//! same flight ([`arts2_raw`]), and
//! five GPS logs: Featherweight's
//! tracker and ground-station CSV ([`fwgps`]), KML ([`kml`]), NMEA 0183 ([`nmea`]), Missile
//! Works' RTx tracker's `.rtx` file and its CSV and text exports ([`rtx`]) and GPX 1.0 and 1.1,
//! checked on invented files only ([`gpx`]), every column kept as a channel on its own clock. A
//! table from a logger no other reader knows is read by the units its header states, each held
//! to the column's name and values, or by a column mapping the caller gives ([`generic`]). A zip
//! archive of a flight's files is unpacked in memory, within a bound, and each file read on its
//! own ([`archive`]); an Excel workbook's sheets are written out as text, each cell as the
//! workbook stores it, and each sheet read on its own ([`workbook`]); a team flight computer's
//! binary records are read by a layout the caller states, from a raw file ([`records`]) or a
//! serial console's capture of them as base64
//! ([`capture`]), and a mapping or a layout from the JSON file a person writes for their own
//! logger, as the command line and Python take them ([`custom`]); the Blue Raven's USB download to a serial terminal, each line held to its
//! CRC ([`blueraven_usb`]); a course's Teensy data logger, one sensor's reading a line
//! ([`teensy`]); one team's radio telemetry packets, `255:…:254`, as its flight computer
//! logged them, as a receiver heard them, and inside an RTF document ([`packets`]); a Raven FIP
//! export pasted below no header, under the header the caller states ([`raven::read_rows`]); and
//! an older Excel `.xls` worksheet's cells written out as text ([`xls`]), such as an AltiMAX
//! altimeter's export ([`altimax`]); a MARSA altimeter's download and a spreadsheet's re-save of
//! it ([`marsa`]); and AltOS's radio telemetry decoded to a table ([`altos_telem_csv`]),
//! a table of AltOS's own columns with no `version` header, and an AltOS export a spreadsheet
//! re-saved with formula cells, each formula kept as text ([`altos`]). [`readings`] takes a first set of readings from the
//! record: liftoff, apogee, the top speed, landing and the descent, set and tested on PerfectFlite
//! logs. The guide's [reading a flight log][guide-log] page shows them on a log, and [the layouts read][guide-layouts] lists what each reader keeps.
//! An AltiMAX's own `.ALS` file ([`altimax_als`]) and an Entacore AIM's `.xtra` files and raw
//! dumps of its memory ([`entacore_raw`]) are read too, each layout worked out from one sample
//! flight and held to the maker's conversion of it. The rest of the readings, smoothing and
//! reconstruction are planned for [M7.2][roadmap] of the roadmap.
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
mod table;
pub mod teensy;
pub mod workbook;
pub mod xls;

#[cfg(test)]
mod synthetic;
