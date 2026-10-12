//! The simulator's guide (M0.8a1, ADR-247): its landing page, *HPR Sim*, and *A tour of HPR Sim*.
//!
//! Both pages' numbers are quotes from the committed reports, as the guides' are. The tour's
//! pictures are terminal figures `cargo xtask cli` draws from the page's own examples
//! (`cli/terminal.rs`); its end is a public flight picked by a rule ([`median_flight`]), never by
//! hand: the one whose apogee miss, without its sign, is the median of the real-flights report's.

use super::{Quote, REAL, Reports, above, fixed, meters, percent, set_number, signed_percent};

/// The landing page.
pub(super) const LANDING: &str = "docs/the-simulator.md";
/// The tour.
pub(super) const TOUR: &str = "docs/simulator-tour.md";

/// The pointer of the public flight whose unsigned apogee miss is the median of the report's: the
/// middle one of an odd count, the lower of the two middle ones of an even count. Reads every
/// flight's miss, so a rerun that moves the middle names another flight.
pub(super) fn median_flight(r: &Reports) -> Result<String, String> {
    let flights = public_flights(r)?;
    let mut misses = Vec::with_capacity(flights);
    for at in 0..flights {
        let miss = r.number(REAL, &format!("/flights/{at}/apogee_error_percent"))?;
        misses.push((miss.abs(), at));
    }
    misses.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
    let (_, at) = misses[(flights - 1) / 2];
    Ok(format!("/flights/{at}"))
}

/// How many public flights the report holds, counted from its flights, so a flight dropped from
/// them changes the count a page shows (a test holds the summary's count to it).
fn public_flights(r: &Reports) -> Result<usize, String> {
    match r.get(REAL, "/flights")?.as_array().map_or(0, Vec::len) {
        0 => Err(format!("{REAL} holds no flights")),
        flights => Ok(flights),
    }
}

/// A flight's miss, its logged and simulated apogees and times to apogee, as the report gives them.
struct Flight {
    miss: f64,
    logged: f64,
    simulated: f64,
    logged_s: f64,
    simulated_s: f64,
}

/// The median flight's numbers.
fn median(r: &Reports) -> Result<Flight, String> {
    let flight = median_flight(r)?;
    let number = |field: &str| r.number(REAL, &format!("{flight}/{field}"));
    Ok(Flight {
        miss: number("apogee_error_percent")?,
        logged: number("log_apogee_m")?,
        simulated: number("hpr_apogee_m")?,
        logged_s: number("log_time_to_apogee_s")?,
        simulated_s: number("hpr_time_to_apogee_s")?,
    })
}

/// Which way a miss read: "low" below the log, else "high".
fn read(miss: f64) -> &'static str {
    if miss < 0.0 { "low" } else { "high" }
}

/// The landing page's quotes.
pub(super) fn landing_quotes() -> Vec<Quote> {
    vec![
        Quote {
            template: "{} public flights of university rockets, where they missed by {} on average \
                       without signs and by {} with them, outside the project's own {} target",
            holes: |r| {
                if r.flag(REAL, "/summary/within_target")? {
                    return Err("the public flights now meet their target".to_owned());
                }
                Ok(vec![
                    public_flights(r)?.to_string(),
                    percent(
                        r.number(REAL, "/summary/mean_absolute_apogee_error_percent")?,
                        2,
                    ),
                    signed_percent(r.number(REAL, "/summary/mean_apogee_error_percent")?, 2),
                    percent(r.number(REAL, "/summary/target_percent")?, 0),
                ])
            },
        },
        Quote {
            template: "{} logged flights of fliers' own designs they missed by {} without signs and \
                       averaged {} the logged ones",
            holes: |r| {
                Ok(vec![
                    super::flights(r)?.to_string(),
                    percent(set_number(r, "hpr", "mean_absolute_percent")?, 2),
                    above(set_number(r, "hpr", "mean_percent")?),
                ])
            },
        },
        Quote {
            template: "OpenRocket, flying the same files, missed by {} without signs and averaged {}",
            holes: |r| {
                Ok(vec![
                    percent(set_number(r, "openrocket", "mean_absolute_percent")?, 2),
                    above(set_number(r, "openrocket", "mean_percent")?),
                ])
            },
        },
        copernicus(),
    ]
}

/// The ERA5 weather's credit, with the years the fixture report's page gives.
fn copernicus() -> Quote {
    Quote {
        template: "generated using Copernicus Climate Change Service information {}, and contains \
                   modified Copernicus Climate Change Service information {}",
        holes: |r| {
            Ok(vec![
                r.copernicus_year("Generated using")?,
                r.copernicus_year("Contains modified")?,
            ])
        },
    }
}

/// The tour's quotes.
pub(super) fn tour_quotes() -> Vec<Quote> {
    vec![
        Quote {
            template: "one of {} university rockets' logged flights, where HPR Sim's apogee missed \
                       by {} on average without signs",
            holes: |r| {
                Ok(vec![
                    public_flights(r)?.to_string(),
                    percent(
                        r.number(REAL, "/summary/mean_absolute_apogee_error_percent")?,
                        2,
                    ),
                ])
            },
        },
        Quote {
            template: "of the {} flights, the one whose miss, without its sign, is the median is {}, \
                       the report's case {}",
            holes: |r| {
                let flight = median_flight(r)?;
                Ok(vec![
                    public_flights(r)?.to_string(),
                    r.title(&flight)?.to_owned(),
                    r.text(REAL, &format!("{flight}/id"))?.to_owned(),
                ])
            },
        },
        Quote {
            template: "{}'s altimeter read {} at the top, {} s after the rocket passed {} on the way \
                       up",
            holes: |r| {
                let flight = median_flight(r)?;
                let name = r.title(&flight)?;
                let name = name.split(',').next().unwrap_or(name).to_owned();
                let it = median(r)?;
                // The report times the climb from the height it aligns the traces at, not liftoff.
                let from = r.number(REAL, "/align_height_m")?;
                Ok(vec![
                    name,
                    meters(it.logged, 1),
                    fixed(it.logged_s, 1),
                    meters(from, 0),
                ])
            },
        },
        Quote {
            template: "would have read {}, at {} s: a miss of {}, so HPR Sim read {}",
            holes: |r| {
                let it = median(r)?;
                Ok(vec![
                    meters(it.simulated, 1),
                    fixed(it.simulated_s, 1),
                    signed_percent(it.miss, 2),
                    read(it.miss).to_owned(),
                ])
            },
        },
        Quote {
            template: "For a waiver or a field's ceiling, {}",
            holes: |r| {
                // A miss is simulated less logged, over logged, so a negative one read low: the
                // side that flatters a ceiling (this module's
                // `the_median_flights_miss_follows_from_its_readings_as_written` pins the sign).
                Ok(vec![
                    if median(r)?.miss < 0.0 {
                        "a low reading is the side that flatters: the rocket climbs past the \
                         height that was planned for"
                    } else {
                        "a high reading mainly costs height: the rocket tops out below the height \
                         that was planned for"
                    }
                    .to_owned(),
                ])
            },
        },
        Quote {
            template: "The report puts the miss {}",
            holes: |r| {
                let flight = median_flight(r)?;
                let kind = r.text(REAL, &format!("{flight}/explanation_kind"))?;
                if kind != "drag" {
                    return Err(format!(
                        "the report explains the median flight's miss by `{kind}`, not the drag: \
                         reword the page and this quote"
                    ));
                }
                let example =
                    r.number(REAL, &format!("{flight}/example_drag_apogee_error_percent"))?;
                Ok(vec![format!(
                    "down to HPR Sim's drag: flown on the drag the team's example gives, the \
                     apogee missed by {}",
                    signed_percent(example, 2)
                )])
            },
        },
        Quote {
            template: "On the {} flights HPR Sim's apogee missed by {} to {} one by one, by {} on \
                       average without signs, and by {} with them",
            holes: |r| {
                let flights = public_flights(r)?;
                let mut misses = Vec::new();
                for at in 0..flights {
                    misses.push(r.number(REAL, &format!("/flights/{at}/apogee_error_percent"))?);
                }
                let least = misses.iter().copied().fold(f64::INFINITY, f64::min);
                let most = misses.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                Ok(vec![
                    flights.to_string(),
                    signed_percent(least, 2),
                    signed_percent(most, 2),
                    percent(
                        r.number(REAL, "/summary/mean_absolute_apogee_error_percent")?,
                        2,
                    ),
                    signed_percent(r.number(REAL, "/summary/mean_apogee_error_percent")?, 2),
                ])
            },
        },
        Quote {
            template: "That average is outside the project's own {} target",
            holes: |r| {
                if r.flag(REAL, "/summary/within_target")? {
                    return Err("the public flights now meet their target".to_owned());
                }
                Ok(vec![percent(r.number(REAL, "/summary/target_percent")?, 0)])
            },
        },
        Quote {
            template: "So on these flights the misses go both ways, while on {} logged flights of \
                       fliers' own designs they lean {}, averaging {} the logged ones",
            holes: |r| {
                // "Both ways" holds while the public flights hold a miss on each side.
                let flights = public_flights(r)?;
                let mut sides = (false, false);
                for at in 0..flights {
                    let miss = r.number(REAL, &format!("/flights/{at}/apogee_error_percent"))?;
                    sides = (sides.0 || miss < 0.0, sides.1 || miss > 0.0);
                }
                if sides != (true, true) {
                    return Err("the public flights' misses no longer go both ways".to_owned());
                }
                let mean = set_number(r, "hpr", "mean_percent")?;
                Ok(vec![
                    super::flights(r)?.to_string(),
                    super::lean(mean).to_owned(),
                    above(mean),
                ])
            },
        },
        copernicus(),
    ]
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use serde_json::json;

    use super::super::{Reports, written};
    use super::*;

    /// Reports holding public flights with these misses, ids `f0`, `f1`, ….
    fn with_misses(misses: &[f64]) -> Reports {
        let flights: Vec<_> = misses
            .iter()
            .enumerate()
            .map(|(at, miss)| json!({ "id": format!("f{at}"), "apogee_error_percent": miss }))
            .collect();
        let mut files = std::collections::BTreeMap::new();
        files.insert(
            REAL,
            json!({ "flights": flights, "summary": { "flights": misses.len() } }),
        );
        Reports {
            files,
            read: RefCell::new(Vec::new()),
        }
    }

    fn picked(misses: &[f64]) -> String {
        let r = with_misses(misses);
        let flight = median_flight(&r).unwrap();
        r.text(REAL, &format!("{flight}/id")).unwrap().to_owned()
    }

    /// The rule picks the middle flight by its miss without its sign, whichever side it read,
    /// the lower of the two middle ones of an even count, wherever it sits in the report.
    #[test]
    fn the_tour_ends_on_the_median_miss_by_rule() {
        assert_eq!(picked(&[-9.0, 1.0, 4.0]), "f2");
        assert_eq!(picked(&[4.0, -9.0, -1.0]), "f0");
        // Without its sign: −3 is the middle of 1, 3 and 5.
        assert_eq!(picked(&[5.0, -3.0, 1.0]), "f1");
        // An even count: the lower of the two middle ones (2 of 1, 2, 3, 5).
        assert_eq!(picked(&[-1.0, 5.0, -3.0, 2.0]), "f3");
        assert_eq!(picked(&[7.0]), "f0");
        // No flights fails closed.
        assert!(median_flight(&with_misses(&[])).is_err());
    }

    /// On the committed report the rule picks Genesis today; and a reader's subtraction of the
    /// two readings as the page writes them gives the report's miss at the page's precision.
    #[test]
    fn the_median_flights_miss_follows_from_its_readings_as_written() {
        let root = crate::designs::root().unwrap();
        let r = Reports::load(&root).unwrap();
        // The pages count the flights; the summary they quote is over the same flights.
        assert_eq!(
            public_flights(&r).unwrap(),
            r.count(REAL, "/summary/flights").unwrap()
        );
        let it = median(&r).unwrap();
        let (logged, simulated) = (written(it.logged, 1), written(it.simulated, 1));
        let miss = (simulated - logged) / logged * 100.0;
        assert_eq!(signed_percent(miss, 2), signed_percent(it.miss, 2));
        // The quotes say "read low" from the sign; the sign is the readings'.
        assert_eq!(read(it.miss) == "low", simulated < logged);
    }
}
