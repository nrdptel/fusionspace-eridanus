# The discharge of a small vent hole with flow across its exit

This note asks what discharge coefficient Cd a hobby bay's vent has while the outside air sweeps across it, and
how that moves the lag constant C of [altimeter-port-lag](altimeter-port-lag.md) ([P]) and #872's vent term, for
the parent's invented bays. It reads NASA vent and liner tests, film-cooling abstracts and orifice handbooks.
**Inference** marks this note's own algebra; "by eye" marks figure readings.

**In short.** A stream across the exit makes the vent far harder to blow through than the parent's 0.62 orifice.
At low flow the sign is settled by every exit-side test found; the size rests on two rigs and liner fits.

- **Rohde's low Cd doesn't carry over (§1).** TN D-5467's cross flow is on the inlet side and its Cd is taken
  against the duct's total pressure; the parent's §6 reading of it doesn't apply to a bay.
- **At low flow the drop is linear (§2).** Below a hole speed of about a tenth to a third of the stream's,
  Δp = k·ρ·w·u, with w the stream and u the mean hole speed. Two independent rigs: k 0.19 to 0.41 at Mach 0.15 to
  0.44 and about 0.46 to 0.66 at Mach 0.7 to 0.9 (**Inference** from [BZ] Fig. 7 and [W1]'s tables). Liner
  correlations give 0.17 to 0.5 [HR] and a faster rise above Mach 0.4 to 0.5 [SY].
- **Cd falls toward zero, as the fourth root (§4, Inference).** Against the outside static, Cd = I^(1/4)/√(2k),
  I the jet-to-stream head ratio: 0.09 to 0.16 at I = 10⁻⁴, 0.28 to 0.50 at 10⁻². No test found goes below a
  mass-flux ratio of 0.008 [BZ], and none showed outside air drawn in through an outflowing hole.
- **C grows ×1.7 to ×84 (§5, Inference).** The vent's steady Cp becomes about 2·k·S + C, S the hole-to-flight speed
  ratio. Makers' rules move from C = 10⁻⁴ to 0.02 to C_eff = 0.0025 to 0.11: 11 to 48 m of steady lag at 300 m/s
  in 98 mm bays, not 0.5 m. With no cross flow, a short hole's Cd is 0.63 to 0.88, not 0.62 (§3); that lowers C
  by up to half, which the cross-flow term swamps everywhere but the 38 mm bays.
- **A separate offset (§6).** With no flow, a flush cavity sits −0.015 to +0.021 of the dynamic pressure q off the
  stream static on a flat wall at Mach 0.7 to 0.9 [W1] [N2]: a vent-site Cp of either sign, not lag.

## 1. Rohde's data are inlet-side (source-stated, with Inference)

Rohde's cross flow runs past the hole's inlet, not its exit [R1] p. 1. The ideal jet velocity uses the main duct's
total pressure [R1] p. 9, and the velocity-head ratio (P_T − p_j)/(P_T − p_d) is 1 or more by construction [R1]
p. 10. For exit-side data he defers to NACA outlet tests [R1] Appendix, p. 26. **Inference:** near a ratio of 1 his
ideal velocity is the stream's own, so his Cd under 0.2 at low ratios (Fig. 8, p. 15) is close to the jet-to-stream
speed ratio, not a bay's Cd.

## 2. Exit-side tests: the drop goes linear at low flow

- **Budoff and Zorumski (NASA TM X-2361, 1971) [BZ].** Aluminum plates 3.2 mm thick, 2.18% open area: one 6.3 mm
  hole, four of 3.2 mm, 16 of 1.6 mm or 64 of 0.8 mm, so depth over bore 0.5 to 4 [BZ] p. 7. A stream at Mach
  0, 0.15, 0.29 and 0.44 crossed the plate; air was blown into it or drawn out at 2 to 400 cm/s averaged over the
  plate [BZ] p. 6, about 0.9 to 180 m/s in the holes (**Inference**). Δp is static to static [BZ] p. 3, the bay's
  own definition, and resistance R = Δp/V [BZ] p. 6. Flow into the stream, the bay's direction, met more
  resistance than flow out, the gap widening with stream speed [BZ] pp. 7 to 9.
- **What Fig. 7 shows.** With no stream, R falls to zero as the flow does, the orifice law. With a stream, R levels
  off at low flow at a value set by the stream's Mach number [BZ] Fig. 7, p. 8. Plateau R/ρc for flow into the
  stream, by eye: 2.6, 5.5 and 8.2 at Mach 0.15, 0.29 and 0.44 (one hole); 2.4, 4.9, 7.8 (four); 1.8, 3.4, 5.5
  (16); 2.3, 3.9, 3.9 (64; its fall at Mach 0.44 the authors explain only by guesses, p. 9).
- **As k (Inference).** A flat R means Δp = k·ρ·w·u, u the mean hole speed, with k = (R/ρc)·σ/M: 0.38 to 0.41,
  0.35 to 0.39, 0.26 to 0.27, and 0.19 to 0.33. Flow out of the stream, one hole: 0.26 to 0.28. The plateau runs
  from the lowest point, u/w about 0.008, to u/w 0.1 to 0.3, where the square law takes over (Fig. 7's bend). No
  zero-flow offset stands out of the scatter. R is normalized by a standard ρc; at Mach 0.44 k may be 13% low.
- **Acoustic-liner correlations agree in form.** Grazing flow adds a steady resistance 0.3·M/σ, normalized by ρc
  [HR] eq. (5), p. 4, or M/(σ·(2 + 1.256·δ*/d)) with δ* the boundary layer's displacement thickness and d the bore
  [HR] eq. (7), p. 5. **Inference:** k = 0.3, or 0.5 at δ*/d = 0 falling to 0.17 at δ*/d = 3. Syed et al. measured
  sheets to Mach 0.8, flow both ways; their resistance tracks Rice–Heidelberg in δ*/d but grows faster above Mach
  0.4 to 0.5 [SY] Summary, §3.4 to 3.5, pp. 17 to 18. Above Mach 0.9, and for drilled tube walls: no data.
-  **Saturn venting tests (Walters et al., NASA CR-61241, 1968) [W1].** A 19 mm flat-plate hole [W1] p. 1-3, Mach 0.7 to
  1.9, boundary layers about 0.4 to 4 in thick, mass-flux ratio r = ρ_j·u_j/(ρ·w) from 0.05 up [W1] pp. 2-7 to 2-8.
  K_ps, the coefficient against the plate static ahead of the jet [W1] p. 2-11, App. A, in run 20 (Mach 0.70): 0.230,
  0.346 and 0.611 at r 0.051, 0.105 and 0.318 [W1] tabulated data. Across runs 19, 20, 185 and 186, K at r ≈ 0.05 is
  0.20 to 0.25. **Inference:** below r 0.3, r ≈ k_A·Δp/q with k_A 0.66 to 1.11, the linear law with k = 1/(2·k_A) =
  0.45 to 0.76; through the origin, 0.46 to 0.66. The thicker layer gives the larger k_A (more flow), as
  Rice–Heidelberg predicts. The lip port's K at r ≈ 0.05 is 0.20 to 0.27: no local reference rescues Cd.
-  **Other vent data agree.** Aircraft outlets: K is zero at zero flow, its slope steepest there; the lowest points sit
  near r 0.2 [N1] p. 5, Fig. 5(a). Titan's 25 mm vent: K falls as the pressure ratio nears 1 [J1] p. 4. Ares I design
  curves meet K = 0 at a pressure ratio of 1 for Mach 0.3 and up, the still-air curve near 0.6 [A1] Fig. 7 (by eye).
  SP-8060's ±10% for "flow interference" [S1] Table I, p. 14 fits high flow ratios only (Inference).
-  **Film-cooling holes: abstracts only.** Hay, Lampard and Benmansour varied cross flow on both sides to Mach 0.4 and
  pressure ratios 0 to 2 [HLB]; Gritsch, Schulz and Wittig took the exit stream to Mach 1.2 at pressure ratios 1 to 2.25
  [GSW01]. Exit cross flow usually lowers Cd but can raise it, the "crossover", which a measured local exit static
  removed [RO2]. Their full texts are paywalled; no Cd is quoted from them. **Inference:** a bay's I of 10⁻⁴ to
  10⁻² needs a pressure ratio within 2.5×10⁻⁴ to 0.03 of 1 (§4's Cd), where their points couldn't be checked.
  Their convention, ideal flow to the free-stream static [HE] eq. (1), is the bay's.

## 3. A short hole's Cd with no cross flow

- **The bay jet's Reynolds number is low (Inference).** With the mean hole speed u = V·w/(n·H·A), which doesn't
  depend on Cd, a 2 mm wall and the parent's §2 areas, hole Re runs 30 to 610 (sea level and 3 km, n 1 to 1.4,
  100 to 300 m/s). The 38 mm PerfectFlite-formula bay drills four 0.27 mm holes, depth over bore 7.4.
- **Thick holes reattach.** Cd rises from 0.61 to a peak of 0.81 near depth over bore 2, then starts to fall, as
  Rohde reports from Lichtarowicz et al. [R1] p. 17. Idelchik's thick-edged orifice, from its table: 0.61 at 0.3,
  0.68 at 0.67, 0.76 at 1.0, 0.81 at 2 to 4 [ID1] diagram 4-11, p. 137. NACA TN 3663 at zero cross flow agrees within
  0.01 at 0.32 and 0.67 at pressure ratios near 1 [DG] Fig. 2, p. 18, by eye.
-  **Low Re raises it more, less surely.** For a thin plate Idelchik's low-Re method gives a hump, Cd 0.58 at Re 25,
  0.72 at 200, 0.60 again at 10⁵ [ID1] diagram 4-10, p. 136 (from its tables, Inference). For thick holes it reaches
  0.82 to 0.92 at Re 100 to 4,000; values above 0.82 are unverified here. Lichtarowicz et al. measured to Re 1 [LDM],
  but only the abstract was read.
- **Per row (Inference).** Cd 0.81 to 0.88 for the 38 and 54 mm rows, 0.72 to 0.85 for 75 mm, 0.63 to 0.76 for
  98 mm, 0.63 to 0.85 for Eggtimer's and Stroick's rules: C ×0.50 to ×0.97 of the parent's. The parent's split,
  an orifice term plus a Poiseuille term, is the standard form [SY] eqs. (1) to (3), pp. 3 to 4: no double count.
- **Edges.** A rounded inlet raises Cd: 0.66 at radius over bore 0.04, 0.85 at 0.2 [ID1] diagram 4-12, p. 138
  (its table). A drill's breakout burr sits on the bay side, the inlet, and lowers Cd by an amount no source bounds:
  the one cause found that takes Cd below 0.62.

## 4. Cd against the head ratio (Inference)

The combined drop, Δp = k·ρ·w·u + ρ·u²/(2·Cd0²), has the linear part from §2 and the still-air part from §3;
Syed et al. fit liners with the same two terms [SY] eq. (14), p. 14. Where the linear part dominates, the effective Cd
against the outside static is √(u/(2·k·w)) = I^(1/4)/√(2k), with I = (Cd·√(2Δp/ρ)/w)² the head ratio.

| I | k = 0.2 | k = 0.66 | Check |
|---|---|---|---|
| 10⁻⁴ | 0.16 | 0.09 | below every test |
| 2.5×10⁻³ (r 0.05) | 0.35 | 0.19 | [W1]: 0.20 to 0.25 |
| 10⁻² | 0.50 | 0.28 | [W1] at r 0.1: 0.29 to 0.35 |

So yes, Cd falls toward zero at low head ratio, but as I^(1/4), not to zero at a finite ratio. The two rigs give
the band's two ends: Budoff's k at Mach 0.44 or less, Walters' at 0.7 to 0.9, on different rigs.

## 5. What it does to C and the parent's §2 table (Inference)

The bay's outflow fixes S = u/w = V/(n·H·A) = 0.62·√C (parent's C, n = 1; n = 1.4 divides S): independent of Cd.
The linear law gives Δh = k·S·w·w_b/g, a steady Cp of 2·k·S. Low end: the larger of 2·0.2·S and C with §3's top
Cd; high end: 2·0.66·S + C (Cd 0.62, as if burred). For the 38 mm rows, inside [W1]'s data, the high end is
[W1]'s own K at r 0.089 and 0.059, 0.27 to 0.31 and 0.22 to 0.25, which gives (0.62/K)².

| Bay, rule (holes) | 10⁴·C, parent | S | C_eff | × C | Steady lag, 300 m/s | Parent |
|---|---:|---:|---:|---:|---:|---:|
| 38, PerfectFlite (4) | 206 | 0.089 | 0.036 to 0.11 | 1.7 to 5.3 | 160 to 500 m | 95 m |
| 38, Missile Works (3) | 91 | 0.059 | 0.024 to 0.072 | 2.6 to 7.9 | 110 to 330 m | 42 m |
| 54, PerfectFlite (4) | 28 | 0.033 | 0.013 to 0.046 | 4.7 to 16 | 60 to 210 m | 13 m |
| 54, Missile Works (3) | 12.5 | 0.022 | 0.0088 to 0.030 | 7.0 to 24 | 40 to 140 m | 5.8 m |
| 75, PerfectFlite (4) | 4.9 | 0.014 | 0.0055 to 0.019 | 11 to 38 | 25 to 85 m | 2.2 m |
| 75, Missile Works (3) | 2.2 | 0.0092 | 0.0037 to 0.012 | 17 to 56 | 17 to 57 m | 1.0 m |
| 98, PerfectFlite (4) | 1.2 | 0.0068 | 0.0027 to 0.0091 | 23 to 76 | 12 to 42 m | 0.53 m |
| Any, Stroick at Mach 1 (3) | 1.6 | 0.0078 | 0.0031 to 0.011 | 20 to 66 | 14 to 48 m | 0.73 m |
| Any, Eggtimer (3) | 0.98 | 0.0061 | 0.0025 to 0.0082 | 25 to 84 | 11 to 38 m | 0.45 m |

- **The law's scaling changes.** The lag now goes as 1/(vent area), not 1/area², so a well-vented bay gains the
  most. It still goes as w², so on a vertical climb a vent is still a steady Cp, now of 2·k·S. k may rise with Mach
  (0.2 to 0.4 below Mach 0.44, 0.5 to 0.7 at 0.7 to 0.9, on two rigs; [SY]), so C_eff may grow in the climb.
- **The time constant halves for the same lag.** With Δh linear in w_b, the bay relaxes as first order with
  τ = k·S·w/g = C_eff·w/(2g), half the orifice law's C·w/g: 40 to 140 ms for 98 mm makers' bays at 300 m/s.
  The parent's §3 to §4 top-speed results follow τ and were simulated with the orifice law, so they need a rerun.
- **Against #872's 0.02 and the logs.** Every 38 and 54 mm makers' bay reaches 0.02 or more at the high end; 75 and
  98 mm bays reach 0.0025 to 0.019. The logs' low readings run Cp +0.005 to +0.15 [P] §2; the parent found only
  under-vented bays reach them; now makers' vents in small bays do (a consistency, not a test). n = 1 here.

## 6. The no-flow offset

- **Flat wall.** With no flow, [W1]'s cavity sat +0.005, +0.021, +0.005 and −0.008 q off the upstream static
  (±0.005 q from 1 psf printing); thin-plate outlets −0.015 to +0.005 q at Mach 0.7 and 0.85, up to +0.055 q at
  Mach 1.3 [N2] Fig. 21, p. 35, by eye. The wall around the hole reads ±0.05 q: the lip +0.03 to +0.045 q, a port
  just downstream down to −0.05 q [W1]. Recessed outlets sit 0.10 to 0.20 q low [N2] p. 8.
-  **Not everywhere.** None stands out of Budoff's scatter [BZ] (Inference); Syed et al. found about 1.4% of q at Mach
  0.3 and subtracted it as instrument bias [SY] §2.3.2, p. 9 (by eye, Fig. 11a). Its sign and cause are unbounded.
- **What it means (Inference).** An offset is a static Cp at the vent site, of either sign, the size of #872's
  0.02 bound. It belongs with the port term, not the lag; a bay sitting between the offset and Δp = 0 draws air in.

## 7. What no source here bounds

- **Below the data.** No outflow point sits below r 0.008 [BZ] at Mach 0.44 or below r 0.05 [W1] at Mach 0.7 to
  0.9. Every bay but the 38 mm ones sits below 0.05; the linear law there is a fit, not a measurement.
-  **A thick layer over a small hole.** At 200 m/s it is 8 to 19 mm thick 0.5 to 1.5 m back (**Inference**), δ/d 2 to
  70, past [W1]'s 5.3. Rice–Heidelberg's k falls with δ*/d (0.08 to 0.15 for 0.3 mm bores, δ* = δ/8); Budoff's 0.8
  mm holes gave k at or above that formula's, so it may not.

## What this suggests

-  **#872's vent term.** Model the vent as Δp = k·ρ·w·u + ρ·u²/(2·Cd0²) with k 0.2 to 0.7 (perhaps 0.1 for
  sub-mm holes, §7) and Cd0 0.62 to 0.88, not as an orifice with a smaller Cd. Taking k 0.7 and Cd0 0.62 errs toward
  more lag, the side a top-speed bound takes. A 0.02 vent bound then covers makers' vents in 75 and 98 mm bays only.
- **The parent note.** Its §6 reading of Rohde and its §2 lag table assume no cross flow; §5 here replaces them.
  Its §4 simulations should be rerun with the linear law's halved τ.
- **The readings' known risk.** Makers' vents lag 11 to 48 m at 300 m/s in 98 mm bays and hundreds of meters in
  38 mm bays, not under 1 m; add the vent-site offset, ±0.02 q, as a static error of unknown sign.

## Open questions

- Cd of a drilled hole in a body tube's wall blowing out at r below 0.05, Mach 0.3 to 0.9. A blow-down test of
  one bay in a car's slipstream or a tunnel would measure k and the offset together.
- The full texts of [HLB], [GSW01] and Gritsch et al.'s correlation (AIAA J. 36(6), 1998), for their lowest
  pressure-ratio points with a 90° hole. The lead "Rowbury, Chew and Hills" wasn't found; Rowbury's co-authors
  on cross flow are Oldfield and Lock [RO1] [RO2].
- Lichtarowicz et al.'s low-Re data [LDM]; Walters' final report (NASA CR-98482) and NOL-TR-61-125: not reached.

## Sources

Read 2026-11-05. Printed page numbers. NTRS marks its reports public use permitted; [A1] (AIAA) and the rest: cite.
- [BZ] M. Budoff, W. E. Zorumski, NASA TM X-2361 (1971). <https://ntrs.nasa.gov/citations/19710027643>
- [W1] W. P. Walters, R. M. Glasgow, J. M. Baker, NASA CR-61241 (1968). <https://ntrs.nasa.gov/citations/19680026493>
- [HR] L. J. Heidelberg, E. J. Rice, L. Homyak, NASA TP-1613 (1980). <https://ntrs.nasa.gov/citations/19800012838>
- [SY] A. A. Syed, J. Yu, H. W. Kwan, E. Chien, NASA/CR-2002-211749. <https://ntrs.nasa.gov/citations/20020065231>
- [R1] J. E. Rohde, H. T. Richards, G. W. Metger, NASA TN D-5467 (1969). <https://ntrs.nasa.gov/citations/19690028630>
- [N1] W. J. Nelson, P. E. Dewey, NACA RM L52H20 (1952). <https://ntrs.nasa.gov/citations/19930087254>
- [N2] P. E. Dewey, A. R. Vick, NACA TN 3466 (1955). <https://ntrs.nasa.gov/citations/19930084212>
- [J1] A. L. Johns, M. L. Jones, NASA TM X-2995 (1974). <https://ntrs.nasa.gov/citations/19740011801>
- [S1] *Compartment Venting*, NASA SP-8060 (1970). <https://ntrs.nasa.gov/citations/19710018690>
- [A1] Q. Wang, S. Arner, Ares I systems tunnel venting, AIAA (2009). <https://ntrs.nasa.gov/citations/20090034248>
- [DG] R. T. Dittrich, C. C. Graves, NACA TN 3663 (1956). <https://ntrs.nasa.gov/citations/19930084890>
- [ID1] I. E. Idel'chik, *Handbook of Hydraulic Resistance*, AEC-tr-6630 (1966), diagrams 4-10 to 4-12.
  <https://www.nrc.gov/docs/ML1220/ML12209A041.pdf>
- [HE] C. Heneka, A. Schulz, H.-J. Bauer, Proc. IMechE A 225(7), 985 (2011).
  <https://publikationen.bibliothek.kit.edu/1000040273>
- [HLB] N. Hay, D. Lampard, S. Benmansour, J. Eng. Power 105(2), 243 (1983), abstract. doi:10.1115/1.3227408
- [GSW01] M. Gritsch, A. Schulz, S. Wittig, J. Turbomach. 123(4), 781 (2001), abstract. doi:10.1115/1.1397306
- [RO1], [RO2] D. A. Rowbury, M. L. G. Oldfield, G. D. Lock, J. Turbomach. 123(2), 258 and 123(3), 593 (2001).
- [LDM] A. Lichtarowicz, R. K. Duggins, E. Markland, J. Mech. Eng. Sci. 7(2), 210 (1965), abstract.
- [P] [altimeter-port-lag](altimeter-port-lag.md) §2 and §6.
- [#872](https://github.com/nrdptel/fusionspace-eridanus/issues/872), the top-speed bound's port and vent terms.
