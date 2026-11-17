# ADR-354: The search index is held to mdBook's own bar, not to its weight on 2026-11-10 (2026-11-17)

- **Status:** accepted; changes the size limit that `cargo xtask site` holds the search index to ([the writing guide's section][guide]); written with [M1.14b3][m1-14b3], whose pages did not fit under the old limit
- **Summary:** The site's search index was held to its weight on 2026-11-10, 1,675,059 bytes compressed. The file stores every section's text, so that limit held the guide to the words it had that week: a week later main had 2,684 bytes of room, about 920 words. The limit is now 10,000,000 bytes of the file, the size past which mdBook itself calls an index very large. That is room for about 139,000 more words; [#1522](https://github.com/nrdptel/fusionspace-eridanus/issues/1522) decides what follows.

[m1-14b3]: ../decisions-and-roadmap.md#m1-14b3
[guide]: ../writing.md#the-search-index
[adr-353]: 0353-nasas-check-cases-1-and-2-both-differences-past-the-codes-spread-traced.md
[voice]: ../../validation/reports/voice.md
[i-414]: https://github.com/nrdptel/fusionspace-eridanus/issues/414
[i-1522]: https://github.com/nrdptel/fusionspace-eridanus/issues/1522
[main-run]: https://github.com/nrdptel/fusionspace-eridanus/actions/runs/44104806244

**Context.** mdBook writes one file, `searchindex.js`, that the search box reads. mdBook warns
when the index passes 10 MB, and in early November the guide sat at that warning with fifteen
pages left out of search ([#414][i-414]). The cause was a fault in the search library, which
filed each section's words two and three times. Since 2026-11-10 `cargo xtask site` rebuilds
the index without the copies and every page is in search again.

That fix set a new limit: the rebuilt file, compressed with gzip, could weigh no more than
main's file had before the rebuild, 1,675,059 bytes. The rebuilt file came to about 1.60 MB
that day, which left about 75 KB of room. The rule beside the limit: shorten the words a page
added, and raise the limit only with a decision record that measures the new one. This is
that record.

The measurements, each from a build's `site search:` line and [the voice report][voice]'s
word count:

| Build | Words on the site's pages | The file, bytes | Compressed with gzip, bytes |
|---|--:|--:|--:|
| main at fdb60fc2 ([its CI run][main-run], the first site job) | 574,042 | 8,029,039 | 1,672,375 |
| this increment, with its pages | 576,388 | 8,058,955 | 1,678,711 |

- **Main had 2,684 bytes of room.** Its file weighs 2.9 compressed bytes for each word on the
  site, so the room was about 920 words.
- **The file's weight is the guide's words.** The index keeps every section's text, which the
  search shows excerpts from, beside the list of words. So a limit on the file is a limit on
  how long the guide may be. The old limit fixed that length at what the guide held on
  2026-11-10 plus 75 KB, and one week of writing used the 75 KB up.
- **This increment's pages add 2,346 words and 6,336 compressed bytes,** 3,652 past the old
  limit. They are [M1.14b3][m1-14b3]'s account of two check cases ([ADR-353][adr-353]): a
  table, two causes and what the comparison does not see, on the pages the milestone's *done
  when* names. The room held about two fifths of those words, and the next increment's
  pages would meet the same limit with no room at all.

**Decision.**

1. **The limit is 10,000,000 bytes of the file as written** (`LIMIT_BYTES` in
   `xtask/src/site/search.rs`). That is the size past which mdBook logs that a search index
   is very large, the bar [#414][i-414] was opened about. mdBook counts the index before it
   is wrapped and escaped for the file, a little less than the file itself (9,999,756 against
   10,020,860 bytes for main's file at b7313b32), so this check trips before mdBook's would.
2. **Why that number.** The project has no measurement of its own of what size hurts a
   reader. mdBook's warning is the one bar the tool has, and #414 was opened against it. A
   limit raised by some margin over today's file would be this record's guess.
3. **What it costs a reader.** The file is fetched once, compressed, when a reader first
   opens the search. Today that is 1,678,711 bytes. At today's ratio of compressed to written
   bytes, a file at the limit would be about 2.08 MB, 24% more than the old limit allowed.
   That figure is an estimate: the ratio moves with what the pages say. The build's summary
   line prints the compressed size on every run, so the cost stays in view.
4. **How long it lasts.** The file is at 8,058,955 bytes, so 1,941,045 are left: about
   139,000 words at today's 14 bytes a word. From 2026-11-10 to main at fdb60fc2 the
   compressed file grew by about 72 KB, some 25,000 words in a week. At that rate the room
   lasts five to six weeks. The rate is one week's.
5. **What follows is [#1522][i-1522].** mdBook's bar is the last one to move to. Before the
   file reaches it, a decision record measures the ways to make the index lighter (long
   reference tables out of search, long figures out of the word list, a search for each part
   of the guide) and picks one.

**What doesn't change.** Every page stays in search, and the test that fails when
`book.toml` leaves one out stands. The rebuild and its checks against mdBook's own counts
stand. The rule beside the limit stands: shorten the words a page added first, and change
the limit only with a decision record that measures the new one.
