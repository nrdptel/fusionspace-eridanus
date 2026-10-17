# ADR-278: An evidence register: what backs each claim, and an open issue until a real sample does (2026-10-17)

- **Status:** accepted
- **Summary:** The docs say honestly when a reader, model or check rests on invented samples, an assumption or another program alone, but nothing tracks finding the real evidence later: of the 24 distinct gaps the site stated on 2026-10-17, 12 had no issue, roadmap entry or research topic when surveyed, 7 of them flight-log format readers (#645 has since been filed for GPX). Milestone M2.10 adds `validation/evidence.toml`, one row per claim with the kind of evidence behind it and the kind wanted, and a check that fails when a page states such a gap without a row, or a row short of a real sample has no open `needs-sample` issue. A report counts the rows by kind on the site, with no combined score.

**Context.** [VALIDATION.md](../VALIDATION.md) names four levels of evidence (analytic and unit
tests, component references, code to code, real flights), and the accuracy page's matrix applies
them to each model and reading. Nothing applies them to the flight-log format readers, nor to the
smaller assumptions inside a model or a format page. Those live in prose: a page says "checked on invented
files only", "assumed" or "no sample", and whether anyone follows it up depends on whether the
work that wrote it filed something.

A survey of the site on 2026-10-17 found 24 distinct gaps of this kind (one gap is often stated on
several pages; FL is [the flight-log formats page](../format/flight-logs.md)). 9 were tracked, 3 in
part, and 12 by nothing:

| Id | Gap | Main page | Tracked when surveyed |
|---|---|---|---|
| G1 | GPX: built from the schemas, checked against gpxpy on invented files; what a writer puts in `ele` | FL, GPX | no (#645 since) |
| G2 | Code paths no sample reaches: decimal commas, 1904-dated workbooks, a negative Raven `.FIPa` temperature, an RTx fix at (0, 0) | FL | no |
| G3 | AltOS radio telemetry: 11 packet types not decoded; ground speed and climb rate units unchecked | FL, telemetry | no |
| G4 | AltOS `.eeprom`: TeleMetrum v1 converter assumed, TeleMega v4 IMU not read, formats 5, 9 and 19 refused | FL, AltOS | in part (#604) |
| G5 | ARTS2: 50 records a second assumed; where the file stores its rate | FL, ARTS2 | no |
| G6 | Layouts resting on one flight or no export: AltiMAX `.ALS`, AIM `.xtra` and raw dump, 7 of 63 raw downloads, 5 of 6 RRC3 `.rff` | FL | no |
| G7 | Featherweight summaries: only the maximum and pad altitude checked; other units as written | FL, Featherweight | no |
| G8 | GPS heights: a phone app's altitude unit as written; KML heights the least sure | FL, GPS logs | no |
| G9 | Formats with no sample (CATS, "Juno" and the others listed), refused, not shipped | FL, formats that need samples | yes (research note, #603, #604) |
| G10 | The analyzer's readings: an invented `.pf2` and one real log by hand; withheld for other loggers | [log readings](../physics/log-readings.md) | yes (M7.2, #524) |
| G11 | Reading your own logger: an invented logger and an invented GPS table | the own-logger guide (M7.1d2) | in part (M7.2, #600) |
| G12 | Dryden turbulence, unvalidated for rockets | [turbulence](../physics/turbulence.md) | yes (#39) |
| G13 | A dropped booster's or middle stage's own flight | [staging](../physics/staging.md) | yes (#179, #354) |
| G14 | Base drag with the motor burning | [aero](../physics/aero.md) | yes (#633) |
| G15 | Thrust moment about an off-axis center of gravity, unsized | `crates/hpr-sim/src/issues.rs` | yes (#219) |
| G16 | `.ork` roll angles: which way an angle turns against OpenRocket | [ork](../format/ork.md) | no |
| G17 | An unstable sustainer's apogee assumes it stays upright; rod and rail differ, not traced | [ork](../format/ork.md) | no |
| G18 | `.ork` automatic inner-tube radius and ring bore: unit tests only | [ork](../format/ork.md) | yes (M2.2) |
| G19 | RASAero drag tables: one real export, to Mach 0.75; past its last angle, assumed | [aero](../physics/aero.md) | no |
| G20 | The direction of the ejection push (ADR-086) | [recovery](../physics/recovery.md) | no |
| G21 | Four of seven public altimeters assumed barometric | [accuracy](../accuracy.md) | in part (research note) |
| G22 | Martin's flutter formula, not compared with flutter data | [flutter](../physics/flutter.md) | yes (research topic) |
| G23 | The motor finder, replayed on stand-in searches; a live check is barred by ThrustCurve's license (ADR-145), so this row's kind and wanted kind are both `spec`, and it has no open issue | [motor stock](../motor-stock.md) | no |
| G24 | The simulation as a whole, short of its real-flight targets | [accuracy](../accuracy.md) | yes (M2.6 to M2.9) |

The tracked ones show the pattern works when someone files it.

The rocketry case makes the gap hard to close by waiting. The maintainer does no outreach, so
evidence comes from public sources, the private collection, or files users volunteer. For GPX, a
search on 2026-10-17 found no openly downloadable GPX of a real rocket flight: the dedicated
trackers (Featherweight, AltOS, Eggfinder, Missile Works, BigRedBee) write KML, CSV or raw NMEA,
and GPX comes from phone apps, handheld receivers and mapping tools. That is the kind of finding a
row should keep, so the next search doesn't repeat it.

**Sources.** The modeling and simulation standards ask for exactly this record:

- NASA-STD-7009B (2024) §4.2.6 (b) and (d) record "the validation metrics, referents, and data
  sets" and "the aspects of the M&S not validated"; §4.1.1.8 tracks each problem "from discovery
  through closure"; its validation levels (Table 9) set a real referent apart from a representative
  one, and a model counts as a referent only at the same level.
- NASA-HDBK-7009B (2026) Table 24 grades where a referent came from (none, expert opinion, an
  analogous system, another model, the real system), and §5.6.3.2 sets a wanted level early and
  compares the reached one with it.
- Sandia's Predictive Capability Maturity Model (SAND2007-5948) §5.1 scores each element's assessed
  and required level and shows the shortfall; §5.2 advises against any combined score.
- The FDA's guidance on computational modeling credibility (2023, after ASME V&V 40) §VI.B, Table 2,
  lists the kinds of evidence (code verification, calibration, bench and in-vivo validation,
  plausibility) and says the kinds don't rank strength, so kind and adequacy are separate fields.
- SQLite's requirements matrix ties each documented statement to its evidence and flags evidence
  whose statement changed. GDAL fetches real samples apart from its repository and skips a test
  with a named reason when one is absent. GPSBabel's manual marks untested devices and asks users
  with one to report. FFmpeg's sample suite never replaces or removes a sample.

**Decision.**

1. *The register.* `validation/evidence.toml`, one row per claim, with a generated schema like the
   project's other files. Fields: `id`; `surface` (the reader, model or output); `pages` (the
   anchors that state the claim); `kind`, one of `real` (a file or measurement from the device or
   flight itself), `sibling` (a real file from a related device or firmware), `code-to-code`
   (another program, such as gpxpy or OpenRocket), `spec` (built from a published layout or
   schema), `calibration` (checked against the data it was fitted to), `reasoning` (an argument
   only) or `none`; `wanted` (the kind that would settle it); `not_checked`; `samples` (each real
   sample's SHA-256 and the test that reads it, the file under `refs/` with its license);
   `issue`, which closes only when the row reaches its wanted kind (a wanted kind below `real`
   takes an ADR, as for G23, the one such row now); and `searched`, a dated list of where a search looked and what it found.
2. *The check.* A `cargo xtask evidence` step, run by the gate and CI, fails when a page states
   that a surface rests on invented samples, an assumption or no sample without naming a row;
   when a row short of `real` has no open issue labelled `needs-sample`, or its issue is closed
   with the kind short of its wanted kind; when a `real` row lists no sample with its hash and the
   test that reads it; or, in the gate's `refs` step (the samples live under the gitignored
   `refs/`, so CI checks only the register's structure), when a sample is missing, its hash
   differs, or its test skipped. A skipped test is never evidence. The phrases that state a gap
   are a committed list with a test that each one fires on a canary page, and a fixture of the
   24 surveyed sentences must each be detected; a test pins the phrase list's length and the row ids,
   so removing a phrase or a row fails until an ADR amends this one.
3. *The report.* A committed report counts the rows by kind and surface, and a docs page shows it,
   with no combined score; rows with no search, or whose last search is over 30 days old, are listed as due.
4. *The request.* Each format page's request for a sample (each entry under the flight-log page's
   formats that need samples, and each format section that asks for one) links its issue, so a user with a file
   knows where to attach it. This is the only ask; there is no outreach.
5. *The rule.* Work that ships a surface on less than a real sample adds its row and its issue in
   the same pull request.

**Alternatives.** Issues alone: no link from the page's sentence to its issue, so the two drift
and nothing fails when one is missing (the 12 untracked gaps). Widening the accuracy matrix: it
has one row per model and four fixed columns, and the readers' gaps are per field and per format.
Keeping the prose: the status quo.

**Consequences.** M2.10 goes next in the queue, after M7.1, ahead of M7.2's readings, which lean
on the same readers. The first run of the check will fail on every gap above until each has a row
and an issue; that is the milestone's work. Issue work then takes the `needs-sample` issues
like any other, and a periodic search keeps each row's `searched` list current.
