//! OpenRocket's thrust, mass, center of mass and drag coefficient along its own flights, and HPR
//! Sim's own-history test (M2.3c11, ADR-349).
//!
//! `validation/oracles/openrocket/histories.py` records OpenRocket 24.12's inputs at its own
//! times: for two public designs in `own::PUBLIC_RECORD`, committed, and for the collection's
//! flights in [`COLLECTION_RECORD`], which stays under the gitignored `corpus-out/`. A flight
//! that lacks an input says why, under a reason from the oracle's closed list ([`REASONS`]).
//!
//! The own-history test bounds what a history's resampling costs: a flight is flown, its own
//! thrust, mass and center of mass are sampled at the times OpenRocket's flight of the design
//! holds, and the flight is flown again on those histories in place of its own
//! ([`hpr_sim::ThrustHistory`], [`hpr_sim::MassHistory`]). Its apogee must come back within
//! `own::OWN_HISTORY_BOUND`, 0.05%, of the first, about a thirtieth of the 1.63% mean absolute
//! gap between the two programs' apogees on 60 real flights (ADR-349 §3).

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::fixture_flights::Tally;

/// OpenRocket's inputs along its flights of the collection (private, ADR-151).
pub(crate) const COLLECTION_RECORD: &str = "corpus-out/hpr-sim-fixtures/openrocket-histories.json";

/// The script that writes both records.
pub(crate) const ORACLE: &str = "validation/oracles/openrocket/histories.py";

/// The oracle's reasons a flight lacks an input, in its order, with the report's words for
/// each: the record's `reasons` must name exactly these.
pub(crate) const REASONS: [(&str, &str); 6] = [
    ("not_flown", "OpenRocket didn't fly it to an apogee"),
    (
        "another_flight",
        "the flight flown isn't the one OpenRocket's record above holds: its apogee moved by \
         more than 1e-5 of itself",
    ),
    (
        "thrust_not_finite",
        "OpenRocket gave a thrust as not a number",
    ),
    ("mass_not_finite", "OpenRocket gave a mass as not a number"),
    (
        "cg_not_finite",
        "OpenRocket gave a center of mass as not a number",
    ),
    (
        "drag_not_finite",
        "OpenRocket gave a drag coefficient as not a number above Mach 0",
    ),
];

/// How many of the collection's flights hold OpenRocket's inputs, and why the rest don't.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct Inputs {
    /// The flights the record holds: every flight OpenRocket was given.
    pub flights: usize,
    /// The flights holding every input.
    pub holding_all: usize,
    /// The rest, by the oracle's reason: every reason of [`REASONS`], in its order.
    pub lacking: Vec<Tally>,
    /// The largest move of a flight's apogee from the one OpenRocket's flight record holds,
    /// relative, over the flights compared, to two significant digits: how far this run of
    /// OpenRocket moved from the recorded one, which isn't bit-identical from run to run.
    pub largest_apogee_move: f64,
}

impl Inputs {
    /// Holds the counts to each other and to the `flown_by_openrocket` flights of the report.
    pub(crate) fn check_consistent(&self, flown_by_openrocket: usize) -> Result<(), String> {
        let reasons: Vec<&str> = self.lacking.iter().map(|t| t.reason.as_str()).collect();
        let expected: Vec<&str> = REASONS.iter().map(|(name, _)| *name).collect();
        if reasons != expected {
            return Err(format!(
                "inputs: the reasons are {reasons:?}, not {expected:?}"
            ));
        }
        let lacking: usize = self.lacking.iter().map(|t| t.flights).sum();
        if self.holding_all + lacking != self.flights {
            return Err(format!(
                "inputs: {} holding all and {lacking} lacking one are not the {} flights",
                self.holding_all, self.flights
            ));
        }
        let not_flown = self.lacking[0].flights;
        if self.flights - not_flown != flown_by_openrocket {
            return Err(format!(
                "inputs: {} flights less {not_flown} not flown are not the {flown_by_openrocket} \
                 OpenRocket flies",
                self.flights
            ));
        }
        Ok(())
    }

    /// The report's section.
    pub(crate) fn to_markdown(&self) -> String {
        let mut page = String::new();
        let _ = writeln!(page, "\n## OpenRocket's inputs along its flights\n");
        let _ = writeln!(
            page,
            "For M2.3c11 (ADR-349), `{ORACLE}` flies each flight again as OpenRocket's flights \
             above are flown and records, at OpenRocket's own time steps from launch to apogee, \
             its thrust, mass, center of mass and drag coefficient. The record stays on this \
             machine. M2.3c12 flies HPR Sim on those inputs to split the gap between the two \
             simulators by cause. Of the {} flights OpenRocket was given, {} hold every input. \
             Each of the rest is counted under the first reason, from the oracle's own list, \
             that applies to it. OpenRocket isn't bit-identical from one run to the next: run \
             again, the flights' apogees moved from those its record above holds by up to {:.1e} \
             of themselves, so a flight is taken as the recorded one within 1e-5.\n",
            self.flights, self.holding_all, self.largest_apogee_move
        );
        let _ = writeln!(
            page,
            "| why a flight lacks an input | flights |\n|---|---:|"
        );
        for (tally, (_, words)) in self.lacking.iter().zip(REASONS) {
            let _ = writeln!(page, "| {words} | {} |", tally.flights);
        }
        page
    }
}

/// `value` rounded to `digits` significant digits.
pub(crate) fn significant(value: f64, digits: i32) -> f64 {
    if value == 0.0 {
        return 0.0;
    }
    let scale = 10f64.powi(digits - 1 - value.abs().log10().floor() as i32);
    (value * scale).round() / scale
}

/// The collection's record, checked against the scripts and conditions that wrote it: its counts,
/// and its flights by id, each as [`crate::gap_split::OpenRocketInputs::of`] reads it.
pub(crate) fn collection_inputs(
    root: &Path,
    conditions: &str,
) -> Result<(Inputs, BTreeMap<String, Value>), String> {
    let rerun = format!("run `refs/venv/bin/python {ORACLE} --collection {COLLECTION_RECORD}`");
    let record = crate::ork_flights::read_json(&root.join(COLLECTION_RECORD))
        .map_err(|error| format!("{error}; {rerun}"))?;
    let stale = |what: &str| format!("{COLLECTION_RECORD} was written from other {what}; {rerun}");
    let inputs = record["inputs_sha256"]
        .as_object()
        .filter(|inputs| inputs.contains_key(ORACLE))
        .ok_or_else(|| format!("{COLLECTION_RECORD} has no inputs_sha256 for {ORACLE}"))?;
    for (path, recorded) in inputs {
        let bytes = std::fs::read(root.join(path)).map_err(|error| format!("{path}: {error}"))?;
        if recorded.as_str() != Some(crate::ork_supply::sha256(&bytes).as_str()) {
            return Err(stale(&format!("scripts ({path} differs)")));
        }
    }
    if record["conditions_sha256"].as_str()
        != Some(crate::ork_supply::sha256(conditions.as_bytes()).as_str())
    {
        return Err(stale("conditions"));
    }
    let flights_record = std::fs::read(root.join(crate::fixture_flights::RECORD))
        .map_err(|error| format!("{}: {error}", crate::fixture_flights::RECORD))?;
    if record["flights_record_sha256"].as_str()
        != Some(crate::ork_supply::sha256(&flights_record).as_str())
    {
        return Err(stale("OpenRocket flights"));
    }
    if !record["openrocket"]
        .as_str()
        .is_some_and(|v| v.starts_with("24.12"))
    {
        return Err(format!("{COLLECTION_RECORD} is not OpenRocket 24.12's"));
    }
    let mut named: Vec<&str> = record["reasons"]
        .as_object()
        .ok_or_else(|| format!("{COLLECTION_RECORD} lists no reasons"))?
        .keys()
        .map(String::as_str)
        .collect();
    let mut expected: Vec<&str> = REASONS.iter().map(|(name, _)| *name).collect();
    named.sort_unstable();
    expected.sort_unstable();
    if named != expected {
        return Err(format!(
            "{COLLECTION_RECORD}'s reasons are {named:?}, not the {expected:?} this report words"
        ));
    }
    let columns: Vec<&str> = record["columns"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .collect();
    if columns != crate::gap_split::COLUMNS {
        return Err(format!(
            "{COLLECTION_RECORD}'s columns are {columns:?}, not the {:?} the split reads",
            crate::gap_split::COLUMNS
        ));
    }
    let flights = record["flights"]
        .as_array()
        .ok_or_else(|| format!("{COLLECTION_RECORD} has no flights"))?;
    let mut largest: f64 = 0.0;
    for flight in flights {
        if let Some(moved) = flight.get("apogee_moved") {
            let moved = moved.as_f64().filter(|m| m.is_finite()).ok_or_else(|| {
                format!("{COLLECTION_RECORD}: an apogee move that isn't a number")
            })?;
            largest = largest.max(moved.abs());
        }
    }
    let mut counted = Inputs {
        flights: flights.len(),
        largest_apogee_move: significant(largest, 2),
        holding_all: 0,
        lacking: REASONS
            .iter()
            .map(|(name, _)| Tally {
                reason: (*name).to_owned(),
                flights: 0,
            })
            .collect(),
    };
    for flight in flights {
        match flight.get("lacks") {
            None => counted.holding_all += 1,
            Some(reason) => {
                let tally = counted
                    .lacking
                    .iter_mut()
                    .find(|tally| Some(tally.reason.as_str()) == reason.as_str())
                    .ok_or_else(|| format!("{COLLECTION_RECORD}: an unknown reason {reason}"))?;
                tally.flights += 1;
            }
        }
    }
    let mut by_id = BTreeMap::new();
    for flight in flights {
        let id = flight["flight_id"]
            .as_str()
            .ok_or_else(|| format!("{COLLECTION_RECORD}: a flight without an id"))?;
        if by_id.insert(id.to_owned(), flight.clone()).is_some() {
            return Err(format!("{COLLECTION_RECORD} holds a flight twice"));
        }
    }
    Ok((counted, by_id))
}

/// The own-history test's flights, which M2.3c12 also flies on the collection
/// ([`crate::gap_split`]).
pub(crate) mod own {
    use hpr_sim::{
        EventKind, FlightResult, FlightStep, MassHistory, Observer, SimError, Simulation,
        ThrustHistory,
    };
    #[cfg(test)]
    use serde_json::Value;

    /// OpenRocket's inputs along its flights of the public designs, which the test samples at.
    #[cfg(test)]
    pub(crate) const PUBLIC_RECORD: &str = "validation/fixtures/ork/openrocket-histories.json";

    /// The largest relative difference of the own-history flight's apogee from the flight's own.
    pub(crate) const OWN_HISTORY_BOUND: f64 = 5e-4;

    /// A flight and its own-history flight.
    #[derive(Debug, Clone, PartialEq)]
    pub(crate) struct OwnHistory {
        /// The flight's apogee above the site, m.
        pub(crate) apogee_m: f64,
        /// The apogee flying the histories sampled from it, m.
        pub(crate) history_apogee_m: f64,
        /// The flight's largest Mach number.
        pub(crate) max_mach: f64,
        /// Its thrust, sampled.
        pub(crate) thrust: ThrustHistory,
        /// Its mass and center of mass, sampled.
        pub(crate) mass: MassHistory,
    }

    impl OwnHistory {
        /// The own-history apogee's difference from the flight's own, relative.
        pub(crate) fn relative(&self) -> f64 {
            (self.history_apogee_m - self.apogee_m) / self.apogee_m
        }
    }

    /// Samples a flight's thrust and mass at given times as its steps pass them.
    struct Sampler<'a> {
        times_s: &'a [f64],
        next: usize,
        thrust_n: Vec<f64>,
        mass_kg: Vec<f64>,
        max_mach: f64,
    }

    impl Observer for Sampler<'_> {
        fn step(&mut self, step: &dyn FlightStep) -> Result<(), SimError> {
            while let Some(&t) = self.times_s.get(self.next) {
                if t > step.end_s() {
                    break;
                }
                let sample = step.sample(t.max(step.start_s()))?;
                self.thrust_n.push(sample.thrust_n);
                self.mass_kg.push(sample.mass_kg);
                self.next += 1;
            }
            let end = step.sample(step.end_s())?;
            self.max_mach = self.max_mach.max(end.mach);
            Ok(())
        }
    }

    pub(crate) fn apogee_m(result: &FlightResult) -> Result<f64, String> {
        result
            .event(EventKind::Apogee)
            .map(|event| event.sample.height_above_ground_m)
            .ok_or_else(|| "the flight has no apogee".to_owned())
    }

    /// Flies `build`'s simulation, samples its thrust, mass and center of mass at `times_s` (rising,
    /// the first at launch), and flies it again with those as histories in place of its own.
    pub(crate) fn own_history(
        build: impl Fn() -> Result<Simulation, String>,
        times_s: &[f64],
    ) -> Result<OwnHistory, String> {
        let simulation = build()?;
        let mut sampler = Sampler {
            times_s,
            next: 0,
            thrust_n: Vec::with_capacity(times_s.len()),
            mass_kg: Vec::with_capacity(times_s.len()),
            max_mach: 0.0,
        };
        let result = simulation
            .run(&mut sampler)
            .map_err(|error| error.to_string())?;
        if sampler.next != times_s.len() {
            return Err(format!(
                "the flight ended at {} of {} sample times",
                sampler.next,
                times_s.len()
            ));
        }
        let cg_m: Vec<f64> = times_s
            .iter()
            .map(|&t| -simulation.mass_properties(&result, t).cg_m.z)
            .collect();
        let thrust = ThrustHistory::new(times_s.to_vec(), sampler.thrust_n)
            .map_err(|error| error.to_string())?;
        let mass = MassHistory::new(times_s.to_vec(), sampler.mass_kg, cg_m)
            .map_err(|error| error.to_string())?;
        Ok(OwnHistory {
            apogee_m: apogee_m(&result)?,
            history_apogee_m: apogee_m(&flown_on(&build, thrust.clone(), mass.clone())?)?,
            max_mach: sampler.max_mach,
            thrust,
            mass,
        })
    }

    /// `build`'s simulation flown on `thrust` and `mass` in place of its own.
    pub(crate) fn flown_on(
        build: impl Fn() -> Result<Simulation, String>,
        thrust: ThrustHistory,
        mass: MassHistory,
    ) -> Result<FlightResult, String> {
        build()?
            .with_thrust_history(thrust)
            .and_then(|s| s.with_mass_history(mass))
            .map_err(|error| error.to_string())?
            .run(&mut ())
            .map_err(|error| error.to_string())
    }

    /// The times of a record flight's rows.
    #[cfg(test)]
    pub(crate) fn times_s(flight: &Value) -> Result<Vec<f64>, String> {
        flight["rows"]
            .as_array()
            .ok_or("a flight without rows")?
            .iter()
            .map(|row| {
                row[crate::gap_split::TIME]
                    .as_f64()
                    .ok_or_else(|| "a row without a time".to_owned())
            })
            .collect()
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::own::{
        OWN_HISTORY_BOUND, OwnHistory, PUBLIC_RECORD, apogee_m, flown_on, own_history, times_s,
    };
    use super::*;
    use hpr_core::geodesy::Geodetic;
    use hpr_io::ork;
    use hpr_sim::{
        Environment, EventKind, FlightResult, FlightSettings, MassHistory, Rail, Simulation,
        ThrustHistory,
    };
    use serde_json::Value;

    fn record() -> Value {
        let root = crate::ork::root().unwrap();
        serde_json::from_str(&std::fs::read_to_string(root.join(PUBLIC_RECORD)).unwrap()).unwrap()
    }

    /// A public flight as OpenRocket flew it: the design, its configuration, the site and the
    /// rod, and OpenRocket's times.
    pub(crate) struct Public {
        pub(crate) file: String,
        rocket: hpr_design::Rocket,
        configuration: String,
        site: Geodetic,
        rail: Rail,
        times_s: Vec<f64>,
        /// OpenRocket's record of the flight.
        pub(crate) flight: Value,
    }

    impl Public {
        pub(crate) fn build(&self) -> Result<Simulation, String> {
            self.build_at(self.site.height_m)
        }

        /// The flight from the same place `height_m` above the ellipsoid, in the standard air
        /// there.
        pub(crate) fn build_at(&self, height_m: f64) -> Result<Simulation, String> {
            let site = Geodetic {
                height_m,
                ..self.site
            };
            Simulation::new(
                &self.rocket,
                &self.configuration,
                Environment::standard(site).map_err(|e| e.to_string())?,
                self.rail,
                FlightSettings {
                    accept_design_errors: true,
                    ..FlightSettings::default()
                },
            )
            .map_err(|e| e.to_string())
        }
    }

    pub(crate) fn public_flights() -> Vec<Public> {
        let root = crate::ork::root().unwrap();
        let record = record();
        let mut out = Vec::new();
        for design in record["designs"].as_array().unwrap() {
            let file = design["file"].as_str().unwrap();
            let bytes = std::fs::read(root.join(file)).unwrap();
            assert_eq!(
                crate::ork_supply::sha256(&bytes),
                design["sha256"].as_str().unwrap(),
                "{file} is not the file OpenRocket flew"
            );
            let read = ork::read(&bytes).unwrap();
            let built = ork::design(&read.value).value;
            for flight in design["flights"].as_array().unwrap() {
                assert!(flight.get("lacks").is_none(), "{file} lacks an input");
                let conditions = &flight["conditions"];
                let number = |key: &str| conditions[key].as_f64().unwrap();
                assert_eq!(conditions["isa_atmosphere"], true);
                assert_eq!(number("wind_average_m_s"), 0.0);
                out.push(Public {
                    file: file.to_owned(),
                    rocket: built.rocket.clone(),
                    configuration: flight["configuration"].as_str().unwrap().to_owned(),
                    site: Geodetic::from_degrees(
                        number("launch_latitude_deg"),
                        number("launch_longitude_deg"),
                        number("launch_altitude_m"),
                    )
                    .unwrap(),
                    rail: crate::ork_flights::rod_rail(conditions).unwrap(),
                    times_s: times_s(flight).unwrap(),
                    flight: flight.clone(),
                });
            }
        }
        out
    }

    /// The public record was written by the scripts in the tree, with the reasons this module
    /// words.
    #[test]
    fn the_public_record_is_the_oracles_and_names_the_reasons() {
        let root = crate::ork::root().unwrap();
        let record = record();
        let inputs = record["inputs_sha256"].as_object().unwrap();
        assert!(inputs.contains_key(ORACLE));
        for (path, recorded) in inputs {
            let bytes = std::fs::read(root.join(path)).unwrap();
            assert_eq!(
                recorded.as_str(),
                Some(crate::ork_supply::sha256(&bytes).as_str()),
                "{path} changed since {PUBLIC_RECORD} was written: run {ORACLE} --public again"
            );
        }
        assert!(record["openrocket"].as_str().unwrap().starts_with("24.12"));
        let mut named: Vec<&str> = record["reasons"]
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect();
        named.sort_unstable();
        let mut expected: Vec<&str> = REASONS.iter().map(|(name, _)| *name).collect();
        expected.sort_unstable();
        assert_eq!(named, expected);
    }

    #[test]
    fn own_histories_at_openrockets_times_come_back_within_the_bound() {
        let flights = public_flights();
        let flown: Vec<(&Public, OwnHistory)> = flights
            .iter()
            .map(|public| {
                (
                    public,
                    own_history(|| public.build(), &public.times_s).unwrap(),
                )
            })
            .collect();
        for (public, own) in &flown {
            eprintln!(
                "{}: apogee {:.4} m, on its own histories {:.4} m ({:+.5}%), Mach {:.3}, {} times",
                public.file,
                own.apogee_m,
                own.history_apogee_m,
                100.0 * own.relative(),
                own.max_mach,
                public.times_s.len()
            );
        }
        assert!(flown.len() >= 2, "two public designs at least");
        assert!(
            flown.iter().any(|(_, own)| own.max_mach > 1.0),
            "one of them passes Mach 1"
        );
        for (public, own) in &flown {
            assert!(
                own.relative().abs() <= OWN_HISTORY_BOUND,
                "{}: {:+.5}%",
                public.file,
                100.0 * own.relative()
            );
        }
    }

    /// Each history is flown: 5% more thrust raises the apogee by more than 20 times the bound,
    /// and 20% more mass delays the rail exit by more than 5%. The apogee is no probe of the mass:
    /// more mass can raise it or lower it. The rail exit's time grows with the mass for any
    /// rocket on a fixed thrust.
    #[test]
    fn each_history_moves_the_apogee() {
        for public in public_flights() {
            let own = own_history(|| public.build(), &public.times_s).unwrap();
            let times = public.times_s.clone();
            let scaled = |f: &dyn Fn(f64) -> f64| times.iter().map(|&t| f(t)).collect::<Vec<_>>();
            let thrust =
                ThrustHistory::new(times.clone(), scaled(&|t| 1.05 * own.thrust.thrust_n(t)))
                    .unwrap();
            let heavier = MassHistory::new(
                times.clone(),
                scaled(&|t| 1.20 * own.mass.mass_kg(t)),
                scaled(&|t| own.mass.cg_aft_of_nose_m(t)),
            )
            .unwrap();
            let rail_exit_s =
                |flight: &FlightResult| flight.event(EventKind::RailExit).unwrap().sample.time_s;
            let own_flight =
                flown_on(|| public.build(), own.thrust.clone(), own.mass.clone()).unwrap();
            let pushed = flown_on(|| public.build(), thrust, own.mass.clone()).unwrap();
            let weighed = flown_on(|| public.build(), own.thrust.clone(), heavier).unwrap();
            let apogee_moved =
                (apogee_m(&pushed).unwrap() - own.history_apogee_m) / own.history_apogee_m;
            let exit_moved = rail_exit_s(&weighed) / rail_exit_s(&own_flight) - 1.0;
            eprintln!(
                "{}: 5% more thrust {:+.3}% apogee, 20% more mass {:+.3}% rail exit time",
                public.file,
                100.0 * apogee_moved,
                100.0 * exit_moved
            );
            assert!(apogee_moved > 20.0 * OWN_HISTORY_BOUND, "{}", public.file);
            assert!(exit_moved > 0.05, "{}", public.file);
        }
    }

    fn inputs(holding_all: usize, lacking: [usize; 6]) -> Inputs {
        Inputs {
            flights: holding_all + lacking.iter().sum::<usize>(),
            largest_apogee_move: 5.8e-7,
            holding_all,
            lacking: REASONS
                .iter()
                .zip(lacking)
                .map(|((name, _), flights)| Tally {
                    reason: (*name).to_owned(),
                    flights,
                })
                .collect(),
        }
    }

    #[test]
    fn the_inputs_counts_hold_to_each_other_and_to_openrockets_flights() {
        let counted = inputs(77, [1, 1, 0, 0, 1, 0]);
        assert_eq!(counted.check_consistent(79), Ok(()));
        assert!(
            counted
                .check_consistent(80)
                .unwrap_err()
                .contains("not the 80 OpenRocket flies")
        );
        let mut wrong = counted.clone();
        wrong.flights += 1;
        assert!(
            wrong
                .check_consistent(79)
                .unwrap_err()
                .contains("holding all")
        );
        let mut renamed = counted.clone();
        renamed.lacking.swap(2, 3);
        assert!(
            renamed
                .check_consistent(79)
                .unwrap_err()
                .contains("reasons")
        );
        let page = counted.to_markdown();
        for (_, words) in REASONS {
            assert!(page.contains(&format!("| {words} |")), "{words}");
        }
        assert!(page.contains("Of the 80 flights OpenRocket was given, 77 hold every input."));
        assert!(page.contains("by up to 5.8e-7 of themselves"));
        assert_eq!(significant(5.756_123e-7, 2), 5.8e-7);
        assert_eq!(significant(-0.012_49, 2), -0.012);
    }
}
