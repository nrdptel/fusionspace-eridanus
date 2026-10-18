# Fin flutter measured in flight, on missile fins and on soft roots, against Martin's formula

**In short: 15 more fins with a measured flutter onset add to the 26 in
[flutter-measured-onsets](flutter-measured-onsets.md), and three of them read on the flattering side.** Ten are wings
flown on Langley rocket models between 1952 and 1956, with speed from radar and air from radiosondes. One is a missile's
rocket-motor fin in a tunnel, three are small tunnel plates (two from a new group, at Liège) and one is a hobby
rocket's fin seen fluttering on onboard video. A sixteenth, a 1951 wing, may sit in the data Martin's line was drawn
through and is not counted. HPR Sim's reading of eq. 18 errs by how the fin is built:
- Uniform fins (solid plates, laminated spruce) read 0.35 to 0.84 times the onset, the safe side.
- Built-up wood wings with metal inserts, most with soft tips and trailing edges, read 0.72 to 1.50 times the onset;
  three of them and the 1951 wing read above 1, the flattering side.

Martin's own figure 3 check, the reading that decides, places 11 of the 12 fluttering flight wings and missile fins
scored here (the 1951 wing included) on the flutter side; it places a steel-stiffened delta on the safe side, a
flattering error. Leaving part of the root unclamped lowered the onset speed by 8 to 23% in one Langley test, a butted
root filler about 12% in another. None of these fins is glued to a tube, and no sounding-rocket fin failure was found
with its speed, geometry and material stated. All sources read 2026-10-18. Ratios are our own computation
(**Inference**): Martin's flutter speed over the lowest measured onset, with HPR Sim's reading ([section 6](#6-method));
above 1 is the flattering side.

## 1. Where Martin's line comes from, and a wrong lead

- **The line was drawn by November 1951.** TN 4197 replaces Martin's RM L51J30 (S2, p. 1 footnote). The 1951 memo has
  the same constant in $X = 39.3\,A^3/((t/c)^3(A+2))$ (S1, p. 11), the same worked examples (pp. 4–5) and a figure 3
  (p. 14) that looks the same as TN 4197's (p. 19), compared by eye, not point by point. Langley flights reported after
  1951 are therefore outside the data the line was drawn through, which M2.11 asks for.
- **Wallskog's copy of the 1951 line** (S3, Fig. 7 p. 32), digitized at 300 dpi on its axis ticks, runs at $D/G_E$ =
  0.284 to 0.288 over $G_E$ = 0.1 to 10 Msi, inside the project's own reading of the band, 0.25 to 0.31 ([flutter
  page](../physics/flutter.md#the-line-in-martins-data)).
- **NACA TN 4402 is not a correction of Martin.** It reports forces on a T-tail oscillating in yaw (1958, title read).
  No later correction was found; Wallskog's (S3) is the only published check of the line against new flight data found.

## 2. Langley rocket-model flights, 1951 to 1956

Speed from radar, air from radiosondes or tabulated density, flutter from telemetered accelerometers or strain gauges.

**Wallskog, RM L55A14 (S3): five wings flown for drag that fluttered, and three earlier deltas he plotted.** The five
are 3% thick NACA 65-series wings of laminated mahogany with 2024-T3 inserts, on wing-body models flown to about Mach
1.55 to 1.95 (pp. 1–4). Flutter was found afterward in the accelerometer data; no wing failed (p. 5). Wallskog derived
each wing's effective $G_E$ from its section and plotted all eight against Martin's 1951 line (Fig. 7, p. 32).

| Wing (report, planform) | $G_E$, Msi | Onset: Mach, ratio | Lowest Mach still fluttering, ratio | $D_0/G_E$ |
|---|---|---|---|---|
| W1 (L55A14, 52.5° delta) | 1.00 | 1.585 (at peak), 0.75 | 0.99, 1.19 | 0.75 |
| W2 (L55A14, 45° delta) | 1.26 | 0.885, 1.05 | 0.83, 1.37 | 1.16 |
| W3 (L55A14, diamond) | 0.77 | 1.575 (at peak), 0.96 | 0.96, 1.81 | 0.46 |
| W4 (L55A14, diamond) | 1.19 | 1.12, 1.14 | 0.81, 2.21 | 0.63 |
| W5 (L55A14, arrow) | 0.47 | 1.09, 0.72 | 0.94, 0.99 | 1.76 |
| D1 (L52E06a, 60° delta) | 0.18 | hidden in boost | 1.07, 0.98 | 1.93 |
| D2 (L54D12a, same, steel inlays) | 2.00 | about 1.69, 1.43 | 1.40 or lower, 2.06 or more | 0.18 |
| D3 (L51B28, 60° delta, 6.5% thick) | 0.097 | 1.11 (slowing), 1.50 | failed at 0.99, 1.68 | 0.36 |

- **Inputs.** Planforms from Fig. 1 (pp. 14–17), all with pointed tips. Mach, speed and density at onset and end read
  by eye from the time histories (Fig. 5, pp. 24–26), about ±0.02 in Mach, (±2% in a ratio). $G_E$ is Wallskog's
  effective value, digitized from Fig. 7, not a material modulus; it uses $J \approx 0.151\,c t^3$ (p. 3). Martin's own
  $J = c t^3/6$ lowers every $G_E$ by 9.5% and every ratio by 5%. Our sea-level $X$ from the planforms matches Fig. 7's
  within 3% for six of the eight, so Wallskog took $X$ near ground pressure, as HPR Sim does at launch; W1's and W4's
  read 7% above Fig. 7's, whose values would raise their ratios about 3.5%.
- **End readings run high:** the models slowed into thinner air, and eq. 18's speed grows as $1/\sqrt{p}$. Wallskog (p.
  7): soft tips and trailing edges make the computed $G_E$ too high; strong wings may still flutter.
- **D2 is the flattering error on the deciding check.** The steel inlays raised the delta's $G_E$ elevenfold over D1's
  but left its tip and trailing edge as soft; it fluttered over about the same speeds as D1 (S5, p. 1). Its figure 3
  ratio, 0.17 to 0.18, is below the band, yet it fluttered from about Mach 1.69. Boost noise hid the true onset (p. 4),
  so its 1.43 is a lower bound.
- **D3 may sit in Martin's own data**: RM L51B28 is Langley's and dated before RM L51J30. It passed Mach 1.24 without
  flutter, began at 1.11 while slowing and lost a wing at 0.99 (S6, pp. 1, 4). Not counted.

**Lauten and O'Kelly, RM L55J21 (S7): three spruce wings built to flutter.** Laminated spruce, center lamina grain
streamwise, outer laminae along the quarter chord, attached to plates welded to a sleeve over a 5 in rocket motor (p.
4). Table VII (p. 17) gives the onset of sustained flutter with Mach, speed, density and dynamic pressure.

| Model | Onset (Table VII) | Aspect ratio, taper, $t/c$ | Ratio at $G$ 98 ksi (75–125) | $D_0/G$ |
|---|---|---|---|---|
| J1, 45° swept | Mach 1.142, 1,242 ft/s, 0.00236 slug/ft³ | 1.366, 0.20, 0.04 | 0.53 (0.47–0.60) | 2.85 |
| J2, 60° swept | Mach 1.15, 1,295 ft/s, 0.00221 slug/ft³ | 1.379, 0.20, 0.04 | 0.52 (0.46–0.59) | 2.92 |
| J3, 60° delta | Mach 0.98, 1,105 ft/s, 0.00227 slug/ft³ | 1.152, 0, 0.03 | 0.54 (0.48–0.62) | 3.60 |

- Table VII prints 0.00277 for J3's density, but its own dynamic pressure (1,386 lb/ft²) gives 0.00227; 0.00277 would
  mean 1.19 atm (Inference). $G$ is assumed: Sitka spruce's $G_{LR}$ and $G_{LT}$ are 0.064 and 0.061 times $E_L$ = 1.57
  Msi, so 96 to 100 ksi (S10, Tables 5–1, 5–3b); Table 5–1's footnote adds 10% to $E_L$ (105–111 ksi, ratios 5%
  higher); 75 to 125 ksi covers it.
- Brief bursts came earlier, J2 at Mach 1.005 and J3 at 0.72 in boost and at 0.96 (pp. 6–7); against those Martin
  reads 0.60 and 0.74 at 98 ksi (0.53 to 0.84 over 75 to 125 ksi). All three sit far into figure 3's flutter region.

## 3. Tunnel fins: a missile fin and three small plates

| Case | Fin: root/tip/span, thickness | Material, $G$ used | Onset | Mach | Ratio |
|---|---|---|---|---|---|
| A, ASROC | 10.66/6.44/4.22 in, 0.050 in | 2024 Al, 27.6 GPa (assumed) | 757–1,299 ft/s | 0.69–1.30 | 0.38–0.82 |
| L1, Liège | 200/200/450 mm, 1.0 mm | aluminum, 26.3 GPa (assumed) | 28.3 m/s | 0.08 | 0.78–0.81 |
| L2, Liège | 190/100/450 mm, 1.0 mm | same | 37.87 m/s (damping fit) | 0.11 | 0.51 |
| P1, Belgrade | 200/125/300 mm, 1.0 mm | printed PLA, 0.48–0.55 GPa | 10.5–12.5 m/s, 5 fins | 0.03 | 0.35–0.45 |

- **A: ASROC rocket-motor fin, Levey and Hanson, RM SL58H08a (1958, S8).** 0.4-scale flat plates in the Langley 9- by
  18-inch flutter tunnel, with Mach, density, speed of sound and dynamic pressure at every point (Table I, p. 7). The
  root is clamped over only 6.0 in, the leading-edge corner overhung (Fig. 1, p. 8). The planform is not a trapezoid, so
  the range spans three root-chord readings (6.0, 8.91, 10.66 in; 0.38 to 0.63 on the last; the 6.0 in reading takes a
  6.0 in tip, as eq. 18 has no taper above 1). Two rows disagree with their own dynamic pressure (−14%, +59%) and are
  left out; one, a fin with its root restraint cut, reaches 1.10 on one reading. After TN 4197; Langley again, not a new
  group.
- **L1, L2: Oleffe, master's thesis, Université de Liège (2020, S9).** Flat plates clamped on a shaker cart, so the
  root is held in twist but not rigid in plunge (pp. 37, 48). Thickness and material are the thesis's model values, not
  measurements (p. 43); L1's measured torsion frequency is within 2% of the 1 mm model's (Tables 7–8), which supports
  them. Onsets from Table 9 (p. 53); air assumed sea level. Open access, cite the numbers. A new group.
- **P1: Dinulović et al., Materials 18(5):1127 (2025, S11).** Four ±45° printed layers; modulus and density measured
  (Table 2), $G$ assumed isotropic ($E/2.6$) or scaled by the torsion frequency. Table 1's four 0.2 mm layers make 0.8
  mm, not Table 2's 1 mm (ratio 0.25–0.32 at 0.8 mm). Table 2's chords read as swapped, the photo showing the clamped
  edge wider (as printed, 0.56–0.72). CC BY 4.0. Belgrade again: a fin, not a group.
- **Figure 3 places all four on the flutter side** ($D/G_E$ 1.3 to 6,000 at the test's pressure), so none is a
  flattering error on that check. All are thinner for their chord ($t/c$ 0.0047 to 0.0083) than Martin's wings.

## 4. A softer root lowers the onset

- **Partial root clamping, NASA TM 101547 (Doggett and Soistmann, 1989, S12).** One 45° aluminum delta, 1.63 mm thick,
  sawed along the root so 100, 74, 47, 34 and then 21% of the root chord stayed clamped, at Mach 0.10 to 0.35 (pp.
  1–2, 5–6). From its flutter-speed index (Fig. 15(b), p. 8) and torsion frequencies (Fig. 10), read by pixels, the
  onset speed over the fully clamped one is 0.92, 0.83, 0.83 and 0.77 at 74, 47, 34 and 21% (Inference: sawing leaves
  mass and chord unchanged); 0.98 and 0.91 at 74 and 21% if the index used the calculated torsion frequency. The
  report's index rises as clamping is removed (p. 8); the torsion frequency falls faster.
- **ASROC root variants (S8, Table I(b), Fig. 1(b), p. 9), all at Mach 1.30.** A root filler butted on, as the
  full-scale fin was built, fluttered at 1,846 lb/ft² against 2,400 for an integral filler: 23% less dynamic pressure.
  Both fluttered near 993 ft/s, in different air; at equal air that is about 12% less speed (Inference). Removing the
  forward 2.15 in of a full-length clamp cost at least 2.5% in speed (a bound). A full root clamp removed flutter (pp.
  5–6).
- **What this means for HPR Sim (Inference).** Martin's line rests on clamped wings. On these two tests a softer root
  lowers the onset by 2.5% (a bound) to 23%, so on such a fin his speed would read up to 1/0.77 = 1.30 times high. The
  highest safe-side ratios found so far, 0.82 to 0.84, would become 1.06 to 1.09. Neither test is a fin bonded to a tube
  wall that can itself flex, which is the hobby case; its size remains unmeasured.

## 5. A hobby rocket's fin seen fluttering in flight

**PSAS LV2c, Brothers, Oregon, 2010-06-27 (S13).** Portland State Aerospace Society's rocket, on an N2000W, shows a main
fin fluttering for about 1 s on its aft-looking onboard video, centered 3.73 s after liftoff, about 4° of amplitude;
the fins survived. Two flight computers' accelerometer logs, aligned at first motion, differ by 2 to 4%: 275 to 287 m/s
at the window's start and 318 to 326 m/s at its center, Mach 0.79 to 0.98 by log and assumed temperature. The team's own
model gives root 370 mm, tip 202 mm, span 151 mm, 3.0 mm thick; the fins were flat plates in external bolted clamps.
- **Martin reads 0.49 to 0.82 times the onset** (217–225 m/s at G10's 7.69 GPa, 159–165 m/s at 4.14 GPa), the safe
  side. The sign rests on the material, inferred from a comment in the team's fin CAD part ("Scrap FR4 or G10"); were
  the fins aluminum, the ratio would be 1.25 to 1.52. Figure 3 places it on the flutter side on every reading.
- **Inputs:** planform and thickness from the team's model, material inferred, onset from video at an assumed 30 frames
  per second, speed measured, air assumed (pad at 4,500 ft, ISA to a 30 °C day). No license stated: cite only. The only
  hobby fin in either note with an in-flight onset.

**Weaker evidence, no score.** WPI (2008, S14): two 0.063 in G10 fins vibrated at 22.4 and 27.7 m/s in a student tunnel
(ratio 0.65 to 0.90), but the speeds come from fan settings, the drawings carry no unit and the vibration ran at the
first bending frequency, which fits buffeting too. Onsets with no planform (S15): a Loki J1026 record attempt (2023,
carbon fins, near Mach 2.3 by our estimate), an XPRS video (2004).

## 6. Method

Eq. 18 as [the flutter page](../physics/flutter.md) reads it: $(V_f/a)^2 = G_E/D$, with $D = (24\varepsilon\gamma/\pi)\,
p\, K\, (\lambda+1)/2$ and $K = A^3/((t/c)^3 (A+2))$; $\varepsilon = 0.25$, $\gamma = 1.4$, $A = 2s/(c_r + c_t)$, $t/c$
on the root chord, $G_E$ the material's $G$ for a flat fin (Wallskog's effective $G_E$ for his wings). The scripts
reproduce the project's aluminum example (1,169 m/s). Measured rows take $p = \rho a^2/\gamma$; $D_0/G_E$ is the figure
3 ratio at sea-level pressure, as HPR Sim takes it at launch.

## What the project can use

- **For M2.11, by group** (public domain unless noted): Langley's flights W1–W5, D1, D2, J1–J3 and the ASROC fin;
  Liège's L1, L2 (open access, cite only); Belgrade's P1 (CC BY); PSAS's LV2c (cite only). With the earlier note, six
  groups. Air is measured for every Langley flight and per ASROC point, assumed for the rest.
- **The flattering-side cases** for the sign test: D2 (figure 3 says safe; 1.43 or more), W2 and W4 (1.05, 1.14).

## What this suggests

- The flutter page could name a third flattering class beside coupled laminates and near-sonic built-up wings: fins
  whose stiffness sits inboard while tip and trailing edge stay soft (D2, W2, W4), such as a fin that tapers in
  thickness or a stiff core with soft edges. Figure 3 is no guarantee there: D2 sat below the band and fluttered.
- A soft-root factor of the measured size (onset 0.77 to 0.98 times the clamped one) would cover the uncoupled plates'
  highest ratio; whether a glued fin's root is softer or stiffer than these sawed clamps needs a measurement.

## Open questions

- No fin glued to a tube has a measured onset; the root factor above is from clamped tunnel fins with the clamp cut.
- Laminate plates still unread: Dunn and Dugundji (DTIC ADA254323, down), Dunn's and Landsberger's MIT theses
  (rate-limited), the Duke delta (paywalled); the woven glass lead is analysis only, dropped.
- No sounding-rocket fin failure was found with speed, fin geometry and material (NTRS searches on Nike, Aerobee, Arcas,
  Black Brant and Scout fins); an AFIT thesis on sounding-rocket fins (DTIC ADA502110) returned HTTP 403.
- The Wallskog readings (Fig. 5 time histories by eye, Fig. 7 by pixel) and the TM 101547 pixel readings have one reader
  each; RM L51J30's and TN 4197's figure 3 were not compared point by point.
- LV2c's fin material needs a second source; it decides its sign. The J1026 planform is in a forum attachment not read.
  AIAA 2025-99192, a rocket flown to study transonic fin flutter, sits behind a bot check, not read.

## Sources, all read 2026-10-18 (NTRS: <https://ntrs.nasa.gov/citations/ID>)

- **S1.** Martin, NACA RM L51J30 (1951), pp. 4–5, 11, Fig. 3 p. 14. ID 19930086797.
- **S2.** Martin, NACA TN 4197 (1958), p. 1 footnote, Fig. 3 p. 19, references pp. 16–17. ID 19930085030.
- **S3.** Wallskog, NACA RM L55A14 (1955), pp. 1–7, Figs. 1, 5 pp. 14–26, Fig. 7 p. 32. ID 19930088664.
- **S4.** Judd and Lauten, NACA RM L52E06a (1952), pp. 1–4, Fig. 6 p. 23. ID 19930087050.
- **S5.** Lauten and Judd, NACA RM L54D12a (1954), pp. 1–5. ID 19930088218.
- **S6.** Lauten and Mitcham, NACA RM L51B28 (1951), pp. 1–4, Fig. 6 p. 21. ID 19930086580.
- **S7.** Lauten and O'Kelly, NACA RM L55J21 (1956), pp. 4–7, Table VII p. 17, Fig. 1 pp. 18–20. ID 19930089020.
- **S8.** Levey and Hanson, NACA RM SL58H08a (1958), pp. 1–6, Table I p. 7, Fig. 1 pp. 8–9. ID 19710075830.
- **S9.** Oleffe, master's thesis, Liège (2020), pp. 2, 37–53. <https://matheo.uliege.be/handle/2268.2/10270>
- **S10.** Forest Products Laboratory, Wood Handbook, FPL-GTR-190 (2010), Table 5–1 p. 5–2, Table 5–3b p. 5–13.
- **S11.** Dinulović et al., Materials 18(5):1127 (2025), Tables 1–4, CC BY 4.0. <https://doi.org/10.3390/ma18051127>
- **S12.** Doggett and Soistmann, NASA TM 101547 (1989), pp. 1–2, 5–8, Figs. 10, 15. ID 19890010723.
- **S13.** PSAS, LV2c launch page, logs, `LV2c.ork`. <https://archive.psas.pdx.edu/lv2c_launchdata-2010-06-27/>
- **S14.** Fleming et al., WPI MQP (2008), pp. v, 32–35, 97–106. <https://digital.wpi.edu/show/n583xw37x>
- **S15.** rocketryforum.com threads: the Loki J1026 record attempt (2023), XPRS video (2004), N1100 fins lost (2006).
