# The lapse rate above a hot launch field, when only the field's temperature is known

**In short: on afternoons above 30 °C at nine western and plains sonde stations, air cools by 11.1 K/km from the
surface to 1 km, 10.2 K/km to 2 km and 9.6 K/km to 3 km (19,351 soundings, 1995–2026, 1σ 1.0, 0.8 and 0.8 K/km).**
The 1976 standard's 6.5 K/km, anchored at the field's temperature and pressure, therefore makes the air 1–3 km above
the field too thin: by 1.45% at 1 km, 2.21% at 2 km and 2.63% at 3 km on average. Thin air predicts apogee high, by
about 0.2–0.5% (**Inference**, from the drag elasticities in [apogee-bias](apogee-bias.md)): the safe side for a
waiver ceiling. The unshifted standard that `hpr sim` flies today errs the other way, 3–8 times as much (air
5.9–7.8% too dense, apogee 1.8–4.0% low), so the anchoring that
[issue #1199](https://github.com/nrdptel/fusionspace-eridanus/issues/1199) asks for leaves a smaller, conservative
error. **How far to trust it:** the lapses and densities are this note's computation from public soundings and hold
on held-out years and stations; the apogee figures are arithmetic on borrowed elasticities. Only two hours are
sampled (about 09:25–10:25 and 15:05–16:25 local solar time), and only to 3 km above the field.

## 1. The soundings and the method

- **Data.** IGRA 2.2's full-record files (dated 2026-11-04) and station list for Santa Teresa (about 140 km from
  Spaceport America), Albuquerque, Midland, Reno (about 140 km from Black Rock) and Denver (ending July 2022), and
  as held-out stations Tucson, Las Vegas (from 2011), Dodge City and Salt Lake City [1].
- **Filter.** Nominal 00 UTC (or 23 UTC) soundings from 1995 on whose surface record is above 30 °C and which reach
  3 km above the station. 99% of release times in the headers fall at 23:00–23:59 UTC, about 15:05 (Reno) to
  16:25 (Dodge City) local solar time (**Inference**, from longitude). A second set takes the 17 and 18 UTC releases
  (449 at 17:xx UTC; station medians 09:25–10:25 local solar time): 468 soundings, 350 of them from 2025–2026.
- **Screening.** 20,119 soundings passed the filter; 768 were left out because their surface record has no height
  (142 of Midland's 160 are from 2022). They are coarse too: a median 4–7 levels in the lowest 3 km, against
  7–9 for those kept. 19,351 remain. Months run February to November, 92% of them May to September.
- **Levels and heights.** IGRA holds standard levels plus significant levels, added where interpolation would not
  reproduce the sounding [2] ¶11, ¶53: a median 7–9 in the lowest 3 km. Heights are integrated from the surface
  with virtual temperature (each station's 5th–95th percentiles within −1.0 to +1.1 m of the reported heights
  below 4 km). Temperature is linear in height between levels, pressure log-linear; 50 m trapezoid layer means.
- **The anchor** is the sounding's surface record, a screen-level reading perhaps some way from the release point
  [2] ¶22. Density is p/(R T) of dry air, as the project flies offline; §4 adds the humidity.

## 2. The lapse on hot afternoons, by station

Mean lapse from the surface, ± 1σ, K/km; 0–3 km percentiles; median mixed-layer depth (§3). This note's computation:

| station | elevation | n | surface | 0–1 km | 0–2 km | 0–3 km | 0–3 km p5–p95 | mixed layer |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| Santa Teresa, NM | 1,254 m | 2,981 | 34.0 °C | 11.27 ± 0.79 | 10.38 ± 0.50 | 9.79 ± 0.58 | 8.68–10.53 | 3,301 m |
| Albuquerque, NM | 1,619 m | 1,920 | 33.1 °C | 11.24 ± 0.83 | 10.36 ± 0.50 | 9.92 ± 0.44 | 9.07–10.54 | 3,638 m |
| Midland, TX | 875 m | 2,901 | 34.3 °C | 11.02 ± 0.89 | 10.00 ± 0.74 | 9.20 ± 0.74 | 7.93–10.25 | 2,665 m |
| Reno, NV | 1,518 m | 1,304 | 32.6 °C | 12.24 ± 1.01 | 10.88 ± 0.55 | 10.19 ± 0.53 | 9.23–10.94 | 3,734 m |
| Denver, CO | 1,611 m | 672 | 32.4 °C | 10.70 ± 0.94 | 10.08 ± 0.56 | 9.74 ± 0.42 | 8.99–10.35 | 3,245 m |
| Tucson, AZ | 751 m | 4,069 | 35.5 °C | 10.95 ± 0.99 | 10.17 ± 0.66 | 9.62 ± 0.69 | 8.25–10.44 | 3,039 m |
| Las Vegas, NV | 697 m | 1,792 | 37.0 °C | 11.58 ± 0.94 | 10.48 ± 0.63 | 9.86 ± 0.68 | 8.42–10.58 | 3,480 m |
| Dodge City, KS | 786 m | 1,902 | 33.7 °C | 10.17 ± 1.10 | 9.13 ± 1.02 | 8.55 ± 0.80 | 7.28–9.90 | 1,704 m |
| Salt Lake City, UT | 1,289 m | 1,810 | 33.7 °C | 11.23 ± 0.82 | 10.13 ± 0.52 | 9.73 ± 0.45 | 8.93–10.39 | 3,417 m |
| all nine | | 19,351 | | 11.13 ± 1.03 | 10.16 ± 0.77 | 9.58 ± 0.76 | 8.07–10.51 | |

- **Steeper than 6.5 K/km nearly always.** The 0–3 km lapse's 5th percentile is 7.3–9.2 K/km at every station.
  Station means are 9.2–10.2 K/km, near the dry adiabat (g/c_p [3]; 9.77 K/km here), except Dodge City's 8.55.
- **Steeper than the dry adiabat over 0–1 km**, because of the surface layer (below). Above it the lapse eases:
  layer means of 8.1–9.5 K/km from 1 to 2 km and 7.4–9.1 K/km from 2 to 3 km, by station.
- **Stable across years.** The pooled 0–3 km lapse is 9.63, 9.55 and 9.58 K/km in 1995–2004, 2005–2014 and
  2015–2026. It is 9.30 K/km at 30–33 °C and 9.96 K/km above 36 °C, 9.94 K/km in May and 9.15 K/km in October.
- **Late morning is gentler.** At 17–18 UTC (468 soundings, surface above 30 °C) the pooled lapse is 9.98, 8.70 and
  8.39 K/km to 1, 2 and 3 km; Santa Teresa's 82 read 10.46, 9.04 and 8.76 K/km.

## 3. The superadiabatic surface layer and the mixed layer

**The lowest 200 m cools at 13–21 K/km on hot afternoons, up to twice the dry adiabat.** A lapse above 9.8 K/km is
superadiabatic, and the lowest layers often become so near the ground in the afternoon [4].

- **Size.** The mean 0–200 m lapse runs from 13.1 K/km (Dodge City) to 21.2 K/km (Reno); it exceeds 9.8 K/km in
  83% (Dodge City) to 98.5% (Las Vegas) of soundings. From the surface to the first level above it (a median 47 to
  125 m up), the mean lapse is 20–34 K/km. At 17–18 UTC the 0–200 m lapse is 14–19 K/km by station.
- **What it does (Inference).** The screen reading is about 2 K warmer than a 6.5 K/km line through the air 200 m
  up, so an anchored column starts about 2 K too warm: most of the 0–1 km density error in §4.
- **The mixed layer.** The parcel method takes the height where virtual potential temperature first regains its
  surface value [2] ¶13. Its median is 2.7–3.7 km at eight stations and 1.7 km at Dodge City; it is above 3 km in
  12% (Dodge City) to 83% (Reno) of afternoons. At 17–18 UTC the median is 1.1–1.4 km (Denver's one: 1.75 km).
- **Its limits.** Keeping the surface level lowers parcel heights (a median 146 m over 505 stations [2] Table 3);
  standard-resolution levels raise them [2] ¶56. These depths are good to a few hundred meters.

## 4. Density at 1 to 3 km: what each construction would fly

**What the code does.** `Ussa76::with_offset(ΔT, P₀)` adds ΔT to the standard's temperature at every height and
integrates pressure from P₀ at sea level; `Ussa76::anchored(h, T, p)` picks ΔT and P₀ to pass through T and p at
height h (`crates/hpr-atmos/src/ussa76.rs`, lines 28–32, 265 and 295). Both keep the lowest layer's 6.5 K/km
[5] Table 4. With no field pressure, #1199 could anchor at the standard's field pressure (B) or offset from
101,325 Pa at sea level (C), whose field pressure is higher on a hot day: a warm column loses pressure slowly.

The profile's density over the sounding's, minus 1, mean ± 1σ (this note's computation, n = 19,351, dry air; negative is
air too thin, apogee predicted high). The last column is the share of soundings with the 0–1 km mean too dense:

| profile flown | field | 1 km | 2 km | 3 km | 0–1 km mean | 0–3 km mean | too dense |
|---|---:|---:|---:|---:|---:|---:|---:|
| A: anchored, field T, p | 0 | −1.45 ± 0.31% | −2.21 ± 0.44% | −2.63 ± 0.65% | −0.94 ± 0.26% | −1.75 ± 0.34% | 0.4% |
| B: anchored, standard field p | −0.08 ± 0.42% | −1.53% | −2.29% | −2.71% | −1.03 ± 0.51% | −1.83 ± 0.55% | 2.5% |
| C: offset, 101,325 Pa at sea level | +1.02 ± 0.41% | −0.45% | −1.21% | −1.64% | +0.06 ± 0.46% | −0.75 ± 0.45% | 53% |
| D: unshifted standard (today) | +9.25 ± 1.13% | +6.74% | +4.97% | +3.54% | +7.76 ± 1.03% | +5.94 ± 0.87% | 100% |
| E: 9.77 K/km from field T and p | 0 | −0.43% | −0.24% | +0.20% | −0.43 ± 0.26% | −0.27 ± 0.35% | 5.0% |

- **The sign.** With a steeper real lapse the air aloft is colder than A's, and at the same height colder air is
  denser even though its pressure falls faster: d ln ρ/dz = −(g/R − Γ)/T, and g/R is 34.2 K/km (**Inference**).
- **Two errors, kept apart.** The unshifted standard's error (D) is the wrong temperature at the field: +9.25% too
  dense there. Anchoring fixes that; what is left (A) is the wrong lapse, growing from 0 at the field to −2.6% at
  3 km. Over 0–1 km, A's error is 12% of D's in size; over 0–3 km, 29%.
- **Field pressure barely matters if anchored.** Real field pressure on these afternoons is 0.08% above the
  standard's at the station's height (1σ 0.42%), so B is within 0.1% of A on average. C is not: its field pressure is
  1.0% high, which cancels the lapse error by luck near the ground and is too dense over 0–1 km in 53% of afternoons.
- **By station** A's 0–3 km mean runs from −1.30% (Dodge City) to −2.11% (Reno). **Late morning** (17–18 UTC,
  n = 468), A's 0–1 and 0–3 km means are −0.86% and −1.20%, D's +6.77% and +5.55%.
- **Humidity pulls the other way.** The project's dry air is denser than the real moist air by a mean 0.30%
  (Las Vegas) to 0.67% (Dodge City) at the field and 0.16–0.27% at 3 km. Counted against moist air, A's means
  become −0.57% (0–1 km) and −1.42% (0–3 km), too dense in 6.2% and 0.5% of soundings; D's become +8.17% and +6.30%.
- **An anchor error moves everything.** At fixed field pressure, 1 K too warm at the field thins the air 0.32% there
  and 0.29%, 0.26%, 0.23% at 1, 2, 3 km (aloft, pressure falls more slowly). §3's surface layer makes that matter.

## 5. What it does to apogee

Apogee moves by −|e_D| times the layer-mean density error up to apogee, |e_D| from apogee-bias's table (0.51 for
the 1,113 m Monte Carlo rocket, 0.31 for Calisto at 3,181 m). **Inference** throughout; drag weights the low climb.

| rocket, layer | D: unshifted | A: anchored, dry | A, moist truth | E: 9.77 K/km, dry |
|---|---:|---:|---:|---:|
| 1,113 m, \|e_D\| = 0.51, 0–1 km | +7.76% → −3.96% | −0.94% → +0.48% | −0.57% → +0.29% | −0.43% → +0.22% |
| 3,181 m, \|e_D\| = 0.31, 0–3 km | +5.94% → −1.84% | −1.75% → +0.54% | −1.42% → +0.44% | −0.27% → +0.08% |
| 3,181 m, weighted to 0–1 km | +7.76% → −2.41% | −0.94% → +0.29% | −0.57% → +0.18% | −0.43% → +0.13% |

- **Which way the standard lapse errs.** Anchored at the field, 6.5 K/km predicts apogee 0.2–0.5% high on hot
  afternoons: not the flattering side for a waiver ceiling, where a low apogee flatters. The unshifted standard
  predicts 1.8–4.0% low, the flattering side that #1199 names.
- **Mach (unbounded).** At 3 km A's air is about 9 K warmer than the real air, so its speed of sound is about 1.7%
  high and its Mach number 1.7% low (**Inference**). Near Mach 1 that moves drag either way; not sized here.
- **Barometric heights ([#1055](https://github.com/nrdptel/fusionspace-eridanus/issues/1055) and
  [#1061](https://github.com/nrdptel/fusionspace-eridanus/issues/1061)).** Converting a logged pressure to height
  through A, with the field's T and p, reads high: by +0.79 ± 0.42% at 1 km, +1.24 ± 0.46% at 2 km and
  +1.67 ± 0.50% at 3 km against the sounding's own heights, and low in only 3.5%, 1.1% and 0.4% of soundings. At
  17–18 UTC: +0.37%, +0.64% and +0.88%. The standard-air error it replaces is about ΔT/T (#1055), near 10% for
  35 °C at a 1,400 m field. A speed or rate taken from those heights inherits the same fraction (**Inference**).

## 6. Would a simple rule beat 6.5 K/km?

**Yes for a best estimate, no for a bound.** Rules were fitted on the five named stations' 1995–2014 soundings
(n = 5,171) and scored on their 2015–2026 soundings, the four held-out stations and the late-morning set. Field T
and p known; 0–3 km mean density error, mean / RMS, dry air (this note's computation):

| rule | held-out years (4,607) | held-out stations (9,573) | 17–18 UTC (468) |
|---|---:|---:|---:|
| 6.5 K/km (A) | −1.84% / 1.86% | −1.67% / 1.71% | −1.20% / 1.25% |
| dry adiabat, 9.77 K/km (E) | −0.36% / 0.47% | −0.20% / 0.43% | +0.30% / 0.48% |
| four layers: 16.56, 9.87, 9.38, 8.51 K/km | −0.10% / 0.32% | +0.06% / 0.38% | +0.56% / 0.67% |

- **The four layers** are the training means for 0–0.2, 0.2–1, 1–2 and 2–3 km. They cut the 0–1 km RMS from 1.04%
  to 0.25% on held-out years and from 0.94% to 0.27% on held-out stations.
- **A mixed-layer depth adds nothing.** 9.77 K/km to a depth D, 6.5 K/km above, fits best with D ≥ 3 km (RMS 0.45%).
- **Both rules lose the bound.** At 17–18 UTC (mixed layer 1.1–1.4 km) the dry adiabat's 0–3 km air is too dense
  in 78% of soundings (97% against moist air), +1.21% at 3 km; on afternoons, against moist air, in half. Near
  6.5 K/km the 0–3 km mean is too dense (moist) in 0.5% and 4.5% at the two hours (0–1 km: 6.2%, 12%); at 8.0 K/km
  in 6.1% and 40%.

## What the project can use

- **IGRA 2.2:** its page states no license or use constraint and warns of jumps from instrument changes [1]. Cached
  under `refs/` (nine stations); derived numbers may be published, as US government data (**Inference**).
- **Cited only:** Seidel et al. [2] (free to read), the AMS glossary [3] and the SPC page [4].

## What this suggests

1. **For #1199, anchor; without a field pressure, anchor at the standard's field pressure (B), not at 101,325 Pa
   (C).** Expected: the 0–1 km density error falls from +7.76% to −1.03% on hot afternoons, conservative in 97.5%
   of them; C would be too dense in 53%. The printed run could say the lapse is the standard's 6.5 K/km.
2. **Keep 6.5 K/km for safety outputs** such as a waiver ceiling: anchored, it errs thin in 99.6% of hot afternoons
   over 0–1 km (dry). Say on the page that it reads apogee 0.2–0.5% high on such days.
3. **Offer the dry adiabat or the four layers only as a best estimate,** for comparisons with flown logs: they
   remove about 75–83% of the RMS error on held-out years and stations, 46–62% at late morning, where they lean dense.
4. **For #1055 and #1061,** a correction through anchored 6.5 K/km would flip the warm-day misses (heights 0.8–1.7%
   high at 1–3 km); the fit could draw the 0–3 km lapse, 9.6 ± 0.8 K/km afternoons, 8.4 ± 0.6 K/km late morning.
5. **For [M2.8, honest uncertainty](../ROADMAP.md):** a lapse draw by layer (1σ 1.0, 0.95 and 1.4 K/km for
   0–1, 1–2 and 2–3 km; up to 1.6 K/km at some stations) and the anchor's error.

## Open questions

- Midday itself, 12:00–14:00 local, falls between the two sampled hours; how fast the mixed layer deepens then.
- The lowest 200 m at full resolution (NWS 1 s data), to bound the interpolation between significant levels.
- Humid hot fields (the Southeast, Gulf coast): Dodge City's gentler lapse hints at a smaller error there.
- Above 3 km, for flights to 6–9 km, where the mixed layer ends and the lapse may fall below 6.5 K/km.
- Which field temperature a flyer has (a forecast, a phone, a nearby station), at 0.32% of field density per kelvin.
- The Mach term near Mach 1, and the sondes' daytime radiation error, neither sized here.

## Sources

Read 2026-11-05. Downloads, scripts and outputs are in the gitignored `refs/research/hot-day-lapse-rate/`.

1. NOAA NCEI, IGRA 2.2: `access/data-por/` files for USM00072364, 72365, 72265, 72489, 72469, 72274, 72388,
   72451 and 72572 (dated 2026-11-04), `doc/igra2-station-list.txt`, `doc/igra2-data-format.txt` (v2.2, 2023-01-19),
   and the product page. <https://www.ncei.noaa.gov/data/integrated-global-radiosonde-archive/>,
   <https://www.ncei.noaa.gov/products/weather-balloon/integrated-global-radiosonde-archive>
2. D. J. Seidel, C. O. Ao, K. Li, "Estimating climatological planetary boundary layer heights from radiosonde
   observations: Comparison of methods and uncertainty analysis", J. Geophys. Res. 115, D16113 (2010): ¶11, ¶13,
   ¶22, ¶53, ¶56, Table 3. <https://doi.org/10.1029/2009JD013680>
3. AMS Glossary of Meteorology, "adiabatic lapse rate", edited 2021-08-16.
   <https://glossary.ametsoc.org/wiki/adiabatic-lapse-rate/>
4. NOAA SPC, "Environmental Temperature Lapse Rates". <https://www.spc.noaa.gov/exper/soundings/help/lapse.html>
5. NOAA, NASA, USAF, *U.S. Standard Atmosphere, 1976*, NOAA-S/T 76-1562, Table 4, as the project's
   [atmosphere page](../physics/atmosphere.md) transcribes it. <https://ntrs.nasa.gov/citations/19770009539>
6. The project's notes [apogee-bias](apogee-bias.md) (elasticity table), [ERA5 at launch
   sites](era5-surface-temperature.md) and [Santa Teresa's midday soundings](era5-santa-teresa-midday.md).
