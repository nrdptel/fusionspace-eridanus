# The flown masses of NC State's 2025 Pelicanator flights

**In short.** Three of the Pelicanator's 2025 flights have an IMU log, all on an AeroTech L1520T: 2025-02-22
(mass not stated), the 2025-03-08 vehicle demonstration flight (VDF, 42.9 lb, 3.7 lb ballast) and launch week,
2025-05-04 at Bragg Farms (39.8 lb in the summary table, 40.1 lb by its section masses, 0.77 lb ballast). Every mass
is a table sum, not a stated scale reading. At those masses the launch-week log holds **3,444–3,470 N s** of
in-flight impulse against **3,627 N s** on the VDF: **4.3–5.0% less**, and 6.6–7.3% under the certified 3,715.9 N s.
The earlier reading that the two boosts "match" compared speeds and left out the 3.1 lb mass difference. The log also
shows a soft start that doesn't depend on mass: 26 ms from first motion to 5 g against 4–6 ms, no initial thrust
spike, and a burn 0.10–0.15 s longer. How sure: high on the log arithmetic (method choices move the impulse ratio by
0.1% or less); medium on the size, which moves 2.4% per pound of mass error and can't be pinned without a weighed
mass. In a simple vertical model the loss costs 345–405 ft of apogee, about the whole 383 ft shortfall, but that model
and the team's prediction don't share a baseline, so the split between motor and wind stays open. This moves
[M7.4, fault diagnosis](../ROADMAP.md): the flight can serve as a real motor-underperformance case with a stated
cause, carrying a mass caveat.

This builds on [anomalous-public-flights](anomalous-public-flights.md) (the fault, the first speed comparison),
[sl-weighed-mass-flights](sl-weighed-mass-flights.md) §2 (how the VDF's 42.9 lb was found) and
[student-launch-apogees](student-launch-apogees.md) (the launch-week prediction pairs).

## 1. The flights, their masses and their logs

| flight | site | log [NA] | motor | liftoff mass | how found | ballast |
|---|---|---|---|---|---|---|
| 2025-02-22, first VDF try | Bayboro, NC | `pelicanator_launch_1.csv` | L1520T | not stated | none | "ballasted" |
| 2025-03-08, VDF | Dalzell, SC | `pelicanator_launch_2.csv` | L1520T | 42.9 lb (19.46 kg) | table sum | 3.7 lb |
| unknown | unknown | `pelicanator_launch_3`: "No data recovered" | | | | |
| 2025-05-04, launch week | Toney, AL | `pelicanator_launch_4.csv` | L1520T | 39.8 or 40.1 lb | table sums | 0.77 lb |

- **Motor, each flight.** The FRR names the L1520T for the VDF [N25F p. 94, 183]; the PLAR's launch summary names it
  for launch week [N25PL p. 7]. Its Fig. 2.12, titled for the L1520T, plots "VDF Flight 1", "VDF Flight 2" and
  "Huntsville" as measured from the air-brake log [N25PL p. 15], which puts the 02-22 flight on the same motor.
- **VDF, 42.9 lb** [N25F p. 183]. The FRR's as-built table sums to 39.68 lb wet with 0.46 lb ballast (p. 30); the VDF
  carried 3.7 lb (p. 94); 39.68 − 0.46 + 3.7 = 42.92 lb. The simulation table calls its section masses "based on the
  measured VDF masses" (p. 55), but no page names a scale or a whole-vehicle weighing.
- **Launch week, 39.8 or 40.1 lb.** The summary table gives 39.8 lb, 0.77 lb ballast, 2.6 lb payload, 7 mph, 9:45 am
  CDT [N25PL p. 7]. Its descent table gives take-off section masses of 8.3, 13.5 and 18.3 lb, 40.1 lb in all, and
  14.2 lb for the fin can at landing (p. 13). **Inference:** both are sums; the nose's 8.3 lb is the FRR's 7.90 lb
  with the ballast and payload swapped (7.90 − 0.46 + 0.77 − 2.55 + 2.6 = 8.26 lb). The log's metadata gives
  `rocket_dry_mass_kg: 14.5` [NA]; its schema doesn't define "dry". Read as without the loaded motor, liftoff is
  14.5 + 3.651 = 18.15 kg (40.0 lb), which agrees with the tables; read as burnout mass, it is 36.1 lb.
- **What burns.** The FRR's fin-can breakdown lists 3.90 lb of propellant grains, a 0.14 lb delay/smoke grain, 1.63 lb
  of other motor parts and a 2.66 lb case with eye bolt (p. 31); the PLAR's fin can drops 4.1 lb (p. 13).
- **02-22 and the missing log.** The FRR calls 02-22 a "ballasted flight" aimed at 4,000 ft (p. 98). The repository
  has no `pelicanator_launch_3.csv` at its head commit (listed 2026-11-03); its README says "No data recovered" [NA].
  The FRR planned an optional flight on March 29 (p. 94). **Inference:** that flight, or another between 03-08 and
  05-04.
- **Dates.** Converted to UTC, the logs' first-motion stamps read 20:42:17 (02-22), 19:19:33 (03-08) and 14:45:10
  (05-04). The last is 9:45 CDT, as the PLAR says; the metadata's `11:45:10-04:00` is an hour later. The club's launch
  catalog lists only 2025–26 flights; `/launches/` returns 404 [NL].

## 2. The motor

- **Certified data** [TC]: Tripoli, 2010-06-04; total impulse 3,715.9 N s, average 1,567.8 N, maximum 1,765.3 N, burn
  2.36 s, loaded 3,651 g, propellant 1,854 g, RMS-75/3840 case.
- **The curves disagree slightly with the summary.** The two data files (manufacturer `.eng`, 2010; user `.rse`, 2020)
  hold the same samples, a 1,696.6 N peak and a 2.6 s end. With the implied zero at t = 0, both integrate to 3,769.0 N
  s (this note's trapezoid sum), 1.4% above the certified total.
- **Burnout mass** = liftoff − 1.854 kg (4.09 lb). Mass falls with delivered impulse at 3,715.9/1.854 = 2,004 m/s.

## 3. How the logs were read

- **Data.** Raw IMU packets at 500 Hz: `scaledAccelZ` in g on the body axis (−1 g at rest, so −Z points up);
  `deltaVelZ` agrees where no packets are missing (190.08 against 190.08 m/s on the VDF).
- **First motion.** The first sample above the pad mean + 0.03 g before the motor state, for the shape metrics; the
  start of the unbroken run above 1.15 g, for the integrals (they differ by 6 ms on launch week, 0 elsewhere).
- **Burn end.** The first sample after the peak where the axial specific force goes negative (thrust below drag).
- **Gravity.** Speed along the axis is ∫(a − g) dt, a vertical approximation. With the tilt below 27° in the burn,
  it understates speed by under 1.5 m/s (**Inference**).
- **Drag.** D/m_b = k v², with k fitted to the axial deceleration 0.3–1.3 s after burn end. The brakes were shut then
  on all three flights (on 02-22 they opened 5.3 s after first motion; the other two logs read encoder 0 in every
  row). Power-off drag overstates drag under thrust, so J reads slightly high on every flight.
- **Impulse.** T = m a + k m_b v², with m falling by T dt / 2,004 m/s from the stated liftoff mass.
- **Gaps.** Launch week's burn has 16 gaps of about 20 ms from 1.0 s to burn end (320 ms missing); 02-22's has 3 (54
  ms). The integrals bridge them by trapezoid; laying launch week's gaps onto the other two logs moves their impulse
  by 0.06% at most.
- **Range and clipping.** The 3DM-CX5-15's standard range is ±8 g, with ±2, 4, 20 and 40 g options [MS p. 2]. The logs
  read up to 10.9 g in the boost and launch week's 21.5 g at 56.8 s, so the unit had a wider option and the boost does
  not clip (**Inference**). Initial bias ±0.002 g is 0.04 m/s over a 2 s burn.

## 4. What the logs show

| measure | 02-22 | 03-08 VDF | 05-04 launch week |
|---|---|---|---|
| first motion to 1.5 g / 5 g / 8 g | 2 / 4 / 6 ms | 2 / 6 / 8 ms | 10 / 26 / 32 ms |
| initial spike | 10.9 g at 14 ms | 10.4 g at 14 ms | none; 9.5 g plateau from 42 ms |
| peak specific force | 10.90 g at 0.01 s | 10.90 g at 0.96 s | 10.69 g at 0.73 s |
| first motion to burn end | 2.07 s | 2.02 s | 2.17 s |
| thrust down to half its peak | 1.97 s | 1.90 s | 2.03 s |
| net Δv over the burn | 190.3 m/s | 190.1 m/s | 194.0 m/s |
| axial speed at burn end | 170.0 m/s | 170.2 m/s | 172.8 m/s |
| drag share, ∫D/m_b dt | 5.6 m/s | 5.7 m/s | 6.6 m/s |
| speed at 12 ft of travel | 81.8 ft/s, 302 ms | 82.7 ft/s, 300 ms | 83.1 ft/s, 314 ms |
| tilt at 2 s / 5 s / 8 s | 11° / 11° / 14° | 27° / 31° / 38° | 13° / 17° / 21° |

- **Speeds and tilt.** The 173.6 and 170.8 m/s in [anomalous-public-flights](anomalous-public-flights.md) are this
  table's 172.8 and 170.2 m/s, read over slightly different windows. Tilt is the body axis's angle from vertical by
  the IMU's own attitude quaternion.
- **Repeatability.** The two VDF-campaign flights gained the same net Δv to 0.12% (190.30 against 190.08 m/s).
  **Inference:** the 02-22 flight flew close to the VDF's mass, if its motor matched.

## 5. Does the launch-week log show an impulse loss?

| flight | mass used | in-flight J | vs 3,715.9 N s | liftoff mass giving 3,715.9 N s |
|---|---|---|---|---|
| 02-22 | 42.9 lb (assumed) | 3,629 N s | −2.3% | 43.9 lb |
| 03-08 VDF | 42.9 lb | 3,627 N s | −2.4% | 44.0 lb |
| 05-04 launch week | 39.8 lb | 3,444 N s | −7.3% | 42.9 lb |
| 05-04 launch week | 40.1 lb | 3,470 N s | −6.6% | |

- **At the stated masses, yes:** launch week delivered 157–183 N s (4.3–5.0%) less than the VDF. The lighter rocket
  should have gained 211.7 m/s of thrust Δv on the VDF's impulse; it gained about 200.6.
- **Method choices barely matter.** Drag ×0.7 or ×1.3, exhaust speed ±10% and a burn end 0.2 s later move each
  impulse by up to ±0.8%, but the launch-week/VDF ratio only between −4.9% and −5.1%.
- **Mass decides it.** Each impulse moves 85–87 N s (2.4%) per pound. The gap closes if launch week weighed 1.8–2.1
  lb more than its tables (41.9 lb on the 39.8 lb reading) or the VDF 1.85–2.2 lb less (40.7 lb). Each 0.1 lb of
  propellant burned on the pad before liftoff lowers the launch-week impulse by a further 0.25%; the PLAR says that
  burn "used significant fuel" but gives no amount [N25PL p. 15], so this side is unbounded.
- **The log alone can't separate the two readings.** An accelerometer measures thrust per unit mass: a 42.9 lb
  launch-week rocket with a normal motor fits the integral as well. Coast drag doesn't decide it either: k/ρ, which is
  C_D A/2m_b, reads 2.16e-4 on the VDF and 2.14e-4 on launch week. Equal drag areas would make the two burnout masses
  agree within 1%, not 8% apart. But 02-22, with the VDF's Δv, reads 1.94e-4, 10% lower, so drag scatter between
  flights is as large as the effect. The team's flown C_D, 0.46 for the VDF (0.44 predicted) [N25F p. 101] and 0.43
  for launch week [N25PL p. 16], don't say how they were found.
- **The shape is mass-free evidence.** A slow rise (26 ms to 5 g against 4–6 ms), no ignition spike, and thrust
  holding on 0.06–0.13 s longer are what a soft start looks like (**Inference**). The missing spike matches the PLAR's
  reading of its Fig. 2.12: the usual initial peak "was not present" [N25PL p. 15]. The longer tail doesn't: in the
  figure the Huntsville curve falls before both VDF curves, perhaps on a different time zero. Taken at 39.8 lb, the
  thrust peaks at 1,844 N and averages 1,786 N over 0.2–1.6 s, against 2,017 and 1,942 N on the VDF at 42.9 lb. The
  PLAR reports a measured peak of 1,932.05 N (p. 15). By this note's method that takes about 41.7 lb, but the figure's
  VDF curves sit 3–4% above this note's at 42.9 lb, so the team's method differs and its peak doesn't pin the mass.
  The team suspects the igniter; the liner and combustion remnants looked normal after flight (p. 24).
- **What the log can't show.** The video's roughly 3 s of small flame before full thrust [N25PL p. 15] happened on the
  pad, where the IMU reads 1 g until thrust exceeds weight. Launch week's pad noise is 1.1–2.8 times the VDF's, but
  wind can do that too.

## 6. The 383 ft shortfall, cause by cause

The PLAR's apogees: 4,226 ft flown against 4,609 ft (RocketPy), 4,701 ft (OpenRocket) and 4,854 ft (RASAero II) [N25PL
p. 9]. Its post-flight run on the flight's thrust curve came within 29 ft (p. 15); a curve taken from the log and
flown at the mass it was derived with reproduces the boost by construction, so that match can't confirm the loss on
its own. The sizes below come from a one-dimensional vertical model: each flight's log thrust curve, its coast C_D A,
the pad density (from the log's metadata) with a standard lapse, and mass falling with impulse. **Inference**
throughout; only differences within the model are meant. The model and RocketPy don't share a baseline: on the
certified curve at 39.8 lb the model gives 4,974–5,089 ft (scaled to 3,715.9 N s, or as filed), 365–480 ft above
RocketPy's 4,609 ft, which already holds the day's wind. So the terms below don't sum to the 383 ft.

- **Impulse loss: −345 to −405 ft.** At 39.8 lb, launch week's own curve gives 4,435–4,442 ft and the VDF's curve
  4,838 ft (−396 to −404 ft); at 40.1 lb, about −345 ft. Against the certified curve the loss is 540–650 ft.
- **Mass: ±155–160 ft per pound.** The flown 39.8–40.1 lb sits 0.1–0.4 lb over the FRR's competition prediction
  of 39.68 lb (p. 55), about −15 to −65 ft. Which mass RocketPy used on the day isn't stated; the PLAR's lessons
  name "weight creep" and winds aloft as reasons the declared apogee was too high (p. 23).
- **Weathercocking, wind and model drag: about 210–215 ft** in the model's frame, what's left after the impulse loss
  (4,435–4,442 ft modeled against 4,226 ft flown): tilt of 13–21° in the first 8 s, wind (7 mph [N25PL p. 7]; 12.9
  km/h in the metadata) and the model's drag share it.
- **Air brakes and pad offset: 0.** "Active but not deployed" [N25PL p. 7]; the encoder reads 0 in all rows. The −26
  ft pad reading is the air-brake computer's zeroing (p. 14).

**Verdict, as a range.** The soft start is visible in the log whatever the mass. An impulse loss of 4.3–5.0% against
the VDF (157–183 N s) holds at the masses stated, more if propellant burned on the pad (0.25% per 0.1 lb, unbounded).
It rests on two sums 3.1 lb apart, and a 1.8–2.2 lb error in either would erase it. The VDF side is the firmer: its
sections are "based on the measured VDF masses" (FRR p. 55). The flight is usable as a motor-underperformance case for
[M7.4](../ROADMAP.md) with that caveat. The loss is large enough to account for the whole 383 ft on its own; how the
shortfall splits between motor and wind needs a run in RocketPy's frame.

## What the project can use

- **AirbrakesV2 logs** (MIT [NA]): derived numbers can be published with attribution, and a 10 ms thrust-per-mass
  trace committed under the MIT notice.
- **NC State reports** (no license stated): cite only. **ThrustCurve** [TC]: cite; the API marks the `.rse` file "PD"
  and gives no license for the `.eng`. **The datasheet** [MS]: © Parker, cite.
- **Suggestion, fault diagnosis:** fly launch week twice in HPR Sim at 39.8 lb, once on the certified curve and once
  on the log-derived curve. Expected effect: the certified curve overshoots as the team's runs did (+9% to +15% [N25PL
  p. 9]), and the log curve removes 540–650 ft of it in the vertical model (**Inference**).
- **Suggestion, motor checks:** score rise time and initial spike, which are mass-free, beside impulse, so a
  motor-fault case doesn't rest on table masses alone.
- **Suggestion, repeatability:** the 02-22 and 03-08 boosts agree to 0.12% in Δv; with a weighed mass for one of them,
  they would bound L1520T flight-to-flight variation.

## Open questions

- Was the launch-week rocket weighed? Which of 39.8 and 40.1 lb is it, and what did the 02-22 flight weigh?
- How much propellant burned during the roughly 3 s on the pad?
- Which mass produced the PLAR's 1,932.05 N peak and the Fig. 2.12 curves, and on what time zero?
- How does the 383 ft split between motor and wind in RocketPy's own frame, on the certified and the log curve? Which
  flight is `pelicanator_launch_3`, and why is launch 4's metadata time an hour after its log?

## Sources (read 2026-11-03)

1. **[N25F]** NC State 2025 FRR, pp. 30–31, 55, 94, 98, 101, 183 (PDF pages):
   <https://ncsurocketry.org/wp-content/uploads/2025/03/north-carolina-state-university-2025-frr-report.pdf>.
2. **[N25PL]** NC State 2025 PLAR, pp. 7, 9, 13–16, 23–24 (PDF pages):
   <https://ncsurocketry.org/wp-content/uploads/2025/06/north-carolina-state-university-2025-plar-report.pdf>.
3. **[NA]** AirbrakesV2 at afdcecb44e97 (head on 2026-11-03), `launch_data/`, MIT:
   <https://github.com/NCSU-High-Powered-Rocketry-Club/AirbrakesV2>.
4. **[NL]** NC State launch catalog, 2025–26 entries only: <https://ncsurocketry.org/launch-catalog/>.
5. **[TC]** ThrustCurve, AeroTech L1520T: <https://www.thrustcurve.org/motors/AeroTech/L1520T/>; API search and
   download (motor `5f4294d2000231000000038e`): <https://www.thrustcurve.org/api/v1/search.json>.
6. **[MS]** MicroStrain 3DM-CX5-15 datasheet, document 8400-0115 Rev F, p. 2, the Wayback Machine's 2023-11-10 copy:
   <https://www.microstrain.com/sites/default/files/3dm-cx5-15_datasheet_8400-0115_rev_f.pdf>.
