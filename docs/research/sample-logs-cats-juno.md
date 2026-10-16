# Public sample logs for the loggers HPR Sim can't read yet

This note asks which public, license-clear flight logs exist for the loggers HPR Sim's importers don't read: the
CATS Vega and "Juno", which M7.1c23 in the [roadmap](../ROADMAP.md) names with no sample, and the Eggtimer Quasar,
Altus Metrum's newer boards, Featherweight's newer devices, Entacore's native files and Jolly Logic's. For each:
where the files are, their format, whether it is documented outside GPL code, the data's license and the count. It
serves M7.1's list of formats without samples, and builds on [the Debrief formats note](debrief-log-formats.md),
[suite-avionics](suite-avionics.md) (CATS is GPL-3.0) and [altimeter-height-formulas](altimeter-height-formulas.md).
Sources are makers' manuals and pages, public repositories pinned by commit, and the files; no GPL source code was
opened. **Inference** marks this note's reasoning. Read 2026-10-16.

**In short.**

- **The CATS Vega has public samples, under GPL-3.0.** CATS's EuRoC 2023 repository holds 11 Vega `.cfl` logs from
  7 teams, 22 ground-station CSVs and 8 summary files; a second repository adds two 2021 flights' processed
  CSVs. The only license is the repository's GPL-3.0, so the files can be cached under `refs/` and cited, not
  committed. The `.cfl` byte layout is published only in GPL code; the ground-station CSV states its units.
- **"Juno" is a rocket, not a logger:** the roadmap's first "EuRoC/Juno CSV layouts" is RocketPy's Juno III data,
  whose RRC3 and GNSS tables HPR Sim's validation already flies, by fixed column, not through the importers.
- **Five of RocketPy's EuRoC 2023 tables are CATS Vega data**, four with the Vega's raw `.cfl` beside them. The
  filtered column reads above the Vega's own peak by 0.04% to 1.66%; on Lince by 59.9 m.
- **Commercial loggers:** public samples of the Eggtimer Quasar's CSV (2 files), AltOS `.eeprom` log formats 5
  (7 files) and 9 (2) and the Entacore AIM USB's binary `.aim` (4 maker's examples), none with a license; and a
  TeleMega v4.0 `.eeprom` in log format 19, which HPR Sim refuses, with AltosUI's CSV beside it (GPL-3.0). None
  was found for the EasyTimer V2, TeleMini, EasyMotor, MicroPeak, Blue Jay, Eggtimer ION or AltimeterThree.

## 1. CATS Vega

**euroc23-team-data** ([S1], commit `2dca527`, 2023-10-30). CATS's ground stations logged every EuRoC 2023 team's
telemetry; a `.cfl` was added where a working Vega was recovered (README lines 6–7). Counted from the git tree:
13 team folders; 11 `.cfl` (8 main, 3 payload) in 7 of them; 22 ground-station CSVs and one `.xlsx`; 8 `*_stats.txt`.

- **`.cfl`**, binary. In `20_red/main.cfl` (1,272,588 bytes): a NUL-ended version string `3.0.1-euroc`, then
  records that each open with a 4-byte little-endian counter and a 4-byte field holding a single bit (0x10, 0x20,
  0x40 …). The manual's "bit mask" for record types fits that (**Inference**, from the bytes). The manual names
  the file, the Configurator's *Export CSV* and a command-line `flight_parse`, but no layout; for the record
  types it links the firmware's enum, which is GPL source [S4]. A clean-room reader needs the GPL Configurator run
  as an outside oracle, its CSV export decoded against the bytes. That has not been tried.
- **`*_stats.txt`**, the Vega's own summary: peak height, speed and acceleration, the pad's height above sea level,
  the liftoff time in UTC. `20_red`: 3,014.89 m, 300.70 m/s.
- **Ground-station CSV**: `link,ts[deciseconds],state,errors,lat[deg/10000],lon[deg/10000],altitude[m],`
  `velocity[m/s],battery[decivolts],pyro1,pyro2`. Whole meters; two radio links' rows interleave, so packets
  repeat (`20_red`: 1,150 rows, 679 distinct stamps over 138.7 s, about 4.9 Hz). Its peak, 3,014 m, matches the
  stats file. The manual says the altitude sent is a barometer-fed Kalman estimate [S4, "Estimation Algorithms"].

**euroc21-team-data** ([S2], commit `ef54912`): CSVs CATS's Python parser made from two 2021 flights. A README
lists the records at 100 Hz: barometers, IMUs, a magnetometer, a high-g accelerometer, the state estimate and
"filtered barometer data (median filtered over 9 samples)". **cats-logs** ([S3], commit `0dc87a4`): 3 `.cfl` (two
simulations, one a copy of a euroc23 file) and 84 older dumps, mostly drone tests. **CATS Flights** [S7], the
maker's gallery: 13 flights with the `.cfl` to download; its terms license only CATS to reuse them: cite only.

**The Configurator's CSV export.** The only public copy found is in RocketPy (MIT), TU Wien's Hedy, EuRoC 2025
(`data/rockets/hedy/cats_tust/`) [S6]: five files by record, such as `baro.csv` (`ts,id,T,P`) and
`filteredDataInfo.csv` (`ts,filteredAltitudeAGL,filteredAcceleration`), 100 Hz. The notebook states no permission
for this data, unlike its others. Hedy is a liquid rocket: a format sample only.

## 2. "Juno", and RocketPy's EuRoC tables

The original roadmap line ([S9], `docs/ROADMAP.md:365` at the first public commit) reads "The EuRoC/Juno CSV
layouts". RocketPy's `data/rockets/juno3/` [S5] is Juno III, Projeto Jupiter, Spaceport America Cup 2023, shared
with RocketPy by a team member (README lines 3–10). HPR Sim already scores it as a real flight [S10]. Its files:

- `cots_altimeter.csv`, an RRC3 (README line 21): `time,altitude,pressure,velocity,temperature,events,voltages`,
  lowercase, no units; 611 rows at 20 Hz to 30.5 s; altitude in meters, pressure in hPa; the last two rows corrupt.
  HPR Sim's RRC3 reader matches column names regardless of case (`crates/hpr-flightdata/src/rrc3.rs:242`), so it
  probably reads it (**Inference**; untested).
- `cots_GNSS.csv`, device unstated: `time,ALT,HORZV,VERTV,LAT,LON`, mostly 10 Hz then 1 Hz, ALT in feet above sea
  level; no GPS header HPR Sim knows. `srad_telemetry.csv`, the team's board: 26 columns, two named `Time`, no units.

No rocketry altimeter named Juno was found; the Alti-2 Juno is a skydiving altimeter [S8]. **Inference:** "Juno"
needs no format entry, only a line in the guide saying what the name meant.

**Five EuRoC 2023 tables in RocketPy are CATS Vega data.** Each notebook names its team number, and each number
matches a euroc23 folder [S1, S6]. The tables' columns, `ts,filtered_altitude_AGL,filtered_acceleration`, are the
names of CATS's 2021 export, at 0.01 s from −0.75 s (a pre-launch buffer; the manual's "PREFILLING" gives no length).

| RocketPy table | euroc23 folder | Table peak | Vega's own peak (stats) | Telemetry peak |
|---|---|---:|---:|---:|
| `astg/altimeter_halcyon.csv` | `01_astg` | 3,470.83 m | 3,466.94 m | |
| `genesis/flight_data_faraday.csv` | `11_faraday` | 2,916.69 m | no stats file | 2,914 m |
| `camoes/flight_data.csv` | `20_red` | 3,016.00 m | 3,014.89 m | 3,014 m |
| `lince/main_data.csv` | `23_star` | 3,668.52 m | 3,608.63 m | 3,607 m |
| `polito/altimeter_cavour.csv` | `17_polito` | 2,789 m | no `.cfl` | 2,789 m |

- **Cavour's table is the ground station's log** with repeated rows removed: same first rows and peaks (2,789 m,
  314 m/s) as `17_polito/gs_log_main_1.csv`. HPR Sim's record already says so [S10].
- **Sign and size.** The filtered column reads above the Vega's peak by +0.04% (Camões), +0.11% (Halcyon) and
  +1.66% (Lince, +59.9 m, at a logged 368.5 m/s, near Mach 1). The 2021 README lists "filtered barometer data,
  median filtered over 9 samples" apart from the state estimate, so `filtered_altitude_AGL` is most likely the
  median-filtered barometer, not the Kalman height (**Inference**, 2021 firmware applied to 2023 logs). HPR Sim's
  records call Genesis's column "probably" the Vega's Kalman estimate and Lince's computer unnamed [S10].
- **The reading moves Lince's score.** Its record logs 3,587.72 m; the Vega's stats give 3,608.63 m (+0.58%); the
  table's highest row is 3,668.52 m (+2.25%).

## 3. Commercial loggers

**Eggtimer.** The Quasar's detail CSV, downloaded over WiFi: `T` (s from launch detect), `Alt` (raw barometric
altitude), `FAlt`, `FVeloc`, then event columns `LDA`, `Apogee`, `N-O`, `Drogue`, `Main` that hold the altitude
at the event's sample and zero elsewhere [B1, pp. 39–40]. Heights are in feet; the guide has no metric setting.
Ascent 10–33 samples/s (default 20), descent 1–10 [B1, pp. 28–30]. Firmware 1.02d (March 2023) dropped the
low-velocity column [B2]. The ION's columns are `T, Alt, Veloc, FAlt, FVeloc, LDA, LowV, Apogee, N-O` [B3,
pp. 19–21]. [B4] lists no download for the Apogee, Quark or EZ-DD.

- **Public samples:** 2 Quasar files in one repository with no license [B6], both with a tenth column, `AUX`,
  the guide doesn't list, and a memory dump (`Page,Sub-Page,RAddr,Address,Data`) no Eggtimer document mentions.
  13 other public `FAlt,FVeloc` hits (11 distinct files) don't name the device; the three headers look alike.
  The notes give both flights' motors and one pad mass, and an EasyMini `.eeprom` flew beside the Quasar once.
- **HPR Sim's reader** names `AUX`, `LowV` and any column after `LDA` as events
  (`crates/hpr-flightdata/src/eggtimer.rs:50`), so both layouts probably read (**Inference**; no test file).

**Altus Metrum.** AltosUI's CSV export is described as one format [B5, §12.5.1]. The EasyTimer has no barometer
[B5, §9], so an EasyTimer export may not pass the AltOS CSV reader (**Inference**). Altus publishes bytes per
sample for each device, not the records (16 bytes for EasyMini and TeleMini v3, 32 for TeleMega and EasyMega;
[B5, App. F, Table 11]). HPR Sim reads `.eeprom` log formats 1, 6, 7, 14, 15, 16 and 22 and refuses the rest
([the guide](../format/flight-logs.md)). A code search for `"log_format"` in `.eeprom` files found 39 public files
outside the maintainer's repositories: EasyMini 25, TeleMetrum 10, TeleGPS 2, TeleMega 1, EasyMega 1. Only
**format 9** (TeleGPS, 2 files [B7, B8], one named as corrupt) is outside HPR Sim's list. A search for `TeleMega`
found 7 distinct TeleMega v1.0 text-form files in **format 5**, in one repository [B9]. None states a license.
The code search missed a third: a TeleMega v4.0 file (AltOS 1.9.6) in **format 19**, with the AltosUI CSV export
of the same flight beside it, in CATS's GPL-3.0 euroc21 repository [S2, `Aris-Piccard/TeleMega …/`]. The guide
lists the TeleMega v4 under format 15 and its IMU as unread for want of a sample that ran it. No
public EasyTimer, TeleMini or EasyMotor `.eeprom` was found. **MicroPeak** is the one memory layout here documented
outside GPL code, a byte table in its manual [B10, App. C]; no public log was found.

**Featherweight.** The Blue Jay's CSV, 50 samples/s, is described column by column in prose, and the passage names
the Blue Raven twice [B11]; it is probably the Blue Raven low-rate header HPR Sim reads (**Inference**; no sample).
The Swift tracker exports its GPS log "like a regular flight" with a KML [B12]; no sample.

**Entacore.** The AIM USB's maker's software zip holds 4 example `.aim` flights (2,357 to 33,973 bytes, 2013–2015)
[B13]: binary, all four opening with the same 16 bytes, no published layout, terms unstated. The AIM XTRA's native
`.xtra` and its *Export emulation data* CSV are unpublished [B14, p. 38]; no public `.xtra` was found.

**Jolly Logic.** The AltimeterThree's app shares a flight
"as a spreadsheet" [B16], with no column list published and no public file found. No "AltimeterFour" exists in
any source found. **Others:** the RRC3+ downloads through mDACS like the RRC3 [B17], so the existing readers
probably cover it; the SparkyVT computer (MIT) has no data files; no project called "RocketTalk" was found.

## 4. The list

| Format | Public samples | Layout documented outside GPL | Data terms |
|---|---:|---|---|
| CATS Vega `.cfl` | 11 [S1]; 13 in the gallery [S7] | no | GPL-3.0; gallery: cite only |
| CATS ground-station CSV | 22 [S1] | the header states units | GPL-3.0 |
| CATS processed CSVs | 2021 parser's [S2]; Configurator's, 1 set [S6] | no; self-named | GPL-3.0; MIT, no permission |
| Eggtimer Quasar detail CSV | 2 [B6] | column list [B1] | no license |
| Eggtimer Quasar memory dump | 1 [B6] | no | no license |
| AltOS `.eeprom` format 5 / 9 / 19 | 7 [B9] / 2 [B7, B8] / 1 [S2] | no | none / none / GPL-3.0 |
| Entacore AIM USB `.aim` | 4 [B13] | no | maker's, unstated |
| Blue Jay CSV; Eggtimer ION CSV | 0 | prose [B11]; column list [B3] | |
| MicroPeak memory | 0 | byte table [B10] | |
| EasyTimer V2, TeleMini, EasyMotor; `.xtra`; AltimeterThree | 0 | no | |

**Which devices flyers use** has weak evidence. In a public mirror of Tripoli's altitude records (MIT, flights
1998–2021, 129 rows naming electronics) [B18], rows name Featherweight 24 times, Altus Metrum 22, Missile Works 20,
Adept 19, PerfectFlite 17, G-Wiz 16, Eggtimer 0. Record flights are few and high-end; no poll with counts was found.

## What this suggests

- The guide's M7.1c23 list can say CATS has samples (GPL-3.0: cached, not committed), its `.cfl` layout only in
  GPL code, and that "Juno" was a rocket whose tables HPR Sim's validation already reads.
- Genesis's and Lince's records can name the CATS Vega, and Lince's logged apogee can be compared with the Vega's
  own summary: the column read moves its score by up to 2.25%.
- One test file each would settle the Quasar's `AUX` column, the ION, and AltOS formats 5 and 9; those samples
  lack a license, so a test needs an invented file built from the manual, or the owner's permission.
- The format-19 TeleMega file can be decoded against the AltosUI CSV beside it, from the cache, never committed.
- A `.cfl` reader would need the CATS Configurator as an oracle (allowed: running a GPL tool, not reading it).

## Open questions

- The `.cfl` field meanings, and whether the Configurator's export can serve as an oracle; not tried.
- Whether 2023 firmware's `filtered_altitude_AGL` is still the 9-sample barometer median. `11_faraday` has no
  stats file, so Genesis's column has no Vega peak to compare with.
- Juno III's GNSS device; whether the Blue Jay header is the Blue Raven's byte for byte (needs one file). Not
  surveyed: Rocketry Forum attachments (no license), G-Wiz, Adept, Estes, Apogee Components, MicroPeak's `.mpd`.

## Sources

Makers' documents and files are cited, cached under the ignored `refs/` only. All read 2026-10-16, with GitHub
code searches for `FAlt,FVeloc` in CSV (56 hits), `"log_format"` (44) and `TeleMega` in `.eeprom`.

- [S1] catsystems/euroc23-team-data @ `2dca527a`: README lines 6–19, `LICENSE` (GPL-3.0), `*/main_stats.txt`,
  `*/gs_log_main_1.csv`. <https://github.com/catsystems/euroc23-team-data>
- [S2] catsystems/euroc21-team-data @ `ef54912c`: `Aris-Euler/CATS/README.md`, `Aris-Piccard/CATS/README.md`.
  <https://github.com/catsystems/euroc21-team-data>; [S3] catsystems/cats-logs @ `0dc87a47`.
- [S4] CATS User Manual (2026-09-18) <https://catsystems.io/manual>; [S7] <https://catsystems.io/flights>, terms.
- [S5] [S6] RocketPy (MIT) @ `9bd6ad3a` <https://github.com/RocketPy-Team/RocketPy>: `data/rockets/juno3/` (README
  lines 3–23), `data/rockets/{astg,genesis,camoes,lince,polito,hedy/cats_tust}/`, their notebooks in `docs/examples/`.
- [S8] Alti-2 Juno, avpay.aero's listing; [S9] this repository's `docs/ROADMAP.md:365` at `aeae3820`; [S10] the
  [real-flight report](../../validation/reports/real-flights.md), entries `juno-iii`, `cavour`, `genesis`, `lince`.
- [B1] Eggtimer Quasar User's Guide 1.02S/T/X; [B2] its release notes 1.02X; [B3] ION Users Guide 1.04A; [B4]
  Altimeters & AV Bay page <https://eggtimerrocketry.com/>. [B5] *The Altus Metrum System* (release notes to
  1.9.22); [B10] *MicroPeak Owner's Manual* <https://altusmetrum.org/AltOS/doc/>.
- [B6] NateOrlowski06/Rocket-Data; [B7] Olin-Rocketry/sensing, `software/Legacy/IREC Launch Data/`; [B8]
  osu-bsli/irec, `flight-data-logs/brute_test_flight_2025-11-23/`; [B9] cmontalvo251/aerospace,
  `rockets/Launch_Society/Flight_Data/`. All on GitHub, none with a license.
- [B11] Blue Jay manual (2026-05-12), "Low-rate downloaded data interpretation"; [B12] Swift manual (2026-04-10)
  <https://www.featherweightaltimeters.com/>; [B13] Entacore downloads, "AIM USB Fw 4.00 Sw 4.00" zip; [B14] AIM
  XTRA & BASE Manual 4.x.x v7 (2025-02-02) <https://www.entacore.com/downloads/>.
- [B15] Jolly Logic support <https://jollylogic.com/support/>; [B16] AltimeterThree's App Store listing, v5.3.01;
  [B17] RRC3+, LOC Precision's listing; [B18] jafrado/tra-records (MIT), `TripoliRecords2016-*.csv`.
