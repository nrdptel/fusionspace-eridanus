# ADR-049: What a step in radius costs, and why the obvious fix is not taken yet (2026-09-20)

- **Status:** accepted
- **Summary:** What a step in radius costs, and why the obvious fix is not taken yet

**Status:** accepted. **Milestone:** M1.8e15.

**Context.** [Issue #87](https://github.com/nrdptel/hpr-sim/issues/87) collected the places where
the body's supersonic normal force jumps with a small change of shape. The largest of them, and the
last without an owner, was a **step in radius**: a joint where one component's fore radius does not
match the previous one's aft radius. The march cannot cross one — its tangent body needs a profile
without a jump in it — and the model around it then takes the **whole body** off the second-order
shock-expansion method, at every Mach number.

M1.8e15 asks for #87 closed or narrowed to the step alone, with the step's measured size in an ADR
and the guide. It says a step "needs a model of its own rather than a decision about one that
exists". Searching the pinned sources for one came up empty: MIL-HDBK-762 treats a rearward-facing
step only as **base drag** and cites Zukoski on separation ahead of a forward-facing one; NACA
TN 3527, the method's own source, requires a continuous profile; NASA TR-1386 is nose **drag**; and
Washington and Pettis's boattails are continuous. Nothing in `refs/papers/` gives a step's normal
force faster than sound.

**Decision.** Measure it, publish it, and leave the model alone for now.

- **What a step costs is measured and pinned**, by
  `a_step_takes_the_whole_body_off_the_method`, so that a fix can be weighed against a number
  rather than argued about.
- **The step's own force stays slender-body theory's** `(2/A_ref)ΔA` at the joint, the limit of a
  transition whose length goes to zero. That is already an extrapolation — Barrowman 1967 p. 18
  assumes no discontinuities, and the guide says so.
- **Issue #87 is narrowed to the step alone.** Its other switches are owned elsewhere, and the two
  that were not now have issues of their own: [#120](https://github.com/nrdptel/hpr-sim/issues/120)
  (a lip longer than its boattail's drop in diameter) and
  [#121](https://github.com/nrdptel/hpr-sim/issues/121) (a pointed tip steeper than the cone
  tables' 30°). Its body text is rewritten: it said the threshold was the coverage gate's millionth
  of the area, and it is not.

**Measured**, on the tests' straight rocket (a tangent-ogive nose and three tubes, 1.3 m, the
reference pinned at 54 mm) at Mach 3 and 4°. With no step it reads `C_N` = 0.899592 with its centre
of pressure 16.9492 calibres aft of the tip. The reference is pinned on purpose: left on
`ReferenceDiameter::Maximum`, a step **up** widens the reference with it (56 mm at 1 mm, 58 mm at
2 mm), and a ratio of two coefficients taken on two different areas is not a quantity. A first pass
published that ratio; it read the step-up cost as −13% to −18% and the centre of pressure as moving
*forward*, both of which are artifacts of the moving reference. The test now pins the reference.

- **Where the switch sits is a pair, not a number.** At a joint where the radius changes but the
  slope does not, the tangent body merges two elements whose radii agree to a billionth of the
  radius (`shock_expansion::lay_out`) and refuses them past that: **2.7e−11 m** here, bisected, the
  same either way. At a joint where the slope changes too, a step **up** is refused earlier, by the
  requirement that the elements' corners stay in order along the body: 1e−12 × the body's length ×
  the change of slope, which on the tests' finned rocket (1.3 m, a 0.027 → 0.022 m boattail) is
  **1.3e−13 m**, bisected — 208× finer, and a function of the body rather than of the step. Neither
  is the run's coverage gate at a millionth of the fore area (13.5 nm of radius), which is looser
  than both and is what refuses every step anyone could draw; all three give the same reading, and
  the test pins which owns which range.
- **What it costs at the threshold:** −8.6519% of the normal force and 1.0285 calibres of centre of
  pressure, **wherever on the body the step is and whichever way it goes** — at that size the shape
  is flush to a part in 1e9, so the whole difference is the method itself.
- **What it costs as the step grows** — and the two directions part, because a step down takes area
  off the body while a step up adds area that carries slender-body normal force of its own.
  Stepping **down**: at 1 mm, −10.62% and 1.194 calibres at the nose's joint, −10.29% and 0.995 at
  the last; at 2 mm, −12.55% and 1.359 against −11.89% and 0.960. Stepping **up**: at 1 mm, −6.72%
  and 0.867 against −7.05% and 1.064; at 2 mm, −4.75% and 0.706 against −5.41% and 1.099. The
  centre of pressure moves aft in every row.
- **On a boattailed body the switch is larger:** −11.3409% and 1.0951 calibres on the tests' finned
  rocket, at 1.3e−13 m of step up or 2.7e−11 m of step down. That is the commonest high-power shape
  there is, and it is the case a fix most needs to handle.

**Why the obvious fix is not taken.** The obvious fix is to stop the march *at* the step and let
the body ahead of it keep the method, as the run already ends at a flare (ADR-047). It was built in
this milestone's first commit (on its pull request, which is squashed on merge — that commit is
where the three readings below come from and the only place the prototype survives), measured, and
reverted:

- **It does not stay inside ADR-034's objection.** That decision rejected mixing the method's
  shares with slender-body theory's on a *measured* case, a boattail. A step's remainder is
  supposed to be a tube, whose slender-body share is zero — but "the run stopped at a step" does
  not imply "what follows is a tube". On the tests' finned rocket with its boattail's fore radius
  stepped 2.8e−11 m, the mixture reads `C_N` 0.8283554 with its centre of pressure at **16.7209
  calibres — forward of both pure models**, the method's 16.7286 and slender-body theory's 17.8237.
  A reading outside the envelope of the two models it is made of is the pathology ADR-034 measured.
  (Stated as a moment, the mixture crosses the pure method at 17.08 calibres and pure slender-body
  theory at 6.06, and between those two stations it is *less* restoring than either — an earlier
  draft of this ADR had that direction backwards.)
- **It does not close the band it was meant to close.** Matching the coverage gate to `lay_out`'s
  tolerance only lines the two up where the slope does not change. At a joint where it changes the
  corner-ordering bound binds instead, at 1.3e−13 m, so a boattailed rocket comes off the method
  anyway — worth −11.34% and 1.0951 calibres, the same switch `main` has today. The commonest shape
  the fix was meant to help is the one it does not.
- **It cannot tell a step from a shape the method has no reading for** — *as built*. The prototype
  keyed off the joint, not the shape, so any reason the run closed (a *non-conical* flare, a lip out
  of its wake, a flare under `SupersonicFlare::SlenderBody`) kept the forebody marched as soon as
  its fore radius was a picometre off: a **new** switch of +7.2% and 0.69 calibres on an ogive
  flare, where today's behaviour is continuous, and `BodyModel::BEFORE_M1_8E6` would stop
  reproducing earlier results on such a body. This one is a property of the two-line shortcut rather
  than of the idea — keying off which shape closed the run avoids it — so it is recorded as what a
  fix has to get right, not as evidence the idea cannot work. The rejection rests on the two above.

So the fix needs to be built on which *shape* stopped the run, not on whether the joint was flush,
and on what the march itself accepts rather than on a tolerance guessed to match it. That is a
model, not a condition, and it is what the narrowed issue #87 now asks for.

**Consequences.**

- Nothing in `hpr-aero` changed. No committed number moved, and no design in `validation/designs/`
  has a step.
- The guide keeps the step in its table of switches and gains a section saying what it costs, where
  the threshold is and what a fix would have to handle.
- Issue #87 stays open, narrowed to the step, with the three measurements above in it.

**Not chosen: ship the truncating fix anyway and record its faults.** Two of them are *new*
discontinuities larger than ones the same milestone left in the table, and one is a body that gets
no reading at all where today it gets one. A milestone whose subject is a jump in the model should
not add two.

**Not chosen: read a step as a flare of zero length, drawn out to its corner's limit (ADR-047).**
A step is infinitely past that limit whatever its rise, so every step would read the same held
value and a step of a micron would read like a step of a centimetre. ADR-047's drawn-out rule is
itself measured against nothing, and this would lean the whole of a step's force on it.

**Not chosen: model the step's supersonic normal force.** There is no source in hand. What one
would need is a measurement of normal force on a body with a forward- or rearward-facing step at
small angles of attack, which none of the pinned reports provides.
