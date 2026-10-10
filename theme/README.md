# The documentation site's theme

mdBook reads this folder, `theme/` beside `book.toml`, for the site's look (ADR-164,
`docs/decisions/0164-the-fusionspace-product-system.md`). `cargo xtask site` checks every file
listed here.

| File | What it is | Terms |
|---|---|---|
| `fusionspace-mdbook.css` | The FusionSpace product system's mdBook theme, Rev A (`nrdptel/fusionspace-design` at `f45454f`, `product/web/mdbook/`), unchanged | Apache-2.0 |
| `fonts/*.woff2` | Archivo and Cascadia Mono, the product system's WOFF2 subsets (`product/web/fonts/`), unchanged | SIL OFL 1.1, the license texts beside them |
| `fonts/fonts.css` | The product system's `@font-face` rules (`product/web/fonts.css` at `f45454f`), each address without the `fonts/` folder, as mdBook serves this file beside the fonts: the subsets with their `unicode-range`, and the metric-matched fallback faces. mdBook links it from every page in place of its own fonts | Apache-2.0 |
| `icons/*.svg` | The product system's icons that `cargo xtask site` draws in each page's menu bar, page arrows, search field and code buttons, in place of mdBook's Font Awesome glyphs (`product/icons/` at `f45454f`), unchanged | The project owner's, all rights reserved, not under this repository's license (`THIRD-PARTY-NOTICES.md`) |
| `favicon.svg`, `favicon.png` | The site's favicon: the product system's web kit's (`kit/web/favicon.svg` and its 32 px `favicon-32.png` at `f45454f`), unchanged; mdBook serves both at the site's root in place of its book | The project owner's, all rights reserved, not under this repository's license (`THIRD-PARTY-NOTICES.md`) |
| `logo/fusion-space-horizontal-void.svg`, `logo/fusion-space-mark-void.svg` | The brand kit's one-color horizontal lockup and mark (`kit/logo/svg/` at `f45454f`), unchanged; `cargo xtask site` draws their paths inline, in the text's color, as each page's header and in the title block's owner field | The project owner's, all rights reserved, not under this repository's license (`THIRD-PARTY-NOTICES.md`) |
| `hpr.css` | hpr-sim's additions: the page's frame (the header and the title block every page ends with), figures at their size, and fixes for mdBook's own defaults ([#383](https://github.com/nrdptel/fusionspace-eridanus/issues/383)) listed below the table | MIT OR Apache-2.0 |
| `hpr.js` | hpr-sim's additions to mdBook's script: a click on the menu bar's lockup follows its link instead of mdBook's smooth scroll to the top; a window widened past 720 px opens the sidebar; the sidebar's `aria-hidden` and its links' focus follow whether it is drawn | MIT OR Apache-2.0 |
| `head.hbs` | What mdBook adds to every page's head: `color-scheme: light dark`, and the browser bar's colors, Paper on a light system and Void on a dark one, ahead of mdBook's own white | MIT OR Apache-2.0 |
| `../docs/images/copy.svg` | The copy button's icon: the product system's copy icon (`product/icons/copy.svg` at `f45454f`), its `currentColor` stroke set to Void (`#0B0F1C`, the system's darkest ink) so `cargo xtask figures`, which accepts only the system's colors, passes it; `hpr.css` uses it only as a mask, so the button's own color draws it | The project owner's, all rights reserved, not under this repository's license (`THIRD-PARTY-NOTICES.md`) |

What `hpr.css` changes in mdBook's own look, where the product system's theme doesn't reach:

- the system's colors in light and navy, and for readers with scripts off: code in ink on the
  surface, search hits in the action fill, the copy button's icon in the button's color;
- square corners and no shadows; the help popup a dialog with a 2 px ink border, kept 16 px from
  a phone's edges; the current chapter underlined;
- the system's icons at its sizes, 20 px in the menu bar and 24 px for the page arrows and code
  buttons, stroked in the text's color, the previous page's arrow mirrored;
- selected text on the action color at 22 %, as the system's `::selection` sets it;
- motion at the system's 100, 160 and 240 ms, snapping with reduced motion, and no smooth
  scroll from mdBook's script (`hpr.js`);
- the system's focus ring, a 2 px outline in the action color, 2 px off, for the keyboard;
- for forced colors, a border on each box set apart only by its fill, and the copy icon and
  search hits in the reader's own colors;
- `scroll-padding-top` under the sticky menu bar, so nothing focused hides under it;
- each font's metric-matched fallback second in its stack;
- the menu bar as the system's header, the lockup in the text's color with its clear space, no
  menu button from 720 px, where the sidebar stays open; the title block at every page's end;
- nothing cut off on a phone: the menu bar's lockup on a row of its own, the sidebar's frame kept
  off the page with scripts off, long search results wrapped.

`cargo xtask site`'s page check fails if mdBook's colors, corners, shadows or motion come back, as far
as it reads them, or its focus ring, a box or icon forced colors would erase, text under WCAG 2.2
AA's contrast, a sticky bar taller than the scroll padding, or a smooth scroll on the title's
click, an icon off 16, 20 or 24 px, a selection off the action color, a lockup off 24 px, the
ink or its clear space, a menu button from 720 px, a sidebar out of reach while shown, or a
title block off the system's or not last; its file checks fail a page without the header, the
lockup or the title block, and
a page's head without the scheme and the bar's colors, a `fonts.css` that isn't the system's, a
Font Awesome glyph, an icon that isn't the system's, a second `h1`, and a favicon that isn't
`favicon.svg` and `favicon.png` as committed. Hover and most styles scripts set aren't read yet
([#432](https://github.com/nrdptel/fusionspace-eridanus/issues/432)).

`book.toml` sets `hash-files = false`: mdBook 0.5 renames the files it copies to add a hash, and
rewrites the font addresses only in its own default fonts, so with hashing on, `fonts.css` would
name files that aren't there.
