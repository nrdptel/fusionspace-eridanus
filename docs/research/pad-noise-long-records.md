# Pad against rest noise from long public pad records

Is a barometric altimeter noisier on the pad or at rest after landing, by how much, and per logger family? This
note answers it from the long pad records that public logs do hold: AltOS radio telemetry captured on the ground
(`.telem`), a team-built board that logs from power-on, and one that logs a few seconds of pad. It builds on
[public-logs-evidence-gaps](public-logs-evidence-gaps.md) §5, which found no public file with 3 s of quiet pad,
and on [altimeter-sensor-noise](altimeter-sensor-noise.md) §3; it uses that §5's method (cited as *gaps* §5). Sources
are repositories pinned by commit, Altus Metrum's documents, makers' manuals and airport weather, read 2026-10-28;
no GPL code was opened. **Inference** marks this note's reasoning.

**In short.**

- **A rest is about as noisy as a pad on short windows.** On four boards with both, the rest's median σ over
  3 to 15 s windows is 0.83 to 1.04 times the pad's (AltOS twice, team-built MS5607 and LPS-class boards once
  each); three of the four are unlicensed. Over 30 s or more, slow wander makes either noisier, sign unsettled.
- **Long pads exist for AltOS boards, but only at 1 Hz.** 12 public `.telem` files in 6 repositories hold 15 to
  962 s of contiguous pad. Median σ over 15-sample windows is 0.43 to 0.84 m (4.2 to 10.1 Pa) on ten
  TeleMega and TeleMetrum records, and 0.23 m on a TeleMini v3. At 1 s spacing the noise is close to white.
- **No long pad record is public for PerfectFlite, Blue Raven, RRC3 or Eggtimer.** Their files start at or just
  before launch detection (*gaps* §5; PerfectFlite's manual states 1.4 s).
- **Pads drift but stay stationary in noise.** Over 11 to 65 min, the minute means span 1.9 to 2.7 m in three
  records and about 10 m in one ramp of unknown cause. Board temperature rose 0.2 to 12 K, while σ moved −25% to
  +7% from early to late, with no consistent sign. The Allan deviation is least, 1 to 4 Pa, at 10 to 60 s.
- **What it moves.** #725 can take its pad segment from AltOS telemetry and the team boards; for the other
  families only rests exist, and how far their rests stand for their pads is not bounded. The synthetic `pad_s`
  draw should follow each logger's file cut. A pad longer than about 30 s also needs a drift term.

## 1. Where long pad records are

- **AltOS telemetry, 19 files counted.** GitHub code search for `.telem` files (2026-10-28) returned 17 hits:
  15 AltOS files in 8 repositories, the project's own sample and a racing car log. Four more, too large for the
  index, sit in two listed repositories' trees.
- **What the 19 hold.** The files were decoded from the published packet layouts alone [A1]. 12 carry pressure
  in pad state: TeleMega packets `0x08` and `0x14`, TeleMetrum v2 `0x0A` and TeleMini v3 `0x11`. Five hold only
  configuration and GPS packets, one holds 7 packets at one instant, and one has no flight state.
- **The radio sends ground data once per second.** Sensor packets go out once a second on the ground, ten times
  a second in ascent [A1] pp. 3–5, so a pad record is a 1 Hz series. Pad pressures read as pascals (means of
  73,438 to 101,279), not the Pa × 10 the document's tables state; 1 Pa is the step.
- **Pad state means an upright board, not the rail.** Boards with an accelerometer enter pad mode only if they
  point up during the power-on self test [A2] pp. 8–9. One record (UNHSEDS) holds pad packets 3.3 h earlier and
  20.9 km from the launch, about 44 m lower by pressure; only its last contiguous run is used.
- **Team boards, 2 found.** Projeto Jupiter's Juno III board [R8] logs from power-on: LPS-class, about 3 Hz,
  65 min of pad and 5.6 h after landing. An airbrake team's MS5607 board [R7] logs at 100 Hz, but its file starts
  4.1 h after boot, with 9.7 s before launch (9.4 s of quiet pad) and 34 s at rest. Column-name searches for BMP,
  MS56xx, LPS and DPS sensors found mostly parts lists; that search was not exhaustive.

## 2. AltOS pads at the telemetry rate

Each record's pad is split into runs with no gap over 10 s. Each run is cut into non-overlapping blocks of 15
samples, and each block loses a least-squares line. σ is the blocks' median [10th–90th percentile], lag-1 the
blocks' median; for white noise this median reads about 6% below the true σ. Height comes from pressure by the
1976 standard atmosphere. The Allan deviation uses non-overlapping bins; values at 60 s on runs under 200 s
rest on two or three intervals.

| Record [ref] | Packet | Longest run (s) | Blocks | σ (m) | σ (Pa) | Lag-1 | Allan least (Pa, at τ) |
|---|---|---:|---:|---|---:|---:|---|
| KarmanSims NH_F01 [R2] | `0x08` | 763 | 47 | 0.695 [0.464–1.014] | 8.3 | −0.19 | 2.90 at 30 s |
| KarmanSims PTR_F03 [R2] | `0x08` | 962 | 61 | 0.608 [0.412–0.809] | 7.3 | −0.07 | 2.11 at 30 s |
| KarmanSims, Dec. [R2] | `0x08` | 674 | 72 | 0.631 [0.474–0.848] | 7.6 | −0.18 | 2.13 at 30 s |
| UNHSEDS, last run [R3] | `0x08` | 303 | 19 | 0.842 [0.62–1.08] | 10.1 | −0.11 | 2.75 at 10 s |
| HART_CS_2020 [R4] | `0x08` | 671 | 35 | 0.692 [0.554–0.921] | 8.3 | −0.11 | 3.91 at 10 s |
| UGA-IREC [R6] | `0x08` | 172 | 10 | 0.538 [0.463–0.628] | 6.5 | −0.12 | 1.01 at 60 s |
| JEFF, flight 5 [R5] | `0x14` | 905 | 58 | 0.462 [0.326–0.597] | 4.3 | −0.14 | 1.49 at 30 s |
| JEFF, flight 6 [R5] | `0x14` | 208 | 13 | 0.615 [0.494–0.842] | 5.7 | −0.18 | 2.91 at 30 s |
| JEFF, 2nd board [R5] | `0x14` | 132 | 19 | 0.432 [0.363–0.504] | 4.2 | −0.09 | 1.16 at 60 s |
| JEFF, TeleMetrum [R5] | `0x0A` | 111 | 6 | 0.747 [0.717–0.819] | 7.2 | −0.34 | 3.56 at 30 s |
| CosmoSoft [R1] | `0x11` | 97 | 4 | 0.232 [0.215–0.250] | 2.8 | 0.09 | 1.65 at 10 s |

- **Close to white at 1 s.** Block lag-1 medians run −0.34 to +0.09. White noise in 15-sample blocks with a
  line removed gives a median of −0.14 (20,000 simulated blocks), so these fit white noise (**Inference**).
  Whole runs read −0.03 to +0.42. From 1 s to 10 s the Allan deviation falls 1.4 to 3.0 times (2.4 to
  10.3 Pa at 1 s), against 3.2 for white noise: a slower term joins within seconds on some boards.
- **Single samples, not 1 s means.** These σ match the 0.52–1.14 m of AltOS 100 Hz pad windows
  ([altimeter-sensor-noise](altimeter-sensor-noise.md) §3; *gaps* §5's shorter windows give 0.17–1.99 m). A
  mean of 100 white samples would be ten times quieter, so each packet carries one reading (**Inference**).
- **Board to board, not by family.** One TeleMega (`0x14`) read 4.3 and 5.7 Pa on two pads of one day, another
  (`0x08`, serial 3442) 8.3 and 7.3 Pa; the `0x08` boards span 6.5 to 10.1 Pa. Oversampling is not documented
  [A2], so the spread can't be assigned. All but UNHSEDS (0.84 m) sit at or below the synthetic `altos` 0.78–0.82 m.
- **The 1 s spread can't be measured at 1 Hz:** a 1 s running mean of a 1 Hz series is the series itself.

## 3. Pad against rest on the same board

| Board, family [ref] | Rate | Pad: windows; σ (m); 1 s | Rest: windows; σ (m); 1 s | Rest/pad σ |
|---|---|---|---|---:|
| TeleMega, AltOS [R2] | 1 Hz | 47 × 15 samples; 0.695 | 13 × 15 samples; 0.576 | 0.83 |
| TeleMega, AltOS [R5] | 1 Hz pad; log rest | 58 × 15 samples; 0.462 | 1 × 17.6 s; 0.477; 0.14 | 1.03 |
| Team board, LPS-class [R8] | 3 Hz | 788 × 5 s; 0.589; 0.275 | 729 × 5 s; 0.593; 0.280 | 1.01 |
| Team board, MS5607 [R7] | 25 Hz updates | 3 × 3 s; 0.216; 0.029 | 11 × 3 s; 0.214; 0.040 | 0.99 |

- **The ratio is 0.83 to 1.04 on 4 boards** (a second count gives 1.04 and 1.02 for rows 2 and 4). Rests begin
  2 s after the last sample over 3 m from the tail's median, as in *gaps* §5; on the Juno III board, the first
  hour from 163 s after launch. Row 2's rest is the onboard log. The 1 s spread's ratio is 1.02 on the LPS-class
  board and 1.38 on the MS5607 board, the second on 3 pad windows.
- **Over longer windows the sign varies:** about 1.6 on the MS5607 board (34 s against 9.4 s), 0.94 on the
  LPS-class board (last 600 s of pad against first 600 s of rest; 1 s spread 0.85). Beyond 15 s the cause and
  the sign are unbounded.
- **The MS5607 board holds each reading for four to five rows.** 78% of consecutive logged rows are equal, and
  a new value comes every 40 ms (median). On the logged rows lag-1 is 0.79; on the update points, 0.05 to 0.11.
  This is a hold like the Blue Raven's in *gaps* §5, not sensor correlation.
- **The LPS-class board's Allan curves match:** 3.79 and 3.88 Pa at 1 s, 1.42 and 1.32 Pa at 30 s, 3.18 and
  4.01 Pa at 300 s (pad, rest). Its temperature read 27.0 to 39.0 °C over the pad and 36.3 to 40.3 °C in the rest.

## 4. What pollutes a pad window

| Cause | Sign | Size measured | Basis |
|---|---|---|---|
| Isolated outliers, cause not shown | spikes | 4 to 8 beyond 4 robust σ in 3,100 samples | 10 AltOS pads |
| Moving between sites in pad state | a step | 44 m, 3.3 h apart | UNHSEDS GPS fixes |
| Board warm-up after power-on | level drift; σ unchanged | +0.2 to +5.3 K (AltOS), +12 K (Juno III) | §2, §3 |
| Weather | level drift | 0 to 1.2 m over 13 min; 1.0 to 1.5 m over 11 min | ASOS [W1], [W2] |
| One-way ramp, cause unknown | level drift; σ unchanged | about 10 m over 11 min on one record | unbounded |

- **Warm-up does not raise σ consistently.** First against last 180 s: 0.97 against 0.73 m, 0.52 against
  0.46 m, 0.80 against 0.86 m on three long AltOS pads; 0.565 against 0.603 m (block medians) on the LPS-class
  board's first and last 600 s.
- **The drift is a level, removed by a line on short windows.** Minute means span 1.9 m over 13 min
  (KarmanSims), 2.1 m over 15 min (JEFF flight 5) and 2.7 m over 65 min (Juno III, net +2.1 m). Portsmouth
  airport's altimeter setting moved one 0.34 hPa step over the KarmanSims pad: 0 to 1.2 m of weather.
- **One record ramps.** The HART_CS_2020 board sat still (GPS minute means within 11 m) as its height rose 9.7 to
  10.0 m in 11 min, every minute higher; its block σ (0.69 m) is ordinary. Corvallis airport, 8.5 km away, logged
  pressure falling 0.7 to 1.0 hPa/h (1.0 to 1.5 m of it), wind 18 to 20 kt gusting 25 to 27 kt [W1]. A smooth
  one-way rise does not look like gusts; the rest is unexplained. Its Allan deviation reaches 37.4 Pa at 300 s.
- **Makers warn of wind and light.** Featherweight asks for several small vents so wind at the pad disturbs the
  reading less [M2] p. 7, and warns of light on the sensor during motion [M2] p. 8. Missile Works recommends
  several ports against wind [M3] p. 4; PerfectFlite's 160 ft launch-detect default resists wind gusts [M1]
  p. 25. None states a size.

## 5. Families with no long pad record

- **PerfectFlite.** The StratoLoggerCF stores 28 samples (1.4 s) before launch detection [M1] p. 25. Its
  "OnPad" serial telemetry sends altitude from the continuity beeps onward [M1] p. 27; no public capture found.
- **Blue Raven.** Low-rate files start 1.90 to 1.92 s before liftoff (2 flights, *gaps* §5). Its flight simulation
  records while on the ground [M2] p. 17; whether its baro column then holds real readings is not stated.
- **RRC3 and Eggtimer.** Files start at launch detection: 0.45 s or less before motion for RRC3, none for
  Eggtimer. Rests (*gaps* §5): PerfectFlite 17 (0.13–0.25 m at 20 Hz), Blue Raven 1, RRC3 and Eggtimer none.

## 6. What pad length a synthetic draw should use

- **Follow the file cut for files that mimic a logger.** Real files hold 0.30 s of pad for AltOS logs, 0.14 to
  0.62 s for EasyMini, about 1.9 s for the Blue Raven, 0.45 s or less for PerfectFlite and RRC3, and none for
  Eggtimer (*gaps* §5). Today's 2 to 10 s gives an analyzer a better pad zero than any real file allows.
- **Long pads are a separate kind of record:** minutes at 1 Hz from AltOS telemetry, an hour at 3 Hz from Juno
  III. With the Allan floor of 1 to 4 Pa at 10 to 60 s, a pad over about 30 s needs drift (**Inference**).
- **Size the pad zero by the window.** N white samples average to σ/√N: 0.3 s at 100 Hz of 0.6 m gives 0.11 m,
  0.45 s at 20 Hz of 0.18 m 0.06 m. The Allan floor, 0.1 to 0.35 m, bounds any pad average (**Inference**).

## What this suggests

1. **Word #725's pad segment by source.** AltOS: measure pads at the telemetry rate from §2's records (σ and
   lag-1 only, no 1 s spread). Team boards: §3's two boards. PerfectFlite, Blue Raven, RRC3, Eggtimer: use rests,
   citing the 0.83–1.04 found on 4 boards of other families, not as a bound. Effect: a pad row for 2 families.
2. **Draw `pad_s` per profile from each logger's file cut** (0.30 s AltOS, 1.9 s Blue Raven, under 0.45 s
   PerfectFlite and RRC3, none for Eggtimer), as public-logs-evidence-gaps proposed. Add a "logs from power-on"
   case only with a drift term (Allan floor 1 to 4 Pa at 10 to 60 s; a 10 m ramp in 11 min seen once).
   Expected effect: the pad zero is tested at the precision real files allow (0.06 to 0.11 m, **Inference**).
3. **Keep the synthetic `altos` σ (0.78–0.82 m):** it sits near the top of the 0.23–0.84 m block medians (one
   of 11 records is higher, by 3%), the conservative side for a smoother test.

## Open questions

- Does any family's rest differ from its pad beyond 15 s, where wind and sun act? Two boards disagree in sign.
- Does a Blue Raven in simulation, or a StratoLoggerCF in OnPad telemetry, record real pad pressure? Either
  would give those families a pad. What oversampling does each AltOS board use (4.2 to 10.1 Pa between boards)?
- No license-clear AltOS record holds both a long pad and a rest; the one pair with clear terms is a team board.

## Sources

Licenses are from GitHub's API. Unlicensed data is cached under `refs/research/pad-noise-long-records/` and cited
here as statistics only; MIT data may be cached; neither is committed. Repositories read 2026-10-28.

- [R1] UCC-Rocket-and-Space-Exploration/CosmoSoft @`72bd620a65ae`, MIT, `sample_data/` (4 `.telem`).
  <https://github.com/UCC-Rocket-and-Space-Exploration/CosmoSoft>
- [R2] AIAANortheastern/KarmanSims @`0d4e22e783be`, no license, `Flight Data - Recorded/` (`10-28-17_NH_PTR/`,
  `12-17-17-InB/`).
  <https://github.com/AIAANortheastern/KarmanSims>
- [R3] Maeswoon/UNHSEDS @`3374f721ba07`, no license, `Archive/Engineering/2017-2018/USRC/`.
  <https://github.com/Maeswoon/UNHSEDS>
- [R4] lunng/HART_CS_2020 @`ad0843c8c9c9`, MIT, `Data/` (pad only). <https://github.com/lunng/HART_CS_2020>
- [R5] Mines-Rocketry-Club/JEFF-modular-rocket-system @`9459442b63a1`, no license, `Avionics/All the flight data/`
  (4 `.telem`, flight 5's AltosUI CSV export). <https://github.com/Mines-Rocketry-Club/JEFF-modular-rocket-system>
- [R6] parsa-novin/UGA-IREC @`ebcda9369dac`, no license, `Legacy Code/` (3 `.telem`).
  <https://github.com/parsa-novin/UGA-IREC>
- [R7] osu-bsli/2026-Airbrakes-Code @`a83aef7b400a`, no license, `data/novflight.csv`.
  <https://github.com/osu-bsli/2026-Airbrakes-Code>
- [R8] RocketPy-Team/RocketPy @`9bd6ad3af8f9`, MIT, `data/rockets/juno3/` (srad_telemetry.csv, README).
  <https://github.com/RocketPy-Team/RocketPy>
- GPS-only `.telem`: robderstadt/Rockets @`7acb50496cdc`, osu-bsli/irec @`ce38b644e580`, both no license; one
  more in [R6].
- [A1] Altus Metrum, *AltOS Telemetry* (built 2025-11-17), pp. 2–6, 10. Creative Commons ShareAlike 3.0.
  <https://altusmetrum.org/AltOS/doc/telemetry.pdf>
- [A2] Altus Metrum, *The Altus Metrum System*, v1.9.22.2, 2025-10-13, pp. 8–9, 77. Creative Commons ShareAlike 3.0.
  <https://altusmetrum.org/AltOS/doc/altusmetrum.pdf>
- [M1] PerfectFlite, *StratoLoggerCF manual*, pp. 25, 27.
  <http://www.perfectflite.com/Downloads/StratoLoggerCF%20manual.pdf>
- [M2] Featherweight, *Blue Raven User's Manual* (2026), pp. 7, 8, 17. <https://www.featherweightaltimeters.com/>
- [M3] Missile Works, *RRC3+/RRC3 User Manual*, rev. 1.71, p. 4. <https://www.missileworks.com/downloads/>
- [W1] Iowa Environmental Mesonet, ASOS record for CVO (Corvallis), 2020-01-27 20:00–24:00 UTC.
  <https://mesonet.agron.iastate.edu/request/download.phtml>
- [W2] The same, PSM (Portsmouth), 2017-10-28 19:00–21:00 UTC, 27 km from the KarmanSims pad.
