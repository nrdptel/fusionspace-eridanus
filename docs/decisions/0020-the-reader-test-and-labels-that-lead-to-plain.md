# ADR-020: The reader test, and labels that lead to plain words (2026-09-18)

- **Status:** accepted
- **Summary:** The reader test, and labels that lead to plain words

**Context.** M0.4e asks for a reader new to the project to answer ten new-user questions from
the site alone, and for every term the reader flags as unclear to be fixed. The first cold pass
answered five of the ten fully and five in part. The reader found four problems: no way to describe
your own rocket, no path from a motor file to a flight, pages that disagreed about clusters, and an
ambiguous height for wind tables. It flagged about forty terms. One finding ran across every page:
a milestone or Loft lesson label linked to the top of a long file (`ROADMAP.md`, the lessons
table), which says nothing about the label, and about ninety labels in the API reference were bare
([issue #44](https://github.com/nrdptel/hpr-sim/issues/44)). The roadmap has no heading per
milestone to link to, and a Markdown anchor comes only from a heading, which may not hold a label
(ADR-016).

**Decision.**

- **The test reads the built site, cold.** A reader new to the project gets a copy of
  `target/site`, the API reference included, and nothing else from the repository, except a file that a page links on
  GitHub, as a reader would click through. The ten questions are listed in the PR. Each answer
  cites its pages; every flagged term is fixed; then a second new reader, on the fixed site, answers
  the same ten again, to show the fixes read as meant.
- **Every label leads to a line of plain words.** *Decisions and the roadmap* has a row for every
  milestone and increment of the roadmap, anchored by its id (`M1.10` as `#m1-10`), saying in a
  sentence what it covers and whether it is done. Each Loft lesson a page names has a row too
  (`#l15`), which links its section of the lessons file. The guide's labels link those rows; the
  crates' documentation links them by the site's address.
- **The table can't go stale.** The site check fails if a milestone of `ROADMAP.md` has no row, a
  row names no milestone, or a row's status isn't the roadmap's: `done` when checked off, `blocked`
  when marked so, `not yet done` otherwise.
- **An `<a id="...">` is an anchor.** A page's raw HTML may name an anchor for a place that isn't a
  heading, such as a table row. mdBook keeps the `id`, and GitHub scrolls to it, so the link
  check accepts it like a heading's.
- **The API reference is checked for bare labels,** as the guide is: the visible text of every
  rustdoc page, except the source pages, which quote the code as written.
- **Questions become examples.** What a new user most asked for, and couldn't do from the pages,
  is shown by a program that CI runs and the page quotes: flying your own rocket with its centre of
  pressure and stability margin, reading a motor file and the bundled motors, each kind of wind,
  and recording a trajectory.

**Alternatives.**

- Headings for milestones and lessons on the records page: a heading can't hold a label, so its
  anchor would be words (`#staging-clusters-and-air-starts`) that a writer can't derive from the
  label.
- Linking each label to its phase of the roadmap or its section of the lessons file: closer than
  the top of the file, but still a long list to search, and the reader lands on GitHub rather than
  a line of plain words.
- Anchors in `ROADMAP.md` itself: it is a working file, edited with every milestone, and not a
  page of the site, so a reader still leaves the site to decode a label.

**Consequences.**

- Checking a milestone off in `ROADMAP.md` fails the site check until its row on *Decisions and the
  roadmap* says `done`; the error gives the row to write. A new milestone needs a row.
- A page that names a lesson without a row fails the link check until the row is added.
