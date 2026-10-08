# ADR-158: Three how-to guides run like the command-line guide, on a rocket of their own, with `--delay` for `--motor` (2026-10-04)

- **Status:** accepted
- **Summary:** M4.5f: the guides "Fly your .ork", "Pick a motor" and "Check stability for a certification flight", and the README's "Install and first flight", show `hpr`'s output made by running each command, as `docs/cli.md` does, so CI fails when one goes stale. They fly a hand-written public design, `validation/fixtures/ork/guides/level-1.ork`, whose two motors are in hpr's catalog, so they run offline. `hpr sim --delay` sets `--motor`'s ejection delay. Guidance numbers are quoted from their sources, dated, never stated as hpr's

**Context.** [ADR-144](0144-the-2026-10-03-planning-review-leaner-bookkeeping.md) §8f asks for three
how-to guides and the README's "Install and first flight" section. A guide that shows a command's
output by hand goes stale when the physics moves. The public `.ork` fixtures name motors outside
hpr's 32 (an H128W, a K550W), so none flies as saved with no network, and CI's examples run
offline over an empty cache. And `--motor` set no delay: on a rocket whose parachute opens on the
motor's charge, every motor tried flew with the parachute shut, so a guide to picking a motor
could not show a delay being picked.

**Decision.**

1. **Generated outputs.** `cargo xtask cli` runs the examples of `README.md` and of the three
   guides (`GUIDES` in `xtask/src/cli.rs`) exactly as it runs `docs/cli.md`'s, between the same
   markers, and `--check` fails when one is stale. A command whose output a guide shows only by
   name, such as one that needs the network, is a plain block with no output.
2. **A rocket for the guides.** `level-1.ork` is hand-written in OpenRocket's 1.10 schema: a
   66 mm phenolic airframe, a 38 mm mount and one parachute on the ejection charge, saved with
   an AeroTech H170M at 10 s (the default) and an I175WS at 9 s, both in the bundled catalog.
   OpenRocket 24.12 opened it with two warnings, both then fixed: a surface finish on the inner
   tube, which it ignores, and the H170M marked single-use where its database lists a reload,
   which dropped that configuration. Its launch mass and margin at rod clearance then agreed with
   hpr's on both configurations. That check was run once by hand and is not committed: it is not
   a validation case, and the guides quote no comparison from it.
3. **`--delay S|P`,** which requires `--motor`, gives that motor its ejection delay, in seconds
   after burnout (0 or more, finite), or plugged; the library's `Motor::with_delay` does the work.
   The configuration's label adds `--delay S`. Without it, nothing changes.
4. **Guidance is quoted, not made.** The stability guide quotes the NAR's and Tripoli's safety
   codes and Level 1 rules, Barrowman's TIR-30, OpenRocket's user guide, NASA's Student Launch
   handbook and the Spaceport America Cup's design guide, each with its revision or the date read
   (2026-10-04), and says the codes name no margin. hpr flags nothing as pass or fail. The trust
   section gives the flattering side: on the private designs, hpr's margin reads up to 0.1108
   calibres above OpenRocket's, and between Mach 0.8 and 1.2 its centre of pressure sits up to
   2.36 calibres behind NASA's wind tunnel's.

**Consequences.** Every output in the guides and the README is what the current `hpr` prints; a
physics change rewrites them with `cargo xtask cli`, and their hand-written prose numbers (the
coast, the margin's worked example) need a reader's eye then. The NAR and Tripoli pages refuse
scripted fetches, so their quotes rest on one reading through a browser-like fetch; Barrowman's
and OpenRocket's were read again from the source. The guides' rocket is not checked against
OpenRocket in CI.
