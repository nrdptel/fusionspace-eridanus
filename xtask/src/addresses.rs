//! The repository's and the site's addresses after the rename (ADR-198 §5–6, ADR-199 §5; M10.1d8),
//! checked by `cargo test -p xtask`. The repository moved to `nrdptel/fusionspace-eridanus` and the
//! site to `https://hpr.fusionspace.co/`. GitHub redirects the old repository's URLs but not its
//! old project site, and crates.io and PyPI keep the URLs a release is published with, so no tracked
//! file outside `docs/decisions/` (which keeps its history) names the old repository or the old
//! site:
//!
//! - the old repository, `nrdptel/` then `hpr-sim`, except as the start of the private fixtures
//!   repository's name, `hpr-sim-fixtures`, which kept its name;
//! - the old site, `nrdptel.github.io/` then `hpr-sim`.
//!
//! Either is matched in any case, since GitHub reads its names without case. A line that talks about
//! the old name as the old name is exempt only as listed in [`EXEMPT`], each entry matching a whole
//! line's text, and an entry that matches no line is itself a problem, so the list can't outlive
//! what it excuses. This file spells the old names in two pieces so that it names neither.

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::process::Command;

    /// The folder whose files keep their history, relative to the workspace root.
    const HISTORY: &str = "docs/decisions/";
    /// The owner's account, the first half of both old names.
    const OWNER: &str = "nrdptel";
    /// The old repository's name, and the old site's path.
    const OLD: &str = "hpr-sim";
    /// What may follow [`OLD`] after the owner's account: the private fixtures repository, which
    /// kept its name (ADR-151).
    const FIXTURES: &str = "-fixtures";

    /// A line allowed to name an old address: the files it may be in, its text with the old
    /// repository written `{repo}` and the old site `{site}`, and why it stays.
    struct Exempt {
        files: &'static [&'static str],
        line: &'static str,
        why: &'static str,
    }

    /// The lines that talk about the old names as the old names.
    const EXEMPT: [Exempt; 2] = [
        Exempt {
            // The roadmap holds the entry until it is done, then its archive does.
            files: &["docs/ROADMAP.md", "docs/roadmap-done.md"],
            line: "outside `docs/decisions/`, no tracked file names `{repo}` (but `-fixtures`) or",
            why: "M10.1d8's done-when names the strings it removes",
        },
        Exempt {
            files: &["docs/ROADMAP.md", "docs/roadmap-done.md"],
            line: "`{site}`, nor `book.toml`'s `site-url` or xtask's `SITE_PATH` `/hpr-sim/`;",
            why: "M10.1d8's done-when names the strings it removes",
        },
    ];

    /// The old repository's name, `owner/name`.
    fn old_repository() -> String {
        format!("{OWNER}/{OLD}")
    }

    /// The old site's address, without its scheme.
    fn old_site() -> String {
        format!("{OWNER}.github.io/{OLD}")
    }

    /// An exemption's text with the old names written in.
    fn exempt_text(exempt: &Exempt) -> String {
        exempt
            .line
            .replace("{repo}", &old_repository())
            .replace("{site}", &old_site())
    }

    /// Whether `line` names the old repository or the old site, in any case. The old repository's
    /// name followed by [`FIXTURES`] is the fixtures repository's, and doesn't count.
    fn names_an_old_address(line: &str) -> bool {
        let lower = line.to_ascii_lowercase();
        let repository = old_repository();
        lower.contains(&old_site())
            || lower
                .match_indices(&repository)
                .any(|(at, _)| !lower[at + repository.len()..].starts_with(FIXTURES))
    }

    /// The problems in the file `path` with the text `text`: each line naming an old address that
    /// no exemption covers, as `path:line: text`. `used` counts, for each of [`EXEMPT`], the lines
    /// it covered.
    fn problems_in(path: &str, text: &str, used: &mut [usize]) -> Vec<String> {
        if path.starts_with(HISTORY) {
            return Vec::new();
        }
        let mut problems = Vec::new();
        for (number, line) in text.lines().enumerate() {
            if !names_an_old_address(line) {
                continue;
            }
            let covered = EXEMPT.iter().position(|exempt| {
                // The line is covered only if, without the exempt text, it names no old address.
                let text = exempt_text(exempt);
                exempt.files.contains(&path)
                    && line.contains(&text)
                    && !names_an_old_address(&line.replacen(&text, "", 1))
            });
            match covered {
                Some(index) => used[index] += 1,
                None => problems.push(format!("{path}:{}: {}", number + 1, line.trim())),
            }
        }
        problems
    }

    /// The problems in the tracked files at `root`, and each exemption that covered no line.
    fn problems(root: &Path) -> Vec<String> {
        let output = Command::new("git")
            .arg("-C")
            .arg(root)
            .args(["ls-files", "-z"])
            .output()
            .unwrap();
        assert!(output.status.success(), "git ls-files failed");
        let listing = String::from_utf8(output.stdout).unwrap();
        let mut used = [0; EXEMPT.len()];
        let mut found = Vec::new();
        for path in listing.split('\0').filter(|path| !path.is_empty()) {
            // A file deleted in the working tree but not yet from the index names nothing.
            let Ok(bytes) = std::fs::read(root.join(path)) else {
                continue;
            };
            found.extend(problems_in(
                path,
                &String::from_utf8_lossy(&bytes),
                &mut used,
            ));
        }
        for (exempt, count) in EXEMPT.iter().zip(used) {
            if count == 0 {
                found.push(format!(
                    "no line of {} is `{}`: drop the exemption ({})",
                    exempt.files.join(" or "),
                    exempt_text(exempt),
                    exempt.why
                ));
            }
        }
        found
    }

    #[test]
    fn no_tracked_file_names_the_old_repository_or_site() {
        let root = crate::designs::root().unwrap();
        assert_eq!(problems(&root), Vec::<String>::new());
    }

    /// Each form an old address takes is found, the fixtures repository and the new names are not,
    /// and an exemption covers only its own line in its own files.
    #[test]
    fn an_old_address_is_found_in_any_form() {
        let repository = old_repository();
        let site = old_site();
        let named = [
            format!("https://github.com/{repository}/issues/12"),
            format!("[site](https://{site}/start-here.html)"),
            format!("HTTPS://{}/x", site.to_ascii_uppercase()),
            format!("gh pr list -R {repository}"),
            format!("{repository}.git"),
            format!("{repository}-fixtures and {repository}"),
        ];
        for line in &named {
            assert!(names_an_old_address(line), "{line}");
        }
        let clean = [
            format!("{repository}-fixtures"),
            format!("https://github.com/{OWNER}/fusionspace-eridanus/issues/12"),
            "https://hpr.fusionspace.co/start-here.html".to_owned(),
            format!("crates/{OLD}/src/lib.rs and refs/{OLD}-fixtures"),
        ];
        for line in &clean {
            assert!(!names_an_old_address(line), "{line}");
        }

        let mut used = [0; EXEMPT.len()];
        let planted = format!("see https://{site}/accuracy.html\n");
        assert_eq!(
            problems_in("README.md", &planted, &mut used),
            [format!("README.md:1: see https://{site}/accuracy.html")]
        );
        assert_eq!(
            problems_in("docs/decisions/0198-x.md", &planted, &mut used),
            Vec::<String>::new()
        );
        let done_when = exempt_text(&EXEMPT[0]);
        assert_eq!(
            problems_in(
                "docs/roadmap-done.md",
                &format!("  {done_when}\n"),
                &mut used
            ),
            Vec::<String>::new()
        );
        assert_eq!(used, [1, 0]);
        // An exempt line that gains another old address is not covered.
        let grown = format!("  {done_when} see https://{site}/\n");
        assert_eq!(
            problems_in("docs/roadmap-done.md", &grown, &mut used).len(),
            1
        );
        // The same line elsewhere is not exempt.
        assert_eq!(
            problems_in("docs/start-here.md", &done_when, &mut used),
            [format!("docs/start-here.md:1: {done_when}")]
        );
    }
}
