# ADR-015: The validation harness: cases, references, tolerances and reports (2026-09-17)

- **Status:** accepted
- **Summary:** The validation harness: cases, references, tolerances and reports

**Context.** M2.1 is the first end-to-end milestone: hpr's numbers against other tools' for whole
flights. M2.1a builds the harness the suite runs on. What shapes it is less the plumbing than
Loft's five validation lessons (`docs/research/loft-lessons.md`): a comparison that wasn't
like-for-like (L75), a reference regenerated whenever it disagreed (L76), hand-written "stored
results" (L77), suites that skipped themselves and reported green (L78), and 10 of 12 metrics with
no gate at all (L79).

Already in hand: five oracle fixtures under `validation/fixtures/` with their generator scripts,
and M1.7a's recovery comparison, whose references are real RocketPy output.

**Decision.**

- **A case is a TOML file, a reference is JSON a generator wrote, and the harness only ever reads
  the reference.** `validation/cases/<id>.toml` says what to fly and which metrics to compare
  against which reference file and case; `validation/fixtures/**` holds what the oracle said. A run
  writes nothing but the report, so hpr cannot move its own goal posts (L76) — there is no
  `--update-references` flag, and there will not be one: a reference moves when its generator runs.
- **Every reference value carries a source** naming the oracle, the generator and the field it came
  from, and a value with a blank source is refused (L77). The harness builds those strings from the
  generator's own provenance block rather than trusting a hand-written label.
- **Every metric a case reports has a tolerance that bounds something, or a written reason why it
  is not scored** (L79). A tolerance is a fraction, an absolute difference, or both; one that
  bounds nothing — including an infinite or negative bound — accepts nothing rather than passing
  quietly, and a case that measures or publishes anything it does not account for is refused, in
  both directions: hpr may not measure a metric the case does not gate, and the reference may not
  publish one the case ignores.
- **A metric that cannot honestly be scored is declared, not smoothed over.** The descent cases
  carry no absolute floors: an absolute bound wide enough to carry Valetudo's near-zero northward
  drift would also have been 8.7x looser than 3% of that case's whole drift, which is a weakened
  check wearing a tolerance's clothes. The alternative is `not_scored = "<reason>"`: the harness
  measures and prints the metric with both numbers and the reason, and counts it apart from the
  verdict. Loft excused its two largest misses as "no single target" (L82), so a blank reason fails
  outright and the whole excused set is pinned by a test named after what it is. **No metric uses
  it today.** It was written for Valetudo's northward drift, where hpr read 28x RocketPy; the cause
  turned out to be the gravity model below, and the metric is now gated at 3% like every other. The
  mechanism stays for M2.1b, whose predicted-mode supersonic cases are gaps by construction.
- **A RocketPy comparison flies RocketPy's gravity model.** hpr's default is the full
  normal-gravity vector, which above the ellipsoid leans a few parts in 10⁶ toward the equator;
  RocketPy
  applies gravity to the vertical axis alone. The difference is invisible in every metric that
  matters and decisive in the one that does not: 5.2e-4 m of northward drift over an 800 m
  descent, against a 2.0e-5 m Coriolis signal. hpr already ships `GravityModel::VerticalTaylor`
  as RocketPy's formula "for like-for-like comparisons", so the harness uses it, and Valetudo's
  northward drift comes to −1.8% instead of +2704%. The general rule this is an instance of: when
  the oracle's model is a documented simplification of hpr's and hpr can be asked for the same
  simplification, the comparison uses it and says so, rather than reporting the modelling gap as
  a physics gap (L75).
- **The cases that must run are locked** in `validation/cases/lock.toml`, and a locked case that is
  not there is an error, not a skip (L78). A committed case that is not locked is an error too, so
  a case cannot be added and forgotten — and both checks live in the command, not only in a test.
  `--fast` may only leave out cases the lock marks slow, names them in the report, and writes
  `latest-fast.{md,json}` rather than the committed record.
- **The oracle's inputs come from the reference's own record of what it flew** (L75): the descent
  cases take the site, the wind, the devices and the state at the first deployment from the
  fixture, so a case cannot compare hpr against hpr. That extends to the vehicle: the reference
  records which design it flew and what it weighed, and the harness refuses a case that names
  another design or whose mass differs by more than 1e-9, so a copied case file cannot report the
  difference between two rockets as a difference in the physics. The one place that knows a
  generator's JSON shape is `hpr_validate::rocketpy`, and a file that does not name the oracle,
  the generator and the command that produced it is not a reference at all.
- **The report is committed**, in Markdown for people and JSON for machines, and carries no
  timestamp, so a run that changes nothing changes no bytes and a number that moves shows up in
  the diff. It carries each reference's SHA-256 and the generator's own description of what the
  oracle modelled and what it had to override, so a hand-edited reference or an unlike comparison
  shows up in the report rather than only in git history. A test asserts the committed Markdown is
  byte-for-byte what the harness produces. The JSON is pinned in two parts, because it carries
  full-precision floats and hpr's determinism promise is bit-identical results **on one platform**,
  not across three: everything that cannot differ by platform (cases, metric names, sources,
  tolerances, verdicts, reasons, hashes) is compared exactly, and the numbers through the Markdown
  the committed JSON renders, to the six decimals that report prints. That is the resolution at
  which the descents reproduce on macOS, Windows and Linux, measured, not assumed — CI failed the
  first time this was asserted at full precision. If a platform ever diverges at six decimals, the
  answer is to find out why, not to loosen the comparison.
- **M2.1a's first cases are M1.7a's descents.** They are the only references in hand that cover a
  whole hpr flight path end to end, and reusing them means the harness ships with five real cases
  rather than a demonstration. M2.1b adds the ascent cases in both modes, the CI job and the
  regeneration workflow, which is where `Flight` grows a variant.

**Consequences.**

- M2.1b adds a `Flight::WholeFlight` variant, the same-drag and predicted modes, `hpr-validate`'s
  own binary if one is wanted, and the CI job. The metric names M2.1 lists (apogee, time to
  apogee, maximum velocity and Mach, rail-exit velocity, burnout state, time-series RMS) arrive
  with it.
- M2.4's census reads `validation/reports/latest.json`.
- `hpr-validate` reads files, so it is not part of the pure core and `cargo xtask wasm-check`
  leaves it out, as `ARCHITECTURE.md` already says.
