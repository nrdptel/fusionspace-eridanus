# ADR-203: The FusionSpace HPR packages (2026-10-08)

- **Status:** accepted; carries out [ADR-199, FusionSpace HPR](0199-the-2026-10-07-fusionspace-hpr.md) §2–4
- **Summary:** M10.1d9: the 14 published crates are `fusionspace-hpr` and `fusionspace-hpr-*`, keeping their Rust library names (`hpr`, `hpr_core`, …); the Python distribution is `fusionspace-hpr`, imported as `fusionspace.hpr` from an implicit `fusionspace` namespace; stamps, `--version`, `--help`, the site and README titles, the User-Agent, cache folders, release title and archives say FusionSpace HPR; a `hpr-sim` stamp of either designation still reads.

**Context.** ADR-199 named the suite FusionSpace HPR and asked that every package carry the
brand before release 0.1 publishes, since crates.io and PyPI keep a published name for good. It
left open how far a rename reaches into the code: a crate's package name is what `cargo add` and
`cargo -p` take, while its library name is what `use` takes.

**Decision.**

1. **Crates.** Each published package is renamed: `hpr` → `fusionspace-hpr`, each `hpr-*` →
   `fusionspace-hpr-*`. Each keeps its Rust library name through `[lib] name` (`hpr`,
   `hpr_core`, …), so code reads `use hpr::…` as before, the facade imports under the command's
   name, and no example or doc changes its imports. The workspace's dependency keys keep the
   short names with `package = "fusionspace-hpr-…"`, so member manifests and feature strings
   don't change. Folders and the unpublished crates (`hpr-py`, `hpr-ffi`, `hpr-wasm`,
   `hpr-validate`, `xtask`) keep their names, as ADR-199 allows. Every `cargo -p` in the scripts,
   CI and xtask takes the package name. `publish-crates.sh`'s list follows, held by its order
   test.
2. **Python.** The distribution is `fusionspace-hpr`, built by maturin with
   `module-name = "fusionspace.hpr._hpr"` from `python/fusionspace/hpr/`, with no
   `fusionspace/__init__.py`. So `fusionspace` is an implicit namespace package (PEP 420) that
   later FusionSpace packages can share. maturin built it, so ADR-199's fallback
   (`fusionspace_hpr`) isn't needed. The documented idiom is `from fusionspace import hpr`.
   Classes and `HprError` report `__module__` as `fusionspace.hpr`.
3. **The name in use.**
   - The stamp is `FusionSpace HPR 0.1.0 · FS-ACHERNAR · SW · TOOL 001`
     (`hpr_core::tool::NAME`). Readers recognise only pairs a build wrote: the new name with
     `TOOL 001`, and `hpr-sim` (`EARLIER_NAME`) with `TOOL 001` or `TOOL 005`. An `.eng` stamped
     by any of them reads without the stamp and writes back one new stamp, and its tests keep
     the old literals.
   - `hpr --version`, and each subcommand's, prints the stamp. `--help` opens "FusionSpace
     HPR · Sim". Before this, a subcommand's `--version` said `hpr-sim`.
   - The `.hpr` media type is `application/vnd.fusionspace.hpr+json` (`hpr_format::MEDIA_TYPE`),
     defined on the format page and not registered with IANA.
   - The cache folders are `co.fusionspace.hpr` (macOS), `FusionSpace\hpr\cache` (Windows) and
     `fusionspace-hpr` (Linux, XDG). The old ones aren't read.
   - The User-Agent is `fusionspace-hpr/<version> (+https://hpr.fusionspace.co)`, one constant
     (`hpr_net::Http::USER_AGENT`) that xtask reuses and the scripts rebuild.
   - Parquet's `created_by` and the plot's credit say FusionSpace HPR. The schema's types are
     `@fusionspace/hpr-design-types`.
   - The release is titled `FusionSpace HPR <version>`. Its archives are
     `fusionspace-hpr-<version>-<target>`, and its wheels `fusionspace_hpr-<version>-…`.
   - The site's title, the README's heading and the banners say FusionSpace HPR.
4. **Checks.** An xtask test refuses `pip install hpr-sim` and an import of the old `hpr`
   package outside `docs/decisions/`, with one exemption: M10.1d9's done-when, which names them.
   Another test holds the package and library names, the Python layout, the release title and
   archive names, the book title, the README heading and the title block to this ADR.

**Consequences.** A user writes `cargo add fusionspace-hpr` and `use hpr::…`. That split is
common on crates.io, and the crates' pages say it. A developer's motor cache moves with the
folder: `cargo xtask ork-cli --fetch` refills it, or copy the old folder across. Many guide pages
still call the project hpr-sim in running prose; the landing page, README, install, Python and
release pages say FusionSpace HPR. `ARCHITECTURE.md`'s crate map lists folders, which keep their
names.
