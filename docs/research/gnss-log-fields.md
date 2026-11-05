# Which GNSS fields hobby trackers' logs record

Re-checked 2026-11-04: 14 claims corrected, 4 sharpened, §7's datums added; sources read 2026-10-28 and 2026-11-04.

Which hobby trackers' logs keep what a GNSS trust test needs: vertical speed, fix type, satellites, an accuracy
figure (DOP, hAcc, vAcc), the epoch's own time and rate, the tie to the barometer's clock, and the altitude datum.
It serves [M7.2](../ROADMAP.md)'s rule (trust GNSS altitude once its vertical speed agrees with the barometer's in
coast, [gnss-in-boost](gnss-in-boost.md) §5) and [#717](https://github.com/nrdptel/fusionspace-eridanus/issues/717)'s
survey, building on [the Eggfinder forum flights](forum-eggfinder-gpx-vs-baro.md), [log formats](debrief-log-formats.md)
and [the CATS and Juno III samples](sample-logs-cats-juno.md). Private logs give aggregates only; **Inference** marks
reasoning.

**In short.** Two families keep a receiver's vertical speed: Featherweight's tracker log (`VERTV`, whole ft/s,
10 Hz to apogee) and Altus's TeleMega and TeleGPS logs and telemetry (climb, cm/s, 1 Hz); Juno III's table shares
`VERTV` but no UTC, fix or satellite column. None keeps hAcc or vAcc or puts a barometer on the GNSS epoch's clock
(one Altus flight's climb likely describes a moment 0.5 s after its tick). AltosUI's CSV drops the climb (0 of 60
files with GPS); Eggtimer, Big Red Bee and NMEA give only 1 Hz altitude. `VERTV` and the altitude rate differ by a
median 1.5 m/s once shifted. Logs that state a datum say above sea level; Altus states none; `PUBX,00` is ellipsoidal.

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
ascent and 1 after apogee, up to 6,144, "only data with a valid GPS fix" [S1] p. 16, and sends 1 packet a second to
the station [S1] pp. 14 to 15; fields [S1] pp. 16 to 17, hpr's `fwgps.rs`. Satellite counts come with counts above
24, 32 and 40 dB-Hz [S1] pp. 10, 17; `FLAGS` undefined. Blue Raven [S4] and Swift ([S3] p. 15, 2026-04-09) log no GNSS.
Counted 2026-10-28 over files under `refs/` (not `scratch/`) whose first line opens `UTCTIME,UNIXTIME` (tracker) or
`TRACKER,DATE,TIME` (station), distinct by SHA-256:

- **Tracker logs (30 files, 11,414 rows).** Every `UTCTIME` falls on a tenth of a second (8,758) or 1 ms before one
  (2,656): the receiver's 10 Hz epoch, truncated (**Inference**), not a receive time. `FIX` is 3 on every row, as
  unfixed rows are never written; a loss of fix shows only as a gap (186 steps over 1 s).
- **Station logs (13 files, 37,397 rows).** Only 600 times (1.6%) fall on a tenth of a second, near the 1% of
  chance: a receive clock (**Inference**). The radio's latency is unmeasured.
- **`VERTV` is not the differenced altitude.** Over 1 s windows of 10 Hz rows above 30 ft/s (18 files with 20 or more
  windows), the median per-file gap is 3.6 m/s unshifted, 1.5 m/s at each file's best shift (0 in 6 files, 0.1 to
  0.7 s in 12), over 5 m/s in 3. **Inference:** a separate velocity, likely the receiver's, often a little offset.
- **Can the test run?** Yes, on the tracker log, against another altimeter's log aligned by liftoff (detected from
  GPS [S1] p. 16, so a late fix moves it). Steps of 0.30 m/s, no vAcc. Station logs: 1 Hz, latency.

## 3. Altus Metrum

- **Receivers** ([S14] Table 12, p. 78; [S16] Table 2, p. 23): TeleMetrum v1 SkyTraq, v2 u-blox MAX-7Q, v3 MAX-8Q,
  v4 MAX-8C or MAX-10S; TeleMega v1 to v3 MAX-7Q, v4 to v6 MAX-8Q, v7 MAX-10S; TeleGPS v1 and v2 MAX-7Q, v3 and v4
  MAX-10S. EasyMega has no GPS. No document says whether the firmware reads NMEA or UBX, or which dynamic model.
- **Telemetry** (packet 0x05, [S15] Tables 15, 16, pp. 6 to 7): altitude (int16, m), position, time to the second,
  P, H and VDOP (times 5), ground speed (cm/s), **climb rate (int16, cm/s)**, course; flags: satellites used, valid,
  course-valid (speed, climb). 1 Hz in every phase [S15] p. 6; each header has the tick, 1/100 s [S15] Table 1, p. 2.
- **Flash log.** Sensors at 100 Hz in ascent, 10 Hz in descent [S14] App. F, p. 77; TeleGPS every 1 s by default
  [S16] §2.2, p. 4. hpr's `altos_eeprom.rs` found the unpublished layout from samples: a 32-byte GPS record (TeleMega,
  TeleGPS) with position, UTC, flags, speed, climb, dilutions; TeleMetrum v2's lacks the last three. 19 of 39 distinct
  `.eeprom` files hold GPS records, about 1 s apart.
- **The logged climb is cm/s.** Over 1 s steps climbing more than 5 m, the median of altitude change over mean
  climb is 1.001 to 1.022 in each of 4 public Mega-style logs (33 to 306 steps; the TeleGPS v2 log's 1.018 to 1.022
  on this note's decoding, **Inference**) and 1.002 and 1.003 in 2 private ones. hpr keeps `climb_rate` with no unit.
- **AltosUI CSV** ([S14] §12.5.1, p. 37, lists no columns). Counted 2026-11-04: of 106 distinct AltOS CSVs (by the
  `#version,serial,flight` header), 60 carry GPS, all with `locked`, `nsat`, `pdop`, `hdop`, `vdop`, a time, and 59
  a GPS altitude. **None has climb, ground speed, course, hAcc or vAcc.** Time 1 s, altitude 1 m in 58 of 59, held.
- **Clocks.** GPS and sensor records share one tick counter. In three public flight logs tick minus UTC second
  stays within 0.04 s all flight: a fixed point of the cycle, how far from the epoch no document says.

**One flight with both files.** A public TeleMega v6 booster flight [S21] has the flash log and its CSV. Over 15
coast fixes the climb at its tick reads 4.6 to 11.4 m/s below AltosUI's `speed`; matched 0.52 s later the rms gap
falls from 8.7 to 2.8 m/s. Against the CSV's pressure altitude, GPS altitude is best 0.63 s later in an offset-only
fit (rms 15 m) but 0.26 s earlier with a gain: weak. Over 227 drogue fixes the speeds agree, median −0.1 m/s.
**Inference:** the climb describes a moment about 0.5 s after its tick, or the barometer lags (unlikely). GNSS then
looks slow while decelerating, so the test errs toward holding it out: at this CSV's 10 to 38 m/s² of coast, 0.5 to
0.7 s is 5 to 26 m/s. **Can the test run?** Yes on TeleMega and TeleGPS flash logs and any `.telem`, the climb read
in cm/s, tick offset unknown (0 to 0.7 s here, unbounded elsewhere); on CSVs, TeleMetrum logs only by differencing 1 m.

## 4. Eggtimer Rocketry

- **Eggfinder TX and Mini.** The TX sends its module's NMEA once a second at 9600 baud [S5] pp. 6, 10; the TX, not
  the Mini, takes an OpenLog [S9]; else the RX dongle (gone April 2026 [S9]) or an app on the LCD receiver's
  Bluetooth logs it [S8] pp. 5, 19, the LCD every 10th fix [S8] p. 15. Module in 2016: an A2235-H ([S10] post 28).
- **The one raw TRS stream** ([S10] post 21, 6,032 s). Per second: GGA, GSA, GSV (one second in five), RMC, all
  `GP`, no VTG, then `{  }` and `<n>`. Good GGA and RMC times end `.000`; GGA altitude is in 0.1 m on all 3,979 good
  sentences; HDOP 0.7 to 50, VDOP 1.1 to 50. `<n>` is likely the AGL feet the receiver shows [S7] p. 55, the apogee
  after landing [S7] p. 49 (**Inference**). Untimed (3,877 of 4,129 sit between one second's RMC and the next GGA),
  it pairs with a GNSS second by line order, ±1 s; a poster says it is placed "asynchronously" ([S10] post 22).
- **Quasar, Proton, CSVs.** Armed, a Quasar radios its NMEA "which includes GMT time" and AGL altitude "in 100'
  increments", about once a second [S6] pp. 13, 33, 43; OpenLog pads can record it [S6] p. 8. Its CSV and the
  TRS's hold no GPS [S6] p. 40, [S7] p. 35; a Proton radios altitude, velocity, status every 2 s [S8] p. 18.
- **Can the test run?** Roughly, on a TRS stream, GGA differenced at 1 Hz against `<n>` (**Inference:** 1 s of
  pairing error is 100 m in a 100 m/s coast). Not on a Quasar's: a 30.5 m step makes a 1 s baro speed meaningless.

## 5. Big Red Bee

- **BRB900L and BRB433L (current).** "u-blox SAM10" (likely SAM-M10Q, **Inference**), a barometer [S11] p. 1; a 1 Hz
  log, format "TBD" [S11] p. 3. The `$BRLTX` echo: position, MSL altitude, `vel` ("ground speed or velocity"), `vacc`,
  satellites in view; no time or fix type [S11] pp. 6 to 7. MSL altitude uses "the barometric sensor and GPS" [S11]
  pp. 4, 12: `alt` may blend (**Inference**). No sample read.
- **BRB900 (older).** A u-blox NEO-6Q, no barometer; `$BRBTX` gives the fix's UTC (whole seconds), satellites
  used, HDOP, altitude and VDOP every N s; raw NMEA can be echoed [S12] pp. 1 to 4, 6, 9. The BeeLine GPS logs
  position, altitude, GPS time and satellites in view, about 2.5 h at 1 Hz [S13] pp. 3, 8. As documented, none runs it.

## 6. Juno III, CATS, and what NMEA and UBX carry

- **Juno III** (RocketPy, MIT [S18]): `time,ALT,HORZV,VERTV,LAT,LON`, 444 rows, "a log from the COTS GNSS sensor";
  `ALT` whole feet (§7), `VERTV` whole ft/s (by check), 10 Hz to 29.7 s, 1 Hz from 30.4 s; no fix, satellites, UTC
  or device. `VERTV` fits the RRC3's velocity best shifted 1.45 s (rms 1.8 to 2.0 m/s; clock zero and lag in one).
- **CATS Vega.** An IMU and an MS5607 [S19] §2.1; a MAX-M10S, 10 Hz GNSS telemetry and a logged "GNSS trajectory"
  [S24]. Which GNSS fields the flash log keeps is undocumented, and no CATS table read here has one.
- **NMEA 0183.** GGA: fix UTC, quality, satellites used, HDOP, MSL altitude, geoid separation ([S17] pp. 24 to 25,
  [S22]); GSA: mode, DOPs; RMC: UTC, status, ground speed, course; none has a vertical speed (**Inference**, [S22]'s
  list). u-blox's `PUBX,00` has `vVel` (positive down), hAcc and vAcc, off by default [S17] pp. 36 to 37, 164.
- **UBX-NAV-PVT** [S17] pp. 98 to 99: iTOW (the epoch, ms), fixType, numSV, hAcc and vAcc (mm), velD (positive
  down), sAcc (mm/s); off by default [S17] p. 166. No hobby log read here keeps it.

**Speed from differenced altitude (Inference throughout).** Rounding to a step q adds σ = q/√6 to a 1 s difference:
0.04 m/s at 0.1 m, 0.41 m/s at 1 m. Independent 5 m (95%) epoch errors [S20] p. v would give √2 × 5/1.96 = 3.6 m/s,
an upper bound if most error drifts slowly. On 1 m Altus data (steps 0.5 to 1.5 s apart, |mean climb| 0.5 to 15 m/s,
σ as 1.4826 × MAD of the gap from the logged climb): about 0.6 m/s over [S21]'s 288 steps, 2.1 m/s over 582 on a
TeleGPS v2 log [S25]. Expect 0.3 to 2 m/s at 1 Hz, against 0.05 m/s for a receiver's own velocity (50% at 30 m/s,
[S23] Table 1, p. 4). Stamped at the later fix a 1 s difference reads 5 to 19 m/s high in coast; stamp the midpoint.

## 7. Altitude datums

A GNSS height is above mean sea level (the geoid) or the WGS84 ellipsoid. At the US pads below, the geoid lies 17 to
34 m under the ellipsoid (NGS GEOID18), so an ellipsoidal height read as MSL is 17 to 34 m low.
- **Stated MSL.** Featherweight `ALT`, "Altitude (ASL, Feet)" [S1] p. 17 (station logs: `Alt asl`); NMEA GGA, MSL
  with the geoid separation beside it [S17] pp. 24 to 25 (−31.0 m on all 3,979 good GGA of the TRS stream); the
  Quasar's "ASL altitude" [S6] p. 43; the Eggfinder LCD's "GPS ASL" [S8] p. 23; BRB900L `alt` [S11] p. 6.
- **Ellipsoid, or both.** `PUBX,00`'s `altRef` is "above user datum ellipsoid", WGS84 unless set otherwise [S17]
  p. 36; NAV-PVT has `height` (ellipsoid) and `hMSL`, mm [S17] p. 99. No cached u-blox document names its geoid.
- **Unstated.** Altus's three manuals ([S15] Table 15: altitude, int16, m), BRB900 `$BRBTX` [S12] p. 6 (LCD: MSL,
  p. 8), the BeeLine log [S13] p. 3, CATS, and Juno III's README [S18].
- **Pads against terrain** (**Inference**; public logs, NED 10 m terrain, GEOID18): six Featherweight logs sit −3.2
  to +7.5 m from terrain read as MSL, +24.8 to +37.2 m as ellipsoidal; [S21] −0.7 against +27.8 m; the TeleGPS v2
  log +1.1 against +24.4 m; Juno III +9.2 against +32.5 m. All behave as MSL.
- **hpr.** `GnssAltitude` is "above mean sea level as the receiver states it" (`log.rs`), Altus's with no document
  behind it; no reader takes `PUBX,00` or NAV-PVT. **Inference:** a datum error moves heights, not the speed test.

## What this suggests

- **M7.2's rule.** Build and test the speed check on Featherweight tracker logs and Altus TeleMega and TeleGPS
  logs. Fit or carry a GNSS time offset (0.1 to 0.7 s seen on both families), and keep a height check beside the
  speed check: 3 Featherweight files disagree with themselves by over 5 m/s even after a shift.
- **hpr's AltOS readers.** Read `climb_rate` as cm/s, checked per file against the differenced altitude as `fwgps.rs`
  checks `VERTV`. A future `PUBX,00` or NAV-PVT `height` reader must not file it as `GnssAltitude`.
- **#717's survey.** Per format: speed logged or differenced, the epoch's time or a receive clock's, the accuracy
  field (none, DOP, vAcc) and the datum; difference at the midpoint.

## Open questions

- Which instant an AltOS GPS record's tick marks (about 0.5 s on one flight); a climb over int16's 327.67 m/s.
- Whether `VERTV` is velD; `FLAGS`; whether `#SATS` counts used satellites; station latency (two logs of one flight).
- The TRS `<n>`'s epoch; current Eggfinder modules; the BRB900L log format, `vel` and `vacc`; the CATS Vega's GNSS
  fields; Juno III's GNSS device and how its zero was set against the RRC3's.
- Dropped on re-check: a second `VERTV` pass (36 files, 2.2 m/s unshifted) that no cached script reproduces.

## Sources

Cached under `refs/research/gnss-log-fields/` and `-verify/`. Cited, not reproduced: makers' manuals state no license,
u-blox allows reproduction only with permission, [S21] and [S25] have none; Altus documents are CC ShareAlike 3.0,
[S18] MIT, [S19] CC BY 4.0. FW is `https://www.featherweightaltimeters.com/uploads/1/0/9/5/109510427/`, ET
`https://eggtimerrocketry.com/wp-content/uploads/`, BRB `https://cdn.shopify.com/s/files/1/0062/3919/1158/files/`, UB
`https://content.u-blox.com/sites/default/files/`.

- [S1] Featherweight, *GPS Tracker User's Manual*, 2025-02-20, FW `gps_tracker_manual_2025feb20b.pdf`.
- [S2] Featherweight, GPS tracker page, <https://www.featherweightaltimeters.com/original-gps-tracker.html>.
- [S3] Featherweight, *Swift User's Manual*, 2026-04-10, FW `swift_users_manual_apr_10_2026.pdf`.
- [S4] Featherweight, *Blue Raven User's Manual*, 2026-05-12, FW `blue_raven_users_manual_2026may12.pdf`.
- [S5] Eggtimer, *Eggfinder User's Guide* Rev B6, ET `2018/06/Eggfinder_Users_Guide_RevB6.pdf`.
- [S6] Eggtimer, *Quasar User's Guide* 1.02S, ET `2024/02/Eggtimer-Quasar-1_02s.pdf`.
- [S7] Eggtimer, *TRS User's Guide*, ET `2018/06/Eggtimer_TRS_Users_Guide_2_07.pdf`.
- [S8] Eggtimer, *Eggfinder LCD Receiver User's Guide* 2.05a, ET `2025/01/Eggfinder-LCD-Users-Guide-2_05a.pdf`.
- [S9] Eggtimer, Eggfinder page, <https://eggtimerrocketry.com/home/eggfinder-gps-tracking-system/>.
- [S10] Rocketry Forum thread 135349, <https://www.rocketryforum.com/threads/135349/>, posts 21, 22, 28, 34.
- [S11] Big Red Bee, *BRB900L and BRB433L GPS Tracker User Manual* v0.10, BRB `BRB900_User_Manual_v0.4.pdf`.
- [S12] Big Red Bee, *BRB900* manual (2013), BRB `brb_lcd6.pdf`.
- [S13] Big Red Bee, *GPS User's Guide* V0.20, December 2024, BRB `beelineGPS_20.pdf`.
- [S14] Altus Metrum, *The Altus Metrum System*, v1.9.22.2, <https://altusmetrum.org/AltOS/doc/altusmetrum.pdf>.
- [S15] Altus Metrum, *AltOS Telemetry*, v1.9.22.2, <https://altusmetrum.org/AltOS/doc/telemetry.pdf>.
- [S16] Altus Metrum, *TeleGPS Owner's Manual*, v1.9.22.2, <https://altusmetrum.org/AltOS/doc/telegps.pdf>.
- [S17] u-blox, *M10 interface description* R03, UB `u-blox-M10-SPG-5.10_InterfaceDescription_UBX-21035062.pdf`.
- [S18] RocketPy, Juno III data, <https://github.com/RocketPy-Team/RocketPy/tree/master/data/rockets/juno3>.
- [S19] Binz, Stojoski, Jost, "CATS", 26th ESA PAC Symposium (2024), <https://arxiv.org/abs/2511.04725>.
- [S20] US DoD, *GPS SPS Performance Standard*, 5th ed., April 2020,
  <https://www.gps.gov/sites/default/files/2025-07/2020-SPS-performance-standard.pdf>.
- [S21] ISSUIUC, a TeleMega log and CSV, <https://github.com/ISSUIUC/flight-data/tree/master/20240323>.
- [S22] E. S. Raymond, *NMEA Revealed*, gpsd, <https://gpsd.gitlab.io/gpsd/NMEA.html>.
- [S23] u-blox, *MAX-M10S Data Sheet*, UBX-20035208 R08, UB `MAX-M10S_DataSheet_UBX-20035208.pdf`.
- [S24] CATS, Vega product page, <https://www.catsystems.io/vega>.
- [S25] Olin Rocketry, TeleGPS v2 log, <https://github.com/Olin-Rocketry/sensing>, `software/Legacy/IREC Launch Data/`.
