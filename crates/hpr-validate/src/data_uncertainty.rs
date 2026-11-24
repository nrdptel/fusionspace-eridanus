//! The data part of a real flight's validation uncertainty: the standard uncertainty `u_D` of
//! its log's apogee, and the correction its log's apogee takes
//! ([M1.14b17][m1-14b17]; [the decision that splits the validation uncertainty][adr-373] and
//! [the one that fixes this part's rules][adr-375]).
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
//! - A recognized effect of known sign is corrected, not left in the uncertainty (3.2.4; the
//!   note to 6.3.1 says enlarging the uncertainty instead should be avoided). For a log's error
//!   equally likely anywhere from `lo` to `hi`, the log's apogee is corrected by the mean,
//!   `−(lo + hi)/2` ([`range_correction_m`]), and only the spread about the mean stays in
//!   `u_D`: `(hi − lo)/√12` ([`range_uncertainty`]; F.2.4.5 corrects by the mean and keeps the
//!   variance about it, and 4.3.7 gives that variance). For `±a` the correction is zero and the
//!   spread the first rule's `a/√3`.
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
//! **The zero on the pad.** A reading is a difference of two standard altitudes, not of two
//! pressures. Write the sensor's error at the apogee as its error on the pad, `e₀`, plus how
//! far it has changed since, `d`. The reading's error is then
//! `−d/(ρₐ·g₀) − e₀·(1/ρₐ − 1/ρ₀)/g₀`, with `ρₐ` and `ρ₀` the standard's densities at the
//! apogee and on the pad. The zero removes `e₀` from the first term only: a pascal is more
//! height in the thinner air at the apogee, so an offset the two pressures share doesn't
//! cancel. The first term is [`Term::PressureBounds`], the second [`Term::PadOffsetBounds`].
//!
//! The components add as a root sum of squares ([`data_part`]), each taken as independent of
//! the others, and the corrections add.
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
    /// How far the sensor's pressure error may have changed between the zero on the pad and
    /// the apogee, within `±bound_pa`, Pa: `bound/√3`, as a height at the apogee.
    PressureBounds {
        /// The component's name in the report.
        what: &'static str,
        /// The bound, Pa.
        bound_pa: f64,
        /// Where the bound is stated.
        from: &'static str,
    },
    /// The sensor's pressure error on the pad, within `±bound_pa`, Pa, which a zero on the pad
    /// leaves in a difference of two standard altitudes: `bound/√3` times the difference
    /// between a pascal's height at the apogee and on the pad.
    PadOffsetBounds {
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
    /// The log's apogee less what it should read, equally likely anywhere from `low_m` to
    /// `high_m`, m (negative where the log reads low): the log's apogee is corrected by
    /// [`range_correction_m`] and the component is [`range_uncertainty`].
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
    /// The log's own height column less the 1976 standard's reading of its own pressure column,
    /// at the log's highest row, m, which the report measures on the log
    /// (`FlightRow::pressure_reading_apogee_m`): the altimeter's formula against the standard's.
    /// It is known, so the log's apogee is corrected by it and it adds nothing to `u_D`.
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
            | Self::PadOffsetBounds { what, from, .. }
            | Self::PressureDeviation { what, from, .. }
            | Self::PressureStep { what, from, .. }
            | Self::HeightRange { what, from, .. }
            | Self::HeightStep { what, from, .. }
            | Self::TypicalOfReading { what, from, .. }
            | Self::FormulaGap { what, from } => (what, from),
        }
    }

    /// Whether the term is a pressure's, and so needs [`Measured::meters_per_pascal`].
    #[must_use]
    pub fn is_pressure(&self) -> bool {
        matches!(
            self,
            Self::PressureBounds { .. }
                | Self::PadOffsetBounds { .. }
                | Self::PressureDeviation { .. }
                | Self::PressureStep { .. }
        )
    }

    /// Whether the term is the pad's offset, and so needs [`Measured::pad_meters_per_pascal`].
    #[must_use]
    pub fn is_pad_offset(&self) -> bool {
        matches!(self, Self::PadOffsetBounds { .. })
    }

    /// The component for a log whose apogee reads `log_apogee_m`: the correction the log's
    /// apogee takes, m, and the standard uncertainty that stays, m. `None` for a pressure term
    /// with no conversion, or a formula gap with no gap measured.
    #[must_use]
    pub fn component_m(&self, log_apogee_m: f64, measured: &Measured) -> Option<(f64, f64)> {
        match *self {
            Self::PressureBounds { bound_pa, .. } => Some((
                0.0,
                range_uncertainty(-bound_pa, bound_pa) * measured.meters_per_pascal?,
            )),
            Self::PadOffsetBounds { bound_pa, .. } => Some((
                0.0,
                range_uncertainty(-bound_pa, bound_pa)
                    * (measured.meters_per_pascal? - measured.pad_meters_per_pascal?).abs(),
            )),
            Self::PressureDeviation { sigma_pa, .. } => {
                Some((0.0, sigma_pa * measured.meters_per_pascal?))
            }
            Self::PressureStep { step_pa, .. } => {
                Some((0.0, step_uncertainty(step_pa) * measured.meters_per_pascal?))
            }
            Self::HeightRange { low_m, high_m, .. } => Some((
                range_correction_m(low_m, high_m),
                range_uncertainty(low_m, high_m),
            )),
            Self::HeightStep { step_m, .. } => Some((0.0, step_uncertainty(step_m))),
            Self::TypicalOfReading { share, plus_m, .. } => {
                Some((0.0, share * log_apogee_m.abs() + plus_m))
            }
            Self::FormulaGap { .. } => measured.formula_gap_m.map(|gap_m| (0.0 - gap_m, 0.0)),
        }
    }
}

/// The correction, m, for a log whose error (its value less what it should read) is equally
/// likely anywhere from `low_m` to `high_m`: the mean error taken off, `−(low + high)/2` (JCGM
/// 100:2008, 3.2.4 and F.2.4.5). Zero for `±a`, written as a subtraction from zero so that it is
/// never a negative zero.
#[must_use]
pub fn range_correction_m(low_m: f64, high_m: f64) -> f64 {
    0.0 - (low_m + high_m) / 2.0
}

/// The standard uncertainty of a value equally likely anywhere from `low` to `high`, about its
/// mean: `|high − low|/√12` (JCGM 100:2008, 4.3.7, eq. 7 with `a` the half-width). For `±a` it
/// is `a/√3`, and for `low = high` it is zero.
#[must_use]
pub fn range_uncertainty(low: f64, high: f64) -> f64 {
    (high - low).abs() / 12.0_f64.sqrt()
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
/// `dp = −ρ·g₀·dH`). With an apogee of zero it is the pad's.
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

/// What a flight's own site and log add to its instrument's terms.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Measured {
    /// How far a pascal moves the reading at the log's apogee, m/Pa ([`meters_per_pascal`]).
    pub meters_per_pascal: Option<f64>,
    /// How far a pascal moves a reading on the pad, m/Pa ([`meters_per_pascal`] with no apogee).
    pub pad_meters_per_pascal: Option<f64>,
    /// The log's height column less the 1976 standard's reading of its pressure column, at its
    /// highest row, m.
    pub formula_gap_m: Option<f64>,
}

impl Measured {
    /// The two conversions at a site `site_elevation_m` above sea level for a log whose apogee
    /// reads `log_apogee_m`, with the log's own formula gap.
    ///
    /// # Errors
    ///
    /// [`AtmosError`] if the standard atmosphere has no air there.
    pub fn at(
        site_elevation_m: f64,
        log_apogee_m: f64,
        formula_gap_m: Option<f64>,
    ) -> Result<Self, AtmosError> {
        Ok(Self {
            meters_per_pascal: Some(meters_per_pascal(site_elevation_m, log_apogee_m)?),
            pad_meters_per_pascal: Some(meters_per_pascal(site_elevation_m, 0.0)?),
            formula_gap_m,
        })
    }
}

/// What is known of the altimeter that wrote a public flight's log.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Instrument {
    /// The flight's id.
    pub id: &'static str,
    /// The altimeter, and how it is known.
    pub altimeter: &'static str,
    /// Each stated source of error. Empty when no document read gives the altimeter, or the
    /// pressure sensor its maker names, an accuracy.
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

/// The MS5607's error with no zero, as bounds on its offset on the pad.
const MS5607_PAD_OFFSET: Term = Term::PadOffsetBounds {
    what: "pad offset",
    bound_pa: 200.0,
    from: "the same sheet, p. 4: total error band with no autozero, 0 to 50 °C, 300 to 1100 \
           mbar, −2.0 to +2.0 mbar, for the sensor's error on the pad, which the zero leaves in \
           a difference of two standard altitudes",
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

/// A CATS Vega's formula against the 1976 standard's, as the research note measures it on the
/// flight's own log: the log reads `low_by_m` low.
const fn vega_formula(low_by_m: f64) -> Term {
    Term::HeightRange {
        what: "formula",
        low_m: -low_by_m,
        high_m: -low_by_m,
        from: "docs/research/cats-vega-height.md, \"Against HPR Sim's reading\": near the peak the \
               Vega's formula reads 0.19 m (Genesis) and 0.23 m (Lince) below the 1976 standard's \
               pressure altitude less the pad's",
    }
}

/// The highest value of a Vega's 9-sample median against the coast's own curve: the log reads
/// high.
const VEGA_PEAK: Term = Term::HeightRange {
    what: "peak",
    low_m: 0.9,
    high_m: 3.2,
    from: "docs/research/cats-vega-filtered-column.md, \"Noise lifts the maxima\": on four Vega \
           logs the 9-sample median's highest value sits 0.9 m to 3.2 m above a parabola \
           through the coast, as the highest of a noisy series does",
};

const NO_MAKER: &str = "RocketPy's notebook calls it a COTS altimeter and names no make or sensor";

/// A log's own formula gap, measured by the report.
const LOG_FORMULA_GAP: Term = Term::FormulaGap {
    what: "formula",
    from: "this report: the log's height column less the 1976 standard's reading of its own \
           pressure column, at the log's highest row",
};

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
            Term::PadOffsetBounds {
                what: "pad offset",
                bound_pa: 100.0,
                from: "the same sentence: the manual gives one figure and no part of it that a \
                       zero removes, so it is taken for the error on the pad as for its change \
                       from the pad to the apogee",
            },
            Term::PressureStep {
                what: "step",
                step_pa: 4.053,
                from: "the same manual, p. 12: baro resolution 0.00004 atm",
            },
            Term::HeightRange {
                what: "formula",
                low_m: -0.067_056,
                high_m: 0.067_056,
                from: "docs/research/altimeter-height-formulas.md: Raven height columns are the \
                       standard atmosphere's altitude of their own logged pressure to 0.22 ft, \
                       either way",
            },
        ],
        without: "",
    },
    Instrument {
        id: "prometheus-2022",
        altimeter: "Altus Metrum TeleMetrum (MS5607: the log steps its pressure by 1 Pa)",
        terms: &[
            MS5607_SENSOR,
            MS5607_PAD_OFFSET,
            MS5607_NOISE_WIDEST,
            MS5607_STEP,
            LOG_FORMULA_GAP,
        ],
        without: "",
    },
    Instrument {
        id: "juno-iii",
        altimeter: "Missile Works RRC3 (MS5607)",
        terms: &[
            MS5607_SENSOR,
            MS5607_PAD_OFFSET,
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
            LOG_FORMULA_GAP,
            Term::HeightRange {
                what: "cut",
                low_m: -20.0,
                high_m: -10.0,
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
            MS5607_PAD_OFFSET,
            Term::HeightRange {
                what: "step",
                low_m: -1.0,
                high_m: 0.0,
                from: "the telemetry sends the height in whole meters cut toward zero \
                       (docs/research/cats-vega-height.md), so it reads up to 1 m low",
            },
            Term::HeightRange {
                what: "filter",
                low_m: -1.3,
                high_m: 1.3,
                from: "docs/research/cats-vega-filtered-column.md, \"For Cavour\": the telemetry \
                       is the Kalman filter's height, within about 1.3 m of the flight's own \
                       barometric apogee",
            },
            Term::HeightRange {
                what: "formula",
                low_m: -0.23,
                high_m: -0.19,
                from: "docs/research/cats-vega-height.md, \"Against HPR Sim's reading\": the Vega's \
                       formula reads 0.19 m and 0.23 m low on the two logs it is measured on; \
                       Cavour's telemetry keeps no pressure to measure it on, so it is taken as \
                       anywhere between the two",
            },
        ],
        without: "",
    },
    Instrument {
        id: "genesis",
        altimeter: "CATS Vega (MS5607), the 9-sample median of its barometric height",
        terms: &[
            MS5607_SENSOR,
            MS5607_PAD_OFFSET,
            VEGA_PEAK,
            vega_formula(0.19),
        ],
        without: "",
    },
    Instrument {
        id: "lince",
        altimeter: "CATS Vega (MS5607), the 9-sample median of its barometric height",
        terms: &[
            MS5607_SENSOR,
            MS5607_PAD_OFFSET,
            VEGA_PEAK,
            vega_formula(0.23),
        ],
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

/// One component of a log apogee's data part.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataComponent {
    /// The component's name: sensor, noise, step, formula, and so on.
    pub what: String,
    /// The correction it gives the log's apogee, m: what to add to the log's apogee for an
    /// effect of known sign. Zero for an effect that goes either way.
    pub correction_m: f64,
    /// Its standard uncertainty, m.
    pub standard_m: f64,
    /// Where its figure is stated.
    pub from: String,
}

/// The data part of one public real flight: its log apogee's correction and standard
/// uncertainty and what they are made of, or why it has none.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataPart {
    /// The altimeter, and how it is known.
    pub altimeter: String,
    /// How far a pascal moves the reading at the log's apogee, m/Pa ([`meters_per_pascal`]), for
    /// a part with a pressure term.
    pub meters_per_pascal: Option<f64>,
    /// The same on the pad, m/Pa, for a part with a pad-offset term.
    pub pad_meters_per_pascal: Option<f64>,
    /// Each component, in its instrument's order.
    pub components: Vec<DataComponent>,
    /// What to add to the log's apogee, m: the sum of the components' corrections. `None` for
    /// an altimeter with no published accuracy.
    pub correction_m: Option<f64>,
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

/// `instrument`'s data part for a log whose apogee reads `log_apogee_m`, with what the flight's
/// site and log add ([`Measured`]).
///
/// # Errors
///
/// A sentence, if a pressure term has no conversion, a conversion isn't a positive number, a
/// formula gap has no gap measured, or a figure isn't finite or is a negative uncertainty.
pub fn data_part(
    instrument: &Instrument,
    log_apogee_m: f64,
    measured: &Measured,
) -> Result<DataPart, String> {
    let positive = |per_pascal: Option<f64>| per_pascal.is_none_or(|v| v.is_finite() && v > 0.0);
    if !positive(measured.meters_per_pascal) || !positive(measured.pad_meters_per_pascal) {
        return Err(format!(
            "{}: a pascal's height isn't a positive number",
            instrument.id
        ));
    }
    let components = instrument
        .terms
        .iter()
        .map(|term| {
            let (what, from) = term.named();
            let (correction_m, standard_m) = term
                .component_m(log_apogee_m, measured)
                .filter(|(correction_m, standard_m)| {
                    correction_m.is_finite() && standard_m.is_finite() && *standard_m >= 0.0
                })
                .ok_or_else(|| {
                    format!(
                        "{}: the {what} component has nothing to be computed from",
                        instrument.id
                    )
                })?;
            Ok(DataComponent {
                what: what.to_owned(),
                correction_m,
                standard_m,
                from: from.to_owned(),
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let uses_pressure = instrument.terms.iter().any(Term::is_pressure);
    let uses_pad = instrument.terms.iter().any(Term::is_pad_offset);
    let has_part = !components.is_empty();
    Ok(DataPart {
        altimeter: instrument.altimeter.to_owned(),
        meters_per_pascal: measured.meters_per_pascal.filter(|_| uses_pressure),
        pad_meters_per_pascal: measured.pad_meters_per_pascal.filter(|_| uses_pad),
        // Summed from zero, in the instrument's order, so that no sum is a negative zero.
        correction_m: has_part.then(|| {
            components
                .iter()
                .fold(0.0, |sum, each| sum + each.correction_m)
        }),
        data_m: has_part.then(|| root_sum_of_squares(&components)),
        components,
        without: instrument.without.to_owned(),
    })
}

impl DataPart {
    /// Whether the part is `id`'s own: its instrument's terms, each computed from the part's own
    /// conversions and formula gap and the log's apogee, with their sum and root sum of squares.
    /// The conversions and the gap are the part's own here; the report's check holds them to
    /// the flight's site and row.
    #[must_use]
    pub fn holds(&self, id: &str, log_apogee_m: f64) -> bool {
        let Some(instrument) = instrument(id) else {
            return false;
        };
        let measured = Measured {
            meters_per_pascal: self.meters_per_pascal,
            pad_meters_per_pascal: self.pad_meters_per_pascal,
            formula_gap_m: self.formula_gap_m(id),
        };
        data_part(instrument, log_apogee_m, &measured).as_ref() == Ok(self)
    }

    /// The formula gap the part was computed from, m, for a flight `id` whose instrument has
    /// one ([`Term::FormulaGap`]): the component's correction, with its sign turned back.
    #[must_use]
    pub fn formula_gap_m(&self, id: &str) -> Option<f64> {
        instrument(id)?
            .terms
            .iter()
            .position(|term| matches!(term, Term::FormulaGap { .. }))
            .and_then(|at| self.components.get(at))
            .map(|component| 0.0 - component.correction_m)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The rules, each on a case small enough to do in the head.
    #[test]
    fn the_rules_on_cases_worked_by_hand() {
        // Bounds of ±3: 3/√3 = √3, and nothing to correct.
        assert!((range_uncertainty(-3.0, 3.0) - 3.0_f64.sqrt()).abs() < 1e-15);
        assert_eq!(range_correction_m(-3.0, 3.0), 0.0);
        assert!(range_correction_m(-3.0, 3.0).is_sign_positive());
        // One value: all correction, no spread. A log 0.23 m low gets 0.23 m.
        assert_eq!(range_uncertainty(-0.23, -0.23), 0.0);
        assert_eq!(range_correction_m(-0.23, -0.23), 0.23);
        // A log 0 to 1 m low: half a meter back, and 1/√12 = 0.288675 m stays.
        assert_eq!(range_correction_m(-1.0, 0.0), 0.5);
        assert!((range_uncertainty(-1.0, 0.0) - 0.288_675).abs() < 1e-6);
        // A log 10 to 20 m low: 15 m back, and 10/√12 = 2.886751 m stays.
        assert_eq!(range_correction_m(-20.0, -10.0), 15.0);
        assert!((range_uncertainty(-20.0, -10.0) - 2.886_751).abs() < 1e-6);
        // A log 0.9 to 3.2 m high: 2.05 m off, and 2.3/√12 = 0.663953 m stays.
        assert!((range_correction_m(0.9, 3.2) + 2.05).abs() < 1e-15);
        assert!((range_uncertainty(0.9, 3.2) - 0.663_953).abs() < 1e-6);
        // The same either way round.
        assert_eq!(range_uncertainty(10.0, 20.0), range_uncertainty(20.0, 10.0));
        assert_eq!(
            range_correction_m(10.0, 20.0),
            range_correction_m(20.0, 10.0)
        );
        // A step of 12 has √12; of 1 ft, 0.3048/3.4641 = 0.087988 m.
        assert!((step_uncertainty(12.0) - 12.0_f64.sqrt()).abs() < 1e-15);
        assert!((step_uncertainty(0.3048) - 0.087_988).abs() < 1e-6);
    }

    /// Macie XL's first flight, by hand. Its PerfectFlite's header reads 2765 ft, 842.772 m.
    /// Precision, typical: 0.1% of 842.772 m is 0.842772 m, plus 1 ft, 0.3048 m: 1.147572 m.
    /// Calibration, typical: 0.05% of 842.772 m is 0.421386 m. The header's step of 1 ft:
    /// 0.3048/√12 = 0.087988 m. Squares: 1.316922 + 0.177566 + 0.007742 = 1.502230, whose root
    /// is 1.225655 m. Nothing has a known sign, so nothing is corrected.
    #[test]
    fn a_perfectflite_logs_data_part_worked_by_hand() {
        let macie = instrument("macie-xl-1").expect("the flight is in the table");
        let part =
            data_part(macie, 2765.0 * 0.3048, &Measured::default()).expect("no pressure term");
        let by_hand = [1.147_572, 0.421_386, 0.087_988];
        assert_eq!(part.components.len(), by_hand.len());
        for (component, expected) in part.components.iter().zip(by_hand) {
            assert!(
                (component.standard_m - expected).abs() < 1e-6,
                "{}: {}",
                component.what,
                component.standard_m
            );
            assert_eq!(component.correction_m, 0.0, "{}", component.what);
        }
        let u_d = part.data_m.expect("a published accuracy");
        assert!((u_d - 1.225_655).abs() < 1e-6, "{u_d}");
        assert_eq!(part.correction_m, Some(0.0));
        assert_eq!(part.meters_per_pascal, None);
        assert_eq!(part.pad_meters_per_pascal, None);
        assert!(part.holds("macie-xl-1", 2765.0 * 0.3048));
        assert!(!part.holds("macie-xl-1", 2766.0 * 0.3048));
    }

    /// NDRT's 2020 flight, by hand, with the pressure's conversion. The site is at 206 m and the
    /// log's apogee reads 1320.357 m, so the altimeter reads at 1526.357 m: a geopotential
    /// altitude of 6356766 × 1526.357/(6356766 + 1526.357) = 1525.991 m. The 1976 standard's
    /// troposphere there: T = 288.15 − 0.0065 × 1525.991 = 278.2311 K; p = 101325 ×
    /// (278.2311/288.15)^5.255876 = 84286.67 Pa; ρ = p/(287.0531 × T) = 1.055337 kg/m³. One pascal
    /// is 1/(1.055337 × 9.80665) = 0.096625 m. On the pad, at 206 m (205.993 m geopotential):
    /// T = 286.8110 K, p = 98874.72 Pa, ρ = 1.200956 kg/m³, and one pascal is 0.084909 m.
    ///
    /// The Raven's 1 mbar bounds: 100/√3 = 57.7350 Pa, 5.57863 m at the apogee. The same bounds
    /// on the pad's offset: 57.7350 × (0.096625 − 0.084909) = 0.67643 m. Its step of 4.053 Pa:
    /// 4.053/√12 = 1.17000 Pa, 0.11305 m. Its formula, 0.22 ft either way: 0.067056/√3 =
    /// 0.038715 m. Squares: 31.12112 + 0.45755 + 0.012781 + 0.001499 = 31.59296, whose root is
    /// 5.62076 m. Nothing has a known sign, so nothing is corrected.
    #[test]
    fn a_ravens_data_part_worked_by_hand() {
        let measured = Measured::at(206.0, 1320.357, None).expect("the troposphere");
        let per_pa = measured.meters_per_pascal.expect("at the apogee");
        let pad_per_pa = measured.pad_meters_per_pascal.expect("on the pad");
        assert!((per_pa - 0.096_625).abs() < 1e-6, "{per_pa}");
        assert!((pad_per_pa - 0.084_909).abs() < 1e-6, "{pad_per_pa}");
        let ndrt = instrument("ndrt-2020").expect("the flight is in the table");
        let part = data_part(ndrt, 1320.357, &measured).expect("every term has its input");
        let by_hand = [
            ("sensor", 5.578_63),
            ("pad offset", 0.676_43),
            ("step", 0.113_05),
            ("formula", 0.038_715),
        ];
        assert_eq!(part.components.len(), by_hand.len());
        for (component, (what, expected)) in part.components.iter().zip(by_hand) {
            assert_eq!(component.what, what);
            assert!(
                (component.standard_m - expected).abs() < 1e-5,
                "{what}: {}",
                component.standard_m
            );
            assert_eq!(component.correction_m, 0.0, "{what}");
        }
        let u_d = part.data_m.expect("a published accuracy");
        assert!((u_d - 5.620_76).abs() < 1e-5, "{u_d}");
        assert_eq!(part.correction_m, Some(0.0));
        assert!(part.holds("ndrt-2020", 1320.357));
        // A pressure term with no conversion is refused, not read as zero; so is a pad-offset
        // term with no conversion on the pad.
        assert!(data_part(ndrt, 1320.357, &Measured::default()).is_err());
        let no_pad = Measured {
            pad_meters_per_pascal: None,
            ..measured
        };
        assert!(data_part(ndrt, 1320.357, &no_pad).is_err());
        // A conversion that isn't a positive number is refused.
        for bad in [0.0, -0.1, f64::NAN, f64::INFINITY] {
            let wrong = Measured {
                meters_per_pascal: Some(bad),
                ..measured
            };
            assert!(data_part(ndrt, 1320.357, &wrong).is_err(), "{bad}");
        }
        // A formula gap with no gap measured is refused too.
        let prometheus = instrument("prometheus-2022").expect("the flight is in the table");
        let no_gap = Measured {
            meters_per_pascal: Some(0.14),
            pad_meters_per_pascal: Some(0.1),
            formula_gap_m: None,
        };
        assert!(data_part(prometheus, 3895.83, &no_gap).is_err());
        let with_gap = Measured {
            formula_gap_m: Some(0.195),
            ..no_gap
        };
        assert!(data_part(prometheus, 3895.83, &with_gap).is_ok());
    }

    /// Juno III's corrections, by hand, on round conversions. Its log's formula reads 1.3 m
    /// below the standard's (a gap of −1.3 m), so the log's apogee gets 1.3 m back and nothing
    /// is left uncertain. Its cut leaves the log 10 m to 20 m low: 15 m back, and 10/√12 =
    /// 2.886751 m stays. Together 16.3 m. With a pascal 0.1 m at the apogee and 0.09 m on the
    /// pad: the sensor 100/√3 × 0.1 = 5.773503 m, the pad's offset 200/√3 × 0.01 = 1.154701 m,
    /// the noise 3.6 × 0.1 = 0.36 m, the step 10/√12 × 0.1 = 0.288675 m. Squares: 33.333333 +
    /// 1.333333 + 0.1296 + 0.083333 + 0 + 8.333333 = 43.212933, whose root is 6.573654 m.
    #[test]
    fn a_known_error_is_corrected_and_only_its_spread_stays() {
        let juno = instrument("juno-iii").expect("the flight is in the table");
        let measured = Measured {
            meters_per_pascal: Some(0.1),
            pad_meters_per_pascal: Some(0.09),
            formula_gap_m: Some(-1.3),
        };
        let part = data_part(juno, 3151.507, &measured).expect("every input");
        let by_hand = [
            ("sensor", 0.0, 5.773_503),
            ("pad offset", 0.0, 1.154_701),
            ("noise", 0.0, 0.36),
            ("step", 0.0, 0.288_675),
            ("formula", 1.3, 0.0),
            ("cut", 15.0, 2.886_751),
        ];
        assert_eq!(part.components.len(), by_hand.len());
        for (component, (what, correction, standard)) in part.components.iter().zip(by_hand) {
            assert_eq!(component.what, what);
            assert!(
                (component.correction_m - correction).abs() < 1e-12,
                "{what}"
            );
            assert!((component.standard_m - standard).abs() < 1e-6, "{what}");
        }
        let correction = part.correction_m.expect("a published accuracy");
        assert!((correction - 16.3).abs() < 1e-12, "{correction}");
        let u_d = part.data_m.expect("a published accuracy");
        assert!((u_d - 6.573_654).abs() < 1e-6, "{u_d}");
        assert_eq!(part.formula_gap_m("juno-iii"), Some(-1.3));
        assert!(part.holds("juno-iii", 3151.507));
        // A log that reads high is corrected down: a Vega's peak, 2.05 m, less its formula's
        // 0.19 m the other way.
        let genesis = instrument("genesis").expect("the flight is in the table");
        let part = data_part(genesis, 2916.7, &measured).expect("every input");
        let correction = part.correction_m.expect("a published accuracy");
        assert!((correction + 1.86).abs() < 1e-12, "{correction}");
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
                let part =
                    data_part(each, 3000.0, &Measured::default()).expect("no term to compute");
                assert_eq!(part.data_m, None);
                assert_eq!(part.correction_m, None);
                assert!(part.components.is_empty());
                assert!(part.holds(each.id, 3000.0));
            }
        }
        assert_eq!(instrument("no-such-flight"), None);
    }

    /// Which way each rule errs, pinned on every kind of term: a larger bound, deviation, step,
    /// range, share or constant never gives a smaller component, a larger gap gives a larger
    /// correction and no uncertainty, and a component left out never makes `u_D` larger.
    #[test]
    fn a_larger_error_never_gives_a_smaller_part() {
        let measured = Measured {
            meters_per_pascal: Some(0.1),
            pad_meters_per_pascal: Some(0.09),
            formula_gap_m: Some(-1.3),
        };
        let standard = |term: Term| term.component_m(1000.0, &measured).expect("every input").1;
        let (what, from) = ("", "");
        let pairs = [
            (
                Term::PressureBounds {
                    what,
                    bound_pa: 100.0,
                    from,
                },
                Term::PressureBounds {
                    what,
                    bound_pa: 200.0,
                    from,
                },
            ),
            (
                Term::PadOffsetBounds {
                    what,
                    bound_pa: 100.0,
                    from,
                },
                Term::PadOffsetBounds {
                    what,
                    bound_pa: 200.0,
                    from,
                },
            ),
            (
                Term::PressureDeviation {
                    what,
                    sigma_pa: 3.6,
                    from,
                },
                Term::PressureDeviation {
                    what,
                    sigma_pa: 13.0,
                    from,
                },
            ),
            (
                Term::PressureStep {
                    what,
                    step_pa: 1.0,
                    from,
                },
                Term::PressureStep {
                    what,
                    step_pa: 10.0,
                    from,
                },
            ),
            (
                Term::HeightRange {
                    what,
                    low_m: -20.0,
                    high_m: -10.0,
                    from,
                },
                Term::HeightRange {
                    what,
                    low_m: -25.0,
                    high_m: -10.0,
                    from,
                },
            ),
            (
                Term::HeightStep {
                    what,
                    step_m: 0.3048,
                    from,
                },
                Term::HeightStep {
                    what,
                    step_m: 1.0,
                    from,
                },
            ),
            (
                Term::TypicalOfReading {
                    what,
                    share: 0.0005,
                    plus_m: 0.0,
                    from,
                },
                Term::TypicalOfReading {
                    what,
                    share: 0.001,
                    plus_m: 0.0,
                    from,
                },
            ),
            (
                Term::TypicalOfReading {
                    what,
                    share: 0.001,
                    plus_m: 0.0,
                    from,
                },
                Term::TypicalOfReading {
                    what,
                    share: 0.001,
                    plus_m: 0.3048,
                    from,
                },
            ),
        ];
        for (smaller, larger) in pairs {
            assert!(standard(larger) > standard(smaller), "{larger:?}");
        }
        // The pad's offset is nothing where a pascal is the same height at both ends.
        let level = Measured {
            pad_meters_per_pascal: Some(0.1),
            ..measured
        };
        let offset = Term::PadOffsetBounds {
            what,
            bound_pa: 200.0,
            from,
        };
        assert_eq!(offset.component_m(1000.0, &level), Some((0.0, 0.0)));
        // A formula gap is all correction.
        let gap = Term::FormulaGap { what, from };
        assert_eq!(gap.component_m(1000.0, &measured), Some((1.3, 0.0)));
        let juno = instrument("juno-iii").expect("the flight is in the table");
        let whole = data_part(juno, 3151.507, &measured).expect("every input");
        let u_d = whole.data_m.expect("a published accuracy");
        for skipped in 0..whole.components.len() {
            let mut fewer = whole.components.clone();
            let left_out = fewer.remove(skipped);
            let without = root_sum_of_squares(&fewer);
            if left_out.standard_m > 0.0 {
                assert!(without < u_d, "{}", left_out.what);
            } else {
                assert_eq!(without, u_d, "{}", left_out.what);
            }
        }
        // The sensor is the largest component of Juno III's, now that its cut is corrected.
        let largest = whole
            .components
            .iter()
            .max_by(|a, b| a.standard_m.total_cmp(&b.standard_m))
            .expect("six components");
        assert_eq!(largest.what, "sensor");
    }
}
