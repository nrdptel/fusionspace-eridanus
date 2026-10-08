# ADR-207: The name check on the site's pages (2026-10-08)

- **Status:** accepted; carries out [ADR-206, the name check in two steps][adr-206] §2
- **Summary:** M0.9a2 holds every page `SUMMARY.md` renders, and `SUMMARY.md` itself, to `cargo xtask names`. About 2,280 mentions were rewritten: comparisons and tables say "HPR Sim", other prose "the simulator" or "this project", the command and library `hpr` in code. Two names keep the old one by rule, not by entry: a decision record's file name and the flight engine's folder in a path, `crates/hpr-sim/`. On the records page, a decision record's row inherits a whole-page allowance of the record its first cell's `[adr-NNN]` link names, and nothing else there inherits. 15 entries keep the site's anchors, stamps and history, and 73 records up to ADR-205 are allowed whole for their rows. 16 headings that said `hpr` were rewritten, 15 of them with new anchors, and every link to those changed.

[adr-199]: 0199-the-2026-10-07-fusionspace-hpr.md
[adr-203]: 0203-the-fusionspace-hpr-packages.md
[adr-205]: 0205-the-2026-10-08-one-name-one-design.md
[adr-206]: 0206-the-name-check-in-two-steps.md

**Context.** With the site's 61 pages and `SUMMARY.md` added to the check, `main` at `11d0b59`
held 2,485 mentions the allowlist didn't cover: 438 on the `.ork` format page, 406 on the
aerodynamics page, 159 on the records page (121 of them in decision records' rows) and the rest
across the guides, model pages and references. About 70 were paths under `crates/hpr-sim/` (the
flight engine's folder) or links to decision records whose file names spell the old name. The
check had to say which of those may stay, and how a records-page row inherits an allowance.

**Decision.**

1. **What the check reads.** Every page `SUMMARY.md` links, and `SUMMARY.md` itself, since it is
   the site's sidebar, are Markdown surfaces. The list comes from `SUMMARY.md`'s links, so a page
   added to the site joins the check with no other change.
2. **Two names that keep the old one, by rule** (chosen over an entry per mention): the file name
   of a tracked decision record, linked or quoted, since a record is never renamed; and the old
   name as the flight engine's folder in a path (`crates/` before it, `/` after it), which kept
   its name when the package was renamed ([ADR-203][adr-203]). Neither covers a bare `hpr`, the
   old name elsewhere on the same line, or an untracked record's name. The four M0.9a1 entries
   those rules cover were dropped.
3. **A records-page row inherits only its own record's allowance.** The row's record is the
   `[adr-NNN]` link that opens its first cell, whose label `ADR-NNN:` names the same number. A
   note after that link, such as "superseded by", stays in the row. A row inherits only a
   whole-page entry for that record's file, which only a record up to ADR-205 may have.
   A link to a record later in a row, a milestone's or a lesson's row, and the page's prose
   inherit nothing. The 73 records whose rows held 119 mentions are allowed whole. Their rows
   give each record's summary as it was written, which is history; two rows that shared a phrase
   with a milestone's row lost one mention each to its rewrite. A dropped entry fails on the
   records page.
4. **How the pages were rewritten** (chosen, following [ADR-199][adr-199] §1 and
   `docs/writing.md`). "HPR Sim" names the simulator wherever "the simulator" could mean
   OpenRocket, RocketPy or RASAero too: in comparisons, tables and column headers, so nearly
   everywhere on the `.ork`, aerodynamics, mass, accuracy and validation pages. Elsewhere a page's
   first mention says "HPR Sim" and later ones "the simulator". The command a reader types and the
   Rust library are `hpr` in code. "This project" or "FusionSpace HPR" is the project, its tests or
   its licenses. The flight-log page names FusionSpace HPR · Analyzer. Headings say "the
   simulator", since "HPR Sim" in a heading makes an anchor that spells the old name; 16 headings
   changed, 15 of them with new anchors, and every link to those, including the two OpenRocket flight reports' links and their
   generators.
5. **What stays, by entry.** 15 entries on the site's pages cover 17 mentions. Most are anchors of
   headings that name the `hpr sim` command (`cli.md#hpr-sim`). The rest are old stamps that must
   still read (the `.eng` and `.ork` creator lines), a committed test log's quoted header, the
   cache folders builds before 0.1 used, the private fixtures repository's name, and the house
   style's own statement of the rule. A source quoted on a page was changed at the source: the
   motor example's first line, and the export example's default folder, now
   `fusionspace-hpr-export`.

**Consequences.** M0.9a is met: every surface ADR-205 §1 lists is checked, and the allowlist has
111 entries (38 for text, 73 records allowed whole). A new page, heading or row that calls the
product `hpr` fails `cargo test -p xtask`. A record from ADR-206 on gives its row no allowance, so
its summary must use the names.
