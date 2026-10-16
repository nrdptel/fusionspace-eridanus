# Can one flight log separate airframe drag from delivered impulse?

M7.3 fits a drag scale, an impulse scale and mass to a log, and M7.5 fits a correction for drag and impulse across
flights. This note asks which of these a barometric trace, with or without an accelerometer, can tell apart, how
strongly the estimates correlate, and what noise, sample rate and priors that takes. It builds on
[flight-derived C_D](flight-derived-cd.md), which found that apogee-matched C_D absorbs motor and mass errors.
**In short: mass never separates; drag and impulse separate in principle, but model error, not noise, decides how
well.** Only thrust/mass and drag/mass enter the motion, so scaling mass, impulse and drag together leaves every
sample unchanged: mass must come from the scale. In an invented flight (**Inference**, §4) a full baro trace pins a
drag scale and an impulse scale to Cramér–Rao bounds of 0.04% and 0.015%, but they correlate at r = +0.976 whatever
the noise or rate, and a C_D rising as 1 + 0.2M² but fitted as a constant moves the drag estimate 5.4%, over 100
times its bound. NASA's flight-test practice met the same gap with a factor of 5–10 on the bounds [S1] p. 137.
Hobby and student practice splits by phase: drag from the coast, impulse from the boost, never both from apogee. How
far to trust it: the theory and NASA numbers are read from primary sources; the rocket numbers are one invented
flight, an example and not a bound; no hobby or student work reports a correlation for a joint fit.

## 1. What a log can determine at all

- **The equation.** In vertical flight dv/dt = k_I·T(t)/m(t) − k_D·½ρv²C_D·A/m(t) − g, with k_I and k_D
  the impulse and drag scales. They enter only as k_I/m and k_D/m, and gravity enters as the acceleration g,
  not as a weight. Scale every mass, k_I and k_D by one factor and every altitude and accelerometer sample is
  unchanged. This is *structural* non-identifiability, a redundant parameterization; a likelihood interval that
  runs off to infinity for finite data is *practical* [S8] §2.3.1–2.3.2. Mass against the two scales is structural.
- **What weakly breaks it (Inference).** With the propellant mass known and only the dry mass free, the shape of T/m
  over the burn changes a little: the sketch (§4) then gives SDs of about 3% on each of k_D, k_I and dry mass from baro
  alone, with r(k_D, k_I) = +1.000, and 0.9% with an accelerometer. A mass weighed to better than 0.9% beats both, so
  mass belongs on a weighed prior, as [ADR-257][adr257] (calibrated intervals for M7.3's fits) already has it.
- **The texts' framing.** Identifiability belongs to a parameterization and an input. When two inputs move
  together, only a "combined equivalent" effect can be estimated [S1] pp. 86–87. Here the inputs are thrust and drag,
  and the boost-then-coast sequence is the maneuver that keeps them apart.
- **Apogee alone is structural too.** Every (k_D, k_I) on one equal-apogee curve gives the same number; arXiv 2512.22248
  calls its two-knob fit under-determined [S7] §4.1. **Inference** from [apogee-bias](apogee-bias.md)'s elasticities
  and a toy vertical model: 1% of impulse moves apogee as much as 1.1–5.5% of C_D, so a ±5% impulse uncertainty alone
  spreads an apogee-matched C_D by ±5.5–28%.

## 2. How correlation is measured, and how far to trust the bound

- **Definitions.** Each parameter's Cramér–Rao bound is the root of the matching diagonal of the inverse information
  matrix; the correlation is the off-diagonal over the root of its two diagonals [S2] p. 23, [S1] p. 37.
- **Correlation is not accuracy.** A highly correlated pair can sit inside a weakly correlated one's ellipse;
  redefining a reference axis can move a correlation anywhere in −1 to 1; and it misses dependence among three or
  more parameters [S2] pp. 24–26. Report the bounds and the correlation together.
- **The bound is optimistic on flight data.** On 18 PA-30 maneuvers the scatter of the estimates was about 9 times
  the mean bound, and practice applied a factor of 5 to 10 [S1] p. 137; correcting for the residuals' spectrum gave
  3 to 6, plus about 2 more from experience [S1] p. 141. The cause is colored residuals: model error sits in the
  same band as the dynamics [S3] p. 75. In 200 simulated runs with colored noise the uncorrected bounds fell short of
  the scatter by more than 3 times; their correction brought the ratio to 1.25–1.67 [S3] pp. 77–78, Table 3.
- **No |r| > 0.9 rule was found in a primary source.** Klein and Murphy's collinearity check uses
  variance-decomposition proportions above 0.5, and adds prior information as a mixed estimator, trading variance
  for bias [S4] p. 18-6.
- **Bearing on ADR-257 (Inference).** ADR-257's coverage test is the empirical form of that factor. Its second set
  of synthetic flights, from a generator that differs from the fit's model, is where colored residuals show: bounds
  read straight from the information matrix should be expected to under-cover there.

## 3. What professional reconstruction and hobby practice estimate, and from what

- **Ares I-X (2009).** The best estimated trajectory is an iterated extended Kalman filter driven by IMU data, with
  radar and air-data vanes as observations and radar biases as solved-for states [S5] pp. 2, 12. Thrust was not
  fitted from the trajectory: it was rebuilt from chamber pressure [S5] p. 15.
- **NACA rocket-boosted drag models.** Drag came only from the decelerating flight after burnout, by Doppler radar
  and a telemetered longitudinal accelerometer, with radiosonde air data [S9] pp. 113–114.
- **Planetary entry, the same problem in another product.** An accelerometer measures force per mass, so density and the
  force coefficient appear only as a product (**Inference**); without pressure sensors, reconstructions assume the
  aerodynamics and solve for the atmosphere [S6] p. 2. With heat-shield pressures, Mars 2020's axial-force coefficient
  was reconstructed about 2% below the pre-flight model [S10] abstract. **Inference:** splitting a product takes a
  second, independent measurement or a stated prior.
- **Hobby practice holds the motor and tunes drag.** Apogee's 2012 back-tracking method names the confound (a motor
  assumed at 80 N·s that delivered 75 N·s, −6.25%) [S12] pp. 2, 4–5; its 2025 article takes C_D from the coast
  instead, to remove the motor, which is "rarely reliable to the nominal values" [S13] p. 5. OpenRocket's guide says
  to weigh the rocket, then adjust only drag [S14]; NAR's 2017 TARC talk says to tune C_D only once the motor and
  other factors are controlled [S15] slide 5.
- **Reading thrust needs drag known, and the reverse.** A Harvey Mudd course note gets thrust from the boost as
  T = m(a + g) + D once drag is known [S16] eqs. 1.1–1.3. The RMIT 2025 paper on a J825R flight (500 Hz IMU, 50 Hz
  baro) finds each burn sample is one equation in two unknowns: "either thrust or drag must be known" [S11] pp. 9–10,
  eq. 12, Fig. 18.
- **The only joint fits found fail to separate.** arXiv 2512.22248 infers C_D and a thrust factor from apogee alone,
  with priors uniform on 0.3–0.9 and 0.8–1.2 [S7] p. 3. On 8 valid flights of one 66 mm rocket its mean absolute
  apogee error is 12.3 m against OpenRocket's 19.9 m [S7] Table 1, §6.2; it calls its values "effective" [S7] §6.4.
  **Inference** from its Table 1: across its 12 flights fitted C_D and thrust factor correlate at r = −0.92,
  and 11 of 12 fitted C_D lie at 0.838–0.939, at the top of the 0.3–0.9 training range or (one) above it. A 2000
  student fit of mean thrust and a drag constant to video put drag at "three times higher" than the literature [S17].
- **Apogee agreement hides offsetting errors.** Box et al.'s flight: simulated apogee 1.5% low (3,539 against
  3,594 m) while peak speed was 11% high (372.5 against 335 m/s) [S18] p. 28, Table 8. Peak speed is a second datum.
- **Boost-phase drag magnifies impulse error (Inference).** In the toy model drag takes only 2.4–7.2% of the
  impulse during the burn, so an impulse error read as boost drag is magnified 14–42 times. Purdue 2020's boost C_D
  at 2.14 times OpenRocket's ([flight-derived C_D](flight-derived-cd.md) §1) needs only a 3–8% impulse or mass error.

## 4. A numeric sketch of the information (Inference)

An invented rocket: 2.0 kg at liftoff, 1.7 kg dry, 54 mm, C_D 0.5 constant, a 600 N·s motor at a constant 300 N for
2.0 s, standard-atmosphere density, vertical, no wind. Apogee 2,162 m at 19.5 s; peak speed 284 m/s (Mach 0.83).
Baro 1 m RMS white at 20 Hz; accelerometer 0.1 g RMS white at 100 Hz. Information from central-difference
sensitivities; SDs are Cramér–Rao bounds as a percent of nominal, mass known.

| fit | SD k_D | SD k_I | r(k_D, k_I) |
|---|---|---|---|
| baro, liftoff to apogee | 0.041% | 0.015% | +0.976 |
| baro, boost only (0–2 s) | 15.5% | 0.39% | +0.956 |
| baro, coast only, burnout state free | 0.092% | n/a | n/a |
| accelerometer, liftoff to apogee | 0.31% | 0.046% | +0.095 |
| accelerometer, its bias and scale factor free | 2.3% | 2.2% | −0.989 |
| baro + accelerometer, baro offset and accelerometer bias and scale free | 0.056% | 0.027% | +0.964 |

- **Noise and rate move the bounds, not the correlation.** With white noise the SDs scale as σ/√f: 3 m at 10 Hz
  gives 0.17% on k_D, 0.3 m at 50 Hz 0.008%, and r stays +0.976 in both, since it depends only on the shapes of the
  sensitivities. No source found states a rate or noise at which drag and impulse separate in practice; RMIT's
  virtual tests at 9, 100 and 1,000 Hz improved with rate, worst at the rail and at sharp thrust features [S11] p. 5.
- **Sign.** r is positive: more impulse is offset by more drag. Here apogee moves +1.10% per 1% of impulse and
  −0.48% per 1% of drag.
- **Model error dominates (linearized least-squares bias, full trace).** C_D × (1 + 0.2M²) fitted as constant:
  k_D +5.4% (baro) or +5.7% (baro + accelerometer), k_I −0.7% or −0.6%. A thrust slope of ±20% end to end at the
  same impulse: k_D ∓2.6%, k_I ∓0.95%. An altitude reading 1% high (a non-standard day): k_I +1.2% or +1.1%.
  The larger bias in each case is 60–150 times its white-noise bound.
- **Predicting apogee from the log cut at burnout (ADR-257's comparison).** With priors of 10% on k_D and 5% on k_I,
  a boost-only fit of both gives an apogee SD of 3.85% (baro) or 0.30% (both sensors). A +5% thrust slope then moves
  fitted k_D by −9.8% or −24% and predicted apogee by +3.5% or +11%. With k_D held at its prior and impulse fitted
  alone, the same slope moves apogee −1.2% or −0.2%; a Mach-rising C_D stays invisible to that fit (+3.3%, +3.0%).

## 5. Priors

- **Impulse.** Published tools use σ 1% (a 5% alternate run) [S19] pp. 5, 7 and 2.5% [S20]; arXiv 2512.22248 uses
  uniform ±20% [S7]. The evidence on lots is wider and biased low: [high-power lot impulse](hpr-lot-impulse.md) puts
  current H-and-up lots 0 to −4% with −9% not excluded, and [motor data](motor-data.md) §5 gives certification
  scatter. NFPA 1125 caps a type's impulse SD at 6.7% of its mean (hpr-lot-impulse §1). **Inference:** an impulse
  scale centered at 0.97–1.00 with σ 3–5% covers the H-and-up evidence. F-and-smaller retests
  ran a median −9.4% (−21.5% to +8.5%, [motor data](motor-data.md) §5), so those need a prior centered near 0.91.
- **Drag.** Tools use σ 3.8% [S20] and 20% [S18] p. 27, [S19] Table 4. The coast fits' quartiles 0.89–1.11
  ([flight-derived C_D](flight-derived-cd.md) §2) are about σ 16% if normal (**Inference**).
- **Calibration across flights.** In Kennedy and O'Hagan's form, observation = ρ·η(x, θ) + δ(x) + noise, with η
  the code, θ the calibration knobs and δ a discrepancy function; the fitted θ is only "best-fitting" [S21]
  §4.2–4.3. In O'Hagan's simple-machine example the true slope is 0.65; fits that ignore δ settle at 0.529–0.605
  with unlimited data, and only a constrained δ comes close (peak ≈ 0.63) [S22] slides 11, 21, 25; realistic priors
  on δ are needed [S23] abstract. **Inference:** M7.5's correction is such a δ, identifiable only through the shape
  put into it, and only from flights that weigh boost against coast differently (different motors, masses, speeds).

## What this suggests

1. **M7.3: never free mass beside both scales.** Put the weighed mass on a prior (the symmetry is exact), and report
   each scale's interval with r(k_D, k_I). Expected: without this, a baro fit's mass and scales trade one for one.
2. **M7.3: expect ADR-257's model-mismatch set to fail bounds read from the information matrix.** NASA's factor
   was 3–10 [S1], [S3]; the sketch's biases are 60–150 times the bound, so no factor fitted on other seeds covers a
   missing C_D(M) or thrust shape: model those first, then fit the factor as the ADR allows.
3. **ADR-257's burnout prediction: fit impulse only from the boost** and carry drag from its prior or a coast fit.
   Expected in the sketch: a thrust-shape error costs 0.2–1.2% of apogee instead of 3.5–11%; a Mach-rising C_D
   costs +3.0–3.3% instead of −1.9% to +2.3%, so the coast fit's C_D(M) matters.
4. **With an accelerometer, free its bias and scale factor beside the baro offset**: the accelerometer alone with a
   free scale factor sits at r = −0.989.
5. **M7.5 and two-knob truing: drag from coasts, impulse from boosts, never both from apogees**; add peak speed where
   a log has it; give the correction a shape (drag scale tied to Mach) and report r between its knobs across flights.

## Open questions

- Jategaonkar's and Klein and Morelli's books and Walter and Pronzato were not reached; Brynjarsdóttir and O'Hagan
  was read only as its abstract, Mars 2020's reconstruction only as its abstract.
- The sketch is one invented rocket with linearized biases; it leaves out wind, rail, pitch-over, baro lag and the
  Mach error in [altimeter Mach error](altimeter-mach-error.md). The private collection's own fits (aggregates only)
  would give the real correlation.
- Accelerometer scale error at 5–30 g on hobby boards is not bounded by any source read.
- Not reached: SERA-3 (AIAA 2018-2382), a UMD airbrake estimator (AIAA 2024), Tang and Jiang's 2024 UKF
  identification of experimental rockets (CC BY 4.0, PDF refused), NAR R&D reports.

## Sources

Read 2026-10-16; copies in the gitignored `refs/papers/` and `refs/research/drag-impulse-identifiability/`, with
the sketch's script. Terms: S1, S2, S4, S9 are US Government works; S3, S5, S6, S10 are marked public use permitted;
S7 is CC BY 4.0; S20 is MIT; the rest reserved or unstated, cited only.

- **S1.** Maine and Iliff, NASA RP-1168 (1986). <https://ntrs.nasa.gov/citations/19870020066>
- **S2.** Maine and Iliff, NASA RP-1077 (1981). <https://ntrs.nasa.gov/citations/19810019327>
- **S3.** Morelli and Klein, J. Guid. Control Dyn. 20(1):74–80 (1997), journal pages; on NTRS as NASA TM-97-208111.
  <https://ntrs.nasa.gov/citations/19980048414>
- **S4.** Klein and Murphy, NASA/TM-1997-207993. <https://ntrs.nasa.gov/citations/19980210397>
- **S5.** Karlgaard et al., Ares I-X best estimated trajectory (2011). <https://ntrs.nasa.gov/citations/20110014643>
- **S6.** Karlgaard and Schoenenberger, entry atmosphere estimation. <https://ntrs.nasa.gov/citations/20170001233>
- **S7.** Pandey and Pandey, arXiv:2512.22248v1 (2025). <https://arxiv.org/abs/2512.22248>
- **S8.** Raue et al., Bioinformatics 25(15):1923–1929 (2009).
  <https://academic.oup.com/bioinformatics/article/25/15/1923/213246>
- **S9.** Evans, NACA Report 1160 (1954). <https://ntrs.nasa.gov/citations/19930092188>
- **S10.** Karlgaard et al., MEDLI2 reconstruction, AIAA SciTech 2022. <https://ntrs.nasa.gov/citations/20210024320>
- **S11.** Bawa and Michalski, RMIT preprint (2025), in <http://ftp.demec.ufpr.br/foguete/bibliografia/>.
- **S12.** Van Milligan, Peak of Flight 303. <https://www.apogeerockets.com/education/downloads/Newsletter303.pdf>
- **S13.** Van Milligan, Peak of Flight 651. <https://www.apogeerockets.com/education/downloads/Newsletter651.pdf>
- **S14.** OpenRocket user guide, overrides and surface finish.
  <https://openrocket.readthedocs.io/en/latest/user_guide/overrides_and_surface_finish.html>
- **S15.** Barber, NARCON 2017; the live link returns 403, so read through `web.archive.org/web/2020id_/` and it.
  <https://nar.org/wp-content/uploads/2016/07/NARCON-2017-TARC-Flight-Testing-Barber.pdf>
- **S16.** Harvey Mudd AdvRoc course note. <https://pages.hmc.edu/spjut/AdvRoc/PDF/ThrustCurveFromFlightData.pdf>
- **S17.** Benson, St. Lawrence University student project (2000).
  <https://myslu.stlawu.edu/~jmil/physics/legacy/student_projects/2000/stacey.shtml>
- **S18.** Box, Bishop and Hunt, J. Aerosp. Eng. 24(1) (2011), author copy. <https://eprints.soton.ac.uk/73938/>
- **S19.** Eerland, Box, Fangohr and Sóbester, AIAA 2017-1556, author copy. <https://eprints.soton.ac.uk/404214/>
- **S20.** RocketPy, Monte Carlo analysis notebook (MIT).
  <https://github.com/RocketPy-Team/RocketPy/tree/master/docs/notebooks/monte_carlo_analysis>
- **S21.** Kennedy and O'Hagan, J. R. Stat. Soc. B 63:425–464 (2001); preprint.
  <https://www.asc.ohio-state.edu/statistics/comp_exp/jour.club/kennedy01.pdf>
- **S22.** O'Hagan, "Inverting an Imperfect Model", slides, Warwick (2015).
  <https://warwick.ac.uk/fac/sci/maths/research/events/2014-15/nonsymposium/uccm/>
- **S23.** Brynjarsdóttir and O'Hagan, Inverse Problems 30 (2014). <https://tonyohagan.co.uk/academic/abs/simmach.html>

[adr257]: ../decisions/0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
