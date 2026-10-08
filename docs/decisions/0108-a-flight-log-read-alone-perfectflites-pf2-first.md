# ADR-108: A flight log read alone: PerfectFlite's `.pf2` first, heights after a running median, an invented log in CI (2026-09-29)

- **Status:** accepted
- **Summary:** A flight log read alone: PerfectFlite's `.pf2` first, heights after a running median, an invented log in CI

**Context.** M4.2d is done when `hpr analyze` is tested on a log with no design file present, its
JSON validating. `hpr-flightdata` was an empty skeleton; the importers are M7.1 and the readings
M7.2 (ADR-046). M4.2d needs one reader and enough readings for the command to be worth running.
Debrief's notes (`docs/research/debrief-*.md`) give the formats, the thresholds and the
provenance design. Its twelve public fixtures come from publicly shared flights, but their
upstream terms aren't recorded; ADR-046 said they "may appear in examples and doctests".

**Decision.**

1. **The first reader is PerfectFlite's `.pf2`** (`hpr_flightdata::perfectflite`): plain text, a
   barometer only, and a public Pnut log that states its own apogee, which makes it the cheapest
   format with a real cross-check. Units from Debrief's reading of exported files (feet, ft/s,
   °F); the columns from the `Data:` line where Debrief assumed them; a stated height not marked
   in feet refused rather than guessed; SI at the boundary. The record, `log::FlightLog`, is the
   one every later reader fills.
2. **A first set of readings** (`hpr_flightdata::readings`): liftoff, apogee and the time to it,
   the top speed in the climb, landing and the mean descent rate. The top acceleration is
   withheld for a log with no accelerometer. Each reading is `Reading::Read` with a `Source`, or
   `Reading::Withheld` with a `Reason` code and a sentence carrying the log's numbers, the
   non-optional field Debrief's notes recommend. Debrief's corpus-set thresholds are taken as
   they are (3 m, 2 m and 5 m for a second, 4,000 m/s, 20%) and said to be corpus-set, with no
   citation. The landing adds one bound of hpr's, a fall from rest in vacuum, which only drag can
   slow: the height lost from apogee to the landing sample may not come down faster, allowing one
   step of the altitude's rounding in the height and one sample for the apogee's time. A test
   sweeps a vacuum hop's apogee through a foot at 10 to 100 Hz and fails without either
   allowance. The pad is the median of the raw altitude before it first rises
   `PAD_RISE_M` (1 m, hpr's choice), so one jittery first sample doesn't set it. A pad more than
   3 m from the logger's zero withholds liftoff (`starts_off_the_pad`), and with it the top speed
   and the landing (`needs`), which are read against the pad. A record whose channels differ in
   length from its clock, or whose times don't increase, is withheld whole (`bad_record`): the
   reader never builds one, but a program can. A log sampled so fast that the 0.3 s window would
   hold more than 1,000 samples either side (`MAX_MEDIAN_HALF_WINDOW`, over about 6.7 kHz) is
   withheld whole too (`sampled_too_fast`): the median, kept sorted as it slides, costs the
   record's length times the window's, which a far finer clock would make grow with the square of
   the file's length.
3. **Heights after a running median, not a Hampel filter.** Debrief despikes with a Hampel filter
   (0.3 s, threshold 4). On the public Pnut log the trace dips just before the ejection pulse and
   stays lower after it, and those samples widen the pulse's own window's spread until the filter
   keeps it: the Hampel-filtered peak is the pulse's 1,028 ft against the 1,009 ft the logger
   states. The running median (the Hampel filter at threshold 0, Pearson et al. 2015, §2) removes
   any pulse up to half its window wide and reads a noise-free coasting peak low by at most
   `g ((⌈K/2⌉ + ½) Δt)² / 2`, 0.077 m at 20 Hz with `K = 3`, held by a property test. It reads
   1,010 ft.
   `filter::hampel` is kept to show the difference in tests. The apogee's time is the middle of the
   run of samples at the filtered peak, as the one-foot resolution leaves it flat: on the invented
   flight the first sample of the run reads over 0.2 s early, the middle 0.02 s late.
4. **An invented log in CI; the real one only where fetched.**
   [Rule 4](../../CONTRIBUTING.md#4-private-data-stays-private), on third-party data of unclear
   terms, outranks ADR-046's line, so no Debrief fixture is committed.
   `validation/fixtures/logs/synthetic-pnut.pf2` is a flight made up for the tests (boost at 50
   m/s², a drag-free coast, drogue and main at fixed rates, an ejection pulse of the real one's
   shape), written by `hpr_flightdata`'s own test code and held to it. Every reading is checked
   against its closed form within a bound worked out from the rounding and the filter. The public
   Pnut log is read by a test that runs where `refs/` has it, which holds the numbers the docs
   quote: stated 1,009 ft, read 1,010 ft, pulse 1,028 ft, liftoff 0.15 s, top speed 257 ft/s. In CI
   it prints that it skipped, but the harness still lists it as `ok`, so the docs say plainly that
   this check isn't in CI.
5. **`hpr analyze <log>`** reads a `.pf2` by its extension or a first line naming PerfectFlite,
   and refuses anything else, naming M7.1. Its JSON is `output::Analyze`, each reading tagged
   `status: read | withheld` (`schema/cli/analyze.schema.json`). The done-when's "no design file
   present" is tested literally: the log copied alone into an empty folder, `hpr` run there.

**Consequences.** M4.2 closes. The readings are checked on one invented flight and one real
one; the private corpus waits for M7.1, and each leg's descent rate, Mach number and burnout for
M7.2. A barometric altitude is printed as the logger converted it, uncorrected for the day's air.

**Not chosen: commit Debrief's public fixtures**, as ADR-046 allowed. Their upstream terms are
unrecorded, and the invented log checks more (a closed-form truth) in CI.

**Not chosen: the Hampel filter with a physical bound on climb rate.** It would need a speed to
bound by, which is what the pulse corrupts; the median needs nothing.
