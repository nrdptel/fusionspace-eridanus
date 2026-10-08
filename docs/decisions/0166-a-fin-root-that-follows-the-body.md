# ADR-166: A fin set on a nose cone or a transition, its root along the surface (2026-10-05)

- **Status:** accepted
- **Summary:** M4.5g4: a freeform fin outline's last point may stand above its first, and the planform gains `root_m`, the root's points between its trailing and leading edges. On a body tube the root stays level. A fin set may now sit on a nose cone or a transition: the tree checks that each of its root's points lies within a micron of the surface, that the straight pieces between them cut off or add at most 0.1% of the fin's area, and that it has no tab or fillet. Its body radius is the surface's at the root leading edge, from which its heights are measured. The `.ork` reader draws the root through 63 points on the parent's profile. OpenRocket's *Pods--airframes and winglets* flies all five configurations, each apogee within 0.5% of OpenRocket's record; its cockpit's area, mass and centre of mass match OpenRocket's on OpenRocket's own root points

**Context.** M4.5g4 asks that *Pods--airframes and winglets*, whose freeform outlines "don't run
from the root's leading edge to its trailing edge", fly with them read as written, or that an ADR
measure why they can't be. Only one of the example's two freeform sets has such an outline: the
*Cockpit*, a single 7 mm balsa fin on the last 50 mm of the nose cone, a tangent ogive 136.525 mm
long of base radius 16.8275 mm. Its points are `(0, 0)`, `(9.35, 6.96)` and `(50, 2.2277)` mm. The
*Wings* end on the root and were already read.

What the file and OpenRocket 24.12 say (the oracle run through JPype, no source read):

- The last point's height is the ogive's rise over the root:
  `r(x_LE + 50 mm) − r(x_LE) = 2.22766 mm`, to `3.5e-18` m. OpenRocket measures a fin's heights
  from the surface at the root leading edge, and the root runs along the surface.
- OpenRocket's `getRootPoints` draws the root through 21 points, 2.5 mm apart, on the ogive.
  `getPlanformArea` is 1.44954e-4 m², the outline closed along those points. Closed straight along
  the chord instead, the triangle would be 1.6350e-4 m², 12.8% more.
- Its mass is 1.72126e-4 kg and its centre of mass 19.1163 mm aft of the root leading edge and
  17.882 mm from the axis, where `getBodyRadius`, the radius at the root leading edge, is
  14.5998 mm.
- At OpenRocket's wind direction θ = 0 the single fin lies in the plane of the airflow and adds no
  normal force. At θ = 90° it adds 0.2117 per radian.

hpr refused the outline twice over. `hpr-design` attached fin sets to a body tube only, and its
freeform planform had to end at `h = 0`.

**Decision.**

1. **The planform.** `FinPlanform::Freeform` keeps `points_m`, from the root leading edge `[0, 0]`
   to the root trailing edge `[c, h_r]`, and gains `root_m`: the root's points from the trailing
   edge forward, empty for a straight root, serialized only when present. `h_r` may be above zero,
   and every point stays at or above the root leading edge. The polygon is the outline followed
   by the root, simple and turning clockwise in `[x, h]`. Every outline over a level root turns
   that way; one that turns the other way runs below its root, and is refused by name. The area,
   centroid, mass and both fin aerodynamic models integrate the whole polygon. The span is still
   its highest point.
2. **Where it may sit.** On a body tube a fin set's root must be level: an outline ending at
   `h = 0` with no root points. On a nose cone or a transition, `FinSet::root_radius_on` checks
   that the root stays on the body's length and that each root point, its ends included, lies
   within `FinSet::ROOT_ON_SURFACE_M`, 1 µm, of `h = r(x_LE + x) − r(x_LE)`. The set must carry
   no tab or fillet, whose models take a cylinder. Between points the straight pieces cut across a
   curved surface. `FinSet::root_on_surface` sums each piece's parabolic sliver, `(2/3) δ Δx`
   with `δ` the surface's distance from the piece's midpoint. Their total must stay within
   `FinSet::ROOT_SLIVER_SHARE`, 0.1% of the fin's area, a tenth of the 1% bar `cargo xtask ork`
   holds masses to. OpenRocket's own 20 pieces on the cockpit come to 0.032%; the chord alone,
   which a hand-written design could give, to 11.4% of its area, and is refused. Its body radius,
   for its mass and for fin–body interference, is `r(x_LE)`. A level trapezoid on a cone stands off its surface and is refused.
   Lugs, buttons and tube fins still attach to a body tube only.
3. **The reader.** A freeform set on a nose cone or a transition, placed from its parent's top,
   middle or bottom, whose outline ends within the micron of the surface, is read with
   `ROOT_PIECES` = 64 straight pieces along the parent's profile. Each refusal is a sentence that
   names its reason:
   - a trapezoidal or elliptical set there;
   - a tab or a fillet;
   - placement after a sibling or at an absolute station, where the reader can't find the
     surface;
   - a parent of automatic radius, which the layout settles from its neighbours only after the
     reader has drawn the root: a root drawn on the radius OpenRocket cached could stand off the
     surface the layout settles on, and fail the whole design;
   - a root past the parent's ends;
   - an outline ending off the surface;
   - a body that narrows along the root.

   Last, the reader runs the layout's own check (`FinSet::root_radius_on`) on the root it drew, so
   a fin it reads never fails the layout. On a body tube an outline that ends off the root is
   refused as before. The exporter writes the
   outline alone, as OpenRocket does, and reading it back draws the same root.
4. **Why 64 pieces, not OpenRocket's 20.** The chord of each piece sits inside a convex surface.
   On the cockpit 64 pieces put the root at most 0.14 µm inside the ogive and the area 3.1e-5
   above the curve's; OpenRocket's 20 give 2.9e-4 more area than hpr's 64. The test
   `a_root_along_a_nose_cone_is_openrockets` pins both and reproduces OpenRocket's area, mass and
   centre of mass to its printed digits on OpenRocket's 20 pieces.

**Measured.** `cargo xtask ork-flights` flies all five configurations of *Pods--airframes and
winglets* (it needs `--accept-design-errors` in `hpr sim`: its centering rings are children of the
18 mm motor tube, reaching 16.3 mm from its axis where it has 9.0 mm of room, which hpr's checks
rightly flag). Apogee, hpr less OpenRocket:

| configuration | OpenRocket (m) | hpr (m) | Δ |
|---|---:|---:|---:|
| [A8-3] | 32.3 | 32.3 | −0.13% |
| [B6-4] | 90.5 | 90.7 | +0.18% |
| [C6-5] | 199.7 | 200.7 | +0.48% |
| [C12-6] | 207.5 | 207.7 | +0.10% |
| [D16-6] | 245.7 | 246.0 | +0.10% |

Every descent meets ADR-153's targets. The configurations flown rise from 43 to 48, and none of
the report's other rows changes. The cockpit's drag is part of each flight: the [C6-5] without it
would reach 204.4 m.

**What it leaves.** At rod clearance hpr's margin is 0.071 to 0.076 calibres above OpenRocket's on
all five, which is the flattering side. One-part probes against OpenRocket size it, and neither
cause is this decision's: removing the cockpit leaves hpr's centre of pressure where it was. The
probes put 0.97 mm of the 2.56 mm CP gap on OpenRocket's fin-count factor (×0.948, five fins)
taken across the airframe's fin sets, which hpr takes per set
([#325](https://github.com/nrdptel/hpr-sim/issues/325)). They put 1.58 mm on the kinked freeform
*Wings*' own CP and normal force ([#326](https://github.com/nrdptel/hpr-sim/issues/326)). The
cockpit's own normal force across the airflow (θ = 90°, Mach 0.3) is 0.2784 per radian in hpr
against OpenRocket's 0.2117, 31.5% more, with its CP 0.13 mm from OpenRocket's. Closed straight,
hpr's would be 0.2802, so the curved root isn't the cause. It acts forward of the centre of mass,
which shortens hpr's margin, the safe side, and
`the_pods_cockpit_reads_above_openrocket_across_the_airflow` pins it (#326). On a
curved root the supersonic tip-cone model still reflects at the level line through the root
leading edge, and a fin on a body that narrows along its root (a boattail) is refused, not read.

**Rejected.**

- **Closing the outline straight along the chord.** It would add 12.8% to the cockpit's area, and
  the file doesn't hold that fin.
- **A planform that asks its parent for the surface.** `FinPlanform` would stop being
  self-contained: its area, mass and aerodynamics would all need the body.
- **Refusing it, with this ADR as the measurement.** The file can be read as written, and the
  measurement above shows it.
