# What backs each claim

This page lists the gaps the docs state in their own evidence, what would settle each, and the
issue waiting for it.

The pages say plainly when a file reader, a model or a check rests on less than the real thing:

- a format read only from files invented for its tests;
- a value assumed because no sample states it;
- a model checked only against another program.

Each such statement is a row of the register, [`validation/evidence.toml`][register]. A page that
states a gap in one of the check's phrases links its row here, so the gap has a place to be closed. A row short of the
evidence it wants keeps an open issue labeled [`needs-sample`][label] until a real sample or
measurement settles it. [A decision record][adr] set this up.

> **How far to trust it.** This page is a ledger, not a grade. It counts rows by the kind of
> evidence behind them and never adds the kinds into one score. A calibration and a code to code
> comparison answer different questions, and neither stands in for a real flight. The check
> behind it, `cargo xtask evidence`, runs in every build. It checks that a gap names a row, not
> that it names the right one, and it can't see a gap worded otherwise.

The check fails a page that states a gap in one of its 23 phrases ([the list][phrases]) without
linking a row. It fails a row whose page or anchor is gone, and a row short of its wanted kind with
no open issue. It can't see a gap inside double quotation marks or a code block. It skips the pages
about the project's own process: the roadmap, the plan, the writing guide and the release steps. The
gaps it's known to miss, most of them the aerodynamics page's "no measurement", wait in
[#685][misses].

## The kinds of evidence

A row's *kind* says where its evidence comes from. The kinds follow the FDA's guidance on
computational models (2023, §VI.B) and NASA's modeling handbook's referent grades
(NASA-HDBK-7009B, Table 24), which [the decision record][adr] cites. They're categories, not a
scale, apart from one rule: `real` is what every row wants unless a decision record sets less. The
four kinds on [Accuracy](accuracy.md#four-kinds-of-evidence) are a different list: they say how a
model is checked.

| Kind | What it means | Example |
|---|---|---|
| real | A file or a measurement from the device or flight itself | A logger's own download of a flown rocket |
| sibling | A real file from a related device or firmware | Another model of the same maker's altimeter |
| code to code | Another program's answer to the same input | GPX values compared with gpxpy's |
| spec | Built from a published layout, schema or model, and checked on invented inputs | A reader written from a format's manual and tested on files made for its tests |
| calibration | Checked against the data it was fitted to, or seen while it was chosen | Flights compared with the simulation after its models were tuned beside them |
| reasoning | An argument only | A unit inferred from what a rocket can do |
| none | Nothing yet | A format no one has a file of |

The report below has a column only for a kind some row has: no row is `sibling` yet. A row also says
what its evidence doesn't cover (*not checked*). It lists each real sample with its SHA-256 hash
(a fingerprint of the file's bytes) and the test that reads it. It logs each search for better
evidence by date: each place it looked and what it found, so the next doesn't repeat it.

## Sending a sample

If you have a file a row asks for, attach it to the row's issue. Or say there where it can be
downloaded, and under what license. A sample is kept outside the repository, in the `refs/` folder
the checks fetch into. Its hash lets every check confirm it reads the same bytes.
[Checking a claim](checking-a-claim.md) shows how a pinned file is fetched and verified.

## The register

<!-- evidence: report; written by `cargo xtask evidence` from validation/evidence.toml; do not edit -->

The register holds 34 rows, as of October 29, 2026.

**By kind.** Each surface's rows, counted by the kind of evidence behind them now:

| Surface | Real · rows | Code to code · rows | Spec · rows | Calibration · rows | Reasoning · rows | None · rows | Rows |
|---|---:|---:|---:|---:|---:|---:|---:|
| Flight-log readers | 0 | 3 | 2 | 0 | 3 | 3 | 11 |
| The flight analyzer | 0 | 0 | 1 | 1 | 0 | 0 | 2 |
| Flight models | 0 | 0 | 1 | 0 | 3 | 1 | 5 |
| Aerodynamics | 0 | 2 | 2 | 0 | 2 | 0 | 6 |
| `.ork` import | 0 | 0 | 1 | 0 | 1 | 0 | 2 |
| Accuracy against real flights | 0 | 0 | 0 | 1 | 1 | 0 | 2 |
| Fin flutter | 0 | 0 | 1 | 0 | 0 | 0 | 1 |
| The motor finder | 1 | 0 | 0 | 0 | 0 | 0 | 1 |
| Design export | 0 | 1 | 0 | 0 | 0 | 0 | 1 |
| The synthetic flights | 0 | 0 | 0 | 0 | 1 | 0 | 1 |
| The smoother | 0 | 0 | 0 | 1 | 0 | 0 | 1 |
| Flight-log readings' 90% intervals | 0 | 0 | 0 | 1 | 0 | 0 | 1 |
| All | 1 | 6 | 8 | 4 | 11 | 4 | 34 |

**Short of what's wanted.** 33 of the 34 rows don't yet have the kind of evidence they want. Each has an open issue labeled `needs-sample`, linked in its row below.

**The rows.**

| Row | Surface | Claim | Evidence | Wanted | Issue | Last search |
|---|---|---|---|---|---|---|
| <a id="g1"></a>G1 | Flight-log readers | GPX tracks are read as the GPX 1.0 and 1.1 schemas define them, checked against gpxpy on three invented files. **Not checked:** what a real tracker or phone writes, and what its elevation (`ele`) is measured from. | code to code | real | [#654](https://github.com/nrdptel/fusionspace-eridanus/issues/654) | 2026-10-20 |
| <a id="g2"></a>G2 | Flight-log readers | Paths no sample log reaches are checked by the readers' tests alone: decimal commas, workbooks dated from 1904, a Raven `.FIPa` temperature below zero, an RTx fix at (0, 0). **Not checked:** a real file that takes any of these paths. | spec | real | [#655](https://github.com/nrdptel/fusionspace-eridanus/issues/655) | 2026-10-17 |
| <a id="g3"></a>G3 | Flight-log readers | AltOS radio telemetry is decoded from AltOS's documents for the packet types the samples hold. **Not checked:** the eleven packet types no sample holds, which are left out; the GPS ground speed, climb rate and course, which nothing checks. | spec | real | [#656](https://github.com/nrdptel/fusionspace-eridanus/issues/656) | 2026-10-17 |
| <a id="g4"></a>G4 | Flight-log readers | AltOS `.eeprom` logs are read in formats 1, 6, 7, 14, 15, 16 and 22, with the TeleMetrum v1's converter assumed. **Not checked:** that the TeleMetrum v1's converter measures against the pressure sensor's own supply, which no document states; the TeleMega v4's IMU, which its one sample never ran; 4 of the 14 sample files, with neither an export nor telemetry beside them; formats 5, 9 and 19, refused. | reasoning | real | [#657](https://github.com/nrdptel/fusionspace-eridanus/issues/657) | 2026-10-28 |
| <a id="g5"></a>G5 | Flight-log readers | ARTS2 raw data is timed at 50 records a second, the rate the maker's DataAnalyzer gives every file. **Not checked:** where the file stores its rate, if it does, and a file recorded at another rate. | code to code | real | [#658](https://github.com/nrdptel/fusionspace-eridanus/issues/658) | 2026-10-17 |
| <a id="g6"></a>G6 | Flight-log readers | Some layouts rest on one sample or on no export to compare with: the AltiMAX `.ALS`, the AIM `.xtra` and raw dump, 7 of 63 raw downloads, 5 of 6 RRC3 `.rff` files, and each case of an AltOS CSV re-saved by a spreadsheet. **Not checked:** a second flight of each layout read beside its maker's export; the raw downloads with no export beside them. | code to code | real | [#659](https://github.com/nrdptel/fusionspace-eridanus/issues/659) | 2026-10-17 |
| <a id="g7"></a>G7 | Flight-log readers | Featherweight summaries are checked on their maximum and pad altitude only. **Not checked:** the speeds, accelerations, times and the rest, converted from the unit the file writes and checked against nothing. | none | real | [#660](https://github.com/nrdptel/fusionspace-eridanus/issues/660) | 2026-10-17 |
| <a id="g8"></a>G8 | Flight-log readers | GPS heights are kept in the unit the file writes: a phone app's altitude as written, KML heights the least sure. **Not checked:** each height's unit and what it is measured from, beside a barometric log of the same flight. | reasoning | real | [#661](https://github.com/nrdptel/fusionspace-eridanus/issues/661) | 2026-10-17 |
| <a id="g9"></a>G9 | Flight-log readers | Formats with no sample on terms that let their bytes be tested (CATS's and the others listed) have no reader of their own. **Not checked:** the formats the list names, of which no sample can be tested on its terms. | none | real | [#662](https://github.com/nrdptel/fusionspace-eridanus/issues/662) | 2026-10-17 |
| <a id="g10"></a>G10 | The flight analyzer | Checked on an invented flight of known readings, one real log when fetched, and the synthetic flights the bounds were tuned on: 171 with a barometer logged 43 ways, 71 with an accelerometer 3 more, the 500 Hz IMU's 38 once more at ±8 g, which 28 of their tops reach; every reading within its bound (`cargo xtask synthetic`). **Not checked:** other real logs, with no truth beside them; flights outside the tuning set; loggers noisier than its six profiles; a high-g part held at its stated range, beyond every tuning flight; a quiet part's single-valued thrust plateau, read as flat, given only as a floor; accelerometer noise correlated between samples; a canopy's filling time (synthetic canopies open at once). | calibration | real | [#663](https://github.com/nrdptel/fusionspace-eridanus/issues/663) | 2026-10-17 |
| <a id="g11"></a>G11 | The flight analyzer | Reading your own logger by a stated mapping or layout is shown on one invented flight's logger and GPS tracker files. **Not checked:** a real logger's files read by a mapping or layout someone states. | spec | real | [#664](https://github.com/nrdptel/fusionspace-eridanus/issues/664) | 2026-10-17 |
| <a id="g12"></a>G12 | Flight models | Dryden gusts are generated as MIL-F-8785C gives them for aircraft, and no flight uses them. **Not checked:** any rocket flight, for which the model is unvalidated. | spec | real | [#665](https://github.com/nrdptel/fusionspace-eridanus/issues/665) | 2026-10-17 |
| <a id="g13"></a>G13 | Flight models | A dropped booster or middle stage is flown after the split as a point, a mass with only its recovery device's drag. **Not checked:** the whole flight of the dropped part, which is not validated at all. | none | real | [#666](https://github.com/nrdptel/fusionspace-eridanus/issues/666) | 2026-10-17 |
| <a id="g14"></a>G14 | Aerodynamics | Base drag with the motor burning takes the motor case off the base. **Not checked:** any measurement of base drag under power. | reasoning | real | [#667](https://github.com/nrdptel/fusionspace-eridanus/issues/667) | 2026-10-17 |
| <a id="g15"></a>G15 | Flight models | Thrust about a center of gravity off the axis turns the rocket in HPR Sim, and not in OpenRocket. **Not checked:** which of the two is right, which nothing measures. | reasoning | real | [#668](https://github.com/nrdptel/fusionspace-eridanus/issues/668) | 2026-10-17 |
| <a id="g16"></a>G16 | `.ork` import | A part's roll angle in an `.ork` file turns the way HPR Sim assumes. **Not checked:** which direction OpenRocket turns it. | reasoning | real | [#669](https://github.com/nrdptel/fusionspace-eridanus/issues/669) | 2026-10-17 |
| <a id="g17"></a>G17 | Flight models | The apogee printed for an unstable sustainer assumes it stays upright, and flights off a rod and off a rail differ. **Not checked:** an unstable sustainer's real flight; where the rod and the rail differ, not traced. | reasoning | real | [#670](https://github.com/nrdptel/fusionspace-eridanus/issues/670) | 2026-10-17 |
| <a id="g18"></a>G18 | `.ork` import | An inner tube's automatic outer radius is held to OpenRocket only on fourteen probe designs, and a ring's automatic bore rests on its unit tests. **Not checked:** OpenRocket's values for a real design's automatic radii, which the library's files never cache, and for a ring's automatic bore. | spec | real | [#671](https://github.com/nrdptel/fusionspace-eridanus/issues/671) | 2026-10-17 |
| <a id="g19"></a>G19 | Aerodynamics | RASAero II's exported tables are flown from one real export, to Mach 0.75. **Not checked:** a real export flown faster, checked by unit tests alone; past the export's last angle, and at 0° faster than Mach 1.3, the values are assumed. | spec | real | [#672](https://github.com/nrdptel/fusionspace-eridanus/issues/672) | 2026-10-17 |
| <a id="g20"></a>G20 | Flight models | A pushed part leaves through the end the simulator assumes, from the way its body points as it falls. **Not checked:** a measured deployment; the directions are assumptions, not measurements, and for a drogue between the booster and the avionics bay a guess. | reasoning | real | [#673](https://github.com/nrdptel/fusionspace-eridanus/issues/673) | 2026-10-17 |
| <a id="g21"></a>G21 | Accuracy against real flights | Four of the seven public altimeters are taken as barometric, and the designs' fin edges are guesses. **Not checked:** each of the four altimeters' sensor, none of them calibrated; each design's fin edges. | reasoning | real | [#674](https://github.com/nrdptel/fusionspace-eridanus/issues/674) | 2026-10-17 |
| <a id="g22"></a>G22 | Fin flutter | Martin's flutter criterion (NACA TN 4197) places a fin against the line his missile and wind-tunnel wings drew. **Not checked:** a hobby rocket's flutter, against which nothing here is checked; that the result errs on the safe side, which nothing shows; a fin glued to a tube, less stiff at its root than Martin's clamped wings. | spec | real | [#675](https://github.com/nrdptel/fusionspace-eridanus/issues/675) | 2026-10-17 |
| <a id="g23"></a>G23 | The motor finder | The motor finder is replayed on ThrustCurve.org's own searches of three makers, recorded unchanged on October 29, 2026, each record the same, field for field, as in ThrustCurve.org's raw answer of October 28, 2026, and each solid motor's as in the snapshot `hpr-motor` compiles from it. **Not checked:** the tests of fetching one motor on demand, which replay invented answers for cases the real records don't hold; and the in-stock list, recorded on October 1, four weeks before the searches. | real | real | none | 2026-10-29 |
| <a id="g24"></a>G24 | Accuracy against real flights | The simulation's apogees miss seven public real flights by 6.06% on average and 53 private ones by 12.95%, both outside the 5% target. **Not checked:** a flight held out from every choice of model and every fit, predicted before its log is read. | calibration | real | [#676](https://github.com/nrdptel/fusionspace-eridanus/issues/676) | 2026-10-17 |
| <a id="g25"></a>G25 | Design export | The `.ork` files `hpr convert` writes are checked by opening them in OpenRocket. **Not checked:** whether RockSim opens them. | code to code | real | [#677](https://github.com/nrdptel/fusionspace-eridanus/issues/677) | 2026-10-17 |
| <a id="g26"></a>G26 | Aerodynamics | Drag is compared with the RASAero II curves of RocketPy's example rockets, of which Calisto's is the one real RASAero II export. **Not checked:** a measured drag of any of them; the other curves' rockets, which record no fins or surface finish, so HPR Sim flies stated guesses for them; and the comparison misses past Mach 1.6, where against Calisto's export HPR Sim is within 10% at 8 of the 17 rows from Mach 1.2 to 2.0. | code to code | real | [#678](https://github.com/nrdptel/fusionspace-eridanus/issues/678) | 2026-10-17 |
| <a id="g27"></a>G27 | Aerodynamics | Tube fins' normal force and center of pressure are compared with OpenRocket 24.12's on 14 probe designs and its tube fin rocket. **Not checked:** a measured tube fin's lift or center, of which none was found; which code is right, as OpenRocket's tubes lift 1.26 to 1.86 times HPR Sim's at every Mach number; the interference between tubes, an unchecked estimate. | code to code | real | [#679](https://github.com/nrdptel/fusionspace-eridanus/issues/679) | 2026-10-17 |
| <a id="g28"></a>G28 | Aerodynamics | A flared body's supersonic normal force rests on one measured flare, and a flare past its corner's limit flies a rule checked against nothing. **Not checked:** a second measured flare; any flare past its corner's limit; the attachment test, a wedge's limit rather than a flare's own; a near-flat flare, read by the older generalized method. | reasoning | real | [#680](https://github.com/nrdptel/fusionspace-eridanus/issues/680) | 2026-10-17 |
| <a id="g29"></a>G29 | Aerodynamics | Faster than sound, a boattail's share of the body's force comes from the shock-expansion method, and a blunt tip flies it behind a Newtonian cap. **Not checked:** the boattail's share, unvalidated; a blunt tip, checked on a sphere-cone only. | spec | real | [#681](https://github.com/nrdptel/fusionspace-eridanus/issues/681) | 2026-10-17 |
| <a id="g30"></a>G30 | Flight-log readers | The AltiMAX's Excel export keeps its voltages as the file states them. **Not checked:** the voltages, which nothing checks. | none | real | [#682](https://github.com/nrdptel/fusionspace-eridanus/issues/682) | 2026-10-17 |
| <a id="g31"></a>G31 | Flight-log readers | An RRC3's Temperature column that states no unit is kept as written; its Voltages column that states none is read in volts. **Not checked:** the voltage's unit, which is assumed; a file whose voltage can be checked against another record of the same flight. | reasoning | real | [#683](https://github.com/nrdptel/fusionspace-eridanus/issues/683) | 2026-10-20 |
| <a id="g32"></a>G32 | The synthetic flights | The synthetic loggers' noise follows the makers' datasheets and the noise measured on a few public logs, each drawn term tested on its own; the accelerometer's residual bias, the GNSS loop's range, damping and noise, the dropouts, the port's place, the Bosch-class part on two profiles, PerfectFlite's offset and one bench sensor's slow noise on every barometer are assumed, and the MS5607's error read as a gain is inferred. **Not checked:** a synthetic log as a whole against real logs of the same family: the pad's noise and its correlation, and the ejection pulses. | reasoning | real | [#725](https://github.com/nrdptel/fusionspace-eridanus/issues/725) | 2026-10-17 |
| <a id="g33"></a>G33 | The smoother | At burnout, the top speed, apogee and a drawn moment, the smoother's 90% intervals on a log's height and vertical speed hold between 94.3% and 99.4% of the synthetic flights' truths in the three sets' cells, and two logger cells 89.2% and 89.7%, on the 171, 820 and 845 flights smoothed of three sets of 200, 1,000 and 1,000, its rules set with those sets in view. **Not checked:** a real flight's smoothed height and speed against a truth taken apart from the log, such as a second logger's or a ground tracker's record of the same flight. | calibration | real | [#981](https://github.com/nrdptel/fusionspace-eridanus/issues/981) | none |
| <a id="g34"></a>G34 | Flight-log readings' 90% intervals | Seven readings of the height track state 90% intervals fitted on 2,000 synthetic flights of their own. On sets A and B each passes the two-sided coverage test (the top speed below its transonic mark, on 287 and 286 readings), and the intervals narrowed, widened and shifted down each fail it. The misses aren't spread evenly: the height's, the top speed's and the descent rate's gather on days more than 10 K from standard, a warm day's above the interval (#1055, #1061), and the times lean by logger (#1056, #1062). **Not checked:** a real flight's readings against a truth taken apart from the log, such as a second logger's or a ground tracker's record of the same flight. | calibration | real | [#1052](https://github.com/nrdptel/fusionspace-eridanus/issues/1052) | none |

**Due a search.** A row is due one when it has none yet, or its last was more than 30 days before October 29, 2026:

- [G33](#g33)
- [G34](#g34)

**Searches so far.**

- [G1](#g1), October 17, 2026, looked at:
  - the download pages and documents of Featherweight, AltOS, Eggfinder, Missile Works and BigRedBee
  - public repositories of flight logs

  It found no openly downloadable GPX of a real rocket flight. The dedicated trackers write KML, CSV or raw NMEA. GPX comes from phone apps, handheld receivers and mapping tools.
- [G1](#g1), October 20, 2026, looked at:
  - Rocketry Forum thread 135349's attachments, fetched from the forum's attachment links, their GPX files counted for `geoidheight`
  - GitHub code search for GPX files with `geoidheight`, with and without rocketry words
  - OpenStreetMap traces tagged rocket
  - Zenodo, figshare, Kaggle and other data sites
  - phone-app posts (the survey in docs/research/public-logs-evidence-gaps.md)

  It found a source to cite only. Thread 135349 (2016, no terms stated, cached under refs/samples-search/G1) holds four GPX files of an Eggfinder's track of two flights. None has `geoidheight`. Beside them is the receiver's raw NMEA, whose GGA sentences state the geoid separation. GitHub's GPX files with `geoidheight` are everyday phone tracks (#645).
- [G2](#g2), October 17, 2026, looked at:
  - GitHub code search for `.FIPa` and `.rtx` files and semicolon-separated altimeter exports (mDACS, Eggtimer and Blue Raven headers split by `;`)
  - the Harvey Mudd College advanced rocketry course's flight data page (its 11 workbooks fetched)
  - a University of Minnesota course directory (one `.FIPa`)
  - the 1904 date flag, counted in every workbook of the private collection

  It found nothing usable: the one public decimal-comma export found is a SpaceTeam Aachen RRC3 export (2021) in the GPL-3.0 repositories M0WW0M/European-FlightData and catsystems/euroc21-team-data. Its European-FlightData copy is byte for byte a file the private collection already holds. None of 126 private workbooks (copies included) and 11 public course workbooks uses the 1904 date system. No public `.rtx` was found; the Minnesota `.FIPa` (no license) records a June flight.
- [G3](#g3), October 17, 2026, looked at:
  - GitHub code search for `.telem` files: 16 rocketry files in 9 repositories
  - 15 of them downloaded, each line's packet type counted against the AltOS Telemetry document's table
  - each repository's license
  - PSAS's launch repositories

  It found a partial answer: real packets of 3 of the 11 undecoded types under MIT, but no flight and no export beside them. A TeleMega's pad recording is in lunng/HART_CS_2020 (MIT): types 0x08 and 0x09, 538 of each. UCC-Rocket-and-Space-Exploration/CosmoSoft (MIT) holds a TeleMini v3's pad packets (type 0x11), beside an AltosUI CSV of a later flight that a spreadsheet re-saved. Flights with types 0x08, 0x09 and 0x14 exist with no license (Maeswoon/UNHSEDS, with AltosUI's telemetry CSV beside it; AIAANortheastern/KarmanSims; Mines-Rocketry-Club/JEFF-modular-rocket-system). No export found has a GPS ground speed, climb rate or course column.
- [G4](#g4), October 17, 2026, looked at:
  - the JSON headers of 17 public `.eeprom` files from GitHub code search (`log_format` in `.eeprom`)
  - PSAS's launch repositories (psas/flight_data-2010.06.27, Launch-8, Launch-10 to Launch-14)
  - Mines-Rocketry-Club's flight data

  It found a partial answer, to cite only: PSAS flew a TeleMetrum v1.0 (serial 221) from 2010 to 2013. Its October 17, 2010 and June 30, 2013 files are already in the private collection. A third, in psas/flight_data-2010.06.27 (no license), is an older text-form `.eeprom` beside a table of time, height, speed and acceleration not in AltosUI's CSV layout. No TeleMega v4 file whose IMU ran, and no new file in log format 5, 9 or 19, was found. The other public files are TeleMetrum v2 to v4 (format 7), EasyMini (6 and 14) and TeleMega v6 (22).
- [G4](#g4), October 28, 2026, looked at:
  - GitHub code search for `log_format` in `.eeprom` files, every header of the 44 hits parsed
  - the names `TeleMetrum-v1` and `TeleMega-v4` across GitHub

  It found a lead for the TeleMetrum v1's converter: ISSUIUC/flight-data holds a TeleMetrum v1.2 `.eeprom` in log format 1 with AltosUI's export beside it (no license; cache under refs/ only). No TeleMega v4 file whose IMU ran. Format 5, 9 and 19 samples were found earlier and are listed in the CATS and Juno sample note (docs/research/sample-logs-cats-juno.md); the October 17 search's none was too narrow.
- [G5](#g5), October 17, 2026, looked at:
  - the ARTS2 User Manual V1.0.0 (pp. 7, 9, 16)
  - PSAS's launch repositories for ARTS2 flights beyond the samples (May 31, 2009; June 27, 2010; and October 17, 2010)
  - PSAS's ARTS-2 checklist page
  - web searches for DataAnalyzer documents

  It found a partial answer: the manual says the recording rate is set per flight profile in DataAnalyzer (p. 7). The memory fills in 82 s to 26 min, depending on the rate (p. 9). The sensors are read at 200 Hz whatever rate is recorded (p. 16), so files at other rates can exist. PSAS's 2009 and June 27, 2010 ARTS2 tables (no license) each hold 16,384 or 16,385 records 0.020 s apart. At 200 a second, 16,384 records last 82 s. By inference, the memory holds 16,384 records, and the rates run from about 10 to 200 a second. PSAS's checklist sets 50 Hz in both profiles. No file at another rate, and no document of where the `.odf` keeps the rate, was found. Each sample flight also carried a TeleMetrum, whose liftoff and deployment times could time the ARTS2's records.
- [G6](#g6), October 17, 2026, looked at:
  - GitHub code search for mDACS's export header
  - listings of team flight data repositories
  - the Harvey Mudd College advanced rocketry course page and its dofpro.org mirror, for AIM XTRA files
  - M0WW0M/European-FlightData, for AltiMAX files

  It found 7 RRC3 `.rff` files in RIT-Launch-Initiative/flight-data (MIT, commit ffa9f44), saved by mDACS 1.61 from 3 launches in 2023 and 2024. 6 have mDACS's CSV export beside them. 4 of the exports have units typed into their headers by hand, as the team's procedure says. The team's notes record avionics-bay failures in flight on two vehicles. The private collection holds 5 of the exports and none of the `.rff`; all are cached under refs/samples-search/G6/rit-flight-data. AIM `.xtra`: the course page lists 15 `.xtra` files of 2017 beside AIM CSV exports. Every `.xtra` link returned HTTP 404 and the mirror's one link is an empty file; only the CSVs download. AltiMAX: the only public `.ALS` found is the sample already held. A second AltiMAX flight in the same GPL-3.0 repository has its Excel export only.
- [G7](#g7), October 17, 2026, looked at:
  - GitHub code search for Featherweight's summary header (`Rocket Name`, `Launch time UTC`) in CSV files

  It found 16 summaries in 10 repositories, 3 of them under MIT. RIT-Launch-Initiative/flight-data (MIT, commit ffa9f44) holds two Blue Raven flights, each with its summary, low-rate and high-rate CSVs. It also holds a Featherweight GPS tracker's summary beside its track; RMIT-Hive-Rocketry/GCS (MIT) a Blue Raven summary with both rate CSVs (June 20, 2026). Neither is in the private collection. Lmx154/RL-RocketPy-Notebook (MIT) has a summary and low-rate pair the private collection already holds. The RIT files are cached under refs/samples-search/G7/rit-flight-data.
- [G8](#g8), October 17, 2026, looked at:
  - GitHub code search for Featherweight GPS tracker CSVs (`UTCTIME,UNIXTIME,ALT`, 12 files in 7 repositories) with KML or a barometric log beside them
  - the Harvey Mudd College course page's AIM KML files

  It found two MIT flights each with a tracker file (RIT's a summary CSV, Lmx154's a track), a KML and a Blue Raven barometric log. Their KMLs are `altitudeMode` absolute. One is RIT-Launch-Initiative/flight-data's second airbrake testbed flight (the app's and the desktop's KML). Its two files disagree on the date (April 7, 2026 by the Blue Raven's clock, May 16, 2026 by the tracker's UTC): their pairing has to be shown first. The other is Lmx154/RL-RocketPy-Notebook's Lonestar 2026 flight, whose CSVs the private collection already holds and whose KML is new. Peaks read 3.6% and 3.8% higher on the tracker than on the Blue Raven. The course page's KML links return HTTP 404. Both are cached under refs/samples-search/G8.
- [G9](#g9), October 17, 2026, looked at:
  - GitHub code search for the Eggtimer ION header, Blue Jay, MicroPeak, EasyMotor, TeleMini, EasyTimer and AltimeterThree files
  - GitHub code search for CATS's `.cfl` and its exports' column names
  - GitHub repository search for flight data collections
  - M0WW0M/European-FlightData

  It found nothing usable for the listed formats. The CATS hits are the GPL-3.0 catsystems/cats-logs and euroc21 files already known. The only new logger files are two Eggtimer Quantum CSVs (lower-case header, GPL-3.0, Skyward's 2022 flight), a device the list doesn't name.
- [G10](#g10), October 17, 2026, looked at:
  - team flight data repositories on GitHub with a license and a logger's own figures beside its log: RIT-Launch-Initiative/flight-data, RMIT-Hive-Rocketry/GCS, Lmx154/RL-RocketPy-Notebook, AerospaceTeamGraz/RocketFlightData, ISSUIUC/flight-data, curtscraw/osu_rocketry_data

  It found these, under MIT: RIT-Launch-Initiative/flight-data's 6 RRC3 exports beside their `.rff` files, whose summary statistics the team's note describes. RIT's two Blue Raven flights' summaries state the peak, the first burnout's time and each channel's firing time. RMIT-Hive-Rocketry/GCS has a Blue Raven flight with its summary. None is in the private collection but 5 of the RRC3 exports. The other repositories have no license, except Lmx154/RL-RocketPy-Notebook (MIT), whose files the private collection already holds.
- [G11](#g11), October 17, 2026, looked at:
  - team repositories whose own flight computer's files come with a stated layout and another record of the same flight: RIT-Launch-Initiative/flight-data, Lmx154/RL-RocketPy-Notebook, curtscraw/osu_rocketry_data, ISSUIUC/flight-data

  It found RIT-Launch-Initiative/flight-data (MIT, commit ffa9f44): the team's airbrake controller writes `data.csv` with units in each column name (`accel_x__m_s2`), a `params.csv` and a raw capture. The airbrake-test README states the files' columns, units and meanings. On the two testbed flights a Blue Raven flew beside it. None of these files is in the private collection; they're cached under refs/samples-search/G11/rit-flight-data. Lmx154's flight computer log with unit-named columns and alignment notes is already held privately; curtscraw/osu_rocketry_data states its units in `data_units.txt` but has no license.
- [G12](#g12), October 17, 2026, looked at:
  - web searches for measured against predicted dispersion of small and sounding rockets, with the wind measured
  - web searches for Dryden or other turbulence models checked against rocket flights
  - the team repositories found for the rows above, for wind records beside their logs

  It found nothing usable: the dispersion work found is simulation or wind-weighting studies. No team repository found holds a measured wind profile beside its flight logs.
- [G13](#g13), October 17, 2026, looked at:
  - GitHub repository and code search for staged flights with the booster's own log
  - ISSUIUC/flight-data
  - Minnesota Space Grant's Midwest Rocketry Competition reports, whose rockets drop a booster after burnout

  It found a partial answer, to cite only: ISSUIUC/flight-data (no license) holds booster logs of two two-stage flights with the sustainer's beside them. On November 18, 2023 they're two StratoLoggers; on March 23, 2024, a TeleMega v6 `.eeprom` with a CSV export beside it. There are also files named for a booster alone (October 1, 2023). Every booster file checked is already in the private collection, and no design file was found. The competition's reports describe booster descents but link no log.
- [G14](#g14), October 17, 2026, looked at:
  - Craven, College of Aeronautics Report 129 (1960): its record and full text at Cranfield
  - NACA free-flight reports on base pressure with rocket motors burning, by web search
  - measurements of base pressure on hobby rockets in flight

  It found nothing beyond the October 17, 2026 power-on base drag note. Craven's record describes a subsonic stream with a supersonic jet at a jet-to-base diameter ratio of 0.1875. There, slight under-expansion raises base pressure and higher stagnation pressure lowers it again. Its full text returned HTTP 403. No measurement on a hobby rocket under power was found.
- [G15](#g15), October 17, 2026, looked at:
  - web searches for flight tests of rockets with a measured lateral center-of-gravity offset or mass asymmetry and their drift under thrust

  It found nothing usable: only simulation and dispersion analyses, none a logged flight with a known offset.
- [G16](#g16), October 17, 2026, looked at:
  - OpenRocket's user guide (basic rocket design page) and wiki
  - openrocket/openrocket issues found by searching for angle, rotation, rail button and mirror (#940, #1548, #1640)

  It found a partial answer: #940 puts 0 degrees at the top of the rocket and #1548 discusses pods' angles with rear-view pictures. No source states which way the angle turns. The OpenRocket 24.12 jar, run on a probe with one part turned by a known angle, remains the way to settle it. This search didn't run it.
- [G17](#g17), October 17, 2026, looked at:
  - web searches for logged two-stage flights whose sustainer went unstable after the split
  - The Rocketry Forum staging threads
  - Spaceport Rocketry's staging guide
  - a 2018 UTN Haedo stage-separation paper (rtyc.utn.edu.ar, article 101)
  - the private collection's design and log catalog, counted only

  It found nothing usable: the forum threads found describe sustainers that didn't light. The UTN paper is a wind-tunnel study of a first stage, CC BY-NC 4.0, with no flight. The private collection holds 148 flights of designs with two or more stages that carry a log. None is yet screened for a sustainer unstable after the split. No flights of one design off both a rod and a rail were found.
- [G18](#g18), October 17, 2026, looked at:
  - the private collection's 714 .ork files (copies included), opened and counted for inner tubes and centering rings set to `auto`

  It found that no file holds OpenRocket's computed radii, as the issue says, but the designs exist. 43 files have an inner tube with an automatic outer radius (43 tubes), 446 have rings with an automatic bore (1,179 rings), 35 have both. Reading OpenRocket's own values needs its jar run on them as an oracle. A GitHub code search is no help, as .ork files are zipped.
- [G19](#g19), October 17, 2026, looked at:
  - GitHub code search for .CDX1 design files and RASAero II's export header ("CD Power-Off" with "CN Potential", "CA Power-Off", extension:csv)
  - the repositories found: anbanpillay/ASRI-PyROPS, rkuhlf/rocket-simulation, activecontrols/Propulsion, stanford-ssi/rockets-irec, spaceconcordia/rocketry-performance, AIAANortheastern/KarmanSims
  - the private collection's RASAero designs, counted only

  It found a partial answer: anbanpillay/ASRI-PyROPS@76cd2a46 holds a RASAero II CSV export (rocketpy_migration/data/aerodynamics/rasaero_data.csv) to Mach 5 at 0°, 2° and 4°. Its workbooks Inputs/RASAeroII15 Detailed.xlsx and RASAeroII15.xlsx hold RASAero II output at 0° to 15°, 20° and 25° at Mach 0.1, 0.5, 1.1, 2 and 5. These would check the values assumed past 4° and at 0° faster than Mach 1.3. No license, so cite only; no flight log with them. The other exports found hold 0° only. The private collection has 76 RASAero design files (copies included) and no exports. 12 of their flights carry a log, a few of them past about 255 m/s (837 ft/s) by a plausible logged speed. So RASAero II exports of them would meet the row.
- [G20](#g20), October 17, 2026, looked at:
  - web searches for measured deployments: onboard video or accelerometer traces of ejection, nose cone and drogue separation
  - student post-flight reports (Stanford SSI wiki, NAU capstone)
  - Rocket Reviews ejection tests
  - the private collection's logs, counted only

  It found nothing usable: the reports describe outcomes, not which end each part left through. The private collection has 227 logs (copies included) on 94 flights with tilt, gyroscope or orientation channels (Blue Raven, AltOS and team loggers). They show how a body pointed at deployment, not which end a part left through, and there's no video.
- [G21](#g21), October 17, 2026, looked at:
  - CATS's EuRoC 2023 team data (github.com/catsystems/euroc23-team-data, GPL-3.0) and the CATS Vega paper (arXiv 2511.04725, ESA symposium 2024), for Cavour, Genesis and Lince
  - EPFL Rocket Team's repositories (EPFLRocketTeam/BellaLui, MorgesHAB/ERT2020GS) and RocketPy's Bella Lui data, for Bella Lui

  It found a partial answer: the Vega paper (§2.4, p. 3) says its one-axis Kalman filter trusts the IMU most during the burn. As the speed falls in the coast, the filter weights the barometer more, so a Vega height near apogee rests mostly on its barometer. The euroc23 ground-station CSVs give only altitude and velocity, and the raw .cfl logs' layout is published only in GPL code. EPFLRocketTeam/BellaLui (no license, cite only) has driver folders for a BME280 barometer and a BNO055 IMU on the 2020 flight computer. Nothing says which made the logged height; no fin-edge drawings were found for any of the four.
- [G22](#g22), October 17, 2026, looked at:
  - web searches for measured flutter of hobby-size fins (wind tunnel, ground test, logged failure)
  - Cal Poly's 2013 Fin Flutter Analysis (digitalcommons.calpoly.edu/aerosp/109)
  - a 2008 WPI project report
  - Dinulović et al., FME Transactions 50(3) 2022 and Aerospace 11(1) 2024 (doi 10.3390/aerospace11010026)
  - the same group's Materials 2025 test of 3D-printed PLA plates, cited in the hardware line's note

  It found a candidate: Dinulović, Rašuo, Slavković and Zajić, 'Flutter Analysis of Tapered Composite Fins: Analysis and Experiment', FME Transactions 50 (2022) 576 to 585. In its Table 4, p. 583, five clamped e-glass fins fluttered at 19.6 to 21.5 m/s (64 to 71 ft/s) in a wind tunnel. They're 0.65 mm (0.026 in) thick, 180 mm (7.1 in) root, 90 mm (3.5 in) tip, 262 mm (10.3 in) span. The paper evaluates TN 4197's speed on thickness over the mean chord and an effective shear modulus of 7.96 GPa. That gives 15.3 m/s (50 ft/s), 21.9% to 28.8% below. HPR Sim's reading, thickness over the root chord, gives about 10.7 m/s (35 ft/s) at sea level on that modulus, 45% to 50% below. Both are on the safe side for the speed, on that one fin, which HPR Sim's deciding check (Martin's figure 3) doesn't score yet. The paper is copyright the Faculty of Mechanical Engineering, Belgrade, all rights reserved, so cite only. Its fins are clamped plates at low speed, not fins glued to a tube. The Materials 2025 plates, which fluttered at about 10 to 12 m/s (33 to 39 ft/s), are the same group's, so not an independent test. The Cal Poly report found no measured flutter speed, and the WPI report says flutter wasn't tested. The 2024 Aerospace article (CC BY) couldn't be reached (HTTP 403).
- [G23](#g23), October 20, 2026, looked at:
  - ThrustCurve.org's pages, for a license on its motor records: the site's footer, its API and contribute pages
  - the site's own source repository, JohnCoker/thrustcurve3, with its API client
  - GitHub's other copies of the site's data

  It found no license for the motor records (the home page reserves all rights; the API page states none). A lead: JohnCoker/thrustcurve3 (ISC, commit 3325878d, cached under refs/samples-search/G23) holds a 2016 sample dump of 280 real motor records and 411 curve files. The records have the fields the stand-ins leave out, and each curve file its own class (104 public domain, 16 free, 291 unmarked). ISC grants rights in 'this software', so using the records is a licensing question.
- [G23](#g23), October 28, 2026, looked at:
  - ThrustCurve.org's contribute, API and privacy pages and its full-text site summary
  - openrocket/motor-database and JohnCoker/TCTG

  It found no openly licensed motor database. The only license statement is the contribute page's class for each file, which the API doesn't return; openrocket/motor-database (GPL-3.0) points other projects to the API.
- [G23](#g23), October 29, 2026, looked at:
  - ThrustCurve.org's API: the searches of AeroTech, Cesaroni Technology and Loki Research the tests replay

  It found the three answers, of 307, 296 and 60 records, now committed unchanged as published facts with ThrustCurve.org's credit. Each solid motor's record is the same, field for field, as in the October 28 snapshot `hpr-motor` ships; AeroTech's seven hybrids are not in that snapshot, which holds solid motors only.
- [G24](#g24), October 17, 2026, looked at:
  - public pairs of a design and a log not yet used
  - CATS's EuRoC 2021 team data (github.com/catsystems/euroc21-team-data, GPL-3.0)
  - GitHub repository and code searches for EuRoC designs (extension:ork)
  - the private collection's catalog, counted only

  It found a partial answer: three loggers' records of one 2021 flight in euroc21-team-data@ef54912c (STA-CARL2: AltiMAX, RRC3, Eggtimer). No design file was found for it. The private collection pairs a design with a parsing log on 339 flights, 53 of them compared so far. So the samples a held-out set needs exist. RIT-Launch-Initiative/flight-data (MIT) also holds public pairs: `rocket.ork` designs beside their Featherweight flight logs and an OpenRocket simulation export (2026/AirbrakesL1Tests). What's missing is the held-out protocol: a set fixed and predictions written before its logs are read.
- [G25](#g25), October 17, 2026, looked at:
  - Apogee Components' RockSim information page
  - OpenRocket's documentation and wiki on importing and exporting
  - The Rocketry Forum threads on RockSim and OpenRocket files

  It found nothing usable: no source says whether RockSim opens .ork files. OpenRocket's documents describe only OpenRocket writing RockSim's .rkt files. The maker's page offers RockSim 11 for Mac and Windows with a 30-day trial, so the check needs RockSim itself run on files `hpr convert` wrote.
- [G26](#g26), October 17, 2026, looked at:
  - GitHub code searches for RASAero design files of RocketPy's example rockets (Valetudo, Calisto, Juno III, extension:CDX1)
  - RocketPy's data/rockets folder
  - RocketPy-Team/RocketPaper (MIT)

  It found a partial answer: no RASAero design file or measured drag curve of any of them was found. RocketPaper@23d9271d holds Valetudo's real RRC3 log (Valetudo_RRC3/RRC3_original.csv, MIT, with a note that some values make no sense). Valetudo's RASAero II curve is one of those compared, and a drag could be inferred from its coast once its mass and motor are settled.
- [G27](#g27), October 17, 2026, looked at:
  - web searches for wind-tunnel or flight measurements of tube fins' normal force and center of pressure (NARAM R&D reports, DTIC, NTRS)
  - PSAS's fin design references
  - the private collection's .ork files, counted only

  It found no wind-tunnel or flight measurement of a tube fin's lift or center. The private collection has 8 .ork files (copies included) with a tube fin set. 1 of their flights has a parsing log, which can check an apogee but not the lift or center.
- [G28](#g28), October 17, 2026, looked at:
  - NTRS, for reports on flared bodies
  - NASA TM X-653 (Jorgensen, Spahr and Hill, 1962)
  - NASA TM X-30 (Kirk and Chapman, 1959)
  - NACA RM A57I30 (Dennis, 1957)
  - NACA RM L57J29 (Kehlet, 1958)
  - the Redstone Arsenal reports TM X-30 draws on

  It found candidates, all US government work, public use permitted. TM X-653 measured normal-force slope and center of pressure of sharp and blunted cone-cylinders with 10° and 20° conical flares. Its range is Mach 0.60 to 5.82 (figs. 4 and 6). Its 20° flare separated at Mach 1.20 and 1.37 (fig. 7), below the speed where a 20° wedge's shock attaches. So it covers a flare past its corner's limit. TM X-30 gives flare-alone normal-force slopes and centers for 80 cone-cylinder-flare bodies with flares of 5° to 20° at Mach 2.18, 2.81 and 4.04. It sets them beside the second-order shock-expansion method. RM A57I30 measures flares at Mach 3.0 to 6.3; RM L57J29 has no force data. The Redstone reports 6R3P, 6R3N2 and 6R3N1 weren't found online.
- [G29](#g29), October 17, 2026, looked at:
  - NTRS, for measured supersonic forces on boattailed and blunt-nosed bodies
  - NACA RM A52E22 (Dennis and Cunningham, 1952)
  - NASA TM X-653 (1962)
  - NASA TN D-916
  - NACA RM E54B11 (Moskowitz and Jack, 1954), which the project already holds

  It found candidates, US government work, public use permitted. RM A52E22 tabulates lift, drag and moment against angle (tables V to VIII) for 3/4-power and 1/2-power blunt bodies of fineness 3 and 5. They're at Mach 2.75 to 5.00, blunt tips that aren't sphere-cones. RM E54B11's table I gives pressure coefficients by meridian at 3° and 9° over four boattails at Mach 3.12. The boattail's own share of the normal force can be integrated from it, though the project uses it only for drag. TM X-653's blunted nose is another sphere-cone.
- [G30](#g30), October 17, 2026, looked at:
  - GitHub code search for flight-log CSVs
  - CATS's EuRoC 2021 team data (github.com/catsystems/euroc21-team-data@ef54912c, GPL-3.0)
  - Rocketronics' AltiMAX manuals, already held
  - the private collection's AltiMAX files, counted only

  It found a candidate: STA-CARL2/sta-carl2-altimax/sta-carl2-altimax.xls, a second real AltiMAX Excel export (9,940 rows, GPL-3.0, cache under refs/ only). Its header and events are in German (`ZEIT`, `RAMPE`); an RRC3 and an Eggtimer log of the same flight come with it. Of its five voltage channels, one stays near 0.5 V and one holds about 8.2 V throughout. Two fall from about 7.7 V to about 0.5 V within 0.1 s after the export's own pyro events (24.9 s and 123.1 s). The RRC3 logs its drogue at 23.8 s, the AltiMAX's apogee time, and its main at 121.4 s. That checks the voltages' behavior and their tie to the flight, not a second measurement of the same battery. The private collection has one AltiMAX Excel export.
- [G31](#g31), October 17, 2026, looked at:
  - GitHub code search for RRC3 exports ("Temperature Events Voltages" extension:csv)
  - CATS's EuRoC 2021 team data (GPL-3.0)
  - RocketPy-Team/RocketPaper (MIT)
  - RocketPy's Juno III data
  - the private collection's RRC3 logs and the other loggers on their flights, counted only

  It found a partial answer: euroc21-team-data@ef54912c's STA-CARL2 RRC3 export states no units. Its altitude is in meters and its pad temperature 36.1. No other logger on that flight records temperature. RocketPaper@23d9271d's real Valetudo RRC3 file states `Temperature [°C]` beside both a feet and a meters altitude column. In the private collection, 90 RRC3 CSVs (copies included), 4 with bracketed units.
- [G31](#g31), October 20, 2026, looked at:
  - public flight-log repositories on GitHub, and airport weather records

  It found two RRC3 exports in feet, one in °C by three other sensors on its flight and one in °F by its airport's air, named in [§4 of the public-logs note](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/public-logs-evidence-gaps.md#4-reader-paths-second-flights-and-units-g2-g6-g7-g30-g31). Their voltages fit volts, a weak check.
- [G32](#g32), October 17, 2026, looked at:
  - the public logs the sensor noise note measured at rest (five Blue Raven flights, ten StratoLoggerCF, five AltOS, one RRC3, one team-built board)
  - the makers' datasheets it cites

  It found pad and rest noise for four families, upper bounds that mix in wind and handling, and the datasheets' white noise. It found no family's noise measured over a whole flight, and no hobby receiver's GNSS loop measured against a truth.

**Real samples.** Each file a real row reads, kept under `refs/`, with its SHA-256 and the test that reads it:

- [G23](#g23): `refs/snapshots/thrustcurve/2026-10-28/search.json`, SHA-256 `6fac854e349521f526c2f8cafadb91b23c91527261380a9461c27a7af892f510`; ThrustCurve.org's answer to a whole-catalog search, recorded unchanged on October 28, 2026 by `validation/oracles/thrustcurve/snapshot.py`; its records used as published facts with ThrustCurve.org's credit; read by `fusionspace-hpr-net`'s test `each_record_is_the_raw_snapshots_own`.

<!-- evidence: end -->

[register]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/evidence.toml
[label]: https://github.com/nrdptel/fusionspace-eridanus/issues?q=is%3Aissue+label%3Aneeds-sample
[adr]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0278-an-evidence-register-for-every-claim.md
[phrases]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/xtask/src/evidence/pages.rs
[misses]: https://github.com/nrdptel/fusionspace-eridanus/issues/685
