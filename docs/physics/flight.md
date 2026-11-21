# Rigid-body flight

See the equations that fly a rocket from rail to ground as a rigid body, free to turn, that lightens
as its motor burns.

## In short

- **What it models:** a rocket's flight from the pad to the ground, rail included. The rocket is
  a rigid body free to move and turn in every direction (six degrees of freedom). It gets lighter
  as its motor burns.
- **Sources:** the equations of motion in RocketPy's technical documentation (RocketPy 1.13.0).
  The RocketPy paper (Ceotto et al., 2021) is cited but was not fetched.
- **How well it is validated:** by analytic and unit tests, against RocketPy, and on eight of
  NASA's check cases. In a vacuum, a tumbling rocket's center of mass stays on the exact parabola
  to 1.7e-6 m over 22 s. On six of the check cases, a free body on the same equations differs
  from the codes NASA published by more than they differ from each other. It ends 19.51 mm
  (0.77 in) lower after a 4.4 km (2.7 mi) fall, turns up to 0.01633 deg/s apart, and puts a
  dragged sphere up to 105.0 mm (4.1 in) away. A named difference between the models brings each
  inside the codes' spread at each code's largest difference. For one code in the sphere's cases
  it does so without explaining the gap. Read at every sample, case 2's tumbling brick's rates
  are left just past the spread. Two shots from sea level come out inside it as flown. There the
  codes sit about 2 m (6.6 ft) apart ([a free body against NASA's check cases](#a-free-body-against-nasas-check-cases)). Five of
  RocketPy's example rockets were flown from the pad to the ground by both codes, with the same
  declared drag. They agree on height, speed, time and acceleration within 3%. The largest scored
  difference is +1.783%, a peak acceleration on the rail ([validation report][report],
  [whole flights against RocketPy](#whole-flights-against-rocketpy)). The path agrees as well,
  except the apogee drift of NDRT 2020 and of rockets that leave the rail slowly in a wind. In
  those, HPR Sim's apogee drift is −4.333% to −38.158% from RocketPy's
  ([validation report][report]). The causes are HPR Sim's
  [body lift](../glossary.md#body-lift), which RocketPy's normal force leaves out, and its later
  release from the rail ([ADR-026][adr-026]). For Juno III, another cause is HPR Sim's simpler
  fin model. A sixth, Prometheus 2022, passes Mach 1 on its drag table and
  agrees as well. Its drifts again differ from RocketPy's through body lift. With HPR Sim's own
  drag, against RocketPy flying the drag its examples ship, HPR Sim's heights differ from
  RocketPy's by −7.280% to +10.302% ([validation report][report]). The gaps are larger where the two drags differ most.
  HPR Sim's is well below the example's for two rockets. For Prometheus 2022 it is above the
  example's at high speed. That rocket has flown through Mach 1 on HPR Sim's drag since
  [the drag through Mach 1](../decisions-and-roadmap.md#m1-8b1) was built
  ([Accuracy](../accuracy.md#whole-flights-with-each-codes-own-drag)). Twelve public flights have been
  compared with their altitude logs at the apogee, seven of them along the climb too. [60 more][fixture-report] have
  been compared with their logs, on apogee alone
  ([Accuracy: real flights](../accuracy.md#real-flights)).
- **What it leaves out:** staging and delayed ignition (both on [Staging](staging.md)),
  turbulence and thrust misalignment. Tip-off (the pivot as the rocket leaves the rail) is left
  out unless a flight asks for it ([Tip-off](#tip-off)), and that phase is held to RocketPy's
  on RocketPy's normal force, and on HPR Sim's own in still air only. RocketPy's tip-off
  phase puts the pivot's cost at up to 3.765% of the apogee in the landing point and 0.414% in
  the apogee, one code against another with no flight to check it
  ([below](#what-leaving-tip-off-out-costs)). Its
  small-angle aerodynamics are used at every angle of attack (the angle between the rocket's axis
  and its path through the air), with no stall. A flight that reaches Mach 5, the top of the
  aerodynamic models, stops with an error. The rotational equations leave out the Earth's rate;
  on NASA's tumbling brick that moves a body rate by up to 0.01633 deg/s within 30 s
  ([#1521][i-1521]).

## About the model

### What the equations do

At each instant of a flight, HPR Sim adds up every force on the rocket and every turning effect (a
moment). They come from the thrust, the weight, the air and the propellant burning away. From
those it works out how fast the rocket speeds up and how fast its turning changes. The
integrator ([Time integration](integration.md)) then carries the rocket forward, one short step
at a time.

Burning propellant makes the rocket lighter, moves its center of mass, and lets the exhaust carry
away some of the rocket's turning (jet damping). The equations keep all three. They are
RocketPy's, from its technical documentation ([RP-EOM], under *Code and sources*), and are written
out under *Equations of motion*.

### Code and sources

Code: `hpr_sim::{dynamics, flight, free_body, histories, rail, recorder, state}`. Decisions:
[ADR-011][adr-011] (equations of motion, aerodynamic coupling, rail, phases and termination) and
[ADR-353][adr-353] and [ADR-356][adr-356] (the differences from NASA's check cases, traced), and
[ADR-357][adr-357] and [ADR-358][adr-358] (tip-off: its cost, the phase, and what is held). The integrator and
events are in [Time integration](integration.md), and the frames in [Frames](frames.md).

Sources:

- **[RP-EOM]** RocketPy technical documentation, "Equations of Motion" v0 (Kane's method with the
  Reynolds transport theorem) and v1 (the form solved). They are `docs/technical/equations_of_motion.rst`
  and `equations_of_motion_v1.rst` in RocketPy 1.13.0 (`refs/rocketpy`, MIT).
- **[C21]** G. H. Ceotto, R. N. Schmitt, G. F. Alves, L. A. Pezente and B. S. Carmo, "RocketPy:
  Six Degree-of-Freedom Rocket Trajectory Simulator", *J. Aerosp. Eng.* 34(6), 2021,
  doi:10.1061/(ASCE)AS.1943-5525.0001331. Cited, not fetched.
- **[N15]** D. G. Murri, E. B. Jackson and R. O. Shelton, *Check-Cases for Verification of
  6-Degree-of-Freedom Flight Vehicle Simulations*, NASA/TM-2015-218675, both volumes, 2015,
  with the errata of September 8, 2025, and the codes' time histories posted with it
  (<https://nescacademy.nasa.gov/flightsim/2015>). Cases 1 to 3 and 6 to 10 are flown
  ([below](#a-free-body-against-nasas-check-cases)).

- **[RP-TIP]** RocketPy's tip-off phase, `Flight.udot_rail2`, in its development branch (MIT;
  no release carries it). [Its pull request #920](https://github.com/RocketPy-Team/RocketPy/pull/920)
  added the phase and [#1217](https://github.com/RocketPy-Team/RocketPy/pull/1217) rewrote it.
  By its description the rewrite simplifies the phase's function, changes what RocketPy
  reports inside the phase, and corrects how the phase ends: it finds the moment each button
  leaves by a root search on the solver's own interpolation of a step, and it corrects what
  happened when a parachute opened inside the phase or the rail was too short for it. It
  also took away the phase's switch. HPR Sim's [tip-off phase](#tip-off) takes its form from
  the first: the free equations plus the force and the moment the last guide adds. The
  comparisons below fly the rewrite.

## The equations

### State

- The state holds the nose tip $O$'s position $\mathbf{r}_O$ and velocity $\mathbf{v}_O$ in the
  [launch frame](../glossary.md#launch-frame-enu) $L$. It also holds the attitude quaternion $q$,
  four numbers that give the rocket's orientation, turning body axes into $L$'s. Last come the
  body rates $\boldsymbol{\omega}$ (how fast it turns about each [body axis](../glossary.md#body-frame)) relative to
  $L$, in body axes: 13 components in all.
- The state follows the nose tip because it is fixed in the body and is the body origin
  ([the design-tree decision][adr-007]).
  The center of mass $\mathbf{r}$ (from $O$, body axes) moves as propellant burns.
- The norm of the quaternion (its length, 1 for a pure rotation) drifts slightly between steps.
  Every use normalizes it, and it is never reset at an event ([Time integration](integration.md)).

### Equations of motion

[RP-EOM] v0 derives the motion of a variable-mass system (one whose mass changes as it flies)
about a point fixed in the body. It uses two standard tools:

- Kane's equations, a systematic way to write the equations of motion, for the rigid parts;
- the Reynolds transport theorem, which accounts for mass flowing across a boundary, for the
  propellant and the gas leaving the nozzle.

Its assumptions:

- the gas flow inside the motor is quasi-steady, changing slowly enough to treat as steady at each
  instant. So the integrals over the space it flows through, the control volume, don't change;
- the flow is axisymmetric: the same all round the rocket's axis;
- the exhaust momentum is lumped into the thrust at the nozzle exit.

v1 solves the result for $\dot{\mathbf{v}}$ and $\dot{\boldsymbol{\omega}}$, the rates of change of the velocity and of the body rates.
The lines below are that form, in body axes, with primes for time derivatives seen from the
rocket. $\mathbf{T}_{03}$, $\mathbf{T}_{04}$, $\mathbf{T}_{20}$ and $\mathbf{T}_{21}$ are labels carried over from [RP-EOM] v1, so each line can
be matched against it:

- $\mathbf{T}_{03}$ comes from the momentum of mass moving inside the rocket: gas flowing to the nozzles, and
  the center of mass shifting as propellant burns. The rotation turns that momentum into a force,
  $\boldsymbol{\omega} \times \mathbf{T}_{03}$, as it does for anything moving inside a turning body (a Coriolis force).
- $\mathbf{T}_{04}$ is the thrust $\mathbf{T}$ with the propellant's internal-momentum terms (*Measured thrust curves
  and the internal-momentum terms*, below).
- $\mathbf{T}_{20}$ gathers every force: $\mathbf{T}_{04}$, the weight $\mathbf{W}$, the air force $\mathbf{A}$, and two terms from the
  rotation. They are $\boldsymbol{\omega} \times \mathbf{T}_{03}$ and $-\boldsymbol{\omega} \times (\boldsymbol{\omega} \times m\mathbf{r})$, the second because the center of mass is not at $O$.
- $\mathbf{T}_{21}$ gathers every moment about $O$. They come from the rotation itself, from the jet and the changing
  inertia ($\sum_k \dot{m}_k S_k - I_O'$), and from the weight, the air and the thrust. The last term of $\mathbf{T}_{21}$,
  $\boldsymbol{\omega} \times \mathbf{h} + \mathbf{h}'$, is for parts that move along the airframe. Here $\mathbf{h}$ is their angular momentum about $O$,
  relative to the airframe. The term is zero for a part on the axis, and zero when nothing moves
  ([Moving mass](moving-mass.md)).
- The last lines give the angular acceleration $\dot{\boldsymbol{\omega}}$ and the nose tip's acceleration $\mathbf{a}_O$ (and from it
  $\mathbf{a}_L$ in $L$). Then comes the rate at which the attitude $q$ changes.

$$
\begin{aligned}
\mathbf{T}_{03} &= 2 \sum_k \dot{m}_k (\mathbf{n}_k - \mathbf{r}) - 2m\,\mathbf{r}', \\
\mathbf{T}_{04} &= \mathbf{T} - m\,\mathbf{r}'' - 2\dot{m}\,\mathbf{r}' \\
&\quad + \sum_k \ddot{m}_k (\mathbf{n}_k - \mathbf{r}), \\
\mathbf{T}_{20} &= -\boldsymbol{\omega} \times (\boldsymbol{\omega} \times m\mathbf{r}) + \boldsymbol{\omega} \times \mathbf{T}_{03} \\
&\quad + \mathbf{T}_{04} + \mathbf{W} + \mathbf{A}, \\
\mathbf{T}_{21} &= -\boldsymbol{\omega} \times (I_O \boldsymbol{\omega}) \\
&\quad + \Bigl(\sum_k \dot{m}_k S_k - I_O'\Bigr) \boldsymbol{\omega} \\
&\quad + \mathbf{r} \times \mathbf{W} + \mathbf{M}_A + \mathbf{M}_T \\
&\quad - \boldsymbol{\omega} \times \mathbf{h} - \mathbf{h}', \\
\dot{\boldsymbol{\omega}} &= I_c^{-1} (\mathbf{T}_{21} - \mathbf{r} \times \mathbf{T}_{20}), \\
\mathbf{a}_O &= \mathbf{T}_{20}/m - \dot{\boldsymbol{\omega}} \times \mathbf{r}, \\
\mathbf{a}_L &= q\, \mathbf{a}_O\, q^*, \\
\dot{q} &= \tfrac12\, q \otimes (0, \boldsymbol{\omega})
\end{aligned}
$$

- Mass terms:
  - $m$, $\dot{m} = \sum_k \dot{m}_k \le 0$ and $\ddot{m}_k$ are the mass, its rate and each motor's mass acceleration.
  - $I_O$ and $I_c$ are the inertia about $O$ and about the center of mass ($I_O = I_c + m(\lvert \mathbf{r} \rvert^2 \mathbf{1} - \mathbf{r}\mathbf{r}^{\mathrm{T}})$).
  - $\mathbf{n}_k$ is motor $k$'s nozzle exit.
- Forces and moments:
  - $\mathbf{T}$ is the thrust along $\mathbf{z}_B$, the motors' `thrust_at_pressure_n`, with moment $\mathbf{M}_T = \sum_k \mathbf{n}_k \times \mathbf{T}_k$ about $O$.
  - $\mathbf{W}$ is the weight: normal gravity (centrifugal term included) plus the Coriolis force
    $-2m\, \boldsymbol{\Omega} \times \mathbf{v}_{\mathrm{cg}}$, both acting at the center of mass.
  - $\mathbf{A}$ and $\mathbf{M}_A$ are the aerodynamic force and its moment about $O$ (below).
- The nozzle gyration tensor $S_k$ describes how the exhaust leaving motor $k$'s exit disc is
  spread about $O$. Times the mass flow $\dot{m}_k$, it gives the jet damping in $\mathbf{T}_{21}$:

  $$
  \begin{aligned}
  S_k &= \frac{r_e^2}{4} \operatorname{diag}(1, 1, 2) \\
  &\quad + \lvert \mathbf{n}_k \rvert^2\, \mathbf{1} - \mathbf{n}_k \mathbf{n}_k^{\mathrm{T}}
  \end{aligned}
  $$

  It is integrated here from v0's boxed
  rotational equation. The jet term there is $\int \mathbf{r} \times (\boldsymbol{\omega} \times \mathbf{r})\, \mathrm{d}\dot{m}$ over the exit disc of
  radius $r_e$, with a uniform jet. For a disc at $n_k$ on the axis, this gives

  $$
  \left(\frac{r_e^2}{4} + n_k^2,\ \frac{r_e^2}{4} + n_k^2,\ \frac{r_e^2}{2}\right)
  $$

  - RocketPy 1.13.0's code (`rocket.py:984-985`, `evaluate_nozzle_gyration_tensor`) uses
    $0.25\, n^2$ for the transverse distance term instead of $n^2$. Its $n$ is measured from the center
    of dry mass, not from the nose tip, so the entries can't be compared directly. The
    RocketPy comparison below corrects that term to $n^2$.
  - A motor that gives no nozzle exit radius contributes $r_e = 0$.
- For an axisymmetric rocket turning slowly about a transverse axis, the
  equations reduce exactly to the classical jet damping about the center of mass,

  $$
  I_c \dot{\omega} = \left[\dot{m} \left(\frac{r_e^2}{4} + l^2\right) - \dot{I}_c\right] \omega
  $$

  with $l$ from the center of mass to the nozzle exit. The
  $\dot{m}\, l^2$ part damps, and the falling inertia partly offsets it
  (`dynamics::tests::jet_damping_matches_the_classical_form`).
- Time derivatives of the mass properties:
  - $\mathbf{r}'$, $\mathbf{r}''$, $\dot{m}$ and $I_O'$ come from central differences of `Assembly::mass_properties` with a
    half-width of 1e-4 s.
  - The stencil, the times each difference samples, is kept inside the current integration
    interval. Every thrust-curve knot and burnout is a stop time, so no difference straddles a
    change in the thrust's slope. Near an interval's end the derivative is taken at the nearest
    valid center and extended linearly.
  - $\ddot{m}_k$ differences each motor's mass flow the same way.
  - After burnout (an interval starting past every burnout) the rates are zero. They are also zero
    over intervals shorter than 2e-5 s, where differences would be rounding noise.
  - A part that moves along the airframe adds its own rates in closed form, not by differences
    ([Moving mass](moving-mass.md)).
- A motor that burns through an interval is evaluated at its one-sided limit inside the burn, $t$
  clamped to $(0, t_{\mathrm{end}})$. The last stage of the step ending at burnout (or the first after
  ignition) then sees the burning motor. It sees the pressure correction too, which switches off at
  $t_{\mathrm{end}}$. Without this, fixed-step RK4 converged at
  first order (5.1 mm (0.20 in) of apogee at 2 ms).
- **Measured thrust curves and the internal-momentum terms.** A static test measures
  $\mathbf{T}_{\mathrm{exit}} - \mathrm{d}\mathbf{P}_{\mathrm{int}}/\mathrm{d}t$, where $\mathbf{P}_{\mathrm{int}} = m\mathbf{r}' - \dot{m}(\mathbf{n} - \mathbf{r})$ is the internal momentum of the burning
  propellant, so a `.eng` curve already contains it. $\mathbf{T}_{04}$ subtracts $-m\mathbf{r}'' - 2\dot{m}\mathbf{r}' + \ddot{m}(\mathbf{n} - \mathbf{r})$
  again.
  - HPR Sim keeps the terms as RocketPy does, for parity in
    [the RocketPy comparison](../decisions-and-roadmap.md#m2-1).
    The form is exact for the [RP-EOM] model, not for a measured curve.
  - The size of the double count on Valetudo: it lifts off at 1.56 ms with 73 N of thrust against
    95 N of weight. At liftoff 21 N comes from $\ddot{m}(\mathbf{n} - \mathbf{r})$. The double count changes the burnout
    speed by at most 0.05 m/s (0.16 ft/s).
- Earth's rotation enters only through the Coriolis force. The rotational equations use
  $\boldsymbol{\omega}$ relative to $L$, which differs from the inertial rate by at most 7.3e-5 rad/s
  ([Frames](frames.md) and [the flight decision][adr-011]). On a tumbling body of three unequal
  moments the effect of leaving it out grows past that:
  [NASA's check cases 2 and 3](#a-free-body-against-nasas-check-cases) measure it.

### Aerodynamics in flight

`hpr-aero` gives coefficients at a flow condition ([Aerodynamics](aero.md)). The engine applies them as follows.

- The air velocity uses the wind at the center of mass's height. Wind vectors are
  taken in $L$'s axes.
- The axial force is $-q A C_A \mathbf{z}_B$, from the whole rocket's drag. It is taken at the center of mass's
  airspeed, Mach number, angle of attack and Reynolds number per meter ($V/\nu$). The drag is power-on
  (`DragConditions::thrusting`, with the burning motors' cross-section) while any motor burns in
  the interval. `Simulation::with_full_base_drag_under_power` keeps the whole base's drag instead,
  as OpenRocket does, for comparisons with it ([base drag under power](aero.md#drag)). The force
  acts along the axis, so it has no moment about $O$.
- Normal and side forces are taken component by component. Each body and fin set is evaluated at
  its own local flow: $\mathbf{v}_O - \mathbf{v}_{\mathrm{wind}} + \boldsymbol{\omega} \times \mathbf{p}_i$ at the station $\mathbf{p}_i$ the aerodynamics gives it
  (`AeroModel::component_station_m`). That station is the component's small-angle center of
  pressure wherever one model carries the component. Faster than sound, where the
  [shock-expansion method and slender-body theory share one](aero.md#the-body-faster-than-sound-in-a-flight),
  it is not. There the station is joined between the two models while the force joins their slopes
  and moments. The two agree only at the ends of the join. A body can sit between the models at
  every speed (a lip riding half in its boattail's wake). There a tube's station can sit two
  calibers from its own center of pressure
  ([issue #106](https://github.com/nrdptel/fusionspace-eridanus/issues/106)). That moves the lever
  arm the damping below uses by that much. The margins don't use the station. The apogee and the
  drift may lean either way, so a flight inside the join warns of it
  ([the flight-path warnings](../VALIDATION.md#the-flight-path-warnings)). A component's $C_N$ acts along the crossing air $\hat{\mathbf{w}}$ and its $C_Y$
  along $\mathbf{z}_B \times \hat{\mathbf{w}}$ ([Frames](frames.md)), with moments $-qA\, M_N (\mathbf{z}_B \times \hat{\mathbf{w}}) + qA\, M_Y \hat{\mathbf{w}}$ about $O$ from its
  moment coefficients.
- At any angle of attack, a fin set's normal force follows the crossflow $V \sin\alpha$. Its
  model's $C_N = C_{N\alpha}\, \alpha$ is used as $C_{N\alpha} \sin\alpha$, the substitution Niskanen keeps for bodies (eq.
  3.16–3.17). No source in hand gives a large-angle fin model, and stall is not modeled.
  - At small angles nothing changes (1.5% less at 0.3 rad).
  - The force now vanishes for axial flow from the tail as well as from the nose.
  - A force linear in $\alpha$ stays large at $\alpha = \pi$ and flips with rounding noise in the lateral
    velocity. A calm vertical flight falling tail first after apogee then collapsed the step size
    and never landed (`tests::a_calm_vertical_flight_falls_tail_first_and_lands`).
- The rotation's contribution to each local flow is the only aerodynamic pitch and
  yaw damping. At a component's station the rotation adds a velocity $\boldsymbol{\omega} \times \mathbf{p}_i$, which changes the
  angle at which the air meets that component. The station is the lever arm. Where it is not the
  center of pressure (above), the damping is levered from the wrong point by the same distance.
  - Only components with a [normal-force slope](../glossary.md#normal-force-slope) damp this way:
    nose cones, transitions and fin sets. A boattail's slope is negative, so it takes some away.
  - Body tubes give none at small angles: their own slope is 0, and their body lift grows with
    $\sin^2 \alpha$.
  - `hpr-aero` has no pitch or yaw damping coefficients. They would have to replace the
    local-flow damping, not add to it.
  - A flight on another tool's normal force keeps this damping. The table
    ([The normal force from RASAero II](aero.md#the-normal-force-from-rasaero-ii)) gives the
    normal force at the center of mass's airflow. Each component adds only its force in its own
    local flow minus its force in the center of mass's airflow: the part the rotation makes.
    Without rotation that part is exactly zero ([ADR-032][adr-032]).
- The fins' cant drives the roll and the roll rate damps it. The moment about $\mathbf{z}_B$ is
  $qAd\, (C_{l0} \cos\alpha + C_{lp}\, pd/(2V))$, at the center of mass's Mach number. Here $q$ is the dynamic pressure, $A$ and $d$ the
  reference area and diameter, $C_{l0}$ the cant's rolling moment, $C_{lp}$ the damping and
  $p = \omega_z$. The cant's forcing follows the axial flow,
  $\cos\alpha$, so it vanishes broadside and reverses tail first
  (`AeroModel::roll`, [Roll: forcing and damping](aero.md#roll-forcing-and-damping)). The damping
  is written $\rho V A d^2 C_{lp}\, p/4$, so it fades smoothly as the airspeed does. The roll rate also
  changes through inertia coupling (turning about one axis driving turning about another) when a
  rocket's mass isn't symmetric about its axis. While a motor burns, it also changes through the
  jet and the falling inertia in $\mathbf{T}_{21}$. At constant speed a canted rocket spins up to the steady rate
  where cant and damping balance, as the closed form gives
  (`tests::canted_fins_spin_to_the_analytic_balance`).
- Limits:
  - The models are small-angle: no stall, and body lift and fins extended by $\sin\alpha$. They
    overstate the forces at large $\alpha$. In normal flights large $\alpha$ occurs near apogee, where the
    dynamic pressure is small, and off the rail in strong crosswinds. `Sample::angle_of_attack_rad`
    shows where it happens.
  - The normal force and the drag buildup cover Mach 0 to 5
    ([Fins through Mach 1](aero.md#fins-through-mach-1),
    [Drag through Mach 1](aero.md#drag-through-mach-1), [the drag's decision][adr-028]). A fin
    set's CP moves with the Mach number. Each component takes its local airflow at its CP for the
    center of mass's Mach number. At Mach 5 or faster they refuse the
    flow, and the flight stops with `SimError::Aero`. A drag override table covers drag at any
    Mach, but the normal force still stops at Mach 5.

## Running a flight

### Phases

- **Pad.**
  - The rocket starts at rest with the rail's attitude. Its aft end is at the rail's foot: the
    aft-most point it stands on, of its body, of any part on its outside or of its nozzle exits.
    A swept fin whose tip's trailing edge reaches past the tube counts there, as do a tube fin's
    and a pod's aft ends. So the rail's length runs from that point to the rail's top, and a
    standoff under the rocket shortens it.
  - It stays until the force along the rail, $\mathbf{T}_{20} \cdot \mathbf{z}_B$ with $\boldsymbol{\omega} = 0$, exceeds friction
    $\mu \lvert \mathbf{T}_{20\perp} \rvert$ (the liftoff event). $\mathbf{T}_{20\perp}$ is the rail's reaction: the weight, the aerodynamic
    force and the mass terms across the axis.
- **Rail.** One degree of freedom, along the rail, with no rotation:

  $$
  a = \frac{\mathbf{T}_{20} \cdot \mathbf{z}_B - \mu \lvert \mathbf{T}_{20\perp} \rvert}{m}
  $$

  - It ends when the aft edge of the aft-most rail button or launch lug passes the top of the
    rail. In a flight that asks for tip-off it ends sooner, when the guide next ahead of that
    one does. With no guides, it ends when the aft end does.
  - If the speed along the rail falls to zero, the rocket returns to the pad phase, at rest where
    it stopped. It is held there, even if the force down the rail exceeds friction and a real
    rocket would slide back.
- **Tip-off.** Only when the flight asks for it: between the rail and free flight, the rocket
  slides on its last guide alone and pivots about it ([Tip-off](#tip-off)).
- **Free.** Six degrees of freedom until the ground.
- **Rail geometry.**
  - A button's axial extent is its outer diameter, and a lug's is its length.
  - The rocket leaves after traveling $L - (s_{\mathrm{aft}} - s_{\mathrm{guide}})$. RocketPy ends its rail phase when
    the forward button reaches the top (`flight.py:1716-1730`, `effective_1rl`). HPR Sim keeps the
    rocket guided to the last guide ([Loft lesson L26](../decisions-and-roadmap.md#l26)).
  - The pivot about the last guide ("tip-off") is left out unless the flight asks for it
    ([Tip-off](#tip-off)). Without it the rocket leaves the rail with no angular velocity. On thirteen
    example designs, RocketPy's tip-off phase moves the
    landing point by up to 3.765% of the apogee and the apogee by up to 0.414%, one code
    against another ([what leaving tip-off out costs](#what-leaving-tip-off-out-costs)).
  - Friction is Coulomb friction: a user coefficient times the net reaction $\lvert \sum N \rvert$, the force
    pressing the guides onto the rail. With a moment across two buttons (a crosswind at low
    speed), the true friction $\mu(\lvert N_1 \rvert + \lvert N_2 \rvert)$ is larger. No source gives a default coefficient,
    so the default is zero.

### Tip-off

A flight that asks for it flies one more phase between the rail and free flight. That phase is
tip-off, the turn a rocket picks up on its last guide alone. A guide is a rail button or a
launch lug, and the last one is the aft-most. Ask for the phase with
`Simulation::with_tip_off` in the Rust library. The command line and the Python package don't
offer it yet. It is off by default, so every flight on this
site and in the reports flies without it unless it says otherwise.

> **How far to trust it.** One code against another, and no real flight. On RocketPy's thirteen
> example designs the phase agrees with RocketPy's when both fly RocketPy's normal force. The
> turning rate where the last guide leaves is within 0.003 deg/s on all 26 flights. On HPR Sim's
> own normal force the 13 still-air flights agree as well. The 13 crosswind flights do not, by
> up to 4.010 deg/s, and that is open
> ([the tip-off phase against RocketPy's](#the-tip-off-phase-against-rocketpys)). A flight you
> run flies HPR Sim's own normal force, unless you give it a table of another
> (`Simulation::with_normal_force_table`). On HPR Sim's own, a crosswind tip-off is not held to
> RocketPy's.

**What it does.** Once the guide ahead of the last one leaves the top of the rail, the last guide
holds the rocket alone. The rocket keeps sliding up the rail on it, free to pitch and yaw about
it, with roll held. Gravity turns the nose down, and a crosswind turns it downwind when the
air's force acts ahead of the guide. When the last guide's aft edge passes the top, the rocket
is free, with the turning rate it has picked up.

**The equation.** The free equations give the acceleration $\mathbf{a}$ of the nose tip and
the angular acceleration $\dot{\boldsymbol{\omega}}$ the rocket would have with nothing holding it.
The guide adds a force $\mathbf{F}$ on the pivot, square to the rail, and a moment $\tau$ about
the long axis $\mathbf{z}_B$. Write $\mathbf{d}$ for the pivot's position from the center of
mass and $\mathbf{r}$ for the center of mass's from the nose tip. $I_c$ is the inertia about
the center of mass. The two change the motion by

$$
\begin{aligned}
\Delta\dot{\boldsymbol{\omega}} &= I_c^{-1} (\mathbf{d} \times \mathbf{F} + \tau\, \mathbf{z}_B), \\
\Delta\mathbf{a} &= \frac{\mathbf{F}}{m} - \Delta\dot{\boldsymbol{\omega}} \times \mathbf{r}
\end{aligned}
$$

Three conditions fix the three unknowns, the two parts of $\mathbf{F}$ across the rail and
$\tau$. The pivot's acceleration has no part across the rail, in either direction, and the
angular acceleration has none about the long axis. With no friction the three equations are
linear, and friction is solved with them (below). Source:
[RP-TIP], written for this page's equations about the nose tip.

- The pivot is on the rocket's axis, at the middle of the aft-most guide.
- Friction is the rail's, on that force: $\mu \lvert \mathbf{F} \rvert$ on the pivot along
  the rail, against the sliding. It is solved with the force, since it turns the rocket a
  little. RocketPy's rail has none.
- The phase starts when the aft edge of the guide next ahead of the last one passes the top of
  the rail. It ends when the last guide's does. A rocket with one guide, or none, has no such
  stretch and flies as before. A rail too short to hold both guides pivots the rocket from
  liftoff.
- A rocket that stops on its last guide returns to the pad phase, at rest, as on the rail.
- Flying the phase by default is [a later increment](../decisions-and-roadmap.md#m1-14b10).

**For a burnt-out rocket in no air** the three conditions reduce to a pendulum hung from the
guide. Write $\ell$ for the distance along the axis from the last guide to the center of mass.
$I$ is the inertia about the center of mass for a turn in the rail's vertical plane, and
$\theta$ is the rail's angle above the horizontal. The nose turns down at

$$
\dot\omega = \frac{m g\, \ell \cos\theta}{I + m \ell^2}
$$

A test holds the phase to that within 1e-9 of its size on four rails, 5° to 60° off the vertical.

**A worked example.** Take a 20 kg (44 lb) rocket on a rail 5° off the vertical, so
$\theta = 85°$. Its center of mass is 0.80 m (2.6 ft) ahead of its last guide. Its inertia
about the center of mass is 9.0 kg m².

$$
\begin{aligned}
m g\, \ell \cos\theta &= 20 \times 9.80665 \\
&\quad \times 0.80 \times \cos 85° \\
&= 13.675\ \text{N m}, \\
I + m \ell^2 &= 9.0 + 20 \times 0.80^2 \\
&= 21.8\ \text{kg m}^2, \\
\dot\omega &= \frac{13.675}{21.8} = 0.627\ \text{rad/s}^2
\end{aligned}
$$

That is 35.9 deg/s². If the last guide holds the rocket alone for 0.07 s, about what Juno
III's does, it leaves turning nose-down at about 2.5 deg/s. Juno III, a real example, is under
[the tip-off phase against RocketPy's](#the-tip-off-phase-against-rocketpys).

**What it leaves out.** A real button sits a body radius off the axis and has play in its slot,
and a lug slides on a rod. Here the pivot is a pin on the axis that lets nothing through across
the rail. The rail does not flex or whip. The rocket cannot slide back down, as on the rail. No
real flight checks any of it.

### Events and termination

- Thrust-curve knots, burnouts and the time cap are stop times. A burnout that is reached records
  a `Burnout` event.
- Liftoff is the pad force margin rising through zero. It is checked also at each interval's
  start, for a thrust that jumps at a knot. Stall is the speed along the rail
  falling through zero.
- Rail exit is the travel along the rail reaching the exit travel.
- Apogee is the center of mass's ellipsoidal-height rate, $\hat{\mathbf{u}}(\mathbf{r}_{\mathrm{cg}}) \cdot \mathbf{v}_{\mathrm{cg}}$, falling through
  zero. $\hat{\mathbf{u}}$ is the ellipsoid normal at its position. A flight that has let a part go records one
  apogee ([Released mass](released-mass.md)).
- Ground hit is the center of mass's ellipsoidal height reaching the launch site's, descending
  ([Frames](frames.md), [Loft lesson L35](../decisions-and-roadmap.md#l35)).
- A user event is a function of the `Sample`, in free flight.
- A mass shift (`Shift`) is a part starting to move along the airframe, and a release
  (`MassRelease`) a part leaving it. Each comes on its trigger ([Moving mass](moving-mass.md),
  [Released mass](released-mass.md)).
- Atmosphere and wind heights are $h - N$, the height
  [above sea level](../glossary.md#height-above-sea-level-msl). $h$ is the ellipsoidal height.
  The geoid undulation $N$, the height of sea level above the ellipsoid, is given in `Environment`
  (no geoid model).
- The flight ends in exactly one of these ways ([Loft lesson L25](../decisions-and-roadmap.md#l25)):
  - `GroundHit`;
  - `NoLiftoff` (the last burnout passes on the pad);
  - `StalledOnRail` (it lifted off, then stopped on the rail after burnout);
  - `TimeCap`;
  - `StepLimit`.

  Any other failure is an error.
- Turbulence and thrust misalignment are not modeled in a flight. Recovery is modeled, and has
  [its own page](recovery.md). So are motors lit at their own times and a powered separation,
  on [Staging](staging.md).

### Integration settings

- `FlightSettings::default()` uses Dormand-Prince 5(4) with `rtol = atol = 1e-8`
  and unit weights, a one-hour cap and 10⁶ steps.
- Accuracy against cost was measured on Valetudo (K400C) in a 5 m/s (16 ft/s) wind
  from the west. Each apogee is compared with the one at 1e-11 (873.98272 m (2,867.3974 ft)):

  | tolerance, `rtol = atol` (ratio; state units) | apogee error | run time (release build) |
  |--:|--:|--:|
  | 1e-4 | 2.3e-2 m | 0.36 ms |
  | 1e-6 | 6.8e-5 m | 0.59 ms |
  | 1e-8 | 1.1e-6 m | 1.13 ms |
  | 1e-10 | 1e-7 m | 2.72 ms |

### Thrust and mass from another program

A flight can take its thrust, and its mass with its center of mass, from another program, as
numbers against time, in place of its own (`Simulation::with_thrust_history` and
`Simulation::with_mass_history`, since [M2.3c11](../decisions-and-roadmap.md#m2-3c11)). It is
for one job: splitting the gap between HPR Sim's apogee and OpenRocket's by cause, one input
swapped at a time, as a drag table swaps the drag ([ADR-349][adr-349]).
[The split](../decisions-and-roadmap.md#m2-3c12) is in
[Accuracy](../accuracy.md#real-flights-of-the-private-collection): drag is the largest part on
59 of the 68 flights both programs fly. It is not a model, and a flight on
it is not a prediction. It is in the Rust library only, for the validation harness: no `hpr`
command, Python call or file option takes a history.

> **How far to trust it.** These are checks of the swap itself, code against code and against
> closed forms, not of either program's physics. A flight given its own thrust and mass, sampled
> at OpenRocket's times, came back to its own apogee within 0.001% on the two public designs
> tried, where the test holds it to 0.05%. In a vacuum it follows the closed forms to 1e-7 m/s
> (both under *Verification*). Those closed forms are straight up, with no rotation. Of the
> turning terms, only a mass history's inertia about the nose tip and its rate are checked, each
> against its own formula (also under *Verification*). For how close either program comes to a real flight, rely on
> [Accuracy](../accuracy.md#real-flights).

- **Between samples.** A history is a list of samples. Between two of them, $(t_i, y_i)$ and
  $(t_{i+1}, y_{i+1})$, it is the straight line

  $$
  \begin{aligned}
  y(t) &= y_i + (y_{i+1} - y_i)\, s, \\
  s &= \frac{t - t_i}{t_{i+1} - t_i}.
  \end{aligned}
  $$

  Before the first sample and after the last it holds that sample's value. So its rate is the
  slope of the piece it is on, finite, with a jump at each corner, and its second derivative is
  zero inside a piece. Every sample where the slope changes is a stop time, where an integration
  step must end ([time integration](integration.md)), so no step straddles a corner.
- **Thrust.** The history's value is the thrust as flown, along the axis $+z_B$. It takes the
  place of the motors' thrusts with their correction for the air's pressure, which isn't applied
  to it again. It acts at the mean of the lit motors' nozzle exits, so a symmetric cluster's
  thrust turns the rocket no more than HPR Sim's own does. An asymmetric cluster's turning
  moment is left out.
- **What stays HPR Sim's under a thrust history.** The motors still set the burnout event and the
  base area a burning motor frees ([base drag](aero.md)). They also set the mass flow behind jet
  damping and the nozzle terms $\dot m_k$ and $\ddot m_k$ of the equations above. So those
  terms' push, 21 N at Valetudo's liftoff ([internal momentum](../glossary.md#internal-momentum)),
  stays on top of the history's thrust.
- **Mass.** The history gives the whole rocket's mass $m(t)$ and its center of mass $x(t)$, in
  meters aft of the nose tip, the body origin $O$. In body axes the center of mass is
  $\mathbf{r} = (r_x, r_y, -x)$, with HPR Sim's own small offsets off the axis. Then $\dot m$
  and $\mathbf{r}'$ are the history's slopes and $\mathbf{r}'' = 0$.
- **What stays HPR Sim's under a mass history.** The inertia about the center of mass $I_c$. The
  inertia about $O$ comes from it by the parallel-axis theorem with the history's mass and
  center, $I_O = I_c + m\,(|\mathbf{r}|^2 \mathbf{1} - \mathbf{r}\mathbf{r}^{\mathsf{T}})$, with
  $\mathbf{1}$ the identity matrix, so the inertia's difference from OpenRocket's lands in the split's
  remainder. The motors' mass flow stays theirs in the nozzle terms, as under a thrust history.
  Where the history's mass still falls after the motors burn out, its $\dot m$ is in the turning
  terms with no nozzle flow to match, so jet damping is not the classical form there. That too
  lands in the remainder. A structure-mass scale changes only the inertia under a mass history,
  since the history gives the mass.
- **Time.** A history's clock is the flight's: 0 at launch.
  OpenRocket's clock starts at the same instant: its launch and ignition events are both at 0 s.
- **What is refused, with the reason.** An empty sample list, lists of different lengths, times
  that don't rise, a number that isn't finite, a negative thrust, a mass that isn't positive, or a first sample after launch. A
  thrust that doesn't end at zero, since it would stop while the motor still pushed. A thrust
  history on a flight with no motor lit at a known time. Either history on a flight that splits,
  by a separation or an ejection, or that moves or releases a part: a history is the whole
  stack's, with no part's own.

## Verification

Unit tests in `hpr_sim::tests`, `hpr_sim::dynamics::tests` and `hpr_sim::rail::tests`, at the
default settings. The numbers were measured on September 17, 2026.

- A tumbling, spinning Valetudo in a vacuum starts from the nose tip at 500 m (1,640 ft) with
  $\mathbf{v} = (30, -20, 80)$ m/s.
  - The center of mass stays on the closed-form parabola to 1.7e-6 m over 22 s.
  - The angular momentum stays constant to 7.8e-7 (relative).
  - Apogee and ground contact are within 4.7e-7 s and 1.1e-8 s of the parabola's. Those times are
    located (root-found) on the exact ellipsoidal height.
- Falling nose down through uniform air with $C_D = 0.5$, the speed follows $v_t \tanh(g t/v_t)$
  to 5.5e-9 $v_t$. It is the terminal velocity $v_t$ to 1e-5 after 150 s.
- Torque-free precession is the wobble of a spinning body with no torque on it. Valetudo
  ($I_a/I_t$ = 0.0019) spinning at 25 rad/s with a transverse rate turns in body axes at
  $\Omega = (I_a - I_t)\, \omega_z/I_t$ = −24.95 rad/s. The rate matches to 4.7e-7 rad/s over 3 s, and the
  angular momentum in $L$ holds to 1.5e-6.
- The pitch oscillation is checked against linear theory at 100 m/s (328 ft/s), with no drag or
  gravity. The two-state linear model (path turning, restoring moment $K_1$, rotational damping
  $K_2$) predicts a 1.44954 s period. The flight measures 1.44965 s (8e-5), and the decay per half period is within
  0.2% of the model's.
- The flight equations give the classical jet damping to 1e-6. The mass rate equals the motor's
  $-F/c$ to 1e-6, including stencils clipped by an interval's end.
- A powered vertical climb in vacuum runs from 0.5 s to 3.0 s. It ends at the speed of the axial
  equation, integrated independently from the motor and the assembly, to 4.3e-8 m/s.
- On a rail at 60°, friction with $\mu = 0.3$ removes exactly $\mu g \cos E$ from the acceleration along the
  rail.
- Valetudo weathercocks ([Loft lesson L20](../decisions-and-roadmap.md#l20)). In a 5 m/s (16 ft/s)
  wind from the west, its unit axis has an east component of −0.12 at burnout. Its apogee is 96 m (315 ft) upwind, against 1.0 m (3.3 ft, Earth
  rotation) in calm air.
- In a calm vertical flight with no Earth rotation, the rocket falls tail first after apogee and
  still lands, in under 20,000 evaluations.
- RK4 at 2, 1 and 0.5 ms gives the same apogee as Dormand-Prince to 1.2e-5 m
  and 2.1e-7 s.
- A user event on the thrust fires at burnout, and burnout is still
  recorded once.
- A 2,000 m (6,562 ft) rail gives `StalledOnRail`, with only liftoff and burnout recorded.
- Observer errors end the flight with that error. Starts before ignition
  or underground are refused. A cleared recorder records the same rows again. `Simulation`,
  `Environment` and `Recorder` are `Send + Sync`.
- Two runs of one `Simulation` record the same bits for every channel at every step
  ([Loft lesson L24](../decisions-and-roadmap.md#l24)). So does a second `Simulation` built the
  same way.
- A normal flight hits the ground ([Loft lesson L25](../decisions-and-roadmap.md#l25)). A rail with $\mu = 20$ at 60° gives
  `NoLiftoff` at rest. A 5 s cap gives `TimeCap` at exactly 5 s, and a 40-step limit gives
  `StepLimit`.
- A 3 m (9.8 ft) rail is tilted to 1.3 rad ([Loft lesson L26](../decisions-and-roadmap.md#l26)).
  The rail exit comes at the last button's travel to 1e-6 m. Across the rail the rocket stays within 1e-9 m, with no rotation.
  Friction ($\mu = 0.3$) delays the exit and slows it.
- From a 1 m (3.3 ft) rod tilted 5 to 20 degrees, HPR Sim's rocket is on OpenRocket's bearing at
  apogee to within 0.03 degrees. It loses the same apogee to within 0.27 percentage points
  ([`.ork`: a tilted launch rod](../format/ork.md#a-tilted-launch-rod)).
- Events come in order: liftoff, rail exit, burnout, apogee, ground hit.
  Apogee's vertical speed is below 1e-6 m/s and ground contact's height below 1e-6 m. Recorder rows
  fall on the interval or at events.
- The synthetic 54 mm (2.1 in) rocket on an I175 passes Mach 1 and lands. It does both on HPR
  Sim's own drag and on a constant drag table.
- A Valetudo flight to the ground costs about 1.3 ms, measured on October 9, 2026 (`docs/perf.md`).
- Thrust and mass histories (`histories_fly_the_closed_form_of_a_varying_mass_under_constant_thrust`).
  In a vacuum, with gravity $g$ of 9.80665 m/s² (32.174 ft/s²), long after its own motor's
  burnout, Valetudo rises straight up at a speed $v_0$ of 50 m/s (164 ft/s) on a thrust history
  $F$ of 300 N. Its mass history, not Valetudo's own mass, starts at $m_0$, 10 kg, and falls at
  $q$, 1.5 kg/s, so $\dot m = -q$. Its center of mass moves aft at 0.05 m/s (0.16 ft/s), so $r'$
  is −0.05 m/s (−0.16 ft/s) along $+z_B$, which points to the nose. After a time $\tau$, the
  nose tip rises at

  $$
  \begin{aligned}
  v(\tau) &= v_0 - g\tau \\
  &\quad + \frac{F + 2 q r'}{q} \ln \frac{m_0}{m_0 - q\tau},
  \end{aligned}
  $$

  which reaches 101.686 m/s (333.62 ft/s) after 2 s. The flight follows it to 1e-7 m/s. The term
  $2 q r'$ is the axial part of the equations' $-2\dot m\,\mathbf{r}'$. Without it the speed would
  be 0.036 m/s (0.12 ft/s) higher. Each history alone follows its own closed form to 1e-7 m/s
  too: the thrust on the design's burnt-out mass, 8.26 kg, gains $(F/m - g)\,\tau$, 53.0 m/s
  (174 ft/s) in 2 s, and the mass alone the closed form above with $F = 0$. With a corner 1 s in,
  where the mass flow halves and the center's rate drops to 0.02 m/s (0.066 ft/s), a step ends at the corner
  and each piece follows its own closed form to 1e-7 m/s.
- Under a mass history, the inertia about the nose tip and its rate are the parallel-axis
  formula and its derivative, with the history's mass and center and the design's own inertia
  about its center of mass, to 1e-12 and 1e-6 of their sizes
  (`a_mass_history_moves_the_assembly_s_inertia_by_the_parallel_axis_theorem`).
- A mass history read back at 1 s gives the history's mass and center along the axis, with the
  design's offsets off the axis and inertia. A separation, before or after a history, refuses
  it with the reason, and so do a mass shift and a mass release.
- The own-history test (`cargo test -p xtask histories`). Two public designs are flown, sampled at
  the times of OpenRocket 24.12's own flight of each (`validation/fixtures/ork/openrocket-histories.json`),
  and flown again on those samples. Both are a 60 mm (2.4 in) airframe with a conical nose and
  three fins: `pods-none` on an AeroTech H128W, the control of
  [a tilted launch rod](../format/ork.md#a-tilted-launch-rod), and `mach-probe` on a J570W, made
  for this test. On `pods-none`, a 251-sample flight to Mach 0.76, the apogee
  goes from 782.7197 m (2,567.978 ft) to 782.7214 m (2,567.984 ft), +0.00022%. On `mach-probe`, a
  567-sample flight to Mach 1.73, it goes from 2,030.4623 m (6,661.622 ft) to 2,030.4496 m
  (6,661.580 ft), −0.00063%. The bound is 0.05%, about a
  thirty-fifth of the 1.73% mean absolute gap between the two programs' apogees on 60 real flights
  ([accuracy](../accuracy.md#real-flights-of-the-private-collection)). On the same
  flights, 5% more thrust raises the apogee by 2.8% and 3.0%, and 20% more mass delays the rail
  exit by 7.9% and 7.2%: both histories are flown.

### A free body against NASA's check cases

NASA's Engineering and Safety Center published check cases for flight simulations that move a
body in all six degrees of freedom **[N15]**. Each case is a simple body flown from a stated
start. Six codes flew the cases in the air, labeled SIM 1 to SIM 6 without their names: five
of NASA's own and JSBSim, an open-source simulator. Five or six posted a time history for each
case, beside the report. The report names no code as
right: the codes' agreement with each other is the measure.

HPR Sim flies eight of the cases with a free body, `hpr_sim::FreeBody`: a mass and an inertia,
with no design, no motor and no aerodynamics. It is not a second model. Its gravity and
Coriolis force come from the flight's Earth model, its air and wind from the flight's
environment at the height a rocket's flight reads them, its accelerations from the equations
above, and its steps from the integrator at the default tolerances. A case's damping or drag
is a load the test adds. At every step it forms a force and a moment from the state, the air
and the velocity through the air. A test
(`the_derivative_is_a_coasting_rockets`) evaluates a coasting rocket by the equations a flight
integrates, and the free body of that rocket's mass and inertia at the same state: the two
agree to rounding.

- **Case 1, the dragless sphere.** A sphere of 1 slug (14.59 kg) is dropped from 30,000 ft
  (9,144 m) over 0° N, 0° E, at rest relative to the Earth, and falls for 30 s with no drag.
  Its altitude is compared.
- **Case 2, the tumbling brick.** A brick with three unequal moments of inertia starts at the
  same place, turning at 10, 20 and 30 deg/s about its three axes relative to the stars, with
  no aerodynamics. Its three body rates relative to the stars (inertial rates) are compared
  over 30 s.
- **Case 3, the tumbling brick with damping.** Case 2 with a damping moment about each axis.
  The report gives three coefficients of −1, a reference area, a span and a chord, but no
  formula. The test takes the usual one: about each axis $\tfrac14 \rho V S l^2 C \omega$,
  which is the dynamic pressure $\tfrac12\rho V^2$ times $S l C$ times the rate made
  dimensionless, $\omega l / 2V$. Here $V$ is the speed through the air, $\omega$ the rate
  relative to the stars about that axis, and $l$ the span for roll and yaw and the chord for
  pitch. The codes split on the rate. SIM 4 damps the one relative to the stars, and SIM 1 comes
  nearer it than the other. SIM 5 and SIM 6 damp the one relative to the Earth. SIM 2 fits
  neither. The test takes the first.
  The inertial rates are compared.
- **Cases 6 to 10, the sphere with drag.** Case 1's sphere with a drag coefficient of 0.1,
  acting along its velocity through the air whichever way it points. It is dropped from
  30,000 ft in still air (case 6), in a steady 20 ft/s (6.1 m/s) wind from the west (7), and in
  a wind from the west that grows by 3 ft/s every 1,000 ft of height, 70 ft/s (21 m/s) at the
  start (8). In cases 9 and 10 it is shot from sea level at 1,000 ft/s (305 m/s) up and
  1,000 ft/s east or north. Its position is compared: the distance between HPR Sim's and the
  code's. Each code's position is read from the latitude, longitude and altitude in its file,
  since two of the codes write no Earth-fixed position.

**How to read the table.** A difference is HPR Sim's value less a code's, the largest over the
30 s; for a position, the distance. The spread is the largest difference between any two codes.
One code is left out when leaving it out cuts the spread by more than half. As the report
numbers the codes, that is SIM 1 in cases 1 and 3, SIM 6 in case 2, SIM 3 in cases 6 to 8 and
SIM 2 in case 10. The model difference is a named difference between HPR Sim's model
and the codes', worked out apart from the flight. The last column is what that leaves of the
largest difference from any kept code.

| Quantity | Codes kept | The codes' spread | HPR Sim's largest difference from a kept code (size) | Model difference | Most left after the model difference |
|---|---|--:|--:|--:|--:|
| Case 1, altitude | 5 of 6 | 0.1424 mm (0.0056 in) | 19.51 mm (0.77 in) | −19.50 mm (−0.77 in) | 0.1364 mm (0.0054 in) |
| Case 2, inertial roll rate | 4 of 5 | 0.0008991 deg/s | 0.01077 deg/s | +0.01077 deg/s | 0.0008818 deg/s |
| Case 2, inertial pitch rate | 4 of 5 | 0.001242 deg/s | 0.01633 deg/s | −0.01633 deg/s | 0.001005 deg/s |
| Case 2, inertial yaw rate | 4 of 5 | 0.0003532 deg/s | 0.005500 deg/s | +0.005500 deg/s | 0.0001268 deg/s |
| Case 3, inertial roll rate | 4 of 5 | 0.003609 deg/s | 0.004873 deg/s | −0.003755 deg/s | 0.003176 deg/s |
| Case 3, inertial pitch rate | 4 of 5 | 0.003792 deg/s | 0.005633 deg/s | −0.005609 deg/s | 0.001123 deg/s |
| Case 3, inertial yaw rate | 4 of 5 | 0.003503 deg/s | 0.005102 deg/s | +0.004433 deg/s | 0.0007793 deg/s |
| Case 6, position | 5 of 6 | 85.03 mm (3.3 in) | 102.6 mm (4.0 in) | 180.2 mm (7.1 in) | 77.57 mm (3.1 in) |
| Case 7, position | 5 of 6 | 85.24 mm (3.4 in) | 102.8 mm (4.0 in) | 180.4 mm (7.1 in) | 77.63 mm (3.1 in) |
| Case 8, position | 5 of 6 | 87.74 mm (3.5 in) | 105.0 mm (4.1 in) | 182.3 mm (7.2 in) | 77.68 mm (3.1 in) |
| Case 9, position | 6 of 6 | 1,989 mm (78 in) | 1,980 mm (78 in) | 1,253 mm (49 in) | 1,991 mm (78 in) |
| Case 10, position | 5 of 6 | 1,986 mm (78 in) | 1,977 mm (78 in) | 1,251 mm (49 in) | 1,988 mm (78 in) |

For a position the model difference is a distance, and it differs from code to code; the table
gives the largest. Distances don't subtract, so the last column need not be the difference
of the two before it.

Cases 1 to 3 and 6 to 8 come out past the codes' spread. Each is traced. A named difference
between the models, computed apart from the flight, is taken off at each code's largest
difference. What is left is inside the spread
([the decisions that trace them][adr-353], [and the later cases][adr-356],
[the report][check-cases]). In cases 1 to 3 the difference explains the gap; in cases 6 to 8 it
does not explain SIM 1's, below. A stricter reading, below, leaves case 2 just past the spread.
Cases 9 and 10 come out inside it as flown.

- **Case 1 is the gravity model.** HPR Sim's sphere ends 19.51 mm (0.77 in) lower after a fall of 4.4 km
  (2.7 mi). It falls in WGS 84's [normal gravity](../glossary.md#normal-gravity)
  ([Gravity](gravity.md)), which holds every term of the model Earth's field. The codes fall
  in a field cut off after its second term, $J_2$, the one for the Earth's bulge at the equator.
  On the equator the next term, $J_4$, pulls harder by about 4.4 parts in a million of the
  attraction, 4.34e-5 m/s². Held for 30 s, that acceleration moves a body 19.5 mm (0.77 in),
  which a test of the tool works by hand (`the_gravity_gap_is_j4s_over_the_fall`). The tool integrates the exact gap along the fall
  and gets 19.50 mm (0.77 in).
- **Case 2 is the Earth's rate.** The codes turn the brick by Euler's equations in its rate
  relative to the stars. HPR Sim's rotational equations use the rate relative to the launch
  frame, which turns with the Earth, and add the Earth's rate back only when an inertial rate
  is read out. The rates the two sets of equations turn start one Earth's rate apart, 0.0042
  deg/s, and the inertial rates read out start the same. On a tumbling body of three unequal
  moments the inertial rates then drift apart, by up to 0.01633 deg/s in pitch within the
  30 s, 3.9 times the Earth's rate. Integrating both sets of equations apart from the flight
  gives the differences in the table. What the missing term costs a rocket's flight is not measured: that, and whether
  to carry the Earth's rate in the rotational equations, is [#1521][i-1521].
- **Case 3 is the Earth's rate again.** The same two sets of equations are damped alike, on
  the rate relative to the stars, along the same fall. They leave at most 0.003176, 0.001123
  and 0.0007793 deg/s, each inside the spread. Read at every sample, roll and yaw stay inside.
  Pitch comes to the spread itself, 0.003791 against 0.003792 deg/s. Against SIM 4, which damps as the test does, at most 0.0002691 deg/s
  is left. Read at every sample, the most is left in roll and pitch against SIM 5 and SIM 6.
  Those damp the rate relative to the Earth. In yaw the most is left against SIM 2.
- **Cases 6 to 8 are the gravity and the codes' own air.** HPR Sim's sphere ends within 21 mm
  (0.8 in) of three codes and 102.6 to 105.0 mm (4.0 to 4.1 in) from SIM 1. The codes don't fly
  the same air. Set against HPR Sim's 1976 standard atmosphere, SIM 4's density is within
  4.1e-7 of it. SIM 1's is up to 1.35e-3 denser: more drag, so its sphere falls less. The model
  difference flies a point mass twice. Once it falls in the codes' gravity through the code's
  own density, read from its file; once in HPR Sim's gravity and atmosphere. Less that, HPR Sim
  sits 0.004812 to 0.01569 mm (0.00019 to 0.00062 in) from the nearest code. It sits 77.57 to 77.68 mm (3.05 to 3.06 in) from
  SIM 1, inside the spread. SIM 1 is brought inside, not explained. Its own air, flown, would
  put its sphere 180.2 mm (7.1 in) above HPR Sim's; its file is 102.6 mm (4.0 in) above. With
  the gravity alone, cases 6 and 7 were left 0.01 mm (0.0004 in) past the spread, so the air is part of the
  trace ([the decision][adr-356]).
- **Cases 9 and 10 are inside the spread, as flown.** Shot kilometers downrange, HPR Sim ends
  1,980 and 1,977 mm (78 in) from the farthest kept code, SIM 3. That is inside spreads of 1,989
  and 1,986 mm (78 in): the codes themselves sit about 2 m (6.6 ft) apart. HPR Sim ends 15.60 mm (0.61 in)
  from SIM 4 and 13.68 mm (0.54 in) from SIM 5 in both. Less the same model difference it sits
  0.03830 mm (0.0015 in) and 0.04217 mm (0.0017 in) from SIM 5. But it sits 1,991 and 1,988 mm
  (78 in) from SIM 3, just past the spread, so no cause is claimed for these cases. Here the launch
  frame's flat axes carry the body far from the pad. So the cases hold the conversion that gives
  apogee and landing points, to the codes' own agreement ([the report][check-cases]).

**A stricter reading.** The last column takes the model difference at the moment of each
code's largest difference. Taken at every sample of the 30 s instead, what is left is
0.1369 mm (0.0054 in) in case 1, inside the spread, and 0.0008993, 0.001242 and 0.0003533 deg/s in case 2:
over the spread by 0.0000001701, 0.00000007201 and 0.00000007634 deg/s. In case 2 the spread
is set by one kept code, SIM 2, against two that agree with each other, so a flight that sat
exactly on those two would differ from SIM 2 by the spread and no less. HPR Sim, less the
model difference, sits 0.0000005866, 0.0000009229 and 0.0000002790 deg/s from the code nearest
it. That distance is the integrator's step error at the default tolerances: at tolerances of
1e-12 it falls to 0.000000002126, 0.000000003051 and 0.000000001089 deg/s. By this reading
case 2 is not inside the spread: the excess is small, named and left open. The report's
"traced" is the first reading's.

**What the comparison does not see.**

- **The Coriolis force's sign, in case 1.** The force carries the falling sphere east, and that
  eastward speed is in turn pushed up, so the fall ends higher whichever way the Earth is taken
  to turn. Case 1's altitude holds the size of the term, 7.0 mm (0.28 in) of the fall, which a
  test pins on the free body against its closed form. It does not hold the sign. Cases 6 to 10
  compare whole positions, the eastward drift with them, so they do.
- **What SIM 1 does in cases 6 to 8.** 77.6 mm (3.1 in) of its difference is not accounted
  for, only inside the spread. SIM 3, left out there, is about 171 mm (6.7 in) off.
- **The attitude.** The codes' files also hold the body's Euler angles, which are not
  compared ([#759][i-759]). An error in the attitude reaches the inertial rates only through
  the Earth's rate turned into the body.
- **An error the kept codes share.** The codes are the reference, so it would pass unseen.

The codes' files carry no stated terms, so they are not committed: `cargo xtask check-cases`
reads them from an archive that `cargo xtask refs fetch` downloads and checks, and the
committed report holds only differences and spreads.

### Whole flights against RocketPy

`cargo xtask validate` flies six of RocketPy's example rockets from the pad to the ground. It
compares fifteen numbers of each with RocketPy 1.13.0's own flight
([ADR-021][adr-021], the whole-flight comparison). Both codes fly one declared drag coefficient,
a constant $C_{D0}$ of 0.5 on the same reference area. So what is compared is the equations of
motion, the motor and the air, not the drag. The rest is RocketPy's where HPR Sim has it: its gravity
formula, standard atmosphere and frictionless rail. So are the example's rail angles, parachutes
and motor, and a declared wind.

| result | value |
|---|---|
| cases scored | 6, one of them (Prometheus 2022) past Mach 1, since [M1.8a](../decisions-and-roadmap.md#m1-8a) |
| height, speed, time, acceleration | all scored, all within 3% of RocketPy's |
| largest of those | +1.783%, Bella Lui's peak acceleration, on the rail |
| largest in apogee | +1.208%, Prometheus 2022; RocketPy flown with HPR Sim's body lift and rail release comes within 0.01% ([case file](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/cases/flight-prometheus-2022-generic-motor.toml)) |
| path without wind (drift of apogee and landing) | all scored, within 1.9% (largest −1.811%, Valetudo's landing, in still air) |
| path in wind | Calisto's scored (largest +1.257%), and NDRT 2020's landing; Juno III's, Bella Lui's and Prometheus 2022's, and NDRT 2020's apogee drift, differ by 4.3 to 38% and are reported, not scored: HPR Sim's body lift and rail release, and Juno III's fin slope ([ADR-026][adr-026]) |

What the two codes still do differently, and how much it moves:

- **The wind.** A rocket that leaves the rail slowly in a wind meets the air at a steep angle.
  Juno III, at 18 m/s (59 ft/s) in an 8.5 m/s (28 ft/s) wind, is 26° off the airflow. There HPR Sim's normal force includes
  body lift ([Aerodynamics](aero.md#bodies-of-revolution)), which RocketPy's leaves out. Much of
  it acts ahead of the center of mass, the nose's above all. So it moves the center of pressure
  forward and weakens the turn into the wind. HPR Sim turns into it less: Juno III's apogee is
  245.3 m (805 ft) from the pad, against 396.6 m (1,301 ft) in RocketPy. HPR Sim cannot model the
  airfoil lift curve Juno III's example gives its fins, so it flies a flat-plate fin slope. Given
  HPR Sim's body lift, its rail release and that fin slope, RocketPy puts Juno III's apogee
  248.3 m (815 ft) out. Every windy drift then comes within 1.3% of HPR Sim's ([ADR-026][adr-026]).
  HPR Sim's growth of drag with the angle of attack moves no drift by more than 0.1%.
- **RocketPy's equations, corrected.** HPR Sim's equations of motion follow RocketPy's technical
  documentation, which measures the center of mass from the
  [center of dry mass](../glossary.md#center-of-dry-mass). RocketPy 1.13.0's code reads that
  vector the other way round. So during the burn it takes the turning moments about the wrong
  point, and its rockets turn into the wind too far. The fix is
  [a pull request to RocketPy](https://github.com/RocketPy-Team/RocketPy/pull/1196), merged on
  November 16, 2026. It is built on
  [an earlier one](https://github.com/RocketPy-Team/RocketPy/pull/1188), merged too. No
  release carries either yet. RocketPy 1.13.0 as installed still has
  the error, and the comparison applies both fixes. Without them,
  HPR Sim's drifts in wind were up to −60.8% short of RocketPy's at apogee and +151% beyond it at
  landing ([ADR-026][adr-026],
  [issue #50](https://github.com/nrdptel/fusionspace-eridanus/issues/50)).
- **The rail.** HPR Sim's rail equation keeps the terms for the center of mass moving inside the
  body as the propellant burns. RocketPy's rail equation (`udot_rail1`) leaves them out. At a
  sharp ignition spike, with thrust and mass the same to five digits, HPR Sim's acceleration is 1.2
  to 1.3 m/s² higher. HPR Sim also guides the rocket until its last rail button leaves, where
  RocketPy frees it at the first.
- **Calisto's two peaks.** Its acceleration peaks twice, 0.9% apart, and the rail terms make
  HPR Sim's maximum the other peak. So the time of the peak (−96.8%) is reported but not scored.
- **The parachutes.** RocketPy counts the air a canopy drags along (added mass). HPR Sim has none. So
  the peak deceleration as NDRT 2020's main opens is 83% higher in HPR Sim, and that number is
  reported but not scored. The speeds at landing agree within 0.03%.

The metrics are measured as RocketPy defines them. Speeds and accelerations are at the center of
dry mass. Heights are from that point's height at launch, since RocketPy's starts at the ground.
The rail exit comes when the rocket has traveled RocketPy's `effective_1rl`, the forward button at
the top of the rail. HPR Sim's own rail-exit event waits for the last one. Each case file argues
its tolerances. [Accuracy][accuracy] gives every result.

### What leaving tip-off out costs

HPR Sim flies without tip-off unless a flight asks for it ([Tip-off](#tip-off)). RocketPy's tip-off phase puts the cost at up to 3.765% of the apogee in
the landing point and 0.414% in the apogee. RocketPy flew thirteen example designs. Both are measured
from a rocket held straight to its last guide, as HPR Sim holds it. The landing point moves
farther from the pad in most flights, and the largest apogee move is down. That is one code
against another: the phase is in RocketPy's development branch, in no release, and no flight
checks it. The landing point's cost is past the plan's bar of 1% of the apogee, so HPR Sim
gained [a model of the pivot](../decisions-and-roadmap.md#m1-14b11). It is held to RocketPy's
on RocketPy's normal force and in still air, not yet in a crosswind on HPR Sim's own
([below](#the-tip-off-phase-against-rocketpys)).

Tip-off is the turn a rocket picks up between its guides leaving the rail. Once the forward
guide is off, the last one holds the rocket alone, and gravity and the wind turn it about that
guide. By default HPR Sim keeps the rocket straight until the last guide leaves, then frees it
with no turning rate.

`cargo xtask tip-off` flies the comparison and writes [the report][tip-off]
([the decision record][adr-357]):

- **What flies.** RocketPy, at the merge of its
  [pull request #1217](https://github.com/RocketPy-Team/RocketPy/pull/1217) (`65b9838f`),
  flies the thirteen
  public designs built from its examples and tests, on its own inputs. They are twelve rockets;
  Calisto comes twice, from RocketPy's getting-started notebook and from its tests. A Calisto id
  in the report ends in the motor's position in meters, in the rocket's own coordinates as its
  source sets it. Each source measures from its own zero point, and the two zero points are
  0.118 m (4.6 in) apart. So in both the nozzle sits 2.533 m (8.31 ft) behind the nose's tip.
  The fins differ, and the notebook's rail buttons and its center of mass without the motor sit
  0.118 m (4.6 in) farther back. Each design flies from its example's rail on a constant drag
  coefficient of 0.5. It flies in still air and in a 20 mph (8.94 m/s) wind square across the
  rail's heading.
- **Three ways.** With the phase off, RocketPy frees the rocket where its forward button
  leaves the rail. With it on, the rocket slides on, free to pitch and yaw about its last
  button, until that button leaves too. Guided, the phase is off on a rail longer by the
  distance between the buttons. That holds the rocket straight to where its last button leaves
  the real rail, as HPR Sim holds it.
- **What HPR Sim's release leaves out** is the phase on less the guided flight.
  - The apogee moves by at most −0.414%: −10.9 m (−36 ft) for Juno III in still air. None of
    the 26 apogees moves by 1%.
  - The landing point moves by more than 1% of the apogee in 6 of the 26 flights. They are on
    three rockets: Valetudo, Juno III and Prometheus 2022. The largest share is 3.765% of the apogee,
    28.5 m (93 ft) for Valetudo in the crosswind. The largest distance is 92.0 m (302 ft), for
    Juno III in the crosswind.
  - With the phase on, the rocket lands farther from the pad in 23 of the 26 flights. One of
    them, the vertical rail's in still air, moves by 6 mm (0.2 in), which the report's table
    rounds to zero. In those 23, a rocket guided to its last button lands nearer the pad than
    one that pivots. For a landing-area estimate that understates how far the rocket travels:
    the flattering side. In the other three, all in the crosswind, the guided rocket lands
    0.3 to 3.6 m (1 to 12 ft) farther from the pad.
- **The pivot itself is small.** With the phase on, a rocket leaves its last button turning
  at up to 2.38 deg/s, its axis at most 0.083° off the rail's.
- **[The plan's rule][adr-351]** set the phase on against the phase off. That moves 9 of the 26 landing
  points by more than 1% of the apogee, by up to 4.585%, and one apogee by +1.165%. Part of
  that is RocketPy's early release and not the pivot. Bella Lui, freed at its forward button
  in the crosswind, has turned into the wind at 15.78 deg/s by the time its last button
  leaves. With the phase on it turns at 0.60 deg/s.
- **A worked example.** Juno III leaves a 5.20 m (17.1 ft) rail at 85°, its buttons 1.410 m
  (4.63 ft) apart. In still air its last button leaves at 21.53 m/s (70.6 ft/s), 0.071 s
  after the forward one. With the phase on, gravity has it turning nose-down at 2.26 deg/s
  by then, its axis 0.079° off the rail's line; guided, it isn't turning. Guided, it reaches
  2,623.9 m (8,609 ft). With the phase on it reaches 2,613.1 m (8,573 ft) and lands 91.3 m
  (300 ft) farther from the pad. Each figure is rounded on its own; unrounded, the apogee moves
  down by 10.86 m (35.6 ft).
- **The reference moved once.** The report flies one fixed commit of RocketPy's source. At
  first that was the merge of RocketPy's
  [pull request #920](https://github.com/RocketPy-Team/RocketPy/pull/920), which added the
  phase. RocketPy then rewrote the phase ([RP-TIP] says what changed), and the report now
  flies the rewrite ([the decision record][adr-363]). No figure quoted on this page changed.
  Between the two commits, a turning rate where the last button leaves moved by at most
  0.000020 deg/s, and no flight's apogee or landing point by more than 1.04 mm (0.041 in).
  Each of the four differences HPR Sim's phase is held to is under a hundredth of the bar it
  is held to. The flights without the phase moved as far as those with it, so these are
  differences between two commits of RocketPy, not the measured effect of the rewrite: the
  report does not test which change made each. Its last table gives each flight, and the
  earlier report's data is kept as
  `validation/fixtures/flight/rocketpy-tip-off-at-pull-920.json`.
- **What the comparison leaves out.** RocketPy's pivot is a frictionless pin with roll held;
  a real button has play in its slot. The guided flight frees the rocket at its last
  button's position, and HPR Sim half a button's length later, at the button's aft edge. The
  thirteen fly one drag coefficient, and a wind that does not change with height. One
  rocket's still-air flights open the apogee parachute with no lag, and the report says
  which and why.

### The tip-off phase against RocketPy's

HPR Sim's [tip-off phase](#tip-off) is inside every bar below on all 26 flights when both codes
fly RocketPy's normal force. On HPR Sim's own normal force it is on the 13 still-air flights
and not on the 13 crosswind flights. That is one code against another, with no flight to check
either.

`cargo xtask tip-off-model` flies the comparison and writes [the report][tip-off-model]. The
gate and CI hold it (`--check`). [The decision record][adr-358] gives what is
held and what stays open.

- **What flies.** The same thirteen designs, rails, sites, winds, parachutes and drag
  coefficient as RocketPy's flights above. HPR Sim flies each guided, as it does by default,
  and with the phase. A design whose dry mass, reference area, motor or distance between its
  last two guides differs from what
  RocketPy flew is refused.
- **Twice over.** On *RocketPy's normal force*, HPR Sim flies the force RocketPy flew. That is
  RocketPy's slope at the Mach number times the angle of attack, at RocketPy's center of
  pressure, read from RocketPy with each flight. The phase then meets one drag and one normal
  force in both codes. Damping in pitch and yaw stays each code's own. On *its own*,
  HPR Sim flies its own aerodynamics.
- **The bars** are [the tip-off decision record's][adr-357]. Where the last guide leaves, the
  turning rate is within 10% of RocketPy's or 0.1 deg/s, whichever is larger. The angle off
  the rail is within 10% or 0.005°. Each is compared as a pair. Nose-down is below the rail's
  line in the rail's vertical plane. Nose-right is to the right of the rail's heading, which
  is downwind in the crosswind. The
  two codes' apogee moves are within 0.2% of RocketPy's guided apogee, and their landing-point
  moves, as vectors, within 1% of it.
- **On RocketPy's normal force, 26 of 26 flights are inside every bar.** The largest
  difference in the turning rate is 0.003 deg/s against a bar of 0.238 deg/s, and in the angle
  0.0001° against 0.0083°. In the apogee move it is 0.002% of the apogee, and in the
  landing-point move 0.029%.
- **On its own normal force, 13 of 26 are:** all 13 in still air, none in the crosswind.
  - All 13 crosswind flights are past the turning rate's bar, by up to 4.010 deg/s against a
    bar of 0.100 deg/s. 10 are past the angle's, by up to 0.0671°. 3 are past the apogee
    move's, by up to 0.379% of the apogee. 4 are past the landing-point move's, by up to
    4.082%.
  - The cause is the normal force. In the crosswind the air meets the rocket at 9.7° to 25.6°
    as the last guide leaves. There HPR Sim's own normal-force coefficient is 1.23 to 2.71
    times RocketPy's, and its moment about the last guide 0.35 to 15.20 times. The turn
    follows the moment. On 10 of the 13 the force acts ahead of the guide, and the nose turns
    downwind, farther on HPR Sim's own. On 3 it acts behind the guide, and the nose turns
    upwind, less far on HPR Sim's own. In signed terms the rate is farther nose-right on all
    13, by 0.136 to 4.011 deg/s.
  - Which force is nearer a real rocket's at those angles, no reference here says
    ([issue #1538][i-1538]). HPR Sim's adds [body lift](aero.md#body-lift), which RocketPy
    leaves out. Holding these 13 to the bars is
    [an open increment](../decisions-and-roadmap.md#m1-14b12).
- **Which way leaving the phase out errs.** With the phase, HPR Sim lands farther from the pad
  in 22 of the 26 flights on RocketPy's normal force, as RocketPy does. On its own it does in
  20. A move of under 0.1 m (0.33 ft) either way is not counted here. The count of 23 under
  *What leaving tip-off out costs* includes the vertical rail's still-air flight, which moves 6 mm (0.2 in). For a landing-area
  estimate, a flight without the phase lands nearer the pad than one with it in most of these.
- **A worked example.** Juno III in still air, on HPR Sim's own normal force, leaves its last
  guide at 21.5 m/s (70.7 ft/s), turning nose-down at 2.254 deg/s. RocketPy's turns at
  2.255 deg/s. Its axis is 0.0788° below the rail's line, and RocketPy's 0.0789°. The phase
  lowers HPR Sim's apogee by 11.33 m (37.2 ft) and RocketPy's by 10.86 m (35.6 ft). It lands
  HPR Sim's rocket 95.4 m (313 ft) farther from the pad, and RocketPy's 91.3 m (300 ft).
- **What the comparison leaves out.** Everything the comparison above does: one drag
  coefficient, a wind that does not change with height, and a frictionless rail. It also takes
  RocketPy's normal force as RocketPy states it for a rocket at rest in the air stream. In
  flight RocketPy works each surface's angle from the air's speed at that surface. And in the
  crosswind the rocket leaves the rail at 9.7° to 25.6° to the air. Both codes carry
  small-angle models there, and 7 of the 13 are past the 15° the accuracy work assumes.

[adr-007]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0007-design-tree-stations-placement-automatic-radii.md
[adr-011]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0011-rigid-body-flight-equations-of-motion.md
[adr-353]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0353-nasas-check-cases-1-and-2-both-differences-past-the-codes-spread-traced.md
[adr-356]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0356-nasas-check-cases-3-and-6-to-10-the-tests-loads-and-the-codes-own-air.md
[check-cases]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/check-cases.md
[i-759]: https://github.com/nrdptel/fusionspace-eridanus/issues/759
[i-1521]: https://github.com/nrdptel/fusionspace-eridanus/issues/1521
[adr-349]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0349-m2-3c6-in-four-the-public-nose-probe-first.md
[adr-032]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0032-normal-force-overrides-from-rasaero-ii-the.md
[report]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/latest.md
[fixture-report]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/fixture-flights.md
[adr-021]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0021-whole-flights-against-rocketpy-what-is-compared.md
[adr-351]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0351-m1-14b-in-eight-the-basic-finners-normal-force-first.md
[adr-357]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0357-tip-off-measured-with-rocketpys-past-the-plans-bar-the-model-in-two-more-increments.md
[tip-off]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/tip-off.md
[tip-off-model]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/tip-off-model.md
[adr-358]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0358-m1-14b9-in-two-the-tip-off-phase-on-rocketpys-normal-force-first.md
[adr-363]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0363-m1-14b10-in-three-the-tip-off-reference-at-rocketpys-rewritten-phase-first.md
[i-1538]: https://github.com/nrdptel/fusionspace-eridanus/issues/1538
[adr-026]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0026-the-path-in-wind-rocketpys-corrected-equations.md
[adr-028]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0028-drag-through-mach-1-niskanens-appendix-b-stoneys.md
[accuracy]: ../accuracy.md#whole-flights-against-rocketpy
