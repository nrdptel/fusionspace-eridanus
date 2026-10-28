//! An offline motor catalog: motor metadata, thrust curves, and each curve's provenance and
//! license.
//!
//! [`Catalog::bundled`] is the catalog compiled into the crate: ThrustCurve.org's whole solid
//! catalog, captured 2026-10-28, with 1,004 motors and 1,700 curve files; 80 of the motors have no
//! file that reads, and can't fly. Every lookup reads it ([ADR-302][adr-302]). The decision record
//! on solid motors and the first bundled catalog is [ADR-005][adr-005]. Newer curves are fetched
//! and cached by `hpr-net`, the online layer that starts at [M5.1][m5-1].
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
//! [adr-302]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0302-every-motor-by-name.md
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
    /// The snapshot's files the index leaves out, each with its reason.
    #[serde(default)]
    pub left_out: Vec<LeftOut>,
}

/// A snapshot file a catalog leaves out, and why: a file whose uploader restricted it, or one
/// that can't be read as one motor's curve.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LeftOut {
    /// ThrustCurve.org's data-file id.
    pub simfile_id: String,
    /// ThrustCurve.org's id for the motor the file is listed under.
    pub motor_id: String,
    /// Why it is left out.
    pub reason: String,
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
/// credit ([ADR-287][adr-287]). ThrustCurve.org stores them with the motor, from the certifying
/// body or derived from its data or a curve, not worked out from each file; some are printed
/// coarser than 1%. Its *Motor Statistics* page
/// (<https://www.thrustcurve.org/info/motorstats.html>) defines the burn time between the
/// 5%-of-peak crossings.
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
    /// Propellant mass, g, where the record states one. A file isn't checked against it, but
    /// flying one whose header gives more than [`PROPELLANT_SHORT_BOUND`] less warns
    /// ([`CatalogMotor::propellant_warning`]).
    #[serde(default)]
    pub propellant_mass_g: Option<f64>,
}

/// How far below its motor's record a file's propellant mass may be, as a share of the record's,
/// before flying it warns: 5%, the rule a fetched file has had since issue #578 (the reasoning,
/// with its survey, is `hpr_net::on_demand::PROPELLANT_SHORT_BOUND`'s, which is this). A light
/// propellant mass with a right loaded mass flies too heavy a burnout mass and too low an apogee,
/// the flattering side for a waiver; a heavy one isn't warned of.
pub const PROPELLANT_SHORT_BOUND: f64 = 0.05;

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

    /// The signed gap in percent, as the warning and `hpr motors show` print it: two decimals,
    /// and more for a figure that fails, so it never prints as within 1% (`+1.004%`, not
    /// `+1.00%`). A failing gap so close to 1% that its quotient rounds to it prints as `>+1%` or
    /// `<-1%`, and one against a stated figure that isn't positive as `n/a`.
    pub fn percent_text(&self) -> String {
        let percent = 100.0 * self.relative();
        let mut decimals = 2;
        let mut text = format!("{percent:+.decimals$}%");
        if self.within_bound() {
            return text;
        }
        if self.stated.is_nan() || self.stated <= 0.0 {
            return "n/a".to_owned();
        }
        while decimals < 17 && rounds_inside(&text) {
            decimals += 1;
            text = format!("{percent:+.decimals$}%");
        }
        // `within_bound` compares the difference with 1% of the stated figure, and the quotient
        // can round onto 1% when the difference is one unit past it.
        if rounds_inside(&text) {
            let over = if self.file > self.stated {
                ">+1%"
            } else {
                "<-1%"
            };
            return over.to_owned();
        }
        text
    }

    /// Whether the file's figure is within [`CHECK_BOUND`] of the stated one,
    /// `|file − stated| ≤ 0.01 × stated`. A stated figure that isn't positive, or a NaN either
    /// side, fails.
    pub fn within_bound(&self) -> bool {
        self.stated > 0.0 && (self.file - self.stated).abs() <= CHECK_BOUND * self.stated
    }
}

/// Whether a signed percentage as text (`+1.00%`) reads as within 1%.
fn rounds_inside(text: &str) -> bool {
    text.trim_start_matches(['+', '-'])
        .trim_end_matches('%')
        .parse::<f64>()
        .is_ok_and(|magnitude| magnitude <= 100.0 * CHECK_BOUND)
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
    /// Whether a curve under this license is bundled: every one but a file its uploader marked
    /// "other", restricted ([ADR-288][adr-288] §1). A file's measured points are used as facts,
    /// as ThrustCurve.org's records are ([ADR-287]); free-usage files and files with no license
    /// go in like the rest, each with its label kept beside it ([ADR-288][adr-288] §2).
    ///
    /// [ADR-287]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0287-thrustcurve-orgs-motor-records-used-as-published-facts.md
    ///
    /// [adr-288]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0288-every-solid-motor-ships-in-the-package.md
    pub fn is_bundleable(self) -> bool {
        self != Self::Other
    }
}

impl Catalog {
    /// The catalog compiled into this crate, which `hpr motors`, `hpr sim --motor` and a design's
    /// motors are looked up in: ThrustCurve.org's whole solid catalog ([M5.7][m5-7]), every solid
    /// motor in the snapshot and every file but a restricted one, each with its stored check, and
    /// each file left out with its reason ([ADR-301][adr-301]).
    ///
    /// [m5-7]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m5-7
    /// [adr-301]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0301-the-whole-catalog-in-two-increments.md
    ///
    /// # Errors
    ///
    /// [`MotorError::Catalog`] if the index doesn't deserialize, which a test rules out.
    pub fn bundled() -> Result<&'static Self, MotorError> {
        // Read once and shared: the index of every solid motor is a few megabytes of JSON.
        static BUNDLED: std::sync::OnceLock<Result<Catalog, MotorError>> =
            std::sync::OnceLock::new();
        BUNDLED
            .get_or_init(|| Self::from_json(bundled::CATALOG_JSON))
            .as_ref()
            .map_err(Clone::clone)
    }

    /// Reads a catalog index.
    ///
    /// # Errors
    ///
    /// [`MotorError::Catalog`] if the text isn't a catalog index, or a curve's stored
    /// [`CurveCheck::passes`] isn't what its stored figures give against its motor's stated ones:
    /// the flag ranks the files, so one that says a failing file passes would fly it silently.
    pub fn from_json(text: &str) -> Result<Self, MotorError> {
        let catalog: Self =
            serde_json::from_str(text).map_err(|error| MotorError::Catalog(error.to_string()))?;
        for motor in &catalog.motors {
            for curve in &motor.curves {
                let passes = curve
                    .check
                    .gaps(&motor.stated)
                    .iter()
                    .all(FigureGap::within_bound);
                if passes != curve.check.passes {
                    return Err(MotorError::Catalog(format!(
                        "{}'s curve {} stores passes: {}, but its figures give {passes}",
                        motor.designation, curve.simfile_id, curve.check.passes
                    )));
                }
            }
        }
        Ok(catalog)
    }

    /// The motors `name` names ([ADR-302][adr-302]): those whose designation or common name
    /// matches it, ignoring ASCII case, spaces and hyphens (`j760`, `1266J760-19A`, `K 940`).
    ///
    /// - A name may start with a maker, its full name or short one and a space, such as
    ///   `AeroTech H128W` or `Estes Industries C6`: then only that maker's motors match.
    /// - Of one maker's matches, a motor whose designation is the name is the one it names: `D5`
    ///   is Quest's D5, not the D5-P whose common name is D5. Matches of different makers all
    ///   stay, whether by designation or common name: `G80` finds Roadrunner's and Estes' G80 and
    ///   AeroTech's G80T, so a caller that wants one refuses it as ambiguous rather than flying a
    ///   maker the name didn't say. Each maker's short name and designation names one alone, as
    ///   no maker has two motors of one designation (a test holds it).
    ///
    /// [adr-302]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0302-every-motor-by-name.md
    pub fn find<'a>(&'a self, name: &str) -> impl Iterator<Item = &'a CatalogMotor> + 'a {
        let (maker, rest) = self.maker_and_name(name);
        self.named(rest, |motor| {
            maker.is_none_or(|maker| maker == motor.manufacturer)
        })
    }

    /// As [`Catalog::find`], among the motors of the maker `maker` names, by its full or short
    /// name with only its ASCII letters and digits compared, ignoring case (`Cesaroni
    /// Technology Inc.` is Cesaroni Technology's, `AEROTECH` AeroTech's): the lookup a design
    /// file's motor, which names its maker apart, takes.
    pub fn find_by_maker<'a>(
        &'a self,
        maker: &str,
        name: &str,
    ) -> impl Iterator<Item = &'a CatalogMotor> + 'a {
        let maker = maker_key(maker);
        self.named(name, move |motor| {
            maker_key(&motor.manufacturer) == maker
                || maker_key(&motor.manufacturer_abbrev) == maker
        })
    }

    /// As [`Catalog::find_by_maker`], for a design file's motor, which may also give its curve's
    /// `digest` and its case's `diameter_mm`. Cesaroni's 54 mm `1016J360-15A` has the common
    /// name `J360`, which is the designation of a 38 mm Cesaroni motor; a file that saved the
    /// first as `J360` must not fly the second, about 20% short on impulse:
    ///
    /// - where `digest` maps to a bundled file ([`crate::openrocket_curves::file_for`]) of
    ///   exactly one motor the name matches by designation or common name, that motor alone;
    /// - else, where every motor [`Catalog::find_by_maker`] gives is more than 1 mm from
    ///   `diameter_mm` and another the name matches is within it, every match, so the caller
    ///   refuses the motor as ambiguous rather than flying a case the design doesn't hold.
    pub fn find_for_design<'a>(
        &'a self,
        maker: &str,
        name: &str,
        digest: Option<&str>,
        diameter_mm: Option<f64>,
    ) -> Vec<&'a CatalogMotor> {
        let maker = maker_key(maker);
        let of = |motor: &CatalogMotor| {
            maker_key(&motor.manufacturer) == maker
                || maker_key(&motor.manufacturer_abbrev) == maker
        };
        let all = self.matching(name, of);
        if let Some(id) = digest.and_then(crate::openrocket_curves::file_for) {
            let owners: Vec<&CatalogMotor> = all
                .iter()
                .copied()
                .filter(|motor| motor.curves.iter().any(|curve| curve.simfile_id == id))
                .collect();
            if let [owner] = owners[..] {
                return vec![owner];
            }
        }
        let picked = prefer_designations(all.clone(), name);
        let fits = |motor: &&CatalogMotor| {
            diameter_mm.is_some_and(|mm| (motor.diameter_mm - mm).abs() <= 1.0)
        };
        let misfit = diameter_mm.is_some() && !picked.iter().any(fits);
        if misfit
            && all
                .iter()
                .any(|motor| fits(motor) && !picked.iter().any(|kept| std::ptr::eq(*kept, *motor)))
        {
            return all;
        }
        picked
    }

    /// The motors `of` keeps whose designation or common name is `name`, narrowed as
    /// [`Catalog::find`] says: within one maker, those whose designation is the name if any is.
    fn named<'a>(
        &'a self,
        name: &str,
        of: impl Fn(&CatalogMotor) -> bool,
    ) -> std::vec::IntoIter<&'a CatalogMotor> {
        prefer_designations(self.matching(name, of), name).into_iter()
    }

    /// Every motor `of` keeps whose designation or common name is `name`.
    fn matching(&self, name: &str, of: impl Fn(&CatalogMotor) -> bool) -> Vec<&CatalogMotor> {
        let key = normalize(name);
        self.motors
            .iter()
            .filter(|motor| {
                of(motor)
                    && (normalized_is(&motor.designation, &key)
                        || normalized_is(&motor.common_name, &key))
            })
            .collect()
    }

    /// The maker `name` starts with, by full or short name, followed by a space, and the rest of
    /// the name; or no maker and the whole name. The longest maker's name that fits is taken.
    fn maker_and_name<'a>(&'a self, name: &'a str) -> (Option<&'a str>, &'a str) {
        let name = name.trim();
        let mut best: Option<(&str, &str, usize)> = None;
        for motor in &self.motors {
            for said in [&motor.manufacturer, &motor.manufacturer_abbrev] {
                let Some(head) = name.get(..said.len()) else {
                    continue;
                };
                let rest = &name[said.len()..];
                if head.eq_ignore_ascii_case(said)
                    && rest.starts_with(char::is_whitespace)
                    && best.is_none_or(|(_, _, length)| said.len() > length)
                {
                    best = Some((motor.manufacturer.as_str(), rest.trim_start(), said.len()));
                }
            }
        }
        best.map_or((None, name), |(maker, rest, _)| (Some(maker), rest))
    }

    /// The name that selects `motor` alone through [`Catalog::find`]: its designation where that
    /// finds it alone, else its maker's short name and its designation, such as `Estes C6`.
    pub fn selector(&self, motor: &CatalogMotor) -> String {
        let alone = self.find(&motor.designation).count() == 1;
        if alone {
            motor.designation.clone()
        } else {
            format!("{} {}", motor.manufacturer_abbrev, motor.designation)
        }
    }
}

/// Of `matches` for `name`, each maker's motors whose designation is the name where that maker
/// has one, and the rest of its matches where it hasn't: a name never picks one maker over
/// another.
fn prefer_designations<'a>(matches: Vec<&'a CatalogMotor>, name: &str) -> Vec<&'a CatalogMotor> {
    let key = normalize(name);
    let exact = |motor: &CatalogMotor| normalize(&motor.designation) == key;
    let makers_with_exact: Vec<&str> = matches
        .iter()
        .filter(|motor| exact(motor))
        .map(|motor| motor.manufacturer.as_str())
        .collect();
    matches
        .into_iter()
        .filter(|motor| exact(motor) || !makers_with_exact.contains(&motor.manufacturer.as_str()))
        .collect()
}

/// A maker's name with only its ASCII letters and digits, lowercase, and a trailing company
/// suffix (`Inc`, `Ltd`, `LLC`) dropped: `Cesaroni Technology Inc.` compares as Cesaroni
/// Technology.
fn maker_key(text: &str) -> String {
    let key: String = text
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .map(|c| c.to_ascii_lowercase())
        .collect();
    ["inc", "ltd", "llc"]
        .iter()
        .find_map(|suffix| key.strip_suffix(suffix).filter(|rest| !rest.is_empty()))
        .map_or_else(|| key.clone(), str::to_owned)
}

/// Whether `normalize(name)` is `key`, without building it: a lookup compares every motor's two
/// names, and `hpr motors list` looks each listed motor's selector up, so building them made the
/// listing quadratic in allocations (51.7 ms for 1,004 motors; 12.6 ms without).
fn normalized_is(name: &str, key: &str) -> bool {
    name.chars()
        .filter(|c| !c.is_whitespace() && *c != '-')
        .map(|c| c.to_ascii_lowercase())
        .eq(key.chars())
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
        let (curve, text) = self.default_curve_text().ok_or_else(|| {
            MotorError::Inconsistent(format!("{} has no bundled curve", self.designation))
        })?;
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
        self.default_curve_text().map(|(curve, _)| curve)
    }

    /// The curve a design file's motor flies, with its bundled file's text: the file OpenRocket's
    /// curve for `digest` maps to ([`crate::openrocket_curves::file_for`]) where it is one of this
    /// motor's bundled files ([ADR-288][adr-288] §3), else [`CatalogMotor::default_curve`].
    ///
    /// [adr-288]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0288-every-solid-motor-ships-in-the-package.md
    pub fn curve_for_digest(&self, digest: Option<&str>) -> Option<(&CatalogCurve, &'static str)> {
        let mapped = digest.and_then(crate::openrocket_curves::file_for);
        mapped
            .and_then(|id| {
                let curve = self.curves.iter().find(|curve| curve.simfile_id == id)?;
                Some((curve, bundled_curve_text(&curve.file)?))
            })
            .or_else(|| self.default_curve_text())
    }

    /// [`CatalogMotor::default_curve`] with its bundled file's text.
    pub fn default_curve_text(&self) -> Option<(&CatalogCurve, &'static str)> {
        self.ranked_curves()
            .into_iter()
            .find_map(|curve| Some((curve, bundled_curve_text(&curve.file)?)))
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

    /// The warning for flying `curve` when its header's propellant mass is more than
    /// [`PROPELLANT_SHORT_BOUND`] below the record's, naming both masses and the gap; `None` when
    /// it isn't (a file exactly at the bound draws none), when the record states no propellant
    /// mass, or when the file isn't bundled or doesn't read.
    pub fn propellant_warning(&self, curve: &CatalogCurve) -> Option<String> {
        let record_g = self
            .stated
            .propellant_mass_g
            .filter(|g| g.is_finite() && *g > 0.0)?;
        let text = bundled_curve_text(&curve.file)?;
        let (_, propellant_kg, _) = read_curve_file(curve, text).ok()?;
        let file_g = propellant_kg * 1000.0;
        // Compared as masses, so a file exactly at the bound doesn't warn; a NaN warns.
        if file_g >= record_g * (1.0 - PROPELLANT_SHORT_BOUND) {
            return None;
        }
        Some(format!(
            "{} {}'s curve file {} gives {file_g:.1} g of propellant, {:+.2}% from \
             ThrustCurve.org's record of {record_g:.1} g. If the file's loaded mass is right, it \
             flies too heavy a burnout mass and too low an apogee",
            self.manufacturer_abbrev,
            self.designation,
            curve.simfile_id,
            (file_g / record_g - 1.0) * 100.0
        ))
    }

    /// Every warning flying `curve` draws: its failing check ([`CatalogMotor::check_warning`])
    /// and a light propellant mass ([`CatalogMotor::propellant_warning`]), in that order.
    pub fn flight_warnings(&self, curve: &CatalogCurve) -> Vec<String> {
        self.check_warning(curve)
            .into_iter()
            .chain(self.propellant_warning(curve))
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
            .map(|gap| format!("{} {}", gap.figure.name(), gap.percent_text()))
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

    /// The bundled catalog, which these tests hold.
    fn bundled() -> &'static Catalog {
        Catalog::bundled().unwrap()
    }

    /// M5.7a's 32 files, as ThrustCurve.org's statistics fixture lists them, are each indexed
    /// under their motor and pass their check, so none of the oracle-held files warns.
    #[test]
    fn m5_7a_s_32_files_are_indexed_and_pass() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../../validation/fixtures/motor/thrustcurve-analyze-stats.json"
        ))
        .unwrap();
        let files: Vec<&str> = fixture["curves"]
            .as_array()
            .unwrap()
            .iter()
            .map(|c| c["file"].as_str().unwrap())
            .collect();
        assert_eq!(files.len(), 32);
        for file in files {
            let (motor, curve) = bundled()
                .motors
                .iter()
                .find_map(|m| m.curves.iter().find(|c| c.file == file).map(|c| (m, c)))
                .unwrap_or_else(|| panic!("{file} isn't indexed"));
            assert!(curve.check.passes, "{file}");
            assert_eq!(motor.check_warning(curve), None, "{file}");
        }
    }

    /// Loft lesson L41: Loft's bundled curves were 45 public domain, 38 with no license, 22
    /// unknown and 3 "free" (which can include GPL), with no label kept. Here every bundled file
    /// carries ThrustCurve.org's label for it, and none is one its uploader restricted
    /// ([ADR-288][adr-288] §1–2 takes in the rest).
    ///
    /// [adr-288]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0288-every-solid-motor-ships-in-the-package.md
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
        assert!(!CurveLicense::Other.is_bundleable());
        assert!(CurveLicense::Free.is_bundleable());
        assert!(CurveLicense::Unknown.is_bundleable());
        assert!(CurveLicense::PublicDomain.is_bundleable());
    }

    /// Loft lesson L42: Loft's impulse checks allowed −8% to +8%, and a mis-sourced curve flew
    /// about 26% high until caught. Every figure the bundled index lists for a motor is its
    /// default file's own, bit for bit: `cargo xtask motor-catalog` works them out with the
    /// methods the simulator flies (#295), so what `hpr motors` lists is what flies. Each file the
    /// oracle fixture holds is also within 1% of what ThrustCurve.org's own code works out from it
    /// (M1.3's bound; [`every_bundled_curve_matches_thrustcurve_statistics_code`] holds it to
    /// 1e-12): M5.7a's 32 files until M5.7c runs the code over every file stored as passing
    /// ([ADR-301][adr-301]).
    ///
    /// [adr-301]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0301-the-whole-catalog-in-two-increments.md
    #[test]
    fn every_bundled_curve_impulse_within_1pct_of_thrustcurve() {
        let oracle: AnalyzeOracle = serde_json::from_str(include_str!(
            "../../../validation/fixtures/motor/thrustcurve-analyze-stats.json"
        ))
        .unwrap();
        let catalog = bundled();
        let mut checked = 0;
        let mut listed = 0;
        for motor in &catalog.motors {
            for curve in &motor.curves {
                if motor.default_curve() != Some(curve)
                    && !oracle.curves.iter().any(|c| c.file == curve.file)
                {
                    continue;
                }
                let text = bundled_curve_text(&curve.file).unwrap();
                let thrust = motor.thrust_curve(curve, text).unwrap();
                let theirs = oracle.curves.iter().find(|c| c.file == curve.file);
                for (stated, computed, by_thrustcurve, what) in [
                    (
                        motor.total_impulse_ns,
                        thrust.total_impulse_ns(),
                        theirs.map(|t| t.total_impulse_ns),
                        "total impulse",
                    ),
                    (
                        motor.average_thrust_n,
                        thrust.average_thrust_n(),
                        theirs.map(|t| t.average_thrust_n),
                        "average thrust",
                    ),
                    (
                        motor.burn_time_s,
                        thrust.burn_time_s(),
                        theirs.map(|t| t.burn_time_s),
                        "burn time",
                    ),
                    (
                        motor.max_thrust_n.unwrap(),
                        thrust.peak_thrust_n(),
                        theirs.map(|t| t.max_thrust_n),
                        "peak thrust",
                    ),
                ] {
                    if motor.default_curve() == Some(curve) {
                        assert_eq!(
                            stated.to_bits(),
                            computed.to_bits(),
                            "{} ({}): the index's {what} {stated} is not its curve's {computed}",
                            motor.designation,
                            curve.simfile_id,
                        );
                    }
                    let Some(by_thrustcurve) = by_thrustcurve else {
                        continue;
                    };
                    let error = (computed - by_thrustcurve) / by_thrustcurve;
                    assert!(
                        error.abs() <= 0.01,
                        "{} ({}): {what} {stated} against ThrustCurve's code's {by_thrustcurve} \
                         ({:+.2}%)",
                        motor.designation,
                        curve.simfile_id,
                        100.0 * error
                    );
                }
                if motor.default_curve() == Some(curve) {
                    assert_eq!(
                        ImpulseClass::from_total_impulse(motor.total_impulse_ns).unwrap(),
                        motor.impulse_class,
                        "{}",
                        motor.designation
                    );
                    listed += 1;
                }
                checked += usize::from(theirs.is_some());
            }
        }
        assert_eq!(checked, oracle.curves.len());
        assert_eq!(checked, 32);
        assert_eq!(
            listed,
            catalog
                .motors
                .iter()
                .filter(|m| !m.curves.is_empty())
                .count()
        );
    }

    /// Every bundled entry's size, masses and delays are its default curve file header's (#295):
    /// the masses to the microgram, as `cargo xtask motor-catalog` writes a `.eng` header's
    /// kilograms in grams. A motor with no file that reads lists its record's (`xtask`'s
    /// `every_record_is_a_motor_and_every_file_is_indexed_or_left_out`).
    #[test]
    fn every_bundled_size_mass_and_delay_is_the_header_s() {
        let catalog = bundled();
        for motor in &catalog.motors {
            let Some(curve) = motor.default_curve() else {
                assert!(motor.curves.is_empty(), "{}", motor.designation);
                continue;
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
        assert!(j760.iter().any(|m| m.designation == "1266J760-19A"));
        assert!(j760.iter().all(|m| m.common_name == "J760"));
        assert_eq!(catalog.find("1266J760-19A").count(), 1);
        let plugged = catalog.find("L3200").next().unwrap();
        assert_eq!(plugged.delays().delays, [Delay::Plugged]);
        assert!(catalog.find("no such motor").next().is_none());
        // ADR-302: a maker's name narrows the match, and within one maker a designation beats
        // a common name; across makers both stay, so `C6` finds Southern Cross's SCR-C6 too.
        let c6: Vec<_> = catalog.find("C6").collect();
        assert!(c6.len() > 1 && c6.iter().any(|m| m.designation == "SCR-C6"));
        let estes: Vec<_> = catalog.find("Estes C6").collect();
        assert_eq!(estes.len(), 1);
        assert_eq!(estes[0].manufacturer_abbrev, "Estes");
        assert_eq!(catalog.find("estes industries c6").count(), 1);
        assert_eq!(catalog.find("EstesC6").count(), 0);
        let d5: Vec<_> = catalog.find("D5").collect();
        assert_eq!(d5.len(), 1);
        assert_eq!(d5[0].designation, "D5");
        assert!(catalog.find("D5-P").all(|m| m.designation == "D5-P"));
        assert_eq!(catalog.find("Quest no such motor").count(), 0);
        // Every motor's selector finds it alone, and no maker has two motors of one designation.
        for motor in &catalog.motors {
            let selector = catalog.selector(motor);
            let found: Vec<_> = catalog.find(&selector).collect();
            assert_eq!(found.len(), 1, "{selector}");
            assert_eq!(found[0].motor_id, motor.motor_id, "{selector}");
        }
        // Every bundled motor with a file that reads builds; one with none can't fly.
        for motor in &catalog.motors {
            if motor.curves.is_empty() {
                assert!(motor.bundled_motor().is_err(), "{}", motor.designation);
                continue;
            }
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
        // The index's first public-domain file, the one `with` relabels.
        let first_pd = bundled()
            .motors
            .iter()
            .flat_map(|m| &m.curves)
            .find(|c| c.license == CurveLicense::PublicDomain)
            .unwrap()
            .simfile_id
            .clone();
        let first_license = |catalog: Catalog| {
            catalog
                .motors
                .iter()
                .flat_map(|m| &m.curves)
                .find(|c| c.simfile_id == first_pd)
                .unwrap()
                .license
        };
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
        let first_su = bundled()
            .motors
            .iter()
            .position(|m| m.motor_type == MotorType::SingleUse)
            .unwrap();
        assert_eq!(hybrid.motors[first_su].motor_type, MotorType::Hybrid);
        assert!(hybrid.motors[first_su].bundled_motor().is_err());
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
        // M5.7a's 32 files; M5.7c runs the code over every file stored as passing (ADR-301).
        assert_eq!(oracle.curves.len(), 32);
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
        // ADR-066's check holds M5.7a's 32 files, and the whole catalog doesn't widen it (ADR-288).
        assert_eq!(record.curves.len(), 32);
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
        assert_eq!(files.len(), record.curves.len(), "a file twice or missing");
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
    /// bit for bit, over every file of the whole catalog. A failing file is stored with its
    /// failing check, and its warning names it.
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
                // A failing file is stored with its check (ADR-298 §2) and warned of when flown.
                assert_eq!(
                    curve.check.passes,
                    motor.failing_figures(curve).is_empty(),
                    "{name}"
                );
                match motor.check_warning(curve) {
                    None => assert!(curve.check.passes, "{name}"),
                    Some(warning) => {
                        assert!(!curve.check.passes, "{name}");
                        assert!(warning.contains(&curve.simfile_id), "{name}: {warning}");
                    }
                }
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
        // A failing figure never prints as within 1%; a passing one keeps two decimals.
        assert_eq!(gap(101.004, 100.0).percent_text(), "+1.004%");
        assert_eq!(gap(98.9996, 100.0).percent_text(), "-1.0004%");
        assert_eq!(
            gap(101.0_f64.next_up(), 100.0).percent_text(),
            "+1.00000000000001%"
        );
        assert_eq!(gap(101.0, 100.0).percent_text(), "+1.00%");
        assert_eq!(gap(100.996, 100.0).percent_text(), "+1.00%");
        assert_eq!(gap(102.0, 100.0).percent_text(), "+2.00%");
        // One unit past 1%, the quotient needs 16 decimals to read as past it.
        let past = gap(242.70152480324737, 240.2985394091558);
        assert!(!past.within_bound());
        assert_eq!(past.percent_text(), "+1.0000000000000002%");
        assert_eq!(gap(-1.0, -1.0).percent_text(), "n/a");
    }

    /// A stored pass its figures contradict is refused when the index is read: the flag ranks the
    /// files, so a wrong one would fly a failing file silently.
    #[test]
    fn a_stored_pass_its_figures_contradict_is_refused() {
        let flipped = bundled::CATALOG_JSON.replacen("\"passes\": true", "\"passes\": false", 1);
        assert!(matches!(
            Catalog::from_json(&flipped),
            Err(MotorError::Catalog(message)) if message.contains("stores passes: false")
        ));
    }

    /// A failing figure never prints as within 1%: over 20,000 stated figures from 0.1 to about
    /// 40,000, the first few floats past 1% on either side each print past it.
    #[test]
    fn a_failing_gap_never_prints_inside_one_percent() {
        let mut checked = 0;
        for i in 0..20_000 {
            let stated = 0.1 + f64::from(i) * 1.999_37;
            for (start, step) in [(stated * 1.01, 1.0), (stated * 0.99, -1.0)] {
                let mut file: f64 = start;
                for _ in 0..4 {
                    let gap = FigureGap {
                        figure: CheckedFigure::TotalImpulse,
                        file,
                        stated,
                    };
                    if !gap.within_bound() {
                        let text = gap.percent_text();
                        assert!(!rounds_inside(&text), "{file} against {stated}: {text}");
                        checked += 1;
                    }
                    file = if step > 0.0 {
                        file.next_up()
                    } else {
                        file.next_down()
                    };
                }
            }
        }
        assert!(checked > 100_000, "{checked}");
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

    /// ADR-298 §5's default on the whole catalog: every motor with a bundled file has a default,
    /// and it passes the 1% check wherever one of its files does (M5.7b2).
    #[test]
    fn every_motor_s_default_passes_where_one_of_its_files_does() {
        let catalog = bundled();
        let mut passing = 0;
        for motor in &catalog.motors {
            let default = motor.default_curve();
            assert_eq!(
                default.is_some(),
                !motor.curves.is_empty(),
                "{}",
                motor.designation
            );
            if motor.curves.iter().any(|c| c.check.passes) {
                passing += 1;
                assert!(
                    default.is_some_and(|c| c.check.passes),
                    "{}",
                    motor.designation
                );
            }
        }
        assert_eq!(passing, 239);
    }

    /// A `.ork` motor's digest picks the file OpenRocket's curve maps to, ahead of the default,
    /// only when that file is one of the motor's own (ADR-288 §3, ADR-302 §4).
    #[test]
    fn a_digest_takes_its_own_file_first() {
        let catalog = bundled();
        let table: serde_json::Value =
            serde_json::from_str(include_str!("../data/openrocket-curves.json")).unwrap();
        // Every digest in the table whose file belongs to a motor with another default.
        let mut moved = 0;
        for entry in table["files"].as_array().unwrap() {
            let digest = entry["digest"].as_str().unwrap();
            let mapped = crate::openrocket_curves::file_for(digest).unwrap();
            let Some(motor) = catalog
                .motors
                .iter()
                .find(|m| m.curves.iter().any(|c| c.simfile_id == mapped))
            else {
                continue;
            };
            let default = &motor.default_curve().unwrap().simfile_id;
            let picked = |digest| motor.curve_for_digest(digest).map(|(c, _)| &c.simfile_id);
            assert_eq!(picked(Some(digest)), Some(&mapped), "{digest}");
            assert_eq!(picked(None), Some(default));
            assert_eq!(
                picked(Some("00000000000000000000000000000000")),
                Some(default)
            );
            if *default != mapped {
                moved += 1;
                // Another motor's digest leaves the default standing.
                let other = table["files"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .filter_map(|e| e["digest"].as_str())
                    .find(|d| {
                        crate::openrocket_curves::file_for(d)
                            .is_some_and(|f| motor.curves.iter().all(|c| c.simfile_id != f))
                    })
                    .unwrap();
                assert_eq!(picked(Some(other)), Some(default));
            }
        }
        assert!(
            moved > 0,
            "no digest in the table picks a file other than its default"
        );
    }

    /// ADR-302's light-propellant warning errs toward warning, the side a light file flatters:
    /// a file more than 5% under the record warns with its negative gap, one at 5% or less, or
    /// heavier, doesn't, and a record with no mass (none, zero or not finite) gives none.
    #[test]
    fn a_light_propellant_file_warns_and_only_then() {
        let catalog = bundled();
        let (base, curve, file_g) = catalog
            .motors
            .iter()
            .find_map(|motor| {
                let (curve, text) = motor.default_curve_text()?;
                let (_, propellant_kg, _) = read_curve_file(curve, text).ok()?;
                (propellant_kg > 0.0).then(|| (motor, curve.clone(), propellant_kg * 1000.0))
            })
            .unwrap();
        let warning = |record_g: Option<f64>| {
            let mut motor = base.clone();
            motor.stated.propellant_mass_g = record_g;
            motor.propellant_warning(&curve)
        };
        let light = warning(Some(file_g / 0.949)).expect("5.1% light warns");
        assert!(light.contains("-5.10%"), "{light}");
        assert!(light.contains(&curve.simfile_id), "{light}");
        assert_eq!(warning(Some(file_g / 0.951)), None, "4.9% light");
        assert_eq!(warning(Some(file_g)), None, "equal");
        assert_eq!(warning(Some(file_g * 0.5)), None, "heavier");
        for record in [None, Some(0.0), Some(f64::NAN), Some(f64::INFINITY)] {
            assert_eq!(warning(record), None, "{record:?}");
        }
        // Both of a flight's warnings come back, the check's first.
        let mut motor = base.clone();
        motor.stated.propellant_mass_g = Some(file_g / 0.9);
        let mut failing = curve.clone();
        failing.check.passes = false;
        let warnings = motor.flight_warnings(&failing);
        assert_eq!(warnings.len(), 2, "{warnings:?}");
        assert!(warnings[0].contains("is more than 1% from"), "{warnings:?}");
        assert!(warnings[1].contains("of propellant"), "{warnings:?}");
    }

    /// ADR-302: a name never picks one maker's motor over another's. Each motor is found by each
    /// of its names, unless its own maker has another motor whose designation is that name; and
    /// a name several makers use finds each of them (`G80`, `J300`).
    #[test]
    fn a_name_never_picks_one_maker_over_another() {
        let catalog = bundled();
        for motor in &catalog.motors {
            for name in [&motor.designation, &motor.common_name] {
                let shadowed = normalize(name) != normalize(&motor.designation)
                    && catalog.motors.iter().any(|other| {
                        other.manufacturer == motor.manufacturer
                            && normalize(&other.designation) == normalize(name)
                    });
                let found = catalog.find(name).any(|other| std::ptr::eq(other, motor));
                assert_eq!(found, !shadowed, "{} by {name}", motor.motor_id);
            }
        }
        for name in ["G80", "J300"] {
            let makers: std::collections::BTreeSet<&str> = catalog
                .find(name)
                .map(|motor| motor.manufacturer.as_str())
                .collect();
            assert!(makers.len() > 1, "{name}: {makers:?}");
        }
    }

    /// `normalized_is` answers as comparing `normalize`'s strings does, on every pair of names of
    /// the catalog's first 200 motors.
    #[test]
    fn normalized_is_is_normalize_compared() {
        let catalog = bundled();
        let names: Vec<&str> = catalog
            .motors
            .iter()
            .take(200)
            .flat_map(|motor| [motor.designation.as_str(), motor.common_name.as_str()])
            .chain(["", "J 760", "j-760", "1266J760-19A", "ÄÖ", "K\u{a0}940"])
            .collect();
        for name in &names {
            for other in &names {
                let key = normalize(other);
                assert_eq!(normalized_is(name, &key), normalize(name) == key, "{name} {other}");
            }
        }
    }

    /// A design file's `J360` from Cesaroni is the 38 mm motor of that designation where the
    /// design's case is 38 mm or not given, and is refused (both motors given) where the case is
    /// 54 mm, the size of the motor whose common name is `J360`; a maker's company suffix is
    /// ignored.
    #[test]
    fn a_design_s_case_size_keeps_a_common_name_from_flying_another_motor() {
        let catalog = bundled();
        let found = |maker: &str, diameter_mm: Option<f64>| -> Vec<(String, f64)> {
            catalog
                .find_for_design(maker, "J360", None, diameter_mm)
                .into_iter()
                .map(|motor| (motor.designation.clone(), motor.diameter_mm))
                .collect()
        };
        let small = vec![("J360".to_owned(), 38.0)];
        assert_eq!(found("Cesaroni Technology", None), small);
        assert_eq!(found("Cesaroni Technology", Some(38.4)), small);
        assert_eq!(found("Cesaroni Technology Inc.", Some(38.0)), small);
        assert_eq!(found("Cesaroni", Some(54.0)).len(), 2);
        assert_eq!(
            catalog
                .find_by_maker("Cesaroni Technology Inc.", "J360")
                .count(),
            1
        );
    }

    /// ADR-298 §5: the default is the first ranked file that is bundled. A passing file listed
    /// after a failing one flies; a passing file that isn't bundled is passed over for a failing
    /// one that is, and a motor with no bundled file has no default.
    #[test]
    fn the_default_file_passes_where_a_bundled_one_does() {
        let catalog = bundled();
        let base = catalog
            .motors
            .iter()
            .find(|m| m.curves.first().is_some_and(|c| c.check.passes))
            .unwrap();
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
