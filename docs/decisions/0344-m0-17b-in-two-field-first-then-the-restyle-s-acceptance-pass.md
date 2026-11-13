# ADR-344: M0.17b in two: Field first, then the restyle's acceptance pass (2026-11-13)

- **Status:** accepted; splits [M0.17b][m0-17b] ([ADR-339][adr-339]'s second increment) into [M0.17b1][m0-17b1] and [M0.17b2][m0-17b2], writes M0.17b2's user tasks before it is built ([ADR-282][adr-282]), and closes M0.17b1.
- **Summary:** M0.17b asks for two things that each take a session: a fourth color theme, Field, read by the page check at every width as light and dark are, and the acceptance pass for the whole restyle of the docs site. This record ships them one after the other. M0.17b1 adds Field to the theme switch and to the page check. M0.17b2 runs the acceptance pass over M0.17a's and M0.17b1's work and closes issue #1364.

[m0-17b]: ../decisions-and-roadmap.md#m0-17b
[m0-17b1]: ../decisions-and-roadmap.md#m0-17b1
[m0-17b2]: ../decisions-and-roadmap.md#m0-17b2
[adr-282]: 0282-the-project-tests-each-ui-milestone-itself.md
[adr-339]: 0339-m0-17-the-docs-site-at-the-tool-level-queued-second.md
[adr-343]: 0343-the-docs-site-drawn-as-a-tool-the-frame-off.md
[i1364]: https://github.com/nrdptel/fusionspace-eridanus/issues/1364
[i452]: https://github.com/nrdptel/fusionspace-eridanus/issues/452

**Context.** [ADR-339][adr-339] gave M0.17b one *done when*: the theme switch offers Auto,
Light, Dark and Field, where Field is mdBook's `rust` theme as the pinned stylesheet defines
it; `cargo xtask site` passes its page check at every width in light, dark and field; and the
pull request holds the acceptance pass ([ADR-282][adr-282]) for the whole restyle, M0.17a's
with it.

What is left, surveyed on `main` at 7669cadc:

- **Field.** The pinned stylesheet (`theme/fusionspace-mdbook.css`) already gives mdBook's
  `rust` class Field's colors, but nothing offers it. The switch has three buttons
  (`xtask/src/site/switch.rs`). `theme/hpr.js` forgets a saved `rust` theme. The page check
  (`xtask/src/site/pages.js`) knows two themes, light and navy: their color roles, their
  action color, which code stylesheet each loads, the theme each page is read in after the one
  it loads in, the print reading's comparison, and the switch's own check. Each needs Field
  added, with canaries that fail without it. Reading every page in a third theme may lengthen
  the site step, on this machine and on CI's runners.
- **The acceptance pass.** It covers every page the tasks reach, at the page check's 41
  widths, in every theme the site has, three once Field is in. A reviewer who did not build
  the work runs the tasks and judges the captures, and another who knows none of the code
  does a cold read. The first pass of this kind cost most of a working session to look at
  (ADR-282's amendment of 2026-10-23), and that one changed words only.

Both in one session leaves no room for the reviews and fixes either may need.

**Decision.**

1. **M0.17b in two.** M0.17b is met when both increments are.
   - **M0.17b1, Field.** *Done when:* the theme switch offers Auto, Light, Dark and Field,
     where Field is mdBook's `rust` theme as the pinned stylesheet defines it, and a saved
     Field choice is kept across pages. `cargo xtask site` passes its page check at every
     width in light, dark and field, reading each page in Field as it reads the other two,
     with Field's color roles from the product system's *Semantic roles* table. The page
     check fails a color off Field's roles when the page is read in Field, and a switch whose
     Field button doesn't put the page in Field, each with a canary.
   - **M0.17b2, the restyle's acceptance pass.** *Done when:* the acceptance pass
     ([ADR-282][adr-282]) is in its pull request and covers the whole restyle, M0.17a's
     ([ADR-343][adr-343]) and M0.17b1's: the tasks in decision 2 are done end to end on the
     built site, in a browser, and the record lists who ran them, on which commit, with each
     finding's fix or issue. [#1364][i1364] closes with it.
2. **M0.17b2's tasks,** written before it is built, as [ADR-282][adr-282] asks. Each is done
   in a browser on the built site, at a phone's width (320 px) and a desktop's (1,280 px), in
   each of Auto (light system), Light, Dark and Field unless the task names one:
   1. Open the landing page and reach the page about launch-day weather from the sidebar, or
      from the menu button on a phone, in two clicks or fewer.
   2. On the page about Monte Carlo dispersion, switch to Field, read its first table,
      equation, code block and note, follow a link to another page, and find Field still chosen there; then
      switch back to Auto.
   3. With the keyboard alone, from the top of the page about Monte Carlo dispersion: reach
      the theme switch, choose Dark, reach the sidebar's entry for the current page and the
      next page's arrow, and follow it.
   4. Print the page about Monte Carlo dispersion from Dark and from Field, in the browser's
      print preview: it prints in the light theme's colors, with no table of contents or
      buttons drawn, and the page's version-and-date line closes it.
   5. Press the print button and then *Print the guide*: the whole guide's print page ends
      with its title block.
   6. From the footer of the page about Monte Carlo dispersion, read its version and date, and
      follow its *About HPR Sim* link to the page about the project, Start here.

   No task reads a file, so none needs an input file. The screens are every page the tasks
   reach, at the page check's widths, in light, dark and field.
3. **[#452][i452]** (no field theme) closes with M0.17b1, and [#1364][i1364] stays open until
   M0.17b2.

**Consequences.**

- Field ships one session before the acceptance pass that looks at it, so its captures are
  looked at in M0.17b2 rather than in its own pull request. M0.17b1's pull request still takes
  the captures ADR-282 asks of every change to the site's styles, at its own size.
- The site step reads each page in a third theme. M0.17b1's pull request gives the step's time
  before and after, here and on CI's runners.

**How it was checked.** Both *done when*s fail on `main` at 7669cadc: the switch has three
buttons, Auto, Light and Dark (`switch.rs`, `CHOICES`), and the page check reads light and navy
only, so M0.17b1's fails; no acceptance pass for the restyle is recorded in any pull request,
so M0.17b2's fails.
