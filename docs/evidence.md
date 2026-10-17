# What backs each claim

Every gap this site states in what backs a reader, a model or a check, with the kind of evidence
behind it, the kind that would settle it, and the open issue that waits for it.

The pages say plainly when something rests on less than the real thing: a format read only from
files invented for its tests, a value assumed because no sample states it, a model checked only
against another program. This page is the register of those statements. Each one is a row of
[`validation/evidence.toml`][register], and each page that states a gap links its row here, so
the gap has a place to be closed. A row short of the evidence it wants keeps an open issue
labeled [`needs-sample`][label] until a real sample or measurement settles it
([ADR-278][adr], the decision that set this up).

> **How far to trust it.** This page is a ledger, not a grade. It counts rows by the kind of
> evidence behind them; it never adds the kinds into one score, because a calibration and a code
> to code comparison answer different questions, and neither stands in for a real flight. The
> check behind it, `cargo xtask evidence`, fails the build when a page states a gap in one of its
> committed phrases without linking a row, when a row's page or anchor is gone, or when a row
> short of its wanted kind has no open issue. It can't find a gap a page states in other words:
> a sentence that avoids every phrase on the list passes unseen, and so does a phrase inside
> quotation marks, which quote another document. The phrases are the list in
> [`xtask/src/evidence/pages.rs`][phrases]; the gaps the list is known to miss, most of them in
> the aerodynamics page's "no measurement", wait in [#685][misses].

## The kinds of evidence

A row's *kind* says where its evidence comes from, after the kinds in the FDA's guidance on
computational models (2023, §VI.B) and the referent grades of NASA's modeling handbook
(NASA-HDBK-7009B, Table 24). The kinds are categories, not a scale, apart from one rule: `real`
is what every row wants unless a decision record sets less.

| Kind | What it means | Example |
|---|---|---|
| real | A file or a measurement from the device or flight itself | A logger's own download of a flown rocket |
| sibling | A real file from a related device or firmware | Another model of the same maker's altimeter |
| code to code | Another program's answer to the same input | GPX values compared with gpxpy's |
| spec | Built from a published layout, schema or model, and checked on invented inputs | A reader written from a format's manual and tested on files made for its tests |
| calibration | Checked against the data it was fitted to, or seen while it was chosen | Flights compared with the simulation after its models were tuned beside them |
| reasoning | An argument only | A unit inferred from what a rocket can do |
| none | Nothing yet | A format no one has a file of |

A row also says what its evidence doesn't cover (*not checked*), lists each real sample with its
SHA-256 hash and the test that reads it, and keeps a dated list of where a search for better
evidence looked and what it found, so the next search doesn't repeat it.

## Sending a sample

If you have a file a row asks for, attach it to the row's issue, or say there where it can be
downloaded and under what license. A sample is kept outside the repository unless its license
allows it in, with its hash in the register so every check reads the same bytes
([Checking a claim](checking-a-claim.md) shows how a pinned file is fetched and verified).

## The register

<!-- evidence: report; written by `cargo xtask evidence` from validation/evidence.toml; do not edit -->

The register holds 31 rows, as of October 17, 2026.

**By kind.** Each surface's rows, counted by the kind of evidence behind them now:

| Surface | Code to code | Spec | Calibration | Reasoning | None | Rows |
|---|---:|---:|---:|---:|---:|---:|
| Flight-log readers | 3 | 2 | 0 | 4 | 2 | 11 |
| The flight analyzer | 0 | 2 | 0 | 0 | 0 | 2 |
| Flight models | 1 | 1 | 0 | 2 | 1 | 5 |
| Aerodynamics | 2 | 2 | 0 | 2 | 0 | 6 |
| `.ork` import | 0 | 1 | 0 | 1 | 0 | 2 |
| Accuracy against real flights | 0 | 0 | 1 | 1 | 0 | 2 |
| Fin flutter | 0 | 1 | 0 | 0 | 0 | 1 |
| The motor finder | 0 | 1 | 0 | 0 | 0 | 1 |
| Design export | 1 | 0 | 0 | 0 | 0 | 1 |
| All | 7 | 10 | 1 | 10 | 3 | 31 |

**Short of what's wanted.** 30 of the 31 rows don't yet have the kind of evidence they want; each has an open issue labeled `needs-sample`, linked in its row below.

**The rows.**

| Row | Surface | Claim | Evidence | Wanted | Issue | Last search |
|---|---|---|---|---|---|---|
| <a id="g1"></a>G1 | Flight-log readers | GPX tracks are read as the GPX 1.0 and 1.1 schemas define them, checked against gpxpy on three invented files. Not checked: what a real tracker or phone writes, and what its `ele` is measured from. | code to code | real | [#654](https://github.com/nrdptel/fusionspace-eridanus/issues/654) | 2026-10-17 |
| <a id="g2"></a>G2 | Flight-log readers | Paths no sample log reaches are checked by the readers' tests alone: decimal commas, workbooks dated from 1904, a Raven `.FIPa` temperature below zero, an RTx fix at (0, 0). Not checked: a real file that takes any of these paths. | spec | real | [#655](https://github.com/nrdptel/fusionspace-eridanus/issues/655) | none |
| <a id="g3"></a>G3 | Flight-log readers | AltOS radio telemetry is decoded from AltOS's documents for the packet types the samples hold. Not checked: the eleven packet types no sample holds, which are left out; the GPS ground speed and climb rate, which nothing checks. | spec | real | [#656](https://github.com/nrdptel/fusionspace-eridanus/issues/656) | none |
| <a id="g4"></a>G4 | Flight-log readers | AltOS `.eeprom` logs are read in formats 1, 6, 7, 14, 15, 16 and 22, with the TeleMetrum v1's converter assumed. Not checked: that the TeleMetrum v1's converter measures against the pressure sensor's own supply, which no document states; the TeleMega v4's IMU, which its one sample never ran; 4 of the 14 sample files, with neither an export nor telemetry beside them; formats 5, 9 and 19, refused. | reasoning | real | [#657](https://github.com/nrdptel/fusionspace-eridanus/issues/657) | none |
| <a id="g5"></a>G5 | Flight-log readers | ARTS2 raw data is timed at 50 records a second, the rate the maker's DataAnalyzer gives every file. Not checked: where the file stores its rate, if it does, and a file recorded at another rate. | code to code | real | [#658](https://github.com/nrdptel/fusionspace-eridanus/issues/658) | none |
| <a id="g6"></a>G6 | Flight-log readers | Some layouts rest on one sample or on no export to compare with: the AltiMAX `.ALS`, the AIM `.xtra` and raw dump, 7 of 63 raw downloads, 5 of 6 RRC3 `.rff` files, and each case of an AltOS CSV re-saved by a spreadsheet. Not checked: a second flight of each layout read beside its maker's export; the raw downloads with no export beside them. | code to code | real | [#659](https://github.com/nrdptel/fusionspace-eridanus/issues/659) | none |
| <a id="g7"></a>G7 | Flight-log readers | Featherweight summaries are checked on their maximum and pad altitude only. Not checked: the speeds, accelerations, times and the rest, converted from the unit the file writes and checked against nothing. | none | real | [#660](https://github.com/nrdptel/fusionspace-eridanus/issues/660) | none |
| <a id="g8"></a>G8 | Flight-log readers | GPS heights are kept in the unit the file writes: a phone app's altitude as written, KML heights the least sure. Not checked: each height's unit and what it is measured from, beside a barometric log of the same flight. | reasoning | real | [#661](https://github.com/nrdptel/fusionspace-eridanus/issues/661) | none |
| <a id="g9"></a>G9 | Flight-log readers | Formats with no sample (CATS's, a logger named Juno and the others listed) have no reader and are refused. Not checked: every format the list names: no sample on terms that let its bytes be tested. | none | real | [#662](https://github.com/nrdptel/fusionspace-eridanus/issues/662) | none |
| <a id="g10"></a>G10 | The flight analyzer | The analyzer's readings are checked on an invented flight whose every reading is known, and on one real log by hand. Not checked: real logs of other loggers, for which the readings are withheld. | spec | real | [#663](https://github.com/nrdptel/fusionspace-eridanus/issues/663) | none |
| <a id="g11"></a>G11 | The flight analyzer | Reading your own logger by a stated mapping or layout is shown on one invented flight's logger and GPS tracker files. Not checked: a real logger's files read by a mapping or layout someone states. | spec | real | [#664](https://github.com/nrdptel/fusionspace-eridanus/issues/664) | none |
| <a id="g12"></a>G12 | Flight models | Dryden turbulence is an aircraft model, flown as its standard gives it. Not checked: any rocket flight: it is unvalidated for rockets. | spec | real | [#665](https://github.com/nrdptel/fusionspace-eridanus/issues/665) | none |
| <a id="g13"></a>G13 | Flight models | A dropped booster or middle stage flies on after the split, as its own rocket. Not checked: the whole flight of the dropped part, which is not validated at all. | none | real | [#666](https://github.com/nrdptel/fusionspace-eridanus/issues/666) | none |
| <a id="g14"></a>G14 | Aerodynamics | Base drag with the motor burning takes the motor case off the base. Not checked: any measurement of base drag under power. | reasoning | real | [#667](https://github.com/nrdptel/fusionspace-eridanus/issues/667) | none |
| <a id="g15"></a>G15 | Flight models | Thrust about a center of gravity off the axis turns the rocket, so some vertical calm flights drift sideways. Not checked: which is right: OpenRocket's flights don't drift, and nothing measures it. | code to code | real | [#668](https://github.com/nrdptel/fusionspace-eridanus/issues/668) | none |
| <a id="g16"></a>G16 | `.ork` import | A part's roll angle in an `.ork` file turns the way HPR Sim assumes. Not checked: which direction OpenRocket turns it. | reasoning | real | [#669](https://github.com/nrdptel/fusionspace-eridanus/issues/669) | none |
| <a id="g17"></a>G17 | Flight models | The apogee printed for an unstable sustainer assumes it stays upright, and flights off a rod and off a rail differ. Not checked: an unstable sustainer's real flight; where the rod and the rail differ, not traced. | reasoning | real | [#670](https://github.com/nrdptel/fusionspace-eridanus/issues/670) | none |
| <a id="g18"></a>G18 | `.ork` import | An inner tube's automatic outer radius and a ring's automatic bore rest on their unit tests. Not checked: OpenRocket's own values for designs that use them: no oracle covers them. | spec | real | [#671](https://github.com/nrdptel/fusionspace-eridanus/issues/671) | none |
| <a id="g19"></a>G19 | Aerodynamics | RASAero II's exported tables are flown from one real export, to Mach 0.75. Not checked: a real export flown faster, checked by unit tests alone; past the export's last angle, and at 0° faster than Mach 1.3, the values are assumed. | spec | real | [#672](https://github.com/nrdptel/fusionspace-eridanus/issues/672) | none |
| <a id="g20"></a>G20 | Flight models | A pushed part leaves through the end the simulator assumes, from the way its body points as it falls. Not checked: a measured deployment: the directions are assumptions, not measurements, and for a drogue between the booster and the avionics bay a guess. | reasoning | real | [#673](https://github.com/nrdptel/fusionspace-eridanus/issues/673) | none |
| <a id="g21"></a>G21 | Accuracy against real flights | Four of the seven public altimeters are taken as barometric. Not checked: each of the four altimeters' sensor; none is calibrated. | reasoning | real | [#674](https://github.com/nrdptel/fusionspace-eridanus/issues/674) | none |
| <a id="g22"></a>G22 | Fin flutter | Martin's flutter formula gives a fin's flutter speed. Not checked: flutter data: nothing here is checked against a hobby rocket. | spec | real | [#675](https://github.com/nrdptel/fusionspace-eridanus/issues/675) | none |
| <a id="g23"></a>G23 | The motor finder | The motor finder is replayed on three stand-in searches in ThrustCurve.org's shape. | spec | spec ([ADR-278](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0278-an-evidence-register-for-every-claim.md)) | none | none |
| <a id="g24"></a>G24 | Accuracy against real flights | The simulation's apogees miss the real flights by 6.04% on average, outside the 5% target. Not checked: a flight held out from every choice of model and every fit, predicted before its log is read. | calibration | real | [#676](https://github.com/nrdptel/fusionspace-eridanus/issues/676) | none |
| <a id="g25"></a>G25 | Design export | The `.ork` files `hpr convert` writes are checked by opening them in OpenRocket. Not checked: whether RockSim opens them. | code to code | real | [#677](https://github.com/nrdptel/fusionspace-eridanus/issues/677) | none |
| <a id="g26"></a>G26 | Aerodynamics | Drag is compared with the RASAero II curves of RocketPy's example rockets, of which Calisto's is the one real RASAero II export. Not checked: the other curves' rockets, which record no fins or surface finish, so HPR Sim flies stated guesses for them; a measured drag of any of them. | code to code | real | [#678](https://github.com/nrdptel/fusionspace-eridanus/issues/678) | none |
| <a id="g27"></a>G27 | Aerodynamics | Tube fins' normal force and center of pressure are compared with OpenRocket 24.12's on 14 probe designs and its tube fin rocket. Not checked: a measured tube fin's lift or center, of which none was found; the interference between tubes, an unchecked estimate. | code to code | real | [#679](https://github.com/nrdptel/fusionspace-eridanus/issues/679) | none |
| <a id="g28"></a>G28 | Aerodynamics | A flared body's supersonic normal force rests on one measured flare, and a flare past its corner's limit flies a rule checked against nothing. Not checked: a second measured flare; any flare past its corner's limit. | reasoning | real | [#680](https://github.com/nrdptel/fusionspace-eridanus/issues/680) | none |
| <a id="g29"></a>G29 | Aerodynamics | Faster than sound, a boattail's share of the body's force comes from the shock-expansion method, and a blunt tip flies it behind a Newtonian cap. Not checked: the boattail's share, unvalidated; a blunt tip, checked on a sphere-cone only. | spec | real | [#681](https://github.com/nrdptel/fusionspace-eridanus/issues/681) | none |
| <a id="g30"></a>G30 | Flight-log readers | The AltiMAX's Excel export keeps its voltages as the file states them. Not checked: the voltages, unchecked against anything. | reasoning | real | [#682](https://github.com/nrdptel/fusionspace-eridanus/issues/682) | none |
| <a id="g31"></a>G31 | Flight-log readers | An RRC3's Temperature and Voltages columns, which state no unit, are read in the units its manual pairs with the file's altitude unit. Not checked: the units, assumed: a file whose temperature or voltage can be checked against another record of the same flight. | reasoning | real | [#683](https://github.com/nrdptel/fusionspace-eridanus/issues/683) | none |

**Due a search.** A row is due one when it has none yet, or its last was more than 30 days before October 17, 2026: [G2](#g2), [G3](#g3), [G4](#g4), [G5](#g5), [G6](#g6), [G7](#g7), [G8](#g8), [G9](#g9), [G10](#g10), [G11](#g11), [G12](#g12), [G13](#g13), [G14](#g14), [G15](#g15), [G16](#g16), [G17](#g17), [G18](#g18), [G19](#g19), [G20](#g20), [G21](#g21), [G22](#g22), [G23](#g23), [G24](#g24), [G25](#g25), [G26](#g26), [G27](#g27), [G28](#g28), [G29](#g29), [G30](#g30) and [G31](#g31).

**Searches so far.**

- [G1](#g1), October 17, 2026: looked at the download pages and documents of Featherweight, AltOS, Eggfinder, Missile Works and BigRedBee, and public repositories of flight logs; found no openly downloadable GPX of a real rocket flight: the dedicated trackers write KML, CSV or raw NMEA, and GPX comes from phone apps, handheld receivers and mapping tools.

<!-- evidence: end -->

[register]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/evidence.toml
[label]: https://github.com/nrdptel/fusionspace-eridanus/issues?q=is%3Aissue+label%3Aneeds-sample
[adr]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0278-an-evidence-register-for-every-claim.md
[phrases]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/xtask/src/evidence/pages.rs
[misses]: https://github.com/nrdptel/fusionspace-eridanus/issues/685
