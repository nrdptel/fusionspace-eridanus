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
- **Numbers and dates**:

  | rule | write |
  |---|---|
  | a space before the unit, except degrees and percent | `21.5 °C`, `45°`, `7.0%` |
  | on the site and in SVG figures, that space is non-breaking (U+00A0), so a line never ends between a number and its unit; in terminal, JSON and CSV output it is a plain space, which copying and `grep` need ([`data.md`](https://github.com/nrdptel/fusionspace-design/blob/main/product/data.md#numbers)) | `21.5&nbsp;°C` in a page's source, `&#160;` in an SVG; `21.5 °C` in `hpr`'s output |
  | commas in groups of three in prose; none in terminal output or anything copied | `5,104 ft`; `1280 ft` |
  | precision that matches what is known | `5,104 ft` from a barometer, not `5,103.87 ft` |
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

Most of the product system's rules are not met yet. The
[design audit](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/design-conformance.md)
checks each of its sections against the site, the command line, the exports and the plot. Of
the 64 sections that apply to them today, 10 are met and 47 are not. Each gap has an issue, and
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
