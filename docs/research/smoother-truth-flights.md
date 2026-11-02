# Public flights with an independent altitude truth for a smoothed apogee

Re-checked 2026-11-02: 13 claims corrected.

This note asks which public, license-clear flights log a barometer and an accelerometer beside an independent
altitude (radar, optical tracking, a survey-grade or ordinary GNSS fix, a second instrument), and for each: the
rates, the truth's own error, apogee, Mach, and whether the raw samples are published. It serves
[the smoother](../physics/smoother.md) of M7.2, whose 90% intervals are checked only on synthetic flights
([evidence row G33](../evidence.md#g33), issue #981). It builds on [smoothing methods](altimeter-smoothing-methods.md),
[GNSS in boost](gnss-in-boost.md), [public flights](public-flights.md) and [two altimeters](dual-altimeter-spread.md).
**Inference** marks this note's arithmetic and readings of the files. Read 2026-10-29.

**In short.** No public flight pairs a hobby baro-plus-accelerometer log with a radar, optical or RTK truth that could
test a 90% interval. Five flights in MIT repositories pair one with an ordinary GNSS fix: that is the route.
- **Ground tracking can't do it.** Contest optical tracking scatters about 2% (one SD) at best and 3% to 5% typically,
  gives one height and no speed (§2). The one student flight under a radar range used no radar track.
- **The GNSS truths are ordinary u-blox fixes**, 1 Hz (AltOS) or 10 Hz (Featherweight), with no stated vertical error.
  Two receivers on one rocket agree to within 4 m from 16 s through apogee, up to 136 m apart in boost and 170 m in
  early coast. Agreement can't show the error they share, so no truth here has a known vertical error (§4).
- **GNSS reads 3.0% to 6.0% above the standard-atmosphere barometer at apogee on all six logs** (40 to 232 m) (§3).
  The largest implies a day about 15 K warm, the edge of the smoother's day's-air prior (1.65 SD), so a
  barometer-only smoothing of it would sit at its 90% interval's edge (**Inference**, §5).

## 1. What a truth must be for this check

The smoother fuses the GNSS heights it is given ([the smoother](../physics/smoother.md)), so a GNSS truth is
independent only when held out of the input, or taken from a second receiver. On set B its median standard errors
are 84.3 m in height and 1.23 m/s in speed at apogee, so its 90% half-widths are about 139 m and 2.0 m/s
(**Inference**: 1.645 standard errors). A truth whose own error is well under those, a few meters and a few tenths
of a meter per second, can count misses; one at 2% of a 3 km apogee (60 m) can only expose a gross bias.

## 2. Ground tracking: radar, theodolite, optical

- **The rule sets the instrument, not the result.** FAI asks for two trackers at least 300 m apart, ±0.5° in both
  angles, and each station's altitude within 10% of the mean [B1] Annex F p. 50; it bars triangulation from records [B1]
  §4.9.1 p. 20. One station's h = d·tan e gives δh/h = δe/(sin e cos e): ±0.5° at 45° is ±1.75% (**Inference**).
- **Closure doesn't bound error.** Shifting both angles of 173 NARAM-47 flights the same way kept tracks closing up to
  15° off, with altitudes off 11.3% on average (SD 10.0%) [B2] pp. 9 to 12: a sensitivity study, not an error.
- **Against a calibrated altimeter, about 2%.** In 2005, one crew on a 256.5 m GPS-measured baseline tracked 13 flights
  to 479 to 1,100 ft, closures "typically < 1%" [B3] pp. 4, 9. The reference altimeter read −1.20% in a calibration
  chamber [B3] p. 2. Optical minus that corrected altimeter averages −1.2%, SD 2.1%, range −5.4% to +2.2%
  (**Inference**, from [B3] Table V p. 8). A 1996 set of 16 flights gives SD 4.8% [B4] Table 1 p. 3 (**Inference**); 14
  flights in 2002 and 2003 at 6 sites (presented 2004) read about 6% above two baro units, SD about 3% [B5] Table 3
  p. 9, Fig. 7 p. 11 (**Inference**, read by pixel scale), which may be the altimeters' bias, not the track's [B3] p. 2.
- **Contest tracking sights the ejection puff** [B5] p. 3, so it reads low whenever a charge fires before apogee
  (**Inference**). Optical gives one height per flight and no vertical speed.
- **Range-grade tracking is good enough but carries no hobby log.** Sandia's optical mounts gave 0.70 m RMS height
  on surveyed targets [B6] Table 4-3 p. 21; a 1950s radar phototheodolite about ±100 ft [B7] pp. 9, 14.

| Flight | Onboard | Truth | Apogee, Mach | Raw samples, terms |
|---|---|---|---|---|
| NAR study, 2002–2003 [B5] | 2 baro+accel units, 9 others | optical | 630–1,020 ft | plots; member-only |
| STAHR, Esrange, 2024-10-24 [B8] | MS5607; SCA3300, MC3419; MAX-M10S | GNSS bad | 21 km, 2.1 | no; personal use |
| ESA radar trial, planned 2026 [B9] | teams' reflectors | 76–81 GHz FMCW radar | about 1 km | not flown, as found |

STAHR's 21 km is the barometer's own estimate. Its GNSS height was plausible only until launch and its signal was lost
11.5 s after liftoff [B8] pp. 30, 52, 54, so the report compares altitude only with simulation; it suspects
a wrong accelerometer scale (a ±6 g sensor read at ±24 g) in boost [B8] pp. 45 to 46, 60 to 61. STERN asks
for over 3 km, over Mach 1 and stored and sent telemetry [B8] p. 7 Table 2.1, so STERN flights fly instrumented under a
tracked range; none read publishes the track. Spaceport America Cup and EuRoC score with team altimeters; no NAR or
Tripoli radar trial was found.

## 3. GNSS truths: the flights

| Flight (team, date) | Baro + accel | GNSS | Baro apogee | GNSS − baro | Mach ≈ |
|---|---|---|---|---|---|
| Prometheus (Western, SAC, 2022-06-24) | TeleMetrum; TeleMega | each, 1 Hz | 3,904; 3,898 m | +229; +232 m | 0.9 |
| Hyperion-II (Western, 2025-08-21) | TeleMega; Blue Raven | TeleMega, 1 Hz | 2,853 m | +87 m (3.0%) | 0.8 |
| Itzamna (UTRGV, 2026-03-29) | MARV board | Featherweight, 10 Hz | 2,927 m | +91 m (3.1%) | 0.96 |
| Testbed 2 (RIT, 2026-05-16) | Blue Raven; team board | Featherweight, 10 Hz | 1,132 m | +40 m (3.5%) | 0.45–0.48 |
| Juno III (São Paulo, SAC, 2023-06-23) | RRC3, no accel; board | COTS; own GPS | 3,215 m | +154 m (4.8%) | 0.8 |

Rates: AltOS rows at 100 Hz; Blue Raven 50 Hz (Testbed 2 also 500 Hz); MARV baro 47 Hz, IMU 92 Hz; Testbed 2's board
100 Hz; RRC3 20 Hz, Juno III's board about 3 Hz. All five sit in MIT repositories [A1] to [A4]; Prometheus and Juno III
were shared with RocketPy by the teams, with no data license of their own [A1]. Prometheus's two logs are +5.9% and
+6.0%. Apogees are standard-atmosphere heights from each file's pressure (Juno III's RRC3 has a one-sample charge spike
to 191 hPa at 30.45 s, left out; its flight card says 3,213 m [A1]); Mach is each log's peak speed over 343 to 351 m/s
(**Inference**, from the files). HPR Sim reads the AltOS, Blue Raven and Featherweight files ([reading a flight
log](../reading-a-flight-log.md)).
- **Prometheus** [A1] `data/rockets/prometheus/`: TeleMetrum 7,081 rows, TeleMega 6,234 rows with a 3-axis IMU.
  Each logs pressure, acceleration, its own Kalman height and speed, GPS altitude in whole meters, satellites and
  DOPs; 238 GPS updates each, no gap over 2.5 s, lock held through boost. No GPS climb rate.
- **Hyperion-II** [A2]: TeleMega 3,418 rows (78 GPS updates) and a Blue Raven at 0.02 s; air brakes possibly used.
- **Itzamna** [A3]: Featherweight 407 rows, mostly 0.1 or 1 s apart (a few 2 to 3 s gaps in descent), a 3D fix on every
  row through boost. MARV's primary accelerometer clips at 6.0 g in boost; its second peaks there at 13.7 g, unclipped
  (its 15.99 g is two samples after landing); IMU models not named. The folder's Blue Raven file is not this flight's
  ascent, and the GPS summary's date is a day later (**Inference**).
- **Testbed 2** [A4]: Featherweight `VERTV` in whole ft/s, a Blue Raven at 2 ms, the team's own 100 Hz board with
  its fixed-gain Kalman states. The Raven's file date disagrees with the GPS's; their standard-atmosphere peaks
  agree to about 1 ft, so one flight with a wrong Raven clock is likely (**Inference**). Air brakes enabled.
- **Juno III** [A1] `data/rockets/juno3/`: the RRC3 has no accelerometer; the team board logs raw accelerometer
  counts at about 3 Hz, units unstated. Its COTS GNSS dipped 86 ft below the pad at 3.6 s.
- **Cited only.** PSAS Launch 12 (2015) published raw GPS radio samples beside a TeleMetrum, an ARTS2 and an IMU,
  enough for a precise post-processed track, but carries no license [A5]. A dual ZED-F9P payload logged RINEX with no
  barometer or IMU [A6]. No RTK or PPK rocket height, no Student Launch raw GPS, and no Zenodo, figshare or Kaggle
  set pairing all three was found.

## 4. What the GNSS truth is worth

| u-blox receiver | Stated error | Limits |
|---|---|---|
| MAX-M8Q | 2.5 m CEP horizontal; no vertical | 4 g, 50 km, 500 m/s; 10 Hz (GPS+GLONASS) |
| MAX-M10S | 1.5 m CEP horizontal; no vertical | 4 g, 80 km, 500 m/s; 3 Hz max on default GNSS (10 Hz GPS+Galileo) |
| ZED-F9P (no flight here) | 2.0 m vertical PVT; 0.01 m + 1 ppm RTK | 4 g, 80 km, 500 m/s; 20 Hz RTK, GPS only |

Sources: [G1] p. 7, [G2] pp. 4 to 5, [G3] pp. 4 to 5: CEP and PVT are 24 h static medians, RTK over a 1 km baseline,
velocity 0.05 m/s [G1] 50% at 30 m/s. Limits assume the "Airborne 4 g" model; Prometheus's boost reached 9 to 11 g
(**Inference**). AltOS's u-blox boards carry a MAX-7Q, M8 or M10S by version [G6] p. 78 (no MAX-7Q datasheet read), and
the files don't say which. u-blox says high-dynamics models "can result in a higher standard deviation" [G4] p. 22, and
no tracker documents its model.
- **In boost, the fix is not a truth.** A sounding-rocket receiver that held lock through boost scattered 14.5 to
  29.6 m RMS per axis there against a reference receiver, 0.8 to 3.2 m in free flight after [G5] p. 6 Table 1; carrier
  phase, and so RTK, was "almost entirely absent" in a payload's boost [A6] p. 4.
- **Two receivers, one rocket (Inference).** Prometheus's fixes paired by UTC second (239 pairs), each less its pad
  reading (1,390 and 1,394 m): TeleMega minus TeleMetrum +136 m at 4.4 s near burnout (4.7 s), +170 m at 6.3 s, +10 m at
  14.3 s, within ±4 m from 16.3 s to 30.3 s (apogee near 29 s), −3.65 m mean (+0.35 m on raw heights) and 13 m worst
  over the 208 fixes after apogee. From 3 s to about 7 s a 6%-scaled baro height lay between them.
- **Speed.** AltOS logs no GPS climb rate; differencing 1 Hz heights even at ±4 m gives about ±5.7 m/s, wider than
  the 2.0 m/s half-width at apogee (**Inference**). Featherweight's `VERTV` steps 0.3 m/s, its source undocumented.
- **Use (Inference).** After about 15 s, as a truth of unknown error: both receivers tracked the same satellites
  through the same air, so their agreement bounds only the error they don't share. Boost and early coast out.

## 5. The gap is the day's air, and one flight sits at the prior's edge (Inference)

Warm air makes standard-atmosphere heights read short, and every log here has GNSS above the barometer. The smoother
reads a barometer as h − kψ(h), where k = ΔT/T_pad (plus the sensor's gain) and ψ(h) = T_pad ∫ dz/T over the
standard atmosphere (`air_regressor` in `hpr_flightdata`). Without GNSS it holds k at a prior whose variance is a
uniform's over ±(15 K / T_pad + 0.0025), the 15 K being the synthetic flights' range (`AIR_TEMPERATURE_SPREAD_K`).
At Prometheus's pad (1,330 m on the standard atmosphere, T = 279.5 K) that is ±0.0562, SD 0.0324. Its TeleMega gap
of 232 m over ψ = 4,342 m gives k = 0.0534, a day about 14.9 K warm: 95% of the prior's half-range and 1.65 SD, so
with the GNSS held out its apogee truth would sit at the edge of a 90% interval the day's air dominates. A day a few
kelvin warmer would put it outside, on the flattering side for a low apogee. The others give 0.9 to 1.05 SD, Juno
III 1.36 (a day about 12 K warm). The standard atmosphere puts 6 °C at Prometheus's pad, while both its boards read
33 to 34 °C there, 27 K over; board heat aside, a surface that hot fits a layer-mean 15 K (**Inference**). With five
flights all inside, the coverage's one-sided 95% lower bound is only 55% (Clopper–Pearson, 0.05^(1/5)), so these
flights can expose a failure, not certify 90%.

## 6. Ranked shortlist

1. **Prometheus** (MIT repository, data by permission): two baro+accel logs, two receivers, Mach 0.9, 3.9 km. Smooth
   each log with both GNSS held out, test against the pair's mean after 15 s, or fuse one and test the other. Missing:
   GPS speed, a stated error, hardware versions, the day's sounding.
2. **Testbed 2** (MIT): 500 Hz IMU, 10 Hz GPS with `VERTV`, a team Kalman to compare. Missing: a sure Raven clock;
   Mach 0.45 to 0.48; air brakes.
3. **Itzamna** (MIT): Mach 0.96, 10 Hz GPS locked through boost. Missing: IMU models; the primary clips at 6 g.
4. **Hyperion-II** (MIT): TeleMega and Blue Raven. Missing: GPS speed; air-brake state.
5. **Juno III** (MIT repository, data by permission): mostly height: no accelerometer on the RRC3, raw counts at 3 Hz.

## What this suggests

1. **Run G33's check on Prometheus first,** both logs, GNSS held out, apogee and descent heights against the two
   receivers. It needs no new reader, and it tests the 15 K prior at its edge (§5).
2. **Size the day's-air prior on real launch days, not the synthetic range.** One real flight already reached
   15 K; a prior from soundings at launch sites (or the log's pad temperature) would bound a hotter one.
3. **Score a held-out GNSS truth only after about 15 s;** its shared vertical error is unknown (§4).
4. **Say in the docs that optical tracking checks an apogee to about 2% at best,** with no speed: not a coverage test.

## Open questions

- **How warm was Prometheus's day aloft?** A sounding near Spaceport America on 2022-06-24 would say whether its 6.0%
  is all air or part receiver bias, and so how close to 15 K it really was. Not fetched.
- **Can PSAS's raw GPS samples give a precise track?** The only sub-meter truth found, but unlicensed [A5].
- **Is a climb rate here Doppler or differenced, and how late?** Fits on these files can't tell lag from scale.
- **Did the ESA radar trial fly,** or does a STERN team hold its range's radar track? Not found; two papers unread.

## Sources

Repositories are MIT, read at the commits given; NAR reports (member-only), u-blox datasheets and the FAI, Sandia,
STAHR and IMS documents carry no reuse license and are cited only. Cached copies sit under the ignored `refs/`.

- [A1] RocketPy-Team/RocketPy @`a50d07d5`, MIT, `data/rockets/{prometheus,juno3}/`; the teams gave the data by
  permission (`juno3/README.txt`, the Prometheus example notebook). <https://github.com/RocketPy-Team/RocketPy>
- [A2] werocketry/Hyperion-II @`efda132f`, MIT. <https://github.com/werocketry/Hyperion-II>
- [A3] Lmx154/RL-RocketPy-Notebook @`80c9cd6a`, MIT, `data/Lonestar_2026_Itzamina/`.
  <https://github.com/Lmx154/RL-RocketPy-Notebook>
- [A4] RIT-Launch-Initiative/flight-data @`ffa9f44f`, MIT, `2026/Testbed/Testbed2/`.
  <https://github.com/RIT-Launch-Initiative/flight-data>
- [A5] psas/Launch-12, no license, `data/GPS/README.markdown`. <https://github.com/psas/Launch-12>
- [A6] Ghini, Mazzoni, "Doppler-based GNSS variometry for velocity estimation in high-dynamic sounding rocket
  missions", Research Square preprint, doi:10.21203/rs.3.rs-9493522/v1, CC BY 4.0, p. 4.
- [G1] u-blox, MAX-M8 data sheet UBX-15031506 R06, p. 7.
- [G2] u-blox, MAX-M10S data sheet UBX-20035208 R08, p. 4.
- [G3] u-blox, ZED-F9P-04B data sheet UBX-21044850 R05, pp. 4 to 5.
- [G4] u-blox, u-blox 8 / M8 Receiver Description UBX-13003221 R28, pp. 21 to 22.
- [G5] Bull, Diehl, Montenbruck, Markgraf, "Flight performance evaluation of three GPS receivers for sounding rocket
  tracking", NASA NTRS 20020060112, p. 6. <https://ntrs.nasa.gov/citations/20020060112>
- [G6] Altus Metrum, AltOS manual v1.9.22.2, October 2025, p. 78. <https://altusmetrum.org/AltOS/doc/altusmetrum.pdf>
- [B1] FAI Sporting Code Section 4, Volume S, 2025, §4.9.1 p. 20, Annex F p. 50.
  <https://ciam.fai.org/sites/default/files/sc4_vol_s_space_2025_v2.pdf>
- [B2] Bundick, Gaff, "Can You See Me Now? A Study of Optical Tracking Precision", NARAM-50 R&D, 2008.
- [B3] Kidwell, Ash-Poole, "Measuring Absolute Accuracy of Altimeters", NARAM-47 R&D, 2005.
- [B4] Ring, "Altimeters versus Optical Tracking in NAR Competition", NARAM-38 R&D, 1996.
- [B5] Kidwell, Ash-Poole, "A comparison of altimeters and optical tracking", NARAM-46 R&D, 2004.
- [B6] Miller et al., "Gimballed Tracking Mount Pointing Angle Qualification", SAND2021-13913, 2021, Table 4-3.
  <https://www.osti.gov/servlets/purl/1829199>
- [B7] Brunn, Stillwell, NACA RM H55J18, 1956, pp. 9, 14.
  <https://ntrs.nasa.gov/api/citations/19930089147/downloads/19930089147.pdf>
- [B8] Tritsch, Ring, Kerstan, STAHR, STERN 3 Projektschlussbericht V1.01, RWTH Aachen, 2025.
  <https://oa.tib.eu/renate/handle/123456789/19085>
- [B9] IEEE MTT-S, ESA, Czech Rocket Society, IMS2025 student design competition "Radar Tracking Challenge", pp. 1 to 2.
