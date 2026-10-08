# ADR-007: Design tree: stations, placement, automatic radii, overrides, motors and checks (2026-09-17)

- **Status:** accepted
- **Summary:** Design tree: stations, placement, automatic radii, overrides, motors and checks

**Context.** M1.4b assembles M1.4a's parts into rockets. Loft's tree placed parts from OpenRocket
conventions it had read from OpenRocket's source, which this project can't do. It chose the
reference diameter from any component, internal ones included (L47). It flew a motor wider than
its mount and fins off the airframe without complaint (L50). RocketPy takes a rocket's mass,
centre and inertia as inputs and adds the motor; it computes nothing from geometry.

**Decision.**

- **Body origin at the nose tip.** `z_ref = 0`, so station `s` (aft of the tip) is `z = −s`.
- **One `Component` type holding a `Part` enum.** Body components (nose cone, body tube, transition)
  are a stage's list and stack from `s = 0` through every stage. External parts (fins, tube fins,
  lugs, rail buttons) hang off body tubes. Internal parts hang off body components or inner tubes.
  Roles are checked when the tree resolves, so importers build one type. Fins on noses and
  transitions are refused until a milestone models a sloped root.
- **Positions** are `top`, `middle`, `bottom`, `after` (the previous sibling) and `absolute`, with
  offsets positive aft. An attached part's extent is its root chord for fins, a whole row for lugs
  and buttons, and its packed length for packed parts.
- **Automatic radii** are a list of named dimensions on the component; stored values for them are
  ignored. Body radii take neighbours across stage boundaries. The previous component wins for
  tubes, and the next is a fallback. Shoulders take the adjoining tube's bore, rings their parent's
  bore and the widest overlapping on-axis sibling tube, and packed parts the bore.
- **Overrides** apply mass (rescaling the tensor with the mass), then the centre (along the axis
  from the forward end of what is overridden, and optionally off it, keeping the tensor about the
  centre), then the full tensor. A component's overrides cover it alone, or its subtree with `overrides_include_children`.
  A stage's cover the stage. Deeper overrides apply first; motors are never covered. The inertia
  override has no OpenRocket counterpart; RocketPy's examples need it.
- **Motors.** A `MotorMount` on a body tube or inner tube, with an overhang. Configurations put an
  embedded `SolidMotor`, with its case diameter and length, in each mount. The nozzle exit sits at
  the mount's aft end plus the overhang, on the mount's axis. All motors ignite at `t = 0` until
  M1.9. Catalog references wait for the facade (M4.1) and the online catalog (M5.4).
- **Reference diameter** defaults to the widest body component in any stage. `nose_base` and
  `custom` are the alternatives.
- **Checks** return typed findings with a severity.
  - Errors: a motor wider than its mount or wholly outside it, an external part that doesn't
    overlap its body tube, an internal part off the rocket or reaching past its parent's bore
    (measured about the parent's own axis), and a stage centre moved off the rocket by an override.
  - Warnings: a motor past its mount's top, attachments and internal parts past their parent's ends,
    a ring crossing an inner tube, radius steps, no nose cone.
  - Lengths compare with 1 nm of slack.
  - The flight engine (M1.6) must refuse designs with errors unless the caller accepts them.
  - `Layout::place_motors` takes any configuration, so a candidate motor is checked before it is
    stored.
- **Errors and limits.** An error inside a stage or component carries its id
  (`DesignError::InComponent`). Components nest at most 32 levels. `MassProperties::validate` now
  also refuses inertia on a body with no mass.
- **Public test designs** live in `validation/designs/` as JSON of `hpr_design::Rocket`. The format
  is provisional: M3.3 defines the open format and migrates them.
- **RocketPy comparison with substituted curves.** RocketPy's data files carry their own terms
  (`THIRD-PARTY-NOTICES.md`).
  - The committed fixture takes inputs only from RocketPy's notebooks and test code. It keeps each
    example rocket's inputs exactly, and pairs its motor with the bundled public-domain curve
    nearest in impulse.
  - Valkyrie is left out, because its inputs exist only in a data file.
  - Mass composition doesn't depend on which curve schedules the burn. Values at ignition are
    identical, and at burnout they agree to 1e-9. A local run with the examples' own curves stays
    under `refs/`.
  - The fixture also samples RocketPy at its LSODA knots. There hpr agrees to the solver's
    accuracy (1e-9), and the test holds 1e-8.

**Consequences.**

- M1.5 takes stations, radii and the reference area from `Layout`.
- M1.6 refuses designs with error findings, and takes `Assembly::mass_properties(t)` as the
  time-varying mass model.
- M1.9 adds ignition times, separation and per-stage assemblies to configurations.
- M2.2 measures OpenRocket's override order (L51) and automatic-radius rules with the jar. M3.1
  maps `.ork` positions, auto flags and overrides onto these types, and reports any rule that
  doesn't map.
