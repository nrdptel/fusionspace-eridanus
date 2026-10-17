# Field tools: checklists, equipment and ground tests

Re-checked 2026-10-17: 20 claims corrected.

**Bottom line:** three of Neer's 2026-10-07 field-tool ideas fill gaps no other tool fills: a checklist built from
the rocket's own configuration, a check of each altimeter's settings against the simulated flight, and one
frequency board across vendors. Launch-day apps today hold static, hand-edited lists. Vendor apps each manage only
their own boards. Clubs coordinate frequencies on a whiteboard. For ejection testing, one primary source has
measured ejection pressure, in a test chamber (Fetter, 2025). It shows the usual calculator formula under-predicts
an empty bay's peak about fourfold (a 7-shot mean), while in one shot a packed parachute cut the peak eightfold
and a packed bay needed several times the calculated charge to separate. A sizing tool can err on the flattering
side in either direction, so the measured outcomes must drive it. Nothing here is built yet; where each piece goes
is in [ADR-194, the field tools][adr-194]. Researched October 7, 2026, from product pages, manuals and papers.

## What exists

| tool | what it does; what it lacks |
|---|---|
| [HPR Flyer Ready][flyer] (phones; free; 2022) | one editable checklist; more lists considered, none found since |
| [Rocket Logs][logs] (phones; $7.99; 2025) | fleet, log, charges, checklist, XLSX/JSON; no motor or altimeter data |
| [FlightCardReader][fcr] (Python) | reads photographed paper flight cards; after the fact |
| [Launch Sentinel][sentinel] (Windows; 2025) | live map (needs internet), shared-channel warning; Featherweight only |
| [LDRS 42's frequency sheet][ldrs] (2024) | a shared sheet and a whiteboard at the RSO table; manual |
| Vendor apps (Featherweight, Eggtimer, AltosUI, DataCap, Missile Works, Jolly Logic) | own boards only |
| Altimeter test modes (Eggtimer, Altus Metrum, Featherweight) | fire a ground-test charge; outcome not recorded |

Of the charge calculators, [Info Central][ic] and [Altimeter Cloud][ac] size by gas law (C·D²·L or PV = nRT), and
[Mountain Man][mm] follows Fetter's force-first model. Most ignore altitude and the packed parachute; none keeps
the user's test log (Mountain Man's form sends a result to the vendor).

No tool found builds a checklist from the configuration. The nearest check of settings is Blue Raven's on-board
flight simulation, which "verifies that deployment settings work as expected" (manual, 12 May 2026): it flies the
board through a flight built from burn accelerations and durations, with launch angle, density and drag. The gap
is fidelity and reach: one vendor's board, a simplified flight, not the planned flight's Monte Carlo spread. No
rocketry battery tracker was found; RC hobby apps such as Lipo-Buddy (Google Play) track packs.

## Checklists and the launch guide

NASA Student Launch requires one: "Each team shall use a launch and safety checklist" (2026 handbook, requirement
5.1), included in the FRR report. It also asks for procedures covering recovery, payload, electronics, rocket and
motor preparation, pad setup, igniter installation, launch, troubleshooting and post-flight inspection, with
safety steps marked (2026, pp. 31–32; the [2027 handbook][sli-2027] keeps both, pp. 33–34). NAR's Level 3 rules
(rev. 2025-06-23) require "a pre-launch checklist covering airframe, electronics, and motor preparation"
([NAR][nar-l3]).

The order of several steps is a rule, not a habit. Tripoli's Unified Safety Code, Version 2 (effective January
2026, [tripoli.org](https://www.tripoli.org/)), keeps v1.04's numbers and wording for these four:
- 7-8: the motor igniter is connected only after every other flight electronic is active;
- 13-8: recovery electronics stay inhibited until the rocket is raised to launch position, and are armed before
  the igniter is connected;
- 13-9: staging is armed after recovery, before the igniter;
- 11-6: above 2,560 N·s, recovery is electronic and doesn't rely on the motor.

So a checklist built from the configuration can do two things a static list can't: drop the steps a rocket doesn't
need (no altimeter steps on motor ejection), and keep the rule-ordered steps in their order, each step citing its
rule. The *guide* is the same steps with the why beside each, plus a page on the docs site.

## Equipment and frequencies

**Settings that can be read without the vendor's app:**
- Altus Metrum's telemetry has a configuration packet (type 0x04) carrying the apogee delay, the main altitude and
  the callsign, in a spec it licenses as "Creative Commons ShareAlike 3.0"
  ([spec](https://altusmetrum.org/AltOS/doc/telemetry.html)). A receiver can read it over the air.
- Featherweight's Blue Raven prints its current deployment settings over USB serial with the documented `<get
  deploy>` command (manual, 12 May 2026). Featherweight's binary `FWT` packets, bound for the phone over
  Bluetooth, "are not intended to be user-accessible" (GPS tracker manual, 20 Feb 2025).
- PerfectFlite's StratoLoggerCF beeps its main altitude and battery voltage at power-up by default; Missile Works'
  RRC3 does so only when its Audio Options turn them on (manual v1.71). A user can type them in.
- Eggfinder trackers send plain NMEA 0183; Featherweight's ground station writes documented `@`-prefixed records.

## The settings check

Each setting can fail on more than one side, so each side is its own edge. An edge is compared with the Monte
Carlo flight's one-sided 95%/95% Wilks bound on the side that makes it fail, runs and failed runs printed. hpr
reports where the setting sits against each edge, never a verdict.

| setting | fails when it is too… | compared with |
|---|---|---|
| main altitude | high: it fires at or near apogee | the lowest apogee (low-side bound) |
| main altitude | low: not open by landing | height lost inflating at top drogue speed (high), plus baro error |
| apogee delay (after detection) | long: drogue opens fast | speed after the delay (high-side), vs the drogue's rating |
| Mach lockout | short: a baro fooled near Mach fires | the latest time above the lockout Mach (high-side bound) |
| Mach lockout | long: it still holds at apogee | the earliest apogee (low-side bound) |
| motor delay | short: the charge fires while climbing fast | the latest apogee, less the delay's tolerance |
| motor delay | long: it fires well after apogee | the earliest apogee, plus the delay's tolerance |
| backup offset | none: a rule, not a bound | backup fires after primary; its main stays above the low edge |

Vendors define a lockout differently (a time, or a speed from integrated acceleration), so each lockout check
reads the vendor's own definition.

**Frequencies.** Featherweight offers 28 frequencies with two channels each, 56 in all (GPS manual); other
trackers have their own plans. Big launches assign them on a whiteboard and a shared sheet; Spaceport America
Cup's mission control assigns every team's ([2025 IREC DTEG][dteg] §4.2). License rules by band, as of October
2026 (not legal advice):
- 420–450 MHz: ham license, callsign at least each 10 min
  ([97.119](https://www.law.cornell.edu/cfr/text/47/97.119)).
- 902–928 MHz: license-free for certified devices ([47 CFR
  15.247](https://www.law.cornell.edu/cfr/text/47/15.247)).
- 433 MHz license-free use forbids regular periodic transmissions except short, widely spaced bursts ([47 CFR
  15.231](https://www.law.cornell.edu/cfr/text/47/15.231)); a tracker sending every second likely needs Part 97
  (unverified inference).

**Batteries.** Draws are on the datasheets: Featherweight's GPS manual gives about 4.5 h on 400 mAh (its product
page now claims 16 h, mode unstated), and an RRC3 draws about 6 mA but up to about 35 mA while it beeps and
flashes on the pad (vendor manuals), so the pad state, not the nominal draw, sets the budget. A pad-hold budget
that overestimates runtime is the flattering side.

## Ejection testing

**Fetter's measurements** ("Using Black Powder for Parachute Deployment", vNARCON-2025, Rev 1.2, 23 Feb 2025,
[PDF][fetter]); pages are the paper's printed numbers:
- **The rig:** a 4 in × 12 in PVC chamber (150.8 in³), a 100 psi transducer and an Arduino logger writing to SD;
  Goex FFFF powder measured by volume.
- **Empty chamber, 0.5 g in a 2 cc vial, sea level:** Fig. 7-5 (p. 24) prints a 7-shot mean of 28.49 psi, standard
  deviation 4.04 psi (14%). Its seven peaks (23.14 to 36.64 psi) average 28.76 psi; 4.04 is their population
  deviation, the sample one 4.36 psi (15%). Gas-law sizing predicts 6.629 psi; Fetter's model, adding the reaction
  heat to the bay's air, 23.75 psi. Once cooled (another shot): 1.2 psi measured, 1.062 predicted.
- **With a parachute stand-in** (a Nomex-wrapped cotton shirt) filling the chamber: a peak of 3.58 psi, one shot.
  In his TR-1 fixture (2.15 in × 9 in, two 2-56 pins), 0.15 g ejected an empty nose cone very energetically;
  packed, 0.3 g to 0.7 g failed, 1.0 g separated cleanly and 1.3 g very energetically, "significantly more black
  powder than either of the BP models predict" (the usual calculator gives 0.28 g). His VTS-1 fixture (3 in × 15
  in, two parachutes) put out one parachute at 1.2 g and both at 1.6 g; 1.9 g was very energetic.
- **Margins:** from the charge that first sheared the pins to an energetic one, Fetter says "around 40%" (§16, p.
  69); per fixture it is 30% (TR-1, 1.0 to 1.3 g) and 58% (VTS-1, 1.2 to 1.9 g, Table 16-1, p. 77), or 19% from
  1.6 g as his text counts. An unreinforced phenolic tube survived 3.0 g and failed at 4.0 g (p. 67), about 2
  times 1.9 g.
- **At 3.3 psia** (about 39,000 ft by Fetter; 36,000 ft in the standard atmosphere), in a 3 in printed canister,
  peaks fell below his model: 10.2 psi against 20.1 empty; packed, against 2.20, the one shot reads 0.9 psi in
  Table 12-1 (p. 56), 0.8 in Fig. 11-8's caption (p. 55) and 0.68 in his log (Appendix 6, file 75, p. 87). Three
  empty shots there in the 2 cc vial burned slowly, peaking at only 1.32 to 3.97 psi (files 61 to 63).

**Jarvis** ([high-altitude deployment, 2013][jarvis]): charges in latex tubing in a vacuum chamber burned about
10% to 20% of their powder; charges in short brass tubes about 50%, in long brass tubes and tee fittings about
100%. He found sealing unneeded for a full burn but recommends it, so the e-match and first burn are surer.

**What this means for hpr's sizing ([M6.7][m6-7]):**
- Subtracting the parachute's volume from the bay *raises* the predicted pressure, but the measurement *lowers*
  it. That correction would err on the flattering side; hpr doesn't ship it.
- No pressure alone sizes a packed bay: two 0.5 g shots in the same tube, with the parachute packed in the front
  half or the back half, separated once and failed once. A packed bay is sized to the measured outcomes, at least
  the charges that separated his fixtures.
- Two outputs need two sides: separation needs the low side, the bulkhead load the high (empty-bay) side, his
  largest shot rather than the mean.
- Between his two ambient pressures the peak fell from 28.5 to 10.2 psi empty (2.8 times) and from 3.58 psi packed
  to 0.9, 0.8 or 0.68 psi (4.0, 4.5 or 5.3 times). Using 0.9 grows a packed charge least, the flattering side;
  0.68 gives the largest charge. No shot sits between 14.7 and 3.3 psia, so a fall linear in ambient pressure is
  an interpolation, not a model; and the holder changed between the two, which in the empty vacuum shots moved the
  peak 2.6 to 7.7 times (the vial lower). hpr never lowers the bulkhead load with altitude.
- One shot varies by about 15% (1σ, sample): one clean separation at the minimum is no margin.
- A sea-level ground test overstates a charge at altitude. Above 20,000 ft, where Fetter reports the community's
  experience of incomplete burns, hpr prints no black-powder size; it names long or enclosed charge holders
  (Jarvis) or CO₂ instead.
- Fetter labels outcomes (no, partial, clean, energetic, very energetic, "1 of 2" parachutes, airframe failure)
  with video but defines no scale; ordering them is this note's reading. It is the best record format found; most
  flyers record only pass or fail.

**Hardware.** Ground tests fire from a launch controller or an altimeter's test mode. No open pressure-logger
design or dataset was found besides Fetter's tables. The product system sets the firing rules: a physical switch,
a ground-test mode that times out, a typed or held confirmation, and a 10 s countdown that SAFE cancels (its
`embedded.md`). E-matches have a rated maximum test current (0.04 A for MJG's Firewire, [maker][firewire]).

**Regulation, as of October 2026 (not legal advice):** ATF's 2006 final rule declined an exemption of up to two
pounds of black powder "for use in model rocket ejection systems" ([71 FR 46079][atf-2006]). Its 2026-09-25 final
rule, "Implementing the Safe Explosives Act" (91 FR 61074, effective 2026-10-26), keeps the 62.5 g motor exemption
and again declines one for small amounts of ejection powder. NFPA 1122 and 1127 (2026) are paywalled.

## Open questions

- Dropped: "70 cm trackers are set in kHz" (no source); "per-rocket lists were never built" for HPR Flyer Ready
  (the thread ends with them considered); "closed" for Rocket Logs and "some launch controllers test above an
  e-match's test current" (no source).
- Which of Fetter's three packed vacuum readings is the true peak, and how much of the fall is the holder change
  rather than the ambient pressure: no shot pairs one holder at both pressures packed.

## Clean room

- Altus Metrum's AltOS and AltosUI are GPL: use only its manual and the CC telemetry spec.
- `blue-raven-cli` on GitHub is AGPL: not read, not to be read.
- Fetter's paper is all rights reserved: cite numbers and equations, quote only phrases, copy no schematic.
  Mountain Man's calculator has no stated license: port nothing.
- Featherweight's Bluetooth protocol is undocumented and the vendor says not to use it: use its serial records.

[adr-194]: ../decisions/0194-the-2026-10-07-field-tools-checklists-equipment-ground-tests.md
[m6-7]: ../decisions-and-roadmap.md#m6-7
[flyer]: https://www.rocketryforum.com/threads/i-made-a-simple-launch-checklist-mobile-app.175899/
[logs]: https://www.rocketryforum.com/threads/rocket-logs-a-rocketry-companion-app.194729/
[fcr]: https://github.com/cabbey/FlightCardReader
[sentinel]: https://www.rocketryforum.com/threads/193342/page-3
[ldrs]: https://www.rocketryforum.com/threads/ldrs-42-gps-frequency-tracker-spreadsheet.186090/
[ic]: https://info-central.rocketlabdelta.com/recovery/black-powder-sizing/
[ac]: https://www.altimetercloud.com/tools/bpcharge_calculator/
[mm]: https://bp.mountainmanrockets.com/
[sli-2027]: https://www.nasa.gov/wp-content/uploads/2026/08/g-739882-2027-sli-handbook-508-aug5.pdf
[nar-l3]: https://narocket.clubexpress.com/content.aspx?page_id=22&club_id=114127&module_id=673325
[dteg]: https://www.soundingrocket.org/uploads/9/0/6/4/9064598/2025-irec_dteg_v1.1.4_02-01-25.pdf
[fetter]: http://www.speedmotionrockets.com/Using%20Black%20Powder%20for%20Parachute%20Deployment%20-%20Rev1_2.pdf
[jarvis]: https://wikis.mit.edu/confluence/download/attachments/120175068/Jim_Jarvis_Highaltitude_deployment_2013.pdf
[firewire]: https://electricmatch.com/pyrotechnics/see/6/5/mjg-firewire-initiator--standard
[atf-2006]: https://www.federalregister.gov/d/06-6862
