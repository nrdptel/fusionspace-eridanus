# ADR-161: A `.ork` motor fetched from ThrustCurve takes the file holding OpenRocket's curve for its digest (2026-10-04)

- **Status:** accepted
- **Summary:** amends ADR-154 §2 and §4: a fetch may name a ThrustCurve data file to take ahead of the rank's order (`Wanted::prefer`, `hpr motors fetch --file`). `hpr sim` names one for a `.ork` motor whose digest a committed table holds: the file whose curve is the one OpenRocket flies. `cargo xtask ork-curves` writes the table for the motors of OpenRocket's examples by comparing curves; it holds ids alone. With it, OpenRocket's three-stage example's first configuration reads +0.31% of OpenRocket's largest speed in `hpr sim`, not +5.7%, and every configuration is within 2.48% of OpenRocket's apogee

**Context.** [ADR-154](0154-motors-fetched-from-thrustcurve-by-name.md) fetches a `.ork` motor
with no curve by its manufacturer and designation, and takes the motor's files in order of who
measured them. A motor ThrustCurve holds several files of can then fly a different curve from the
one OpenRocket flies for the digest the `.ork` records. M4.5g2's *done when* asks that each
configuration of OpenRocket's *Three stage low power rocket* fly in `hpr sim` within 5% of
OpenRocket's apogee and largest speed ([ADR-160](0160-several-powered-separations.md)). Its
first configuration's A8 is such a motor:

- ThrustCurve's first certification file of the Estes A8 burns 0.536 s.
- OpenRocket's database holds two A8 curves, and the example records the digest of the one that
  burns 0.73 s.
- On the fetched file, `hpr sim` read 82.8 m/s against OpenRocket's 78.4 (+5.7%).

The `.ork` records only the digest, and OpenRocket's way of computing one is not published, so hpr
can't compute the digest of a fetched file and compare.

**Decision.**

1. **A preferred file.** `hpr_net::on_demand::Wanted` gains `prefer`, a ThrustCurve data file
   id. `find_with` takes that file first when it reads, and the rest keep ADR-154's order behind
   it. If the RASP answer lacks the file, the RockSim answer is read too before any file is
   taken. Offline, with no copy of that answer but RASP files to take, the preference is given
   up and the RASP files are taken as before: a cache filled before this decision still flies.
   Without a preferred file, nothing changes: the same requests, in the same order, and the same
   file.
2. **The table.** `crates/hpr-cli/data/openrocket-curves.json` maps a digest to a file id. `cargo
   xtask ork-curves` writes it, for each motor a configuration of OpenRocket 24.12's examples
   places:
   - It reads OpenRocket's curve for the digest from OpenRocket's own database
     ([ADR-067](0067-curves-come-from-openrockets-own-database-by.md)).
   - It finds the motor on ThrustCurve by manufacturer and designation, as `hpr sim` does, and
     reads every file of it.
   - For each file it takes the largest difference in thrust at any sample time of either curve,
     over OpenRocket's peak thrust. A file matches under 1%, and the least difference is written.
   - Of files alike in thrust, the one whose masses are nearest OpenRocket's is written: the
     larger difference of the propellant's and the dry motor's mass, as hpr builds the motor from
     the file, over OpenRocket's loaded mass. Both differences are rounded to a millionth, so
     noise decides nothing; files alike in both keep the order `hpr sim` takes them in.
3. **What the table holds.** Ids, the motor's maker and designation as the example writes them,
   and the differences. It holds no curve: ThrustCurve states no licence for its records, and
   OpenRocket's database states no terms
   ([ADR-145](0145-recorded-answers-and-their-licences-stand-in.md)). It names only motors of
   OpenRocket's public examples, so it says nothing of a private design.
4. **`hpr sim`.** A `.ork` motor fetched by ADR-154 §4 names the table's file for its digest, if
   the table has one. The note on the motor says which kind of file flew:
   - the table's file: "the file whose curve is the one OpenRocket flies for the digest";
   - any other: ADR-154's "may not be the curve OpenRocket flies".
5. **`hpr motors fetch --file ID`** takes a file first, as the flight does. So the command an
   offline refusal names fills the cache the flight reads: with a preferred file, the refusal's
   command carries `--file`. When the file asked for is not the one taken, the output says so.

**Result.** On 2026-10-04, `cargo xtask ork-curves`:

- It compared 29 motors.
- 28 matched a file with a difference of 0 to the curves' stored digits: 27 RASP files and one
  RockSim file, a B6 whose RASP file differs by 2.7% of its peak. Each match's masses are
  OpenRocket's to a millionth of the loaded mass.
- On four motors, files alike in thrust differed in mass, and the mass picked the file written.
  Each was already the first in `hpr sim`'s order.
- The matched motors' other files number 36. Twenty-one hold the same curve, within 4 parts in
  a million of the peak. The other 15 differ by 2.7% of the peak or more, 12 of them by 34% or
  more. The 1% bound falls between, and the command prints these counts and the four above.
- The one left out, a HyperTEK hybrid, has no solid curve in OpenRocket's database.

`hpr sim` on OpenRocket's three-stage example, with the curves fetched, against OpenRocket's
flight in `validation/reports/openrocket-flights.json`. The example is `datafiles/examples/` in
the pinned jar, unzipped outside `refs/`; the command is `hpr sim three.ork --config N --json`
(`--offline` once fetched), its `summary.apogee.gain_m` and `summary.max_speed_m_s`. `gain_m` is
the height gained from where the rocket stood, as OpenRocket counts altitude. `hpr sim` flies the
file's parachutes, which open before apogee on the first two, so two OpenRocket apogees are
shown:

- its own record, which opens them too: the like-for-like one;
- its flight with nothing deployed, the one ADR-076 holds the report's flights to, since those
  hold the parachutes.

| motors | apogee, hpr (m) | OpenRocket's record (m) | OpenRocket, nothing deployed (m) | largest speed, hpr / OpenRocket (m/s) |
|---|---|---|---|---|
| A8-5, B6-0, B6-0 | 273.6 | 276.7 (−1.15%) | 277.1 (−1.28%) | 78.61 / 78.37 (+0.31%) |
| C6-5, B6-0, B6-0 | 492.8 | 495.7 (−0.58%) | 505.4 (−2.48%) | 121.55 / 119.69 (+1.55%) |
| C6-7, C6-0, C6-0 | 680.1 | 689.5 (−1.37%) | (the same flight) | 132.02 / 129.85 (+1.67%) |

The A8 file is the table's. The B6 and C6 files were already the first in ADR-154's order. These
figures come from a run with the network; no CI test can repeat it, as no ThrustCurve answer for
these motors is committed (ADR-145).

**Consequences.**

- Each motor of OpenRocket's examples that `hpr sim` fetches flies OpenRocket's own curve, where
  ThrustCurve holds it.
- A digest outside the table still flies ADR-154's file, and says it may not be OpenRocket's
  curve. That includes every motor of a design that isn't one of OpenRocket's examples.
- The table goes stale if ThrustCurve's files change. `cargo xtask ork-curves` rewrites it, and
  needs the network, the pinned jar and the motor-database record.
- The CI tests stay off the network. Two stand-in RockSim files were added to the invented
  motor's answers ([ADR-145](0145-recorded-answers-and-their-licences-stand-in.md)), one under
  the id the table gives the A8, so a test flies the table's file. The CLI test fails when the
  table isn't consulted, and when the note doesn't say the file is OpenRocket's.
