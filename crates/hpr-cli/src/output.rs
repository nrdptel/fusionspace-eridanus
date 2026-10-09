//! The types `--json` prints: one per command's output, and [`ErrorDocument`] for a failure.
//!
//! They belong to this crate, not the libraries', so that the published schemas change only when
//! the command's output does. Each derives [`JsonSchema`]; [`schemas`] generates the documents
//! `cargo xtask cli` writes to `schema/cli/`, and the tests check every output against them.
//!
//! Units are SI and named in the field, except where a motor's catalog states millimeters or
//! grams, as its field names say.
//!
//! Every document also opens with a `tool` object, [`Tool`]: the program that wrote it, its
//! version and its designation, as a drawing's title block names them (ADR-164).

use hpr::hpr_core::tool;
use schemars::JsonSchema;
use serde::Serialize;

/// The program that wrote a document or a file: the title block every `--json` document, every
/// file `hpr` writes and `hpr --version` carry (ADR-164; the product system's `data.md`: "Every
/// export carries the designation and version of the tool that made it").
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, JsonSchema)]
pub struct Tool {
    /// The program: `FusionSpace HPR`.
    pub name: &'static str,
    /// Its version, such as `0.1.0`.
    pub version: &'static str,
    /// Its designation in the FusionSpace product system: `FS-ACHERNAR · SW · TOOL 001`.
    pub designation: &'static str,
}

/// FusionSpace HPR, at this build's version ([`hpr::hpr_core::tool`]).
pub const TOOL: Tool = Tool {
    name: tool::NAME,
    version: tool::VERSION,
    designation: tool::DESIGNATION,
};

/// A document as `--json` prints it: [`TOOL`] first, then the command's own fields.
#[derive(Debug, Serialize)]
pub(crate) struct Stamped<'a, T> {
    /// The program that wrote it.
    pub(crate) tool: Tool,
    /// The command's output.
    #[serde(flatten)]
    pub(crate) document: &'a T,
}

/// `hpr motors list`: the bundled catalog's motors, with the figures their curve files give.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
pub struct MotorList {
    /// Where the catalog comes from.
    pub catalog: CatalogInfo,
    /// The motors that pass the filters, in catalog order (by impulse class).
    pub motors: Vec<ListedMotor>,
}

/// Where the bundled catalog comes from.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
pub struct CatalogInfo {
    /// The source, such as `ThrustCurve.org data files marked public domain`.
    pub source: String,
    /// The date the curve files were downloaded, `YYYY-MM-DD`.
    pub captured: String,
    /// Which motors were taken, and why.
    pub selection: String,
}

/// One motor in the catalog: size and delays from its curve file's header, impulse, thrust and burn
/// time worked out from its curve, as `hpr motors show` and the simulator work them out.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
pub struct ListedMotor {
    /// The manufacturer's full designation, such as `1266J760-19A`.
    pub designation: String,
    /// The common name, such as `J760`.
    pub common_name: String,
    /// The manufacturer.
    pub manufacturer: String,
    /// The manufacturer's abbreviation, such as `CTI`.
    pub manufacturer_abbrev: String,
    /// The impulse class, such as `J`.
    pub impulse_class: String,
    /// `single_use`, `reload` or `hybrid`.
    pub motor_type: MotorKind,
    /// Casing diameter, mm.
    pub diameter_mm: f64,
    /// Casing length, mm.
    pub length_mm: f64,
    /// Total impulse, N·s, from the curve.
    pub total_impulse_ns: f64,
    /// Average thrust, N, from the curve: total impulse over the burn time.
    pub average_thrust_n: f64,
    /// Burn time, s, from the curve, by NFPA 1125.
    pub burn_time_s: f64,
    /// The delays as the curve file's header writes them, such as `6,8,10`, `4-6` or `P`
    /// (plugged).
    pub delays: Option<String>,
}

/// Whether a motor is used once or reloaded into a reusable case.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MotorKind {
    /// A single-use motor.
    SingleUse,
    /// A reload for a reusable case.
    Reload,
    /// A hybrid: listed, but HPR Sim models solid motors only.
    Hybrid,
}

/// `hpr motors search`: motor.fusionspace.co's motors that pass the filters, with their stock and
/// prices.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
pub struct MotorSearch {
    /// The credits, shown with every list: motor.fusionspace.co's, as its data license (CC BY
    /// 4.0) asks, with its caution to check stock and price on the vendor's own page; then
    /// ThrustCurve.org's, whose figures the site repeats.
    pub attribution: Vec<String>,
    /// Where the list was read from.
    pub read_from: ReadFrom,
    /// When motor.fusionspace.co built the list, ISO 8601 UTC as the site writes it.
    pub generated_at: String,
    /// The motors that pass the filters, cheapest first: by one motor's price at the cheapest
    /// vendor with it in stock, then by maker and designation. Motors with no such price in U.S.
    /// dollars come last, by maker and designation.
    pub motors: Vec<FoundMotor>,
}

/// `hpr motors fetch`: a motor fetched from ThrustCurve.org into the cache, so `hpr sim` flies it
/// with no network after.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
pub struct MotorFetch {
    /// ThrustCurve.org's credit, shown with every fetched motor.
    pub attribution: Vec<String>,
    /// The motor, and the curve file HPR Sim flies it by.
    pub motor: ThrustCurveMotor,
}

/// A motor found on ThrustCurve.org by name, and the curve file HPR Sim flies it by: the first RASP
/// file that reads, ranked by who measured it (a certification test, the manufacturer, a user),
/// else the first RockSim file the same way.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
pub struct ThrustCurveMotor {
    /// The manufacturer, as ThrustCurve.org names it, such as `AeroTech`.
    pub manufacturer: String,
    /// The designation, as ThrustCurve.org spells it, such as `F27R/L`.
    pub designation: String,
    /// The common name, such as `F27`, when ThrustCurve.org gives one.
    pub common_name: Option<String>,
    /// ThrustCurve.org's id of the motor.
    pub motor_id: String,
    /// ThrustCurve.org's id of the curve file flown.
    pub file_id: String,
    /// The file's format: `eng` or `rse`.
    pub format: FileFormat,
    /// Who measured the curve, as ThrustCurve.org says: `cert` (a certification test), `mfr`
    /// (the manufacturer) or `user`; `null` when it doesn't say.
    pub measured_by: Option<String>,
    /// The file's license, as ThrustCurve.org says: `PD` (public domain), `free` or `other`;
    /// `null` when its contributor named none.
    pub license: Option<String>,
    /// The file's page on ThrustCurve.org, when it gives one.
    pub page: Option<String>,
    /// Where the file was read from: fetched now, or the cache.
    pub read_from: ReadFrom,
}

/// One motor that passed `hpr motors search`'s filters, as motor.fusionspace.co lists it. The
/// figures are ThrustCurve.org's published ones, which the site repeats.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
pub struct FoundMotor {
    /// The manufacturer, as the site names it, such as `Cesaroni Technology`.
    pub manufacturer: String,
    /// The designation, as ThrustCurve.org spells it, such as `3683L851-P`.
    pub designation: String,
    /// The designation without its propellant code, such as `L851`.
    pub common_name: Option<String>,
    /// The impulse class, such as `L`.
    pub impulse_class: String,
    /// `single_use`, `reload` or `hybrid`, when the site says.
    pub motor_type: Option<MotorKind>,
    /// Diameter, mm.
    pub diameter_mm: f64,
    /// Total impulse, N·s, as stated.
    pub total_impulse_ns: Option<f64>,
    /// Average thrust, N, as stated.
    pub average_thrust_n: Option<f64>,
    /// Burn time, s, as stated.
    pub burn_time_s: Option<f64>,
    /// The propellant's trade name, such as `White Lightning`.
    pub propellant: Option<String>,
    /// The delays it comes with, s, comma-separated, or `P` for plugged.
    pub delays: Option<String>,
    /// In stock at one vendor or more.
    pub in_stock: bool,
    /// How many vendors have it in stock.
    pub in_stock_vendor_count: u32,
    /// The vendor with it in stock at the lowest price of one motor; null when out of stock.
    pub cheapest_in_stock: Option<FoundOffer>,
}

/// A vendor's offer of a motor in stock, as motor.fusionspace.co lists it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, JsonSchema)]
pub struct FoundOffer {
    /// The vendor's name.
    pub vendor: String,
    /// The product page, where stock and price are the vendor's to say.
    pub url: String,
    /// The price of one motor, in hundredths of the currency (cents), when the page shows one.
    pub unit_price_cents: Option<u64>,
    /// The price of the pack, in hundredths of the currency, when the page shows one.
    pub price_cents: Option<u64>,
    /// How many motors the pack holds.
    pub pack_size: u32,
    /// The currency, such as `USD`.
    pub currency: String,
}

/// `hpr motors show`: each motor's figures, worked out from its thrust curve.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
pub struct MotorShow {
    /// What kind of result this is: copied from a curve file, or computed from it.
    pub kind: crate::trust::Kind,
    /// The trust note the text form ends with: where the figures come from, what they were
    /// checked against, and what to go by instead (decision record ADR-213).
    pub trust: String,
    /// The motors found: every catalog match for a name, or every motor in a file.
    pub motors: Vec<MotorFigures>,
    /// What the reader accepted with a caveat, in file order.
    pub warnings: Vec<Warning>,
}

/// One motor's figures, worked out by HPR Sim from its thrust curve.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
pub struct MotorFigures {
    /// The motor's name: the catalog's designation, or the file's.
    pub name: String,
    /// The manufacturer, as the catalog or the file writes it.
    pub manufacturer: String,
    /// Where the motor and its curve come from.
    pub source: MotorSource,
    /// Casing diameter, m.
    pub diameter_m: f64,
    /// Casing length, m.
    pub length_m: f64,
    /// Propellant mass, kg.
    pub propellant_mass_kg: f64,
    /// Loaded motor mass, kg.
    pub loaded_mass_kg: f64,
    /// Total impulse, N·s: the thrust curve's area, joining its points with straight lines.
    pub total_impulse_ns: f64,
    /// The impulse class the total impulse falls in, such as `J`.
    pub impulse_class: String,
    /// Average thrust, N: total impulse over the burn time.
    pub average_thrust_n: f64,
    /// Peak thrust, N: the curve's largest point.
    pub peak_thrust_n: f64,
    /// Burn time, s, by NFPA 1125: from when thrust first reaches 5% of its peak to when it last
    /// falls to it.
    pub burn_time_s: f64,
    /// When the NFPA 1125 burn starts, s.
    pub burn_start_s: f64,
    /// When the NFPA 1125 burn ends, s.
    pub burn_end_s: f64,
    /// The curve's last time, s.
    pub curve_end_s: f64,
    /// The ejection delays offered.
    pub delays: Vec<Delay>,
}

/// Where a motor comes from.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum MotorSource {
    /// The bundled catalog, with a public-domain curve from ThrustCurve.org.
    Catalog {
        /// The curve file's ThrustCurve.org page.
        curve_url: String,
        /// The curve's format: `eng` or `rse`.
        format: FileFormat,
    },
    /// A motor file named on the command line.
    File {
        /// The path as given.
        path: String,
        /// The file's format: `eng` or `rse`.
        format: FileFormat,
    },
}

/// A motor file format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum FileFormat {
    /// RASP `.eng`.
    Eng,
    /// RockSim `.rse`.
    Rse,
}

/// One ejection delay setting.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, JsonSchema)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum Delay {
    /// The ejection charge fires this many seconds after burnout.
    Seconds(f64),
    /// No ejection charge: the forward closure is plugged.
    Plugged,
    /// A `0`: the RASP format means a charge at burnout, but most files mean plugged.
    ZeroOrPlugged,
}

/// Something a reader accepted with a caveat.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, JsonSchema)]
pub struct Warning {
    /// The motor it is about, when it is about one.
    pub motor: Option<String>,
    /// The 1-based line in the file, when it is about one.
    pub line: Option<usize>,
    /// How serious it is.
    pub kind: WarningKind,
    /// What was found, and how it was read.
    pub message: String,
}

/// How serious a warning is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum WarningKind {
    /// A whole motor had an error and was left out.
    Skipped,
    /// A value was dropped or ignored.
    Dropped,
    /// Something unusual was read as it stands.
    Unusual,
}

/// `hpr sim`: what was flown, from where, and how the flight went.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
pub struct SimFlight {
    /// The design and the configuration flown.
    pub design: SimDesign,
    /// What kind of result this is: every figure in it is simulated, none measured.
    pub kind: crate::trust::Kind,
    /// The trust note the text form ends with: what the figures are, what HPR Sim's apogee
    /// was checked against, with the committed report's numbers, and what to rely on instead
    /// (decision record ADR-211).
    pub trust: String,
    /// The motors flown.
    pub motors: Vec<SimMotor>,
    /// The recovery devices flown, in the file's order: a `.ork`'s parachutes and streamers as
    /// OpenRocket flies them (decision record ADR-153). Empty when none is flown; the notes say
    /// why.
    pub recovery: Vec<SimDevice>,
    /// The launch site, rail and wind.
    pub launch: Launch,
    /// The flight's metrics.
    pub summary: Summary,
    /// The flight's events, in order.
    pub events: Vec<SimEvent>,
    /// The recording files written, in the order given.
    pub exports: Vec<Export>,
    /// The `--plot` figure written, if one was: an SVG of the altitude, speed and acceleration
    /// against time, the events marked.
    pub plot: Option<String>,
    /// What the flight leaves out of the design, such as its parachutes, and what to make of the
    /// numbers it leaves out.
    pub notes: Vec<String>,
    /// What the design's or the motor file's reader accepted with a caveat, and what the design's
    /// checks found unusual but buildable.
    pub warnings: Vec<InputWarning>,
    /// Where the flight went past what HPR Sim's numbers have been checked for (the operating
    /// envelope, decision record ADR-179): empty for a flight inside it all.
    pub flags: Vec<SimFlag>,
    /// The known errors in HPR Sim's drag the flight meets, each by its issue number (decision
    /// record ADR-180): empty for a flight that meets none.
    pub issues: Vec<SimIssue>,
}

/// `hpr mc`: a design flown many times, each flight's inputs scattered at random about the
/// nominal flight's, and how far its apogee and landing spread.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
pub struct McRun {
    /// The design and the configuration flown.
    pub design: SimDesign,
    /// What kind of result this is: every figure in it is simulated, none measured.
    pub kind: crate::trust::Kind,
    /// The trust note the text form ends with: what the figures are, what HPR Sim's apogee
    /// was checked against, with the committed report's numbers, and what to rely on instead
    /// (decision record ADR-211).
    pub trust: String,
    /// The motors flown.
    pub motors: Vec<SimMotor>,
    /// The nominal launch site, rail and wind, which the flights scatter about.
    pub launch: Launch,
    /// One standard deviation of each scattered input, as given; zero leaves it at its nominal
    /// value.
    pub dispersion: McDispersion,
    /// The run's seed: the same seed, design and options fly the same flights, bit for bit, on
    /// one platform and build.
    pub seed: u64,
    /// The flights tried.
    pub runs: u64,
    /// The flights that flew to the ground.
    pub flown: usize,
    /// The flights that failed, counted, with why. Each spread counts them as tried with no
    /// value.
    pub failed: McFailed,
    /// The nominal flight's figures: the flight `hpr sim` flies with the same options.
    pub nominal: McNominal,
    /// The apogee's spread, m above the site.
    pub apogee_m: McSpread,
    /// The spread of the landing's distance from the pad, m.
    pub landing_distance_m: McSpread,
    /// Where the flights landed, as ellipses.
    pub landing: McLanding,
    /// The operating envelope's flags the flights raise, each with how many raised it.
    pub flags: Vec<McFlag>,
    /// The known errors in HPR Sim's drag or stability that the nominal flight meets.
    pub issues: Vec<SimIssue>,
    /// The `--export` file of every flight's draw and outcome, if one was written.
    pub export: Option<Export>,
    /// What the flights leave out of the design, and what to make of their numbers.
    pub notes: Vec<String>,
    /// Problems with the inputs that the flights went ahead despite.
    pub warnings: Vec<InputWarning>,
}

/// One standard deviation of each input `hpr mc` scatters, in the units of its option.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, JsonSchema)]
pub struct McDispersion {
    /// Each stage's mass without motors, as a fraction of it (0.02 is 2%).
    pub mass_sd_fraction: f64,
    /// Each stage's center of mass along the axis, m.
    pub cg_sd_m: f64,
    /// The rocket's zero-lift drag coefficient, as a fraction of it.
    pub drag_sd_fraction: f64,
    /// Each motor's total impulse, as a fraction of it, its propellant mass with it.
    pub impulse_sd_fraction: f64,
    /// Each motor's burn time, as a fraction of it, at the same impulse.
    pub burn_time_sd_fraction: f64,
    /// Each motor's ejection delay, s.
    pub delay_sd_s: f64,
    /// The wind's speed at every height, as a fraction of it.
    pub wind_sd_fraction: f64,
    /// The wind's direction, degrees, about the nominal one.
    pub wind_from_sd_deg: f64,
    /// The rail's angle above the horizon, degrees.
    pub inclination_sd_deg: f64,
    /// The rail's heading, degrees.
    pub heading_sd_deg: f64,
    /// Each recovery device's lag after its trigger, s.
    pub deployment_lag_sd_s: f64,
}

/// The flights of a run that failed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, JsonSchema)]
pub struct McFailed {
    /// How many failed.
    pub count: usize,
    /// Why, one entry for each distinct reason, in the order each first happened.
    pub reasons: Vec<McFailure>,
}

/// A reason flights of a run failed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, JsonSchema)]
pub struct McFailure {
    /// Where: making the flight's inputs from its draw, or flying them.
    pub at: McFailedAt,
    /// The error.
    pub reason: String,
    /// How many flights failed so.
    pub count: usize,
    /// The first such flight's index, from 0: its row in the `--export` file.
    pub first_index: u64,
}

/// Where a flight of a run failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum McFailedAt {
    /// Making its inputs: its draw left one impossible, such as a motor with no impulse.
    Inputs,
    /// Flying them: the flight refused them, such as a negative mass, or stopped with an error.
    Flight,
}

/// The nominal flight's figures, beside the spreads.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, JsonSchema)]
pub struct McNominal {
    /// Its apogee, m above the site, if it has one.
    pub apogee_m: Option<f64>,
    /// Its landing's distance from the pad, m, if it landed.
    pub landing_distance_m: Option<f64>,
    /// Its landing, m east of the pad.
    pub landing_east_m: Option<f64>,
    /// Its landing, m north of the pad.
    pub landing_north_m: Option<f64>,
}

/// How a figure spreads over a run's flights. A failed flight, or one without the figure, counts
/// as tried with no value; a statistic is `null` when too few flights have one.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, JsonSchema)]
pub struct McSpread {
    /// The flights tried.
    pub attempted: usize,
    /// The flights with a value.
    pub count: usize,
    /// The mean.
    pub mean: Option<f64>,
    /// The sample standard deviation (`n − 1`), `null` with fewer than two values.
    pub standard_deviation: Option<f64>,
    /// The smallest value.
    pub min: Option<f64>,
    /// The 5th percentile.
    pub p05: Option<f64>,
    /// The median.
    pub p50: Option<f64>,
    /// The 95th percentile.
    pub p95: Option<f64>,
    /// The largest value.
    pub max: Option<f64>,
}

/// Where a run's flights landed: the part that keeps the nose, the flight itself until a split
/// with nothing left to burn, and the sustainer after a powered separation.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
pub struct McLanding {
    /// The flights that landed.
    pub count: usize,
    /// The landings' mean, m east of the pad; `null` with none.
    pub center_east_m: Option<f64>,
    /// The landings' mean, m north of the pad; `null` with none.
    pub center_north_m: Option<f64>,
    /// The 50% and 95% ellipses of the scatter, then the 95% ellipse for the next flight; the
    /// scatter's need two landings, the next flight's three.
    pub ellipses: Vec<McEllipse>,
}

/// An ellipse on the ground about a run's landings.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, JsonSchema)]
pub struct McEllipse {
    /// Which kind it is.
    pub kind: EllipseKind,
    /// Its level: the share of a normal scatter it holds, or for the next flight the chance it
    /// lands inside.
    pub level: f64,
    /// Its center, m east of the pad.
    pub center_east_m: f64,
    /// Its center, m north of the pad.
    pub center_north_m: f64,
    /// Half its long axis, m.
    pub semi_major_m: f64,
    /// Half its short axis, m.
    pub semi_minor_m: f64,
    /// Its long axis's heading, degrees clockwise from true north, in `[0, 180)`.
    pub major_heading_deg: f64,
    /// The share of the flights tried that landed inside it, a failed flight counted outside.
    pub inside: f64,
}

/// What an ellipse about a run's landings holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum EllipseKind {
    /// A share of the landings, were they a normal scatter with the run's mean and covariance.
    Scatter,
    /// Where the next flight lands with the ellipse's chance, allowing for the run's spread being
    /// only an estimate.
    NextFlight,
}

/// An envelope flag the flights of a run raise.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, JsonSchema)]
pub struct McFlag {
    /// Which flag.
    pub flag: FlagKind,
    /// How many flights raise it.
    pub flights: usize,
}

/// The sidecar `hpr mc --export` writes beside its CSV, `runs.meta.json` for `runs.csv`: the
/// program that wrote it, the run it holds, the bundled catalog's as-of date and how far to trust
/// the figures (the product system's `data.md`, ADR-174, ADR-214).
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
pub struct McExportMeta {
    /// The CSV, by its file name.
    pub file: String,
    /// The design flown, by its file name.
    pub design: String,
    /// The configuration flown, as `hpr mc` names it, such as `[C6-5]`.
    pub configuration: String,
    /// The run's seed.
    pub seed: u64,
    /// The flights tried: the CSV's rows after its header.
    pub runs: u64,
    /// One standard deviation of each scattered input.
    pub dispersion: McDispersion,
    /// The bundled motor catalog's as-of date, `YYYY-MM-DD`: the day its curve files were
    /// downloaded. A build fixes it; whether a motor flown came from the catalog is `hpr mc
    /// --json`'s `motors[].source`.
    pub catalog_as_of: String,
    /// What kind of figures the CSV holds: `simulated`.
    pub kind: crate::trust::Kind,
    /// The trust note `hpr mc` ends with: the spread is its inputs', not the model's error.
    pub trust: String,
}

/// A flag the flight raises, with the peak that raised it.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
pub struct SimFlag {
    /// Which flag.
    pub flag: FlagKind,
    /// The peak that raised it: the top Mach number, for `high_angle_of_attack` the largest
    /// angle of attack that counts, rad, for `unstable_under_power` the least static margin
    /// while a motor burns, calibres, or for `unstable_without_margin` the largest pitch-moment
    /// slope C_mα where the margin is undefined while a motor burns, per radian.
    pub peak: Peak,
    /// What the flag means for this flight, in a sentence.
    pub message: String,
}

/// A known error in HPR Sim's drag, stability or flight path that the flight meets.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
pub struct SimIssue {
    /// The issue's number on GitHub: 18, 67, 68, 70, 72, 73 or 222 for drag; 179 or 354 for a
    /// separated part's drag, also of kind drag; 64, 87, 120, 121, 172, 325 or 326 for stability;
    /// 8, 106, 213 or 219 for the flight's path, whose signs depend on the case.
    pub issue: u32,
    /// Which numbers it bears on.
    pub kind: IssueKind,
    /// The issue's address.
    pub url: String,
    /// The flight's top Mach number.
    pub max_mach: Peak,
    /// The ids of the parts whose shape meets the issue's condition; empty for base drag (68)
    /// and friction (18), which every rocket has, for 121 and 172, for a separated part's
    /// (179, 354), and for the wind's (8) and the center of gravity's (219). For 213 the single
    /// pod sets, for 106 the body parts the supersonic join covers.
    pub parts: Vec<String>,
    /// What the issue means for this flight, with which way its numbers lean.
    pub message: String,
}

/// Which numbers a known issue bears on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum IssueKind {
    /// The drag: the apogee, the top speed, the drift, the flutter margin.
    Drag,
    /// The stability margin and the center of pressure.
    Stability,
    /// The flight's path, through something other than the drag or the margin (a moment, the
    /// damping, the wind): the apogee and the drift.
    Flight,
}

/// The library's kind, matched whole: `hpr_sim`'s kind isn't `#[non_exhaustive]`, so a new one
/// fails to compile here rather than print as another.
impl From<hpr::hpr_sim::issues::IssueKind> for IssueKind {
    fn from(kind: hpr::hpr_sim::issues::IssueKind) -> Self {
        match kind {
            hpr::hpr_sim::issues::IssueKind::Drag => Self::Drag,
            hpr::hpr_sim::issues::IssueKind::Stability => Self::Stability,
            hpr::hpr_sim::issues::IssueKind::Flight => Self::Flight,
        }
    }
}

/// The flags a flight raises: the operating envelope's (decision records ADR-143 and ADR-179),
/// and a rocket unstable under power (issue 335), by its static margin or, where it has none, by
/// its pitching moment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum FlagKind {
    /// A static margin below zero, in the weakest plane, from the rail exit to apogee or the
    /// first deployment while a motor burns: the rocket is unstable under power, and its apogee is
    /// not a prediction.
    UnstableUnderPower,
    /// A pitch-moment slope C_mα above zero, from the rail exit to apogee or the first
    /// deployment while a motor burns, where HPR Sim can give no static margin: the rocket is
    /// unstable under power, and its apogee is not a prediction. Raised only when
    /// `unstable_under_power` isn't.
    UnstableWithoutMargin,
    /// Faster than the fastest public flight HPR Sim has been compared with an independent
    /// reference on.
    BeyondValidatedRange,
    /// Above 15° more than 1 s after the rail exit and before apogee or the first deployment,
    /// where a 15° angle would give a normal force of at least a fifth of the rocket's weight.
    HighAngleOfAttack,
    /// Past Mach 2.5, the core band's top.
    OutsideCoreBand,
    /// Past Mach 3.5, the envelope's top.
    BeyondEnvelope,
}

/// The design flown.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
pub struct SimDesign {
    /// The file's name, without its folder.
    pub file: String,
    /// The file's format.
    pub format: DesignFormat,
    /// The rocket's name, as the file writes it.
    pub name: String,
    /// The id of the motor configuration flown.
    pub configuration: String,
    /// Its name, as the file writes it; often empty.
    pub configuration_name: String,
    /// What the text output calls it: its name, or where it has none its motors and delays in
    /// brackets, such as `[H128W-14]`. After `--motor`, the file's configuration and the motor
    /// given, such as `[H128W-14] with --motor H54`, or the motor's name where the file has none.
    pub configuration_label: String,
    /// Every motor configuration the file holds, the one flown among them, before `--motor`
    /// changed any.
    pub configurations: Vec<DesignConfiguration>,
}

/// A motor configuration of the design file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, JsonSchema)]
pub struct DesignConfiguration {
    /// Its id, which `--config` also takes.
    pub id: String,
    /// Its name, as the file writes it; often empty.
    pub name: String,
    /// What the text output calls it: its name, or where it has none its motors and delays in
    /// brackets, such as `[H128W-14]`. `--config` takes it, its id, or its place in this list
    /// counting from 1.
    pub label: String,
    /// Whether `hpr sim` flies it as the file has it, without `--motor`, with the curves this
    /// run read: a motor whose curve wasn't fetched in this run leaves it out.
    pub flies: bool,
    /// Why it isn't flown as read, in a few words, such as `its motor's curve to fetch`; `None`
    /// when it flies.
    pub not_flown: Option<String>,
}

/// A design file format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DesignFormat {
    /// An OpenRocket `.ork` file.
    Ork,
    /// A rocket's JSON (`.json`): a `hpr_design::Rocket` alone.
    HprJson,
    /// A document of the HPR design format (`.hpr`).
    Hpr,
    /// A design in the HPR design format's zip container (`.hprz`).
    Hprz,
}

/// A recovery device flown.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
pub struct SimDevice {
    /// Its name, as the file writes it, or its id where the name is empty.
    pub name: String,
    /// The part that carries it: `null` for the rocket, or the sustainer that keeps its nose;
    /// otherwise the index of the part that came apart from it, as a landing's `body` numbers it.
    pub body: Option<usize>,
    /// Whether HPR Sim added it rather than read it from the file: a tumble a separated part needs,
    /// as a part flies alone with only its devices' drag. It brakes no fall HPR Sim counts as
    /// predicted.
    pub added: bool,
    /// What opens it.
    pub opens_at: DeviceEvent,
    /// For a device set to a height, that height above the launch site, m.
    pub height_above_ground_m: Option<f64>,
    /// Seconds from its event to its opening; for `launch`, from the launch.
    pub delay_s: f64,
    /// Its drag area once open, `C_D S`, m².
    pub drag_area_m2: f64,
    /// When it opened, s after launch; `None` if it never did in this flight.
    pub opened_s: Option<f64>,
}

/// What opens a recovery device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DeviceEvent {
    /// The apogee.
    Apogee,
    /// A height on the way down, or the apogee if that is lower.
    Altitude,
    /// The ejection charge of its stage's motor.
    Ejection,
    /// The launch.
    Launch,
    /// The separation that frees its part, when the part flies on its own.
    Separation,
}

/// One motor flown.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
pub struct SimMotor {
    /// Its designation, such as `168H54-10A`.
    pub designation: String,
    /// The id of the motor mount it is in, which `--mount` takes, as it takes the mount's name.
    pub mount: String,
    /// The mount's name, as the file writes it.
    pub mount_name: String,
    /// How many of it fly: one per tube of a cluster, and one per pod of a pod set.
    pub count: usize,
    /// How many of those never light: in a tube the design marks failed, or set never to light.
    pub unlit: usize,
    /// Where it comes from.
    pub source: SimMotorSource,
    /// When it lights, in words: `at launch`, or as the design says.
    pub ignition: String,
    /// Its ejection delay as the design sets it, if it does; `--motor` sets none.
    pub delay: Option<Delay>,
}

/// Where a flown motor comes from.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SimMotorSource {
    /// The design file's own configuration.
    Design,
    /// The bundled catalog: named with `--motor`, or a `.ork` file's motor with no curve in the
    /// file, found there by its manufacturer and designation.
    Catalog {
        /// The catalog's as-of date, `YYYY-MM-DD`: the day its curve files were downloaded.
        as_of: String,
    },
    /// A motor file named with `--motor`.
    File {
        /// The file's name, without its folder.
        file: String,
        /// The file's format: `eng` or `rse`.
        format: FileFormat,
    },
    /// A curve fetched from ThrustCurve.org: a `--motor` name the bundled catalog lacks, or a
    /// design's motor with no curve of its own, found by its manufacturer and designation.
    ThrustCurve(Box<ThrustCurveMotor>),
}

/// Where and how the rocket was launched.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
pub struct Launch {
    /// The site's latitude, degrees north.
    pub latitude_deg: f64,
    /// The site's longitude, degrees east.
    pub longitude_deg: f64,
    /// The site's elevation above sea level, m.
    pub elevation_m: f64,
    /// The rail's length, m, from the rocket's aft end at the start to the rail's top.
    pub rail_length_m: f64,
    /// The rail's angle above the horizon, degrees: 90 is vertical.
    pub inclination_deg: f64,
    /// The direction the rail leans toward, clockwise from true north, degrees.
    pub heading_deg: f64,
    /// The wind's speed, m/s, the same at every height; 0 for calm air.
    pub wind_speed_m_s: f64,
    /// The direction the wind blows from, clockwise from true north, degrees.
    pub wind_from_deg: f64,
    /// The atmosphere: always `standard`, the 1976 US Standard Atmosphere.
    pub atmosphere: String,
}

/// A flight's metrics, as the library's `hpr_sim::metrics::FlightSummary` gives them, each peak
/// marked if it came after apogee. Heights are the center of gravity's above the launch site;
/// speeds are relative to the ground.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
pub struct Summary {
    /// Why the flight ended.
    pub termination: Termination,
    /// The center of gravity's height above the launch site at launch, m: not zero, as the rocket
    /// stands on the rail.
    pub launch_height_m: Option<f64>,
    /// The speed as the rocket left the rail, m/s.
    pub rail_exit_speed_m_s: Option<Peak>,
    /// The highest point.
    pub apogee: Option<Apogee>,
    /// The top speed, m/s.
    pub max_speed_m_s: Option<Peak>,
    /// The top Mach number.
    pub max_mach: Option<Peak>,
    /// The top dynamic pressure, Pa.
    pub max_dynamic_pressure_pa: Option<Peak>,
    /// The top acceleration of the nose tip relative to the launch site's frame, from liftoff
    /// until a recovery device opens, m/s²: the motion's, not what an accelerometer reads.
    pub max_acceleration_m_s2: Option<Peak>,
    /// The top acceleration under the recovery devices, m/s²: the opening shock.
    pub max_descent_acceleration_m_s2: Option<Peak>,
    /// The least static stability margin, calibres, from the rail exit to apogee or the first
    /// deployment.
    pub min_static_margin_cal: Option<Peak>,
    /// The least flight margin, calibres, over the same span: the margin at the flight's Mach
    /// number, along the axis.
    pub min_flight_margin_cal: Option<Peak>,
    /// The least static margin, calibres, over the same span while a motor burns: below zero, the
    /// rocket is unstable under power and `flags` has `unstable_under_power`.
    pub min_powered_static_margin_cal: Option<Peak>,
    /// The largest pitch-moment slope C_mα, per radian, over the same span while a motor burns,
    /// at instants where the static margin is undefined; null when the margin is defined at
    /// every such instant. Above zero, the air turns the rocket away from its path and `flags`
    /// has `unstable_without_margin`.
    pub max_powered_moment_slope_per_rad: Option<Peak>,
    /// The stability as the rocket left the rail.
    pub rail_exit_stability: Option<Stability>,
    /// The largest angle of attack, rad, more than 1 s after the rail exit and before apogee or
    /// the first deployment, counting only instants where a 15° angle would give a normal force of
    /// at least a fifth of the rocket's weight.
    pub max_angle_of_attack_rad: Option<Peak>,
    /// Where and how fast the rocket landed. With no recovery device opened, the fall from apogee
    /// rests on small-angle aerodynamics far outside their range: not a prediction (the notes say
    /// so).
    pub landing: Option<Landing>,
    /// Where each part that came apart from the rocket landed, if any did.
    pub body_landings: Vec<Landing>,
}

/// Why a flight ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Termination {
    /// The center of mass reached the ground.
    GroundHit,
    /// Every motor burned out before the rocket lifted off.
    NoLiftoff,
    /// The rocket lifted off but stopped on the rail after every motor burned out.
    StalledOnRail,
    /// The time cap was reached.
    TimeCap,
    /// The integrator's step limit was reached.
    StepLimit,
    /// The stack separated, and each part flew on as its own descent.
    Separated,
    /// A way of ending this build of `hpr` doesn't name.
    Other,
}

/// A metric's extreme and when it came.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, JsonSchema)]
pub struct Peak {
    /// The value, in the unit the field's name gives.
    pub value: f64,
    /// When, s after launch.
    pub time_s: f64,
    /// The height above the launch site then, m.
    pub height_above_ground_m: f64,
    /// Whether it came after apogee, in the fall. With no recovery device opened, a peak in the
    /// fall is not a prediction: the fall rests on small-angle aerodynamics far outside their
    /// range.
    pub after_apogee: bool,
}

/// The highest point.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, JsonSchema)]
pub struct Apogee {
    /// When, s after launch.
    pub time_s: f64,
    /// The height above the launch site, m.
    pub height_above_ground_m: f64,
    /// The height gained from where the center of gravity stood at launch, m, as OpenRocket's
    /// altitude counts.
    pub gain_m: Option<f64>,
}

/// The rocket's stability at one instant.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, JsonSchema)]
pub struct Stability {
    /// When, s after launch.
    pub time_s: f64,
    /// The height above the launch site, m.
    pub height_above_ground_m: f64,
    /// The dynamic pressure, Pa.
    pub dynamic_pressure_pa: f64,
    /// The center of gravity, m aft of the nose tip.
    pub cg_station_m: f64,
    /// The reference diameter the margins are counted in, m.
    pub reference_diameter_m: f64,
    /// The static margin: the air along the axis, at Mach 0, in the rocket's weakest plane.
    pub static_margin: Margin,
    /// The flight margin: the air along the axis, at the flight's Mach number, in the rocket's
    /// weakest plane. The angle of attack is left out.
    pub flight_margin: Margin,
}

/// A stability margin and what it rests on.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, JsonSchema)]
pub struct Margin {
    /// The Mach number.
    pub mach: f64,
    /// The total angle of attack, rad.
    pub angle_of_attack_rad: f64,
    /// The direction the air crosses the rocket in, rad from the body's `x` axis toward its `y`:
    /// the weakest plane's. A rocket with a fin set of one or two fins has a margin in each
    /// plane, and this is its least; any other has one margin, given at 0.
    pub roll_rad: f64,
    /// The normal-force slope `C_Nα` on the reference area, per radian.
    pub normal_force_slope_per_rad: f64,
    /// The sum of the parts' slope magnitudes, per radian: the scale the net slope is judged by.
    pub slope_magnitude_sum_per_rad: f64,
    /// The pitch-moment slope about the center of mass, per radian; negative restores.
    pub pitch_moment_slope_per_rad: f64,
    /// The center of pressure, m aft of the nose tip; `null` when the margin is.
    pub cp_station_m: Option<f64>,
    /// The margin, calibres: positive with the center of pressure aft of the center of mass;
    /// `null` when the net slope is too small for the quotient to mean anything.
    pub margin_cal: Option<f64>,
}

/// Where and how fast a body landed: where its center of mass came down to the launch site's
/// height on the ellipsoid. There is no terrain.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, JsonSchema)]
pub struct Landing {
    /// The body: `null` for the rocket, or the index of a part that came apart from it.
    pub body: Option<usize>,
    /// When, s after launch.
    pub time_s: f64,
    /// Latitude, degrees north.
    pub latitude_deg: f64,
    /// Longitude, degrees east.
    pub longitude_deg: f64,
    /// Distance east of the launch site, m.
    pub east_m: f64,
    /// Distance north of the launch site, m.
    pub north_m: f64,
    /// Horizontal distance from the launch site, m.
    pub distance_m: f64,
    /// The speed at the ground, m/s.
    pub ground_hit_speed_m_s: f64,
    /// The rate of descent at the ground, m/s.
    pub descent_rate_m_s: f64,
}

/// One event of a flight.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, JsonSchema)]
pub struct SimEvent {
    /// What happened.
    pub kind: EventKind,
    /// The device, part or motor it is about, by its index, for the kinds that have one.
    pub index: Option<usize>,
    /// When, s after launch.
    pub time_s: f64,
    /// The center of gravity's height above the launch site, m.
    pub height_above_ground_m: f64,
    /// The speed relative to the ground, m/s.
    pub speed_m_s: f64,
    /// The center of gravity's vertical velocity, m/s, positive up: a descent's rate is its
    /// negative.
    pub vertical_velocity_m_s: f64,
}

/// What happened at an event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum EventKind {
    /// The rocket started to move along the rail.
    Liftoff,
    /// The rocket left the rail.
    RailExit,
    /// Every motor lit, or due to light at a known time, has burned out.
    Burnout,
    /// The highest point.
    Apogee,
    /// The center of mass came down to the launch site's height.
    GroundHit,
    /// A recovery device's charge fired.
    Trigger,
    /// A recovery device deployed.
    Deployment,
    /// A recovery device was released.
    Release,
    /// The stack came apart.
    Separation,
    /// A piece left the airframe at an ejection.
    Ejection,
    /// A part started to move along the airframe.
    Shift,
    /// A part left the airframe.
    MassRelease,
    /// A user event.
    User,
    /// A motor lit after launch.
    Ignition,
    /// An event this build of `hpr` doesn't name.
    Other,
}

/// A recording file written.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, JsonSchema)]
pub struct Export {
    /// The path as given.
    pub path: String,
    /// The file's format.
    pub format: ExportFormat,
    /// The rows recorded: one every `--interval` seconds and one at every event.
    pub rows: usize,
    /// For a CSV file, the sidecar written beside it, which names the program that wrote it
    /// ([`ExportMeta`]); `null` for the other formats, which name it inside.
    pub meta: Option<String>,
}

/// The sidecar `hpr sim --export` writes beside a CSV recording, `flight.meta.json` for
/// `flight.csv`: the program that wrote the recording (the `tool` every document opens with),
/// what it records, the bundled catalog's as-of date and how far to trust it. A CSV opens cleanly
/// in a spreadsheet only without comment lines, so these go here (the product system's
/// `data.md`, ADR-174, ADR-214). A JSON recording carries the same in its own fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, JsonSchema)]
pub struct ExportMeta {
    /// The recording, by its file name.
    pub file: String,
    /// The design flown, by its file name.
    pub design: String,
    /// The configuration flown, as `hpr sim` names it, such as `[C6-5]`.
    pub configuration: String,
    /// The rows recorded.
    pub rows: usize,
    /// The bundled motor catalog's as-of date, `YYYY-MM-DD`: the day its curve files were
    /// downloaded. A build fixes it; whether a motor flown came from the catalog is `hpr sim
    /// --json`'s `motors[].source`.
    pub catalog_as_of: String,
    /// What kind of figures the recording holds: `simulated`.
    pub kind: crate::trust::Kind,
    /// The trust note `hpr sim` ends with.
    pub trust: String,
}

/// A recording file's format, from its extension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExportFormat {
    /// `.csv`: a header of column names with their units, then one line per row.
    Csv,
    /// `.json`: `{"columns": [...], "rows": [[...], ...]}`.
    Json,
    /// `.parquet`: Apache Parquet.
    Parquet,
    /// `.geojson`: the center of mass's path on the Earth.
    Geojson,
    /// `.kml`: the same path, for Google Earth.
    Kml,
}

/// Something a reader accepted with a caveat, or the design's checks found unusual.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, JsonSchema)]
pub struct InputWarning {
    /// Where: in a `.ork` file, a path of element names from the root or a zip entry's name; in a
    /// motor file, its name and line; `design checks` for a check's finding.
    pub at: String,
    /// How serious it is.
    pub kind: WarningKind,
    /// What was found, and how it was read.
    pub message: String,
}

/// `hpr convert`: a motor file converted, which has `motors`, or a design, which has `rocket`.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
#[serde(untagged)]
pub enum Convert {
    /// Motors, written as a `.eng` or `.rse` file.
    Motors(ConvertMotors),
    /// A design, written as a `.ork`, `.hpr` or `.hprz` file.
    Design(ConvertDesign),
}

/// `hpr convert` of motors: the motors read, and the file they were written to.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
pub struct ConvertMotors {
    /// Where the motors came from: a motor file, or the bundled catalog.
    pub input: MotorSource,
    /// The file written.
    pub output: ConvertedFile,
    /// The motors written, by the names the file gives them, in its order.
    pub motors: Vec<String>,
    /// What the reader flagged in the input, then what the conversion dropped or wrote
    /// differently, such as `.rse` figures a `.eng` file has no place for.
    pub warnings: Vec<Warning>,
}

/// `hpr convert` of a design: the design read, and the file it was written to.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
pub struct ConvertDesign {
    /// The design file read.
    pub input: DesignFilePath,
    /// The design file written.
    pub output: DesignFilePath,
    /// The rocket's name, as the design writes it.
    pub rocket: String,
    /// How many motor configurations the design holds, flyable or not.
    pub configurations: usize,
    /// The version of the HPR design format a `.hpr` or `.hprz` input was written in, when it was
    /// older than the version written and was migrated to it; `null` otherwise.
    pub migrated_from: Option<String>,
    /// The files written into a `.hprz` beside the design, by name, in order; empty for any other
    /// output.
    pub attachments: Vec<String>,
    /// What the `.ork` reader flagged in the input, what the `.ork` writer flagged in the output,
    /// and each attachment of a `.hprz` input that the output has no place for.
    pub warnings: Vec<InputWarning>,
}

/// A design file `hpr convert` read or wrote.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, JsonSchema)]
pub struct DesignFilePath {
    /// The path as given.
    pub path: String,
    /// Its format: `ork`, `hpr` or `hprz`.
    pub format: DesignFormat,
}

/// A motor file `hpr convert` wrote.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, JsonSchema)]
pub struct ConvertedFile {
    /// The path as given.
    pub path: String,
    /// Its format: `eng` or `rse`.
    pub format: FileFormat,
}

/// `hpr weather`: a site's air and wind, level by level, from one source.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
pub struct Weather {
    /// The source, and the credit its terms ask for wherever the data is shown.
    pub source: WeatherSource,
    /// What kind of result this is: a forecast, a balloon's measurement or a reanalysis.
    pub kind: crate::trust::Kind,
    /// The trust note the text form ends with: what the profile is, what it was checked
    /// against, and what to go by instead (decision record ADR-213).
    pub trust: String,
    /// Where the source's answer was read from.
    pub read_from: ReadFrom,
    /// Where the profile is: Open-Meteo's grid point, the balloon's release, or the site the
    /// forecast or file was read at.
    pub position: WeatherPosition,
    /// The time the profile is for, UTC, `YYYY-MM-DDTHH:MM:SSZ`: the launch time asked for, the
    /// balloon's release, or the time a forecast run is for.
    pub time: String,
    /// The start of the forecast run, for GFS and RAP.
    pub run: Option<String>,
    /// The profile's levels, lowest first: the ground, then the levels above it; ERA5's levels
    /// are pressure levels only, from 1000 hPa up, some of them below the ground where it is high.
    pub levels: Vec<ProfileLevel>,
    /// The levels the source gave but the profile leaves out, and why.
    pub dropped: Vec<DroppedLevel>,
    /// The file the profile was written to, as given.
    pub profile: Option<String>,
}

/// A weather source and its credit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, JsonSchema)]
pub struct WeatherSource {
    /// Which source.
    pub name: WeatherSourceName,
    /// The credit to show with its data.
    pub attribution: String,
}

/// The weather sources.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum WeatherSourceName {
    /// Open-Meteo's forecast or historical-forecast API.
    OpenMeteo,
    /// The University of Wyoming's radiosonde archive.
    Wyoming,
    /// NOAA's GFS forecast, from NOMADS.
    Gfs,
    /// NOAA's RAP forecast, from NOMADS.
    Rap,
    /// An ECMWF ERA5 pressure-level file.
    Era5,
}

/// Where a source's answer was read from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ReadFrom {
    /// A file given on the command line: `--from`, or an ERA5 file.
    File {
        /// The path as given.
        path: String,
    },
    /// Fetched now, and saved in the cache.
    Network {
        /// When, in seconds since the Unix epoch.
        fetched_at_unix_s: u64,
    },
    /// The cache, still fresh.
    Cache {
        /// When the copy was fetched, in seconds since the Unix epoch.
        fetched_at_unix_s: u64,
    },
    /// The cache, older than the source keeps a copy fresh: offline, or the fetch failed.
    StaleCache {
        /// When the copy was fetched, in seconds since the Unix epoch.
        fetched_at_unix_s: u64,
        /// Online, why the fetch failed or its answer was refused.
        reason: Option<String>,
    },
}

/// Where a weather profile is.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, JsonSchema)]
pub struct WeatherPosition {
    /// Degrees north.
    pub latitude_deg: f64,
    /// Degrees east.
    pub longitude_deg: f64,
}

/// One level of a weather profile.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, JsonSchema)]
pub struct ProfileLevel {
    /// Geometric height above mean sea level, m.
    pub height_msl_m: f64,
    /// Pressure, Pa.
    pub pressure_pa: Option<f64>,
    /// Temperature, K.
    pub temperature_k: f64,
    /// Relative humidity over liquid water, a fraction; absent where the source gives none
    /// (ERA5 as read, dry air).
    pub relative_humidity: Option<f64>,
    /// Wind speed, m/s.
    pub wind_speed_m_s: Option<f64>,
    /// The direction the wind blows from, degrees clockwise from true north.
    pub wind_from_deg: Option<f64>,
}

/// A level a weather source gave that the profile leaves out.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, JsonSchema)]
pub struct DroppedLevel {
    /// Its pressure, Pa, for a forecast's pressure level.
    pub pressure_pa: Option<f64>,
    /// Its line in the answer, for a sounding's row (the header is line 1).
    pub line: Option<usize>,
    /// Why it was left out.
    pub reason: DropReason,
}

/// Why a weather level was left out.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DropReason {
    /// Its pressure is not below the ground's, or its height not above it: the model's values
    /// under high ground.
    BelowGround,
    /// A value is missing.
    NoData,
    /// A value is out of range.
    OutOfRange,
    /// A sounding's row with the same pressure as its neighbours, not the middle one.
    SamePressure,
    /// A sounding's row not above the last one kept.
    NotAbove,
    /// A sounding's row whose height doesn't fit the thickness its pressure and temperature give.
    Thickness,
    /// A reason this build of `hpr` doesn't name.
    Other,
}

impl DropReason {
    /// A few words for the text output.
    pub fn describe(self) -> &'static str {
        match self {
            Self::BelowGround => "below the ground",
            Self::NoData => "a value is missing",
            Self::OutOfRange => "a value is out of range",
            Self::SamePressure => "a repeat of its pressure",
            Self::NotAbove => "not above the row before",
            Self::Thickness => "its height doesn't fit the layer's thickness",
            Self::Other => "another reason",
        }
    }
}

/// `hpr analyze`: a flight log's readings, taken from the log alone, with no design file and no
/// simulation. Heights are meters above the logger's own zero, which a PerfectFlite takes on the
/// pad; times are seconds on the log's clock.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
pub struct Analyze {
    /// What kind of result this is: every reading is measured, from the log.
    pub kind: crate::trust::Kind,
    /// The trust note the text form ends with: what the readings are, what they were checked
    /// against, and what to go by instead (decision record ADR-213).
    pub trust: String,
    /// The log read.
    pub log: AnalyzedLog,
    /// What the file states about the flight: the logger's own figures, printed beside HPR Sim's
    /// readings and never in place of them.
    pub stated: LoggerStated,
    /// How the readings were taken.
    pub method: AnalyzeMethod,
    /// Liftoff.
    pub liftoff: LogReading<LiftoffReading>,
    /// The highest point.
    pub apogee: LogReading<ApogeeReading>,
    /// The top vertical speed from liftoff to apogee.
    pub max_speed: LogReading<MaxSpeedReading>,
    /// The top acceleration.
    pub max_acceleration: LogReading<MaxAccelerationReading>,
    /// Landing, and the descent before it.
    pub landing: LogReading<LandingReading>,
}

/// The log `hpr analyze` read.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
pub struct AnalyzedLog {
    /// The file, as given.
    pub path: String,
    /// Its format.
    pub format: LogFormatName,
    /// The logger, as the file names it.
    pub logger: String,
    /// The logger's serial number, as the file states it.
    pub serial_number: Option<String>,
    /// The logger's firmware version, as the file states it.
    pub firmware: Option<String>,
    /// The flight's number in the logger's memory, as the file states it.
    pub flight_number: Option<u32>,
    /// How many samples the file holds.
    pub samples: usize,
    /// The first sample's time, s.
    pub first_time_s: f64,
    /// The last sample's time, s.
    pub last_time_s: f64,
    /// What the reader noticed and worked around, such as a sample count that differs from the
    /// one the file states.
    pub notes: Vec<String>,
}

/// A flight log's format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum LogFormatName {
    /// PerfectFlite's `.pf2`: the Pnut, the StratoLogger and the StratoLoggerCF.
    PerfectFlitePf2,
    /// A format this build of `hpr` doesn't name.
    Other,
}

/// What a flight log states about the flight.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, JsonSchema)]
pub struct LoggerStated {
    /// The apogee the logger computed, m above its own zero; `null` if the file states none, or
    /// states something that isn't a height, such as a PerfectFlite's `PWRLOSS`.
    pub apogee_m: Option<f64>,
    /// The launch site's elevation, m above mean sea level, as the logger states it.
    pub ground_elevation_msl_m: Option<f64>,
}

/// How the readings were taken. Every field but `altitude_resolution_m` is `null` for a log too
/// short to read, or one withheld whole as `bad_record`; all but it and `sample_interval_s` for
/// one withheld as `sampled_too_fast`; and `pad_altitude_m` for one withheld as `no_climb`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, JsonSchema)]
pub struct AnalyzeMethod {
    /// The median interval between samples, s.
    pub sample_interval_s: Option<f64>,
    /// The running median's span, s: every height and time is read from the altitude after it.
    pub median_window_s: Option<f64>,
    /// How far below its true peak the running median can read a peak bent by gravity alone, m.
    pub peak_bound_m: Option<f64>,
    /// The altitude's resolution in the log's format, m: a PerfectFlite writes whole feet.
    pub altitude_resolution_m: f64,
    /// The pad: the median of the altitude before it first rises 1 m, m.
    pub pad_altitude_m: Option<f64>,
}

/// A reading, or why the log can't support it.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum LogReading<T> {
    /// The reading.
    Read(T),
    /// The log can't support the reading.
    Withheld(WithheldReading),
}

/// Why a reading was withheld.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, JsonSchema)]
pub struct WithheldReading {
    /// The reason, as a code.
    pub reason: WithheldReason,
    /// The reason in words, with the log's own numbers.
    pub detail: String,
}

/// The reason a reading was withheld.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum WithheldReason {
    /// The log has fewer than three samples.
    TooShort,
    /// The altitude never climbs 3 m above where the log starts.
    NoClimb,
    /// The pad, the median altitude before the first meter of rise, is more than 3 m from the
    /// logger's zero: the log didn't start on the pad.
    StartsOffThePad,
    /// The log ends before the rocket is seen to land.
    EndsBeforeLanding,
    /// The altitude reaches the ground sooner than a fall from rest at apogee in vacuum could.
    FasterThanFreeFall,
    /// The log has no speed column.
    NoSpeedColumn,
    /// The top speed is above 4,000 m/s.
    ImplausibleSpeed,
    /// The climb's speed swings negative by more than 20% of its top.
    NoisySpeed,
    /// The top speed falls on the liftoff sample itself.
    SpeedPeakAtLiftoff,
    /// The log has no accelerometer.
    NoAccelerometer,
    /// The reading needs another, which was withheld.
    Needs,
    /// The record breaks what every reader guarantees; only a record built by hand can.
    BadRecord,
    /// The samples come so often that the 0.3 s median would hold more than 1,000 either side.
    SampledTooFast,
    /// A reason this build of `hpr` doesn't name.
    Other,
}

/// Where a reading's value came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReadingSource {
    /// The logger's barometric altitude, after the running median.
    Barometer,
    /// A speed column the logger computed from its own barometric altitude.
    LoggerSpeedFromBarometer,
    /// A source this build of `hpr` doesn't name.
    Other,
}

/// Liftoff.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, JsonSchema)]
pub struct LiftoffReading {
    /// The last sample on the pad, s: the rocket had risen less than the altitude's resolution
    /// then, and rose past it within one sample interval after.
    pub time_s: f64,
    /// Where it came from.
    pub source: ReadingSource,
}

/// The highest point.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, JsonSchema)]
pub struct ApogeeReading {
    /// When, s.
    pub time_s: f64,
    /// When, s after liftoff; `null` if liftoff was withheld.
    pub time_after_liftoff_s: Option<f64>,
    /// The filtered altitude there, m.
    pub altitude_m: f64,
    /// Whether the log may have ended before the peak, so the altitude is a floor.
    pub is_floor: bool,
    /// The highest sample the log holds before the filter: above the apogee when the median set
    /// a pulse aside.
    pub highest_sample: HighestSample,
    /// Where it came from.
    pub source: ReadingSource,
}

/// The highest sample of the altitude.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, JsonSchema)]
pub struct HighestSample {
    /// When, s.
    pub time_s: f64,
    /// The altitude, m.
    pub altitude_m: f64,
}

/// The top vertical speed from liftoff to apogee.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, JsonSchema)]
pub struct MaxSpeedReading {
    /// The speed, m/s, up.
    pub speed_m_s: f64,
    /// When, s.
    pub time_s: f64,
    /// The filtered altitude then, m.
    pub altitude_m: f64,
    /// Where it came from.
    pub source: ReadingSource,
}

/// The top acceleration.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, JsonSchema)]
pub struct MaxAccelerationReading {
    /// The acceleration, m/s².
    pub acceleration_m_s2: f64,
    /// When, s.
    pub time_s: f64,
}

/// Landing, and the descent before it.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, JsonSchema)]
pub struct LandingReading {
    /// The first sample within 2 m of the pad that stays under 5 m for a second, s: before
    /// touchdown by the time the last 2 m took.
    pub time_s: f64,
    /// From liftoff to landing, s.
    pub flight_time_s: f64,
    /// From apogee to landing, s.
    pub descent_time_s: f64,
    /// The mean rate of descent from apogee to landing, m/s: the height lost over the time
    /// taken, drogue and main together.
    pub mean_descent_rate_m_s: f64,
    /// Where it came from.
    pub source: ReadingSource,
}

/// `hpr completions`: a shell completion script.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, JsonSchema)]
pub struct Completions {
    /// The shell, such as `bash`.
    pub shell: String,
    /// The script.
    pub script: String,
}

/// What `--json` prints when a command fails.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, JsonSchema)]
pub struct ErrorDocument {
    /// The failure.
    pub error: ErrorBody,
}

/// A failure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, JsonSchema)]
pub struct ErrorBody {
    /// What kind of failure it is; the exit status says the same.
    pub kind: ErrorKind,
    /// What went wrong, for a person.
    pub message: String,
    /// The command that failed; `null` for a usage error, where the command line may name none.
    pub command: Option<String>,
    /// For `not_available`, the milestone that brings the command, such as `M4.2b`.
    pub milestone: Option<String>,
    /// What to do about it, each a sentence a person reads, the most useful last; empty when
    /// there is nothing to suggest. The text output writes each as a `help:` line.
    pub help: Vec<String>,
}

/// What kind of failure an error document reports, with its exit status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ErrorKind {
    /// Exit 1: an input was missing, unreadable or refused.
    Input,
    /// Exit 2: the command line was wrong.
    Usage,
    /// Exit 3: the command's milestone hasn't come yet.
    NotAvailable,
}

impl ErrorDocument {
    /// A failure of `kind`.
    pub fn new(
        kind: ErrorKind,
        message: impl Into<String>,
        command: Option<&str>,
        milestone: Option<&str>,
    ) -> Self {
        Self {
            error: ErrorBody {
                kind,
                message: message.into(),
                command: command.map(str::to_owned),
                milestone: milestone.map(str::to_owned),
                help: Vec::new(),
            },
        }
    }

    /// The same failure, with `help`: what to do about it.
    #[must_use]
    pub fn with_help(mut self, help: Vec<String>) -> Self {
        self.error.help = help;
        self
    }
}

/// The JSON Schema of each output, by the file name it is published under.
pub fn schemas() -> Vec<(&'static str, String)> {
    let mut schemas = vec![
        ("motors-list.schema.json", schema::<MotorList>()),
        ("motors-show.schema.json", schema::<MotorShow>()),
        ("motors-search.schema.json", schema::<MotorSearch>()),
        ("motors-fetch.schema.json", schema::<MotorFetch>()),
        ("sim.schema.json", schema::<SimFlight>()),
        ("sim-export-meta.schema.json", schema::<ExportMeta>()),
        ("mc.schema.json", schema::<McRun>()),
        ("mc-export-meta.schema.json", schema::<McExportMeta>()),
        ("completions.schema.json", schema::<Completions>()),
        ("convert.schema.json", schema::<Convert>()),
        ("analyze.schema.json", schema::<Analyze>()),
        ("weather.schema.json", schema::<Weather>()),
        ("error.schema.json", schema::<ErrorDocument>()),
    ];
    schemas.sort_by_key(|(name, _)| *name);
    schemas
}

/// One type's schema, pretty-printed with a trailing newline.
#[expect(
    clippy::expect_used,
    reason = "a `Schema` is a JSON value with string keys, which always serializes"
)]
fn schema<T: JsonSchema>() -> String {
    let mut schema = schemars::schema_for!(T);
    stamp_schema(&mut schema);
    let text = serde_json::to_string_pretty(&schema).expect("a JSON value serializes");
    format!("{text}\n")
}

/// Adds [`Stamped`]'s `tool` to a document's schema: a required property beside the document's
/// own. On a union, such as `hpr convert`'s, it sits beside the `anyOf`, so every branch has it.
fn stamp_schema(schema: &mut schemars::Schema) {
    let mut tool = schemars::schema_for!(Tool);
    tool.remove("$schema");
    let Some(root) = schema.as_object_mut() else {
        return;
    };
    let properties = root
        .entry("properties")
        .or_insert_with(|| serde_json::Value::Object(serde_json::Map::new()));
    if let Some(properties) = properties.as_object_mut() {
        properties.insert("tool".to_owned(), tool.to_value());
    }
    let required = root
        .entry("required")
        .or_insert_with(|| serde_json::Value::Array(Vec::new()));
    if let Some(required) = required.as_array_mut() {
        required.insert(0, serde_json::Value::from("tool"));
    }
}
