# ADR-298: Every solid motor in four increments, each file's check first (2026-10-27)

- **Status:** accepted; splits [M5.7][m5-7] into four increments, M5.7a to M5.7d, and plans every remaining increment, each with one of M5.7's *done when* bullets as its own, word for word, M5.7a's on the bundled files and M5.7b's widened to hold it on every file M5.7b adds
- **Summary:** M5.7 compiles ThrustCurve.org's whole solid catalog into `hpr-motor` ([ADR-288][adr-288]). It is several sessions' work, and its first bullet needs a fresh snapshot from ThrustCurve.org's API, which has answered HTTP 503 since 2026-10-25. M5.7a builds each file's stored check, the default-file rule and the warning on the 32 bundled files first; M5.7b compiles in the fresh snapshot; M5.7c holds every passing file to the oracles; M5.7d adds `hpr motors update` and the motor page's figures.

[m5-7]: ../decisions-and-roadmap.md#m5-7
[adr-005]: 0005-solid-motors-statistics-consumption-grains-file.md
[adr-154]: 0154-motors-fetched-from-thrustcurve-by-name.md
[adr-161]: 0161-openrocket-curves-by-digest.md
[adr-186]: 0186-data-network-and-python-fixes.md
[adr-287]: 0287-thrustcurve-orgs-motor-records-used-as-published-facts.md
[adr-288]: 0288-every-solid-motor-ships-in-the-package.md
[issue-885]: https://github.com/nrdptel/fusionspace-eridanus/issues/885

**Context.** M5.7 is done when four things hold ([ADR-288][adr-288] gives the plan):

1. a fresh snapshot's every solid motor and every file but a restricted one are in `hpr-motor`,
   with the files left out listed and `hpr motors list` and `hpr sim --motor` covering each motor
   offline;
2. each file stores its [ADR-005][adr-005] 1% check, the default file passes where one does,
   `hpr motors show` lists every file with its check and label, and flying a failing file warns;
3. the lesson L42 and statistics-code tests run over every file stored as passing, with the oracle
   fixture covering each, `xtask motor-catalog`'s 1% failure runs in CI, and a recount agrees;
4. `hpr motors update` adds newer motors and files, and the motor page gives the bundle's date,
   counts and size from the tool.

A survey of main (93a40c5b) on 2026-10-27 found:

- **The bundle:** 32 motors, one file each, all public domain. `cargo xtask motor-catalog` writes
  the index from the files alone ([ADR-186][adr-186]): every figure is the file's own, and none of
  ThrustCurve.org's record figures is committed. The 1% comparison with those figures runs only
  where the uncommitted capture sits under `refs/`.
- **The network:** ThrustCurve.org's API answered HTTP 503 on every request on 2026-10-25,
  2026-10-26 and 2026-10-27, with a maintenance or error page ([issue #885][issue-885]). The cached
  2026-09-17 capture holds 41 files the site deleted by 2026-10-17, so it can't stand in for a
  fresh snapshot.
- **The choice of file:** a catalog motor flies its first bundled file. A fetched motor flies the
  first RASP file that reads, ranked by who measured it, then the RockSim files the same way
  ([ADR-154][adr-154]). A `.ork` motor the catalog lacks is fetched, and the file its OpenRocket
  curve maps to by its digest goes first ([ADR-161][adr-161]); a catalog motor's digest isn't read.

Only the first bullet needs the network. The second needs each file's figures and its motor's
record figures, which the pinned capture holds for the 32 bundled motors.

**Decision.**

1. **The increments,** in this order:

   | Increment | Its *done when* | Needs |
   |---|---|---|
   | M5.7a Each file's check and the default file | M5.7's second bullet, on the bundled files; the 32 motors' record figures committed | nothing new |
   | M5.7b The whole catalog, from a fresh snapshot | M5.7's first bullet, and M5.7a's second-bullet rules on every file it adds | ThrustCurve.org's API |
   | M5.7c Every passing file held to the oracles | M5.7's third bullet | M5.7b's files |
   | M5.7d Newer motors online, and the bundle described | M5.7's fourth bullet | M5.7b's bundle |

2. **The order.** M5.7a needs no network, and it fixes the shape M5.7b's files land in: a file's
   own figures, its motor's record figures and the check between them. M5.7b then mostly writes
   data, and its *done when* also holds M5.7a's check, default, listing and warning on every file
   it adds, so the second bullet covers the whole catalog, not only the 32. `xtask
   motor-catalog` refuses a file more than 1% off today, the rule the 32 were chosen by; M5.7b
   stores such files instead, each with its failing check. M5.7c's oracle fixture runs
   ThrustCurve.org's statistics code over M5.7b's files. M5.7d's update compares with M5.7b's
   bundle date.
3. **The record figures are committed.** [ADR-287][adr-287] lets the project use ThrustCurve.org's
   records as published facts. M5.7a commits the 32 bundled motors' records verbatim, copied from
   the pinned 2026-09-17 capture, beside the curve files. `cargo xtask motor-catalog` reads them,
   stores each file's check in the index, and, where the capture sits under `refs/`, fails if a
   committed record differs from it. The motor's listed figures stay its default file's own
   ([ADR-186][adr-186]); the record's sit beside them.
4. **The check.** A file passes when its total impulse, average thrust and burn time are each
   within 1% of its motor's record: `|file − record| ≤ 0.01 × record`, so a gap of exactly 1%
   passes, as `xtask motor-catalog` has measured it since M1.3. The figures are definition A of
   the ThrustCurve note: impulse over the whole curve, burn time between the 5%-of-peak crossings,
   and average thrust as impulse over that burn time. The index stores both figures of each pair
   and the result. A test recomputes every stored check from the file and the record.
5. **The default file.** Among a motor's bundled files, one that passes goes first, then ADR-154's
   order: RASP before RockSim, each ranked by who measured it (a certification test, the maker,
   a user), the index's order within a rank. While each catalog motor has one file, a `.ork`
   motor's digest has nothing to choose between there. When M5.7b gives a motor several, its
   catalog lookup takes the file the digest maps to first, as [ADR-288][adr-288] §3 asks.
6. **The warning** names the motor, the file and each failing figure's signed gap, file against
   record. A figure inside 1% isn't named.
7. **If ThrustCurve.org's API is still down** when M5.7b comes up, M5.7b is blocked on it, not on
   the maintainer. The next unblocked milestone in the queue goes first.

**Consequences.**

- M5.7a changes no flight: all 32 bundled files pass, so each motor's default stays its one file.
- The motor page stops saying none of ThrustCurve.org's figures is bundled.
- M5.7b's PR regenerates any report that flies one of the 32 if the default rule picks another
  file for it ([ADR-288][adr-288]).
