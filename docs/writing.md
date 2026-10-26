# Writing these pages

Learn the house style that every page and doc comment on this site follows, for its words, its
layout and its design.

**This page is the house style for FusionSpace HPR's documentation.** It's for anyone who writes or
edits a page of this site, a model page, or a doc comment. It applies to every new page and to
every page a change touches. Older pages move to it as they're edited, not all at once.

The aim is simple. A hobby rocketeer who knows some physics and some code, but not this project,
reads a page once and knows what it says. They also know how far to trust it.

> **How far to trust it.** This page is rules, not results. The build holds some of them on every
> change, and reviews hold the rest. Where this page and a check disagree, the check is what the
> build enforces, and the gap is a bug to report.

`cargo xtask spelling` reads each page's words and names, and counts its exceptions on every
run. `cargo xtask voice` fails a stock phrase and counts the voice section's patterns against
its limits. `cargo xtask guides` reads every number in a guide's text from a committed report.
`cargo xtask site` counts each page's sheets and fails one outside two to six. It also fails a
trust note or an alert off its shape, a hyphen for a minus sign, a column of numbers not
right-aligned or without a label and a unit, and a note or table cell over 120 words.

## The rules

1. **Bottom line first.** The first sentence of a page, a section or a paragraph says what it
   concludes. The detail and the reasoning come after. On a page where a person is talking, the
   reasoning is in the maintainer's own words ([Voice](#voice)).
2. **One idea a sentence.** Aim for about 20 words a sentence on average. A sentence over 25
   words is flagged for a second look; it is not forbidden.
3. **Short paragraphs.** Five sentences at most.
4. **Numbers carry their meaning.** Every number has a unit, and says what it is compared with:
   "apogee 1.2% above RocketPy's", not "1.2% error".
5. **One fact, one precision.** The same fact is written with the same precision everywhere it
   appears. If one page gives the real-flight apogee error as 6.06%, no other page rounds it to 6%.
6. **Labels are links.** An internal label, such as a milestone or a decision record, is a link
   with a few words of meaning, never bare. The site's checks enforce this one
   ([the documentation site decision record](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0016-the-documentation-site-mdbook-over-docs-and.md)).
7. **Words before equations.** Say in words what an equation does, give the equation, then work
   an example with real numbers.
8. **Honesty comes first.** These standing rules still apply. A page opens by saying how far to
   trust it. Anything unvalidated says so in its first paragraph. Example code runs in CI, and an
   accuracy number comes from the committed validation report
   ([Checking a claim](checking-a-claim.md)).

### Equations

Equations are typeset, as a textbook sets them. You write them in LaTeX between dollar signs, and
they become MathML when the site builds. They're drawn in New Computer Modern Math, the face of
LaTeX's own textbooks (the product system's *Type* rule; [the decision](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0264-typeset-equations.md)).

GitHub shows the same source as math too. A formula the converter can't read fails the build, with
its page and line. `cargo xtask site-math --check docs/<page>.md` converts a page's formulas as the
build does. It notes any formula long enough to risk a phone's column.

- **In a sentence,** `$…$`: "the drag force $F_D$ grows as $v^2$". A symbol the text names is
  math too, so the sentence and the equation match: $\rho$, not ρ or `rho`. Keep an inline
  formula short, a few symbols. It can't wrap, and one wider than a phone's column fails the page
  check. Inline, write a fraction with a slash, $(\lambda + 1)/2$; only a number's fraction is
  stacked, $\tfrac12$.
- **On its own,** `$$…$$` as its own paragraph, with a blank line above and below. Use it for an
  equation the page is about, or anything longer than a few symbols. It is centered, unnumbered;
  the text names it.

  It must fit a 320 px phone, about 30 characters of math. Write a longer one on several lines
  with `\begin{aligned}`, each line broken before `=` or an operator and aligned at `&=`. A
  product written side by side breaks with no operator, never an invented `\times`. Never shrink
  or scroll one.

  In a list item, indent the `$$` lines to the item's text. A line right under a display fails
  the build, since it would show as raw text.

  ```text
  $$
  \begin{aligned}
  q &= \tfrac12 \rho v^2, \\
  D &= q\, C_D A
  \end{aligned}
  $$
  ```

- **Notation,** after ISO 80000-2. One-letter variables are italic (the default). Words, names of
  more than one letter, units and the differential d are upright: `\mathrm{Re}`, `\mathrm{d}t`,
  `9.81\,\mathrm{m/s^2}`. Vectors are bold, `\mathbf{v}`, never `\vec`, whose arrow browsers
  misplace (the build refuses it).

  A one-letter subscript that labels stays italic, as these pages have always written it ($C_D$,
  $c_r$). A word or an abbreviation is upright ($T_{\mathrm{base}}$, $x_{\mathrm{CG}}$). A script
  inside a script would be under 12 px: write $C_{D0}$, not `C_{D_0}`, and $\exp(-x^2/2)$, not
  `e^{-x^2/2}`.
- **Numbers:** a value tied to a symbol goes in the math, its unit upright,
  $g = 9.81\,\mathrm{m/s^2}$. A value in a sentence stays in words and plain units, as every other
  number does. A power of ten is $10^{-6}$ in math, and 1e-6 only where a program reads or prints
  it.
- **Never in a heading** or an image's description: the build refuses math there.
- **Not math:** code, field names, file names, command lines and program output stay in code,
  `mass_kg`, `hpr sim`. A quoted reading keeps its unit in words, "a top speed of 355 m/s
  (1,165 ft/s)". Doc comments and terminal output keep plain Unicode, `F = ½ ρ v² C_D A`, which
  reads better there than LaTeX source. A dollar sign that isn't math is written `\$`.

## Voice

Write as one person explaining their own work to another flyer: plain words, short sentences,
each thing said once. These pages are one maintainer's project, and they should read that way.

A read-through on October 18, 2026 found them correct but generic. Bullets opened with a bold
label, lists of three all looked alike and caveats were stacked on caveats. Pages opened with
"FusionSpace HPR is a ..." where a person would say why they built it
([the decision record](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0284-the-maintainers-own-voice.md)).

Each rule here comes from that read-through or from
[a survey of plain technical writing](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/plain-voice-docs.md).
Each shows a passage from these pages as it stood on October 19, 2026, then the same passage in
this voice.

### Where "I" goes

Use "I" where a person is talking: the README, the landing pages, the pages that say why
something exists, the guides and the release notes. It's one person's project, so there's no
company "we". The bottom line still comes first ([rule 1](#the-rules)), and the why follows it,
in the maintainer's words.

Reference pages stay impersonal: a model's equations, a file format, a command's options, the
glossary. There the program is the subject, and "you" gives the steps, as it does in a tutorial
or a how-to.

*Before*, in [the contributing guide](https://github.com/nrdptel/fusionspace-eridanus/blob/main/CONTRIBUTING.md):
"Publishing packages, creating releases and tags, and changing repository settings are the
maintainer's to do."

*After*: "Only I publish packages, create releases and tags, or change the repository's settings."

### Plain words

Use the plain word. Leave out the words that tell a reader something is "easy" or "quick": what's
"quick" for you may not be for them. Never use a stock phrase. The check holds a list of them and
fails the build on one ([the check](#the-check)).

*Before*, in [Pick a motor](pick-a-motor.md): "Flying a few more motors is quick: change `--motor`
and `--delay` and run again."

*After*: "To fly a few more motors, change `--motor` and `--delay` and run it again."

### Short sentences

Aim for about 20 words a sentence ([rule 2](#the-rules)), and use contractions where you'd say
them: "isn't", "doesn't", "it's". A sentence that carries five things is better as a few
sentences, or as a list.

*Before*, in the README: "Flight in six degrees of freedom (6-DOF: three of position, three of
rotation), with variable mass: aerodynamics from subsonic to supersonic, the standard atmosphere or
a measured one, winds that change with height, staging, clusters and air starts, and recovery with
drift."

*After*: "It flies a rocket in six degrees of freedom (6-DOF), three of position and three of rotation,
with its mass changing in flight. The aerodynamics run from subsonic to supersonic. The air is the
standard atmosphere or a measured one, with winds that change with height. It covers staging,
clusters and air starts, and recovery with drift."

### Say it once

Say a thing once, where it belongs. Don't announce what a section covers, point back to what was
said ("as mentioned"), or recap it at the end (`In summary`). The heading and the first sentence
already say what a section is about.

*Before*, in [the OpenRocket format page](format/ork.md): "This section compares HPR Sim's whole
flights with OpenRocket 24.12's on the 54 configurations of OpenRocket's example designs that HPR
Sim can fly, in 15 of the examples."

*After*: "HPR Sim's whole flights are compared with OpenRocket 24.12's on 54 configurations. They're
the configurations of OpenRocket's example designs that HPR Sim can fly, in 15 of the examples."

### Paragraphs and lists

Write a thought as a paragraph. Use a list for what really is a list: steps, options, parts. Give
a list as many items as there are, not three of the same shape because three sounds finished.

Open an item with a bold label only where a reader scans for it, as in a list of options. A model
page's *In short* box is another such place. Bold-label items count toward the limit, except the
four labels of a model page's *In short* box ([below](#measured-not-gated)). A line that opens with a bold label reads
like a form, as this pair shows.

*Before*, in the README: "**Status: RELEASED.** Version 0.1.0, released October 8, 2026; work toward
0.2 goes on here."

*After*: "Version 0.1.0 came out on October 8, 2026, and work toward 0.2 goes on here."

### Labels

Say what a thing is before its label. Use a label (a milestone or a decision number) only where
the reader needs to follow it. A sentence that opens with one sends the reader to look it up
before they know why.

*Before*, in [Accuracy](accuracy.md): "Neither set is held out: both were seen during development,
so neither is a blind test until [M2.7](decisions-and-roadmap.md#m2-7) holds flights out, predicted
before their logs are read."

*After*: "Neither set is held out. Both were seen during development, so neither is a blind test.
That waits for the planned milestone that holds flights out, predicted before their logs are read
([held-out flights](decisions-and-roadmap.md#m2-7))."

### Honest, once

Honesty stays exactly as strict ([rule 8](#the-rules)): what was checked, against what, and where
it stops. Say it once and plainly, like someone who checked. Put a hedge on the claim it
qualifies, not on every clause around it.

*Before*, in [the simulator's tour](simulator-tour.md): "Here the drag may read high, so the apogee
may read low: for a field's ceiling, the side that flatters ([Trusting a
simulation](trusting-a-simulation.md#which-miss-flatters))."

*After*: "Here the drag may read high. If it does, the apogee reads low, the side that flatters a
field's ceiling ([Trusting a simulation](trusting-a-simulation.md#which-miss-flatters))."

### The limits

`cargo xtask voice` counts these on each page, per 100 words, and its report marks a page over a
limit. A page over one is where to look first. On a page that isn't held, it doesn't fail the
build ([the check](#the-check)).

| Counted | What it counts | Limit, count per 100 words |
|---|---|---:|
| Bold-label items | a list item that opens with bold text, less the four labels of a model page's *In short* box | 0.5 |
| Labels | a milestone, decision or lesson number, linked or not | 0.5 |
| Sentences over 25 words | in paragraphs and list items | 0.5 |
| Hedges | may, might, could, possibly, perhaps, probably, likely, seems, appears and the like | 1.0 |
| Stacked hedges | a sentence holding two or more hedges | 0.1 |
| Minimizers | simply, just, easy, easily, quick, quickly | 0.1 |
| Signposts | "in this section", "as mentioned", "let's", "in short," and the like | 0.1 |
| First person on a reference page | "I", "me", "my" and their forms (I'm, mine, myself) on a model, format, command or glossary page | 0 |

Hedges leave out a capital "May" and "could not". Minimizers leave out "just" before one of
sixteen words of place or time ("just below", "just after"). They also leave out "how quick" and
"how quickly", which measure something here.

First person counts a capital I before a lower-case word other than "is". It doesn't count one
after a word that makes it a numeral or a name ("Type I"), or before a motor word ("an I motor").
So it leaves out "where I is the inertia", and an I at a clause's end or before a comma ("so did
I"). It still counts a symbol I before another word ("the current I flowing").

A quotation inside one sentence isn't read for hedges, minimizers, signposts, first person or who's
talking, though the stock list reads it.

The sentence limit comes from the target this guide already set. That's fewer than 10% of
sentences over 25 words, at about 20 words a sentence, or 0.5 per 100 words. On a page of
shorter sentences the limit is stricter than 10%. On a page with many words in headings and
tables it's looser, since those words count but hold no sentences. So the report prints each
page's share too.

A bold-label limit of 0.5 still allows a list of ten options on a 2,000-word page. It doesn't
allow a bold label on every item of every list. Labels get one every 200 words, because each one
sends the reader off to look it up.

Hedges get 1.0, because a single hedge is often the honest word. That's about the top of the
range the survey measured on one writer's blog: 2.2 to 11.9 per 1,000 words. That count used a
mostly different list of hedges.

Stacked hedges, minimizers and signposts get one every 1,000 words. Each adds little: a second
hedge, a word that calls a step easy, a sentence that points back at the page. Use one only on
purpose.

First person on a reference page gets 0, because the program is the subject there. The other
limits were set against these pages as they stood on October 19, 2026. Most of them were then
over the sentence limit, and about half over the bold-label limit. The report gives the current
counts.

Who's talking (the report's register) has no limit of its own, since the right amount depends on
the kind of page. Only first person on a reference page is held to one, above. The report prints
it per page: "I", "we", "you" and contractions per 100 words.

For scale, the survey found 1.0 to 3.5 contractions per 100 words on most personal pages. On
reference pages it found 0 to 0.25. One author wrote about 3 "I" per 100 words on a blog and
none on the project's reference pages.

### The check

`cargo xtask voice` reads this repository's pages, writes
[the report](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/voice.md)
and fails on a stock phrase. With no path, it also fails when a held page goes over a limit or
misses a sentence target ([below](#measured-not-gated)). A test fails when the report and the
pages disagree, so run it after editing a page. The site's build runs the check too.

Give it paths, such as `cargo xtask voice ../fusionspace-site/site`, and it reads those instead
and prints the report. It reads Markdown, HTML and text, a command's help in a `.help` file, and
a crate's front page in a `.rs` file. That's how
fusionspace.co's pages and the design
system's get checked. In the report, a rate in bold is over its limit, and `ref` marks a
reference page. With `--long` first, it lists each sentence over 25 words in the prose instead, with its file and line.

On any page, the stock-phrase list fails a build. It holds 41 entries in 9 groups. Each
is named by a published list or by the design system's own writing rules. The lists are style
guides, studies, and Wikipedia's pages on word choice and on signs of generated writing:

- puffery: `seamless`, `cutting-edge`
- inflated words: `pivotal`, `testament`
- words that spread with generated text: `delve`, `showcase`
- buzz verbs: `leverages`, `utilize`
- note-that phrases: `please note`
- vague sources: `research has shown`
- wordy phrases: `in order to`, `a number of`
- recaps: `in conclusion`
- chat residue: `I hope this helps`

The list leaves out words that have a technical sense in this project. They are robust, deploy,
streamline (a streamlined body), boost and powerful (a more powerful motor). Terms such as
`magic bytes` and `seamless tube` pass. The list is in
[`xtask/src/voice.rs`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/xtask/src/voice.rs),
each entry with its sources and a test that fails without it.

It reads the README, the changelog, the contributing guide, the pull request and issue templates
(less the guidance in their comments) and `docs/`. It leaves out:

- the decision records and the done-work archive, which are history;
- the records pages, written from the decision records and the roadmap, and the roadmap itself,
  the open work;
- the research notes, which keep their wording until they're next edited;
- the site's table of contents, which is links.

It counts what a program can count. Voice is partly taste, so a cold reader checks each rewritten page
against these rules too.

### Measured, not gated

Every site page is held: each page the site's table of contents lists, and the README and the
404 page beside them. The one exception is the decisions and roadmap page, a record written from
the roadmap and the decision records. On a held page, one rate over its limit, or one sentence
target missed, fails the build. The report marks those pages `held`. A page added to the table of
contents is held from its first commit
([the plan that rewrote the site](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0285-the-rest-of-the-site-in-nine.md)).

The command-line tool's help is held the same way. A test pins each command's `--help` in a
snapshot under `crates/hpr-cli/tests/help/`, and the check holds every snapshot. It reads only the
help's prose: option names, notes such as `[default: 0]` and example commands are code to it, so a
default that needs words goes in a sentence. Change a command's help and the test fails until you
regenerate the snapshots with
`HPR_BLESS=1 cargo test -p fusionspace-hpr-cli --test help`
([the plan for the program's words](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0292-the-programs-words-in-four.md)).

Every command's messages are held too: what `hpr sim`, `weather`, `analyze` and the others
print. The source doesn't say which string ends up in a message, so the check reads a census
of the code. It reads `hpr-cli`, `hpr`, the twelve crates under it and the Python package's
`hpr-py`: every `#[error]` text and every other string that reads as prose, less tests and
markup. Each crate is one held reference page. A
placeholder such as `{path}` is one code word. A joined message is read in pieces, so a long one
can pass. `cargo xtask voice --long --messages` lists each long sentence with its file and line, and
`--messages` the whole census; the [voice report](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/voice.md) counts what it read.

The Python package's docs and the crates' front pages are held too. A test of the package writes
its docstrings, as `help()` shows them, to `crates/hpr-py/tests/docstrings.md`. The check holds
that snapshot as a reference page, and the package's README beside it. Change a docstring and
the test fails until you write the snapshot again with `HPR_BLESS=1 scripts/python-tests.sh 3.13`.
Each crate's front page is the `//!` docs at the top of its `src/lib.rs`, the page docs.rs shows.
The check holds each one as a reference page, and refuses a crate that has none.

On the other pages the check reads, such as the changelog and the vision, readability is
measured, not used to fail a build. The counts show where to look, and a person decides what to
change.

The site also holds a model page's *In short* box to its shape. It's four bulleted items, each
opening with its label in bold: `**What it models:**`, `**Sources:**`, `**How well it is
validated:**` and `**What it leaves out:**`, in that order. A reader scans the box, and on a short
page its four labels alone would exceed the limit. So on a box of that shape the check doesn't
count those four labels as bold-label items. A box the site refuses keeps them in the count, and
any other bold label on a model page counts.
A cap of about 150 words on the box was planned and isn't built yet.

The targets for user pages are at most 20 words a sentence on average and fewer than 10% of
sentences over 25 words. The report prints both for every page.

## The FusionSpace rules

FusionSpace HPR · Sim is a FusionSpace product, `FS-ACHERNAR · SW · TOOL 001` (Achernar, a star
of [project Eridanus](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0198-the-2026-10-07-project-eridanus.md)).
Its pages also follow the product system's [writing rules](https://github.com/nrdptel/fusionspace-design/blob/main/product/writing.md)
and [number rules](https://github.com/nrdptel/fusionspace-design/blob/main/product/data.md#numbers)
([the product system decision record](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0164-the-fusionspace-product-system.md)).
They don't conflict with the rules above; they add these:

- **US English**: color, center, meter, catalog, license, analyze, gray. A product or command
  name keeps the spelling it shipped with, and a quotation or a source's title keeps its own.
- **No em dashes.** Use a colon, a comma or a period. An en dash only in ranges: 5,100–5,500 ft.
- **Both are checked.** `cargo xtask spelling` flags each British spelling on its list and names
  the US word to write. Since [M0.6d3](decisions-and-roadmap.md#m0-6d3), that's center, meter,
  catalog, license, color, analyze, behavior and modeling. Since
  [M0.9e](decisions-and-roadmap.md#m0-9e) (the words to the design), it's also:

  - caliber, fiber, liter and maneuver;
  - labeled, traveled, signaled, fueled and the other doubled l's;
  - favor, neighbor, vapor, honor and the other -our words;
  - defense, aluminum, gray, judgment, sulfur, aging and program;
  - the endings -ize, -izing and -ization for -ise, -ising and -isation, on the stems on its list
    (normalize, optimization, summarizing).

  It flags every em dash, in the pages and their code blocks, the crates, `xtask` and the schema
  alike. It reads the pages, the README, the scripts, the crates, `xtask` and the schema. In a
  page it reads the prose, and each code block as its language reads: a `rust` block's comments
  and strings, a `text` block whole. In a source file it reads the comments and strings. It skips
  addresses, the frozen decision records and its own word list.
- **Names are checked too** (since [M0.6e](decisions-and-roadmap.md#m0-6e)). The check reads every
  name in the code, and a string that is one bare key such as `"center_x"`. It reads each name in
  the writing too: in backticks, joined by `_`, or in a format string.
  It reports `centre_of_pressure_m` as "call it `center_of_pressure_m`".

  The check keeps a short list of exceptions, which it counts on every run. An
  exception is a quotation, a source's title or a name that keeps its spelling. It can also be a
  key held by files saved before the rename, or by another program's format.

  `--fix` replaces words but not names. A renamed name that is saved to a file keeps its old key
  as a serde alias, which HPR Sim reads and never writes. An em dash needs its sentence
  rewritten. CI runs the check in the site step.
- **Numbers on the site are checked** (since [M0.9e](decisions-and-roadmap.md#m0-9e)). Outside
  code, `cargo xtask site` fails a hyphen-minus written for a minus sign, `-7.28%`. The page
  should have the real one, `−7.28%` (U+2212).

  It also fails a table column whose cells are all numbers, unless its delimiter right-aligns it,
  `---:`. A number here is a figure with its units, sign, spread or range, such as
  `1,400 m (4,593 ft)`, `−7.28%` or `−29 to −34%`. A cell in code, a date or a version is a label,
  and so is any cell under a head that says "version". A cell with any other word makes its column a
  text column. An empty cell, a dash, "n/a" or "none" doesn't count.

  Since [M0.9f2](decisions-and-roadmap.md#m0-9f2), `cargo xtask site` also reads the head of
  each right-aligned column, on the site and in both READMEs, as a label and then a unit:

  - **The label** is a word of two or more letters outside code and math that isn't a unit, before
    the head's first bracket, comma, `·` or `;`. A code identifier alone (`apogee_agl_m`), a figure
    or a unit alone (`(kg)`) is no label.
  - **The unit** goes where a unit goes: the first word in brackets, after a comma, `·` or `;`, or
    after "in" (`time (s)`, `Worst time, s`, `CP, calibers aft of the tip`, `Offset in bytes`), or
    "per" and what it's per (`slope, per rad`). A unit word elsewhere in the label doesn't count:
    `Error in apogee` gives none.
  - **A pure number** names its kind as the label's last word: a coefficient, ratio, fraction,
    share, factor, number, count, code, or Mach (`drag coefficient`, `Reynolds number`). The first
    five also serve where a unit goes (`(coefficient)`). One of no kind says `(no unit)`.
  - **Or each value carries its unit** right after its figure (`0.4 m`, `+1.2%`); a bracket there,
    `3.80 (+73.2%)`, is another quantity's.
  - **A count** has whole numbers and a head that names what it counts, with a plural, `count` or
    `number`, in its label or where a unit goes (`Flights`, `Files differing`).

  A head that ends in a figure (`| Mach 0.9 |`) names no kind and no unit, so it fails unless
  each value carries its unit. It is a value, so the table is on its side: turn it.
  The unit words the check knows are `UNITS` and `SPELLED_UNITS` in `xtask/src/site/numbers.rs`.
- **Short notes and cells** (since [M0.9f2](decisions-and-roadmap.md#m0-9f2)). `cargo xtask site`
  fails a note (a whole quote block) or a table cell over 120 words, on the site and in both
  READMEs. Code, link text and HTML's text count. A note keeps what the reader must know first:
  for a trust note, its kind of figure, what it was checked against with numbers, and what to rely
  on instead. The supporting detail goes in the page's own text, below the note. A cell holds a value or a
  phrase, and its explanation goes under the table. A table or a quote block written in raw HTML
  fails too, as the checks read only Markdown's. A footnote is neither, and isn't read.
- **A trust note**, in one fixed shape, goes on every result someone might fly on. It's a quote
  block that opens with `**How far to trust it.**`. Then it says what kind of figure it is
  (estimate, measurement, copied value) and what it was checked against, with numbers. Last, it
  says what to rely on instead when it matters. Never a go/no-go verdict.

  The label appears nowhere else: a heading says what its section covers ("Accuracy of the margin"),
  and a qualifier goes in the note's first sentence. The site check fails any other use of the
  label, and a quote block that opens with a bold "not validated" sentence.
- **Signal words** are those of ANSI Z535, the US safety-sign standard, and no others: DANGER,
  WARNING, CAUTION, NOTICE, NOTE. Each is a panel above the text with the hazard, the consequence
  and how to avoid it. NOTE carries a statement with no hazard.

  On the site, write one as a GitHub alert: a quote block whose first line is `[!NOTE]`,
  `[!CAUTION]` or `[!WARNING]`. DANGER and NOTICE have none yet, and `[!TIP]` and `[!IMPORTANT]`
  fail the build. `cargo xtask site` draws every alert and trust note as the product system's note,
  the signal word in a strip above the message. It fails any other `>` quote block: quote a source
  inside a sentence instead. `mdbook serve` shows plain quote blocks; `cargo xtask site` builds the
  pages as published.
- **Numbers and dates**:

  | rule | write |
  |---|---|
  | a space before the unit, except degrees and percent | `21.5 °C`, `45°`, `7.0%` |
  | on the site and in SVG figures, that space is non-breaking (U+00A0), so a line never ends between a number and its unit; in terminal, JSON and CSV output it is a plain space, which copying and `grep` need ([`data.md`](https://github.com/nrdptel/fusionspace-design/blob/main/product/data.md#numbers)) | `21.5&nbsp;°C` in a page's source, `&#160;` in an SVG; `21.5 °C` in `hpr`'s output |
  | commas in groups of three in prose; none in terminal output or anything copied | `5,104 ft`; `1280 ft` |
  | precision that matches what is known | `5,104 ft` from a barometer, not `5,103.87 ft` |
  | heights, distances and speeds in SI, then the US units in brackets, converted from the SI as written: feet for heights, rails and distances, inches for a part's size in millimeters or centimeters, miles for long distances across the ground; ft/s, or mph for the wind. A motor's or a mount's nominal size (the motor size a mount takes) stays in millimeters alone, as motors are named (`a 54 mm motor`), but a measured bore gives inches (`a 28.956 mm (1.140 in) bore`). Exempt: worked arithmetic, e-notation, and SI written as the conversion of a US figure before it (`1.52 in (38 mm)`); the site check fails the rest, on every page ([M0.9c16, US units on the model pages](decisions-and-roadmap.md#m0-9c16), [ADR-219](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0219-figures-at-their-size-and-us-units-on-the-guides.md)) | `1,400 m (4,593 ft)`, `16.2 m/s (53 ft/s)`, `5 m/s (11 mph)`, `0.6 m (24 in)` |
  | dates in prose; in tables and files | October 4, 2026; 2026-10-04 |
- **FusionSpace** is one word.
- **The product's name** ([ADR-199, the public name](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0199-the-2026-10-07-fusionspace-hpr.md) §1):
  "FusionSpace HPR" is the suite. "FusionSpace HPR · Sim" or "HPR Sim" is the simulator at its
  first mention on a page, and "HPR Sim" or "the simulator" after it. Write "HPR Sim" wherever
  "the simulator" could mean OpenRocket, RocketPy or RASAero too: in comparisons, tables and
  column headers. "This project" or "FusionSpace HPR" is the project, its tests or its licenses.

  `hpr`, in code formatting, is the command and the Rust library, never the product in prose.
  Call the design format "the HPR design format". A heading someone links to says "the
  simulator", since "HPR Sim" in a heading makes an anchor that spells the old name.
- **The name is checked** (since [M0.9a1, the packages' text](decisions-and-roadmap.md#m0-9a1)).
  `cargo xtask names`, which `cargo test -p xtask` runs,
  fails on the old name, `hpr-sim`, in any case and with a non-breaking hyphen too. It also fails
  on a lowercase `hpr` standing for the product in prose.

  It reads the text that ships in a package. That's the README, and the crates' descriptions,
  READMEs, rustdoc and messages. It's also the Python package, the schemas and their bindings, the
  licenses, the notices and the CHANGELOG. Since [M0.9a2, the site's
  pages](decisions-and-roadmap.md#m0-9a2), it also reads every page of this site and its table of
  contents.

  A word joined to the name (`hpr-core`, `.hpr`, `hpr.fusionspace.co`) is another name, and in a
  message `hpr` followed by a subcommand is the command. Two names are never mentions. One is a
  decision record's file name, which keeps the name it was written under. The other is the
  flight engine's folder in a path, `crates/hpr-sim/`. Each problem prints as its file, line and
  rule, such as ``README.md:14: a bare `hpr` for the product: …``.

  A mention that must stay (history, an old stamp files still carry, an anchor) goes on the
  allowlist, `ALLOW` in `xtask/src/names.rs`. An entry gives the file and the exact text around
  the mention: one line, at most 120 bytes, at least 8 beyond the mention. It also gives how many
  mentions it covers, and why it stays. An entry that matches nothing, or a different number of
  mentions, fails.

  No page of this site may be allowed whole. Only a decision record or the roadmap's archive may
  be. On the records page, a decision record's row, which gives the record's summary as it was
  written, inherits that record's allowance. A record up to
  [ADR-205, the decision that set the rule](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0205-the-2026-10-08-one-name-one-design.md)
  may be allowed whole, and its row with it. No other row or line there inherits anything.

Older pages move to these rules as they are edited. The product system milestone,
[M0.6](decisions-and-roadmap.md#m0-6), added the check for the first two. On the site, nothing
checks the non-breaking space yet. `hpr sim --plot`'s SVG figure, which the rule covers, joins
each number to its unit with one, and a test holds it (`a_number_keeps_its_unit_on_its_line` in
`crates/hpr-cli/src/plot/tests.rs`).

Many of the product system's rules are not met yet. The
[design audit](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/design-conformance.md)
checks each of its sections against the site, the command line, the exports and the plot. Of
the 64 sections that apply to them today, 37 are met and 20 are not.

Each gap has an issue and the milestone that closes it. The command line, exports and plot came
first, then the site's look. Then came the spelling, minus signs, number columns and READMEs.
The rest of the words ([M0.9f](decisions-and-roadmap.md#m0-9f)) comes before release 0.2
([ADR-208, the design audit](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0208-the-design-audit.md)).
None of these rules changes a number the simulator computes.

## Four kinds of page

Pages follow [Diátaxis](https://diataxis.fr/), a common way to sort documentation by what the
reader is doing. Each page is one kind. A page that mixes them is split when it's next touched.

| kind | the reader wants to | example on this site |
|---|---|---|
| Tutorial | learn by doing, start to finish | [Getting started](getting-started.md) |
| How-to guide | get one task done | [Launch-day weather](weather.md) |
| Reference | look a fact up | [The command line](cli.md) |
| Explanation | understand why | [How a flight is simulated](how-a-flight-is-simulated.md) |

### The guides' format

*Rocketry explained*'s guides are explanation pages about the physics behind HPR Sim's numbers,
such as [Trusting a simulation](trusting-a-simulation.md). Each follows one format
([the guides' decision record](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0195-the-2026-10-07-guides-rocketry-explained.md)),
and the build holds the parts marked as checked:

1. **It opens with what it covers, who it is for and how far to trust it.** That's the
   one-sentence lead, a paragraph that names its reader, then the trust note.
2. **Words come before equations, then a worked example.** Its numbers are printed by a program
   run in CI: a block that `cargo xtask cli` regenerates, or heights and misses quoted from a
   committed report.
3. **Every number in its text is read from a committed report, or sits in a regenerated block**
   (checked). A quote in `xtask/src/guides.rs` fills its numbers from a report.
   `cargo xtask guides` fails a page that doesn't hold the quote word for word. It also fails any
   number in the page's text that no quote reads. Its tests move each value every quote reads and
   fail unless the page check then fails: a mutation probe for every number.

   Titles and image descriptions count as text, and a decision record's label doesn't count.
   Inline code may hold a number only where a regenerated block on the page shows the same text,
   such as a command's flag. A code block that isn't regenerated holds none. HTML other than a
   comment fails. Write small numbers in digits, so the check sees them.
4. **Each figure shows one idea,** in the system's drawing language. `cargo xtask guides` draws
   it from the reports (checked: a test fails a committed figure the reports no longer draw).
5. **Uncertainty is drawn as a band, and the flattering side is marked.** The marked side is tied
   to the test that pins which way the quantity errs, and the guide names that test.
6. **Nothing is clipped** (checked, by the three checks in [Nothing cut off](#nothing-cut-off)).
7. **No verdicts, and nothing copied.** No guide says a rocket is safe or will pass: the safety
   code and the range safety officer decide. A charge size appears only as a range, after
   "ground test first".

   The NFPA's codes, the certification exams' question pools, the magazines and OpenRocket's
   share-alike documentation are cited and linked, never copied. No exam question is answered.
   NASA's public-domain figures may be reused with credit.

A new guide goes in `SUMMARY.md` under *Rocketry explained*, and in `guides()` in
`xtask/src/guides.rs` with its quotes and figures. One with a regenerated block also goes in the
list of guides in `xtask/src/cli.rs`.

The product guides, under *Product guides*, are a product's landing page and its tour, such as
[A tour of HPR Sim](simulator-tour.md). They follow the same format and join the same lists:
`SUMMARY.md` under *Product guides*, `guides()`, and the list of guides in `xtask/src/cli.rs`.

Their pictures of the command line are terminal figures. `cargo xtask cli` draws each from the
run of an example on the same page, long lines wrapped and never cut. Lettered balloons mark the
lines the text explains. The same command fails a committed figure the run no longer draws.

Each is named, with its balloons, in `SHOTS` in `xtask/src/cli.rs`. The example's text block
stays on the page beside it, for a reader to copy or search
([the simulator's guide's decision record](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0247-the-simulators-guide-in-four-increments.md)).

## Nothing cut off

Nothing on this site may be clipped by an edge or a corner. That means no text, number, mark or
button, in a figure or on a page, at any window width. Three checks fail the build when something
is ([the decision record](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0204-the-no-clipping-checks.md)
has the rules in full):

- **Figures stay in their frame.** `cargo xtask figures` reads every SVG under `docs/`, and
  `cargo xtask cli --check` draws each `hpr sim --plot` figure and checks it. Each line, shape,
  stroke, arrowhead and label must lie inside the figure's frame and inside any clip it sits in.
  The frame is its `viewBox`, the area the SVG file says it draws in.

  A label is measured as if drawn in the widest of the common system fonts, character by
  character. That's because a figure shown as an image draws in whatever font the reader's
  computer has.
- **No text over text.** In a figure, two labels' boxes may not overlap by more than rounding.
  On a page, two runs of text fail when they overlap by more than 2 px sideways and a quarter of
  the shorter one's height. Two lines set tightly, or two runs that touch, pass.
- **No page wider than the window.** `cargo xtask site` opens every page in headless Chrome at
  each width from 320 to 1,920 px, 40 px apart. A page fails if it scrolls sideways, or if text
  or a box passes the window's left or right edge. It also fails if a box cuts its own text
  short. A code block or a table may scroll sideways inside its own box.

CI runs the figure check in the tests on every system, and `cargo xtask site` in its `site` job.
`scripts/gate.sh test site` runs both locally. A failure names the page or figure, the widths,
and the label or element with how far it reaches.

`cargo xtask site` also loads pages built to fail, and fails if it stops finding them. The figure
check's tests do the same with a committed figure's label moved out or laid over another. Not
covered: the API reference under `api/`, and a reader whose system font is wider than every font
the figure check measures.

When a check fails, change the layout: wrap the line, shorten the label, give it a row of its
own, or make the figure taller. Never crop, hide or shrink the frame to pass. The page check
needs Chrome or Chromium. If it isn't where the system usually keeps it, set `HPR_CHROME` to the
program's path.

### Figures in the system's colors and fonts

The same figure check holds figures to the FusionSpace product system's
[foundations](https://github.com/nrdptel/fusionspace-design/blob/main/product/foundations.md).
It reads every SVG under `docs/` and `theme/`, and each figure `hpr sim --plot` draws (see
[the command line](cli.md)).

Every color a figure paints is one of the system's color tokens, the named colors it lists by
job (*Semantic roles*). They're the light theme's, such as `ink` (`#0B0F1C`) for text and
`predicted` (`#A22488`) for a simulated line. They're also the state colors for chips and badges
(`ok-fill`, `caution-fill`, `danger-fill`, `info-fill`), with the text that goes on them.

A figure shown as an image can't follow the page's dark theme, so it is drawn on the light one,
its own background included. Four things fail:

- a color that isn't a token, written by name, as `currentColor`, or in a `style`;
- an opacity below 1 or a gradient, which blend tokens into colors none of them is;
- a shape or a line of text with no `fill` set on it or around it, which draws in black;
- text whose `font-family` doesn't start with Cascadia Mono (labels and numbers) or Archivo
  (prose).

An image can't load the site's fonts. A reader without Cascadia Mono installed sees the next
family in the list, and the frame check measures every one of those. It can't measure Archivo
yet, so for now every figure sets its text in Cascadia Mono.

The check can't tell whether a line type is the right one, so the author picks it from the
system's table (*Lines and shape*):

- dashed (`8 4`), 2 px, for anything simulated;
- a chain line, a long dash and a dot as on a drawing (`24 3 1 3`), 1 px, for a reference such
  as the ground;
- dotted (`1 3`), 1 px, for an event.

The figure on [*How a flight is simulated*](how-a-flight-is-simulated.md) draws the dashed and
chain lines, with a sample of each in its key. The plot from `hpr sim --plot` draws all three.

### The pages in the system's look

`cargo xtask site`'s page check, the one that opens every page at every width, also reads each
page's computed style. That's the colors, sizes and outlines the browser ends up drawing. So the
site keeps the product system's look where mdBook, the program that builds the site, would draw
its own. It reads the light and navy themes, and the light one with scripts off.

Colors off the system's palette, rounded corners, shadows, motion off its timings and type off its
sizes all fail it. Body text is 16 px up to a 1,024 px window, grows with the window, and is
18 px from 1,600 px. The check reads each size against the one the window's width gives. Prose
takes its column's whole width, and the column stops at 1,120 px, so a line is as long as its
column allows.

It also fails prose held narrower than the box that holds it, by a max-width, a width or margins
set to auto. It fails a column allowed wider or narrower than 1,120 px, or off center beside the
sidebar. Gutters off 16 or 32 px fail it too
([the decision on the column and the reading scale](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0250-the-column-and-the-reading-scale.md)),
and so do:

- text under the contrast that WCAG 2.2 AA asks for against the background it is drawn on. WCAG
  is the W3C's Web Content Accessibility Guidelines, and AA its level. The contrast is 4.5 : 1, or
  3 : 1 for text of 24 px and over (18.66 px in bold). It's measured after any opacity between
  the text and that background fades the text;
- one control of each kind the keyboard reaches without the system's focus ring. That's the
  outline drawn round a link or button when Tab reaches it. It's 2 px wide in the action color
  (`#3350D6` on the light theme) and 2 px clear of its edge;
- what Windows' high-contrast mode would erase. That mode, forced colors, repaints the page in a
  few colors of the reader's own. It would erase a box set apart from what is behind it only by
  its fill, with no border there. It would also erase an icon not drawn in its text's color. The
  check can't switch the mode on; it applies the stylesheets' rules for it and judges from the
  page's own colors;
- a bar stuck to the top of the window reaching further down than the page's
  `scroll-padding-top`. That's the room the browser leaves at the top when it scrolls to what a
  reader tabs to, so the bar never hides it;
- something fixed to the window inside a box whose `transform` or `will-change` would carry it
  with the page instead. The page's own layer on a phone would too;
- a `>` quote block left on a page;
- a note not drawn as the system's. The system's note has a head, then the message. The head
  says its kind's signal word in Cascadia Mono 12 px semibold capitals, across the note's top. A
  rule or a fill sets it apart (a caution's in the caution fill, a warning's in the danger fill).
  The note sits inside a border on every side, 2 px on a warning;
- a font list without its metric-matched fallback second. That's a stand-in, Arial or Menlo,
  scaled to the real font's size, so text doesn't jump when the real font arrives;
- a page whose Cascadia Mono fallback sets a line more than 0.1% longer or shorter than Cascadia
  Mono does, regular or semibold. The fallback is the family `'Cascadia Mono Fallback'`, drawn
  until Cascadia Mono arrives. The site scales Menlo, DejaVu Sans Mono, Courier New, Liberation
  Mono, Cousine, Droid Sans Mono and Noto Sans Mono to it. It can, since all of a monospace font's
  letters share one width ([#468](https://github.com/nrdptel/fusionspace-eridanus/issues/468)).
  The check measures the font that the browser running it picks, so a machine checks only its own
  fonts; CI's Linux runners check theirs. A page that declares no Cascadia Mono face, as most of
  the check's test pages, is skipped;
- a smooth scroll a script asks for when the menu bar's title is clicked;
- an icon that isn't square at one of the system's icon sizes, 16, 20 or 24 px. That covers the
  icons in the menu bar, the page arrows, the search field and a code block's buttons. The site
  draws them at 20 and 24 px. A note's icon, 14 px as the system's note draws it, isn't read;
- selected text not drawn on the action color at 22 % opacity.

A file check fails a page whose head doesn't declare light and dark color schemes. It fails one
that doesn't set the browser bar to the system's light canvas (`#F3F4F7`) on a light system. The
same goes for its dark canvas (`#0B0F1C`) on a dark system. It also fails a `fonts.css` that
isn't the system's.

mdBook draws its buttons with Font Awesome, the icon set it ships, and names the book in an `h1`
above each page's own title. After mdBook builds the site, `cargo xtask site` draws each icon as
the system's icon of the same meaning. It also turns the book's name into a paragraph. A file
check then fails:

- a page with a Font Awesome icon, or an icon that isn't the system's;
- a page with a second `h1` (the print page, which holds every chapter, keeps one per chapter);
- a favicon that isn't the system's.

mdBook also names the book in text in its menu bar. It shows the table of contents beside the
page only from 1,080 px, with a menu button below that. It ends a page with its arrows. The build
redraws that frame:

- the menu bar becomes the system's header. It holds the FusionSpace lockup, 24 px tall in the
  text's color, then HPR, one link to the landing page. The lockup is the logo's mark with the
  FusionSpace name beside it;
- the menu button stays at every width. From 720 px it puts the table of contents away and
  brings it back. Dragging the table's edge to the window's side puts it away too. The choice is
  kept from page to page, and a first visit shows the table from 720 px. With scripts off, the
  table stays open from 720 px and the button goes;
- the arrows under the page name the pages they lead to: "Previous" or "Next", then the page's
  title. They stand under the text, inside the column;
- every page ends with the title block, the box of facts about a document an engineering drawing
  ends with ([title block](glossary.md#title-block)). Its entries are written once at the end of
  [Start here](start-here.md#title-block), and each page adds its own title after the site's.

A file check fails a page:

- without one `header` holding the lockup, or without the table of contents, its box or its
  menu button;
- with mdBook's script that opens the table of contents only from 1,080 px, or with one that
  forgets that the reader put it away;
- with an arrow under the page that doesn't name the page it leads to;
- without one `footer`, after the page's text, holding the title block with its fields in order
  and the page's own title.

The page check fails, at every width:

- a lockup not 24 px tall, not in the text's color, or cut by its box;
- a lockup closer to another icon or text in the header, or to the window's side, than a quarter
  of its height. That's the logo's clear space, the brand's rule;
- no menu button, with scripts on; with scripts off, a menu button at 720 px or wider, or none
  below 720 px;
- a table of contents hidden at 720 px or wider on a first visit. That's read with scripts on
  and off, and after a narrower window is widened past 720 px;
- a table of contents hidden but still reached by the keyboard or a screen reader, or one shown
  but not reached;
- once a run, a menu button that doesn't put the table of contents away or bring it back;
- once a run, a drag of the table's edge to the window's side that doesn't put it away;
- arrows under the page outside the column or off its edges, under 44 px tall, or naming no
  page;
- a title block without its 2 px border in the text's color, or with entries not in Cascadia
  Mono;
- a title block off the column's edges (it is centered as the column is), or with anything
  below it.

Inside that frame the build lays each page out as an engineering drawing set is laid out:

- every page opens with an intro over its title: a small tag with one corner cut off, holding the
  site's designation, beside a status chip. The designation is `FS-ACHERNAR · SW · TOOL 001`, the
  product's number in the FusionSpace register, as the title block gives it. The chip shows the
  title block's *Status*, one of the product system's drawing words. It's IN PREPARATION, outlined
  in the link color with a square icon, or RELEASED, filled in green with a circle. The site has one
  status, so every page shows the same chip;
- the paragraph right under the title is the page's lead, set larger and in gray. It's one
  sentence, of at most 25 words, ending in a period, that says what the page covers and what a
  reader gets from it. Write it as the first paragraph of every page, under the `#` title and
  before anything else (on a model page, before *In short*). A sentence ends at a period,
  question mark or exclamation mark followed by a space. So a lead can't name an abbreviation
  with a period in it: write "the 1976 US Standard Atmosphere", not "the U.S. Standard
  Atmosphere";
- each `##` section is a sheet. It has a 2 px rule in the text's color across its top, then its
  number, `SHEET 2 / 5`, in 12 px capitals over its name. On the print page, which holds every
  chapter, each chapter counts its own sheets. Write each `##` heading in the page's text, not
  inside another element, or the build fails;
- the header holds a theme switch, three buttons: Auto (the theme your system asks for), Light
  and Dark, the chosen one filled in. From 960 px it stands in the header. Narrower, the header
  has no room for it, and it stands at the top of the table of contents. The menu button opens
  that below 720 px. The switch needs scripts, so without them it isn't drawn;
- mdBook's own theme menu, with six themes for two looks, is gone. A reader who had saved one of
  the other four is put back on Auto.

A file check fails a page:

- without the tag and the chip over its title;
- without a lead of one sentence and at most 25 words under it;
- with a `##` heading outside a sheet, or sheets numbered out of order;
- without the switch in place of mdBook's theme menu;
- with a heading that skips a level going down (a `####` right after a `##`; headings go in
  order).

The page check fails, at every width:

- a tag not in Cascadia Mono 12 px, without its cut corner or 1 px border, or not above the title;
- a chip not in Cascadia Mono 12 px at weight 600 in capitals, or under 24 px tall;
- a chip with its icon other than 14 px, in other colors than its status's, before its tag or
  not above the title;
- a lead not in Archivo 20 px on 28 px lines at weight 400, not in the muted ink, or not right
  under the title;
- a sheet without its 2 px rule in the text's color;
- a sheet with its number missing, out of order, not in Cascadia Mono 12 px or not above its
  name;
- a `##` heading outside a sheet;
- a switch not in the header from 960 px, or not in the table of contents below it. Below 720 px,
  the check opens the table of contents to read it;
- a switch with buttons other than Auto, Light and Dark, under 44 px tall or not in Cascadia
  Mono 14 px;
- a switch with a button other than Auto chosen for a first-time reader, or the chosen one not
  filled in the text's color;
- mdBook's theme button drawn, the switch drawn with scripts off, or the page in one of mdBook's
  themes the switch doesn't offer.

Once a run, at the width it reads print at, the check presses Dark and then Auto. It fails a page
whose theme doesn't follow. It also loads the page with mdBook's Coal saved, and fails one that
doesn't forget it and press Auto.

A page has two to six sheets (its `##` sections, above), as the product system's page anatomy
asks. `cargo xtask site` and a site test fail a page outside that range, with no exceptions. A
page with more to say groups its sections. A `##` heading names the group, and the sections
under it move down one level. They keep their words, so every link to them still lands
([M0.9d13, the pages' sheets and trust](decisions-and-roadmap.md#m0-9d13)).

Every page opens with how far to trust it, as the product system asks of a docs site. That's a
trust note in the shape above, between the title's lead and the first sheet. A model page says
it in its first sheet, the *In short* box, whose *How well it is validated* item gives the
evidence. `cargo xtask site` and a site test fail a page that doesn't. A page that states rules
or records rather than results, such as this one, says so in its note and names what checks it.

mdBook draws three icons as characters of text:

- `❱` beside a heading that folds open in the table of contents;
- `✓` beside the chosen theme;
- `»` before a heading a link jumped to.

The build draws the fold toggle as the system's chevron, and fills in the chosen theme on the
theme switch. A heading jumped to takes no mark, since the page scrolls it to the top.

The page check fails a text made only of such marks outside a page's `main`. It also fails one
drawn by a stylesheet before or after an element anywhere. Such marks are arrows, technical
symbols, shapes, dingbats, guillemets, emoji and the private characters icon fonts draw in. A
key's name in the help popup (`←`) and a mark used as a word in prose (`Mach → 0`) pass.

The check also reads each page as it would print, once. It does so at the window width it tries
nearest the printable width of a sheet of paper. That's 720 px: an A4 or Letter sheet inside
Chrome's default margins. CI splits the widths over three machines, and each reads print at its
own nearest. The stylesheets' print rules are applied, and their screen-only rules set aside.

A headless browser can't print from a page, so this reads the print rules, not a printed page.
Where the browser would break the pages isn't seen. In the light theme and in the navy one, it
fails:

- a color that isn't the light theme's, and on a page with both themes, anything drawn in
  different colors from the two;
- navigation or a control drawn: the table of contents, the menu bar, a button or a text field;
- a link to another page, in the text or the title block, without its address after it, in
  parentheses. A page of this site shows its path from the page, such as `cli.html#units`;
- a table row, a note or the title block that can split across two sheets;
- a title block not drawn, or drawn without its version and date of issue;
- anything fixed to the window, which a browser draws on every sheet over what stands there.

The print page, every chapter on one page, shows on screen only a cover. The cover says what it
prints, holds a button that prints it, and says how to print from the browser. Its chapters wait
in a template, which a browser reads but doesn't draw. The site's script places them in the page
when the browser is about to print, and takes them away after.

Drawn on screen, all of them were too slow for a phone (below). The check places them as printing
does before it reads the page on paper. It fails a page where they don't all arrive, or don't all
leave again.

`mdbook serve` still shows mdBook's icons, menu bar, theme menu and two `h1`, with no sheets. It
shows the landing page's title block as a table. Only `cargo xtask site` builds the pages as
published. Not read: text inside SVG images or over a picture, and hover
([#432](https://github.com/nrdptel/fusionspace-eridanus/issues/432)).

### How fast the pages load

This part of the check is a lab test of how fast each page loads and answers. It uses a simulated
phone on a slow network, not measured visits by readers. So it says whether a page is likely to
be quick, not how quick it is for you.

It loads every page of the guide, though not the API reference: rustdoc's own pages under `api/`,
which the layout check leaves out too. It measures Google's three
[Core Web Vitals](https://web.dev/articles/vitals). Each is held to the bound the product system
sets, Google's "good":

- **Largest Contentful Paint (LCP)**, when the largest text or picture on the first screen is
  drawn: at most 2.5 s;
- **Cumulative Layout Shift (CLS)**, how far what is drawn moves afterwards, as a share of the
  screen: at most 0.1;
- **Interaction to Next Paint (INP)**, how long the page takes to answer a key or a tap. The
  check presses Tab and taps the menu button twice; search, scrolling and the other buttons
  aren't tried. The slowest of these must take at most 200 ms.

The phone and the network are the ones Google's
[Lighthouse](https://developer.chrome.com/docs/lighthouse) uses for a phone. That's a screen 412
pixels wide, half a second's wait on every request (562.5 ms) and 1.5 Mbit/s down. A fast
computer is slowed to the phone's speed.

The check first times two short loops as Lighthouse does. It slows the processor by the factor
Lighthouse's guide gives for that time. Then it times the loops again to make sure the slowdown
took. On the Mac the check was built on, the factor is about 16. A slower machine gets a smaller
one, and the guide puts a high-end desktop at about 4.

A machine that would need less than 2 is refused, since a slowdown that small can't be told from
none. Every page is loaded cold, nothing cached. A page over a bound is loaded three times and
judged by the middle result.

The bounds are meant for real visits, and the site collects no numbers from them. Two things make
the lab slower than a visit. First, its server speaks HTTP/1.1, where a browser opens at most six
connections to one server. GitHub Pages, which serves the site, speaks HTTP/2 with no such limit.
Second, each page is a first visit, though a reader's second page finds the stylesheet, scripts
and fonts already saved.

One could err either way: the slowdown for a fast machine carries Lighthouse's guide past the
machines it was fitted to. And a phone draws text in its own fonts until the site's arrive. The
fallback is matched to Arial and to the Linux and ChromeOS fonts made to Arial's widths. So on a
Mac or on CI's Linux runners the largest CLS was 0.067. But an Android phone draws in Roboto,
which isn't matched and isn't measured ([#461](https://github.com/nrdptel/fusionspace-eridanus/issues/461)).

In one run on October 11, 2026, the processor was slowed 16.4 times. The slowest LCP was 2.06 s
and the slowest INP 96 ms, both on [*Aerodynamics*](physics/aero.md). The largest CLS was 0.067
([*How a flight is simulated*](how-a-flight-is-simulated.md)). On CI's Linux runners, slowed
about 7 times, the worst INP was 136 ms, the worst LCP 2.19 s and the worst CLS 0.067. A runner
slowed 12.1 times measured INP 200 ms on the aerodynamics page, at the bound
([#462](https://github.com/nrdptel/fusionspace-eridanus/issues/462)).

LCP moves by a tenth or more from one run to the next. The check loads one page at a time, as
Lighthouse asks, since pages loaded side by side slow each other down. Each run prints its own
figures in the line that starts `page vitals:`. Three pages the check serves itself must each
fail their own vital and pass the other two, or the check fails. One page's stylesheet arrives
late, one moves its text after drawing it, and one's menu button keeps the page busy.

What it took to pass is recorded in
[ADR-232, the pages' vitals measured](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0232-the-pages-vitals-measured.md):

- one stylesheet instead of ten, asked for first with mdBook's sidebar script;
- four fonts asked for early so that text doesn't move when they arrive. That's two more than
  the product system allows: with its two, CLS reached 0.17 on two pages;
- the theme switch placed before the page is first drawn;
- the print page's cover.

## Where this comes from

The rules follow the UK Government Digital Service's
[style guide](https://www.gov.uk/guidance/style-guide), and the sentence-length checks that
Datadog and GitLab run with the [Vale](https://vale.sh/) linter. The page kinds are Daniele
Procida's [Diátaxis](https://diataxis.fr/) framework. The decision to adopt them is in the
[2026-10-03 review's decision record](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0144-the-2026-10-03-planning-review-leaner-bookkeeping.md).

The voice rules come from
[a survey of plain technical writing](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/plain-voice-docs.md).
It drew on the style guides and studies that name stock phrases. It also drew on 28 pages, from
nine sources of technical writing, that read as if one person wrote them.
