# ADR-283: The rest of the readings: an axial accelerometer settled by three witnesses, burnout where the thrust's fall ends, and the main found back from landing at the ground's density (2026-10-18)

- **Status:** accepted; ships [M7.2c][m7-2c] as [ADR-280][adr-280] planned it; follows [ADR-281][adr-281] (one height track, the bounds held on the tuning set)
- **Summary:** The flight-log readings now add the top acceleration, burnout, the flight time and the descent under each canopy, each within a bound its error stayed inside on the synthetic flights' tuning set or withheld by name, with each reading's errors on that set in the report. The top acceleration comes from an accelerometer along the rocket's axis only, its gravity settled by three witnesses (the family's word, the pad, the coast to apogee), and is a floor where the part clipped. Burnout is the least specific force after the thrust falls through zero, or, without an accelerometer, a floor at the top speed's time. The main's opening is found back from landing as Debrief finds it, walked on the rate's running median at the ground's density. On their own logs, every tuning flight with a barometer that came down under two canopies had its main's opening found, and no main was named on a flight under one, on any log. Of the 1,058 records the sample logs hold, the top acceleration is given on 184, burnout on 435, the drogue's descent on 267 and the main's on 459, and every new reason has a case.

[m7-2c]: ../decisions-and-roadmap.md#m7-2c
[adr-280]: 0280-m7-2-in-seven-the-synthetic-flights-first.md
[adr-281]: 0281-readings-on-every-logger.md
[page]: ../physics/log-readings.md
[report]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/synthetic-flights.md#errors-on-the-tuning-set
[i779]: https://github.com/nrdptel/fusionspace-eridanus/issues/779
[i780]: https://github.com/nrdptel/fusionspace-eridanus/issues/780
[i781]: https://github.com/nrdptel/fusionspace-eridanus/issues/781
[i787]: https://github.com/nrdptel/fusionspace-eridanus/issues/787
[i789]: https://github.com/nrdptel/fusionspace-eridanus/issues/789

**Context.** ADR-280's M7.2c row asks for the top acceleration from the accelerometer, marked clipped
at the part's stated range; burnout, measured from the accelerometer or derived from the speed's
peak; the descent rate under each canopy, the main found back from landing as Debrief finds it; and
the flight time. Each names its method and provenance, has a withheld reason with a corpus case, and
reports its error on the tuning set. Debrief, the project owner's earlier analyzer (MIT,
`lib/analyze/index.ts`), takes burnout as the first reading at or below zero after the peak, calls a
part clipped when its top holds flat over 4 samples or 0.05 s within 0.3% or 0.25 m/s², and finds the
main from landing by the run of descent rates within 1.6 times the last 2 s's median, or 3 m/s more.
Its marks were set on real logs with no truth beside them; the tuning set has one.

**Decision.**

1. **The axial accelerometer only.** Each family's axial column is named in a table, with whether
   the family says its readings hold gravity and the part's range where it states one; any other
   column that names the axis is read with neither. A board's three axes don't say which lies along
   the rocket, and their magnitude never falls below zero at burnout: `axis_unstated`. A part
   mounted nose down is turned over where the cleaned trace's (§3) first reading past 2 g is
   negative: a pad reads 1 g or 0 either way up and a rocket leaves it at 3 g or more, so that
   reading is the thrust's, while a canopy's shock may be the largest. A sample that isn't a number
   is left out, and so is one that repeats the time before it, the first kept, as AltOS's primary
   track keeps it. A part sampled faster than every 0.002 s, the tuning set's fastest, is
   `sampled_too_fast`.
2. **Gravity by three witnesses,** each within 0.25 g: the family's word; the pad, read off the
   accelerometer itself as the steady level before the thrust lifts it by 1 g, no later than
   liftoff's latest time by the heights (about 1 g holds gravity, about 0 doesn't); and the coast's
   last second before apogee's earliest time, where a rocket falls freely but for a small drag
   (about 0 holds gravity, about −1 g doesn't). A pad or coast that reads neither says nothing.
   Gravity is settled when one witness or more says and none disagrees; otherwise
   `gravity_unstated`.
3. **The thrust and the top acceleration.** The turn-over, the thrust's end and burnout are found on
   a cleaned trace: a Hampel filter over 0.1 s at five robust standard deviations (Pearson et al.,
   2015), HPR Sim's choice, which takes out a charge's shock or one bad sample and keeps a step. A
   burn runs from a cleaned reading past 2 g to where the cleaned trace's 0.05 s mean next falls
   through zero, or to apogee, a later one only if it holds past 2 g for 0.5 s in all, as a shock
   doesn't (measured once, when the rule was chosen, on the sample logs: every such span within 3 s
   of apogee held for at most 0.47 s, and every longer one came more than 3 s before apogee;
   [#789][i789]); the burns are searched from liftoff's earliest time by the heights, as a rocket at
   rest reads 1 g. The top acceleration is the highest specific force over the burns, or from there
   to apogee with none, read off the samples themselves. Its bound adds a tenth of it for the part's
   scale, 0.05 g of bias, how far the pad strays from its level, `g (1 − cos 20°)` for a part that
   zeroed gravity on a tilted rail, five times the noise measured within 0.5 s of it, a step, and
   the jump to a neighboring sample. It is clipped where it reaches the part's stated range, within
   two steps and 0.25 g, or holds flat at its top over Debrief's run, flat meaning within Debrief's
   tolerance narrowed to half the part's noise where that is less, never below half a step. A
   clipped top is a floor, marked `is_floor`.
4. **Burnout** is where the thrust's fall ends: the least of the cleaned trace's 0.05 s mean,
   `2h + 1` samples, after it falls through zero. Its reading is the first sample within twice the
   averaged noise bound of the least, less half the window; its bound their span, an interval and
   the window's `2h` intervals. With no climb back of four noise bounds before apogee, a least
   within three of zero, or more than one burn, which the tuning set's flights never make, the
   accelerometer gives none, and burnout is the top speed's time: a floor, as a rocket gains speed
   only while the thrust beats drag and weight.
5. **The flight time** is landing's time less liftoff's, its bound the sum, a reading of its own.
6. **The canopies.** The main's opening is Debrief's walk back from landing, with two changes: the
   walk runs on the rate's running median over 2 s, or twice the fit where that is longer, and
   every rate is compared at the ground's density, scaled by `exp(−h/2H)` with `H` = 7,000 m,
   under the troposphere's least density scale height (7,289 m at the tropopause on a day 15 K
   cold), as a canopy falls as the density to the −½. A drogue came first if Debrief's ratio
   holds, 1.4, and the 2 s just before the run fell faster than its mark. The opening's bound is
   the time a canopy takes to slow a fall to the mark from any speed, `τ = (v_m/2g) ln((v₁ +
   v_m)/(v₁ − v_m))`, half the fit and two samples. Each leg's rate is the height it loses over its time.
   Under one canopy the drogue is `one_canopy` and the main's leg runs from apogee; a leg losing no
   more than 3 m or a tenth of its top's height is `short_leg`; a fit that would need more than
   10 s to hold four steps is `step_too_coarse`.
7. **The report.** `cargo xtask synthetic` checks each new reading on the tuning set, now also
   logged with its accelerometer at 500, 100 and 20 Hz; a floor holds while it is no more than its
   bound above the truth; a drogue that the truth has none of is counted as none to read; and each
   reading's errors over every log, the mean, the least and the most, beside the mean bound, are
   in the [report][report].

**Rejected, with the measurement that ruled each out.** Each was measured once, when the rule was
chosen, with the rule put back in the code; no check reruns the counts, while the unit test named
in a row fails with its rule put back.

| Rule | Ruled out by |
|---|---|
| Debrief's flat-top tolerance alone | called 82 of 284 tuning-set tops clipped, though no tuning flight reaches its part's range |
| Flat within the part's whole noise | still 17 such tops, on smooth thrust crests read at 20 Hz |
| The pad as the samples before liftoff less its bound | no pad on most Raven logs, which start about 0.5 s before liftoff against a liftoff bound near 0.9 s: 69 of the 80 Raven CSV records were withheld as `gravity_unstated` |
| The family's word and the pad alone | the Raven's family doesn't say, and its logs that start in the thrust have no pad; the coast settles them |
| Turning the trace over where the climb's largest reading is negative | a canopy's shock past the thrust turned a trace over: `a_spike_neither_tops_nor_turns_the_trace` |
| The top searched to apogee | a charge's or a canopy's shock in the coast was taken for the top: the same test |
| The top searched to the first burn's end | missed a second burn's top: `a_second_burn_is_searched_for_the_top_and_leaves_burnout_to_the_speed` |
| Every later span past 2 g a burn | took a canopy's shock for a burn, and it for the top: the same second-burn test |
| The top searched from the log's start | a knock on the pad before liftoff was taken for the top: `a_bump_on_the_pad_isnt_the_top` |
| The thrust's end and burnout found on the samples, uncleaned | one sample of −60 g in the thrust moved burnout off its bound: the same test |
| Refusing an accelerometer clock that repeats a time | withheld 48 of the 50 AltOS CSV records as `clock_out_of_order` |
| Debrief's burnout mark, the first reading at or below zero | comes before burnout by as long as the thrust takes to tail off below the drag |
| The reading at the first near-least sample, uncorrected for the mean | took up to 0.778 of its bound on the tuning set's accelerometer logs, against 0.433 corrected |
| The walk on the raw rate | missed the drogue on 2 of the 87 two-canopy flights: a noise spike on a 125 s main leg stopped the walk 125 s after the opening |
| The walk at each rate's own density | named a main on a tuning flight that opened one canopy at apogee: its rate high up passed the mark |

**Consequences.**

- No tuning flight's true top reaches its part's stated range, the nearest 0.492 of it, as the
  [report][report] prints; so clipping at a range is held by unit tests alone. The ADXL375 read at
  20 Hz is quieter than half its 49 mg step, 19.8 mg by the synthetic flights' noise model, and
  writes a thrust's plateau as one value, so a plateau as long as Debrief's run reads as flat: 4
  tuning-set tops, all on it, all floors, which can't flatter it.
- Burnout's averaged noise is the samples' over √(2h + 1), as if each sample's noise were its
  own; a part whose noise runs from sample to sample would read a bound too narrow ([#787][i787]).
- The coast witness assumes the drag in the last second before apogee's earliest time is under
  0.25 g. A rocket still fast at apogee, arcing over, reads its drag there; the witness then says
  nothing, or, past 0.75 g, could disagree with a family's word and withhold the reading.
- A canopy's filling time isn't in the opening's bound: the synthetic flights open each canopy at
  once ([#779][i779]). A rocket that came down under no canopy has its fall read as one canopy's
  ([#780][i780]).
- 136 corpus records give no top acceleration as `axis_unstated`: their accelerometer columns
  don't say which lies along the rocket, as a board's three axes don't. A magnitude of the three
  could serve a top acceleration but not burnout, and is left for later ([#781][i781]).
- The methods, bounds and their tests are on [the readings' page][page].
