# What motor delays measure in public flight logs

How far does a motor's ejection delay stray from its label in real flights, and does
[motor delay spread](motor-delay-spread.md) §7's Monte Carlo distribution hold there? It serves the "delay mismatch"
fault (M7.4 in the [roadmap](../ROADMAP.md)), the settings check (M6.8) and the optimum-delay output. It builds on
[NAR's delay reports](nar-delay-reports.md) (Staab's 39 field flights) and
[faulty public flights](anomalous-public-flights.md). Read 2026-11-04: public course pages and logs, team
repositories, forum files, ThrustCurve; no source code. **Inference** marks this note's own timing and arithmetic.
Trust it for the sign and rough size of in-use scatter in model motors (26 flights, 9 types); thinly for high power.

**In short.**
- **In flight, delays center on their labels but scatter twice as wide as §7.** On 30 flights timed from an
  accelerometer, the delay ran a median 0.04 s long, mean −0.02 s (95% t interval −0.40 to +0.36 s), SD 1.03 s.
  §7's 90% interval held 21 or 22 of 30 (70% to 73%), its 95% interval 25 (83%). Staab's field flights: 67%, 74%.
- **The extra scatter is between motor types, not between firings.** Pooled within-type SD is 0.49 s on 18 degrees
  of freedom, the NAR sheets' 0.50 s. Type means run from 2.55 s early (one D13-10W) to 2.4 s late (one H128W-10).
- **Against the same type's NAR sheet, warm flights ran short and cold ones long.** 5 of 9 types fired 0.36 to
  0.82 s before their sheet's mean. The four later were Quest C6-5 (+0.61 s) and three types flown in air near
  freezing: Estes E12-6 (+1.16 s, +20%), AeroTech G77-10R (+0.16 s) and G79-10W (+0.58 s) (**Inference**, 9 types).
- **4 of 30 flights broke NFPA 1125's band (13%, 95% 3.8% to 30.7%):** three early, one late by 2.4 s. With
  Staab's 5 of 39, field flights miss 9 of 69 times (13.0%, 95% 6.1% to 23.3%); certification firings almost never.
- **A log's burnout can come before the certification clock starts,** so a log-timed delay read 0.03 to 0.22 s long
  on the high-power flights and the D13 (**Inference**). That is the opposite sign to motor delay spread §1's
  inference; on the black-powder flights the two clocks agree within 0.01 s.
- **Not bounded here:** drilled delays (none confirmed flown), high power beyond 4 flights, and any temperature slope.

## 1. What was timed, and how

**Sources.** All flights below have motor ejection only and a stated motor and delay. Harvey Mudd College's
advanced rocketry course posts its flights' logs, motors, air temperatures and wind [1]. Its first-flight lab posts
2015 test flights [2]. Logs were read from the project's local, uncommitted cache of these public pages, plus files
fetched for this note [1]. Rockets are not named; results are per motor type. ThrustCurve gave each type's delay
options and burn time [5].

**Loggers and rates.** A Featherweight Raven, axial accelerometer at 400 Hz (16 flights, 2017). A course logger
with an IMU at about 43 Hz that clips at 8 g (7 flights, 2019). A newer course logger with a 16 g IMU near 1.1 kHz,
a high-g accelerometer and a barometer (7 flights, 2021 to 2023). A PerfectFlite Pnut barometer at 20 Hz (30 flights,
2015, used only in §4). Every course site sat at 91 to 92.5 kPa pad pressure, about 800 to 900 m.

**Definitions (Inference).**
- *Burnout* is where the axial accelerometer reading (specific force) first stays below zero, that is where thrust
  falls below drag. §5 sizes how far that is from certification's 5%-of-peak burnout.
- *Ejection* is the first sample at least 0.5 s after burnout whose axial reading leaves the median of the 50 to
  250 ms before it by more than 3 g. On the 1.1 kHz logger it is a 15 to 20 ms burst; the airframe jolt follows
  0.13 to 0.38 s later. This note times the burst, so a jolt-based reading would add up to 0.4 s.
- *Offset* is measured delay minus label. The NFPA band is 1.5 s or 20%, whichever is greater, capped at 3 s.
- Nine flights were checked by eye; on two high-power flights boost vibration tripped the rule, so their spikes
  were read by eye. A second, independent timing of 27 logs agreed within 9 ms on 25 (J270W 18 ms; the G78 is
  taken from the burst in its untrimmed file). Three later-added flights (G77, G79, one H128W) rest on it alone.

**Labels** come from the course tables and file names. 9 s is the F67W's longest delay on ThrustCurve, so it
wasn't shortened. AeroTech's 2023 matrix lists the H128W as a 14 s adjustable reload [9], so the H128W-10 may be
drilled. No flight is known drilled. Motor ejection rests on the course tables and one charge per log (**Inference**).

## 2. The offsets, by class

| Set (accelerometer-timed) | Flights (types) | Median | Mean (95% t) | SD | Early | Outside NFPA band |
|---|---:|---:|---:|---:|---:|---:|
| Model, black powder (C6, E12) | 15 (4) | +0.02 s | −0.14 s (−0.68 to +0.41) | 0.98 s | 7 | 2 |
| Model, composite (D13, F67, G77, G78, G79) | 11 (5) | +0.21 s | −0.04 s (−0.74 to +0.67) | 1.05 s | 4 | 1 |
| High power (H128W, J270W) | 4 (3) | −0.07 s | +0.46 s | 1.31 s | 2 | 1 |
| All | 30 (12) | +0.04 s | −0.02 s (−0.40 to +0.36) | 1.03 s | 13 | 4 |

- **The median's 95% interval** from order statistics is −0.43 to +0.67 s on all 30 flights (**Inference**).
  The data can't tell a 3% early bias, as §7 assumes for model motors, from none.
- **Black powder without the cold E12s** runs early: 9 C6 flights, median −0.85 s, mean −0.79 s (95% −1.30 to
  −0.28 s).
- **Composite model motors center on the label.** The seven F67-9W flights averaged 9.39 s (SD 0.72 s, range
  8.07 to 10.19 s); the one-flight types are in §3, the D13-10W 25% early.
- **The four high-power flights** read 12.41 s on a 10 s H128W, 14.02 and 13.84 s on two 14 s H128Ws, and
  13.59 s on a 14 s J270W. None supports §7's 10% early center, but four flights can't size it.
- **Timing resolution** is 2.5 ms (Raven), 23 ms (2019 logger) and about 1 ms (newer logger). The event
  definitions, not the sample rate, set the error: about 0.05 s on black powder and up to 0.3 s on high power.

## 3. Flight against the bench, motor type by motor type

| Type | Flights | Flight mean | Flight SD | NAR sheet mean [6] | Flight minus sheet |
|---|---:|---:|---:|---:|---:|
| Estes C6-5 | 3 | 3.83 s | 0.39 s | 4.28 s | −0.45 s |
| Estes C6-7 | 3 | 5.82 s | 0.46 s | 6.24 s | −0.42 s |
| Quest C6-5 | 3 | 4.98 s | 0.25 s | 4.37 s | +0.61 s |
| Estes E12-6 (air −5 to +8 °C) | 6 | 6.84 s | 0.26 s | 5.68 s | +1.16 s |
| AeroTech D13-10W | 1 | 7.45 s | | 8.27 s | −0.82 s |
| AeroTech F67-9W | 7 | 9.39 s | 0.72 s | 9.75 s | −0.36 s |
| AeroTech G77-10R (air 1 °C) | 1 | 10.21 s | | 10.05 s | +0.16 s |
| AeroTech G78-10G | 1 | 9.45 s | | 10.23 s | −0.78 s |
| AeroTech G79-10W (air 4 °C) | 1 | 9.74 s | | 9.16 s | +0.58 s |

- **Firing scatter matches the bench.** Within a type, flights scatter 0.12 to 0.72 s, pooled 0.49 s; motor delay
  spread found 0.50 s within NAR's groups. What widens the field spread is each type's own offset.
- **Five of nine types fired early against their sheets;** the nine types' median gap is −0.36 s. Sheets
  and flights are years apart, so lot and age can't be told apart. Weaver's black-powder delays shortened 0.47 s a year
  ([NAR's delay reports](nar-delay-reports.md) §3), which has the right sign.
- **The Quest C6's burn ran long in flight:** 2.73 to 2.99 s from liftoff to burnout, against 2.24 s to 5% of
  peak on ThrustCurve's certified curve [5]. Its +0.61 s against the sheet is timed from that later burnout.

## 4. A baro-only check on 15 more model flights

The 2015 lab flights carry only a 20 Hz barometer. Ejection shows as a one-sample altitude dip from the charge's
pressure pulse; burnout is liftoff plus ThrustCurve's burn time to 5% of peak [5]. 16 of 30 flights show a dip of
5 ft or more before landing; one with two equal dips was dropped, leaving 15 (**Inference**, chosen by size).

- **Offsets: median +0.15 s, mean +0.14 s (95% −0.26 to +0.53 s), SD 0.71 s, 7 of 15 early,** on Estes and
  Quest B4, B6 and C6 motors with 2 to 7 s labels. None broke the NFPA band (95% upper bound 21.8%).
- §7's 90% interval held 12 of 15. Liftoff from a barometer and an assumed burn time add about ±0.25 s each.
- **These don't enter §2's totals.** Quest C6 burn times differ by 0.5 to 0.75 s between ThrustCurve and the Raven
  logs (§3), which moves three of the 15 by that much. The 23 logs at 10 Hz from 2011 [2] weren't timed.

## 5. Which burnout the clock starts from (sign and size)

**A log-timed delay reads long, not short, on the high-power flights and the D13 (Inference).** Certification
times from thrust falling under 5% of peak ([motor delay spread](motor-delay-spread.md) §1); a log's burnout is
where thrust falls under drag, which comes earlier whenever drag exceeds 5% of peak thrust.
- On five flights from the newer logger, drag at burnout was 9% to 19% of the boost's peak specific force.
  On each type's ThrustCurve curve, thrust takes 0.03 s (H128W) to 0.22 s (J270W) to fall from there to
  5% [5]. So the certification-clock delay is that much shorter than the log's.
- On the Raven's black-powder flights the two clocks agree within 0.01 s: drag was 3% to 9% of peak, and the
  thrust drop is steep. On six of them drag was under 5%, where §1's sign holds, by under 0.01 s. On the D13-10W,
  drag was 12% of peak: about 0.17 s on its certified curve.
- **An inertial "acceleration below zero" starts earlier still**, where thrust falls under drag plus weight. On the
  forum flight in §7 that crossing came 0.07 to 0.10 s before the specific-force one.
- Motor delay spread §1 inferred "a delay timed from a log reads short": true only if drag at burnout is under 5%
  of peak thrust.

## 6. Temperature and altitude

- **The cold E12-6s ran 20% long against their sheet,** at air −5 to +8 °C (course table, mean 0.1 °C) [1].
  The black-powder slope motor delay spread uses (−0.30% per K) predicts about +6% at 0 °C. Motor temperature wasn't
  logged; the avionics read 5 to 17 °C at power-up, so the motors may have been warmer than the air (**Inference**).
  Lot and cold can't be told apart: within the morning, the slope was −0.30% per K, 95% −1.41% to +0.81%.
- **Seven F67-9W flights at 7 to 23 °C show no slope:** +0.27% per K, 95% −1.35% to +1.88% (**Inference**). That
  interval holds zero, the black-powder figure and the −0.15% per K of 71 bench-fired F motors
  ([two grain temperatures](hobby-motor-temperature-firings.md) §2).
- **The cold composite flights ran long too.** The G77-10R and G79-10W, in air at 1 and 4 °C, fired 1.5% and 6.4%
  past their sheets (NAR's, fired at 20 °C ± 5 °C); the F67, D13 and G78 in warmer air fired short.
- **No lengthening at altitude shows.** Motor delay spread §6's pressure rule gives 3% to 9% longer composite
  delays at 91 to 92.5 kPa, for burn-rate exponents 0.3 to 0.8 (**Inference**). The three warm composite types
  instead fired 3.7%, 9.9% and 7.6% short of their sheets. Lot and age effects of that size would hide it.

## 7. Flights left out, and why

- **Label unknown.** Three RIT L1 flights on one day, AeroTech HP-H195NT, motor ejection [3]. Ejection came 11.5 to
  12.4 s after liftoff plus ThrustCurve's 1.16 s burn. Their design file says 10 s; the motor ships at 14 s,
  drillable (**Inference**). That is +1.5 to +2.4 s or −1.6 to −2.5 s, so they're not counted.
- **A stated label that may not be what flew.** A forum J270W marked 15 s, flown with a TeleMega [4], fired 11.4 to
  11.5 s after burnout, 3.5 s early ([faulty public flights](anomalous-public-flights.md) timed it the same way).
- **Ambiguous label.** One G80T flight is "G80-14T" in the course table and "G80T-12" in its file name [1]. It fired
  11.36 s after burnout: 0.64 s early if drilled to 12 s, 2.64 s early if not.
- **Not separable.** Two dual-deploy flights whose altimeter and motor charges can't be told apart, and four GPS-only
  flights. The private collection's 151 flights from one flight-log site fly electronics; 55 name a delay
  (a pattern count). Whether their motor charges were kept, and fired later, wasn't searched.

## What this suggests

- **Draw a per-type offset, then per-firing scatter.** For M6.8 and the optimum-delay output, a type offset with
  SD near 0.91 s (**Inference**: √(1.03² − 0.49²)) plus 0.49 s per firing reproduces 30 flights' 1.03 s SD.
  §7's single model-wide center reproduces the bench, not use. With a NAR sheet for the type, its mean still
  missed the flight mean by 0.16 to 1.16 s here.
- **Give M7.4's delay mismatch a field rate of about 13%** (9 of 69 flights, 95% 6.1% to 23.3%), most of them early.
  A log showing a charge 2 s or more off the label is not rare enough to call a fault on its own.
- **Time a logged delay from the log's own burnout, and say so.** The analyzer should state that its burnout comes
  before the 5%-of-peak point, so it reads 0 to 0.2 s long against a certified delay. Correct motor delay spread §1.
- **Fold in AltimeterCloud.** Its flight pages print "Burnout (first)" and "Possible ejection" times from Mercury
  accelerometer logs [7]; the static page showed no motor and no terms, its script-built parts weren't read.

## Open questions

- **Drilled delays in use.** Motor delay spread found drilled AeroTech delays 0.8 s late on the bench. No flight
  here is known drilled; the G80T and the RIT flights might be.
- **High power.** Four timed flights. A forum post reports a CTI J240-16 drilled to 14 s that ejected 23 s after
  burnout [8]; only a search summary was read, so it isn't counted.
- **Cold motors.** No flight logged motor temperature; the coldest composite flights flew in 1 and 4 °C air, one
  each. Whether cold, not lot, made the E12, G77 and G79 run long needs paired warm and cold flights.

## Sources

Read 2026-11-04. The RIT team repository is MIT; the course pages and forum files carry no license: cited, cached
under `refs/`, not copied, and the course logs are reported here as per-type statistics only.

1. Harvey Mudd College, E178 advanced rocketry, "Flight Data" page: flight tables (motor, air temperature, wind) and
   log files, `https://pages.hmc.edu/spjut/AdvRoc/FlightData.md.html`; 13 logger files fetched from its
   `FlightData/` folder to `refs/research/delay-offset-in-flight-logs/` (`hmc/`, `verify/dl/`).
2. Harvey Mudd College, E80 "First Flight" lab page, appendix "Rocket Data" (Pnut at 20 samples per second, launch
   site at about 3,000 ft): `https://pages.hmc.edu/spjut/NewE80/FirstFlightLab.html`
3. RIT Launch Initiative, `flight-data`, `2026/AirbrakesL1Tests/` (README, `rocket.ork`, controls module logs):
   <https://github.com/RIT-Launch-Initiative/flight-data>
4. "Failed Level 2 attempt", The Rocketry Forum, posts 1 and 49, with the poster's TeleMega CSV:
   <https://www.rocketryforum.com/threads/failed-level-2-attempt-but-many-things-learned.146905/>; Altus Metrum,
   *The Altus Metrum System* (CSV columns), <https://altusmetrum.org/AltOS/doc/altusmetrum.pdf>
5. ThrustCurve motor search API and simulator files (delays, certified and maker curves) for Estes A8, B4, B6, C6,
   E12; Quest B6, C6; AeroTech D13W, F67W, G78G, G80T, H128W, HP-J270W, HP-H195NT:
   <https://www.thrustcurve.org/api/v1/search.json>
6. NAR S&T motor data sheets as parsed for [motor delay spread](motor-delay-spread.md) (its source 8): Estes C6, E12;
   Quest C6; AeroTech D13, F67W (2017), G77, G78, G79.
7. Altimeter Cloud, flight 1691 page (Mercury V1; "Burnout (first)", "Possible ejection"):
   <https://www.altimetercloud.com/flights/1691/>
8. "Motor Ejection Delay Issue", The Rocketry Forum, post 1904426 (search summary only; the page refused plain
   fetches): <https://www.rocketryforum.com/posts/1904426/>
9. AeroTech, Master Motor Matrix, 4/2/23 (H128W-14A, RMS-29/180 reload, adjustable delay), as cached for
   [motor delay spread](motor-delay-spread.md).
