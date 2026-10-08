# ADR-037: Body lift by Jorgensen's crossflow at every speed, and a boattail's measured share faster than sound (2026-09-19)

- **Status:** accepted
- **Summary:** Body lift by Jorgensen's crossflow at every speed, and a boattail's measured share faster than sound

**Context.** M1.8e5 (ADR-036) ranked the causes of the gap between NASA's Arcas Robin body alone
(TN D-4014, fins off) and hpr's supersonic body: crossflow's size first, the boattail's share
second. hpr's body lift was Galejs's `K (A_plan/A_ref) sin² α` with `K` = 1.1 at every Mach
number; the tunnel's fins-off curvature implies 0.66 to 1.05 from Mach 2.3 and Jorgensen (NASA TR
R-474) about 0.9. A boattail took TN 3527 footnote 8's share, −0.18 to −0.03 per radian where
slender-body theory gives −1.32; nothing measured it. Measured, the fitted-nose body read +14.9% to
+73.2% like for like.

**Decision.**

- **Body lift is Jorgensen's `η C_dn (A_p/A_r) sin² α`** (TR R-474 eq. 2.12), at every Mach
  number: one model, so no join and no jump; below Mach 1 it brings the committed Arcas Robin
  bodies' fitted slopes closer to the tunnel (short at Mach 0.6: +41% to +25%). `C_dn` is Fig. 1's
  (subcritical; from `M_n` 0.6 to 1.2 its Ames points), `η` Fig. 4's against fineness and Fig. 6's
  against the crossflow Mach number `M_n = M sin α`. Fig. 6 holds for bodies of fineness 10 to 12,
  so for fineness `f` hpr takes `η = η₆ [η₄(f) + (1 − η₄(f)) r] / [0.69 + 0.31 r]`, with `r` the
  most that Fig. 6's rise `(η₆ − 0.69)/0.31` has reached up to that `M_n`: a judgement that keeps
  Fig. 4 at low `M_n`, gives Fig. 6 back near `f` = 10.6, gives every fineness Fig. 5's `η C_dn`
  past `M_n` 0.8 (the physics review found that letting the share fall back with Fig. 6's dip at
  `M_n` = 1, an artifact of Jorgensen's division by Fig. 1's peak, brought the length's effect back
  there), and stays below 1. hpr pairs `η C_dn` with its own attached-flow term, not the
  `sin 2α cos(α/2)` Jorgensen subtracted to find it. Both curves are sampled
  at Fig. 6's eleven points, so their product is Jorgensen's own Fig. 5 there (within 3%) rather
  than the product of two steep curves read separately. The fineness is the body's length over
  its largest diameter. The drop in `C_dn` past the critical crossflow Reynolds number is left out:
  Jorgensen computes it only for illustration, with no data.
- **A supersonic boattail takes Washington and Pettis's measured increment** (MICOM RD-TM-68-5,
  1968, Fig. 5: `ΔC_Nα / [1 − (D_B/D)²]` against `√(M² − 1)/(L_B/D)`) on the share the method gives
  a cylinder of its length and fore radius in its place, at their Fig. 6 centre of pressure. Their
  boattails were conical, 4° to 9.5°, 0.82 to 1.18 diameters long, to 0.72 to 0.86 of the diameter;
  anything else is an extrapolation, stated, as is a transition that isn't conical (it takes the
  same correlation from its length and radii); past the curve's end (a short or steep boattail at
  a high Mach number) the last value is held. A
  tube behind the boattail keeps the method's share. The run the method covers is unchanged.
- **The old rules stay selectable** (`BodyModel`, `BodyLift::Galejs`, `SupersonicBoattail::Footnote8`),
  so M1.8e5's fixture reproduces unchanged and sweeps of `K` stay possible; the default is the new
  model.
- **The split.** Blunt tips, which ADR-036 kept in M1.8e6 for coverage, move to a new M1.8e7 with
  the lip behind a boattail: together they keep the committed Arcas Robin designs off the method.
  The old M1.8e7 (issues #87 and #90, M1.8e's 15% bullet) becomes M1.8e8. No done-when changes.
- **M1.8a's comparison gains a miss.** The committed designs fly slender-body theory past Mach 1 and
  read 37% low fins off at Mach 2.96 (short); Galejs's larger body lift had covered enough of that
  for the whole rocket to pass at −13.4%. With Jorgensen's it reads −16.3%, outside M1.8a's 15%.
  The miss is recorded and pinned in `normal_force_against_mach`, not hidden; M1.8e7 is the fix.
- **"The Arcas Robin through a flight's path in the report"** means, as for M1.8e2 and M1.8e4,
  the committed fixture `arcas-robin-crossflow.json` and the guide's tables pinned to it; the
  validation report holds whole flights only.
- **The wind oracle flies the new body lift.** `wind_response.py` carries the same tables (a test
  checks they are the library's) and its constant-`K` sweep adds 1.1.

**Consequences.**

- Like for like, the fitted-nose body reads +3.4% to +41.0% (`arcas-robin-crossflow.json`): each
  change takes about half of the old excess off. From Mach 3.96 both models are within 15% (+3.4% to
  +7.3%); Mach 1.5 to 2.96 still read 16% to 41% high, which the readings can't split between
  body lift and the slope at `α → 0`; at Mach 1.5 and 1.8 on the short model, where the tunnel's
  points from −5° to +4° barely curve (its 6° points need a factor of 0.47 and 0.58 on body lift,
  hpr's is about 0.9), it is mostly body lift.
- At the tunnel's 62 plotted angles from 5.5° to 21.7° (`arcas-robin-high-alpha.json`, read for this
  milestone), 48 are within 15% (34 before).
- **The moment, checked** (a partial model swap can match a slope with its lift in the wrong place):
  the body's centre of pressure at the tunnel's low angles, against its fins-off pitching moment
  (`arcas-robin-fins-off-moment.json`, read for this milestone), was 0.90 to 3.85 calibres aft of
  the tunnel's on the short model and 0.59 to 1.94 on the long; now −0.19 to +1.59 and −0.88 to
  −0.18 (±0.5 from the readings). The long model's low-angle points are M1.8a's, which issue #97
  suspects read 3% to 5% high in slope; its numbers here rest on how that is settled. Where the crossflow is supersonic (`M_n` ≥ 0.95) hpr
  now reads 1% to 16% high, where Galejs's constant read 7% to 22% low.
- Whole flights: every gated metric still passes. The drifts reported as model differences moved
  (Juno III's apogee drift from −42.5% to −38.2%); RocketPy flown with hpr's rail release, body lift
  and fin slope lands within 1.3% of hpr's in every windy case.
