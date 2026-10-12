# ADR-244: The flutter guide: the guide rocket's fins read two ways, and the halved constant marked (2026-10-11)

- **Status:** accepted; carries out [ADR-195](0195-the-2026-10-07-guides-rocketry-explained.md) §6 and §7 for its fin-flutter guide, on [ADR-242](0242-the-guides-format-and-the-trust-guide.md)'s format; closes M0.7a4
- **Summary:** The third guide, *When fins flutter*, explains what flutter is, what Martin's criterion's flutter speed depends on, and what its two readings (the ratio of the flutter speed to the airspeed at max q, and Martin's own chart) say about the guides' rocket. Its numbers come from a new report, `guide-rocket-flutter.json`, which `cargo xtask guides` writes from the design and its flights through `hpr sim --json` and fails when stale. The figure draws the rocket's flutter speed against its fins' thickness, with Martin's band shaded and the halved constant of *Peak of Flight* #291 and #411 marked in the caution color: a flutter speed that reads high, tied to a new test in `hpr_sim::flutter` of that direction.

**Context.** M0.7a4's *done when* is M0.7a2's: the guide meets the format, ADR-195 §7 and M0.7a's
terms on numbers and marks, is read cold in a docs review, and passes M0.7a1's checks. No
committed report held a rocket's flutter numbers; the model page's worked example prints from an
example program the guides' checker can't read. Nothing measured checks HPR Sim's flutter speed on
a hobby fin, so, unlike the stability guide, there are no HPR Sim errors to mark. The flattering
error ADR-195's context names is in print: the form most often copied halves Martin's constant.
That claim came from a forum page; for this guide the issues themselves were read: *Peak of
Flight* #291 (2011, eq. 1) and #411 (2016, eq. 1) carry the halved constant, and #615 (2023,
pp. 2 and 5) corrects it. The research note had named #416 for the 2016 article; it is #411.

**Decision.**

1. **A report the guide owns.** `validation/reports/guide-rocket-flutter.json`: the guides'
   rocket's fin panel (`FlutterPanel::of_fins`), the built-in birch plywood's shear modulus with
   its source, the flutter speed and Martin's figure 3 ratio in the launch site's standard air,
   each configuration flown by `hpr sim --json` in process with its peak dynamic pressure and
   `√(q_f / q)`, the halved constant's speed, and the flutter speed through fin thicknesses of 1
   to 8 mm at the same outline. `cargo xtask guides` writes it; `--check` and the guides' test fail
   a committed copy whose numbers sit more than 1e-9 of their size from what the design gives now.
   The tolerance is relative, not ADR-243's absolute one, since a flight's numbers (a dynamic
   pressure near 5 × 10⁴ Pa) carry the platforms' last bits at their own scale.
2. **The fins' modulus is the built-in material's, said so.** The design's birch plywood is
   OpenRocket's, at OpenRocket's density, which `materials::shear_modulus_of` doesn't match to a
   built-in material. The guide takes the built-in birch plywood's cited 0.75 GPa and says why.
3. **Both readings, no verdict.** The guide quotes the ratio `V_f / V` at the faster motor's max q
   and the fins' figure 3 ratio against the band, and the thickness each would need, rounded up to
   a tenth of a millimeter so the page never asks for less than the criterion does. It says that
   Martin's unfailed wings flew to at least Mach 1.3, so his chart shows nothing either way about a
   fin above the band on a slower rocket, and that neither reading is a line to fly by.
4. **The flattering mark is the halved constant.** The figure draws HPR Sim's flutter speed and,
   above it in the caution color, the halved constant's, with Martin's band (the measured spread of
   his line, 0.25 to 0.31) shaded as the uncertainty. `hpr_sim::flutter`'s new
   `the_halved_constant_reads_a_higher_flutter_speed` pins that the halved form reads `√2` above
   eq. 18 on any panel in any air, and `V_f / V` higher by the same factor; a halved constant in
   the model fails it. The guide's `the_flattering_line_is_the_halved_constant` pins that the
   caution line is that speed and the only caution stroke. The guide names the first.
5. **HPR Sim's own readings, in words.** Where Martin leaves a choice, HPR Sim takes the lower
   flutter speed, with two exceptions (an airfoiled fin's modulus, a booster's max q); and what the
   criterion leaves out, the fin's attachment above all, may flatter more. The guide says so and
   links the model page's readings, since no measurement sizes them.
6. **A direction test's name holds no digit.** The guides' checker reads inline code with a
   number as a number, so the guide names a test without one; the new test is also listed on the
   model page.

**Alternatives considered.**

- **Mark HPR Sim's own errors, as the stability guide does.** Rejected: nothing measured checks
  HPR Sim's flutter speed on a hobby fin, so no mark would rest on a comparison.
- **Quote the example program's table** (`fin_flutter`, five materials on a synthetic rocket).
  Rejected: its output isn't a report the guides' checker reads, and the guides use one rocket.
- **Name the 2016 issue the forum's page names, unread.** Rejected: the research note's
  citation was wrong; the guide cites only issues read.
- **Name the motor in the prose.** Rejected: a designation such as `H170M-10` reads as a label to
  the site's checks and holds a number no quote can cover; the guide says "the faster of its
  motors" and the report records the label.

**Consequences.**

- A change to `hpr_sim::flutter`, the plywood's modulus, the design or the flight that moves a
  number fails `cargo test -p xtask` until `cargo xtask guides` rewrites the report and the guide
  follows.
- By Martin's criterion the guides' rocket's 3 mm plywood fins fly past their flutter speed on
  either motor. The guide states it as the criterion's reading, not a verdict; the `hpr` command
  prints no flutter speed yet.
