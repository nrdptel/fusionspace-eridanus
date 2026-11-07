# ADR-326: M7.3d3 in four, the halves sealed first (2026-11-07)

- **Status:** accepted; splits [M7.3d3][m7-3] into M7.3d4 to M7.3d7, each with its own *done when*, M7.3d's *done when* whole in the last, and so plans every remaining increment of M7.3d; carries out M7.3d4 under [ADR-322][adr-322] §2.
- **Summary:** The private collection's held-out flights are fixed by ADR-322's encoding, and the SHA-256 of their sorted ids is committed on its own, before any half's number exists. Each flight's log is chosen from its pinned logs by the apogee nearest the one the collection states, read and cut at the boost, counted over the whole collection only: of 72 flights not left out, 64 have a barometer's log the analyzer reads an apogee from, 32 are cut and 30 refused at the transonic limit. The fix to the cut ([#1307](https://github.com/nrdptel/fusionspace-eridanus/issues/1307)), the collection's errors by halves and the scoreboard row follow, in that order.

[adr-151]: 0151-hpr-sim-fixtures-joins-the-reference-library-as.md
[adr-209]: 0209-the-2026-10-08-best-on-the-market-measured.md
[adr-257]: 0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
[adr-321]: 0321-m7-3c4-in-three-the-track-as-a-receiver-reads-it.md
[adr-322]: 0322-the-2026-11-05-progress-review-accuracy-before-reader-facing-work.md
[adr-325]: 0325-m7-3d2-the-public-flights-prediction-from-the-boost.md
[i1307]: https://github.com/nrdptel/fusionspace-eridanus/issues/1307
[m7-3]: ../ROADMAP.md#phase-5-flight-data-and-forensics

**Context.** [ADR-321][adr-321] split M7.3d in three; M7.3d1 and M7.3d2 are merged, and M7.3d3
holds what is left of M7.3d's *done when*: the collection's fits as aggregates, its prediction
errors beside the forward simulation's with [ADR-257][adr-257] §3's paired bootstrap interval,
each for both of M2.7's halves ([ADR-322][adr-322] §2), and the scoreboard's
`accuracy.from-boost`. Surveyed on 2026-11-07, four pieces of work stand between that and main:

- **The seal.** ADR-322 §2 asks for a commit holding the held-out ids' SHA-256 before any half's
  number is reported. Pull requests here are squash-merged, so a branch's commit order doesn't
  reach main: only a pull request of its own puts the seal on main first.
- **Each flight's log.** The manifest names the flight, its design and its logged apogee, but not
  which log read that apogee: it says so in free text. 63 of its 83 flights have more than one
  log file in the collection's catalog, from a second altimeter, a GNSS tracker, a raw and a
  processed export, or a summary.
- **The cut.** On the public flights' barometers the boost's cut falls inside the burn on 4 of 5
  cuts ([ADR-325][adr-325] §5, [#1307][i1307]). A prediction from such a cut is not the method's
  case, so the collection's errors are worth reporting only after the cut is fixed, or the cuts
  inside the burn are counted.
- **The fits and the row.** Each flight flown from its `.ork` in its ERA5 air, as
  `fixture-flights` flies it, fitted to the cut and on to apogee; then the scoreboard row, each
  other tool's cell read from its docs.

**Decision.**

1. **Four increments, in this order.**
   - **M7.3d4 The halves sealed.** *Done when:* `cargo xtask fixture-flights` splits the private
     collection's flights by ADR-322's encoding, a test pinning it on invented ids against digests
     computed outside the code, with a mutation of its bound failing; the SHA-256 of the sorted
     held-out ids is committed, and `fixture-flights --check` fails when it no longer matches;
     each flight's log is chosen by a stated rule from its pinned logs, read, and cut or refused
     for its reason, counted over the whole collection in a committed report that gives no
     half's number.
   - **M7.3d5 The cut on a real barometer** (#1307). *Done when:* synthetic logs with a barometer's
     dips as deep and long as the public logs' (up to 0.5 s) are cut after burnout, set A's within
     the test's 2.83 s bound; both reports count the cuts before the motor's burn ends.
   - **M7.3d6 The collection's errors by halves.** *Done when:* each cut log's fit is flown to
     apogee as `fixture-flights` flies it; a committed report gives each half's mean absolute and
     signed errors beside the forward simulation's on the same flights, ADR-257 §3's paired
     bootstrap 95% intervals and the calibration half's fit aggregates; the held-out half's stay
     private, their file's SHA-256 committed and checked.
   - **M7.3d7 The scoreboard row.** *Done when:* M7.3d's *done when*, whole, the scoreboard's
     `accuracy.from-boost` included.
2. **The seal covers the whole manifest.** The digest is of every held-out id among the
   manifest's 83 flights, the population M7.3d's collection numbers draw from, whether or not a
   flight is left out of the errors or has a usable log. A flight's half depends on its id alone,
   so when the manifest gains a flight the old flights stay where they were and
   `fixture-flights` writes a new digest; `--check` fails until it is committed. The digest's file
   is `validation/reports/held-out.sha256`. A test pins the encoding on invented ids whose digests
   Python's `hashlib` computed; another pins the bound at ⌊(2⁶⁴ − 1) / 3⌋ =
   6,148,914,691,236,517,205 (2⁶⁴ − 1 is divisible by 3, so that is the largest integer below
   2⁶⁴ / 3), and fails when `<=` becomes `<` (run once).
3. **A flight's log is the one nearest its logged apogee.** Of the flight's logs in the catalog,
   outside its unpinned `_scratch/` folder as `cargo xtask logs` leaves it, each is read alone,
   with no sheet, recording or companion file. Of those whose height track is barometric (the
   logger's altitude, a pressure, or a barometric altitude channel) and gives the analyzer an
   apogee, the chosen log is the one whose apogee is nearest the manifest's logged apogee, the
   first in the catalog's order on a tie. The manifest's apogee was read by hand from one
   altimeter's log, so the nearest log finds that altimeter again without a hand-made table,
   which would be one more private file to keep in step. Measured: of the 64 chosen logs, 1
   reads an apogee more than 2% from the logged one. The report counts it rather than refusing
   it: either altimeter logged the same flight.
4. **Counts only, until M7.3d6.** The report, `validation/reports/collection-boost.{json,md}`,
   counts over the whole collection: 83 flights, 11 left out by the manifest, 3 with no log that
   reads alone, 5 with a log read but no barometric apogee, 64 chosen; 32 cut, 30 refused at the
   transonic limit, 2 for a log slower than 8 Hz. Of the 241 pinned logs tried, 199 read alone
   and 117 gave a barometric apogee. Nothing is given per flight or per half
   ([ADR-151][adr-151]).

**What doesn't change.** M7.3d's *done when* is whole in M7.3d7, and no clause is narrowed: the
three before it each take a part of it, or a fix its numbers need. ADR-322's encoding, and the
rule that held-out flights never feed a choice of model ([ADR-209][adr-209] §4), stand.

**Consequences.** `cargo xtask fixture-flights` writes the digest and the counts, and `--check`
holds both, in the gate's `refs` step. CI, without `refs/`, holds the committed page to its
counts and the digest's file to its form. The 30 flights refused at the transonic limit can't
be predicted by this method: a barometer's reading past it may follow the airflow, not the
height. The collection's predictions will come from at most the 32 cut, fewer if M7.3d5 moves
cuts.
