# ADR-165: A `.ork`'s separation with nothing left to burn flies in `hpr sim` when each part has drag from the split (2026-10-04)

- **Status:** accepted
- **Summary:** M4.5g3: a `.ork` configuration whose only separation comes with nothing ahead of it left to burn, such as a payload dropped at its booster's ejection charge, is read and flown. From the split every part flies as a point with only its own devices' drag, ADR-014's model unchanged, so `hpr::ork::tumbling` flies it only when the part that keeps the nose has a device of its own open by the split, and refuses it by name otherwise. `lowerstageseparation` opens a device on the split's own trigger. OpenRocket's *Deployable payload* and *ARC payload rocket* fly: the payload's apogee within 1.1% of OpenRocket's record and every descent within ADR-153's targets. The coast with no drag that the rule refuses would put one payload 4.00% above OpenRocket's own free flight of it

**Context.** [ADR-144](0144-the-2026-10-03-planning-review-leaner-bookkeeping.md) §8 asks that every
in-scope OpenRocket example fly as saved in `hpr sim`. Two did not, in any configuration: the
*Deployable payload* (five) and the *ARC payload rocket* (one). In each, a finless payload, a nose
and a short tube, rides on a finned booster, and the booster drops away at its motor's ejection
charge. [ADR-076](0076-a-ork-files-ignitions-and-one-powered-separation.md) refused such a
separation, as it can come before apogee with nothing left to burn (issue
[#184](https://github.com/nrdptel/hpr-sim/issues/184)). There hpr's flight flies every part as a
point with only its devices' drag ([ADR-014](0014-separation-bodies-their-masses-and-their.md)), so
a part with nothing open would coast as if in a vacuum.

What the files and OpenRocket's record say:

- Each payload's parachute opens at `lowerstageseparation`, OpenRocket's "Lower stage separation",
  with no delay. The booster's opens at `ejection`, the same instant as the split.
- OpenRocket 24.12's record (`validation/fixtures/ork/openrocket-flights.json`) opens the
  payload's parachute 0.001 s after the separation in all six configurations, so the payload
  never flies free. The record holds the branch that keeps the nose.
- Two of the six separate before OpenRocket's apogee: C6-3 at 4.86 s, rising at 27.9 m/s, and
  C6-5 at 6.86 s, at 6.7 m/s.
- The record's flight of each configuration with every device set never to open still separates.
  There the payload flies on its own airframe: C6-3's reaches 258.5 m, where C6-7, the same rocket
  separating after apogee, reaches 266.6 m.

**Decision.**

1. **Read it.** The `.ork` reader turns a separation with nothing ahead of it left to burn into a
   `Staging` when it is the configuration's only one and its time is known before the flight
   (`hpr_io::ork::staging`). Beside another separation it stays refused by name: each of several
   must hand the flight on to a sustainer
   ([ADR-160](0160-several-powered-separations.md)). A separation at or after apogee is still left
   out, and the stack flies whole (ADR-076).
2. **`lowerstageseparation`.** A device set to it opens on the trigger of the split right behind
   its stage, its delay after, when that split comes with nothing ahead of it left to burn
   (`hpr::ork::separated_recovery`). The record's 0.001 s is OpenRocket's next step. A sustainer's
   split under power, a split further back, or no split at all still refuses the word: a device
   opening under power is not something the flight flies, and the meaning of the word for a split
   further back was not probed.
3. **Drag from the split, or refused.** `hpr::ork::tumbling` flies a lone split with nothing left
   to burn only when the part that keeps the nose has a device of its own whose trigger's time,
   known before the flight, is at or before the split's. Otherwise it refuses
   (`RecoveryRefused::Coasts`), naming the part and the split's time. The part is not tumbled
   instead: a finless payload on its own is unstable, and nothing measured says how it flies. The
   dropped part tumbles from the split until its own first device opens
   ([ADR-159](0159-a-powered-separation-in-hpr-sim.md)). The test is `Separation::powered_at`,
   the flight's own, and `Trigger::known_time_s`. A device at apogee that would open before the
   split is still refused, as its time is not known before the flight.
4. **The flight's apogee is the payload's.** When the stack comes apart before its own apogee,
   the flight's summary takes the apogee of body 0, the part that keeps the nose
   (`hpr_sim::metrics`), and only at a separation: an ejection's body 0 may be the nose cone
   alone, so one before apogee leaves the summary with none. `hpr sim` lists that part's events
   after the stack's, takes the descent line from its landing, and notes the split "with nothing
   left to burn". A dropped part that peaks higher gets a note too; C6-3's booster, tumbling,
   peaks 0.9 m above the payload.
5. **The comparison.** `cargo xtask ork-flights` flies the climb compared whole for such a
   configuration, as its own parachute already sets apart a flight whose parachute opens before
   apogee. It flies the split with the descent, holding the payload's apogee to 5% and its descent
   to ADR-153's targets. A new table, *Separations with nothing left to burn*, has both. It also
   flies the refused flight, the payload coasting from the split with no drag to its own apogee,
   against the record's flight with nothing deployed.
6. **Design checks stay.** The *Deployable payload*'s payload mass and parachute are 25 mm across
   in a 21 mm bore. hpr's design checks
   ([ADR-155](0155-design-checks-that-match-reality.md)) find two errors there, so `hpr sim`
   flies it only with `--accept-design-errors`, and says why. The check is right: the parts can't
   fit as written. ADR-155's finding that OpenRocket's examples raise no design error held over
   the configurations hpr flew then. The *ARC payload rocket* raises none.

**Result.** `cargo xtask ork-flights`, the payload's flight with the file's parachutes, against
OpenRocket 24.12's record (each apogee from where its tool's flight starts):

| design | motors | split, OR / hpr (s) | before OR's apogee | apogee OR / hpr (m) | Δ | landing speed Δ | flight time Δ | targets |
|---|---|---|---|---|---:|---:|---:|---|
| ARC payload rocket | F50T-9 | 10.43 / 10.43 | no | 452.2 / 457.0 | +1.05% | +0.00% | +0.32% | met |
| Deployable payload | A8-3 | 3.73 / 3.73 | no | 32.2 / 32.2 | +0.09% | +0.00% | −0.05% | met |
| Deployable payload | B4-4 | 5.03 / 5.03 | no | 96.7 / 96.5 | −0.20% | +0.00% | −0.01% | met |
| Deployable payload | C6-3 | 4.86 / 4.86 | yes | 233.2 / 233.3 | +0.04% | +0.00% | +0.17% | met |
| Deployable payload | C6-5 | 6.86 / 6.86 | yes | 265.3 / 264.4 | −0.32% | +0.00% | −0.21% | met |
| Deployable payload | C6-7 | 8.86 / 8.86 | no | 266.6 / 265.3 | −0.48% | +0.00% | −0.71% | met |

**What the refused flight would get wrong.** Flown on from C6-3's split with no drag, the payload
reaches 268.8 m, against 258.5 m for OpenRocket's flight of it on its own airframe: 10.3 m, or
4.00%, high. It even climbs 3.5 m above the whole stack's 265.3 m, where OpenRocket's payload
falls 8.1 m short of its whole-stack flight. On C6-5, split at 6.7 m/s, the coast is 0.43% low,
as the climb left is short. On these two, the error grows with the speed at the split. A real payload whose
parachute opens late would land later and further away too; the record holds no such flight, so
that is not measured.

The climb compared, the whole stack's, reads C6-3's apogee 13.75% above OpenRocket's record. That
record's apogee is under the payload's parachute, opened 2.12 s before the apogee of the flight
with nothing deployed. Against that flight it is +2.63%, so the census accepts the row with its
cause named. The private library's flights are unchanged.

**Consequences.** Both examples fly in `hpr sim`, the *Deployable payload* with
`--accept-design-errors`, offline once their motors are fetched. The booster's flight after the
split is compared with nothing, as OpenRocket's record keeps only the payload's branch (#179). A
payload whose parachute opens after the split, or that has none, is refused by name, though the
library still flies ADR-014's coast for a program that asks for it. A separation with nothing left
to burn beside another, and `lowerstageseparation` on any split but the one right behind the
device's stage, stay refused. `hpr sim`'s figure and exported rows follow the stack, so they
end at the split, and a note says so: the parts' descents keep only their events.
