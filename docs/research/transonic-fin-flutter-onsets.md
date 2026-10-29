# Measured flutter onsets near Mach 1, against Martin's formula

**In short: three more reports give measured onsets of low-aspect-ratio wings between Mach 0.8 and 1.2 with the geometry
and stiffness stated, none among TN 4197's cited references, and on them HPR Sim's reading of Martin's eq. 18 reads up
to 1.41 times the onset, the same worst case R&M 2944 gave.** NASA TN D-1594 (1963) tested 19 solid, about 4% thick
wings of seven planforms in a transonic tunnel at air densities near and above sea level's; 18 models of six planforms
have a stiffness curve (nine their own, nine borrowed or scaled). TM X-136 (1959) adds a sharply tapered wing, and ARC
R&M 2949 (dated 1953) adds 32 swept wooden wings flown on rockets by the R&M 2944 group. The ratio runs 0.57 to 1.41
over 153 onset points between Mach 0.8 and 1.2; 76 of them read above 1, the flattering side. It peaks near Mach 1
because the formula fixes one flutter dynamic pressure at every Mach number, while the measured one dips there. Solid
swept wings read highest (1.37 at 60°, 1.31 and 1.21 at 45°, the last a magnesium wing); unswept or 30° wings read 0.59
to 0.98. R&M 2949 also describes how its group measured the "stiffness at 0.7 s", which supports the reading used for
R&M 2944's column. How sure: the onsets and air states are tabulated; the stiffness is read from plotted curves, and it
moves a ratio by up to −26% and +24% on the softest-tipped wing. This moves M2.11's flattering-side bound and the
scoreboard's flutter row.

Sources read 2026-10-29. Ratios are this note's computation (**Inference**): Martin's flutter speed over the measured
onset, exactly as in [flutter-measured-onsets, section 6](flutter-measured-onsets.md#6-how-the-ratios-were-computed),
with $G_E = 6\,GJ/(c\,t^3)$ from the measured torsional stiffness $GJ$ (TN 4197 eq. 12, p. 13). Above 1 is the
flattering side. None of these wings is in [flutter-measured-onsets](flutter-measured-onsets.md) or
[fin-flutter-tests](fin-flutter-tests.md), and none is TN 4197's RM L51A11, RM L53G10b or TN 3312. TN D-1594 supersedes
two earlier Langley reports (RM L53G10a, 1953; RM L55I15a, 1956), and TN 4197 says its figure 3 holds tunnel data beyond
its reference list (p. 6), so whether those earlier points sit in Martin's figure is unknown.

## 1. Why the error peaks near Mach 1

- **The formula fixes a flutter dynamic pressure.** Eq. 18 sees the air only through $\rho a^2 = \gamma p$, so its speed
  is a fixed $q_f$ for each fin, the same at every Mach number and height
  ([the flutter page](../physics/flutter.md#the-criterion)). The ratio is then $\sqrt{q_f/q_e}$, with $q_e$ the
  measured dynamic pressure at onset. Its Mach-number dependence is that of $q_e$ alone.
- **The measured $q_e$ dips near Mach 1.** On most tunnel wings the lowest flutter dynamic pressure lies between Mach
  0.78 and 1.16 (TM X-136's 3% wing at 0.68), then rises steeply: on TN D-1594's 645 wing from 1,103 lb/ft² at Mach
  1.153 to 2,586 at Mach 1.321 (S1, Table II(g), p. 30). R&M 2949 tabulates no dynamic pressure; its
  measured-over-calculated speeds bottom out near Mach 1.1, or 0.95 resolved normal to the sweep (S3, s. 6.3.1, p. 6).
  So the ratio peaks at the dip; above Mach 1.2 the solid wings fall to 0.48 to 0.88, except the 60° wings (1.20 to
  1.36).
- **Not light air.** AGARD 445.6 read high where its tunnel air was thinnest. Here the flattering ratios sit at
  0.0018 to 0.0054 slug/ft³ (0.93 to 2.8 kg/m³), 0.76 to 2.3 times sea level's 0.00238 (S1, S2, Table II):
  the dip, not light air.

## 2. Solid wings in the Langley transonic blowdown tunnel (TN D-1594)

Unangst and Jones (S1) tested seven planforms at Mach 0.76 to 1.42, all with taper 0.6 and NACA 65A004 sections (the 60°
wings came out about 5% thick). The AR-4 wings have a 3.91 in (99 mm) streamwise root, 2.57 in (65 mm) tip and 5.35 in
(136 mm) panel span, panel aspect ratio 1.65 (Fig. 1 p. 31, Table I). Models were solid: compreg (resin-impregnated,
compressed maple), some wrapped in 0.006 or 0.018 in of glass cloth, one perforated aluminum and one magnesium (p. 5).
Roots were held by filler blocks and four bolts on a 3 in sting (p. 6). The tunnel pressure was raised at about constant
Mach number until an observer and root strain gauges saw flutter, then shut (pp. 6–7); Table II gives Mach number,
density, speed and dynamic pressure at every first flutter point, so each counted point is an onset at that Mach number.

| Wing (S1) | Material, models | Sweep, $A$ | $G_E$ median, ksi | Points at M 0.8–1.2 | Ratio there | Above M 1.2 |
|---|---|---|---|---|---|---|
| 400 | compreg, 3 | 0°, 1.65 | 440–483 | 21 | 0.84–0.93 | 0.61–0.72 |
| 430 | compreg + glass, 5 | 30°, 1.65 | 306–386 | 15 | 0.59–0.98 | 0.50–0.60 |
| 445 | compreg (+ glass), 3 | 45°, 1.65 | 378–577 | 25 | 0.72–1.19 | 0.49–0.70 |
| 452 | compreg + glass, 1 | 52.5°, 1.65 | 509 | 12 | 0.90–1.02 | 0.73–0.88 |
| 460 | compreg + glass, 4; aluminum, 1 | 60°, 1.65 | 467; 1,336 | 23 | 1.17–1.37 | 1.20–1.36 |
| 645 | magnesium, 1 | 45°, 2.75 | 2,408 | 14 | 1.07–1.21 | 0.77–0.87 |

- **Counts:** 155 onset points from 18 models, counted from Table II pp. 24–30 (a point counts where a panel enters
  flutter); 110 lie at Mach 0.8 to 1.2 and 48 of those read above 1. Nine models have their own measured stiffness
  curve; eight take a sibling's, scaled by torsion frequency squared, and the 645's is scaled from aluminum. One 460 row
  has no tabulated dynamic pressure; it is computed from density and speed (above Mach 1.2, no band figure moves). The
  245 wing (panel aspect ratio 0.91, pine and balsa) has no measured stiffness and the report says its points may lie
  above the boundary (p. 12): not scored.
- **The 645 check.** Its stiffness curve is the report's own scaling of an aluminum wing's measured one by the moduli of
  the two metals (Fig. 6, p. 35), and this note's $G_E$ from it is 2.25 to 2.47 Msi (15.5 to 17.0 GPa) along the span, a
  magnesium modulus. So $GJ$ taken on the section normal to the sweep, with $J = c\,t^3/6$, recovers the material's
  shear modulus, which is HPR Sim's reading for a solid fin. Its ratio, 1.07 to 1.21, is a metal fin on the
  flattering side at Mach 0.83 to 1.15, highest (1.21) at the dip, Mach 1.153, $q_e$ = 1,103 lb/ft².
- **The 460 wings** read high at every Mach number tested, so sweep or their soft tips, not the dip alone, drive them.
  Their stiffness falls steeply outboard (Fig. 7(a)), so the station read moves the ratio from 1.01 to 1.70 at the
  worst point, 1.37 (model 2, Mach 0.985, 1,434 lb/ft², p. 29). The aluminum model reads 1.29 at Mach 0.867. The
  compreg models often cracked inside the root block in subsonic flutter (p. 12), which lowers some onsets.
- **The 445 wings** read above 1 at Mach 0.84 to 0.94 only (up to 1.19, model 3 at Mach 0.882, 971 lb/ft², p. 27).

## 3. A taper-0.2 wing in the same tunnel (TM X-136)

Unangst (S2) tested one planform, 45° sweep and taper 0.2 (panel aspect ratio 1.83, panel taper 0.242, 4.75 in or
121 mm streamwise root), machined from solid Consoweld, a resin-impregnated paper, at 3% and 4% thickness (p. 5,
Table I p. 14). Table II (pp. 16–17) gives Mach number, density, speed and dynamic pressure at each start of flutter.
- **4% wings:** 13 onsets at Mach 0.76 to 0.95 read 1.03 to 1.31; the highest is model 2 at Mach 0.871, 7.60 lb/in²
  (52 kPa). Second- and third-region onsets at Mach 1.06 to 1.09 read 0.69 to 0.95. At Mach 1.10 to 1.18 four runs
  reached 28 to 30 lb/in² without flutter, a survival bound only.
- **3% wings:** 0.97 to 1.05 at Mach 0.64 to 0.74, 0.48 to 0.65 at Mach 1.05 to 1.39.
- **Counts:** 39 onsets (first, second and third occurrences, Table II); 23 lie at Mach 0.8 to 1.2, 11 above 1.
- **Stiffness:** only the 4% model 1 has a measured curve (Fig. 2, p. 20); $G_E$ along it is 0.62 to 0.99 Msi, median
  0.70 Msi (4.8 GPa). The 3% wings take the same material modulus; model 2 is scaled by its torsion frequency squared.
  The station read moves these ratios between −6% and +19%.

## 4. Swept wooden wings flown on rockets (R&M 2949)

Molyneux and Ruddlesden (S3), the R&M 2944 group at RAE, flew 37 untapered wings swept 20°, 40° and 60° on 3 in and
5 in rockets (report dated October 1953; no flight dates given): 32 gave a flutter speed, four didn't flutter up to
about 2,000 ft/s and one lost its telemetry (Table 4, p. 11; counted). Each wing is 1.53 ft (466 mm) root to tip with a
1.06, 1.31 or 2.00 ft (323, 399 or 610 mm) chord in the line of flight, 9.4%, 7.7% or 5.0% thick: Martin's aspect ratio
1.44, 1.17 or 0.77 (Table 1, p. 8). Of the 32 scored, 22 are plywood and balsa with spruce or Alclad box spars, six
are plywood and balsa with no spar, and four are solid spruce (Table 2, p. 9). Speed came from Doppler radar; the
onset is where divergent or, on 13 models, intermittent oscillation began (s. 3, s. 6.1, pp. 2–4), so onsets are
measured, not bounds.

- **Ratios (Inference):** 0.82 to 1.41, median 1.17, 27 of 32 above 1. At Mach 0.8 to 1.2: 0.84 to 1.41, 17 of 20
  above 1. By sweep: 20° wings 1.20 to 1.41, 40° 1.04 to 1.31, 60° 0.82 to 1.18. The highest, 1.41, is model 1161 (20°,
  Mach 1.14, 1,275 ft/s, $m_\theta$ = 2,520 lb·ft/rad; Tables 3 and 4, pp. 10–11).
- **Inputs:** $GJ = 0.7\,s\,m_\theta$ from the line-of-flight torsional stiffness $m_\theta$ (Table 3), the
  line-of-flight chord and thickness, $p$ at sea level and $a = V/M$ from Table 4 (ground launch; air assumed, as
  for R&M 2944).
  $G_E$ comes out 12 to 128 ksi (0.08 to 0.88 GPa). Most, like R&M 2944's, carry their stiffness in a spar with
  soft balsa edges, the built-up class that already read high; the four solid-spruce wings read 1.17 to 1.31 too
  (models 1124, 1164, 1172, 1178).
- **The stiffness convention.** The report defines $m_\theta$ as the "wing torsional stiffness measured at
  0.7 s" and describes the test: root rigidly fixed, a pure torque applied to the section at 70% semi-span, and that
  section's rotation measured (s. 3, p. 2; Table 3 notation). For a uniform cantilever that is $GJ/(0.7\,s)$, the
  reading flutter-measured-onsets used for R&M 2944. R&M 2944 uses the same notation but doesn't describe its test, so
  that its 0.92 to 1.41 stands on the group's own description is an inference.

## 5. How far the inputs move these ratios

| Input | Direction | Size | Applies to |
|---|---|---|---|
| Station on the stiffness curve (lowest to highest $G_E$) | either | −26% to +24% (460), −3% to +1% (645) | S1, S2 |
| Streamwise instead of normal chord in $G_E$ | lower | ×0.96 (30°) to ×0.73 (60°); 645 ×0.86 | S1, S2 |
| Untested models scaled by torsion frequency² | either | up to ×1.40 in $G_E$, ×1.18 ratio (460 model 4) | S1, S2 |
| Panel-mean torsion frequency where panels differ | either | about ±5% | S1 |
| $GJ$ taken as $m_\theta$ times 1 ft, or times $s$ | lower, higher | −3%, +20% | S3 |
| Stiffness measured normal to the sweep (s. 3 allows either) | either | roughly ×0.81 to ×1.49, most at 60° | S3 |
| Air at sea level (S3 only) | unbounded here | flights near the ground; heights not given | S3 |

The normal-chord reading is the one that recovers magnesium's modulus on the 645 wing (section 2). On the streamwise
reading the 460 wings read 0.85 to 1.00, the 645 wing 0.92 to 1.04 and TM X-136's 4% wing 0.96 to 1.22. Scripts:
`refs/research/transonic-fin-flutter-onsets/scripts/` (d1594.py, x136.py, rm2949.py); d1594.py reproduces the
project's aluminum example (1,169 m/s), and all three use the same $q_f$.

## 6. Leads read and not scored

- **TN 3902 (supersedes RM L51C08, 1951), free-fall bombs:** four solid aluminum wings fluttered at Mach 0.85, 1.03 and
  1.07, but each is three chords long and the 1951 bomb-drop data may sit in Martin's figure 3, which TN 4197
  says includes "rocket and bomb-drop tests" (S10, p. 6). RM L8I30 (1948, one wing at Mach 0.92) predates the line too.
- **RM L52K14 (1953):** tunnel copies of the free-fall wings, aspect ratio 7.38, not low.
- **TM X-79 (1959):** foam-covered aluminum inserts and flat plates at Mach 0.70 to 1.10, tested in Freon-12 (p. 3),
  which the earlier notes don't score.
- **Survival only:** RM L51K28a (magnesium and steel 45° wings, no flutter to Mach 1.23) and the YF-102 scale wings.
  The F-100 scale wings (solid magnesium, 45°) met mild flutter near their first bending frequency at Mach 1.06 to
  1.15, a one-mode bending flutter that eq. 18's torsion criterion doesn't model; stiffness is given as influence
  coefficients (S9, Summary, p. 5). Not scored here.
- **RM L57F24** stays figure-only: experiment over theory, no dimensions. **R&M 2944's stiffness column:** section 4.

## What the project can use

- **Cases for M2.11, by license.** Public domain (US government work, cached): TN D-1594's 18 scored models (155
  onsets, air measured per point) and TM X-136's five (39 onsets). Cite only (Crown copyright, published 1956, cached
  under `refs/`): R&M 2949's 32 wings, air assumed. TN D-1594 and TM X-136 add a Langley transonic-tunnel source; R&M
  2949 adds wings, not a group.
- **The flattering-side bound near Mach 1** stays 1.41, now on two RAE reports, and adds solid wings: 1.37 (60° compreg,
  1.01 to 1.70 by station), 1.31 (TM X-136) and 1.21 for a solid metal wing (645, 1.17 to 1.22 by station).
- **A per-point check.** Since the ratio is $\sqrt{q_f/q_e}$, the tables' $q_e$ columns score directly, no air state
  needed, once $q_f$ is computed for each wing.

## What this suggests

- The flutter page could say that a solid fin, not only a built-up wing, can read up to about 1.2 to 1.4 times the
  onset near Mach 1 at ordinary air density, worst on swept fins; expected effect, a flattering-side margin stated for
  Mach 0.85 to 1.15 instead of none.
- A transonic factor on $q_f$ of about 0.5 (1/1.41²) between Mach 0.85 and 1.15 would put every measured case here on
  the safe side. It would be a fitted bound; applied above Mach 1.2 too, it would make the speed 1.6 to 2.9 times
  conservative on most wings, but only 1.04 to 1.18 on the 60° wings.
- The 645 wing (solid magnesium, stiffness from the report's scaling) is the cleanest new case for a sign test: no
  station ambiguity worth more than 3%.

## Open questions

- The stiffness station: TN D-1594's curves vary 1.5 to 3 times along the span on the swept wings, and Martin doesn't
  say where $JG$ is measured on a tapered wing. A whole-wing (tip twist) reading would sit nearer the low end.
- Sweep: eq. 18 has no sweep term, and the 60° compreg wings read high at every Mach number. Uniform swept wings read
  high too (R&M 2949's 60° solid spruce 1.17 and 1.18, the 45° magnesium 645 1.07 to 1.21), which points at sweep more
  than soft tips; a 60° wing of one known modulus in a tunnel would settle it.
- The figures were read by pixel on one render; a second trace found the 645 curve first read 1–7% high (corrected
  here), and other figures agree within about 1–6% apart from the tip points. TN D-1594 Figs. 2, 3 and 5 were traced
  once. The 194 onset rows were transcribed by hand from scans; all were re-read against the tables.
- No case is a fin bonded to a tube; the soft-root factor of fin-flutter-tests (0.77 to 0.98) would add to these.

## Sources (NTRS: <https://ntrs.nasa.gov/citations/ID>), all read 2026-10-29

- **S1.** Unangst and Jones, NASA TN D-1594 (1963), pp. 1–7, 12, Table I pp. 17–23, Table II pp. 24–30, Figs. 1–7
  pp. 31–36. ID 19630002604. Public domain.
- **S2.** Unangst, NASA TM X-136 (1959), pp. 1–5, Table I pp. 14–15, Table II pp. 16–17, Fig. 2 p. 20. ID 19980236841.
- **S3.** Molyneux and Ruddlesden, ARC R&M 2949 (1953, published 1956), ss. 2–7 pp. 1–6, Tables 1–4 pp. 8–11.
  <https://aerade.cranfield.ac.uk/handle/1826.2/3512>. Crown copyright; cite only.
- **S4.** Doggett, Rainey and Morgan, NASA TM X-79 (1959), pp. 1–5. ID 19660024036.
- **S5.** Lauten and Nelson, NACA TN 3902 (1957), Table I. ID 19930084784; RM L51C08 (1951), ID 19930086625.
- **S6.** Bursnall, NACA RM L52K14 (1953), Summary. ID 19930087443.
- **S7.** Lundstrom, Lauten and Angle, NACA RM L8I30 (1948), Summary. ID 19930085467.
- **S8.** Lauten and Sylvester, NACA RM L51K28a (1952), Summary. ID 19930087975.
- **S9.** O'Kelly, free-flight tests of YF-102 and F-100 scale wings (1954–1955), Summaries. IDs 20090023313,
  20090023309.
- **S10.** Martin, NACA TN 4197 (1958), pp. 5–6, eq. 12 p. 13, references pp. 16–17. ID 19930085030.
- **S11.** Molyneux, Ruddlesden and Cutt, ARC R&M 2944 (1951), Table 1 p. 8.
  <https://aerade.cranfield.ac.uk/handle/1826.2/3507>
