# ADR-055: M3.1c split, and the motors a `.ork` flies: its own curve first, and only what lights at launch (2026-09-21)

- **Status:** accepted
- **Summary:** M3.1c split, and the motors a `.ork` flies: its own curve first, and only what lights at launch

**Context.** M3.1c asks for everything a `.ork` holds beyond the airframe: motor configurations and
embedded curves, recovery settings, pods and parallel stages, stored conditions and results, and
`extensions.x-openrocket` for the rest, with Loft lessons L57, L64, L65 and L66. In the reference
library that is 174 motor configurations, 212 `<motor>` elements, 137 parachutes and streamers,
9 `podset` and 3 `parallelstage`, and a stored simulation in most designs (`cargo xtask ork`):
more than one pull request. The reading of each also
rests on what OpenRocket means by its words, which its file-format page ([F]) mostly does not say.

**Decision.**

1. **M3.1c is split into four increments**, each carrying part of M3.1c's *done when* unchanged,
   and M3.1c's own bullet stays unticked until the last is done:
   - M3.1c1, motors and their configurations (L57, L65);
   - M3.1c2, recovery and separation settings;
   - M3.1c3, stored launch conditions and results (L64), carrying "a design's stored results are
     read back";
   - M3.1c4, pods, parallel stages and the rest (L66), carrying "a document with unknown content
     round-trips through `extensions.x-openrocket`".
2. **A configuration is what `<rocket>` declares plus what the mounts put in it.** Each mount's
   `<motor configid>` joins its configuration; a configuration only a mount names is read as one of
   its own, with a warning (L65). A per-configuration `<ignitionconfiguration>` overrides the
   mount's event and delay each on its own, falling back to the mount's for the one it leaves out.
3. **A motor's curve is the archive's `thrustcurves/<digest>.rse` first, then the bundled
   catalog.** The file-format page says OpenRocket itself looks in its motor database first and
   falls back to the embedded curve. hpr takes the embedded one first because the digest "uniquely
   identifies the functional characteristics" of a curve (OpenRocket's GitHub wiki, file format
   1.2): it is exactly the curve the design was saved with, while hpr's bundled catalog is 32
   motors chosen for validation, not OpenRocket's database. The cost is that on a file carrying
   curves, hpr and OpenRocket can fly different curves for the same motor: the file specification
   says OpenRocket prefers its database's, which "may have more accurate or updated data". The
   motor is built from the curve's header, as a catalog motor is, and the mass and centre of
   gravity listed beside each thrust point are not used; a case size that differs from the
   design's by more than a millimetre is warned about. The catalog match is on manufacturer
   (its full name or abbreviation) and designation, compared without case, spaces or hyphens,
   because both are needed: the library's Estes `B4` is not the catalog's Quest `B4`. Two matches
   are refused as ambiguous. A hybrid is never given a curve, whether the design or its embedded
   curve says so: the project flies commercial solid motors only. A motor with no curve is kept
   with its reason, and nothing is invented for it.
4. **`<delay>none</delay>` is a plugged motor, and `0` a charge at burnout.** OpenRocket's
   technical documentation defines the delay as the time "between the motor burnout and the
   ignition of the ejection charge", which "can also be replaced by 'P', which stands for
   plugged" (p. 8), and describes "zero-delay motors, that ignite the ejection charge immediately
   at burnout" (p. 10); issue #2002 calls typing `none` into the delay box the way to say plugged.
   OpenRocket flies a `0` that way: its "Parallel booster staging" example stores the E12-0's
   burnout and ejection charge at the same 2.44 s. Older designs may mean plugged by it — until
   23.09 added *plugged* to the delay list (#2090), OpenRocket's own examples used `0` so (#2111) —
   and hpr reads it as the file says, as OpenRocket does. `hpr_motor::Delay::Seconds` now documents
   zero as a value a file may state plainly.
5. **The ignition words are OpenRocket 24.12's own**: `automatic`, `launch`, `ejectioncharge`,
   `burnout` and `never`. The library's files use all but `ejectioncharge`, whose spelling was
   measured by setting it through OpenRocket's public setters and saving, in a probe not committed;
   M3.1c2 commits one for every event word. `automatic` lights "the lowest stage … at launch" and each stage above at "the ejection
   charge of the lower stage" (OpenRocket's FAQ, "How do I create a staged rocket?"). An unknown
   word is kept as written.
6. **The rocket flies a configuration only when it can be flown as written.** Every motor must be
   read, have a curve and a case size, sit in a mount read as the single tube it is, and light at
   launch — `launch`, or `automatic` in the bottom stage (the last in the file), at no delay — and no stage may be
   switched off (OpenRocket removes an inactive stage from the flight: release notes 22.02.beta.05,
   PR #1478). A mount holding two motors for one configuration keeps it out too, since which
   OpenRocket would fly is not known. Two more rules are about the rocket rather than its motors:
   the rocket and its motor mounts must have been read without a single warning — nothing left
   out (a pod, a parallel stage, a part hpr cannot shape), dropped or simplified (a cluster of tubes
   read as one, a flipped nose cone read forward, a material it could not read) or assumed (a
   shoulder of no wall read as solid) — and the rocket must have one stage (OpenRocket drops a booster when it separates, and hpr would
   carry it to the ground until M3.1c2 reads separation and M1.9 flies it). Only those
   configurations become `hpr_design::Rocket::configurations`. The reason is that
   `hpr_design::Configuration` lights every motor at `t = 0` until [M1.9] brings staging: flying a
   two-stage configuration there would light the sustainer on the pad. Every other configuration
   stays in `hpr_io::ork::Motors` with its first reason (`NotFlown`).
7. **`hpr-io` depends on `hpr-motor`**, for the curve reader and the catalog; both are pure crates
   that build for `wasm32-unknown-unknown`, so the pure core is unchanged. `ARCHITECTURE.md`'s crate
   map says so.
8. **`cargo xtask ork` counts the motors and fails if a configuration the importer passed as
   flyable does not assemble.**

**Consequences.**

- On 2026-09-21, the library reads 206 motors into 174 configurations and leaves 6 out, 4 inside
  pod sets and 2 inside parallel stages. 4 motors have an embedded curve, all in the library's one
  schema-1.11 file, a two-stage design, so none of them flies yet; 2 have a bundled one; 3 are
  hybrids and 197 have no curve hpr holds. So 1 of the 174 configurations flies, and it assembles;
  1 more is held back only by its airframe's warnings, shoulders of no wall read as solid, which
  M2.2's oracle is to settle. The catalog, not the reader, is the main limit, until [M5.1]'s
  online layer and cache bring ThrustCurve.org's curves.
- A cluster mount, read as one tube since M3.1b3, keeps its configurations out rather than flying
  one motor where the file has several.
- Recovery, separation, stored results, pods and `extensions.x-openrocket` are M3.1c2 to M3.1c4.

[F]: https://openrocket.readthedocs.io/en/latest/dev_guide/file_specification.html
[M1.9]: https://nrdptel.github.io/hpr-sim/decisions-and-roadmap.html#m1-9
[M5.1]: https://nrdptel.github.io/hpr-sim/decisions-and-roadmap.html#m5-1
