# Exporting a flight

This page shows how to save a flight as files other programs read: a table of numbers over time
for a spreadsheet or a plotting tool (CSV, JSON or Parquet), and a map of the flight path and the
landing (GeoJSON or KML, which Google Earth, QGIS and most web maps open). It flies the rocket of
[Getting started](getting-started.md) again and writes all five files. It needs the first page's
setup, and a little Rust.

Without writing Rust, `hpr sim --export` writes the same five formats for a `.ork` or HPR design
file, every quantity HPR Sim tracks in each ([The command line](cli.md#exporting-the-recording)).
Its maps mark the landing when a parachute or streamer opened, as a `.ork` file's do, and no
landing otherwise ([the landing](cli.md#the-landing)).

> **How far to trust it.** Exact copies of a simulation that is not validated. Every number in a
> file reads back to exactly the value the simulator computed, and tests check that. The flight
> itself is the first flight's, and
> [Accuracy of these numbers](getting-started.md#accuracy-of-these-numbers) on that page applies
> to it too.

## Run it

```bash
cargo run --example export_flight -p fusionspace-hpr-sim --features parquet -- my-flight
```

This runs
[`crates/hpr-sim/examples/export_flight.rs`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/crates/hpr-sim/examples/export_flight.rs),
which writes five files into the folder `my-flight` (without a folder, into `fusionspace-hpr-export` in
the system's temporary folder) and prints what it wrote. Parquet is an optional
[feature](https://doc.rust-lang.org/cargo/reference/features.html) of `fusionspace-hpr-sim`, named
`parquet`, so the command turns it on; without it, cargo says the example needs it. The feature adds
no other library; it is optional so that a program with no use for a binary file format leaves it
out. In your own program, turn it on in `Cargo.toml` with
`fusionspace-hpr-sim = { ..., features = ["parquet"] }` (the crate your code calls `hpr_sim`).

<!-- quote: crates/hpr-sim/examples/export_flight.output.txt -->
```text
wrote 67 rows of 5 columns, a path of 67 points
  flight.csv
  flight.json
  flight.parquet, 3228 bytes
  flight.geojson
  flight.kml
landed at 32.99000° N, 106.96904° W, 90 m from the pad, at 58.9 s
```

CI runs it on macOS, Windows and Linux and fails if it prints anything else. The files hold every
number to its last digit, which may differ in the last place between operating systems, so the
program prints a rounded summary instead of the files.

## The five files

| file | what it holds | opens in |
|---|---|---|
| `flight.csv` | a header giving each column in words with its unit in brackets (`time [s]`, `cg east [m]`, ...), then one line per recorded moment, lines ending in CRLF as RFC 4180 says | a spreadsheet, pandas, any plotting tool |
| `flight.json` | the same table as `{"tool": {...}, "columns": [...], "rows": [[...], ...]}`, each column's unit in its name (`time_s`, `cg_east_m`) | any programming language |
| `flight.parquet` | the same table in [Apache Parquet](https://parquet.apache.org/), a binary format that stores it column by column: one column of 64-bit numbers per recorded column, with the CSV header's names, and the program that wrote it in the file's key-value metadata | [pandas](https://pandas.pydata.org/) with [pyarrow](https://arrow.apache.org/docs/python/) (Apache Arrow's Python library), [DuckDB](https://duckdb.org/) |
| `flight.geojson` | the flight path as a line, and a point for each landing: the rocket's and any separated body's; and a `tool` object naming the program that wrote it | QGIS, geojson.io, web maps |
| `flight.kml` | the same path and landing | Google Earth |

### Which program wrote a file

Each file but the CSV names the program that wrote it, its version and its designation in the
FusionSpace product system, the way a drawing's title block names the tool that made it:

- **JSON and GeoJSON:** a `tool` object at the top,
  `{"name": "FusionSpace HPR", "version": "0.1.0", "designation": "FS-ACHERNAR \u00b7 SW \u00b7 TOOL 001"}`.
  The text is ASCII, so the designation's middle dot `·` is written as its escape `\u00b7`, which
  every JSON reader reads back as the dot. In GeoJSON the object is a *foreign member*, a key the
  standard doesn't define, which RFC 7946 (section 6.1) allows and map programs pass over.
- **KML:** a comment on the second line, `<!-- FusionSpace HPR 0.1.0 · FS-ACHERNAR · SW · TOOL 001 -->`.
- **Parquet:** three entries of the file's key-value metadata: `tool` (`FusionSpace HPR`), `tool_version`
  and `designation`. pyarrow shows them with `pyarrow.parquet.read_metadata(path).metadata`.

The CSV holds only the header and the rows, so a spreadsheet opens it cleanly. Its header gives
each column in words with its unit in brackets, as the FusionSpace product system writes one:
`time [s]` for the column the other formats call `time_s`, `velocity east [m/s]` for
`velocity_east_m_s`, `acceleration up [m/s^2]` for `acceleration_up_m_s2` and `dynamic pressure
[Pa]` for `dynamic_pressure_pa`; a column with no unit, such as `mach`, is its words alone. The
library's `export::csv_header` gives one from the other. `hpr sim` writes the title block beside
the CSV instead: `flight.meta.json` for `flight.csv`, with the same `tool` object, the CSV's file
name, the design and configuration flown, the number of rows, and:

- `catalog_as_of`: the day the bundled motor catalog's curve files were downloaded, such as
  `2026-09-17`, which a build fixes. Whether a motor came from the catalog is in `hpr sim
  --json`'s `motors`, whose text line says "from the bundled catalog, as of 2026-09-17". The
  day of the run itself is left out, so the same run writes the same bytes.
- `kind` and `trust`: what the numbers are (`simulated`) and the
  [*How far to trust it*](cli.md#the-accuracy-note-on-every-result) note `hpr sim` ends with.

A JSON recording from `hpr sim` carries the same four, with `design` and `configuration`, between
`tool` and `columns`, so a saved flight keeps its note. The library's `export::csv` writes the
CSV alone, and `export::json` the rows alone; `export::json_with` adds a program's own fields.

To look at the Parquet file from Python, install pandas and pyarrow (`pip install pandas pyarrow`;
pandas can't read Parquet on its own), then:

```bash
python -c "import pandas; print(pandas.read_parquet('my-flight/flight.parquet'))"
```

or, with DuckDB's command-line program, `duckdb -c "SELECT * FROM 'my-flight/flight.parquet'"`.

The tables hold whatever [channels](recording-a-trajectory.md#record-something-else) the
recorder kept, one row per recorded moment. The maps need the recorder to keep the time and the
center of gravity's position (`Channel::Time` and `Channel::CgPosition`); they place each recorded
position on the Earth, with the same conversion as the landing point in
[Flight metrics](physics/metrics.md).

A map line in KML looks like this, one `longitude,latitude,height` per recorded moment (the digits
here are cut short):

```xml
<LineString>
<altitudeMode>absolute</altitudeMode>
<coordinates>
-106.97,32.99,1400.93591...
-106.97,32.98999...,1403.85869...
```

## Heights: two datums

A height needs something to count from, a datum. The two map formats use different ones, because
their standards say so:

- **GeoJSON** heights are above the WGS 84 ellipsoid, the smooth shape GPS uses
  ([ellipsoidal height](glossary.md#ellipsoidal-height);
  [RFC 7946](https://www.rfc-editor.org/rfc/rfc7946#section-4), section 4).
- **KML** heights with `altitudeMode` `absolute` are above sea level, which KML takes from the
  EGM96 geoid ([height above sea level](glossary.md#height-above-sea-level-msl);
  [OGC KML 2.2, 07-147r2](https://www.ogc.org/standard/kml/)).

Sea level sits above or below the ellipsoid by the
[geoid undulation](glossary.md#height-above-sea-level-msl) `N`, up to about 100 m. The simulator has
no model of it, so it uses the value the flight was given (`Environment::with_geoid_undulation_m`,
zero unless set), the same for every point of the flight. The geoid's slope, about 5 cm per
kilometer and up to some 30 cm in mountains, moves it by centimeters to decimeters over a rocket's
few kilometers. For example, at a site where sea level
is 25 m below the ellipsoid (`N = −25` m), a point 1500 m above the ellipsoid is written as 1500 m
in GeoJSON and as 1525 m in KML. The first flight leaves `N` at zero, so its two files agree; at the real
site sea level is some tens of meters below the ellipsoid, so give `N` for heights you mean to
trust.

## How the files are checked

- **Numbers:** each is written in the shortest form that reads back to the same number, and the
  tests read every file back and compare it with the recording exactly, not within a tolerance. A
  value that isn't a finite number is refused with an error rather than written as `NaN` or
  `null`, which different programs read differently.
- **GeoJSON:** checked against the published GeoJSON schema
  ([geojson.org/schema](https://geojson.org/schema/FeatureCollection.json)), with longitude
  before latitude as the standard requires. A test also shows the check rejects a broken file.
- **KML:** parsed by a strict XML parser, and checked for the KML 2.2 namespace, the height mode
  and every coordinate.
- **Parquet:** HPR Sim writes the file itself, from the format's specification. The tests read it
  back with an independent reader, Apache's own Parquet library for Rust, and compare every number
  bit for bit. One test uses a recording of every channel (30 columns today) long enough to fill at
  least three [data pages](glossary.md#parquet-data-page) per column, and another compares a small
  file with bytes worked out by hand from the specification. Once, by hand, when this was written,
  pyarrow 25.0.1 and DuckDB 1.5.5 each read the example's file and a 30-column, 3005-row file and
  got the same numbers as the matching CSV. CI does not repeat that check.

The tests are in
[`crates/hpr-sim/src/export.rs`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/crates/hpr-sim/src/export.rs)
and
[`crates/hpr-sim/src/export/parquet.rs`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/crates/hpr-sim/src/export/parquet.rs).

## What it leaves out

- **Parquet compression and statistics:** the Parquet file is not compressed and carries no
  per-page minimum and maximum. It takes 8 bytes per number, plus a header of about 20 bytes per
  page of up to 1024 numbers and a short index of the columns at the end of the file: the
  example's is 3212 bytes, and its CSV about 5.5 kB. Without the minimums and maximums, a tool
  such as DuckDB can't skip parts of the file when filtering (say `WHERE time_s > 10`) and reads
  it all, which for one flight takes no noticeable time.
- **A path over the antimeridian** (±180° longitude) is not cut in two as RFC 7946 asks, so a map
  would draw it the long way round the globe.
- **Landing heights:** a landing is a point on the ground, without a height.
- **A summary file:** the flight's peaks and margins ([Flight metrics](physics/metrics.md)) are
  not in these files. A `FlightSummary` converts to JSON on its own with `serde_json::to_string`,
  but that path writes a value that isn't finite as `null` rather than refusing it.

The choices are recorded in
[ADR-079: exports as text built in the core](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0079-exports-as-text-built-in-the-core-heights-on.md)
and
[ADR-080: Parquet written in-house, read back by Apache's library](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0080-parquet-written-in-house-read-back-by-apaches.md).

## The program

This is the whole program, line for line the file CI runs. Everything above the recorder is the
first flight's setup, which [Getting started](getting-started.md#the-program-step-by-step)
explains step by step.

<!-- quote: crates/hpr-sim/examples/export_flight.rs -->
```rust
//! A flight written out for other programs: the flight of `first_flight.rs`, recorded every 1 s
//! and at every event, saved as CSV, JSON and Parquet tables and as a GeoJSON and a KML map of its
//! path and landing.
//!
//! Run it from anywhere in the repository, naming the folder to write the five files to. Parquet
//! is an optional feature of the library, so the command turns it on:
//!
//! ```text
//! cargo run --example export_flight -p fusionspace-hpr-sim --features parquet -- my-flight
//! ```
//!
//! Without a folder it writes them to `fusionspace-hpr-export` in the system's temporary folder. The
//! documentation site's *Exporting a flight* page (`docs/exporting-a-flight.md`) walks through it.
//! What it prints is kept next to it in `export_flight.output.txt`, and CI checks that the two
//! still agree (`cargo xtask examples --check`).

#![allow(
    clippy::print_stdout,
    reason = "the project's lints forbid printing in library code, and this program exists to print"
)]
#![allow(
    clippy::disallowed_methods,
    reason = "the library builds each file's text without I/O; this program writes the files"
)]

use std::error::Error;
use std::path::PathBuf;

use hpr_atmos::ConstantWind;
use hpr_core::geodesy::Geodetic;
use hpr_design::Rocket;
use hpr_sim::{
    CanopyType, Channel, Device, DeviceDrag, Environment, FlightMetrics, FlightSettings, Rail,
    Recorder, Simulation, Termination, Trigger, export,
};

fn main() -> Result<(), Box<dyn Error>> {
    // The first flight: Valetudo on a K400C, from a 3 m vertical rail in New Mexico, in 5 m/s of
    // wind from the west, with a drogue at apogee and a main at 150 m.
    let rocket: Rocket = serde_json::from_str(include_str!(
        "../../../validation/designs/rocketpy-valetudo.json"
    ))?;
    let site = Geodetic::from_degrees(32.99, -106.97, 1400.0)?;
    let environment =
        Environment::standard(site)?.with_wind(ConstantWind::new(5.0, 270_f64.to_radians())?);
    let simulation = Simulation::new(
        &rocket,
        "example",
        environment,
        Rail::vertical(3.0),
        FlightSettings::default(),
    )?
    .with_recovery(vec![
        Device::new(
            "drogue",
            DeviceDrag::canopy(CanopyType::FlatCircular, 0.6),
            Trigger::Apogee,
        )
        .with_lag_s(0.5),
        Device::new(
            "main",
            DeviceDrag::canopy(CanopyType::FlatCircular, 2.4),
            Trigger::Altitude {
                height_above_ground_m: 150.0,
            },
        )
        .with_lag_s(1.0),
    ])?;

    // Two watchers on one flight: the metrics, for the landing, and a recorder of the time, the
    // height above the pad and the center of gravity's position, which a map needs.
    let recorder = Recorder::new(
        vec![
            Channel::Time,
            Channel::HeightAboveGround,
            Channel::CgPosition,
        ],
        Some(1.0),
    )?;
    let mut watchers = (FlightMetrics::new(), recorder);
    let flight = simulation.run(&mut watchers)?;
    if flight.termination != Termination::GroundHit {
        return Err(format!("the flight ended with {:?}", flight.termination).into());
    }
    let (metrics, recorder) = watchers;
    let summary = metrics.summary(&flight, simulation.environment())?;

    // The path on the Earth, then the five files. Each function returns the file's contents:
    // text, or bytes for Parquet, which is a binary format.
    let track = export::track(&recorder, simulation.environment())?;
    let files = [
        ("flight.csv", export::csv(&recorder)?.into_bytes()),
        ("flight.json", export::json(&recorder)?.into_bytes()),
        ("flight.parquet", export::parquet(&recorder)?),
        (
            "flight.geojson",
            export::geojson(&track, &summary)?.into_bytes(),
        ),
        (
            "flight.kml",
            export::kml(&track, &summary, "Valetudo, K400C")?.into_bytes(),
        ),
    ];
    let folder = match std::env::args_os().nth(1) {
        Some(folder) => PathBuf::from(folder),
        None => std::env::temp_dir().join("fusionspace-hpr-export"),
    };
    std::fs::create_dir_all(&folder)?;
    for (name, contents) in &files {
        std::fs::write(folder.join(name), contents)?;
    }

    // What was written, and where it landed, to five decimal places of a degree (about a meter).
    // The site is west of Greenwich, so the longitude is printed as degrees west.
    println!(
        "wrote {} rows of {} columns, a path of {} points",
        recorder.rows().len(),
        recorder.columns().len(),
        track.len()
    );
    for (name, contents) in &files {
        // The Parquet file's size is the same on every system: its numbers take 8 bytes each,
        // however many digits they have.
        if name.ends_with(".parquet") {
            println!("  {name}, {} bytes", contents.len());
        } else {
            println!("  {name}");
        }
    }
    if let Some(landing) = summary.landing {
        println!(
            "landed at {:.5}° N, {:.5}° W, {:.0} m from the pad, at {:.1} s",
            landing.latitude_deg, -landing.longitude_deg, landing.distance_m, landing.time_s
        );
    }
    Ok(())
}
```
