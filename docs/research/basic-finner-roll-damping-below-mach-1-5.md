# Measured Basic Finner roll damping, Mach 0.2 to 1.4

HPR Sim's roll-damping row ([aero](../physics/aero.md#roll-against-the-arcas-robin-and-the-basic-finner),
[ADR-031](../decisions/0031-roll-from-canted-fins-and-roll-damping-by.md)) checks nothing measured below Mach 1.5: its
Mach 0.07 entry is Barrowman's computed −34.21. This note puts the measured subsonic and transonic sets on one
convention and scores HPR Sim and Barrowman against them, building on [the finner note](finner-reference-data.md)
and [the Nicolaides note](nicolaides-roll-damping-convention.md).

**In short: seven measured sets exist below Mach 1.4, and the clean ones agree on about −17 to −19.5 from Mach 0.2 to
1.0. HPR Sim's damping, and Barrowman's −34.21, read about twice that.**

- **The clean subsonic value.** Three public tunnel sets at low spin rate (AEDC, Belgrade's T-38, INCAS) and NAL as
  replotted (spin rate not stated) give C_lp −17.1 to −19.4 at Mach 0.2 to 1.0, at α = 0; NOL's Mach 0.22 curves span
  −17.0 to −20.3 with spin rate (§1–2). Two restricted range sets (DRDC, DREV) agree with that band up to about
  Mach 1.1, then rise toward a peak near Mach 1.4.
- **HPR Sim reads high, by +66% to +99% at Mach 0.2 to 0.4** and +91% to +138% at Mach 0.48 to 1.0 against public
  sets; +100% to +150% against seven range fits at Mach 0.8 to 1.1; +20% to +90% at Mach 1.2 to 1.4 (§3).
  Barrowman's −34.21 at Mach 0.07 is +68% to +101% above the low-speed sets.
- **The 1.27 factor does not apply to −34.21.** It is computed, not data, and HPR Sim reproduces it within 2.0% in the
  stated convention. Divided by 1.27 anyway, it would still read +32% to +59% high (§4).
- **What it moves.** On measured references the row reads high at every Mach number, about twice below Mach 1. The
  steady spin from canted fins reads 22% to 24% low by Eastman's damping-to-forcing correlation, and about 25% low
  against AEDC's measured forcing and damping at Mach 0.22 (§5). Apogee isn't touched directly.

Confidence: high that HPR Sim's subsonic damping is about twice the measured: five public tunnel sets and DRDC's
range agree within about ±10% from Mach 0.2 to 1.0 (NOL's Mach 0.77 set excluded, §2). Medium on single readings,
four sets being read from figures. How the gap splits between causes is unbounded (§5).

## 1. The measured sets

All values here are C_lp per p·d/(2V) on area πd²/4 and length d, at α = 0 unless noted. NOL's coefficient is
L/(q·A_p·b·n) per p·b/(2V), converted by 144/π = 45.84 as in the earlier note (AEDC's replot of NOL at Mach 0.22
confirms it, below). The others state the target convention. "Read" says how a value was taken.

| Set | Facility, method | Mach | pd/2V | Re, length | C_lp | Read |
|---|---|--:|---|---|--:|---|
| NAVORD 6652 [N] | NOL tunnel, spin decay | 0.22 | 0.04, 0.07, 0.10 | 1.0 × 10⁶ | −17.0, −18.4, −20.3 | pixel, Fig. 3 |
| NAVORD 6652 [N] | same | 0.77 | 0.012, 0.021, 0.030 | 2.5 × 10⁶ | −8.8, −15.5, −17.4 | pixel, Fig. 4 |
| AEDC-TR-76-58 [A] | VKF Tunnel A, spin-up | 0.22 | fit 0.005–0.03 | 1.05 × 10⁶ | about −18 | eye, Fig. 9a |
| Samardžić 2013 [S] | VTI T-38, forced | 0.2 | under 0.004 | not stated | −19.2 (−18.5 to −19.8) | pixel, Fig. 11 |
| Bunescu 2024 [I] | INCAS trisonic, free decay | 0.4 | up to 0.02 | not stated | −17.41 | Table 3 |
| Bunescu 2024 [I] | same | 0.95 | up to 0.01 | not stated | −17.14 | Table 3 |
| Murthy 1982 [M] | NAL tunnel | 0.48–0.99 | not known | not known | −17.6 to −19.4 (9 points) | pixel, in [I] |
| Murthy 1982 [M] | same | 1.09–1.11 | not known | not known | −14.7 to −16.2 (3 points) | pixel, in [I] |
| TM 2002-136 [D] | DRDC range, 2° cant | 0.77–1.11 | about 0.016 | 5.3–7.9 × 10⁶ | restricted (3 fits) | Table 10 |
| DREV-TM-9703 [F] | DREV range, 2°, 4° | 1.06–1.38 | 0.017, 0.033 | 7.0–9.2 × 10⁶ | restricted | Tables VIII–IX |

- **NOL** [N] (Regan 1964): a 0.75 in model spun on a sting to 50, 90 and 130 rev/s and let decay, fitted piecewise
  (PDF p. 8 on). Figs. 3 and 4 (PDF pp. 23–26) read on 200-dpi renders against their 0.1 grid (±0.003, ±0.15 converted).
  The text says C_lp depends strongly on spin rate at α = 0, that doubt was raised about the technique, and that a
  repeat at Mach 0.22 differed by at most 15% (PDF p. 12). Re: Table 1's 1.60 and 3.92 × 10⁶ per ft on 7.5 in.
  **Inference:** pd/2V = π·n·d/V with V about 80 and 257 m/s, from Table 1's Reynolds numbers.
- **AEDC** [A] (Jenke 1976): a 1.8 in model, fins canted 5°, spinning up freely; C_lp fitted over pd/2V 0.005 to
  0.03 below Mach 1.3 (PDF p. 15), ±4.1% at Mach 0.22 (PDF p. 17); V 254 ft/s, Re_d 1.05 × 10⁵ (PDF p. 12). By eye,
  Fig. 9a (PDF p. 42) puts the α = 0 point near −17.8 and NOL's near −17.4, matching NOL's 50 rev/s reading, so the
  45.84 factor holds subsonic too; the report calls the agreement good (PDF p. 18). Steady spin about 145 rad/s,
  pd/2V 0.043 (**Inference**). Drawn chord 0.94 d ([earlier note](basic-finner-transonic-tables.md#5-the-fin-chord)).
- **T-38** [S] (Samardžić et al. 2013): forced roll oscillation on a five-component balance, C_lp per p·d/(2V) on S
  and d (p. 190), 0.25° to 1.5° at 1 to 15 Hz (p. 191). Fig. 11 (p. 193), Mach 0.2, read by marker centroids: −18.5 to
  −19.8 over |α| ≤ 2°, mean −19.2; AEDC's replotted α = 0 point there reads −17.2. **Inference:** the roll rate peaks
  at 2.5 rad/s or less, so pd/2V stays under about 0.004 for any model under 0.2 m across.
- **INCAS** [I] (Bunescu et al. 2024): a 60 mm model spun to 1,000 rpm and released, C_lp per p·d/(2V) on the
  cross-section and diameter (eq. 2, p. 16). Table 3 (p. 22) averages six fits of each spin direction, which differ by
  ±10% at Mach 0.4 and ±17% at 0.95; the authors blame 0.4 mm shape errors and average them out (pp. 17, 21).
  **Inference:** pd/2V is about 0.02 (Mach 0.4) and 0.01 (0.95) at 1,000 rpm, falling to 0 as it decays.
- **NAL** [M] (H. Sundara Murthy 1982): not reached (AIAA, no access); read from its replot in [I] Fig. 10 (p. 22) by
  marker centroids. That figure's NOL points lie within 0.11 of NOL's Table 2 × 45.84 at six supersonic Mach numbers,
  so it uses this convention and reads to about ±0.2, though its Mach 0.77 NOL point (−13.3) matches none of NOL's
  three spin curves. One Mach 0.48 marker hides a second (about 0.495, −17.6); the three near Mach 1.1 overlap too.
- **DRDC** [D] and **DREV** [F]: free flight, no sting, low yaw. Spin about 60 deg/m at 2° cant ([D] PDF p. 24), 65
  and 125 deg/m at 2° and 4° ([F] PDF p. 28); pd/2V is that in rad/m times d/2 (**Inference**). DREV converted BRL
  799's values to its convention and found excellent agreement (PDF p. 27). Its Mach 1.377 shot, a 4° round, grew to
  10° of yaw (PDF p. 28). Both reports' terms bar redistribution (#619): only rounded percentage bands are given
  here. Re: [D] Table 11 at the fits' Mach numbers (4.1–8.1 × 10⁶ for all shots, PDF p. 20); [F] Table V.

## 2. Spin rate, and which sets are clean

- **A hobby rocket can spin slowly:** Valetudo with 1° of cant at 100 m/s (the aero page's example) has pd/2V 0.0069.
- **NOL's Mach 0.77 set isn't clean.** Its value doubles from 50 to 130 rev/s (−8.8 to −17.4), and no other set
  near that Mach reads below −14.7. Its Mach 0.22 set varies by ±9% across pd/2V 0.04 to 0.10, rising with spin.
- **The low-spin sets agree.** T-38 (pd/2V under 0.004), AEDC (0.005–0.03) and INCAS (0.02 to 0) span −17.1 to
  −19.2 at Mach 0.2 to 1.0; NAL adds −17.6 to −19.4. DRDC's range fits (0.016, Mach 0.77 to 1.11) agree with them.
- **Reynolds number** spans 1.0 × 10⁶ (AEDC, NOL) to about 8 × 10⁶ (DRDC) on length with no trend outside the ±10%
  scatter. **Inference:** a Reynolds-number effect larger than about 10% is not supported between these.
- **Near Mach 1.1** NAL dips to −14.7 to −16.2 while DRDC and DREV hold near the subsonic band; above it DREV rises
  toward a peak both place near Mach 1.4 ([D] printed p. 11, PDF p. 26; [F] PDF p. 26).

## 3. HPR Sim and Barrowman against the clean sets

HPR Sim's value is N·C_lp·k_R(B) from the aero page's equations, computed by this note, not flown: below Mach 0.8 the
fin's slope over its strips, a straight line to the supersonic value at M_s = 1.2, then linear theory with tip cones.
The same arithmetic reproduces the page's five published values (−31.62 at Mach 1.51 to −15.36 at 3.00) and −33.53 at
0.07 to 0.01, with k_R(B) = 1.1667 at τ = 3. Error is (HPR Sim − measured)/measured; positive means more damping.

| Mach | Set | Measured | HPR Sim | HPR Sim error | Barrowman's −34.21 |
|--:|---|--:|--:|--:|--:|
| 0.2 | T-38 | −19.2 | −33.70 | +75.5% | +78.2% |
| 0.22 | AEDC | −18.0 | −33.75 | +87.5% | +90.1% |
| 0.22 | NOL, by spin rate | −17.0 to −20.3 | −33.75 | +66.2% to +98.5% | +68.5% to +101.2% |
| 0.4 | INCAS | −17.41 | −34.33 | +97.2% | +96.5% |
| 0.48–0.99 | NAL, 9 points | −17.6 to −19.4 | −34.72 to −41.54 | +91.1% to +116.9% | |
| 0.77 | NOL, 130 rev/s | −17.4 | −37.00 | +112.6% | |
| 0.95 | INCAS | −17.14 | −40.71 | +137.5% | |
| 0.8–1.1 | DRDC and DREV range, 7 fits | restricted | not given | +100% to +150% | |
| 1.09–1.11 | NAL, 3 points | −14.7 to −16.2 | −43.80 to −44.18 | +170.7% to +200.0% | |
| 1.2–1.4 | DREV range, 4 fits | restricted | not given | +20% to +90% | |

- **Sign and size.** High everywhere below Mach 1.4, by +66% to +99% at low speed. The gap grows toward Mach 1.1
  because HPR Sim's fin slope rises with 1/β (+11% from Mach 0.2 to 0.8, then the join's climb to −46.3 at Mach 1.2)
  while the measured stays flat or dips. Between Mach 1.0 and 1.2 the join's slope has the wrong sign: the
  measurements fall or hold where it climbs, and they peak near Mach 1.4 where it has already fallen (**Inference**).
- **It joins the supersonic finding** ([§5](nicolaides-roll-damping-convention.md#5-what-it-moves-in-hpr-sim)):
  the excess peaks near Mach 1.1, then falls to +20% to +90% at 1.2–1.4, +20% at 1.51 and +5% at 3.00. The range
  fits share shots (DREV's multiple fits combine single-fit rounds): not independent points.

## 4. Does the 1.27 factor apply to Barrowman's −34.21?

No (**Inference**). ADR-031 read −34.21 off Fig. 5-7's computed curve, not its plotted points. The 1.27 lies between
those points and BRL 799's range rounds: a slip in passing data on. The curve is Barrowman's own calculation, and HPR
Sim gets −33.53 from his equations in the convention Fig. 5-7 states (πd²/4, d, p·d/(2V)), within 2.0%. Divided by
1.27 anyway (or × π/4) it would be −26.9, still +32% to +59% above the low-speed sets. So the method itself, the fin's
whole slope spread evenly over its strips, overstates this fin's damping; the check was not misread.

## 5. Causes of the gap, with sign and size

- **The body factor k_R(B)** (Barrowman eq. 3-122): ×1.167, all of it toward more damping. Without it HPR Sim would
  give −28.74 at Mach 0.07, still +42% to +69% above the low-speed sets. ADR-031's review notes it is a force ratio on
  a moment; weighted by the moment it would be 3.4% to 4.2% smaller (for the Arcas Robin's fins).
- **Even strips at the fin's mean slope, and no fin–fin interference.** Each strip lifts at the whole fin's slope per
  area; the four panels are summed as if alone. ADR-031's review expected this to overstate a short fin's subsonic
  damping. Sign: high. Size: what is left after k_R, a factor of 1.42 to 1.69 at low speed (**Inference**); no source
  read here separates the two parts: **unbounded** apart.
- **Fin section and chord.** The wedge fins (8% thick) can't be drawn, and AEDC's 0.94 d chord gives about 6% less fin
  area than DREV's 1.0 d. Sign of the thickness effect subsonic: unknown, **unbounded**; the chord lowers AEDC's value
  a few percent, which widens HPR Sim's apparent excess at Mach 0.22 by about as much (the other sets don't share it).
- **The measured side's own spread.** Spin rate (NOL ±9% at Mach 0.22), spin direction (INCAS ±10%), α within ±2°
  (T-38 ±4%), reading by eye (±0.2 to ±0.5). Together about ±10%, a fifth of the gap or less.

**The steady spin.** HPR Sim's steady roll rate is set by the ratio of damping to forcing:
C_lp/C_lδ = −2·k_R·∫ξ² dA/(A_fin·(r_t + y_MAC)·d·k_T), −2.70 for the Basic Finner with k_T(B) = 0.935 at τ = 3 (this
note's evaluation of Barrowman eq. 3-105). Eastman's empirical correlation, which ARL-TR-846 states holds at all
speeds, puts it at −2.15·y_c/d [R eqs. 4–5, PDF pp. 13–14]: −2.15 here at y_c = 1.0 d, 1.8% to 4.6% above BRL 711's
seven measured ratios (2.055 to 2.111, Table 7, PDF p. 21; ARL's own predicted column implies y_c ≈ 0.955 d). Against
those, HPR Sim's ratio is +28% to +32% high, so its steady spin reads 22% to 24% low at matched forcing (**Inference**).
**AEDC measured the forcing too:** C_l0 ≈ 0.77 at α = 0, Mach 0.22, 5° cant (Fig. 9a, PDF p. 42, by pixel). HPR
Sim's is 4 × 3.337 (one fin, per radian) × k_T 0.935 × 0.0873 rad = 1.089 on this note's geometry, +41%, part of it
AEDC's 0.94 d chord. Measured, the steady pd/2V is 0.77/17.8 = 0.043, as observed, so bearing friction is small; HPR
Sim's 1.089/33.75 = 0.032 reads about 25% low. Same sign as the Arcas Robin's +14% to +48% forcing at Mach 1.5–1.8.
**Apogee:** roll rate doesn't enter the trajectory directly; any effect through roll-pitch coupling is **unbounded**.

## Sets screened and not used

- **AD-A068327** [C] (Collins 1978, AEDC 4T): tested the Basic Finner (2.5° cant, 4.5 in) at Mach 0.22 to 1.15
  (Table 1, PDF p. 32) but shows only the Modified Basic Finner (Fig. 5, Table 6); the rest is on AEDC microfilm.
- **AFATL-TR-68-82** [T]: roll lock-in and a Mach 3.5 analysis (Table III); no subsonic C_lp. **ARL-TR-846** [R]:
  its Basic Finner rows are BRL 711's, Mach 1.59 to 2.68 (Table 7).
- **Not reached:** West, AFATL-TR-81-87 (no copy on the Internet Archive or by web search on 2026-10-29; DTIC 403);
  Bhagwandin, J. Spacecraft and Rockets 53(4), 720–729, 2016, doi 10.2514/1.A33419 (AIAA, no access; a CFD
  prediction, not a measurement); Murthy's note itself, and a 1986 item of the same title in J. Spacecraft and
  Rockets 23(1), 119, doi 10.2514/3.28073, no author listed, which may be an erratum to it.

## What the project can use

- **Terms.** NAVORD 6652 (released "without restrictions"), AEDC-TR-76-58, AD-A068327, AFATL-TR-68-82 (changed to
  public release) and ARL-TR-846 are open US Government reports: readings can be committed with method. Samardžić 2013
  is "All rights reserved" (p. 189), Bunescu 2024 CC BY-NC-ND 4.0 (p. 15): cite their values, no fixture. Murthy 1982
  is AIAA's: cite through [I]. DRDC and DREV stay local (#619). [S] and [I] are cached under `refs/research/`.
- **Suggestion: replace the row's Mach 0.07 entry** with measured points: AEDC Fig. 9a (Mach 0.22, −18 ± 0.7) and
  NOL Fig. 3's 50 rev/s curve (Mach 0.22, −17.0 ± 0.15) as committed readings, and NOL's other curves as a spin-rate
  band. Cite T-38, INCAS and NAL beside them. Expected row: about +66% to +99% at Mach 0.2 to 0.4, against −2.0%
  today. Keep −34.21 as the method check it is, labeled as theory.
- **Score DRDC's three range fits locally** (statistics only, #619), inside the §3 band. **accuracy.md's "nothing
  measured below Mach 1.5"** and the aero page's "a subsonic spin may read low" can then give the sign and size:
  damping about twice the measured, steady spin about 22% to 25% low. A model change (not proposed here) sized on
  these sets would need a second measured shape, or it fits one body.

## Open questions

- How the factor of 1.4 to 1.7 splits between strip loading and panel interference (a lifting-surface chart would).
- Whether AEDC's +41% forcing excess holds at the 1.0 d chord, and above Mach 0.22 (no public forcing set found).
- Why NAL dips near Mach 1.1 while the ranges don't; Murthy's note and the 1986 item would say.

## Sources (read 2026-10-29)

1. **[N]** F. J. Regan, NAVORD Report 6652, 1964. <https://archive.org/details/DTIC_AD0600975> (`refs/papers/`).
2. **[A]** L. M. Jenke, AEDC-TR-76-58, 1976. <https://archive.org/details/DTIC_ADA027027> (`refs/papers/`).
3. **[C]** J. A. Collins, *Verification Test of the AEDC High Alpha Roll Dynamics System*, 1978.
   <https://archive.org/details/DTIC_ADA068327> (`refs/papers/`).
4. **[S]** M. Samardžić et al., FME Transactions 41(3), 189–194, 2013.
   <https://simlab.mas.bg.ac.rs/_media/istrazivanje/fme/vol41/3/03_msamardzic.pdf>
5. **[I]** I. Bunescu et al., INCAS Bulletin 16(1), 15–24, 2024, doi 10.13111/2066-8201.2024.16.1.2.
   <https://bulletin.incas.ro/files/bunescu_pricop_stoican_hothazie__vol_16_iss_1.pdf>
6. **[M]** H. S. Murthy, J. Spacecraft and Rockets 19(1), 86–87, 1982, doi 10.2514/3.62210 (landing page only).
7. **[D]** A. Dupuis, DRDC Valcartier TM 2002-136, 2002, as in
   [basic-finner-transonic-tables.md](basic-finner-transonic-tables.md#sources-read-2026-10-20-unless-noted).
8. **[F]** A. D. Dupuis, W. Hathaway, DREV-TM-9703, 1997. <https://archive.org/details/DTIC_ADA636861>
9. **[T]** J. D. Nicolaides et al., AFATL-TR-68-82, 1968. <https://archive.org/details/DTIC_AD0852114>
10. **[R]** A. G. Mikhail, ARL-TR-846, 1995. <https://archive.org/details/DTIC_ADA298617>
11. J. S. Barrowman, NASA/TM-2001-209983 (MS thesis, 1967), eqs. 3-105 and 3-122.
    <https://ntrs.nasa.gov/citations/20010047838>
