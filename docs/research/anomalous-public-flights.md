# Public logs of faulty flights

Which public flight logs record a known fault, under terms that let HPR Sim use them? Fault diagnosis (M7.4 in the
[roadmap](../ROADMAP.md)) must analyze at least 2 real anomalous flights if the data exists. For each faulty flight
this note gives the fault and how it is known, the logger and format, whether HPR Sim reads it, the license and
whether a design is public. It builds on [more public flights](public-flights.md),
[AerospaceNU's logs](aerospacenu-flights.md), [UND's flights](und-flight-configs.md),
[sample logs](sample-logs-cats-juno.md) and [the logger families](debrief-log-formats.md). Read 2026-10-17: data
files, READMEs, notebooks, team reports and forum posts only; no GPL source code was opened. **Stated** marks a fault
the source names; **Inference** marks one read from the trace by this note, with the evidence.

**In short.** The data exists: M7.4's two real flights can come from MIT-licensed logs HPR Sim already reads.
- **Four MIT flights with a fault and the inputs to fly them.** Two have the fault **stated**: Astra (EuRoC 2022,
  ballistic) and Juno III (Spaceport America Cup 2023, drogue only), both from RocketPy's data with notebook inputs.
  Two have a public OpenRocket design and a COTS motor, but the fault only from the trace: Hyperion-II (2025, no
  drogue deceleration, about 40 m/s at impact) and UND's IREC 2024 flight (loss of control under thrust).
- **The stated faults with the richest data are cite-only.** NC State's 2025–26 club flights (main at apogee, tangled
  main, a backup altimeter whose charges never fired) come with team text, two readable loggers and `.ork` files, but
  no license. So do two telemetry files linked from forum threads (a chute lost at motor ejection, which the trace
  puts 1.2 s before apogee; a ballistic return from about 6,200 m). The Rocketry Forum's terms grant personal use
  only.
- **Of 74 flights checked in five collections, 20 show a fault in their trace;** forums and course archives gave
  10 more logs. Recovery faults dominate. None of the sources searched holds a usable trace of fin flutter, coning
  or an airstart failure.
- **An official apogee can be an ejection spike.** ESRA's 2024 score sheet gives UND's flight 11,753 ft, the peak
  sample of a 1.1 s pressure swing after the drogue state, 29.7% above the coast peak of 2,761.1 m (**Inference**).

## Usable: the license allows use

All MIT by the repository's `LICENSE` and GitHub's license API. "Reads" names HPR Sim's reader for the file, from
`validation/reports/log-import.json`, `validation/log-mappings.json` and the real-flights report.

Each entry: the fault and how it is known; the logger and format, and HPR Sim's reader; the design.
- **Hyperion-II, 2025-08-21, Cesaroni M2505:** no drogue deceleration, the main at about 440 m, about 40 m/s at impact
  (**Inference**; both loggers show the fall, only the TeleMega reaches the ground). Blue Raven CSV and TeleMega AltOS
  CSV, both read. `.ork` v2 and v3.
- **UND IREC 2024, AeroTech N3300R:** loss of control under thrust; the primary log stops at 1.88 s (**Inference**).
  A TeleMega and a baro-only Altus board (**Inference** from the columns), AltOS CSV, read. `.ork` and `.CDX1`.
- **Astra, EuRoC 2022, Cesaroni L1350:** ballistic descent (**stated** in RocketPy's notebook), about 94 m/s near
  the ground. An unnamed logger's 1 Hz CSV, read as a mapped table. Notebook inputs.
- **Juno III, Spaceport America Cup 2023, the team's motor:** only the drogue ejected (**stated**, README), 24–27 m/s.
  RRC3 and GNSS CSVs, read; its climb is already flown. Notebook inputs.
- **UND Student Launch 2024, probably the payload flight, AeroTech L1420R:** the main fired at about 730 ft, yet
  about 28 m/s to impact (**Inference**); both logged `PWRLOSS` at apogee. 2 × StratoLogger `.pf2`, read. `.ork`.
- **Valkyrie, 2025-07-26, Cesaroni K650:** the main never slowed the descent, 20.0 m/s against about 4.4 m/s
  under the main (**Inference**). A merged altitude CSV, read as a mapped table. Notebook inputs.
- **NC State "Pelicanator", Student Launch 2025, AeroTech L1520T:** slow ignition, 4,226 ft against 4,609 ft predicted
  (**stated**, report), not confirmed by the IMU. Parker-LORD IMU CSV, not read yet (it needs a mapping). No design.

The evidence, one flight each:
- **Hyperion-II** (`werocketry/Hyperion-II@efda132`): the TeleMega enters its drogue state at 24.10 s at 2,852 m,
  then falls at 91 m/s at 34 s and 173.7 m/s at 46.1 s, 2,081 m in 22 s, 88% of a vacuum fall. The Blue Raven
  apogee channel fires at 25.40 s and the TeleMega's drogue channel from 27.2 s; neither slows it. The main fires at
  about 433 m. Air brakes may have flown, which touches the climb, not the descent.
- **UND IREC 2024** (`UND-ARC/IREC-ARC-2024@94a991b`): the primary's tilt goes 6.3° at launch, 23.5° at 1.09 s, 55.8°
  at 1.88 s, with lateral acceleration to about 20 g and roll to about 910 °/s; its log stops still under thrust. The
  backup coasts smoothly to 2,761.1 m, then swings up to 3,582.4 m within 1.1 s, starting 1.6 s after its drogue
  state. The prediction was 29,856 ft. What failed is unstated; the team's flutter sheet put peak speed at 72% of the
  limit.
- **Astra** and **Juno III** (RocketPy `9bd6ad3`, `data/rockets/`): Astra climbs to 3,249 m and falls 520 → 145 m
  in 4 s; its log ends at 145 m. Juno III's GNSS log descends at 24–27 m/s; HPR Sim already flies its climb in
  [the real-flights report](../../validation/reports/real-flights.md). Erebus 11 (MIT) is also **stated** ballistic,
  but its descent rows fall about 3,000 m in about 14 s, three times a vacuum fall: usable only to apogee.
- **UND SL 2024** (`UND-ARC/Nasa-SLI-2024@97bee76`): the secondary's `Main At: 52.90 Seconds` is followed by 27.8 m/s
  to the ground, the same as before it. Which flight it is rests on [UND's flights](und-flight-configs.md).
- **Valkyrie** (RocketPy, `valkyrie/`): 511 m at 98 s to 12 m at 123 s. The notebook's drogue alone gives about
  20.8 m/s, the main about 4.4 m/s (ρ = 1.1 kg/m³, **Inference**); the source says nothing.
- **Pelicanator** (`NCSU-High-Powered-Rocketry-Club/AirbrakesV2@afdcecb`): the boost's integrated acceleration,
  173.6 m/s, matches 170.8 m/s on the same motor's earlier flight. Without both flights' masses, the stated
  impulse loss can't be confirmed. Air brakes flew on the climb.

## Cite-only: a stated fault and a readable log

No license, a GPL-3.0 source or a personal-use forum: cache under `refs/`, cite, publish statistics only. Each
entry: the fault in the source's words; this note's trace reading (**Inference**); the logger; the design.
- **NC State "Jackpot 2", 2026-03-14, AeroTech L1390G:** main at apogee and an air-brake pressure spike (team).
  +1,165 Pa in the boost; 9.6 m/s against 24.6–32.3 m/s under drogue. EasyMini AltOS CSV, Fluctus CSV. `.ork`.
- **NC State "Jackpot 3", 2026-04-04:** the main "deployed with tangled lines" (team). 9.0 m/s under main against
  4.0 m/s on a sister flight. Same loggers. `.ork`.
- **NC State "Jackpot 1", 2026-02-21:** "neither ejection charges on the Fluctus fired" (team). The EasyMini's
  separation spike is +5,469 Pa, 5–12 × the other flights'. Same loggers. `.ork`.
- **Forum A, 2018, J270W with a 15 s delay:** "lost the parachute at the motor ejection charge" (poster). +8.1%
  pressure 1.2 s before apogee, so a delay of about 11.5 s; an 11–19 m/s fall. TeleMega AltOS CSV. No design.
- **Forum B, 2014, CTI K590:** both charges fired and the body didn't separate (thread). About 194 m/s from 2,763 m
  to 436 m. TeleMetrum `.telem`. No design.
- **Cavour, EuRoC 2023, Cesaroni L995:** ballistic (notebook). 73–78 m/s to the last row. CATS telemetry CSV.
- **Lince, EuRoC 2023, Cesaroni M1101:** main at apogee (notebook). A −737 m pressure transient at deployment.
  CATS Vega CSV.
- **Camões, EuRoC 2023, the team's motor:** drogue only (notebook). 26–28 m/s to the ground. CATS Vega CSV.
- **Toledo, Student Launch 2024, an old AeroTech K1000T:** the motor "severely underperformed" (report). Apogee 19%
  below the same vehicle's earlier flight, with the masses differing. Eggtimer CSV. No design.
- **Clemson CURE, IREC 2025:** main at apogee, no drogue, both boards lost power (team text, MIT). The log ends 5 s
  after a 9,904 ft apogee. The team's converted CSV. A design whose match to the flight is unconfirmed.
- **Haybales, AerospaceNU, 2026-02-22:** the board rebooted in flight (file tags). The reset came at the main fire;
  14.6 m/s to the ground. SillyGoose text. No design.
- **Top Shot, DOFPro, 2017, Estes E9-6:** CATO (file name). A 94.5 g spike; never above 7 m. Raven FIP CSV. `.ork`.
- **Mini DX3, DOFPro, 2021, AeroTech G138T:** "shredded at Max-Q" (page). The log ends 1.03 s after liftoff.
  The course's Teensy logger. `.ork`.

Notes:
- **NC State** ([launch catalog](https://ncsurocketry.org/launches/)): air brakes deployed on five of the six 2025–26
  climbs, all three Jackpot faults' among them, so only recovery faults suit a test. Jackpot 4 is the control.
- **Forum A and B** sit on Dropbox and Google Drive, linked from threads (sources F1 and F3). Both hold call signs and
  pad positions, never to be quoted. A also shows a transonic dip: 276 m of false descent at 2.4–3.1 s.
- **Cavour, Lince, Camões** are MIT inside RocketPy but byte-identical to GPL-3.0 files of
  `RocketPy-Team/euroc23-team-data`, or derived from its source `catsystems/euroc23-team-data` ([sample
  logs](sample-logs-cats-juno.md)). HPR Sim already flies Cavour and Lince.
- **DOFPro** says "Creative Commons" with no variant ([public flights](public-flights.md)); a probe of the
  importer reads the Raven file, but gets no altitude track from the Teensy log.

Also found, weaker: a MIT-licensed Blue Raven log of a breakup near Mach 1.4 (**Inference**; WVU's IREC 2026 flight,
the example in `rex-mcall/Shred-O-Vision`; the team's ownership of the data is open), NC State's 2025 first test
flight (MIT; the fault came after the log ended), PSAS LV2c's 2010 fin flutter (video only, about 4° for about 1 s, no
trace signature), four more AerospaceNU traces (GPL-3.0, no team text), and Harvey Mudd's 2008–09 lawn darts (raw
counts with no calibration).

## Text and plots only

- **CATO:** WPI, Student Launch 2019, an L1030 that "blew up around 3 seconds into flight" (plot only).
- **Fin flutter:** a forum J-record attempt, roll rate saturated at 2,292 °/s after flutter at about 1.8 s (plots).
- **Coning:** RASAero II's comparison page cites 3.5° coning from onboard video for one flight; no trace.
- **Late motor ejection:** RocketSmith (arXiv 2606.00097, P1, CC BY 4.0), about 14 s against 10 s; a figure only.
- **Recovery:** LMU's tangled drogue (MIT, plots), a UKSEDS team's ballistic return (MIT, design public, no log),
  Berkeley 2018, Stanford's JARVIS I (data behind a sign-in), Toledo's 2024 swapped deployment wiring.

## Where the faults are, and where they aren't

| source | flights checked | faulty with a trace | clean | other |
|---|---|---|---|---|
| AerospaceNU (GPL-3.0) | 36 | 7 (**Inference** but 1) | 24 | 5 ambiguous or unusable |
| RocketPy's logged flights | 14 | 7 (6 **stated**) | 4 | 3 unexplained descents |
| Team repositories (UND, Hyperion-II, UTRGV) | 11 | 3 | 8 | |
| NC State 2025–26 | 6 | 3 (**stated**) | 3 | |
| Debrief's real samples | 7 | 0 | 5 | 1 ambiguous, 1 not scanned |
| Forums, makers, course archives | (search) | 10 logs, 2 from forums | | 4 threads, 4 sites: text or plots |

Coverage of M7.4's list: early, late and no drogue, main failure, separation to ballistic and ejection-pressure
spikes have several real cases each. A delay mismatch, a CATO, motor underperformance, structural failure and a
Mach barometer error have one to three. Fin flutter, coning and airstart failure have none with a usable trace, and
weathercocking appears only as a remark in DOFPro's text. Makers' manuals, Zenodo, Kaggle, data.world and Figshare
held no failed-flight log.

## Licenses

- **MIT:** `werocketry/Hyperion-II`, `UND-ARC/IREC-ARC-2024`, `UND-ARC/Nasa-SLI-2024`, `RocketPy-Team/RocketPy`
  (Astra, Erebus 11, Valkyrie, Juno III), NC State's `AirbrakesV2`, `rex-mcall/Shred-O-Vision`.
- **GPL-3.0, cite-only:** AerospaceNU's `rocket-flight-data`; the euroc23 tables RocketPy copies.
- **No terms, cite-only:** ncsurocketry.org, Toledo's Drive files, CURE's `Rocket-Test-Data`, PSAS, DOFPro (variant
  unnamed), Harvey Mudd, the off-forum Dropbox and Drive files.
- **The Rocketry Forum** (terms, F9, §2.1, §6.2, §10.1): "limited, personal" use; no data mining; members license
  the site only.

## What this suggests

1. **M7.4's first real cases.** Fly Astra and Juno III (stated faults, MIT, inputs in their notebooks) and
   Hyperion-II and UND IREC 2024 (public `.ork`, COTS motors) as M7.4's real flights. *Done when* each one's log
   digest, fault label and source (stated or inferred) are committed. *Effect:* M7.4's real-flight count 0 → 4.
2. **Read an ejection transient as one.** The importer's peak reads the forum B board's ejection transient
   (+149 m, 2.4%) and the WVU breakup (+40%). M7.4's evidence should mark short transients after a charge, as
   the real-flights report's cut already does for Lince and Prometheus.
3. **Treat ESRA's "actual" apogees with care.** If a survey of ESRA score sheets ever scores flights, read each
   against its log where one exists; UND's 2024 row reads a transient, not the coast peak (**Inference**).
4. **Same-vehicle controls.** NC State's Jackpot 4 and Toledo's earlier flight show the value of a nominal flight of the
   same rocket; M7.4's ranking can use such pairs for motor faults, where the climb alone is ambiguous.

## Open questions

- Hyperion-II: did the drogue separate, and did the air brakes open? No team write-up was found.
- UND IREC 2024: what failed at about 1.2 s? The team's report wasn't read.
- Was forum A's delay really 15 s as flown? Why does forum B's main state appear if nothing separated?
- Pelicanator and Toledo: the flights' masses decide whether the stated motor faults show in the logs.
- Would NC State, WVU or DOFPro license their logs? The project does no outreach, so they stay cite-only.
- Which Mini DX3 design flew on 2021-04-10?

## Sources

- [H1] werocketry/Hyperion-II@efda132, `flight-data/2025-08-21_*`, `DSS.md`, MIT:
  <https://github.com/werocketry/Hyperion-II>.
- [U1] UND-ARC/IREC-ARC-2024@94a991b, `Analysis/IREC_*_Flight_Data.csv`, `Fin_Flutter_Analysis_2.pdf`, MIT:
  <https://github.com/UND-ARC/IREC-ARC-2024>; ESRA 2024 score sheet: <https://www.soundingrocket.org/2024-sa-cup.html>.
- [U2] UND-ARC/Nasa-SLI-2024@97bee76, `Data/*_PDF.pf2`, MIT: <https://github.com/UND-ARC/Nasa-SLI-2024>.
- [R1] RocketPy@9bd6ad3, `data/rockets/` (astra, juno3, valkyrie, erebus11, polito, lince, camoes) and the
  notebooks: <https://github.com/RocketPy-Team/RocketPy>.
- [N1] NC State HPRC launch catalog and CSVs: <https://ncsurocketry.org/launches/>; 2025 PLAR:
  <https://ncsurocketry.org/wp-content/uploads/2025/06/north-carolina-state-university-2025-plar-report.pdf>
  (pp. 1, 8–9); AirbrakesV2@afdcecb, `launch_data/`, MIT:
  <https://github.com/NCSU-High-Powered-Rocketry-Club/AirbrakesV2>.
- [T1] University of Toledo 2024 PLAR, pp. 4–6, 10–11:
  <https://drive.google.com/file/d/1R4YYDarPl5a44CkIbad4gGJkqk-Njnmq>.
- [C1] CURocketEngineering/Avionics@8da505b, `docs/FlightTests.md`, MIT; Rocket-Test-Data@aee4d79, no license:
  <https://github.com/CURocketEngineering>.
- [A1] AerospaceNU `rocket-flight-data@e951b1f`, GPL-3.0: see [AerospaceNU's logs](aerospacenu-flights.md).
- [D1] DOFPro flight data: <https://dofpro.org/RCK/fltdata/FlightData.html>.
- [F1] <https://www.rocketryforum.com/threads/failed-level-2-attempt-but-many-things-learned.146905/>, posts 1, 49.
- [F3] <https://www.rocketryforum.com/threads/i-managed-to-lose-a-rocket-with-gps-telemetry.68671/>, posts 1, 16.
- [F9] The Rocketry Forum, terms: <https://www.rocketryforum.com/help/terms/>.
- [P1] RocketSmith, arXiv 2606.00097 v3, §5.1, Fig. 17: <https://arxiv.org/abs/2606.00097>.
- [P2] PSAS LV2c launch, 2010-06-27: <https://archive.psas.pdx.edu/lv2c_launchdata-2010-06-27/>.
- [W1] WPI 2019 PLAR, §1.4.2, read from the Wayback Machine's 2024 copy of `aiaa.wpi.edu/static/hprc/goats/`.
- [S1] rex-mcall/Shred-O-Vision, `examples/mothman_avenged/`, MIT: <https://github.com/rex-mcall/Shred-O-Vision>.
