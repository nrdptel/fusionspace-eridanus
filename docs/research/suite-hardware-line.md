# The hardware line: printing, boards, payloads and active control

**Bottom line:** each piece of hardware Neer listed has a simulation side, which hpr can build and
check now, and a physical side, which waits for rules more than for engineering. The rules order
the physical side. Printable files and camera shrouds carry no radio and no control, so they come
first. Trackers and flight computers need FCC and export answers. Airbrake and canard hardware
needs a ruling on US export control. Drones and planes released from a rocket come last, because
competition rules ban them and drone rules don't fit them. Researched October 6, 2026, as a
companion to [the avionics note](suite-avionics.md); [ADR-191, the 2026-10-06 ideas][adr-191]
records the order. Not legal advice: every legal question below is Neer's to take to a lawyer.
The deeper export research, with the enforcement record and what others do, is in
[the export control note](export-control.md).

The same day, Neer added to the plans printing the designs made in these products, several flight
computer models, several tracking modules, and camera, drone and plane payloads, canards and
airbrakes, among others.

## What the plan already holds

- Airbrakes, simulated, drag only, with a reference apogee controller ([M6.4, airbrakes][m6-4]);
  roll control on tail-fin tabs, then canards, roll only ([M6.5, roll control][m6-5]; ADR-193 and
  [roll and drag control](roll-and-drag-control.md)).
- 3MF, STL and OBJ export of any part ([M8.3a, mesh export][m8-3a]).
- A ground station, a GPS tracker, a flight computer and one board built by Neer
  ([M13.1 to M13.4, avionics][m13-4]), under [ADR-162][adr-162]'s guardrails: no code guides a
  rocket to a point or lifts a receiver's speed limits, and radios stay in license-free bands.
- Camera shrouds, a camera planner, printed-infill mass and released payloads, as ideas in
  [the design ideas](ideas-design-and-physics.md) and [the avionics ideas](ideas-avionics.md).

## Export control: a finding new to this project

**The US Munitions List's exclusion for hobby rockets doesn't cover a rocket with active
controls; whether anything else then covers such a rocket is a lawyer's question.** The list
covers rockets in Category IV(a)(5). Its Note 3 to paragraph (a) excludes NFPA 1122 model and
high-power rockets "made of paper, wood, fiberglass, or plastic containing no substantial metal
parts" and flown on motors "certified for consumer use", and adds: "Such rockets must not
contain active controls (e.g., RF, GPS)" ([22 CFR 121.1](https://www.ecfr.gov/current/title-22/chapter-I/subchapter-M/part-121/section-121.1),
read in the eCFR text of October 1, 2026). The State Department's October 2024 proposed rule keeps
that condition in its new definition of an amateur rocket ([89 FR 84482](https://www.federalregister.gov/documents/2024/10/23/2024-24091)).
An AIAA paper of April 2026 argues that the wording catches parachute-deployment electronics and
GPS tracking too ([AIAA](https://aiaa.org/wp-content/uploads/2026/04/ITAR-Reforms-Paper.pdf)).

Two more things follow:

- The same category lists rocket flight control "specially designed" for IV(a), and thrust
  vector control with no threshold.
- [The avionics note](suite-avionics.md) says published open-source software is outside the
  Commerce Department's rules (EAR, [15 CFR 734.7](https://www.law.cornell.edu/cfr/text/15/734.7)).
  That holds for the EAR. The State Department's rules (ITAR) have their own public-domain test
  (22 CFR 120.34), which doesn't name the internet. Whether publishing airbrake or canard
  *firmware* is an export is a question for a lawyer.

BPS.space sells its thrust-vectoring kit only to US citizens and residents, "due to US export
restrictions" ([Signal](https://joe-barnard-oh4g.squarespace.com/signal)).

**What this means for the plan.** hpr's simulated airbrakes and canards (M6.4, M6.5) are models in
a published simulator, as RocketPy's airbrakes are; they go ahead. Control *firmware* and control
*hardware* wait for Neer's answer to the questions below. The flight computer and tracker
(M13.2 to M13.4) already wait for his FCC and export calls; Note 3 adds a question to that call.

## Club and competition rules

- **Tripoli:** its Safety Code has no rule on guidance, canards or airbrakes (version 1.04, March
  2025, and version 2.0, January 2026). Both allow thrust to weight below 3:1 only with "an active
  stability system" and the RSO's approval. §11-6 asks for electronically controlled recovery above
  2,560 N·s of total installed impulse, that is above K. Version 2.0's §14-3.7 allows a radio-controlled
  rocket-boosted glider up to 45° from vertical if it can be controlled during boost (30°
  otherwise).
- **NAR:** its high-power code says "I will not launch my rocket at targets"
  ([NAR](https://www.nar.org/content.aspx?page_id=22&club_id=114127&module_id=669238)).
- **IREC** (Design, Test and Evaluation Guide 2026, V1.1,
  [ESRA](https://www.esrarocket.org/s/2026-IREC-DTEG-V11.pdf)), the most explicit text:
  - §7.1: control only "for pitch and/or roll stability augmentation, or for aerodynamic braking",
    never "actively guided towards a designated spatial target or GPS location";
  - §7.2.1: stable with the controls inert and neutral;
  - §7.3 and §7.4: surfaces go neutral on an abort, and don't move until burnout or a set
    height;
  - §9.1.3: payloads "may not implement gliders, drones, balloons, or any other UAS".
- **NASA Student Launch 2026:** a drone stays tethered until the range safety officer allows its
  release ([handbook](https://www.nasa.gov/wp-content/uploads/2025/08/g-677852-2026-sli-handbook-508-aug-7-optimized.pdf)).
- **FAA:** Part 101 has no rule on guidance. A drone released in flight falls under the drone
  rules (Part 107 or the recreational exception, and Remote ID), whose line-of-sight and 400 ft
  limits don't fit a release at apogee.

IREC's §7 makes good checks for the simulated airbrakes: stable with the surfaces inert, dormant
until burnout, neutral beyond a tilt limit.

## Radio rules beyond the avionics note

- 47 CFR 15.231(a), the 433 MHz band: "Continuous transmissions … are not permitted", so it
  can't carry a telemetry stream without a license.
- Part 97 forbids amateur traffic with a "pecuniary interest" (§97.113), so a product sold as
  license-free can't depend on the ham bands.
- Advertising a transmitter counts as marketing it, which needs prior authorization (§2.803). A
  pre-certified radio module (§15.212) spares a product its own radio certification.

## Printing designs

**3MF over STL.** 3MF carries units (millimeters by default), several objects and metadata
([3MF core spec](https://github.com/3MFConsortium/spec_core/blob/master/3MF%20Core%20Specification.md));
STL carries no scale. A slicer's own per-object settings are undocumented, so hpr writes plain
3MF.

**What a printed part needs from the simulator:** a heat limit and a layer-direction warning.

| material | heat deflection at 1.80 MPa (°C) | interlayer adhesion (MPa) | tensile yield of a printed bar (MPa) |
|---|---|---|---|
| PLA | 55 | 17 | 51 |
| PETG | 68 | 18 | 47 |
| ASA | 86 | 11 | 42 |
| PC-CF | 106 | 23 | 64 |

These are Prusament's datasheet values, one maker's
([PLA](https://prusament.com/wp-content/uploads/2022/10/PLA_Prusament_TDS_2021_10_EN.pdf),
[PETG](https://prusament.com/wp-content/uploads/2022/10/PETG_Prusament_TDS_2021_10_EN.pdf),
[ASA](https://prusament.com/wp-content/uploads/2022/10/ASA_Prusament_TDS_2022_16_EN.pdf),
[PC-CF](https://prusament.com/wp-content/uploads/2023/04/TDS_Prusament-PCCF_2023_EN.pdf)). The
heat column is ISO 75 at the higher of its two loads; at 0.45 MPa ASA reads 93 °C and PC-CF
114 °C, which would flatter a heat warning. The adhesion column is Prusa's own test and the
tensile column ISO 527-1, so they compare only roughly. PLA softens in a hot car. A fin printed
upright, so that bending pulls its layers apart, is the weak case.

**Flutter of printed fins is unvalidated.** The one wind-tunnel test found was at 10 to 12 m/s
([Materials, 2025](https://pmc.ncbi.nlm.nih.gov/articles/PMC11901434/)). A printed fin's flutter
speed takes a measured or cited shear modulus and is labeled unvalidated. Flutter speed is a flattering-side
output: too high is the worst error.

## Payloads

- **Cameras:** a shroud is light (Apogee's 75 mm hood weighs 4.2 g) and adds drag as a
  protuberance. The forum rule of thumb (drag rises by the shroud's share of frontal area) is a lead
  to check against OpenRocket's pods, not a citable model.
- **Released payloads:** hpr already flies an ejected part under its own chute
  ([M1.11, ejected parts][m1-11]). A glider or a drone needs a glide or powered-flight model after
  release. RocketPy chains separate flights and has no glide model
  ([RocketPy](https://docs.rocketpy.org/en/latest/user/deployable.html)).

## The proposed order

Simulation work, gated only by physics:

1. Airbrakes (M6.4), with IREC §7's checks.
2. 3MF export (M8.3a), then the printed-part warnings.
3. A camera shroud as a protuberance, checked against OpenRocket.
4. Released payloads as chained flights, then a glide model.
5. Roll control (M6.5): tail-fin tabs, then canards, roll only.

Physical products, least regulated first:

1. Printable design files.
2. Camera shroud kits.
3. A receive-only ground station.
4. A family of GPS trackers on a pre-certified radio module.
5. A family of flight computers.
6. Drag-only airbrake and roll-only tab hardware (M13.6), after a ruling; roll canards later.
7. Drone and plane payloads.

**Questions for Neer's lawyer:**

1. Does a rocket with drag-only or roll-only control, or with just a GPS tracker, fall under
   USML IV(a)(5)? Should FusionSpace ask the State Department for a commodity jurisdiction ruling?
2. Is publishing airbrake or canard firmware an ITAR export?
3. Does publishing a design count as marketing it under 47 CFR 2.803?
4. Does selling a tracker abroad need a buyer screen under 15 CFR 744.3?

[adr-162]: ../decisions/0162-the-suite-and-its-releases.md
[adr-191]: ../decisions/0191-the-2026-10-06-ideas-web-tools-watches-repo-layout-hardware.md
[m1-11]: ../decisions-and-roadmap.md#m1-11
[m6-4]: ../decisions-and-roadmap.md#m6-4
[m6-5]: ../decisions-and-roadmap.md#m6-5
[m8-3a]: ../decisions-and-roadmap.md#m8-3a
[m13-4]: ../decisions-and-roadmap.md#m13-4
