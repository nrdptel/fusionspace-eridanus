# ADR-044: What the answer follows when it follows the mesh is a crossing of the tangent cone, not a reduced element (2026-09-20)

- **Status:** accepted
- **Summary:** What the answer follows when it follows the mesh is a crossing of the tangent cone, not a reduced element

**Context.** ADR-043 left the blunt tip's handover cap at 24° because a steeper cap puts the
second-order shock-expansion march into readings whose answer follows the element count instead of
settling, and recorded the blocker as issue #108: *a reading of `η < 0` whose answer settles as the
nose is cut finer*. That framing came from the observation that a steep cap reduces most of the
nose to the generalized method (`η < 0`, TN 3527 p. 13, issue #81). M1.8e13 was to move the cap
once such a reading existed. It cannot start there, because the framing is wrong.

**Decision.** Record the measured cause, and re-aim the work at it. The count that says an answer
cannot be trusted is `ShockExpansionBody::tangent_cone_crossings`: how many times the marched
surface pressure passes through its own tangent cone's. `cargo xtask aero` stores it beside every
reading of the cap sweep in `validation/fixtures/aero/blunt-tips.json`, and the guide's
[What a crossing is, and what it costs](https://nrdptel.github.io/hpr-sim/physics/aero.html#what-a-crossing-is-and-what-it-costs)
explains it. Issue #108 is re-scoped to the crossing, and what is left of the old M1.8e13 — the
reading, then the vertical-tip switch of issue #87 — becomes M1.8e16, after the flare and the step,
because it is the only one of the three that is blocked.

- **A crossing is counted within one segment only.** Where a nose meets a cylinder, a boattail or
  a flare, `p_c` itself steps — to the free stream's pressure, to footnote 8's, or to a steeper
  cone's — so the gap can change sign without ever passing through zero and there is no pole.
  Counting those would report a crossing on an ordinary boattailed or flared design, which was the
  first rule's mistake. Within a segment the profile is continuous, so `p_c` is too.
- **A crossing is a pole in the rate, and it separates the sweep's three meshes exactly.** Along an element the
  method relaxes as `e^(−η)` with `η = k (x − x₂)`, the rate per unit length being
  `k = (∂p/∂s)₂ / ((p_c − p₂) cos δ₂)`. A crossing closes `p_c − p₂` while the gradient carries on,
  so `k` runs to infinity. The pressure is unharmed: `k (p_c − p) cos δ₂` is the gradient, which
  stays finite. The loading is not, because eq. 19 relaxes it at the pressure's `k` while its own
  gap `Λ_c − Λ` does not close with it, so at a crossing the loading is driven onto the tangent
  cone's arbitrarily fast. A march applies `k` from one corner across a whole element, so it takes
  one step of whatever size the mesh gives it: at Mach 4.63 under a 30° cap the element at the
  second crossing sheds 98% of the loading's gap in its own length on the 40-element march and 12%
  on the 160-element one. Over the sweep's thirty-two readings — four caps at eight Mach numbers on
  one nose — twenty-seven have no crossing and hold to 0.012 per radian across 10, 40 and 160
  elements; the five that cross move by at least 0.035, up to 0.69. No overlap, and nearly three
  times (2.9×) between the two groups
  (`over_the_sweeps_meshes_a_crossing_separates_the_readings_that_move`).
- **A crossing is a flag, not a verdict, and the ADR claims no more.** It does not prove an answer
  never settles: 28° at Mach 5 crosses at every mesh, and its 0.69 spread is all in the coarse end
  — from 60 elements to 640 it holds to 0.005 per radian, tighter than the worst reading that never
  crosses, where 30° at Mach 4.63 moves by more than 0.2 over the same range
  (`a_crossing_says_the_answer_moved_not_that_it_never_settles`). Nor does a
  zero prove the opposite: 26° at Mach 5 and 28° at Mach 4.63 read zero at the flown 10 elements
  per curve and two at 40 and 160, and both move; the count is not even monotone in the mesh. What
  is claimed is only what the sweep measured: across its three meshes the crossings, and only the
  crossings, mark the readings that move. The count is not consulted during a flight, and the
  sweep is one nose, so another blunt nose above Mach 4 could cross under the flown cap without
  saying so. The guide says all of this.
- **Reduced elements do not move an answer.** TN 3527's fineness-3 tangent ogive reduces 27 of 160
  elements at Mach 5.05 and 50 of 160 at Mach 6.28, and settles to 0.002 per radian from 10
  elements to 160. There `η < 0` comes from the *gradient* changing sign, with the surface pressure
  below its tangent cone's the whole way; the gap never closes, so the rate stays bounded
  (`a_reduced_element_settles_where_tn3527s_own_bodies_never_cross`). That ogive does not cross at
  either Mach number hpr can check it against the report at, which is why the report could state
  its condition (gradient and `p_c − p₂` of one sign, p. 13) and stop: it never had to say what a
  crossing does.
- **Issue #108 was asking half a question.** Its trouble does not begin on the `η < 0` side. On the
  Mach 4.63 march under a 30° cap the step is taken at the *second* crossing by an element that is
  inside the method, not reduced, and how much of the loading's gap it sheds in one step is the
  mesh's answer: 98% on 40 elements against 12% on 160. But the gap it sheds — the loading standing
  about a quarter above its tangent cone's — was opened by the reduced stretch behind it, which is
  hpr's `η = 0` reading and not
  the report's rule. So the two are tangled: a different reading of `η < 0` changes the size of the
  step, and neither can be judged without the other. What is settled is that a rule for `η < 0`
  alone is not obviously enough, which is why `|η|` failed in ADR-043 while leaving the crossing
  where it was.
- **Not chosen: clamp the rate, or the exponent.** Bounding `η`, or the exponent `η Δx` an element
  may apply, would make every answer settle, and would be arbitrary at exactly the point where the
  physics is unknown — the bound, not the method, would then set the loading through the crossing.
  What is missing is a statement about the loading where the pressure gap closes, and a clamp
  hides the question instead of answering it.
- **Not chosen: renumber the milestones.** Making the remainder M1.8e14 would push the flare to
  e15 and the step to e16, and would leave ADR-043's record of the last split describing entries
  that no longer exist. Milestone ids carry one increment level, so the remainder takes the next
  free number, e16, and the roadmap's order does the rest.

**Consequences.**

- Nothing a rocket flies changes. The cap is still 24°, and no committed number moved except the
  new count beside each reading of the sweep.
- `ShockExpansionBody::tangent_cone_crossings` is public, and its documentation says plainly that
  a non-zero count means the answer is the mesh's rather than the model's. `reduced_elements`
  stays, and is no longer the thing to read for that.
- The guide's *What the cap is worth* now carries the crossing count in all three of its tables
  and a section explaining it, so a reader meets the real cause where they meet the numbers.
- **Issue #108 is re-scoped, not closed, and the gap stays visible.** What would close it is a
  reading of the loading through a crossing that settles as the nose is cut finer. Nothing
  measured here is one. The blunt tip's handover past 24°, and issue #87's vertical-tip switch
  with it, wait on that as M1.8e16; the flare (M1.8e14) and the step (M1.8e15) do not, so they go
  first.
- **The counts are integers from a bare sign test, and that is safe here by a wide margin.** The
  sweep's floats are rounded to six decimals because a reduced march is only reproducible to about
  2.4e-12 relative across platforms (ADR-043). A crossing count has no such rounding: it is the
  sign of `p_c − p₂`. A test holds the gap either side of every crossing above 1e-8 of the
  pressure, four orders clear of that drift, so the committed integers do not turn on a last bit
  (`a_crossing_is_a_pole_in_the_rate_the_march_relaxes_at`).
- The sweep costs one more march per cell to count crossings, about a second of `cargo xtask aero`.
