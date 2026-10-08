# ADR-087: Mass that moves along the airframe (2026-09-26)

- **Status:** accepted
- **Summary:** Mass that moves along the airframe

**Context.** M1.12 (VISION V18) asks for payload mass that moves along the airframe or leaves it,
on an event or a schedule, with the mass, centre of mass and inertia updated through the flight,
and for the equations of motion to carry the moving mass's relative-motion terms or an ADR to show
them negligible. That is two pieces of work: a mass that moves inside a rocket that stays whole,
and a mass that leaves while the rest flies on in six degrees of freedom. Today a parting turns
every body into a point mass (ADR-085), so the second needs the vehicle swapped as a powered
separation does. The equations (ADR-011) already carry a moving centre of mass (`r′`, `r″`) and a
changing inertia (`I_O′`), fed by central differences of the motors' mass properties while a motor
burns.

**Decision.**

1. **Split M1.12 into M1.12a (a mass that moves) and M1.12b (a mass released).** M1.12a's *done
   when* is the milestone's first bullet for a move, its second bullet, and a test of the
   relative-motion terms. M1.12b keeps the release half of the first bullet.
2. **A `MassShift` moves one internal part** (by component id, with everything inside it) a set
   travel along the axis, positive aft, over a set time, on the triggers a recovery device has.
   One with a trigger known before the flight starts then. The flight watches for the apogee and
   for a height on the way down, as it does for a device. Once a shift's start is known, its
   start, its end and 16 equal intervals between are stop times, so even a fixed step takes 16
   steps across it: review measured RK4 at 10 ms taking a 10 ms move in one step off by 0.071 m/s
   in the centre's velocity, and 1.2e-4 m/s with the stops.
3. **The motion is a cycloid**, `s(τ) = τ − sin(2πτ)/2π` of the travel over `τ = (t − t₀)/T`, the
   cam designer's cycloidal motion (R. L. Norton, *Design of Machinery*, the chapter on cam
   design): at rest at both ends with zero acceleration there, so neither the part's speed nor its
   acceleration jumps. A move's profile is not given by any source for rockets; this is a choice
   that keeps the equations smooth, and it is stated in the docs.
4. **The mass properties are exact.** The part's point contribution about the new centre is taken
   out where it was and put back where it is (`MassProperties::with_part_moved`, the
   parallel-axis theorem); its own inertia moves with it unchanged.
5. **The shift's rates are in closed form**, not differenced: `r′`, `r″` and `I_O′` gain the part's
   terms, including their coupling to a burning motor's `M′` and `M″`, which are still
   differenced. A first draft differenced the whole, 0.1 ms apart as for a motor; in free flight
   that held the centre's velocity only to 1e-8 m/s, the stencil's `(2πh/T)²` error, so the
   closed form replaced it.
6. **The relative-motion terms are carried, not shown negligible.** Summing over the particles
   about the nose tip gives the equations of ADR-011 exactly, plus `ω × h + h′` in the rotational
   equation, where `h = Σ m ρ × ρ′` is the moving parts' angular momentum relative to the
   airframe. It is zero for a part on the axis, and not for one off it. `T21` subtracts it.
7. **What a shift refuses.** A body component or an external one; one copy of a cluster's; a part
   that holds a motor (the motor would stay put); a part inside another that moves; a part in a
   stage, or inside a component, whose overridden mass covers it; a travel that can take the part
   out of the component that holds it, forward moves added together and aft moves added
   together, or past where the design already puts it (review found a weight in a nose cone's
   shoulder refused at any travel); a move shorter than 10 ms, nearer an impact than a motion
   (63 km/s² per metre of travel at that bound); a shift in a flight with a separation or
   ejections, whose pieces are fixed with every part where the design puts it (M1.12b's to lift);
   and, in flight, a shift that starts on the pad or the rail. The rail has no stop at its foot,
   and review found an aft shift at launch lifting the rocket at 4 µs, where it could stall and
   hang above the pad.
8. **`Simulation::mass_properties(flight, t)`** gives the stack's mass properties as a flight flew
   them: a shift with a trigger known before the flight started then, and one the flight watched
   for started where the flight's `EventKind::Shift` event says.

**Consequences.**

- `shifts::tests` holds the milestone's bullets. The 54 mm test design with 200 g of ballast moved
  0.3 m aft over 1 s from 5 s: its mass, centre and inertia before, halfway and after match the
  two-body hand calculation (reduced mass times the separation's `|L|² E − L Lᵀ`) to 1e-15. Its
  static margin at every coast sample is the hand value, `−m Δ s(τ)/(M d)`, to 1e-12 calibres:
  4.30 before, 3.00 after.
- In free flight, with no air and no gravity, the ballast 1 cm off the axis moving while the
  rocket turns about all three axes keeps the angular momentum about the centre to 6.9e-12 of
  itself and the centre's velocity to 2.6e-12 m/s. The part's relative angular momentum peaks at
  1.8% of the whole, so without `ω × h + h′` the error would be of that order.
- The closed-form rates match differences of the mass properties during the burn and after it:
  `r′` to 2e-11 m/s, `r″` to 3e-8 of itself (the difference's own error), `I_O′` to 1e-9.
- In a vacuum, a drogue that opens halfway through a move keeps the centre's velocity to
  1e-12 m/s, and the centre then falls freely to 1e-9 m/s while the ballast finishes. Every
  refusal has a test that checks which rule fired.
- A flight with no shift runs the same arithmetic as before: the validation report, the corpus
  flights and the real flights reproduce.
- Left for later: M1.12b's release; a shift with a separation or ejections; a mass that moves
  across the axis or turns.
