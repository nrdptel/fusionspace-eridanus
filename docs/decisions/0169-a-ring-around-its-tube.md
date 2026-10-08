# ADR-169: A ring around its tube needs room in the part around it (2026-10-05)

- **Status:** accepted
- **Summary:** M4.5k: a centering ring on its tube's axis whose bore takes the tube's outside wraps the tube, and is measured in the part around it at its own station that holds it whole; amends how two earlier decisions read the pods examples' rings, which now fly as saved

**Context.** M4.5 asks that every in-scope OpenRocket example fly as saved. On 2026-10-05,
`cargo xtask ork-cli` counted 13 of OpenRocket's 56 example configurations that `hpr sim` flew
only with `--accept-design-errors`. Eight of them were the two pods examples. Each draws its
centering rings as children of an 18 mm motor mount, and the fit check
([ADR-155](0155-design-checks-that-match-reality.md)) measured each ring in that mount's bore:

| Example | Ring, outside | Ring, bore | Mount, outside | Mount, bore | What holds the mount, bore |
|---|---:|---:|---:|---:|---:|
| *Pods--airframes and winglets* | 32.54 mm | 18.75 mm | 18.69 mm | 18.03 mm | 32.59 mm (the airframe) |
| *Pods--powered with recovery deployment* | 32.97 mm | 19.33 mm | 19.28 mm | 18.16 mm | 31.45 mm (a coupler) |

*Pods--powered*'s motor tube also runs 34.9 mm past the coupler's fore end, into the booster's
airframe, whose bore is 33.02 mm; one of its two rings sits there.

ADR-166 and ADR-168 read the rings as inside the mount and kept the error. But each ring's bore
is wider than the mount's outside, so the ring can't be inside the mount at all: it is around
it. OpenRocket lets a ring be a child of an inner tube, and the files' authors typed each bore
0.05 mm over the mount's outside. A ring there is glued around the motor tube, as every centering
ring is.

**Decision.**

1. **A ring that wraps its tube.** A centering ring whose parent is an inner tube, that shares the
   tube's axis (the tube is neither a cluster nor off the axis), and whose inner radius is at least
   the tube's outer radius less `LENGTH_TOLERANCE_M`, wraps the tube.
2. **It is measured in the part around it at its own station.** Out from the tube, parent by
   parent, the first part with room for parts (a tube's bore, a nose cone's or transition's
   largest radius) whose span overlaps the ring's is the one around it. If that part holds the
   whole ring (to `LENGTH_TOLERANCE_M`), the ring's radial reach is measured in it, with ADR-155's
   fit tolerance for its bore: `internal_part_tight_in_parent` within the tolerance,
   `internal_part_wider_than_parent` past it, each naming that part. The ring is also checked for
   overlap against the other tubes in that part (`ring_overlaps_inner_tube`). Its span along the
   axis is still checked against its own tube.
3. **Otherwise, as before.** A ring with no part around it that holds it whole (one on a motor
   tube where it sticks out of the airframe, or across the airframe's end) is measured in its
   tube's bore, an error for a ring this wide; it wraps the tube, so it is never a cap at the
   tube's end. A ring whose bore is smaller than the tube's outside, even by 1 µm, is inside the
   tube and measured in its bore as before, a cap at the tube's end included. So is a ring on a
   clustered or off-axis tube, which sits beside the tube, not around it.

**Consequences.**

- Of OpenRocket's 56 example configurations, `hpr sim` flies 46 as saved (38 before), 5 with
  `--accept-design-errors` and refuses 5. Across the survey's 170, 121 fly as saved (108 before):
  the 8 pods configurations and 5 in the private design library.
- The five that still need the flag are *Deployable payload*'s. Its payload and its parachute are
  drawn 25 mm across in a 21 mm bore. Nothing wraps there, so the errors stand; M4.5 still needs a
  decision on them.
- Of *Pods--powered*'s two rings, the one in the airframe fits. The one in the coupler reaches
  the coupler's outside, 0.76 mm past its bore, inside ADR-155's 0.8 mm for a 31 mm bore: it
  warns as a fit to sand.
- A first version of this decision measured a wrapping ring in its tube's holder wherever the ring
  sat. Review found it let a ring on a motor tube's overhang, outside the rocket, pass; it also
  warned for *Pods--powered*'s ring in the airframe as though it were in the coupler. This
  version replaces it.
- The masses don't change: a ring is an annulus about the body axis wherever it hangs in the tree.
- `openrocket-flights` no longer lists design errors for the two pods examples; their flights are
  unchanged.

**Alternatives.**

- *Keep the error* (ADR-166, ADR-168): flags a ring that can't be where it is drawn, when it can.
- *Wrap at the fit tolerance* (a bore up to ADR-155's tolerance under the tube's outside): a press
  fit read as wrapping. No file needs it, and it would let a ring 0.5 mm into a tube's wall pass as
  around the tube, so the round-off tolerance alone is used.
