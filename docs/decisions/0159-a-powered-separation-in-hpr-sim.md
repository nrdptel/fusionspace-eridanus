# ADR-159: A `.ork`'s powered separation flown in `hpr sim`, each part under its own devices, a part with none tumbling (2026-10-04)

- **Status:** accepted
- **Summary:** M4.5g1: `hpr sim` flies a `.ork` configuration whose booster drops away under power. `hpr::ork::separated_recovery` puts each parachute and streamer on the part its stage is in, and adds the tumbles the flight needs: the booster tumbles from the split until its first own device opens and releases the tumble, and a sustainer with no device tumbles from its apogee. `FlightBuilder::separation` takes the separation, so the command flies the library's flight, bit for bit. Each part's landing is printed. The sustainer's descent is held to OpenRocket's with ADR-153's targets; the booster's is not validated

**Context.** [ADR-144](0144-the-2026-10-03-planning-review-leaner-bookkeeping.md) §8 asks that every
in-scope OpenRocket example fly as saved in `hpr sim`. Its *Two stage high power rocket* was
refused: `hpr sim` flew no separation, although the library has flown one since
[ADR-074](0074-ignition-times-and-powered-staging-the-sustainer.md), read from a `.ork` by
[ADR-076](0076-a-ork-files-ignitions-and-one-powered-separation.md). Issue #240's second item
asks for it, and [ADR-153](0153-a-orks-recovery-flown-as-openrocket-flies-it.md) §5 left a
separated stage's devices to this increment.

The library's rule shapes the answer. After a separation each part flies as a point with no
airframe drag of its own ([ADR-014](0014-separation-bodies-their-masses-and-their.md)),
so every part must carry a device, and a booster's must be open from the split
(`Simulation::with_separation`). OpenRocket's example opens the booster's parachute at the
booster's own apogee, about 2 s after the split.

**Decision.**

1. **Each device on its part.** A device in a stage aft of the split rides the booster, body 1;
   the rest ride the sustainer, body 0. Each event is the part's own: `apogee` is that part's
   apogee, as the flight fires it per body.
2. **Tumbles where the flight needs them** (`hpr::ork::tumbling`). The booster tumbles from the
   split, on `DeviceDrag::tumbling_stages` over its stages
   ([ADR-013](0013-streamer-and-tumble-drag.md)), until its first own device in file order
   opens, which releases the tumble. Under its parachute it then has the parachute's drag alone,
   as OpenRocket flies a descent (its technical documentation v13.05, §4.2.5). A sustainer with no
   device of its own tumbles from its apogee, as the `ork_two_stage` example and
   `cargo xtask ork-flights` already flew it. A file whose recovery the mapping refuses flies
   these tumbles alone, and the notes say why.
3. **The facade carries it.** `FlightBuilder::separation` hands the separation to the flight.
   `FlightBuilder::inputs`, the Monte Carlo run's inputs, refuses a builder with one, as those
   inputs hold no separation and would drop it silently.
4. **The command.** `hpr sim` flies the file's staging when no `--motor` is given (a motor of the
   user's own can't be told when the stages separate, so that stays refused). Each device names
   its part; each separated part's landing is printed, marked rough. A note says the booster
   tumbles side-on from the split, where a real one flies nose-first for a while, so its peak and
   landing are likely too low and too close to the pad (#179), and a map export draws no pin for
   it. The descent figure, the braking test and the landing's caveat look at the sustainer's
   devices only, and not at a tumble hpr added: a sustainer that only tumbles lands with "not a
   prediction", as a rocket falling on its airframe does. The JSON marks such a device `added`.
5. **The comparison.** `cargo xtask ork-flights` now flies the two two-stage configurations'
   descents the same way and holds the sustainer's to ADR-153's targets, set before that ADR
   measured anything: OpenRocket's record holds the sustainer's branch alone.

**Result.** `cargo xtask ork-flights`, both configurations of the two-stage example:

| motors | deployments, OpenRocket / hpr (s) | landing speed Δ | flight time Δ | targets |
|---|---|---:|---:|---|
| H148R-0, then H148R-0 | 10.98 / 10.94; 40.25 / 39.16 | +0.00% | +0.32% | met once sized |
| I357T-14, then I59WN-P | 16.26 / 16.21; 79.95 / 79.47 | +0.00% | +0.73% | met |

The first configuration's main opens 1.09 s early, against a target of 0.81 s. Its apogee is
1.79% below OpenRocket's (the climb gap [ADR-076](0076-a-ork-files-ignitions-and-one-powered-separation.md)
reports), and OpenRocket opens a device set to a height at the end of its step, 8.9 m below it
here. With both taken out, 0.065 s is left, within the target, as for the dual-deploy example's
misses in ADR-153. Over the 30 in-scope public descents, 22 meet every target as written and all
30 once sized. The private library (aggregate only) gains 2 descents that separate under power,
both meeting every target: 31 of its 34 meet them as written and all 34 once sized, its ranges
unchanged.

**Consequences.** OpenRocket's two-stage example flies as saved in `hpr sim`: its first
configuration offline once its H148R is fetched, the second once its I357T and I59WN are too. The booster's
flight after the split is not compared with anything: OpenRocket flies it as a rigid body on its
own aerodynamics, and its record keeps only the sustainer's branch. A booster with several
devices is released from its tumble by the first in file order, not the first to open. A
booster's device set to a height above the booster's own apogee opens at that apogee with no
note saying so, as the flight records no apogee for a separated part. Two
separations (#183) and an unpowered one before apogee (#184) stay refused by name, for M4.5g2
and M4.5g3.
