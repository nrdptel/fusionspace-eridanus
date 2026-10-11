//! Steps to a first answer (M0.11, ADR-209 §10, ADR-241): how many steps each tool takes to open a
//! design, fly it and get its landing spread, counted from `validation/steps/` for the
//! scoreboard.
//!
//! `validation/steps/steps.toml` lists each tool's three ways in: a command line, a script and a
//! window. For HPR Sim, the command line and the script are files beside it that a test runs
//! (`crates/hpr-py/tests/test_first_answer.py`), so every counted step works; for OpenRocket and
//! RocketPy, each step is listed with the page of the tool's documentation it comes from, with the
//! release and the date the pages were read. A way in that a tool doesn't have, or a task it
//! can't do that way, says so with its reason.
//!
//! The rules (ADR-241): a command is one command line; a script line is one statement, imports
//! included, never split over lines; a click is one press of a mouse button, a double click one;
//! an entry is one value typed, such as a file's name or a number. Clicks and entries are counted
//! apart, and apart from commands and script lines. The count to a task runs from a fresh start
//! (the tool open, or a shell or Python prompt, and the design's `.ork` file on disk) through the
//! last step its task names, so a step that serves two tasks counts once. The reader refuses what
//! it can't count without a judgment: a statement over two lines, an indented block, tasks out
//! of order, a step with no source.

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::scoreboard::Tool;

/// The record of every tool's steps.
pub(crate) const STEPS: &str = "validation/steps/steps.toml";

/// One of the three tasks, in the order they're done.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Task {
    /// The tool has read the design and holds it.
    Open,
    /// One flight, its apogee shown.
    Fly,
    /// Where the rocket may land: a spread of landing points over many flights, their inputs
    /// scattered.
    Spread,
}

impl Task {
    const ALL: [Self; 3] = [Self::Open, Self::Fly, Self::Spread];

    pub(crate) fn words(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Fly => "fly",
            Self::Spread => "spread",
        }
    }
}

/// A way into a tool.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum Interface {
    CommandLine,
    Script,
    Window,
}

impl Interface {
    const ALL: [Self; 3] = [Self::CommandLine, Self::Script, Self::Window];

    pub(crate) fn words(self) -> &'static str {
        match self {
            Self::CommandLine => "command line",
            Self::Script => "script",
            Self::Window => "window",
        }
    }

    /// The kinds of step this way in takes.
    fn kinds(self) -> &'static [StepKind] {
        match self {
            Self::CommandLine => &[StepKind::Command],
            Self::Script => &[StepKind::Line],
            Self::Window => &[StepKind::Click, StepKind::Entry],
        }
    }
}

/// A kind of step, each counted apart.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum StepKind {
    Command,
    Line,
    Click,
    Entry,
}

impl StepKind {
    pub(crate) fn words(self, count: usize) -> String {
        let (one, many) = match self {
            Self::Command => ("command", "commands"),
            Self::Line => ("line", "lines"),
            Self::Click => ("click", "clicks"),
            Self::Entry => ("entry", "entries"),
        };
        format!("{count} {}", if count == 1 { one } else { many })
    }
}

/// A page a step comes from.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Source {
    pub id: String,
    pub title: String,
    pub url: String,
}

/// One step.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Step {
    pub kind: StepKind,
    /// The command or line as typed, or what is clicked or typed.
    pub what: String,
    /// The id of the [`Source`] it comes from; a file's own steps name the file.
    pub source: String,
}

/// The steps that reach one or more tasks, or the tasks a way in can't reach.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Section {
    pub tasks: Vec<Task>,
    #[serde(default)]
    pub steps: Vec<Step>,
    /// Why these tasks can't be done this way: no step reaches them.
    #[serde(default)]
    pub cannot: Option<String>,
    /// The id of the source that shows it can't.
    #[serde(default)]
    pub source: Option<String>,
}

/// One way into one tool.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Way {
    pub interface: Interface,
    /// A file of HPR Sim's commands or script lines, run by a test, read into `sections`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    /// Why the tool has no such way in.
    #[serde(default)]
    pub cannot: Option<String>,
    #[serde(default)]
    pub source: Option<String>,
    /// The open milestone that brings the way in, for HPR Sim.
    #[serde(default)]
    pub milestone: Option<String>,
    #[serde(default)]
    pub section: Vec<Section>,
}

/// One tool's steps.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ToolSteps {
    pub tool: Tool,
    /// The release whose documentation was read; HPR Sim's files run on every commit instead.
    #[serde(default)]
    pub release: Option<String>,
    /// When that release came out.
    #[serde(default)]
    pub released: Option<String>,
    /// When the steps were counted.
    pub counted_on: String,
    #[serde(default)]
    pub source: Vec<Source>,
    pub way: Vec<Way>,
}

/// Every tool's steps, as `steps.toml` lists them with HPR Sim's files read in.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Record {
    pub tool: Vec<ToolSteps>,
}

/// What one way into one tool comes to.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Outcome {
    /// Every task done: the steps of each kind through the last.
    Done(Counts),
    /// Not every task: why, the steps of each kind through the last task it reaches (if any)
    /// and which that is, and the milestone that brings it, for HPR Sim.
    Cannot {
        reason: String,
        reached: Option<(Task, Counts)>,
        milestone: Option<String>,
    },
}

/// Steps of each kind.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct Counts(pub Vec<(StepKind, usize)>);

impl Counts {
    pub(crate) fn of(&self, kind: StepKind) -> usize {
        self.0
            .iter()
            .find(|(k, _)| *k == kind)
            .map_or(0, |(_, count)| *count)
    }
}

impl Record {
    /// Reads `steps.toml` under `root` and the files it names, and checks it.
    pub(crate) fn read(root: &Path) -> Result<Self, String> {
        let text = fs::read_to_string(root.join(STEPS))
            .map_err(|error| format!("reading {STEPS}: {error}"))?;
        let mut record: Self =
            toml::from_str(&text).map_err(|error| format!("{STEPS}: {error}"))?;
        for tool in &mut record.tool {
            for way in &mut tool.way {
                if let Some(file) = &way.file {
                    if !way.section.is_empty() {
                        return Err(format!(
                            "{STEPS}: {}'s {} reads {file} and lists steps too",
                            tool.tool.slug(),
                            way.interface.words()
                        ));
                    }
                    let text = fs::read_to_string(root.join(file))
                        .map_err(|error| format!("reading {file}: {error}"))?;
                    way.section = parse_file(file, &text, way.interface)?;
                }
            }
        }
        record.check()?;
        Ok(record)
    }

    /// Holds the record to the rules in the module's docs.
    fn check(&self) -> Result<(), String> {
        let tools: Vec<Tool> = self.tool.iter().map(|tool| tool.tool).collect();
        if tools != [Tool::Hpr, Tool::Openrocket, Tool::Rocketpy] {
            return Err(format!(
                "{STEPS}: lists {tools:?}; it needs HPR Sim, OpenRocket and RocketPy, in that order"
            ));
        }
        for tool in &self.tool {
            let name = tool.tool.slug();
            let at = |what: &str| format!("{STEPS}: {name}: {what}");
            if !is_date(&tool.counted_on) {
                return Err(at(&format!(
                    "counted_on `{}` isn't a date",
                    tool.counted_on
                )));
            }
            if tool.tool != Tool::Hpr
                && !(tool.release.is_some() && tool.released.as_deref().is_some_and(is_date))
            {
                return Err(at("needs the release read and the date it came out"));
            }
            let ids: BTreeSet<&str> = tool.source.iter().map(|s| s.id.as_str()).collect();
            if ids.len() != tool.source.len() {
                return Err(at("a source id twice"));
            }
            let mut used = BTreeSet::new();
            let ways: Vec<Interface> = tool.way.iter().map(|way| way.interface).collect();
            if ways != Interface::ALL {
                return Err(at(&format!(
                    "lists ways {ways:?}; it needs the command line, the script and the window, \
                     in that order"
                )));
            }
            for way in &tool.way {
                let at = |what: &str| at(&format!("{}: {what}", way.interface.words()));
                let cite = |source: &str, used: &mut BTreeSet<String>| {
                    if way.file.as_deref() == Some(source) || ids.contains(source) {
                        used.insert(source.to_owned());
                        Ok(())
                    } else {
                        Err(at(&format!("names a source `{source}` it doesn't list")))
                    }
                };
                if way.milestone.is_some() && (tool.tool != Tool::Hpr || way.cannot.is_none()) {
                    return Err(at(
                        "only a way into HPR Sim it doesn't have names a milestone",
                    ));
                }
                if way.file.is_some() && tool.tool != Tool::Hpr {
                    return Err(at("only HPR Sim's ways in are files a test runs"));
                }
                if let Some(cannot) = &way.cannot {
                    if !way.section.is_empty() || way.file.is_some() || cannot.trim().is_empty() {
                        return Err(at("a way in the tool doesn't have lists no steps"));
                    }
                    match (&way.source, &way.milestone) {
                        (Some(source), _) => cite(source, &mut used)?,
                        (None, Some(_)) => {}
                        (None, None) => return Err(at("says it can't, from no source")),
                    }
                    continue;
                }
                let tasks: Vec<Task> = way
                    .section
                    .iter()
                    .flat_map(|section| section.tasks.iter().copied())
                    .collect();
                if tasks != Task::ALL {
                    return Err(at(&format!(
                        "names tasks {tasks:?}; each of open, fly and spread once, in order"
                    )));
                }
                let last = way.section.len() - 1;
                for (index, section) in way.section.iter().enumerate() {
                    match (&section.cannot, section.steps.is_empty()) {
                        (None, true) => {
                            return Err(at(
                                "a section with no steps: name its tasks with the next one's",
                            ));
                        }
                        (Some(_), false) => return Err(at("a task it can't do lists steps")),
                        (Some(cannot), true) => {
                            if index != last || cannot.trim().is_empty() {
                                return Err(at("only the last section says it can't"));
                            }
                            let source = section
                                .source
                                .as_deref()
                                .ok_or_else(|| at("says it can't, from no source"))?;
                            cite(source, &mut used)?;
                        }
                        (None, false) => {
                            if section.source.is_some() {
                                return Err(at("a section that can names a source of its own"));
                            }
                        }
                    }
                    for step in &section.steps {
                        if !way.interface.kinds().contains(&step.kind) {
                            return Err(at(&format!("a {:?} step", step.kind)));
                        }
                        if step.what.trim().is_empty() {
                            return Err(at("a step says nothing"));
                        }
                        cite(&step.source, &mut used)?;
                    }
                }
            }
            if let Some(unused) = ids.iter().find(|id| !used.contains(**id)) {
                return Err(at(&format!("source `{unused}` backs no step")));
            }
        }
        Ok(())
    }

    /// A tool's steps.
    pub(crate) fn of(&self, tool: Tool) -> Option<&ToolSteps> {
        self.tool.iter().find(|steps| steps.tool == tool)
    }
}

impl ToolSteps {
    /// A way in.
    pub(crate) fn way(&self, interface: Interface) -> Option<&Way> {
        self.way.iter().find(|way| way.interface == interface)
    }
}

impl Way {
    /// What the way in comes to, counted.
    pub(crate) fn outcome(&self) -> Outcome {
        if let Some(reason) = &self.cannot {
            return Outcome::Cannot {
                reason: reason.clone(),
                reached: None,
                milestone: self.milestone.clone(),
            };
        }
        let mut counts = Counts(
            self.interface
                .kinds()
                .iter()
                .map(|kind| (*kind, 0))
                .collect(),
        );
        let mut reached = None;
        for section in &self.section {
            if let Some(reason) = &section.cannot {
                return Outcome::Cannot {
                    reason: reason.clone(),
                    reached,
                    milestone: None,
                };
            }
            for step in &section.steps {
                if let Some((_, count)) = counts.0.iter_mut().find(|(kind, _)| *kind == step.kind) {
                    *count += 1;
                }
            }
            reached = section.tasks.last().map(|task| (*task, counts.clone()));
        }
        Outcome::Done(counts)
    }
}

/// Whether `text` is a date, `YYYY-MM-DD`.
fn is_date(text: &str) -> bool {
    let parts: Vec<&str> = text.split('-').collect();
    matches!(parts[..], [y, m, d] if y.len() == 4 && m.len() == 2 && d.len() == 2
        && parts.iter().all(|part| part.bytes().all(|b| b.is_ascii_digit())))
}

/// The sections of one of HPR Sim's step files: a `# task: open, fly` line starts one, naming the
/// tasks its steps reach; any other line starting with `#`, and a blank line, is skipped; every
/// other line is one step. A line is refused if it continues onto the next (a trailing `\`, an
/// open bracket or a triple quote) or is indented, since a step is one whole command or
/// statement.
pub(crate) fn parse_file(
    file: &str,
    text: &str,
    interface: Interface,
) -> Result<Vec<Section>, String> {
    let kind = match interface {
        Interface::CommandLine => StepKind::Command,
        Interface::Script => StepKind::Line,
        Interface::Window => return Err(format!("{file}: a window's steps can't be a file")),
    };
    let mut sections: Vec<Section> = Vec::new();
    for (index, line) in text.lines().enumerate() {
        let at = |what: &str| format!("{file}:{}: {what}", index + 1);
        if let Some(tasks) = line.strip_prefix("# task:") {
            let tasks = tasks
                .split(',')
                .map(|task| match task.trim() {
                    "open" => Ok(Task::Open),
                    "fly" => Ok(Task::Fly),
                    "spread" => Ok(Task::Spread),
                    other => Err(at(&format!("`{other}` isn't open, fly or spread"))),
                })
                .collect::<Result<Vec<_>, _>>()?;
            sections.push(Section {
                tasks,
                steps: Vec::new(),
                cannot: None,
                source: None,
            });
            continue;
        }
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        if line.starts_with(char::is_whitespace) {
            return Err(at(
                "an indented line: write each step as one whole statement",
            ));
        }
        if !whole(line) {
            return Err(at(
                "a step that runs onto the next line: write it as one line, so it counts as one",
            ));
        }
        let section = sections
            .last_mut()
            .ok_or_else(|| at("a step before any `# task:` line"))?;
        section.steps.push(Step {
            kind,
            what: line.trim_end().to_owned(),
            source: file.to_owned(),
        });
    }
    Ok(sections)
}

/// Whether `line` is a whole command or statement: its brackets close outside its quoted text, it
/// opens no triple-quoted text, and it doesn't end in a `\` that joins the next line.
fn whole(line: &str) -> bool {
    if line.contains("\"\"\"") || line.contains("'''") || line.trim_end().ends_with('\\') {
        return false;
    }
    let mut depth = 0_i32;
    let mut quote = None;
    let mut escaped = false;
    for c in line.chars() {
        match quote {
            Some(q) => {
                if escaped {
                    escaped = false;
                } else if c == '\\' {
                    escaped = true;
                } else if c == q {
                    quote = None;
                }
            }
            None => match c {
                '"' | '\'' => quote = Some(c),
                '#' => break,
                '(' | '[' | '{' => depth += 1,
                ')' | ']' | '}' => {
                    depth -= 1;
                    if depth < 0 {
                        return false;
                    }
                }
                _ => {}
            },
        }
    }
    depth == 0 && quote.is_none()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record() -> Record {
        let root = crate::designs::root().expect("the repository root");
        Record::read(&root).expect("the committed steps")
    }

    /// HPR Sim's counts, from the files the Python tests run: one command opens and flies, a
    /// second gets the spread; two script lines open, three fly, four get the spread.
    #[test]
    fn hpr_sims_steps_are_counted_from_its_files() {
        let record = record();
        let hpr = record.of(Tool::Hpr).expect("HPR Sim's steps");
        let cli = hpr.way(Interface::CommandLine).expect("its command line");
        assert_eq!(
            cli.outcome(),
            Outcome::Done(Counts(vec![(StepKind::Command, 2)]))
        );
        let script = hpr.way(Interface::Script).expect("its script");
        assert_eq!(
            script.outcome(),
            Outcome::Done(Counts(vec![(StepKind::Line, 9)]))
        );
        let tasks: Vec<_> = script
            .section
            .iter()
            .map(|s| (s.tasks.clone(), s.steps.len()))
            .collect();
        assert_eq!(
            tasks,
            [
                (vec![Task::Open], 2),
                (vec![Task::Fly], 3),
                (vec![Task::Spread], 4)
            ]
        );
        assert!(matches!(
            hpr.way(Interface::Window).expect("its window").outcome(),
            Outcome::Cannot {
                milestone: Some(_),
                ..
            }
        ));
    }

    #[test]
    fn a_step_over_two_lines_is_refused() {
        let file = "steps.py";
        for (text, line) in [
            ("# task: open, fly, spread\nrun = f(a,\n  b)\n", 2),
            ("# task: open, fly, spread\nhpr mc x.ork \\\n --runs 3\n", 2),
            ("# task: open, fly, spread\ns = \"\"\"a\nb\"\"\"\n", 2),
            ("# task: open, fly, spread\nif x:\n    y()\n", 3),
        ] {
            let error = parse_file(file, text, Interface::Script).expect_err(text);
            assert!(error.starts_with(&format!("steps.py:{line}: ")), "{error}");
        }
        let fine = "# task: open, fly, spread\nprint(f\"{a['b)']:.1f}\")  # (\n";
        let sections = parse_file(file, fine, Interface::Script).expect("one whole line");
        assert_eq!(sections[0].steps.len(), 1);
    }

    #[test]
    fn a_step_before_its_task_or_a_task_unknown_is_refused() {
        let error = parse_file("f", "hpr sim x.ork\n# task: open\n", Interface::CommandLine)
            .expect_err("a step before its task");
        assert!(error.contains("before any `# task:`"), "{error}");
        let error = parse_file(
            "f",
            "# task: open, land\nhpr sim x\n",
            Interface::CommandLine,
        )
        .expect_err("an unknown task");
        assert!(error.contains("`land` isn't"), "{error}");
    }

    /// The record's checks fail closed: each mutation of the committed record is refused by the
    /// rule it breaks.
    #[test]
    fn the_record_refuses_what_it_cant_count() {
        let committed = record();
        let refused = |change: &dyn Fn(&mut Record), expected: &str| {
            let mut record = committed.clone();
            change(&mut record);
            let error = record.check().expect_err(expected);
            assert!(error.contains(expected), "{expected}: {error}");
        };
        let openrocket = 1;
        refused(&|r| r.tool.swap(1, 2), "in that order");
        refused(
            &|r| r.tool[openrocket].released = None,
            "the date it came out",
        );
        refused(
            &|r| r.tool[openrocket].counted_on = "Oct 11".to_owned(),
            "isn't a date",
        );
        refused(
            &|r| {
                r.tool[openrocket].way.pop();
            },
            "in that order",
        );
        refused(
            &|r| {
                let way = r.tool[openrocket]
                    .way
                    .iter_mut()
                    .find(|w| !w.section.is_empty())
                    .unwrap();
                way.section[0].steps[0].source = "nowhere".to_owned();
            },
            "doesn't list",
        );
        refused(
            &|r| {
                let way = r.tool[openrocket]
                    .way
                    .iter_mut()
                    .find(|w| !w.section.is_empty())
                    .unwrap();
                way.section[0].steps[0].kind = StepKind::Line;
            },
            "a Line step",
        );
        refused(
            &|r| {
                let way = r.tool[openrocket]
                    .way
                    .iter_mut()
                    .find(|w| !w.section.is_empty())
                    .unwrap();
                way.section.swap(0, 1);
            },
            "once, in order",
        );
        refused(
            &|r| {
                let source = r.tool[openrocket].source[0].clone();
                r.tool[openrocket].source.push(Source {
                    id: "spare".to_owned(),
                    ..source
                });
            },
            "backs no step",
        );
        refused(
            &|r| r.tool[openrocket].way[0].milestone = Some("M9.1".to_owned()),
            "milestone",
        );
    }

    #[test]
    fn a_way_that_cant_counts_what_it_reached() {
        let section = |tasks: Vec<Task>, steps: usize, cannot: Option<&str>| Section {
            tasks,
            steps: (0..steps)
                .map(|n| Step {
                    kind: if n % 2 == 0 {
                        StepKind::Click
                    } else {
                        StepKind::Entry
                    },
                    what: "x".to_owned(),
                    source: "s".to_owned(),
                })
                .collect(),
            cannot: cannot.map(str::to_owned),
            source: cannot.map(|_| "s".to_owned()),
        };
        let way = Way {
            interface: Interface::Window,
            file: None,
            cannot: None,
            source: None,
            milestone: None,
            section: vec![
                section(vec![Task::Open], 3, None),
                section(vec![Task::Fly], 2, None),
                section(vec![Task::Spread], 0, Some("no Monte Carlo")),
            ],
        };
        let Outcome::Cannot {
            reason,
            reached: Some((task, counts)),
            milestone: None,
        } = way.outcome()
        else {
            panic!("{:?}", way.outcome());
        };
        assert_eq!((reason.as_str(), task), ("no Monte Carlo", Task::Fly));
        assert_eq!(
            (counts.of(StepKind::Click), counts.of(StepKind::Entry)),
            (3, 2)
        );
    }

    #[test]
    fn dates() {
        assert!(is_date("2026-10-11"));
        assert!(!is_date("2026-1-11"));
        assert!(!is_date("2026-10-1x"));
    }
}
