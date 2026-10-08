# ADR-068: OpenRocket's flights of the public designs, and what its metric words mean (2026-09-25)

- **Status:** accepted
- **Summary:** OpenRocket's flights of the public designs, and what its metric words mean

**Context.** M2.2d asks for the public designs that fly in a report against OpenRocket (apogee,
largest speed, stability margin), with Loft lessons L80 and L81 live. Nothing yet asked OpenRocket
to fly a design, and hpr had no written meaning for OpenRocket's summary words. L80 is Loft's
finding that one word can be a different quantity in another tool or version; Loft took OpenRocket
24.12's meanings from its source, which this project may not read. L81 is Loft scoring a metric as
0 when its event never happened. hpr has no stability-margin output yet (M1.10), so hpr's side of
the report needs its own pull request.

**Decision.**

1. **M2.2d splits in two.** d1 is OpenRocket's side: the flights, and a written definition for
   each summary word, with L80 and L81. d2 is hpr's side, carrying the parent's *done when*
   unchanged.
2. **OpenRocket flies every public configuration in calm air.**
   `validation/oracles/openrocket/flights.py` flies every motor configuration of the jar's 17
   examples and the seven Loft demos. Each flight takes the launch conditions of the design's first
   stored simulation, with no wind and no turbulence (the average wind model on all), so it can be
   repeated and flown by hpr. Extensions are not carried over. The conditions are recorded with
   each flight. Of the demos, only `demo-payload-separation.ork` has a motor OpenRocket finds, and
   `demo-quirks.ork` does not open (a booster set it refuses): 57 runs. One, *Pods--powered with
   recovery deployment* `[C6-7; 2× A3-4, B6-0]`, is aborted at 1.81 s ("Stage began to tumble under
   thrust"); its figures are where it stopped, so it is recorded as aborted and is no reference,
   which leaves 56 complete flights. One more flight, the simple example with its parachute set
   never to open, gives a complete flight with no deployment. The record is committed, as the
   automatic-radius record is, because every design is public; a second run is byte for byte the
   same, and a test holds the scripts' hashes to the committed files.
3. **Each summary word is measured, not read.** The record keeps each word beside the quantities
   of the time series it could mean, and the steps either side of every event with their time,
   speed and altitude. The test holds OpenRocket 24.12's definitions on all 56 complete flights, to
   1e-9:
   - the apogee, largest speed and largest Mach number are the peaks of their columns;
   - the time to apogee is the time of the highest step, not the apogee event (13 differ); the
     flight time is ground hit's, the last step;
   - the rod-clearance speed is at the first step past the rod's length (the step before is short
     of it on all 56), which is 0.06% to 7.50% above the speed at the rod's end;
   - the deployment and ground-hit speeds are the total velocity interpolated linearly at the
     event; 53 deployments fall between steps, and the deployment speed is the **last**
     deployment's (17 flights have a different first; on one the last is the slower);
   - the stability margin, which has no summary word, is the column at rod clearance, exactly
     (CP − CG) ÷ reference length (Niskanen 2009, p. 12), with OpenRocket's default reference.
4. **One word has no definition.** The largest acceleration is the peak of total acceleration
   before the first deployment (56 of 56; over the whole flight it differs on 15). But the optimum
   delay is not the apogee event less the last burnout on 15 flights, nor, on the same 15, the
   apogee event of the same configuration flown again with nothing deployed, which the record also
   holds. Those 15 are exactly the flights on which a recovery device deploys before apogee (an
   early ejection charge alone, on 5 others, changes nothing), and the re-flights' own optimum delay
   is their apogee event less their last burnout on all 56. So an early deployment changes the
   figure, but how is not measured; it is withheld: M2.2d1's
   *done when* asks each word to be defined **or** withheld with the flights that rule out its
   candidates, so the gap stays visible rather than guessed.
5. **Definitions are per tool and version.** `hpr_validate::flight_metrics::definition` gives
   OpenRocket 24.12's and RocketPy 1.13.0's (at the centre of dry mass, with the rail exit found
   between steps, ADR-021), and `None` for any other version, read from a `.ork`'s `creator`. No
   older OpenRocket jar is pinned, so its words are withheld rather than read as 24.12's.
   OpenRocket does not document which point its speeds belong to, so its point is *unstated*.
6. **A metric is never scored when an event is missing** (`flight_metrics::compare`). OpenRocket
   writes `NaN` exactly when the event is missing. When neither flight had the event, the metric is
   withheld; when only one did, the comparison fails, since the flights disagree. A value from hpr
   that is not a number fails too, and every metric of an aborted reference is withheld.

**Consequences.** L80 and L81 are live, owned by M2.2d1. The record is the reference for M2.2d2,
which flies hpr in the recorded conditions and compares the apogee, largest speed and margin at rod
clearance by these definitions: hpr's margin must be taken at the recorded rod-clearance step (its
time, Mach number and mass), and its speed there at the step past the rod, not at the rod's end.
The motor's centre of mass (ADR-067) meets the margin there. Stored results in files written by
other OpenRocket versions stay readable, but their summary words are withheld until another
version's jar is pinned and measured.
