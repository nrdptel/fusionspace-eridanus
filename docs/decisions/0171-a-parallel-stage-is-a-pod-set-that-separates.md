# ADR-171: A parallel stage is a pod set that separates (2026-10-05)

- **Status:** accepted
- **Summary:** M4.5m: hpr's design gains a parallel stage, a stage laid out as a pod set beside a body tube of an axial stage before it, weighed to its own stage and dropped by its own separation as a part that tumbles. The `.ork` reader reads OpenRocket's `<parallelstage>` on a rocket of one axial stage; a motor in it lights as one in that stage does, and a parallel stage with no motor that lights stays on. OpenRocket's *Parallel booster staging* flies both configurations as saved, each apogee within 1.3% of OpenRocket's

**Context.** M4.5 asks that every in-scope OpenRocket example fly as saved. After
[M4.5l](0170-a-packed-part-wider-than-its-bore.md), OpenRocket 24.12's *Parallel booster
staging* was refused in both configurations: one for a motor in a part hpr doesn't read, the
other for an airframe not read as written. The file's sustainer is one axial stage: a nose cone
and a body tube holding an I115W. Inside the tube sits a `<parallelstage>`, *Booster Set*: two
copies of a booster tube with fins and an E12, placed around the tube like a pod set
(`instancecount`, `radiusoffset`, `angleoffset`, an axial position), and a separation of its own,
`ejection` with no delay. [ADR-058](0058-what-a-ork-holds-that-hpr-does-not-model-kept.md) kept
the element whole for writing the file back, unread.

What OpenRocket records ([`openrocket-flights.json`](../../validation/fixtures/ork/openrocket-flights.json)):
in configuration 1, `[I115W-10; 2× E12-0]`, both motors light at 0 s, and the E12s burn out, fire
their charges and separate at 2.44 s. Configuration 2, `[I115W-10; None]`, has no motor in the
boosters, no `STAGE_SEPARATION` event and one flight branch: the boosters ride to apogee and land
with the sustainer.

**Probe.** A scratch copy with the `<parallelstage>` renamed to a `<podset>` was read as pods of
the sustainer's stage. Configuration 2 flew within 0.7% of OpenRocket's apogee, but in
configuration 1 the E12-0s' charges were then the sustainer's charges: they opened its
parachute at 2.44 s, under power. A parallel stage needs a stage of its own.

**Decision.**

1. **The design holds a parallel stage as a stage.** `Stage.parallel` is
   `Some(ParallelStage { on, position, pods })`: the body tube it hangs on (by id, in an axial
   stage before it), where along that tube, and the copies, as a pod set's count, radial offset
   and angle. The layout builds it as a pod set under that tube, its parts tagged with its own
   stage index, so its mass, inertia and ends count to its stage and not to the tube's. A placement
   `after` a sibling is refused: a parallel stage sits along its tube. `PlacedStage.hung_on`
   names the stage it hangs on.
2. **A separation drops what hangs together.** Separations stay keyed by stage index: one after
   stage *k* drops stages *k*+1 on. A parallel stage's carrier is its `hung_on` stage, an axial
   stage's the axial stage before it, and every stage dropped must have its carrier in the drop,
   or be the first dropped. A separation that would drop a parallel stage hung on a stage the
   nose keeps together with a stage behind it is refused.
3. **The dropped stage is a piece beside its carrier.** Like a payload, it is hosted by the
   piece that carried it. Unlike a payload, it tumbles from the split: it flies as a point with
   only its devices' drag, as every dropped stage does in hpr
   ([ADR-165](0165-an-unpowered-separation-in-hpr-sim.md)).
4. **The reader reads a parallel stage on a rocket of one axial stage.** OpenRocket numbers a
   parallel stage right after the stage holding it, depth-first, so on one axial stage its
   number is its place in hpr's list. A rocket of more than one axial stage keeps its parallel
   stages unread, as before, and says so by name. So does a parallel stage inside a pod or
   another parallel stage, one with no nose cone, body tube or transition in it, and one placed
   `after` its neighbour. One directly under `<rocket>` stays in the tally of tags hpr does not
   read (OpenRocket refuses that file).
5. **Ignition beside a stage.** "The stage below" is the next axial stage. A motor in a parallel
   stage set to `automatic` lights at launch when its carrier is the last axial stage, as
   OpenRocket lit the E12s and the I115W together. `launch` and `never` are read as written. Any
   other event in a parallel stage is refused by name, and so is `upperignition` as its
   separation event: hpr has no reading for either.
6. **A parallel stage with no motor that lights stays on.** Its `burnout` or `ejection`
   separation never comes, as OpenRocket's configuration 2 shows. Axial stages keep the rule
   they had.
7. **Overrides near a parallel stage.** A mass override covering what its tube holds, or the
   tube's whole stage, and a drag override on a parallel stage or covering the stage it hangs
   on, are refused: none says whether it covers the parallel stage, and no probe has measured
   OpenRocket's reading. An override on the parallel stage itself is read as its total, every
   copy's, as a pod set's is; that is hpr's reading, not a measured one.
8. **The writer puts it back inside its tube.** The stages on a tube are written in order, at
   the places they were read from where something kept lies under them, and after the tube's
   parts otherwise. When only some of them keep something, which one held it isn't known: the
   writer says so, as it does for a parallel stage on no body tube of an axial stage, which it
   leaves out. Read, written and read again, the design is the same
   (`parallel_stages_round_trip_inside_their_tube`, `a_parallel_stages_kept_parts_and_its_place`).

**Measured.** [`openrocket-flights.md`](../../validation/reports/openrocket-flights.md), each
flight under OpenRocket's stored conditions, recovery as saved:

| Configuration | Apogee, OpenRocket (m) | hpr (m) | Δ | Largest speed Δ | Margin off the rod (cal), OR / hpr |
|---|---:|---:|---:|---:|---:|
| `[I115W-10; 2× E12-0]` | 1123.6 | 1137.5 | +1.23% | +3.25% | 2.462 / 2.461 |
| `[I115W-10; None]` | 851.8 | 857.6 | +0.68% | +1.56% | 2.910 / 2.907 |

At rod clearance the masses agree to 0.7 g (0.06%), the centres of mass to 0.2 mm and the
centres of pressure in every printed digit, and both descents meet their targets. `hpr sim` on the
file, with no flags, flies configuration 1 with both motors lit at launch and drops *Booster
Set* at 2.44 s, at the E12s' charge, OpenRocket's 2.44 s. Configuration 2 flies as one stack.
The 3.25% in largest speed is not traced: it is inside the record's spread with no named cause
(−0.69% to +6.21%).

`cargo xtask ork-cli` now flies 135 of the survey's 170 configurations as saved (131 before),
53 of OpenRocket's 56 examples (51), and refuses 26 (30). The other two new ones are the private
library's copy of the same example, so they are no independent evidence. No other
configuration's outcome changed.

**Why a stage, not a pod set.** The probe above is the reason: a booster's charge is its own
stage's. A pod set in the sustainer's stage would also never drop.

**Left open.** Parallel stages on rockets of more than one axial stage stay unread: OpenRocket's
numbering interleaves them with the axial stages, and no example or library design has one to
check a reading against. A dropped booster tumbles as a point from the split, where a real one
flies nose-first for a while, so its peak and landing are rough, as the CLI's note says.
