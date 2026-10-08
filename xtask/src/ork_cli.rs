//! `cargo xtask ork-cli [--fetch] [--check]`: flies every motor configuration of every `.ork` in
//! `cargo xtask ork`'s survey through `hpr sim` itself, offline, and writes how many fly (M4.5j).
//!
//! The survey is [`crate::ork::default_survey`]: every `.ork` under `refs/` but the generated
//! scratch tree and the private flight collection, then the example designs inside the pinned
//! OpenRocket jar. Each configuration is flown by `hpr_cli::run`, the function the `hpr` binary
//! is a `main` around, as `hpr sim --json --offline --config ID FILE`: exit 0 is a flight. A
//! configuration refused as saved is flown again with `--accept-design-errors`; one that flies
//! then is counted as held back only by the design's checks. One refused both ways is counted
//! under the reason the reader gives for leaving it out as `hpr sim` reads the file, its fetched
//! curves from the cache ([`hpr_cli::sim::left_out`]), a hybrid motor (out of scope until
//! release 1.0, `CONTRIBUTING.md` rule 6) before any other, or as refused after reading when the
//! reader left nothing out.
//!
//! "Offline" is the cache's state: a motor with no curve in the file or the bundled catalog flies
//! only once `hpr sim` has fetched it from ThrustCurve.org. `--fetch` flies each configuration
//! once with the network first, which fills the cache; the counts are then taken offline. This is
//! M4.5's "offline after one fetch".
//!
//! The design library under `refs/` is other people's (CONTRIBUTING.md rule 4), so the report holds
//! counts only, by where a design came from ([`crate::ork_corpus_flights::source`]), except for
//! OpenRocket's own examples, which are listed by file and configuration. A file found in two
//! places is counted in each. `--check` flies the survey again and fails unless the committed
//! report is what it writes.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use hpr_cli::Exit;
use hpr_io::ork::{self, NoCurve, NotFlown};
use serde::{Deserialize, Serialize};

pub const USAGE: &str = "\
  ork-cli [--fetch] [--check]
                           Fly every motor configuration of the .ork survey through
                           `hpr sim --offline` and write how many fly, by source and by
                           reason, to validation/reports/ork-cli-flights.{json,md}. --fetch
                           flies each with the network first, to fill the motor cache;
                           --check fails unless the committed report is unchanged.";

/// The counts, committed.
pub(crate) const REPORT_JSON: &str = "validation/reports/ork-cli-flights.json";
/// The same, for a person.
pub(crate) const REPORT_MD: &str = "validation/reports/ork-cli-flights.md";
/// Where the jar's examples are written, for `hpr sim` to read them as files.
const EXAMPLES_DIR: &str = "target/ork-cli";
/// The source the jar's examples are counted under ([`crate::ork_corpus_flights::source`]).
const EXAMPLES: &str = "OpenRocket's examples";

/// The reason a configuration with a hybrid motor is refused.
const HYBRID: &str = "a hybrid motor (out of scope until release 1.0)";
/// The reason a configuration the reader left nothing out of was refused anyway.
const AFTER_READING: &str = "refused by hpr sim after reading";
/// A curve-less motor whose file records no digest: `hpr sim` supplies a fetched curve only by
/// the digest, so it fetches none.
const NO_DIGEST: &str = "a motor with no curve whose file records no digest, so none is fetched";
/// A motor whose curve was found and can't be used, or that names nothing to look up.
const UNUSABLE: &str = "a motor whose curve can't be looked up or used";
/// A curve-less motor with a digest whose curve isn't in the cache after the fetch.
const NOT_FETCHED: &str =
    "a motor with no curve fetched: ThrustCurve.org had none by its name, or the fetch failed";

/// How one configuration fared.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Outcome {
    /// `hpr sim` flew it as saved.
    Flies,
    /// Only with `--accept-design-errors`: the design's checks found an error.
    FliesWithFlag,
    /// Not at all, for this reason.
    Refused(String),
}

/// The counts for one source.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Counts {
    /// `.ork` files found.
    pub files: usize,
    /// Of those, files the reader refused.
    pub unreadable: usize,
    /// Of those read, files holding no motor configuration (`hpr sim` flies them only with
    /// `--motor`).
    pub no_configuration: usize,
    /// Motor configurations in the files read.
    pub configurations: usize,
    /// Flown as saved.
    pub flies: usize,
    /// Flown only with `--accept-design-errors`.
    pub flies_with_flag: usize,
    /// Refused both ways, by reason.
    pub refused: BTreeMap<String, usize>,
}

impl Counts {
    fn add(&mut self, outcome: &Outcome) {
        self.configurations += 1;
        match outcome {
            Outcome::Flies => self.flies += 1,
            Outcome::FliesWithFlag => self.flies_with_flag += 1,
            Outcome::Refused(why) => *self.refused.entry(why.clone()).or_default() += 1,
        }
    }

    fn refused_total(&self) -> usize {
        self.refused.values().sum()
    }
}

/// One configuration of one of OpenRocket's examples.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Example {
    /// The file's name inside the jar.
    pub file: String,
    /// Its number in the file's list, as `hpr sim --config` takes it.
    pub configuration: usize,
    /// Its motors' designations, in the file's order.
    pub motors: String,
    /// How it fared.
    pub outcome: Outcome,
}

/// The report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Report {
    /// What this is.
    pub about: String,
    /// The whole survey.
    pub total: Counts,
    /// By source.
    pub sources: BTreeMap<String, Counts>,
    /// OpenRocket's examples, each configuration.
    pub examples: Vec<Example>,
}

const ABOUT: &str = "Every motor configuration of every .ork in `cargo xtask ork`'s survey, flown \
                     by `hpr sim --offline` after one fetch. Written by `cargo xtask ork-cli`.";

pub(crate) fn run(args: &[String]) -> Result<(), String> {
    let mut fetch = false;
    let mut check = false;
    for argument in args {
        match argument.as_str() {
            "--fetch" => fetch = true,
            "--check" => check = true,
            _ => return Err(format!("usage:\n{USAGE}")),
        }
    }
    let root = crate::ork::root()?;
    let files = crate::ork::default_survey(&root)?;
    if files.is_empty() {
        return Err(
            "no .ork files found: the reference library is fetched by `cargo xtask refs fetch`"
                .to_owned(),
        );
    }
    let report = survey(&root, &files, fetch)?;
    let json = serde_json::to_string_pretty(&report).map_err(|error| error.to_string())? + "\n";
    let markdown = markdown(&report);
    if check {
        let committed = |name: &str| {
            fs::read_to_string(root.join(name)).map_err(|error| format!("{name}: {error}"))
        };
        if committed(REPORT_JSON)? != json || committed(REPORT_MD)? != markdown {
            return Err(format!(
                "{REPORT_JSON} is not what `hpr sim` flies today, or the motor cache lacks a \
                 curve it fetched: run `cargo xtask ork-cli --fetch` once with the network, then \
                 `cargo xtask ork-cli`, and commit the report if it changed"
            ));
        }
    } else {
        fs::write(root.join(REPORT_JSON), &json)
            .map_err(|error| format!("{REPORT_JSON}: {error}"))?;
        fs::write(root.join(REPORT_MD), &markdown)
            .map_err(|error| format!("{REPORT_MD}: {error}"))?;
    }
    println!("{}", headline(&report.total));
    for (source, counts) in &report.sources {
        println!(
            "  {source}: {} files, {} configurations: {} fly, {} more with the flag, {} refused",
            counts.files,
            counts.configurations,
            counts.flies,
            counts.flies_with_flag,
            counts.refused_total()
        );
    }
    Ok(())
}

/// Flies every configuration of `files`.
fn survey(root: &Path, files: &[crate::ork::Case], fetch: bool) -> Result<Report, String> {
    let examples_dir = root.join(EXAMPLES_DIR);
    fs::create_dir_all(&examples_dir)
        .map_err(|error| format!("{}: {error}", examples_dir.display()))?;
    let mut total = Counts::default();
    let mut sources: BTreeMap<String, Counts> = BTreeMap::new();
    let mut examples = Vec::new();
    for (path, bytes) in files {
        let relative = relative(root, path);
        let source = crate::ork_corpus_flights::source(&relative);
        let counts = sources.entry(source.to_owned()).or_default();
        counts.files += 1;
        total.files += 1;
        let Ok(file) = ork::read(bytes) else {
            counts.unreadable += 1;
            total.unreadable += 1;
            continue;
        };
        let design = ork::design(&file.value).value;
        let configurations = &design.motors.configurations;
        if configurations.is_empty() {
            counts.no_configuration += 1;
            total.no_configuration += 1;
            continue;
        }
        // A jar entry is written out for `hpr sim` to read; a file on disk is flown where it is.
        let on_disk = match relative.split_once(".jar!") {
            Some((_, entry)) => {
                let name = Path::new(entry)
                    .file_name()
                    .ok_or_else(|| format!("{relative}: no file name"))?;
                let written = examples_dir.join(name);
                fs::write(&written, bytes)
                    .map_err(|error| format!("{}: {error}", written.display()))?;
                written
            }
            None => Path::new(path).to_path_buf(),
        };
        let on_disk = on_disk.display().to_string();
        for (index, configuration) in configurations.iter().enumerate() {
            if fetch {
                // The network pass only fills the cache; its outcome is not counted.
                fly(&on_disk, &configuration.id, &[])?;
            }
            let outcome = if fly(&on_disk, &configuration.id, &["--offline"])? {
                Outcome::Flies
            } else if fly(
                &on_disk,
                &configuration.id,
                &["--offline", "--accept-design-errors"],
            )? {
                Outcome::FliesWithFlag
            } else {
                let why = hpr_cli::sim::left_out(&on_disk, &configuration.id)?;
                Outcome::Refused(reason(configuration, why).to_owned())
            };
            counts.add(&outcome);
            total.add(&outcome);
            if source == EXAMPLES {
                examples.push(Example {
                    file: Path::new(&on_disk)
                        .file_name()
                        .map(|name| name.to_string_lossy().into_owned())
                        .unwrap_or_default(),
                    configuration: index + 1,
                    motors: configuration
                        .motors
                        .iter()
                        .map(|motor| motor.designation.as_str())
                        .collect::<Vec<_>>()
                        .join("; "),
                    outcome,
                });
            }
        }
    }
    examples.sort_by(|a, b| (&a.file, a.configuration).cmp(&(&b.file, b.configuration)));
    Ok(Report {
        about: ABOUT.to_owned(),
        total,
        sources,
        examples,
    })
}

/// A path as the survey names it, relative to the repository root.
fn relative(root: &Path, path: &str) -> String {
    let root = root.display().to_string();
    path.strip_prefix(&root)
        .map(|rest| rest.trim_start_matches(['/', '\\']))
        .unwrap_or(path)
        .replace('\\', "/")
}

/// Whether `hpr sim --json` with `extra` flies configuration `id` of `file`: true on exit 0,
/// false on exit 1 (an input refused). Any other exit is a fault in this command, not an outcome.
fn fly(file: &str, id: &str, extra: &[&str]) -> Result<bool, String> {
    let mut args = vec!["hpr", "sim", "--json", "--config", id];
    args.extend_from_slice(extra);
    args.extend(["--", file]);
    let mut out = Vec::new();
    let mut err = Vec::new();
    match hpr_cli::run(args, &mut out, &mut err) {
        Exit::Success => Ok(true),
        Exit::Failure => Ok(false),
        // With `--json` the error document is on standard output.
        exit => Err(format!(
            "`hpr sim` exited {exit:?} on a configuration of a surveyed file: {}",
            String::from_utf8_lossy(&out).trim()
        )),
    }
}

/// Why a configuration refused both ways was: a hybrid motor first, then `why`, the reader's
/// reason as `hpr sim` reads the file ([`hpr_cli::sim::left_out`]), a missing curve split by its
/// cause in the file.
fn reason(configuration: &ork::MotorConfiguration, why: Option<NotFlown>) -> &'static str {
    let motors: Vec<MotorCurve> = configuration
        .motors
        .iter()
        .map(|motor| MotorCurve {
            hybrid: is_hybrid(motor.kind.as_deref()),
            unresolved: match motor.curve {
                ork::Curve::Unresolved { why, .. } => Some(why),
                _ => None,
            },
            digest: motor.digest.is_some(),
        })
        .collect();
    reason_of(&motors, why)
}

/// What one motor of a refused configuration says of its curve, in the file.
#[derive(Debug, Clone, Copy, Default)]
struct MotorCurve {
    /// Its `<type>` is a hybrid.
    hybrid: bool,
    /// Why the reader found no curve, if it found none.
    unresolved: Option<NoCurve>,
    /// The file records the curve's digest.
    digest: bool,
}

/// Whether a `.ork` motor's `<type>` says it is a hybrid.
fn is_hybrid(kind: Option<&str>) -> bool {
    kind.is_some_and(|kind| kind.trim().eq_ignore_ascii_case("hybrid"))
}

/// [`reason`], from the configuration's motors and the reader's reason for leaving it out.
fn reason_of(motors: &[MotorCurve], why: Option<NotFlown>) -> &'static str {
    if motors
        .iter()
        .any(|m| m.hybrid || m.unresolved == Some(NoCurve::Hybrid))
    {
        return HYBRID;
    }
    match why {
        None => AFTER_READING,
        Some(NotFlown::NoCurve) => {
            if motors
                .iter()
                .any(|m| m.unresolved == Some(NoCurve::NotFound) && !m.digest)
            {
                NO_DIGEST
            } else if motors
                .iter()
                .any(|m| m.unresolved.is_some_and(|w| w != NoCurve::NotFound))
            {
                UNUSABLE
            } else {
                NOT_FETCHED
            }
        }
        Some(NotFlown::UnreadMotor) => "a motor in a part hpr doesn't read",
        Some(NotFlown::NoMotor) => "no motor",
        Some(NotFlown::InactiveStage) => "a stage switched off",
        Some(NotFlown::NoSize) => "a motor of no stated size",
        Some(NotFlown::IgnitionNotFlown) => "an ignition hpr doesn't fly",
        Some(NotFlown::AirframeNotAsWritten) => "the airframe not read as written",
        Some(NotFlown::SeparationNotFlown) => "a separation hpr doesn't fly",
        Some(_) => "a reason this survey doesn't name",
    }
}

/// The report's one sentence, which the docs quote word for word.
pub(crate) fn headline(total: &Counts) -> String {
    format!(
        "`hpr sim` flies {} of the survey's {} motor configurations as saved, offline after one \
         fetch, and {} more with `--accept-design-errors`; {} are refused.",
        total.flies,
        total.configurations,
        total.flies_with_flag,
        total.refused_total()
    )
}

/// The sentence on OpenRocket's examples, which the docs quote word for word too.
pub(crate) fn examples_line(report: &Report) -> String {
    let examples = report.sources.get(EXAMPLES).cloned().unwrap_or_default();
    format!(
        "Of OpenRocket's {} example configurations, {} fly as saved, {} more with \
         `--accept-design-errors`, and {} are refused.",
        examples.configurations,
        examples.flies,
        examples.flies_with_flag,
        examples.refused_total()
    )
}

fn markdown(report: &Report) -> String {
    let mut out = String::from(
        "# The `.ork` survey through `hpr sim`\n\n\
         Written by `cargo xtask ork-cli` (M4.5j); don't edit it by hand. Every motor \
         configuration of every `.ork` in `cargo xtask ork`'s survey (the design library under \
         `refs/` and the examples inside OpenRocket 24.12's jar) is flown by `hpr sim --offline`, \
         once the motors it fetches from ThrustCurve.org are cached. A configuration refused as \
         saved is flown again with `--accept-design-errors`. The library is other people's \
         designs, so it is counted, not listed; a file found in two places is counted in each.\n\n",
    );
    out.push_str(&headline(&report.total));
    out.push(' ');
    out.push_str(&examples_line(report));
    out.push_str("\n\n## By source\n\n");
    out.push_str(
        "| Source | Files | Unreadable | No configuration | Configurations | Fly | Fly with the \
         flag | Refused |\n|---|---:|---:|---:|---:|---:|---:|---:|\n",
    );
    let row = |name: &str, c: &Counts| {
        format!(
            "| {name} | {} | {} | {} | {} | {} | {} | {} |\n",
            c.files,
            c.unreadable,
            c.no_configuration,
            c.configurations,
            c.flies,
            c.flies_with_flag,
            c.refused_total()
        )
    };
    for (source, counts) in &report.sources {
        out.push_str(&row(source, counts));
    }
    out.push_str(&row("**All**", &report.total));
    out.push_str("\n## Refused, by reason\n\n| Reason | Configurations |\n|---|---:|\n");
    for (why, n) in &report.total.refused {
        out.push_str(&format!("| {why} | {n} |\n"));
    }
    out.push_str(
        "\n## OpenRocket's examples\n\n| File | Configuration | Motors | Outcome |\n|---|---:|---|---|\n",
    );
    for example in &report.examples {
        let outcome = match &example.outcome {
            Outcome::Flies => "flies".to_owned(),
            Outcome::FliesWithFlag => "flies with `--accept-design-errors`".to_owned(),
            Outcome::Refused(why) => format!("refused: {why}"),
        };
        out.push_str(&format!(
            "| {} | {} | {} | {outcome} |\n",
            example.file, example.configuration, example.motors
        ));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn report() -> Report {
        let text = fs::read_to_string(crate::ork::root().unwrap().join(REPORT_JSON)).unwrap();
        serde_json::from_str(&text).unwrap()
    }

    #[test]
    fn the_committed_report_adds_up() {
        let report = report();
        let mut sum = Counts::default();
        for counts in report.sources.values() {
            assert_eq!(
                counts.configurations,
                counts.flies + counts.flies_with_flag + counts.refused_total()
            );
            sum.files += counts.files;
            sum.unreadable += counts.unreadable;
            sum.no_configuration += counts.no_configuration;
            sum.configurations += counts.configurations;
            sum.flies += counts.flies;
            sum.flies_with_flag += counts.flies_with_flag;
            for (why, n) in &counts.refused {
                *sum.refused.entry(why.clone()).or_default() += n;
            }
        }
        assert_eq!(sum, report.total);
        let listed = report.examples.len();
        assert_eq!(
            Some(listed),
            report.sources.get(EXAMPLES).map(|c| c.configurations)
        );
    }

    #[test]
    fn the_committed_markdown_is_the_json_rendered() {
        let report = report();
        let root = crate::ork::root().unwrap();
        let committed = fs::read_to_string(root.join(REPORT_MD)).unwrap();
        assert_eq!(committed, markdown(&report));
    }

    #[test]
    fn the_docs_quote_the_headline() {
        let report = report();
        let root = crate::ork::root().unwrap();
        let page = fs::read_to_string(root.join("docs/format/ork.md")).unwrap();
        // The page wraps its prose, so it is compared with its lines joined.
        let joined = page.split_whitespace().collect::<Vec<_>>().join(" ");
        for sentence in [headline(&report.total), examples_line(&report)] {
            assert!(
                joined.contains(&sentence),
                "docs/format/ork.md doesn't quote the report: {sentence}"
            );
        }
    }

    #[test]
    fn a_hybrid_is_named_before_the_readers_reason() {
        assert!(is_hybrid(Some(" Hybrid ")));
        assert!(!is_hybrid(Some("single-use")));
        assert!(!is_hybrid(None));
        let by_type = MotorCurve {
            hybrid: true,
            ..MotorCurve::default()
        };
        let by_curve = MotorCurve {
            unresolved: Some(NoCurve::Hybrid),
            digest: true,
            ..MotorCurve::default()
        };
        for hybrid in [by_type, by_curve] {
            assert_eq!(reason_of(&[hybrid], Some(NotFlown::NoCurve)), HYBRID);
            assert_eq!(reason_of(&[hybrid], None), HYBRID);
        }
        assert_eq!(reason_of(&[MotorCurve::default()], None), AFTER_READING);
    }

    #[test]
    fn a_missing_curve_is_split_by_its_cause() {
        let missing = |digest| MotorCurve {
            unresolved: Some(NoCurve::NotFound),
            digest,
            ..MotorCurve::default()
        };
        let curved = MotorCurve {
            digest: true,
            ..MotorCurve::default()
        };
        let unusable = MotorCurve {
            unresolved: Some(NoCurve::Unusable),
            digest: true,
            ..MotorCurve::default()
        };
        let no_curve = Some(NotFlown::NoCurve);
        assert_eq!(reason_of(&[curved, missing(false)], no_curve), NO_DIGEST);
        // No digest is named before the other causes, whichever motor comes first.
        assert_eq!(
            reason_of(&[missing(true), missing(false)], no_curve),
            NO_DIGEST
        );
        assert_eq!(reason_of(&[unusable, missing(false)], no_curve), NO_DIGEST);
        assert_eq!(reason_of(&[missing(true), unusable], no_curve), UNUSABLE);
        assert_eq!(reason_of(&[curved, missing(true)], no_curve), NOT_FETCHED);
    }

    #[test]
    fn a_path_is_named_from_the_root() {
        let root = Path::new("/w/hpr");
        assert_eq!(
            relative(root, "/w/hpr/refs/loft-fixtures/a.ork"),
            "refs/loft-fixtures/a.ork"
        );
        assert_eq!(
            crate::ork_corpus_flights::source(&relative(
                root,
                "/w/hpr/refs/openrocket/OpenRocket-24.12.jar!datafiles/examples/x.ork"
            )),
            EXAMPLES
        );
    }
}
