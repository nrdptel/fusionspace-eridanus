# Field tools: checklists, equipment and ground tests

**Bottom line:** three of Neer's 2026-10-07 field-tool ideas fill gaps no other tool fills: a
checklist built from the rocket's own configuration, a check of each altimeter's settings
against the simulated flight, and one frequency board across vendors. Launch-day apps today hold
static, hand-edited lists. Vendor apps each manage only their own boards. Clubs coordinate
frequencies on a whiteboard. For ejection testing, one primary source has measured airframe
pressure (Fetter, 2025). It shows the usual calculator formula under-predicts an empty bay's
peak about fourfold (a 7-shot mean), while in one shot a packed parachute cut the peak eightfold
and a packed bay needed several times the calculated charge to separate. A sizing tool can err
on the flattering side in either direction, so the measured outcomes must drive it. Nothing here is
built yet; where each piece goes is in [ADR-194, the field tools][adr-194]. Researched October 7,
2026, from product pages, manuals and papers only (the clean-room rules are at the end).

## What exists

| tool | what it does | what it lacks |
|---|---|---|
| [HPR Flyer Ready](https://www.rocketryforum.com/threads/i-made-a-simple-launch-checklist-mobile-app.175899/) (iOS, Android; free; 2022) | one editable checklist | per-rocket lists were planned, never built |
| [Rocket Logs](https://www.rocketryforum.com/threads/rocket-logs-a-rocketry-companion-app.194729/) (iOS, Android; about $8; closed; 2025) | fleet, flight log, charge calculator, editable checklist, inventory; XLSX and JSON export | no motor data, no altimeter settings, hand-written checklist |
| [FlightCardReader](https://github.com/cabbey/FlightCardReader) (Python, self-hosted) | reads photographed paper flight cards into a database | after the fact; doesn't help prepare |
| [Launch Sentinel](https://www.rocketryforum.com/threads/launch-sentinel-%E2%80%93-live-rocket-tracking-and-channel-monitoring-currently-enabled-for-featherweight-trackers.193342/page-3) (Windows; 2025) | live map; warns when two trackers share a channel | Featherweight only; maps need internet |
| [LDRS 42's frequency sheet](https://www.rocketryforum.com/threads/ldrs-42-gps-frequency-tracker-spreadsheet.186090/) (2024) | a shared sheet plus a whiteboard at the RSO table | manual |
| Vendor apps: Featherweight UI, Eggtimer's on-board web page, Altus Metrum's AltosUI, PerfectFlite DataCap, Missile Works, Jolly Logic | each sets, tests and downloads its own boards | none reads another vendor's devices or the planned flight |
| Charge calculators: [Info Central](https://info-central.rocketlabdelta.com/recovery/black-powder-sizing/), [Altimeter Cloud](https://www.altimetercloud.com/tools/bpcharge_calculator/), [Mountain Man Rockets](https://bp.mountainmanrockets.com/) | the first two size by gas law (C·D²·L or PV = nRT); the third follows Fetter's force-first model | most ignore altitude and the packed parachute; none keeps a ground-test record |
| Altimeter test modes (Eggtimer, Altus Metrum, Featherweight) | fire a charge from the board for a ground test | outcome isn't recorded anywhere |

No tool found builds a checklist from the configuration. The nearest check of settings is
Blue Raven's on-board flight simulation, which "verifies that deployment settings work as
expected" (manual, 10 Sep 2025): it flies the board through a flight built from burn
accelerations and durations, with launch angle, density and drag. The gap is fidelity and
reach: one vendor's board, a simplified flight, not the planned flight's Monte Carlo spread. No rocketry battery tracker was found; RC hobby apps
([LipoBuddy](https://lipobuddy.rocks/)) track packs and cycles.

## Checklists and the launch guide

NASA Student Launch requires one: "Each team shall use a launch and safety checklist" (2026
handbook, requirement 5.1), included in the FRR report. It also asks for procedures covering
recovery, payload, electronics, rocket and motor preparation, pad setup, igniter installation,
launch, troubleshooting and post-flight inspection, with safety steps marked
([2026 handbook](https://www.nasa.gov/wp-content/uploads/2025/08/g-677852-2026-sli-handbook-508-aug-7-optimized.pdf),
pp. 31–32). NAR's Level 3 rules (rev. 2025-06-23) require "a pre-launch checklist covering
airframe, electronics, and motor preparation"
([NAR](https://narocket.clubexpress.com/content.aspx?page_id=22&club_id=114127&module_id=673325)).

The order of several steps is a rule, not a habit. Tripoli's Unified Safety Code v1.04 (March
2025, [PDF](http://www.4cornersrocketry.org/Tripoli_Unified_Safety_Code_1.04.pdf)):

- 7-8: the motor igniter is connected only after every other flight electronic is active;
- 13-8: recovery electronics stay inhibited until the rocket is vertical on the pad, and are
  armed before the igniter is connected;
- 13-9: staging is armed after recovery, before the igniter;
- 11-6: above 2,560 N·s, recovery is electronic and doesn't rely on the motor.

So a checklist built from the configuration can do two things a static list can't: drop the
steps a rocket doesn't need (no altimeter steps on motor ejection), and keep the rule-ordered
steps in their order, each step citing its rule. The *guide* is the same steps with the why
beside each, plus a page on the docs site.

## Equipment and frequencies

**Settings that can be read without the vendor's app:**

- Altus Metrum's telemetry has a configuration packet (type 0x04) carrying the apogee delay,
  the main altitude and the callsign, in a spec licensed CC BY-SA 3.0
  ([spec](https://altusmetrum.org/AltOS/doc/telemetry.html)). A receiver can read it over the air.
- Featherweight's Blue Raven prints its current deployment settings over USB serial with the
  documented `<get deploy>` command (manual, 10 Sep 2025). Featherweight's binary `FWT` packets,
  bound for the phone over Bluetooth, "are not intended to be user-accessible" (GPS tracker
  manual, 20 Feb 2025).
- PerfectFlite's StratoLoggerCF and Missile Works' RRC3 beep their main altitude and battery
  voltage at power-up, so a user can type them in.
- Eggfinder trackers send plain NMEA 0183. Featherweight's ground station writes documented
  `@`-prefixed serial records.

## The settings check

Each setting can fail on more than one side, so each side is its own edge. An edge is compared
with the Monte Carlo flight's one-sided 95%/95% Wilks bound on the side that makes it fail, runs
and failed runs printed. hpr reports where the setting sits against each edge, never a verdict.

| setting | fails when it is too… | compared with |
|---|---|---|
| main altitude | high: it fires at or near apogee | the lowest apogee (low-side bound) |
| main altitude | low: the main isn't open before landing | the height lost while the main inflates at the fastest drogue descent (high-side bound), plus the altimeter's rated error |
| apogee delay (after detection) | long: the drogue opens at speed | the descent speed after the delay (high-side bound), against the drogue's rated opening speed |
| Mach lockout | short: a baro reading fooled near Mach fires a charge | the latest time above the lockout Mach (high-side bound) |
| Mach lockout | long: it still holds at apogee | the earliest apogee (low-side bound) |
| motor delay | short: the charge fires while climbing fast | the latest apogee, against the delay less its rated tolerance |
| motor delay | long: it fires well after apogee | the earliest apogee, against the delay plus its rated tolerance |
| backup offset | none: a rule, not a bound | the backup fires after the primary, and its main stays above the main's low edge |

Vendors define a lockout differently (a time, or a speed from integrated acceleration), so each
lockout check reads the vendor's own definition.

**Frequencies.** Featherweight offers 28 frequencies with two channels each, 56 in all (GPS
tracker manual, 20 Feb 2025). Other trackers have their own channel plans; 70 cm ones are set in kHz.
Big launches assign them on a whiteboard and a shared sheet, and Spaceport America Cup's mission
control assigns every team's
([2025 IREC DTEG](https://www.soundingrocket.org/uploads/9/0/6/4/9064598/2025-irec_dteg_v1.1.4_02-01-25.pdf)).
The license rules differ by band, as of October 2026 (not legal advice):

- 420–450 MHz (70 cm) needs an amateur license; the callsign is sent at least every 10 minutes
  ([47 CFR 97.119](https://www.law.cornell.edu/cfr/text/47/97.119)).
- 902–928 MHz is license-free for certified devices under
  [47 CFR 15.247](https://www.law.cornell.edu/cfr/text/47/15.247).
- 433 MHz license-free use forbids regular periodic transmissions except short, widely spaced
  bursts ([47 CFR 15.231](https://www.law.cornell.edu/cfr/text/47/15.231)); a tracker sending
  every second likely needs Part 97 (unverified inference).

**Batteries.** Draws are on the datasheets: Featherweight's GPS runs about 4.5 h on 400 mAh, and
an RRC3 draws about 6 mA but up to about 35 mA while it beeps and flashes on the pad (vendor
manuals), so the pad state, not the nominal draw, sets the budget. A pad-hold budget that
overestimates runtime is the flattering side.

## Ejection testing

**Fetter's measurements** ("Using Black Powder for Parachute Deployment", vNARCON-2025, Rev 1.2,
23 Feb 2025, [PDF](http://www.speedmotionrockets.com/Using%20Black%20Powder%20for%20Parachute%20Deployment%20-%20Rev1_2.pdf)):

- **The rig:** a 4 in × 12 in PVC chamber (150.8 in³), a 100 psi transducer and an Arduino logger
  writing to SD; Goex FFFFg powder.
- **Empty chamber, 0.5 g, sea level:** a peak of 28.5 psi averaged over 7 shots, standard
  deviation 4 psi (14%). The usual gas-law sizing predicts 6.629 psi; Fetter's model, which adds
  the reaction heat to the air already in the bay, predicts 23.75 psi. Once cooled: 1.2 psi
  measured, 1.062 psi predicted.
- **With a parachute stand-in** (a Nomex-wrapped cotton shirt) filling the chamber: a peak of
  3.58 psi, one shot. In his TR-1 fixture (2.15 in × 9 in, two 2-56 pins), 0.15 g ejected an
  empty nose cone very energetically; packed, 0.3 g to 0.7 g failed, 1.0 g separated cleanly
  and 1.3 g very energetically, "significantly more black powder than either of the BP models
  predict" (the usual calculator gives 0.28 g). His VTS-1 fixture (3 in × 15 in, two
  parachutes) first separated both at 1.6 g.
- **Margins:** about 40% above the charge that just sheared the pins gave an energetic
  deployment on both fixtures. An unreinforced phenolic tube failed between 3 g and 4 g, 1.5 to
  2 times the 1.9 g energetic charge.
- **At 3.3 psia** (about 39,000 ft by Fetter; about 36,000 ft in the standard atmosphere), peaks
  fell below even his model's prediction: 10.2 psi against 20.1 empty, 0.9 against 2.20 packed.
- **Shot to shot:** the seven empty-bay peaks ran from 23.14 to 36.64 psi.

**Jarvis** ([high-altitude deployment, 2013](https://wikis.mit.edu/confluence/download/attachments/120175068/Jim_Jarvis_Highaltitude_deployment_2013.pdf?version=1&modificationDate=1491853101000&api=v2)):
latex-wrapped charges in a vacuum chamber burned about 10% to 20% of their powder; better-contained
charges burned 50% to 100%.

**What this means for hpr's sizing ([M6.7][m6-7]):**

- Subtracting the parachute's volume from the bay *raises* the predicted pressure, but the
  measurement *lowers* it. That correction would err on the flattering side; hpr doesn't ship it.
- No pressure alone sizes a packed bay: two 0.5 g shots in the same tube, with the parachute
  packed in the front half or the back half, separated once and failed once. A packed bay is
  sized to the measured outcomes, at least the charges that separated his fixtures.
- Two outputs need two sides: separation needs the low side, the bulkhead load the high
  (empty-bay) side, his largest shot rather than the mean.
- The peak falls well before 20,000 ft: between his two ambient pressures it fell from 28.5 to
  10.2 psi empty (2.8 times) and from 3.58 to 0.9 psi packed (4.0 times). hpr grows a packed
  charge by the packed fall at the deployment's altitude, taken as linear in ambient pressure
  between his two points (an interpolation, not a model), and never lowers the bulkhead load.
- One shot varies by 14% (1σ), so one clean separation at the minimum charge is no margin.
- A sea-level ground test overstates a charge at altitude. Above 20,000 ft, where Fetter reports
  the community's experience of incomplete burns, hpr prints no black-powder size; it names
  sealed canisters (Jarvis) or CO₂ instead.
- Fetter's ordinal outcome scale (none, partial, clean, energetic, very energetic, with video)
  is the best record format found. Most flyers record only pass or fail.

**Hardware.** Ground tests are fired from a launch controller or an altimeter's test mode. No
open pressure-logger design or dataset was found besides Fetter's tables. The product system
already sets the firing rules: a physical switch, a ground-test mode that times out, a typed or
held confirmation, and a 10 s countdown that SAFE cancels (`refs/fusionspace-design/product/embedded.md`).
E-matches have a rated maximum test current (0.04 A for MJG's Firewire,
[maker](https://electricmatch.com/pyrotechnics/see/6/5/mjg-firewire-initiator--standard)), and
some launch controllers test above it.

**Regulation, as of October 2026 (not legal advice):** ATF's 2006 final rule declined an exemption
of up to two pounds of black powder "for use in model rocket ejection systems"
([Federal Register](https://www.federalregister.gov/documents/2006/08/11/06-6862/commerce-in-explosives-hobby-rocket-motors-2004r-7p)).
A 2026-09-25 notice, "Implementing the Safe Explosives Act", was not read; check it before any
regulatory text ships. NFPA 1122 and 1127 (2026 editions) are paywalled.

## Clean room

- Altus Metrum's AltOS and AltosUI are GPL: use only its manual and the CC BY-SA telemetry spec.
- `blue-raven-cli` on GitHub is AGPL: not read, not to be read.
- Fetter's paper is all rights reserved: cite its equations and numbers, quote nothing longer
  than a phrase, copy no schematic. Mountain Man Rockets' calculator has no stated license: port
  nothing.
- Featherweight's Bluetooth protocol is undocumented, and the vendor says not to use it: go
  through its documented serial commands and records.

[adr-194]: ../decisions/0194-the-2026-10-07-field-tools-checklists-equipment-ground-tests.md
[m6-7]: ../decisions-and-roadmap.md#m6-7
