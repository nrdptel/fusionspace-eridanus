# OpenRocket `.ork` design files

A `.ork` file is an OpenRocket design: the tree of parts a rocket is built from, the materials they
are made of, the motors flown in it, and the results of the simulations OpenRocket last ran. It is
the format hobby designs are most often shared in, so reading it is how a design gets into HPR Sim
without being typed again.

**What works today:** HPR Sim reads the three `.ork` container forms and turns the XML into a design tree.
It keeps what it doesn't model, and writes a design back out as a `.ork` that reads back as the
same design ([writing a `.ork` back out](#writing-a-ork-back-out)). OpenRocket 24.12 opens each
written file whose original it opens, and flies it to the original's apogee within 0.5%
([checked in OpenRocket](#checked-in-openrocket)). Layout is normalized; comments, processing instructions
and XML namespaces are dropped.

It currently builds supported stages, body components, tubes, rings, fins, lugs, rail buttons,
recovery gear, motor configurations, recovery settings and stored simulations. Staged, clustered
and air-start configurations fly, with each stage separation under power, one or several in turn
([staged, clustered and air-start flights](#staged-clustered-and-air-start-flights)). So does a
payload dropped with nothing left to burn, when its own device opens at the split, and so do
boosters strapped beside the core on a rocket of one stage on the axis
([Parallel stages](#parallel-stages)). Pods, unsupported shapes and recovery behavior are not
fully modeled; the trust limits below
are measured against OpenRocket 24.12 and the current reference corpus. A Rust program reads a
file with `hpr_io::ork`. From a terminal, [`hpr sim`](../cli.md#hpr-sim) flies one, with its
parachutes and streamers. It refuses what it can't fly as written, such as a payload that would
coast with no drag after its split.

> **How far to trust it.** The design is read from the file, and its flight is simulated; each is
> checked below.
>
> - **The shape is cross-checked.** The airframe's key geometry was compared with a second program
>   that reads `.ork` files, RocketSerializer, and with OpenRocket itself. That geometry is the nose
>   cone, the transitions, the fin sets, where each sits, and the body radius. Over 71 designs in the current scratch-excluding survey, HPR Sim's value is within the survey's 1-in-10⁹ comparison tolerance of
>   OpenRocket's for all 1,171 numbers. The survey found 75 files, 73 readable and 72 with a design
>   that lays out; four readable designs are not opened by OpenRocket. Where parts sit is checked against OpenRocket alone; mass
>   and the center of gravity are checked in [Mass properties](../physics/mass.md#checked-against-openrocket)
>   ([checked against RocketSerializer](#checked-against-rocketserializer)).
> - **Few motor configurations fly with HPR Sim alone.** Motors are read, but a configuration flies
>   only when every motor in it has a thrust curve and lights at a moment HPR Sim can fly, its stages
>   come apart in a way HPR Sim flies, and the airframe was read without a warning. Most designs don't
>   carry their curves. With the file's own curves and HPR Sim's small bundled catalog, **4 of the 170
>   motor configurations** in the reference library's 72 designs fly. When a caller also supplies
>   OpenRocket's own motor database, as the validation survey does, **145** fly. HPR Sim doesn't ship that database
>   ([motors in the reference library](#motors-in-the-reference-library)). `hpr sim` fetches a
>   missing curve from ThrustCurve.org by the file's manufacturer and designation, and caches it
>   ([motors from ThrustCurve.org](../cli.md#motors-from-thrustcurveorg)). With those curves
>   `hpr sim` flies **136** of the 170 as saved, and 9 more with `--accept-design-errors`
>   ([the count](#how-many-configurations-hpr-sim-flies)).
> - **Parachutes and streamers fly as OpenRocket flies them.** HPR Sim lands within 0.02% of
>   OpenRocket's landing speed on 50 of its 53 example flights and within 0.12% on all 53. On the
>   51 with no named cause for an apogee gap, less the *Base drag hack*'s three, its flight time is
>   −1.57% to +3.16% off
>   ([flown as OpenRocket flies them](#flown-as-openrocket-flies-them)). A stage
>   separation is flown when a motor ahead of it is still burning or yet to light at that moment and
>   none behind it is, and a configuration's several such separations are flown in turn, from the
>   tail forward. One at a stage's first burnout may drop the stage's other motors still burning,
>   as OpenRocket does
>   ([when parachutes open and stages separate](#when-parachutes-open-and-stages-separate)). A lone
>   separation with nothing left to burn, such as a payload's, is flown when the part that keeps the
>   nose has a device of its own open by then
>   ([ADR-165](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0165-an-unpowered-separation-in-hpr-sim.md),
>   a payload's split). A Rust
>   program and `hpr sim` fly such powered separations, each device on the part its stage is in
>   ([M4.5g1](../decisions-and-roadmap.md#m4-5g1), powered separation in `hpr sim`;
>   [M4.5g2](../decisions-and-roadmap.md#m4-5g2), several separations); a dropped stage's own
>   flight is not validated. `hpr sim` flies a configuration whose separation can
>   only come after apogee as one stack, with a note.
> - **Whole flights are compared with OpenRocket's on its own examples.** On 41 of the 53
>   configurations of OpenRocket's examples that both programs fly to the end, the stability margin off the rod agrees
>   within 0.016 calibres, taken with the air along the rocket's axis as OpenRocket's is. On the *Tube fin rocket* it is 1.08 calibres below OpenRocket's
>   ([tube fins](../physics/aero.md#tube-fins)), and on the three-stage example's three
>   configurations 0.039 to 0.058 calibres below. On the *Pods--airframes and winglets* example's
>   five it is 0.071 to 0.076 calibres above, the flattering side
>   ([#325](https://github.com/nrdptel/fusionspace-eridanus/issues/325), [#326](https://github.com/nrdptel/fusionspace-eridanus/issues/326)),
>   and on the three of *Pods--powered with recovery deployment* 0.070 calibres above. Where nothing named explains a difference, HPR Sim's
>   apogee is from 4.34% low to 2.06% high. Parachutes that open while the rocket still climbs move
>   six apogees by more than 5%, and HPR Sim's tube-fin drag a seventh. The eighth, 37.93% low, is a
>   pod example's sustainer that turns over before apogee in both programs in the conditions of
>   OpenRocket's record. OpenRocket aborts its own flight of that example's first configuration,
>   so HPR Sim's is compared at two points up to the abort, weaker evidence than an apogee: at
>   OpenRocket's last row, 1.81 s, its height is 1.18% below OpenRocket's and its speed 4.02% above
>   ([flights OpenRocket aborted](#flights-openrocket-aborted)). Two of the six, the *Base drag
>   hack* example on a D12-3 and an E12-4, stay more than 5% high with the parachute held, which
>   HPR Sim's drag coefficient explains
>   ([a part set to no drag](#a-part-set-to-no-drag)). A two-stage design, a three-stage design, a cluster, an air start
>   and boosters beside the core are each within 5% of OpenRocket's apogee and largest speed. Five of their apogees are compared
>   with OpenRocket's flight with no parachute, since its parachute opened before apogee
>   ([HPR Sim's flights against OpenRocket's](#the-simulators-flights-against-openrockets)).
> - **One private flight is far off, and it is the supersonic one.** On the private designs, one
>   apogee is more than 5% from OpenRocket's: `C06/1` (a private design's first motor configuration,
>   under an [anonymised id](#the-simulators-flights-of-the-private-designs)), the only supersonic
>   flight, +13.60%. Flown on
>   OpenRocket's own drag it reads +1.11%, so its cause is in the drag. Which drag is right is open
>   ([#222: HPR Sim's supersonic pressure drag and base drag under power](https://github.com/nrdptel/fusionspace-eridanus/issues/222),
>   [a supersonic flight](#a-supersonic-flight-and-a-cause-in-the-drag)).
> - **Pods are read, weighed and flown** ([Pods](#pods)). Five small probe designs with pods of
>   bodies, fins, a tail cone, winglets or motors are compared with OpenRocket, and so is
>   OpenRocket's *Pods--airframes and winglets*, whose stability margin reads 0.07 calibres above
>   OpenRocket's, the flattering side ([below](#fins-on-a-nose-cone-or-a-transition)).
> - **Parallel stages are read and flown on a rocket of one stage on the axis** ([Parallel
>   stages](#parallel-stages)). OpenRocket's *Parallel booster staging* flies both its
>   configurations within 1.3% of OpenRocket's apogee. A parallel stage HPR Sim can't read, such as
>   one on a rocket of several stages, is kept whole and the design marked *reduced*
>   ([what HPR Sim keeps](#what-the-simulator-keeps-for-writing-the-file-back)).
> - **Some parts are left out.** A part HPR Sim cannot give an honest shape, such as a launch lug on a
>   nose cone, is left out. Each one is named in a warning rather than guessed at
>   ([what is left out, and why](#what-is-left-out-and-why)). None in the reference library is.
> - **Fins on a nose cone or a transition are read with their root along its surface**
>   ([Fins on a nose cone or a transition](#fins-on-a-nose-cone-or-a-transition)).
> - **Tube fins are read, weighed and flown**, each tube as a ring wing
>   ([Tube fins sized from the body](#tube-fins-sized-from-the-body);
>   [the aerodynamics](../physics/aero.md#tube-fins)). OpenRocket's example flies 6.95% higher in
>   HPR Sim than in OpenRocket, a net gap in the drag.
> - **Every one of the 72 designs in the current reference survey lays out**, meaning every part gets a
>   position and a radius. A radius the file leaves with nothing to be worked out from gets OpenRocket's own
>   default of 25 mm, with a warning
>   ([when an automatic radius has nothing to take](#when-an-automatic-radius-has-nothing-to-take)).
> - **A written file flies in OpenRocket as its original does.** Of 151 configurations OpenRocket
>   flies both ways, all 151 reach the original's apogee within 0.5%, and 142 exactly. This checks
>   the file, not HPR Sim's physics ([checked in OpenRocket](#checked-in-openrocket)).

## Opening a file today

This is the doctest on `hpr_io::ork`, which CI runs. It stops at the document: the tree of
elements the file said, with nothing interpreted. To go one step further and get an
`hpr_design::Rocket` out of that tree, see
[opening a design, in full](#opening-a-design-in-full).

```rust
let xml = br#"<?xml version="1.0" encoding="UTF-8"?>
<openrocket version="1.10" creator="OpenRocket 24.12">
  <rocket><name>Sounder</name></rocket>
</openrocket>"#;

let read = hpr_io::ork::read(xml)?;
assert_eq!(read.value.container, hpr_io::ork::Container::Xml);
assert_eq!(read.value.document.version.to_string(), "1.10");
let rocket = read.value.document.root.child("rocket").expect("a rocket");
assert_eq!(rocket.child("name").expect("a name").text(), "Sounder");
assert!(read.warnings.is_empty());
```

`read` takes bytes, not a path: `hpr-io` does no file I/O, so that it can also run in a browser.
Hand it the bytes of a real `.ork` and the three containers are all the same call. What comes back
is an `OrkFile` (the container, the document, and the archive's other entries) beside the
warnings the read raised.

Code: `hpr_io::ork` ([API reference](../api/hpr_io/ork/index.html)), written for
[M3.1a](../decisions-and-roadmap.md#m3-1a). Rules below are from the format documentation unless
marked **Observed** (seen in real files) or **Policy** (HPR Sim's own choice).

## Sources

- **[F]** OpenRocket file format documentation,
  <https://openrocket.readthedocs.io/en/latest/dev_guide/file_specification.html>, and the
  `fileformat.txt` shipped with the program. There is **no XSD**: nothing machine-checkable
  describes a `.ork`, and every OpenRocket release has added tags.
- **Observed:** 75 `.ork` files, all cached under `refs/` and never committed: 27 from the private
  design corpus, 20 hand-authored fixtures from [Loft][loft] and 1 from Debrief (this project's two
  predecessors), 17 example designs inside the pinned `OpenRocket-24.12.jar`, 9 from the
  `openrocket-database` parts library, and 1 cached elsewhere. Generated files under
  `refs/scratch/` are excluded from the default survey. By the files' own `creator` attribute,
  55 of the 73 readable files were written by OpenRocket and 18 were hand-authored, so where a
  count says what a real OpenRocket writes it is given over those 55. Every count below is printed
  by `cargo xtask ork` unless it names another source.
- OpenRocket's own Java source is **not** consulted: it is GPL, and this project is MIT OR
  Apache-2.0 ([ADR-051][adr-051]): it is built from published documentation and real
  files, never from another program's source, which is what "clean room" means here.

## The three containers

The first bytes say which one a file is in ([F]; [Loft lesson L56](../decisions-and-roadmap.md#l56): Loft told them apart by their magic
bytes, and a malformed file had to give an error rather than crash):

| container | first bytes | what it holds |
|---|---|---|
| zip | `50 4b 03 04` (`PK␃␄`) | an entry called `rocket.ork` (the design as XML) beside anything else the design carries |
| gzip | `1f 8b` | the design document, compressed on its own. What older OpenRocket versions wrote |
| XML | `<`, after an optional byte-order mark and blank lines | the design document itself |

**Observed:** of the 73 files that open, 71 are zip and 2 are plain XML. No gzip `.ork` survives in
the current scratch-excluding survey, so that path is held by a test rather than by a real file.

**Policy.** Inside a zip, the design is the entry named `rocket.ork`. If there is none, the first
entry whose name ends in `.ork` or `.xml` is read as the design and a warning says so. Every other
entry is kept byte for byte as an *attachment*: `thrustcurves/*.rse` motor curves (schema 1.11),
`preview.png`, lookup tables. **Observed:** 38 `.png`, 11 `.jpg`, 3 `.gif` and 3 `.rse`
attachments across the corpus, but all three `.rse` curves come from the single schema 1.11 file,
so the 1.11 attachments [M3.1c](../decisions-and-roadmap.md#m3-1c) will read
([Loft lesson L57](../decisions-and-roadmap.md#l57): Loft threw them away) are a sample of one.
Nothing reads them yet; they are kept so that
[M3.1c](../decisions-and-roadmap.md#m3-1c) can, and so an export can put them back.

## The schema version

The root element carries it: `<openrocket version="1.10" creator="OpenRocket 24.12">`. Version 1.9
is OpenRocket 23.09, 1.10 is 24.12, and 1.11 (26.xx, documented but not yet released) adds embedded
`.rse` curves, CSV lookup tables, a gravity model and `preview.png`.

**Observed**, across the corpus:

| schema | files | written by |
|---|---|---|
| 1.4 | 4 | OpenRocket 13.05, 15.03 |
| 1.5 | 10 | OpenRocket 15.03 |
| 1.8 | 5 | OpenRocket 22.02 |
| 1.9 | 3 | OpenRocket 23.09 |
| 1.10 | 53 | OpenRocket 24.12, and hand-written fixtures |
| 1.11 | 1 | an OpenRocket 26.xx snapshot |

**Policy.** A version past 1.11 is read anyway, with a warning: a newer file is mostly an older
file with tags added, and refusing it outright would help nobody. A version that is not
`major.minor` is an error.

## The document

The design document is read into a tree of elements and text and nothing is interpreted. That is
deliberate: with no schema to check against, the only way to be sure a later step has not quietly
dropped something is to keep the whole file and be able to write it back.

**Policy**, and what the tree keeps:

- Every element, in document order, with its attributes in the order they were written.
- The text of a leaf element exactly as written, including leading and trailing spaces:
  `<name> Sounder </name>` is not the same as `<name>Sounder</name>` until something decides to
  trim it.
- Text an element holds *beside* child elements. **Observed:** OpenRocket writes this, and only
  here; a simulation's `<warning>` prints its own message after its fields, in 48 elements of 19
  files, and `cargo xtask ork` finds no other tag that does it.
- The blank text that only lays the file out (the indentation between child elements) is **not**
  kept, so that writing the document out again is free to lay it out afresh.
- An XML comment or processing instruction is dropped, and counted as a warning. No `.ork` tag
  carries meaning in one. A comment, a processing instruction or a CDATA section splits a run of
  text in two as it is parsed; the pieces are joined back, because dropping the comment leaves the
  writer nowhere to put the split.

The one thing the tree does **not** keep is XML namespaces: a prefix and its declaration are
dropped, and two attributes differing only by prefix become one. No `.ork` OpenRocket writes uses
them, and a document that declares any says so in a warning.

Writing the document back out uses one fixed style (not W3C Canonical XML, which is a different
thing): two-space indentation, `<tag/>` for an empty element, and character references for the characters that would otherwise change when read again
(a carriage return in text; a tab, newline or return in an attribute). **Reading a document,
writing it, and reading it again gives the same document**. That is what "keeps everything" means
here. **Invariant (tested):**
`parse(to_xml(parse(x))) == parse(x)`: by `hpr_io::ork::tests::any_document_written_and_read_again_is_unchanged`
over generated trees, by `reading_writing_and_reading_again_gives_the_same_document` and
`awkward_text_and_attributes_survive_being_written` on awkward text, and by `cargo xtask ork` over
every real file in the corpus.

## The values inside the tags

A component's numbers sit in leaf elements, and three things about them are not obvious. All three
are mistakes [Loft][loft] made, and each is settled here by what the corpus shows rather than by a
specification, because `.ork` has none. Code: `hpr_io::ork::value`
([API reference](../api/hpr_io/ork/value/index.html)), decided in [ADR-052][adr-052].

**A dimension may be automatic.** `<aftradius>auto 0.025</aftradius>` means "OpenRocket works this
out from the neighbouring components, and 0.025 m is what it last worked out" (so OpenRocket 24.12
does; older releases may differ, [see below](#checked-against-the-answers-openrocket-cached)). A bare `auto`
(`<outerradius>auto</outerradius>`) is the same with nothing worked out yet, and is the commoner
form: 309 of the 413 automatic dimensions in the corpus cache no number, so a reader that resolves
them cannot treat the cached value as a shortcut. HPR Sim keeps both halves: which it is, and the cached number.
Keeping only the number is
[Loft lesson L58](../decisions-and-roadmap.md#l58): it turned automatic dimensions into hand-typed
ones the next time the design was saved. **Observed:** 413 automatic dimensions across the corpus,
on seven tags.

| tag | automatic |
|---|---|
| `outerradius` | 131 |
| `innerradius` | 80 |
| `cd` | 79 |
| `radius` | 43 |
| `packedradius` | 36 |
| `aftradius` | 30 |
| `foreradius` | 14 |

**A tag may be written under two names.** OpenRocket renamed several and writes both, so an older
reader still finds one. Two of those renames are that and nothing more, and HPR Sim reads either
name, taking the newer. **Observed:** on every element that carries both, the two agree: on the
text, and on the `type`/`method` attribute that says what the number is measured from.

| newer | older | elements with both | agree on text | differ on frame |
|---|---|---|---|---|
| `axialoffset` | `position` | 642 | 642 | 0 |
| `instancecount` | `fincount` | 109 | 109 | 0 |

Two that disagree is not something OpenRocket writes, so it raises a warning, whether they
disagree on the number or on where the number is measured from.

**Three more pairs look the same and are not.** The newer name of each carries a `method`
attribute (the frame the number is measured in) that the older name never carries:

| newer | older | elements with both | agree on text | differ on frame |
|---|---|---|---|---|
| `angleoffset` | `rotation` | 95 | 95 | **95** |
| `angleoffset` | `radialdirection` | 26 | 26 | **26** |
| `radiusoffset` | `radialposition` | 0 | n/a | n/a |

For a long time HPR Sim read neither name of any of them. The two angle pairs agree on the number
every time and differ on the frame every time; `radiusoffset` (on 106 elements) and
`radialposition` (on 542) are never written together at all, so nothing about *them* had been
measured. Reading one as the other would move a component without saying so.

[M3.1b3](../decisions-and-roadmap.md#m3-1b3) settled all three without ever having to say what the
older name's frame is (see
[how far off the axis](#the-parts-on-and-inside-the-body), below).

**A stated zero is a value.** `<overridecd>0.0</overridecd>` means no drag at all, not "no
override": reading it as missing is
[Loft lesson L63](../decisions-and-roadmap.md#l63), which charged a zero-drag part full drag. A
component declares its own mass, center of gravity and drag coefficient with three tags, and
whether each covers the components inside it with three more, all read independently. A part's or
a stage's drag coefficient goes into the design as its `drag_override`, with its own flag, and
flies as OpenRocket 24.12 flies it: on the reference area, once per fin, lug or button, in place
of all the part's own drag ([A part's stated drag coefficient](../physics/aero.md#a-parts-stated-drag-coefficient),
[ADR-167](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0167-a-part-s-drag-override-as-openrocket-flies-it.md)).
One on the `<rocket>` element itself has no place in the design and is not applied; no file seen
has one.

**Observed** override tags: `overridemass` 108, `overridesubcomponentsmass` 95,
`overridesubcomponents` 10, `overridecg` 14, `overridesubcomponentscg` 9, `overridecd` 2,
`overridesubcomponentscd` 2. The third of those is the single flag that files before schema 1.9
use in place of the three per-quantity ones. **Policy:** it is read as OpenRocket 24.12 reads it:
as setting all three, in files of schema 1.4, 1.8 and 1.10 alike. Where a part writes both forms,
the one written later wins, quantity by quantity: `<overridesubcomponents>true</overridesubcomponents>`
then `<overridesubcomponentsmass>false</overridesubcomponentsmass>` covers the parts inside for the
center of gravity and the drag but not the mass. No element in the corpus carries both forms. Eleven
[probe designs](../glossary.md#probe-design) measured this ([M2.2e6](../decisions-and-roadmap.md#m2-2e6), the old override flag),
and the tests in `hpr_validate::openrocket` hold HPR Sim to them. HPR Sim no longer warns about the
flag. Not measured: a value other than `true` or `false`, which is dropped with a warning (so the
design is not flown), and a flag tag written twice on one part, which is read at its first copy,
also with a warning.

## Warnings, not failures

A design written by an older OpenRocket, or by another program, should still open. Everything that
departs from [F] but leaves the file readable is a warning that travels with the result
(`hpr_io::ork::Warning`), not an error:

| kind | means | raised for |
|---|---|---|
| `Skipped` | a whole part was left out | a **component** this reader cannot give an honest shape ([below](#what-is-left-out-and-why)); an **attachment** entry that could not be decompressed, or one that would pass the unpacking limit; a damaged *design* entry is an error, not a warning |
| `Dropped` | a value was ignored | a comment or processing instruction; an XML namespace; a tag whose text is not the number, count or flag it should be; two names for one value that disagree; a dimension the file does not give, read as zero |
| `Unusual` | read as it stands | a schema version past 1.11; no `creator` attribute; a design entry not called `rocket.ork`; a surface finish or an axial-offset method this reader has no rule for; an automatic radius with nothing to take, given OpenRocket's 25 mm default; a part inside an inner tube set off the body's axis, placed from the body's axis ([below](#clusters)); a `<rocket>` holding nothing |

**Observed:** reading the corpus's containers and documents raises **no warnings at all**: every
file that opens is ordinary. Building a *rocket* from those documents raises 21 warnings over 73
readable files: 4 dropped, 8 skipped and 9 unusual ([below](#measured-on-the-reference-library)). Every kind of warning the container and document readers
can raise is therefore exercised by a test rather than by a file anyone shipped.

Only these stop a read:

- bytes that are none of the three containers;
- a damaged zip or gzip;
- a zip with no entry that could be the design;
- a design that is not UTF-8, or not well-formed XML;
- a root element that is not `<openrocket>`;
- a `version` attribute that is missing or is not `major.minor`;
- a document that nests more than 64 elements deep;
- an archive that unpacks to more than 256 MiB.

### Why there are two limits

A deflate stream can expand by about a thousand to one, so a `.ork` of a megabyte can ask for more
than a gigabyte of memory, and one of forty megabytes can ask for more than a machine has. Running
out is an abort, not an error. So a read decompresses at most 256 MiB out of one archive (the
largest in the corpus unpacks to 2,052,024 bytes, document and attachments together) and an entry
that would pass the limit is left out with a warning. If the entry left out was the design, the read fails rather than
returning half a file. `hpr_io::ork::container::unpack_within` takes another limit, for a caller
with less memory to spend.



Reading descends the tree, and so does the XML parser underneath. A file written to nest deeply
enough exhausts the stack, which is a crash rather than an error, where
[Loft lesson L56](../decisions-and-roadmap.md#l56) asks for an error. Feeding `roxmltree` documents
of increasing nesting in a debug test build, on a 2 MiB test-thread stack, it read 120 levels and
died on 130; the process aborts, so this one measurement cannot itself be a committed test, and
the figure moves with the stack a platform gives a thread. That is the argument for not relying on
it.

So HPR Sim counts the nesting **before** the text reaches the parser, with a scan that skips
comments, CDATA and processing instructions and tracks quotes, and refuses anything past 64 levels.
The scan can only ever count *more* levels than a parser will descend (XML forbids a raw `<` in an
attribute value, so every element start it sees is a real one) and
`hpr_io::ork::tests::the_depth_scan_never_undercounts` holds it to that over generated documents.

**Observed** for the depth, over the 73 files that open: 53 nest 11 deep, and the distribution runs 4, 7, 9, 10,
11, 12, 13, 14 and 17. An ordinary single-stage design reaches 11; the deepest, at 17, is
OpenRocket's own parallel-booster example, where each nested stage or inner tube costs two levels
and a component's appearance three. So 64 leaves about three more levels of nesting than anything
anyone has written.

## Checked against real files

`cargo xtask ork` reads every `.ork` in the reference library and in the OpenRocket jar's example
set, writes them back out, reads them again, and reports counts. The private corpus stays private:
the per-file detail goes to a gitignored `corpus-out/`, and only counts are published. On
2026-09-23, excluding generated `refs/scratch/` files:

| | |
|---|---|
| files found | 75 |
| opened | 73 |
| written and read back unchanged | 73 |
| warnings raised reading the container and the document | 0 (building a *rocket* from them raises 39; see [below](#measured-on-the-reference-library)) |
| refused, not well-formed XML | 2 |
| deepest nesting | 17 |
| elements holding text beside children | 48, in 19 files |
| largest unpacked (document and attachments) | 2,052,024 bytes |

The two that do not open are hand-written fixtures for Loft's browser tests, and neither is XML: a
`<databranch>` is closed with `</flightdata>`. Python's `expat` refuses both at the same lines
HPR Sim does, so this is the files' fault and not the reader's. They are listed by name in the
survey, which fails if either one ever behaves differently.

Here the *reference library* is every `.ork` under `refs/`, fetched by `cargo xtask refs fetch`;
the 17 example designs inside the OpenRocket jar (also under `refs/`) are counted with it unless a
table lists them apart.

**Every other file imports without an error.** An import error is a file that does not read, or a
design that reads but does not lay out. Warnings are not errors, because a warning never stops an
import. The survey counts both kinds of error for each source. On 2026-09-23, excluding generated
`scratch/` files from the default scan:

| source | files | read | laid out | errors |
|---|---|---|---|---|
| the private design library (`loft-fixtures`) | 27 | 27 | 27 | 0 |
| the OpenRocket 24.12 jar's examples | 17 | 17 | 17 | 0 |
| everything else under `refs/` | 34 | 32 | 31 | 2 (the two Loft fixtures that are not XML) |

One file in the last row reads but holds no design, so it has nothing to lay out.

**What you can check yourself.** The corpus is not public, so these counts are not reproducible on
a fresh clone: `cargo xtask ork` needs `cargo xtask refs fetch` first, and stops with
"no .ork files found" without it. What *is* reproducible anywhere is everything the tests cover:
`cargo test -p fusionspace-hpr-io`, and the same command on any `.ork` files you have, with
`cargo xtask ork --dir <path>`.

[api]: https://hpr.fusionspace.co/api/hpr_design/tree/struct.Rocket.html
[adr-051]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0051-m3-1-split-and-the-ork-document-kept-whole.md
[adr-052]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0052-what-a-ork-value-means-automatic-dimensions-two.md
[loft]: https://github.com/nrdptel/fusionspace-loft
[debrief]: https://github.com/nrdptel/fusionspace-debrief

### Snapshots of public designs

Seven small designs from [Loft][loft], the project owner's earlier tool, are committed in
`validation/fixtures/ork/loft-demo/` (MIT). The test
`hpr_io::ork::tests::loft_demo_designs_read_as_snapshotted` reads each with `hpr_io::ork::design`,
and a synthetic design whose motor flies from the bundled catalog besides, and compares a summary of
what it reads with a committed [`insta`](https://insta.rs) snapshot. A snapshot is a saved copy of
the output that the next run must match. Each summary records:

- the stages and body components;
- the structure's mass and center of mass, to nine significant figures;
- the motor configurations, and which of them fly;
- the recovery settings and the stored simulations;
- what is kept in [`x-openrocket`](#what-the-simulator-keeps-for-writing-the-file-back), and every
  warning.

**A snapshot shows that the reading has not changed, not that it is right.** None of these numbers
is compared with another program here; the geometry is, in
[the next section](#checked-against-rocketserializer). For example, `demo-stable.ork` is a 38 mm trainer on
an AeroTech H128W. Its snapshot says the structure weighs 0.540 kg and that its one configuration
does not fly (`"flown": []`), because that motor has no thrust curve in the file or in HPR Sim's
small bundled catalog ([motors](#motors-and-their-configurations)). Each field's meaning is in the
section of this page that reads it.

**When a snapshot changes,** the test fails and shows the old and new output. Run
`cargo insta review` (from `cargo install cargo-insta`) to see them side by side, and accept only a
change you can explain (or rerun with `INSTA_UPDATE=always cargo test -p fusionspace-hpr-io` and read the
diff in git); the new `.snap` file goes in the same pull request. The private reference
library is never snapshotted; `cargo xtask ork` reports it only as counts, as above.

### Checked against RocketSerializer

**In short.** [RocketSerializer][rocketserializer] is a second program that reads `.ork` files.
RocketPy's team wrote it to turn an OpenRocket design into RocketPy's inputs. This check compares
HPR Sim's reading of each design's shape with RocketSerializer's. Where the two differ, OpenRocket
itself, run on the same file, decides which is right.

On 2026-09-23 the current scratch-excluding check covered 71 designs. Of 1,171 numbers:
- HPR Sim matched RocketSerializer on 1,061;
- on the other 110, OpenRocket gave HPR Sim's number, not RocketSerializer's;
- HPR Sim never differed from both.

**Every one of HPR Sim's 1,171 numbers is also OpenRocket's.** That includes the 1,061 where it
matched RocketSerializer. So no agreement here is a mistake the two readers happen to share.

This checks the *reading*: the dimensions in the file, and where each part sits. It does not check
mass, the center of gravity, or any physics. The ongoing [M2.2](../decisions-and-roadmap.md#m2-2)
work compares those properties, motors, stored results and flights with OpenRocket; completed
mass-property results are in [Mass properties](../physics/mass.md).
The check was added by [M3.1d2](../decisions-and-roadmap.md#m3-1d2), the roadmap step that finished
reading `.ork` files, and [ADR-059][adr-059] records how it was decided.

**What is compared.** What RocketSerializer reports about the airframe's shape:

| part | numbers |
|---|---|
| the nose cone | its shape, length and base radius, and, for a Haack series nose, its shape parameter (0 for a Von Kármán nose) |
| each transition | its length, and its radius at each end |
| each trapezoidal or elliptical fin set | the number of fins, root chord, tip chord, span, sweep, [cant](../glossary.md#cant) and cross-section (square, rounded or airfoil edges) |
| each of those parts | its [station](../glossary.md#station): where its front sits, in meters aft of the nose tip |
| the rocket | its body radius: the largest radius the file writes as a number |

RocketSerializer also reports each part's name, and a fin set's sweep angle when the file writes
one (the files here write the sweep as a length); those are not compared.

**How a difference is settled.** A script, `validation/oracles/rocketserializer/geometry.py`, runs
RocketSerializer's *extractors*, the functions it uses to pull each part out of a file, on every
design. It also asks OpenRocket 24.12, loaded on the same file, for each of the same numbers. It
writes what both programs read to a JSON file, the *record*. `cargo xtask ork` then compares
HPR Sim's reading with the record. Two numbers count as the same when they differ by less than 1
part in 10⁹:

- If HPR Sim's number is RocketSerializer's, they **agree**.
- If it is not, but it is OpenRocket's, then **RocketSerializer differs**.
- Otherwise **HPR Sim differs**, and the survey fails.

Whatever RocketSerializer says, the survey also fails when one of HPR Sim's numbers is not
OpenRocket's. Otherwise a mistake HPR Sim and RocketSerializer made together would pass as
agreement. One such case is known. OpenRocket limits a fin's cant to 15°, while HPR Sim and
RocketSerializer take a larger cant as written, so the survey would fail on such a file.
[Issue #148](https://github.com/nrdptel/fusionspace-eridanus/issues/148) tracks what HPR Sim should do. No file here
cants a fin past 15°.

**A worked example.** Loft's public `demo-dual-deploy.ork` puts its fin set at the bottom of the
booster tube. The same tube also holds the drogue parachute, packed 0.08 m long, listed before the
fins in the file. The parachute sits inside the tube, so it adds nothing to where the fins are.
The file places the fins relative to the tube's aft end, and OpenRocket and HPR Sim both put the
fins' front 1.45 m aft of the nose tip. RocketSerializer adds
up the lengths of every part listed before the fins in the same tube, the parachute's 0.08 m
included, so it puts them at 1.45 + 0.08 = 1.53 m. The record keeps that sum of earlier lengths
for every part, so this cause is checked, not assumed.

**The results.** From `cargo xtask ork` on 2026-09-23, over the 75 files it finds (the reference
library under `refs/`, excluding generated `refs/scratch/`, and the 17 examples inside the OpenRocket jar):

| | all designs | each file once |
|---|---|---|
| designs compared (files OpenRocket opens) | 71 | 51 |
| numbers compared | 1,171 | 855 |
| HPR Sim agrees with RocketSerializer | 1,061 | 772 |
| RocketSerializer differs, and HPR Sim's number is OpenRocket's | 110 | 83 |
| HPR Sim differs from both | **0** | **0** |
| HPR Sim's number is OpenRocket's | 1,171 | 855 |

Some files are in the library more than once: 13 of Loft's private designs are copies of the jar's
examples, for one. The second column counts each file's content once.

The 110 differences, by cause:

| cause | count |
|---|---|
| a fin set's station: RocketSerializer adds the lengths of the parts before it in its parent, as in the worked example | 75 |
| a transition's radius: RocketSerializer looks transitions up in OpenRocket by name, and takes the first of that name | 15 |
| no cause shown | 20 |

**Stations rest on OpenRocket.** RocketSerializer does not read a station, or a transition's
radius, from the file. It loads the design into OpenRocket and walks OpenRocket's tree, adding up
lengths as it goes, which is where the worked example's extra 0.08 m comes from. So its stations
are not an independent reading. It agrees with HPR Sim on 8 of the 96 fin sets' stations and 18 of
the 22 transitions'; the rest are among the 110 above, where HPR Sim's station is OpenRocket's. For
stations this is really a check against OpenRocket alone. For lengths, chords, spans, counts and
cant, which RocketSerializer reads from the file, it agrees with HPR Sim every time.

For the 20 with no cause shown, the reason RocketSerializer's number differs has not been traced.
In each of them HPR Sim's number is OpenRocket's, and the per-file record holds all three programs'
numbers:
- 17 are stations, 13 of fin sets and 4 of transitions.
- 1 is a nose cone's base radius, and 1 is the body radius.
- 1 is a nose cone's shape: a nose written as an `ogive` of shape parameter 0. OpenRocket draws it
  as a cone, and so does HPR Sim, while RocketSerializer passes on the word `ogive`.

**Four things OpenRocket does that the check allows for.**

- **A design worked out again.** OpenRocket's first reading of an automatic radius can differ
  from the answer it settles on once it works the design out again
  ([when an automatic radius has nothing to take](#when-an-automatic-radius-has-nothing-to-take)).
  The script saves each design once, to a throwaway, before it reads anything, so every number in
  the record, RocketSerializer's included, comes from the settled design.

- **A canted fin.** OpenRocket turns a canted fin about the middle of its root chord. That moves
  the front of the root aft by half the chord times (1 − cos δ), where δ is the cant: 38 µm for a
  0.495 m root at 1°, as in the OpenRocket jar's *Simulation extensions* example. The file places the root before it is turned, and so does HPR Sim. So the script
  reads OpenRocket's station with the cant set to zero, then puts the cant back. It keeps the
  turned station too. For all 10 canted fin sets, the turned station is aft of the unturned one by
  exactly that amount, and `cargo xtask ork` checks it.
- **A shape word.** An ogive of shape parameter 0 is a cone
  ([the spine](#the-spine-stages-and-body-components) says why). To tell which shape OpenRocket
  really draws, the script records OpenRocket's radius a quarter, a half and three quarters of the
  way along the nose. HPR Sim's profile is compared with those three radii: all 72 noses
  RocketSerializer reports match. So a
  nose's shape counts as OpenRocket's when OpenRocket draws HPR Sim's profile, whatever word it
  uses.
- **A leading comment.** OpenRocket 24.12 refuses a file that begins with a long enough comment. For
  3 files, the script removes the comment from the copy OpenRocket reads, and the record says so.

**What it leaves out.**
- RocketSerializer reports no body tube, no inner part, no mass, no fin thickness, and no freeform
  or tube fins, so none of those is compared here.
- 6 parts inside pods or parallel stages are not compared. HPR Sim reads pods since
  [M1.13b](../decisions-and-roadmap.md#m1-13b) and parallel stages inside a body tube since
  [M4.5m](../decisions-and-roadmap.md#m4-5m), but the cross-check still leaves their parts out
  ([issue #211](https://github.com/nrdptel/fusionspace-eridanus/issues/211)).
- 3 values RocketSerializer gives nothing for are not compared: 1 body radius, in a file that
  writes every radius as `auto`, and 2 fin cross-sections, in a file that writes none.
- 4 files are not compared, because OpenRocket 24.12 does not open them. One is Loft's
  `demo-quirks.ork`, whose parallel stage sits directly under the rocket. Two are the Loft fixtures
  above that are not XML. The fourth holds no design.

**Run it yourself.** CI does not run RocketSerializer or OpenRocket. It holds HPR Sim to their saved
output for the seven public Loft designs, `validation/fixtures/ork/rocketserializer-loft-demo.json`,
with `cargo test -p xtask rocketserializer`. OpenRocket opens six of the seven. Of their 80
numbers, 74 agree, and the other 6 are fin stations that RocketSerializer adds earlier lengths to,
as in the worked example.

To make a record yourself you need Python 3.11, [`uv`](https://docs.astral.sh/uv/), Java 17 and
the OpenRocket 24.12 jar (`cargo xtask refs fetch` downloads the jar to `refs/openrocket/`). From
the repository root:

```sh
# Once: an environment of its own, with every package pinned.
uv venv -p 3.11 refs/venv-rs
uv pip install -p refs/venv-rs --no-deps -r validation/oracles/rocketserializer/requirements.txt

# The seven public designs, written over the committed record; nothing should change:
refs/venv-rs/bin/python validation/oracles/rocketserializer/geometry.py \
    validation/fixtures/ork/rocketserializer-loft-demo.json validation/fixtures/ork/loft-demo
git diff validation/fixtures/ork/

# The whole reference library and, with --jar, the jar's example designs; then compare:
refs/venv-rs/bin/python validation/oracles/rocketserializer/geometry.py \
    corpus-out/rocketserializer.json refs --jar
cargo xtask ork
```

For your own files, give the script their directory in place of `refs`: it writes what
RocketSerializer and OpenRocket read. Comparing HPR Sim's numbers with them is not automated yet;
`cargo xtask ork` compares HPR Sim with the record of the reference library only.

[rocketserializer]: https://github.com/RocketPy-Team/RocketSerializer
[adr-059]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0059-the-rocketserializer-cross-check-three-readers.md

## The spine: stages and body components

The trunk of a design is its **spine**: the stages, and inside each of them the nose cones, body
tubes and transitions that stack end to end along the axis. `hpr_io::ork::rocket` turns a document's
spine into an [`hpr_design::Rocket`][api] (shapes, lengths, radii, walls, materials and overrides)
and everything else (the tubes and rings inside the body, the fins and lugs on it, the recovery
gear) is counted and left for [M3.1b3](../decisions-and-roadmap.md#m3-1b3).

**An automatic radius is marked, not filled in.** Where the file says `auto`, the component carries
the dimension's name and `Rocket::layout()` works the radius out from the neighbours, including
across a stage boundary, so a booster's first component takes its radius from the stage ahead of
it. That is [Loft lesson L59](../decisions-and-roadmap.md#l59): Loft resolved within a stage only,
and a booster came out as whatever number happened to be cached. A stated wall is also kept when
the radius it sits in is automatic; judging the wall against a radius that is not known yet threw
it away, which is half of [Loft lesson L61](../decisions-and-roadmap.md#l61).

**The shape parameter is not the same number.** OpenRocket writes the ogive's parameter as
`κ = ρ_tangent/ρ` (a tangent ogive's radius of curvature over this one's), so `κ = 1` is a tangent
ogive and `κ = 0` an infinite radius, which is a cone. `hpr-design` states the same shape the other
way up, as `ρ/ρ_tangent`, so the two are reciprocals: reading one as the other turns every secant
ogive into a bulged one. The power, parabolic and Haack parameters carry over unchanged. All four
are Niskanen's appendix A, equations A.3 and A.7 to A.9.

<a id="not-settled"></a>

**Readings this page is not sure of**, every one of them for the OpenRocket oracle
([M2.2](../decisions-and-roadmap.md#m2-2)) to settle. The first is the spine's; the rest come from
the parts. Two readings this table held are settled: a shoulder, and a tube, of zero wall thickness
weigh nothing, as in OpenRocket 24.12, measured in [M2.2b1](../decisions-and-roadmap.md#m2-2b1)
([ADR-061][adr-061]).

| what the file says | how it is read | why it is in doubt |
| --- | --- | --- |
| no `shapeclipped` on a transition | clipped | it is the shape that reaches both radii; only 1 of the 21 transitions in the corpus states it |
| an angle, **which way it turns** | the same way HPR Sim's own frames turn | HPR Sim measures a roll angle right-handed about an axis pointing at the **nose**; OpenRocket's technical documentation puts its own `x` axis along the centerline pointing **aft** and leaves the rest unstated. If it means what that implies, every angle read here is mirrored; see [below](#which-way-round) |
| a `polished` finish | 2 µm | the number is the author's, from 2013; a newer OpenRocket may have moved it ([below](#the-surface-finish)) |

**Measured on the reference library** (`cargo xtask ork`, 73 readable files): 72 designs' spines lay
out, over 92 stages and 274 body components: 178 body tubes, 73 nose cones, 23 transitions. Two
of the stages, and two nose cones and two body tubes, are parallel stages, read since
[M4.5m](../decisions-and-roadmap.md#m4-5m) ([Parallel stages](#parallel-stages)). The
survey marked 327 automatic dimensions, including 7 body radii that took OpenRocket's 25 mm default.
The counts are what may be published; the per-file detail stays in the gitignored `corpus-out/`.

*(When this was written, 3 designs did not lay out, and they were thought to be waiting on parts that
were not read yet. Reading those parts, in [M3.1b3](../decisions-and-roadmap.md#m3-1b3), showed
otherwise. One document holds no design at all, and two have radii with nothing to take, which
[M3.1b4](../decisions-and-roadmap.md#m3-1b4) settled:
[when an automatic radius has nothing to take](#when-an-automatic-radius-has-nothing-to-take).
All 72 designs lay out now.)*


## The parts on and inside the body

Everything that is not the spine hangs off it, and HPR Sim reads it into the same
[`hpr_design::Rocket`][api]. Four families, with the words this page uses for them:

- **Tubes inside a body component**: an *inner tube* (usually the motor mount), a *coupler* (the
  short tube that joins two airframe sections), an *engine block* (the ring that stops a motor
  sliding forward). Each can hold parts of its own.
- **Rings that center them**: a *centering ring*, a disc with a hole (its **bore**) that holds a
  motor tube on the airframe's axis, and a *bulkhead*, the same disc with no hole.
- **What sits on the outside**: fin sets, tube fins, *launch lugs* (the tubes a launch rod passes
  through) and *rail buttons*.
- **What is packed in the bore**: mass objects (an altimeter, a battery), parachutes, streamers
  and shock cords. HPR Sim reads the cylinder each takes up when packed, not what it does when it
  opens.

Code: [`hpr_io::ork::attached`](../api/hpr_io/ork/attached/index.html), decided in
[ADR-053: the parts on and inside a `.ork` body][adr-053].

### Opening a design, in full

`rocket` reads the airframe alone. To get the motors as well, call `hpr_io::ork::design` instead
([motors and their configurations](#motors-and-their-configurations)).

This is the doctest on [`hpr_io::ork::rocket`](../api/hpr_io/ork/component/fn.rocket.html), which CI runs.
Hand it the bytes of a `.ork`, and what comes back is a design and the warnings reading it raised:

```rust
let read = hpr_io::ork::read(xml)?;
let design = hpr_io::ork::rocket(&read.value.document);

// Anything the reader could not take at face value travels with the result. Nothing here did.
assert!(design.warnings.is_empty(), "{:?}", design.warnings);

// The ring's outer radius says `auto`, so the design carries the dimension, not a number...
use hpr_design::AutoDimension;
let ring = &design.value.stages[0].components[1].children[0];
assert!(ring.auto.contains(&AutoDimension::OuterRadius));

// ...and the layout works it out: the bore of the tube the ring sits in, 0.05 - 0.002.
let layout = design.value.layout()?;
let (_, placed) = layout.find("ring").expect("the ring");
let hpr_design::Part::CenteringRing(ring) = &placed.part else { panic!("a ring") };
assert!((ring.outer_radius_m - 0.048).abs() < 1e-12);
assert!(placed.own.mass_kg > 0.0);
```

**`design.warnings` is where a part that was left out is named.** It is a `Vec` of
[`Warning`](../api/hpr_io/ork/struct.Warning.html), each carrying where in the file it happened,
how much was lost (`Skipped`, `Dropped` or `Unusual`) and a sentence saying what was read and how:
for example "a launch lug sits on a nose cone, and HPR Sim attaches one only to a body tube; it was
left out". Read them: a design that opens cleanly raises none, and the 73 readable files of the
reference library raise 10 between them, every one of them explained on this page
([what the 10 warnings are](#what-the-warnings-are)).

| `.ork` tag | read as | notes |
| --- | --- | --- |
| `innertube`, `tubecoupler`, `engineblock` | [`InnerTube`][p-inner] | may hold parts of its own |
| `centeringring` | [`CenteringRing`][p-ring] | bore and outer radius may both be automatic |
| `bulkhead` | [`CenteringRing`][p-ring] with no bore | |
| `trapezoidfinset`, `ellipticalfinset`, `freeformfinset` | [`FinSet`][p-fins] | with its tab, its cant (the angle the fins are turned to induce roll) and its section (the shape along the chord: square, rounded or airfoil) |
| `tubefinset` | [`TubeFinSet`][p-tubefins] | an `auto` radius closes the ring around the body, as OpenRocket works it out ([Tube fins sized from the body](#tube-fins-sized-from-the-body)); both sets in the corpus are written so |
| `launchlug`, `railbutton` | [`LaunchLug`][p-lug], [`RailButton`][p-button] | a row of them is one part with a count and a spacing; a rail button's `<screwheight>` is its screw head, weighed since [M4.5i](../decisions-and-roadmap.md#m4-5i) ([mass](../physics/mass.md#fins-rail-buttons-and-roll-inertia)) |
| `masscomponent` | [`MassComponent`][p-mass] | |
| `parachute`, `streamer`, `shockcord` | [`Parachute`][p-chute], [`Streamer`][p-streamer], [`ShockCord`][p-cord] | the packed shape, not the deployment |
| `podset` | [`PodSet`][p-podset] | the pod's nose cones, body tubes and transitions, and everything on them ([Pods](#pods)) |
| `parallelstage` | a [`Stage`][p-stage] with its [`ParallelStage`][p-parallel] layout | inside a body tube on a rocket of one stage on the axis: a stage of its own, laid out as a pod set on that tube ([Parallel stages](#parallel-stages)); anywhere else, kept whole ([what HPR Sim keeps](#what-the-simulator-keeps-for-writing-the-file-back)) |

**Angles in a `.ork` are degrees.** Nothing in the file says so, and every length beside them is in
meters, so it is easy to read one as radians, which would make `<angleoffset>180</angleoffset>`
more than twenty-eight turns instead of half of one.

**Observed:** of the **993** angles the corpus writes, **188** are not zero, and **178 of those are
larger than 2π**, more than a whole turn, which no component is written at. The values themselves
are 180, 90, 45, 30 and 120, carrying the float dust (`119.99999999999999`) of a conversion that
went through radians and came back. `cargo xtask ork` prints all three counts, and
`hpr_io::ork::tests::angles_are_degrees_not_radians` holds the reading.

A fin's **cant** is the same degrees: the corpus's two non-zero cants are 1.0 and −3.98, which as
radians would be 57° and 228°, a fin turned past half a turn from the airflow.

**Where a part sits** is one tag: `<axialoffset method="bottom">` on the newer name,
`<position type="bottom">` on the older, with the same five words (`top`, `middle`, `bottom`,
`after` and `absolute`) which are [`hpr_design::Position`][p-position] unchanged, but for a rail
button. OpenRocket gives a button no length and puts its center at the position, a row's first
button there and the rest aft, so HPR Sim moves the offset to where the button's forward edge must
be ([issue #151](https://github.com/nrdptel/fusionspace-eridanus/issues/151); measured on probes
from the top, the middle and the bottom). **Observed:** one tube coupler in the corpus has neither
tag; it is read flush with its parent's forward end, with a warning.

**How far off the axis** a part sits is `radialposition` on some tags and `radiusoffset` on others.
[ADR-052][adr-052] read neither, for want of a source saying what the older name measures from.

That question need not be answered, because **the two never meet**: `radialposition` (542 elements)
is written on the parts *inside* a body, `radiusoffset` (106) on the parts *on* it and on the pods,
and no element carries both. They are two tags on different components, not two names for one. So
each is read where it is the only name its part has, and neither is ever read as the other.

<a id="which-way-round"></a>

**Which way round** a part sits is the angle, and there the two names do meet. `angleoffset` is the
newer one; the older is `rotation` on a fin set (95 elements carry both) and `radialdirection` on
everything else (26). On all 121, the two **agree on the number**, so which one is read cannot
change an angle. They differ on the *frame* (`relative` to the parent against `fixed` in the
rocket), but that is the same angle for every parent HPR Sim builds, all of which sit on the
rocket's own axis. A pod set hangs from a body tube, which is on the axis too, and OpenRocket 24.12
puts its pods in the same places whichever of its three frame words the file writes (`relative`,
`fixed`, or `mirror_xy`, mirrored) ([Pods](#pods)). Whether a part *inside* a pod measures its angle
from the pod or from the rocket has not been tested against OpenRocket.

**Which *direction* the angle turns is assumed, and is not settled.** HPR Sim measures a roll angle
from `x_B` toward `y_B`, right-handed about `+z_B`, which points at the nose
([frames](../physics/frames.md)). OpenRocket's [technical documentation][techdoc] (§3.1.4) puts its
own `x` axis along the centerline pointing **aft**, and says nothing about the other two. A
right-handed angle about an aft-pointing axis is a left-handed one about HPR Sim's `+z_B`, so if
that is what OpenRocket means, every angle read here is **mirrored**: a mass object at 90° sits on
the other side of the airframe, and a canted fin set rolls the other way. Nothing in the reference
library can settle it, because a mirrored design is still a perfectly valid design; one
deliberately asymmetric design put through the [OpenRocket oracle](../decisions-and-roadmap.md#m2-2)
will. Until then HPR Sim takes the number unchanged, and this is on the list of
[readings that are not settled](#not-settled).

**An override that covers the parts inside a component** is read from the mass flag. A `.ork` says
"this figure covers the components inside this one" once for the mass, once for the center of
gravity and once for the drag; `hpr-design` says it once for the whole component, so the two cannot
always agree. Mass is the quantity the flag is written for (95 of the 104 written in the corpus
are the mass flag), so the mass flag decides, and a center-of-gravity flag that disagrees with it
raises a warning. Nothing in the corpus disagrees. This matters from this milestone on and not
before: until a component had parts inside it, "covers the parts inside" covered nothing.

**What a part takes from its parent** is resolved by [`Rocket::layout()`][p-layout], never here. A coupler's or
a ring's automatic outer radius is its parent's bore (inside a nose cone or a transition, the bore at
the part's narrower end, [below](#inside-a-nose-cone-or-a-transition)); a ring's automatic bore is the widest motor
tube beside it that overlaps it along the axis; a packed part fills the room left in the bore.
Automatic **outer** radii resolve in a pass before any ring's automatic **bore**, so a ring's answer
cannot depend on whether the tube inside it was written first:
[Loft lesson L60](../decisions-and-roadmap.md#l60), where Loft resolved as it walked and a bulkhead
inside a coupler stayed `NaN` (not a number, meaning no numeric value was available).

### Inside a nose cone or a transition

A coupler, an engine block or a ring can sit inside a hollow nose cone or transition, with its outer
radius written `auto`. A nose's bore narrows toward the tip, so "the parent's bore" needs a place
along the nose. Since [M2.2e7](../decisions-and-roadmap.md#m2-2e7) HPR Sim takes it where OpenRocket
24.12 does ([ADR-096][adr-096], the decision on fillets and a nose's bore):

- The radius is the parent's outer radius at the part's narrower end, less the parent's wall. The
  nose's shoulder is left out, even when the part reaches into it.
- If the wall is thicker than that radius, the radius is zero.
- A filled (solid) nose cone has no bore. The part is left out with a `Skipped` warning.
- A packed part (a parachute, a mass component) inside a nose cone still takes no automatic radius.
  It is left out with a `Skipped` warning too.
- A coupler at the very tip, where the wall meets the axis, has no radius at all. OpenRocket weighs
  it as nothing; HPR Sim refuses to lay out the design.
- A tube whose own wall is thicker than the automatic radius it gets is a solid rod. This holds in
  a body tube too, and is here because two of the probes below measure it, one in each.

For example, take a conical nose cone 200 mm long, 50 mm in radius at its base, with a 2 mm wall,
and a coupler 30 mm long whose aft end sits at the nose's base. Its narrower end is its forward end,
30 mm ahead of the base, where the cone's radius is `50 × 170/200` = 42.5 mm. So its outer radius
is 42.5 − 2 = 40.5 mm.

**An inner tube written `auto` keeps 9.5 mm.** OpenRocket 24.12 does not work out an automatic
radius for an `innertube`, the tag a motor mount is usually written with. It keeps the 9.5 mm it
starts with, in a nose cone or a body tube alike. HPR Sim reads it the same way, with no warning, so
a design holding one flies as OpenRocket flies it. A `tubecoupler`
or an `engineblock` written `auto` fills its parent as above.

**Measured.** Fourteen probe designs in `validation/oracles/openrocket/conventions.py` ask
OpenRocket these questions. They hold a coupler of automatic radius at a nose's bottom, past its
base, with a shoulder, long from its middle, at the tip, and with a wall thicker than its bore; an
ogive nose; a transition; an engine block; a centering ring and a bulkhead; a mass component inside;
and an inner tube written `auto`, in a nose and in a tube. HPR Sim flies 13 of the 14. Every part
inside a nose cone, transition or tube but one weighs OpenRocket's mass at OpenRocket's station:
the test holds the mass to 1e-14 and the station to 1e-15. Three things are pinned apart:

- One mass component, packed with an automatic radius inside the coupler, sits 20.75 mm from
  OpenRocket's. OpenRocket shortens its packed length to keep its volume, and HPR Sim keeps the
  written length
  ([#186: OpenRocket repacks a part that does not fit](https://github.com/nrdptel/fusionspace-eridanus/issues/186)).
- The nose cones' and the transition's own walls keep the gaps they already had: up to 3.9e-5 of
  the mass and 2.5e-6 m in the center, since HPR Sim measures a wall square to the surface.
- The coupler at the tip is the 14th probe. HPR Sim refuses it, and OpenRocket weighs it as nothing.

The test `an_automatic_radius_inside_a_nose_reads_as_openrocket_does` in `hpr-validate` holds
HPR Sim to these answers. Before, HPR Sim had no bore to give such a part, so it left the part out
with a warning, and a design holding one did not fly.

### Tube fins sized from the body

A *tube fin set* is a ring of short open tubes glued along the airframe in place of flat fins. In
OpenRocket, a tube fin set's radius can be written `auto`. OpenRocket then sizes the tubes from
the body tube they sit on. Since [M2.2e8](../decisions-and-roadmap.md#m2-2e8) HPR Sim works that
radius out as OpenRocket 24.12 does ([ADR-098][adr-098], the decision on a tube fin set's
automatic radius). Before, HPR Sim left such a set out with a `Skipped` warning
([#133][issue-133]). HPR Sim reads and weighs tube fins now, and since
[M2.2e9](../decisions-and-roadmap.md#m2-2e9) a design that holds them flies
([below](#what-tube-fins-still-lack)).

**The rule.** With three tubes or more, each tube touches the body and its two neighbours, so the
ring closes:

- The tubes' axes sit on a circle of radius `R + r` around the airframe's axis, where `R` is the
  body tube's outer radius and `r` the tubes' radius.
- Neighbouring axes are `2π/N` apart around that circle, for `N` tubes, so the straight line
  between them (the chord) is `2(R + r) sin(π/N)` long.
- Two tubes of radius `r` touch when their axes are `2r` apart. Setting the chord to `2r` gives
  `r = R sin(π/N) / (1 − sin(π/N))`.

One or two tubes can't close a ring. For those, OpenRocket gives the tubes the body's radius, and
so does HPR Sim. The rule is
[`TubeFinSet::closing_radius_m`](../api/hpr_design/fins/struct.TubeFinSet.html#method.closing_radius_m).
HPR Sim applies it when it lays the design out, the step that works out every automatic dimension
([Automatic dimensions](../physics/design.md#automatic-dimensions)).

For example, take four tubes on a body of 50 mm radius. `sin(π/4)` is 0.7071, so
`r = 50 × 0.7071 / 0.2929` = 120.7 mm. The axes then sit 170.7 mm from the airframe's axis, and
neighbours are `2 × 170.7 × 0.7071` = 241.4 mm apart, which is `2r`: they touch. Fewer tubes
must be wider to close the ring. Three to five tubes come out wider than the body, six exactly as
wide, and more than six narrower:

| tubes on a 50 mm body | radius |
| --- | --- |
| 1 or 2 | 50 mm, the body's |
| 3 | 323.2 mm |
| 4 | 120.7 mm |
| 5 | 71.3 mm |
| 6 | 50.0 mm, as wide as the body |
| 8 | 31.0 mm |

Three more things are read as OpenRocket reads them:

- **A wall thicker than the radius** is cut to the radius, so the tubes are solid rods. OpenRocket
  weighs them that way, as it does an inner tube's ([above](#inside-a-nose-cone-or-a-transition)).
- **More than 8 tubes are read as 8,** with a `Dropped` warning, whether the radius is `auto` or
  stated. OpenRocket 24.12 reads 9, 12, 20 and 100 tubes as 8. HPR Sim caps the count before it
  checks its own limit of 64 parts in a row, so 100 tubes read as 8 rather than being refused.
  That cap is OpenRocket's reading of the file, not a limit of the physics: HPR Sim's own design
  files take any count.
- **A radial offset** (tubes standing off the body) changes none of OpenRocket's numbers. HPR Sim
  reads such a set sitting on the body, with a `Dropped` warning, as it did before.

**Measured.** Nineteen [probe designs](../glossary.md#probe-design), small designs made only to ask
OpenRocket one question each, are in `validation/oracles/openrocket/conventions.py`. Each carries
one tube fin set:

- on a 50 mm tube: `auto` sets of 1, 2, 3, 4, 5, 6, 8, 9, 12, 20 and 100 tubes; six tubes of a
  stated 20 mm radius, with and without a 10 mm radial offset; twelve tubes of a stated radius;
  six `auto` tubes with a wall thicker than their radius; and four `auto` tubes on a tube whose own
  radius is `auto`, taken from the nose cone ahead of it;
- on a 20 mm tube: 1, 2 and 5 `auto` tubes. One and two take the body's 20 mm; five take the
  closed form's 28.5 mm.

OpenRocket's answers are in `validation/fixtures/ork/openrocket-conventions.json`. The test
`a_tube_fin_sets_automatic_radius_reads_as_openrocket_does` in `hpr-validate` holds HPR Sim to them:

| quantity | HPR Sim against OpenRocket, on all 19 |
| --- | --- |
| the tubes' radius and wall | within 1e-15, relative, in the test (equal when the oracle script compares them in Python) |
| the number of tubes | the same |
| each part's mass | within 1e-14, relative; the one nose cone, on the probe whose tube takes its radius from it, within 5e-5, the gap nose walls already had ([above](#inside-a-nose-cone-or-a-transition)) |
| each part's center of mass, along the airframe | within 1e-15 m; that nose cone within 5e-6 m |

OpenRocket's *Tube fin rocket* example is the only design in the reference library with tube
fins, in two copies. It now reads with its tube fins, and its mass is within 5.0e-6 of OpenRocket's
and its center of mass within 2.4e-6 of its length ([Mass properties](../physics/mass.md#tube-fins)).

**What differs: the roll and pitch inertia.** HPR Sim keeps its own figure for both, each a
departure: a difference kept on purpose, measured and pinned by the same test.

- **Roll** (spin about the airframe's axis). For two tubes or more, OpenRocket's roll inertia for
  the set is larger than any mass inside the ring could have. For a single tube the two agree.
- **Pitch** (turning end over end). OpenRocket's figure has no term for how far the tubes sit from
  the airframe's axis: per kilogram, it is the number of tubes times one tube's own figure about
  its own middle. On the probes it is 1.3 to 2.6 times HPR Sim's. Some mass inside the ring could
  have that much, but not tubes where both programs put them.

[Mass properties](../physics/mass.md#tube-fins) has the numbers. On the *Tube fin rocket*, HPR Sim's
roll inertia is 98.7% below OpenRocket's and its pitch inertia 1.98% below, almost all of it
OpenRocket's pitch rule.

<a id="what-tube-fins-still-lack"></a>**How they fly.** Since
[M2.2e9](../decisions-and-roadmap.md#m2-2e9), tube fin aerodynamics, a design with tube fins
flies. HPR Sim's aerodynamics takes each tube as a ring wing: a cited slope, a center from
Fletcher's measurements and HPR Sim's own derivation, and drag from the fin rules
([Tube fins](../physics/aero.md#tube-fins)). The *Tube fin rocket*'s apogee is 6.95% above
OpenRocket's. On OpenRocket's own drag, HPR Sim's apogee is within 0.03%, so the net gap is the
drag's ([ADR-099][adr-099], tube fins flown as ring wings). OpenRocket's tube-fin drag was refined
against measured flights, so HPR Sim's probably reads low
([#228](https://github.com/nrdptel/fusionspace-eridanus/issues/228)).

### The surface finish

A surface finish is a **roughness height** `R_s`, the size of the bumps a painted or bare surface
leaves, which is what sets skin friction ([drag](../physics/aero.md)). OpenRocket writes one of
five words for it; what each is worth in micrometers is **not in the file format documentation**,
and these are the only numbers on this page that come from neither the documentation nor the
corpus:

| `<finish>` | OpenRocket calls it | roughness | where that number comes from | in the corpus |
| --- | --- | --- | --- | --- |
| `rough` | Rough | 500 µm | [the author][forum] | 8 |
| `unfinished` | Unfinished | 150 µm | [the author][forum] | 2 |
| `normal` | Regular paint | 60 µm | [technical documentation][techdoc] §6 **and** the [user guide's dialog][dialog] | 348 |
| `smooth` | Smooth paint | 20 µm | [the author][forum] | 88 |
| `polished` | Polished | 2 µm | [the author][forum]; **see the caveat below** | 14 |

Only the default is officially documented, and it twice over: the [technical
documentation][techdoc] section 6 says that in its test design "the 'regular paint' finish was
selected, which corresponds to an average surface roughness of 60 µm", and the [user guide's
body-tube dialog][dialog] reads "Component finish: Regular paint (2.36 mil)", which is 59.9 µm. The
[user guide][finishes] names all five and their order: "Rough, Unfinished, Regular paint, Smooth
paint, and Polished, each with a decreasing (CD) from rough to polished", but gives **no**
numbers. The other four come from OpenRocket's author, in [The Rocketry Forum thread "Open Rocket
Finishes"][forum] (post #6, 22 August 2013): "Rough (500 µm) / Unfinished (150 µm) / Regular paint
(60 µm) / Smooth paint (20 µm) / Polished (2 µm)".

Each becomes a [`Finish::Custom`][finish-api] height (a roughness given as a number) rather than
one of HPR Sim's named finishes, whose names ("raw wood", "dip-galvanized metal") mean other
surfaces that happen to share a height.

**`polished` is the one to doubt.** Its 2 µm rests on a 2013 forum post and on nothing else, and
the number sits oddly: in the [table the OpenRocket technical documentation itself
reprints][techdoc] (Table 3.2, from Hoerner), 2 µm is *aircraft-type sheet metal*, while "finished
and polished surface" is **0.5 µm**. So OpenRocket's "Polished" is not the row its name points at,
and a later version could reasonably have moved it. Rocketry-forum posts from 2023 onward list nine
finishes rather than five, which suggests the list has indeed changed, though neither the 23.09 nor
the 24.12 release notes mention it.

What it would cost: skin friction in the fully-rough branch goes as `R_s^0.2`
([drag](../physics/aero.md)), so 0.5 µm instead of 2 µm is a **1.32×** change in the skin-friction
coefficient of whatever part says `polished`: **14 components** in the corpus. No file in the
corpus writes any word but the five, so nothing here can settle it; the
[OpenRocket oracle](../decisions-and-roadmap.md#m2-2) can. A word HPR Sim has no sourced roughness
for takes HPR Sim's default and says so in a warning.

**A part with no `<finish>`** gets HPR Sim's own default, 20 µm, but OpenRocket reads it as regular
paint, 60 µm ([issue #216](https://github.com/nrdptel/fusionspace-eridanus/issues/216)). On the pod probes that
raised HPR Sim's apogee 6.0% to 7.3% above OpenRocket's. OpenRocket writes a finish on every outside
part it saves, so this touches hand-written files and other tools' files. Until it is fixed, set a
finish on every outside part.

### Clusters

An inner tube can be a [cluster](../glossary.md#cluster): several like tubes side by side, each
holding a motor. The file names a pattern in `clusterconfiguration`, spreads it with
`clusterscale` (1 unless written) and turns it with `clusterrotation` (degrees, as every `.ork`
angle). HPR Sim reads every tube into the inner tube's list of places,
[`cluster_m`](../api/hpr_design/parts/struct.InnerTube.html#structfield.cluster_m), and the
motor in it becomes one motor per tube ([Clusters](../physics/design.md#clusters)).

No published document gives the patterns, so OpenRocket 24.12 was asked, run as an outside program
through its public interface (`validation/oracles/openrocket/clusters.py`, [ADR-075][adr-075]).
Each pattern is a figure of points `pₖ`, one per tube, in units of `2 R s` (defined below):

| pattern | tubes | figure |
|---|---|---|
| `single` | 1 | one tube |
| `double`, `3-row`, `4-row` | 2 to 4 | a row, one apart |
| `3-ring`, `4-ring`, `5-ring` | 3 to 5 | a triangle, a square, a pentagon, each of side one |
| `6-ring` | 6 | a hexagon of side one |
| `3-star`, `4-star`, `5-star`, `6-star` | 4 to 7 | a center tube and a ring of radius one |
| `9-grid` | 9 | a 3 × 3 grid, 1.4 apart |
| `9-star` | 9 | a center tube and eight on a ring of radius 1.4 |

The unit is `2 R s`, for the tube's outer radius `R` and the scale `s`: at scale 1 neighbouring
tubes touch, except in `9-grid`, whose rows and columns are 1.4 diameters apart, and `9-star`,
whose ring is 1.4 diameters from its center tube. The pattern is
turned by the tube's roll angle `θ` less the rotation `ρ` (`Rot` turns a point by that angle):

`[x, y]ₖ = 2 R s · Rot(θ − ρ) · pₖ`

measured from the tube's own radial offset. For example, a `3-ring` of 40 mm tubes at scale 1 puts
the three axes 23.09 mm from the center (`40 mm / √3`). OpenRocket's `(y, z)` are read as HPR Sim's
`(x, y)`, the same assumption as for every roll angle ([above](#which-way-round)).
On 25 probes (every pattern, a scale, a rotation, a radial offset, and all three at once) HPR Sim
puts every tube within 1e-15 m of where OpenRocket does (the test
`every_tube_of_a_cluster_is_where_openrocket_puts_it` in `hpr-validate`). A pattern name OpenRocket
doesn't know (`4-square`), it reads as one tube; so does HPR Sim, with a warning.

A cluster's mass and center of mass agree with OpenRocket's. Its inertia does not: OpenRocket weighs
the tubes as if stacked on the cluster's axis, and HPR Sim weighs each where it sits
([Mass properties](../physics/mass.md#clusters-and-fillets)). Two more readings differ from what
a builder would expect:

- A centering ring with an automatic bore beside a cluster takes one tube's radius as its bore, as
  OpenRocket gives it, so the tubes run through the ring and that mass counts twice. For a 3-ring
  of 40 mm tubes in a ring 98 mm across, the ring weighs about two thirds more than one with three
  holes would. The design checks warn of it (`ring_overlaps_inner_tube`).
- A part inside an inner tube set off the body's axis is read at its own offset from the body's
  axis, with no parent's offset added. OpenRocket places it from the tube's axis: an engine block
  in a tube 10 mm off the axis sits on that tube's axis in OpenRocket and on the body's axis in
  HPR Sim. The simulator warns of it (`Unusual`), and no file in the survey has one
  ([#181](https://github.com/nrdptel/fusionspace-eridanus/issues/181)). A cluster on the axis, the common case,
  is not affected, and neither is a motor in a mount that sits in the body tube, whose nozzle takes
  its mount's offset.

A configuration with a motor in a cluster flies with one motor in every tube. On OpenRocket's
*Clustered motors* example, four motors in a 4-ring, all five configurations are within 1% of
OpenRocket's apogee and largest speed, with the parachute-opening cause removed
([HPR Sim's flights against OpenRocket's](#the-simulators-flights-against-openrockets); the
[M1.9c milestone](../decisions-and-roadmap.md#m1-9c), which set a 5% bar before measuring).

### Pods

A [pod set](../physics/design.md#pods) hangs pods beside a body tube: each pod is the stack of nose
cones, body tubes and transitions written inside the `podset`, with everything on and in them, and
`instancecount` of them are spaced evenly around the axis. HPR Sim reads it into a
[`PodSet`][p-podset] ([M1.13b](../decisions-and-roadmap.md#m1-13b)). The first pod's roll angle is
`angleoffset`, read as every angle is ([which way round](#which-way-round)). Pods fly from
[M1.13c1](../decisions-and-roadmap.md#m1-13c1), each pod's parts with their own normal force and
drag ([aerodynamics: Pods](../physics/aero.md#pods)).

Five probe designs with pods of bodies, fins, a tail cone, winglets and motors, and the same
airframe without pods, fly within 0.81% of OpenRocket's apogee
([M1.13c2](../decisions-and-roadmap.md#m1-13c2), a pod design against OpenRocket). They are
listed apart from the designs, at the end of
[the report](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/openrocket-flights.md#pod-probes).

The one private design with pods, `C02`, has only a launch lug on each. So its flights, within
2.2% of OpenRocket's apogee
([HPR Sim's flights of the private designs](#the-simulators-flights-of-the-private-designs)), check
the pods' placement and weight, not their aerodynamics. Of OpenRocket's own two pod examples,
*Pods--airframes and winglets* flies since [M4.5g4](#fins-on-a-nose-cone-or-a-transition), the
milestone that reads its cockpit fin on the nose cone. *Pods--powered with recovery deployment*
flies since [M4.5i](../decisions-and-roadmap.md#m4-5i), the milestone that weighs its rail buttons'
screw heads and flies motors in more than one pod set ([ADR-168][adr-168]); its flights are
[below](#the-simulators-flights-against-openrockets).

**How far from the axis** depends on the `method` written on `radiusoffset`, and no document says
how. OpenRocket 24.12 was asked, as an outside oracle, on probe designs
(`validation/oracles/openrocket/pods.py`, eighteen of them). It puts each pod's axis at this
distance from the body's (OpenRocket's own names for the methods in brackets):

| `radiusoffset` method | distance | tube 50 mm in radius, pods 10 mm in radius, number 20 mm |
| --- | --- | --- |
| `relative` ("surface of the parent component") | tube radius + pod radius + the number | 80 mm |
| `surface` ("… without offset") | tube radius + pod radius; the number is ignored | 60 mm |
| `free` ("center of the parent component") | the number, from the axis | 20 mm, inside the tube |

The **pod radius** is the widest of the pod's own parts. One probe has a stated 10 mm nose, a
10 mm tube, a 15 mm tube and another 10 mm tube, and OpenRocket puts the pod at the 15 mm radius:
not the first part's, the first tube's or the last part's. HPR Sim's test
(`every_pod_is_where_openrocket_puts_it`, in `hpr-validate`) holds every part in every pod to
OpenRocket's place, to 10⁻¹⁵ m. An automatic radius inside a pod can only take a radius stated in
the pod, so HPR Sim takes the widest stated one.

HPR Sim fixes each pod's distance when it reads the file, so it cannot work out an automatic tube
radius later. When the tube's radius is automatic, HPR Sim uses the number OpenRocket cached for it
and warns that it did. With nothing cached, the pod set is left out. A method HPR Sim does not know
is read as `relative`, and a missing `radiusoffset` as touching the tube, each with a warning.

**A pod of no length.** OpenRocket's own example *Pods--airframes and winglets* hangs its
winglets from a pod whose only part is a tube of no length, no wall and, usually, no radius.
OpenRocket names that tube "(phantom body)". HPR Sim reads such a pod as written:

- The tube weighs nothing.
- The pod's radius is the tube's, usually 0, and the distance table above applies with it: for
  `relative`, the pod's axis is the body tube's radius plus the number out.
- Fins and a launch lug on the tube sit on its surface, as on any tube. With a radius of 0 that is
  the pod's own axis: the fins' roots meet there, and a lug's axis is the lug's own radius out
  from it, at the lug's angle. A lug 3 mm in radius on a pod 62 mm from the axis, turned to 180°,
  has its axis 59 mm from the body's.
- Everything turns with its pod, as in any pod.
- A tube of no length has no room inside and no wall. A pod set whose tube of no length holds a
  part inside it is left out, with a warning. A fin tab deeper than the tube's radius is dropped,
  also with a warning, and the fins are read without it.

OpenRocket 24.12 agrees on six probes:

- two fins at 90° on a tube of no radius;
- three fins on each of two pods, on a tube of no radius;
- two fins on a tube 10 mm in radius;
- two pods at 30° on a tube 10 mm in radius, three fins each at 20°, which shows each fin turned
  with its pod;
- a lug turned to 180°, and two at 0°.

Each fin's root and each lug's axis is where OpenRocket puts it, to 10⁻¹⁵ m (the test
`every_pod_is_where_openrocket_puts_it`, in `hpr-validate`).

**A pod set that holds nothing** is read too, and weighs nothing, in OpenRocket as in HPR Sim. A
mass override on one is dropped, with a warning. OpenRocket 24.12 puts that mass at the rocket's
tip, on its axis, where no design means it to be.

**What it weighs.** The test `pods_weigh_as_openrocket_s`, in `hpr-validate`, compares HPR Sim with
OpenRocket on every probe:

| quantity | how close to OpenRocket |
| --- | --- |
| each tube, fin set and lug in a pod: mass and center | 2 parts in 10¹⁵ |
| a pod's nose cone: mass, center | 8.3 parts in 10⁸, 4.0 parts in 10⁹ |
| the whole rocket: mass | 1.2 parts in 10⁷ |
| the whole rocket: center of mass | 1.10 parts in 10⁷ with pods that have a length, 1.13 parts in 10⁷ with pods of no length |
| roll inertia, with OpenRocket's fin shortcut in place of HPR Sim's | 2 parts in 10⁹ |
| pitch inertia about `x_B`, with no fins in the pods | 6.1 parts in 10⁷ |

OpenRocket weighs a nose cone a little differently from HPR Sim. The probes' own nose, on the
airframe, is 5.1 parts in 10⁷ apart in mass, and that alone puts the bare airframe 1.17 parts in
10⁷ from OpenRocket's. No probe with pods is further from OpenRocket than that.

HPR Sim works out a fin's roll inertia exactly, and OpenRocket takes a shortcut: the two differ by
up to 5% on a fin set ([mass: fins](../physics/mass.md#fins-rail-buttons-and-roll-inertia)).
The 2 parts in 10⁹ holds only once OpenRocket's shortcut is swapped in for HPR Sim's
([ADR-062][adr-062]: why fin roll inertia differs). OpenRocket's rule for a fin's pitch inertia is
not known, so where the pods hold fins the pitch gap is only pinned: 2.6 parts in 10⁷ to 3.4 parts
in 10⁵.

OpenRocket gives one pitch inertia for both pitch axes. With one or two pods the rocket is not the
same both ways, so HPR Sim's inertia about `y_B` differs from that one number, by 0.3% to 1.1% on
the probes of pods that have a length ([M1.13b1](../decisions-and-roadmap.md#m1-13b1)). HPR Sim's is
the true inertia about that axis.
With three pods the two agree.

**A flipped nose cone**, anywhere in a design, is a tail cone, and HPR Sim reads it as one: a
transition from the nose's base radius to a point. Its base is forward, so an automatic radius
takes the part ahead of it. With one end a point, a clipped and an unclipped transition are the
same whole nose shape ([shapes](../physics/shapes.md)). The only one in the corpus closes a pod.

**Left out, with a reason.** A pod set is left out with a `Skipped` warning when HPR Sim cannot lay
it out, in any of these cases:

- a pod holds a nose cone or transition of no length, or a part of negative length;
- a pod's tube of no length holds a part inside it;
- its pods' radii are all automatic;
- the tube's radius is automatic with nothing cached;
- the distance comes out negative;
- it hangs from anything but a body tube;
- it sits inside a pod.

The corpus has none of these. A part directly inside a pod set that is not a nose cone, body tube
or transition is left out on its own. An override on a pod set is its pods' total, since a pod set
weighs nothing of its own.

`cargo xtask ork` counts them. On the corpus, 2026-09-27:

| pod sets | count |
| --- | --- |
| written | 9 |
| read into `PodSet` | 9, holding 12 pods |
| left out | 0 |

`cargo xtask ork` on 2026-09-27 also weighs each design against OpenRocket 24.12:

- *Pods--powered with recovery deployment* (a public example): its mass is now within 1% of
  OpenRocket's. It was 12.5% light with its pods left out
  ([M1.13b1](../decisions-and-roadmap.md#m1-13b1)).
- *Pods--airframes and winglets*: with its winglet pod left out, its roll inertia was 1.78% below
  OpenRocket's, even with OpenRocket's fin shortcut. With the pod read, it is no longer among the
  designs `cargo xtask ork` lists outside 1% with that shortcut. Its freeform cockpit, on the nose
  cone, is read since [M4.5g4](../decisions-and-roadmap.md#m4-5g4)
  ([Fins on a nose cone or a transition](#fins-on-a-nose-cone-or-a-transition)).

### Parallel stages

**A `<parallelstage>` inside a body tube, on a rocket of one stage on the axis, is read as a stage
of its own, laid out as a pod set on that tube.** OpenRocket uses one for boosters strapped beside
the core. HPR Sim reads it since [M4.5m](../decisions-and-roadmap.md#m4-5m)
([ADR-171: a parallel stage is a pod set that separates][adr-171]). Its copies sit as a pod set's
pods do ([Pods](#pods)): `instancecount` of them, `radiusoffset` from the axis, the first at
`angleoffset`, at its axial position along the tube. Its parts weigh to its own stage, not to the
tube's, and its own `separationevent` drops it, read as an axial stage's is.

The design's side is in [Parallel stages](../physics/design.md#parallel-stages), and the flight's
in [Boosters beside the core](../physics/staging.md#boosters-beside-the-core). When its motors
light, and why one with no motor that lights stays on, is in
[Delays and ignition](#delays-and-ignition). The writer puts a parallel stage it read back inside
its tube.

**Left out, with a `Skipped` warning naming the reason.** HPR Sim leaves out a parallel stage:

- on a rocket of more than one stage on the axis: OpenRocket numbers the parallel stages among the
  axial ones, and no example or library design has one to check a reading against;
- inside a pod or another parallel stage;
- with no nose cone, body tube or transition in it;
- placed `after` the part before it, since a parallel stage sits along its tube;
- in any case where a pod set would be left out ([Pods](#pods)).

One written directly under `<rocket>`, as in Loft's `demo-quirks.ork`, is not read there, and
stays in the tally of tags HPR Sim does not read. OpenRocket 24.12 refuses to open that file. Every
parallel stage left out is kept whole for writing the file back
([what HPR Sim keeps](#what-the-simulator-keeps-for-writing-the-file-back)).

**Measured.** OpenRocket's *Parallel booster staging* example flies both its configurations as
saved. Under OpenRocket's stored conditions, recovery as saved
([the report](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/openrocket-flights.md)).
A configuration is named by its motors, stage by stage with the sustainer first, the stages
separated by `;`: `[I115W-10; 2× E12-0]` is an I115W-10 in the core above two E12-0s in the
boosters, and `None` is a stage with no motor.

| configuration | apogee, OpenRocket / HPR Sim (m) | Δ | largest speed Δ |
|---|---|---:|---:|
| `[I115W-10; 2× E12-0]` | 1123.6 / 1137.5 | +1.23% | +3.25% |
| `[I115W-10; None]` | 851.8 / 857.6 | +0.68% | +1.56% |

In the first, the boosters drop at 2.44 s, at the E12s' charge, as in OpenRocket. In the second,
they hold no motor and stay on. The survey's only other parallel stages are the library's copy of
the same example and `demo-quirks.ork`'s.

[adr-171]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0171-a-parallel-stage-is-a-pod-set-that-separates.md
[adr-172]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0172-a-stage-s-first-burnout.md

### What is left out, and why

A part HPR Sim cannot give an honest shape is **left out with a `Skipped` warning** naming the part
and the reason, rather than guessed at. The design still opens and still lays out; what is missing
is named, never silent. No part in the whole corpus is left out now, under these rules:

| rule | in the corpus |
| --- | --- |
| an external part other than a fin set on anything but a body tube: a lug's or a button's footing on a nose cone is not a straight line | none |
| a fin set on a nose cone or a transition whose root can't be read along the surface ([below](#fins-on-a-nose-cone-or-a-transition)) | none; the 2 fin sets it caught, the cockpit of *Pods--airframes and winglets* in the jar's copy and the corpus's, are read since [M4.5g4](../decisions-and-roadmap.md#m4-5g4) |
| a freeform outline on a body tube that does not end on the root, which would have to be closed along a body it never touches | none |
| a part whose automatic radius needs a bore its parent has not got: a filled nose cone, or a packed part inside a nose cone ([above](#inside-a-nose-cone-or-a-transition)) | none; the 1 coupler in a nose cone it caught is read since [M2.2e7](../decisions-and-roadmap.md#m2-2e7) |

Another rule left out the 2 tube fin sets whose radius OpenRocket sizes from the body, until
[M2.2e8](../decisions-and-roadmap.md#m2-2e8) worked that radius out
([Tube fins sized from the body](#tube-fins-sized-from-the-body)).

#### Fins on a nose cone or a transition

This covers how HPR Sim reads a fin set that sits on a nose cone or a transition, where the body's
surface is not level along the fin's root (the edge where the fin meets the body). It is checked
on one fin on one design, against OpenRocket only, and no measured flight checks it.

OpenRocket gives a fin's outline as points `(x, h)`: `x` aft of the root's leading edge and `h`
out from the body's surface there. The root runs along the surface. On a body tube the surface is
a straight line at `h = 0`; on a nose cone it curves. The cockpit of OpenRocket's *Pods--airframes
and winglets* is a single fin on the last 50 mm of a
[tangent ogive](../glossary.md#tangent-ogive) nose. Its outline is `(0, 0)`, `(9.35, 6.96)` and
`(50, 2.23)` mm: the last point is 2.23 mm up because the ogive's surface rises 2.23 mm over those
50 mm.

HPR Sim reads such a fin as it is written ([ADR-166][adr-166], fins on a nose cone):

- The outline is kept as written. The root is drawn through 63 points on the parent's profile
  between its ends, 64 straight pieces in all. On the cockpit each piece sits at most 0.14 µm
  inside the curve (`a_root_along_a_nose_cone_stays_within_its_documented_sag`).
- The fin's heights are measured from the radius at its root leading edge, which is also the
  body radius for its mass and its fin–body interference.
- The design's layout checks that every root point is within a micron of the surface, and that
  the straight pieces between them cut off or add no more than 0.1% of the fin's area.

**How well it matches.** Through OpenRocket's own 21 root points, HPR Sim's area, mass and center of
mass for the cockpit are OpenRocket's (1.44954e-4 m², 0.172 g), to the six digits OpenRocket
printed (`a_root_along_a_nose_cone_is_openrockets`). HPR Sim's 64 pieces give 0.029% less area,
nearer the curve. Closed straight along its chord instead, the cockpit would have 12.8% more area
than the file's. All five configurations of the example fly in `hpr sim` as saved. Its centering
rings wrap its 18 mm motor tube and fit the airframe around it
([a ring around its tube](../physics/design.md#wider-than-its-parent)):

| configuration | OpenRocket's apogee (m) | HPR Sim's (m) | difference |
|---|---:|---:|---:|
| [A8-3] | 32.3 | 32.3 | −0.13% |
| [B6-4] | 90.5 | 90.7 | +0.18% |
| [C6-5] | 199.7 | 200.7 | +0.48% |
| [C12-6] | 207.5 | 207.7 | +0.10% |
| [D16-6] | 245.7 | 246.0 | +0.10% |

These come from the committed comparison ([HPR Sim's flights against
OpenRocket's](#the-simulators-flights-against-openrockets)). The cockpit's drag counts: without it
the [C6-5] would reach 204.4 m.

**What disagrees.**

- **Stability margin.** As the rocket leaves the rod, HPR Sim's margin is 0.071 to 0.076 calibres
  above OpenRocket's on all five configurations. That is the flattering side, and it does not come
  from the cockpit: at the margin's wind direction the single fin lies in the airflow's plane, and
  neither code counts it. Probes put it on two older departures:
  - OpenRocket's fin–fin interference factor, taken across the airframe's fin sets
    ([#325](https://github.com/nrdptel/fusionspace-eridanus/issues/325));
  - the freeform wings' own center of pressure and normal force
    ([#326](https://github.com/nrdptel/fusionspace-eridanus/issues/326)).
- **Cockpit normal force.** Across the airflow, the cockpit's own normal force is 31.5% above
  OpenRocket's (0.2784 against 0.2117 per radian at Mach 0.3), with its center of pressure
  0.13 mm from OpenRocket's. The curved root isn't the cause
  ([#326](https://github.com/nrdptel/fusionspace-eridanus/issues/326)). It acts forward of the center of mass,
  so it shortens HPR Sim's margin, the safe side.

A fin set there is left out, with its reason, when:

- it is trapezoidal or elliptical (HPR Sim reads the root along the surface only from a freeform
  outline);
- it carries a tab or a fillet, whose models take a body tube;
- it is placed after a sibling or at an absolute station, where the reader can't find the
  surface;
- the body's radius is automatic, which the layout settles only after the reader draws the root;
- its root runs past the body's ends;
- its outline ends off the surface;
- the body narrows along its root (a boattail).

A design built in code that puts such a fin on a nose cone gets the same checks from the layout,
as a `DesignError::Geometry`, where the reader leaves the fin out with a warning. On a body tube an
outline that ends off the root is still left out. Writing the design back, HPR Sim writes the
outline alone, as OpenRocket does, so a root drawn through other points than the reader's comes back
through the reader's.

[adr-166]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0166-a-fin-root-that-follows-the-body.md
[adr-168]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0168-pods-powered-flies.md

One more thing is read as the simpler part HPR Sim models, with a warning so that what is missing
from a mass is visible: a **row** of more than one ring read as one. It is a measured departure,
not a silent compatibility claim ([ADR-064][adr-064]). A rail button's **screw head** (2) was
another until [M4.5i](../decisions-and-roadmap.md#m4-5i) weighed it
([Mass properties](../physics/mass.md#fins-rail-buttons-and-roll-inertia); [ADR-168][adr-168]).
A **cluster** of motor tubes was a third (4) until
[M1.9b](../decisions-and-roadmap.md#m1-9b) read every tube ([above](#clusters)). A fin's
**fillets** (5) were another until [M2.2e7](../decisions-and-roadmap.md#m2-2e7) weighed them as
OpenRocket does ([Mass properties](../physics/mass.md#fin-fillets)).

**A tube of no wall thickness carries no mass**, among them couplers in two of OpenRocket's own
example designs. Reading those as solid would invent the mass: a solid coupler filling a 50 mm
airframe for 180 mm is a few hundred grams the design never had. Since
[M2.2b1](../decisions-and-roadmap.md#m2-2b1) it is the rule for every part, and it is not warned
of: OpenRocket 24.12 gives an inner tube, coupler, lug, nose cone, transition, body tube or shoulder
of no wall no mass either, measured on probe designs ([ADR-061][adr-061]), and a solid body
component is written `<thickness>filled</thickness>`. An inner tube, coupler or lug that writes no
thickness at all is still read as no wall, with a warning; OpenRocket gives it a wall of its own,
and no file in the library has one ([Mass properties](../physics/mass.md#what-a-ork-leaves-unsaid-and-overrides)).

### Checked against the answers OpenRocket cached

`auto 0.0125` is not just a flag: the number is what OpenRocket itself last worked out (in
OpenRocket 24.12, at least; older releases may differ, as the note below explains). That makes
an oracle for the resolution rules that needs no OpenRocket, and `cargo xtask ork` runs it over the
corpus: on every automatic dimension that caches a number and sits on a component the file gives
an `<id>`. On 2026-09-27, with pods read, **71 of 75 agree** to a part in 10⁹; the 4 added since
2026-09-23 are the pods' nose cones, and all 4 agree. Of the 4 that disagree, the 2 body radii are settled in HPR Sim's favour
[below](#openrocket-settles-it); the 2 packed radii follow the bore of that tube, so they are settled
with it by the bore rule, not measured (the oracle reads body radii only).

**What that number is, measured.** OpenRocket 24.12 writes the radius it resolved, not the number
it read: a tube that says `auto 0.04` and has nothing to take is saved again as `auto 0.025`. It
also ignores the number when it opens a file ([below](#when-an-automatic-radius-has-nothing-to-take)).
One older statement disagrees. The 2021 change that started writing the number,
[OpenRocket PR #998](https://github.com/openrocket/openrocket/pull/998), calls it the "manual
value", the last one typed in. So a file saved by some release between then and 24.12 may cache a
hand-typed number rather than an answer. Which releases wrote which is not settled. That is one
more reason HPR Sim never reads the number as a radius.

It reads the number in one place only: as the wall of a `filled` tube whose radius is automatic but
reachable, which has no other number to be solid to, and says so in a warning. No file in the
corpus has one. (A tube whose radius takes OpenRocket's default is solid to that default instead,
[below](#when-an-automatic-radius-has-nothing-to-take).)

**What the oracle does not reach.** A cached answer only exists where OpenRocket wrote one, and it
never writes one for two of the tags that matter most here: across the whole corpus, `outerradius`
caches a number **0 times out of 131** and `innerradius` **0 out of 80**. So the 71 comparisons are
all `aftradius`, `foreradius`, `radius` and `packedradius`, and the two rules this milestone adds,
**an inner tube's automatic outer radius** and **a ring's automatic bore**, have *no oracle coverage
at all*. They rest on their unit tests and on the argument for them, until an OpenRocket oracle
reads those parts too: the one [below](#held-against-openrocket-itself) reads body radii only, and
[M2.2](../decisions-and-roadmap.md#m2-2) is where the rest belongs. `cargo xtask ork` prints the
per-tag denominators and names the tags nothing reaches, so the gap is in the report rather than
only here.
Since [M2.2e7](../decisions-and-roadmap.md#m2-2e7), fourteen probe designs run in OpenRocket do
reach an inner tube's automatic outer radius, a ring's and a bulkhead's, inside a nose cone, a
transition and a body tube ([above](#inside-a-nose-cone-or-a-transition)); the library's own
files still cache no such number.

The four cached numbers that disagree (not the four inside a pod) are one body tube and the
parachute packed inside it (whose radius follows the tube's bore), in **OpenRocket's own "Dual
parachute deployment" example**, which the corpus holds twice: once inside the jar and once cached
beside it. Its spine is a nose cone and four body tubes; the third tube *states* a radius of
0.028321 m, every automatic radius caches 0.028321 m too, and HPR Sim resolves them all to it,
except that the first tube caches 0.025 m.

<a id="openrocket-settles-it"></a>

**OpenRocket itself settles it, and agrees with HPR Sim.** Run on that file
([below](#when-an-automatic-radius-has-nothing-to-take)), OpenRocket 24.12 first reads the first tube
as 0.025 m, its default. Once it works the design out again, as it does when saving, it reads
0.028321 m, and writes that. The first reading depends on an unrelated part, the coupler inside
the *second* tube, whose own radius is automatic: the tenth and eleventh rows of the table below
are the same small design without and with such a coupler, and only the one with it is first read
at the default. So the 0.025 m in the file is most likely a first reading that an earlier save wrote out: the probe
reads before and after a save, not twice without one, and nothing yet checks which radius
OpenRocket's own simulation uses after a plain open ([M2.2](../decisions-and-roadmap.md#m2-2) will).
HPR Sim is held to the answer OpenRocket settles on.

<a id="held-against-openrocket-itself"></a>

**Held against OpenRocket itself.** `cargo xtask ork` also compares every body radius HPR Sim
resolves with the one OpenRocket 24.12 settles on for the same file, read from the oracle's
committed results in `validation/fixtures/ork/openrocket-automatic-radius.json`, and prints the
result on its line "body radii against OpenRocket 24.12 run on the same file". It covers 18 of the
19 files OpenRocket was run on: the 17 examples in its jar, and the parachute catalog below (it
refuses the 19th, below). **67 of 67
agree**: body radii this time, a different 67 from the cached numbers above. Unlike the cached
answers, this reaches every body radius, fixed or automatic, including the Dual parachute tube.

### When an automatic radius has nothing to take

Sometimes a chain of automatic radii has no fixed radius anywhere along it, so the
[neighbour rule](../physics/design.md#automatic-dimensions) has nothing to work from. For
example, a nose cone's base follows the tube behind it, and that tube follows the nose cone. Two
designs in the reference library do this. HPR Sim gives each such radius **OpenRocket's own default,
25 mm**, as a fixed radius, and raises a warning at its tag naming the radius. A document whose
`<rocket>` holds nothing at all is not a design, and is reported that way.

**If you see that warning,** the design has a radius its author never set. OpenRocket 24.12 shows a
tube there at 25 mm too, and a nose cone's base or a transition's end that looks at another
automatic radius at −1 m, which no shape can have. Neither is likely to be the rocket that was built. Set the radius in the design
(in OpenRocket, untick *Automatic* and type the diameter) and open it again.

**Why 25 mm.** OpenRocket's user guide doesn't say what happens here. Its issue tracker does:

- A maintainer: "OR returns the default radius"
  ([#1988](https://github.com/openrocket/openrocket/issues/1988#issuecomment-1397654629)).
- An open issue: a tube left with nothing to take "reverts to default diameter"
  ([#1992](https://github.com/openrocket/openrocket/issues/1992)).
- A user reports a nose cone that "may be retaining the default 1.969 in. base diameter"
  ([#871](https://github.com/openrocket/openrocket/issues/871)). That is 50.0 mm across, or 25 mm
  of radius, but it is a user's guess, so the number rests on the measurement below.

Since 2021, OpenRocket's dialogs grey out the checkbox where a radius would have nothing to take
([PR #998](https://github.com/openrocket/openrocket/pull/998)), though #1988 shows a later version
still leaving one ticked. So chains like these come mostly from older or hand-written files.

**Measured.** `validation/oracles/openrocket/automatic_radius.py` runs the OpenRocket 24.12 program
on fifteen small designs of its own and records the radius it gives each body component. It
records it twice: when the file is first opened, and once OpenRocket has worked the design out
again, as saving makes it do. The results are committed in
`validation/fixtures/ork/openrocket-automatic-radius.json`, and the test
`hpr_io::ork::tests::a_radius_with_nothing_to_take_is_openrockets_default` holds HPR Sim to them.

In the table, radii are listed forward to aft. "30 to 20" is a transition's forward and aft
radius, and "\|" is a stage boundary. OpenRocket's column is the settled answer.

| design | what the file says | OpenRocket 24.12 | HPR Sim |
|---|---|---|---|
| a tube | `auto` | 25 mm | 25 mm |
| a tube | `auto 0.04` | 25 mm | 25 mm |
| two tubes | `auto`, `auto 0.04` | 25, 25 mm | 25, 25 mm |
| a nose cone | `auto` | 25 mm | 25 mm |
| a nose cone | `auto 0.03` | 25 mm | 25 mm |
| a transition | `auto 0.03` to `auto 0.02` | 25 to 25 mm | 25 to 25 mm |
| a nose cone, a tube | `auto 0.03`, `auto 0.03` | **−1 m**, 25 mm | 25, 25 mm |
| a nose cone, a tube, a transition, a tube | `auto 0.033`, `auto`, `auto` to 22 mm, 22 mm | **−1 m**, 25 mm, **−1 m** to 22 mm, 22 mm | 25, 25, 25 to 22, 22 mm |
| a nose cone, a tube (the control) | `auto`, 30 mm | 30, 30 mm | 30, 30 mm |
| a nose cone, two tubes, a tube | `auto`, `auto`, `auto`, 30 mm | 30, 30, 30, 30 mm | 30, 30, 30, 30 mm |
| the same, a coupler of automatic radius in the third component | as above | 30, 30, 30, 30 mm (first read: 30, **25**, 30, 30) | 30, 30, 30, 30 mm |
| a tube, two tubes | 30 mm, `auto`, `auto` | 30, 30, 30 mm | 30, 30, 30 mm |
| a tube, a transition, a tube | 30 mm, 30 mm to `auto`, `auto` | 30, 30 to **−1 m**, 25 mm | 30, 30 to 25, 25 mm |
| a nose cone, a transition, a tube | `auto`, `auto` to 20 mm, 20 mm | **−1 m**, **−1 m** to 20, 20 mm | 25, 25 to 20, 20 mm |
| a nose cone, a tube \| a tube | `auto`, `auto` \| 30 mm | 30, 30 \| 30 mm | 30, 30 \| 30 mm |

What the table shows:

- **The number cached after `auto` is ignored.** OpenRocket gives the tube that caches 0.04 m
  25 mm, and HPR Sim does the same. The cache is an answer OpenRocket once wrote, never an input.
- **A chain that reaches a fixed radius takes it**, in every case probed (up to two automatic radii
  in between) and across a stage boundary too, in both programs. The default is only for a chain with nothing
  fixed on it.
- **HPR Sim departs from OpenRocket in one way, on purpose.** Where a nose cone's base or a
  transition's end looks at another automatic radius, OpenRocket gives it −1 m. No shape can have a
  negative radius, so HPR Sim gives it 25 mm too, and the chain is one radius end to end. That is 6
  of the 39 radii in the table.
- **OpenRocket's first reading can differ from its answer.** With a coupler of automatic radius in
  the third component, OpenRocket first reads the tube ahead of it at its default, then corrects it
  to 30 mm when it works the design out again. That is the Dual parachute example's cached 25 mm
  ([above](#checked-against-the-answers-openrocket-cached)).

**Worked example.** The eighth row is the chain in [Loft][loft]'s quirks fixture, on a small copy
the oracle script `automatic_radius.py` writes itself: a nose cone whose base is automatic (it
caches 0.033 m), an automatic tube, and a transition whose forward end is automatic and whose aft
end is fixed at 22 mm, then a 22 mm tube. Each automatic radius follows another automatic radius. A
transition's two ends never follow each other, so the fixed 22 mm stops at the transition and none
of the three ever reaches it. (The same is why, in the thirteenth row, a transition's fixed forward
end doesn't reach its automatic aft end.)

HPR Sim gives all three 25 mm. The nose cone ends at 25 mm, the tube is 25 mm, and the transition
narrows from 25 mm to 22 mm. The cached 0.033 m is not used, and three warnings name the tags.

**A tube's wall is judged against the default.** A tube whose automatic radius takes the default
has its wall read again now there is a radius to read it against, by the rule a stated radius
gets: `filled`, or a wall at least as thick as 25 mm, is solid to 25 mm. The test
`a_tube_given_the_default_has_its_wall_judged_against_it` holds that.

**The three files this settles** (`cargo xtask ork` prints the counts):

| file | what it holds | what happens |
|---|---|---|
| [Debrief][debrief]'s `sample-design.ork` | a `<rocket>` with a name, a comment and nothing else, plus a stored simulation | holds no design; counted apart, not as a failure. Its stored simulation is read ([stored simulations](#what-openrocket-last-did-stored-simulations)) |
| the `openrocket-database` parachute catalog | four tubes, every radius a bare `auto`, carrying the catalog's parachutes | lays out, four tubes at 25 mm, just as OpenRocket 24.12 opens it |
| [Loft][loft]'s `demo-quirks.ork` | the worked example's chain, and a parallel stage placed directly under the rocket | lays out as in the worked example. **OpenRocket 24.12 will not open this file**: it refuses a parallel stage there, so its answers for this chain come from the oracle script's copy. HPR Sim opens it and skips the parallel stage with a warning: HPR Sim reads a parallel stage only inside a body tube ([Parallel stages](#parallel-stages)) |

How it was decided, and the sources quoted in full, are in [ADR-054][adr-054].

### Measured on the reference library

`cargo xtask ork`, over the 73 readable files, on 2026-09-23, with the rows for parts left out
and warnings raised from 2026-09-28:

| | |
|---|---|
| designs whose `Rocket` lays out | 72 of the 72 files that hold a design |
| documents that hold no design | 1, among the 73 readable files |
| automatic radii given OpenRocket's default, 25 mm | 7, in 2 designs: 5 on body tubes, 1 on a nose cone, 1 on a transition |
| body radii against OpenRocket 24.12 run on the same file | 67 of 67 agree, over 18 of the 19 files it was run on |
| body components | 270 |
| parts on and inside them | 752 |
| by kind | 194 centering rings, 156 inner tubes, 132 parachutes, 104 fin sets, 81 mass components, 40 shock cords, 27 launch lugs, 16 rail buttons, 2 streamers |
| automatic dimensions marked for the layout to resolve | 320, plus the 7 above given the default: 327 in the files |
| parts left out, with a reason | 2; 4 before [M2.2e8](../decisions-and-roadmap.md#m2-2e8) read tube fins sized from the body, 5 before [M2.2e7](../decisions-and-roadmap.md#m2-2e7) read a coupler inside a nose cone |
| parts that lay out weighing nothing | 14, every one explained (below) |
| warnings raised | 16: 2 dropped, 5 skipped, 9 unusual (below); 18 before tube fins sized from the body were read ([M2.2e8](../decisions-and-roadmap.md#m2-2e8)), 21 before fin fillets and the coupler inside a nose cone were read ([M2.2e7](../decisions-and-roadmap.md#m2-2e7)), 31 before the old override flag was read as OpenRocket reads it ([M2.2e6](../decisions-and-roadmap.md#m2-2e6)), 35, with 12 skipped, before pods were read ([Pods](#pods)) |
| tags no milestone reads yet | none since parallel stages are read ([Parallel stages](#parallel-stages)); 3 `parallelstage` before, and 9 `podset` too before pods were read ([Pods](#pods)) |

**The 14 parts that weigh nothing** are worth checking, because a structural part with no mass is
silent by nature: the design lays out, the report is written, and the mass is simply missing. All
14 are accounted for: 7 inner tubes (couplers among them) and 4 launch lugs whose wall the file
states as zero, which OpenRocket gives no mass too ([ADR-061][adr-061]); 1 mass object the file
says weighs 0 kg; and 2 transitions the designs *override* to zero mass:
which is OpenRocket's ["base drag hack"](https://openrocket.readthedocs.io/en/latest/), a massless,
dragless transition added only to change the base geometry. `cargo xtask ork` counts them by kind,
so a new one would show up. Before
[M2.2b1](../decisions-and-roadmap.md#m2-2b1) there were 21: the 7 more (2 body tubes, 2 fin sets,
2 inner tubes and a nose cone) name no material, and now take OpenRocket's default.

<a id="what-the-warnings-are"></a>**What the 10 warnings are.** Every one is a reading this page
explains, and none of them means a file is broken. Before
[M4.5m](../decisions-and-roadmap.md#m4-5m) read parallel stages there were 12. Before
[M4.5i](../decisions-and-roadmap.md#m4-5i) weighed a rail button's screw head there were 14.
Before [M4.5g4](../decisions-and-roadmap.md#m4-5g4) read fins on a nose cone there were 16. Before
[M2.2e8](../decisions-and-roadmap.md#m2-2e8) read tube fins sized from the body there were 18.
Before [M2.2e7](../decisions-and-roadmap.md#m2-2e7) weighed fin fillets and read an
automatic radius inside a nose cone there were 21. Before [M2.2e6](../decisions-and-roadmap.md#m2-2e6) read the old override flag as
OpenRocket does there were 31, before [M1.13b](../decisions-and-roadmap.md#m1-13b) read pods 35, before
[M1.9b](../decisions-and-roadmap.md#m1-9b) read clusters 39, and
before [M2.2b3](../decisions-and-roadmap.md#m2-2b3) 57: 5 more for a
`packedradius` the file does not give, read as zero, which HPR Sim now reads as OpenRocket's 12.5 mm
([packed parts](../physics/mass.md#packed-parts)).

| kind | count | what raised it |
|---|---|---|
| `Unusual` | 1 | a part with no axial offset |
| `Unusual` | 7 | automatic radii with nothing along their chains to take, given OpenRocket's default ([above](#when-an-automatic-radius-has-nothing-to-take)) |
| `Unusual` | 1 | a `<rocket>` holding nothing, so the document holds no design |
| `Skipped` | 1 | a tally of the tags HPR Sim does not read where they are written, kept in `x-openrocket`, one per design that has any: the parallel stage directly under the rocket in `demo-quirks.ork` ([Parallel stages](#parallel-stages)); 3 before parallel stages were read, 7 before pods were read |

Every design that holds a design lays out. The one readable document that doesn't is Debrief's demonstration file, which holds
no design at all: it is a stored simulation with a rocket's name on it. The two designs that needed
a radius the file doesn't give are
[above](#when-an-automatic-radius-has-nothing-to-take).

[adr-054]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0054-an-automatic-radius-with-nothing-to-take-is.md
[adr-053]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0053-the-parts-on-and-inside-a-ork-body-degrees-what.md
[techdoc]: https://openrocket.sourceforge.net/techdoc.pdf
[dialog]: https://openrocket.readthedocs.io/en/latest/_images/body_tube_config.png
[finishes]: https://openrocket.readthedocs.io/en/latest/user_guide/overrides_and_surface_finish.html
[forum]: https://www.rocketryforum.com/threads/open-rocket-finishes.57558/post-585716
[issue-133]: https://github.com/nrdptel/fusionspace-eridanus/issues/133
[finish-api]: https://hpr.fusionspace.co/api/hpr_design/finish/enum.Finish.html
[p-inner]: https://hpr.fusionspace.co/api/hpr_design/parts/struct.InnerTube.html
[p-podset]: https://hpr.fusionspace.co/api/hpr_design/parts/struct.PodSet.html
[p-stage]: https://hpr.fusionspace.co/api/hpr_design/tree/struct.Stage.html
[p-parallel]: https://hpr.fusionspace.co/api/hpr_design/tree/struct.ParallelStage.html
[p-ring]: https://hpr.fusionspace.co/api/hpr_design/parts/struct.CenteringRing.html
[p-fins]: https://hpr.fusionspace.co/api/hpr_design/fins/struct.FinSet.html
[p-tubefins]: https://hpr.fusionspace.co/api/hpr_design/fins/struct.TubeFinSet.html
[p-lug]: https://hpr.fusionspace.co/api/hpr_design/parts/struct.LaunchLug.html
[p-button]: https://hpr.fusionspace.co/api/hpr_design/parts/struct.RailButton.html
[p-mass]: https://hpr.fusionspace.co/api/hpr_design/parts/struct.MassComponent.html
[p-chute]: https://hpr.fusionspace.co/api/hpr_design/parts/struct.Parachute.html
[p-streamer]: https://hpr.fusionspace.co/api/hpr_design/parts/struct.Streamer.html
[p-cord]: https://hpr.fusionspace.co/api/hpr_design/parts/struct.ShockCord.html
[p-position]: https://hpr.fusionspace.co/api/hpr_design/tree/enum.Position.html
[p-layout]: https://hpr.fusionspace.co/api/hpr_design/tree/struct.Rocket.html#method.layout

## Motors and their configurations

**In short.** HPR Sim reads every motor a design names, in every configuration, with when it lights
and its ejection delay. It finds the thrust curve in the file itself, in curves a caller supplies,
or in HPR Sim's bundled catalog. A configuration becomes one the rocket can fly only when every
motor in it has a curve and lights at a moment HPR Sim can fly. Most designs in the reference
library name motors the bundled catalog doesn't hold, so **4 of their 170 configurations fly** with
HPR Sim alone, and **145** with OpenRocket's motor database supplied (`cargo xtask ork` counts it).
The rest are read, kept, and say why not.

A **configuration** is one set of motors to fly the design with: OpenRocket calls it a *flight
configuration*, and a design can have several, one per motor choice. The file keeps it in two
places:

- `<rocket>` declares each one: a `configid`, a name, whether it is the one OpenRocket opens with
  (`default="true"`), and which stages fly.
- Each **motor mount** (a body tube or inner tube holding a motor) holds a `<motor configid="…">`
  per configuration: the manufacturer, the designation such as `H148R`, a `digest` (OpenRocket's
  fingerprint of the thrust curve's data), the case diameter and length, and the ejection delay. The mount also says when its motor lights, and may
  say it differently for each configuration.

So HPR Sim collects each mount's motor into its configuration. A mount that names a configuration
the rocket never declares still gets one, with a warning ([L65](../decisions-and-roadmap.md#l65)).

### Where the thrust curve comes from

A `<motor>` names a motor; it does not describe one. HPR Sim looks for its thrust curve in three
places, in this order:

1. **Inside the file.** From schema 1.11 (the `version` on the file's `<openrocket>` element),
   OpenRocket can save each motor's curve in the archive as `thrustcurves/<digest>.rse`, named by
   the `digest` the `<motor>` gives ([L57](../decisions-and-roadmap.md#l57)). That is exactly the
   curve the design was saved with. OpenRocket 24.12 still writes 1.10, so few files carry one yet:
   one file in the reference library does.
2. **Curves a caller supplies**, each for one digest, through
   [`design_with`](../api/hpr_io/ork/fn.design_with.html). A supplied curve is never matched by
   name, only by the digest. HPR Sim ships none: the validation survey supplies OpenRocket's own
   motor database ([motors in the reference library](#motors-in-the-reference-library)).
3. **HPR Sim's bundled catalog**: [32 motors from ThrustCurve.org](../physics/motor.md). The
   manufacturer and the designation must both match, ignoring case, spaces and hyphens. Both are
   needed: an Estes `B4` is not a Quest `B4`.

OpenRocket's own order is the other way round: its motor database first, then the curve in the
file, because the database "may have more accurate or updated data" (its file specification).
HPR Sim's catalog is far smaller than that database, and the curve in the file is the exact one, so
HPR Sim reads the file first. So on a file that carries curves, HPR Sim and OpenRocket can fly
different curves for the same motor.

HPR Sim builds the motor from the curve file's header (its case size and its loaded and propellant
masses) as it does for any catalog motor, and does not use the mass or center-of-gravity column
listed beside each thrust point. Where the design's case size and the curve's differ by more than a
millimeter, a warning says so: the design's size places the motor, and the curve's gives its mass. A motor found in none of the three places is kept with its reason. Nothing is
invented for it. A hybrid motor (solid fuel burned with a liquid or gas oxidiser) never gets a
curve: HPR Sim flies commercial solid motors only.

To fly a motor none of the three places has, build the configuration yourself: read its `.eng` or `.rse` file
with `hpr_motor` ([Solid motors](../physics/motor.md)) and put it in an
`hpr_design::Configuration`.

### Delays and ignition

| the file says | it means | source |
|---|---|---|
| `<delay>6.0</delay>` | the ejection charge fires 6 s after burnout | OpenRocket's [technical documentation][techdoc], p. 8 |
| `<delay>0.0</delay>` | the charge fires at burnout | the same, p. 10: "zero-delay motors" |
| `<delay>none</delay>` | plugged: no ejection charge | the same, p. 8 ("P … stands for plugged"); OpenRocket [issue #2002](https://github.com/openrocket/openrocket/issues/2002) |
| `<ignitionevent>automatic</ignitionevent>` | the bottom stage lights at launch; a stage above lights at the ejection charge of the stage below | OpenRocket's [FAQ](https://wiki.openrocket.info/FAQ), "How do I create a staged rocket?" |
| `launch`, `burnout`, `ejectioncharge`, `never` | at launch; at the first burnout or ejection charge of the stage below; never | OpenRocket 24.12's labels, with every word measured by a committed probe ([below](#what-the-words-mean)). A word HPR Sim does not know is kept as written |
| `<ignitiondelay>1.5</ignitiondelay>` | 1.5 s after that event | the [technical documentation][techdoc], section 4.2.6 |

A `0` means something different here than in a motor file. An `.eng` file has no word for plugged,
so HPR Sim reads its `0` as "zero or plugged" ([ejection delay](../glossary.md#ejection-delay)). A
`.ork` has `none` for plugged, and OpenRocket flies a `0` as a charge at burnout: in its own
"Parallel booster staging" example, the E12-0's burnout and ejection charge are stored at the same
2.44 s. But until OpenRocket 23.09 put *plugged* in its delay list
([issue #2090](https://github.com/openrocket/openrocket/issues/2090)), few authors knew to type
`none`, and some used `0` to mean plugged, as OpenRocket's own examples did
([issue #2111](https://github.com/openrocket/openrocket/issues/2111)). So a `0` in an older design
may be meant as plugged; HPR Sim reads it as the file says, as OpenRocket does. 23 motors in the
reference library have one.

A configuration's own `<ignitionconfiguration>` replaces the mount's event and delay one at a time:
whichever it leaves out, the mount's own value stands.

HPR Sim flies each motor's ignition as the file says it. Here `d` is the ignition delay, and a
missing delay is 0. "The stage below" is the next stage aft on the axis, since a parallel stage
hangs beside its stage, not below it. Its burnout is its first motor's: the first of its motors to
burn out, by each one's ignition time plus its curve's burn time, whichever mount holds it, and the
first in the file on a tie. The rules are also in the
[`hpr_io::ork::staging`](../api/hpr_io/ork/staging/index.html) module's documentation.

| the file says | HPR Sim lights the motor |
|---|---|
| `launch` plus `d` | `d` seconds after launch |
| `automatic` plus `d`, in the bottom stage | `d` seconds after launch (an [air start](../glossary.md#air-start) when `d` is more than 0) |
| `automatic` plus `d`, in a stage above | as `ejectioncharge` |
| `ejectioncharge` plus `d` | at the burnout of the stage below's motor, plus that motor's ejection delay, plus `d`; refused when the stage below has motors in more than one mount |
| `burnout` plus `d` | `d` seconds after the stage below's first burnout: the first of its motors to burn out, by each one's ignition and curve |
| `never` | never: the motor rides loaded, with no thrust |
| `burnout` or `ejectioncharge`, in the bottom stage | never, since it has no stage below |
| `ejectioncharge` or `automatic`, when the stage below's motor is plugged | never, since a plugged motor fires no charge |
| `automatic` plus `d`, in a [parallel stage](#parallel-stages) hung on the last stage on the axis | `d` seconds after launch, as OpenRocket lights the E12s of its *Parallel booster staging* with the I115W at 0 s |
| `launch` plus `d` or `never`, in a parallel stage | as written |
| any other event in a parallel stage | refused by name: HPR Sim has no reading for it |

A motor that never lights is flown as OpenRocket flies it: carried with all its propellant, giving
no thrust. So is a motor that waits on one that never lights: that follows from the probes, since
a motor that never lights has no burnout or charge, but no probe chains two. A stage that
separates at the burnout or ejection charge of its own motor, when that motor never lights, never
separates. A parallel stage with no motor that lights stays on too: its `burnout` or `ejection`
separation never comes, as in configuration 2 of *Parallel booster staging*, where OpenRocket
records no `STAGE_SEPARATION` event and the boosters land with the core
([ADR-171][adr-171]). `upperignition` as a parallel stage's separation is refused by name.

These rules come from a committed probe, `validation/oracles/openrocket/unlit_motors.py`
([M2.2e10](../decisions-and-roadmap.md#m2-2e10)). It flies OpenRocket's *Two stage high power
rocket* example, an I59WN over an I357T, five ways. In three, the booster never lights (set
`never`, or at `burnout` or `ejectioncharge` in the bottom stage) and the sustainer lights at
launch. In two, the booster is plugged and lights at launch, and the sustainer waits on its charge
(`ejectioncharge`, or `automatic`). Each time OpenRocket lights one motor and separates nothing;
the booster's separation is set at its own motor's `burnout` and `ejection` in two of them, which
measures that rule, and `never` in the other three.
By apogee the rocket has lost exactly that motor's propellant: the sustainer's 0.272 kg, or the
booster's 0.1792 kg. The unlit one keeps all of its own. Two controls fly the example as written,
and with the booster not plugged, where its charge lights the sustainer 14 s after its burnout.
One exception is recorded apart: with an H148R-0 in both stages, OpenRocket's mass column loses
nothing while a motor burns ([#185](https://github.com/nrdptel/fusionspace-eridanus/issues/185)). The test
`openrocket_flies_a_motor_whose_ignition_never_comes_unlit` in `crates/hpr-io/src/ork/tests.rs`
holds the record.

A configuration is still not flown when a motor names a word HPR Sim does not know, or waits on a
stage below with no motor (not yet probed), or on the charge of a motor that states no delay. It is
not flown either when a motor waits on the charge of a stage below with motors in more than one
mount (`ejectioncharge`, or `automatic` in a stage above), since which fires its charge first is not
measured, or when no motor of it lights at all, since the rocket would not leave the pad. Its
reason is *a motor HPR Sim can't light as written*.

**A stage's burnout is its first motor's.** A second committed probe,
`validation/oracles/openrocket/first_burnout.py`, shows it
([M4.5n](../decisions-and-roadmap.md#m4-5n), a stage's first burnout; [ADR-172][adr-172]). It
flies the first configuration of OpenRocket's *Pods--powered with recovery deployment*, whose
booster holds a motor in its main tube and one in each of two pods, with nothing deployed. As
saved, the main tube's B6 burns out first, at 0.86 s, so it can't tell the first burnout from the
main tube's. With a C6 in the main tube instead, burning out at 1.86 s, OpenRocket separates the
booster and lights the sustainer at the pods' burnout, 1.01 s. With C6s in the pods instead, it
does both at the B6's 0.86 s. So the stage burns out with its first motor, whichever mount holds
it, not with the main tube's or the last. The record is
`validation/fixtures/ork/openrocket-first-burnout.json`.

### Which configurations the rocket flies

HPR Sim reads each motor's ignition ([above](#delays-and-ignition)) and each stage's separation
([below](#when-parachutes-open-and-stages-separate)) into the flight, since the
[M1.9c milestone](../decisions-and-roadmap.md#m1-9c) (decision record
[ADR-076, a `.ork` file's ignitions and one powered separation][adr-076]). A configuration becomes
one of the rocket's only when all of these hold. Otherwise flying it would be wrong, for example
lighting a sustainer on the pad.

- Every motor has a thrust curve, and a case diameter and length.
- Every motor lights at a moment HPR Sim can fly: at launch, at a time after launch, or after the
  stage below burns out or fires its charge. Or it never lights, as
  [above](#delays-and-ignition).
- No motor sits in a part HPR Sim doesn't read yet, such as a parallel stage on a rocket of several
  stages or a pod set HPR Sim could not lay out ([M3.1c4](../decisions-and-roadmap.md#m3-1c4)).
  A motor in a parallel stage HPR Sim reads flies ([Parallel stages](#parallel-stages)). A motor in
  a [cluster](../glossary.md#cluster) of motor tubes flies with one motor in every tube
  ([above](#clusters)).
- No stage is switched off in the configuration's own stage list
  (`<stage number="1" active="false"/>`). OpenRocket leaves a switched-off stage out of the flight,
  and HPR Sim flies every stage.
- The rocket and its motor mounts were read without a single warning: nothing left out (a pod, a
  parallel stage HPR Sim can't read, a part HPR Sim could not shape), nothing dropped or simplified (a ring written as
  a row read as one, a material that could not be read), and
  nothing assumed (a shape HPR Sim does not know read as a cone).
  Otherwise HPR Sim might fly a different rocket from the design, so no configuration of it is
  flown. One design in the library was held back only by its shoulders of no wall; since
  [M2.2b1](../decisions-and-roadmap.md#m2-2b1) reads them as OpenRocket does, it flies.
- No mount holds two motors for the configuration. Which one OpenRocket would fly is not known, so
  neither flies.
- Its stages come apart in a way HPR Sim can fly. Each separation is flown, from the tail forward,
  and each one's time must be known before the flight: a time after launch, or after a motor lights, burns out or
  fires its ejection charge. At that time a motor ahead of it must still be burning or yet to
  light, and no motor behind it may be. That is a powered separation, which drops the stages
  behind it and flies the rest on as a sustainer
  ([Several separations](../physics/staging.md#several-separations)). One exception: a separation
  at its own stage's first `burnout` may drop the stage's own other motors still burning, as
  OpenRocket does. It drops only motors lit strictly before the split, and only when the split is
  powered: a motor ahead of it burns then, or lights then or later. The reader marks such a
  staging `drops_burning`, and HPR Sim's flight drops a burning motor only for a marked staging. A
  delay after that burnout counts too, though OpenRocket's probe measured only a split with none.
  The dropped part flies with no thrust, and `hpr sim` notes the impulse it leaves out
  ([A booster dropped still burning](../physics/staging.md#a-booster-dropped-still-burning)). A
  motor behind it still to light, or lit at the split itself, is refused, and so is any other
  separation while a motor behind it burns. Any other separation must come only at apogee or on
  the way down; the configuration then flies whole
  ([below](#when-parachutes-open-and-stages-separate)). A separation at the burnout or charge of the stage's own motor that never lights never comes, and
  is left out ([above](#delays-and-ignition)). Otherwise its reason is *stages HPR Sim can't
  separate as written*: a separation at or after apogee beside another, one that comes before the
  separation behind it, a negative delay, a sustainer already burnt out at the
  split, a separation at launch, or one at the ignition of a motor that never lights, for example.
  A separation timed after launch but before the rocket leaves the rod fires only as it leaves:
  HPR Sim's flight fires none on the rod
  ([#231](https://github.com/nrdptel/fusionspace-eridanus/issues/231)).

Every other configuration is still read, whole, with the first reason it can't be flown. The
motors' reasons are checked first, each across every motor, and the two about the whole rocket
last.

This is the doctest on
[`hpr_io::ork::design`](../api/hpr_io/ork/fn.design.html), which CI runs. The Estes F15 has no
curve in the file, so it comes from the bundled catalog: 49.6 N·s over 3.45 s, as ThrustCurve.org
lists it. The rocket assembles with it. `assemble` takes the configuration's `configid`; its name
is for display, and may be one of OpenRocket's templates, such as `[{motors}]`.

```rust
let read = hpr_io::ork::read(xml)?;
let design = hpr_io::ork::design(&read.value).value;

// The F15 has no curve in this file, so it comes from the bundled catalog...
let motor = &design.motors.configurations[0].motors[0];
assert!(matches!(motor.curve, hpr_io::ork::Curve::Catalog { .. }));
assert_eq!(motor.delay, Some(hpr_motor::Delay::Seconds(4.0)));
let impulse_ns = motor.curve.motor().expect("a curve").curve().total_impulse_ns();
assert!((impulse_ns - 49.61).abs() < 0.01, "{impulse_ns}");

// ...and it ignites at launch, so the configuration is one the rocket flies.
let assembly = design.rocket.assemble("c1")?;
assert_eq!(assembly.motors[0].mount, "body");
```

### How many configurations `hpr sim` flies

**In short.** `hpr sim` flies 136 of the survey's 170 motor configurations as saved, offline
after one fetch, and 9 more with `--accept-design-errors`; 25 are refused. These are counts, not
accuracy. A configuration counted as flying is one `hpr sim` accepts and flies to the ground. How
close each flight comes to OpenRocket's is
[HPR Sim's flights against OpenRocket's](#the-simulators-flights-against-openrockets).

The survey is every `.ork` that [Checked against real files](#checked-against-real-files) reads.
That is the design library under `refs/`, which is other people's designs and not in the
repository, and the 17 examples inside OpenRocket 24.12's jar. A *motor configuration* is one set
of motors to fly a design with ([above](#motors-and-their-configurations)). A file found in two
places is counted in each.

The three counts of the same 170 differ by where the thrust curves come from:

| Curves from | Configurations that fly | Counted by |
|---|---:|---|
| The file and HPR Sim's bundled catalog alone | 4 | `cargo xtask ork` |
| Those, and ThrustCurve.org's, as `hpr sim` fetches them | 136, and 9 more with `--accept-design-errors` | `cargo xtask ork-cli` |
| Those, and OpenRocket's own motor database | 145 | `cargo xtask ork` |

HPR Sim doesn't ship OpenRocket's database, so 136 is what a user of `hpr sim` gets. The 145 is the
validation survey's count ([Motors in the reference library](#motors-in-the-reference-library)).

The 136 come from [the committed report][ork-cli-report], which `cargo xtask ork-cli` writes
([the CLI's corpus count, M4.5j](../decisions-and-roadmap.md#m4-5j)). It runs each configuration
through the same function the `hpr` command runs, as `hpr sim --json --offline --config ID FILE`.
A first pass with the network caches every curve `hpr sim` fetches
([Where the thrust curve comes from](#where-the-thrust-curve-comes-from)), and the counts are
taken offline after it.

A configuration refused as saved is flown again with `--accept-design-errors`. The 9 that fly
then were refused only because the design's own [checks](../physics/design.md#checks) found an
error, such as a ring or tube wider than the tube it sits in. Their flights list those errors in their
notes ([The command line](../cli.md)).

Of OpenRocket's 56 example configurations, 54 fly as saved, 0 more with
`--accept-design-errors`, and 2 are refused. *Deployable payload*'s five fly with warnings: its
payload and its parachute are drawn 25 mm across in a 21 mm bore, so they can't fit as drawn, but
a packed part's width sets only its own inertia
([a packed part wider than its bore](../physics/design.md#a-packed-part-wider-than-its-bore),
[M4.5l](../decisions-and-roadmap.md#m4-5l)). *Parallel booster staging*'s two fly since
[M4.5m](../decisions-and-roadmap.md#m4-5m) read its boosters
([Parallel stages](#parallel-stages)). *Pods--powered with recovery deployment*'s first flies
since [M4.5n](../decisions-and-roadmap.md#m4-5n) read its booster's burnout as its first motor's
([Delays and ignition](#delays-and-ignition)). The 2 refused are *Simulation extensions* and
*Simulation scripting*, for a hybrid motor: HPR Sim flies commercial solid motors only until release
1.0.

Most of the library's refusals are motors with no curve whose file records no
[digest](#motors-in-the-reference-library), OpenRocket's fingerprint of the curve. `hpr sim`
supplies a fetched curve only by its digest, so it fetches none for them. The report gives every
reason's count.

Counting again needs the private library under `refs/` and, once, the network. Run it after any
change to the reader or to `hpr sim`:

```sh
cargo xtask ork-cli --fetch   # once, with the network: fills the motor cache
cargo xtask ork-cli           # offline: writes validation/reports/ork-cli-flights.{json,md}
cargo xtask ork-cli --check   # fails unless the committed report is still what hpr sim flies
```

[ork-cli-report]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/ork-cli-flights.md

### Motors in the reference library

**In short.** Most designs in the reference library don't carry their motors' curves. They name each
curve by its digest, OpenRocket's fingerprint (a hash) of the curve's data, and OpenRocket finds
the curve in the motor database that ships inside its program. The validation survey supplies that
database to HPR Sim, so 145 of the 170 configurations fly instead of 4. Every curve involved matches
OpenRocket's total impulse, and in every configuration HPR Sim flies with a supplied curve,
OpenRocket places that curve too. What still differs is where the motor's weight sits (below).

How it works:

- An oracle, `validation/oracles/openrocket/motor_database.py`, records OpenRocket 24.12's own
  database: 1,452 motors.
- `cargo xtask ork` hands each solid motor's curve to the reader for its digest, through
  [`design_with`](../api/hpr_io/ork/fn.design_with.html) and `SuppliedCurves`. A curve the file
  embeds still comes first, and HPR Sim's bundled catalog last.
- A supplied curve is never matched by name. In the database, 286 manufacturer-and-designation
  pairs have more than one curve.
- The database's curves come from ThrustCurve.org by way of OpenRocket, and neither publishes terms
  for reusing them. So the record stays on the machine that ran it, and only counts are published
  here.

To repeat the survey you need the private design library under `refs/`, which most readers won't
have, plus Java 17 and the OpenRocket jar (`cargo xtask refs fetch`), as for the
[mass comparison](../physics/mass.md#checked-against-openrocket). Then, from the repository root:

```sh
refs/venv/bin/python validation/oracles/openrocket/motor_database.py \
    corpus-out/openrocket-motors.json refs --jar
cargo xtask ork
```

`cargo xtask ork`, over the 72 designs, with that record, on 2026-10-05, after
[M4.5n](../decisions-and-roadmap.md#m4-5n) read a stage's first burnout. The counts are of **motors**, one per mount per configuration:

| motors | count |
|---|---|
| read into their configurations | 208: 138 single-use, 65 reloads, 3 hybrids, and 2 not written, read like the rest |
| left out, in parts not read yet | 0; 2, both in parallel stages, before they were read ([Parallel stages](#parallel-stages)), and 6 before pods were read ([Pods](#pods)) |
| thrust curve from the file itself | 4 |
| thrust curve from OpenRocket's database, by digest | 178 |
| thrust curve from the bundled catalog | 1 |
| no curve | 25: 3 hybrids, and 22 with no curve in any of the three places |
| ejection delays | 132 in seconds, 21 at 0 s, 53 plugged (`none`), 2 not written |

And of **configurations**, over 80 motor mounts in 63 designs (none named only by a mount):

| configurations | count |
|---|---|
| declared | 170 |
| the rocket flies | 145, in 41 designs, and all 145 assemble |
| left out, by the first reason the reader finds | 24 a motor with no curve, 1 stages HPR Sim can't separate as written, and none now a motor HPR Sim can't light as written (1 before [M4.5n](../decisions-and-roadmap.md#m4-5n) read a stage's first burnout); before parallel stages were read, 2 more for a motor in a part not read and 2 for an airframe not read exactly as written |

On 2026-09-28, after [M2.2e8](../decisions-and-roadmap.md#m2-2e8) read tube fins sized from the
body, 109 flew in 30 designs, and 15 were held back as an airframe not read exactly as written
and 19 as stages HPR Sim can't separate as written. On 2026-09-25, before pods, the old override flag, fillets, the bore inside a nose cone and tube
fins were read, 93 flew in 23 designs, and 30 were held back as an airframe not read exactly as
written.

**What the database changed.** With the bundled catalog alone, 166 configurations are held back
for want of a curve, and 4 fly. The table follows those 166. A configuration that now has its
curves can still be held back for another reason, so these counts differ from the table above:

| what became of the 166 | configurations |
|---|---|
| fly | 141 |
| held back for another reason: stages HPR Sim can't separate as written | 1 |
| still no curve: the motor records no digest, and the bundled catalog lacks it | 20 |
| still no curve: a hybrid | 3 |
| still no curve: a digest the database lacks | 1 |

> **How far to trust it.** The curves are copied, from the designs' own files or from OpenRocket's
> motor database.
>
> - **Total impulse** is within 0.1% of OpenRocket's on every curve, and the survey fails otherwise.
>   All of them agree to the last bit. The two checks differ in strength:
>   - The 3 curves the designs embed are real checks. HPR Sim parses each file itself, as for the
>     [bundled curves](../physics/motor.md#validation).
>   - For the 1,288 solid database curves, both codes integrate the same samples, which OpenRocket
>     has already parsed. That check proves the hand-off, not two independent readings.
> - **The curve OpenRocket flies.** The oracle opens each design in OpenRocket with the database
>   loaded, and records the digests of the motors it places in each configuration. The survey fails
>   unless, in every configuration HPR Sim flies with a supplied curve, OpenRocket places each
>   supplied curve too. It does in all 142 such configurations (177 motors), and OpenRocket opens
>   every design they are in, as the survey prints.
> - **What the survey doesn't supply:**
>   - the 164 hybrid motors;
>   - 6 digests that are each shared by two motors whose data differ (samples, case or masses), in
>     OpenRocket 24.12's database;
>   - one motor whose propellant mass gives an [effective exhaust velocity](../glossary.md#effective-exhaust-velocity)
>     of 10.1 km/s, which HPR Sim refuses as impossible.
>   - Other masses are taken as OpenRocket holds them, including some that are physically unlikely.
> - **Where the weight sits differs.** HPR Sim builds every motor from its envelope. The center of
>   mass sits at mid-case, and the dry case is a thin tube. OpenRocket gives each database motor a
>   fixed center of mass of its own, and treats the motor as a solid cylinder for inertia. The 1,288
>   solid motors are 1,221 digests once the 54 repeats, the 12 motors sharing 6 digests and the
>   refused one are set aside. For 163 of those 1,221, OpenRocket's center of mass is more than 1 mm
>   from mid-case. Among the 55 distinct supplied motors the designs fly, 6 are, by up to
>   6.5 mm. This matters to stability margin and roll, and
>   [M2.2d](../decisions-and-roadmap.md#m2-2d) will meet it when it flies these designs against
>   OpenRocket.

How this was decided, and the counts in full, is in [ADR-067][adr-067]. The reading order for
curves is in [ADR-055][adr-055].

[adr-067]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0067-curves-come-from-openrockets-own-database-by.md
[adr-055]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0055-m3-1c-split-and-the-motors-a-ork-flies-its-own.md

## When parachutes open and stages separate

**In short.** HPR Sim reads when each parachute and streamer opens, with the drag coefficient it
states, and when each stage separates, in every configuration. Since the
[M4.5a milestone](../decisions-and-roadmap.md#m4-5a) (a `.ork`'s recovery flown), parachutes and
streamers fly as HPR Sim's own [recovery devices](../physics/recovery.md), the way OpenRocket flies
them ([flown as OpenRocket flies them](#flown-as-openrocket-flies-them)). Since the [M1.9c milestone](../decisions-and-roadmap.md#m1-9c)
(`.ork` staging flown against OpenRocket), a configuration's powered separation is flown: one
whose time is known before the flight, when a motor ahead of it is still burning or yet to light
and none behind it is. It drops the booster and flies the sustainer on. Since
[M4.5n](../decisions-and-roadmap.md#m4-5n) (a stage's first burnout), one at its own stage's
first burnout may also drop the stage's other motors still burning, as OpenRocket does. Since the
[M4.5g2 milestone](../decisions-and-roadmap.md#m4-5g2) (several separations), a configuration's
several powered separations are flown in turn, from the tail forward. A separation at apogee or
on the way down is part of the descent: it is left out, and the configuration flies whole, even
when a sustainer sits ahead of it. That assumes every motor is spent by apogee, which the report's
tool checks of every flight. Any other separation, such as one after a sustainer has already burnt
out, is not flown ([which configurations the rocket flies](#which-configurations-the-rocket-flies)). Every device and stage in the reference library is read. The separations of its 2 parallel
stages are read since [M4.5m](../decisions-and-roadmap.md#m4-5m)
([Parallel stages](#parallel-stages)), and the 2 parachutes inside pods since [Pods](#pods) are.

A **deployment** is an event, a height for the event that needs one, and a delay after it. A
parachute or streamer states its own, and a configuration may change any of the three:

- `<deployevent>ejection</deployevent>`, `<deployaltitude>200.0</deployaltitude>` (meters) and
  `<deploydelay>0.0</deploydelay>` (seconds) are the device's own.
- `<deploymentconfiguration configid="…">` changes them for one configuration. Whichever of the
  three it leaves out, HPR Sim keeps the device's own, as it does for a motor's ignition. That is
  HPR Sim's reading: OpenRocket was not probed on a file that leaves one out.

A stage's **separation** is written the same way, with `<separationevent>`,
`<separationaltitude>`, `<separationdelay>` and `<separationconfiguration configid="…">`. The stage
that states it is the one that drops away: OpenRocket's labels (below) speak of the "current
stage" and the "upper stage".

### What the words mean

OpenRocket's documentation lists none of these words. The committed probe
`validation/oracles/openrocket/events.py` runs OpenRocket 24.12, sets every value of each event
through the program's public setters, saves the design, and records the word written and the label
OpenRocket shows. Its results are in `validation/fixtures/ork/openrocket-events.json`, and the test
`hpr_io::ork::tests::every_event_word_openrocket_writes_is_read` holds HPR Sim's reader to every
word.

| deploy word | OpenRocket's label |
|---|---|
| `launch` | "Launch (plus NN seconds)" |
| `ejection` | "First ejection charge of this stage" |
| `apogee` | "Apogee" |
| `altitude` | "Specific altitude during descent" |
| `lowerstageseparation` | "Lower stage separation" |
| `never` | "Never" |

| separation word | OpenRocket's label |
|---|---|
| `launch` | "Launch" |
| `ignition`, `burnout`, `ejection` | "Current stage motor ignition", "… burnout", "Current stage ejection charge" |
| `upperignition` | "Upper stage motor ignition" |
| `altitudeascending`, `apogee`, `altitudedescending` | "Specific altitude during ascent", "Apogee", "Specific altitude during descent" |
| `never` | "Never" |

The same probe measures three things the words do not say:

- **A deploy height is above the ground**, meaning the launch site: OpenRocket has no terrain. The
  probe flies in calm air with a fixed seed, so it writes the same numbers every run. On a pad
  1,000 m above sea level, a parachute set to `altitude` 30 m opened at 29.9 m above the ground,
  1,029.9 m above the sea. A height read above the sea would never have been reached.
- **Set above apogee, it did not open.** Set to 100 m, on a flight whose apogee was 51.7 m, the
  parachute never opened, and the flight reached the ground. That is one run of one design, not a
  rule OpenRocket states. HPR Sim's own altitude trigger opens at apogee instead
  ([Recovery](../physics/recovery.md#triggers-lag-and-release)), so the two differ here. HPR Sim
  keeps its own rule for a `.ork` too, and `hpr sim`'s notes say when it applies.
- **`<cd>auto</cd>`** is reported as 0.8 for a parachute, on the canopy's area: OpenRocket's
  [technical documentation][techdoc] gives 0.8 as the default (section 4.2.5), and the probe reads
  it back. For a streamer it is worked out from the strip's length and material, on the strip's
  area (appendix C, equations C.4 and C.5): 0.089, 0.060 and 0.050 for strips 0.5, 1.0 and 1.5 m
  long, in a material of 67 g/m², which the three values imply by equation C.5. The documentation puts that estimate's accuracy at about 20%, and one
  user's report ([issue #2031](https://github.com/openrocket/openrocket/issues/2031)) finds a
  2.5 by 44 in streamer's 0.06 far too small. HPR Sim keeps the `auto` or the stated number as the
  file wrote it.

### Flown as OpenRocket flies them

**A configuration's parachutes and streamers fly as OpenRocket flies them, and land at
OpenRocket's speed to within 0.02% on 50 of its 53 example flights, and within 0.12% on all 53.**
[`hpr::ork::recovery`](../api/hpr/ork/fn.recovery.html) turns each one into a flight device, and
[`hpr sim`](../cli.md#hpr-sim) flies them through it, so its numbers are the library's. This is
checked against OpenRocket alone, not against a real flight. [Recovery](../physics/recovery.md#from-an-openrocket-file)
has the model and the full results; the decision record on flying a `.ork`'s recovery,
[ADR-153][adr-153], has the reasoning.

- **How.** The drag is the file's `C_D`, or OpenRocket's own for `auto`: 0.8 on a parachute's
  canopy, and appendix C's estimate for a streamer. Each device opens fully at once, the file's
  delay after its event, and the descent is a point mass under the open devices' drag alone, as
  in OpenRocket. `apogee`, `altitude`, `ejection` and `launch` fly; `never`, and `ejection` on a
  plugged motor, open nothing.
- **How close.** On 48 of OpenRocket's example flights (the report's 51 with no named cause for
  an apogee gap, less the *Base drag hack*'s three, whose flight time runs up to +7.23%), the
  landing speed is +0.00% to +0.02% off OpenRocket's and the flight time −1.57% to +3.16%. 40 meet
  every target set before measuring; the other 8 miss only on a deployment's time, which the
  climb's own apogee gap and OpenRocket's time step explain
  ([the report's descents](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/openrocket-flights.md#descents)).
- **The limits.** A device inside a part HPR Sim doesn't read and an unknown event are refused, and
  the flight falls on its airframe alone with the reason in its notes. A device set to open at
  its stage's ejection charge, in a stage where no motor lights, never opens, as in OpenRocket
  ([M4.5i](../decisions-and-roadmap.md#m4-5i); [ADR-168][adr-168]). One set to open at `lowerstageseparation` opens at a split with nothing left to burn,
  and is refused otherwise. A device set above the apogee opens
  at apogee, where OpenRocket's one probed run never opened it. HPR Sim counts a height from the
  ground, OpenRocket from where the center of mass starts on the pad.
- **A stage that separates under power.** [`hpr::ork::separated_recovery`](../api/hpr/ork/fn.separated_recovery.html)
  puts each device on the part its stage is in. The booster tumbles from the split until its first
  own device opens, and a sustainer with no device tumbles from its apogee. The sustainer's descent
  is held to the same targets: on the *Two stage high power rocket*'s two configurations, its
  landing speed is +0.00% off OpenRocket's and its flight time +0.32% and +0.73%. The first
  meets them once sized, as its main opens 1.09 s early on an apogee 1.79% low. OpenRocket's
  record keeps only the sustainer's branch, so the booster's descent is not compared
  ([ADR-159][adr-159], powered separation in `hpr sim`).

[adr-153]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0153-a-orks-recovery-flown-as-openrocket-flies-it.md
[adr-155]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0155-design-checks-that-match-reality.md
[adr-159]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0159-a-powered-separation-in-hpr-sim.md

### Recovery in the reference library

`cargo xtask ork`, over the 72 designs, on 2026-10-05, after
[M4.5m](../decisions-and-roadmap.md#m4-5m) read parallel stages:

| quantity | count |
|---|---|
| parachutes and streamers read | 136: 134 parachutes, 2 streamers; 134 before parallel stages were read |
| left out, inside pod sets HPR Sim did not read then | 2; since pods are read ([Pods](#pods)), none |
| drag coefficient | 79 `auto`, 57 stated |
| deploy event | 79 `ejection`, 32 `apogee`, 18 `altitude`, 6 `lowerstageseparation`, 1 `never` |
| devices a configuration changes | 11, with 25 changes in all |
| stages that state a separation | 20 of 92: 14 `ejection`, 4 `upperignition`, 2 `burnout`; 15 changes per configuration |
| separations left out, in parallel stages HPR Sim does not read | 0; 2 before parallel stages were read ([Parallel stages](#parallel-stages)) |

How this was decided is in [ADR-056][adr-056].

[adr-056]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0056-a-ork-designs-recovery-and-separation-read-as.md

## What OpenRocket last did: stored simulations

**In short.** A `.ork` keeps the simulations OpenRocket last ran on the design, and HPR Sim reads
them back: the launch conditions each was flown in, the ten summary figures, and each stage's time
series with its events. Reading a result is not the same as validating it. [M2.2b5](../decisions-and-roadmap.md#m2-2b5) applies a
**reference gate** (a screen deciding whether stored data may be used as a reference datum): it
checks current status, two known OpenRocket provenance markers, plausible stored values, and
internally consistent series data. It does not re-run OpenRocket or compare the stored flight with
a fresh HPR Sim or OpenRocket flight; an eligible stored run may still disagree with that later
comparison. Every parseable result stays visible, including results that fail the screen.

The reference screen and HPR Sim's reproduction screen answer different questions. The first asks
whether stored data is suitable as a reference datum. The second asks whether HPR Sim can reproduce
the named design and motor configuration. A complete OpenRocket result can pass the first and fail
the second when its motor curve is absent or its airframe is reduced. A later flight-comparison gate
must report both screens rather than treating either one as a physics validation.


A stored simulation has three parts:

- **`<conditions>`**: the launch rod, the wind, the launch site, the atmosphere and the time step.
- **The summary**: ten figures on `<flightdata>`, such as `maxaltitude` and `optimumdelay`.
- **The time series**: a `<databranch>` per stage, whose `types` name its columns (`Time`,
  `Altitude` and the rest: 58 in the files OpenRocket 24.12 writes, other versions differ) and
  whose `<datapoint>` rows give a value for each, beside the flight's `<event>`s (`launch`,
  `apogee`, `recoverydevicedeployment` and so on) and any `<warning>` OpenRocket stored.

### What the numbers mean

The file-format page gives units only for the multilevel wind (meters, m/s and radians), and its
own example writes a launch rod's direction as `90.0` and a wind's as `1.5707963267948966`. In this
section, **SI** means the International System of Units; lengths are meters, times seconds, speeds
meters per second, pressures pascals and angles radians unless the table says otherwise. A **denominator**
is the count of runs used to calculate a reference statistic. The
committed probe `validation/oracles/openrocket/conditions.py` runs OpenRocket 24.12, sets the
conditions through its public setters, saves, loads them again and flies them. Its results are in
`validation/fixtures/ork/openrocket-conditions.json`, and the test
`hpr_io::ork::tests::wind_direction_is_not_rod_direction` holds HPR Sim's reader to them.

| the file says | it means | HPR Sim gives |
|---|---|---|
| `<launchrodangle>5.0</launchrodangle>` | the rod tilts 5 degrees from vertical | `rod_angle_rad`, 0.0873 |
| `<launchroddirection>45.0</launchroddirection>` | toward a compass bearing of 45 degrees, clockwise from north | `rod_direction_rad`, 0.785 |
| `<winddirection>0.5</winddirection>` | the wind blows **from** a bearing of 0.5 radians, about 29 degrees | `wind_from_rad`, 0.5 |
| `<windturbulence>0.1</windturbulence>` | turbulence intensity: the wind speed's standard deviation over its mean | `wind_turbulence`, 0.1 |
| `<atmosphere model="extendedisa">` with `<basetemperature>` and `<basepressure>` | the standard atmosphere from a temperature (K) and pressure (Pa) at the launch site | `Atmosphere::Extended` |

Here `ISA` means International Standard Atmosphere, the reference atmosphere; `K` means kelvin,
`Pa` pascal and `rad` radian. The probe also flies the example, in three ways that settle what the
directions mean:

- **The rod's direction is a compass bearing.** In calm air, a rod tilted 10 degrees toward
  bearing 0 (north) lands the rocket 21.3 m north, and toward 90 (east), 21.3 m east, while the
  example's own wind setting stays at 90 degrees. OpenRocket's preferences page still calls the
  direction relative to the wind; the program does not treat it so.
- **The wind's direction is where it blows from.** From a vertical rod, in a steady 5 m/s wind
  from bearing 90 (east), the rocket lands 48.4 m west; from bearing 0, 48.4 m south.
- **`<launchintowind>` rewrites the rod.** With it true, OpenRocket overwrites the rod's direction
  with the wind's bearing, in degrees: a wind from 0.5 radians is written as a rod direction of
  28.648.

All of this is measured on OpenRocket 24.12. A file written by a much older version may have meant
the rod's direction otherwise, which is not measured.

The rod's direction and the wind's are different numbers in different units.
[Loft][loft] read the wind's direction from `launchroddirection`
([L64](../decisions-and-roadmap.md#l64)). HPR Sim reads each from its own tag.

The probe saves a flight and compares one stored row with the same quantities as OpenRocket held
in eight columns: time, altitude, vertical velocity, two angles, latitude, air temperature and air
pressure. These are **SI** (the International System of Units): lengths in meters, times in seconds,
pressures in pascals and speeds in meters per second; angles are in radians and latitude in degrees.
Values are rounded when stored: to three decimal places (287.857 K, 0.218 rad), so a small quantity
keeps few digits, and a large one to four significant figures (100,796.6 Pa is stored as 100,800).
`NaN` means “not a number”: OpenRocket did not
compute that quantity at that step; HPR Sim stores it as Rust's `None` value, meaning no numeric
value is available. A design with stored results therefore survives being saved as JSON and read back.

This is from the test [`stored_results_are_read_back`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/crates/hpr-io/src/ork/tests.rs#L2777): a stored run reads back through `design.simulations`, and a column comes out by its name.

[`hpr_io::ork::StoredSimulation::reference_exclusion`](https://hpr.fusionspace.co/api/hpr_io/ork/simulations/struct.StoredSimulation.html#method.reference_exclusion) then classifies the stored run without
altering it. An explicitly `uptodate` run must name both `RK4Simulator` and `BarrowmanCalculator`,
the simulator and aerodynamic calculator markers shown in the public file specification's
Simulation Data example. These strings identify recorded provenance, not the complete OpenRocket
version, settings or design. The run must have finite, non-negative values, positive altitude, speed
and time to apogee, and no contradiction between its summary and time series. A missing summary or
flightdata, a backwards time series, or a time-series altitude materially different from the stored
apogee is excluded with a stable reason. The comparison allows `max(0.1% of the stored apogee,
1 mm)` for stored-value rounding; it is a data-integrity policy, not an accuracy claim about the
flight model. For example, with a 100 m stored apogee, 100.05 m passes this screen and 100.2 m fails
it; the [comparison test](https://github.com/nrdptel/fusionspace-eridanus/blob/main/crates/hpr-validate/src/openrocket.rs#L361)
pins the two-sided allowance and its boundary. For this screen, HPR Sim treats the first branch with an apogee event in file order as the summary
branch; the [selection test](https://github.com/nrdptel/fusionspace-eridanus/blob/main/crates/hpr-validate/src/openrocket.rs#L326)
exercises that implementation policy. The rule has not been independently validated against
multi-stage OpenRocket output. A single
altitude-bearing branch is checked even when its apogee event is missing. Multiple branches without
an apogee event are uninspectable because their summary branch cannot be identified. Times in rows
and events must be finite, non-negative and ordered. When the summary is present, no time may exceed
the stored flight time beyond the 1 ms allowance for stored precision; the first apogee event must
also agree with `timetoapogee` within 1 ms. Fatal events
(such as `SIM_ABORT`, with case, underscore and hyphen variants normalized) are excluded. There is
no arbitrary minimum apogee: a small but self-consistent flight is not rejected merely for being
small.

`Design::reproduction_exclusion` is a second, stricter screen. It excludes reduced designs, missing
or unknown configurations, configurations HPR Sim cannot assemble, and configurations left out while
reading the design. These reasons describe HPR Sim's current ability to reproduce the design, not
the stored result's provenance or physical correctness.

### What the summary words mean

This section records what OpenRocket 24.12 means by each of the ten summary figures on
`<flightdata>` (the *summary words*, such as `maxvelocity`), measured on flights OpenRocket ran
itself. HPR Sim's own flights of the same configurations are compared with them in the next
section, [HPR Sim's flights against OpenRocket's](#the-simulators-flights-against-openrockets).

A word names a quantity, but not how it was taken. `deploymentvelocity` could be the speed at the
first parachute or at the last. `timetoapogee` could be the apogee event's time or the time of the
highest stored step. Another tool, or another OpenRocket version, can use the same word for a
different quantity ([Loft lesson](../glossary.md#loft-lesson) [L80](../decisions-and-roadmap.md#l80)).
So HPR Sim will compare a stored value only through a written definition for the version that wrote
it. **Only OpenRocket 24.12's are measured.** A file's writer is its root `creator` attribute
(`OpenRocket 24.12`). For any other version, every summary value is withheld, not compared.

**How they were measured.** The [oracle](../glossary.md#oracle) script
[`flights.py`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/oracles/openrocket/flights.py)
runs the OpenRocket 24.12 program file (its jar). It flies every motor configuration of the public
designs: the 17 examples inside the jar and the seven Loft demos. Each flight uses the design's
first stored launch conditions, in calm air (no wind, no turbulence). Of the demos, only one has a
motor OpenRocket finds, and `demo-quirks.ork` does not open. That leaves 18 designs with 57 motor
configurations between them, so 57 runs. OpenRocket aborted one of them, *Pods--powered with
recovery deployment* `[C6-7; 2× A3-4, B6-0]` (its motors by stage, sustainer first, the stages
separated by `;`: a C6-7 in the sustainer, two A3-4s and a B6-0 in the booster), at 1.81 s and
85 m up ("Stage began to tumble under thrust"). Its figures are where the run stopped, not a
flight's, so it is not a reference, which leaves 56 complete flights. The record,
`validation/fixtures/ork/openrocket-flights.json`, keeps each word beside the quantities of
OpenRocket's own time series (its stored steps) that the word could mean. The test
[`stored_metric_definitions_are_per_tool_and_version`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/crates/hpr-validate/src/tests.rs)
holds each defined word to its quantity on all 56, to 1 part in 10⁹. Nine of the ten words are
defined; the tenth, `optimumdelay`, is withheld, with the flights that rule out its two obvious
readings.

| word | OpenRocket 24.12 means | shown by the record |
|---|---|---|
| `maxaltitude` | the largest altitude above the launch site | 56 of 56 |
| `maxvelocity` | the largest total speed (not the vertical speed) | 56 of 56 |
| `maxmach` | the largest Mach number | 56 of 56 |
| `timetoapogee` | the time of the highest stored step | 56 of 56; the apogee event's time differs on 13 |
| `flighttime` | the time of ground hit, the last step | 56 of 56 |
| `launchrodvelocity` | the total speed at the first step past the rod's length | 56 of 56; 0.06% to 7.50% above the speed at the rod's end, placed by interpolating linearly in height |
| `deploymentvelocity` | the total speed at the **last** deployment, interpolated between the steps either side | 56 of 56; 53 deployments fall between steps, and on 17 flights the first deployment's speed differs |
| `groundhitvelocity` | the total speed at ground hit, the last step | 56 of 56 |
| `maxacceleration` | the largest total acceleration before the first deployment | 56 of 56; over the whole flight it differs on 15, whose largest acceleration comes after a deployment |
| `optimumdelay` | **not measured**: withheld | on 15 flights it is not the time of apogee less the time of the last burnout, nor that of the same flight flown again with nothing deployed |
| (no word) stability margin | the distance from the center of gravity (CG) aft to the center of pressure (CP), in calibres, at rod clearance | the stability column, 56 of 56 |

The stability margin follows Niskanen's definition ([N09](../physics/aero.md#code-and-sources) p. 12): the
CP's distance behind the [CG](../glossary.md#center-of-gravity-cg), divided by the reference length
and so counted in [calibres](../glossary.md#calibre-caliber). OpenRocket's reference length is by
default the largest body diameter, and that is the setting on all 57 runs. The optimum delay is the
best [ejection delay](../glossary.md#ejection-delay). The 15 flights it misses are exactly those on
which a recovery device deploys before apogee; five others fire an ejection charge before apogee
but deploy only after it, and agree. Flown again with nothing deployed, every one of the 56 gives an
`optimumdelay` equal to its apogee less its last burnout. So an early deployment is what changes the
figure, but how is not known, and the figure is not used.

OpenRocket does not say which point on the rocket its speeds belong to. RocketPy's are the
[center of dry mass](../glossary.md#center-of-dry-mass)'s, and RocketPy's rail exit is where the
rocket's travel equals the rail, found between steps
([ADR-021](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0021-whole-flights-against-rocketpy-what-is-compared.md)).
So RocketPy's and OpenRocket's largest speed and rail-exit speed are kept as different definitions.

**A worked example.** The *Chute release* example with a G40W-7 fires its ejection charge after
apogee and opens its main parachute lower down:

| deployment | time, s | speed, m/s |
|---|---|---|
| first | 9.301 | 8.305 at the 9.3 s step, 8.329 at 9.3025 s: 8.314 interpolated |
| last | 20.514 | 14.231, on a step |

OpenRocket's `deploymentvelocity` is 14.231 m/s: the last deployment. A comparison using the first
deployment would have set HPR Sim's value against 8.31 m/s instead. The interpolation matters too.
For the *3D printable nose cone and fins* example with a B6-4, the one deployment is at 4.861 s,
between steps at 4.86 s (0.6575 m/s) and 4.8625 s (0.6330 m/s). OpenRocket reports 0.6477 m/s, the
value interpolated at 4.861 s, not either step's.

**An event that never happened has no value.** The record also holds a 58th flight, outside the
57: the *A simple model rocket* example edited so its parachute never opens. It flies to the ground, and OpenRocket writes
`NaN` for `deploymentvelocity`. On every complete flight, a speed taken at an event is `NaN` exactly
when the event is missing. Loft scored such values as 0
([L81](../decisions-and-roadmap.md#l81)). HPR Sim's
[`compare`](https://hpr.fusionspace.co/api/hpr_validate/flight_metrics/fn.compare.html)
never scores them. When neither flight had the event, the metric is withheld. When only one did,
the flights disagree about what happened, and the comparison fails. Every figure of an aborted run
is withheld. The test
[`metric_for_missing_event_is_withheld_not_scored`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/crates/hpr-validate/src/tests.rs)
checks all three.

### The simulator's flights against OpenRocket's

This section compares HPR Sim's whole flights with OpenRocket 24.12's on the 54 configurations of
OpenRocket's example designs that HPR Sim can fly, in 15 of the examples. OpenRocket aborts its own
flight of one of them, so the figures below are over the other 53, and that one is checked up to
the abort ([below](#flights-openrocket-aborted)). It is a
[code-to-code comparison](../glossary.md#code-to-code-comparison): agreeing with OpenRocket is not
the same as agreeing with real flights. Use it to judge how closely HPR Sim's flight matches
OpenRocket's on ordinary hobby rockets.

- The [stability margin](../glossary.md#stability-margin) as the rocket leaves the rod agrees
  within 0.016 [calibres](../glossary.md#calibre-caliber) on 41 of the 53. HPR Sim's margin here is
  taken with the air along the rocket's axis (the 0° plane), as OpenRocket's is, not in HPR Sim's
  weakest plane: on `[C6-7; B6-0]` of *Pods--powered with recovery deployment* it is +3.74
  calibres along the axis, while `hpr sim` prints a least margin of −4.21 calibres in the weakest
  plane. The *Tube fin rocket*'s
  is 1.08 calibres short of OpenRocket's: the two codes' tube fins differ
  ([Tube fins](../physics/aero.md#tube-fins)). The three-stage example's three are 0.039 to 0.058
  calibres short, about as much as HPR Sim's center of mass sits further aft there. The
  *Pods--airframes and winglets* example's five are 0.071 to 0.076 calibres long, the flattering
  side: HPR Sim's center of pressure sits 2.6 mm aft of OpenRocket's there
  ([#325](https://github.com/nrdptel/fusionspace-eridanus/issues/325), fin–fin interference;
  [#326](https://github.com/nrdptel/fusionspace-eridanus/issues/326), a kinked fin's center of pressure). The
  *Pods--powered with recovery deployment* example's three are 0.070 calibres long, the same side:
  HPR Sim's center of pressure sits 2.4 mm aft of OpenRocket's there, a gap not yet traced.
- *Pods--airframes and winglets* flies since [M4.5g4](../decisions-and-roadmap.md#m4-5g4), which
  reads its cockpit fin on the nose cone
  ([Fins on a nose cone or a transition](#fins-on-a-nose-cone-or-a-transition)). Its five apogees
  are within 0.5% of OpenRocket's, −0.13% to +0.48%, and its margin 0.07 calibres high.
- *Pods--powered with recovery deployment* flies since [M4.5i](../decisions-and-roadmap.md#m4-5i),
  which weighs its rail buttons' screw heads and flies its motors in two pod sets
  ([ADR-168][adr-168]). Its first configuration, `[C6-7; 2× A3-4, B6-0]`, flies since
  [M4.5n](../decisions-and-roadmap.md#m4-5n) (a stage's first burnout), checked up to OpenRocket's
  abort ([below](#flights-openrocket-aborted)). Its sustainer alone, `[C6-5; None]`, reads 0.36% low, and its booster
  alone with its two pod motors, `[None; 2× A10-3, B6-4]`, 1.86% low. The two pod parachutes of
  that flight open at launch + 5.35 s in both programs; its landing speed is +0.00% off
  OpenRocket's and its flight time −1.02%, and both descents meet their targets
  ([ADR-153][adr-153]). Its staged flight, `[C6-7; B6-0]`, is the eighth apogee more than 5% off
  (below). The file flies in `hpr sim` as saved: the centering rings on the booster's 18 mm
  motor tube wrap it ([a ring around its tube](../physics/design.md#wider-than-its-parent)). One fits the booster's airframe; the other, in a coupler,
  reaches 0.76 mm past the coupler's bore and warns as a fit to sand.
- The mass at launch agrees within 0.21% on all 53, and the
  [center of mass (CG)](../glossary.md#center-of-gravity-cg) as the rocket leaves the rod within
  0.016 calibres on all but the three-stage example's three, which are 0.043 to 0.062 calibres
  further aft ([M2.2e1](../decisions-and-roadmap.md#m2-2e1), mass and center of mass). That is
  where OpenRocket's recorded mass falls as if every motor of the booster's type burned from
  launch ([#185](https://github.com/nrdptel/fusionspace-eridanus/issues/185)); whether that accounts for the
  gap is not sized.
- On the 36 flights with no named cause, HPR Sim's apogee is from 4.34% low to 2.06% high. Only 14
  read high: the five *Airstart timing* flights, the *ARC payload rocket*, the *Deployable
  payload* on an A8-3, four of the five *Pods--airframes and winglets* flights, both *Parallel
  booster staging* flights (+1.23% and +0.68%, [Parallel stages](#parallel-stages)), and *Base
  drag hack (short-wide)*, the highest.
- On the six *Dual parachute deployment* flights, HPR Sim's apogee is 0.63% to 4.34% low. The cause
  is not traced.
- A two-stage design, a three-stage design, a cluster, an air start and boosters beside the core
  fly, and every flight of each is within 5% of OpenRocket's apogee and largest speed, the bar the
  [M1.9c milestone](../decisions-and-roadmap.md#m1-9c) set before measuring. Five of their
  apogees are compared with OpenRocket's flight with no parachute, since its parachute opened
  early ([staged, clustered and air-start flights](#staged-clustered-and-air-start-flights),
  below).
- Six probe designs, five with pods and one without, written for
  [M1.13c2](../decisions-and-roadmap.md#m1-13c2) (a pod design against OpenRocket) and not counted
  among the 53, fly within 0.81% of OpenRocket's apogee and 1.24% of its largest speed, their
  margins within 0.0014 calibres ([aerodynamics: Pods](../physics/aero.md#pods)).
- The bar, set for the whole corpus ([M2.2](../decisions-and-roadmap.md#m2-2), OpenRocket
  comparisons), is that every apogee more than 5% off has a written cause. Eight apogees are
  more than 5% off, and each has a named cause. Seven are sized in one of two ways:

  | how the cause is sized | flights | within 5% after |
  |---|---:|---:|
  | OpenRocket flies the flight again with its causes taken out ([M2.2e4](../decisions-and-roadmap.md#m2-2e4)) | 6 | 4 |
  | HPR Sim flies the flight again on OpenRocket's own drag ([ADR-097][adr-097]) | 1 | 1 |

  The two of the six still over 5% are the *Base drag hack* on a D12-3 and an E12-4. With
  nothing deployed in OpenRocket either, HPR Sim reads 5.84% and 8.92% high. Flown on OpenRocket's
  own drag along that flight, HPR Sim comes within 0.10%, so what is left is the drag coefficient:
  by OpenRocket's part-by-part breakdown, the nose
  ([#177](https://github.com/nrdptel/fusionspace-eridanus/issues/177), a very blunt nose's drag; see
  [below](#a-part-set-to-no-drag)).

  The seventh is the *Tube fin rocket*, 6.95% high. On OpenRocket's drag HPR Sim comes within 0.03%,
  so its cause is HPR Sim's own drag: the net gap is the drag's, though parts of it could cancel.
  Its written breakdown is in [ADR-099][adr-099].

  The eighth is *Pods--powered with recovery deployment* on `[C6-7; B6-0]`, 37.93% low, its
  largest speed 15.76% low. Its sustainer is unstable when the air crosses it edge-on to its
  two-fin strake set, and in the conditions of OpenRocket's record it turns over before apogee in
  both programs, which is the cause the report names. OpenRocket's own record tumbles at 2.84 s: its calm, vertical flight holds the
  angle of attack at zero until then. Seeded with a 0.5° rod tilt, OpenRocket aborts the flight
  at 1.71 s and 51 m up (a scratch run, not committed). HPR Sim's sustainer turns over as the
  Earth's rotation at the record's site, 28.61° N, tips it: its angle of attack passes 90° at
  2.25 s. At `hpr sim`'s default site, on the equator, nothing tips it and it stays upright
  ([Stability margin](../physics/metrics.md#stability-margins)). Its descent lands at OpenRocket's speed (+0.00%), but its flight time is
  52.64% short. Since [#329](https://github.com/nrdptel/fusionspace-eridanus/issues/329), HPR Sim's printed
  margin is the weakest direction's: `hpr sim` prints this configuration's least margin as
  −4.21 calibres, at the separation (0.86 s), where it printed +1.56 before
  ([Stability margin](../physics/metrics.md#stability-margins)).

**How they were flown.** `cargo xtask ork-flights` flies every configuration of the record in the
section above that HPR Sim can fly. It uses the conditions OpenRocket flew: the launch rod as
recorded (its length, its angle from the vertical and the compass bearing it leans toward, as
[below](#a-tilted-launch-rod)), the recorded site, the
[standard atmosphere](../glossary.md#standard-atmosphere) and no wind. Each figure is taken the
way OpenRocket takes it, by the definitions above:

- The **apogee** is the highest point above where the rocket started. OpenRocket counts altitude
  from 0 at launch, so HPR Sim counts its center of mass's height from where it stood on the pad.
- The **largest speed** is the peak speed on the way up, taken over every sample from launch to
  apogee. OpenRocket's rockets come down under parachutes, but HPR Sim's flight compared here flies
  none, so its unbraked fall is left out. Until [M2.2e7](../decisions-and-roadmap.md#m2-2e7), HPR Sim ended
  the search at the first sample whose vertical speed was no longer above zero. A rocket held on the
  pad can read a few µm/s up or down
  ([#223: a vertical-speed drift on the pad](https://github.com/nrdptel/fusionspace-eridanus/issues/223)), and
  on one private flight that stopped the search before liftoff.
- The **stability margin at rod clearance** is the distance from the center of mass aft to the
  [center of pressure](../glossary.md#center-of-pressure-cp), in body diameters. It is taken at
  the step where OpenRocket's rocket had just travelled past the rod's length, at that step's time
  and Mach number.

The design checks are HPR Sim's tests of whether the parts fit together. Where they only warn,
HPR Sim flies the design as OpenRocket does; the comparison flies even a design with errors, and the
report lists any errors. They find none on OpenRocket's examples. The *Deployable payload*'s payload
mass component and parachute are 25 mm across in a 21 mm bore, so they can't go in as written,
but as packed parts they warn
([a packed part wider than its bore](../physics/design.md#a-packed-part-wider-than-its-bore)).
The two pods examples write each centering ring as a child of the 18 mm motor mount it wraps. A
ring around its tube is checked in the part around it at its own station
([a ring around its tube](../physics/design.md#wider-than-its-parent),
[M4.5k](../decisions-and-roadmap.md#m4-5k)). *Pods--airframes and winglets*' rings fit its
airframe. Of *Pods--powered*'s, one fits the booster's airframe and one, in a coupler, reaches
0.76 mm past the coupler's 31.45 mm bore: a fit to sand, so it warns. Elsewhere the checks give
only [warnings](../physics/design.md#checks) ([ADR-155][adr-155], the decision on which fits
warn): a coupler whose wall reaches 0.46 mm past the bore it
sits in, a fit within the tolerance for that size; bulkheads sized to the airframe on the ends of
couplers, which are caps; and a 29 mm motor in a 28.956 mm mount, which its real case fits.

The record holds 57 powered configurations. The two payload designs, whose payload drops away
with nothing left to burn, fly since [M4.5g3](../decisions-and-roadmap.md#m4-5g3) (a payload's
split), the three-stage example since [M4.5g2](../decisions-and-roadmap.md#m4-5g2) (several
separations), *Pods--airframes and winglets* since
[M4.5g4](../decisions-and-roadmap.md#m4-5g4) (fins on a nose cone), and three of the four of
*Pods--powered with recovery deployment* since [M4.5i](../decisions-and-roadmap.md#m4-5i) (screw
heads and motors in several pod sets), its first, which OpenRocket aborts, since
[M4.5n](../decisions-and-roadmap.md#m4-5n) (a stage's first burnout), and both of *Parallel
booster staging* since [M4.5m](../decisions-and-roadmap.md#m4-5m) (parallel stages). The other 3
are listed in the report with the reason HPR Sim doesn't fly them yet: a motor with no thrust curve,
among them the one powered Loft demo.

Of the other six Loft demos, five have no motor OpenRocket finds, and OpenRocket does not open
`demo-quirks.ork`.

**Where to find it.** The committed
[report](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/openrocket-flights.md)
has every flight's figures, and its JSON twin the same with more digits. Flying them needs the
pinned OpenRocket jar and the record of OpenRocket's motor database, which only a local checkout
holds. CI does not fly them again. It checks that every figure of OpenRocket's in the report is
the record's, and that every outcome, summary and table follows from HPR Sim's figures.

**Results** (HPR Sim less OpenRocket):

| metric | named cause | flights | median | range |
|---|---|---:|---:|---|
| apogee | none | 36 | −0.18% | −4.34% to +1.95% |
| apogee | OpenRocket's parachute opened before apogee | 15 | +0.10% | −1.24% to +13.80% |
| apogee | HPR Sim's own drag, sized on OpenRocket's | 1 | +6.95% | +6.95% |
| apogee | the rocket turns over before apogee in both tools | 1 | −37.93% | −37.93% |
| largest speed | none | 51 | +0.46% | −0.69% to +6.10% |
| largest speed | HPR Sim's own drag, sized on OpenRocket's | 1 | +6.40% | +6.40% |
| largest speed | the rocket turns over before apogee in both tools | 1 | −15.76% | −15.76% |
| margin at rod clearance | none | 53 | −0.0003 cal | −1.0768 to +0.0764 cal |

The margin's −1.0768 cal is the *Tube fin rocket*'s, whose tube fins' center of pressure the two
programs place differently ([aerodynamics: Tube fins](../physics/aero.md#tube-fins)). The
+0.0764 cal is the *Pods--airframes and winglets* example's on an A8-3: its five flights read
+0.0706 to +0.0764 cal, HPR Sim's margin above OpenRocket's, the flattering side
([#325](https://github.com/nrdptel/fusionspace-eridanus/issues/325), [#326](https://github.com/nrdptel/fusionspace-eridanus/issues/326)).
The three *Pods--powered with recovery deployment* flights read +0.0695 to +0.0702 cal, on the
same side. The other 44 flights' margins are within −0.0580 to +0.0045 cal. Below
−0.0151 cal are only the three-stage example's three, −0.0387 to −0.0580 cal.

The rocket's mass and center of mass, over the same 53 flights (HPR Sim less OpenRocket):

| quantity | flights | median | range |
|---|---:|---:|---|
| mass at launch | 53 | +0.008% | −0.016% to +0.209% |
| mass as the rocket leaves the rod | 53 | +0.012% | −0.030% to +3.257% |
| center of mass as the rocket leaves the rod | 53 | +0.0007 cal | −0.0028 to +0.0624 cal |

A positive center-of-mass difference means HPR Sim's is further aft than OpenRocket's. That shortens
the margin by the same number of calibres, so the flight at +0.0624 cal, the three-stage example
on an A8-5, is also the one whose margin is 0.0580 cal short.

- **Mass doesn't explain the apogee misses with no named cause.** On 35 of those 36 flights
  HPR Sim's mass is within 0.05% of OpenRocket's at launch. As the rocket leaves the rod it is
  within 0.04% too, on all but four: the two-stage example on two H148R-0 motors, where HPR Sim is
  1.65% heavier, the three-stage example on C6 motors, 3.26% heavier (see
  [below](#staged-clustered-and-air-start-flights)), and the two *Parallel booster staging*
  flights, 0.041% and 0.063% heavier. The 36th, the *Base drag hack* on a C11-5,
  is 0.21% heavier, which would lower its apogee, yet it reads 2.06% high. On the largest miss,
  *Dual parachute deployment* with a J570W (−4.34%), HPR Sim is 0.01% *lighter* as it leaves the
  rod, which would raise its apogee, not lower it.
- **The largest launch-mass differences are on the *Base drag hack*.** Its three flights are
  0.18% to 0.21% heavier at launch, which would lower their apogees, yet all three read high.
  As the rocket leaves the rod, only the two-stage flight on two H148R-0 motors (+1.65%) and the
  three three-stage flights (+1.80% to +3.26%) differ more.
- **The largest center-of-mass differences are on two designs.** The three-stage example's three
  flights are +0.043 to +0.062 cal, as the rocket leaves the rod at the moment OpenRocket's
  recorded mass reads light ([#185](https://github.com/nrdptel/fusionspace-eridanus/issues/185), above). The
  six *Dual parachute deployment* flights are +0.010 to +0.016 cal, and they account for almost
  all of those flights' margin difference. That design overrides a part's mass, covering
  everything inside it, which HPR Sim applies as OpenRocket does: so its launch masses match
  exactly, and the matched mass says nothing about HPR Sim's own mass model there. Every other
  flight is within 0.0054 cal, the most the *Pods--airframes and winglets* example on a D16-6.

An early parachute can move only the apogee, which is why more flights count toward the largest
speed than toward the apogee. The early-parachute group's median means little: on a rocket whose
parachute opens only a little early, the parachute's cost and HPR Sim's own miss can cancel. The
largest speed's +6.10% is the *Base drag hack* on an E12-4, the same drag coefficient that
leaves its apogee high ([below](#a-part-set-to-no-drag)).

**A named cause, and its size.** A named cause is only a guess until it is sized
([M2.2e4](../decisions-and-roadmap.md#m2-2e4), sizing the causes; decision
[ADR-073](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0073-each-named-cause-sized-by-openrockets-own-flight.md)).
So for every flight with a named cause, the oracle flies OpenRocket's same configuration again
with the cause taken out, and the report compares HPR Sim's apogee with that flight too. Nothing
else changes between the two OpenRocket flights, so the difference between them is what the cause
costs in OpenRocket, and what is left against HPR Sim is what the cause does not explain. A cause
counts as sized to the bar when every such flight, with all the flight's causes taken out, is within
the same 5% of HPR Sim's.

- **HPR Sim's flight compared here flies no parachute.** Its descent is compared on a second flight,
  with the devices ([flown as OpenRocket flies them](#flown-as-openrocket-flies-them)). With a
  short motor delay, OpenRocket's parachute opens while the rocket is still climbing,
  and stops it lower. On the *A simple model rocket* example with a C6-3, the parachute opens
  2.99 s before the apogee the same flight reaches with nothing deployed. OpenRocket's apogee is
  280.2 m, and HPR Sim's, with no parachute, is 318.9 m: +13.80%. Flown again with nothing deployed,
  OpenRocket climbs to 322.4 m, and against that HPR Sim reads −1.07%. On the *3D printable nose
  cone and fins* example the same pair reads +12.19% and −0.16%, and on the *Clustered motors*
  example +9.43% and −0.72%.
- **The check both ways.** A test (`a_parachute_moves_openrockets_apogee_only_when_it_opens_before_it`
  in `xtask/src/ork_flights.rs`) runs over all 56 flights of OpenRocket's record that finished.
  On the 41 whose parachute opens at or after apogee, the flight with nothing deployed reaches
  exactly the same apogee, to the last bit: nothing else differs between the two runs. On 14 of
  the 15 whose parachute opens before apogee, the flight with nothing deployed climbs higher. The
  15th opens only 0.10 s early, and its two apogees differ by 1 mm. OpenRocket records that
  flight every 0.05 s near the top, so its highest recorded point can move by up to 3 mm with
  timing alone, and 1 mm is within that.
- <a id="a-part-set-to-no-drag"></a>**A part set to no drag flies as OpenRocket flies it**, since
  [M4.5h](../decisions-and-roadmap.md#m4-5h) (a part's drag override;
  [ADR-167][adr-167], the decision). The *Base drag hack (short-wide)* example is a short, wide
  rocket with four fins. Behind it sits a weightless cone, flaring from a point to the body's full
  width, that OpenRocket is told has no drag: a modelers' trick for stubby rockets, which moves
  the center of pressure aft, since the cone still gives lift. Before then, HPR Sim read the setting
  but charged the cone the drag of its shape
  ([#165](https://github.com/nrdptel/fusionspace-eridanus/issues/165), the drag override not applied). Now it
  flies the cone with no drag, as OpenRocket does
  ([A part's stated drag coefficient](../physics/aero.md#a-parts-stated-drag-coefficient)).
  HPR Sim's apogee against OpenRocket's:

  | motor | before | now | now, recovery in both | now, against OpenRocket's flight with nothing deployed |
  |---|---:|---:|---:|---:|
  | C11-5 | −16.77% | +1.95% | +1.95% | +1.95% |
  | D12-3 | −15.47% | +12.96% | +4.52% | +5.84% |
  | E12-4 | −19.13% | +10.79% | +7.80% | +8.92% |

  The "recovery in both" column is the apogee of each flight's descent in the
  [report's JSON](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/openrocket-flights.json)
  (`descent.apogee_m`), also in [ADR-167's][adr-167] table. The C11-5's parachute opens after apogee, so it changes nothing there. On the D12-3 and E12-4
  OpenRocket's parachute opens 1.80 s and 1.24 s before apogee, while HPR Sim's flight in the "now"
  column flies none: that early parachute is the named cause of +12.96% and +10.79%. With it held
  in OpenRocket too, the D12-3 and E12-4 still read 5.84% and 8.92% high, more than 5% off.
  Flown on OpenRocket's own drag along that flight, HPR Sim comes within 0.10% of it, so what is
  left is the drag coefficient, not the setting.

**What the causes leave.** On the three C6-3 flights, what is left is −0.16%, −1.07% and −0.72%,
like the flights with no named cause. The *Deployable payload*'s C6-3, +13.75% against
OpenRocket's flight, is left at +2.63%, but that pair is not like for like. In OpenRocket's flight
with nothing deployed the payload still drops its booster and climbs on alone, while HPR Sim's
flight compared here climbs as the whole stack. The *Base drag hack* reads high on all three motors,
+1.95% to +8.92% with nothing deployed in either program, more so on bigger motors. That is more
than any other flight with no named cause reads high: 20 of those 32 read low, and the other 11
that read high, the five *Airstart timing* flights, the two payload examples' *ARC payload
rocket* and *Deployable payload* on an A8-3, and four *Pods--airframes and winglets* flights,
are at most +1.05%.

A part-by-part comparison of the two programs' drag at the same speeds, a one-off check that is
not part of the report, suggested two candidates, both on the side of HPR Sim flying higher. The
report now sizes one of them, and nothing here measures which program is right.

- **The nose.** OpenRocket charges this very blunt, rounded nose, whose length is only 0.58 of its
  diameter, some pressure drag even at low speed. Its coefficient, on the body's cross-section of
  48.7 cm², goes from 0.0117 at Mach 0.1 to 0.0482 at Mach 0.25, about
  the fastest these flights go. HPR Sim charges it 0.0008, read off a straight line between two
  round heads Hoerner measured: a hemisphere at 0.01 and a head one diameter long at −0.05
  ([blunt ellipsoids](../physics/aero.md#blunt-ellipsoids-below-mach-08); [ADR-173][adr-173], a
  blunt ellipsoid's measured drag). A one-off probe, not committed, found that bringing the E12-4
  within 5% would take about 0.013 to 0.015, more than the hemisphere's. HPR Sim keeps the
  measurement, and the gap stays. By OpenRocket's breakdown, this is where the drag coefficient
  differs.
- **The base while the motor burns.** HPR Sim takes the motor's area off the base drag while it
  burns, and OpenRocket 24.12 doesn't. The report flies HPR Sim again with OpenRocket's base drag
  under power: on the D12-3 and E12-4 it reaches the same apogee as on its own drag, so this
  candidate moves nothing on this rocket.

[#177](https://github.com/nrdptel/fusionspace-eridanus/issues/177) (a very blunt nose's drag below Mach 0.8)
holds the numbers; [ADR-173][adr-173] the measurement and the decision.

**The margin, part by part.** The report also lists the parts of each margin at rod clearance: the
mass, the center of mass, the center of pressure and the reference diameter. The reference
diameters agree exactly, and the centers of pressure within 0.25 mm on every flight but two
examples'. The *Tube fin rocket*'s tube fins put HPR Sim's 26.6 mm forward of OpenRocket's, and on
*Pods--airframes and winglets* HPR Sim's is 2.6 mm aft. Apart from those two and the three-stage
example's (above), the largest margin gaps are on the *Dual parachute deployment* example,
−0.0097 to −0.0151 calibres: HPR Sim's center of mass is 0.6
to 0.9 mm aft of OpenRocket's there. Mass times that gap, from the report's JSON figures, stays
between 1.1 and 1.35 g·m on all six motors, while the rocket's mass runs from 1.49 to 2.18 kg. So
the gap is probably in the airframe, not the motors, but it is not traced yet.

**What it leaves out.** Every flight is calm and vertical, with no wind and no recovery. One
flight, the *Dual parachute deployment* example on a J570W, is briefly faster than sound
(OpenRocket's largest Mach number is 1.147). The other 52 stay below Mach 0.72, so this barely
tests HPR Sim faster than sound. That flight is also the largest gap with no named cause: −4.34% in
apogee, with its largest speed −0.69%. The Earth is not the same in both programs. HPR Sim's is the
[WGS 84](../glossary.md#wgs-84) ellipsoid, with its gravity and rotation. OpenRocket's runs record
a flat Earth for 23 of the 53 flights and a spherical one for the other 30 (the record's
`geodetic` field). Each program keeps its own
model, and the effect of the difference is not measured. The decision is [ADR-069][adr-069].

#### Staged, clustered and air-start flights

Since the [M1.9c milestone](../decisions-and-roadmap.md#m1-9c), HPR Sim flies three kinds of flight
it used to leave out: a rocket that drops a booster, a cluster of motors, and an
[air start](../glossary.md#air-start). The bar was written down before any of them was measured
(decision record [ADR-076][adr-076]): every flight of a design within 5% of OpenRocket's apogee
and of its largest speed. Where OpenRocket's parachute opened before apogee, the apogee is held to
OpenRocket's flight of the same configuration with nothing deployed. All three designs meet it.
So does a fourth, the three-stage example, flown since the
[M4.5g2 milestone](../decisions-and-roadmap.md#m4-5g2) (several separations). A fifth staged
design, *Pods--powered with recovery deployment*, flown since
[M4.5i](../decisions-and-roadmap.md#m4-5i), does not: in the conditions of OpenRocket's record
its sustainer turns over before apogee in both programs ([above](#the-simulators-flights-against-openrockets)).
Its first configuration, which OpenRocket aborts, is checked up to the abort instead
([below](#flights-openrocket-aborted)). A sixth, *Parallel booster staging*, flown since [M4.5m](../decisions-and-roadmap.md#m4-5m)
([Parallel stages](#parallel-stages)), meets it. The figures are HPR Sim less OpenRocket:

| design | what it tests | flights | apogee | largest speed |
|---|---|---:|---|---|
| *Two stage high power rocket* | a booster that separates, and a sustainer lit after it | 2 | −1.79% and −0.13% | −0.54% and −0.04% |
| *Three stage low power rocket* | a booster, then a middle stage, each dropping at its burnout as the stage ahead lights | 3 | −1.48% to −0.95%, two against the flight with nothing deployed | +0.25% to +1.64% |
| *Clustered motors* | four motors in a 4-ring, one in each tube | 5 | −0.79% to −0.35%, three against the flight with nothing deployed | +0.20% to +0.92% |
| *Airstart timing* | a 3-ring of I211W motors beside a K550W in its own mount, the ring lit at launch or 1, 2, 4 or 6 s later | 5 | +0.48% to +1.03% | +0.46% to +0.81% |
| *Parallel booster staging* | two boosters beside the core, lit with it at launch and dropped at their charge at 2.44 s; or carried to apogee with no motor | 2 | +1.23% and +0.68% | +3.25% and +1.56% |

- **The two-stage flights separate when OpenRocket's do**, at 1.535 s and 1.515 s after launch
  ([ADR-076][adr-076]). OpenRocket's record keeps the part with the nose, so both programs'
  apogee and largest speed are the sustainer's. For its descent, HPR Sim gives the booster a
  recovery device that opens at the split and the sustainer one that opens at its apogee. Neither
  acts on the climb, which is what is compared. How HPR Sim flies a separation is in
  [Staging](../physics/staging.md#against-openrocket).
- **The three-stage flights separate twice when OpenRocket's do**, at 0.857 s and 1.714 s on the
  B6-0 boosters and at 1.860 s and 3.720 s on the C6-0s: at the booster's burnout, then at the
  middle stage's. HPR Sim flies each split in turn, from the tail forward
  ([Several separations](../physics/staging.md#several-separations)). The sustainer lands within
  0.009% of OpenRocket's landing speed on each.
- **The rocket a configuration builds doesn't carry the separations.** The reader hands them
  over separately, as the configuration's `staging` and `later_stagings`
  (`MotorConfiguration::stagings` lists them all, from the tail forward). A program turns them
  into the flight's separations with `hpr::ork::separations` and passes those to the flight. HPR Sim refuses that flight unless each
  part has a recovery device, so the example gives both parts one.
  [`hpr::ork::separated_recovery`](../api/hpr/ork/fn.separated_recovery.html) gives each part
  the file's own devices instead, and `hpr sim` flies a powered separation that way
  ([Separation](../cli.md#separation)). The example
  [`ork_two_stage.rs`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/crates/hpr/examples/ork_two_stage.rs)
  does this for a made-up two-stage `.ork`; run it with
  `cargo run --example ork_two_stage -p fusionspace-hpr`.
- **On two H148R-0 motors, OpenRocket's rocket gets lighter twice as fast before the sustainer
  lights.** From launch to rod clearance it loses 0.0823 kg, and HPR Sim's 0.0412 kg: 1.9992 times
  as much. The launch masses agree, so HPR Sim is 1.65% heavier as it leaves the rod. On the other
  configuration, with two different motors, the two programs lose the same mass. OpenRocket's
  recorded mass falls as if both H148Rs lost propellant from launch. The three-stage example
  shows the same: OpenRocket's drop is 2.0000, 1.9998 and 2.9944 times HPR Sim's, the count of
  motors of the booster's type in each configuration, so HPR Sim is 1.80% to 3.26% heavier as it
  leaves the rod. What that does to the apogee
  is not sized
  ([#185](https://github.com/nrdptel/fusionspace-eridanus/issues/185)).
- **Three of the cluster flights have a parachute that opens early** in OpenRocket, 0.37 s,
  2.59 s and 0.59 s before apogee. Against its own record, the C6-3 reads +9.43%; against its
  flight with nothing deployed, −0.72%.

#### Flights OpenRocket aborted

**HPR Sim's flight of *Pods--powered with recovery deployment*'s first configuration,
`[C6-7; 2× A3-4, B6-0]`, is within 5% of OpenRocket's at two points up to where OpenRocket
stops.** That is a check of the climb's first 1.81 s, not of an apogee. OpenRocket aborts the
flight at 1.81 s: "Stage began to tumble under thrust." Its record then has no apogee, largest
speed or margin, so the first table withholds them. Up to the abort the record is a flight like
any other, so `cargo xtask ork-flights` compares HPR Sim's height above its start and its speed at
two points. The first is OpenRocket's separation: its row just after the split, against HPR Sim's
sample at its separation event, the whole stack's center of mass. The second is OpenRocket's last
row. It is met when both separate at the same time and each value is within 5% of OpenRocket's,
the band an apogee gets before it needs a written cause ([ADR-172][adr-172]):

| | OpenRocket | HPR Sim | Δ |
|---|---:|---:|---:|
| separation, s | 0.86 | 0.86 | 0 |
| height at the separation, m | 22.44 | 22.18 | −1.17% |
| speed at the separation, m/s | 46.08 | 46.06 | −0.04% |
| height at OpenRocket's last row (1.81 s), m | 85.0 | 84.0 | −1.18% |
| speed there, m/s | 76.8 | 79.9 | +4.02% |

**This is weaker evidence than an apogee.** It is two points on the way up, and the second is
where OpenRocket's flight is breaking up. When OpenRocket aborts is its own call, too: the
committed probe of this configuration with nothing deployed
([Delays and ignition](#delays-and-ignition)) aborts at 2.40 s, not 1.81 s.

The booster's B6 burns out first, so the booster drops at 0.86 s with both pods' A3s still
burning, as in OpenRocket ([Delays and ignition](#delays-and-ignition)). Its least static margin,
in the weakest plane, is −4.21 calibres at the split: the same unstable sustainer as
`[C6-7; B6-0]` (above). The +4.02% in speed comes just as both rockets start to turn over:
OpenRocket's speed peaked at 77.5 m/s and is falling at its last row, while HPR Sim's still climbs.
The report's
[*Flights OpenRocket aborted*](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/openrocket-flights.md#flights-openrocket-aborted)
section has the row.

**Whether HPR Sim's sustainer turns over depends on how it is launched.** In the record's
conditions, a 0.15 m rod at 28.61° N with no wind, HPR Sim's sustainer turns over, its angle of
attack past 90°, at 2.04 s. From `hpr sim`'s defaults, a 1.5 m rail with no wind, it does not, so
the apogee `hpr sim` prints assumes the unstable sustainer stays upright. Why the two launches
differ is not traced. `hpr sim` prints the least margin of −4.21 calibres with a warning that the
rocket is unstable under power and its apogee is not a prediction
([Flight metrics: unstable under power](../physics/metrics.md#unstable-under-power);
[#335](https://github.com/nrdptel/fusionspace-eridanus/issues/335)).

[adr-069]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0069-hprs-flights-of-the-public-designs-against.md
[adr-167]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0167-a-part-s-drag-override-as-openrocket-flies-it.md
[adr-173]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0173-a-blunt-ellipsoid-s-subsonic-pressure-drag-from-hoerner.md

### A tilted launch rod

HPR Sim flies a tilted launch rod the way OpenRocket records it
([M2.2e5](../decisions-and-roadmap.md#m2-2e5), issue
[#173](https://github.com/nrdptel/fusionspace-eridanus/issues/173)). This is checked only against OpenRocket,
on one small probe airframe and one private design, from a 1 m rod in calm air. A tilted rod in wind (or with OpenRocket's
*launch into wind* setting), a longer rod and the rocket's roll on the rod are not tested.

OpenRocket stores the rod's angle from the vertical and the compass bearing it leans toward, as
[the conditions probe](#what-the-numbers-mean) found. The flight comparisons here give HPR Sim's
rail the same bearing and an elevation of 90 degrees less the tilt, so a rod 10 degrees from the
vertical is a rail at 80 degrees. Like the vertical rod before it, the rail has no friction. The
conversion is in the comparison tool (`cargo xtask ork-flights`), which takes the conditions from
OpenRocket's record. The `.ork` reader stores the file's degrees as radians, as the table above
says, but loading a design does not set up a rail: a program that flies it gives its own.

**How it was checked.** Four small probe designs fly the airframe of the
[pod probes](../physics/aero.md#pods) (a 0.2 m cone, a 0.6 m tube 60 mm across, three fins and an
AeroTech H128W) in both programs: from a rod tilted 5° toward north, 10° toward east, 10° toward
south-west and 20° toward east. The same airframe from OpenRocket's default vertical rod
(`pods-none`) is the control. From the
[public report's table](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/openrocket-flights.md#tilted-rod-probes)
(2026-09-27):

| rod | apogee lost to the tilt, OpenRocket | HPR Sim | where the rocket is at apogee, OpenRocket | HPR Sim |
|---|---:|---:|---|---|
| 5° toward north | 0.65% | 0.63% | 99.2 m north | 98.5 m north |
| 10° toward east | 2.60% | 2.52% | 195.0 m east | 194.3 m east |
| 10° toward south-west | 2.60% | 2.54% | 138.5 m west, 138.1 m south | 138.1 m west, 137.6 m south |
| 20° toward east | 10.09% | 9.82% | 370.6 m east | 369.6 m east |

At apogee both rockets are on the same bearing from the pad to within 0.03 degrees, and HPR Sim's is
at most 0.64% nearer. In calm air, the direction of the tilt changes only where the rocket goes,
not how high: the two 10° rods lose the same apogee. The test
`tilted_rods_fly_as_openrocket_flies_them` (in `xtask/src/ork_flights.rs`) holds each probe within
0.1 degrees of OpenRocket's bearing, 1% of its distance and 0.5 percentage points of the apogee it
loses ([ADR-094][adr-094]). The small differences left are partly OpenRocket's position being
read from its highest recorded step rather than the apogee itself, and partly sideways motion
across the tilt: HPR Sim's fits Earth's rotation in sign and size, and OpenRocket's, the other way,
has no measured cause.

**What differs: the margin.** OpenRocket's rocket reaches its first step past the rod's end
([rail exit](../glossary.md#rail-exit-and-rail-exit-velocity)) at a small
[angle of attack](../glossary.md#angle-of-attack) that grows with the tilt: none from the vertical
rod, 0.116 degrees at 10 and 0.228 at 20. Its center of pressure moves forward with the angle, so
its stability margin falls, by 0.016 calibres at 20 degrees. HPR Sim takes the margin at no angle of
attack, so there the margins part by 0.017 calibres. Taken at OpenRocket's angle, HPR Sim's margin
falls by 0.012 calibres, to 0.005 from OpenRocket's. So most of the gap is the angle at which each
program looks. The rest grows with the tilt too, because HPR Sim's center of pressure moves about a
fifth less than OpenRocket's for the same angle.

**Vertical rods too.** OpenRocket records a direction for a vertical rod as well (90 degrees by
default). On a vertical rail it only sets which way the fins face on the pad. Taking it as
recorded moved the public flights' apogees by under 0.001%, enough to change one rounded apogee
in the report by 0.1 m.

[adr-094]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0094-a-tilted-launch-rod-flown-as-openrocket-records.md

### OpenRocket's flights of the private designs

This section counts OpenRocket 24.12's flights of the private design library, the *corpus*: 27
`.ork` files, which stay out of this repository
([M2.2e2](../decisions-and-roadmap.md#m2-2e2), OpenRocket flies the corpus). They are flown the
same way as [the public designs](#what-the-summary-words-mean): every motor
[configuration](../glossary.md#configuration), in calm air, from the launch conditions of the
design's [first stored simulation](#what-openrocket-last-did-stored-simulations). HPR Sim's flights
of them are compared [below](#the-simulators-flights-of-the-private-designs).

The flights are written to the gitignored `corpus-out/`, because a flight's numbers can identify
someone's design. Only counts are published. `cargo xtask ork-flights --corpus` prints them from that
record, and never prints a file name.

Counts on 2026-09-25, by where the design was found. *Distinct* means distinct in content (by
SHA-256 hash). *Flown to the end* means the simulation ran to its end, not stopped by OpenRocket
(*aborted*). The last row shows each such flight recorded the
[stability margin](../glossary.md#stability-margin) the next comparison needs.

| | the private library | OpenRocket's examples | elsewhere under `refs/` |
|---|---|---|---|
| design files | 27, all distinct | 17 | 31, 25 distinct |
| refused by OpenRocket | 0 | 0 | 4 |
| configurations declared | 89 | 56 | 24 |
| with no motor OpenRocket loaded | 1 | 0 | 16 |
| aborted by OpenRocket | 0 | 1 | 0 |
| flown to the end | 88, in all 27 designs | 55, in all 17 | 8, in 4 designs |
| with a margin at rod clearance | 88 | 55 | 8 |

- OpenRocket flew every configuration of the private library it loaded a motor into. None was
  refused and none aborted.
- The one configuration without a motor names one in its file, with its thrust curve's digest, but
  OpenRocket loaded no motor for it, so it had nothing to fly.
- The record doesn't keep OpenRocket's warnings on loading a file, or the digest of the thrust
  curve each flight used. So a flight on a curve other than the one saved in the file would not
  show here. HPR Sim's comparison checks each curve against the motor record before it compares
  ([below](#the-simulators-flights-of-the-private-designs)).
- 15 of the 27 files are public designs: 13 are the very files of OpenRocket's examples, and two
  are edited copies of examples. So the library holds 12 private designs, other people's.
- All 27 files have a stored simulation, so every flight took its launch conditions from
  the design's first one. As those simulations were set, OpenRocket flew 64 of the 88 on a
  spherical Earth and 24 on a flat one. HPR Sim uses the
  [WGS 84](../glossary.md#wgs-84) ellipsoid, and the effect of that difference is not measured yet.
- The examples column is a new run of the 17 examples inside OpenRocket's jar from
  [M2.2d1](../decisions-and-roadmap.md#m2-2d1) (OpenRocket's flights of the public designs), with
  the same 56 configurations and one abort. That earlier run's 57 also counted a Loft demo, which
  is in *elsewhere* here.
- *Elsewhere* is mostly the test and demo designs of [Loft](../glossary.md#loft-lesson) and
  Debrief, the two projects this one took over, plus one private design log's `.ork` and nine from
  OpenRocket's database repository. OpenRocket refuses four of them. Most of their configurations
  have no motor.
- **Left out: the library's 4 RASAero `.CDX1` and 4 RockSim `.rkt` files.** HPR Sim can't read
  either format yet, so there is nothing to compare them with, and the flight script fails on three
  of them ([#168](https://github.com/nrdptel/fusionspace-eridanus/issues/168)). The decision is
  [ADR-071][adr-071].

To repeat it, you need the private library under `refs/`, which most readers won't have, plus Java
17 and the OpenRocket jar (`cargo xtask refs fetch`), as for the
[mass comparison](../physics/mass.md#checked-against-openrocket). Then, from the repository root:

```sh
# --jar also flies the 17 examples inside OpenRocket's jar
refs/venv/bin/python validation/oracles/openrocket/flights.py \
    corpus-out/openrocket-flights.json refs --jar
cargo xtask ork-flights --corpus
```

[adr-071]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0071-the-corpus-openrocket-flies-is-its-ork-files.md

### The simulator's flights of the private designs

This section compares HPR Sim's flights of the 12 private designs with OpenRocket's
([M2.2e3](../decisions-and-roadmap.md#m2-2e3), HPR Sim's flights of the corpus), as
[the public comparison](#the-simulators-flights-against-openrockets) does for OpenRocket's examples,
by the same definitions. **It is a [code-to-code](../glossary.md#code-to-code-comparison) comparison
with no target, and HPR Sim flies 11 of the 12 designs.** On four of them, `C03`, `C06`, `C08` and
`C09`, HPR Sim's stability margin is clearly larger than OpenRocket's, so it calls those rockets
more stable than OpenRocket does. `C03` and `C09` are open
([#172](https://github.com/nrdptel/fusionspace-eridanus/issues/172)); `C08` has a lead
([#186](https://github.com/nrdptel/fusionspace-eridanus/issues/186)); and `C06`'s is mostly its center of mass.
A likely cause is its airfoil fins, which OpenRocket weighs at 0.85 of a slab and HPR Sim at 0.6851
(the [mass survey's fin-section cause](../physics/mass.md#checked-against-openrocket)); that
cause is sized on the structure alone, not on this flight. A fifth, `C12`,
reads larger too, but mostly because it leaves a tilted rod at an angle of attack
([below](#a-tilted-launch-rod)). One flight, the only supersonic one, climbs 13.60% higher than
OpenRocket's, and the cause is the drag coefficient
([below](#a-supersonic-flight-and-a-cause-in-the-drag)). Nobody without the
private library can fly them again: CI checks only that the report adds up and names nothing of a
design.

**How far it gets.** [M2.2](../decisions-and-roadmap.md#m2-2), the OpenRocket comparison, asks for
at least 20 designs compared in five ways: apogee, largest speed, stability margin, mass and center
of mass. With the public report's 15, these make 26, so that count is met. Its mass conventions
([M2.2b](../decisions-and-roadmap.md#m2-2b)) are rolled up too, and two checks carried over from
Loft end it ([below](#m2-2-closes)): one passes, and one, the tube-fin center of pressure, is
missed, with the gap measured and pinned. Staging and clusters
([M1.9](../decisions-and-roadmap.md#m1-9)) added one of these private designs and three public
ones, a tilted launch rod ([M2.2e5](../decisions-and-roadmap.md#m2-2e5)) one private design, and
reading the old override flag as OpenRocket does ([M2.2e6](../decisions-and-roadmap.md#m2-2e6))
another. Weighing fin fillets and reading an automatic radius inside a nose cone
([M2.2e7](../decisions-and-roadmap.md#m2-2e7),
[#174: airframes read simpler than written](https://github.com/nrdptel/fusionspace-eridanus/issues/174)) added
two more, `C01` and `C06`, and flying tube fins
([M2.2e9](../decisions-and-roadmap.md#m2-2e9)) OpenRocket's *Tube fin rocket*. The twentieth,
`C04`, flies since [M2.2e10](../decisions-and-roadmap.md#m2-2e10): one of its motors is set to
light at an event that never comes, and HPR Sim now flies that motor unlit, as OpenRocket does
([above](#delays-and-ignition)). HPR Sim's apogee is 0.88% above OpenRocket's (the report's row
`C04/1`). It is a staged flight, the kind in which OpenRocket's recorded mass has gone wrong
before ([#185](https://github.com/nrdptel/fusionspace-eridanus/issues/185)). A check run locally on the private
file, not committed, found OpenRocket's mass falling by each burning motor's propellant along this
flight, so that fault does not show there; no committed check holds it.

<a id="m2-2-stays-open"></a><a id="m2-2-closes"></a>Two checks carried over from
[Loft](../glossary.md#loft-lesson), the project before this one, end the comparison
([M2.2f](../decisions-and-roadmap.md#m2-2f)). [L82](../decisions-and-roadmap.md#l82) passes;
[L19](../decisions-and-roadmap.md#l19) does not, and its gap is measured instead:

- [L19](../decisions-and-roadmap.md#l19): a tube fin set's center of pressure should be within a
  quarter [calibre](../glossary.md#calibre-caliber) of OpenRocket's. It is not. On the *Tube fin
  rocket* HPR Sim's is 1.07 calibres forward, which is nearly all of the 1.08 calibres between the
  two margins, HPR Sim's 0.79 and OpenRocket's 1.87. A test measures and pins the gap on 14 probe
  designs and on this rocket, and nothing measured says which code is nearer
  ([tube fins](../physics/aero.md#tube-fins-against-openrocket)).
- [L82](../decisions-and-roadmap.md#l82): a case let off a pass-or-fail bar still counts in the
  error statistics. A test holds the [accuracy census](../glossary.md#accuracy-census) to it. Every
  number the reports compare is counted, including the misses that have a written cause or
  explanation. A rocket with two references counts against both: RocketPy's flight of Juno III,
  for one, and its team's altimeter log. The results stored inside `.ork` files never enter the
  census, so this does not cover them.

Still not flown:

- stages HPR Sim can't separate yet: the second of the two private designs the old override flag was
  blamed for. Its flag reads the same either way; what holds it is a separation that comes while
  a motor behind it still burns. The two public payload designs that waited here fly since
  [M4.5g3](../decisions-and-roadmap.md#m4-5g3) (a payload's split);
- none held back by what HPR Sim leaves out any more. The library's copy of *Parallel booster
  staging* flies since [M4.5m](../decisions-and-roadmap.md#m4-5m) read parallel stages. The pods
  themselves fly since [M1.13c1](../decisions-and-roadmap.md#m1-13c1), the cockpit on *Pods--airframes and
  winglets*' nose since [M4.5g4](../decisions-and-roadmap.md#m4-5g4), and *Pods--powered with
  recovery deployment*, once held back by its rail buttons' screw heads, since
  [M4.5i](../decisions-and-roadmap.md#m4-5i).

**What is published.** The designs are other people's, so the
[report](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/openrocket-library-flights.md)
holds only differences: HPR Sim's number less OpenRocket's, in per cent for apogee, largest speed
and mass, and in calibres (OpenRocket's reference diameters) for the
[stability margin](../glossary.md#stability-margin), the center of mass (CG) and the center of
pressure (CP). A design's own values are never written: next to its difference, HPR Sim's value
would let anyone work out OpenRocket's, and so the design's. Each design is an id, `C01` to `C12`,
in the order of its file's SHA-256 hash, and each flight is the design's id and the configuration's
place in the file: `C09/2` is the second configuration of design `C09`. A CI test checks that the
report holds only these ids, reasons and differences from fixed lists, rounded so that no rounding
residue gives a value back, and that its summary matches its rows.

**Which flights are compared.** HPR Sim flies a configuration only when it can show both programs
used the same thrust curves. Each curve must be the one saved in the design file, or one from
OpenRocket's motor database matched by its digest (OpenRocket's fingerprint of a curve), and the
record of OpenRocket's run must show it loading exactly those curves. One configuration is left out
because HPR Sim's curve came from its own catalog, found by the motor's name. A tilted rod is flown
as recorded, as in [A tilted launch rod](#a-tilted-launch-rod).

The 35 flights (from the committed report, 2026-09-28):

| metric | flights | median | from | to |
|---|---:|---:|---:|---:|
| apogee, no named cause | 28 | −0.31% | −4.84% | +1.17% |
| apogee, OpenRocket's parachute open before apogee | 6 | −0.32% | −3.09% | +2.15% |
| apogee, HPR Sim's own drag coefficient | 1 | +13.60% | +13.60% | +13.60% |
| largest speed, no named cause | 34 | +0.27% | −0.65% | +2.28% |
| largest speed, HPR Sim's own drag coefficient | 1 | +7.98% | +7.98% | +7.98% |
| margin at rod clearance | 35 | +0.0308 cal | −0.0166 cal | +0.1108 cal |
| mass at launch | 35 | +0.000% | −0.770% | +0.070% |
| center of mass at rod clearance | 35 | −0.0004 cal | −0.1102 cal | +0.0198 cal |

For example, `C09/9` reads −4.84% in apogee: HPR Sim's rocket peaks 4.84% lower than OpenRocket's on
the same design and motor. Its largest speed is +0.26%, so the two agree on the climb under thrust
and part on the coast, where drag matters most.

- One apogee is more than 5% from OpenRocket's: `C06/1`, +13.60%. Its cause is sized
  [below](#a-supersonic-flight-and-a-cause-in-the-drag). No flight has a part with a drag override.
- An early parachute lowers OpenRocket's apogee, so it can make HPR Sim read high but not low. It
  cannot explain the four that read low (`C03/3`, `C03/4`, `C02/4`, `C02/5`). It could explain `C07/1`, which reads +0.68%
  with the parachute 0.55 s early, and `C02/2`, which reads +2.15% with the parachute 1.45 s
  early, the longest of the six; how much of either it explains is not measured.
- The 14 flights of `C03` and `C09`, launched above sea level, all read low in apogee. The 15th
  above sea level, `C06/1`, reads high for its drag (below). Of the 20 at sea level, the 4 of
  `C07`, `C08` and `C11` read high, and so does `C04/1`, flown since a motor that never lights
  flies; `C01`, flown since fillets are weighed, once each way; `C02`, flown since pods fly,
  reads high once and low four times; `C05`, flown since the old override flag is read as
  OpenRocket reads it, high four times and low once, all within 0.26%; and `C12`, flown since a
  tilted rod flies, high once and low twice. Most of the public report's flights with no
  named cause read low as well (16 of 21), and every public flight launches at sea level (its
  [record](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/fixtures/ork/openrocket-flights.json)
  gives a launch altitude of 0 m throughout). So altitude does not yet explain the sign.
  [M2.2e4](../decisions-and-roadmap.md#m2-2e4) sized only the apogees more than 5% off, so this
  pattern is still untested.
- **HPR Sim's margin is larger than OpenRocket's** on four designs: HPR Sim calls them more stable,
  the direction to worry about. `C03` and `C09` come first, then `C08` and `C06/1`. `C03` reads
  +0.056 to +0.073 calibres: on every flight HPR Sim's CP sits 0.061 calibres further aft than
  OpenRocket's, and the CG accounts for the rest. `C09` reads about +0.04, half from its CP (+0.020)
  and half from its CG, which HPR Sim puts forward of OpenRocket's by 0.0145 to 0.0273 calibres. The
  reference diameters agree on all 35 flights, so the calibres are the same. On the public designs
  the margin gap is at most 0.0151 calibres, and the largest (−0.0151) has HPR Sim calling the
  rocket *less* stable. No milestone covers it yet: it is
  [#172](https://github.com/nrdptel/fusionspace-eridanus/issues/172). A third design, `C08`, a
  two-stage rocket flown since the [M1.9c milestone](../decisions-and-roadmap.md#m1-9c), reads
  +0.1108 calibres because HPR Sim puts its CG 0.1102 calibres forward of OpenRocket's at rod
  clearance, with the mass there within about 0.001%. That gap is older than the staging: HPR Sim's
  structure alone puts the CG forward by less than the mass survey's 1% of length. A stage whose
  mass is set at several times its parts' own weight magnifies where those parts sit, and two of
  them differ: an airfoil fin set, a departure HPR Sim keeps on purpose ([ADR-062][adr-062]), and
  packed parachutes whose automatic radius OpenRocket may meet by stretching the packed length
  ([#186](https://github.com/nrdptel/fusionspace-eridanus/issues/186)).
- `C05` reads within 0.0007 calibres on four flights; on `C05/2` it reads +0.0113, nearly all
  from its CG (−0.0111 calibres), with the mass there within 0.005%. Not traced.
- `C06/1` reads +0.0604 calibres, +0.0540 at OpenRocket's angle of attack. Most of it is its CG,
  0.0447 calibres forward of OpenRocket's, with HPR Sim's mass at launch 0.770% below OpenRocket's.
  The mass survey names a sized cause for this design: its airfoil fins. OpenRocket weighs an
  airfoil fin at 0.85 of a square slab of its outline, and HPR Sim at 0.6851, so HPR Sim's fins are
  lighter. Weighed OpenRocket's way, the design's structure comes within the survey's 1% thresholds
  ([the fin-section cause](../physics/mass.md#checked-against-openrocket)). Those thresholds are
  coarser than this flight's 0.0447 calibres, and no flight has been flown with OpenRocket's fin
  weighting, so the fins are a lead for this gap, not its size.
- `C01` reads −0.0166 and +0.0025 calibres. On `C01/1` HPR Sim calls the rocket *less* stable, from
  a CG 0.0198 calibres further aft.
- `C12`, launched from a tilted rod, reads +0.0125 to +0.0366 calibres, nearly all from its CP.
  OpenRocket's rocket clears the rod at an angle of attack, as on
  [the probes](#a-tilted-launch-rod). With HPR Sim's CP taken at that angle (the report's *at OR's
  α*), `C12` reads +0.0033 to +0.0074, so most of its gap is the angle each program looks at.
- HPR Sim's [design checks](../physics/design.md#checks) find errors on 8 of the flights. On
  `C07/1`, `C11`'s two and `C12`'s three, an inner part is wider than its parent's bore by more
  than the fit tolerance; on `C06/1` a motor's case can't fit its mount; on `C08/1` a fin is off
  its body tube. On all of `C03`'s flights the inner part is within the tolerance, so it only
  warns ([ADR-155][adr-155], the decision on which fits warn). Such a part puts mass in a slightly
  different place, not lift, so it cannot move the CP, and `C03`'s CG agrees within 0.013 calibres.
- The one design HPR Sim does not fly waits on stages HPR Sim can't separate as written: a
  separation that comes while a motor behind it still burns. The report lists each
  configuration with its coarse reason; the breakdown is in [ADR-072][adr-072] (HPR Sim's flights of
  the private library) and [ADR-095][adr-095] (the old override flag).

To repeat it, you need the private library, OpenRocket's flights of it and the motor record, as
[above](#openrockets-flights-of-the-private-designs), and OpenRocket's drag curves. Then:

```sh
refs/venv/bin/python validation/oracles/openrocket/drag_curves.py \
    corpus-out/openrocket-drag-curves.json refs
cargo xtask ork-flights --library           # writes the report
cargo xtask ork-flights --library --check   # compares with the committed one
```

#### A supersonic flight, and a cause in the drag

`C06/1` is the first supersonic flight in either OpenRocket report. HPR Sim's apogee is 13.60% above
OpenRocket's, and its largest speed 7.98% above. The mass at launch is within 0.77%, and the
difference builds during the burn. No parachute opens early, and no part states its own drag
coefficient. [M2.2e4](../decisions-and-roadmap.md#m2-2e4) (causes for apogees
more than 5% off) asks for a written cause with a size, not just a candidate.

Part by part, with the rocket pointing into the airflow, OpenRocket's drag differs from HPR Sim's in
two ways, and they pull opposite ways:

- **Base drag while a motor burns.** HPR Sim takes the burning motor's cross-section off the aft
  base, following Niskanen (who cites Fleeman for it): a base the size of the motor has no base
  drag. OpenRocket 24.12 keeps the whole base's drag while the motor burns. On `C06` the motor fills
  most of the base, so HPR Sim has less drag under power and climbs higher
  ([Aerodynamics](../physics/aero.md#drag)). What OpenRocket does is measured from its own output.
  Which rule is right is not: neither has been checked against a measured flight. On `C06/1` the
  choice moves HPR Sim's apogee difference from +13.60% to −10.71%, about 24 percentage points, more
  than any other known cause
  ([#222](https://github.com/nrdptel/fusionspace-eridanus/issues/222), open on both drag questions).
  - The measurement: a committed probe, `validation/oracles/openrocket/base_drag.py`, records
    OpenRocket's base drag on its own example designs, the rows while a motor burns against the
    rows after, in `validation/fixtures/ork/openrocket-base-drag.json`. On all 42 of its
    flights of one data branch (nothing separating), OpenRocket's base drag while
    a motor burns is exactly what the whole base gives, to 1e-12. It does not subtract the motor,
    even where the motor covers 94% of the reference area, and the drag it flies is the sum that
    includes that base drag. A test in `hpr-validate` holds it ([ADR-097][adr-097], the decision
    on sizing a drag cause).
  - Motors in [pods](#pods) are among them: the one such flight of OpenRocket's
    powered-pods example that burns pod motors keeps its base whole too, and HPR Sim's opt-in treats
    a pod's motors the same way.
- **Supersonic pressure drag.** This is the drag from the pressure on the nose, the fins' edges and
  any step faster than sound, much of it [wave drag](../glossary.md#wave-drag). HPR Sim's is about
  twice OpenRocket's, read from OpenRocket's per-component output but not kept as a record. OpenRocket gives the nose almost none well above Mach 1, and the fins about a
  quarter of HPR Sim's. That gives HPR Sim more drag
  ([#222: which supersonic pressure drag is right](https://github.com/nrdptel/fusionspace-eridanus/issues/222)).

**How the cause is sized.** HPR Sim flies the flight again on OpenRocket's own drag coefficient. An
oracle, `validation/oracles/openrocket/drag_curves.py`, flies each configuration as `flights.py`
does, with nothing deployed. From launch to apogee it records OpenRocket's drag coefficient as two
curves in Mach number: *power on*, the rows while a motor burns, and *power off*, the rows after the
last one that burns. `cargo xtask ork-flights --library` flies HPR Sim on them as a
[drag table](../physics/aero.md#drag), for any apogee more than 5% off with no other named cause
and no stage separation. Where HPR Sim's apogee then comes within 5% of OpenRocket's, the apogee's
cause is *HPR Sim's own drag coefficient*, and the same goes for the largest speed, judged on its
own number. That says the difference is in the drag, not which drag is right, so each such flight
also gets a written breakdown, held by a test ([ADR-097][adr-097], the decision on sizing a drag cause).

The report also flies HPR Sim's own drag with OpenRocket's base rule alone, through
`Simulation::with_full_base_drag_under_power` ([Aerodynamics](../physics/aero.md#drag)):

| `C06/1`, flown by HPR Sim on | apogee Δ | largest speed Δ |
|---|---:|---:|
| its own drag | +13.60% | +7.98% |
| OpenRocket's drag coefficient | +1.11% | +1.04% |
| its own drag with OpenRocket's base rule | −10.71% | −3.43% |

On OpenRocket's drag the flight is within the 5% bar, so the drag is the cause. Switching HPR Sim to
OpenRocket's base rule alone lowers its apogee from +13.60% to −10.71%, 24.3 percentage points.
Switching the rest of the drag to OpenRocket's then raises it to +1.11%, 11.8 percentage points.
A look at OpenRocket's per-component columns, not kept as a record, points that rest to the
supersonic pressure drag. The second number is by subtraction, and the split depends on which
change is made first. Which supersonic pressure drag is right is open: neither code has been
checked against a measurement on this shape, and the public report has no supersonic flight.

The drag curves are OpenRocket's numbers for private designs, so they stay in the gitignored
`corpus-out/` with the other records.

[adr-072]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0072-hprs-flights-of-the-private-library-under.md
[adr-095]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0095-the-single-pre-1-9-override-flag-read-as.md

### Stored simulations in the reference library

`cargo xtask ork`, over the 73 readable files, on 2026-09-25:

| quantity | count |
|---|---|
| stored simulations | 174, in 61 documents: 139 `uptodate`, 17 `external`, 11 `outdated`, 7 `notsimulated` |
| stored reference screen | 91 eligible, 83 excluded: 47 inconsistent, 17 external, 11 outdated, 7 not-simulated and 1 missing simulator; only the 91 may enter a stored-data reference denominator (the count of runs used to calculate a reference statistic) |
| HPR Sim reproduction screen | of those 91, 40 can be reproduced when [OpenRocket's database](#motors-in-the-reference-library) is supplied (1 without it); 51 can't: 40 because their configuration can't be flown, 11 because the design is reduced |
| with launch conditions | 173; 129 state the wind's direction, and 135 launch into the wind |
| atmosphere | 172 `isa`, 1 not written |
| with a summary | 162 |
| with a time series | 142, over 176 stage branches and 97,541 rows |

The screen counts are separate from the unconditional census below: conditions, summaries, time
series, branches, rows and events are counted across all 174 stored simulations, including runs
that fail either screen. The counts are an as-of snapshot of this private corpus, not a universal
property of every OpenRocket file.

How the stored-result policy was decided is in [ADR-065][adr-065]; the underlying read-back policy
is in [ADR-057][adr-057].

[adr-065]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0065-stored-results-are-references-only-when-current.md
[adr-062]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0062-fins-and-rail-buttons-against-openrocket-roll.md
[adr-057]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0057-a-ork-designs-stored-simulations-read-back-as.md

## What the simulator keeps for writing the file back

**In short.** Some of what a `.ork` holds, HPR Sim's design does not model: a parallel stage it
can't read, OpenRocket's 3D-view settings, a simulation's plug-ins, a part's color, a material's group. HPR Sim keeps
each whole, beside
the design, in an *extension* (a named slot for data another program wrote) called `x-openrocket`,
at a path that leads back to where it was, so that writing the file back out puts it back
([writing a `.ork` back out](#writing-a-ork-back-out)). A design whose rocket is missing parts
this way says it is **reduced**; the flag is on the design, not on its rocket, so check it before
using the rocket on its own.

Four kinds of thing are kept: parts, sections, tags and attributes. A tag or attribute is kept
for one of two reasons: no reader asked for it, or the reader that asked dropped what it says.

- **Parts** HPR Sim does not read: a parallel stage outside a body tube, or on a rocket of several
  stages ([Parallel stages](#parallel-stages), [L66](../decisions-and-roadmap.md#l66)), a pod set it cannot lay out ([Pods](#pods)), a part HPR Sim
  cannot give an honest shape ([above](#what-is-left-out-and-why)), or a tag it has never seen.
- **Sections** of the document HPR Sim does not read: `<photostudio>` (the 3D view), `<docprefs>`
  (the design's own materials), anything else beside `<rocket>` and `<simulations>`, and the parts
  of a stored simulation beyond its conditions and results, such as an `<extension>`.
- **Tags** no reader asks for, inside a part, stage or stored simulation HPR Sim does read, or
  inside a tag it does read: a part's color (`<appearance>`), a catalog preset, a comment, a wind's
  standard deviation, a drag coefficient stated on the `<rocket>` element itself, or a tag HPR Sim
  has never seen. HPR Sim records every tag its readers ask for while it reads a file, so a tag is
  kept when nothing asked for it.
- **Tags a reader asks for and then drops** are kept too, because the design does not hold what
  they say. Examples: an override on a pod set that holds nothing,
  a ring written as a row of three, a tube fin set of more than eight tubes, a fin section or
  finish HPR Sim has no reading for, or the older of two names for one value when the two disagree. A value HPR Sim reads exactly, such
  as a fin set's count, is not kept.
- **Attributes** no reader asks for, on an element HPR Sim does read: a material's `group`, an
  event's `id`, or the reference an angle or radius offset is measured from, which HPR Sim does not
  read yet but for a pod set's `radiusoffset` ([Pods](#pods);
  [issue #145](https://github.com/nrdptel/fusionspace-eridanus/issues/145)). An attribute whose value a reader
  drops is kept too, such as a material's declared kind where the part needs another.

**A kept value goes back as it was.** When the design is written out as a `.ork` again, each kept
tag goes back in place of anything the writer would have written under that name from the design.
Take a ring written as a row of three. HPR Sim reads one ring and warns, and the file it writes
still says `<instancecount>3</instancecount>`. So OpenRocket reads the rings the original had.
Reading the written file gives the same warning. As with the original, that warning means HPR Sim
doesn't fly the rocket's configurations, since only a rocket read without such a warning flies
([ADR-055](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0055-m3-1c-split-and-the-motors-a-ork-flies-its-own.md)).
A rail button's screw height was this page's example until
[M4.5i](../decisions-and-roadmap.md#m4-5i): the design holds it now
([`RailButton::screw_height_m`](../api/hpr_design/parts/struct.RailButton.html)), and the writer
writes `<screwheight>` from the design.

Each is kept with its **path**, such as `openrocket/rocket/stage[0]/bodytube[1]/podset[0]`: the
podset that is the first part inside the second part of the first stage. A part counts among all
the parts beside it, since their order is where they stack. A tag's step starts with `@` and
counts only among the tags of its own name: `openrocket/rocket/stage[0]/nosecone[0]/@appearance[0]`
is that nose cone's first `<appearance>`, wherever it stood, and a tag inside it adds another,
such as `…/@wind[0]/@gusts[0]`. A section counts the same way. A tag that belongs to one
configuration, by its `configid`, names it and counts only among that configuration's tags of its
name: `…/@deploymentconfiguration(b)[0]`. OpenRocket reads no meaning into
the order of tags, and counting by name lets an export put each tag back without knowing where
it stood ([ADR-109](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0109-m3-2-split-and-a-ork-written-from-the-design.md)). An attribute is kept with
the path of the element it was on, its name and its value. The function `hpr_io::ork::element_at`
follows a path back to the element.

The extension is written under its namespace when the design is saved as JSON, and reads back
unchanged. The test `hpr_io::ork::tests::unknown_content_round_trips_through_x_openrocket` checks
both, and that each kept element is the one at its path in the file:

```rust
let json = serde_json::to_string(&design.extensions).expect("JSON");
assert!(json.starts_with(r#"{"x-openrocket":"#), "{json}");
let back: Extensions = serde_json::from_str(&json).expect("read back");
assert_eq!(back, design.extensions);
```

**What is not kept.** The text of a second copy of a tag a reader takes once by name. A reader asks
for a tag by name and uses the first copy, so the second's own value is taken as read, though its
attributes and anything unread inside it are kept. 13 fin tabs in the library carry a second
`<tabposition>` this way. That text, and everything else, is still in the document itself, which
HPR Sim keeps whole when it opens a file ([ADR-051][adr-051]).

### Kept in the reference library

`cargo xtask ork`, over the 73 readable files, on 2026-10-05, after
[M4.5m](../decisions-and-roadmap.md#m4-5m) read parallel stages:

| quantity | count |
|---|---|
| parts kept | 1, in 1 reduced design: the parallel stage directly under the rocket in `demo-quirks.ork` ([Parallel stages](#parallel-stages)). There were 3, in 3, before parallel stages were read. On 2026-09-23 there were 17, in 10; pods ([Pods](#pods)), tube fins ([Tube fins sized from the body](#tube-fins-sized-from-the-body)), a tube coupler and 2 freeform fin sets on a nose cone ([Fins on a nose cone or a transition](#fins-on-a-nose-cone-or-a-transition)) are read since |
| sections kept | 87: 42 `<photostudio>`, 36 `<docprefs>`, 9 simulation `<extension>`s |
| tags kept | 1,857, most often a part's `<appearance>` (309), `<radialdirection>` (170), `<instanceseparation>` (159), a wind's `<standarddeviation>` (129) and `<preset>` (127). None of them is a value a reader drops; 16 were, rail buttons' `<screwheight>`, before [M4.5i](../decisions-and-roadmap.md#m4-5i) read it into the design. Parts' drag overrides, 2 with their 2 flags before [M4.5h](../decisions-and-roadmap.md#m4-5h) (a part's drag override), are read into the design since |
| attributes kept | 3,209, most often an event's `id` (1,623), a material's `group` (594), an active stage's `number` (201) and a stored branch's optimum altitude and its time (168 each) |
| kept elements and attributes found again at their path | 5,154 of 5,154 (the survey fails if one is not) |

How this was decided is in [ADR-058][adr-058].

[adr-058]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0058-what-a-ork-holds-that-hpr-does-not-model-kept.md
[adr-064]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0064-clusters-fillets-and-unread-parts-remain-visible.md
[adr-096]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0096-fin-fillets-and-an-automatic-radius-inside-a.md
[adr-097]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0097-a-cause-in-the-drag-sized-by-hpr-flying.md
[adr-098]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0098-a-tube-fin-sets-automatic-radius-read-as.md
[adr-099]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0099-tube-fins-flown-as-ring-wings.md
[adr-075]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0075-a-cluster-is-one-tube-repeated-and-a-motor-in-it.md
[adr-076]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0076-a-ork-files-ignitions-and-one-powered-separation.md

## Writing a `.ork` back out

**In short.** `hpr_io::ork::export` writes a design as a `.ork` file for OpenRocket 24.12: schema
1.10 (the file-format version OpenRocket 24.12 writes), zipped with the design as `rocket.ork`,
as OpenRocket packs one. It writes from HPR Sim's design alone, not from a copy of the file it came
from, so a design built in HPR Sim is written the same way. One exception: a value kept from the
original file is written as the file had it, even after the design is edited. Read back, the
written file gives the same design, bit for bit, for each of the 73 designs HPR Sim reads among the
`.ork` files the checks use ([which files](#checked-in-openrocket)). Some are other people's
private designs, so the counts are published but those files are not. OpenRocket 24.12 opens
every written file whose original it opens, and flies each configuration it can fly (151) to
the original's apogee within 0.5%: the largest difference is 0.000162%.

The written file is not a copy of the original. It is laid out afresh, and it states some values
the original left to OpenRocket's defaults, such as each part's roll angle. Its root's `creator`
attribute names the program that wrote it, its version and its designation in the FusionSpace
product system, as every file HPR Sim writes does:
`creator="FusionSpace HPR 0.1.0 · FS-ACHERNAR · SW · TOOL 001"` (`hpr-sim` before release 0.1's
rename; a file with that `creator` still reads).
The measurements above were made before the attribute took this form; opening such a file in
OpenRocket has not been checked since.

**How it is written.** Each part of the design is written as the tags HPR Sim reads it from. What
HPR Sim keeps but doesn't model goes back where it was
([what HPR Sim keeps](#what-the-simulator-keeps-for-writing-the-file-back)): a parallel stage
HPR Sim can't read in its place among the parts, a part's color in the part, a simulation's plug-in
in the simulation. A value HPR Sim read and dropped goes back as the file wrote it. A ring written
as a row keeps its `<instancecount>`, even though HPR Sim reads one ring. A parallel stage HPR Sim
read goes back inside its body tube ([Parallel stages](#parallel-stages)). Two functions do the
work:

- `export::document(&design)` gives the document, with a warning for anything kept that has no
  place any more.
- `export::write(&design, &attachments)` gives the file's bytes. The attachments are the
  original's other zip entries (`OrkFile::attachments`), such as a preview image.

The API reference has a worked example
([`hpr_io::ork::export`](https://hpr.fusionspace.co/api/hpr_io/ork/export/index.html)).

**Seven mistakes of Loft's exporter** ([Loft](../glossary.md#loft-lesson) is the project before
this one) are pinned by two tests, `round_trip_keeps_delays_ignition_conditions_and_override_flags`
([L67](../decisions-and-roadmap.md#l67)) and `fin_points_clusters_and_floats_round_trip_exactly`
([L68](../decisions-and-roadmap.md#l68)), in `hpr_io::ork::export::tests`. They run on invented
designs, as does a third test holding a small design's written document to one worked out by
hand:

| Loft wrote | HPR Sim writes |
|---|---|
| a plugged delay as 0 s | `none`, which reads back as plugged |
| ignition only for the whole mount | each configuration's own ignition event and delay |
| no launch conditions | every condition of every stored simulation |
| masses it had worked out, as overrides | only the overrides the design has, with their flags |
| a freeform fin as a trapezoid of the same area | every point of the outline |
| a cluster without its scale and rotation | the cluster's pattern, scale and rotation |
| each number to six decimals | the shortest number that reads back to the same bits |

**Numbers to the last bit.** A length of 0.1 + 0.2 meters is written `0.30000000000000004`,
because `0.3` would read back as a different number. Angles are stored in radians and written in
degrees. The writer picks the shortest number of degrees that converts back to exactly the stored
radians: a fin set at 45° is written `45`.

**What OpenRocket insists on.** These were found by opening written files in OpenRocket 24.12, and
flying them ([checked in OpenRocket](#checked-in-openrocket)). Reading a file back in HPR Sim is not
enough: before the last of these was found, every design read back the same, yet one of
OpenRocket's own examples flew 0.6% to 0.9% low and a private design 10% to 11% low.

- An id that isn't a UUID (a 36-character code such as `0f0e0d0c-0b0a-4900-8800-070605040302`)
  makes OpenRocket refuse the whole file. So such an id is left out, and OpenRocket gives the part
  one of its own. A part the file gave no id reads back with the same id HPR Sim made up for it.
- OpenRocket reads an inner tube's roll angle, and a parachute's or a mass's, only under the older
  tag `radialdirection`. So they are written under that tag.
- OpenRocket applies a part's catalog preset (`<preset>`, a maker's part number) at the moment
  it reads that tag, over the sizes and material it has read so far. HPR Sim keeps the tag without
  reading it, and writes it first, after the part's name and id, as OpenRocket does. Written
  after the part's own sizes, it had OpenRocket fly the catalog's part instead: one design flew
  11% low.

**What is still lost.** None of these happens in the reference library:

- A tag the file leaves out, where HPR Sim assumes a value (a radius with nothing to take, a missing
  wall or density). The written file states the assumption, so reading it again no longer warns.
- A second motor in one mount for one configuration.
- A configuration with no id, or one declared twice.
- Simulation rows and events HPR Sim left out on reading.
- The text of a second copy of a tag ([above](#what-the-simulator-keeps-for-writing-the-file-back)).

After an edit to the design, a kept tag's text still wins over the edit. Change a ring written as
a row in HPR Sim and its original `<instancecount>` is written as it was. A rail button's screw height is the
design's since [M4.5i](../decisions-and-roadmap.md#m4-5i), so an edit to it is written.

**Checked on the reference library.** `cargo xtask ork` writes every design out, reads it back,
compares the two, and writes the design read back once more. It fails on any difference. On
2026-09-29:

| quantity | count |
|---|---|
| designs written | 73 |
| read back as the same design | 73 |
| written again the same, byte for byte | 73 |
| export warnings | 0 |

```bash
cargo xtask ork                             # counts, and fails if a design comes back different
cargo xtask ork --export corpus-out/written  # also saves each written file, by the original's path
```

How this was decided is in [ADR-109][adr-109].

[adr-109]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0109-m3-2-split-and-a-ork-written-from-the-design.md

### Checked in OpenRocket

**In short.** OpenRocket 24.12 flew each design twice: as the original file, and as HPR Sim's export
of it. Every export opened where its original did, and every
[configuration](../glossary.md#configuration) that flew both ways reached the same apogee within
0.5%. Both flights are OpenRocket's, so this checks the file HPR Sim writes, not HPR Sim's physics;
how HPR Sim's own flights compare is in
[HPR Sim's flights against OpenRocket's](#the-simulators-flights-against-openrockets). HPR Sim
itself flies a written file exactly as it flies the original, since it reads back the same design.

**The files.** The check uses every `.ork` file the survey (`cargo xtask ork`) finds:

| where | files |
|---|---|
| the example designs that ship with OpenRocket | 17 |
| the private design library ([the corpus](#openrockets-flights-of-the-private-designs); the report's `refs/loft-fixtures`) | 27 |
| other `.ork` files under `refs/`, mostly Loft's and Debrief's test designs ([listed above](#openrockets-flights-of-the-private-designs); the report's *elsewhere under refs/*) | 31 |
| all | 75 |

HPR Sim reads 73 of them; the other two are malformed XML, which OpenRocket refuses too.
OpenRocket's examples and most of the other files are public.

**How they are flown.** Each flight takes its launch conditions from the file's
[first stored simulation](#what-openrocket-last-did-stored-simulations), or OpenRocket's defaults
when it has none, in calm air, with random seed 1. So a difference between the two flights comes
from the file alone. A configuration passes when the export's apogee is within 0.5% of the
original's. The 0.5% is the roadmap's bar
([M3.2](../decisions-and-roadmap.md#m3-2), writing `.ork` files), set before measuring. The
flights agree about 3,000 times more closely, so the report also gives the largest difference.

| quantity | count |
|---|---|
| designs | 75 |
| opened as written | 71 |
| opened as exported, of those | 71 |
| designs with a configuration flown both ways | 48 |
| configurations of the opened designs | 169 |
| configurations flown neither way | 18 |
| configurations flown both ways | 151 |
| within 0.5% of the original's apogee | 151 |
| the same apogee to the last bit | 142 |
| largest apogee difference | 0.000162% |

- **Four designs don't open as written.** HPR Sim can't read two of them, so it writes nothing.
  OpenRocket refuses the exports of the other two with the same error as their originals.
- **23 opened designs have nothing to fly.** They have no configuration, or none with a motor.
- **Eighteen configurations fly neither way.** Seventeen have no motor. OpenRocket aborts the
  other one both times, for the same cause.
- **Nine configurations differ, but by very little.** All nine are in the private design library
  (the report's `refs/loft-fixtures` row). The largest
  difference is 0.000162%. Their cause has not been traced.

The committed report,
[`openrocket-export-flights.md`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/openrocket-export-flights.md),
gives these counts by where the designs came from. The flights themselves stay in the gitignored
`corpus-out/`, since they could identify private designs
([OpenRocket's flights of the private designs](#openrockets-flights-of-the-private-designs)).

**Running it again.** It needs Java 17, the OpenRocket jar fetched by `cargo xtask refs fetch`,
and a Python environment at `refs/venv` with JPype, which lets the [oracle](../glossary.md#oracle)
scripts drive OpenRocket, as for [OpenRocket's mass](../physics/mass.md#checked-against-openrocket).
Without the private library, the commands fly the public files alone.
The first command flies the originals, and is needed only when `refs/` or the flight scripts
change. The second writes each design back out; the third flies those files; the last compares:

```text
refs/venv/bin/python validation/oracles/openrocket/flights.py corpus-out/openrocket-flights.json refs --jar
cargo xtask ork --export corpus-out/ork-export
refs/venv/bin/python validation/oracles/openrocket/flights.py corpus-out/openrocket-export-flights.json corpus-out/ork-export
cargo xtask ork-export-flights
```

The last command stops with an error when the flights are stale: when HPR Sim would now write
different files, when an original has changed since it was flown, or when the two sets of
flights used a different OpenRocket or different scripts. With `--check`, it compares its result
with the committed report instead of writing it.

## What is not read yet

A parallel stage on a rocket of several stages on the axis, or anywhere but inside a body tube,
is kept, not modeled ([what HPR Sim keeps](#what-the-simulator-keeps-for-writing-the-file-back)).
One inside a body tube on a rocket of one stage on the axis is read and flown
([Parallel stages](#parallel-stages)), and so are pods ([Pods](#pods)).

What keeping the whole document buys you is this: when HPR Sim meets a part it does not model, it
can say so and carry the part's own XML along untouched in `x-openrocket`, rather than dropping it
silently the way Loft did with pods and parallel stages. The writer puts that XML back where it
was ([writing a `.ork` back out](#writing-a-ork-back-out)). [ADR-064][adr-064]
confirms that the reduced flag and this preserved content are the current rule for b4, not a hidden
mass estimate.

[adr-061]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0061-what-a-ork-leaves-unsaid-read-as-openrocket.md
