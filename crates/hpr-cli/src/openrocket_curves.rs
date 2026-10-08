//! Which ThrustCurve.org data file holds the curve OpenRocket flies for a `.ork` motor's digest
//! ([ADR-161](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0161-openrocket-curves-by-digest.md)):
//! `hpr sim` takes that file first when it fetches the motor, where ThrustCurve holds several
//! files of it with different curves.
//!
//! The table, `data/openrocket-curves.json`, is written by `cargo xtask ork-curves`. It covers
//! the motors of OpenRocket 24.12's examples, each matched by comparing OpenRocket's curve with
//! every file's; it holds ids alone, no curve. A digest it lacks is fetched by ADR-154's rule.

use std::sync::OnceLock;

use serde::Deserialize;

/// The table as committed.
const TABLE: &str = include_str!("../data/openrocket-curves.json");

#[derive(Deserialize)]
struct Table {
    files: Vec<Entry>,
}

#[derive(Deserialize)]
struct Entry {
    digest: String,
    simfile_id: String,
}

/// The ThrustCurve id of the file holding OpenRocket's curve for `digest`, if the table has it.
pub(crate) fn file_for(digest: &str) -> Option<String> {
    static FILES: OnceLock<Vec<Entry>> = OnceLock::new();
    // The committed table parses (the test below holds it to), so an empty one is never taken.
    let files = FILES.get_or_init(|| {
        serde_json::from_str::<Table>(TABLE).map_or_else(|_| Vec::new(), |table| table.files)
    });
    files
        .iter()
        .find(|entry| entry.digest.eq_ignore_ascii_case(digest))
        .map(|entry| entry.simfile_id.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The table parses, names each digest once, and gives the three-stage example's A8 the file
    /// whose curve is OpenRocket's (the other certification file of the A8 burns 0.2 s shorter).
    #[test]
    fn the_table_reads_and_names_openrockets_a8() {
        let table: Table = serde_json::from_str(TABLE).unwrap();
        let mut digests: Vec<String> = table
            .files
            .iter()
            .map(|entry| entry.digest.to_ascii_lowercase())
            .collect();
        let count = digests.len();
        digests.sort();
        digests.dedup();
        assert_eq!(digests.len(), count);
        assert!(count >= 28, "{count}");
        assert_eq!(
            file_for("22aec01287ea1e3b8c6f66b26fe5fea6").as_deref(),
            Some("5f4294d20002e900000004e3")
        );
        assert_eq!(
            file_for("F3A785E1523935CAF239C7366CF81FEE").as_deref(),
            Some("5f4294d20002e90000000897")
        );
        assert_eq!(file_for("00000000000000000000000000000000"), None);
    }
}
