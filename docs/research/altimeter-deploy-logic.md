# How common altimeters detect apogee, lock out Mach and fire backups

What does each common hobby altimeter fire its drogue on, how does it keep a Mach dip from firing it, what is its
main altitude measured from, and what are the defaults and ranges of those settings? It serves the settings check
(M6.8 in the [roadmap](../ROADMAP.md)), whose table is in [suite field tools](suite-field-tools.md#the-settings-check),
and the early and late deployment faults (M7.4). It builds on [altimeter Mach error](altimeter-mach-error.md), which
covers how far a baro reads wrong near Mach, and [altimeter height formulas](altimeter-height-formulas.md).
Read 2026-10-17: eleven current manuals and two sets of release notes from seven makers, each checked against the
maker's download page that day; makers' documentation only, no firmware source. **Inference** marks this note's own
reading. Trust the tabulated settings (each a maker's printed number); where a manual states no threshold, none is given
here.

**In short.**
- **Three ways to find apogee.** Baro speed reaching zero (StratoLoggerCF, RRC2+/RRC2L/RRC3, Eggtimer), a Kalman filter
  (Altus Metrum, fusing an accelerometer where the board has one; the AIM XTRA adds GPS), or a 2-of-3 vote of baro, gyro
  and accelerometer (Blue Raven). Eggtimer fires 1 s after the highest altitude, so its true delay is the setting + 1 s.
- **Two Mach lockouts are numbers you type:** Altus Metrum's, a time after launch its manual says to leave at 0, and
  Blue Raven's Vel1, a speed (default 400 ft/s, 122 m/s) that gates only its baro backup. PerfectFlite and Missile Works
  lock out automatically on an unpublished velocity test; Eggtimer waits for baro speed under 100 ft/s (30.5 m/s) for
  1 s; the AIM and Chute Release have no Mach rule. So M6.8's lockout rows come in three kinds, most with no setting.
- **Every main altitude is height above the pad's own baro reading**, fired only on descent, with defaults of 300
  to 820 ft and fixed ranges topping out between 1,000 ft (RRC2+) and 9,999 ft (SLCF). Only the AIM takes
  meters; Altus Metrum takes either; the rest take feet. Only Altus Metrum and Chute Release say what happens
  when apogee is below the main altitude (main 2 s after the apogee charge; release at once).
- **No two makers write backups alike:** a delay after apogee (0 to 5 s SLCF, 1 to 30 s RRC3, 0 to 30 s Eggtimer), a
  lower second main, or a descent-speed trigger (Eggtimer FailSafe 100 ft/s for 1 s; Blue Raven −200 ft/s). On
  Altus Metrum a delay cancels if its conditions turn false; on the Ravens it fires anyway.

## 1. Apogee detection

| device | fires apogee on | stated latency or smoothing | apogee delay setting |
|---|---|---|---|
| StratoLoggerCF | baro speed = 0, 20 samples/s | "typically within +/- 0.2 seconds" | 0 to 5 s, 1 s steps, default 0 |
| RRC3, RRC2L | baro "displacement velocity of 0" | 21-element FIR on 20 Hz data | RRC3 1–30 s (default 1), RRC2L 0–9 |
| RRC2+ | baro; refers to the RRC3 manual | not stated | DIP 2: apogee + 1 s |
| Quantum, Proton | 1 s past the highest baro altitude | filter not described | 0–30 s; Proton default 0 |
| Blue Raven | 2 of 3: baro rising, tilt > 90°, accel speed ≤ 0 | 50 Hz checks, baro filtered | each channel, 1,500 ms |
| EasyMini, TeleMega | Kalman vertical speed = 0 | baro de-weighted near Mach | seconds; no default or range |
| AIM XTRA | Kalman of GPS, baro and two accelerometers | baro 20 ms, accels 10 ms | Time Delay per line; no range |
| Chute Release | no apogee; waits until falling | spike filter since firmware 1.2 | none |

Sources by row: [SL] PDF pp. 4, 25, 27, 49; [R3] pp. 10, 12, 24, [RL] p. 7; [R2] pp. 1, 3; [E1] pp. 21, 30–31, [E3] p.
25;
[F1] pp. 13, 15–16; [A1] pp. 39, 66–67; [EN] PDF pp. 6–7, 22–23; [JL] PDF p. 5, [JP].

- **Eggtimer** uses the baro for deployment even on the Proton, which has an accelerometer [E3] p. 37. Since firmware
  1.08Q (Quantum) and 1.04U (Proton) it also declares nose-over after 1 s below apogee at under 400 ft/s [E2] p. 3,
  [E4] p. 4. The Quantum guide gives the drogue delay as "+0 to +9 secs" on p. 2 and up to 30 s on p. 6; release 1.07A
  raised the maximum to 30 s [E1] pp. 2, 6, [E2] p. 6. The Quantum guide recommends zero for dual deploy but
  names no default [E1] p. 21. **Inference:** the drogue fires about 1 s plus the setting
  after true apogee, plus an unstated filter lag.
- **RRC3** oversamples, reads at 50 Hz and averages to 20 Hz before the FIR [R3] p. 24. **Inference:** 21 taps at
  20 Hz span 1.05 s, so a symmetric FIR would lag about 0.5 s; the manual calls it "recursive", whose lag a tap
  count does not fix, and the taps aren't published, so this is not a bound.
- **Blue Raven's** default Apo channel needs a burnout count of at least 1 and latches the apogee event [F1] pp. 11,
  13, 25. The accelerometer vote assumes vertical flight [F1] p. 16. The 3rd channel is the backup apogee: baro pressure
  increasing while accelerometer speed is under 400 ft/s ("Vel1", a "transonic inhibit") [F1] pp. 11, 14, 25.
- **Altus Metrum** fuses baro and accelerometer below 30 km; the baro-only EasyMini has "no need to set a 'mach delay'"
  [A1] pp. 63–64, 66. Accelerometer-only apogee under-estimates on a parabolic path [A1] p. 67. **Inference:** it fires
  early.
- **AIM** lines each take a mode: Apogee, Altitude after apogee, Climb altitude, Time, Peak velocity, Input, Disabled;
  each output mode has an On Time and a Time Delay [EN] PDF pp. 19, 22–23. The baro can be dropped for sealed bays [EN]
  PDF p. 21.

## 2. Mach lockout: three kinds

- **A time you type (Altus Metrum).** "Apogee lockout": seconds after launch in which apogee cannot fire; default 0,
  and the manual says leave it there because the Kalman filter is not fooled [A1] p. 39. Before firmware 1.8.1,
  boards with accelerometers ignored the baro above 330 m/s, which fired drogues early on long supersonic flights
  [A1] p. 110.
- **A speed test (Eggtimer fixed, Blue Raven typed).** Eggtimer enables its outputs once noise-filtered baro speed stays
  under 100 ft/s for at least 1 s after launch detect; shocks above about 800 ft/s (244 m/s) can fake a descent [E1] p.
  30, [E3] p. 37. The maker says the zippers it knows of came from rockets flying horizontally [E1] p. 30. Blue Raven's
  Vel1 (adjustable per channel, default 400 ft/s) gates only channels that use "pressure increasing" [F1] pp. 14, 25;
  the 2-of-3 primary has no Mach number.
- **Automatic and unpublished, or absent.** StratoLoggerCF "MachLock" waits for speed to stay below a "predetermined
  threshold for a period of time" ([SL] PDF p. 40); "No mach delay is necessary" (PDF p. 30). A shop-vac test trips it
  and delays apogee by "several seconds" (PDF pp. 40–41). Missile Works claims "built-in mach immunity" ([R2] p. 1,
  [RL] p. 1); the RRC3 drops anomalous velocity changes, worst sonic to subsonic ([R3] p. 24). No thresholds. The
  AIM manual has no Mach logic (port sizing only, [EN] PDF p. 16); Chute Release never fires near apogee.

## 3. Main altitude and its ground reference

| device | range and step | default | zero taken from | apogee below main |
|---|---|---|---|---|
| StratoLoggerCF | 100–9,999 ft, 1 ft (PC) or 100 ft (buttons) | 700 ft | pad pressure tracked to launch | not stated |
| RRC3, RRC2L | 300–3,000 ft, 100 ft | 500 ft | 10 s start-up, then averaged to arming | not stated |
| RRC2+ | DIP: 300, 500, 800, 1,000 ft | 300 ft (p. 2 says 500) | not stated | not stated |
| Quantum | 100–500 ft by 50, 600–3,000 by 100 | 500 ft | readings about 10 s after arming | not stated |
| Proton | 100–500 ft by 50, 600–2,000 by 100 | 500 ft | readings about 10 s after arming | not stated |
| Blue Raven | not stated (Raven4: 32 ft steps) | 700 ft; 4th 500 ft | "from baro sensor", time unstated | not stated |
| EasyMini, TeleMega | typed freely, m or ft | 250 m (820 ft), EasyMini | recorded pad height | main 2 s after apogee |
| AIM XTRA | meters, range not stated | not stated (example 150 m) | not stated | not stated |
| Chute Release | 100–800 ft by 100, then 1,000 ft | not stated | sampled once at power-on | releases at once |

Sources by row: [SL] PDF pp. 6, 21, 23–26; [R3] pp. 3, 9, 11, 23, [RL] pp. 6–7; [R2] pp. 1–3; [E1] pp. 21, 29;
[E3] pp. 28, 36; [F1] pp. 12, 14, 22, 25, [F2] p. 8; [A1] pp. 15, 39; [EN] PDF pp. 19, 22–23; [JL] PDF pp. 3, 5–6, [JP].
- **Units.** RRC settings are feet codes worth 10 or 100 ft whatever the Units setting [R3] pp. 11–12; Eggtimer and
  Blue Raven take feet; the AIM's spinners are metric only [EN] PDF p. 23; Altus Metrum enters the main in the
  preferred unit since release 1.3 and exports metric [A1] pp. 46, 131.
- **Rounding.** Raven4 stores altitudes in 32 ft steps [F2] p. 8. **Inference:** a typed 700 ft is held as 672 or
  704 ft. The Blue Raven's step is not stated.
- **Chute Release** 5X and 10X editions multiply every level by 5 or 10 [J5], [J10]. Chutes need 50 to 125 ft below
  the setting to open ([JL] PDF p. 2), 50 to 150 ft on the product page [JP]. The guide says to power on close to
  launch for an up-to-date ground level ([JL] PDF p. 6). **Inference:** near sea level 1 hPa is about 8.3 m
  (dh/dp = RT/(gp), T = 288 K), so a ground pressure fall of 1 hPa after power-on releases it about 8 m low.
- **Apogee below main, by reading.** **Inference:** RRC's phase "Apogee was detected and Altitude > Main Altitude"
  ([R3] p. 19) implies main at apogee; Eggtimer's condition is already true at nose-over; Blue Raven's main fires
  1.5 s after the Apo channel ([F1] pp. 15, 25). None of these is a stated rule.

## 4. Backups and delays

- **StratoLoggerCF:** for two units, give one an apogee delay and the mains different altitudes; no numbers [SL]
  PDF pp. 26–27. The manual warns a delay risks a fast deployment (PDF p. 27).
- **RRC3/RRC2L:** mode 2 "Dual Deploy Backup" fires the drogue at apogee + Drogue Delay; if main comes due during
  the delay, the drogue fires at once [R3] pp. 11–12. Mode 3 "Apogee Only" fires main at apogee + 1 s. The Aux
  output can be a timer or altitude backup; the manual's examples are a drogue at apogee + 5 s and a 1,000 ft main
  behind a 1,500 ft primary [R3] p. 22.
- **Eggtimer:** as the second of two units, set the drogue 1.0 to 1.5 s (Quantum) or 1.0 to 3.0 s (Proton) after the
  primary, checked by simulation [E1] p. 21, [E3] p. 25. FailSafe fires the main on fast descent after nose-over:
  Quantum 50 to 200 ft/s by 25, 500 to 3,000 ms by 250 [E1] p. 28; Proton 50 to 300 ft/s (default 100), 0.3 to 3.0 s
  (default OFF) [E3] p. 29; both recommend 100 ft/s for 1 s; the Quantum guide says most rockets reach that 4 to 5 s
  after apogee.
- **Blue Raven:** the Main channel's second condition set fires on apogee plus baro speed below −200 ft/s (−61 m/s,
  "Bvel1", scaled with pressure); the 4th fires below 500 ft [F1] pp. 12, 14, 25–26. A channel delay fires even if its
  conditions turn false [F1] p. 15. Raven4 defaults: 0 s on Apo and Main, 2 s backup apogee, 1.5 s backup main [F2]
  p. 5.
- **Altus Metrum:** fixed-channel modes include redundant apogee or main, the second charge 2 s later [A1] p. 41.
  TeleMega's extra channels fire when every enabled condition holds (acceleration, ascent rate, height, tilt, time,
  motor count, state, delay); the delay cancels if any condition turns false [A1] pp. 42–44. Recommended: redundant
  apogee with flight state after drogue, + 1 to 2 s, redundant main below the primary [A1] p. 66.
- **AIM:** any line can be a Time-mode backup or Apogee with a Time Delay; On Time of 2 s or more [EN] PDF p. 23.
- **Chute Release:** none.

## 5. Launch detection and arming

A flight that never reaches the arming height fires nothing ([SL] PDF p. 27 says so outright).
- StratoLoggerCF 160 to 300 ft, default 160 ft [SL] PDF p. 27. RRC3/RRC2L 100 to 300 ft by 10, default 300 ft; RRC2+
  fixed 200 ft [R3] pp. 3, 11, [R2] p. 4. RRC refuses to arm if the pad already meets the arming height [R3] p. 10.
- Eggtimer 50 to 500 ft by 50, default 200 ft; the Proton also at 3 to 20 g held 0.2 to 2.0 s (default 3 g, 0.5 s)
  [E1] p. 24, [E3] pp. 31–32. The Proton won't arm until 3 minutes after power-on [E3] p. 20.
- AIM: Kalman speed above 30 m/s for 0.5 s [EN] PDF p. 40. Chute Release: above 100 ft for 3 s [JL] PDF p. 5.
- Altus Metrum: Boost to Fast to Coast at 200 m/s ascent; launch threshold not stated [A1] pp. 7, 44. Blue Raven:
  liftoff rule not stated; Raven4 counts readings above 3 g until they add to 3 ft/s [F2] p. 4.
- Landing rules disagree inside Eggtimer's own guides (30 ft for 5 s, or 10 ft change in 5 to 10 s after half of
  apogee) [E1] pp. 31, 37, [E3] p. 38, [E4] p. 11; RRC: speed above −3 ft/s for 2 s after main [R3] p. 10.

## What this suggests

1. **M6.8 reads a lockout kind per device, not one number.** Altus Metrum: lockout time against the earliest apogee
   (low side) and the latest time above Mach. Eggtimer: the simulated time when speed stays under 100 ft/s for 1 s
   must fall before the lowest-bound apogee, a model check with no setting. Blue Raven: Vel1 against the latest
   time above 400 ft/s, on baro channels only. SLCF, RRC, AIM, Chute Release: "automatic, threshold unpublished" or
   "none", with the flight's time above the Mach rule of [altimeter Mach error](altimeter-mach-error.md) as a warning.
2. **M6.8's apogee-delay row adds Eggtimer's 1 s** to the typed delay before comparing descent speed with the drogue.
3. **M6.8's main row is well defined for every device** (AGL from the pad, descent only), but the unit trap is real:
   AIM meters, Altus either, the rest feet; RRC codes in 10 and 100 ft. A Raven4 setting is rounded to 32 ft.
4. **M6.8's arming row:** the lowest-bound apogee must clear the arming height (SLCF default 160 ft, RRC 300 ft,
   Eggtimer 200 ft); below it nothing fires.
5. **M7.4's early-main fault** has two stated causes: apogee below the main (Altus Metrum 2 s after apogee, Chute
   Release at once) and a drogue failure tripping FailSafe or Bvel1 near apogee; an accelerometer-only apogee on a
   tilted path firing early is a third (**Inference** from [A1] p. 67). Its late-drogue fault: the delays above,
   Eggtimer's 1 s, RRC3's ~0.5 s filter (**Inference**) and MachLock's "several seconds" seen in ground tests.
6. **Backup offsets stay a rule** in M6.8 (backup after primary, its main above the low edge): makers' advice runs
   1 to 3 s between charges (RRC3's example uses 5 s) and gives no number for a lower main.

## Open questions

- The thresholds and hold times of MachLock and Missile Works' velocity test, the RRC3 filter taps, and Eggtimer's
  filter lag; none is published, so M7.4 cannot bound their late side from these sources.
- Behavior with apogee below the main for StratoLoggerCF, RRC, Eggtimer, Blue Raven and the AIM (read, not stated).
- Whether Blue Raven's default Apo channel waits its stated 1,500 ms: the defaults table has no delay row [F1] p. 25.
  Its quick start says the 4th channel also fires on fast descent [F1] p. 5; the table shows that only on Main.
- Blue Raven's liftoff, landing and ground-pressure timing; Altus Metrum's launch threshold, pad averaging, and its
  apogee delay's range; the AIM's ranges, defaults and pad zero; Chute Release's default level.
- The RRC2+ manual gives its default main as 300 ft on pp. 1 and 3 and 500 ft on p. 2.

## Sources

All are makers' documents read 2026-10-17. Eggtimer ("All Rights Reserved"), Featherweight, PerfectFlite, Missile
Works, Entacore and Jolly Logic state no reuse license: cited, cached privately, not copied. Altus Metrum's manual
is under the Creative Commons ShareAlike 3.0 License. Pages: "PDF p." is the PDF page (SLCF prints PDF − 2,
AIM PDF − 4); Altus Metrum pages are printed (PDF = printed + 6); the rest give PDF pages (release notes and Raven4
print none).
- [SL] PerfectFlite, StratoLoggerCF manual, 50 pp. (Last-Modified 2021-10-27),
  <http://www.perfectflite.com/Downloads/StratoLoggerCF%20manual.pdf>.
- [R2] Missile Works, RRC2+ User Manual rev. 1.03; [RL] RRC2L User Manual v1.02 (firmware 1.73); [R3] RRC3+/RRC3
  User Manual rev. 1.71, May 2024. All via <https://www.missileworks.com/downloads>.
- [E1] Eggtimer Quantum User's Guide 1.09G (2024); [E2] Quantum Release Notes 1.09G; [E3] Eggtimer Proton User's
  Guide 2.01C (2024); [E4] Proton Release Notes 2.01C. Under <https://eggtimerrocketry.com/wp-content/uploads/2024/02/>.
- [F1] Featherweight, Blue Raven User's Manual, 2026-05-12; [F2] Raven User's Manual (Raven4), 2020-08-15. Under
  <https://www.featherweightaltimeters.com/uploads/1/0/9/5/109510427/>.
- [A1] Packard, Garbee, Finch, Towns, *The Altus Metrum System*, v1.9.22.2, 13 Oct 2025,
  <https://altusmetrum.org/AltOS/doc/altusmetrum.pdf>.
- [EN] Entacore, AIM XTRA & BASE Manual 4.x.x v7, 2025-02-02,
  <https://downloads.entacore.com/aimxtra/AIM%20XTRA%20&%20BASE%20Manual%204.x.x%20v7.pdf>.
- [JL] Jolly Logic, Chute Release guide (2015, scanned),
  <https://jollylogic.com/wp-content/uploads/2015/12/JollyLogicChuteReleaseWeb.pdf>; [JP]
  <https://jollylogic.com/products/chuterelease/>; [J5] <https://jollylogic.com/products/chute-release-5x/>;
  [J10] <https://jollylogic.com/products/chute-release-10x/>.
