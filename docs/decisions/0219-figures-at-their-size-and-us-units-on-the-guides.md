# ADR-219: Figures at their size on the site, US units in the guides' prose; M0.9c13 split (2026-10-09)

- **Status:** accepted; carries out the product system's `foundations.md` (*Width*), `web.md` (*Accessibility*) and `data.md` (*Units for rocketry*, *Numbers*) for the site's part of #382 and for #400 (M0.9c13), and splits #400's model pages and records into M0.9c16
- **Summary:** a figure on the site may reach past the 750 px prose column, up to the product system's 1,120 px content width or the page's own, and is shown at its size wherever that holds it; in a narrower window it shrinks and mdBook's zoom shows it at its size. `cargo xtask site`'s page check fails a figure shown smaller than drawn with room for it, or without a zoom. Every height, distance and speed in the guides' prose and tables gives its US units in brackets after the SI, `1,400 m (4,593 ft)`, and a site check fails one in SI alone or with a bracket that isn't its conversion. The model pages, the accuracy page, the validation plan and the records page follow in M0.9c16; the file formats' pages stay in the file's units.

[adr-164]: 0164-the-fusionspace-product-system.md
[adr-208]: 0208-the-design-audit.md
[adr-210]: 0210-us-units-in-brackets.md
[adr-216]: 0216-the-plots-type-and-spacing.md

**Context.** The plot `hpr sim --plot` draws is 960 px wide, with 12 px text, the product system's
floor. The site showed it in mdBook's 750 px prose column, scaled to fit, so its text read at about
9.4 px (#382, the audit's *web.md · Accessibility* row; [ADR-208][adr-208]). The system's
`foundations.md` (*Width*) allows content up to 1,120 px and prose up to 68 characters: the column
is a measure for prose, not for figures. Separately, `data.md` asks every height, distance and speed
in the units US flyers read; [ADR-210][adr-210] gives them in brackets after the SI on the command
line, and the site's prose still gave 470 heights, distances and speeds in SI alone on its guide
pages and 632 more on 23 model and records pages, as the check below counts them (#400).
[ADR-216][adr-216] queued both as M0.9c13.

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
   while the box holding the page has room for it (up to 1,120 px), or that is shown narrower than
   drawn without a zoom in its label. Two canaries, a figure shown at half its size with room and
   one wider than any window with no zoom, must fail at every width, and the ordinary canary page
   holds a zoomable wide figure that must pass. The pixel of slack covers `clientWidth`'s whole
   pixels and Chrome sizing a scaled image from its height snapped to 1/64 px.
3. **US units in brackets in the guides.** A site check (`xtask/src/site/units.rs`) reads each
   page's text as it renders (prose, headings, list items, table cells, link text, image
   descriptions; not code) and fails a figure in m, km, m/s or km/h that no bracket follows, or
   whose bracket's figure isn't it converted:
   - heights and distances in ft, in or mi; speeds in ft/s, or mph for the wind; the writer picks
     which, as `data.md`'s table gives them (a part's size in inches, a rail in feet);
   - the US figure converts the SI figure as written, with the exact factors, to any precision:
     it may differ from the exact conversion by half its own last place plus what rounding the SI
     figure to its last place can move it. `1,400 m (4,593 ft)` passes and `(4,600 ft)` doesn't:
     rounding is chosen in the meters. A range needs a range in its bracket;
   - exempt: a figure joined to an operator (`×`, `·`, `÷`, `/`, `*`, `+`, a spaced minus, `=`),
     a term of a worked calculation stated in the unit it computes in; and e-notation (`1e-9 m/s`),
     a tolerance written as a program writes it.
   The failure suggests a bracket that passes. The numbers a summary page traces to the report
   (#298) are read without these brackets, since each is its SI figure converted.
4. **The split.** M0.9c13 covers the guides: every page of the site but the model pages under
   `physics/`, the accuracy page, the validation plan and the records page, whose 632 figures
   are M0.9c16, queued after M0.9c15; #400 stays open for it. The check lists those pages
   as deferred and `cargo xtask site` prints how many figures they hold, so M0.9c16 is done
   when that list is empty.
5. **The file formats' pages are exempt.** A page under `format/` gives the numbers a file stores,
   in the file's units (OpenRocket's meters, ERA5's geopotential); a bracket there would describe
   the reader's units, not the file's.

**Alternatives considered.**

- **Draw the plot 720 px wide, to fit the column.** Rejected: the column is a measure for prose;
  the figure would still shrink on a narrower window, and the plot `hpr sim --plot` writes for a
  flyer's own screen or print would lose width to the site's layout.
- **A box that scrolls sideways around a wide figure.** Rejected: it cuts the figure at the box's
  edge, which the project's rule against anything cut off by an edge forbids, and hides part of it
  until scrolled.
- **An opt-out comment per paragraph or table for the units check.** Rejected for now: the
  exemptions above are rules a reader can predict; no guide needed another.
- **A feet column beside each SI column in tables.** Not needed: no guide's table had a unit only
  in its head with bare figures, which the check doesn't read; cells give brackets like prose.
- **A units switch on the site.** Waits for the app's units control (M9.1), as [ADR-210][adr-210]
  §3 decided for the command line.

**Consequences.** The audit's *web.md · Accessibility* row loses the plot's 9.4 px text; its
*data.md · Units for rocketry* row names the units check, and stays not met until M0.9c16. A new
guide page, or a new figure on one, fails the site check until its heights, distances and speeds
carry their US units. #382 closes with this increment.
