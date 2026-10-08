# ADR-167: A part's drag override, as OpenRocket flies it (2026-10-05)

- **Status:** accepted
- **Summary:** M4.5h: a `.ork` part's or stage's stated drag coefficient flies in hpr as OpenRocket 24.12 flies it, measured on 25 probes; a step down belongs to the part ahead of it; *Base drag hack (short-wide)*, recovery in both, within 2.1% and 4.7% on a C11-5 and a D12-3, 8.0% high on an E12-4 for its drag coefficient (the nose by OpenRocket's breakdown, #177), not the setting: not met for the E12-4, recorded.

**Context.** A `.ork` part may state its own drag coefficient (`<overridecd>`, and
`<overridesubcomponentscd>` for the parts inside it). hpr read both (Loft lesson L63) but had
nowhere to put them, so a part stated at zero was charged the drag of its shape (#165). OpenRocket's
*Base drag hack (short-wide)* read 15.5% to 19.1% under OpenRocket's apogee for it. M4.5h asks that
it fly with the setting applied, each configuration within 5% of OpenRocket's record.

OpenRocket's documentation names the checkbox, not what it replaces, on which area, or where a
step's base drag goes. `validation/oracles/openrocket/drag_override.py` measures it: OpenRocket
24.12 run as an external oracle on 25 probe designs, variants of four airframes (a nose and tube
with a flare behind, the example's shape, carrying fins, lugs, buttons and inner parts; a
boattail; a step down; a step up) with one or two parts' drag stated, at Mach 0.1 to 2.0. What
it shows, on the rocket's total (`getAerodynamicForces`, what OpenRocket flies; its per-part
breakdown does not add up to the total and is not used):

- The stated number is added as it is: it is on the rocket's reference area and does not change
  with Mach number. A stage at 0.5 adds exactly 0.5 at every Mach number.
- It replaces the part's friction, pressure drag and base drag. The base drag of a step down
  belongs to the part ahead of the step: with the tube ahead stated, the step's base goes; with
  the part behind stated, it stays. A step up belongs to the part behind (its fore face).
- A fin set's counts once per fin (three fins at 0.5 add 1.5), a lug's and a button's once per
  instance.
- `overridesubcomponentscd` set, the parts attached lose their drag; a stage's, everything in it
  (a stage at 0.5 covering its children has a `C_D0` of exactly 0.5).
- On an inner tube, a ring or a mass component, it changes nothing.
- One more thing it does: with an aft part stated, the other parts' friction rises, by 6.8% on the
  example (the friction a shorter body would have), but not with a part in the middle stated.

**Decision.**

- **`hpr_design::DragOverride`** `{ coefficient, include_children }`, as `drag_override` on a
  `Component` and on a `Stage`, its own flag apart from the mass overrides' (the file has one per
  quantity). Format 0.2 gains the optional key in place: every document written before reads the
  same. `PlacedComponent` and `PlacedStage` carry it.
- **hpr-aero applies the measured rule.** A stated part's drag terms are its coefficient times its
  instances and copies; a part its parent's or stage's override covers has none; a stage's
  coefficient is a term of its own, listed under the stage's id. `ComponentDragTerms` keeps the
  step down from the part ahead apart (`fore_step_down_area_ratio`), so that it goes with that
  part. `Drag::stated` holds the stated coefficients, beside friction, pressure, base and
  parasitic drag. A stated part's own drag is never computed: a shape the buildup refuses flies.
  A negative or non-finite coefficient is refused by name, on any part or stage; so is what
  wasn't probed: a coefficient on a pod set, on a part in a pod, on a tube fin set, or covering a
  pod set.
- **Friction is kept as the shape gives it.** OpenRocket's rise in the other parts' friction is
  not copied: a part whose drag is stated is still there, and the body still that long. On the
  example it is about 1% of `C_D0`; the probes' test bounds it.
- **The `.ork` reader** puts `<overridecd>` and its flag in the design, reading the single older
  flag (`overridesubcomponents`) as covering drag too, as ADR-095 measured; the writer writes them
  from the design. A coefficient stated on the `<rocket>` element itself has no place in the design
  and is not applied; no public or private design has one. `cargo xtask ork-flights` names that
  case "the rocket's own drag override not applied"; its probe that flew hpr without the parts set to
  no drag is gone with the parts it could remove.
- **An early parachute's leftover is sized on OpenRocket's drag.** A flight whose apogee is more
  than 5% off with OpenRocket's own flight with nothing deployed is flown again by hpr on
  OpenRocket's drag curve, as ADR-097 does for a flight with no named cause, and compared with that
  undeployed flight.

**Measurements.** The probes, held by
`hpr_validate::openrocket::tests::a_stated_drag_coefficient_moves_the_drag_as_openrocket_s`: the
change in `C_D0` from each airframe with nothing stated, hpr's less OpenRocket's, is at most 0.0059
for bodies at every Mach number but a nose's and a boattail's, held up to Mach 0.6 (faster, the
two buildups' own drag of those parts differs by up to 0.21 and 0.061). A step charged to the
wrong part moves it by 0.043 or more. Where lugs or buttons are stated it is up to 0.072 (hpr's
lugs and buttons drag more than OpenRocket's). The tube covering its children is held as a body,
once the lugs' and buttons' own gaps are taken off; the stage covering its children is exactly 0.5
in both codes. A stage alone and the inner parts agree within 1e-9, the fins within 1e-4.
Mutation probes: the step left with the part behind, a fin set counted once, the step's drag
dropped with the stated part, and a lug or a button left uncovered each fail it.

*Base drag hack (short-wide)*, in `validation/reports/openrocket-flights.md`: its apogee against
OpenRocket's record, before → after:

| motors | before | after | both with recovery | both with nothing deployed | hpr on OpenRocket's drag, nothing deployed |
|---|---:|---:|---:|---:|---:|
| C11-5 | −16.77% | +2.06% | +2.06% | — | — |
| D12-3 | −15.47% | +13.16% | +4.65% | +6.03% | −0.10% |
| E12-4 | −19.13% | +11.00% | +7.97% | +9.13% | −0.03% |

OpenRocket's parachute opens 1.80 s and 1.24 s before apogee on the D12-3 and the E12-4, which
the "after" column compares hpr's flight with nothing deployed against. Flown on OpenRocket's own
drag, hpr's apogee comes within 0.10% of OpenRocket's with nothing deployed: what is left is the
drag coefficient. By OpenRocket's breakdown it is the nose: OpenRocket's `C_D0` for the example
is 0.274 at Mach 0.3 and 0.378 at Mach 0.5, hpr's 0.209 and 0.237, and OpenRocket's nose takes
0.064 and 0.141 of pressure drag, hpr's none (read from OpenRocket's calculator on the example
once, while sizing this; no committed fixture holds them). The nose is an ellipsoid 0.58 calibres long, near a
hemisphere. hpr gives elliptical noses Stoney's measured curves, zero below Mach 0.8 (ADR-028);
OpenRocket carries eq. 3.87's rise from rest, which ADR-028 rejected after review because it "gave
elliptical noses subsonic drag the data doesn't show".

**Result.** The setting flies. The done-when names `hpr sim`, which flies the recovery, so the
like-for-like comparison is the column with recovery in both: the C11-5 and the D12-3 are within
5% (+2.06%, +4.65%; `hpr sim` itself, on the curves it fetches, +2.5% and +4.9%). **Not met for
the E12-4** (+7.97%; `hpr sim` +8.1%), recorded. The "after" column, hpr with nothing deployed
against OpenRocket's early parachute, mixes that parachute into the gap; on OpenRocket's own drag
hpr is still 6.6% above OpenRocket's record on the D12-3, all of it the parachute.

The E12-4's miss is the drag coefficient: against OpenRocket's flight with nothing deployed hpr is
9.13% high on its own drag and 0.03% low on OpenRocket's. By OpenRocket's breakdown, which does not
add up to its total, it is the nose. The nearest measurement found is Hoerner's: a hemispherical
head's forebody pressure drag, from measured pressure distributions at low speed, is 0.01 on the
cylinder's frontal area (*Fluid-Dynamic Drag*, 1965, p. 3-12, Fig. 20; a long ellipsoidal head
−0.05). hpr's 0 is within 0.01 of it; OpenRocket's 0.064 at Mach 0.3 is six times it. Hoerner
gives no subsonic point for such a head's rise with Mach number, so the curve between stays open
in issue #177. Copying OpenRocket's here would trade a gap against OpenRocket for one against the
measurement.

**Consequences.** Of 656 designs in the private collection, 10 state a coefficient, on nose cones,
freeform fin sets, transitions and stages, none covering children: hpr now reads and applies
it. None of them is among the private flights the reports compare. The JSON
schema and the TypeScript and Python types gain the key. M4.5's other blockers stay as they were.
