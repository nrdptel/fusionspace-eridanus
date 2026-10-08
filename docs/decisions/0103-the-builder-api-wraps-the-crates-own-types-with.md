# ADR-103: The builder API wraps the crates' own types, with no default materials (2026-09-28)

- **Status:** accepted
- **Summary:** The builder API wraps the crates' own types, with no default materials

**Context.** M4.1 asks the `hpr` crate for a builder API in the manner of RocketPy's
(`Environment`, `Motor`, `Rocket`, `Flight`), trait-based custom models, at least 4 examples and a
rustdoc guide. Building a rocket in code took about 140 lines of struct literals in
`own_rocket.rs`, since `hpr_design` has only public fields and no constructors. The milestone is
more than one pull request's safe share, and its two halves are separable: the builder, then the
models a user can put in hpr's place.

**Decision.**

1. **M4.1 is split.** M4.1a is the builder, with Loft lesson L95; M4.1b is a drag-model trait the
   flight calls in place of hpr's drag, the custom aero example, two more examples and the rustdoc
   guide. M4.1's own bullets close with M4.1b.
2. **The builder wraps, never replaces.** Each type hands over what it makes: a `Rocket`'s
   `design()` is the `hpr_design::Rocket` a design file holds, a `FlightBuilder`'s `simulation()`
   the `hpr_sim::Simulation` it runs, an `Environment`'s `sim()` the `hpr_sim::Environment`. What
   the builder doesn't offer (staging, clusters, pods, shifts, user events) stays reachable, and
   no physics is written twice. The crates are re-exported by name (`hpr::hpr_sim`), not renamed.
3. **No default materials or walls.** Every part names its material (`rocket::material(id)` finds a
   built-in one) and a hollow part its wall: each default would be a guess at the rocket's mass. The
   defaults the builder does take change the drag or the placing, not the mass: fins square-edged,
   every surface the design's default finish (mass-production paint), a part flush with its tube's
   aft end, a motor tube with no overhang, a mass a point until packed. A position places a
   packing's end, so packing a mass moves its centre by half its length unless it is placed by its
   middle: 22 mm of the example rocket's centre of gravity for its 15 cm recovery bay.
4. **Order is the tree's.** Body parts stack from the nose in the order added; fins, the motor tube
   and masses go on the last body tube. A nose after any body part, fins before any tube, a second
   motor tube and a motor with no tube are refused with `Error::Order`, whose `Order` says which,
   so a binding can match it. The nose's base radius is the rocket's diameter, a tube's the aft
   radius of the part before it unless given; the nose's shoulder's radius is left automatic, to
   fit the tube behind it. Fins take a named `FinPlanform`, not a list of lengths that could be
   given in the wrong order. Ids are the part's kind, numbered from the second (`tube`, `tube-2`).
5. **One motor, lit at launch.** `set_motor` replaces the design's one configuration, named after
   the designation. `Motor::from_catalog` takes only motors with a bundled curve, and refuses a
   name that matches several (`I175` matches two) rather than taking the first. The delay is set,
   never read from the designation.
6. **Weighing runs the flight's checks.** `Rocket::assemble`, and so the mass properties and the
   margin, refuse with `Error::DesignChecks` what `Simulation::new` refuses: a program never shows
   a margin for a rocket the flight won't fly. The same findings reach a flight as
   `Error::Sim(SimError::DesignChecks)`, from the simulation beneath. A flight's settings can set
   `accept_design_errors` to fly such a design anyway; the builder's weighing has no such switch.
7. **Parachutes go on the rocket**, as RocketPy's `add_parachute` has it: a `Device`, drag only;
   its mass is a `Mass` the user adds.
8. **An elevation is both heights.** `Environment::new` takes the site's elevation as its height
   above mean sea level and above the ellipsoid, as `hpr_sim::Environment` does with no undulation;
   `from_sim` takes an environment built with one.
9. **L95, degenerate designs.** The builder checks each part's numbers as it is added (finite,
   positive or not negative) and names each in `Error::Domain`; fins go through
   `FinSet::validate`, a nose's and a transition's shape through `Profile`. The rail's numbers are
   checked when the flight is set up, in the degrees they were given. Put straight
   into a design, a zero-radius tube, a NaN length or span, zero fins, a negative mass and a
   negative mass override are refused by `Simulation::new`'s design checks, each by its part; a
   rocket with no fin set flies, tumbling, to finite numbers.

**Evidence.** `cargo test -p hpr`: the builder's rocket, assembled and flown in `own_rocket.rs`'s
conditions, has that example's mass properties at six times, the same dry mass properties, and a
`FlightResult` equal to the hand-built tree's, bit for bit
(`the_builder_makes_the_rocket_own_rocket_builds_by_hand`); `degenerate_designs_error_or_stay_finite`
pins each refusal by its part and quantity; `packing_a_mass_moves_its_centre_unless_placed_by_its_middle`
pins the packing's shift to 1e-12 m against `m L / 2 M`; `weighing_refuses_what_flying_refuses`
refuses a 54 mm motor in the 29 mm tube both ways.

**Consequences.**

- The facade's first two examples, `build_and_fly` and `motor_choice`, run in CI; the guide's page
  *The builder* explains them.
- `FlightBuilder` borrows its rocket and environment. The Python and WebAssembly bindings
  (M4.3, M4.4) can't hold a borrow, so they will want an owned launch description; that is theirs
  to add.
- A design fault reaches the caller as `Error::Design` from weighing and as
  `Error::Sim(SimError::Design)` from a flight, the crate each came through.
- Custom drag waits for M4.1b. Until then a drag table (`Simulation::with_drag_table`) through
  `FlightBuilder::simulation` is the way to put another source's drag in hpr's place.
