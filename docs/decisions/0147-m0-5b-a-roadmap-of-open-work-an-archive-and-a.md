# ADR-147: M0.5b and M0.5d, a roadmap of open work, an archive and tables kept by the tool, and a queue the check follows (2026-10-04)

- **Status:** accepted; §6's roadmap budget raised to 28,000 bytes by [ADR-163](0163-the-second-pass-products-and-priorities.md), to 31,000 by [ADR-164](0164-the-fusionspace-product-system.md), to 36,000 by [ADR-194](0194-the-2026-10-07-field-tools-checklists-equipment-ground-tests.md), to 38,500 by [ADR-195](0195-the-2026-10-07-guides-rocketry-explained.md), to 40,500 by [ADR-196](0196-the-2026-10-07-product-guides.md), to 43,000 by [ADR-197](0197-the-2026-10-07-parts-catalog.md), to 43,500 by [ADR-199](0199-the-2026-10-07-fusionspace-hpr.md), and to 46,500 by [ADR-209](0209-the-2026-10-08-best-on-the-market-measured.md)
- **Summary:** The roadmap now holds only open work, in an order set by a queue at its top that the checks follow; done milestones move unchanged to an archive file, and the site's tables of records follow, by a command rather than by hand

**Context.** ADR-144 §1b asks for a roadmap of open milestones only, in queue order, the done ones
in an archive, one line and a link each, and checks that measure characters as well as lines. §1d
asks that the records page and similar mirrors be generated, so that finishing a milestone touches
at most the roadmap entry, the project status, the ADR if any, and start-here's table. The roadmap
was at its 1,000-line cap, 82 KB, with 186 of its 237 entries done (corrected on 2026-10-04: an
earlier wording called 82 KB a cap; only the lines were capped). Its rules made file order the order
of work, but ADR-144 §2 set a different one (M0.5, M6.2d2, M4.5, M1.14, M6.2e, then M6.3 onward),
which could only be stated in prose. Moved entries must stay byte-identical, proved by a script.

**Decision.**

1. **The tool moves entries; nobody edits them.** `cargo xtask records` moves every checked-off
   entry (its checkbox line and every line up to the next entry, stub or phase heading) from the
   roadmap to `docs/roadmap-done.md`, under the same phase heading, and never changes a line of
   it. Where a done entry's parent is still open, the archive holds a one-line stub for the parent
   (`- **M6.2** is open; its entry: [roadmap](...)`), linking the parent's phase; where an open
   entry's parent is done, the roadmap holds the matching "is done" stub. A stub has no checkbox,
   so neither the checks nor the site read it as a milestone. When a parent is finished in turn,
   its entry replaces its stub and its done increments stay under it.
2. **A file the move could get wrong is refused, not guessed at.** The roadmap's parser fails, with
   the line, on a heading other than a phase's after the first phase; a phase heading given twice;
   text under a stub; text under an increment indented less than the increment's own text (it
   would read as the parent's but move with the increment); a line the site reads as a milestone
   that the strict form `- [ ] **M1.2 Title.**` doesn't match; and a done parent the archive
   doesn't hold. A failure to read the archive, other than its absence, stops the command, and the
   archive is written before the roadmap, so a failed write can't lose a moved entry.
3. **The move was proved.** A one-off proof script compares `origin/main`'s
   roadmap with the two new files, entry by entry: 237 entries, 236 byte-identical and one, M0.5,
   with two lines appended after its unchanged text (its split note, ADR-146); none missing,
   duplicated or edited; every phase's intro unchanged. It prints the head's changed lines too:
   the rules and the new queue, which this record sets. The roadmap went from 1,000 lines and
   81,644 bytes to 282 lines and about 18.8 KB. `roadmap::tests` hold the code to the same rules,
   including a parent finished after its increments were archived, and idempotence.
4. **"In queue order" is read as an explicit queue, and the archive keeps whole entries.** Phases
   stay in the file and entries in their phases; a `## Queue` list in the head, one milestone id a
   line, is the order of work. ADR-144's "one line and a link each" is the archive's index, one
   line per entry linking its phase, kept beside the full entries rather than in place of them,
   so every *done when* stays readable in the tree, as the byte-identical rule requires.
5. **The check follows the queue.** The current milestone is the first queue entry that is open and
   not blocked, and when the queue runs out, the first such entry in file order with no open parent.
   An entry whose open increments are all blocked counts as blocked, so M2.3, whose only open
   increment is the blocked M2.3c, is no longer picked by file order; an entry under a parent marked
   blocked is blocked too. The project status names the current milestone or any link of the chain of
   first open, unblocked increments under it. The check fails unless the head has exactly one
   `## Queue` section, and on any list-like line in it that isn't `N. M<id> ...`, or that names a
   missing or done id, so the queue can't be skipped or go stale silently.
6. **Budgets the check enforces.** The roadmap's budget is now 24,000 bytes, each line at
   most 120 characters (the longest open entry's line is 118), and still 40 lines an entry,
   tested on fixtures over each limit. The archive has no budget: it is read on demand, by phase.
7. **The records page's tables follow the files** (M0.5d). The same command regenerates the
   page's table of milestones: a row per entry of the roadmap and its archive, ordered by phase
   and id, its status, and a link to its phase in whichever file holds it. Only each row's plain
   words are hand-written; a new milestone's row starts from its title. A decision record with no
   row gets one, from its title and summary, with milestone and record labels made links (19 rows
   added, ADR-125 to ADR-145, whose rows had never been written). The phase links and the record
   links are regenerated as well. A row for a milestone neither file holds fails the command.
8. **Every reader of the roadmap reads both files where status matters.** The Loft lessons check
   (a lesson's tests must be live once its milestone is done) and the site's milestone check read
   the roadmap and its archive together.

**Consequences.** Finishing a milestone is one checked box, one queue line, one command, the status
notes, the ADR if any, and start-here's table: the roadmap entry's move, the archive's index, the
records page's row and link, and the decision index follow from the command. The roadmap is about
a fifth of its old size. An archived entry can't be tightened in place any
more; a later change to met work is a new ADR, as the roadmap's rules already said. The 19 rows
added from summaries read less plainly than hand-written ones; improving a row's words by hand is
kept by later runs of the command.
