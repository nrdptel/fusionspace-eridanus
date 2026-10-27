# ADR-297: A table too wide for its column stacks its rows (2026-10-27)

- **Status:** accepted; closes #498; departs from `data.md`'s *Tables* rule on wide tables
- **Summary:** A table wider than its column no longer scrolls sideways in its box. Its rows stack, each cell labeled by a copy of its column's head, and the page check fails a table that still scrolls, at every width from 320 px.

[adr-282]: 0282-the-project-tests-each-ui-milestone-itself.md

**Context.** The product system's `data.md` says: "Wrap wide tables in a scrolling region rather
than turning rows into cards." The site did, and the page check let a table's box scroll
sideways. On a phone that hides a table's last columns until the reader scrolls the table, with
nothing on screen to say there is more. #498 found it on the simulator's comparison page. A
headless count of the 75 guide pages in a 320 px frame found 309 tables wider than their box.
#975's narrower cell padding on a phone (8 px a side, none at the outer edges) brought that to
213, and 132 of those have four to ten columns. The maintainer holds every surface to having
nothing cut off by an edge or a corner, with a layout that fits in place of one that scrolls. A
table cut at its box's edge falls short of that.

**Decision.**

1. **A table too wide for its column stacks its rows** (`theme/hpr.js`, `theme/hpr.css`). Each
   row becomes a group under the table's own rules: 1 px between rows, 2 px of ink above the
   first, as under a head. In each cell, a copy of its column's head stands beside the value, in
   the head's type (Cascadia Mono 12/16, capitals). It stands above the value when the value holds
   a formula, since MathML doesn't break a line. There is no box, fill or shadow, so the rows are
   not cards, and the markup stays a real table. A table that fits is drawn as before. The copies
   stay hidden on paper and while a table fits.
2. **It is measured only where it is laid out.** A sheet off screen skips its layout
   (`content-visibility: auto`), and measuring a table in it would lay the sheet out. So a table
   in view is measured as the script runs, before the page is first drawn. The rest are measured
   when a resize observer sees them laid out. It watches an empty ruler at the top of each table's
   box, which is as wide as the box and never taller, so stacking can't set it off again within a
   frame. A stacked table stands again once its box is as wide as the table was.
3. **The page check fails a table that scrolls sideways** in the nearest box around it that
   scrolls (`a table wider than its box, scrolling sideways`), at each of its 41 widths. A canary plants a table whose cell holds a
   box 2,000 px wide, which no stacking fits. Code blocks still scroll: a command can't wrap. A
   line of data can, in a `text,wrap` fence.

**Consequences.** On a phone, a wide table reads down rather than across. Comparing one column
down its rows takes more scrolling than on a wide screen, but nothing is out of sight. Without
the script, a table scrolls in its box as before. A page's acceptance pass ([ADR-282][adr-282])
sees its tables as a reader does, stacked or not. The cost is small: on the page check's phone run, the largest
layout shift on the check's phone run is 0.022, under its bound of 0.1, and the slowest largest
paint 1.98 s, under its 2.5 s.
