# ADR-278: An evidence register: what backs each claim, and an open issue until a real sample does (2026-10-17)

- **Status:** accepted
- **Summary:** The docs say honestly when a reader, model or check rests on invented samples, an assumption or another program alone, but nothing tracks finding the real evidence later: of the 24 distinct gaps the site stated on 2026-10-17, 12 had no issue, roadmap entry or research topic, 8 of them flight-log readers. Milestone M2.10 adds `validation/evidence.toml`, one row per claim with the kind of evidence behind it and the kind wanted, and a check that fails when a page states such a gap without a row, or a row short of a real sample has no open `needs-sample` issue. A report counts the rows by kind on the site, with no combined score.

**Context.** [VALIDATION.md](../VALIDATION.md) names four levels of evidence (analytic and unit
tests, component references, code to code, real flights), and the accuracy page's matrix applies
them to each physics model. Nothing applies them to the flight-log readers, nor to the smaller
assumptions inside a model or a format page. Those live in prose: a page says "checked on invented
files only", "assumed" or "no sample", and whether anyone follows it up depends on whether the
work that wrote it filed something.

A survey of the site on 2026-10-17 found 24 distinct gaps of this kind (one gap is often stated on
several pages). 9 were tracked, 3 in part, and 12 by nothing:

- GPX: built from the schemas and checked against gpxpy on invented files; what a real writer puts
  in `ele` is unknown.
- Code paths no sample reaches: decimal commas, 1904-dated workbooks, a negative Raven `.FIPa`
  temperature, an RTx fix at (0, 0).
- AltOS radio telemetry: 11 packet types not decoded; ground speed and climb rate units unchecked.
- ARTS2: the rate of 50 records a second is assumed; where the file stores it is unknown.
- Layouts that rest on one flight or no export: the AltiMAX `.ALS`, the AIM `.xtra` and raw dump,
  7 of 63 raw downloads, 5 of 6 RRC3 `.rff` files.
- Featherweight summaries: only the maximum and pad altitude checked; other units as written.
- GPS heights: a phone app's altitude unit kept as written; KML heights the least sure.
- `.ork` roll angles: which way an angle turns against OpenRocket is assumed.
- An unstable sustainer's apogee assumes it stays upright; why a rod and a rail launch differ is
  not traced.
- RASAero drag tables: one real export, checked only to Mach 0.75; past its last angle, assumed.
- The direction of the ejection push (ADR-086).
- The motor finder, replayed on stand-in searches (by design: ThrustCurve's license, ADR-145).

The tracked ones (CATS and "Juno" samples, the analyzer's readings, Dryden turbulence, a dropped
booster's flight, powered base drag, an off-axis thrust moment, `.ork` inner tubes, Martin's
flutter formula, the simulation as a whole) show the pattern works when someone files it.

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
   `issue`; and `searched`, a dated list of where a search looked and what it found.
2. *The check.* A `cargo xtask evidence` step, run by the gate and CI, fails when a page states
   that a surface rests on invented samples, an assumption or no sample without naming a row;
   when a row short of `real` has no open issue labelled `needs-sample`, or its issue is closed
   with the kind unchanged; when a sample's hash differs from the cached file; or when a `real`
   row's test skipped. A skipped test is never evidence.
3. *The report.* A committed report counts the rows by kind and surface, and a docs page shows it,
   with no combined score; rows whose last search is over 30 days old are listed as due.
4. *The request.* Each format page's request for a sample links its issue, so a user with a file
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
