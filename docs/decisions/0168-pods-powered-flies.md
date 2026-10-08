# ADR-168: *Pods--powered* flies, and the margin is the weakest plane's (2026-10-05)

- **Status:** accepted
- **Summary:** M4.5i: a rail button's screw head weighed as half an ellipsoid; the thrusting motors' area told apart per pod set; a device at the charge of a stage where no motor lights never opens, as in OpenRocket; the margin reported in the rocket's weakest plane (#329); OpenRocket's *Pods--powered with recovery deployment* flies its three complete configurations, two within 1.9% of OpenRocket's apogee, the third turning over before apogee in both tools

**Context.** M4.5 asks that every in-scope OpenRocket example fly as saved. *Pods--powered with
recovery deployment* flew none of its four configurations in `hpr sim`. The first reason given was
its two rail buttons' screw heads, which the reader dropped. Patching each reason out in turn found
three more behind it:

1. The design's checks find its centering rings, children of the booster's 18 mm motor tube,
   16.5 mm from that tube's axis where it has 9.1 mm of room. They can't fit as written, as on
   *Pods--airframes and winglets* ([ADR-166](0166-a-fin-root-that-follows-the-body.md)), so the
   checks stand and `hpr sim` flies the file with `--accept-design-errors`.
2. The aerodynamic model refused motor mounts in more than one pod set: the thrusting motors' area
   in pods was one total (ADR-092), which two sets of bases can't share.
3. In `[None; 2× A10-3, B6-4]` the sustainer's parachute opens at its stage's ejection charge, and
   the sustainer has no motor. hpr refused the configuration's whole recovery for it.

With all three patched, the sustainer of `[C6-7; B6-0]` turned over in hpr's flight of the
record's conditions: apogee 95 m against OpenRocket's 152.7 m. hpr's summary said its least margin
was +1.56 cal.

**Decision.**

1. **A rail button's screw head is half a solid ellipsoid of revolution** on the flange, as wide as
   the button: radius `a`, half the outer diameter, and the screw's height `h` along its axis.
   Volume `2/3 π a² h`, centre `3h/8` above the flange, moment of inertia `2/5 m a²` about its own
   axis and `m (a²/5 + 19 h²/320)` across it through its centre. It has no drag. The design holds
   `screw_height_m` (default 0), the reader fills it from `<screwheight>`, and the writer writes it.
   A button with no screw keeps the three discs' mass properties bit for bit.
2. **Each pod set holding motor mounts has its own thrusting area.**
   `DragConditions::thrusting_pod_motor_areas_m2` holds one total per set, in
   `Layout::motor_pod_sets`' order, and each set's pods share their set's. At most
   `MOTOR_POD_SETS`, 4, sets are told apart; a rocket with more is refused by name. The fixed
   number keeps the conditions a plain value, built at every step without allocating.
3. **A device at its stage's charge, in a stage where no motor lights, never opens**, as in
   OpenRocket. `hpr sim` notes it and flies the other devices (`NeverDeploys::NoLitMotor`).
   `RecoveryRefused::NoMotor` goes.
4. **The margin is the least over the direction the air crosses the rocket** (#329). A fin set of
   one or two fins carries `Σ sin²(φ − θ_k)` of its force in the plane at `φ` (Niskanen 2009
   eq. 3.51, `roll_sum`). The flight flies each plane's force, but the margins it reported were
   the 0° plane's alone, which can be the strongest. `hpr_sim::metrics::weakest_margin` gives the
   static and flight margins, `hpr::Rocket::margin` and `hpr sim`'s margins (with the plane's
   direction, `roll_rad`). It is found in closed form: the net slope `S(φ)`, its moment `M(φ)`
   and the sum of the parts' slope magnitudes are each `a + b cos 2φ + c sin 2φ`. Three planes give
   their coefficients, and a quotient of two such sums is stationary where
   `P sin 2φ + Q cos 2φ + R = 0`. The centre of pressure `M/S` is foremost at one of its roots,
   and the margin's conditioning `S/Σ` is worst at one of its. Each root and the axial plane are
   evaluated through the model; a plane where no margin can be given wins, else the least margin.
   A rocket whose fin sets are each of three or more fins, or that flies a normal-force table,
   keeps its margin bit for bit.
5. **A flight that turns over before apogee in both tools gets a named cause.** OpenRocket's
   record tumbles before its apogee, hpr's least static margin to apogee in the weakest plane is
   negative, and hpr's own angle of attack passes 90° before its apogee. The report writes all
   three (`openrocket_tumbles_s`, `hpr_least_margin_cal`, `hpr_turns_over_s`), and names the
   cause "the rocket turns over before apogee in both tools". Such a flight has no rigid-body
   reference: OpenRocket's apogee there is its tumble model's. ADR-076's bar for staged designs
   leaves it out by name, and so does the descent comparison's group of flights with no named
   cause. `a_flight_turns_over_only_when_both_tools_flights_do` holds each sign as needed.
6. The aborted `[C6-7; 2× A3-4, B6-0]` stays refused: hpr doesn't work out which of the booster's
   three mounts burns out first to light the sustainer, and OpenRocket aborts the flight at 1.81 s
   (ADR-068).

**Measurements.** OpenRocket 24.12 was run as an oracle on probe designs (scratch scripts, not
committed). The rail button probes use a 25 mm body, a 10 mm button 8 mm tall on 2 mm discs, a
6 mm waist and 1000 kg/m³. Screws of 0, 1, 2 and 5 mm, and two other diameters, gave:

- the volume to 2e-16 with `2/3` in single precision, 0.6666666865348816 (OpenRocket's);
  hpr's double `2/3` gives 1.1e-8 of the volume at 5 mm;
- the volume depends on the outer diameter alone;
- the head's centre `4h/3π` above the flange, fitted to 1e-14. That is a flat semicircle's
  centroid, 0.049 h further out than the solid's `3h/8`;
- the rocket's drag coefficient, centre of pressure and normal force unchanged by the screw,
  bit for bit;
- the button's centre moved by the screw under a mass override as without one.

`a_rail_buttons_screw_head_is_half_an_ellipsoid` holds the probe volumes to 1e-14, with the
single-precision gap taken out. It reproduces OpenRocket's measured centre at 2 mm to 1e-12 under
OpenRocket's own volume and centroid, holds hpr's to its closed form, and checks a hemisphere's
inertia (83/320 m a² across) for a head as tall as it is wide. Both buttons of the example
override their mass, 0.05 g each, so only the screw's centre reaches its flight.

The charge probes flew the jar's *Two stage high power rocket* with the sustainer's drogue set to
its stage's charge. With no sustainer motor, with one never lit, or with a plugged one, it never
opened: stages joined or apart, at a booster charge at 6.515 s. With an I59WN of 20 s delay it
opened at its own charge, at 30.12 s, past the booster's. OpenRocket's own label for the setting is
"First ejection charge of this stage" (`validation/fixtures/ork/openrocket-events.json`). In the
pods example's `[None; 2× A10-3, B6-4]`, OpenRocket opens only the booster's two pod parachutes,
at launch + 5.35 s by that configuration's own setting. The sustainer's parachute never opens.

The sustainer of *Pods--powered*, in hpr's model at Mach 0.2 with its centre of mass after the
booster drops, has a margin of +1.86 cal with the air crossing at 0°, +0.17 cal at 45° and
−3.44 cal at 90°. At 90° its two-fin strake set is edge-on and only the single fin at 0° works.
OpenRocket, on the same sustainer, puts the centre of pressure at 0.2954 m with C_Nα 3.49 at 90°;
hpr's are 0.2993 m and 3.47. In a scratch run seeded with a 0.5° rod tilt, OpenRocket aborts the
configuration at 1.71 s at 51 m. With 2 m/s of wind it aborts at 0.16 s. `hpr sim`'s least margin
for `[C6-7; B6-0]` was +1.56 cal and is now −4.21 cal, at the separation.
`the_least_margin_is_the_weakest_plane_s` holds the closed form on four uneven fin layouts at
three Mach numbers and three centres of mass. The margin is never above any of 3,600 scanned
planes', and within 1e-5 cal of the scan's least. Putting the axial plane back fails it.

**Result.** `cargo xtask ork-flights`, against OpenRocket 24.12's record and its launch
conditions:

| configuration | apogee, OpenRocket / hpr (m) | Δ | largest speed Δ | descent |
|---|---:|---:|---:|---|
| `[C6-5; None]` | 131.0 / 130.5 | −0.36% | +0.15% | met: landing speed +0.00%, flight time +0.86% |
| `[None; 2× A10-3, B6-4]` | 115.6 / 113.5 | −1.86% | +0.12% | met: both pod parachutes at 5.35 s, landing speed +0.00%, flight time −1.02% |
| `[C6-7; B6-0]` | 152.7 / 94.8 | −37.93% | −15.76% | missed: flight time −52.64%, the rocket turns over |

**Not met, recorded:** `[C6-7; B6-0]`. Its sustainer is unstable when the air crosses it edge-on
to its strakes, in both tools, and in the record's conditions both flights turn over. OpenRocket's record shows a tumble at 2.84 s, before its apogee at
4.07 s: its calm, vertical flight holds the angle of attack at zero until about 2.5 s. hpr's least
margin to apogee is −4.21 cal, and its angle of attack passes 90° at 2.25 s, as the Earth's
rotation at the record's site, 28.61° N, tips it. Where each tool's rocket turns over depends on
what first tips it, so neither apogee is a reference for the other. At `hpr sim`'s default site,
on the equator, nothing tips hpr's flight: it stays upright to 251.3 m. The margins at rod
clearance, taken with the air along the axis as OpenRocket's are, are all 0.07 cal above
OpenRocket's, as on *Pods--airframes and winglets*: not yet traced, two candidate causes filed
as #325 and #326.

**Consequences.** Motor mounts may sit in up to four pod sets. A screw head is part of a button's
mass. A configuration with a device that never opens still flies its other devices. At each
instant, every rocket with a one- or two-fin set reports a margin no higher than before, or none
where its weakest plane has none; a least over a staged flight can then come from another stage.
The other OpenRocket examples' figures are unchanged. The `.ork` survey's kept tags fall
from 1,835 to 1,819 and its dropped values lose the screw heads. The design format's schema and
types gain `screw_height_m`; the CLI's JSON margin gains `roll_rad`. `xtask`'s report gains a
named cause, and the census accepts the three new flights, one over the bar, with this ADR's
reason.
