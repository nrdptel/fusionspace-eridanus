# The anchored standard lapse above a hot field's mixed layer, to 10 km

**In short: the 1976 standard's 6.5 K/km, anchored at a hot field, keeps the air too thin all the way to 10 km above
it; the error peaks at 3–4 km and eases, but does not change sign.** On the same IGRA 2.2 afternoons as
[hot-day-lapse-rate](hot-day-lapse-rate.md) (19,248 of its 19,351 soundings reach 10 km), the band-mean density
error is −2.66% at 3–4 km, −2.14% at 5–6 km and −1.67% at 8–9 km, too dense in at most 1.3% of afternoons per
band. From the field to apogee it is −2.09% for a 6 km flight and −1.98% for a 9 km one, too dense in 1 of 19,248
(0–6 km, by 0.01%). Apogee is predicted high: about +0.6% to +1.2% on the project's drag elasticities, +1.4% to
+1.5% on this note's toy flights. A barometric height through the same profile reads high, +2.53% (+152 m) at
6 km and +3.06% (+276 m) at 9 km. **So "anchored errs safe" holds for flights to 9–10 km on hot afternoons**
([#1199](https://github.com/nrdptel/fusionspace-eridanus/issues/1199)), and a warm-day height correction through it
overshoots, more so aloft ([#1055](https://github.com/nrdptel/fusionspace-eridanus/issues/1055)). **How far to
trust it:** the densities are computed from public soundings and hold on held-out years and stations; the apogee
figures are arithmetic on borrowed elasticities and invented toy rockets. Two hours of the day, nine stations.

## 1. The soundings and the method

- **The same afternoons.** IGRA 2.2's files for the earlier note's nine stations [1], from its cache [2]: 00 or
  23 UTC releases from 1995 on, surface above 30 °C, a surface height in the record. Of its 19,351, 19,248 reach
  10 km above the field (`a10-summary.txt`); late morning (17–18 UTC), 465 of 468. Primary and held-out stations
  are the earlier note's.
- **Heights** are integrated from the surface with virtual temperature, now to 11.5 km above it. Against IGRA's
  reported heights: median +0.1 m, 5th–95th percentiles −0.9 to +1.1 m at 500 hPa, −1.4 to +1.6 m at 250 hPa
  (`split-rule.txt`). Geopotential, as the standard's layers are; 9 km above these fields, geometric is 15–17 m more.
- **The profile flown** is hpr-atmos's `Ussa76::anchored`, rebuilt in the script: the standard's layers fixed in
  geopotential height above sea level (6.5 K/km below 11 km, isothermal above), all shifted by one offset to pass
  through the field's temperature, pressure scaled to the field's [3], [4]. Variant A takes the sounding's field
  pressure; B the standard's pressure at the field height.
- **Density** is dry, p/(R T), as the project flies; §4 adds humidity. Errors are the profile's density over the
  sounding's, minus 1, on a 50 m grid, trapezoid-averaged over a band or column. Negative is too thin: apogee high.
- **It joins up.** A's 1, 2 and 3 km errors (−1.45%, −2.21%, −2.63%) and 0–1 and 0–3 km means (−0.94%, −1.75%)
  are the earlier note's §4 values to the second decimal.

## 2. The air above 3 km

**Above the mixed layer the real lapse eases to about 6.6 K/km at 4–6 km, then steepens again to 7.1–7.6 K/km at
6–10 km.** The pooled mean is above 6.5 K/km in every band; station means dip to 6.38–6.47 K/km at 4–6 km at five
of nine stations, and single afternoons often do.

| layer above field | 3–4 km | 4–5 km | 5–6 km | 6–7 km | 7–8 km | 8–9 km | 9–10 km |
|---|---:|---:|---:|---:|---:|---:|---:|
| mean lapse, K/km | 7.37 | 6.62 | 6.61 | 7.13 | 7.51 | 7.63 | 7.50 |
| 1σ, K/km | 1.57 | 1.40 | 1.30 | 1.10 | 0.96 | 0.95 | 1.15 |
| afternoons below 6.5 K/km | 27.3% | 44.4% | 44.9% | 26.9% | 13.9% | 10.7% | 15.1% |

- **By station** the band means run 6.38–8.53 K/km (lowest Midland's at 4–5 km; above 6 km, Reno's 9–10 km layer,
  6.85). **Late morning** (n = 465) eases to 6.27 K/km at 5–6 km; its other bands above 3 km are within 0.4 K/km.
- **The mixed layer**, by the parcel method of Seidel et al. [5] as the earlier note applied it, now measured to its
  top rather than cut at 4.5 km: pooled median 3,113 m above the field, 10th–90th percentiles 93–4,453 m. It
  reaches above 4.5 km in 9.1% of afternoons (0.6% at Dodge City, 19.3% at Albuquerque) and above 6 km in at most
  0.5% at any station. Late morning: median 1,234 m.
- **The tropopause is far above 11 km.** IGRA's flagged tropopause (in 14,910 of the 19,248) has a median of
  15.1–15.9 km above sea level by station. The WMO 1957 criterion [6], applied here from 500 hPa up, agrees:
  medians 15.0–15.8 km, 5th percentiles 11.8–12.8 km (`a10-summary.txt`).

## 3. Density error from 3 to 10 km

**A's air is too thin in every band to 10 km; the error is largest just above the mixed layer and shrinks by about
37% by 9 km.** Mean, median and 10th–90th percentiles of the band-mean error, and the share too dense
(n = 19,248, dry air; this note's computation, `a10-summary.txt`):

| band above field | A mean | A median | A p10 to p90 | A too dense | late morning A | D: unshifted |
|---|---:|---:|---:|---:|---:|---:|
| 2–3 km | −2.45% | −2.59% | −2.96 to −1.71% | 0.1% | −1.51% | +4.22% |
| 3–4 km | −2.66% | −2.78% | −3.51 to −1.64% | 0.0% | −1.77% | +2.99% |
| 4–5 km | −2.50% | −2.53% | −3.60 to −1.35% | 0.1% | −1.77% | +2.13% |
| 5–6 km | −2.14% | −2.15% | −3.25 to −1.00% | 0.4% | −1.49% | +1.43% |
| 6–7 km | −1.87% | −1.90% | −2.97 to −0.75% | 1.1% | −1.22% | +0.57% |
| 7–8 km | −1.74% | −1.77% | −2.77 to −0.66% | 1.3% | −1.16% | −0.47% |
| 8–9 km | −1.67% | −1.69% | −2.63 to −0.68% | 1.0% | −1.19% | −1.62% |
| 9–10 km | −1.62% | −1.63% | −2.56 to −0.70% | 1.3% | −1.28% | −2.87% |

- **Why it eases without crossing.** A's air stays warmer than the real air, and more so with height: +9.25 K at
  3 km, +10.35 K at 6 km, +13.12 K at 9 km (means). Warmer air loses pressure more slowly, so A's pressure runs
  high, +0.60%, +1.96% and +3.88% at the same heights. The error shrinks with height wherever the real lapse is
  below Γ_A + (g/R − Γ_A)ΔT/T_A: about 7.6 K/km for a 10 K excess at 255 K, 8.0 for 13 K at 247 K. At 3–4 km the
  real 7.37 sits at it and the error holds; above 4 km the lapse is 6.6–7.6 K/km, below it, so the error shrinks,
  slowly: about 0.1 point per km from 6 to 9 km (**Inference**, from these rows).
- **Where it crosses, if it does.** Counting the first 50 m point above 1 km where a sounding's error reaches zero:
  1.9% of afternoons cross by 6 km, 5.0% by 9 km and 6.9% by 10 km; the median crossing among those is 7,300 m.
  The pooled median profile never crosses (−1.63% at 9 km). Late morning: 8.2% by 6 km, 13.8% by 9 km.
- **By station**, A's 8–9 km band runs −1.34% (Midland) to −2.10% (Denver), too thin at every station in every
  band; held-out years and stations are within 0.2 points of the training years (`split-rule.txt`). B, at the
  standard's field pressure, reads −2.75% at 3–4 km and −1.75% at 8–9 km, too dense in 0.1–1.8% per band.
- **The unshifted standard that `hpr sim` flies today (D) changes sign near 7 km:** +7.76% too dense over 0–1 km,
  −1.62% too thin at 8–9 km. Its 0–9 km mean is still too dense, +2.54%, in every afternoon.

## 4. The mean to apogee, and what it does to apogee

**From the field to a 6 or 9 km apogee, A's air is about 2% too thin, too dense in 1 of 19,248.** Column means,
mean and 10th–90th percentiles, share too dense (`a10-summary.txt`):

| column | A, dry | A p10 to p90 | A too dense | A, moist truth | B, dry | late morning A | D: unshifted |
|---|---:|---:|---:|---:|---:|---:|---:|
| 0–3 km | −1.75% | −2.10 to −1.31% | 0.0% | −1.42% | −1.83% | −1.19% | +5.94% |
| 0–6 km | −2.09% | −2.69 to −1.40% | 0.0% | −1.87% | −2.17% | −1.44% | +4.06% |
| 0–9 km | −1.98% | −2.66 to −1.25% | 0.0% | −1.82% | −2.06% | −1.35% | +2.54% |

- **Humidity fades aloft:** dry air is denser than moist by 0.24%, 0.04% and 0.01% at 3, 6 and 9 km. **By station**,
  A's 0–9 km mean runs −1.47% (Dodge City) to −2.36% (Reno); held-out years −2.01%, stations −1.93%, training −2.05%.

**Apogee.** Apogee moves by −|e_D| times the density error the drag sees, with e_D the drag elasticity (percent of
apogee per 1% more drag). The column mean weights every meter alike, but drag is largest low and fast. So this note
flew three invented single-stage rockets straight up through every 10th sounding (n = 1,925), once in the
sounding's air and once in A's, with an illustrative drag curve peaking at Mach 1.1 (`toy.py`, `toy.txt`):

| toy rocket | apogee, median | peak Mach | its e_D | effective error | column mean | apogee, A over real | Mach part |
|---|---:|---:|---:|---:|---:|---:|---:|
| R3 | 3,065 m | 0.96 | −0.45 | −1.55% | −1.77% | +0.84% | +0.14% |
| R6 | 6,306 m | 1.62 | −0.62 | −2.13% | −2.08% | +1.41% | +0.09% |
| R9 | 8,730 m | 2.44 | −0.81 | −2.16% | −1.98% | +1.45% | −0.29% |

- **The effective error** is the density part of the apogee change over e_D: within 0.2 points of the column mean
  at 6 and 9 km, so the column mean is a fair stand-in there (**Inference**, for these toys).
- **On the project's own elasticities** (apogee-bias: 0.31 for Calisto at 3,181 m, 0.57 for Project R [7], [8]),
  a −2.1% error moves a 6 or 9 km apogee by about +0.6% to +1.2%: about 40–70 m at 6 km, 60–110 m at 9 km
  (**Inference**). The toys' larger e_D gives +1.4% to +1.5%, low in 0.3% of R3's flights and none of the others.
  Project R flew to "nearly 100,000 ft" [8], so 0.31–0.57 brackets a 6–9 km flight only loosely.
- **The Mach term, sized for the toys.** A's warmer air raises the speed of sound about 2.0% at 6 km and 2.8% at
  9 km (**Inference**, from §3's temperatures), so Mach reads low. R3 and R6, long near Mach 1, lost drag by it
  (+0.14%, +0.09%); R9, mostly past the drag peak, gained drag (−0.29%). At most a fifth of the density term.

## 5. Barometric heights above 3 km

**Converting a logged pressure to height through A, with the field's T and p, reads high, and more so with height:
+2.53% at 6 km and +3.06% at 9 km, low in only 8 and 3 of 19,248 afternoons, by at most 32 m.** Against the
sounding's own heights, the earlier note's convention (`a10-summary.txt`):

| true height | A: mean | A: in meters, p10 to p90 | late morning A | standard-air altimeter |
|---|---:|---:|---:|---:|
| 3 km | +1.67% | +30 to +67 m | +0.88% | −6.99% |
| 6 km | +2.53% | +90 to +205 m | +1.50% | −6.21% |
| 9 km | +3.06% | +157 to +383 m | +1.85% | −5.72% |

- **By station**, A's 9 km error runs +2.03% (Dodge City) to +3.87% (Reno). **The standard-air altimeter** (pressure
  altitude at the flight minus that at the field) reads low by 5.7–7.0%, the larger error #1055 is about; anchoring
  trades it for a high reading a quarter to half its size.

## 6. The 11 km layer edge, and what the anchoring does to it

**Anchoring does not move the edge.** `build` adds the one offset to every layer's base temperature; the bases stay
at fixed geopotential heights above sea level, the first at 11 km ([4], lines 91–99 and 312–316). So A turns
isothermal 11 km above sea level, 9.4 km above Albuquerque's 1,619 m field and 10.3 km above Las Vegas's 697 m.

- **Nothing up to 9 km above these fields reaches it.** At 10 km, A is −1.85% against −1.38% for the same profile
  kept at 6.5 K/km past 11 km: thinner still, since the real air keeps cooling to its tropopause near 15–16 km.
- **A 2,000 m field would reach it at 9 km** (**Inference**). The thin side should hold to about the real
  tropopause; above it the real air stops cooling while A's pressure excess grows, so the sign may flip. The crate
  finds a +20 K anchor 14% denser than the standard at 20 km [4], lines 284–288. Not measured here.

## 7. A layered best estimate

**For a best estimate rather than a bound, the earlier note's four layers to 3 km plus fitted lapses above remove
about 73% of A's RMS error to 9 km.** The lapses above 3 km, fitted as the primary stations' 1995–2014 means
(n = 5,116), are 7.42, 6.61, 6.60, 7.17, 7.55, 7.68 and 7.50 K/km, by km from 3 to 10 km. On the 0–9 km column,
mean / RMS / too dense (`split-rule.txt`): held-out years −0.06% / 0.55% / 43.0% against A's −2.01% / 2.08% / 0.0%;
held-out stations +0.01% / 0.55% / 48.0% against −1.93% / 2.01% / 0.0%. It loses the bound, like the dry adiabat.

## What this suggests

1. **For #1199, the safe-side claim extends to 9–10 km on hot afternoons.** Expected: anchored at the field, the
   0–6 and 0–9 km means stay 1.3–2.7% thin (10th–90th percentiles), too dense in 1 of 19,248, apogee 0.6–1.5% high.
   The page could say the anchored profile is checked to 10 km above fields below 1.7 km, and not above 11 km MSL.
2. **For #1055, an anchored 6.5 K/km correction overshoots aloft:** heights +2.5% at 6 km, +3.1% at 9 km. A fit
   could take §7's layers instead, or carry the overshoot as a known high bias by height.
3. **Far above the field, prefer a sounding** (the crate's advice [4]); past about 12 km above sea level is this
   note's reading of "far" (**Inference**), and the note stops short of it.
4. **For [M2.8, honest uncertainty](../ROADMAP.md):** a lapse draw above 3 km, 1σ about 1.0–1.6 K/km by km (§2).

## Open questions

- Above 10 km over the field and near the real tropopause, 15–16 km MSL: where A turns too dense, and by how much.
- The sondes' daytime radiation error aloft: a warm bias would make A thinner than shown (**Inference**); unsized.
- Midday, humid fields and fields above 1.7 km (none sampled); the Mach term and e_D for real 6–9 km flights.

## Sources

Read 2026-11-05. IGRA 2.2 states no use constraint [1] (derived numbers publishable as US government data,
**Inference**); Gou et al. [6] is CC BY 4.0; Seidel [5] and Rogers [8] are cited only. Scripts (`extract10.py`,
`analyze10.py`, `toy.py`, `split_rule.py`) and outputs (`a10-summary.txt`, `a10-1718-summary.txt`, `toy.txt`,
`split-rule.txt`) are in the gitignored `refs/research/hot-day-lapse-above-3km/`.

1. NOAA NCEI, IGRA 2.2: `access/data-por/` files for USM00072364, 72365, 72265, 72489, 72469, 72274, 72388,
   72451 and 72572 (dated 2026-11-04), `doc/igra2-station-list.txt` and `doc/igra2-data-format.txt` (v2.2,
   2023-01-19; LVLTYP2 = 2 marks the tropopause). <https://www.ncei.noaa.gov/data/integrated-global-radiosonde-archive/>
2. The project's note [hot-day-lapse-rate](hot-day-lapse-rate.md) and its cache, `refs/research/hot-day-lapse-rate/`.
3. NOAA, NASA, USAF, *U.S. Standard Atmosphere, 1976*, NOAA-S/T 76-1562, Table 4, as the project's
   [atmosphere page](../physics/atmosphere.md) transcribes it. <https://ntrs.nasa.gov/citations/19770009539>
4. `crates/hpr-atmos/src/ussa76.rs` at commit fe8e7513: layer table, lines 91–99; `anchored`, line 295; the offset
   added to every base, lines 312–316; the doc comment on far-above-field flights, lines 284–288.
5. D. J. Seidel, C. O. Ao, K. Li, J. Geophys. Res. 115, D16113 (2010), ¶13, as read for [2] (the publisher refused
   a fetch for this note). <https://doi.org/10.1029/2009JD013680>
6. Y. Gou et al., "The dilemma in identifying WMO-defined tropopause height using high-resolution radiosondes",
   EGUsphere preprint (2025), p. 4, lines 110–115, restating WMO (1957); CC BY 4.0.
   <https://doi.org/10.5194/egusphere-2024-4198>
7. The project's note [apogee-bias](apogee-bias.md), "How much apogee a cause buys" (elasticity table).
8. C. E. Rogers, "Postflight Analysis of the OuR Project R Rocket Flight", High Power Rocketry (1997), PDF p. 7.
   <https://www.rasaero.com/dloads/OuR%20Project%20R%20Rocket.pdf>
