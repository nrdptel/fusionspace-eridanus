# Which of HPR Sim's body terms carries the Arcas Robin's excess pitching moment

HPR Sim's fins-off Arcas Robin body has a pitching-moment slope (C_mα) about the 70% center 3 to 9 per rad above NASA
TN D-4013's from Mach 0.6 to 1.2 ([#1219](https://github.com/nrdptel/fusionspace-eridanus/issues/1219)). This note
splits it into HPR Sim's body terms on both tunnel models, which differ only in cylinder length, tests which change fits
both, and reads what published tests say of each term, for #1219's per-row body error and
[#1198](https://github.com/nrdptel/fusionspace-eridanus/issues/1198)'s CP rows. It builds on
[transonic-body-normal-force](transonic-body-normal-force.md); **Inference** marks this note's fits, readings and
arithmetic.

**In short: the excess sits in body lift, the viscous crossflow term, not in the boattail or the nose.**

- **Body lift's moment matches the excess on both models.** About the 70% center it adds +3.13 per rad (short model) and
  +7.12 (long); the excess at Mach 0.6 and 0.8 is +3.68 and +7.17. Slender-body theory's terms alone (nose, full
  boattail, lip) land 0.55 and 0.02 per rad above the tunnel: 2.1% and 0.1%.
- **Only body lift scales right with length.** The long model's excess is 1.95 ± 0.2 times the short's. Body lift's
  moment scales by 2.28 (2.17 if its station moves), the boattail's by 1.33, a nose CP shift's by 1.00. A boattail-only
  fit misses the short model by −1.1 per rad and the long one by +0.3 to +1.35: opposite signs.
- **The tunnel's body still carries extra force,** 0.62 (short) and 1.19 per rad (long) above slender-body theory,
  centered at 70% to 75% of length; HPR Sim's body lift sits at 54% and 53%. With the forces, the fits want body lift at
  34% to 42% of HPR Sim's and the boattail at 66% to 74% of theory, or body lift at 60% to 80% moved 0.175 of length
  aft. DATCOM's boattail rule lands 22% to 24% low; Moore's 6° limit leaves the long model +2.0 to +2.4 high.
- **TN D-4014's supersonic moments back the size, not the place.** At Mach 1.5 to 2.96 HPR Sim's force excess implies
  body lift at 23% to 65% of its own, but the excess sits 0.5 to 6.6 calibers from the tip. With no body lift at all,
  +0.4 to +3.0 per rad remain: a second term errs faster than sound.

Confidence: high that body lift carries most of the subsonic excess and the boattail alone can't; low on size against
station (forces read to ±0.2 per rad). The fix moves the rocket's CP aft 0.05 to 0.41 caliber: flattering for #1198.

## 1. HPR Sim's body, term by term

Recomputed by hand from `body_term` and `body_terms` in `crates/hpr-aero/src/model.rs` and the tables in `crossflow.rs`;
it reproduces the parent's 29.6 and 43.3 per rad (Inference). Each part has a potential (slender-body) term at its own
center and body lift, η C_dn (A_plan/A_ref) sin²α, at its planform centroid [10]. Slopes use the tunnel's C_m angles
(−3.1° to 4.1° short, −3.8° to 4.1° long); C_mα is about the 70% center (12.74 and 16.64 calibers from the
tip), nose-up positive, per rad, at Mach 0.6.

| term | short: at (cal) | C_Nα | C_mα | long: at (cal) | C_Nα | C_mα |
|---|--:|--:|--:|--:|--:|--:|
| nose, potential | 2.33 | +1.999 | +20.80 | 2.33 | +1.998 | +28.60 |
| boattail, potential | 17.75 | −1.323 | +6.63 | 23.33 | −1.323 | +8.84 |
| lip, potential | 18.19 | +0.178 | −0.97 | 23.77 | +0.178 | −1.27 |
| body lift, nose | 2.59 | 0.164 | +1.66 | 2.59 | 0.203 | +2.85 |
| body lift, cylinder | 10.78 | 0.852 | +1.67 | 13.57 | 1.499 | +4.61 |
| body lift, boattail and lip | 17.75 | 0.041 | −0.20 | 23.33 | 0.050 | −0.34 |
| HPR Sim, total | | 1.910 | +29.59 | | 2.605 | +43.30 |
| TN D-4013, fins off | | 1.53 | +25.90 | | 2.19 | +35.58 |

Body lift's C_mα changes under 0.2 per rad from Mach 0.6 to 1.2. Tunnel C_N slopes are the fixture's, C_m the parent's
readings of Figs. 7(c), 9(c) (±0.3 per rad); the report's ±0.05 in C_m is 1.8% to 3.4% of them at 4° [1] p. 4.

## 2. Which change fits both models

**The length test (Inference).** The long model has 5.6 calibers more cylinder, so each term's change scales by its own
ratio. The ±0.2 combines the short model's reading with the long one's spread at Mach 0.6 and 0.8 (35.58, 36.73).

| change in HPR Sim | short ΔC_mα | long ΔC_mα | long over short |
|---|--:|--:|--:|
| needed: tunnel less HPR Sim, mean of Mach 0.6 and 0.8 | −3.68 | −7.17 | 1.95 ± 0.2 |
| nose CP 1 caliber aft | −2.00 | −2.00 | 1.00 |
| boattail loses all its potential lift | −6.63 | −8.84 | 1.33 |
| lip removed | +0.97 | +1.27 | wrong sign |
| body lift removed | −3.13 | −7.12 | 2.28 |
| body lift moved aft by 0.1 of length | −1.92 | −4.17 | 2.17 |

**Least-squares fits (Inference)** to the tunnel's C_mα (σ 0.3 per rad), then with its C_Nα (σ 0.2): Mach 0.6 and
0.8 (4 rows), then all rows but the short model's Mach 0.9, 0.95, 1.2 dips (8). HPR Sim scores χ² 1,451 and 3,454.
Best χ² run 2 to 4 times the degrees of freedom, so the one-sigma ranges (0.02 grid) are 1.4 to 2 times too narrow.

| fitted change, moments only | 4 rows: best | χ² | 8 rows: best | χ² |
|---|---|--:|---|--:|
| body lift scaled by g | g = −0.04 | 12.9 | g = −0.04 | 23.0 |
| body lift moved aft by Δξ of length | Δξ = 0.175 | 10.0 | Δξ = 0.175 | 20.1 |
| boattail keeps a share k_b | k_b = 0.28 | 47.6 | k_b = 0.24 | 114.6 |
| nose CP moved aft | +2.7 cal | 142.3 | +2.95 cal | 320.5 |

- **One term:** body lift wins, removed or moved aft to about 71% of length. The boattail fit leaves −1.09 (short) and
  +1.35 per rad (long) at Mach 0.6; the nose's center would sit 0.9 caliber past the nose's end and still miss by 1.2 to
  2.3.
- **Two terms:** on moments alone five pairs reach χ² 7.2 to 9.4 (4 rows), so moments can't separate them. With
  forces: k_b 0.70 to 0.74 with g 0.34 to 0.40 (χ² 11.9, 4 rows) or 0.66 to 0.68 with 0.40 to 0.42 (42.7, 8 rows); or
  g 0.60 to 0.80 with Δξ 0.175 to 0.185 (12.4, 27.1). Boattail with station and nose with body lift score 29.2 to
  119.3.
- **Where the extra force sits.** The tunnel less slender-body theory is +0.62 and +1.19 per rad in force, −0.55 and
  −0.02 in moment: centered at 13.6 and 16.7 calibers (75% and 70% of length, ±0.5), not at 9.78 and 12.58.

**Named models, Mach 0.6 and 0.8 mean (Inference);** tunnel C_mα 25.9, 36.2 (C_Nα 1.47, 2.05). HPR Sim: 29.6, 43.3
(C_Nα +0.44, +0.57). Slender-body only: 26.5, 36.2 (−0.62, −1.19). Moore's 6° boattail (k_b 0.455, no allowance)
with HPR Sim's body lift: 26.0, 38.5 (+1.16, +1.29). Hopkins and DATCOM (§3): 20.3, 27.5 (+0.66, +0.16).

## 3. The boattail: what published tests say

The Arcas boattail turns 15° over 0.78 caliber, to 0.58 of the diameter; slender-body theory gives it −1.323 per rad
at any angle. No public force test of a short, steep boattail alone below Mach 1.2 was found.

- **Moore 1972, from shadowgraphs:** the boundary layer stays attached on a boattail to about 6°, and its displacement
  takes another 1/4° to 1/2° off; without that his lift was 10% worse [3] p. 25. At the Arcas boattail's length, 5.5°
  to 5.75° gives k_b 0.42 to 0.44; 6° with no allowance, 0.455 (Inference).
- **Moore's Fig. 8,** a boattail's normal force over theory's, from Washington's supersonic data and two shells [3] p.
  18, printed p. 38. Read here (±0.05): 0.82 at Mach 0.6, 1.3 at 0.8, 1.7 at 0.9, 0.55 at 1.5, 0.47 at 1.8. The
  supersonic part sits 15% to 20% above HPR Sim's Washington–Pettis share, 0.47 at Mach 1.5 [2]. Its transonic rise
  would add +5.8 (short) and +7.7 per rad (long) from Mach 0.6 to 0.9; the tunnel moves −3.2 and +0.5 (Inference).
- **Hopkins 1951, DATCOM §4.2.1.1's basis:** potential flow to x₀/L = 0.378 + 0.527 x₁/L (x₁: steepest area
  loss), crossflow aft of x₀ only, fitted on 15 bodies of fineness 4.0 to 12.5 [4] pp. 1, 5, eq. (4); [5] p.
  4.2.1.1-1. On the Arcas x₀ is on the cylinder (16.04, 21.09 calibers), so the boattail carries nothing: 22% and 24%
  low (Inference).
- **DATCOM's sample body,** from its lengths a 2.7-caliber boattail averaging 3.6° to 0.66 diameter, measures 0.378 per
  rad on volume^(2/3) against 0.390 with no boattail lift and about 0.17 with full lift [5] pp. 4.2.1.1-2, -3: boattail
  and viscous lift net 5% of the boattail's share (Inference). Hoerner and Borst also find a streamline afterbody's
  negative load smaller than theory's [9] p. 19-4.

So at Mach 0.6 to 0.8 the sources span a boattail share from 0 (DATCOM) through 0.42 to 0.455 (Moore's limit) to 0.82 to
1.3 (his Fig. 8). The Arcas fits put it at 0.66 to 0.74 beside a smaller or later body lift; zero fails (§2).

## 4. Body lift: how large at 2° to 4°, and where

HPR Sim's body lift is Jorgensen's crossflow term at the planform centroid [10], [7]: on the fit angles 1.06 (short) and
1.75 per rad (long), 55% and 67% of each body's C_Nα. It was aimed at high angles [7] p. 26.

- **Perkins and Jorgensen, Mach 1.98,** a 3-caliber ogive on a 10-caliber body, 0° to 20° [6] p. 1. At 5° the
  cylinder carries "considerably greater" lift than potential theory [6] p. 9. A uniform crossflow puts the viscous
  load's centroid too far forward [6] p. 10: the effective crossflow drag starts near zero at the tip and peaks 2 to 3
  diameters behind the nose joint [6] pp. 10, 11. Its sign moves HPR Sim toward the tunnel.
- **Hoerner and Borst:** at small angles the lee-side vortex pair forms at or behind the tail and moves forward with
  angle [9] p. 19-5. Boundary-layer growth adds a linear slope of (0.0001 to 0.0002) ℓ/d per degree on d² [9] pp.
  19-1, 19-7, eq. (8): 0.13 to 0.27 (short) and 0.17 to 0.35 per rad (long), 10% to 25% of HPR Sim's body lift, though
  as a slope at α → 0 (Inference).
- **DATCOM §4.2.1.2:** Allen and Perkins' crossflow is good to ±10% at fineness about 20 or more (the Arcas: 18, 24)
  [5] p. 4.2.1.2-1; at low angles it recommends Hopkins's aft-only crossflow [5] pp. 4.2.1.2-2, 4.2.2.2-1.
- **Robinson 1953,** transonic ogive-cylinder pressures: Allen and Perkins' force is "rather accurate" at the lower
  angles [8] p. 7, loads fair below 12°; lower η and C_dc would agree better at higher angles [8] p. 8.
- **Moore's afterbody lift,** linear, partly from Spring and Gwin's tests [3] p. 18: 0.05 per rad at Mach 0.6, 0.25 at
  0.8, 0.34 at 1.0, 0.23 of the cylinder behind the nose (Figs. 7, 9, pp. 37, 39; plotted to 7.5 calibers of afterbody,
  carried flat to the Arcas's 13.2 and 18.8). HPR Sim lacks it; it would add +0.3 to +1.9 (short) and +0.4 to +2.8 per
  rad (long) to C_mα (Inference), unseen in the long model's flat 35.2 to 36.7.

Dropping body lift on the nose alone, as the near-zero tip load suggests, takes out 1.66 and 2.85 per rad (C_mα 27.9
and 40.5): 45% and 40% of the excess. The rest needs the cylinder's share smaller or further aft.

## 5. TN D-4014's supersonic moments

The committed `arcas-robin-crossflow.json` holds TN D-4014's fins-off slope (every low point) and CP (|α| ≤ 4.5°)
beside HPR Sim's [2]. Moved to the 70% center (Inference; the CP uses 6 of 7 points at Mach 2.96 and long 3.96):

| Mach | 1.5 | 1.8 | 2.3 | 2.96 | 3.96 | 4.63 |
|---|--:|--:|--:|--:|--:|--:|
| short: excess C_mα, per rad | +5.6 | +5.0 | +4.9 | +4.0 | +3.0 | +1.5 |
| short: excess C_Nα | +0.90 | +0.70 | +0.57 | +0.65 | +0.28 | +0.14 |
| short: body-lift share from force | 0.23 | 0.39 | 0.52 | 0.47 | 0.77 | 0.89 |
| long: excess C_mα | | +8.9 | +7.7 | +10.2 | +6.2 | +3.0 |
| long: excess C_Nα | | +0.63 | +0.60 | +0.63 | +0.20 | +0.16 |
| long: body-lift share from force | | 0.61 | 0.64 | 0.65 | 0.88 | 0.90 |

- **The size roughly agrees:** the fixture's 6° to 8° rows at Mach 1.5 to 2.3 need η C_dn 0.47 to 0.79, not 0.90 to
  0.97 [2]: 0.53 to 0.76 of it, overlapping the force-implied 0.23 to 0.65 only in part.
- **The place doesn't.** At Mach 1.5 to 2.96 the excess centers 4.2 to 6.6 (short) and 0.5 to 3.8 calibers (long), ahead
  of body lift's 9.8 and 12.6. Removing all body lift takes 3.4 to 3.6 and 6.6 to 7.2 per rad, leaving +0.4 to +2.2 and
  +0.9 to +3.0: the nose (shock-expansion) or boattail (Washington–Pettis) term errs too, split unbounded.

## What the project can use

All sources are public US government reports except Hoerner and Borst, a copyrighted book cited for facts only. The
project may cite each and commit its own readings with figure and page.

1. **Let #1219's rows carry §1's terms,** so body lift's +3.13 and +7.12 show beside the +3.68 and +7.17 excesses.
2. **Change body lift, after an ADR:** move it to near 71% of length (size 0.60 to 0.80), or scale it to 0.34 to 0.42
   with the boattail at 0.66 to 0.74. The move alone leaves body C_mα −0.7 to +0.8 per rad on both models (now +14%,
   +20%); the force-implied share cuts the supersonic excess 25% to 48%.
3. **Land it with #1198's fin work, not before.** On #1198's CP angles (−2° to 2°) body lift is 0.59 and 0.81 per
   rad. Scaled to 0 to 0.75, it moves the whole rocket's CP aft 0.05 to 0.23 (short) and 0.09 to 0.41 caliber (long) at
   Mach 0.6 to 1.0; moved 0.175 of length aft, 0.13 to 0.30. The parent's swap found the same sign.
4. **Don't adopt DATCOM's x₀ rule or a boattail-only cap here:** 22% to 24% low on both models, or the long model +2.0
   to +2.4 per rad high with forces +1.2 to +1.3 high.
5. **Place the viscous load from pressures:** Perkins and Jorgensen's and Robinson's tables, integrated, would split
   "smaller" from "further aft" at 5° and up.

## Open questions

- **Size or station?** They trade off on fins-off forces read to ±0.2 per rad. Tinling and Allen's NASA TN D-1297
  (1962), an ogive-cylinder's subsonic loads, is the lead; a library record was found, no copy.
- **The short model's dips** at Mach 0.9, 0.95 and 1.2 (+2.3 to +5.3 per rad beyond its Mach 0.6 excess) appear only on
  the model that kept its fin anchors. No term the two models share explains them.
- **The supersonic forward excess** (nose or boattail term) is unbounded; Fig. 8's shells' boattail angles weren't read.

## Sources (all read 2026-11-05)

1. J. C. Ferris, NASA TN D-4013, 1967: p. 4; Figs. 7(c), 9(c). <https://ntrs.nasa.gov/citations/19670020050>
2. C. D. Babb, D. E. Fuller, NASA TN D-4014, 1967, through `validation/fixtures/aero/arcas-robin-crossflow.json`
   (`measured`, `hpr.current`, `high_alpha`, `boattail`). <https://ntrs.nasa.gov/citations/19670020031>
3. F. G. Moore, NWL TR-2796, 1972: printed pp. 18, 25; Figs. 7 to 9, printed pp. 37 to 39 (PDF pp. 45 to 47). Approved
   for public release (cover). <https://archive.org/details/DTIC_AD0754098>
4. E. J. Hopkins, NACA RM A51C14, 1951: pp. 1, 4 to 6; eq. (4); Fig. 5. <https://ntrs.nasa.gov/citations/19930086500>
5. *USAF Stability and Control DATCOM*, 1978: pp. 4.2.1.1-1 to -3, 4.2.1.2-1, -2, 4.2.2.2-1 (PDF pp. 815 to 817, 841,
   842, 912). Approved for public release. <https://archive.org/details/DTIC_ADB072483>
6. E. W. Perkins, L. H. Jorgensen, NACA TN 3716, 1956: pp. 1, 9 to 11. <https://ntrs.nasa.gov/citations/19930084497>
7. L. H. Jorgensen, NASA TR R-474, 1977: p. 26. <https://ntrs.nasa.gov/citations/19770026166>
8. H. L. Robinson, NACA RM L53L28a, 1953: pp. 7, 8. <https://ntrs.nasa.gov/citations/19930088063>
9. S. F. Hoerner, H. V. Borst, *Fluid-Dynamic Lift*, 1985: pp. 19-1, 19-4, 19-5, 19-7. Copyrighted; cited only.
10. `crates/hpr-aero/src/model.rs` (`body_term`, `body_terms`), `crossflow.rs`, the Arcas designs in
    `validation/designs/`.
