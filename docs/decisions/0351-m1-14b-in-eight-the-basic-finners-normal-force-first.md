# ADR-351: M1.14b in eight: the Basic Finner's normal force first (2026-11-16)

- **Status:** accepted; splits [M1.14b][m1-14b] into [M1.14b1][m1-14b1] to [M1.14b8][m1-14b8], each with its own *done when*, M1.14b's *done when* whole across the eight, and so plans every remaining increment of M1.14b; under [ADR-143][adr-143] §6b, not changed; amended 2026-11-16 ([#1504][i-1504]): M1.14b4's *done when* states how case 3's damping moments are formed, a condition added, none removed
- **Summary:** ADR-143 §6b names nine references for Mach 1.5 to 2.5. Three are refused on what the research notes found; the other six need work the tree lacks, in eight increments. The first ships here: against DREV-TM-9703's free-flight range fits of the Basic Finner, HPR Sim's normal-force slope reads 11.2% and 16.1% low at Mach 1.832 and 2.375, and its center of pressure 0.34 and 0.18 calibers forward, the side that understates stability.

[m1-14b]: ../decisions-and-roadmap.md#m1-14b
[i-1504]: https://github.com/nrdptel/fusionspace-eridanus/issues/1504
[m1-14b1]: ../decisions-and-roadmap.md#m1-14b1
[m1-14b2]: ../decisions-and-roadmap.md#m1-14b2
[m1-14b3]: ../decisions-and-roadmap.md#m1-14b3
[m1-14b4]: ../decisions-and-roadmap.md#m1-14b4
[m1-14b5]: ../decisions-and-roadmap.md#m1-14b5
[m1-14b6]: ../decisions-and-roadmap.md#m1-14b6
[m1-14b7]: ../decisions-and-roadmap.md#m1-14b7
[m1-14b8]: ../decisions-and-roadmap.md#m1-14b8
[m1-14c]: ../decisions-and-roadmap.md#m1-14c
[m1-14d]: ../decisions-and-roadmap.md#m1-14d
[m1-14h]: ../decisions-and-roadmap.md#m1-14h
[m2-3c7]: ../decisions-and-roadmap.md#m2-3c7
[adr-143]: 0143-the-operating-envelope-and-a-stop-rule-for.md
[adr-162]: 0162-the-suite-and-its-releases.md
[adr-236]: 0236-the-2026-10-11-review-supersonic-flights-from-published-comparisons.md
[finner-note]: ../research/finner-reference-data.md
[database-note]: ../research/duke-flight-database.md
[nesc-note]: ../research/nesc-check-cases.md
[wind-note]: ../research/wind-from-descent.md
[defects-note]: ../research/rocketpy-apogee-defects.md
[reference]: ../../validation/fixtures/aero/basic-finner-drev.json
[report]: ../../validation/fixtures/aero/basic-finner-normal-force.json
[design]: ../../validation/designs/range-basic-finner.json
[aero]: ../physics/aero.md#the-basic-finners-normal-force-against-drevs-range
[plan]: ../VALIDATION.md#the-references-named-for-mach-15-to-25
[i-619]: https://github.com/nrdptel/fusionspace-eridanus/issues/619
[i-859]: https://github.com/nrdptel/fusionspace-eridanus/issues/859
[i-1502]: https://github.com/nrdptel/fusionspace-eridanus/issues/1502
[i-1513]: https://github.com/nrdptel/fusionspace-eridanus/issues/1513
[i-1514]: https://github.com/nrdptel/fusionspace-eridanus/issues/1514
[report-b2]: ../../validation/fixtures/aero/basic-finner-drag-damping.json
[wedge]: ../../validation/fixtures/aero/basic-finner-wedge.json
[wedge-note]: ../research/basic-finner-wedge-drag.md
[aero-b2]: ../physics/aero.md#the-basic-finners-drag-and-pitch-damping-against-drevs-range

**Context.** M1.14b's *done when* is one line: each reference in [ADR-143][adr-143] §6b is
reported or refused. That section names nine. A survey of the tree on `main` at commit b42b63de
found where each stands.

- **The Rocket Flight Database.** [Its note][database-note] read the release and its history.
  [ADR-236][adr-236] already says its admission rule can't serve as a selection. Nothing
  records M1.14b's verdict on it.
- **The Basic Finner's free-flight data (DREV-TM-9703).** Its roll damping is scored, from
  readings cached outside the repository. Its normal force, center of pressure, drag and pitch
  damping are not compared with anything. The report is approved for public release, so its
  values may now be committed, typed in the project's own layout
  ([the note][finner-note]).
- **ARL-TN-1128 and the Mach 2.45 drag point.** ADR-143's correction of 2026-11-13 refuses
  both: the first holds no coefficients, and the second is doubted by its own authors.
- **NASA's six-degree-of-freedom check cases.** [Their note][nesc-note] sorted the 17
  atmospheric cases. HPR Sim has flown none, and the data files are not pinned.
- **ASME V&V 20's validation uncertainty.** No report gives a real flight's validation
  uncertainty. The standard has not been read.
- **Tip-off against RocketPy.** HPR Sim has no tip-off. RocketPy merged an opt-in tip-off phase
  in its pull request #920 ([the note on RocketPy's defects][defects-note]); the pinned
  RocketPy here is 1.13.0, which does not carry it.
- **Wind from the GPS descent.** [Its note][wind-note] gives the method and what bounds it.
  No code reconstructs a wind from a log.
- **MAPLEAF.** The program has not been installed or run.

**Decision.**

1. **Eight increments, in this order.** The Basic Finner goes first. Of the items not
   refused, it is the only one that holds measured coefficients inside Mach 1.5 to 2.5, the
   band M1.14b is named for, and [the supersonic increment][m1-14d] needs them. Its drag and
   pitch damping follow, from the same table. The check cases, tip-off, the wind, the
   validation uncertainty and MAPLEAF come after: none of them holds a measured coefficient
   in that band. They are M1.14b1 to M1.14b8.

2. **The increments.**
   - **M1.14b1 The Basic Finner's normal force against DREV's range.** *Done when:*
     DREV-TM-9703 Table IX's `C_Nα` and `C_mα` at every row inside the envelope are in a
     committed reference, typed with their probable errors, table and pages, and a committed
     design draws the model of the report's Fig. 2; a committed report sets HPR Sim's
     small-angle `C_Nα` and center of pressure beside each row's, the measured center of
     pressure with its probable error; a test that runs in CI recomputes every figure of the
     report from the committed design and reference, and pins on each row which way HPR Sim's
     center of pressure errs and by how much; a mutation that flips the sign of the
     center-of-pressure error fails that test; the accuracy page and the aerodynamics page give
     the errors by band, the rows from Mach 1.5 to 2.5 apart from those below and above, and say
     what the comparison leaves out; and the validation plan lists every item of ADR-143 §6b
     with its verdict or the increment that owns it.
   - **M1.14b2 The Basic Finner's drag and pitch damping against DREV's range.** *Done when:*
     the report's axial-force coefficient at zero angle of attack, `C_X0`, and its
     pitch-damping coefficient, `C_mq`, at every Mach number inside the envelope are in the
     committed reference with their probable errors, tables and pages; a committed report,
     recomputed by a test in CI, sets HPR Sim's drag coefficient at zero angle of attack beside
     each `C_X0`, with the single-wedge fins' pressure and base drag either modeled by a cited
     method or bounded by a cited figure that the report states beside the error; it sets
     `C_mq` beside HPR Sim's pitch damping expressed as the same coefficient about the same
     reference point, or a decision record says why it can't be expressed so, with the
     difference measured on a flown oscillation; a test pins which way each errs; and both
     pages give the errors by band.
   - **M1.14b3 NASA's check cases 1 and 2.** *Done when:* the check cases' data files and
     both volumes of NASA/TM-2015-218675 are pinned in `validation/refs.lock.toml`, with a row
     in the notices; a free body of a given mass and inertia, with no motor and no design,
     flies case 1 (the dragless sphere) and case 2 (the tumbling brick) from the report's
     initial conditions and unit factors; a committed report gives, for each case, HPR Sim's
     largest difference over the 30 s from each code's file, in altitude for case 1 and in
     each inertial body rate for case 2, beside the codes' own spread: the largest
     difference among the codes, with any one code left out, and named, whose leaving cuts
     that spread by more than half, its own difference given beside; any difference of HPR
     Sim's from a code that is kept which is larger than that spread is traced, in a decision
     record, to a named difference in the models whose size, computed apart from the flight,
     leaves no more than that spread unexplained; a check in the gate's `refs` step holds the
     report; and the accuracy page's
     rows for frames, gravity and rigid-body flight cite it.
   - **M1.14b4 NASA's check cases 3 and 6 to 10.** *Done when:* the same free body, with
     the report's constant roll, pitch and yaw damping coefficients applied by the test as
     moments, each on the report's reference area, the dynamic pressure and a reference length
     (the report gives a span and a chord but no formula, so the test states which length goes
     with which axis and how it makes the rates dimensionless), flies case 3, the tumbling brick with damping, and the report gives HPR Sim's largest difference in each inertial body rate from each code's file beside the codes'
     own spread; with a constant drag coefficient applied by the test along its velocity
     through the air, it flies cases 6 to 10: the sphere falling in still air, in a steady
     wind and in a wind that varies with height, and the two shots from sea level, and the
     report gives its largest difference in position, as the distance between the two
     positions, the same way; both are held and traced by M1.14b3's rules; a test holds the drag to act
     along the air-relative velocity at an angle of attack of 90°; and the accuracy page
     cites it.
   - **M1.14b5 Tip-off, measured with RocketPy's.** *Done when:* RocketPy at a pinned commit
     that carries its tip-off phase (its pull request #920) flies each public design built
     from RocketPy's examples twice, the phase on and off, in still air and in a 20 mph
     (8.9 m/s) crosswind; a committed report gives what the phase moves on each: the apogee,
     the landing point, and the attitude and pitch rate where the last guide leaves the rail;
     if it moves any apogee by more than 1%, or any landing point by more than 1% of the
     apogee, HPR Sim gains a tip-off model, with its equation and source, held by a test to
     RocketPy's on those designs; otherwise the flight page states the measured cost where it
     says tip-off is left out; and a check holds the report.
   - **M1.14b6 Wind from the GPS descent, against the reanalysis.** *Done when:* a documented
     reconstruction, the GPS ground velocity plus `w/g` times its rate of change with `w` the
     descent rate (Hock and Franklin 1999), is pinned by a test on synthetic descents at 5 m/s
     and at 25 m/s through a wind whose speed changes by 2 m/s per 100 m of height, recovering
     the wind within 0.3 m/s root-mean-square at both; a second reading, which takes the
     ground velocity as the wind, fails that line at 25 m/s; the reconstruction runs on every
     public flight whose log holds a GPS track of the descent, and a committed report gives
     the reconstructed wind less ERA5's by height band: the root-mean-square and the mean
     difference in speed, with each band's count of flights and samples, and lists each
     public flight it could not run on with the reason; with fewer than three flights
     compared, the validation plan's row says the reconstruction is a method, not yet a
     comparison; the page on wind says what a canopy's glide and swing leave unbounded; and a
     check holds the report.
   - **M1.14b7 A validation uncertainty for each real flight.** *Done when:* each public real
     flight's row in the real-flights report gives its apogee's comparison error beside a
     validation uncertainty, the root sum of squares of three stated parts: the numerical
     part from halving the time step, the input part from the flight's dispersion over its
     stated input uncertainties, and the data part from the log's own apogee uncertainty; the
     combination follows ASME V&V 20 as a cited source that was read gives it; the report
     counts the flights whose error lies within twice their validation uncertainty; a test
     pins the combination on a case worked by hand; and the accuracy page explains the three
     parts and what an error outside the band means.
   - **M1.14b8 MAPLEAF as a second open oracle.** *Done when:* MAPLEAF (MIT) at a pinned
     commit, run as an outside program, flies at least three of the public designs on the
     inputs it can share with HPR Sim, and a committed report gives the differences in apogee
     and in rail-exit speed, each input the two could not share listed beside them; or a
     decision record refuses it with what was measured: the failure to install or run at the
     pin, quoted, by its documented route and by one other, or the designs it could not fly,
     each with its reason.

3. **The verdicts that need no flight.** Each rests on a research note that read the source.
   - **The Rocket Flight Database: refused as a reference set.** Its admission rule keeps a
     flight only when an OpenRocket fork lands within ±10% of the measured apogee. On its
     historical flights that rule kept 3 and removed 10, and one more run failed. Its 25 rows
     from RASAero II's comparison page repeat that page's measured apogees to the foot, so on
     those flights it adds no apogee the page lacks. Its 4 flights from Mach 1.5 to 2.5 fly team or unlisted
     motors, outside the commercial-motor scope ([ADR-162][adr-162]). Its 3 historical
     flights fly sounding-rocket motors past Mach 4.9. Its 17 flights on commercial motors
     are the set that [the published supersonic flights][m2-3c7] take from the comparison
     page and RASAero II's installer, where they will be reported. The fork's own
     predictions are refused as an oracle: they are screened by the same rule
     ([the note][database-note], §3 to §6).
   - **ARL-TN-1128 and the Mach 2.45 drag point: refused,** as ADR-143's correction records
     ([the note][finner-note], §3 and §4).
   - **NASA's check cases 4, 5 and 11 to 17: refused.** One rule sorts the 17. A case is
     flown when what it checks is code HPR Sim ships: its frames, gravity, rotation,
     rigid-body equations, atmosphere and wind. The test gives the body: its mass, its
     inertia and its loads, which are constant damping coefficients in case 3 and a constant
     drag coefficient in cases 6 to 10. A case is refused when it needs an Earth or a
     vehicle that HPR Sim does not ship, or lies outside the envelope ([ADR-143][adr-143]).
     The note refused 11 to 17 and left 3, 4 and 5 waiting for a model. Case 3's damping law
     is the test's, and what it loads is the shipped rigid-body equations, so case 3 joins
     M1.14b4. Cases 4 and 5 are case 6's fall under another Earth: round, with
     inverse-square gravitation, fixed in case 4 and turning in case 5. HPR Sim ships no
     such Earth, so they would check a gravity law built for the test and no shipped code
     that cases 1 and 6 to 10 leave unchecked. Cases 1 and 6 to 10 fly on WGS-84, the Earth HPR
     Sim has; where case 1's field differs from HPR Sim's normal gravity, M1.14b3's rule
     has the difference traced. Cases 11 to 16 are an aircraft's, under flight control.
     Case 17 is a 314-tonne two-stage rocket to orbit, past Mach 9 in its files: outside the
     envelope ([the note][nesc-note], §2 and §6).
   - **References found since,** such as AGARD Model B ([#859][i-859]), are not §6b's items
     and stay issues.

4. **What M1.14b1 fixes for the later increments.**
   - **Every row inside the envelope is typed and scored,** not only the two inside Mach 1.5
     to 2.5. Table IX has eight rows. Six lie at or below Mach 3.5, from Mach 1.077 to 3.147;
     the other two, at Mach 3.734 and 4.300, are outside the envelope. The pages give the six
     in three groups: below Mach 1.5, Mach 1.5 to 2.5, and Mach 2.5 to 3.5. Leaving out the
     rows below Mach 1.5 would have hidden the slope's largest misses.
   - **The measured center of pressure is derived, not printed.** It is the moment reference,
     5.50 calibers from the nose tip, less `C_mα / C_Nα`. Its probable error is the ratio's,
     to first order, from the two printed probable errors, taken as independent: the report
     gives no correlation. The table prints them in whole percent, so one printed as 0% is
     taken as 0.5%.
   - **The measured slopes are straight-line fits.** The table holds both cubic terms,
     `C_Nα3` and `C_mα3`, at zero on the six rows, so each slope is fitted over the angles
     the models flew: a mean squared yaw of 0.7 to 1.9 square degrees. The comparison
     therefore gives HPR Sim twice: as the angle of attack goes to zero, the headline, and at each
     fit's root-mean-square yaw, where its body lift adds to the slope.
   - **The reference length is the body's diameter.** The report's nomenclature gives the
     reference area, the body's cross-section, and does not state the length. Taken as the
     diameter, the center of pressure comes out at 6.93 to 8.35 calibers on a body 10 calibers
     long, behind the center of gravity at 5.50, and where NOL's wind tunnel puts it, within
     that reading's error ([the note][finner-note], §2).
   - **No target.** Each error is reported beside the fit's probable error. The report does
     not define that term. In its usual sense it is 0.674 of a standard deviation.
   - **The fins take HPR Sim's square section.** The report draws its fins as single wedges, a
     section HPR Sim does not have. Its fin lift uses the planform alone, so the normal force
     and center of pressure compare. The drag does not, which is why M1.14b2 must model or
     bound the wedge.

**What doesn't change.** M1.14b's *done when* is whole across the eight. Each of §6b's nine
items is reported by an increment or refused: by decision 3, by ADR-143's correction, or, for
MAPLEAF, by the decision record M1.14b8 allows. M1.14b is met when all eight increments are.
No clause is narrowed and no tolerance is set or moved. The increments add what the one line
left out: which coefficients, which rows, the tests that hold each report, and the pages.
[The validation plan][plan] lists the nine with their verdicts and owners.

**What M1.14b1 found.** [The reference][reference] holds the six rows, [the design][design]
the model, and [the report][report] the comparison. `C_Nα` is per radian on the body's
cross-section; the center of pressure is in calibers from the nose tip; an error in the center
of pressure is HPR Sim's less DREV's, positive aft.

| Mach | DREV `C_Nα` (probable error) | HPR Sim | error | DREV center of pressure (probable error) | HPR Sim | error, calibers |
|---:|---:|---:|---:|---:|---:|---:|
| 1.077 | 18.47 (2%) | 13.86 | −24.9% | 8.35 (0.06) | 8.28 | −0.07 |
| 1.293 | 18.27 (3%) | 13.71 | −24.9% | 8.29 (0.08) | 8.26 | −0.03 |
| 1.832 | 11.78 (4%) | 10.46 | −11.2% | 7.87 (0.10) | 7.54 | −0.34 |
| 2.375 | 10.52 (2%) | 8.83 | −16.1% | 7.30 (0.04) | 7.12 | −0.18 |
| 2.718 | 9.01 (2%) | 8.17 | −9.3% | 7.19 (0.03) | 6.91 | −0.27 |
| 3.147 | 8.69 (1%) | 7.56 | −13.0% | 6.93 (0.02) | 6.70 | −0.23 |

HPR Sim's slope reads low on every row, by more than the fit's probable error. Its center of
pressure lies forward of the measured one on every row, by more than the probable error on all
but Mach 1.293. A center of pressure too far forward understates the static margin, so the
error is on the side that does not flatter. At each fit's root-mean-square yaw, 0.84° to 1.38°,
the slope's miss is 0.8 to 2.4 percentage points smaller and the center of pressure 0.02 to
0.04 calibers further forward, so both statements hold there too. The cause is not found
([#1502][i-1502]). The row at Mach 1.077 is an input to [the transonic increment][m1-14c],
the rows from Mach 1.293 to 2.375 to [the supersonic increment][m1-14d], and the two above
Mach 2.5 to [the extended band's][m1-14h]. Two issues track them: [#619][i-619] the two rows
below Mach 1.5, and [#1502][i-1502] the four above. [The aerodynamics page][aero] gives the table with
HPR Sim's slope split into its body's part and its fins'.

**What M1.14b2 found.** [The reference][reference] now holds each row's `C_X0` and `C_mq`
from Table IX, and each shot's own `C_X0` (Table VIII) and Reynolds number (Table V).
[The report][report-b2] sets HPR Sim's drag and pitch damping beside them, and
`hpr_aero::tests::basic_finner_drag_and_damping_against_drev` recomputes it. Three choices
were made without a source that settles them, each stated in the report and on
[the page][aero-b2]:

- **The wedge is modeled, both of its terms.** The design draws the fins square, whose blunt
  leading edge HPR Sim charges a stagnation pressure: as drawn the drag reads 34.5% to 138.1%
  high, which says nothing about the rest of the body. So the square edges' two terms come out,
  and the wedge's go in from cited sources ([the wedge note][wedge-note], [its figures][wedge]):
  the faces by the oblique shock (NACA Report 1135) where it stands attached, and by Vincenti
  and Wagoner's transonic value at Mach 1.077, where it stands off, an upper bound for a
  finite wing with nothing below it, so the band's low end takes the faces at zero; the trailing edges by
  two-dimensional turbulent base drag read from Hoerner's and Love's figures, a band. At Mach
  1.077 the two figures disagree, and the band spans both.
- **`C_mq` is per `q d/(2V)`.** The report states no denominator. Its eq. [1], with no spin,
  reduces to the damping rate of a swing with the rate made dimensionless by `q d/(2V)`, an
  inference.
  A free flight measures `C_mq + C_mα̇` together; HPR Sim has no `C_mα̇`, so its read-out
  ([`AeroModel::pitch_damping`][aero-b2], held to the flight engine's moment by a test in
  `hpr_sim`) is its whole damping.
- **The Reynolds number is the shots'.** Each row's drag is taken at the mean of its shots'
  Table V Reynolds numbers.

| Mach | DREV `C_X0` (shots) | HPR Sim `C_D0` (wedge band) | error | DREV `C_mq` (probable error) | HPR Sim | error |
|---:|---:|---:|---:|---:|---:|---:|
| 1.077 | 0.863 (0.854 to 0.868) | 0.950 (0.781 to 1.011) | +10.1% | −322.4 (5%) | −404.3 | +25.4% |
| 1.293 | 0.731 (0.701 to 0.760) | 0.706 (0.681 to 0.730) | −3.5% | −487.4 (6%) | −405.6 | −16.8% |
| 1.832 | 0.585 (0.567 to 0.598) | 0.488 (0.482 to 0.494) | −16.5% | −377.7 (4%) | −284.3 | −24.7% |
| 2.375 | 0.484 (0.473 to 0.500) | 0.384 (0.380 to 0.388) | −20.6% | −306.5 (4%) | −226.2 | −26.2% |
| 2.718 | 0.435 (0.400 to 0.452) | 0.342 (0.338 to 0.346) | −21.3% | −335.5 (3%) | −202.7 | −39.6% |
| 3.147 | 0.373 (0.350 to 0.385) | 0.301 (0.297 to 0.305) | −19.3% | −260.1 (3%) | −181.0 | −30.4% |

From Mach 1.832 up the drag is below every shot at both ends of the band, and below DREV's
even with the fins' bases at a vacuum, so the miss is not the wedge's base; its cause is not
found ([#1513][i-1513]). The trailing edges' two-dimensional band lies 1.3 to 3.1 times above
HPR Sim's own rule for a square fin's trailing edge, which may matter on real rockets
([#1514][i-1514]). The damping is less than measured on every row but Mach 1.077, by 2.8 to
13.2 times the fit's probable error: the side that does not flatter. At Mach 1.077 it is
25.4% more, five times the probable error, the side that does. The test pins each row's sign;
at Mach 1.077 the drag's band straddles DREV's, so there it pins the nominal value's.
The rows go to the same increments as M1.14b1's: Mach 1.077 to [the transonic][m1-14c], 1.293
to 2.375 to [the supersonic][m1-14d], and the two above 2.5 to [the extended band's][m1-14h],
whose *done when* re-measure the Basic Finner's rows in their bands: these rows are among them.

**Consequences.** The roadmap's M1.14b entry gains its eight increments and M1.14b1 is checked.
The *done when* of M1.14c, of M1.14d and of M1.14h each gain the Basic Finner's rows in its
band, to be re-measured.
The accuracy page gains a row, the aerodynamics page a section, and the validation plan the
list of §6b's nine. `cargo xtask aero` writes the report, `cargo xtask designs` the design, and
`hpr_aero::tests::basic_finner_normal_force_against_drev` recomputes both sides in CI.
