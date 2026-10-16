# ADR-273: The 2026-10-15 review: OpenRocket's supersonic ogive drag as a cause (2026-10-15)

- **Status:** accepted; tightens [M2.3c6][m2-3c6] ([ADR-226][adr-226]); no release moves and no milestone is added
- **Summary:** OpenRocket's issue #3391 (2026-10-14, not yet confirmed by its maintainers) reports that OpenRocket 24.12 gives a tangent-ogive nose almost no pressure drag above Mach 1: about 0.0007 against about 0.13 at Mach 1.5 by the method its own technical documentation describes. HPR Sim found the same gap on a supersonic flight in its collection (#222) and left open which code was right. M2.3c6 splits the apogee gap to OpenRocket into thrust, mass and drag. Its drag part must now give this term on its own line: a public probe sets the two codes' nose pressure drag beside the equivalent-cone value, and the collection's flights past Mach 1 report how much of the gap the term accounts for, with its sign.

[m2-3c6]: ../decisions-and-roadmap.md#m2-3c6
[adr-226]: 0226-the-2026-10-10-review-the-gap-to-openrocket-by-cause.md
[i-222]: https://github.com/nrdptel/fusionspace-eridanus/issues/222
[or-3391]: https://github.com/openrocket/openrocket/issues/3391
[aero]: ../physics/aero.md

**Context.** HPR Sim trails OpenRocket 24.12 on the collection of logged flights: a mean absolute
apogee error of 12.95% against 12.02%. [M2.3c6][m2-3c6] is the milestone that says why, by
swapping OpenRocket's thrust, mass and drag into HPR Sim's flight one at a time ([ADR-226][adr-226]).
On the first supersonic flight in the collection, [#222][i-222] found HPR Sim's supersonic
pressure drag about twice OpenRocket's. Most of that gap was at the nose, a tangent ogive, where
OpenRocket gave almost none well above Mach 1, and at the airfoiled fins, where HPR Sim's was about
four times OpenRocket's. A base-drag rule pulled the other way. Neither code had been checked
against a measurement on that shape, so #222 left the question open.

OpenRocket's [#3391][or-3391], opened on 2026-10-14 and read as filed for this record, reports
which number OpenRocket's own method gives. On a 150 mm tangent ogive on a 50 mm tube (fineness 3),
24.12's component analysis gives the nose a pressure drag coefficient of about 0.0007 at Mach 1.5.
The development branch gives exactly 0. The method in OpenRocket's own technical documentation
gives about 0.13. The reporter traces this to a change that is right below Mach 1 but also feeds
the supersonic part. OpenRocket's maintainers haven't confirmed it yet. The same method's equivalent cone, with half-angle $\varepsilon$ where
$\sin\varepsilon = 1/\sqrt{1 + 4f^2}$ for fineness $f$, gives

$$C_{D,p} = 2.1\sin^2\varepsilon + \frac{0.5\sin\varepsilon}{\sqrt{M^2 - 1}}$$

which at $f = 3$ and $M = 1.5$ is $2.1/37 + 0.5/(\sqrt{37}\sqrt{1.25}) = 0.0568 + 0.0735 = 0.130$.
From Mach 1.3 up, HPR Sim's nose drag uses this expression, with a factor of 1 for a tangent ogive;
between Mach 1 and 1.3 it joins the subsonic value with a cubic ([the aerodynamics page][aero]).
So if the report holds, OpenRocket's drag on a tangent ogive past Mach 1 is low and its apogee high.
This doesn't make HPR Sim right: the term still needs a measurement, as #222 says. It does mean
part of any gap to OpenRocket on these flights has a cause outside HPR Sim, and that part should
be measured, not assumed.

**Decision.** M2.3c6's *done when* gains two requirements. Nothing in it is removed or loosened.

- The fixture report's drag part splits off, as its own line, the pressure drag of
  tangent-ogive noses above Mach 1. It is measured by swapping OpenRocket's per-component nose
  pressure drag, as its public interface reports it, into HPR Sim's flight. The flights are those
  whose HPR Sim flight passes Mach 1 with a tangent-ogive nose (HPR Sim's peak decides, since
  OpenRocket's low nose drag would push its own flights faster). The line gives, as aggregates
  over them, the share of the HPR Sim less OpenRocket apogee gap that the swap accounts for, with
  its sign. Each such flight is counted as swapped or skipped, each skip with its reason, and
  #222's flight is among those swapped or an ADR says why not, so the line can't be met on no
  flights.
- A public probe, an invented rocket with a fineness-3 tangent-ogive nose, records OpenRocket
  24.12's nose pressure drag coefficient (pressure only, friction apart, on the nose's base area)
  at Mach 1.3, 1.5 and 2.0 beside HPR Sim's and the equivalent-cone value above. It cites #3391, and a test pins HPR Sim's value at each Mach number.

These two requirements are the whole change; the roadmap entry states both.

The idea is recorded as promoted in `docs/research/ideas.md`. It adds no user-facing surface.

**Consequences.** The gap to OpenRocket on supersonic flights gets a cited, signed cause in the
reference instead of a guess. If OpenRocket fixes #3391 in its next release, [M2.3c3, OpenRocket's
newest release](../decisions-and-roadmap.md#m2-3c3), flies both versions, and the line shows
how much of the gap the fix closes. The probe records what 24.12 gives beside what its documented
method gives. Whether either code matches a measured ogive's wave drag stays open in #222; a
measured reference is a separate idea (`docs/research/ideas.md`).
