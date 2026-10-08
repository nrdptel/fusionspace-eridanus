# ADR-198: Neer's 2026-10-07 project name: Eridanus, its stars, and the design system's catch-up (2026-10-07)

- **Status:** accepted; amends [ADR-164, the FusionSpace product system][adr-164] §1 (the pin moves) and §2 (the designation), and [ADR-162, the suite and its releases][adr-162] §1 ("the names stay `hpr-sim` and `hpr`"); §3's external names and §4's stamp name amended by [ADR-199, FusionSpace HPR](0199-the-2026-10-07-fusionspace-hpr.md)
- **Summary:** the project is Eridanus, `FS-ERI`, and the repository becomes `nrdptel/fusionspace-eridanus` when Neer renames it. Each product takes one of Eridanus's stars as its internal name: the simulator is Achernar (`FS-ACHERNAR`), the app Zaurak, the flight analyzer Angetenar, avionics Sceptrum, the competition kit Acamar, the motor designer Rana and the recovery designer Cursa. External names don't change: the program is still `hpr-sim`, the command `hpr`. The designation `FS · SW · TOOL 005`, which the design system lists as retired, becomes `FS-ACHERNAR · SW · TOOL 001`, and files stamped with the old one still read. GitHub doesn't redirect a project site after a rename, so the rename comes before release 0.1 publishes anything. M10.1d7 and M10.1d8 do the code side, and catch up with the 21 design-system commits since the pin.

[adr-162]: 0162-the-suite-and-its-releases.md
[adr-164]: 0164-the-fusionspace-product-system.md
[adr-191]: 0191-the-2026-10-06-ideas-web-tools-watches-repo-layout-hardware.md
[adr-195]: 0195-the-2026-10-07-guides-rocketry-explained.md

**Context.** On October 7, 2026, Neer decided to take in the design system's updates since the pin and plan them into
the project, and, first, to name the overall project after a constellation with many named stars, each product taking
one of its stars; the repository would be renamed `fusionspace-<constellation>` ([VISION](../VISION.md), that date).
Neer named the project Eridanus (20 named stars).

- **The design system's naming rule** (`product/writing.md`, Names, added on 2026-10-06): a project takes one of the
  88 constellations, coded `FS-<ABBR>`. Each product takes one of its IAU-approved stars as an internal name that
  never changes, coded `FS-<STAR>`. The external name is what customers read; it can be the star's or another, never
  another star's. Where both appear, the external name leads: `Vega · FS-VEGA · EMB · FIRMWARE 002`. Designations
  read `<code> · [<tag> ·] <kind> <number>`.
- **Eridanus's named stars** come from the IAU's list as the design repository's Callsign tool carries it: 14 of
  magnitude 5 or brighter (Achernar 0.45, Cursa, Acamar, Zaurak, Rana, Ran, Theemin, Azha, Beemim, Sceptrum, Beid,
  Keid, Angetenar, Zibal), and 6 faint exoplanet hosts named for rivers (Ayeyarwady, Mouhoun, Montuno, Koeia, Tojil,
  Chaophraya).
- **hpr's designation clashes.** ADR-164 §2 took `FS · SW · TOOL 005` as the number after the four web tools. The
  design repository's notes say 005 and 006 retired with Loft and Debrief. Callsign took 007 and then moved to its
  own star. The register entry for 005 that ADR-164 asked the maintainer for was never made.
- **The designation is load-bearing.** `hpr_core::tool::DESIGNATION` feeds `--version`, the `tool` object of
  every `--json` output, the `.hpr` format's metadata, the plot's SVG, the weather profile, and the stamp in every
  exported `.eng`, `.rse`, `.ork`, XML and Parquet file. `hpr_motor::eng` recognises its own stamp by a comment line
  that ends with it, and drops it on reading; the other readers ignore the value. 52 files outside the ADRs spell it
  out, 16 of them generated schemas and bindings.
- **A rename moves every URL except the site.** GitHub redirects issues, pull requests and `git` operations to a
  renamed repository, but "all existing information, with the exception of project site URLs, is automatically
  redirected"; it suggests a custom domain so that "the site's URL isn't impacted by renaming the repository"
  ([GitHub Docs, renaming a repository](https://docs.github.com/en/repositories/creating-and-managing-repositories/renaming-a-repository)).
  Reusing the old name later breaks the redirects. 241 tracked files name `nrdptel/hpr-sim` or
  `nrdptel.github.io/hpr-sim` (not counting the private `hpr-sim-fixtures`, which keeps its name), 208 of them outside
  the ADRs, and nothing has been published: crates.io and PyPI would bake the repository and
  documentation URLs into 0.1.0's metadata.
- **The design repository moved 21 commits past hpr's pin** (`32770a4`, 2026-10-04, to `f45454f`, 2026-10-07). None of
  the files hpr copied in changed (`product/cli/fs_style.rs` and the mdBook theme match), nor did `cli.md`,
  `web.md`, `desktop.md`, `foundations.md` or the tokens. What changed:
  - the naming rule above, and `tools/build/project.py`'s `--star` and `--name`;
  - `data.md`: a non-breaking space between a number and its unit;
  - `mobile.md`, a new `watch.md`, `review.md`'s phone and watch items, SF Symbols, and Swift and Compose parts now
    listed as Apache-2.0;
  - `tools/build/kit_clip.py`, a clipping check: no sideways scroll, no text cut by its box and no text over other
    text at 320 to 1280 px in both themes; every SVG text line inside its canvas and off other text; phone and watch
    captures inside the screen's outline.

**Decision.**

1. **The project is Eridanus, `FS-ERI`.** The repository becomes `nrdptel/fusionspace-eridanus`. Neer renames it ([rule
   7](../../CONTRIBUTING.md#7-releases-are-the-maintainers): repository settings are his), and never reuses the name
   `hpr-sim` for a new repository. Because the design rule names repositories after the internal name, this one is named
   after the project. The repository holds the whole suite (ADR-191 §2), so the project code fits.
2. **Each product takes a star.** The internal names, with the meaning that chose them:

   | Product (ADR-163) | Star | Code | Why |
   |---|---|---|---|
   | The simulator: library, CLI, Python | Achernar | `FS-ACHERNAR` | "the river's end", the brightest; every product runs into it |
   | The app | Zaurak | `FS-ZAURAK` | "the boat": what people ride in |
   | The flight analyzer | Angetenar | `FS-ANGETENAR` | "the bend in the river": where a flight leaves its simulation |
   | Avionics | Sceptrum | `FS-SCEPTRUM` | "the sceptre": the controller on board |
   | The competition kit | Acamar | `FS-ACAMAR` | also "the river's end": the finish line |
   | The motor designer | Rana | `FS-RANA` | "the frog": it leaps |
   | The recovery designer | Cursa | `FS-CURSA` | "the footstool": where a flight comes to rest |

   Seven bright stars stay spare: Ran, Theemin, Azha, Beemim, Beid, Keid and Zibal. The four web tools (Motor Finder,
   Charge, Window, Muster) are rebuilt as library code and app screens (ADR-191, M9.6), so they are parts of Achernar
   and Zaurak, not products; if Neer wants one kept as a product, it takes a spare. The field kit (M6.8) and the parts
   catalog (M5.6) are parts of products too.
3. **External names stay.** The program is still `hpr-sim` (`hpr_core::tool::NAME`, the PyPI package) and the command
   and crates `hpr`. Renaming crates and the package is churn with no reader asking for it; picking product names stays
   open for the maintainer, for later. The docs site's title stays until then.
4. **The simulator's designation is `FS-ACHERNAR · SW · TOOL 001`.** Its stamp reads
   `hpr-sim 0.1.0 · FS-ACHERNAR · SW · TOOL 001`, external name first, as the rule asks. Readers keep accepting a
   file stamped `FS · SW · TOOL 005`, because files written since M0.6b carry it. The other products number their own
   designations from 001 when their first milestone ships.
5. **The rename comes before release 0.1 publishes.** After it, the docs site moves to
   `nrdptel.github.io/fusionspace-eridanus/` and the old site's links break, so it costs least while the only links to
   it are in this repository. A custom domain under fusionspace.co would make the site independent of the repository's
   name. That needs the maintainer's DNS, so it's optional and left to the maintainer.
6. **The catch-up is two increments of M10.1, ahead of its checklist (M10.1d4):**
   - **M10.1d7, the designation and the design pin:** §4 in code; an `.eng` with the old stamp reads without keeping
     it and writes back with one stamp, its tests keeping the old literal; the pin
     moved to the design repository's head with ADR-164's list of what changed; `docs/writing.md` gains the
     non-breaking space between a number and its unit on the site and in SVG figures, but not in terminal, JSON or
     CSV output, where copying and `grep` need a plain space; the README's design section stops promising M0.6, which
     shipped; the banners rebuilt with `project.py --star Achernar --tag SW --kind Tool --name hpr-sim`, whose
     defaults would give `FS-ACHERNAR · PRODUCT 001`.
   - **M10.1d8, the new name's URLs, after Neer's rename:** outside the ADRs, which keep their history, every
     `nrdptel/hpr-sim` (but the fixtures repository) and `nrdptel.github.io/hpr-sim` points at the new name, including
     `book.toml`'s `site-url`, xtask's `SITE_PATH`, the links the CLI prints and the Python package's URLs;
     `cargo xtask site` passes; the PyPI trusted-publisher step in the maintainer's release steps names the new
     repository.
7. **The clipping checks join the guides' milestone.** M0.7a's done-when already checks every figure's viewBox and the
   page's width from 320 to 1,920 px (ADR-195 §7). It gains the design system's third check, no text over other text,
   with a test that overlaps two labels and fails, and all three checks also run on the site's other pages and on
   `hpr sim --plot` output.
8. **The phone and watch rules wait for their milestones.** `mobile.md`, `watch.md`, the review items, SF Symbols and
   the Swift and Compose parts apply at M9.4 (mobile) and the watch milestone after it. They match ADR-191 §3: the
   phone is the hub, and a watch never commands hardware.

**Consequences.**

- The local folder keeps its name. Local tooling is keyed to its path, and a `git remote set-url` is all a rename
  needs locally; GitHub's redirect covers it until then.
- The design repository's register (Callsign's names in use, `kit/projects/`) needs project Eridanus and its seven
  stars. That's Neer's repository, so it stays open for the maintainer.
- If Neer renames late, after a publish, crates.io and PyPI keep the old URLs in 0.1.0's metadata, and GitHub's
  redirect covers everything except the docs site.
