# Public flights with a design and a log, published since the October survey

HPR Sim checks its apogees against seven public real flights ([accuracy: real flights](../accuracy.md#real-flights)),
and M2.7 holds a third of its flights out. This note asks which COTS-motor flights with both a design file (`.ork`,
`.rkt`, `.CDX1`) and an altimeter log were published after the survey of 2026-10-11
([public flights](public-flights.md)), above all from the 2026 IREC (the event once called the Spaceport America Cup)
and EuRoC seasons. It counts them by source, license, format, logger and top Mach, and says which HPR Sim reads
today. It leaves out flights that [public flights](public-flights.md), [AerospaceNU's flights](aerospacenu-flights.md),
[UND's configurations](und-flight-configs.md), [Duke's database](duke-flight-database.md),
[IREC 2025's COTS reports](irec-2025-cots-reports.md) and [Student Launch masses](sl-weighed-mass-flights.md) already
list. All sources read 2026-11-05. Top Mach values are estimates, marked **Inference**.

**In short.** No source published a new COTS flight with both a design and a log after the survey. The search did
turn up 21 such flights, flown 2024-03 to 2026-09 and posted 2025-06 to 2026-09, that no earlier note pairs as a
design and a log; only 2 carry a license.
- **Since 2026-10-11: none.** RocketPy has no release after v1.13.0 and no new example data. ESRA posts 146 report
  PDFs and a scoresheet for IREC 2026, with no design file or log. EuRoC posts winners only, and NASA collects
  Student Launch logs without publishing them. The one new public flight (Zenodo, MIT, 2026-10-25) has a log and no
  design file.
- **Usable now: 2 flights, both subsonic.** The Macie XL flights in rlhatcher/rocketry-files (MIT) pair an
  OpenRocket 24.12 design with StratoLogger `.pf2` logs, which HPR Sim reads. They measured 843 m and 978 m against
  the file's own stored simulations of 704.6 m and 691.6 m, 20% and 41% higher. That gap, and an ejection spike in
  one log, need handling before the pair joins the scored set (§2).
- **Cite only: 19 flights.** 10 sit in GitHub repositories with no license, among them the one transonic flight
  found, PSAS Launch 14 at about Mach 1.1. The other 9 are a Rocketry Forum thread's flights up to about Mach 1.0,
  whose designs were adjusted after the flights.
- **IREC 2026: 81 COTS apogees, reports only.** That row was already counted. Three O5500X flights likely passed
  Mach 1 (**Inference**).

## 1. Where the search looked

| source | searched | new design + log flights | why not more |
|---|---|---|---|
| GitHub team repositories | 3,573 file trees; 79 read by hand | 12 (latest 2026-09-29) | no log or license |
| RocketPy and RocketPy-Team | v1.13.0 against `develop` | 0 | no example or data file changed |
| seven open-data portals | 88 queries, 2025 on | 0 (1 log only) | data.gov not searched |
| ESRA, IREC 2026 | scoresheet and the 2.45 GB report folder | 0 | 146 files, all PDF |
| EuRoC 2025, 2026 | site, winners list, rules | 0 | winners list only |
| NASA Student Launch 2026 | handbook, results news | 0 | logs go to NASA, none posted |
| Team sites, forums | 6 web searches, one forum thread read signed in | 9 | no license stated |

GitHub's 79 candidates broke down as follows. Kept: 7 repositories with 12 flights, plus 1 duplicate. Rejected:
7 for a team-built or non-solid motor, 3 for active control, 25 with no flight log, 2 with flights before 2025,
8 copies of OpenRocket's source and 1 GPL simulator (listed by tree, never opened), 6 of the project's own
repositories, and 19 last pushed before 2025-06-01. Code search
reads only default branches and can't see inside a zipped `.ork`, so some repositories can be missed. The
portals' queries were: Zenodo 25, figshare 18, Mendeley 11, Kaggle 9, Dataverse 12, OSF 8 and Hugging Face 5.

## 2. The two usable flights: Macie XL

[rlhatcher/rocketry-files](https://github.com/rlhatcher/rocketry-files) is MIT (GitHub's license API), and its
flight data were committed 2026-06-15. Both flights belong to its "MRC Summer 26" campaign and fly one OpenRocket
24.12 design, `macie/macie-xl.ork` (schema 1.10, within the range HPR Sim reads).

| flight | motor | measured apogee | stored simulation | ratio | top Mach |
|---|---|---|---|---|---|
| 1 | AeroTech I284W | 2,765 ft AGL (843 m), `.pf2` header | 704.59 m, Mach 0.353 | 1.20 | about 0.44 |
| 2 | CTI 601I350-16A | 3,208 ft AGL (978 m), `.pf2` header | 691.648 m, Mach 0.361 | 1.41 | about 0.48 |

- **Logs.** Both flights carry a StratoLogger CF `.pf2`, serial 5686, flights 5 and 6. Flight 1 also carries a
  Grissom CSV, whose filtered altitude peaks at 847.6 m, 0.6% above the `.pf2` header (checked for this note); its
  149.6 m/s peak speed is 7% above the `.pf2`'s 458 ft/s. Flight 2 also carries an EasyTimer V2 CSV in AltOS's
  layout; its `height` and `speed` columns are zero throughout, as for a board with no barometer. Flight 2's apogee
  therefore rests on one logger. That EasyTimer file is already logged on issue #662 as a candidate sample.
- **An ejection spike.** Flight 1's `.pf2` has one sample of 3,454 ft at 12.80 s, between 2,742 and 2,744 ft,
  with the apogee charge set to 0 s delay. A reader that takes the highest sample would give 1,053 m, not the
  header's 843 m (checked for this note).
- **Top Mach** is the repository's stated peak speed, about 150 and 164 m/s, over 343 m/s (**Inference**).
- **The gap.** The repository's `index.md` says both flights beat their simulation (about 2,310 ft and 2,270 ft).
  The stored runs' peak speeds, 120.0 and 122.6 m/s, sit 20% to 25% under the logs'. On flight 2 the 978 m tops
  every one of the file's nine stored runs, including a J350W at 825.8 m. **Inference:** the stored runs predate
  the as-flown mass, or flight 2 flew more motor than its label. The repository doesn't say which.
- **The stage's override (found November 10, 2026).** The design's stage carries a mass override whose flag
  says it doesn't cover the stage's parts. OpenRocket 24.12 adds such a mass to what the parts weigh
  ([the `.ork` page](../format/ork.md#the-values-inside-the-tags)). HPR Sim holds such a stage back, so `hpr sim`
  refuses the file and neither flight can be predicted until it flies one
  ([ADR-334](../decisions/0334-m2-3d1-in-three-the-euroc-2022-predictions-first.md)). **Inference:** if the
  builder meant the override as the whole rocket's mass, the stored runs fly a heavier rocket than the one
  that flew, which would read low, the side the gap is on. The repository doesn't say which was meant.
- **Predicted (November 10, 2026).** HPR Sim now reads such a stage as OpenRocket weighs it, and both flights'
  predictions are merged, each flown as its stored simulation from the file as saved
  ([ADR-337](../decisions/0337-the-macie-xl-predictions-each-flight-s-stored-simulation-flown-as-saved.md),
  [the predictions](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/public-predictions.md)).
  No error is computed yet.
- **Reads today.** `.pf2` has a reader ([flight logs](../format/flight-logs.md)); the page lists EasyTimer V2 logs
  among formats with no tested sample. The Grissom CSV states units in its header, so the unit-header table reader
  likely takes it. None of these was run for this note (**Inference**).

## 3. Cite-only flights no earlier note lists

GitHub repositories with no license (all `https://github.com/<repository>`):

| repository | flights | motor | logger, format | apogee | top Mach |
|---|---|---|---|---|---|
| psas/Launch-14 | 1, 2026-05-16 | N2540 (**Inference**) | `.pf2`; TeleMetrum `.eeprom` v7 | 15,856 ft | ~1.1 |
| RIT-Launch-Initiative/IREC-2026-Systems | 1 (OTIS, 2025) | M6000ST | 2 RRC3 CSVs | 8,587 ft | 0.7 to 0.84 |
| saividyud/NASA_25-26_Rocket_Simulations | 4, 2025-26 | I500T, L1940X, L1256WS | AltOS | 493 to 1,426 m | 0.3 to 0.53 |
| tonuonu/MyLevel1Peregrine | 2, 2026 | H128W, J350W | CATS Vega `.cfl` | 141.6, 986.4 m | 0.14, 0.50 |
| gabriel84-py/fus-e_code | 1, September 2026 | not stated | team board CSV | 141 m | about 0.13 |
| LukeSchreiber/High-Power-Rocket-and-Flight-Computer | 1 | Estes E16 | team board CSV | 119.1 m | about 0.29 |

- **Launch 14** is the only transonic flight found. Its `.pf2` header gives 15,856 ft AGL. The design file
  (schema 1.6, written by OpenRocket 15.03, last changed 2021-06-25) stores a CTI N2540 run of 5,048 m at Mach
  1.127, and the measured 4,833 m lies nearest that run. The motor and the Mach rest on that 2021 run
  (**Inference**): the log's speed column is corrupt through the transonic part, and the 2026 mass may differ.
  The project's evidence register already cites PSAS's launch repositories as log samples
  ([evidence](../evidence.md)), but not as a design-and-log pair.
- **OTIS's** RRC3 files first appeared on 2025-06-13 in `irec-2025-analysis`; they are counted once. Its design
  also comes as a RASAero `.CDX1`, one of two found (proj-g/rocket's, with pre-2025 flights, is the other).
- **The Student Launch files** don't say which of the four full-scale `.ork` files (ballast 0.58 to 1.44 lb) flew
  which flight. The full-scale motors are inferred from burnout times in the team's OpenRocket exports
  (**Inference**). Several research notes already use this repository's logs (for example
  [boost accelerometer scale](accel-scale-at-boost-logs.md)), but not as a design-and-log pair.
- **Peregrine's** repository holds a description of the CATS `.cfl` layout whose header names CATS's GPL-3.0
  firmware source as its source of truth, so the clean room bars it from issue #861.

[Rocketry Forum thread 194371](https://www.rocketryforum.com/threads/openrocket-sims-vs-real-flights.194371/),
posted 2025-10, holds 9 AeroTech flights (E30T to I284W, 2024-03-02 to 2025-10-11). They fly two OpenRocket 24.12
designs, each flight with a Blue Raven low-rate and high-rate CSV pair and a Featherweight summary CSV. Apogees run
from 769 to 7,199 ft. Top Mach runs from 0.20 to 0.99, from each summary's peak speed and its board temperature
(**Inference**). A sunlit board reads warm, so these values err low. All three formats have readers (not run). The
member calls the attached designs "post-flite, adjusted", and the zips hold "masses-fixed" exports: tuned to the
flights, so not independent predictions. The posts state no license, and the forum's terms (§2.1) grant a
personal-use license only.

**Log only, MIT:** [Zenodo 23645282](https://zenodo.org/records/23645282) (2026-10-25), an Estes Lil' Spite on
an AeroTech G74W-4. Its 100 Hz custom-board log has 9,572 rows, with an apogee near 505 m from pressure and Mach
near 0.41 from the accelerometer (both **Inference**; one axis reads 31.94 g, at the ±32 g clip, so the speed may
read low). It has no design file and no weather.

## 4. IREC 2026: apogees only

- **The scoresheet** ([ESRA, past events](https://www.esrarocket.org/past-irec-events)) has 141 ranked rows. Of
  the 101 COTS rows, 81 scored an apogee: 67 in 10k (2,320 to 13,571 ft) and 14 in 30k (17,751 to 31,581 ft). The
  other 20 scored 0: 10 "Major" damage, 7 "NoFly", 3 "RocketLost".
- **The report folder** is one zip of 2,445,666,982 bytes holding 146 PDFs dated 2026-07-27, with no design file,
  log or spreadsheet. Each of the 81 has a report. [Public flights](public-flights.md) already counts this row.
- **Terms.** The 2026 rules publish every report, adding that "DO NOT SHARE is not an option" (§2.6.2.1,
  p. 16). Two reports carry CC BY-NC-ND 4.0; none of the others carries a license, and at least four carry
  proprietary notices, mostly in drawing title blocks.
- **Loggers named** in the 78 to 80 COTS reports matched by file name (text matches, not proof of what flew; two
  passes differed by one or two): AltOS family 57 to 58, Blue Raven 39, RRC3 25, StratoLogger 10 to 11.
  **Inference:** most logs would be in formats HPR Sim reads.
- **Top Mach.** At least 8 scored 30k COTS teams name the AeroTech O5500X; three of them scored 29,518 to
  30,683 ft. The 2025 note bounded that motor past Mach 1 from 27,541 ft up
  ([IREC 2025 §3](irec-2025-cots-reports.md)), so these three likely passed Mach 1 (**Inference**). Teams on other
  motors (an O3400 at 29,921 ft, an N4000 at 31,581 ft) can't be bounded from these data.

## 5. Counts

| set | flights | license | design | loggers | top Mach | reads today |
|---|---|---|---|---|---|---|
| Macie XL | 2 | MIT | `.ork` | StratoLogger, Grissom, EasyTimer | 0.44 to 0.48 | `.pf2` (not run) |
| GitHub, no license | 10 | none | `.ork`, 1 `.CDX1` | `.pf2`, AltOS, RRC3, CATS, own | 0.13 to 1.1 | 6 logs (not run) |
| forum thread | 9 | none | `.ork` | Blue Raven, Featherweight summary | 0.20 to 0.99 | logs (not run) |
| Zenodo 23645282 | 1 | MIT | none | custom board | about 0.41 | log likely |
| IREC 2026 | 81 (counted before) | 2 CC BY-NC-ND | none | none posted | 3 likely above 1 | apogee only |

By Mach, the 21 design-and-log flights split into 5 below 0.4, 12 from 0.4 to 0.8 (OTIS at its lower 0.7), and 4
from 0.8 to about 1.1; all estimates.

## What this suggests

- **Fly the two Macie XL flights** as public real flights once the 20% and 41% gaps against the file's own runs are
  traced and flight 1's ejection spike is handled. That takes the set from 7 to 9
  (12 with RocketPy's three that wait on weather conversion), with two more subsonic cases for M2.7's held-out
  half. A flight 41% over its own design's simulation may be a mass or motor record error, not a test of drag.
- **The transonic gap stays.** The four design-and-log flights at Mach 0.8 or more (Launch 14 and three forum
  flights) all lack a license, and the forum's designs were tuned to their flights. M2.6's public transonic cases
  still rest on RocketPy's set.
- **Competition bodies won't fill the set.** ESRA, EuRoC and NASA publish no designs or logs. Team repositories are
  the only steady source, and most carry no license.
- **Reader gaps seen in public files.** CATS `.cfl` (2 flights here; issue #861) and AltOS `.eeprom` format 25
  (a 2026-03-21 TeleMega file from an unlicensed SRAD repository), which HPR Sim refuses today.

## Open questions

- What explains Macie XL's 41% on flight 2: the as-flown mass, the motor, or the logger? The repository gives no
  flight mass.
- Which motor flew PSAS Launch 14, and at what mass? Only the 2021 design's runs point to the N2540.
- Would PSAS or the forum member add a license? The project does no outreach, so this stays open.
- What did the AltimeterTwo read on the Zenodo Lil' Spite flight? Its paper has no DOI yet.
- data.gov's catalog was not searched.

## Sources

All read 2026-11-05.
- GitHub repositories named in §2 and §3, read through `gh api` (trees, license, commits). rlhatcher/rocketry-files:
  `macie/index.md`, `macie/macie-xl.ork`, the two `.pf2` files, the Grissom and EasyTimer CSVs.
- RocketPy: <https://github.com/RocketPy-Team/RocketPy/releases>,
  <https://github.com/RocketPy-Team/RocketPy/compare/v1.13.0...develop>.
- Zenodo: <https://zenodo.org/records/23645282>; <https://zenodo.org/records/23886073> (`.ork` rebuilds of 10
  RocketPy flights by its own exporter, CC BY 4.0; no new flight).
- ESRA: <https://www.esrarocket.org/past-irec-events>; <https://www.esrarocket.org/s/final_scores_ext_7-13.pdf>;
  IREC Rules and Requirements 2026 v1.0,
  <https://www.esrarocket.org/s/IREC-Rules-and-Requirements-Document-2026-v10-f8wf.pdf>, pp. 13, 16.
- EuRoC: <https://euroc.pt/>; Competition Rules v1.1, §4.4, p. 13.
- NASA, 2026 Student Launch Handbook,
  <https://www.nasa.gov/wp-content/uploads/2025/08/g-677852-2026-sli-handbook-508-aug-7-optimized.pdf>, req. 2.6.8.3.
- Rocketry Forum thread 194371 and its terms, <https://www.rocketryforum.com/help/terms/> §2.1.
- HPR Sim's readers: [reading a flight log](../reading-a-flight-log.md), [flight logs](../format/flight-logs.md).
