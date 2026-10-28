# Which GNSS fields hobby trackers' logs record

This note asks which hobby trackers' logs keep what a GNSS trust test needs: the receiver's vertical speed, fix
type, satellite count, an accuracy figure (DOP, or hAcc and vAcc in meters), the epoch's own time, the rate, and
how that time ties to the barometer's clock. It serves [M7.2](../ROADMAP.md)'s GNSS fusion rule, which would trust
GNSS altitude once its vertical speed agrees with the barometer's in coast ([gnss-in-boost](gnss-in-boost.md) §5),
and issue [#717](https://github.com/nrdptel/fusionspace-eridanus/issues/717)'s survey. It builds on
[the Eggfinder forum flights](forum-eggfinder-gpx-vs-baro.md), [log formats](debrief-log-formats.md) and
[the CATS and Juno III samples](sample-logs-cats-juno.md). Sources: makers' manuals, Altus Metrum's telemetry
document, u-blox's interface description, the GPS performance standard, public logs, and counts over the public
and private sample logs under `refs/` (aggregates only). **Inference** marks this note's reasoning. Read 2026-10-28.

**In short.** Of the logs read here, two families keep a receiver's vertical speed: Featherweight's tracker log
(`VERTV`, whole ft/s, 10 Hz to apogee) and Altus Metrum's TeleMega and TeleGPS flash logs and telemetry (climb,
cm/s, 1 Hz); Juno III's GNSS table has the same columns as Featherweight's. None keeps hAcc or vAcc, and none puts
a barometer on the GNSS epoch's clock: Featherweight's tracker has no barometer, and in one Altus flight the GPS
record's tick sat about 0.6 s before the moment its values describe. AltosUI's CSV export drops the climb (0 of
59 files with GPS). Eggtimer, Big Red Bee and standard NMEA give only 1 Hz altitude to difference. Featherweight's
`VERTV` and its own altitude rate differ by a median 1.3 to 1.5 m/s once shifted 0.1 to 0.7 s, more in 3 files.

## 1. What each log keeps

| Log | Vertical speed | Fix; satellites; accuracy | Epoch time; rate | Barometer's clock |
|---|---|---|---|---|
| Featherweight tracker | `VERTV`, whole ft/s | `FIX` always 3; `#SATS`; none | `UTCTIME` 1 ms; 10, 1 Hz | none |
| Featherweight station | `VERTV`, relayed | `FIX` 0 or 3; `#TOT`; none | receive time only; 1 Hz | none |
| Altus TeleMega, TeleGPS log | climb, cm/s | valid bit; used; P, H, VDOP | UTC, whole s; 1 Hz | one tick, mixed |
| Altus `.telem` | climb, cm/s | valid bit; used; P, H, VDOP | UTC, whole s; 1 Hz | packet's tick |
| Altus TeleMetrum v2+ log | none | valid bit; used; none | UTC, whole s; 1 Hz | one tick, interleaved |
| AltosUI CSV | none; 1 m altitude | `locked`; `nsat`; P, H, VDOP | whole s; held between fixes | one time column |
| Eggtimer TRS (NMEA) | none; 0.1 m altitude | GGA, GSA; count; DOPs | GGA `.000`; 1 Hz | line order only |
| Eggtimer Quasar | none | in its NMEA; DOPs | GMT in NMEA; about 1 Hz | 100 ft steps |
| BRB900 `$BRBTX` | none | none; used; H, VDOP | UTC, whole s; every N s | no barometer |
| BRB900L `$BRLTX` echo | none (`vel` unclear) | none; in view; `vacc` | none; 1 Hz log | `alt` may blend |
| Juno III `cots_GNSS.csv` | `VERTV`, whole ft/s | none | none, own zero; 10, 1 Hz | separate file |

## 2. Featherweight GPS tracker

A u-blox M10 computing 10 solutions a second, to 80 km and 500 m/s [S2]. The tracker logs 10 samples a second on
ascent and 1 after apogee, up to 6,144, "only data with a valid GPS fix" [S1] p. 16, and sends one packet a second
to the ground station's log [S1] pp. 14 to 15; fields [S1] pp. 16 to 17 and hpr's `fwgps.rs`. Satellite counts
come with counts above 24, 32 and 40 dB-Hz [S1] p. 10; `FLAGS`, "GPS quality flags", is not defined. Neither the
Blue Raven [S4] nor the Swift (no flight log as of 2026-04-09, [S3] p. 15) logs GNSS. Counts over the distinct
public and private files, by SHA-256:

- **Tracker logs (31 files, 11,780 rows).** Every `UTCTIME` falls on a tenth of a second (9,124) or 1 ms before one
  (2,656): the receiver's 10 Hz epoch, truncated (**Inference**), not a logger's receive time. `FIX` is 3 on every
  row, because unfixed rows are never written; a loss of fix shows only as a gap (196 steps over 1 s).
- **Ground-station logs (13 files, 37,397 rows).** Only 600 times (1.6%) fall on a tenth of a second, near the 1%
  of chance: a receive clock (**Inference**). The radio's latency is unmeasured.
- **`VERTV` is not the log's differenced altitude.** Over 1 s windows of 10 Hz rows above 30 ft/s, two passes with
  different window rules (18 and 36 files) put the median per-file gap at 2.2 to 3.6 m/s unshifted and 1.3 to
  1.5 m/s at each file's best shift, of 0.1 to 0.7 s in most files; 3 files stay over 5 m/s shifted.
  **Inference:** a separate velocity solution, likely the receiver's, on a slightly different time.

**Can the test run?** Yes, on the tracker log, against another altimeter's log aligned by liftoff (detected from
GPS [S1] p. 16, so a late fix moves it). Steps of 0.30 m/s, no vAcc. Station logs: 1 Hz, latency.

## 3. Altus Metrum

- **Receivers** ([S14] Table 12, p. 78; [S16] Table 2, p. 23): TeleMetrum v1 SkyTraq, v2 u-blox MAX-7Q, v3 MAX-8Q,
  v4 MAX-8C or MAX-10S; TeleMega v1 to v3 MAX-7Q, v4 to v6 MAX-8Q, v7 MAX-10S; TeleGPS v1 and v2 MAX-7Q, v3 and v4
  MAX-10S. EasyMega has no GPS. No document says whether the firmware reads NMEA or UBX, or which dynamic model.
- **Telemetry** (packet 0x05, [S15] Tables 15 and 16, pp. 6 to 7): altitude (int16, m), position, date and time to
  the second, PDOP, HDOP and VDOP (each times 5), ground speed (cm/s), **climb rate (int16, cm/s)**, course; flags
  hold satellites used, valid, and a course-valid bit covering speed and climb. Sent at 1 Hz in every phase [S15]
  p. 6, each header carrying the board tick in 1/100 s [S15] Table 1, p. 2.
- **Flash log.** Sensors at 100 Hz in ascent and 10 Hz in descent [S14] App. F, p. 77; TeleGPS logs every 1 s by
  default [S16] §2.2, p. 4. Altus publishes no record layout; hpr's `altos_eeprom.rs` worked one out from samples:
  a 32-byte GPS record (TeleMega, TeleGPS) holds position, UTC, flags, speed, climb and dilutions; a TeleMetrum v2
  record has no speed, climb or dilution. Of 39 distinct `.eeprom` files, 19 hold GPS records, about 1 s apart.
- **The logged climb is cm/s.** Over 1 s steps that climb more than 5 m, the median of altitude change over mean
  climb is 1.001 to 1.014 in each of 4 public Mega-style logs (33 to 306 steps) and 1.002 and 1.003 in 2 private
  ones. hpr keeps `climb_rate` as written, its unit unchecked (`altos_eeprom.rs`, `altos_telem.rs`).
- **AltosUI CSV** ([S14] §12.5.1, p. 37, lists no columns). Of 106 distinct AltOS CSVs, 59 carry GPS: all have
  `locked`, `nsat`, `hdop` and a time, 58 `pdop`, `vdop` and a GPS altitude. **None has climb, ground speed,
  course, hAcc or vAcc.** GPS time is whole seconds and altitude whole meters (57 of 58), held until the next fix.
- **Clocks.** GPS and sensor records share one tick counter. In three public flight logs tick minus UTC second
  stays within 0.04 s all flight: a fixed point of the cycle, how far from the epoch no document says.

**One flight with both files.** A public TeleMega v6 booster flight [S21] has the flash log and its CSV. Over 15
coast fixes the climb at its tick reads 4.6 to 11.4 m/s below AltosUI's `speed` estimate; matched 0.52 s later the
rms gap falls from 8.7 to 2.8 m/s. GPS altitude against the CSV's pressure, the barometer alone, fits best 0.63 s
later (rms 15 m). Over 227 drogue fixes the two speeds agree, median −0.1 m/s. **Inference:** here the GPS values
describe a moment 0.5 to 0.7 s after their tick, or the barometer lags that much (unlikely); one flight can't tell.
At face value GNSS looks slow while decelerating, so the test errs toward holding GNSS out: at 10 to 30 m/s² of
coast deceleration, 0.5 to 0.7 s is 5 to 21 m/s. Unbounded across boards and firmware.

**Can the test run?** Yes on TeleMega and TeleGPS flash logs and any `.telem`, once the climb is read in cm/s and
the tick offset is carried as unknown, 0 to 0.7 s. On CSVs and TeleMetrum logs only by differencing 1 m altitude.

## 4. Eggtimer Rocketry

- **Eggfinder TX and Mini.** The TX sends its module's NMEA once a second at 9600 baud [S5] pp. 6, 10; the TX, not
  the Mini, takes an OpenLog logger [S9]. Otherwise a log is the receiving computer's: the RX dongle (discontinued
  April 2026 [S9]) or an app on the LCD receiver's Bluetooth [S8] pp. 5, 19; the LCD keeps every 10th valid fix
  [S8] p. 15. No guide names the module; the maker named a SiRFstarIV A2235-H in 2016 [S10].
- **The one raw TRS stream** ([S10], 6,032 s). Per second: GGA, GSA, GSV (one second in five), RMC, all `GP`, no
  VTG, then `{  }` and `<n>`. Every good GGA and RMC time ends `.000`; GGA altitude is in 0.1 m on all 3,979 good
  sentences; HDOP 0.7 to 50, VDOP 1.1 to 50 (50 without a fix). `<n>` is likely the AGL altitude in feet the
  receiver shows [S7] p. 55, switching to the apogee after landing [S7] p. 49 (**Inference**). It has no time of
  its own: 3,877 of 4,129 sit between one second's RMC and the next second's GGA, so it pairs with a GNSS second
  only by line order, a ±1 s ambiguity no document settles.
- **Quasar, Proton, CSVs.** Armed, a Quasar radios its NMEA "which includes GMT time" and AGL altitude "in 100'
  increments", about once a second [S6] pp. 13, 33, 43; OpenLog pads can record it [S6] p. 8. Its CSV and the
  TRS's hold no GPS [S6] p. 40, [S7] p. 35; a Proton radios altitude, velocity, status every 2 s [S8] p. 18.

**Can the test run?** Roughly, on a TRS stream, GGA differenced at 1 Hz against `<n>` (**Inference:** 1 s of
pairing error is 100 m in a 100 m/s coast). Not on a Quasar's: a 30.5 m step makes a 1 s baro speed meaningless.

## 5. Big Red Bee

- **BRB900L and BRB433L (current).** "u-blox SAM10" modules (likely the SAM-M10Q, **Inference**) and a barometer
  [S11] p. 1; a 1 Hz log from liftoff, format "TBD" [S11] p. 3. The receiver's `$BRLTX` echo gives position, MSL
  altitude, `vel` ("ground speed or velocity"), `vacc`, satellites in view: no time, no fix type [S11] pp. 6 to 7.
  GPS calibrates the barometer and the app's altitude uses both [S11] pp. 4, 12: `alt` may blend (**Inference**).
- **BRB900 (older).** A u-blox NEO-6Q, no barometer; `$BRBTX` gives the fix's UTC (whole seconds), satellites
  used, HDOP, altitude and VDOP every N s; raw NMEA can be echoed [S12] pp. 1 to 4, 6, 9.
- **BeeLine GPS.** Records position, altitude, GPS time and satellites in view, about 2.5 h at 1 Hz [S13] pp. 3, 8.

**Can the test run?** Not as documented: no vertical speed, no barometer on the older units, no BRB900L sample.

## 6. Juno III, CATS, and what NMEA and UBX carry

- **Juno III** (RocketPy, MIT [S18]): `time,ALT,HORZV,VERTV,LAT,LON`, 444 rows, "a log from the COTS GNSS sensor";
  `ALT` whole feet above sea level, `VERTV` whole ft/s (by check), 10 Hz to 29.7 s and 1 Hz from 30.4 s; no fix
  type, satellites, dilution or UTC. Nothing names the device. Its clock has its own zero: `VERTV` fits the RRC3's
  velocity best shifted about 1.45 s (rms 1.8 to 2.0 m/s), so clock offset and receiver lag are one shift here. In
  boost (2 to 3 s) `VERTV` reads −36 to +16 m/s against the RRC3's 123 to 198 m/s: a format sample only.
- **CATS Vega.** The paper lists an IMU and an MS5607 [S19] §2.1, but the board needs a radio-transparent airframe
  "for good GNSS reception" (§2.5) and the ground station shows its GNSS coordinates (§3.2); the maker lists a
  MAX-M10S [S24]. Which GNSS fields its flash log keeps is undocumented, and no CATS table read here has one.
- **NMEA 0183.** GGA: fix UTC (hhmmss.ss), quality, satellites used, HDOP, MSL altitude ([S17] pp. 24 to 25,
  [S22]). GSA: fix mode, PDOP, HDOP, VDOP, no time. RMC: UTC, status, ground speed, course. None of the standard
  sentences [S22] lists has a vertical speed (**Inference** from the list); u-blox's `PUBX,00` has one (`vVel`,
  positive down) with hAcc and vAcc, off by default [S17] pp. 36 to 37, 164.
- **UBX-NAV-PVT** [S17] pp. 98 to 99: iTOW (the epoch, ms), fixType, numSV, hAcc and vAcc (mm), velD (positive
  down), sAcc (mm/s); off by default [S17] p. 166. No hobby log read here keeps it.

**Speed from differenced altitude (Inference throughout).** Rounding to a step q adds σ = q/√6 to a 1 s
difference: 0.04 m/s at 0.1 m, 0.41 m/s at Altus's 1 m. Independent 5 m (95%) epoch errors [S20] p. v would give
σ = √2 × 5/1.96 = 3.6 m/s, an upper bound if, as is likely, most of the error drifts slowly. Measured on 1 m Altus
data: robust σ 0.58 m/s over [S21]'s 288 slow steps, 2.1 m/s on a public TeleGPS v2 sample. Expect about 0.3 to
2 m/s at 1 Hz, against 0.05 m/s for a receiver's own velocity (50% at 30 m/s, [S23] Table 1, p. 4). A 1 s
difference is centered 0.5 s before the later fix: stamped at that fix it reads high by 0.5 s times the
deceleration, 5 to 15 m/s in a 10 to 30 m/s² coast. Stamp it at the midpoint.

## What this suggests

- **M7.2's rule.** Build and test the speed check on Featherweight tracker logs and Altus TeleMega and TeleGPS
  logs. Fit or carry a GNSS time offset (0.1 to 0.7 s seen on both families), and keep a height check beside the
  speed check: 3 Featherweight files disagree with themselves by over 5 m/s even after a shift.
- **hpr's AltOS readers.** Read `climb_rate` as vertical speed in cm/s, checked per file against the differenced
  altitude as `fwgps.rs` checks `VERTV`. Until then the speed test can't run on any Altus log through hpr.
- **#717's survey.** Per format, report whether the speed is logged or differenced, whether the time is the
  epoch's or a receive clock's, and the accuracy field (none, DOP, vAcc); difference at the midpoint.

## Open questions

- Which instant an AltOS GPS record's tick marks (0.5 to 0.7 s on one flight); a climb over int16's 327.67 m/s.
- Whether `VERTV` is the receiver's velD; `FLAGS`; whether `#SATS` counts used or tracked satellites; the ground
  station's latency (needs a tracker and station log of one flight).
- The TRS `<n>`'s epoch; current Eggfinder modules; the BRB900L log format, `vel`'s meaning and `vacc`'s unit.
- Which GNSS fields the CATS Vega logs; Juno III's GNSS device and how its zero was set against the RRC3's.

## Sources

Read 2026-10-28; PDFs cached under `refs/research/gnss-log-fields/` or `refs/research/gnss-in-boost/`. Cited,
not reproduced: no maker's manual states a license, u-blox allows reproduction only with its permission, [S21]
has no license; Altus documents are Creative Commons ShareAlike 3.0, [S18] MIT, [S19] arXiv. FW is
`https://www.featherweightaltimeters.com/uploads/1/0/9/5/109510427/`, ET `https://eggtimerrocketry.com/wp-content/`
`uploads/`, BRB `https://cdn.shopify.com/s/files/1/0062/3919/1158/files/`, UB `https://content.u-blox.com/sites/`
`default/files/`.

- [S1] Featherweight, *GPS Tracker User's Manual*, 2025-02-20, FW `gps_tracker_manual_2025feb20b.pdf`.
- [S2] Featherweight, GPS tracker page, <https://www.featherweightaltimeters.com/original-gps-tracker.html>.
- [S3] Featherweight, *Swift User's Manual*, 2026-04-10, FW `swift_users_manual_apr_10_2026.pdf`.
- [S4] Featherweight, *Blue Raven User's Manual*, 2026-05-12, FW `blue_raven_users_manual_2026may12.pdf`.
- [S5] Eggtimer, *Eggfinder User's Guide* Rev B6, ET `2018/06/Eggfinder_Users_Guide_RevB6.pdf`.
- [S6] Eggtimer, *Quasar User's Guide* 1.02S, ET `2024/02/Eggtimer-Quasar-1_02s.pdf`.
- [S7] Eggtimer, *TRS User's Guide*, ET `2018/06/Eggtimer_TRS_Users_Guide_2_07.pdf`.
- [S8] Eggtimer, *Eggfinder LCD Receiver User's Guide* 2.05a, ET `2025/01/Eggfinder-LCD-Users-Guide-2_05a.pdf`.
- [S9] Eggtimer, Eggfinder page, <https://eggtimerrocketry.com/home/eggfinder-gps-tracking-system/>.
- [S10] Rocketry Forum thread 135349, <https://www.rocketryforum.com/threads/135349/>, post 28 and its files.
- [S11] Big Red Bee, *BRB900L and BRB433L GPS Tracker User Manual* v0.10, BRB `BRB900_User_Manual_v0.4.pdf`.
- [S12] Big Red Bee, *BRB900* manual (2013), BRB `brb_lcd6.pdf`.
- [S13] Big Red Bee, *GPS User's Guide* V0.20, December 2024, BRB `beelineGPS_20.pdf`.
- [S14] Altus Metrum, *The Altus Metrum System*, v1.9.22.2, <https://altusmetrum.org/AltOS/doc/altusmetrum.pdf>.
- [S15] Altus Metrum, *AltOS Telemetry*, v1.9.22.2, <https://altusmetrum.org/AltOS/doc/telemetry.pdf>.
- [S16] Altus Metrum, *TeleGPS Owner's Manual*, v1.9.22.2, <https://altusmetrum.org/AltOS/doc/telegps.pdf>.
- [S17] u-blox, *M10 SPG 5.10 Interface Description* UB `u-blox-M10-SPG-5.10_InterfaceDescription_UBX-21035062.pdf`.
- [S18] RocketPy, Juno III data, <https://github.com/RocketPy-Team/RocketPy/tree/master/data/rockets/juno3>.
- [S19] Binz, Stojoski, Jost, "CATS: Empowering the next generation of rocket scientists through educational
  flight computers", 26th ESA PAC Symposium (2024), <https://arxiv.org/abs/2511.04725>.
- [S20] US DoD, *GPS SPS Performance Standard*, 5th ed., April 2020,
  <https://www.gps.gov/sites/default/files/2025-07/2020-SPS-performance-standard.pdf>.
- [S21] ISSUIUC, a TeleMega log and CSV, <https://github.com/ISSUIUC/flight-data/tree/master/20240323>.
- [S22] E. S. Raymond, *NMEA Revealed*, gpsd, <https://gpsd.gitlab.io/gpsd/NMEA.html>.
- [S23] u-blox, *MAX-M10S Data Sheet*, UBX-20035208 R08, UB `MAX-M10S_DataSheet_UBX-20035208.pdf`.
- [S24] CATS, Vega product page, <https://www.catsystems.io/vega>.
