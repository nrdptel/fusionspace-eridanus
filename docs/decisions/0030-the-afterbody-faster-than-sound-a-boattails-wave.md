# ADR-030: The afterbody faster than sound: a boattail's wave drag, the base behind it, and a lip in its wake (2026-09-18)

- **Status:** accepted
- **Summary:** The afterbody faster than sound: a boattail's wave drag, the base behind it, and a lip in its wake

**Context.** ADR-029 left Calisto's drag −29.8% to −24.4% under RASAero II from Mach 1.2 and
named a boattail's supersonic wave drag as a candidate, with the Arcas Robin's measured 15°
boattail arguing against taking MIL-HDBK-762's chart whole. M1.8b3's targets, set before
measuring (`ROADMAP.md`): the Arcas Robin's 11 fins-off rows from Mach 1.5 within 10%, the 2
already within staying; Calisto's 17 supersonic rows within 10%; each row's change reported.
hpr's boattail was Niskanen's rule (eq. 3.88) at every speed: a share of the base drag on the area
removed, 0 for boattails longer than three times their drop in diameter.

What the sources give (all NACA and NASA reports are U.S. Government works; pinned in
`validation/refs.lock.toml`):

- **Fig. 5-122's source.** MIL-HDBK-762 cites none. Jack, NACA TN 2972 (1953), computed conical
  boattails by Van Dyke's second-order theory at Mach 1.5 to 4.5, 3° to 11°, area ratio 0.2 to
  0.8. The chart's axes are the small-disturbance similarity variables, and it agrees with Jack's
  points within −10.4% to +8.0% for area ratios up to 0.6 (83 points inside it); first-order
  theory reads far higher. So the chart is second-order theory, not measurement.
- **Measured boattails, jet off, turbulent**: Cortright and Schroeder (RM E51F26, Mach 1.91,
  5.6° to 9.3°, boattail drag and base pressure; "the method of characteristics overestimated the
  side pressure drag by about 18 to 20 percent", p. 17), de Moraes and Nowitzky (RM L54C16, Mach
  1.59, 5° and 10°), Compton (TN D-6789, 3°, 5° and 10° at Mach 0.3 to 2.20, the points to 1.3
  from his NASA TM X-1960 in another tunnel; those near Mach 1 his report calls "questionable",
  p. 9), Moskowitz and Jack (RM E54B11, Mach 3.12, 7.1°), Love (TN 3819, Kurzweg's base pressures
  at Mach 3.24, 2.5° to 15°), and Cubbage (RM L57B21, Mach 0.6 to 1.28, 5.6° to 45°, a boundary
  layer 0.20 d thick). Cubbage's boattails stay attached at 16° and "between boattail angles of
  16° and 30°, the external flow separates completely" (p. 8); a separated boattail's pressure is
  "approximately equal to the pressure measured at the base of a cylindrical model" (p. 6).
  Measured transonic boattail drag is half-way up its rise by about Mach 0.89 (Compton's 10°) to
  0.92–0.96 (Cubbage's), peaks at Mach 1.0 to 1.1, and at 1.2 is 0.83 to 0.90 of that peak
  (Cubbage).
- **The base behind a boattail**: MIL-HDBK-762 Fig. 5-141 (p. 5-210, after Rubin, Brazzel and
  Henderson 1970, Mach 2.5 to 3.5), `p_cyl/p_bt = 0.442 + 0.558 a_b`, with Love's cylinder
  correlation (Fig. 5-139, p. 5-208). Used as a pressure ratio at Mach 1.59 and 1.91 it
  over-predicts the measured relief.
- **Through Mach 1** the handbook finds no method and advises holding the supersonic value to a
  peak between Mach 1.0 and 1.2, "with a sharp reduction to a lower value at subsonic speeds"
  (p. 5-47).
- **The lip**: TN D-4014 (p. 6) finds the chamber force low at Mach 1.50 and 1.80 "particularly for
  the fin-off condition. This is believed to be because of the reflex lip at the model base; when
  separation occurs over the afterbody boattail at the higher Mach numbers or the boundary layer
  is thickened by the addition of the fins, the effect of the reflex lip is masked." RASAero II's
  own comparison with the tunnel (Rogers 2022, slide 2) left the lip out, "assumed that this small
  lip was buried in the boattail boundary layer". hpr took it as a shoulder in the free stream,
  0.085.

**Decision** (`hpr_aero::afterbody`, and `hpr_aero::drag::couple_afterbody` for the coupling):

- **A boattail's supersonic wave drag is Fig. 5-122**, digitized (eight curves at 20 abscissae,
  ±(0.005 + 2%)), log-log in `x` and linear in the area ratio, to 0 at `a = 1` as `(1 − √a)²`,
  **held to the 2D Prandtl–Meyer limit** `−C_p,PM(M, θ)(1 − a)` (NACA Report 1135, eq. 44 and
  171c), which a round boattail can't exceed and the chart's small-angle law can near Mach 1.
  Past the chart's end (`x = 1.4`) its share of that limit closes on 1 as `1/x`, the form of the
  quasi-cylinder solution's recovery; against Jack's 28 points there, −2.7% to +8.0%. Curved
  boattails are taken as the cone through their ends.
- **Separation**: from 16° to 30° a straight-line blend in the half-angle from the attached value
  to the base drag coefficient on the annulus (Cubbage).
- **Through Mach 1**: the rule to Mach 0.8, where the buildup's other transonic terms start
  (Niskanen p. 47); a straight line to Mach 1; from Mach 1 the attached drag, held at its Mach 1.2
  value to Mach 1.2 (the handbook's advice), blended with the separated value at the Mach number
  itself, so a boattail steep enough to separate completely drags like the step it tends to
  (physics review). History: a first draft joined the rule at Mach 0.8 to the chart's near-sonic
  value at Mach 1, which read the Arcas Robin +85% to +117% there; the next draft held the Mach
  1.2 value and moved the start to Mach 0.9, a choice made right after seeing that target. The
  validation audit caught it, and Compton's transonic data, transcribed then, put the measured
  rise's half-way at about 0.89, earlier than the 0.95 a start at 0.9 gives. The start went back
  to the buildup's own 0.8, half-way at 0.9. It costs the Arcas Robin 2 of its 4 rows within 10%.
- **A boattail in parts** (physics review): a narrowing part right after another
  drags as a blend, by a merge weight, of its own drag as a boattail and its share of the boattail
  it continues: the cone from that boattail's start through its aft end less the cone through its
  fore end (held at 0 until the fourth round, below). The weight is 1 for a turn of up to 3° between the parts, 0 from 10°
  (a corner), linear between, and faded by what the parts between have left of that boattail
  (below). So parts of one straight cone add up to one cone, a corner keeps each part its own
  boattail, and a pair drags between the two. It applies at every speed, so below Mach 0.8 a
  curved boattail in parts drags as the cones through its ends rather than part by part as eq.
  3.88 would (physics review: a 6°, 9°, 12° boattail in three 20 mm parts, 0.00716 part by part,
  0 merged, 2.8% of `C_D0` at Mach 0.5). (Superseded in the third round, below:) A part shallower than 1° merges only in proportion to
  its angle. The first draft left each part its own boattail (the same cone drawn as two transitions
  read +10% at Mach 1.5 on the Arcas Robin); the second merged any adjacent parts into one cone
  (a 15° boattail closed by a near-vertical transition read +13% at Mach 0.6 against a step
  down); the third blended the cones' geometry, which gave a gentler second part negative drag.
  The 3° and 10° are a judgement.
- **The base behind a boattail** (the last body component a boattail, or a lip in its wake):
  from Mach 2.5, Fig. 5-141 with Love's cylinder, taken as the ratio of the two pressure
  coefficients and applied to hpr's own base drag (Fleeman's, unchanged, #68); below Mach 2.5
  that ratio at Mach 2.5, chosen because it matches the measured bases at Mach 1.59 and 1.91,
  which therefore check it in sample; back to 1 from Mach 1 to 0.8; toward 1 with the separation
  weight.
- **A lip in a boattail's wake**: a lip behind a boattail, drawn as a shoulder, a step up or both,
  in one part or several, loses its pressure drag while its top rises up to a quarter of the
  boattail's drop in diameter above the boattail's end, keeps all of it from half, and a
  straight-line share between; a lip in parts takes, at each part, the smallest share any top so
  far leaves, a step up by its fore radius and a shoulder by its aft radius too (the physics
  review found one fraction for both moved the drag 25% when a nanometre of tube split them), and
  the base behind it takes the same share of the relief, less the lip's own length's fade. The quarter and half are a judgement made knowing the Arcas Robin's lip, the one
  measured, rises 0.17, and all 44 Arcas Robin rows depend on it (with the lip as a shoulder in
  undisturbed air each would read 0.065 to 0.086 higher). The first draft gave any shoulder up to
  the boattail's fore diameter no drag, and the second required an exact match of radii; the
  physics review showed both switching abruptly (a flare back to full diameter got none; a
  micrometre of step or tube moved the Arcas Robin 25%).
- **Gaps and steps are continuous** (physics and code reviews): the flow behind the
  boattails is shared among their tails, the surfaces it may still follow, and each tail holds its
  share faded over one fall (its drop in diameter) by what follows it: a tube's, a lip's or a
  part's length, a step's or a narrowing part's drop in diameter, and a lip's rise. A narrowing
  part moves to a continuation of each tail the share it merges with (the turn's weight, times,
  from the third round, the smaller half-angle over the larger, the larger at most 1°, so a part
  narrowing by nothing is a tube), fades what
  it leaves as a step and a tube, and takes what no tail then holds as its own boattail; a step
  down does the same as a boattail of no length, fully separated, so a closure drawn ever shorter
  is a step. The base and each lip add the tails' holds, at most 1, and tails in the same state
  are one, so there are at most as many as pairs of parts. A change of `ε` in any radius or length
  changes the drag in proportion to `ε`
  (`drag::tests::a_part_narrowing_by_nothing_is_a_tube_and_one_of_no_length_a_step`, behind
  boattails of 2° to 14°; `a_partial_merge_shares_the_flow`; `a_zigzag_boattail_keeps_its_tails_few`;
  `a_sharp_corner_keeps_its_boattails_apart`, `a_lip_in_a_boattails_wake_fades_with_its_rise`,
  `a_lip_drawn_as_a_step_up_is_a_lip`, `a_hairline_step_before_a_lip_changes_nothing`,
  `soft_merges_stay_between_their_limits`). Between Mach 0.8 and 1.2 a part of no length still
  drags its own boattail's straight-line rise where a step drags the base drag's curve, as before.
  The rules the reviews rejected, in turn: one tail that each narrowing part replaced (a part
  narrowing by one ulp after the Arcas Robin's boattail read +15%, a nanometre's widening over
  100 mm −5.7%, a 1 µm closure +4%); several tails and the strongest taken (a partial merge halved
  a lip's wake, the tails grew exponentially on a zigzag boattail, 90 ms a drag call at 26 parts,
  and a part narrowing by `ε` behind a shallow boattail merged into it, −7.5%). With the lip's
  1.3 mm now a length, the Arcas Robin's base keeps 0.944 of its relief; its forebody rows, which
  leave the base out, don't move. A straight cone drawn in parts behind a boattail it partly
  merges with can drag a little differently from the one cone, the old tail's unmerged share
  getting a second chance at a later part: an 8° cone behind a 14° part reads the same in 2 or 4
  parts, up to −0.45% of `C_D0` in 8 and −2.24% in 512, worst at Mach 1.0 (physics review's Mach
  scan). Fourth round: a part's share is no longer held at 0 where the chart makes the longer cone
  drag less than the shorter, so extending a boattail can lower its drag and a part's pressure
  drag can be below 0; merged wholly, the shares add up to the whole cone's drag exactly, which
  is not below 0, and a pair of parts stays between its two limits. Held at 0, they had read a
  7° boattail from 98 mm to 44 mm in 4 parts 1.35% high at Mach 1.0, and a 5° one closing to an
  eighth of its diameter 5% high in 2 at Mach 1.3 (`a_straight_cone_in_parts_is_one_cone`). Third
  round (physics and code reviews): the 1° factor
  now takes the smaller angle over the larger (the larger at most 1°), so a straight cone under
  1° merges wholly (the product of the two angles over 1° had put a 0.8° cone in 8 parts 3.3%
  high); a step down's corner now shelters a lip behind a plain step with no boattail ahead, as a
  closure drawn ever shorter already did: a 98 mm airframe stepping down to a 54 mm motor tube
  showing for 12 mm, then a 62 mm retainer, reads 12% to 23% lower in `C_D0` from Mach 0.3 to 2.5
  (15% at 0.3, 12% at 0.95, 23% at 2.5), unmeasured; the retainer's step keeps 12/44 of its drag
  (`a_retainer_behind_a_step_down_is_in_its_wake`), and none of the shelter is left once the
  exposed motor tube is as long as the step's drop in diameter; and fades now add where steps multiplied
  (behind a 10° boattail, a 1 mm step and a 2 mm tube leave the base 0.433 of its relief, was
  0.513).
- **Fig. 5-141 is power-off**, as is Fleeman's base drag it scales; under power hpr applies both to
  what the motors leave of the base. Love's value held past Mach 5.5 would ask for less than a
  vacuum, and is clamped there.
- **Checks**: `validation/fixtures/aero/measured-boattails.json` transcribes 193 boattail drags
  (Compton's whole Fig. 12, 152 of them), 12 base pressures and Jack's 151 points, with figure,
  page, reading uncertainty and the points a report calls questionable; the readings were checked
  a second time against the scans, which corrected seven base pressures and Compton's first
  supersonic readings (taken from the theory symbols). `cargo xtask aero` compares them in
  `drag-vs-mach.json`, and `hpr_aero::tests::boattails_against_measurements` pins every group.
- **Tests that pinned the old model**, re-pinned with the change recorded in each:
  `drag_against_mach` (2 rows of 44 within 10%, was 8), `supersonic_cd_against_rasaero_tables`,
  `drag_against_mach_depends_on_the_fins_and_finish` (0, 0, 0, 2, was 3, 5, 6, 8), and
  `calistos_supersonic_gap_survives_every_plausible_fin_and_finish`, whose assertion no longer
  holds, renamed `calistos_rows_by_fin_and_finish` to pin every combination's rows by band and
  the error ranges of the committed and the best inputs (physics review).

**Result: M1.8b3's targets are not met.**

- **Measured boattails** (fixture; "in sample" marks rows that helped build the model):
  - Attached, 3° to 10°, from Mach 1.2 to 3.12: −21.9% to +28.3%, within 0.0123 in drag
    coefficient (58 rows of 20 boattails); the largest percentages are 3° and 5° boattails whose
    drag is 0.01 to 0.02. Inviscid theory reads such boattails up to about 20% high.
  - From Mach 1.0 to 1.1, Cubbage's 5.6° and 8°: −18.2% to −5.4% (4, under the peak; partly in
    sample, as his peak was weighed in holding the Mach 1.2 value). Compton's
    questionable points from Mach 0.95 to 1.1: −46.2% to +60.0% (27). Through the rise, Mach 0.85
    to 0.95: −77.5% to +7.6% (28). Under the rule, to Mach 0.8: −100% to −83.5% (58, #73).
  - Cubbage's 16° in his thick boundary layer: +26.4% to +54.2% from Mach 1.0 (9), −30.2% to
    +60.4% below (6). His separated 30° and 45°: −2.8% to +6.6% (3, in sample: they set the
    separation angles).
  - The base drag behind 5° to 10° boattails at Mach 1.59 and 1.91 (8, in sample): within 0.0102
    of the measured on the cylinder's area, though behind small bases that is up to about 40% of
    the base's own drag, and the correlation ignores the angle (Cortright and Schroeder's relief
    grows from 5.6° to 9.3° at one area ratio). Behind Kurzweg's at Mach 3.24 (4): within 0.003.
- **The Arcas Robin, fins off, from Mach 1.5: 0 of 11 within 10%**, +13.5% to +24.1% (was +8.1%
  to +71.1%, 2 within). RMS 19.1% (was 41.9%). The 2 within before were within because the lip's
  0.085 made up for the missing wave drag. If the rest of hpr's forebody were right, the tunnel's
  boattail would drag about 0.12 at Mach 1.5 against hpr's 0.196: the same over-prediction as
  Cubbage's 16°, larger, for a 15° boattail whose flow NASA reports separating at the higher Mach
  numbers. All 44 rows now read high, 2 within 10% (was 8). Fins off, row for row: at Mach 0.6
  and 0.8, +12.0% to +22.8% (was +40.7% to +49.1%); at 0.9 and 0.95, +38.8% to +54.1% (was +18.3%
  to +43.2%); from 1.0 to 1.2, +20.3% to +50.8% (was −9.2% to +17.2%).
- **Calisto against RASAero II: 8 of 17 supersonic rows within 10%**, −14.9% to −5.1% (was 0,
  −29.8% to −24.4%); transonic 3 of 7, −10.1% to +16.4% (was 2); subsonic unchanged, 15 of 15.
  The inputs the export doesn't record span most of the supersonic rest: rounded fins 4.76 mm
  thick, smooth, have 15, 4 and 14 rows within 10% by band, airfoil fins 6.35 mm thick 11, 4 and
  17, and no combination every row. The committed design keeps ADR-009's rule; picking the inputs
  that fit would be tuning. Calisto's 18.4° boattail is steeper than any attached boattail
  measured, so this agreement is no support for the model at that angle. The getting-started
  variant moves from 6 of 17 supersonic to 0, 23% to 32% high.
- **Why not tuned.** The one correction the evidence asks for, less wave drag for steep boattails
  in a thick boundary layer, has no cited method among these sources, and fitting it to the
  Arcas Robin would make the target its own calibration. It stays an open gap (#72).
- **Whole flights don't move**: no boattailed rocket in the suite passes Mach 0.8, and
  `cargo xtask validate --check` reproduces the committed report.

**Consequences.**

- The drag of a supersonic rocket with a boattail rises: Calisto's `C_D0` at Mach 1.5 from 0.443
  to 0.542. A steep boattail in a thick boundary layer reads high, and a gentle one reads low
  through Mach 1; the guide says so.
- Issues: #72, steep boattails in a thick boundary layer; #73, the subsonic rule against Cubbage
  and Compton. #68 (Fleeman's base drag against Love's) is unchanged: the relief multiplies
  Fleeman's value.
- M1.8b's done-when is carried: M1.8's drag bullet recorded not met here and in ADR-029, the
  Arcas Robin compared, predicted Prometheus flying.
