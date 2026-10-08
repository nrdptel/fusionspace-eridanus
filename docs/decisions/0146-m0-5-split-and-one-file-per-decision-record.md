# ADR-146: M0.5 split, and one file per decision record (2026-10-04)

- **Status:** accepted
- **Summary:** Leaner bookkeeping in nine increments, one per item of its plan; each decision record in its own file, with a generated index that keeps every older link working, and a number traced to a record now traced to that record alone

**Context.** ADR-144 §1 made M0.5 *Leaner bookkeeping* the next milestone, with nine items, a to
i. The decision log was one file of 946 KB and 11,957 lines holding 146 records; every link into
it named a heading's anchor (531 links, 150 distinct anchors, across docs, code comments,
schemas and generated reports). Item i, a report on the first 20 pull requests after M0.5, can't
hold until that work is done.

**Decision.**

1. **M0.5 is split into M0.5a to M0.5i,** one per item of ADR-144 §1, each with that item as its
   *done when*, unchanged. M0.5i is marked `[blocked]` once the others are done, until 20 pull
   requests have merged; the parent stays open until then.
2. **One file per record.** Each record moves to `docs/decisions/NNNN-slug.md` (MADR's layout):
   its heading, one level up (`# ADR-NNN: Title (date)`), then two lines from the old index's row,
   `- **Status:**` and `- **Summary:**`, then its body unchanged. Inside a body, a link into the old
   log now names the record's file, and a relative link gains the `../` the move needs (9 lines). A
   one-off proof script compares every file with `origin/main`'s log: 146 records, 0 problems, 9
   lines differing only by those rewrites. It also checks, apart from the rewrite rule, that every
   relative link in every record reaches a file; review caught that the first run had broken two in
   ADR-067 that way, and `records::tests::every_relative_link_in_a_record_resolves` now guards it.
3. **The index is generated.** `cargo xtask records` writes `docs/DECISIONS.md` from the files:
   a short head (what earns a record, after Nygard, Fowler and MADR, and how to write one) and
   one table row per record with its link, summary and status. Each row carries the anchor its
   heading had in the one-file log, as `<a id>`, so the 531 old links, and any outside the
   repository, still land on the record's row, one click from its file. The same command writes
   the records page's block of `[adr-NNN]: ...` definitions. `cargo test -p xtask` fails when
   either is stale (`records::tests::the_generated_records_are_current`), and a record file
   that doesn't parse, or a gap in the numbers, fails it too.
4. **Links in hand-written Markdown under `docs/` and in `THIRD-PARTY-NOTICES.md` now name the
   record's file** (209 links, rewritten by a one-off script). Code comments, schemas and generated
   reports keep theirs: they land on the index's rows, and rewriting them would regenerate schemas
   and reports for no reader's gain.
5. **A number traced to a record is traced to that record alone.** The site check traces each
   number on a model page's *In short* to a file its item links. A link to one record used to
   count the whole 946 KB log as the source, so a number matched anywhere in it. With the split,
   one page failed: *Rigid-body flight* gave the slow rockets' drifts as "4.7 to 43%" and the
   predicted apogees as −6.985% to +10.306%. Neither matched the record it linked; both had
   passed on unrelated numbers elsewhere in the log, and both were stale against the committed
   report. The page now quotes the report, which its item links: apogee
   drifts −4.333% to −38.158%, predicted apogees −7.280% to +10.302%. Three other pages carry the
   old figures, without the check's reach (it reads only *In short*): issue #298 tracks them.

**Consequences.** A new record is one new file plus `cargo xtask records`; the index row, the
records page's link and the old hand-kept table entry are no longer edited by hand. Reading one
record costs its own size, a median of 5.9 KB and at most 18 KB, instead of a slice of a 946 KB
file. The project's rules name the new layout. The rest of M0.5 follows in its own increments.
