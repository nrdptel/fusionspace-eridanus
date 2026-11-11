# ADR-337: The Macie XL predictions, each flight's stored simulation flown as saved (2026-11-10)

- **Status:** accepted; carries out [M2.3d5][m2-3d5] under [ADR-328][adr-328] §1a and [ADR-334][adr-334]; widens [ADR-161][adr-161]'s table of curve files by digest to the motors of public flights predicted from a design file. [M2.3d1][m2-3d1] is met once [M2.3d4][m2-3d4] is too, and [M2.3d2][m2-3d2], the errors, stays last.
- **Summary:** The last two of the five new public flights are predicted: both flights of Macie XL, one OpenRocket design in a public repository. The repository gives no weather or rail, and of the launch site only each log's pad height, so each flight is flown as the stored simulation its repository names for it, from the design file as OpenRocket 24.12 saved it. HPR Sim predicts 706.2 m (2,317 ft) for flight 1 and 692.6 m (2,272 ft) for flight 2, as their altimeters would read them. Both are tagged *not traced*. No error is computed. Flight 1's motor now flies the bundled curve file the design's digest names, which is not the catalog's first file.

[m2-3d1]: ../decisions-and-roadmap.md#m2-3d1
[m2-3d2]: ../decisions-and-roadmap.md#m2-3d2
[m2-3d4]: ../decisions-and-roadmap.md#m2-3d4
[m2-3d5]: ../decisions-and-roadmap.md#m2-3d5
[adr-161]: 0161-openrocket-curves-by-digest.md
[adr-288]: 0288-every-solid-motor-ships-in-the-package.md
[adr-328]: 0328-the-2026-11-08-review-twelve-public-flights-predictions-first.md
[adr-334]: 0334-m2-3d1-in-three-the-euroc-2022-predictions-first.md
[adr-335]: 0335-a-stage-s-added-mass-as-openrocket-weighs-it.md
[i1412]: https://github.com/nrdptel/fusionspace-eridanus/issues/1412
[i1413]: https://github.com/nrdptel/fusionspace-eridanus/issues/1413
[repo]: https://github.com/rlhatcher/rocketry-files/tree/2ddcaf8858a71f9b4d910877c603ed95fe0a9769/macie
[predictions]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/public-predictions.md
[table]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/crates/hpr-motor/data/openrocket-curves.json
[lock]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/refs.lock.toml
[ork-page]: ../format/ork.md
[pf2]: ../format/pf2.md
[apogee]: ../glossary.md#apogee

**Context.** [ADR-328][adr-328] §1a asks for five new public flights' inputs, sources and predicted
[apogees][apogee] on `main` before any of their errors is computed. [ADR-334][adr-334] merged the
three EuRoC 2022 flights' and left the two Macie XL flights waiting on the design file's stage,
whose mass override adds to its parts. HPR Sim reads such a stage as OpenRocket weighs it since
[ADR-335][adr-335], so the file now flies.

The Macie XL flights differ from the other ten public flights in what their record is. There is no
example notebook. The record is [the repository][repo]: one OpenRocket 24.12 design file,
`macie/macie-xl.ork`, a page about the rocket, and each flight's logs. The page lists the two
flights and names each flight's files `maciexl-<flight>-<source>`, with `or` for the OpenRocket
simulation. The design file holds nine stored simulations, and two of them carry those names:
`maciexl-1-or` on an AeroTech I284W and `maciexl-2-or` on a Cesaroni 601I350-16A. Nothing in the
repository gives the date, the weather, the rail or the mass the rocket was flown at. Of the
launch site it gives one thing: each log's header holds its pad's height above sea level, 11.3 m
(37 ft) for flight 1 and 3.7 m (12 ft) for flight 2.

**Decision.**

1. **The repository is pinned, and nothing of it is committed.** [The lock][lock] pins
   `rlhatcher/rocketry-files` (MIT) at commit `2ddcaf88`, of 27 June 2026, as a checkout under
   `refs/public-flights/`. The design file and the two logs are read from there, and the
   predictions keep each one's SHA-256. The checkout stays out of the default corpus of `cargo
   xtask ork`: the repository holds 17 designs, and taking them in would move every count [the
   `.ork` page][ork-page] quotes. That is a change of its own ([#1413][i1413]).
2. **Each flight is flown as its stored simulation.** The design file is read whole, with no
   edit. The flight's stored simulation gives the motor configuration and the launch conditions:
   a site at 28.61° N, 80.6° W at sea level, a vertical 1 m rod, a 2 m/s wind from the east and
   the standard atmosphere. All nine simulations in the file carry the same conditions. A stored
   simulation is refused if it lacks one of these, or was run in another atmosphere or wind
   model; tests on an invented design try each refusal here and in §3. The wind is flown
   steady: the file's turbulence, a tenth of the wind's speed, is left out. The parachutes are
   not flown; the file opens the first at apogee, so neither moves it.
3. **Each motor flies the curve the design's digest names.** A `.ork` motor records OpenRocket's
   digest of its curve. [The table][table] that maps a digest to the bundled file holding that
   curve ([ADR-161][adr-161], [ADR-288][adr-288] §3) covered only the motors of OpenRocket's own
   examples, so both Macie XL motors took the catalog's first file. For the 601I350-16A that is
   the digest's file. For the I284W it is not: the first file's thrust is as far as 0.372989 of
   OpenRocket's peak thrust from the digest's curve, and another bundled file matches it with no
   difference. `cargo xtask ork-curves` now also matches the motor each predicted flight flies,
   and the table holds both digests. Its candidates for these two are the bundled catalog's
   files, the ones `hpr sim` chooses from, so nothing is fetched for them. A prediction is
   refused unless the file flown is the one the table gives for the design's digest. Any other
   `.ork` that names the I284W by this digest now flies the matching file too. A digest the
   table lacks still takes the first file ([#1412][i1412]).
4. **The apogee rule is ADR-328's.** Each log is a PerfectFlite `.pf2`, whose header holds the
   apogee the altimeter worked out ([the `.pf2` page][pf2]). That reading is the rule for both
   flights, never the highest sample: flight 1's samples hold an ejection spike. A PerfectFlite
   has a barometer and no accelerometer, so the prediction is read as a barometric altimeter
   would read it. In the standard atmosphere that is within 0.1 m of the height climbed.
5. **Both flights are tagged not traced.** A stored simulation's launch conditions are what a
   simulation was run with, not a record of the day, so the site, the air, the wind and the rod
   are not traced. The logs' pad heights are not flown: a prediction reads no log, and the
   stored simulation's sea level is flown as saved. No record says the rocket was weighed as
   flown. And HPR Sim's reader warns on
   both curve files: the I284W file's impulse is 1.28% from ThrustCurve.org's figure for the
   motor, and the 601I350-16A file's loaded mass is 616.3 g against ThrustCurve.org's record of
   782.0 g. The file the design names is flown all the same; the warnings are kept with each
   prediction.
6. **What was known when the choices were made.** The apogee rule was merged with ADR-328, before
   HPR Sim could fly this file. The tags, and the choice to fly each stored simulation's own
   conditions, were written after `hpr sim` had flown the file at that command's defaults, and
   with both measured apogees already in the research notes. The repository's page says both
   flights flew higher than its simulations. Neither choice was made on a number: the file is
   flown as saved, and nothing in it was changed. This order is the maintainer's statement; no
   check can show it.
7. **OpenRocket's stored apogees are shown and never counted.** The design file keeps the apogee
   OpenRocket's own run of each simulation reached, 704.6 m and 691.6 m. Each prediction carries
   that number as another simulator's, not a measurement. No difference from it is computed.
8. **The report gains a list; the merged rows don't move.** The two flights are a second list
   in the predictions' data, with the fields a design file needs. The three EuRoC 2022 rows are
   written byte for byte as they were merged. The code's list of merged apogees gains two lines,
   and the writer and CI hold all five to it.

**The predictions.** From [the committed page][predictions]:

| Flight | Impulse flown (N·s) | Liftoff mass (kg) | Predicted apogee, as its altimeter would read it | As a height | Every input traced |
|---|---:|---:|---:|---:|---|
| Macie XL, flight 1 (AeroTech I284W) | 599.5 | 4.278 | 706.2 m (2,317 ft) | 706.3 m (2,317 ft) | no |
| Macie XL, flight 2 (Cesaroni 601I350-16A) | 600.8 | 4.338 | 692.6 m (2,272 ft) | 692.7 m (2,273 ft) | no |

**What doesn't change.** No model changes. The three EuRoC 2022 predictions reproduce. The
`.ork` survey's corpus and counts are as they were. The table's 28 rows for OpenRocket's examples
are as they were.

**Consequences.** All five predictions are on `main`, each merged before its error. The
real-flights report still rests on 7 flights until [M2.3d2][m2-3d2] compares all twelve. M2.3d1
stays open until what is left of M2.3d4 is done. Anyone with the pinned checkout can fly the two
again: `cargo xtask public-predictions --check` fails when a flown prediction, a file's SHA-256
or a stored condition differs from the committed one.
