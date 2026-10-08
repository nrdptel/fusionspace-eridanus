# ADR-013: Streamer and tumble drag (2026-09-17)

- **Status:** accepted
- **Summary:** Streamer and tumble drag

**Context.** M1.7b needs a cited drag model for a streamer and for a body tumbling with nothing
deployed. Knacke's manual, which M1.7a leaned on, has **no streamer data at all** (checked: the
type tables 5-1 to 5-5, the measured-drag section 5.2.3, the miscellaneous-decelerator section
5.8.4 and the contents) and no bluff-body crossflow table. What is in hand
(`docs/research/streamer-and-tumble-drag.md`):

- The **OpenRocket technical documentation v13.05** (CC BY-SA, a published document, pinned):
  Appendix C fits a streamer drag coefficient to its own wind-tunnel tests, and §3.5 fits a
  tumbling model to 22 m drop tests. Its Java source is GPL and is never read.
- **Carruthers and Filippone (2005)**, peer-reviewed wind-tunnel measurements of streamers and
  flags, whose AIAA copy is paywalled and whose author post-print states no licence.
- **Kidwell's NARAM-43 drop tests (2001)**, OpenRocket's own reference for its appendix, with per
  streamer masses and measured descent rates. No licence stated.
- Loft's L29 lesson: its recovery defaults came from OpenRocket's **source**, which must not be
  ported.

**Decision.**

- **A streamer is a drag area from a correlation on its planform area `S = l w` and aspect ratio
  `AR = l/w`, and hpr carries both correlations** (`StreamerModel`), because they disagree by a
  factor that runs from 1.9 (at 80 g/m²) to 5.8 (at 10 g/m²) and only one of them survives a
  comparison with a free drop.
  - **The default is Carruthers and Filippone's**, all three of its printed curves:
    `0.561 AR^−0.480` at `S = 0.025 m²` (eq. 2), `0.6514 AR^−0.6075` at 0.05 m² (the trend line on
    Figure 3, which the text does not repeat) and `0.405 AR^−0.494` at 0.075 m² (eq. 1). hpr
    interpolates between neighbours linearly in `ln S` and holds the end curve outside; that is
    hpr's choice, documented as such. Blending only the two extremes, as the first draft did,
    reads 18% below the paper's own middle curve at `AR = 3.3` (found in review).
  - **`StreamerModel::OpenRocket`** is appendix C's `C_Dm = 0.034 ((ρ_m + 25)/105)((l + 1)/l)`,
    kept for comparing with OpenRocket in M2.2. It is the only one of the two that uses the
    material.
  - **The measurement that decides it.** Recomputed here from Kidwell's Table 1 and his results,
    with his normalisation to a notional 5 g weight and his distance-over-time rates compared
    against the same average from the closed-form fall (both corrected in review): his crêpe
    streamer, the one he left unpleated, descends at 2.80 m/s, a `C_D` of 0.155 on its planform.
    Filippone's correlation gives 3.05 m/s (+9%); appendix C gives 5.28 m/s (+88%). Kidwell's
    pleated streamers descend slower still (Micafilm at 2.04 m/s, `C_D` 0.338), which neither
    model reaches.
  - **Above the largest fitted area hpr holds the end curve rather than extrapolating.** The
    paper's trend would give a lower `C_D` (about `S^−0.3`), but the only free-drop measurement in
    hand is higher than either, so the clamp is the closer of the two. `recovery.md` states both.
  - **Pleats are not modelled**, so hpr predicts a faster descent for a folded streamer. That is
    the safe direction for a landing, and it is stated in `docs/physics/recovery.md`.
  - hpr's streamer descent rates will therefore differ from OpenRocket's by 1.4 to 2.4 times,
    depending on the fabric (the drag-area ratio runs 1.9 to 5.8). M2.2 will see that; it is the
    intended difference, not a defect.
- **Tumble is the technical documentation's §3.5 model**, `C_D S = 1.42 A_f + 0.56 A_bt`, computed
  from the airframe by `DeviceDrag::tumbling`: `A_bt` by integrating the outer diameter along the
  axis (each body component's mean diameter times its length), `A_f` as one fin's planform area
  times Table 3.4's efficiency factor for the fin count. More than eight fins is refused, because
  the table stops there.
  - Its constants come from Hoerner's *Fluid-Dynamic Drag*, which is copyrighted with no legal
    free copy. hpr cites the documentation, and the research note records NASA TN D-540 and
    TR R-474, which are free and carry the same numbers, for when a pinned source is needed.
  - **hpr states what it can demonstrate, not the documentation's accuracy claim.** Replaying its
    own Table 3.3 through hpr's reading of the model gives −5.8%, −5.4%, −7.2%, +19.0% and −10.0%
    on the five drop-test models, not the 3 to 14% it claims: the finless tube wants a body
    coefficient near 0.79 where the model prints 0.56, and the text pins neither area convention.
    The spread is a test (`the_tumble_model_against_its_own_drop_tests`) and is what the docs
    quote (found in review: the claim had been repeated without checking it).
  - **The fit is for small models** (44 to 103 mm, 6.8 to 160 g, 5.0 to 6.6 m/s). A high-power
    booster is outside it, and above `Re ≈ 2e5` a cylinder's crossflow drag falls by about half,
    so hpr will read slow there. The limit is documented rather than extrapolated.
- **A tumbling body is a device with a trigger, like a canopy.** hpr does not decide by itself
  when a rocket tumbles: no source in hand says when a stage becomes unstable enough, and the same
  documentation declines to model the analogous twirling streamer regime.
- **Both new sources are pinned and cite-only** (`validation/refs.lock.toml`,
  `THIRD-PARTY-NOTICES.md`): the Filippone post-print and Kidwell's report state no licence, so
  their numbers are used and their text is never copied or redistributed.
- **M1.7 is split again.** M1.7b is streamers and tumble, with the first of M1.7's remaining
  *done when* bullets; M1.7c is separated bodies, with the second. Separation needs its own
  decision about how a body's mass properties and drag are defined, and `Assembly` has no split.

**Consequences.**

- M1.7c flies separated bodies. Since the descent phase already drops airframe aerodynamics, a
  separated body needs mass properties and its own device, not an aerodynamic model — which is
  the cheapest honest way in.
- M2.2's OpenRocket comparison should compare streamers under both models, and report the gap
  rather than tune either.
- If a streamer model is ever fitted to more drop data, `StreamerModel` is the place for it; the
  research note lists what would be needed.
