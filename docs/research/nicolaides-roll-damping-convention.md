# Why Barrowman's Basic Finner roll damping points sit 1.27 times Nicolaides' range data

This note asks why the five "wind tunnel" roll-damping points in Barrowman's Fig. 5-7 sit about 1.27 times Nicolaides'
own range data. HPR Sim's roll-damping row is scored against those points
([ADR-031](../decisions/0031-roll-from-canted-fins-and-roll-damping-by.md),
[aero](../physics/aero.md#roll-against-the-arcas-robin-and-the-basic-finner)).
[finner-reference-data.md](finner-reference-data.md#2-hpr-sims-roll-damping-against-the-measured-sets) found they
sit above four measured sets (DREV, BRL 711, NOL and AEDC), and left the cause open. This note reads Barrowman's
source, BuWeps TN-100A, and tests each candidate cause Mach by Mach against BRL 799's range curve.

**In short: the gap is one constant scale factor, about 1.27, not a real aerodynamic difference. Fig. 5-7 is the
outlier.**

- **TN-100A was reached and holds no such data.** The scan (AD287400, 486 pages) is the text of chapters 1 to 7 and
  the figures of chapters 1 to 6. It defers the roll moments to an "Appendix A" that the scan lacks (§1).
- **Fig. 5-7's points are 1.269 times BRL 799's range curve** (Nicolaides and Bolz, as NOL replotted it), standard
  deviation 0.016 over five Mach numbers from 1.51 to 3.00, with no trend (slope −0.004 per unit Mach). Divided by
  57.3, four of the five land on single BRL 799 rounds (§3). **Inference:** they are Nicolaides' range rounds,
  rescaled once, and their "wind tunnel" label is wrong.
- **A convention slip fits; nothing else does.** 4/π = 1.273 fits the curve to −0.3% on average and 1.25 to +1.5%;
  both lie inside the reading band. A rate slip (2), a span slip (3 or 9) and fin-base frontal area (1.41) are about
  10% or more away. A real facility or model difference changes with Mach; this factor doesn't (§4).
- **What it moves.** Scored against BRL 799's curve, HPR Sim's damping reads **+5.4% to +20.4% high** at Mach 1.51
  to 3.00, not −5.9% to −16.2% low. With the same forcing, HPR Sim's steady roll rate from canted fins reads 5% to
  17% low. Apogee isn't touched directly.

Confidence: high that the factor is constant and that Fig. 5-7 is the outlier (it is 1.12 to 1.38 times every other
set, from four facilities). Medium on which conversion slipped, and on whether the factor is 4/π or 1.25: the
appendix that would show it wasn't in the scan, and BRL 799 itself was seen only as NOL's replot.

## 1. What TN-100A holds

*Missile Flight and Astrodynamics*, John D. Nicolaides, Bureau of Naval Weapons Technical Note 100A, cover dated
1959–61, "Released to ASTIA by the Bureau of Naval Weapons without restriction" (PDF p. 4) [N1]. Barrowman cites it
as 1959 [B1 PDF p. 86, printed p. 76]. The PDF from NTRL has 486 pages.

- **Contents** (PDF pp. 8–10): chapters 1 to 7 (trajectories, angular motion, the linear and non-linear theory,
  then projectiles, bombs, rockets and other classes). No appendix is listed or present. The text is PDF pp. 12–323;
  the figures of chapters 1 to 6 follow, marked "pt. 2" (PDF pp. 325–485), mostly hand-drawn. Chapter 7 has no
  figures in the scan.
- **Roll damping.** §3.9 sets up pure rolling motion and says "Both of these roll moments will be discussed in
  Appendix A" (PDF p. 152); it also derives a skin-friction roll damping for bodies of revolution (PDF pp. 155–156,
  Fig. 3.37). None of the 161 figure pages plots C_lp against Mach (viewed as contact sheets). Fig. 6.19 (PDF p. 475)
  plots steady roll rate against angle of attack at Mach 0.23 to 2.48, unscaled and with no source stated. A search
  of the text's rough OCR for Fig. 5-7's Mach numbers and for "finner" found no table; chapter 7 was only sampled.
- **Range roll figures.** Fig. 3.35 (PDF p. 415) fits theory to roll angle against distance for four rounds (a
  finned model by its sketch); Fig. 3.36 (PDF p. 416) fits roll rate against distance and is marked "BRL 711".
  The text's one tunnel roll study found is a low-speed lock-in test (PDF p. 256).
- **The coefficient form.** Fig. 3.37 writes the damping moment as C_lp (pd/2u) ½ρu² S d (PDF p. 417), the form
  Nicolaides keeps in 1968: per p·d/(2V), on Q·S·d [N2 PDF p. 9], with "Reference area" left blank (PDF p. 8).
  Neither gives S. **Inference:** a rate slip of 2 between Nicolaides and Barrowman is unlikely; the area is unstated.

## 2. What Fig. 5-7 states

- **Barrowman's convention.** C_l is the moment over q·A_r·L_r, and C_lp is its slope against p·L_r/(2V) [B1 PDF
  p. 7]. Fig. 5-7 prints A_r = 78.6 in² and L_r = 10 in [B1 PDF p. 80, printed p. 70]: A_r is π·L_r²/4 within 0.1%.
  **Inference:** L_r is the body diameter (Fig. 5-6 is drawn in diameters and doesn't print one), so Fig. 5-7
  claims the convention of DREV and AEDC: area πd²/4, length d, rate p·d/(2V).
- **The points.** Five crosses labeled "wind tunnel", read for ADR-031 at Mach 1.51, 1.82, 2.27, 2.60 and 3.00 as
  −33.60, −27.45, −23.48, −20.92 and −18.32 (±0.3; a second reading agrees within 0.05). The axis's 3.0 tick sits
  0.10 Mach past the even spacing of the others; the last point is read by its tick label. Barrowman says reference
  21 gives "a full set of data" for the Basic Finner and "does not indicate the degree of accuracy" (§5.30, PDF p. 76).
- **Not his own computation.** The points scatter both sides of his computed curve (below it at Mach 1.51 and 1.82,
  above at 2.60 and 3.00). His text prints a 5.68% comparison without saying what it averages; read off the figure,
  the curve is 2% to 7% from the points (7% at Mach 1.51). That is data against theory, not a replot.

## 3. Fig. 5-7 against BRL 799, Mach by Mach

BRL 799 is Nicolaides and Bolz's range report on the pure rolling motion of finned missiles, NOL's reference 2
[F5 PDF p. 15]. It wasn't found on archive.org. NAVORD 6652 Fig. 7 (PDF pp. 31–32) replots its rounds as "ballistic
range (BRL Report No. 799)" beside NOL's tunnel points, in NOL's coefficient L/(q·A_p·b·n) per p·b/(2V) [F5 PDF
pp. 6–7]. NOL finds its own values "slightly below the free flight data" (PDF p. 13).

**How it was read (Inference).** Fig. 7 was rendered at 200 dpi and each page calibrated on its own printed grid
(residuals 0.5 px or less); the "2.0", "4.0" and "5.0" labels confirm the scale. The calibration puts NOL's Table 2
tunnel points (PDF p. 17) within 0.002 of their tabulated values, with the fifth plotted at Table 1's Mach 2.76, not
Table 2's 2.70. The range curve was fitted to clean columns away from the circles; reading error about ±0.002 in C.
A first reading, on a scale fitted with 2.70, read the Mach 1.50 circle (0.587) as the curve (0.573); a second,
independent reading caught it. The factor 45.84 (144/π: four 1 d × 1 d panels on a 3 d span) puts NOL's form on
πd²/4 and d [finner-reference-data.md §1].

| Mach | Fig. 5-7 | BRL 799 curve, NOL form | × 45.84 | Fig. 5-7 ÷ curve | nearest BRL 799 round (Mach, NOL form) |
|--:|--:|--:|--:|--:|---|
| 1.51 | −33.60 | −0.573 | −26.3 | 1.279 | 1.50, −0.587 |
| 1.82 | −27.45 | −0.480 | −22.0 | 1.248 | 1.80, −0.480 |
| 2.27 | −23.48 | −0.398 | −18.2 | 1.287 | about 2.24, −0.406 |
| 2.60 | −20.92 | −0.358 | −16.4 | 1.275 | 2.62, −0.355 |
| 3.00 | −18.32 | −0.318 | −14.6 | 1.257 | 2.92, −0.324 and 2.99, −0.315 |

- **The ratio is flat.** Mean 1.269, standard deviation 0.016, least-squares slope −0.004 per unit Mach, within the
  reading error of both figures.
- **The points match rounds.** Fig. 5-7 ÷ 57.3 gives −0.586, −0.479, −0.410, −0.365 and −0.320. At Mach 1.51, 1.82,
  2.27 and 3.00 that lies within 0.005 of a BRL 799 round 0.04 or less away in Mach; at 2.60 it is 3% above the
  nearest. The figure has at least 14 circles. **Inference:** Barrowman's five points are BRL 799 rounds (or round
  groups), passed on with one constant factor.
- **The other sets agree with NOL's replot, not with Fig. 5-7.** DREV's range (four rows, Mach 1.83 to 3.15) lies
  within about 5% of the BRL 799 curve and below Fig. 5-7 at every row. NOL's tunnel reads 0.91 to 0.97 times the
  curve; AEDC's Mach 2.5 point is about 3% above it, and BRL 711 with its journal points 0% to 15% above it
  (**Inference**, read against the curve) [finner-reference-data.md §1]. Fig. 5-7 is 1.12 to 1.38 times them all.

## 4. Testing each candidate cause

The factor to explain is 1.269 (1.248 to 1.287 by Mach). Each candidate is the ratio it would produce between a
value in Fig. 5-7's stated convention and the same moment in the measured sets' convention. Misfit is the measured
ratio over the candidate, minus one.

| Candidate | Factor | Mean misfit (range by Mach) | Verdict |
|---|--:|--:|---|
| A πd²/4 value taken as on d² and converted once more (one 4/π step) | 1.273 | −0.3% (−2.0% to +1.1%) | fits |
| A conversion factor of 180/π where 144/π belongs | 1.250 | +1.5% (−0.2% to +3.0%) | fits |
| Rate p·d/V against p·d/(2V) | 2 or 0.5 | −37% or +154% | excluded |
| Span 3 d as the length (or area × length) | 3, 9 | over −57% | excluded |
| Frontal area with the fins' bases (0.32 d² added) | 1.407 | −9.8% (−11.3% to −8.5%) | excluded |
| Per degree against per radian, alone | 57.3 | far off | excluded |

- **The two that fit can't be separated here.** They differ by 1.8%, about the reading error of Fig. 5-7 (±0.3, 0.9%
  to 1.6%) plus that of NOL's Fig. 7. The curve favors 4/π; matching single rounds favors 1.25, since 57.3 times the
  NOL-form round values reproduces four of Fig. 5-7's points within 2%. No convention gives 180/π by itself, so the
  1.25 row is a coincidence until a source shows the step.
- **Where the slip sits is open.** TN-100A's missing appendix, Barrowman's reading of it, or NOL's replot of BRL 799
  could each hold it. If NOL's replot held it, BRL 799's own values would sit 1.27 times above BRL 711 (same lab,
  same authors, its definition stated), NOL's tunnel, AEDC and DREV. **Inference:** the slip lies upstream of
  Fig. 5-7, and either way Fig. 5-7 disagrees with four independent sets.

**Why not a real difference (Inference).** A model or facility difference changes with Mach; this factor doesn't.

- NOL's tunnel against the same range curve: 0.941, 0.967, 0.932, 0.965, then 0.923 at Mach 2.76 as plotted, a
  non-monotonic 3% to 8% spread, which NOL ascribes partly to the range's small yaw (PDF p. 13).
- Fin thickness raises the damping through Busemann's second-order term, as in Barrowman's FIN method [B1 PDF p. 92].
  For a single wedge of half-angle ε the lift slope grows by 1 + 2(K₂/K₁)ε, with K₁ = 2/β and
  K₂ = ((γ+1)M⁴ − 4β²)/(4β⁴). For ε = 0.04 rad (fins 8% of the diameter thick, B1 Fig. 5-6) that is +5.1% at Mach
  1.5, +5.1% at 2.0 and +7.2% at 3.0: at most a third of the gap, and it changes with Mach. BRL 711's thicker fins
  would add about twice as much.
- A blunt-base, Reynolds-number or model-scale effect of 27% at every Mach from 1.5 to 3.0 has no support in any
  of the five sets. None is bounded here, but none would be flat.

**What it says about Barrowman's supersonic theory (Inference).** His computed curve lies 2% to 7% from the points.
If the points are 1.27 times too large, his curve reads roughly 20% to 35% above the measured damping, so its
agreement is no evidence for his supersonic method. HPR Sim's first-order damping sits between them (§5). ADR-031's
subsonic check against his Mach 0.07 value is untouched: no data, and no factor, enter it.

## 5. What it moves in HPR Sim

HPR Sim's damping, N·C_lp·k_R(B), is −31.62, −25.31, −20.12, −17.61 and −15.36 at the five Mach numbers (aero page).

| Mach | vs Fig. 5-7 (now) | vs BRL 799 curve | vs Fig. 5-7 × π/4 | vs Fig. 5-7 ÷ 1.25 |
|--:|--:|--:|--:|--:|
| 1.51 | −5.9% | +20.4% | +19.8% | +17.6% |
| 1.82 | −7.8% | +15.0% | +17.4% | +15.3% |
| 2.27 | −14.3% | +10.3% | +9.1% | +7.1% |
| 2.60 | −15.8% | +7.3% | +7.2% | +5.2% |
| 3.00 | −16.2% | +5.4% | +6.8% | +4.8% |

- **Sign.** HPR Sim overstates the Basic Finner's supersonic roll damping, most near Mach 1.5. **Inference:** linear
  theory's 1/β load rises faster than the fins' toward Mach 1, the pattern ADR-031 found in the Arcas Robin forcing.
- **Effect on a flight.** The steady roll rate is p = −(C_l0/C_lp)(2V/d), so with the forcing unchanged it reads low
  by 1/(1 + error): about 5% low at Mach 3 and 17% low at Mach 1.5. Where the forcing also reads high (+14% to +48%
  at Mach 1.5 to 1.8 on the Arcas Robin), the two partly cancel. Apogee isn't touched directly; any effect through
  roll-coupled motion is unbounded here.
- **Against DREV** the row stays as the earlier note gave it.

## What the project can use

- **TN-100A** [N1]: a US Government work released without restriction. Cached under `refs/`; cite freely. It holds
  no roll-damping numbers to commit.
- **NAVORD 6652 Fig. 7** [F5]: "Released to DDC by the Naval Ordnance Laboratory without restrictions". The BRL 799
  curve readings above can be committed as a fixture with their method and ±0.002 band: the only free range set
  that spans Mach 1.5 to 3.0 (BRL 711's public points span 1.59 to 2.68), and Nicolaides' own.
- **Suggestion (#763):** score the roll-damping row against BRL 799's curve and NOL's Table 2, with BRL 711 and
  ARL-TR-846 as committed points and DREV scored locally (statistics only, #619). Expected row: +5.4% to +20.4% on
  BRL 799, +11% to +30% on NOL's tunnel. Keep Fig. 5-7's points out of the scored row, or carry them as a disputed
  set divided by 1.27 with this note as the reason. ADR-031's −5.9% to −16.2% and the aero page's "reads low for want
  of a thickness term" would both change: the sign flips, and thickness would push HPR Sim further high.
- **Wording.** ADR-031 and the aero page say Barrowman's curve is 5.68% from the points "on average"; his text
  doesn't say "average", and the largest gap read off the figure is about 7%.

## Open questions

- **TN-100A's Appendix A.** Is there a third part (or a revision B) holding the roll moments? DTIC's citation page
  returned 403 on 2026-10-29, and only AD287400 was found on NTRL. Its table would settle 4/π against 1.25.
- **BRL 799 itself,** to read its coefficient definition and confirm NOL's replot. Not in archive.org's DTIC
  mirror as searched on 2026-10-29.
- **NSWC TR-3773** (Daniels and Hardy, 1978) [N3] reviews Nicolaides' roll work but covers incompressible flow only;
  its references were not followed.

## Sources (read 2026-10-29 unless noted)

- **N1.** Nicolaides, *Missile Flight and Astrodynamics*, BuWeps TN-100A, 1959–61, AD287400 (from NTRL's download).
  <https://ntrl.ntis.gov/NTRL/dashboard/searchResults/titleDetail/AD287400.xhtml>
- **N2.** Nicolaides et al., *Flight Dynamics of the Basic Finner in Various Degrees of Freedom*, AFATL-TR-68-82, 1968.
  <https://archive.org/details/DTIC_AD0852114>
- **N3.** Daniels, Hardy, NSWC TR-3773, 1978. <https://archive.org/details/DTIC_ADA055718>
- **B1.** Barrowman, NASA/TM-2001-209983 (MS thesis, 1967). <https://ntrs.nasa.gov/citations/20010047838>
- **F5.** Regan, NAVORD Report 6652, 1964. <https://archive.org/details/DTIC_AD0600975>
- **F1, F2, F6, F7** (DREV-TM-9703, BRL 711, AEDC-TR-76-58, ARL-TR-846): as in
  [finner-reference-data.md](finner-reference-data.md#sources-all-read-2026-10-18), read 2026-10-18.

US Government reports (N1–N3, B1, F5) are released without restriction. DREV-TM-9703's terms bar redistribution:
this note compares against it only in words and ranges over several rows, never its values (#619).
