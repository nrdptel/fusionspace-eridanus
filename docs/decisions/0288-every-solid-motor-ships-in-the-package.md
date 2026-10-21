# ADR-288: Every solid motor ships in the package (2026-10-21)

- **Status:** accepted; replaces ADR-287 §2's "free usage" line and §4's rule that only a passing curve is bundled; ADR-154's fetch stays for motors and files newer than the bundle
- **Summary:** M5.7 will compile ThrustCurve.org's solid-motor catalog into `hpr-motor`: every record, and every curve file HPR Sim reads but a restricted one, so no motor in the snapshot needs a fetch. Each file carries its check against its motor's published figures. A motor's default file passes where one does, and flying a failing file warns. `hpr motors update` adds newer motors and files when online. Until then, 32 motors come built in.

[adr-005]: 0005-solid-motors-statistics-consumption-grains-file.md
[adr-066]: 0066-every-curve-hpr-flies-is-integrated-as.md
[adr-067]: 0067-curves-come-from-openrockets-own-database-by.md
[adr-154]: 0154-motors-fetched-from-thrustcurve-by-name.md
[adr-161]: 0161-openrocket-curves-by-digest.md
[adr-284]: 0284-the-maintainers-own-voice.md
[adr-287]: 0287-thrustcurve-orgs-motor-records-used-as-published-facts.md
[m5-7]: ../decisions-and-roadmap.md#m5-7
[l42]: ../research/loft-lessons.md
[motor]: ../physics/motor.md#the-bundled-motors
[tc-note]: ../research/thrustcurve-data.md
[tc-size]: ../research/thrustcurve-data.md#7-the-whole-solid-catalog-2026-10-21
[ideas]: ../research/ideas.md

**Context.** `hpr` ships 32 motors that a flyer can name ([the motor page][motor]). Any other
motor flies from a file the flyer has, or from a `.ork`'s own curve, or is fetched from
ThrustCurve.org the first time it flies ([ADR-154][adr-154]). So a flyer with no signal and no
cache can name only those 32. OpenRocket ships a motor database built from ThrustCurve.org inside
its program: 1,452 entries in 24.12, 164 of them hybrids ([ADR-067][adr-067]).
[ADR-287][adr-287] let the project use ThrustCurve.org's records as published facts and placed
bundling them as an idea.

On 2026-10-21 the maintainer decided that the package should carry every motor's data, so it
works offline with nothing to fetch, with as much data and as many options as possible.

The catalog is small ([the size note][tc-size]). The 2026-09-17 capture held 1,004 solid motors,
278 of them out of production, and 1,712 curve files. That's about 2.7 MB as text (2.05 MB of
curve files, 0.61 MB of records) and about 0.4 MB compressed with xz. What it adds to the program
and the web build is measured in M5.7. 726 motors have two to four files. 666 of those have
exactly two, one RASP and one RockSim file, and 335 have files from more than one source (a
certification test, the maker, a user). 80 motors have no file. A one-off script gave these
counts; M5.7's tool replaces them.

**Decision.** None of this is built yet. Until M5.7 ships, [the motor page][motor] describes what
the program does.

1. **The whole solid catalog is compiled in.** Every solid motor's record and every curve file
   HPR Sim reads go into `hpr-motor`, from one dated snapshot. Out-of-production motors are
   included.
   - M5.7 takes a fresh snapshot. The site has changed since the capture: by 2026-10-17, 41
     unlicensed files had been deleted and 34 public-domain files added ([the note][tc-note]).
   - A motor with no curve is listed but can't fly.
   - HPR Sim's reader refuses 2 of the 1,712 files, RockSim files whose times run backwards
     ([the motor page][motor] counts the 1,710 that read). Both motors have a RASP file that reads.
     The tool lists each file it leaves out, with the reason.
   - A file whose uploader marked it "other (restricted)" stays out. On 2026-10-17 the site had
     none.
   - Hybrids wait for release 1.0's scope.
2. **"Free usage" files go in like the rest.** On 2026-10-20 the maintainer judged all of the
   site's data free to use as facts ([ADR-287][adr-287]). ADR-287 still kept these files out,
   because the license might be copyleft, which asks that work built on the file carry the same
   terms. This ADR drops that line:
   - The uploaders chose a free license so that others could use the files. That's more
     permission than the 1,084 files with no license have (2026-10-17).
   - The bundle uses each file's measured points as facts, as ADR-287 uses the records.
   - The risk left is that one of these files is copyleft and its terms reach the bundle. Each
     file's label is kept beside it, so the 33 (2026-10-17) can be found and taken out if anyone
     asks. In the 2026-09-17 capture, 7 motors had only free-usage files, so taking those out
     would leave them with no curve.

   This line is the project's own reading, told to the maintainer with the recommendation. It
   isn't legal advice.
3. **Every file keeps its check, and the check moves to the flight.** ADR-287 §4 bundled only a
   curve that passes the bundle's rule ([ADR-005][adr-005]): within 1% of its motor's published
   total impulse, average thrust and burn time, burn time as ThrustCurve.org defines it
   (definition A in [the note][tc-note], section 5). Meanwhile ADR-154 flies a fetched file
   without that check. Now every file is checked when the bundle is built, and the result is
   stored with it.
   - A motor's default file is one that passes, where one does, chosen by ADR-154's ranking by who
     measured it. With none passing, it's the first by that ranking. A `.ork` motor whose
     OpenRocket curve maps to a file by its digest ([ADR-161][adr-161]) still takes that file
     first.
   - `hpr motors show` lists every file with its check and its label.
   - Flying a file that fails prints a warning naming each failing figure and its signed gap.

   Only 416 of the 1,712 files passed in the 2026-09-17 survey ([the ThrustCurve note][tc-note],
   section 5), so the warning will be common. How many motors have a passing file isn't counted
   yet; M5.7's tool prints it. A pass means the file agrees with the published figures within
   1%. It says nothing about the curve's shape between them.
4. **The bundle has a date, and the network is optional.** A release can refresh the snapshot,
   and the motor page states its date and counts. `hpr motors update` adds motors and files newer
   than the bundle to the cache when online. Offline it says the bundle's date. `hpr motors fetch`
   stays.
5. **Credit** is ADR-287's: ThrustCurve.org's credit and link on every output that shows its
   figures, and `THIRD-PARTY-NOTICES.md` names the bundle and this ADR.

**Consequences.**

- [M5.7][m5-7] does the work, third in the queue, after M0.14 (the docs in the maintainer's
  voice) and M0.9f (the docs' number and date style). [ADR-284][adr-284] §6 keeps those two
  together, and M0.9f's check on long `hpr sim` warnings will then cover M5.7's new warning.
  Release 0.2's milestones don't depend on M5.7, but it goes ahead of them, so release 0.2 moves
  back by its length.
- The checks that hold the bundle to 1% today keep holding, over every file stored as passing:
  [lesson L42][l42]'s test (`every_bundled_curve_impulse_within_1pct_of_thrustcurve`), the test
  against ThrustCurve.org's own statistics code
  (`every_bundled_curve_matches_thrustcurve_statistics_code`), with its oracle fixture regenerated
  to cover each file, and `cargo xtask motor-catalog`'s failure over 1%, run in CI rather than only
  where the uncommitted captures are. A recount from the raw snapshot, outside the stored results,
  must agree with them, so a file can't escape the checks by being marked failing.
- The 32 bundled curves keep [ADR-066][adr-066]'s check: hpr's total impulse within 0.1% of
  OpenRocket's integration of the same file. M5.7 doesn't extend that check to the other files.
  If the default rule picks a different file for one of the 32, M5.7's PR regenerates the reports
  that fly it.
- The idea placed by ADR-287 leaves [the ideas list][ideas]. M5.7 takes over its done-when, a
  listing's figures equal to the records.
- Certification and maker data sheets might give curves for some of the 80 motors without one.
  That needs research first.
