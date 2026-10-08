# ADR-164: hpr-sim follows the FusionSpace product system, as `FS · SW · TOOL 005` (2026-10-04)

- **Status:** accepted; §1's pin and §2's designation (`FS-ACHERNAR · SW · TOOL 001`) amended by [ADR-198, project Eridanus](0198-the-2026-10-07-project-eridanus.md)
- **Summary:** hpr-sim adopts the FusionSpace product system (`nrdptel/fusionspace-design`, `product/`, Rev A, pinned at `32770a4`) for its command line, documentation site, plots, exports, writing and every later app and device. It takes the designation `FS · SW · TOOL 005`. Apache-2.0 files from the system are copied in with their license lines; brand files only as README banners, outside the repository's license. Milestone M0.6 makes the existing surfaces comply, after M4.5 and before release 0.1; later milestones and every release checklist take the system's rules and checklists. The CLI keeps SI first with US units in brackets, and carries no brand banner

**Context.** On 2026-10-04 Neer decided hpr-sim would take in and follow his FusionSpace design
system, taking the time needed to fold it in. His public repository
[`fusionspace-design`](https://github.com/nrdptel/fusionspace-design) holds the FusionSpace
brand (the four-cone mark, colors, lockups) and, under `product/`, a product system: rules for
how every FusionSpace site, tool, CLI, app, device, board and rocket looks, reads and behaves.
Rev A is dated October 4, 2026 and was approved in review on October 5 (UTC). It is built
around hpr-sim in places:

- `cli.md` names `hpr` as the model command-line tool, and its exit codes (0 done, 1 refused or
  failed, 2 usage, 3 not available yet) and `--json` error documents are the ones `hpr` already
  has.
- `web.md` ships an mdBook theme because hpr-sim's documentation uses mdBook, and `writing.md`
  calls hpr-sim's README and Start here page the model for project docs.
- `desktop.md` lists hpr-sim's simulator, design tree, 3D replay and Monte Carlo runs first among
  its desktop apps, and makes Tauri 2 the default shell.
- `data.md` shows a readout "± 227 ft (1σ) · hpr-sim 0.9" and a flight chart with hpr-sim's
  prediction dashed beside the measured trace.

Its eight principles: drawn, not decorated; show the working; say how far to trust it; built for the
field; quiet until it matters; one sweep (the brand's gradient once per view); numbered like parts;
native where it counts. hpr-sim's own rules already hold the second and third
([rule 1](../../CONTRIBUTING.md#1-correct-physics-first), the guardrails' "no verdicts",
`docs/writing.md`).

What hpr-sim does today, against the system, measured on `main` at `920dec9`:

| Surface | Today | The system asks |
|---|---|---|
| Docs site | mdBook's default themes and fonts | the Rev A mdBook theme, Archivo and Cascadia Mono served from the site, the gradient strip, a title block |
| CLI color | none: plain text everywhere | ANSI roles only (`error:` bold red, `warning:` bold yellow, `help:` bold blue); `--color auto\|always\|never`; `NO_COLOR`, `FORCE_COLOR`, `CLICOLOR_FORCE` in a set order, per stream |
| CLI errors | `error: …` then the hint in the same sentence | the hint on its own `help:` line, last |
| `hpr --version` | `hpr 0.1.0` | the designation beside the version |
| Exports | design files record `tool` and `tool_version`; the recordings, the plot and `--json` documents name neither; nothing names the designation | each carries tool, version and designation, like a title block |
| `hpr sim --plot` | its own palette (`#2a78d6`, `#eb7a2f`), sans-serif; altitude solid, speed and acceleration dashed `6 4`; events numbered with a list | token colors only; every simulated series dashed in the predicted color; events dotted with numbered balloons; ground a chain line; axis titles with units; no legend box; a caption and a text summary |
| Spelling | British: "centre" 2,477 times, "metre" 540, "catalogue" 230, "licence" 141, "colour" 34, in prose, comments and names (`centre_of_pressure_m`) | US English in prose, screens and code comments |
| Dashes | 953 em dashes | none; an en dash only in ranges |

The last two rows count case-insensitive matches with `git grep -I -o -i <word> -- docs README.md
crates xtask schema python scripts`, this record left out; "centre" is 1,157 of them in `docs/`,
1,097 in the crates and 90 in the JSON schemas.

**Decision.**

1. **Adopt the system.** hpr-sim follows `product/` of fusionspace-design. The revision it follows
   is pinned in `validation/refs.lock.toml` (`32770a4`, Rev A, moved to `f45454f` by M10.1d7; *Pin
   moves*, below) and fetched into the gitignored `refs/fusionspace-design` by
   `cargo xtask refs fetch`. A later revision is adopted by a PR that moves the pin and lists what
   changed. The project's [rules](../../CONTRIBUTING.md#the-rules) come first; they agree with the
   system, which asks for the same honesty. The system's `writing.md` extends
   [`docs/writing.md`](../writing.md); the latter keeps its own rules (Diátaxis page kinds, sentence
   lengths).
2. **The designation.** hpr-sim as a whole is `FS · SW · TOOL 005`, the next number after the
   four tools in the system's register (Motor Finder 001, Charge 002, Window 003, Muster 004),
   in the grammar of `writing.md`. The suite's products ([ADR-162](0162-the-suite-and-its-releases.md))
   are parts of it and take no designation of their own until Neer names them (ADR-162 §6 keeps
   product names his).
3. **What is copied in, and under which terms.**
   - The system's code-like files (tokens, `fusionspace.css`, the mdBook theme, the CLI styles) are
     Apache-2.0. They are copied in unchanged with their SPDX line and the revision, each with a
     row under *Bundled* in `THIRD-PARTY-NOTICES.md`. They stay Apache-2.0 alone inside this
     MIT-or-Apache-2.0 project, as the bundled `.orc` files do.
   - The fonts (Archivo, Cascadia Mono; SIL OFL 1.1) are bundled with their license texts beside
     them and served from the site. The system's `fonts.css` carries no license line and is not on
     its Apache-2.0 list, so hpr writes its own `@font-face` rules instead of copying it. No font or style comes from a third party at run time, which
     `web.md` asks for and offline-first needs.
   - Brand files are the owner's, all rights reserved. Only `.github/brand/` holds any: the README
     banners and the social preview made by the system's `tools/build/project.py`, and its
     `project.json`. The README and `THIRD-PARTY-NOTICES.md` say these are outside the
     repository's license. Nothing from `logo/` or `kit/` is compiled into a crate, a binary, a
     wheel or the site.
4. **M0.6, the product system, after M4.5 and before M1.14a.** Release 0.1 ships the CLI and the
   docs; restyling them before the first publish costs least, and nothing in M0.6 touches physics.
   It changes surfaces that exist and adds none, so ADR-144 §6's rule on new surfaces doesn't
   apply. Five increments, each with its own *done when* in the roadmap: a, the docs site; b, the
   CLI and exports; c, the plot; d, US spelling and no em dashes in prose, CLI text and comments,
   with a check; e, public names in US spelling before the first publish, with old design files
   still read.
5. **Rules that bind later milestones.**
   - **Every release** (ADR-162 §2's checklist): `review.md`'s *Before release* lists hold for the
     surfaces the release ships: *Every product* and *Command line* from 0.1; *Web and PWA* from
     0.5; the desktop and mobile checklists with the apps.
   - **M9.0** starts from `desktop.md`'s default, Tauri 2, with the content in the system web
     view and `fusionspace.css`. An all-Rust toolkit is measured against it as the milestone
     says, and replaces it only with a measured reason written into the ADR, since `desktop.md`
     keeps egui to internal tools for want of native text scaling and contrast themes.
   - **M9.1 to M9.4** follow `web.md`, `desktop.md`, `mobile.md` and the tokens; the app's units
     default to US, as `data.md` asks; the pad-day flight card is `rockets.md`'s.
   - **M6.7** (ejection charges) uses `writing.md`'s "How far to trust it" shape and ANSI Z535
     signal words, and says "ground test first".
   - **M13** (avionics) follows `embedded.md` (arming, three-state channels, UNFIRED first, the
     beeps) and `hardware.md`.
6. **Not adopted.**
   - **US units first in the CLI.** `data.md` asks tools to default to the units US flyers expect.
     `hpr` prints SI with US units in brackets (`390.1 m (1280 ft)`), which `cli.md`'s own
     example shows; it stays. The apps default to US units (5).
   - **The braille banner** that `cli.md` allows on `--version`. It is a brand file, all rights
     reserved, and a binary distributed under MIT or Apache-2.0 should not carry it.

**Alternatives considered.**

- **Reference the system without changing anything yet.** Rejected: the CLI and the docs ship in
  release 0.1, and changing their look and their JSON names after the first publish costs users.
- **Run M0.6 first.** Rejected: M4.5 is mid-way (g3, g4 left) and changes `hpr sim`'s output,
  which b and c then restyle once instead of twice.
- **Keep British spelling in names.** Rejected for the public API: nothing is published yet, and
  `writing.md` asks for US English in code comments, which would then sit beside British names.
  Design files with the old keys keep reading (e).
- **Link the fonts from Google Fonts.** Rejected: the site would load a third party's files, and
  break offline.

**Consequences.**

- `ROADMAP.md`'s budget rises from 28,000 bytes ([ADR-163](0163-the-second-pass-products-and-priorities.md)
  §6) to 31,000: M0.6's entry and the notes on M9.0 to M9.4 take it to 29,926, and the rest leaves
  room to split an increment before something is archived. Archiving M0.6 frees the room,
  but nothing lowers the budget again by itself. No other budget changes.
- A new `refs/` checkout, `fusionspace-design`; it holds no `.ork`, `.rkt` or `.eng` file, so the
  `.ork` survey is unchanged.
- The README opens with the banner and gains a *Design* section; "Fusion Space" becomes
  "FusionSpace" where hpr-sim wrote it as two words.
- For the maintainer, no action if fine: add `FS · SW · TOOL 005` to the system's register, and set
  `.github/brand/social-preview-dark.png` as the repository's social preview.

**Pin moves.**

- **`32770a4` (2026-10-04) to `f45454f` (2026-10-07), M10.1d7 ([ADR-198](0198-the-2026-10-07-project-eridanus.md)).**
  This covers 21 commits, `ab4d769` to `f45454f`.
  - Nothing hpr copies in changed: `product/cli/fs_style.rs`, the mdBook theme's CSS, the five
    WOFF2 fonts and the two OFL texts are byte-identical. Nor did `cli.md`, `web.md`, `desktop.md`,
    `foundations.md`, or the CSS and JSON tokens.
  - `writing.md`, *Names*: a project takes a constellation (`FS-<CON>`), and its products take its
    stars (`FS-<STAR>`). The internal name never changes and the external name leads. The old
    web tools keep their `FS · SW · TOOL 00n`. So hpr's designation becomes
    `FS-ACHERNAR · SW · TOOL 001`.
  - `data.md`: a number and its unit are separated by a non-breaking space (U+00A0), which
    [`docs/writing.md`](../writing.md) now takes for the site and SVG figures.
  - `product/mobile.md` gains *Screens* and *Real screens* sections; the Android icons' tint moves
    to `?android:attr/colorControlNormal`; a Roboto font (OFL) joins `product/mobile/fonts/`.
    hpr copies none of these.
  - `product/README.md` gains a `watch.md` row and `mobile/` and `watch/` folders. Their Swift and
    Kotlin parts are now under Apache-2.0. `review.md` gains phone accessibility and large-text
    items, and an Apple Watch and Wear OS section. There is a new `watch.md` and SF Symbols. These
    apply at M9.4 and later (ADR-198 §8).
  - `product/tokens/FusionSpaceColors.swift` changed: watchOS always uses the dark roles, and there
    is a new `labelStrong()` font. hpr copies no Swift tokens, so ADR-198's context ("nor the
    tokens") holds only for the CSS and JSON ones.
  - Tooling: `tools/build/project.py` gains `--star` and `--name`, and the banners are rebuilt
    with them. The new `kit_clip.py` clipping check passes on the banners' SVGs and joins M0.7a
    for the site (ADR-198 §7). `kit_mobile.py`, `kit_watch.py`, `kit_symbols.py`, `kit_devices.py`
    and Callsign (`tools/callsign/`) are new too. The livery and the specimen previews were
    re-rendered.
