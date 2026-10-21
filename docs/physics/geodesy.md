# Geodesy: the ellipsoid, coordinates, distance and bearing

HPR Sim models the Earth as a flattened ellipsoid, and this page shows how it converts coordinates
and measures distance and bearing between places.

## In short

- **What it models:** the Earth's shape, as the WGS 84 ellipsoid (a sphere slightly flattened at
  the poles). It converts between latitude, longitude and height and Earth-centered x, y, z. It
  gives the local east, north and up directions, and the distance and bearing between two places.
- **Sources:** the NGA's WGS 84 standard, NGA.STND.0036 (2014). C. F. F. Karney, *Geodesics on
  an ellipsoid of revolution* (2011), appendix B. Karney, *Algorithms for geodesics* (2013),
  through the `geographiclib-rs` crate.
- **How well it is validated:** for the conversions, the derived ellipsoid values reproduce the
  standard's Table 3.5 to its printed digits. Unit tests make random round trips from −10 km (−6.2 mi)
  to +1000 km (621 mi). They return latitude within 1e-14 rad and height within 2e-8 m. The
  conversions are not compared with another library, a simulator or a real flight. For distance
  and bearing, all 500,000 lines of Karney's published test set match within 15 nanometers (nm,
  billionths of a meter). Distance is within 11.18 nm, the far point within 14.02 nm, and the
  bearings within 15 nm of sideways miss ([below](#distance-and-bearing-geodesics)). CI checks
  every 500th line and the 21 mirror lines. The whole set is checked where it has been
  downloaded, and has been measured on macOS.
- **What it leaves out:** height above sea level, which needs the geoid, up to about 100 m (328 ft) from
  the ellipsoid ([Frames](frames.md#earth-centered-earth-fixed-ecef)). HPR Sim has no geoid model.
  A flight takes that difference at the site as an input. Nothing in a flight uses distance and
  bearing yet. The landing distance `hpr` prints is measured on a flat map from the pad's east
  and north offsets, and there is no command for geodesics.

## Sources

Code: `hpr_core::geodesy` and `hpr_core::geodesic`. Conventions: [Frames](frames.md).

- **[NGA]** NGA.STND.0036_1.0.0_WGS84, *Department of Defense World Geodetic System 1984, Its
  Definition and Relationships with Local Geodetic Systems*, 2014-07-08. Pinned as
  `wgs84-nga-stnd-0036`. US government work.
- **[Karney]** C. F. F. Karney, *Geodesics on an ellipsoid of revolution*, arXiv:1102.1215v1,
  2011, appendix B. Pinned as `karney-2011-geodesics`.
- **[Karney2013]** C. F. F. Karney, *Algorithms for geodesics*, J. Geodesy 87 (2013) 43–55,
  arXiv:1109.4448v2. Pinned as `karney-2013-algorithms-for-geodesics`.
- **[GeodTest]** C. F. F. Karney, *Test set for geodesics*, doi:10.5281/zenodo.32156, CC0.
  Pinned as `karney-geodtest`.

## Ellipsoid and coordinates

The WGS 84 ellipsoid, conversions between geodetic and Earth-centered coordinates, and the local
axes.

### WGS 84 ellipsoid

- The defining parameters ([NGA] Table 3.1) are $a = 6378137.0\,\mathrm{m}$ and $1/f = 298.257223563$.
- The derived values are $b = a(1 - f)$, $e^2 = f(2 - f)$ and $E = a e$.
- These reproduce [NGA] Table 3.5 to its printed digits
  (`wgs84_derived_geometry_matches_table_3_5`):

  | quantity | value |
  |---|--:|
  | $b$ | 6356752.3142 m (20,855,486.595 ft) |
  | $e^2$ | 6.694379990141e-3 |
  | $E$ | 5.2185400842339e5 m |

### Geodetic to ECEF ([NGA] eqs. 4-14, 4-15)

$$
\begin{aligned}
N &= \frac{a}{\sqrt{1 - e^2 \sin^2\varphi}}, \\
X &= (N + h) \cos\varphi \cos\lambda, \\
Y &= (N + h) \cos\varphi \sin\lambda, \\
Z &= \left(\frac{b^2}{a^2} N + h\right) \sin\varphi
\end{aligned}
$$

The code writes $b^2/a^2$ as $1 - e^2$.

### ECEF to geodetic ([Karney] appendix B)

This is Vermeille's closed form, which Karney extended to cover points near the Earth's center.

**Setup.** Let $R = \sqrt{X^2 + Y^2}$, $x = R/a$ and $y = \sqrt{1 - e^2}\, Z/a$. The geodetic latitude
follows from the largest real root $\kappa$ of (B1):

$$
\begin{aligned}
&\kappa^4 + 2e^2\kappa^3 - (x^2 + y^2 - e^4)\,\kappa^2 \\
&\quad - 2e^2 y^2 \kappa - e^4 y^2 = 0
\end{aligned}
$$

**Solving for $u$.**

$$
\begin{aligned}
r &= \frac{x^2 + y^2 - e^4}{6}, \\
S &= \frac{e^4 x^2 y^2}{4}, \\
d &= S\,(S + 2r^3)
\end{aligned}
$$

If $d \ge 0$, with $T$ the real cube root and $\sqrt{d}$ given the sign of $S + r^3$
($u = 0$ if $T = 0$):

$$
\begin{aligned}
T &= \left(S + r^3 \pm \sqrt{d}\right)^{1/3}, \\
u &= r + T + \frac{r^2}{T}
\end{aligned}
$$

If $d < 0$:

$$
\begin{aligned}
\psi &= \operatorname{ph}\left(-S - r^3 + i\sqrt{-d}\right), \\
u &= r\left(1 + 2\cos\frac{\psi}{3}\right)
\end{aligned}
$$

**Solving for $\kappa$.** The last line is (B5). When $u < 0$, the code takes
$v + u = e^4y^2/(v - u)$, which avoids cancellation.

$$
\begin{aligned}
v &= \sqrt{u^2 + e^4 y^2}, \\
w &= \frac{\left((v + u) - y^2\right) e^2}{2v}, \\
\kappa &= \frac{v + u}{\sqrt{(v + u) + w^2} + w}
\end{aligned}
$$

**Latitude, height and longitude.** The first line is (B2); the next two are (B3).

$$
\begin{aligned}
\varphi &= \operatorname{ph}\left(\frac{R}{\kappa + e^2} + \frac{iZ}{\kappa}\right), \\
h &= \left(1 - \frac{1 - e^2}{\kappa}\right) \sqrt{D^2 + Z^2}, \\
D &= \frac{\kappa R}{\kappa + e^2}, \\
\lambda &= \operatorname{ph}(X + iY)
\end{aligned}
$$

$\operatorname{ph}$ is the argument, computed with `atan2`; $\lambda = 0$ on the axis.

The closed form fails only in the equatorial plane within $a e^2$ (42.7 km (27 mi)) of the center. There
the paper needs its limiting forms (B6)–(B7). The code returns `CoreError::Domain` instead.

**Measured accuracy** (property tests, 256 cases each per run):

- geodetic → ECEF → geodetic: latitude within 1e-14 rad and height within 2e-8 m, from
  −10 km (−6.2 mi) to +1000 km (621 mi);
- ECEF → geodetic → ECEF: within 1e-7 m per 6400 km (3,977 mi) of radius, out to 46,000 km (28,583 mi).

### Local ENU axes

`ecef_from_enu_rotation(φ, λ)` has columns $\hat{\mathbf{e}}$, $\hat{\mathbf{n}}$ and $\hat{\mathbf{u}}$ ([Frames](frames.md)). A test checks that $\hat{\mathbf{u}}$
equals the normalized gradient of $x^2/a^2 + y^2/a^2 + z^2/b^2$ at the foot point, i.e. the ellipsoid
normal.

## Distance and bearing: geodesics

How far is the landing from the pad, and in which direction? On a flat map that is Pythagoras.
On the Earth it is a **[geodesic](../glossary.md#geodesic)**, the shortest path over the
ellipsoid's surface. Its length is the distance, and its direction where it leaves is the
[bearing](../glossary.md#bearing).

Geodesists call a bearing an *azimuth*. Here the two mean the
same. `hpr_core::geodesic` solves the two classic problems
([API reference](https://hpr.fusionspace.co/api/hpr_core/geodesic/index.html)):

- The inverse (`Ellipsoid::geodesic_inverse`): from two places, the distance $s_{12}$ and the
  azimuths $\alpha_1$ (leaving the first) and $\alpha_2$ (arriving at the second).
- The direct (`Ellipsoid::geodesic_direct`): from a place, a bearing $\alpha_1$ and a distance, the
  latitude and longitude you reach and the azimuth $\alpha_2$ there.

Azimuths are clockwise from true north, in radians from $-\pi$ to $\pi$, so due west is $-\pi/2$. For the 0°
to 360° a compass uses, take `.to_degrees().rem_euclid(360.0)`. $\alpha_2$ is the direction of travel
on arrival, so the bearing back to the start is $\alpha_2 \pm \pi$. Over a long path it differs from
$\alpha_1 \pm \pi$ because meridians converge. Heights are ignored: the path runs on the ellipsoid's
surface, and the direct problem's end has no height until you give it one.

**Method.** The code is Karney's GeographicLib, as georust's `geographiclib-rs` 0.2.7 (MIT) ports
it. The simulator uses the crate rather than a port of its own, so the code is Karney's line for
line ([the geodesics decision record][adr-127]). [Karney2013] maps the ellipsoid onto an auxiliary sphere,
where a geodesic is a great circle (the sphere's shortest path). [Karney2013] corrects distance and
longitude with series in the flattening to sixth order (§2). The inverse finds $\alpha_1$ by Newton's
method (§4), from a starting guess (§5).

Karney states that round-off stays under 15 nm in both problems on WGS 84 (§7, page 10). He adds
that up to a flattening of 1/150 the series' truncation is smaller still (page 9). Past 1/150 the
series lose accuracy, so the simulator refuses such an ellipsoid; WGS 84's flattening is 1/298.
The familiar haversine formula treats the Earth as a sphere, which the Earth is not. Vincenty's ellipsoidal method is less accurate than Karney's, and
its inverse sometimes fails to converge (§7).

**Worked example.** A pad at 32.9904° N, 106.9750° W and a landing at 33.0000° N, 106.9680° W:

| quantity | value |
|---|--:|
| distance $s_{12}$ | 1,249.614 m (4,099.78 ft) |
| bearing from the pad $\alpha_1$ | 31.567° |
| azimuth on arrival $\alpha_2$ | 31.571° |

The other way round, 2 km (1.2 mi) from the same pad on a bearing of 60° reaches 32.999415° N,
106.956466° W, heading 60.010°. At this range a flat map gives the same distance if it uses
the ellipsoid's curvature at the middle latitude. That gives 1,249.614 m (4,099.78 ft) again, to
under a millimeter. The geodesic matters over long paths, where a flat map's error grows. A unit test,
`the_guides_worked_example`, holds these numbers.

**Validation.** [GeodTest] is Karney's set of 500,000 WGS 84 geodesics. They were worked
separately from the code, with the series carried to thirtieth order and high-precision
arithmetic. Each end is known to 1e-18°, so they check this code's method and its rounding.

They come in nine kinds of 50,000 or 100,000 lines each, listed in the
[geodesics report](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/geodesics.md).
The kinds are random pairs, nearly antipodal pairs (almost opposite sides of the Earth), short
paths and paths near the poles. The rest run nearly along a meridian or the equator, or near a
*vertex*, the point where a geodesic runs due east or west. `crates/hpr-core/tests/geodtest.rs`
solves every line both ways and measures five errors, each held to Karney's 15 nm on every line:

| error | what it is | largest |
|---|---|---|
| inverse distance | the computed $s_{12}$ against the set's | 11.18 nm |
| inverse landing | solve the direct problem with the inverse's own $\alpha_1$ and $s_{12}$, and measure how far it lands from the second place | 11.26 nm |
| inverse azimuths × $m_{12}$ | each azimuth's error times the *reduced length* $m_{12}$, how far the far end moves sideways per radian the start's bearing turns: the sideways miss the error stands for | 8.49 nm |
| direct end point | the computed end against the set's, in Earth-centered coordinates | 14.02 nm |
| direct heading × $a$ | the angle between the computed and the set's direction of travel at the end, as directions in space, times the Earth's radius $a$ | 13.99 nm |

The inverse's azimuths can't be checked on the 50,000 "between vertices" lines. There $m_{12}$ is
at most 1e-13 m, so any azimuth error reads as no miss. Their distance and landing are checked.

**Mirror lines: two paths of the same length.** Suppose the second place's latitude is exactly
the first's negated ($\varphi_2 = -\varphi_1$) and the two azimuths differ ($\alpha_1 \ne \alpha_2$). Then two geodesics of the
same length join the places, one the mirror of the other. The second geodesic has $\alpha_1$ and $\alpha_2$ swapped
(GeographicLib's `GeodSolve` manual, *Multiple solutions*). Either answer is right. Where
$\alpha_1 = \alpha_2$, as on the between-vertices lines, the geodesic is unique.

The set has 21 mirror lines once its numbers are read as `f64` (the 64-bit floating-point numbers
the simulator computes in). They are all nearly antipodal, with $m_{12}$ under a centimeter, so their azimuths are nearly
undetermined. On these lines the test scores the simulator's answer against whichever pair it is
nearer to. That error is within 15 nm on all 21. On 4 the nearer pair is the swapped one.

CI checks every 500th line (1,000 of them) and the 21 mirror lines, which are committed. The
whole set is checked where `cargo xtask refs fetch` has downloaded it, against the committed
report. The whole set has been measured only on macOS, by the debug build that wrote the report's
table. A release build there moves three cells by up to 1.83 nm. On any other build the test
holds the 15 nm bound without comparing the table.

**What it leaves out.** Heights: two places at 3,000 m (9,843 ft) are as far apart as the same places at
sea level. Only WGS 84 is measured. On any other ellipsoid up to a flattening of 1/150, the
accuracy is Karney's claim, not something this project has measured. A distance of many trips
round the Earth carries its own rounding: one step of `f64` in the distance. That is at least
15 nm past 67,109 km (41,700 mi). Nothing in a flight uses geodesics yet, and `hpr` has no command for them.

[Karney2013]: https://arxiv.org/abs/1109.4448
[GeodTest]: https://doi.org/10.5281/zenodo.32156
[adr-127]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0127-m5-3c1-geodesics-through-geographiclib-held-to.md
