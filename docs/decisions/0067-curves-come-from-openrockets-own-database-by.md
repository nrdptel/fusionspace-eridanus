# ADR-067: Curves come from OpenRocket's own database by digest, each held to its impulse (2026-09-25)

- **Status:** accepted
- **Summary:** Curves come from OpenRocket's own database by digest, each held to its impulse

**Context.** [M2.2c2][roadmap] asks that the curves the reference library embeds are held to the
0.1% of total impulse that [ADR-066](0066-every-curve-hpr-flies-is-integrated-as.md)
set for the bundled ones, and that every configuration held back only for want of a curve flies or
is named with its reason, both counted by `cargo xtask ork`. The first half is small: the library's
75 files embed 3 distinct `.rse` curves, which 4 motor references use. The second half is supply.
Of 202 motor references, 196 had no curve: 3 hybrids and 193 neither embedded nor in hpr's
32-curve bundled catalog. That left 162 of 170 configurations held back for want of a curve, and 2
flying. A `.ork` names its curve by OpenRocket's digest, a hash of the curve's data that
"uniquely identifies the functional characteristics" of the curve (OpenRocket's GitHub wiki, file
format 1.2), and OpenRocket finds most curves in the motor database it ships inside its jar. There
were two places to get curves: ThrustCurve.org through `hpr-net` ([M5.1][roadmap], not built, and
matched by name), or OpenRocket's own database, which is where the reference program gets the
curve it flies.

**Decision.**

1. **An oracle records OpenRocket's database, and what OpenRocket flies.**
   `validation/oracles/openrocket/motor_database.py` does three things:
   - It loads the database with `MotorDatabaseLoader`, the public class OpenRocket 24.12 loads it
     with at startup. It stops if OpenRocket's user motor directories hold any file, so the record
     is the jar's alone.
   - For each of the 1,452 motors it records the digest, the identity, the envelope, the masses
     and the centre of mass. It records the samples, and OpenRocket's total impulse, average and
     peak thrust and burn times.
   - It hands every embedded `thrustcurves/*.rse` to OpenRocket's loader, keyed by the entry's
     SHA-256, as `motors.py` does for bundled files.
   - It opens every design with the database bound, as the program does, and records the digests
     of the motors OpenRocket places in each configuration, by configuration id.

   It walks the designs `cargo xtask ork` surveys: those under `refs/` but `refs/scratch/`, and with
   `--jar` the jar's example designs. It finds and opens them as `mass.py` does, retrying a design
   OpenRocket refuses for a leading comment without it. The record goes to the
   gitignored `corpus-out/openrocket-motors.json` and is never committed. The database comes from
   ThrustCurve.org through OpenRocket, with unstated terms, and the embedded curves belong to the
   designs' owners. Only counts are published. OpenRocket is run, never read.
2. **`hpr-io` takes supplied curves, by digest only.**
   - `SuppliedCurves` maps a digest to a solid motor and its `CaseSize`. It refuses a case that is
     not finite and positive.
   - `ork::design_with(file, &supplied)` looks in this order: the embedded curve, then a supplied
     curve for the motor's digest (`Curve::Supplied`), then the bundled catalog.
   - A supplied curve is never matched by name: the database holds 286 manufacturer-and-designation
     pairs under more than one digest.
   - A `<motor>` typed `hybrid` is refused before any lookup. The caller supplies solid motors
     only, since a `SolidMotor` carries no type.
   - `ork::design` passes nothing to `design_with`, so its output and its tests are unchanged.
   - A reason says where hpr looked: the supplied curves lack the digest, or no digest is recorded.
   - A supplied case more than a millimetre from the design's is warned about, as a catalog one is.
   - A supplied curve comes ahead of the bundled catalog. Where both hold a motor, OpenRocket's
     masses are the ones flown, and they need not match the ThrustCurve.org metadata the catalog
     carries. One library motor moved from the catalog to the database this way.
     Matching OpenRocket is the point of the survey.
3. **The survey screens the database before supplying it.**
   - Hybrids are never supplied: 164 motors under 161 digests.
   - 6 digests are held by two motors that differ in samples, case or masses, so none of them is
     supplied.
   - One motor is refused, Cesaroni 836J210-16A. Its propellant mass gives an exhaust velocity of
     10.1 km/s, outside the 200 to 5,000 m/s that `SolidMotor::from_envelope` accepts.
   - 54 repeats of a digest with the same samples and envelope (to 1e-9) collapse to one.
   - That leaves 1,221 digests supplied, from the 1,288 solid motors less the 54 repeats, the 12
     motors sharing the 6 digests and the refused one.
   - Other masses are taken as OpenRocket holds them, including some that give physically
     unlikely exhaust velocities. OpenRocket flies the same masses.
4. **Every curve is held to OpenRocket's impulse, or the survey fails.** hpr's integration is
   compared with OpenRocket's `getTotalImpulseEstimate`. All are within 0.1%, and **identical to the
   last bit**. The two sets are different kinds of check:
   - The 3 distinct embedded curves are real checks, as the 32 bundled ones were. hpr parses the
     entry's bytes, and OpenRocket parses the same file with its own loader.
   - The 1,288 solid database curves are checked against the samples OpenRocket has already
     parsed, so their agreement proves the hand-off, not two readings.
   - A curve outside 0.1% is not supplied.
   - The survey fails on any of these:
     - a curve outside 0.1%;
     - an embedded curve missing from the record, or one OpenRocket refused;
     - a record missing a list, or with no solid motor to check;
     - a record written by another version of any of the four oracle scripts (with `motors.py`,
       `automatic_radius.py` and `geometry.py`), by another OpenRocket release, or with another
       jar than the pinned one;
     - a flown configuration whose supplied curves OpenRocket does not place (item 5).
5. **OpenRocket places the curves hpr is supplied.** For every configuration hpr flies with a
   supplied curve, the survey finds the same design (by SHA-256) and configuration id in the
   record, and requires OpenRocket's placed digests to include every supplied one. They do in all
   67 such configurations (67 motors), and OpenRocket opens every design they are in. A mismatch,
   a configuration id OpenRocket lacks, a design missing from the record or one the oracle failed
   on fails the survey; a configuration in a design OpenRocket does not open is counted apart and
   printed. Configuration ids are compared in lowercase, as OpenRocket keys them.
6. **Each configuration the bundled catalog left without a curve is followed.** The survey reads
   every design twice, with and without the supply, and counts what became of the 162:
   - **66 fly.** With the 2 that already did, 68 of 170 configurations fly, in 16 designs, and
     all 68 assemble.
   - **72 are held back for another reason,** by the first reason the reader finds:
     - 29 for an airframe not read exactly as written
     - 20 for a motor in a cluster
     - 12 for more than one stage
     - 11 for a motor igniting in flight
   - **24 still have no curve,** each for a named reason:
     - 20 record no digest, and the bundled catalog has no such motor
     - 3 are hybrids
     - 1 has a digest the database lacks

   The stored-run reproduction screen ([ADR-065](0065-stored-results-are-references-only-when-current.md))
   moves from 1 to 40 reproducible of the 91 eligible runs. One supplied case differs from its
   design's by 7 mm in length, and the survey warns about it.

**Consequences.** M2.2c is met: every configuration held back only for want of a curve flies or is
named, and every curve hpr flies from the library is within 0.1% of OpenRocket's impulse. Two
things are left:
- **Where a motor's weight sits is not OpenRocket's.** hpr builds every motor from its envelope,
  whether bundled, embedded or supplied: centre of mass at mid-case, the dry case a thin tube.
  OpenRocket gives each database motor a fixed centre of mass of its own, and treats the motor as
  a solid cylinder for inertia. For 163 of the 1,221 supplied motors, OpenRocket's centre of mass
  is more than 1 mm from mid-case. Among the 31 distinct supplied motors in configurations that
  fly, 3 are, by up to 5.0 mm. The survey prints both counts. [M2.2d][roadmap] flies these designs against OpenRocket
  and will meet this in the stability margin. It is the milestone to decide whether hpr takes
  OpenRocket's centre of mass, or records the difference as a departure.
- **Motors that record no digest** (older files) stay unflown. A name rule could fly them, but it
  would be hpr's choice among curves.

`Curve` is `#[non_exhaustive]`, so the new variant breaks no caller.

[roadmap]: https://github.com/nrdptel/hpr-sim/blob/main/docs/ROADMAP.md

---
