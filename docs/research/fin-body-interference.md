# Should HPR Sim add the body lift its fins carry over?

Re-checked 2026-10-29: 10 claims corrected.

OpenRocket's development line now adds the lift that fins induce on the body behind and beside them ([OpenRocket after
24.12](openrocket-after-24-12.md) §3): the subsonic interference factor goes from Barrowman's 1 + τ to (1 + τ)²,
τ = r/(s + r) with r the body radius and s the fin's span from the body. This note asks which form the measurements
support for hobby fins (spans 0.5 to 2 calibers), what it would do to HPR Sim's margins and apogees, and where it fades
with Mach. **In short: the theory is settled and the data are split.** In slender-body theory the fin factor K_W plus
the body factor K_B is exactly (1 + τ)², so Barrowman's 1 + τ reads a fin set's slope 33%, 25% and 17% below it at
spans of 0.5, 1 and 2 calibers. On the slope, two low-speed models with fins ending at the base side with K_B: the
Martin model, whose fin balances isolate the panels, and the Basic Finner; the Cranfield rocket does too if b is read
tip to tip, not as its report defines it. NASA's Arcas Robin tunnel data, HPR Sim's main measured reference, side
against it: its fin increment matches HPR Sim's fins *without* K_B, and K_B would take its subsonic slope errors from
−4.6% to +9.5% up to +26% to +44%, and the transonic +27.1% to +46% to +69%. On OpenRocket's examples the margin would
move −0.015 to +0.91 caliber from 24.12's (median +0.49, aft on 39 of 42 configurations). Apogee barely moves: 0.0% in
calm air, −0.03% to −2% in wind on an invented rocket, far short of the +8.73% bias. "Inference" marks this note's
own arithmetic; scripts are cached privately under `refs/research/fin-body-interference/scripts/`.

## 1. The two factors (NACA Report 1307)

- **K_W(B), eq. 14 (p. 4).** The fin's lift beside the body, in slender-body theory: 1 at τ = 0, 2 at τ = 1. 1307
  recommends it for every combination; linear theory for rectangular wings, βA 2 to 6, agrees within 5% (p. 5) [N1].
- **K_B(W), eq. 21 (p. 6).** The body's lift induced by the fins. It is (1 + τ)² − K_W, so OpenRocket's (1 + τ)²
  is K_W + K_B to the digit, not an approximation (Inference; USAF DATCOM states the same sum [D1]).
- **Values (Inference, eqs. 14 and 21 evaluated; checked by eye against 1307's Figs. 2 and 5).**

  | Fin span, calibers | τ | K_W | K_B | (1 + τ)² | Barrowman's 1 + τ | K_B / K_W |
  |---:|---:|---:|---:|---:|---:|---:|
  | 0.5 | 0.500 | 1.450 | 0.800 | 2.250 | 1.500 | 55% |
  | 1 | 0.333 | 1.284 | 0.493 | 1.778 | 1.333 | 38% |
  | 2 | 0.200 | 1.162 | 0.278 | 1.440 | 1.200 | 24% |

  Barrowman's straight line runs 3.3% to 3.8% above K_W. Adding 1307's K_B on top of 1 + τ (rather than switching
  to (1 + τ)²) gives about 10% more carryover: 0.370 of the fin set at τ = 1/3 against 0.333.
- **Afterbody.** "At subsonic speeds no distinction is made between the afterbody and no-afterbody cases" (p. 10): 1307
  applies the full slender K_B below Mach 1 even when, as on most hobby rockets, the body ends at the fins' trailing
  edge. Supersonic, Chart 4(b) and eqs. 30 and 31 cover a body ending there, where carryover is much smaller (§4).

## 2. What Barrowman and DATCOM did

- **Barrowman 1966.** He names both factors and keeps only the fin's: K_T(B) is "considerably greater than" K_B(T) for
  r_t/(s + r_t) < 0.4, so a straight line for K_T(B) and no K_B(T) is "a conservative and reasonable approximation"
  (p. 36) [B1]. At τ = 0.4 slender K_B is 45% of K_W, not small (Inference); "conservative" fits the sign, since
  dropping aft lift moves the CP forward and the margin reads low.
- **Barrowman 1967 thesis.** Keeps K_B: eq. 3-98 subsonic, then 1307's no-afterbody chart supersonic (eq. 3-100,
  Fig. 3-9), summed in eq. 3-106 and placed in the CP by eq. 3-107 (pp. 31–37) [B2]. His validation was Aerobee 350
  data at Mach 2 to 8 (C_Nα within 7.83%, CP within 1.18%); "the lack of subsonic experimental data prevents any
  concrete conclusions" (p. 74). Neither form was checked subsonic by Barrowman.
- **USAF DATCOM §4.3.1.2.** Method 1 is 1307's K_N + K_W + K_B, slender-body subsonic and transonic, separate
  supersonic charts with and without an afterbody; "generally ... within ±10 percent" [D1]. Its Mach 0.25 sample (an
  aircraft wing, r/s = 0.145) reads +5.4% with K_B and −10% without (Inference). **Missile DATCOM** uses 1307's
  slender K_B subsonic and states no accuracy for it (Table 33, p. 102) [D2].

## 3. Measured data with the fins at the base

The hobby layout, fins ending at or near the base, is the case that matters. Four sources reach it, all with the total
split into body and fins only by subtraction; none put pressure taps on the body beside a base fin.

- **Martin research model, AEDC-TR-75-125, Mach 0.59** ([its note](dtic-subsonic-finner-data-grm.md) §4–5; τ = 0.52,
  trailing edge in the base plane). Its fins sat on their own balances: measured 2.45 to 2.66 per rad against slender
  K_W's 2.44, so **K_W is confirmed by the panels**. The whole rocket's linear terms without K_B read 4.52 against 5.97
  to 6.39 measured (−24% to −29%); K_B adds 1.30 (OpenRocket's form) to 1.38 per rad against a measured body-side
  residual of 1.5 to 1.7, giving −1% to −9%, and the CP lands at 6.48 to 6.52 d, inside the measured 6.40 to 6.57 d
  (Inference). Those use slender theory's nose, 2.0 per rad. The body alone (parts 10 and 16, printed tables on Vol. II
  PDF pp. 150 and 155; part 16 read, its broken C_N digits kept where C_m = XCP × C_N) reads about 2.5 per rad (2.48 to
  2.54), CP 2.5 to 2.7 d from the nose (points 2.4 to 2.6, slope ratio about 2.7). With it in place of the nose the
  rocket reads −15% to −22% without K_B and −1% to +8% with it, CP 6.57 to 6.72 d against the measured 6.40 to
  6.57 d: at best on its aft edge, the flattering side (Inference, provisional; adding both may count some lift twice).
- **Basic Finner, Mach 0 to 0.5** ([its note](dtic-subsonic-finner-data.md) §5; τ = 1/3, fins flush with the base).
  Barrowman's linear terms read 10.84 per rad against 12.9 to 15.0 measured (−16% to −28%; Mach 0 terms against data
  up to Mach 0.5); with K_B, 13.79 to 14.11 (−8% to +9%). The CP moves from 0.15 to 0.25 caliber ahead of the measured
  8.04 to 8.14 d to 0.04 to 0.17 aft: the slope closes, the CP crosses onto the flattering side (Inference).
- **Cranfield College of Aeronautics Note 29, Mach 0.12** ([hobby tunnel data](hobby-rocket-tunnel-data.md)). A 2.35 in
  rocket, four plate fins flush with the base, τ = 0.41 to 0.59 [C1]. Its theory multiplies the fin-alone slope by
  (1 + d/b)² (printed p. 10), b being the "local net wing span" (p. 9); Figs. 10 and 11 plot against fin semi-span.
  Only a tip-to-tip b gives (1 + τ)², implying fin-alone slopes 73% to 96% of slender theory, near Barrowman's; b over
  both panels gives 41% to 52%, over one 17% to 26% (Inference). That theory reads the measured fin increments 4%, 7%
  and 16% low (printed p. 7) under any reading; with one (1 + τ) divided out, 32% to 47% low if b is tip to tip.
- **NASA's Arcas Robin tunnel models, Mach 0.6 to 4.63** (`validation/fixtures/aero/arcas-robin-wind-tunnel.json`;
  τ = 0.3485, trailing edge 0.12 caliber ahead of the base, the aft part of the root over a 15° boattail). The fins-on
  minus fins-off slope, which includes any carryover, is 0.97 (0.71 to 1.14) of HPR Sim's fin terms *without* K_B over
  22 rows. K_B would need 1.39 subsonic; the measured excess over HPR Sim's fins is +0.87 ± 0.60 of K_B's 3.33 per rad
  at the short model's Mach 0.6 and negative at 0.8, so full K_B sits 3.5 to 6.8 standard errors off in each Mach 0.6 to
  0.8 row, 9.3 and 12.8 at 0.9 (Inference; standard errors from fit residuals). Caveat: with no fin balances, a high
  isolated-fin slope could hide a real K_B, and 1307's cylinder doesn't describe the boattail under the root.
- **Hagerty configuration 1a** ([its note](hagerty-tunnel-lift.md) §4; τ = 0.265, a body behind these fins). Barrowman
  plus Jorgensen's body lift reads −1% to +3%; with K_B, +16% to +27% at 5.4° to 23.4° (Inference).

So on the slope two independent low-speed sources (one with fin balances) want K_B, though the Martin model's CP then
lands aft of the measurement once its own body is used; Cranfield agrees if its span is read tip to tip; the Arcas, the
one measured set HPR Sim's fixtures test against, and the Hagerty reading do not. Nothing reached separates the two
readings on the Arcas: a fin slope that is too high plus a real K_B, or no carryover onto a short boattailed base.

## 4. Where K_B acts, and where it fades

- **Subsonic CP.** 1307 places the carryover on a lifting line at the quarter chord with an image in the body; Chart 16
  puts it 0.1 to 0.5 root chord behind the root leading edge by planform and βA (pp. 68–70, read by eye), about 0.27
  on the Arcas fin, 0.1 chord ahead of its own CP [N1]. So it moves the rocket's CP aft by nearly the fin share's lever.
  Barrowman's thesis uses the same lifting line (eq. 3-101) [B2]. Placing it at the fin CP instead changes the CP shifts
  below by up to 0.05 caliber on the Arcas and 0.11 on the census (Inference).
- **Through Mach 1.** 1307 keeps the slender K_B until βA(1 + λ)(1/(βm) + 1) passes 4 (eq. 22, pp. 7–8), and DATCOM
  keeps slender interference transonically [D1]. That switch falls at Mach 1.08 (Calisto) to 1.44 (Arcas) on the rockets
  here, 1.41 for the Basic Finner's rectangle (Inference). 1307 applies its slender factors through the transonic range
  and warns that nonlinear effects can make them fail there (p. 30).
- **Supersonic, no afterbody (Inference, eqs. 23 to 31).** The carryover falls fast once the supersonic rule applies:
  29% to 46% of the slender value at Mach 1.4 to 1.5, 26% at 2.3 (Arcas), 13% (Basic Finner) to 20% (Arcas) near Mach 3
  and 12% at 4.63 (Arcas). With an afterbody the Arcas keeps 56% to 76%. Without one, the carryover's CP sits at 0.65 to
  0.67 root chord, on the fin root (Chart 14(b)); with one, Chart 14(a) puts it near 0.9 root chord at Mach 1.5 and
  behind the trailing edge from about Mach 1.8 (read from the chart). 1307 says configurations without afterbodies,
  computed with afterbody theory, read high and "fall more in line" with the no-afterbody charts (Fig. 16, p. 21); it
  tabulates no numbers for them, and no measured supersonic K_B for a base fin was found.
- **OpenRocket's fade** (full to Mach 0.9, gone by 1.5) understates base-fin carryover at Mach 1.5 (0 against 0.18 of a
  slender 0.49 on the Basic Finner) and at Mach 2, where the no-afterbody value is 22% of slender (Inference).

## 5. What adding it would move

- **The census margin.** OpenRocket 24.12 uses 1 + τ, so K_B would move HPR Sim away from it, not toward an error. Over
  42 configurations of 11 examples (pods and tube fins not parsed, the parallel booster not reproduced), the margin
  moves −0.015 to +0.91 caliber, median +0.49, aft on 39 (slender K_B: −0.016 to +0.97); the slope rises 22% to 29%.
  Of the report's 53 scored margins, 41 sit within 0.016 caliber of OpenRocket's today; with K_B only the Base drag
  hack's 3, whose fins stand past the body's end, move forward and stay within it, or none with the carryover at 1307's
  quarter chord (Inference; the script reproduces the report's HPR CP within 1 mm on all 42). The census would need
  re-basing on OpenRocket's development line, which runs its own K_B (#3220, #3263).
- **Barrowman's worked examples.** His 1966 method leaves K_B out, so all five would fail their 1% check (+15% to +30%
  slope with both Arcon-Hi fin sets; CP +0.01 to +0.35 caliber aft).
- **The Arcas Robin rows** (`validation/fixtures/aero/normal-force-vs-mach.json`; K_B slender to Mach 1.2, eq. 30 from
  1.5 where 1307's rule passes 4, carryover at the fin CP; Inference):

  | Mach band | slope error today | with K_B | CP error today, cal | with K_B, cal |
  |---|---|---|---|---|
  | 0.6 to 0.8 | −4.6% to +9.5% | +25.5% to +44.2% | −0.36 to +0.91 | +0.74 to +1.71 |
  | 0.9 to 1.2 | −8.8% to +27.1% | +20.3% to +69.4% | −0.41 to +2.36 | +0.18 to +3.18 |
  | 1.5 to 4.63 | −3.3% to +9.4% | −1.1% to +24.0% | −0.53 to +0.35 | −0.15 to +0.58 |

  The slope moves away at 20 of 22 rows, all but Mach 4.63. The CP moves toward zero at 7: the short model's
  Mach 1.0 (−0.41 to +0.18), 2.3 and 2.96, and the long model's 1.8 to 3.96 (−0.53 to −0.09 at 1.8); the long
  model's 1.8 and 2.3 misses come from a body 15% to 19% high ([aero](../physics/aero.md)), and the short model's is 17%
  high at 2.3 and 2.96: those close for the wrong reason. OpenRocket's fade: +46% at Mach 1.2. **No variant helps the
  transonic overshoot** of [transonic normal force](transonic-normal-force.md): K_B scales with the fin slope, peaking
  at Mach 1.2. Ironbark's panel code reads the short model +25% to +46% at Mach 0.6 to 0.8 ([Ironbark](ironbark.md)
  §3), near HPR Sim plus K_B (+26% to +44%), as panel codes carry fin lift onto the body.
- **Calisto against RASAero II.** HPR Sim matches it to +0.06% at Mach 0.1 today; K_B makes it +33% to +36% there.
  RASAero II's method is unknown (open); the match suggests it also uses 1 + τ (Inference, not shown).
- **Apogee (Inference, a planar model of an invented 98 mm rocket, C_Nα 10 to 12.5 per rad, margin 1.5 to 1.95
  calibers).** Calm air: no change (0.00%; −0.01% to −0.03% from a 5° rail). Wind of 3, 5, 8 and 12 m/s: −0.03%,
  −0.09%, −0.24%, −0.52%; for a rocket slow off the rail, −0.15% to −2.17%. A stiffer rocket turns further
  into the wind and climbs less. The sign is toward the logs, but tenths of a percent can't explain the [+8.73%
  bias](apogee-bias.md). Coning and oscillation drag, which stiffness also changes, are not modeled here.

## What this suggests

1. **Don't add K_B to the default model yet.** The only measured whole-rocket reference HPR Sim tests against reads a
   slope further off in every band, and its fin increment leaves no room for K_B unless the isolated-fin slope is too
   high. Barrowman's form also errs on the safe side for the margin: dropping aft lift reads the margin low.
2. **Settle the Arcas contradiction first.** Split its fin increment into panel and body shares: compare HPR Sim's
   isolated fin slope with a fin-balance test of a similar planform, or a panel code on the fins alone. If the panel
   slope runs 1.4 times high subsonic, K_B and a lower fin slope would close the Arcas's slope as K_B alone closes the
   Martin model's, whose CP then sits aft of the measurement (§3).
3. **Measure the Basic Finner's CP crossover** in its supersonic rows with no-afterbody K_B before an option ships: a CP
   aft of the measurement flatters a safety output, and the Martin model with its own body crosses subsonic (§3).
4. **When comparing with OpenRocket's development line**, expect its margins tenths of a caliber above HPR Sim's (up to
   0.91 on the examples, aft on 39 of 42) and read it as this model choice, not a defect on either side.
5. A K_B option needs a Barrowman-only path for the worked examples and 1307's no-afterbody chart past eq. 22 (no fade).

## Open questions

- Why the Arcas fin increment leaves no room for K_B while the Martin model's balances show it. A fin-panel slope that
  runs high, the 15° boattail under the root, or the tunnel's own scatter; none is sized.
- The Martin model's body-alone slope at Mach 0.59 comes from a broken scan (about 2.5 per rad); a clean copy would
  confirm it, and how much of the body's own lift beside the fins K_B already counts is not sized.
- Supersonic carryover for a base fin: 1307 tabulates none of its no-afterbody comparison points, and NACA RM A51J04 and
  A52B06 (later supersonic fin-body tests) were not read.
- Cranfield defines b as net span (p. 9), not giving (1 + τ)²; reading it tip to tip, and 32% to 47%, is unproven.

## Sources

- [N1] W. C. Pitts, J. N. Nielsen and G. E. Kaattari, *Lift and Center of Pressure of Wing-Body-Tail Combinations at
  Subsonic, Transonic, and Supersonic Speeds*, NACA Report 1307, 1957: eqs. 14, 21 to 31; Figs. 2, 5, 16; Charts 4, 14
  and 16. <https://ntrs.nasa.gov/citations/19930091008>
- [B1] J. S. and J. A. Barrowman, *The Theoretical Prediction of the Center of Pressure*, NARAM-8, 1966, p. 36.
  <https://www.apogeerockets.com/downloads/barrowman_report.pdf>
- [B2] J. S. Barrowman, *The Practical Calculation of the Aerodynamic Characteristics of Slender Finned Vehicles*, MS
  thesis, 1967 (NASA/TM-2001-209983), pp. 30–37, 62–74. <https://ntrs.nasa.gov/citations/20010047838>
- [D1] R. D. Finck, *USAF Stability and Control DATCOM*, AFWAL-TR-83-3048, §4.3.1.2.
  <https://archive.org/details/DTIC_ADB072483>
- [D2] W. B. Blake, *Missile DATCOM User's Manual, 1997 Fortran 90 Revision*, AFRL-VA-WP-TR-1998-3009, 1998, Table 33
  (printed p. 102). <https://archive.org/details/DTIC_ADA344707>
- [C1] K. D. Harris and G. G. Appleby, College of Aeronautics Note 29, Cranfield, 1955, printed pp. 7, 9, 10,
  Figs. 1–2, 10–11. <https://dspace.lib.cranfield.ac.uk/handle/1826/11132> (it blocks automated and browser fetches;
  CORE's record <https://api.core.ac.uk/v3/outputs/74410615> confirms it).
- [G1] G. R. Gomillion, AEDC-TR-75-125 Vol. II, 1976, read through [the Martin model
  note](dtic-subsonic-finner-data-grm.md) and on PDF pp. 150 and 155. <https://archive.org/details/DTIC_ADB010054>
- Fixtures `validation/fixtures/aero/normal-force-vs-mach.json` and `arcas-robin-wind-tunnel.json`, and the [OpenRocket
  flights report](../../validation/reports/openrocket-flights.md), at 0f55d9de (re-checked at 4477b0b4).

**Terms.** Every link was tried 2026-10-29 (C1's gave 403; CORE's record was read). N1 and B2 are US government works
that NTRS marks public use permitted; D1, D2 and G1 carry "approved for public release, distribution unlimited" on their
covers. All seven are cached privately in `refs/`, never committed. B1 and C1 show no license: cite only, short quotes
and numbers.
