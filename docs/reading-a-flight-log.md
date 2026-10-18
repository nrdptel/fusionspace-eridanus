# Reading a flight log

Read an altimeter's flight log with FusionSpace HPR's analyzer to learn how high your rocket went,
how fast it climbed and when it landed.

This page is for anyone with an altimeter's log who wants to know what their rocket did: how high
it went, how fast it climbed, when it landed. FusionSpace HPR · Analyzer reads the log on its own.
It needs no design file and runs no simulation, so it works whatever the rocket was designed in, or
if it was never designed on a computer at all.

**What works today:** logs from PerfectFlite altimeters in their `.pf2` format, the Pnut's, the
StratoLogger's and the StratoLoggerCF's ([the format](format/pf2.md)), and from the other loggers
below. The library also reads
PerfectFlite's CSV and text exports, AltOS's CSV, the Featherweight Raven's FIP CSV, the Blue
Raven's CSV, the RRC3's and Eggtimer's (and an Eggtimer TRS's terminal text), PerfectFlite's `.pfa`,
AltimeterCloud's and Entacore AIM's CSV, AerospaceNU's SillyGoose logs and its flight computer
board's CSV and ground station text, Silicdyne Fluctus's CSV, the ARTS2's interpreted data, a
phone app's `launch` CSV, an Eggtimer's download saved as RTF, the Raven's `.FIPa` and the
RRC3's `.rff` raw downloads, the Featherweight app's flight summaries (figures, no samples), the
R-DAS's raw text (its counts kept as written), AltOS's radio telemetry and its `.eeprom` raw
downloads, the ARTS2's `.odf` and raw CSV, and GPS tracks from Featherweight's GPS tracker, KML and
NMEA files, Missile Works' RTx tracker's `.rtx` file and its exports, and GPX files (checked on
invented files only, [evidence row G1](evidence.md#g1)), and tables from loggers no other reader knows, by the units their headers
state, into the same record, those in a CSV layout also from the sheets of an Excel workbook, a team
flight computer's binary records by a layout the caller states, a Blue Raven's USB download, each
line held to its CRC, and a course's Teensy data logger, its units held to the flight's physics, and
one team's radio telemetry packets, `255:…:254`, a Raven FIP export pasted below no header, under a
header you state, an AltiMAX altimeter's `.xls` export and its own `.ALS` file, Entacore AIM's
`.xtra` files and raw dumps, a MARSA altimeter's download, AltOS's radio telemetry decoded to a
table, and AltOS's CSV re-saved by a spreadsheet, its formulas kept as text, or cut to its columns,
and tables whose headers state too little, by a column mapping you state, the sample tables'
mappings each holding every unit to a physical check ([the layouts read](format/flight-logs.md)).
`hpr analyze` and Python read each layout the library tells from the file alone by its own reader,
one sheet of a workbook or one recording of a raw dump at a time
([below](#a-file-of-several-records)), and a table or binary records by a column mapping or a
record layout ([Reading your own logger](your-own-logger.md)). A zip archive's files must be
unpacked first. Since
[M7.2b, readings on every logger](decisions-and-roadmap.md#m7-2b), the readings come from any
logger's barometric altitude or pressure, each within a stated bound; a GPS track gives none.
Formats no sample holds yet,
such as a CATS flight computer's, are [listed as needing
samples](format/flight-logs.md#formats-that-need-samples) ([evidence row G9](evidence.md#g9)).

> **How far to trust it.** Readings of the altimeter's own log, measured, not simulated. Each
> states a bound on its error, held on HPR Sim's synthetic flights: on 171 simulated flights, each
> logged at every rate from 500 to half a sample a second, every reading stayed within its bound,
> and the largest error took 0.717 of it. The bounds were tuned on those same flights, and no real
> log has a truth beside it to test them. On one real flight, a public log that isn't committed
> here and so isn't checked in CI, the analyzer reads the apogee as 1,009 ft above the pad, as the
> altimeter states it: two readings of one pressure trace, not a measurement of the height
> ([evidence row G10](evidence.md#g10)). Most of an apogee's bound is the day's air: a barometric
> altimeter turns pressure into height by the standard atmosphere, so on a day 15 K from standard
> its heights are off by up to 6.92%. The top speed is worked out from the heights and withheld
> from 227.7 m/s (747 ft/s), where a barometer's port may read the flow over the nose. For a
> record or a certification, the altimeter's own reading is the one to log. The rules behind each
> reading are on [Flight-log readings](physics/log-readings.md), with what they were checked
> against. A reading the log can't support is left out and says why, rather than printed as a
> number.

## From the command line

```bash
hpr analyze flight.pf2
```

reads any layout the library tells from the file alone, and prints the readings as text, each with
the bound its error stays within ("within …"), or as one JSON document with `--json`.
[`hpr analyze`](cli.md#hpr-analyze) shows its output on an example log, and its
[JSON schema](https://github.com/nrdptel/fusionspace-eridanus/blob/main/schema/cli/analyze.schema.json) lists
the fields. Another logger's table is read by a column mapping, `--mapping`, and its binary
records by a record layout, `--layout`: each a JSON file you write, as
[Reading your own logger](your-own-logger.md) works through on an invented logger.

### A file of several records

A spreadsheet's workbook, an `.xlsx` or an `.xls` written as one bare worksheet (as an AltiMAX
altimeter's Excel export is), holds sheets, and an Entacore AIM raw dump can hold several recordings. `hpr analyze`
reads one at a time, each by its own reader. With no choice given, it reads the workbook's one
sheet that reads as a log, or the dump's one recording. Where there are several, or no sheet
reads, it refuses and names them, a sheet by its number and name, such as `2 (Second)`. Choose
one, counted from 0:

```bash
hpr analyze flight.xlsx --sheet 2
hpr analyze flight.dump --recording 1
```

From Python, `FlightLog.read(path, sheet=2)` and `FlightLog.read(path, recording=1)` do the same.
A Featherweight Raven's FIP export that lost its header line when it was pasted into a workbook is
read under a header you state, `--fip-header`
([Reading your own logger](your-own-logger.md#a-fip-export-below-no-header)).

## From a program

The library is `hpr_flightdata`. It reads a log's text into a record in SI units, then takes the
readings from that record. It doesn't depend on the simulator, so a program that only reads logs
doesn't build one. This program reads the invented log the tests use, and prints each reading's
value and where it came from; each also carries its bound, as `hpr analyze` prints it:

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

liftoff           0.80 s
apogee            390.1 m (1280 ft) at 10.28 s, source: barometer
highest sample    400.5 m at 11.35 s
top speed         78.3 m/s at 2.20 s, source: barometer
top acceleration  withheld (no_accelerometer): the log has no accelerometer; HPR Sim doesn't difference a height twice to make one, as its steps would read as spikes of many g
landing           45.80 s; down from apogee at 10.9 m/s on average
```

## What the readings say

The log is of a flight invented for the tests
([the flight](physics/log-readings.md#checked-against)): 80 m/s (262 ft/s) at burnout, 2.1 s after
the start of the log, and a coast with no drag to 390.3 m (1,280.5 ft) at 10.26 s. Each bound
below is the one [`hpr analyze`](cli.md#hpr-analyze) prints for the same log.

- **Liftoff**, 0.80 s, within 0.81 s: the last sample before the altitude climbs out of the pad's
  noise allowance, at least 2 m (6.6 ft) and a step. The rocket left the pad at 0.50 s; it took 0.30 s to
  climb that far.
- **Apogee**, 390.1 m (1,280 ft) at 10.28 s, which is 10.275 s rounded: the top is flat over
  several samples, and the analyzer takes the middle. It is the top of the altitude after a 0.3 s
  [running median](glossary.md#running-median). It is 0.17 m (0.56 ft) below the true apogee, well
  within its 31.9 m (105 ft) bound, most of which allows for the day's air.
- **The highest sample**, 400.5 m (1,314 ft), a second after apogee, is the ejection charge's
  pressure pulse, not the rocket. The median sets it aside.
- **The top speed**, 78.3 m/s (257 ft/s) at 2.20 s, within 20.6 m/s (68 ft/s): worked out from
  the heights, as the steepest slope of a 0.5 s fit, not read from the altimeter's speed column.
  The true speed at burnout is 80.0 m/s (262.5 ft/s); the fit across the corner at burnout reads it
  a little low.
- **The top acceleration** is withheld: a PerfectFlite has no accelerometer.
- **Landing**, 45.80 s, within 2.11 s: the first sample back within the pad's allowance. The
  rocket touches down 0.34 s later, at 6 m/s (20 ft/s) under its main.
- **The mean descent rate**, 10.9 m/s (36 ft/s), within 2.1 m/s (6.9 ft/s): the height lost from
  apogee to landing over the time taken, drogue and main together.

What the file states about itself, such as the altimeter's own apogee of 1,281 ft, is kept in
`log.stated`, beside the analyzer's readings and never in their place. The codes, such as
`no_accelerometer` and `barometer`, are the ones the JSON output of `hpr analyze` uses. The top
speed is `max_speed` in both. Python's `FlightLog.readings()` returns the same readings, bounds
included, as a dict ([From Python](your-own-logger.md#from-python)).

## What it doesn't do yet

- Read a format no sample holds yet, such as a CATS flight computer's
  ([evidence row G9](evidence.md#g9)): the formats that need samples are
  [listed](format/flight-logs.md#formats-that-need-samples), each with what a reader would need, and
  a sample can be offered in [#662](https://github.com/nrdptel/fusionspace-eridanus/issues/662).
- Take the top acceleration from an accelerometer, burnout, or each canopy's descent rate, or
  find the deployments ([M7.2c, the rest of the readings](decisions-and-roadmap.md#m7-2c)); give
  the Mach number and dynamic pressure; or state a 90% interval on a reading
  ([M7.2e](decisions-and-roadmap.md#m7-2e)).
- Take a reading from a GPS track: a receiver's altitude lags the climb, so no bound can be
  stated for it.
- Compare a flight with its simulation ([M7.3](decisions-and-roadmap.md#m7-3)).
- Check the heights near the speed of sound: it withholds the top speed from 227.7 m/s
  (747 ft/s), but not the heights read there.
- Correct a barometric altitude for the day's air. The altitude is the altimeter's own
  conversion, which assumes a standard atmosphere
  ([Barometric altimeter](glossary.md#barometric-altimeter)); the bound allows for air up to 15 K
  from standard instead.
