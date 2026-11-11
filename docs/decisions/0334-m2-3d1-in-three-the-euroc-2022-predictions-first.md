# ADR-334: M2.3d1 in three, the EuRoC 2022 predictions first (2026-11-10)

- **Status:** accepted; splits [M2.3d1][m2-3d1] into [M2.3d3][m2-3d3], [M2.3d4][m2-3d4] and [M2.3d5][m2-3d5], each with its own *done when*, M2.3d1's *done when* whole across the three, and so plans every remaining increment of M2.3d1; carries out M2.3d3 under [ADR-328][adr-328] §1a. [M2.3d2][m2-3d2], the errors, stays last, after all three. [ADR-336][adr-336] reopens M2.3d4 in three and puts M2.3d5 before what is left of it.
- **Summary:** HPR Sim can't fly the Macie XL design yet: its stage's mass override adds to its parts, and HPR Sim holds such a stage back. So the five predictions merge in two pull requests, and no error is computed until both have merged. This record merges the first three, flown from their example notebooks in their launch hours' ERA5 air: Astra 3,037.1 m (9,964 ft), Andromeda 3,581.0 m (11,749 ft) and Erebus 11 2,714.1 m (8,904 ft), as their altimeters would read them. Andromeda is counted on its team's thrust file and tagged *not traced*; the catalog motor's 3,050.3 m (10,008 ft) is shown beside it and never counted.

[m2-3d1]: ../decisions-and-roadmap.md#m2-3d1
[m2-3d2]: ../decisions-and-roadmap.md#m2-3d2
[m2-3d3]: ../decisions-and-roadmap.md#m2-3d3
[m2-3d4]: ../decisions-and-roadmap.md#m2-3d4
[m2-3d5]: ../decisions-and-roadmap.md#m2-3d5
[adr-082]: 0082-real-flights-read-from-refs-compared-over-the.md
[adr-328]: 0328-the-2026-11-08-review-twelve-public-flights-predictions-first.md
[adr-336]: 0336-m2-3d4-in-three-the-inertia-and-the-refusals-last.md
[i1225]: https://github.com/nrdptel/fusionspace-eridanus/issues/1225
[ork-stage]: ../format/ork.md#the-values-inside-the-tags
[era5-convert]: ../format/era5.md#converting-a-current-file
[andromeda]: ../research/andromeda-impulse.md
[predictions]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/public-predictions.md
[record]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/fixtures/weather/euroc-2022-conversion.json
[script]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/oracles/netcdf/euroc_2022.py
[apogee]: ../glossary.md#apogee
[era5]: ../glossary.md#era5
[netcdf]: ../glossary.md#netcdf

**Context.** [ADR-328][adr-328] §1a asks for one pull request that merges five new public
flights' inputs, sources and predicted [apogees][apogee] before any of their errors is computed:
Astra, Andromeda and Erebus 11 from RocketPy's examples, and two flights of the Macie XL design.

The Macie XL design can't be flown today. `hpr sim` refuses it with *the stage's override doesn't
cover its parts (`overridesubcomponentsmass` is false)*. The file's stage carries a mass override
and a center of gravity override, each with its own such flag false. OpenRocket 24.12 adds such a mass to
what the parts weigh, measured on four probe designs ([the `.ork` page][ork-stage]). HPR Sim reads
a stage's override as the whole stage's, so since [#1225][i1225] it holds such a stage back
rather than fly it far too light. Flying it as OpenRocket weighs it is work on the design tree
with its own probes, and it changes how many of the survey's designs fly. It is not a step inside
a predictions file.

The three EuRoC 2022 flights have no such gap. Their designs build from the notebooks' inputs as
the seven flown ones do, and their weather needs only the conversion the guide already gives.

**Decision.**

1. **Three increments, in this order.** M2.3d1 is met when all three are, and M2.3d2 follows.
   - **M2.3d3, the EuRoC 2022 predictions.** *Done when:* a pull request of its own merges
     Astra's, Andromeda's and Erebus 11's inputs, sources, apogee rules and predicted apogees,
     flown in their [ERA5][era5] air from the [netCDF-4][netcdf] file converted by a committed,
     tested script with both files' SHA-256 committed, Andromeda counted on its team's curve with
     the catalog curve's prediction beside it, and no error computed.
   - **M2.3d4, a stage's override that adds to its parts.** *Done when:* an `.ork` stage whose
     mass or center override doesn't cover its parts is flown as OpenRocket 24.12 weighs it,
     its mass, center and inertia held to OpenRocket's on probe designs, and is no longer held
     back.
   - **M2.3d5, the Macie XL predictions.** *Done when:* M2.3d1's *done when*, whole: a pull
     request of its own merges both Macie XL flights' inputs, sources, apogee rules and
     predicted apogees, the design flown as saved from its pinned repository, no error computed,
     and with it all five predictions are on `main`.
2. **The guard holds across two pull requests.** ADR-328 merges predictions first so that no
   input can be tuned to its own error. That needs each flight's prediction on `main` before
   that flight's error is computed, which two pull requests give as well as one. M2.3d2 stays
   after M2.3d5, so none of the five is compared until all five are merged. A test fails if one
   of the three joins the compared flights while it is still only on the predictions list.
3. **The apogee rule, fixed before the flights were flown.** Each of the three is read by the
   seven's rule ([ADR-082][adr-082]): the highest reading of its log's height column. Four of
   the seven logs are cut before a transient; these three are read whole: each was looked at for its columns and for a transient near its
   peak, and each peak is the whole log's highest reading. No notebook says how its altimeter
   makes a height, so each is taken as barometric, as the four of the seven with no record are,
   and the prediction is read the same way. The height climbed is kept beside it. The measured
   apogees are already in the research notes; the rule guards the inputs, not that knowledge.
   That the rules came before the flights is the maintainer's statement: both reached the
   repository in one change, so no check can show it.
4. **Traced means the record states it.** A flight's record here is its example notebook. What
   no RocketPy example records is a placeholder in the designs built from them, the seven's too,
   and doesn't count against the tag: the surface finish, the rail buttons' size, the fins'
   thickness where no airfoil file gives it, and their edge shape where no airfoil file or
   published section does. They move the drag, so traced means stated, not exact. Astra and
   Erebus 11 are tagged traced.
   Andromeda is not: it flew air brakes whose opening no record gives, and its thrust file holds
   more impulse than the reload it is named for ([the research note][andromeda]).
5. **Andromeda's two curves.** The counted prediction flies the team's thrust file, as ADR-328
   says and as every RocketPy flight flies its example's own file. Its design carries the bundled
   public-domain Cesaroni 8429M2020-P, the motor the notebook names, in place of the bundled
   curve nearest the team file's impulse. The committed design then flies the catalog motor as it
   stands, at the counted flight's mass, and that flight is the one shown beside the counted
   prediction. It enters no mean. The other cases' stand-in curves are near their examples' in
   impulse, not always the nearest: the catalog has grown since the first were chosen, and the
   oracle script's comments give the impulses.
6. **The weather is converted by a script, not read in Rust.** ADR-328 allows either. A netCDF-4
   file is HDF5 inside, and a reader for it is a large dependency for one file. The guide's
   [conversion][era5-convert] is already held by a test on a converted extract. The
   [script][script] runs that conversion on the whole file and fails unless the four fields keep
   their values bit for bit (148,000 of them), the axes and times are the source's, and RocketPy
   builds the same 37 levels from both files at each flight's site and hour. Its
   [record][record] holds both files' SHA-256 and the levels RocketPy read. `public-predictions`
   flies the converted file only if its SHA-256 is the recorded one and HPR Sim reads those
   levels from it: the same pressures, and the temperature, wind and geopotential height within
   1e-12 of each value's size (1e-12 outright for a value under 1).
7. **What a prediction may read.** The module that flies a prediction hashes its flight's log,
   which pins the file the rule will be applied to, and never parses it. Its report has no field
   for a measured apogee or an error. A test fails if the committed file is not, byte for byte,
   what the report's types write, if a key in it names an error, a measured value or a
   difference, or if the module's source names one of the log readers' functions.
8. **The merged numbers are kept in the code.** Each flight's apogee, as a reading and as a
   height, and Andromeda's on the catalog curve, as first merged and to the digit the page
   prints, are a list in the module. The writer refuses a flown prediction that
   has moved from it, and CI fails a committed one that has. A prediction that has to change is
   revised beside its merged number by the change that compares it (ADR-328 §1b).

**The predictions.** From [the committed page][predictions]:

| Flight | Impulse flown (N·s) | Liftoff mass (kg) | Predicted apogee, as its altimeter would read it | As a height | Every input traced |
|---|---:|---:|---:|---:|---|
| Astra | 4,280.2 | 12.670 | 3,037.1 m (9,964 ft) | 3,155.4 m (10,352 ft) | yes |
| Andromeda, the team's curve | 9,866.7 | 25.323 | 3,581.0 m (11,749 ft) | 3,729.5 m (12,236 ft) | no |
| Erebus 11 | 2,418.2 | 6.996 | 2,714.1 m (8,904 ft) | 2,808.7 m (9,215 ft) | yes |

Andromeda on the catalog M2020, never counted: 8,428.7 N·s, 3,050.3 m (10,008 ft).

**What doesn't change.** M2.3d's and M2.3d1's *done when* stand but for one clause: the five
predictions merge in two pull requests of their own, not one (decision 2). They still merge
before any of their errors, and no flight is dropped. No model changes.
The seven flights' report reproduces; only the SHA-256 it records for the design fixture moved,
because the fixture gained three cases.

**Consequences.** The public predictions are on `main` for three of the five. The Macie XL
flights wait on M2.3d4, which also lets other `.ork` designs with such a stage fly. Until
M2.3d2, the real-flights report still rests on 7 flights. Anyone can check that these three
predictions were not changed afterward: the merged apogees are in the code, to the digit the page
prints, where a change to them shows in its history. CI fails a committed prediction that has
moved from them, and where RocketPy's files are,
`cargo xtask public-predictions --check` fails when a flown prediction differs from the
committed one.
