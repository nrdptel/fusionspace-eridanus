# ADR-283: The rest of the readings: an axial accelerometer settled by three witnesses, burnout where the thrust's fall ends, and the main found back from landing at the ground's density (2026-10-18)

- **Status:** accepted; ships [M7.2c][m7-2c] as [ADR-280][adr-280] planned it; follows [ADR-281][adr-281] (one height track, the bounds held on the tuning set); amended 2026-10-19 for M7.2c
- **Summary:** The flight-log readings now add the top acceleration, burnout, the flight time and the descent under each canopy, each within a bound its error stayed inside on the synthetic flights' tuning set or withheld by name, with each reading's errors on that set in the report. The top acceleration comes from an accelerometer along the rocket's axis only, its gravity settled by three witnesses (the family's word, the pad, the coast to apogee) and which way up it reads by three more (the pad, the coast and the thrust, each as written), and is a floor where the part clipped, at its stated range or held flat, or where the accelerometer shows no end to the thrust. Burnout is the least specific force after the thrust falls through zero, or, without an accelerometer, a floor: the top speed's time less its bound. The main's opening is found back from landing as Debrief finds it, walked on the rate's running median at the ground's density. On their own logs, the main's opening was found on 87 tuning flights with a barometer, each within its bound, and no main was named on a flight under one canopy, on any log. Of the 1,058 records the sample logs hold, the top acceleration is given on 179, burnout on 434, the drogue's descent on 267 and the main's on 459, and every new reason has a case.

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
[noise]: ../research/altimeter-sensor-noise.md

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
   reading is the thrust's, while a canopy's shock may be the largest (see the amendment below).
   A sample that isn't a number
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
3. **The thrust and the top acceleration.** The turn-over (see the amendment below), the thrust's
   end and burnout are found on a cleaned trace: a Hampel filter over 0.1 s at five robust standard deviations (Pearson et al.,
   2015), HPR Sim's choice, which takes out a charge's shock or one bad sample and keeps a step. A
   burn runs from a cleaned reading past 2 g to where the cleaned trace's 0.05 s mean next falls
   through zero, or to apogee, that holds past 2 g for 0.5 s in all, as a shock or a knock doesn't,
   or the first such span where none does (measured once, when the rule was chosen, on the sample
   logs: of the spans after the first, every one that started within 3 s of apogee held for at most
   0.47 s, and every longer one started more than 3 s before it; [#789][i789]); the burns are
   searched from liftoff's earliest time by the heights, as a rocket at rest reads 1 g, or the log's
   start where liftoff is withheld. The top acceleration is the highest specific force over the
   burns, or from there to apogee with none (see the amendment below), read off the samples
   themselves. Its bound adds a tenth
   of it for the part's scale, 0.05 g of bias, how far the pad strays from its level,
   `g (1 − cos 20°)` for a part that zeroed gravity on a tilted rail, five times the noise measured
   within 0.5 s of it, a step, and the jump to a neighboring sample. It is clipped where it reaches
   the part's stated range, within two steps and 0.25 g, or holds flat at its top over Debrief's
   run, flat meaning within Debrief's tolerance narrowed to half the part's noise where that is
   less, never below half a step. A clipped top is a floor, marked `is_floor`.
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
| Debrief's flat-top tolerance alone | called 82 of 284 tuning-set tops clipped, though no tuning flight reaches its part's range (see the amendment below) |
| Flat within the part's whole noise | still 17 such tops, on smooth thrust crests read at 20 Hz |
| The pad as the samples before liftoff less its bound | no pad on most Raven logs, which start about 0.5 s before liftoff against a liftoff bound near 0.9 s: 69 of the 80 Raven CSV records were withheld as `gravity_unstated` |
| The family's word and the pad alone | the Raven's family doesn't say, and its logs that start in the thrust have no pad; the coast settles them |
| Turning the trace over where the climb's largest reading is negative | a canopy's shock past the thrust turned a trace over: `a_spike_neither_tops_nor_turns_the_trace` |
| Turning it over by the first span, either way, to hold past 2 g for 0.5 s alone | turned a short burn before a heavy drag over, its 40 g read as 7.9 g, measured: `a_short_burn_before_a_heavy_drag_reads_nose_up` |
| The coast witness to which way up read wherever apogee is | read a pad, or a thrust the log ends in, as a coast that says nose down: `a_coast_before_the_trace_moves_says_nothing`, `an_accelerometer_that_ends_in_the_thrust_gives_its_top_as_a_floor` |
| The top searched to apogee | a charge's or a canopy's shock in the coast was taken for the top: the same test |
| The top searched to the first burn's end | missed a second burn's top: `a_second_burn_is_searched_for_the_top_and_leaves_burnout_to_the_speed` |
| Every later span past 2 g a burn | took a canopy's shock for a burn, and it for the top: the same second-burn test |
| The first span past 2 g a burn at any length | a knock and its rebound before the thrust read as a first burn, the thrust as a second: `a_burn_holds_past_two_g_or_is_the_only_one` |
| The top searched from the log's start | a knock on the pad before liftoff was taken for the top: `a_bump_on_the_pad_isnt_the_top` |
| The thrust's end and burnout found on the samples, uncleaned | one sample of −60 g in the thrust moved burnout off its bound: the same test |
| Refusing an accelerometer clock that repeats a time | withheld 48 of the 50 AltOS CSV records as `clock_out_of_order` |
| Debrief's burnout mark, the first reading at or below zero | comes before burnout by as long as the thrust takes to tail off below the drag |
| The reading at the first near-least sample, uncorrected for the mean | took up to 0.6 of its bound on the tuning set's accelerometer logs, against 0.262 corrected |
| The walk on the raw rate | missed the drogue on 2 of the 87 flights whose main's opening their own logs give: a noise spike on a 125 s main leg stopped the walk 125 s after the opening |
| The walk at each rate's own density | named a main on a tuning flight that opened one canopy at apogee: its rate high up passed the mark |

**Consequences.**

- No tuning flight's true top reaches its part's stated range (see the amendment below), the
  nearest 0.492 of it, as the [report][report] prints; so clipping at a range is held by unit tests alone. The ADXL375 read at
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

**Amendment for M7.2c, the rules as built (2026-10-19).** A review of the first build found that
§1's turn-over could be set by a knock on the pad, or by a heavy drag after a short burn, that a
top with no end to its thrust was given as measured, that `hpr analyze` named a range for a top
that only held flat, and that no tuning flight tested clipping at a range. The rules as built now:

1. **Which way up (§1), by three witnesses, as gravity is (§2).** Each is read on the trace as
   written, before it is turned, from physics that holds whether or not the part takes gravity
   out, within 0.25 g. The pad, read as §2 reads it but ended where the trace first moves 1 g
   from it either way: about 1 g says nose up, about −1 g nose down, about 0 nothing. The coast's
   last second before apogee's earliest time, read only where a coast was flown, after liftoff's
   latest time and the trace's first move and before an apogee the log doesn't end at: a part
   reads `−D/m` there, or `−D/m − g` without gravity, never above 0 nose up while it coasts, its
   thrust ended and no canopy open, so −0.75 g or less says nose up, 0.75 g or more nose down,
   and nearer 0 nothing. The thrust: the first span of
   the cleaned trace, in either sign, that holds past 2 g for 0.5 s, searched from liftoff's
   earliest time as the burns are (§3), says its sign unless a span of the other sign comes
   before it; where none holds that long, the sign every span shares, a short burn's. The way is
   settled when the pad or the thrust says and no witness disagrees: the coast alone settles
   nothing, as its second can hold a weak thrust still burning or a canopy opened early, either
   of which reads the other way, so it can only disagree. Otherwise, or where the trace so
   turned, gravity put back, never passes 2 g and so shows no thrust, the top acceleration and
   the accelerometer's burnout are withheld with a new reason, `direction_unstated`, the 27th,
   and burnout comes from the top speed. A top or burnout earlier than liftoff's earliest time is
   never given. A first rule, the thrust's span alone, turned a short burn before a heavy drag
   over, reading its 40 g as 7.9 g, measured: `a_short_burn_before_a_heavy_drag_reads_nose_up`
   holds the case, read by its pad with gravity in and withheld with gravity out. A second rule
   let any one witness settle the way, and a review read a 40 g burn as 8.9 g, measured, by a
   coast alone whose second held a canopy opened early, and a weak thrust turned over by a coast
   whose second held the thrust still burning: `a_lone_coast_settles_nothing` holds both. `a_dip_on_the_pad_doesnt_turn_the_trace_over`,
   `a_knock_with_its_rebound_first_doesnt_turn_the_trace_over`,
   `a_pad_that_disagrees_with_the_thrust_is_withheld` and
   `a_weak_thrust_without_gravity_is_withheld_not_turned` hold the rule. Of the sample logs,
   against the first rule: one Raven CSV record it turned over reads nose up, its burnout now
   from the accelerometer; two AltOS CSV records it turned over, and a third, are withheld, as
   AltOS takes gravity out, so their pads say nothing, and their short burns come before a heavy
   drag, so their thrusts say nothing, leaving their coasts alone; one Raven CSV record whose
   coast reads near +1 g, against a pad and a thrust that say nose up, is withheld; and one
   Entacore AIM CSV record (`f5df1597457a39ea`) stays withheld, as nothing says. The top
   acceleration is given on 179 records, not 184, and burnout on 434, not 435.
2. **A thrust with no end (§3, §4).** A top whose last burn never falls through zero before the
   accelerometer's samples end or apogee is a floor, `measured`: the log, or the accelerometer
   alone, may stop while the motor still pushes. Burnout isn't read off it, and comes from the top
   speed. The synthetic report counts this cause as `thrust_unended`; no tuning-set top reads so.
   `an_accelerometer_that_ends_in_the_thrust_gives_its_top_as_a_floor` holds it.
3. **Why a top clipped (§3).** A clipped top says why in a new field, `Clip` in the library and
   `clip` in the JSON and Python: `range` where it reaches the part's stated range, within two
   steps and 0.25 g, or `flat` where it holds flat at its top below that range or with none
   stated. `hpr analyze` names the part's range only for `range`.
4. **Clipping on the tuning set (§7; Consequences).** The tuning set gains a log of each of the
   500 Hz IMU profile's 38 flights with its low-g part set to ±8 g, a full scale the BMX160 and the
   MPU-6000 offer on the same 16 bits ([the sensor noise note][noise]). 28 of their true tops reach
   that range and 10 don't; 28 tops read as clipped at the range, each a floor that holds, and the
   4 held flat stay on the AltOS-like profile's high-g part read at 20 Hz. So clipping at a range
   is held on the tuning set for the low-g part, and only by unit tests for the high-g part's
   ±180 g. With the new log the [report][report] gives the top acceleration on 290 logs and as a
   floor on 32, and burnout from the accelerometer on 300, its mean bound 0.582 s; no reading's
   largest share of its bound changed. `a_part_set_to_its_lower_range_clips_there` holds the log.
5. **One canopy (§6), as the code states it.** A drogue no faster than the main's mark at the
   ground's density reads as one canopy, whatever its ratio to the main: the walk back from
   landing never leaves the main's run, which then reaches apogee, and the main's descent is the
   whole descent's. Debrief's ratio of 1.4 is a second test the drogue must also pass, not the one
   that decides (`close_rates_are_one_canopy`: a drogue at 12 m/s over a main at 10 m/s, whose
   mark is 16 m/s). The rate's running median is taken in samples at the descent's median
   interval, never more than the descent holds.
