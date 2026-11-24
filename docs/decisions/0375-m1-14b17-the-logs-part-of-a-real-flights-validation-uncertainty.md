# ADR-375: M1.14b17, the log's part of a real flight's validation uncertainty (2026-11-23)

- **Status:** accepted; fixes the rules [ADR-373][adr-373] decision 2 left open for the data part; changes no comparison error, apogee or mean of the report
- **Summary:** Each public real flight's log apogee gets a standard uncertainty u_D from a published accuracy, its altimeter's or its pressure sensor's: 8 of the 12 flights, from 1.23 m (4.0 ft) to 10.09 m (33.1 ft). What is known to make a log read low or high is corrected, not added to u_D: Juno III's log by +16.32 m (53.5 ft). Each error against its corrected log is at least 17 times its u_D. Four flights have none and are listed by name. Both apogees are barometric readings, so the standard atmosphere's distance from the day's air is in both and stays out of u_D: counted as heights, an error changes by at most 0.10 points of its apogee.

[adr-373]: 0373-m1-14b7-in-three-the-numerical-part-first.md
[note]: ../research/validation-uncertainty-vv20.md
[report]: ../../validation/reports/real-flights.md#how-much-of-each-error-the-log-could-be
[sensors]: ../research/altimeter-sensor-noise.md
[formulas]: ../research/altimeter-height-formulas.md
[pairs]: ../research/dual-altimeter-spread.md
[vega]: ../research/cats-vega-height.md
[vega-column]: ../research/cats-vega-filtered-column.md

**Context.** [ADR-373][adr-373] decision 2 gave M1.14b17 the data part: each public flight's log
apogee gets a standard uncertainty from its altimeter's published accuracy and its reading's
conversion to a height. It left three things open.

- **How a published figure becomes a standard uncertainty.** A datasheet gives limits, a manual
  gives a "typical" figure or an accuracy with no confidence level, and some effects have a known
  sign.
- **Which flights have a published accuracy at all.** Three logs' altimeters have a manual with
  an accuracy. Five more name a pressure sensor whose datasheet has one. Four logs come from an
  altimeter no document names.
- **The share both sides take from the same barometer.** It was to be combined as V&V 20 combines
  a shared error, from a source that was read, or measured to cancel. The standard's own section
  on it is paywalled and was not read. The two open texts that discuss it say the plain root sum
  of squares should not be used for a parameter in both the simulation and the data (Eça, Dowding
  and Roache 2020, p. 7), and that its effects "should approximately cancel" if treated correctly
  (the 2008 Lisbon workshop's procedure, p. 4). Neither gives the shared term's formula
  ([the note][note]). So this record takes the second route and measures.

**Decision.**

1. **What u_D is the uncertainty of.** The log's apogee as a barometric reading: the 1976 standard
   atmosphere's altitude of the pressure at the apogee, less that of the pressure on the pad. That
   is the quantity the report compares. HPR Sim's apogee is read the same way, from the pressure of
   the air the flight is flown in. The report's flight table says which logs name a barometric
   altimeter and which are taken as one.
2. **The rules** are the *Guide to the expression of uncertainty in measurement*'s (JCGM
   100:2008).
   - Bounds of ±a, with nothing known inside them, give `a/√3` (4.3.7, eq. 7). A maker's limits
     are read this way, and so is a maker's accuracy with no confidence level (4.3.7, example 2).
   - A recognized effect of known sign is corrected, not left in the uncertainty (3.2.4; the note
     to 6.3.1 says enlarging the uncertainty instead should be avoided). For a log's error equally
     likely anywhere from `lo` to `hi`, the log's apogee is corrected by the mean, `−(lo + hi)/2`,
     and only the spread about the mean stays, `(hi − lo)/√12` (F.2.4.5 corrects by the mean and
     keeps the variance about it; 4.3.7 gives that variance).
   - A value kept in steps of `δ` gives `δ/√12` (F.2.2.1).
   - A maker's "typical" figure has no rule in the guide, which leaves it to judgment (F.2.3.1).
     It is taken as one standard deviation. Read as bounds it would be `√3` times smaller.
   - The components are taken as independent and add as a root sum of squares. The corrections
     add.
3. **Pressure to height.** A pressure error `δp` at the apogee is `δp/(ρ·g₀)` of height, the
   standard atmosphere's hydrostatic equation, with `ρ` its density at the site's elevation plus
   the log's apogee.
4. **The zero on the pad.** A reading is a difference of two standard altitudes, not of two
   pressures. Write the sensor's error at the apogee as its error on the pad, `e₀`, plus its change
   since, `d`. The reading's error is `−d/(ρₐ·g₀) − e₀·(1/ρₐ − 1/ρ₀)/g₀`, with `ρₐ` and `ρ₀` the
   standard's densities at the apogee and on the pad. The zero removes `e₀` from the first term
   only. So each pressure sensor has two components: "sensor", the bound on `d`, and "pad offset",
   the bound on `e₀` times the difference between a pascal's two heights.
5. **The components, by altimeter.** Each figure's source is printed in [the report][report].

   | Altimeter | Flights | Components |
   |---|---|---|
   | Featherweight Raven | NDRT 2020 | the manual's one pressure error, as bounds on the change since the pad and again on the pad's offset; its stated resolution as a step; its formula's gap from the standard atmosphere, either way ([the note][formulas]) |
   | Altus Metrum TeleMetrum (MS5607) | Prometheus | the datasheet's error band after a zero at one pressure, as bounds; its band with no zero, on the pad's offset; its noise at the lowest oversampling it lists; its output step; the log's own formula gap, corrected |
   | Missile Works RRC3 (MS5607) | Juno III | the same two bands; the datasheet's noise at the RRC3's oversampling; the log's 10 Pa step ([the note][sensors]); the log's own formula gap, corrected; the report's cut of the log, 10 m to 20 m low, corrected by 15 m |
   | CATS Vega (MS5607), telemetry | Cavour | the same two bands; whole meters cut toward zero, corrected by half a meter; the Kalman filter's height against the barometric apogee; the formula's gap, corrected ([the notes][vega-column]) |
   | CATS Vega (MS5607), 9-sample median | Genesis, Lince | the same two bands; how far the median's highest value sits above the coast's curve, corrected ([the note][vega-column]); the formula's gap, corrected ([the note][vega]) |
   | PerfectFlite StratoLoggerCF | Macie XL, both flights | the manual's typical precision and typical calibration accuracy; the header's whole feet |

6. **A log's own formula gap is measured with its sign.** Where a log keeps its pressure, the
   report already measured the largest gap between its height column and the standard's reading of
   that pressure. It now also keeps the gap at the log's highest row, signed. Juno III's is
   −1.32 m: the RRC3's formula reads low there, as its manual's formula does ([the note][formulas]).
7. **The corrected error.** Each row gives its corrections, their sum, and the error against the
   corrected log, `E` less the correction. The band M1.14b18 draws is about that error. The
   report's flight table, its percentages and its means stay on the logs as logged;
   [#1615](https://github.com/nrdptel/fusionspace-eridanus/issues/1615) asks whether they should
   take the corrections.
8. **No published accuracy, no u_D.** Bella Lui's log is the team's own avionics, and no document
   read names the sensor that made its height. Astra's, Andromeda's and Erebus 11's notebooks say
   "COTS altimeter" and name no make. The report lists the four by name. They get no band when the
   input part comes.
9. **The barometer's conversion is left out of u_D, with its measurement.** On a day that isn't
   the standard atmosphere's, a barometric reading isn't the height climbed. Each row gives that
   difference in HPR Sim's reading (the reading less the height) and in the log's (the log's
   apogee less the height at which the same altimeter, in the same air, reads it). The report then
   gives the error both ways: as readings, and as heights.
10. **The air's own error is an input's.** The log's altimeter read the day's real air. HPR Sim's
    reading uses the air the flight is flown in. Where that air is wrong, HPR Sim's reading moves
    and the log's doesn't, by about 0.35% of the reading for each kelvin of the air column
    ([the note][formulas]). M1.14b18's dispersion draws the air once, through the flight and
    through HPR Sim's reading both. This adds an input to M1.14b18; its done-when stands as
    ADR-373 wrote it.
11. **The check.** The report's own check recomputes each row's data part from the table above, its
    log's apogee, its site's elevation and its row's own formula gap, and refuses a row that
    differs: a part halved, a component dropped, a correction changed, a part given to a flight
    with no altimeter named, a part withheld from one that has one, another site's pressure
    conversion at the apogee or on the pad, another gap or the same gap with its sign turned. It
    holds HPR Sim's apogee as a height to the flight's row. The log's apogee as a height has no
    other copy in the report: only the check that flies the flights again, where the logs are,
    holds it.

**Why not the alternatives.**

- *Leave a known effect uncorrected and add its mean to u_D.* An earlier draft did, as a root mean
  square about zero. The guide says to correct. On Juno III the draft gave a u_D of 17.16 m and
  left the error 16 m nearer zero: a wider band about a smaller error, the flattering side twice.
- *Put the conversion in u_D,* by treating the log's apogee as a height. It would put on one side
  an error both sides carry, which the Lisbon text says "will contribute two terms" where none
  belongs. A larger u_D makes a wider band, the flattering side.
- *Use the spread of altimeter pairs for every flight.* Two altimeters on one rocket differ by
  0.37% at the median over 59 published flights, about 0.39% for one unit ([the note][pairs]).
  ADR-373's done-when asks for the published accuracy, and that figure is other flights'. The
  report sets it beside u_D instead. It would also give the four unnamed altimeters a band.
- *Read "typical" as bounds.* It gives the smaller figure, and "typical" promises less than a limit.
- *The datasheet's band with no zero for the whole sensor term,* 2.0 mbar. Every altimeter here
  zeroes on the pad, so the band after a zero bounds the change since. The band with no zero
  bounds what the zero leaves, decision 4's second term.

**Which way the choices lean.** A larger u_D makes a wider band later, which an error fits inside
more easily. Three choices lean it large: a "typical" figure as one standard deviation (Macie
XL, whose errors are over 100 times u_D either way); the Raven's one figure used both for the
pad's offset and for the change since (NDRT 2020, the one flight whose u_D is over the pairs'
figure); and Prometheus's noise at the MS5607's noisiest setting, its altimeter's being
unpublished. The Raven's double use is larger than using the figure once, not the largest
reading: as independent bounds on the two pressures it would give
100/√3 Pa × √(0.0966² + 0.0849²) m/Pa = 7.43 m in place of 5.62 m. One choice leans small: the
sheet's band after an autozero is taken for the change since the pad's zero (the first point
below); its band with no autozero is twice as wide. The other figures are makers' limits for a
kind of sensor, and on 7 of the 8 flights u_D is under the pairs' 0.39%, so on that evidence it
is more likely too small. (Amended for
[#1616](https://github.com/nrdptel/fusionspace-eridanus/issues/1616): the first version named two
choices, and called the Raven's the larger reading.)

**What was measured** ([the report][report]).

- u_D runs from 1.23 m (Macie XL's first flight) to 10.09 m (Prometheus), 0.14% to 0.43% of the
  log's apogee.
- The largest correction is +16.32 m, on Juno III: 15.00 m for the cut and 1.32 m for the formula.
- Each of the 8 flights' errors against its corrected log is at least 17 times its own u_D; the
  fewest times is on Lince.
- On 7 of the 8, u_D is under the pairs' 0.39% of the apogee. NDRT 2020's is 0.43%.
- The conversion is as large as 282.4 m in a reading (Prometheus, whose log reads 3,895.8 m).
  The error as heights differs from the error as readings by at most 22.6 m (Prometheus), and
  as a share of the apogee by at most 0.10 points (Juno III). In meters that difference is larger
  than u_D on 2 of the 8 flights. It is the error itself counted in the day's air instead of the
  standard's, a share of the error, so the share is what the comparison keeps.

**What this does not decide.**

- Whether the datasheet's "autozero at one pressure point" covers a zero taken on the pad minutes
  before the flight, at another bay temperature. The sheet doesn't define it.
- Which board Prometheus's TeleMetrum is. The maker's manual lists another sensor for its first
  versions. The MS5607 is taken because the Altus Metrum logs step their pressure by 1 Pa, that
  part's output step ([the note][sensors], §3).
- Which Raven wrote NDRT's log. The accuracy is the Raven4 manual's "less than 1 mbar … over
  most conditions". The same sentence gives 3 mbar over −40 to 85 °C, which is not used.
- The lag of a vented bay and its ports, a bay outside 0 to 50 °C, the sensor's drift with age
  and with its supply voltage (the datasheet lists both apart), the pad sample's own noise and
  step, and how far noise lifts the highest of many samples on the two logs with a noise
  component. None has a published figure that fits.
- The formula gap's own uncertainty. The gap corrects the log with nothing left in u_D, but it is
  measured against the log's first pressure row, so that row's noise and step stay in it: 0.44 m
  on Juno III's 10 Pa steps (3.6 Pa of noise and 10/√12 Pa of step, at 0.0962 m/Pa on the pad),
  and 1.10 m to 1.24 m on the other four logs if their noise is the sensor's noisiest setting.
  Added to each u_D as one more component it would raise none by more than 0.08 m (Cavour,
  7.19 m to 7.27 m), so it is left out
  ([#1616](https://github.com/nrdptel/fusionspace-eridanus/issues/1616)).
- Whether a PerfectFlite reads a difference of two standard altitudes or takes its pad as sea
  level ([the note][formulas]). The two differ by 0.68% at a 300 m pad, and the report has no site
  elevation for Macie XL to size it with.
- Whether the components are independent. A sensor's noise and its error band may overlap, and so
  may its pad offset and its change since.
- Whether the Vega's peak, measured on four logs, is the same on Genesis's and Lince's.
- What V&V 20's own section on a shared parameter would give. It was not read.
- Whether the report's headline errors should take the corrections
  ([#1615](https://github.com/nrdptel/fusionspace-eridanus/issues/1615)).

**Done when** (ADR-373's, for M1.14b17), and where each clause is met:

- *Each public flight's row gives its log apogee's standard uncertainty, each component cited.*
  The section's table and its list of sources, for the 8 flights with a published accuracy:
  3 from the altimeter's own manual, 5 from the datasheet of a pressure sensor: the one the maker
  names, or for Prometheus's TeleMetrum, whose manual lists two, the MS5607 its log's 1 Pa step
  points to.
- *The share the simulation and the log take from the same barometer is combined as a cited source
  gives it, or shown by a measurement to cancel.* The section's second table: the conversion in
  both readings and the error both ways.
- *A test pins one flight's data part worked by hand.* Two do: a PerfectFlite's on Macie XL's
  first flight, and a Raven's on NDRT 2020 with the pressure's conversion at the apogee and on
  the pad. A third pins Juno III's corrections on round figures.
- *A flight whose altimeter has no published accuracy is listed by name, with no band.* The four
  of decision 8.

**Consequences.** Each row of the report's validation uncertainty gains its log's apogee, HPR
Sim's apogee as a height, the log's apogee as a height, and its data part with its correction. A
flight's row gains its formula gap at the log's apogee, signed. The report flies no more flights
than before: the height at which the altimeter reads the log's apogee is asked of the flight
already flown. M1.14b18 takes u_D and the corrected errors as they are and adds the air to its
inputs.
