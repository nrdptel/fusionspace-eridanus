# ADR-082: Real flights read from refs, compared over the ascent, with checked explanations (2026-09-26)

- **Status:** accepted
- **Summary:** Real flights read from refs, compared over the ascent, with checked explanations

**Context.** M2.3b compares hpr with real flights: at least six in the report, each with its
apogee error and an altitude-trace RMS, the mean absolute apogee error against the 5% target of
`docs/VALIDATION.md`, and an explanation for each outlier. RocketPy 1.13.0's documentation flies
ten rockets against their teams' logs, most in an ERA5 file of the day. Its logs were shared with
RocketPy by the teams (each notebook records the permission), its motor files come from
ThrustCurve.org with each file's own terms, and only some of those are public domain. The public
designs of M1.4 fly a substitute bundled curve (ADR-007), which says nothing about a real flight.

**Decision.**

1. **The inputs stay in `refs/`.** `cargo xtask real-flights` reads each log, the example's own
   thrust file and the ERA5 file from the pinned RocketPy checkout, and commits only
   `validation/reports/real-flights.{json,md}`: each flight's apogees, errors, RMS and counts,
   and every file's SHA-256. Committing the logs would redistribute data shared with RocketPy,
   not with us (rule 4). So the flights run where the checkout is, as the corpus flights do
   (ADR-071); `--check` flies them again. CI, without `refs/`, holds the committed report to
   itself: its summary to its rows, its page to its data, and each explanation to its numbers
   (`hpr_validate::tests::real_flight_cases_report_apogee_and_trace_rms`).
2. **Seven flights:** the five with a public design already (Bella Lui, NDRT 2020, Prometheus,
   Juno III, Cavour), and Genesis and Lince, added to `rocket_mass.py` for this: COTS motors, in
   EuRoC 2023's classic file (their motors have no dry mass, like Cavour's). Left for later:
   Astra and Andromeda (COTS, but EuRoC 2022's file is netCDF-4 and needs ADR-081's conversion);
   Camões, Erebus 11, Halcyon and Hedy (their teams' own motors, outside rule 6's COTS scope);
   Valetudo and Defiance (an apogee, no trace); Valkyrie (inputs only in a data file, ADR-007).
   Juno III also flies its team's motor, but its design was already public: hpr flies the thrust
   file like any other, and nothing of a research motor is modelled.
3. **As a user would fly it:** the design (the example's masses, inertias and geometry), hpr's own
   aerodynamics, the example's own thrust file read as RocketPy reads it (a `(0, 0)` point first
   for `.eng`, clipped at the example's burn time, reshaped where the example reshapes; the
   total impulse matches RocketPy's reading of all seven to 1e-15, but Juno III's),
   the example's rail, site and launch hour (local hours converted to UTC), hpr's WGS84 Earth,
   and the ERA5 profile of ADR-081. Juno III's thrust file ends in five negative points, which
   hpr refuses; they are read as zero, adding 2.62 N s to its 8800 (reported), as RocketPy's
   flight holds its thrust at zero too. Prometheus flies the weather of 24 June 2023 for a 2022
   flight, as RocketPy's example does; RocketPy has no file of the day, and the row says so.
4. **hpr is read as the log's altimeter reads.** A barometric altimeter converts the pressure it
   measures to the standard atmosphere's altitude and subtracts the pad's: on a day warmer than
   the standard the air column is thicker and it reads less than the height climbed, by several
   per cent at Spaceport America in June. Prometheus's TeleMetrum and Juno III's RRC3 log their
   pressure, and their height columns are that reading less the first row's, to 0.195 m and
   1.321 m over the rows compared (the report computes the gap). So hpr's
   height goes through the same conversion, of the ERA5 pressure at its centre of mass
   (`Ussa76::pressure_altitude_m`, `hpr_validate::real_flight::Barometer`). NDRT's Featherweight
   Raven is a barometric altimeter by make. Cavour's CATS Vega, and Genesis's and Lince's filtered
   estimates, fuse a barometer with an accelerometer, and Bella Lui's avionics are unnamed: those
   four are marked *assumed* barometric, with the evidence in each row. The report gives hpr's
   apogee as a height too, and the mean both ways. Two logs also carry satellite (GNSS) heights,
   which the report reads: Juno III's rises 3369.3 m above its pad and Prometheus's 4133.0 m, so
   their barometric apogees are 0.935 and 0.943 of those, where hpr's conversion makes its own
   0.921 and 0.932 of its height. The conversion has the right direction and nearly the right
   size; it reads 1 to 2 points lower on both, on the side of both flights' misses. Juno III's
   log is cut while still climbing (item 5). Lining the two logs up by vertical speed (17.5 m/s,
   the log's at its cut), the satellite height there is about 3350 m, which would make its ratio
   0.941.
5. **Apogee** is the log's highest reading, read up to the recovery. Three logs have pressure
   transients near apogee: Juno III's reading, as it levels off, dips 94 m, then rises 62 m above
   the level within 0.3 s (the flight card gives that spike, 3213 m); Prometheus's drops 600 m and
   returns 8 m above its highest reading before; Lince's filtered height swings by hundreds of
   metres up to 3668.5 m, where its highest before, 3587.7 m, is its flight card's. No one
   threshold separates these from the ascent's own dips (Lince's drops over 50 m at 8.5 s), so
   each log is read to a time set by hand, before its transient, with the reason in its note
   (24.60 s, 29.58 s, 26.80 s; Juno III's cut also drops its two corrupt rows). At Juno III's cut
   its own velocity column still reads 17.5 m/s up and its satellite height climbs another 19 m,
   so its apogee may be 10 m to 20 m low: its miss would be about half a point larger, and on the
   recorded thrust about half a point smaller.
6. **The trace RMS is over the ascent**: each clock is aligned where its trace first reaches 30 m,
   since a log's zero is its own (armed, launch detected, power on), not ignition; the RMS runs
   over every log row from there to the first of the two apogees, hpr's heights interpolated on
   a 0.01 s grid of its dense output. The descent is left out: its events are the team's. The
   early climb of a barometric log is not a measure of the boost: Prometheus's barometer reaches
   150 m 0.87 s after its own speed column, integrated, puts it there, as the pressure in its bay
   lags or under-reads at speed, so no explanation rests on it.
7. **Diagnostic flights.** Each flight is flown again on the example's own drag (a constant, the
   notebook's knots, or its CSV files, scaled as the notebook specifies, on the example's radius),
   and, where the example reshapes its thrust file to a burn time and an impulse, on the file as
   recorded. Neither is a second prediction: a team's drag is an estimate, from a table, RASAero
   II, CFD or a constant the notebook doesn't source, and may have been tuned to its flight.
   RocketPy 1.13.0 itself doesn't fly two of the drags as written: a `power_off_drag` assigned
   after the rocket is built, or scaled in place, never reaches the function its flight evaluates,
   so it flies Bella Lui on 0.43 and Juno III on the unscaled curve. hpr flies what the notebooks
   specify.
8. **An explanation is a checked claim.** An outlier (outside the 5% target) must carry one, and a
   flight inside it must not. `drag` claims the flight on the team's drag is within the target,
   and the one on the recorded thrust file, where there is one, is not: the miss is consistent
   with hpr's drag. `thrust` claims the flight on the recorded thrust file
   is within the target and the one on the team's drag is not: the miss is consistent with the
   impulse the notebook sets, and not with drag. CI checks
   each claim against the row, each row's percentages against its metres, and the words, the
   digests of the committed files read and of each flight's inputs against the code's, so a
   change that makes a claim false or the report stale fails.

**Consequences.** Over seven flights, the mean absolute apogee error is 6.04%, outside the 5% target
(−8.90% to +10.40%; mean −0.25%), and the trace RMS is at most 7.26% of an apogee. The target is
missed however the assumed altimeters are read: 6.63% with only the three known barometric logs read
so (Lince, at +7.99%, would then miss with no checked explanation), and 4.47% only were every log a
height, which three of them are known not to be. Reading hpr as a barometer moves its apogee by
−7.9% (Juno III, in June's heat) to +1.7% (NDRT 2020, in February). Five flights are outliers. Four
are consistent with hpr's drag: on their teams' drag NDRT 2020 (+10.40%) lands at +0.17%, Prometheus
(−8.90%) at +0.45%, Cavour (+5.63%) at −2.29% and Genesis (−5.85%) at −0.08%; hpr's drag is low for
two and high for two. Prometheus's reading also rests on weather a year off its day; its share can't
be told apart, since Juno III, on its own day, shows as large a gap against its satellite height.
Juno III (−7.24%) is consistent with its motor's impulse: the notebook reshapes its team's curve to
8800 N s, 4.9% below the file (9249.0 N s; 9251.7 N s with its negative end read as zero), and on
the file as recorded hpr lands at +1.13%, where on the team's drag it lands at −9.05%. Lince, inside
the target, is −12.20% on its team's drag, not investigated. These are consistent explanations, not
proofs: the teams' drags and the reshape are estimates too. A log is a single flight, with its own
sensor and filter; the numbers are those seven flights', not a bound. The report is not reproduced
in CI, only held to itself there.
