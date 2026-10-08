//! The Python package's names after the rename (ADR-199 §2; M10.1d9), checked by `cargo test -p
//! xtask`. The package is `fusionspace-hpr` on PyPI and imports as `fusionspace.hpr`, so no tracked
//! file outside `docs/decisions/` (which keeps its history) tells anyone to install or import the
//! old names:
//!
//! - `pip install` of the old distribution, `hpr-sim`, in any spelling pip takes for it (PEP 503
//!   folds case and runs of `-`, `_` and `.`, so `HPR_Sim` is the same name), with or without
//!   options, quotes, extras or a version;
//! - an import of the old top-level package, `hpr`, written as Python writes one: the package or
//!   a module under it after `import` (alone or in a list of several, with or without `as`), or
//!   after `from`, anywhere on a line (inside a `python -c "..."` too). An import of
//!   `fusionspace.hpr` (`import fusionspace.hpr as hpr`, `from fusionspace import hpr`) is the new
//!   name, and `hpr_core` or `hprz` are other names.
//!
//! A line that quotes an old name as the old name is exempt only as listed in [`EXEMPT`], and an
//! entry that matches no line is itself a problem, so the list can't outlive what it excuses. This
//! file spells the old names in pieces so that it names neither.

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::process::Command;

    /// The folder whose files keep their history, relative to the workspace root.
    const HISTORY: &str = "docs/decisions/";
    /// The old distribution's name, in two pieces.
    const OLD_DISTRIBUTION: [&str; 2] = ["hpr", "-sim"];
    /// The old top-level package's name.
    const OLD_PACKAGE: &str = "hpr";

    /// A line allowed to quote an old name: the files it may be in, text it contains, and why.
    struct Exempt {
        files: &'static [&'static str],
        text: &'static str,
        why: &'static str,
    }

    /// The lines that quote the old names as the old names, with `{dist}` for the old
    /// distribution and `{pkg}` for the old package.
    const EXEMPT: [Exempt; 1] = [Exempt {
        // The roadmap holds the entry until it is done, then its archive does.
        files: &["docs/ROADMAP.md", "docs/roadmap-done.md"],
        text: "says `pip install {dist}` or `import {pkg}`",
        why: "M10.1d9's done-when names the strings it removes",
    }];

    fn old_distribution() -> String {
        OLD_DISTRIBUTION.concat()
    }

    fn exempt_text(exempt: &Exempt) -> String {
        exempt
            .text
            .replace("{dist}", &old_distribution())
            .replace("{pkg}", OLD_PACKAGE)
    }

    /// A distribution name as PEP 503 compares it: lower case, each run of `-`, `_` and `.` one `-`.
    fn normalized(name: &str) -> String {
        let mut out = String::new();
        let mut separator = false;
        for c in name.chars() {
            if matches!(c, '-' | '_' | '.') {
                separator = true;
                continue;
            }
            if separator && !out.is_empty() {
                out.push('-');
            }
            separator = false;
            out.push(c.to_ascii_lowercase());
        }
        out
    }

    fn is_word(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_'
    }

    /// The byte offsets where `word` starts as a whole word in `line`: not right after a letter,
    /// digit, `_` or `.`, and not right before a letter, digit or `_`.
    fn words<'a>(line: &'a str, word: &'a str) -> impl Iterator<Item = usize> + 'a {
        line.match_indices(word)
            .map(|(at, _)| at)
            .filter(move |&at| {
                let before = line[..at].chars().next_back();
                let after = line[at + word.len()..].chars().next();
                !before.is_some_and(|c| is_word(c) || c == '.') && !after.is_some_and(is_word)
            })
    }

    /// Whether `line` runs `pip install` with the old distribution among its arguments.
    fn installs_the_old_distribution(line: &str) -> bool {
        let old = old_distribution();
        ["pip", "pip3"]
            .into_iter()
            .flat_map(|pip| words(line, pip).map(move |at| at + pip.len()))
            .any(|end| {
                let rest = line[end..].trim_start();
                let Some(rest) = rest.strip_prefix("install") else {
                    return false;
                };
                if rest.starts_with(is_word) {
                    return false;
                }
                // The arguments, to the end of the command.
                let end = rest.find(['`', ';', '&', '|', ')']).unwrap_or(rest.len());
                rest[..end].split_whitespace().any(|argument| {
                    let argument = argument.trim_matches(['"', '\'']);
                    let name = argument
                        .split(['[', '=', '<', '>', '~', '!', '@', ' '])
                        .next()
                        .unwrap_or("");
                    !argument.starts_with('-') && normalized(name) == old
                })
            })
    }

    /// The dotted names in `text` up to the first character no name holds.
    fn dotted(text: &str) -> &str {
        let end = text
            .find(|c: char| !(is_word(c) || c == '.'))
            .unwrap_or(text.len());
        &text[..end]
    }

    fn is_old_package(module: &str) -> bool {
        module == OLD_PACKAGE || module.starts_with(&format!("{OLD_PACKAGE}."))
    }

    /// Whether `line` imports the old top-level package, or a module under it, in any of the
    /// forms the module's documentation lists.
    fn imports_the_old_package(line: &str) -> bool {
        words(line, "import").any(|at| {
            let before = line[..at].trim_end();
            // `from <module> import ...`: the module is the word before `import`.
            let module_start = before
                .rfind(|c: char| !(is_word(c) || c == '.'))
                .map_or(0, |i| i + 1);
            let module = &before[module_start..];
            let head = before[..module_start].trim_end();
            let from = head
                .strip_suffix("from")
                .is_some_and(|ahead| !ahead.ends_with(|c: char| is_word(c) || c == '.'));
            if from && !module.is_empty() {
                return is_old_package(module);
            }
            // `import a, b.c as d, ...`.
            let mut rest = line[at + "import".len()..].trim_start();
            loop {
                let name = dotted(rest);
                if name.is_empty() {
                    return false;
                }
                if is_old_package(name) {
                    return true;
                }
                rest = rest[name.len()..].trim_start();
                if let Some(after) = rest.strip_prefix("as ") {
                    rest = after.trim_start();
                    rest = rest[dotted(rest).len()..].trim_start();
                }
                match rest.strip_prefix(',') {
                    Some(after) => rest = after.trim_start(),
                    None => return false,
                }
            }
        })
    }

    fn names_an_old_python_name(line: &str) -> bool {
        installs_the_old_distribution(line) || imports_the_old_package(line)
    }

    /// The problems in the file `path` with the text `text`, as `path:line: text`. `used` counts,
    /// for each of [`EXEMPT`], the lines it covered.
    fn problems_in(path: &str, text: &str, used: &mut [usize]) -> Vec<String> {
        if path.starts_with(HISTORY) {
            return Vec::new();
        }
        let mut problems = Vec::new();
        for (number, line) in text.lines().enumerate() {
            if !names_an_old_python_name(line) {
                continue;
            }
            let covered = EXEMPT.iter().position(|exempt| {
                let text = exempt_text(exempt);
                exempt.files.contains(&path)
                    && line.contains(&text)
                    && !names_an_old_python_name(&line.replacen(&text, "", 1))
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
                    "no line of {} holds `{}`: drop the exemption ({})",
                    exempt.files.join(" or "),
                    exempt_text(exempt),
                    exempt.why
                ));
            }
        }
        found
    }

    #[test]
    fn no_tracked_file_installs_or_imports_the_old_python_names() {
        let root = crate::designs::root().unwrap();
        assert_eq!(problems(&root), Vec::<String>::new());
    }

    /// Each form the old names take is found; the new names, other names that start the same
    /// way and prose that only mentions the package are not; and an exemption covers only its own
    /// line in its own files.
    #[test]
    fn an_old_python_name_is_found_in_any_form() {
        let dist = old_distribution();
        let pkg = OLD_PACKAGE;
        let named = [
            format!("pip install {dist}"),
            format!("Once it is, `pip install {dist}` installs it"),
            format!("python -m pip install --upgrade \"{dist}[test]\""),
            format!(
                "uv pip install {}==0.1.0",
                dist.to_ascii_uppercase().replace('-', "_")
            ),
            format!("pip3 install numpy {}", dist.replace('-', ".")),
            format!("import {pkg}"),
            format!("    import {pkg}  # the package"),
            format!(">>> import {pkg}.x as y"),
            format!("import os, {pkg}"),
            format!("import numpy as np, {pkg}"),
            format!("from {pkg} import Rocket"),
            format!("from {pkg}._{pkg} import (x,"),
            format!("python -c \"import json; import {pkg}\""),
        ];
        for line in &named {
            assert!(names_an_old_python_name(line), "{line}");
        }
        let clean = [
            "pip install fusionspace-hpr".to_owned(),
            format!("pip install {dist}-fixtures"),
            format!("pip install {pkg}"),
            format!("pip installs {dist} no more"),
            "from fusionspace import hpr".to_owned(),
            "import fusionspace.hpr as hpr".to_owned(),
            format!("import {pkg}_core"),
            format!("import {pkg}z"),
            format!("use {pkg}::Rocket;"),
            format!("differently from {pkg}. The probes'"),
            format!("reads from {pkg} on three details"),
            format!("import {{ readDesign }} from \"./{pkg}-design.ts\";"),
            format!("the {dist} crate"),
        ];
        for line in &clean {
            assert!(!names_an_old_python_name(line), "{line}");
        }

        let mut used = [0; EXEMPT.len()];
        let planted = format!("```python\nimport {pkg}\n```\n");
        assert_eq!(
            problems_in("docs/python.md", &planted, &mut used),
            [format!("docs/python.md:2: import {pkg}")]
        );
        let planted = format!("RUN pip install {dist}\n");
        assert_eq!(
            problems_in("scripts/x.sh", &planted, &mut used),
            [format!("scripts/x.sh:1: RUN pip install {dist}")]
        );
        assert_eq!(
            problems_in("docs/decisions/0199-x.md", &planted, &mut used),
            Vec::<String>::new()
        );
        let done_when = format!(
            "    `docs/decisions/` {}; an `.eng`",
            exempt_text(&EXEMPT[0])
        );
        assert_eq!(
            problems_in("docs/ROADMAP.md", &done_when, &mut used),
            Vec::<String>::new()
        );
        assert_eq!(used, [1]);
        // An exempt line that gains another old name is not covered, nor is it elsewhere.
        let grown = format!("{done_when}; import {pkg}");
        assert_eq!(problems_in("docs/ROADMAP.md", &grown, &mut used).len(), 1);
        assert_eq!(problems_in("README.md", &done_when, &mut used).len(), 1);
    }
}
