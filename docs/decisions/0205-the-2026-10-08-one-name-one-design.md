# ADR-205: Neer's 2026-10-08 sweep: one name and one design on every surface, checked by a tool (2026-10-08)

- **Status:** accepted; orders M0.9a and M0.9b ahead of M0.7a in the queue
- **Summary:** M10.1d9 renamed the packages, stamps and titles, but the words around them still say hpr-sim: 242 times in 35 of the site's rendered pages, 113 in the crates, 5 in the README, and in the license, notice and crate descriptions every package ships with. M0.9a renames them by ADR-199's rules and adds `cargo xtask names`, which fails on `hpr-sim`, or a bare `hpr` naming the product in prose, outside a narrow allowlist whose entries each give their reason and must match. The site's landing page introduces FusionSpace HPR as the suite and the Sim as its first product. M0.9b audits every shipped surface against the pinned design system, rule by rule, in a table kept in the repository; each rule that applies is met and checked, or has an issue. M0.9a should land before release 0.1 publishes, because a published version's README and crate docs never change.

[adr-196]: 0196-the-2026-10-07-product-guides.md
[adr-199]: 0199-the-2026-10-07-fusionspace-hpr.md
[adr-203]: 0203-the-fusionspace-hpr-packages.md

**Context.** On October 8, 2026, after release 0.1's checklist was met, Neer noted that hpr.fusionspace.co was still
headlined by hpr-sim, and asked for a thorough sweep so that every name and every design standard lines up with
fusionspace-design, with no gaps, while noting the docs cover only the simulator for now ([VISION](../VISION.md), that
date). Measured on `main` at `ce54815`:

- **The titles changed; the text didn't.** The site's `<title>` and header read FusionSpace HPR ([ADR-203][adr-203]),
  but the text says hpr-sim. The 62 pages `SUMMARY.md` renders hold 242 mentions in 35 files; the landing page,
  `start-here.md`, holds 14, as in "What hpr-sim is". The crates hold 113 (package descriptions, rustdoc, help text,
  messages and tests), the README 5. Text that ships with every package says it too: crate descriptions such as
  `fusionspace-hpr`'s "The hpr-sim library facade", the license files ("the hpr-sim contributors"),
  `THIRD-PARTY-NOTICES.md` and the CHANGELOG. About 107 of the site's 242 are not the product's name: paths under
  `crates/hpr-sim/`, ADR file names, `cli.md#hpr-sim` anchors, `hpr-sim-fixtures` and crate ids. The prose also uses
  a bare `hpr` for the product ("hpr's apogees") on many pages, where ADR-199 §1 says HPR Sim.
- **No check stops a new one.** M10.1d9's done-when listed the surfaces that had to change, and nothing fails when a
  page written later says hpr-sim. The guides (M0.7a) and product guides (M0.8a), next in the queue, will write the
  most new prose the site has had.
- **The design system's rules are adopted, not audited.** [ADR-164](0164-the-fusionspace-product-system.md) adopted
  them and M0.6 restyled the site, the command line and the plot. M10.1d7 moved the pin to `f45454f`. No table lists
  each rule in the 14 files under `product/` against the surfaces that ship, so a rule nobody re-read can be missed
  the way the name was.
- **The docs describe the Sim alone,** because it is the only product shipped. That is correct for 0.1. The landing
  page should still say what the suite is, so the analyzer (0.2) and the app (0.5) join it without a rewrite.
  [ADR-196][adr-196]'s product guides already give each product its own landing page.
- **A published version's text is fixed.** crates.io and PyPI keep the README and crate docs each version was
  published with, so 0.1.0's would say hpr-sim for good.

**Decision.**

1. **M0.9a, the name everywhere,** goes first in the queue. Prose follows [ADR-199][adr-199] §1: "FusionSpace HPR"
   for the suite, "FusionSpace HPR · Sim" at a product's first mention on a page, "HPR Sim" or "the simulator"
   after it; `hpr`, in code formatting, is the command and the Rust library, never the product's name in prose.
   Its done-when:
   - **the check:** `cargo xtask names` (run by `cargo test -p xtask`) fails on `hpr-sim` anywhere in these
     surfaces, in any case and with the non-breaking hyphen U+2011 too, and on a lowercase `hpr` outside code
     formatting in their prose:
     - the pages `SUMMARY.md` renders, the records page included, and the README;
     - every published crate's `Cargo.toml` description, README and rustdoc, and `pyproject.toml`;
     - Python docstrings, the schemas' text and any bindings generated from them;
     - the command line's help and messages;
     - `LICENSE-MIT` and its copies, `THIRD-PARTY-NOTICES.md` and the CHANGELOG;
     - the banners' `project.json`, from which the PNGs are rebuilt;
   - **the allowlist:** each entry names a file and the exact text or a narrow pattern, with its reason (history
     such as "called hpr-sim before 0.1", an old stamp that must still read, `hpr-sim-fixtures`, a path under
     `crates/hpr-sim/`, an anchor, a crate id). An entry that matches nothing fails as stale. Only ADR-205 and
     earlier ADRs, and the roadmap's archive, may be allowed whole; a records-page row inherits the allowance of the
     ADR its `[adr-NNN]` link names, and nothing else;
   - **the tests:** one plants a mention in each kind of surface and fails; one removes an entry and fails; one
     widens an entry to a whole page and fails;
   - **the landing page:** a test reads `start-here.md`'s first paragraph and fails unless it names FusionSpace HPR
     and each shipped product, from one list in `xtask` that each release milestone extends.
2. **M0.9b, the design audit,** follows it. `docs/research/design-conformance.md` names the pinned commit in full,
   read from `validation/refs.lock.toml`, and has a row for every `##` section of the 14 files under `product/`,
   marked as applying to a surface that ships today (the docs site, the command line, the exports and their stamps,
   the plot, the README and banners) or not. Each applying row says met or not met and what holds it: a named test,
   or "reviewed at" the commit. Each "not met" has an issue naming the milestone that will meet it. Rows for
   surfaces that don't exist yet (apps, watches, firmware, hardware) name the milestone that will apply them. A test
   fails when a section is missing from the table, or when the table's commit differs from `refs.lock.toml`'s or
   doesn't start with `xtask/src/site/theme.rs`'s `DESIGN_REV`, so moving the pin reopens the audit. The section
   list is committed beside the table, so the test runs in CI, where `refs/` is absent; a `refs/` check, skipped
   there and saying so, compares that list with the pinned files.
3. **Release 0.1 waits for M0.9a.** Publishing stays Neer's, after M0.9a merges. M0.9b doesn't hold the release: the
   site can change after publishing, and anything in the audit that touches a published file goes in an issue for 0.1.1
   or 0.2.
4. **M0.7a and M0.8a follow,** so the guides are written under the check.
