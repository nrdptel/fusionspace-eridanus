# ADR-374: The main's opening bound holds both the split and the walk (2026-11-23)

- **Status:** accepted; changes the opening's bound and 90% interval of [ADR-366][adr-366] where the split is earlier than the walk's start; changes no reading, mark, window or rate
- **Summary:** Where the descent's rate passes the main's mark more than once, the likeliest split ([ADR-366][adr-366]) can be earlier than Debrief's walk back from landing, and the log alone can't say which is the opening ([#1593](https://github.com/nrdptel/fusionspace-eridanus/issues/1593)). The reading stays the split; its bound adds the time between the two, and its 90% interval runs from the lower end of the interval about the split to the upper end of the interval about the walk's start. On the swinging flight of [#903](https://github.com/nrdptel/fusionspace-eridanus/issues/903) the bound goes from 0.76 s to 19.21 s. In the synthetic census the main opening's mean bound goes from 2.03 to 2.05 s.

[adr-366]: 0366-the-mains-opening-at-the-likeliest-split.md
[adr-283]: 0283-the-rest-of-the-readings.md
[page]: ../physics/log-readings.md#the-canopies
[report]: ../../validation/reports/synthetic-flights.md

**Context.** [ADR-283][adr-283] §6 derives the opening's bound for a descent whose rates go from
fast to slow once: the slowing time `τ` of quadratic drag to the mark, half the 1 s fit, and two
sample intervals. [ADR-366][adr-366] moved the opening to the likeliest single change of how
often the rate is within the mark, which is the walk's start in that case and never later than it
otherwise. It kept the bound. A physics review of that change found a shape where the split is
wrong and the walk right: 30 slow rates, 200 fast, 100 slow, 25 fast and 1,000 slow split at 230,
where the walk's 355 is the opening. At 20 Hz that is 6.25 s early, against a bound of about 1 s.
The reading had both places at hand and printed a bound proven only where they agree.

**Options.**

1. **Withhold the opening** where they disagree. This also withholds #903's flight, where the
   split is right (90.00 s against the header's 87.15 s charge) and the walk stopped at a swing
   18 s later. It gives up a good reading to avoid a wide one.
2. **Add the time between them to the bound.** The reading stays the likeliest place; the bound
   states what the log can't settle.
3. **Choose between them** by a further rule. Nothing in the rates separates a swing under the
   main from a slow stretch of the drogue: both are a fast stretch beside slow ones.

**Decision.**

1. **The bound.** Where the walk's start (the first slow rate after the last fast one) is later
   than the split, the opening's bound is
   `τ + ½ window + 2 Δt + (t_walk − t_split)`, with `τ` at the split's height and `Δt` the
   coarser sample interval of the two places. `τ` grows with height, and the split is the higher,
   so it is the larger of the two; where the main's rate up there passes the mark, `τ` is the time
   since apogee, which holds the walk's start as well. The bound then holds an opening within its own margin of the
   split or of the walk's start. Where the two are the same sample, nothing changes.
2. **The 90% interval.** Its shares ([ADR-283][adr-283]) were fitted on the bound about one place,
   and put the interval wholly before the reading. Taken on the widened bound they would move it
   further before the split, away from the walk's start. So it is the shares' interval about the
   split, with the bound about one place, from its lower end, to the same interval about the
   walk's start, to its upper end. On #903's flight it runs from 89.58 s to 108.34 s. Where the
   split is right, the interval reaches on to just before the walk's start, as the bound reaches
   it: it gives up width for holding either place.
3. **The reading** stays the split, as [ADR-366][adr-366] sets it, and so do the opening's
   height and each leg's rate.
4. **The legs' rates.** The legs meet at the split, so the stretch between the two belongs to the
   main's. Where it falls slower than the rest of the drogue's leg, as in #1593's shape, the
   drogue's rate reads fast; where it falls faster, slow, the flattering side for a
   descent-energy figure. The main's rate errs the other way round. No bound is added to either
   rate: which way each errs depends on the stretch, and the page says so.
5. **Tests.** `a_split_before_the_walks_start_is_the_slow_stretchs_start` pins the issue's
   sequence (split 230, walk 355). `a_slow_stretch_in_the_drogues_leg_keeps_the_opening_in_its_bound`
   flies the shape (a drogue at 18 m/s slowed to 6 m/s for 8 s, then 18 m/s for 3 s before the
   main) at 20 Hz smooth, in 0.3048 m steps and at 5 Hz: the opening lies within its bound, the
   interval's upper end within 1 s of it, and the drogue's rate reads more than 0.5 m/s faster
   than the true rate from apogee to the opening. Without the added time the test fails: the
   opening reads at the slow stretch's start, about 11 s early, against a bound of about 1 s. The
   #903 flight's test pins its bound at 19.21 s and its interval, and checks the bound reaches the
   walk's 108.45 s.

**Consequences.**

- **The synthetic census** ([report][report]): the main opening's mean bound over its 2,843
  readings goes from 2.03 s to 2.05 s. On the tuning flights logged at 5 Hz in 10 Pa steps, the
  largest share of its bound an opening's error took falls from 0.833 (flight 38) to 0.528
  (flight 162): the split and the walk differ on flight 38, whose bound now holds both. No count of readings given or withheld moves. `cargo xtask logs --check` and
  `cargo xtask real-flights --check` reproduce their committed reports.
- **A wide bound is a finding.** Where a swing or a slow stretch sits far from the split, the bound
  is that wide; it says the log can't place the opening closer, not that the reading is wrong.
- **Still outside it:** a canopy's filling time
  ([#779](https://github.com/nrdptel/fusionspace-eridanus/issues/779)), as before.
