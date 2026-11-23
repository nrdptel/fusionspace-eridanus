# ADR-375: M1.14b17, the log's part of a real flight's validation uncertainty (2026-11-23)

- **Status:** accepted; fixes the rules [ADR-373][adr-373] decision 2 left open for the data part; changes no comparison error, apogee or mean
- **Summary:** Each public real flight's log apogee gets a standard uncertainty u_D from its altimeter's published accuracy, where one is published: 8 of the 12 flights, from 1.23 m (4.0 ft) to 17.16 m (56.3 ft), each flight's error at least 10 times its own. Four flights' altimeters have none and are listed by name, with no u_D. Both apogees are barometric readings, so the standard atmosphere's distance from the day's air, as large as 282.4 m (927 ft), is in both and stays out of u_D: counted as heights, an error changes by at most 0.10 points of its apogee. An error in the simulated air is an input's, for M1.14b18.

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

- **How a maker's figure becomes a standard uncertainty.** A datasheet gives limits, a manual
  gives a "typical" figure or an accuracy with no confidence level.
- **Which flights have a published accuracy at all.** Four logs come from an altimeter no document
  names.
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
   the air the flight is flown in.
2. **The rules** are the *Guide to the expression of uncertainty in measurement*'s (JCGM
   100:2008).
   - Bounds of ±a, with nothing known inside them, give `a/√3` (4.3.7, eq. 7). A maker's limits
     are read this way, and so is a maker's accuracy with no confidence level (4.3.7, example 2).
   - An effect of known sign that is not corrected keeps its mean beside its spread (F.2.4.5). For
     a value equally likely anywhere from `lo` to `hi` that is the root mean square about zero,
     `√((lo² + lo·hi + hi²)/3)`. For ±a it is the first rule's figure; for one value it is its size.
   - A value kept in steps of `δ` gives `δ/√12` (F.2.2.1).
   - A maker's "typical" figure has no rule in the guide, which leaves it to judgment (F.2.3.1).
     It is taken as one standard deviation. Read as bounds it would be `√3` times smaller.
   - The components are taken as independent and add as a root sum of squares.
3. **Pressure to height.** A pressure error `δp` at the apogee is `δp/(ρ·g₀)` of height, the
   standard atmosphere's hydrostatic equation, with `ρ` its density at the site's elevation plus
   the log's apogee.
4. **The components, by altimeter.** Each figure's source is printed in [the report][report].

   | Altimeter | Flights | Components |
   |---|---|---|
   | Featherweight Raven | NDRT 2020 | the manual's pressure error as bounds; its stated resolution as a step; its formula's gap from the standard atmosphere ([the note][formulas]) |
   | Altus Metrum TeleMetrum (MS5607) | Prometheus | the datasheet's error band after a zero at one pressure, as bounds; its noise at the lowest oversampling it lists; its output step; the log's own gap between its height and its pressure |
   | Missile Works RRC3 (MS5607) | Juno III | the same band; the datasheet's noise at the RRC3's oversampling; the log's 10 Pa step ([the note][sensors]); the log's own gap; the report's cut of the log, 10 m to 20 m low |
   | CATS Vega (MS5607), telemetry | Cavour | the same band; whole meters cut toward zero; the Kalman filter's height against the barometric apogee and the formula's gap ([the notes][vega-column]) |
   | CATS Vega (MS5607), 9-sample median | Genesis, Lince | the same band; how far the median's highest value sits above the coast's curve ([the note][vega-column]); the formula's gap ([the note][vega]) |
   | PerfectFlite StratoLoggerCF | Macie XL, both flights | the manual's typical precision and typical calibration accuracy; the header's whole feet |

5. **No published accuracy, no u_D.** Bella Lui's log is the team's own avionics, and no document
   read names the sensor that made its height. Astra's, Andromeda's and Erebus 11's notebooks say
   "COTS altimeter" and name no make. The report lists the four by name. They get no band when the
   input part comes.
6. **The barometer's conversion is left out of u_D, with its measurement.** On a day that isn't
   the standard atmosphere's, a barometric reading isn't the height climbed. Each row gives that
   difference in HPR Sim's reading (the reading less the height) and in the log's (the log's
   apogee less the height at which the same altimeter, in the same air, reads it). The report then
   gives the error both ways: as readings, and as heights.
7. **The air's own error is an input's.** The log's altimeter read the day's real air. HPR Sim's
   reading uses the air the flight is flown in. Where that air is wrong, HPR Sim's reading moves and
   the log's doesn't, by about 0.35% of the reading for each kelvin of the air column
   ([the note][formulas]). M1.14b18's dispersion draws the air once, through the flight and through
   HPR Sim's reading both. This adds an input to M1.14b18; its done-when stands as ADR-373 wrote it.
8. **The check.** The report's own check recomputes each row's data part from the table above, its
   log's apogee, its site's elevation and its row's own formula gap, and refuses a row that
   differs: a part halved, a component dropped, a part given to a flight with no altimeter named, a
   part withheld from one that has one, another site's pressure conversion, another gap.

**Why not the alternatives.**

- *Put the conversion in u_D,* by treating the log's apogee as a height. It would put on one side
  an error both sides carry, which the Lisbon text says "will contribute two terms" where none
  belongs. A larger u_D makes a wider band, the flattering side.
- *Use the spread of altimeter pairs for every flight.* Two altimeters on one rocket differ by
  0.37% at the median over 59 published flights, about 0.39% for one unit ([the note][pairs]).
  ADR-373's done-when asks for the published accuracy, and that figure is other flights'. The
  report sets it beside u_D instead. It would also give the four unnamed altimeters a band.
- *Read "typical" as bounds.* It gives the smaller figure, and "typical" promises less than a limit.
- *The datasheet's band without a zero,* 1.5 to 2.0 mbar. Every altimeter here zeroes at the pad,
  and its error on the pad is in both of the two pressures a reading is the difference of.

**What was measured** ([the report][report]).

- u_D runs from 1.23 m (Macie XL's first flight) to 17.16 m (Juno III), 0.14% to 0.54% of the
  log's apogee. Of Juno III's 17.16 m, the cut is 15.28 m.
- Each of the 8 flights' errors is at least 10 times its own u_D; the fewest times is on Juno III.
- On 6 of the 8, u_D is under the pairs' 0.39% of the apogee.
- The conversion is as large as 282.4 m in a reading (Prometheus, whose log reads 3,895.8 m).
  The error as heights differs from the error as readings by 22.6 m at the most (Prometheus), and
  as a share of the apogee by at most 0.10 points (Juno III).

**What this does not decide.**

- Whether the datasheet's "autozero at one pressure point" covers a zero taken on the pad minutes
  before the flight, at another bay temperature. The sheet doesn't define it.
- Which board Prometheus's TeleMetrum is. The maker's manual lists another sensor for its first
  versions. The MS5607 is taken because the Altus Metrum logs step their pressure by 1 Pa, that
  part's output step ([the note][sensors], §3).
- Which Raven wrote NDRT's log. The accuracy is the Raven4 manual's.
- The lag of a vented bay and its ports, a bay outside 0 to 50 °C, and how far noise lifts the
  highest of many samples on the two logs with a noise component. None has a published figure.
- Whether the components are independent. A sensor's noise and its error band may overlap.
- What V&V 20's own section on a shared parameter would give. It was not read.
- Whether u_D is too small. The pairs' figure says it is on 6 of the 8. A smaller u_D makes a
  narrower band, so the choice doesn't flatter the comparison.

**Done when** (ADR-373's, for M1.14b17), and where each clause is met:

- *Each public flight's row gives its log apogee's standard uncertainty, each component cited.*
  The section's table and its list of sources, for the 8 flights with a published accuracy.
- *The share the simulation and the log take from the same barometer is combined as a cited source
  gives it, or shown by a measurement to cancel.* The section's second table: the conversion in
  both readings and the error both ways.
- *A test pins one flight's data part worked by hand.* Two do: a PerfectFlite's on Macie XL's
  first flight, and a Raven's on NDRT 2020 with the pressure's conversion.
- *A flight whose altimeter has no published accuracy is listed by name, with no band.* The four
  of decision 5.

**Consequences.** Each row of the report's validation uncertainty gains its log's apogee, HPR
Sim's apogee as a height, the log's apogee as a height, and its data part. The report flies no more
flights than before: the height at which the altimeter reads the log's apogee is asked of the
flight already flown. M1.14b18 takes u_D as it is and adds the air to its inputs.
