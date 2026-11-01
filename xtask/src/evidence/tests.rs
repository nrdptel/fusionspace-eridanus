//! The evidence check's tests: a clean register and pages pass, and each rule fails a minimal
//! change to them by its own message.

use super::pages::{EXCLUDED, PHRASES};
use super::samples::TestRun;
use super::*;

/// A register that keeps every rule: a row short of its wanted kind with an open issue, a row at
/// a wanted kind a decision record sets, and a real row with its sample.
const REGISTER_TEXT: &str = r#"
as_of = "2026-10-17"

[[row]]
id = "G1"
surface = "Flight-log readers"
claim = "GPX tracks: built from the schemas, checked against gpxpy on an invented file"
pages = ["format/logs.md#gpx"]
kind = "code-to-code"
wanted = "real"
not_checked = "what a writer puts in `ele`"
issue = 645

[[row.searched]]
on = "2026-10-17"
looked = ["the trackers' download pages", "public repositories of flight logs"]
found = "no openly downloadable GPX of a real rocket flight"

[[row]]
id = "G2"
surface = "The motor finder"
claim = "Replayed on stand-in searches"
pages = ["motors.md#the-finder"]
kind = "spec"
wanted = "spec"
wanted_adr = "ADR-145"

[[row]]
id = "G3"
surface = "Flight-log readers"
claim = "A flight's CSV, from the flight itself"
pages = ["format/logs.md#csv"]
kind = "real"
wanted = "real"

[[row.searched]]
on = "2026-09-01"
looked = ["a club's archive"]
found = "one CSV with its license"

[[row.sample]]
path = "refs/logs/flight.csv"
sha256 = "b009e155888c4c667894e60956a8ed19699a06e0bdd628d0efc07fd13feea3ce"
license = "CC-BY-4.0, from a club's archive"
package = "fusionspace-hpr-flightdata"
target = "lib"
test = "csv::tests::reads_a_real_flight"
"#;

/// The bytes whose SHA-256 the register's sample gives.
const SAMPLE_BYTES: &[u8] = b"time,altitude\n0,0\n";

/// A page in a directory, linking two rows from it.
const LOGS: &str = "# Flight logs

## GPX

The GPX reader is checked on invented files only ([evidence row G1](../evidence.md#g1)).

## CSV

A real flight's file backs the CSV reader ([G3](../evidence.md#g3)).
";

/// A page at the top, linking one row.
const MOTORS: &str = "# Motors

## The finder

The finder replays recorded searches ([G2](evidence.md#g2)).
";

/// The sentences the 2026-10-17 survey found stating each of ADR-278's 24 gaps on the guide's
/// pages (ADR-278 §2's fixture), one per row of its table, as (row, sentence).
const SURVEYED: [(&str, &str); 24] = [
    (
        "G1",
        "No sample log in the collection is a GPX file, so the reader is checked only on three invented files, against gpxpy 1.6.2, an outside GPX reader.",
    ),
    ("G2", "Decimal commas are tested on invented files only."),
    (
        "G3",
        "Left out: the eleven packet types no sample holds; and the GPS fields nothing checks.",
    ),
    (
        "G4",
        "Assumed: that the TeleMetrum v1's converter measures against the pressure sensor's own supply, which no document states; the exports of its flights agree.",
    ),
    (
        "G5",
        "Assumed: the rate, 50 records a second, which DataAnalyzer gives all three files.",
    ),
    (
        "G6",
        "Of the raw downloads, 7 of their 63 sample files have none, and are unchecked.",
    ),
    (
        "G7",
        "The speeds, accelerations, times and the rest are converted from the unit the file writes and checked against nothing.",
    ),
    (
        "G8",
        "Heights from KML files are the least sure: one writer's are feet written as meters, kept as written, and two files rise less than half as high as the barometer on the same flight did.",
    ),
    (
        "G9",
        "Three of the formats the roadmap names have no sample among the logs the survey reads.",
    ),
    (
        "G10",
        "How well it is validated: against an invented flight whose every reading is known exactly, each reading lands within the bound the log's rounding and the filter allow.",
    ),
    (
        "G11",
        "Every example below reads one invented flight's files, its logger's and its GPS tracker's; it flew nowhere.",
    ),
    (
        "G12",
        "What it leaves out: Dryden is an aircraft model, unvalidated for rockets.",
    ),
    (
        "G13",
        "The flight of a dropped booster or middle stage after its split is not validated at all.",
    ),
    ("G14", "Base drag under power is unvalidated."),
    (
        "G15",
        "HPR Sim flies the turn that thrust makes about a center of gravity off the axis; OpenRocket's flights don't, and nothing measures which is right.",
    ),
    (
        "G16",
        "Which direction the angle turns is assumed, and is not settled.",
    ),
    (
        "G17",
        "From hpr sim's defaults it does not, so the apogee hpr sim prints assumes the unstable sustainer stays upright. Why the two launches differ is not traced.",
    ),
    (
        "G18",
        "The two rules this milestone adds, an inner tube's automatic outer radius and a ring's automatic bore, have no oracle coverage at all. They rest on their unit tests and on the argument for them.",
    ),
    (
        "G19",
        "Only up to Mach 0.75 on real data. Only one real export has been flown, Calisto's. Faster than that, the table is checked by unit tests alone.",
    ),
    ("G20", "These are assumptions, not measurements (ADR-086)."),
    (
        "G21",
        "Four of the seven altimeters' kinds are assumed, none is calibrated, and the designs' fin edges are guesses, so read the numbers as those seven flights', not as a bound on every rocket.",
    ),
    (
        "G22",
        "His unfailed wings flew to at least Mach 1.3; nothing here is checked against a hobby rocket.",
    ),
    (
        "G23",
        "That is checked on recorded answers, below: eight from motor.fusionspace.co and two curve files from ThrustCurve.org, beside three stand-in searches in ThrustCurve.org's shape.",
    ),
    (
        "G24",
        "Against the logs of seven real flights, HPR Sim's apogees miss by 6.04% on average, outside the 5% target, and by −8.90% to +10.40% one by one.",
    ),
];

/// The decision records the register's rows may name.
fn decision_files() -> BTreeMap<u32, String> {
    BTreeMap::from([
        (145, "0145-thrustcurve-terms.md".to_owned()),
        (
            278,
            "0278-an-evidence-register-for-every-claim.md".to_owned(),
        ),
    ])
}

/// The report's page for `register`, its report current.
fn page_for(register: &str) -> String {
    let register = Register::parse(register).unwrap();
    let region = report::region(&register, &decision_files()).unwrap();
    format!("# Evidence\n\nWhat backs each claim.\n\n{region}\n\nMore words.\n")
}

/// Everything the check reads, owned, so a test can change one piece.
struct Case {
    register: String,
    pages: BTreeMap<String, String>,
    decisions: BTreeMap<u32, String>,
    page: Option<String>,
    schema: Option<String>,
    issues: Option<IssueStates>,
}

impl Case {
    /// The case that passes.
    fn clean() -> Self {
        Self {
            register: REGISTER_TEXT.to_owned(),
            pages: BTreeMap::from([
                ("format/logs.md".to_owned(), LOGS.to_owned()),
                ("motors.md".to_owned(), MOTORS.to_owned()),
            ]),
            decisions: decision_files(),
            page: Some(page_for(REGISTER_TEXT)),
            schema: Some(schema()),
            issues: Some(IssueStates::from([(645, IssueState::Open)])),
        }
    }

    /// The clean case with its register changed from `from` to `to`.
    fn register(from: &str, to: &str) -> Self {
        Self {
            register: swap(REGISTER_TEXT, from, to),
            ..Self::clean()
        }
    }

    /// The clean case with the page `name`'s text changed from `from` to `to`.
    fn page(name: &str, from: &str, to: &str) -> Self {
        let mut case = Self::clean();
        let text = case.pages.get_mut(name).unwrap();
        *text = swap(text, from, to);
        case
    }

    fn check(&self) -> Outcome {
        check(&Inputs {
            register: &self.register,
            pages: &self.pages,
            decisions: &self.decisions,
            page: self.page.as_deref(),
            schema: self.schema.as_deref(),
            issues: self.issues.as_ref(),
        })
    }

    /// Asserts the check finds exactly one problem, and that it holds `expected`.
    fn fails_only(&self, expected: &str) {
        let problems = self.check().problems;
        assert!(
            problems.len() == 1 && problems[0].contains(expected),
            "expected only {expected:?}, found {problems:#?}"
        );
    }

    /// Asserts the check finds no problem, with the report's page written from this case's
    /// register.
    fn passes(mut self) {
        self.page = Some(page_for(&self.register));
        let problems = self.check().problems;
        assert!(problems.is_empty(), "{problems:#?}");
    }

    /// Asserts one of the check's problems holds `expected`.
    fn fails(&self, expected: &str) {
        let problems = self.check().problems;
        assert!(
            problems.iter().any(|problem| problem.contains(expected)),
            "{expected:?} not in {problems:#?}"
        );
    }
}

/// `text` with its first `from` made `to`; a `from` that isn't there fails the test, so a
/// mutation can't silently change nothing.
fn swap(text: &str, from: &str, to: &str) -> String {
    assert!(text.contains(from), "{from:?} isn't in {text}");
    text.replacen(from, to, 1)
}

/// The clean register and pages pass, with nothing left unchecked.
#[test]
fn evidence_a_clean_register_and_pages_pass() {
    assert_eq!(Case::clean().check(), Outcome::default());
}

/// Rule 1: a block that states a gap and links no row fails, at its line, with the link to add
/// written from the page's directory.
#[test]
fn evidence_a_gap_with_no_row_fails() {
    Case::page(
        "motors.md",
        "searches ([G2](evidence.md#g2)).\n",
        "searches ([G2](evidence.md#g2)).\n\nIts thrust curves are assumed.\n",
    )
    .fails_only(
        "docs/motors.md:7: states a gap (\"are assumed\") but links no row of \
         validation/evidence.toml (link evidence.md#g<N>)",
    );
    Case::page("format/logs.md", "## CSV\n", "## CSV\n\nNo sample yet.\n").fails_only(
        "docs/format/logs.md:9: states a gap (\"no sample\") but links no row of \
         validation/evidence.toml (link ../evidence.md#g<N>)",
    );
}

/// Rule 2: a link to a row the register doesn't have fails.
#[test]
fn evidence_a_link_to_a_missing_row_fails() {
    Case::page(
        "motors.md",
        "searches ([G2](evidence.md#g2)).\n",
        "searches ([G2](evidence.md#g2)).\n\nSee [the register](evidence.md#g9).\n",
    )
    .fails_only("docs/motors.md:7: links row G9, which validation/evidence.toml doesn't have");
}

/// Rule 3: what TOML or the types refuse fails as the register's one problem.
#[test]
fn evidence_a_register_that_does_not_parse_fails() {
    Case::register("kind = \"spec\"", "kind = \"guess\"").fails_only("validation/evidence.toml: ");
    Case::register("kind = \"spec\"", "kind = \"guess\"").fails("unknown variant `guess`");
    Case::register("issue = 645\n", "issue = 645\nshade = \"red\"\n").fails("unknown field");
    Case::register("[[row.sample]]\n", "[[row.sample]]\nsize = 3\n").fails("unknown field");
}

/// Rule 3: each of the register's rules fails a register that breaks it, by its own message,
/// and as a problem of the register alone where the change touches nothing else.
#[test]
fn evidence_each_register_rule_fails_its_row() {
    let alone = |from: &str, to: &str, expected: &str| {
        let case = Case::register(from, to);
        case.fails(expected);
        let problems = case.check().problems;
        assert!(
            (problems.iter()).all(|problem| problem.starts_with("validation/evidence.toml: ")),
            "{problems:#?}"
        );
    };
    // Ids: their form, once each, in order. These change what the pages link too.
    Case::register("id = \"G2\"", "id = \"G02\"").fails("G02: `G02` isn't `G` and a number");
    Case::register("id = \"G2\"", "id = \"g2\"").fails("`g2` isn't `G` and a number");
    Case::register("id = \"G3\"", "id = \"G1\"").fails("G1: listed twice");
    Case::register("id = \"G2\"", "id = \"G4\"")
        .fails("G3: comes after G4: the rows go in increasing order");
    // The fields a row must fill.
    alone(
        "claim = \"Replayed on stand-in searches\"",
        "claim = \" \"",
        "G2: has no claim",
    );
    alone(
        "surface = \"The motor finder\"",
        "surface = \"\"",
        "G2: has no surface",
    );
    alone(
        "claim = \"Replayed on stand-in searches\"",
        "claim = \"Replayed on\\nstand-in searches\"",
        "G2: its claim holds a line break",
    );
    alone(
        "found = \"one CSV with its license\"",
        "found = \"one CSV\\r\\nwith its license\"",
        "G3: its found holds a line break",
    );
    Case::register("pages = [\"motors.md#the-finder\"]", "pages = []")
        .fails("G2: lists no page that states its claim");
    // The decision record behind a wanted kind other than real.
    alone(
        "wanted_adr = \"ADR-145\"\n",
        "",
        "G2: wants spec evidence, not real, so it needs wanted_adr",
    );
    alone(
        "issue = 645\n",
        "issue = 645\nwanted_adr = \"ADR-278\"\n",
        "G1: wants real evidence, which needs no wanted_adr, but names ADR-278",
    );
    alone(
        "wanted_adr = \"ADR-145\"",
        "wanted_adr = \"ADR-14\"",
        "G2: wanted_adr `ADR-14` isn't `ADR-` and 3 or 4 digits",
    );
    alone(
        "wanted_adr = \"ADR-145\"",
        "wanted_adr = \"ADR-999\"",
        "G2: wanted_adr ADR-999 names no record in docs/decisions/",
    );
    // A row short of its wanted kind says what isn't checked and names its issue.
    alone(
        "not_checked = \"what a writer puts in `ele`\"\n",
        "",
        "G1: has code-to-code evidence and wants real, but says nothing in not_checked",
    );
    alone(
        "not_checked = \"what a writer puts in `ele`\"\n",
        "not_checked = \" \"\n",
        "says nothing in not_checked",
    );
    alone(
        "issue = 645\n",
        "",
        "G1: has code-to-code evidence and wants real, but names no issue",
    );
    // Samples: only on a real row, under refs/, with their hash and their test.
    alone(
        "kind = \"real\"",
        "kind = \"sibling\"",
        "G3: lists samples, but its kind is sibling: only a real row has samples",
    );
    alone(
        "path = \"refs/logs/flight.csv\"",
        "path = \"logs/flight.csv\"",
        "G3: sample logs/flight.csv: its path isn't under refs/",
    );
    alone(
        "path = \"refs/logs/flight.csv\"",
        "path = \"refs/../secret.csv\"",
        "its path isn't under refs/",
    );
    alone(
        "sha256 = \"b009",
        "sha256 = \"B009",
        "isn't 64 lowercase hex digits",
    );
    alone(
        "sha256 = \"b009e155888c4c667894e60956a8ed19699a06e0bdd628d0efc07fd13feea3ce\"",
        "sha256 = \"b009e155\"",
        "isn't 64 lowercase hex digits",
    );
    for field in ["license", "package", "target", "test"] {
        let line = (REGISTER_TEXT.lines())
            .find(|line| line.starts_with(&format!("{field} = ")))
            .unwrap();
        alone(line, &format!("{field} = \"\""), &format!("has no {field}"));
    }
    // What goes to cargo and the test harness is a name, never an option.
    for field in ["package", "target", "test"] {
        let line = (REGISTER_TEXT.lines())
            .find(|line| line.starts_with(&format!("{field} = ")))
            .unwrap();
        for value in ["--include-ignored", "a b", "x;y"] {
            alone(
                line,
                &format!("{field} = \"{value}\""),
                &format!("its {field} `{value}` isn't a name"),
            );
        }
    }
    // Dates: real ones, searches oldest first and none after as_of.
    alone(
        "as_of = \"2026-10-17\"",
        "as_of = \"2026-02-30\"",
        "as_of `2026-02-30` isn't a date",
    );
    alone(
        "on = \"2026-09-01\"",
        "on = \"2026-9-1\"",
        "G3: its search on `2026-9-1` isn't on a date",
    );
    alone(
        "found = \"no openly downloadable GPX of a real rocket flight\"\n",
        "found = \"no openly downloadable GPX of a real rocket flight\"\n\n[[row.searched]]\n\
         on = \"2026-10-01\"\nlooked = [\"a forum\"]\nfound = \"nothing\"\n",
        "G1: its search on 2026-10-01 isn't after the one before it",
    );
    alone(
        "found = \"no openly downloadable GPX of a real rocket flight\"\n",
        "found = \"no openly downloadable GPX of a real rocket flight\"\n\n[[row.searched]]\n\
         on = \"2026-10-17\"\nlooked = [\"a forum\"]\nfound = \"nothing\"\n",
        "G1: its search on 2026-10-17 isn't after the one before it",
    );
    alone(
        "on = \"2026-10-17\"",
        "on = \"2026-10-18\"",
        "G1: its search on 2026-10-18 is after as_of, 2026-10-17",
    );
    // Where a search looked: a list of places, not empty (and none of them empty, below) nor
    // broken across lines.
    alone(
        "looked = [\"a club's archive\"]",
        "looked = []",
        "G3: its search on 2026-09-01 needs both where it looked and what it found",
    );
    alone(
        "found = \"one CSV with its license\"",
        "found = \" \"",
        "G3: its search on 2026-09-01 needs both where it looked and what it found",
    );
    alone(
        "looked = [\"a club's archive\"]",
        "looked = [\"a club's\\narchive\"]",
        "G3: its looked holds a line break",
    );
}

/// Rule 4: a row's page must be one the check reads, and its anchor must be on it.
#[test]
fn evidence_a_row_page_whose_anchor_is_gone_fails() {
    Case::register("\"format/logs.md#gpx\"", "\"format/logs.md#gpx-tracks\"").fails_only(
        "validation/evidence.toml: G1: its page format/logs.md#gpx-tracks: docs/format/logs.md \
         has no anchor #gpx-tracks",
    );
    Case::register(
        "\"format/logs.md#gpx\"",
        "\"format/logs.md#gpx\", \"plan.md#steps\"",
    )
    .fails_only("G1: its page plan.md#steps: docs/plan.md isn't a page the check reads");
    Case::register(
        "\"format/logs.md#gpx\"",
        "\"format/logs.md#gpx\", \"format/logs.md\"",
    )
    .fails_only("G1: its page `format/logs.md` needs the anchor of its section");
}

/// Rule 5: the pages that link a row are the pages it lists, both ways.
#[test]
fn evidence_pages_and_links_must_agree() {
    // A page links a row that doesn't list it.
    Case::page(
        "motors.md",
        "searches ([G2](evidence.md#g2)).\n",
        "searches ([G2](evidence.md#g2)).\n\nSee [G1](evidence.md#g1).\n",
    )
    .fails_only("G1: docs/motors.md:7 links it, but its pages don't list motors.md");
    // A page a row lists doesn't link it.
    Case::register(
        "\"format/logs.md#gpx\"",
        "\"format/logs.md#gpx\", \"motors.md#the-finder\"",
    )
    .fails_only("G1: its page docs/motors.md never links it: link evidence.md#g1 where");
    // No page links the row.
    Case::page("motors.md", " ([G2](evidence.md#g2))", "")
        .fails_only("G2: no page links it: link evidence.md#g2 where");
}

/// Rule 6: a row short of its wanted kind needs its issue open and labeled `needs-sample`; an
/// open one passes, and `--offline` checks the rest, saying it didn't check the issues.
#[test]
fn evidence_a_short_row_needs_an_open_issue() {
    let mut case = Case::clean();
    case.issues = Some(IssueStates::new());
    case.fails_only("G1: its issue #645 isn't among the issues labeled `needs-sample`: label it");
    case.issues = Some(IssueStates::from([(645, IssueState::Closed)]));
    case.fails_only("G1: its issue #645 is closed, but the row has code-to-code evidence");
    case.issues = Some(IssueStates::from([
        (645, IssueState::Open),
        (1, IssueState::Closed),
    ]));
    assert_eq!(case.check(), Outcome::default());
    case.issues = None;
    let outcome = case.check();
    assert!(outcome.problems.is_empty(), "{:#?}", outcome.problems);
    assert_eq!(outcome.notes.len(), 1);
    assert!(
        outcome.notes[0].starts_with("the issues' states were not checked (--offline)"),
        "{:?}",
        outcome.notes
    );
    // Offline, a closed issue goes unseen: the note is the only sign.
    case.issues = None;
    assert!(case.check().problems.is_empty());
}

/// Rule 7: a real row with no sample fails by its own message.
#[test]
fn evidence_a_real_row_needs_its_sample() {
    let start = REGISTER_TEXT.find("[[row.sample]]").unwrap();
    let mut case = Case::clean();
    case.register = REGISTER_TEXT[..start].to_owned();
    case.fails_only(
        "validation/evidence.toml: G3: its kind is real but it lists no sample with its SHA-256 \
         and the test that reads it",
    );
}

/// Rule 8: the report must be what the register writes now, on a page that holds it once.
#[test]
fn evidence_a_stale_report_fails() {
    let mut case = Case::clean();
    let page = case.page.take().unwrap();
    case.page = Some(swap(&page, "holds 3 rows", "holds 4 rows"));
    case.fails_only(
        "docs/evidence.md's report is not what validation/evidence.toml writes now; run `cargo \
         xtask evidence --write`",
    );
    case.page = None;
    case.fails_only("docs/evidence.md is missing");
    case.page = Some(format!("{page}\n{REGION_END}\n"));
    case.fails_only("docs/evidence.md must hold `<!-- evidence: end -->` once");
    // The register's own change stales it too.
    let mut case = Case::register("claim = \"Replayed on", "claim = \"Played on");
    case.fails_only("docs/evidence.md's report is not what validation/evidence.toml writes now");
    case.page = Some(page_for(&case.register));
    assert_eq!(case.check(), Outcome::default());
}

/// Rule 9: the committed schema must be what the types write now.
#[test]
fn evidence_a_stale_schema_fails() {
    let mut case = Case::clean();
    case.schema = Some(swap(
        &schema(),
        "\"title\": \"Register\"",
        "\"title\": \"Old\"",
    ));
    case.fails_only(
        "schema/validation/evidence.schema.json is not what the register's types write now; run \
         `cargo xtask evidence --write`",
    );
    case.schema = None;
    case.fails_only("schema/validation/evidence.schema.json is missing");
    // A Windows checkout's line ends don't make it stale.
    case.schema = Some(schema().replace('\n', "\r\n"));
    assert_eq!(case.check(), Outcome::default());
}

/// Each phrase states a gap on a canary page, and linking a row there settles it.
#[test]
fn evidence_each_phrase_fires_on_a_canary_page() {
    for phrase in PHRASES {
        let words = phrase.trim_end_matches('*');
        let text = format!("# Canary\n\nThis reader rests on {words} today.\n");
        let block = pages::blocks(&text).pop().unwrap();
        assert!(
            pages::first_match(&block.text.to_lowercase(), phrase).is_some(),
            "{phrase:?} isn't in {block:?}"
        );
        let reading = read_page("canary.md", &text);
        assert_eq!(reading.problems.len(), 1, "{phrase:?}: {reading:?}");
        assert!(
            reading.problems[0].starts_with("docs/canary.md:3: states a gap (\""),
            "{phrase:?}: {reading:?}"
        );
        let linked = format!(
            "# Canary\n\nThis reader rests on {words} today ([evidence row G1](evidence.md#g1)).\n"
        );
        let reading = read_page("canary.md", &linked);
        assert!(reading.problems.is_empty(), "{phrase:?}: {reading:?}");
        assert_eq!(reading.links, [("G1".to_owned(), 3)]);
    }
}

/// Each sentence of ADR-278's survey ([`SURVEYED`]), alone in a paragraph with no link, states a
/// gap the check finds.
#[test]
fn evidence_the_surveyed_sentences_are_detected() {
    for (at, (id, sentence)) in SURVEYED.iter().enumerate() {
        assert_eq!(*id, format!("G{}", at + 1));
        let reading = read_page("canary.md", &format!("# Canary\n\n{sentence}\n"));
        assert_eq!(
            reading.problems.len(),
            1,
            "{id}: {sentence:?} isn't detected"
        );
    }
}

/// The phrase list, pinned: removing a phrase takes a decision record amending ADR-278 (ADR-278
/// §2), and this test changes with it.
#[test]
fn evidence_the_phrase_list_is_pinned() {
    assert_eq!(PHRASES.len(), 23);
    assert_eq!(
        PHRASES,
        [
            "no sample",
            "invented files only",
            "three invented files",
            "against an invented",
            "one invented flight",
            "assumed:",
            "is assumed",
            "are assumed",
            "unvalidated",
            "not validated",
            "checked against nothing",
            "least sure",
            "nothing measures which",
            "nothing here is checked",
            "no oracle coverage",
            "tests alone",
            "one real",
            "stand-in searches",
            "outside the 5% target",
            "rests on one",
            "unchecked",
            "assumptions, not measurements",
            "differ is not traced",
        ]
    );
    let mut seen = BTreeSet::new();
    for phrase in PHRASES {
        assert_eq!(phrase, phrase.to_lowercase(), "{phrase:?} isn't lowercase");
        assert_eq!(phrase, phrase.trim(), "{phrase:?} has spaces at an end");
        assert!(seen.insert(phrase), "{phrase:?} is listed twice");
    }
}

/// Phrases match whole words in any case, a curly apostrophe as a straight one, and `*` takes
/// any ending.
#[test]
fn evidence_phrases_match_whole_words() {
    assert_eq!(pages::gap("Assumed: the rate"), Some("assumed:"));
    assert_eq!(pages::gap("(no sample)"), Some("no sample"));
    assert_eq!(pages::gap("presumed"), None);
    assert_eq!(pages::gap("it is assumedly so"), None);
    assert_eq!(pages::gap("no samples"), None);
    assert_eq!(
        pages::gap("Assumed, for now"),
        None,
        "only `assumed:` stands alone"
    );
    assert_eq!(
        pages::gap("unvalidated, and the rate is assumed"),
        Some("unvalidated"),
        "the first in the text"
    );
    assert_eq!(
        pages::gap("outside the 5% target"),
        Some("outside the 5% target")
    );
    assert_eq!(pages::gap("outside the 15% target"), None);
    // Curly apostrophes read as straight ones.
    assert_eq!(
        pages::gap_in("it isn\u{2019}t checked", &["isn't checked"]),
        Some("isn't checked")
    );
    // A trailing `*` takes any ending, and no beginning.
    let invent = ["invent*"];
    assert_eq!(pages::gap_in("Invented here", &invent), Some("invent*"));
    assert_eq!(pages::gap_in("an invention", &invent), Some("invent*"));
    assert_eq!(pages::gap_in("reinvented", &invent), None);
    // On a tie, the longest.
    assert_eq!(
        pages::gap_in("no sample yet", &["no", "no sample"]),
        Some("no sample")
    );
}

/// A phrase in quotation marks, straight or curly, quotes another document and states no gap; the
/// same phrase outside them, in the same block, does. A quote with no partner drops nothing.
#[test]
fn evidence_quoted_phrases_state_no_gap() {
    let quoted = "The standard says the mass \"is assumed and should not be specified\".";
    assert_eq!(pages::gap(quoted), None);
    assert_eq!(
        pages::gap(&format!("{quoted} The rate is assumed here.")),
        Some("is assumed")
    );
    let curly = "Its manual: \u{201c}is assumed\u{201d}, it says.";
    assert_eq!(pages::gap(curly), None);
    assert_eq!(
        pages::gap(&format!("{curly} Ours is assumed too.")),
        Some("is assumed")
    );
    assert_eq!(pages::gap("A 3\" tube is assumed."), Some("is assumed"));
    assert_eq!(
        pages::gap("\u{201c}The rate is assumed."),
        Some("is assumed")
    );
    assert_eq!(pages::unquoted("a \"b\" c \"d"), "a   c \"d");
    let page = "# Quotes\n\nIts manual says \"is assumed\" of the mass.\n";
    assert!(read_page("quotes.md", page).problems.is_empty());
}

/// Rule 10: a block that states a gap and links the bare list of issues fails, a row's link or
/// not; a link to a numbered issue doesn't.
#[test]
fn evidence_a_gap_asks_in_its_row_s_issue() {
    let list = "https://github.com/nrdptel/fusionspace-eridanus/issues";
    let asks = format!("searches ([G2](evidence.md#g2)); no sample yet: [send one]({list}).\n");
    Case::page("motors.md", "searches ([G2](evidence.md#g2)).\n", &asks).fails_only(
        "docs/motors.md:5: asks for a sample in the issue list; link the row's issue instead",
    );
    let label = "https://github.com/nrdptel/fusionspace-eridanus/labels/needs-sample";
    for dest in [
        format!("{list}/"),
        format!("{list}?q=label%3Aneeds-sample"),
        label.to_owned(),
    ] {
        let asks = format!("searches ([G2](evidence.md#g2)); no sample yet: [send one]({dest}).\n");
        Case::page("motors.md", "searches ([G2](evidence.md#g2)).\n", &asks)
            .fails_only("asks for a sample in the issue list");
    }
    // With no row linked as well, both rules fire.
    let reading = read_page("x.md", &format!("No sample yet: [send one]({list}).\n"));
    assert_eq!(reading.problems.len(), 2, "{reading:?}");
    // A numbered issue, or the list in a block that states no gap, passes.
    let numbered = format!("searches ([G2](evidence.md#g2)); no sample yet: [#645]({list}/645).\n");
    let case = Case::page("motors.md", "searches ([G2](evidence.md#g2)).\n", &numbered);
    assert_eq!(case.check(), Outcome::default());
    assert!(
        read_page("x.md", &format!("Report a bug in [the issues]({list}).\n"))
            .problems
            .is_empty()
    );
    assert!(!pages::is_issue_list(&format!("{list}/645")));
    assert!(!pages::is_issue_list(
        "https://github.com/someone/else/issues"
    ));
}

/// Two headings of one text take GitHub's numbered anchors, `notes` then `notes-1`.
#[test]
fn evidence_repeated_headings_are_numbered() {
    let anchors = crate::site::anchors_of("# Notes\n\nText.\n\n## Notes\n\n<a id=\"g7\"></a>\n");
    for anchor in ["notes", "notes-1", "g7"] {
        assert!(anchors.contains(anchor), "{anchor} not in {anchors:?}");
    }
}

/// A tight list item, a paragraph in a list item or a quote, a table's rows and a raw HTML
/// element each form a block of their own; a code block's text is in none.
#[test]
fn evidence_blocks_split_as_a_reader_sees_them() {
    let page = "- a tight item with no sample
- another

1. a loose item

   its paragraph, assumed

> A quoted line, unvalidated.

| Head | Cell |
|---|---|
| a row | `no sample` |

<aside class=\"fs-note\">An invented file in a note, <a href=\"../evidence.md#g4\">G4</a></aside>

```text
no sample in code
```

    no sample, indented code
";
    let blocks = pages::blocks(page);
    let found: Vec<(usize, &str)> = (blocks.iter())
        .map(|block| (block.line, block.text.as_str()))
        .collect();
    assert_eq!(
        found,
        [
            (1, "a tight item with no sample"),
            (2, "another"),
            (4, "a loose item"),
            (6, "its paragraph, assumed"),
            (8, "A quoted line, unvalidated."),
            (10, "Head Cell"),
            (12, "a row no sample"),
            (14, "An invented file in a note, G4"),
        ]
    );
    let aside = blocks.last().unwrap();
    assert_eq!(aside.links, ["../evidence.md#g4"]);
    assert_eq!(
        pages::row_named("format/x.md", &aside.links[0]).as_deref(),
        Some("G4")
    );
    // A comment's text is hidden, a block's tags part words, and links count wherever they are.
    let html = pages::blocks("<div>assumed<!-- no sample --></div><div>x</div>\n");
    assert_eq!(html.len(), 1);
    assert_eq!(html[0].text, "assumed x");
    let linked = pages::blocks("Rests on *invented* files ([G2](../evidence.md#g2 \"row\")).\n");
    assert_eq!(linked[0].text, "Rests on invented files (G2).");
    assert_eq!(linked[0].links, ["../evidence.md#g2"]);
}

/// A link names a row when it resolves, from its page's directory, to the report's page with
/// `#g` and a number, in either case.
#[test]
fn evidence_links_resolve_from_the_page() {
    let named = |page: &str, dest: &str| pages::row_named(page, dest);
    assert_eq!(
        named("format/x.md", "../evidence.md#g1").as_deref(),
        Some("G1")
    );
    assert_eq!(named("format/x.md", "evidence.md#g1"), None);
    assert_eq!(named("x.md", "./evidence.md#g1").as_deref(), Some("G1"));
    // The report's anchors are lowercase, and a fragment's case counts.
    assert_eq!(named("x.md", "./evidence.md#G1"), None);
    assert_eq!(named("x.md", "evidence.md#g12").as_deref(), Some("G12"));
    assert_eq!(named("x.md", "evidence.md#gx"), None);
    assert_eq!(named("x.md", "evidence.md#g"), None);
    assert_eq!(named("x.md", "evidence.md"), None);
    assert_eq!(named("x.md", "#g1"), None);
    assert_eq!(named("x.md", "../evidence.md#g1"), None);
    assert_eq!(
        named("a/b/x.md", "../../evidence.md#g3").as_deref(),
        Some("G3")
    );
    assert_eq!(
        named("x.md", "https://example.org/docs/evidence.md#g1"),
        None
    );
    // `#g01` names no row the register can have, so the check fails it.
    assert_eq!(named("x.md", "evidence.md#g01").as_deref(), Some("G01"));
}

/// The pages the check reads: those the summary lists, once each, less the excluded ones.
#[test]
fn evidence_reads_the_summary_s_pages() {
    let summary = "# Summary\n\n[Start here](start-here.md)\n\n- [Logs](format/logs.md)\n  \
                   - [Plan](plan.md)\n- [Evidence](evidence.md)\n- [Again](start-here.md)\n\
                   - [Web](https://example.org/a.md)\n- [Writing](./writing.md)\n";
    assert_eq!(pages::scanned(summary), ["start-here.md", "format/logs.md"]);
    for excluded in [
        "decisions-and-roadmap.md",
        "plan.md",
        "writing.md",
        "releasing.md",
        "evidence.md",
    ] {
        assert!(EXCLUDED.contains(&excluded), "{excluded}");
    }
}

/// The report for the clean register, whole, with no percentage or combined score.
#[test]
fn evidence_the_report_reads_as_written() {
    let register = Register::parse(REGISTER_TEXT).unwrap();
    let region = report::region(&register, &decision_files()).unwrap();
    let expected = format!(
        "{REGION_START}

The register holds 3 rows, as of October 17, 2026.

**By kind.** Each surface's rows, counted by the kind of evidence behind them now:

| Surface | Real · rows | Code to code · rows | Spec · rows | Rows |
|---|---:|---:|---:|---:|
| Flight-log readers | 1 | 1 | 0 | 2 |
| The motor finder | 0 | 0 | 1 | 1 |
| All | 1 | 1 | 1 | 3 |

**Short of what's wanted.** 1 of the 3 rows doesn't yet have the kind of evidence it wants. It \
has an open issue labeled `needs-sample`, linked in its row below.

**The rows.**

| Row | Surface | Claim | Evidence | Wanted | Issue | Last search |
|---|---|---|---|---|---|---|
| <a id=\"g1\"></a>G1 | Flight-log readers | GPX tracks: built from the schemas, checked against \
gpxpy on an invented file. **Not checked:** what a writer puts in `ele`. | code to code | real | \
[#645](https://github.com/nrdptel/fusionspace-eridanus/issues/645) | 2026-10-17 |
| <a id=\"g2\"></a>G2 | The motor finder | Replayed on stand-in searches. | spec | spec \
([ADR-145](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/\
0145-thrustcurve-terms.md)) | none | none |
| <a id=\"g3\"></a>G3 | Flight-log readers | A flight's CSV, from the flight itself. | real | \
real | none | 2026-09-01 |

**Due a search.** A row is due one when it has none yet, or its last was more than 30 days \
before October 17, 2026:

- [G2](#g2)
- [G3](#g3)

**Searches so far.**

- [G1](#g1), October 17, 2026, looked at:
  - the trackers' download pages
  - public repositories of flight logs

  It found no openly downloadable GPX of a real rocket flight.
- [G3](#g3), September 1, 2026, looked at:
  - a club's archive

  It found one CSV with its license.

**Real samples.** Each file a real row reads, kept under `refs/`, with its SHA-256 and the test \
that reads it:

- [G3](#g3): `refs/logs/flight.csv`, SHA-256 \
`b009e155888c4c667894e60956a8ed19699a06e0bdd628d0efc07fd13feea3ce`; CC-BY-4.0, from a club's \
archive; read by `fusionspace-hpr-flightdata`'s test `csv::tests::reads_a_real_flight`.

{REGION_END}"
    );
    assert_eq!(region, expected);
    assert!(!region.contains('%'));
    assert!(!region.contains('\u{2014}'), "no em dash");
    // Each row's anchor is its id in lowercase, as the pages link it.
    for id in ["g1", "g2", "g3"] {
        assert!(crate::site::anchors_of(&region).contains(id), "{id}");
    }
    // A `|` in a cell is escaped, so it doesn't end the cell.
    let mut piped = register.clone();
    piped.row[1].claim = "a | b".to_owned();
    let region = report::region(&piped, &decision_files()).unwrap();
    assert!(region.contains("| a \\| b. |"), "{region}");
    // With no rows, the report says so and nothing else.
    let empty = Register::parse("as_of = \"2026-10-17\"\n").unwrap();
    assert_eq!(
        report::region(&empty, &decision_files()).unwrap(),
        format!(
            "{REGION_START}\n\nThe register holds no rows yet, as of October 17, 2026.\n\n\
             {REGION_END}"
        )
    );
}

/// Each place a search looked is an item of its own, in a list nested in the search's item, and
/// what it found is a paragraph of the search's item after that list, as the site's parser reads
/// the report; a search's list with an empty place in it fails the check.
#[test]
fn evidence_each_place_it_looked_is_its_own_item() {
    use pulldown_cmark::{Event, Parser, Tag, TagEnd};
    let register = Register::parse(REGISTER_TEXT).unwrap();
    let region = report::region(&register, &decision_files()).unwrap();
    let (_, searches) = region.split_once("**Searches so far.**").unwrap();
    let (searches, _) = searches.split_once("**Real samples.**").unwrap();
    // Each item's own text, by its depth: 1 for a search, 2 for a place it looked. Each block
    // of an item's own text ends in a line break.
    let mut items: Vec<(usize, String)> = Vec::new();
    let mut open: Vec<usize> = Vec::new();
    for event in Parser::new_ext(searches, crate::site::options()) {
        match event {
            Event::Start(Tag::Item) => {
                items.push((open.len() + 1, String::new()));
                open.push(items.len() - 1);
            }
            Event::End(TagEnd::Item) => {
                let item = open.pop().unwrap();
                if !items[item].1.ends_with('\n') {
                    items[item].1.push('\n');
                }
            }
            Event::End(TagEnd::Paragraph) => {
                if let Some(&item) = open.last() {
                    items[item].1.push('\n');
                }
            }
            Event::Text(text) | Event::Code(text) => {
                if let Some(&item) = open.last() {
                    items[item].1.push_str(&text);
                }
            }
            _ => {}
        }
    }
    assert_eq!(
        items,
        [
            (
                1,
                "G1, October 17, 2026, looked at:\n\
                 It found no openly downloadable GPX of a real rocket flight.\n"
                    .to_owned()
            ),
            (2, "the trackers' download pages\n".to_owned()),
            (2, "public repositories of flight logs\n".to_owned()),
            (
                1,
                "G3, September 1, 2026, looked at:\nIt found one CSV with its license.\n"
                    .to_owned()
            ),
            (2, "a club's archive\n".to_owned()),
        ]
    );
    // A place in the register is one item in the report, whatever it holds.
    let mut wide = register.clone();
    wide.row[2].searched[0].looked = vec![
        "a club's archive, 2016 to 2024".to_owned(),
        "a forum".to_owned(),
    ];
    let region = report::region(&wide, &decision_files()).unwrap();
    assert!(
        region
            .contains("looked at:\n  - a club's archive, 2016 to 2024\n  - a forum\n\n  It found"),
        "{region}"
    );
    // An empty place, among others or alone, fails as an empty list does, and only it.
    for looked in [
        "[\"a club's archive\", \" \"]",
        "[\"\", \"a forum\"]",
        "[\"\"]",
    ] {
        Case::register(
            "looked = [\"a club's archive\"]",
            &format!("looked = {looked}"),
        )
        .fails_only("G3: its search on 2026-09-01 needs both where it looked and what it found");
    }
    // A place opens its item in the report's list, so one that Markdown would read as structure
    // fails, and one that only looks like it reads as text.
    for (place, what) in [
        ("- a forum", "a list marker"),
        ("* a forum", "a list marker"),
        ("1. a forum", "a numbered list marker"),
        ("12) a forum", "a numbered list marker"),
        ("# a forum", "a heading"),
        ("> a forum", "a quote"),
        ("---", "a thematic break"),
        ("_ _ _", "a thematic break"),
        ("```", "a code fence"),
        ("<div>a forum", "an HTML tag"),
    ] {
        Case::register(
            "looked = [\"a club's archive\"]",
            &format!("looked = [\"a club's archive\", \"{place}\"]"),
        )
        .fails_only(&format!(
            "G3: its search on 2026-09-01 has a place that opens as {what} in the report's list: \
             `{place}`"
        ));
    }
    for place in [
        "#645's thread",
        "1.5 m tubes",
        "-40 °C logs",
        "a forum - 2016",
        "2016 to 2024, a forum",
    ] {
        Case::register(
            "looked = [\"a club's archive\"]",
            &format!("looked = [\"{place}\"]"),
        )
        .passes();
    }
    // A place's spaces are the report's to set: leading ones would make it a code block.
    let mut spaced = register.clone();
    spaced.row[2].searched[0].looked = vec!["      a club's archive".to_owned()];
    let region = report::region(&spaced, &decision_files()).unwrap();
    assert!(
        region.contains("looked at:\n  - a club's archive\n"),
        "{region}"
    );
}

/// No sentence the report writes itself runs over 25 words, the writing guide's limit, however
/// many rows are short of their wanted kind or due a search: the rows due are a list.
#[test]
fn evidence_the_report_s_own_sentences_stay_short() {
    let longest = |register: &Register| -> usize {
        let region = report::region(register, &decision_files()).unwrap();
        // The report's own words, as a reader sees them: not the tables, nor what the searches
        // found.
        let (own, _) = region
            .split_once("**Searches so far.**")
            .unwrap_or((&region, ""));
        let own = own.replace("**", "");
        (own.lines())
            .filter(|line| !line.starts_with('|') && !line.starts_with("<!--"))
            .flat_map(|line| line.split(". "))
            .map(|sentence| sentence.split_whitespace().count())
            .max()
            .unwrap_or_default()
    };
    let mut register = Register::parse(REGISTER_TEXT).unwrap();
    assert!(longest(&register) <= 25, "{}", longest(&register));
    // 200 rows, all short of their wanted kind and all due a search.
    for number in 4..=200 {
        let mut row = register.row[0].clone();
        row.id = format!("G{number}");
        row.searched.clear();
        register.row.push(row);
    }
    assert!(longest(&register) <= 25, "{}", longest(&register));
    // One row, short of its wanted kind.
    register.row.truncate(1);
    assert!(longest(&register) <= 25, "{}", longest(&register));
}

/// A row is due a search with none yet, or with its last more than 30 days before as_of: 30
/// days isn't due, 31 is.
#[test]
fn evidence_searches_fall_due_after_30_days() {
    let due = |last: &str| {
        let text = swap(
            REGISTER_TEXT,
            "on = \"2026-10-17\"",
            &format!("on = \"{last}\""),
        );
        let text = swap(&text, "as_of = \"2026-10-17\"", "as_of = \"2026-10-31\"");
        let register = Register::parse(&text).unwrap();
        let ids: Vec<String> = (report::due(&register).unwrap().iter())
            .map(|row| row.id.clone())
            .collect();
        ids.contains(&"G1".to_owned())
    };
    assert_eq!(report::DUE_AFTER_DAYS, 30);
    assert!(!due("2026-10-01"), "30 days before isn't due");
    assert!(due("2026-09-30"), "31 days before is due");
    assert!(!due("2026-10-31"));
    let register = Register::parse(REGISTER_TEXT).unwrap();
    let ids: Vec<&str> = (report::due(&register).unwrap().iter())
        .map(|row| row.id.as_str())
        .collect();
    assert_eq!(ids, ["G2", "G3"], "G2 has no search, G3's is 46 days old");
}

/// The schema takes the register, as JSON, and refuses an unknown key, a kind it doesn't list
/// and an id of another form, as the reader does.
#[test]
fn evidence_the_schema_takes_the_register_and_refuses_strays() {
    let schema: serde_json::Value = serde_json::from_str(&schema()).unwrap();
    let validator = jsonschema::validator_for(&schema).unwrap();
    let json = |text: &str| -> serde_json::Value {
        let value: toml::Value = toml::from_str(text).unwrap();
        serde_json::to_value(value).unwrap()
    };
    let errors = |text: &str| -> Vec<String> {
        (validator.iter_errors(&json(text)))
            .map(|error| format!("{}: {error}", error.instance_path()))
            .collect()
    };
    assert_eq!(errors(REGISTER_TEXT), Vec::<String>::new());
    assert!(Register::parse(REGISTER_TEXT).is_ok());
    for (from, to) in [
        ("issue = 645\n", "issue = 645\nshade = \"red\"\n"),
        (
            "as_of = \"2026-10-17\"\n",
            "as_of = \"2026-10-17\"\nowner = \"x\"\n",
        ),
        ("kind = \"spec\"", "kind = \"guess\""),
        ("wanted = \"real\"", "wanted = \"code_to_code\""),
        ("id = \"G2\"", "id = \"G02\""),
        ("pages = [\"motors.md#the-finder\"]", "pages = []"),
        ("issue = 645", "issue = \"645\""),
        ("sha256 = \"b009", "sha256 = \"B009"),
        (
            "looked = [\"a club's archive\"]",
            "looked = \"a club's archive\"",
        ),
        ("looked = [\"a club's archive\"]", "looked = []"),
    ] {
        let text = swap(REGISTER_TEXT, from, to);
        assert!(!errors(&text).is_empty(), "the schema takes {to:?}");
    }
    // The reader refuses the same strays the schema's types do.
    for (from, to) in [
        ("issue = 645\n", "issue = 645\nshade = \"red\"\n"),
        ("kind = \"spec\"", "kind = \"guess\""),
        (
            "looked = [\"a club's archive\"]",
            "looked = \"a club's archive\"",
        ),
    ] {
        assert!(
            Register::parse(&swap(REGISTER_TEXT, from, to)).is_err(),
            "{to:?}"
        );
    }
}

/// `--samples`: a missing file, a differing hash, and a test that didn't run, was ignored,
/// skipped or failed each fail by their own message; a test that ran and passed passes.
#[test]
fn evidence_samples_fail_unless_their_test_ran_and_passed() {
    let register = Register::parse(REGISTER_TEXT).unwrap();
    let present = |_: &str| Ok(Some(SAMPLE_BYTES.to_vec()));
    let passed = "running 1 test\ntest csv::tests::reads_a_real_flight ... ok\n\n\
                  test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 41 filtered out; \
                  finished in 0.00s\n";
    let run_with = |success: bool, output: &str| {
        let run = TestRun {
            success,
            output: output.to_owned(),
        };
        samples::problems(&register, &present, &|_| Ok(run.clone()))
    };
    assert_eq!(run_with(true, passed), Vec::<String>::new());

    let missing = samples::problems(&register, &|_| Ok(None), &|_| unreachable!());
    assert_eq!(
        missing,
        ["validation/evidence.toml: G3: sample refs/logs/flight.csv: missing"]
    );
    let other = samples::problems(
        &register,
        &|_| Ok(Some(b"other".to_vec())),
        &|_| unreachable!(),
    );
    assert_eq!(other.len(), 1);
    assert!(
        other[0].ends_with(
            "hash differs: d9298a10d1b0735837dc4bd85dac641b0f3cef27a47e5d53a54f2f3f5b2fcffa"
        ),
        "{other:?}: the actual hash, so a changed file can be told from a typo"
    );

    let fails = |success: bool, output: &str, expected: &str| {
        let problems = run_with(success, output);
        assert!(
            problems.len() == 1 && problems[0].contains(expected),
            "{expected:?}: {problems:?}"
        );
    };
    fails(
        true,
        "running 0 tests\n\ntest result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 42 \
         filtered out\n",
        "its test did not run",
    );
    fails(
        true,
        &format!("{passed}\nskipped: refs/logs/flight.csv is not there\n"),
        "its test skipped",
    );
    fails(
        true,
        "running 1 test\ntest csv::tests::reads_a_real_flight ... ignored\n\n\
         test result: ok. 0 passed; 0 failed; 1 ignored; 0 measured; 41 filtered out\n",
        "its test did not run",
    );
    fails(
        false,
        "running 1 test\ntest csv::tests::reads_a_real_flight ... FAILED\n\n\
         test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 41 filtered out\n",
        "its test failed",
    );
    fails(
        true,
        "test result: ok. 1 passed; 0 failed; 1 ignored; 0 measured; 40 filtered out\n",
        "its test did not run",
    );
    fails(false, "error: no such package\n", "its test failed");
    fails(true, "", "its test did not run");
    fails(true, &format!("{passed}{passed}"), "its test did not run");
    // No real row, no sample, nothing run.
    let start = REGISTER_TEXT.find("[[row]]\nid = \"G3\"").unwrap();
    let no_real = Register::parse(&REGISTER_TEXT[..start]).unwrap();
    assert!(samples::problems(&no_real, &|_| unreachable!(), &|_| unreachable!()).is_empty());
}

/// Dates are days of the calendar, counted as Hinnant's algorithm counts them, and read as
/// prose writes them.
#[test]
fn evidence_dates_are_calendar_days() {
    assert_eq!(day_number("1970-01-01"), Some(0));
    assert_eq!(day_number("2000-03-01"), Some(11_017));
    assert_eq!(day_number("1969-12-31"), Some(-1));
    let between = |a: &str, b: &str| day_number(b).unwrap() - day_number(a).unwrap();
    assert_eq!(between("2026-09-17", "2026-10-17"), 30);
    assert_eq!(between("2024-02-28", "2024-03-01"), 2);
    assert_eq!(between("2026-12-31", "2027-01-01"), 1);
    for date in ["2024-02-29", "2000-02-29", "2026-04-30", "2026-12-31"] {
        assert!(day_number(date).is_some(), "{date}");
    }
    for date in [
        "2026-02-29",
        "2100-02-29",
        "2026-04-31",
        "2026-13-01",
        "2026-00-10",
        "2026-10-00",
        "2026-1-01",
        "26-10-17",
        "2026-10-17T00",
        "",
    ] {
        assert_eq!(day_number(date), None, "{date}");
    }
    assert_eq!(prose_date("2026-10-17").unwrap(), "October 17, 2026");
    assert_eq!(prose_date("2026-09-01").unwrap(), "September 1, 2026");
    assert!(prose_date("2026-02-30").is_err());
}

/// `gh issue list`'s output reads as open and closed states; any other state, and a list as
/// long as its limit, are refused.
#[test]
fn evidence_issue_states_read_from_gh() {
    let states = parse_issue_states(
        r#"[{"number": 645, "state": "OPEN"}, {"number": 3, "state": "CLOSED"}]"#,
    )
    .unwrap();
    assert_eq!(
        states,
        IssueStates::from([(3, IssueState::Closed), (645, IssueState::Open)])
    );
    assert!(parse_issue_states(r#"[{"number": 1, "state": "MERGED"}]"#).is_err());
    assert!(parse_issue_states("not json").is_err());
    let full: Vec<String> = (1..=ISSUE_LIMIT)
        .map(|n| format!(r#"{{"number": {n}, "state": "OPEN"}}"#))
        .collect();
    assert!(parse_issue_states(&format!("[{}]", full.join(","))).is_err());
}

/// Row ids and decision labels: `G` and a number with no leading zero; `ADR-` and 3 or 4 digits.
#[test]
fn evidence_ids_and_labels_read() {
    assert_eq!(row_number("G1"), Some(1));
    assert_eq!(row_number("G24"), Some(24));
    for id in ["G0", "G01", "g1", "G", "G1a", "H1", "G-1"] {
        assert_eq!(row_number(id), None, "{id}");
    }
    assert_eq!(adr_number("ADR-278"), Some(278));
    assert_eq!(adr_number("ADR-1000"), Some(1000));
    for label in ["ADR-27", "ADR-10000", "adr-278", "ADR278", "ADR-27a"] {
        assert_eq!(adr_number(label), None, "{label}");
    }
}

/// `--write` on a tree of its own: it writes the schema and fills the page's report, leaves the
/// rest of the page as it was, and the check then reads the tree clean.
#[test]
fn evidence_write_fills_the_report_and_the_schema() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    let put = |path: &str, text: &str| {
        let path = root.join(path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, text).unwrap();
    };
    put(REGISTER, REGISTER_TEXT);
    put(
        "docs/SUMMARY.md",
        "# Summary\n\n- [Logs](format/logs.md)\n- [Motors](motors.md)\n- [Evidence](evidence.md)\n",
    );
    put("docs/format/logs.md", LOGS);
    put("docs/motors.md", MOTORS);
    for name in decision_files().values() {
        put(&format!("docs/decisions/{name}"), "# A record\n");
    }
    let page = format!("# Evidence\n\nAbove.\n\n{REGION_START}\n{REGION_END}\n\nBelow.\n");
    put(PAGE, &page);
    write(root).unwrap();
    let gathered = Gathered::read(root).unwrap();
    assert_eq!(gathered.schema.as_deref(), Some(schema().as_str()));
    let written = gathered.page.as_deref().unwrap();
    assert!(
        written.starts_with("# Evidence\n\nAbove.\n\n<!-- evidence: report;"),
        "{written}"
    );
    assert!(
        written.ends_with("<!-- evidence: end -->\n\nBelow.\n"),
        "{written}"
    );
    let issues = IssueStates::from([(645, IssueState::Open)]);
    assert_eq!(check(&gathered.inputs(Some(&issues))), Outcome::default());
    // A second write changes nothing.
    write(root).unwrap();
    assert_eq!(std::fs::read_to_string(root.join(PAGE)).unwrap(), written);
}

/// The rows of ADR-278's table, G1 to G24, which the register must keep (ADR-278 §2).
const ADR_278_ROWS: usize = 24;

/// The rows the register holds: ADR-278's and the gaps the survey found beyond its table.
const REGISTER_ROWS: usize = 37;

// LIVE: passes once validation/evidence.toml and docs/evidence.md are written
/// The committed register, pages, report and schema pass the check, offline; the register's
/// rows are G1 to G34, ADR-278's 24 among them, pinned: removing a row takes a decision record
/// amending ADR-278 (ADR-278 §2), and this test changes with it.
#[test]
fn the_committed_register_and_pages_are_current() {
    let root = crate::designs::root().unwrap();
    let gathered = Gathered::read(&root).unwrap();
    let outcome = check(&gathered.inputs(None));
    assert_eq!(outcome.problems, Vec::<String>::new());
    let register = Register::parse(&gathered.register).unwrap();
    let ids: Vec<&str> = register.row.iter().map(|row| row.id.as_str()).collect();
    for n in 1..=ADR_278_ROWS {
        assert!(
            ids.contains(&format!("G{n}").as_str()),
            "G{n}, a row of ADR-278's table, is gone"
        );
    }
    let expected: Vec<String> = (1..=REGISTER_ROWS).map(|n| format!("G{n}")).collect();
    assert_eq!(ids, expected);
    assert_eq!(gathered.schema.as_deref(), Some(schema().as_str()));
    let page = gathered.page.as_deref().unwrap();
    let region = report::region(&register, &gathered.decisions).unwrap();
    assert_eq!(report::committed_region(page).unwrap(), region);
}
