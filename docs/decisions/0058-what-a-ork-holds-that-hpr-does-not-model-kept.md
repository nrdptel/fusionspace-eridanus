# ADR-058: What a `.ork` holds that hpr does not model, kept whole in `x-openrocket` (2026-09-21)

- **Status:** accepted
- **Summary:** What a `.ork` holds that hpr does not model, kept whole in `x-openrocket`

**Context.** M3.1c4 carries the last of M3.1c's *done when*: Loft lesson L66 (Loft dropped pods,
parallel stages and booster sets, and its export lost the note that the rocket was reduced), and "a
document with unknown content round-trips through `extensions.x-openrocket`". hpr's design has no
pods until M1.13, and writing a `.ork` is M3.2; ADR-051 already keeps the whole document in
`OrkFile`.

**Decision.**

1. **`hpr_io::ork::Design` gains `extensions`, with one namespace, `x-openrocket`,** serialized
   under that name. It holds what hpr does not model, each element whole with its path.
2. **Parts:** every child of a `<subcomponents>` under `<rocket>` that the reader (the code that
   walks the tree into `hpr_design` types) did not read — a pod set, a parallel stage, a part left
   out for want of an honest shape, a tag it has never seen. The walk records the path of every
   stage and component it reads, and a child it did not is kept whole. A design with any is
   *reduced* (`Design::is_reduced`), which is derived from the extension rather than stored beside
   it, so an export cannot keep one and lose the other.
3. **Sections:** every child of `<openrocket>` besides the first `<rocket>` and the first
   `<simulations>`, and every child of a stored `<simulation>` that no reader asks for.
4. **Tags:** every child tag no reader asks for in an element hpr does read — the rocket, a stage,
   a part, a stored simulation, and, below them, any tag a reader did ask for — such as a part's
   `<appearance>` or a wind's `<standarddeviation>`. The readers record each tag and attribute they
   ask for, by name and element, while `design()` reads, so one is kept exactly when nothing asked
   for it. A tag asked for only on some paths, such as a shoulder's radius when the shoulder has no
   length, is kept whenever it was not asked; a tag read and then dropped with a warning (a fin's
   fillet) is named in that warning and not kept, since it was asked for.
5. **Attributes:** every attribute no reader asks for on an element hpr does read, with the path of
   that element: a material's `group`, an event's `id`. The survey shows two hpr should read: the
   reference an angle offset or a radius offset is measured from (#145).
6. **A path leads back.** `openrocket/rocket/stage[0]/bodytube[1]/podset[0]` counts each step among
   its parent's `<subcomponents>`, as a warning's path does; a section's step counts among its
   parent's elements, and so does a tag's, marked `@`, at any depth. `hpr_io::ork::element_at`
   follows one, and `cargo xtask ork` fails if a kept element or attribute is not found again.
7. **The round trip is to JSON and back, and to the file.** A test writes the extension out as JSON
   and reads it back unchanged, and finds every kept element at its path in the document it came
   from. That is what an export needs; the export itself is M3.2. An unknown namespace beside
   `x-openrocket` is not read back yet; the design format (M3.3) decides how namespaces travel.
8. **Not kept:** the text of a second copy of a tag a reader takes once by name, since the reading
   is recorded by name and the reader uses the first copy; the second's attributes and unread
   children are kept. 13 fin tabs in the library carry a second `<tabposition>`. The text stays in
   the document `OrkFile` keeps whole (ADR-051), as everything does; M3.2 starts from both.

**Consequences.** On 2026-09-21 the library keeps 17 parts in 10 reduced designs (9 pod sets, 3
parallel stages, and the 5 parts left out), 87 sections, 1,947 tags and 3,132 attributes, and all
5,183 are found again at their paths. With M3.1c1 to M3.1c4, M3.1c's *done when* is met: L57, L64,
L65 and L66's tests are live, a design's stored results are read back, and unknown parts,
sections, tags and attributes round-trip through `x-openrocket`, short of the text of a second
copy of a tag a reader takes once (item 8).
