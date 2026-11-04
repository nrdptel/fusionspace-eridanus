# When altimeters fire the drogue, timed from the log's own apogee

How long after the barometric peak in its own log does each common altimeter detect apogee and fire its drogue?
It serves the late-drogue fault (M7.4 in the [roadmap](../ROADMAP.md)) and the settings check's apogee-delay row
(M6.8), which today add only the typed delay. It builds on [altimeter deploy logic](altimeter-deploy-logic.md) §1,
which found most makers' latency unpublished, and on [early deployment bias](early-deployment-bias.md), which saw
altimeters fire "a fraction of a second early" on private flights. Read 2026-11-04: public logs already cached by
earlier notes, the makers' manuals, no firmware source. **Inference** marks this note's own timing and reasoning.
How far to trust it: the signs and medians for PerfectFlite, RRC3, Eggtimer, Raven and the Blue Raven rest on 24
to 93 passing flights each; RRC3, Eggtimer and Raven rest mostly or only on the private set's aggregates; AltOS rests
mostly on one team's flights; firmware versions could not be separated.

**In short.**
- **Baro-speed altimeters fire at or slightly before the peak.** StratoLogger SL100 and CF detect apogee 0.21 to
  0.29 s (median) before the fitted peak, on 90 of 93 flights; the RRC3 fires within ±0.06 s of it (IQR, 26
  private flights). PerfectFlite's "+/- 0.2 seconds" is about right in size (past 0.2 s early on 63 of 78 subsonic
  flights) but one-sided: the error is early, not centered.
- **Two makers add about 1 s, and logs show it.** Eggtimer's nose-over comes 1.00 s after its highest logged sample
  (median and IQR, 37 private and 8 public logs), 0.84 to 1.04 s after the fitted peak (IQR, median +0.88 s); the
  drogue fires at nose-over plus the typed delay. The Raven fires 0.86 to 0.93 s after the peak (IQR, 16 subsonic
  private flights), from its filter.
- **The Blue Raven fires at its apogee vote on most flights, later on a third.** The vote trails the peak by 0.30 s
  (public) and 0.47 s (private) at the median. The apogee output's voltage collapses 0.04 s after the vote on 14 of 23
  public logs; on 9 the charge waits 0.5 to 4 s more, and the log doesn't say whether a delay was typed. Firing
  medians: +0.85 s public (10 passing flights), +0.50 s private (24), subsonic tail to about +4 s. The log's
  `Apo_fired` turns true 1.5 s after the firing, as the manual defines it (**Inference**, §5).
- **Kalman boards fire near the peak:** AltOS baro-only boards +0.04 s median (25 flights), boards with a 3-axis IMU
  −0.02 s (19), both within about ±0.2 s on most flights. Most of these come from one team.
- **For M7.4 and M6.8:** add a per-device latency to the typed delay: about −0.25 s PerfectFlite, 0 RRC3 and AltOS,
  +0.9 s Eggtimer (to nose-over), +0.9 s Raven, +0.5 s Blue Raven with a tail to +4 s (§7). A late drogue is the
  flattering-side worry; each extra second adds about 10 m/s of descent before the charge.

## 1. What was timed, and how

- **Logs.** Every public log in the project's caches whose format records both baro altitude and an apogee or drogue
  event: 318 files in six readable formats found by header, 196 with an event and a usable trace, 181 after dropping
  15 duplicate copies (same device, peak within 0.5%, event within 0.15 s). The private collection adds 312 files not
  identical to a public one; it is reported only as aggregates, in its own column. The two sets may share a few
  flights in different copies (downsampled or trimmed).
- **The peak ("true apogee").** A quadratic least-squares fit to the raw baro altitude over 2 s around the
  highest 5-sample-median reading, ending at the first logged event, so no charge gas enters the fit. When the
  event comes before that reading, the fit uses the 2 s before the event and extrapolates. One pass drops samples
  more than 6 median absolute deviations plus 0.5 m off a 5-sample median (single-sample charge dips).
- **Quality rule.** A fit counts if its curvature lies within −3.0 to −7.5 m/s², near −g/2 as a coast through
  apogee should, and its vertex time has a standard error of 0.10 s or less. 124 of 181 public and 178 of 229
  private flights pass; Eggtimer's noisier baro fails most often (10 of 12 public, 15 of 38 private).
- **Is the extrapolation biased?** On the 6 public StratoLogger flights with a typed delay whose fits pass both ways,
  a fit centered on the peak moved it −0.07 to +0.20 s (median +0.04 s) from the extrapolated one (**Inference**).
- **Sampling.** Events are logged on the sample clock: 20 Hz (PerfectFlite, RRC3, most Eggtimer logs, Raven baro), 50 Hz
  (Blue Raven), up to 100 Hz (AltOS in ascent). So an event time is good to 0.01 to 0.05 s, and the fitted peak to
  about 0.1 s.
- **Mach.** A flight is put apart when its log's own speed reaches 340 m/s: the accelerometer speed for Blue Raven and
  Raven, AltOS's own Kalman `speed` column, the baro speed (0.5 s slope of the median altitude) for the rest. Baro
  speed is unreliable through Mach, so the baro-flagged rows mix real supersonic flights with pressure transients.

## 2. The results

Latency is the event time minus the fitted peak; negative means before apogee. Medians, with the IQR or range.

| Device, event | Public n | Public median (IQR) | Mach ≥ 1, n: median | Private n | Private median (IQR) |
|---|--:|---|---|--:|---|
| SL100, apogee detect | 9 | −0.22 s (−0.26 to −0.15) | none | 12 | −0.29 s (−0.30 to −0.24) |
| SLCF, apogee detect | 13 | −0.21 s (−0.27 to −0.15) | 5: −0.32 s (baro) | 44 | −0.25 s (−0.30 to −0.22) |
| RRC3, drogue event | 6 | −0.08 s (−0.16 to +0.72) | 2: −0.04, +1.90 s (baro) | 26 | +0.01 s (−0.05 to +0.06) |
| AltOS baro-only, drogue state | 25 | +0.04 s (−0.02 to +0.07) | 2: +0.07, +0.14 s | 0 | none |
| AltOS 3-axis IMU, drogue state | 19 | −0.02 s (−0.16 to +0.08) | 6: −0.17 s | 1 | too few |
| AltOS radio, no IMU, drogue state | 4 | −0.27 s (range −0.36 to +0.81) | 3: +0.01 s | 3 | too few |
| Eggtimer, drogue output | 2 | +1.16, +1.48 s | none | 21 | +1.01 s (+0.86 to +1.21) |
| Raven, apogee pyro drop | 1 | +2.35 s | 2: +2.15, +3.12 s | 16 | +0.89 s (+0.86 to +0.93) |
| Blue Raven, apogee vote | 11 | +0.30 s (+0.18 to +0.93) | 2: +0.21, +2.66 s | 24 | +0.47 s (+0.23 to +1.31) |
| Blue Raven, apogee firing | 10 | +0.85 s (+0.34 to +1.40) | not split | 24 | +0.50 s (+0.23 to +1.35) |
| Blue Jay, apogee vote | 12 | +0.16 s (+0.03 to +0.29) | none | 0 | none |

The private Raven, Mach ≥ 1: 12 flights, +0.89 s median (range +0.17 to +6.81 s). Private Blue Raven vote, Mach ≥ 1:
5, +1.36 s (range +0.72 to +2.68 s). A Blue Raven firing is `Apo_fired` minus 1.5 s (§5). Counts are flights passing the
quality rule; all numbers **Inference** from the logs.

## 3. PerfectFlite StratoLogger: early, by a quarter second

- **The header states the firing.** Each `.pf2` names its model, firmware, `Apogee Delay` and `Drogue At`; the drogue
  time minus the delay is the detection. Public firmware is 1.0 on every CF and 1.1 on 13 of 14 SL100 logs.
- **Detection precedes the peak on 22 of 22 public subsonic flights** (range −0.46 to −0.07 s; mean −0.22 s, 95% t
  interval −0.27 to −0.17 s, both models pooled) and on 54 of 56 private ones. At −0.25 s the rocket still climbs
  about 2.5 m/s (g·t), which [early deployment bias](early-deployment-bias.md) bounds at under 0.3% of apogee.
- **The maker's figure** is "typically within +/- 0.2 seconds" [1] PDF p. 4. The logs put it about right in size
  (past 0.2 s early on 13 of 22 public and 50 of 56 private subsonic flights) and show a sign: early, and late by
  more than 0.05 s on only 2 passing private fits (**Inference**). Typed delays of 0 to 3 s
  add on top; the header states them.
- **Above Mach 1 by baro speed** (5 public, 10 private) detection is as early or earlier (−0.68 to +0.07 s). The
  manual warns MachLock can hold off apogee "several seconds" after a vacuum-pump test [1] PDF pp. 40 to 41; no
  flight here fired late from it.
- **Sources:** 18 public CF flights from 10 sources (6 from one team's archive), 9 SL100 flights from 2.

## 4. RRC3, Eggtimer and Raven

- **RRC3: at the peak.** It fires at a "displacement velocity of 0" [2] PDF p. 10, a velocity from a "recursive 21
  element FIR filter" on 20 Hz data [2] PDF p. 24. The parent note noted a symmetric 21-tap FIR would lag about 0.5 s
  but did not treat it as a bound; the logs show no such lag: private drogue events sit −0.15 to +0.17 s from the peak
  (mean +0.00 s, 95% −0.03 to +0.04 s, 26 flights). Public primaries read −0.31 to −0.04 s; three units at +0.72 to
  +1.90 s are likely backups with a typed delay, which the export doesn't state.
- **Eggtimer: 1 s by design, plus nothing hidden.** Its guide puts nose-over "approximately one second after
  apogee" [3] PDF p. 20. In every log with markers, `Apogee` marks the highest raw `Alt` sample (14 of 14 public),
  `N-O` follows it by 1.00 s (median, 37 private and 8 public logs), and `Drogue` follows `N-O` by the typed delay
  (0 s on 29 of 35 private logs, 0.5 or 1.5 s on the rest). The marked sample itself sits −0.12 s (median, −0.16
  to +0.04 s) from the fitted peak, so the drogue fires about 0.9 s plus the delay after apogee (**Inference**).
  No filter lag beyond the 1 s shows. Files don't say which model (Quasar, Quantum, TRS, Proton) or firmware flew.
- **Raven: about 0.9 s late, from its filter.** The 2014 manual's default apogee output fires on "filtered baro
  pressure measurements" increasing, delay 0 [5] PDF pp. 5, 8. The apogee output's voltage drop coincides with the
  charge's axial jolt (median −0.01 s, IQR −0.05 to +0.01 s, 33 private flights), so it marks the firing; it falls
  0.82 to 1.10 s after the peak on 16 subsonic private flights (**Inference**: a filter lag, as no delay is typed by
  default). The 10 public Raven flights with a drop are one course's two airframes; on 4 of the 6 with a jolt, it
  precedes the drop by 1.05 to 1.45 s (some other charge), so they are left unread.

## 5. Blue Raven and Blue Jay: the column that trails the charge

- **The apogee vote.** The Blue Raven latches "Apogee Detected" when 2 of 3 agree: baro pressure increasing (a
  "strong low-pass filter"), total velocity below 0, gyro tilt past 90° [4] PDF p. 15. That flag lags the peak
  by +0.30 s (public) and +0.47 s (private) at the median, with a tail to about 3 s; above Mach 1 by the
  log's own accelerometer speed, +1.36 s (private, 5 flights).
- **When the charge fires.** `Apo_fired` turns true 1.54 s after the flag on 14 of 23 public Blue Raven logs, 15 of
  17 Blue Jay logs (the other 2 within 0.04 s) and 26 of 34 private ones at 1.52 to 1.54 s; the rest carry other
  delays. The manual defines that event as true "1.5 seconds after the apogee deployment channel first turns on"
  [4] PDF p. 15. The output's own voltage agrees: `Apo_Volts` collapses 1.50 to 1.52 s before `Apo_fired` in all 15
  public logs with a collapse (31 of 32 private), and recovers about 1.0 s later in 10 of 15 (27 of 32), as the
  manual's 1.0 s output time would [4] PDF p. 15 (**Inference**). So the firing is `Apo_fired` minus 1.5 s.
- **So the 1,500 ms default the parent note found** is the channel's delay option, and most flights don't wait it on
  the apogee output. On 9 of 23 public logs the charge fires 0.5 to 4 s after the vote (waits of 0.5, 1.0, 1.5, 3.0
  and 3.7 s seen, one exactly 1.5 s), which the log doesn't say were typed. Firing medians are in §2's table; the
  subsonic tail reaches about +4 s, and one supersonic private flight fired more than 10 s after its peak.
  Reading `Apo_fired` as the firing would place every Blue Raven drogue 1.5 s too late.
- **Blue Jay** (17 files of Southern Utah University student flights, Apache-2.0 [7]): flag +0.16 s median (IQR +0.03 to
  +0.29 s, 12 flights), all below 340 m/s.

## 6. AltOS, and the log's own raw peak

- **AltOS fires at the drogue state.** The manual fires the apogee charge the typed Apogee Delay "after apogee has
  been detected" [6] PDF p. 45; with an accelerometer it pegs apogee "within a few sensor samples" [6] PDF p. 72.
  The `drogue` state starts +0.04 s from the peak on baro-only boards (IQR −0.02 to +0.07 s, range −0.15 to +0.36 s)
  and −0.02 s on 3-axis boards (IQR −0.16 to +0.08 s). The drogue sense voltage then drops after the typed delay:
  +1.01 s median after the drogue state on 25 baro-only boards, most of them 1 s backups (15 fall 0.85 to 1.1 s
  after it; 6 within 0.15 s, with no delay) (**Inference**).
- **AltOS caveat.** Boards are classed by the CSV's columns, not named: 30 of the 52 baro-only and 3-axis flights are
  one team's. Radio boards without an IMU (TeleMetrum class) give 7 public and 3 private flights, too few to read.
- **Never take the log's raw maximum as apogee near a charge.** The highest 5-sample-median reading sits more than
  0.5 s after the fitted peak on 38 of 124 public and 48 of 178 private flights, from pressure noise after
  separation. Timed from it, an on-time charge would look early.

## 7. What the project can use

- **M7.4's late-drogue model** (suggestion): drogue time = fitted apogee + typed delay + device latency, drawn from:
  PerfectFlite −0.25 s (SD about 0.1 s); RRC3 0.00 s (SD 0.08 s); AltOS 0.0 s (±0.2 s); Eggtimer +0.9 s to nose-over
  (+0.7 to +1.6 s), then the typed delay; Raven +0.9 s (SD 0.07 s) subsonic, a tail to +6.8 s above Mach 1 (12 private
  flights); Blue Raven firing +0.5 s median with a subsonic tail to +4 s (empirical, not normal; it includes any typed
  wait the log doesn't state). Expected effect: Eggtimer, Raven and Blue Raven drogues open about 9 to 40 m/s faster
  than a typed-delay-only model says (g·t, drag ignored, an upper bound), the flattering-side gap M7.4 must not hide.
- **M6.8's apogee-delay row** (suggestion): compare descent speed at typed delay + latency, not the typed delay;
  keep Eggtimer's 1 s from the parent note (now measured), add Raven's 0.9 s, Blue Raven's 0.5 s and its tail.
- **M7.1's readers:** a Blue Raven firing is `Apo_fired` minus 1.5 s; an Eggtimer `Apogee` column is a retrospective
  mark, not a detection; a Raven's pyro-voltage drop marks its charge.
- **Data terms.** Logs stay in the uncommitted caches of the notes in Sources; this note copies no rows. Licensed
  sets: UND-ARC and werocketry (MIT), Shred-O-Vision (MIT), the Blue Jay set (Apache-2.0), AerospaceNU (GPL-3.0
  data, cited only); the rest state no license and are cited, not copied. The scripts sit in
  `refs/research/apogee-fire-latency-in-logs/scripts/` and could become a check against the M7.1 readers.

## Open questions

- Firmware: only PerfectFlite logs state it, and all public CF logs carry 1.0, so no firmware split was possible.
- Blue Raven: a 500 Hz high-rate log paired with its low-rate log would time the charge's jolt directly; pairing
  by the `Sync` counter alone proved unreliable here. Why does the vote trail the peak by 1 to 3 s on some flights,
  and were the 0.5 to 4 s waits on 9 public logs typed delays?
- Raven4 and current Raven firmware: the 0.9 s here fits the 2014 defaults; newer defaults weren't checked in logs.
- RRC3 and Eggtimer exports don't state the typed delay, so a backup's 0.7 to 1.9 s can't be split into delay and
  latency. More AltOS teams, and AltOS above Mach 1 beyond 11 flights, would firm up its rows.
- MachLock's "several seconds" ground-test delay never showed in flight; its thresholds stay unpublished.

## Sources

Read 2026-11-04. Makers' documents state no reuse license unless noted: cited, cached privately, not copied.
- [1] PerfectFlite, *StratoLoggerCF* manual, PDF pp. 4, 40 to 41:
  <http://www.perfectflite.com/Downloads/StratoLoggerCF%20manual.pdf>
- [2] Missile Works, *RRC3+/RRC3 User Manual* v1.71, PDF pp. 10, 24: <https://www.missileworks.com/downloads>
- [3] Eggtimer Rocketry, *Quantum User's Guide* 1.09G (2023), PDF p. 20:
  <https://eggtimerrocketry.com/wp-content/uploads/2024/02/Eggtimer-Quantum-1_09G.pdf>
- [4] Featherweight, *Blue Raven User's Manual*, 2026-05-12, PDF p. 15:
  <https://www.featherweightaltimeters.com/uploads/1/0/9/5/109510427/blue_raven_users_manual_2026may12.pdf>
- [5] Featherweight, *Raven User's Manual*, 2014-05-20, PDF pp. 5, 8:
  <https://www.featherweightaltimeters.com/uploads/1/0/9/5/109510427/raven_users_manual_2014may20.pdf>
- [6] Packard et al., *The Altus Metrum System* v1.9.22.2 (Creative Commons ShareAlike 3.0), PDF pp. 45, 72:
  <https://altusmetrum.org/AltOS/doc/altusmetrum.pdf>
- [7] Evelyn81 (Southern Utah University), *Unguided Rocket Flight Telemetry Datasets*, Apache 2.0, Featherweight
  BlueJay at 50 Hz: <https://www.kaggle.com/datasets/evelyn81/unguided-rocket-flight-telemetry-datasets>
- Logs, with each cache's source and license recorded where it was fetched:
  [public logs evidence gaps](public-logs-evidence-gaps.md), [altimeter bay temperature](altimeter-bay-temperature.md),
  [anomalous public flights](anomalous-public-flights.md), [public flights](public-flights.md),
  [hobby deployment fast logs](hobby-deployment-fast-logs.md), [ejection dip cause](ejection-dip-cause.md),
  [public CDX1 supersonic](public-cdx1-supersonic.md). Key repositories: <https://github.com/UND-ARC/Nasa-SLI-2024>
  (MIT), <https://github.com/werocketry/Hyperion-II> (MIT), <https://github.com/rex-mcall/Shred-O-Vision> (MIT),
  <https://github.com/AerospaceNU/rocket-flight-data> (GPL-3.0 data), <https://dofpro.org/RCK/fltdata/FlightData.html>.
- Private collection: aggregates only (counts, medians, spreads), no flight or design named.
