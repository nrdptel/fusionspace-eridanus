# ADR-193: Neer's 2026-10-07 control scope: drag-only airbrakes, roll-only canards and fin tabs (2026-10-07)

- **Status:** accepted; amends [ADR-192, the 2026-10-06 follow-up][adr-192] §4 and [ADR-162, the suite and its releases][adr-162] §4
- **Summary:** hpr's active control, simulated or built, is limited to airbrakes that only add drag and to canards and tail-fin tabs that only roll the rocket, stable with every surface inert and with any one surface stuck. The controller has two commands, brake deployment and roll, through a fixed mixer, so nothing can command pitch or yaw. Active pitch and yaw control joins the never tier, and the backlog's thrust vector control and active fins leave it. Roll tabs come before roll canards, because a tail spanning as much as the canards removed most of their roll in NASA's supersonic tests. M6.5 becomes roll control (queued before the flight computer logic); new M13.6 holds the control hardware for a ruling. Hardware stays in the held tier.

**Context.** On October 7, 2026, Neer called for more research and proposed the line the
competition rules (IREC) draw: drag or roll stabilization is fine, guidance toward a target never
is, and the rocket must be stable with the controls switched off. Airbrakes would only add drag,
canards would only control roll, neither with active pitch or yaw control, and servo-driven roll
tabs on the fins were to be considered too ([VISION](../VISION.md), that date).

The research is in [the roll and drag control note](../research/roll-and-drag-control.md). It found:

- **The rules:** no regulation or ruling separates drag-only or roll-only control from Note 3's
  "active controls". The 2024 proposal's docket asks for exactly that line: an IREC student form
  letter, ESRA and a NAR trustee all wrote in. State hasn't answered.
- **Practice:** IREC, EuRoC, Launch Canada and Wisconsin Space Grant all allow, or set, roll or
  drag control. NASA set US students a roll-control task in 2016-17.
- **Enforcement:** none found against hobby roll or drag control.
- **The aerodynamics:** a tail-fin tab behaves like fin cant scaled by Glauert's effectiveness.
  At Mach 1.6 to 3.5, a tail spanning as much as the canards removed about 70% to 130% of their
  roll (NASA TP-2157); no subsonic data was found. Hinged tabs can buzz from about Mach 0.70 to
  1.24 (NACA RM L53I29).

**Decision.**

1. **The control scope.** hpr's active control is limited to three devices: airbrakes that only add
   drag, and canards and hinged trailing-edge tabs on the tail fins that only roll the rocket.
   This holds for the simulator, the reference controllers, the shared embedded crate and any
   hardware.
   - The rocket must be stable with every surface inert, and with any one surface stuck anywhere
     in its travel.
   - This is stricter than IREC §7.1, which also allows pitch stability augmentation.
2. **Built so it can't steer.**
   - The controller interface has two commands only: brake deployment (0 to 1) and roll.
   - A fixed mixer turns the roll command into equal and opposite deflections on opposite
     surfaces. There is no pitch or yaw channel.
   - The simulator models a stuck surface or a single deployed flap as a *fault*, never as a
     command.
   - Hardware (M13.6) drives each function from one actuator through a common linkage where the
     design allows it. Otherwise the stuck-surface check covers each actuator.
3. **The flight rules become checks:**
   - neutral through boost (IREC §7.4);
   - neutral on an abort, a power loss, or a tilt past 30° (§7.3);
   - stable when inert (§7.2.1).
4. **Tabs before canards for roll.** M6.5 models both. Tabs come first: they sit on the tail, so
   no tail can cancel them, and they don't move the center of pressure forward. Canards carry a
   warning wherever the tail's exposed span exceeds 0.75 of theirs, the bound TP-2157 found for
   useful roll control.
5. **The tiers** (amends [ADR-192][adr-192] §4):
   - *Never* gains active pitch and yaw control. Thrust vector control was already there.
   - The ideas backlog's "thrust vector control and active fins with a controller in the loop"
     (Neer, 2026-10-03, tier LATER) is withdrawn, since his request rules out pitch and yaw
     control.
   - The *held* tier now holds only drag-only and roll-only firmware and hardware. It stays held:
     the narrowing isn't a ruling.
   - Tab hardware comes first (M13.6). Roll-canard hardware follows only if M6.5's canard check
     shows the canards keep their roll on the design.
   - When the hardware nears, the commodity jurisdiction request describes this one item: drag-only
     and roll-only by construction, and stable when inert.
6. **The queue** (amends [ADR-162][adr-162] §4). M6.5 is queued just before M13.3, the flight
   computer logic. A controller is simulated and checked before any firmware would run it. Release
   1.0 doesn't need it, and the lines ADR-162 orders after 1.0 keep their places.

**Alternatives considered.**

- *Follow IREC exactly, pitch augmentation included.* Rejected: the project rules out active pitch or
  yaw. A pitch channel also makes a steering capability one software change away.
- *Canards first, as M6.5 had it.* Rejected. Where the tail spans as much as the canards, NASA's
  supersonic data show it removes most of their roll, and TM X-3070 found canards "not suitable
  devices for roll control". No subsonic data says otherwise. Tabs sit on the tail, so no tail can
  cancel them, and they need no forward surface that costs static margin.
- *Treat the narrowing as enough to publish firmware.* Rejected. The docket shows that hobbyists,
  ESRA and a NAR trustee read the rule this way, but State hasn't, and the rule text gives no
  exception.
- *Keep TVC and active fins in the backlog for the simulator only.* Rejected: the simulator would
  then model a capability hpr refuses to build, through a controller interface that would need a
  pitch channel.

**Consequences.**

- [M6.4, airbrakes][m6-4] gains the one-command rule, the single-flap fault, and the neutral-gate
  checks.
- [M6.5][m6-5] becomes "Roll control: tail-fin tabs and canards", queued at 34.
- A new M13.6 holds the control hardware, gated on a ruling.
- [VISION](../VISION.md) gains the request and V45. Its *Not now* line on TVC and active fins
  points here.
- [The export control note](../research/export-control.md) and
  [the hardware line](../research/suite-hardware-line.md) point to the new note.

[adr-162]: 0162-the-suite-and-its-releases.md
[adr-192]: 0192-the-2026-10-06-follow-up-best-over-first-and-export-control.md
[m6-4]: ../decisions-and-roadmap.md#m6-4
[m6-5]: ../decisions-and-roadmap.md#m6-5
