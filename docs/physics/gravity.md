# Gravity and Earth rotation

This page covers the gravity a rocket feels, which changes with latitude and height, and the
Coriolis push the spinning Earth gives it.

## In short

- **What it models:** the pull a rocket feels. It is WGS 84 normal gravity, the gravity of a
  smooth, spinning model Earth, which changes with latitude and height. It also models the
  Coriolis effect, the sideways push on anything that moves over the spinning Earth.
- **Sources:** the NGA's WGS 84 standard, NGA.STND.0036 (2014), chapter 4 and appendix B.
- **How well it is validated:** the derived constants reproduce the standard's printed values to
  their last digit. At 11 points, both poles and heights up to 200 km (124 mi) included, gravity's
  strength matches the published formulas within 2e-14 relative. A reference script evaluates the
  formulas in 40-digit arithmetic. HPR Sim's copy of RocketPy's formula matches RocketPy at 8 points, to under 1e-12
  relative. No real-flight check.
- **What it leaves out:** the real Earth's gravity anomalies, typically within ±1e-4 relative.
  RocketPy's formula, an option for like-for-like comparisons, differs from the exact field by
  1.4e-5 relative at 100 km (62 mi).

## Code and sources

Code: `hpr_core::gravity` (the normal gravity field) and `hpr_core::earth` (what the flight engine
uses). Source: **[NGA]** NGA.STND.0036_1.0.0_WGS84 (2014), chapter 4 and appendix B, pinned as
`wgs84-nga-stnd-0036`.

## Normal gravity

What normal gravity is, the WGS 84 parameters that define it, and the formulas that compute it.

### What normal gravity is

Normal gravity $\gamma$ is the gravity of the level ellipsoid: the attraction of the WGS 84 ellipsoid
plus the centrifugal acceleration of the Earth's rotation. It is what a body at rest on the
rotating Earth feels.

- In the Earth-fixed launch frame, the only extra inertial term is Coriolis, $-2\boldsymbol{\Omega} \times \mathbf{v}$. Adding a
  centrifugal term as well would count it twice.
- Normal gravity ignores the real Earth's gravity anomalies, typically within ±1e-4 relative
  (±100 mGal). A geopotential model such as EGM2008 is a later option.

### Defining parameters and derived constants

The four defining parameters ([NGA] Table 3.1) are:

- $a = 6378137.0\,\mathrm{m}$
- $1/f = 298.257223563$
- $GM$ is $3.986004418\times10^{14}\,\mathrm{m^3/s^2}$
- $\omega = 7.292115\times10^{-5}\,\mathrm{rad/s}$

Everything else is derived from them ([NGA] appendix B). The lines below are, in order, the
second eccentricity $e'$ and eqs. B-18, B-19, B-20, B-24, B-25 and B-26:

$$
\begin{aligned}
e' &= E/b, \\
q_0 &= \tfrac12 \biggl[ \left(1 + \frac{3}{e'^2}\right) \arctan e' \\
&\qquad - \frac{3}{e'} \biggr], \\
q_0' &= 3 \left(1 + \frac{1}{e'^2}\right) \\
&\quad \times \left(1 - \frac{\arctan e'}{e'}\right) - 1, \\
m &= \frac{\omega^2 a^2 b}{GM}, \\
\gamma_e &= \frac{GM}{ab} \left(1 - m - \frac{m e' q_0'}{6 q_0}\right), \\
\gamma_p &= \frac{GM}{a^2} \left(1 + \frac{m e' q_0'}{3 q_0}\right), \\
k &= \frac{b \gamma_p - a \gamma_e}{a \gamma_e}
\end{aligned}
$$

**Cancellation.** The closed forms for $q$ and $q'$ cancel about five digits at $\varepsilon = E/u \le 0.082$.
A first version computed $k$ with a relative error of 2.6e-11, so it no longer rounded to Table
3.6. Below $\varepsilon = 0.5$ the code therefore uses the series from

$$
\arctan \varepsilon = \sum_{n \ge 0} \frac{(-1)^n \varepsilon^{2n+1}}{2n+1}:
$$

$$
\begin{aligned}
q &= \sum_{n \ge 1} \frac{(-1)^{n+1}\, 2n\, \varepsilon^{2n+1}}{(2n+1)(2n+3)}, \\
q' &= \sum_{n \ge 1} \frac{(-1)^{n+1}\, 6\, \varepsilon^{2n}}{(2n+1)(2n+3)}
\end{aligned}
$$

**Check.** These reproduce the printed values to their last digit:

| quantity | value | printed in |
|---|--:|---|
| $q_0$ | 7.334625787083e-5 | eq. B-18 |
| $q_0'$ | 2.688041300461e-3 | eq. B-19 |
| $\gamma_e$ | 9.7803253359 m/s² | Table 3.6 |
| $\gamma_p$ | 9.8321849379 m/s² | Table 3.6 |
| $k$ | 1.931852652458e-3 | Table 3.6 |
| $m$ | 3.449786506841e-3 | Table 3.6 |

### Formulas

- On the ellipsoid (Somigliana, eq. 4-1):

  $$
  \gamma = \gamma_e \frac{1 + k \sin^2\varphi}{\sqrt{1 - e^2 \sin^2\varphi}}
  $$

- As a Taylor series in height (eq. 4-3):

  $$
  \begin{aligned}
  \frac{\gamma_h}{\gamma} &= 1 - \frac{2}{a} \bigl(1 + f + m \\
  &\qquad - 2f \sin^2\varphi\bigr) h \\
  &\quad + \frac{3}{a^2}\, h^2
  \end{aligned}
  $$

  RocketPy uses this form. The reference values in
  `validation/fixtures/earth/wgs84-normal-gravity.json` give its error against the exact field. It is 1e-8 relative at 1.4 km (0.87 mi) and 3e-7 at
  30 km (19 mi). At 100 km (62 mi) it is 1.4e-5,
  and at 200 km (124 mi) 1.1e-4.
- For the exact field (eqs. 4-5 to 4-13), ellipsoidal-harmonic coordinates $(u, \beta)$ give the components
  $\gamma_u$ and $\gamma_\beta$. The code rotates them into ECEF with $R_1$ (eq. 4-18). Eq. 4-8 is
  used in the equivalent form

  $$
  u^2 = \tfrac12 \left[s + \sqrt{s^2 + 4E^2z^2}\right],
  $$

  with $s = x^2 + y^2 + z^2 - E^2$, which never divides by $s$.
  - $\beta$ comes from `atan2`, so every quadrant works.
  - The field is undefined on the focal disc ($u = 0$, the equatorial plane within 522 km (324 mi) of the
    center). The code returns an error there.
- The local components are in the ENU axes at the point. $-\boldsymbol{\gamma} \cdot \hat{\mathbf{u}}$ is the exact normal component $\gamma_h$
  (eq. 4-16), and $\boldsymbol{\gamma} \cdot \hat{\mathbf{n}}$ is $\gamma_\varphi$ (eq. 4-23, positive north). The length is $\lvert \boldsymbol{\gamma}_{\mathrm{total}} \rvert$ (eq. 4-4).
  - Above the ellipsoid, the vector tilts slightly toward the equator: $\gamma_\varphi < 0$ in the northern
    hemisphere, −8.1e-4 m/s² at 45° N and 100 km (62 mi).
  - That sign is confirmed independently. The reference script differentiates the normal
    potential numerically, and its value on the ellipsoid matches Table 3.6's $U_0$.

## In the flight engine

The gravity models a flight can use, and RocketPy's formula for like-for-like comparisons.

### Gravity models for the flight engine (`earth::GravityModel`)

| model | vector in $L$ | use |
|---|---|---|
| `constant { g_mps2 }` | $(0, 0, -g)$ | analytic tests; comparisons with tools that use 9.80665 |
| `vertical_taylor` | $(0, 0, -\gamma_h)$ from eq. 4-3 at $(\varphi_0, h_0 + z)$ | like-for-like with RocketPy's formula (see its flight quirks below) |
| `vertical` | $(0, 0, -\lvert\gamma\rvert)$ exact at $(\varphi_0, \lambda_0, h_0 + z)$ | along the launch vertical |
| `ellipsoidal` (default) | the full vector at the body's position, rotated into $L$ | everything else |

The ellipsoidal model follows the vertical as it turns downrange: by about $d/(N + h)$ east-west
and $d/(M + h)$ north-south, with $M$ the meridian radius,

$$
M = \frac{a(1 - e^2)}{(1 - e^2 \sin^2\varphi)^{3/2}}.
$$

A test checks both at 20 km (12 mi), to 1e-4 east and 1e-3 north (the curvature changes along a
meridian). Earth rotation (`earth::EarthRotation`) is `coriolis` by default, $-2\boldsymbol{\Omega} \times \mathbf{v}$ with
$\boldsymbol{\Omega} = \omega\,(0, \cos\varphi_0, \sin\varphi_0)$, or `ignore`.

**`STANDARD_GRAVITY_MPS2 = 9.80665`** is the conventional $g_0$ (3rd CGPM, 1901; also used by the
U.S. Standard Atmosphere 1976). It is a unit convention, not a model of local gravity. Loft used it
as gravity, which put Loft 0.3% off RocketPy at the equator
([Loft's lesson on constant gravity](../decisions-and-roadmap.md#l1)).

### RocketPy 1.13.0, for like-for-like cases

Recorded by `validation/oracles/rocketpy/gravity.py` into
`validation/fixtures/earth/rocketpy-gravity.json`, for
[the validation milestone's RocketPy comparison](../decisions-and-roadmap.md#m2-1).

- The formula is Somigliana (4-1) times the Taylor factor (4-3), with Table 3.6 constants
  (`rocketpy/environment/environment.py`, `somigliana_gravity`). It matches
  `NormalGravity::taylor_mps2` to under 1e-12 relative.
- A flight samples the formula at 100 points between 0 and `max_expected_height` (80 km (50 mi) by
  default). It holds the last value above that. At 45° and 100 km (62 mi) a flight uses
  9.563982 m/s², where the formula gives 9.504874.
- RocketPy feeds it height above sea level, not above the ellipsoid.
- With no latitude given, the default of 0 gives equatorial gravity.
- Coriolis is included as $-2\boldsymbol{\Omega} \times \mathbf{v}$ with $\Omega = 2\pi/(86164.1\,\mathrm{s})$, in the 6-DOF and parachute
  equations but not in the rail phase.

## Tests that pin this

- `gravity::tests::somigliana_matches_published_values`
  ([Loft's lesson on constant gravity](../decisions-and-roadmap.md#l1), and the *done when* of
  [the core math, frames and Earth milestone](../decisions-and-roadmap.md#m1-1)):
  - Table 3.6 constants to their printed digits.
  - At 11 latitude/longitude/height points, including the equator, both poles, launch sites and
    heights up to 200 km (124 mi):
    - surface (4-1), Taylor (4-3), $\lvert\gamma\rvert$ (4-4) and $\gamma_h$ (4-16) within 1e-6 relative, and in
      fact within 2e-14 relative;
    - $\gamma_\varphi$ and the ECEF vector within 1e-12 m/s².
  - The reference values come from `validation/oracles/wgs84/normal_gravity.py`. It evaluates
    the published formulas in 40-digit arithmetic and checks the constants against the tables. It
    also checks the vector against the gradient of the normal potential.
- `taylor_series_matches_the_rocketpy_oracle`: 8 points against RocketPy's formula.
- `exact_field_reduces_to_somigliana_on_the_ellipsoid`: 37 latitudes, zero horizontal
  component.
- `q_functions_match_appendix_b`: the printed $q_0$ and $q_0'$, and continuity where the series
  hands over to the closed form.
- `earth::tests`: the models agree at the pad; the ellipsoidal model follows the vertical
  downrange; the Coriolis direction and magnitude.

