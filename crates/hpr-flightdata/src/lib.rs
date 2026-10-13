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
//! ([`raven`]), the Blue Raven's CSV ([`blueraven`]), Missile Works RRC3's ([`rrc3`]) and
//! Eggtimer's ([`eggtimer`]), every column kept as a channel on its own clock. [`readings`] takes a first set of
//! readings from the record: liftoff, apogee, the top speed, landing and the descent, set and
//! tested on PerfectFlite logs. The guide's [reading a flight log][guide-log] page shows them on a
//! log, and [the layouts read][guide-layouts] lists what each reader keeps. The other loggers'
//! formats, spreadsheets and plain CSV are planned for milestone [M7.1c][roadmap] of the roadmap,
//! and the rest of the readings, smoothing and reconstruction for [M7.2][roadmap].
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

pub mod altos;
pub mod blueraven;
pub mod eggtimer;
pub mod error;
pub mod filter;
pub mod import;
pub mod log;
pub mod perfectflite;
pub mod raven;
pub mod readings;
pub mod rrc3;
mod table;

#[cfg(test)]
mod synthetic;
