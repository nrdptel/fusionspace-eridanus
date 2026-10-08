# ADR-050: A reduced element is read by the generalized method wherever it has a tangent cone of its own (2026-09-20)

- **Status:** accepted
- **Summary:** A reduced element is read by the generalized method wherever it has a tangent cone of its own

**Status:** accepted. **Milestone:** M1.8e19.

**Context.** Along each element the second-order shock-expansion method relaxes the pressure toward
its tangent cone's as `p = p_c − (p_c − p₂) e^(−η)`, `η = (∂p/∂s)₂ (x − x₂) / ((p_c − p₂) cos δ₂)`
(TN 3527 eqs. 8 and 9). That form is monotone, so it holds only where the gradient behind the
corner points at `p_c`; the report keeps it for `η ≥ 0` (p. 13) and says that at `η = 0` "all
equations reduce to those given by the generalized shock-expansion method". Where `η < 0` hpr takes
`η = 0`, which is issue #81's reading. Until now it took it **only on the nose**: behind the nose
the march refused outright, on the ground that a cylinder, a boattail or a long shallow flare would
otherwise carry its corner's loading over any length.

A conical flare is one element behind the nose, and on a *near-flat* one it is reduced — so the
march refused a run of Mach rows, the table (built downward from Mach 5, needing the join's whole
0.3 Mach inside it) vanished, and a flare of about 0.03816° to 0.05882° on the tests' rocket, a
rise of a third of a millimetre over 0.3 m, took the whole body to slender-body theory at every
Mach number: −8.3% of the normal force and 1.16 calibres at Mach 3 and 4°. Shallower still the
refusal lifted the table's start in steps — −4.6% and 0.75 calibres at 0.00090182° — and not even
monotonically. ADR-047 recorded all of it; issue #117 and M1.8e19 carry it. M1.8e19 asks for the
region's edges **derived** rather than bisected, and then for a rule that carries the reading
across or a demonstration that refusing is right.

**Decision.**

- **The edges are two turns, each the zero of one of the quantities whose signs must agree.**
  `η`'s sign is the sign of `(∂p/∂s)₂` over the sign of `p_c − p₂`. Each of those is a smooth
  function of the turn through the corner, and on every corner state measured here each has a
  single zero, so an element is reduced on exactly the open interval between them:

  - the **crossing**, where `p₂(θ) = p_c(δ₁ + θ)` — the compression lands the pressure on its
    tangent cone's, and `η` has a pole;
  - the **balance**, where `(∂p/∂s)₂ = 0` — eq. 4 rearranged to
    `sin(δ₁ + θ) = (Ω₁/Ω₂(θ)) (sin δ₁ + r (∂p/∂s)₁ / B₁)` — the corner's own compression cancels
    the gradient the body ahead delivers, and `η` is zero.

  *A single zero* is not a promise, and a corner that breaks it exists: a 25° cone with 20 mm of
  tube behind it at Mach 7 meets its tangent cone's three times (about 0.91°, 7.3° and 24°), so a
  flare there is reduced on **two** bands. `flare_reduction_turns_rad` therefore sweeps the
  widening turns at a hundred stations before it brackets, and reports a corner with any number of
  crossings but one as an error rather than returning whichever root it reached
  (`a_corner_whose_gap_has_three_zeros_is_refused_rather_than_guessed_at`). A pair of roots inside
  one station would still be missed. Which of the two is the shallower is not fixed either — on the
  tests' rocket the crossing is below the balance from about Mach 1.5 up and above it below that,
  where both turns are shallower than the element-merging floor anyway, so no corner is drawn.

  Both are functions of the corner's own state alone: the Mach number, pressure, gradient, radius
  and angle the body hands to it, plus the free stream it was read in.
  `ShockExpansionBody::aft_flow` now reports all six, and `flare_reduction_turns_rad` takes just
  that and solves the two equations — a contraction on `Ω₁/Ω₂` for the balance, false position
  over the turns a widening corner can make for the crossing, started from
  `θ ≈ (1/p₁ − 1)√(M₁² − 1)/(γM₁²)`. Neither root is promised: each is returned with the residual
  it left, and a body whose cone flow is harder leaves more (4e-9 of the free stream's pressure on
  a fatter body at Mach 2.6). Nothing bisects the model's refusal, which is the search the
  milestone asked to be rid of; bracketing an explicit residual is not that.

  On the tests' flared rocket the derivation reproduces all three numbers issue #117 bisected: the
  band's lower edge is the **crossing at Mach 4.70**, 0.038161270°; its upper edge is the
  **balance at Mach 5**, 0.058820517°; and the shallowest angle that lifted the join's start is
  the **crossing at Mach 2.20**, 0.000901825°. The non-monotone part is explained too: the refused
  rows at a given angle are the Mach numbers between the two turns' inverses, an interval that
  narrows as the flare flattens until it holds no 0.05-Mach row at all.

  **What limits the digits is now stated, not observed.** Where the tangent cone is slender-cone
  theory's closed form (under `SLENDER_CONE_RAD`, 0.029°) the crossing closes to the last bits of
  an `f64` — the residual is under 2e-14 of the free stream's pressure at Mach 2.0, 2.2 and 3.0.
  Above it the cone flow is a Taylor–Maccoll integration, blended with the closed form up to
  0.0573°, and the residual is that integration's, about 1e-10 (Mach 4.3 to 5). Divided by the
  gap's slope in the turn that is about 2e-10°, which is the 2.6e-10° the three CI platforms were
  seen to spread the band's lower edge over. The spread was the cone's, not the search's, and the
  two edges past Mach 4 are pinned to 2e-9° rather than tighter for that reason.

- **The reading is carried across, by TN 3527's own reduction, wherever the element has a tangent
  cone of its own.** The refusal keeps only its first clause: an element whose angle is under
  `CONE_ANGLE_FLOOR_RAD` still refuses. That is issue #123.

  **What that line is and is not.** It is *not* that a widening element relaxes toward something
  and a cylinder does not: a reduced element does not relax at all. `decay_rate` returns zero, so
  in `ElementFlow::at` the decay is 1, `p_c` and `Λ_c` are multiplied by nothing, and the element
  holds `p₂` and `Λ₂` over its whole length — on a flare exactly as on a cylinder. What differs is
  what the element is being read *against*. On a widening element `p_c` and `Λ_c` are a real
  cone's at the flow's own Mach number, so the two branches either side of the region are two
  readings of one picture and they meet where `η = 0`. A cylinder's `Λ_c = tan δ (dC_N/dα)_tc` is
  identically zero and its `p_c` is the free stream's, so there is no cone there to meet; a
  boattail's is footnote 8's `p_c = p₀`, `(dC_N/dα)_tc = 2`, a stand-in rather than a solution of
  that element's flow.

  **And the length the old refusal worried about is real, admitted and measured.** Because a
  reduced element's reading does not depend on its length, what it costs grows with it. On the
  body alone, either side of its own crossing at Mach 5, the tests' rocket's 0.3 m flare moves
  `C_Nα` by +0.40% and its centre of pressure by 0.064 calibres, and the same body with a **2 m**
  flare by +2.61% and 0.761 calibres. That is the failure the blanket refusal existed to prevent,
  now taken on the flare with its size on the page rather than avoided by throwing the body off
  the method. Nothing measures it on a cylinder or a boattail, which is why those keep the
  refusal, and #123 is what would close that.

  **The trade is not one-way, and that is the decision.** On a body whose reduced rows sit at the
  bottom of the table, the old refusal truncated the table there, so the join carried the reading
  up from slender-body theory continuously and the pole was outside it; reading those rows puts
  the pole inside the table, where it is a step between two neighbouring Mach rows. What is given
  up on such a body is a continuous join; what is bought is that an arbitrarily small change of
  shape no longer moves the whole body between two models. The refusal is taken to be **not**
  right because the reason published for it does not hold — a reduced element behaves the same
  whatever it sits on — and because refusing one whole side of a pole is not the same as having no
  reading there: TN 3527 names `η = 0` as what its equations become, and both sides of the pole
  are then readings of the method. What the method does not say is what the loading does through
  the pole itself, and that is issue #108, unchanged by this either way.

  The measurement says the rest plainly. With the reading
  carried, the table starts at Mach 1.2 at fifteen flare angles from zero to a degree, and swept
  in sixteen steps from a cylinder to 0.08° — well past the region's steep end — the whole
  rocket's normal force falls all the way at both Mach 3 and Mach 4, with no neighbouring pair
  moving it by a fifth of a percent, bar the single pair that straddles that Mach number's
  crossing. The old switch moved it by 8.3% in one step. Both switches are gone
  (`a_near_flat_flare_reads_through_and_leaves_only_the_corners_crossing`), and their sizes are
  still measured, because they are the size of the fallback the model dropped to:
  `SupersonicFlare::SlenderBody` on the same rocket still reads −8.30% and 1.1574 calibres at
  0.058820517° and Mach 3, and −4.62% and 0.7522 calibres at 0.00090182° and Mach 2
  (`a_near_flat_flare_marches_every_row_and_the_fallback_is_still_measured`).

- **What is left is the crossing itself, and it is reported, not hidden.** At the crossing `η` has
  a pole. The pressure rides through — the gap it multiplies is zero there — but the loading does
  not: on the side the method still owns, `η → +∞` sheds the corner's loading onto the tangent
  cone's within the element's own length; on the reduced side it holds the corner's. (Which side
  is which follows the two turns' order, which is not fixed.) Measured on the whole
  rocket at 4° with a ±1e-9° probe either side of that Mach number's own crossing:

  | Mach | normal force | centre of pressure |
  |---|---|---|
  | 2.00 | +0.00032% | −0.0000016 calibres |
  | 3.00 | +0.011% | +0.00018 calibres |
  | 4.00 | +0.055% | +0.0017 calibres |
  | 4.95 | +0.129% | +0.0051 calibres |

  Widening the probe a hundredfold, to ±1e-7°, leaves the figure where it is at Mach 3, 4 and
  4.95, which is what says it is a step and not the reading's ordinary movement. **At Mach 2 it
  does not**: +0.00032% is already about what the reading moves over a ±1e-7° probe there, so that
  row is an upper bound on the step rather than a measurement of one, and the test only pins the
  ±1e-9° figure for it.

  **That is one rocket, and it is not a bound.** The step is the loading's gap at the pole, so it
  grows with the length of the element holding it, and where the region sits depends on the body
  ahead. On the **body alone** at Mach 5, either side of its own crossing: the tests' rocket's
  body +0.40% and 0.064 calibres; the same with a 2 m flare +2.61% and 0.761; the same with the
  tube cut to 0.1 m +4.34% and 0.186, its crossing at **1.388°**; a 10° cone with 0.3 m of tube
  +3.82% and 0.251, crossing at 0.696°. With a short shoulder the region is not near-flat at all —
  0.7° to 4.6°, where real flares live.

  **There is a bound, though, and it is exact rather than sampled.** The two sides of the pole
  take the two constants eq. 19 relaxes between — `Λ_c = tan δ₂ (dC_N/dα)_tc` on the side the
  method owns, `Λ₂ = (λ₂/λ₁) Λ₁` on the reduced side — and along a conical flare both are
  constant, so eq. 19's `C_Nα = (2π/A_ref) ∫ Λ r dx` integrates a constant:

  `ΔC_Nα = (2π/A_ref) (Λ₂ − Λ_c) · ½(r_fore + r_aft) · L`

  `flare_reduction_turns_rad` returns `Λ₂ − Λ_c` beside the turns as
  `crossing_loading_gap_per_rad` (6.116195e-4 per radian on the tests' rocket's body at Mach 5),
  and the formula reproduces the measured step to a part in 1e5 at flare lengths of 0.3, 1, 2 and
  5 m. So the numbers above are an illustration and the formula is the claim, which a reader
  evaluates for their own flare in one line.

  **And the pole is crossed in Mach as well as in shape.** On that short-shouldered body with a 1°
  flare the table's rows step **−2.77% and 0.14 calibres** from Mach 2.90 to 2.95, against a fifth
  of that either side. Under the old rule those rows were refused, so the table started above them
  and the join covered the pole; it is now inside the table.
  `what_the_crossing_costs_is_the_loading_gap_times_the_element_that_holds_it` pins all of it.

  The whole-rocket worst above is a 64th of the switch it replaces in the force and a 227th of it
  in the centre of pressure — the step at Mach 4.95 against the switch measured at Mach 3, so a
  comparison of sizes rather than of one flight condition; the body-alone numbers are not that
  small. It is not a new question, though, but ADR-044's: the loading through a tangent-cone
  crossing, open as issue #108. (That issue had been closed by mistake
  while ADR-044 was being written, which said in terms that it was re-scoped and *not* closed; it
  is reopened.) The region's other edge, the balance, leaves nothing at all —
  `η` is zero there, so the two readings coincide by construction, and a ±1e-9° probe moves the
  rocket's force and station by under 1e-7.

**Not chosen: read `η < 0` as `η = +∞` instead, putting the element on its tangent cone.** That is
continuous at the crossing — which is exactly where the pressure already sits on the cone — but it
tears the other edge open: at the balance the accepted branch is the `η = 0` reading, so the whole
gap `p_c − p₂` and the loading with it would step there, and that gap is far larger than the
crossing's (1.7e-3 of the free stream's pressure at Mach 5 against nothing). The two edges demand
opposite branches, and `η = 0` is the one the report names.

**Not chosen: blend the two branches across the band.** A weight running from the tangent cone's
loading at the crossing to the corner's at the balance would be continuous at both. It is also a
model with no source: nothing sets the weight, and the band is exactly where the method has no
statement to make. ADR-047 rejected a fade at a flare's detachment boundary for the same reason.

**Not chosen: let the table skip a refused row.** Building the table from the rows that hold and
interpolating over the ones that do not would have removed both switches without touching the
march — and would have published readings at Mach numbers where the method returns nothing.

**Consequences.**

- A flared rocket's reading is continuous and monotone in the flare's angle across the whole
  near-flat region, at every Mach number the table covers. No committed flight number moves: no
  validation case has a flare.
- `AftFlow` gains `pressure_ratio`, `gradient_p0_per_m`, `radius_m`, `loading_per_rad` and
  `free_stream_mach`, which with the two it had make it the corner's whole state — so `flare_reduction_turns_rad` takes it
  and nothing else, and the two halves of one state cannot be passed in disagreeing with each
  other. It and `ReductionTurns` are public, and the
  guide's *A near-flat flare* explains them with the table above.
- `ShockExpansionBody::reduced_elements` now counts reduced elements anywhere on the body, not
  only on the nose, and `slope`'s error list says the refusal is a cylinder's or a boattail's.
- `validation/fixtures/aero/blunt-tips.json` changes in six strings only: the refusal TN D-4865's
  own Newtonian start hits on the Arcas Robin from Mach 3.96 is a **cylinder's** element, so it
  still refuses and ADR-038's evidence stands; the message names the cause more exactly.
- Issue #117 is closed. Issue #123 is opened for the cylinder and the boattail. Issue #81 is
  unchanged: what the report would have done where `η < 0` is still unknown, and this ADR widens
  where hpr's own reading of it is used rather than narrowing it.
- ADR-047's two tests are replaced. `the_march_refuses_two_bands_of_flare_and_the_model_keeps_slender_body_theory`
  and `a_near_flat_flare_lifts_the_joins_start_in_steps` became
  `a_near_flat_flare_marches_every_row_and_the_fallback_is_still_measured` and
  `a_near_flat_flare_reads_through_and_leaves_only_the_corners_crossing`; three more were added —
  `the_turns_a_reduced_element_lies_between_come_from_the_corners_own_state` for the derivation,
  `what_the_crossing_costs_is_the_loading_gap_times_the_element_that_holds_it` for the sizes, and
  `a_corner_whose_gap_has_three_zeros_is_refused_rather_than_guessed_at` for the counter-example.
  A fourth, `a_corner_behind_a_relaxed_body_still_has_both_turns`, guards a body that hands the
  free stream's own pressure to the corner, where the crossing is a turn of nothing and rounding
  it through the isentropic relations would have decided the answer on one bit. The low-Mach band
  ADR-047 also described is untouched and still pinned by
  `where_the_corners_turn_runs_out_the_join_carries_the_reading`.
