# Wind reconstructed from a GPS-tracked descent, and how it compares with ERA5

**In short: a rocket under a parachute is a slow dropsonde, and the dropsonde's method carries over: wind is the GPS
ground velocity plus (w/g) times its rate of change, w the descent rate [1, appendix; 2, eq. 4].** Dropsondes measured
this way are good to 0.5–2.0 m/s above 10 m [1, p. 416]. Under a hobby main at 6–8 m/s the lag term is under 0.2 m/s
above about 50 m, so the raw ground velocity is nearly the wind; under a 20–30 m/s drogue it is 0.2–1.8 m/s and must be
corrected (**Inference**, §2). ERA5, the reanalysis the real-flight comparisons fly, misses soundings by 1.4–1.9 m/s
RMS in wind speed through the lowest 3 km, with a bias of no more than ±0.35 m/s [17, Table 3]. A one-minute point wind
also differs by 1.1–1.4 m/s RMS between towers 0.8–1.1 km apart on flat ground [23], an error a descent measured at the
flight's time and place doesn't have. The descent's own large unknown is the canopy: round canopies swing ±10° to ±40°
and glide at up to 45° at slow descent [6, pp. 5-2, 5-3, 5-16], a bias no source bounds for hobby canopies. **How far
to trust it:** the response model and the sonde and ERA5 numbers are primary and agree with each other; every
hobby-rocket number is **Inference** from them, and no published work checks a hobby, CanSat or balloon-payload descent
wind against a tower or sounding. Hock and Franklin [1] were read once, from the paper's text layer; the verifying pass
could not reach it, so claims resting on [1] alone are unrechecked.

This builds on [dispersion-sigmas](dispersion-sigmas.md) §6 (wind sigmas in other tools) and
[era5-surface-temperature](era5-surface-temperature.md) (ERA5's temperature and pressure at launch sites).

## 1. How a falling body follows the wind

- **The model.** Lally and Leviton (1958), given in Hock and Franklin's appendix [1, pp. 419–420]. Quadratic drag acts
  on the velocity relative to the air; the vertical balance gives w² = 2mg/(ρ C_D A). When w is much larger than the
  horizontal slip, u = ẋ − ẍ ż/g, with ż the (negative) vertical velocity. Kepert writes u = s + τ ṡ with τ = w/g,
  about 1.2 s for a dropsonde [2, eq. 4, p. 2]; Li derives the same from the drag equations [3, eqs. 3.5–3.14, pp.
  35–36]. Put in words: the body takes up a change in wind with time constant w/g, and in that time it falls w²/g, the
  lag height.
- **Its assumptions.** A steady fall, small vertical wind and a constant C_D [3, pp. 36–37]. The simplified form
  departs from the full one by at most 0.5 m/s in hurricane eyewall winds [1, p. 420]. Wind comes from the receiver's
  Doppler velocity, not from differenced positions [1, p. 410]; the acceleration from 1 s differences [1, p. 420].
- **What it leaves out.** The canopy's added mass: Knacke gives it for opening [6, p. 5-40], no source read for steady
  horizontal response; it lengthens τ by an unbounded amount. Radiosonde descents skip the correction [5, p. 173].

## 2. The lag at hobby descent rates (Inference, from τ = w/g)

The uncorrected wind error is the lag height times the shear, (w²/g)(dU/dz), and always points toward the wind of the
layer above: the body is late.

| Descent | w (m/s) | τ (s) | Lag height (m) | Error at shear 0.005 / 0.01 / 0.02 s⁻¹ (m/s) |
|---|---|---|---|---|
| Main | 6 | 0.61 | 3.7 | 0.02 / 0.04 / 0.07 |
| Main | 8 | 0.82 | 6.5 | 0.03 / 0.07 / 0.13 |
| Dropsonde | 11 | 1.12 | 12.3 | 0.06 / 0.12 / 0.25 |
| Drogue | 20 | 2.04 | 40.8 | 0.20 / 0.41 / 0.82 |
| Drogue | 30 | 3.06 | 91.8 | 0.46 / 0.92 / 1.84 |

- **Near the ground.** On a logarithmic profile with 8 m/s at 10 m and roughness length 0.03 m, the shear is 0.138 s⁻¹
  at 10 m and 0.028 s⁻¹ at 50 m, so a 6 m/s main errs by 0.51 m/s at 10 m and 0.10 m/s at 50 m. Dropsondes lose the
  last 4–10 m and err by several m/s below 10 m [1, p. 411, p. 416].
- **Total drift barely lags.** For a first-order lag the drift falls short of the wind's by τ times the change in the
  body's velocity over the leg: about 25 m under a drogue (τ = 2.5 s, a 10 m/s change).
- **Noise.** The correction adds τ√2 σ_v/Δt: 0.06–0.09 m/s under a main and 0.27 m/s under a 25 m/s drogue at 1 Hz,
  taking σ_v ≈ 0.075 m/s from a u-blox MAX-M10S's 0.05 m/s, a 50% figure [13, Table 1 and note 2, p. 4].

## 3. What sondes achieve, and what that needs

- **Dropsondes.** Maker's wind accuracy 0.5 m/s, resolution 0.1 m/s [1, Table 2, p. 410]. Zero-wind test: mean error
  0.17 m/s, 90% under 0.30 m/s, about 2 m/s with very poor satellite geometry; against a tracking radar, mean deviation
  0.5 m/s (Saarnimo 1998, as cited) [1, p. 415]. Error budget of the corrected wind: horizontal velocity about 0.5 m/s,
  the fall-speed term under 0.5 m/s, acceleration noise under 0.5 m/s with 1 s averaging; total 0.5–2.0 m/s above 10 m
  [1, p. 416]. A filter removes scales of about 3 s or less [1, p. 416].
- **Rates.** 2 Hz on the 1999 sonde [1, p. 408]; the RD41 sends wind at 4 Hz and falls at about 11 m/s at sea level and
  21 m/s at 12 km [4]. Its canopy is built with a small glide factor so the wind readings hold [4].
- **Radiosondes.** The RS41's and RS92's 0.15 m/s and 2° are the spread of twin soundings, a repeatability, not an
  accuracy [7, Table 1, p. 2487]. GRUAN filters out the pendulum motion to 40 s resolution and states 0.4–1 m/s [8,
  Sect. 9, p. 4484]; an RS41's 55 m line swings with a 15 s period [5, p. 170]. A 9 m line (a 6 s swing) and a 3 m/s
  ascent made winds usable down to 50 m scales, against 300 m before [9, p. 4187]. Twin-sonde tests agree to 0.1–0.2
  m/s [10, p. 5; 11, p. 173], but the sondes share one balloon and one swing, so this measures GPS processing, not the
  error against the true wind (**Inference**). The WMO requirement is 1 m/s per component in the troposphere [11, p.
  175].
- **Descending radiosondes are the closest published case.** RS41 descents, some under a parachute, Europe, autumn
  2019: descent winds minus the ECMWF short forecast scatter by about 2 m/s, as ascents do, with a mean near zero [5,
  p. 173]. They fit the forecast better than ascents (less pendulum), and slightly worse when slower, put down to a
  stronger swing when the parachute works well [5, p. 173]. Parachutes sometimes open late or partly [5, p. 168].
- **Hobby trackers log slowly in descent.** The Featherweight tracker logs 10 samples/s in ascent and 1/s after apogee,
  with horizontal velocity and heading among its fields [14, pp. 16–17]. At 1 Hz the vertical spacing is 6 m under a 6
  m/s main and 25 m under a 25 m/s drogue; the usable resolution is set by the smoothing (§4).
- **The named lead is about aircraft.** MDPI *Atmosphere* 12(4):470 estimates wind from civil aircraft flight data with
  a forward-backward smoother; its accuracy is shown only in simulation, and its run on airline data has no independent
  wind truth [12]. Its smoother is the reusable idea.

## 4. The canopy's own motion: the bias term

- **Glide and oscillation.** Average oscillation is ±10° to ±40° for flat circular canopies, ±10° to ±30° conical, ±10°
  to ±15° hemispherical [6, Table 5-1, p. 5-3]. Unstable canopies oscillate above 25 ft/s (7.6 m/s), glide below 15
  ft/s (4.6 m/s) and mix the two between [6, p. 5-2]; model canopies at 10 ft/s glided at up to 45° without oscillating
  [6, p. 5-16]. Hobby mains at 5–8 m/s sit in or just above the mixed band (8 m/s is 26 ft/s; **Inference**).
- **Size (Inference).** A steady glide at angle γ adds w·tan γ along the canopy's heading: at 45° that equals the
  descent rate, 5–8 m/s, larger than any other term here. Hobby canopies usually turn, so the heading rotates and the
  glide partly averages out; what is left is bounded by no source read.
- **Swing on the shock cord (Inference).** On line length L the period is 2π√(L/g): 2.8 s at 2 m, 3.5 s at 3 m, 4.9 s
  at 6 m. A body swinging by θ moves at up to θ√(gL): 0.95 m/s at 10° on 3 m, 2.7 m/s at 20° on 6 m. The swing is
  zero-mean, but a 1 Hz log aliases a 3–5 s swing. GRUAN's 40 s filter spans about 2.7 periods of a 15 s swing [8, p.
  4484; 5, p. 170]; on that ratio a 4 s swing needs about 11 s of smoothing, about 65 m of height under a 6 m/s main
  and 275 m under a 25 m/s drogue.
- **Drogues and streamers.** τ = w/g assumes drag the same in every direction; a tumbling airframe or a fluttering
  streamer has no response study (unbounded).

## 5. ERA5's wind miss in the lowest 3 km

Taszarek et al. paired 3.7 million soundings from 232 North American and European stations (1980–2018, mostly 00 and 12
UTC) with ERA5 at the same hour and nearest grid point [17, Table 3, p. 3221]. ERA5 wind speed:

| Layer or level | Pearson r | Mean error (m/s) | MAE (m/s) | RMSE (m/s) |
|---|---|---|---|---|
| 10 m | 0.71 | −0.19 | 1.40 | 1.90 |
| 0–1 km mean wind | 0.90 | +0.35 | 1.38 | 1.93 |
| 1–3 km mean wind | 0.97 | −0.15 | 1.00 | 1.42 |
| 850 hPa (about 1.5 km) | 0.94 | −0.28 | 1.37 | 1.91 |
| 700 hPa (about 3 km) | 0.96 | −0.31 | 1.31 | 1.83 |

- **A floor, and speed only.** ERA5 assimilates these soundings [15, §5.3], so a field 50–150 km from a sounding site,
  at an afternoon hour with none launched, misses by more (**Inference**), by an amount no source measures. The table
  scores speed, not direction, which drift also needs. US sondes' winds in the lowest 250 m are themselves contaminated
  by pre-release data and a forced profile [18, pp. 2255, 2259].
- **Terrain sets the near-surface miss.** Flat and offshore towers, hourly: r 0.93–0.96, RMSE 1.06–1.35 m/s. NREL's
  Boulder mast and one other mountain tower: r 0.38–0.47, ERA5 1.7–2.3 m/s slow, RMSE 3.3–3.7 m/s [19, Table 4]. On 291
  European masts the yearly-mean bias is −0.72 m/s on simple terrain, −1.39 on medium and −2.64 on complex [20, Fig. 9,
  p. 5091]; French masts agree, −0.5 m/s flat, to −1.7 m/s in mountains [21, §4.2]. At 245 European 10 m stations
  hourly r is 0.6–0.85 at most sites [22, pp. 4870–4871]. Over 18,000 US stations ERA5 runs fast in the northeast,
  often by over 2 m/s, and slow in most of the west [26, p. 15].
- **Its ensemble spread is no error bar.** The 10-member zonal-wind spread is about 0.4 m/s at 1000 hPa and 0.6 m/s at
  850 hPa in the 2010s (about 0.6 and 1.0 m/s around 1980), global mean, read by eye [15, Fig. 12b]: three to five
  times smaller than the measured speed RMSE, though one is a component and the other a speed. ERA5's model cycle
  assigns radiosonde u and v an error of 1.80 m/s at 1000 and 850 hPa and 1.90 m/s at 700 hPa [16, Table 2.6, p. 42]:
  the error of the observation, not ERA5's own.
- **One grid box and one hour.** ERA5 is hourly on a 0.25° grid [15, §3, §10.1]. Thirteen sonic towers on a flat 3 × 3
  km patch in New Mexico: at 10 m, 1-minute speeds differ 1.1 m/s RMS under 800 m apart and 1.4 m/s beyond 1,100 m,
  falling to 0.4 and 0.65 m/s for 30-minute means; direction 38–47° at 1 minute [23, §3.2]. How much of that survives
  at 1–3 km over a 1–2 minute descent, and the error of a flight between ERA5's hours, are measured by no source read.

## 6. The comparison, and what it means for drift

- **Point against point.** A sonde-quality descent wind (0.5–2.0 m/s) and ERA5 (1.4–1.9 m/s RMS) are of the same size.
  The descent wins on what ERA5 can't have: it is measured at the flight's place and minutes, so the 1.1–1.4 m/s
  one-minute representativeness and the hour-to-hour error drop out. It loses on the canopy glide (§4), which is
  unbounded and can exceed the whole ERA5 miss. The one published descent-against-model check, RS41 against the ECMWF
  forecast, scatters by about 2 m/s, the two errors combined [5, p. 173] (**Inference** on the reading).
- **Drift error from ERA5 (Inference).** Drift error ≈ column-mean wind error × descent time. Dual deploy from 3,000 m:
  drogue at 25 m/s to 450 m (102 s) × 1.42 m/s = 145 m; main at 6 m/s (75 s) × 1.93 m/s = 145 m; 205 m if independent,
  290 m if they add. One main from 1,000 m at 5 m/s (200 s): about 390 m. So 100–400 m, 1σ, speed only; a 20° direction
  error on a 10 m/s wind adds about 3.4 m/s across track.
- **Measured drift misses.** A JAXA sounding-rocket payload (RATS, S-520-31) landed about 1 km from its latest GFS
  forecast track, inside an Earth-GRAM Monte Carlo's 1σ ellipse [24, abstract; full paper not read]. RocketPy's
  Valetudo examples record two landing points 100 m apart [25], bounding nothing. An IREC team's forecast drift study
  on 2,900 sonde tracks is in [dispersion-sigmas](dispersion-sigmas.md) §6.

## What this suggests

- **M7.3's wind fit.** Fit the wind as a first-order lag of the body (τ = w/g, w from the log's own descent rate),
  smoothed over several swing periods (about 10 s), rather than reading ground velocity as wind; apply the correction
  under a drogue, where it is 0.2–1.8 m/s. Use ERA5 as the prior with an error near 1.8–2 m/s, not its ensemble spread
  (**Inference**: ERA5's speed RMSE [17] and the error assigned to sonde winds [16]). Expected: the fit weighs ERA5 by
  its measured miss. For Monte Carlo landing spread, the same miss widens the wind term about threefold.
- **M1.14b's reference and the body-lift drift test.** With ERA5 wind, drift is uncertain by about 100–400 m (1σ); a
  body-lift effect smaller than that needs the descent-fitted wind. Total drift is nearly free of lag (about 25 m under
  a drogue), so such a test stands or falls on canopy glide and the wind profile, not on sensor response.
- **A measurement that would set the US number.** Score ERA5 against the descent winds of the collection's GPS-logged
  flights by height band and publish the aggregate RMSE: the launch-field figure no source gives. Flights with a field
  anemometer or a sounding nearby also bound the glide term near the ground.

## Open questions

- How large is a hobby canopy's glide, and does its turning average it out? A magnetometer would show the heading.
- How far does a rocket swing on its shock cord under a main, and what added mass slows its horizontal response?
- ERA5's direction and vector error by height over US land away from assimilated soundings, and between its hours.
- Not read: RATS's drift and ellipse (full paper), IOM 107's profiler figures, starute and parafoil wind papers.

## Sources

All read 2026-10-18. CC BY, cached under `refs/research/wind-from-descent/`: [5, 7–9, 12, 15, 19–24]. Cite only:
[1–4, 10, 11, 13, 14, 16–18, 26]; [6] is cleared for public release; [25] is MIT.

1. Hock, Franklin, "The NCAR GPS dropwindsonde," *BAMS* 80, 1999, doi:10.1175/1520-0477(1999)080<0407:TNGD>2.0.CO;2.
2. Kepert, dropsonde turbulence, AMS 28th Hurricanes conf., 2008, <https://ams.confex.com/ams/pdfpapers/138590.pdf>.
3. S. Li, turbulence in the hurricane boundary layer, thesis, U. Western Ontario, 2012, hdl:20.500.14721/28045.
4. Vaisala, "Dropsonde RD41" datasheet B211706EN.
5. Ingleby et al., "On the quality of RS41 radiosonde descent data," *AMT* 15, 2022, doi:10.5194/amt-15-165-2022.
6. Knacke, *Parachute Recovery Systems Design Manual*, NWC TP 6575, <https://apps.dtic.mil/sti/pdfs/ADA247666.pdf>.
7. Y. Kawai et al., RS41 and RS92 characteristics, *AMT* 10:2485–2498, 2017, doi:10.5194/amt-10-2485-2017.
8. R. J. Dirksen et al., GRUAN data processing for the RS92, *AMT* 7:4463–4490, 2014, doi:10.5194/amt-7-4463-2014.
9. Faber, Gerding, Köpnick, high-resolution radiosonde winds, *AMT* 16:4183–4193, 2023, doi:10.5194/amt-16-4183-2023.
10. D. Edwards et al., "Overview of Met Office intercomparison of Vaisala RS92 and RS41," Met Office, 2014.
11. J. Nash et al., WMO IOM Report 107 (Yangjiang 2010), Sect. 11.
12. Gao et al., flight data-based wind and air data estimation, *Atmosphere* 12(4):470, 2021, doi:10.3390/atmos12040470.
13. u-blox, MAX-M10S data sheet UBX-20035208 R08, Table 1.
14. Featherweight, GPS tracker manual, 2025-02-20, pp. 16–17.
15. H. Hersbach et al., "The ERA5 global reanalysis," *QJRMS* 146:1999–2049, 2020, doi:10.1002/qj.3803.
16. ECMWF, IFS Documentation Cy41r2, Part I: Observations, 2016, Table 2.6, p. 42.
17. Taszarek et al., ERA5 and MERRA-2 against rawinsondes, *J. Climate* 34, 2021, doi:10.1175/JCLI-D-20-0484.1.
18. Fovell, Gallagher, HRRR v3 boundary layer verification, *Wea. Forecast.* 35, 2020, doi:10.1175/WAF-D-20-0101.1.
19. G. Gualtieri, ERA5 against tall towers, *Energies* 14:4169, 2021, doi:10.3390/en14144169.
20. M. Dörenkämper et al., New European Wind Atlas, Part 2, *GMD* 13:5079–5102, 2020, doi:10.5194/gmd-13-5079-2020.
21. B. Jourdier, reanalyses for wind power over France, *Adv. Sci. Res.* 17:63–77, 2020, doi:10.5194/asr-17-63-2020.
22. Molina et al., ERA5 surface wind against HadISD, *Int. J. Climatol.* 41:4864–4878, 2021, doi:10.1002/joc.7103.
23. De Wekker et al., near-surface wind representativeness, *Atmosphere* 17(7):630, 2026, doi:10.3390/atmos17070630.
24. Nagata, Yamada, Nakao, RATS landing and forecast wind, *Trans. JSASS* 65:244–252, 2022, doi:10.2322/tjsass.65.244.
25. RocketPy (MIT), Monte Carlo and Valetudo notebooks, <https://github.com/RocketPy-Team/RocketPy>.
26. W. Jin et al., "WeatherReal," arXiv:2409.09371, 2024, p. 15.
