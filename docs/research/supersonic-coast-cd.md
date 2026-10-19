# Flight-measured coast drag past Mach 1, against the simulators' curves

This note collects every public case found where one rocket has a flight-measured coast C_D and a code's C_D over
the same Mach range above 0.8. It asks whether their ratio moves through Mach 1: can one scale correct a code's
airframe drag, or does the correction need a Mach term? It builds on [flight-derived-cd](flight-derived-cd.md),
whose coast pairs against OpenRocket and RockSim were all subsonic. **In short: against RASAero II the ratio moves by
less than 0.1 through Mach 1, and the one flight against OpenRocket says it rises by about 45%.** Eight flights
were fitted against RASAero II above Mach 1.2, to 2.94. Their supersonic ratios run 0.85–1.08. Through Mach 1 the
level steps by about zero on one 38 mm rocket and by +0.03 and +0.09 on two others. What moves is the shape: on
the 38 mm rocket the flight's drag rise ends at Mach 0.98–1.00 against RASAero's 1.04–1.06, so the ratio swings
from about 0.92 to 1.08 (±0.03) over Mach 0.92–1.00. One J825R flight to Mach 1.5 (Bawa and Michalski) reads
0.69–0.79 of OpenRocket subsonic and 1.03–1.10 supersonic. That flight's mass wasn't weighed, and its velocity
drifted. A finish setting moved one Mach 2.8 flight's code curve by 15–40%, more than any Mach term. No public
pair against RockSim, and none against OpenRocket with a weighed mass, passes Mach 0.8. **How sure:** medium on
the RASAero side, four sources by three groups; low on the OpenRocket side, one flight read off a plot.

## 1. How the cases were found and read

Two passes searched in parallel: competition and academic sources (Spaceport America Cup reports, AIAA papers,
theses), then hobby sources (The Rocketry Forum, RASAero II's comparison pages, club archives, GitHub). The first
ran 29 web searches and opened 16 documents. The second read 13 forum threads and RASAero's pages, and ran six
GitHub searches. A case counts when one rocket has both a flight C_D and a code C_D over the same Mach range.
Bands: subsonic below Mach 0.8, transonic 0.8–1.2, supersonic above 1.2. The ratio is flight C_D over code C_D.
Most numbers were read by pixel off plots, each with its read-off error. **Inference** marks this note's arithmetic.

Every fit here divides drag by mass. A 1% mass error is a 1% C_D error in every band at once, so it moves the
level, not the Mach trend ([flight-derived-cd §5](flight-derived-cd.md)). An accelerometer offset hurts most near
apogee, where drag per mass is smallest. That is the slow end of a coast, so it bends the subsonic band most.

## 2. Coast fits against RASAero II (eight flights)

| Rocket; motor; peak Mach | Method; mass | Subsonic | Transonic | Supersonic |
|---|---|---|---|---|
| 38 mm min-diameter; Loki J1026, 2023; 2.85 [S5] #9, #71 | accel; weighed | n/a | n/a | 0.94 at Mach 2.60 |
| Same line; J510, 2023; ≈2.2 [S5] #87 | accel, 2 units; estimated | 0.98–1.05 | 0.92–1.08 | 1.00–1.04 |
| Rebuilt; J1026 and K627, 2024; 2.66 [S5] #122–#130 | accel; weighed | 0.96–1.04 | 0.94–1.13 | 0.97–1.06 |
| 3 in; CTI M2020; 2.84 [S6] #1 | accel, 2 units; wet weight | n/a | n/a | ≈1.0 rough; 1.15–1.4 smooth |
| Proteus 6, 6 in; P9381; 2.94 [S7] | accel velocity; design mass | n/a | n/a | ≈1.08 (effective) |
| Violent Agreement I216 and J530 SS; 1.5, 2.4 [S14] | accel | 0.84–0.88 | n/a | 0.85–0.97 |

- **J1026, 2023.** The source gives 0.347 measured against RASAero's 0.369 power-off at Mach 2.60 [S5] #71. Mass
  was weighed: 907 g after boost. Density came from the nearest weather balloon, and the high-g data were scaled
  so that integrated velocity reaches zero at the barometric apogee. **Inference:** RASAero's screenshot uses a
  1.630 in reference diameter and the source a 1.61 in nose. On RASAero's area the ratio is 0.92. Below Mach 2.45
  the fins fluttered and drag jumped, so only Mach 2.47–2.75 counts.
- **J510, 2023, read by pixel** off forum thumbnails [S5] #87: about ±0.01 in C_D, ±0.015 in the steep rise and
  the noisy subsonic band. Motor not weighed, so the mass is estimated, and the two units' velocities disagree.

| Mach | 0.5 | 0.85 | 0.92 | 0.96 | 0.98 | 1.00 | 1.05 | 1.2 | 1.4 | 1.6 | 1.8 |
|---|---|---|---|---|---|---|---|---|---|---|---|
| Flight | 0.437 | 0.411 | 0.413 | 0.453 | 0.492 | 0.572 | 0.578 | 0.564 | 0.529 | 0.476 | 0.449 |
| RASAero II | 0.415 | 0.419 | 0.447 | 0.486 | 0.509 | 0.528 | 0.568 | 0.549 | 0.511 | 0.476 | 0.443 |
| Ratio | 1.05 | 0.98 | 0.92 | 0.93 | 0.97 | 1.08 | 1.02 | 1.03 | 1.04 | 1.00 | 1.01 |

- **Rebuilt rocket, 2024.** The masses were weighed [S5] #130, but no RASAero curve for this build was posted. Its
  ratios use the 2023 file and are only indicative. Read off the plot (±0.005): 0.40 subsonic, 0.56 at the peak (Mach
  0.98–1.04), 0.37 at Mach 2.6 [S5] #123. Near Mach 1 that gives 0.94–1.09 (J1026) and up to 1.13 (K627). RASAero
  predicted 27,000 ft for the J1026 [S5] #110. It flew a 24,750 ft record [S8], so the prediction ran 9% high while
  the coast C_D sat within a few percent. **Inference:** the miss sits in the boost. Burnout speed was 3,000 ft/s,
  against 3,107 ft/s on the same motor type in 2023.
- **M2020 to Mach 2.84** [S6] #1. The paint stripped off in flight. The source fits about 5 s after burnout (Mach
  about 2.6 to 1.5) and finds it closest to RASAero's "rough camouflage" finish. Read off the plot (±0.1), the
  flight runs 1.15–1.4 times the "smooth paint" curve. The source back-computes a 9% thrust shortfall. Before the
  flight RASAero predicted 45,400 ft; it flew 34,190 ft. With rough paint and the shortfall, RASAero gives 35,000 ft.
  **Inference:** the finish alone was about −12% of the prediction, the motor the rest.
- **Proteus 6** [S7]. The page shows velocity, not C_D. **Inference, read off the plot (±10 ft/s):** from 10 s to
  30 s RASAero loses 1,316 ft/s and the two accelerometers 1,367 and 1,372 ft/s. Taking gravity's share as 600–644
  ft/s, the drag share flown is 1.07–1.08 times the simulated (±0.04). It carries the design file's mass.
- **Violent Agreement**: already in [flight-derived-cd §2](flight-derived-cd.md), not counted twice.

**Inference, by band.** Against RASAero II the supersonic ratio spans 0.85–1.08 on smooth-finish settings, and 1.0 on
the stripped rocket with the rough setting. On the rockets with both sides of Mach 1, the level steps by about zero
on the J510 (band means +0.005; cell by cell −0.05 to +0.06) and by +0.09 (I216) and +0.03 (J530). The 2024 flights
against the older file climb about +0.1 from Mach 0.5 to 2.6. That is a different build, so it is a weak sign.

**The rise's position, not its size.** On both 38 mm builds the flight stays flat to Mach 0.91–0.94 and levels off
by 0.98–1.00. RASAero bends upward at 0.90 and peaks at 1.04–1.06 [S5] #87. Scaling a code curve by one
number then leaves a ±8% swing over Mach 0.92–1.02 from shape alone. Mach here comes from integrated velocity: a
2% velocity error moves it by 0.02, a quarter to two-fifths of the shift. On a transonic flight, one flyer read
drag divergence starting near Mach 0.87, against OpenRocket 24.12's 1.0 [S12] #1, #3. That claim comes without a C_D.

## 3. The one coast fit against OpenRocket past Mach 1

Bawa and Michalski (RMIT) flew a slender conical-nose rocket on an AeroTech J825R to about Mach 1.5 [S1]. They
reconstruct drag from a 1-DOF balance: thrust from the ThrustCurve file, acceleration from two 500 Hz IMUs (p. 2),
density from logged pressure and temperature, velocity and height by integrating acceleration (pp. 3–5). Their baro
and inertial apogees differ by about 1,500 ft (p. 5). The paper gives an initial loaded mass, never says it was
weighed and gives no value (p. 3). It says the coast C_D is "considerably lower" than RASAero's and closer to
OpenRocket's (p. 9). [flight-derived-cd](flight-derived-cd.md) left this case out because the paper prints no
values. This note reads its Fig. 17 (p. 9) by pixel, at 300 dpi, on the tick marks. The read-off error is ±0.01 on
the smooth curves. The flight line oscillates by ±0.05 from Mach 0.6 to 0.8, then dips smoothly from 0.94 to
0.69 and back to 0.90 over Mach 0.8–1.0. Each band takes its median, which there summarizes a dip, not noise.

| Mach | 0.3–0.4 | 0.5–0.6 | 0.8–0.9 | 0.9–1.0 | 1.0–1.1 | 1.1–1.2 | 1.2–1.3 | 1.3–1.4 | 1.4–1.5 |
|---|---|---|---|---|---|---|---|---|---|
| Flight | 0.47 | 0.56 | 0.74 | 0.73 | 0.88 | 0.84 | 0.80 | 0.78 | 0.76 |
| OpenRocket | 0.68 | 0.71 | 0.77 | 0.81 | 0.89 | 0.86 | 0.77 | 0.72 | 0.69 |
| RASAero II | 1.04 | 1.07 | 1.25 | 1.44 | 1.69 | 1.63 | 1.52 | 1.42 | 1.35 |
| Flight/OpenRocket | 0.69 | 0.79 | 0.96 | 0.90 | 0.99 | 0.98 | 1.03 | 1.08 | 1.10 |

**Inference.** Against OpenRocket the mean ratio is about 0.74 subsonic, 0.96 transonic and 1.07 supersonic: +45%
from Mach 0.3–0.6 to 1.2–1.5. Against RASAero it stays flat at 0.45–0.59 (RASAero's curve ends at Mach 1.44), but
RASAero reads 1.5 times OpenRocket subsonic (1.04 against 0.68). Such gaps happen: ADR-029 records another rocket's
RASAero curve at 1.44 times its OpenRocket export at Mach 0.3. The paper gives no RASAero inputs. Three things weaken
the rise. The subsonic points come from the slow end of the coast, where integrated-velocity drift weighs most, and
the 1,500 ft apogee gap says it drifted, of unknown sign. A flight path below the assumed vertical reads late-coast
C_D high, which would make the true rise larger. The mass error moves every band together.

## 4. Other pairs, and leads that turned out not to be pairs

- **A 130 mm amateur sounding rocket to Mach 2** [S2] p. 17. A 2005 drag-prediction chapter plots coast C_D
  measured on it (credited to Wickman's 1997 book) against the chapter's own component method. Read by pixel
  (±0.015): ratio 1.01–1.07 subsonic, 1.0 at Mach 0.9–1.0, 1.13–1.21 at 1.05–1.2, 1.19–1.27 from 1.3 to 2.0.
  The chapter says its method under-predicts the drag rise and the Mach of peak drag (p. 16), and that this nose lies
  outside the range it was built for (pp. 16–17). How the drag was measured, the motor and the mass are unknown.
- **"OpenRocket off by almost 0.3"** (Blenk and Rupram, AIAA 2025) is CFD, not flight [S3] Table 1, p. 7. At Mach
  0.8 OpenRocket gives 0.513 and the team's CFD 0.275 (airbrakes in); 0.770 against 0.395 (out).
- **PSAS LV2c, N2000W, Mach 1.10** [S15]. The log and an OpenRocket file are public, but the page says its own C_D
  method "does not appear to work very well", and the file lists a different motor. Not counted.
- **Apogee-only evidence.** RASAero's altitude table lists 36 flights [S9]. **Inference:** the 13 above 20,000 ft are
  simulated high in 10 cases, median +6.1%; the 23 lower flights in 15 cases, median +2.8%. Coast fits on the same
  kind of rocket sit at 0.92–1.08 of RASAero's curve, so drag can carry a few percent of that gap; the 2024 J1026 and
  the M2020 point to boost and finish for the rest. RASAero scales thrust with ambient pressure; its author says
  OpenRocket and RockSim don't [S11] #10, #13. His N1048 example integrates to 16,300 N·s at altitude against 14,138
  N·s certified (**Inference:** +15.3%). One flyer reports OpenRocket reading high drag on transonic flights, without
  a C_D or a log [S13] #7; another found it within 3–10% of his flights up to Mach 2 [S11] #1.

## 5. Does the ratio move through Mach 1?

- **Against RASAero II: little in level, clearly in shape.** Three rockets with both sides step by 0 to +0.09.
  The rise ends 0.05–0.08 in Mach earlier in flight than in the code, so a ratio taken near Mach 1 swings ±8%.
- **Against OpenRocket: one flight says yes, by about +45%**, from an unweighed mass and a drifting velocity.
  Against a semi-empirical method, one flight of unknown method says +15–20%.
- **The finish setting is the larger term.** Smooth against rough camouflage moved one Mach 2.8 flight's code curve
  by 15–40% [S6] #1. RASAero's author reports single-stage Mach 3+ flights and the AeroPac 104K two-stage
  matching rough camouflage, and Three Carb Yen and MESOS smooth paint; on MESOS the switch is worth about 10% of
  apogee [S10] #9, #12.
- **What it means for HPR Sim.** No pair is against HPR Sim. Its drag is compared with RASAero II curves through Mach
  2, code to code ([ADR-029](../decisions/0029-drag-against-rasaero-ii-through-mach-2-the-gap.md)). Its formulas read
  19–38% high through Mach 0.8–1.2 on the most HPR-like tunnel set, peaking at Mach 1.08 where flight peaks at
  1.04–1.05 ([transonic-drag-rise](transonic-drag-rise.md)). That fits the rise-position shift found here.

## What this suggests

1. **M7.5's airframe-drag correction above Mach 0.8.** *Suggestion:* one airframe scale per rocket, plus a shift of
   the drag rise's Mach, not a supersonic scale. Widen the stated uncertainty from Mach 0.9 to 1.1. *Expected
   effect:* on the RASAero side, removes the ±8% shape swing near Mach 1; the level stays within 0.1. The RMIT
   case's +45% stays an open risk until a weighed pair confirms or refutes it.
2. **ADR-029's comparison through Mach 1.** *Suggestion:* compare the Mach where the rise ends and where it peaks,
   not only the level. Two flights put the plateau at 0.98–1.00.
3. **Fit a public transonic coast against OpenRocket and HPR Sim.** Harvey Mudd's course page publishes Raven and
   AIM XTRA logs of one 38 mm rocket on a J825R and a J354, with an OpenRocket file [S4]. *Inference from the logs:*
   peaks of 1,222 ft/s (Mach 1.05–1.10) and 1,019 ft/s (Mach ≈0.90). No license is stated; a fit stays private,
   only its ratio published. *Expected effect:* the first transonic coast pair against OpenRocket with a design file.
4. **Start supersonic coast fits after residual thrust.** On the J510, the 2024 J1026 and the K627, C_D dips for
   0.1 to 0.7 in Mach after the logged burnout [S5] #87, #123.

## Open questions

- Does the RMIT rise survive a weighed mass and a baro-fused velocity? Its RASAero inputs, and why that curve
  sits 1.5 times OpenRocket's, are unknown.
- The best public set for a weighed supersonic fit, the 2024 J1026 logs and their OpenRocket file [S5] #124–#141,
  sits behind the forum's sign-in, with no terms stated. It was not fetched.
- How Wickman measured the 130 mm rocket's drag, and its motor and mass: the book wasn't reached.
- Stratos II+'s flight-data paper (IAC-18) was not reached. Teams' later competition reports (25–100 MB) weren't
  searched inside.
- Whether the shape shift holds on a finned rocket fatter than 38 mm: every RASAero transonic reading here is one
  airframe line.

## Sources

Read 2026-10-18; copies in the gitignored `refs/research/supersonic-coast-cd/`. Terms reserved or unstated; cited.

- **S1.** Bawa and Michalski, "An Experimental Framework for In-Flight Determination of Rocket Aerodynamic Drag",
  AIAA 2025-104785 (Regional Student Conferences, 2025); preprint in <http://ftp.demec.ufpr.br/foguete/bibliografia/>.
- **S2.** "Drag Coefficient Prediction, Chapter 1" (2005, no author named), measured data from J. H. Wickman, *How to
  Make Amateur Rockets* (1997). <http://www.braeunig.us/space/pdf/Drag_Coefficient_Prediction.pdf>
- **S3.** Blenk and Rupram, "Aerodynamic Analysis of a High-Powered, Multi-Stage Rocket", AIAA 2025-99705.
- **S4.** Harvey Mudd College, advanced rocketry course flight data.
  <https://pages.hmc.edu/spjut/AdvRoc/FlightData.md.html>
- **S5.** TRF thread 181367, "J record attempt using Loki J1026", posts #9, #71, #87, #110, #122–#130, #141.
  <https://www.rocketryforum.com/threads/j-record-attempt-using-loki-j1026.181367/>
- **S6.** TRF thread 181857, "Leave No Doubt - an M2020 to Mach 3 ish", post #1.
  <https://www.rocketryforum.com/threads/leave-no-doubt-an-m2020-to-mach-3-ish.181857/>
- **S7.** RASAero II comparison with flight data, Proteus 6. <http://www.rasaero.com/comp-flight-Proteus6.htm>
- **S8.** Tripoli Rocketry Association, record "Single Stage J 24750", 2024-06-02.
  <https://www.tripoli.org/content.aspx?page_id=1537&club_id=795696&item_id=502586>
- **S9.** RASAero II comparisons with altitude data. <http://www.rasaero.com/comparisons-alt.htm>
- **S10.** TRF thread 176970, "MESOS Flight to 293K ft", posts #9, #12.
  <https://www.rocketryforum.com/threads/mesos-flight-to-293k-ft-rasaero-ii-prediction-290k-ft.176970/>
- **S11.** TRF thread 194463, "Large RASAero-OpenRocket Apogee Discrepancies?", posts #1, #10, #13.
  <https://www.rocketryforum.com/threads/large-rasaero-openrocket-apogee-discrepancies.194463/>
- **S12.** TRF thread 194818, "Blue Raven Flight Data -- The Anatomy of a Chuff and a Bounce on the Rail", #1, #3.
  <https://www.rocketryforum.com/threads/blue-raven-flight-data-the-anatomy-of-a-chuff-and-a-bounce-on-the-rail.194818/>
- **S13.** TRF thread 189301, "OpenRocket underestimating real world performance", post #7.
  <https://www.rocketryforum.com/threads/openrocket-underestimating-real-world-performance.189301/>
- **S14.** RASAero II comparisons with flight data, Violent Agreement and Violent Agreement SS
  ([flight-derived-cd](flight-derived-cd.md) S1, S2). <http://www.rasaero.com/comp-flight-ViolentAgreement.htm>
- **S15.** Portland State Aerospace Society, LV2c launch data, 2010-06-27.
  <https://archive.psas.pdx.edu/lv2c_launchdata-2010-06-27/>
