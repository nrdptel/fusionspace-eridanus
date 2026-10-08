# ADR-053: The parts on and inside a `.ork` body: degrees, what is left out, and a sourced finish (2026-09-20)

- **Status:** accepted
- **Summary:** The parts on and inside a `.ork` body: degrees, what is left out, and a sourced finish

**Context.** M3.1b2 read a design's spine. M3.1b3 reads what hangs off it — the tubes, couplers,
rings and bulkheads inside a body component, the fins, tube fins, lugs and rail buttons on it, and
the mass objects and recovery gear packed in it — which is 765 parts across the reference corpus
against the spine's 285. Five things had to be settled first, and [ADR-052][adr-052] left one of
them open on purpose: what the older of two names for a radial offset is measured from.

**Decision.**

1. **Angles in a `.ork` are degrees.** Nothing in the file says so and every length beside them is
   in metres, so it is a mistake waiting to happen: read as radians,
   `<angleoffset>180</angleoffset>` is more than twenty-eight turns instead of half of one. The
   corpus settles it — of the 993 angles written in it, 188 are not zero and **178 of those are
   larger than 2π**, more than a whole turn — and the values themselves are 180, 90, 45, 30 and
   120, carrying the float dust (`119.99999999999999`) of a conversion that went through radians
   and back. `cargo xtask ork` prints the three counts and
   `hpr_io::ork::tests::angles_are_degrees_not_radians` holds the reading.
2. **`radialposition` and `radiusoffset` are read, each on the parts that carry it, and never one
   as the other.** ADR-052 left both unread for want of a source saying what the older name's
   frame is. That question does not have to be answered to read them, because the corpus shows the
   two never meet: `radialposition` (542 elements) is written on the parts *inside* a body — inner
   tubes, couplers, rings, mass objects, recovery gear — and `radiusoffset` (106) on the parts
   *on* it (fins, tube fins and rail buttons: 95) and on the 11 pods and parallel stages, and no
   element carries both. They are two tags on
   different components, not two names for one. The same goes for the angle: `angleoffset` and the
   older `rotation` or `radialdirection` are read as one number because on all 121 elements that
   carry both they agree on it, and the frames they differ on — `relative` to the parent against
   `fixed` in the rocket — are the same angle for every parent this reader builds, all of which
   sit on the rocket's own axis. A pod set does not, and a pod set is M3.1c's work.
3. **A part this reader cannot give an honest shape is left out with its reason, never guessed.**
   Four rules, five elements in the corpus: an external part on anything but a body tube (2 fin
   sets on a nose cone, whose root is not a straight line); a freeform outline that does not end on
   the root (the same 2, which would have to be closed along a root they never touch); a tube fin
   set whose radius OpenRocket sizes from the body, which hpr has no rule for (2, [#133][issue]);
   and a part whose automatic radius needs a bore its parent has not got (1 coupler in a nose
   cone). The design still opens and still lays out; the warning names the part and the reason, and
   `cargo xtask ork` counts them.
4. **A tube of no wall thickness carries no mass**, which is the opposite of the rule the spine
   gives a body component and a shoulder, and deliberately so. A body component has a `filled`
   spelling in the file, so a zero there is ambiguous and is read as solid; an inner tube has no
   such spelling, and OpenRocket's own geometry makes a tube's bore its outer radius less its wall,
   so a wall of nothing is a part of nothing. Reading those 12 elements as solid would invent the
   mass instead — a solid coupler filling a 50 mm airframe for 180 mm is a few hundred grams two of
   OpenRocket's own example designs never had. `hpr_design::parts::hollow_cylinder` therefore
   accepts a zero wall and answers zero mass, which its formula already did.
5. **An inner tube's automatic outer radius is its parent's bore**, which is the rule a centering
   ring already had (`AutoDimension::OuterRadius`, extended to `InnerTube`): 37 couplers and engine
   blocks in the corpus need it. Automatic **outer** radii resolve in a pass of their own, before
   any ring's automatic **bore**, because a ring's bore reads its siblings' outer radii — resolving
   both in one pass would give a ring whose bore depended on whether the tube inside it was written
   first. That is [Loft lesson L60][l60], and it is why `hpr_design`'s resolution is two passes.
6. **The five surface-finish words take the roughness heights OpenRocket's author published**: 500,
   150, 60, 20 and 2 µm for `rough`, `unfinished`, `normal`, `smooth` and `polished`. The file
   format documentation gives none of them. The default is confirmed twice over — the [OpenRocket
   technical documentation][techdoc] section 6 says regular paint "corresponds to an average
   surface roughness of 60 µm", and the user guide's body-tube dialog reads "Regular paint
   (2.36 mil)", which is 59.9 µm — and the other four come from the program's author on The
   Rocketry Forum. Each is a `Finish::Custom` height rather than one of hpr's named finishes, whose
   names mean other surfaces. **`polished` is not settled:** 2 µm rests on that post alone, and the
   technical documentation's own Table 3.2 puts 2 µm at *aircraft sheet metal* while "finished and
   polished surface" is 0.5 µm — so OpenRocket's label does not match the row its name points at,
   and a later version could have moved it (forum posts from 2023 list nine finishes, not five,
   though no release note mentions them). It would be worth 1.32× in skin friction on the 14
   components that say it. No file in the corpus writes any word but the five. An unknown word
   takes hpr's default and says so.
7. **Which way an angle turns is assumed, and said to be assumed.** hpr measures a roll angle
   right-handed about an axis pointing at the nose; the OpenRocket technical documentation §3.1.4
   points its own `x` along the centreline *aft* and leaves the other two axes unstated. If that is
   what it means, every angle read here is mirrored — a mass object at 90° on the other side, a
   canted fin set rolling the other way. No file can settle it, because a mirrored design is still
   a valid design, so it goes on the guide's list of readings that are not settled, for one
   asymmetric design through the M2.2 oracle to decide.
8. **Whether an override covers the parts inside a component is taken from the mass flag.** A
   `.ork` says it once per quantity and `hpr-design` says it once for the component, so the two
   cannot always agree; mass is the quantity the flag is written for (95 of the 104 in the corpus),
   and a centre-of-gravity flag that disagrees raises a warning. Until this milestone the spine
   read it as never covering the children, which was harmless while no component had any.
9. **M3.1b splits once more.** M3.1b3 is the parts; M3.1b4 is the three designs of the 76 that
   still do not lay out, none of which this milestone could reach: one document holds no rocket at
   all, and two have a chain of automatic radii with no fixed radius anywhere to resolve against.

**Consequences.** 765 parts read across 73 designs that lay out, and every claim above is a count
`cargo xtask ork` prints. *Some* of the resolution rules now have an oracle that needs no
OpenRocket: `auto 0.0125` is the answer OpenRocket itself last worked out, so the layout can be
held to it — 67 of the 71 cached dimensions on a component the file gives an id agree to a part in
10⁹. It reaches less far than that sounds: OpenRocket never caches a number for `outerradius`
(0 of 131) or `innerradius` (0 of 80), so **the two rules point 5 adds have no oracle coverage at
all**, and rest on their tests and on the argument for them. `cargo xtask ork` prints the per-tag
denominators and names the tags nothing reaches. The four that
do not are one body tube and the parachute packed inside it, in one design the corpus holds twice,
and **that file's caches contradict each other**: the nose cone ahead of that tube caches
0.028321 m, and a nose cone's automatic base radius *is* the radius of the component behind it,
while the tube itself caches 0.025 m. hpr resolves it to 0.028321 m, with the design's three other
cached radii and its one stated radius against the single odd one. That is an argument from the
file, not a proof — it shows the cache is inconsistent, not which half went stale, and which one
OpenRocket would compute today is for M2.2 to settle. It is ADR-052's point about a cached value,
made checkable.

Point 4 is a reading OpenRocket could settle, and M2.2 will: it is listed with the spine's two
open readings on the guide's `.ork` page. Point 6's `polished` is the one number here that a newer
OpenRocket may have moved; it is written down as unsettled rather than left to be discovered. Fin
fillets, the screw head on a rail button, motor clusters and instanced rings are read as the
simpler part hpr models, each with a warning, so what is missing from a mass is never silent.

[adr-052]: 0052-what-a-ork-value-means-automatic-dimensions-two.md
[l60]: https://github.com/nrdptel/hpr-sim/blob/main/docs/research/loft-lessons.md
[issue]: https://github.com/nrdptel/hpr-sim/issues/133
[techdoc]: https://openrocket.sourceforge.net/techdoc.pdf
