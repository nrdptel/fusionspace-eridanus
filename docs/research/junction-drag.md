# How much drag the fin–body junction adds

[Apogee-bias](apogee-bias.md) named fin–body interference drag as the largest bounded candidate for HPR Sim's high
apogees on real flights: Gregorek's "as much as 10%" of C_D, or 3–7% of apogee. This note asks what that term measures
to be. **In short: the junction data don't support 10% on thin HPR fins.** Hoerner's correlation, fitted to foils about
10% to 55% thick, crosses zero near 7% thickness. For fins 1–4% thick it extrapolates to a small *favorable* term, −0.5%
to −5.7% of C_D. NASA CR-3811's flat slab is also small and negative. Of two subsonic tests with fins on and off a
rocket, the Arcas models leave about 6% of the forebody's drag (4–8%) for interference at Mach 0.6. A 1955 Cranfield
rocket with 1.2 mm square-edged fins leaves about none at Mach 0.12 (±9% of C_D). HPR Sim over-predicts the Arcas fin
increment, by 10% at Mach 0.6 and 63% at 0.8. Gregorek's rule cites no source; no source measured fillets on a rocket.
So the evidence weakens interference as the cause of the high bias without ruling it out: nobody measured a filleted,
painted HPR fin at flight Reynolds numbers. "Inference" marks this note's own arithmetic.

## 1. Where Gregorek's 10% comes from

- **The 10% is stated, not sourced.** Estes TR-11 puts interference at "as much as 10% above the sum of the
  fin and body tube drag" [J1] PDF p. 13. It repeats a 10% of total drag at PDF p. 42, again citing nothing.
- **The estimating rule.** Eq. 21 (printed p. 25, PDF p. 29) extends each fin to the body's centerline:
  C_D,int = C*_DOF × (c_R·d/2)/S_BT × N. It charges the fin's section drag to the root chord times the body radius. The
  report says this extra area's drag "has been shown to be about equal to" the interference drag, with no citation [J1]
  PDF p. 29. Box, Bishop and Hunt use the same form (their eq. 44), crediting Mandell et al. (1973) [J10] p. 13.
- **It scales with chord and body diameter, not fin thickness,** unlike the junction data (§2). **Inference:**
  it resembles Hoerner's rule that a wing nacelle's interference nets out to about the profile drag of the
  wing it covers [J2] p. 8-16: a body on a wing, not a thin fin at a tube's stern. A link is not shown.
- **Its own worked rockets.** The interference row of the Alpha table is 18.6% of total C_D for square-edged fins (0.154
  of 0.826), 10.3% for streamlined fins and 7.7% for the up-rated model [J1] PDF p. 48 (**Inference**: the percentages).
- **The flight check leans toward the term, weakly.** C_D came from theodolite-tracked altitudes; the report
  accepts tracks that close within 10% [J1] PDF p. 45. With the term, theory against flight was +6% (Alpha B),
  −31% (Alpha C) and −7% (up-rated) [J1] PDF p. 50; without it −14%, −38% and −14% (**Inference**). B and
  the up-rated model then miss by more than 10%. With one rocket type at −31%, three flights can't size it.

## 2. The junction correlations

**Hoerner's correlation (1965).** Where a foil meets a wall, the two boundary layers merge, and the pressure
rise over the foil's rear slows them further [J2] p. 8-10. Hoerner calls the extra drag "independent of the
span" and refers it to the chord squared (eq. 11 and 12, fig. 23, p. 8-10):

    ΔD / (q c²) = 0.8 (t/c)³ − 0.0003        or        ΔD / (q t²) = 0.75 (t/c) − 0.0003 / (t/c)²

The two leading constants differ in the source itself. The −0.0003 is the value at t/c = 0, the flat plate.

- **Range.** Fig. 23's points span t/c ≈ 0.10–0.57 (read by eye), from European and NACA tests and Hoerner's
  own water tunnel (p. 8-11). They hold "for ratios in the order of δ/c = 10%", the wall layer's thickness
  over the chord. The figures nearby run at Re_c ≈ 3×10⁵ to 2×10⁶, so this is a low-speed fit.
- **Thin foils.** The data "even suggest that the interference drag will be negative" below t/c ≈ 8% (p. 8-10);
  the fit crosses zero at 0.07. HPR fins, 3–6 mm on 150–250 mm roots, are t/c 0.012–0.04: below every point.
- **At the stern.** Two wing roots' C_Dc fell from about 0.004 mid-body to 0.0005–0.001 at the stern (fig. 36,
  p. 8-15, by eye). Tail junction drag at the end of a fuselage "can very well be negative" at zero lift, the
  boundary layer there being thick (p. 8-16). Rocket fins sit at the stern.
- **Per root or per side.** Hoerner's own tail estimate takes fig. 23's value per corner, and counts a V tail's
  two panels as "four corners … at the fuselage" (p. 8-12): per side. This note gives both readings; per side
  doubles every Hoerner number.

**NASA CR-3811 (Kubendran, McMahon and Hubbartt, 1984).** A 56.9 mm slab with an elliptical nose, 1.22 m long,
stood on a flat plate at 15.24 m/s; the plate's turbulent layer was 22.9 mm thick at the nose [J3] pp. 6, 11.
Surveys 16 body widths downstream found the junction drag "small and negative" (p. 1): −3% of the two boundary
layers' drag in isolation, and −1% of one face's friction for a wing 16 widths in chord and span
(pp. 28–29, 31). The authors call the term's size, "and even the sign", an open question (pp. 2–3). Its one
station sits before any trailing edge, where Hoerner puts the cause, so it fits a flat fin's mid-chord only.

**Fillets.** Hoerner's gains from a radius alone are "but limited": the best radius is 4–8% of the chord,
leaving about 0.65–0.95 of the bare corner's drag (fig. 29, p. 8-12, read by eye). On a NACA 0010 corner a
radius of 0.4t cut the corner drag by about 20%, and a radius of 2t about 80% once the lost wetted area is
counted (fig. 31, p. 8-13). NACA TR 540 found normal fillets on mid-wing combinations changed the drag by
nothing appreciable, though small fillets markedly improved an unsatisfactory low-wing junction [J4] PDF
p. 21. **Inference:** a 5–15 mm fillet on a 150–250 mm root (r/c 0.02–0.10, r ≈ 1–5t) can cut a positive
junction term by 5–80%. If the bare term is about zero, so is the fillet's effect. TR-11 says filleted
streamlined fins could cut its term to "a negligible fraction", but lacked the data to size it [J1] PDF p. 42.

## 3. On invented HPR fins (Inference throughout)

These are invented, representative sizes. ΔC_D = N · C_Dc · c² / A on the body's cross-section A, per root;
double the Hoerner columns for the per-side reading. At 1–2 m from the nose the body's turbulent layer is
about 14–40 mm thick, so δ/c ≈ 0.06–0.27, near Hoerner's 10%. The Gregorek column takes C*_DOF 0.008–0.010,
about what TR-11's Alpha table implies for streamlined fins (0.046 over 4.42); for square-edged fins it implies
about 0.035, which would multiply that column by about 3.5 [J1] PDF p. 48.

| Case | body d, mm | fins | root c, mm | t, mm | t/c | Hoerner eq. 11 | same, t/c = 0.10 | Gregorek eq. 21 |
|------|-----------:|-----:|-----------:|------:|------:|---------------:|-----------------:|----------------:|
| A | 54 | 3 | 150 | 3 | 0.020 | −0.0087 | +0.015 | +0.042 to +0.053 |
| B | 54 | 3 | 150 | 6 | 0.040 | −0.0073 | +0.015 | +0.042 to +0.053 |
| C | 98 | 4 | 250 | 6 | 0.024 | −0.0096 | +0.017 | +0.052 to +0.065 |
| D | 98 | 4 | 150 | 6 | 0.040 | −0.0030 | +0.006 | +0.031 to +0.039 |
| E | 98 | 4 | 250 | 3 | 0.012 | −0.0099 | +0.017 | +0.052 to +0.065 |

- Against a total C_D of 0.35–0.6, Hoerner gives −0.5% to −2.8% per root, −1.0% to −5.7% per side: an
  extrapolation. A 10%-thick airfoil fin would give +1.0% to +4.7%, or +2.0% to +9.5%. Gregorek gives +5.2%
  to +18.6% with streamlined fins' C*_DOF.
- With apogee-bias's elasticity (−0.31 to −0.57 per unit C_D change), Hoerner's term would *raise* apogee by
  about 0.2–1.6% (per root) or 0.3–3.2% (per side). Gregorek's would lower it by about 1.6–11%.
- Per root, Gregorek's rule is 0.0014–0.0033 of q c². Hoerner's eq. 11 is −0.0003 at t/c 0.02–0.04 and +0.0005
  at 0.10 (+0.0010 per side). So Gregorek's rule is 1.4–7 times even a thick airfoil's junction drag.

## 4. A subsonic rocket with fins on and off: NASA's Arcas tunnel models

**The test.** Half-scale Arcas models on a sting in Langley's 8-foot transonic tunnel, 9.8 × 10⁶ per meter
[J5] pp. 1, 3–4: a 5.72 cm body, four double-wedge fins partly on a 15° boattail, grit strips behind the nose and
each fin's leading edge; short and long bodies, fins at 0°, 2° and off. The axial force is corrected to free-stream
base pressure, so these are forebody numbers, ±0.004 in C_A [J5] pp. 4–5, Fig. 11. HPR Sim compares against them
([aero, Arcas drag](../physics/aero.md#drag-against-the-arcas-robin-wind-tunnel)); the table's columns are rows
`arcas-robin-short` of `validation/fixtures/aero/drag-vs-mach.json` (Fig. 11 readings).

| Mach | short model, fins on | fins off | fin increment, measured | HPR Sim's fins |
|-----:|---------------------:|---------:|------------------------:|---------------:|
| 0.6 | 0.2987 | 0.2217 | 0.0770 | 0.0848 (+10%) |
| 0.8 | 0.3299 | 0.2308 | 0.0991 | 0.1615 (+63%) |

- **A buildup without interference leaves about 6%.** Niskanen's turbulent friction (eq. 3.78), Mach factor (3.82) and
  thickness factor (3.85) [J12] on the fins' mean chord (7.15 cm, Re 7.0 × 10⁵, wetted over reference area 11.7) give
  0.059 at Mach 0.6 and 0.057 at 0.8. That leaves +0.018 ± 0.006 at Mach 0.6 (the root-sum-square of two ±0.004
  readings): 6.1% of the forebody's drag, 4–8% with the error. At 0.8 it leaves +0.042, 13% (**Inference**).
- **That leftover is not all junction, nor a bound.** The grit strips on every fin add drag nobody sized. The
  fins' inner part sits in the body's boundary layer, which lowers their friction, the other sign. The
  leftover grows with Mach while friction falls, as a compressibility effect over the boattail would, not as
  a fixed junction drag would (**Inference**; the report doesn't say).
- **Gregorek's rule over-corrects at Mach 0.6.** Root chord times radius over fin area is 0.65, so the rule adds
  0.038: a fin increment of 0.097 against 0.077 measured (+26%), and 0.094 against 0.099 at 0.8 (**Inference**).
- **HPR Sim reads high already.** Its fins carry 0.036 of friction and 0.049 from the airfoil leading-edge term
  at Mach 0.6. Its fins-off forebody reads 12–14% high on the short model and 23% on the long (same file).
  Gregorek's term on top would widen the fin gaps to about +60% at Mach 0.6 and +100% at 0.8 (**Inference**);
  a negative Hoerner term would narrow them slightly.
- **The Reynolds length matters as much as the leftover.** HPR Sim follows Niskanen and takes the friction
  Reynolds number on the rocket's whole length ([aero](../physics/aero.md), [J12] eq. 3.12). On this model
  that gives a fin C_f of 0.0028 against 0.0045 on the fin's own chord, a 0.023 difference in fin drag
  (**Inference**). That is about the size of the whole leftover above.
- **Fin anchors.** The short model kept its external root blocks with the fins off; the long model's slots
  were filled flush [J5] pp. 3–6. The long model's fin increment exceeds the short one's by 0.021 and 0.020,
  about 7% of the forebody. **Inference:** root hardware may cost more than the junction itself, but body
  length differs too, and the report blames the anchors mainly at Mach 0.975–1.2.

## 5. Other measurements

- **Free flight, 3, 4 and 5 fins (NACA RM L7A08, 1947).** Rocket-boosted bodies with bluff fins flew up to
  about Mach 1.4 [J7] pp. 1–4. Going from 4 to 5 fins raised each fin's interference by about 8% of a fin's
  drag at Mach 1.0 and 0% at 1.35; the effect reverses above that. The author warns the scatter allows "no
  clear quantitative conclusions" (p. 2). No finless body flew, so total interference is unknown.
- **Supersonic base pressure.** Fins move the base-pressure coefficient "in the order of" 10% either way on
  straight bodies [J2] p. 16-11. On the boat-tailed RM-10, −C_pB rose from 0.09 to 0.13 with fins.
  **Inference:** about ±0.01–0.015 in C_D on a straight HPR tail, a fin–base effect, not the junction.
  NACA TR 1160 has RM-10 drag with and without fins at Mach 1.6–3.4 but separates no interference term
  [J9], nor does TN D-4014 on the Arcas models [J6]. No public junction data at Mach 1–2.5 was found.
- **Low speed, three spans and none (Harris and Appleby, CoA Note 29, 1955).** A full-scale rocket, 59.7 mm by 503 mm,
  carried four 1.22 mm square-edged plate fins of 21, 30 and 43 mm span on an 82.6 mm root, the aft 29.7 mm of it past
  the base; no fillets, no trip wires on fin tests; 40 m/s, Mach 0.12 [J16] PDF pp. 2, 5–6, 13. At −1° and +2° the fins
  added 0.00–0.03 to a fins-off C_D of 0.32–0.34 on the body's cross-section, each ±0.02 (PDF pp. 4, 7, 12).
  **Inference:** at Re_c ≈ 2 × 10⁵, friction alone on these fins is 0.013–0.025 laminar or 0.028–0.050 turbulent, so the
  junction leftover is about zero (−0.013 to +0.011 against laminar friction) or negative (against turbulent); at ±0.02
  it can't be extrapolated to zero span. Gregorek's eq. 21 gives 0.028–0.035 here (0.018–0.023 counting only the 52.8 mm
  of root ahead of the base), or 0.08–0.12 with square-edged C*_DOF: 2.6–4 times the largest fin-set increment, 0.03.
- **One HPR flight (Box, Bishop and Hunt).** A 76 mm rocket on an L730, with eq. 44's term, simulated 3,539 m against
  3,594 m flown (−1.5%), peak speed 11% high [J11] Table 8, p. 42. "A single result" [J10] p. 16; it bounds nothing.
- **Checked, no answer.** DeMar had no fins-off model, and fin shapes differed "not measurable" [J13] PDF pp. 5,
  8. A 2024 KTH CFD study removed the fins [J14] PDF p. 14. Peak of Flight 514 claims fillets help, unmeasured
  [J15]. RM L55H09 measured the RM-10's fin drag rise at Mach 0.81–1.41 [J8] Fig. 10; not reduced here.

## What this suggests

1. **Don't add Gregorek's rule as a default.** It is uncited, ignores thickness, over-corrects Arcas at Mach 0.6 and
   Cranfield at 0.12, and adds 5–19% to C_D on thin HPR fins where the junction data say about zero.
2. **Pin the Arcas fin increment as a check first.** Fins-on less fins-off, short model, Mach 0.6 and 0.8,
   from data already committed. Any later fin-drag change, junction or friction, must not widen it.
3. **Test the fin friction's Reynolds length** (rocket length against fin mean chord) on that increment
   before any interference term. It moves Arcas fin drag by 0.023, the size of the whole leftover.
4. **If a junction term is wanted, use Hoerner's eq. 11**, per side as Hoerner applies it, flagged as
   extrapolated below t/c 0.10. Expect −1.0% to −5.7% in C_D on thin fins: the wrong way for the high bias.
5. **Look elsewhere for the high bias.** Fin–body interference, as measured, is unlikely to supply 3–7% of
   apogee. Apogee-bias's other open causes (as-built mass, early deployments, finish, rail buttons) move up.

## Open questions

- No fillet-on/off test, and no fins-on/off test of a square-edged fin above Re_c 2 × 10⁵, was found. The horseshoe
  vortex ahead of a square leading edge, in a wall layer 2–13 fin thicknesses deep (CR-3811 had 0.4), is unbounded.
- The Arcas grit strips' drag; the fin thickness (TN D-4013 Fig. 1(b) labels 0.04d at the root, 2.3 mm against
  the design's 3.81 mm, which would raise the Mach 0.6 leftover to about 0.020); the long model; RM L55H09.
- Simpson's junction-flow review (Annu. Rev. Fluid Mech. 33, 2001) is paywalled. No ESDU junction item is
  public. The Tetrault and Schetz strut–wall papers and Barber's strut data were not reached.
- How fig. 23's data were reduced, per root or per side; Mach 1–2.5; angle of attack ([J2] p. 8-11: about C_L²).

## Sources

All read 2026-10-11. NACA and NASA reports (public use per NTRS) are cached in the gitignored `refs/`; the
rest are all rights reserved or unstated: cited, not copied.

- **J1.** Gregorek, Estes TR-11, 1970; PDF pages. <https://www.plans.rocketshoppe.com/pubs/Estes/estTR-11/TR-11.pdf>
- **J2.** Hoerner, Fluid-Dynamic Drag, 1965. <https://archive.org/details/FluidDynamicDragHoerner1965>
- **J3.** Kubendran, McMahon and Hubbartt, NASA CR-3811, 1984. <https://ntrs.nasa.gov/citations/19840021784>
- **J4.** Jacobs and Ward, NACA Report 540, 1935. <https://ntrs.nasa.gov/citations/19930091613>
- **J5.** Ferris, NASA TN D-4013, 1967; printed pages. <https://ntrs.nasa.gov/citations/19670020050>
- **J6.** Babb and Fuller, NASA TN D-4014, 1967. <https://ntrs.nasa.gov/citations/19670020031>
- **J7.** Mastrocola, NACA RM L7A08, 1947; printed pages. <https://ntrs.nasa.gov/citations/19930085604>
- **J8.** Howell and Braslow, NACA RM L55H09, 1955. <https://ntrs.nasa.gov/citations/20050019382>
- **J9.** Evans, NACA Report 1160, 1954. <https://ntrs.nasa.gov/citations/19930092188>
- **J10.** Box, Bishop and Hunt, report, 2009. <https://cambridgerocket.sourceforge.net/AerodynamicCoefficients.pdf>
- **J11.** Box, Bishop and Hunt, J. Aerosp. Eng. 24(1):31–45, 2011, author copy; journal pages.
  <https://pages.hmc.edu/spjut/AdvRoc/PDF/CambridgeRocketSimPaper.pdf>
- **J12.** Niskanen, OpenRocket MSc thesis, TKK, 2009 (not the code). <https://openrocket.sourceforge.net/thesis.pdf>
- **J13.** DeMar, NAR R&D, 1995. <https://www.rocketmime.com/rockets/ModelRocketDragAnalysis.pdf>
- **J14.** Seiz de Filippi, KTH, 2024. <https://kth.diva-portal.org/smash/get/diva2:1881325/FULLTEXT01.pdf>
- **J15.** Apogee Peak of Flight 514. <https://www.apogeerockets.com/education/downloads/Newsletter514.pdf>
- **J16.** Harris and Appleby, CoA Note 29, 1955; PDF pages. <https://dspace.lib.cranfield.ac.uk/handle/1826/11132>
