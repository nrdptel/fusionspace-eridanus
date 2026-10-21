# ADR-288: Every solid motor ships in the package (2026-10-21)

- **Status:** accepted; replaces ADR-287 §2's "free usage" line and §4's rule that only a passing curve is bundled; ADR-154's fetch stays for motors newer than the bundle
- **Summary:** M5.7: ThrustCurve.org's whole solid-motor catalog, every record and every curve file but a restricted one, is compiled into `hpr-motor`, so no motor needs a fetch; each file carries its check against its motor's published figures, a motor's default file is a passing one where it has one, flying a failing file warns, and `hpr motors update` adds newer motors when online.

[adr-005]: 0005-solid-motors-statistics-consumption-grains-file.md
[adr-066]: 0066-every-curve-hpr-flies-is-integrated-as.md
[adr-067]: 0067-curves-come-from-openrockets-own-database-by.md
[adr-154]: 0154-motors-fetched-from-thrustcurve-by-name.md
[adr-161]: 0161-openrocket-curves-by-digest.md
[adr-287]: 0287-thrustcurve-orgs-motor-records-used-as-published-facts.md
[m5-7]: ../ROADMAP.md#phase-2-library-surfaces-and-interop
[motor]: ../physics/motor.md#the-bundled-motors
[tc-note]: ../research/thrustcurve-data.md
[ideas]: ../research/ideas.md

**Context.** `hpr` ships 32 motors ([the motor page][motor]). Any other motor is fetched from
ThrustCurve.org the first time it flies ([ADR-154][adr-154]), so a flyer with no signal and no
cache can fly only those 32. OpenRocket ships ThrustCurve.org's whole catalog inside its program:
1,452 motors in 24.12, hybrids included ([ADR-067][adr-067]). [ADR-287][adr-287] let the project
use ThrustCurve.org's records as published facts and placed bundling them as an idea.

On 2026-10-21 the maintainer asked for every motor's data in the package, so it works offline
with nothing to fetch, and said more data and more options are better.

The catalog is small. The 2026-09-17 capture ([the ThrustCurve note][tc-note]) held 1,004 solid
motors, 278 of them out of production, and 1,712 curve files. That's about 2.7 MB as text and
about 0.4 MB compressed. 726 motors have two to four files: a certification test's, the maker's,
users'.

**Decision.**

1. **The whole solid catalog is compiled in.** Every solid motor's record and every curve file go
   into `hpr-motor`, from one dated snapshot. Out-of-production motors are included. A file whose
   uploader marked it "other (restricted)" stays out; the site has none. Hybrids wait for release
   1.0's scope.
2. **"Free usage" files go in like the rest.** ADR-287 kept them out because the license might be
   copyleft. But their uploaders chose a free license so that others could use them. That's more
   permission than the 1,084 files with no license have. The rule is to follow each uploader's
   evident wish: public domain, free usage and unlabelled files are shared; a restricted one
   isn't. Each file's label is kept beside it. On 2026-10-17 there were 33 such files; 7 motors
   had no other file in the 2026-09-17 capture.
3. **Every file keeps its check, and the check moves to the flight.** ADR-287 §4 bundled only a
   curve that passes the bundle's rule ([ADR-005][adr-005]): within 1% of its motor's published
   total impulse, average thrust and burn time. Meanwhile ADR-154 flies a fetched file without that
   check. Now every file is checked when the bundle is built, and the result is stored with it.
   - A motor's default file is one that passes, where one does, chosen by ADR-154's ranking by who
     measured it. With none passing, it's the first by that ranking. A `.ork` motor still takes
     first the file holding its digest ([ADR-161][adr-161]).
   - `hpr motors show` lists every file with its check.
   - Flying a file that fails prints a warning naming the figure and the gap.

   Only 198 of 588 public-domain files passed on 2026-10-17, so the warning will be common. It
   says by how much, so a flyer can judge it.
4. **The bundle has a date, and the network is optional.** Each release refreshes the snapshot,
   and the motor page states its date and counts. `hpr motors update` adds motors newer than the
   bundle to the cache when online. Offline it says the bundle's date. `hpr motors fetch` stays.
5. **Credit** is ADR-287's: ThrustCurve.org's credit and link on every output that shows its
   figures, and `THIRD-PARTY-NOTICES.md` names the bundle and this ADR.

This is the maintainer's call on a licensing question. It isn't legal advice.

**Consequences.**

- [M5.7][m5-7] does the work, second in the queue, after M0.14. It closes the simulator's widest
  gap with OpenRocket that needs no new physics, and it touches nothing on release 0.2's path.
- The 32 bundled curves stay checked to [ADR-066][adr-066]'s 0.1% of total impulse, as now.
- The idea placed by ADR-287 leaves [the ideas list][ideas]; M5.7 replaces it.
- Motors that ThrustCurve.org lists without a curve (80 in the 2026-09-17 capture, 53 out of
  production) still can't fly. Certification and maker data sheets might fill some; that needs
  research first.
