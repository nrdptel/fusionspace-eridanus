# ADR-112: M3.3b: the `.hprz` container and migrations (2026-09-29)

- **Status:** accepted
- **Summary:** M3.3b: the `.hprz` container, the first migration (0.2), and the design format on the command line

**Context.** M3.3b asks for a zip container of a design and its attachments that reads back the
same, a migration test from an older version to the current one, the comparison with `.ork`,
`.rkt`, `.CDX1` and `.rpy`, and `hpr convert` writing and `hpr sim` reading `.hpr`. Version 0.1
(ADR-111) was the only version, so a migration had nothing to start from, and its document already
used `attachments` for the source `.ork`'s other entries, which the brief's container calls
"attachments" too.

**Decision.**

1. **Version 0.2 renames `attachments` to `source_files`** (type `SourceFile`), leaving
   "attachment" to mean a file carried beside the design in a `.hprz`. It is a real change made to
   documents that already exist, so it takes a new version with a migration, and 0.1's schema stays
   committed beside 0.2's.
2. **The version policy, amended.** ADR-111 §3 let 0.1 change in place until a release. From 0.2,
   while the major version is 0, the current version may still change in place only by a change
   that leaves every document already written readable with the same meaning; anything else (a
   rename, a removal, a key made required, a meaning changed) takes a new minor version, a
   migration from the one before, the old schema kept, and a document the old version's program
   wrote committed under `crates/hpr-format/fixtures/`, which a test migrates.
3. **Migrations work on the JSON, oldest first.** Each step rewrites the top-level object of one
   version into the next, since the old types are gone; the result is then read as a current
   document, so it is held to every current rule. A step refuses what its version doesn't
   define. `read_json` returns the version the text was written in; `from_json` migrates silently.
4. **The source's airframe, recorded.** `hpr sim --motor` refuses a `.ork` rocket whose airframe
   or a motor mount was not read exactly as written (ADR-106): a part left out, a value dropped,
   something assumed. A document keeps no reader's warnings, and the `.ork` written from it can
   state outright what the source left to be assumed: on the corpus, 2 of 73 designs gave another
   reason, or none, when their written `.ork` was read again. So 0.2's `provenance.source` records
   `airframe_not_as_written`, set by `DesignFile::from_ork`. 0.1 kept it only in a configuration
   left out for it, which the `.ork` reader asks after the motors and their ignition; the migration
   works it out: a configuration left out for the airframe gives the reason; one that flies, or is
   left out only for its separation (the one check after the airframe), shows the airframe was
   read as written; otherwise, including a document with no configuration, it is marked unknown
   (`migrate::UNKNOWN`), and `hpr sim` refuses another motor, saying so. On the corpus it recovers
   5 of the 8 reasons and marks 3 unknown, marks 28 of the 65 designs read as written unknown, and
   is never wrong; `cargo xtask ork` fails on a wrong one.
5. **The container.** A `.hprz` is a zip archive whose first entry, `design.hpr`, is the document
   exactly as a `.hpr` file, so unzipping one gives a `.hpr`; every other entry is an attachment,
   kept byte for byte in order. Entries are deflated and dated 1980-01-01, so with one build the
   bytes depend only on the contents. An attachment's name is a relative path with `/` between
   folders: none of its parts empty, `.` or `..`, longer than 255 bytes, ending in `.` or a space,
   or naming a Windows device with or without an extension (`CON`, `CONIN$`, `COM0` to `COM9`,
   `LPT¹` and the like), no `\`, `:` or control character, not `design.hpr` in any case; no two
   names the same ignoring case (compared upper-cased then lower-cased, so `ς`, `σ` and `Σ` meet,
   and `ß` meets `ss`, a conservative refusal), none also another's folder, and none in a
   top-level folder named `design.hpr`. A container holds at most 256 MiB unpacked, which
   the writer and the reader both hold to. The reader checks every name before it decompresses,
   passes over a folder's own entry if it is empty, refuses a symbolic link and a name beyond ASCII
   not marked as UTF-8, and
   refuses an archive whose central directory has more records than the zip reader keeps, which it
   does when two share a name (it keeps one and drops the other without a word). Unicode
   normalization (two spellings of one accented letter) is not checked.
6. **The command line.** `hpr convert` takes a design from any of `.ork`, `.hpr` and `.hprz` to
   any, by extension, and `--attach` adds files to a `.hprz` under their file names; a `.hprz`'s
   attachments that the output has no place for are warnings. Its JSON is the motor document
   unchanged or a design document with `rocket` (untagged, so no existing output changes). A
   conversion keeps the provenance it read: it names where the design came from, which rewriting it
   doesn't change. `hpr sim` reads `.hpr` and `.hprz` as it reads the `.ork`, with the airframe's
   reason from the provenance; a migrated document and a container's unread attachments are notes.

**Measured** (2026-09-29, `cargo xtask ork`): the 73 corpus documents are valid against the 0.2
schema, read back the same, write M3.2a's `.ork` byte for byte, and read back from it as first read;
each, rewritten as 0.1, migrates back to the same document but for the airframe's reading,
which 0.1 didn't record.
109 configurations fly three ways to the same apogee (largest relative difference 0).

**Not chosen: taking the airframe's reason from the `.ork` written from the document.** It is what
the first draft did; the corpus check above showed it wrong for 2 designs, one of them flyable
with another motor where its `.ork` is not.

**Not chosen: reading a 0.1 document with no sign of its airframe as read as written.** The first
draft of the migration did; review showed a rocket with a part left out flying another motor from
its 0.1 document. Marking it unknown means `hpr sim` refuses another motor in 28 corpus designs
that could fly one, until their `.ork` is converted again; that is the cheaper mistake.

**Not chosen: the source's files as container entries.** A `.hprz` could store `source_files` as
entries rather than inside the document. Keeping the document whole means the container's design
is a `.hpr` any reader takes, and a `.hpr` and a `.hprz` hold one design the same way.

**Not chosen: changing 0.1 in place.** The rename breaks every 0.1 document already written, and
a migration needs an older version to prove itself on.
