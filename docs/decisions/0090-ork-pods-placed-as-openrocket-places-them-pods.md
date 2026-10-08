# ADR-090: `.ork` pods placed as OpenRocket places them; pods of no length left out (2026-09-27)

- **Status:** accepted; pods of no length and the empty pod set left out superseded by ADR-091
- **Summary:** `.ork` pods placed as OpenRocket places them; pods of no length left out

**Context.** M1.13b reads a `.ork` file's `podset` into `hpr_design::PodSet` (ADR-089). The file
writes a count, a `radiusoffset` and an `angleoffset`, each with a `method`, and no document says
what the offset is measured from. The corpus holds 9 pod sets. Three pods are a tube of no length,
drawn to hang fins off the axis, which `hpr-design` cannot lay out. One pod set holds no body
component at all. One pod ends in a flipped nose cone whose automatic radius cannot resolve while
it is read pointing forward.

**Decision.**

1. **Split b1, b2.** M1.13b1 reads every pod set whose pods are body components with a length, and
   counts all of them. M1.13b2 is the rest: pods of no length, and the empty pod set. It needs fins
   that stand off a tube, or a pod of no length in `hpr-design`, and carries M1.13b's bullet
   unchanged.
2. **The distance from the axis is OpenRocket's.** OpenRocket 24.12 was run as an oracle on eight
   probes (`validation/oracles/openrocket/pods.py`). It puts each pod's axis at `R + ρ + v` for
   `relative`, `R + ρ` for `surface` (the number ignored) and `v` for `free`. Here `R` is the
   tube's outer radius, `v` the number and `ρ` the pod's widest radius: a probe whose widest part
   is neither its first part, its first tube nor its last settles that. An automatic radius in a
   pod can only take one stated in the pod, so `ρ` is the widest stated radius. An unknown method
   is read as `relative`, with a warning. `PodSet` keeps a fixed distance, so an automatic tube
   radius is taken from its cached number, with a warning, as a filled tube's thickness already
   is; with none cached, the pod set is left out. A later change can move the rule into the layout.
3. **The angle is `angleoffset`, read as every angle** (the direction is still assumed, as the
   `.ork` page says). Its `method` changes nothing on the probes, for a pod set on a tube on the axis.
4. **A flipped nose cone is a tail cone**, read as a transition from its base radius, the fore
   radius, to a point. Its shoulder goes on the base. With one end a point, a clipped and an
   unclipped transition are the same shape (`hpr_design::shapes`), so the solid is the nose's,
   turned end for end. This replaces "read pointing forward, with a warning" everywhere; the only
   one in the corpus closes a pod. `cargo xtask ork` maps its cached `aftradius` to `foreradius`.
5. **Left out, with a `Skipped` warning:** a pod set on anything but a body tube, one inside a pod,
   one with no body component, one whose pod has a part of no length, one whose pod radii are all
   automatic, one on a tube whose automatic radius cached nothing, and one whose distance is
   negative. A child of a pod set that is not a body component is left out with its own warning.
   An override on a pod set is its pods' total, as a stage's is, with a warning when the file
   does not say so.

**Consequences.** `cargo xtask ork` prints the pod sets written, read, pods read and the reasons any
were left out: 9, 5, 8 and 4 on 2026-09-27. Cached numbers agree on 71 of 75, the pods' 4 among
them. `hpr_validate`'s tests hold every part in the 9 probes' pods to OpenRocket's place to 1e-15 m.
Mass is within 1.2e-7, the centre within 1.1e-7 and roll inertia within 2e-9. OpenRocket's one
pitch inertia is hpr's about `x_B` within 6.1e-7; with one or two pods hpr's about `y_B` differs
by 0.3% to 1.0%, and hpr's is the true one. The public *Pods--powered with recovery deployment*
now weighs within 1% of OpenRocket, from 12.5% light. Its two configurations that had no reason
beyond "a motor in a part not read" now give the real one in `openrocket-flights.md`.
