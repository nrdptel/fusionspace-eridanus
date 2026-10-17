# Measured flutter onsets of hobby-size fins

**In short: published tables give at least 26 distinct fins or wings with a measured flutter onset and the fin's inputs
stated, from four groups, none among the tests Martin cites, so M2.11's floor of three fins from two groups can be met
from readable tables.** Ten are flat plates near hobby size in tunnels: six aluminum plates with a 305 mm (12 in) root
chord at Mach 0.62–3.00 (NASA TM X-53, public domain), three graphite/epoxy plates (an MIT thesis) and the known
e-glass fin from Belgrade. Sixteen more are rocket-launched wings, reported in 1951, whose onsets were measured in
flight (ARC R&M 2944). Our computation of HPR Sim's flutter speed (Martin's eq. 18 with thickness over the root chord, a
flat fin's material shear modulus) does not err on one side. It reads 0.14–0.65 times the measured onset on every flat
plate whose plies aren't laid to couple bending and twisting. It reads high, the flattering side, on two laminates whose
plies couple bending to twist (1.13–1.42 times), on the 1951 wings near Mach 1 (up to 1.41 times, median 1.13 over 22
wings) and on a weakened tunnel wing at low air density (up to 1.50 times). No case is a fin glued to a tube. Hobby
flights give only failure and survival speeds, which bound the onset from above, never an onset.

All sources read 2026-10-17. Ratios are our own computation (**Inference**), Martin's flutter speed over the lowest
measured onset; above 1 is the flattering side. Method in [section 6](#6-how-the-ratios-were-computed).

## 1. What M2.11 needs

ADR-279's M2.11 scores HPR Sim's flutter readings against at least three distinct fins measured fluttering, from at
least two research groups, none of the wings Martin's line was drawn through. Each fin needs its planform, thickness,
mounting, material and shear modulus, and the test's pressure, temperature and Mach number, each marked measured or
assumed; repeats of one design count once. One case was known before, B1 below ([evidence row G22](../evidence.md#g22)).
Martin's figure 3 draws on three NACA tunnel reports (his refs. 3–5: RM L51A11, RM L53G10b, TN 3312) and on uncited
Langley rocket-model data on missiles and bombs (TN 4197, pp. 5–6). None of the tests below is among his 13
references; only the 1959 Langley plates could, in principle, sit in his uncited data.

## 2. Tunnel tests near hobby size

| Case | Fin: root/tip/span, thickness | Material; $G_E$ used | Onset | Ratio |
|---|---|---|---|---|
| B1 | 180/90/262 mm, 0.65 mm | e-glass [0/45/−45/90]s; 7.96 GPa (ply 4.3) | 19.6–21.5 m/s, 5 fins | 0.55 (0.40) |
| N1A | 305/0/111 mm, 0.81 mm, 70° delta | 2024-T3 aluminum; 27.6 GPa | 14 points, Mach 0.63–3.00 | 0.15–0.34 |
| N1B | 305/0/82 mm, 0.81 mm, 75° delta | same | 10 points, Mach 0.62–2.55 | 0.18–0.32 |
| N1C | 305/0/54 mm, 0.66 mm, 80° delta | same | 9 points, Mach 0.63–3.00 | 0.26–0.40 |
| N2A | 305/102/148 mm, 0.81 mm | same | 5 points, Mach 1.30–3.00 | 0.14–0.24 |
| N2B | 305/102/109 mm, 0.81 mm | same | 4 points, Mach 1.30–3.00 | 0.30–0.40 |
| N2C | 305/102/72 mm, 0.81 mm | same | 5 points, Mach 1.30–3.00 | 0.37–0.47 |
| M1 | 76/76/305 mm, 0.80 mm | graphite/epoxy [0₂/90]s; 5.8–6.2 GPa | 25.9 m/s | 0.63–0.65 |
| M2 | same | [+45₂/0]s; 24.4–32.3 GPa | 27.7 m/s | 1.24–1.42 |
| M3 | same | [+30₂/0]s; 20.5–26.7 GPa | 27.1 m/s | 1.13–1.29 |

Sources: B1 S1 Tables 3–4 p. 583; N S3 Table I p. 7; M S4 Table 6a p. 103. All are clamped at the root. B1's air is
assumed sea level, M's 1 atm and 15 °C (logged, not tabulated); S3 tabulates density and speed of sound per point. The
aluminum modulus is assumed (a 5% change in $G$ moves the speed 2.5%). Ratio ranges for M1–M3 span the thesis's two
ply property sets, not an uncertainty.

- **Belgrade (B1).** The known case; S1 raised the speed in 0.5 m/s steps and took the onset from a root accelerometer.
  Its quasi-isotropic layup couples bending and twist only a little. The paper's "NACA formula" gives 15.3 m/s with
  thickness over the mean aerodynamic chord (p. 580); HPR Sim's reading gives about 10.7 m/s (35 ft/s), as ADR-279
  found. The same group's 2024 article (S2, CC BY) tests a fin of nearly the same outline and the same ply constants
  (180/80/260 mm, 0.6 mm, about 19.0 m/s, Table 5 p. 24); it may be the same sample, so it is counted with B1, not as a
  second fin or a second group.
- **NASA Langley, Hanson and Levey (N1A–N2C).** Six flat 2024-T3 plates, clamped over the whole root, in the 9- by
  18-inch supersonic aeroelasticity tunnel: 47 flutter points and one no-flutter point (1B at Mach 3.00, where Martin's
  speed says it should flutter) (S3, pp. 3–4, Table I p. 7, Fig. 1 p. 14). Onsets run 209–684 m/s. The table gives
  density, speed of sound, dynamic pressure and frequencies for every point, so no air state is assumed. Edges are
  beveled. These plates are far thinner for their chord than hobby fins (thickness ratio 0.0022–0.0027 against roughly
  0.01–0.03) and swept 70–80°, which eq. 18 ignores; both push Martin low. Langley was Martin's laboratory; the
  report is dated May 1959, after TN 4197, and gives no test dates.
- **MIT, Hollowell (M1–M3).** Six-ply graphite/epoxy rectangles, aspect ratio 4, clamped in a vise by a bonded tab, in
  a 0–32 m/s tunnel; onset from strain-gauge traces, one run each (S4, pp. 51–55, 62–67, Table 6a p. 103). A
  [±45/0]s plate didn't flutter to 32 m/s, where Martin gives 34.5–39.6 m/s: consistent. Two plates turned the other
  way, [−45₂/0]s and [−30₂/0]s, bent away statically (diverged) first, at 12.5 and 11.7 m/s (Table 5, p. 95). M2
  and M3 are where Martin reads high: their plies couple bending to twist, which lowers the flutter speed and which
  eq. 18 has no term for. The thesis may be viewed but not reproduced: cite its numbers only.

## 3. Onsets measured in flight

The only flight source with measured onsets is ARC R&M 2944 (S7): RAE fired 32 models, each a pair of wings on a solid
rocket (most 3 in, 76 mm), over an artillery range; the report is dated November 1951. A vibration pick-up in one wing
was telemetered and Doppler radar gave the speed to better than 0.5%; the onset is where the oscillation turns regular
and divergent (s. 3.1, p. 3). Table 1 (p. 8) gives measured onsets for 22 wings of 16 designs (two sets were built to
one design, s. 5.2): 495–1,090 ft/s (151–332 m/s), Mach 0.44–0.98, and failure speeds for most. Each wing is 2 ft
(0.61 m) root to tip, 1 ft chord, 10% thick, untapered and unswept, built up from wood, plywood and a metal spar, with
the measured twisting stiffness given.

Martin's own rule for a built-up wing turns that stiffness into an effective modulus, $G_E = 6JG/(c t^3)$ (TN 4197
eq. 12, p. 13). With it, his speed is 0.92–1.41 times the onset, median 1.13; 21 of 22 read high (Inference). The
stressed-skin wings give 1.07–1.15, the 30% spar wings 0.92–1.14, the 25% spar wings at Mach 0.75–0.98
1.24–1.41. The report's own finding points the same way: compressibility cuts the margin over incompressible theory
from +50% at Mach 0.4 to −25% at Mach 0.9 (Summary, s. 5.3). Caveats: the conversion reads Table 1's stiffness
"measured at 0.7 s" as torque over twist at 0.7 span; taking $JG$ as the stiffness times 1 ft moves every ratio −15%,
times the span +20%, and the transonic wings stay above 1 (lowest 1.05). One group and one planform, three to five times
hobby size. Crown copyright, published 1956; it has most likely expired, but that is not a legal check, so cite only.

## 4. Hobby flights: failures and survivals

No hobby source records when flutter began. What they give bounds the onset from above (a fin that shredded) or the
failure speed from below (one that survived). Ratios use G10 at 7.69 GPa (1.116 Msi) unless stated; G10 at 4.1 GPa (600
ksi) instead lowers Martin's speed by 27% and raises every G10 ratio below by 1.36 times.

| Source | Fins | Speed, how known | Over Martin's |
|---|---|---|---|
| S8 rows 18–20; S9 pp. 5–6 | 1/8 in plywood (inferred) | survived 140, 201; shredded 282 m/s; logged | 1.85; 2.60 |
| S8 rows 21–22 | same planform, 1/16 in | survived 104; shredded 223 m/s; logged | 2.7; 5.8 |
| S8 rows 11–17 | five G10 fin sets (inferred) | shredded at simulated peaks | 1.39–3.2 |
| S10 Table 6 p. 57, pp. 31–35 | 1/32 in G10 test fin | lost 2.3 s in on flight 1, early on flight 3 | none |
| S11 | 1.91 mm acrylic, bolted | found broken; peak about 200 m/s (estimated) | 3.1 |
| S12 pp. 2–3, 7 | 3/16 in G10, slotted (inferred) | survived 375 m/s (logged) | 0.72 |

- S8 is a hobbyist's 2001 spreadsheet of contributed flights; only the plywood rows' speeds are marked measured, the
  material is inferred from which modulus each row's ratio was computed with, and no row gives the mounting or the
  site's altitude; the plywood ratios take S9's assumed 3,000 ft (sea level adds 4%). Its plywood modulus, 1.07 GPa (155
  ksi), is marked measured to ASTM D3044; the newsletter that reports the same rocket (S9) uses 2.62 GPa (380 ksi),
  which lowers the plywood ratios to 1.18 and 1.66.
- In S10 a 1/16 in G10 test fin showed flutter in the accelerometer data near burnout and survived; the team says its
  accelerometer under-read speed, flight 2 lost no fin (pp. 34–35), flight 3's loss is in the addendum, and the report
  contradicts itself on the test fins' planform.
- S11's ratio takes $A = 2s/(c_r + c_t)$ on its polygon fin; span squared over area gives 2.6. S12's is 0.85 at the
  thesis's own 5.5 GPa (800 ksi).
- None of these sources shows that a fin failed by flutter rather than divergence, overload or a broken bond, and most
  speeds are simulated. They agree with Martin being low against shredding, by 1.4 to 5.8 times, and cannot show the
  flattering side. S12's survival below Martin's speed says nothing about its sign.

## 5. Tunnel tests too large or too different to count

- **NASA TM 4180** (S5): 6061-T6 plates with an 883 mm root, Mach 0.23–1.05 at low density, Table II PDF pp. 9–10;
  ratios 0.50–0.77 on the two trapezoids.
- **AGARD wing 445.6** (S6, reprinting TN D-1616): laminated mahogany, a 4% airfoil, 45° sweep, 556 mm root, Table II
  p. 49. Solid model 0.98–0.99; the weakened model reads 0.93 at Mach 0.34 and 1.24–1.50 between Mach 0.90 and 1.07,
  where the air was thinnest. Eq. 18 has no mass-ratio term, so it reads high when the air is light against the wing.
- **Figure-only:** NASA TM 101547 (aluminum deltas, root clamping 21–100%, a flutter-speed index in Figs. 13–15) and
  NACA RM L57F24 (rectangular plates, experiment over theory in Fig. 6, no dimensions). Digitizing would add cases.

## 6. How the ratios were computed

Eq. 18 as [the flutter page](../physics/flutter.md) states it: $(V_f/a)^2 = G_E/D$, with
$D = (24\varepsilon\gamma/\pi)\, p\, K\, (\lambda+1)/2$ and $K = A^3/((t/c)^3 (A+2))$; $\varepsilon = 0.25$,
$\gamma = 1.4$, $A = 2s/(c_r + c_t)$, $t/c$ on the root chord. At sea level $24\varepsilon\gamma p/\pi$ is 39.3 psi (271
kPa), the constant hobby forms carry. The script reproduces the project's `fin_flutter` aluminum example (1,169 m/s)
when its fin is 3.175 mm thick. For tunnel rows, $p = \rho a^2/\gamma$ from the tabulated density and speed of sound.
For the MIT laminates, $G_E = 12 D_{66}/t^3$ from Table A-2 (p. 111), the plate's own twisting modulus, which is HPR
Sim's reading for a flat fin (material $G$).

For a flat fin, Martin's eq. 12 applied to the plate's own twist stiffness would double $G_E$ (a flat plate's torsion
constant is $c t^3/3$, his airfoil's $c t^3/6$) and raise every flat-plate ratio by 1.41 times; HPR Sim keeps the lower
reading ([readings](../physics/flutter.md#readings-where-the-source-leaves-room)). On that other reading B1 is 0.78, M1
0.92, the TM X-53 plates 0.20–0.66, and M2 and M3 1.6–2.0. Martin's center-of-gravity correction, which some hobby
forms include, would raise the 1951 wings' speeds by a further 7–67% (their inertia axes sit at 34–47% chord,
Table 1). Neither is applied above.

## What the project can use

- **The cases for M2.11, by license.** Public domain: TM X-53's six plates (S3), with every point's air state in the
  table; TM 4180 and TM 100492 as analogues. Cite the numbers only: B1 (S1), the three MIT plates (S4) and R&M 2944's 16
  designs (S7). CC BY: S2, though it adds no independent fin. That is ten tunnel fins from three groups (Belgrade, MIT,
  Langley) and sixteen flight-measured designs from a fourth (RAE).
- **Air state, marked per case.** Measured in S3 (density and speed of sound per point), S5 and S6 (density, speed and
  Mach per point); assumed near sea level for S1 and S4 (S4 logged pressure and temperature but didn't tabulate them);
  assumed for S7 (ground launch, onset within the 1.6 s burn).
- **The sign test M2.11 asks for.** Ratios run from 0.14 to 1.42 over the counted tunnel fins and 0.92 to 1.41 over the
  flight wings. The largest flattering-side error is +42% on a coupled laminate (M2) and +41% on a transonic flight wing
  (S7), against HPR Sim's reading.

## What this suggests

- M2.11 can score against S3, S4, B1 and S7 now, and report the error by class, not as one number: uncoupled flat plates
  0.14–0.65 (safe side), coupled laminates 1.13–1.42, built-up wings near Mach 1 to 1.41, light air to 1.50.
- The flutter page could say where measured cases sit on the flattering side: fins whose plies couple bending to twist
  (off-axis layups such as M2's) and speeds near Mach 1.
- Hobby failure speeds belong in a check that every recorded shred sits above the computed speed, not in the score.
- Martin's figure 3 check, the reading that decides, has not been placed for any of these cases; M2.11 needs it.

## Open questions

- No measured case is a fin glued to a tube; the softer root is what hobby fins add, and its size is unknown here.
- The R&M 2944 ratios rest on one reading of its stiffness column; a second source for that convention would settle it.
- Not reached: Dunn and Dugundji (DTIC ADA254323) and Landsberger and Dugundji (1985), more MIT laminate plates (DTIC
  was down); the Duke cropped-delta plate (Tang, Henry and Dowell, AIAA J 37(11), 1999, paywalled); woven glass/epoxy
  plates (Abang Haji Abdul Majid et al., Acta Mech., 2008); a 1/16 in G10 fin's onset logged near Mach 0.9 on onboard
  video (a 2004 forum post, page gone). FalconLAUNCH V lost carbon fins at 11,000 ft with no speed published.
- The Belgrade fins' 2022 and 2024 descriptions differ by 10 mm of tip chord and 0.05 mm of thickness; which is right is
  not settled.

## Sources, all read 2026-10-17

- **S1.** Dinulović, Rašuo, Slavković, Zajić, FME Transactions 50 (2022) 576–585, Tables 3–4 p. 583.
  <https://doi.org/10.5937/fme2203576D>
- **S2.** Dinulović, Bengin (printed "Benign"), Rašuo, Aerospace 11(1):26 (2024), Fig. 15–16 p. 21, Table 5 p. 24,
  CC BY 4.0. <https://doi.org/10.3390/aerospace11010026>
- **S3.** Hanson and Levey, NASA TM X-53 (1959), pp. 3–4, Table I p. 7, Fig. 1 p. 14.
  <https://ntrs.nasa.gov/citations/19630003076>
- **S4.** Hollowell, S.M. thesis, MIT (1981), pp. 51–67, Table 5 p. 95, Table 6a p. 103, Tables A-1, A-2
  pp. 110–111. <https://hdl.handle.net/1721.1/65005>
- **S5.** Dansberry, Rivera, Farmer, NASA TM 4180 (1990), Table II. <https://ntrs.nasa.gov/citations/19900013239>
- **S6.** Yates, NASA TM 100492 (1987), Table II p. 49. <https://ntrs.nasa.gov/citations/19880001820>
- **S7.** Molyneux, Ruddlesden, Cutt, ARC R&M 2944 (1951, published 1956), ss. 2–5, Table 1 p. 8.
  <https://aerade.cranfield.ac.uk/handle/1826.2/3507>
- **S8.** McDonald, "Fin Flutter Estimator" spreadsheet, revision 6 (2001), DataSet sheet rows 11–23.
  <https://info-central.rocketlabdelta.com/archive/design_finflutter.html>
- **S9.** Howard, Peak of Flight 291 (Apogee Components, 2011), pp. 5–6.
- **S10.** MIT Rocket Team, NASA Student Launch Flight Readiness Review (2012), pp. 31–35, Table 6 p. 57, and its
  addendum (April 2012).
- **S11.** Nakka, "Fin Design Case Study 1" (flight C-34, 1984). <https://www.nakka-rocketry.net/fin-casestudy-1.htm>
- **S12.** Foster, Utah State University honors thesis (2023), pp. 2–3, 6–7.
  <https://digitalcommons.usu.edu/honors/966>
- **S13.** Martin, NACA TN 4197 (1958), eqs. 10–12 pp. 12–13, eq. 18, references pp. 16–17.
- **S14.** Bennett, Fin-Flutter-Velocity-Calculator, BSD-2-Clause, commit ef5e50a: worked examples, none measured.
  <https://github.com/jkb-git/Fin-Flutter-Velocity-Calculator>
