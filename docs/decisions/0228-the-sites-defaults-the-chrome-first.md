# ADR-228: The site's defaults split again: the chrome first (2026-10-10)

- **Status:** accepted; splits M0.9d6 ([ADR-224][adr-224]'s rest of #383) in two; adds M0.9d8
- **Summary:** M0.9d6 took the last of #383 and #443: the system's icons for mdBook's Font Awesome glyphs, the favicon, one `h1` a page, print, and the Core Web Vitals measured. M0.9d6 now covers what the page's chrome draws: after mdBook builds the site, `cargo xtask site` swaps each glyph for the system's icon of the same meaning and makes the menu bar's title a paragraph; a file check fails a glyph left, an icon not the system's or a second `h1`; the favicon is the system's; and the page check fails an icon off 16, 20 or 24 px and a selection off the action role at 22 %, each with canaries. The new M0.9d8 keeps the old done-when whole (#383 and #443 closed) and takes print and the Core Web Vitals. The field theme, which the audit counted under #383 but which is no mdBook default, moves to #452.

[adr-224]: 0224-the-sites-defaults-the-head-and-the-keyboard-first.md

**Context.** ADR-224 left M0.9d6 four classes of #383 and all of #443. Three of the classes are
in the markup mdBook writes around every page: its menu bar, page arrows, search spinner and
code-block buttons draw Font Awesome 6.2 glyphs, inline (1,024 across the guide's 65 pages, as
`cargo xtask site` counts those it swaps; #383 measured them at 13 px in the menu bar and 40 px
for the arrows); its favicon is a book; and its menu bar names
the book in an `h1` above the page's own title, so every page has two. ADR-224 named two ways to
change that markup: carry a changed copy of mdBook's MPL-2.0 page template, or rewrite the built
HTML as the notes are rewritten (ADR-225). Print needs a reading of its own in the page check
(light values whatever the theme, link addresses, rows kept whole), and the Core Web Vitals a lab
measurement with a throttled phone. One increment holds the chrome with its checks and
canaries; it can't hold the two readings too. Two items were added to #383 after the audit:
`::selection`, a color mdBook leaves to the browser, and a field theme.

**Decision.**

1. **M0.9d6: the chrome, the favicon and the selection.**
   - The build rewrites the chrome, as it rewrites the notes: no copy of mdBook's template.
     `xtask/src/site/chrome.rs` names each place mdBook draws a glyph (by a mark in the tag the
     glyph sits in) and the system's icon for it: menu, sun for the theme menu, search, print,
     external for the repository (the system draws no maker's logos), edit, refresh for the
     spinner, chevron for the arrows (mirrored for the previous page), and plus, minus, copy,
     simulate and refresh for the code buttons' templates. A glyph in a place it doesn't name
     fails the build, so a newer mdBook can't add one unseen.
   - The icons and the favicon (the web kit's SVG and its 32 px PNG, which mdBook serves from
     `theme/`) are copied unchanged into `theme/`, compared with the pinned revision like the
     rest of the theme; the build checks the site serves the committed favicon.
   - A file check fails a built page with a Font Awesome glyph, an icon span holding anything
     but one of the system's icons, or more than one `h1`. `print.html`, every chapter on one
     page, keeps each chapter's `h1`, as a printed book opens each chapter with its title; it
     loses the menu bar's like every page.
   - The page check fails an icon of the chrome drawn at this width that isn't square at 16, 20
     or 24 px (`foundations.md`, *Icons*), and text whose selection isn't the action role at
     22 % opacity, the system's `::selection`, in every reading of colors.
2. **M0.9d8: print and the Core Web Vitals**, with M0.9d6's old done-when unchanged: #383 closed,
   each class of mdBook default it lists failing `xtask site` or a site test when it comes back,
   and #443 closed, the vitals measured by `xtask site`. It goes after the queued M0.9d7, which
   rebuilds the menu bar as the system's lockup header.
3. **The field theme is #452,** not #383: #383 lists what mdBook draws where the system's theme
   doesn't reach, and a missing theme is a feature of its own (`principles.md` §4). The audit's
   *principles.md · 4* row names #452.

**Consequences.** `mdbook serve` still shows mdBook's glyphs and two `h1`; the published site is
what `xtask site` builds. The page check reads one more property per element with text in each
color reading, and one box per icon at each width. The rewrite is tested on mdBook 0.5.4's
output; a newer mdBook that renames a glyph's place fails the build until `PLACES` names it.
The copy button keeps its icon painted through a mask (`docs/images/copy.svg`), at 24 px. Three
text marks mdBook still draws, `❱` on the sidebar's fold toggles, `✓` beside the chosen theme and
`»` before a heading jumped to, are icons no check reads; they stay with #383 for M0.9d8, and the
audit's *Icons* row stays not met until then.
