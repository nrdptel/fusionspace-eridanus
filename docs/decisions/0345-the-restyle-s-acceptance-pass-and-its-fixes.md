# ADR-345: The restyle's acceptance pass and its fixes (2026-11-13)

- **Status:** accepted; records [M0.17b2][m0-17b2]'s acceptance pass ([ADR-282][adr-282]) of the docs site's restyle ([ADR-343][adr-343], [ADR-344][adr-344]) and the fixes it led to; changes `docs/writing.md`'s rule that a code block may scroll sideways on a phone.
- **Summary:** A reviewer that didn't build the restyle ran [ADR-344][adr-344]'s six tasks on the built site, and a cold reader did them from the guide alone. At 320 px the keyboard couldn't reach the menu button, focus rings were cut at the column's edge, and code blocks hid their lines' ends. This record fixes the three, adds a page-check rule and canaries for each, and answers the cold read's findings in the site's words.

[m0-17b2]: ../decisions-and-roadmap.md#m0-17b2
[adr-282]: 0282-the-project-tests-each-ui-milestone-itself.md
[adr-343]: 0343-the-docs-site-drawn-as-a-tool-the-frame-off.md
[adr-344]: 0344-m0-17b-in-two-field-first-then-the-restyle-s-acceptance-pass.md
[i1451]: https://github.com/nrdptel/fusionspace-eridanus/issues/1451
[i1452]: https://github.com/nrdptel/fusionspace-eridanus/issues/1452

**Context.** [ADR-344][adr-344] wrote six tasks for the restyle's acceptance pass. A reviewer
ran them on the built site at commit 9a56668e, in headless Chrome 157 over its DevTools
protocol, at 320 and 1,280 px, in Auto (a light system), Light, Dark and Field. At 1,280 px
every task passed. At 320 px three things failed:

- **The menu button.** mdBook draws it as a label for a hidden box, and no key reaches a label.
  On a phone the table of contents and the theme switch are behind it, so task 3 (choose Dark
  and reach the sidebar with the keyboard alone) couldn't be done: Tab went round 95 stops with
  no menu, switch or sidebar among them.
- **Focus rings.** 86 of the 190 stops on the page about Monte Carlo dispersion had their ring
  cut 3 px at the column's left edge, in every theme. The pinned system stylesheet pads the
  content's box `16px - --page-padding`, 1 px under 720 px, and the box scrolls, so it cut the
  4 px a ring reaches out. The header's title link, on a row of its own under 520 px, spans the
  bar, so its ring passed both of the window's sides. The page check missed both: it read one
  control of each kind, and it stopped at the first box that scrolls, as a reader can scroll it.
  This box doesn't scroll sideways, so its cut was never seen.
- **Code blocks.** The page's first code block holds 430 px of text in a 286 px box, and the
  end of each long line was out of sight until the box was scrolled. `docs/writing.md` allowed
  it. The product system doesn't: `web.md`, *Accessibility*, lets only data tables and charts
  scroll sideways at 320 px (WCAG 2.2's reflow rule, 1.4.10), and its mdBook theme promises
  nothing cut off on a phone.

The cold reader did all six tasks and named five places where a reader is left unsure.
The switch's four buttons say nothing of what Field is for. The print button is labeled *Print
this book*, where the site says *guide* and the button opens the whole guide's print page. The
sidebar marks the current page with a class only, so a screen reader hears an ordinary link. The
footer's *Version 0.1.0* sits beside a date a month after that release, with no word of why.
And the explanations of the switch, the footer and printing share one long section at the end
of Start here.

**Decision.**

1. **Code wraps on a phone.** Under 720 px a code block wraps its long lines at any character
   (`theme/hpr.css`); copying takes the code as written. From 720 px a code block keeps its
   lines and may scroll inside its box. The page check fails a code block, or the `code` in
   it, that scrolls sideways under 720 px; `canary-code-scrolls.html` must fail there and pass
   from 720 px.
2. **Rings whole.** Under 720 px the content's box takes mdBook's page padding inside it, so
   the text stays 16 px from the window's side and a ring has room. A table's box gets 4 px a
   side beyond the table's edges at every width. Under 520 px the header's title keeps 6 px
   from each side of the bar. The page check now reads every control the keyboard reaches, not
   one of each kind, and fails a ring cut by the window's side, or by the side of a box that
   clips what passes it, where the control itself shows whole. A box that scrolls sideways cuts
   it too at the end of its scroll, as a ring adds nothing to what a box scrolls. A box that
   clips with `overflow: clip` is left to the window's rule: on the site only mdBook's page body
   clips so, at the window's sides. Sideways only, as a page scrolls up and down and the sticky
   bar has its own rule. `canary-focus-cut.html` (a box that hides what passes it) and
   `canary-focus-scrollbox.html` (a scrolling box with 1 px of padding) must fail.
3. **The menu button by keyboard.** `theme/hpr.js` puts the menu button in the tab order as a
   button that Enter and Space press. A press that opens the table of contents moves the focus
   to its first control, the theme switch under 960 px. The key is the button's, so it goes no
   further: mdBook's search follows a result marked as focused on an Enter that reaches the
   document. The page check presses Enter and Space on the menu button once a run, at the
   narrowest width it reads, and fails a button out of the tab order or one a key doesn't
   press. `canary-menu-unreached.html` and `canary-menu-key-dead.html` must fail.
4. **The cold read's words.** Each theme button's tooltip says what it is for, Field's "For
   reading outdoors: a white page, darker gray text and stronger rules". The print button is
   *Print the guide*. `theme/hpr.js` marks the sidebar's current entry `aria-current="page"`.
   Start here's *Reading these pages* ends in three short parts, the header and the theme
   switch, the footer, and printing, and the footer's says the version and the date can be
   weeks apart, and why.
5. **Kept as they are.**
   - The footer's words, which [ADR-343][adr-343] fixed and its check reads exactly. Start
     here, where its link leads, explains them.
   - A printed page's footer link keeps its address after it, as every printed link does, so on
     paper the footer's last link wraps to a second line under the version and the date.
   - Under 1,080 px mdBook's header scrolls away with the page and comes back when the reader
     scrolls up.

   The pass's other two findings off every task's path become issues: no link to skip the
   table of contents (from the top of a page at 1,280 px, Tab takes 96 presses to reach Dark;
   [#1451][i1451]), and the footer's link, 15 px tall at 320 px, under the system's 24 px
   target ([#1452][i1452]).

**Consequences.**

- On a phone a long line of code reads in full without scrolling, broken where it reaches the
  box's side. From 720 px nothing changes.
- The page check reads every control's ring at every width, so a new box that cuts a ring, in a
  table, a note or the header, fails the build where it used to pass unseen. The page check's
  canaries number 223.
- A keyboard on a phone reaches the table of contents and the theme switch.

**How it was checked.** The pass's record is in M0.17b2's pull request: who ran it, on which
commits, each task's result and each finding's fix or issue. The fixes were checked by the same
measurements that found them: after them, the ring scan finds no ring cut in 180 stops at 320 px
or in 186 at 1,280 px, in Light, Dark and Field. `cargo xtask site` passed at all 41 widths,
with the six new canaries failing as they must.
