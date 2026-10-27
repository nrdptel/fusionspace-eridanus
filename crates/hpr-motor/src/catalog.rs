//! An offline motor catalog: motor metadata, thrust curves, and each curve's provenance and
//! license.
//!
//! [`Catalog::bundled`] is the catalog compiled into the crate: 32 curve files that ThrustCurve.org
//! marks public domain (the decision record on solid motors and the bundled catalog,
//! [ADR-005][adr-005]; how they were chosen, and the curves left out, are in
//! `docs/research/thrustcurve-data.md`). Other curves are fetched and cached by `hpr-net`, the
//! online layer that starts at [M5.1][m5-1].
//!
//! Every figure a motor lists in the bundled index comes from its curve file alone, written by
//! `cargo xtask motor-catalog` (the rule [ADR-186][adr-186] set, issue #295). ThrustCurve.org
//! grants no license for its API's motor records and reserves copyright. [ADR-287][adr-287] lets
//! the project use those records as published facts, with credit, so each motor also carries its
//! record's [`StatedFigures`], and each curve file its [`CurveCheck`] against them, the 1% rule
//! of [ADR-005][adr-005]. A motor flies its [`CatalogMotor::default_curve`], a passing file
//! where it has one ([ADR-298][adr-298]). [ADR-288][adr-288] plans the whole solid catalog for
//! [M5.7][m5-7].
//!
//! Size, masses and delays are the file header's; total impulse, average and peak thrust and burn
//! time are worked out from the curve by [`ThrustCurve`]'s own methods, the ones the simulator
//! flies (the trapezoid integral and the NFPA 1125 burn window; their definitions and sources are
//! on [`crate::curve`]). Only the
//! names that identify a motor (ThrustCurve.org's id for it, designation, maker, single-use or
//! reload, case, propellant) and the file's provenance (its ThrustCurve.org data-file id, who
//! produced the data, its license and the download date) are written by hand, in the command's
//! table.
//!
//! The index keeps the units motor files and catalogs use (mm, g, N·s, N, s), in fields named for
//! them. [`CatalogMotor::motor`] converts to SI and takes the envelope (diameter, length and
//! masses) from the index rather than the curve file's header. In the bundled index the two are
//! the same; another index (from [`Catalog::from_json`]) can correct a header that is wrong
//! ([Loft lesson L43][l43]: one said 75 mm for a 54 mm motor).
//!
//! [adr-005]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/DECISIONS.md#adr-005-solid-motors-statistics-consumption-grains-file-models-and-the-bundled-catalog-2026-09-17
//! [adr-186]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/DECISIONS.md#adr-186-the-data-network-and-python-fixes-of-release-01-2026-10-06
//! [adr-287]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/DECISIONS.md#adr-287-thrustcurveorgs-motor-records-used-as-published-facts-2026-10-20
//! [adr-288]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/DECISIONS.md#adr-288-every-solid-motor-ships-in-the-package-2026-10-21
//! [adr-298]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0298-every-solid-motor-in-four-increments.md
//! [l43]: https://hpr.fusionspace.co/decisions-and-roadmap.html#l43
//! [m5-1]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m5-1
//! [m5-7]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m5-7

use serde::{Deserialize, Serialize};

use crate::bundled;
use crate::class::ImpulseClass;
use crate::curve::ThrustCurve;
use crate::delay::DelayList;
use crate::error::MotorError;
use crate::motor::SolidMotor;
use crate::{eng, rse};

/// A motor catalog.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Catalog {
    /// Where the curve files come from.
    pub source: String,
    /// Where and when the curve files were downloaded.
    pub snapshot: Snapshot,
    /// The rule that chose the curves.
    pub selection: String,
    /// The motors.
    pub motors: Vec<CatalogMotor>,
}

/// Where and when a catalog's curve files were downloaded. Each file's own URL and SHA-256 are on
/// its [`CatalogCurve`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Snapshot {
    /// The site the files were downloaded from.
    pub files_url: String,
    /// The download date, `YYYY-MM-DD` (the latest, if the files were downloaded on several).
    pub captured: String,
}

/// Whether a motor is used once or reloaded into a reusable case.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum MotorType {
    /// A single-use motor (ThrustCurve `SU`).
    #[serde(rename = "SU")]
    SingleUse,
    /// A reload for a reusable case (ThrustCurve `reload`).
    #[serde(rename = "reload")]
    Reload,
    /// A hybrid motor (ThrustCurve `hybrid`). ThrustCurve lists them; HPR Sim doesn't model them (COTS
    /// solids only), and the bundled catalog has none.
    #[serde(rename = "hybrid")]
    Hybrid,
}

/// One motor's catalog entry, in the units motor files use.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogMotor {
    /// ThrustCurve.org's id for the motor.
    pub motor_id: String,
    /// Manufacturer name.
    pub manufacturer: String,
    /// Manufacturer abbreviation.
    pub manufacturer_abbrev: String,
    /// Full designation, such as `1266J760-19A`.
    pub designation: String,
    /// Common name, such as `J760`.
    pub common_name: String,
    /// Impulse class letter.
    pub impulse_class: ImpulseClass,
    /// Single-use or reload.
    pub motor_type: MotorType,
    /// The case a reload fits, such as `Pro54-3G`.
    pub case_info: Option<String>,
    /// The propellant.
    pub prop_info: Option<String>,
    /// Available delays as written (`6,10,14`, `4-6`, `P`; the bundled index copies the curve
    /// file's header); see [`CatalogMotor::delays`].
    pub delays: Option<String>,
    /// Diameter, mm.
    pub diameter_mm: f64,
    /// Length, mm.
    pub length_mm: f64,
    /// Total impulse, N·s ([`ThrustCurve::total_impulse_ns`] in the bundled index).
    pub total_impulse_ns: f64,
    /// Average thrust, N ([`ThrustCurve::average_thrust_n`] in the bundled index).
    pub average_thrust_n: f64,
    /// Peak thrust, N ([`ThrustCurve::peak_thrust_n`] in the bundled index).
    pub max_thrust_n: Option<f64>,
    /// Burn time, s ([`ThrustCurve::burn_time_s`], the NFPA 1125 window, in the bundled index).
    pub burn_time_s: f64,
    /// Propellant mass, g.
    pub propellant_mass_g: Option<f64>,
    /// Loaded mass, g.
    pub total_mass_g: Option<f64>,
    /// The figures ThrustCurve.org's record states for the motor, which each curve file is
    /// checked against ([`CurveCheck`]).
    pub stated: StatedFigures,
    /// The motor's curves.
    pub curves: Vec<CatalogCurve>,
}

/// The figures ThrustCurve.org's record of a motor states (its API's `totImpulseNs`,
/// `avgThrustN`, `burnTimeS` and `maxThrustN`), copied unchanged: published facts, used with
/// credit ([ADR-287][adr-287]). ThrustCurve.org works them out from a curve file it chooses, with
/// the burn time between the 5%-of-peak crossings (its *Motor Statistics* page,
/// <https://www.thrustcurve.org/info/motorstats.html>).
///
/// [adr-287]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0287-thrustcurve-orgs-motor-records-used-as-published-facts.md
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct StatedFigures {
    /// Total impulse, N·s.
    pub total_impulse_ns: f64,
    /// Average thrust, N.
    pub average_thrust_n: f64,
    /// Burn time, s.
    pub burn_time_s: f64,
    /// Peak thrust, N, where the record states one. No file is checked against it.
    pub max_thrust_n: Option<f64>,
}

/// The relative bound of a curve file's check: each figure within 1% of the motor's stated one
/// ([ADR-005][adr-005], [ADR-298][adr-298]).
///
/// [adr-005]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0005-solid-motors-statistics-consumption-grains-file.md
/// [adr-298]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0298-every-solid-motor-in-four-increments.md
pub const CHECK_BOUND: f64 = 0.01;

/// A curve file's check against its motor's [`StatedFigures`] ([ADR-005][adr-005]'s 1% rule,
/// definition A of the ThrustCurve.org data notes): the file's total impulse over its whole
/// curve, its NFPA 1125 burn time between the 5%-of-peak crossings, and its average thrust as
/// that impulse over that burn time, each worked out by [`ThrustCurve`]'s own methods, the ones
/// the simulator flies. The file passes when all three are within [`CHECK_BOUND`] of the stated
/// figures: `|file − stated| ≤ 0.01 × stated`, so a gap of exactly 1% passes. A pass says the
/// file agrees with the published figures; it says nothing about the curve's shape between them.
///
/// [adr-005]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0005-solid-motors-statistics-consumption-grains-file.md
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CurveCheck {
    /// The file's total impulse, N·s ([`ThrustCurve::total_impulse_ns`]).
    pub total_impulse_ns: f64,
    /// The file's average thrust, N ([`ThrustCurve::average_thrust_n`]).
    pub average_thrust_n: f64,
    /// The file's burn time, s ([`ThrustCurve::burn_time_s`]).
    pub burn_time_s: f64,
    /// Whether all three are within [`CHECK_BOUND`] of the motor's stated figures.
    pub passes: bool,
}

/// A figure a curve file's check compares.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum CheckedFigure {
    /// Total impulse.
    TotalImpulse,
    /// Average thrust.
    AverageThrust,
    /// Burn time.
    BurnTime,
}

impl CheckedFigure {
    /// The figure's name in words: `total impulse`, `average thrust`, `burn time`.
    pub fn name(self) -> &'static str {
        match self {
            Self::TotalImpulse => "total impulse",
            Self::AverageThrust => "average thrust",
            Self::BurnTime => "burn time",
        }
    }
}

/// One figure of a curve file's check: the file's value and the motor's stated one.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FigureGap {
    /// Which figure.
    pub figure: CheckedFigure,
    /// The file's value.
    pub file: f64,
    /// ThrustCurve.org's stated value.
    pub stated: f64,
}

impl FigureGap {
    /// The signed relative gap, `(file − stated) / stated`: positive when the file's figure is
    /// the larger.
    pub fn relative(&self) -> f64 {
        (self.file - self.stated) / self.stated
    }

    /// Whether the file's figure is within [`CHECK_BOUND`] of the stated one,
    /// `|file − stated| ≤ 0.01 × stated`. A stated figure that isn't positive, or a NaN either
    /// side, fails.
    pub fn within_bound(&self) -> bool {
        self.stated > 0.0 && (self.file - self.stated).abs() <= CHECK_BOUND * self.stated
    }
}

impl CurveCheck {
    /// The check of a curve against a motor's stated figures.
    pub fn of(thrust: &ThrustCurve, stated: &StatedFigures) -> Self {
        let mut check = Self {
            total_impulse_ns: thrust.total_impulse_ns(),
            average_thrust_n: thrust.average_thrust_n(),
            burn_time_s: thrust.burn_time_s(),
            passes: false,
        };
        check.passes = check.gaps(stated).iter().all(FigureGap::within_bound);
        check
    }

    /// The three figures against `stated`: total impulse, average thrust, burn time.
    pub fn gaps(&self, stated: &StatedFigures) -> [FigureGap; 3] {
        [
            FigureGap {
                figure: CheckedFigure::TotalImpulse,
                file: self.total_impulse_ns,
                stated: stated.total_impulse_ns,
            },
            FigureGap {
                figure: CheckedFigure::AverageThrust,
                file: self.average_thrust_n,
                stated: stated.average_thrust_n,
            },
            FigureGap {
                figure: CheckedFigure::BurnTime,
                file: self.burn_time_s,
                stated: stated.burn_time_s,
            },
        ]
    }
}

/// Where a thrust curve came from, under what terms, and how it compares with its motor's stated
/// figures.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogCurve {
    /// ThrustCurve's simfile id.
    pub simfile_id: String,
    /// The file format.
    pub format: CurveFormat,
    /// Who produced the data.
    pub source: CurveSource,
    /// The data license.
    #[serde(default)]
    pub license: CurveLicense,
    /// The file's path relative to the catalog index.
    pub file: String,
    /// SHA-256 of the file's bytes, hex.
    pub sha256: String,
    /// Where the file was downloaded from.
    pub url: String,
    /// ThrustCurve's page for the file.
    pub info_url: Option<String>,
    /// The download date, `YYYY-MM-DD`.
    pub retrieved: String,
    /// The file's check against its motor's stated figures, worked out when the index is written.
    pub check: CurveCheck,
}

/// A thrust-curve file format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum CurveFormat {
    /// RASP `.eng`.
    #[serde(rename = "RASP")]
    Rasp,
    /// RockSim `.rse`.
    #[serde(rename = "RockSim")]
    RockSim,
}

/// Who produced a curve (ThrustCurve's `source`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum CurveSource {
    /// A certification body's test data.
    #[serde(rename = "cert")]
    Certification,
    /// The manufacturer.
    #[serde(rename = "mfr")]
    Manufacturer,
    /// A ThrustCurve user.
    #[serde(rename = "user")]
    User,
}

/// A curve's data license, as ThrustCurve records it. ThrustCurve's API leaves the key out when no
/// license is recorded, which reads as [`CurveLicense::Unknown`].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum CurveLicense {
    /// Public domain.
    #[serde(rename = "PD")]
    PublicDomain,
    /// "Free": by ThrustCurve's definition this includes GPL and Creative Commons terms.
    #[serde(rename = "free")]
    Free,
    /// Other, restricted terms.
    #[serde(rename = "other")]
    Other,
    /// No license recorded.
    #[default]
    #[serde(rename = "unknown", alias = "")]
    Unknown,
}

impl CurveLicense {
    /// Whether a curve under this license may be bundled in this `MIT OR Apache-2.0` crate: only
    /// public domain. "Free" can mean GPL, and the others are restricted or unknown.
    pub fn is_bundleable(self) -> bool {
        self == Self::PublicDomain
    }
}

impl Catalog {
    /// The catalog compiled into this crate.
    ///
    /// # Errors
    ///
    /// [`MotorError::Catalog`] if the bundled index doesn't deserialize, which a test rules out.
    pub fn bundled() -> Result<Self, MotorError> {
        Self::from_json(bundled::CATALOG_JSON)
    }

    /// Reads a catalog index.
    ///
    /// # Errors
    ///
    /// [`MotorError::Catalog`] if the text isn't a catalog index.
    pub fn from_json(text: &str) -> Result<Self, MotorError> {
        serde_json::from_str(text).map_err(|error| MotorError::Catalog(error.to_string()))
    }

    /// The motors whose designation or common name matches `name`, ignoring ASCII case, spaces
    /// and hyphens (`j760`, `1266J760-19A`, `K 940`).
    pub fn find<'a>(&'a self, name: &str) -> impl Iterator<Item = &'a CatalogMotor> + 'a {
        let key = normalize(name);
        self.motors.iter().filter(move |motor| {
            normalize(&motor.designation) == key || normalize(&motor.common_name) == key
        })
    }
}

/// Lowercase, without spaces or hyphens.
fn normalize(name: &str) -> String {
    name.chars()
        .filter(|c| !c.is_whitespace() && *c != '-')
        .map(|c| c.to_ascii_lowercase())
        .collect()
}

/// A curve file's thrust curve and its header's propellant and loaded masses, kg.
fn read_curve_file(
    curve: &CatalogCurve,
    text: &str,
) -> Result<(ThrustCurve, f64, f64), MotorError> {
    let one = |count: usize| {
        if count == 1 {
            Ok(())
        } else {
            Err(MotorError::Inconsistent(format!(
                "the curve file {} holds {count} motors, not one",
                curve.file
            )))
        }
    };
    match curve.format {
        CurveFormat::Rasp => {
            let file = eng::parse(text)?.value;
            one(file.entries.len())?;
            let entry = &file.entries[0];
            Ok((
                entry.thrust_curve()?,
                entry.propellant_mass_kg,
                entry.total_mass_kg,
            ))
        }
        CurveFormat::RockSim => {
            let file = rse::parse(text)?.value;
            one(file.engines.len())?;
            let engine = &file.engines[0];
            Ok((
                engine.thrust_curve()?,
                engine.propellant_mass_g * 1e-3,
                engine.initial_mass_g * 1e-3,
            ))
        }
    }
}

/// The text of a bundled curve file, by its path in the bundled index.
pub fn bundled_curve_text(file: &str) -> Option<&'static str> {
    bundled::CURVE_FILES
        .iter()
        .find(|(path, _)| *path == file)
        .map(|(_, text)| *text)
}

impl CatalogMotor {
    /// The delay settings read from [`CatalogMotor::delays`]; empty when none are recorded.
    pub fn delays(&self) -> DelayList {
        DelayList::parse(self.delays.as_deref().unwrap_or(""))
    }

    /// Reads a curve of this motor from its file's text.
    ///
    /// # Errors
    ///
    /// - [`MotorError::Inconsistent`] if `curve` is not one of this motor's curves, or the file
    ///   doesn't hold exactly one motor.
    /// - The format's parse error ([`eng::parse`], [`rse::parse`]).
    /// - [`ThrustCurve::new`]'s errors.
    pub fn thrust_curve(
        &self,
        curve: &CatalogCurve,
        text: &str,
    ) -> Result<ThrustCurve, MotorError> {
        self.check_own(curve)?;
        read_curve_file(curve, text).map(|(thrust, _, _)| thrust)
    }

    /// Refuses a curve listed under another motor, which would pair this motor's masses and
    /// envelope with the wrong thrust.
    fn check_own(&self, curve: &CatalogCurve) -> Result<(), MotorError> {
        if self.curves.contains(curve) {
            Ok(())
        } else {
            Err(MotorError::Inconsistent(format!(
                "curve {} is not one of {}'s curves",
                curve.simfile_id, self.designation
            )))
        }
    }

    /// The motor with the curve read from `text`, built with [`SolidMotor::from_envelope`] from
    /// this entry's diameter, length and masses. The curve file's header is used only for a mass
    /// the metadata lacks.
    ///
    /// # Errors
    ///
    /// As [`CatalogMotor::thrust_curve`] and [`SolidMotor::from_envelope`], and
    /// [`MotorError::Inconsistent`] for a hybrid (HPR Sim models solids only) or when neither the
    /// metadata nor the file gives the masses. The text is not checked against the curve's SHA-256:
    /// pass the file the curve names.
    pub fn motor(&self, curve: &CatalogCurve, text: &str) -> Result<SolidMotor, MotorError> {
        if self.motor_type == MotorType::Hybrid {
            return Err(MotorError::Inconsistent(format!(
                "{} is a hybrid; HPR Sim models solid motors only",
                self.designation
            )));
        }
        self.check_own(curve)?;
        let (thrust, header_propellant_kg, header_total_kg) = read_curve_file(curve, text)?;
        let propellant_kg = self
            .propellant_mass_g
            .map_or(header_propellant_kg, |g| g * 1e-3);
        let total_kg = self.total_mass_g.map_or(header_total_kg, |g| g * 1e-3);
        if !(propellant_kg > 0.0 && total_kg > 0.0) {
            return Err(MotorError::Inconsistent(format!(
                "{} has no propellant and loaded masses",
                self.designation
            )));
        }
        SolidMotor::from_envelope(
            thrust,
            self.diameter_mm * 1e-3,
            self.length_mm * 1e-3,
            propellant_kg,
            total_kg,
        )
    }

    /// The motor with its default curve ([`CatalogMotor::default_curve`]).
    ///
    /// # Errors
    ///
    /// [`MotorError::Inconsistent`] if the entry has no bundled curve file, and
    /// [`CatalogMotor::motor`]'s errors.
    pub fn bundled_motor(&self) -> Result<SolidMotor, MotorError> {
        let curve = self.default_curve().ok_or_else(|| {
            MotorError::Inconsistent(format!("{} has no bundled curve", self.designation))
        })?;
        // `default_curve` takes only a curve with a bundled file.
        let text = bundled_curve_text(&curve.file).unwrap_or_default();
        self.motor(curve, text)
    }

    /// The motor's curves in the order its default is taken ([ADR-298][adr-298] §5): a file
    /// whose check passes first, then the order [ADR-154][adr-154] fetches in, RASP before
    /// RockSim, each ranked by who measured it (a certification test, the maker, then a user),
    /// the index's order within a rank.
    ///
    /// [adr-154]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0154-motors-fetched-from-thrustcurve-by-name.md
    /// [adr-298]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0298-every-solid-motor-in-four-increments.md
    pub fn ranked_curves(&self) -> Vec<&CatalogCurve> {
        let mut curves: Vec<&CatalogCurve> = self.curves.iter().collect();
        // Stable: within a rank, the index's order.
        curves.sort_by_key(|curve| {
            let format = match curve.format {
                CurveFormat::Rasp => 0,
                CurveFormat::RockSim => 1,
            };
            let source = match curve.source {
                CurveSource::Certification => 0,
                CurveSource::Manufacturer => 1,
                CurveSource::User => 2,
            };
            (!curve.check.passes, format, source)
        });
        curves
    }

    /// The curve a flight takes when none is named: the first of
    /// [`CatalogMotor::ranked_curves`] whose file is bundled, so a passing file where one is.
    pub fn default_curve(&self) -> Option<&CatalogCurve> {
        self.ranked_curves()
            .into_iter()
            .find(|curve| bundled_curve_text(&curve.file).is_some())
    }

    /// The figures of `curve`'s check outside [`CHECK_BOUND`] of this motor's stated ones, in the
    /// order total impulse, average thrust, burn time; empty when the file passes.
    pub fn failing_figures(&self, curve: &CatalogCurve) -> Vec<FigureGap> {
        curve
            .check
            .gaps(&self.stated)
            .into_iter()
            .filter(|gap| !gap.within_bound())
            .collect()
    }

    /// The warning for flying `curve` when its check fails, naming the motor, the file and each
    /// failing figure's signed gap, file against ThrustCurve.org's stated figure; `None` when it
    /// passes.
    pub fn check_warning(&self, curve: &CatalogCurve) -> Option<String> {
        if curve.check.passes {
            return None;
        }
        let gaps: Vec<String> = self
            .failing_figures(curve)
            .iter()
            .map(|gap| format!("{} {:+.2}%", gap.figure.name(), 100.0 * gap.relative()))
            .collect();
        Some(format!(
            "{}'s curve file {} is more than 1% from ThrustCurve.org's figures for it: {}",
            self.designation,
            curve.simfile_id,
            gaps.join(", ")
        ))
    }
}

#[cfg(test)]
mod tests {
    use sha2::{Digest, Sha256};

    use super::*;
    use crate::delay::Delay;

    fn bundled() -> Catalog {
        Catalog::bundled().unwrap()
    }

    /// Loft lesson L41: Loft's bundled curves were 45 public domain, 38 with no license, 22
    /// unknown and 3 "free" (which can include GPL).
    #[test]
    fn bundled_curves_have_permissive_license() {
        let catalog = bundled();
        assert!(!catalog.motors.is_empty());
        let mut files = 0;
        for motor in &catalog.motors {
            for curve in &motor.curves {
                assert!(
                    curve.license.is_bundleable(),
                    "{} {}: {:?}",
                    motor.designation,
                    curve.simfile_id,
                    curve.license
                );
                assert!(bundled_curve_text(&curve.file).is_some(), "{}", curve.file);
                files += 1;
            }
        }
        // Every compiled-in file is indexed, so nothing unlicensed rides along.
        assert_eq!(files, bundled::CURVE_FILES.len());
        // Each file is byte for byte what was downloaded from its recorded URL.
        for motor in &catalog.motors {
            for curve in &motor.curves {
                let text = bundled_curve_text(&curve.file).unwrap();
                let digest: String = Sha256::digest(text.as_bytes())
                    .iter()
                    .map(|byte| format!("{byte:02x}"))
                    .collect();
                assert_eq!(digest, curve.sha256, "{}", curve.file);
                assert!(curve.url.contains(&curve.simfile_id));
            }
        }
        assert!(!CurveLicense::Free.is_bundleable());
        assert!(!CurveLicense::Unknown.is_bundleable());
    }

    /// Loft lesson L42: Loft's impulse checks allowed −8% to +8%, and a mis-sourced curve flew
    /// about 26% high until caught. Every figure the bundled index states is its curve's own, bit
    /// for bit: `cargo xtask motor-catalog` works them out with the methods the simulator flies
    /// (#295), so what `hpr motors` lists is what flies. Each is also within 1% of what
    /// ThrustCurve.org's own code works out from the same file (M1.3's bound; the fixture
    /// [`every_bundled_curve_matches_thrustcurve_statistics_code`] holds to 1e-12).
    #[test]
    fn every_bundled_curve_impulse_within_1pct_of_thrustcurve() {
        let oracle: AnalyzeOracle = serde_json::from_str(include_str!(
            "../../../validation/fixtures/motor/thrustcurve-analyze-stats.json"
        ))
        .unwrap();
        let catalog = bundled();
        let mut checked = 0;
        for motor in &catalog.motors {
            for curve in &motor.curves {
                let text = bundled_curve_text(&curve.file).unwrap();
                let thrust = motor.thrust_curve(curve, text).unwrap();
                let theirs = oracle
                    .curves
                    .iter()
                    .find(|c| c.file == curve.file)
                    .unwrap_or_else(|| panic!("{}: not in the oracle record", curve.file));
                for (stated, computed, by_thrustcurve, what) in [
                    (
                        motor.total_impulse_ns,
                        thrust.total_impulse_ns(),
                        theirs.total_impulse_ns,
                        "total impulse",
                    ),
                    (
                        motor.average_thrust_n,
                        thrust.average_thrust_n(),
                        theirs.average_thrust_n,
                        "average thrust",
                    ),
                    (
                        motor.burn_time_s,
                        thrust.burn_time_s(),
                        theirs.burn_time_s,
                        "burn time",
                    ),
                    (
                        motor.max_thrust_n.unwrap(),
                        thrust.peak_thrust_n(),
                        theirs.max_thrust_n,
                        "peak thrust",
                    ),
                ] {
                    assert_eq!(
                        stated.to_bits(),
                        computed.to_bits(),
                        "{} ({}): the index's {what} {stated} is not its curve's {computed}",
                        motor.designation,
                        curve.simfile_id,
                    );
                    let error = (stated - by_thrustcurve) / by_thrustcurve;
                    assert!(
                        error.abs() <= 0.01,
                        "{} ({}): {what} {stated} against ThrustCurve's code's {by_thrustcurve} \
                         ({:+.2}%)",
                        motor.designation,
                        curve.simfile_id,
                        100.0 * error
                    );
                }
                assert_eq!(
                    ImpulseClass::from_total_impulse(motor.total_impulse_ns).unwrap(),
                    motor.impulse_class,
                    "{}",
                    motor.designation
                );
                checked += 1;
            }
        }
        assert_eq!(checked, 32);
    }

    /// Every bundled entry's size, masses and delays are its curve file header's (#295): the
    /// masses to the microgram, as `cargo xtask motor-catalog` writes a `.eng` header's kilograms
    /// in grams.
    #[test]
    fn every_bundled_size_mass_and_delay_is_the_header_s() {
        let catalog = bundled();
        for motor in &catalog.motors {
            let [curve] = &motor.curves[..] else {
                panic!("{}: {} curves", motor.designation, motor.curves.len());
            };
            let text = bundled_curve_text(&curve.file).unwrap();
            let (diameter_mm, length_mm, delays, propellant_g, total_g) = match curve.format {
                CurveFormat::Rasp => {
                    let entry = eng::parse(text).unwrap().value.entries.remove(0);
                    (
                        entry.diameter_mm,
                        entry.length_mm,
                        Some(entry.delays),
                        entry.propellant_mass_kg * 1e3,
                        entry.total_mass_kg * 1e3,
                    )
                }
                CurveFormat::RockSim => {
                    let engine = rse::parse(text).unwrap().value.engines.remove(0);
                    (
                        engine.diameter_mm,
                        engine.length_mm,
                        engine.delays,
                        engine.propellant_mass_g,
                        engine.initial_mass_g,
                    )
                }
            };
            let name = &motor.designation;
            assert_eq!(motor.diameter_mm, diameter_mm, "{name}");
            assert_eq!(motor.length_mm, length_mm, "{name}");
            assert_eq!(motor.delays, delays, "{name}");
            assert!(
                (motor.propellant_mass_g.unwrap() - propellant_g).abs() <= 5e-7,
                "{name}"
            );
            assert!(
                (motor.total_mass_g.unwrap() - total_g).abs() <= 5e-7,
                "{name}"
            );
        }
    }

    /// Of ThrustCurve.org's API records the bundled index copies only each motor's four stated
    /// figures (ADR-298; `records.json` beside the files holds the records unchanged): no
    /// certifying body, update date, availability or adjustable-delay flag, and no API capture.
    #[test]
    fn bundled_index_holds_no_thrustcurve_record_fields() {
        for key in [
            "cert_org",
            "updated_on",
            "availability",
            "delay_adjustable",
            "motors_url",
            "api/v1",
        ] {
            assert!(
                !bundled::CATALOG_JSON.contains(key),
                "the bundled index holds {key}"
            );
        }
        let catalog = bundled();
        assert_eq!(
            catalog.snapshot.files_url,
            "https://www.thrustcurve.org/simfiles/"
        );
        for curve in catalog.motors.iter().flat_map(|m| &m.curves) {
            assert!(curve.url.starts_with(&catalog.snapshot.files_url));
        }
    }

    /// Loft lesson L43: a catalog's envelope overrides the curve file's header; one header said
    /// 75 mm for a 54 mm motor. The bundled index now takes its envelope from the headers
    /// themselves (#295; `cargo xtask motor-catalog` refuses a reload whose case names another
    /// diameter), so the override is shown on an index entry made to differ from its header: an
    /// invented 1000 mm and 12,000 g for the N3300R, whose file says 1060 mm.
    #[test]
    fn metadata_overrides_header_envelope() {
        let catalog = bundled();
        let bundled_entry = catalog.find("N3300R").next().unwrap();
        let curve = &bundled_entry.curves[0];
        let text = bundled_curve_text(&curve.file).unwrap();
        let header = &eng::parse(text).unwrap().value.entries[0];
        assert_eq!(header.length_mm, 1060.0);
        assert_eq!(bundled_entry.length_mm, 1060.0);
        let motor = CatalogMotor {
            length_mm: 1000.0,
            total_mass_g: Some(12000.0),
            ..bundled_entry.clone()
        };
        let built = motor.motor(curve, text).unwrap();
        // The envelope default centers the dry mass on the index's length, not the header's.
        assert!((built.dry().cg_m - 0.500).abs() < 1e-12);
        let loaded = built.state(0.0).total;
        assert!((loaded.mass_kg - 12.000).abs() < 1e-9);
        assert!((bundled_entry.bundled_motor().unwrap().dry().cg_m - 0.530).abs() < 1e-12);
        // With no index masses, the header's are used.
        let bare = CatalogMotor {
            propellant_mass_g: None,
            total_mass_g: None,
            ..motor.clone()
        };
        let fallback = bare.motor(curve, text).unwrap();
        assert!((fallback.state(0.0).total.mass_kg - header.total_mass_kg).abs() < 1e-12);
        assert!((fallback.dry().cg_m - 0.500).abs() < 1e-12);
    }

    /// M1.3's done-when: parse-write-parse round trips are identical, on every bundled file.
    #[test]
    fn every_bundled_file_round_trips_bit_for_bit() {
        let catalog = bundled();
        for motor in &catalog.motors {
            for curve in &motor.curves {
                let text = bundled_curve_text(&curve.file).unwrap();
                match curve.format {
                    CurveFormat::Rasp => {
                        let first = eng::parse(text).unwrap().value;
                        let written = eng::write(&first).unwrap();
                        let second = eng::parse(&written).unwrap().value;
                        assert_eq!(second, first, "{}", curve.file);
                        assert_eq!(eng::tests::bits(&second), eng::tests::bits(&first));
                        assert_eq!(eng::write(&second).unwrap(), written);
                    }
                    CurveFormat::RockSim => {
                        let first = rse::parse(text).unwrap().value;
                        let written = rse::write(&first).unwrap();
                        let second = rse::parse(&written).unwrap().value;
                        assert_eq!(second, first, "{}", curve.file);
                        assert_eq!(rse::tests::bits(&second), rse::tests::bits(&first));
                        assert_eq!(rse::write(&second).unwrap(), written);
                    }
                }
            }
        }
    }

    #[test]
    fn finds_motors_and_reads_delays() {
        let catalog = bundled();
        let j760: Vec<_> = catalog.find("j760").collect();
        assert_eq!(j760.len(), 1);
        assert_eq!(j760[0].designation, "1266J760-19A");
        assert_eq!(catalog.find("1266J760-19A").count(), 1);
        let plugged = catalog.find("L3200").next().unwrap();
        assert_eq!(plugged.delays().delays, [Delay::Plugged]);
        assert!(catalog.find("no such motor").next().is_none());
        // Every bundled motor builds.
        for motor in &catalog.motors {
            let built = motor.bundled_motor().unwrap();
            assert!(
                built.propellant_initial_mass_kg() > 0.0,
                "{}",
                motor.designation
            );
        }
    }

    #[test]
    fn catalog_json_reads_thrustcurve_license_values_and_rejects_others() {
        assert!(matches!(
            Catalog::from_json("{}"),
            Err(MotorError::Catalog(_))
        ));
        let with = |license: &str| {
            Catalog::from_json(&bundled::CATALOG_JSON.replacen("\"license\": \"PD\",", license, 1))
        };
        let first_license = |catalog: Catalog| catalog.motors[0].curves[0].license;
        assert!(with("\"license\": \"GPL\",").is_err());
        assert_eq!(
            first_license(with("\"license\": \"free\",").unwrap()),
            CurveLicense::Free
        );
        // ThrustCurve's API leaves the key out, or blank, when no license is recorded.
        assert_eq!(first_license(with("").unwrap()), CurveLicense::Unknown);
        assert_eq!(
            first_license(with("\"license\": \"\",").unwrap()),
            CurveLicense::Unknown
        );
        let hybrid = bundled::CATALOG_JSON.replacen(
            "\"motor_type\": \"SU\"",
            "\"motor_type\": \"hybrid\"",
            1,
        );
        let hybrid = Catalog::from_json(&hybrid).unwrap();
        assert_eq!(hybrid.motors[0].motor_type, MotorType::Hybrid);
        assert!(hybrid.motors[0].bundled_motor().is_err());
    }

    #[test]
    fn a_curve_of_another_motor_is_refused() {
        let catalog = Catalog::bundled().unwrap();
        let (this, other) = (&catalog.motors[0], &catalog.motors[1]);
        let curve = &other.curves[0];
        let text = bundled_curve_text(&curve.file).unwrap();
        assert!(other.motor(curve, text).is_ok());
        assert!(matches!(
            this.motor(curve, text),
            Err(MotorError::Inconsistent(message)) if message.contains(&curve.simfile_id)
        ));
        assert!(this.thrust_curve(curve, text).is_err());
    }

    #[derive(Debug, serde::Deserialize)]
    struct AnalyzeOracle {
        oracle: String,
        curves: Vec<AnalyzeCurve>,
    }

    #[derive(Debug, serde::Deserialize)]
    struct AnalyzeCurve {
        file: String,
        total_impulse_ns: f64,
        burn_time_s: f64,
        average_thrust_n: f64,
        max_thrust_n: f64,
        burn_start_s: f64,
        burn_end_s: f64,
    }

    /// hpr's statistics against ThrustCurve.org's own code (`simulate/analyze/analyze.js`, run
    /// unchanged by `validation/oracles/thrustcurve/analyze_stats.js`) on every bundled curve.
    /// Unlike the 1% check against the stored values, which chose the bundle, this can fail: it
    /// checks that hpr computes what ThrustCurve computes, to rounding.
    #[test]
    fn every_bundled_curve_matches_thrustcurve_statistics_code() {
        let oracle: AnalyzeOracle = serde_json::from_str(include_str!(
            "../../../validation/fixtures/motor/thrustcurve-analyze-stats.json"
        ))
        .unwrap();
        assert!(oracle.oracle.contains("analyze.js at commit 577afa6"));
        let catalog = bundled();
        assert_eq!(oracle.curves.len(), bundled::CURVE_FILES.len());
        let mut worst = 0.0f64;
        for expected in &oracle.curves {
            let (motor, curve) = catalog
                .motors
                .iter()
                .find_map(|m| {
                    m.curves
                        .iter()
                        .find(|c| c.file == expected.file)
                        .map(|c| (m, c))
                })
                .unwrap();
            let thrust = motor
                .thrust_curve(curve, bundled_curve_text(&curve.file).unwrap())
                .unwrap();
            let (start, end) = thrust.burn_window_s();
            for (ours, theirs, what) in [
                (
                    thrust.total_impulse_ns(),
                    expected.total_impulse_ns,
                    "total impulse",
                ),
                (thrust.burn_time_s(), expected.burn_time_s, "burn time"),
                (
                    thrust.average_thrust_n(),
                    expected.average_thrust_n,
                    "average thrust",
                ),
                (thrust.peak_thrust_n(), expected.max_thrust_n, "peak thrust"),
                (start, expected.burn_start_s, "burn start"),
                (end, expected.burn_end_s, "burn end"),
            ] {
                let error = (ours - theirs).abs() / theirs.abs().max(1e-3);
                worst = worst.max(error);
                assert!(
                    error <= 1e-12,
                    "{}: {what} {ours} against {theirs}",
                    expected.file
                );
            }
        }
        eprintln!("largest relative difference from ThrustCurve's code: {worst:.2e}");
    }

    /// OpenRocket 24.12's record of the same files, written by
    /// `validation/oracles/openrocket/motors.py`.
    #[derive(Debug, serde::Deserialize)]
    struct OpenRocketRecord {
        openrocket: String,
        inputs_sha256: std::collections::BTreeMap<String, String>,
        jar_sha256: String,
        curves: Vec<OpenRocketCurve>,
    }

    #[derive(Debug, serde::Deserialize)]
    struct OpenRocketCurve {
        file: String,
        sha256: String,
        #[serde(default)]
        motors: Vec<OpenRocketMotor>,
        #[serde(default)]
        driver_error: Option<String>,
    }

    #[derive(Debug, serde::Deserialize)]
    struct OpenRocketMotor {
        common_name: String,
        total_impulse_ns: f64,
        max_thrust_n: f64,
        average_thrust_n: f64,
        burn_time_s: f64,
        burn_time_estimate_s: f64,
        first_time_s: f64,
        points: usize,
    }

    /// hpr's total impulse against OpenRocket 24.12's own estimate for the same file, on every
    /// bundled curve (M2.2c1). Both integrate the file's points with the trapezoid rule, so the
    /// milestone's 0.1% is met with room to spare; the check is the bound the milestone names, and
    /// the run prints the spread.
    ///
    /// Three more of OpenRocket's numbers are the same quantity as one of hpr's, so they are held
    /// too: `getMaxThrustEstimate` is the largest *listed* thrust, which is
    /// [`ThrustCurve::peak_thrust_n`] on every bundled curve (none lists a sample a step skips),
    /// `getBurnTimeEstimate` is the 5%-of-peak window ([`ThrustCurve::burn_time_s`], the NFPA 1125
    /// rule), and `getBurnTime` is the curve's whole duration ([`ThrustCurve::end_time_s`]), not a
    /// window. OpenRocket prepends an origin to a file whose first point is after ignition, exactly
    /// as hpr does, so the point counts are held equal as well.
    ///
    /// One quantity is a real difference, printed here and written down with its size in ADR-066
    /// rather than held: OpenRocket's average thrust divides the impulse *inside* the window by the
    /// window, where hpr (with ThrustCurve.org's code) divides the whole curve's impulse by it, so
    /// hpr's is the higher on every curve.
    #[test]
    fn openrocket_s_total_impulse_matches_every_bundled_curve() {
        let record: OpenRocketRecord = serde_json::from_str(include_str!(
            "../../../validation/fixtures/motor/openrocket-curve-stats.json"
        ))
        .unwrap();
        assert_eq!(record.openrocket, "24.12");
        // The record moves only when its script runs, never by hand (Loft lesson L76).
        assert_eq!(
            record.inputs_sha256["motors.py"],
            "1049b94c2c0da896dd84887a06b5ff8fd72865350aa98a2f9e1c8b7e14d5e3e7",
        );
        assert_eq!(
            record.jar_sha256,
            "4959b72f52f5f607941e9722abbb7b7f0c4a38ebbbf84204a329db9f31c4f897",
        );
        assert_eq!(record.curves.len(), bundled::CURVE_FILES.len());
        let catalog = bundled();
        let mut worst_impulse = 0.0f64;
        let mut worst_peak = 0.0f64;
        let mut average = Vec::new();
        let mut identical = true;
        let mut files = std::collections::BTreeSet::new();
        for entry in &record.curves {
            let file = entry
                .file
                .strip_prefix("crates/hpr-motor/data/thrustcurve/")
                .expect("the record names the bundled file");
            assert_eq!(entry.driver_error, None, "{file}");
            let [theirs] = &entry.motors[..] else {
                panic!("{file}: OpenRocket read {} motors", entry.motors.len());
            };
            let (motor, curve) = catalog
                .motors
                .iter()
                .find_map(|m| m.curves.iter().find(|c| c.file == file).map(|c| (m, c)))
                .unwrap_or_else(|| panic!("{file} is not a bundled curve"));
            // The record is tied to the bytes OpenRocket read, which are the bytes hpr reads.
            assert_eq!(curve.sha256, entry.sha256, "{file}");
            assert!(files.insert(file.to_owned()), "{file} twice");
            // The two catalogs spell a designation differently (ThrustCurve's `131G84-10A` is
            // OpenRocket's `131-G84-GR-10A`, read from the file's own header), so the common name
            // is what can be compared; the file's SHA-256 is what ties the record to the motor.
            assert_eq!(
                normalize(&motor.common_name),
                normalize(&theirs.common_name),
                "{file}"
            );
            let text = bundled_curve_text(file).unwrap();
            let thrust = motor.thrust_curve(curve, text).unwrap();
            for (ours, theirs, what, bound) in [
                (
                    thrust.total_impulse_ns(),
                    theirs.total_impulse_ns,
                    "total impulse",
                    1e-3,
                ),
                (
                    thrust.peak_thrust_n(),
                    theirs.max_thrust_n,
                    "peak thrust",
                    1e-12,
                ),
                (
                    thrust.burn_time_s(),
                    theirs.burn_time_estimate_s,
                    "burn time",
                    1e-12,
                ),
                (
                    thrust.end_time_s(),
                    theirs.burn_time_s,
                    "curve duration",
                    1e-12,
                ),
            ] {
                assert!(theirs > 0.0, "{file}: {what} {theirs}");
                let error = (ours - theirs).abs() / theirs;
                if what == "total impulse" {
                    worst_impulse = worst_impulse.max(error);
                } else {
                    worst_peak = worst_peak.max(error);
                }
                assert!(error <= bound, "{file}: {what} {ours} against {theirs}");
                // Each is the same quantity computed twice, and each comes out bit for bit equal,
                // which is what the guide and ADR-066 claim.
                identical &= ours == theirs;
            }
            // Both readers prepend `(0, 0)` to a file whose first point is after ignition (29 of
            // the 32), so both count that point and neither starts late.
            assert_eq!(thrust.times_s()[0], 0.0, "{file}");
            assert_eq!(theirs.first_time_s, 0.0, "{file}");
            assert_eq!(thrust.times_s().len(), theirs.points, "{file}");
            // The one definition that differs, measured rather than held, and signed: hpr's
            // whole-curve numerator makes it the higher on every curve.
            average.push(
                (thrust.average_thrust_n() - theirs.average_thrust_n) / theirs.average_thrust_n,
            );
        }
        assert_eq!(
            files.len(),
            bundled::CURVE_FILES.len(),
            "a file twice or missing"
        );
        assert!(
            identical,
            "impulse, peak thrust, burn time and duration are no longer bit for bit OpenRocket's"
        );
        average.sort_by(f64::total_cmp);
        let mid = average.len() / 2;
        let median = if average.len().is_multiple_of(2) {
            0.5 * (average[mid - 1] + average[mid])
        } else {
            average[mid]
        };
        let (low, high) = (average[0] * 100.0, average[average.len() - 1] * 100.0);
        assert!(low > 0.0, "hpr's average thrust is no longer the higher");
        eprintln!(
            "against OpenRocket 24.12 on {} curves: total impulse, peak thrust, the 5% burn-time \
             window and the curve's duration are all bit for bit equal (worst relative differences \
             {worst_impulse:.2e} and {worst_peak:.2e}); hpr's average thrust is higher by \
             {low:+.4}% to {high:+.4}% (median {:+.4}%), because its numerator is the whole curve's \
             impulse and OpenRocket's is the window's",
            average.len(),
            median * 100.0,
        );
    }

    /// ADR-298 §4: every bundled file's stored check is its curve's figures against its motor's
    /// stated ones, worked out again here, and the stated figures are the committed record's,
    /// bit for bit. Each of the 32 passes, as they were chosen to, so none warns.
    #[test]
    fn every_bundled_check_is_its_file_against_its_record() {
        let records: serde_json::Value =
            serde_json::from_str(include_str!("../data/thrustcurve/records.json")).unwrap();
        let records = records["records"].as_array().unwrap();
        let bits = |value: &serde_json::Value| value.as_f64().map(f64::to_bits);
        let catalog = bundled();
        let mut checked = 0;
        for motor in &catalog.motors {
            let name = &motor.designation;
            let record = records
                .iter()
                .find(|r| r["motorId"] == motor.motor_id.as_str())
                .unwrap_or_else(|| panic!("{name}: no record"));
            let stated = &motor.stated;
            assert_eq!(
                Some(stated.total_impulse_ns.to_bits()),
                bits(&record["totImpulseNs"])
            );
            assert_eq!(
                Some(stated.average_thrust_n.to_bits()),
                bits(&record["avgThrustN"])
            );
            assert_eq!(
                Some(stated.burn_time_s.to_bits()),
                bits(&record["burnTimeS"])
            );
            assert_eq!(
                stated.max_thrust_n.map(f64::to_bits),
                bits(&record["maxThrustN"])
            );
            for curve in &motor.curves {
                let text = bundled_curve_text(&curve.file).unwrap();
                let thrust = motor.thrust_curve(curve, text).unwrap();
                assert_eq!(CurveCheck::of(&thrust, stated), curve.check, "{name}");
                assert!(curve.check.passes, "{name}");
                assert!(motor.failing_figures(curve).is_empty(), "{name}");
                assert_eq!(motor.check_warning(curve), None, "{name}");
                checked += 1;
            }
        }
        assert_eq!(checked, bundled::CURVE_FILES.len());
    }

    /// ADR-298 §4: a figure exactly 1% from the stated one passes on either side, and the next
    /// float past it fails. A stated figure that isn't positive, or a NaN, fails.
    #[test]
    fn a_figure_exactly_one_percent_away_passes_and_one_past_fails() {
        let gap = |file: f64, stated: f64| FigureGap {
            figure: CheckedFigure::BurnTime,
            file,
            stated,
        };
        assert!(gap(101.0, 100.0).within_bound());
        assert!(gap(99.0, 100.0).within_bound());
        assert!(gap(100.0, 100.0).within_bound());
        assert!(!gap(101.0_f64.next_up(), 100.0).within_bound());
        assert!(!gap(99.0_f64.next_down(), 100.0).within_bound());
        assert_eq!(gap(101.0, 100.0).relative(), 0.01);
        assert_eq!(gap(99.0, 100.0).relative(), -0.01);
        assert!(!gap(0.0, 0.0).within_bound());
        assert!(!gap(-1.0, -1.0).within_bound());
        assert!(!gap(f64::NAN, 100.0).within_bound());
        assert!(!gap(100.0, f64::NAN).within_bound());
    }

    /// ADR-298 §4 and §6: each checked figure fails the check alone, and the warning names that
    /// figure only, with its signed gap, file against stated. Each case moves one stated figure
    /// of the bundled J760 so its file's figure sits 1.5% to 3% from it.
    #[test]
    fn each_figure_alone_fails_the_check_and_is_named() {
        let catalog = bundled();
        let motor = catalog.find("J760").next().unwrap();
        let curve = &motor.curves[0];
        let thrust = motor
            .thrust_curve(curve, bundled_curve_text(&curve.file).unwrap())
            .unwrap();
        let all = [
            CheckedFigure::TotalImpulse,
            CheckedFigure::AverageThrust,
            CheckedFigure::BurnTime,
        ];
        for (figure, scale, gap) in [
            (CheckedFigure::TotalImpulse, 1.02, "+2.00%"),
            (CheckedFigure::AverageThrust, 0.97, "-3.00%"),
            (CheckedFigure::BurnTime, 1.015, "+1.50%"),
        ] {
            let mut moved = motor.clone();
            let check = &curve.check;
            match figure {
                CheckedFigure::TotalImpulse => {
                    moved.stated.total_impulse_ns = check.total_impulse_ns / scale;
                }
                CheckedFigure::AverageThrust => {
                    moved.stated.average_thrust_n = check.average_thrust_n / scale;
                }
                CheckedFigure::BurnTime => moved.stated.burn_time_s = check.burn_time_s / scale,
            }
            moved.curves[0].check = CurveCheck::of(&thrust, &moved.stated);
            let moved_curve = &moved.curves[0];
            assert!(!moved_curve.check.passes, "{figure:?}");
            let failing = moved.failing_figures(moved_curve);
            assert_eq!(failing.len(), 1, "{figure:?}");
            assert_eq!(failing[0].figure, figure);
            assert!((failing[0].relative() - (scale - 1.0)).abs() < 1e-12);
            let warning = moved.check_warning(moved_curve).unwrap();
            assert_eq!(
                warning,
                format!(
                    "1266J760-19A's curve file {} is more than 1% from ThrustCurve.org's \
                     figures for it: {} {gap}",
                    curve.simfile_id,
                    figure.name()
                )
            );
            for other in all.iter().filter(|other| **other != figure) {
                assert!(!warning.contains(other.name()), "{warning}");
            }
        }
    }

    /// ADR-298 §5: a motor's files rank a passing file first, then RASP before RockSim, each by
    /// who measured it (a certification test, the maker, a user), the index's order within a
    /// rank.
    #[test]
    fn files_rank_passing_first_then_by_format_and_source() {
        let catalog = bundled();
        let base = &catalog.motors[0];
        let file = |id: &str, format, source, passes| {
            let mut curve = base.curves[0].clone();
            curve.simfile_id = id.to_owned();
            curve.format = format;
            curve.source = source;
            curve.check.passes = passes;
            curve
        };
        use CurveFormat::{Rasp, RockSim};
        use CurveSource::{Certification, Manufacturer, User};
        let mut motor = base.clone();
        motor.curves = vec![
            file("user rasp pass", Rasp, User, true),
            file("cert rse fail", RockSim, Certification, false),
            file("cert rasp fail", Rasp, Certification, false),
            file("mfr rse pass", RockSim, Manufacturer, true),
            file("user rasp fail", Rasp, User, false),
            file("mfr rasp pass", Rasp, Manufacturer, true),
            file("user rasp pass 2", Rasp, User, true),
        ];
        let order: Vec<&str> = motor
            .ranked_curves()
            .iter()
            .map(|curve| curve.simfile_id.as_str())
            .collect();
        assert_eq!(
            order,
            [
                "mfr rasp pass",
                "user rasp pass",
                "user rasp pass 2",
                "mfr rse pass",
                "cert rasp fail",
                "user rasp fail",
                "cert rse fail",
            ]
        );
    }

    /// ADR-298 §5: the default is the first ranked file that is bundled. A passing file listed
    /// after a failing one flies; a passing file that isn't bundled is passed over for a failing
    /// one that is, and a motor with no bundled file has no default.
    #[test]
    fn the_default_file_passes_where_a_bundled_one_does() {
        let catalog = bundled();
        let base = &catalog.motors[0];
        let passing = base.curves[0].clone();
        let mut failing = passing.clone();
        failing.simfile_id = "failing".to_owned();
        failing.check.passes = false;
        let mut not_bundled = passing.clone();
        not_bundled.simfile_id = "not bundled".to_owned();
        not_bundled.file = "curves/not-bundled.eng".to_owned();
        let mut motor = base.clone();
        motor.curves = vec![failing.clone(), passing.clone()];
        assert_eq!(motor.default_curve(), Some(&passing));
        motor.curves = vec![not_bundled.clone(), failing.clone()];
        assert_eq!(motor.default_curve(), Some(&failing));
        assert!(motor.bundled_motor().is_ok());
        motor.curves = vec![not_bundled];
        assert_eq!(motor.default_curve(), None);
        assert!(matches!(
            motor.bundled_motor(),
            Err(MotorError::Inconsistent(message)) if message.contains("no bundled curve")
        ));
    }
}
