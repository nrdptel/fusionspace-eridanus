# The documentation site's theme

mdBook reads this folder, `theme/` beside `book.toml`, for the site's look (ADR-164,
`docs/decisions/0164-the-fusionspace-product-system.md`). `cargo xtask site` checks every file
listed here.

| File | What it is | Terms |
|---|---|---|
| `fusionspace-mdbook.css` | The FusionSpace product system's mdBook theme, Rev A (`nrdptel/fusionspace-design` at `f45454f`, `product/web/mdbook/`), unchanged | Apache-2.0 |
| `fonts/*.woff2` | Archivo and Cascadia Mono, the product system's WOFF2 subsets (`product/web/fonts/`), unchanged | SIL OFL 1.1, the license texts beside them |
| `fonts/fonts.css` | hpr-sim's own `@font-face` rules for those files; mdBook links it from every page in place of its own fonts | MIT OR Apache-2.0 |
| `hpr.css` | hpr-sim's additions: the title block that ends the landing page, figures at their size, and fixes for mdBook's own defaults ([#383](https://github.com/nrdptel/fusionspace-eridanus/issues/383)) listed below the table | MIT OR Apache-2.0 |
| `../docs/images/copy.svg` | The copy button's icon: the product system's copy icon (`product/icons/copy.svg` at `f45454f`), its `currentColor` stroke set to Void (`#0B0F1C`, the system's darkest ink) so `cargo xtask figures`, which accepts only the system's colors, passes it; `hpr.css` uses it only as a mask, so the button's own color draws it | The project owner's, all rights reserved, not under this repository's license (`THIRD-PARTY-NOTICES.md`) |

What `hpr.css` changes in mdBook's own look, where the product system's theme doesn't reach:

- the system's colors in light and navy, and for readers with scripts off: code in ink on the
  surface, search hits in the action fill, the copy button's icon in the button's color;
- square corners and no shadows; the help popup a dialog with a 2 px ink border, kept 16 px from
  a phone's edges; the current chapter underlined;
- motion at the system's 100, 160 and 240 ms, snapping with reduced motion;
- nothing cut off on a phone: the menu bar's title on a row of its own, the sidebar's frame kept
  off the page with scripts off, long search results wrapped.

`cargo xtask site`'s page check fails if mdBook's colors, corners, shadows or motion come back, as far
as it reads them: hover, focus and the styles scripts set aren't read yet ([#432](https://github.com/nrdptel/fusionspace-eridanus/issues/432)).

`book.toml` sets `hash-files = false`: mdBook 0.5 renames the files it copies to add a hash, and
rewrites the font addresses only in its own default fonts, so with hashing on, `fonts.css` would
name files that aren't there.
