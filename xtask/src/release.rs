//! What the release workflow (`.github/workflows/release.yml`, ADR-187) relies on in the
//! repository's own files, checked by `cargo test -p xtask` so a change that breaks a release shows
//! up on the pull request that makes it, not on the day of the release.
//!
//! - Release 0.1 publishes the library crates and `hpr-cli`; only `hpr-py`, `hpr-ffi`, `hpr-wasm`,
//!   `hpr-validate` and `xtask` say `publish = false` (ADR-162 §2), and no published crate
//!   depends on one of them, which crates.io would refuse.
//! - Every published crate has the description, license and repository crates.io shows.
//! - cargo-about accepts the licenses `cargo deny` allows, the same list, so the license texts a
//!   release carries are written for exactly the dependencies CI lets in.
//! - The apogees the smoke tests expect are the ones the docs print, which `cargo xtask cli` and
//!   the Python page's test keep current, so a change that moves a flight moves them too.
//! - Each published crate's folder holds `LICENSE-MIT` and `LICENSE-APACHE`, byte for byte the
//!   repository's, since cargo packs only a crate's own folder (issue #372).
//! - `scripts/release/publish-crates.sh` names every published crate once, each after the
//!   workspace crates it depends on, as `cargo metadata` gives them (issue #372).
//! - The glibc floor in `scripts/release/linux.sh` is the one the install pages state.
//! - `scripts/release/package_managers.py` writes the Homebrew formula and the Scoop manifest for
//!   exactly the archives a release has, and `cargo binstall`'s metadata names those archives
//!   (issue #434).
//! - The names a user meets are FusionSpace HPR's (ADR-199 §2, §4): every published crate is
//!   `fusionspace-hpr` or `fusionspace-hpr-*` and keeps its library's Rust name, the Python package
//!   is `fusionspace-hpr` imported as `fusionspace.hpr` under a namespace package, and the
//!   release's title, its archives, the site's title and the README's heading say FusionSpace HPR.

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::process::Command;

    use toml::Value;

    /// The crates release 0.1 leaves unpublished (ADR-162 §2, M10.1c's *done when*).
    const UNPUBLISHED: [&str; 5] = ["hpr-ffi", "hpr-py", "hpr-validate", "hpr-wasm", "xtask"];

    fn root() -> PathBuf {
        // xtask's manifest folder is the workspace root's child.
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf()
    }

    fn read(path: &Path) -> Value {
        let text = fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        toml::from_str(&text).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
    }

    /// Every workspace member's manifest, by package name.
    fn manifests() -> Vec<(String, Value)> {
        let root = root();
        let mut paths = vec![root.join("xtask/Cargo.toml")];
        for entry in fs::read_dir(root.join("crates")).unwrap() {
            let manifest = entry.unwrap().path().join("Cargo.toml");
            if manifest.is_file() {
                paths.push(manifest);
            }
        }
        paths
            .iter()
            .map(|path| {
                let manifest = read(path);
                let name = manifest["package"]["name"].as_str().unwrap().to_owned();
                (name, manifest)
            })
            .collect()
    }

    fn publishes(manifest: &Value) -> bool {
        manifest["package"].get("publish").and_then(Value::as_bool) != Some(false)
    }

    #[test]
    fn only_the_named_crates_stay_unpublished() {
        let workspace = read(&root().join("Cargo.toml"));
        assert!(
            workspace["workspace"]["package"].get("publish").is_none(),
            "the workspace sets `publish` for every crate; each unpublished crate says it itself"
        );
        let unpublished: BTreeSet<String> = manifests()
            .iter()
            .filter(|(_, manifest)| !publishes(manifest))
            .map(|(name, _)| name.clone())
            .collect();
        let named: BTreeSet<String> = UNPUBLISHED.iter().map(|&n| n.to_owned()).collect();
        assert_eq!(unpublished, named);
    }

    /// The packages a manifest's dependency tables name, outside its dev-dependencies: plain and
    /// build ones, also under `[target.'cfg(..)']`, by the `package` a renamed one points to.
    fn shipped_dependencies(manifest: &Value) -> Vec<String> {
        let mut tables = vec![manifest];
        if let Some(targets) = manifest.get("target").and_then(Value::as_table) {
            tables.extend(targets.values());
        }
        let mut names = Vec::new();
        for table in tables {
            for section in ["dependencies", "build-dependencies"] {
                let Some(dependencies) = table.get(section).and_then(Value::as_table) else {
                    continue;
                };
                for (key, spec) in dependencies {
                    let package = spec.get("package").and_then(Value::as_str).unwrap_or(key);
                    names.push(package.to_owned());
                }
            }
        }
        names
    }

    #[test]
    fn no_published_crate_needs_an_unpublished_one() {
        let mut seen = 0;
        for (name, manifest) in manifests().iter().filter(|(_, m)| publishes(m)) {
            for dependency in shipped_dependencies(manifest) {
                seen += 1;
                assert!(
                    !UNPUBLISHED.contains(&dependency.as_str()),
                    "{name} is published, and needs {dependency}, which isn't"
                );
            }
        }
        assert!(seen > 0, "no dependency was read");
    }

    #[test]
    fn shipped_dependencies_are_read_wherever_a_manifest_puts_them() {
        let manifest: Value = toml::from_str(
            r#"
            [dependencies]
            a = "1"
            [build-dependencies]
            b = "1"
            [dev-dependencies]
            c = "1"
            [target.'cfg(windows)'.dependencies]
            d = "1"
            [target.'cfg(unix)'.dependencies]
            renamed = { package = "hpr-validate", version = "0.1" }
            "#,
        )
        .unwrap();
        let mut names = shipped_dependencies(&manifest);
        names.sort();
        assert_eq!(names, ["a", "b", "d", "hpr-validate"]);
    }

    #[test]
    fn every_published_crate_says_what_crates_io_shows() {
        let workspace = read(&root().join("Cargo.toml"));
        let shared = &workspace["workspace"]["package"];
        for key in ["license", "repository"] {
            assert!(shared.get(key).is_some(), "the workspace has no {key}");
        }
        for (name, manifest) in manifests().iter().filter(|(_, m)| publishes(m)) {
            let package = &manifest["package"];
            let description = package.get("description").and_then(Value::as_str);
            assert!(
                description.is_some_and(|d| !d.trim().is_empty()),
                "{name} has no description"
            );
            for key in ["license", "repository"] {
                let inherited = package
                    .get(key)
                    .and_then(|v| v.get("workspace"))
                    .and_then(Value::as_bool);
                // A crate that bundles third-party data states the project's license and the
                // data's together (`fusionspace-hpr-io`, ADR-187): the project's stays a term.
                let shared_key = shared[key].as_str().unwrap_or_default();
                let joined = key == "license"
                    && package.get(key).and_then(Value::as_str).is_some_and(|own| {
                        own.strip_prefix(&format!("({shared_key}) AND "))
                            .is_some_and(|rest| !rest.trim().is_empty())
                    });
                assert!(
                    inherited == Some(true) || joined,
                    "{name} doesn't take {key} from the workspace"
                );
            }
        }
    }

    #[test]
    fn cargo_about_accepts_what_cargo_deny_allows() {
        let strings = |value: &Value| -> BTreeSet<String> {
            let list = value.as_array().unwrap();
            list.iter()
                .map(|v| v.as_str().unwrap().to_owned())
                .collect()
        };
        let deny = read(&root().join("deny.toml"));
        let about = read(&root().join("scripts/release/about.toml"));
        let allowed = strings(&deny["licenses"]["allow"]);
        assert!(!allowed.is_empty());
        assert_eq!(strings(&about["accepted"]), allowed);
    }

    /// The number after `marker` in `text`, up to the next `)`.
    fn number_after(text: &str, marker: &str) -> String {
        let at = text.find(marker).unwrap_or_else(|| panic!("no `{marker}`")) + marker.len();
        let rest = &text[at..];
        rest[..rest.find(')').unwrap()].trim().to_owned()
    }

    #[test]
    fn the_smoke_tests_expect_the_apogees_the_docs_print() {
        let root = root();
        let read = |path: &str| fs::read_to_string(root.join(path)).unwrap();
        let cli = number_after(&read("scripts/release/smoke-cli.sh"), "abs(apogee - ");
        // SI first, then feet in brackets: `apogee                846.1 m (2776 ft) above the site`.
        // The feet are the apogee's own: whole feet of a height that prints as `cli` to a tenth of
        // a meter, so either end of its half-tenth band, and a stale feet figure fails.
        let apogee_m: f64 = cli.parse().unwrap();
        let lines: BTreeSet<String> = [-0.05, 0.05]
            .iter()
            .map(|half| {
                let feet = ((apogee_m + half) / 0.3048).round();
                format!("apogee                {cli} m ({feet} ft) above the site")
            })
            .collect();
        assert!(
            read("docs/cli.md")
                .lines()
                .any(|printed| lines.iter().any(|line| printed.starts_with(line.as_str()))),
            "docs/cli.md has none of {lines:?}"
        );
        let python = number_after(
            &read("scripts/release/smoke_python.py"),
            "abs(flight.apogee_m - ",
        );
        let line = format!("Apogee:    {python} m at");
        assert!(
            read("docs/python.md").contains(&line),
            "docs/python.md has no `{line}`"
        );
    }

    /// The workflow and cargo-about's settings name the same cargo-about version, and the Python
    /// build uses CI's pinned maturin (read from scripts/python-tests.sh by scripts/release/python.sh).
    #[test]
    fn the_release_pins_agree() {
        let root = root();
        let workflow = fs::read_to_string(root.join(".github/workflows/release.yml")).unwrap();
        let about = fs::read_to_string(root.join("scripts/release/about.toml")).unwrap();
        let pinned = workflow
            .lines()
            .find_map(|line| line.trim().strip_prefix("CARGO_ABOUT: cargo-about@"))
            .expect("release.yml pins cargo-about in its env");
        assert!(
            about.contains(&format!("cargo-about {pinned}")),
            "about.toml doesn't name cargo-about {pinned}"
        );
        let tests = fs::read_to_string(root.join("scripts/python-tests.sh")).unwrap();
        assert!(
            tests
                .lines()
                .any(|l| l.starts_with("MATURIN=\"") && l.ends_with('"'))
        );
    }

    /// `cargo metadata --no-deps`, run on the workspace by the cargo running these tests.
    fn metadata() -> serde_json::Value {
        let output = Command::new(crate::workspace::cargo())
            .args([
                "metadata",
                "--format-version",
                "1",
                "--no-deps",
                "--offline",
            ])
            .current_dir(root())
            .output()
            .expect("cargo metadata runs");
        assert!(
            output.status.success(),
            "cargo metadata failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        serde_json::from_slice(&output.stdout).expect("cargo metadata prints JSON")
    }

    /// The published crates (`publish` not `false`, which metadata gives as `[]`), each with the
    /// folder of its manifest and the workspace crates it depends on. A dev-dependency named by
    /// path alone is left out: cargo drops it from the published manifest.
    fn published_crates() -> BTreeMap<String, (PathBuf, Vec<String>)> {
        let metadata = metadata();
        let packages = metadata["packages"].as_array().unwrap();
        let members: BTreeSet<&str> = packages
            .iter()
            .map(|p| p["name"].as_str().unwrap())
            .collect();
        let mut published = BTreeMap::new();
        for package in packages {
            if package["publish"].as_array().is_some_and(Vec::is_empty) {
                continue;
            }
            let name = package["name"].as_str().unwrap().to_owned();
            let folder = Path::new(package["manifest_path"].as_str().unwrap())
                .parent()
                .unwrap()
                .to_path_buf();
            let mut needs = Vec::new();
            for dependency in package["dependencies"].as_array().unwrap() {
                let needed = dependency["name"].as_str().unwrap();
                let path_only_dev = dependency["kind"].as_str() == Some("dev")
                    && dependency["req"].as_str() == Some("*");
                if members.contains(needed) && !path_only_dev && !needs.iter().any(|n| n == needed)
                {
                    needs.push(needed.to_owned());
                }
            }
            published.insert(name, (folder, needs));
        }
        published
    }

    /// What is wrong with publishing `order`, one crate at a time, given each published crate's
    /// workspace dependencies: a crate missing, named twice or not published, or one before a
    /// crate it needs.
    fn order_problems(order: &[String], needs: &BTreeMap<String, Vec<String>>) -> Vec<String> {
        let mut problems = Vec::new();
        let mut seen = BTreeSet::new();
        for (at, name) in order.iter().enumerate() {
            if !seen.insert(name.as_str()) {
                problems.push(format!("{name} is named twice"));
            }
            let Some(needed) = needs.get(name) else {
                problems.push(format!("{name} isn't a published crate"));
                continue;
            };
            for dependency in needed {
                if !order[..at].contains(dependency) {
                    problems.push(format!("{name} comes before {dependency}, which it needs"));
                }
            }
        }
        for name in needs.keys() {
            if !seen.contains(name.as_str()) {
                problems.push(format!("{name} is published but not named"));
            }
        }
        problems
    }

    /// The crates `scripts/release/publish-crates.sh` publishes, in its order: the words of its
    /// `CRATES=(...)` list.
    fn publish_order() -> Vec<String> {
        let script = fs::read_to_string(root().join("scripts/release/publish-crates.sh")).unwrap();
        let start = script
            .find("\nCRATES=(")
            .expect("publish-crates.sh has a CRATES list")
            + 9;
        let end = start + script[start..].find(')').unwrap();
        script[start..end]
            .split_whitespace()
            .map(str::to_owned)
            .collect()
    }

    #[test]
    fn the_publish_script_puts_each_crate_after_its_dependencies() {
        let needs: BTreeMap<String, Vec<String>> = published_crates()
            .into_iter()
            .map(|(name, (_, needed))| (name, needed))
            .collect();
        assert_eq!(needs.len(), 14, "release 0.1 publishes 14 crates");
        assert!(
            needs.values().any(|n| !n.is_empty()),
            "no dependency was read"
        );
        let order = publish_order();
        assert_eq!(order_problems(&order, &needs), Vec::<String>::new());
    }

    #[test]
    fn a_publish_order_out_of_step_is_refused() {
        let needs: BTreeMap<String, Vec<String>> = [
            ("a".to_owned(), vec![]),
            ("b".to_owned(), vec!["a".to_owned()]),
            ("c".to_owned(), vec!["a".to_owned(), "b".to_owned()]),
        ]
        .into();
        let order =
            |names: &[&str]| -> Vec<String> { names.iter().map(|&n| n.to_owned()).collect() };
        assert!(order_problems(&order(&["a", "b", "c"]), &needs).is_empty());
        assert_eq!(
            order_problems(&order(&["a", "c", "b"]), &needs),
            ["c comes before b, which it needs"]
        );
        assert_eq!(
            order_problems(&order(&["a", "b"]), &needs),
            ["c is published but not named"]
        );
        assert_eq!(
            order_problems(&order(&["a", "a", "b", "c", "d"]), &needs),
            ["a is named twice", "d isn't a published crate"]
        );
    }

    #[test]
    fn every_published_crate_carries_the_two_licenses() {
        let root = root();
        let crates = published_crates();
        assert_eq!(crates.len(), 14, "release 0.1 publishes 14 crates");
        for license in ["LICENSE-MIT", "LICENSE-APACHE"] {
            let original = fs::read(root.join(license)).unwrap();
            assert!(!original.is_empty());
            for (name, (folder, _)) in &crates {
                let copy = fs::read(folder.join(license))
                    .unwrap_or_else(|e| panic!("{name} has no {license}: {e}"));
                assert!(
                    copy == original,
                    "{name}'s {license} differs from the repository's; copy it again"
                );
            }
        }
    }

    /// The install pages state the floor scripts/release/linux.sh builds and checks for.
    #[test]
    fn the_pages_state_the_glibc_floor_the_release_builds_for() {
        let root = root();
        let linux = fs::read_to_string(root.join("scripts/release/linux.sh")).unwrap();
        let floor = linux
            .lines()
            .find_map(|l| l.strip_prefix("GLIBC_FLOOR=\""))
            .and_then(|rest| rest.strip_suffix('"'))
            .expect("linux.sh sets GLIBC_FLOOR");
        for (page, wording) in [
            ("docs/releasing.md", format!("(glibc) {floor} or later")),
            (
                "docs/getting-started.md",
                format!("(glibc) {floor} or later"),
            ),
            ("docs/python.md", format!("glibc {floor} or later")),
            ("CHANGELOG.md", format!("(glibc) {floor} or later")),
        ] {
            // Lines are wrapped anywhere, so spaces and line ends count alike.
            let text = fs::read_to_string(root.join(page)).unwrap();
            let text = text.split_whitespace().collect::<Vec<_>>().join(" ");
            assert!(text.contains(&wording), "{page} doesn't say `{wording}`");
        }
    }

    /// The READMEs in `writing.md`'s order (*READMEs and docs*, #387): what it is, then a status
    /// in a drawing word (`principles.md` §7) with the version, what works and what doesn't,
    /// how to install it, where the details are, and last the license with the designation and
    /// the version. Every published crate shows the project README on crates.io.
    #[test]
    fn the_readmes_follow_the_writing_order_with_the_version() {
        let root = root();
        let workspace = read(&root.join("Cargo.toml"));
        let version = workspace["workspace"]["package"]["version"]
            .as_str()
            .unwrap();
        let licensed = format!("`FS-ACHERNAR · SW · TOOL 001` · version {version} · part of");
        let readmes: [(&str, &[&str]); 2] = [
            (
                "README.md",
                &[
                    "## What works today",
                    "## What doesn't work yet",
                    "## Install and first flight",
                    "## Where the details are",
                    "## License",
                ],
            ),
            (
                "crates/hpr-py/README.md",
                &[
                    "## What works today",
                    "## What doesn't work yet",
                    "## Install",
                    "## Where the details are",
                    "## License",
                ],
            ),
        ];
        for (path, order) in readmes {
            let text = fs::read_to_string(root.join(path)).unwrap();
            // The sections' headings, not a `## ` line inside a fenced block.
            let mut fenced = false;
            let headings: Vec<&str> = text
                .lines()
                .filter(|line| {
                    if line.starts_with("```") {
                        fenced = !fenced;
                    }
                    !fenced && line.starts_with("## ")
                })
                .collect();
            let mut at = 0;
            for heading in order {
                let found = headings[at..].iter().position(|h| h == heading);
                assert!(
                    found.is_some(),
                    "{path}: `{heading}` missing or out of order"
                );
                at += found.unwrap_or_default() + 1;
            }
            assert_eq!(
                headings.last(),
                order.last(),
                "{path}: the license comes last"
            );
            let before_sections = text.split("\n## ").next().unwrap_or_default();
            let status = before_sections
                .lines()
                .find(|line| line.starts_with("**Status: "))
                .unwrap_or_else(|| panic!("{path}: no status before the first section"));
            assert!(
                ["IN PREPARATION", "RELEASED", "WITHDRAWN"]
                    .iter()
                    .any(|word| status.starts_with(&format!("**Status: {word}.**"))),
                "{path}: the status is not a drawing word: {status}"
            );
            assert!(
                status.contains(&format!("Version {version},")),
                "{path}: the status doesn't give version {version}"
            );
            let license = text.rsplit("\n## License\n").next().unwrap_or_default();
            assert!(license.contains(&licensed), "{path}: no `{licensed}`");
        }
        assert_eq!(
            workspace["workspace"]["package"]["readme"].as_str(),
            Some("README.md")
        );
        for (name, manifest) in manifests().iter().filter(|(_, m)| publishes(m)) {
            assert_eq!(
                manifest["package"]["readme"].get("workspace"),
                Some(&Value::Boolean(true)),
                "{name} doesn't show the project README"
            );
        }
    }

    /// ADR-199 §2 and §4: the names of the packages, the release and the pages.
    #[test]
    fn every_name_a_user_meets_is_fusionspace_hprs() {
        let root = root();
        let mut published = 0;
        for (name, manifest) in manifests().iter().filter(|(_, m)| publishes(m)) {
            published += 1;
            assert!(
                name == "fusionspace-hpr" || name.starts_with("fusionspace-hpr-"),
                "{name} is published under a name without FusionSpace's prefix"
            );
            // The library keeps its Rust name, so code says `use hpr_core::…` and `use hpr::…`.
            let lib = name.strip_prefix("fusionspace-").unwrap().replace('-', "_");
            assert_eq!(
                manifest["lib"]["name"].as_str(),
                Some(lib.as_str()),
                "{name}'s library"
            );
        }
        assert_eq!(published, 14);

        let python = root.join("crates/hpr-py");
        let pyproject = read(&python.join("pyproject.toml"));
        assert_eq!(
            pyproject["project"]["name"].as_str(),
            Some("fusionspace-hpr")
        );
        assert_eq!(
            pyproject["tool"]["maturin"]["module-name"].as_str(),
            Some("fusionspace.hpr._hpr")
        );
        assert!(python.join("python/fusionspace/hpr/__init__.py").is_file());
        assert!(
            !python.join("python/fusionspace/__init__.py").exists(),
            "`fusionspace` is an implicit namespace package (PEP 420), shared with other packages"
        );

        let text = |path: &str| fs::read_to_string(root.join(path)).unwrap();
        let workflow = text(".github/workflows/release.yml");
        assert!(workflow.contains("--title \"FusionSpace HPR $version\""));
        assert!(workflow.contains("dist/fusionspace-hpr-*.tar.gz"));
        assert!(!workflow.contains("dist/hpr-"));
        assert!(
            text("scripts/release/archive.sh")
                .contains("name=\"fusionspace-hpr-$version-$target\"")
        );
        assert_eq!(
            read(&root.join("book.toml"))["book"]["title"].as_str(),
            Some("FusionSpace HPR")
        );
        let readme = text("README.md");
        assert_eq!(
            readme.lines().find(|line| line.starts_with("# ")),
            Some("# FusionSpace HPR")
        );
        assert!(
            text("docs/start-here.md")
                .lines()
                .any(|line| line.starts_with("| Title | FusionSpace HPR · Sim: ")),
            "the landing page's title block names FusionSpace HPR · Sim"
        );
    }

    /// Python 3.10 or later (`write_text(newline=...)`), as `python3` or `python`.
    fn python() -> Command {
        let probe = ["-c", "import sys; sys.exit(sys.version_info < (3, 10))"];
        for program in ["python3", "python"] {
            let works = Command::new(program)
                .args(probe)
                .output()
                .is_ok_and(|o| o.status.success());
            if works {
                return Command::new(program);
            }
        }
        panic!(
            "neither python3 nor python is Python 3.10 or later, which package_managers.py needs"
        );
    }

    /// Runs scripts/release/package_managers.py on invented archives named `files`, each holding
    /// its own name's target as its bytes. Returns its output and the folder it wrote into.
    fn package_managers(
        version: &str,
        files: &[&str],
    ) -> (std::process::Output, tempfile::TempDir) {
        let dir = tempfile::tempdir().unwrap();
        let archives = dir.path().join("archives");
        fs::create_dir(&archives).unwrap();
        for file in files {
            let target = file
                .trim_start_matches(&format!("fusionspace-hpr-{version}-"))
                .trim_end_matches(".tar.gz")
                .trim_end_matches(".zip");
            fs::write(archives.join(file), target).unwrap();
        }
        let output = python()
            .arg(root().join("scripts/release/package_managers.py"))
            .args([
                version.as_ref(),
                archives.as_os_str(),
                dir.path().join("out").as_os_str(),
            ])
            .output()
            .unwrap();
        (output, dir)
    }

    /// The formula, byte for byte, and the manifest, as parsed JSON with Unix line ends, for a
    /// release with an archive for every target the script knows (issue #434): the five the
    /// workflow builds and an Intel Mac's, which waits on issue #438. The wheels and the source
    /// package beside the archives are left alone. Each sha256 is the invented archive's:
    /// `shasum -a 256` gives the same.
    #[test]
    fn package_managers_write_the_formula_and_manifest_for_every_archive() {
        let (output, dir) = package_managers(
            "9.8.7",
            &[
                "fusionspace-hpr-9.8.7-aarch64-apple-darwin.tar.gz",
                "fusionspace-hpr-9.8.7-x86_64-apple-darwin.tar.gz",
                "fusionspace-hpr-9.8.7-aarch64-unknown-linux-gnu.tar.gz",
                "fusionspace-hpr-9.8.7-x86_64-unknown-linux-gnu.tar.gz",
                "fusionspace-hpr-9.8.7-x86_64-pc-windows-msvc.zip",
                "fusionspace-hpr-9.8.7-aarch64-pc-windows-msvc.zip",
                "fusionspace_hpr-9.8.7-cp310-abi3-win_amd64.whl",
                "fusionspace_hpr-9.8.7.tar.gz",
            ],
        );
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let release = "https://github.com/nrdptel/fusionspace-eridanus/releases/download";
        let formula = fs::read_to_string(dir.path().join("out/fusionspace-hpr.rb")).unwrap();
        let expected = format!(
            r##"# Written by scripts/release/package_managers.py in nrdptel/fusionspace-eridanus from the
# release's archives; a release replaces it.
class FusionspaceHpr < Formula
  desc "Flight simulator for hobby and high-power rockets"
  homepage "https://hpr.fusionspace.co"
  license all_of: [
    {{ any_of: ["MIT", "Apache-2.0"] }},
    "Apache-2.0",
  ]

  on_macos do
    on_arm do
      url "{release}/v9.8.7/fusionspace-hpr-9.8.7-aarch64-apple-darwin.tar.gz"
      sha256 "602526fe5d0720f8fb8f60272c924a754eb90c461381908e5d8a5f04f6e504c0"
    end

    on_intel do
      url "{release}/v9.8.7/fusionspace-hpr-9.8.7-x86_64-apple-darwin.tar.gz"
      sha256 "838636fb51c7c295f53ac0becfa66f4e7726c388af3fc9397e57c97f5f97c840"
    end
  end

  on_linux do
    on_arm do
      url "{release}/v9.8.7/fusionspace-hpr-9.8.7-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "86bf552cb29f61118330fb817bc3819ddbade898c26ec8c0a3c825f4bc5e2e27"
    end

    on_intel do
      url "{release}/v9.8.7/fusionspace-hpr-9.8.7-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "1bb87e07be9e52fc0dca53a1c0718fac23e8b4202b9c346e12f075762507a544"
    end
  end

  def install
    bin.install "hpr"
  end

  test do
    assert_match version.to_s, shell_output("#{{bin}}/hpr --version")
  end
end
"##
        );
        assert_eq!(formula, expected);

        let manifest = fs::read_to_string(dir.path().join("out/fusionspace-hpr.json")).unwrap();
        assert!(manifest.ends_with("}\n") && !manifest.contains('\r'));
        let manifest: serde_json::Value = serde_json::from_str(&manifest).unwrap();
        let next = "fusionspace-hpr-$version";
        let expected = serde_json::json!({
            "version": "9.8.7",
            "description": "Flight simulator for hobby and high-power rockets: the hpr command line.",
            "homepage": "https://hpr.fusionspace.co",
            "license": {
                "identifier": "(MIT OR Apache-2.0) AND Apache-2.0",
                "url": "https://github.com/nrdptel/fusionspace-eridanus#license"
            },
            "architecture": {
                "64bit": {
                    "url": format!("{release}/v9.8.7/fusionspace-hpr-9.8.7-x86_64-pc-windows-msvc.zip"),
                    "hash": "b8616d900e5165cdc066288852e6c1b100e78e5520260926a0f20f7e33f8e4f8",
                    "extract_dir": "fusionspace-hpr-9.8.7-x86_64-pc-windows-msvc"
                },
                "arm64": {
                    "url": format!("{release}/v9.8.7/fusionspace-hpr-9.8.7-aarch64-pc-windows-msvc.zip"),
                    "hash": "dcd7f6db8776003159f0f672ed64d6b4d00420d2152678c1592145b07bda7fb4",
                    "extract_dir": "fusionspace-hpr-9.8.7-aarch64-pc-windows-msvc"
                }
            },
            "bin": "hpr.exe",
            "checkver": {"github": "https://github.com/nrdptel/fusionspace-eridanus"},
            "autoupdate": {"architecture": {
                "64bit": {
                    "url": format!("{release}/v$version/{next}-x86_64-pc-windows-msvc.zip"),
                    "extract_dir": format!("{next}-x86_64-pc-windows-msvc")
                },
                "arm64": {
                    "url": format!("{release}/v$version/{next}-aarch64-pc-windows-msvc.zip"),
                    "extract_dir": format!("{next}-aarch64-pc-windows-msvc")
                }
            }}
        });
        // Nothing else: no `suggest` of the Visual C++ runtime, which the Windows program
        // doesn't need from release 0.2 (it links the C runtime statically, archive.sh).
        assert_eq!(manifest, expected);
        // The license is the one `fusionspace-hpr-io` states (ADR-187): the program compiles in
        // OpenRocket's Apache-2.0 parts catalog through that crate.
        let io = read(&root().join("crates/hpr-io/Cargo.toml"));
        assert_eq!(
            manifest["license"]["identifier"].as_str(),
            io["package"]["license"].as_str()
        );
    }

    /// Release 0.1's three archives: an operating system with one architecture's archive says
    /// so, so Homebrew refuses the other rather than installing a program it can't run, and the
    /// manifest has no `arm64` entry.
    #[test]
    fn package_managers_name_only_the_architectures_a_release_has() {
        let (output, dir) = package_managers(
            "0.1.0",
            &[
                "fusionspace-hpr-0.1.0-aarch64-apple-darwin.tar.gz",
                "fusionspace-hpr-0.1.0-x86_64-unknown-linux-gnu.tar.gz",
                "fusionspace-hpr-0.1.0-x86_64-pc-windows-msvc.zip",
            ],
        );
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let formula = fs::read_to_string(dir.path().join("out/fusionspace-hpr.rb")).unwrap();
        for wanted in [
            "  on_macos do\n    depends_on arch: :arm64\n\n    on_arm do\n",
            "  on_linux do\n    depends_on arch: :x86_64\n\n    on_intel do\n",
        ] {
            assert!(
                formula.contains(wanted),
                "the formula lacks {wanted:?}:\n{formula}"
            );
        }
        assert_eq!(formula.matches("    on_").count(), 2);
        assert!(!formula.contains("  depends_on :"));
        let manifest = fs::read_to_string(dir.path().join("out/fusionspace-hpr.json")).unwrap();
        let manifest: serde_json::Value = serde_json::from_str(&manifest).unwrap();
        let arches = manifest["architecture"].as_object().unwrap();
        assert_eq!(arches.keys().collect::<Vec<_>>(), ["64bit"]);
    }

    /// An archive for a target neither file knows stops the script, as does a folder with no
    /// archive for the version, so a release never ships a formula that quietly leaves one out.
    #[test]
    fn package_managers_refuse_an_unknown_target_and_an_empty_folder() {
        let (output, _dir) = package_managers(
            "9.8.7",
            &[
                "fusionspace-hpr-9.8.7-x86_64-pc-windows-msvc.zip",
                "fusionspace-hpr-9.8.7-aarch64-apple-darwin.tar.gz",
                "fusionspace-hpr-9.8.7-riscv64gc-unknown-linux-gnu.tar.gz",
            ],
        );
        assert!(!output.status.success());
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("no formula or manifest entry for riscv64gc-unknown-linux-gnu"),
            "{stderr}"
        );

        let (output, _dir) = package_managers(
            "9.8.7",
            &["fusionspace-hpr-9.8.6-x86_64-pc-windows-msvc.zip"],
        );
        assert!(!output.status.success());
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("no fusionspace-hpr-9.8.7-<target> archives"),
            "{stderr}"
        );
    }

    /// A Linux or Mac archive packed as a .zip, or a Windows one as a .tar.gz, stops the script:
    /// the formula and the manifest each take one kind, and a release packs no other.
    #[test]
    fn package_managers_refuse_an_archive_of_the_wrong_kind() {
        for wrong in [
            "fusionspace-hpr-9.8.7-x86_64-unknown-linux-gnu.zip",
            "fusionspace-hpr-9.8.7-x86_64-pc-windows-msvc.tar.gz",
        ] {
            let (output, _dir) = package_managers(
                "9.8.7",
                &[wrong, "fusionspace-hpr-9.8.7-aarch64-apple-darwin.tar.gz"],
            );
            assert!(!output.status.success(), "{wrong} was taken");
            let stderr = String::from_utf8_lossy(&output.stderr);
            assert!(stderr.contains("is a .zip on Windows only"), "{stderr}");
        }
    }

    /// The version is the crate's, `9.8.7`, not the tag's `v9.8.7`, which would put `vv9.8.7`
    /// in every URL.
    #[test]
    fn package_managers_refuse_a_version_that_is_not_a_release_version() {
        for wrong in ["v9.8.7", "9.8", "9.8.7 ", ""] {
            let (output, _dir) = package_managers(
                wrong,
                &[&format!(
                    "fusionspace-hpr-{wrong}-aarch64-apple-darwin.tar.gz"
                )],
            );
            assert!(!output.status.success(), "{wrong:?} was taken");
            let stderr = String::from_utf8_lossy(&output.stderr);
            assert!(stderr.contains("isn't a release version"), "{stderr}");
        }
    }

    /// A release with Mac archives only says `depends_on :macos`, and one with Linux archives
    /// only says `depends_on :linux`, so Homebrew on the other system refuses the formula.
    #[test]
    fn package_managers_name_the_one_operating_system_a_formula_serves() {
        for (system, archives) in [
            (
                "macos",
                [
                    "fusionspace-hpr-9.8.7-aarch64-apple-darwin.tar.gz",
                    "fusionspace-hpr-9.8.7-x86_64-apple-darwin.tar.gz",
                ],
            ),
            (
                "linux",
                [
                    "fusionspace-hpr-9.8.7-aarch64-unknown-linux-gnu.tar.gz",
                    "fusionspace-hpr-9.8.7-x86_64-unknown-linux-gnu.tar.gz",
                ],
            ),
        ] {
            let mut files = archives.to_vec();
            files.push("fusionspace-hpr-9.8.7-x86_64-pc-windows-msvc.zip");
            let (output, dir) = package_managers("9.8.7", &files);
            assert!(
                output.status.success(),
                "{}",
                String::from_utf8_lossy(&output.stderr)
            );
            let formula = fs::read_to_string(dir.path().join("out/fusionspace-hpr.rb")).unwrap();
            assert_eq!(formula.matches("  depends_on :").count(), 1, "{formula}");
            assert!(
                formula.contains(&format!("\n  depends_on :{system}\n")),
                "{formula}"
            );
            assert!(formula.contains(&format!("  on_{system} do\n    on_arm do\n")));
            assert!(!formula.contains("depends_on arch:"), "{formula}");
        }
    }

    /// `cargo binstall fusionspace-hpr-cli` looks for the archives scripts/release/archive.sh
    /// packs, under the release's tag, and finds `hpr` in the one folder each holds; Windows'
    /// are .zip files. binstall's own build host is off, so it takes only this repository's.
    #[test]
    fn binstall_fetches_the_release_archives() {
        let root = root();
        let cli = read(&root.join("crates/hpr-cli/Cargo.toml"));
        let binstall = &cli["package"]["metadata"]["binstall"];
        let archive = "fusionspace-hpr-{ version }-{ target }";
        assert_eq!(
            binstall["pkg-url"].as_str(),
            Some(
                format!(
                    "{{ repo }}/releases/download/v{{ version }}/{archive}{{ archive-suffix }}"
                )
                .as_str()
            )
        );
        assert_eq!(
            binstall["bin-dir"].as_str(),
            Some(format!("{archive}/{{ bin }}{{ binary-ext }}").as_str())
        );
        assert_eq!(binstall["pkg-fmt"].as_str(), Some("tgz"));
        assert_eq!(
            binstall["disabled-strategies"]
                .as_array()
                .map(Vec::as_slice),
            Some([Value::from("quick-install")].as_slice())
        );
        let overrides = binstall["overrides"].as_table().unwrap();
        assert_eq!(
            overrides.keys().collect::<Vec<_>>(),
            ["aarch64-pc-windows-msvc", "x86_64-pc-windows-msvc"]
        );
        for table in overrides.values() {
            assert_eq!(table["pkg-fmt"].as_str(), Some("zip"));
        }
        let packer = fs::read_to_string(root.join("scripts/release/archive.sh")).unwrap();
        for packs in [
            "name=\"fusionspace-hpr-$version-$target\"",
            "cp \"$built\" README.md \"$out/$name/\"",
            "7z a -tzip \"$name.zip\" \"$name\"",
            "tar -czf \"$name.tar.gz\" \"$name\"",
        ] {
            assert!(packer.contains(packs), "archive.sh no longer says {packs}");
        }
        // The release names its tag `v<version>`.
        let workflow = fs::read_to_string(root.join(".github/workflows/release.yml")).unwrap();
        assert!(workflow.contains("gh release create \"v$version\""));
    }

    /// How [`pe`] lays a test program out; the default is an ordinary x86-64 program.
    #[derive(Clone, Copy)]
    struct Pe {
        /// The optional header's magic: 0x20b for PE32+, 0x10b for 32-bit PE32.
        magic: u16,
        /// Each delay-load descriptor's attributes: 1 for RVAs, 0 for the old form's addresses.
        delay_attributes: u32,
        /// The DLL names in a second section, apart from the descriptors.
        names_apart: bool,
        /// SizeOfOptionalHeader; the section table follows the optional header.
        optional_size: usize,
        /// NumberOfRvaAndSizes, which may claim more directories than the header holds.
        directories: u32,
    }

    impl Default for Pe {
        fn default() -> Self {
            Self {
                magic: 0x20b,
                delay_attributes: 1,
                names_apart: false,
                optional_size: 240,
                directories: 16,
            }
        }
    }

    /// A minimal PE file whose import table names `imports` and whose delay-load table names
    /// `delayed`: the headers, a section at RVA 0x1000 (file offset 0x200) holding the import
    /// descriptors (20 bytes each, the name's RVA at +12) and the delay-load descriptors (32
    /// bytes, the attributes at +0, the name's RVA at +4), each list ended by a zero descriptor,
    /// then the names, in the same section or in a second one at RVA 0x2000. That is the layout
    /// Microsoft's PE format page gives, and all the checker reads.
    fn pe(imports: &[&str], delayed: &[&str], shape: Pe) -> Vec<u8> {
        fn put(file: &mut [u8], at: usize, bytes: &[u8]) {
            file[at..at + bytes.len()].copy_from_slice(bytes);
        }
        fn le(value: usize) -> [u8; 4] {
            u32::try_from(value).unwrap().to_le_bytes()
        }
        const FIRST_RVA: usize = 0x1000;
        const SECOND_RVA: usize = 0x2000;
        const RAW: usize = 0x200;
        let delay_at = (imports.len() + 1) * 20;
        let mut first = vec![0u8; delay_at + (delayed.len() + 1) * 32];
        let mut second = Vec::new();
        for (i, name) in imports.iter().chain(delayed).enumerate() {
            let (names, base) = if shape.names_apart {
                (&mut second, SECOND_RVA)
            } else {
                (&mut first, FIRST_RVA)
            };
            let rva = le(base + names.len());
            names.extend_from_slice(name.as_bytes());
            names.push(0);
            if i < imports.len() {
                put(&mut first, i * 20 + 12, &rva);
            } else {
                let at = delay_at + (i - imports.len()) * 32;
                put(&mut first, at, &shape.delay_attributes.to_le_bytes());
                put(&mut first, at + 4, &rva);
            }
        }
        let mut file = vec![0u8; RAW];
        put(&mut file, 0, b"MZ");
        put(&mut file, 0x3c, &0x40u32.to_le_bytes());
        put(&mut file, 0x40, b"PE\0\0");
        put(&mut file, 0x44, &0x8664u16.to_le_bytes()); // x86-64
        let sections: u16 = if shape.names_apart { 2 } else { 1 };
        put(&mut file, 0x46, &sections.to_le_bytes());
        let optional_size = u16::try_from(shape.optional_size).unwrap();
        put(&mut file, 0x54, &optional_size.to_le_bytes());
        let optional = 0x58;
        put(&mut file, optional, &shape.magic.to_le_bytes());
        put(&mut file, optional + 24, &0x1_4000_0000u64.to_le_bytes()); // image base
        put(&mut file, optional + 108, &shape.directories.to_le_bytes());
        let directory = |index: usize| optional + 112 + 8 * index;
        put(&mut file, directory(1), &le(FIRST_RVA)); // imports
        put(&mut file, directory(1) + 4, &le(delay_at));
        if !delayed.is_empty() {
            put(&mut file, directory(13), &le(FIRST_RVA + delay_at)); // delay-load imports
            put(&mut file, directory(13) + 4, &le(32));
        }
        // The section table, written last, so it holds whatever bytes it shares with the
        // directories a short optional header claims.
        let table = optional + shape.optional_size;
        let mut raw = RAW;
        for (i, (body, rva)) in [(&first, FIRST_RVA), (&second, SECOND_RVA)]
            .into_iter()
            .take(sections.into())
            .enumerate()
        {
            let header = table + 40 * i;
            put(&mut file, header, b".idata\0\0");
            put(&mut file, header + 8, &le(body.len()));
            put(&mut file, header + 12, &le(rva));
            put(&mut file, header + 16, &le(body.len()));
            put(&mut file, header + 20, &le(raw));
            raw += body.len();
        }
        file.extend_from_slice(&first);
        file.extend_from_slice(&second);
        file
    }

    /// An ordinary x86-64 program importing `imports`, and delay-loading `delayed`.
    fn pe_importing(imports: &[&str], delayed: &[&str]) -> Vec<u8> {
        pe(imports, delayed, Pe::default())
    }

    /// Runs scripts/release/check_windows_runtime.py on `bytes` written to a file.
    fn windows_runtime(bytes: &[u8]) -> (bool, String) {
        let dir = tempfile::tempdir().unwrap();
        let exe = dir.path().join("hpr.exe");
        fs::write(&exe, bytes).unwrap();
        let output = python()
            .arg(root().join("scripts/release/check_windows_runtime.py"))
            .arg(&exe)
            .output()
            .unwrap();
        let said = String::from_utf8_lossy(&output.stdout).into_owned()
            + &String::from_utf8_lossy(&output.stderr);
        (output.status.success(), said)
    }

    /// The Windows program must not need the Visual C++ runtime, which a clean Windows may lack:
    /// release 0.1.0's `hpr.exe` imported VCRUNTIME140.dll. The check fails on that import,
    /// whether linked or delay-loaded and in any case, and on the C++ runtime's MSVCP140.dll, and
    /// passes the Universal C Runtime's api-ms-win-crt-* names, which Windows 10 and later carry.
    #[test]
    fn the_windows_runtime_check_refuses_the_visual_cpp_runtime() {
        let (passed, said) = windows_runtime(&pe_importing(
            &[
                "KERNEL32.dll",
                "api-ms-win-crt-runtime-l1-1-0.dll",
                "bcrypt.dll",
            ],
            &[],
        ));
        assert!(passed, "{said}");
        assert!(said.contains("api-ms-win-crt-runtime-l1-1-0.dll"), "{said}");
        // Windows' own msvcrt.dll isn't the Redistributable's.
        let (passed, said) = windows_runtime(&pe_importing(&["KERNEL32.dll", "msvcrt.dll"], &[]));
        assert!(passed, "{said}");
        // Names kept in another section than the descriptors are read there.
        let apart = Pe {
            names_apart: true,
            ..Pe::default()
        };
        let (passed, said) = windows_runtime(&pe(&["KERNEL32.dll"], &["VCRUNTIME140.dll"], apart));
        assert!(!passed && said.contains("needs VCRUNTIME140.dll"), "{said}");
        let (passed, said) = windows_runtime(&pe(&["KERNEL32.dll", "bcrypt.dll"], &[], apart));
        assert!(
            passed && said.contains("imports KERNEL32.dll, bcrypt.dll"),
            "{said}"
        );
        for (imports, delayed, named) in [
            (
                &["KERNEL32.dll", "VCRUNTIME140.dll"][..],
                &[][..],
                "VCRUNTIME140.dll",
            ),
            (
                &["KERNEL32.dll", "vcruntime140.dll"][..],
                &[][..],
                "vcruntime140.dll",
            ),
            (
                &["KERNEL32.dll"][..],
                &["VCRUNTIME140_1.dll"][..],
                "VCRUNTIME140_1.dll",
            ),
            (
                &["KERNEL32.dll", "MSVCP140.dll"][..],
                &[][..],
                "MSVCP140.dll",
            ),
            (
                &["KERNEL32.dll", "msvcr120.dll"][..],
                &[][..],
                "msvcr120.dll",
            ),
            (
                &["KERNEL32.dll", "ucrtbased.dll"][..],
                &[][..],
                "ucrtbased.dll",
            ),
        ] {
            let (passed, said) = windows_runtime(&pe_importing(imports, delayed));
            assert!(!passed, "{named} was taken: {said}");
            assert!(said.contains(&format!("needs {named}")), "{said}");
        }
    }

    /// A file the check can't read, or one it reads no imports from, fails rather than passing
    /// with nothing checked.
    #[test]
    fn the_windows_runtime_check_fails_closed() {
        let (passed, said) = windows_runtime(b"#!/bin/sh\necho not a Windows program\n");
        assert!(!passed && said.contains("not a Windows program"), "{said}");
        let (passed, said) = windows_runtime(&pe_importing(&[], &[]));
        assert!(!passed && said.contains("no imports"), "{said}");
        let mut cut = pe_importing(&["KERNEL32.dll"], &[]);
        cut.truncate(0x210);
        let (passed, said) = windows_runtime(&cut);
        assert!(!passed && said.contains("not a Windows program"), "{said}");
        // No 32-bit Windows target ships, so a PE32 file is refused, not read.
        let pe32 = Pe {
            magic: 0x10b,
            ..Pe::default()
        };
        let (passed, said) = windows_runtime(&pe(&["KERNEL32.dll"], &[], pe32));
        assert!(!passed && said.contains("not a 64-bit PE"), "{said}");
        // The old delay-load form holds addresses, which the check doesn't read.
        let old_form = Pe {
            delay_attributes: 0,
            ..Pe::default()
        };
        let (passed, said) = windows_runtime(&pe(&["KERNEL32.dll"], &["bcrypt.dll"], old_form));
        assert!(!passed && said.contains("old delay-load form"), "{said}");
    }

    /// NumberOfRvaAndSizes counts only the directories SizeOfOptionalHeader holds: here it says
    /// 16 in a header with room for 11, so the delay-load directory (13) would be read from the
    /// section table that follows, and the check would chase a stray RVA.
    #[test]
    fn the_windows_runtime_check_reads_only_the_directories_the_header_holds() {
        let short = Pe {
            optional_size: 112 + 8 * 11,
            ..Pe::default()
        };
        let (passed, said) = windows_runtime(&pe(&["KERNEL32.dll"], &[], short));
        assert!(passed, "{said}");
        let huge = Pe {
            directories: u32::MAX,
            ..Pe::default()
        };
        let (passed, said) = windows_runtime(&pe(&["KERNEL32.dll"], &["bcrypt.dll"], huge));
        assert!(
            passed && said.contains("imports KERNEL32.dll, bcrypt.dll"),
            "{said}"
        );
    }

    /// The release builds the Windows program with the C runtime linked in, for the Windows
    /// targets only, and its smoke test runs the import check on every `.exe`.
    #[test]
    fn the_windows_archives_link_the_c_runtime_statically() {
        let root = root();
        let packer = fs::read_to_string(root.join("scripts/release/archive.sh")).unwrap();
        assert!(
            packer.contains("*-windows-msvc)")
                && packer.contains("_RUSTFLAGS=-C target-feature=+crt-static\""),
            "archive.sh no longer links the C runtime statically on Windows"
        );
        let smoke = fs::read_to_string(root.join("scripts/release/smoke-cli.sh")).unwrap();
        assert!(smoke.contains("scripts/release/check_windows_runtime.py \"$hpr\""));
    }

    /// archive.sh's Windows branch, run by bash with `target` set: it exports the static C
    /// runtime's flags, and refuses a RUSTFLAGS or CARGO_ENCODED_RUSTFLAGS that is set at all,
    /// even empty, since cargo then ignores the target's flags. Unix only: on Windows `bash` may
    /// be WSL's rather than Git's.
    #[cfg(unix)]
    #[test]
    fn archive_refuses_any_rustflags_on_windows() {
        let packer = fs::read_to_string(root().join("scripts/release/archive.sh")).unwrap();
        let arm = packer
            .split_once("\n  *-windows-msvc)\n")
            .and_then(|(_, rest)| rest.split_once("\n    ;;\n"))
            .map(|(body, _)| body)
            .expect("archive.sh has a *-windows-msvc) branch");
        let script = format!(
            "set -euo pipefail\ntarget=x86_64-pc-windows-msvc\n{arm}\n\
             printf %s \"$CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_RUSTFLAGS\"\n"
        );
        let run = |set: &[(&str, &str)]| {
            let mut bash = Command::new("bash");
            bash.arg("-c")
                .arg(&script)
                .env_remove("RUSTFLAGS")
                .env_remove("CARGO_ENCODED_RUSTFLAGS")
                .env_remove("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_RUSTFLAGS");
            for (key, value) in set {
                bash.env(key, value);
            }
            bash.output().unwrap()
        };
        let output = run(&[]);
        assert!(output.status.success(), "{output:?}");
        assert_eq!(output.stdout, b"-C target-feature=+crt-static");
        for set in [
            ("RUSTFLAGS", ""),
            ("RUSTFLAGS", "-D warnings"),
            ("CARGO_ENCODED_RUSTFLAGS", ""),
            ("CARGO_ENCODED_RUSTFLAGS", "-Dwarnings"),
        ] {
            let output = run(&[set]);
            assert!(!output.status.success(), "{set:?} was taken");
            let stderr = String::from_utf8_lossy(&output.stderr);
            assert!(
                stderr.contains("would drop the static C runtime"),
                "{stderr}"
            );
        }
    }
}
