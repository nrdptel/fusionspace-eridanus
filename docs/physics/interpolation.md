# Interpolation tables

Learn how HPR Sim reads values between a table's points, along straight lines or a spline, and what
each table does past its ends.

## In short

- **What it models:** reading values between a table's points, such as drag coefficient against
  Mach number, along straight lines or a smooth natural cubic spline. Each table sets what
  happens past its ends, and every lookup says if it went there.
- **Sources:** the slope form of the cubic spline in C. de Boor, *A Practical Guide to Splines*
  (Springer, 2001).
- **How well it is validated:** analytic and unit tests only. The spline through (0, 0), (1, 1)
  and (2, 0) matches its closed form, $y = 3x/2 - x^3/2$ on $[0, 1]$, and property tests check
  that both kinds hit every point. Not compared with another simulator or a flight.
- **What it leaves out:** tables of more than one input. A natural spline can overshoot where data
  turn sharply (a thrust spike, a transonic drag peak), and there is no overshoot-free (monotone)
  cubic yet, so use linear tables there.

## Code and sources

Code: `hpr_core::interp` (`Table1D`). Today it carries the drag tables that override HPR Sim's own
drag, $C_D(M)$ ([Aerodynamics](aero.md)); thrust curves and soundings interpolate on their own
([Solid motors](motor.md), [Atmosphere](atmosphere.md)). Every table states how it interpolates and
what happens outside its range, and every lookup reports whether it extrapolated.

## Knots

A table has $n \ge 2$ knots $(x_i, y_i)$. The $x_i$ strictly increase, and every value and every
secant slope $\delta_i = (y_{i+1} - y_i)/h_i$, with $h_i = x_{i+1} - x_i$, is finite. Construction and
deserialization both enforce this.

## Interpolation

Inside $[x_0, x_{n-1}]$, with $t = (x - x_i)/h_i$ on interval $i$:

- **Linear:** $y = (1 - t)\, y_i + t\, y_{i+1}$. This form returns the knot values exactly.
- **Natural cubic spline:** the cubic Hermite form with knot slopes $s_i$:

  $$
  \begin{aligned}
  y &= h_{00}(t)\, y_i + h_{10}(t)\, h_i s_i \\
    &\quad + h_{01}(t)\, y_{i+1} + h_{11}(t)\, h_i s_{i+1}, \\
  h_{00} &= (1 + 2t)(1 - t)^2, \\
  h_{10} &= t(1 - t)^2, \\
  h_{01} &= t^2(3 - 2t), \\
  h_{11} &= -t^2(1 - t)
  \end{aligned}
  $$

  The second derivative at the ends of interval $i$ is

  $$
  \begin{aligned}
  y''(x_i^+) &= \frac{6\delta_i - 4s_i - 2s_{i+1}}{h_i}, \\
  y''(x_{i+1}^-) &= \frac{-6\delta_i + 2s_i + 4s_{i+1}}{h_i}
  \end{aligned}
  $$

  Equating the two at each interior knot and multiplying by $h_{i-1} h_i/2$ gives

  $$
  \begin{aligned}
  &h_i s_{i-1} + 2(h_{i-1} + h_i)\, s_i + h_{i-1} s_{i+1} \\
  &\quad = 3(h_i \delta_{i-1} + h_{i-1} \delta_i)
  \end{aligned}
  $$

  Setting $y''$ to zero at both ends (the "natural" or free-end condition) closes the system with
  $2s_0 + s_1 = 3\delta_0$ and $s_{n-2} + 2s_{n-1} = 3\delta_{n-2}$. This is the slope form of cubic
  spline interpolation (C. de Boor, *A Practical Guide to Splines*, rev. ed., Springer, 2001,
  ch. IV). Every row is strictly diagonally dominant, so the Thomas algorithm needs no pivoting.
  With two knots the spline is the straight line.

A natural spline can overshoot where the data turn sharply (a thrust spike, a transonic drag
peak). Use linear tables for such data; a monotone cubic can be added when a model needs one.

## Extrapolation

| policy | below $x_0$ | above $x_{n-1}$ |
|---|---|---|
| `clamp` (default) | $y_0$ | $y_{n-1}$ |
| `linear` | $y_0 + m_0 (x - x_0)$ | $y_{n-1} + m_{n-1} (x - x_{n-1})$ |
| `error` | `CoreError::OutOfRange` | `CoreError::OutOfRange` |

$m$ is the end interval's secant for a linear table and the spline's end slope $s_0$ or $s_{n-1}$
for a cubic one, so linear extrapolation of a natural spline stays twice differentiable. A lookup
at NaN is an error under every policy.

## Tests that pin this (`crates/hpr-core/src/interp.rs`)

- `natural_spline_matches_the_three_knot_closed_form`: knots $(0,0)$, $(1,1)$, $(2,0)$ give slopes
  $(3/2, 0, -3/2)$ and $y = 3x/2 - x^3/2$ on $[0, 1]$, symmetric about $x = 1$.
- Property tests: both kinds reproduce the knots exactly; linear values stay within each
  interval's end values; the spline's second derivative is continuous at interior knots and zero
  at the ends; the spline reproduces straight lines, extrapolation included; each policy behaves
  as in the table above; a JSON round trip rebuilds an identical table.
- `rejects_malformed_knots`, `deserializing_checks_the_knots`: malformed input fails.
