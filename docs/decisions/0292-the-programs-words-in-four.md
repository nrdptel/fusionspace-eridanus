# ADR-292: The program's words in four increments, the help first (2026-10-25)

- **Status:** accepted; splits [M0.14d][m0-14d] into four increments, M0.14d1 to M0.14d4, and plans every remaining increment, the last with M0.14d's own *done when*; extends [ADR-285][adr-285] §1's hold to the help; amended 2026-10-25 for M0.14d1's acceptance pass and for M0.14d2's census and tasks
- **Summary:** M0.14d covers `hpr`'s help, its warnings and errors, the Python package's docs and each crate's top-level docs. That's too much for one rewrite: the help is about 4,400 words, but the messages are about 270 error types and 80 to 90 places that push a warning, in 14 crates. They go in four increments. The help comes first, with a snapshot of each command's `--help` that `cargo xtask voice` reads and holds to the voice limits, as the site's pages are held.

[m0-14d]: ../decisions-and-roadmap.md#m0-14d
[adr-282]: 0282-the-project-tests-each-ui-milestone-itself.md
[adr-284]: 0284-the-maintainers-own-voice.md
[adr-285]: 0285-the-rest-of-the-site-in-nine.md

**Context.** M0.14d is done when "`hpr`'s help, warnings and errors, the Python docs and each
crate's top-level docs pass `xtask voice` and an acceptance pass; a snapshot pins each `--help`."
On 2026-10-25, main (094d49dc) has:

- **The help.** 20 commands, from `hpr` down to `hpr weather era5`, print 4,394 words with
  `--help`, counted by `wc -w`. Nothing pins them, and `xtask voice` can't read them yet. It reads
  a text file as Markdown, so an option's description, indented ten spaces, is a code block to it:
  of `hpr sim --help`'s 472 words it scores 145.
- **The errors and warnings.** 267 `#[error]` attributes in 14 crates, and 80 to 90 calls that
  push a warning (76 and 89, by two counts). The crates' library code holds roughly 60,000 words in string literals of four
  words or more, before any test module, comments left out (two counts differ by up to a tenth).
  That's only a rough measure of the messages: it also counts the help, column names, sample text and report wording, and it misses
  the shortest messages. No check reads any of it.
- **The Python docs.** The package's README (425 words), its module's docstring (about 100
  words), and about 4,200 words of docstrings written as Rust doc comments in
  `crates/hpr-py/src`.
- **The crates' top-level docs.** The `//!` comments of the 18 crates' `lib.rs`, about 4,700
  words in all, which docs.rs shows as each crate's front page.

At the pace M0.14c kept, each surface but the help is a session's work or more, so M0.14d is
split. ADR-285's hold carries over: once an increment's text passes, a check keeps it passing.

**Decision.**

1. **The increments:**

   | Increment | What | Size on main |
   |---|---|---|
   | M0.14d1 The help | each command's `--help`, a snapshot of each, and the voice check reading them | 20 screens, 4,394 words |
   | M0.14d2 What the design commands say | the errors, warnings and notes `hpr sim`, `hpr mc`, `hpr convert`, `hpr motors list` and `hpr motors show` print, from `hpr-cli` and the crates under them: `hpr`, `hpr-sim`, `hpr-io`, `hpr-motor`, `hpr-design`, `hpr-aero`, `hpr-core`, `hpr-format`, `hpr-atmos` | 160 `#[error]` attributes; roughly 25,000 words of literals |
   | M0.14d3 What the data commands say | the same for `hpr weather`, `hpr motors fetch` and `search`, and `hpr analyze`, from `hpr-cli` and `hpr-net`, `hpr-flightdata`, `hpr-analysis`, `hpr-forensics` | 96 `#[error]` attributes; roughly 30,000 words of literals |
   | M0.14d4 The Python package and the crates' front pages | the Python package's README, module and docstrings, `hpr-py`'s own messages, and each crate's `//!` docs; with M0.14d's own *done when* | about 9,400 words of docs, and about 65 of `hpr-py`'s literals |

   Both rows list `hpr-cli`: each takes its own commands' code, and M0.14d2 also takes the shared
   code (the dispatch, the output and the option readers) and the other commands (`completions`
   and those not available yet). Any other crate is read once, in the row that lists it, though
   the other row's commands call it too.

   `hpr-validate` is left out: it builds the validation reports, isn't published and prints
   nothing a user of `hpr` sees. Its 11 errors reach only the project's own tools.
2. **The help's reading.** `cargo xtask voice` reads a file ending in `.help` as clap lays out a
   piped `--help`. Clap doesn't wrap when it isn't writing to a terminal, so each line is one
   run of prose. Read as prose:
   - the command's description;
   - each paragraph about an option or an argument;
   - each command's summary in a list of commands;
   - each example's caption.

   Left out as code: the `Usage:` line, the section titles, the names of options, arguments and
   commands, the notes clap writes in brackets at a line's end (`[default: auto]`, and the same
   form written by hand), the example commands and the guide's address. So a default that needs words is written as a sentence. Text in backticks is code, as on a page. The help is a reference page ([ADR-284][adr-284]
   §2), so first person counts against a limit of zero.
3. **The snapshots.** `crates/hpr-cli/tests/help/` holds one file a command, named by its path
   (`hpr-motors-show.help`). A test walks the command tree `hpr` parses, so a new command needs
   its snapshot and a removed one leaves none behind, and compares each with what the built `hpr`
   prints. `HPR_BLESS=1` writes them again, and the diff is reviewed in the PR. Only `--help` is
   pinned. A command's `-h` prints the first paragraph of the same texts, so, apart from clap's own
   line about `-h` and `--help`, it holds nothing the voice check misses.
4. **The hold.** Every snapshot is held as ADR-285 §1 holds a site page: no stock phrase, every
   rate within its limit, at most 20 words a sentence on average and fewer than one sentence in
   ten over 25 words. CI runs it in the site step, as it does for the pages, and the report
   lists the snapshots beside them.
5. **The messages,** for M0.14d2 and M0.14d3. A message is built from literals and the values put
   into them, so it can't be scored as a page. Each increment adds a census to `cargo xtask voice`:
   each `#[error]` text and each warning or note text of its crates, with each placeholder read
   as one code word. Its crates are held as the help is. Where a message is assembled from more than one
   literal, the census reads the literals it can see and says how many it can't join. Each
   increment's ADR amendment settles the census's rule before its rewriting starts, with
   that increment's tasks. A rewritten message keeps every number, unit, limit and the input it
   names, and the tests that match a message's words move with it. Pin which refusal fired by
   its value, not by its words.
6. **M0.14d4** holds the Python package's README and docstrings, as `help()` shows them, and each
   crate's `//!` docs, as docs.rs shows them, and carries M0.14d's own *done when*. A crate's top
   page is a reference page, so it stays impersonal.
7. **How each increment works,** as ADR-285 §3 has it: its acceptance tasks ([ADR-282][adr-282])
   are written here first, a claims pass reads each changed sentence against the old one, its PR
   shows text before and after, a reviewer who didn't write it does the tasks on the built
   program, and a cold read checks the text against each voice rule. A message or a help
   screen is terminal text, so its capture is the text itself, which the snapshot or the census
   holds. A site page an increment changes is captured at 320 and 1,280 px in both themes, as
   ADR-282 amended allows for words alone.

**M0.14d1's tasks,** each done with the release build of `hpr` from the help alone, then checked
against what the command does:

1. From `hpr --help`, find the command that flies a design and fly the guide's design,
   `validation/fixtures/ork/guides/level-1.ork`. Then find out what "not available yet" means by
   running `hpr optimize`.
2. From `hpr sim --help`, fly that design on an H170M with a 10 s delay in a 4 m/s wind from the
   west, off a 2 m rail leaning 5° toward the east, and export its recording as a `.csv`.
   Hostile: `--interval 0` and `--delay x` are refused with a message saying what's allowed.
3. From `hpr mc --help`, fly 50 flights with the motor's impulse scattered by 3% and seed 7, twice,
   and see the same numbers both times; save every flight to a `.csv`.
4. From `hpr motors --help` and its commands' help, list Cesaroni's 38 mm motors, work out the
   H170M's figures, and read a public `.eng` file from `validation/`. Hostile: an invented
   `.eng` cut off mid-curve is refused with a message.
5. From `hpr convert --help`, write a catalog motor as a `.eng` file, turn that into a `.rse`, and
   turn the guide's design into an `.hprz`.
6. From `hpr weather --help` and its commands' help, say which source to use for a launch next
   week outside the U.S., and which for one already flown. Then read a saved answer file from
   the tests' samples with no network. Hostile: an invented answer file is refused.
7. From `hpr analyze --help`, read the PerfectFlite sample
   (`crates/hpr-flightdata/tests/samples/perfectflite-export.csv`) and give its apogee; print a
   mapping for your own logger's `.csv` with `--print-mapping`. Hostile: an invented log is
   refused with a message.
8. From `hpr completions --help`, write a completion script for zsh.

The cold read takes each of the 20 screens against each rule of the voice section and lists what
it finds; each finding is fixed in the PR or filed.

**Amendment for M0.14d1's acceptance pass (2026-10-25).** Task 4's hostile file, a `.eng` cut off
mid-curve, wasn't refused when cut at a line's end. The file is still well formed, and its curve
just stops: a G84 curve cut after 13 of its 29 points read as an F motor of 79.1 N·s, with only a warning
that the curve ends above zero thrust. The task stands as written. The fix is in the motor readers:
the `.eng` and `.rse` readers, and their writers, refuse a curve whose last thrust is above half its
peak; when other entries in the file read, that entry is skipped, with a warning. The RASP format
says the final point must have zero thrust, but real files don't always. Of the 3,585 curves the
readers read from 1,294 of the 1,301 motor files under the project's reference checkouts, 11 end
above zero, the highest at 10.5% of its peak, so half the peak refuses none of
them. A test reads those files and prints the figures wherever the checkouts are. A cut where the
thrust is under half its peak, in the tail-off or a sustain phase, keeps the warning.

**Amendment for M0.14d2: the census's rule and its tasks (2026-10-25).** The source doesn't say
which literal ends up in a message. A refusal's `what` is written where the error is made, a
warning's text is often built in one function and pushed in another, and a note can pass through a
variable. So the census reads more than the messages. That holds a stock phrase or first person
anywhere in the code's prose, but the extra prose also dilutes the rates, so `--long --messages`
lists each long sentence.

1. **The files.** Each tracked `.rs` file under `src/` of the nine crates and of `hpr-cli`, less
   test code: what a `#[cfg(test)]` marks (an item, a variant, a field, a match arm, or a module
   by its own `#![cfg(test)]`), and a module declared under it, with the modules inside that
   module. From `hpr-cli` it leaves out the files of M0.14d3's commands: `analyze.rs`,
   `weather.rs`, `motor_fetch.rs`, `motor_search.rs` and `waiting.rs` (what a fetch prints while
   it waits).
2. **What it reads.** Each `#[error("…")]` text, and each other string literal in the code that
   reads as prose: three words or more, a placeholder counting as one, and at least one of them a
   word of letters. It doesn't read markup, comments, doc comments, byte strings or other
   attributes. Markup is a literal that starts with a tag and holds a closing tag, a tag closed on
   itself, an attribute, a declaration or a comment: a piece of an SVG figure or of a file's XML.
   A note that only opens with an element's name (`<Shape> is …`) is read. The help's attributes are held by the snapshots, and a
   `reason = "…"` is read only by the code's readers. An `#[error(transparent)]`, or an `#[error]`
   text with no word of its own (`"{0}"`), passes on another error's text, which is read where
   it's written.
3. **How it reads them.** Each line of a literal is a paragraph of a reference page. A placeholder
   (`{path}`, `{0:.1}`) is one code word, like text in backticks. The pieces a message joins at run
   time are read one by one. The census counts the literals with a placeholder, which it can't
   see whole; pieces joined with no placeholder are read in pieces too, so the count is a guide, not a bound.
4. **The hold.** There is one page a crate, named by its `src/` directory, and it's held as a help
   snapshot is (§4). No stock phrase, every rate within its limit (first person at zero), and both
   sentence targets. The report lists the ten pages beside the site's, with the census's counts.
   `cargo xtask voice --long --messages` lists each long sentence with its file and line.

On main (cdb01265), this rule reads about 21,800 words. Two crates' pages were over the limit on
long sentences: `hpr-sim` (0.90 per 100 words, and 31 of its 231 sentences over 25 words) and
`hpr-aero` (0.82). `hpr-cli`'s page held six stock phrases. A census reads pieces, and a message is
read whole, so the tasks below read the joined messages.

**M0.14d2's tasks.** Each is done with the release build of `hpr` and the messages alone, then
checked against what the command did:

1. Fly Loft's demo designs (`validation/fixtures/ork/loft-demo/`) as the files have them. From the
   errors alone, say why each can't fly and what to change, then fly `demo-stable.ork` on a
   catalog motor that fits.
2. Hostile `hpr sim`: a file that isn't there, a `.eng` given as the design, a `--motor` that no
   catalog holds, `--rail-length 0` and `--wind -3`. Each is refused with a message that names the
   input and what's allowed.
3. `hpr mc` with `--runs 0` and with `--mass-sd 2` is refused, or the failed flights are counted
   and the message says why they failed.
4. `hpr convert`: a design to an extension it can't write, and a motor file to a design format.
   Each is refused with a message that says what it can write.
5. `hpr motors show` with a name the catalog lacks, and with a `.eng` file whose curve ends above
   zero thrust. `hpr motors list --class Z` and `--diameter 1000` match nothing. Each message says
   what happened and what to try.
6. A cold read of every census page against each rule of the voice section, and of the joined
   messages the tasks printed. Each finding is fixed in the PR or filed.

**Consequences.**

- The help can't drift: a change to it fails the snapshot test until the snapshot is written
  again, and the voice check reads the new text.
- The messages wait two more increments. Until then no check reads them, and the help's hold
  says nothing about them.
- The roadmap's M0.14d entry becomes its four increments, which fit its byte budget.
