# Transonic pressure drag of fineness-3 noses, measured without a faired tare

**In short: three sets that need no tare put a 3- to 4-caliber nose's pressure drag 0.011 to 0.020 above Stoney's
curves at Mach 1.00–1.02, within 0.007 of them from Mach 0.90 to 0.99, and 0.003 to 0.024 below them at Mach
1.05–1.10.** The sets are a 3:1 tangent ogive's 12 nose orifices in the Langley 8-foot tunnel (TN D-3516), a
fineness-4.08 cone's tabulated pressures at 0.005% blockage (TN 4233), and a free-flight 3.5-caliber ogive with
measured base pressure (RM L52E06). Integrated here, the 3:1 ogive drags −0.001 to 0.007, 0.049 to 0.057 and 0.094
to 0.102 at Mach 0.90, 1.00 and 1.10. Stoney's 3:1 parabola, as HPR Sim carries it, gives 0.000, 0.037 and 0.107;
MIL-HDBK-762's ogive 0.007, 0.052 and 0.109; HPR Sim's current tangent ogive 0.062, 0.164 and 0.234. So the ±0.02
left on Stoney's curves is the right size but has a sign: low at Mach 1.00–1.02, high just past the window. If
the 8-foot tunnel's "Mach 1.00" is free-air Mach 0.96–0.99, the ogive's gap there grows to +0.020 to +0.047. How
sure: high on the sign at Mach 1.0, which three methods share; medium on sizes, read off plots and tables (±0.003 to
±0.01) with tunnel Mach in doubt by up to 0.036. It moves [#67][i67]: the fix's direction stands; on the 3:1 ogive
these gaps are a tenth to a fifth of the current formula's excess. **Inference** marks this note's readings and
arithmetic.

## 1. The data sets

| Id | Report; facility | Blockage | Nose, fineness | Mach in 0.9–1.1 | Measured |
|---|---|--:|---|---|---|
| CM | D-3516 [S1]; Langley 8 ft, slotted | 0.15–0.18% | tangent ogive, 3 | 0.90, 1.00, 1.10 | 12 orifices |
| MI | X-1656 [S2]; Lewis 8 × 6 ft, porous | 0.73% | tangent ogive, 3 | 0.905, 0.996, 1.108 | 2 tap rows, integrated |
| PA | TN 4233 [S3]; Ames 14 ft; 2 × 2 ft | 0.005%; 0.25% | cone, 4.08 | 13, 15 points, 0.899–1.1 | 11 orifices |
| HA | RM L52E06 [S4]; rocket free flight | none | ogive, 3.5 | continuous | total less base |
| ST | RM L53K17 [S5], R-100 [S6]; gun | none | cone, parabola, 3 | continuous | total less a tare |

CM's blockage is 0.15–0.18% (**Inference**, below). Reynolds number: CM 0.8–6.0 × 10⁶ per ft [S1] p. 1; MI not
given; PA 2.4 × 10⁶ on the cone; HA 15–45 × 10⁶ and ST 8–15 × 10⁶ on body length. Only HA measured base
pressure; the integrated sets need none, and ST has none.

- **CM.** A 4.096 in body 50.0 in long, nose 12.29 in (3.00 calibers), ogive radius 37.89 in, 37.8° included tip
  angle, smooth and tripped near the tip [S1] pp. 2–4, Fig. 1 p. 19. Nose orifices sit at 1.01 to 11.80 in [S1]
  Table II p. 18. Pressures did not change with stagnation pressure, so Reynolds number had no effect [S1] p. 6. Mach
  numbers 0.70, 0.90, 1.00, 1.10, 1.20 [S1] p. 5. The report calls the slotted section square without giving its
  size [S1] p. 3; blockage is this note's arithmetic for a section 7 to 8 ft across (**Inference**).
- **MI.** An 8 in body, 3:1 tangent ogive with an 18.9° initial half-angle, taps about 2 in apart in a vertical and a
  horizontal row [S2] pp. 4, 13; the modified 8-ft, 3.1%-porosity section, Fig. 4(e)–(g), pp. 19–20.
- **PA.** A 5.50 in cone on a 1.347 in cylinder; orifices 1–11 at x/l 0.082 to 0.994 [S3] Fig. 1 p. 33. Cp is
  tabulated to 0.001 for both tunnels [S3] Table I(a), (b), p. 21; random error ±0.002 in Cp and ±0.004 in Mach at
  Mach 1.0 [S3] p. 6. Reynolds number on the cone length, Mach from plenum pressure [S3] p. 5.
- **HA.** Configuration A, finless, open hollow base [S4] pp. 3–4; fore drag is the flight's total drag less
  measured base drag [S4] p. 6, Fig. 12 p. 19. Hart cites tunnel pressures that make the nose's pressure drag
  negligible below Mach 0.9.
- **ST.** Stoney's tare (afterbody, fins and base) at Mach 1.2 is the cone model's measured total less calculated
  friction and theoretical cone pressure drag; it is held at 0.025 subsonically, and faired between at drag-rise and
  peak Mach numbers taken from a body with a 7.13-caliber parabolic nose [S5] pp. 5–6.

Reached but outside the question: Solomon's 20° to 30° cones (fineness 1.37 and below, Re 55,000–143,000) [S7] p. 3;
Lee and Summers' fineness-5 Kármán nose, balance foredrag with friction [S8] p. 3; TM X-1655's 10° cone (fineness
2.84), plotted only, used here for its blockage figure [S9]; Kell's free-flight cone-cylinder (R&M 3290), whose
abstract reports afterbody pressures only.

## 2. Forebody pressure drag, Mach 0.9 to 1.1

On the nose's base area, pressure only, $C_{D,p} = \int C_p \, d(r/R)^2$ from tip to shoulder (**Inference**: the
integrals are this note's). The Mach numbers are the tunnels' indicated values; parentheses give the run's own Mach.

| Id, nose | 0.90 | 0.95 | 1.00 | 1.05 | 1.10 | Where |
|---|--:|--:|--:|--:|--:|---|
| CM, 3:1 ogive | −0.001 to 0.007 | not run | 0.049 to 0.057 | not run | 0.094 to 0.102 | [S1] Fig. 5 p. 27 |
| MI, 3:1 ogive, 0.73% | 0.017 (0.905) | not run | 0.024 (0.996) | not run | 0.101 (1.108) | [S2] Fig. 4 |
| PA, cone 4.08, 0.005% | 0.021 (0.899) | 0.053 (0.964) | 0.086 (1.001) | 0.094 | 0.088 | [S3] Table I(b) |
| PA, cone 4.08, 0.25% | 0.020 (0.901) | 0.049 (0.960) | 0.075 (1.001) | 0.102 | 0.091 | [S3] Table I(a) |
| HA, ogive 3.5, flight | 0.000 to 0.005 | 0.003 | 0.042 | 0.067 | 0.085 | [S4] Fig. 12 p. 19 |

- **CM, how read.** Symbol centers were found as the interiors of the plotted symbols on a 300-dpi render, four by
  eye. Each curve's zero is its gridline; the printed labels sit 12 px (0.007 in Cp) lower, which gives the range's
  low end. The tip ahead of 1.01 in carries 0.008 to 0.011 of each total. Holding the tip or using a cubic moves a
  total by at most 0.001. The rise from Mach 0.9 has no zero ambiguity: 0.050 at 1.00, 0.095 at 1.10, 0.121 at 1.20.
  At Mach 0.70 and 1.20 the ogive drags 0.005 to 0.012 and 0.120 to 0.128.
- **PA inside the fairing window** (0.005%): 0.062 at 0.980, 0.071 at 0.991, 0.097 at 1.008, 0.098 at 1.020, 0.096 at
  1.032, with a peak near Mach 1.01–1.04. Each total holds orifice 1 to the tip and orifice 11 to the shoulder
  (±0.003). At Mach 0.600 and 0.797 it is −0.001.
- **MI, how read.** p/P₀ read off ruler overlays, both rows averaged, ±0.003 in p/P₀, which is ±0.008 in Cp; Cp
  from the isentropic free-stream ratio at the stated Mach. Totals ±0.01.
- **HA, how read.** Fore drag by pixel column, ±0.002: 0.076 at Mach 0.80, 0.077 at 0.95, 0.114 at 1.00, 0.137 at
  1.05, 0.153 at 1.10. Nose pressure drag is fore drag less 0.076 (all friction, per Hart), with that friction
  lowered as Hart's turbulent estimate falls (0.002 at 0.95 to 0.008 at 1.10). Totals ±0.01.

## 3. Against Stoney's curves

Stoney's parabola is R-100 Fig. 12(a) as HPR Sim's curves carry it (`hpr_aero::nose_drag`); his cone is the table
in that module's unit test, and at Mach 1.05, 0.157 ± 0.005, is read off the print here. The whole section is this
note's arithmetic (**Inference**). The cone and the 3.5-caliber ogive are compared with Stoney's 3:1 cone and
parabola scaled by eq. B.9 ("at 4.08", "at 3.5"), the rule HPR Sim applies to other finenesses. Measured
minus Stoney, signed:

| Pair | 0.90 | 0.96–0.99 | 1.00 | 1.01–1.02 | 1.05 | 1.10 |
|---|--:|--:|--:|--:|--:|--:|
| CM − parabola | −0.001 to +0.007 | not run | +0.012 to +0.020 | not run | not run | −0.013 to −0.006 |
| PA − cone at 4.08 | +0.000 | +0.001 to +0.002 | +0.011 | +0.011 to +0.017 | −0.018 | −0.024 |
| HA − parabola at 3.5 | 0.000 to +0.005 | +0.003 (0.95) | +0.014 | not run | −0.006 | −0.003 |
| MIL-HDBK-762 ogive − parabola | +0.007 | not given | +0.015 | not given | not given | +0.002 |

- **Inside the window the sign is one-sided.** Every set but MI (0.73% blockage, §4) reads above Stoney at Mach
  1.00–1.02, by 0.011 to 0.020. From 0.90 to 0.99 they agree within 0.007. Past 1.03 Stoney's curves read 0.003 to
  0.024 high.
- **The pattern fits a true tare that rises later than the fairing.** Measured nose drag climbs earlier and peaks
  sooner than Stoney's: the cone near Mach 1.01–1.04, his 3:1 cone near 1.07 on the print. **Inference:** his tare
  is 0.01–0.02 too high near Mach 1.0 and too low near 1.05–1.10; its rise was timed on a long-nosed body [S5]
  pp. 5–6.
- **Scaling caveat (Inference).** Eq. B.9 changes a curve's height, not its Mach; a slenderer nose should peak nearer
  Mach 1, so part of the cone's −0.018 and −0.024 at 1.05 and 1.10 may be fineness, not tare.
- **HPR Sim's closed form** reads +73% at Mach 0.899, +44% at 1.001, +86% at 1.050 and +115% at 1.100 on PA's cone;
  Stoney's cone scaled by B.9 reads −2% to −4% from 0.899 to 0.991, −11% to −18% at 1.001–1.020, and +19%
  and +27% at 1.05 and 1.10 (a script port of the code's formulas). On the 3:1 ogive the current curve is 2.9 to 3.3
  times CM's at Mach 1.00.
- **What it says about the ±0.02.** Its size holds: no set sits more than 0.024 from Stoney through Mach 0.9–1.1 at
  the indicated Mach. Its shape doesn't: a symmetric ±0.02 over 0.96–1.03 should be read as +0.01 to +0.02 at
  1.00–1.02 and −0.005 to −0.024 at 1.05–1.10, with up to +0.047 if tunnel Mach is corrected (§4).

## 4. Wall interference and Mach error, by facility

**Inference, from two comparisons: blockage lowers a nose's measured pressure drag at tunnel Mach 1.00–1.02, so the
tunnel rows above are, if anything, low there.** Page's cone at 0.25% reads 0.011 under its 0.005% run at indicated
Mach 1.001 and 1.020, then 0.007 over at 1.050; the two agree to 0.001 at 0.90 (**Inference**, from Table I). He
places interference at Mach 0.99–1.05 [S3] p. 17. MI's ogive at 0.73% reads 0.025 to 0.033 under CM's at Mach 1.0
and agrees at 1.1 (**Inference**).

- **CM, Langley 8 ft, slotted.** Empty-tunnel Mach varies ±0.005 subsonic to ±0.02 at the top Mach [S1] p. 3. At
  Mach 1.00 a shock stands at 24 in, 2.9 calibers behind the shoulder; the nose shock's reflection lands at 30 in at
  Mach 1.1 and 48 in at 1.2, behind the nose [S1] p. 6. **Inference:** Page's sonic-point formula, with this model's
  sonic point at its shoulder and a half-height of 3.5 to 4 ft, puts free-air Mach 1 at indicated 1.033–1.036. Page
  derives it for porous, circular tunnels and finds it fails for his larger model [S3] pp. 17–18, so free-air Mach
  for CM's 1.00 lies between about 0.96 and 0.99. Stoney's parabola there is 0.010 to 0.029, giving gaps of +0.020
  to +0.047.
- **PA, Ames 14 ft, 0.005%.** Free-air sonic flow is at indicated Mach 1.0066, where the cone's Cp is about
  8% higher [S3] p. 17. **Inference:** that point integrates to 0.094, 0.020 above Stoney's scaled cone at Mach 1.
- **MI, Lewis 8 × 6 ft, perforated, 0.73%.** Interference peaks at Mach 1.1 and is weakest on the ogive [S2] p. 6;
  large models hold the terminal shock forward [S2] p. 7. Its sister report shows cone pressures depressed at Mach 1
  on every model, which delays the drag rise [S9] p. 7. Its Fig. 14 decrement is about 0.008 to 0.009 of P₀ at
  0.18% and 0.015 to 0.021 at 2.91% [S9] p. 54, which is 0.022 to 0.057 in Cp at Mach 1 (**Inference**).
- **HA, free flight.** No walls. Hart lays a shift in the force break between configurations to Mach error [S4]
  p. 6 and puts the probable Mach error at about ±0.01 [S4] p. 4.
- **ST, gun-launched free flight.** No walls; ±0.008 in total drag (transonic-drag-rise §2).

## What the project can use

All nine reports are NACA or NASA works on NTRS, each marked public use permitted (NTRS records, read 2026-10-28):
the project may cache them under `refs/` and commit readings with report, page and figure. MIL-HDBK-762's values are
those read in [transonic-drag-rise](transonic-drag-rise.md) §5 (Distribution A). Nothing private was read.

- **A nose-only reference set for #67's fix** (a suggestion). PA's Table I(b) can be transcribed whole into a
  fixture: an interference-free, pressure-only cone curve through the fairing window. CM's integrated ogive row and
  its script, with the zero range, add a 3:1 tangent ogive. Unlike whole-rocket sets, these pin the nose term alone.
- **Expected readings today (Inference):** HPR Sim's tangent ogive at Mach 1.00 is +0.108 to +0.115 over CM, its
  cone +0.038 over PA at 1.001. Stoney's curves, the proposed fix, would read −0.011 to −0.020 at 1.00–1.02.
- **For the fix.** A 3:1 tangent ogive could take Stoney's parabola with +0.015 ± 0.01 added at Mach 1.00–1.02,
  which lands on MIL-HDBK-762's 0.052 at Mach 1.0. Cones could take Stoney's cone scaled by B.9, with the
  ±0.02 kept signed as in §3. Expected effect: under 0.02 on a nose term of 0.04 to 0.16 near Mach 1, in a band
  about 0.05 Mach wide, against the current formula's excess of 0.015 to 0.14 from Mach 0.9 to 1.1.
- **Which way it errs (Inference).** Stoney's curves low at Mach 1.00–1.02 mean too little drag, so apogee high; the
  size on a flight is not bounded here.

## Open questions

- **Free-air Mach of CM's "1.00".** Between 0.96 and 0.99 here, and the 8-ft section's size was not read; a
  low-blockage ogive run would fix it.
- **Mach 0.95 and 1.05 on an ogive.** CM ran neither; MI's runs are too blocked to stand in.
- **TM X-1655's 10° cone** (fineness 2.84) at 0.18% blockage is plotted through Mach 0.9–1.1 [S9]; integrating it
  would add a fineness-3 cone, with its blockage known.
- **Parabola and von Kármán noses.** No integrated transonic pressure data at fineness 3 were found; Stoney's
  parabola stands in for the tangent ogive, within 0.69% of its radius (transonic-drag-rise §5).
- **Not reached:** AEDC's cone-cylinder pressure reports (Robertson and Chevalier, on DTIC), Kell's R&M 3290 full
  text, and Hoerner's chapter 16 sources (a book, cite only).

## Sources, all read 2026-10-28

Cached under `refs/research/stoney-tare-base-pressure/` unless noted, with renders and scripts (`scripts/`).

- **S1.** Czarnecki and Monta, NASA TN D-3516 (1966): pp. 1–6, Table II p. 18, Fig. 1 p. 19, Fig. 5 p. 27.
  <https://ntrs.nasa.gov/citations/19660024157>
- **S2.** Mitchell, NASA TM X-1656 (1968): pp. ii, 3–4, 6–7, Fig. 2 p. 13, Fig. 4 pp. 19–20.
  <https://ntrs.nasa.gov/citations/19680026661>
- **S3.** Page, NACA TN 4233 (1958): pp. 4–6, 16–18, Table I p. 21, Fig. 1 p. 33.
  <https://ntrs.nasa.gov/citations/19930085182>
- **S4.** Hart, NACA RM L52E06 (1952): pp. 1–4, 6, Fig. 12 p. 19 (in `refs/papers/`).
  <https://ntrs.nasa.gov/citations/19930087048>
- **S5.** Stoney, NACA RM L53K17 (1954): pp. 5–6, Fig. 4 (in `refs/research/transonic-drag-rise/`).
  <https://ntrs.nasa.gov/citations/19930087953>
- **S6.** Stoney, NASA TR R-100 (1961): Fig. 12(a), p. 16 (in `refs/papers/`).
  <https://ntrs.nasa.gov/citations/19630004995>
- **S7.** Solomon, NACA Report 1242 (1955): p. 3, Fig. 12. <https://ntrs.nasa.gov/citations/19930090996>
- **S8.** Lee and Summers, NACA RM A57I09 (1957): pp. 1, 3–4. <https://ntrs.nasa.gov/citations/19930089881>
- **S9.** Mitchell, NASA TM X-1655 (1968): pp. 4, 7–8, Fig. 14 p. 54. <https://ntrs.nasa.gov/citations/19680026662>
- Kell, ARC R&M 3290 (1958), catalog record and abstract only; no report licence, site copyright, cite only:
  <https://reports.aerade.cranfield.ac.uk/handle/1826.2/3868>

[i67]: https://github.com/nrdptel/fusionspace-eridanus/issues/67
