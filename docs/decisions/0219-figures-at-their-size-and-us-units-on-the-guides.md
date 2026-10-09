# ADR-219: Figures at their size on the site, US units in the guides' prose; M0.9c13 split (2026-10-09)

- **Status:** accepted; carries out the product system's `foundations.md` (*Width*), `web.md` (*Accessibility*) and `data.md` (*Units for rocketry*, *Numbers*) for the site's part of #382 and for #400 (M0.9c13); amends ADR-216 §5, splitting #400's model, file format and records pages into M0.9c16
- **Summary:** a figure on the site may reach past the 750 px prose column, up to the product system's 1,120 px content width or the page's own, and is shown at its size wherever that holds it; in a narrower window it shrinks and mdBook's zoom shows it at its size. `cargo xtask site`'s page check fails a figure shown smaller than drawn with room for it, or without a zoom. Every height, distance and speed in the guides' prose and tables gives its US units in brackets after the SI, `1,400 m (4,593 ft)`, and a site check fails one in SI alone or with a bracket that isn't its conversion. The model pages, the file formats' pages, the accuracy page, the validation plan and the records page follow in M0.9c16, with millimeters.

[adr-164]: 0164-the-fusionspace-product-system.md
[adr-208]: 0208-the-design-audit.md
[adr-210]: 0210-us-units-in-brackets.md
[adr-216]: 0216-the-plots-type-and-spacing.md

**Context.** The plot `hpr sim --plot` draws is 960 px wide, with 12 px text, the product system's
floor. The site showed it in mdBook's 750 px prose column, scaled to fit, so its text read at about
9.4 px (#382, the audit's *web.md · Accessibility* row; [ADR-208][adr-208]). The system's
`foundations.md` (*Width*) allows content up to 1,120 px and prose up to 68 characters: the column
is a measure for prose, not for figures. Separately, `data.md` asks every height, distance and
speed in the units US flyers read; [ADR-210][adr-210] gives them in brackets after the SI on the
command line. The site's prose still gave 478 heights, distances and speeds in SI alone on 24 guide
pages, and 739 more on 28 model, file format and records pages, as the check below counts them
(#400). [ADR-216][adr-216] queued both as M0.9c13.

**Decision.**

1. **Figures at their size.** The theme (`theme/hpr.css`) lets a figure's box reach past the
   prose column on both sides, centered on it, up to 1,120 px or the width of the box that holds
   the page (`.content`, measured as a CSS container). An image narrower than that box is shown at
   its size; a wider one shrinks to fit, and mdBook's zoom (a click) shows it over the page at its
   size, up to the window's. Prose keeps its column. So the plot's text reads at 12 px wherever the
   window leaves 960 px beside the sidebar (from about 1,300 px with it open, 1,000 px with it
   closed), and a reader in a narrower window has one click to it.
2. **The page check holds it.** At every width from 320 to 1,920 px, `cargo xtask site`'s page
   check fails a figure in a page's `main` that is shown more than a pixel narrower than drawn
   while the box holding the page has room for it (up to 1,120 px), that is shown narrower than
   drawn without a zoom in its label, or whose size it can't read. Two canaries, a figure shown at
   half its size with room and one wider than any window with no zoom, must fail at every width;
   a figure reaching past a narrow `main` as the theme lays it out, and the ordinary page's
   zoomable wide figure, must pass. The pixel of slack covers `clientWidth`'s whole pixels and
   Chrome sizing a scaled image from its height snapped to 1/64 px. Since a figure now reaches
   past `main`, the check's rule that a box holding the page mustn't be wider inside than out
   applies to the boxes that scroll or clip; one whose overflow is visible, as mdBook's `main`
   is, hands what reaches past it to the box around it, which is measured in turn, and a canary
   with a label moved past the window from such a `main` must fail.
3. **US units in brackets in the guides.** A site check (`xtask/src/site/units.rs`) reads each
   page's text as it renders (prose, headings, list items, table cells, link text, image
   descriptions; not code) and fails a figure in m, cm, km, m/s or km/h that no bracket follows,
   or whose bracket's figure isn't it converted:
   - the units: heights, rails and other distances in ft, a part's size in in, long distances
     across the ground in mi; speeds in ft/s, and the wind in mph. The check accepts any of a
     quantity's US units, since it can't tell a wind from another speed or a part from a
     distance; which one fits is the writer's, and a review's;
   - the US figure converts the SI figure as written, with the exact factors, to any precision:
     it may differ from the exact conversion by half its own last place plus what rounding the SI
     figure to its last place can move it. `1,400 m (4,593 ft)` passes and `(4,600 ft)` doesn't:
     rounding is chosen in the meters. A range needs a range in its bracket;
   - exempt: a figure joined to an operator (`×`, `·`, `÷`, `/`, `*`, `+`, a spaced minus, `=`),
     which is a term or result of a worked calculation, stated in the unit it computes in; and
     e-notation (`1e-9 m/s`), a tolerance written as a program writes it.
   The failure suggests a bracket that passes. The pages that sum up the validation report must
   quote its numbers as it gives them (#298); that check skips the feet in these brackets, which
   are the report's meters converted, and still reads anything else a bracket says.
4. **The split.** M0.9c13 covers the guides: every page of the site but the model pages under
   `physics/`, the file formats' pages under `format/`, the accuracy page, the validation plan
   and the records page. Their 739 figures are M0.9c16's, queued after M0.9c15; #400 stays open
   for it. The check lists those pages as deferred and `cargo xtask site` prints how many figures
   they hold, so M0.9c16 is done when that list is empty.
5. **Millimeters wait for M0.9c16.** Most name a motor's or a mount's size (`a 54 mm motor`),
   which `data.md` keeps in millimeters; the rest are parts' sizes, which take inches. The check
   can't tell them apart by the unit, so M0.9c16 adds millimeters with that exception.

**Alternatives considered.**

- **Draw the plot 720 px wide, to fit the column.** Rejected: the column is a measure for prose;
  the figure would still shrink on a narrower window, and the plot `hpr sim --plot` writes for a
  flyer's own screen or print would lose width to the site's layout.
- **A box that scrolls sideways around a wide figure.** Rejected: it cuts the figure at the box's
  edge, which the project's rule against anything cut off by an edge forbids, and hides part of it
  until scrolled.
- **Exempt the file formats' pages for good,** since a file stores SI. Rejected: their prose also
  gives heights and speeds a flyer reads (a pad's height, a rail, the wind); M0.9c16 brackets
  those and leaves the values a file stores, quoted as code, as the file writes them.
- **An opt-out comment per paragraph or table.** Rejected for now: the exemptions above are rules
  a reader can predict; no guide needed another.
- **A units switch on the site.** Waits for the app's units control (M9.1), as [ADR-210][adr-210]
  §3 decided for the command line.

**What it leaves out.** A table that gives a unit in its head or in another cell has bare figures
the check doesn't read; on the guides, the one such height, the command line's default rail
length, now gives its feet. Units spelled out (`300 meters`) aren't read; the guides have none. Nor is
the text of raw HTML blocks. A review reads new ones.

**Consequences.** The audit's *web.md · Accessibility* row loses the plot's 9.4 px text; its
*data.md · Units for rocketry* row names the units check, and stays not met until M0.9c16. A new
guide page, or a new figure on one, fails the site check until its heights, distances and speeds
carry their US units. #382 closes with this increment.
