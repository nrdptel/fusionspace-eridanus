# ADR-194: Neer's 2026-10-07 field tools: checklists, equipment and ground tests (2026-10-07)

- **Status:** accepted; amends [ADR-162, the suite and its releases][adr-162] §2 (release 0.4's contents) and §3 (the queue), and [ADR-147, a roadmap of open work][adr-147] §6 (its budget)
- **Summary:** Neer's launch-field ideas land in three places. Release 0.4 gains M6.8, a field kit in the library and the CLI: a launch checklist and guide built from the rocket's configuration, a check of each altimeter's settings against the simulated flight, and a ground-test log that M6.7's charge sizing reads. M6.7 is tightened by the one measured source found (Fetter, 2025): a packed bay needed several times the calculated charge to separate, so a packed bay is sized to his measured outcomes, never from the empty-bay pressure; the bulkhead load takes the empty-bay peak's high side; the charge grows with altitude, and above 20,000 ft no black-powder size is printed. After the mobile apps, M9.7 adds an equipment register and a cross-vendor frequency board. M13.7 is a ground-test bench (a pressure logger and a wired firing box), in the public tier. The roadmap's budget rises to 36,000 bytes.

**Context.** On October 7, 2026, Neer decided to add tools for the launch field and for testing, in
software and hardware: a launch checklist and guide; managing radios, altimeters, trackers, payloads
and cameras; and ejection testing. He asked to check first whether they already exist, and to
research and add them if not ([VISION](../VISION.md), that date). The repository plans only pieces:
a flight card in pad day ([M9.4][m9-4]), charge sizing ([M6.7][m6-7]), a ground-test log inside the
rebuilt Charge tool ([M9.6][m9-6]), and a watch checklist in
[the watches note](../research/suite-watches.md). Outside, the research
([the field tools note](../research/suite-field-tools.md)) found:

- **Checklists:** apps keep static, hand-edited lists. None builds one from the rocket's
  configuration, though Student Launch requires a launch and safety checklist (2026 handbook,
  requirement 5.1) and Tripoli's safety code fixes the order of the arming steps (7-8, 13-8, 13-9).
- **Equipment:** each vendor's app manages only its own boards. Blue Raven's on-board simulation
  checks its settings against a simplified flight; nothing checks them against the planned
  flight's spread. Big launches coordinate frequencies on a whiteboard.
- **Ejection testing:** calculators size by gas law, and nobody keeps a structured ground-test
  record. Fetter's 2025 chamber measurements are the one primary source with measured pressure.
  An empty bay peaked about four times above the usual formula (28.5 psi measured against 6.629
  psi predicted, 7 shots, 0.5 g). In one shot, a parachute stand-in filling the bay cut the peak
  to 3.58 psi, and his packed test fixtures needed 1.0 g and 1.6 g to separate cleanly, where the
  usual calculator gives 0.28 g and 0.465 g.

**Decision.**

1. **M6.7 is tightened** with Fetter's measurements, each rule with a test and a mutation probe:
   - a packed bay is sized to separate, never from the empty-bay pressure (subtracting the
     parachute's volume, as some calculators do, raises the prediction where the measurement
     fell), and his two fixtures as packed get at least 1.4 times the charges that separated them;
   - the bulkhead load takes the high side: his model scaled to his largest shot, not the mean;
   - with altitude, a packed charge grows by the packed bay's measured fall in peak pressure
     (4.0 times at 3.3 psia), interpolated in ambient pressure between his sea-level and 3.3 psia
     shots, and the bulkhead load never falls; above 20,000 ft, where black
     powder burns incompletely, no black-powder size is printed.
2. **M6.8, the field kit, joins release 0.4** (the competition kit), queued after M3.6, and
   [M10.4][m10-4] waits for it. It is library code, the CLI and printed pages, so it adds no GUI
   (ADR-162 §3) and no new user-facing surface. Three parts:
   - *the checklist and guide*, built from the design: steps chosen by recovery type, altimeters,
     tracker, staging and charge type; rule-ordered steps kept in order and citing their rule;
     the guide gives the why beside each step and is also a page on the docs site;
   - *the settings check*: main altitude, apogee delay, Mach lockout, backup offset and motor
     delay, each edge against the Monte Carlo flight's one-sided 95%/95% Wilks bound on its
     failing side ([the note's table](../research/suite-field-tools.md#the-settings-check)),
     never a verdict;
   - *the ground-test log*, in Fetter's columns and outcome scale, keyed by bay and packing.
     M6.7's sizing shows the logged spread, and never suggests a charge at or below a logged
     failure, nor under 1.4 times the smallest separation logged (his 40% margin).
   - Every safety rule the checklist cites carries its source date, a stale-after date and "not
     legal advice".
3. **M9.7, equipment and frequencies,** comes after the mobile apps, last in the queue. It holds a
   register of altimeters, trackers, radios, cameras, payloads and batteries, a settings snapshot
   with each flight, a pad-hold battery budget that errs short, and a frequency board across
   vendors. The board's license notes carry a source date, a stale-after date (12 months) and
   "not legal advice". Pad day ([M9.4][m9-4]) shows the checklist with a tap at each step, and a
   watch can mirror it ([ADR-191][adr-191]).
4. **M13.7, the ground-test bench,** is hardware Neer builds: a pressure logger for a sealed
   chamber and a wired firing box under the product system's ground-test rules (a switch, a mode
   that times out, a confirmation, a countdown SAFE cancels). It flies nothing and moves no
   control surface, so it is in the public tier ([ADR-192][adr-192]). Like M13.4 to M13.6, it is
   not queued: it waits for Neer. Its logs feed M6.8's log, giving hpr pressure data of its own.
5. **The ideas backlog** (LATER) gains: a ground station that reads other vendors' documented
   telemetry, such as Altus Metrum's configuration packet, to check settings over the air and
   list busy channels; a wireless firing box after the wired one; camera recording reminders
   and a run-time budget.
6. **Device data goes through documented interfaces only:** published serial commands, file
   formats and telemetry specs. Featherweight's Bluetooth packets are out: the vendor says not to
   use them. GPL and AGPL tools are read for their manuals only.
7. **The roadmap's budget rises** from 31,000 to 36,000 bytes: the three new entries, plus about
   1,000 bytes of headroom so the next split of the milestone in progress doesn't hit the cap. The
   roadmap still holds open work only.

**Alternatives considered.**

- *The whole field kit after 1.0, with pad day.* Rejected: Student Launch teams need the checklist
  in their reports, and the settings check and the log need only the simulator and M6.7, both
  built by release 0.4. The equipment register stays after 1.0, because it is only useful on a
  phone at the field.
- *Subtract the packed parachute's volume, as some calculators do.* Rejected: it raises the
  predicted pressure where Fetter measured a fall, an error on the flattering side.
- *Warn at altitude but still print a size.* Rejected: at 3.3 psia Fetter measured half his
  model's peak empty and under half packed, so a printed sea-level size would flatter.
- *Read vendors' Bluetooth links directly.* Rejected: undocumented, the vendor asks users not to,
  and reverse engineering risks the clean room.
- *A wireless firing box first.* Rejected for now: a wired box is simpler to make safe, and the
  bench needs only one.

**Consequences.**

- [ROADMAP](../ROADMAP.md) gains M6.8, M9.7 and M13.7. M6.7's *done when* is tightened, and
  release 0.4 (M10.4) lists M6.8. The queue moves M10.4 and every later entry down by one, and
  M9.7 is added at the end.
- [VISION](../VISION.md) gains the request and V46 to V48.
- [The ideas backlog](../research/ideas.md) records the LATER items.
- `xtask` raises `ROADMAP_MAX_BYTES` to 36,000.

[adr-147]: 0147-m0-5b-a-roadmap-of-open-work-an-archive-and-a.md
[adr-162]: 0162-the-suite-and-its-releases.md
[adr-191]: 0191-the-2026-10-06-ideas-web-tools-watches-repo-layout-hardware.md
[adr-192]: 0192-the-2026-10-06-follow-up-best-over-first-and-export-control.md
[m6-7]: ../decisions-and-roadmap.md#m6-7
[m9-4]: ../decisions-and-roadmap.md#m9-4
[m9-6]: ../decisions-and-roadmap.md#m9-6
[m10-4]: ../decisions-and-roadmap.md#m10-4
