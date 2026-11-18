# ADR-357: Tip-off, measured with RocketPy's: past the plan's bar, the model in two more increments (2026-11-18)

- **Status:** accepted; splits [M1.14b5][m1-14b5]'s *done when* ([ADR-351][adr-351], decision 2) into M1.14b5, [M1.14b9][m1-14b9] and [M1.14b10][m1-14b10]: the *done when* changed, M1.14b5 keeping the measurement and M1.14b9 the model in ADR-351's words; M1.14b9 and M1.14b10 go ahead of [M1.14b6][m1-14b6] to [M1.14b8][m1-14b8]
- **Summary:** RocketPy's tip-off phase, at the merge of its pull request #920, flies thirteen example rockets in still air and in a 20 mph (8.94 m/s) crosswind. Turned on, it moves 9 of 26 landing points by more than 1% of the apogee and one apogee by more than 1%, so the plan calls for a tip-off model. Against a rocket guided to its last button, as HPR Sim guides it, the phase moves the landing point by up to 3.765% of the apogee and the apogee by up to 0.414%. The model is M1.14b9's, and flying it by default is M1.14b10's.

[m1-14b5]: ../decisions-and-roadmap.md#m1-14b5
[m1-14b6]: ../decisions-and-roadmap.md#m1-14b6
[m1-14b8]: ../decisions-and-roadmap.md#m1-14b8
[m1-14b9]: ../decisions-and-roadmap.md#m1-14b9
[m1-14b10]: ../decisions-and-roadmap.md#m1-14b10
[adr-351]: 0351-m1-14b-in-eight-the-basic-finners-normal-force-first.md
[adr-026]: 0026-the-path-in-wind-rocketpys-corrected-equations.md
[adr-144]: 0144-the-2026-10-03-planning-review-leaner-bookkeeping.md
[adr-347]: 0347-the-2026-11-14-review-landing-points-held-to-tracked-landings-predictions-first.md
[report]: ../../validation/reports/tip-off.md
[report-json]: ../../validation/reports/tip-off.json
[flight]: ../physics/flight.md#what-leaving-tip-off-out-costs
[pr-920]: https://github.com/RocketPy-Team/RocketPy/pull/920
[pr-1196]: https://github.com/RocketPy-Team/RocketPy/pull/1196

**Context.** HPR Sim guides a rocket along the rail until its last guide leaves the top, then
frees it with no turning rate. A real rocket is held by one guide alone once the guide ahead of
it is off the rail, and gravity and the wind turn it about that guide: tip-off. [M1.14b5][m1-14b5]'s
*done when* asks what leaving that out costs, measured with RocketPy's tip-off phase, and sets a
rule: if the phase moves any apogee by more than 1%, or any landing point by more than 1% of the
apogee, HPR Sim gains a tip-off model held to RocketPy's; otherwise the flight page states the
cost. Every measured figure below is in [the committed report][report] ([as data][report-json]),
which `cargo xtask tip-off --check` holds in the gate's `refs` step.

**Decision.**

1. **The oracle is RocketPy's development branch at one commit.** The phase is RocketPy's
   [pull request #920][pr-920], `Flight(..., use_udot_rail2=True)`, which no release carries.
   `validation/refs.lock.toml` pins its merge, `ebe1b775cd9270c93018dc6d7867313d6eed4e5c`, as
   `rocketpy-tip-off`, and `validation/oracles/rocketpy/tip_off.py` flies from that checkout. It
   refuses to run unless the checkout is at the pinned commit, its `rocketpy` folder is
   unchanged, and Python loaded RocketPy from it.
   - **What the phase is.** Released RocketPy frees the rocket where its upper button leaves the
     rail. With the phase on, the rocket slides on from there, free to pitch and yaw about its
     lower button with roll held, until the lower button leaves too.
   - **The equations.** Each flight uses the commit's equations with RocketPy's
     [pull request #1196][pr-1196], the sign correction every whole flight of this suite flies
     with ([ADR-026][adr-026]). A second run flies the commit as pinned. The report keeps the
     first whole and the second's largest moves.
   - **The inputs.** The thirteen rockets are the public designs built from RocketPy's
     examples (`validation/fixtures/design/rocketpy-rocket-mass.json`). Each flies from its
     example's rail and site, in the standard atmosphere, on a declared constant drag
     coefficient of 0.5, with its parachutes and no sensor noise. The wind is none, or 20 mph
     (8.94 m/s) square across the rail's heading at every height.

2. **Three flights a rocket and wind, and two differences.** The phase off, the phase on, and a
   third: the phase off on a rail longer by the distance between the buttons. That third flight
   is guided until the lower button leaves the real rail, which is HPR Sim's release.
   - **On less off** is the plan's rule. It mixes two things: the pivot, and RocketPy's earlier
     release, which lets the rocket weathercock while its lower button is still on the rail.
   - **On less guided** is what HPR Sim's release leaves out: the pivot alone.
   - **A landing point's move** is the distance between the two landing points, as a share of
     the other flight's apogee. The report also gives how much farther from the pad the rocket
     lands with the phase on.
   - **One declared change.** NDRT 2020's rail is vertical. In still air the rocket falls
     tail-first through its apogee parachute's 1 s lag, and RocketPy's integration of that fall
     does not finish: the script's watchdog stops a flight that runs past ten minutes. Those
     three flights open that parachute at apogee with no lag. The
     report says so beside them. The change acts after apogee, on a flight that goes straight
     up and down: the three land within 6 mm (0.2 in) of each other.

3. **The measurement: the plan's rule calls for a model.**

   | What the phase moves | On less off | On less guided |
   |---|--:|--:|
   | Apogees moved by more than 1% | 1 of 26 | 0 of 26 |
   | Landing points moved by more than 1% of the apogee | 9 of 26 | 6 of 26 |
   | Rockets with a move past a bar | 6 of 13 | 3 of 13 |
   | Largest apogee move | +1.165% | −0.414% |
   | Largest landing-point move, of the apogee | 4.585% | 3.765% |
   | Landings farther from the pad with the phase on | 22 of 26 | 23 of 26 |

   - **On less off.** The one apogee past 1% is Bella Lui's in the crosswind, +5.6 m (18 ft).
     There the released RocketPy frees a slow rocket early and it turns into the wind at
     15.78 deg/s by the time its lower button leaves; with the phase on it turns at 0.60 deg/s.
     The largest landing-point move is Juno III's in the crosswind, 113.7 m (373 ft).
   - **On less guided.** The six landing points past the bar are Valetudo's, Juno III's and
     Prometheus 2022's, in both winds: 3.621% and 3.765%, 3.481% and 3.692%, and 1.417% and 1.565%
     of the apogee. The largest in distance is Juno III's in the crosswind, 92.0 m (302 ft).
     The largest apogee move is Juno III's in still air, −10.9 m (−36 ft).
   - **The pivot itself is small.** With the phase on, a rocket leaves its lower button
     turning at up to 2.38 deg/s, its axis at most 0.083° off the rail's. The three rockets
     past the bar are the three that leave turning fastest in still air, at 1.49 to
     2.26 deg/s.
   - **Which way leaving it out errs.** Guided, the rocket lands nearer the pad than with the
     phase on in 23 of the 26 flights: in every flight in still air (the vertical rail's by
     6 mm) and in ten of the thirteen in the crosswind. The three that land farther from the
     pad guided do so by 0.3, 2.9 and 3.6 m (1, 10 and 12 ft). A landing point nearer the pad
     than the real one is the flattering side ([ADR-347][adr-347]).
   - **As pinned,** without pull request #1196, the reading is the same in kind: on less off,
     one apogee and 7 landing points past the bars; on less guided, no apogee and 6 landing
     points, the largest 3.270% of the apogee.

4. **The split.** The measurement is past the bar, so the *done when* asks for a model, and a
   model held to RocketPy's on thirteen designs is more than this increment. M1.14b5 keeps the
   measurement; the model takes ADR-351's words whole.
   - **M1.14b5 Tip-off, measured with RocketPy's.** *Done when:* RocketPy at a pinned commit
     that carries its tip-off phase (its pull request #920) flies each public design built
     from RocketPy's examples twice, the phase on and off, in still air and in a 20 mph
     (8.9 m/s) crosswind; a committed report gives what the phase moves on each: the apogee,
     the landing point, and the attitude and pitch rate where the last guide leaves the rail,
     and which moves are past 1% of the apogee or, for a landing point, 1% of the apogee; the
     flight page states the measured cost where it says tip-off is left out, and that the
     measurement calls for a model; and a check holds the report.
   - **M1.14b9 The tip-off pivot, held to RocketPy's.** *Done when:* HPR Sim gains a tip-off
     model, with its equation and source, held by a test to RocketPy's on those designs, where:
     - the model is a phase of the flight: from the forward guide leaving the rail until the
       last guide leaves, the rocket slides along the rail free to pitch and yaw about its
       last guide, with roll held, under gravity, thrust and its aerodynamic loads; the flight
       page gives the equation, its source and a worked example; and a test holds the turning
       acceleration at the instant the forward guide leaves, on a body of fixed mass with no
       thrust and no air, to $m g r \cos\theta / (I + m r^2)$ within 1e-9 of its size, with
       $r$ from the guide to the center of mass, $I$ the pitch inertia about the center of
       mass and $\theta$ the rail's angle above the horizontal;
     - HPR Sim flies each of the report's thirteen designs from the report's rail and site, in
       still air and in the report's crosswind, guided as today and with the model; a
       committed report, held by a check, sets beside RocketPy's for each of the 26: the pitch
       rate and the angle off the rail's axis where the last guide leaves, and what the model
       moves from the guided flight in the apogee and, as a vector, in the landing point;
     - on each of the 26, the pitch rate is within 10% of RocketPy's or 0.1 deg/s, whichever
       is larger, and the angle within 10% or 0.005°; the apogee's move is within 0.2% of the
       guided apogee of RocketPy's move; and the landing point's move is within 1% of the
       guided apogee of RocketPy's, as the distance between the two moves;
     - a figure past its bar is not met: an ADR gives its cause with its sign and size, the
       report keeps it visible, and no bar is widened;
     - and the model is off unless a flight asks for it: every other committed report
       reproduces unchanged.
   - **M1.14b10 Tip-off in every flight that has two guides.** *Done when:* a flight of a
     design with two or more guides flies M1.14b9's phase unless told not to; each committed
     report it moves is regenerated in the same pull request; a committed table gives, for
     each real flight HPR Sim compares, what the phase moved in the apogee and, where a
     landing point is compared, in the landing point, with the real flights' mean absolute
     apogee error before and after; a comparison with a code that has no such phase
     (OpenRocket, RocketPy as released) flies with it off and says so; a test pins which way a
     landing point errs without the phase, with a mutation probe; and if the real flights'
     mean absolute apogee error rises with the phase on, an ADR measures the rise and its
     cause before the default changes, or keeps the phase opt-in with that measurement on the
     flight page.

5. **Why the bars of M1.14b9 are these.** They are set by what matters at the plan's 1%, not
   by what the model is expected to reach.
   - On the three rockets past the bar the landing point's move grows with the pitch rate, so
     a rate 10% off is worth about a tenth of the move: at most 0.38% of the apogee on the
     largest, 3.765%. The 0.1 deg/s floor is under the smallest rate of an inclined rail in
     still air, 0.11 deg/s, whose flight moves its landing point by 0.086% of the apogee.
   - The landing point's bar is the plan's own 1% of the apogee, taken on the difference
     between the two codes' moves. A model that moved nothing would miss it on the six flights
     of decision 3, and miss the apogee's 0.2% on four of them (−0.414%, −0.345%, −0.345% and
     −0.249%).
   - The two codes fly different aerodynamics after the rail, which is why moves are
     compared, each code against its own guided flight, and not the flights themselves.

6. **M1.14b9 and M1.14b10 come next, ahead of M1.14b6 to M1.14b8.** ADR-351 put the three
   after tip-off without knowing its size. The measurement puts HPR Sim's landing points on
   the flattering side, and a flattering-side error is the worst class for a safety number
   ([ADR-144][adr-144] §6, [ADR-347][adr-347]). This
   record does not call the omission a known wrong number in shipped physics: its one
   reference is another code's unreleased phase, checked against no flight, and the flight
   page has said tip-off is left out since the rail was built. It is a measured omission with
   its model next in line.

**What this leaves open.**

- **No flight checks the phase.** The comparison is code to code. RocketPy's pivot is a
  frictionless pin with roll held; a real button has play in its slot, and a lug slides on a
  rod. M1.14b10's table is the first look at real flights.
- **One drag, one wind.** The thirteen fly a constant drag coefficient and a wind that does
  not change with height. The moves on a rocket's own drag, or in a sheared wind, are not
  measured.
- **The descent RocketPy does not finish** (decision 2) is RocketPy's own, in a tail-first
  fall; its cause was not traced. It is recorded here and in the oracle script, and nothing
  in HPR Sim rests on it.
