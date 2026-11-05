# Measured normal force and center of pressure of finned bodies, Mach 0.8–1.2

Re-checked 2026-11-05: 16 claims corrected.

Between Mach 0.8 and 1.2, HPR Sim's normal-force slope (C_Nα) runs up to +27.1% high against NASA's Arcas Robin tunnel
tests, and its center of pressure (CP) up to 2.36 calibers off ([aero](../physics/aero.md#normal-force-through-mach-1)).
The Arcas is HPR Sim's only *measured* transonic reference (the other is RASAero II, a code), so nothing says whether
the miss comes from the model or the Arcas. [M1.14c](../decisions-and-roadmap.md#m1-14c), the open transonic
increment, is to end by its stop rule on a measured gap. This note asks which other public measurements give C_Nα and
CP of an ogive- or cone-cylinder body with planar fins through Mach 0.8–1.2, with geometry complete enough to fly,
by Mach points, fin shape and terms of use, and what each shows about the transonic peak's size and position.

**In short: two kinds of source are good enough to fly, and they disagree with HPR Sim in different ways.**

- **The Basic Finner** (10° cone, L/d 10, four square fins) has the densest grid. From Mach 0.77–0.79 to a peak at
  **Mach 1.07–1.10** its C_Nα rises 22% (tunnel, 0° roll), 13% (45° roll) and 27% (range, fits ±16–17%), then
  falls 5% to 8% by Mach 1.18. HPR Sim's fins peak at their M_s, where linear theory starts (1.2 on the Arcas).
- **Two NASA Langley databases** (Allen 2001 and 2005) are tabulated and public use, on ogive and ogive-like bodies.
  But they have only three transonic points, Mach 0.6, 0.9 and 1.2. Their C_Nα rises steadily, by +7% to +11% at
  0.9 and +11% to +34% at 1.2 over Mach 0.6. HPR Sim rises +21% and +76% over the same span on the Arcas.
- **Only the Air Force Finner dips like the Arcas at Mach 0.8–0.9, inside its fits' errors; none moves its CP as
  far.** The short Arcas's CP moves about 2.6 calibers aft from Mach 0.8 to 1.2 (the long model's goes 1.0
  caliber forward at Mach 1.0 first); the Allen sets and the Basic Finner move theirs 0.1 to 0.8 caliber aft.

Shapes with a dense grid but only plots disagree on the sign at Mach 1.0: a Blue Scout model peaks there (+23% over
0.8), and a finned store dips (at most −6% over 0.8), its CP 0.68 caliber or more forward. The peak's size and
position depend on the shape, and no one public source pins them for a hobby rocket. Confidence: high that these are the
best open sources reached; medium overall, since most figures are plot readings (the Basic Finner's tables agree). Every
slope and CP below marked **Inference** is this note's own fit or plot reading.

## 1. What HPR Sim does here, and what the Arcas shows

HPR Sim's fins follow Prandtl–Glauert to Mach 0.8, then a join rising to linear theory's peak at the fin set's M_s
([aero](../physics/aero.md#normal-force-through-mach-1)). On the short Arcas model its slope rises from 10.54 per rad at
Mach 0.6 to 12.77 at 0.9 and 18.54 at 1.2; the tunnel's falls to 9.92 at Mach 0.8 and 10.22 at 0.9, then jumps to 15.79
at 1.0. **Inference:** that is −7.5% at Mach 0.9 and +39.5% at 1.2 against the Arcas's own Mach 0.6 value; HPR Sim
reads +21% and +76%. The aero page says no closed-form method covers this region and the join is not fitted to it.

## 2. The sources

| Source | Body; fins | Mach, 0.8–1.2 | Data | Terms |
|---|---|---|---|---|
| Basic Finner, tunnel and range [B1, B2] | 10° cone, L/d 10; 4 square | 0.78–1.18, 5 points | B2's plot | release |
| Basic Finner, range [B3] | same | 1.056–1.116, 1.293 | tables | Crown |
| Basic Finner, range [B4] | same | 0.975–1.264 | tables | release |
| Allen 2001 [A1] | 3-cal ogive, L/D 12; 12 pairs | 0.60, 0.90, 1.20 | tables | public use |
| Allen 2005 [A2, A3] | drawn nose, L/D 13.5; 4 tails | 0.60, 0.90, 1.18 | files | public use |
| Air Force Finner [B2, B5] | 2.5-cal ogive, L/d 10; 4 deltas | 0.6–1.25, dense | plotted | release |
| RM SL56A30 store [A4] | fineness 7.7; 4 trapezoids | 0.60–1.03, 1.2 | plotted | public use |
| TN D-1958 Blue Scout [A5] | multi-stage; 4 swept | 0.60–1.03 | plotted | public use |

All give geometry complete enough to rebuild, except Allen 2005's nose profile, which must be assumed; Allen 2001's
fins are pairs. "Release" means DTIC's public-release statement, "public use" NTRS's marking for NASA and NACA work,
and "Crown" Canadian Crown copyright: DREV-TM-9703's values may be committed (#1119, #763), B1's and B9's not (#619).

### The Basic Finner

- **Geometry.** A 10° half-angle cone 2.836 d long, a cylinder to 10 d, tip radius 0.002 d (AEDC-TR-76-58 Fig. 2
  [B5], PDF p. 25). Four fins flush with the base, each 1 d span, a wedge from a sharp leading edge to 0.08 d thick at
  the trailing edge (DREV §3.1 [B3]; ARL §3.1 [B2], PDF p. 18). The chord is 1.0 d (0.94 d on AEDC's drawing; the
  [tables note](basic-finner-transonic-tables.md) settles 1.0 d). Moment reference 5.5 d from the tip in DREV and ARL.
- **The dense set.** DRDC TM 2002-136 [B1] measured it in a tunnel (Re 2.9–4.4 × 10⁶ on length at Mach 0.5–1.38)
  and a range; ARL-TR-6725 Fig. 5 [B2] (PDF p. 25) replots it. **Inference,** by pixel calibration (±0.3 per rad):
  C_Nα 15.0 at Mach 0.5; 16.1 (tunnel) and 16.7 (range) at 0.78; 17.9–18.3 at 0.92; 19.7 (tunnel) at 1.07; 21.1
  (range) at 1.10; 18.0–18.2 at 1.18; 18.3 at 1.29. B1's tables agree within 0.2 per rad (the tables note).
- **Tabulated range points.** DREV-TM-9703 Table IX (PDF p. 48) has multiple fits of C_Nα at Mach 1.077 and 1.293
  (probable errors ±2%, ±3%); single shots at Mach 1.056–1.116 spread 36%, lowest to highest (Table VII, PDF p. 43);
  the readings stay cached under `refs/` (#619). The report says a peak "at approximately Mach 1.25 seems to have
  been missed" (PDF p. 26), which the dense set doesn't support. BRL 934 Table III (PDF p. 41): 19.6 ± 5% at Mach
  0.975, 17.9 ± 4% at 1.139, 16.7 ± 4% at 1.264; rounds between 1.011 and 1.077 carry probable errors of 19.5–56.5%.
  Its text gives about 25 near Mach 1.1 (PDF p. 12), which the other sources don't support.
- **CP (Inference),** from C_mα/C_Nα about 5.5 d on the tunnel pairs: 8.14 d from the tip at Mach 0.5, 8.31 at 0.8,
  8.36 at 0.9, 8.47 at 1.07 and 8.49 at 1.20. About 0.2 caliber aft from Mach 0.8, with no forward jump at Mach 1.
- **What it says.** From B1's tables, the peak is +22% (tunnel, 0°), +13% (45°) and +27% (range) above Mach
  0.77–0.79, at Mach 1.07–1.10. A cone, not an ogive, but its square fins are closer to a hobby rocket's than the
  Arcas's.

### Allen 2001: twelve fins on a tangent ogive

- **Model.** A 3.0 in body 36.0 in long, with a 3-caliber tangent-ogive nose and a plain cylinder (PDF p. 8; Fig. 2,
  PDF p. 265). Twelve fin planforms with unswept trailing edges, aspect ratio 0.67–6 and a modified double wedge of
  t/c 0.031–0.071 (Table 1, PDF p. 18). One pair in the horizontal plane at roll 0°. The fins sit with their trailing
  edges 2.2–2.6 calibers ahead of the base (**Inference** from the stations, PDF p. 11).
- **Tests.** Langley 8-ft TPT at Mach 0.60, 0.90 and 1.20; α from −2° to 25°; Re_L about 8.1, 6.0 and 5.1 × 10⁶
  (**Inference** from Table 4(a)'s unit Reynolds numbers, PDF pp. 20–21). Whole-model C_N, C_A, C_m, the fin's own
  normal force, the fin alone and the body alone, all in tables. The scan's text layer garbles digits; read the image.
- **Fits (Inference),** least squares from about −1.4° to +4.4°, CP in calibers from the nose:

| Configuration | C_Nα 0.60 / 0.90 / 1.20, per rad | 0.90 vs 0.60 | 1.20 vs 0.60 | CP 0.60 / 0.90 / 1.20 |
|---|---|--:|--:|---|
| Fin 12 pair (root 6, tip 3, span 4.5 in) | 25.8 / 28.6 / 33.2 | +10.9% | +28.5% | 7.95 / 7.96 / 8.29 |
| Fin 8 pair (delta, root 6, span 3 in) | 12.9 / 13.8 / 14.3 | +7.3% | +11.0% | 7.70 / 7.79 / 7.87 |
| Body alone | 2.46 / 2.59 / 2.72 | +5.4% | +10.8% | 2.88 / 3.05 / 2.92 |

  Sources: Table 4(hh), PDF pp. 152–153; 4(v), pp. 104–105; 4(kk), pp. 164–165. The fin-12 Mach 1.20 fit has
  4 points from 0° to 4.4°. On its own area the metric fin's slope is 3.08 / 3.47 / 3.77 per rad for fin 12 and
  2.89 / 3.23 / 3.07 for fin 8: the delta peaks between Mach 0.9 and 1.2, the clipped delta not on this grid. That
  share is the measured counterpart of the aero page's fins-on less fins-off split for the Arcas.

### Allen 2005: four tail fins, every run as a file

- **Model.** A 3.15 in body 42.528 in long (L/D 13.5): a 6.293 in nose to 2.694 in, a cylinder, a cone frustum up to
  3.150 in, a long cylinder and a 1.371 in boattail of about 2.3° (Fig. 6, PDF p. 42). The nose profile is drawn but
  not named, so a rebuild must assume one. Four tail fins: root 1.992 in, tip 1.213 in, exposed span 1.953 in, swept
  leading edge, 0.110 in thick (Fig. 9, PDF p. 43).
- **Tests.** 8-ft TPT at Mach 0.60, 0.90 and 1.18 (the files label configuration 3's runs 1.19); α −2° to 22°; roll
  0° to 90°; 1.5 × 10⁶ per ft (p. 10). The supplement holds all 1,059 runs as ASCII tables [A3].
- **Fits (Inference),** α −2° to +4°, fins undeflected, CP in calibers from the nose:

| Configuration, roll | C_Nα 0.60 / 0.90 / 1.19, per rad | 0.90 vs 0.60 | 1.19 vs 0.60 | CP 0.60 / 0.90 / 1.19 |
|---|---|--:|--:|---|
| Body + tails, 0° (runs 1412, 1392, 1372) | 6.59 / 7.19 / 8.59 | +9.1% | +30.3% | 9.89 / 10.13 / 10.56 |
| Body + tails, 45° (runs 1410, 1390, 1370) | 6.13 / 6.59 / 8.23 | +7.4% | +34.1% | 9.67 / 9.86 / 10.45 |
| Body alone, 0° (runs 1451, 1442, 1433) | 2.23 / 2.31 / 2.55 | +3.5% | +14.5% | 3.40 / 3.44 / 3.66 |

### Dense grids, plotted only

- **Air Force (Modified Basic) Finner.** A 2.5-caliber ogive, 10 calibers long, four clipped-delta fins (root
  1.333 d, tip 0.667 d, span 0.5 d; AEDC Fig. 2 [B5]). ARL Fig. 5 [B2] replots range data from Mach 0.6 to 2.5.
  Their source's multiple fits (DRDC TM 2002-008 [B9] Table 8, PDF p. 37) put C_Nα 10% and 9% below its Mach 0.65
  value at 0.78 and 0.91, the Arcas's dip, with 8% and 13% standard errors; from Mach 0.91 to 0.99 the CP moves 0.69
  caliber aft (**Inference**).
- **RM SL56A30 [A4],** a finned store with tabulated ordinates. Static margin 9% of length below Mach 1, zero near
  1.0, stable again at 1.2 (PDF p. 8); no data between 1.03 and 1.2. **Inference:** at 7.58 calibers (largest radius
  2.474 in, Table I) that is 0.68 caliber forward; a pixel reading of Fig. 12 (PDF p. 33) puts the CP past the CG near
  Mach 1.02, about 0.8 caliber from 0.8. The figure's C_Lα dips 6% at Mach 1.0 below 0.8 (8% below 0.6); C_Nα adds
  C_D0 per radian, which rises through Mach 1, so its dip is smaller.
- **TN D-1958 [A5],** a 1/15-scale Blue Scout without its base flare. The text gives C_Nα 0.184 per degree at Mach
  0.8 (PDF p. 7). **Inference,** Fig. 8(a): a peak of about 0.228 near Mach 1.0 (+23%), and x_cp/l from 0.80 to 0.85.

### Read and set aside

RAE TN Aero 2811 [A6] is finless; its CP jumps forward at Mach 0.95, with two C_m curves by the side that Mach is
approached from (PDF p. 7). NAL TR-814 and RM L53J06 ([low-speed note](hobby-rocket-tunnel-data.md)) stop at Mach 0.95
and 0.88; TN D-5378 has a blunt nose; RM L54E13, TN D-918, TM X-1942 and TM-78441 measure the wrong quantity or range.

## 3. What DATCOM offers

USAF DATCOM [B6] has no transonic method for the body or the fin-body. For the body it fairs smoothly between the
subsonic and supersonic values (4.2.1.1-B), with test data "as a guide in fairing" for C_mα (4.2.2.1-B). For fins it
builds lift from wing-alone charts: a force-break Mach and ratios just above it, faired to Mach 0.6 and 1.4 (4.1.3.2-B),
and says very thin or very low-aspect-ratio wings have no transonic dip. Its interference method is the supersonic
slender-body one, "applied at transonic speeds" (4.3.1.2-B). NACA RM A51A12 [B7] (17 rectangular wings, A 1–6, Mach
0.4–1.10) could check fins alone. Missile DATCOM [B8] uses MBB and RAeS (ESDU) sheets; none found public.

## 4. Comparison

| Source | Mach of the peak | Peak over Mach 0.8 | Mach 1.2 over 0.8 | CP, Mach 0.8 to 1.2 |
|---|--:|--:|--:|---|
| Arcas Robin, short (tunnel) | 1.0–1.2 | +59% (at 1.0) | +55% | 2.6 calibers aft |
| HPR Sim on the Arcas, short | 1.2 (its M_s) | +71% | +71% | 1.6 calibers aft |
| Basic Finner (B1, from 0.77–0.79) | 1.07–1.10 | +13% to +27% | +7% to +13% (at 1.18) | 0.14–0.19 aft |
| Allen 2001, fin 12 (from 0.9) | ≥ 1.2 | no 0.8 point | +16% from 0.9 | 0.34 caliber aft from 0.9 |
| Allen 2005, body + tails, 0° | ≥ 1.19 | no 0.8 point | +19% from 0.9 | 0.43 caliber aft from 0.9 |
| RM SL56A30 store (C_Lα) | dip at 1.0 | −6% (the dip) | +26% | 0.68–0.8 caliber forward at 1.0 |

**Inference** throughout. The Arcas and HPR Sim rows use the aero page's short-model table; its CP in meters is
converted at about 0.0573 m a caliber, the ratio of its own meter and caliber columns. The 2.36-caliber miss is the
long model's, whose CP moves 0.97 caliber forward at Mach 1.0 and is 1.52 aft of its 0.8 place at 1.2 (Fig. 12).

## What this suggests

1. **Fly the Basic Finner in HPR Sim** at Mach 0.78, 0.92, 1.07, 1.18 and 1.29, and against DREV's tabulated 1.077
   and 1.293. Read ARL Fig. 5 with the project's pixel method into a fixture, with page and figure for each point.
   Expected effect: the first check of the peak's *Mach*. If HPR Sim peaks at the fins' M_s rather than near 1.07,
   the join is misplaced, not only too high. The Basic Finner's 0.2-caliber aft CP move is a sign check on CP.
2. **Fly Allen 2005 configuration 3 straight from its ASCII files** at Mach 0.60, 0.90 and 1.19, and the body-alone
   runs. No plot reading is needed. Expected effect: a second check of the +25.0% (Mach 0.9) and +20.2% (1.2) rows on a
   cruciform tail. If HPR Sim also reads high there, the steep join is generic, not an Arcas quirk.
3. **Fly Allen 2001 fins 12 and 8 and the body alone,** with the two-fin set across the pitch plane, as tested
   (horizontal fins, pitching). The fin's own tabulated load also tests HPR Sim's fin share without the body.
4. **Keep RM SL56A30 and TN D-1958 as range evidence,** not fixtures: they bound the Mach 1.0 sign (at most −6% to
   +23% over 0.8).

## Open questions

- **Whether the Arcas's dip is real.** The Air Force Finner's is the same size but inside its fits' errors; its
  tunnel data (AFATL-TR-81-87) were not reached.
- **Wall interference** near Mach 1.2 (slotted 8-ft and DRDC tunnels) is not bounded; older reports stop at Mach 1.03.
- **Not found:** TM-2002-211653 (Triservice database), TN D-794 (Scout models, no PDF), TN D-1228; TM X-3070 unread.
- **Whether the Allen 2005 files may be committed** as a public-domain fixture, or only readings of them, is open.
- **No hobby shape at hobby Reynolds numbers:** all are about 10⁶ to 3 × 10⁷ on length, none with fillets.

## Sources (read 2026-10-16; B1 and B9 2026-10-20; all re-checked 2026-11-05)

- **A1.** J. M. Allen, NASA/TM-2001-210652, 2001. <https://ntrs.nasa.gov/citations/20010020654>
- **A2.** J. M. Allen, NASA/TM-2005-213541, 2005. <https://ntrs.nasa.gov/citations/20050160470>
- **A3.** Same, supplement (data files). <https://ntrs.nasa.gov/citations/20050165156>
- **A4.** T. L. Fischetti, NACA RM SL56A30, 1956. <https://ntrs.nasa.gov/citations/20050028473>
- **A5.** T. C. Kelly, R. J. Keynton, NASA TN D-1958, 1963. <https://ntrs.nasa.gov/citations/19630012011>
- **A6.** D. J. Kettle, RAE TN Aero 2811, 1962 (Crown copyright). <https://archive.org/details/DTIC_AD0329780>
- **B1.** Dupuis, DRDC TM 2002-136, 2002 (Crown; links in [the tables note](basic-finner-transonic-tables.md)).
- **B2.** Bhagwandin and Sahu, ARL-TR-6725, 2013. <https://archive.org/details/DTIC_ADA592550>
- **B3.** Dupuis and Hathaway, DREV-TM-9703, 1997. <https://archive.org/details/DTIC_ADA636861>
- **B4.** MacAllister, BRL Report 934, 1955. <https://archive.org/details/DTIC_AD0068959>
- **B5.** Jenke, AEDC-TR-76-58, 1976. <https://archive.org/details/DTIC_ADA027027>
- **B6.** Finck, USAF Stability and Control DATCOM, 1978. <https://archive.org/details/DTIC_ADB072483>
- **B7.** Nelson and McDevitt, NACA RM A51A12, 1951. <https://ntrs.nasa.gov/citations/19930086595>
- **B8.** Vukelich et al., Missile DATCOM Vol. I, 1988. <https://archive.org/details/DTIC_ADA211086>
- **B9.** Dupuis and Hathaway, DRDC TM 2002-008, 2002 (Crown; links as B1).
