# ADR-056: A `.ork` design's recovery and separation, read as written, with OpenRocket's words measured (2026-09-21)

- **Status:** accepted
- **Summary:** A `.ork` design's recovery and separation, read as written, with OpenRocket's words measured

**Context.** M3.1c2 reads when each parachute and streamer opens, the drag coefficient it states,
and when each stage separates (ADR-055). OpenRocket's file-format page shows the tags but lists none
of the words they take, and says nothing of whether a deploy height is above the ground or the sea.
ADR-055 read the ignition words with one of them, `ejectioncharge`, resting on a probe that was not
committed, and promised one that was.

**Decision.**

1. **Read, not flown.** `hpr_io::ork::Recovery` holds each device's settings and each stage's
   separation as the file states them. Turning them into `hpr_sim::recovery::Device`s is left to the
   flight that uses them: `hpr-io` does not depend on `hpr-sim` (ARCHITECTURE.md), and two of the
   choices below are that step's to make.
2. **The words are OpenRocket 24.12's own, measured and committed.**
   `validation/oracles/openrocket/events.py` sets every value of the ignition, deployment and
   separation events through OpenRocket's public setters on its own examples, saves, and records
   the word and the label; `validation/fixtures/ork/openrocket-events.json` holds them, and a test
   holds every reader to every word. This settles ADR-055's `ejectioncharge`. A word not listed is
   kept as written.
3. **A setting is an event, a height and a delay, each `None` where the file is silent**, and a
   per-configuration setting replaces them one at a time, as ignition does (ADR-055), so a file
   that leaves one out still reads. That is hpr's reading: OpenRocket was not probed on such a
   file. An empty or unknown event word is kept with a warning.
4. **A deploy height is above the ground** (the launch site; OpenRocket has no terrain). The probe
   flies in calm air with a fixed seed, sets a parachute to open at 30 m on a pad 1,000 m above sea
   level, and it opens at 29.9 m above the ground; a height above the sea would never be reached.
5. **A height the rocket never reaches is recorded, not resolved.** The probe sets the same
   parachute to 100 m on a flight whose apogee is 51.7 m, and in that run OpenRocket never opens
   it, though the flight reaches the ground. One run of one design, not a stated rule.
   `hpr_sim::recovery::Trigger::Altitude` opens at apogee in that case. Which the flight of a
   `.ork` follows is for the step that flies it to decide, in the open.
6. **`<cd>` is kept as the file wrote it**, `auto` or a number (`Dimension`). OpenRocket reports an
   automatic parachute's as 0.8, on the canopy's area, the default its technical documentation
   gives (section 4.2.5) and the probe reads back; an automatic streamer's comes from the strip's
   length and material, on the strip's area (appendix C, equations C.4 and C.5), an estimate the
   documentation puts at about 20% and one user (issue #2031) finds far too small for a 2.5 by
   44 in streamer. The probe records three. Choosing a model is the flight's business, as with the
   height.
7. **A device inside a part hpr does not read is kept apart** (`UnreadDevice`), as a motor is, and
   so is a parallel stage's separation.
8. **The warnings about these settings say so in their path** (`…/deployment`, `…/drag`,
   `…/separation`, shared constants), so they do not count against the airframe when ADR-055's rule
   decides which configurations fly; a test pins both sides.

**Consequences.** On 2026-09-21 the library's 137 parachutes and streamers are read, 2 more left
out inside pod sets; 77 leave their drag to OpenRocket and 60 state it; 18 of its 93 stages state a
separation, and 2 parallel stages' separations are left out. The two open choices, a height above apogee and an automatic drag coefficient, are on
the `.ork` page.
