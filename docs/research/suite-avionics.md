# Flight computers, GPS trackers and a ground station: research for the avionics line

**Bottom line:** most of the work can be built and checked in software, against hpr's simulated
flights and real logs, before any board exists. Hardware needs the maintainer's hands for building, bench
tests, radio range and flights. The safest order is ground station, then GPS tracker, then flight
computer: the first transmits nothing, the second transmits but fires nothing, the third fires
charges. This note backs the avionics milestones ([M13.1 to M13.4](../ROADMAP.md), [ADR-162, the suite and its
releases][adr-162]) and extends [the avionics ideas](ideas-avionics.md). Researched 2026-10-04;
nothing here is implemented. Not legal advice.

## The landscape and what may be ported

| project | software | hardware | radio |
|---|---|---|---|
| Altus Metrum (TeleMega, EasyMega, TeleGPS) | GPL-2 ([AltOS](https://altusmetrum.org/AltOS/)) | TAPR OHL | 70 cm amateur band, license needed ([radio](https://altusmetrum.org/Radio/)) |
| CATS Vega | GPL-3.0 | GPL-3.0 ([org](https://github.com/catsystems)) | |
| SparkyVT HPR flight computer | MIT ([repo](https://github.com/SparkyVT/HPR-Rocket-Flight-Computer)) | | |
| RocketTalk 5.1 | CC0 ([repo](https://github.com/AllDigital33/RocketTalk-5.1)) | | |
| Eggfinder, Featherweight GPS | closed | closed | 900 MHz ISM ([Eggfinder](https://eggtimerrocketry.com/home/eggfinder-gps-tracking-system/), [Featherweight](https://www.featherweightaltimeters.com/uploads/1/0/9/5/109510427/gps_tracker_manual_2025feb20.pdf)) |

Only SparkyVT's and RocketTalk's code may be ported, with attribution. AltOS and CATS stay off
limits but for their documented formats, as the clean-room rule says.

## Embedded Rust is ready enough

- [Embassy](https://github.com/embassy-rs/embassy) covers STM32, nRF and RP2040/RP2350.
- Driver crates exist under permissive licenses:
  - barometers: [`ms56xx`](https://github.com/Rechenmaschine/ms56xx), [`bmp390-rs`](https://docs.rs/bmp390-rs/latest/bmp390_rs/);
  - IMUs: [`icm42688`](https://crates.io/crates/icm42688);
  - high-g accelerometer: the ADXL375 in [`adxl3xx`](https://docs.rs/adxl3xx/latest/adxl3xx/);
  - GNSS: u-blox in [`ublox`](https://lib.rs/crates/ublox);
  - LoRa: SX126x and SX127x in [`lora-phy`](https://github.com/lora-rs/lora-rs).
- An STM32H7 does f64 in hardware. The RP2350 has a double-precision coprocessor at about half
  that speed ([dmitry.gr](https://dmitry.gr/?r=06.+Thoughts&proj=11.+RP2350)).
- No hpr crate is `no_std` today. The first step is a new no-heap crate holding the estimator,
  the atmosphere and the deployment state machine, shared by the firmware and the simulator.

## Radio and law

- **US, 902 to 928 MHz, unlicensed:** §15.247 needs a 6 dB bandwidth of 500 kHz or more, or
  frequency hopping over 50 or more channels
  ([eCFR](https://www.ecfr.gov/current/title-47/chapter-I/subchapter-A/part-15/subpart-C/subject-group-ECFR2f2e5828339709e/section-15.247)).
  Hopping needs 50 channels when the 20 dB bandwidth is under 250 kHz (25 above it), at most
  0.4 s on each in 20 s (in 10 s for the wider channels), at 1 W (0.25 W with fewer than 50 channels). Common 125 kHz LoRa needs
  hopping. Selling a transmitter needs FCC certification. Five or fewer
  home-built units for personal use are exempt ([§15.23](https://www.law.cornell.edu/cfr/text/47/15.23)).
- **70 cm amateur band:** needs a license.
- **EU, 865 to 868 MHz:** 25 mW and a 1% duty cycle or listen-before-talk
  ([ERC 70-03](https://docdb.cept.org/download/3700)).
- **GNSS limits:** u-blox M10 in its airborne mode stops giving fixes above 80 km or 500 m/s
  ([MAX-M10S manual](https://content.u-blox.com/sites/default/files/MAX-M10S_IntegrationManual_UBX-20053088.pdf)).
  The airborne mode also limits acceleration (under 4 g), which a boost exceeds. A tracker has
  no fix above 500 m/s, about Mach 1.47 at sea level and Mach 1.7 at 11 km, so the design must
  expect a dropout and a resume.
- **Export control:** published open-source software is generally outside the EAR
  ([15 CFR 734.7](https://www.law.cornell.edu/cfr/text/15/734.7)). GNSS receivers built for
  missiles, or giving fixes above 600 m/s, are controlled (ECCN 7A105,
  [Commerce](https://www.space.commerce.gov/wp-content/uploads/2022-03-US-export-controls-GPS-GNSS-equipment.pdf)).
  The project never ships code that lifts the GNSS limits, and never ships guidance to a point.
  Selling hardware needs the maintainer's legal call first. The EAR exclusion is not the whole answer: the
  State Department's US Munitions List excludes hobby rockets only if they carry no active
  controls, "e.g., RF, GPS" ([the hardware line note](suite-hardware-line.md), researched
  2026-10-06).

## Safety

- NAR's high-power code keeps onboard firing circuits inhibited until the pad
  ([NAR](https://nar.org/content.aspx?page_id=22&club_id=114127&module_id=669238)). Tripoli's
  Safety Code §11-6 asks for electronically controlled recovery, not the motor's charge, above
  2,560 N·s of total installed impulse, that is above K (versions 1.04 and 2.0, read 2026-10-06;
  an earlier search snippet said "from K up").
- IREC's 2025 rules are a useful bar
  ([DTEG](https://www.soundingrocket.org/uploads/9/0/6/4/9064598/2025-irec_dteg_v1.1.6_02-14-25.pdf)):
  - redundant recovery electronics must be fully independent (§6.9);
  - one recovery computer must be commercial (§6.10);
  - home-built electronics need ground and flight proof (§6.11);
  - home-built computers may not light air-starts (§5.13).
- Pyro hardware practice: low-side MOSFETs, a separate pyro battery and a bulk capacitor against
  brownout ([Bdale Garbee](https://gag.com/bdale/blog/posts/Batteries_and_Pyro_Circuits.html)).
- The safety outputs are firing, arming, continuity, the Mach lockout and tilt, altitude and
  timer lockouts. The two failures are a premature deployment (a Mach spike, noise or a brownout
  reset) and a missed one (a lockout that never clears, or a sensor dropout). Firmware fails
  toward no air-start but toward deploying eventually (a backup timer).

## Hardware license and tools

CERN-OHL-P v2 is the permissive hardware license that fits MIT or Apache-2.0
([FAQ](https://gitlab.com/ohwr/project/cernohl/-/wikis/faq)); choosing it is Neer's licensing
call. Designs made in KiCad are not GPL ([libraries
exception](https://spdx.org/licenses/KiCad-libraries-exception.html)), and `kicad-cli` runs ERC
and DRC headless in CI ([docs](https://docs.kicad.org/9.0/en/cli/cli.html)). Renode emulates STM32
and nRF chips deterministically in CI
([Memfault](https://interrupt.memfault.com/blog/test-automation-renode)).

## What software can prove, and the gates after it

Checkable without hardware:

1. Deployment and estimator logic as host-tested `no_std` code, fed by hpr's simulated sensor
   streams (with Mach spikes, clipping and GNSS dropout) and by real logs read by the M7.1
   importers; fault-injection Monte Carlo on top.
2. Packet codecs, the tracker's schedule and the ground station against recorded or simulated
   packets.
3. Firmware in Renode against replayed sensors.
4. Schematic ERC, board DRC and the bill of materials.

Then, each with Neer: a development board replaying logs, a bench test of the pyro channels, a
passenger flight beside a commercial primary, and only then a flight as primary.

## First increments

The roadmap holds them, each with its *done when*: [M13.1 to M13.4](../ROADMAP.md). In short, a
packet codec and ground-station decoder first; then the tracker's `no_std` logic, tested against
a simulated flight with the receiver's speed and acceleration limits; then the flight computer's
deployment logic under a fault-injection Monte Carlo whose run count and targets are fixed before
it runs; then hardware, with Neer.

[adr-162]: ../decisions/0162-the-suite-and-its-releases.md
