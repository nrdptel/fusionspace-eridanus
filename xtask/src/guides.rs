//! The guides of *Rocketry explained* (M0.7, ADR-195, ADR-242): every number in a guide's text is
//! read from a committed report by a quote here, and every figure is drawn from one.
//!
//! A guide's text may hold a number only where a [`Quote`] puts it: the check fills each quote's
//! holes from the committed reports and fails a page that doesn't hold the result word for word,
//! and it fails any number in the page's text that no quote covers. The text is what a reader
//! reads as prose: paragraphs, list items, table cells, headings and image descriptions. Code is
//! not, since it quotes commands and their output, so the numbers in a block `cargo xtask cli`
//! regenerates, or in Python and its output that the Python package's docs test runs and rewrites
//! (`crates/hpr-py/tests/test_docs.py`), follow the program instead; nor is a decision record's
//! label (`ADR-211`).
//!
//! A quote reads the reports through [`Reports`], which records each value read. The tests move
//! every value a quote read, one at a time, and require each hole to change under one of the moves
//! and the page check to fail on it: a mutation probe for every number a guide quotes.
//!
//! The figures are drawn from the same reports, by `cargo xtask guides`, and held to the figure
//! checks; `--check`, run by a test in CI, fails a committed figure that is not what the reports
//! draw now. The same command writes the three reports the guides own, the guide rocket's center of
//! pressure ([`stability`]), its fin-flutter numbers ([`flutter`]) and its drag ([`drag`]), and
//! `--check` fails each when the design no longer gives it.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use serde_json::Value;

use crate::fixture_flights::{BINS_PERCENT, REPORT_JSON as FIXTURE, REPORT_MD as FIXTURE_PAGE};

mod compared;
mod drag;
mod flutter;
mod stability;
mod tour;

pub const USAGE: &str = "  \
guides [--check]         Write the guide rocket's center-of-pressure, flutter and drag reports, draw
                           the Rocketry explained guides' figures from the committed reports, and
                           check every number in the guides' text against the reports. With
                           --check, fail on a stale report or figure instead of writing it.";

/// HPR Sim's apogees against the public flights' logs (M2.3b).
const REAL: &str = "validation/reports/real-flights.json";

/// The reports a quote or a figure may read, as JSON.
const REPORTS: [&str; 8] = [
    FIXTURE,
    REAL,
    crate::scoreboard::SCOREBOARD_JSON,
    stability::REPORT,
    stability::TUNNEL,
    flutter::REPORT,
    drag::REPORT,
    drag::TUNNEL,
];
/// The reports a quote or a figure may read as text, each read whole at the pointer `""`: the
/// fixture report's page, which holds the weather's attribution.
const REPORT_PAGES: [&str; 1] = [FIXTURE_PAGE];

/// One guide: its page, the quotes its text holds, and the figures it shows.
struct Guide {
    /// The page, from the repository's root.
    page: &'static str,
    /// The quotes its text holds.
    quotes: Vec<Quote>,
    /// Each figure's file, from the root, and what draws it.
    figures: Vec<(&'static str, Draw)>,
}

/// What draws a figure from the reports.
type Draw = fn(&Reports) -> Result<String, String>;

/// Text a page holds word for word, its holes (`{}`) filled from the reports.
struct Quote {
    /// The text, `{}` for each hole.
    template: &'static str,
    /// The holes' values, in order.
    holes: fn(&Reports) -> Result<Vec<String>, String>,
}

impl Quote {
    /// The text, with its holes filled.
    fn render(&self, reports: &Reports) -> Result<String, String> {
        let holes = (self.holes)(reports)?;
        let parts: Vec<&str> = self.template.split("{}").collect();
        if parts.len() != holes.len() + 1 {
            return Err(format!(
                "the quote \"{}\" has {} holes and {} values",
                self.template,
                parts.len() - 1,
                holes.len()
            ));
        }
        let mut text = parts[0].to_owned();
        for (hole, part) in holes.iter().zip(&parts[1..]) {
            text.push_str(hole);
            text.push_str(part);
        }
        Ok(text)
    }
}

/// The guides.
fn guides() -> Vec<Guide> {
    vec![
        Guide {
            page: "docs/trusting-a-simulation.md",
            quotes: trust_quotes(),
            figures: vec![("docs/images/trust-apogee-misses.svg", draw_misses)],
        },
        Guide {
            page: "docs/stability-in-flight.md",
            quotes: stability::quotes(),
            figures: vec![(
                "docs/images/stability-cp-against-mach.svg",
                stability::draw_cp_against_mach,
            )],
        },
        Guide {
            page: "docs/when-fins-flutter.md",
            quotes: flutter::quotes(),
            figures: vec![(
                "docs/images/flutter-speed-by-thickness.svg",
                flutter::draw_speed_by_thickness,
            )],
        },
        Guide {
            page: tour::LANDING,
            quotes: tour::landing_quotes(),
            figures: Vec::new(),
        },
        Guide {
            page: tour::TOUR,
            quotes: tour::tour_quotes(),
            figures: Vec::new(),
        },
        Guide {
            page: crate::features::PAGE,
            quotes: compared::quotes(),
            figures: Vec::new(),
        },
        Guide {
            page: "docs/drag-through-mach-1.md",
            quotes: drag::quotes(),
            figures: vec![(
                "docs/images/drag-against-mach.svg",
                drag::draw_drag_against_mach,
            )],
        },
    ]
}

/// The committed reports, parsed, and every value read from them.
pub(crate) struct Reports {
    files: BTreeMap<&'static str, Value>,
    /// Each value read: its report and its JSON pointer.
    read: RefCell<Vec<(&'static str, String)>>,
}

impl Reports {
    /// Reads [`REPORTS`] and [`REPORT_PAGES`] under `root`.
    fn load(root: &Path) -> Result<Self, String> {
        let mut files = BTreeMap::new();
        for file in REPORTS {
            let text =
                fs::read_to_string(root.join(file)).map_err(|error| format!("{file}: {error}"))?;
            let value: Value =
                serde_json::from_str(&text).map_err(|error| format!("{file}: {error}"))?;
            files.insert(file, value);
        }
        for file in REPORT_PAGES {
            let text =
                fs::read_to_string(root.join(file)).map_err(|error| format!("{file}: {error}"))?;
            files.insert(file, Value::String(text));
        }
        Ok(Self {
            files,
            read: RefCell::new(Vec::new()),
        })
    }

    /// The value at `pointer` in `file`, recorded as read.
    fn get(&self, file: &'static str, pointer: &str) -> Result<&Value, String> {
        self.read.borrow_mut().push((file, pointer.to_owned()));
        self.files
            .get(file)
            .and_then(|value| value.pointer(pointer))
            .ok_or_else(|| format!("{file} has no {pointer}"))
    }

    /// The finite number at `pointer` in `file`, recorded as read.
    fn number(&self, file: &'static str, pointer: &str) -> Result<f64, String> {
        self.get(file, pointer)?
            .as_f64()
            .filter(|value| value.is_finite())
            .ok_or_else(|| format!("{file}'s {pointer} is not a number"))
    }

    /// The count at `pointer` in `file`, recorded as read.
    fn count(&self, file: &'static str, pointer: &str) -> Result<usize, String> {
        self.get(file, pointer)?
            .as_u64()
            .and_then(|count| usize::try_from(count).ok())
            .ok_or_else(|| format!("{file}'s {pointer} is not a count"))
    }

    /// The text at `pointer` in `file`, recorded as read.
    fn text(&self, file: &'static str, pointer: &str) -> Result<&str, String> {
        self.get(file, pointer)?
            .as_str()
            .ok_or_else(|| format!("{file}'s {pointer} is not text"))
    }

    /// The pointer of the public flight `id` in [`REAL`]. Finding it reads the flights' ids, which
    /// is a lookup, not a value a quote shows, so they aren't recorded.
    fn flight(&self, id: &str) -> Result<String, String> {
        let flights = self
            .files
            .get(REAL)
            .and_then(|value| value.pointer("/flights"))
            .and_then(Value::as_array)
            .ok_or_else(|| format!("{REAL} has no flights"))?;
        let at = flights
            .iter()
            .position(|flight| flight["id"].as_str() == Some(id))
            .ok_or_else(|| format!("{REAL} has no flight `{id}`"))?;
        Ok(format!("/flights/{at}"))
    }

    /// A public flight's title without the motor its report names in brackets at the end:
    /// "Cavour, Politecnico di Torino, EuRoC 2023". The site reads a bare designation such as
    /// `L995` as a label, and the guide names the flight, not its motor.
    fn title(&self, flight: &str) -> Result<&str, String> {
        let title = self.text(REAL, &format!("{flight}/title"))?;
        Ok(match title.rfind(" (") {
            Some(at) if title.ends_with(')') => &title[..at],
            _ => title,
        })
    }

    /// Whether the flag at `pointer` in `file` is set, recorded as read.
    fn flag(&self, file: &'static str, pointer: &str) -> Result<bool, String> {
        self.get(file, pointer)?
            .as_bool()
            .ok_or_else(|| format!("{file}'s {pointer} is not a flag"))
    }

    /// The year the fixture report's page credits the Copernicus Climate Change Service with,
    /// after `lead` ("Generated using" or "Contains modified").
    fn copernicus_year(&self, lead: &str) -> Result<String, String> {
        let page = self.text(FIXTURE_PAGE, "")?;
        let marker = format!("{lead} Copernicus Climate Change Service information ");
        let at = page
            .find(&marker)
            .ok_or_else(|| format!("{FIXTURE_PAGE} doesn't say \"{marker}\""))?;
        let year: String = page[at + marker.len()..]
            .chars()
            .take_while(char::is_ascii_digit)
            .collect();
        if year.len() == 4 {
            Ok(year)
        } else {
            Err(format!("{FIXTURE_PAGE} gives no year after \"{marker}\""))
        }
    }

    /// The pointer of the fixture report's set of every flight compared, its name checked.
    fn every_flight(&self) -> Result<&'static str, String> {
        let name = self.text(FIXTURE, "/sets/0/name")?;
        if name == "every flight compared" {
            Ok("/sets/0")
        } else {
            Err(format!(
                "{FIXTURE}'s first set is \"{name}\", not every flight compared"
            ))
        }
    }

    /// HPR Sim's error histogram over every flight compared: counts in the bins
    /// [`BINS_PERCENT`] bounds, below the first bound, between each pair and at or above the last.
    fn histogram(&self) -> Result<Vec<usize>, String> {
        let set = self.every_flight()?;
        (0..=BINS_PERCENT.len())
            .map(|bin| self.count(FIXTURE, &format!("{set}/hpr/histogram/{bin}")))
            .collect()
    }
}

/// How many of the histogram's bins hold only low misses, a simulated apogee below the logged
/// one: those whose upper bound is at most zero. A miss is simulated less logged, over logged
/// (`fixture_flights`' `a_low_simulated_apogee_is_a_negative_miss` pins the sign), so these are
/// the bins of negative misses, the side that flatters a ceiling.
fn low_bins() -> usize {
    BINS_PERCENT.iter().filter(|&&bound| bound <= 0.0).count()
}

/// The flights that read low.
fn low_count(reports: &Reports) -> Result<usize, String> {
    Ok(reports.histogram()?[..low_bins()].iter().sum())
}

/// The flights that read high, or as logged: those in the bins from zero up.
fn high_count(reports: &Reports) -> Result<usize, String> {
    Ok(reports.histogram()?[low_bins()..].iter().sum())
}

/// How low the lowest miss read, from the lowest bin that holds a flight: "between 15% and 20%
/// low", or "more than 30% low" from the open bin below the first bound.
fn lowest(reports: &Reports) -> Result<String, String> {
    let histogram = reports.histogram()?;
    let bin = histogram
        .iter()
        .position(|&count| count > 0)
        .filter(|&bin| bin < low_bins())
        .ok_or("no flight read low")?;
    Ok(match bin {
        0 => format!("more than {}% low", whole(-BINS_PERCENT[0])),
        _ => format!(
            "between {}% and {}% low",
            whole(-BINS_PERCENT[bin]),
            whole(-BINS_PERCENT[bin - 1])
        ),
    })
}

/// How high the highest miss read, from the highest bin that holds a flight: "past +30% high",
/// from the open bin at or above the last bound, or "between +20% and +25% high".
fn highest(reports: &Reports) -> Result<String, String> {
    let histogram = reports.histogram()?;
    let bin = histogram
        .iter()
        .rposition(|&count| count > 0)
        .filter(|&bin| bin >= low_bins())
        .ok_or("no flight read high")?;
    let last = BINS_PERCENT.len();
    Ok(if bin == last {
        format!("past {} high", signed_whole(BINS_PERCENT[last - 1]))
    } else {
        format!(
            "between {} and {} high",
            signed_whole(BINS_PERCENT[bin - 1]),
            signed_whole(BINS_PERCENT[bin])
        )
    })
}

/// The trust guide's quotes.
fn trust_quotes() -> Vec<Quote> {
    vec![
        Quote {
            template: "{} logged flights",
            holes: |r| Ok(vec![flights(r)?.to_string()]),
        },
        Quote {
            template: "apogee averaged {} the logged one",
            holes: |r| Ok(vec![above(set_number(r, "hpr", "mean_percent")?)]),
        },
        Quote {
            template: "misses averaged {}, and {} were within 10% either way",
            holes: |r| {
                Ok(vec![
                    percent(set_number(r, "hpr", "mean_absolute_percent")?, 2),
                    set_count(r, "hpr", "within_10_percent")?.to_string(),
                ])
            },
        },
        Quote {
            template: "The misses lean {}: {} of the {} simulated apogees are above the logged one",
            holes: |r| {
                Ok(vec![
                    lean(set_number(r, "hpr", "mean_percent")?).to_owned(),
                    high_count(r)?.to_string(),
                    flights(r)?.to_string(),
                ])
            },
        },
        Quote {
            template: "the lowest read {}, the highest {}",
            holes: |r| Ok(vec![lowest(r)?, highest(r)?]),
        },
        Quote {
            template: "averaged {} and missed by {} without signs",
            holes: |r| {
                Ok(vec![
                    above(set_number(r, "openrocket", "mean_percent")?),
                    percent(set_number(r, "openrocket", "mean_absolute_percent")?, 2),
                ])
            },
        },
        Quote {
            template: "within 5% on {} of the {} flights",
            holes: |r| {
                Ok(vec![
                    set_count(r, "hpr_over_openrocket", "within_5_percent")?.to_string(),
                    set_count(r, "hpr_over_openrocket", "n")?.to_string(),
                ])
            },
        },
        Quote {
            template: "{} public flights",
            holes: |r| Ok(vec![r.count(REAL, "/summary/flights")?.to_string()]),
        },
        Quote {
            template: "missed them by {} on average without signs, from {} to {} one by one, and \
                       with signs by {}, so on these the highs and lows nearly cancel",
            holes: |r| {
                let flights = r.get(REAL, "/flights")?.as_array().map_or(0, Vec::len);
                if flights == 0 || flights != r.count(REAL, "/summary/flights")? {
                    return Err(format!("{REAL}'s flights and its summary's count differ"));
                }
                let mut misses = Vec::new();
                for at in 0..flights {
                    misses.push(r.number(REAL, &format!("/flights/{at}/apogee_error_percent"))?);
                }
                let least = misses.iter().copied().fold(f64::INFINITY, f64::min);
                let most = misses.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                let unsigned = r.number(REAL, "/summary/mean_absolute_apogee_error_percent")?;
                let signed = r.number(REAL, "/summary/mean_apogee_error_percent")?;
                // "Nearly cancel" holds while the signed mean is under a quarter of the unsigned.
                if signed.abs() >= unsigned / 4.0 {
                    return Err(format!(
                        "the public flights' signed mean, {signed:.2}%, no longer nearly cancels: \
                         reword the page and this quote"
                    ));
                }
                Ok(vec![
                    percent(unsigned, 2),
                    signed_percent(least, 2),
                    signed_percent(most, 2),
                    signed_percent(signed, 2),
                ])
            },
        },
        Quote {
            template: "still outside the project's own {} target",
            holes: |r| {
                if r.flag(REAL, "/summary/within_target")? {
                    return Err("the public flights now meet their target".to_owned());
                }
                Ok(vec![percent(r.number(REAL, "/summary/target_percent")?, 0)])
            },
        },
        Quote {
            template: "quotes the {}, the larger set",
            holes: |r| Ok(vec![flights(r)?.to_string()]),
        },
        Quote {
            template: "Of the {} flights, {} read low, the lowest of them {}",
            holes: |r| {
                Ok(vec![
                    flights(r)?.to_string(),
                    low_count(r)?.to_string(),
                    lowest(r)?,
                ])
            },
        },
        Quote {
            template: "the public flight of {} read {} on its altimeter at the top, and an altimeter \
                       riding HPR Sim's flight of its design would have read {}. The miss is ({} − \
                       {}) / {} = {}",
            holes: |r| {
                let flight = r.flight("cavour")?;
                let logged = r.number(REAL, &format!("{flight}/log_apogee_m"))?;
                let simulated = r.number(REAL, &format!("{flight}/hpr_apogee_m"))?;
                let miss = r.number(REAL, &format!("{flight}/apogee_error_percent"))?;
                Ok(vec![
                    r.title(&flight)?.to_owned(),
                    meters(logged, 0),
                    meters(simulated, 0),
                    fixed(simulated, 0),
                    fixed(logged, 0),
                    fixed(logged, 0),
                    signed_percent(miss, 2),
                ])
            },
        },
        Quote {
            template: "HPR Sim's flight of {} would have read {} at the top; the team's altimeter \
                       read {}, a miss of {}",
            holes: |r| {
                let flight = r.flight("juno-iii")?;
                let logged = r.number(REAL, &format!("{flight}/log_apogee_m"))?;
                let simulated = r.number(REAL, &format!("{flight}/hpr_apogee_m"))?;
                let miss = r.number(REAL, &format!("{flight}/apogee_error_percent"))?;
                Ok(vec![
                    r.title(&flight)?.to_owned(),
                    meters(simulated, 1),
                    meters(logged, 1),
                    signed_percent(miss, 2),
                ])
            },
        },
        Quote {
            template: "altimeter read {} higher than the simulated one",
            holes: |r| {
                let flight = r.flight("juno-iii")?;
                let logged = r.number(REAL, &format!("{flight}/log_apogee_m"))?;
                let simulated = r.number(REAL, &format!("{flight}/hpr_apogee_m"))?;
                // The two readings as the page writes them, so the reader's subtraction agrees.
                let higher = written(logged, 1) - written(simulated, 1);
                if higher <= 0.0 {
                    return Err(
                        "Juno III's simulated reading is no longer below its log".to_owned()
                    );
                }
                Ok(vec![meters(higher, 1)])
            },
        },
        Quote {
            template: "across the {} flights the misses leaned {} on average",
            holes: |r| {
                Ok(vec![
                    flights(r)?.to_string(),
                    lean(set_number(r, "hpr", "mean_percent")?).to_owned(),
                ])
            },
        },
        Quote {
            template: "generated using Copernicus Climate Change Service information {}, and contains \
                       modified Copernicus Climate Change Service information {}",
            holes: |r| {
                Ok(vec![
                    r.copernicus_year("Generated using")?,
                    r.copernicus_year("Contains modified")?,
                ])
            },
        },
    ]
}

/// A mean miss with the side it lies on: "+8.73% above", "−0.25% below".
fn above(mean: f64) -> String {
    let side = if mean < 0.0 { "below" } else { "above" };
    format!("{} {side}", signed_percent(mean, 2))
}

/// Which way a mean miss leans: "high" or "low".
fn lean(mean: f64) -> &'static str {
    if mean < 0.0 { "low" } else { "high" }
}

/// The flights compared.
fn flights(reports: &Reports) -> Result<usize, String> {
    set_count(reports, "hpr", "n")
}

/// A number of the set of every flight compared, for one simulator.
fn set_number(reports: &Reports, simulator: &str, field: &str) -> Result<f64, String> {
    let set = reports.every_flight()?;
    reports.number(FIXTURE, &format!("{set}/{simulator}/{field}"))
}

/// A count of the set of every flight compared, for one simulator.
fn set_count(reports: &Reports, simulator: &str, field: &str) -> Result<usize, String> {
    let set = reports.every_flight()?;
    reports.count(FIXTURE, &format!("{set}/{simulator}/{field}"))
}

/// `value` rounded to `decimals`, as a number.
fn written(value: f64, decimals: usize) -> f64 {
    format!("{value:.decimals$}").parse().unwrap_or(f64::NAN)
}

/// `value` to `decimals` places, its thousands set off by commas and a minus sign as `−`.
fn fixed(value: f64, decimals: usize) -> String {
    let text = format!("{:.decimals$}", value.abs());
    let (whole, fraction) = text.split_once('.').unwrap_or((&text, ""));
    let mut grouped = String::new();
    for (at, digit) in whole.chars().enumerate() {
        if at > 0 && (whole.len() - at) % 3 == 0 {
            grouped.push(',');
        }
        grouped.push(digit);
    }
    if !fraction.is_empty() {
        grouped.push('.');
        grouped.push_str(fraction);
    }
    // A value that rounds to zero is written without a sign.
    if value < 0.0 && text.chars().any(|c| c.is_ascii_digit() && c != '0') {
        format!("−{grouped}")
    } else {
        grouped
    }
}

/// A whole number of `value`, as [`fixed`] writes it.
fn whole(value: f64) -> String {
    fixed(value, 0)
}

/// `value`, a whole number, with its sign: `+30%`, `−5%`, `0%`.
fn signed_whole(value: f64) -> String {
    signed_percent(value, 0)
}

/// `value` percent to `decimals` places.
fn percent(value: f64, decimals: usize) -> String {
    format!("{}%", fixed(value, decimals))
}

/// `value` percent to `decimals` places, with its sign: `+8.73%`, `−0.25%`; zero has none.
fn signed_percent(value: f64, decimals: usize) -> String {
    let text = fixed(value, decimals);
    if value > 0.0 && text.chars().any(|c| c.is_ascii_digit() && c != '0') {
        format!("+{text}%")
    } else {
        format!("{text}%")
    }
}

/// A height in meters to `decimals` places, then in feet to the foot, converted from the meters as
/// written (a foot is 0.3048 m exactly): `2,923.4 m (9,591 ft)`.
fn meters(value: f64, decimals: usize) -> String {
    let feet = written(value, decimals) / 0.3048;
    format!("{} m ({} ft)", fixed(value, decimals), fixed(feet, 0))
}

/// What a block of a page holds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    /// Text a reader reads: a paragraph, a list item, a table cell, a heading, an image's
    /// description, or a link's or an image's title.
    Prose,
    /// HTML other than a comment, which the check can't read.
    Html,
    /// A code block that `cargo xtask cli` doesn't regenerate.
    Code,
    /// Inline code that holds a digit.
    InlineCode,
}

/// The numbers a formula quotes, each with its sign: every number in its LaTeX but those that
/// are its form, an exponent or a subscript (`v^2`, `x_{12}`, `^{1.5}`) and the digits of a
/// fraction of numbers (`\tfrac12`, `\frac{1}{2}`). `$h = 3048$` quotes 3048, `$C_m = -0.25$`
/// quotes -0.25, and `$q = \tfrac12 \rho v^2$` quotes nothing.
fn formula_quotes(latex: &str) -> Vec<String> {
    let chars: Vec<char> = latex.chars().collect();
    let mut quotes = Vec::new();
    // The depths of the open braces that are a script's or a fraction's, the brace depth, and
    // how many of the next arguments are a script's or a fraction's own.
    let mut form_depth: Vec<usize> = Vec::new();
    let mut depth = 0usize;
    let mut next_is_form = 0usize;
    let mut at = 0;
    while at < chars.len() {
        let c = chars[at];
        if c == '\\' {
            let name: String = chars[at + 1..]
                .iter()
                .take_while(|c| c.is_ascii_alphabetic())
                .collect();
            if matches!(name.as_str(), "frac" | "tfrac" | "dfrac") {
                next_is_form = 2;
            }
            at += 1 + name.chars().count().max(1);
            continue;
        }
        if c.is_ascii_digit() {
            if !form_depth.is_empty() {
                // Inside a script's or a fraction's braces: its form.
                at += 1;
                continue;
            }
            if next_is_form > 0 {
                // An unbraced script or fraction takes one digit each: `v^2`, `\tfrac12`.
                next_is_form -= 1;
                at += 1;
                continue;
            }
            let start = at;
            while at < chars.len()
                && (chars[at].is_ascii_digit()
                    || (chars[at] == '.' && chars.get(at + 1).is_some_and(char::is_ascii_digit)))
            {
                at += 1;
            }
            let before = chars[..start].iter().rev().find(|c| !c.is_whitespace());
            let sign = if matches!(before, Some('-' | '−')) {
                "-"
            } else {
                ""
            };
            let digits: String = chars[start..at].iter().collect();
            quotes.push(format!("{sign}{digits}"));
            continue;
        }
        match c {
            '^' | '_' => next_is_form = 1,
            '{' => {
                depth += 1;
                if next_is_form > 0 {
                    form_depth.push(depth);
                    next_is_form -= 1;
                }
            }
            '}' => {
                if form_depth.last() == Some(&depth) {
                    form_depth.pop();
                }
                depth = depth.saturating_sub(1);
            }
            c if c.is_whitespace() => {}
            _ => next_is_form = 0,
        }
        at += 1;
    }
    quotes
}

/// A block of a page: its line, its words and what it is.
struct Block {
    line: usize,
    text: String,
    kind: Kind,
}

/// A page as the check reads it: its blocks, the text of the regenerated blocks, and what is
/// wrong with its Python markers, each with its line. The regenerated blocks are those
/// `cargo xtask cli` writes, between a `<!-- cli: example … -->` and its `<!-- cli: end -->`, and
/// the output of the Python the package's docs test runs and rewrites, between a
/// `<!-- python: example -->` and its `<!-- python: end -->`; the Python itself is written by
/// hand, so its numbers regenerate nothing.
struct Read {
    blocks: Vec<Block>,
    regenerated: String,
    markers: Vec<(usize, String)>,
}

/// Which regenerated region the reader is in.
enum Region {
    /// None: a code block is the page's own.
    Outside,
    /// `cargo xtask cli`'s: every code block is the command's run.
    Cli,
    /// The Python docs test's, opened on `line`: the opening fences of its code blocks so far.
    Python { line: usize, fences: Vec<String> },
}

/// The marker that opens a block `cargo xtask cli` regenerates, and the one that ends it.
const CLI_EXAMPLE: &str = "<!-- cli: example";
const CLI_END: &str = "<!-- cli: end -->";
/// The markers around Python and its output that the Python package's docs test runs, holds to
/// the page and rewrites (`crates/hpr-py/tests/test_docs.py`). Each must be a comment on its own,
/// as the test finds it, and the region must hold what the test runs: [`PYTHON_FENCES`]. The
/// check fails anything else, so it trusts no number the test doesn't run.
const PYTHON_EXAMPLE: &str = "<!-- python: example -->";
const PYTHON_END: &str = "<!-- python: end -->";
/// The opening fences of the code blocks a Python region holds, in order: the code, its output.
const PYTHON_FENCES: [&str; 2] = ["```python", "```text"];

/// `html` without its comments; `None` if a comment doesn't close in it, which fails closed.
fn without_comments(html: &str) -> Option<String> {
    let mut out = String::new();
    let mut rest = html;
    while let Some(at) = rest.find("<!--") {
        out.push_str(&rest[..at]);
        let end = rest[at + 4..].find("-->")?;
        rest = &rest[at + 4 + end + 3..];
    }
    out.push_str(rest);
    Some(out)
}

/// `page` as a reader reads it ([module docs](self)), block by block, each block's spaces
/// collapsed. Every block-level element starts a new block, so a nested item's words never run
/// into its parent's. Inline code stands as `\u{FFFC}` in its block, so no quote runs across it,
/// and is a block of its own when it holds a digit. A formula stands as `\u{FFFC}` too: its
/// integers are its form (an exponent, the 2 of ½, a count of terms), and each number with a
/// decimal point in it is a quote, read as prose is ([`formula_quotes`]).
fn prose(name: &str, page: &str) -> Read {
    // The comparison table's markers count on its own page only (`features.rs`).
    let tables_allowed = name == crate::features::PAGE;
    let options = Options::ENABLE_TABLES
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS
        | Options::ENABLE_GFM
        | Options::ENABLE_MATH;
    let line = |offset: usize| 1 + page[..offset].matches('\n').count();
    let mut blocks = Vec::new();
    let mut regenerated = String::new();
    let mut markers = Vec::new();
    let mut text = String::new();
    let mut start: Option<usize> = None;
    // The code block being read: where it starts, its opening line, and its text.
    let mut code: Option<(usize, String, String)> = None;
    let mut region = Region::Outside;
    // The comparison table's regions (`features.rs`), as the lines each opens and closes on.
    let mut tables: Vec<(usize, usize)> = Vec::new();
    let mut table: Option<usize> = None;
    let flush = |text: &mut String, start: &mut Option<usize>, blocks: &mut Vec<Block>| {
        let words = text.split_whitespace().collect::<Vec<_>>().join(" ");
        if let Some(at) = start.take()
            && !words.is_empty()
        {
            blocks.push(Block {
                line: line(at),
                text: words,
                kind: Kind::Prose,
            });
        }
        text.clear();
    };
    for (event, range) in Parser::new_ext(page, options).into_offset_iter() {
        let block_html = matches!(event, Event::Html(_));
        match event {
            Event::Start(Tag::CodeBlock(_)) => {
                let fence = page[range.clone()].lines().next().unwrap_or("");
                code = Some((range.start, fence.trim().to_owned(), String::new()));
            }
            Event::End(TagEnd::CodeBlock) => {
                if let Some((at, fence, body)) = code.take() {
                    match &mut region {
                        Region::Cli => {
                            regenerated.push_str(&body);
                            regenerated.push('\n');
                        }
                        Region::Python { fences, .. } => {
                            if fence == PYTHON_FENCES[1] {
                                regenerated.push_str(&body);
                                regenerated.push('\n');
                            }
                            fences.push(fence);
                        }
                        Region::Outside => blocks.push(Block {
                            line: line(at),
                            text: body,
                            kind: Kind::Code,
                        }),
                    }
                }
            }
            Event::Text(words) => match &mut code {
                Some((_, _, body)) => body.push_str(&words),
                None => {
                    start.get_or_insert(range.start);
                    text.push_str(&words);
                }
            },
            Event::InlineMath(latex) | Event::DisplayMath(latex) => {
                start.get_or_insert(range.start);
                text.push('\u{FFFC}');
                let quotes = formula_quotes(&latex);
                if !quotes.is_empty() {
                    blocks.push(Block {
                        line: line(range.start),
                        text: quotes.join(" "),
                        kind: Kind::Prose,
                    });
                }
            }
            Event::Code(span) => {
                start.get_or_insert(range.start);
                text.push('\u{FFFC}');
                if span.chars().any(|c| c.is_ascii_digit()) {
                    blocks.push(Block {
                        line: line(range.start),
                        text: span.to_string(),
                        kind: Kind::InlineCode,
                    });
                }
            }
            Event::Html(html) | Event::InlineHtml(html) => {
                let trimmed = html.trim();
                let at = line(range.start);
                let lower = trimmed.to_ascii_lowercase();
                if block_html && trimmed == PYTHON_EXAMPLE {
                    if matches!(region, Region::Outside) {
                        region = Region::Python {
                            line: at,
                            fences: Vec::new(),
                        };
                    } else {
                        markers.push((at, "a Python example opens inside another".to_owned()));
                    }
                } else if block_html && trimmed == PYTHON_END {
                    match std::mem::replace(&mut region, Region::Outside) {
                        Region::Python {
                            line: opened,
                            fences,
                        } => {
                            if fences != PYTHON_FENCES {
                                markers.push((
                                    opened,
                                    format!(
                                        "a Python example holds the code blocks {fences:?}: it \
                                         must hold one `{}` block and then one `{}` block, as the \
                                         docs test runs them",
                                        PYTHON_FENCES[0], PYTHON_FENCES[1]
                                    ),
                                ));
                            }
                        }
                        other => {
                            markers.push((at, format!("`{PYTHON_END}` closes no Python example")));
                            region = other;
                        }
                    }
                } else if lower.contains("python: example") || lower.contains("python: end") {
                    markers.push((
                        at,
                        format!(
                            "a Python marker must be a comment on its own, `{PYTHON_EXAMPLE}` or \
                             `{PYTHON_END}`, as the docs test finds it"
                        ),
                    ));
                } else if lower.contains("features:")
                    && !(tables_allowed
                        && block_html
                        && (trimmed == crate::features::REGION_START
                            || trimmed == crate::features::REGION_END))
                {
                    markers.push((
                        at,
                        format!(
                            "a comparison table's marker must be a comment on its own, \
                             `{}` or `{}`, and only on {}",
                            crate::features::REGION_START,
                            crate::features::REGION_END,
                            crate::features::PAGE
                        ),
                    ));
                } else if trimmed == crate::features::REGION_START {
                    if table.replace(at).is_some() {
                        markers.push((at, "a comparison table opens inside another".to_owned()));
                    }
                } else if trimmed == crate::features::REGION_END {
                    match table.take() {
                        Some(opened) => tables.push((opened, at)),
                        None => {
                            markers.push((at, "a comparison table's end opens none".to_owned()))
                        }
                    }
                } else if trimmed.starts_with(CLI_EXAMPLE) {
                    if matches!(region, Region::Python { .. }) {
                        markers.push((at, "a command's example opens in a Python one".to_owned()));
                    }
                    region = Region::Cli;
                } else if trimmed == CLI_END {
                    if matches!(region, Region::Python { .. }) {
                        markers.push((at, "a command's end closes a Python example".to_owned()));
                    }
                    region = Region::Outside;
                }
                let shown = without_comments(&html);
                if shown
                    .as_deref()
                    .is_none_or(|shown| !shown.trim().is_empty())
                {
                    blocks.push(Block {
                        line: line(range.start),
                        text: trimmed.to_owned(),
                        kind: Kind::Html,
                    });
                }
            }
            Event::SoftBreak | Event::HardBreak => text.push(' '),
            Event::Start(Tag::Link { title, .. } | Tag::Image { title, .. }) => {
                if !title.is_empty() {
                    blocks.push(Block {
                        line: line(range.start),
                        text: title.to_string(),
                        kind: Kind::Prose,
                    });
                }
            }
            Event::Start(
                Tag::Paragraph
                | Tag::Heading { .. }
                | Tag::Item
                | Tag::List(_)
                | Tag::TableCell
                | Tag::BlockQuote(_)
                | Tag::FootnoteDefinition(_),
            )
            | Event::End(
                TagEnd::Paragraph
                | TagEnd::Heading(_)
                | TagEnd::Item
                | TagEnd::List(_)
                | TagEnd::TableCell
                | TagEnd::BlockQuote(_)
                | TagEnd::FootnoteDefinition,
            ) => flush(&mut text, &mut start, &mut blocks),
            _ => {}
        }
    }
    flush(&mut text, &mut start, &mut blocks);
    if let Some(opened) = table {
        markers.push((opened, "a comparison table never ends".to_owned()));
    }
    // The table is written from its record, which `features.rs` holds to its rules and
    // `--check` to the page, so its words and numbers follow the record, as a command's example
    // follows the program.
    blocks.retain(|block| {
        !(tables.iter()).any(|&(opened, closed)| (opened..=closed).contains(&block.line))
    });
    if let Region::Python { line: opened, .. } = region {
        markers.push((
            opened,
            format!("a Python example never closes with `{PYTHON_END}`"),
        ));
    }
    Read {
        blocks,
        regenerated,
        markers,
    }
}

/// `text` without its decision records' labels, `ADR-211`, which name a record, not a quantity.
fn without_labels(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(at) = rest.find("ADR-") {
        out.push_str(&rest[..at]);
        let after = &rest[at + 4..];
        let digits = after.chars().take_while(char::is_ascii_digit).count();
        out.push(' ');
        rest = &after[digits..];
    }
    out.push_str(rest);
    out
}

/// Where `quote` stands in `text` as a whole: not glued to a word or a longer number around it,
/// so "53 logged flights" is not in "153 logged flights", "M53 logged flights" or "−53 logged
/// flights", and "within 5%" is not in "within 5.5%".
fn occurrences(text: &str, quote: &str) -> Vec<std::ops::Range<usize>> {
    let starts_with_digit = quote.starts_with(|c: char| c.is_ascii_digit());
    let ends_with_digit = quote.ends_with(|c: char| c.is_ascii_digit());
    text.match_indices(quote)
        .map(|(at, _)| at..at + quote.len())
        .filter(|found| {
            let before = text[..found.start].chars().next_back();
            let mut after = text[found.end..].chars();
            let (next, then) = (after.next(), after.next());
            let glued_before = before.is_some_and(|c| {
                c.is_alphanumeric() || (starts_with_digit && "−-+.,#_".contains(c))
            });
            let glued_after = next.is_some_and(|c| {
                c.is_alphanumeric()
                    || (ends_with_digit
                        && (c == '%'
                            || (matches!(c, '.' | ',')
                                && then.is_some_and(|c| c.is_ascii_digit()))))
            });
            !glued_before && !glued_after
        })
        .collect()
}

/// `text` with every whole occurrence of `quote` replaced by a space.
fn without_quote(text: &str, quote: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut from = 0;
    for found in occurrences(text, quote) {
        if found.start < from {
            continue;
        }
        out.push_str(&text[from..found.start]);
        out.push(' ');
        from = found.end;
    }
    out.push_str(&text[from..]);
    out
}

/// What is wrong with `page` (named `name`), given its quotes as rendered: a quote its text
/// doesn't hold word for word; each number in its text that no quote covers; a number in code
/// that no block on the page regenerates; and HTML, which the check can't read.
fn page_problems(name: &str, page: &str, quotes: &[String]) -> Vec<String> {
    let read = prose(name, page);
    let mut problems: Vec<String> = (read.markers.iter())
        .map(|(line, problem)| format!("{name}:{line}: {problem}"))
        .collect();
    for quote in quotes {
        let held = read
            .blocks
            .iter()
            .any(|block| block.kind == Kind::Prose && !occurrences(&block.text, quote).is_empty());
        if !held {
            problems.push(format!(
                "{name} doesn't hold \"{quote}\", as the committed reports write it now; change \
                 the page to it"
            ));
        }
    }
    // The longest quote first, so a shorter one inside it doesn't leave a piece behind.
    let mut longest_first: Vec<&String> = quotes.iter().collect();
    longest_first.sort_by_key(|quote| std::cmp::Reverse(quote.len()));
    for block in &read.blocks {
        let line = block.line;
        match block.kind {
            Kind::Html => problems.push(format!(
                "{name}:{line}: HTML in a guide (`{}`) can show what the check can't read; write \
                 it in Markdown",
                block.text
            )),
            Kind::Code => {
                if let Some(number) = crate::site::quoted(&block.text).first() {
                    problems.push(format!(
                        "{name}:{line}: `{number}` is in a code block nothing regenerates: make \
                         it an example `cargo xtask cli` runs, or Python between \
                         `{PYTHON_EXAMPLE}` and `{PYTHON_END}`, or leave the number out"
                    ));
                }
            }
            Kind::InlineCode => {
                if !read.regenerated.contains(&block.text) {
                    problems.push(format!(
                        "{name}:{line}: `{}` holds a number that no regenerated block on the \
                         page shows: a `cargo xtask cli` example's, or a Python example's output",
                        block.text
                    ));
                }
            }
            Kind::Prose => {
                let mut rest = block.text.clone();
                for quote in &longest_first {
                    rest = without_quote(&rest, quote);
                }
                for number in crate::site::quoted(&without_labels(&rest)) {
                    problems.push(format!(
                        "{name}:{line}: `{number}` is read from no report: quote it from one in \
                         xtask/src/guides.rs, show it in a block `cargo xtask cli` regenerates, \
                         or write the sentence without it"
                    ));
                }
            }
        }
    }
    problems
}

/// What is wrong with the guides as the `reports` render their quotes: every page's problems,
/// and each quote that can't be rendered.
fn problems_of(root: &Path, reports: &Reports) -> Result<Vec<String>, String> {
    let mut problems = Vec::new();
    for guide in guides() {
        let page = fs::read_to_string(root.join(guide.page))
            .map_err(|error| format!("{}: {error}", guide.page))?;
        let mut rendered = Vec::new();
        for quote in &guide.quotes {
            match quote.render(reports) {
                Ok(text) => rendered.push(text),
                Err(error) => problems.push(format!(
                    "{}: the quote \"{}\" can't be read: {error}",
                    guide.page, quote.template
                )),
            }
        }
        problems.extend(page_problems(guide.page, &page, &rendered));
    }
    Ok(problems)
}

/// Each figure, from the root, as the reports draw it, held to the figure checks.
fn figures(reports: &Reports) -> Result<Vec<(&'static str, String)>, String> {
    let mut drawn = Vec::new();
    for guide in guides() {
        for (path, draw) in guide.figures {
            let figure = draw(reports).map_err(|error| format!("{path}: {error}"))?;
            let mut problems = crate::figures::check(&figure);
            problems.extend(crate::figures::drawing::check(&figure));
            if !problems.is_empty() {
                let problems: Vec<String> = problems.iter().map(ToString::to_string).collect();
                return Err(format!(
                    "{path} draws a figure the figure check refuses; change its layout so it \
                     fits, never the crop, and draw in the tokens:\n  {}",
                    problems.join("\n  ")
                ));
            }
            drawn.push((path, figure));
        }
    }
    Ok(drawn)
}

/// Fails unless every guide's numbers are the reports' and every figure is committed as drawn.
pub(crate) fn check(root: &Path) -> Result<(), String> {
    let mut problems: Vec<String> = stability::stale(root)?.into_iter().collect();
    problems.extend(flutter::stale(root)?);
    problems.extend(drag::stale(root)?);
    problems.extend(crate::features::stale(root)?);
    let reports = Reports::load(root)?;
    problems.extend(problems_of(root, &reports)?);
    for (path, figure) in figures(&reports)? {
        if fs::read_to_string(root.join(path)).ok().as_deref() != Some(figure.as_str()) {
            problems.push(format!(
                "{path} is not what the reports draw now; run `cargo xtask guides` and commit it"
            ));
        }
    }
    if problems.is_empty() {
        Ok(())
    } else {
        Err(problems.join("\n"))
    }
}

/// Runs the command.
pub fn run(args: &[String]) -> Result<(), String> {
    let root = crate::designs::root()?;
    match args {
        [] => {
            // Written only when stale, so another platform's last bits leave no diff.
            if stability::stale(&root)?.is_some() {
                let report = stability::written(&stability::generate(&root)?)?;
                fs::write(root.join(stability::REPORT), report)
                    .map_err(|error| format!("{}: {error}", stability::REPORT))?;
            }
            if flutter::stale(&root)?.is_some() {
                let report = flutter::written(&flutter::generate(&root)?)?;
                fs::write(root.join(flutter::REPORT), report)
                    .map_err(|error| format!("{}: {error}", flutter::REPORT))?;
            }
            if drag::stale(&root)?.is_some() {
                let report = drag::written(&drag::generate(&root)?)?;
                fs::write(root.join(drag::REPORT), report)
                    .map_err(|error| format!("{}: {error}", drag::REPORT))?;
            }
            crate::features::write(&root)?;
            let reports = Reports::load(&root)?;
            for (path, figure) in figures(&reports)? {
                fs::write(root.join(path), figure).map_err(|error| format!("{path}: {error}"))?;
            }
            check(&root)?;
        }
        [flag] if flag == "--check" => check(&root)?,
        _ => return Err(format!("unknown arguments {args:?}\n\n{USAGE}")),
    }
    println!(
        "guides: every number in the guides' text is the committed reports', and every figure is \
         as they draw it"
    );
    Ok(())
}

/// Text set for an SVG's character data or attribute.
fn escaped(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('\'', "&apos;")
        .replace('"', "&quot;")
}

/// The figure's colors: the product system's light roles (`figures::drawing::TOKENS`).
const CANVAS: &str = "#F3F4F7";
const RULE: &str = "#D6DAE4";
const INK: &str = "#0B0F1C";
const INK_MUTED: &str = "#566079";
const CAUTION: &str = "#B34F0C";

/// The figure's width, px.
const WIDTH: f64 = 720.0;
/// The plot's left and right edges, top and bottom, px.
const PLOT_LEFT: f64 = 64.0;
const PLOT_RIGHT: f64 = 680.0;
const PLOT_TOP: f64 = 152.0;
const PLOT_BOTTOM: f64 = 392.0;

/// The x of the bound `BINS_PERCENT[bound]`, between bins `bound` and `bound + 1`.
fn bound_x(bound: usize) -> f64 {
    let bin = (PLOT_RIGHT - PLOT_LEFT) / (BINS_PERCENT.len() + 1) as f64;
    PLOT_LEFT + (bound + 1) as f64 * bin
}

/// The trust guide's figure: HPR Sim's apogee misses on every flight compared, by bin, the band
/// within 10% either way shaded, and the low side, which flatters a ceiling, marked.
fn draw_misses(reports: &Reports) -> Result<String, String> {
    let histogram = reports.histogram()?;
    let flights = flights(reports)?;
    if histogram.iter().sum::<usize>() != flights {
        return Err(format!("the histogram doesn't hold the {flights} flights"));
    }
    let mean = signed_percent(set_number(reports, "hpr", "mean_percent")?, 2);
    let within = set_count(reports, "hpr", "within_10_percent")?;
    let low = low_count(reports)?;
    let width = BINS_PERCENT[1] - BINS_PERCENT[0];
    // The key reads the outer bars as one bound either way, so the bounds must be symmetric.
    if BINS_PERCENT[0] != -BINS_PERCENT[BINS_PERCENT.len() - 1] {
        return Err("the histogram's outer bounds are not one bound either way".to_owned());
    }
    let bin = (PLOT_RIGHT - PLOT_LEFT) / histogram.len() as f64;
    let most = histogram.iter().copied().max().unwrap_or(0);
    // The count axis runs to the next even count above the tallest bar, in steps of two.
    let top = (most / 2 + 1) * 2;
    let per_flight = (PLOT_BOTTOM - PLOT_TOP) / top as f64;
    let height = PLOT_BOTTOM + 140.0;
    let year = reports.copernicus_year("Contains modified")?;
    let title = format!("HPR Sim's apogee against {flights} logged flights");
    let lines = [
        "Each bar counts the flights whose simulated apogee missed the logged one".to_owned(),
        format!(
            "by that much: simulated less logged, over logged, in bins of {}%.",
            whole(width)
        ),
        format!(
            "The misses averaged {mean}; {within} of {flights} were within 10% either way (shaded)."
        ),
        format!(
            "The two outer bars hold every miss past {}% either way.",
            whole(BINS_PERCENT[BINS_PERCENT.len() - 1])
        ),
    ];
    let mut svg = String::new();
    let _ = writeln!(
        svg,
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{WIDTH}\" height=\"{height}\" \
         viewBox=\"0 0 {WIDTH} {height}\" font-family=\"'Cascadia Mono', ui-monospace, Menlo, \
         Consolas, monospace\" font-size=\"12\" role=\"img\" aria-labelledby=\"title\" \
         aria-describedby=\"desc\">"
    );
    let _ = writeln!(svg, "<title id=\"title\">{}</title>", escaped(&title));
    let _ = writeln!(
        svg,
        "<desc id=\"desc\">{}</desc>",
        escaped(&lines.join(" "))
    );
    let _ = writeln!(
        svg,
        "<rect width=\"{WIDTH}\" height=\"{height}\" fill=\"{CANVAS}\"/>"
    );
    let _ = writeln!(
        svg,
        "<text x=\"40\" y=\"36\" font-size=\"18\" font-weight=\"600\" fill=\"{INK}\">{}</text>",
        escaped(&title)
    );
    for (at, line) in lines.iter().enumerate() {
        let _ = writeln!(
            svg,
            "<text x=\"40\" y=\"{}\" fill=\"{INK_MUTED}\">{}</text>",
            62 + 16 * at,
            escaped(line)
        );
    }
    // The band within 10% either way, behind the bars.
    let ten = |sign: f64| {
        BINS_PERCENT
            .iter()
            .position(|&bound| bound == sign * 10.0)
            .ok_or("no bin bound at 10%")
    };
    let (from, to) = (bound_x(ten(-1.0)?), bound_x(ten(1.0)?));
    let _ = writeln!(
        svg,
        "<rect x=\"{from:.1}\" y=\"{PLOT_TOP}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"{RULE}\"/>",
        to - from,
        PLOT_BOTTOM - PLOT_TOP
    );
    let _ = writeln!(
        svg,
        "<text x=\"{:.1}\" y=\"{}\" text-anchor=\"middle\" fill=\"{INK_MUTED}\">within 10%</text>",
        f64::midpoint(from, to),
        PLOT_TOP - 10.0
    );
    let _ = writeln!(
        svg,
        "<text x=\"{PLOT_LEFT}\" y=\"{}\" fill=\"{INK_MUTED}\">flights</text>",
        PLOT_TOP - 10.0
    );
    // The count axis.
    for count in (0..=top).step_by(2) {
        let y = PLOT_BOTTOM - count as f64 * per_flight;
        let _ = writeln!(
            svg,
            "<line x1=\"{:.1}\" y1=\"{y:.1}\" x2=\"{PLOT_LEFT}\" y2=\"{y:.1}\" stroke=\"{INK_MUTED}\" \
             stroke-width=\"1\"/><text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"end\" \
             fill=\"{INK_MUTED}\">{count}</text>",
            PLOT_LEFT - 4.0,
            PLOT_LEFT - 8.0,
            y + 4.0
        );
    }
    // The bars, each with its count above it.
    for (at, &count) in histogram.iter().enumerate() {
        if count == 0 {
            continue;
        }
        let x = PLOT_LEFT + at as f64 * bin + 4.0;
        let y = PLOT_BOTTOM - count as f64 * per_flight;
        let _ = writeln!(
            svg,
            "<rect x=\"{x:.1}\" y=\"{y:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"{INK_MUTED}\"/>\
             <text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" fill=\"{INK}\">{count}</text>",
            bin - 8.0,
            PLOT_BOTTOM - y,
            x + (bin - 8.0) / 2.0,
            y - 6.0
        );
    }
    // The axes, and zero as a reference: a chain line.
    let zero = bound_x(
        BINS_PERCENT
            .iter()
            .position(|&bound| bound == 0.0)
            .ok_or("no bin bound at zero")?,
    );
    let _ = writeln!(
        svg,
        "<line x1=\"{PLOT_LEFT}\" y1=\"{PLOT_TOP}\" x2=\"{PLOT_LEFT}\" y2=\"{PLOT_BOTTOM}\" \
         stroke=\"{INK}\" stroke-width=\"1\"/><line x1=\"{PLOT_LEFT}\" y1=\"{PLOT_BOTTOM}\" \
         x2=\"{PLOT_RIGHT}\" y2=\"{PLOT_BOTTOM}\" stroke=\"{INK}\" stroke-width=\"1\"/>\
         <line x1=\"{zero:.1}\" y1=\"{PLOT_TOP}\" x2=\"{zero:.1}\" y2=\"{PLOT_BOTTOM}\" \
         stroke=\"{INK}\" stroke-width=\"1\" stroke-dasharray=\"24 3 1 3\"/>"
    );
    for (bound, &value) in BINS_PERCENT.iter().enumerate() {
        let _ = writeln!(
            svg,
            "<text x=\"{:.1}\" y=\"{}\" text-anchor=\"middle\" fill=\"{INK_MUTED}\">{}</text>",
            bound_x(bound),
            PLOT_BOTTOM + 18.0,
            signed_whole(value).trim_end_matches('%')
        );
    }
    let _ = writeln!(
        svg,
        "<text x=\"{:.1}\" y=\"{}\" text-anchor=\"middle\" fill=\"{INK_MUTED}\">miss, %: \
         simulated apogee less logged, over logged</text>",
        f64::midpoint(PLOT_LEFT, PLOT_RIGHT),
        PLOT_BOTTOM + 40.0
    );
    // The two sides, the low one marked as the one that flatters a ceiling.
    let low_end = bound_x(low_bins() - 1);
    let y = PLOT_BOTTOM + 60.0;
    let bracket = |from: f64, to: f64, color: &str, width: f64| {
        format!(
            "<path d=\"M {from:.1} {:.1} V {y:.1} H {to:.1} V {:.1}\" fill=\"none\" \
             stroke=\"{color}\" stroke-width=\"{width}\"/>",
            y - 6.0,
            y - 6.0
        )
    };
    let _ = writeln!(
        svg,
        "{}",
        bracket(PLOT_LEFT + 2.0, low_end - 2.0, CAUTION, 2.0)
    );
    let _ = writeln!(
        svg,
        "{}",
        bracket(low_end + 2.0, PLOT_RIGHT - 2.0, INK_MUTED, 1.0)
    );
    let _ = writeln!(
        svg,
        "<text x=\"{PLOT_LEFT}\" y=\"{}\" font-weight=\"600\" fill=\"{CAUTION}\">flatters a \
         ceiling: {low} read low</text>",
        y + 20.0
    );
    let _ = writeln!(
        svg,
        "<text x=\"{PLOT_RIGHT}\" y=\"{}\" text-anchor=\"end\" fill=\"{INK_MUTED}\">mainly costs height: \
         {} read high</text>",
        y + 20.0,
        high_count(reports)?
    );
    let _ = writeln!(
        svg,
        "<text x=\"40\" y=\"{}\" fill=\"{INK_MUTED}\">Drawn by cargo xtask guides from \
         {FIXTURE}.</text>",
        height - 32.0
    );
    let _ = writeln!(
        svg,
        "<text x=\"40\" y=\"{}\" fill=\"{INK_MUTED}\">Weather: ERA5; contains modified Copernicus \
         Climate Change Service information {year}.</text>",
        height - 16.0
    );
    svg.push_str("</svg>\n");
    Ok(svg)
}

#[cfg(test)]
mod tests {

    #[test]
    fn a_formula_quotes_its_numbers_but_not_its_form() {
        let none = Vec::<String>::new();
        assert_eq!(formula_quotes("q = \\tfrac12 \\rho v^2"), none);
        assert_eq!(
            formula_quotes("\\frac{1}{2} x_{12} + y^{1.5} + C_{D0}"),
            none
        );
        assert_eq!(formula_quotes("h = 3048"), ["3048"]);
        assert_eq!(formula_quotes("C_m = -0.25"), ["-0.25"]);
        assert_eq!(
            formula_quotes("V_f = 1.8\\,a, 24\\,\\varepsilon"),
            ["1.8", "24"]
        );
        assert_eq!(formula_quotes("v^2 + 3"), ["3"]);
    }

    #[test]
    fn a_decimal_in_a_formula_is_read_as_a_quote() {
        let read = prose(
            "docs/x.md",
            "The ratio $V_f/a = 1.8$ and $q = \\tfrac12 \\rho v^2$.\n",
        );
        let quotes: Vec<&str> = read
            .blocks
            .iter()
            .filter(|b| matches!(b.kind, Kind::Prose) && b.text.contains("1.8"))
            .map(|b| b.text.as_str())
            .collect();
        assert_eq!(quotes, ["1.8"]);
        assert!(read.blocks.iter().all(|b| !b.text.contains("12")));
    }
    use super::*;

    fn reports() -> (std::path::PathBuf, Reports) {
        let root = crate::designs::root().unwrap();
        let reports = Reports::load(&root).unwrap();
        (root, reports)
    }

    /// The guides hold the reports' numbers and no others, and their figures are current.
    #[test]
    fn the_guides_are_current() {
        let root = crate::designs::root().unwrap();
        check(&root).unwrap();
    }

    /// The moves of `value` the probe tries: a count to none, or none to one, so that any bin of
    /// a histogram can empty or fill; a number by a tenth and to its negative, so a sign can
    /// flip; text with every digit moved on and a word added; a flag flipped; a list without its
    /// last item, so a count of rows can move.
    fn moves(value: &Value) -> Vec<Value> {
        match value {
            Value::Number(number) if number.is_u64() => {
                vec![Value::from(u64::from(number.as_u64() == Some(0)))]
            }
            Value::Number(number) => {
                let value = number.as_f64().unwrap_or(0.0);
                if value == 0.0 {
                    vec![Value::from(1.0), Value::from(-1.0)]
                } else {
                    vec![Value::from(value * 1.1), Value::from(-value)]
                }
            }
            Value::String(text) => {
                let moved: String = text
                    .chars()
                    .map(|c| match c.to_digit(10) {
                        Some(digit) => char::from_digit((digit + 1) % 10, 10).unwrap_or(c),
                        None => c,
                    })
                    .collect();
                vec![Value::from(format!("{moved} moved"))]
            }
            Value::Bool(flag) => vec![Value::Bool(!flag)],
            Value::Array(items) if !items.is_empty() => {
                vec![Value::from(items[..items.len() - 1].to_vec())]
            }
            _ => Vec::new(),
        }
    }

    /// The mutation probes: for every quote, each value it reads is moved in turn. Each of the
    /// quote's holes must change under one of the moves, and each move that changes the text must
    /// fail the page check, so no number a guide quotes can go stale unseen.
    #[test]
    fn every_number_a_guide_quotes_fails_when_its_report_moves() {
        let (root, reports) = reports();
        for guide in guides() {
            let page = fs::read_to_string(root.join(guide.page)).unwrap();
            let base: Vec<String> = guide
                .quotes
                .iter()
                .map(|quote| quote.render(&reports).unwrap())
                .collect();
            assert!(page_problems(guide.page, &page, &base).is_empty());
            for (index, quote) in guide.quotes.iter().enumerate() {
                reports.read.borrow_mut().clear();
                let holes = (quote.holes)(&reports).unwrap();
                let read: Vec<(&str, String)> = reports.read.borrow().clone();
                assert!(!read.is_empty(), "\"{}\" reads nothing", quote.template);
                let mut changed = vec![false; holes.len()];
                let tries = read.iter().flat_map(|(file, pointer)| {
                    let value = reports.files[file].pointer(pointer).unwrap();
                    moves(value)
                        .into_iter()
                        .map(move |value| (*file, pointer.clone(), value))
                });
                for (file, pointer, value) in tries {
                    let mut files = reports.files.clone();
                    let leaf = files
                        .get_mut(file)
                        .and_then(|value| value.pointer_mut(&pointer))
                        .unwrap();
                    *leaf = value;
                    let spoiled = Reports {
                        files,
                        read: RefCell::new(Vec::new()),
                    };
                    let Ok(now) = (quote.holes)(&spoiled) else {
                        continue;
                    };
                    if now == holes {
                        continue;
                    }
                    for (hole, (before, after)) in holes.iter().zip(&now).enumerate() {
                        changed[hole] |= before != after;
                    }
                    let mut quotes = base.clone();
                    quotes[index] = quote.render(&spoiled).unwrap();
                    assert!(
                        !page_problems(guide.page, &page, &quotes).is_empty(),
                        "{file}'s {pointer} moved and {} still passed",
                        guide.page
                    );
                }
                for (hole, changed) in changed.iter().enumerate() {
                    assert!(
                        changed,
                        "hole {hole} of \"{}\" moves with nothing it reads",
                        quote.template
                    );
                }
            }
        }
    }

    /// A number in the comparison table, which `features.rs` writes from its record, passes;
    /// the same number after the table's end fails, and a table that never ends or ends twice
    /// fails by its marker.
    #[test]
    fn the_comparison_table_follows_its_record() {
        let start = crate::features::REGION_START;
        let end = crate::features::REGION_END;
        let page = format!(
            "# A page\n\n{start}\n\n| a | b |\n|---|---|\n| OpenRocket 24.12 | yes |\n\n{end}\n"
        );
        let name = crate::features::PAGE;
        assert_eq!(page_problems(name, &page, &[]), Vec::<String>::new());
        let after = format!("{page}\nRead on 2026.\n");
        let problems = page_problems(name, &after, &[]);
        assert!(
            problems.iter().any(|p| p.contains("`2026`")),
            "{problems:?}"
        );
        let open = format!("# A page\n\n{start}\n\n| a |\n|---|\n| 24.12 |\n");
        let problems = page_problems(name, &open, &[]);
        assert!(
            problems.iter().any(|p| p.contains("never ends")),
            "{problems:?}"
        );
        let twice = format!("{page}\n{end}\n");
        let problems = page_problems(name, &twice, &[]);
        assert!(
            problems.iter().any(|p| p.contains("opens none")),
            "{problems:?}"
        );
        // On any other guide, the markers exempt nothing and fail.
        let problems = page_problems("docs/drag-through-mach-1.md", &page, &[]);
        assert!(
            problems.iter().any(|p| p.contains("only on")),
            "{problems:?}"
        );
        assert!(
            problems.iter().any(|p| p.contains("`24.12`")),
            "{problems:?}"
        );
        // A marker with text after it is no marker, so it can't hold a region open.
        let tailed = page.replace(
            &format!("{end}\n"),
            &format!("{end} tail\n\nWithin 3%.\n\n{end}\n"),
        );
        let problems = page_problems(name, &tailed, &[]);
        assert!(
            problems.iter().any(|p| p.contains("a comment on its own")),
            "{problems:?}"
        );
    }

    /// A number no quote reads fails, in a paragraph, a heading or an image's description; one
    /// in code, or a decision record's label, doesn't.
    #[test]
    fn a_number_no_quote_reads_fails() {
        let quotes = ["53 logged flights".to_owned()];
        let page = "# A page\n\nAcross 53 logged flights it was fine.\n";
        assert!(page_problems("p", page, &quotes).is_empty());
        for spoiled in [
            "# A page\n\nAcross 53 logged flights it missed by 12%.\n",
            "# A page\n\nAcross 53 logged flights.\n\n## The 3 sides\n",
            "# A page\n\nAcross 53 logged flights.\n\n![It shows 4 bars.](x.svg)\n",
            "# A page\n\nAcross 53 logged flights.\n\n- an item of 1,708 m\n",
            "# A page\n\nAcross 54 logged flights.\n",
        ] {
            let problems = page_problems("p", spoiled, &quotes);
            assert!(!problems.is_empty(), "{spoiled}");
        }
        let fine = "# A page\n\nAcross 53 logged flights, `--runs 200`, as ADR-211 decided.\n\n\
                    <!-- cli: example `hpr mc x --runs 200` -->\n\n```text\n\
                    $ hpr mc x --runs 200\n812.3\n```\n\n<!-- cli: end -->\n";
        assert_eq!(page_problems("p", fine, &quotes), Vec::<String>::new());
        // HTML can show a number the check can't read; a comment shows nothing.
        let html = "# A page\n\nAcross 53 logged flights, <span>12</span>.\n";
        assert!(!page_problems("p", html, &quotes).is_empty());
        let html = "# A page\n\n<div>Across 53 logged flights.</div>\n";
        assert!(!page_problems("p", html, &quotes).is_empty());
        let comment = "# A page\n\nAcross 53 logged flights.\n\n<!-- 12 -->\n";
        assert!(page_problems("p", comment, &quotes).is_empty());
        // Only a label's own digits go: a label at the end, or with none, leaves the rest.
        assert_eq!(without_labels("as ADR-211 said"), "as   said");
        assert_eq!(without_labels("ADR-"), " ");
        assert_eq!(without_labels("ADR-x 12 é"), " x 12 é");
        assert_eq!(
            crate::site::quoted(&without_labels("ADR-211, ADR-12 by 7%")).len(),
            1
        );
        // A page that drops a quote, with no number left behind, fails on the quote.
        let dropped = page_problems("p", "# A page\n\nAcross the logged flights.\n", &quotes);
        assert_eq!(dropped.len(), 1);
        assert!(
            dropped[0].contains("doesn't hold \"53 logged flights\""),
            "{dropped:?}"
        );
        // A nested item's number doesn't run into its parent's words.
        let nested = "# A page\n\nAcross 53 logged flights.\n\n- The misses\n  - 12% read low\n";
        assert!(!page_problems("p", nested, &quotes).is_empty());
        // HTML after a comment is still HTML; a comment that doesn't close fails closed.
        let after = "# A page\n\nAcross 53 logged flights.\n\n<!-- c --><div>99</div>\n";
        assert!(!page_problems("p", after, &quotes).is_empty());
        let open = "# A page\n\nAcross 53 logged flights.\n\n<!-- c\n12 -->\n";
        assert!(!page_problems("p", open, &quotes).is_empty());
        // Titles show as hover text, so they are read.
        for titled in [
            "# A page\n\nAcross 53 logged flights ([report](x.md \"missed by 12%\")).\n",
            "# A page\n\nAcross 53 logged flights.\n\n![bars](x.svg \"12 bars\")\n",
            "# A page\n\nAcross 53 logged flights ([report][r]).\n\n[r]: x.md \"99 of them\"\n",
        ] {
            assert!(!page_problems("p", titled, &quotes).is_empty(), "{titled}");
        }
        // A quote glued to a longer number or a word is not the quote.
        for glued in [
            "# A page\n\nAcross 153 logged flights.\n",
            "# A page\n\nAcross M53 logged flights.\n",
            "# A page\n\nAcross −53 logged flights.\n",
            "# A page\n\nAcross 53 logged flightsx.\n",
        ] {
            assert!(!page_problems("p", glued, &quotes).is_empty(), "{glued}");
        }
        // Inline code with a number must be what a regenerated block shows; a code block that
        // isn't regenerated may hold no number.
        let regenerated = "# A page\n\nAcross 53 logged flights, `--runs 200` and `95%`.\n\n\
                           <!-- cli: example `hpr mc x --runs 200` -->\n\n```text\n\
                           $ hpr mc x --runs 200\n 5%   95%\n```\n\n<!-- cli: end -->\n";
        assert_eq!(
            page_problems("p", regenerated, &quotes),
            Vec::<String>::new()
        );
        let unshown = "# A page\n\nAcross 53 logged flights, the median `1072.4` m.\n";
        assert!(!page_problems("p", unshown, &quotes).is_empty());
        let plain = "# A page\n\nAcross 53 logged flights.\n\n```text\nmiss = 12 / 100\n```\n";
        assert!(!page_problems("p", plain, &quotes).is_empty());
        // The output of Python the docs test runs is regenerated, only between its markers, and
        // only in a region holding just the code and its output; the code is written by hand.
        let python = "# A page\n\nAcross 53 logged flights, `12`.\n\n\
                      <!-- python: example -->\n\n```python\nprint(6 * 2)\n```\n\n\
                      ```text\n12\n```\n\n<!-- python: end -->\n";
        assert_eq!(page_problems("p", python, &quotes), Vec::<String>::new());
        let end = "<!-- python: end -->\n";
        for (spoiled, what) in [
            (
                python.replace("<!-- python: example -->", "<!-- python -->"),
                "unmarked",
            ),
            (
                python.replace(end, "<!-- python: end -->\n\n```text\n13\n```\n"),
                "after",
            ),
            (python.replace("`12`", "`6 * 2`"), "the code's number"),
            (
                python.replace(end, "<!-- python: end --><!-- -->\n"),
                "end with more",
            ),
            (
                python.replace(end, "The end: `<!-- python: end -->`.\n"),
                "end in prose",
            ),
            (
                python.replace(end, "~~~text\n13\n~~~\n\n<!-- python: end -->\n"),
                "a tilde",
            ),
            (
                python.replace(end, "    13\n\n<!-- python: end -->\n"),
                "indented",
            ),
            (python.replace(end, ""), "never closed"),
            (python.replace("```text\n12\n```\n", ""), "no output"),
            (python.replace("```python\n", "```py\n"), "another fence"),
            (format!("{python}\n<!-- python: end -->\n"), "closes none"),
            (
                python.replace(
                    "<!-- python: example -->\n",
                    "Text <!-- python: example -->\n",
                ),
                "inline",
            ),
            (
                python.replace("```python", "<!-- python: example -->\n\n```python"),
                "nested",
            ),
        ] {
            assert!(
                !page_problems("p", &spoiled, &quotes).is_empty(),
                "{what}: {spoiled}"
            );
        }
        // A quote split by inline code is not the quote.
        let split = "# A page\n\nAcross 53 `logged` flights.\n";
        assert!(!page_problems("p", split, &quotes).is_empty());
    }

    /// The numbers are written as the site writes them: commas for thousands, `−` for a minus,
    /// a sign on a miss, feet from the meters as written.
    #[test]
    fn numbers_are_written_as_the_site_writes_them() {
        assert_eq!(fixed(2923.3555, 1), "2,923.4");
        assert_eq!(fixed(-1234567.0, 0), "−1,234,567");
        assert_eq!(fixed(-0.004, 2), "0.00");
        assert_eq!(signed_percent(8.73, 2), "+8.73%");
        assert_eq!(signed_percent(-0.2454, 2), "−0.25%");
        assert_eq!(signed_percent(0.0, 0), "0%");
        assert_eq!(percent(12.95, 2), "12.95%");
        assert_eq!(meters(2923.3555, 1), "2,923.4 m (9,591 ft)");
        assert_eq!(meters(228.0, 0), "228 m (748 ft)");
    }

    /// The worked misses add up as the page writes them: the miss the report gives is the one
    /// a reader works from the heights the page prints, to the page's precision.
    #[test]
    fn the_worked_misses_add_up() {
        let (_, reports) = reports();
        for (id, decimals) in [("cavour", 0), ("juno-iii", 1)] {
            let flight = reports.flight(id).unwrap();
            let logged = reports
                .number(REAL, &format!("{flight}/log_apogee_m"))
                .unwrap();
            let simulated = reports
                .number(REAL, &format!("{flight}/hpr_apogee_m"))
                .unwrap();
            let miss = reports
                .number(REAL, &format!("{flight}/apogee_error_percent"))
                .unwrap();
            let worked = 100.0 * (written(simulated, decimals) - written(logged, decimals))
                / written(logged, decimals);
            assert_eq!(signed_percent(worked, 2), signed_percent(miss, 2), "{id}");
        }
    }

    /// The figure's low side is the bins of negative misses, as `fixture_flights` signs a miss:
    /// its caution bracket ends at zero, and its count is the flights in those bins.
    #[test]
    fn the_flattering_mark_is_the_low_side() {
        let (_, reports) = reports();
        let zero = BINS_PERCENT.iter().position(|&bound| bound == 0.0).unwrap();
        assert_eq!(low_bins(), zero + 1);
        assert!(BINS_PERCENT[..low_bins()].iter().all(|&bound| bound <= 0.0));
        let figure = draw_misses(&reports).unwrap();
        let caution = figure
            .lines()
            .find(|line| line.starts_with("<path") && line.contains(CAUTION))
            .unwrap();
        assert!(
            caution.contains(&format!("H {:.1} ", bound_x(zero) - 2.0)),
            "{caution}"
        );
        let low = low_count(&reports).unwrap();
        let histogram = reports.histogram().unwrap();
        assert_eq!(low, histogram[..=zero].iter().sum::<usize>());
        assert!(figure.contains(&format!("flatters a ceiling: {low} read low")));
    }

    /// The figure passes the figure checks, and draws what its description says.
    #[test]
    fn the_figure_draws_its_description() {
        let (_, reports) = reports();
        let figure = draw_misses(&reports).unwrap();
        assert!(crate::figures::check(&figure).is_empty());
        assert!(crate::figures::drawing::check(&figure).is_empty());
        let flights = flights(&reports).unwrap();
        assert!(figure.contains(&format!("against {flights} logged flights")));
    }
}
