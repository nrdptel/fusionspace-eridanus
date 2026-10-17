# How far hobby GNSS receivers lag a rocket in boost

This note asks what receiver documents say of dynamic models and speed and altitude limits for the GNSS parts in
hobby trackers, what sounding-rocket papers measure for lag and dropout in boost, and when GNSS altitude can be
trusted again. It serves [M7.2](../ROADMAP.md)'s baro, accelerometer and GNSS fusion, the "GNSS altitude
distrusted during boost" idea in [flight-data ideas](ideas-flight-data.md), and GNSS-only apogees in
`accuracy.collection`. It builds on [smoothing methods](altimeter-smoothing-methods.md) (its [R11] is [S1] here)
and [avionics](suite-avionics.md). Sources: makers' documents, DLR and NASA papers, a loop paper and thesis, a team
report and forum posts (what a flyer reports). **Inference** marks this note's reasoning, arithmetic and a small
filter simulation. Read 2026-10-17.

**In short.** No receiver document gives a filter lag or output delay, so the size of a boost lag can't be read
from a datasheet. Two failures are measured and must be kept apart: tracking loops slip or lose lock at ignition
and burnout, and the receiver's navigation filter falls behind while still flagging a valid 3-D fix. In one flight
a receiver's altitude fell up to 440 m behind; on a bench replay of that climb another receiver's 3-D fix lagged
43 m. Two GNSS-only apogees read low, by 223 m and 532 m. The fix flag is no test of trust; this note proposes
agreement with the barometer's vertical speed instead (**Inference**).

## 1. Dynamic models and limits in the documents

u-blox's dynamic platform model (`dynModel`, `CFG-NAVSPG-DYNMODEL`) sets limits a solution must pass. Columns:
max altitude (m), max horizontal speed (m/s), max vertical speed (m/s). M9 and M10 share a table.

| Model (code) | u-blox 7 [A1] | M8 [A2] | M9 [A4], M10 [A6] | Checked on |
|---|---|---|---|---|
| Portable (0) | 12,000 / 310 / 50 | 12,000 / 310 / 50 | 12,000 / 310 / 50 | altitude and velocity |
| Airborne <1g (6) | 50,000 / 100 / 100 | 50,000 / 100 / 100 | 80,000 / 100 / 6,400 | altitude only |
| Airborne <2g (7) | 50,000 / 250 / 100 | 50,000 / 250 / 100 | 80,000 / 250 / 10,000 | altitude only |
| Airborne <4g (8) | 50,000 / 500 / 100 | 50,000 / 500 / 100 | 80,000 / 500 / 20,000 | altitude only |

- A failed check "invalidates" the solution ([A2] §8.1 p. 22; [A4] §3.1.7.1 p. 12; [A6] §2.2.1 p. 16). Airborne
  models give "a higher standard deviation in the reported position", with no number, and no 2-D fix.
- Portable is the default on u-blox 7 ([A1] App. B.3 p. 185), on M8 standard-precision firmware ([A2] App. C.10
  p. 450; Automotive on its dead-reckoning firmware), M9 ([A4] Table 3 p. 11) and M10 ([A5] p. 168).
  **Inference:** a default-model receiver should drop its fix, not lag, once vertical speed passes 50 m/s; a
  lagging valid fix points to an Airborne model, whose model rows are checked on altitude only. That doesn't lift
  the operational speed limit below, which the datasheets state for the Airborne 4 g model itself.
- A fix is valid only if PDOP and the receiver's own accuracy estimate are under their masks, defaults PDOP 25.0
  and pAcc 100 m on M9 and M10 ([A3] p. 250; [A5] p. 169); when it fails, NMEA shows status V and GGA quality 0
  ([A2] §31.1.6 p. 141). No document ties pAcc or vAcc to the true error under high dynamics. **Inference:** a
  solution 440 m behind whose own estimate stays under 100 m is reported as a valid 3-D fix.
- Operational limits: NEO-M8 4 g, 50,000 m, 500 m/s, "Assuming Airborne < 4 g platform" ([A7] Table 1 p. 6);
  NEO-M9N and MAX-M10S 4 g, 80,000 m, 500 m/s, "Assuming Airborne 4 g platform" ([A8] p. 4; [A9] Table 1 p. 4).
  No u-blox document read says whether the altitude and speed limits act together or apart.
- Default rate is 1 Hz on M9 and M10 ([A3] p. 252; [A5] p. 170), at most 5 to 25 Hz ([A7] p. 6; [A8] p. 4; [A9]
  p. 4). The UBX iTOW tag is the epoch's GPS time ([A2] §9.3 p. 27); no document gives the epoch-to-output delay.
  **Inference:** a logger stamping sentences on arrival shifts the track by it; 440 m is 1.5 to 2.2 s at 200 to 300 m/s.
- **Quectel L76** (PMTK commands; **Inference:** MediaTek-based): 10,000 m, 515 m/s, 4 g ([A10] Table 2 p. 13);
  LC76G 10,000 m, 490 m/s, 4 g ([A12] p. 2). The L76-LB's PMTK886 modes stop at 10,000 m, Balloon at 80,000 m;
  outside, the position "will be incorrect" ([A11] §3.2.43 pp. 46 to 47); no speed limit per mode.
- **SkyTraq:** six navigation modes, airborne and balloon among them, with no limits per mode ([A13] p. 79);
  PX1122R "Altitude < 80,000m and velocity < 515m/s" ([A15] p. 2).

## 2. What tracker makers say

- **Altus Metrum** ([A19] Table 12 p. 78): TeleMetrum v1 SkyTraq, then u-blox MAX-7Q, MAX-8Q, MAX-8C or MAX-10S;
  TeleMega and TeleGPS the same u-blox line. Under high acceleration the position "may not get updated as the GPS
  receiver loses position fix", resuming in coast (§12.1.2 p. 30). No maker's document names the dynamic model set.
- **Eggtimer:** losing GPS through boost and early coast is "fairly normal", and GPS altitude tends to "lag behind
  the latitude/longitude for several seconds at least" ([A18] pp. 6, 12 to 13): the only lag statement found in
  any maker's document. No manual of six read names the module.
- **Featherweight:** the manuals don't name the receiver; the product page names a u-blox M10 and gives 80 km and
  500 m/s ([W1]), matching [A9]. Weak signal makes it "likely to lose GPS fix at launch" ([A16] p. 10).
- **Big Red Bee:** the BeeLine guide names no receiver and says either limit may be passed "but not both" ([A20]
  p. 15). The BRB900 manual names a u-blox SAM10, gives positions to 80 km (p. 1), then a cut-out above 1,000 knots
  OR 60,000 ft (p. 2) [A21].

## 3. What flights measure

| Source | Vehicle, peak | Receiver | Event | Measured |
|---|---|---|---|---|
| [S1] p. 5 | PMWE-1 | DLR Phoenix | liftoff, burnout | lost or overshot several hundred m, up to 10 s |
| [S4] p. 5 | Test Maxus-4, 17 to 18 g | Allstar | liftoff | no 4-satellite fix until T+7 s |
| [S4] p. 5 | same | G12, Orion | liftoff | kept lock; all satellites back in 8 to 9 s |
| [S3] Table 2 p. 7 | same | Orion vs G12 | 24 s window, 6 s burn | 90.0 m RMS (free flight 1.5 m) |
| [S3] Table 1 p. 3 | Texus, simulator | Orion | signal cut | fix 145 s late unaided, 2 s with aiding |
| [S6] pp. 8 to 9 | VSB-30, 11.3 g | Phoenix-HD | whole flight | no boost loss; 60 s false lock near apogee |
| [S9] p. 5 | MAPHEUS-15 | mosaic-X5 | tower exit | no position T+0 to T+4.5 s |
| [S10] pp. 11 to 12 | Traveler IV, 18.8 g | two consumer | liftoff | no lock T+0 to T+278 s (apogee T+151 s) |
| [S11] post 3 | HPR, not given | SAM-M10Q | first 7 s | altitude up to 440 m behind, valid 3-D fix |
| [S11] post 4 | bench replay | PX1105R | boost | 3-D fix 43 m and 27 m/s behind |
| [S12] post 1 | two HPR flights | u-blox M8 | apogee | 532 m and 223 m under the barometer |

- **Professional receivers** output unfiltered single-point fixes ([S7] p. 2; [S3] p. 7), so (**Inference**)
  their boost errors are loop errors. Test Maxus-4's came from stress on the 10 MHz crystal at high jerk ([S4]
  p. 7); a box-car filter switched on by mistake degraded its high-dynamics fixes ([S4] p. 6), the one
  professional record of a receiver's own filter hurting a boost.
- **The 440 m flight** (a flyer reports, [S11] post 3, 2026-10-10): for the first 7 s of the climb a SAM-M10Q kept
  at least 4 satellites and a valid 3-D fix while its altitude fell up to 440 m behind. Replayed on a bench
  simulator into a SAM-M10Q, the same trajectory gave valid fixes hundreds of meters off in 3 of 4 runs. On a bench
  replay into a PX1105R (post 4), an EKF on its raw pseudoranges and Doppler plus an IMU stayed within 8 m (3-D).
  The posts don't give the dynamic model, motor, peak acceleration, speed or vAcc. Post 1 shows three SAM-M10Q
  flights and says of its lag plots that the filter "catches up eventually" in coast and under chute.
- **Low apogees** (a flyer reports, [S12]): the poster's own u-blox M8 flight read 2,568 m against the barometer's
  3,100 m, which matched local soundings; another team's flight read 357 m against 580 m (post 1). Post 5 plots
  one of the poster's supersonic flights in Airborne <1g: no lock in the burn, relock just after apogee, then still
  climbing, lagging, before it converged. **Inference:** that may be the 2,568 m flight; the post doesn't say,
  and a reader of the posted log gives a third apogee, 2,303.8 m (post 12).

## 4. Why it lags (Inference on cited formulas)

- **Tracking loops.** A third-order PLL has no steady error from acceleration; it errs by (jerk/λ)/ωn³ cycles,
  with Bn = 0.7845 ωn, and holds lock while the error stays under 0.125 cycle ([S14] eqs. 2 to 4 p. 371, where
  25 Hz tolerates about 78 g/s; the Bn form in [S15] eq. 2.58 p. 30, its threshold eq. 2.63 p. 31). With
  λ = 0.190 m at L1 the jerk limit is 5 g/s at 10 Hz, 17 g/s at 15 Hz and 78 g/s at 25 Hz. A motor rising to
  15 g in 0.1 s gives about 150 g/s, so slips at ignition and a sharp burnout are expected in an unaided hobby
  receiver. No hobby part's loop bandwidth is published.
- **Navigation filter.** A constant-velocity filter on position fixes is a type-2 loop: it lags by a/ωn² in boost
  (reads low) and leads by about (g + D/m)/ωn² in coast (reads high), shedding the burnout error with time
  constant about 1/(ζωn). A simulation (noise-free 10 Hz fixes, filter sigma 3 m, boosts of 5 to 15 g, drag in
  coast) with ωn 0.47 to 0.69 rad/s gave 103 to 261 m low in boost; by apogee (21 to 25 s) the lag had turned
  into a lead of +20 to +43 m. Fed Doppler velocity at 0.1 m/s, the same filter lags under 1 m.
- So a lag of hundreds of meters means the receiver isn't following its own Doppler: Doppler weighted down,
  measurements rejected against a low-dynamics model, or loops already slipping. The reports can't tell which.
- **Signs.** In boost GNSS altitude reads low. At apogee a settled filter should read high by tens of meters
  (unmeasured); one still recovering lock reads low by hundreds (once with a described relock; [S12]'s second
  low apogee has no lock record). A GNSS-only apogee has no one-sided correction.

## 5. When GNSS altitude can be trusted again

- Professional receivers recover 7 to 10 s after liftoff or burnout, with screening ([S1] p. 5; [S4] p. 5), and
  4.5 s after a tower exit ([S9] p. 5); newly tracked satellites gave outliers to 100 m ([S5] p. 7) and one false
  lock lasted about 60 s ([S6] pp. 8 to 9). Hobby receivers flag valid fixes while hundreds of meters off
  ([S11]), so fix flag and satellite count are no test. No hobby source gives a convergence time or vAcc trace.
- **Candidate test (Inference).** Trust GNSS altitude once its vertical speed has agreed with the barometric or
  integrated-accelerometer vertical speed within a few m/s for at least 2 s, and its altitude change since then
  agrees with the barometer's within its error. Use vAcc only as support: no source shows it growing in a lag.
  The barometer is no truth in boost either: the [S11] flyer left it out because its static-port error in boost
  reached up to 100 m (post 4), so the test belongs in coast.
- **Inference:** a GNSS-only apogee is safe only when that test passed before apogee with time to spare; one
  relocked at or after apogee ([S12] post 5; [S10], T+278 s against apogee T+151 s) is not an apogee.

## What this suggests

- M7.2's fusion: GNSS altitude at zero weight from liftoff until the speed-agreement test passes, never on fix
  flag, satellite count or vAcc alone, then an inflated variance for a few seconds. A mutation trusting GNSS from
  liftoff should fail on a synthetic log: altitude low by a/ωn² in boost (ωn near 0.5 rad/s, up to 440 m), a 0 to
  10 s dropout at ignition and burnout, and a valid fix flag throughout (§3, §4).
- `accuracy.collection`: admit a GNSS-only apogee only with a recorded trust time before apogee, and carry tens
  of meters both ways as its error until a measured case says otherwise.
- The reader could record the dynamic model when the log states it, since (**Inference**, §1) Portable should
  drop the fix and Airborne keep a lagging one.

## Open questions

- No measured consumer-receiver filter lag against a truth in a real boost; the 440 m flight lacks its model,
  motor and peak acceleration, and the flyer's best-settings table is an image not read.
- The high apogee bias after a settled coast is predicted, not measured. Loop bandwidths for u-blox, Quectel and
  SkyTraq parts are unpublished. No u-blox or Quectel document says how the altitude and speed limits combine;
  SkyTraq's [A15] writes "and", and Big Red Bee's two manuals disagree.
- The mode each tracker configures. Not read: the RTx manual, the Venus838 and S1216 datasheets, and Kaplan (ed.,
  1996) itself (used through [S14], [S15]). [S4] Table 1's per-axis boost errors (max 341 m) are OCR, not used.

## Sources

Read 2026-10-17; cached under `refs/research/gnss-in-boost/`, never committed; all cited, not copied. u-blox,
Quectel and SkyTraq bar reproduction without permission; makers' manuals state no license; papers keep their
copyright except [S4] (NTRS: public use permitted) and [S9] (CC BY); the Altus manual is under the "Creative Commons
ShareAlike 3.0 License". u-blox documents are at `https://content.u-blox.com/sites/default/files/` plus the name.

- [A1] u-blox 7 Receiver Description, GPS.G7-SW-12001-B1:
  `products/documents/u-blox7-V14_ReceiverDescriptionProtocolSpec_%28GPS.G7-SW-12001%29_Public.pdf`.
- [A2] M8 Receiver Description R28: `products/documents/u-blox8-M8_ReceiverDescrProtSpec_UBX-13003221.pdf`.
- [A3] M9 SPG 4.04 Interface Description R02: `u-blox-M9-SPG-4.04_InterfaceDescription_UBX-21022436.pdf`.
- [A4] NEO-M9N Integration Manual R10: `NEO-M9N_Integrationmanual_UBX-19014286.pdf`.
- [A5] M10 SPG 5.10 Interface Description R03: `u-blox-M10-SPG-5.10_InterfaceDescription_UBX-21035062.pdf`.
- [A6] MAX-M10S Integration Manual R05: `MAX-M10S_IntegrationManual_UBX-20053088.pdf`.
- [A7] NEO-M8 (FW3) Data Sheet R14: `NEO-M8-FW3_DataSheet_UBX-15031086.pdf`; [A8] NEO-M9N-00B Data Sheet R08:
  `NEO-M9N-00B_DataSheet_UBX-19014285.pdf`; [A9] MAX-M10S Data Sheet R08: `MAX-M10S_DataSheet_UBX-20035208.pdf`.
- [A10] Quectel L76/L76-L Hardware Design V3.2,
  <https://www.quectel.com/content/uploads/2024/02/Quectel_L76L76-L_Hardware_Design_V3.2-3.pdf>.
- [A11] Quectel L76-LB/L26-LB/LC86L GNSS Protocol Specification V1.1,
  <https://quectel.com/content/uploads/2024/04/Quectel_L76-LBL26-LBLC86L_GNSS_Protocol_Specification_V1.1.pdf>.
- [A12] Quectel LC76G Series GNSS Specification V1.1,
  <https://quectel.com/content/uploads/2024/03/Quectel_LC76G_Series_GNSS_Specification_V1.1-1-1.pdf>.
- [A13] SkyTraq AN0028 v1.4.49, <https://www.skytraq.com.tw/homesite/AN0028.pdf>; [A15] PX1122R datasheet,
  <https://www.skytraq.com.tw/homesite/PX1122R_DS.pdf>.
- [A16] Featherweight GPS tracker manual, 2025-02-20,
  <https://www.featherweightaltimeters.com/uploads/1/0/9/5/109510427/gps_tracker_manual_2025feb20b.pdf>;
  [W1] <https://www.featherweightaltimeters.com/original-gps-tracker.html>.
- [A18] Eggfinder User's Guide Rev B6,
  <http://eggtimerrocketry.com/wp-content/uploads/2018/06/Eggfinder_Users_Guide_RevB6.pdf>.
- [A19] Altus Metrum manual v1.9.22.2, <https://altusmetrum.org/AltOS/doc/altusmetrum.pdf>.
- [A20] Big Red Bee BeeLine GPS guide V0.20, <https://cdn.shopify.com/s/files/1/0062/3919/1158/files/beelineGPS_20.pdf>;
  [A21] BRB900 manual Version 0.10 (October 2026),
  <https://cdn.shopify.com/s/files/1/0062/3919/1158/files/BRB900_User_Manual_v0.4.pdf>.
- [S1] Braun, Barf, Markgraf, EUCASS 2019-0241, <https://www.eucass.eu/doi/EUCASS2019-0241.pdf>.
- [S3] Markgraf et al., ESA PAC Symposium 2001, DLR GSOC document `eurock_xv.pdf`, <https://www.dlr.de/de/rb/medien/>
  (publikationen/publikationen/gsoc-dokumente).
- [S4] Bull, Diehl, Montenbruck, Markgraf, ION NTM 2002, <https://ntrs.nasa.gov/citations/20020060112>.
- [S5] Montenbruck, Markgraf, AIAA 2003-5722, DLR GSOC document `aiaa_gnc_035722.pdf` (same DLR path as [S3]).
- [S6] Markgraf, Montenbruck, Phoenix-HD, ISLT 2005, `islt_05.pdf`; [S7] DLR Phoenix datasheet v1.1,
  `phoenix_datasheet_v11.pdf` (both DLR GSOC documents, same path as [S3]).
- [S9] Braun et al., Eng. Proc. 126, 39 (2026), doi:10.3390/engproc2026126039,
  <https://elib.dlr.de/214653/1/engproc-126-00039.pdf>.
- [S10] Aitoumeziane et al., Traveler IV Apogee Analysis, USC RPL (2019), <https://uscrpl.com/s/Traveler-IV-Whitepaper>.
- Rocketry Forum: [S11] <https://www.rocketryforum.com/threads/mitigating-gnss-issues-in-boost.199286/> (2026);
  [S12] <https://www.rocketryforum.com/threads/gps-vs-altimeter-data.172016/> (2022).
- [S14] Ebinuma, Kato, Earth Planets Space 64, 369 to 377 (2012), doi:10.5047/eps.2011.11.005; [S15] Lian, MSc
  thesis, University of Calgary (2004), <https://www.ucalgary.ca/engo_webdocs/GL/04.20208.PLian.pdf>.
