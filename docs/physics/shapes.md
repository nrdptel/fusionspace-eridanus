# Nose cones, transitions and solids of revolution

See the nose cone and transition shapes HPR Sim supports, and how it finds each one's volume, center
of mass, inertia and surface area.

## In short

- **What it models:** the outer shape of nose cones and transitions (conical, ogive, elliptical,
  power, parabolic and Haack series), and the volume, center of mass, inertia and surface areas
  of each, solid or as a shell of given wall thickness.
- **Sources:** G. A. Crowell Sr., *The Descriptive Geometry of Nose Cones* (1996), and appendix A
  of the published *OpenRocket technical documentation* v13.05 (2013).
- **How well it is validated:** by analytic tests only, the first of four
  [kinds of evidence][levels]. Filled shapes match closed forms to 1e-10 (relative), and
  independent high-precision integrals to 1e-12 on 22 noses and transitions; 20 walls match to
  1e-10. Not compared with OpenRocket, weighed parts or a real flight.
- **What it leaves out:** OpenRocket's documentation doesn't say how it measures wall thickness.
  Measuring it radially instead of square to the surface changes wall volume by 1.4% on a cone
  three calibers (base diameters) long. Where a steep end is cut square to the axis, rather than
  following the wall's inner corner as HPR Sim does, the part gains up to 2.24% of wall mass in this
  page's examples. The OpenRocket comparison ([M2.2](../decisions-and-roadmap.md#m2-2)) is to check both.

## Code and sources

Code: `hpr_design::shapes` (profiles) and `hpr_design::solids` (volume, centroid, moments, areas).

Sources:

- **[CR]** G. A. Crowell Sr., *The Descriptive Geometry of Nose Cones* (1996), pp. 1–6 and 12–14.
  Cited, not pinned: the only copy found is on a plain-http university mirror, and the reference
  library ([ADR-002][adr-002]) pins over https. [TD] gives the same curves.
- **[TD]** S. Niskanen, *OpenRocket technical documentation* v13.05 (2013), appendix A, pp. 102–106,
  pinned as `openrocket-techdoc-13.05`. This is the published document, not the program's source.

## Profiles

A profile gives the outer radius $r(x)$ at distance $x$ aft of its forward end, $0 \le x \le L$. Each
shape is a normalized curve $g(\xi)$ with $g(0) = 0$ at the tip and $g(1) = 1$ at the base:

| shape | $g(\xi)$ or $y(x)$ | parameter | source |
|---|---|---|---|
| conical | $\xi$ | none | [CR] p. 1; [TD] A.1 |
| ogive | $y(x)$, below | $\rho/\rho_t \ge R/L$ | [CR] p. 4 |
| elliptical | $\sqrt{1 - (1 - \xi)^2}$ | none | [TD] A.6 |
| power series | $\xi^n$ | $0 < n \le 1$ | [CR] p. 2; [TD] A.8 |
| parabolic series | $(2\xi - K'\xi^2)/(2 - K')$ | $0 \le K' \le 1$ | [CR] p. 5; [TD] A.7 |
| Haack series | $g(\theta)$, below | $0 \le C \le 2/3$ | [CR] p. 6; [TD] A.9–A.10 |

The ogive's height and the Haack series are:

$$
\begin{aligned}
y &= \sqrt{\rho^2 - (x - \rho\cos\alpha)^2} \\
&\quad + \rho\sin\alpha, \\
\alpha &= \arctan\frac{R}{L} - \arccos\frac{\sqrt{L^2 + R^2}}{2\rho}, \\
g &= \sqrt{\frac{\theta - (\sin 2\theta)/2 + C\sin^3\theta}{\pi}}, \\
\theta &= \arccos(1 - 2\xi)
\end{aligned}
$$

- **Ogive.** The shape is Crowell's secant ogive: an arc of radius $\rho$ through the tip and the base
  rim, with $\rho$ given as a multiple of the tangent radius $\rho_t = (R^2 + L^2)/(2R)$.
  - 1 is the tangent ogive, whose slope is zero at the base.
  - Values above 1 meet the base at an angle.
  - Values below 1 bulge past $R$ before the base.
  - The arc reaches the tip only while its center is not above the axis, which needs
    $\rho \ge (L^2 + R^2)/(2L)$.

  The center is computed on the chord's perpendicular bisector and the height as

  $$
  y = \frac{x\,(2x_c - x)}{\sqrt{\rho^2 - (x - x_c)^2} - y_c},
  $$

  which avoids the cancellation near the tip of a slender ogive.
- **OpenRocket's ogive parameter is not adopted.** [TD] contradicts itself about it:
  - A.3 defines it as $\kappa = \rho_t/\rho$.
  - A.4–A.5 describe the first $L$ of a tangent ogive of length $L/\kappa$, which at $L = 4$, $R = 1$,
    $\kappa = \tfrac12$ has $\rho = 24.8$, not $\rho_t/\kappa = 17.0$.
  - The planned `.ork` importer ([M3.1](../decisions-and-roadmap.md#m3-1)) must settle the mapping by running the
    OpenRocket jar.
- **Haack.** Monotone for $C \le 2/3$, because

  $$
  \frac{\mathrm{d}(g^2)}{\mathrm{d}\theta} = \frac{\sin^2\theta\,(2 + 3C\cos\theta)}{\pi}.
  $$

  $C = 0$ is the von Kármán (LD-Haack) ogive and $C = 1/3$ the LV-Haack. [TD] limits $C$ to $1/3$
  in the program.
- **Two shapes without drag data.** A Haack nose or widening transition above $C = 1/3$, and a
  bulged secant ogive (arc radius below the tangent ogive's), have no drag data faster than
  sound, so the simulator's drag buildup refuses them since
  [M1.8b1](../decisions-and-roadmap.md#m1-8b1). Their shapes, mass properties and center of pressure
  are unaffected, and a drag table stands in for the drag
  ([Aerodynamics](aero.md#drag-through-mach-1)).
- **Blunt tips.** The elliptical, power-series ($n < 1$) and Haack slopes are infinite at the tip.
  $n = 0$ (a flat cylinder) is rejected; model it as a tube and a bulkhead.
- **Errors in [CR].** Its prose says "greater than twice the length" for the bulged secant ogive,
  against its own formula. Its ogive and ellipsoid areas are wrong, and the author marked them so.
  Its elliptical formula measures $x$ from the base, and its ellipse CP ratio should read $2L/3$.
  HPR Sim uses none of those.

### Transitions

A transition runs from fore radius $R_f$ to aft radius $R_a$ over $L$.

- **Orientation.** The shape's tip lies at the **smaller** end. Growing aft,
  $r = R_f + (R_a - R_f)\,g(x/L)$. A boattail is the mirror image,

  $$
  \begin{aligned}
  r &= R_a \\
  &\quad + (R_f - R_a) g(1 - x/L).
  \end{aligned}
  $$

  [TD] doesn't say how a shrinking transition is oriented; this choice keeps both ends' radii
  exact and the profile monotone ([Loft lesson L49](../decisions-and-roadmap.md#l49)).
- **Clipped** ([TD] §A.7): cut a whole nose cone of base radius $\max(R_f, R_a)$ where its radius
  is $\min(R_f, R_a)$, with the nose length chosen so the piece is $L$ long.
  - Shapes other than the ogive invert $g$ by bisection.
  - The ogive's curve depends on its fineness, so the nose length is found by bisection too.
  - Conical and tangent-ogive transitions are the same clipped or not; the test checks this to
    1e-12.
  - A clipped ogive with $\rho/\rho_t < 1$ is rejected: its profile isn't monotone, so the cut is
    ambiguous.

## Solids of revolution

Per unit density, with inner radius $r_i$ (zero when filled):

$$
\begin{aligned}
V &= \pi \int (y^2 - r_i^2)\,\mathrm{d}x, \\
\bar{x} &= \frac{\pi}{V} \int x\,(y^2 - r_i^2)\,\mathrm{d}x, \\
J_a &= \frac{\pi}{2} \int (y^4 - r_i^4)\,\mathrm{d}x, \\
J_t &= \pi \int \frac{y^4 - r_i^4}{4}\,\mathrm{d}x \\
&\quad + \pi \int x^2 (y^2 - r_i^2)\,\mathrm{d}x \\
&\quad - V\bar{x}^2, \\
S &= 2\pi \int y \sqrt{1 + y'^2}\,\mathrm{d}x, \\
A_p &= 2 \int y\,\mathrm{d}x, \\
x_p &= \frac{\int x y\,\mathrm{d}x}{\int y\,\mathrm{d}x}
\end{aligned}
$$

Each slice is an annulus: $(\pi/2)(y^4 - r_i^4)\,\mathrm{d}x$ about the axis and
$(\pi/4)(y^4 - r_i^4)\,\mathrm{d}x$ about its diameter, moved to the reference plane by the
parallel-axis theorem. $S$ excludes the end faces.

- **Numerics.** Each half of the profile is integrated from its own end in $u = s^2$, where $u$ is
  the normalized distance from that end. The substitution removes the $u^{-1/2}$ singularity of a
  blunt tip's surface integrand, and measuring from the end keeps the tip exact. The integrals use
  `hpr_core::quadrature` ([Quadrature](quadrature.md)) at a relative tolerance of 1e-12.
- **Walls** ([ADR-006][adr-006], component geometry and mass properties). A wall of thickness
  $t$ is the part of the solid within $t$ of the outer surface, so $t$ is measured normal to the
  surface, which is how molded and laid-up shells are made. Its inner radius is the lower envelope
  of circles of radius $t$ on the profile, over $s \in [0, L]$:

  $$
  \begin{aligned}
  r_i(x) &= \max(0,\ m), \\
  m &= \min_{s} \bigl[y(s) - d(s)\bigr], \\
  d(s) &= \sqrt{t^2 - (x - s)^2}
  \end{aligned}
  $$

  with the minimum taken over the $s$ where $\lvert s - x\rvert \le t$. The surface is the profile
  over its own length, ends included, with no extension past a cut end.
  - The envelope is exact for any continuous profile. A point above the lower half of some
    surface point's circle has the profile crossing its height closer than $t$, so it is in the
    wall anyway.
  - **Cut ends.** Where the surface meets the end plane at an obtuse angle inside the wall (the
    small end of a transition, the base of a bulged ogive), the rim's circle rounds the wall's
    inner corner.
    - A square cut would add a sliver of $t^2 (\tan\varphi - \varphi)/2$ of section per unit rim
      length, with $\varphi$ the surface's angle to the axis: 3.1e-4 $t^2$ at 7°. On steep ends it matters: a
      square-cut part is heavier than this model by 1.26% of wall mass for a 27 to 49 mm (1.1 to 1.9 in) transition
      over 15 mm (0.59 in, 56°), and by 2.24% for 20 to 37.3 mm (0.79 to 1.47 in) over 10 mm (0.39 in, 60°), both with $t = 2\,\mathrm{mm}$. The
      OpenRocket comparison ([M2.2](../decisions-and-roadmap.md#m2-2)) should check how real parts and OpenRocket treat such
      ends.
    - The sliver grows without bound only as the end turns vertical. There, a square cut (made by
      extending the surface along its tangent) closes the end with a disc of thickness $t$.
    - The first version did extend along finite end tangents only. Its wall mass jumped by 8.3%
      between shapes whose end slopes rounded to finite and to infinite.
  - The minimum comes from a 32-point scan, a golden-section search, and the two end points, which
    the search only approaches from inside.
  - The hollow is integrated separately. It is split where $r_i$ reaches zero and where the nearest
    surface point moves between the lateral surface and a rim, since both are kinks.
  - [TD] doesn't say how OpenRocket measures thickness, and [CR] measures it radially. The two
    differ by a factor $\sqrt{1 + y'^2}$ in wall volume, 1.4% for a cone three calibers long. The
    OpenRocket comparison ([M2.2](../decisions-and-roadmap.md#m2-2)) will measure OpenRocket's choice.

## Verification

- **Closed forms** (`shapes::tests::nose_volumes_match_closed_forms`, 1e-10 relative):
  - Every shape's filled volume and centroid.
  - Cone and ogive: $V$, $\bar{x}$, $S$ and $A_p$ from arc integrals.
  - Half spheroid: $\bar{x} = 5L/8$, and $S$ for prolate and oblate cases.
  - Power series: $V = \pi R^2 L/(2n+1)$, $\bar{x} = L(2n+1)/(2n+2)$, and the paraboloid's $S$.
  - Power series and parabolic series: $A_p$ and $x_p$ too; parabolic series by polynomial
    integrals.
  - Haack: $V = \pi R^2 L\,(\tfrac12 + 3C/16)$ and

    $$
    \bar{x} = \frac{L(11 + 3C)}{2(8 + 3C)},
    $$

    integrated in $\theta$.
  - Loft's tangent-ogive value, $R = 0.04\,\mathrm{m}$, $L = 0.25\,\mathrm{m}$ gives 6.7509e-4 m³
    ([Loft lesson L91](../decisions-and-roadmap.md#l91)).
- **mpmath references** (`solids::tests::filled_solids_match_the_mpmath_references`):
  - 22 noses and transitions of every family, in both directions, clipped and not.
  - All seven quantities, to 1e-12 relative (worst measured 2.4e-14).
  - Reference: `validation/fixtures/design/shape-integrals.json`, from
    `validation/oracles/design/shapes.py` (40-digit tanh-sinh on the defining formulas, Haack in
    $\theta$, with no code shared with Rust).
  - These references alone check the wetted areas of the power series ($n \ne \tfrac12$) and parabolic
    series, both Haack areas, and the moments of inertia of the filled shapes.
- **Walls against mpmath** (`solids::tests::walls_match_the_mpmath_references`):
  - 20 walls: 11 noses of every family, and 9 transitions both ways, including unclipped blunt
    ends and clipped ones.
  - Volume, centroid and both moments to 1e-10 relative (worst measured 5.9e-12).
  - Reference: `validation/fixtures/design/wall-integrals.json`, from
    `validation/oracles/design/walls.py`, which shares no code with Rust.
    - It finds the envelope from the roots of its derivative, bracketed from the window edges and
      solved by bisection.
    - It splits the integrals at the kinks it finds, and requires every hollow integral's error
      estimate below 1e-18.
  - Reviews found three faults this test now pins:
    - Blunt transition ends failed to converge, or were 5e-6 low, until the end points became
      candidates.
    - Tangent extensions made wall mass jump with the end slope.
    - The oracle itself first missed minima next to the window edge and left kinks unsplit.
- **Walls by hand:**
  - A conical wall is the cone minus the same cone moved aft by $t/\sin\beta$: volume, centroid and
    both moments by hand, to 1e-9. A cone so thick that its hollow is 3.8 mm (0.15 in) long matches the same
    formula to 1e-12 (`a_nearly_filled_cone_matches_the_offset_cone`).
  - A conical transition's wall is the square-cut frustum shell less the fore rim's sliver, in
    polar coordinates about the rim: mass and centroid to 1e-10, sliver section to 1e-10
    (`mass::tests::hollow_transition_and_freeform_fin_cg_are_exact_centroids`).
  - A tangent-ogive wall is bounded by the concentric arc of radius $\rho - t$: volume by hand, to
    1e-10.
  - A tube matches the hollow-cylinder formulas.
  - A wall thicker than the body fills it, and a thin wall's volume tends to $S\,t$.
- **Profiles:** each ends at 0 and $R$, slopes match central differences, and parameters out of
  range are errors ([Loft lesson L48](../decisions-and-roadmap.md#l48)).
  - Haack tips use $\theta = 2\arcsin\sqrt{\xi}$ and a Taylor series for $\theta - (\sin 2\theta)/2$
    below $\theta = 0.1$, so the tip slope is $+\infty$, never NaN.
  - Ogive radius ratios up to 1e12 give the cone. A power series at the minimum exponent, 0.05,
    matches its closed-form volume as a nose and as transitions both ways
    (`extreme_parameters_stay_accurate_or_fail_loudly`).
  - Unknown fields in a shape or wall are rejected.
  - Transitions hit both radii and are monotone both ways, clipped or not
    ([Loft lesson L49](../decisions-and-roadmap.md#l49)); bulged ogives are excluded because their profile is deliberately
    not monotone.
  - Power-series exponents below 0.05 are rejected. Blunter profiles approach a flat face the
    integrals can't resolve, and unclipped transitions below about 0.038 fail to converge.

[adr-002]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0002-the-reference-library-lock-file-fetch-verify-and.md
[adr-006]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0006-component-geometry-and-mass-properties-frames.md
[levels]: ../accuracy.md#four-kinds-of-evidence
