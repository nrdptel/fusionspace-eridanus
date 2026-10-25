# ADR-292: The program's words in four increments, the help first (2026-10-25)

- **Status:** accepted; splits [M0.14d][m0-14d] into four increments, M0.14d1 to M0.14d4, and plans every remaining increment, the last with M0.14d's own *done when*; extends [ADR-285][adr-285] §1's hold to the help; amended 2026-10-25 for M0.14d1's acceptance pass
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

   A row's crates are where its messages are read. A crate the other row's commands also call is
   read once, in the row that lists it. `hpr-cli`'s shared code (the dispatch, the output and the
   option readers) and its other commands (`completions` and those not available yet) go with
   M0.14d2.

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

**Consequences.**

- The help can't drift: a change to it fails the snapshot test until the snapshot is written
  again, and the voice check reads the new text.
- The messages wait two more increments. Until then no check reads them, and the help's hold
  says nothing about them.
- The roadmap's M0.14d entry becomes its four increments, which fit its byte budget.
