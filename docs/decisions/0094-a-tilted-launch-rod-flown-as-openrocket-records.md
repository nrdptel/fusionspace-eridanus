# ADR-094: A tilted launch rod flown as OpenRocket records it (2026-09-27)

- **Status:** accepted
- **Summary:** A tilted launch rod flown as OpenRocket records it

**Context.** `cargo xtask ork-flights` flew only a vertical rod, so one private design (`C12`,
three configurations) launched from a tilted rod was listed, not compared (issue #173). M2.2's
bar is 20 designs with the five spreads (ADR-072); with the old M2.2e5 blocked on three issues,
the tilted rod is split off as M2.2e5 and the bar moves unchanged to M2.2e6. `conditions.py`
had already measured OpenRocket 24.12's reading: the rod's angle is from the vertical, and its
direction is a compass bearing (a rod tilted 10 degrees toward 0 lands the example north of the
pad, toward 90 east). hpr's `Rail` takes a bearing clockwise from true north and an angle above
the horizon.

**Decision.**

1. **The rail.** `rod_rail` builds hpr's rail from the recorded rod: the azimuth is OpenRocket's
   direction, the elevation `π/2` less its angle, frictionless and unrolled as the vertical rod
   was. A rod tilted below 0 or from 90 degrees is named (`ROD_NOT_TAKEN`) rather than flown.
   A vertical rod keeps its recorded direction too (90 degrees by default), so the rule has no
   jump as the tilt goes to zero; on a vertical rail it only turns the rocket about its axis on
   the pad, which moved the public flights' apogees by under 0.001%.
2. **Rod probes.** `rod_probes.py` writes `pods-none`'s airframe (ADR-093) with a stored
   simulation, as OpenRocket 24.12 writes one, whose rod tilts 5 degrees toward north, 10 toward
   east and toward south-west, and 20 toward east. `flights.py` flies them from each file's
   stored conditions with the other public designs; they are listed apart as probes, as ADR-093's
   are, and `pods-none` is their vertical control.
3. **Where the rocket goes.** `flights.py` now records the position east and north of the pad
   at the row of the largest altitude, and the angle of attack at the rod-clearance row. The
   report gives hpr's position at apogee from where it started beside OpenRocket's, and hpr's
   margin at OpenRocket's angle of attack beside its margin at none; the private report adds
   that margin's difference (`margin_at_openrocket_alpha_cal`), a difference like its others.
4. **The bar.** Each probe within 5% of OpenRocket's apogee and largest speed (ADR-076); its
   position at apogee within 0.1 degrees of OpenRocket's bearing and 1% of its distance; the
   apogee the rod takes off, against `pods-none`, within 0.5 points of OpenRocket's; and hpr's
   margin at OpenRocket's angle of attack within 0.006 calibres of OpenRocket's. The bounds were
   set after measuring 0.03 degrees, 0.64%, 0.27 points and 0.0048 calibres; a bearing read
   anticlockwise fails them by far, and an angle read from the horizon makes a vertical rod a
   horizontal rail, which hpr refuses. A test also points the rail at the bearings
   `conditions.py` measured OpenRocket's rocket landing on. The bearing's residue is motion
   across the tilt: hpr's fits Earth's rotation on eastward motion in sign and size, and
   OpenRocket's goes the other way with no measured cause (hpr's 10-degree east rod
   peaks 0.17 m above its south-west one, OpenRocket's 0.02 m below; at 20 degrees east hpr's
   rocket is 0.09 m south at apogee, OpenRocket's 0.09 m north), and
   part of the distance's is that OpenRocket's position is its highest 0.05 s row, hpr's its
   apogee event: a longer probe would take more of both bounds.

**Consequences.**

- Met on the probes: the tilt takes 0.65%, 2.60% and 10.09% off OpenRocket's apogee at 5, 10 and
  20 degrees, and 0.63%, 2.52% and 9.82% off hpr's; the bearing at apogee agrees within 0.03
  degrees, and the distance within 0.64%. The south-west rod loses what the east one does.
- OpenRocket's rocket reaches its rod-clearance row at an angle of attack that grows with the
  tilt (0.116 degrees at 10, 0.228 at 20, none from a vertical rod), where hpr's margin is taken
  at none. OpenRocket's margin falls 0.016 calibres at 20 degrees, so the margins part by 0.017;
  hpr's margin at OpenRocket's angle falls 0.012 and leaves 0.005. The residue grows with the
  tilt because hpr's CP moves about a fifth less than OpenRocket's for the same angle. The
  report's margin stays at no angle of attack, as ADR-069 defined it.
- `C12`'s three configurations fly: apogee −0.53% to +0.14%, largest speed within 0.33%, margin
  +0.0125 to +0.0366 calibres, nearly all from the CP; at OpenRocket's angle of attack
  +0.0033 to +0.0074, so most of it is the angle. The two reports now hold 15 designs with the
  five spreads, against M2.2e6's 20.
- Taking the recorded rod on a vertical flight turned several public designs' sideways drift
  with the rocket: hpr's vertical flights of three designs drift 1.7 to 10.5 m by apogee where
  OpenRocket's go straight up, which the new positions show (issue #219).
- Not tested: wind with a tilted rod, a rod longer than 1 m on the probes, and roll on the rod.
