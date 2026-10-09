//! How far to trust a result: the note `hpr sim`, `hpr mc` and the plot end with (ADR-211).
//!
//! The note has the three parts the design system's writing guide asks of every result someone
//! might fly on, in order: what kind of figure it is, what it was checked against with numbers,
//! and what to rely on instead. Its numbers are the committed report's:
//! `validation/reports/fixture-flights.json`, HPR Sim's apogee against 55 logged flights of
//! fliers' own designs, flown as drawn (M2.3c1, ADR-184). That is the largest comparison with
//! real flights the project has, and the least flattering: on it the simulated apogee averages
//! 9.8% above the altimeter's. A test holds each figure here to the report, so a regenerated
//! report that moves one fails until the note is updated.

use std::fmt::Write as _;

/// Where the comparisons are set out in full.
pub(crate) const ACCURACY_URL: &str = "https://hpr.fusionspace.co/accuracy.html";

/// What comes before the link at the note's end.
pub(crate) const MORE: &str = " More: ";

/// The note's label, as the site writes it.
pub(crate) const LABEL: &str = "How far to trust it.";

/// The flights the report compares: those HPR Sim flies of the collection's logged flights.
pub(crate) const FLIGHTS: usize = 55;
/// The mean of the apogee errors, percent, to a tenth: simulated less logged, over logged, so
/// positive is a simulated apogee above the altimeter's.
pub(crate) const MEAN_PERCENT: f64 = 9.8;
/// The flights whose error is strictly smaller than 10% either way.
pub(crate) const WITHIN_10: usize = 27;
/// The flights whose error is below zero: a simulated apogee under the altimeter's.
pub(crate) const LOW: usize = 14;
/// How far below the altimeter's the lowest simulated apogee read, at most, percent: the lower
/// edge of the lowest histogram bin holding a flight.
pub(crate) const LOW_AT_MOST_PERCENT: f64 = 20.0;

/// What kind of result a command printed: the kind every figure in it is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    /// From a model of the flight, not measured.
    Simulated,
}

/// The note's second and third parts: what the apogee was checked against, with the report's
/// numbers, and what to rely on instead, with where to read more.
fn checked_and_instead() -> String {
    format!(
        "Against {FLIGHTS} logged flights of fliers' own designs, the simulated apogee averaged \
         {MEAN_PERCENT:.1}% above the altimeter's; it was within 10% on {WITHIN_10} and low on \
         {LOW}, by at most {LOW_AT_MOST_PERCENT:.0}%. Plan a waiver or a field's ceiling with room \
         above this apogee; the altimeter's reading is the one to log, and the RSO decides.\
         {MORE}{ACCURACY_URL}"
    )
}

/// `hpr sim`'s note, and the plot's: one flight.
pub(crate) fn flight() -> String {
    format!(
        "{LABEL} Simulated from the design file, not measured. {}",
        checked_and_instead()
    )
}

/// `hpr mc`'s note: the spread it prints is its inputs', not the model's error.
pub(crate) fn runs() -> String {
    format!(
        "{LABEL} Simulated from the design file, not measured; the spread above comes from the \
         inputs' scatter alone, not from the model's error. {}",
        checked_and_instead()
    )
}

/// `text` in lines of at most `width` characters, broken at spaces; a word longer than `width`
/// gets a line of its own.
pub(crate) fn wrap(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut line = String::new();
    for word in text.split_whitespace() {
        if !line.is_empty() && line.chars().count() + 1 + word.chars().count() > width {
            lines.push(std::mem::take(&mut line));
        }
        if !line.is_empty() {
            line.push(' ');
        }
        // Writing to a String can't fail.
        let _ = write!(line, "{word}");
    }
    if !line.is_empty() {
        lines.push(line);
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The committed report, read from the repository.
    fn report() -> serde_json::Value {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../validation/reports/fixture-flights.json"
        );
        let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{path}: {e}"));
        serde_json::from_str(&text).unwrap_or_else(|e| panic!("{path}: {e}"))
    }

    /// The bounds of the report's histogram bins, percent, each bin closed below, as its
    /// Markdown form states them.
    const BINS: [f64; 13] = [
        -30.0, -25.0, -20.0, -15.0, -10.0, -5.0, 0.0, 5.0, 10.0, 15.0, 20.0, 25.0, 30.0,
    ];

    #[test]
    fn the_notes_numbers_are_the_committed_reports() {
        let report = report();
        let set = &report["sets"][0];
        assert_eq!(set["name"], "every flight compared");
        let hpr = &set["hpr"];
        assert_eq!(hpr["n"].as_u64(), Some(FLIGHTS as u64));
        let mean = hpr["mean_percent"].as_f64().unwrap_or(f64::NAN);
        assert_eq!(format!("{mean:.1}"), format!("{MEAN_PERCENT:.1}"));
        assert_eq!(hpr["within_10_percent"].as_u64(), Some(WITHIN_10 as u64));
        let histogram: Vec<u64> = hpr["histogram"]
            .as_array()
            .map(|bins| bins.iter().filter_map(serde_json::Value::as_u64).collect())
            .unwrap_or_default();
        assert_eq!(histogram.len(), BINS.len() + 1);
        assert_eq!(histogram.iter().sum::<u64>(), FLIGHTS as u64);
        // Bin i holds errors from BINS[i - 1] up to BINS[i]; the first is open below.
        let zero = BINS.iter().position(|&b| b == 0.0).unwrap_or(BINS.len());
        let low: u64 = histogram[..=zero].iter().sum();
        assert_eq!(low, LOW as u64);
        let lowest = histogram.iter().position(|&n| n > 0).unwrap_or(0);
        assert!(
            lowest > 0,
            "a flight below the lowest bound has no edge to quote"
        );
        assert_eq!(-BINS[lowest - 1], LOW_AT_MOST_PERCENT);
    }

    #[test]
    fn the_bins_are_the_reports() {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../validation/reports/fixture-flights.md"
        );
        let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{path}: {e}"));
        let bounds = BINS
            .iter()
            .map(|b| {
                if *b >= 0.0 {
                    format!("+{b}")
                } else {
                    format!("{b}")
                }
            })
            .collect::<Vec<_>>()
            .join(", ");
        assert!(
            text.contains(&format!("bounded at {bounds}%, each closed below")),
            "{bounds}"
        );
    }

    #[test]
    fn each_note_has_its_three_parts_in_order() {
        for note in [flight(), runs()] {
            let kind = note.find("not measured").unwrap_or(usize::MAX);
            let checked = note.find("logged flights").unwrap_or(usize::MAX);
            let instead = note.find("the one to log").unwrap_or(usize::MAX);
            assert!(note.starts_with(LABEL), "{note}");
            assert!(kind < checked && checked < instead, "{note}");
            assert!(note.ends_with(ACCURACY_URL), "{note}");
        }
        assert!(runs().contains("not from the model's error"));
    }

    #[test]
    fn wrapping_keeps_every_word_within_the_width() {
        let note = flight();
        let lines = wrap(&note, 60);
        assert!(lines.len() > 1);
        assert_eq!(lines.join(" "), note);
        for line in &lines {
            assert!(line.chars().count() <= 60 || !line.contains(' '), "{line}");
        }
        assert_eq!(wrap("", 10), Vec::<String>::new());
        assert_eq!(wrap("abcdefghijkl x", 5), ["abcdefghijkl", "x"]);
    }
}
