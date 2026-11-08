# ADR-327: M7.3d5, the fall must hold and still fall (2026-11-07)

- **Status:** accepted; carries out [M7.3d5][m7-3] under [ADR-326][adr-326] §1 and closes [#1307][i1307].
- **Summary:** The boost's cut waits until the climb speed's fall has held 0.75 s and the speed is the lowest of those 0.75 s. On the public logs no cut falls inside the burn now (4 of 5 did): Bella Lui and NDRT 2020 are cut after their burns, and the other 5 reach the transonic limit first. Set A's cuts come 1.08 s to 2.30 s after burnout, inside the 2.83 s bound; 1,787 synthetic logs with a dip in the boost are each cut after burnout or refused as transonic. The collection: 26 cut; none of the 20 whose burn and liftoff are known is cut inside its motor's burn.

[adr-325]: 0325-m7-3d2-the-public-flights-prediction-from-the-boost.md
[adr-326]: 0326-m7-3d3-in-four-the-halves-sealed-first.md
[i1307]: https://github.com/nrdptel/fusionspace-eridanus/issues/1307
[m7-3]: ../ROADMAP.md#phase-5-flight-data-and-forensics

**Context.** The cut was the first sample where the climb speed, a line fitted over the last
0.5 s, fell 10% and at least 10 m/s below its top. On the public flights' barometers that cut 4 of
5 logs inside the burn ([ADR-325][adr-325] §5, [#1307][i1307]). Measured once on those logs with no
cut taken, each fall that far during a burn recovered within 0.48 s of its first sample, and the
deepest went 63.2 m/s below its top. NDRT 2020's coast shows falls of up to 0.45 s, and a new top
between them.

**Decision.**

1. **The fall must hold.** The cut is at the first sample where the fall has held for
   `HOLD_S` = 0.75 s, about half again the longest fall measured inside a burn. The cut is decided at
   that sample, so the log is cut there: nothing later is read.
2. **And still be falling.** At that sample the speed must also be the lowest of the last 0.75 s.
   A dip's catch-up can read above the true speed; the fall back from that false top can outlast
   0.75 s, but on the synthetic dips it leveled off while the motor burned. With the hold alone, 416 of 778 synthetic dipped logs whose falls outlast 0.5 s were cut
   in the burn (run once); with both conditions none is.
3. **The test.** On each of set A's first 200 synthetic flights that is cut, a dip
   `depth · sin²(π (t − start) / span)` is taken off the height: start at ignition, 0.3 or 0.6 of
   the way through the burn, span 0.2 to 0.6 s, depth 10, 20 or 40 m. Every dipped log whose speed
   falls by the cut's amount before burnout must be cut after burnout, within the 2.83 s bound,
   before apogee, or refused as transonic. Among the dips whose falls each last 0.5 s at most, as
   the public logs' did, the falls must reach 63 m/s and 0.45 s (the public logs': 63.2 m/s,
   0.48 s). Measured: 1,787 dipped logs whose
   speed falls by the cut's amount, 1,635 cut 0.14 s to 2.30 s after burnout, 152 refused as
   transonic; 1,009 with every fall within 0.5 s, those falls to 0.5 s and 295 m/s.
4. **The counts.** `from-boost` gives the cuts inside the thrust file's burn, `collection-boost`
   those inside the motor's burn, the end of the `.ork` configuration's thrust curve after the log's
   liftoff (on a design with several motors, the longest), and how many it can't tell.

**Results.** Public logs: Bella Lui cut at 3.96 s (burn 2.43 s after liftoff at 0.21 s), predicted
+1.02% against the forward simulation's +0.86%; NDRT 2020 cut at 7.39 s, 6.90 s after liftoff,
predicted −3.58% against +10.40%. Prometheus, Juno III and Lince, once cut on a dip, now reach
227.7 m/s first and are refused. Collection: 26 cut (32 before), 36 refused at the transonic limit
(30 before), 2 for a sparse log; of the 26, none of the 20 whose burn and liftoff are known is inside its burn, and for 6 it
isn't known.

**Consequences.** Cuts come later. On set A they come 1.08 s to 2.30 s after burnout, from 0.33 s
to 1.48 s: the range's ends 0.75 s and 0.82 s later. The public cuts moved more, 1.50 s
(Bella Lui) and 2.55 s (NDRT 2020): Bella Lui's earlier cut came before its burn's end, NDRT
2020's before a later top in its coast. NDRT 2020's fitted
speed reads a new top 1.4 s into its coast, and its cut, about 3.5 s after the burn ends, waits for
the fall from it: past the 2.83 s that bounds set A's. A later cut leaves the fit more of the coast, so its predictions are not the
same test as before; the 2.83 s bound the set A test holds was set for the cut without the hold and
is kept. One bad reading moved the cut on 8 of the 255 spiked logs of set A's cut flights (785 spiked in
all), by 0.70 s at most (0.05 s before); why a
spike moves them that far isn't traced. Set B's spread was measured before the hold only.
A log whose motor's thrust falls below weight and drag well before burnout is still cut before its
burnout. The collection's errors by halves (M7.3d6) start from these 26 cuts.
