# ADR-371: M1.14b6, wind from the GPS descent: the method's settings and what is compared (2026-11-22)

- **Status:** accepted; carries out [ADR-351][adr-351] decision 2's line for M1.14b6
- **Summary:** The wind is read from a GPS-tracked descent as the ground velocity plus `w/g` times its rate of change, from a quadratic fitted over 10 s around each fix, and only where the body falls steadily at 2 to 35 m/s (7 to 115 ft/s). It is run on sixteen public flights, every one HPR Sim flies or predicts and RocketPy's other example logs: four are compared with ERA5, two are reconstructed without a readable ERA5 file of their day, two fell too fast for a canopy and eight logged no position. The compared speed is 3.50 m/s (7.8 mph) from ERA5's RMS. Amended 2026-11-23 (#1605): three public team logs are read too, 19 flights in all; with Itzamna, five are compared, 3.49 m/s RMS. Prometheus's two receivers agree to 0.51 m/s (1.1 mph) RMS, a floor on the noise between receivers.

[adr-351]: 0351-m1-14b-in-eight-the-basic-finners-normal-force-first.md
[note]: ../research/wind-from-descent.md
[report]: ../../validation/reports/descent-wind.md
[page]: ../physics/wind.md#wind-from-a-gps-tracked-descent

**Context.** ADR-351 asked for a documented reconstruction, the GPS ground velocity plus `w/g`
times its rate of change (Hock and Franklin 1999), pinned on synthetic descents at 5 and 25 m/s
through a wind that changes by 2 m/s per 100 m, within 0.3 m/s RMS at both, with the ground
velocity alone failing that line at 25 m/s; then run on every public flight whose log holds a
GPS track of the descent and set beside ERA5 by height band. The [research note][note] gives the
method and its bounds but no settings: how the velocity and its rate are taken from fixes, which
fixes are a steady fall, and what a log's repeated fixes mean.

The sixteen flights are the seven the real-flights report flies, the five predicted
before their logs were compared (three EuRoC 2022 flights and two Macie XL flights), and four
more example logs in RocketPy's checkout whose rockets HPR Sim doesn't fly: Camões and Halcyon
(EuRoC 2023), Hedy (EuRoC 2025) and Valkyrie (2025). The notebooks give Camões's, Halcyon's and
Hedy's site, hour and ERA5 file; Valkyrie's reads a live forecast, and its log holds no position.
Eight of the sixteen logs hold a position. Two public team logs in the public-flights note also
hold GPS fixes in the descent, Hyperion-II's TeleMega and UTRGV's Featherweight GPS; they are not
read yet ([#1605](https://github.com/nrdptel/fusionspace-eridanus/issues/1605)), a gap against
[ADR-351][adr-351]'s line, which asks for every public flight with a GPS track of the descent.

**Decision.**

1. **The fit.** At each fix, a least-squares quadratic in time over the fixes within 10 s
   centered on it: the slope is the velocity, twice the curvature the acceleration. A fit is
   kept only with at least five fixes reaching a quarter of the window on each side, so a fix at
   a track's end, or with no fix for 2.5 s on one side, as beside a gap, is not read. A shorter
   gap inside a window is fitted across. 10 s spans about two and a half periods of a 4 s swing,
   the note's reading (§4) of GRUAN's filter for a radiosonde's swing; on a synthetic 0.95 m/s
   swing of 2 to 5 s period it leaves under 0.15 m/s RMS. It differentiates positions rather
   than reading the receiver's Doppler velocity, as Hock and Franklin do, because none of the
   logs keeps a velocity vector.
2. **A steady fall.** A fix is read where the fit gives a descent of 2 to 35 m/s and a vertical
   acceleration of at most 0.5 m/s². In a steady fall the descent rate changes only with the
   air's density, `w²/(2H)`, 0.04 m/s² at 25 m/s; an opening or a landing changes it by several
   m/s². 35 m/s is past the note's fastest drogue (30 m/s); the tests pin the method at 5, 25
   and 33 m/s. A faster fall is not a canopy's, and the lag model is not shown to hold for a
   tumbling body: such a flight is listed as not run, with its median rate.
3. **A new fix is the receiver's.** A receiver repeats its last fix between updates and after it
   loses lock. A row is a new fix only when the receiver's own values change: latitude and
   longitude, and its altitude where the track reads it; in AltOS's CSV, its GPS clock with the
   lock flag set, the fix timed on that clock. A barometric height that changes under a repeated
   position is not a new fix: such a row would read a motionless rocket, then a jump. The report
   counts each flight's repeats and its longest gap between fixes.
4. **ERA5 as HPR Sim reads it.** Each compared flight's ERA5 wind is read as a sounding at its
   site and hour, interpolated by components in height: for the real-flights report's flights,
   its file and hour; for the other example logs, their notebooks'. A height outside ERA5's
   levels is counted and not compared. Heights above the pad come from the receiver's first fix
   where the track reads the receiver's altitude, and from the barometric log where it doesn't,
   the site's elevation added for ERA5.
5. **Not compared, and why.** Prometheus's ERA5 file is of the year after its flight. Hedy's is
   netCDF-4, which HPR Sim reads only through a converted copy checked against the source, as
   EuRoC 2022's is; none is made for it here. The public predictions' weather is not read here;
   their one log with a position, Astra's, fell too fast to read.
6. **Two receivers.** Prometheus's two receivers are reconstructed apart and their winds compared
   at fixes within 0.5 s. The report gives each one's highest fix and how far apart their last
   fixes are, so the reader can check they rode one rocket. Their difference is a floor on the
   noise between receivers: an error of the lag model, or a motion both share, cancels in it.
7. **Bands** below 500 m, 500 to 1,000, 1,000 to 2,000, 2,000 to 3,000 m and from 3,000 m up,
   above the pad, pooled over the compared flights, each with its counts of flights and fixes,
   the mean and RMS of the difference in speed and the RMS of the vector difference.
8. **What is committed.** Each compared flight's sums by band, from which CI recomputes the
   bands, and each fix's wind only where the log is read whole from RocketPy's MIT repository.
   CATS publishes EuRoC logs under GPL-3.0, and three of RocketPy's copies are files of
   RocketPy-Team's GPL-3.0 fork of its 2023 repository (the notices' `rocketpy` row), so CATS
   logs give statistics only. The page ends with the
   Copernicus credit.
9. **Held.** `cargo xtask descent-wind --check` reproduces the report in the gate's `refs` step.
   In CI, a test recomputes its bands from the committed sums and its page from its data, checks
   it lists every flight of its table in order, the public flights HPR Sim flies or predicts
   first, and credits ERA5, and checks that the validation plan's
   row calls the reconstruction a method exactly while fewer than three flights are compared.

**Results** ([the report][report]). Juno III (102 fixes under its drogue), Genesis (56 under its
main), Camões (353 under its drogue) and Halcyon (1,951 under its main) are compared. Over 2,462
fixes the reconstructed speed is 0.87 m/s below ERA5's on average and 3.50 m/s from it RMS. Each
flight's mean is 1.1 to 1.5 m/s off, above ERA5's on three and below it on Halcyon, which gives
four fixes in five. Prometheus's receivers agree to 0.51 m/s RMS over 170 fixes, 1.35 m/s at most.

**Consequences.**

- Four compared flights meet ADR-351's count for a comparison, but nothing in them separates
  ERA5's miss at a launch field from a canopy's glide, which nothing bounds for hobby canopies
  ([the wind page][page]).
- A checked conversion of Hedy's ERA5 file, or an ERA5 file of Prometheus's day, would add a
  fifth or sixth compared flight.
- M7.3's wind fit can take this reading of the descent as its data and ERA5 as its prior.
- **Since (2026-11-23, [#1605](https://github.com/nrdptel/fusionspace-eridanus/issues/1605)):**
  the report also reads three public team logs, 19 flights in all. Itzamna is compared, in an ERA5
  cut committed for it; five compared flights over 2,558 fixes give 3.49 m/s RMS. The figures
  above are this record's, of 16 flights.
