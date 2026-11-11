//! Predictions merged before their errors: the public real flights that join the seven of
//! [`crate::real_flight`], each flown from its record alone ([M2.3d3][m2-3d3], decisions
//! [ADR-328][adr-328] §1a and [ADR-334][adr-334]).
//!
//! A flight here has a design, a motor, a mass, a site, a rail and its day's weather, each from a
//! named record, and a *rule* for the apogee its log will be read at. It has no measured apogee:
//! nothing in this module parses a flight log, and neither [`PredictionRow`] nor
//! [`PredictionsReport`] has a field for a measured height or an error. The log's bytes are
//! hashed, so the report pins the file the rule will later be applied to, and never read as
//! numbers. The comparison is a later pull request's ([M2.3d2][m2-3d2]), which must print each
//! prediction here beside its error and may not change one without saying so.
//!
//! What is flown is what [`crate::real_flight`] flies for the seven: the design under
//! `validation/designs/` built from the example's inputs, on the example's own thrust file with
//! its burn time, from the example's rail and site, in the ERA5 air of the launch hour, on HPR
//! Sim's own aerodynamics. The three EuRoC 2022 flights' weather file is netCDF-4, which
//! [`hpr_io::netcdf`] refuses; `validation/oracles/netcdf/euroc_2022.py` converts it as the guide
//! says and checks the result, and [`CONVERSION_RECORD`] holds both files' SHA-256 and the levels
//! RocketPy read at each flight's site and hour. The converted file is flown only if its SHA-256
//! is the committed one and HPR Sim reads those levels from it. Each prediction is held to the
//! apogee first merged ([`MERGED`]).
//!
//! The predicted apogee is read as the log's altimeter would read it ([`Barometer`]: the standard
//! atmosphere's altitude of the flown air's pressure at the apogee, less the pad's), because that
//! is what it will be compared with; the height climbed is kept beside it.
//!
//! Two more flights have no example notebook ([the Macie XL predictions][m2-3d5], decision
//! [ADR-337][adr-337]): [`DESIGN_FILE_FLIGHTS`], both of one OpenRocket design saved in a pinned
//! public repository ([`ROCKETRY_FILES`]). Each is flown from that file alone, read as OpenRocket
//! 24.12 saved it: the motor configuration, rod, wind and air of the stored simulation the
//! repository names for the flight, on the bundled curve the file's digest names. With them all
//! five predictions are here ([`MERGED`]).
//!
//! [m2-3d3]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m2-3d3
//! [m2-3d2]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m2-3d2
//! [adr-328]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0328-the-2026-11-08-review-twelve-public-flights-predictions-first.md
//! [adr-334]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0334-m2-3d1-in-three-the-euroc-2022-predictions-first.md
//! [m2-3d5]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m2-3d5
//! [adr-337]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0337-the-macie-xl-predictions-each-flight-s-stored-simulation-flown-as-saved.md

use std::fmt::Write as _;
use std::path::Path;

use hpr_atmos::ConstantWind;
use hpr_core::geodesy::Geodetic;
use hpr_design::Rocket;
use hpr_io::era5::{Era5Profile, Era5Request, UtcTime};
use hpr_io::netcdf::NetCdf;
use hpr_io::ork;
use hpr_sim::{Environment, EventKind, FlightMetrics, FlightSettings, Rail, Simulation};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::real_flight::{
    Altimeter, Barometer, FileRead, Heights, MASS_FIXTURE, ROCKETPY, RealFlightError,
    example_thrust_of, fixture_case_of, fmt_number, launch_day_at, parse_thrust, rail_from,
    read_bytes, rocket_with_curve, sha256_hex,
};

/// The predictions, as data.
pub const PREDICTIONS_JSON: &str = "validation/reports/public-predictions.json";

/// The predictions, as a page.
pub const PREDICTIONS_MD: &str = "validation/reports/public-predictions.md";

/// The record of the EuRoC 2022 weather file's conversion: the source's and the converted file's
/// path and SHA-256, and the levels RocketPy read at each flight's site and hour, written by
/// `validation/oracles/netcdf/euroc_2022.py`.
pub const CONVERSION_RECORD: &str = "validation/fixtures/weather/euroc-2022-conversion.json";

/// How long a prediction flies at most, s. Every flight here reaches its apogee in under a
/// minute; with no parachute in the design it then falls to the ground and stops.
const MAX_TIME_S: f64 = 600.0;

/// What "traced" means in [`PredictedFlight::untraced`], as the page says it.
pub const TRACED_RULE: &str = "An input is traced when the flight's own record states it: here \
    RocketPy's example notebook, which holds the team's inputs. What no RocketPy example records \
    is a placeholder in the designs built from them, the seven compared flights' too, and doesn't \
    count against the tag: the surface finish, the rail buttons' size, the fins' thickness where \
    no airfoil file gives it, and their edge shape where no airfoil file or published section \
    does. They move the drag, so a traced flight's inputs are stated, not exact. A flight is \
    tagged not traced when its record leaves out, or contradicts, something else that moves the \
    apogee.";

/// A prediction as it was first merged, in the digits its page prints.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MergedPrediction {
    /// The flight's public id.
    pub id: &'static str,
    /// Its predicted apogee as the log's altimeter would read it, m.
    pub apogee_m: f64,
    /// Its predicted apogee as a height above the pad, m.
    pub height_apogee_m: f64,
    /// Its catalog curve's predicted apogee as the altimeter would read it, m, where it has one.
    pub catalog_apogee_m: Option<f64>,
}

/// The predictions as first merged ([the EuRoC 2022 predictions][m2-3d3], then [the Macie XL
/// predictions][m2-3d5]), which [`PredictionsReport::check_consistent`] holds every report to,
/// the committed one in CI and a flown one before it is written: each flight's apogee as a
/// reading and as a height, and Andromeda's on the catalog curve. The order is
/// [`NEW_FLIGHTS`]', then [`DESIGN_FILE_FLIGHTS`]'.
///
/// These lines are not edited. A prediction that has to change is revised beside its merged
/// number, with the reason, by the change that compares it ([twelve public flights, predictions
/// first][adr-328], §1b); until that exists, a changed prediction fails here and the writer
/// refuses it.
///
/// [m2-3d3]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m2-3d3
/// [m2-3d5]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m2-3d5
/// [adr-328]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0328-the-2026-11-08-review-twelve-public-flights-predictions-first.md
pub const MERGED: [MergedPrediction; 5] = [
    MergedPrediction {
        id: "astra",
        apogee_m: 3037.1,
        height_apogee_m: 3155.4,
        catalog_apogee_m: None,
    },
    MergedPrediction {
        id: "andromeda",
        apogee_m: 3581.0,
        height_apogee_m: 3729.5,
        catalog_apogee_m: Some(3050.3),
    },
    MergedPrediction {
        id: "erebus-11",
        apogee_m: 2714.1,
        height_apogee_m: 2808.7,
        catalog_apogee_m: None,
    },
    MergedPrediction {
        id: "macie-xl-1",
        apogee_m: 706.2,
        height_apogee_m: 706.3,
        catalog_apogee_m: None,
    },
    MergedPrediction {
        id: "macie-xl-2",
        apogee_m: 692.6,
        height_apogee_m: 692.7,
        catalog_apogee_m: None,
    },
];

/// How far a prediction may sit from its merged number and still print as it: half of the last
/// digit the page prints, m.
const MERGED_WITHIN_M: f64 = 0.05;

/// The apogee a flight's log will be read at: the rule, never the value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Reference {
    /// The log: under `refs/rocketpy/data/rockets/` for a flight in [`NEW_FLIGHTS`], under the
    /// pinned repository's checkout for one in [`DESIGN_FILE_FLIGHTS`]. Its bytes are hashed and
    /// never parsed here.
    pub log: &'static str,
    /// The rule, in words: which column, read how far.
    pub rule: &'static str,
    /// What the log's height is taken to be, and so how the prediction is read.
    pub altimeter: Altimeter,
}

/// A second prediction shown beside a flight's counted one and never counted: the design flown
/// as committed, on its own bundled catalog curve.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CatalogCurve {
    /// The motor, as the catalog names it.
    pub motor: &'static str,
    /// Why it is shown, and why it is not the counted one.
    pub why: &'static str,
}

/// One flight predicted before its log is compared.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PredictedFlight {
    /// The flight's public id, the one the real-flights report will list it under.
    pub id: &'static str,
    /// The rocket, the team and the event, in words.
    pub title: &'static str,
    /// The design flown, by file stem under `validation/designs/`, which, less its `rocketpy-`,
    /// is its case name in [`MASS_FIXTURE`].
    pub design: &'static str,
    /// Where the site, date, rail and geometry come from, in RocketPy 1.13.0's notebook.
    pub source: &'static str,
    /// The motor, as its record names it.
    pub motor: &'static str,
    /// Where the thrust file and its burn time come from.
    pub motor_source: &'static str,
    /// Where the mass comes from.
    pub mass_source: &'static str,
    /// The site: latitude and longitude, degrees, and elevation above sea level, m.
    pub site: (f64, f64, f64),
    /// The launch hour the weather is read at, UTC: year, month, day, hour.
    pub utc: (i32, u32, u32, u32),
    /// The rail: length, m; inclination from the horizontal and heading from north, degrees.
    pub rail: (f64, f64, f64),
    /// The apogee rule.
    pub reference: Reference,
    /// What moves the apogee and no record states, or a record contradicts; empty when every
    /// input is traced ([`TRACED_RULE`]).
    pub untraced: &'static [&'static str],
    /// The catalog curve's prediction, where the counted curve is not the catalog's.
    pub catalog_curve: Option<CatalogCurve>,
    /// What else a reader should know before the comparison; empty when nothing.
    pub note: &'static str,
}

/// The flights, in the order the page lists them: EuRoC 2022's three.
///
/// Each site, date and rail is the example notebook's (`docs/examples/<name>_flight_sim.ipynb` in
/// RocketPy 1.13.0). The notebooks give local time with `timezone="Portugal"`: Western European
/// Summer Time, UTC+1, until 30 October 2022, so 16 h is 15:00Z and 14 h is 13:00Z.
///
/// Every rule and tag here was written before any of the three was flown, and each log was looked
/// at only for its columns and for a transient near its peak (none: each peak is the whole log's
/// highest reading). That order is the maintainer's statement: the rules and the first flown
/// numbers reached the repository in one change, so no check can show it.
pub const NEW_FLIGHTS: [PredictedFlight; 3] = [
    PredictedFlight {
        id: "astra",
        title: "Astra, Faraday Rocketry UPV, EuRoC 2022 (L1350)",
        design: "rocketpy-astra",
        source: "astra_flight_sim.ipynb:123-131 (site, date), :247-280 (rocket, rail buttons, \
                 motor position, nose, fins), :316-318 (rail)",
        motor: "Cesaroni 4263L1350-P",
        motor_source: "astra_flight_sim.ipynb:155-157: RocketPy's copy of the certified curve, \
                       cut at the notebook's burn time",
        mass_source: "astra_flight_sim.ipynb:249-251 (mass, inertia and center without the motor), \
                      :158-162 (the grain, whose density and size give the propellant), :168-170 \
                      (no motor dry mass or inertia)",
        site: (39.3897, -8.288_963_9, 160.0),
        utc: (2022, 10, 13, 15),
        rail: (12.0, 84.0, 133.0),
        reference: Reference {
            log: "astra/flight_data.csv",
            rule: "the highest reading of the log's `alt` column, in meters, the log read whole",
            altimeter: Altimeter::AssumedBarometric(
                "the notebook calls it a COTS altimeter's flight (astra_flight_sim.ipynb:383-385), \
                 one row a second beside a latitude and a longitude, and records no more: \
                 barometric assumed, as hobby altimeters are",
            ),
        },
        untraced: &[],
        catalog_curve: None,
        note: "",
    },
    PredictedFlight {
        id: "andromeda",
        title: "Andromeda, Aristotle Space and Aeronautics Team, EuRoC 2022 (the team's curve)",
        design: "rocketpy-andromeda",
        source: "andromeda_flight_sim.ipynb:123-131 (site, date), :244-277 (rocket, rail buttons, \
                 motor position, nose, fins), :388-390 (rail)",
        motor: "the team's thrust file, flown in the notebook under the name of a Cesaroni M2020",
        motor_source: "andromeda_flight_sim.ipynb:153 and :162-165: \"obtained by the team, after \
                       testing\", cut at the notebook's burn time; the catalog file is commented \
                       out at :163",
        mass_source: "andromeda_flight_sim.ipynb:246-248 (mass, inertia and center without the \
                      motor), :166-170 (the grain), :176-178 (no motor dry mass or inertia)",
        site: (39.388_692, -8.287_814, 160.0),
        utc: (2022, 10, 14, 13),
        rail: (12.0, 84.0, 133.0),
        reference: Reference {
            log: "andromeda/flight_data.csv",
            rule: "the highest reading of the log's `alt(m)` column, the log read whole",
            altimeter: Altimeter::AssumedBarometric(
                "the notebook calls it a COTS altimeter (andromeda_flight_sim.ipynb:459-461), \
                 logged in feet and in meters with a vertical speed, and records no more: barometric assumed, as \
                 hobby altimeters are",
            ),
        },
        untraced: &[
            "the flight carried the team's air brakes, and no record says when or how far they \
             opened: none is flown here, so any opening leaves this prediction high",
            "the thrust file holds more impulse than the Cesaroni M2020 reload it is named for is \
             certified at, so the notebook does not settle which motor the logged flight burned",
        ],
        catalog_curve: Some(CatalogCurve {
            motor: "Cesaroni 8429M2020-P",
            why: "the reload the notebook names, on its certified curve as bundled, whole. Shown \
                  beside the counted prediction and never counted: every RocketPy flight here \
                  is counted on its example's own thrust file",
        }),
        note: "both tags are the research note's findings (docs/research/andromeda-impulse.md), \
               written down before this flight was flown",
    },
    PredictedFlight {
        id: "erebus-11",
        title: "Erebus 11, BME Suborbitals, EuRoC 2022 (a Cesaroni Pro54 K)",
        design: "rocketpy-erebus-11",
        source: "erebus_flight_sim.ipynb:123-131 (site, date), :251-284 (rocket, rail buttons, \
                 motor position, nose, fins, tail), :320-322 (rail)",
        motor: "a Cesaroni Pro54 K-class motor, as the notebook says; the example's thrust file",
        motor_source: "erebus_flight_sim.ipynb:153 and :162-165, cut at the notebook's burn time",
        mass_source: "erebus_flight_sim.ipynb:253-255 (mass, inertia and center without the \
                      motor), :166-172 (the grain), :163 and :173-174 (no motor dry mass or \
                      inertia)",
        site: (39.3897, -8.288_963_9, 160.0),
        utc: (2022, 10, 15, 15),
        rail: (12.0, 84.0, 133.0),
        reference: Reference {
            log: "erebus11/flight_data_filtered.csv",
            rule: "the highest reading of the log's `alt` column, in meters, the log read whole",
            altimeter: Altimeter::AssumedBarometric(
                "the notebook calls it a COTS altimeter (erebus_flight_sim.ipynb:390-392), a file \
                 named filtered, and records no more: barometric assumed, as hobby altimeters are",
            ),
        },
        untraced: &[],
        catalog_curve: None,
        note: "",
    },
];

/// A public repository pinned in `validation/refs.lock.toml`, which a design-file flight's
/// design and log are read from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PinnedRepository {
    /// Its name in the lock, which `cargo xtask refs fetch <name>` takes.
    pub name: &'static str,
    /// Where it is cloned from.
    pub url: &'static str,
    /// The commit it is pinned at.
    pub commit: &'static str,
    /// Its checkout, from the repository root.
    pub dest: &'static str,
    /// Its license, as GitHub's license API reads its `LICENSE`.
    pub license: &'static str,
}

/// The repository both Macie XL flights are read from: its design file and the two logs, as
/// they stand at its pinned commit of 27 June 2026. Nothing of it is committed here but SHA-256
/// hashes and the numbers flown from it.
pub const ROCKETRY_FILES: PinnedRepository = PinnedRepository {
    name: "rocketry-files",
    url: "https://github.com/rlhatcher/rocketry-files.git",
    commit: "2ddcaf8858a71f9b4d910877c603ed95fe0a9769",
    dest: "refs/public-flights/rocketry-files",
    license: "MIT",
};

/// What "traced" means for a flight in [`DESIGN_FILE_FLIGHTS`], as the page says it.
pub const DESIGN_FILE_TRACED_RULE: &str = "For a flight read from a design file, the record is \
    that file and its repository's page about the flight. The file states the rocket and names \
    the motor. A stored simulation's launch conditions are what a simulation was run with, not a \
    record of the day, so the site, the air, the wind and the rod count as not traced unless \
    the page states them. So does the mass, unless the page says the rocket was weighed as \
    flown, and the motor's curve where the reader warns that its file and its certifier's \
    record disagree.";

/// One flight predicted from a design file in a pinned public repository, before its log is
/// compared.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DesignFileFlight {
    /// The flight's public id, the one the real-flights report will list it under.
    pub id: &'static str,
    /// The rocket and the flight, in words.
    pub title: &'static str,
    /// The repository.
    pub repository: PinnedRepository,
    /// The design file, in the repository.
    pub design: &'static str,
    /// The stored simulation flown, by its name in the design file: its motor configuration and
    /// its launch conditions.
    pub simulation: &'static str,
    /// Where the pairing of flight and simulation, and the design, come from.
    pub source: &'static str,
    /// The motor, as the stored simulation's configuration names it.
    pub motor: &'static str,
    /// Where the motor and its curve come from.
    pub motor_source: &'static str,
    /// Where the mass comes from.
    pub mass_source: &'static str,
    /// Where the site, rod, wind and air come from.
    pub conditions_source: &'static str,
    /// The apogee rule; its log is in the repository.
    pub reference: Reference,
    /// What moves the apogee and no record states, or a record contradicts
    /// ([`DESIGN_FILE_TRACED_RULE`]).
    pub untraced: &'static [&'static str],
    /// What else a reader should know before the comparison; empty when nothing.
    pub note: &'static str,
}

/// What both Macie XL flights leave untraced.
const MACIE_XL_SITE_UNTRACED: &str = "no record gives the day, its air and wind, or the rail, \
    and of the site only each log's header gives the pad's height above sea level (12 ft for \
    flight 1, 37 ft for flight 2): the stored simulation's are flown, its sea level among them, \
    and all nine simulations in the file carry the same ones";
/// The same, for the mass.
const MACIE_XL_MASS_UNTRACED: &str = "no record says the rocket was weighed as flown: the \
    file's parts and overrides are flown";
/// The same, for the motor's curve.
const MACIE_XL_CURVE_UNTRACED: &str = "the reader warns that the motor's curve file disagrees \
    with ThrustCurve.org's record of the motor (the figures are in the flight's \
    `reader_warnings` in the JSON); the file the design's digest names is flown";
/// Flight 1's own: the masses its motor flies are not the ones OpenRocket flew.
const MACIE_XL_1_MOTOR_MASS_UNTRACED: &str = "the motor flies the bundled catalog's masses for \
    it, 313.6 g of propellant and 555.52 g loaded, as every catalog motor does, where the curve \
    file the design's digest names, and OpenRocket's own record of that digest, give 310.016 g \
    and 555.072 g (the table's `mass_difference`)";
/// What the page says of how both are flown.
const MACIE_XL_NOTE: &str = "the stored simulation's wind carries a turbulence of a tenth of \
    its speed, which is not flown: the wind here is steady. The parachutes are not flown \
    either; the file opens the first at apogee, so neither moves it";

/// The flights read from a design file, in the order the page lists them: Macie XL's two.
///
/// Both fly `macie/macie-xl.ork`, an OpenRocket 24.12 design. The repository's page about the
/// rocket (`macie/index.md`) lists the two flights and names each flight's files
/// `maciexl-<flight>-<source>`, `or` for the OpenRocket simulation; the design file holds a
/// stored simulation under each of those names, and that one is flown.
///
/// The apogee rule is the one [twelve public flights, predictions first][adr-328] fixed for a
/// log with an ejection spike, merged before HPR Sim could fly this file at all. The tags, and
/// the choice to fly each stored simulation's own conditions, were written after `hpr sim` had
/// flown the file at that command's defaults, and with both measured apogees already in the
/// research notes. Neither choice was made on a number: the file is flown as it is saved, and
/// nothing in it was changed.
///
/// [adr-328]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0328-the-2026-11-08-review-twelve-public-flights-predictions-first.md
pub const DESIGN_FILE_FLIGHTS: [DesignFileFlight; 2] = [
    DesignFileFlight {
        id: "macie-xl-1",
        title: "Macie XL, flight 1 (AeroTech I284W)",
        repository: ROCKETRY_FILES,
        design: "macie/macie-xl.ork",
        simulation: "maciexl-1-or",
        source: "macie/index.md (the two flights; the file names that pair `maciexl-1-or` with \
                 flight 1) and macie/macie-xl.ork, read whole as OpenRocket 24.12 saved it",
        motor: "AeroTech I284W",
        motor_source: "the stored simulation's motor configuration in macie/macie-xl.ork, which \
                       names the motor and the digest of its curve; the bundled ThrustCurve.org \
                       file that digest names is flown",
        mass_source: "macie/macie-xl.ork: its parts and the stage's mass and center overrides, \
                      which add to the parts as OpenRocket 24.12 weighs them; the motor's masses \
                      are the bundled catalog's for the motor, 313.6 g of propellant and 555.52 g \
                      loaded",
        conditions_source: "the stored simulation's launch conditions in macie/macie-xl.ork",
        reference: Reference {
            log: "macie/flights/MRC Summer 26/maciexl-1-sl.pf2",
            rule: "the apogee the altimeter worked out, in the log's header (its `Apogee:` \
                   line, in feet above the pad, 0.3048 m to the foot), which leaves out the \
                   ejection spike among the samples; never the highest sample",
            altimeter: Altimeter::Barometric(
                "the log's first line names a PerfectFlite altimeter, `PerfectFlite SLCF`, and \
                 a PerfectFlite has a barometer and no accelerometer (docs/format/pf2.md)",
            ),
        },
        untraced: &[
            MACIE_XL_SITE_UNTRACED,
            MACIE_XL_MASS_UNTRACED,
            MACIE_XL_CURVE_UNTRACED,
            MACIE_XL_1_MOTOR_MASS_UNTRACED,
        ],
        note: MACIE_XL_NOTE,
    },
    DesignFileFlight {
        id: "macie-xl-2",
        title: "Macie XL, flight 2 (Cesaroni 601I350-16A)",
        repository: ROCKETRY_FILES,
        design: "macie/macie-xl.ork",
        simulation: "maciexl-2-or",
        source: "macie/index.md (the two flights; the file names that pair `maciexl-2-or` with \
                 flight 2) and macie/macie-xl.ork, read whole as OpenRocket 24.12 saved it",
        motor: "Cesaroni 601I350-16A",
        motor_source: "the stored simulation's motor configuration in macie/macie-xl.ork, which \
                       names the motor and the digest of its curve; the bundled ThrustCurve.org \
                       file that digest names is flown",
        mass_source: "macie/macie-xl.ork: its parts and the stage's mass and center overrides, \
                      which add to the parts as OpenRocket 24.12 weighs them; the motor's masses \
                      are the bundled catalog's for the motor, which are OpenRocket's for the \
                      design's digest",
        conditions_source: "the stored simulation's launch conditions in macie/macie-xl.ork",
        reference: Reference {
            log: "macie/flights/MRC Summer 26/maciexl-2-sl.pf2",
            rule: "the apogee the altimeter worked out, in the log's header (its `Apogee:` \
                   line, in feet above the pad, 0.3048 m to the foot); never the highest sample",
            altimeter: Altimeter::Barometric(
                "the log's first line names a PerfectFlite altimeter, `PerfectFlite SLCF`, and \
                 a PerfectFlite has a barometer and no accelerometer (docs/format/pf2.md)",
            ),
        },
        untraced: &[
            MACIE_XL_SITE_UNTRACED,
            MACIE_XL_MASS_UNTRACED,
            MACIE_XL_CURVE_UNTRACED,
        ],
        note: MACIE_XL_NOTE,
    },
];

/// The catalog curve's prediction, as the report keeps it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogCurveRow {
    /// The motor, as the catalog names it.
    pub motor: String,
    /// Why it is shown and not counted.
    pub why: String,
    /// The impulse flown, N s.
    pub total_impulse_ns: f64,
    /// The mass at liftoff, kg: the counted flight's, since the design's propellant is its
    /// grains' and only the curve differs.
    pub liftoff_mass_kg: f64,
    /// Its predicted apogee as the log's altimeter would read it, m. Never counted.
    pub predicted_apogee_m: f64,
    /// Its predicted apogee as a height above the pad, m.
    pub predicted_height_apogee_m: f64,
}

/// One flight's prediction, as the report keeps it. It has no field for a measured apogee or an
/// error, on purpose.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PredictionRow {
    /// The flight's public id.
    pub id: String,
    /// The rocket, team and event.
    pub title: String,
    /// The design flown.
    pub design: String,
    /// Where the site, date, rail and geometry come from.
    pub source: String,
    /// The motor, as its record names it.
    pub motor: String,
    /// Where the thrust file and its burn time come from.
    pub motor_source: String,
    /// The thrust file flown, from the repository root.
    pub thrust_file: String,
    /// The burn time the example cuts it at, s.
    pub burn_time_s: Option<f64>,
    /// The impulse flown, N s.
    pub total_impulse_ns: f64,
    /// Where the mass comes from.
    pub mass_source: String,
    /// The mass the record gives without the motor, kg.
    pub mass_without_motor_kg: f64,
    /// The mass flown at liftoff, kg: the above and the grain.
    pub liftoff_mass_kg: f64,
    /// The site's latitude, degrees north.
    pub latitude_deg: f64,
    /// The site's longitude, degrees east.
    pub longitude_deg: f64,
    /// The site's elevation above sea level, m.
    pub elevation_m: f64,
    /// The rail's length, m.
    pub rail_length_m: f64,
    /// The rail's inclination from the horizontal, degrees.
    pub rail_inclination_deg: f64,
    /// The rail's heading from north, degrees.
    pub rail_heading_deg: f64,
    /// The hour the weather is read at.
    pub weather_time_utc: String,
    /// The log the rule will be applied to, from the repository root.
    pub reference_log: String,
    /// That log's SHA-256: its bytes hashed, never parsed.
    pub reference_log_sha256: String,
    /// The apogee rule.
    pub reference_rule: String,
    /// What the log's height is taken to be.
    pub altimeter: String,
    /// Where that comes from.
    pub altimeter_evidence: String,
    /// Whether every input is traced to the record.
    pub all_inputs_traced: bool,
    /// What isn't, in words.
    pub untraced: Vec<String>,
    /// The predicted apogee as the log's altimeter would read it, m: the counted prediction.
    pub predicted_apogee_m: f64,
    /// The predicted apogee as a height above the pad, m.
    pub predicted_height_apogee_m: f64,
    /// The predicted time from ignition to apogee, s.
    pub predicted_time_to_apogee_s: f64,
    /// The flight's largest Mach number.
    pub hpr_max_mach: f64,
    /// The catalog curve's prediction, shown beside and never counted.
    pub catalog_curve: Option<CatalogCurveRow>,
    /// Every file read, with its SHA-256; the log's is its bytes' alone.
    pub files: Vec<FileRead>,
    /// The SHA-256 of the flight's entry in [`NEW_FLIGHTS`].
    pub inputs_sha256: String,
    /// What else a reader should know.
    pub note: String,
}

/// One design-file flight's prediction, as the report keeps it. Like [`PredictionRow`], it has
/// no field for a measured apogee or an error, on purpose.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DesignFilePredictionRow {
    /// The flight's public id.
    pub id: String,
    /// The rocket and the flight.
    pub title: String,
    /// The repository the design and the log are read from.
    pub repository: String,
    /// The commit it is pinned at.
    pub repository_commit: String,
    /// Its license.
    pub repository_license: String,
    /// The design file flown, from the repository root.
    pub design_file: String,
    /// Its SHA-256.
    pub design_file_sha256: String,
    /// The stored simulation flown, by name.
    pub simulation: String,
    /// The motor configuration that simulation flies, by the file's id for it.
    pub configuration: String,
    /// Where the pairing and the design come from.
    pub source: String,
    /// The motor.
    pub motor: String,
    /// Where the motor and its curve come from.
    pub motor_source: String,
    /// The digest the design file records for the motor's curve.
    pub motor_digest: String,
    /// ThrustCurve.org's id of the bundled curve file flown, the one that digest names.
    pub curve_file: String,
    /// The impulse flown, N s.
    pub total_impulse_ns: f64,
    /// Where the mass comes from.
    pub mass_source: String,
    /// The mass flown without the motor, kg.
    pub mass_without_motor_kg: f64,
    /// The mass flown at liftoff, kg.
    pub liftoff_mass_kg: f64,
    /// Where the site, rod, wind and air come from.
    pub conditions_source: String,
    /// The site's latitude, degrees north.
    pub latitude_deg: f64,
    /// The site's longitude, degrees east.
    pub longitude_deg: f64,
    /// The site's elevation above sea level, m.
    pub elevation_m: f64,
    /// The rod's length, m.
    pub rail_length_m: f64,
    /// The rod's inclination from the horizontal, degrees.
    pub rail_inclination_deg: f64,
    /// The rod's heading from north, degrees.
    pub rail_heading_deg: f64,
    /// The steady wind's speed, m/s.
    pub wind_speed_m_s: f64,
    /// The bearing it blows from, degrees from north.
    pub wind_from_deg: f64,
    /// The air.
    pub atmosphere: String,
    /// The log the rule will be applied to, from the repository root.
    pub reference_log: String,
    /// That log's SHA-256: its bytes hashed, never parsed.
    pub reference_log_sha256: String,
    /// The apogee rule.
    pub reference_rule: String,
    /// What the log's height is taken to be.
    pub altimeter: String,
    /// Where that comes from.
    pub altimeter_evidence: String,
    /// Whether every input is traced to the record.
    pub all_inputs_traced: bool,
    /// What isn't, in words.
    pub untraced: Vec<String>,
    /// What the reader warned of in the design file and the flown motor, each with its place.
    pub reader_warnings: Vec<String>,
    /// The predicted apogee as the log's altimeter would read it, m: the counted prediction.
    pub predicted_apogee_m: f64,
    /// The predicted apogee as a height above the pad, m.
    pub predicted_height_apogee_m: f64,
    /// The predicted time from ignition to apogee, s.
    pub predicted_time_to_apogee_s: f64,
    /// The flight's largest Mach number.
    pub hpr_max_mach: f64,
    /// The apogee OpenRocket's own run of this simulation reached, as the design file stores
    /// it, m: another simulator's number, not a measurement. Shown, and never counted.
    pub openrocket_stored_apogee_m: f64,
    /// Every file read, with its SHA-256; the log's is its bytes' alone.
    pub files: Vec<FileRead>,
    /// The SHA-256 of the flight's entry in [`DESIGN_FILE_FLIGHTS`].
    pub inputs_sha256: String,
    /// What else a reader should know.
    pub note: String,
}

/// The weather file's conversion, as [`CONVERSION_RECORD`] holds it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WeatherConversion {
    /// The record's path.
    pub record: String,
    /// The conversion, as code.
    pub conversion: String,
    /// The netCDF-4 file, from the repository root.
    pub source: String,
    /// Its SHA-256.
    pub source_sha256: String,
    /// The converted file flown, from the repository root.
    pub converted: String,
    /// Its SHA-256.
    pub converted_sha256: String,
}

/// The predictions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PredictionsReport {
    /// What wrote it.
    pub generator: String,
    /// What "traced" means.
    pub traced_rule: String,
    /// The EuRoC 2022 weather file's conversion.
    pub weather_conversion: WeatherConversion,
    /// The flights, in [`NEW_FLIGHTS`]' order.
    pub flights: Vec<PredictionRow>,
    /// What "traced" means for a flight read from a design file.
    pub design_file_traced_rule: String,
    /// The flights read from a design file, in [`DESIGN_FILE_FLIGHTS`]' order.
    pub design_file_flights: Vec<DesignFilePredictionRow>,
}

/// What one flight of a rocket gave.
struct Flown {
    apogee_m: f64,
    height_apogee_m: f64,
    time_to_apogee_s: f64,
    max_mach: f64,
    total_impulse_ns: f64,
    liftoff_mass_kg: f64,
    /// The motors' mass at liftoff, kg: each one's case and propellant.
    motor_mass_kg: f64,
}

/// Reads [`CONVERSION_RECORD`].
fn conversion(root: &Path) -> Result<WeatherConversion, RealFlightError> {
    let input = |what: String| RealFlightError::Input {
        flight: "the EuRoC 2022 weather".to_owned(),
        what,
    };
    let bytes = read_bytes(root, CONVERSION_RECORD, "reading the conversion's record")?;
    let record: Value = serde_json::from_slice(&bytes)
        .map_err(|error| input(format!("{CONVERSION_RECORD}: {error}")))?;
    let text = |pointer: &str| -> Result<String, RealFlightError> {
        record
            .pointer(pointer)
            .and_then(Value::as_str)
            .map(str::to_owned)
            .ok_or_else(|| input(format!("{CONVERSION_RECORD} has no {pointer}")))
    };
    Ok(WeatherConversion {
        record: CONVERSION_RECORD.to_owned(),
        conversion: text("/conversion")?,
        source: text("/source/path")?,
        source_sha256: text("/source/sha256")?,
        converted: text("/converted/path")?,
        converted_sha256: text("/converted/sha256")?,
    })
}

/// How far HPR Sim's reading of a level may sit from RocketPy's, relative (to 1 for a value under
/// 1): the two do the same arithmetic on the same stored numbers, and this leaves room for its
/// order, as `hpr_io`'s own comparison on the committed extracts does.
const LEVELS_WITHIN: f64 = 1e-12;

/// Holds HPR Sim's reading of the converted weather file, at a flight's site and hour, to the
/// levels RocketPy read from it, which [`CONVERSION_RECORD`] keeps: the same pressures, and the
/// temperature, both wind components and the geopotential height within [`LEVELS_WITHIN`].
///
/// RocketPy records a geometric height `h = R Z/(R − Z)` for its Earth radius `R`; it is turned
/// back into the geopotential height `Z = R h/(R + h)` to compare with the file's own.
///
/// # Errors
///
/// [`RealFlightError::Input`] naming the first level that differs, or what the record lacks.
fn levels_are_rocketpys(
    root: &Path,
    flight: &PredictedFlight,
    weather: &WeatherConversion,
    weather_bytes: &[u8],
) -> Result<usize, RealFlightError> {
    let input = |what: String| RealFlightError::Input {
        flight: flight.id.to_owned(),
        what,
    };
    let bytes = read_bytes(root, CONVERSION_RECORD, "reading the conversion's record")?;
    let record: Value = serde_json::from_slice(&bytes)
        .map_err(|error| input(format!("{CONVERSION_RECORD}: {error}")))?;
    let lacks = || input(format!("{CONVERSION_RECORD} has no levels for this flight"));
    let reading = record["rocketpy_readings"]
        .as_array()
        .and_then(|readings| readings.iter().find(|reading| reading["id"] == flight.id))
        .ok_or_else(lacks)?;
    let radius = reading["earth_radius_m"].as_f64().ok_or_else(lacks)?;
    let theirs = reading["levels"].as_array().ok_or_else(lacks)?;
    let file = NetCdf::parse(weather_bytes)
        .map_err(|error| input(format!("{}: {error}", weather.converted)))?;
    let (year, month, day, hour) = flight.utc;
    let (latitude_deg, longitude_deg, _) = flight.site;
    let time = UtcTime::from_civil(year, month, day, hour, 0, 0.0)
        .map_err(|error| input(error.to_string()))?;
    let profile = Era5Profile::read(
        &file,
        Era5Request {
            latitude_deg,
            longitude_deg,
            time,
        },
    )
    .map_err(|error| input(format!("{}: {error}", weather.converted)))?;
    if profile.times.len() != 1 || profile.levels.len() != theirs.len() || theirs.is_empty() {
        return Err(input(format!(
            "{}: HPR Sim reads {} levels at {} times, and RocketPy read {} at one",
            weather.converted,
            profile.levels.len(),
            profile.times.len(),
            theirs.len()
        )));
    }
    // RocketPy sorts by height and HPR Sim by pressure; both are lowest first.
    for (ours, theirs) in profile.levels.iter().zip(theirs) {
        let number = |key: &str| theirs[key].as_f64().ok_or_else(lacks);
        let pressure_pa = number("pressure_pa")?;
        let height_m = number("height_m")?;
        let pairs = [
            ("pressure", ours.pressure_pa, pressure_pa, 0.0),
            (
                "temperature",
                ours.temperature_k,
                number("temperature_k")?,
                LEVELS_WITHIN,
            ),
            (
                "east wind",
                ours.wind_east_m_s,
                number("wind_east_m_s")?,
                LEVELS_WITHIN,
            ),
            (
                "north wind",
                ours.wind_north_m_s,
                number("wind_north_m_s")?,
                LEVELS_WITHIN,
            ),
            (
                "geopotential height",
                ours.geopotential_height_m,
                radius * height_m / (radius + height_m),
                LEVELS_WITHIN,
            ),
        ];
        for (what, ours, theirs, within) in pairs {
            // A NaN on either side is a difference too.
            let gap = (ours - theirs).abs();
            if gap.is_nan() || gap > within * theirs.abs().max(1.0) {
                return Err(input(format!(
                    "{}: at {pressure_pa} Pa HPR Sim reads a {what} of {ours}, and RocketPy \
                     read {theirs}",
                    weather.converted
                )));
            }
        }
    }
    Ok(theirs.len())
}

/// Predicts one flight: flies it from its record and reads its apogee as its log's altimeter
/// would. The log is hashed, never parsed.
///
/// # Errors
///
/// [`RealFlightError`] if a file is missing or doesn't read, the thrust file holds negative
/// thrust, the converted weather file is not the committed conversion or doesn't read as the
/// levels RocketPy read, or HPR Sim can't fly it.
pub fn predict(
    root: &Path,
    flight: &PredictedFlight,
    weather: &WeatherConversion,
) -> Result<PredictionRow, RealFlightError> {
    let input = |what: String| RealFlightError::Input {
        flight: flight.id.to_owned(),
        what,
    };
    let mut files = Vec::new();
    let mut read = |relative: String, what: &'static str| -> Result<Vec<u8>, RealFlightError> {
        let bytes = read_bytes(root, &relative, what)?;
        files.push(FileRead {
            sha256: sha256_hex(&bytes),
            path: relative,
        });
        Ok(bytes)
    };

    // The design, with the example's own thrust and burn time from the fixture.
    read(MASS_FIXTURE.to_owned(), "reading the design fixture")?;
    let case = fixture_case_of(root, flight.id, flight.design)?;
    let mass_without_motor_kg = case["rocket"]["mass"]
        .as_f64()
        .ok_or_else(|| input(format!("{MASS_FIXTURE} gives {} no mass", flight.design)))?;
    let (thrust_path, burn, reshape) = example_thrust_of(root, flight.id, flight.design)?;
    if reshape.is_some() {
        return Err(input(
            "the example reshapes its thrust file, which no prediction here expects".to_owned(),
        ));
    }
    let thrust_bytes = read(thrust_path.clone(), "reading a thrust file")?;
    let (curve, zeroed_ns) = parse_thrust(
        flight.id,
        &thrust_path,
        &String::from_utf8_lossy(&thrust_bytes),
        burn,
        None,
    )?;
    // The seven's report records the impulse a zeroed negative thrust removes; this one has no
    // place for it, so a file that holds one is refused, not flown changed.
    if zeroed_ns != 0.0 {
        return Err(input(format!(
            "{thrust_path} holds negative thrust ({zeroed_ns} N s), which no prediction here \
             expects"
        )));
    }
    let design_path = format!("validation/designs/{}.json", flight.design);
    let design_bytes = read(design_path.clone(), "reading a design")?;
    let design: Value = serde_json::from_slice(&design_bytes)
        .map_err(|error| input(format!("{design_path}: {error}")))?;
    let rocket = rocket_with_curve(flight.id, &design, &design_path, &thrust_path, &curve)?;

    // The weather: the converted file, only if it is the committed conversion.
    let weather_bytes = read(
        weather.converted.clone(),
        "reading the converted ERA5 file, which validation/oracles/netcdf/euroc_2022.py writes",
    )?;
    if sha256_hex(&weather_bytes) != weather.converted_sha256 {
        return Err(input(format!(
            "{} is not the conversion {CONVERSION_RECORD} records: run \
             validation/oracles/netcdf/euroc_2022.py",
            weather.converted
        )));
    }
    // And only if HPR Sim reads from it, here and now, the levels RocketPy read.
    levels_are_rocketpys(root, flight, weather, &weather_bytes)?;
    let (environment, air) = launch_day_at(
        flight.id,
        flight.site,
        flight.utc,
        &weather.converted,
        &weather_bytes,
    )?;
    let (latitude_deg, longitude_deg, elevation_m) = flight.site;
    let rail = rail_from(flight.rail);

    // The log: its bytes' SHA-256, and nothing else of it.
    let log_path = format!("{ROCKETPY}/data/rockets/{}", flight.reference.log);
    let log_sha256 = sha256_hex(&read(log_path.clone(), "hashing a flight log")?);

    let launch = Launch {
        id: flight.id,
        configuration: "example",
        environment: &environment,
        air: &air,
        air_name: &weather.converted,
        elevation_m,
        rail,
        altimeter: flight.reference.altimeter,
    };
    let fly_once = |rocket: &Rocket| flown_to_apogee(rocket, &launch);
    let counted = fly_once(&rocket)?;
    // The design as committed, on its own bundled catalog curve: shown, never counted.
    let catalog_curve = match flight.catalog_curve {
        Some(catalog) => {
            let as_committed: Rocket = serde_json::from_value(design.clone())
                .map_err(|error| input(format!("{design_path}: {error}")))?;
            let flown = fly_once(&as_committed)?;
            Some(CatalogCurveRow {
                motor: catalog.motor.to_owned(),
                why: catalog.why.to_owned(),
                total_impulse_ns: flown.total_impulse_ns,
                liftoff_mass_kg: flown.liftoff_mass_kg,
                predicted_apogee_m: flown.apogee_m,
                predicted_height_apogee_m: flown.height_apogee_m,
            })
        }
        None => None,
    };
    let (year, month, day, hour) = flight.utc;
    let (rail_length_m, rail_inclination_deg, rail_heading_deg) = flight.rail;

    Ok(PredictionRow {
        id: flight.id.to_owned(),
        title: flight.title.to_owned(),
        design: flight.design.to_owned(),
        source: flight.source.to_owned(),
        motor: flight.motor.to_owned(),
        motor_source: flight.motor_source.to_owned(),
        thrust_file: thrust_path,
        burn_time_s: burn,
        total_impulse_ns: counted.total_impulse_ns,
        mass_source: flight.mass_source.to_owned(),
        mass_without_motor_kg,
        liftoff_mass_kg: counted.liftoff_mass_kg,
        latitude_deg,
        longitude_deg,
        elevation_m,
        rail_length_m,
        rail_inclination_deg,
        rail_heading_deg,
        weather_time_utc: format!("{year:04}-{month:02}-{day:02}T{hour:02}:00Z"),
        reference_log: log_path,
        reference_log_sha256: log_sha256,
        reference_rule: flight.reference.rule.to_owned(),
        altimeter: flight.reference.altimeter.kind().to_owned(),
        altimeter_evidence: flight.reference.altimeter.evidence().to_owned(),
        all_inputs_traced: flight.untraced.is_empty(),
        untraced: flight
            .untraced
            .iter()
            .map(|&what| what.to_owned())
            .collect(),
        predicted_apogee_m: counted.apogee_m,
        predicted_height_apogee_m: counted.height_apogee_m,
        predicted_time_to_apogee_s: counted.time_to_apogee_s,
        hpr_max_mach: counted.max_mach,
        catalog_curve,
        files,
        inputs_sha256: sha256_hex(format!("{flight:?}").as_bytes()),
        note: flight.note.to_owned(),
    })
}

/// What flying a rocket to its apogee needs beside the rocket.
struct Launch<'a> {
    /// The flight's id, for an error.
    id: &'a str,
    /// The motor configuration, by id.
    configuration: &'a str,
    /// The site, its air and its wind.
    environment: &'a Environment,
    /// The air a barometer reads, the environment's own.
    air: &'a dyn hpr_atmos::Atmosphere,
    /// What the air is called in an error.
    air_name: &'a str,
    /// The site's elevation above sea level, m.
    elevation_m: f64,
    /// The rail.
    rail: Rail,
    /// What the log's height is taken to be.
    altimeter: Altimeter,
}

/// Flies `rocket` to its apogee and reads it as the log's altimeter would.
fn flown_to_apogee(rocket: &Rocket, launch: &Launch<'_>) -> Result<Flown, RealFlightError> {
    let input = |what: String| RealFlightError::Input {
        flight: launch.id.to_owned(),
        what,
    };
    let sim = |source: hpr_sim::SimError| RealFlightError::Sim {
        flight: launch.id.to_owned(),
        source,
    };
    let settings = FlightSettings {
        max_time_s: MAX_TIME_S,
        ..FlightSettings::default()
    };
    let simulation = Simulation::new(
        rocket,
        launch.configuration,
        launch.environment.clone(),
        launch.rail,
        settings,
    )
    .map_err(sim)?;
    let motors = &simulation.assembly().motors;
    let total_impulse_ns = motors
        .iter()
        .map(|motor| motor.mounted.motor.curve().total_impulse_ns())
        .sum();
    let motor_mass_kg = motors
        .iter()
        .map(|motor| {
            let motor = &motor.mounted.motor;
            motor.dry().mass_kg + motor.propellant_initial_mass_kg()
        })
        .sum();
    let liftoff_mass_kg = simulation.assembly().mass_properties(0.0).mass_kg;
    let mut heights = Heights {
        next: 0,
        rows: Vec::new(),
    };
    let mut metrics = FlightMetrics::new();
    let result = simulation
        .run(&mut (&mut heights, &mut metrics))
        .map_err(sim)?;
    let max_mach = metrics
        .summary(&result, launch.environment)
        .map_err(sim)?
        .max_mach
        .ok_or_else(|| input("the flight has no Mach number".to_owned()))?
        .value;
    let apogee = result
        .event(EventKind::Apogee)
        .ok_or_else(|| input("the flight has no apogee".to_owned()))?
        .sample;
    let start_m = heights
        .rows
        .first()
        .map(|row| row.1)
        .ok_or_else(|| input("the flight has no steps".to_owned()))?;
    // What the log's altimeter would read at the apogee, above its start.
    let atmos = |error: hpr_atmos::AtmosError| input(format!("{}: {error}", launch.air_name));
    let apogee_m = match launch.altimeter {
        Altimeter::Height(_) => apogee.height_above_ground_m - start_m,
        Altimeter::Barometric(_) | Altimeter::AssumedBarometric(_) => {
            Barometer::on_pad(launch.air, launch.elevation_m, start_m)
                .map_err(atmos)?
                .reading_m(apogee.height_above_ground_m)
                .map_err(atmos)?
        }
    };
    Ok(Flown {
        apogee_m,
        height_apogee_m: apogee.height_above_ground_m - start_m,
        time_to_apogee_s: apogee.time_s,
        max_mach,
        total_impulse_ns,
        liftoff_mass_kg,
        motor_mass_kg,
    })
}

/// The air a design-file flight is flown in, as the report names it.
const STANDARD_AIR: &str = "the 1976 US Standard Atmosphere, for the stored simulation's `isa`";

/// What a design file's stored simulation states for a flight: its motor configuration, its
/// launch conditions and its one motor's curve.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct StoredLaunch<'a> {
    /// The motor configuration's id.
    pub(crate) configuration_id: &'a str,
    /// The launch rod's length, m.
    pub(crate) rod_length_m: f64,
    /// The rod's angle from the vertical, rad.
    pub(crate) rod_angle_rad: f64,
    /// The compass direction the rod leans toward, rad.
    pub(crate) rod_direction_rad: f64,
    /// The steady wind's speed, m/s.
    pub(crate) wind_speed_m_s: f64,
    /// The compass direction the wind blows from, rad.
    pub(crate) wind_from_rad: f64,
    /// The launch site's height above sea level, m.
    pub(crate) elevation_m: f64,
    /// The launch site's latitude, degrees north.
    pub(crate) latitude_deg: f64,
    /// The launch site's longitude, degrees east.
    pub(crate) longitude_deg: f64,
    /// The apogee OpenRocket stored for its own run, m above the site.
    pub(crate) openrocket_stored_apogee_m: f64,
    /// The configuration's one motor.
    pub(crate) motor: &'a ork::OrkMotor,
    /// The digest the file gives the motor.
    pub(crate) motor_digest: &'a str,
    /// The bundled curve file that digest names.
    pub(crate) simfile_id: &'a str,
}

/// Read the stored simulation named `simulation` out of `design`, refusing whatever the
/// prediction would otherwise have to choose ([ADR-337][adr-337] §2 and §3).
///
/// It refuses unless the design holds one simulation of that name, the simulation states its
/// configuration, rod, wind, site and its own apogee, it ran in the standard atmosphere and in
/// OpenRocket's average wind, the configuration is one the reader flies, with one motor and no
/// staging, and that motor's digest names, in the committed table
/// ([`hpr_motor::openrocket_curves`]), the bundled curve file the reader chose.
///
/// # Errors
///
/// A sentence naming what the stored simulation lacks; `design_path` names the file in it.
///
/// [adr-337]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0337-the-macie-xl-predictions-each-flight-s-stored-simulation-flown-as-saved.md
pub(crate) fn stored_launch<'a>(
    design: &'a ork::Design,
    simulation: &str,
    design_path: &str,
) -> Result<StoredLaunch<'a>, String> {
    let named: Vec<_> = design
        .simulations
        .iter()
        .filter(|stored| stored.name == simulation)
        .collect();
    let [stored] = named.as_slice() else {
        return Err(format!(
            "{design_path} holds {} simulations named `{}`",
            named.len(),
            simulation
        ));
    };
    let unstated = |what: &str| format!("the stored simulation `{}` states no {what}", simulation);
    let conditions = stored
        .conditions
        .as_ref()
        .ok_or_else(|| unstated("launch conditions"))?;
    let configuration_id = conditions
        .configuration
        .as_deref()
        .ok_or_else(|| unstated("motor configuration"))?;
    let rod_length_m = conditions
        .rod_length_m
        .ok_or_else(|| unstated("rod length"))?;
    let rod_angle_rad = conditions
        .rod_angle_rad
        .ok_or_else(|| unstated("rod angle"))?;
    let rod_direction_rad = conditions
        .rod_direction_rad
        .ok_or_else(|| unstated("rod direction"))?;
    let wind_speed_m_s = conditions
        .wind_speed_m_s
        .ok_or_else(|| unstated("wind speed"))?;
    let wind_from_rad = conditions
        .wind_from_rad
        .ok_or_else(|| unstated("wind direction"))?;
    let elevation_m = conditions
        .launch_altitude_m
        .ok_or_else(|| unstated("launch altitude"))?;
    let latitude_deg = conditions
        .latitude_deg
        .ok_or_else(|| unstated("latitude"))?;
    let longitude_deg = conditions
        .longitude_deg
        .ok_or_else(|| unstated("longitude"))?;
    if !matches!(conditions.atmosphere, Some(ork::Atmosphere::Isa {})) {
        return Err(format!(
            "the stored simulation `{}` was not run in the standard atmosphere",
            simulation
        ));
    }
    if !conditions
        .wind_model
        .as_deref()
        .is_some_and(|model| model.eq_ignore_ascii_case("average"))
    {
        return Err(format!(
            "the stored simulation `{}` was not run in an average wind",
            simulation
        ));
    }
    let openrocket_stored_apogee_m = stored
        .results
        .as_ref()
        .and_then(|results| results.max_altitude_m)
        .ok_or_else(|| unstated("apogee of OpenRocket's own run"))?;

    // Its configuration: one motor, lit at launch, on the bundled curve the file's digest names.
    let configuration = design
        .motors
        .configurations
        .iter()
        .find(|configuration| configuration.id == configuration_id)
        .ok_or_else(|| format!("{design_path} has no motor configuration `{configuration_id}`"))?;
    if let Some(left_out) = &configuration.left_out {
        return Err(format!(
            "the configuration of `{}` is not flown: {}",
            simulation, left_out.message
        ));
    }
    if configuration.stagings().next().is_some() {
        return Err(format!(
            "the configuration of `{}` stages, which no prediction here expects",
            simulation
        ));
    }
    let [motor] = configuration.motors.as_slice() else {
        return Err(format!(
            "the configuration of `{}` flies {} motors, not one",
            simulation,
            configuration.motors.len()
        ));
    };
    let (Some(motor_digest), ork::Curve::Catalog { simfile_id, .. }) =
        (&motor.digest, &motor.curve)
    else {
        return Err(format!(
            "{} is not flown on a bundled curve named by a digest",
            motor.designation
        ));
    };
    if hpr_motor::openrocket_curves::file_for(motor_digest).as_deref() != Some(simfile_id.as_str())
    {
        return Err(format!(
            "{}: the bundled curve file {simfile_id} is not the one the design's digest \
             {motor_digest} names",
            motor.designation
        ));
    }
    Ok(StoredLaunch {
        configuration_id,
        rod_length_m,
        rod_angle_rad,
        rod_direction_rad,
        wind_speed_m_s,
        wind_from_rad,
        elevation_m,
        latitude_deg,
        longitude_deg,
        openrocket_stored_apogee_m,
        motor,
        motor_digest,
        simfile_id,
    })
}

/// Predicts one design-file flight: flies the stored simulation the repository names for it,
/// from the design file as saved, and reads its apogee as its log's altimeter would. The log is
/// hashed, never parsed.
///
/// # Errors
///
/// [`RealFlightError`] if a file is missing or doesn't read; the design holds a part HPR Sim
/// doesn't read; the stored simulation is missing, named twice, or lacks a condition; it was
/// run in other than the standard atmosphere and an average wind; its configuration is left
/// out, stages, or flies other than one motor on the bundled curve its digest names; or HPR Sim
/// can't fly it.
pub fn predict_design_file(
    root: &Path,
    flight: &DesignFileFlight,
) -> Result<DesignFilePredictionRow, RealFlightError> {
    let input = |what: String| RealFlightError::Input {
        flight: flight.id.to_owned(),
        what,
    };
    let sim = |source: hpr_sim::SimError| RealFlightError::Sim {
        flight: flight.id.to_owned(),
        source,
    };
    let mut files = Vec::new();
    let mut read = |relative: String, what: &'static str| -> Result<Vec<u8>, RealFlightError> {
        let bytes = read_bytes(root, &relative, what)?;
        files.push(FileRead {
            sha256: sha256_hex(&bytes),
            path: relative,
        });
        Ok(bytes)
    };

    // The design, whole, as OpenRocket saved it.
    let design_path = format!("{}/{}", flight.repository.dest, flight.design);
    let design_bytes = read(
        design_path.clone(),
        "reading a design file, which `cargo xtask refs fetch rocketry-files` fetches,",
    )?;
    let file =
        ork::read(&design_bytes).map_err(|error| input(format!("{design_path}: {error}")))?;
    let design = ork::design(&file.value);
    if design.value.is_reduced() {
        return Err(input(format!(
            "{design_path} holds a part HPR Sim doesn't read, so it is not flown as saved"
        )));
    }

    // The stored simulation the repository names for the flight: its configuration and its
    // launch conditions, each one stated, and its one motor on the curve its digest names.
    let StoredLaunch {
        configuration_id,
        rod_length_m,
        rod_angle_rad,
        rod_direction_rad,
        wind_speed_m_s,
        wind_from_rad,
        elevation_m,
        latitude_deg,
        longitude_deg,
        openrocket_stored_apogee_m,
        motor,
        motor_digest,
        simfile_id,
    } = stored_launch(&design.value, flight.simulation, &design_path).map_err(input)?;
    // The reader's warnings, less those about another configuration's motor.
    let others: Vec<&str> = design
        .value
        .motors
        .configurations
        .iter()
        .flat_map(|other| &other.motors)
        .map(|other| other.designation.as_str())
        .filter(|designation| !designation.is_empty() && *designation != motor.designation)
        .collect();
    let reader_warnings: Vec<String> = file
        .warnings
        .iter()
        .chain(&design.warnings)
        .filter(|warning| {
            warning.message.contains(motor.designation.as_str())
                || !others.iter().any(|other| warning.message.contains(other))
        })
        .map(|warning| format!("{}: {}", warning.at, warning.message))
        .collect();

    // The site, the steady wind and the standard air of the stored simulation.
    let site = Geodetic::from_degrees(latitude_deg, longitude_deg, elevation_m)
        .map_err(|error| sim(error.into()))?;
    let wind = ConstantWind::new(wind_speed_m_s, wind_from_rad)
        .map_err(|error| input(format!("the stored wind: {error}")))?;
    let environment = Environment::standard(site).map_err(sim)?.with_wind(wind);
    let rail = Rail {
        azimuth_rad: rod_direction_rad,
        elevation_rad: std::f64::consts::FRAC_PI_2 - rod_angle_rad,
        ..Rail::vertical(rod_length_m)
    };

    // The log: its bytes' SHA-256, and nothing else of it.
    let log_path = format!("{}/{}", flight.repository.dest, flight.reference.log);
    let log_sha256 = sha256_hex(&read(log_path.clone(), "hashing a flight log")?);

    let flown = flown_to_apogee(
        &design.value.rocket,
        &Launch {
            id: flight.id,
            configuration: configuration_id,
            environment: &environment,
            air: &*environment.atmosphere,
            air_name: "the standard atmosphere",
            elevation_m,
            rail,
            altimeter: flight.reference.altimeter,
        },
    )?;

    Ok(DesignFilePredictionRow {
        id: flight.id.to_owned(),
        title: flight.title.to_owned(),
        repository: flight.repository.url.to_owned(),
        repository_commit: flight.repository.commit.to_owned(),
        repository_license: flight.repository.license.to_owned(),
        design_file_sha256: sha256_hex(&design_bytes),
        design_file: design_path,
        simulation: flight.simulation.to_owned(),
        configuration: configuration_id.to_owned(),
        source: flight.source.to_owned(),
        motor: flight.motor.to_owned(),
        motor_source: flight.motor_source.to_owned(),
        motor_digest: motor_digest.to_owned(),
        curve_file: simfile_id.to_owned(),
        total_impulse_ns: flown.total_impulse_ns,
        mass_source: flight.mass_source.to_owned(),
        mass_without_motor_kg: flown.liftoff_mass_kg - flown.motor_mass_kg,
        liftoff_mass_kg: flown.liftoff_mass_kg,
        conditions_source: flight.conditions_source.to_owned(),
        latitude_deg,
        longitude_deg,
        elevation_m,
        rail_length_m: rail.length_m,
        rail_inclination_deg: rail.elevation_rad.to_degrees(),
        rail_heading_deg: rail.azimuth_rad.to_degrees(),
        wind_speed_m_s,
        wind_from_deg: wind_from_rad.to_degrees(),
        atmosphere: STANDARD_AIR.to_owned(),
        reference_log: log_path,
        reference_log_sha256: log_sha256,
        reference_rule: flight.reference.rule.to_owned(),
        altimeter: flight.reference.altimeter.kind().to_owned(),
        altimeter_evidence: flight.reference.altimeter.evidence().to_owned(),
        all_inputs_traced: flight.untraced.is_empty(),
        untraced: flight
            .untraced
            .iter()
            .map(|&what| what.to_owned())
            .collect(),
        reader_warnings,
        predicted_apogee_m: flown.apogee_m,
        predicted_height_apogee_m: flown.height_apogee_m,
        predicted_time_to_apogee_s: flown.time_to_apogee_s,
        hpr_max_mach: flown.max_mach,
        openrocket_stored_apogee_m,
        files,
        inputs_sha256: sha256_hex(format!("{flight:?}").as_bytes()),
        note: flight.note.to_owned(),
    })
}

/// Predicts every flight in [`NEW_FLIGHTS`], then every one in [`DESIGN_FILE_FLIGHTS`].
///
/// # Errors
///
/// The first flight's [`RealFlightError`].
pub fn run(root: &Path) -> Result<PredictionsReport, RealFlightError> {
    let weather_conversion = conversion(root)?;
    // The file the conversion was made from is the one in the checkout.
    let source = read_bytes(
        root,
        &weather_conversion.source,
        "reading RocketPy's EuRoC 2022 ERA5 file",
    )?;
    if sha256_hex(&source) != weather_conversion.source_sha256 {
        return Err(RealFlightError::Input {
            flight: "the EuRoC 2022 weather".to_owned(),
            what: format!(
                "{} is not the file {CONVERSION_RECORD} was made from: fetch it again with \
                 `cargo xtask refs fetch rocketpy`",
                weather_conversion.source
            ),
        });
    }
    let flights = NEW_FLIGHTS
        .iter()
        .map(|flight| predict(root, flight, &weather_conversion))
        .collect::<Result<Vec<_>, _>>()?;
    let design_file_flights = DESIGN_FILE_FLIGHTS
        .iter()
        .map(|flight| predict_design_file(root, flight))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(PredictionsReport {
        generator: "cargo xtask public-predictions".to_owned(),
        traced_rule: TRACED_RULE.to_owned(),
        weather_conversion,
        flights,
        design_file_traced_rule: DESIGN_FILE_TRACED_RULE.to_owned(),
        design_file_flights,
    })
}

/// Whether the prediction of the flight `id` is the one first merged ([`MERGED`]), to the digit
/// the page prints: its apogee as a reading and as a height, and its catalog curve's where it
/// has one.
fn held_to_merged(
    id: &str,
    apogee_m: f64,
    height_apogee_m: f64,
    catalog_apogee_m: Option<f64>,
) -> Result<(), String> {
    let merged = MERGED
        .iter()
        .find(|merged| merged.id == id)
        .ok_or_else(|| format!("{id}: it has no merged prediction to be held to"))?;
    if merged.catalog_apogee_m.is_some() != catalog_apogee_m.is_some() {
        return Err(format!(
            "{id}: its catalog curve is not its merged prediction's"
        ));
    }
    let held = [
        ("predicted apogee", Some(apogee_m), Some(merged.apogee_m)),
        (
            "predicted apogee as a height",
            Some(height_apogee_m),
            Some(merged.height_apogee_m),
        ),
        (
            "catalog curve's apogee",
            catalog_apogee_m,
            merged.catalog_apogee_m,
        ),
    ];
    for (what, now, first) in held {
        if let (Some(now), Some(first)) = (now, first)
            && (now - first).abs() > MERGED_WITHIN_M
        {
            return Err(format!(
                "{id}: its {what} is {now} m, and its merged prediction is {first} m: a merged \
                 prediction is not changed, it is revised beside the old one (ADR-328 §1b)"
            ));
        }
    }
    Ok(())
}

impl PredictionsReport {
    /// Whether the report holds together without the files it was flown from, as CI checks it:
    /// every prediction is a positive, finite height, each tag is its list's, each apogee is the
    /// one first merged ([`MERGED`]) to the digit the page prints, and the page is this report's
    /// rendering.
    ///
    /// # Errors
    ///
    /// The first thing that doesn't hold, in words.
    pub fn check_consistent(&self, markdown: &str) -> Result<(), String> {
        for row in &self.flights {
            let mut heights = vec![
                ("predicted apogee", row.predicted_apogee_m),
                (
                    "predicted apogee as a height",
                    row.predicted_height_apogee_m,
                ),
                ("time to apogee", row.predicted_time_to_apogee_s),
                ("largest Mach number", row.hpr_max_mach),
                ("impulse", row.total_impulse_ns),
                ("liftoff mass", row.liftoff_mass_kg),
            ];
            if let Some(catalog) = &row.catalog_curve {
                heights.push(("catalog curve's apogee", catalog.predicted_apogee_m));
                heights.push((
                    "catalog curve's apogee as a height",
                    catalog.predicted_height_apogee_m,
                ));
                heights.push(("catalog curve's impulse", catalog.total_impulse_ns));
                heights.push(("catalog curve's liftoff mass", catalog.liftoff_mass_kg));
            }
            if let Some((what, value)) = heights
                .iter()
                .find(|(_, value)| !(value.is_finite() && *value > 0.0))
            {
                return Err(format!("{}: its {what} is {value}", row.id));
            }
            if row.all_inputs_traced != row.untraced.is_empty() {
                return Err(format!("{}: its traced tag is not its list's", row.id));
            }
            if row.liftoff_mass_kg <= row.mass_without_motor_kg {
                return Err(format!(
                    "{}: it lifts off no heavier than without its motor",
                    row.id
                ));
            }
            if row.reference_log_sha256.len() != 64 {
                return Err(format!("{}: its log has no SHA-256", row.id));
            }
            held_to_merged(
                &row.id,
                row.predicted_apogee_m,
                row.predicted_height_apogee_m,
                row.catalog_curve
                    .as_ref()
                    .map(|catalog| catalog.predicted_apogee_m),
            )?;
        }
        for row in &self.design_file_flights {
            let heights = [
                ("predicted apogee", row.predicted_apogee_m),
                (
                    "predicted apogee as a height",
                    row.predicted_height_apogee_m,
                ),
                ("time to apogee", row.predicted_time_to_apogee_s),
                ("largest Mach number", row.hpr_max_mach),
                ("impulse", row.total_impulse_ns),
                ("liftoff mass", row.liftoff_mass_kg),
                ("mass without the motor", row.mass_without_motor_kg),
                (
                    "apogee of OpenRocket's own stored run",
                    row.openrocket_stored_apogee_m,
                ),
            ];
            if let Some((what, value)) = heights
                .iter()
                .find(|(_, value)| !(value.is_finite() && *value > 0.0))
            {
                return Err(format!("{}: its {what} is {value}", row.id));
            }
            if row.all_inputs_traced != row.untraced.is_empty() {
                return Err(format!("{}: its traced tag is not its list's", row.id));
            }
            if row.liftoff_mass_kg <= row.mass_without_motor_kg {
                return Err(format!(
                    "{}: it lifts off no heavier than without its motor",
                    row.id
                ));
            }
            if row.reference_log_sha256.len() != 64 {
                return Err(format!("{}: its log has no SHA-256", row.id));
            }
            if row.design_file_sha256.len() != 64 {
                return Err(format!("{}: its design file has no SHA-256", row.id));
            }
            held_to_merged(
                &row.id,
                row.predicted_apogee_m,
                row.predicted_height_apogee_m,
                None,
            )?;
        }
        // Every merged prediction is here, in the order it was merged.
        let ids = self
            .flights
            .iter()
            .map(|row| row.id.as_str())
            .chain(self.design_file_flights.iter().map(|row| row.id.as_str()));
        if !ids.eq(MERGED.iter().map(|merged| merged.id)) {
            return Err(format!(
                "the {} flights here are not the {} merged predictions, in their order",
                self.flights.len() + self.design_file_flights.len(),
                MERGED.len()
            ));
        }
        if markdown != self.to_markdown() {
            return Err(format!(
                "{PREDICTIONS_MD} is not {PREDICTIONS_JSON}'s rendering: rerun `cargo xtask \
                 public-predictions`"
            ));
        }
        Ok(())
    }

    /// Whether `other`, flown now, is this committed report: the same words, files and digests,
    /// and the same numbers but for the last digits platforms round differently.
    ///
    /// # Errors
    ///
    /// The first difference, in words.
    pub fn reproduces(&self, other: &PredictionsReport) -> Result<(), String> {
        if (
            &self.generator,
            &self.traced_rule,
            &self.weather_conversion,
            &self.design_file_traced_rule,
        ) != (
            &other.generator,
            &other.traced_rule,
            &other.weather_conversion,
            &other.design_file_traced_rule,
        ) {
            return Err("the report's header differs".to_owned());
        }
        if (self.flights.len(), self.design_file_flights.len())
            != (other.flights.len(), other.design_file_flights.len())
        {
            return Err(format!(
                "{} flights committed, {} flown",
                self.flights.len() + self.design_file_flights.len(),
                other.flights.len() + other.design_file_flights.len()
            ));
        }
        let same = crate::report::same_but_for_platform_rounding;
        for (a, b) in self.flights.iter().zip(&other.flights) {
            // Every field that is not a flown number, compared whole.
            let words = |row: &PredictionRow| {
                let mut row = row.clone();
                for value in [
                    &mut row.total_impulse_ns,
                    &mut row.liftoff_mass_kg,
                    &mut row.predicted_apogee_m,
                    &mut row.predicted_height_apogee_m,
                    &mut row.predicted_time_to_apogee_s,
                    &mut row.hpr_max_mach,
                ] {
                    *value = 0.0;
                }
                if let Some(catalog) = &mut row.catalog_curve {
                    catalog.total_impulse_ns = 0.0;
                    catalog.liftoff_mass_kg = 0.0;
                    catalog.predicted_apogee_m = 0.0;
                    catalog.predicted_height_apogee_m = 0.0;
                }
                row
            };
            if words(a) != words(b) {
                return Err(format!(
                    "{}: its inputs, sources or files differ from the committed prediction's",
                    a.id
                ));
            }
            let mut numbers = vec![
                ("impulse", a.total_impulse_ns, b.total_impulse_ns),
                ("liftoff mass", a.liftoff_mass_kg, b.liftoff_mass_kg),
                (
                    "predicted apogee",
                    a.predicted_apogee_m,
                    b.predicted_apogee_m,
                ),
                (
                    "predicted apogee as a height",
                    a.predicted_height_apogee_m,
                    b.predicted_height_apogee_m,
                ),
                (
                    "time to apogee",
                    a.predicted_time_to_apogee_s,
                    b.predicted_time_to_apogee_s,
                ),
                ("largest Mach number", a.hpr_max_mach, b.hpr_max_mach),
            ];
            if let (Some(x), Some(y)) = (&a.catalog_curve, &b.catalog_curve) {
                numbers.push((
                    "catalog curve's impulse",
                    x.total_impulse_ns,
                    y.total_impulse_ns,
                ));
                numbers.push((
                    "catalog curve's liftoff mass",
                    x.liftoff_mass_kg,
                    y.liftoff_mass_kg,
                ));
                numbers.push((
                    "catalog curve's apogee",
                    x.predicted_apogee_m,
                    y.predicted_apogee_m,
                ));
                numbers.push((
                    "catalog curve's apogee as a height",
                    x.predicted_height_apogee_m,
                    y.predicted_height_apogee_m,
                ));
            }
            if let Some((what, x, y)) = numbers.iter().find(|(_, x, y)| !same(*x, *y)) {
                return Err(format!(
                    "{}: the {what} is {x} committed and {y} flown: a merged prediction is not \
                     changed, it is revised beside the old one (ADR-328 §1b)",
                    a.id
                ));
            }
        }
        for (a, b) in self
            .design_file_flights
            .iter()
            .zip(&other.design_file_flights)
        {
            // Every field that is not a flown number, compared whole: the words, the files and
            // their digests, the stored simulation's conditions and OpenRocket's stored apogee.
            let words = |row: &DesignFilePredictionRow| {
                let mut row = row.clone();
                for value in [
                    &mut row.total_impulse_ns,
                    &mut row.mass_without_motor_kg,
                    &mut row.liftoff_mass_kg,
                    &mut row.predicted_apogee_m,
                    &mut row.predicted_height_apogee_m,
                    &mut row.predicted_time_to_apogee_s,
                    &mut row.hpr_max_mach,
                ] {
                    *value = 0.0;
                }
                row
            };
            if words(a) != words(b) {
                return Err(format!(
                    "{}: its inputs, sources or files differ from the committed prediction's",
                    a.id
                ));
            }
            let numbers = [
                ("impulse", a.total_impulse_ns, b.total_impulse_ns),
                (
                    "mass without the motor",
                    a.mass_without_motor_kg,
                    b.mass_without_motor_kg,
                ),
                ("liftoff mass", a.liftoff_mass_kg, b.liftoff_mass_kg),
                (
                    "predicted apogee",
                    a.predicted_apogee_m,
                    b.predicted_apogee_m,
                ),
                (
                    "predicted apogee as a height",
                    a.predicted_height_apogee_m,
                    b.predicted_height_apogee_m,
                ),
                (
                    "time to apogee",
                    a.predicted_time_to_apogee_s,
                    b.predicted_time_to_apogee_s,
                ),
                ("largest Mach number", a.hpr_max_mach, b.hpr_max_mach),
            ];
            if let Some((what, x, y)) = numbers.iter().find(|(_, x, y)| !same(*x, *y)) {
                return Err(format!(
                    "{}: the {what} is {x} committed and {y} flown: a merged prediction is not \
                     changed, it is revised beside the old one (ADR-328 §1b)",
                    a.id
                ));
            }
        }
        Ok(())
    }

    /// The predictions as a page.
    #[must_use]
    pub fn to_markdown(&self) -> String {
        let mut out = String::new();
        let weather = &self.weather_conversion;
        let _ = write!(
            out,
            "# Public flights predicted before they are compared\n\n\
             Written by `cargo xtask public-predictions` ([the EuRoC 2022 predictions, \
             M2.3d3][m2-3d3], and [the Macie XL predictions, M2.3d5][m2-3d5]; decisions: [twelve \
             public flights, predictions first, ADR-328][adr-328], [the predictions in two \
             parts, ADR-334][adr-334] and [the Macie XL predictions, ADR-337][adr-337]). What it \
             is for is on the [documentation site][site].\n\n\
             - **What this is:** {} public real flights that join the seven of the \
             [real-flights report][real], each flown by HPR Sim from its record alone. Each \
             flight's prediction was merged before its log was compared: the three EuRoC 2022 \
             flights' first, then the two Macie XL flights'. Their measured apogees were \
             already public, so the targets were known; what merging first does is make an input \
             changed after seeing HPR Sim's own miss show as a change to a merged number. The \
             code holds each prediction to the number first merged.\n\
             - **What it holds:** each flight's inputs with their sources, the rule its log's \
             apogee will be read by, and HPR Sim's predicted apogee. It holds no measured apogee \
             and no error: those come in a later pull request, which must print each prediction \
             below beside its error.\n\
             - **How it is flown:** the three EuRoC 2022 flights by the seven's method. The \
             design is built from the example's inputs and flown on the example's own thrust \
             file, from its rail and site, in the ERA5 air of its launch hour, on HPR Sim's own \
             aerodynamics. The two Macie XL flights have no example: their design is an \
             OpenRocket file in a pinned public repository, read whole as OpenRocket 24.12 \
             saved it. Each is flown as the stored simulation the repository names for it: on \
             its motor, from its rod, in its steady wind and standard atmosphere, on HPR Sim's \
             own aerodynamics.\n\
             - **The predicted apogee** is read as the log's altimeter would read it: the standard \
             atmosphere's altitude of the flown air's pressure at the apogee, less the pad's. The \
             height \
             climbed is beside it.\n\
             - **Traced:** {} {}\n\
             - **Files:** the examples' thrust files and the logs are read from the pinned \
             RocketPy checkout (`refs/rocketpy`, which `cargo xtask refs fetch rocketpy` \
             fetches), and the weather file is converted from the checkout's. The Macie XL \
             design file and logs are read from the pinned `{}` checkout (`{}`, which `cargo \
             xtask refs fetch {}` fetches). None is \
             committed, and each one's SHA-256 is in the JSON. A log's \
             bytes are hashed to pin the file its rule will be applied to; they are never read as \
             numbers here.\n\n\
             [m2-3d3]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m2-3d3\n\
             [adr-328]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0328-the-2026-11-08-review-twelve-public-flights-predictions-first.md\n\
             [adr-334]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0334-m2-3d1-in-three-the-euroc-2022-predictions-first.md\n\
             [m2-3d5]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m2-3d5\n\
             [adr-337]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0337-the-macie-xl-predictions-each-flight-s-stored-simulation-flown-as-saved.md\n\
             [site]: https://hpr.fusionspace.co/accuracy.html#predicted-before-compared\n\
             [real]: real-flights.md\n\n",
            self.flights.len() + self.design_file_flights.len(),
            self.traced_rule,
            self.design_file_traced_rule,
            ROCKETRY_FILES.name,
            ROCKETRY_FILES.dest,
            ROCKETRY_FILES.name,
        );
        out.push_str(
            "## Predictions\n\n\
             | flight | impulse flown (N s) | liftoff mass (kg) | predicted apogee (m) | as a \
             height (m) | time to apogee (s) | largest Mach | every input traced |\n\
             |---|---:|---:|---:|---:|---:|---:|---|\n",
        );
        for row in &self.flights {
            let _ = writeln!(
                out,
                "| {} | {} | {} | {} | {} | {} | {} | {} |",
                row.title,
                fmt_number(row.total_impulse_ns, 1),
                fmt_number(row.liftoff_mass_kg, 3),
                fmt_number(row.predicted_apogee_m, 1),
                fmt_number(row.predicted_height_apogee_m, 1),
                fmt_number(row.predicted_time_to_apogee_s, 2),
                fmt_number(row.hpr_max_mach, 3),
                if row.all_inputs_traced { "yes" } else { "no" },
            );
        }
        for row in &self.design_file_flights {
            let _ = writeln!(
                out,
                "| {} | {} | {} | {} | {} | {} | {} | {} |",
                row.title,
                fmt_number(row.total_impulse_ns, 1),
                fmt_number(row.liftoff_mass_kg, 3),
                fmt_number(row.predicted_apogee_m, 1),
                fmt_number(row.predicted_height_apogee_m, 1),
                fmt_number(row.predicted_time_to_apogee_s, 2),
                fmt_number(row.hpr_max_mach, 3),
                if row.all_inputs_traced { "yes" } else { "no" },
            );
        }
        let catalog: Vec<(&PredictionRow, &CatalogCurveRow)> = self
            .flights
            .iter()
            .filter_map(|row| row.catalog_curve.as_ref().map(|catalog| (row, catalog)))
            .collect();
        // A title without the bracket that says which curve the counted flight is on.
        let name = |row: &PredictionRow| -> String {
            row.title
                .rsplit_once(" (")
                .map_or(row.title.as_str(), |(name, _)| name)
                .to_owned()
        };
        if !catalog.is_empty() {
            out.push_str(
                "\n## On the catalog curve, never counted\n\n\
                 The same design, at the same mass, on the bundled certified curve of the motor \
                 its record names, where the counted flight is on another curve. It is here so \
                 the reader can see what the choice of curve moves; it enters no mean.\n\n\
                 | flight | catalog motor | impulse flown (N s) | liftoff mass (kg) | predicted \
                 apogee (m) | as a height (m) |\n|---|---|---:|---:|---:|---:|\n",
            );
            for (row, catalog) in &catalog {
                let _ = writeln!(
                    out,
                    "| {} | {} | {} | {} | {} | {} |",
                    name(row),
                    catalog.motor,
                    fmt_number(catalog.total_impulse_ns, 1),
                    fmt_number(catalog.liftoff_mass_kg, 3),
                    fmt_number(catalog.predicted_apogee_m, 1),
                    fmt_number(catalog.predicted_height_apogee_m, 1),
                );
            }
            out.push('\n');
            for (row, catalog) in &catalog {
                let _ = writeln!(out, "- **{}:** {}.", name(row), catalog.why);
            }
        }
        out.push_str("\n## Each flight's inputs and its apogee rule\n\n");
        for row in &self.flights {
            let _ = writeln!(
                out,
                "- **{}** (`{}`, design `{}`):\n  \
                 - Design, site and rail: RocketPy 1.13.0's {}. The site is {}°, {}° at {} m; the \
                 rail is {} m long, {}° from the horizontal, heading {}°.\n  \
                 - Motor: {} ({}). Thrust file `{}`{}.\n  \
                 - Mass: {} kg without the motor, {} kg at liftoff ({}).\n  \
                 - Weather: ERA5 at {}, from the converted file below.\n  \
                 - Apogee rule: {}, in `{}`. Altimeter: {} ({}).\n  \
                 - Every input traced: {}{}",
                row.title,
                row.id,
                row.design,
                row.source,
                row.latitude_deg,
                row.longitude_deg,
                row.elevation_m,
                row.rail_length_m,
                row.rail_inclination_deg,
                row.rail_heading_deg,
                row.motor,
                row.motor_source,
                row.thrust_file,
                row.burn_time_s
                    .map_or_else(String::new, |burn| format!(", burn time {burn} s")),
                row.mass_without_motor_kg,
                fmt_number(row.liftoff_mass_kg, 3),
                row.mass_source,
                row.weather_time_utc,
                row.reference_rule,
                row.reference_log,
                row.altimeter,
                row.altimeter_evidence,
                if row.all_inputs_traced {
                    "yes.".to_owned()
                } else {
                    format!("no. Not traced: {}.", row.untraced.join("; "))
                },
                if row.note.is_empty() {
                    String::new()
                } else {
                    format!(" Note: {}.", row.note)
                },
            );
        }
        for row in &self.design_file_flights {
            let _ = writeln!(
                out,
                "- **{}** (`{}`):\n  \
                 - Design: `{}`, SHA-256 `{}`, at commit `{}` of `{}` ({}). From {}.\n  \
                 - Flown as: the stored simulation `{}`, motor configuration `{}` ({}). The site \
                 is {}°, {}° at {} m; the rod is {} m long, {}° from the horizontal, heading \
                 {}°; the wind is a steady {} m/s from {}°; the air is {}.\n  \
                 - Motor: {} ({}). Curve file `{}`, for the digest `{}`.\n  \
                 - Mass: {} kg without the motor, {} kg at liftoff ({}).\n  \
                 - Apogee rule: {}, in `{}`. Altimeter: {} ({}).\n  \
                 - Every input traced: {}{}\n  \
                 - OpenRocket's own run of this simulation, stored in the design file, reached \
                 {} m. That is another simulator's number, not a measurement; it is shown and \
                 never counted.\n  \
                 - The reader's warnings on this file and motor are in the JSON under this \
                 flight's `reader_warnings`: {} in all.",
                row.title,
                row.id,
                row.design_file,
                row.design_file_sha256,
                row.repository_commit,
                row.repository,
                row.repository_license,
                row.source,
                row.simulation,
                row.configuration,
                row.conditions_source,
                row.latitude_deg,
                row.longitude_deg,
                row.elevation_m,
                row.rail_length_m,
                fmt_number(row.rail_inclination_deg, 1),
                fmt_number(row.rail_heading_deg, 1),
                row.wind_speed_m_s,
                fmt_number(row.wind_from_deg, 1),
                row.atmosphere,
                row.motor,
                row.motor_source,
                row.curve_file,
                row.motor_digest,
                fmt_number(row.mass_without_motor_kg, 3),
                fmt_number(row.liftoff_mass_kg, 3),
                row.mass_source,
                row.reference_rule,
                row.reference_log,
                row.altimeter,
                row.altimeter_evidence,
                if row.all_inputs_traced {
                    "yes.".to_owned()
                } else {
                    format!("no. Not traced: {}.", row.untraced.join("; "))
                },
                if row.note.is_empty() {
                    String::new()
                } else {
                    format!(" Note: {}.", row.note)
                },
                fmt_number(row.openrocket_stored_apogee_m, 1),
                row.reader_warnings.len(),
            );
        }
        let _ = write!(
            out,
            "\n## The weather file\n\n\
             The three EuRoC 2022 flights' ERA5 file is netCDF-4, which HPR Sim does not read. \
             `validation/oracles/netcdf/euroc_2022.py` converts it as the [guide][era5] says, \
             `{}`, and checks the result: the four fields hold the source's values bit for bit, \
             and RocketPy reads the same levels from both files at each flight's site and hour. \
             Its record is `{}`.\n\n\
             | file | SHA-256 |\n|---|---|\n\
             | `{}` (netCDF-4, as RocketPy ships it) | `{}` |\n\
             | `{}` (converted, flown) | `{}` |\n\n\
             [era5]: https://hpr.fusionspace.co/format/era5.html#converting-a-current-file\n",
            weather.conversion,
            weather.record,
            weather.source,
            weather.source_sha256,
            weather.converted,
            weather.converted_sha256,
        );
        out
    }
}
