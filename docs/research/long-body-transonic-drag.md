# Transonic drag of finned bodies of fineness 15 to 20

**In short: no public measurement found gives the drag of a flat-based, thin-finned body of fineness 15–20 from Mach
0.8 to 1.2, so whether that length moves the drag-rise start, the peak Mach or the peak's height is still unmeasured.**
Of the 177 models in NASA's collection of free-flight drag, three are 15 to 20 calibers long, and all three are
boattailed; the collection's longest flat-based body is 12.13 calibers. The three start their main rise by Mach
0.95–0.97 and peak at 1.01–1.17 in the order of their nose shapes, the same order and range as the shorter boattailed
bodies of [transonic-drag-rise](transonic-drag-rise.md). The one flat-based long body found, the Army's 2.75-inch
rocket in coast (20.1 calibers), comes from a fire-control table fitted to flights. It rises from Mach 0.78 and peaks
at 1.28, 1.45 times its Mach 0.78 value, behind a blunt fuze nose and wraparound fins that no HPR rocket shares. What
the measurements show length doing is friction: about +0.010 of C_D per caliber, which lowers the peak's ratio to the
subsonic drag. **Inference:** a flat-based HPR body of fineness 15–20 would peak 1.29 to 1.49 times its Mach 0.9 drag
(1.36–1.46 on the short flat-based bodies). HPR Sim's +38% at Mach 0.9 on Hart's body
([#635](https://github.com/nrdptel/fusionspace-eridanus/issues/635)) would read +30% to +42% on it. How sure: high that
the sources searched hold no such set; medium on the long sets' values, read off plots (±0.003 off the rise, ±0.01 on
it). This moves [M1.14c](../decisions-and-roadmap.md#m1-14c)'s drag gap only by inference. **Inference** marks this
note's own readings and arithmetic.

## 1. What was searched

- **The Langley free-flight collection.** Stoney's TR R-100 lists 177 models flown by rocket or helium gun [S1]
  p. 2, with their geometry [S1] Tables I–II, pp. 4–7. Counted from those tables on 2026-10-29: configurations 36
  (15.00 calibers), 97 (17.20) and 111 (17.78) lie in 15–20; 96 (21.20) and 112 (24.50) are longer. Their base areas
  are 0.52, 0.19, 0.19, 0.19 and 0.19 of the frontal area. Fifteen have a flat base (base area 1.00): 1–8, 22, 78, 92,
  98 and 113–115; four of them (22, 113–115) list no fins and a flared afterbody. The longest is 98, 12.13 calibers,
  which is transonic-drag-rise's S3.
- **Later NTRS reports**, searched by title words on 2026-10-29 (fin-stabilized, fineness ratio, zero-lift drag,
  rocket drag, sounding rocket, the vehicles' names). Henning's 12- and 18-caliber missiles flew Mach 1.4–3.2 only [S9]
  p. 1. Welsh and DeMoraes's four long bodies, TR R-100's 96, 97, 111 and 112, flew Mach 0.95–1.7 [S3] p. 2.
- **Sounding rockets.** None of the three screened gives measured coast drag through Mach 1. The Nike-Cajun's measured
  coast drag lies above Mach 1.8 [S5] Fig. 13, p. 28. The Nike-Apache handbook calls its transonic drag estimated,
  since the stage flies below Mach 1.5 only where the air is too thin to matter [S6] PDF p. 16. The Super Loki dart is
  29.6 calibers long [S7] Table 3.2; its coast table jumps from Mach 0.90 to 1.00, and this note found no basis given
  for it [S7] Table 4.1.
  **Inference:** sounding rockets pass Mach 1 under thrust, so their transonic coast drag is seldom measured.
- **Army rockets.** One report tabulates the 2.75-inch rocket's coast drag from flights [S4] (§3). The Zuni and the
  older 2.75-inch motors were not reached.
- **Tunnels.** None found tests a long finned body through Mach 1. Two cautions for any that turns up: wall
  interference near Mach 1 turns a rapid drag rise gradual and delays divergence by about 0.013 in Mach [S8] p. 1; on a
  finned body free flight rose 0.02 in Mach earlier than on a sting and read 15% higher [S10] p. vii.

## 2. The long sets

| Id | Source; test | Nose | Total l/d; base | Fins |
|---|---|---|---|---|
| P15 | Piland [S2]; TR R-100 config. 36 [S1]; flight | parabolic, 4.50 cal | 15.00; boattail, 0.52 A | 3, 45°, 2.8% |
| P15C, P15S | Piland [S2]; same body and flights | 30° cone, sphere tip | 13.5–15; as P15 | as P15 |
| W17 | Welsh and DeMoraes [S3]; config. 97 [S1] | 8° cone, 7.16 cal | 17.20; cut cone, 0.19 A | 3, swept, 2.8% |
| W18 | Welsh and DeMoraes [S3]; config. 111 [S1] | parabolic, 10.65 cal | 17.78; parabolic, 0.19 A | 3, 45°, 2.8% |
| H20 | Dahlke and Batiuk [S4]; fire-control table, flights | M423 fuze on M151 | 20.1; flat, nozzle | 3 wraparound |
| H24 | same | M261 warhead nose | 24.1; flat, nozzle | 3 wraparound |

- **P15.** A mahogany body, 75 in long and 5 in across, smoothly coated; polished duralumin fins of hexagonal section,
  0.25 in thick [S2] p. 2, on a 9 in chord; the afterbody tapers over 2 calibers at 4.02° [S1] p. 49. It was boosted
  to about Mach 1.1, the sustainer took it to 1.8, and the drag came from the coast, Re 20 to 70 × 10⁶ on length,
  within ±0.01 in C_D and Mach [S2] p. 4.
- **P15C, P15S.** Only the part ahead of station 7.65 in changes [S2] p. 2, so these bodies are 13.5 to 15 calibers
  (**Inference**); Piland lays the cone's extra drag partly to its lower fineness [S2] p. 5.
- **W17, W18.** Wooden bodies, lacquered, 7.5 in across with 3.28 in bases [S3] p. 3; Re 40 to 155 × 10⁶ [S3] p. 2.
- **H20, H24.** The MK66 motor with the M151 warhead and M423 fuze is 55.288 in long, with the M261 66.178 in, on a
  2.75 in diameter [S4] Fig. 1, p. 6, Table 6 p. 13. The three wraparound fins hinge in a recess at the nozzle, with
  10° edge bevels [S4] pp. 4, 15. ARDEC derived the coast drag from flight analysis of the coast phase; the power-on
  curve is built from an estimated base drag, so it is not a measurement [S4] p. 10.
- **What TR R-100 adds.** Its curves are faired through the flight points and may differ slightly from the first
  reports' [S1] p. 6; its friction curves are Van Driest's method [S1] p. 6. Accuracy is ±0.01 in C_D and Mach, and
  sharp changes in the drag rise may be eased by the averaging [S1] pp. 2–3.

## 3. Drag coefficients by Mach number

On the frontal area, zero lift, total drag with base. Read on 2026-10-29 off 300 dpi renders with the grids found by
pixel (**Inference**; ±0.003 off the rise, ±0.01 on it; P15C and P15S ±0.01, dashed curves); H20 and H24 are the
table's own knots, interpolated linearly. "Start" is where the slope first reaches 0.1 per unit Mach.

| Id | Start | Peak (Mach) | 0.8 | 0.9 | 1.0 | 1.1 | 1.2 | Peak / 0.9 | Figure |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---|
| P15 | 0.96–0.97 | 0.363 (1.14–1.17) |  | 0.236 | 0.29 | 0.360 | 0.362 | 1.54 | [S1] p. 49; [S2] Fig. 8 |
| P15C | ≈ 1.00 | 0.418 (1.1–1.2) |  | 0.27 | 0.29 | 0.416 | 0.416 | 1.55 | [S2] Fig. 8, p. 19 |
| P15S | 0.96–0.97 | past 1.2 |  | 0.28 | 0.39 | 0.459 | 0.48 | > 1.7 | [S2] Fig. 8, p. 19 |
| W17 | ≤ 0.950 | 0.249 (1.01–1.04) |  |  | 0.243 | 0.242 | 0.234 | ≥ 1.40 | [S1] p. 110 |
| W18 | ≤ 0.962 | 0.199 (1.06–1.15) |  |  | 0.185 | 0.199 | 0.196 | ≥ 1.24 | [S1] p. 124 |
| H20 | ≤ 0.78 | 1.012 (1.28) | 0.715 | 0.809 | 0.960 | 1.000 | 1.010 | 1.25 | [S4] Table 6, p. 13 |
| H24 | 0.76 | 0.920 (1.22–1.30) | 0.640 | 0.740 | 0.830 | 0.890 | 0.917 | 1.24 | [S4] Table 6, p. 13 |

- **P15.** Piland's own Fig. 8 and TR R-100's fairing agree within 0.003 off the rise and 0.01 on it, Mach 0.88 to
  1.3; the curve starts at 0.88. Friction is 0.20, 85% of the Mach 0.9 drag; the rest, 0.036, grows to 0.166 at the
  peak. 73% of the rise falls between Mach 0.95 and 1.02 (S3 91%, HE 62% in transonic-drag-rise). It is down under
  0.5% at Mach 1.2 and 2–3% at 1.3.
- **P15C, P15S.** On these dashed curves, Start is where the main rise begins. Both read 0.27–0.28 from Mach 0.84 to
  0.94. The sphere tip rises from 0.96–0.97 and is still climbing at Mach 1.25. The 30° cone's main rise is Mach
  1.00–1.08; Piland notes the late rise and leaves it unexplained [S2] p. 5.
- **W17, W18.** The data begin already on the rise (W17 about 0.178 at Mach 0.950, W18 about 0.160 at 0.962), so
  their ratios are lower bounds on that first point. W17 rises steeply and falls about 6% by Mach 1.2; W18 climbs
  gradually to Mach 1.06–1.10. Values here were read with the scans' 0.35°–1.40° tilts taken out.
- **H20, H24.** On the earliest value the ratios are 1.45 (H20, 0.700 at Mach 0.78) and 1.51 (H24, 0.610 at 0.76).
  Only 29% and 17% of their rises fall between Mach 0.95 and 1.02, far more gradual than any NACA set. MICOM found
  H20's coast drag may be high by up to 10% [S4] p. 10. H24's table falls from 0.680 at Mach 0.68 to 0.610 at 0.74
  before it rises; **Inference:** a fitting artifact or a real dip, which the table can't tell apart.

## 4. What length changes, cause by cause

- **Friction raises the subsonic drag: +0.010 per caliber.** Transonic-drag-rise's S1 to S3 share a nose and fins and
  add 3.22 calibers of cylinder; at Mach 0.8 they gain 0.033, 0.010 per caliber. P15's friction, 0.20 over 77.2 frontal
  areas of body and fin surface [S1] Table I, p. 49, is C_f 0.0026; a caliber of cylinder adds 4 frontal areas, so 0.010
  (**Inference**). From Hart's 11.0 calibers to 15–20 that is +0.04 to +0.09 (+0.03 to +0.11 for C_f 0.002–0.003).
- **The transonic increment shows no length trend.** Peak less the Mach 0.9 value: 0.111, 0.098 and 0.103 on S1, S2
  and S3 (8.91 to 12.13 calibers), within the read error; P15, boattailed, gives 0.127 (**Inference**, from the tables).
- **A thicker boundary layer lowers subsonic base drag.** Hoerner's correlation puts base drag as 1/√ of the
  forebody's friction ([cylinder base drag](cylinder-base-drag.md#3-boundary-layer-thickness)). Taking HE's friction,
  0.127 at Mach 0.9 in HPR Sim's terms, to 0.167–0.217 cuts its finned base drag, 0.124, by 0.016–0.029
  (**Inference**). If HPR Sim's 0.127 includes the fins' friction, the body's own is smaller and the cut larger,
  about 0.021–0.036 for a body-only 0.09. That note finds the correlation a poor fit near Mach 1, so this is a rough
  size, not a bound. By Mach 1.1 the
  thick- and thin-layer bases there agree within 0.003.
- **The peak's height relative to subsonic falls with length.** Add the friction to both HE's peak, 0.504, and its
  Mach 0.9 value, 0.346: +0.04 gives 1.41, +0.09 gives 1.36. S3 taken to 15–20 calibers gives 1.34 to 1.29. With the
  base cut taken off the Mach 0.9 value, HE's stays at 1.46–1.49 (**Inference**). So 1.29 to 1.49, likely a little
  below the short bodies'.
- **The peak Mach follows the nose, not the length.** Long boattailed bodies: 8° cone 1.01–1.04 (W17); parabolic
  1.06–1.17 (W18, P15); sphere tip past 1.2. Short boattailed ones: 3:1 cone 1.07–1.08, parabola 1.15–1.22, von
  Kármán level from 1.1 (SC, SP, SV). No flat-based long body with an HPR nose was found, so the flat-based peak at
  Mach 1.04–1.05 (HE, S1, S3) is not shown to hold at 15–20 calibers; the shift is unbounded.
- **The start does not move outside the short bodies' range.** Long bodies start by 0.95–0.97 (P15C near 1.00);
  short ones at 0.85–0.96. H20 and H24 start by 0.76–0.78, with a nose and fins unlike any other set.

## 5. The reach of #635 (Inference)

[#635](https://github.com/nrdptel/fusionspace-eridanus/issues/635) adds Hart's configuration E, HE, a fineness-11.0,
flat-based, thin-finned body, as a measured transonic drag row, with HPR Sim's expected error +19% to +38% from Mach 0.8
to 1.2. No set found here can join it at 15–20 calibers with a flat base. Carried to those lengths by §4:

- **Mach 0.9.** HPR Sim's excess, 0.130 (transonic-drag-rise §5), over a drag 0.04–0.09 higher, reads +30% to +34%
  in place of +38% if the base is unchanged. If the base falls as Hoerner's correlation implies, HPR Sim's base rule,
  which has no length term, keeps its error and the reading rises to +39% to +42%.
- **Mach 1.1.** The excess, 0.143, is mostly the nose, which length doesn't change: +29% becomes +25% to +27%.
- **The peak Mach.** HPR Sim's 1.08 against 1.04–1.05 carries over only if the flat-based peak doesn't move with
  length, which no set here shows.

So HE's row speaks for longer HPR bodies through the friction term, which HPR Sim computes from the wetted area, and
within these sizes; it is not a measurement of them.

## What the project can use

TR R-100, Piland, Welsh and DeMoraes, Royall and Garland, the Nike-Apache handbook, the Super Loki report, Couch and
Brooks, Henning and Jaffe are on NTRS marked public use permitted (checked 2026-10-29). The 2.75-inch report is approved
for public release, distribution unlimited [S4] cover. The project may cache all of them and commit readings with
figure and page. They are cached under `refs/research/long-body-transonic-drag/`, TR R-100 already under `refs/papers/`.
Nothing private was read.

- **P15 as a second whole-body transonic row for M1.14c (a suggestion).** It is 15 calibers long with a smooth
  parabolic nose, three thin fins and a modest boattail, flown free from Mach 0.88 to about 1.7; Piland's Fig. 2(a)
  gives its coordinates. Expected effect: it would show whether HPR Sim's early rise and 1.08–1.10 peak, found on HE,
  hold on a body whose friction is 85% of its subsonic drag; measured here, start 0.96–0.97, peak 0.363 at Mach
  1.14–1.17.
- **The P15, P15C and P15S trio as a nose check on one long body**, as SC, SV and SP are on a short one: their
  differences are the noses' (±0.01).
- **H20 and H24 are not references**: a fire-control fit, wraparound fins in a recess, a fuze nose. They are the only
  flat-based long bodies with coast drag found, and they say a long body can rise early and peak late.

## What this suggests

Length 15–20 doesn't overturn transonic-drag-rise's picture. The measurements tie length to friction, +0.010 per
caliber, and show no length trend in the transonic increment (0.098–0.111 on S1–S3). So the peak's ratio to the
subsonic drag falls a little (**Inference**: 1.29–1.49), and HPR Sim's relative error on a long flat-based rocket keeps
its sign and size: +30% to +42% at Mach 0.9 against +38% on HE. Where the peak sits for a flat-based HPR rocket of 15–20
calibers is not measured: 1.04–1.08 remains an inference from 9- to 12-caliber bodies, and the long boattailed bodies
place it by nose shape anywhere from 1.01 to 1.17.

## Open questions

- A flat-based, thin-finned body of fineness 15–20 with an ogive or von Kármán nose, flown through Mach 0.8–1.2 in
  coast: none found. DTIC's Zuni and older 2.75-inch rocket range tests were not reached.
- Why the 2.75-inch rocket rises from Mach 0.78 and peaks at 1.28: nose, wraparound fins, the recess, or the fit.
- Whether a long body's base drag near Mach 0.9 falls as Hoerner's correlation says (§4).
- Why P15C's rise comes about 0.03–0.05 in Mach after P15's.

## Sources, all read 2026-10-29

- **S1.** Stoney, NASA TR R-100 (1961): pp. 2–3, 6, Tables I–II pp. 4–7, configurations 36, 97, 111 pp. 49, 110,
  124.
  <https://ntrs.nasa.gov/citations/19630004995>
- **S2.** Piland, NACA RM L52F23 (1952): pp. 1–5, Fig. 8 p. 19. <https://ntrs.nasa.gov/citations/19930090417>
- **S3.** Welsh and DeMoraes, NACA RM L51E18 (1951): pp. 1–3. <https://ntrs.nasa.gov/citations/19930086651>
- **S4.** Dahlke and Batiuk, *HYDRA 70 MK66 Aerodynamics and Roll Analysis*, US Army MICOM RD-SS-90-6 (1990), DTIC
  ADA226182: cover, pp. 4, 6, 10, 15, Table 6 p. 13. DTIC's copy refused a script; read from the Internet Archive's
  scan. <https://archive.org/details/DTIC_ADA226182>
- **S5.** Royall and Garland, NACA RM L57D26 (1957): Fig. 13 p. 28. <https://ntrs.nasa.gov/citations/19930089896>
- **S6.** Galloway and Crough, *Nike Apache Performance Handbook*, GSFC X-721-66-568 (1966): PDF p. 16.
  <https://ntrs.nasa.gov/citations/19670015760>
- **S7.** Bollermann and Walker, *Super Loki Dart Meteorological Rocket System*, final report (1968): Tables 3.2 and 4.1
  (PDF pp. 14, 30). <https://ntrs.nasa.gov/citations/19680026183>
- **S8.** Couch and Brooks, NASA TN D-7331 (1973): summary, p. 1. <https://ntrs.nasa.gov/citations/19740001903>
- **S9.** Henning, NASA Memo 12-2-58L (1959): summary, p. 1. <https://ntrs.nasa.gov/citations/19980228138>
- **S10.** Jaffe, JPL TM 33-704 (1975): abstract, p. vii. <https://ntrs.nasa.gov/citations/19750008472>
