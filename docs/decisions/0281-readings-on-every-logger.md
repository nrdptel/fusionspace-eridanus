# ADR-281: Readings on every logger: one height track, a pad its samples can vouch for, and a top that must be a coast's (2026-10-18)

- **Status:** accepted; ships [M7.2b][m7-2b] as [ADR-280][adr-280] planned it; retires [ADR-251][adr-251] §6 (readings on PerfectFlite logs only); meets [#646][i646] and [#702][i702]
- **Summary:** The readings (liftoff, apogee, the top speed, landing) now come from every logger that records a barometric altitude or a pressure, each within a bound its error stayed inside on the synthetic flights' tuning set, logged at every rate from 500 to half a sample a second, at every step to 10 m (33 ft) or 100 Pa, and from cropped starts, or withheld with one of 23 named reasons. A GNSS track gives none. Three rules carry most of the weight. The pad is taken only as far as its samples can vouch that the rocket was still on it. A track whose heights are above the pad only by a column's word must start near its zero. A top that isn't a coast's is withheld: one the track fell from and climbed back to, or a spike that climbs in and falls out faster than physics allows. On the 1,058 records the sample logs hold, apogee is given on 594, liftoff on 555, the top speed on 354 and landing on 473, and every reason has a case.

[m7-2b]: ../decisions-and-roadmap.md#m7-2b
[m7-2c]: ../decisions-and-roadmap.md#m7-2c
[adr-251]: 0251-flight-log-importers-in-two-increments.md
[adr-274]: 0274-the-rest-of-the-importers-planned-teensy-logger-first.md
[adr-280]: 0280-m7-2-in-seven-the-synthetic-flights-first.md
[i646]: https://github.com/nrdptel/fusionspace-eridanus/issues/646
[i702]: https://github.com/nrdptel/fusionspace-eridanus/issues/702
[i761]: https://github.com/nrdptel/fusionspace-eridanus/issues/761
[i762]: https://github.com/nrdptel/fusionspace-eridanus/issues/762
[page]: ../physics/log-readings.md
[anomalous]: ../research/anomalous-public-flights.md
[heights]: ../research/altimeter-height-formulas.md

**Context.** Until now the readings ran on PerfectFlite logs only ([ADR-251][adr-251] §6), with marks
tuned on Debrief's samples: liftoff half a foot above the pad, apogee the filtered peak. ADR-280's
M7.2b row asks for every family with an altitude, pressure or GNSS track, each reading within a bound
held on the tuning set at each family's rate and step, or withheld by name; #646's two mapping
mistakes noted, tested both ways; each reading's method and provenance; a corpus case for every
reason; and `hpr analyze` and Python reading each family. Logged at other rates, the old marks failed
in measurable ways: liftoff read half a foot above the pad came up to 6 s early on a noisy 500 Hz
barometer, and a log that starts at launch, as many loggers write them, has no pad at all.

**Decision.**

1. **One height track.** Each reading comes from the first of these the log holds: the logger's
   barometric altitude, a pressure turned into a height by the 1976 standard atmosphere, as every
   documented logger does it ([the height formulas][heights]), a barometric altitude in a channel,
   or a GNSS altitude. A GNSS track gives no reading (`receiver_lag`, `receiver_dropout`): a
   receiver's filter lags a climb and overshoots its top, and its fix drops out at liftoff. A clock
   that doesn't increase gives none either (`clock_out_of_order`).
2. **The pad its samples can vouch for.** The climb is the first filtered sample `2(a + s)` above the
   log's start, where `a` is the 2 m liftoff allowance and `s` the track's step; the pad is the median
   of the samples to the last within half that of the start. A rocket leaving the pad at 2 g, the
   least the NAR and Tripoli codes' 3:1 thrust to weight leaves, and already moving at `v` when the
   log starts, would have climbed out of that band unless `v ≤ (rise − A t²/2)/t`. So the samples
   bound how far up it may already be, `v²/(2A)`, and that much enters the liftoff, apogee and
   landing bounds. Where it could pass the allowance, the pad is short (`short_pad`): a track on the
   logger's own zero reads its heights from that zero, and any other withholds them.
3. **A start near the zero (#646).** A track that is above the pad only by a column mapping's or a
   header's word must start within 19.9 m of its zero: an hour at the National Weather Service's
   *pressure rising rapidly* rate (0.06 inHg an hour) at sea level's density, plus the 3 m noise.
   Further off, it is withheld (`starts_off_the_pad`), as an altitude above sea level mapped as one
   above the ground reads. The hour is HPR Sim's choice.
4. **A top that is a coast's.** Three checks, each tested both ways.
   - *The clock (#646).* A coast loses `g t²/2` in the last `t` before its top, 78 m in 4 s. A track
     that loses no more than twice its noise and step is on a clock in another unit, such as
     milliseconds read as seconds, or isn't a height (`slower_than_free_coast`).
   - *A fall and a climb back.* A track that falls more than twice its noise and step below a peak in
     the 4 s before its top, then climbs to it, holds a top that isn't the coast's
     (`disturbed_top`).
   - *A spike* ([#702][i702]). Slowing at gravity and drag, a coast climbs no more than `g Δ²` in its
     last second, while drag's upward share stays under g, which holds for any ballistic coefficient
     over 150 kg/m² even at 120 m/s across; a 29 mm rocket of 0.3 kg has about 800. And nothing
     falls from its top faster than gravity pulls it. Allowing for the median's window `w` and the
     noise, a fall from rest can show at the filtered top only after it has gathered
     `v = √(2g(2(N + s) + g w²/8))`, and then falls no more than `v L + g L²/2` over the windows'
     span `L`. A track that breaks both limits, past twice its noise and step, holds a spike at its
     top, and its apogee, top speed and landing are withheld (`disturbed_top`). Either half alone
     isn't: a parachute open before apogee slows the climb into the top but not the fall, and a
     charge that raises the bay's pressure at the top drops the track after the coast's own climb.
     Both tops are the vehicle's.
5. **Bounds held on the tuning set.** Each tuning flight is read on its own logger's log and, where
   it has a barometer, on the same barometer written again at 6 rates by 6 ways of writing it, and
   two recordings cropped to start 0.3 s before liftoff, 0.1 s after it, and 0.5 s after it behind
   the logger's zero, as a logger that records from launch writes them: 43 logs a flight. `cargo xtask synthetic` fails unless every reading given holds. The
   worst share of its bound any reading reached is 0.717, an apogee at 20 Hz.
6. **Method and provenance.** Each reading names the track it came from, how it was read, and
   whether it was measured or derived. The heights and times are measured. The top speed is derived
   from them, and withheld from 227.7 m/s, Mach 0.8 in the coldest air the bounds allow below 20 km,
   where a barometer's port may read the flow over the nose. *Clipped* waits for an accelerometer's top acceleration, at
   [M7.2c][m7-2c].
7. **A case for every reason.** `cargo xtask logs` counts each reading given and withheld, by family
   and by reason, and names a case for every reason: the corpus's first record withheld for it,
   by anonymized id, or, where the corpus has none, an invented sample committed under
   `validation/fixtures/logs/readings/`. The survey reads each invented sample and fails unless it
   withholds a reading for its reason. Five stand in today: `bad_record`, `faster_than_free_fall`,
   `sampled_too_fast`, `starts_off_the_pad` and `step_too_coarse`.
8. **Every surface.** `hpr analyze` reads every family with no design file, and Python's
   `FlightLog.read` and `FlightLog.readings()` read the same files into the same readings. Three
   kinds of file hold more than one record, and until now only the survey read them: a workbook
   of several sheets, an Entacore AIM raw dump of several recordings, and a Raven FIP export
   pasted into a workbook below no header. Each is now read one record at a time
   (`import::read_one`): `--sheet N` reads a sheet by its own format's reader, `.xlsx` or
   `.xls`, `--recording N` a dump's recording, and `--fip-header FILE` the FIP rows under the
   header line the flyer states, as [ADR-274][adr-274] decided. With no choice, a file of one
   record that reads is read, and any other is refused, naming the sheets or recordings to
   choose from. A zip archive stays a container: its files are logs of families already read,
   and are given one by one once unpacked. Invented files of each kind, committed under
   `validation/fixtures/logs/several/`, are read on both surfaces and refused both ways.

**Rejected, with the measurement that ruled each out.**

| Rule | Ruled out by |
|---|---|
| Back the pad off until no sample could be a moving rocket | withheld liftoff on 56 of the 57 Raven `.FIPa` records, whose pads hold 0.3 to 0.5 s |
| End the pad 1 m above the start | read liftoff 5.3 s early on a tuning flight logged at half a sample a second |
| A liftoff allowance of 1 m | 6.8 s early at half a sample a second on one tuning flight, against a 4.5 s bound |
| No pad error in the bounds | apogees on the logs cropped to start after liftoff fell outside their bounds |
| The coast check at its physical `g t²/2` | refused up to 29 of 171 genuine tuning flights on one kind of log |
| A fast climb into the top alone | withheld 93 corpus tops, many falling only 1 to 8 m in the next second, as an early parachute does |
| A fast fall from the top alone | 26 of its 75 corpus tops climbed in at a coast's rate: a charge's spike down after a true top |

**Consequences.**

- Two of #702's public logs, the transients of [anomalous public flights][anomalous] §2, now read
  right. The forum TeleMetrum log reads its coast's peak: the one-sample spike after the charge, which
  the import's raw peak took, is removed by the median. The Blue Raven breakup log is withheld by the
  spike check. Synthetic logs of both shapes are committed tests, because neither log can be.
- A spike that climbs into the top no faster than a coast can, at most `g Δ²` plus twice the noise
  and step in its last second (16.4 m at 20 Hz on a PerfectFlite), is still read as the top, however
  high it stands: the check is one-sided by design. No synthetic flight carries a charge's transient, so the bounds
  were never held against one ([#761][i761]).
- Some tops withheld as `disturbed_top` are the vehicle's own: a pressure dip in the coast's last
  4 s, or an early parachute followed by a charge at the top. The error is conservative, and how
  many of the corpus's 76 are such tops isn't measured ([#762][i762]).
- The bounds hold on the tuning set's six logger profiles. A logger noisier than they are, or a
  rocket slower off the pad than 2 g, can be read outside them; [the readings' page][page] says so.
