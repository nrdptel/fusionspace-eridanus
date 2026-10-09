//! How far to trust a result: the note `hpr sim`, `hpr mc`, the plot, `hpr weather`,
//! `hpr analyze` and `hpr motors show` end with (ADR-211, ADR-213).
//!
//! The note has the three parts the design system's writing guide asks of every result someone
//! might fly on, in order: what kind of figure it is, what it was checked against with numbers,
//! and what to rely on instead. Its numbers are the committed report's:
//! `validation/reports/fixture-flights.json`, HPR Sim's apogee against 55 logged flights of
//! fliers' own designs, flown as drawn (M2.3c1, ADR-184). That is the largest comparison with
//! real flights the project has, and the least flattering: on it the simulated apogee averages
//! 9.8% above the logged apogee, each the height climbed in the day's air. A test holds each
//! figure here to the report, so a regenerated report that moves one fails until the note is
//! updated.

use std::fmt::Write as _;

/// The site, where each note's page is.
pub(crate) const SITE: &str = "https://hpr.fusionspace.co/";

/// Where the comparisons are set out in full.
pub(crate) const ACCURACY_URL: &str = "https://hpr.fusionspace.co/accuracy.html";

/// What comes before the link at the note's end.
pub(crate) const MORE: &str = " More: ";

/// The note's label, as the site writes it.
pub(crate) const LABEL: &str = "How far to trust it.";

/// The flights the report compares: those HPR Sim flies of the collection's logged flights.
pub(crate) const FLIGHTS: usize = 55;
/// The mean of the apogee errors, percent, to a tenth: simulated less logged, over logged, so
/// positive is a simulated apogee above the logged one.
pub(crate) const MEAN_PERCENT: f64 = 9.8;
/// The flights whose error is strictly smaller than 10% either way.
pub(crate) const WITHIN_10: usize = 27;
/// The flights whose error is below zero: a simulated apogee under the logged one.
pub(crate) const LOW: usize = 14;
/// How far below the logged apogee the lowest simulated apogee read, at most, percent: the lower
/// edge of the lowest histogram bin holding a flight.
pub(crate) const LOW_AT_MOST_PERCENT: f64 = 20.0;

/// What kind of result a command printed: the kind every figure in it is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum Kind {
    /// From a model of the flight, not measured.
    Simulated,
    /// Measured: an altimeter's log, or a weather balloon's sounding.
    Measured,
    /// A weather model's forecast.
    Forecast,
    /// A reanalysis: a weather model's fit to past observations, such as ERA5.
    Reanalysis,
    /// Copied from a data file, such as a motor's curve file, or computed from it.
    Copied,
}

/// The note's second and third parts: what the apogee was checked against, with the report's
/// numbers, and what to rely on instead, with where to read more.
fn checked_and_instead() -> String {
    format!(
        "Against {FLIGHTS} logged flights of fliers' own designs, each logged apogee the height \
         climbed in the day's air, the simulated apogee averaged {MEAN_PERCENT:.1}% above the \
         logged one; it was within 10% on {WITHIN_10} and read low on {LOW}, by at most \
         {LOW_AT_MOST_PERCENT:.0}%. Plan a waiver or a field's ceiling with room \
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

/// What `hpr weather`'s notes say to go by instead: the same for every source.
const WEATHER_INSTEAD: &str = "On launch day, go by the latest forecast and the wind measured at \
                               the field; the RSO decides.";

/// Not checked yet, for every weather source: what its figures would need to be compared with.
const WEATHER_UNCHECKED: &str = "not yet checked here against measured winds or a flight's log";

/// `hpr weather`'s note on a weather model's forecast, Open-Meteo's: its profile gives back the
/// answer's levels, checked on recorded answers (`docs/weather.md`).
pub(crate) fn open_meteo() -> String {
    format!(
        "{LABEL} A weather model's forecast, not a measurement. HPR Sim's profile gives back \
         every level it keeps of Open-Meteo's answer to rounding error, checked on recorded \
         answers; the \
         forecast itself is {WEATHER_UNCHECKED}. {WEATHER_INSTEAD}{MORE}{SITE}weather.html"
    )
}

/// `hpr weather`'s note on GFS and RAP, decoded from NOAA's files: the decoder is checked
/// against ecCodes on recorded files (`docs/nomads.md`).
pub(crate) fn nomads() -> String {
    format!(
        "{LABEL} A weather model's forecast, not a measurement. HPR Sim decodes the model's file \
         as ecCodes, the European weather center's reference decoder, does, checked on recorded \
         files, and interpolates it to the site; the forecast itself is {WEATHER_UNCHECKED}. \
         {WEATHER_INSTEAD}{MORE}{SITE}nomads.html"
    )
}

/// `hpr weather`'s note on a University of Wyoming balloon sounding (`docs/soundings.md`).
pub(crate) fn sounding() -> String {
    format!(
        "{LABEL} A weather balloon's measurement, at its station and its time, not at the launch \
         site. HPR Sim's profile gives back every level it keeps of the archive's answer to \
         rounding error, checked on recorded soundings; how much the distance to the station and the \
         hours between change a flight is not yet checked. {WEATHER_INSTEAD}\
         {MORE}{SITE}soundings.html"
    )
}

/// The public flights flown in ERA5 weather and compared with their logs: the committed report
/// `validation/reports/real-flights.json` (M2.3b).
pub(crate) const ERA5_FLIGHTS: usize = 7;
/// Their mean absolute apogee error, percent, to a hundredth, as the report's summary gives it.
pub(crate) const ERA5_MEAN_ABSOLUTE_PERCENT: f64 = 6.04;

/// `hpr weather`'s note on an ERA5 file (`docs/format/era5.md`). The flights it quotes test the
/// simulation and the weather together, so they bound neither alone.
pub(crate) fn reanalysis() -> String {
    format!(
        "{LABEL} A reanalysis: a weather model's fit to past observations, on its grid and at its \
         hours, not a measurement at the site. HPR Sim reads the file as RocketPy, an open-source \
         simulator, does, checked on two real launch days, and reads no humidity, so the air is \
         taken as dry. {ERA5_FLIGHTS} public flights flown in ERA5 weather missed their logged \
         apogees by {ERA5_MEAN_ABSOLUTE_PERCENT:.2}% on average, the simulation's error and the \
         weather's together; the winds themselves are not yet checked against measured winds. \
         For a flight flown, the altimeter's log is the measurement. \
         {WEATHER_INSTEAD}{MORE}{SITE}format/era5.html"
    )
}

/// `hpr analyze`'s note: readings of an altimeter's log (`docs/reading-a-flight-log.md`).
pub(crate) fn log() -> String {
    format!(
        "{LABEL} Measured by the altimeter's barometer, from its own log, read by HPR Sim, not \
         simulated. HPR Sim's readings are checked on an invented log whose every number is \
         known, and once by hand on a real one, not in the automatic tests; nothing yet checks a \
         barometer's errors near Mach 0.9, which can upset the top speed and the heights near \
         it. For a record or a certification, the altimeter's own reading is the one to log, and \
         the RSO decides.{MORE}{SITE}reading-a-flight-log.html"
    )
}

/// What `hpr motors show`'s notes say about the figures and what to go by instead.
const MOTOR_FIGURES: &str = "the size, masses and delays as the file's header gives them, the \
                             impulse, thrusts and burn time computed from its curve, which can \
                             differ from the maker's rated figures";

/// What `hpr motors show`'s notes say to go by instead.
const MOTOR_INSTEAD: &str = "The motor's printed data and its maker's instructions come first, \
                             and the RSO decides.";

/// `hpr motors show`'s note on the bundled catalog, its files downloaded on `captured`: the
/// total impulse and peak thrust are held to OpenRocket's on every bundled curve
/// (`hpr_motor::catalog`'s `openrocket_s_total_impulse_matches_every_bundled_curve`).
pub(crate) fn catalog_motor(captured: &str) -> String {
    format!(
        "{LABEL} Copied from ThrustCurve.org's curve file, downloaded {captured}, not measured \
         by HPR Sim: {MOTOR_FIGURES}. The total impulse and peak thrust match OpenRocket's \
         reading of the same file on every bundled curve, a check of the arithmetic, not of the \
         motor; no figure is checked against the maker's or the certifying bodies' data. {MOTOR_INSTEAD}{MORE}{SITE}pick-a-motor.html"
    )
}

/// `hpr motors show`'s note on a motor file the user named.
pub(crate) fn motor_file() -> String {
    format!(
        "{LABEL} Read from the motor file named above, not measured by HPR Sim: {MOTOR_FIGURES}. \
         HPR Sim doesn't check a motor file against the maker's or the certifying bodies' data. \
         {MOTOR_INSTEAD}{MORE}{SITE}pick-a-motor.html"
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

    /// The ERA5 note's flights and their mean error are the committed report's.
    #[test]
    fn the_era5_notes_numbers_are_the_committed_reports() {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../validation/reports/real-flights.json"
        );
        let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{path}: {e}"));
        let report: serde_json::Value =
            serde_json::from_str(&text).unwrap_or_else(|e| panic!("{path}: {e}"));
        let summary = &report["summary"];
        assert_eq!(summary["flights"].as_u64(), Some(ERA5_FLIGHTS as u64));
        let mean = summary["mean_absolute_apogee_error_percent"]
            .as_f64()
            .unwrap_or(f64::NAN);
        assert_eq!(
            format!("{mean:.2}"),
            format!("{ERA5_MEAN_ABSOLUTE_PERCENT:.2}")
        );
        assert!(reanalysis().contains("7 public flights flown in ERA5 weather missed"));
        assert!(reanalysis().contains("by 6.04% on average"));
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
