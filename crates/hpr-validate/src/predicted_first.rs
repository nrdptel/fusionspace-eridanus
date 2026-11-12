//! The public flights predicted before they were compared, each beside its log's apogee and its
//! error ([the errors][m2-3d2], decisions [ADR-328][adr-328] §1b and [ADR-342][adr-342]).
//!
//! [`crate::public_predictions`] flies five public flights from their records and holds no
//! measured value. This module is the comparison that module left for later. For each flight it
//! reads the log's apogee by the rule merged with the prediction ([`ApogeeRule`], the machine's
//! form of [`crate::public_predictions::Reference::rule`]) and puts beside it:
//!
//! - the prediction as first merged ([`crate::public_predictions::MERGED`]), to the digit its
//!   page printed;
//! - the prediction HPR Sim flies today, which is the merged one unless a revision says why it
//!   changed ([`crate::public_predictions::REVISIONS`]);
//! - the error of each against the log's apogee, per cent of the log's:
//!   `100 (predicted − logged) / logged`.
//!
//! The error counted in the report's means is today's prediction's. Where any prediction was
//! revised, the report also gives the means over the merged numbers, so a revision can't lower
//! an error out of sight.
//!
//! Two more numbers are shown beside a flight and never counted: Andromeda's prediction on its
//! catalog curve, and for the Macie XL flights the apogee of the OpenRocket simulation stored in
//! the design file.
//!
//! [m2-3d2]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m2-3d2
//! [adr-328]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0328-the-2026-11-08-review-twelve-public-flights-predictions-first.md
//! [adr-342]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0342-the-errors-before-the-remaining-openrocket-parity-probes.md

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::public_predictions::{
    DESIGN_FILE_FLIGHTS, MERGED, MergedPrediction, NEW_FLIGHTS, PredictionsReport, REVISIONS,
    Revision, held_to,
};
use crate::real_flight::{RealFlightError, read_bytes, sha256_hex};

/// How a log's apogee is read: what [`crate::public_predictions::Reference::rule`] says in
/// words, as code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApogeeRule {
    /// The highest reading of the named column of a comma-separated log whose first line names
    /// its columns, in meters, the log read whole: a row whose reading isn't a number fails the
    /// read; it is never skipped.
    HighestOfColumn(&'static str),
    /// The apogee a PerfectFlite altimeter worked out, on the `Apogee:` line of its `.pf2`
    /// log's header, in feet above the pad (0.3048 m to the foot, exactly); never the highest
    /// sample, which an ejection charge's pressure spike can raise.
    PerfectFliteHeader,
}

/// Each predicted flight's rule, by its public id, in [`MERGED`]'s order.
///
/// The words were merged with the predictions and are not edited here; a test holds each entry
/// to them: the column it reads is the one the words name, and the header rule is the one the
/// words give for a PerfectFlite log.
pub const RULES: [(&str, ApogeeRule); 5] = [
    ("astra", ApogeeRule::HighestOfColumn("alt")),
    ("andromeda", ApogeeRule::HighestOfColumn("alt(m)")),
    ("erebus-11", ApogeeRule::HighestOfColumn("alt")),
    ("macie-xl-1", ApogeeRule::PerfectFliteHeader),
    ("macie-xl-2", ApogeeRule::PerfectFliteHeader),
];

/// A log's apogee as its rule reads it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LogApogee {
    /// The apogee, m above the pad.
    pub apogee_m: f64,
    /// The rows or samples the log holds.
    pub rows: usize,
    /// The highest sample, m, where the rule reads a header line and not the samples: shown,
    /// never compared.
    pub highest_sample_m: Option<f64>,
}

impl ApogeeRule {
    /// Whether the rule merged in words, `words`, is this one: the words name the column this
    /// reads, or give the header rule.
    #[must_use]
    pub fn is_the_rule_in(self, words: &str) -> bool {
        match self {
            Self::HighestOfColumn(column) => {
                words.contains(&format!(
                    "the highest reading of the log's `{column}` column"
                )) && words.contains("the log read whole")
            }
            Self::PerfectFliteHeader => {
                words.contains("its `Apogee:` line")
                    && words.contains("0.3048 m to the foot")
                    && words.contains("never the highest sample")
            }
        }
    }

    /// The apogee of the log whose text is `text`.
    ///
    /// # Errors
    ///
    /// What the rule can't read, in words: no such column or two of them, a row whose reading
    /// isn't a finite number, no rows, a `.pf2` the reader refuses, or a header that states no
    /// apogee.
    pub fn read(self, text: &str) -> Result<LogApogee, String> {
        match self {
            Self::HighestOfColumn(column) => highest_of_column(column, text),
            Self::PerfectFliteHeader => {
                let log = hpr_flightdata::perfectflite::read(text)
                    .map_err(|error| format!("the `.pf2` doesn't read: {error}"))?;
                let apogee_m = log
                    .stated
                    .apogee_m
                    .filter(|apogee| apogee.is_finite() && *apogee > 0.0)
                    .ok_or_else(|| "its header states no apogee".to_owned())?;
                let highest = log
                    .altitude_m
                    .iter()
                    .copied()
                    .filter(|height| height.is_finite())
                    .fold(f64::NEG_INFINITY, f64::max);
                Ok(LogApogee {
                    apogee_m,
                    rows: log.time_s.len(),
                    highest_sample_m: highest.is_finite().then_some(highest),
                })
            }
        }
    }
}

/// The highest reading of `column` in a comma-separated log whose first line names its columns.
fn highest_of_column(column: &str, text: &str) -> Result<LogApogee, String> {
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    let mut lines = text.lines();
    let header = lines.next().ok_or_else(|| "the log is empty".to_owned())?;
    let mut places = header
        .split(',')
        .enumerate()
        .filter(|(_, name)| name.trim() == column)
        .map(|(index, _)| index);
    let index = places
        .next()
        .ok_or_else(|| format!("its first line names no `{column}` column"))?;
    if places.next().is_some() {
        return Err(format!("its first line names `{column}` twice"));
    }
    let mut highest = f64::NEG_INFINITY;
    let mut rows = 0_usize;
    for (number, line) in lines.enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        // Line numbers count from 1, and the header is line 1.
        let reading = line
            .split(',')
            .nth(index)
            .and_then(|field| field.trim().parse::<f64>().ok())
            .filter(|reading| reading.is_finite())
            .ok_or_else(|| {
                format!(
                    "line {}: its `{column}` is not a number, and the rule reads the log whole",
                    number + 2
                )
            })?;
        highest = highest.max(reading);
        rows += 1;
    }
    if rows == 0 {
        return Err("the log has no rows".to_owned());
    }
    Ok(LogApogee {
        apogee_m: highest,
        rows,
        highest_sample_m: None,
    })
}

/// A number shown beside a flight's counted prediction and never counted.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShownOnly {
    /// What it is, in words.
    pub what: String,
    /// Its apogee, m, as the log's altimeter would read it.
    pub apogee_m: f64,
    /// Its error against the log's apogee, per cent.
    pub apogee_error_percent: f64,
}

/// One predicted flight beside its log, as the real-flights report keeps it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComparedPrediction {
    /// The flight's public id.
    pub id: String,
    /// The rocket, the team and the event.
    pub title: String,
    /// What measured the log's height ([`crate::real_flight::Altimeter::kind`]).
    pub altimeter: String,
    /// Where that comes from.
    pub altimeter_evidence: String,
    /// Whether the flight was flown in the ERA5 air of its launch hour; otherwise in the
    /// standard atmosphere and steady wind of a stored simulation.
    pub era5_weather: bool,
    /// The log, from the repository root.
    pub reference_log: String,
    /// Its SHA-256, in hex: the file the rule was merged for.
    pub reference_log_sha256: String,
    /// The rule its apogee is read by, as merged with the prediction.
    pub reference_rule: String,
    /// The rows or samples the log holds.
    pub log_rows: usize,
    /// The log's apogee by that rule, m above the pad.
    pub log_apogee_m: f64,
    /// The log's highest sample, m, where the rule reads a header line: shown, never compared.
    pub log_highest_sample_m: Option<f64>,
    /// The prediction as first merged, m, to the digit its page printed
    /// ([`MergedPrediction::apogee_m`]).
    pub merged_apogee_m: f64,
    /// The same as a height above the pad, m.
    pub merged_height_apogee_m: f64,
    /// The merged prediction's error against the log's apogee, per cent.
    pub merged_apogee_error_percent: f64,
    /// The prediction HPR Sim flies today, m, as the log's altimeter would read it: the merged
    /// one to its printed digit, unless revised.
    pub hpr_apogee_m: f64,
    /// The same as a height above the pad, m.
    pub hpr_height_apogee_m: f64,
    /// Today's prediction less the log's apogee, per cent of the log's: the error counted.
    pub apogee_error_percent: f64,
    /// Why today's prediction is not the merged one, when it isn't
    /// ([`Revision::because`]).
    pub revised_because: Option<String>,
    /// Whether every input is traced to the flight's record, as tagged with the prediction.
    pub all_inputs_traced: bool,
    /// What moves the apogee and no record states, as tagged with the prediction.
    pub untraced: Vec<String>,
    /// HPR Sim's largest Mach number over the flight.
    pub hpr_max_mach: f64,
    /// The flight on its catalog curve, where the counted curve is not the catalog's.
    pub catalog_curve: Option<ShownOnly>,
    /// The OpenRocket simulation stored in the design file, for a flight read from one.
    pub openrocket_stored: Option<ShownOnly>,
}

fn percent_of(value: f64, reference: f64) -> f64 {
    100.0 * (value - reference) / reference
}

/// What a prediction's row gives the comparison, whichever kind of flight it is.
struct Predicted<'a> {
    id: &'a str,
    title: &'a str,
    altimeter: &'a str,
    altimeter_evidence: &'a str,
    era5_weather: bool,
    reference_log: &'a str,
    reference_log_sha256: &'a str,
    reference_rule: &'a str,
    apogee_m: f64,
    height_apogee_m: f64,
    all_inputs_traced: bool,
    untraced: &'a [String],
    hpr_max_mach: f64,
    catalog_curve: Option<(String, f64)>,
    openrocket_stored_apogee_m: Option<f64>,
}

fn predicted(report: &PredictionsReport) -> Vec<Predicted<'_>> {
    let examples = report.flights.iter().map(|row| Predicted {
        id: &row.id,
        title: &row.title,
        altimeter: &row.altimeter,
        altimeter_evidence: &row.altimeter_evidence,
        era5_weather: true,
        reference_log: &row.reference_log,
        reference_log_sha256: &row.reference_log_sha256,
        reference_rule: &row.reference_rule,
        apogee_m: row.predicted_apogee_m,
        height_apogee_m: row.predicted_height_apogee_m,
        all_inputs_traced: row.all_inputs_traced,
        untraced: &row.untraced,
        hpr_max_mach: row.hpr_max_mach,
        catalog_curve: row.catalog_curve.as_ref().map(|catalog| {
            (
                format!("on the catalog's curve, {}", catalog.motor),
                catalog.predicted_apogee_m,
            )
        }),
        openrocket_stored_apogee_m: None,
    });
    let design_files = report.design_file_flights.iter().map(|row| Predicted {
        id: &row.id,
        title: &row.title,
        altimeter: &row.altimeter,
        altimeter_evidence: &row.altimeter_evidence,
        era5_weather: false,
        reference_log: &row.reference_log,
        reference_log_sha256: &row.reference_log_sha256,
        reference_rule: &row.reference_rule,
        apogee_m: row.predicted_apogee_m,
        height_apogee_m: row.predicted_height_apogee_m,
        all_inputs_traced: row.all_inputs_traced,
        untraced: &row.untraced,
        hpr_max_mach: row.hpr_max_mach,
        catalog_curve: None,
        openrocket_stored_apogee_m: Some(row.openrocket_stored_apogee_m),
    });
    examples.chain(design_files).collect()
}

/// Reads each predicted flight's log by its rule and puts it beside the prediction `flown` gives
/// and the one first merged.
///
/// `committed` is the predictions report as committed: each log must be the file whose SHA-256
/// it holds, the file the rule was merged for.
///
/// # Errors
///
/// [`RealFlightError`] for a log that can't be read, isn't the committed predictions' file or
/// doesn't read by its rule, and for a prediction that differs from its merged number without a
/// revision that says why.
pub fn compare(
    root: &Path,
    flown: &PredictionsReport,
    committed: &PredictionsReport,
) -> Result<Vec<ComparedPrediction>, RealFlightError> {
    let input = |flight: &str, what: String| RealFlightError::Input {
        flight: flight.to_owned(),
        what,
    };
    let committed = predicted(committed);
    predicted(flown)
        .into_iter()
        .map(|flight| {
            let id = flight.id;
            let rule = RULES
                .iter()
                .find(|(rule_id, _)| *rule_id == id)
                .map(|(_, rule)| *rule)
                .ok_or_else(|| input(id, "it has no apogee rule".to_owned()))?;
            if !rule.is_the_rule_in(flight.reference_rule) {
                return Err(input(
                    id,
                    format!(
                        "its rule as code, {rule:?}, is not the rule merged with it: {}",
                        flight.reference_rule
                    ),
                ));
            }
            let bytes = read_bytes(root, flight.reference_log, "reading the flight's log")?;
            let sha256 = sha256_hex(&bytes);
            let pinned = committed
                .iter()
                .find(|row| row.id == id)
                .ok_or_else(|| input(id, "the committed predictions don't hold it".to_owned()))?;
            if sha256 != pinned.reference_log_sha256 || sha256 != flight.reference_log_sha256 {
                return Err(input(
                    id,
                    format!(
                        "{} is not the log its prediction was merged for (SHA-256 {sha256}, \
                         committed {})",
                        flight.reference_log, pinned.reference_log_sha256
                    ),
                ));
            }
            let text = String::from_utf8(bytes)
                .map_err(|_| input(id, format!("{} is not UTF-8", flight.reference_log)))?;
            let log = rule
                .read(&text)
                .map_err(|what| input(id, format!("{}: {what}", flight.reference_log)))?;
            let row =
                compared(&MERGED, &REVISIONS, &flight, log).map_err(|what| input(id, what))?;
            row.check(&MERGED, &REVISIONS)
                .map_err(|what| input(id, what))?;
            Ok(row)
        })
        .collect()
}

/// The row of `flight` against `log`: the merged numbers from `merged`, and the reason from
/// the flight's entry in `revisions` where today's prediction is not the merged one.
fn compared(
    merged: &[MergedPrediction],
    revisions: &[Revision],
    flight: &Predicted<'_>,
    log: LogApogee,
) -> Result<ComparedPrediction, String> {
    let first = merged
        .iter()
        .find(|merged| merged.id == flight.id)
        .ok_or_else(|| "it has no merged prediction".to_owned())?;
    let shown = |what: String, apogee_m: f64| ShownOnly {
        what,
        apogee_m,
        apogee_error_percent: percent_of(apogee_m, log.apogee_m),
    };
    let catalog_apogee_m = flight.catalog_curve.as_ref().map(|(_, apogee_m)| *apogee_m);
    let revision = held_to(
        merged,
        revisions,
        flight.id,
        flight.apogee_m,
        flight.height_apogee_m,
        catalog_apogee_m,
    )?;
    Ok(ComparedPrediction {
        id: flight.id.to_owned(),
        title: flight.title.to_owned(),
        altimeter: flight.altimeter.to_owned(),
        altimeter_evidence: flight.altimeter_evidence.to_owned(),
        era5_weather: flight.era5_weather,
        reference_log: flight.reference_log.to_owned(),
        reference_log_sha256: flight.reference_log_sha256.to_owned(),
        reference_rule: flight.reference_rule.to_owned(),
        log_rows: log.rows,
        log_apogee_m: log.apogee_m,
        log_highest_sample_m: log.highest_sample_m,
        merged_apogee_m: first.apogee_m,
        merged_height_apogee_m: first.height_apogee_m,
        merged_apogee_error_percent: percent_of(first.apogee_m, log.apogee_m),
        hpr_apogee_m: flight.apogee_m,
        hpr_height_apogee_m: flight.height_apogee_m,
        apogee_error_percent: percent_of(flight.apogee_m, log.apogee_m),
        revised_because: revision.map(|revision| revision.because.to_owned()),
        all_inputs_traced: flight.all_inputs_traced,
        untraced: flight.untraced.to_vec(),
        hpr_max_mach: flight.hpr_max_mach,
        catalog_curve: flight
            .catalog_curve
            .as_ref()
            .map(|(what, apogee_m)| shown(what.clone(), *apogee_m)),
        openrocket_stored: flight.openrocket_stored_apogee_m.map(|apogee_m| {
            shown(
                "OpenRocket 24.12's simulation stored in the design file".to_owned(),
                apogee_m,
            )
        }),
    })
}

impl ComparedPrediction {
    /// Whether the row holds together without the files it was read from, as CI checks it: its
    /// merged numbers are `merged`'s, today's prediction is the merged one or the one a revision
    /// in `revisions` gives, the reason kept is that revision's, each percentage is its meters',
    /// and the traced tag is its list's.
    ///
    /// # Errors
    ///
    /// The first thing that doesn't hold, in words.
    pub fn check(&self, merged: &[MergedPrediction], revisions: &[Revision]) -> Result<(), String> {
        let first = merged
            .iter()
            .find(|merged| merged.id == self.id)
            .ok_or_else(|| "it has no merged prediction".to_owned())?;
        if (self.merged_apogee_m, self.merged_height_apogee_m)
            != (first.apogee_m, first.height_apogee_m)
        {
            return Err(format!(
                "its merged prediction is kept as {} m ({} m as a height), and the one merged is \
                 {} m ({} m)",
                self.merged_apogee_m,
                self.merged_height_apogee_m,
                first.apogee_m,
                first.height_apogee_m
            ));
        }
        let revision = held_to(
            merged,
            revisions,
            &self.id,
            self.hpr_apogee_m,
            self.hpr_height_apogee_m,
            self.catalog_curve.as_ref().map(|catalog| catalog.apogee_m),
        )?;
        if self.revised_because.as_deref() != revision.map(|revision| revision.because) {
            return Err("its revision's reason is not the one in `REVISIONS`".to_owned());
        }
        if !(self.log_apogee_m.is_finite() && self.log_apogee_m > 0.0) {
            return Err(format!("its log's apogee is {}", self.log_apogee_m));
        }
        if self.reference_log_sha256.len() != 64 {
            return Err("its log has no SHA-256".to_owned());
        }
        if self.all_inputs_traced != self.untraced.is_empty() {
            return Err("its traced tag is not its list's".to_owned());
        }
        if !(self.hpr_max_mach.is_finite() && self.hpr_max_mach > 0.0) {
            return Err(format!("its largest Mach number is {}", self.hpr_max_mach));
        }
        let same = crate::report::same_but_for_platform_rounding;
        let shown = [&self.catalog_curve, &self.openrocket_stored];
        let percentages = [
            (
                "apogee error",
                self.apogee_error_percent,
                percent_of(self.hpr_apogee_m, self.log_apogee_m),
            ),
            (
                "merged prediction's error",
                self.merged_apogee_error_percent,
                percent_of(self.merged_apogee_m, self.log_apogee_m),
            ),
        ]
        .into_iter()
        .chain(shown.into_iter().flatten().map(|shown| {
            (
                "error of a number shown beside it",
                shown.apogee_error_percent,
                percent_of(shown.apogee_m, self.log_apogee_m),
            )
        }));
        for (what, kept, derived) in percentages {
            if !same(kept, derived) {
                return Err(format!("its {what} is {kept}, its meters give {derived}"));
            }
        }
        Ok(())
    }
}

/// Whether the ids of `rows` are the predicted flights', in [`MERGED`]'s order, and each row is
/// the flight the code lists: its title, its log and its rule.
///
/// # Errors
///
/// The first thing that doesn't hold, in words.
pub fn check_listed(rows: &[ComparedPrediction]) -> Result<(), String> {
    if !rows
        .iter()
        .map(|row| row.id.as_str())
        .eq(MERGED.iter().map(|merged| merged.id))
    {
        return Err(format!(
            "the predicted flights compared are not the {} merged, in their order",
            MERGED.len()
        ));
    }
    let listed = NEW_FLIGHTS
        .iter()
        .map(|flight| (flight.id, flight.title, flight.reference.rule, true))
        .chain(
            DESIGN_FILE_FLIGHTS
                .iter()
                .map(|flight| (flight.id, flight.title, flight.reference.rule, false)),
        );
    for (row, (id, title, rule, era5_weather)) in rows.iter().zip(listed) {
        if (
            row.id.as_str(),
            row.title.as_str(),
            row.reference_rule.as_str(),
        ) != (id, title, rule)
            || row.era5_weather != era5_weather
        {
            return Err(format!(
                "{}: its title, rule or air is not the code's for {id}",
                row.id
            ));
        }
        let read_by = RULES.iter().find(|(rule_id, _)| *rule_id == id);
        if !read_by.is_some_and(|(_, read_by)| read_by.is_the_rule_in(rule)) {
            return Err(format!("{id}: its rule as code is not its rule in words"));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A comma-separated log reads its named column whole: the highest reading, every row
    /// counted, whichever place the column has.
    #[test]
    fn the_highest_reading_of_the_named_column_is_the_apogee() {
        let log = ApogeeRule::HighestOfColumn("alt(m)")
            .read("t(s),alt(ft),alt(m)\n0,0,0.0\n1,328.1,100.0\n2,656.2,200.5\n3,600,182.9\n\n")
            .expect("it reads");
        assert_eq!(
            log,
            LogApogee {
                apogee_m: 200.5,
                rows: 4,
                highest_sample_m: None
            }
        );
        // The column first, as Erebus 11's log has it, and a byte-order mark before the header.
        let first = ApogeeRule::HighestOfColumn("alt")
            .read("\u{feff}alt,t\n0,0\n31.5,1\n12,2\n")
            .expect("it reads");
        assert_eq!((first.apogee_m, first.rows), (31.5, 3));
    }

    /// The rule reads the log whole, so nothing is skipped: a row that isn't a number, a missing
    /// column, a column named twice and an empty log each fail, with the place.
    #[test]
    fn a_log_that_does_not_read_whole_is_refused() {
        let read = |text: &str| {
            ApogeeRule::HighestOfColumn("alt")
                .read(text)
                .expect_err("it is refused")
        };
        assert!(read("t,alt\n0,1\n1,n/a\n").contains("line 3"), "a word");
        assert!(read("t,alt\n0,1\n1\n").contains("line 3"), "a short row");
        assert!(read("t,alt\n0,NaN\n").contains("line 2"), "not finite");
        assert!(read("t,alt\n0,inf\n").contains("line 2"), "not finite");
        assert!(read("t,height\n0,1\n").contains("no `alt` column"));
        assert!(read("alt,t,alt\n0,1,2\n").contains("twice"));
        assert!(read("t,alt\n").contains("no rows"));
        assert!(read("").contains("empty"));
    }

    /// A PerfectFlite log's apogee is its header's, in feet at 0.3048 m, never its highest
    /// sample: here a spike of 3,000 ft among samples that peak at 2,765 ft.
    #[test]
    fn a_perfectflite_logs_apogee_is_its_headers_not_its_highest_sample() {
        let text = "PerfectFlite SLCF\r\nApogee: 2765' AGL\r\nGround Elevation: 12' MSL\r\n\
                    NumSamps: 4\r\n0.00, 0, 0\r\n0.05, 2765, 10\r\n0.10, 3000, 10\r\n\
                    0.15, 2700, -10\r\n";
        let log = ApogeeRule::PerfectFliteHeader.read(text).expect("it reads");
        assert!((log.apogee_m - 2765.0 * 0.3048).abs() < 1e-9, "{log:?}");
        assert_eq!(log.rows, 4);
        let highest = log.highest_sample_m.expect("it has samples");
        assert!((highest - 3000.0 * 0.3048).abs() < 1e-9, "{highest}");
        // A header that states a power loss, not a height, gives no apogee to compare.
        let lost = ApogeeRule::PerfectFliteHeader
            .read("PerfectFlite SLCF\nApogee: PWRLOSS\n0, 0, 0\n")
            .expect_err("no apogee");
        assert!(lost.contains("states no apogee"), "{lost}");
    }

    /// Each rule as code is the rule merged in words with its prediction, and no predicted
    /// flight is without one.
    #[test]
    fn each_rule_is_the_one_merged_in_words() {
        let words = NEW_FLIGHTS
            .iter()
            .map(|flight| (flight.id, flight.reference.rule))
            .chain(
                DESIGN_FILE_FLIGHTS
                    .iter()
                    .map(|flight| (flight.id, flight.reference.rule)),
            );
        assert!(
            RULES
                .iter()
                .map(|(id, _)| *id)
                .eq(MERGED.iter().map(|merged| merged.id))
        );
        for ((id, rule), (flight, words)) in RULES.iter().zip(words) {
            assert_eq!(*id, flight);
            assert!(rule.is_the_rule_in(words), "{id}: {words}");
        }
        // The other kind of rule is not the one in a flight's words, so a swap fails.
        assert!(!ApogeeRule::PerfectFliteHeader.is_the_rule_in(NEW_FLIGHTS[0].reference.rule));
        assert!(
            !ApogeeRule::HighestOfColumn("alt").is_the_rule_in(NEW_FLIGHTS[1].reference.rule),
            "Andromeda's column is `alt(m)`"
        );
        assert!(
            !ApogeeRule::HighestOfColumn("alt")
                .is_the_rule_in(DESIGN_FILE_FLIGHTS[0].reference.rule)
        );
    }

    /// An invented row: a flight merged at 1,000.0 m against a log at 1,100 m.
    fn row(hpr_apogee_m: f64, revised_because: Option<&str>) -> ComparedPrediction {
        ComparedPrediction {
            id: "invented".to_owned(),
            title: "An invented flight".to_owned(),
            altimeter: "barometric".to_owned(),
            altimeter_evidence: "invented".to_owned(),
            era5_weather: true,
            reference_log: "refs/invented.csv".to_owned(),
            reference_log_sha256: "0".repeat(64),
            reference_rule: "invented".to_owned(),
            log_rows: 3,
            log_apogee_m: 1100.0,
            log_highest_sample_m: None,
            merged_apogee_m: 1000.0,
            merged_height_apogee_m: 1040.0,
            merged_apogee_error_percent: percent_of(1000.0, 1100.0),
            hpr_apogee_m,
            hpr_height_apogee_m: hpr_apogee_m + 40.0,
            apogee_error_percent: percent_of(hpr_apogee_m, 1100.0),
            revised_because: revised_because.map(str::to_owned),
            all_inputs_traced: true,
            untraced: Vec::new(),
            hpr_max_mach: 0.5,
            catalog_curve: None,
            openrocket_stored: None,
        }
    }

    const INVENTED: [MergedPrediction; 1] = [MergedPrediction {
        id: "invented",
        apogee_m: 1000.0,
        height_apogee_m: 1040.0,
        catalog_apogee_m: None,
    }];

    fn revision(apogee_m: f64, because: &'static str) -> Revision {
        Revision {
            id: "invented",
            apogee_m,
            height_apogee_m: apogee_m + 40.0,
            catalog_apogee_m: None,
            because,
        }
    }

    /// A prediction that differs from its merged number fails unless a revision keeps both
    /// numbers and says why ([ADR-328][adr-328] §1b); a revision without a reason, one that
    /// states another number, one that changes nothing, and two for one flight fail too.
    ///
    /// [adr-328]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0328-the-2026-11-08-review-twelve-public-flights-predictions-first.md
    #[test]
    fn a_changed_prediction_needs_a_revision_that_keeps_both_numbers() {
        // As merged, to the printed digit: no revision needed, none allowed to be claimed.
        row(1000.04, None).check(&INVENTED, &[]).expect("as merged");
        let claimed = row(1000.04, Some("a reason"))
            .check(&INVENTED, &[])
            .expect_err("no revision is listed");
        assert!(claimed.contains("reason is not the one"), "{claimed}");

        // Changed, with no revision: refused, and the message names the list.
        let changed = row(1050.0, None)
            .check(&INVENTED, &[])
            .expect_err("a changed prediction");
        assert!(
            changed.contains("1050") && changed.contains("1000") && changed.contains("REVISIONS"),
            "{changed}"
        );

        // Changed, with its revision and the reason: passes, and the merged number stays.
        let revised = row(1050.0, Some("the fins' sweep was read from the wrong edge"));
        let listed = [revision(
            1050.0,
            "the fins' sweep was read from the wrong edge",
        )];
        revised.check(&INVENTED, &listed).expect("revised");
        assert_eq!(revised.merged_apogee_m, 1000.0);
        assert!(revised.merged_apogee_error_percent < revised.apogee_error_percent);

        // The row must carry the revision's reason, not another and not none.
        for reason in [None, Some("another reason")] {
            let error = row(1050.0, reason)
                .check(&INVENTED, &listed)
                .expect_err("the reason differs");
            assert!(error.contains("reason is not the one"), "{error}");
        }
        // A revision with no reason.
        let silent = row(1050.0, Some(" "))
            .check(&INVENTED, &[revision(1050.0, " ")])
            .expect_err("no reason");
        assert!(silent.contains("gives no reason"), "{silent}");
        // A revision that states a number other than the one flown.
        let stale = row(1050.0, Some("a reason"))
            .check(&INVENTED, &[revision(1060.0, "a reason")])
            .expect_err("another number");
        assert!(stale.contains("revision says 1060"), "{stale}");
        // A revision that gives the merged numbers back.
        let empty = row(1000.0, Some("a reason"))
            .check(&INVENTED, &[revision(1000.0, "a reason")])
            .expect_err("nothing changed");
        assert!(empty.contains("changes nothing"), "{empty}");
        // Two revisions of one flight.
        let twice = row(1050.0, Some("a reason"))
            .check(
                &INVENTED,
                &[revision(1050.0, "a reason"), revision(1050.0, "a reason")],
            )
            .expect_err("revised twice");
        assert!(twice.contains("revised twice"), "{twice}");
    }

    /// The merged number a row keeps is the code's, and its percentages are its meters'.
    #[test]
    fn a_rows_merged_number_and_percentages_are_held() {
        let mut edited = row(1000.0, None);
        edited.merged_apogee_m = 1090.0;
        edited.merged_apogee_error_percent = percent_of(1090.0, 1100.0);
        let error = edited.check(&INVENTED, &[]).expect_err("an edited merge");
        assert!(error.contains("the one merged is 1000"), "{error}");

        let mut flattered = row(1000.0, None);
        flattered.apogee_error_percent = -1.0;
        let error = flattered.check(&INVENTED, &[]).expect_err("a wrong error");
        assert!(error.contains("its meters give"), "{error}");

        let mut shown = row(1000.0, None);
        shown.openrocket_stored = Some(ShownOnly {
            what: "invented".to_owned(),
            apogee_m: 990.0,
            apogee_error_percent: 0.0,
        });
        let error = shown
            .check(&INVENTED, &[])
            .expect_err("a wrong shown error");
        assert!(error.contains("shown beside it"), "{error}");

        let mut tagged = row(1000.0, None);
        tagged.untraced = vec!["the mass".to_owned()];
        let error = tagged.check(&INVENTED, &[]).expect_err("a wrong tag");
        assert!(error.contains("traced tag"), "{error}");
    }

    /// The error's sign: a prediction below its log is negative.
    #[test]
    fn an_error_is_the_prediction_less_the_log_over_the_log() {
        assert_eq!(percent_of(900.0, 1000.0), -10.0);
        assert_eq!(percent_of(1100.0, 1000.0), 10.0);
    }
}
