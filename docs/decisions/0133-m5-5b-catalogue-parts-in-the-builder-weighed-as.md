# ADR-133: M5.5b, catalogue parts in the builder, weighed as OpenRocket builds them (2026-10-01)

- **Status:** accepted
- **Summary:** M5.5b: catalogue parts in the builder (`from_catalog` on `Nose`, `Tube`, `Transition`, `MotorTube`, and the new `Fitting`); what the file leaves unsaid as OpenRocket 24.12 builds it, but a hollow part's shoulder takes its wall; a stated mass scales the part's density; a part with an undefined material refused; every part held to OpenRocket's built mass and centre, the two codes' hollow walls each checked

**Context.** M5.5b's *done when* is "a rocket built from catalog parts flies through the builder;
each part's mass as built is held to OpenRocket's for its preset". ADR-132 reads OpenRocket's 16
bundled `.orc` files into `hpr_io::orc::Part`s. A part gives its sizes and material, and
sometimes its mass, but never a shape parameter, a shoulder's wall or a parachute's drag
coefficient. The builder (`hpr::rocket`) made noses, tubes, transitions, fins, one motor tube and
point masses; it had no couplers, rings, bulkheads, lugs or transition shoulders, and a nose's base
always took the rocket's diameter.

**Decision.**

1. **A `from_catalog` per builder part.** `Nose`, `Tube`, `Transition` and `MotorTube` (a body
   tube as the motor tube) each get `from_catalog(&orc::Part)`, and a new builder type, `Fitting`,
   takes the rest: a tube coupler or engine block (an `InnerTube`, as the `.ork` reader makes
   them), a centering ring or bulkhead (`CenteringRing`), a launch lug, and a parachute or
   streamer (their `hpr_design` parts, packed into a point). `Fitting` also has constructors to the
   caller's sizes (`coupler`, `centering_ring`, `bulkhead`, `launch_lug`), and
   `Rocket::add_fitting` puts one on the last body tube, flush with its aft end unless `at` places
   it, weighing it there so a part the design can't take is refused where it is added. Each part
   is named by its maker and part number. A catalogue nose or transition states its own diameters;
   the builder's nose no longer always takes the rocket's. `Tube::with_length_m` and
   `MotorTube::with_length_m` cut a tube sold long, and its mass follows. A part of
   the wrong kind, a material the file names but doesn't define, a nose or transition neither
   filled nor given a wall, or a shape the builder doesn't know is `Error::Catalog` with a
   `CatalogProblem`.
2. **What the file leaves unsaid is OpenRocket's choice,** measured by applying every bundled part
   and 6 probe parts to a new OpenRocket 24.12 component (`RocketComponent.loadPreset`, its public
   API, run never read) in `validation/oracles/openrocket/orc_built.py`: an ogive is tangent, a
   parabola's `K′` is 1, a Haack series is von Kármán's (`C` 0), a power series' exponent is ½;
   elliptical, Haack and power-series transitions are clipped, the rest not; no shoulder is
   capped; a filled part's shoulders are solid. The test checks each of these on every part.
3. **But a hollow part's shoulder takes the part's wall** (solid where the wall is thicker than its
   radius). OpenRocket gives it a wall of zero, so it weighs nothing; a molded plastic nose cone's
   shoulder is a tube of the same plastic, and a shoulder of no mass would be a number known to be
   wrong ([rule 1](../../CONTRIBUTING.md#1-correct-physics-first)). The catalogue's stated masses
   side with the wall: on the 74 hollow, shouldered parts that state one, the file's density weighs
   nearer it with the shoulder's wall than without on 46, and the median stated mass is 0.97 of the
   mass with the wall and 1.30 of the mass without.
4. **A stated mass scales the part's density**, so that the part as the catalogue sizes it weighs
   that mass, and the material's name says so. This is what OpenRocket does for a rigid part; for
   a parachute it sets a mass override instead, the same for a part left as it is. A density,
   unlike an override, follows a later change: a tube cut shorter weighs less. OpenRocket leaves a
   streamer's stated mass unused; the one streamer stating one weighs it here.
5. **A part naming an undefined material is refused** (1 part, a nose cone), where OpenRocket
   weighs it as zero. A parachute whose file names no line material, or one it doesn't define, has
   weightless lines, as in OpenRocket, which finds no density for them: 8 parachutes, 6 of them
   stating their mass.
6. **Thresholds, set before measuring:** a nose cone's or transition's mass within 1e-3 of
   OpenRocket's and its centre of mass within 1e-3 of its length; every other part within 1e-12.
   A parachute's or streamer's centre is where it is packed and is not compared. Nor are the
   moments of inertia: the fixture records none, and `hpr_design`'s are checked against hand
   sums on their own (`docs/physics/mass.md`).
7. **`CatalogProblem::Kind` names the kinds as strings** (`found`, `builder`), not as an enum of
   `hpr_io` kinds, so the error type doesn't tie `hpr`'s public API to the reader's enum, which
   may grow.

**Consequences.** M5.5b is met. `crates/hpr/tests/catalog_openrocket.rs` builds all 3,449 parts
through the builder and holds each to `crates/hpr/tests/fixtures/orc/openrocket-built.json`, every
part in one count: 2,230 tubes, couplers, blocks, rings, bulkheads, lugs, parachutes and streamers
within 1e-14 of OpenRocket's mass and their centres within 1e-13 of their length; 1,029 filled
nose cones and transitions within 2.0e-4 in mass and 7.0e-5 of their length in centre; 181 hollow
ones, their hollow shoulders taken out in closed form, within 6.3e-4 and 9.7e-4; 4 hollow ones
whose walls differ (below); in each group, masses stated in ounces apart by OpenRocket's rounded
ounce instead (185); 1 streamer's stated mass; and 4 refused, which OpenRocket weighs as
zero (1 undefined material, 3 tube-like parts with no bore). hpr's 85 filled conical noses equal
the closed form to 1e-12, so their differences are OpenRocket's volumes. The hollow parts' gap is
the two codes' walls, each checked on its own: OpenRocket's 185 hollow parts follow a wall whose
inner radius at each station is `r − t √(1 + r′²)`, integrated in the test, to 1.1e-4 of their
length in centre, and the 111 stating no mass to 2.5e-4 in mass; hpr's wall, every point within
`t` of the surface, equals integrals worked out in the test, in volume and centre, on 113 hollow
nose cones (16 cones, 87 tangent ogives, 10 ellipsoids) to 1e-9. Dropping the slope term from the
station-wise wall fails the test, and so does moving a cone's centre. The 4 whose walls differ by
more than the threshold are short, blunt elliptical nose cones, each with hpr's wall checked:
three up to 0.48% heavier here (the test asserts hpr's is the heavier), one stating its mass
1.0e-3 of its length apart in centre; the largest centre gap is 1.7e-3. Two mutations,
ellipsoid transitions not clipped and a parabola's `K′` of 0.75, each fail the test. `crates/hpr/examples/catalog_rocket.rs` builds LOC Precision's
2.56 in airframe from the catalogue, its fins by hand, and flies it on an AeroTech H170 to
1,119 m. A nominal 29 mm motor doesn't fit LOC's 29 mm motor tube (bore 28.956 mm) under the
design checks, so the example uses the 38 mm tube; that is issue #280. What is checked is each
part's mass as its file describes it: a catalogue's sizes and densities are the makers' or the
database's, and none was weighed here.
