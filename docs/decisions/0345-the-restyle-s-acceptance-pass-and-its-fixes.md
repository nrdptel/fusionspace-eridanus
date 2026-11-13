# ADR-345: The restyle's acceptance pass and its fixes (2026-11-13)

- **Status:** accepted; records [M0.17b2][m0-17b2]'s acceptance pass ([ADR-282][adr-282]) of the docs site's restyle ([ADR-343][adr-343], [ADR-344][adr-344]) and the fixes it led to; changes `docs/writing.md`'s rule that a code block may scroll sideways.
- **Summary:** A reviewer that didn't build the restyle ran [ADR-344][adr-344]'s six tasks on the built site, and a cold reader did them from the guide alone. At 320 px the keyboard couldn't reach the menu button, focus rings were cut at the column's edge, and code blocks hid their lines' ends. The captures of every page the tasks reach found a code block cut beside the open table of contents and the header left over a page's end on a phone; checking the cold read found a table of contents shut with reduced motion still a keyboard stop. This record fixes them, adds a page-check rule and canaries for each, and answers the cold read's findings in the site's words.

[m0-17b2]: ../decisions-and-roadmap.md#m0-17b2
[adr-282]: 0282-the-project-tests-each-ui-milestone-itself.md
[adr-343]: 0343-the-docs-site-drawn-as-a-tool-the-frame-off.md
[adr-344]: 0344-m0-17b-in-two-field-first-then-the-restyle-s-acceptance-pass.md
[i1451]: https://github.com/nrdptel/fusionspace-eridanus/issues/1451
[i1452]: https://github.com/nrdptel/fusionspace-eridanus/issues/1452
[i1453]: https://github.com/nrdptel/fusionspace-eridanus/issues/1453
[i1454]: https://github.com/nrdptel/fusionspace-eridanus/issues/1454

**Context.** [ADR-344][adr-344] wrote six tasks for the restyle's acceptance pass. A reviewer
ran them on the built site at commit 9a56668e, in headless Chrome over its DevTools protocol,
at 320 and 1,280 px, in Auto (a light system), Light, Dark and Field. At 1,280 px every task
passed. At 320 px three things failed (the pull request holds the pass's record and its
measurements):

- **The menu button.** mdBook draws it as a label for a hidden box, and no key reaches a label.
  On a phone the table of contents and the theme switch are behind it, so task 3 (choose Dark
  and reach the sidebar with the keyboard alone) couldn't be done: Tab went round the page with
  no menu, switch or sidebar among its stops.
- **Focus rings.** Every link at the column's left edge on the page about Monte Carlo dispersion
  had its ring cut there, in every theme. The pinned system stylesheet pads the
  content's box `16px - --page-padding`, 1 px under 720 px, and the box scrolls, so it cut 3 of the
  4 px a ring reaches out. The header's title link, on a row of its own under 520 px, spans the
  bar, so its ring passed both of the window's sides. The page check missed both: it read one
  control of each kind, and it stopped at the first box that scrolls, as a reader can scroll it.
  This box doesn't scroll sideways, so its cut was never seen.
- **Code blocks.** The page's first code block is wider than its box at 320 px, and the end of
  each long line was out of sight until the box was scrolled. `docs/writing.md` allowed
  it. The product system doesn't: `web.md`, *Accessibility*, lets only data tables and charts
  scroll sideways at 320 px (WCAG 2.2's reflow rule, 1.4.10), and its mdBook theme promises
  nothing cut off on a phone. The look at the captures found the page's second code block cut at its box's
  right side at 1,000 and 1,040 px, where the open table of contents narrows the column, its
  comments ending mid-word.

The cold reader did all six tasks and named five places where a reader is left unsure.
The switch's four buttons say nothing of what Field is for. The print button is labeled *Print
this book*, where the site says *guide* and the button opens the whole guide's print page. The
sidebar marks the current page with a class only, so a screen reader hears an ordinary link. The
footer's *Version 0.1.0* sits beside a date a month after that release, with no word of why.
And the explanations of the switch, the footer and printing share one long section at the end
of Start here.

**Decision.**

1. **Code wraps.** A code block wraps its long lines at any character, at every width
   (`theme/hpr.css`); copying takes the code as written. A box that scrolls still cuts a
   line's end at its side until the reader scrolls it, which no width can promise to avoid.
   The page check fails a code block, or the `code` in it, that scrolls sideways;
   `canary-code-scrolls.html` must fail at every width.
2. **Rings whole.** Under 720 px the content's box takes mdBook's page padding inside it, so
   the text stays 16 px from the window's side and a ring has room. A table's box gets 4 px a
   side beyond the table's edges at every width. Under 520 px the header's title keeps 6 px
   from each side of the bar. The header's menu and search buttons stand 8 px inside the bar's
   row, 34 px tall, below the 4 px strip fixed along the window's top, and from 960 px its
   print, repository and edit links 5 px inside it and the theme switch 2 px lower. A link the
   keyboard focuses stops 8 px above the window's bottom (`scroll-padding-bottom`). The page check now reads every control the keyboard reaches, not
   one of each kind, and fails a ring cut by the window's side, or by the side of a box that
   clips what passes it, where the control itself shows whole. A box that scrolls sideways cuts
   it too at the end of its scroll, as a ring adds nothing to what a box scrolls. A box that
   clips with `overflow: clip` is left to the window's rule: on the site only mdBook's page body
   clips so, at the window's sides. Up and down a page scrolls, so the check reads the window's
   top, or the strip's bottom where a strip is fixed there, while the page is at its top, and
   fails a page whose scroll padding at the bottom is under a ring's 4 px.
   `canary-focus-cut.html` (a box that hides what passes it), `canary-focus-scrollbox.html` (a
   scrolling box with 1 px of padding), `canary-focus-top.html` (a link at the window's top) and
   `canary-focus-bottom.html` (no scroll padding at the bottom) must fail.
3. **The menu button by keyboard.** `theme/hpr.js` puts the menu button in the tab order as a
   button that Enter and Space press. A press that opens the table of contents moves the focus
   to its first control, the theme switch under 960 px. The key is the button's, so it goes no
   further: mdBook's search follows a result marked as focused on an Enter that reaches the
   document. The page check presses Enter and Space on the menu button once a run, at the
   narrowest width it reads, and fails a button out of the tab order or one a key doesn't
   press. `canary-menu-unreached.html` and `canary-menu-key-dead.html` must fail. Put away,
   the table of contents leaves the tab order whole: `theme/hpr.js` takes its list's box out
   with its links. Chrome makes a box that scrolls and holds nothing the keyboard reaches a tab
   stop of its own. mdBook hides a shut table of contents once its slide ends, but with reduced
   motion set no slide runs, so one opened and shut again stayed drawn off the window's side, and
   on a phone Tab stopped on its list's box, out of sight.
   The page check fails a table of contents put away that holds a box the keyboard focuses;
   `canary-sidebar-scroller.html`, a table of contents drawn off the window's side under
   720 px and shown from there, must fail under 720 px and pass from it.
4. **The cold read's words.** Each theme button's tooltip says what it is for, Field's "For
   reading outdoors: a white page, darker gray text and stronger rules". The print button is
   *Print the guide*, and on paper the footer leaves off its link to the page about the project,
   whose address took a second line under the version and the date, which close the page.
   `theme/hpr.js` marks the sidebar's current entry `aria-current="page"`.
   Start here's *Reading these pages* ends in three short parts, the header and the theme
   switch, the footer, and printing, and the footer's says the version and the date can be
   weeks apart, and why.
5. **Kept as they are.**
   - The footer's words, which [ADR-343][adr-343] fixed and its check reads exactly. Start
     here, where its link leads, explains them.
   - Where the table of contents is shut (under 720 px, or put away at any width),
     mdBook's header scrolls away with the page and comes back when the reader scrolls up. Where it stood in between is not kept: decision 6.

   The pass's other findings off every task's path become issues: no link to skip the
   table of contents (from the top of a page at 1,280 px, Tab passes every chapter before it
   reaches the switch; [#1451][i1451]), and the footer's link, shorter than the system's 24 px
   target at 320 px ([#1452][i1452]), as the header's print, repository and edit links are. A
   second run adds two: Escape doesn't close the table of contents on a phone, and the print
   page's title doesn't name it ([#1453][i1453]). The look at the captures adds one: in Field
   nothing marks the table of contents' edge, both white, with no rule between them
   ([#1454][i1454]).
6. **The header leaves at once.** Where the table of contents is shut (under 720 px, or put away at any width),
   when a reader scrolling up turned to scroll down, mdBook's script left the header where the page then was, and it stayed there, a band
   over the page's text, until scrolled out. Near a page's end it can't be: the look at the
   captures found it over the Monte Carlo page's last paragraph and its page arrows at 360 to
   480 px. `theme/hpr.css` holds a header not stuck to the window at the page's top, so it
   leaves the window as the reader scrolls down and comes back, stuck, when the reader scrolls
   up. The page check sets the `top` mdBook's script would and fails a header that takes it;
   `canary-menu-lifted.html` must fail and `canary-menu-held.html` pass.

**Consequences.**

- A long line of code reads in full without scrolling, broken where it reaches the box's side.
  Code laid out in columns loses its columns where a line wraps.
- The page check reads every control's ring at every width, so a new box that cuts a ring, in a
  table, a note or the header, fails the build where it used to pass unseen. The page check's
  canaries number 228.
- A keyboard on a phone reaches the table of contents and the theme switch.
- Where the table of contents is shut (under 720 px, or put away at any width), the header goes as the reader
  scrolls down, rather than scrolling out with the
  page, and comes back, stuck to the window's top, when the reader scrolls up.

**How it was checked.** The pass's record is in M0.17b2's pull request: who ran it, on which
commits, each task's result and each finding's fix or issue. The fixes were checked by the same
measurements that found them: after them, a scan pressing Tab through the page about Monte Carlo
dispersion found no ring cut by a box or by the window's sides, top or bottom, at 320 and 1,280 px in
Auto, Light, Dark and Field. `cargo xtask site` passed at all 41 widths, with the nine new canaries that must
fail failing and `canary-menu-held.html` passing. Every page the six tasks reach was captured at
the page check's 41 widths in Light, Dark and Field and looked at for clipping: that look found
the code block cut at 1,000 and 1,040 px and the header over the Monte Carlo page's end at
360 to 480 px (decisions 1 and 6), and every capture the fixes changed was looked at again.
