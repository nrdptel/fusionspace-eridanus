# ADR-353: NASA's check cases 1 and 2: both differences past the codes' spread, traced (2026-11-17)

- **Status:** accepted; the decision record that [M1.14b3][m1-14b3]'s *done when* asks for ([ADR-351][adr-351], decision 2); no *done when* changed
- **Summary:** A free body of the report's mass and inertia flies NASA/TM-2015-218675's check cases 1 and 2 on HPR Sim's gravity, Coriolis force, rigid-body equations and default integrator. In case 1 its altitude ends 19.51 mm (0.77 in) below the codes', 137 times their spread: WGS 84's normal gravity holds the terms past J2 that the codes' gravitation drops, and that accounts for all but 0.1364 mm (0.0054 in). In case 2 its inertial body rates differ by up to 0.01633 deg/s, 12 to 16 times the spread: the rotational equations leave the Earth's rate out, and that accounts for all but 0.001005 deg/s. Both remainders are inside the spread.

[m1-14b3]: ../decisions-and-roadmap.md#m1-14b3
[m1-14b4]: ../decisions-and-roadmap.md#m1-14b4
[m1-14f]: ../decisions-and-roadmap.md#m1-14f
[adr-351]: 0351-m1-14b-in-eight-the-basic-finners-normal-force-first.md
[adr-011]: 0011-rigid-body-flight-equations-of-motion.md
[nesc-note]: ../research/nesc-check-cases.md
[report]: ../../validation/reports/check-cases.md
[report-json]: ../../validation/reports/check-cases.json
[flight]: ../physics/flight.md#a-free-body-against-nasas-check-cases
[i-759]: https://github.com/nrdptel/fusionspace-eridanus/issues/759
[i-1521]: https://github.com/nrdptel/fusionspace-eridanus/issues/1521
[i-1524]: https://github.com/nrdptel/fusionspace-eridanus/issues/1524

**Context.** [M1.14b3][m1-14b3] flies two of NASA's six-degree-of-freedom check cases
([the note][nesc-note]): case 1, a sphere dropped with no drag from 30,000 ft, and case 2, a
brick of three unequal moments of inertia tumbling with no aerodynamics, each for 30 s. The
report names no code as the answer. Five or six codes flew each case, and their published time
histories are the reference. The increment's *done when* sets HPR Sim's largest difference
from each code beside the codes' own spread, and asks that any difference from a kept code
larger than that spread be traced, in a decision record, to a named difference between the
models. The size of that difference must be computed apart from the flight and must leave no
more than the spread unexplained. Both cases came out past the spread, so this is that record.
Every measured figure below is in [the committed report][report] ([as data][report-json]), which
`cargo xtask check-cases --check` holds in the gate's `refs` step.

**Decision.**

1. **The free body is the flight's code, not a second model.** `hpr_sim::FreeBody` has a mass
   and an inertia and nothing else: no design, no motor, no aerodynamics. Its derivative takes
   gravity and the Coriolis force from the flight's Earth model and the two accelerations from
   the flight's own free-motion equations, and the flight's integrator steps it at its default
   tolerances. A test evaluates a coasting rocket by the equations a flight integrates and the
   free body of that rocket's mass and inertia at the same state: the two agree to rounding.

2. **The spread, as computed.** The spread of a quantity is the largest difference between
   any two codes at a time every code's file has (the files share a 0.1 s grid; one code's is
   at 0.01 s). The code left out is the one whose leaving cuts the spread most, and only when
   that cuts it by more than half. With three codes or fewer none is left out (since
   [ADR-356](0356-nasas-check-cases-3-and-6-to-10-the-tests-loads-and-the-codes-own-air.md): with two).
   - Case 1, altitude, six codes: 0.6351 mm over all; SIM 1 left out; 0.1424 mm over the rest.
     SIM 1's own difference from HPR Sim is −20.00 mm.
   - Case 2, inertial rates, five codes: 0.003015, 0.004742 and 0.001152 deg/s in roll, pitch
     and yaw over all; SIM 6 left out of each; 0.0008991, 0.001242 and 0.0003532 deg/s over
     the rest. SIM 6's own differences are +0.007986, −0.01177 and +0.004861 deg/s.

3. **Case 1's difference is the gravity model's.** HPR Sim's altitude ends 19.51 mm below
   four of the five kept codes and 19.37 mm below the fifth, after a fall of 4.4 km.
   - **The named difference.** HPR Sim falls in WGS 84's normal gravity, the closed form that
     holds every even zonal harmonic of the level ellipsoid. The codes fall in gravitation cut
     off after J2 (the report's eq. 24), with the centrifugal term their rotating Earth
     supplies. On the equator the first term past J2 strengthens the attraction by about 4.4
     parts in a million.
   - **Its size, apart from the flight.** The report's tool drops a point down the site's
     vertical in the codes' field and integrates the gap in downward acceleration twice along
     that fall: −19.50 mm at 30 s, growing as the square of the time. It uses no flight of
     HPR Sim's and no code's file.
   - **What it leaves** of each kept code's largest difference: 0.1364 mm against SIM 2 and
     0.004434 to 0.005492 mm against the other four. The spread is 0.1424 mm.

4. **Case 2's difference is the Earth's rate, left out of the rotational equations.** HPR
   Sim's inertial rates differ from the kept codes by up to 0.01077, 0.01633 and 0.005500
   deg/s in roll, pitch and yaw.
   - **The named difference.** The codes turn the brick by Euler's equations in its rate
     relative to the stars. HPR Sim's rotational equations treat the launch frame as inertial
     ([ADR-011][adr-011]): they turn it by the same equations in its rate relative to the
     Earth-fixed launch frame, and its inertial rate is that plus the Earth's, turned into the
     body.
   - **Its size, apart from the flight.** The report's tool integrates both sets of equations
     from the report's start by a fixed-step method of its own and takes one from the other:
     up to +0.01077, −0.01633 and +0.005500 deg/s. It uses no flight of HPR Sim's. Its
     integration of the codes' own equations sits within 0.000000003047 deg/s of SIM 1's and
     SIM 4's files in every rate, so the start was read as those codes flew it.
   - **What it leaves** of each kept code's largest difference: at most 0.0008818, 0.001005
     and 0.0001268 deg/s, each against SIM 2. The spreads are 0.0008991, 0.001242 and
     0.0003532 deg/s.
   - **The size of what is left out was understated.** ADR-011 gives the Earth's rate, 7.3e-5
     rad/s (0.0042 deg/s), as the most the rotational equations miss. That is the size of the
     term. On a tumbling body of three unequal moments its effect grows: up to 0.01633 deg/s
     in pitch within the 30 s, 3.9 times the Earth's rate. [#1521][i-1521] carries the choice
     between putting the Earth's rate into the rotational equations and measuring what its
     absence costs a flight inside the envelope, for [the audit's physics gaps][m1-14f].

5. **How "leaves no more than that spread" is read, and a stricter figure beside it.** The
   *done when* speaks of a code's largest difference and the size of the model difference.
   The report takes the model difference at the sample where the code's largest difference
   falls and gives what is left; a quantity is traced when that is no more than the spread
   for every kept code. All four quantities are.
   - **The stricter figure** is the same subtraction's largest at any sample of the 30 s. It
     is given beside the first, because it does not stay inside the spread: 0.0008993,
     0.001242 and 0.0003533 deg/s in case 2, over the spread by 0.0000001701, 0.00000007201
     and 0.00000007634 deg/s. In case 1 it is 0.1369 mm, inside.
   - **Why it can't.** In each rate the spread is set by SIM 2 against SIM 1 and SIM 4, which
     agree with each other. A flight that sat exactly on SIM 1 would differ from SIM 2 by the
     spread and no less. HPR Sim, less the model difference, sits 0.0000005866, 0.0000009229
     and 0.0000002790 deg/s from the code nearest it, so against SIM 2 it comes to the spread
     plus a part of that.
   - **That distance is the integrator's step error.** Flown again at tolerances of 1e-12 in
     place of the default 1e-8, it falls to 0.000000002126, 0.000000003051 and 0.000000001089
     deg/s, the level at which the tool's own integration meets those files. The report gives
     both flights. The comparison that counts is the default's, the one a rocket flies on.

6. **What the comparison does not see.**
   - **The Coriolis force's sign.** The force carries the falling sphere east, and that
     eastward speed is in turn pushed up, so the lift is even in the sign of the Earth's rate.
     Case 1's altitude holds the size of the term (7.0 mm of the fall, a closed form a test
     pins on the free body) and not its sign. The eastward drift would, and it is not compared
     here; [the next increment][m1-14b4] compares positions.
   - **The attitude.** Case 2 compares inertial body rates, as the *done when* names them. The
     files also hold Euler angles, which are not compared: an error in the attitude reaches
     the inertial rates only through the Earth's rate turned into the body. [#759][i-759]
     stays open for them.
   - **A common error of the codes.** The codes are the reference. An error that all the kept
     codes share would pass unseen.

**What doesn't change.** M1.14b3's *done when* stands as ADR-351 wrote it; no clause is
narrowed and no tolerance is set. HPR Sim's gravity and rotational equations are as they
were: the increment adds a reference and two measured differences. ADR-011's choice stands
until [#1521][i-1521] is worked. [The page on rigid-body flight][flight] gives the comparison
to a reader.

**Amended 2026-11-17 ([ADR-356](0356-nasas-check-cases-3-and-6-to-10-the-tests-loads-and-the-codes-own-air.md), [#1524][i-1524]).** The spread rule may leave a code out of three, where it had needed four or more: two must remain, and ADR-351's *done when* holds no other condition. No figure here moves.

**Amended 2026-11-17.** Withdrawn by [ADR-355](0355-withdraw-nasas-check-cases-1-and-2-to-land-them-again-in-order.md) and landed again unchanged.
