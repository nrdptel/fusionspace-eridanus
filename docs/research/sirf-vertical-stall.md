# Why a SiRFstarIV receiver's altitude stalls in a rocket's climb

This note asks what in a SiRFstarIV receiver makes its altitude stay near the pad through boost while it reports
a valid 3-D fix, as the Eggfinder's Maestro Wireless A2235-H did on two flights in
[forum-eggfinder-gpx-vs-baro](forum-eggfinder-gpx-vs-baro.md) (apogee 79 to 82% low, 40 to 62 s late). It reads
SiRF's and CSR's protocol manuals, four SiRFstarIV module makers' documents, Eggtimer Rocketry's manuals, and one
more public flight with a SiRF receiver beside a SkyTraq. It serves [M7.2d](../ROADMAP.md)'s GNSS trust rule
("trusting GNSS from liftoff fails", ADR-280's gating) and builds on [gnss-in-boost](gnss-in-boost.md), which
covers u-blox's dynamic models and a filter-lag simulation. Read 2026-10-28; counts come from scripts run on the
cached files. **Inference** marks this note's reasoning.

**In short.**

- **No document names the mechanism.** No SiRF, CSR or module document gives a filter time constant, a separate
  weighting for altitude, a vertical speed clamp or a dynamic model a user can choose. The fix is a Kalman-filter
  solution by default, and two SiRFstarIV module makers rate it to **4 g and a jerk of 5 m/s³**; past them, Telit
  says, it "will lose track". A boost passes that jerk by roughly 16 to 100 times (**Inference**). Moderate
  confidence that the filter is the cause; how it stalls and why it takes 40 to 62 s to recover is unpublished.
- **Altitude hold, static navigation, dead reckoning and the speed and altitude limits don't fit.** Each has a
  default, a trigger or a flag that these flights rule out (§3). High confidence.
- **Both Eggtimer flights carry the A2235-H:** the TRS guide names it too, so LDRS 35 is a second flight of it.
- **A third SiRF flight stalls and then reads high.** On a Finnish hybrid flight the SiRF track stayed within 6 m
  of the pad for points 0 to 9 while a SkyTraq beside it climbed 1,154 m; the SiRF then peaked 62 points late and
  174 m (8.1%) *above* the SkyTraq. A SiRF-only apogee can be wrong in either direction.
- **No NMEA field flags it.** Through the LDRS stall: GGA quality 1, 7 to 9 satellites, GSA 3-D, RMC status A and
  mode A, VDOP 1.4 to 2.4. Only the binary protocol's MID 41 carries the receiver's own vertical error estimate.
- **No setting is known to avoid it.** No `$PSRF` command touches the filter, and the binary-protocol switches that
  exist (altitude hold, static navigation) aren't the cause.

## 1. The receiver in these trackers

Eggtimer Rocketry's manuals name the Maestro Wireless A2235H on the Eggfinder TX (revisions B4d to C1), the
Eggfinder Mini (A7, A10), the LCD-GPS handheld and the Eggtimer TRS (A5 to B4) [B2] to [B7], [B10], [B13]; the
TRS guide 2.07 calls it "SiRF IV", one sample a second [B9] p. 48. From TX D3/D4 and Mini C3b (2023 on) the boards
carry a "pre-mounted GPS module" [B8], a Quectel one by the maker's support pages [B21], model not named.
Featherweight, Big Red Bee, Altus Metrum and Missile Works name u-blox parts (gnss-in-boost §2; [B17]); no other
SiRF rocket tracker was found.

- The A2235-H is a ROM part: "CSR Premier ROM 9333 ... equivalent to Release 2.2 (i.e. F/W 4.1.2)", no EEPROM and
  no backup battery [D6] p. 8. Default output NMEA at 4,800 baud, 1 Hz, 5 Hz selectable [D6] pp. 19, 36.
- TX B6 to C1 and the Minis list no processor that could send the module a command [B3] to [B7] p. 7, so they run
  its start-up defaults (**Inference**). Their serial ports are documented at 9,600 baud [B1] p. 10; the module
  takes 4,800 or 9,600 baud NMEA from two pins strapped at start-up [D6] p. 20.
- With no EEPROM or backup battery, a setting sent to the module probably lasts only while power stays on
  (**Inference**; [D6] p. 8 and p. 34, which names "BBRAM or EEPROM" as the store).
- The makers warn without sizes or remedies. GPS altitude lags "for several seconds at least" [B1] p. 13; the
  OpenLog note finds it "really far off during ascent" against the TRS barometer [B14] p. 5; the 2023 LCD notes,
  written after the Quectel change, still call it "unlikely to be accurate during the ascent" [B12] p. 5.
- The TRS's `<n>` value, which HPR Sim drops today, is the barometric altitude above ground in feet, once a second
  ([B14] p. 5; "Real-time AGL Altitude in feet (changes to apogee at landing)" [B9] p. 55), which answers the
  earlier note's open question. The LDRS stream falls to 39 ft, then holds 2,514 ft on all 2,804 later records.

## 2. What the documents say the filter does

- **Kalman filter by default.** MID 2 position mode 4 is "> 3-SV solution (Kalman filter)", modes 5 and 6 are
  least-squares point solutions [D3] p. 278; MID 41 Nav Type 100 is "4 or more SV KF solution" [D3] p. 333. The
  configuration defaults for firmware 4.1.2 on give "Kalman filtering mode" with smoothed measurements on [D3]
  pp. 496 to 497. Track smoothing is off by default [D3] p. 494, and off in the A2235-H maker's own mode-control
  example [D6] p. 36 (decoded against [D3] pp. 124 to 125).
- **No dynamic-model setting.** The 614-page One Socket Protocol ICD [D3] and the older SiRF Binary manual [D4]
  offer nothing like u-blox's `dynModel`; the NMEA input messages have none either [D1] p. 2-1; [D2].
- **Dynamic limits.** No A2235-H document states one. Two other SiRFstarIV modules do:

| Source | Acceleration | Jerk | Speed | Altitude | Export pair |
|---|--:|--:|--:|--:|---|
| Telit JF2 [D10] p. 21 | 4 G | 5 m/s³ | 600 m/s | −500 to 24,000 m | >514 m/s and >18,288 m |
| OriginGPS ORG1411 [D13] p. 17 | 4 g | 5 m/s³ | 600 m/s | −500 to 24,000 m | 515 m/s and 18,288 m |

- Telit: the receiver "will lose track if any of the following limits are exceeded" [D10] p. 21. No document says
  what the filter does at or past a limit, how it edits measurements, or how long it takes to recover.
- **Size against the flights (Inference).** The J415W flight's barometer reached about 122 m/s near T+3 s, a mean
  near 4 g over the burn. Thrust rising by about 40 m/s² in 0.1 to 0.5 s, and falling by about 50 m/s² at
  burnout, is 80 to 500 m/s³; the rise time is assumed, not measured. Speed (122 m/s) sits 4.9 times under
  600 m/s; apogee (1,406 m) 17 times under 24,000 m.

**Assessment (Inference).** A filter tuned for vehicles, rated for limits a boost exceeds, fits the signature in
kind: a valid fix, satellites kept, altitude far behind, a slow recovery. It doesn't explain the 40 to 62 s, nor
why GGA stays at quality 1 where Telit predicts loss of track. Moderate confidence; the path is unbounded.

## 3. Mechanisms the documents rule out

- **Altitude hold.** Three modes: automatic, always (a user altitude, −1,000 to 10,000 m), or off, which "forces
  all fixes to be 3-D" [D3] pp. 124 to 126. The automatic mode governs 3-satellite solutions; the LDRS flight
  used 7 to 9. A held altitude is constant, but the J415W track's height rose on 55 of 58 drogue points. Hold shows
  only in MID 2 and MID 41 bits [D3] pp. 278 to 279, 333 to 334; NMEA GSA mode counts satellites ("3D (>3 SVs
  used)" [D1] p. 1-5), so NMEA can't show hold either way.
- **Static navigation.** Off by default [D3] p. 494; [D12] p. 17; [D13] p. 29. When on, it pins position below
  1.5 m/s and releases on a 40 m move [D11] p. 12; a rocket at 4 g covers 40 m in about 1.4 s (**Inference**), and
  pinning freezes the whole position, not a slowly rising altitude. Something pins the LDRS pad all the same:
  speed 0.00 on 1,211 of 1,223 pad epochs, 22 distinct positions, one course (175.65°) on every zero-speed epoch;
  the 2016 GPX pad has 5 distinct positions in 101 points. That hold released at liftoff, when position began to
  change (§4), so it isn't the 40 s stall.
- **Dead reckoning** (default time-out 15 s [D3] pp. 125, 494) is flagged as GGA quality 6 and RMC mode E [D1]
  pp. 1-3, 1-8; the LDRS flight was quality 1 throughout, 6 only on the pad.
- **DOP mask and limits.** Solutions over the DOP mask "are marked invalid" [D3] p. 127; LDRS flight HDOP was 0.9
  to 1.3. The 600 m/s limit sits 4.9 times above these flights' speed; an A2235-H's altitude limit is unpublished.
- **The app.** The LDRS stall is in the TRS's raw GGA, and the J415W GPX has the same shape, so (**Inference**) not
  the app. Moderate to high confidence.

## 4. The LDRS 35 log, re-read for flags

The TRS log of thread 135349, checksum-valid sentences only; the barometer is the `<n>` after each epoch.

| T+ (s) | Baro AGL, m | GNSS above pad, m | GNSS climb, m/s | RMC speed, m/s | Sats | VDOP |
|---|--:|--:|--:|--:|--:|--:|
| 0 | 47.9 | 0.0 | 0.1 | 0.1 | 9 | 1.4 |
| 7 | 630.0 | 2.3 | 0.5 | 5.8 | 9 | 1.4 |
| 13 (baro apogee) | 765.4 | 21.6 | 5.3 | 24.5 | 9 | 1.4 |
| 14 | 760.8 | 28.0 | 6.4 (max) | 16.8 | 9 | 1.4 |
| 17 to 52 | 727.6 to 170.7 | 34.0 to 152.2 | +0.9 to +5.8 | 0.3 to 17.2 | 9 | 1.4 |

- GNSS climb stays at or under 0.5 m/s from T+0 to T+7 while the barometer rises 582 m, then stays positive from
  T+17 to T+52 while the barometer falls 557 m.
- GSA 3-D on every flight epoch with a GSA; GGA quality 1; 7 to 9 satellites; RMC status A and mode A on all 73
  flight epochs; mean GSV signal 32.6 to 36.6 dB-Hz. Nothing in the default output marks the stall.
- The horizontal solution looks live: RMC speed 21.4 to 24.7 m/s at T+11 to T+13, near apogee (**Inference**).

## 5. Iso-Haisu 2011: a SiRF beside a SkyTraq

The Finnish Astronautical Society flew a hybrid (HyperTEK L540) on 2011-02-02 with two receivers; its page says
the SkyTraq "was able to maintain a correct position" while the SiRF "went completely haywire" [B15]. Neither
model is named. Its KMZ [B16] holds two untimed tracks of 216 points each; heights are above each track's first
point.

| Point | SkyTraq, m | SiRF, m | Note |
|--:|--:|--:|---|
| 9 | 1,154 | 6 | SiRF within 6 m of the pad for points 0 to 9 |
| 23 | 2,138 (peak) | 189 | SiRF 91% low at the SkyTraq peak |
| 56 | 1,566 | 1,709 | SiRF above the SkyTraq from here (one 337 m step) |
| 85 | 994 | 2,312 (peak) | 62 points late, 174 m (8.1%) above the SkyTraq peak |
| 169 to 215 | 297 to 12 | 315 to 15 | within 20 m to landing |

- **Cadence (Inference): one point a second.** At 2 s a point the SkyTraq's coast deceleration would be 2.9 m/s²,
  less than g, which a coasting rocket can't do; at 1 s it is 11.6 m/s².
- **Here the horizontal stalled too:** over points 0 to 9 the SkyTraq moved 358 m sideways, the SiRF 62 m.
  **Inference:** the whole solution lagged, not height alone.
- **Which peak is true is open.** The rocket's record, 2,350 m [B19], states no instrument or datum; it is 212 m
  above the SkyTraq peak and 38 m above the SiRF's. The SkyTraq profile is ballistically consistent (**Inference**).
- Against u-blox flights (gnss-in-boost §3, gnss-log-fields), the SiRF tracks share what those don't: near-zero
  climb through the burn, a peak 40 to 62 s late, and an apogee error of either sign (**Inference**, three flights).

## What this suggests

- **M7.2d's trust rule must be two-sided and can't lean on flags.** The height-window test of
  forum-eggfinder-gpx-vs-baro rejects boost and coast on all three flights; Iso-Haisu reads high for points 56 to
  168, so agreement must be tested both ways. Fix quality, satellites, GSA mode, VDOP and RMC mode all pass
  through the stall and can't stand in for agreement.
- **#915's reason text** should say GNSS may lag the climb and then peak low or above the barometer, not only
  overshoot; Iso-Haisu is the high case.
- **A TRS `<n>` reader** (AGL feet, 1 Hz, now documented by the maker) makes the LDRS log a one-file GNSS and
  barometer pair for #717's survey, with no clock offset to fit.
- **A zero-speed pad with a frozen course** is a receiver hold; a launch detector reading RMC speed should expect
  it near zero for a second or two after liftoff. **For users (an idea):** an A2235-H's altitude isn't an apogee.

## Open questions

- SiRF's filter process model, measurement editing and recovery time. The ICD points to a configuration-kit guide
  (CS-206097-UG) and an SDK guide (CS-210291-UG) for the defaults [D3] p. 498; neither was found.
- Whether a 5 Hz navigation rate (`$PSRF103,00,6,00,0` [D6] p. 36), the only documented setting that touches
  timing, changes the stall; and whether binary-protocol settings survive a power cycle on an A2235-H.
- Whether a maker-set fix-altitude cap plays a part: the ICD's "Maximum allowed altitude for a position fix" has
  an example value of 2,400 m [D3] p. 492, and the Iso-Haisu SiRF peak sits about 2,402 m above sea level.
- Iso-Haisu's two receiver models and its 2,350 m record's instrument.
- Rocketry Forum posts were not read: every fetch got a Cloudflare 403 and the browser extension was not
  connected. Leads: thread 139965 post 1684691 (an Eggfinder rebuilt on a u-blox NEO-6M), thread 69873 (BRB900
  against Eggfinder), post 1813939 (BRB900 against a MAWD altimeter).
- Whether the Quectel Eggfinders (TX D3/D4, Mini C3b) still stall; no public track was found.

## Sources

Read 2026-10-28, cached under `refs/research/sirf-vertical-stall/` (cited only: the SiRF and CSR documents carry
confidentiality footers on public mirrors; Eggtimer's manuals and the Iso-Haisu files state no license).

- [D1] SiRF, NMEA Reference Manual CS-129435-MAP1, 2010 (GSD4e),
  <https://gps.0xdc.ru/static/sirf/soft/win32/sirflive/GSD4e%20NMEA%20Manual%20.pdf>; [D2] CSR, NMEA Reference
  Guide Issue 3, 2012, <https://gps.0xdc.ru/static/sirf/doc/SirfStar/NMEA-Reference-Manual-CS-129435-MA-3.pdf>
- [D3] CSR, SiRFstar One Socket Protocol ICD Issue 15, CS-129291-DCP15, 2013, <https://gps.0xdc.ru/static/sirf/doc/
  SirfStar/One_Socket_Protocol_Interface_Control_Document_(OSP_ICD)_(CS-129291-DC-15).pdf>
- [D4] SiRF, Binary Protocol Reference Manual Rev 2.4, 2008,
  <https://cdn.sparkfun.com/datasheets/Sensors/GPS/SiRF_Binary_Protocol.pdf>
- [D6] Lantronix, A2235-H User Guide PMD-00029 Rev A, 2019,
  <https://cdn.lantronix.com/wp-content/uploads/pdf/A2235-H-UG.pdf>
- Telit, under <http://www.iot.com.tr/uploads/pdf/>: [D10] JF2 Product Description r8, 2015,
  `Telit_Jupiter_JF2_Product_Description_r8.pdf`; [D11] JF2 and JN3 MEMS Sensor Application Note Rev.2, 2012,
  `Telit_Jupiter_JF2_JN3_MEMS_Application_Note_r2.pdf`; [D12] Using SiRF Star IV with an External Host Rev.0,
  2011, `Telit_Using_SiRF_STAR_IV_with_an_external_Host_Application_Note_r0.pdf`.
- [D13] OriginGPS, Hornet ORG1411 Datasheet Rev 4.1, 2018,
  <https://download.mikroe.com/documents/datasheets/Hornet-ORG1411.pdf>
- Eggtimer Rocketry, under <https://eggtimerrocketry.com/wp-content/uploads/>: [B1]
  `2018/06/Eggfinder_Users_Guide_RevB6.pdf`; [B2] to [B5] TX manuals B4d, B6, B7 (`2018/06/`), C1 (`2022/01/`);
  [B6], [B7] Mini A7 (`2018/10/`), A10 (`2020/09/`); [B8] TX D3-D4, Mini C3b (`2025/03/`); [B9]
  `2018/06/Eggtimer_TRS_Users_Guide_2_07.pdf`; [B10] TRS assembly manuals A5, B2c (`2018/06/`), B4 (`2022/04/`);
  [B12] `2026/07/Eggfinder-LCD-Release-Notes-2_05c.pdf`; [B13]
  `2018/06/Eggfinder_LCD-GPS_Assembly_Users_Guide_w_Pix.pdf`; [B14] `2021/07/OpenLog-and-Eggfinder-Users-Manual.pdf`.
  [B21] <https://eggtimerrocketry.com/eggfinder-tx-support/> and `/eggfinder-mini-support/`.
- [B15] Iso-Haisu 2011 flight page, <https://haisunaata.aapo.iki.fi/realtime/2011/>, and its `data/news.html`
  (the 2011-02-02 date); [B16] its
  `data/realtime-flight-track.kmz`; [B19] <https://haisunaata.aapo.iki.fi/teknisia_tietoja_iso.html>.
- [B17] Missile Works T3 GPS, <https://store2263081.ecwid.com/T3-XB-GPS-Tracking-System-p90226446>.
