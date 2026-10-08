# ADR-034: The body's supersonic normal force in flight: tabulated shock-expansion shares, joined linearly from Mach 1.2 (2026-09-19)

- **Status:** accepted
- **Summary:** The body's supersonic normal force in flight: tabulated shock-expansion shares, joined linearly from Mach 1.2

**Context.** M1.8e1 built the second-order shock-expansion method (ADR-033) as a library model,
but a flight still took slender-body theory's body terms at every Mach number, and
`hpr-sim`'s dynamics cached the bodies' damping stations at Mach 0. M1.8e2 flies the method for
a pointed nose and the cylinder behind it; M1.8e3 takes the boattail, crossflow and blunt tips.
One run of the method takes about 2.5 ms in a release build and 4 ms in debug, so it can't run
at each step of a flight. A flight's damping needs each component's own force at its own station
(ADR-011), so the method's force has to land on components, not only on the whole body.

**Decision.**

- **What it covers.** The nose, if it is the first body and pointed, and the body tubes straight
  behind it at the same radius. The run stops at the first transition, step in radius (over a
  millionth of the area) or gap. Each covered component takes its own segment's share
  (`ShockExpansionBody::segment_slopes`). Body lift (Galejs's `sin² α` term) is unchanged.
- **Only a body it can finish.** The method flies only if no body after the covered run has a
  potential-flow slope of its own (a boattail, a flare, a step). Otherwise the whole body keeps
  slender-body theory until M1.8e3. The first draft flew the method's nose and cylinder beside
  slender-body theory's boattail; the physics review showed that this moves the body's centre of
  pressure further from the wind tunnel's than slender-body theory alone. On the Arcas Robin at
  Mach 2.3, with the centre of mass 12 calibers aft, the body's moment slope about it was
  −31.6 calibers mixed, −25.8 by slender-body theory, and −25.7 by the method with its own
  boattail.
- **A table, built when first needed.** The shares are tabulated every 0.05 in Mach from Mach 5
  (the normal force's limit) down to the lowest Mach at which the method holds, every share is
  positive and every share's centre of pressure lies on its own segment, and interpolated
  linearly. The table is built the first time a flow faster than Mach 1.2 asks for it
  (`OnceLock`, shared by a model's clones), which took 0.3 s in a debug build on the
  development Mac, once per model. Built eagerly, it took the `hpr-aero` unit tests from 4.7 s
  to 238 s. Between rows the interpolation stays within 1e-4 of the method on the Arcas Robin.
- **The join.** From `M_j` = the larger of Mach 1.2 and the table's first row, over 0.3 in Mach:
  each covered component's slope, moment and damping station are slender-body theory's plus
  `w (shock-expansion − slender-body)`, `w = (M − M_j)/0.3` clamped to [0, 1]. Everything is
  linear in Mach, so it is continuous by construction; tests probe the join's ends, table rows,
  points between rows and Mach 4.999 at ±1e-9, on a body joined at 1.2 and on a 20° cone joined
  higher. Mach 1.2 to 1.5 is a judgement: below Mach 1.2 the flow over the nose is transonic,
  which the method doesn't cover, and Mach 1.5 is the lowest Mach at which TN D-4014 measured the
  Arcas Robin and M1.8e1 checked the method.
- **No refusal in flight.** A body the method can't take keeps slender-body theory at every
  Mach. The table is only read inside its rows, so a flight never meets the method's refusals.
- **Stations.** `dynamics.rs` no longer caches body stations; each evaluation asks for them at
  its Mach number, as it already did for fins. Below the join they are the same numbers. A
  covered cylinder's one station now serves its body lift too (it was the planform centroid), a
  compromise the nose already made.

**Result.** The Arcas Robin's nose and cylinder alone (TN D-4014), with the fitted secant-ogive
nose (ADR-033), through the flight's path: the method's slope and CP on table rows (Mach 1.5, 1.8,
2.3) and within 1e-4 between them; against the measured body, short model +16.4% to −18.7%, long
model −13.7% to −26.4%, as in M1.8e1. The committed design, whose power-series nose the method
refuses and which has a boattail, stays at M1.8a's slender-body values, 61% to 82% low. No
validation case flies past Mach 1.06, so the validation report is unchanged.

**Consequences.** `AeroModel::supersonic_body`, `SupersonicBody`, `SUPERSONIC_JOIN_START_MACH`
and `SUPERSONIC_JOIN_WIDTH_MACH` in `hpr-aero`. A rocket with a boattail or a pointed nose the
method refuses gets nothing from M1.8e2. Small changes in geometry can switch a body between the
two models (a step in radius past a millionth of the area, a nose just past Fig. 2, a boattail
appearing), and the join's start snaps to the 0.05 grid; both matter for dispersion and
optimisation studies ([issue #87](https://github.com/nrdptel/hpr-sim/issues/87)). `cargo xtask aero` adds the flight's values to
`validation/fixtures/aero/shock-expansion.json`.

**Update (M1.8e3, 2026-09-19).** The join's start no longer snaps to the grid: where the method
stops holding above Mach 1.2, bisection between the two rows finds that Mach to the last bit of
an `f64` (about 48 halvings, each one run of the method) and the table gains a row there. It has
to be that exact: the shares climb from zero like `√(M − M_start)` (the tip cone's surface flow
turning sonic), so the row's shares are `√δ`-sized for a start off by `δ`. The physics review
measured 24 halvings (`δ` up to 3e-9) on a 20° cone stepped by 2e-9°: the cylinder's row share
jumped 2.2e-5 to 2.5e-4 and the blended slope just above the start by 1.5e-6 per radian, a
sawtooth in shape. At full resolution the row's share is 1e-7 and the slope moves by under 4e-10
per radian. Between that row and the first even one the table is linear where the method rises
like a root: at Mach 1.345955 the cylinder's share reads 30% low (0.203 against 0.291), under
1e-3 per radian after the join's weight (not fixed; recorded). A 20° cone
now joins from Mach 1.341910, not 1.35. The model switches in #87 remain (M1.8e5). M1.8e3 was
split: the boattail became M1.8e4, crossflow and blunt tips M1.8e5, which carries M1.8e's bullet.
