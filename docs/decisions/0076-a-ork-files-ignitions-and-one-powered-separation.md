# ADR-076: A `.ork` file's ignitions and one powered separation flown against OpenRocket (2026-09-25)

- **Status:** accepted; its refusal of a motor that never lights, and its reading of one private record, superseded by ADR-100
- **Summary:** A `.ork` file's ignitions and one powered separation flown against OpenRocket

**Context.** M1.9c carries M1.9's first bullet: a `.ork` two-stage design and a cluster design each
within the per-case tolerance of OpenRocket's flight. hpr-io read every motor's ignition and every
stage's separation (M3.1c2, ADR-056) but flew only what lights at launch (ADR-055), left clustered
configurations out (ADR-075 §7) and flew no rocket of more than one stage. No tolerance for a
flight of a `.ork` against OpenRocket's had been written down: ADR-069 holds each apogee more than
5% off to a named cause, and holds nothing to a bound.

**Decision.**

1. **The tolerance, set before any staged or clustered flight was measured.** A design is within
   when every configuration of it that hpr flies has its apogee within 5% of OpenRocket's and its
   largest speed within 5% of OpenRocket's. Where OpenRocket's parachute opened before its apogee
   (ADR-069 §4), the apogee is held to OpenRocket's flight of the same configuration with nothing
   deployed, which the record holds (ADR-073). 5% is M2.2's bar for an apogee with no named cause;
   the largest speed is held to the same.

2. **Ignition, as the file says it.** `launch` plus `d`, and `automatic` plus `d` in the bottom
   stage, light at `t = d` (OpenRocket's *Airstart timing* example writes its air starts as
   `automatic` plus 1, 2, 4 and 6 s, and its record lights them then). `automatic` in a stage above
   is `ejectioncharge`: the burnout of the stage below's motor plus its ejection delay and `d`.
   `burnout` is that burnout plus `d`. Each becomes an `hpr_design::Ignition` (ADR-074 §2). The
   stage below is the next stage aft, and its motors must sit in one mount, whose first burnout is
   then the stage's. A configuration is left out (`IgnitionNotFlown`) when a motor is set `never`,
   names a word hpr does not know, waits on a plugged motor's charge, or on a stage below with no
   motor or with motors in more than one mount. hpr has no motor that is never lit on purpose, and
   choosing among mounts would be a guess. One private configuration sets its bottom stage's motor to `burnout`,
   with no stage below; OpenRocket's record lights it at launch. One flight is not a rule, so hpr
   leaves it out rather than read the word as `launch`.
3. **One powered separation.** hpr's flight takes one separation, so a configuration flies at most
   one. Its time comes from the file: `launch` plus `d` is a time; `ignition` and `upperignition`
   are the ignition of the stage's own motor or of the stage above's, plus `d`; `burnout` and
   `ejection` are the stage's own motor's burnout plus `d`, and plus its ejection delay for
   `ejection`. `never` flies the stack whole. A missing delay is 0, as a missing ignition delay is;
   a negative or non-finite one on a separation hpr flies is refused. The separation is powered when, at that time, a motor
   ahead of it is still burning or yet to light: the test hpr-sim makes when it fires (ADR-074 §3),
   made here from the times rather than from where the motors sit, so a file whose sustainer has
   already burnt out by the split is not called powered. A motor behind the split still burning
   there is refused too, as hpr-sim would. The reader gives the split as
   `hpr_io::ork::Staging` (the boundary, a `Time` or `Burnout` trigger, and the resolved time), and
   `hpr::ork::separation` turns it into hpr-sim's `Separation`: the rocket the configuration builds
   does not carry it, so a program that flies a staged `.ork` configuration must pass it on, as
   the example `ork_two_stage` does. Left out (`SeparationNotFlown`): two stages or more that
   separate (#183), a stage that states no event, `altitudeascending` (hpr has no such trigger),
   and anything else.
4. **An unpowered separation is the descent's.** `apogee` and `altitudedescending` can't come
   before apogee: the climb is the whole stack's in both programs, and the separation is part of
   the descent, which hpr's `.ork` flights do not fly (ADR-069 §4), so the configuration flies
   whole. That assumes every motor is spent by apogee; `cargo xtask ork-flights` refuses to report
   a flight where one is not. Any other separation with nothing ahead of it left to burn could
   come before apogee,
   where hpr would fly both parts as point masses without their airframes' drag (ADR-014), so it
   is left out (#184).
5. **Clusters fly.** `NotFlown::Cluster` is dropped: a configuration with a motor in a clustered
   tube flies with a motor in every tube (ADR-075 §2). OpenRocket still weighs the tubes stacked on
   the cluster's axis (ADR-075 §5), which the report does not hide.
6. **The report's flight.** `cargo xtask ork-flights` gives a staged configuration its separation
   and the two devices hpr's descent needs on separated bodies (ADR-074 §3): the booster tumbling
   from the split, the sustainer from its apogee. Neither acts on the climb, which is what is
   compared. OpenRocket's record holds its branch 0, the one that keeps the nose, so both
   programs' apogee and largest speed are the sustainer's. A staged flight records its separation
   time beside OpenRocket's, and a clustered flight its count of clustered motors. The mass and
   centre of mass at rod clearance now count a motor as loaded until it lights.

**Consequences.** Measured on 2026-09-25, after §1 was committed:

- *Two stage high power rocket*, both configurations: `[H148R-0; H148R-0]` apogee −1.79%, largest
  speed −0.54%; `[I59WN-P; I357T-14]` −0.13% and −0.04%. Both separate when OpenRocket's do, at
  1.535 s and 1.515 s.
- *Clustered motors* (four motors in a `4-ring`), all five: apogee −0.35% for the A8-3; −0.79%,
  −0.72% and −0.72% for the B4-4, C6-3 and C6-5 against OpenRocket's flights with nothing deployed,
  whose parachutes opened 0.37 s, 2.59 s and 0.59 s early (+9.43% for the C6-3 against its
  record); −0.72% for the C6-7. Largest speed +0.20% to +0.92%.
- *Airstart timing* (a `3-ring` of I211W beside a K550W in its own mount, the ring lit at launch or 1, 2, 4 or 6 s
  after): apogee +0.48% to +1.03%, largest speed +0.46% to +0.81%.
- So M1.9's first bullet is met, and a test (`a_two_stage_and_a_cluster_design_are_within_5_percent_of_openrocket`)
  holds the committed report to §1.
- OpenRocket's mass on `[H148R-0; H148R-0]` falls 1.9992 times as fast as hpr's before the
  sustainer lights: 0.0823 kg against 0.0412 kg from launch to rod clearance. The launch masses
  agree to 3e-6 kg, and on the other configuration, with two different motors, the rod-clearance
  masses agree to 1.05e-6 of the mass. OpenRocket's recorded mass falls as if both H148Rs lost
  propellant from launch. Its effect on the apogee
  is not sized (#185).
- The public report flies 33 configurations (was 21), 6 apogees more than 5% off, each with a named
  cause (was 5), 5 of them within 5% once it is removed. The private library's flies 18 (was 17),
  in 5 private designs (was 4), and the two reports make 13 designs toward M2.2's 20 (was 9).
- Not flown: the three-stage example (#183) and two payload designs whose separation has no motor
  ahead of it (#184).
- The private flight newly flown, C08/1 (two stages), reads +0.1108 calibres in margin because its
  centre of mass at rod clearance is 0.1102 calibres forward of OpenRocket's, its mass within
  about 0.001%. That predates this record: hpr's structure alone is forward by less than the mass
  survey's 1% of length, and a stage mass override several times its parts' weight magnifies it.
  Two parts differ: an airfoil fin set (ADR-062 §4's kept departure) and packed parachutes
  whose automatic radius OpenRocket may resolve by stretching the packed length (#186).
- The 5% on the largest speed is loose: every flight above is within 0.93% of it, so a speed error
  of several per cent would still pass. It is kept because it was set before measuring (§1); the
  report's numbers are what to read. CI does not fly OpenRocket, so it checks the committed report
  against §1, and `cargo xtask ork-flights --check` re-flies hpr's side on a machine with the jar.
- At the split, hpr's reference point, the centre of mass, jumps forward from the stack's to the
  sustainer's, which on a vertical flight raises its height by that distance. Whether
  OpenRocket's branch 0 altitude does the same is not measured (#187). The jump is less than the
  stack's centre of mass from the nose at the split, which is forward of where it is at rod
  clearance, since only the booster burns between: 1.320 m then on *Two stage high power rocket*,
  under 0.2% of the H148R apogee (666 m) and under 0.1% of the other (1382 m), about the size of
  that flight's whole gap (−0.13%).
