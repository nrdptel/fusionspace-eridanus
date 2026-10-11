# ADR-232: The pages' vitals, measured as a phone on slow 4G (2026-10-10)

- **Status:** accepted; ships M0.9d10 (#443)
- **Summary:** `cargo xtask site` now loads every page of the guide (not rustdoc's API reference) cold in headless Chrome as Lighthouse's mobile preset (a 412 px phone, 562.5 ms on every request, 1.5 Mbit/s down) with the CPU slowed to a phone's by Lighthouse's BenchmarkIndex and the slowdown confirmed, measures LCP, CLS and INP, and fails a page over 2.5 s, 0.1 or 200 ms (the median of three loads for a page over), with a canary for each vital. To pass, every page links one stylesheet, asked for first with mdBook's sidebar script, and preloads four fonts, two more than the product system's limit; the theme switch is placed before the first paint; a phone's menu tap no longer slides the page; and the print page shows a cover on screen, its chapters placed in the page only when it prints. The page check now also fails anything fixed to the window on paper, which had covered the top of each printed chapter's tag.

**Context.** `web.md`, *Performance*, holds the site to the Core Web Vitals' "good" bounds at the
75th percentile of real visits: LCP at most 2.5 s, CLS at most 0.1, INP at most 200 ms
(<https://web.dev/articles/vitals>). The site has no visitors' numbers to read, and a check that
runs on every pull request can only load pages in a lab. The question is how to make a lab load
stand in for a phone, and what the site needed to pass.

The lab follows Lighthouse's mobile preset as it applies it through DevTools: the Moto G Power's
screen (412 by 823 CSS pixels at 1.75), each request delayed 562.5 ms and the link held to
188,743 bytes a second down and 86,400 up (Lighthouse's `core/config/constants.js`, scaled as its
`core/lib/emulation.js` and the Lantern constants do). Throttling the CPU by a fixed factor would
make the bound depend on the machine, so the check measures the machine first with Lighthouse's
`computeBenchmarkIndex` (ported, Apache-2.0) and slows it by the piecewise rule of Lighthouse's
CPU throttling calculator. This Mac scores about 4,400, which the calculator's last line, carried
past the machines it was fitted to, turns into a slowdown of 16; a CI runner scores lower and is
slowed less (CI's runners scored 2,334 to 2,726, slowed 7.4 to 9.1 times). A machine under
800 is refused: under 150 it can't be slowed to the phone, and under 800 its slowdown, less than
2, is too small to confirm (decision 1). The site is served
compressed, as GitHub Pages serves it, but over HTTP/1.1, which allows six connections to one
server where GitHub Pages' HTTP/2 has no limit, and each page is a first visit. Both err slow.

The API reference is left out, as the layout check leaves it out: its 1,497 pages are rustdoc's
own, drawn by rustdoc's stylesheets and scripts, not the site's theme, and measuring them would
add an estimated 35 minutes a run at the rate the guide's pages take.

Chrome 154 has two quirks the method had to meet. The emulated wait shows before a navigation's
`responseEnd`, not its `responseStart`, so the check confirms throttling by `responseEnd`. A
headless Chrome flags a layout shift in the half second after a page loads as following input
(`hadRecentInput`) when nothing was pressed, so the check counts flagged shifts made before its
first input.

Measured this way on `main` and on this branch as it went, the site failed each vital. Figures
marked *(a probe)* come from development runs at the slowdown given, not from committed output.

- **LCP.** Each page linked ten stylesheets, more requests than six connections carry at once,
  so they arrived in rounds and a phone's first paint came after 2.5 s.
- **CLS.** Text first drawn in the fallback fonts moved when the faces arrived; with the two
  fonts the system allows preloaded, CLS reached 0.18 on several pages (a probe). The theme
  switch, moved into the sidebar by the site's script after the first paint, shifted a phone's
  first screen. Inline code took its padding only when mdBook's script marked it.
- **INP.** On the print page, every chapter on one page (some 48,000 elements), the check
  measured INP 424 ms and LCP 2.92 s (medians of three loads, slowed 16.9 times); a trace of one
  load put the first Tab at 0.39 s and a menu tap at 0.45 s. On the aerodynamics page, the
  longest chapter, a tap took 0.30 s, as Chrome redrew the whole page on each frame of mdBook's
  slide.

**Decision.**

1. **The check** (`xtask/src/site/vitals.rs`, with `vitals.js` in the page and a small DevTools
   client in `cdp.rs`) loads every page but the API reference cold, presses Tab, taps the menu
   button twice, and takes INP as the slowest interaction's slowest event, CLS as the largest
   session window (shifts under 1 s apart, a window at most 5 s), and LCP before the first
   input. A page over a bound is loaded three times and its median held. Three canaries served
   by the check, a stylesheet held back 2.5 s, a box that grows 300 ms after load and a menu
   button that keeps the page busy 300 ms, must each fail their own vital alone, or the check
   fails. No canary's vital depends on the CPU's speed, so each browser runs the benchmark
   again once throttled, and again after its last page, and fails unless it ran slower by at
   least half the slowdown beyond 1 (a slowdown of 16 needs 8.5; an ignored throttle shows
   about 1); each load must also show the network's wait. A load that hasn't settled in 15 s
   fails, and so does a page of the guide whose menu button is missing or off screen. One
   browser loads one page at a time, as Lighthouse's `docs/variability.md` asks ("DO NOT
   collect multiple Lighthouse reports at the same time on the same machine"), and CI's three
   site machines each take a third of the pages. With two browsers at once on CI's four-core
   runners, the slowest INP reached 160 to 208 ms and failed a page.
2. **One stylesheet, asked for first.** The build joins the ten stylesheets into
   `theme/site.css` in their order (`bundle.rs`), print's inside `@media print`, the highlighting
   for both themes from `highlight.css`, as the theme's own rules color code over all three. Its
   link and a preload of mdBook's sidebar script, which blocks the page's text until it arrives,
   stand first in the head's additions, before the fonts. With the fonts first, the stylesheet or
   the script waited a round trip on a long page, whose own text holds a connection longest: the
   aerodynamics and print pages drew at 2.4 to 2.5 s, and at 2.0 s with them first.
3. **Four preloaded fonts, against the system's two.** `web.md` says to preload at most the two
   fonts above the fold. A phone's first screen draws in four: Archivo for text and SemiBold for
   bold, Cascadia Mono SemiBold for titles and Regular for inline code. On the site as this
   decision ships it, with only the system's two preloaded, CLS was 0.174 on the accuracy page
   and 0.177 on the Python page (medians of three); with four it is at most 0.067. A fifth
   pushed LCP to 2.50 s (a probe). This is a deviation from the system, measured, and the
   conformance row says so.
4. **No shift after the first paint.** A script the build writes after the header places the
   theme switch in the sidebar on a narrow window before the page is drawn; inline code has its
   padding from the stylesheet.
5. **The menu tap moves the page at once.** On a phone only the sidebar slides; the page's
   `transform` isn't animated, its margin still is. Measured without this, the aerodynamics
   page's tap took 0.30 s.
6. **The print page is a cover on screen.** The build puts the print page's chapters in a
   `<template>` behind a cover: the designation tag, "Print the guide", what it prints, a button
   that prints and how to print from the browser (`print.rs`). The site's script places a copy of
   the chapters in the page on `beforeprint`, highlighting their code as mdBook does, and removes
   it on `afterprint`; the cover hides on paper, so the printed book is as before. mdBook's call
   to print as the page loads is taken out: a browser with no print window (a headless one, and
   possibly a phone) starts printing and never ends, which left the chapters in the page, and a
   print window opening unasked helps no one. Rejected, from probes: `content-visibility: auto`
   on the chapters, which answered a tap in 16 ms at a 4 times slowdown but left 2 of the
   page's 895 headings in Chrome's accessibility tree; hiding the chapters on screen with
   `display: none`, whose first Tab still took 0.20 s at a 16.9 times slowdown, as Chrome walks
   every element in the page; and dropping the print page, the one way to keep the whole guide
   on paper or in a PDF for the field. With the cover, the first Tab took 22 ms at 16.9 times.
7. **Nothing fixed on paper.** The system's colored rule along the window's top
   (`body::before`) is fixed there, and a browser prints what is fixed on every sheet, over the
   text: it covered the top border of the tag opening each printed chapter. On paper it now
   stands once, and the tag keeps its chamfer's stroke without background graphics. The page
   check's paper reading fails anything fixed to the window, with a canary, and fails a print
   page whose chapters don't all arrive as it prints or don't all leave after, with canaries
   both ways.

**Consequences.** In one run on the site as this decision ships it, one page at a time at a 16.1
times slowdown, the slowest LCP was 2.35 s (the PF2 format page), the largest CLS 0.067 (the page
on how a flight is simulated) and the slowest INP 88 ms (the aerodynamics page). Runs with four
browsers at once on this Mac gave 1.99 to 2.16 s, 0.067 and 88 to 152 ms: LCP and INP move by a
tenth or more between runs, and LCP's least margin to its bound is about 0.15 s. The check takes
about six minutes on this Mac. The header's "Print this book" icon now opens the cover, one more
step to the print window, and the book prints only with scripts on. The check depends on Chrome's
DevTools protocol and its Event Timing and layout-shift entries; a Chrome that changes them fails
the check through its canaries, its throttle checks or a load with nothing to read, rather than
passing silently. The lab is not the field: it runs over HTTP/1.1 and loads every page cold, which
make it slower than a visit, while the CPU slowdown on a fast machine extrapolates Lighthouse's
calculator, which could err either way. Field numbers would settle it, and the site collects none.
