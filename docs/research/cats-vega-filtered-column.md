# The CATS Vega's `filtered_altitude_AGL`, and each flight's apogee from its own pressure

This note settles what the `filtered_altitude_AGL` column in RocketPy's EuRoC 2023 tables is, and reads each
flight's apogee from the Vega's logged pressure several ways. It serves the
[real-flight comparisons](../accuracy.md#real-flights), where Genesis's and Lince's logs are scored on that column
([report](../../validation/reports/real-flights.md)), and issue [#603]. It builds on
[cats-vega-height](cats-vega-height.md), which decoded three `.cfl` logs and found the column to be a 9-sample
running median of the maker's formula, and on [sample-logs-cats-juno](sample-logs-cats-juno.md) §2 (the table
peaks against the Vega's own). Sources are the eleven EuRoC 2023 `.cfl` logs, CATS's own log parser run as an
outside oracle (its code not read), the maker's manual and release notes. **Inference** marks this note's
reasoning and arithmetic. Read 2026-10-19.

**In short.** The column is the barometer's 9-sample median, on all eleven published 2023 logs, by two readings.
- **CATS's own parser names it.** Run on the eleven logs, CATS's documented command-line parser [C2] writes the
  0x100 record as `filtered_data_info` (`filtered_altitude_AGL`, `filtered_acceleration`) and the Kalman state
  as `flight_info` (`height`, `velocity`). Its values equal this project's byte decode on every row, and RocketPy's
  four tables equal its processed CSV on every row (Halcyon 65,882, Genesis 17,933, Camões 14,346, Lince 44,454).
- **All eleven logs share one layout and one filter.** Each is firmware `3.0.1-euroc`; on each, the column is the
  9-sample median of the manual's formula to 0.002 m (§1). Halcyon's fitted pad altitude matches its stats file's.
- **The table-over-stats gaps are filter noise and post-event transients.** Camões's +0.04% is the median's peak
  1.11 m over the Kalman's, both before the apogee event. Halcyon's +0.11% is mostly a transient: up to the
  apogee event, around a pressure jolt 0.37 s ahead of it, the two peaks differ by 1.12 m; after it the median
  reaches 3,470.83 m (§2).
- **A noise-free barometric apogee sits on the Kalman peak, not the median's.** A parabola through the raw pressure
  heights puts the four apogees 0.9 m to 3.2 m below the median's peak and within 0.6 m of the Kalman's (§3).
  Lince's column, cut at the apogee event, gives 3,587.60 m; the report's 3,587.72 m is 0.03 s past it.
- **What moves: almost nothing.** Every reading from pressure moves Genesis's error by −0.10 to +0.06 points and
  Lince's by −0.06 to +0.09; the 6.04% mean stays 6.04%. Only the stats file's 3,608.63 m, a transient, moves
  Lince by 0.6 points (§4).

## 1. The column by oracle and by fit

**The oracle.** The manual documents a command-line parser, `log_visualizer.py` in `catsystems/cats-logs`, that
"provides the Configurator's plotting functionality" and writes raw and processed CSV files [C1] "Using the Python
Plotting Tool". Its commit log says it was updated to parse EuRoC 2023 flights (2023-11-22) [C2]. It is GPL-3.0:
this note ran it as an outside tool, `python -m catslogs.log_visualizer -i <log> -o <dir>`, and read only its
output. Its pinned requirements don't install on current Python, so it ran on current plotly, numpy and pandas;
it wrote every CSV, then stopped at its HTML step. Its processed files put zero at liftoff, as RocketPy's do.

**What it says.** For each record type of [cats-vega-height](cats-vega-height.md) §2 the parser's names match the
byte fits: 0x20 `baro` (`T`, `P`), 0x40 `flight_info` (`height`, `velocity`, `acceleration`), 0x100
`filtered_data_info`, 0x200 `flight_states`, 0x400 `event_info` (state 5's event `EV_APOGEE`). It names the two left
open there: 0x80 `orientation_info` (a quaternion, raw norm 10,000) and 0x2000 `voltage_info` (millivolts) [C2].

**The eleven logs.** [D1] holds 11 `.cfl` files (8 main, 3 payload, listed 2026-10-19). Each is read below by the
byte decode, the oracle and a fit of the 0x100 height as a running median of the manual's formula (101,250 Pa,
exponent 1/5.257) less one constant, over every row from 1 s after liftoff. The last column is the gap
between the median's and the Kalman's peaks up to the apogee event:

| Log | Pressure rows | 0x100 rows | Oracle rows equal | Median-9 miss | Apogee event | Kalman below median |
|---|--:|--:|---|--:|--:|--:|
| `01_astg/main` (Halcyon) | 65,882 | 65,882 | all | 0.002 m | 28.07 s | 1.12 m |
| `05_asat/main` | 18,780 | 18,780 | all | 0.002 m | 22.13 s | 0.54 m |
| `05_asat/payload` | 24,205 | 24,205 | all | 0.002 m | 22.07 s | 1.76 m |
| `07_aesir/main_1` | 27,229 | 27,230 | all | 0.002 m | 25.16 s | 1.61 m |
| `07_aesir/main_2` | 27,228 | 27,229 | all | 0.002 m | 25.16 s | 0.96 m |
| `11_faraday/main` (Genesis) | 17,932 | 17,933 | all | 0.002 m | 24.37 s | 1.98 m |
| `12_bristol/main` | 27,400 | 27,400 | all | 0.002 m | 26.29 s | 3.35 m |
| `12_bristol/payload` | 2,588 | 2,588 | all | 0.002 m | none | n/a |
| `20_red/main` (Camões) | 14,345 | 14,346 | all | 0.002 m | 24.37 s | 1.11 m |
| `23_star/main` (Lince) | 44,454 | 44,454 | all | 0.002 m | 26.05 s | 3.13 m |
| `23_star/payload` | 36,131 | 36,130 | all | 0.002 m | 26.09 s | 3.34 m |

- **Every log names firmware `3.0.1-euroc`**, the release CATS titled "EuRoC 2023" (2023-10-09) [C4], most likely
  the version EuRoC mandated (**Inference**; the guide names none). One window fits all eleven: 9 samples ending one
  before the newest. Bristol's payload log ends 25.1 s after liftoff, still coasting.
- **The stats files carry the Kalman's overall peak.** On all 8 logs with a `*_stats.txt`, its maximum height is
  the highest 0x40 height. On 4 of the 8 it comes after the apogee event, 0.22 m to 42.1 m above the Kalman's
  peak before it (a fifth by 0.01 m). **Inference:** post-ejection transients; a stats peak is not an apogee.
- **The firmware's notes don't touch it.** The Vega's release notes from 2.0.0 (2021) to 3.0.2 (current) mention
  no change to the barometer's filtering or the log's height [C4]; the `3.0.1-euroc` release added only a firmware
  version check. The CATS site's changelog covers its flight-log website and online manual only [C5]. A 2025
  Configurator export already showed the same median ([cats-vega-height](cats-vega-height.md) §2).

**The Configurator itself was not run.** Its releases include macOS builds for both chip families [C3] (20
releases listed 2026-10-19, from 0.1.1 in 2022 to 2.0.3 on 2026-10-05). Its CSV export runs from a GUI page, a
file picker then an export folder [C1] "Flight Logs", [C3] 0.3.10; no command-line mode is documented. The macOS
builds are not notarized, so opening one needs a Privacy & Security exception [C3] 2.0.0, a system setting this
note did not change. The parser above stands in for it; the manual presents it as the same plotting.

## 2. Halcyon's and Camões's gaps

**Camões (+0.04%).** The table's peak, 3,016.00 m at 23.78 s after liftoff, and the stats file's Kalman peak,
3,014.89 m at 24.11 s, both come before the apogee event at 24.37 s: the 1.11 m is the median against the Kalman.

**Halcyon (+0.11%).** Halcyon (team 1, `01_astg`) flew a hybrid motor (its notebook builds a `HybridMotor` [R1]),
outside the project's scope before 1.0, so it is not in the report. Its fitted pad altitude, 112.1947 m, matches
its stats file's 112.1946 m. Its log shows an event the others don't:
- **A jolt before the Vega's apogee event.** At 27.70 s after liftoff the pressure height drops 30.6 m between two
  samples and about 135 m within 0.1 s, with a large IMU reading; the Kalman velocity turns from +1.7 m/s to
  −39 m/s and the Vega declares apogee at 28.07 s. On Genesis, Camões and Lince the first such drop comes 0.03 s,
  1.80 s and 0.7 s after their events. **Inference:** a separation or another system's charge; the log can't tell.
- **Up to the event** the median peaks at 3,467.84 m (27.68 s, before the jolt) and the Kalman at 3,466.72 m
  (27.74 s, its lag carrying it 0.04 s into the jolt): 1.12 m, 0.03%, as on the sister flights.
- **After it** the pressure height recovers to above its earlier peak while the filter, no longer using the IMU,
  re-converges: the median reaches 3,470.83 m at 28.83 s and the Kalman 3,466.94 m at 29.81 s (the stats peak).
  Of the +3.88 m gap, 2.98 m is this recovery (**Inference**).
- **The official 3,450 m matches no Vega reading.** The flight card's value [R1] is 16.1 m to 20.8 m below every
  reading in §3; both ground-station logs (5,810 and 6,170 rows) peak at 3,466 m [D1].

## 3. Each flight's apogee from its pressure

Readings up to each apogee event, from the log's own pressure. "Vega's own" is the manual's formula less the
Vega's pad altitude; "1976, pad" is [HPR Sim's rule](../../crates/hpr-validate/src/real_flight.rs), the standard
atmosphere's pressure altitude less the pad's, the pad's pressure the mean of the 75 or 76 samples logged in the
0.75 s before liftoff. The parabola is a least-squares fit to every 1976, pad height in the 3 s and 4 s before the
event (on Halcyon, ending at 27.65 s, before the jolt): it averages the noise out. Times in parentheses: from the event.

| Reading (m) | Genesis | Lince | Camões | Halcyon |
|---|--:|--:|--:|--:|
| One sample, Vega's own | 2,919.28 (−0.20 s) | 3,589.17 (−0.06 s) | 3,019.29 (−0.14 s) | 3,469.75 (−0.46 s) |
| Median-9: the column | 2,916.69 (−0.19 s) | 3,587.60 (−0.02 s) | 3,016.00 (−0.59 s) | 3,467.84 (−0.39 s) |
| Kalman (0x40) | 2,914.71 (−0.11 s) | 3,584.48 (0.00 s) | 3,014.89 (−0.26 s) | 3,466.72 (−0.33 s) |
| One sample, 1976, pad | 2,919.86 | 3,589.87 | 3,019.52 | 3,469.42 |
| Median-9, 1976, pad | 2,917.27 | 3,588.31 | 3,016.23 | 3,467.51 |
| Parabola, 1976, pad (3 s to 4 s) | 2,915.8 | 3,585.1 to 3,585.3 | 3,015.4 | 3,466.1 |
| Highest after the event: column, Kalman | none higher | 3,668.52, 3,608.63 | none higher | 3,470.83, 3,466.94 |
| Official (flight card) [R1] | 2,916 | 3,587 | 3,015 | 3,450 |

- **What the officials match.** Genesis's 2,916 m and Lince's 3,587 m are the column cut to whole meters; Camões's
  3,015 m is the Kalman peak rounded; Halcyon's matches nothing. The cards follow no one reading (**Inference**).
- **Noise lifts the maxima.** The single-sample peak sits 3.3 m to 4.8 m above the parabola, and the median's
  0.9 m to 3.2 m (like with like, 1976, pad). Each is the highest of a noisy series: the residual about the parabola
  is 1.7 m to 1.8 m (one standard deviation) on all four. The Kalman peak, moved into the same frame (by the median
  rows' 1976 less Vega offset, −0.34 m to +0.70 m), sits within 0.6 m of the parabola on all four (**Inference**).
  This answers [cats-vega-height](cats-vega-height.md)'s open question: the median's lead is its noise, not lag.
- **The fit's window.** A 2 s window puts the vertex past the window's end on Lince and Halcyon (3,587.5 m, 3,467.3 m),
  so only 3 s and 4 s are quoted; on Genesis and Camões all three agree within 0.1 m. Halcyon's 3 s and 4 s vertices
  also fall 0.1 s past its end; ending it at 27.60 s to 27.69 s gives 3,465.8 m to 3,466.4 m (**Inference**).
- **The pad.** The first pre-liftoff sample instead of the mean moves the 1976 reading by up to −1.95 m (Genesis),
  any one by −8.1 m to +11.5 m (noise 19 Pa to 32 Pa, one standard deviation). The Vega's own pad altitude sits
  −0.55 m (Halcyon) to +0.48 m (Lince) from the formula's height of that mean; on all 10 logs with an apogee,
  −4.98 m to +0.52 m.
- **Lince cut at the event.** The report's 3,587.72 m, read to 26.80 s, is the row at 26.08 s, 0.03 s after the
  event; up to the event the column peaks at 3,587.60 m (26.03 s). The stats peak comes 3.71 s after it.

## 4. What it moves in the real-flight comparisons

HPR Sim scores a log's highest row up to `until_s` against its own apogee read the 1976, pad way from noise-free
ERA5 pressure. With the report's hpr apogees (2,746.0 m, 3,709.2 m) and other five rows (**Inference**):

| Genesis log, Lince log | Genesis error | Lince error | Mean of 7 |
|---|--:|--:|--:|
| The column, as now (or cut at Lince's event, 3,587.60 m): 2,916.69 m, 3,587.72 m | −5.85% | +3.39% | 6.04% |
| Highest single sample, 1976, pad: 2,919.86 m, 3,589.87 m | −5.95% | +3.32% | 6.04% |
| 9-sample median, 1976, pad: 2,917.27 m, 3,588.31 m | −5.87% | +3.37% | 6.04% |
| Parabola, 1976, pad: 2,915.8 m, 3,585.1 m to 3,585.3 m | −5.82% | +3.46% | 6.04% |
| Kalman before the event: 2,914.71 m, 3,584.48 m | −5.79% | +3.48% | 6.04% |
| Lince's stats peak: 3,608.63 m | −5.85% | +2.79% | 5.95% |

- **Sign and size.** Pressure readings move Genesis's error −0.10 to +0.06 points, Lince's −0.06 to +0.09, the mean
  under 0.01; the noise-free parabola shrinks Genesis's miss 0.03 points and grows Lince's 0.07 to 0.08.
- **For [#603].** Its third *done when* (what `filtered_altitude_AGL` is) is now found, not inferred: the barometer's
  9-sample median, by CATS's own parser's names and by fit on 11 logs. Its second (which Lince reading is scored)
  has its numbers: the column 3,587.72 m, cut at the event 3,587.60 m, noise-free 3,585.1 m to 3,585.3 m, the stats
  3,608.63 m a transient 3.71 s after the event, +0.58% above the scored value.
- **For Cavour ([#818]).** Its radio log is the Kalman height, within 0.6 m of a noise-free barometric apogee on four
  sister flights, so its coasting 2,785 m to 2,786 m (truncated) is within about 1.3 m of its own (adding up to 0.7 m
  frame offset; **Inference**); [cats-vega-height](cats-vega-height.md) §4's 2,786 m to 2,789 m band was the median's.

## What this suggests

- **Record the column as found.** Genesis's and Lince's evidence can cite the parser's name (`filtered_data_info`
  in `catsystems/cats-logs`) beside the median fit. Expected effect: none on numbers; [#603]'s last item closes.
- **Cut Lince at its event, 26.05 s,** if the cut should mean "the Vega's apogee event" rather than "before the
  swing". Expected effect: −0.12 m, +0.003 points.
- **Score noise-free readings, later.** A `.cfl` reader (clean-room, from the record table here, checked against
  the cached oracle CSVs) could read each log's pressure and fit its apogee. Expected effect on today's two:
  Genesis −5.85% to −5.82%, Lince +3.39% to +3.46%, the mean unchanged; it matters more on noisier logs.
- **Don't score a stats file's peak.** In 4 of 8 it is a post-event transient, up to 42.1 m high.

## Open questions

- What jolted Halcyon at 27.70 s, and where its official 3,450 m comes from (neither is public).
- Whether a Configurator export (a GUI app) names columns as the parser does; [cats-vega-height](cats-vega-height.md)
  §3's 2025 export, `filteredAltitudeAGL`, suggests so.
- Whether the parabola's 3 s to 4 s window is right for slower coasts; a GNSS height would check it.

## Sources

CATS's site and release notes state no license of their own (the manual's source is in GPL-3.0 `cats-embedded`):
cited, cached under the ignored `refs/research/cats-vega-*/` only. The EuRoC 2023 data and both CATS tools are
GPL-3.0: the data are cached, never committed; the parser was run, its code not read; only numbers computed from them
are published. RocketPy is MIT. All read 2026-10-19. The decode and fits are scripts in the cache.
- [C1] CATS User Manual, last updated 2026-09-18: "Flight Logs", "Visualizing the Flight Data", "Using the Python
  Plotting Tool". <https://catsystems.io/manual>
- [C2] catsystems/cats-logs @ `0dc87a47`, GPL-3.0: run as an outside tool on all 11 logs; commit `a82986e2`
  (2023-11-22) message. <https://github.com/catsystems/cats-logs>
- [C3] CATS Configurator releases, GPL-3.0: notes of 0.3.10 and 2.0.0, assets of 2.0.3.
  <https://github.com/catsystems/cats-configurator/releases>
- [C4] CATS firmware releases, notes of v2.0.0 to 2026.10.1 (10 releases listed 2026-10-19).
  <https://github.com/catsystems/cats-embedded/releases>
- [C5] CATS Flights changelog, releases 01 to 09 (2026-07-30 to 2026-10-16). <https://catsystems.io/changelog>
- [D1] catsystems/euroc23-team-data @ `2dca527a`, GPL-3.0: all 11 `.cfl`, all 8 `*_stats.txt`, `01_astg/`
  `GS_log_main_1.csv` and `GS_log_main_1_1.csv`. <https://github.com/catsystems/euroc23-team-data>
- [R1] RocketPy (MIT) @ `9bd6ad3a`: `docs/examples/{halcyon,genesis,camoes,lince}_flight_sim.ipynb` (first cell:
  team, official apogee), `data/rockets/{astg,genesis,camoes,lince}/`. <https://github.com/RocketPy-Team/RocketPy>

[#603]: https://github.com/nrdptel/fusionspace-eridanus/issues/603
[#818]: https://github.com/nrdptel/fusionspace-eridanus/issues/818
