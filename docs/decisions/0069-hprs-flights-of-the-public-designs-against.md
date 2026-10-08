# ADR-069: hpr's flights of the public designs against OpenRocket's (2026-09-25)

- **Status:** accepted
- **Summary:** hpr's flights of the public designs against OpenRocket's

**Context.** M2.2d2 carries M2.2d's *done when*: the public designs that fly are in a report
against OpenRocket (apogee, largest speed, stability margin), by the definitions ADR-068 measured.
hpr flies 21 of the record's 57 powered configurations, in five of OpenRocket's examples; the
motors of most come from OpenRocket's own database by digest (ADR-067), which lives under the
gitignored `corpus-out/`, and the examples from the pinned jar. hpr flies no recovery device read
from a `.ork` (ADR-057 reads them), and has no margin output (M1.10).

**Decision.**

1. **`cargo xtask ork-flights` flies them and writes the report.** Each configuration flies in its
   recorded conditions: a vertical rod of the recorded length, the site's latitude, longitude and
   height, hpr's standard atmosphere and calm air. `validation/reports/openrocket-flights.{md,json}`
   are committed. `--check` flies them again and compares, where the jar and the database record
   are fetched. CI does not fly them: its tests hold every OpenRocket figure in the report to the
   record, every outcome to `compare`, the summary and the page to the flights, and every powered
   configuration of the record to a flight or a named reason. After a change to the physics,
   `--check` must be run by hand.
2. **Each metric is taken by ADR-068's definition.** Apogee: the largest height of hpr's centre of
   mass, at its apogee event, counted from its height at the start, since OpenRocket's altitude is
   0 at launch. Largest speed: the centre of mass's, sampled at each step's end and three points
   inside it from the dense output, up to apogee. hpr's unbraked fall is left out, since
   OpenRocket's rockets come down under parachutes. Margin: at the recorded rod-clearance step's
   time and Mach number, hpr's centre of pressure with the air along the axis, less its centre of
   mass at that time, over its reference diameter. hpr's mass and centre of mass at that time are
   its own, reported beside OpenRocket's.
3. **Design checks are recorded, not enforced.** OpenRocket flies a design whatever hpr's checks
   find, so hpr flies it too and the report lists the errors its checks found: on two examples, an
   inner part 0.46 mm wider than the room for it, and a 29 mm motor in a 28.956 mm mount.
4. **No recovery is flown.** A reference whose parachute opened before its apogee (6 of the 21, plus
   2 with a part set to no drag) is marked with how long before the apogee of the same flight with
   nothing deployed, which the record holds, and summarised apart. Flying a `.ork`'s recovery is
   later work.
5. **A difference is summarised under its named cause, and each apogee off by more than 5% needs
   one** (M2.2's parent bar, applied here). Two causes are named: a reference parachute open before
   apogee, which puts OpenRocket's apogee lower; and a part whose drag OpenRocket is told is zero,
   which hpr reads (L63) but cannot apply (#165). For the second, the report flies the same
   configuration again with those parts removed. That is a probe, not the override, since it takes
   their mass, lift and shape too. A part set to any other drag coefficient stops the command.

**Consequences.** Measured on 2026-09-25: the margin at rod clearance is within 0.016 calibres on
all 21 (−0.0151 to +0.0037). With no named cause, hpr's apogee is low on all 12, by 0.06% to
4.34% (median −1.46%), and the largest speed −0.69% to +0.85% (18). The *Dual parachute
deployment* example reads 0.63% to 4.34% low on all six motors, untraced; its J570W flight, the
only one past Mach 1 (1.147), is the largest. Five apogees are more than 5% off, each with a named
cause:

- two C6-3 flights whose parachute opened 2.59 s and 2.99 s early (+12.19%, +13.80%). The same
  rockets' C6-7 flights, the same climb with no early parachute, read −0.16% and −1.07%;
- *Base drag hack (short-wide)*, whose aft transition is set to no drag: −15.47% to −19.13%. With
  the transition removed, its C11-5 flight reads −0.29% in apogee and +0.04% in speed. Its D12-3
  and E12-4 flights also have early parachutes, and the probe overshoots their speeds (+1.99%,
  +5.23%), so how the two causes split there is not measured.

M2.2d is met; M2.2e carries the corpus.
