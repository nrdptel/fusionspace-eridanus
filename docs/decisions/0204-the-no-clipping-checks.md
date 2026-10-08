# ADR-204: The no-clipping checks (2026-10-08)

- **Status:** accepted; carries out [ADR-195, the guides](0195-the-2026-10-07-guides-rocketry-explained.md) §6's last bullet, for M0.7a1
- **Summary:** M0.7a is split in five: the no-clipping checks first (M0.7a1), then the format with the trust guide, then stability, flutter and drag through Mach 1, one guide each. Three checks enforce Neer's rule that nothing is clipped: `xtask figures` holds every SVG to its viewBox and its clips, with text measured in the widest of a set of system fonts character by character; the same check fails text over text; `xtask site` loads every guide page in headless Chrome at each width from 320 to 1,920 px and fails sideways scroll, text past the left or right edge or cut by its box, and text over text. Pages built to fail run with every page check, and the figure check's tests move a committed figure's label out. The first run found four real clips, each fixed by layout.

**Context.** ADR-195 §6 asks that a figure's text or marks leaving its frame fail the build, and so
does a page that scrolls sideways at any width from 320 to 1,920 px. M0.7a's *done when* adds a
check for text over text and asks all three to run on the site's existing pages and on
`hpr sim --plot`. M0.7a as a whole (the format, four guides, three checks) is more than one
pull request, and every guide's figures depend on the checks, so they come first.

**Decision.**

1. **The split.** M0.7a1, the checks; M0.7a2, the format in `docs/writing.md` and the guide on how
   far to trust a simulation; M0.7a3 stability; M0.7a4 fin flutter; M0.7a5 drag through Mach 1,
   which closes M0.7a once Neer has read all four. M0.7a's own *done when* is unchanged.
2. **Figures** (`xtask/src/figures.rs`). The frame is the root `viewBox`, or `0 0 width height`.
   Transforms compose down the tree; a box is bounded by its transformed corners. Paths are
   bounded by their points and Bézier control points, an arc by its whole ellipse. Newlines in
   text draw as spaces; a negative size is malformed. Strokes add
   half their width, miter joins their miter length up to `stroke-miterlimit`, markers a circle
   reaching their viewport's farthest corner from the reference point. A one-rectangle
   `clip-path` is a frame of its own, since what leaves it is cut off, and a pattern's content
   must fit its tile. A clip on the root `<svg>`, whose coordinates browsers place differently,
   and a clip on a clip's own rectangle are refused. The check fails closed: an element, attribute, unit, font family or
   character it can't measure fails the figure.
3. **A conservative glyph width.** A figure shown through `<img>` can't load the site's fonts, so
   it draws in the reader's. Each character's advance and ink (above and below the baseline, and
   past its advance) is the largest over a set of common system fonts: Verdana, Tahoma, Arial,
   Helvetica, Helvetica Neue, Times, Times New Roman, Georgia, Trebuchet MS, SF Pro and DejaVu Sans and
   Serif for proportional text; Menlo, Monaco, Courier, Courier New, Andale Mono, PT Mono,
   SF Mono, Cascadia Mono, Consolas, Lucida Console and DejaVu Sans Mono for monospace; with
   Lucida Grande, Arial Unicode and Apple Symbols where those lack a character. Bold faces serve
   weights from 500, a face with none emboldened by em/24, FreeType's strength.
   `scripts/glyph-widths.py` reads them with fontTools and writes `figures/glyphs.rs`, whose
   header lists each file and version. Rerunning it needs the same fonts; the committed table is
   the record. Kerning and ligatures are left out: they almost always narrow a line, and the
   few pairs that widen one (Verdana's `f?` by 0.054 em) are covered only up to the largest
   overhang, added once per line.
4. **Text over text in a figure:** two text boxes may not overlap by more than 0.01 px each way.
5. **Pages** (`xtask/src/site/pages.rs`). A server on `127.0.0.1` serves the built site, since
   frames opened from `file://` can't be measured. Up to four headless Chrome processes, one tab
   each, as headless Chrome opens no more, load each page fresh into a frame at every width,
   with nothing remembered, so mdBook shows or hides its sidebar as for a first-time reader.
   After every font face loads (one that fails fails the page), a page fails where
   `scrollWidth > clientWidth` (the body's too); where text or a box passes the left or right
   edge outside a box that scrolls sideways (a drawer put away off the left, mdBook's sidebar on
   a narrow window, aside), since mdBook's
   `overflow-x: clip` hides that from `scrollWidth`; where a box that doesn't scroll cuts its
   text by more than 1 px (boxes of 2 px or less, which hide text for screen readers, aside);
   and where two runs of text overlap more than 2 px sideways and a quarter of the shorter one's
   height, which lets tight line heights and touching runs pass. The boxes that hold the page's
   `main` stand for the window: mdBook's `.content` scrolls (`overflow-y: auto` makes it scroll
   sideways too), so one wider inside than out fails; only boxes inside `main`, a code block or a
   table's wrapper, may scroll sideways. The rustdoc under `api/` is
   rustdoc's layout and isn't checked. Scrollbars stay on. On Linux, Chrome runs with
   `--no-sandbox`: it serves only the site just built, and Ubuntu 24.04's AppArmor stops the
   sandbox from starting on CI. Without Chrome the check fails, naming `HPR_CHROME`.
6. **Canaries.** Each run loads pages built to fail (a label past the edge, plainly and in a
   clipping box, and inside a `main` held by a box that scrolls, as mdBook's is; a label cut
   short; two labels on top of each other) and an ordinary page that must pass. If one isn't found as expected at every width, the check fails as broken. The
   figure check's tests move a committed figure's label out, and lay one over another.
7. **What the first run found, fixed by layout:** `--plot`'s hatch line sat on its tile's edge,
   and its series' 2 px strokes lost half their width at liftoff and at the altitude panel's
   floor; each panel's clip now reaches half a stroke past the axes. On every page from 320 to
   480 px, mdBook cut the title to "Fusio…"; below 520 px it now takes its own row. In tables,
   a superscript rose into the line above; cells now take the paragraphs' line height.

**Alternatives considered.**

- **A Chrome DevTools Protocol crate** to drive one browser. Rejected: a new dependency tree, when
  a static server and a script that posts its results back need only the standard library.
- **Resize one loaded page through every width.** Rejected: mdBook decides on its sidebar at load,
  so a resized page isn't what a reader at that width sees.
- **Average glyph widths, or the site's own fonts.** Rejected: an `<img>` figure can't use the
  site's fonts, and an average passes a label that a wider font clips.
- **Allow series to be clipped at the plot's panels.** Rejected: half a stroke hidden at the axis
  is clipping by Neer's rule, and growing the clip by half a stroke costs nothing.

**Consequences.**

- `cargo xtask site` needs Chrome or Chromium, as it needs mdBook; CI's `site` job has it, and the
  page check adds about 30 s on an M-series Mac.
- A guide's figure with a label in a font or character the table lacks fails until the table
  grows, by rerunning the script with that font.
- M0.7c's live figures reuse both checks at each input's extremes.
