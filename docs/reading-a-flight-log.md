# Reading a flight log

Read a PerfectFlite altimeter's log with FusionSpace HPR's analyzer to learn how high your rocket
went, how fast it climbed and when it landed.

This page is for anyone with an altimeter's log who wants to know what their rocket did: how high
it went, how fast it climbed, when it landed. FusionSpace HPR · Analyzer reads the log on its own.
It needs no design file and runs no simulation, so it works whatever the rocket was designed in, or
if it was never designed on a computer at all.

**What works today:** logs from PerfectFlite altimeters in their `.pf2` format, the Pnut's, the
StratoLogger's and the StratoLoggerCF's ([the format](format/pf2.md)). The library also reads
PerfectFlite's CSV and text exports, AltOS's CSV, the Featherweight Raven's FIP CSV, the Blue
Raven's CSV, the RRC3's and Eggtimer's (and an Eggtimer TRS's terminal text), PerfectFlite's `.pfa`,
AltimeterCloud's and Entacore AIM's CSV, AerospaceNU's SillyGoose logs and its flight computer
board's CSV and ground station text, Silicdyne Fluctus's CSV, the ARTS2's interpreted data, a
phone app's `launch` CSV, an Eggtimer's download saved as RTF, the Raven's `.FIPa` and the
RRC3's `.rff` raw downloads, the Featherweight app's flight summaries (figures, no samples), the
R-DAS's raw text (its counts kept as written), AltOS's radio telemetry and its `.eeprom` raw downloads, the ARTS2's `.odf` and raw CSV, and GPS tracks from Featherweight's GPS tracker, KML and NMEA files and Missile Works' RTx tracker's `.rtx` file and its exports, and tables from loggers no other reader knows, by the units their headers state, into the same record, those in a CSV layout also from the sheets of an Excel workbook (the library only, not `hpr analyze`), a team flight computer's binary records by a layout the caller states (the library only), a Blue Raven's USB download, each line held to its CRC (the library only), and a course's Teensy data logger, its units held to the flight's physics, and one team's radio telemetry packets, `255:…:254`, a Raven FIP export pasted below no header, under a header you state, an AltiMAX altimeter's `.xls` export and its own `.ALS` file, Entacore AIM's `.xtra` files and raw dumps, a MARSA altimeter's download, AltOS's radio telemetry decoded to a table, and AltOS's CSV re-saved by a spreadsheet, its formulas kept as text, or cut to its columns, and tables whose headers state too little, by a column mapping you state, the sample tables' mappings each holding every unit to a physical check (the library only) ([the layouts read](format/flight-logs.md)); `hpr analyze` takes its
readings from those with [M7.2](decisions-and-roadmap.md#m7-2), which validates them on other
loggers. Formats no sample holds yet, such as a CATS flight computer's, are
[listed as needing samples](format/flight-logs.md#formats-that-need-samples).

> **How far to trust it.** Readings of the altimeter's own log, not a simulation. On an invented flight whose every number is known, the apogee comes
> within a quarter of a meter and one sample of the truth, and liftoff within a tenth of a second.
> The landing is read at the first sample within 2 m (6.6 ft) of the pad, so early by the time the
> last 2 m (6.6 ft) take: 0.33 s at 6 m/s (20 ft/s), 0.5 s at 4 m/s (13 ft/s). On one real flight, a
> public log that isn't committed here and so isn't checked in CI, the analyzer reads 1,010 ft where
> the altimeter states 1,009 ft. The analyzer has no check yet for a barometer's errors near the
> speed of sound. If the flight may have come near Mach 0.9, about 300 m/s (984 ft/s), treat the
> top speed and the heights near it with care: the barometer's error can pull the top speed down
> too, so a low reading doesn't clear it. The rules behind each reading are on
> [Flight-log readings](physics/log-readings.md), with what they were checked against. A reading the
> log can't support is left out and says why, rather than printed as a number.

## From the command line

```bash
hpr analyze flight.pf2
```

prints the readings as text, or as one JSON document with `--json`.
[`hpr analyze`](cli.md#hpr-analyze) shows its output on an example log, and its
[JSON schema](https://github.com/nrdptel/fusionspace-eridanus/blob/main/schema/cli/analyze.schema.json) lists
the fields.

## From a program

The library is `hpr_flightdata`. It reads a log's text into a record in SI units, then takes the
readings from that record. It doesn't depend on the simulator, so a program that only reads logs
doesn't build one. This program reads the invented log the tests use:

<!-- quote: crates/hpr-flightdata/examples/read_a_log.rs -->
```rust
//! A flight log read on its own: the invented PerfectFlite log the tests use, its readings printed
//! with where each came from, or why it was withheld. No design file, no simulation.
//!
//! It uses the workspace crate `hpr-flightdata`, which doesn't pull in the simulator, and `serde`
//! and `serde_json` to print each code as the JSON output spells it.
//!
//! Run it from anywhere in the repository:
//!
//! ```text
//! cargo run --example read_a_log -p fusionspace-hpr-flightdata
//! ```
//!
//! The guide's page *Reading a flight log* (`docs/reading-a-flight-log.md`) quotes it and what it
//! prints, which is kept next to it in `read_a_log.output.txt`; CI checks that the two still agree
//! (`cargo xtask examples --check`).

#![allow(
    clippy::print_stdout,
    reason = "the project's lints forbid printing in library code, and this program exists to print"
)]

use std::error::Error;

use hpr_flightdata::perfectflite::{self, FOOT_M};
use hpr_flightdata::readings::{self, Reading};

/// The invented log: a Pnut's file of a flight made up for the tests.
const LOG: &str = include_str!("../../../validation/fixtures/logs/synthetic-pnut.pf2");

/// A code as the JSON output spells it, such as `no_accelerometer`.
fn code(value: impl serde::Serialize) -> String {
    serde_json::to_value(value)
        .ok()
        .and_then(|json| json.as_str().map(str::to_owned))
        .unwrap_or_default()
}

/// A reading's value as text, or why it was withheld.
fn show<T>(reading: &Reading<T>, value: impl Fn(&T) -> String) -> String {
    match reading {
        Reading::Read(read) => value(read),
        Reading::Withheld(withheld) => {
            format!("withheld ({}): {}", code(withheld.reason), withheld.detail)
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let log = perfectflite::read(LOG)?;
    println!("{}: {} samples", log.logger, log.time_s.len());
    if let Some(stated) = log.stated.apogee_m {
        println!("it states an apogee of {:.0} ft", stated / FOOT_M);
    }

    let read = readings::read(&log);
    println!();
    println!(
        "liftoff           {}",
        show(&read.liftoff, |liftoff| format!("{:.2} s", liftoff.time_s))
    );
    println!(
        "apogee            {}",
        show(&read.apogee, |apogee| format!(
            "{:.1} m ({:.0} ft) at {:.2} s, source: {}",
            apogee.altitude_m,
            apogee.altitude_m / FOOT_M,
            apogee.time_s,
            code(apogee.source)
        ))
    );
    println!(
        "highest sample    {}",
        show(&read.apogee, |apogee| format!(
            "{:.1} m at {:.2} s",
            apogee.highest_sample.altitude_m, apogee.highest_sample.time_s
        ))
    );
    println!(
        "top speed         {}",
        show(&read.max_speed, |speed| format!(
            "{:.1} m/s at {:.2} s, source: {}",
            speed.speed_m_s,
            speed.time_s,
            code(speed.source)
        ))
    );
    println!(
        "top acceleration  {}",
        show(&read.max_acceleration, |top| format!(
            "{:.1} m/s²",
            top.acceleration_m_s2
        ))
    );
    println!(
        "landing           {}",
        show(&read.landing, |landing| format!(
            "{:.2} s; down from apogee at {:.1} m/s on average",
            landing.time_s, landing.mean_descent_rate_m_s
        ))
    );
    Ok(())
}
```

It prints:

<!-- quote: crates/hpr-flightdata/examples/read_a_log.output.txt -->
```text
PerfectFlite Pnut: 984 samples
it states an apogee of 1281 ft

liftoff           0.55 s
apogee            390.1 m (1280 ft) at 10.28 s, source: barometer
highest sample    400.5 m at 11.35 s
top speed         79.9 m/s at 2.10 s, source: logger_speed_from_barometer
top acceleration  withheld (no_accelerometer): a PerfectFlite logger has no accelerometer; HPR Sim doesn't difference the altitude twice to make one, as its one-foot steps would read as spikes of many g
landing           45.85 s; down from apogee at 10.9 m/s on average
```

## What the readings say

The log is of a flight invented for the tests
([the flight](physics/log-readings.md#checked-against)): 80 m/s (262 ft/s) at burnout, 2.1 s after
the start of the log, and a coast with no drag to 390.3 m (1,280.5 ft) at 10.26 s.

- **Liftoff**, 0.55 s: the last sample before the altitude shows the rocket moving. The rocket
  left the pad at 0.50 s, but its first 0.15 m (0.5 ft) rounds to 0 ft.
- **Apogee**, 390.1 m (1,280 ft) at 10.28 s, which is 10.275 s rounded: the top is flat over
  several samples, and the analyzer takes the middle. It is the top of the altitude after a 0.3 s
  [running median](glossary.md#running-median). It is 0.17 m (0.6 ft) below the true apogee: the
  file rounds to whole feet.
- **The highest sample**, 400.5 m (1,314 ft), a second after apogee, is the ejection charge's
  pressure pulse, not the rocket. The median sets it aside.
- **The top speed**, 79.9 m/s (262 ft/s) at 2.10 s: the altimeter's own speed column, which it
  works out from its barometer. The true speed at burnout is 80.0 m/s (262.5 ft/s); the file rounds
  to whole feet per second.
- **The top acceleration** is withheld: a PerfectFlite has no accelerometer.
- **Landing**, 45.85 s: the first sample within 2 m (6.6 ft) of the pad. The rocket touches
  down 0.29 s later, at 6 m/s (20 ft/s) under its main.
- **The mean descent rate**, 10.9 m/s (36 ft/s): the height lost from apogee to landing over the time
  taken, drogue and main together.

What the file states about itself, such as the altimeter's own apogee of 1,281 ft, is kept in
`log.stated`, beside the analyzer's readings and never in their place. The codes, such as
`no_accelerometer` and `barometer`, are the ones the JSON output of `hpr analyze` uses. The top
speed is `max_speed` in both.

## What it doesn't do yet

- Read a format no sample holds yet, such as a CATS flight computer's: the formats that
  need samples are [listed](format/flight-logs.md#formats-that-need-samples), each with what a
  reader would need. Nor does it take readings from the logs the library already reads beside
  PerfectFlite's ([M7.2](decisions-and-roadmap.md#m7-2)).
- Split the descent into the drogue's and the main's rates, find the deployments, or give the
  Mach number and dynamic pressure ([M7.2](decisions-and-roadmap.md#m7-2)).
- Read GPX tracks ([M7.1d1](decisions-and-roadmap.md#m7-1d1)), or take a column mapping or
  a record layout for your own logger anywhere but the Rust library: `hpr analyze` and Python
  come with [M7.1d2](decisions-and-roadmap.md#m7-1d2).
- Compare a flight with its simulation ([M7.3](decisions-and-roadmap.md#m7-3)).
- Check a barometric reading near the speed of sound. Debrief stops trusting one above Mach 0.9;
  the analyzer doesn't check yet.
- Correct a barometric altitude for the day's air. The altitude is the altimeter's own
  conversion, which assumes a standard atmosphere
  ([Barometric altimeter](glossary.md#barometric-altimeter)).
