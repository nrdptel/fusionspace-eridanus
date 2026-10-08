# ADR-089: A pod is a stack of body components repeated around the axis (2026-09-27)

- **Status:** accepted
- **Summary:** A pod is a stack of body components repeated around the axis

**Context.** M1.13 (VISION V19) asks for pods: bodies beside the airframe, as side pods or
outboard motor pods, with their mass properties off the axis, a cited normal force, drag and
interference for each, the `.ork` reader taking them, and a pod design matching OpenRocket. That
is more than one pull request, and the mass is what the rest stands on.

**Decision.**

1. **Split a to c.** M1.13a is the pod's mass, done when a pod's mass properties match the
   hand-computed parallel-axis values (the milestone's first bullet) and what a pod holds is
   repeated in every pod. M1.13b reads `.ork` pod sets. M1.13c is the aerodynamics and the
   OpenRocket comparison (the second bullet), which needs a design in both codes, so it comes
   after the reader.
2. **A `PodSet` part, external, on a body tube**, with `count`, `radial_offset_m` and `angle_rad`,
   placed along its tube by the usual positions. Its children are the pod's body components (nose
   cone, body tubes, transitions): they take no position and stack aft from the pod set's
   position, and their automatic radii resolve among themselves by the stage rule. Anything else
   as a direct child is refused. This is OpenRocket's own layout for a pod set (a component
   assembly of body components), so the reader in M1.13b maps element to element.
3. **One pod, repeated as a rotational pattern.** The pod is laid out on the body's axis; pod `k`
   is it turned by `φ_k = angle + 2π k / count` about the body's axis and moved to
   `r (cos φ_k, sin φ_k)`. The copies a cluster already uses (ADR-075) carry a roll angle for
   this (`Placement`; a cluster's is 0, so its numbers are unchanged bit for bit), and nested
   copies compose. Review found that moving without turning put a one-fin pod's fin inward on the
   far pod and a symmetric pair's centre 20 mm off the axis; turning keeps what a pod holds in the
   same place relative to the airframe on every pod, as a fin set's fins are. Whether OpenRocket
   turns a pod's contents is to be checked with its jar in M1.13b or c. Each copy is weighed where
   it sits, with its parallel-axis term `I_p = I_cg + m (|d|² E − d dᵀ)` (Meriam and Kraige,
   appendix B), and a motor mount in a pod gives a motor per pod. The pod set weighs nothing.
   Overrides follow ADR-075: one on the pod set must cover its pods (`overrides_include_children`)
   and sets their total; one on a part inside a pod is each copy's, a centre `cg_xy_m` measured in
   the pod as written.
4. **Checks.** A pod's body components are not internal parts: they are neither "outside the
   rocket" nor "past their parent's end". A pod set past its tube's end raises no warning, since
   pods hang from pylons and outboard boosters often run past the tube; one that doesn't touch its
   tube at all is still an error.
5. **Refused until M1.13c.** The aerodynamic model refuses a design with pods (`Unsupported`,
   naming the pod set), before a pod's tube could be read as part of the airframe; so does the
   tumble model. A mass shift and a release refuse a pod's body component as a body component,
   and an ejection refuses it as a joint or a payload with its own message, even for a single pod
   whose one copy would otherwise pass. A part inside several pods is several copies and is
   refused as a cluster's is. The nose-base reference diameter takes the airframe's nose, never a
   pod's.
6. **Refused trees.** A pod set in a pod (copies would multiply), an empty pod set, one of more
   than 64 pods, and an override on a pod set that doesn't cover its pods. Geometry checks for
   pods (overlap with the airframe or each other, radius steps, the airframe's extent) are left
   to #206.

**Consequences.** `hpr-design` gains `PodSet`, `Part::PodSet`, `Component::length_m` and
`Placement`; `PlacedComponent::copies_m` becomes `copies` and `contents_copies_m`
`contents_copies`, each a list of placements. The design page's *Pods* section works two pods and
one off the axis by hand; the test pins them to 1e-15 kg, m and kg·m², and parses the page's JSON.
A pod can't fly until M1.13c; a `.ork` file's pods are still kept unread until M1.13b.
