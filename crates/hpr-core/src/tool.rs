//! The program every file and document FusionSpace HPR writes names, as a drawing's title block names
//! the tool that made it: its name, its version and its designation in the FusionSpace product
//! system ([ADR-164][adr-164], which adopts that system; its `data.md`: "Every export carries the
//! designation and version of the tool that made it").
//!
//! [adr-164]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0164-the-fusionspace-product-system.md
//!
//! The writers put these three in whatever place their format keeps for it: a `.eng` file's
//! comment, an XML comment or attribute, a JSON object, Parquet's key-value metadata.

/// The program's name: `FusionSpace HPR`, the suite's public name ([ADR-199][adr-199] §4).
///
/// [adr-199]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0199-the-2026-10-07-fusionspace-hpr.md
pub const NAME: &str = "FusionSpace HPR";

/// The name earlier builds wrote, which readers still recognize: `hpr-sim`, written by every
/// build until [ADR-199, the public name][adr-199], renamed the program, with either designation
/// (builds after [ADR-198, project Eridanus][adr-198], wrote the new designation under the old
/// name). Nothing writes it.
///
/// [adr-198]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0198-the-2026-10-07-project-eridanus.md
/// [adr-199]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0199-the-2026-10-07-fusionspace-hpr.md
pub const EARLIER_NAME: &str = "hpr-sim";

/// Its version, the workspace's: `0.1.0` until the first release changes it.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Its designation in the FusionSpace product system, `<code> · <tag> · <kind> <number>` in the
/// system's grammar: the simulator's internal name, Achernar (`FS-ACHERNAR`, a star of project
/// Eridanus), then software tool 1 ([ADR-198][adr-198] §4).
///
/// [adr-198]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0198-the-2026-10-07-project-eridanus.md
pub const DESIGNATION: &str = "FS-ACHERNAR · SW · TOOL 001";

/// The designations earlier builds wrote, which readers still recognize: `FS · SW · TOOL 005`,
/// written by every build from the command line's restyle until [ADR-198][adr-198] retired it.
/// Nothing writes these.
///
/// [adr-198]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0198-the-2026-10-07-project-eridanus.md
pub const EARLIER_DESIGNATIONS: [&str; 1] = ["FS · SW · TOOL 005"];

/// The three on one line, `FusionSpace HPR 0.1.0 · FS-ACHERNAR · SW · TOOL 001`, for a format that
/// keeps one line of text for it: the external name first, as the system's naming rule asks.
pub fn stamp() -> String {
    format!("{NAME} {VERSION} · {DESIGNATION}")
}

/// Whether `text` is a stamp this program wrote, at this version or any other: a name, a space,
/// and a version (starting with a digit, as every Cargo version does), then ` · ` and a
/// designation at the end. Surrounding white space is ignored. The pairs recognized are the ones
/// some build wrote: [`NAME`] with [`DESIGNATION`], and [`EARLIER_NAME`] with [`DESIGNATION`] or
/// an [earlier designation](EARLIER_DESIGNATIONS).
#[must_use]
pub fn is_stamp(text: &str) -> bool {
    let text = text.trim();
    let written: [(&str, &[&str]); 2] = [
        (NAME, &[DESIGNATION]),
        (EARLIER_NAME, &[DESIGNATION, EARLIER_DESIGNATIONS[0]]),
    ];
    written.iter().any(|(name, designations)| {
        let Some(rest) = text
            .strip_prefix(name)
            .and_then(|rest| rest.strip_prefix(' '))
        else {
            return false;
        };
        designations.iter().any(|designation| {
            rest.strip_suffix(designation)
                .and_then(|version| version.strip_suffix(" · "))
                .is_some_and(|version| {
                    version.starts_with(|c: char| c.is_ascii_digit())
                        && !version.contains(char::is_whitespace)
                })
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The line names all three, in the order a title block reads, the external name first.
    #[test]
    fn the_stamp_reads_name_version_designation() {
        assert_eq!(
            stamp(),
            format!(
                "FusionSpace HPR {} · FS-ACHERNAR · SW · TOOL 001",
                env!("CARGO_PKG_VERSION")
            )
        );
        assert!(is_stamp(&stamp()));
        assert!(!stamp().contains("hpr-sim"));
    }

    /// ADR-199 §4: a stamp under the old name, `hpr-sim`, with either designation, is still
    /// recognized, as is the new name with the current designation; the new name with the
    /// retired designation was never written, so it isn't one.
    #[test]
    fn stamps_under_the_earlier_name_are_recognized() {
        assert!(is_stamp("hpr-sim 0.1.0 · FS-ACHERNAR · SW · TOOL 001"));
        assert!(is_stamp("hpr-sim 0.1.0 · FS · SW · TOOL 005"));
        assert!(is_stamp(
            "FusionSpace HPR 0.1.0 · FS-ACHERNAR · SW · TOOL 001"
        ));
        assert!(is_stamp(
            " FusionSpace HPR 2.0.0-rc.1 · FS-ACHERNAR · SW · TOOL 001 "
        ));
        assert_ne!(NAME, EARLIER_NAME);
        for text in [
            "FusionSpace HPR 0.1.0 · FS · SW · TOOL 005",
            "FusionSpace HPR · FS-ACHERNAR · SW · TOOL 001",
            "FusionSpace  HPR 0.1.0 · FS-ACHERNAR · SW · TOOL 001",
            "FusionSpace HPR0.1.0 · FS-ACHERNAR · SW · TOOL 001",
            "fusionspace-hpr 0.1.0 · FS-ACHERNAR · SW · TOOL 001",
            "FusionSpace 0.1.0 · FS-ACHERNAR · SW · TOOL 001",
            "HPR 0.1.0 · FS-ACHERNAR · SW · TOOL 001",
            "FusionSpace HPR x · FS-ACHERNAR · SW · TOOL 001",
            "FusionSpace HPR 0.1.0 · FS-ACHERNAR · SW · TOOL 001, edited",
        ] {
            assert!(!is_stamp(text), "{text}");
        }
    }

    /// A stamp with this designation or the retired one, from any version, is recognized; the
    /// retired designation is never written; and a text that only looks like a stamp isn't one.
    #[test]
    fn stamps_with_the_earlier_designation_are_recognized() {
        assert!(is_stamp("hpr-sim 0.1.0 · FS · SW · TOOL 005"));
        assert!(is_stamp("  hpr-sim 0.0.9 · FS · SW · TOOL 005 "));
        assert!(is_stamp("hpr-sim 9.9 · FS-ACHERNAR · SW · TOOL 001"));
        assert!(!stamp().contains("TOOL 005"));
        assert!(!EARLIER_DESIGNATIONS.contains(&DESIGNATION));
        for text in [
            "FS · SW · TOOL 005",
            "FS-ACHERNAR · SW · TOOL 001",
            "hpr-sim · FS · SW · TOOL 005",
            "hpr-sim  · FS-ACHERNAR · SW · TOOL 001",
            "hpr-simulator 0.1.0 · FS · SW · TOOL 005",
            "hpr-sim 0.1.0 FS · SW · TOOL 005",
            "hpr-sim 2 · 3 s burn · FS · SW · TOOL 005",
            "hpr-sim 0.1.0 · FS · SW · TOOL 006",
            "hpr-sim 0.1.0 · FS · SW · TOOL 005, edited",
            "a note by hpr-sim 0.1.0 · FS · SW · TOOL 005",
            "hpr-sim x · FS · SW · TOOL 005",
            "hpr-sim says: · FS-ACHERNAR · SW · TOOL 001",
        ] {
            assert!(!is_stamp(text), "{text}");
        }
    }
}
