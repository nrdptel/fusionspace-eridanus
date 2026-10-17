# Which of RASAero II's comparison flights M2.3c7 can fly

**In short: 17 of the 36 flights on RASAero II's comparison page fly a COTS solid motor and ship as a `.CDX1` design
in RASAero II's installer. Only 4 of those 17 pass Mach 1, short of the floor of 5 in
[M2.3c7, faster than sound](../decisions-and-roadmap.md#m2-3c7).** The four are Don't Debate This (CTI N5800,
about Mach 3.05), Torrent (AMW M1850GG, 1.23), Rabia (AMW L1080BB, 1.16) and Raven (AeroTech J570W, 1.07). These
Machs come from the peak speed RASAero II stored in each file, divided by the speed of sound at the pad's temperature;
for these four the Rocket Flight Database's predicted values agree to within 0.02 [S4]. Raven sits within 0.1 of the
Mach 1 cut. The other 19 flights fall out for three reasons, all from the page and the installer: 6 ship no design
at all, 6 ship only as an older RASAero (I) `.ALX1` file, and 7 ship a `.CDX1` but fly a team, experimental or
historical motor. The two flights with the best measured supersonic data, Violent Agreement (CTI I216) and its SS
sustainer (CTI J530), use COTS motors, but no design ships for either [S2, S3].

The page's predicted-apogee column needs care. In all 26 rows that ship an `.ALX1`, it equals the "RASAero
Prediction" in that older file's comments. In 19 of the 24 shipped `.CDX1` files, it differs from the apogee
RASAero II itself stored, by −6.11% to +2.95%. Only one shipped design is a post-flight reconstruction: AeroPac
104K, whose file stores the 7.64° launch angle the page says was tuned. None of the 17 eligible flights is flagged.

**What this covers.** It covers the page as read on 2026-10-16, with its 36 rows, notes and per-flight pages
[S1, S3]. It covers the data files of the RASAero II 1.0.2.0 installer, unpacked without running anything [S2].
It covers the Rocket Flight Database v1.2 [S4] and ThrustCurve's motor search [S5]. **How far to trust it.** Every
count comes from the cached page, the files and the database's CSV. The Machs are derived from stored peak speeds,
not measured, and are marked **Inference** where used. No design was flown in HPR Sim. The designs stay in the
gitignored `refs/`: the installer grants no rights to its example files, and the site's comparisons page is
"All rights reserved" [S2, S3]. This note gives counts, motors and derived numbers only. It builds on
[apogee bias](apogee-bias.md) §2, which gives the page's mean signed error of +3.47% and mean absolute error of
6.24% over all 36 flights, and on [ADR-236: supersonic flights from published comparisons][adr-236].

[adr-236]: ../decisions/0236-the-2026-10-11-review-supersonic-flights-from-published-comparisons.md

## 1. What the page and the installer hold

- **The page.** It has 36 flight rows, plus an average row [S1]. Each row gives the rocket, the motor, the diameter,
  how the apogee was measured, the measured apogee, RASAero II's apogee and the percent error. The page states its
  mean error of 3.47% and says "80.6% of the flights within +/-10%"; both match the table (29 of 36). On the files,
  the page says: "RASAero II rocket files (.CDX1 files) for most of these rockets are included in the RASAero II
  software download" [S1].
- **The installer.** `RASAero_II_Setup_Version_1.0.2.0.zip` is 1,803,920 bytes, with sha256 `932e6ad7…bef6`. It
  holds one `.msi`, last saved 2019-05-15, which unpacks to 62 files [S2]. The data files are:
  - 24 example `.CDX1` designs, plus a byte-identical copy of the AeroPac one;
  - 26 RASAero (I) `.ALX1` files;
  - 2 RockSim `.rkt` files, one of them a kit that is not on the page;
  - a default launch site, `LSD.xml`;
  - a motor file, `rasp.eng`, with some 880 motors (the count depends on how placeholders are read).

  Every motor a design names is in `rasp.eng`.
- **The matches.** Each `.CDX1` matches one page row. Its comments repeat that row's measured apogee to the foot, and
  its motor string agrees with the row's motor, so all 24 matches are certain. Across the 36 rows: 24 have a `.CDX1`,
  6 have only an `.ALX1`, and 6 have nothing. That accounts for all 36 [S2].
- **The `.CDX1` format, for M2.3c7's importer.** The file is plain XML with root `RASAeroDocument` and
  `FileVersion` 2. Its top-level elements are `RocketDesign`, `LaunchSite`, `Recovery`, `MachAlt` and
  `SimulationList` (one `Simulation`):
  - `LaunchSite` holds the site altitude in feet, the temperature in °F, the rod length in feet and the rod angle.
    Wind is set in 5 files.
  - The `Simulation` holds the motor string (`<designation>  (<maker code>)`) and the launch weight.
  - It also holds RASAero II's own results: `MaxAltitude` in feet, `MaxVelocity` in ft/s and `TimetoApogee`.
    The Proteus 6 file's comments quote its stored 3,063 ft/s and 81,499 ft, which confirms the units.
  - It stores no Mach history; `MachAlt` is empty or zero in every file [S2].
  - HPR Sim has no `.CDX1` reader yet. **Inference** from `crates/hpr-io/src/` on main.

## 2. The 17 flights M2.3c7 can take

Eligible means two things: the motor matches a ThrustCurve entry by designation and maker [S5], and a `.CDX1`
ships. The Mach column is the stored `MaxVelocity` over the speed of sound at the file's pad temperature, which
is a slight lower bound because air at burnout is colder (**Inference**). In brackets is the database's
OpenRocket Plus prediction [S4]. Err is the page's published error, simulated minus measured; "stored" is the
error of the apogee stored in the `.CDX1` against the same measurement (**Inference**: our arithmetic).

| Flight | Motor (ThrustCurve) | Apogee by | Mach (DB) | Band | Err page | Err stored |
|---|---|---|---|---|---|---|
| Don't Debate This | CTI N5800 | baro, 2 avg | 3.05 (3.04) | 2.5–3.5 | +10.14% | +9.56% |
| Torrent | AMW M1850GG, out of production | baro | 1.23 (1.22) | 1–2.5 | +8.16% | +7.10% |
| Rabia | AMW L1080BB, out of production | baro | 1.16 (1.14) | 1–2.5 | +0.25% | −4.30% |
| Raven | AeroTech J570W | baro | 1.07 (1.07) | 1–2.5, ≤0.1 from 1 | +5.37% | +5.86% |
| Rabia, short fin can | CTI L730 | baro | 0.87 (0.86) | below 1 | −1.97% | −3.39% |
| 12 more | AeroTech, CTI, AMW I to M | baro | 0.55–0.85 | below 1 | −7.89 to +11.52% | −8.03 to +11.53% |

The 12 subsonic flights are Blister, Ion Drive, Cancer Descending, Byrum, Gibb, Thunder & Lightning, EZI-65 on
the J450ST, and five Caliber Isp flights. Over all 17, the page's errors average +2.49% signed and 5.61% absolute; the
stored apogees give +1.92% and 5.73% (**Inference**: our arithmetic). The page gives a measured peak speed for none of
the 17; the Caliber page's "just over 700 ft/sec" is RASAero II's predicted peak [S3]. So HPR Sim's own Mach decides
the band for all 17, as ADR-236 allows.

All 17 are among the 25 page flights in the Rocket Flight Database [S4]. That database admits only flights that
its OpenRocket fork predicts within ±10%. Being in it is not a condition here, since eligibility above comes from
the page and the installer alone. But it means every eligible flight is one that a Barrowman-family code already
fits well (**Inference**).

## 3. The 19 that fall out, by cause

| Cause | Flights | Count | Past Mach 1 |
|---|---|---|---|
| No design, COTS motor | Violent Agreement (I216), its SS (J530), ThreeCarbYen (CTI N/N/M) | 3 | 2 measured, 1 likely |
| No design, team motor | MESOS (KIP-EX), Project R (R class), Proteus 6.5 (Loki-EX) | 3 | all; MESOS 4.18 measured |
| `.ALX1` only | Laser/LOC-2.1 ×2 and EZI-65 (J125), EZI-65 J100, McGarvey, Hubbub | 6 | Hubbub unknown, rest unlikely |
| `.CDX1`, team motor | Qu8k (Q18000), Proteus 6 (P9381), A-601 (P4935) | 3 | all 3 |
| `.CDX1`, motor not listed | Full Metal Jacket ×2 (O10000), L500 rocket (Ace), AeroPac 104K (N1048) | 4 | all 4 |

- **Violent Agreement and its SS sustainer.** Their measured peak Mach is at least 1.5 and 2.4. Each per-flight page
  plots drag coefficient from the coast to those Machs, and the altitude is baro corrected to a balloon's pressure
  [S1 Note 9, S3]. With these two, the count past Mach 1 would be 6. Each page shows a RASAero II scale drawing
  with part lengths, weight and CP, but no design file ships and no full dimension list is given [S2, S3].
- **The motors.** ThrustCurve's search refuses classes P and above, so Proteus 6, A-601 and Qu8k can't be COTS
  entries there. It lists no O10000, L500, J100 or N1048 [S5]. The J125 rows name no maker; AeroTech's J125W, out
  of production, is ThrustCurve's only J125. ThreeCarbYen's three CTI motors are on ThrustCurve, but its measured
  apogee is itself extrapolated, 175,000 ft between GPS fixes [S1 Note 2].
- **Reconstructions.** The page flags two flights: MESOS, whose sustainer delay and launch angle were tuned
  [S1 Note 1; the delay also S6 p. 14], and AeroPac 104K [S1 Note 3]. Project R's run takes a coning angle from
  onboard video [S1 Note 5], a post-flight input the page doesn't call a reconstruction.

## 4. Bands over all 36

These counts use the page's measured Mach where it gives one (MESOS, Proteus 6 and the two Violent Agreement
flights). Otherwise they use the pad-temperature Mach from the stored peak speed, or "unknown" when no
`.CDX1` ships.

| Set | Below 1 | 1–2.5 | 2.5–3.5 | Beyond 3.5 | Unknown |
|---|---|---|---|---|---|
| All 36 | 13 | 9 | 4 | 1 | 9 |
| COTS solid, with or without a file (25) | 13 | 5 | 1 | 0 | 6 |
| COTS solid with a `.CDX1` (17) | 13 | 3 | 1 | 0 | 0 |

- **Near a cut.** Full Metal Jacket's Black Rock flight sits at 2.41 and Violent Agreement SS at "at least" 2.4, both
  within 0.1 of 2.5; Raven sits within 0.1 of 1. Using the cold standard stratosphere (216.65 K) instead of the pad's
  air raises every Mach by 16 to 20%. That would push Don't Debate This to 3.58 and Qu8k to 3.88, past 3.5. A peak at
  burnout a few thousand feet up is far warmer than that, so the pad values stand (**Inference**). Flying the files
  settles it.
- **The unknowns.** Of the 9, ThreeCarbYen, Project R and Proteus 6.5 are almost surely past 2.5: they reach
  72,000 to 175,000 ft, and Proteus 6.5 flies the same motor as Proteus 6 (measured 2.94). The J125, J100 and
  McGarvey flights are likely subsonic. Hubbub flies Rabia's motor on a wider body and reaches a lower apogee, so
  it could fall on either side of 1 (**Inference** throughout).

## 5. What this suggests

1. **M2.3c7 can't meet its floor of 5 from this page.** Its done-when anticipates this case, and the ADR it calls
   for would record 4, with these causes. Raven, at 1.07, can drop below 1 if HPR Sim's own Mach comes out 7%
   lower, so the scored count could be 3. Don't Debate This has the largest published error of the four (+10.14%),
   from a baro reading at 56,573 ft after Mach 3; see [altimeter Mach error](altimeter-mach-error.md).
2. **Two routes would widen the set, and each changes the done-when, so each needs an ADR.**
   - An `.ALX1` reader adds 5 COTS rows, but only Hubbub might pass Mach 1.
   - After 1.0, the 7 non-COTS shipped designs, flown on the curves in the shipped `rasp.eng`, add 7 flights past
     Mach 1. Before 1.0 the scope rule forbids them.
3. **Show both RASAero II numbers.** M2.3c7's "beside RASAero II's published errors" should name its column. The
   page's column matches the RASAero (I) comment wherever an `.ALX1` ships. For Rabia, the stored RASAero II
   apogee is 4.55 points from the page's (−4.30% against +0.25%).
4. **Read units from the file.** The importer must convert `LaunchSite` (feet, °F) and its results (feet, ft/s).
   Its first check can compare the parsed launch conditions and motor string against the comments in the same
   file, which repeat them.
5. **The best supersonic flights are not flyable.** The Violent Agreement pair would be the strongest public
   references above Mach 1: COTS motors, measured Mach and a drag curve. Without a design file they stay out of the
   count.

## Open questions

- Was the page's prediction column ever rerun in RASAero II, or carried over from RASAero (I)? The page calls
  every value RASAero II's. Only the `.ALX1` comments and the stored `.CDX1` results bear on it [S1, S2].
- Are ThrustCurve's Cesaroni I205 and I285 the loads flown in 2004 and 2005? Is "J350W-L" ThrustCurve's J350W or
  J350W-OLD? Is the "J125 (Hist)" the `.ALX1` files name ThrustCurve's J125W? `rasp.eng` gives it another
  length, so the 5 `.ALX1` COTS rows, and the 25 above, rest on that match. Is AeroPac's "N1048 (AT)" a
  prototype? None of these is settled.
- What is Hubbub's peak Mach? Its `.ALX1` stores no speed, so only flying it would tell.
- What peak Mach does HPR Sim give for each of the 17? That needs the `.CDX1` importer M2.3c7 builds.

## Sources

1. **S1.** C. E. Rogers, "RASAero II Comparisons with Altitude Data" (table, Notes 1–9, closing paragraph).
   <https://www.rasaero.com/comparisons-alt.htm>, read 2026-10-16.
2. **S2.** RASAero II download page, <https://www.rasaero.com/dl_software_ii.htm>, and installer
   `RASAero_II_Setup_Version_1.0.2.0.zip` (sha256 `932e6ad7ad20d89f96594d0324dddc2a30a63324ddbb0c043dbce9a0bb56bef6`),
   fetched 2026-10-16; data files cached in `refs/` only, not redistributed.
3. **S3.** The comparisons page, <https://www.rasaero.com/comparisons.htm>, and the per-flight comparison pages:
   <https://www.rasaero.com/comparisons-flight.htm>, and
   `comp-flight-ViolentAgreement.htm`, `comp-flight-ViolentAgreementSS.htm`, `comp-flight-Proteus6.htm`,
   `comp-flight-Cal.htm` on the same site, read 2026-10-16.
4. **S4.** A. Yu, Rocket Flight Database v1.2.0 (CC BY 4.0), `flight_comparison.csv`, README and LICENSE,
   read 2026-10-16.
   <https://zenodo.org/records/20531977>. Its `peak_mach` is predicted by its OpenRocket fork, whose code was not
   opened.
5. **S5.** ThrustCurve motor search API, <https://www.thrustcurve.org/api/v1/search.json>, queried 2026-10-16
   (common name, designation and impulse class; all availabilities).
6. **S6.** C. E. Rogers, "RASAero II Comparison with MESOS 293K Flight Data, Rev B", p. 14, read
   2026-10-16.
   <https://www.rasaero.com/dloads/RASAero%20II%20Comparison%20with%20MESOS%20293K%20Flight%20Data%20-%20Rev%20B.pdf>
