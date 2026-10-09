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
}
