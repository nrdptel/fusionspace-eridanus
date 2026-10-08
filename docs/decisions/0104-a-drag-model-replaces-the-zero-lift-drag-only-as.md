# ADR-104: A drag model replaces the zero-lift drag only, as a drag table does (2026-09-29)

- **Status:** accepted
- **Summary:** A drag model replaces the zero-lift drag only, as a drag table does

**Context.** M4.1b asks for trait-based custom models: a drag model the flight calls in place of
hpr's own, an example that overrides a built-in model through the trait, and a rustdoc guide.
The wind and the atmosphere were already traits (`hpr_atmos::Wind`, `Atmosphere`), taken by
`Environment::with_wind` and `with_atmosphere`; the drag was a table or hpr's buildup. A drag
table already had a path through the model and the flight, pinned by tests: the template.

**Decision.**

1. **`hpr_aero::DragModel` gives the zero-lift drag coefficient only**, on the rocket's reference
   area, from a `DragQuery`: the flow, the drag conditions (Reynolds number per metre, whether a
   motor is thrusting, as the buildup reads them) and hpr's own buildup at that flow
   (`DragQuery::buildup`, which calls the new `AeroModel::buildup_drag`). As with a table, the
   flight scales it by `f(α)` for the angle of attack; the normal force, centre of pressure, roll
   and damping stay hpr's, so a rocket's margin doesn't change. A whole-force model (normal force
   and moments) would have to replace the local-flow damping too (ADR-032's reasoning), and is
   left until someone needs one.
2. **A model and a table replace each other**: the last set is the one flown, on
   `AeroModel`, `Simulation` and `FlightBuilder`. Neither is added to the other.
3. **A model is held as `Arc<dyn DragModel>`**, `Debug + Send + Sync` as a `Wind` is.
   `with_drag_model` takes a model and shares it; `with_shared_drag_model` takes an
   `Arc<dyn DragModel>` as it is, which the facade's clonable `FlightBuilder` uses, so every
   flight of one builder shares one model. There is no blanket `impl DragModel for Arc<T>`: it
   would let an `Arc` be wrapped in a second one, silently. Two `AeroModel`s are equal when they
   hold the same model (`Arc::ptr_eq`). A serialized `AeroModel` names its model by its `Debug`
   text, and leaves the field out when there is none, so no committed output changes.
4. **A query is narrow.** `DragQuery` gives the flow, the conditions as the buildup reads them,
   the reference area, the length and hpr's buildup (whole and by component), but not the
   `AeroModel` itself, whose `drag` would ask the model again without end. Its constructor is the
   crate's own, so fields can be added without breaking a model; a model is tried by giving it
   to an `AeroModel`.
5. **Checks.** A coefficient that is negative or not finite is refused with
   `AeroError::Domain { what: "zero-lift drag coefficient from a drag model" }`, and a Mach
   number that is negative or not finite before the model is asked. The model's own errors,
   including the buildup's refusals it passes on, come back wrapped in the new
   `AeroError::DragModel`, so a caller can tell them from hpr's. Like a table, a model accepts
   any finite Mach number; the flight's normal force still ends at Mach 5.
6. **A model is the whole stack's**, as a table is: a flight with a powered separation refuses it
   at the separation, since the sustainer's own model is built from the design.
7. **Examples.** `custom_drag` (hpr's drag scaled, and a curve against Mach number with power-on
   and power-off columns, invented), `custom_wind` (a veering wind through the `Wind` trait) and
   `fin_sizing` (a design study as a loop) join `build_and_fly` and `motor_choice`: five builder
   examples. The three open the parachute at apogee, so the motor's charge doesn't cut the climb
   short and change the apogee they compare. The guide in the reference is `hpr::guide`, five chapters of docs with doctests;
   the site's page is *Models of your own*.

**Evidence.** `cargo test -p hpr-aero custom`: a model handing back the buildup gives the
buildup's `C_D0` and `C_A` bit for bit over Mach 0 to 4.9 and 0° to 120°; a constant model equals
a constant table to Mach 7; the model sees the conditions as read under
`with_full_base_drag_under_power`; each refusal pinned by its `what` or value, a model's own
error by its wrapping; two models holding one shared model are equal; a serialized model names
its drag model. `cargo test -p hpr`
(`a_drag_model_is_flown_in_place_of_hprs_drag`): the builder's rocket flown with that model has a
`Flight` equal to the one without, and a constant model's `FlightResult` equals the constant
table's. `staging_refuses_what_it_cannot_fly` refuses a model at a powered separation, and fails
when `with_drag_model` doesn't mark the override (mutation checked). A wind that isn't finite
stops the flight, named by the Reynolds number's check (`a_wind_that_is_not_finite_stops_the_flight`).

**Consequences.**

- M4.1 closes: the examples run in CI, rustdoc builds with no warnings, and `custom_drag`
  overrides hpr's drag through the trait.
- An atmosphere of your own has no test of its own through the facade yet, and no example.
- `AeroModel` is no longer `UnwindSafe` or `RefUnwindSafe`, as `hpr_sim::Environment`, which
  holds an `Arc<dyn Wind>`, already wasn't. A caller that catches a panic around one wraps it in
  `AssertUnwindSafe`.
- hpr doesn't check a wind's value where it reads it; a wind that isn't finite is caught later,
  by the drag's Reynolds number while the rocket climbs. The site says so, and #237 tracks a check
  of its own.
- The Python and WebAssembly bindings (M4.3, M4.4) will need a way to call back into their
  languages for a drag model; that is theirs to design.
