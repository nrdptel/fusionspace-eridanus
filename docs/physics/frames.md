# Frames and sign conventions

This page defines the axes and signs all of HPR Sim shares, from the Earth-centered frame to the
rocket's body, attitude and launch angles.

## In short

- **What it models:** the directions and signs the whole simulator shares. They define a frame
  fixed to the Earth at its center, the launch-site frame (east, north, up) and the rocket's body
  frame. They also define its angle to the airflow, its attitude (which way it points) and its
  launch angles.
- **Sources:** the WGS 84 standard, NGA.STND.0036 (2014), as in [Geodesy](geodesy.md). J. Solà,
  *Quaternion kinematics for the error-state Kalman filter* (2017). RocketPy 1.13.0, for its
  launch-angle convention.
- **How well it is validated:** unit tests, and one check against another simulator. For 8 rail
  setups, launch angles give RocketPy's starting attitude to 1e-12 rad. Over 1e6 integration
  steps, attitude stays within 1e-9 rad of the exact answer. No real-flight check.
- **What it leaves out:** heights are above the WGS 84 ellipsoid, not sea level. The two are up
  to about 100 m (328 ft) apart. The attitude equations leave out the Earth's rotation rate,
  7.3e-5 rad/s, which is tiny next to a rocket's pitch rates
  ([the rigid-body flight decision][adr-011]).

## Sources and units

Where the frames are defined in the code, and the units and rotation rules every frame uses.

### Code and sources

This is the single definition of the frames, and every crate follows it. Code:
`hpr_core::{geodesy, frames, attitude}`. [The frames and gravity decision][adr-003] records why
these choices were made.

### Units and angles

- SI throughout: meters, seconds, kilograms, radians. Degrees exist only at I/O boundaries
  (`Geodetic::from_degrees`).
- Every rotation is right-handed. $R_x(\theta)$, $R_y(\theta)$ and $R_z(\theta)$ turn vectors by $+\theta$ about the
  named axis (active rotations).

## The frames

The three frames: one fixed to the Earth, one at the launch site, and one fixed to the rocket.

### Earth-centered Earth-fixed (ECEF)

The WGS 84 conventional terrestrial frame:

- origin at the Earth's center of mass;
- $+Z$ along the rotation axis toward the north pole;
- $+X$ through the prime meridian at the equator;
- $+Y$ completes the right-handed set (90° E).

A **geodetic position** $(\varphi, \lambda, h)$ gives the latitude $\varphi$ (positive north, in $[-\pi/2, \pi/2]$)
and the longitude $\lambda$ (positive east). The height $h$ is above the ellipsoid, along its
normal.

$h$ is **ellipsoidal height, not height above mean sea level**. The two differ by the geoid
undulation $N$ ($h = H + N$, with $\lvert N \rvert$ up to about 100 m (328 ft)). Inputs quoted above sea level must be
converted before use. HPR Sim has no geoid model, so a flight takes $N$ at the launch site as an
input. [Atmosphere](atmosphere.md#height-datum) shows where it is used.

### Launch frame `L` (East-North-Up)

- The origin is the launch site's geodetic position $(\varphi_0, \lambda_0, h_0)$, at the pad on the ground.
- The axes are $x_L$ east, $y_L$ north, $z_L$ up along the ellipsoid normal at the origin.
- To ECEF, $\mathbf{r}_{\mathrm{ECEF}} = \mathbf{r}_0 + R\,\mathbf{r}_L$. The columns of $R$ are, in ECEF components:

  $$
  \begin{aligned}
  \hat{\mathbf{e}} &= \begin{pmatrix} -\sin\lambda_0 \\ \cos\lambda_0 \\ 0 \end{pmatrix}, \\
  \hat{\mathbf{n}} &= \begin{pmatrix} -\sin\varphi_0 \cos\lambda_0 \\ -\sin\varphi_0 \sin\lambda_0 \\ \cos\varphi_0 \end{pmatrix}, \\
  \hat{\mathbf{u}} &= \begin{pmatrix} \cos\varphi_0 \cos\lambda_0 \\ \cos\varphi_0 \sin\lambda_0 \\ \sin\varphi_0 \end{pmatrix}
  \end{aligned}
  $$

- $L$ is Earth-fixed, so not inertial. It turns with the Earth at
  $\boldsymbol{\Omega} = \omega\,(0, \cos\varphi_0, \sin\varphi_0)$ in $L$ components, with $\omega = 7.292115\times10^{-5}\,\mathrm{rad/s}$. The translational
  equations in $L$ add the Coriolis term $-2\boldsymbol{\Omega} \times \mathbf{v}$. The centrifugal term is already inside normal
  gravity and must not be added again ([Gravity](gravity.md)).
- $L$ is a tangent plane, so $z_L$ is not altitude. A point at $z_L = 0$ at horizontal distance
  $d$ from the pad is about $d^2/(2\rho)$ above the ellipsoid, with $\rho$ the Earth's
  radius of curvature at the pad. At 10 km (6.2 mi), that is
  7.8 m (26 ft). Height above the ellipsoid comes from `LaunchFrame::geodetic_from_enu`. The
  flight engine detects apogee and ground contact with that height, not with $z_L$
  ([Rigid-body flight](flight.md), [Loft's lesson on apogee height](../decisions-and-roadmap.md#l35)).

### Body frame `B`

- The origin is the nose tip, on the axis ([the design-tree decision][adr-007]). Positions in
  mass properties are measured from it.
- $z_B$ lies along the axis of symmetry, positive toward the nose. $x_B$ is the design's
  zero radial direction, the angle from which fins, rail buttons and lugs are placed. $y_B = z_B \times x_B$.
- The thrust of a motor aligned with the axis acts along $+z_B$.
- Design stations $s$ are measured aft from the nose tip, as design files state them. They map to
  $z_B = z_{\mathrm{ref}} - s$ with $z_{\mathrm{ref}} = 0$, so $z_B = -s$ and the whole rocket lies at $z_B \le 0$
  ([Design tree](design.md)).

## Angles and attitude

The rocket's angles to the airflow, its attitude, and the launch angles used for input and output.

### Aerodynamic angles

- The angle of attack $\alpha$ is the total angle between $+z_B$ and the rocket's velocity relative to the
  air, in $[0, \pi]$.
- The flow roll $\varphi$ is the direction in which the air crosses the body, measured in the $x_B$–$y_B$
  plane from $x_B$ toward $y_B$. Fin $k$ of a set sits at $\Lambda_k = \theta_0 + 2\pi k/N - \varphi$ from the lateral
  airflow, with $\theta_0$ the set's base angle.
- There is no separate sideslip angle: $\alpha$ says how far the air crosses the body and $\varphi$
  which way.
- With $\hat{\mathbf{w}} = (\cos\varphi, \sin\varphi, 0)$ the lateral air direction in $B$, the normal
  and side forces are

  $$
  q A_{\mathrm{ref}} \left( C_N \hat{\mathbf{w}} + C_Y\, (\hat{\mathbf{z}}_B \times \hat{\mathbf{w}}) \right):
  $$

  $C_N$ is positive along $\hat{\mathbf{w}}$, the way the
  crossing air pushes the body, and $C_Y$ is across the flow's plane ([Aerodynamics](aero.md)). The
  axial force is $-q A_{\mathrm{ref}} C_A \hat{\mathbf{z}}_B$. $C_A$ is positive when the flow meets the nose and drag
  pushes toward the tail. It is negative past $\alpha = 90°$, when the rocket moves tail first.
- The rolling moment is $q A_{\mathrm{ref}}\, d\, C_l$ about $+z_B$, right-handed, with $d$ the reference diameter. A
  positive one turns $x_B$ toward $y_B$. A fin set's positive cant turns the leading edge of fin 0
  (the fin along $+x_B$) toward $-y_B$. So it turns the rocket about $-z_B$, clockwise seen from
  ahead of the nose, and the roll rate $p = \omega_z$ settles negative
  ([Roll: forcing and damping](aero.md#roll-forcing-and-damping)).

### Attitude

The attitude is a unit Hamilton quaternion $q$ (glam `DQuat`, stored `x, y, z, w`) that maps
body components to launch-frame components (in glam, `q.mul_vec3(v_B)`):

$$
\mathbf{v}_L = q \otimes \mathbf{v}_B \otimes q^*
$$

- $q$ and $-q$ are the same attitude.
- The body angular velocity $\boldsymbol{\omega}_B$ is the rate of $B$ relative to $L$, in $B$ components.
- The kinematics are $\dot{q} = \tfrac12\, q \otimes (0, \boldsymbol{\omega}_B)$ (Solà 2017, eq. 200). The integrator renormalizes $q$
  after every step (`attitude::renormalize`).
- $L$ itself turns at $\boldsymbol{\Omega}$ (above), so $\boldsymbol{\omega}_B$ differs from the inertial rate by $R(q)^{\mathsf{T}} \boldsymbol{\Omega}$, at most
  7.3e-5 rad/s. The flight engine's rotational equations leave that term out
  ([the rigid-body flight decision][adr-011]).

### Launch angles

Launch-rail-style angles describe the attitude for input and output (`frames::LaunchAngles`):

- the azimuth $A$, the heading of the body axis, clockwise from true north ($\pi/2$ is east),
  in $[0, 2\pi)$;
- the elevation $E$, the angle of the body axis above the horizon ($\pi/2$ is vertical), in
  $[-\pi/2, \pi/2]$;
- the roll $\varphi$, the turn about $z_B$, in $(-\pi, \pi]$.

$$
\begin{aligned}
q &= R_z(-A) \otimes R_x(E - \pi/2) \\
  &\quad \otimes R_z(\varphi), \\
z_B \text{ in } L &= \begin{pmatrix} \sin A \cos E \\ \cos A \cos E \\ \sin E \end{pmatrix}
\end{aligned}
$$

- At zero roll, $x_B$ is horizontal, to the right of the heading: $(\cos A, -\sin A, 0)$ in $L$.
  $y_B$ is

  $$
  \begin{pmatrix} \sin E \sin A \\ \sin E \cos A \\ -\cos E \end{pmatrix}
  $$

- Vertical is a singularity: at $E = \pm\pi/2$, $A$ and $\varphi$ turn about the same axis.
  `LaunchAngles::from_quaternion` then reports $A = 0$ and puts the whole turn into $\varphi$. This is
  for reporting only: the state is always the quaternion.
- These angles are RocketPy's 3-1-3 convention:
  - $A$ is the heading and $E$ the inclination.
  - $\varphi$ is the rail-button angular position for a `tail_to_nose` rocket, and $2\pi$ minus it for a
    `nose_to_tail` rocket.
  - RocketPy 1.13.0 sets precession $\psi = -\mathrm{heading}$, nutation $\theta = \mathrm{inclination} - 90°$ and spin $\varphi$,
    then builds $q = q_z(\psi)\, q_x(\theta)\, q_z(\varphi)$ (`rocketpy/simulation/flight.py:1557-1579`,
    `rocketpy/tools.py` `euler313_to_quaternions`).
  - `validation/oracles/rocketpy/attitude.py` builds real RocketPy flights for 8 rail setups, with
    both orientations, and records the initial `e0…e3`.
  - `frames::tests::launch_angles_match_the_rocketpy_oracle` checks that $q$ (w, x, y, z) is the
    same attitude to 1e-12 rad.
  - RocketPy's body $+z$ also points toward the nose.

## Tests that pin this

- In `hpr_core::geodesy::tests`:
  - geodetic → ECEF → geodetic, from −10 km (−6.2 mi) to 1000 km (621 mi);
  - ECEF → geodetic → ECEF, from 6250 km (3,884 mi) to 46,000 km (28,583 mi) from the center;
  - axis points, the sphere limit, and the ENU rotation being proper with $\hat{\mathbf{u}}$ normal to the
    ellipsoid.
- In `hpr_core::frames::tests`:
  - angles → quaternion → angles (off vertical), and quaternion → angles → quaternion (everywhere,
    exactly vertical included);
  - ENU ↔ ECEF ↔ geodetic round trips for sites anywhere, within 2000 km (1,243 mi) and up to 1000 km (621 mi) high;
  - named attitudes, and the tangent-plane rise $d^2/(2\rho)$ due east, $\rho$ there being the
    radius [Geodesy](geodesy.md) calls $N$, plus the site's height.
- In `hpr_core::attitude::tests`: the kinematic equation checked against a closed-form coning
  motion, and norm drift ≤ 1e-12 over 1e6 RK4 steps with attitude error < 1e-9 rad.

[adr-003]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0003-frames-attitude-geodesy-and-the-gravity-model.md
[adr-007]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0007-design-tree-stations-placement-automatic-radii.md
[adr-011]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0011-rigid-body-flight-equations-of-motion.md
