# How far ERA5's low-level temperature and pressure miss at launch sites

**In short: ERA5's air moves HPR Sim's real-flight apogee error by about 0.1%, either way, at the hours rockets fly;
the larger term is the humidity the project leaves out, 0.08–0.65% of apogee and always sim low.** The real-flight
comparisons fly HPR Sim through the ERA5 air of the hour and convert between logged readings and heights through the
same air, so a temperature error enters twice and mostly cancels. The comparison moves by −(1 − |e_D|)/T̄ per kelvin of
ERA5 running warm, −0.14 to −0.25% (e_D is apogee's elasticity to drag), not the 0.35% that
[apogee-bias](apogee-bias.md) §3 gives for the height alone. ERA5 runs slightly cool at a hot afternoon's pad and warm
in a night inversion; its layer mean above the lowest few hundred meters is within about ±0.5 K. **How far to trust
it:** the algebra is checked; the errors come from Iranian and Taklimakan soundings and global statistics, not from a
US launch field, and no source gives the pad-to-3 km layer mean directly. Those layer means are **Inference**.

## How an ERA5 error reaches the comparison

- **What the code does.** `hpr_io::era5` reads pressure levels only (geopotential, temperature, wind), bilinear to
  the site and linear between the two hours around launch; humidity is not read, so the air is dry
  (`crates/hpr-io/src/era5/mod.rs`, its *What it leaves out*). `Barometer` (`crates/hpr-validate/src/real_flight.rs`)
  turns a logged reading, the 1976 standard atmosphere's altitude less the pad's, back into pressure and then into
  the height climbed in the day's ERA5 air (`xtask/src/fixture_flights.rs`); the public flights go the other way,
  HPR Sim's height into a reading, the same algebra to first order. The board's own temperature never enters.
- **The height piece.** Between two pressures the thickness is proportional to the layer-mean virtual temperature
  [5] p. 417, eq. 12.17. An error ΔT̄ makes the converted log height read high by ΔT̄/T̄: 0.33% per kelvin at
  T̄ = 303 K (a desert afternoon), 0.36% at 277 K (a cold morning).
- **The drag piece.** At a given pressure the density is p/(R T), so a warm error thins the air by ΔT/T and the
  simulated apogee rises by |e_D|·ΔT/T. The drag row of apogee-bias's elasticity table gives |e_D| = 0.51 (the
  project's 1,113 m Monte Carlo rocket), 0.31 (Calisto, 3,181 m) and 0.57 (Project R, Mach 2.7).
- **Net (Inference).** (sim − log)/log moves by (1 + |e_D|ε)/(1 + ε) − 1 ≈ −(1 − |e_D|)ε, with ε = ΔT̄/T̄. The
  height wins because |e_D| < 1: **ERA5 too warm makes HPR Sim look low; too cool, high.** Per kelvin, −0.14%
  (|e_D| = 0.57, 303 K) to −0.25% (0.31, 277 K).
- **OpenRocket's column sees only the pad's error in its drag.** It flies the standard atmosphere anchored at ERA5's
  pad temperature and pressure (`xtask/src/fixture_flights.rs`), scored against the same converted log. Its ERA5
  term is about −ΔT̄/T̄ + |e_D|·ΔT_pad/T, and the gap between the codes about |e_D|(ΔT̄ − ΔT_pad)/T, where ΔT̄ is
  weighted as the drag is (**Inference**). Since ERA5's largest errors sit at the pad, the two differ there.

## 1. ERA5's temperature in the lowest 3 km

**Above the lowest few hundred meters ERA5 is nearly unbiased; it scatters about 1–1.8 K per level, and near the
ground it is too cool by day and too warm at night.** Sondes ERA5 assimilated flatter it; the few it never saw read
about 1.5–2.2 times worse at the same levels (**Inference**, from the RMSEs below).

- **ERA5's own fit.** The spread of radiosonde-minus-first-guess temperature near 850 hPa falls from about 1.2 K
  (2000–2014) to about 1.0 K after 2015 [1] §7.2, Fig. 14a (read off the plot). Sondes are assimilated [1] §5.1, so
  this is a forecast fit, not an independent test.
- **North America and Europe, assimilated.** Over 3.7 million soundings from 232 stations, 1980–2018, ERA5's mean
  error is −0.03 K at 10 m, −0.09 K at 850 hPa and −0.06 K at 700 hPa; RMSE 1.85, 0.85 and 0.72 K [2] p. 3218,
  Table 2. Its 0–1 km lapse rate errs by +0.33 K/km on average (mean absolute 1.41 K/km), its 0–3 km by +0.09 K/km
  (0.47 K/km); inversions are sampled least well, and accuracy is lowest over the mountainous western US [2]
  pp. 3219, 3227. No day–night split is given.
- **A desert ERA5 never saw.** At Tazhong in the Taklimakan (1,099 m, moving sand), 158 summer GPS sondes in 2015–16,
  four times a day, none assimilated [3] §2.1–2.2: ERA5's temperature RMSE is 1.83 K at 850 hPa, 1.33 at 800, 1.17
  at 750 and 1.06 K at 700 [4]. Its mean error is about +0.6 K at 850 hPa, near zero at 775 and about −0.3 K at
  700 hPa [3] Fig. 4a (read off the plot). ERA5 is the best of five reanalyses there [3] §3.
- **Arid Iran, by hour and season (assimilated).** Nine stations, 1990–2020, at 00 and 12 UTC (about 03:30 and
  15:30 local); bias is ERA5 less sonde [6] p. 724, eq. 4. In the afternoon, at the six stations with over 1,000
  afternoon soundings each [6] Table 1, 850 hPa (near the ground at the three above 1,000 m) reads +0.06
  to −0.70 K and 700 hPa −0.24 to +0.30 K in every season [6] p. 725, Table 4. Thinly sampled Shiraz (31) and
  Zahedan (70) read −1.71 and −2.58 K at 850 hPa in summer. At night 850 hPa reads +3.07 to +3.16 K at Shiraz
  (1,491 m) and +3.71 to +5.48 K at Isfahan (1,550 m), the shallow inversion ERA5 misses, but −0.89 to −0.94 K at
  Tehran; 700 hPa reads −0.16 to +0.55 K everywhere [6] Table 4. The author calls the 850 hPa seasonal biases not
  climatologically robust at the higher stations, where few soundings reach that level [6] p. 719.
- **The 2 m air over a desert.** At a Rub' al Khali site, July 2022 to December 2023 (a field campaign),
  ERA5's median 2 m bias is +0.2 to +2.9 K at night and −0.3 to −2.5 K by day, by season [7] p. 12, Fig. 2a: the
  daily cycle is too flat at both ends, as apogee-bias's [C8] found for western North America in summer.
- **Layer means (Inference).** Hot-desert afternoon, pad to 1–3 km: bias −0.5 to +0.2 K, a per-flight 1σ of
  0.9–1.35 K (Tazhong's level RMSEs, averaged at a level-to-level correlation of 0.3–1). Cold morning, 0–1 km: the
  aloft −0.16 to +0.55 K, plus a near-ground excess of +0.2 to +3 K over an assumed 0.1–0.3 km that no source sizes.
- **Humidity in the height piece is small.** T_v ≈ T(1 + 0.61q), so 1 g/kg is 0.18 K at 300 K. ERA5's specific
  humidity RMSE at Tazhong is 1.0–1.2 g/kg below 700 hPa [4]; the lowest-500 m mixing-ratio mean error over North
  America and Europe is −0.18 g/kg, on unstable days only [2] Table 2. **Inference:** about 0.2 K random and
  −0.03 K bias, about 0.07% of apogee.

## 2. Pressure and geopotential: an offset at the pad mostly drops out

- **Surface pressure.** The spread of ERA5's first-guess departures from surface-pressure reports falls to about
  0.57 hPa by 2015–2019, against about 0.8 hPa for ERA-Interim [1] Fig. 14d (read off the plot; it includes the
  reports' own error). Against stations, ERA5's surface-pressure RMSE far exceeds its sea-level-pressure RMSE because
  its terrain differs from the stations' heights; correcting for elevation cuts it about tenfold [8] p. 13, Fig. 8.
- **Why it hardly matters here (Inference).** The pad's pressure is read from the profile at the site's own
  elevation, not from ERA5's surface pressure, and heights are differenced from the pad. In a dry-adiabatic column
  from a 1,300 m pad, ±10 m (±0.97 hPa) of offset changes the converted apogee by ±0.011% at 1,000 and 3,000 m. The
  simulated air is about 0.1% denser per hPa too high: −0.04 to −0.07% of apogee through |e_D|.
- **Geopotential.** No published ERA5 bias or RMSE in meters at 850–700 hPa against US sondes was found. At Tazhong
  the height RMSE is 27–33 m at every level [4], and the mean height error is about +2 to +3 m at both 850 and
  700 hPa, so the mean 850–700 hPa thickness error is under 1 m on about 1.7 km [3] Fig. 4c (read off the plot).
  Per sounding, the 1–1.8 K temperature RMSE is 5–7 m of that thickness (**Inference**).
- **A pad below the model's ground** (a playa, a valley floor) sits in extrapolated air: temperature is interpolated
  linearly from the lowest model level to a surface value, then extended by a cubic polynomial, and extrapolating
  into voids "can contaminate the 'real' atmosphere" for fields kept in spectral space or interpolated
  horizontally, as the 0.25° grid is [9]. Example (**Inference**):
  2 K wrong over 200 m under a 1,000 m flight is 0.4 K of layer mean, under 0.1% through the net term.
- **Time.** Linear interpolation between hourly fields misses a sinusoidal 20 K daily cycle by at most
  A·ω²·Δt²/8 = 0.09 K (A = 10 K, ω = 2π/24 h, Δt = 1 h; **Inference**; more near sunrise, where the cycle bends
  harder). Analysis accuracy is not uniform through ERA5's 12-hour windows (09–21 and 21–09 UTC) [9]; the seam at
  21 UTC is 14:00–15:00 in the US Mountain zone. A temperature step there is unsized.

## 3. The apogee term, for two launches

ΔT̄ is ERA5 less the truth over the climb; the net is the comparison's shift, (sim − log)/log.

| launch | ΔT̄ (bias) | T̄ | net shift | sign |
|---|---:|---:|---:|---|
| hot desert, 1.2–1.4 km field, 35 °C, mid-afternoon | −0.5 to +0.2 K | 303 K | −0.05 to +0.11% | mostly sim high |
| the same, one thin station's 850 hPa value as the mean | to −2.6 K | 303 K | up to +0.6% | sim high |
| cold morning, 2 °C, 8–9 am, warm error through the column | +0.1 to +0.55 K | 277 K | −0.02 to −0.14% | sim low |
| the same, +3 K in the lowest 200 m only, 1,000 m apogee | +0.6 K | 277 K | −0.17 to +0.03% | either |

- **The last row (Inference).** The height piece is 3 K × 200/1,000 / 277 K = 0.22%. The air is 1.1% thin where a
  share w of the drag loss happens; w is unmeasured, 0.15–0.4 taken because speed peaks near burnout, low in the
  climb. That gives 0.05–0.25%, so drag can outweigh the height there.
- **Per flight, the scatter is larger than the bias.** A 1σ layer mean of 0.9–1.35 K moves one flight's comparison by
  0.13–0.31% (**Inference**). Over the private collection's 53 scored flights it averages down, but flights at one
  field on one day share their error.
- **Against the collection's +8.73%** ([apogee-bias](apogee-bias.md)): a few hundredths of it at most, on the side of
  the bias by day and against it at dawn.

## 4. Humidity: a larger, one-sided term

**Dry air is too dense, so the simulated apogee runs low; the converted log height is untouched.** ERA5's geopotential
is integrated with virtual temperature, so the height piece already carries the moisture; the density does not. Dry
air is too dense by about 0.61q (**Inference**; saturation pressure by [5] p. 198, eq. 4.B.1):

| air | q | density too high | apogee |
|---|---:|---:|---:|
| 35 °C, 15%, 870 hPa (a desert field) | 6.0 g/kg | 0.37% | −0.11 to −0.21% |
| 30 °C, 70%, 1,000 hPa (a humid southern summer) | 18.6 g/kg | 1.14% | −0.35 to −0.65% |
| 2 °C, 90%, 980 hPa (a cold morning) | 4.0 g/kg | 0.25% | −0.08 to −0.14% |

The reader's own note puts dry air 0.4% too dense at 20 °C and 50% (`crates/hpr-io/src/era5/mod.rs`). ERA5 publishes
relative and specific humidity on the same pressure levels [9]; `hpr_atmos::SoundingProfile` already takes a relative
humidity per level. Reading it would raise HPR Sim's real-flight apogees by 0.08–0.65%, widening the high bias a
little: the honest direction.

## 5. Soundings near launch areas

IGRA 2 headers for 2026 to mid-October, counted by release hour [10]. **ERA5 assimilates these** [1] §5.3, so a
comparison with them is an analysis fit and a lower bound on the error at a pad far from a station.

| station | id | elevation | 2026 soundings | release hours, UTC |
|---|---|---:|---:|---|
| Santa Teresa, NM (near Spaceport America) | USM00072364 | 1,254 m | 569 | 23; 11 to April, 17 from May |
| Birmingham, AL (about 170 km from Huntsville) | USM00072230 | 174 m | 593 | 23 and 11 |
| Reno, NV (about 140 km from Black Rock) | USM00072489 | 1,518 m | 565 | 23 and 11 |
| Lamont, OK (ARM Southern Great Plains) | USM00074646 | 306 m | 489 | 177 at 11, 64 at 05, 70 at 17, 80 at 23 |

Most US releases are at 11 and 23 UTC: pre-dawn, in the deepest inversion, and late afternoon, hours after a midday
launch. Santa Teresa's 17 UTC release (11:00 MDT) since May 2026 is the one near-midday desert series found.

## What this suggests

1. **Replace apogee-bias §3's ERA5 item** with the net term, −(1 − |e_D|)/T̄ per kelvin: about 0.1% either way at
   the hours flown, a 0.13–0.31% per-flight scatter, and not "2 K is 0.7%".
2. **Read ERA5's humidity** in the real-flight comparisons: a bounded, one-sided correction of 0.08–0.65%.
3. **Weather dispersion for [M2.8, honest uncertainty](../ROADMAP.md):** a layer-mean temperature 1σ of about 1 K,
   correlated through the column and larger in the lowest 300 m at dawn, and about 0.6 hPa of surface pressure,
   rather than independent noise per level.
4. **A direct measurement:** ERA5 against Santa Teresa's 17 UTC soundings, May–September 2026, gives the pad-to-3 km
   layer-mean error at a desert field near midday (a lower bound; assimilated).

## Open questions

- ERA5's pad-to-3 km layer-mean temperature error and its vertical correlation at a US launch field, by hour.
- How deep ERA5's warm night layer is at sunrise and how fast it erodes by a 9–11 am launch; the cold rows above
  assume 0.1–0.3 km.
- The share w of a flight's drag loss in its lowest 200 m (one simulation would bound it).
- ERA5's 850–700 hPa geopotential error against US sondes, and a height-corrected surface-pressure error at desert
  stations; any temperature step at the 21 UTC window seam.
- Transonic flights: a warm error lowers the Mach number by ε/2, which moves C_D either way. Unbounded here.

## Sources

Read 2026-10-16; copies in the gitignored `refs/research/era5-surface-temperature/`. Terms: [1], [3], [4], [7] CC BY
4.0; [6] CC BY-NC 4.0; [8] arXiv's; [11] (ERA5 data) CC BY 4.0, which needs the Copernicus attribution on derived
numbers; [5], [9], [10] cited only.

1. Hersbach et al., "The ERA5 global reanalysis", Q. J. R. Meteorol. Soc. 146, 1999–2049 (2020); read as the
   publisher's HTML, cited by section and figure. <https://doi.org/10.1002/qj.3803>
2. Taszarek et al., "Comparison of convective parameters derived from ERA5 and MERRA-2 with rawinsonde data over
   Europe and North America", J. Climate 34, 3211–3237 (2021). <https://doi.org/10.1175/JCLI-D-20-0484.1>
3. Huang et al., "Evaluation of five reanalysis products with radiosonde observations over the Central Taklimakan
   Desert during summer", Earth Space Sci. 8, e2021EA001707 (2021). <https://doi.org/10.1029/2021EA001707>
4. Huang et al., RMSE tables of the reanalyses against the Tazhong sondes, Zenodo (2021), `RMS_error_ERA5.txt`.
   <https://doi.org/10.5281/zenodo.4549921>
5. WMO-No. 8, *Guide to Instruments and Methods of Observation*, Vol. I, 2023 edition: p. 198 eq. 4.B.1, p. 417
   eq. 12.17.
6. Pegahfar, evaluation of ERA5 temperature and humidity against upper-air soundings in Iran, 1990–2020, J. Earth
   Space Phys. 51(3), 719–737 (2025), in Persian with an English summary; Tables 1 (p. 723) and 4 (p. 725).
   <https://doi.org/10.22059/jesphys.2025.400092.1007713>
7. Nelli et al., Remote Sens. 16, 3593 (2024), a Rub' al Khali field campaign. <https://doi.org/10.3390/rs16193593>
8. Jin et al., "WeatherReal", arXiv:2409.09371v1 (2024), pp. 13–15. <https://arxiv.org/abs/2409.09371>
9. ECMWF, "ERA5: data documentation", last modified 2026-09-08 (pressure levels, extrapolation below the surface,
   accuracy and uncertainty). <https://confluence.ecmwf.int/display/CKB/ERA5%3A+data+documentation>
10. NOAA NCEI, Integrated Global Radiosonde Archive 2.2: station list and 2026 data files.
    <https://www.ncei.noaa.gov/data/integrated-global-radiosonde-archive/>
11. Copernicus Climate Data Store, "ERA5 hourly data on pressure levels from 1940 to present".
    <https://doi.org/10.24381/cds.bd0915c6>
