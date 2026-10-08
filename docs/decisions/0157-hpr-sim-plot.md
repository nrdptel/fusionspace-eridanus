# ADR-157: `hpr sim --plot` draws one fixed SVG figure (2026-10-04)

- **Status:** accepted
- **Summary:** M4.5e: `hpr sim --plot FILE.svg` writes one SVG: altitude, speed and acceleration against time in three panels over one time axis, each instant with events a numbered marker listed underneath, and the fall on the airframe alone shaded "not a prediction". It is drawn by hand in `hpr-cli`, with no plotting dependency, from its own sampling of the flight, which adds every step's start and end to the recorder's times so a parachute's opening is drawn at full height

**Context.** [ADR-144](0144-the-2026-10-03-planning-review-leaner-bookkeeping.md) §8e asks that
"`hpr sim --plot` writes an SVG (altitude, speed and acceleration against time, events marked), a
fixed figure set". The recorder's channels hold the nose tip's velocity, not the centre of
gravity's, which the summary's top speed and the events table use. And a recorder samples at
fixed times and at events, where an event's sample is the flight just before the event acts: in
the guide's dual-deploy example, the main's opening shock peaks at 77.5 m/s², and the 0.01 s
samples show 68.7 m/s².

**Decision.**

1. **One figure, always the same set.** `--plot` takes a file name ending in `.svg`, checked with
   the `--export` files before the flight: its folder must exist, and no file is named twice or is
   one the run reads. The figure has three panels over a shared time axis from launch to the
   flight's end: the centre of gravity's height above the site; its speed over the ground and its
   vertical speed (dashed); and the size of the nose tip's acceleration, the summary's peak
   acceleration's quantity, with its vertical part (dashed). No option changes the set, so any
   two flights' figures read alike; data for other plots comes from `--export`.
2. **Events.** Events within half a pixel of each other share a numbered marker above the panels,
   with a dotted line through each panel; markers closer than 20 px stack in rows. A list under
   the figure gives each marker's time and its events, parachutes and streamers by name, wrapped
   to the figure's width.
3. **Not a prediction.** When no recovery device opens within 1 s of apogee (the `hpr sim`
   summary's rule), the span from apogee to the first opening, or to the end, is shaded in every
   panel, labelled, and explained under the list.
4. **Its own sampling.** The figure's trace samples at each `--interval` multiple and each event,
   as the recorder does, and also at the start and end of every integration step, so a jump is
   drawn at its instant. A pixel column keeps its first, least, greatest and last samples, so no
   sampled peak is lost and the file stays near 100 kB.
5. **Drawn in `hpr-cli`, by hand.** SVG text, as the census badges are, with no plotting
   dependency; names are cut to 80 characters and escaped. Colours are the dataviz reference
   palette's first two categorical slots on its light surface, checked by its colour-vision
   validator; the second series is dashed as well, so colour is never the only cue.
6. **The guide's figure is generated.** `cargo xtask cli` draws `docs/images/sim-plot.svg` with
   the command the guide shows, and `--check` fails when it is stale.

**Consequences.** On the dual-deploy example the plotted top speed and opening shock are within
1% of the summary's, which finds peaks inside steps (a test pins both). Of OpenRocket 24.12's 17
examples, run offline over a cache holding the motors already fetched, every configuration
`hpr sim` flies drew a well-formed figure: the 7 files that fly as saved and 2 of the 3
configurations of *Base drag hack*, whose third's motor wasn't in the cache. The other 9 are
refused before the flight, as without `--plot`: 4 for separations and 1 for freeform fins, which
M4.5g works on; 1 for a parallel stage and 1 for dropped rail-button screw heads, later work; and 2
because they fly a hybrid motor, out of scope. The survey is not committed. The figure has no
interactive hover, being a static file; each marker carries a tooltip. A library caller has no
plot: the code lives in the CLI, and moves to a library crate if the planned UI needs it.
