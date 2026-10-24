//! How the pages read: the voice check and its report (M0.14a; ADR-284; the voice section of
//! `docs/writing.md`).
//!
//! A page can be correct and still read as if nobody in particular wrote it. `cargo xtask voice`
//! counts, per page and per 100 words, the patterns behind that ([`Pattern`]): bold-label list
//! items (less the four labels of a model page's *In short* box, [`read_page`]), internal labels, sentences over 25 words, hedges and stacked hedges, minimizers
//! ("simply", "just"), signposts ("In this section"), and first person on a reference page. It
//! also reports the register, which has no limit of its own beyond first person on a reference
//! page: "I", "we", "you" and contractions per 100 words,
//! the mean sentence length and the share of sentences over 25 words (the guide's sentence
//! targets). The counted patterns are held to the limits the guide sets ([`Pattern::limit`]) in
//! the report, and on a [`HELD`] page they fail the check (ADR-285): a page M0.14 has rewritten
//! stays within every limit and both sentence targets.
//!
//! It fails on every page on [`STOCK`], the stock phrases: puffery ("seamless"), inflated words
//! ("pivotal"), the vocabulary that spread with generated text ("delve"), buzz verbs
//! ("leverages"), "it's important to note", vague sources ("research has shown"), wordy phrases
//! ("in order to"), recaps ("In conclusion") and chat residue ("I hope this helps"). Each entry
//! cites the published lists that name it, and a test fails without it. A word with a technical
//! sense in this project stays off the list, or keeps only its other forms: "robust", "deploy",
//! "streamline", "boost" and "powerful" (a more powerful motor) are off it, and "leverage" (a
//! lever arm), "utilization", and the bare "garner" and "foster" (also surnames) are left out of
//! their entries
//! (`docs/research/plain-voice-docs.md`).
//!
//! With no path it reads this repository's pages ([`SCOPE`], less [`LEFT_OUT`]) and writes the
//! per-page report to [`REPORT`]; a test fails when the committed report and the pages disagree,
//! and `cargo xtask site` fails on a stock phrase in them or a held page that misses
//! ([`misses`]). Given paths, it reads each file, or each page under a directory (Markdown, HTML
//! or text), and prints the report, so fusionspace.co's pages and the design system's are read the
//! same way; only a stock phrase fails there.
//!
//! What it reads as prose is the text a reader sees: not code (blocks or inline), math, HTML
//! tags, images or link addresses. Headings and table cells count toward words, labels and stock
//! phrases; only paragraphs and list items count as sentences, and only they are read for hedges,
//! minimizers, signposts and the register.

use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use pulldown_cmark::{Event, LinkType, Parser, Tag, TagEnd};

use crate::site;

pub const USAGE: &str =
    "  voice [PATH...]          Report how the pages read (M0.14a): per page and per 100 words,
                           bold-label items, labels, long sentences, hedges, minimizers and
                           signposts against the writing guide's limits, and the register. With
                           no path it reads this repository's pages and writes
                           validation/reports/voice.md; given paths, it reads each file, or each
                           page under a directory, and prints the report (after `--`, a path may
                           start with `-`). Fails on a stock phrase, and with no path on a
                           held page over a limit or missing a sentence target (ADR-285).
  voice --long PATH...     List each sentence over 25 words on the pages at PATH, one a line
                           with its file, line and length, code and math shown as `…`.";

/// The paths read when no path is given, relative to the workspace root: the pages people read,
/// and the pull request and issue templates.
const SCOPE: [&str; 5] = [
    "README.md",
    "CHANGELOG.md",
    "CONTRIBUTING.md",
    "docs",
    ".github",
];

/// Pages under [`SCOPE`] left as they are (ADR-284 §5), each with the reason.
const LEFT_OUT: [(&str, &str); 7] = [
    ("docs/decisions/", "decision records are history"),
    (
        "docs/DECISIONS.md",
        "the decision records' index, written from their titles",
    ),
    ("docs/decisions-and-roadmap.md", "a records page"),
    (
        "docs/ROADMAP.md",
        "the open work a records page is written from",
    ),
    ("docs/roadmap-done.md", "the done-work archive"),
    (
        "docs/research/",
        "research notes keep their wording until they're next edited",
    ),
    (
        "docs/SUMMARY.md",
        "the site's table of contents: links, not prose",
    ),
];

/// Reference pages, which stay impersonal (ADR-284 §2): a model's equations, a file format, a
/// command's options, a glossary. First person on one counts against a limit of zero.
const REFERENCE: [&str; 4] = [
    "docs/physics/",
    "docs/format/",
    "docs/cli.md",
    "docs/glossary.md",
];

/// The pages held to every limit and both sentence targets (ADR-285): the ones M0.14 has
/// rewritten, each increment adding its own. One that misses ([`misses`]) fails the check.
const HELD: [&str; 61] = [
    // M0.14b, the front doors.
    "README.md",
    "docs/start-here.md",
    "docs/the-simulator.md",
    // M0.14c1, the first guides.
    "docs/404.md",
    "docs/getting-started.md",
    "docs/fly-your-ork.md",
    "docs/pick-a-motor.md",
    "docs/stability-for-certification.md",
    "docs/recording-a-trajectory.md",
    "docs/exporting-a-flight.md",
    "docs/the-builder.md",
    "docs/python.md",
    "docs/custom-models.md",
    "docs/simulator-tour.md",
    "docs/simulator-compared.md",
    "docs/trusting-a-simulation.md",
    "docs/stability-in-flight.md",
    "docs/when-fins-flutter.md",
    "docs/drag-through-mach-1.md",
    // M0.14c2, studies, data and logs.
    "docs/monte-carlo.md",
    "docs/sensitivity.md",
    "docs/optimization.md",
    "docs/online-data.md",
    "docs/weather.md",
    "docs/soundings.md",
    "docs/nomads.md",
    "docs/elevation.md",
    "docs/motor-stock.md",
    "docs/reading-a-flight-log.md",
    "docs/your-own-logger.md",
    "docs/your-own-rocket.md",
    "docs/how-a-flight-is-simulated.md",
    // M0.14c3, the command line and the evidence.
    "docs/cli.md",
    "docs/accuracy.md",
    "docs/checking-a-claim.md",
    "docs/evidence.md",
    "docs/VALIDATION.md",
    // M0.14c4, reference and the environment.
    "docs/api.md",
    "docs/releasing.md",
    "docs/glossary.md",
    "docs/writing.md",
    "docs/plan.md",
    "docs/physics/frames.md",
    "docs/physics/geodesy.md",
    "docs/physics/gravity.md",
    "docs/physics/magnetic.md",
    "docs/physics/atmosphere.md",
    "docs/physics/wind.md",
    "docs/physics/turbulence.md",
    "docs/physics/interpolation.md",
    "docs/physics/quadrature.md",
    // M0.14c5, the rocket and its flight.
    "docs/physics/design.md",
    "docs/physics/shapes.md",
    "docs/physics/mass.md",
    "docs/physics/motor.md",
    "docs/physics/flight.md",
    "docs/physics/integration.md",
    "docs/physics/moving-mass.md",
    "docs/physics/released-mass.md",
    "docs/physics/flutter.md",
    // M0.14c6, aerodynamics.
    "docs/physics/aero.md",
];

/// The writing guide's sentence targets: at most this many words a sentence on average...
const MEAN_TARGET: usize = 20;

/// ...and fewer than one sentence in this many over 25 words (under 10%).
const LONG_SHARE_TARGET: usize = 10;

/// The committed per-page report, relative to the workspace root.
pub(crate) const REPORT: &str = "validation/reports/voice.md";

/// The file extensions read under a directory given as a path.
const EXTENSIONS: [&str; 5] = ["md", "markdown", "html", "htm", "txt"];

/// Directories never walked into, besides hidden ones: build output and dependencies.
const SKIPPED_DIRS: [&str; 3] = ["node_modules", "target", "_build"];

/// What stands in for code and math in the prose: one word in a sentence's length, and never part
/// of a phrase.
const CODE: char = '\u{E000}';

/// Why a stock phrase is on the list: the rows of `docs/research/plain-voice-docs.md` §1.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    Puffery,
    Inflated,
    Vocabulary,
    BuzzVerb,
    NoteThat,
    VagueSource,
    Wordy,
    Recap,
    ChatResidue,
}

impl Kind {
    fn says(self) -> &'static str {
        match self {
            Kind::Puffery => "puffery: say what it does, with a number if there is one",
            Kind::Inflated => "an inflated word: say what happened",
            Kind::Vocabulary => "a word that spread with generated text: use the plain one",
            Kind::BuzzVerb => "a buzz verb: use, help, let or allow",
            Kind::NoteThat => "a note-that phrase: just say it",
            Kind::VagueSource => "a vague source: name the study and its number",
            Kind::Wordy => "a wordy phrase: to, because, now, can",
            Kind::Recap => "a recap: say it once, where it belongs",
            Kind::ChatResidue => "chat residue: a page isn't a conversation",
        }
    }
}

/// One entry of the stock-phrase list.
struct Stock {
    /// The forms matched, in lower case, each a whole word or a run of whole words with no
    /// punctuation between them. The first names the entry.
    forms: &'static [&'static str],
    kind: Kind,
    /// Phrases that start with one of [`Self::forms`] and are left alone: technical terms.
    allowed: &'static [&'static str],
    /// The published lists that name it, numbered as `docs/research/plain-voice-docs.md`'s
    /// sources are: 1 and 2 Wikipedia's "Words to watch" and its signs of generated writing, 3
    /// Google's and 4 Microsoft's style guides, 5 GOV.UK's, 6 to 8 the studies of excess
    /// vocabulary, 9 the design system's writing rules, 10 the Federal Plain Language
    /// Guidelines.
    sources: &'static [u8],
}

/// The stock phrases: the only patterns that fail a build. Each has a test that fails without it
/// (`every_stock_entry_catches_its_sample`), and none is on a model page
/// (`no_stock_phrase_on_the_model_pages`).
const STOCK: &[Stock] = &[
    Stock {
        forms: &["seamless", "seamlessly"],
        kind: Kind::Puffery,
        allowed: &["seamless tube", "seamless tubes", "seamless tubing"],
        sources: &[6, 9],
    },
    Stock {
        forms: &[
            "revolutionary",
            "revolutionize",
            "revolutionizes",
            "revolutionized",
        ],
        kind: Kind::Puffery,
        allowed: &[],
        sources: &[1, 6, 9],
    },
    Stock {
        forms: &["cutting-edge", "cutting edge"],
        kind: Kind::Puffery,
        allowed: &[],
        sources: &[1],
    },
    Stock {
        forms: &["world-class", "world class"],
        kind: Kind::Puffery,
        allowed: &[],
        sources: &[1],
    },
    Stock {
        forms: &["groundbreaking", "ground-breaking"],
        kind: Kind::Puffery,
        allowed: &[],
        sources: &[2, 6, 8],
    },
    Stock {
        forms: &["innovative"],
        kind: Kind::Puffery,
        allowed: &[],
        sources: &[1, 6],
    },
    Stock {
        forms: &["effortless", "effortlessly"],
        kind: Kind::Puffery,
        allowed: &[],
        sources: &[9],
    },
    Stock {
        forms: &["magic", "magical"],
        kind: Kind::Puffery,
        allowed: &["magic bytes", "magic byte", "magic number", "magic numbers"],
        sources: &[9],
    },
    Stock {
        forms: &["vibrant"],
        kind: Kind::Puffery,
        allowed: &[],
        sources: &[2],
    },
    Stock {
        forms: &["nestled"],
        kind: Kind::Puffery,
        allowed: &[],
        sources: &[2],
    },
    Stock {
        forms: &["in the heart of"],
        kind: Kind::Puffery,
        allowed: &[],
        sources: &[2],
    },
    Stock {
        forms: &["testament"],
        kind: Kind::Inflated,
        allowed: &[],
        sources: &[2],
    },
    Stock {
        forms: &["pivotal"],
        kind: Kind::Inflated,
        allowed: &[],
        sources: &[2, 6],
    },
    Stock {
        forms: &["tapestry"],
        kind: Kind::Inflated,
        allowed: &[],
        sources: &[2],
    },
    Stock {
        forms: &[
            "plays a key role",
            "plays a crucial role",
            "plays a vital role",
            "plays a pivotal role",
            "plays a significant role",
            "play a key role",
            "play a crucial role",
            "play a vital role",
            "play a pivotal role",
            "play a significant role",
        ],
        kind: Kind::Inflated,
        allowed: &[],
        sources: &[2],
    },
    Stock {
        forms: &["delve", "delves", "delved", "delving"],
        kind: Kind::Vocabulary,
        allowed: &[],
        sources: &[2, 6, 8],
    },
    Stock {
        forms: &["showcase", "showcases", "showcased", "showcasing"],
        kind: Kind::Vocabulary,
        allowed: &[],
        sources: &[2, 6, 8],
    },
    Stock {
        forms: &["garners", "garnered", "garnering"],
        kind: Kind::Vocabulary,
        allowed: &[],
        sources: &[2, 6, 8],
    },
    Stock {
        forms: &["boast", "boasts", "boasted", "boasting"],
        kind: Kind::Vocabulary,
        allowed: &[],
        sources: &[2, 8],
    },
    Stock {
        forms: &["intricate", "intricacies"],
        kind: Kind::Vocabulary,
        allowed: &[],
        sources: &[2, 6, 8],
    },
    Stock {
        forms: &["realm", "realms"],
        kind: Kind::Vocabulary,
        allowed: &[],
        sources: &[6, 8],
    },
    Stock {
        forms: &["meticulous", "meticulously"],
        kind: Kind::Vocabulary,
        allowed: &[],
        sources: &[2, 6],
    },
    Stock {
        forms: &["commendable"],
        kind: Kind::Vocabulary,
        allowed: &[],
        sources: &[6],
    },
    Stock {
        forms: &["leverages", "leveraged", "leveraging"],
        kind: Kind::BuzzVerb,
        allowed: &[],
        sources: &[2, 3, 5, 6],
    },
    Stock {
        forms: &["utilize", "utilizes", "utilized", "utilizing"],
        kind: Kind::BuzzVerb,
        allowed: &[],
        sources: &[3, 4, 5, 6],
    },
    Stock {
        forms: &["facilitate", "facilitates", "facilitated", "facilitating"],
        kind: Kind::BuzzVerb,
        allowed: &[],
        sources: &[5, 6],
    },
    Stock {
        forms: &["empower", "empowers", "empowered", "empowering"],
        kind: Kind::BuzzVerb,
        allowed: &[],
        sources: &[5, 6],
    },
    Stock {
        forms: &["fosters", "fostered", "fostering"],
        kind: Kind::BuzzVerb,
        allowed: &[],
        sources: &[2, 5, 6],
    },
    Stock {
        forms: &["it's important to note", "it is important to note"],
        kind: Kind::NoteThat,
        allowed: &[],
        sources: &[2],
    },
    Stock {
        forms: &["worth noting"],
        kind: Kind::NoteThat,
        allowed: &[],
        sources: &[2],
    },
    Stock {
        forms: &["it should be noted"],
        kind: Kind::NoteThat,
        allowed: &[],
        sources: &[1],
    },
    Stock {
        forms: &["please note"],
        kind: Kind::NoteThat,
        allowed: &[],
        sources: &[3],
    },
    Stock {
        forms: &["research has shown"],
        kind: Kind::VagueSource,
        allowed: &[],
        sources: &[1],
    },
    Stock {
        forms: &["experts argue", "experts declare"],
        kind: Kind::VagueSource,
        allowed: &[],
        sources: &[1, 2],
    },
    Stock {
        forms: &["in order to"],
        kind: Kind::Wordy,
        allowed: &[],
        sources: &[3, 4, 5, 10],
    },
    Stock {
        forms: &["a number of"],
        kind: Kind::Wordy,
        allowed: &[],
        sources: &[10],
    },
    Stock {
        forms: &["is able to", "are able to"],
        kind: Kind::Wordy,
        allowed: &[],
        sources: &[10],
    },
    Stock {
        forms: &["at this point in time"],
        kind: Kind::Wordy,
        allowed: &[],
        sources: &[10],
    },
    Stock {
        forms: &["in summary"],
        kind: Kind::Recap,
        allowed: &[],
        sources: &[2],
    },
    Stock {
        forms: &["in conclusion"],
        kind: Kind::Recap,
        allowed: &[],
        sources: &[2],
    },
    Stock {
        forms: &["i hope this helps"],
        kind: Kind::ChatResidue,
        allowed: &[],
        sources: &[2],
    },
];

/// A pattern counted per 100 words and held to a limit in the report, and in the build on a
/// [`HELD`] page (ADR-285).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Pattern {
    BoldLabels,
    Labels,
    LongSentences,
    Hedges,
    StackedHedges,
    Minimizers,
    Signposts,
    FirstPersonOnReference,
}

impl Pattern {
    const ALL: [Pattern; 8] = [
        Pattern::BoldLabels,
        Pattern::Labels,
        Pattern::LongSentences,
        Pattern::Hedges,
        Pattern::StackedHedges,
        Pattern::Minimizers,
        Pattern::Signposts,
        Pattern::FirstPersonOnReference,
    ];

    fn name(self) -> &'static str {
        match self {
            Pattern::BoldLabels => "Bold-label items",
            Pattern::Labels => "Labels",
            Pattern::LongSentences => "Sentences over 25 words",
            Pattern::Hedges => "Hedges",
            Pattern::StackedHedges => "Stacked hedges",
            Pattern::Minimizers => "Minimizers",
            Pattern::Signposts => "Signposts",
            Pattern::FirstPersonOnReference => "First person on a reference page",
        }
    }

    /// The most a page may hold per 100 words, as the writing guide's voice section sets it.
    fn limit(self) -> f64 {
        match self {
            Pattern::BoldLabels => 0.5,
            Pattern::Labels => 0.5,
            Pattern::LongSentences => 0.5,
            Pattern::Hedges => 1.0,
            Pattern::StackedHedges => 0.1,
            Pattern::Minimizers => 0.1,
            Pattern::Signposts => 0.1,
            Pattern::FirstPersonOnReference => 0.0,
        }
    }

    /// How many the page holds, or `None` where the pattern doesn't apply.
    fn count(self, page: &Page) -> Option<usize> {
        Some(match self {
            Pattern::BoldLabels => page.bold_labels,
            Pattern::Labels => page.labels,
            Pattern::LongSentences => page.long,
            Pattern::Hedges => page.hedges,
            Pattern::StackedHedges => page.stacked,
            Pattern::Minimizers => page.minimizers,
            Pattern::Signposts => page.signposts,
            Pattern::FirstPersonOnReference if page.reference => page.first_person,
            Pattern::FirstPersonOnReference => return None,
        })
    }

    /// Whether the page holds more than the limit allows: `count × 100 > limit × words`, exactly.
    fn over(self, page: &Page) -> bool {
        self.count(page)
            .is_some_and(|count| count as f64 * 100.0 > self.limit() * page.words as f64)
    }
}

/// Hedge words (the honesty rule's limits use them; stacked in one sentence, they read as
/// defensive). "may" counts only in lower case, so the month doesn't.
const HEDGES: [&str; 15] = [
    "may",
    "might",
    "could",
    "possibly",
    "perhaps",
    "probably",
    "likely",
    "seems",
    "seem",
    "seemed",
    "appears",
    "apparently",
    "presumably",
    "potentially",
    "arguably",
];

/// Minimizers: words that tell the reader a thing is easy (research note §1, sources 3 and 4).
/// "just" before a word of place or time ("just below the speed of sound") and "how quickly"
/// measure something, so they don't count.
const MINIMIZERS: [&str; 6] = ["simply", "just", "easy", "easily", "quick", "quickly"];

/// Words after "just" that make it a measure of place or time, not a minimizer.
const JUST_WHERE: [&str; 16] = [
    "below", "above", "after", "before", "under", "over", "past", "outside", "inside", "short",
    "beyond", "behind", "ahead", "aft", "forward", "off",
];

/// Signposts: sentences about the page instead of its subject. A page's opening sentence on what
/// it covers is the house style, so "this page" isn't one. A form starting `^` counts only at the
/// start of a sentence ("This section compares ...", not "the tests that pin this section"). "In
/// short" counts only as a recap, before a comma: a heading isn't read for signposts, and "In
/// short." is the label of a model page's summary box.
const SIGNPOSTS: [&str; 18] = [
    "in this section",
    "^this section",
    "in the next section",
    "^the next section",
    "the following section",
    "as mentioned",
    "as noted above",
    "as noted earlier",
    "as discussed",
    "as we saw",
    "as you can see",
    "as shown above",
    "as described above",
    "let's",
    "let us",
    "in short",
    "we will now",
    "now we will",
];

/// Words before a capital I that make it a numeral, a name or a motor's impulse class, not the
/// writer: "Type I", "Saturn I", "an I motor".
const NUMERAL_BEFORE_I: [&str; 27] = [
    "type", "types", "class", "classes", "level", "levels", "phase", "part", "parts", "appendix",
    "stage", "grade", "mark", "case", "region", "zone", "model", "series", "saturn", "titan", "an",
    "table", "tables", "figure", "figures", "volume", "chapter",
];

/// Words after a capital I that make it a motor's impulse class: "I motors", "I impulse".
const CLASS_AFTER_I: [&str; 5] = ["motor", "motors", "class", "impulse", "reload"];

/// Contractions ending in `'s` (the rest are told by their ending): pronouns and the like, not
/// possessives.
const S_CONTRACTIONS: [&str; 11] = [
    "it's", "that's", "there's", "here's", "what's", "who's", "where's", "let's", "he's", "she's",
    "how's",
];

/// Words ending in a period that don't end a sentence.
const ABBREVIATIONS: [&str; 23] = [
    "e.g.", "i.e.", "etc.", "vs.", "cf.", "al.", "approx.", "fig.", "figs.", "eq.", "eqs.", "no.",
    "nos.", "dr.", "mr.", "ms.", "st.", "jr.", "ca.", "p.", "pp.", "vol.", "ref.",
];

/// What the check found on one page.
#[derive(Clone, Debug, Default, PartialEq)]
struct Page {
    path: String,
    reference: bool,
    /// On [`HELD`], and read as one of this repository's pages, so the check holds it.
    held: bool,
    words: usize,
    sentences: usize,
    sentence_words: usize,
    long: usize,
    bold_labels: usize,
    labels: usize,
    hedges: usize,
    stacked: usize,
    minimizers: usize,
    signposts: usize,
    first_person: usize,
    we: usize,
    you: usize,
    contractions: usize,
    stock: Vec<Hit>,
}

/// A stock phrase found.
#[derive(Clone, Debug, PartialEq)]
struct Hit {
    line: usize,
    found: String,
    /// The entry's first form.
    entry: &'static str,
    kind: Kind,
    /// The lists that name it ([`Stock::sources`]).
    sources: &'static [u8],
}

/// What a run of text is, for what it's read for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Block {
    Prose,
    Heading,
    Cell,
}

/// A run of text a reader sees, with where its pieces came from in the source.
#[derive(Debug)]
struct Run {
    block: Block,
    text: String,
    /// `(offset in text, offset in source)` at each piece of text appended.
    marks: Vec<(usize, usize)>,
}

/// The runs of a page and its bold-label list items.
#[derive(Debug, Default)]
struct Reading {
    runs: Vec<Run>,
    bold_labels: usize,
}

impl Reading {
    fn open(&mut self, block: Block) {
        self.runs.push(Run {
            block,
            text: String::new(),
            marks: Vec::new(),
        });
    }

    /// Appends text read at `source` in the file, to the open run or a new paragraph.
    fn push(&mut self, text: &str, source: usize) {
        if self.runs.is_empty() {
            self.open(Block::Prose);
        }
        if let Some(run) = self.runs.last_mut() {
            run.marks.push((run.text.len(), source));
            run.text.push_str(text);
        }
    }
}

/// `cargo xtask voice`: with no path, writes [`REPORT`] for the repository's pages; with paths,
/// prints the report for those. Fails on a stock phrase, a missing path or bad usage, and with no
/// path on a held page that misses.
pub fn run(args: &[String]) -> Result<(), String> {
    let (long, args) = match args.split_first() {
        Some((first, rest)) if first == "--long" => (true, rest),
        _ => (false, args),
    };
    // After `--`, every argument is a path, even one that starts with `-`.
    let args = match args.split_first() {
        Some((first, rest)) if first == "--" => rest,
        _ if args.iter().any(|arg| arg.starts_with('-')) => {
            return Err(format!("usage:\n{USAGE}"));
        }
        _ => args,
    };
    if long {
        if args.is_empty() {
            return Err(format!("usage:\n{USAGE}"));
        }
        let mut count = 0;
        for (shown, text, html) in given_files(args)? {
            for line in long_sentences(&shown, &text, html) {
                println!("{line}");
                count += 1;
            }
        }
        println!("{count} sentences over 25 words");
        return Ok(());
    }
    let root = crate::designs::root()?;
    if args.is_empty() {
        let pages = repository_pages(&root)?;
        fs::write(root.join(REPORT), report(&pages))
            .map_err(|e| format!("writing {REPORT}: {e}"))?;
        println!("voice: wrote {REPORT}");
        println!("{}", summary(&pages));
        return failures(&pages);
    }
    let pages = given_pages(&root, args)?;
    print!("{}", report(&pages));
    println!("{}", summary(&pages));
    stock_failure(&pages)
}

/// The check `cargo xtask site` runs: a stock phrase on one of this repository's pages fails it,
/// and so does a held page that misses.
pub(crate) fn check(root: &Path) -> Result<(), String> {
    let pages = repository_pages(root)?;
    println!("{}", summary(&pages));
    failures(&pages)
}

/// Both failures on this repository's pages: the stock phrases and the held pages that miss.
fn failures(pages: &[Page]) -> Result<(), String> {
    let errors: Vec<String> = [stock_failure(pages), held_failure(pages)]
        .into_iter()
        .filter_map(Result::err)
        .collect();
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.join("\n"))
    }
}

/// Whether a page is on [`HELD`].
fn is_held(path: &str) -> bool {
    HELD.contains(&path)
}

/// What keeps a page from passing: each rate over its limit, and each sentence target missed.
/// The comparisons are exact, on the counts, where the report rounds up.
fn misses(page: &Page) -> Vec<String> {
    let mut found = Vec::new();
    for pattern in Pattern::ALL {
        if let Some(count) = pattern.count(page).filter(|_| pattern.over(page)) {
            found.push(format!(
                "{}: {count} in {} words, over the limit of {:.2} per 100 words",
                pattern.name(),
                page.words,
                pattern.limit()
            ));
        }
    }
    if page.sentence_words > MEAN_TARGET * page.sentences {
        found.push(format!(
            "{} words in {} sentences, over the target of {MEAN_TARGET} words a sentence",
            page.sentence_words, page.sentences
        ));
    }
    if page.long > 0 && page.long * LONG_SHARE_TARGET >= page.sentences {
        found.push(format!(
            "{} of {} sentences over 25 words, not under the target of 1 in {LONG_SHARE_TARGET}",
            page.long, page.sentences
        ));
    }
    found
}

/// An error naming each way a held page misses, and each held page that isn't read, or `Ok`.
fn held_failure(pages: &[Page]) -> Result<(), String> {
    let mut message = String::new();
    let mut failing = 0;
    for path in HELD {
        let Some(page) = pages.iter().find(|page| page.path == path) else {
            failing += 1;
            let _ = writeln!(
                message,
                "{path} is held, but it isn't one of the pages read"
            );
            continue;
        };
        let found = misses(page);
        if !found.is_empty() {
            failing += 1;
        }
        for miss in found {
            let _ = writeln!(message, "{path}: {miss}");
        }
    }
    if failing == 0 {
        return Ok(());
    }
    let _ = write!(
        message,
        "held pages that miss the voice section's limits or its sentence targets: {failing} of {} \
         (docs/writing.md#voice; ADR-285); rewrite the sentences, don't drop the page",
        HELD.len()
    );
    Err(message)
}

/// The line printed after a run.
fn summary(pages: &[Page]) -> String {
    let words: usize = pages.iter().map(|page| page.words).sum();
    let stock: usize = pages.iter().map(|page| page.stock.len()).sum();
    let within = pages
        .iter()
        .filter(|page| !Pattern::ALL.iter().any(|pattern| pattern.over(page)))
        .count();
    format!(
        "voice: {} pages, {words} words, {stock} stock phrases; {within} pages within every limit \
         of the writing guide's voice section, {} held; the stock-phrase list holds {} entries in \
         {} groups",
        pages.len(),
        pages.iter().filter(|page| page.held).count(),
        STOCK.len(),
        groups()
    )
}

/// How many groups ([`Kind`]) the stock-phrase list's entries fall in.
fn groups() -> usize {
    let mut kinds: Vec<Kind> = Vec::new();
    for entry in STOCK {
        if !kinds.contains(&entry.kind) {
            kinds.push(entry.kind);
        }
    }
    kinds.len()
}

/// An error naming each stock phrase found, or `Ok` when there is none.
fn stock_failure(pages: &[Page]) -> Result<(), String> {
    let mut message = String::new();
    let mut count = 0;
    for page in pages {
        for hit in &page.stock {
            count += 1;
            let sources: Vec<String> = hit.sources.iter().map(u8::to_string).collect();
            let _ = writeln!(
                message,
                "{}:{}: \"{}\" is on the stock-phrase list (`{}`, named by sources {} of \
                 docs/research/plain-voice-docs.md): {}",
                page.path,
                hit.line,
                hit.found,
                hit.entry,
                sources.join(", "),
                hit.kind.says()
            );
        }
    }
    if count == 0 {
        return Ok(());
    }
    let _ = write!(
        message,
        "{count} stock phrases; docs/writing.md's voice section says what to write instead"
    );
    Err(message)
}

/// This repository's pages: the tracked Markdown under [`SCOPE`], less [`LEFT_OUT`].
fn repository_pages(root: &Path) -> Result<Vec<Page>, String> {
    let mut pages = Vec::new();
    for path in repository_paths(root)? {
        let text = fs::read_to_string(root.join(&path)).map_err(|e| format!("{path}: {e}"))?;
        let mut page = read_page(&path, &text, is_reference(&path), is_model_page(&path));
        page.held = is_held(&path);
        pages.push(page);
    }
    Ok(pages)
}

/// The tracked files under [`SCOPE`] the check reads, relative to `root`, sorted.
fn repository_paths(root: &Path) -> Result<Vec<String>, String> {
    let mut paths: Vec<String> = tracked(root, &SCOPE)?
        .into_iter()
        .filter(|path| path.ends_with(".md"))
        .filter(|path| !LEFT_OUT.iter().any(|(left, _)| leaves(left, path)))
        .collect();
    paths.sort();
    Ok(paths)
}

/// The tracked files under `paths`, relative to `root`.
fn tracked(root: &Path, paths: &[&str]) -> Result<Vec<String>, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["ls-files", "-z", "--"])
        .args(paths)
        .output()
        .map_err(|e| format!("running `git ls-files`: {e}"))?;
    if !output.status.success() {
        return Err(format!(
            "`git ls-files` failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    let listing = String::from_utf8(output.stdout).map_err(|e| format!("git ls-files: {e}"))?;
    Ok(listing
        .split('\0')
        .filter(|path| !path.is_empty())
        .map(str::to_owned)
        .collect())
}

/// Whether `entry` (a directory ending in `/`, or a file) covers `path`.
fn leaves(entry: &str, path: &str) -> bool {
    if entry.ends_with('/') {
        path.starts_with(entry)
    } else {
        path == entry
    }
}

/// Whether a page is a reference page, where first person counts against its limit of 0.
fn is_reference(path: &str) -> bool {
    REFERENCE.iter().any(|entry| leaves(entry, path))
}

/// Whether a page, by its path from the repository's root, is a model page: a file directly under
/// `docs/physics/`, which opens with *In short* (`site.rs`). The four labels of that box aren't
/// counted as bold-label items ([`read_page`], ADR-285 §5).
fn is_model_page(path: &str) -> bool {
    path.strip_prefix(&format!("{}/{}/", site::SOURCE, site::MODELS))
        .is_some_and(|name| name.ends_with(".md") && !name.contains('/'))
}

/// The pages at `args`: each file, and each page under each directory, read whether tracked or
/// not. A path inside this repository is classed as its pages are; any other is not a reference
/// page.
fn given_pages(root: &Path, args: &[String]) -> Result<Vec<Page>, String> {
    let files = given_paths(args)?;
    let root = fs::canonicalize(root).unwrap_or_else(|_| root.to_path_buf());
    let mut pages = Vec::new();
    for file in files {
        let bytes = fs::read(&file).map_err(|e| format!("{}: {e}", file.display()))?;
        let text = String::from_utf8_lossy(&bytes);
        let shown = file.display().to_string();
        let inside = fs::canonicalize(&file).ok().and_then(|full| {
            full.strip_prefix(&root)
                .ok()
                .map(|p| p.to_string_lossy().replace('\\', "/"))
        });
        let reference = inside.as_deref().is_some_and(is_reference);
        let model = inside.as_deref().is_some_and(is_model_page);
        let html = matches!(extension(&file).as_deref(), Some("html" | "htm"));
        let mut page = if html {
            page_from(&shown, &text, read_html(&text), reference)
        } else {
            read_page(&shown, &text, reference, model)
        };
        page.path = shown;
        pages.push(page);
    }
    Ok(pages)
}

/// Each file at `args`, and each page under each directory.
fn given_paths(args: &[String]) -> Result<Vec<PathBuf>, String> {
    let mut files = Vec::new();
    for arg in args {
        let path = PathBuf::from(arg);
        if path.is_dir() {
            walk(&path, &mut files)?;
        } else if path.is_file() {
            files.push(path);
        } else {
            return Err(format!("{arg}: no such file or directory"));
        }
    }
    Ok(files)
}

/// Each page at `args` as `(path shown, text, whether it's HTML)`.
fn given_files(args: &[String]) -> Result<Vec<(String, String, bool)>, String> {
    let mut found = Vec::new();
    for file in given_paths(args)? {
        let bytes = fs::read(&file).map_err(|e| format!("{}: {e}", file.display()))?;
        let html = matches!(extension(&file).as_deref(), Some("html" | "htm"));
        found.push((
            file.display().to_string(),
            String::from_utf8_lossy(&bytes).into_owned(),
            html,
        ));
    }
    Ok(found)
}

/// Each sentence over 25 words in `source`, as `path:line: N words: sentence`, read as the
/// report reads it: prose only, with code and math shown as `…`.
fn long_sentences(path: &str, source: &str, html: bool) -> Vec<String> {
    let reading = if html {
        read_html(source)
    } else {
        read_markdown(source)
    };
    let line_starts = line_starts(source);
    let mut found = Vec::new();
    for run in reading.runs.iter().filter(|run| run.block == Block::Prose) {
        for sentence in sentences(&run.text) {
            let length = word_count(sentence);
            if length <= 25 {
                continue;
            }
            // `sentences` returns slices of `run.text`, so the pointers' difference is the
            // sentence's offset in it.
            let at = sentence.as_ptr() as usize - run.text.as_ptr() as usize;
            let shown: Vec<String> = sentence
                .split_whitespace()
                .map(|token| token.replace(CODE, "`…`"))
                .collect();
            found.push(format!(
                "{path}:{}: {length} words: {}",
                source_line(run, at, &line_starts),
                shown.join(" ")
            ));
        }
    }
    found
}

/// Where each line of `source` starts.
fn line_starts(source: &str) -> Vec<usize> {
    std::iter::once(0)
        .chain(source.match_indices('\n').map(|(i, _)| i + 1))
        .collect()
}

/// The source line, from 1, of offset `at` in `run`'s text.
fn source_line(run: &Run, at: usize, line_starts: &[usize]) -> usize {
    // The marks are in text order, so the last one at or before `at` is a binary search.
    let source_at = run
        .marks
        .partition_point(|&(offset, _)| offset <= at)
        .checked_sub(1)
        .map_or(0, |k| run.marks[k].1 + (at - run.marks[k].0));
    line_starts.partition_point(|&start| start <= source_at)
}

/// A path's extension in lower case.
fn extension(path: &Path) -> Option<String> {
    path.extension()
        .map(|ext| ext.to_string_lossy().to_ascii_lowercase())
}

/// The pages under `dir`, sorted, skipping hidden directories and [`SKIPPED_DIRS`].
fn walk(dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    let mut entries: Vec<PathBuf> = fs::read_dir(dir)
        .map_err(|e| format!("{}: {e}", dir.display()))?
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .collect();
    entries.sort();
    for path in entries {
        let name = path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();
        // A linked folder isn't followed, so a link back up can't loop.
        let linked = fs::symlink_metadata(&path).is_ok_and(|meta| meta.file_type().is_symlink());
        if path.is_dir() {
            if !linked && !name.starts_with('.') && !SKIPPED_DIRS.contains(&name.as_str()) {
                walk(&path, files)?;
            }
        } else if extension(&path).is_some_and(|ext| EXTENSIONS.contains(&ext.as_str())) {
            files.push(path);
        }
    }
    Ok(())
}

/// A Markdown (or plain text) page, read; `model` if it's a model page ([`is_model_page`]).
///
/// A model page's *In short* box is the one list the guide asks every model page to open with,
/// and a reader scans it; on a short page its four labels alone would pass the bold-label limit.
/// So on a model page whose box has the form `site.rs` holds it to ([`site::in_short`]), the
/// count leaves out four: that form is four top-level items, each opening with one of
/// [`site::IN_SHORT_ITEMS`] in bold, which [`read_markdown`] counted (ADR-285 §5). A box the
/// site refuses, or the same box on any other page, keeps its labels in the count.
fn read_page(path: &str, text: &str, reference: bool, model: bool) -> Page {
    let mut reading = read_markdown(text);
    if model && site::in_short(text).is_none() {
        reading.bold_labels = reading
            .bold_labels
            .saturating_sub(site::IN_SHORT_ITEMS.len());
    }
    page_from(path, text, reading, reference)
}

/// The text a reader sees on a Markdown page, as runs.
fn read_markdown(source: &str) -> Reading {
    let mut reading = Reading::default();
    let mut code_block = 0usize;
    let mut image = 0usize;
    let mut item_opening = false;
    let mut autolink = 0usize;
    for (event, range) in Parser::new_ext(source, crate::site::options()).into_offset_iter() {
        match event {
            Event::Start(Tag::CodeBlock(_)) => code_block += 1,
            Event::End(TagEnd::CodeBlock) => code_block = code_block.saturating_sub(1),
            Event::Start(Tag::Image { .. }) => image += 1,
            Event::End(TagEnd::Image) => image = image.saturating_sub(1),
            _ if code_block > 0 || image > 0 => {}
            // An HTML block's text shows on the page, so the HTML reader reads it, its marks moved
            // to the block's place in the file.
            Event::Start(Tag::HtmlBlock) => {
                let block = read_html(&source[range.clone()]);
                reading.bold_labels += block.bold_labels;
                for mut run in block.runs {
                    for mark in &mut run.marks {
                        mark.1 += range.start;
                    }
                    reading.runs.push(run);
                }
                reading.open(Block::Prose);
            }
            // An autolink's or email link's text is its address, which no writer can reword.
            Event::Start(Tag::Link {
                link_type: LinkType::Autolink | LinkType::Email,
                ..
            }) => autolink += 1,
            Event::End(TagEnd::Link) if autolink > 0 => autolink -= 1,
            Event::Text(text) if autolink > 0 => {
                item_opening = false;
                reading.push(&as_code(&text), range.start);
            }
            Event::Start(Tag::Heading { .. }) => reading.open(Block::Heading),
            Event::Start(Tag::TableCell) => reading.open(Block::Cell),
            Event::Start(Tag::Paragraph) => reading.open(Block::Prose),
            Event::Start(Tag::Item) => {
                reading.open(Block::Prose);
                item_opening = true;
            }
            Event::Start(Tag::Strong) => {
                if item_opening {
                    reading.bold_labels += 1;
                }
                item_opening = false;
            }
            Event::Text(text) => {
                if !text.trim().is_empty() {
                    item_opening = false;
                }
                reading.push(&addresses_as_code(&text), range.start);
            }
            Event::Code(_) | Event::InlineMath(_) | Event::DisplayMath(_) => {
                item_opening = false;
                reading.push(&format!(" {CODE} "), range.start);
            }
            Event::SoftBreak | Event::HardBreak => reading.push(" ", range.start),
            Event::End(
                TagEnd::Heading(_) | TagEnd::TableCell | TagEnd::Paragraph | TagEnd::Item,
            ) => reading.open(Block::Prose),
            _ => {}
        }
    }
    reading
}

/// `text` read as one code word, padded with spaces to its length so later marks still line up.
fn as_code(text: &str) -> String {
    let mut out = String::from(CODE);
    let pad = text.len().saturating_sub(CODE.len_utf8());
    out.extend(std::iter::repeat_n(' ', pad));
    out
}

/// `text` with each web address in it ("https://...", "www....") read as one code word, its
/// length kept: an address isn't prose, and no writer can reword it.
fn addresses_as_code(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while !rest.is_empty() {
        let token_end = rest.find(char::is_whitespace).unwrap_or(rest.len());
        let token = &rest[..token_end];
        if token.contains("://") || token.starts_with("www.") {
            out.push_str(&as_code(token));
        } else {
            out.push_str(token);
        }
        rest = &rest[token_end..];
        let space_end = rest
            .find(|c: char| !c.is_whitespace())
            .unwrap_or(rest.len());
        out.push_str(&rest[..space_end]);
        rest = &rest[space_end..];
    }
    out
}

/// Elements whose content a reader doesn't read as prose. `code` stands in as one word.
const HTML_SKIPPED: [&str; 10] = [
    "script", "style", "code", "pre", "svg", "head", "noscript", "template", "math", "kbd",
];

/// Elements that start or end a run of text.
const HTML_BLOCKS: [&str; 30] = [
    "p",
    "div",
    "li",
    "ul",
    "ol",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "td",
    "th",
    "tr",
    "table",
    "section",
    "article",
    "header",
    "footer",
    "nav",
    "main",
    "aside",
    "blockquote",
    "dt",
    "dd",
    "figcaption",
    "caption",
    "summary",
    "details",
    "button",
];

/// The text a reader sees on an HTML page, as runs.
fn read_html(source: &str) -> Reading {
    let mut reading = Reading::default();
    let lower = source.to_ascii_lowercase();
    let mut item_opening = false;
    let mut at = 0;
    let mut text_from = 0;
    while let Some(found) = source[at..].find('<') {
        let start = at + found;
        // A `<` that opens no tag, comment or declaration ("if a < b") is text.
        if !source[start + 1..]
            .starts_with(|c: char| c.is_ascii_alphabetic() || matches!(c, '/' | '!' | '?'))
        {
            at = start + 1;
            continue;
        }
        let text = &source[text_from..start];
        if !text.is_empty() {
            if !decode_entities(text).trim().is_empty() {
                item_opening = false;
            }
            push_decoded(&mut reading, text, text_from);
        }
        if lower[start..].starts_with("<!--") {
            at = lower[start..]
                .find("-->")
                .map_or(source.len(), |end| start + end + 3);
            text_from = at;
            continue;
        }
        let end = tag_end(source, start);
        // A tag the file cuts off has no `>` to drop, and its last character may be several bytes.
        let inner_end = if source[..end].ends_with('>') {
            end - 1
        } else {
            end
        };
        let tag = &lower[start + 1..inner_end.max(start + 1)];
        let closing = tag.starts_with('/');
        let name: String = tag
            .trim_start_matches('/')
            .chars()
            .take_while(char::is_ascii_alphanumeric)
            .collect();
        at = end;
        text_from = end;
        if name.is_empty() {
            continue;
        }
        if !closing && HTML_SKIPPED.contains(&name.as_str()) && !tag.ends_with('/') {
            let close = format!("</{name}");
            let stop = lower[at..].find(&close).map_or(source.len(), |i| at + i);
            if name == "code" {
                item_opening = false;
                reading.push(&format!(" {CODE} "), start);
            }
            at = if stop < source.len() {
                tag_end(source, stop)
            } else {
                stop
            };
            text_from = at;
            continue;
        }
        if HTML_BLOCKS.contains(&name.as_str()) {
            let block = match name.as_str() {
                "h1" | "h2" | "h3" | "h4" | "h5" | "h6" if !closing => Block::Heading,
                "td" | "th" if !closing => Block::Cell,
                _ => Block::Prose,
            };
            reading.open(block);
            item_opening = name == "li" && !closing;
        } else if name == "br" {
            reading.push(" ", start);
        } else if (name == "strong" || name == "b") && !closing {
            if item_opening {
                reading.bold_labels += 1;
            }
            item_opening = false;
        }
    }
    let rest = &source[text_from.min(source.len())..];
    push_decoded(&mut reading, rest, text_from);
    reading
}

/// Pushes HTML text read at `source` in the file with its entities decoded. Each entity and each
/// stretch between entities gets its own mark, so a hit's line stays exact after an entity that
/// decodes shorter than it's written.
fn push_decoded(reading: &mut Reading, text: &str, source: usize) {
    // The text is cut only before a `&` and after an entity's `;`, both one byte, so every cut
    // falls between characters.
    let mut piece = 0;
    for end in text.match_indices('&').map(|(i, _)| i).chain([text.len()]) {
        let part = &text[piece..end];
        // A piece holds at most one entity, at its start, since it ends at the next `&`.
        let written = decode_entity(part).map_or(0, |(c, written)| {
            reading.push(c.encode_utf8(&mut [0; 4]), source + piece);
            written
        });
        if written < part.len() {
            reading.push(&part[written..], source + piece + written);
        }
        piece = end;
    }
}

/// The offset just past the `>` that closes the tag starting at `start`, skipping quoted values.
fn tag_end(source: &str, start: usize) -> usize {
    let mut quote = None;
    for (i, c) in source[start..].char_indices() {
        match (quote, c) {
            (Some(q), _) if c == q => quote = None,
            (Some(_), _) => {}
            (None, '"' | '\'') => quote = Some(c),
            (None, '>') => return start + i + 1,
            _ => {}
        }
    }
    source.len()
}

/// HTML text with its character references replaced: the named ones prose uses, and numbers.
fn decode_entities(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(at) = rest.find('&') {
        out.push_str(&rest[..at]);
        rest = &rest[at..];
        match decode_entity(rest) {
            Some((c, written)) => {
                out.push(c);
                rest = &rest[written..];
            }
            None => {
                out.push('&');
                rest = &rest[1..];
            }
        }
    }
    out.push_str(rest);
    out
}

/// The character a reference at the start of `text` ("&amp;", "&#8230;") stands for, and how many
/// bytes it's written in; `None` if `text` doesn't open with one this reader knows.
fn decode_entity(text: &str) -> Option<(char, usize)> {
    let rest = text.strip_prefix('&')?;
    let end = rest
        .char_indices()
        .take(11)
        .find(|&(_, c)| c == ';')
        .map(|(i, _)| i)?;
    let name = &rest[..end];
    let decoded = match name {
        "amp" => Some('&'),
        "lt" => Some('<'),
        "gt" => Some('>'),
        "quot" => Some('"'),
        "apos" | "#39" => Some('\''),
        "nbsp" => Some(' '),
        "rsquo" => Some('\u{2019}'),
        "lsquo" => Some('\u{2018}'),
        "ldquo" => Some('\u{201C}'),
        "rdquo" => Some('\u{201D}'),
        "mdash" => Some('\u{2014}'),
        "ndash" => Some('\u{2013}'),
        "hellip" => Some('\u{2026}'),
        "middot" => Some('\u{B7}'),
        _ => name
            .strip_prefix("#x")
            .or_else(|| name.strip_prefix("#X"))
            .and_then(|hex| u32::from_str_radix(hex, 16).ok())
            .or_else(|| name.strip_prefix('#').and_then(|dec| dec.parse().ok()))
            .and_then(char::from_u32),
    };
    decoded.map(|c| (c, end + 2))
}

/// One word of a run: where it starts, as written, in lower case, and the stretch of text with no
/// punctuation it sits in (a phrase never spans two).
#[derive(Debug)]
struct Word<'a> {
    at: usize,
    raw: &'a str,
    lower: String,
    stretch: usize,
}

/// The words of `text`: letters and digits, with apostrophes and hyphens inside a word kept. A
/// curly apostrophe reads as a straight one.
fn words(text: &str) -> Vec<Word<'_>> {
    let mut found = Vec::new();
    let mut stretch = 0;
    let mut start: Option<usize> = None;
    for (i, c) in text.char_indices() {
        let inside = c.is_alphanumeric() || matches!(c, '\'' | '\u{2019}' | '-');
        match (inside, start) {
            (true, None) => start = Some(i),
            (false, Some(from)) => {
                push_word(text, from, i, stretch, &mut found);
                start = None;
            }
            _ => {}
        }
        if !inside && !c.is_whitespace() {
            stretch += 1;
        }
    }
    if let Some(from) = start {
        push_word(text, from, text.len(), stretch, &mut found);
    }
    found
}

/// Adds `text[from..to]` to `found` as a word, trimmed of hyphens and apostrophes at its ends, if
/// a letter or digit is left.
fn push_word<'a>(text: &'a str, from: usize, to: usize, stretch: usize, found: &mut Vec<Word<'a>>) {
    let piece = &text[from..to];
    let raw = piece.trim_matches(|c| matches!(c, '-' | '\'' | '\u{2019}'));
    if raw.chars().any(char::is_alphanumeric) {
        let at = from + piece.find(raw).unwrap_or(0);
        found.push(Word {
            at,
            raw,
            lower: raw.to_lowercase().replace('\u{2019}', "'"),
            stretch,
        });
    }
}

/// How many words `text` holds for its length: tokens between spaces with a letter, a digit or
/// code in them.
fn word_count(text: &str) -> usize {
    text.split_whitespace()
        .filter(|token| token.chars().any(|c| c.is_alphanumeric() || c == CODE))
        .count()
}

/// The sentences of a paragraph or list item. A sentence ends at `.`, `!` or `?` before a space
/// (closing quotes and brackets allowed between) and a word that doesn't start in lower case;
/// not after an abbreviation ([`ABBREVIATIONS`]), an initial ("A."), or a word with a period
/// before a letter inside it ("U.S."). A decimal or a version can end one ("Mach 0.9."), and so
/// can an ellipsis ("rises... Then").
fn sentences(text: &str) -> Vec<&str> {
    let mut found = Vec::new();
    let mut from = 0;
    let chars: Vec<(usize, char)> = text.char_indices().collect();
    for (k, &(i, c)) in chars.iter().enumerate() {
        if !matches!(c, '.' | '!' | '?') {
            continue;
        }
        let mut next = k + 1;
        while chars
            .get(next)
            .is_some_and(|&(_, c)| matches!(c, ')' | ']' | '"' | '\u{201D}' | '\u{2019}' | '\''))
        {
            next += 1;
        }
        let end = chars.get(next).map_or(text.len(), |&(j, _)| j);
        if chars.get(next).is_some_and(|&(_, c)| !c.is_whitespace()) {
            continue;
        }
        let following = chars[next..].iter().find(|(_, c)| !c.is_whitespace());
        if following.is_some_and(|&(_, c)| c.is_lowercase()) {
            continue;
        }
        if c == '.' {
            let word_start = text[..i]
                .char_indices()
                .rev()
                .find(|&(_, c)| c.is_whitespace())
                .map_or(0, |(s, c)| s + c.len_utf8());
            let word = text[word_start..=i].trim_start_matches(['(', '"', '\u{201C}']);
            let lower = word.to_lowercase();
            let initial = word.len() == 2 && word.starts_with(|c: char| c.is_uppercase());
            // A stop inside the word before a letter makes it an abbreviation ("e.g.", "U.S.");
            // one before a digit is a decimal or a version ("Mach 0.9.", "release 0.1."), which
            // can end a sentence.
            let dotted = word[..word.len() - 1]
                .split('.')
                .skip(1)
                .any(|rest| rest.starts_with(char::is_alphabetic));
            if ABBREVIATIONS.contains(&lower.as_str()) || initial || dotted {
                continue;
            }
        }
        found.push(&text[from..end]);
        from = end;
    }
    found.push(&text[from..]);
    found
        .into_iter()
        .map(str::trim)
        .filter(|sentence| word_count(sentence) > 0)
        .collect()
}

/// Whether `form`'s words start at `words[i]`, all in one stretch.
fn matches_at(words: &[Word<'_>], i: usize, form: &str) -> bool {
    let mut parts = form.split(' ');
    let mut k = i;
    let stretch = words[i].stretch;
    for part in parts.by_ref() {
        match words.get(k) {
            Some(word) if word.lower == part && word.stretch == stretch => k += 1,
            _ => return false,
        }
    }
    true
}

/// The stock phrases in one run's words, as `(word index, entry)`.
fn stock_in<'w>(words: &[Word<'w>]) -> Vec<(usize, &'static Stock, String)> {
    let mut hits = Vec::new();
    for i in 0..words.len() {
        for entry in STOCK {
            let Some(form) = entry.forms.iter().find(|form| matches_at(words, i, form)) else {
                continue;
            };
            if entry
                .allowed
                .iter()
                .any(|allowed| matches_at(words, i, allowed))
            {
                continue;
            }
            let length = form.split(' ').count();
            let found = words[i..i + length]
                .iter()
                .map(|word| word.raw)
                .collect::<Vec<_>>()
                .join(" ");
            hits.push((i, entry, found));
        }
    }
    hits
}

/// The page's counts, from its runs.
fn page_from(path: &str, source: &str, reading: Reading, reference: bool) -> Page {
    let mut page = Page {
        path: path.to_owned(),
        reference,
        bold_labels: reading.bold_labels,
        ..Page::default()
    };
    let line_starts = line_starts(source);
    for run in &reading.runs {
        page.words += word_count(&run.text);
        page.labels += crate::site::labels(&run.text).len();
        let all = words(&run.text);
        for (i, entry, found) in stock_in(&all) {
            page.stock.push(Hit {
                line: source_line(run, all[i].at, &line_starts),
                found,
                entry: entry.forms[0],
                kind: entry.kind,
                sources: entry.sources,
            });
        }
        if run.block != Block::Prose {
            continue;
        }
        for sentence in sentences(&run.text) {
            let length = word_count(sentence);
            page.sentences += 1;
            page.sentence_words += length;
            if length > 25 {
                page.long += 1;
            }
            read_sentence(sentence, &mut page);
        }
    }
    page
}

/// Counts one sentence's hedges, minimizers, signposts and register. Quoted words aren't the
/// writer's, so they aren't read.
fn read_sentence(sentence: &str, page: &mut Page) {
    let text = unquoted(sentence);
    let words = words(&text);
    let mut hedges = 0;
    for (i, word) in words.iter().enumerate() {
        let next = words.get(i + 1).filter(|next| next.stretch == word.stretch);
        let before = i
            .checked_sub(1)
            .and_then(|k| words.get(k))
            .filter(|before| before.stretch == word.stretch);
        let lower = word.lower.as_str();
        let hedge = if lower == "may" {
            word.raw == "may"
        } else if lower == "could" {
            next.is_none_or(|next| next.lower != "not")
        } else {
            HEDGES.contains(&lower)
        };
        if hedge {
            hedges += 1;
        }
        let minimizer = match lower {
            "just" => next.is_none_or(|next| !JUST_WHERE.contains(&next.lower.as_str())),
            "quick" | "quickly" => before.is_none_or(|before| before.lower != "how"),
            _ => MINIMIZERS.contains(&lower),
        };
        if minimizer {
            page.minimizers += 1;
        }
        if signpost(&text, &words, i) {
            page.signposts += 1;
        }
        if first_person(word, before, next) {
            page.first_person += 1;
        }
        if matches!(
            lower,
            "we" | "we're" | "we've" | "we'll" | "we'd" | "our" | "ours" | "ourselves"
        ) || word.raw == "us"
        {
            page.we += 1;
        }
        if matches!(
            lower,
            "you"
                | "your"
                | "yours"
                | "yourself"
                | "yourselves"
                | "you're"
                | "you've"
                | "you'll"
                | "you'd"
        ) {
            page.you += 1;
        }
        if contraction(lower) {
            page.contractions += 1;
        }
    }
    page.hedges += hedges;
    if hedges >= 2 {
        page.stacked += 1;
    }
}

/// `sentence` with each quotation (a pair of straight or curly double quotes and what's between)
/// replaced by a comma, so its words aren't read and no phrase spans it. An unpaired quote mark,
/// such as an inch mark, is left.
fn unquoted(sentence: &str) -> String {
    let mut out = String::with_capacity(sentence.len());
    let mut rest = sentence;
    loop {
        let open = rest
            .char_indices()
            .find(|&(_, c)| matches!(c, '"' | '\u{201C}'));
        let Some((at, mark)) = open else {
            out.push_str(rest);
            return out;
        };
        let close = if mark == '"' { '"' } else { '\u{201D}' };
        let after = at + mark.len_utf8();
        match rest[after..].find(close) {
            Some(end) => {
                out.push_str(&rest[..at]);
                out.push_str(" , ");
                rest = &rest[after + end + close.len_utf8()..];
            }
            None => {
                out.push_str(rest);
                return out;
            }
        }
    }
}

/// Whether a signpost ([`SIGNPOSTS`]) starts at `words[i]` of `text`.
fn signpost(text: &str, words: &[Word<'_>], i: usize) -> bool {
    SIGNPOSTS.iter().any(|form| {
        let (form, at_start) = match form.strip_prefix('^') {
            Some(form) => (form, true),
            None => (*form, false),
        };
        if (at_start && i > 0) || !matches_at(words, i, form) {
            return false;
        }
        if form == "in short" {
            let last = &words[i + 1];
            return text[last.at + last.raw.len()..].starts_with(',');
        }
        true
    })
}

/// Whether a word is the writer speaking of themself. A capital I counts before a lower-case word
/// other than "is" (not a symbol, as in "where I is the inertia"), and not as a numeral or a
/// motor's class ("Type I", "an I motor", "I motors").
fn first_person(word: &Word<'_>, before: Option<&Word<'_>>, next: Option<&Word<'_>>) -> bool {
    if matches!(word.lower.as_str(), "i'm" | "i've" | "i'd" | "i'll") && word.raw.starts_with('I') {
        return true;
    }
    if matches!(word.lower.as_str(), "me" | "my" | "mine" | "myself") {
        return word.raw != "ME";
    }
    word.raw == "I"
        && next.is_some_and(|next| {
            next.raw.starts_with(char::is_lowercase)
                && next.raw != "is"
                && !CLASS_AFTER_I.contains(&next.lower.as_str())
        })
        && !before.is_some_and(|before| NUMERAL_BEFORE_I.contains(&before.lower.as_str()))
}

/// Whether a lower-case word is a contraction ("don't", "it's"), for the register's count.
fn contraction(lower: &str) -> bool {
    ["n't", "'re", "'ve", "'ll", "'m", "'d"]
        .iter()
        .any(|ending| lower.ends_with(ending) && lower.len() > ending.len())
        || S_CONTRACTIONS.contains(&lower)
}

/// A rate per 100 words, rounded up to two decimals, so a page over a limit never shows a rate
/// at or under it. On a page with no words, a count above 0 is over its limit and shows as `inf`.
fn rate(count: usize, words: usize) -> f64 {
    if words == 0 {
        return if count == 0 { 0.0 } else { f64::INFINITY };
    }
    (count as f64 * 10_000.0 / words as f64).ceil() / 100.0
}

/// The report: the patterns and their limits, then a row for each page.
fn report(pages: &[Page]) -> String {
    let mut out = String::new();
    let words: usize = pages.iter().map(|page| page.words).sum();
    let stock: usize = pages.iter().map(|page| page.stock.len()).sum();
    let within = pages
        .iter()
        .filter(|page| !Pattern::ALL.iter().any(|pattern| pattern.over(page)))
        .count();
    let _ = writeln!(
        out,
        "# How the pages read\n\n\
         `cargo xtask voice` writes this report from the pages. A test fails when the two \
         disagree, so run it after editing a page. [The writing guide's voice \
         section](../../docs/writing.md#voice) says what the pattern columns count and why, and \
         sets the limits.\n\n\
         Rates are per 100 words, rounded up to two decimals, so a page over a limit never shows \
         a rate at or under it; a rate over its limit is in bold. Words a sentence and the share \
         of sentences over 25 words are rounded up too, to 0.1 and to 1%, so neither shows a page \
         inside a target it misses. A stock phrase fails the build, and so does a page marked \
         `held` that's over a limit or misses a sentence target: the {} pages already rewritten \
         ([ADR-285](../../docs/decisions/0285-the-rest-of-the-site-in-nine.md)).\n\n\
         {} pages, {words} words, {stock} stock phrases. {within} pages are within every limit.\n",
        HELD.len(),
        pages.len()
    );
    let _ = writeln!(
        out,
        "| Pattern | Limit per 100 words | Pages over it | Highest rate |\n|---|---:|---:|---|"
    );
    for pattern in Pattern::ALL {
        let over = pages.iter().filter(|page| pattern.over(page)).count();
        let highest = pages
            .iter()
            .filter_map(|page| Some((rate(pattern.count(page)?, page.words), page)))
            .filter(|(rate, _)| *rate > 0.0)
            .max_by(|a, b| a.0.total_cmp(&b.0).then_with(|| b.1.path.cmp(&a.1.path)));
        let highest = highest.map_or_else(
            || "none".to_owned(),
            |(rate, page)| format!("{rate:.2}, `{}`", page.path),
        );
        let _ = writeln!(
            out,
            "| {} | {:.2} | {over} | {highest} |",
            pattern.name(),
            pattern.limit()
        );
    }
    let _ = writeln!(
        out,
        "\nThe register has no limit of its own (first person on a reference page, above, is the one \
         limit on it): \"I\", \"we\", \"you\" and contractions per 100 words, \
         and the guide's sentence targets (at most 20 words a sentence on average, fewer than \
         10% of sentences over 25 words). A reference page is marked `ref`.\n"
    );
    let mut header = String::from("| Page | Words | Words a sentence | Over 25 words |");
    let mut rule = String::from("|---|---:|---:|---:|");
    for pattern in Pattern::ALL {
        let _ = write!(header, " {} |", pattern.name());
        rule.push_str("---:|");
    }
    header.push_str(" I | We | You | Contractions |");
    rule.push_str("---:|---:|---:|---:|");
    let _ = writeln!(out, "{header}\n{rule}");
    for page in pages {
        // Rounded up in whole numbers, as the rates are: tenths of a word, whole percent.
        let mean_tenths = (page.sentence_words * 10).div_ceil(page.sentences.max(1));
        let share = (page.long * 100).div_ceil(page.sentences.max(1));
        let mean = format!("{}.{}", mean_tenths / 10, mean_tenths % 10);
        let marks: Vec<&str> = [(page.reference, "`ref`"), (page.held, "`held`")]
            .into_iter()
            .filter_map(|(on, mark)| on.then_some(mark))
            .collect();
        let kind = if marks.is_empty() {
            String::new()
        } else {
            format!(" ({})", marks.join(", "))
        };
        let _ = write!(
            out,
            "| `{}`{kind} | {} | {mean} | {share}% |",
            page.path, page.words
        );
        for pattern in Pattern::ALL {
            match pattern.count(page) {
                Some(count) if pattern.over(page) => {
                    let _ = write!(out, " **{:.2}** |", rate(count, page.words));
                }
                Some(count) => {
                    let _ = write!(out, " {:.2} |", rate(count, page.words));
                }
                None => out.push_str(" |"),
            }
        }
        let _ = writeln!(
            out,
            " {:.2} | {:.2} | {:.2} | {:.2} |",
            rate(page.first_person, page.words),
            rate(page.we, page.words),
            rate(page.you, page.words),
            rate(page.contractions, page.words)
        );
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn markdown(text: &str) -> Page {
        read_page("page.md", text, false, false)
    }

    /// `--long` lists the sentences the report counts as long, each at its own source line, and
    /// shows code as `…`.
    #[test]
    fn long_sentences_are_the_ones_the_report_counts() {
        let long = "This sentence runs on and on with one word after another, more words than the \
                    writing guide's target allows, past twenty-five of them `here` in all.";
        let text = format!(
            "# A page\n\nA short one. {long}\n\n- A list item.\n  {long}\n\n| {long} |\n|---|\n"
        );
        let found = long_sentences("page.md", &text, false);
        assert_eq!(found.len(), markdown(&text).long);
        assert_eq!(found.len(), 2, "{found:?}");
        assert!(
            found[0].starts_with("page.md:3: 26 words: This sentence"),
            "{found:?}"
        );
        assert!(found[0].ends_with("of them `…` in all."), "{found:?}");
        assert!(found[1].starts_with("page.md:6: 26 words: "), "{found:?}");
        let html = format!("<p>A short one.</p>\n<p>{long}</p>");
        let found = long_sentences("page.html", &html, true);
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found[0].starts_with("page.html:2: "), "{found:?}");
    }

    fn entries(text: &str) -> Vec<&'static str> {
        markdown(text).stock.iter().map(|hit| hit.entry).collect()
    }

    /// One sentence for each stock entry, by its first form. Deleting an entry from [`STOCK`]
    /// fails its sentence here, and an entry added without a sentence fails too.
    const SAMPLES: &[(&str, &str)] = &[
        ("seamless", "It moves seamlessly between the two tools."),
        (
            "revolutionary",
            "This revolutionizes how flights are planned.",
        ),
        ("cutting-edge", "It uses cutting-edge aerodynamics."),
        ("world-class", "A world-class tool for rocketry."),
        ("groundbreaking", "A groundbreaking drag model."),
        ("innovative", "An innovative way to plan a flight."),
        ("effortless", "Planning a flight is effortless."),
        ("magic", "The fit works like magic."),
        ("vibrant", "A vibrant community of flyers."),
        ("nestled", "The pad is nestled in the desert."),
        ("in the heart of", "In the heart of the launch season."),
        ("testament", "The result is a testament to the model."),
        ("pivotal", "Drag is pivotal here."),
        ("tapestry", "A rich tapestry of flights."),
        (
            "plays a key role",
            "The fin flutter plays a key role in the design.",
        ),
        ("delve", "We delve into the drag model."),
        ("showcase", "This page showcases the tools."),
        ("garners", "The release garnered attention."),
        ("boast", "The catalog boasts 32 motors."),
        ("intricate", "The intricacies of staging."),
        ("realm", "In the realm of rocketry."),
        ("meticulous", "Each flight is meticulously checked."),
        ("commendable", "A commendable result."),
        ("leverages", "It leverages the motor catalog."),
        ("utilize", "Utilize the catalog to choose a motor."),
        ("facilitate", "This facilitates checking a design."),
        ("empower", "It empowers flyers to check a design."),
        ("fosters", "The project fosters careful flying."),
        (
            "it's important to note",
            "It\u{2019}s important to note the wind.",
        ),
        ("worth noting", "It's worth noting that the wind changes."),
        (
            "it should be noted",
            "It should be noted that the wind changes.",
        ),
        ("please note", "Please note the limits."),
        ("research has shown", "Research has shown that drag rises."),
        ("experts argue", "Experts argue over the margin."),
        ("in order to", "Run it in order to check the margin."),
        ("a number of", "A number of flights were read."),
        ("is able to", "The reader is able to check it."),
        (
            "at this point in time",
            "At this point in time the model is checked.",
        ),
        ("in summary", "In summary, the drag is right."),
        ("in conclusion", "In conclusion, it works."),
        ("i hope this helps", "I hope this helps you fly."),
    ];

    #[test]
    fn every_stock_entry_catches_its_sample() {
        for entry in STOCK {
            assert!(
                SAMPLES.iter().any(|(name, _)| *name == entry.forms[0]),
                "stock entry `{}` has no sample sentence",
                entry.forms[0]
            );
        }
        for (name, sentence) in SAMPLES {
            assert!(
                entries(sentence).contains(name),
                "`{sentence}` should hit the stock entry `{name}`, and hits {:?}",
                entries(sentence)
            );
        }
        assert!(STOCK.len() >= 30, "the list holds {} entries", STOCK.len());
    }

    #[test]
    fn every_stock_entry_cites_a_source_and_its_allowances_start_with_a_form() {
        for entry in STOCK {
            assert!(
                !entry.sources.is_empty(),
                "{} cites nothing",
                entry.forms[0]
            );
            assert!(entry.sources.iter().all(|s| (1..=10).contains(s)));
            for allowed in entry.allowed {
                assert!(
                    entry.forms.iter().any(|form| allowed.starts_with(form)),
                    "{allowed} starts with no form of {}",
                    entry.forms[0]
                );
            }
        }
    }

    #[test]
    fn technical_terms_pass_and_phrases_stop_at_punctuation() {
        assert!(entries("Read the magic bytes and the magic number first.").is_empty());
        assert!(entries("A seamless tube makes a stronger body.").is_empty());
        assert_eq!(entries("It is magic."), ["magic"]);
        // The stock list reads quoted words, which the counted patterns skip.
        assert_eq!(entries("Don't call it \"seamless\" here."), ["seamless"]);
        assert!(entries("Fly them in order. To check one, run it.").is_empty());
        assert_eq!(
            entries("A number, of course, is one number. A number of flights."),
            ["a number of"]
        );
        assert!(
            entries("Robust fins deploy the main at 500 ft; the streamlined boost.").is_empty()
        );
    }

    #[test]
    fn code_math_and_link_addresses_are_not_prose() {
        let text = "Run `delve` with $x$.\n\n```\nleverage\n```\n\n\
                    See [the page](https://example.com/seamless).\n\n![magic](img.png)\n";
        assert!(entries(text).is_empty());
        assert_eq!(markdown("Run `hpr sim` now.").words, 3);
    }

    #[test]
    fn stock_hits_carry_their_line() {
        let page = markdown("# Title\n\nFirst line.\nIt leverages\nthe catalog.\n");
        assert_eq!(page.stock.len(), 1);
        assert_eq!(page.stock[0].line, 4);
        assert_eq!(page.stock[0].found, "leverages");
        let html = page_from(
            "p.html",
            "<p>One.</p>\n<p>It is\npivotal.</p>",
            read_html("<p>One.</p>\n<p>It is\npivotal.</p>"),
            false,
        );
        assert_eq!(html.stock[0].line, 3);
        // One paragraph of many marks, each shifting the text against the source, with a hit on
        // each of its lines after the first.
        let lines: Vec<String> = (0..40)
            .map(|k| format!("**a** {}", if k == 0 { "c" } else { "delve" }))
            .collect();
        let many = markdown(&lines.join("\n"));
        let found: Vec<usize> = many.stock.iter().map(|hit| hit.line).collect();
        assert_eq!(found, (2..=40).collect::<Vec<_>>());
    }

    #[test]
    fn bold_label_items_are_counted() {
        let page = markdown(
            "- **Plain.** Short words.\n- A plain item.\n- An item with **bold** inside.\n\
             1. **Status:** done.\n\nA **bold** word in a paragraph.\n",
        );
        assert_eq!(page.bold_labels, 2);
        let loose = markdown("- **First.** One.\n\n- **Second.** Two.\n");
        assert_eq!(loose.bold_labels, 2);
        let html = read_html("<ul><li><strong>A:</strong> b</li><li>c <b>d</b></li></ul>");
        assert_eq!(html.bold_labels, 1);
    }

    /// A model page's opening, `box_` its *In short* list, then a section with `after` in it.
    fn model_with(box_: &str, after: &str) -> String {
        format!(
            "# A model\n\nWhat it covers.\n\n## In short\n\n{box_}\n## Method\n\nWords.\n\n{after}"
        )
    }

    /// The *In short* list as `site.rs` holds it.
    const BOX: &str = "- **What it models:** a thing.\n- **Sources:** a paper.\n\
                       - **How well it is validated:** tests.\n- **What it leaves out:** the rest.\n";

    /// `text` read as a model page.
    fn model_page(text: &str) -> Page {
        read_page("docs/physics/a-model.md", text, true, true)
    }

    #[test]
    fn a_model_pages_in_short_labels_are_left_out_only_in_the_form_the_site_holds() {
        // The box as the site holds it: its four labels are left out on a model page, and only
        // those four; on any other page they count.
        let held = model_with(BOX, "- **Option.** One.\n- **Other.** Two.\n");
        assert_eq!(site::in_short(&held), None);
        assert_eq!(model_page(&held).bold_labels, 2);
        assert_eq!(markdown(&held).bold_labels, 6);
        // A label wrapped over two lines, or written in code, is one the site reads as its own.
        for box_ in [
            BOX.replace("is validated:**", "is\n  validated:**"),
            BOX.replace("**Sources:**", "**`Sources:`**"),
        ] {
            let text = model_with(&box_, "");
            assert_eq!(site::in_short(&text), None, "{box_:?}");
            assert_eq!(model_page(&text).bold_labels, 0, "{box_:?}");
        }
        // A box the site refuses keeps every label in the count: a fifth item, a renamed or
        // nested label, the labels out of order, `### In short`, the box after another section,
        // in a quote or in a numbered list.
        let refused = [
            (format!("{BOX}- **Also:** more.\n"), 5),
            (BOX.replace("**Sources:**", "**Source:**"), 4),
            (
                BOX.replace(
                    "- **Sources:** a paper.",
                    "- **Other:** x.\n  - **Sources:** a paper.",
                ),
                5,
            ),
            (BOX.replace("**Sources:**", "**Sources: `a`**"), 4),
            (
                BOX.replace("**What it models:** a thing.", "**Sources:** a thing.")
                    .replacen("**Sources:** a paper.", "**What it models:** a paper.", 1),
                4,
            ),
            (BOX.replace("- ", "1. "), 4),
            (BOX.lines().map(|line| format!("> {line}\n")).collect(), 4),
        ];
        for (box_, labels) in &refused {
            let text = model_with(box_, "");
            assert!(site::in_short(&text).is_some(), "{box_:?}");
            assert_eq!(model_page(&text).bold_labels, *labels, "{box_:?}");
        }
        let deeper = model_with(BOX, "").replace("## In short", "### In short");
        assert!(site::in_short(&deeper).is_some());
        assert_eq!(model_page(&deeper).bold_labels, 4);
        let later =
            model_with(BOX, "").replace("## In short", "## Before\n\nWords.\n\n## In short");
        assert!(site::in_short(&later).is_some());
        assert_eq!(model_page(&later).bold_labels, 4);
    }

    #[test]
    fn every_model_page_leaves_out_exactly_its_boxs_four_labels() {
        let root = crate::designs::root().unwrap();
        let physics = tracked(&root, &["docs/physics"]).unwrap();
        let models: Vec<&String> = physics.iter().filter(|path| is_model_page(path)).collect();
        assert!(models.len() >= 20, "{} model pages", models.len());
        assert!(!is_model_page("docs/physics/sub/page.md"));
        assert!(!is_model_page("docs/physics.md"));
        assert!(!is_model_page("docs/glossary.md"));
        for path in models {
            let text = fs::read_to_string(root.join(path)).unwrap();
            // The site holds the box, so its four items each open with a bold label, which the
            // count leaves out; a bold label nested in an answer still counts.
            assert_eq!(site::in_short(&text), None, "{path}");
            let span = site::in_short_span(&text).unwrap();
            assert!(read_markdown(&text[span]).bold_labels >= 4, "{path}");
            let counted = read_page(path, &text, true, false).bold_labels;
            let model = read_page(path, &text, true, true).bold_labels;
            assert_eq!(model + 4, counted, "{path}");
        }
        // A model page given by its path, as `cargo xtask voice docs/physics/...` reads it, is
        // read as one.
        let path = "docs/physics/quadrature.md";
        let given = given_pages(&root, &[root.join(path).display().to_string()]).unwrap();
        let text = fs::read_to_string(root.join(path)).unwrap();
        assert_eq!(
            given[0].bold_labels,
            read_page(path, &text, true, true).bold_labels
        );
    }

    #[test]
    fn a_decimal_or_a_version_can_end_a_sentence() {
        let text = "Drag peaks near Mach 1.05. It falls after. The rise is 3.2%. Release 0.1.0. \
                    It reads ork.md. Then it flies. The drag rises... It peaks.";
        assert_eq!(
            sentences(text),
            [
                "Drag peaks near Mach 1.05.",
                "It falls after.",
                "The rise is 3.2%.",
                "Release 0.1.0.",
                "It reads ork.md. Then it flies.",
                "The drag rises...",
                "It peaks."
            ]
        );
    }

    #[test]
    fn sentences_end_at_stops_not_abbreviations() {
        let text = "Use a margin, e.g. two calibers. Compare it vs. OpenRocket in Fig. 3. \
                    A. C. Carruthers measured 1.5 m. It works (mostly). Done! Is it? yes it is. \
                    The report says so (p. 49) and shows it (pp. 3-4).";
        let found = sentences(text);
        assert_eq!(
            found,
            [
                "Use a margin, e.g. two calibers.",
                "Compare it vs. OpenRocket in Fig. 3.",
                "A. C. Carruthers measured 1.5 m.",
                "It works (mostly).",
                "Done!",
                "Is it? yes it is.",
                "The report says so (p. 49) and shows it (pp. 3-4)."
            ]
        );
    }

    #[test]
    fn long_sentences_and_their_mean() {
        let long = vec!["word"; 26].join(" ") + ".";
        let page = markdown(&format!("{long} Short one here.\n"));
        assert_eq!(page.sentences, 2);
        assert_eq!(page.long, 1);
        assert_eq!(page.sentence_words, 29);
        let headings = markdown(&format!("# {long}\n\n| {long} |\n|---|\n| a |\n"));
        assert_eq!(headings.sentences, 0);
    }

    #[test]
    fn hedges_stack_only_within_a_sentence() {
        let page =
            markdown("It may possibly fail. It might fail. It could not fail. In May it flew.\n");
        assert_eq!(page.hedges, 3);
        assert_eq!(page.stacked, 1);
    }

    #[test]
    fn first_person_skips_numerals_and_symbols() {
        let count = |text: &str| markdown(text).first_person;
        assert_eq!(
            count("I built it, and I\u{2019}m keeping it. My notes say so."),
            3
        );
        assert_eq!(count("A Type I motor, where I is the inertia."), 0);
        assert_eq!(count("Saturn I flew. The ME column."), 0);
        assert_eq!(
            count("It works out 525.8 N·s, an I motor; I motors burn longer."),
            0
        );
        assert_eq!(
            count("It answers \"what did my rocket do?\" for anyone. I checked."),
            1
        );
        assert_eq!(count("A 12\" tube, and I measured it. I was wrong."), 2);
        assert_eq!(count("Its Tables I and II, and types I and III."), 0);
    }

    #[test]
    fn register_counts() {
        let page = markdown(
            "We think you'll like it. It's ours, and the rocket's fins aren't ours; tell us.\n",
        );
        assert_eq!(page.we, 4);
        assert_eq!(page.you, 1);
        assert_eq!(page.contractions, 3);
    }

    #[test]
    fn signposts_skip_headings_and_page_openings() {
        let page = markdown(
            "## In short\n\nThis page covers drag. In short, it rises. \
             As mentioned, it falls. Let's fly.\n\n**In short.** The box. Its In short says so.\n\n\
             This section compares two. The tests that pin this section pass.\n",
        );
        assert_eq!(page.signposts, 4);
        let minimizers = markdown(
            "Simply run it; it's just easy to do quickly. It flies just below Mach 1, \
             and how quickly it settles depends on its mass.\n",
        );
        assert_eq!(minimizers.minimizers, 4);
    }

    #[test]
    fn labels_count_linked_and_bare() {
        let page = markdown("See [M7.2c](x.md) and ADR-284, not M7 or L.\n");
        assert_eq!(page.labels, 2);
    }

    #[test]
    fn html_reads_text_not_code_scripts_or_styles() {
        let source = "<html><head><title>delve</title><style>p{}</style></head><body>\
                      <script>leverage()</script><p>Plain &amp; short &hellip; <code>utilize</code> \
                      it&rsquo;s fine.</p><!-- seamless --><pre>pivotal</pre></body></html>";
        let page = page_from("p.html", source, read_html(source), false);
        assert!(page.stock.is_empty(), "{:?}", page.stock);
        assert_eq!(page.contractions, 1);
        assert_eq!(page.words, 5);
        assert_eq!(
            decode_entities("a &#8230; b &#x2019; &bogus; &"),
            "a \u{2026} b \u{2019} &bogus; &"
        );
    }

    #[test]
    fn wide_characters_and_stray_marks_are_read_whole() {
        // A non-breaking or thin space before a sentence's last word, as the guide joins a
        // number and its unit.
        for space in ["\u{A0}", "\u{2009}", "\u{202F}"] {
            let text = format!("It flew 1.5{space}m. Then it fell.");
            assert_eq!(
                sentences(&text),
                [format!("It flew 1.5{space}m."), "Then it fell.".to_string()]
            );
        }
        let source = "<p>It flew 1.5&nbsp;m. Then it fell.</p>";
        assert_eq!(
            page_from("p.html", source, read_html(source), false).sentences,
            2
        );
        // An entity cut off by a wide character inside its first 12 bytes.
        assert_eq!(
            decode_entities("x &aaaaaaaaaa\u{E9}; y"),
            "x &aaaaaaaaaa\u{E9}; y"
        );
        // A tag the file cuts off, ending in a wide character.
        let cut = "<p>x</p><a \u{E9}";
        assert_eq!(page_from("p.html", cut, read_html(cut), false).words, 1);
        // A `<` that opens no tag is text, and what follows it is read.
        let stray = "<p>If a < b, it is pivotal.</p>";
        let page = page_from("p.html", stray, read_html(stray), false);
        assert_eq!(page.words, 6);
        assert_eq!(page.stock.len(), 1);
    }

    #[test]
    fn over_a_limit_is_exact_and_shown_rounded_up() {
        let page = Page {
            words: 201,
            long: 1,
            reference: true,
            first_person: 0,
            ..Page::default()
        };
        assert!(!Pattern::LongSentences.over(&page));
        assert_eq!(rate(1, 201), 0.5);
        let over = Page {
            words: 199,
            ..page.clone()
        };
        assert!(Pattern::LongSentences.over(&over));
        assert_eq!(rate(1, 199), 0.51);
        assert_eq!(rate(1, 30_000), 0.01);
        assert!(!Pattern::FirstPersonOnReference.over(&page));
        let speaking = Page {
            first_person: 1,
            ..page.clone()
        };
        assert!(Pattern::FirstPersonOnReference.over(&speaking));
        let elsewhere = Page {
            reference: false,
            ..speaking
        };
        assert_eq!(Pattern::FirstPersonOnReference.count(&elsewhere), None);
        // A bold label with no words is over the limit, and shows a rate over it.
        let empty = markdown("- **:**\n");
        assert_eq!((empty.words, empty.bold_labels), (0, 1));
        assert!(Pattern::BoldLabels.over(&empty));
        assert_eq!(rate(1, 0), f64::INFINITY);
        assert_eq!(rate(0, 0), 0.0);
        assert!(report(&[empty]).contains("| **inf** |"));
    }

    #[test]
    fn sentence_mean_and_share_are_shown_rounded_up() {
        let row = |sentence_words, sentences, long| {
            let page = Page {
                path: "p.md".to_string(),
                words: 100,
                sentences,
                sentence_words,
                long,
                ..Page::default()
            };
            let text = report(&[page]);
            let line = text
                .lines()
                .find(|l| l.starts_with("| `p.md`"))
                .map(str::to_string);
            line.unwrap_or_default()
        };
        // 20.01 words a sentence shows 20.1, past the target, and 1 long in 101 shows 1%.
        assert!(row(2021, 101, 1).starts_with("| `p.md` | 100 | 20.1 | 1% |"));
        // Exact values stay as they are.
        assert!(row(60, 3, 0).starts_with("| `p.md` | 100 | 20.0 | 0% |"));
        assert!(row(0, 0, 0).starts_with("| `p.md` | 100 | 0.0 | 0% |"));
        // 9.1% of sentences over 25 words shows 10%, not inside "fewer than 10%".
        assert!(row(220, 11, 1).starts_with("| `p.md` | 100 | 20.0 | 10% |"));
    }

    #[test]
    fn html_blocks_are_read_and_addresses_are_not() {
        let block = markdown("Before.\n\n<div>\nIt is pivotal.\n</div>\n\n<!-- delve -->\n");
        assert_eq!(block.stock.len(), 1, "{:?}", block.stock);
        assert_eq!(block.stock[0].line, 4);
        assert_eq!(block.words, 4);
        for text in [
            "See <https://example.com/innovative/thing>.",
            "Mail <someone@seamless.example>.",
            "See https://example.com/innovative/thing for it.",
            "See www.innovative.example for it.",
        ] {
            assert!(entries(text).is_empty(), "{text}: {:?}", entries(text));
        }
        assert_eq!(
            entries("An innovative tool, at https://example.com/x."),
            ["innovative"]
        );
        // After entities that decode shorter than they're written, a hit keeps its line.
        let source = format!("<p>a\n{}\nb\nc\nIt is pivotal.</p>", "&rsquo;".repeat(30));
        let page = page_from("p.html", &source, read_html(&source), false);
        assert_eq!(page.stock[0].line, 5);
        // So does a hit on a line after an entity, in an HTML page, an HTML block or the text
        // after the last tag.
        for (path, source, line) in [
            ("a.html", "<p>&amp;\nIt is pivotal.</p>", 2),
            ("b.html", "<p>x &rsquo; &bogus;\ny\nIt is pivotal.</p>", 3),
            ("c.html", "<p>x</p>&amp;\nIt is pivotal.", 2),
            ("e.html", "<p>x\nIt is &#112;ivotal.</p>", 2),
            (
                "f.html",
                "<p>\u{E9}t\u{E9}\n\u{201C}x\u{201D} &amp;\u{E9}\nIt is pivotal.</p>",
                3,
            ),
            (
                "g.html",
                "<p>x</p>\u{2014}&amp;\u{E9}&bogus;\u{E9}\nIt is pivotal.",
                2,
            ),
            (
                "d.md",
                "Intro.\n\n<div>\n&amp;\nIt is pivotal.\n</div>\n",
                5,
            ),
        ] {
            let page = if path.ends_with(".md") {
                markdown(source)
            } else {
                page_from(path, source, read_html(source), false)
            };
            assert_eq!(page.stock.len(), 1, "{path}");
            assert_eq!(page.stock[0].line, line, "{path}");
        }
        assert_eq!(decode_entity("&amp;x"), Some(('&', 5)));
        assert_eq!(decode_entity("&#8230;"), Some(('\u{2026}', 7)));
        assert_eq!(decode_entity("&bogus;"), None);
        assert_eq!(decode_entity("&aaaaaaaaaa;"), None);
        assert_eq!(decode_entity("amp;"), None);
    }

    #[test]
    fn given_paths_read_files_and_folders_but_not_linked_folders() {
        let dir = std::env::temp_dir().join(format!("voice-given-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("sub")).unwrap();
        fs::write(dir.join("a.md"), "We delve.\n").unwrap();
        fs::write(dir.join("b.html"), "<p>It is pivotal.</p>").unwrap();
        fs::write(dir.join("c.txt"), "Plain words.\n").unwrap();
        fs::write(dir.join("sub/d.md"), "Short.\n").unwrap();
        fs::write(dir.join("e.rs"), "// delve\n").unwrap();
        #[cfg(unix)]
        std::os::unix::fs::symlink(&dir, dir.join("sub/up")).unwrap();
        let root = crate::designs::root().unwrap();
        let pages = given_pages(&root, &[dir.to_string_lossy().into_owned()]).unwrap();
        assert_eq!(pages.len(), 4);
        assert_eq!(pages.iter().map(|page| page.stock.len()).sum::<usize>(), 2);
        let one = given_pages(&root, &[dir.join("b.html").to_string_lossy().into_owned()]);
        assert_eq!(one.unwrap()[0].stock.len(), 1);
        let missing = given_pages(&root, &[dir.join("none.md").to_string_lossy().into_owned()]);
        assert!(missing.unwrap_err().contains("no such file or directory"));
        fs::remove_dir_all(&dir).unwrap();
        assert!(
            run(&["-x.md".to_string()])
                .unwrap_err()
                .starts_with("usage")
        );
        let after = run(&["--".to_string(), "/no/such/-x.md".to_string()]);
        assert!(after.unwrap_err().contains("no such file or directory"));
    }

    #[test]
    fn the_guide_states_the_limits_and_the_list_as_the_code_has_them() {
        let root = crate::designs::root().unwrap();
        let guide = fs::read_to_string(root.join("docs/writing.md")).unwrap();
        for pattern in Pattern::ALL {
            let limit = match pattern.limit() {
                0.0 => "0".to_string(),
                limit => format!("{limit:.1}"),
            };
            assert!(
                guide
                    .lines()
                    .any(|line| line.starts_with(&format!("| {} |", pattern.name()))
                        && line.ends_with(&format!("| {limit} |"))),
                "the guide's limits table has no row `| {} | ... | {limit} |`",
                pattern.name()
            );
        }
        let list = format!("It holds {} entries in {} groups", STOCK.len(), groups());
        assert!(guide.contains(&list), "the guide doesn't say \"{list}\"");
    }

    #[test]
    fn the_left_out_and_reference_paths_exist() {
        let root = crate::designs::root().unwrap();
        let all = tracked(&root, &SCOPE).unwrap();
        for (entry, _) in LEFT_OUT {
            assert!(
                all.iter().any(|path| leaves(entry, path)),
                "{entry} matches nothing"
            );
        }
        for entry in REFERENCE {
            assert!(
                all.iter().any(|path| leaves(entry, path)),
                "{entry} matches nothing"
            );
        }
    }

    #[test]
    fn no_stock_phrase_on_the_model_pages() {
        let root = crate::designs::root().unwrap();
        let physics = tracked(&root, &["docs/physics"]).unwrap();
        assert!(physics.len() >= 20, "{} model pages", physics.len());
        for path in physics.iter().filter(|path| path.ends_with(".md")) {
            let text = fs::read_to_string(root.join(path)).unwrap();
            let page = read_page(path, &text, true, is_model_page(path));
            assert!(page.stock.is_empty(), "{path}: {:?}", page.stock);
        }
    }

    /// A page of 1,000 words in 50 sentences of 20 words, none long: within everything.
    fn counted(path: &str) -> Page {
        Page {
            path: path.to_owned(),
            words: 1000,
            sentences: 50,
            sentence_words: 1000,
            ..Page::default()
        }
    }

    #[test]
    fn a_page_misses_each_limit_and_target_only_past_its_edge() {
        let at_edges = Page {
            bold_labels: 5,
            hedges: 10,
            long: 4,
            ..counted("page.md")
        };
        assert!(misses(&at_edges).is_empty(), "{:?}", misses(&at_edges));
        let over = Page {
            bold_labels: 6,
            ..counted("page.md")
        };
        assert_eq!(
            misses(&over),
            ["Bold-label items: 6 in 1000 words, over the limit of 0.50 per 100 words"]
        );
        let wordy = Page {
            sentence_words: 1001,
            ..counted("page.md")
        };
        assert_eq!(
            misses(&wordy),
            ["1001 words in 50 sentences, over the target of 20 words a sentence"]
        );
        // One sentence in ten over 25 words isn't under the target; 5 in 51 is.
        let long = Page {
            long: 5,
            ..counted("page.md")
        };
        assert_eq!(
            misses(&long),
            ["5 of 50 sentences over 25 words, not under the target of 1 in 10"]
        );
        let under = Page {
            long: 5,
            sentences: 51,
            sentence_words: 1020,
            ..counted("page.md")
        };
        assert!(misses(&under).is_empty(), "{:?}", misses(&under));
        // First person counts only on a reference page.
        let personal = Page {
            first_person: 1,
            ..counted("page.md")
        };
        assert!(misses(&personal).is_empty());
        let reference = Page {
            reference: true,
            ..personal
        };
        assert_eq!(
            misses(&reference),
            [
                "First person on a reference page: 1 in 1000 words, over the limit of 0.00 per 100 words"
            ]
        );
        // A page with no sentences meets both targets.
        let empty = Page {
            path: "page.md".to_owned(),
            ..Page::default()
        };
        assert!(misses(&empty).is_empty());
    }

    #[test]
    fn a_held_page_that_misses_or_isnt_read_fails_and_an_unheld_one_does_not() {
        let mut pages: Vec<Page> = HELD.iter().map(|path| counted(path)).collect();
        pages.push(Page {
            bold_labels: 50,
            ..counted("docs/unheld.md")
        });
        assert_eq!(held_failure(&pages), Ok(()));
        pages[0].long = 5;
        let error = held_failure(&pages).unwrap_err();
        assert!(
            error.starts_with(&format!("{}: 5 of 50 sentences over 25 words", HELD[0])),
            "{error}"
        );
        assert!(
            error.ends_with("rewrite the sentences, don't drop the page"),
            "{error}"
        );
        pages.remove(0);
        let error = held_failure(&pages).unwrap_err();
        assert!(
            error.starts_with(&format!(
                "{} is held, but it isn't one of the pages read",
                HELD[0]
            )),
            "{error}"
        );
    }

    /// The site's check reports both failures at once: a stock phrase and a held page that misses.
    #[test]
    fn the_check_fails_on_a_stock_phrase_and_on_a_held_page() {
        let mut pages: Vec<Page> = HELD.iter().map(|path| counted(path)).collect();
        assert_eq!(failures(&pages), Ok(()));
        pages[0].long = 5;
        pages.push(markdown("Run it in order to check the margin."));
        let error = failures(&pages).unwrap_err();
        assert!(error.contains("is on the stock-phrase list"), "{error}");
        assert!(error.contains("held pages that miss"), "{error}");
    }

    #[test]
    fn the_held_pages_pass() {
        let root = crate::designs::root().unwrap();
        let pages = repository_pages(&root).unwrap();
        assert_eq!(held_failure(&pages), Ok(()));
    }

    #[test]
    fn the_committed_report_matches_the_pages() {
        let root = crate::designs::root().unwrap();
        let pages = repository_pages(&root).unwrap();
        let committed = fs::read_to_string(root.join(REPORT)).unwrap_or_default();
        assert!(
            committed == report(&pages),
            "{REPORT} doesn't match the pages: run `cargo xtask voice`"
        );
    }
}
