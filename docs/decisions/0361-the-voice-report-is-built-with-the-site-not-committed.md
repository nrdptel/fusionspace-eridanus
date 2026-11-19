# ADR-361: The voice report is built with the site, not committed (2026-11-19)

- **Status:** accepted; changes how [ADR-284][adr-284]'s voice check keeps its report: the copy under `validation/reports/` and the test that held it to the pages go, and the site's build writes the report onto its own page; the limits, the held set, the stock-phrase list and the rates stay as [ADR-285][adr-285] and [ADR-292][adr-292] set them
- **Summary:** Nearly every pull request rewrote the committed voice report, so any two that edited pages conflicted on it, and a conflicting pull request gets no CI run. The report is no longer committed. The site's build writes it onto a stub page, `docs/voice-report.md`, from the pages each time, and CI fails on exactly what it failed on before. A mutation run shows each of the eleven failures still failing CI's site command, and two branches that edit neighboring pages now merge cleanly where today they conflict.

[adr-284]: 0284-the-maintainers-own-voice.md
[adr-285]: 0285-the-rest-of-the-site-in-nine.md
[adr-292]: 0292-the-programs-words-in-four.md
[i-1495]: https://github.com/nrdptel/fusionspace-eridanus/issues/1495
[page]: ../voice-report.md
[guide]: ../writing.md#the-check
[voice-page]: ../../xtask/src/site/voice_page.rs
[voice]: ../../xtask/voice/src/voice.rs

**Context.** The voice check ([ADR-284][adr-284]) wrote a report of how each page reads to
`validation/reports/voice.md`, and a test failed when that file and the pages disagreed. Its
totals line moved with any edit to a page it covers, and each page's row is its own line, so two
pull requests that edited any pages conflicted on the totals, and two that edited neighboring
pages conflicted on the rows as well. [Issue #1495][i-1495] measured the cost: 37 of the 66
commits on `main` from 2026-11-08 to 2026-11-15 changed the report, and from 11-12 to 11-15 a
pull request that changed it was open 47% of the time. A conflicting pull request gets no CI run,
and merging `main` into it moves its fork point, which asks for a new full gate and new reviews.

**Decision.**

1. **The report isn't committed.** `validation/reports/voice.md` is deleted. A test in
   [the voice crate][voice] fails if it is tracked again.
2. **The site's build writes it.** The site lists a page, [How the pages read][page]
   (`docs/voice-report.md`), whose file is a stub with a marker line. mdBook's preprocessor, the
   one that already typesets the site's math, puts the report written from the pages in its
   place ([`voice_page.rs`][voice-page]). It refuses a page at that path without the marker, a
   table of contents that drops the page, and one that lists it twice, so the page can't be
   replaced unseen or go missing. The stub is itself a held page, so the check reads its words.
3. **The checks don't move.** `cargo xtask site`, which every CI site job runs, calls the voice
   check before it builds, as it did before: a stock phrase on any page fails it, and so does a
   held page over a limit or missing a sentence target. `cargo xtask voice` and `cargo voice`
   fail the same way and now print the report rather than write it, so a pull request's effect
   on a page's rates is still read locally.

**What is lost, and what isn't.** The deleted test compared the committed file with the pages;
it caught a stale file, not a page that reads badly. The report itself never failed a page: it
printed rates, and the failures came from the check, which still runs. The mutation run below changed one held page at a time, ran `xtask site` as CI's site jobs do,
and restored the page. The page was the stub (154 words), the glossary for the reference-page
limit, or the 404 page (51 words) for the two sentence targets, which a longer page dilutes:

| Mutation | Page | What failed |
|---|---|---|
| "It's important to note that…" | stub | a stock phrase |
| two bold-label list items | stub | bold-label items, 2 in 159 words |
| two internal labels | stub | labels, 2 in 161 words |
| one 30-word sentence | stub | sentences over 25 words, 1 in 184 words |
| three hedges | stub | hedges, 3 in 162 words |
| "might possibly" | stub | stacked hedges, 1 in 158 words (and hedges) |
| "Simply" | stub | minimizers, 1 in 157 words |
| "As noted above" | stub | signposts, 1 in 161 words |
| "I wrote this glossary." | glossary | first person on a reference page, 1 in 13,385 words |
| its text as three sentences of 22 to 23 words | 404 | the mean words a sentence, 67 words in 3 sentences |
| one 29-word sentence | 404 | the share over 25 words, 1 of 5 sentences (and sentences over 25 words) |

Each run exited 1 with that message. The site's page now shows the report for exactly the pages
it was built from, where the committed copy could trail them until the next commit refreshed it.

**What it gains.** Two branches off this change, one editing `CHANGELOG.md` and one
`CONTRIBUTING.md` (neighboring rows in the old report), each running `cargo voice` and
committing, merge with no conflict: `git merge-tree --write-tree` exits 0. The same two edits on
`main` before this change, each with its report rewritten, conflict in
`validation/reports/voice.md`: `git merge-tree` exits 1.

**Consequences.** The site's build reads every page once more for the report, about 6 seconds on
the maintainer's machine. `mdbook serve` runs the preprocessor too, so a local preview shows the
report. [The writing guide][guide] says where the report is and how the check runs.
