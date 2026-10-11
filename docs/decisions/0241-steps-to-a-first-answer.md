# ADR-241: Steps to a first answer: what a step is, and a tool that can't (2026-10-11)

- **Status:** accepted; carries out [ADR-209](0209-the-2026-10-08-best-on-the-market-measured.md) §10; amends [ADR-240](0240-the-scoreboard.md) (the steps rows, a cell that says a tool can't); closes M0.11
- **Summary:** The scoreboard counts the steps to open a design's `.ork` file, fly it once and see its apogee, and get its landing spread, from a fresh start, in four rows: commands, script lines (one statement each), clicks and typed entries, each kind apart. HPR Sim's commands and Python lines are files a test runs; OpenRocket's and RocketPy's steps come from their documentation, each step citing its page, with the release and the date read. A way in that a tool lacks, or a task it can't do that way, is a cell that says *can't*, with its source and the steps to the last task it reaches. HPR Sim takes 2 commands or 9 script lines; OpenRocket 24.12 opens and flies in 7 clicks but has no Monte Carlo; RocketPy 1.13.0 reads no `.ork` file.

**Context.** M0.11's *done when*: "opening a design, flying it and getting its landing spread are
scripted in HPR Sim's command line and Python and counted for OpenRocket and RocketPy from their
current docs (clicks and entries apart from script lines, a task a tool can't do marked so), with
versions and dates, in the scoreboard." [ADR-240](0240-the-scoreboard.md) left one steps row,
`steps.first-answer`, unmeasured for all three tools.

What the tools' documentation said on 2026-10-11:

- **OpenRocket 24.12** is its newest release (published 2025-07-27). Its guide opens a file from
  the File menu (*Open…*) and runs a simulation by selecting it and pressing *Run simulations*; the
  apogee shows in the simulation's row. Its features page lists Monte Carlo dispersion under
  *Planned Future Features*, and 24.12's release notes add none. A landing-dispersion analysis is
  merged into its development branch but is in no release. Its command-line arguments set options
  for the window and fly nothing, and its guide to using its Java core stops at a commented-out
  `simulation.simulate()`.
- **RocketPy 1.13.0** (2026-07-22) is a Python library. A rocket is typed in as Python, with its
  drag curves as numbers or files, its mass and inertias; its documentation never mentions a
  `.ork` file. A separate package, `rocketserializer`, converts one into a parameters file, but
  RocketPy's documentation doesn't name it, no documented line loads its output into a flight, and
  its own issue tracker reports its 0.2.0 release failing on OpenRocket 24.12.

**Decision.**

1. **The task.** From a fresh start (the tool open, or a terminal or a Python session, and the
   design's `.ork` file on disk with its motor chosen and, for OpenRocket, a simulation saved in
   it): open the design, fly it once and see its apogee, then get its landing spread, a spread of
   landing points over many flights with their inputs scattered. HPR Sim's runs use the
   [level 1 guide's design](../../validation/fixtures/ork/guides/level-1.ork) and scatter the
   wind's speed by 25% and its direction by 15° in a 4 m/s wind from the west, the wind being what
   spreads a landing most ([Monte Carlo dispersion](../monte-carlo.md)).
2. **What a step is.** A command is one command line. A script line is one statement, imports
   included, never split over lines: statements, not physical lines, so a formatter's line width
   can't change the count. A click is one press of a mouse button, a double click one. An entry is
   one value typed, such as a file's name or a number. Each kind is counted apart, so a row never
   adds clicks to lines. The count to a task runs through the last step its task names, so a step
   that serves two tasks counts once: `hpr sim` opens the design and flies it in one command.
3. **Where the counts come from.** `validation/steps/steps.toml` lists each tool's three ways in
   (command line, script, window). HPR Sim's command line and script are files beside it,
   `hpr-command-line.txt` and `hpr-python.py`, which `crates/hpr-py/tests/test_first_answer.py`
   runs on every platform CI tests, checking each prints its answer; so a counted step works. The
   other tools' steps each cite the page of their documentation they come from, with the release,
   its date and the date read, and quote its labels as printed. The reader refuses what it can't
   count without a judgment: a statement over two lines, an indented block, tasks out of order, a
   step or a *can't* with no source, a source that backs nothing.
4. **A tool that can't.** A way in that a tool lacks, or a task it can't do that way, is a cell
   that says *can't*, with its reason and source, and the steps of the row's kind to the last task
   it does reach (OpenRocket: "7 clicks to fly"). No milestone measures such a cell; the
   milestone check passes it, and fails a cell that is measured and says *can't*. HPR Sim's own
   window, which it doesn't have yet, says *can't yet* and names
   [M9.1](../decisions-and-roadmap.md#m9-1), the desktop app, whose entry now counts its clicks
   and entries. The page's opening summary counts the *can't* cells and calls HPR Sim ahead
   where it is measured and the other tool can't.
5. **The rows.** `steps.command-line`, `steps.script`, `steps.window-clicks` and
   `steps.window-entries` replace `steps.first-answer`, which held no measured cell. Each is held
   by ADR-240's ratchet, higher being worse, so a change that adds a step to HPR Sim's files fails
   unless an ADR accepts it. The page lists every step under *Steps, one by one*.
6. **Recounting.** The other tools' steps are recounted when either publishes a release: for
   OpenRocket by [M2.3c3](../decisions-and-roadmap.md#m2-3c3), which flies its next release and
   now recounts its steps (its development branch's dispersion analysis would then count); for
   RocketPy by [M2.9](../decisions-and-roadmap.md#m2-9), whose cells are checked in each tool's
   current documentation.

**Alternatives.**

- *Physical lines of a script:* rejected, since the same code formatted at another width counts
  differently, and RocketPy's examples wrap each argument onto its own line.
- *RocketPy with the rocket typed in by hand:* rejected for the count, since it isn't the same
  task: the design is the `.ork` file, and typing it in needs drag curves, masses and inertias
  from another tool. The cell's reason says so.
- *RocketPy through `rocketserializer`:* rejected, since RocketPy's documentation doesn't name it,
  nothing documented loads what it writes into a flight, and its released version fails on
  OpenRocket's newest release.
- *OpenRocket's development branch:* rejected until it is released; a user can't download it as
  OpenRocket.
- *One combined number per tool:* rejected, since a click, a typed value and a script line are not
  the same effort, and the milestone asked for them apart.

**Consequences.**

- HPR Sim is ahead on the two ways it has: 2 commands and 9 script lines, where neither other tool
  has a way to the whole answer from the `.ork` file. Its window cells wait on M9.1.
- OpenRocket opens and flies a design in 7 clicks with nothing typed, in a window; HPR Sim does
  it in one command, and has no window yet.

**Limits.** These are counts of documented steps, not timings or a test with flyers; ADR-209 §10
adds a test by at least five flyers outside the project at release 0.5. OpenRocket's count assumes
its file dialog already shows the design's folder, as no documentation describes the operating
system's dialog; its documentation pages are built from its development branch and titled with an
older release, and their screenshots predate 24.12's reworked simulation table, whose buttons may
differ. A preference that runs a simulation as soon as its tab opens would save OpenRocket 2
clicks; its default is undocumented, so it isn't counted.
