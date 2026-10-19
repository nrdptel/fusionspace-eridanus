# Where a static port misreads least through Mach 1, by nose shape and ahead of fins

This note asks what published tests give for the surface pressure coefficient Cp along a rocket's body behind
cone, ogive and other noses, and a few calibers ahead of fins, boattails and the base, at Mach 0.8–1.3 and small
angles of attack, and where a port's error is smallest. It extends [altimeter-mach-error](altimeter-mach-error.md)
§1–2 (one 3-caliber ogive-cylinder, TN D-3516) and its Δh ≈ −0.7·M²·Cp·R·T/g (+ reads high; 0.01 in Cp is
48 m at Mach 0.9, 59 m at 1.0, 85 m at 1.2, sea level). **Inference** marks this note's arithmetic. Read 2026-10-19.

**In short.** In one tunnel, ports 7–9 calibers behind a 10° cone's shoulder and clear of the tail read within
±30 m up to the Mach-1 shock's crossing, which no station avoids; above Mach 1.05 no source sizes any station.
- **In free air the shock crosses the whole body tube between about Mach 0.96 and 1.00.** Behind one 7° cone in a
  near-interference-free tunnel it stood 1.0 caliber aft at Mach 0.96 and 3.8 at 0.99, and had left a 7.2-caliber
  cylinder by 1.00 [P1], [M1] Fig. 10. Tunnels with 0.18–0.73% blockage hold it at 1–3 calibers at Mach 1.0 (§1).
- **The crossing is a jump up.** As Mach rises the shock moves aft [N3] p. 9; a port goes from behind it (Cp +0.01
  to +0.05, 60–280 m low: the makers' "Mach dip") to ahead of it: 410–540 m up in the three measured cases. Slowing
  back through Mach 1 it drops as much (§2). This corrects the direction in altimeter-mach-error §2.
- **A cone shoulder reads high longer than an ogive's.** 1.6 calibers behind a 7° cone, Cp is −0.05 to −0.08 from
  Mach 0.98 to 1.3 (+310 to +475 m) [P1]; behind a 3-caliber ogive, −0.01 to −0.04 by Mach 1.2. No data were found
  for von Kármán, parabolic or power-series noses, nor ogives of fineness 4–6 (§2).
- **Ahead of a boattail or the base a port reads high; ahead of a fin, low.** At Mach 0.9, −0.07 at 0.39 caliber
  ahead of a 15° boattail (+340 m), −0.055 at 0.25 ahead of a blunt base (+260 m), +0.07 at 0.04 ahead of a fin
  root (−350 m). Reach: the base's near 1 caliber, the fin's 0.4; the boattail's measured only to 0.45 (§3).
- **Near Mach 1, at booms ahead of fin tips, the effect grew four to six times and flipped sign once the shock
  passed** [G2] Fig. 35: peaks of Cp +0.09 to +0.16 (−500 to −1,300 m) (§3).
- **A ring averages out angle of attack (Inference).** At 5° and Mach 1 a single port reads −45 to +135 m by roll,
  four equal holes +45 m at any roll. The theory's spread matches [PJ] at Mach 1.98; [E1] measured twice it (§4).

## 1. The sources, and why tunnel Mach 1.0 is not free-air Mach 1.0

| Id | Model | Facility, blockage | Mach |
|---|---|---|---|
| [P1] Page 1958 | 7° cone (fineness 4.08), 7.2-caliber cylinder | Ames 14 ft 0.005%; 2×2 ft 0.25% | 0.60–1.40 |
| [M1] Mitchell 1968 | 3-caliber ogive; 10°, 15°, 42° cones; 8 in | Lewis 8×6 ft, 0.73% | 0.4–2.0 |
| [C2] Coe 1962 | 1-caliber ellipsoidal nose, 9.12 in | Ames 14 ft, about 0.23% | 0.83–1.17 |
| [S1] Shrewsbury 1968 | 10° cone, blunt base or boattails, 4 in | Lewis 8×6 ft, 0.18% | 0.56–1.00, 0–8° |
| [S2] Reubush 1976 | Cone-cylinder, circular-arc boattails, 1 in | Langley 16 ft, 0.0027% | 0.40–1.05 |
| [B1] Burley 1986 | Axisymmetric afterbody with swept tails | Langley 16 ft | 0.6, 0.9 |
| [G2] Gracey 1958 | Booms ahead of fin tips, free flight | Rocket flight, free fall | 0.7–1.3 |
| [E1] Estabrooks 1952 | Fineness-12 body cut to 10 for a sting | Langley 8 ft slotted | 0.89–1.13, 0–20° |

Sources for the cells: [P1] pp. 4–5; [M1] p. ii, p. 3; [C2] pp. 5–6; [S1] pp. 2–4; [S2] pp. 2–4; [B1] pp. 2–4;
[G2] pp. 658–659; [E1] pp. 4–5. [C2]'s blockage is this note's arithmetic.

- **Wall interference moves the shock forward.** Page ran the same cone at 0.25% and 0.005% blockage: at Mach 1.0
  the terminal shock sat on the cylinder in the first and behind the model in the second, and Cp on the cone
  differed by up to 14% [P1] pp. 16–18. Couch and Brooks found significant drag errors near Mach 1 at a blockage
  of 0.03%, and more blockage added a positive pressure increment above Mach 0.96 [CB] p. 1.
- **Page's correction** for a slender model in a porous wall put free-air sonic flow at indicated Mach 1.035 in
  the 2×2 ft tunnel and 1.0066 in the 14 ft [P1] p. 17. **Inference:** it gives offsets of 0.03–0.04 for
  TN D-3516 and [S1], whose "Mach 1.00" shocks at 2.4–3.2 calibers (§3) Page's free-air curve places at Mach
  0.98–0.99. The two disagree, and Page says the formula fails for his larger model [P1] p. 18: say 0.96–0.985.

## 2. Behind the nose: cone, ogive, ellipsoid

**The 7° cone [P1] Table I(b), 14 ft tunnel** (0.005% blockage; tabulated to 0.001, ±0.002 random error, p. 6;
Mach 1.15 and up from Table I(a), outside Page's 0.99–1.05 interference band). Calibers behind the shoulder:

| Mach | 0.38 cal | 0.80 cal | 1.63 cal | Δh at 1.63 cal, sea level (Inference) |
|---|---|---|---|---|
| 0.899 | −0.033 | −0.018 | −0.010 | +48 m |
| 0.964 | −0.352 | −0.205 | +0.011 | −60 m |
| 0.980 | −0.328 | −0.225 | −0.055 | +312 m |
| 0.991 | −0.312 | −0.213 | −0.082 | +475 m |
| 1.001 | −0.296 | −0.198 | −0.077 | +456 m |
| 1.200 | −0.149 | −0.097 | −0.046 | +391 m |
| 1.299 | −0.128 | −0.087 | −0.047 | +468 m |

- **Where the shock stands in free air:** about 1.0 caliber at Mach 0.96, 1.5 at 0.97, 2.2 at 0.98, 3.8 at 0.99,
  off the cylinder by 1.00 ([M1] Fig. 10, plotting Page's schlieren; read ±0.3 caliber). The 0.25% model still
  read +0.016 at 1.63 calibers at Mach 0.981, where the 0.005% model read −0.055 [P1] Table I. After the shock,
  1.63 calibers stays at −0.04 to −0.07 to Mach 1.4: the shoulder's expansion. No orifice is farther aft.
- **Ogive against cones in one tunnel [M1] pp. 6–9, Fig. 10.** The 3-caliber ogive's shock stood 0.5–1 caliber
  nearer the shoulder than the cones' up to Mach 1.06, and was the weakest of the four. TN D-3516's ogive keeps
  |Cp| ≤ 0.02 4–6 calibers aft at Mach 0.7, 0.9, 1.2; a closed tunnel gives −0.02 to −0.03 at 0.3–0.9 [C1].
- **A 1-caliber ellipsoid [C2] Fig. 12(b)** (read ±0.03 in Cp): a −0.6 to −0.7 suction peak at the shoulder at
  Mach 0.83–0.92, ±0.02 to ±0.04 from 1.2 to 2.8 calibers aft below Mach 1, then −0.05 to −0.10 there at
  Mach 1.05 (+330 to +650 m).
- **No data:** von Kármán, parabolic and power-series noses meet the cylinder with zero slope, as an ogive does
  (**Inference**: an ogive-like expansion, unmeasured; the von Kármán's slope ends like √(L − x), maybe sharper).
- **One port through Mach 1 (Inference from [P1]):** at 1.63 calibers Cp goes −0.010 (Mach 0.90), +0.011 (0.964,
  behind the shock, 60 m low), −0.082 (0.991, ahead of it): a +540 m jump in 0.03 of Mach, about 0.1 s at 10 g.
  A fineness-12 body had at least two zero-error stations at each Mach, moving with Mach [G2] p. 659.

## 3. Mid-body, and ahead of the base, a boattail and fins

**Mid-body [S1] Fig. 12, p. 6.** 7.3–9.2 calibers behind a 10° cone and 3 or more ahead of the base, at 0°, Cp was
−0.013 at Mach 0.8 (+50 m) and within ±0.005 at tunnel Mach 0.9 and 1.0 (±30 m). At tunnel Mach 1.0 the shock
stood 2.4–3.2 calibers behind the shoulder: −0.04 ahead of it, +0.04 to +0.05 behind it. No source here sizes the
mid-body above Mach 1.05; [S2] calls its own data above 1.0 in error and doubts them above 0.96 [S2] p. 5.

**Ahead of a blunt base [S1] Fig. 12** (cylinder with a 0.41-caliber sting; increment over the level 3–5 calibers
ahead; read ±0.005 by pixel scan).

| Calibers ahead of base | Mach 0.80 | 0.90 | 1.00 | Δh at Mach 0.9 (Inference) |
|---|---|---|---|---|
| 1.0 | −0.012 | −0.009 | −0.008 | +40 m |
| 0.5 | −0.035 | −0.033 | −0.015 | +160 m |
| 0.25 | −0.061 | −0.055 | −0.030 | +260 m |

A second tunnel (about 3% blockage) agrees: the reach shrinks from about 1 caliber at Mach 0.6 to 0.3 at Mach 1.24
[S3] pp. 12–13, Fig. 15(a). Hobby fins run to the aft end, so a port ahead of them is outside it.

**Ahead of a boattail or a diameter-reducing transition** (reads high; read ±0.01–0.015):

| Calibers ahead | Mach 0.8 | 0.9 | 0.94–0.99 | 1.00 ([S2]: 1.01–1.05) | Source |
|---|---|---|---|---|---|
| 0.39, sharp 15° boattail | −0.12 | −0.06 to −0.07 | not read | −0.02 | [S1] Figs. 10, 19, 22 |
| 0.14, the same | −0.18 | −0.14 to −0.16 | not read | −0.05 | [S1] Figs. 10, 22 |
| 0.30, 16° arc boattail | −0.05 to −0.08 | −0.05 to −0.07 | −0.03 to −0.06 | 0 to −0.03 | [S2] Fig. 7 |

That is +450 m at 0.39 caliber, Mach 0.8, and +720 m at 0.14, Mach 0.9 (**Inference**); none measured beyond 0.45.
On a hobby rocket this matters at a reducing transition (a payload wider than the booster); a flare is unsized.

**Ahead of a fin root [B1] Figs. 8–9** (body row 18° from a 5%-thick tail with 42.6° sweep; root thickness t =
0.11 caliber; Cp read ±0.01; the level 0.6 caliber ahead is about −0.005):

| Ahead of root leading edge | Mach 0.9 | Δh at Mach 0.9 (Inference) | Mach 0.6 |
|---|---|---|---|
| 0.44 cal (3.9 t) | −0.008 | +40 m | −0.01 |
| 0.31 cal (2.8 t) | +0.017 | −80 m | 0.00 |
| 0.17 cal (1.5 t) | +0.050 | −240 m | +0.026 |
| 0.04 cal (0.35 t) | +0.074 | −350 m | +0.018 |

- **Toward Mach 1 [G2] Fig. 35, p. 659.** Booms 13.5 and 16.7 tip thicknesses ahead of fin tips on two free-flight
  models read +0.02 of impact pressure near Mach 0.7, peaked at +0.075 (Mach 0.95) and +0.115 (1.17), then fell to
  −0.035 to −0.045 (16.7 t) and about −0.02 (13.5 t) once the shock passed: Cp +0.09 and +0.16, −500 m and
  −1,300 m (**Inference**). Gracey calls the sizes open to question but accepts the curve's shape [G2] p. 658.
- **A square fin edge's bow shock (Inference from [MO] Fig. 7)** stands about 5 thicknesses ahead at Mach 1.2,
  farther toward 1.1; inside it a port reads kilometers low (+0.51 of q behind a Mach 1.2 normal shock); a leading
  edge swept over 34° has none. Fins 0.03–0.06 caliber thick end their subsonic footprint within 0.1–0.6 caliber.

## 4. Angle of attack: one port or a ring

- **Crossflow theory (Inference, potential flow):** the increment is ΔCp = sin²α·(1 − 4·sin²φ), φ from the
  windward line: +sin²α windward, −3·sin²α at the side, zero at φ = 30°. At 5° (sin²α = 0.0076) and Mach 1,
  sea level, a single port reads −45 to +135 m, cycling as the rocket rolls.
- **A ring (Inference):** any N ≥ 3 equal holes average 1 − 4·sin²φ to −1, so a bay in viscous flow reads
  −sin²α at any roll (+45 m at 5°). In orifice flow 4 equal holes still give exactly −sin²α; 3 give −0.6 to
  −1.4·sin²α by roll. No source tests either.
- **Measured.** At 4° and Mach 0.89–1.13 the increment ran from +0.022 windward to −0.025 at the side, independent
  of Mach [E1] Fig. 7(a), pp. 6–7 (read ±0.007): a spread about twice the theory's 0.0195. At Mach 1.98 and 5° the
  spread around an ogive-cylinder was 0.026–0.035 against 0.030 [PJ] Fig. 5; ahead of [S1]'s boattail, 0.02 at 6°.
- **Aircraft practice** pairs fuselage vents at ±35–45° from the bottom against angle of attack and sideslip [G1]
  p. 79; above Mach 1 a nose boom is preferred, because past its shock its error is the tube's own [G1] p. 82.

## 5. Where a port reads least (Inference, from §2–4)

| Zone | Mach 0.9 | Through Mach 1 | Mach 1.1–1.2 |
|---|---|---|---|
| 1–2 cal behind a 3-caliber ogive's shoulder | +70 to +110 m | +360 to +480 m | +120 to +480 m |
| 1.6 cal behind a 7° cone's shoulder | +48 m | −60 m, a +540 m jump, then +310 to +475 m | +380 to +390 m |
| 2.4–6 cal behind the shoulder | +20 to +50 m | −150 to −280 m, then a 410–540 m jump | up to +180 m |
| 7–9 cal behind, ≥ 1.5 cal ahead of fins and transitions | ±25 m | ±30 m, then the jump (unsized) | unmeasured |
| 0.3–0.5 cal ahead of a boattail or reducing transition | +240 to +340 m | 0 to +330 m | unmeasured |
| Within 0.3 cal ahead of a fin root | −80 to −350 m | larger; sign flips as the shock passes | unmeasured |
| Single port at 5°, any station | add −36 to +109 m | −45 to +135 m | −65 to +194 m (1.2) |

- **Nearer a cone's shoulder it is worse:** 0.4–0.8 caliber aft reads −0.2 to −0.35 at Mach 0.96–1.0, +1,200 to
  +1,900 m [P1]. Ogive rows are altimeter-mach-error §2's; above Mach 1.05, 4–6 calibers behind it read +85 to
  +180 m. The 1.5-caliber margin ahead of the tail is this note's (**Inference**): measured reach is 0.4–1.
- **The common rules land in the bad zones.** "1–2 calibers behind the nose" is the forward suction zone; "4–5
  calibers back" ([PF1] in altimeter-mach-error §3) is the band where the shock stands near Mach 1.
- **A short airframe has no measured quiet zone.** 7 calibers behind the shoulder plus 1.5 ahead of 1–2 caliber
  fin roots needs about 9.5–10.5 calibers of tube. A dual-deploy bay between drogue and main often sits there.

## What this suggests

1. **M7.4's "Mach baro error":** key the Cp(calibers aft, Mach) table by nose class (cone; ogive, also used for
   zero-slope noses, flagged unmeasured), add the base, boattail and fin terms of §3 with their reach, and a
   single-port roll term. Place the shock by free-air Mach (0.96–1.00), not tunnel Mach.
2. **A port-placement check:** flag a port 0–7 calibers behind the shoulder in a flight past Mach 0.95, one within
   1.5 calibers ahead of a fin root, boattail or reducing transition, and a single hole. Each flag carries §5's range.
3. **M7.2's readings:** treat the jump as an upward step on boost and a false descent on the coast back through
   Mach 1, which the deploy logic of [altimeter-deploy-logic](altimeter-deploy-logic.md) must ride out.

## Open questions

- **Zero-slope noses** (von Kármán, parabolic, power series) and ogives of fineness 4–6 at Mach 0.9–1.2, at low
  blockage. AEDC's cone-cylinder pressure reports (Chevalier and Robertson) and ARBRL-MR-03161 (a secant ogive,
  cylinder and boattail with taps around the body to 10°) are on DTIC, which was down on 2026-10-19.
- **The jump's size far aft**, and the mid-body above Mach 1.05: every tunnel is suspect there; a free-flight baro
  log through Mach 1 with a known port position would size both.
- **Fins above Mach 0.9 on a body:** [B1] ran Mach 1.2 but plots only 0.6 and 0.9; NASA TP-2078 and TP-2352 may.
- **Boattail reach past 0.45 caliber:** [S2]'s longer model has taps 0.45–0.47 caliber ahead (Table I), unread.

## Sources

All NACA and NASA reports, US government works, cached under the ignored `refs/`; each is on NTRS at
`https://ntrs.nasa.gov/citations/<id>`. Read 2026-10-19.

- [P1] W. A. Page, NACA TN 4233 (1958), 19930085182. [M1] G. A. Mitchell, NASA TM X-1656 (1968), 19680026661.
- [C1] R. I. Cole, NACA RM L52D30 (1952), 19930087003. [C2] C. F. Coe, NASA TM X-646 (1962), 19650023662.
- [CB] L. M. Couch, C. W. Brooks Jr., NASA TN D-7331 (1973), 19740001903. [E1] B. B. Estabrooks, NACA RM L52D21a
  (1952), 19930087194.
- [S1] G. D. Shrewsbury, NASA TM X-1517 (1968), 19680009768. [S2] D. E. Reubush, NASA TN D-8335 (1976), 19770005055.
- [S3] J. M. Cubbage Jr., NACA RM L56C21 (1956), 19930089150. [N3] Thompson, NACA RM L9J27 (1950), 19930086141.
- [B1] J. R. Burley II, J. R. Carlson, W. P. Henderson, NASA TM-87755 (1986), 19860021223.
- [G1] W. Gracey, NASA RP-1046 (1980), ch. VII, printed pages, 19800015804.
- [G2] W. Gracey, NACA Report 1364 (1958), 19930092348. [MO] W. E. Moeckel, NACA TN 1921 (1949), 19930082597.
- [PJ] E. W. Perkins, L. H. Jorgensen, NACA TN 3716 (1956), 19930084497.
