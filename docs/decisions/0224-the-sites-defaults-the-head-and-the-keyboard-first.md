# ADR-224: The site's defaults split again: the head, the keyboard and forced colors first (2026-10-10)

- **Status:** accepted; splits M0.9d4 ([ADR-221][adr-221]'s rest of #383) in two; adds M0.9d6
- **Summary:** M0.9d4 took the four classes of mdBook default that M0.9d2 left: icons, the head's metas and accessibility, fonts, and print, with the script's smooth scroll. M0.9d4 now covers what the stylesheets, the head and a script of the site's own can fix without changing mdBook's page template: `color-scheme` and the Paper and Void `theme-color` metas, the 2 px Ion focus ring, forced colors, `scroll-padding-top` under the sticky bar, contrast measured, the system's `fonts.css` with its fallback faces in every stack, and no smooth scroll, each failing `xtask site` or a site test when it comes back. The new M0.9d6 keeps the old done-when whole (#383 closed, every class held), adds #443 (Core Web Vitals measured), and takes what needs the template or a print reading: the system's icons for Font Awesome's, the favicon, one `h1` a page, and print.

[adr-221]: 0221-the-sites-defaults-split-by-what-the-screen-draws.md

**Context.** #383's head-and-accessibility class, its fonts and the smooth scroll are fixed by
files mdBook takes as they are: `theme/head.hbs`, which mdBook puts into every page's head ahead
of its own `theme-color`; `theme/hpr.css`; `theme/fonts/fonts.css`; and a script of the site's
own (`additional-js`). Each is read by `xtask site`: the built pages' heads by a file check, and
the rest by the page check, which reads computed styles in headless Chrome. The other classes
change mdBook's page template: its menu bar draws Font Awesome glyphs and a second `h1` (the
book's title) on every page, and print needs its own reading in the page check, with light
values, link addresses and rows kept whole. Replacing the template means carrying a changed copy
of mdBook's `index.hbs`, which is MPL-2.0, and deciding how that file is kept in step with
mdBook. One increment holds the first group with its checks and canaries; it can't hold both.

**Decision.**

1. **M0.9d4: the head, the keyboard, forced colors, fonts and the smooth scroll.**
   - Every built page (the API reference aside) declares `color-scheme: light dark`, and the
     first `theme-color` meta a browser takes is Paper (`#F3F4F7`) on a light system and Void
     (`#0B0F1C`) on a dark one, as the system's `kit/web/head.html` sets them.
   - One element of each kind the keyboard reaches, focused as by the keyboard, draws a 2 px
     solid outline in the action role, 2 px off (`foundations.md`, `focus`); the zoom's hidden
     checkbox draws it on its picture.
   - With the stylesheets' forced-colors rules applied, no block is set apart from what is
     behind it by its fill alone, no inline icon is drawn in a color other than its text's, and
     an icon painted through a mask keeps its own colors.
   - No text is under WCAG 2.2 AA's contrast (4.5 : 1, or 3 : 1 at 24 px or 18.66 px bold)
     against the background it is drawn on, faded by any opacity between them, in every
     reading the page check makes.
   - A bar fixed or stuck to the window's top reaches no further down than the root's
     `scroll-padding-top`.
   - `theme/fonts/fonts.css` holds the system's `@font-face` rules at the pinned revision, each
     address without the `fonts/` folder, and every Archivo or Cascadia Mono text names its
     metric-matched fallback second.
   - A click on the menu bar's title asks for no smooth scroll.
2. **M0.9d6: the rest of #383**, with M0.9d4's old done-when unchanged: the system's icons at
   16, 20 or 24 px, the favicon, one `h1` a page, and print. It goes after M0.9d5, which changes
   the same menu bar for the system's lockup header. It also takes #443, found here: `web.md`'s
   *Performance* asks for Core Web Vitals, which nothing measures; its done-when adds #443
   closed.
3. **The smooth scroll is taken by the site's own script.** mdBook's `book.js` calls
   `scrollTo` with a smooth scroll when the title is clicked; `theme/hpr.js` takes the click on
   its way down to the title and jumps to the top. The page check wraps every scrolling call
   while it clicks the title, so a smooth scroll from any script fails it.

**Consequences.** The page check takes longer: 218 s for 66 pages at 41 widths on the
maintainer's machine, against about 160 s before (#432 tracks its time). The forced-colors
reading applies the stylesheets' `forced-colors: active` rules, as the reduced-motion reading
does; it can't make the browser paint system colors, so it judges what forced colors would drop
(fills and shadows) from the page's own colors. The head check reads each page as served: once
the page loads, mdBook's script sets the first `theme-color` meta, now the light one, to the
page's background when the reader picks a theme, so a reader who picks the light theme on a dark
system gets a Void bar over a Paper page, where mdBook's single meta followed the page. The
system says nothing should depend on the bar's color (`web.md`, *Theme*). Hover styles and
`::selection` aren't read yet; the theme menu still lists mdBook's six themes (#429).
