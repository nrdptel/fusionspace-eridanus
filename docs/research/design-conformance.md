# Design conformance: every section of the product system, against what ships

This note audits FusionSpace HPR against the FusionSpace product system, the rules in the 14 files
under `product/` in `nrdptel/fusionspace-design` that [ADR-164](../decisions/0164-the-fusionspace-product-system.md)
adopted. It has one row for every `##` section of those files ([ADR-205](../decisions/0205-the-2026-10-08-one-name-one-design.md)
§2, [ADR-208](../decisions/0208-the-design-audit.md)), so a rule nobody re-read can't be missed. How far
to trust it: a row held by a test is checked on every change; a row "reviewed at" the commit was read
against the surface once, at that commit, and can drift until the next review.

Pinned commit: `f45454f44669cc049222f2ab19eb648796e56a52`, as `validation/refs.lock.toml` pins it.

- **Applies to:** the surfaces that ship today: the docs **site**, the **CLI** (`hpr`), the files it
  **exports** and their stamps, the **plot** `hpr sim --plot` draws, the **README**s and the
  **banners**; `none` when the section governs none of them.
- **Status:** **met**, held by a named test (`file.rs::test`) or "reviewed at" the commit with what was
  read; **not met**, with its issue and the milestone that will meet it; **later**, for a surface that
  doesn't ship yet, with the milestone that will apply it; **n/a**, for what the project never ships.
- **The checks:** `xtask/src/conformance.rs` (run by `cargo test -p xtask`) fails on a section missing
  from [`design-sections.txt`](design-sections.txt)'s list, a commit other than the lock's, a named test
  that isn't a live `#[test]`, or a milestone that isn't open. Where `refs/` is checked out, it compares
  that list with the pinned files' headings. Moving the pin fails it until every row is audited again.

| File | Section | Applies to | Status | Held by |
|---|---|---|---|---|
| `README.md` | Read in this order | none | n/a | a reading order for the system's own files, each audited in its own rows |
| `README.md` | The idea in one paragraph | site, CLI, exports, plot | not met | #384, #379; M0.9c, M0.9d: a title block on one page of 62; results with no trust note |
| `README.md` | Files | site, CLI, plot | met | `xtask/src/site/theme.rs::the_committed_theme_passes`, `crates/hpr-cli/tests/style.rs::the_styles_are_the_product_systems`: copied files byte for byte at the pin |
| `README.md` | Pointing a project here | README, site | not met | #387, #385; M0.9d, M0.9e: the README's adaptation drops foundations; images outside the tokens |
| `README.md` | Status | CLI, site | not met | #386; M0.9e: Rev A's US spelling: `calibres` in `hpr sim` and the site |
| `README.md` | License | site, CLI, README, banners | met | `xtask/src/site/theme.rs::the_committed_theme_passes` (SPDX lines, font licenses); notices and brand files read |
| `principles.md` | 1. Drawn, not decorated | site, plot | not met | #384, #385, #383, #382; M0.9c, M0.9d: no title blocks or sheet numbers; a figure's line types; soft shadows |
| `principles.md` | 2. Show the working | site, CLI, exports, plot | not met | #380; M0.9c: exports carry no catalog as-of date |
| `principles.md` | 3. Say how far to trust it | CLI, plot, site | not met | #379; M0.9c: `hpr sim`'s apogee has no kind, spread or trust note |
| `principles.md` | 4. Built for the field | CLI, plot, site | not met | #378; M0.9c: no feet in `hpr sim` or the plot |
| `principles.md` | 5. Quiet until it matters | site, CLI, plot | not met | #383, #385; M0.9d: search hits in the caution fill; signal-like figure colors |
| `principles.md` | 6. One sweep | site, banners, plot, CLI | met | reviewed at `f45454f`: one gradient, the 4 px strip; the banners' cover lockup; none in the plot or CLI |
| `principles.md` | 7. Numbered like parts | site, CLI, exports, plot, README | not met | #380, #387; M0.9c, M0.9e: the plot's version is invisible; the README's status word |
| `principles.md` | 8. Native where it counts | CLI, site | not met | #377; M0.9c: diagnostics on stdout |
| `principles.md` | What FusionSpace doesn't look like | site, README, banners, plot, CLI | not met | #383; M0.9d: mdBook's violet border, radii and soft shadow |
| `foundations.md` | Color | site, CLI, plot, README, banners | not met | #385, #383; M0.9d: non-token colors in a figure, the badges and mdBook's defaults |
| `foundations.md` | Type | site, plot, README, banners | not met | #384, #382, #385; M0.9c, M0.9d: a 118-character measure; text under 12 px; off-scale sizes |
| `foundations.md` | Space and layout | site, plot, CLI | not met | #384, #382, #386; M0.9c, M0.9d, M0.9e: no sheet rail; 20 px gutters; left-aligned numbers; off-scale plot spacing |
| `foundations.md` | Lines and shape | site, plot, README, banners | not met | #383, #385; M0.9d: rounded corners and shadows from mdBook; a figure's strokes |
| `foundations.md` | Motion | site | not met | #383; M0.9d: mdBook's transitions, off the durations and reduced motion |
| `foundations.md` | Icons | site | not met | #383; M0.9d: Font Awesome at 13 and 40 px, not the system's set |
| `foundations.md` | Tokens | site, CLI, plot | met | `xtask/src/site/theme.rs::the_committed_theme_passes`, `crates/hpr-cli/src/plot/tests.rs::the_tokens_are_the_product_systems`, `crates/hpr-cli/tests/style.rs::the_styles_are_the_product_systems` |
| `writing.md` | Voice | site, CLI, README, plot | not met | #386; M0.9e: 60-word warnings with stacked colons; long lead notes |
| `writing.md` | How far to trust it | CLI, plot, site | not met | #379, #384; M0.9c, M0.9d: no three-part note on results; notes in several shapes |
| `writing.md` | On screens | site, CLI, plot | not met | #386; M0.9e: ISO dates in running prose |
| `writing.md` | Errors | CLI | not met | #381; M0.9c: a latitude refused in radians, with no flag and no next step |
| `writing.md` | Notes and warnings in documents | site, README | met | reviewed at `f45454f`: Z535 words only, no callout labels; no hazard note exists yet |
| `writing.md` | Names | site, README, CLI, banners | not met | #386; M0.9e: RSO never spelled out; en dashes in joined names |
| `writing.md` | Mechanics | site, README, CLI, plot | not met | #386; M0.9e: British spellings (`calibre` 446 times on the site and README) |
| `writing.md` | READMEs and docs | README, site | not met | #387; M0.9e: status, install, what doesn't work and version missing |
| `writing.md` | Commit messages | none | n/a | commits are not a surface that ships; the squash commits follow it |
| `review.md` | The FusionSpace test | site, CLI, exports, plot, README | not met | #384, #380, #379, #378, #383, #377; M0.9c, M0.9d: six of the eight answers are no; see the principles' rows |
| `review.md` | Template smells | site, README, plot | not met | #383, #385; M0.9d: a colored side border, soft shadows, default sans in images |
| `review.md` | Before release | site, CLI, exports, plot, README | not met | #385, #380, #379, #378, #386, #377, #381; M0.9c, M0.9d, M0.9e: 0.1's lists: tokens, as-of dates, trust, units, spelling, stderr, errors |
| `review.md` | Sources | none | n/a | the system's own bibliography; it sets no rule |
| `data.md` | Numbers | site, CLI, plot, README | not met | #386, #382, #379; M0.9c, M0.9e: hyphen-minus, ungrouped and over-precise numbers; units that wrap |
| `data.md` | Copying and typing numbers | CLI, exports | not met | #381, #380; M0.9c: typed numbers not trimmed or read with separators; non-ASCII JSON |
| `data.md` | Units for rocketry | CLI, plot, exports, README, site | not met | #378; M0.9c: SI alone in `hpr sim`, the plot and the exports |
| `data.md` | Readouts | CLI | not met | #379; M0.9c: values not marked simulated, with no spread |
| `data.md` | Tables | site, plot, CLI, README | not met | #386; M0.9e: 27 of 333 site tables right-align numbers; code identifiers as heads |
| `data.md` | Charts | plot, site | not met | #382, #379; M0.9c: no spread band, banking or data table; 10 px balloons |
| `data.md` | Maps | none | later | M9.4, the drift on the field; no surface draws a map today |
| `data.md` | Live telemetry | none | later | M13.1, the ground station; nothing live ships |
| `data.md` | Files and exports | exports | not met | #380, #378; M0.9c: no as-of date; CSV units not in brackets; no US units |
| `cli.md` | Output | CLI | not met | #377, #381; M0.9c: diagnostics on stdout; no progress; plain table headers |
| `cli.md` | Color | CLI | not met | #381; M0.9c: the Heading and Literal roles unused |
| `cli.md` | Help | CLI | not met | #381; M0.9c: the link before Usage; no examples |
| `cli.md` | Errors | CLI | not met | #381; M0.9c: errors that don't name the flag, the unit typed or a next step |
| `cli.md` | Exit codes | CLI | met | `crates/hpr-cli/src/lib.rs::exit_codes_are_the_documented_ones`, `crates/hpr-cli/tests/cli.rs::every_planned_command_refuses_with_its_milestone` |
| `cli.md` | Interaction and config | CLI | met | reviewed at `f45454f`: no prompts or config file; HPR_OFFLINE and HPR_CACHE_DIR below the flags |
| `cli.md` | The banner | CLI | met | reviewed at `f45454f`: allowed, not required; ADR-164 §6 keeps the brand's braille lockup out |
| `cli.md` | Drop-in styles | CLI | met | `crates/hpr-cli/tests/style.rs::the_styles_are_the_product_systems` |
| `web.md` | Using it in a project | site | not met | #383; M0.9d: a hand-written fonts.css; one theme-color; mdBook's palettes and icons |
| `web.md` | Page anatomy | site | not met | #384; M0.9d: no title block, sheets, intro or lockup header |
| `web.md` | Site patterns | site | not met | #383; M0.9d: code blocks on mdBook's background, not the surface |
| `web.md` | Components | site | not met | #384; M0.9d: notes as blockquotes, without the signal-word strip |
| `web.md` | Theme | site | not met | #383; M0.9d: no Paper and Void theme-color, no forced-colors rule |
| `web.md` | Accessibility | site | not met | #383, #382; M0.9c, M0.9d: default focus ring, two h1, no contrast check; the plot without a table |
| `web.md` | Progressive web apps | none | later | M9.3, the web PWA |
| `web.md` | Notifications | none | later | M9.4, the pad-day app |
| `web.md` | Performance | site | not met | #383; M0.9d: no metric-matched fallback faces |
| `web.md` | Print | site | not met | #383; M0.9d: dark print colors, no link addresses, rows that split |
| `web.md` | Documentation sites | site | not met | #383, #384; M0.9d: code colors outside the theme; pages that don't open with trust |
| `web.md` | fusionspace.co | none | n/a | the company's own site, not built from this repository |
| `desktop.md` | Choosing a toolkit | none | later | M9.0, the UI architecture decision |
| `desktop.md` | What the platform owns | none | later | M9.1, the desktop app |
| `desktop.md` | What FusionSpace owns | none | later | M9.1, the desktop app |
| `desktop.md` | Dense engineering views | none | later | M9.1, the desktop app |
| `desktop.md` | Documents | none | later | M9.1, the desktop app |
| `desktop.md` | Hardware: serial, USB and radios | none | later | M9.1, the desktop app |
| `desktop.md` | Icons and packaging | none | later | M9.1, the desktop app |
| `desktop.md` | Updates and signing | none | later | M9.1, the desktop app |
| `desktop.md` | Accessibility | none | later | M9.1, the desktop app |
| `desktop.md` | Checklist | none | later | M9.1, the desktop app |
| `mobile.md` | What the platform owns | none | later | M9.4, the mobile app |
| `mobile.md` | What FusionSpace owns | none | later | M9.4, the mobile app |
| `mobile.md` | Screens | none | later | M9.4, the mobile app |
| `mobile.md` | Field use | none | later | M9.4, the mobile app |
| `mobile.md` | Bluetooth devices | none | later | M9.4, the mobile app |
| `mobile.md` | Glanceable surfaces | none | later | M9.4, the mobile app |
| `mobile.md` | App icons | none | later | M9.4, the mobile app |
| `mobile.md` | Real screens | none | later | M9.4, the mobile app |
| `mobile.md` | Checklist | none | later | M9.4, the mobile app |
| `watch.md` | What a watch never does | none | later | M9.4, which decides the watch link (ADR-191); a watch app follows it |
| `watch.md` | What the platform owns | none | later | M9.4, which decides the watch link (ADR-191); a watch app follows it |
| `watch.md` | What FusionSpace owns | none | later | M9.4, which decides the watch link (ADR-191); a watch app follows it |
| `watch.md` | Screens | none | later | M9.4, which decides the watch link (ADR-191); a watch app follows it |
| `watch.md` | Always On and ambient | none | later | M9.4, which decides the watch link (ADR-191); a watch app follows it |
| `watch.md` | Complications, Smart Stack and tiles | none | later | M9.4, which decides the watch link (ADR-191); a watch app follows it |
| `watch.md` | The wrist language | none | later | M9.4, which decides the watch link (ADR-191); a watch app follows it |
| `watch.md` | Staying alive and connected | none | later | M9.4, which decides the watch link (ADR-191); a watch app follows it |
| `watch.md` | Real screens | none | later | M9.4, which decides the watch link (ADR-191); a watch app follows it |
| `watch.md` | Checklist | none | later | M9.4, which decides the watch link (ADR-191); a watch app follows it |
| `embedded.md` | Arming and safety | none | later | M13.3, the flight computer's logic |
| `embedded.md` | Energetics behavior | none | later | M13.3, the flight computer's logic |
| `embedded.md` | Channels: three states, and UNFIRED | none | later | M13.3, the flight computer's logic |
| `embedded.md` | Beeps | none | later | M13.3, the flight computer's logic |
| `embedded.md` | Lights | none | later | M13.4, the avionics hardware |
| `embedded.md` | Screens | none | later | M13.4, the avionics hardware |
| `embedded.md` | Buttons and switches | none | later | M13.4, the avionics hardware |
| `embedded.md` | Ground stations and pad boxes | none | later | M13.1, the ground station |
| `embedded.md` | Serial and USB output | none | later | M13.3, the flight computer's logic |
| `hardware.md` | Designations | none | later | M13.4, the avionics hardware |
| `hardware.md` | PCBs | none | later | M13.4, the avionics hardware |
| `hardware.md` | Enclosures | none | later | M13.4, the avionics hardware |
| `hardware.md` | Cables and connectors | none | later | M13.4, the avionics hardware |
| `hardware.md` | Drawings | none | later | M8.3b, the parts' drawings, then M13.4's |
| `rockets.md` | Color | none | later | M12.1, the parachute gores (canopy colors) |
| `rockets.md` | Roll pattern | none | n/a | paint on a physical airframe, for reading roll on video; the project makes no airframes |
| `rockets.md` | Markings | none | later | M0.7a, whose guides' figures are the first to draw CG and CP marks |
| `rockets.md` | Flight card | none | later | M9.4, whose pad-day mode prints the flight card (ADR-163) |
| `rockets.md` | Pad checklist | none | later | M6.8, the field kit's checklists |
