# ADR-208: The design audit: 112 sections against what ships, and three increments to close it (2026-10-08)

- **Status:** accepted; carries out [ADR-205, one name and one design][adr-205] §2 for M0.9b, and adds M0.9c, M0.9d and M0.9e
- **Summary:** `docs/research/design-conformance.md` rows all 112 `##` sections of the FusionSpace product system's 14 `product/` files at the pinned commit `f45454f`. 64 sections sit in the eight files that govern a surface shipping today. Of those, 9 are met (5 held by named tests, 4 by review), 47 are not met, 4 wait for a later surface and 4 govern nothing the project ships. The 48 sections of the platform files (desktop, mobile, watch, embedded, hardware, rockets) wait for the milestones that build those surfaces, except one that never applies. The 47 gaps are 11 issues, #377 to #387, fixed by three new increments queued before the guides: M0.9c (the command line, the exports and the plot), M0.9d (the site's look) and M0.9e (the words). `xtask/src/conformance.rs` fails when a section is missing, the pin moves, a named test isn't live or a named milestone isn't open. Release 0.1 still doesn't wait for the audit (ADR-205 §3), but `review.md`'s *Before release* lists, which ADR-164 §5 applies from 0.1, are not all met.

[adr-164]: 0164-the-fusionspace-product-system.md
[adr-205]: 0205-the-2026-10-08-one-name-one-design.md

**Context.** [ADR-164][adr-164] adopted the product system and M0.6 restyled the site, the
command line and the plot. Nothing listed each rule against the surfaces that ship, and the
name sweep (M0.9a) had shown how a rule nobody re-read is missed. ADR-205 §2 asked for a table
with a row per `##` section, each applying row met (a named test, or reviewed at the commit) or
in an issue with a milestone. Five reviewers each read one or more of the eight applying files
against the site, `hpr`, its exports, the plot, the READMEs and the banners. A second reviewer
for each re-checked every row: they reproduced each defect and opened each named test.

**Decision.**

1. **The table's form.** Five columns: the file, the section as written, the surfaces it
   governs today (site, CLI, exports, plot, README, banners, or `—`), the status and what holds
   it. There are four statuses. ADR-205 named three; **n/a** is added, with its reason, for a section that
   governs nothing the project ships or will ship: the system's reading order, its sources,
   commit messages, the company's own site and a rocket's roll paint.
   - **Met** names live tests as `path/to/file.rs::test`, which the check opens, or says
     "reviewed at `f45454f`" and what was read.
   - **Not met** names its issues and the open milestones that fix them.
   - **Later** names the open milestone that brings the surface.
   The section list sits beside the table in `design-sections.txt`, so CI checks the table
   without `refs/`. The local gate compares the list with the pinned files' headings, and the
   test says when it skipped that.
2. **How a row was judged.** A section is met only when every rule in it that applies to a
   shipped surface holds. A rule the system allows but doesn't require (the braille banner on
   `--version`) is met by not using it. A rule ADR-164 §6 declined counts in its declined form:
   the command line keeps SI first, with US units in brackets. That form is itself not met by
   `hpr sim` (#378).
3. **The gaps, as issues.** The 47 rows share 11 defects, each filed once with its reproduction:
   - **command line, exports and plot:** diagnostics on stdout (#377); no feet in brackets (#378);
     no kind, spread or trust note on results (#379); no catalog as-of date or visible version
     (#380); errors, help, roles, progress and typed numbers (#381); the plot's scale, banking
     and title block (#382);
   - **the site's look:** mdBook's own defaults left unthemed (#383); the page anatomy (#384);
     a figure and the badges drawn outside the tokens (#385);
   - **the words:** US spelling and number style (#386); the READMEs' order (#387).
4. **Three increments, before the guides** (chosen over sending the gaps to release 0.2's
   checklist). M0.9c, M0.9d and M0.9e follow M0.9b in the queue, ahead of M0.7a. ADR-205 §4's
   reason applies: the guides add the most new prose and figures the site has had. Written after
   these increments, they start from a site, a plot and a spelling check that hold the rules.
5. **The platform files** wait for their milestones:
   - desktop: M9.0 chooses the toolkit, M9.1 builds the app;
   - mobile, and the flight card: M9.4;
   - watch: M9.4, which decides the watch link (ADR-191);
   - embedded: M13.3 for logic, M13.4 for lights, screens and buttons, M13.1 for ground stations;
   - hardware: M13.4, and M8.3b for drawings;
   - rockets: the pad checklist M6.8, canopy colors M12.1, and CG and CP marks M0.7a, whose
     figures draw them first. The roll pattern is paint on an airframe, which the project
     doesn't make (n/a).
6. **Release 0.1.** ADR-205 §3 keeps the release from waiting on the audit. But ADR-164 §5
   applies `review.md`'s *Every product* and *Command line* lists from 0.1. M10.1d4's checklist
   didn't read them rule by rule, and seven of their items fail. The one gap in a file that is
   fixed once published is corrected here: the Python package's README said "MIT OR Apache-2.0",
   where its `pyproject.toml` declares `(MIT OR Apache-2.0) AND Apache-2.0`. The rest are the
   command line's behavior and the site, which 0.1.1 can carry. Publishing stays the
   maintainer's call (*Needs Neer*).
7. **Moving the pin reopens the audit.** The test fails when the table's commit differs from
   `validation/refs.lock.toml`'s or doesn't start with the theme's `DESIGN_REV`. A new pin
   means auditing every section again and naming the new commit.

**Alternatives considered.**

- **Mark a section met when most of its rules hold.** Rejected: a row then hides the rule that
  fails. Instead, the evidence in each issue says what holds.
- **One issue per row.** Rejected: 47 issues for 11 defects, and one fix would close many.
- **Send the gaps to M10.2.** Rejected: the guides would be written on a site and plot that
  break the rules they explain.

**Consequences.** The queue grows by three increments before M0.7a. #377 to #387 carry the
work. A rule the system changes reaches this project only when the pin moves, and the audit
runs again then.
