# Reading a flight log

Read an altimeter's flight log with FusionSpace HPR's analyzer to learn how high your rocket went,
how fast it climbed and when it landed.

This page is for anyone with an altimeter's log who wants to know what their rocket did.
FusionSpace HPR · Analyzer reads the log on its own. It needs no design file and runs no
simulation. So it works whatever the rocket was designed in, or if it was never designed on a
computer at all.

**What works today:** logs from PerfectFlite altimeters in their `.pf2` format, the Pnut's, the
StratoLogger's and the StratoLoggerCF's ([the format](format/pf2.md)). The library also reads
these ([the layouts read](format/flight-logs.md)):

- PerfectFlite's CSV and text exports
- AltOS's CSV
- the Featherweight Raven's FIP CSV
- the Blue Raven's CSV
- the RRC3's and Eggtimer's (and an Eggtimer TRS's terminal text)
- PerfectFlite's `.pfa`
- AltimeterCloud's and Entacore AIM's CSV
- AerospaceNU's SillyGoose logs and its flight computer board's CSV and ground station text
- Silicdyne Fluctus's CSV
- the ARTS2's interpreted data
- a phone app's `launch` CSV
- an Eggtimer's download saved as RTF
- the Raven's `.FIPa` and the RRC3's `.rff` raw downloads
- the Featherweight app's flight summaries (figures, no samples)
- the R-DAS's raw text (its counts kept as written)
- AltOS's radio telemetry and its `.eeprom` raw downloads
- the ARTS2's `.odf` and raw CSV
- GPS tracks from Featherweight's GPS tracker, KML and NMEA files, and Missile Works' RTx
  tracker's `.rtx` file and its exports
- GPS tracks in GPX files, checked on invented files only
  ([the evidence register's row on GPX](evidence.md#g1))
- tables from loggers no other reader knows, by the units their headers state, into the same record
- those tables in a CSV layout also from the sheets of an Excel workbook
- a team flight computer's binary records, by a layout the caller states
- a Blue Raven's USB download, each line held to its CRC
- a course's Teensy data logger, its units held to the flight's physics
- one team's radio telemetry packets, `255:…:254`
- a Raven FIP export pasted below no header, under a header you state
- an AltiMAX altimeter's `.xls` export and its own `.ALS` file
- Entacore AIM's `.xtra` files and raw dumps
- a MARSA altimeter's download
- AltOS's radio telemetry decoded to a table
- AltOS's CSV re-saved by a spreadsheet, its formulas kept as text, or cut to its columns
- tables whose headers state too little, by a column mapping you state; the sample tables'
  mappings each hold every unit to a physical check

`hpr analyze` and Python read each layout the library tells from the file alone, each by its own
reader. They read one sheet of a workbook or one recording of a raw dump at a time
([below](#a-file-of-several-records)). They read a table or binary records by a column mapping or
a record layout ([Reading your own logger](your-own-logger.md)). A zip archive's files must be
unpacked first.

Since [readings on every logger](decisions-and-roadmap.md#m7-2b) shipped, they come from any
logger's barometric altitude or pressure, each within a stated bound. A GPS track gives none.
Since [the rest of the readings](decisions-and-roadmap.md#m7-2c) shipped, they include burnout,
the flight time and each canopy's descent rate. They also include the top acceleration from an
accelerometer along the rocket's axis. Formats no sample holds yet, such as a CATS flight
computer's, are [listed as needing samples](format/flight-logs.md#formats-that-need-samples)
([the evidence register's row on them](evidence.md#g9)).

> **How far to trust it.** Readings of the altimeter's own log, measured, not simulated. Each
> states a bound on its error, held on HPR Sim's synthetic flights. They're 171 simulated flights,
> 71 with an accelerometer too, each logged at every rate from 500 to half a sample a second. On
> them, every reading stayed within its bound, and the largest error took 0.717 of it.
>
> The bounds were tuned on those same flights, and no real log has a truth beside it to test
> them. They hold on [six simulated logger profiles](physics/synthetic-flights.md#the-loggers) and
> on rockets leaving the pad at 2 g or more. A noisier logger or a slower rocket can be read
> outside them.
>
> One real flight's log is public, but it isn't committed here, so it isn't checked in CI. On it,
> the analyzer reads the apogee as 1,009 ft above the pad, as the altimeter states it. Those are
> two readings of one pressure trace, not a measurement of the height
> ([the evidence register's row on the analyzer](evidence.md#g10)).
>
> Most of an apogee's bound is the day's air. A barometric altimeter turns pressure into height by
> the standard atmosphere. So on a day 15 K from standard, its heights are off by up to 6.92%.
>
> The top speed is worked out from the heights. It's withheld from 227.7 m/s (747 ft/s), Mach 0.8
> in the coldest air the bounds allow ([the top speed](physics/log-readings.md#each-reading)). At
> those speeds a barometer's port may read the flow over the nose.
>
> If a charge's spike climbs into the top no faster than a coast can, it's read as the apogee.
> That holds however high it stands
> ([#761](https://github.com/nrdptel/fusionspace-eridanus/issues/761), a charge's transient at the
> top). Some tops withheld as `disturbed_top` are the rocket's own
> ([#762](https://github.com/nrdptel/fusionspace-eridanus/issues/762), telling a coast's own top
> from a disturbed one). The main's opening leaves out a canopy's filling time
> ([#779](https://github.com/nrdptel/fusionspace-eridanus/issues/779)). A fall under no canopy is
> read as one canopy's ([#780](https://github.com/nrdptel/fusionspace-eridanus/issues/780)).
>
> A board's three axes give no top acceleration
> ([#781](https://github.com/nrdptel/fusionspace-eridanus/issues/781)). For a record or a
> certification, the altimeter's own reading is the one to log. The rules behind each reading are
> on [Flight-log readings](physics/log-readings.md), with what they were checked against. A
> reading the log can't support is left out and says why, rather than printed as a number.

## From the command line

```bash
hpr analyze flight.pf2
```

reads any layout the library tells from the file alone. It prints the readings as text, each
with the bound its error stays within ("within …"), or as one JSON document with `--json`.
[`hpr analyze`](cli.md#hpr-analyze) shows its output on an example log, and its
[JSON schema](https://github.com/nrdptel/fusionspace-eridanus/blob/main/schema/cli/analyze.schema.json) lists
the fields.

Another logger's table is read by a column mapping, `--mapping`, and its binary records by a
record layout, `--layout`. Each is a JSON file you write.
[Reading your own logger](your-own-logger.md) works through both on an invented logger.

### A file of several records

A spreadsheet's workbook holds sheets. The workbook is an `.xlsx`, or an `.xls` written as one
bare worksheet (as an AltiMAX altimeter's Excel export is). An Entacore AIM raw dump can hold
several recordings.

`hpr analyze` reads one at a time, each by its own reader. With no choice given, it reads the
workbook's one sheet that reads as a log, or the dump's one recording. Where there are several,
or no sheet reads, it refuses and names them, a sheet by its number and name, such as
`2 (Second)`. Choose one, counted from 0:

```bash
hpr analyze flight.xlsx --sheet 2
hpr analyze flight.dump --recording 1
```

From Python, `FlightLog.read(path, sheet=2)` and `FlightLog.read(path, recording=1)` do the same.
A Featherweight Raven's FIP export that lost its header line when it was pasted into a workbook
reads too. It's read under a header you state, `--fip-header`
([Reading your own logger](your-own-logger.md#a-fip-export-below-no-header)).

## From a program

The library is `hpr_flightdata`. It reads a log's text into a record in SI units, then takes the
readings from that record. It doesn't depend on the simulator, so a program that only reads logs
doesn't build one. This program reads the invented log the tests use, and prints each reading's
value and where it came from. Each reading also carries its bound, as `hpr analyze` prints it:

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
        "burnout           {}",
        show(&read.burnout, |burnout| if burnout.is_floor {
            // A floor: the pair printed is the reading and its bound, and the floor, their
            // difference, is the soonest burnout can have come.
            format!(
                "no sooner than {:.2} s less {:.2} s, source: {}",
                burnout.time_s,
                burnout.bound_s,
                code(burnout.source)
            )
        } else {
            format!("{:.2} s, source: {}", burnout.time_s, code(burnout.source))
        })
    );
    println!(
        "landing           {}",
        show(&read.landing, |landing| format!(
            "{:.2} s; down from apogee at {:.1} m/s on average",
            landing.time_s, landing.mean_descent_rate_m_s
        ))
    );
    println!(
        "flight time       {}",
        show(&read.flight_time, |flight| format!(
            "{:.2} s",
            flight.time_s
        ))
    );
    println!(
        "drogue            {}",
        show(&read.drogue_descent, |leg| format!(
            "{:.1} m/s",
            leg.rate_m_s
        ))
    );
    println!(
        "main              {}",
        show(&read.main_descent, |leg| match &leg.opening {
            Some(opening) => format!(
                "{:.1} m/s, opened at {:.2} s, {:.0} m up",
                leg.rate_m_s, opening.time_s, opening.height_m
            ),
            None => format!("{:.1} m/s", leg.rate_m_s),
        })
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
burnout           no sooner than 2.20 s less 1.32 s, source: barometer
landing           45.80 s; down from apogee at 10.9 m/s on average
flight time       45.00 s
drogue            21.7 m/s
main              6.0 m/s, opened at 21.40 s, 148 m up
```

## What the readings say

The log is of a flight invented for the tests
([the flight](physics/log-readings.md#checked-against)). It reaches 80 m/s (262 ft/s) at burnout,
2.1 s after the start of the log. Then it coasts with no drag to 390.3 m (1,280.5 ft) at 10.26 s.
Each bound below is the one [`hpr analyze`](cli.md#hpr-analyze) prints for the same log. A
bound's feet are converted exactly here, where `hpr analyze` rounds them up: the drogue's bound
below reads 7.2 ft/s here and 8 ft/s there.

- **Liftoff**, 0.80 s, within 0.81 s: the last sample before the altitude climbs out of the pad's
  noise allowance. The allowance is at least 2 m (6.6 ft) and a step. The rocket left the pad at
  0.50 s; it took 0.30 s to climb that far.
- **Apogee**, 390.1 m (1,280 ft) at 10.28 s, which is 10.275 s rounded. The top is flat over
  several samples, so the analyzer takes their middle as its time. It's the top of the altitude
  after a 0.3 s [running median](glossary.md#running-median). It's 0.17 m (0.56 ft) below the true
  apogee, well within its 31.9 m (105 ft) bound. Most of that bound allows for the day's air.
- **The highest sample**, 400.5 m (1,314 ft), a second after apogee, is the ejection charge's
  pressure pulse, not the rocket. The median sets it aside.
- **The top speed**, 78.3 m/s (257 ft/s) at 2.20 s, within 20.6 m/s (68 ft/s). It's worked out
  from the heights, as the steepest slope of a 0.5 s fit, not read from the altimeter's speed
  column. The true speed at burnout is 80.0 m/s (262.5 ft/s); the fit across the corner at burnout
  reads it a little low.
- **The top acceleration** is withheld: a PerfectFlite has no accelerometer. HPR Sim doesn't
  difference the heights twice to make one, as their one-foot steps would read as spikes of many g.
- **Burnout**, no sooner than 0.88 s: the top speed's time, 2.20 s, less its 1.32 s bound. With no
  accelerometer, burnout is a [floor](glossary.md#floor): its reading is the top speed's time, and
  the floor that time less its bound. A rocket gains speed only while its thrust beats the drag and
  its weight. So the motor burned out no sooner than that time less the bound. It may have burned
  on after the top speed's time by any amount. The true burnout is 2.10 s.

  The floor is loose by design: on the synthetic flights' tuning set, the top speed came 0.24 s
  before the true burnout on average. The top speed's time ranged from 1.33 s before the true
  burnout to 0.718 s after it. The floor lies its bound, about 1.3 s, earlier still
  ([Checked against](physics/log-readings.md#checked-against)).
- **Landing**, 45.80 s, within 2.11 s: the first sample back within the pad's allowance. The
  rocket touches down 0.34 s later, at 6 m/s (20 ft/s) under its main.
- **The flight time**, 45.00 s, within 2.91 s: landing's time less liftoff's. The true flight time
  is 45.64 s: liftoff is read 0.30 s late and landing 0.34 s early.
- **The mean descent rate**, 10.9 m/s (36 ft/s), within 2.1 m/s (6.9 ft/s). It's the height lost
  from apogee to landing over the time taken, drogue and main together.
- **The drogue's descent**, 21.7 m/s (71 ft/s), within 2.2 m/s (7.2 ft/s), from apogee at 10.28 s
  to the main's opening at 21.40 s. The rocket fell under its drogue at 25 m/s (82 ft/s), but the
  leg starts at apogee, where it falls from rest. Over the same span, the leg's true rate is
  21.74 m/s (71.3 ft/s).
- **The main's descent**, 6.0 m/s (20 ft/s), within 0.8 m/s (2.6 ft/s), from its opening to
  landing, as the flight's main truly fell. The opening is at 21.40 s within 1.06 s, 148.4 m
  (487 ft) up. It's found by walking back from landing until the descent's rate climbs well past
  the main's. The main truly opened at 21.14 s, 150 m (492 ft) up, so the reading is 0.26 s late.
  It's late because the rate is a slope fitted over 1 s. The slope still sees some of the drogue's
  faster fall just after the opening.

What the file states about itself, such as the altimeter's own apogee of 1,281 ft, is kept in
`log.stated`. It sits beside the analyzer's readings, never in their place.

The codes, such as `no_accelerometer` and `barometer`, are the ones the JSON output of
`hpr analyze` uses. The top speed is `max_speed` in both, the top acceleration `max_acceleration`,
and the canopies' legs `drogue_descent` and `main_descent`. A reading that is a floor carries
`is_floor`, and a clipped top acceleration says why in `clip`, `range` or `flat`. Python's
`FlightLog.readings()` returns the same readings, bounds included, as a dict
([From Python](your-own-logger.md#from-python)).

Every code a reading can be withheld with is in
[the table of reasons](physics/log-readings.md#when-a-reading-is-withheld), with when it applies.
Among them are those that came with the accelerometer and the canopies:

- `axis_unstated` (no accelerometer named along the rocket)
- `gravity_unstated` (whether its readings hold gravity)
- `direction_unstated` (which way up it reads)
- `one_canopy` (no drogue's leg in the descent)
- `short_leg` (a leg too short to read)

## What it doesn't do yet

- Read a format no sample holds yet, such as a CATS flight computer's
  ([the evidence register's row on them](evidence.md#g9)). The formats that need samples are
  [listed](format/flight-logs.md#formats-that-need-samples), each with what a reader would need. A
  sample can be offered in [#662](https://github.com/nrdptel/fusionspace-eridanus/issues/662).
- Find the deployments as events: the main's opening is read from the descent's rate, not from
  the log's record of when each charge fired.
- Give the Mach number and dynamic pressure, or state a 90% interval on a reading
  ([the planned 90% intervals](decisions-and-roadmap.md#m7-2e)).
- Take the top acceleration from a board's three axes, which don't say which lies along the
  rocket ([#781](https://github.com/nrdptel/fusionspace-eridanus/issues/781)). It comes only from
  an accelerometer the log names as the rocket's axis
  ([the accelerometer](physics/log-readings.md#the-accelerometer)).
- Allow for a canopy's filling time in the main opening's bound
  ([#779](https://github.com/nrdptel/fusionspace-eridanus/issues/779)).
- Tell a fall under no canopy from one under a canopy: it's read as one canopy's descent
  ([#780](https://github.com/nrdptel/fusionspace-eridanus/issues/780)).
- Take a reading from a GPS track: a receiver's altitude lags the climb, so no bound can be
  stated for it.
- Compare a flight with its simulation
  ([the planned comparison](decisions-and-roadmap.md#m7-3)).
- Check the heights near the speed of sound. It withholds the top speed from 227.7 m/s
  (747 ft/s), Mach 0.8 in the coldest air the bounds allow
  ([the top speed](physics/log-readings.md#each-reading)). It doesn't withhold the heights read
  there.
- Hold its bounds beyond what they were checked on: six simulated logger profiles
  ([the loggers](physics/synthetic-flights.md#the-loggers)) and rockets leaving the pad at 2 g or
  more. A noisier logger or a slower rocket can be read outside its bound.
- Tell a charge's spike from the top when the spike climbs into the top no faster than a coast
  can. Such a spike is read as the apogee, however high it stands
  ([#761](https://github.com/nrdptel/fusionspace-eridanus/issues/761), a charge's transient at the
  top).
- Tell every disturbed top from the rocket's own. Some tops withheld as `disturbed_top` are the
  rocket's own ([#762](https://github.com/nrdptel/fusionspace-eridanus/issues/762), telling a
  coast's own top from a disturbed one).
- Correct a barometric altitude for the day's air. The altitude is the altimeter's own
  conversion, which assumes a standard atmosphere
  ([Barometric altimeter](glossary.md#barometric-altimeter)). The bound allows for air up to 15 K
  from standard instead.
