# ADR-075: A cluster is one tube repeated, and a motor in it one motor per tube (2026-09-25)

- **Status:** accepted
- **Summary:** A cluster is one tube repeated, and a motor in it one motor per tube

**Context.** M1.9b asks that several motors in one mount sum their thrust and mass, that a motor
out give the pitch moment a hand calculation predicts (Loft lesson L31: Loft's clusters were on
the axis only), and that `.ork` clusters be read. What was in hand: a mount held at most one motor,
so a cluster needed a mount per motor; the thrust already acted at each nozzle with its moment
(M1.3), so an off-axis motor already turned the rocket; and the `.ork` reader read a clustered
inner tube as the one tube it is written as, with a warning, leaving its configurations out
(ADR-064). No document gives OpenRocket's cluster patterns: `clusterconfiguration` names one
(`3-ring`, `4-ring` and so on), `clusterscale` spreads it and `clusterrotation` turns it.

**Decision.**

1. **A cluster is a list of places on the inner tube.** `InnerTube::cluster_m` holds each tube's
   axis, `[x, y]` in body axes from the axis the tube's radial offset and angle give; empty is one
   tube, as before, bit for bit. The part weighs every tube, each with its own parallel-axis term.
   Whatever the tube holds (an engine block, a mass) is repeated in every tube, so each placed
   component carries its copies (`PlacedComponent::copies_m`) and its mass counts them all. A mass
   override on a cluster sets the whole cluster's mass, as OpenRocket's does; an override on a part
   inside it is each copy's (OpenRocket 24.12 weighs an engine block overridden to 0.01 kg in a
   3-ring as 0.03 kg), so a part is weighed and overridden in one tube and then repeated. A
   general list rather
   than named patterns keeps the library free of OpenRocket's names; the `.ork` reader turns a
   pattern into the list.
2. **A motor in a cluster is one motor per tube.** A configuration still names one motor per
   mount. Placing it gives one `PlacedMotor` per tube, one after another in the order of the tubes,
   each with its nozzle on its tube's axis. Thrust, mass and the moments then sum over the motors
   as they always did, with no new term in the equations of motion. A cluster mount holds one
   kind of motor, as an OpenRocket clustered tube does; a mixed cluster is a mount per kind, each
   motor its own, so none is sent anywhere as copies of another (Loft's lesson L31). A motor
   index (a device's `Trigger::MotorDelay`, say) counts every placed motor. A burnout of a
   clustered mount is the burnout of its first motor that lights, since they light together.
3. **A motor out is a failed tube.** `MountedMotor::failed_tubes` names tubes whose motor never
   lights: carried loaded, giving no thrust. It serves what-if studies, as a motor that fails to
   light is the cluster's common failure. A sustainer's cut after a powered separation rebuilds
   one entry per mount, keeping the failed tubes.
4. **OpenRocket's patterns, measured.** `validation/oracles/openrocket/clusters.py` asks
   OpenRocket 24.12, run as an external oracle through its public API, for each of its fourteen
   patterns' points and, on 25 probes, for where it puts every tube and what it weighs. The points
   are the figures they look like, in units of the separation between neighbouring axes: rows one
   apart, a triangle and a square of side one, a pentagon of side one, rings of radius one round a
   centre tube (the stars), and a grid and an eight-ring of spacing 1.4. A tube's place is
   `2 R s · Rot(θ − ρ) · pₖ` from the cluster's axis, for the tube's outer radius `R`, the scale
   `s`, its roll angle `θ` and the rotation `ρ` (degrees in the file), with OpenRocket's `(y, z)`
   read as hpr's `(x, y)` as every roll angle is. hpr's reading puts every tube of every probe
   within 1e-15 m of OpenRocket's. A name OpenRocket has no pattern for (`4-square`), it reads as
   one tube; so does hpr, with a warning.
5. **OpenRocket weighs a cluster's tubes stacked on its axis.** On its probes the mass and centre
   agree with hpr's to 1e-12, but a 3-ring at scale 1 and at scale 1.5 have the same inertias:
   OpenRocket counts each tube's own inertia and no parallel-axis term for its spread, while an
   engine block inside each tube is weighed where it is, spread and all. hpr places each tube where
   it is, as the physics asks, and does not copy that. On every cluster on the body's axis, hpr's
   roll inertia is OpenRocket's plus `Σ m |c|²` over the tubes and its pitch inertia (the mean
   across the axis) plus half of it, to 1e-12; on the 3-ring probe that is +5.11% in roll and
   +0.273% in pitch. Off the axis the roll rule holds as well, but three departures are left and
   pinned: a lone tube 10 mm off the axis, whose offset OpenRocket leaves out (+0.303% roll), and
   the pitch of two clusters off the axis (+0.015%, +0.021%), which hpr has not traced.
6. **The survey's cause is sized.** `cargo xtask ork` held the four designs with a cluster outside
   1% in mass and centre, traced to the one-tube reading. They are now within, and their roll
   inertia is outside by +1.01% and +2.08%. The survey names the cause, OpenRocket's stacking, only
   when hpr's roll less the clusters' own spread is within 1% of OpenRocket's; it is +0.04% and
   +0.00%.
7. **Flights of `.ork` clusters wait for M1.9c.** A configuration with a motor in a clustered tube
   is still left out of hpr's flights of a `.ork` (`NotFlown::Cluster`), as one lit in flight is,
   until M1.9c holds such a flight to OpenRocket's. The design itself reads whole, and hpr flies
   it when asked.

**Consequences.**

- M1.9b's bar is met by tests. `hpr_design` holds a cluster's mass, its contents repeated and its
  motors placed, to hand calculations. `hpr_sim::staging` holds the thrust and mass of three
  motors in one mount, and a motor out's angular acceleration at rest to the hand calculation
  `I_c⁻¹ Σ (pᵢ − c) × T ẑ` within 3.7e-7 (L31's test,
  `cluster_motor_out_produces_pitch_moment`). `hpr_io` holds the fourteen patterns, and
  `hpr_validate` every probe's tubes and mass.
- The mass survey's counts move: 62 of 71 within 1% in mass (was 58), 63 in centre (was 59); roll
  with OpenRocket's fin rule 52 within 1% (was 56), the four clusters' spread now the named cause.
- Not modelled: a ring's automatic bore round a cluster is the tube's own radius, as OpenRocket
  gives it, so the tubes run through the ring, and the design checks warn of it. A part inside a
  tube off the body's axis is read at its own radial offset from the body's axis; the `.ork`
  reader adds no parent's offset to it, as before, where OpenRocket places it from the tube's axis
  (measured with an engine block in a tube 10 mm off the axis). The reader warns of it and no
  survey file has one; a cluster on the axis, the common case, is not affected
  ([#181](https://github.com/nrdptel/hpr-sim/issues/181)). Real clusters' thrust misalignment and ignition spread, short of a motor that fails
  outright, are not modelled.
