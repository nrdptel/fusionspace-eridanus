# ADR-301: The whole catalog in two increments, the index before the lookups (2026-10-28)

- **Status:** accepted; amends [ADR-298][adr-298] §1 by splitting [M5.7b][m5-7] into M5.7b1 and M5.7b2, and plans every remaining increment of M5.7 (M5.7b1, M5.7b2, M5.7c, M5.7d); M5.7c and M5.7d keep their *done when* unchanged
- **Summary:** ThrustCurve.org's API answered again on 2026-10-28, and a fresh snapshot was taken: 1,004 solid motors and 1,706 files. Compiling it in is one session's work; switching every lookup to it is another, because it moves flights, shared names and reports. M5.7b1 compiles the snapshot in, with each file's check, beside M5.7a's 32 motors, which the lookups keep using. M5.7b2 switches the lookups to the whole catalog.

[m5-7]: ../decisions-and-roadmap.md#m5-7
[adr-005]: 0005-solid-motors-statistics-consumption-grains-file.md
[adr-186]: 0186-data-network-and-python-fixes.md
[adr-287]: 0287-thrustcurve-orgs-motor-records-used-as-published-facts.md
[adr-288]: 0288-every-solid-motor-ships-in-the-package.md
[adr-298]: 0298-every-solid-motor-in-four-increments.md
[l43]: ../decisions-and-roadmap.md#l43

**Context.** M5.7b is done when "a fresh snapshot's every solid motor and every file but a
restricted one are in `hpr-motor`, figures and labels equal to the snapshot's; each file left out
is listed with its reason, and one past the snapshot's named refusals fails; offline, `hpr motors
list` lists each motor and `hpr sim --motor` flies each with a curve, the counts taken from the raw
snapshot; M5.7a's check, default, listing and warning hold on every file it adds, failing files
stored and warned of."

The snapshot, taken 2026-10-28T11:49:53Z by `validation/oracles/thrustcurve/snapshot.py`:

- 1,004 solid motor records (220 single-use, 784 reload) and 1,706 files for them. No file is
  marked restricted ("other"); 589 are public domain, 33 free to use, and 1,084 carry no license.
- 1,701 files are indexed. Five are left out: two RockSim files whose times run backwards, two
  reload files whose header says 75 mm for a 54 mm case, Loft lesson [L43][l43]'s error, and one
  held back (§5). 924 motors have a file that reads; 80 have none.
- 412 files pass [ADR-005][adr-005]'s 1% check against their motor's record, and 239 motors have
  one that does.

The surface half of the bullet was tried on the branch: with every lookup on the whole catalog,
about 40 of the command-line tests failed. The causes are separate pieces of work:

- 32 designations are shared by two or more makers, so a bare name that found one motor among the
  32 finds several.
- ADR-298 §5's default puts a passing file first. On the whole catalog two of the 32 change file
  (a D5 and an F15 move from a user's RockSim file to a certification test's RASP file). Their
  thrust points give the same total impulse and burn time, but the files differ in format and
  header, so whether a report that flies them moves is M5.7b2's to measure.
- ADR-288 §3 asks a `.ork` motor's catalog lookup to take the file its OpenRocket curve maps to by
  digest first; with one file a motor, that had nothing to choose between.
- The oracle tests (lesson L42's and the statistics code's) hold to the 32 until M5.7c, and the
  converter's round trip over every file is untested on the 1,669 new ones.

**Decision.**

1. **M5.7b splits in two.** Each half takes words of M5.7b's *done when*, and together they take
   all of it:

   | Increment | Its *done when* |
   |---|---|
   | M5.7b1 The whole catalog compiled in | a fresh snapshot's every solid motor and every file but a restricted one are in `hpr-motor`, figures and labels equal to the snapshot's; each file left out is listed with its reason, and one past the snapshot's named refusals fails; M5.7a's check holds on every file it adds, failing files stored |
   | M5.7b2 Every motor by name, offline | offline, `hpr motors list` lists each motor and `hpr sim --motor` flies each with a curve, the counts taken from the raw snapshot; M5.7a's default, listing and warning hold on every file it adds, failing files warned of |

   M5.7c and M5.7d follow, as ADR-298 §1 wrote them. M5.7c's oracle tests run over M5.7b1's
   files; M5.7d's update compares with M5.7b1's snapshot date.
2. **Two indexes until M5.7b2.** `Catalog::whole()` reads the new index of every motor.
   `Catalog::bundled()`, which every lookup uses, reads `catalog-m57a.json`, M5.7a's 32-motor index
   frozen as it was, so M5.7b1 moves no flight and no report. A test holds each of the 32 to the
   whole index: the same record figures, labels and file bytes, and the same check. M5.7b2 points
   `bundled()` at the whole index, deletes the frozen copy, makes a name shared by several makers
   take its maker, puts the digest's file first (ADR-288 §3), and regenerates the reports the new
   defaults move.
3. **What is committed.** Under `crates/hpr-motor/data/thrustcurve/`: every solid record copied
   unchanged from the search answer (`records.json`, used as published facts, [ADR-287][adr-287]);
   every file entry of the download answers without its bytes (`files.json`); and each file's
   bytes, byte for byte (`curves/`). The answers themselves stay under the ignored
   `refs/snapshots/`. Where they are present, `cargo xtask motor-catalog` fails unless the
   committed records, file list and bytes are theirs, and prints the counts it takes from them.
   The data adds about 2 MB of curve files and a 2 MB index to the crate.
4. **Labels and figures.** A motor's names, type, case, propellant and stated figures are its
   record's, unchanged. Its listed figures stay its default file's own ([ADR-186][adr-186]), the
   file ADR-298 §5 ranks first; a motor with no file that reads lists its record's figures and
   can't fly. "Figures equal to the snapshot's" means the record's figures as stored, and each
   file's figures as its own bytes give them.
5. **Which files are left out.** A file is left out, with its reason, when it is restricted, its
   text doesn't read as one motor's curve, its header's sizes or masses can't be a motor's, or a
   reload's header diameter is more than 10% from its case's size. The header sizes in this
   snapshot sit up to 7.3% from their case's (a 161 mm Pro150, a 75 mm 76/6000); the nearest
   standard sizes, 29 and 32 mm, are 10.3% apart; L43's error is 39%. `NAMED` in
   `xtask/src/motor_catalog.rs` lists the five files left out; one it doesn't list fails the
   command. One of the five is held back: a check on what this repository commits refuses its
   bytes until the maintainer clears them, so `snapshot.py` doesn't write it (its `HELD`), and
   the index lists it as left out with that reason, unread. That reason is not the file's own,
   so M5.7b1 takes it in once the maintainer clears it, as a fix to this increment.
6. **Tests held to the 32 until M5.7c.** The lesson-L42 and statistics-code oracle tests and the
   converter's round trip run on M5.7a's 32 files. The round trip over every file is
   [issue #1002](https://github.com/nrdptel/fusionspace-eridanus/issues/1002).

**Consequences.**

- No flight, report or command output changes in M5.7b1; `hpr` still flies 32 motors by name.
- The crate grows by about 4 MB of data. ADR-288 accepted the size; M5.7d's page will give it from
  the tool.
- `validation/oracles/thrustcurve/bundle.py`, which fetched the 32 files, is gone; `snapshot.py`
  takes its place.
