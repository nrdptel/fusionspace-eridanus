# Checking a claim

Trace any number HPR Sim computes to its published source, the test that pins it and any comparison
with another program or a flight.

Every number HPR Sim computes, and every number on this site, should have that trail. Where a
comparison exists, it's with another program or a real flight. This page shows how to follow it,
with two worked examples. It's for anyone who would rather check a claim than trust it. If a trail
breaks, that's a bug, and the last section says how to report it.

> **How far to trust it.** This page is a method, not a result. Its two worked examples follow
> trails that exist today. One is gravity at the equator, 9.7803253359 m/s² as Table 3.6 of the
> WGS 84 standard prints it. The other is a parachute's north drift, +2.865% from RocketPy's as the
> [validation report][report] gives it. The site's build checks that each of this page's links
> into the site or the repository leads somewhere. It doesn't check that the steps still describe
> the code. Where they differ, the source, the test and the report are the authority, not this page.

## The trail

1. **The page.** Each model has a page on this site. Its *In short* says what the model is, where
   it comes from, how well it's validated and what it leaves out. Below that, each section gives
   the equations in words and symbols, and cites its source by a short key in square brackets,
   such as **[NGA]**.
2. **The source.** Near the top of the page, under *Code and sources* (or *Sources*), each key's
   full reference is listed. Downloadable sources are pinned in the [reference lock file][lock]
   by address and SHA-256 hash, so everyone checks the same file. A hash is a fingerprint of the
   file's exact bytes ([the reference library decision][adr-002]). `cargo xtask refs fetch`
   downloads them into the repository's `refs/` folder, which is never committed, and
   `cargo xtask refs verify` checks their hashes again. A few sources couldn't be fetched, such as
   Dormand and Prince's 1980 paper, and the page says so: "cited, not fetched".
3. **The test.** The page's *Verification* or *Tests that pin this* section names the tests that
   hold the code to the source, with each one's [tolerance](glossary.md#tolerance). A tolerance is
   how far a result may be from its reference and still pass. `cargo test` runs all of them, on
   every change, on macOS, Windows and Linux.
4. **The comparison.** Where a model is compared with another program or a high-precision
   calculation:
   - The reference values are a [fixture](glossary.md#reference-value-and-fixture), a file under
     [`validation/fixtures/`][fixtures].
   - A program under [`validation/oracles/`][oracles], an [oracle](glossary.md#oracle), writes
     each fixture, with a record of how.
   - The model page's verification section names the fixture and the test that reads it.
   - Some comparisons, today the descents under a parachute, are also run by the validation
     harness. That's the program behind `cargo xtask validate`, which flies them again and
     rewrites the committed [validation report][report], one row per number. To fly them again and
     check the report without rewriting it, clone the repository and run
     `cargo xtask validate --check` ([what it checks](accuracy.md#the-census)).
   - Each of those has a [case file](glossary.md#validation-case) under
     [`validation/cases/`][cases]. It says what is flown and how close the two must agree, and
     argues for that tolerance.
   - The rules behind this are in [the validation harness decision][adr-015].

The [Accuracy](accuracy.md) page gathers every result from steps 3 and 4 in one place.

## Example: gravity at the equator

The claim is that the simulator's gravity on the equator, on the [WGS 84](glossary.md#wgs-84)
ellipsoid, is 9.7803253359 m/s². That's at height 0 above the ellipsoid, which isn't quite sea
level (see [ellipsoidal height](glossary.md#ellipsoidal-height)).

1. The page is [Gravity and Earth rotation](physics/gravity.md). It models
   [normal gravity](glossary.md#normal-gravity), the pull of an idealized Earth plus the effect of
   its spin. The table under [Defining parameters and
   derived constants](physics/gravity.md#defining-parameters-and-derived-constants) gives $\gamma_e$, the
   value at the equator, as 9.7803253359 m/s², and cites Table 3.6 of **[NGA]**.
2. The source, **[NGA]**, is the US National Geospatial-Intelligence Agency's WGS 84 standard,
   NGA.STND.0036 (2014), a US government work. The lock file pins it as `wgs84-nga-stnd-0036`, and
   `cargo xtask refs fetch wgs84-nga-stnd-0036` downloads it. Table 3.6 prints the same value.
3. The test, under [Tests that pin this](physics/gravity.md#tests-that-pin-this), is
   `gravity::tests::somigliana_matches_published_values` in
   [`crates/hpr-core/src/gravity.rs`][gravity-rs]. It checks the constant against Table 3.6 to its
   printed digits. It also checks gravity at 11 latitudes and heights against
   [values computed from the standard's formulas at 40 digits][gravity-fixture], by a separate
   script, [`normal_gravity.py`][gravity-oracle]. Run it with:

   ```text
   cargo test -p fusionspace-hpr-core somigliana_matches_published_values
   ```

4. No comparison is needed here: the standard itself is the reference. This is the second of the
   four kinds of evidence that [Accuracy](accuracy.md) describes: a model checked against its
   published source.

## Example: how far a rocket drifts under its parachute

The claim is about the NDRT 2020 rocket, one of RocketPy's
[example rockets](glossary.md#example-rockets). HPR Sim's north [drift](glossary.md#drift) under
the parachute differs from RocketPy's by +2.865%, inside the 3% tolerance its case allows.

1. The [validation report][report] has a row for the case `descent-ndrt-2020-nose-to-tail` and
   the metric `drift_north_m`. It gives HPR Sim −50.83 m (−166.8 ft) and RocketPy −49.42 m
   (−162.1 ft). The difference is +2.865%, the tolerance 3.000%, and the verdict *pass*.
2. The case file, [`descent-ndrt-2020-nose-to-tail.toml`][ndrt-case], names the rocket's design
   and the reference file, and holds each [metric](glossary.md#metric) to 3% (its landing speed to
   0.2%). Its opening comment argues why there's no absolute floor, a fixed allowance in meters
   that would pass any smaller difference. A floor big enough to excuse a drift component near zero would hide a real error
   in a large one.
3. The reference values come from [`rocketpy-descent.json`][descent-fixture], which the report
   names with its hash. The script [`recovery.py`][recovery-oracle] wrote it by flying RocketPy
   1.13.0's own parachute phase. The lock file pins that RocketPy by commit, the id of one exact
   version of its code. The reference file's first lines say what RocketPy computes, and what the
   comparison overrides. Both codes start from the same state, with the first parachute opening
   at once.
4. The model page, [Recovery](physics/recovery.md), explains HPR Sim's descent, and its section
   [Against RocketPy](physics/recovery.md#against-rocketpy) explains the differences that remain.
   The likely cause is [added mass](glossary.md#added-mass), which HPR Sim has no term for.
   RocketPy treats the air a canopy drags along as extra mass: 15.9 kg for NDRT's main against
   the rocket's 20.8 kg. It slows the response to the opening, which most likely moves this small
   drift component by 2.86%. That fits the size of the gap, but no test has isolated it yet.
5. The test `recovery::tests::descent_matches_rocketpy_examples` in
   [`crates/hpr-sim/src/recovery.rs`][recovery-rs] replays all five cases on every change, and
   `cargo xtask validate` writes them into the report.

This shows the two codes agree on the physics of a descent. It says nothing about whether either
matches a real parachute on a real day. The [real flights](accuracy.md#real-flights) compared so
far stop at apogee, so no descent has been checked against one yet.

## Rules that keep the trail honest

- A reference is never changed to make a comparison pass. It moves only when its generator runs
  again, a deliberate step recorded in the history. This guards against
  [Loft lesson L76](decisions-and-roadmap.md#l76). Loft, the project before this one, said to
  regenerate a reference when its drift check failed, so the reference moved with Loft's own drag.
- On every pull request, CI reruns every case on macOS, Windows and Linux. It fails if a metric is
  outside its tolerance, or if the committed report no longer matches, apart from last-digit
  rounding. The workflow that regenerates the references runs only when a person starts it. It
  can't commit: its output is a diff for a person to review
  ([how CI and regeneration work](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/VALIDATION.md#in-ci-and-regenerating-the-references)).
- A tolerance lives in its case file, with its reason. Loosening one to turn a failure into a
  pass isn't allowed. A target that can't be met gets a
  [decision record](glossary.md#decision-record-adr) that shows the measurement, and the gap stays
  in the report.
- The site checks its own numbers. Its build fails if any of these breaks:
  - Every number on [Accuracy](accuracy.md) outside code must appear in a file that its
    paragraph, list item or table row links to. A number is anything with two or more digits, a
    decimal point, an exponent or a percent sign. The file can be the validation report, a case
    file or a model page outside its *In short*. A guide page such as this one doesn't count, so
    no page can vouch for itself.
  - It must appear there written the same way: the same digits, and the same sign and percent sign
    where Accuracy writes them. So 2.8 doesn't match 2.865, and −2.865% doesn't match +2.865%.
  - Accuracy's table of the descent results must match the validation report cell by cell, and
    hold every result in it.
  - Every number in a model page's *In short* must appear in the rest of that page, or in a file
    its item links to.
- That check has two blind spots. When a number changes at its source, the build fails until the
  page that quotes it follows. It doesn't if the old number still appears elsewhere in that file.
  And the check can't tell whether a number is quoted in the right context, such as for the right
  rocket. The reviews check that, and the model pages say what each number means.

## When the trail breaks

The trail breaks where a number has no source, no test, or a test that doesn't check what the
page says. If you find one on this site or from the code, please [open an issue][issues] with the
page and the number. The pencil icon at the top of each page opens its source on GitHub, where you
can propose a fix.

[adr-002]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0002-the-reference-library-lock-file-fetch-verify-and.md
[adr-015]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0015-the-validation-harness-cases-references.md
[cases]: https://github.com/nrdptel/fusionspace-eridanus/tree/main/validation/cases
[descent-fixture]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/fixtures/recovery/rocketpy-descent.json
[fixtures]: https://github.com/nrdptel/fusionspace-eridanus/tree/main/validation/fixtures
[gravity-fixture]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/fixtures/earth/wgs84-normal-gravity.json
[gravity-oracle]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/oracles/wgs84/normal_gravity.py
[gravity-rs]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/crates/hpr-core/src/gravity.rs
[issues]: https://github.com/nrdptel/fusionspace-eridanus/issues
[lock]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/refs.lock.toml
[ndrt-case]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/cases/descent-ndrt-2020-nose-to-tail.toml
[oracles]: https://github.com/nrdptel/fusionspace-eridanus/tree/main/validation/oracles
[recovery-oracle]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/oracles/rocketpy/recovery.py
[recovery-rs]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/crates/hpr-sim/src/recovery.rs
[report]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/latest.md
