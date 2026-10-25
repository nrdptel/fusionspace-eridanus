# Public logs for the analyzer's `needs-sample` evidence rows

Which public flight logs could close the [evidence register](../evidence.md)'s rows that wait for a real sample
(G1 to G11, G30, G31), and which hold the quiet pad and coast that issue #725 needs to compare the synthetic
loggers' noise with real logs? It skips the register's October 17 searches and the finds in
[sample-logs-cats-juno](sample-logs-cats-juno.md) and [public-flights](public-flights.md), and builds on
[altimeter-sensor-noise](altimeter-sensor-noise.md) §3. Sources: repositories pinned by commit, data sites, makers'
pages, Altus Metrum's documents, read 2026-10-20; no GPL code opened. No reader was run on the new files, so "reads
today" is inferred from the readers' documented rules. **Inference** marks this note's reasoning.

**In short.**

- **No row closes on clear terms.** Most finds are unlicensed (cache under `refs/`, cite only); the license-clear
  ones are partial: an MIT AltOS KML with both heights (G8), 16 cleaned Blue Jay flights under Apache-2.0 (G9), an
  MIT RRC3 `.rff` with its export (G6), and MIT logs of three families for #725.
- **G31's rule fails on a real file.** An RRC3 export in feet writes `Temperature` in °C (three other sensors on
  the flight agree); the feet-means-°F rule turns 47.2 into 8.4 °C, about 36 K too cold. Another in feet (MIT) is
  in °F. And Entacore's own 11 example `.xtra` files are refused: archive version 8, where the reader takes 16.
- **#725 can't measure a pad as worded.** No public barometric log holds 3 s of quiet pad; loggers cut files at
  or just before launch. Rests after landing are long enough in PerfectFlite (17 logs, 9 MIT) and TeleMega-class
  (3, 2 MIT) logs, and there every synthetic profile is louder than the real median.
- **Two synthetic terms differ in kind.** The Blue Raven's lag-1 of about 0.5 comes from each baro value held for
  two rows, not the sensor; real ejection dips pass the synthetic −16 m in 9 of 34 logs, down to −1,503 m.

## 1. GPS tracks (G1, G8)

- **G1, a real rocket's GPX: none on usable terms.** Rocketry Forum thread 135349 [S1] holds an Eggfinder track
  saved as GPX by Mapsphere beside the same J415W flight's StratoLogger CF `.pf2` (2016-07-16), but the thread and its
  attachments answer HTTP 403 to a plain fetch, and no terms are stated: cite only. GitHub code search, OpenStreetMap
  traces tagged rocket, the data sites in §2 and phone-app searches found nothing more (one 2017 post, no files [S4]).
- **G8, a KML with both heights: one MIT file.** robertrau/libpifly @`bbd34247`, `examples/gps/].kml` [S2]: an
  AltosUI KML of a TeleMega v1.0 flight (firmware 1.9.4, 2020-07-18), barometric and GPS paths of 128 points each,
  `altitudeMode` absolute; its summary states 824 m baro, 828 m GPS. The baro path runs 220.3 to 1,039.3 m with
  the boost mark at 215.3 m, so it peaks 824 m above the pad: both heights are meters above sea level
  (**Inference**). Partial: the barometer is inside the KML, not a separate log.
- **A fuller G8 case, unlicensed:** njitrocketry/irec26-test-data @`811cd862` [S3]: a TeleMetrum KML, AltOS CSV,
  telemetry CSV and a Blue Raven's three files of one 2026 flight.

## 2. Formats with no reader, more loggers, team computers (G9, G10, G11)

- **G9, Featherweight Blue Jay: 16 flights, Apache-2.0.** Kaggle evelyn81/unguided-rocket-flight-telemetry-datasets
  [S5], version 1, no login: 17 CSVs at 50 Hz from Southern Utah University rockets; two share 9,421 data rows, so
  16 flights (**Inference**). Cleaned: the manual's `Year, Month, Day, Time` columns [S6] are gone. Each opens 1.9
  to 2.5 s before liftoff and holds the coast; peaks 1,847 to 6,395 ft AGL. There is no Blue Jay reader, but
  the Blue Raven reader also takes a header opening `Flight_Time_(s),Sync`, as these do, so they should read
  (**Inference**).
- **A G9 lead, unlicensed:** EPFLRocketTeam/2018_SP_FLIGHT_SIM-CONTROLLER @`263b7e29`, 4 Raven files in a
  `#ConvertedFromBinary` text form the `.FIPa` reader wouldn't take (**Inference**). Eggtimer CSVs found with no
  license carry the TRS layout, which is already read.
- **G9, nothing found** for CATS, ION, MicroPeak, AltimeterThree, EasyTimer, TeleMini or EasyMotor in GitHub queries,
  makers' pages or Zenodo, figshare, Harvard Dataverse, OSF, Mendeley Data, Hugging Face and Kaggle.
- **G10, two loggers with their own figures, unlicensed:** njitrocketry/irec26-flight-data @`bcaa7aea` [S3], an
  IREC 2026 flight with a Blue Raven (summary max 11,025.68 ft) and a TeleMetrum (AltOS CSV at 100 Hz, `height` peak
  3,358.3 m, 11,018 ft); with the test flight, 2 flights of 2 families. The test flight's CSV peaks at 2,209 ft
  after the drogue, against its KML's stated 2,136 ft: likely an ejection spike (**Inference**).
- **G11, a stated layout:** Zenodo 17187910 [S7], CC BY 4.0, a payload logger on IST's *Baltasar* at EuRoC 2022;
  the README states every column and unit; 1,019 rows about 0.55 s apart. Its lowest pressure, 68,628.89 Pa, is
  about 3,014 m above the first sample by the 1976 standard atmosphere (**Inference**); the only other record is
  news: 3,000 m [S8] and a 3 km category win [S14]. Partial.

## 3. AltOS telemetry, `.eeprom` and ARTS2 (G3, G4, G5)

- **The eleven undecoded packet types**, from *AltOS Telemetry* [S9] (16 types): 0x01
  TeleMetrum v1, 0x02 TeleMini v1, 0x03 TeleNano, 0x07 companion, 0x08 and 0x12 to 0x14 TeleMega IMU variants,
  0x09 and 0x15 TeleMega Kalman and voltages, 0x11 TeleMini v3. The TeleMega v4.0 has a BMX160 [S10], so 0x12.
- **G3: 5 new `.telem` files, one adds types.** parsa-novin/UGA-IREC @`ebcda936` (unlicensed), a TeleMega file of
  2025-02-08: 0x08 (155 packets) and 0x09 (80) on CRC-good lines, but every 0x09 packet's state is pad. Its 0x08
  pressure, 101,053 to 101,094, fits pascals as the reader corrects the document's unit. The rest are GPS-only.
  The Altus Metrum list's 43 archived months (2010–2015) name 5 `.eeprom` and 2 `.telem` attachments, none kept.
- **G4: 18 new `.eeprom` headers in 6 repositories, none licensed:** EasyMini v1.0 and v3.0 (format 6), v2.0 (14),
  TeleMetrum v2.0 and v4.0 (7). None is format 1, 5, 9, 15 or 19, a TeleMetrum v1 or a TeleMega. No Altus Metrum
  document numbers the log formats, and code search skips large files, so big TeleMega logs can't be found there.
  The EasyMini v3.0 writes format 6, which the [guide's table](../format/flight-logs.md) labels "EasyMini v1"; the
  reader selects by number, so these should still read (**Inference**).
- **G5: nothing new** beyond PSAS's four files; Ozark's 2007 page lists DataAnalyzer 1.6.1, never archived.

## 4. Reader paths, second flights and units (G2, G6, G7, G30, G31)

- **G2: no file reaches an untested path.** 16 public workbooks all use Excel's 1900 dates; no new decimal-comma,
  sub-zero Raven `.FIPa` or RTx file.
- **G6, AltiMAX `.ALS`: 3 flights, unlicensed.** EPFLRocketTeam/SupersonicTestRocket @`3925ed4b`, `data_logs/`:
  Kaltbrunn 2021-06-12 and 2021-09-04 pass the reader's record rule on their first 1,000 records; Cernier
  2021-10-09 on its first 264 only, so likely refused (damage or a strict rule, unknown). Beside Cernier, an
  AltiMAX Excel export re-saved as `.xlsx` has 265 rows matching those 264 records: the first `.ALS`-to-export
  pair after the sample (**Inference**).
  G30, partial: its channel 1 holds 7.91 to 7.99 V, a battery's level, but nothing beside it measures the battery.
- **G6, AIM `.xtra`: the maker's examples, refused.** Entacore's firmware downloads [S11] (`Fw 1.10`, `Fw 1.20`,
  `Fw 2.20`) hold 11 example `.xtra` files, 10 distinct. Each opens with Boost's signature at archive version 8
  (bytes `08 00`); `entacore_raw.rs` requires 16, so all are refused today. No terms stated: cache and cite.
- **G6, RRC3 `.rff` with its export, MIT:** usfsoar/NSL_25-26_EECS @`78e7344e`, `24-25_Code/Launch_Data/
  December_21_2024_Varn/RRC3_Logs/`: an mDACS 1.61 `.rff` and its CSV (2,723 rows at 0.05 s, peak 2,184.3 ft), with
  a team statistics file (apogee 2,176 ft). Both should read today (**Inference**).
- **G7: none in other units.** 14 more distinct Featherweight summaries (12 Blue Raven, 2 tracker) are all feet,
  ft/s, °F and g; one is MIT with its samples beside (RocketryVT/Avionics @`64673ca0`). On njitrocketry's test
  flight the summary's launch time is 1 h after its own low-rate clock's, a time-zone detail for the summary reader.
- **G31, a counter-example:** Volta-Rocketry/CUBESAT-2026 @`eb7e7969`, `CTR/data_files_irec_2025/` (2025 IREC,
  unlicensed). The RRC3 export states no units; its peak, 9,063.1, is feet (the Blue Raven gives 9,104.2 ft AGL),
  and its drogue and main (25.45 s, 113.30 s) match the Blue Raven's (27.0 s, 113.8 s). Its `Temperature` runs
  47.2 to 43.2. The Blue Raven reads 111.8 to 105.8 °F (44.3 to 41.0 °C), the team's board 50.27 to 47.95 °C and a
  payload's sensor 42.03 °C at the pad. As °C the RRC3 sits among them; as °F, the old rule (#852), it is 8.4 °C,
  about 36 K too cold and 26 K below Midland airport's 94 °F that afternoon [S12].
- **G31, the other way:** usfsoar's MIT RRC3 export, also in feet, reads 69.8 to 70.5: as °F, 21.0 to 21.4 °C
  against 60.8 °F (16.0 °C) at Plant City airport, 10.3 km away [S12]; as °C, 54 K above the air. mDACS's own
  preference decides, as its manual says [S13]. Voltages (9.18 to 9.33, 7.60 to 8.58) fit volts, a weak check.

## 5. Real logs for #725's noise check

Profiles: `perfectflite`, `rrc3`, `eggtimer`, `altos`, `imu-500hz`, `gnss-tracker`; 66 files from 16 repositories,
41 measured. Each window loses a least-squares line; σ is the sample standard deviation, lag-1 the residuals'
autocorrelation at one sample, the 1 s spread the σ of the centered 1 s running mean (windows of 3 s or more). A
rest is the tail after the last sample over 3 m from the tail's median, plus 2 s. Synthetic columns re-run each
profile's noise over windows near the real ones (5 s, 15 s, 0.27 s, 1.8 s down the rows): medians of 400 draws,
σ's 10th–90th percentiles within 0.47–1.14 m.

| Profile, rate | Window (s); logs | Real σ (m) | Real lag-1 | Real 1 s | Synth. σ | lag-1 | 1 s |
|---|---|---|---|---|---|---|---|
| perfectflite, 20 Hz | rest 3.6–7.8; 17 | 0.13–0.25 (0.18) | −0.07 to 0.34 | 0.02–0.10 | 0.53 | 0.22 | 0.14 |
| altos, 10 Hz | rest 13–19; 3 | 0.30–0.64 (0.58) | 0.20–0.37 | 0.10–0.27 | 0.82 | 0.07 | 0.30 |
| altos, 100 Hz | pad 0.11–0.25; 5 | 0.17–1.99 | −0.36 to 0.10 | none | 0.78 | 0.11 | none |
| EasyMini (altos), 10 Hz | rest 11–15; 3 | 0.20–0.40 | −0.02 to 0.19 | 0.08–0.09 | as altos | | |
| imu-500hz, baro 50 Hz | pad 1.4; 2 | 0.43, 0.53 | 0.40, 0.53 | none | 0.59 | 0.37 | none |
| imu-500hz, baro 50 Hz | rest 4.3; 1 | 0.71 | 0.48 | 0.06 | 0.59 | 0.42 | 0.13 |
| rrc3, eggtimer, 20 Hz | none usable | none | none | none | 0.61, 0.62 | 0.16, 0.18 | 0.16 |

- **How loggers cut their files, counted.** AltOS eeprom exports start 0.30 s before boost (9 of 9), EasyMini 0.14
  to 0.62 s, AltosUI telemetry exports 1.08 to 1.97 s; none holds a pad-state row. The Blue Raven's low-rate CSV
  starts 1.90 to 1.92 s before liftoff (2 flights). PerfectFlite and RRC3 files start at launch detection (0.45 s
  or less before motion), Eggtimer's with none. The synthetic log draws 2 to 10 s of pad, longer than any public
  barometric file here, which gives an analyzer a better pad zero than a real file allows (**Inference**).
- **Two license-clear logs per family, on rests only:** PerfectFlite (UND-ARC's MIT StratoLogger SL100 files) and
  TeleMega-class (werocketry/Hyperion-II and RocketPy's Prometheus data, MIT). The Blue Raven has two MIT pads of
  1.4 s (Hyperion-II, rex-mcall/Shred-O-Vision) but one rest; RRC3 and Eggtimer none, as their files end at or
  before landing. A Featherweight GPS tracker (Lmx154/RL-RocketPy-Notebook, MIT) holds a 3.0 s pad, but its
  altitude barely moves (σ 0.03 ft after the line), held between fixes (**Inference**): no test of the 3 m assumed.
- **The Blue Raven's correlation is a two-row hold.** From 5 to 20 s (751 rows) its baro column repeats every other
  row: 74% of one parity's consecutive pairs are equal, against 0% and 7% of the other, in both MIT flights. At
  25 Hz its noise is white (lag-1 −0.19 to +0.07). The fast correlated term fitted to these logs is applied to every
  profile, giving the 100 Hz altos profile a lag-1 of 0.71 for that term; four of five real AltOS pads read negative,
  but on 12 to 26 samples, so that sign is weak evidence (**Inference**).
- **Ejection pulses.** The synthetic pulse dips to −16 m and peaks at +32 m. AltOS logs (11) stay inside it (peaks
  +1.5 to +14.7 m, dips to −16.0 m). 7 of 17 PerfectFlite logs dip below −16 m for one or two samples, down to
  −1,503 m; 6 of the 7 are UND's MIT files, whose backups, firing later, dip only −0.6 to −4.3 m. One Blue Raven
  log dips −88 m and one Eggtimer log −21 m: 9 of 34 logs pass the synthetic dip (10 of 35 with a dropped Quasar
  file that jumps to ground level after apogee, −265 m).
- **Accelerometers.** One MIT Blue Raven high-rate pad (491 samples) gives 0.0047 to 0.0056 g per axis, inside the
  profile's 3.2 to 7.9 mg; AltOS pre-boost windows of 14 to 26 samples are too short to rank.

## 6. Row by row

| Row | Best new find | Terms | Closes? | Reads today |
|---|---|---|---|---|
| G1 | Forum GPX beside a `.pf2` | none, login | no (unreachable) | `.pf2` yes |
| G2, G5 | none | | no | |
| G3 | TeleMega `.telem` with 0x08, 0x09 | none | no (pad only) | 0x08, 0x09 no |
| G4 | 18 `.eeprom`, formats 6, 7, 14 | none | no | yes (**Inference**) |
| G6 | 3 `.ALS` + export; 11 `.xtra`; `.rff` + CSV | none; none; MIT | partly | `.xtra` refused |
| G7 | 15 summaries, all feet | 1 MIT | no | yes |
| G8 | TeleMega KML, both heights | MIT | partly | KML yes |
| G9 | 16 Blue Jay flights, cleaned | Apache-2.0 | partly | as Blue Raven (**Inference**) |
| G10 | Blue Raven + TeleMetrum, 2 flights | none | cite only | yes |
| G11 | Baltasar payload log | CC BY 4.0 | partly (news only) | by header, maybe |
| G30 | Cernier export voltages | none | partly | yes |
| G31 | RRC3 °C in a feet file; °F in another | none; MIT | shows the rule wrong | as written (#852) |

## What this suggests

1. **Stop deriving an RRC3's temperature unit from its altitude unit.** Keep an unbracketed `Temperature` as
   written, unrecognized, or decide it from the pad value against an air range; pin both files' readings in a test
   and change the guide's [units section](../format/flight-logs.md#units-no-source-states).
2. **Read Entacore's archive version 8** once its record layout is checked on the maker's 10 distinct examples.
3. **Word #725 for rests**, where PerfectFlite and TeleMega-class reach two MIT logs, and draw the synthetic pad
   per profile from 0.3 to 2 s, as real files are cut.
4. **Model the Blue Raven as a two-row hold over white 25 Hz noise**, drop the 29 ms correlation from the other
   profiles, and widen the ejection dip or add a one-to-two-sample spike in about a quarter of flights: each an
   ADR-280 change citing these counts. List libpifly's KML (G8) and the Blue Jay set (G9) under their rows.

## Open questions

- Is a rest noisier than a pad (no long pad record is public)? Are the StratoLogger dips charge gas or a supply dip?
- Not found: a TeleMetrum v1 `.eeprom`, a TeleMega v4 with its IMU running, more format 5, 9 or 19 files, an ARTS2
  at another rate, and a phone track of a rocket.

## Sources

All read 2026-10-20; licenses from GitHub's API. GitHub repositories are pinned by the commits above; §5's are
werocketry/Hyperion-II @`efda132f`, rex-mcall/Shred-O-Vision @`619af886`, RocketPy-Team/RocketPy @`9bd6ad3a`,
UND-ARC/RocketModels-2023 @`922c084f`, UND-ARC/Nasa-SLI-2024 @`97bee76e`, Lmx154/RL-RocketPy-Notebook @`80c9cd6a`.

- [S1] Rocketry Forum thread 135349, "EggfinderTx --> Mapsphere, In-flight Position and Altitude Peformance?",
  <https://www.rocketryforum.com/threads/135349/>
- [S2] <https://github.com/robertrau/libpifly>. [S3] <https://github.com/njitrocketry/irec26-test-data>,
  <https://github.com/njitrocketry/irec26-flight-data>. [S4] Rocketry Forum thread 142266 (2017). [S5]
  <https://www.kaggle.com/datasets/evelyn81/unguided-rocket-flight-telemetry-datasets>.
- [S6] Featherweight, *Blue Jay User's Manual*, 2026-05-12. [S7] <https://zenodo.org/records/17187910>.
- [S8] <https://ptspace.pt/?p=74708>. [S14] Técnico DEM news, 2022, on dem.tecnico.ulisboa.pt (the EuRoC win).
- [S9] Altus Metrum, *AltOS Telemetry* (built 2025-11-17), <https://altusmetrum.org/AltOS/doc/telemetry.pdf>; [S10]
  *The Altus Metrum System*, v1.9.22.2, 2025-10-13, pp. 78–79. Both under Creative Commons ShareAlike 3.0.
- [S11] Entacore downloads, <https://entacore.com/downloads/>, <https://downloads.entacore.com/aimxtra/>.
- [S12] Iowa Environmental Mesonet, ASOS records for MAF (2025-06-12) and PCM (2024-12-21). [S13] Missile
  Works, *mDACS User Manual*, rev. 1.71, p. 5; *RRC3+/RRC3 User Manual*, rev. 1.71, p. 12.
