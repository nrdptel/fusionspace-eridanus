# Streamer and tumble drag: the sources M1.7b can use

Re-checked 2026-10-17: 20 claims corrected.

Written while shipping M1.7a, whose sources cover canopies only: Knacke's manual has **no streamer
data at all** (checked: the type tables 5-1 to 5-5, the measured-drag section 5.2.3, the
miscellaneous-decelerator section 5.8.4 and the contents; his "ribbon" is always a ribbon canopy).
This note records what M1.7b could cite instead. M1.7b has since shipped (ADR-013,
`docs/physics/recovery.md`); the re-check read every source below from its PDF, so none is still a lead.

## Streamers

**OpenRocket Technical Documentation v13.05** (CC BY-SA, pinned as `openrocket-techdoc-13.05`),
Appendix C "Streamer drag coefficient estimation", printed page 117:

```text
C_Dm = 0.034 · (ρ_m + 25 g/m²)/(105 g/m²) · (l + 1 m)/l          (C.6)
```

on the reference area `A_ref = w · l` (one side of the strip; eq. C.4, printed page 116), with `l` the
length, `w` the width and `ρ_m` the material's surface density. The same appendix appears in
Niskanen's 2009 thesis (printed page 115, pinned as `niskanen-2009-thesis`, CC BY-NC-ND: read, never
copy).

Its own validation, printed page 117: an independent set of streamers gives 12 to 27% error in the
normalized drag coefficient, which the text puts at 10 to 15% in descent velocity. The fit came
from a 40×40×120 cm wind tunnel at 6, 9 and 12 m/s (printed page 114) and `ρ_m` of 10 to 80 g/m²
(polyethylene, cellophane, crêpe paper; Table C.1, printed page 118). The text states no width or
length range; Figure C.1's axes (printed page 115) span roughly `w` 0.01 to 0.09 m and `l` 0.2 to
1.0 m. The appendix says the rocket hangs below the streamer as a point mass and that the twirling
("labile") regime cannot be measured in a tunnel (printed page 113).

**Carruthers and Filippone, "Aerodynamic Drag of Streamers and Flags", *J. Aircraft* 42(4), 2005,
pp. 976–982, DOI 10.2514/1.9754.** Peer-reviewed and independent. On the one-side planform area:

- `C_D = 0.405 AR^(−0.494)` for a 0.075 m² planform (eq. 1);
- `C_D = 0.561 AR^(−0.480)` for 0.025 m² (eq. 2);
- a third trend line, `C_D = 0.6514 AR^(−0.6075)` for 0.05 m², printed only on Figure 3 (the shipped
  model uses all three).

The fits run over `AR = L/W` of 3.3 to 30 (the streamers are 10 to 30; 3.3 is the flag) at 6 to
18.9 m/s and `Re` 2e5 to 2e6. They are for heavy cotton (177 g/m²) with the leading edge (luff)
clamped, which matters when a rocket's streamer is lighter or free at the luff. The AIAA copy is
paywalled; the author post-print at
`https://pure.manchester.ac.uk/ws/files/29884977/POST-PEER-REVIEW-NON-PUBLISHERS.PDF` has no stated
license, so cite it and never redistribute it. It is pinned cite-only (`validation/refs.lock.toml`,
`THIRD-PARTY-NOTICES.md`).

**Kidwell, "Streamer Duration Optimization: Material and Length-to-Width Ratio", NAR R&D, NARAM-43
(2001)**, ref. [46] of Appendix C, cited there as background rather than as C.6's data, at
`https://www.narhams.org/library/rnd/StreamerDuration.pdf` (no stated terms; pinned cite-only like
the paper above). It gives no drag or descent equation (its one equation is a confidence interval),
but it gives free-drop data: 4 in × 40 in streamers with about 5 g of weight at one corner, dropped
20.1 m. The rates are distance over time and normalized to a 5 g weight: 2.04 m/s (Micafilm) to
2.80 m/s (crêpe paper). **Every streamer except the crêpe paper was folded in ¾ in pleats**, and
Kidwell puts crêpe's poor result down to having none, so crêpe is the only flat one.

**What Kidwell's drops imply.** Taking 2.80 m/s as the average over a fall from rest, not as the
terminal speed, the crêpe streamer's terminal speed is 2.88 m/s and its drag coefficient on the
planform is 0.155. C.6 gives 0.0367 at Kidwell's 32.2 g/m², so the measured drag is about 4.2 times
C.6's, about a factor of two (2.06) in terminal speed. Carruthers and Filippone's curves at
`AR = 10` give 0.130 (0.075 m²) and 0.186 (0.025 m²), −16% and +20% from the measured 0.155. The
streamer, 0.1032 m² and 0.1016 m wide, is just outside both models' fits, not inside C.6's. The
shipped replay (`docs/physics/recovery.md`) predicts the flat crêpe streamer +9% fast with the
Carruthers–Filippone default and +88% fast with C.6, and pleated Micafilm (implied 0.338) +58% and
+154% fast, because hpr models no pleats.

**What shipped.** ADR-013 made the Carruthers–Filippone curves the default (`Filippone`, all three
curves, interpolated in `ln S`, clamped at the end curves) and kept C.6 as `OpenRocket`, for
comparing like for like with the OpenRocket oracle. The Kidwell replay is a unit test,
`streamer_models_against_kidwells_drop_tests` in `crates/hpr-sim/src/recovery.rs`, not a
`validation/` case.

## Tumble

The same technical documentation, §3.5 "Tumbling bodies", printed pages 53–55 (this section is
*not* in the 2009 thesis; it was added in the 2013 edition):

```text
½ ρ v₀² (C_D,f A_f + C_D,bt A_bt) = m g                           (3.98)
C_D = (C_D,f A_f + C_D,bt A_bt) / A_ref                           (3.99)
C_D,bt = 0.56  on the body tube's profile (side) area
C_D,f  = 1.42  on the effective fin area: one fin's area times an efficiency factor
```

Table 3.4, printed page 55, the efficiency factors by fin count: 1 → 0.50, 2 → 1.00, 3 → 1.50,
4 → 1.41, 5 → 1.81, 6 → 1.73, 7 → 1.90, 8 → 1.85. The constants were fitted to 22 m drop tests of
five models, and the document says they predict terminal velocity within 3 to 14%. Replaying its
Table 3.3 (printed page 54) through equation 3.98 gives −5.8, −5.4, −7.2, +19.0 and −10.0%, so
**−10 to +19%**; the outlier is the finless tube, which wants a body coefficient near 0.79. That
replay is what `docs/physics/recovery.md`, ADR-013 and `docs/accuracy.md` quote.

The text says the fin half is the unreliable one (changing the error function moved `C_D,f` but
barely moved `C_D,bt`). It calls 0.56 "exactly half" a circular cylinder's crossflow coefficient
(so 1.12, a figure it doesn't print), as expected of a cylinder falling at a random angle, and calls
1.42 similar to a flat plate's 1.17 or an open hemispherical cup's 1.42: it equals the cup's value
and is above the plate's. Its citation for those, Hoerner's *Fluid-Dynamic Drag* (1965), is
copyrighted with no legal free copy, so it can't be pinned.

**Free replacements for Hoerner's numbers.** Both are US Government works, read from their PDFs:

- **NASA TN D-540** (McKinney, 1960), crosswind drag of circular cylinders: Figure 5, printed page
  13, `C_D` on the projected side area at a subcritical Reynolds number of 88,000 (Wieselsberger's
  data, cross-plotted, not McKinney's own runs): `l/d` 1 → 0.63, 3 → 0.74, 10 → 0.82, 20 → 0.92,
  with a flat plate at 1.27 for `l/w = 10` (1.15 to 1.16 at `l/w` 1 to 2, nearer Hoerner's 1.17).
  Its own measurements, plotted at `Re` 5e5 to 1.5e6, fall to about 0.22 to 0.45.
  `https://ntrs.nasa.gov/api/citations/20040047039/downloads/20040047039.pdf`
- **NASA TR R-474** (Jorgensen, 1977): Figure 2, printed page 76, the 2-D cylinder crossflow
  coefficient 1.2 up to the critical `Re` of about 2e5; Figure 4, printed page 77, the
  finite-length ratio `η` ≈ 0.68 at `l/d = 10`.
  `https://ntrs.nasa.gov/api/citations/19770026166/downloads/19770026166.pdf`
- **Avoid NACA TN 3038** (Delany and Sorensen, 1953) as a primary source: its own text (printed
  page 4) says the circular-cylinder coefficient came out near 1.0 instead of 1.2 because of end
  leakage it left uncorrected.

These are not two independent checks: TR R-474's Figure 4 comes from Goldstein (1938), which
carries the same Wieselsberger data, and R-474 cites TN D-540 itself, so `η` × the 2-D 1.2 =
0.68 × 1.2 ≈ 0.82 is one dataset read twice. Halving the finite-length values the way §3.5 halves
the 2-D one gives 0.37 to 0.46 for `l/d` 3 to 20, below OpenRocket's 0.56, which halves the 2-D
1.12 (inference, ignoring end-on drag). Table 3.3's finless tube points the other way, wanting
0.79, so the free sources make 0.56 plausible without settling it. Neither replaces the cup's
1.42, so the fin constant has no free source.

**Reynolds number is the open limit.** A 41 mm booster at 6 m/s is at `Re_d` ≈ 1.7e4 and a 100 mm
one at 25 m/s at 1.7e5 (sea-level air), both subcritical, where these constants were fitted (the
drop tests themselves ran at about 1.8e4 to 5.3e4). TR R-474 puts the drag crisis from about 2e5.
TN D-540's finite cylinders above it, read per fineness ratio, fall to about 0.27 to 0.6 of their
subcritical value, so a large bare tube would really descend about 1.3 to 1.9 times faster than
the constants say (inference from one figure; the 0.6 end, at `l/d = 2`, is read by eye). A finned stage
would change less, since only the body term has a crisis (inference). Nothing pinned measures that
for a tumbling body; hpr documents the limit rather than extrapolating.

## The transition between tumble and a stable fall

No citable physical criterion found, in the pinned PDFs or in the open literature. §3.5 only says
it applies to stages that "normally are not aerodynamically stable", and Appendix C declines to
model the analogous twirling streamer regime, saying it would need flight tests. OpenRocket's
newer detector (pull request #3190, merged 2026-08-03; `docs/research/openrocket-after-24-12.md` §6)
filters the angle of attack over the rocket's own pitch period; its description calls the older
rule unsound and cites no measurement, so it is a software convention, not a validated threshold.
So tumble should be an explicit choice a user makes, not something hpr infers. If hpr ever infers
it, the threshold is an unsourced modeling choice and needs an ADR.

## Open questions

- No free source measures a tumbling body's drag above the drag crisis, nor the fin constant 1.42;
  both rest on Hoerner through OpenRocket's fit.
- hpr models no streamer pleats, so the default predicts a pleated streamer's descent 58% too fast
  on Kidwell's Micafilm case; his are the only free-drop data in hand (he cites
  earlier ones from 1972, 1976 and 1984), one flat streamer and the rest pleated.
