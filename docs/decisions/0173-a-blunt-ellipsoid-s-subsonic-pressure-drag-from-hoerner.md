# ADR-173: A blunt ellipsoid's subsonic pressure drag, from Hoerner's measurement (2026-10-05)

- **Status:** accepted
- **Summary:** M4.5o: below Mach 0.8 an elliptical nose or shoulder takes a pressure drag interpolated in fineness between Hoerner's low-speed measurements (a hemisphere 0.01, a round head one diameter long −0.05, a flat face the step's), held at 0 from 7/12 calibre on, and rises in a straight line from Mach 0.8 to Stoney's scaled curve at Mach 1.2. *Base drag hack (short-wide)*'s 0.577-calibre nose gets 0.0008, so its E12-4 stays 7.80% above OpenRocket's apogee with recovery in both: the 5% bar would need about 0.013 to 0.015, which no measurement supports. Not met, recorded; M4.5 is closed with that gap.

**Context.** M4.5o asks that *Base drag hack (short-wide)*'s E12-4 apogee come within 5% of
OpenRocket's, with recovery as saved, "from a blunt nose's subsonic pressure drag taken from a
cited measurement and pinned by a test against it". It read 7.97% high
([ADR-167](0167-a-part-s-drag-override-as-openrocket-flies-it.md)). Flown on OpenRocket's own drag
curve, hpr comes within 0.03% of OpenRocket's flight, so the whole gap is the drag coefficient. By
OpenRocket's breakdown it is the nose, an ellipsoid 0.577 calibres long: OpenRocket charges it
0.0117 at Mach 0.1 and 0.064 at Mach 0.3, about `0.41 M^1.55`
([#177](https://github.com/nrdptel/hpr-sim/issues/177)). hpr charged every ellipsoid nothing below
Mach 0.8 (ADR-028): eq. B.9 scales Stoney's fineness-3 curve, which is 0 there, and gives 0 at
any fineness. A near-flat ellipsoid therefore got no drag below Mach 0.8, though a flat face
gets 0.85. Above Mach 0.8, B.9's exponent `log₄(f + 1)` made a blunt ellipsoid's drag rise like
`(M − 0.8)^0.33`, with an infinite slope at Mach 0.8: 0.25 by Mach 0.85 at 0.577 calibres.

**The measurement.** Hoerner, *Fluid-Dynamic Drag* (1965), p. 3-12, Fig. 20, prints the forebody
pressure drag of seven heads on a cylinder, "evaluated from pressure distribution": a round head
drawn about one diameter long −0.05, a hemisphere 0.01, a flat face 0.8, on the cylinder's
area, friction not included; the figure gives no Reynolds number. The text explains the sign: a
streamlined half-body has no drag in theory, and "the phenomenon of zero or negative forebody drag
is also found to some extent in real and viscous fluid flow". Niskanen 2009 p. 47 quotes the
hemisphere's 0.01 and says eq. 3.86 "does not take into account the effect of extremely blunt nose
cones (length less than half of the diameter)". A search of Hoerner's chapters 3, 10, 15 and 16,
Stoney's NASA TR R-100, Mason's NSWC TR 81-156 and MIL-HDBK-762 found no measured subsonic point
for an ellipsoidal head of this length between Mach 0.3 and 1. Below the critical Mach number,
about 0.65 to 0.7 for a hemisphere by the Prandtl–Glauert rule, the pressures scale together, and
a half-body's pressure drag stays near 0. OpenRocket's value grows fourfold from Mach 0.1 to 0.25,
which no compressibility effect explains.

**Decision.**

1. Below Mach 0.8, an elliptical nose or shoulder of fineness `f` takes Hoerner's measurement,
   `hpr_aero::nose_drag::ellipsoid_subsonic_pressure_drag`:
   - from the hemisphere (`f = ½`) on, the straight line through 0.01 there and −0.05 at `f = 1`,
     held at 0 once it gets there (`f = 7/12`). A head no blunter than that is charged none, as
     eq. 3.86 charges a tangent joint none; hpr's coefficients stay non-negative;
   - blunter, hpr's interpolation, since Fig. 20 measures no ellipsoid between: eq. B.9's form
     between the flat face (`0.85 q_stag/q`, eq. B.2, at the same Mach number) and the
     hemisphere, `C₀ (0.01/C₀)^(ln(f + 1)/ln 1.5)`, the step at `f = 0`.
   From the hemisphere up the value holds unchanged to Mach 0.8, and blunter heads follow the
   flat face's rise with Mach number. That is hpr's assumption: the measurement is at low
   speed, and from the critical Mach number up the drag starts to rise. Eq. 3.86's `0.8 sin² φ`
   applies where it is more. An ellipse meets its base tangent, so it is 0 there. A caller that
   passes a joint angle above 0 gets a value held to Mach 0.8 that the line may then carry down
   to the scaled curve; no design's ellipse does that.
2. From Mach 1.2, Stoney's ellipsoid scaled by eq. B.9, as before.
3. Between Mach 0.8 and 1.2, where neither source measures, a straight line from the value at
   Mach 0.8 to the scaled curve at 1.2: the line is drawn after scaling, not before. At fineness 3
   it is the line ADR-028 drew. At 0.577 calibres, before → after: 0.248 → 0.069 at Mach 0.85,
   0.315 → 0.137 at 0.9, 0.410 → 0.274 at 1.0, 0.547 at 1.2 in both.

The values come out as follows: 0.0008 for this nose, 0.074 for a ¼-calibre ellipsoid at Mach 0,
0.01 for a hemisphere, and 0 from 7/12 calibre on. Fig. 20 prints no length for the round head.
Drawn 1.4 calibres long instead of one, the line would reach 0 at 0.65 calibre and give this nose
0.005, still under what the 5% bar needs. Tests:
`nose_drag::tests::a_blunt_ellipsoid_takes_hoerners_measured_forebody_drag` and
`an_ellipsoid_rises_from_mach_08_in_a_straight_line`. Mutation probes: the floor at 0 removed, or
eq. 3.86's value ignored, fails the first; the line drawn before the scaling fails the second.

**Result.** `cargo xtask ork-flights`, the descent's apogee with recovery in both programs, against
OpenRocket's (before → after):

| motors | recovery in both | nothing deployed in OpenRocket | hpr on OpenRocket's drag |
|---|---:|---:|---:|
| C11-5 | +2.06% → +1.95% | — | — |
| D12-3 | +4.65% → +4.52% | +6.03% → +5.84% | −0.10% |
| E12-4 | +7.97% → +7.80% | +9.13% → +8.92% | −0.03% |

`hpr sim` on the example's E12-4 reads 341.4 m against OpenRocket's 316.3 m (+7.9%; 342.0 m
before). **Not met for the E12-4**, recorded. A one-off probe, not committed, added a constant to
the nose's drag below Mach 0.8 in `hpr sim`. A constant of 0.01 gave 335.0 m and 0.02 gave
328.4 m, so the 5% bar (332.1 m) needs about 0.015 at Mach 0.1 to 0.27. The committed report
gives a second estimate: this change's 0.00076 moved the descent's apogee from 341.53 m to
340.99 m, which at the same rate needs about 0.013 in all. That is more than the
measured hemisphere's 0.01, for a head longer than a hemisphere, which the measured round head
puts at or below 0. Meeting the bar would mean copying OpenRocket's unmeasured rise and departing
from the measurement. The gap stays in the report, its cause named: OpenRocket's subsonic nose
pressure drag.

M4.5's other clauses are met: OpenRocket's 17 examples fly as saved on 54 of 56 configurations,
offline after one fetch (the other two are hybrid motors, out of scope until 1.0), and the CLI's
corpus count is published (M4.5j). M4.5 and M4.5o close with the E12-4 recorded as not met
([rule 2](../../CONTRIBUTING.md#2-never-weaken-a-check)), rather than take a third increment at a
bar the measurement rules out.

**Consequences.** The census reads six rows better, none worse. No other public flight moves: the
other elliptical nose, *Clustered motors*', is 1.82 calibres long and still gets 0. The private
library report, the real flights and the CLI's count are unchanged.

**Left open.** No measurement covers a blunt head between Mach 0.3 and 1.2. Hsieh's
hemisphere-cylinder pressures (AEDC-TR-76-112) would, but they were not reachable. The x^¼ power
series still joins Stoney's curve before scaling, with the same infinite slope at Mach 0.8, and
power series blunter than x^¼ interpolate toward it.
