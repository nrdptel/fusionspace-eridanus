# ADR-106: `hpr sim` flies the library's flight, the stack whole, from a stated launch (2026-09-29)

- **Status:** accepted
- **Summary:** `hpr sim` flies the library's flight, the stack whole, from a stated launch

**Context.** M4.2b asks for a `.ork` or hpr design flown with a catalog motor or a motor file,
its summary printed and its recording exported, and is done when a public `.ork` flown by
`hpr sim` gives the library's flight bit for bit, with its JSON valid. No public `.ork` in the
repository flies with hpr's own motors: the Loft demos and the pod and rod probes name AeroTech
motors the bundled catalog doesn't hold. The library flies a `.ork`'s powered separation only
through `hpr_sim` (`crates/hpr/examples/ork_two_stage.rs`), and flies none of its recovery
devices (ADR-056). A configuration left out of a `.ork` (`LeftOut`) names only the first reason
on `NotFlown`'s list, and a missing curve comes before an incomplete airframe.

**Decision.**

1. **The flight is the facade's.** The design, cut to the configuration flown, becomes
   `hpr::Rocket::from_design`, and `hpr::Flight::builder` flies it. The rail's angles are set only
   when given, so a default launch is the builder's own rail, bit for bit. A recording is a
   `Recorder` of every channel every `--interval` s (0.01 by default) and at each event; it
   samples the steps' dense output, so recording doesn't change the flight.
2. **The launch is stated, and printed.** Without options the site is at 0° N, 0° E and 0 m, the
   rail vertical, frictionless and 1.5 m long (the builder examples' rail), the air calm and the
   1976 standard atmosphere. No site is neutral; every output states the one used, and the guide
   says to set `--elevation` (1,400 m lifts the example's apogee about 8%, a test pins it). A
   `.ork`'s stored launch conditions are not read. `--wind` is one speed at every height; real
   weather is `hpr weather` (M5.2).
3. **The stack flies whole, with no recovery device**, and the notes say so. The fall from apogee
   then runs on small-angle aerodynamics far outside their range, and settles in a tail-first
   glide on the synthetic 54 mm design (145° angle of attack, #241), so the notes, the text's
   landing line and the schema's `landing` all say the landing is not a prediction. The summary
   still carries it, field for field as the library gives it. A peak the fall sets is no
   prediction either (the dual-deploy demo, flown with `--motor H54` and no parachute, is fastest
   as it reaches the ground): each peak carries `after_apogee`, and the text marks such a top
   speed or Mach number "in the fall: not a prediction". The GeoJSON and KML maps get the summary
   without its landings, so no pin reads as a landing place. Recovery and staging: #240.
4. **Refused rather than flown as another rocket:** a `.ork` configuration with a powered
   separation (`MotorConfiguration::staging`); a motor lit at its stage's separation; a `.ork`
   rocket not read exactly as written, asked of the new `hpr_io::ork::airframe_not_as_written`
   (that rule stood inside `ork::design`, where a missing curve hid it); design-check errors
   unless `--accept-design-errors`, which lists them in the notes; a hybrid motor in a `.rse`,
   whose `Type` says so (a `.eng` doesn't say, so a hybrid's `.eng` flies as a solid, and the
   guide says so).
5. **`--motor` puts one motor in one mount, lit at launch**: a catalog name through
   `Motor::from_catalog`, a `.eng` through `Motor::from_eng`, or a `.rse` through the new
   `Motor::from_rse`, which refuses a hybrid by its `Type`. It goes into the chosen
   configuration's one mount, or `--mount`, or the design's only mount; a cluster's tubes and a pod
   set's pods each carry one, and the output counts them. A `LeftOut` names only its first reason,
   so on a `.ork` each reason no motor of the user's own can fix is asked of the file itself:
   the airframe, a powered separation, a stage switched off, a motor in an unread part, more than
   one stage (hpr can't yet tell when they would separate with another motor), or a
   configuration left out for anything but its motor. The same is asked when the file has no
   configuration at all. A configuration with motors in more than one mount is refused.
6. **The output mirrors the library's summary field for field.** `sim.schema.json` holds every
   field of `FlightSummary`, each event's time, height and speed, the file's configurations with
   their names, the motors with their mount's name and count, the exports, the notes, and the
   warnings of the readers and the design's checks. Paths are shown by file name, so an output
   doesn't depend on where it ran. `--export` refuses a file the run reads, a file named twice
   and a missing folder, before the flight. `--interval` is at least 0.001 s: a finer recording
   of a long fall fills memory.
7. **An example may name a file of the repository** (amending ADR-105 §6), given from the root.
   `cargo xtask cli` reads it from the root wherever it runs. It must be of plain path parts
   (no `..`, no root, no drive), a file, and tracked by git, so nothing gitignored under `refs/`
   reaches the page; an output that shows the root's own path is refused.

**Evidence.** `cargo test -p hpr-cli`, 35 tests. `sim_flies_a_public_ork_as_the_library_does`
flies `validation/fixtures/ork/pod-flights/pods-none.ork` (the repository's own probe) with
`--motor H54`. It flies the same rocket through the facade as a program would, and finds every
summary field and every event's time, height and speed equal to the bit (`to_bits`). The
`--export`ed CSV equals the library's `export::csv` byte for byte, and the document validates
against `sim.schema.json`. The same holds for:

- the probe with the H54 written into the file as its own motor;
- the probe stripped of its configuration, flown with `--motor H54`;
- the F15's `.rse` and the H54's `.eng`, with all five export formats written and the Parquet
  equal to the library's;
- the synthetic 54 mm design from Spaceport America's site on an 85° rail in a 5 m/s wind.

Leaning the rail east puts that design's apogee east of where leaning it west does.
`sim_refuses_what_it_cant_fly`, `sim_refuses_a_motor_it_cant_place` and three more tests pin each
refusal above by its message, most on the pod probe edited as text, and the pod probe's two H54s
are counted. The one refusal no test reaches is a motor in a part hpr doesn't read: no public
file has one. `sim_marks_what_the_fall_sets` pins the fall's marks on the dual-deploy demo, and
the motor-file test finds only the path on both maps. `hpr-io`'s
`an_incomplete_airframe_is_found_behind_a_missing_curve` pins the airframe check behind a
`NoCurve`; `hpr`'s motor test pins `from_rse`'s masses and its hybrid refusal.

**Consequences.** `hpr-cli` enables `hpr`'s `parquet` feature, which adds no dependency. The
README's and guide's tables list `hpr sim` as available; its reference is the guide's page,
`docs/cli.md#hpr-sim`. Issues #240 (recovery and staging in `hpr sim`) and #241 (the
no-recovery descent) hold what is left.
