# ADR-036: The Arcas Robin's supersonic body gap: judged as the tunnel measures; M1.8e6 takes crossflow's size and the boattail (2026-09-19)

- **Status:** accepted
- **Summary:** The Arcas Robin's supersonic body gap: judged as the tunnel measures; M1.8e6 takes crossflow's size and the boattail

**Context.** M1.8e5 sized each cause of the gap between NASA's Arcas Robin body alone (TN D-4014,
fins off) and hpr's supersonic body (`docs/research/body-supersonic-gap.md`, fixture
`validation/fixtures/aero/arcas-robin-gap.json`). M1.8e4 had reported hpr 8.3% high to 27.0% low,
comparing hpr's slope at `α → 0` with a straight line fitted through points from about −5° to
+4°. That line carries crossflow lift. Fitted the same way, with the body lift a flight adds, hpr
reads 15% to 73% high. The fit's slope at `α → 0` and its curvature correlate at −0.95 to −0.96,
so the readings can't say how much of that is body lift and how much the slope at `α → 0`. With
body lift at Jorgensen's `η C_dn` (NASA TR R-474), hpr still reads 8% to 61% high. The one
sized cause that size is the boattail's share: TN 3527's footnote 8 gives −0.18 to −0.03 per
radian, slender-body theory −1.32. M1.8e's 15% bullet names no comparison basis, and M1.8e6's
title ("crossflow and blunt tips", "e5's top ranks") predates the ranking.

**Decision.**

- **The basis, fixed before measuring.** M1.8e6 and M1.8e's 15% bullet (carried by M1.8e7) judge
  the Arcas Robin as M1.8a fits it: hpr's bodies' `C_N` through a flight's path at the tunnel's
  plotted angles, fitted the same way (`hpr.fitted_c_n_alpha`). Slopes at `α → 0` are reported
  beside it, not judged.
- **The tunnel's slope at `α → 0` is never one number.** Report it under both fitted forms
  (`α |α|` and `α³`) and with the curvature held at a cited `K`.
- **M1.8e6's scope follows the ranking:** crossflow's size (Jorgensen's `η C_dn` or a cited
  alternative; the report's points to 16° to 21° show how it grows with `M sin α`) and the
  boattail's share, together.
  Blunt tips stay in it for coverage: the committed design's power-series nose is refused by the
  method. Its done-when is unchanged.

**Consequences.**

- M1.8e6 may change body lift, which drives a slow rocket's drift in wind (ADR-026); it decides
  whether a change applies below Mach 1 and regenerates the report.
- A result judged at `α → 0` alone no longer counts for the Arcas Robin.
- The ranking rests on hand readings of plots (±0.01 in `C_N`; the report states ±0.03 and ±0.1°
  in `α`), and the blunt tip on an assumed scaling; both are stated in the note.
