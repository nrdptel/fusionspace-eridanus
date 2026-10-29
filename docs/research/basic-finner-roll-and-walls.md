# The Basic Finner's roll split and the tunnel-to-range gap near Mach 1.07

[basic-finner-transonic-tables.md](basic-finner-transonic-tables.md#4-what-moves-the-peak-with-sign-and-size) found
that DRDC TM 2002-136's tunnel runs at 45° roll read the Basic Finner's normal-force slope (C_Nα) 2% to 9% below its
0° runs, and that the 0° runs read 6% to 12% above the range near Mach 1.08. Issue #619 needs to know which of these
a roll-free model should be scored against, and how high a transonic peak it can claim. This note asks what theory
and other measurements say about roll orientation on a cruciform (four-fin) body, and what bounds the tunnel's wall,
blockage, sting and Reynolds-number effects. The DRDC and DREV reports' terms of release bar redistribution, so their
values appear here only as percentage ranges over several rows, rounded outward (#619's decision).

**In short: score a roll-free model against the mean of the 0° and 45° runs, with the two rolls as the band's edges.
The claimable rise from Mach 0.79 to the peak is then about +17%, not +21.6%.**

- **Theory says the linear slope cannot depend on roll.** Slender-body theory finds a cruciform's lift and pitching
  moment independent of bank angle, for identical surfaces of any planform [S1]. A symmetry argument extends this to
  any Mach number for the linear term (§1). Any roll effect is nonlinear in angle of attack.
- **Other measurements show the same sign and size.** A NASA body-plus-four-tails model read C_Nα 7.3% to 7.7% lower
  at 45° than at 0° at Mach 0.60 and 0.90, and 4.6% lower at 1.19 [A1, A2]. The gap grows with the fit's α window,
  and a fit with an α|α| term moves most of it out of the linear slope (§2). DRDC fits slopes over ±5°.
- **Most of the "tunnel high" gap is the roll split.** At DREV's tight range point (Mach 1.077, C_Nα ±2%), the roll
  mean of the tunnel runs reads within 2% in C_Nα and 5% to 6% high in C_mα, not 6% and 12% (§3). Against the
  range's C_mα (held to 1%), the 0° runs read 8% to 12% high from Mach 0.77 to 1.11; the 45° runs −3% to +5%.
- **The facility's own effects are not bounded.** No calibration, porosity or sting-interference study of the DRDC
  tunnel is published. Outside bounds: one calibration model's lift at 4° scatters 5% to 7% (1σ) across six tunnels
  near Mach 1 [W4]; sting diameter moved C_mα by about 3% or less at Mach 1.1 to 1.3 in one AEDC study [W5];
  blockage and Reynolds number each push the tunnel low, not high (§4).

Confidence: high on the theory and on the ratios (each recomputed from the cached tables); medium on carrying the NASA
model's sizes to the Basic Finner, and on the cause of the roll split, which no source here measures directly.

## 1. What theory says, and where it stops

- **Spreiter, NACA Report 962 (1950) [S1].** The summary (PDF p. 5) finds that, for slender cruciform wing-body
  combinations, lift and pitching moment do not depend on the angle of bank, and by symmetry that this holds for
  identical surfaces of any planform and aspect ratio. The conclusions (PDF p. 20) restate it for a body inclined in
  pitch and bank: lift and the center of pressure (CP) do not depend on bank angle.
- **Moore and McInville (1996) [S2].** Their summary of slender-body and linear theory (printed p. 5) lists a
  cruciform wing-body-tail with undeflected controls as roll-independent at small angle of attack. They state that
  the 0° plane generally gives slightly more normal force and a slightly more stable missile than 45° (printed p. 2),
  and blame the 45° deficit mainly on body-wing carryover at high angle of attack (printed p. 8).
- **The symmetry argument (Inference).** For small angles the normal-plane force is a 2 × 2 matrix times the
  crossflow-angle vector. A body unchanged by a 90° turn about its axis makes that matrix commute with a 90° rotation,
  so the in-plane slope is the same at every roll. This needs no slender-body, inviscid or low-Mach assumption, only
  that the force is linear in α near zero. The pitching moment follows the same way.
- **Where it breaks (Inference).** Roll enters through terms nonlinear in α. In a fin-by-fin sketch, fin i at
  roll φ sees incidence α·sin(φ + 90°·i): the linear terms sum to the same value at every roll, but a per-fin
  α|α| term sums to 2 at 0° and to 4 × 0.707³ = 1.41 at 45°, 29% less. If the fins' lift curve steepens with
  α, as vortex lift does on low aspect-ratio fins, a straight line fitted over ±5° reads lower at 45°: the
  measured sign. How transonic shocks bend the curve is not bounded by theory.
- **The 2° fin cant (Inference).** All four DRDC fins are canted 2° in the same sense, which keeps the 90° symmetry,
  so the linear slope stays roll-free. Both rolls carry the same cant, so cant cannot explain the gap between them.
  ARL's CFD found no significant change in static slopes between 0° and 2° cant on this shape [S4, PDF p. 25].

## 2. A cruciform body plus tails at nine rolls (NASA TM-2005-213541)

Allen's missile model in Langley's 8-ft Transonic Pressure Tunnel at Mach 0.60, 0.90 and 1.18–1.19, rolls 0° to
±90° [A1]. Its supplement holds the runs as text tables [A2]; configuration 3 is body plus tails, no strakes, fins
undeflected. This is a different body with swept tails; the sizes are a class reference, not the Basic Finner's.
Body radius over semispan is about 0.45 (**Inference** from Fig. 9), against the Basic Finner's 0.33. All rows are
this note's least-squares fits over α −2.5° to 4.5° (**Inference**), per radian on the TM's reference area.

| Mach | C_Nα 0° | C_Nα 45° (mean of ±45°) | 45° against 0° | tails' share | CP at 45° | −90° against 0° |
|---|--:|--:|--:|--:|--:|--:|
| 0.60 | 6.59 | 6.11 | −7.3% | −11.9% | 0.22–0.28 cal forward | −0.1% |
| 0.90 | 7.19 | 6.63 | −7.7% | −11.0% | 0.26–0.27 cal forward | −0.7% |
| 1.19 | 8.59 | 8.19 | −4.6% | −6.9% | 0.11–0.15 cal forward | −2.0% |

- **Shape in roll.** At Mach 0.90 the drop is 3.6% to 4.7% at 22.5°, 7.1% to 8.4% at 45° and 4.1% to 4.9% at 67.5°:
  about half at 22.5°, as a cos 4φ variation predicts. The mean over 0° to 45° equals the 0°/45° midpoint within
  0.2% at all three Mach numbers (**Inference**).
- **The α window.** Over ±2.5° the 45° gap is 3% to 8%; over −2.5° to 6.5° it is 4% to 10%. A fit of
  C_N = a·α + c·α|α| up to 8.5° cuts c at 45° by 12% to 36% (near the sketch's 29%) and leaves 2% to 9% of the
  gap in a, each ±2% to ±5%. The data neither confirm nor rule out a zero linear gap.
- **Floor.** The body alone varies with roll by −2.2% to +4.0% with no pattern, so a few percent is the balance's
  floor. The 0° and −90° runs, the same orientation, agree within 2.0%.
- **Supersonic context.** At Mach 1.99 a body-tail cruciform's small-α slopes are the same at 0°, 22.5° and 45° to
  three digits [S3, Table II, PDF p. 27]. At Mach 1.70 Allen's model reads within 1.7% between 0° and ±45°
  (**Inference**, fits of [A2]).

## 3. The Basic Finner's own split, and the range against the roll mean

Computed from DRDC TM 2002-136 Tables 7, 10, 12 and 13 [D1, PDF pp. 41, 44, 46–47] and DREV-TM-9703 Table IX [D2]
as read in the earlier note; tunnel values interpolated to the range's Mach number (**Inference**).

- **Roll split.** From Mach 0.5 to 1.18 the 45° runs read C_Nα 1% to 10% below the 0° runs, largest near Mach 1.07,
  and their CP lies 0.03 to 0.09 caliber further forward. From Mach 1.25 to 4.1 the two rolls differ by −2% to +6%,
  mostly inside the tunnel's repeatability: 14 same-roll repeat pairs differ by 0% to 4%. The report notes the 45°
  C_mα about 10% low from Mach 0.5 to 1.05 and gives no cause (PDF p. 33).
- **Unlike the NASA model**, whose gap shrinks from Mach 0.90 to 1.19, the Basic Finner's peaks near 1.07. No source
  found explains a transonic peak in the roll split; its cause stays **unbounded** beyond the measured size.
- **The rise to the peak.** From Mach 0.79 to the peak, C_Nα rises +21.6% at 0°, +12.6% at 45° and about +17% for
  the roll mean; C_mα about +26% for the roll mean.
- **The range rolls.** The range models were canted to spin, and spun steadily (PDF pp. 18, 24). A range fit sees the
  roll average, which §2's cos 4φ shape makes the 0°/45° mean (**Inference**). AEDC's static Basic Finner data were
  also taken on a spinning model (AEDC-TR-76-58 [S5], PDF p. 21).
- **Against DREV's Mach 1.077 point** (C_Nα ±2%): the roll mean reads within 2% in C_Nα and 5% to 6% high in C_mα;
  the 0° runs alone 6% to 7% and 11% to 12% high. The tunnel's CP lies 0.05 to 0.15 caliber aft of the range's, at
  both rolls and also at DREV's Mach 1.293 point, where no reflected wave reaches the model (§4).
- **Against this program's range C_mα** (standard deviation 1% or less, Mach 0.77 to 1.11): the 0° runs read 8% to
  12% high, the 45° runs −3% to +5%, the roll mean +3% to +8%. The Mach 1.11 comparison interpolates across the
  tunnel's empty span (Mach 1.076 to 1.176 at 0°, 1.071 to 1.157 at 45°). At Mach 1.29 both rolls read 3% to 5%
  low in C_mα. The range's C_Nα (±14% to ±17%) is too loose to separate the bands.
- **One cause fits both gaps (Inference).** The range's yaw was small (mean squared yaw 1.1 to 2.3 deg², Table 7),
  the tunnel's fits span ±5°. If the curve steepens with α, a ±5° fit reads above a small-α one, most at 0°,
  where the α|α| content is largest. This would put the range near or below the 45° runs, as measured. It is
  not tested: the report tabulates only slopes, and its run-by-run database (PDF p. 30) was not reached.

## 4. Outside bounds on the facility's effects near Mach 1

- **The tunnel** [D1, PDF pp. 27–28, 35]: indraft, 60 × 60 cm, transonic flow from "a perforated chamber with
  boundary layer control through suction", Mach from one side-wall tap, no wall, blockage or sting correction
  described. Its full description is a 1977 commissioning report (Ref. 11) not found online. Model blockage is
  0.28% (**Inference**: body 0.20%, fins 0.08%).
- **Blockage: pushes the tunnel low on the rising flank.** In Langley's slotted tunnels, drag errors near Mach 1
  appeared at blockage as low as 0.03%, and raising blockage from 0.02% to 0.10% delayed drag divergence by about
  0.013 in Mach, a lower effective Mach at the model [W1, printed p. 1]. **Inference:** on the roll-mean rise that
  lag is about −1% in C_Nα on the flank and near zero at the peak; it cannot make the tunnel read high.
- **Reflected nose wave: lands on the model at the peak points (Inference).** A Mach wave from the nose returns from
  the wall, 0.30 m off axis, to the model at about 0.585 m × √(M² − 1): 6.5 to 7.7 calibers from the nose at Mach
  1.054 to 1.076, aft of the 5.5-caliber moment center and ahead of the fins. Above Mach 1.12 it misses the 10-caliber
  model. Perforated walls cancel reflections only at a porosity matched to the Mach number [W2 §5.1.3; W3, p. 4];
  DRDC's porosity is not published. Its sign and size are **unbounded**. It acts on both rolls alike, so it cannot
  cause the roll split.
- **Facility-to-facility spread.** The AGARD-B calibration model in six transonic tunnels (blockage 0.07% to 0.83%)
  scatters in lift at 4° by 5.5% (Mach 0.77), 6.3% (1.0) and 5.2% (1.17), as 1σ over the mean (**Inference** from
  [W4] Tables 1, 3, 6 and 9, pp. 762, 766–767). The authors put this at about ten times one tunnel's repeatability.
- **Sting.** DRDC's sting is at least about 0.4 of the 30 mm base at the balance joint (**Inference**, from the
  drawing's scale [D1 Fig. 24, PDF p. 60]); its diameter at the base and its length are not given. The report itself
  recommends a smaller model "to investigate model to sting diameter effects" (PDF p. 34). In AEDC's 4-ft tunnel,
  sting diameters of 0.40 to 0.73 of the model diameter left a finned aircraft model's C_mα "relatively
  independent" (p. 16) [W5]; read by eye from its Fig. 12 (PDF p. 34), within about ±3% at Mach 1.1 and 1.3
  (**Inference**). The DRDC sting's effect is **unbounded**; ±3% is the nearest outside size.
- **Reynolds number.** The tunnel's Reynolds number on length is about half the range's transonically [D1 Table 11].
  The AEDC study found no significant Reynolds effect over 0.5 to 5.0 × 10⁶ per ft [W5, pp. 5, 53; Fig. A-6, to
  Mach 1.05]. **Inference** (flat-plate formulas): a range boundary layer laminar to the fins in quiet air, against a
  turbulent tunnel one, would cost the tunnel about 1% to 2% of fin lift, the wrong sign to explain the gap. Shock
  and boundary-layer interaction on the fins is **unbounded**.
- **No second facility on this shape near Mach 1.** AEDC-TR-76-58 ran the Basic Finner only at Mach 0.22 in its 4-ft
  tunnel; its transonic runs are the *Modified* Basic Finner [S5, pp. 7–8]. ISL's 2022 tunnel tests measured pitch
  damping only [W6].

## What this suggests

- **Score #619's roll-free slope against the 0°/45° mean, with the two rolls as the band.** Reasons: theory makes the
  linear slope roll-free (§1); the split behaves like fit-window nonlinearity (§2); the mean equals the roll average
  within 0.2% on the nine-roll set; and a rolling rocket and both free-flight sets see that average. Expected effect:
  the tolerance gains a cited roll term (the half-spread, about 1% to 5% in C_Nα by Mach) plus the tunnel's 0% to 4%
  repeatability, and the claimable transonic rise drops from +21.6% to about +17%.
- **Don't score against the 0° runs alone.** They carry the most nonlinear content and read highest against the
  range's tight C_mα.
- **Add the range's C_mα as a second, roll-free check.** It is held to 1% and sees the roll average directly.
  Expected effect: a pitching-moment check whose tolerance needs no roll term, only the facility spread (§4).
- **Cite the AGARD-B six-tunnel spread** (5% to 7% at 4°, 1σ, near Mach 1) for the facility part of the tolerance, in
  place of an unbounded wall term. Expected effect: every term of the transonic tolerance has a source.

## Open questions

- **Why the Basic Finner's roll split peaks near Mach 1.07**, while the NASA model's shrinks transonically, is not
  explained. A fit of C_N against α at both rolls would test the nonlinearity reading; the report's run-by-run data
  are "available from the author" (PDF p. 30), and the project does no outreach.
- **Not reached:** ARL-TR-9017 (Vasile and Sahu, 2020, CFD of a finned projectile at 0°, 22.5° and 45° roll; DTIC
  "Under Maintenance" on 2026-10-29, not in the Internet Archive); the DRDC tunnel's 1977 commissioning report;
  Estabrooks's AEDC-TR-59-12 on perforated-wall interference; the full AGARD-AG-336 volume (Chapter 5 only read);
  Nielsen's *Missile Aerodynamics* (cited only through [S2]).
- **Not read:** NASA TN D-4013 has Arcas runs at 0°, 22.5° and 45° roll from Mach 0.6 to 1.2, only as plots; NASA
  TM 74058 tests a cruciform finned projectile at 0°, 45° and 90° roll from Mach 0.7 to 1.2 [W7]. Either could give a
  second transonic roll split by pixel reading.
- **Not found:** CFD of the Basic Finner at 45° roll; any measured normal-force effect of a reflected bow wave on a
  finned body at Mach 1.0 to 1.12.

## Sources (read 2026-10-29)

- **S1.** J. R. Spreiter, *The Aerodynamic Forces on Slender Plane- and Cruciform-Wing and Body Combinations*, NACA
  Report 962, 1950. <https://ntrs.nasa.gov/citations/19930092025>
- **S2.** F. G. Moore, R. M. McInville, *Nonlinear Aeroprediction Methodology for Roll Positions 0 and 45 Degrees*,
  NSWCDD, 1996, DTIC ADA318705, approved for public release. <https://archive.org/details/DTIC_ADA318705>
- **S3.** J. R. Spahr, NACA RM A54H27, 1954. <https://ntrs.nasa.gov/citations/19710065192>
- **S4.** V. Bhagwandin, J. Sahu, ARL-TR-6725, 2013. <https://archive.org/details/DTIC_ADA592550>
- **S5.** L. M. Jenke, AEDC-TR-76-58, 1976. <https://archive.org/details/DTIC_ADA027027>
- **A1.** J. M. Allen, NASA/TM-2005-213541, 2005. <https://ntrs.nasa.gov/citations/20050160470>
- **A2.** The same report's supplement data files. <https://ntrs.nasa.gov/citations/20050165156>
- **W1.** L. M. Couch, C. W. Brooks Jr., NASA TN D-7331, 1973. <https://ntrs.nasa.gov/citations/19740001903>
- **W2.** N. Malmuth et al., *Transonic Wind Tunnel Wall Interference*, Chapter 5 of AGARD-AG-336 (B. Ewald, ed.),
  1998. <https://www.sto.nato.int/publications/AGARD/AGARD-AG-336/07chap05.pdf>
- **W3.** D. Damljanović et al., ICAS 2016-0216.
  <https://www.icas.org/ICAS_ARCHIVE/ICAS2016/data/preview/2016_0216.htm>
- **W4.** D. Damljanović et al., "Convergence of Transonic Wind Tunnel Test Results of the AGARD-B Standard Model",
  FME Transactions 48(4), 761–769, 2020. <https://doi.org/10.5937/fme2004761D>
- **W5.** F. B. Cyran, AEDC-TR-81-3, 1981, DTIC ADA102612. <https://archive.org/details/DTIC_ADA102612>
- **W6.** *Aerospace* 9(2), 51, 2022 (ISL Basic Finner tests), CC BY 4.0. <https://doi.org/10.3390/aerospace9020051>
- **W7.** R. P. Boyden, C. W. Brooks Jr., E. E. Davenport, NASA TM 74058, 1978.
  <https://ntrs.nasa.gov/citations/19780012129>
- **D1.** A. Dupuis, DRDC Valcartier TM 2002-136, 2002, Crown copyright; see
  [basic-finner-transonic-tables.md](basic-finner-transonic-tables.md) (its D1) for the archived copy and terms.
- **D2.** A. D. Dupuis, W. Hathaway, DREV-TM-9703, 1997. <https://archive.org/details/DTIC_ADA636861>
