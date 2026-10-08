# ADR-172: A stage's first burnout, and a booster dropped still burning (2026-10-05)

- **Status:** accepted
- **Summary:** M4.5n: a `.ork` stage whose motors sit in several mounts burns out when its first motor does, by the motors' ignitions and curves, as OpenRocket 24.12 flies it (measured by a committed probe). The stage above lights there, and the stage separates there, when the file says so. A split at that first burnout may drop the stage's other motors still burning. The flight then leaves their thrust out of the dropped part's flight as a point, and says how much impulse that is. OpenRocket aborts its own flight of *Pods--powered with recovery deployment*'s first configuration, so hpr's flight of it is checked against OpenRocket's up to the abort: at the separation and at OpenRocket's last row, within 5%

**Context.** M4.5 asks that every in-scope OpenRocket example fly as saved. After
[M4.5m](0171-a-parallel-stage-is-a-pod-set-that-separates.md), one configuration of an in-scope
example was still refused: *Pods--powered with recovery deployment*'s first, `[C6-7; 2× A3-4,
B6-0]`. Its booster stage holds a B6-0 in its main motor tube and an A3-4 in each of two pod
tubes, all lit at launch. The sustainer's C6-7 lights at the booster's `burnout`, and the booster
separates at its own `burnout`. hpr read a stage's burnout only from one mount, so it refused the
configuration: *which burns out first is not worked out*
([ADR-168](0168-pods-powered-flies.md) §6). OpenRocket's own flight of it
(`validation/fixtures/ork/openrocket-flights.json`) aborts at 1.808 s: "Stage began to tumble
under thrust." With no apogee, that flight gave nothing to check an apogee against.

**What OpenRocket does.** A committed probe, `validation/oracles/openrocket/first_burnout.py`,
writes `validation/fixtures/ork/openrocket-first-burnout.json`. It runs OpenRocket 24.12 as an
oracle on the example's first configuration with nothing deployed, three ways:

| probe | main tube | pods | separation and sustainer ignition | the booster's other burnouts |
|---|---|---|---|---|
| as written | B6, burns 0.86 s | A3s, 1.01 s | 0.86 s | the A3s at 1.01 s |
| the main tube burns longest | C6, 1.86 s | A3s, 1.01 s | 1.01 s | the C6 at 1.86 s |
| the pods burn longest | B6, 0.86 s | C6s, 1.86 s | 0.86 s | the C6s at 1.86 s |

OpenRocket records one `BURNOUT` per motor. Both the stage's separation and the sustainer's
ignition come at the first burnout in the stage, whichever mount holds it, and not at the main
tube's or the last. After the split, the booster's branch records its other motors burning out
on their own curves' times. The saved file can't tell these readings apart, because its main tube
is also the one that burns out first. The second probe tells them apart. (The third puts an 18 mm
C6 in the 13 mm pod tubes. OpenRocket flies it, and only its event times are used.)

**Decision.**

1. **A stage's burnout is its first motor's.** When the motors of the stage below sit in more than
   one mount, a `burnout` ignition waits on the mount whose motor burns out first. Its burnout time
   is its ignition plus its curve's burn time, known before the flight. On a tie, the first mount
   in the file wins; no probe sets a tie, so that is hpr's choice. A stage's own `burnout`
   separation takes the same mount. One mount reads as before, needing no curve.
   `hpr_io::ork::staging::ignitions` does this in one pass from the tail forward, so each motor is
   worked out once: a motor waits only on stages further aft. (A first version worked each
   stage's first burnout out again for every motor above it, and took time doubling with each
   stage on a hostile file; found in review.) The mount is fixed when the file is read, so a
   Monte Carlo draw that changes burn times keeps it.
2. **A charge among several mounts stays refused,** by name: *which fires its charge first is
   not measured*. OpenRocket's label says "first ejection charge", but no probe has measured it.
3. **A split at a stage's own first burnout may drop the stage's other motors still burning,** as
   OpenRocket's does, when the split is powered: a motor ahead of it burns then, or lights then or
   later, as hpr's flight counts it. Each dropped motor must have lit before the split. A motor
   behind it still to light, or lit at the split itself, is refused; so is one burning in another
   stage behind, and any other split while a motor behind it burns (a time, an ignition, a
   charge). Those are not OpenRocket's first-burnout rule, and leaving thrust out of them could
   leave out most of a motor. The reader marks such a split `Staging::drops_burning`. A delay
   after the first burnout counts too, as hpr reads OpenRocket's `burnout` event; the probe
   measured only a split with none, so a delayed one is unprobed.
4. **The flight drops a burning motor only when told to.**
   `hpr_sim::Separation::drops_burning`, set by `Separation::dropping_burning`, is off by default.
   Off, the flight still refuses a separation while a motor of its aft body burns. On, a powered
   separation may drop a motor that has lit and still burns. Unpowered, or with a motor still to
   light behind it, it is refused either way. `hpr::ork::separation` sets it only for a staging
   the reader marked (rule 3), so a staging written by hand in a `.hpr` keeps the refusal unless
   it says so. The field is optional and false by default, so version 0.2 of the design format
   changes in place: no document written before could hold such a staging. A dropped part flies
   as a point with no thrust
   ([ADR-014](../DECISIONS.md#adr-014-separation-bodies-their-masses-and-their-descents-2026-09-17)),
   with its mass at the split, propellant left included. So it leaves out the impulse its motors
   had left. `BodyFlight::impulse_left_n_s` gives that: each lit motor's curve total less what it
   had given by the split. `hpr sim` names the motors and gives the number.
5. **A flight OpenRocket aborted is checked up to the abort.** `cargo xtask ork-flights` compares
   hpr's height above its start and its speed with OpenRocket's at two points: OpenRocket's first
   separation, and its last row. At the separation OpenRocket's is its row just after it, and
   hpr's is its separation event's sample, the whole stack's centre of mass at the split (the
   sustainer's sits about 0.1 m higher, under 0.5% of the height). The check needs OpenRocket's
   last row to be its highest point, whose height the record holds. Both separations must come at
   the same time, and each value
   within 5% of OpenRocket's: `ABORT_PERCENT`, the band the comparison with OpenRocket
   ([M2.2](../decisions-and-roadmap.md#m2-2)) gives an apogee before it needs a written cause.
   The report writes this as *Flights OpenRocket aborted*, with OpenRocket's cause and when hpr's
   angle of attack first passes 90°. The first table still withholds the apogee, largest speed
   and margin, which OpenRocket's aborted flight does not have. This is weaker evidence than an
   apogee: two points on the way up, the last where OpenRocket's flight is breaking up. The time
   OpenRocket aborts is its own call, too: its probe with nothing deployed aborts at 2.40 s, not
   1.81 s.

**Result.** `cargo xtask ork-flights`, in the record's conditions (a 0.15 m rod at 28.61° N, no
wind):

| | OpenRocket | hpr | Δ |
|---|---:|---:|---:|
| separation, s | 0.86 | 0.86 | 0 |
| height at the separation, m | 22.44 | 22.18 | −1.17% |
| speed at the separation, m/s | 46.08 | 46.06 | −0.04% |
| height at OpenRocket's last row (1.81 s), m | 85.0 | 84.0 | −1.18% |
| speed there, m/s | 76.8 | 79.9 | +4.02% |

The flight is met. hpr's sustainer turns over at 2.04 s, past 90° angle of attack. OpenRocket
called its tumble at 1.81 s. hpr's least static margin, in the weakest plane, is −4.21 calibres
at the separation: the same unstable sustainer as `[C6-7; B6-0]` ([ADR-168](0168-pods-powered-flies.md)).
The +4.02% in speed comes just as both rockets start to turn over: OpenRocket's speed peaked at
77.5 m/s and is falling at its last row, while hpr's still climbs. Which of the two turns over
sooner, and why, is not traced. `the_reference_values_are_the_records_and_the_outcomes_are_compares`
holds the record's side and the outcome. The reader's rules are pinned by
`a_stage_in_several_mounts_burns_out_first_where_its_motors_say` (its cases set the first
burnout by the curves and by the ignition order, with probes that pick the latest burnout, drop
the burn time, or drop a motor still to light, and fail) and
`a_tall_stack_of_stages_in_two_mounts_reads_in_one_pass` (40 stages). The flight's are pinned by
`a_powered_separation_drops_a_burning_booster_only_when_told` (with probes that drop the powered
or the lit condition, and fail), and the opt-in's by
`only_a_staging_marked_so_drops_a_burning_motor`.

`hpr sim` flies the file's first configuration with no flags. The booster drops at 0.860 s with
both A3s still burning, and a note says how much impulse the booster's flight leaves out. The
report's *Parts dropped still burning* gives it: 0.509 N·s. `cargo xtask ork-cli` now flies 136 of the survey's 170 configurations
as saved (135 before), and 54 of OpenRocket's 56 examples (53). The other two are hybrids, out of
scope until release 1.0.

**Left open.**

- `hpr sim`'s own defaults (a 1.5 m rail, no wind) give the unstable sustainer nothing to turn
  it over, so it prints an apogee for a flight that would tumble. Its least margin, −4.21
  calibres, is printed but no warning goes with it ([#335](https://github.com/nrdptel/hpr-sim/issues/335)).
- The dropped booster's leftover thrust is not flown: here 0.509 N·s.
- A charge among several mounts, and the "first" reading of an `ejection` separation, wait on a
  probe.
