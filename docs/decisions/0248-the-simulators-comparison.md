# ADR-248: The simulator's comparison: a record of capabilities read from each tool's documentation, written into the guide and counted on the scoreboard (2026-10-12)

- **Status:** accepted; carries out [ADR-209][adr-209] §8 for the simulator's guide and closes [M0.8a3][m0-8a3] ([ADR-247][adr-247]); fills the scoreboard's `features.count` row ([ADR-240][adr-240])
- **Summary:** The simulator's guide gets a third page, *Compared with other simulators*. Its table of capabilities is not typed into the page: it is written by `cargo xtask guides` from a committed record, `validation/features/simulator.toml`, in which each cell of another tool names the page of that tool's documentation that shows it, the release and the date it was read, and a note for every *partly* and *no*; each of HPR Sim's cells names the page of the site that covers it, or the open milestone that brings it. The reader refuses a cell without its page, a *no* without its note, and a record in which no capability is HPR Sim's alone. The tools are OpenRocket, RocketPy and RASAero II. Only full *yes* cells count, and the scoreboard's `features.count` row counts them for its three tools. The page's measured comparisons are quotes from the committed reports and the scoreboard, held by the guides' number check and its mutation probes.

**Context.** [ADR-209][adr-209] §8: "in the simulator's guide (M0.8a), a *Compared with* section
names the comparable tools, tabulates what each does, gives the measured comparisons and the
gaps, and names at least one capability none of them has, each absence checked against that
tool's current docs with the date." The scoreboard ([ADR-240][adr-240]) has had a
`features.count` row since it was built, waiting for "the simulator guide's comparison table".
The steps to a first answer ([ADR-241][adr-241]) already keep another tool's claims in a record,
each step with the page of its documentation and the date it was read; a capability table is the
same kind of claim.

**Decision.**

1. **The record.** `validation/features/simulator.toml` lists the tools, HPR Sim first, then the
   scoreboard's two in its order, then any other; each other tool with the release whose
   documentation was read, when that release came out, the date it was read and its pages (a
   title and an `https` address each). Each capability is one row, with one cell per tool, in
   the tools' order:
   - another tool's cell says `yes`, `partly` or `no` and names the page that shows it; a
     `partly` or a `no` says in a note what that page shows instead, and a `yes` may carry a
     limit its documentation states, such as OpenRocket's aim at low supersonic speeds;
   - HPR Sim's `yes` names the page of the site that covers it; its `partly` names that page and
     the open roadmap milestone that brings the rest; its `no` names the milestone. A milestone
     with no open entry in the roadmap fails, so closing one without updating its row fails.
   `xtask/src/features.rs` reads it and refuses each break of these rules by name, text that
   would break the Markdown it is set into (`|`, `[`, `]`, `<`, a line break), and a record in
   which no capability has HPR Sim's `yes` and every other tool's `no`.
2. **The page.** The table, in two columns, the capability and every tool's answer in one cell
   that wraps, so it fits a phone's width with nothing scrolled out of sight; the capabilities HPR Sim alone has, the rows where another tool has
   what HPR Sim lacks (each with its milestone), every `partly` and `no` in words, and the pages
   read with their releases and dates, all written between two markers on
   `docs/simulator-compared.md` by `cargo xtask guides`; `--check` fails a stale region. The
   guides' number check leaves that region to its record, as it leaves a command's example to
   the program, and holds the rest of the page as it holds every guide. The markers count on that
   page only, each once and on a line of its own; anywhere else, or with text beside them, they
   fail, so no region can exempt text the record doesn't write. Beside the table, the page says a
   *yes* says nothing of accuracy, past the speed of sound least of all.
3. **The tools.** OpenRocket, RocketPy and RASAero II: the free simulators the hobby uses most,
   OpenRocket and RocketPy already the scoreboard's columns and the project's oracles, RASAero II
   the hobby's usual tool past Mach 1, which ADR-209's own survey cites. RockSim is left out for
   now: it is a paid program the project has no copy of to set its help beside, and the
   scoreboard has no column for it; the page names no tool it doesn't tabulate.
4. **What counts.** A *partly* is not a *yes*: it counts for nothing on the scoreboard and keeps a
   capability from being anyone's alone. A *yes* says a tool can do the thing, never how well;
   how well is the measured section's job, and only a committed report may fill it.
5. **The measured comparisons** are the scoreboard's: the private collection's apogees against
   OpenRocket's, the public flights, the steps to a first answer and the round trips through
   OpenRocket's file, each a quote from the committed report or the scoreboard, probed by moving
   each value read. RASAero II has flown none of them, and the page says so.

**Alternatives considered.**

- **A table typed into the page.** Rejected: nothing would tie a cell to the page that shows it,
  and the scoreboard would have to count a page's Markdown.
- **A column for each tool.** Rejected: five columns don't fit a phone's width. Built that way
  first, it passed the page check at every width, since a table may scroll sideways there, yet at
  375 px its last two columns were out of sight; issue #498 asks the check to fail that.
- **The cells' notes in the table.** Rejected: sentences in every cell can't be read on a phone;
  the table holds a word and a link for each tool, and the notes follow it.
- **A capability count with *partly* as a half.** Rejected: a half is a judgment the record can't
  check; the note says what is there.

**Consequences.**

- A tool's new release, or a page that moves, is a change to the record and its date; the page
  follows by `cargo xtask guides`.
- When a milestone brings HPR Sim a capability, its row's milestone must change to a page, or
  the record fails once the milestone closes.
- The scoreboard's `features.count` cells are measured; HPR Sim's is held like its other numbers,
  so losing a capability needs a decision record.

[adr-209]: 0209-the-2026-10-08-best-on-the-market-measured.md
[adr-240]: 0240-the-scoreboard.md
[adr-241]: 0241-steps-to-a-first-answer.md
[adr-247]: 0247-the-simulators-guide-in-four-increments.md
[m0-8a3]: ../decisions-and-roadmap.md#m0-8a3
