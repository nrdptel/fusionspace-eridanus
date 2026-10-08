# ADR-054: An automatic radius with nothing to take is OpenRocket's default, and a rocket with no stage or component holds no design (2026-09-20)

- **Status:** accepted
- **Summary:** An automatic radius with nothing to take is OpenRocket's default, and a rocket with no stage or component holds no design

**Context.** After M3.1b3, 3 of the 76 readable `.ork` files in the reference library still gave
no rocket that lays out. One, Debrief's `sample-design.ork`, has a `<rocket>` holding a name, a
comment and nothing else; its stored simulation is the whole point of the file. The other two have
a chain of automatic radii with no fixed radius anywhere along it: Loft's `demo-quirks.ork` makes a
nose cone's base, a tube and a transition's forward end automatic, with only the transition's aft
end fixed (the nose caches 0.033 m), and the `openrocket-database` parachute catalogue stacks four
tubes whose radii are all a bare `auto`. `hpr_design`'s neighbour rule ([ADR-007][adr-007]) has
nothing to take in either, so `layout()` refused both. M3.1b4's *done when* asks that each lay out
or be shown to hold no design, and that any rule for such a chain rest on something written down
rather than on a cached number.

**What is written down.** OpenRocket's user guide says only that "the Automatic checkbox will
adjust the dimensions of the component automatically", and its file-format page shows `auto`
without explaining it. The issue tracker says more, in prose:

- A maintainer: when the radius is automatic "and there is no previous or next component to the
  body tube … OR returns the default radius" ([openrocket#1988][or-1988], 2023).
- An open issue: a tube left with nothing to take "reverts to default diameter" ([#1992][or-1992]).
- A user, guessing: the nose cone "may be retaining the default 1.969 in. base diameter"
  ([#871][or-871], 2020), which is 50.0 mm across.
- The change that closed #871 ([PR #998][or-998], 2021) disables the checkbox "when the component
  that you want to get the diameter from already has its auto checkbox checked", or when there is
  none. #1988 shows a later version still leaving one ticked, so the dialogs make such chains rare
  rather than impossible; most come from older or hand-written files. PR #998 also calls the
  number saved beside `auto` the "manual value".

No document gives the default as a number, so it was **measured**.
`validation/oracles/openrocket/automatic_radius.py` runs the OpenRocket 24.12 jar as an external
oracle on fifteen small designs of its own, and with `--library` on the 17 examples inside the jar
and the two library files. It reads each body radius OpenRocket holds when the file is opened,
saves the design (which makes OpenRocket work every radius out again), and reads each radius again.
`validation/fixtures/ork/openrocket-automatic-radius.json` holds the result, with the jar's hash
and the Java and JPype versions:

- Every automatic radius with nothing fixed along its chain settles at **0.025 m**, except a nose
  cone's base or a transition's end that looks at another automatic radius (next bullet but one): a
  lone tube, two tubes, a lone nose cone, a lone transition, and the tube in each longer chain.
- A cached number is ignored: `auto 0.04` and `auto 0.03` both come back 0.025. OpenRocket writes
  back what it resolved, not the number it read (`auto 0.04` is saved as `auto 0.025`).
- Where a nose cone's base or a transition's end looks at another automatic radius, OpenRocket
  settles on **−1 m**, and writes `auto -1.0`. Four shapes show it, 6 radii in all.
- A chain that reaches a fixed radius takes it in OpenRocket as in hpr, in every case probed (up to
  two automatic radii between) and across a stage boundary. The control, a nose before a fixed tube, takes the
  tube's 0.03 m.
- **OpenRocket's first reading is not always its answer.** Where the tube after an automatic tube
  holds a coupler whose own radius is automatic, OpenRocket first reads the automatic tube at its
  default, and settles on the neighbour's radius once it works the design out again. That is what
  happens in its own "Dual parachute deployment" example, whose file caches 0.025 m for that tube;
  M3.1b3 left that cache as a question for this oracle, and the settled answer is hpr's 0.028321 m.

Of the library files, the parachute catalogue settles with its four tubes at 0.025 m, and
`demo-quirks.ork` does not open at all: OpenRocket refuses a `<parallelstage>` directly under
`<rocket>` ("Booster Set not currently compatible with component: Rocket"). Its chain is measured
on the probe's own copy.

**Decision.**

1. **An automatic body radius with no fixed radius anywhere along its chain takes OpenRocket's
   default radius, 25 mm, as a fixed radius, with a warning at its tag naming the radius.** The
   number rests on the maintainers' "default radius" and on the measurement, which the user's
   1.969 in agrees with. It never rests on a cached number, which OpenRocket itself ignores. The rule
   lives in the `.ork` importer (`hpr_io::ork::OPENROCKET_DEFAULT_RADIUS_M`), not in `hpr_design`:
   it is one program's convention, and a design that says nothing about a radius should still be
   refused by `layout()`. `Rocket::unresolvable_body_radii` lists exactly the radii `layout()`
   would refuse, located by stage and index, and `Rocket::fill_unresolvable_body_radii` fills those
   and no others; a property test holds both, and that every radius the neighbour rule reached
   comes out as before.
2. **hpr departs from OpenRocket where OpenRocket answers −1 m.** A negative radius has no
   geometry, so the nose base or transition end that gets it takes the default too, and the chain
   is one radius end to end. `hpr_io`'s test against the fixture holds every other radius to
   OpenRocket's settled answer, bit for bit, and counts the departures: 6 of the 39 radii.
3. **hpr is held to OpenRocket's settled answer, not its first reading.** The first reading
   depends on an unrelated part, and OpenRocket corrects it itself; the test counts the one case
   where the two differ, so another would show.
4. **A tube given the default has its wall judged against it**, by the rule a stated radius gets:
   `filled`, or a wall at least as thick as 25 mm, is solid to 25 mm. Before the radius was known
   the reader had read a `filled` tube as weightless, or as solid to a cached number; those readings
   and their warnings are withdrawn. No corpus file has such a tube; the physics review found the
   gap and a test holds the fix.
5. **A `<rocket>` with no stage or component of any kind holds no design,** and so does a document
   with no `<rocket>`. The reader says so in a warning ending "so the document holds no design", and
   `cargo xtask ork` counts such a document apart from the designs, not as a design that fails to
   lay out. The document is still read whole, and its stored results are M3.1c's to read.
6. **The probe runs OpenRocket through JPype, inside the Python process**
   ([rule 3](../../CONTRIBUTING.md#3-clean-room) permits it). It uses only class and method names
   that `javap` prints for the jar, and binds empty motor and preset databases itself, because the
   graphical providers need a display. This is evidence for M2.2's choice of how to drive the jar
   ([ADR-035][adr-035]), not that choice. It shows the in-process route works headless with Java 17.

**Consequences.** All 75 designs in the reference library lay out, and the 76th document is
counted as holding none. `cargo xtask ork` prints both, and the 7 radii given the default in 2
designs. On the parachute catalogue's four, OpenRocket agrees; on the quirks file's three it can't
be asked, and on the probe's copy of that chain it agrees on the tube and answers −1 m on the other
two. `cargo xtask ork` now also holds every body radius hpr resolves in the 18 designs OpenRocket
opens to OpenRocket's settled answer: **67 of 67 agree**. The in-file oracle's premise, that
`auto 0.0125` is what OpenRocket last worked out, holds for what 24.12 writes, but a cached number
can be a first reading that a save wrote out, and PR #998 says an older release wrote the manual
value; 67 of the 71 cached numbers on components with an `<id>` agree with hpr. The `Rocket` no
longer records that a defaulted radius was automatic; the document the importer keeps still says
`auto`, so a writer working from it loses nothing.

[adr-007]: 0007-design-tree-stations-placement-automatic-radii.md
[adr-035]: 0035-drop-the-orhelper-dependency-how-m2-2-drives.md
[or-871]: https://github.com/openrocket/openrocket/issues/871
[or-998]: https://github.com/openrocket/openrocket/pull/998
[or-1988]: https://github.com/openrocket/openrocket/issues/1988#issuecomment-1397654629
[or-1992]: https://github.com/openrocket/openrocket/issues/1992
