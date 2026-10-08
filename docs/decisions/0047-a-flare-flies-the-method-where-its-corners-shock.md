# ADR-047: A flare flies the method where its corner's shock is attached, and is read drawn out where it is not (2026-09-20)

- **Status:** accepted
- **Summary:** A flare flies the method where its corner's shock is attached, and is read drawn out where it is not

**Status:** accepted. **Milestone:** M1.8e17, the second of the three the old M1.8e14 split into.

**Context.** M1.8e17 asks for a flared body that flies the second-order shock-expansion method
**where the flare's shock is attached**, with no jump across that boundary. The march has always
been able to take a flare; what refused one was the model around it, which ended its run at the
first widening body and then dropped the *whole* body to slender-body theory at every Mach number
— worth −27.5% of the normal force and 0.29 calibres on the tests' rocket at Mach 3.

M1.8e14 measured the march's own edge first (ADR-045) and found it is the corner's isentropic turn
running out, not the shock detaching, and that it lands on **both** sides of a wedge's detachment
angle depending on the tube ahead of the flare. So the attachment test had to be chosen here, not
read off the march's refusal.

**Decision.**

- **The test is the corner's, read at the flow reaching it.** A flare's shock springs from a
  circular corner, not from a point apex, so where it forms the flow is two-dimensional: the
  body's radius is the scale over which the axisymmetric relief acts, and at the corner itself
  there is none of it yet. hpr therefore takes NACA Report 1135's largest deflection behind an
  attached plane oblique shock (eq. 168 into eq. 138,
  `blunt_tip::wedge_detachment_angle_rad`) — **the same test TN D-4865 p. 5 uses to hand a blunt
  tip's Newtonian cap over to this method**, so the project now applies one attachment rule at
  both corners it has.

  It is read at the surface Mach number the march delivers to the corner, not at the free stream:
  a new `ShockExpansionBody::aft_flow` returns it, and the march is downstream-only, so the flare
  cannot change it. On the tests' flared rocket that flow is Mach 1.9998 in a Mach 2.0 free
  stream, and the limit 22.9698° rather than 22.9735°.

  A cone's shock holds to steeper angles than a wedge's and a conical flare on a cylinder sits
  between the two (ADR-045), so **if** that is where a flare's own boundary lies, this errs one
  way only: it stops reading some flares whose shock is still attached, and reads none whose shock
  is not. "Conservative" is the wrong word for what that buys, though. Erring low does not mean no
  reading — it means the reading of a different body, and nothing here measures how different.

- **Past the limit, the flare is read drawn out, not dropped.** A flare steeper than the limit is
  read as one of **the same radii** drawn out to the limiting turn — a longer, shallower flare —
  with its centre of pressure kept on the real flare, at the same fraction along it. This is
  Washington and Pettis's boattail rule turned around (ADR-040: a boattail past 16° reads the
  correlation of one of the same radii drawn out to 16°), and it is what makes the reading
  continuous: **at the limit the drawn-out flare is the real flare**, so the two branches meet by
  construction. The radii, which set how much air the flare turns, are never changed.

- **The 30° of the cone tables caps the angle, not the turn.** Past `CONE_TABLE_CAP_RAD` an
  element has no tangent cone to look up (NASA SP-3007 stops at 30°, ADR-042) — but that is a
  bound on the flare's *surface angle*, while the shock bounds the *turn* at its corner. They are
  bounds on different things, so each caps its own quantity: `flare_corner_limit_rad` returns the
  turn, and the model takes `min(turn + the angle ahead, 30°)` as the angle it draws. On a flare
  behind a cylinder, where the angle ahead is zero, the two coincide and the tables bind from Mach
  2.5192034260 up.

- **The run ends at the flare.** Nothing behind a flare is marched, and it takes slender-body
  theory's share, which for a cylinder is nothing. Keeping the run open would have meant marching
  parts whose stations move when the flare is drawn out.

- **Only a conical flare joins the run, and only in the free stream.** Any other widening shape
  ends the run as before: the corner's turn is then the profile's own slope at its fore end, which
  this test is not written for. A widening part behind a boattail stays a lip in its wake under
  ADR-039, whose flow the march does not compute.

- **The old rule stays selectable.** `BodyModel::with_supersonic_flare(SupersonicFlare::SlenderBody)`
  reproduces every number from before this milestone, and `BodyModel::BEFORE_M1_8E6` takes it.

**What it is worth.** On the tests' flared rocket — an ogive nose, a tube, a 0.3 m conical flare,
a tail tube and four fins — the method reads *less* normal force than slender-body theory and puts
the centre of pressure forward of it, so a flared rocket now reads **less** stable, not more:

The rocket is an ogive nose 0.25 m long on a 27 mm radius, a 0.7 m tube, a 10° conical flare
0.3 m long opening to 79.9 mm, a 0.2 m tail tube and four fins; the reference is its largest
diameter, 0.1598 m.

| | the method | slender-body theory | the method's centre of pressure |
|---|---|---|---|
| Mach 2.0 | 3.5371 per rad at 7.365 calibres | 3.7153 per rad | 0.089 calibres forward |
| Mach 3.0 | 2.8899 per rad at 7.015 calibres | 3.0815 per rad | 0.169 calibres forward |
| Mach 4.95 | 2.4880 per rad at 6.681 calibres | 2.6431 per rad | 0.238 calibres forward |

Nothing here compares either column with a measured flare. That is M1.8e18, which commits TN
D-4865 model 2's readings, and until it lands the guide says the size is unvalidated.

**That it does not jump.** At Mach 2.0 — a row of the table, so the reading is that row's and not
an interpolation — the boundary is a flare of 22.969761173077°. Probing either side of it:

| probe | the normal-force slope moves by |
|---|---|
| ±1e-9° | 4.527e-11 |
| ±1e-7° | 4.527e-9 |
| ±1e-5° | 4.527e-7 |

Each figure is a fraction of the slope itself. The gap falls a hundredfold for each hundredfold in
the probe, so what the probe finds is a slope, not a step: the reading is continuous across the
boundary.

**Its first derivative is not, and this decision does not claim it is.** A cap makes a kink: below
the boundary the flare's angle moves the body the march sees, above it only the radii do. Measured
at Mach 2 on the tests' flared rocket, `dC_Nα/dδ` changes by **−31.4%** across the boundary on the
whole rocket and by **−141.6%** on the flare's own share, where it changes sign. The marched branch
is not smooth in the flare's angle either — the same probe at 20°, away from any boundary, finds
+3.5% and +41.1% — so the kink is a change of degree, not of kind.
`the_cap_makes_a_kink_in_the_slope_even_though_the_reading_holds` pins all six figures. ADR-039
claimed only "continuous in shape" for the same construction, and that is the claim here too.

The same probe in the Mach number, at 18.5° — TN D-4865 model 2's angle, whose shock holds on this
body from Mach 1.767666917849 — gives 3.622e-10, 3.622e-8 and 3.622e-6. That half is taken on the
**rows** the table is built from rather than on a reading between them: the crossing falls inside
the Mach 1.75 to 1.80 interval, where `SupersonicBody::share` runs a straight line, so a reading
there would look continuous whatever the two branches did.
`nothing_jumps_where_the_flares_shock_detaches` pins both halves.

**What still switches, and it is not the boundary above.** Where the march itself refuses a flare,
the model keeps slender-body theory rather than reading a flare it has not marched — and that
refusal is a switch of shape. Three cases, which behave differently. *(The second and third of
them are superseded by ADR-050, which reads that element rather than refusing it; the two tests
named below were replaced with it. The first is unchanged.)*

- **Below Mach 1.5552** on this body the corner's isentropic turn runs out before its shock
  detaches (ADR-045's crossing), so the table simply starts there. The join's weight rises from
  zero over 0.3 Mach, so the reading is continuous in Mach through it — measured at 8.5e-10 for a
  1e-9 probe.
- **A flare of about 0.03816° to 0.05882°** — a rise of a third of a millimetre over 0.3 m — has
  its one element *reduced* aft of the nose (issue #81: the pressure behind the corner moves away
  from its tangent cone's), and the march then refuses a run of Mach numbers from the top down:
  none at 0.0589°, from Mach 5.0 at 0.045°, from Mach 4.70 at 0.03817°. The table is built
  downward from Mach 5 and needs the join's whole width inside it, so once that run reaches Mach
  4.70 there is no table at all and the whole body falls back to slender-body theory at every Mach
  number. (Shallower than the band the refusal only shortens the table and lifts the join, which
  stays continuous.) Crossing the steep edge is worth −8.3% of the normal force and 1.16 calibres
  at Mach 3 and 4°, the size of the switches issue #87 already tracks. It is **not** in #87,
  because M1.8e15's *done when* is to close or narrow that issue to the step alone: it has its own
  issue, #117, and its own milestone, **M1.8e19**.
  `the_march_refuses_two_bands_of_flare_and_the_model_keeps_slender_body_theory` pins the
  mechanism either side and the switch's size.

  The band's two edges are quoted to five digits because that is about what they carry. Each is
  the root of the march's own refusal at a fixed Mach number, which turns on the sign of
  `p_c − p₂`, a difference of two pressures within a thousandth of each other there; the
  cancellation leaves the root about nine significant digits, and the three platforms CI runs
  spread the shallow edge over 2.6e-10° (6.8e-9 relative). Bisecting it to f64 is a statement
  about one machine, not about the model.
- **Shallower still, from about 0.0009°, the join's start steps rather than slides.** The refused
  run of Mach numbers is what the table stops at, so as the flare flattens the table's start walks
  up in 0.05 Mach steps — and not monotonically: 0.00024° and 0.0003° sit at the floor, Mach 1.2,
  while 0.00025° between them does not. The shallowest switch that stays switched is at
  **0.00090182°**, a rise of 4.7 µm over 0.3 m, where the start goes from Mach 1.2 to Mach 2.2:
  −4.6% of the rocket's normal force and 0.75 calibres at Mach 2.
  `a_near_flat_flare_lifts_the_joins_start_in_steps` pins it. This is the same defect as the band
  above, one step down in angle, and #117 and M1.8e19 carry the whole near-flat region, not just
  the band.

**Not chosen: fade the method out approaching detachment.** A weight going to zero at the
boundary would also be continuous, but it throws away the method exactly where it is still valid,
and it needs a fade width nothing measures. ADR-039 rejected the same construction for a boattail,
for the same reason.

**Not chosen: cap the flare's angle by shrinking its rise.** Reading a 30° flare as a 23° one of
the same *length* throws away most of the radius change, which is what sets how much air the flare
turns: the reading would fall far below even slender-body theory's, which depends on the radii
alone. Drawing the flare out keeps the radii and changes only the angle, which is the quantity the
attachment test is about.

Note what that means above the limit: with the radii fixed, the drawn body no longer depends on
the flare's angle at all, so **every flare steeper than the limit reads the same force**. Held at
the limit, in other words, which is the honest description of the rule. There is no bound on how
far that goes: at Mach 2 a 75° flare — an annular face a detached bow shock stands in front of —
reads 1.638 per radian against slender-body theory's 2.010, 18.5% below, where the truth is above
both. It reads a steep flare as less stabilizing than it is, which is the safe direction for a
stability margin and the wrong one for a load. The guide says so.

**Not chosen: read a detached flare by a detached-shock model.** There isn't one here. A bow shock
standing ahead of the juncture with a subsonic pocket behind it is not something a tangent-body
march describes, and inventing one would have been a model with nothing to check it against.
