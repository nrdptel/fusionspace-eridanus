# ADR-043: The blunt tip's handover cap: what it is worth, and what stops it moving (2026-09-20)

- **Status:** accepted
- **Summary:** The blunt tip's handover cap: what it is worth, and what stops it moving

**Context.** A blunt or vertical nose tip flies a Newtonian cap (TN D-4865) that hands over to the
second-order shock-expansion method where the surface slope falls to the largest angle a wedge can
turn the flow through with its shock attached, `δ_max` (ADR-038). hpr hands over at the lesser of
`δ_max` and a cap, `blunt_tip::MAX_HANDOVER_RAD`, 24°: the march reads the normal-force slope of
the tangent cone at the handover, and TN 3527's Fig. 2 stopped at 24°. Since ADR-042 those slopes
reach 30°, so the cap became a choice. `δ_max` passes 24° at Mach 2.06 and 30° at Mach 2.52, so a
30° cap keeps the report's own rule over that band instead of cutting it short, and above Mach
2.52 it still starts the march nearer the report's handover.

**Decision.** The cap stays at 24°, and becomes a parameter of the method
(`ShockExpansionBody::with_handover_cap_rad`, bounded by `blunt_tip::CONE_TABLE_CAP_RAD`) so that
the sweep is measured rather than argued. `cargo xtask aero` writes it to `handover_caps` in
`validation/fixtures/aero/blunt-tips.json`, and the guide's
[What the cap is worth](https://nrdptel.github.io/hpr-sim/physics/aero.html#what-the-cap-is-worth)
carries all three of its tables.

- **A steeper cap reads nearer the report's own case.** On TN D-4865's sphere-cone, the body whose
  handover rule this is, fitted as the tunnel measured it, the whole step from 24° to 30° reads
  nearer wherever a cap binds at all: +12.5% to +11.5% at Mach 2.96, +29.7% to +28.0% at Mach
  3.95, +7.5% to +7.1% at Mach 2.3. It is not monotone in the cap: 28° to 30° at Mach 4.63 reads
  further out, +30.9% against +31.3%. Below Mach 2.06 no cap binds and all four readings are the
  same. The Mach 2.3 and 2.96 rows also read a cone slope held at the tables' Mach 3 row, since
  that is where they start, so the band that most motivates the move is the softest evidence for
  it.
- **And it breaks the march on a nose that flattens fast.** On the Arcas Robin's committed
  power-series nose, a 30° cap puts 109 of 160 elements at Mach 4.63 and 145 at Mach 5 into
  TN 3527's `η < 0`, where the exponential law would run away from the tangent cone's pressure and
  hpr reduces the element to the generalized method (issue #81). The answer then follows the
  element count, and not even in order: 3.047 per radian at 10 elements, 2.928 at 40 and 3.260 at
  160, a spread of 0.33 or 11%, where under the flown cap the same nose moves 3.030 to 3.034, a
  part in a thousand. Through Mach 3.96 sixteen
  times the elements move the answer by under 0.01 per radian under the flown cap and under 0.013
  under the 30° one, so this is the top of the range only. At the flown 10 elements the 30° cap
  reduces 5 of them at Mach 4.63 and 9 at Mach 5.
- **The break sits at the flown cap's own edge, and it is not orderly.** It is not a 30° effect:
  at Mach 5 the element count is worth 0.046 per radian at 26° (40 of 160 reduced), 0.69 at 28°
  (140), and 0.055 at 30° (145), against 0.004 at 24° (2). 28° is the worst of the four, so no
  cap between the ends is a middle ground, and none above the flown one holds its answer to Mach
  5. The flown cap has little room to spare either: at the flown 10 elements the same nose first
  reduces an element at Mach 5.6024 (bisected), just past the range hpr's aerodynamics claim.
- **It is the method, not the arithmetic.** Which elements reduce is a decision on the sign of
  `η`, and none sits close enough to zero to turn on rounding: nudging the Mach number by eight of
  its last bits leaves the same elements reduced and the answer within a part in a billion
  (`a_steeper_handover_moves_the_march_out_of_its_range`). What drifts is the loading, not the
  pressure: under the 30° cap at Mach 4.63, `element_flows` gives the nose's last element a
  loading `Λ` (lift per unit length, per radian of angle of attack) of 0.1485 at 40 elements
  against 0.1590 at 80, where its pressure agrees to 0.0005. A reduced element transports `Λ` by
  the `λ₂/λ₁` ratio at each corner instead of relaxing it toward the tangent cone's, and how much
  of each the march does depends on how the nose is cut up.
- **Which way the pressure sits, measured.** Under the 30° cap at Mach 4.63 and 160 elements, 108
  of the nose's elements carry a pressure *above* their own tangent cone's, from 11.9% of the nose
  back, and 108 of those are 108 of the 109 reduced: the steeper start's pressure does not fall as
  fast as the tangent cone's does while the nose flattens, and the gradient behind each corner
  keeps driving it away from that cone. Under the flown cap not one nose element sits above its
  cone. The trouble is not an over-expansion at the handover, which is the first thing the
  geometry suggests.
- **`|η|` as a relaxation rate was tried and rejected.** Reading `η < 0` as a decay toward the
  tangent cone at the rate's magnitude — `ElementFlow::decay_rate` returning `self.eta_rate()
  .abs()` in place of `.max(0.0)`, a one-line change to measure by hand — leaves the blunt case
  just as loose (2.826 per radian at 10 elements against 3.095 at 160 at Mach 4.63) and makes
  TN 3527's own fineness-3 tangent ogive on a 7-calibre cylinder settle more slowly, 2.7865 to
  2.7131 per radian from 10 to 160 elements at Mach 6.28, where the flown reading moves 2.6992 to
  2.6984. That body is not one of the committed rows of `shock-expansion.json`, which step the
  afterbody by twos; the pair is in issue #108 so that M1.8e13 starts from a checkable baseline.
  hpr's reading is the report's own `η = 0` equations, and nothing measured here beats it.
- **Not chosen: raise the cap and disclose.** It would trade a converged answer at Mach 4 to 5 for
  a nearer one at Mach 2.3 to 4, and the guide would have to say the top of the range depends on
  the element count. A model whose answer moves with its mesh is not a model, and nothing forces
  the trade: 24° is a measured position, not a target that cannot be met.

**Consequences.**

- Nothing a rocket flies changes: the default cap is what it was, and no committed fixture moves.
- The cap's justification changes. It was Fig. 2's edge; it is now the march's range, measured
  over the whole sweep and recorded in the fixture, the guide and issue #108.
- Issue #108 holds what would let the cap move: a reading of `η < 0` whose answer stops depending
  on how finely the nose is cut. The vertical-tip switch of issue #87 — a nose steeper than the
  handover all the way to its base gets no method — waits on the same thing, since its edge is the
  cap.
- **The sweep's numbers are stored to six decimals, which loosens their check.** CI's Linux and
  Windows runs read the 30° cap's Mach 4.63 slope at 160 elements as 3.2597630456558506 where this
  machine reads 3.259763045663582 — 7.7e-12 apart, 2.4e-12 relative — against a fixture check that
  allows 1e-12 relative (`designs::same`). That is the reduced march amplifying the last bits of
  `exp` and `powf`, which each platform's library rounds its own way, and it is the number the
  march is least able to promise. So `handover_caps`, and only it, is written through
  `sweep_number`, which rounds to six decimals and refuses a value sitting on the rounding
  boundary. What this gives up is real and worth stating: within that section a later change that
  moves a number by up to about 5e-7 absolute now passes where 1e-12 used to fail. The guide's
  tables quote three decimals, so nothing a reader sees is affected; everything outside the
  section keeps the old check, and a sweep of the `starts` section found its worst 1-ulp-of-Mach
  sensitivity at 3.2e-14 relative, so it needs nothing.
- **The milestone numbers move with the split.** What ADR-040 to ADR-042 call M1.8e12 — moving the
  handover past 24° — is M1.8e13 from here, with its *done when* carried over word for word; the
  step and the flare become M1.8e14. This increment, the measurement, takes the M1.8e12 number.
- The sweep runs four caps over eight Mach numbers at three element counts each time
  `cargo xtask aero` runs, about four seconds of it.
