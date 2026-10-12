# ADR-252: The sidebar put away at every width, and page arrows that name their pages, from the product system's Rev D (2026-10-12)

- **Status:** accepted; moves the product system's pin from Rev C (`943492d`) to Rev D (`d372afa`), as [ADR-164](0164-the-fusionspace-product-system.md) lets a deliberate update do; reopens the design audit at the new pin ([ADR-208](0208-the-design-audit.md)); amends [ADR-229](0229-the-page-anatomy-the-frame-first.md), whose sidebar stayed open from 720 px with no menu button
- **Summary:** A review of the site found two things on a desktop window. The sidebar could be resized but not put away, and the page arrows were easy to miss: small icons at the window's sides, at the foot of the page, far from the centered column. The product system's Rev D lets a docs site's table of contents be put away and brought back at every width, with the menu button or by dragging its edge to the window's side, and keeps the choice from page to page. It puts the page arrows under the text, inside the column, as bordered buttons that name their pages.

**Context.** [ADR-229](0229-the-page-anatomy-the-frame-first.md) kept the sidebar open from 720 px and took the
menu button away there, following `web.md`'s "no hamburger above 720 px", which was written for a
site's header navigation. On a docs site that left a reader no way to put the table of contents
away. mdBook can do it: its menu button unticks the box that draws the sidebar, and its drag
handle marks the sidebar hidden once the edge comes within 20 px of the window's side. The
site's build forced the sidebar open on every load, and its stylesheet hid the button. And
since [ADR-250](0250-the-column-and-the-reading-scale.md) centered the column, mdBook's arrows sat
at the pane's far edges, about 300 px from the text on a 2,048 px window.

**Decision.**

1. **The table of contents can be put away** (`web.md`, *Site patterns*, Docs): a menu button at
   every width puts it away and brings it back, and dragging its edge to the window's side puts
   it away. Put away, the page's column re-centers in the whole window. The choice is mdBook's
   own (`mdbook-sidebar` in the browser's storage), kept from page to page; a first visit shows
   the sidebar from 720 px, as before. With scripts off nothing could answer the button, so from
   720 px the sidebar stays drawn there and the button goes, as before.
2. **The build keeps the reader's choice.** `frame.rs`'s start-up script, which mdBook runs before
   the page is drawn, shows the sidebar from 720 px unless mdBook stored `hidden`. `theme/hpr.js`
   opens it when a narrow window widens past 720 px, unless the reader put it away. mdBook's drag
   leaves its box ticked, which keeps the sidebar drawn as a 20 px strip with its links reachable,
   so `theme/hpr.js` unticks the box when such a drag ends, as the menu button would, and gives
   the sidebar back its width from before the drag for when it returns.
3. **Page arrows name their pages** (`web.md`, *Page anatomy*): under the text and inside the
   column, the previous at its left edge and the next at its right, each a bordered button at
   least 44 px tall. The system's mdBook theme draws them so. `frame.rs` writes in each arrow the
   direction, in `label` capitals, and the page's title, and drops mdBook's `title` and
   `aria-label`, "Previous chapter" and "Next chapter", so the name a screen reader reads is the
   text drawn (WCAG 2.5.3). The file check fails an arrow that doesn't name its page.
4. **The page check holds both.**
   - The menu button must be drawn at every width.
   - Once a run, at the width that reads print, or past 720 px when that width is narrower, the
     button must put the sidebar away and bring it back, and a drag of the handle to the
     window's side must put it away. "Away" means put away, hidden from screen readers, out of
     the tab order, the button still drawn, the page starting at the window's left.
   - Storage writes are stubbed for that probe, so pages loaded beside it still read a first
     visit.
   - Page arrows must sit under the text, inside the column, on its edges, at least 44 px tall,
     each naming its page.
   - Canaries show each failure and pass its counterpart: no menu button at any width; a model of
     mdBook's sidebar whose button, or whose drag, leaves it drawn; arrows at the window's sides;
     arrows naming no page.

**Consequences.** A reader on a desktop window can give the column the whole window, and the
column re-centers when they do. The choice outlives the page. The page arrows are where a reader
finishing the text looks, with the next page's title on them. The design audit reopens at
`d372afa`. Rev D changed four sections (`README.md`'s *Status*, `web.md`'s *Page anatomy*, *Site
patterns* and *Documentation sites*), which are read again; every other row reviewed at `943492d`
carries over. The site's "no hamburger above 720 px" still holds for a header's own navigation,
which this site doesn't have.
