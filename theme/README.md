# The documentation site's theme

mdBook reads this folder, `theme/` beside `book.toml`, for the site's look (ADR-164,
`docs/decisions/0164-the-fusionspace-product-system.md`). `cargo xtask site` checks every file
listed here.

| File | What it is | Terms |
|---|---|---|
| `fusionspace-mdbook.css` | The FusionSpace product system's mdBook theme, Rev E (`nrdptel/fusionspace-design` at `1b88597`, `product/web/mdbook/`), unchanged: its colors and fonts, and the page's frame: the type scale on mdBook's elements, growing with the window inside the page (16 px body up to a 1,024 px window, 18 px from 1,600 px), and the column centered beside the sidebar up to 1,120 px with its gutters, the prose filling it, and the page arrows under the page ([ADR-250](../docs/decisions/0250-the-column-and-the-reading-scale.md)) | Apache-2.0 |
| `fonts/*.woff2` | Archivo and Cascadia Mono, the product system's WOFF2 subsets (`product/web/fonts/`), unchanged | SIL OFL 1.1, the license texts beside them |
| `fonts/NewCMMathSubset-Book.woff2` | The math face that draws the equations: the product system's renamed subset of New Computer Modern Math Book (`product/web/fonts/`), unchanged ([ADR-264](../docs/decisions/0264-typeset-equations.md)) | GUST Font License (the LPPL 1.3c), the texts beside it (`THIRD-PARTY-NOTICES.md`) |
| `fonts/fonts.css` | The product system's `@font-face` rules (`product/web/fonts.css` at `1b88597`), each address without the `fonts/` folder, as mdBook serves this file beside the fonts: the subsets with their `unicode-range`, and the metric-matched fallback faces. mdBook links it from every page in place of its own fonts | Apache-2.0 |
| `icons/*.svg` | The product system's icons that `cargo xtask site` draws in each page's menu bar, page arrows, search field and code buttons, in place of mdBook's Font Awesome glyphs, and `ok` and `info` in each page's status chip (`product/icons/` at `1b88597`), unchanged | The project owner's, all rights reserved, not under this repository's license (`THIRD-PARTY-NOTICES.md`) |
| `favicon.svg`, `favicon.png` | The site's favicon: the product system's web kit's (`kit/web/favicon.svg` and its 32 px `favicon-32.png` at `1b88597`), unchanged; mdBook serves both at the site's root in place of its book | The project owner's, all rights reserved, not under this repository's license (`THIRD-PARTY-NOTICES.md`) |
| `logo/fusion-space-horizontal-void.svg`, `logo/fusion-space-mark-void.svg` | The brand kit's one-color horizontal lockup and mark (`kit/logo/svg/` at `1b88597`), unchanged; `cargo xtask site` draws their paths inline, in the text's color, as each page's header and in the title block's owner field | The project owner's, all rights reserved, not under this repository's license (`THIRD-PARTY-NOTICES.md`) |
| `../xtask/src/site/fusionspace-check.js` | The product system's page check (`product/web/check/fusionspace-check.js` at `1b88597`), unchanged, not served: the numbers the system's web rules set (the type scale, the reading scale, the widths) and the code that finds prose held narrower than its column and counts the characters of each line as drawn, which `cargo xtask site`'s page check runs | Apache-2.0 |
| `hpr.css` | hpr-sim's additions: the page's frame (the header and the title block every page ends with), figures at their size, and fixes for mdBook's own defaults ([#383](https://github.com/nrdptel/fusionspace-eridanus/issues/383)) listed below the table | MIT OR Apache-2.0 |
| `hpr.js` | hpr-sim's additions to mdBook's script: a click on the menu bar's lockup follows its link instead of mdBook's smooth scroll to the top; a window widened past 720 px opens the sidebar; the sidebar's `aria-hidden` and its links' focus follow whether it is drawn | MIT OR Apache-2.0 |
| `head.hbs` | What mdBook adds to every page's head: `color-scheme: light dark`, and the browser bar's colors, Paper on a light system and Void on a dark one, ahead of mdBook's own white; and, on every page but mdBook's sidebar frame (`toc.html`, which has no title), the link preview a shared address draws, as Open Graph and X's card tags (`og:*` and `twitter:card`, a large image): the page's own title, the book's description from `book.toml`, and the card below by its whole address, `https://hpr.fusionspace.co/images/og-image-dark.png` | MIT OR Apache-2.0 |
| `../docs/images/copy.svg` | The copy button's icon: the product system's copy icon (`product/icons/copy.svg` at `1b88597`), its `currentColor` stroke set to Void (`#0B0F1C`, the system's darkest ink) so `cargo xtask figures`, which accepts only the system's colors, passes it; `hpr.css` uses it only as a mask, so the button's own color draws it | The project owner's, all rights reserved, not under this repository's license (`THIRD-PARTY-NOTICES.md`) |
| `../docs/images/og-image-dark.png` | The link preview's image: the brand kit's card for HPR, 1200 × 630 (`kit/projects/achernar/og-image-dark.png` at `1b88597`, the same file at `075d567`), unchanged. It sits in the guide's images, not here, as mdBook serves no image from `theme/` but the favicon; mdBook copies it to the site's `images/`. `cargo xtask site` compares it with the pinned revision like the files above, fails a site that doesn't serve it as committed, and fails a page whose head doesn't name it with its size | The project owner's, all rights reserved, not under this repository's license (`THIRD-PARTY-NOTICES.md`) |

Since Rev B the product system's theme also fixes mdBook's own defaults where they break the
system's rules ([ADR-250](../docs/decisions/0250-the-column-and-the-reading-scale.md)): the roles
as `--fs-<role>` in every theme, with scripts off and in print; code in ink on the surface; search
hits in the action fill; square corners and no shadows; the help popup; the current chapter
underlined; the system's motion; the focus ring and scroll padding; borders where forced colors
drop a fill; each font's metric-matched fallback second in its stack; nothing cut off on a phone;
and print in the light theme with link addresses. What `hpr.css` adds is what only this site's
build writes:

- the copy button's icon, the system's `copy.svg` as a mask in the button's color;
- the system's icons at its sizes, 20 px in the menu bar and 24 px for the page arrows and code
  buttons, stroked in the text's color, the previous page's arrow mirrored;
- the notes, each kind's strip and border;
- the menu bar as the system's header, the lockup in the text's color with its clear space, the
  menu button at every width, which puts the sidebar away and brings it back (`hpr.js` also puts
  it away when its edge is dragged to the window's side), and the sidebar drawn with scripts off;
- the page arrows' names, the direction in `label` capitals over the page's title;
- the title block at every page's end, the intro's tag, chip and lead, and the sheets;
- the theme switch, and text marks drawn as the system's icons;
- for forced colors and print, what those parts need; the print page's cover.

`cargo xtask site`'s page check fails, with the system's own check (`fusionspace-check.js`), text
off the type scale at the window's reading scale, prose held narrower than its column, a column
off 1,120 px, and a column off center or off the gutters. It fails if
mdBook's colors, corners, shadows or motion come back, as far as it reads them, or its focus ring, a box or icon forced colors would erase, text under WCAG 2.2
AA's contrast, a sticky bar taller than the scroll padding, or a smooth scroll on the title's
click, an icon off 16, 20 or 24 px, a selection off the action color, a lockup off 24 px, the
ink or its clear space, no menu button, a sidebar the button or a drag doesn't put away, a
sidebar out of reach while shown, page arrows off the column's edges or naming no page, or a
title block off the system's or not last; its file checks fail a page without the header, the
lockup or the title block, and
a page's head without the scheme and the bar's colors or the link preview's tags (`og:title`
the page's own title, `og:image` the card by its whole address), a `fonts.css` that isn't the
system's, a
Font Awesome glyph, an icon that isn't the system's, a second `h1`, and a favicon that isn't
`favicon.svg` and `favicon.png` as committed. Hover and most styles scripts set aren't read yet
([#432](https://github.com/nrdptel/fusionspace-eridanus/issues/432)).

`book.toml` sets `hash-files = false`: mdBook 0.5 renames the files it copies to add a hash, and
rewrites the font addresses only in its own default fonts, so with hashing on, `fonts.css` would
name files that aren't there.
