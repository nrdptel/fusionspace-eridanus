# ADR-045: Where a flare's march stops is the corner's isentropic turn, not the shock detaching (2026-09-20)

- **Status:** accepted
- **Summary:** Where a flare's march stops is the corner's isentropic turn, not the shock detaching

**Status:** accepted. **Milestone:** M1.8e14, which is the first of the three the old M1.8e14 splits into.

**Context.** M1.8e14 is to fly a flare through the second-order shock-expansion method, which
already marches one: a 2.75° cone, a tube and a conical flare return a normal-force slope at Mach 3
(6.55 per radian on the body's cross-section for an 18.5° flare). What stops a flared rocket today
is the model around the method, which ends its run at the first widening body. The milestone's
*done when* asks for a flared body that flies the method **where the flare's shock is attached**,
with no jump across that boundary, so the first question is where that boundary is and what the
march itself does on either side of it. NASA TN D-4865 tested exactly this shape and says its
Mach 1.50 run is past attachment even in theory.

**Decision.** Measure the edge before moving it. M1.8e14 bisects, to f64 resolution, the steepest
flare the method marches over Mach, on a pointed 2.75° cone with five calibres of tube and a
conical flare. Both angles are TN D-4865 model 2's; the layout is not — that model is blunt-nosed
with no tube — and the edge depends on the body ahead of the flare, which is what sets the flow
reaching it. Two tests in `crates/hpr-aero/src/shock_expansion.rs` pin the edge and what makes it:
`a_flare_marches_to_the_isentropic_turn_not_to_detachment` and
`past_mach_2_13_the_flare_stops_where_the_cone_tables_do`.

**What the edge is made of.** Two different things, one below Mach 2.13 and one above.

- **The corner's isentropic turn.** Second-order shock-expansion fixes the pressure just behind a
  corner from the Prandtl and Meyer turn there (TN 3527 pp. 7-8, the first of the three conditions
  on eq. 3; `ν` itself is NACA 1135 eq. 171c), so the march stops where the turn would take the
  flow reaching the flare to Mach 1: not where an oblique shock would detach. The two are different
  angles, and neither bounds the other.

  | free stream | the method marches to | a wedge's shock detaches at (NACA 1135) | the method is |
  |---|---|---|---|
  | Mach 1.5 | 11.9312175° | 12.1126689° | 0.1814514° short |
  | Mach 2 | 26.4714031° | 22.9735318° | 3.4978713° past |

  They cross at **Mach 1.547787962528**, both at 13.346819°, bisected to f64 resolution. Below it the method refuses
  flares whose shock is attached; above it the method answers for flares whose shock is not.
- **The cone tables' 30°.** From **Mach 2.129702032593** up, the limit is not the flow at all: the
  tangent cone at each element is looked up in NASA SP-3007 Table 2, which stops at 30°
  (ADR-042), so every Mach number above that
  gives the same edge, 30° plus the millionth of a degree `cone_normal_force_slope` admits for the
  degree conversion's rounding. At Mach 2.5 the flow could turn 29.8° before its shock detached and
  far more before it went sonic; the reference data is what runs out.

**What happens where the shock is detached.** The march keeps marching, and returns a number. It
is a number from an isentropic compression through a corner that a real flow crosses through a
detached bow shock standing ahead of the juncture, with a subsonic pocket behind it, so it is
wrong in a way the march cannot report. Taking a wedge's limit as the stand-in, the band on the
body above is narrow but real for an 18.5° flare, the report's angle: the method answers from
**Mach 1.721760**, and a wedge's shock reaches 18.5° only at **Mach 1.767575**. TN D-4865's Mach
1.50 run is below both, where the method refuses outright.

**Consequences.**

- **M1.8e17 cannot use the march's refusal as its attachment test.** "Where the flare's shock is
  attached" has to be decided by a detachment criterion of its own, evaluated on the flow reaching
  the flare, and the method's own edge sits on both sides of it. That is now what M1.8e17 starts
  from.
- **The criterion itself is still open, and M1.8e17 owns it.** The wedge's largest deflection is a
  conservative stand-in here, not the answer: a cone's shock stays attached to angles a wedge's
  cannot hold (NACA 1135), and a conical flare on a cylinder sits between the two. Every number
  above is stated against the wedge's limit and says so; none of them is claimed as the flare's own
  attachment boundary.
- **Nothing a rocket flies changes.** The run still stops at the first widening body, so no
  committed number moves. The tests are new, and they are the baseline M1.8e17 has to keep.
- **The 30° cap is a limit of the reference data, and the guide says so where it appears.** It is
  the same cap that bounds a pointed tip (ADR-042),
  met from a second direction.

- **The two halves left over take the next free numbers, e17 and e18, and go straight after
  e14.** A milestone id carries one increment level, so `M1.8e14a` is not an id the roadmap's own
  check accepts (ADR-043 settled the same point for the e12/e13 split). The old e14's remaining
  clauses are carried over word for word into M1.8e17 and M1.8e18, and both sit immediately after
  M1.8e14 in the list, which is the execution order: the flare's three pieces stay contiguous and
  nothing that was ahead of them moves behind them.

**Not chosen: let the flare into the run now and measure afterwards.** The boundary is the thing
the *done when* is about; opening the run first would have meant choosing an attachment test with
nothing measured to choose it against, and the two edges above show how easily that choice goes
wrong in both directions.
