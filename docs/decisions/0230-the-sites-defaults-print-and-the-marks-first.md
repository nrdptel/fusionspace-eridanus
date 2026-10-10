# ADR-230: The site's defaults split again: print and the text marks first (2026-10-10)

- **Status:** accepted; splits M0.9d8 ([ADR-228][adr-228]'s print and vitals) in two; adds M0.9d10
- **Summary:** M0.9d8 took the last of #383 and all of #443: print, the three text marks mdBook draws as icons, and the Core Web Vitals measured. M0.9d8 now covers print and the marks: the build draws the sidebar's fold toggle as the system's chevron, the theme menu underlines the chosen theme, a heading jumped to takes no mark, and the site prints in the light theme's values with navigation hidden, link addresses shown and rows, notes and the title block kept whole. The page check fails a text mark drawn as an icon, and reads every page with its print rules applied at the run's width nearest a sheet of paper (720 px), in both themes, failing each print rule with canaries. The new M0.9d10 keeps the old done-when whole (#383 and #443 closed) and takes the Core Web Vitals.

[adr-228]: 0228-the-sites-defaults-the-chrome-first.md

**Context.** ADR-228 left M0.9d8 print, three text marks (`❱` on the sidebar's fold toggles,
`✓` beside the chosen theme, `»` before a heading jumped to; #383's comment) and #443. Print and
the marks are readings of the page's own stylesheets, which the page check already makes for
dark systems, reduced motion and forced colors by applying a media rule in place. The vitals
are not: Largest Contentful Paint, Interaction to Next Paint and Cumulative Layout Shift at a
phone's 75th percentile (`web.md`, *Performance*) need each page loaded on a throttled network
and CPU, a lab setup the check doesn't have (headless Chrome takes CPU throttling only through
its DevTools protocol, which the check doesn't speak). One increment holds print and the marks
with their checks and canaries; it can't hold a throttled lab too.

**Decision.**

1. **M0.9d8: print and the text marks.** Done when #383 is closed, each class of mdBook default
   it lists failing `xtask site` or a site test when it comes back.
   - **The marks.** `xtask site` swaps `toc.js`'s `❱` for the system's chevron at 16 px, which
     the icon reading sizes; a `toc.js` that draws its toggles any other way fails the build.
     `theme/hpr.css` takes away the `✓` and underlines the chosen theme 2 px, as the sidebar
     marks the current page (`web.md`, *Page anatomy*), and takes away the `»`: a heading
     jumped to already scrolls to the top of the window, under the scroll padding. The page check fails a text made
     only of marks (Unicode's arrows, technical symbols, shapes, symbols, dingbats, guillemets
     and the private-use characters icon fonts use) outside `main` or as a `::before` or
     `::after` anywhere, with every element that has an id read as the one jumped to. A key's
     legend (`kbd`) and prose in `main` may hold one: the help popup names the `←` key, and the
     accuracy page ticks its table with `✓`.
   - **Print** (`web.md`, *Print*). `theme/hpr.css` gives every theme the light values on paper,
     colors links in the action role, not `print.css`'s `#4183c4`, writes each link's address
     after it, and keeps table rows and the title block whole (notes already were). The page
     check flips each media list naming `print` or `screen` to how it applies on paper, since a
     headless browser can't be asked to print from a page, and reads the page in the theme it
     is in and, where it switches, the other: it fails a color off the light roles (and every
     other color, shape and contrast reading), an element whose colors differ between the
     themes, navigation or a control drawn, a link out of the page without its address, a row,
     note or title block that can split, and a title block not drawn with its version and date
     of issue. Paper is read once a run, at the checked width nearest 720 px, the printable
     width of an A4 or Letter sheet in Chrome's default margins; a canary's print problem is
     expected there only, and a page or canary read on paper at no width or two fails.
2. **M0.9d10: the pages' vitals,** with M0.9d8's old done-when unchanged: #383 closed, each
   class of mdBook default it lists failing `xtask site` or a site test when it comes back, and
   #443 closed, the Core Web Vitals measured by `xtask site`. It goes after M0.9d9.

**Consequences.** The page check reads each page twice more, on paper, at one width of each run,
and every element once more with `:target` standing in. A color defect on screen that also
prints is named twice, as a color and as print. Where Chrome breaks a printed page isn't seen:
the check reads the print rules, not paginated output. The scripts-off and dark-system readings
aren't repeated on paper. The audit's *Print* and *Icons* rows are met; *Performance* stays not
met until M0.9d10.
