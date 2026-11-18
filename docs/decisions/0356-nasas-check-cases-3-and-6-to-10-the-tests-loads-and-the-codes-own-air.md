# ADR-356: NASA's check cases 3 and 6 to 10: the test's loads, and the codes' own air (2026-11-17)

- **Status:** accepted; the decision record that [M1.14b4][m1-14b4]'s *done when* asks for ([ADR-351][adr-351], decision 2); amends [ADR-353][adr-353]'s spread rule for a case of three codes ([#1524][i-1524]); no *done when* changed
- **Summary:** The free body flies NASA/TM-2015-218675's check cases 3 and 6 to 10 under loads the test applies: case 3's damping moments, and the sphere's constant drag along its velocity through the air. Case 3's inertial rates differ from the kept codes past their spread, and the Earth's rate left out of the rotational equations accounts for all but 0.003176, 0.001123 and 0.0007793 deg/s, inside it. The sphere's positions in cases 6 to 8 differ from SIM 1 past the spread; the gravity past J2 with each code's own air density brings them inside it, to 77.57 to 77.68 mm (3.05 to 3.06 in), without explaining SIM 1's. Cases 9 and 10 are within the spread.

[m1-14b3]: ../decisions-and-roadmap.md#m1-14b3
[m1-14b4]: ../decisions-and-roadmap.md#m1-14b4
[adr-351]: 0351-m1-14b-in-eight-the-basic-finners-normal-force-first.md
[adr-353]: 0353-nasas-check-cases-1-and-2-both-differences-past-the-codes-spread-traced.md
[adr-011]: 0011-rigid-body-flight-equations-of-motion.md
[nesc-note]: ../research/nesc-check-cases.md
[report]: ../../validation/reports/check-cases.md
[report-json]: ../../validation/reports/check-cases.json
[flight]: ../physics/flight.md#a-free-body-against-nasas-check-cases
[i-759]: https://github.com/nrdptel/fusionspace-eridanus/issues/759
[i-1521]: https://github.com/nrdptel/fusionspace-eridanus/issues/1521
[i-1524]: https://github.com/nrdptel/fusionspace-eridanus/issues/1524

**Context.** [M1.14b3][m1-14b3] flew NASA's check cases 1 and 2 with a free body: a mass and an
inertia on HPR Sim's gravity, Coriolis force, rigid-body equations and integrator
([ADR-353][adr-353]). M1.14b4 flies the rest that fit ([the note][nesc-note]). Case 3 is case 2's
tumbling brick with constant roll, pitch and yaw damping. Cases 6 to 10 fly case 1's sphere with
a constant drag coefficient: dropped from 30,000 ft in still air (6), in a steady 20 ft/s wind
from the west (7) and in a wind from the west of `(0.003 h − 20)` ft/s at `h` ft above sea level
(8), and shot from sea level at 1,000 ft/s up and 1,000 ft/s east (9) or north (10). The
*done when* asks for each case's largest difference from each code beside the codes' spread, by
M1.14b3's rules, and a trace of any difference from a kept code past that spread. Every measured
figure below is in [the committed report][report] ([as data][report-json]), which
`cargo xtask check-cases --check` holds in the gate's `refs` step, unless it is marked as a
reading made for this record.

**Decision.**

1. **The loads are the test's; the air is the flight's.** `hpr_sim::FreeBody` gains loads its
   caller supplies (`FreeBodyLoads`): a force through the center of mass and a moment about it,
   in body axes. The caller forms them from the state, the air and the wind at the center of
   mass's height above sea level, and the velocity through the air, which the free body reads
   from its environment as a rocket's flight reads them. A test holds that reading and that the
   loads enter Newton's and Euler's equations as `qF/m` and `I⁻¹M`; another flies a damped spin
   against its exponential. The free body still has no aerodynamics of its own.

2. **Case 3's damping law, as the test states it.** The report gives `C_lp = C_mq = C_nr = −1`,
   a reference area `S`, a span `b` and a chord `c̄`, but no formula. The test takes the usual
   one: about each axis, `q̄ S l C (ω l / 2V)`, with `q̄ = ½ρV²`, `V` the speed through the air,
   `ω` the inertial body rate about that axis and `l` its length: the span for roll and yaw, the
   chord for pitch. `ω l / 2V` is the rate made dimensionless. It is written `¼ ρ V S l² C ω`,
   which needs no division and is zero at rest.
   - **Why the inertial rate.** The codes' files hold their aerodynamic moments, and the
     report sets each code's against this law on its own printed density, speed and rates over
     the first 8 s. The codes split. SIM 4's follow the law in the inertial rate to 3.0e-5 of
     their size in roll and yaw and 8.1e-11 in pitch, and SIM 1's to 1.3e-4 and 3.5e-6 in roll
     and yaw, but 1.0e-2 in pitch. SIM 5's follow it in the rate relative to the Earth to 1.1e-15, and SIM 6's to
     1.2e-10. SIM 2's follow neither: they are 0.1103 to 0.2264 of their size off the law in the
     inertial rate and 0.1103 to 0.235 off it in the rate relative to the Earth.
     The rate relative to the air, which turns with the Earth, is the physical one. The test
     takes the inertial rate, as SIM 4 does and SIM 1 nearly does, so that the named difference below is the
     Earth's rate alone against them; against SIM 5 and SIM 6 the choice is part of what is
     left, and the report gives it.
   - **A test** (`the_damping_is_on_the_inertial_rate_with_the_span_and_the_chord`) holds the
     moment on each axis, and pitch's at `(c̄/b)²` times roll's per unit rate.

3. **The sphere's drag acts along its velocity through the air.** `F = −q̄ S C_D v/|v|`, with
   `C_D = 0.1` on 0.1963495 ft², whichever way the body points. The sphere falls along its body
   z axis, at an angle of attack of 90°, where a drag along the body's axis, as a rocket's is,
   would give no force. A test (`the_drag_acts_along_the_velocity_through_the_air_at_90_degrees`)
   holds the full `½ρV²SC_D`, straight up, at that angle, and in case 7's wind its push east.
   Case 8's wind is HPR Sim's layered wind, linear in its components between a calm level where
   the table's formula crosses zero and levels above the start, so it is the table's to rounding
   over the heights flown; a test holds it.

4. **A position is compared as a distance.** HPR Sim's position and each code's, in Earth-fixed
   axes; the distance between them is the difference, and the spread is the largest distance
   between two codes. Two codes, SIM 2 and SIM 4, write no Earth-fixed position, so every code's
   is read the same way: its latitude, longitude and altitude, turned into Earth-fixed axes on
   WGS 84.

5. **Case 3 is the Earth's rate again.** Five codes; SIM 1 is left out of each rate, as it cuts
   the spread from 0.03192, 0.07435 and 0.01674 deg/s to 0.003609, 0.003792 and 0.003503 deg/s.
   HPR Sim's inertial rates differ from the kept codes by up to 0.004873, 0.005633 and 0.005102
   deg/s, past the spread.
   - **The named difference** is case 2's ([ADR-353][adr-353], decision 4): HPR Sim's rotational
     equations leave the Earth's rate out ([ADR-011][adr-011]). The report's tool integrates
     both sets of equations from the report's start, now with the same damping law in the
     inertial rate on both, along the codes' own fall with no drag through the 1976
     atmosphere, by RK4 at 1 ms. It uses no flight of HPR Sim's.
   - **What it leaves** of each kept code's largest difference: at most 0.003176, 0.001123 and
     0.0007793 deg/s, inside the spread. The stricter reading, at every sample, leaves at most
     0.003287, 0.003791 and 0.002631 deg/s: inside in roll and yaw, and in pitch against a
     spread of 0.003792 deg/s. Against SIM 4, which damps as the test
     does, HPR Sim sits 0.00006587, 0.00006268 and 0.0002691 deg/s from it at every sample;
     at every sample the most is left in roll and pitch against SIM 5 and SIM 6, which damp the
     rate relative to the Earth, and in yaw against SIM 2.

6. **Cases 6 to 8 are the gravity and the codes' own air.** Six codes; SIM 3 is left out, as it
   cuts the spread from 273.1 to 275.5 mm to 85.03, 85.24 and 87.74 mm. HPR Sim's sphere ends within
   21 mm of SIM 4, SIM 5 and SIM 6, and 102.6, 102.8 and 105.0 mm from SIM 1, past the spread.
   - **The codes don't fly the same air.** Against HPR Sim's 1976 atmosphere at each code's own
     altitude, SIM 4's density is within 4.1e-7 of it and SIM 5's within 2.5e-6, SIM 1's up to 1.35e-3
     denser and SIM 2's 3.5e-4. NASA's report traces its own spread to the codes' atmosphere
     tables. In still air the report sets each code's printed drag force against `½ρV²SC_D` on
     its own printed density and speed: SIM 1's agrees to 1.1e-11, so that density is the air it
     flew.
   - **The named difference.** A point mass of the sphere's mass is flown twice from the case's
     start, by RK4 at 1 ms, with the Coriolis force and the same drag and wind. Once as the code
     flies: in gravitation cut off after `J₂` with the centrifugal term, through the code's own
     density, read from its file as a ratio to HPR Sim's at the code's altitude and time. Once
     as HPR Sim flies: in WGS 84's normal gravity and the 1976 atmosphere. The gap is the second
     position less the first. It flies no rigid body and none of HPR Sim's flight code.
   - **What it leaves** of each kept code's largest difference: at most 77.57, 77.63 and
     77.68 mm, against SIM 1, inside the spread. Against the others it leaves 0.0025 to 9.6 mm;
     HPR Sim sits 0.007677, 0.01569 and 0.004812 mm from the nearest kept code.
   - **Gravity alone is not enough.** The report gives the tool with the gravity alone, the
     code flying HPR Sim's air: it leaves 85.04 mm against SIM 1 in case 6 and 85.25 mm in
     case 7, each 0.01 mm past the spread, and 87.71 mm in case 8, inside. The spread is SIM 1
     against the codes that fly the 1976 atmosphere; HPR Sim, less its gravity, sits on those
     codes, so against SIM 1 it comes to the spread and a part of its own small distance from
     them.
   - **SIM 1 is not explained, only brought inside.** Its own air, flown, would put its sphere
     180.2 mm above HPR Sim's, where its file is 102.6 mm above: the density over-explains it,
     and the 77.57 mm left is the other way. What else SIM 1 does differently is not known
     here. The trace meets the rule; it does not account for SIM 1.

7. **Cases 9 and 10 are within the spread.** HPR Sim ends 1980 and 1977 mm from the farthest
   kept code, inside spreads of 1989 and 1986 mm; in case 10 SIM 2 is left out, 46.51 m from HPR
   Sim. Nothing needs a trace. The same model difference is reported: less it, HPR Sim sits
   0.03830 and 0.04217 mm from the nearest kept code, SIM 5, but 1991 and 1988 mm from SIM 3,
   just past the spread, so it is not claimed as their cause. These are the cases where the launch
   frame's flat axes carry a body kilometers downrange, so the conversion HPR Sim uses for apogee
   and landing points is part of what they hold.

8. **[#1524][i-1524], each item.**
   - **The spread rule for three codes.** It now leaves a code out of three when that more than
     halves the spread; two must remain. ADR-351's *done when* holds no other condition.
     ADR-353's "with three codes or fewer none is left out" is amended to this. No committed
     figure moves: every flown case has four codes or more. Test:
     `three_codes_may_leave_one_out_and_two_may_not`.
   - **Digit counts** take a figure's exponent from its printed form, not `log10`, so every
     platform counts the same digits. Test: `figures_keep_four_significant_digits`.
   - **The archive reader** refuses two files of one code, numbers a row as the file numbers its
     lines (blank ones included), and has a test of its own on an archive built in memory:
     `the_archive_is_read_by_folder_and_code`.
   - **The case's quantity** is an enum (`Measure`), and its trace another (`Trace`), not a
     match on the case's number.
   - **`FreeBody::new`'s symmetric inertia** stays exact. Its doc now says the caller averages a
     tensor with its transpose first, so what is flown is what the caller chose, not a repair
     made inside.

9. **What the comparison does not see.**
   - **SIM 1's 77.6 mm in cases 6 to 8,** as above, and SIM 3's 170.5 to 170.7 mm, left out.
   - **The attitude** in any case ([#759][i-759]); for the sphere it moves nothing.
   - **A common error of the codes,** which would pass unseen.

**What doesn't change.** M1.14b4's *done when* stands as ADR-351 wrote it; no tolerance is set.
HPR Sim's gravity, atmosphere and rotational equations are as they were, and
[#1521][i-1521] still carries the Earth's rate. Cases 1 and 2's figures are unchanged. [The page
on rigid-body flight][flight] gives the comparison to a reader.
