# Writing these pages

**This page is the house style for FusionSpace HPR's documentation.** It is for anyone who writes or
edits a page of this site, a model page, or a doc comment. It applies to every new page and to
every page a change touches. Older pages move to it as they are edited, not all at once.

The aim is simple: a hobby rocketeer who knows some physics and some code, but not this project,
reads a page once and knows what it says and how far to trust it.

## The rules

1. **Bottom line first.** The first sentence of a page, a section or a paragraph says what it
   concludes. The detail and the reasoning come after.
2. **One idea a sentence.** Aim for about 20 words a sentence on average. A sentence over 25
   words is flagged for a second look; it is not forbidden.
3. **Short paragraphs.** Five sentences at most.
4. **Numbers carry their meaning.** Every number has a unit, and says what it is compared with:
   "apogee 1.2% above RocketPy's", not "1.2% error".
5. **One fact, one precision.** The same fact is written with the same precision everywhere it
   appears. If one page gives the real-flight apogee error as 6.04%, no other page rounds it to 6%.
6. **Labels are links.** An internal label, such as a milestone or a decision record, is a link
   with a few words of meaning, never bare. The site's checks enforce this one
   ([the documentation site decision record](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0016-the-documentation-site-mdbook-over-docs-and.md)).
7. **Words before equations.** Say in words what an equation does, give the equation, then work
   an example with real numbers.
8. **Honesty comes first.** These standing rules still apply: a page opens by saying how far to
   trust it; anything unvalidated says so in its first paragraph; example code runs in CI; and an
   accuracy number comes from the committed validation report
   ([Checking a claim](checking-a-claim.md)).

## The FusionSpace rules

FusionSpace HPR · Sim is a FusionSpace product, `FS-ACHERNAR · SW · TOOL 001` (Achernar, a star of project
Eridanus: [ADR-198](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0198-the-2026-10-07-project-eridanus.md)),
and its pages also follow the product system's [writing rules](https://github.com/nrdptel/fusionspace-design/blob/main/product/writing.md)
and [number rules](https://github.com/nrdptel/fusionspace-design/blob/main/product/data.md#numbers)
([the product system decision record](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0164-the-fusionspace-product-system.md)).
They don't conflict with the rules above; they add these:

- **US English**: color, center, meter, catalog, license, analyze, gray. A product or command
  name keeps the spelling it shipped with, and a quotation or a source's title keeps its own.
- **No em dashes.** Use a colon, a comma or a period. An en dash only in ranges: 5,100–5,500 ft.
- **Both are checked.** `cargo xtask spelling` flags the UK spelling of eight
  words, whose US forms are center, meter, catalog, license, color, analyze, behavior and
  modeling, and names the US word to write; it flags every em dash, in the pages and their code
  blocks, the crates, `xtask` and the schema alike (since
  [M0.6d3](decisions-and-roadmap.md#m0-6d3)). Other UK spellings (labelled, neighbours, maths,
  grey) are not checked yet. It reads the pages, the README, the scripts, the crates, `xtask` and
  the schema: a page's prose, each code block as its language reads (a `rust` block's comments
  and strings, a `text` block whole), and a source file's comments and strings. It skips
  addresses, the frozen decision records and its own word list.
- **Names are checked too** (since [M0.6e](decisions-and-roadmap.md#m0-6e)): every name in the
  code, a string that is one bare key such as `"center_x"`, and each name in the writing (in
  backticks, joined by `_`, in a format string). It reports `centre_of_pressure_m` as "call it
  `center_of_pressure_m`". A quotation, a source's title, a name that keeps its spelling, or a
  key that files saved before the rename or another program's format hold goes on the check's short
  list of exceptions, which it counts on every run. `--fix` replaces words but not names: a
  renamed name that is saved to a file keeps its old key as a serde alias, which HPR Sim reads
  and never writes. An em dash needs its sentence rewritten. CI runs the check in the site step.
- **A trust note**, in one fixed shape, on every result someone might fly on: a quote block that
  opens with `**How far to trust it.**`, then says what kind of figure it is (estimate,
  measurement, copied value); what it was checked against, with numbers; and what to rely on
  instead when it matters. Never a go/no-go verdict. The label appears nowhere else: a heading
  says what its section covers ("Accuracy of the margin"), and a qualifier goes in the note's
  first sentence. The site check fails any other use of the label, and a quote block that opens
  with a bold "not validated" sentence.
- **Signal words** are those of ANSI Z535, the US safety-sign standard, and no others: DANGER, WARNING, CAUTION, NOTICE, NOTE, each
  as a panel above the text with the hazard, the consequence and how to avoid it.
  NOTE carries a statement with no hazard. On the site, write one as a GitHub alert (a quote
  block whose first line is `[!NOTE]`, `[!CAUTION]` or `[!WARNING]`; DANGER and NOTICE have
  none yet, and `[!TIP]` and `[!IMPORTANT]` fail the build). `cargo xtask site` draws every
  alert and trust note as the product system's note, the signal word in a strip above the
  message, and fails any other `>` quote block: quote a source inside a sentence instead.
  `mdbook serve` shows plain quote blocks; `cargo xtask site` builds the pages as published.
- **Numbers and dates**:

  | rule | write |
  |---|---|
  | a space before the unit, except degrees and percent | `21.5 °C`, `45°`, `7.0%` |
  | on the site and in SVG figures, that space is non-breaking (U+00A0), so a line never ends between a number and its unit; in terminal, JSON and CSV output it is a plain space, which copying and `grep` need ([`data.md`](https://github.com/nrdptel/fusionspace-design/blob/main/product/data.md#numbers)) | `21.5&nbsp;°C` in a page's source, `&#160;` in an SVG; `21.5 °C` in `hpr`'s output |
  | commas in groups of three in prose; none in terminal output or anything copied | `5,104 ft`; `1280 ft` |
  | precision that matches what is known | `5,104 ft` from a barometer, not `5,103.87 ft` |
  | heights, distances and speeds in SI, then the US units in brackets, converted from the SI as written: feet for heights, rails and distances, inches for a part's size in millimeters or centimeters, miles for long distances across the ground; feet per second, or mph for the wind. A motor's or a mount's nominal size (the motor size a mount takes) stays in millimeters alone, as motors are named (`a 54 mm motor`), but a measured bore gives inches (`a 28.956 mm (1.140 in) bore`). Exempt: worked arithmetic, e-notation, and SI written as the conversion of a US figure before it (`1.52 in (38 mm)`); the site check fails the rest, on every page ([M0.9c16, US units on the model pages](decisions-and-roadmap.md#m0-9c16), [ADR-219](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0219-figures-at-their-size-and-us-units-on-the-guides.md)) | `1,400 m (4,593 ft)`, `16.2 m/s (53 ft/s)`, `5 m/s (11 mph)`, `0.6 m (24 in)` |
  | dates in prose; in tables and files | October 4, 2026; 2026-10-04 |
- **FusionSpace** is one word.
- **The product's name** ([ADR-199, the public name](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0199-the-2026-10-07-fusionspace-hpr.md) §1):
  "FusionSpace HPR" is the suite; "FusionSpace HPR · Sim" or "HPR Sim" is the simulator at its
  first mention on a page, "HPR Sim" or "the simulator" after it. Write "HPR Sim" wherever "the
  simulator" could mean OpenRocket, RocketPy or RASAero too: in comparisons, tables and column
  headers. "This project" or "FusionSpace HPR" is the project, its tests or its licenses. `hpr`, in code formatting, is the command
  and the Rust library, never the product in prose. Call the design format "the HPR design
  format". A heading someone links to says "the simulator", since "HPR Sim" in a heading makes
  an anchor that spells the old name.
- **The name is checked** (since [M0.9a1, the packages' text](decisions-and-roadmap.md#m0-9a1)):
  `cargo xtask names`, which `cargo test -p xtask` runs, fails on the old name, `hpr-sim`, in any
  case and with a non-breaking hyphen too, and on a lowercase `hpr` standing for the product in
  prose. It reads the text that ships in a package (the README, the crates' descriptions,
  READMEs, rustdoc and messages, the Python package, the schemas and their bindings, the
  licenses, the notices and the CHANGELOG) and, since
  [M0.9a2, the site's pages](decisions-and-roadmap.md#m0-9a2), every page of this site and its
  table of contents. A word joined to the name (`hpr-core`, `.hpr`, `hpr.fusionspace.co`) is
  another name, and in a message `hpr` followed by a subcommand is the command. Two names are
  never mentions: a decision record's file name, which keeps the name it was written under, and
  the flight engine's folder in a path, `crates/hpr-sim/`. Each problem prints as its file, line
  and rule, such as ``README.md:14: a bare `hpr` for the product: …``. A mention that must stay
  (history, an old stamp files still carry, an anchor) goes on the allowlist, `ALLOW` in
  `xtask/src/names.rs`: the file, the exact text around the mention (one line, at most 120
  bytes, at least 8 beyond the mention), how many mentions it covers, and why it stays. An entry
  that matches nothing, or a different number of mentions, fails. No page of this site may be
  allowed whole; only a decision record or the roadmap's archive may be. On the records page, a decision record's row, which gives the record's summary as it was
  written, inherits that record's allowance. A record up to
  [ADR-205, the decision that set the rule](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0205-the-2026-10-08-one-name-one-design.md)
  may be allowed whole, and its row with it; no other row or line there inherits anything.

Older pages move to these rules as they are edited. [M0.6](decisions-and-roadmap.md#m0-6), the
product system milestone, added the check for the first two. Nothing checks the non-breaking
space yet, and `hpr sim --plot`'s SVG figure, which the rule covers, still writes plain spaces.

Many of the product system's rules are not met yet. The
[design audit](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/design-conformance.md)
checks each of its sections against the site, the command line, the exports and the plot. Of
the 64 sections that apply to them today, 29 are met and 28 are not. Each gap has an issue, and
three milestones close them before the next guides are written: the command line, exports and
plot first, then the site's look, then the words
([ADR-208, the design audit](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0208-the-design-audit.md)).
None of these rules changes a number the simulator computes.

## Four kinds of page

Pages follow [Diátaxis](https://diataxis.fr/), a common way to sort documentation by what the reader is doing. Each page
is one kind; a page that mixes them is split when it is next touched.

| kind | the reader wants to | example on this site |
|---|---|---|
| Tutorial | learn by doing, start to finish | [Getting started](getting-started.md) |
| How-to guide | get one task done | [Launch-day weather](weather.md) |
| Reference | look a fact up | [The command line](cli.md) |
| Explanation | understand why | [How a flight is simulated](how-a-flight-is-simulated.md) |

## Nothing cut off

Nothing on this site may be clipped by an edge or a corner: no text, number, mark or button, in a
figure or on a page, at any window width. Three checks fail the build when something is
([the decision record](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0204-the-no-clipping-checks.md)
has the rules in full):

- **Figures stay in their frame.** `cargo xtask figures` reads every SVG under `docs/`, and
  `cargo xtask cli --check` draws each `hpr sim --plot` figure and checks it. Each line, shape,
  stroke, arrowhead and label must lie inside the figure's frame (its `viewBox`, the area the
  SVG file says it draws in) and inside any clip it sits in. A label is measured as if drawn in
  the widest of the common system fonts, character by character, because a figure shown as an
  image draws in whatever font the reader's computer has.
- **No text over text.** In a figure, two labels' boxes may not overlap by more than rounding.
  On a page, two runs of text fail when they overlap by more than 2 px sideways and a quarter of
  the shorter one's height; two lines set tightly, or two runs that touch, pass.
- **No page wider than the window.** `cargo xtask site` opens every page in headless Chrome at
  each width from 320 to 1,920 px, 40 px apart. A page fails if it scrolls sideways, if text or a
  box passes the window's left or right edge, or if a box cuts its own text short. A code block
  or a table may scroll sideways inside its own box.

CI runs the figure check in the tests on every system, and `cargo xtask site` in its `site` job;
`scripts/gate.sh test site` runs both locally. A failure names the page or figure, the widths,
and the label or element with how far it reaches. `cargo xtask site` also loads pages built to
fail, and fails if it stops finding them; the figure check's tests do the same with a committed
figure's label moved out or laid over another. Not covered: the API reference under `api/`, and
a reader whose system font is wider than every font the figure check measures.

When a check fails, change the layout: wrap the line, shorten the label, give it a row of its
own, or make the figure taller. Never crop, hide or shrink the frame to pass. The page check
needs Chrome or Chromium; if it isn't where the system usually keeps it, set `HPR_CHROME` to the
program's path.

### Figures in the system's colors and fonts

The same figure check holds every SVG under `docs/` and `theme/`, and each figure
`hpr sim --plot` draws (see [the command line](cli.md)), to the FusionSpace product system's
[foundations](https://github.com/nrdptel/fusionspace-design/blob/main/product/foundations.md).
Every color a figure paints is one of the system's color tokens, the named colors it lists by
job (*Semantic roles*): the light theme's, such as `ink` (`#0B0F1C`) for text and `predicted`
(`#A22488`) for a simulated line, and the state colors for chips and badges (`ok-fill`,
`caution-fill`, `danger-fill`, `info-fill`) with the text that goes on them. A figure shown as
an image can't follow the page's dark theme, so it is drawn on the light one, its own
background included. Four things fail:

- a color that isn't a token, written by name, as `currentColor`, or in a `style`;
- an opacity below 1 or a gradient, which blend tokens into colors none of them is;
- a shape or a line of text with no `fill` set on it or around it, which draws in black;
- text whose `font-family` doesn't start with Cascadia Mono (labels and numbers) or Archivo
  (prose).

An image can't load the site's fonts, so a reader without Cascadia Mono installed sees the next
family in the list, and the frame check measures every one of those. It can't measure Archivo
yet, so for now every figure sets its text in Cascadia Mono.

The check can't tell whether a line type is the right one, so the author picks it from the
system's table (*Lines and shape*): dashed (`8 4`), 2 px, for anything simulated; a chain line,
a long dash and a dot as on a drawing (`24 3 1 3`), 1 px, for a reference such as the ground;
dotted (`1 3`), 1 px, for an event. The figure on
[*How a flight is simulated*](how-a-flight-is-simulated.md) draws the dashed and chain lines
with a sample of each in its key; the plot from `hpr sim --plot` draws all three.

### The pages in the system's look

`cargo xtask site`'s page check, which opens every page at every width as described above, also
reads each page's computed style (the colors, sizes and outlines the browser ends up drawing),
so the site keeps the product system's look where mdBook, the program that builds the site,
would draw its own. It reads the light and navy themes, and the light one with scripts off.
Colors off the system's palette, rounded corners, shadows, motion off its timings, type off its
sizes, prose lines past 68 characters and gutters off 16 or 32 px all fail it. So do:

- text under the contrast that WCAG 2.2 AA (the W3C's Web Content Accessibility Guidelines,
  level AA) asks for against the background it is drawn on: 4.5 : 1, or 3 : 1 for text of
  24 px and over (18.66 px in bold), faded by any opacity between the text and that background;
- one control of each kind the keyboard reaches without the system's focus ring: the outline
  drawn round a link or button when Tab reaches it, 2 px wide in the action color (`#3350D6` on
  the light theme) and 2 px clear of its edge;
- what Windows' high-contrast mode (forced colors, which repaints the page in a few colors of
  the reader's own) would erase: a box set apart from what is behind it only by its fill, with
  no border there, or an icon not drawn in its text's color. The check can't switch the mode
  on; it applies the stylesheets' rules for it and judges from the page's own colors;
- a bar stuck to the top of the window reaching further down than the page's
  `scroll-padding-top`, the room the browser leaves at the top when it scrolls to what a reader
  tabs to, so the bar never hides it;
- a `>` quote block left on a page, or a note not drawn as the system's: a head, then the
  message; the head saying its kind's signal word in Cascadia Mono 12 px semibold capitals,
  across the note's top, set apart by a rule or a fill (a caution's in the caution fill, a
  warning's in the danger fill); the note inside a border on every side, 2 px on a warning;
- a font list without its metric-matched fallback second (a stand-in, Arial or Menlo, scaled to
  the real font's size, so text doesn't jump when the real font arrives), and a smooth scroll a
  script asks for when the menu bar's title is clicked;
- an icon in the menu bar, the page arrows, the search field or a code block's buttons that isn't
  square at one of the system's icon sizes, 16, 20 or 24 px (the site draws them at 20 and
  24 px; a note's icon, 14 px as the system's note draws it, isn't read), and selected text not
  drawn on the action color at 22 % opacity.

A file check fails a page whose head doesn't declare light and dark color schemes or doesn't set
the browser bar to the system's light canvas (`#F3F4F7`) on a light system and its dark one
(`#0B0F1C`) on a dark one, and a `fonts.css` that isn't the system's.

mdBook draws its buttons with Font Awesome, the icon set it ships, and names the book in an `h1`
above each page's own title. After mdBook builds the site, `cargo xtask site` draws each icon as
the system's icon of the same meaning and turns the book's name into a paragraph. A file check
then fails:

- a page with a Font Awesome icon, or an icon that isn't the system's;
- a page with a second `h1` (the print page, which holds every chapter, keeps one per chapter);
- a favicon that isn't the system's.

mdBook also names the book in text in its menu bar, shows the table of contents beside the page
only from 1,080 px with a menu button below that, and ends a page with its arrows. The build
redraws that frame:

- the menu bar becomes the system's header: the FusionSpace lockup (the logo's mark with the
  FusionSpace name beside it), 24 px tall in the text's color, then HPR, one link to the landing
  page;
- from 720 px the table of contents stays open and the menu button goes, with scripts off too;
- every page ends with the title block, the box of facts about a document an engineering drawing
  ends with ([title block](glossary.md#title-block)). Its entries are written once at the end of
  [Start here](start-here.md#title-block), and each page adds its own title after the site's.

A file check fails a page:

- without one `header` holding the lockup, or without the table of contents, its box or its
  menu button;
- with mdBook's script that opens the table of contents only from 1,080 px;
- without one `footer`, after the page's text, holding the title block with its fields in order
  and the page's own title.

The page check fails, at every width:

- a lockup not 24 px tall, not in the text's color, cut by its box, or closer than a quarter of
  its height to another icon or text in the header or to the window's side (the logo's clear
  space, the brand's rule);
- a menu button at 720 px or wider, or none below 720 px, with scripts on or off;
- a table of contents hidden at 720 px or wider (with scripts on or off, and after a narrower
  window is widened past 720 px); one hidden but still reached by the keyboard or a screen
  reader; one shown but not reached;
- a title block without its 2 px border in the text's color, with entries not in Cascadia Mono,
  or with anything below it.

Inside that frame the build lays each page out as an engineering drawing set is laid out:

- every page opens with an intro: the site's designation, `FS-ACHERNAR · SW · TOOL 001` (the
  product's number in the FusionSpace register, as the title block gives it), in a small tag with
  one corner cut off, over the page's title;
- each `##` section is a sheet: a 2 px rule in the text's color across its top, then its number,
  `SHEET 2 / 5`, in 12 px capitals over its name. On the print page, which holds every chapter,
  each chapter counts its own sheets. Write each `##` heading in the page's text, not inside
  another element, or the build fails;
- the header holds a theme switch, three buttons: Auto (the theme your system asks for),
  Light and Dark, the chosen one filled in. From 960 px it stands in the header; narrower, the
  header has no room for it and it stands at the top of the table of contents, which the menu
  button opens below 720 px. It needs scripts, so without them it isn't drawn. mdBook's own theme
  menu, with six themes for two looks, is gone; a reader who had saved one of the other four is
  put back on Auto.

A file check fails a page without the tag over its title, with a `##` heading outside a sheet or
sheets numbered out of order, without the switch in place of mdBook's theme menu, or with a
heading that skips a level going down (a `####` right after a `##`; headings go in order). The
page check fails, at every width:

- a tag not in Cascadia Mono 12 px, without its cut corner or 1 px border, or not above the title;
- a sheet without its 2 px rule in the text's color, or with its number missing, out of order,
  not in Cascadia Mono 12 px or not above its name; a `##` heading outside a sheet;
- a switch not in the header from 960 px or not in the table of contents below it (read with
  the table of contents opened below 720 px); with buttons other than Auto, Light and Dark, under
  44 px tall or not in Cascadia Mono 14 px; with a button other than Auto chosen for a first-time
  reader, or the chosen one not filled in the text's color; with mdBook's theme button drawn, the
  switch drawn with scripts off, or the page in one of mdBook's themes the switch doesn't offer.
  Once a run, at the width it reads print at, it presses Dark and then Auto, and fails a page
  whose theme doesn't follow; and it loads the page with mdBook's Coal saved, and fails one that
  doesn't forget it and press Auto.

Each page's own status and one-sentence lead in its intro, two to six sheets a page, and every
page opening with how far to trust it wait for
[M0.9d11, the pages' own words](decisions-and-roadmap.md#m0-9d11).

mdBook draws three icons as characters of text: `❱` beside a heading that folds open in the
table of contents, `✓` beside the chosen theme, and `»` before a heading a link jumped to. The
build draws the fold toggle as the system's chevron; the chosen theme is filled in on the theme
switch; a heading jumped to takes no mark, since the page
scrolls it to the top. The page check fails a text made only of such marks (arrows, technical
symbols, shapes, dingbats, guillemets, emoji and the private characters icon fonts draw in)
outside a page's `main`, or drawn by a stylesheet before or after an element anywhere. A key's
name in the help popup (`←`) and a mark used as a word in prose (`Mach → 0`) pass.

The check also reads each page as it would print, once, at the window width it tries nearest
the printable width of a sheet of paper (720 px: an A4 or Letter sheet inside Chrome's default
margins; CI splits the widths over three machines, and each reads print at its own nearest),
with the stylesheets' print rules applied and their screen-only rules set aside. A headless
browser can't print from a page, so this reads the print rules, not a printed page: where the
browser would break the pages isn't seen. In the light theme and in the navy one, it fails:

- a color that isn't the light theme's, and on a page with both themes, anything drawn in
  different colors from the two;
- navigation or a control drawn: the table of contents, the menu bar, a button or a text field;
- a link to another page, in the text or the title block, without its address after it, in
  parentheses (a page of this site shows its path from the page, such as `cli.html#units`);
- a table row, a note or the title block that can split across two sheets;
- a title block not drawn, or drawn without its version and date of issue.

`mdbook serve` still shows mdBook's icons, menu bar, theme menu and two `h1`, with no sheets,
and the landing page's title block as a table; only `cargo xtask site` builds the pages as published. Not read: text inside
SVG images or over a picture, and hover
([#432](https://github.com/nrdptel/fusionspace-eridanus/issues/432)). The pages' load speed,
Google's Core Web Vitals, is still unchecked until
[M0.9d10, the pages' vitals](decisions-and-roadmap.md#m0-9d10).

## Measured, not gated

Readability will be measured, not used to fail a build. A planned `cargo xtask` report will
print, for each page, the average sentence length, the share of sentences over 25 words, and how many
internal labels it holds per 100 words. The report will show where to look; a person decides
what to change.

One hard check is planned, and may land in the bookkeeping clean-up
([M0.5 milestone](decisions-and-roadmap.md#m0-5)). A model page's *In short* box, the bulleted
summary that opens it (what it models, its sources, how well it is validated, what it leaves
out), will hold at most about 150 words and five bullets.

The targets for user pages, measured at Neer's next review: at most 20 words a sentence on
average, and fewer than 10% of sentences over 25 words.

## Where this comes from

The rules follow the UK Government Digital Service's
[style guide](https://www.gov.uk/guidance/style-guide), and the sentence-length checks that
Datadog and GitLab run with the [Vale](https://vale.sh/) linter. The page kinds are Daniele
Procida's [Diátaxis](https://diataxis.fr/) framework. The decision to adopt them is in the
[2026-10-03 review's decision record](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0144-the-2026-10-03-planning-review-leaner-bookkeeping.md).
