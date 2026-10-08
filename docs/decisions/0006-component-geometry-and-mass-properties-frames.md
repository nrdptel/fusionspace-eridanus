# ADR-006: Component geometry and mass properties: frames, shapes, walls, fins and materials (2026-09-17)

- **Status:** accepted
- **Summary:** Component geometry and mass properties: frames, shapes, walls, fins and materials

**Context.** M1.4a builds the geometry and mass half of `hpr-design`. Loft's mass model went wrong
in the ways lessons L44–L46 and L48–L49 name: pitch inertia only, no radial terms, fin span and
tabs ignored, solid centroids for hollow parts, only a tangent ogive, and a kinked transition
profile. The published sources leave several conventions open. OpenRocket's documentation doesn't
say how wall thickness is measured, its ogive parameter is contradictory, and it doesn't say how
a boattail is oriented or where cant pivots (`docs/physics/shapes.md`, `docs/physics/mass.md`).

**Decision.**

- **Full tensor.** `MassProperties` carries mass, a 3D centre and the full inertia tensor about it
  in body axes, with positive products of inertia. Every part computes its tensor from geometry,
  so one- and two-fin sets, off-axis masses and lugs keep their products of inertia. It is not the
  "two nearly equal transverse moments" of OpenRocket's documentation (§4.2.3).
- **Component frames.** Each part has body axes with its origin on the axis at its forward end (a
  nose cone's tip), so it lies at `z ≤ 0`. The tree (M1.4b) translates and rolls it, and fixes
  the body origin `z_ref` that `frames.md` leaves to M1.4.
- **Shapes.** The ogive is Crowell's secant ogive, parameterized by `ρ/ρ_t` (1 is tangent; below
  1 bulges). OpenRocket's `κ` is not adopted, because its documentation defines it two ways; M3.1
  maps it by running the jar. The Haack parameter may reach 2/3, the monotone limit. A transition's
  shape puts its tip at the smaller end. Clipped transitions follow [TD] §A.7. Parameters out of
  range are errors, never defaults.
- **Walls are measured normal to the surface.** The wall is the part of the solid within `t` of the
  lateral surface: its inner surface is the envelope of circles of radius `t` on the profile, ends
  included and not extended, and the wall fills in near a tip. This is how a molded or laid-up
  shell is made.
  - At a cut end where the surface meets the end plane at an obtuse angle inside the wall, the
    inner corner is rounded rather than square: `t² (tan φ − φ)/2` less section per unit rim
    length, 3.1e-4 `t²` at 7°. On steep ends it is not small: 1.26% of wall mass at 56° and 2.24%
    at 60° for a 2 mm wall. M2.2 checks this against OpenRocket.
  - Extending the surface along its end tangent would cut it square. But that closes a steep end
    with a disc, and it made wall mass jump by 8.3% as an end slope rounded from finite to
    infinite, so it was dropped. Radial thickness, as in Crowell, overstates the wall by
  `√(1 + y′²)`. M2.2 measures what OpenRocket does, and any gap goes in the report, not into this
  model.
- **Fin cross-sections change the mass.** The section's thickness distribution is integrated
  exactly: square, rounded with semicircular edges, or airfoil as the NACA four-digit thickness
  distribution. OpenRocket's documentation uses the section for drag only. A slab would overstate
  an airfoiled fin by 46%; the NACA section is a definition, and it is documented as one.
- **Fin cant** pivots about the fin's span axis through the root mid-chord. Tabs are square slabs.
  The flat root sits at the body radius, and fillets wait for a milestone that needs them.
- **Recovery parts and mass components** are solid cylinders of their packed size. A parachute's
  cloth is `πD²/4`.
- **Numerics.** `hpr_core::quadrature` gains an adaptive vector G7K15 (QUADPACK's `QAG` without
  extrapolation), with substitutions and end-relative evaluation at blunt tips. Filled solids match closed forms
  to 1e-10 and 40-digit mpmath integrals to 1e-12; walls match an independent 25-digit mpmath
  envelope to 1e-10 (worst measured 5.9e-12). Principal moments
  use cyclic Jacobi, not the closed-form eigenvalue method that loses `√ε` for repeated moments.
- **Materials** are values stored in the design (name and density with its kind), not library
  keys, so designs stay complete offline. Built-in values cite primary public sources: USDA's Wood
  Handbook, manufacturers' data sheets and military specifications. Hobby tubes (cardboard, kraft
  phenolic, Blue Tube, Quantum) have no published density, so theirs is derived from the makers'
  published weights and dimensions, with the derivation recorded.
- **Crowell (1996)** is cited but not pinned: its only copy is on a plain-http mirror, and the
  Internet Archive was offline. The pinned OpenRocket documentation gives the same curves, and
  closed forms and mpmath integrals check them.

**Consequences.**

- M1.4b builds the tree on these parts: placement, auto radii, overrides, configurations with
  `SolidMotor`, reference diameter and checks. The part types hold resolved geometry. "Auto or
  fixed" radii and overrides belong to the tree's own types, so these serialized forms don't
  change when the tree arrives.
- Fin cant sign: a positive cant turns fin 0's leading edge toward `−y_B`, and a test pins it.
  M1.8 derives the roll-forcing sign from this geometry.
- M1.5 takes wetted areas, planform areas and centroids from `revolve` and the fin planforms.
- M2.2 compares OpenRocket's component masses, and reports the wall-thickness and cross-section
  conventions if they explain differences.
- M3.1 maps `.ork` shape parameters, the clipped flag, shoulders, tabs, instances and packed sizes
  onto these types, and settles the ogive parameter with the jar.
