# The documentation site's theme

mdBook reads this folder, `theme/` beside `book.toml`, for the site's look (ADR-164,
`docs/decisions/0164-the-fusionspace-product-system.md`). `cargo xtask site` checks every file
listed here.

| File | What it is | Terms |
|---|---|---|
| `fusionspace-mdbook.css` | The FusionSpace product system's mdBook theme, Rev A (`nrdptel/fusionspace-design` at `f45454f`, `product/web/mdbook/`), unchanged | Apache-2.0 |
| `fonts/*.woff2` | Archivo and Cascadia Mono, the product system's WOFF2 subsets (`product/web/fonts/`), unchanged | SIL OFL 1.1, the license texts beside them |
| `fonts/fonts.css` | hpr-sim's own `@font-face` rules for those files; mdBook links it from every page in place of its own fonts | MIT OR Apache-2.0 |
| `hpr.css` | hpr-sim's additions: the title block that ends the landing page, figures at their size, and the system's colors, square corners and motion where mdBook's own stylesheets reach past the theme (#383) | MIT OR Apache-2.0 |

`book.toml` sets `hash-files = false`: mdBook 0.5 renames the files it copies to add a hash, and
rewrites the font addresses only in its own default fonts, so with hashing on, `fonts.css` would
name files that aren't there.
