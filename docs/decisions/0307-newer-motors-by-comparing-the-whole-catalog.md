# ADR-307: Newer motors found by comparing ThrustCurve.org's whole catalog with the bundle (2026-10-29)

- **Status:** accepted; settles how [M5.7d][m5-7d] carries out [ADR-288][adr-288] §4; no done-when changes. First merged in #1040, withdrawn by [ADR-306][adr-306] and landed again with this number and the fixes of its reviews: the printed command for a new motor split into `hpr motors fetch NAME`, which reads it, and `hpr sim DESIGN --motor NAME`, which flies it, the second printed only where `hpr sim` reads the cache; and its words on delisted motors corrected
- **Summary:** `hpr motors update` asks ThrustCurve.org's search for every motor at once and compares the solid ones with the bundled catalog by id, file count and update day, downloading only the motors that may differ. Its date filters aren't used: one of them matched nothing on 2026-10-29. A newer motor is found offline after the update; a newer file of a bundled motor is only cached, and the bundled file keeps flying.

[m5-7d]: ../decisions-and-roadmap.md#m5-7d
[adr-154]: 0154-motors-fetched-from-thrustcurve-by-name.md
[adr-288]: 0288-every-solid-motor-ships-in-the-package.md
[adr-306]: 0306-withdraw-the-motor-update-to-land-it-in-order.md
[adr-298]: 0298-every-solid-motor-in-four-increments.md

**Context.** [ADR-288][adr-288] §4 says `hpr motors update` adds the motors and files newer than
the bundle to the cache when online, and offline says the bundle's date. [ADR-298][adr-298] made
that M5.7d. ThrustCurve.org's API (its `swagger.json`) offers two date filters on a search,
`infoUpdatedSince` and `dataUpdatedSince`. On 2026-10-29 (the filters probed live, not recorded;
the whole search recorded as a test fixture):

- `dataUpdatedSince` matched no motor for any date tried (2026-10-28T11:49:53Z, 2026-10-20,
  2026-10-01, 2026-01-01 and 2025-01-01), while
  `infoUpdatedSince` 2026-01-01 matched 43, the newest updated 2026-09-13;
- the whole search, `search.json?maxResults=5000`, returned all 1,156 records (928,550 bytes),
  1,004 of them solid, the same ids, file counts and update days as the 2026-10-28 snapshot;
- each solid record's `dataFiles` equals the number of files the snapshot listed for it, all 1,004.

**Decision.**

1. **One whole search, compared.** The update reads the whole search and keeps the single-use
   and reload motors. It downloads a motor's RASP and RockSim files when the bundle lacks the
   motor, holds another number of its files than its record lists, or its record's `updatedOn` is
   on or after the snapshot's day (the record gives a day, so that day counts). Every file whose
   id the bundle doesn't hold, indexed or left out, is newer. A search cut short by its
   `maxResults` is refused. The date filters aren't relied on.
2. **What flies.** The downloads are the answers [ADR-154][adr-154]'s fetch reads, and the update
   then looks each newer motor up by name as `hpr sim --motor` and `hpr motors fetch` do, so those
   answers are cached too, and prints the commands that find it. A motor the bundle lacks is then
   read offline by `hpr motors fetch` with its designation, and `hpr sim DESIGN --motor` with it
   flies it; where the bundle has a motor by that name (`hpr sim --motor` tries the bundle first)
   or several makers share it, `hpr motors fetch` with its maker reads it and no `hpr sim`
   command is printed. No command is printed when no file of the motor makes one HPR Sim
   flies, or a word of it is empty, starts with `-`, or holds a character other than ASCII
   letters and digits, a space and `/._+-`.
3. **Offline.** The update reads what an earlier one cached. With any answer it needs missing
   from the cache, it names the bundle's date and succeeds: the bundle flies as it is. Online,
   an answer under a day old comes from the cache, as every ThrustCurve.org answer does. With
   the network down, an update whose every answer is cached reads those copies, the search's
   staleness printed; with one missing, the command fails, its help naming the bundle's date.
4. **The bundle described.** The command prints the bundle's snapshot date, its motors (and how
   many fly), its files and those left out, the files passing the 1% check and their motors, and
   the bytes compiled in (the index and every curve file it reads). `hpr_motor` gives the counts
   (`Catalog::counts`) and the size (`bundled_size_bytes`); `cargo xtask motor-catalog` prints
   the same counts. The motor page and the command guide carry `hpr motors update --offline`'s
   output, which `cargo xtask cli --check` keeps current.
5. **Tested on recorded answers.** ThrustCurve.org listed nothing newer than the bundle on
   2026-10-29, so the tests replay that day's real answers twice: against the bundle as it is,
   which finds nothing, and against the bundle with one motor (ZEVETA ROS-40-5) and one file
   (AeroTech H128W's RockSim file) taken out, which finds exactly those, after which both are
   read offline from the cache (`crates/hpr-net/tests/catalog_update.rs` and the tests in
   `crates/hpr-cli/src/motors_update.rs`).

**Consequences.**

- A file swapped for another under the same motor, with the record's file count and update day
  unchanged, isn't found. Finding it would take two downloads for each of the 924 motors with a
  file, every update.
- An update with no search under a day old in the cache downloads about 0.93 MB for it (928,550
  bytes recorded), and two small answers for each motor that may differ.
- A motor ThrustCurve.org stops listing stays in the bundle, and flies as before where it has a
  file that reads; the update counts them.
