# Design conformance: every section of the product system, against what ships

This note lists where FusionSpace HPR's site, command line, exports and plot do and don't follow
the FusionSpace product system: the shared rules for how every FusionSpace product looks, reads
and behaves, in the 14 files under `product/` in `nrdptel/fusionspace-design`, which
[ADR-164](../decisions/0164-the-fusionspace-product-system.md) adopted. It is for anyone checking
how finished the user-facing parts are. The rules govern presentation (colors, type, wording,
units, errors, provenance); none of them changes a number the simulator computes.

It has one row for every `##` section of those files ([ADR-205](../decisions/0205-the-2026-10-08-one-name-one-design.md)
§2, [ADR-208](../decisions/0208-the-design-audit.md)), so a rule nobody re-read can't be missed.
Of the 64 sections that apply to something shipping today, 11 are met, 46 are not, 3 wait for a
later surface and 4 govern nothing the project ships; the 48 sections for apps, watches,
firmware, hardware and airframes wait for the milestones that build them. How far to trust it:
a row held by a test is checked on every change; a row "reviewed at" the commit was read against
the surface once, at that commit, and can drift until the next review.

Pinned commit: `f45454f44669cc049222f2ab19eb648796e56a52`, as `validation/refs.lock.toml` pins it.

- **Applies to:** the surfaces that ship today:
  - the docs **site**;
  - the **CLI** (`hpr`);
  - the files it **exports**, and their stamps;
  - the **plot** that `hpr sim --plot` draws;
  - the **README**s;
  - the **banners**.

  It says `none` when the section governs none of them.
- **Status:**
  - **met:** held by a named test (`file.rs::test`), or "reviewed at" the commit, with what was read;
  - **not met:** with its GitHub issues and the milestones that fix them. M0.9c fixes the command
    line, the exports and the plot; M0.9d the site's look; M0.9e the words;
  - **later:** a surface that doesn't ship yet, with the milestone that brings it;
  - **n/a:** a section that governs nothing the project ships or will ship.
- **The checks:** `xtask/src/conformance.rs` (run by `cargo test -p xtask`) fails when:
  - a section is missing from [`design-sections.txt`](design-sections.txt)'s list, or the list
    loses a section;
  - the commit isn't the lock's;
  - a named test isn't a live `#[test]`;
  - a milestone isn't open.

  Where `refs/` is checked out, it also compares that list with the pinned files' headings.
  Moving the pin fails the check until every row has been audited again.

| File | Section | Applies to | Status | Held by |
|---|---|---|---|---|
| `README.md` | Read in this order | none | n/a | a reading order for the system's own files, each audited in its own rows |
| `README.md` | The idea in one paragraph | site, CLI, exports, plot | not met | #384; M0.9d: a title block on one page of 62 |
| `README.md` | Files | site, CLI, plot | met | `xtask/src/site/theme.rs::the_committed_theme_passes`, `crates/hpr-cli/tests/style.rs::the_styles_are_the_product_systems`: copied files byte for byte at the pin |
| `README.md` | Pointing a project here | README, site | not met | #387, #385; M0.9d, M0.9e: the README's adaptation drops foundations; images outside the tokens |
| `README.md` | Status | CLI, site | not met | #386; M0.9e: Rev A's US spelling: `calibres` in `hpr sim` and the site |
| `README.md` | License | site, CLI, README, banners | met | `xtask/src/site/theme.rs::the_committed_theme_passes` (SPDX lines, font licenses); reviewed at `f45454f`: the notices and the brand files' terms |
| `principles.md` | 1. Drawn, not decorated | site, plot | not met | #384, #385, #383, #382; M0.9c, M0.9d: no title blocks or sheet numbers; a figure's line types; soft shadows |
| `principles.md` | 2. Show the working | site, CLI, exports, plot | not met | #384; M0.9d: the text and every export carry the motor catalog's as-of date with the design, configuration, tool and version (ADR-214; `crates/hpr-cli/tests/provenance.rs::every_export_carries_the_catalogs_date_and_how_far_to_trust_it`, `crates/hpr-cli/tests/provenance.rs::the_text_dates_a_motor_from_the_bundled_catalog`, `crates/hpr-cli/tests/provenance.rs::the_maps_carry_the_catalogs_date_and_how_far_to_trust_it`, `crates/hpr-cli/tests/provenance.rs::the_parquet_file_and_the_plot_carry_the_catalogs_date`), as does the plot (`crates/hpr-cli/src/plot/tests.rs::the_figure_dates_the_motor_catalog_above_its_title_block_line`); the site's pages, whose notes #384 rebuilds, haven't been re-read against it since |
| `principles.md` | 3. Say how far to trust it | CLI, plot, site | not met | #386, #401; M0.9e, M2.8: every command's result someone might fly on ends with the note, its kind in `--json` (ADR-211, ADR-213; `crates/hpr-cli/tests/trust.rs::sim_says_its_figures_are_simulated_and_ends_with_how_far_to_trust_them`, `crates/hpr-cli/tests/trust.rs::analyze_says_its_readings_are_measured_and_how_far_to_trust_them`, `crates/hpr-cli/tests/trust.rs::motors_show_says_where_its_figures_come_from`, `crates/hpr-cli/tests/weather.rs::every_source_ends_with_how_far_to_trust_it`), as does every export (ADR-214; `crates/hpr-cli/tests/provenance.rs::every_export_carries_the_catalogs_date_and_how_far_to_trust_it`, `crates/hpr-cli/tests/provenance.rs::the_maps_carry_the_catalogs_date_and_how_far_to_trust_it`, `crates/hpr-cli/tests/provenance.rs::the_parquet_file_and_the_plot_carry_the_catalogs_date`), but a barometric apogee prints to a tenth of a meter and no readout carries a spread of its own |
| `principles.md` | 4. Built for the field | CLI, plot, site | not met | #383; M0.9d: no field theme and no print styles on the site. Units on screen hold under ADR-164 §6 and ADR-210 (feet in brackets, no switch): `crates/hpr-cli/tests/units.rs::sim_gives_us_units_in_brackets`, `crates/hpr-cli/tests/units.rs::the_plot_gives_both_units`; offline: the core needs no network |
| `principles.md` | 5. Quiet until it matters | site, CLI, plot | not met | #383, #385; M0.9d: search hits in the caution fill; signal-like figure colors |
| `principles.md` | 6. One sweep | site, banners, plot, CLI | not met | #383; M0.9d: focus and selection are mdBook's, not Ion; one gradient, the strip, holds |
| `principles.md` | 7. Numbered like parts | site, CLI, exports, plot, README | not met | #387; M0.9e: the README's status word; the plot's last line names its version (ADR-214; `crates/hpr-cli/src/plot/tests.rs::the_figure_names_its_tool_and_version_where_a_reader_sees_them`) |
| `principles.md` | 8. Native where it counts | CLI, site | met | `crates/hpr-cli/tests/streams.rs::sim_writes_its_diagnostics_to_stderr`, `crates/hpr-cli/tests/streams.rs::mc_writes_its_diagnostics_to_stderr`, `crates/hpr-cli/tests/streams.rs::json_output_is_one_document_carrying_the_warnings` and the file's test for each other command: results on stdout, `warning:`, `note:` and `help:` lines on stderr, as a Unix tool's |
| `principles.md` | What FusionSpace doesn't look like | site, README, banners, plot, CLI | not met | #383; M0.9d: mdBook's violet border, radii and soft shadow |
| `foundations.md` | Color | site, CLI, plot, README, banners | not met | #385, #383; M0.9d: non-token colors in a figure, the badges and mdBook's defaults |
| `foundations.md` | Type | site, plot, README, banners | not met | #384, #385; M0.9d: a 118-character measure; text under 12 px and off-scale sizes on the site. The plot's text is 12 px, its title the `subtitle` token (`crates/hpr-cli/src/plot/tests.rs::the_figure_sets_its_type_on_the_scale`) |
| `foundations.md` | Space and layout | site, plot, CLI | not met | #384, #386; M0.9d, M0.9e: no sheet rail; 20 px gutters; left-aligned numbers. The plot's title, caption, line, balloon, table and indent steps are on the 4 px scale, as drawn (`crates/hpr-cli/src/plot/tests.rs::the_figure_sets_its_type_on_the_scale`) |
| `foundations.md` | Lines and shape | site, plot, README, banners | not met | #383, #385; M0.9d: rounded corners and shadows from mdBook; a figure's strokes |
| `foundations.md` | Motion | site | not met | #383; M0.9d: mdBook's transitions, off the durations and reduced motion |
| `foundations.md` | Icons | site | not met | #383; M0.9d: Font Awesome at 13 and 40 px, not the system's set |
| `foundations.md` | Tokens | site, CLI, plot | met | `xtask/src/site/theme.rs::the_committed_theme_passes`, `crates/hpr-cli/src/plot/tests.rs::the_tokens_are_the_product_systems`, `crates/hpr-cli/tests/style.rs::the_styles_are_the_product_systems` |
| `writing.md` | Voice | site, CLI, README, plot | not met | #386; M0.9e: 60-word warnings with stacked colons; long lead notes |
| `writing.md` | How far to trust it | CLI, plot, site | not met | #384; M0.9d: one note shape on the site and on every command's result someone might fly on (ADR-211, ADR-213; `crates/hpr-cli/tests/trust.rs::sim_says_its_figures_are_simulated_and_ends_with_how_far_to_trust_them`, `crates/hpr-cli/tests/trust.rs::analyze_says_its_readings_are_measured_and_how_far_to_trust_them`, `crates/hpr-cli/tests/trust.rs::motors_show_says_where_its_figures_come_from`, `crates/hpr-cli/tests/weather.rs::every_source_ends_with_how_far_to_trust_it`), but the site's notes aren't `web.md`'s panels |
| `writing.md` | On screens | site, CLI, plot | not met | #386; M0.9e: ISO dates in running prose |
| `writing.md` | Errors | CLI | not met | #405; M0.9c9: a refusal from inside a flight, such as a Mach past the aerodynamics' range, keeps the library's words and gives no next step. A refused option or path is named as typed, with what to do (since ADR-215; `crates/hpr-cli/tests/cli.rs::launch_options_are_refused_by_name_and_unit`, `crates/hpr-cli/tests/cli.rs::a_missing_file_says_what_to_do`) |
| `writing.md` | Notes and warnings in documents | site, README | met | reviewed at `f45454f`: Z535 words only, no callout labels; no hazard note exists yet |
| `writing.md` | Names | site, README, CLI, banners | not met | #386; M0.9e: RSO never spelled out; en dashes in joined names |
| `writing.md` | Mechanics | site, README, CLI, plot | not met | #386; M0.9e: British spellings (`calibre` 446 times on the site and README) |
| `writing.md` | READMEs and docs | README, site | not met | #387; M0.9e: status, install, what doesn't work and version missing |
| `writing.md` | Commit messages | none | n/a | commit messages are not a surface that ships |
| `review.md` | The FusionSpace test | site, CLI, exports, plot, README | not met | #384, #387, #383, #401; M0.9d, M0.9e, M2.8: five of the eight answers are no (*trust stated* asks a spread); see the principles' rows |
| `review.md` | Template smells | site, README, plot | not met | #383, #385; M0.9d: a colored side border, soft shadows, default sans in images |
| `review.md` | Before release | site, CLI, exports, plot, README | not met | #385, #386, #381; M0.9c, M0.9d, M0.9e: 6 of the 11 items 0.1 must pass fail (ADR-208 §6 counted 8; results on stdout and everything else on stderr passes, and since #403 the trust note: on every command's result, ADR-213, and on every export and the plot, ADR-214); every dataset's "as of" and the title block still fail on the site (#384) |
| `review.md` | Sources | none | n/a | the system's own bibliography; it sets no rule |
| `data.md` | Numbers | site, CLI, plot, README | not met | #386; M0.9e: hyphen-minus, ungrouped and over-precise numbers; units that wrap outside the plot. The plot's caption, notes and tooltips join each number to its unit with a no-break space (`crates/hpr-cli/src/plot/tests.rs::a_number_keeps_its_unit_on_its_line`) |
| `data.md` | Copying and typing numbers | CLI, exports | met | `crates/hpr-cli/tests/cli.rs::every_numeric_option_reads_typed_numbers`, `crates/hpr-cli/tests/cli.rs::typed_numbers_reach_the_flight`: trimmed, `1,280` read and shown as read, `3,9` asked about (ADR-215); `crates/hpr-cli/tests/provenance.rs::every_json_document_is_ascii`: JSON is ASCII (ADR-214) |
| `data.md` | Units for rocketry | CLI, plot, exports, README, site | not met | #400; M0.9c13: SI alone in the site's prose. Held under ADR-164 §6, ADR-210 and ADR-213 (SI first, US in brackets, no `--units`; exports SI) for every command's text (notes and warnings included) and the plot: `crates/hpr-cli/tests/units.rs::sim_gives_us_units_in_brackets`, `crates/hpr-cli/tests/units.rs::sim_gives_the_cg_and_cp_of_its_static_margin`, `crates/hpr-cli/tests/units.rs::sim_notes_give_feet`, `crates/hpr-cli/tests/units.rs::mc_gives_feet_beside_meters`, `crates/hpr-cli/tests/units.rs::mc_landing_meters_carry_a_tenth`, `crates/hpr-cli/tests/units.rs::motors_show_reads_the_designation`, `crates/hpr-cli/tests/units.rs::the_plot_gives_both_units`, `crates/hpr-cli/tests/units.rs::motors_list_gives_inches`, `crates/hpr-cli/tests/units.rs::motors_search_gives_inches`, `crates/hpr-cli/tests/weather.rs::the_levels_give_us_units_in_brackets`, `crates/hpr-cli/tests/cli.rs::motors_fetch_reads_a_cached_motor`, `crates/hpr-cli/src/analyze.rs::a_top_acceleration_gives_g` |
| `data.md` | Readouts | CLI | not met | #401; M2.8: `hpr sim` and `hpr mc` say once, above their figures, that they are simulated, and give the spread in the closing note (ADR-211); `hpr weather`, `hpr analyze` and `hpr motors show` name their kind in theirs (ADR-213); no readout carries a spread of its own |
| `data.md` | Tables | site, plot, CLI, README | not met | #386; M0.9e: 27 of 333 site tables right-align numbers; code identifiers as heads |
| `data.md` | Charts | plot, site | not met | #382, #401; M0.9c12, M2.8: no spread band (ADR-211 §5), banking or data table; balloons collide instead of stepping along a leader. 12 px labels, numbers held to their units (`crates/hpr-cli/src/plot/tests.rs::the_figure_sets_its_type_on_the_scale`, `crates/hpr-cli/src/plot/tests.rs::a_number_keeps_its_unit_on_its_line`) |
| `data.md` | Maps | CLI | not met | #381; M0.9c9: `hpr mc`'s landing-ellipse heading gives no T or M; drawn maps wait for M9.4 |
| `data.md` | Live telemetry | none | later | M13.1, the ground station; nothing live ships |
| `data.md` | Files and exports | exports | met | `crates/hpr-cli/tests/provenance.rs::a_csv_header_gives_each_unit_in_brackets`, `crates/hpr-cli/tests/provenance.rs::every_export_carries_the_catalogs_date_and_how_far_to_trust_it`, `crates/hpr-cli/tests/style.rs::every_file_hpr_writes_names_the_tool`: units in brackets in the CSV header, the source, tool, version and the catalog's as-of date in a sidecar (ADR-214). The US option waits for the app's units control (M9.1), by ADR-210 |
| `cli.md` | Output | CLI | not met | #381; M0.9c9: no progress; plain table headers (diagnostics on stderr since #377) |
| `cli.md` | Color | CLI | not met | #381; M0.9c9: the Heading and Literal roles unused |
| `cli.md` | Help | CLI | not met | #381; M0.9c9: the link before Usage; no examples |
| `cli.md` | Errors | CLI | not met | #405; M0.9c9: a refusal from inside a flight names no option and has no `help:` line. A refused option or path is named as typed, with a `help:` line and the closest file or motor (since ADR-215; `crates/hpr-cli/tests/cli.rs::launch_options_are_refused_by_name_and_unit`, `crates/hpr-cli/tests/cli.rs::a_missing_file_says_what_to_do`, `crates/hpr-cli/tests/cli.rs::motors_show_suggests_what_answers_the_name`) |
| `cli.md` | Exit codes | CLI | met | `crates/hpr-cli/src/lib.rs::exit_codes_are_the_documented_ones`, `crates/hpr-cli/tests/cli.rs::every_planned_command_refuses_with_its_milestone` |
| `cli.md` | Interaction and config | CLI | met | reviewed at `f45454f`: it never asks for input and has no config file; HPR_OFFLINE and HPR_CACHE_DIR below the flags |
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
