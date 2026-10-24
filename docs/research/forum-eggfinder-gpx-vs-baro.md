# A real Eggfinder GPX beside its flight's barometric log

This note reads Rocketry Forum thread 135349 [S1]: an Eggfinder GPS track saved as GPX by the MapSphere app, the
StratoLogger CF `.pf2` of the same J415W flight (2016-07-16), and an Eggtimer TRS receiver log from LDRS 35
(2016-06-11) with GPS and barometer in one stream. It asks what the GPX `ele` is measured from, how far GNSS
altitude misses the barometer in each phase, and what HPR Sim reads today. It serves [M7.2d](../ROADMAP.md)'s GNSS
trust rule, evidence row [G1](../evidence.md#g1) (#654, with #645's datum) and M10.2's analyzer evidence, and
builds on [gnss-in-boost](gnss-in-boost.md) and [public-logs-evidence-gaps](public-logs-evidence-gaps.md) §1.
Read 2026-10-23; counts come from scripts run on the cached files. **Inference** marks this note's reasoning.

**In short.**

- **`ele` is most likely above mean sea level, as NMEA GGA states it.** The pad's `ele` sits 7 m below the USGS
  terrain height; read as ellipsoidal it would be 26 m above. The receiver log's GGA geoid separation, −31.0 m,
  is within 0.5 m of EGM96. Moderate confidence: one app, one receiver, no MapSphere document.
- **GNSS altitude misses the climb rather than lagging it (Inference, §4).** At the 1,406 m barometric apogee it
  reads 1,392 m low; it peaks 252 m above the pad 62 s later, rising through the drogue descent, then sits 6 to
  82 m above the barometer under the main. The LDRS flight repeats it: 608 m low, 40 s late.
- **It agrees within the barometer's error only at T+85 to 87 s**, crossing it on the way up, even allowing a
  hot-day bias of up to +8.4%. Within 21 m, its own offset after landing, it holds only from T+115 s.
- **A one-point speed test passes at apogee** (1.1 vs 2.6 m/s, height 1,392 m off); a 2 s version first passes
  at T+95 s, 78 m off. The LDRS fix stays valid with 7 to 9 satellites.
- **HPR Sim reads all seven distinct files and refuses none**, but drops the TRS's barometer lines as noise,
  passes 1,270 repeated GPX times without a note, and puts the `.pf2`'s main opening 21.3 s late, outside its bound.

## 1. The files and what "Adj" means

The thread's attachments hold seven distinct files (SHA-256 in `refs/.../SHA256SUMS`; three are duplicates by
hash). "Adj" files are the original logs with each altitude replaced by a barometer's, done by a poster's
spreadsheet (posts 13, 23 and 34 [S1]); none of their altitudes is a GNSS measurement.

- **Raw GPX** (`gpslog_16072016_151524`, post 1): 245 points, 22:10:12 to 22:14:53 UTC, `ele` in meters, a
  MapSphere `<valid>` extension on 131 points: `0` on every point from the second to T+0, on one at T+15 s and
  on all 29 from T+129 s. MapSphere states no meaning for it.
- **Adj GPX** (post 13, and its twin in the post-23 spreadsheet): the same positions and times. Each `ele` in the
  flight equals the `.pf2` height at (GPX time − 22:11:55 UTC) plus 189 ft, on all 116 points from 22:11:57 to
  22:14:05, to the foot. The 102 points before read 190 ft, and the 27 after the `.pf2` ends read 232 ft, its
  header's ground elevation. The post-13 copy adds one point with an empty `time`.
- **`.pf2`**: 2,664 samples at 0.05 s, feet above the pad; header apogee 4,613 ft, ground 232 ft MSL, drogue at
  19.00 s, main at 87.15 s.
- **TRS raw NMEA** (post 21; the post-34 spreadsheet's input): 3,979 good GGA, 4,129 barometer lines `<n>`.
- **TRS Adj NMEA and GPX** (post 34): GGA altitude = the next `<n>` + 2,855 ft, in meters, on 1,290 of 1,293 sentences
  (at 16:22:20, where the raw log has no `<n>`, the mean of its neighbors); the GPX copies those altitudes in order,
  but truncates each time to the minute (23 distinct times).

## 2. What `ele` is measured from

GPX 1.1 defines `ele` only as "Elevation (in meters) of the point" and `geoidheight` as the geoid's height above
WGS84 "as defined in NMEA GGA message" [S2]. GGA field 9 is the antenna altitude above mean sea level (geoid) and
field 11 the geoidal separation [S3]. The Eggfinder's receiver is a SiRFstarIV part: its maker names the Maestro
Wireless A2235H (post 28 [S1]), a SiRFstarIV module whose NMEA output offers GGA [S4].

| Pad | Value | Minus terrain [S6] | Read as ellipsoidal |
|---|--:|--:|--:|
| 2016-07-16, raw GPX `ele` (101 pad points) | 47.7 to 48.4 m | −7.4 to −6.7 m | +26.0 to +26.7 m |
| same, `.pf2` "Ground Elevation" | 232 ft (70.7 m) | +15.6 m | n/a |
| same, Adj's pad (the poster's typed elevation) | 190 ft (57.9 m) | +2.8 m | n/a |
| 2016-06-11 LDRS 35, GGA altitude at launch | 874.2 m | +5.8 m | +37.3 m |
| same, the poster's launch altitude | 2,855 ft (870.2 m) | +1.8 m | n/a |

Terrain [S6] is 55.1 m at the 2016-07-16 pad, 55.3 m at its landing point, 868.4 m at the LDRS pad (NAVD88,
within 0.25 m of EGM96 at both by NGS GEOID18). Ellipsoidal: `ele` less EGM96's −33.37 m and −31.44 m [S7]. The
receiver's own separation was −31.0 m in all 3,979 GGA, 0.44 m from EGM96 and 0.31 m from EGM2008 [S7].

- **Conclusion (Inference):** MapSphere wrote the GGA altitude, above mean sea level by the receiver's geoid
  model. The mean-sea-level reading misses the terrain by 7 m, the ellipsoidal one by 26 m. The receiver's own
  scatter at rest is about that size: −7 m before launch, +14 m after landing, so one pad cannot exclude the other
  datum outright. No file carries `geoidheight`, so #645's rule (sea level when `geoidheight` is present) is untested.
- **The `.pf2` ground elevation is no datum check.** PerfectFlite calls it "approximate" [S5] p. 41 and converts
  pressure by the US Standard Atmosphere [S5] p. 2 (that the elevation comes from pad pressure is **Inference**).
  Altimeter settings of 29.77 to 29.78 inHg at 22:12 UTC [S8] give 312 to 321 ft pressure altitude here
  (**Inference**), so 232 ft is 51 ft above terrain and 79 to 89 ft below that, unexplained.

## 3. Lining the two logs up

The `.pf2`'s zero is 22:11:55 UTC, 103 s after the GPX's first point, as post 13 chose [S1]. The Adj file reproduces
that offset exactly; shifting it 0.05 s breaks the fit on 102 (+0.05 s) or 104 (−0.05 s) of 116 points. Two
independent marks bound it (**Inference**): the track first moves 9 m from the pad at 22:11:57, and its position stops
changing at 22:14:02 to 22:14:04, against a barometric landing at 129.45 s. The landing mark gives a zero of
22:11:52.6 to 22:11:54.6, the first-motion mark (liftoff at 0.45 s) 22:11:54.6 to 22:11:56.6, so this note takes
22:11:55 within −2 and +1 s. Two seconds move the barometric height by about 2 s times the phase's speed: about 320 m
in boost, 35 m under the drogue at its mean rate, 8 m under the main; no conclusion below changes.

## 4. GNSS minus barometer, phase by phase

Heights are above each sensor's pad: GNSS `ele` less 48.4 m, its last pad point; the barometer at the GPX time.
Rates: GNSS from consecutive points, the barometer by a 2 s least-squares fit. Phases from the `.pf2`.

| Phase (T+, s) | Points | Barometer, m | GNSS, m | GNSS − baro, m | Rates, GNSS vs baro, m/s |
|---|--:|---|---|---|---|
| Boost, 0 to 3.35 | 2 | 0.3, 118.3 | 0.4, 0.7 | +0.1, −117.6 | 0.4 vs 21.7; 0.2 vs 122.5 |
| Coast, 3.35 to 18.25 | 8 | 828 to 1,405 | 1.3 to 12.9 | −826 to −1,392 | 0 to +6.2 vs +111 to +2.6 |
| Drogue, 19.0 to 87.15 | 65 | 1,376 to 223 | 14.1, 251.7, 237.1 | −1,362 to +16.7 | −3 to +23 vs −30 to −4 |
| Main, 87.15 to 129.5 | 41 | 200 to 3 | 234.0 to 18.0 | +6.1 to +82.3 | −11.7 to +1.5 vs −24.6 to +0.7 |
| Landed, T+130 to 178 | 28 | 0 assumed after the log ends | 19.3 to 21.0 | +19 to +21 | about 0 vs 0 |

- **Drogue:** GNSS rises on 55 of 58 points from T+20 to T+80 s, holding on 3, while the barometer falls.
  **Landed:** the `.pf2` reads −0.3 m at T+130 s, then 13 m from T+131 s to its end; the table takes 0.
- **Spacing:** 117 points, T+0 to T+130 s: 106 gaps of 1 s, 8 of 2 s, one of 3 s, one of 5 s (T+2 to T+7); the
  receiver reports at 1 Hz (post 9 [S1]). Gaps are likely lost radio packets (**Inference**; the LDRS log, another
  link, has 356 bad-checksum and 40 cut sentences by `hpr analyze`); Bluetooth drops (post 1) can't be excluded.
- **Apogee:** barometer 1,406.3 m at T+18.25 s (raw peak 1,407.0 m at 18.20 s); GNSS at T+18 s, 12.9 m. The GNSS
  peak, 251.7 m at T+80 s, is 1,154.6 m (82%) low and 62 s late; the barometer then reads 329 m.
- **Crossing:** GNSS passes the barometer near T+85 s, then reads high by up to 82.3 m (T+97 s), closing to
  +6.1 m at T+127 s. After landing it rests 19 to 21 m above its pad; the terrain there is 0.2 m above the pad [S6].
- **Shape (Inference):** this is not a delayed copy of the climb, which would overshoot after apogee; the vertical
  solution moves at most 23 m/s between points and stays near the pad through boost. Its cause in the receiver is
  undocumented and unbounded here; the maker says altitude "will lag somewhat" on fast flights (post 9 [S1]).

## 5. The barometer's own error, and when they agree

The StratoLogger CF states measurement precision ±(0.1% of reading + 1 ft) and calibration ±0.05%, both typical [S5]
p. 47: ±1.7 m at apogee from precision, ±2.4 m with calibration. Its height is the pressure difference by the US
Standard Atmosphere [S5] p. 2. The day was hot: 99 to 102 °F at Lemoore and Fresno between 21:53 and 22:56 UTC [S8],
22.6 to 24.3 K above standard at the pad. A standard-atmosphere altimeter then reads the climb low by ΔT/T0 of the
reading ([altimeter-height-formulas](altimeter-height-formulas.md) §3): 7.8 to 8.4%, 110 to 119 m at apogee, if the
whole column was that warm. No sounding was read, so that is an upper value (**Inference**).

- **Band (Inference):** a true GNSS height sits within −(0.15% + 0.3 m) and +(8.4% + 0.3 m) of the barometer.
  In flight only T+85 to 87 s fall inside, where the GNSS track crosses the barometer on its way up.
- **With GNSS's own rest error** (−7 m to terrain on the pad; landed, 19 to 21 m above its pad reading), 21 m holds
  from T+115 s to landing (14 points, under 60 m), after a 4 s pass at T+84 to 87 s.
- **Speed (for [gnss-in-boost](gnss-in-boost.md)'s test):** point rates agree within 3 m/s on 24 of 114
  points before T+128 s, at T+18 s (1.1 vs 2.6 m/s, height 1,392 m off), at T+90 to 113 s (height 27 to 82 m off)
  and from T+115 s. The candidate test's 2 s first hold at T+94 to 95 s (76 to 78 m off); 10 s at T+125 s (9.9 m).

## 6. The second flight: LDRS 35, receiver and barometer in one stream

The TRS sends its barometric height between the NMEA sentences, once a second (posts 15, 21 [S1]). Read as feet above
the pad (the spreadsheet's conversion; **Inference**), it shows launch at 16:22:04 UTC and apogee 765.4 m (2,511 ft)
at T+13 s; after landing it repeats 2,514 ft, the apogee it reports after landing (post 34 [S1]), beside 2,680 GGA.

- **GNSS:** 21.6 m above its pad at the barometer's apogee (744 m low); peak 157.8 m at T+53 s, 608 m (79%) low and
  40 s late; then above the barometer from T+53 s: 88 m at T+70 s, 99 m at T+74 s, the last flight sentence.
- **Fix quality said nothing:** GGA quality 1, 7 to 9 satellites, HDOP 0.9 to 1.3 and a 3-D fix in every flight
  GSA; the only quality-6 (estimated) GGA is on the pad. The thread doesn't name the TRS's receiver.

## 7. What HPR Sim reads today

`hpr analyze <file>` on a build of 2026-10-23; the readers' code checked on main at `0ece4ab8`. None refused.

| File (SHA-256 prefix) | Outcome |
|---|---|
| `.pf2` (`87e9cc91`) | Read. Apogee 1,406.3 m at 18.25 s; liftoff 0.45 s; landing 129.45 s; main's opening 108.45 s |
| raw GPX (`893b5951`) | Read: 245 points, 3 channels; every reading withheld (GNSS only); `extensions` counted |
| Adj GPX, post 13 (`9d51c3d9`) | Read: 246 points, the empty `time` a gap; withheld as GNSS, `ele` being barometric |
| Adj GPX, spreadsheet (`cd969f4c`) | Read: 245 points; as above |
| TRS Adj GPX (`3dcc1a9a`) | Read: 1,293 points; 1,270 share the time before them, with no note |
| TRS raw NMEA (`4b002742`) | Read: 66 channels; 8,333 lines skipped as noise, the 4,129 barometer lines among them |
| TRS Adj NMEA (`e1961240`) | Read; its GGA altitude, barometric plus 2,855 ft, is labeled a GNSS altitude |

- **The `.pf2`'s main opening is 21.3 s late, outside its stated 0.76 s.** The 4 s rate falls from −18.2 to
  −7.6 m/s between T+88 and T+92 s, and the header gives the main at 87.15 s. The drogue rate printed, 14.6 m/s
  within 1.2 m/s, would be about 17.3 m/s on the header's main time (1,406.3 m to 211.5 m over 68.9 s; 17.2 m/s
  from the last sample before the charge). **Inference** (`readings/canopies.rs`): the walk back from landing
  stops where the 2 s running median of 1 s fits first passes its mark (about 5.8 m/s: the near-ground rate,
  2.8 m/s, plus 3), at T+107 to 108.4 s; the faster stretch from T+88 to 99 s is never reached.
- The withheld reason says a GNSS filter "overshoots its top"; both real tracks peaked far below it instead.

## What this suggests

- **M7.2d's trust rule:** gate GNSS on height agreement held for a window, not on speed alone, the fix flag or the
  satellite count (speed passes at apogee 1,392 m off). A refs-gated test can pin it: aligned at 22:11:55, a
  height window of 5 s or more gives GNSS zero weight at T+18 s and through T+113 s.
- **G1 (#654):** the raw GPX and `.pf2` are a real pair, cached with SHA-256, license "none stated, cite only".
  A refs-gated test asserting 245 points, `ele` 47.7 to 300.1 m and no `geoidheight` would make the row `real`.
  An invented GPX of the same shape (pad, 1 Hz, a slow climb to a late low peak) keeps CI's test.
- **#645:** add this pad's evidence (−7 m sea level, +26 m ellipsoid) to the reader note's sources.
- **An issue for the `.pf2` main opening** (M10.2): a refs-gated test asserting the opening within 87 to 92 s
  fails today, as a new check should. A second asserts the drogue rate within 1.5 m/s of 17.3 m/s.
- **Smaller fixes:** a note when GPX times repeat; a TRS `<n>` reader once its unit is documented (the LDRS log
  would be a one-file pair); "lags or misses" in the reason.
- **M7.2f:** the pair's known human offset (103 s) is a real case for recovering an offset between two logs.

## Open questions

- MapSphere's source for `ele` and the meaning of its `<valid>` flag: no document found.
- Why the A2235H's vertical solution stalls: no dynamic-model or altitude-filter document read; unbounded.
- Why the `.pf2` ground elevation sits 79 to 89 ft below the day's pressure altitude.
- The column's temperature aloft: a sounding would turn the 8.4% upper value into a measured bias.
- The TRS `<n>` format and unit from Eggtimer's own manual; and which receiver the TRS used.
- Whether `<n>` belongs to the GGA before or after it (post 24 [S1] asks). If after, apogee pairs with the
  16:22:18 GGA: GNSS 28.0 m, 737 m low, 39 s late.

## Sources

Read 2026-10-23. [S1]'s files are cached in `refs/research/forum-eggfinder-gpx-vs-baro/files/` with `SHA256SUMS`,
scripts and lookups beside them, never committed; no terms are stated, so cite only. The rest are cited.

- [S1] Rocketry Forum thread 135349, "EggfinderTx --> Mapsphere, In-flight Position and Altitude Peformance?",
  posts 1 to 34, 2016-07-20 to 07-24, pages 1 and 2 read without a login,
  <https://www.rocketryforum.com/threads/135349/>; attachments from its `cdn.rocketryforum.com/a/` links.
- [S2] Topografix, GPX 1.1 schema, `ele` and `geoidheight`, <https://www.topografix.com/GPX/1/1/gpx.xsd>.
- [S3] gpsd, "NMEA Revealed", GGA section, <https://gpsd.gitlab.io/gpsd/NMEA.html>.
- [S4] Lantronix, A2235-H product page (tech specs), printed at <https://www.farnell.com/datasheets/3708589.pdf>.
- [S5] PerfectFlite StratoLoggerCF manual, printed pp. 2, 41, 47, <http://www.perfectflite.com/Download.html>.
- [S6] USGS Elevation Point Query Service, <https://epqs.nationalmap.gov/v1/json> at the three points above.
- [S7] GeographicLib GeoidEval, EGM96 and EGM2008, <https://geographiclib.sourceforge.io/cgi-bin/GeoidEval>.
- [S8] Iowa Environmental Mesonet, ASOS NLC and FAT, 2016-07-16 20:00 to 24:00 UTC,
  <https://mesonet.agron.iastate.edu/request/download.phtml>.
