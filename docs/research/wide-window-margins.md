# What fills the empty side bands on wide windows

This note asks what the docs site, and fusionspace.co with it, should put in the bare background either side of its
centered 1,120 px column on a wide window. It surveys what other documentation sites do there, then ranks ten options
by what the design system would need, how each folds away at narrow widths with nothing clipped, its cost in reading
comfort, and whether mdBook carries it without a fork. It serves the wide-window layout that
[ADR-250](../decisions/0250-the-column-and-the-reading-scale.md) (the column and the reading scale) left open, and
builds on [design conformance](design-conformance.md). **Inference** marks this note's reasoning. Measured in
headless Chrome, light mode, 2026-10-16.

**In short.** The bands are intended, common, and costly to fill well.
- **The bands are 464 px a side on a 2,048 px window with the sidebar put away, 310 px with it open; on a 1,440 px
  window, 160 px put away and none open** (32 px, the gutter). Measured on the live Atmosphere page (§1).
- **Bare bands are the norm.** MDN leaves 304 px a side at 2,048 px, rustdoc 843 px on the right, Wikipedia 266 and
  278 px. Sites that use the side space mostly put a sticky "On this page" list there (§2).
- **The design system makes the bands on purpose and forbids decorating them** (§4).
- **Ranked first: a station rail in the right band**, the page's sheets drawn as a station axis under a one-line
  "how far to trust it"; numbers only in a 160 px band, hidden below. **Second: notes in the empty sheet rail inside
  the column**, the only option that also works at 1,440 px with the sidebar open. **Third, purely visual: a
  drafting field**, a faint static grid with zone letters in the bands only. All three are mocked up at 2,048 and
  1,440 px with nothing clipped (§5).
- **The prose line is long either way: about 106 characters at 2,048 px, 89 at 1,440 px with the sidebar open and
  105 put away**, against guidance of 45–75 (Bringhurst) to 90 (Butterick). No option changes it (§3).

## 1. The bands, measured

Pane P = window − sidebar (300 px + an 8 px resize strip when open, 0 put away) − any classic scrollbar (15 px on
Windows and Linux, 0 on macOS). Column C = min(1,120, P − 64). Each band, gutter included, is (P − C) / 2. The
column reaches 1,120 px from a 1,184 px window put away, 1,492 px open. The sheet rail (`SHEET n / N` and the
sheet's name, 184 px plus a 32 px gap) stands beside the text from 944 px put away and 1,252 px open
(`theme/hpr.css` lines 173–180; mdBook's sidebar width [MDB-VARS]).

| Window | Sidebar | Column | Band each side | Prose beside the rail | Characters a line |
|---|---|---|---|---|---|
| 2,048 px | put away | 1,120 px | 464 px | 904 px at 18 px | 106 |
| 2,048 px | open | 1,120 px | 310 px | 904 px at 18 px | 106 |
| 1,440 px | put away | 1,120 px | 160 px | 904 px at 17.4 px | 105 |
| 1,440 px | open | 1,068 px | 32 px | 852 px at 17.4 px | 89 |

Characters a line: the median full line of the first long paragraph. A box w wide, 32 px clear of column and edge,
needs a band of w + 64 px: a 240 px rail fits from a 1,728 px window put away, 2,036 px open (**Inference**).

Two facts constrain anything in a band. The page check loads every page at 41 widths from 320 to 1,920 px as a first
visit, so with the sidebar open, and tries the put-away layout once, near 720 px (`xtask/src/site/pages.rs`,
`pages.js`): the widest case, 2,048 px put away, is never measured. And each sheet is paint-contained
(`content-visibility: auto` with an 8 px `overflow-clip-margin`, `hpr.css` lines 161–168), so anything a sheet draws
more than 8 px outside its box is cut off. The check's clip rule treats paint containment as a clip, so it would
catch band text drawn from inside a sheet; a band element must still live outside the sheets to be shown.

## 2. What other sites do with the width

Measured at 2,048 px [SURVEY]:

| Site | Prose width (est. characters) | Beside the text | Bare band |
|---|---|---|---|
| Wikipedia, Vector 2022 | 948 px (~133) | contents list left | 266 px L, 278 px R |
| MDN guide | 768 px (~104) | sticky nav left; sticky "In this article" right | 304 px each side |
| Stripe docs guide | 829 px (~112) | nav at the left edge; sticky "On this page" | ~344 px R |
| Rust Book (mdBook) | 750 px (~98) | 300 px chapter list | ~495 px R |
| rustdoc (std, docs.rs) | 936 px (~132–136) | 200 px sidebar; left-aligned 960 px limiter | 843 px R |
| Tailwind docs | 696 px | sticky "On this page"; 40 px hatched strips at ~5% | 328 px each side |
| Linear docs | 650 px (~91) | nav at the left edge; sticky contents | 396 px R |
| Observable Framework | 640 px (~87) | gray sidebar fill to the left edge; fixed contents | ~320 px R |
| NASA Europa Clipper; ESA Juice | 600 and 565 px | full-bleed hero photo, key-facts grid | under the hero |

- **Wikipedia** limited its width after estimating ~170 characters a line at 1,280 px and ~262 at 1,920 px; its
  full-width toggle shows from 1,400 px [WM-WIDTH] or 1,600 px [WM-NEWS], the sources disagree.
- **Decoration** is rare on docs pages (Tailwind's faint hatch); none runs a full-bleed band through its prose.
- **Wider figures:** Distill's `l-body` < `l-middle` < `l-page` < `l-screen`, one column below 768 px [DISTILL];
  Observable's 896 px figures beside 640 px text; Tufte CSS's `fullwidth` [TUFTE]. At 390 px a 2017 Distill article
  scrolled sideways: one wide figure didn't fold.
- **Grids:** GOV.UK, a 1,020 px page and a two-thirds column for lines of ≤75 characters [GOVUK]; USWDS, 1,024 px
  and a 68 ex measure token [USWDS]; Material 3, consider a third pane from 1,600 dp [M3].

## 3. Reading comfort: what is measured

- **Line length.** Bringhurst: 45–75 characters, 66 ideal [WEBTYPO]; Butterick 45–90 [BUTTERICK]; WCAG 1.4.8
  (AAA): a way to get lines of 80 or fewer [WCAG148]. On screen, Dyson's review reports 100 characters read faster
  than 25 (other differences not significant), but 55 gave better comprehension than 100 [DYSON] pp. 382–383. The
  site's 106 is past every bound but the speed result; 89 is past all but Butterick's. ADR-250 chose the long line
  citing the speed result; the comprehension result cuts the other way. Filling the bands doesn't change it.
- **Contents rails.** In Wikimedia's test 18 people ranked five contents designs; a persistent list in the left
  margin came first in every group [WM-TOC]. A sticky header cut scrolls a session by ~15% for logged-in users
  [WM-STICKY]. NN/g: a rail's contents list should be sticky and may mark the current section; label a right rail
  "On this page" against "right-rail blindness" [NNG-TOC]. On 1,920 px screens 80% of fixations fell on the left
  half [NNG-LEFT].
- **Decoration.** Texture under text slowed a word search; speed fell as the background's contrast rose
  (r = −0.78) [SCHARFF]. Animation as a secondary stimulus degraded information seeking [ZHANG]; WCAG 2.2.2 wants
  a pause for motion over 5 s [WCAG222]. A static, faint drawing beside text is unmeasured (**Inference**: small).
- **Sidenotes.** Tufte CSS folds notes behind a toggle at ≤760 px [TUFTE]; gwern.net hides its sidenote columns at
  ≤1,760 px [GWERN-CSS]. Gwern argues for notes shown by default [GWERN]. Benefit argued, not measured.

## 4. What the site and the design system allow

- **The system intends the bands.** The column is "up to 1120 px", centered "so a window wider than the column has
  equal margins on both sides" (`foundations.md`, pinned revision `1b88597`).
- **It forbids decoration:** "no ornament", "if it has no job, remove it" (`principles.md`), "no background art"
  (`web.md`), "plots of real data" instead of abstract illustration (`review.md`), no hatching behind text, no
  animated backgrounds (`foundations.md`). It has no side rail, aside, sidenote or pattern component; breakpoints are
  literals (640, 720, 880 px). The nearest parts: `.fs-split > .fs-sticky` and `.fs-balloon`.
- **mdBook already lists a page's headings.** 0.5.0 added sidebar heading navigation (`sidebar-header-nav`, on by
  default) [MDB-CL]; the site keeps it. mdBook has no right-side rail; mdbook-pagetoc (MPL-2.0) adds one [PAGETOC].
- **No fork needed.** The site rewrites mdBook's output after the build (ADR-229) and can insert a band element
  beside `main` that way; `#mdbook-sidebar-toggle-anchor:checked ~ .page-wrapper` tells the sidebar states apart.
- **The check's blind spots.** It reads `background-color`, not `background-image`, so a drawn background escapes
  its palette and contrast rules. It fails `position: fixed` only where a transformed box carries it (mdBook's page
  layer under 620 px) and on paper, so a fixed rail hidden there passes.

## 5. The options, ranked

Cost counts the system revision, the build and the check work. "Shows" lists §1's four layouts in order: 2,048 px
put away, 2,048 px open, 1,440 px put away, 1,440 px open.

| Rank | Option | Shows | System need | Cost |
|---|---|---|---|---|
| 1 | Station rail, right band | full, full, numbers, hidden | rail component; rule for a band | medium |
| 2 | Notes in the sheet rail | everywhere the rail is (column ≥ 880 px) | note-in-rail rule | small–medium |
| 3 | Drafting field (visual) | both bands, both, both, none | revise "no background art" | small build, new check |
| 4 | Full-bleed sheet rules | 2,048 px only | rule across the pane | medium |
| 5 | Trust card alone | as 1 | card variant of the trust note | medium |
| 6 | Sidenotes in the right band | as 1 | sidenote component | large |
| 7 | Figures wider than the column | 2,048 px only | revise the 1,120 px rule | large |
| 8 | Live glossary or citation panel | as 1 | panel component, term matcher | large |
| 9 | Faint flight plot of the page's example | as 3 | revise; per-page data | large |
| 10 | Leave bands bare | nothing | none | none |

1. **Station rail.** A sticky "On this page" list in the right band, drawn as a body's station axis: a 2 px ink
   line, one tick per sheet with its number and name, the current sheet's tick heavier. On top, one line of "how
   far to trust it" from the page's own trust note. At most 280 px wide, 32 px clear of the column and the edge.
   Below a 264 px band it keeps only ticks and numbers (96 px in a 160 px band); below 160 px it is gone and the
   sidebar's heading list stands in. Where both show (2,048 px, sidebar open) the sidebar's list should hide. Best
   evidence of any option [WM-TOC] [NNG-TOC], in the system's vocabulary, and a job for the band. Route: the
   post-build rewrite inserts an `aside` beside `main`; `display: none` in print; a mark that doesn't animate.
2. **Notes in the sheet rail.** The 184 px under each sheet's name is empty wherever the column is 880 px or more,
   1,440 px open included. Glossary terms and source keys go there, marked by a letter, folding in under their
   paragraph below 880 px. It fills the white space readers see most, not the bands; a figure would clear a long note.
3. **Drafting field.** A static grid in the rule color (16 px minor, 80 px major), zone letters A–H down each band's
   outer edge like a drawing sheet's border, fading out 64 px before the column so nothing is drawn under text
   [SCHARFF]. The cheapest fill (CSS on the frame), but the system forbids it as written, and the check needs a
   background-image rule first.
4. **The rest.** Full-bleed sheet rules hit the containment clip and add little. A trust card alone repeats the
   opening note; it is better as rank 1's top line. Right-band sidenotes need 1,728 px or more and a placement script.
   Wider figures break the column rule for a gain only at 2,048 px. A glossary panel needs a term matcher and grows
   a search index at 9.06 MB of mdBook's 10 MB warning (`book.toml`). A faint plot is decoration unless it is a
   real, labeled figure, and then it is rank 6's placement problem plus data per page.

**Mock-ups.** Ranks 1–3, built over a saved copy of the live Atmosphere page (sidebar open and put away, 2,048 and
1,440 px), are in the uncommitted research cache, `refs/research/wide-window-margins/mock/` (`build.py`). Each run
checks that every text a mock-up draws stays 16 px inside the window, below the menu bar, and clear of the column
or inside the rail; all twelve shots pass, and two canaries (past the edge, on the column) fail. At 2,048 px, put away:

```text
| band 464 px        | column 1,120 px                                            | band 464 px              |
| A  drafting field  | SHEET 1 / 5   prose ...................................... | HOW FAR TO TRUST IT      |
|                    | In short      prose ...................................... | Tables within 0.1%       |
| B                  | a · term      prose ...................................... | ON THIS PAGE             |
|                    | (rank 2)      prose ...................................... | |- 01 In short           |
| C                  |                                                            | |- 02 Code and sources   |
```

## What this suggests

- Put the three mock-ups to the maintainer before any build. Each needs a design-system revision first: a rail
  component and a band rule (rank 1), a note-in-rail rule (rank 2), or "no background art" relaxed for the bands
  (rank 3).
- Whatever is picked, add to the page check first: widths past 1,920 px (2,048 and 2,560 px), the sidebar put away
  at wide widths, and a `background-image` rule. Keep band elements outside the paint-contained sheets.

## Open questions

- **The prose measure.** Lines run 89–106 characters, past most guidance. A narrower measure with notes beside
  it would fix that but reopens ADR-250's choice to let prose fill the column. Nothing here measures which reads
  better on this site.
- Not measured: fusionspace.co (assumed to follow the same arithmetic), dark mode, `position: sticky` on the floated
  rail. No Wikimedia A/B result exists for its width; Dyson & Haselgrove and Zhang were read via review and abstract.

## Sources

All read 2026-10-16. Screenshots, the Dyson PDF and mdBook's guide (MPL-2.0) are cached under the gitignored
`refs/research/wide-window-margins/`, cite only. Tufte CSS (MIT) and Distill (Apache-2.0) were read, not ported.

- [SURVEY] headless Chrome at 2,048, 1,440 and 390 px: Wikipedia (Model rocket), Stripe docs, MDN's grid guide, the
  Rust Book, std's `Vec`, docs.rs, Tailwind, Linear docs, Observable Framework and Plot, NASA Europa Clipper, ESA Juice.
- mediawiki.org/wiki/Reading/Web/Desktop_Improvements/ + `Features/Limiting_content_width` [WM-WIDTH],
  `Features/Sticky_Header` [WM-STICKY], `Repository/Sticky_Header_and_Table_of_Contents_User_Testing` [WM-TOC].
  [WM-NEWS] meta.wikimedia.org/wiki/Tech/News/2022/48.
- [NNG-TOC] Wang & Brown, "Table of contents", NN/g, 2023, nngroup.com/articles/table-of-contents/. [NNG-LEFT]
  Fessenden, "Horizontal attention leans left", NN/g, 2017, nngroup.com/articles/horizontal-attention-leans-left/.
- [DYSON] Dyson, "How physical text layout affects reading from screen", Behav. Inf. Technol. 23(6), 377–393, 2004.
  [WEBTYPO] webtypography.net §2.1.2 (Bringhurst). [BUTTERICK] practicaltypography.com/line-length.html.
- [WCAG148], [WCAG222] w3.org/WAI/WCAG22/Understanding/ `visual-presentation.html`, `pause-stop-hide.html`.
- [SCHARFF] Scharff, Ahumada & Hill, "Discriminability measures for predicting readability", SPIE 3644, 1999.
  [ZHANG] Zhang, "The effects of animation on information seeking performance", JAIS 1(1), 2000 (abstract).
- [TUFTE] edwardtufte.github.io/tufte-css/ and its `tufte.css`. [GWERN] gwern.net/sidenote; [GWERN-CSS] its
  `static/css/head.css`. [DISTILL] distill.pub/guide/ and `template.v2.js`.
- [GOVUK] design-system.service.gov.uk/styles/layout/. [USWDS] designsystem.digital.gov (layout grid, measure).
  [M3] m3.material.io/foundations/layout/breakpoints/overview.
- [MDB-CL] github.com/rust-lang/mdBook `CHANGELOG.md`, 0.5.0; [MDB-VARS] its
  `crates/mdbook-html/front-end/css/variables.css`.
  [PAGETOC] github.com/slowsage/mdbook-pagetoc.
