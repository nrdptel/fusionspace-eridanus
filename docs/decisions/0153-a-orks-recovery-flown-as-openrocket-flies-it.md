# ADR-153: A `.ork`'s recovery flown as OpenRocket flies it, held to its descents (2026-10-04)

- **Status:** accepted
- **Summary:** M4.5a: a `.ork` configuration's parachutes and streamers become flight devices: the stated drag coefficient, or OpenRocket's own (0.8 on the canopy for a parachute, appendix C for a streamer), opening at once; apogee, altitude, ejection and launch triggers with the file's delay. Targets, set before any hpr descent was measured: each deployment within 0.1 s or 2% of OpenRocket's time, landing speed and flight time within 5%

**Context.** [ADR-144](0144-the-2026-10-03-planning-review-leaner-bookkeeping.md) §8 opened M4.5
*Fly my .ork*; its first increment, M4.5a (issue #240), flies a `.ork`'s recovery: parachutes and
streamers, ejection, apogee and altitude triggers, OpenRocket's "auto" drag coefficient from a
sourced value, held to OpenRocket's descents against a target set before measuring.
[ADR-056](0056-a-ork-designs-recovery-and-separation-read-as.md) reads the settings and leaves the
mapping to the step that flies them. OpenRocket's technical documentation v13.05 (CC BY-SA),
§4.2.5, says how its descent works: once a device deploys, the simulation turns to three degrees
of freedom, the position alone, and "the entire drag coefficient of the rocket is assumed to come
from the deployed recovery devices"; a parachute's default drag coefficient is 0.8 on the
parachute's area (citing Hoerner's *Fluid-Dynamic Drag*, 1965), and a streamer's comes from
appendix C. hpr's descent is the same model ([ADR-012](0012-recovery-drag-areas-triggers-inflation-and-the.md)):
a point mass under the summed drag areas of the open devices, the airframe's drag left out.

**Decision.**

1. **The mapping** (`hpr::ork::recovery`), one device per parachute or streamer of the flown
   configuration, in file order:
   - **Drag.** A parachute: `C_D · π D²/4`, `D` the canopy diameter and `C_D` the stated value,
     or 0.8 for `auto` (§4.2.5). A streamer: a stated `C_D` on its area `l · w`, or for `auto`
     `hpr_sim::recovery::StreamerModel::OpenRocket`, appendix C's equations C.4 and C.5 from the
     strip's length, width and material (ADR-013).
   - **Opening.** At once (`Inflation::Instant`), as OpenRocket's documentation describes no
     filling; the file's delay is the time from the event to the opening.
   - **Events.** `apogee`; `altitude`, the first time on the way down that the centre of mass is
     at or below the height above the ground; `ejection`, the ejection delay of the device's own
     stage's first lit motor; `launch`, the delay after launch. `never`, and `ejection` with a
     motor that has no ejection charge (a plugged motor, or none stated), add no device.
     `lowerstageseparation`, a word hpr does not know, and a device inside a part hpr does not
     read (a pod) are refused by name.
   - **A height above apogee.** OpenRocket's one probed run never opened such a parachute
     (ADR-056 §5); hpr's `Trigger::Altitude` opens it at apogee. hpr keeps its own rule, as an
     altimeter that finds itself below its main's height at apogee would fire, and a flight it
     changes names it, here and in `hpr sim`'s notes.
2. **The comparison.** `cargo xtask ork-flights` flies each public configuration it flies a
   second time with its devices, where OpenRocket's record has no separation and hpr flies no
   staging, and compares three things with OpenRocket's record: each deployment's time, in order,
   against its `RECOVERY_DEVICE_DEPLOYMENT` events; the landing speed against
   `ground_hit_velocity_m_s`; and the flight time against `flight_time_s`. A flight the mapping
   refuses is listed with why. The private library's report carries the same block.
3. **The targets,** set before any hpr descent was measured, over the flights in (2) with no
   named cause for the apogee ([ADR-069](0069-hprs-flights-of-the-public-designs-against.md),
   [ADR-073](0073-each-named-cause-sized-by-openrockets-own-flight.md)) but a parachute open
   before apogee, which this flight flies, each flight held to every one:
   - each deployment within 0.1 s of OpenRocket's or 2% of its time, whichever is larger
     (OpenRocket marks apogee a step late, so the apogee's own time differs by up to a step);
   - the landing speed within 5%;
   - the flight time within 5%. A flight with a named cause is reported apart, with its numbers. A
   miss stays in the report, and this ADR gains a measured explanation or the gap is left open, per
   [rule 2](../../CONTRIBUTING.md#2-never-weaken-a-check).
4. **`hpr sim`** flies the devices of a `.ork` or an hpr design read from one, through the same
   function, so its output stays the library's, bit for bit. A configuration whose recovery the
   mapping refuses flies as before, on its airframe alone, with the reason in its notes.
5. **Out of this increment:** a powered separation in `hpr sim` (issue #240's second item) and a
   device on a separated stage stay with M4.5g, where the separations are.

**Result (measured after the targets were committed).** `cargo xtask ork-flights` compares 32
public descents; the two two-stage configurations separate and are not compared.

- **In scope, 28 flights.** Landing speed +0.00% to +0.02% of OpenRocket's; flight time −1.57% to
  +3.16% (mean absolute 0.80%). 21 meet every target as written.
- **The 7 misses are deployment times alone,** on the dual-deploy example (6 flights) and the
  chute-release example's G80T-10. Each is sized from the committed numbers. hpr's apogee is 0.6%
  to 4.3% below OpenRocket's there, a climb gap the same report shows with no named cause. And
  OpenRocket opens a device set to a height at the end of its integration step: 0.06 to 12.64 m
  below that height over the 13 such deployments, 0.36 to 11.15 m on the 7 that miss. With the apogee's height and time and that step taken out, every deployment
  is within its target; the largest residual is 0.33 s, on a 98.8 s deployment. For a device
  opened at apogee the residual holds by construction, as it takes out the apogee's own time: the
  descent's own evidence is the landing speed and the drogue's speed below. The drogue's speed
  as the main opens agrees within 0.07% on 5 of the 6 dual-deploy flights; the sixth opens the
  main 5 s after apogee, before the drogue reaches its steady speed.
- **Apart, 4 flights with a named cause** for the apogee: 3 with a drag override hpr does not
  apply (#165), 1 put down to hpr's own drag. They land within 0.12%, but their flight times are
  −19.9% to +5.8%, as their apogees are.
- **The private library** (aggregate only): 32 descents compared, all in scope. Landing speed
  −1.53% to +0.04%, flight time −4.98% to +3.72%. 29 meet every target as written; all 32 do once
  sized, on numbers the private report does not publish. The −1.53% landing speed is not explained.

The deployment target is not met as written on 7 public flights. The descent model is not the
cause: the apogee is the climb's (M1.14's work), and the step is OpenRocket's. The report keeps
both columns. A test recomputes every residual and outcome from the record and hpr's numbers.

**Consequences.** The recovery page and the `.ork` format page say what a `.ork`'s recovery flies
as, and how close to OpenRocket. Knacke's filling and canopy data stay available to a design
written in hpr's own terms; a `.ork` gets OpenRocket's model, which its author chose.
