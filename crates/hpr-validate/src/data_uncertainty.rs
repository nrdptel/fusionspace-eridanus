//! The data part of a real flight's validation uncertainty: the standard uncertainty `u_D` of
//! its log's apogee ([M1.14b17][m1-14b17]; [the decision that splits the validation
//! uncertainty][adr-373] and [the one that fixes this part's rules][adr-375]).
//!
//! **What is uncertain.** The report compares two readings: what the log's barometric altimeter
//! read at its apogee, and what the same kind of altimeter would read at HPR Sim's. A reading is
//! the 1976 standard atmosphere's altitude of the pressure at the rocket less that of the
//! pressure on the pad. `u_D` is the uncertainty of the log's reading as that quantity: the
//! pressure sensor's error, its noise, the step the log keeps, the departure of the altimeter's
//! own height formula from the 1976 standard's, and whatever the log's filter or the report's
//! cut does. How far the standard atmosphere is from the day's air is not in it: that conversion
//! sits in both readings ([`crate::validation_uncertainty::UncertaintyRow::conversion_m`]).
//!
//! **The rules**, from the *Guide to the expression of uncertainty in measurement* (JCGM
//! 100:2008):
//!
//! - A quantity known only to lie between two bounds is taken as equally likely anywhere
//!   between them (4.3.7, eq. 7: `u² = a²/3` for bounds `±a`). A maker's limits, and a maker's
//!   accuracy stated with no confidence level (4.3.7, example 2), are read this way.
//! - An effect of known sign that is not corrected keeps its mean beside its spread
//!   (F.2.4.5): for a value equally likely anywhere from `lo` to `hi`, the root mean square
//!   about zero, `√((lo² + lo·hi + hi²)/3)` ([`range_rms`]). For `±a` it is the first rule's
//!   `a/√3`; for one value `b` it is `|b|`.
//! - A digital indication with step `δ` has `u = δ/√12` (F.2.2.1).
//! - A maker's "typical" figure has no rule in the guide (F.2.3.1 leaves it to judgment). It
//!   is taken as one standard deviation, the larger of the two readings a "typical" bound
//!   allows; as bounds it would be `√3` times smaller.
//!
//! **Pressure to height.** A pressure error `δp` at the apogee moves the reading by
//! `δp/(ρ·g₀)`: the standard atmosphere's hydrostatic equation `dp = −ρ·g₀·dH` (U.S. Standard
//! Atmosphere, 1976, NOAA-S/T 76-1562, eq. 1 with the geopotential altitude of eq. 18), at
//! the standard's density `ρ` where the altimeter reads, the site's elevation plus the log's
//! apogee ([`meters_per_pascal`]). The pad's pressure altitude on the day stands a little off
//! its elevation; 100 m either way moves the figure by about 1%.
//!
//! The components add as a root sum of squares ([`data_part`]), each taken as independent of
//! the others.
//!
//! [m1-14b17]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m1-14b17
//! [adr-373]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0373-m1-14b7-in-three-the-numerical-part-first.md
//! [adr-375]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0375-m1-14b17-the-logs-part-of-a-real-flights-validation-uncertainty.md

use hpr_atmos::{AtmosError, Ussa76};
use hpr_core::gravity::STANDARD_GRAVITY_MPS2;
use serde::{Deserialize, Serialize};

/// One stated source of error in a log's apogee, as its source gives it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Term {
    /// A pressure error within `±bound_pa`, Pa: `bound/√3`, as a height.
    PressureBounds {
        /// The component's name in the report.
        what: &'static str,
        /// The bound, Pa.
        bound_pa: f64,
        /// Where the bound is stated.
        from: &'static str,
    },
    /// A pressure's standard deviation, Pa, as a height.
    PressureDeviation {
        /// The component's name in the report.
        what: &'static str,
        /// The standard deviation, Pa.
        sigma_pa: f64,
        /// Where it is stated.
        from: &'static str,
    },
    /// A pressure kept in steps of `step_pa`, Pa: `step/√12`, as a height.
    PressureStep {
        /// The component's name in the report.
        what: &'static str,
        /// The step, Pa.
        step_pa: f64,
        /// Where it is stated.
        from: &'static str,
    },
    /// A height error equally likely anywhere from `low_m` to `high_m`, m, not corrected:
    /// [`range_rms`].
    HeightRange {
        /// The component's name in the report.
        what: &'static str,
        /// The range's lower end, m.
        low_m: f64,
        /// The range's upper end, m.
        high_m: f64,
        /// Where it is stated.
        from: &'static str,
    },
    /// A height kept in steps of `step_m`, m: `step/√12`.
    HeightStep {
        /// The component's name in the report.
        what: &'static str,
        /// The step, m.
        step_m: f64,
        /// Where it is stated.
        from: &'static str,
    },
    /// A maker's typical figure, a share of the reading plus a constant, m, taken as one
    /// standard deviation.
    TypicalOfReading {
        /// The component's name in the report.
        what: &'static str,
        /// The share of the reading.
        share: f64,
        /// The constant, m.
        plus_m: f64,
        /// Where it is stated.
        from: &'static str,
    },
    /// The largest gap between the log's own height column and the 1976 standard's reading of
    /// its own pressure column, m, which the report measures on the log
    /// (`FlightRow::pressure_reading_max_m`): the altimeter's formula against the standard's,
    /// at its full size.
    FormulaGap {
        /// The component's name in the report.
        what: &'static str,
        /// What the gap is.
        from: &'static str,
    },
}

impl Term {
    /// The component's name and where it is stated.
    #[must_use]
    pub fn named(&self) -> (&'static str, &'static str) {
        match *self {
            Self::PressureBounds { what, from, .. }
            | Self::PressureDeviation { what, from, .. }
            | Self::PressureStep { what, from, .. }
            | Self::HeightRange { what, from, .. }
            | Self::HeightStep { what, from, .. }
            | Self::TypicalOfReading { what, from, .. }
            | Self::FormulaGap { what, from } => (what, from),
        }
    }

    /// Whether the term is a pressure's, and so needs [`meters_per_pascal`].
    #[must_use]
    pub fn is_pressure(&self) -> bool {
        matches!(
            self,
            Self::PressureBounds { .. }
                | Self::PressureDeviation { .. }
                | Self::PressureStep { .. }
        )
    }

    /// The component's standard uncertainty, m, for a log whose apogee reads `log_apogee_m`,
    /// where a pascal is `meters_per_pascal` of height and the log's own formula gap is
    /// `formula_gap_m`. `None` for a pressure term with no conversion, or a formula gap with no
    /// gap measured.
    #[must_use]
    pub fn standard_m(
        &self,
        log_apogee_m: f64,
        meters_per_pascal: Option<f64>,
        formula_gap_m: Option<f64>,
    ) -> Option<f64> {
        match *self {
            Self::PressureBounds { bound_pa, .. } => {
                Some(range_rms(-bound_pa, bound_pa) * meters_per_pascal?)
            }
            Self::PressureDeviation { sigma_pa, .. } => Some(sigma_pa * meters_per_pascal?),
            Self::PressureStep { step_pa, .. } => {
                Some(step_uncertainty(step_pa) * meters_per_pascal?)
            }
            Self::HeightRange { low_m, high_m, .. } => Some(range_rms(low_m, high_m)),
            Self::HeightStep { step_m, .. } => Some(step_uncertainty(step_m)),
            Self::TypicalOfReading { share, plus_m, .. } => {
                Some(share * log_apogee_m.abs() + plus_m)
            }
            Self::FormulaGap { .. } => formula_gap_m.map(f64::abs),
        }
    }
}

/// The root mean square about zero of a value equally likely anywhere from `low` to `high`:
/// `√((low² + low·high + high²)/3)`, the mean squared plus the variance `(high − low)²/12` (JCGM
/// 100:2008, 4.3.7 for the variance, F.2.4.5 for an effect left uncorrected). For `±a` it is
/// `a/√3`, and for `low = high = b` it is `|b|`.
#[must_use]
pub fn range_rms(low: f64, high: f64) -> f64 {
    ((low * low + low * high + high * high) / 3.0).sqrt()
}

/// The standard uncertainty of a value kept in steps of `step`: `step/√12` (JCGM 100:2008,
/// F.2.2.1).
#[must_use]
pub fn step_uncertainty(step: f64) -> f64 {
    step.abs() / 12.0_f64.sqrt()
}

/// How far one pascal of pressure error moves a barometric altimeter's reading, m/Pa, at a log's
/// apogee: `1/(ρ·g₀)`, with `ρ` the 1976 standard atmosphere's density at the site's elevation
/// plus the log's apogee and `g₀` standard gravity (the standard's hydrostatic equation,
/// `dp = −ρ·g₀·dH`).
///
/// # Errors
///
/// [`AtmosError`] if the standard atmosphere has no air there.
pub fn meters_per_pascal(site_elevation_m: f64, log_apogee_m: f64) -> Result<f64, AtmosError> {
    let air = Ussa76::standard()
        .sample(site_elevation_m + log_apogee_m)?
        .air;
    Ok(1.0 / (air.density_kg_m3 * STANDARD_GRAVITY_MPS2))
}

/// What is known of the altimeter that wrote a public flight's log.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Instrument {
    /// The flight's id.
    pub id: &'static str,
    /// The altimeter, and how it is known.
    pub altimeter: &'static str,
    /// Each stated source of error. Empty when no document read gives the altimeter an accuracy.
    pub terms: &'static [Term],
    /// Why there are no terms, for an altimeter with no published accuracy; empty otherwise.
    pub without: &'static str,
}

/// The MS5607's error after a zero at one pressure, as bounds.
const MS5607_SENSOR: Term = Term::PressureBounds {
    what: "sensor",
    bound_pa: 100.0,
    from: "TE Connectivity, MS5607-02BA03 datasheet (06/2017), p. 4: total error band with \
           autozero at one pressure point, 0 to 50 °C, 300 to 1100 mbar, −1.0 to +1.0 mbar \
           (its minimum and maximum columns)",
};

/// The MS5607's noise at its lowest oversampling, for an altimeter whose setting isn't published.
const MS5607_NOISE_WIDEST: Term = Term::PressureDeviation {
    what: "noise",
    sigma_pa: 13.0,
    from: "the same sheet, p. 4: resolution RMS 0.130 mbar at an oversampling ratio of 256, the \
           largest it lists; the altimeter's ratio isn't published",
};

/// The MS5607's output step.
const MS5607_STEP: Term = Term::PressureStep {
    what: "step",
    step_pa: 1.0,
    from: "the same sheet, p. 8: the compensated pressure is given in steps of 0.01 mbar",
};

/// A CATS Vega's formula against the 1976 standard's, as the research note measures it.
const fn vega_formula(low_by_m: f64) -> Term {
    Term::HeightRange {
        what: "formula",
        low_m: low_by_m,
        high_m: low_by_m,
        from: "docs/research/cats-vega-height.md, \"Against HPR Sim's reading\": near the peak the \
               Vega's formula reads 0.19 m (Genesis) and 0.23 m (Lince) below the 1976 standard's \
               pressure altitude less the pad's",
    }
}

/// The highest value of a Vega's 9-sample median against the coast's own curve.
const VEGA_PEAK: Term = Term::HeightRange {
    what: "peak",
    low_m: 0.9,
    high_m: 3.2,
    from: "docs/research/cats-vega-filtered-column.md, \"Noise lifts the maxima\": on four Vega \
           logs the 9-sample median's highest value sits 0.9 m to 3.2 m above a parabola \
           through the coast, as the highest of a noisy series does",
};

const NO_MAKER: &str = "RocketPy's notebook calls it a COTS altimeter and names no make or sensor";

/// Each public real flight's altimeter, in the report's order.
pub const INSTRUMENTS: [Instrument; 12] = [
    Instrument {
        id: "bella-lui",
        altimeter: "the team's own avionics",
        terms: &[],
        without: "no document read names the sensor that made the logged height, or how it was \
                  filtered",
    },
    Instrument {
        id: "ndrt-2020",
        altimeter: "Featherweight Raven",
        terms: &[
            Term::PressureBounds {
                what: "sensor",
                bound_pa: 100.0,
                from: "Featherweight Altimeters, Raven4 user's manual (2020-08-15), p. 12: \"less \
                       than 1 mbar (0.1% full-scale) pressure error over most conditions\", with \
                       no confidence level; the log doesn't say which Raven wrote it",
            },
            Term::PressureStep {
                what: "step",
                step_pa: 4.053,
                from: "the same manual, p. 12: baro resolution 0.00004 atm",
            },
            Term::HeightRange {
                what: "formula",
                low_m: 0.067_056,
                high_m: 0.067_056,
                from: "docs/research/altimeter-height-formulas.md: Raven height columns are the \
                       standard atmosphere's altitude of their own logged pressure to 0.22 ft",
            },
        ],
        without: "",
    },
    Instrument {
        id: "prometheus-2022",
        altimeter: "Altus Metrum TeleMetrum (MS5607: the log steps its pressure by 1 Pa)",
        terms: &[
            MS5607_SENSOR,
            MS5607_NOISE_WIDEST,
            MS5607_STEP,
            Term::FormulaGap {
                what: "formula",
                from: "this report: the log's height column against the 1976 standard's reading \
                       of its own pressure column, the largest gap",
            },
        ],
        without: "",
    },
    Instrument {
        id: "juno-iii",
        altimeter: "Missile Works RRC3 (MS5607)",
        terms: &[
            MS5607_SENSOR,
            Term::PressureDeviation {
                what: "noise",
                sigma_pa: 3.6,
                from: "the same sheet, p. 4: resolution RMS 0.036 mbar at an oversampling ratio \
                       of 2048, the RRC3's (Missile Works, RRC3 user manual v1.71, p. 24)",
            },
            Term::PressureStep {
                what: "step",
                step_pa: 10.0,
                from: "docs/research/altimeter-sensor-noise.md, §3: the public RRC3 log keeps its \
                       pressure in steps of 10 Pa",
            },
            Term::FormulaGap {
                what: "formula",
                from: "this report: the log's height column against the 1976 standard's reading \
                       of its own pressure column, the largest gap",
            },
            Term::HeightRange {
                what: "cut",
                low_m: 10.0,
                high_m: 20.0,
                from: "this report's note on the flight: the log is read to 24.60 s, where its own \
                       velocity column still reads 17.5 m/s up, so its apogee may be 10 m to 20 m \
                       low",
            },
        ],
        without: "",
    },
    Instrument {
        id: "cavour",
        altimeter: "CATS Vega (MS5607), its radio telemetry",
        terms: &[
            MS5607_SENSOR,
            Term::HeightRange {
                what: "step",
                low_m: 0.0,
                high_m: 1.0,
                from: "the telemetry sends the height in whole meters cut toward zero \
                       (docs/research/cats-vega-height.md)",
            },
            Term::HeightRange {
                what: "filter",
                low_m: -1.3,
                high_m: 1.3,
                from: "docs/research/cats-vega-filtered-column.md, \"For Cavour\": the telemetry \
                       is the Kalman filter's height, within about 1.3 m of the flight's own \
                       barometric apogee",
            },
            vega_formula(0.23),
        ],
        without: "",
    },
    Instrument {
        id: "genesis",
        altimeter: "CATS Vega (MS5607), the 9-sample median of its barometric height",
        terms: &[MS5607_SENSOR, VEGA_PEAK, vega_formula(0.19)],
        without: "",
    },
    Instrument {
        id: "lince",
        altimeter: "CATS Vega (MS5607), the 9-sample median of its barometric height",
        terms: &[MS5607_SENSOR, VEGA_PEAK, vega_formula(0.23)],
        without: "",
    },
    Instrument {
        id: "astra",
        altimeter: "not named",
        terms: &[],
        without: NO_MAKER,
    },
    Instrument {
        id: "andromeda",
        altimeter: "not named",
        terms: &[],
        without: NO_MAKER,
    },
    Instrument {
        id: "erebus-11",
        altimeter: "not named",
        terms: &[],
        without: NO_MAKER,
    },
    Instrument {
        id: "macie-xl-1",
        altimeter: "PerfectFlite StratoLoggerCF",
        terms: &PERFECTFLITE,
        without: "",
    },
    Instrument {
        id: "macie-xl-2",
        altimeter: "PerfectFlite StratoLoggerCF",
        terms: &PERFECTFLITE,
        without: "",
    },
];

/// A PerfectFlite StratoLoggerCF's stated figures, and its header's step.
const PERFECTFLITE: [Term; 3] = [
    Term::TypicalOfReading {
        what: "precision",
        share: 0.001,
        plus_m: 0.3048,
        from: "PerfectFlite, StratoLoggerCF user's manual (PDF of 2014-12-02), specifications, \
               PDF p. 49: \"Measurement precision: +/- (0.1% reading + 1 foot) typical\"",
    },
    Term::TypicalOfReading {
        what: "calibration",
        share: 0.0005,
        plus_m: 0.0,
        from: "the same page: \"Calibration accuracy: +/- 0.05% typical\"",
    },
    Term::HeightStep {
        what: "step",
        step_m: 0.3048,
        from: "the log's header gives its apogee in whole feet",
    },
];

/// The instrument of the flight `id`.
#[must_use]
pub fn instrument(id: &str) -> Option<&'static Instrument> {
    INSTRUMENTS.iter().find(|each| each.id == id)
}

/// One component of a log apogee's standard uncertainty.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataComponent {
    /// The component's name: sensor, noise, step, formula, and so on.
    pub what: String,
    /// Its standard uncertainty, m.
    pub standard_m: f64,
    /// Where its figure is stated.
    pub from: String,
}

/// The data part of one public real flight: its log apogee's standard uncertainty and what it is
/// made of, or why it has none.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataPart {
    /// The altimeter, and how it is known.
    pub altimeter: String,
    /// How far a pascal moves the reading at the log's apogee, m/Pa ([`meters_per_pascal`]), for
    /// a part with a pressure term.
    pub meters_per_pascal: Option<f64>,
    /// Each component, in its instrument's order.
    pub components: Vec<DataComponent>,
    /// `u_D`, m: the root sum of squares of the components. `None` for an altimeter with no
    /// published accuracy.
    pub data_m: Option<f64>,
    /// Why there is no `u_D`; empty when there is one.
    pub without: String,
}

/// The root sum of squares of components' standard uncertainties, m.
#[must_use]
pub fn root_sum_of_squares(components: &[DataComponent]) -> f64 {
    components
        .iter()
        .map(|each| each.standard_m * each.standard_m)
        .sum::<f64>()
        .sqrt()
}

/// `instrument`'s data part for a log whose apogee reads `log_apogee_m`, where a pascal is
/// `meters_per_pascal` of height ([`meters_per_pascal`]) and the log's own formula gap is
/// `formula_gap_m`.
///
/// # Errors
///
/// A sentence, if a pressure term has no conversion, a formula gap has no gap measured, or a
/// figure isn't finite.
pub fn data_part(
    instrument: &Instrument,
    log_apogee_m: f64,
    meters_per_pascal: Option<f64>,
    formula_gap_m: Option<f64>,
) -> Result<DataPart, String> {
    let components = instrument
        .terms
        .iter()
        .map(|term| {
            let (what, from) = term.named();
            let standard_m = term
                .standard_m(log_apogee_m, meters_per_pascal, formula_gap_m)
                .filter(|value| value.is_finite())
                .ok_or_else(|| {
                    format!(
                        "{}: the {what} component has nothing to be computed from",
                        instrument.id
                    )
                })?;
            Ok(DataComponent {
                what: what.to_owned(),
                standard_m,
                from: from.to_owned(),
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let uses_pressure = instrument.terms.iter().any(Term::is_pressure);
    Ok(DataPart {
        altimeter: instrument.altimeter.to_owned(),
        meters_per_pascal: meters_per_pascal.filter(|_| uses_pressure),
        data_m: (!components.is_empty()).then(|| root_sum_of_squares(&components)),
        components,
        without: instrument.without.to_owned(),
    })
}

impl DataPart {
    /// Whether the part is `id`'s own: its instrument's terms, each computed from the part's own
    /// conversion and the log's apogee, and their root sum of squares. The formula gap is taken
    /// from the part itself, and the report's check holds it to the flight's row.
    #[must_use]
    pub fn holds(&self, id: &str, log_apogee_m: f64) -> bool {
        let Some(instrument) = instrument(id) else {
            return false;
        };
        let gap = self.formula_gap_m(id);
        data_part(instrument, log_apogee_m, self.meters_per_pascal, gap).as_ref() == Ok(self)
    }

    /// The formula gap the part was computed from, m, for a flight `id` whose instrument has
    /// one ([`Term::FormulaGap`]): the component's own figure.
    #[must_use]
    pub fn formula_gap_m(&self, id: &str) -> Option<f64> {
        instrument(id)?
            .terms
            .iter()
            .position(|term| matches!(term, Term::FormulaGap { .. }))
            .and_then(|at| self.components.get(at))
            .map(|component| component.standard_m)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The rules, each on a case small enough to do in the head.
    #[test]
    fn the_rules_on_cases_worked_by_hand() {
        // Bounds of ±3: 3/√3 = √3.
        assert!((range_rms(-3.0, 3.0) - 3.0_f64.sqrt()).abs() < 1e-15);
        // One value: itself.
        assert_eq!(range_rms(0.23, 0.23), 0.23);
        assert_eq!(range_rms(-2.0, -2.0), 2.0);
        // 0 to 1: mean 1/2, variance 1/12, so √(1/4 + 1/12) = 1/√3.
        assert!((range_rms(0.0, 1.0) - 1.0 / 3.0_f64.sqrt()).abs() < 1e-15);
        // 10 to 20: mean 15, variance 100/12, so √(225 + 8.3333) = 15.2753.
        assert!((range_rms(10.0, 20.0) - 15.275_252).abs() < 1e-6);
        // The same either way round, and never below the mean's size or the half-width's share.
        assert_eq!(range_rms(10.0, 20.0), range_rms(20.0, 10.0));
        assert!(range_rms(10.0, 20.0) > 15.0);
        // A step of 12 has √12; of 1 ft, 0.3048/3.4641 = 0.087988 m.
        assert!((step_uncertainty(12.0) - 12.0_f64.sqrt()).abs() < 1e-15);
        assert!((step_uncertainty(0.3048) - 0.087_988).abs() < 1e-6);
    }

    /// Macie XL's first flight, by hand. Its PerfectFlite's header reads 2765 ft, 842.772 m.
    /// Precision, typical: 0.1% of 842.772 m is 0.842772 m, plus 1 ft, 0.3048 m: 1.147572 m.
    /// Calibration, typical: 0.05% of 842.772 m is 0.421386 m. The header's step of 1 ft:
    /// 0.3048/√12 = 0.087988 m. Squares: 1.316922 + 0.177566 + 0.007742 = 1.502230, whose root
    /// is 1.225655 m.
    #[test]
    fn a_perfectflite_logs_data_part_worked_by_hand() {
        let macie = instrument("macie-xl-1").expect("the flight is in the table");
        let part = data_part(macie, 2765.0 * 0.3048, None, None).expect("no pressure term");
        let by_hand = [1.147_572, 0.421_386, 0.087_988];
        assert_eq!(part.components.len(), by_hand.len());
        for (component, expected) in part.components.iter().zip(by_hand) {
            assert!(
                (component.standard_m - expected).abs() < 1e-6,
                "{}: {}",
                component.what,
                component.standard_m
            );
        }
        let u_d = part.data_m.expect("a published accuracy");
        assert!((u_d - 1.225_655).abs() < 1e-6, "{u_d}");
        assert_eq!(part.meters_per_pascal, None);
        assert!(part.holds("macie-xl-1", 2765.0 * 0.3048));
        assert!(!part.holds("macie-xl-1", 2766.0 * 0.3048));
    }

    /// NDRT's 2020 flight, by hand, with the pressure's conversion. The site is at 206 m and the
    /// log's apogee reads 1320.357 m, so the altimeter reads at 1526.357 m: a geopotential
    /// altitude of 6356766 × 1526.357/(6356766 + 1526.357) = 1525.991 m. The 1976 standard's
    /// troposphere there: T = 288.15 − 0.0065 × 1525.991 = 278.2311 K; p = 101325 ×
    /// (278.2311/288.15)^5.255876 = 84286.67 Pa; ρ = p/(287.0531 × T) = 1.055337 kg/m³. One pascal
    /// is 1/(1.055337 × 9.80665) = 0.096625 m. The Raven's 1 mbar bounds: 100/√3 = 57.7350 Pa,
    /// 5.57863 m. Its step of 4.053 Pa: 4.053/√12 = 1.17000 Pa, 0.11305 m. Its formula: 0.22 ft,
    /// 0.067056 m. Squares: 31.12112 + 0.012781 + 0.004497 = 31.13840, whose root is 5.58018 m.
    #[test]
    fn a_ravens_data_part_worked_by_hand() {
        let per_pa = meters_per_pascal(206.0, 1320.357).expect("the troposphere");
        assert!((per_pa - 0.096_625).abs() < 1e-6, "{per_pa}");
        let ndrt = instrument("ndrt-2020").expect("the flight is in the table");
        let part = data_part(ndrt, 1320.357, Some(per_pa), None).expect("every term has its input");
        let by_hand = [
            ("sensor", 5.578_63),
            ("step", 0.113_05),
            ("formula", 0.067_056),
        ];
        assert_eq!(part.components.len(), by_hand.len());
        for (component, (what, expected)) in part.components.iter().zip(by_hand) {
            assert_eq!(component.what, what);
            assert!(
                (component.standard_m - expected).abs() < 1e-5,
                "{what}: {}",
                component.standard_m
            );
        }
        let u_d = part.data_m.expect("a published accuracy");
        assert!((u_d - 5.580_18).abs() < 1e-5, "{u_d}");
        assert!(part.holds("ndrt-2020", 1320.357));
        // A pressure term with no conversion is refused, not read as zero.
        assert!(data_part(ndrt, 1320.357, None, None).is_err());
        // A formula gap with no gap measured is refused too.
        let prometheus = instrument("prometheus-2022").expect("the flight is in the table");
        assert!(data_part(prometheus, 3895.83, Some(0.14), None).is_err());
        assert!(data_part(prometheus, 3895.83, Some(0.14), Some(0.195)).is_ok());
    }

    /// A flight whose altimeter has no published accuracy has no part and says why; every
    /// other flight has terms and no such sentence.
    #[test]
    fn an_altimeter_with_no_published_accuracy_has_no_part() {
        let without = ["bella-lui", "astra", "andromeda", "erebus-11"];
        for each in &INSTRUMENTS {
            assert_eq!(
                each.terms.is_empty(),
                without.contains(&each.id),
                "{}",
                each.id
            );
            assert_eq!(
                each.terms.is_empty(),
                !each.without.is_empty(),
                "{}",
                each.id
            );
            if each.terms.is_empty() {
                let part = data_part(each, 3000.0, None, None).expect("no term to compute");
                assert_eq!(part.data_m, None);
                assert!(part.components.is_empty());
                assert!(part.holds(each.id, 3000.0));
            }
        }
        assert_eq!(instrument("no-such-flight"), None);
    }

    /// Which way each rule errs, pinned: a larger bound, step, share or gap never gives a smaller
    /// component, and a component left out makes `u_D` smaller, never larger.
    #[test]
    fn a_larger_error_never_gives_a_smaller_part() {
        let per_pa = Some(0.1);
        let bounds = |bound_pa| Term::PressureBounds {
            what: "sensor",
            bound_pa,
            from: "",
        };
        assert!(
            bounds(200.0).standard_m(1000.0, per_pa, None)
                > bounds(100.0).standard_m(1000.0, per_pa, None)
        );
        let juno = instrument("juno-iii").expect("the flight is in the table");
        let whole = data_part(juno, 3151.507, per_pa, Some(1.321)).expect("every input");
        let u_d = whole.data_m.expect("a published accuracy");
        for skipped in 0..whole.components.len() {
            let mut fewer = whole.components.clone();
            fewer.remove(skipped);
            assert!(root_sum_of_squares(&fewer) < u_d);
        }
        // The cut is the largest component of Juno III's, and the sensor the next.
        let largest = whole
            .components
            .iter()
            .max_by(|a, b| a.standard_m.total_cmp(&b.standard_m))
            .expect("five components");
        assert_eq!(largest.what, "cut");
    }
}
