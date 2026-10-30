# Measured flutter onsets near Mach 1, against Martin's formula

Re-checked 2026-10-29: 21 claims corrected.

**In short: three more reports give measured onsets of low-aspect-ratio wings between Mach 0.8 and 1.2 with geometry
and stiffness stated, and on them HPR Sim's reading of Martin's eq. 18 reads up to 1.41 times the onset on stiffness
measured in the line of flight, R&M 2944's worst case, and up to 1.47 on the stiffness R&M 2949 also measured normal to
the sweep, read as for a solid fin (1.37 in R&M's own swept frame).** NASA TN D-1594 (1963) tested 19 solid, about 4%
thick tunnel wings at air densities near and above sea level's, TM X-136 (1959) a sharply tapered one, and ARC R&M 2949
(dated 1953) 32 swept built-up and solid-spruce wings flown on rockets. Over 153 onsets at Mach 0.8 to 1.2 the ratio
runs 0.57 to 1.41, 76 above 1, the flattering side (to 1.47, 79 above 1, on R&M 2949's normal stiffness). It peaks near
Mach 1 because the formula fixes one flutter dynamic pressure at every Mach number, while the measured one dips there.
Of the tunnel wings, solid swept ones read highest (1.37 at 60°, 1.31 and 1.21 at 45°, the last magnesium); unswept or
30° ones 0.59 to 0.98. How sure: tunnel air is tabulated, R&M 2949's assumed; stiffness is read from plotted curves,
and the station read moves a ratio by a quarter to a third either way on the softest-tipped wing. This moves M2.11's
flattering-side bound and the scoreboard's flutter row.

Sources read 2026-10-29. Ratios are this note's computation (**Inference**): Martin's flutter speed over the measured
onset, as in [flutter-measured-onsets, section 6](flutter-measured-onsets.md#6-how-the-ratios-were-computed), with
$G_E = 6\,GJ/(c\,t^3)$ from the measured torsional stiffness $GJ$ (TN 4197 eq. 12, p. 13); above 1 is the flattering
side. No wing is in [flutter-measured-onsets](flutter-measured-onsets.md), [fin-flutter-tests](fin-flutter-tests.md)
or TN 4197's references (such as RM L51A11, RM L53G10b, TN 3312). TN D-1594 supersedes RM L53G10a (1953) and
RM L55I15a (1956); TN 4197's figure 3 holds tunnel data beyond its references (p. 6), so those may sit in it.

## 1. Why the error peaks near Mach 1

- **The formula fixes a flutter dynamic pressure.** Eq. 18 sees the air only through $\rho a^2 = \gamma p$, so its speed
  is a fixed $q_f$ per fin at every Mach number and height ([the flutter page](../physics/flutter.md#the-criterion)).
  The ratio is then $\sqrt{q_f/q_e}$, with $q_e$ the measured onset dynamic pressure; it varies with Mach as $q_e$ does.
- **The measured $q_e$ dips near Mach 1.** On the tunnel planforms the lowest flutter dynamic pressure lies between
  Mach 0.78 (one point) and 1.15 (TM X-136's 3% wing at 0.68), then rises steeply: on TN D-1594's 645 wing from 1,103
  lb/ft² at Mach 1.153 to 2,586 at 1.321 (S1, Table II(g), p. 30). On several models it sits at the lowest Mach tested,
  so the fall into the dip shows on some wings only. The lower edge of R&M 2949's measured over 2-D incompressible
  speeds bottoms out near Mach 1.1 (0.95 normal to the sweep; S3, s. 6.3.1, p. 6). Above Mach 1.2 the solid wings fall
  to 0.48 to 0.88, except the 60° wings (1.20 to 1.36).
- **Not light air.** AGARD 445.6 read high where its tunnel air was thinnest. Here the flattering ratios at Mach 0.8 to
  1.2 sit at 0.0018 to 0.0050 slug/ft³ (0.93 to 2.6 kg/m³), 0.76 to 2.1 times sea level's 0.00238 (S1, S2, Table II).

## 2. Solid wings in the Langley transonic blowdown tunnel (TN D-1594)

Unangst and Jones (S1) tested seven planforms at Mach 0.66 to 1.42 (Table II; 0.76 in the summary), all with taper 0.6
and NACA 65A004 sections (the 60° wings about 5% thick); the AR-4 wings have a 3.91 in (99 mm) streamwise root, 2.57 in
(65 mm) tip and 5.35 in (136 mm) panel span, panel aspect ratio 1.65 (Fig. 1 p. 31, Table I). Models were solid compreg
(resin-impregnated, compressed maple), some glass-wrapped (0.006 or 0.018 in), one perforated aluminum and one magnesium
(p. 5), bolted to a 3 in sting (p. 6). Table II gives Mach number, density, speed and dynamic pressure at every first
flutter point, found by raising tunnel pressure at about constant Mach number (pp. 6–7), so each point is an onset.

| Wing (S1) | Material, models | Sweep, $A$ | $G_E$ median, ksi | Points at M 0.8–1.2 | Ratio there | Above M 1.2 |
|---|---|---|---|---|---|---|
| 400 | compreg, 3 | 0°, 1.65 | 440–483 | 21 | 0.84–0.93 | 0.61–0.72 |
| 430 | compreg + glass, 5 | 30°, 1.65 | 306–386 | 15 | 0.59–0.98 | 0.50–0.60 |
| 445 | compreg (+ glass), 3 | 45°, 1.65 | 378–577 | 25 | 0.72–1.19 | 0.49–0.70 |
| 452 | compreg + glass, 1 | 52.5°, 1.65 | 509 | 12 | 0.90–1.02 | 0.73–0.88 |
| 460 | compreg + glass, 4; aluminum, 1 | 60°, 1.65 | 467; 1,336 | 23 | 1.17–1.37 | 1.20–1.36 |
| 645 | magnesium, 1 | 45°, 2.75 | 2,408 | 14 | 1.07–1.21 | 0.77–0.87 |

- **Counts:** 155 onset points from 18 models (Table II pp. 24–30; a point counts where a panel enters flutter); 110 lie
  at Mach 0.8 to 1.2 and 48 of those read above 1. Nine models have their own stiffness curve; eight take a sibling's,
  scaled by torsion frequency squared, and the 645's is scaled from aluminum (one 460 row's $q$, above Mach 1.2, is
  computed from density and speed). The 245 wing (pine and balsa, panel aspect ratio 0.91) has no measured stiffness,
  and its points "do not necessarily identify the precise flutter boundary" (p. 12): not scored.
- **The 645 check.** Its stiffness curve is the report's scaling of an aluminum wing's measured one by the metals'
  moduli (Fig. 6, p. 35); $G_E$ from it is 2.25 to 2.47 Msi (15.5 to 17.0 GPa) along the span, a magnesium modulus, so
  $GJ$ on the section normal to the sweep with $J = c\,t^3/6$ recovers the shear modulus, HPR Sim's solid-fin reading.
  It reads 1.07 to 1.21 at Mach 0.83 to 1.15, a metal fin on the flattering side, highest at Mach 1.153, 1,103 lb/ft².
- **The 460 wings** read high at every Mach number tested, so sweep or their soft tips, not the dip alone, drive them.
  Their stiffness falls steeply outboard (Fig. 7(a)): the station moves the worst point, 1.37 (model 2, Mach 0.985,
  1,434 lb/ft², p. 29), from 1.01 to 1.70 as scripted. Fig. 7(a) has two root $GJ$ points, about 1,720 and 1,920
  lb·in²; the script takes the lower, the faired curve the higher, which lifts the top to 1.80, and four traces put
  the tip at 175 to 205 lb·in², not 217, lowering the bottom to 0.91 to 0.98. The aluminum model reads 1.29 at Mach
  0.867. Compreg models often cracked inside the root block in subsonic flutter (p. 12), lowering some onsets.
- **The 445 wings** read above 1 at Mach 0.79 to 0.94 only (up to 1.19, model 3 at Mach 0.882, 971 lb/ft², p. 27;
  model 1 reads 1.03 at Mach 0.794).

## 3. A taper-0.2 wing in the same tunnel (TM X-136)

Unangst (S2) tested one planform, 45° sweep and taper 0.2 (panel aspect ratio 1.83, panel taper 0.242, 4.75 in or
121 mm streamwise root), machined from solid Consoweld, a resin-impregnated paper, at 3% and 4% thickness (p. 5,
Table I pp. 14–15). Table II (pp. 16–17) gives Mach number, density, speed and dynamic pressure at each flutter start:
39 onsets (first, second and third occurrences), 23 at Mach 0.8 to 1.2, 11 of them above 1.
- **4% wings:** 13 onsets at Mach 0.76 to 0.95 read 1.03 to 1.31, highest model 2 at Mach 0.871, 7.60 lb/in² (52 kPa).
  Later-region onsets at Mach 1.06 to 1.09 read 0.69 to 0.95; four runs at Mach 1.10 to 1.18 reached 28 to 30 lb/in²
  unfluttered. **3% wings:** 0.97 to 1.05 at Mach 0.64 to 0.74, 0.48 to 0.65 at Mach 1.05 to 1.39.
- **Stiffness:** only 4% model 1 has a measured curve (Fig. 2, p. 20): $G_E$ 0.62 to 0.99 Msi, median 0.70 (4.8 GPa).
  The 3% wings take that modulus, model 2 scaled by torsion frequency squared; the station moves ratios −6% to +19%.

## 4. Swept built-up wings flown on rockets (R&M 2949)

Molyneux and Ruddlesden (S3), the R&M 2944 group at RAE, flew 37 untapered wings swept 20°, 40° and 60° on 3 in and
5 in rockets (report dated October 1953; no flight dates given): 32 gave a flutter speed, four didn't flutter up to
about 2,000 ft/s (s. 6.1, p. 4) and one lost its telemetry (Table 4, p. 11; counted). Each wing is 1.53 ft (466 mm) root
to tip, normal to the root, with a 1.06, 1.31 or 2.00 ft (323, 399 or 610 mm) chord in the line of flight, 9.4%, 7.7%
or 5.0% thick: Martin's aspect ratio 1.44, 1.17 or 0.77 (Table 1, p. 8). Normal to the sweep all have one 1.0 ft, 10%
thick section (s. 2 pp. 1–2, Table 1). Of the 32 scored, 22 are plywood and balsa on a spar (18 Alclad box or flanged
box, four solid spruce), six have no spar, four are solid spruce (Table 2, p. 9). Doppler radar gave the speed at which
divergent or, on 13 models, intermittent oscillation began (s. 3, s. 6.1, pp. 2–4): measured onsets.

- **Ratios on line-of-flight stiffness (Inference):** 0.82 to 1.41, median 1.17, 27 of 32 above 1; at Mach 0.8 to 1.2,
  0.84 to 1.41, 17 of 20 above 1. By sweep: 20° wings 1.20 to 1.41, 40° 1.04 to 1.31, 60° 0.82 to 1.18. The highest,
  1.41, is model 1161 (20°, Mach 1.14, 1,275 ft/s, $m_\theta$ = 2,520 lb·ft/rad; Tables 3 and 4, pp. 10–11).
- **Ratios on normal-to-sweep stiffness (Inference):** 0.89 to 1.47, median 1.30, 30 of 32 above 1; at Mach 0.8 to
  1.2, 1.21 to 1.47, all 20 above 1. Highest again 1161, 1.47 (normal $m_\theta$ = 2,400 lb·ft/rad), then 1145 1.46
  (40°, Mach 0.97), 1146 1.44 (40°, Mach 1.00) and 1172 1.42 (60°, Mach 1.02).
- **Inputs:** $GJ = 0.7\,s\,m_\theta$ (Table 3), line-of-flight chord and thickness, $p$ at sea level, $a = V/M$
  (Table 4; air assumed as for R&M 2944; on 30 of 32 wings $M$ is $V$/1,117 ft/s within rounding, a standard day,
  Inference). $G_E$ is 12 to 128 ksi (0.08 to 0.88 GPa). Most carry their stiffness in a spar with soft balsa edges,
  R&M 2944's built-up class that read high; the four solid-spruce wings read 1.17 to 1.31 too (1.27 to 1.42 normal).
- **The stiffness convention.** Table 3's $m_\theta$ is the "wing torsional stiffness measured at 0.7 s": a pure torque
  on a section at 70% semi-span, root fixed, in the line of flight and again normal to the sweep (s. 3, p. 2). For a
  uniform cantilever the first is $GJ/(0.7\,s)$, flutter-measured-onsets' reading for R&M 2944, whose test isn't
  described (its 0.92 to 1.41 rests on this group's description: an inference). R&M 2949 allows either (s. 6.2, p. 5;
  s. 7, p. 6), the normal one scattering less about its own formula. As a uniform beam along the spar,
  $GJ = 0.7\,(s/\cos\Lambda)\,m_\theta$ on the 1.0 ft normal chord, in eq. 18 with the streamwise planform as for the
  645, it scales the ratio by $\sqrt{(m_{\theta,n}/m_{\theta,f})/\cos^2\Lambda}$ (Inference); R&M's own equivalent
  unswept wing (s. 4, p. 3) gives 0.43 to 1.37, not how HPR Sim takes a fin's inputs.

## 5. How far the inputs move these ratios

| Input | Direction | Size | Applies to |
|---|---|---|---|
| Station on the stiffness curve | either | 460: −26% to +24% (−28 to −34%, +31% re-read); 645: −3% to +1% | S1, S2 |
| Streamwise instead of normal chord in $G_E$ | lower | ×0.96 (30°) to ×0.73 (60°); 645 ×0.86 | S1, S2 |
| Untested models scaled by torsion frequency² | either | up to ×1.40 in $G_E$, ×1.18 ratio (460 model 4) | S1, S2 |
| Panel-mean torsion frequency where panels differ | either | about ±5% | S1 |
| $GJ$ taken as $m_\theta$ times 1 ft, or times $s$ | lower, higher | −3%, +20% | S3 |
| Stiffness normal to the sweep (s. 6.2, s. 7) | mostly higher | ×0.81 to ×1.49; raises 19 of 20 at Mach 0.8–1.2 | S3 |
| Sea-level air, standard day | higher with height | ≤ +0.7% launch path; +0.6% per 100 m of site; weather ±1–2% | S3 |

The normal-chord reading recovers magnesium's modulus on the 645 wing (section 2); on the streamwise one the 460 wings
read 0.85 to 1.00, the 645 0.92 to 1.04 and TM X-136's 4% wing 0.96 to 1.22. R&M 2949's normal factor is ×0.95–1.08
at 20°, ×0.81–1.29 at 40°, ×0.96–1.49 at 60°. Its ratio goes as $1/\sqrt{p}$ at fixed $a$ and as $a$ at fixed $p$: a
12½° launch and 1.6 s burn put onsets at most about 63 m (3 in) or 111 m (5 in) above the launcher (S3 ss. 2–3, S11
s. 2), the site's height isn't given, and weather moves it about 1% per 2% of pressure and 1.7% per 10 K (Inference).
Scripts, all with one $q_f$, under `refs/research/`: `transonic-fin-flutter-onsets/scripts/` (d1594.py, x136.py,
rm2949.py; d1594.py reproduces the project's aluminum example, 1,169 m/s) and
`verify-transonic-fin-flutter-onsets/scripts/` (rm2949_readings.py, qf_factor.py).

## 6. Leads read and not scored

- **TN 3902 (supersedes RM L51C08, 1951), free-fall bombs:** four solid aluminum wings fluttered at Mach 0.85, 1.03
  and 1.07, but each is three chords long, and the 1951 data may sit in TN 4197's figure 3 ("rocket and bomb-drop
  tests", S10, p. 6); RM L8I30 (1948, one wing at Mach 0.92) too. RM L52K14 (1953): their tunnel copies, aspect
  ratio 7.38. TM X-79 (1959): foam-covered aluminum inserts and flat plates at Mach 0.70 to 1.10 in Freon-12 (p. 3).
- **Survival only:** RM L51K28a (45° magnesium and steel, no flutter to Mach 1.23), YF-102 scale wings. The F-100's
  (solid magnesium, 45°) met mild bending flutter at Mach 1.06 to 1.15, outside eq. 18's torsion criterion (S9, p. 5).

## What the project can use

- **Cases for M2.11, by license.** Public domain (US government work, cached): TN D-1594's 18 scored models (155
  onsets, air measured per point) and TM X-136's five (39 onsets), a Langley transonic-tunnel source. R&M 2949's 32
  wings (air assumed) add wings, not a group. Its Crown copyright (published 1956) ended in the UK at the end of 2006
  (S12: CDPA Sch. 1 para. 41(2)(a), 1956 Act s. 39), but US copyright was likely restored to 2051 (17 U.S.C. § 104A);
  measured values are facts, so a table the project lays out, citing it, looks defensible, a maintainer's call (tables,
  scans and PDF stay in `refs/`; Inference, not legal advice; R&M 2944 is the same case).
- **The flattering-side bound near Mach 1** is 1.41 on line-of-flight stiffness, on two RAE reports, and 1.47 on R&M
  2949's normal stiffness (1161, Mach 1.14). Solid wings: 1.37 (60° compreg; 1.01 to 1.70 by station as scripted,
  0.91 to 1.80 re-read), 1.31 (TM X-136) and 1.21 for a solid metal wing (645, 1.17 to 1.22 by station).

## What this suggests

- The flutter page could say a solid fin, not only a built-up one, can read about 1.2 to 1.4 times the onset near
  Mach 1 at ordinary density, worst when swept; expected effect, a flattering-side margin for Mach 0.85 to 1.15.
- A transonic factor on $q_f$ for Mach 0.85 to 1.15 depends on the stiffness reading. 0.5 (1/1.41²) clears every
  tunnel point on the central reading (0.53 needed) and R&M 2949 on line-of-flight stiffness, but leaves four R&M 2949
  wings at 1.01 to 1.04 on the normal one (0.46 needed) and, on the stiffest-station reading, 27 tunnel points (20 on
  the 460 wings, 7 on TM X-136's 4% wing) at up to 1.20 (0.35 needed; 0.31 with the 460's faired root). It is a fitted
  bound: at 0.5 above Mach 1.2 too, the tunnel wings' speed is 1.6 to 3.0 times conservative, 1.04 to 1.18 on the 60°
  ones (up to 1.19 high on their stiffest station); R&M 2949's two wings there, 1.1 and 1.7 (1.1 and 1.2 normal).
- The 645 wing (solid magnesium) is the cleanest new case for a sign test: no station ambiguity above 3%.

## Open questions

- The station: along TN D-1594's swept wings $G_E$ varies 1.4 to 2.8 times (1.1 on the 645), $GJ$ 1.7 to 8 times, and
  Martin doesn't say where $JG$ is measured on a tapered wing; a tip-twist reading would sit nearer the low end.
- Which R&M 2949 stiffness matches HPR Sim's inputs: its four solid-spruce wings share one normal section yet give 28
  to 115 ksi on the normal reading, so neither reading is a clean material test there.
- Sweep: eq. 18 has no sweep term, and the 60° compreg wings read high at every Mach number. Uniform swept wings read
  high too (R&M 2949's 60° solid spruce 1.17 and 1.18, or 1.42 and 1.37 normal; the 45° magnesium 645 1.07 to 1.21),
  which points at sweep more than soft tips; a 60° wing of one known modulus in a tunnel would settle it.
- Figures were read by pixel; a second trace found the 645 curve first read 1–7% high (corrected here), and others agree
  within about 1–6% apart from the tips, whose re-reads (Fig. 7(a); TM X-136 Fig. 2, 63 against 70 lb·in²) aren't in
  the scripts. TN D-1594 Figs. 2, 3 and 5 were traced once. All 194 hand-transcribed onset rows were re-read.
- No case is a fin bonded to a tube; the soft-root factor of fin-flutter-tests (0.77 to 0.98) would add to these.

## Sources (NTRS: <https://ntrs.nasa.gov/citations/ID>), all read 2026-10-29

- **S1.** Unangst and Jones, NASA TN D-1594 (1963), pp. 1–7, 12, Table I pp. 17–23, Table II pp. 24–30, Figs. 1–7
  pp. 31–36. ID 19630002604. S1, S2 and S4–S10 are US government works, public domain.
- **S2.** Unangst, NASA TM X-136 (1959), pp. 1–5, Table I pp. 14–15, Table II pp. 16–17, Fig. 2 p. 20. ID 19980236841.
- **S3.** Molyneux and Ruddlesden, ARC R&M 2949 (dated 1953, published 1956), ss. 2–7 pp. 1–6, Tables 1–4 pp. 8–11.
  <https://aerade.cranfield.ac.uk/handle/1826.2/3512>. Crown copyright: see "What the project can use".
- **S4.** Doggett, Rainey and Morgan, NASA TM X-79 (1959), pp. 1–5, ID 19660024036. **S5.** Lauten and Nelson, NACA
  TN 3902 (1957), Table I, ID 19930084784; RM L51C08 (1951), ID 19930086625. **S6.** Bursnall, NACA RM L52K14 (1953),
  Summary, ID 19930087443. **S7.** Lundstrom, Lauten and Angle, NACA RM L8I30 (1948), Summary, ID 19930085467.
  **S8.** Lauten and Sylvester, NACA RM L51K28a (1952), Summary, ID 19930087975.
- **S9.** O'Kelly, NACA RM SL54L22 (1954, YF-102 wings), ID 20090023313; RM SL54G29 (1954, F-100 wings), 20090023309.
- **S10.** Martin, NACA TN 4197 (1958), pp. 5–6, eq. 12 p. 13, references pp. 16–17. ID 19930085030.
- **S11.** Molyneux, Ruddlesden and Cutt, ARC R&M 2944 (dated 1951, published 1956), Table 1 p. 8.
  <https://aerade.cranfield.ac.uk/handle/1826.2/3507>
- **S12.** CDPA 1988 Sch. 1 para. 41, Copyright Act 1956 s. 39 (legislation.gov.uk); 17 U.S.C. § 104A; TNA FAQs.
