# ADR-060: M2.2 split, and the structure's mass held to OpenRocket's (2026-09-21)

- **Status:** accepted
- **Summary:** M2.2 split, and the structure's mass held to OpenRocket's

**Context.** M2.2 asks for at least 20 designs in a report against OpenRocket, with apogee,
velocity, stability margin, mass and centre of mass. It also carries M1.4's deferred mass and CG
checks. But `cargo xtask ork` flies only 1 of 174 motor configurations: the rest mostly lack a
thrust curve. Staging waits for M1.9, and recovery is read but not flown. A mass or CG error would
also spoil every flight compared later. So M2.2 is more than one pull request, and mass comes first.

**Decision.**

1. **M2.2 splits into five increments**, each with its own *done when* in the roadmap:
   - a: each design's structure against OpenRocket's;
   - b: OpenRocket's mass conventions, with Loft lessons L51 and L87;
   - c: the motors OpenRocket flies, for the configurations held back for want of a curve;
   - d: flights to apogee on the public designs, with L80 and L81;
   - e: the corpus, carrying the parent's *done when* unchanged, with L19 and L82.

   L19 (tube fins' centre of pressure) needs a cited tube-fin model, so it goes with e, where the
   report meets the designs that have tube fins.
2. **The structure, not the launch mass.** `validation/oracles/openrocket/mass.py` asks
   OpenRocket 24.12 for `MassCalculator.calculateStructure` on each design's selected
   configuration, after saving it once to settle its automatic dimensions (ADR-054). hpr's side is
   `Layout::structure`. Both are every stage with no motor, so no thrust curve is needed.
3. **Which inertia is which is measured.** The script first reads a probe tube, 1 m by 50 mm with
   a 2 mm wall at 1,000 kg/m³, and records its mass, centre of mass and inertias worked by hand
   beside OpenRocket's.
   - OpenRocket's `ixx` and rotational inertia are the roll inertia; `iyy`, `izz` and the
     longitudinal inertia are the pitch inertia; all are about the centre of mass. They match the
     hand values to 15 digits.
   - hpr's layout of the same probe matches too, which pins hpr's side: the centre of mass at
     `z = −0.5`, and roll about `z`.
   - Pitch is compared as the mean of the two transverse inertias, so neither program's turn about
     the axis matters.
4. **Thresholds set before measuring:** 1% of OpenRocket's mass, and 1% of the rocket's length for
   the centre of mass. A design outside either needs a written cause. Inertias are reported, not
   held to a threshold.
5. **A difference is traced to parts.** The script also records OpenRocket's own per-part
   breakdown (`getCMAnalysis`). `cargo xtask ork` pairs each part with hpr's by id, an assembly
   under one override as a whole. In an older file that writes no ids, parts are grouped by name,
   since OpenRocket then makes up random ids. For every design outside a threshold, the survey
   prints its causes and the parts that differ most. Only a file from a public source is named;
   any other appears as the start of its file's hash.

**Consequences.** On 2026-09-21, over 74 design files (54 distinct by content):
- **Within 1%:** 57 in mass and 58 in centre of mass. The medians are 0.020% and 0.013% of length.
- **Outside a threshold:** 17 files (12 by content, 11 designs). The parts behind each fall into
  five causes, and hpr warns of every one when it reads the file; four have two. `cargo xtask ork`
  works the causes out from those warnings, counts them by content, and fails on a file outside
  with none. By content:
  - a shoulder written with no wall, read as solid, which OpenRocket gives no mass (6);
  - a motor cluster read as one tube (2);
  - fin fillets left out (2);
  - a part with no material, which OpenRocket gives 680 kg/m³ (1);
  - parts hpr keeps unread, such as pods, parallel stages and tube fins (5).
- **The arithmetic for the shoulder cause.** In the jar's *Two stage high power rocket*, the
  difference, 0.3666 kg, is exactly the solid cylinder of polypropylene the shoulder would be. In
  *Airstart timing* it is 5.853 kg of fibreglass. `hpr_io::ork` has left this choice "for the M2.2
  oracle to settle" since M3.1b2: OpenRocket gives such a shoulder no mass. M2.2b decides whether
  hpr follows.
- **Two more conventions, outside no threshold.** hpr scales a part's inertia with an overridden
  mass and OpenRocket does not (Loft's `stage-weighed.ork`: pitch +100.9%, the override 2.009 times
  its parts); and hpr's airfoil fin is lighter (the jar's *Simulation scripting* CONTROL fins, 19.4%).
- **Roll inertia is not explained:** median 2.1% apart, 1.2% to 3.8% even on Loft's public
  designs, whose mass, CG and pitch inertia agree within 0.1% and every part within 0.3 g. It goes
  to M2.2b, with the six
  conventions.
- **CI.** The committed record for Loft's seven public designs is held by `cargo test -p xtask
  ork_mass`.
- **Stale records fail.** The survey fails if the record is out of date (a changed file, or a
  laid-out design missing from it), or if the probe's mapping stops holding.

---
