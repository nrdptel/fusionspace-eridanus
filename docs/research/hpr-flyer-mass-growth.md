# How much heavier a single flyer's fiberglass L2/L3 rocket flies than its file

**In short: a median +7.6%, close to the earlier set's +7.4%, with a longer heavy tail.** This note collects 12
public pairs in which one fiberglass high-power rocket, built by one flyer, has both a design-file mass and a scale
mass: 9 from Rocketry Forum build threads and a posted L3 package, 3 from kit reviews beside the maker's own RockSim
file. Nine of the 12 came out heavier. The median growth is +7.6%, the interquartile range +0.2% to +18.4%, the range
−1.9% to +72.2%. Pooled with [as-built-mass-growth](as-built-mass-growth.md)'s 24 pairs, the 36 give a median +7.4%
(IQR −0.4% to +13.6%): its median holds and its upper quartile moves up half a point. Four weaker pairs, set apart,
would put the 16's median at +7.6% or +11.4%, depending on how a RockSim file's part masses are read (section 3).
Weighed paint is 115 to 174 g/m², +1.6% to +2.4% on the earlier note's 6 kg rocket, at the low end of its +1.7% to
+4.2% bound. How sure: medium on each pair (forum figures, several "about", flagged below); low on the median as a
population figure, because posted builds pick themselves.

## How the pairs were read

- **File mass:** the poster's stated OpenRocket or RockSim mass, or, for kit reviews, the mass the maker's RockSim
  file flies with: its whole-rocket override, which all three set. **Scale mass:** the poster's reading of the
  finished rocket, whole or summed from weighed sections.
- Both sides are reduced to the dry rocket with no motor, as the earlier note does: a loaded motor comes off at the
  poster's own figure or ThrustCurve's total mass [S23]. Growth is scale ÷ file − 1.
- A pair is rejected when the file had already been overridden to the scale, when the file figure is a hand budget
  or a guess, when it is another builder's file of a different rocket, or when only a maker's printed weight is
  given (the earlier note's rule). About 55 candidates failed, most for want of a file mass or a scale figure.
- Pairs with a stated design change or a doubtful side are set apart. Quartiles are inclusive (Python
  `statistics.quantiles`). Post numbers are those on the thread pages.

## 1. Twelve fiberglass pairs

| rocket (public kit or scratch) | file (dry) | scale (dry) | growth | painted | file basis | source |
|---|---|---|---|---|---|---|
| Madcow Tembo, 5.5 in | OpenRocket 10.15 kg | 10.7 kg | +5.4% | no | kit weights | [S1] |
| Performer 150, 6 in | OpenRocket about 35 lb | 41 lb, sections | +17.1% | no | hardware missing | [S4] #6, #52 |
| Prometheus 6, G12 | RockSim pre-build | 33 g over it | +0.2% | unstated | pre-build | [S5] #46 |
| Wildman Ult. Punisher, 6 in | sim 12,572 g | 75 g under it | −0.6% | before clear | pre-assembly | [S6] #39 |
| scratch L3, 4 in | OpenRocket 4.9 kg | about 6 kg | +22.4% | before paint | out-of-date file | [S7] #9, #15 |
| Ultimate Wildman, 6 in | about 35 lb estimate | 37 lb | +5.7% | yes | unstated | [S8] #34, #49 |
| Madcow Level-2, 4 in | RockSim 148.89 oz | 146 oz | −1.9% | unstated | calculated | [S9] #112, #114 |
| Wildman Intimidator 4 | 14.4 lb | 20.1 lb | +39.7% | unstated | initial sim | [S10] #1, #10 |
| modified Ultimate Drago | 20.9 lb | about 36 lb | +72.2% | no (film) | catalog | [S11] #1, #67 |
| Madcow Frenzy XL, 4 in | maker's file 6,350 g | 7,202 g | +13.4% | unstated | override 14 lb | [S12] |
| Madcow Arcas, 2.6 in | maker's file 907 g | 992 g | +9.4% | no | override 32 oz | [S13] |
| Madcow Honest John, 2.6 in | maker's file 850 g | under 30 oz | 0.0% | unclear | override 30 oz | [S14] |

Readings that need judgment:

- **Inferred figures.** The Prometheus 6's dry mass (about 14–16 kg) is inferred from its 20,451 g loaded file, and
  its "scale" is the file updated with four weighed sections. The Ultimate Wildman's 35 lb is called an initial
  estimate, its source assumed to be the model in post 1. Whether the Intimidator 4's 31 lb loaded was weighed isn't
  stated; its two sides carry different motors (an M1350W in the file, an M1101WH flown). The Honest John came in "a
  touch under" 30 oz, read as 0.0%, an upper bound.
- **Kit files.** Frenzy XL's and Arcas's overrides equal the catalog weight; Honest John's (30.0 oz) is 2 oz over its
  28 oz catalog weight, and that reviewer used another maker's file. The scratch L3 (Wildman parts) builder says
  the posted file was out of date (thinner fins, one fillet set) [S7] #11.
- **Alternatives.** Against the modified Drago builder's later file of weighed parts (30.8 lb) it grew +16.9%, which
  leaves the median and moves the upper quartile to +16.9%. The Punisher flew at 12,791 g with a larger main (+1.7%).
- **Set apart (four pairs).** A Formula 200, 8 in: a hand budget of weighed parts and epoxy gives 50.8 lb against
  60.8 lb weighed, +19.7%; another builder's RockSim file gives +18.3% [S2]. An AGM-33 Pike, 8 in: "will be about
  75 pounds" against a "final dry weight" of 35,374 g, +4.0%, a forecast against what may be the updated file [S3]
  #6, #94. Two kit-library files whose growth depends on the RockSim reading (section 3): a Composite Warehouse 4.5 in
  Arcas, +35.9% or +79.1%, against the maker's own weighed 7,790 g after paint [S15]; a Wildman Extreme Wildman 4 in,
  −5.4% or +15.9%, against the reviewer's 25 lb on the pad less its motor [S16].
- **Design change.** A Wildman Jumbo Jart 8 in grew +40.3% (35.6 to 49.9 lb dry), but its airframe grew from 101 to
  115.9 in and its main by a size [S19] #1, #12, #14; the builder: "Mine always are."

## 2. What the pairs say together

| set | n | median | IQR | range |
|---|---|---|---|---|
| the twelve | 12 | +7.6% | +0.2% to +18.4% | −1.9% to +72.2% |
| forum threads and the L3 package | 9 | +5.7% | +0.2% to +22.4% | −1.9% to +72.2% |
| kit reviews with the maker's file | 3 | +9.4% | +4.7% to +11.4% | 0.0% to +13.4% |
| dry mass 10 kg or more | 5 | +5.4% | +0.2% to +5.7% | −0.6% to +17.1% |
| dry mass 2 to 10 kg (two under 1 kg left out) | 5 | +22.4% | +13.4% to +39.7% | −1.9% to +72.2% |
| with the earlier note's 24 pairs | 36 | +7.4% | −0.4% to +13.6% | −20.2% to +72.2% |
| twelve plus the four set apart, part sums | 16 | +7.6% | +0.2% to +20.4% | −5.4% to +72.2% |
| the same, known masses where set | 16 | +11.4% | +3.0% to +20.4% | −1.9% to +79.1% |

- **The sign is firm, the size is not.** 9 of 12 heavier, one level, two lighter by under 2%; the earlier set had
  16 of 24 heavier. The mean (+15.2%) is twice the median: the spread is skewed heavy, so a term centered on +7.4%
  with a symmetric spread understates the upper side.
- **Inference: what the file leaves out sets the tail.** The three pairs above +20% are an out-of-date file, an
  initial sim and a catalog file. Four files from kit weights, a calculation or a check before assembly sit at −1.9%
  to +5.4%; the Performer 150, from weighed parts with hardware missing, grew +17.1%. Only that builder and the
  Pike's name such causes (hardware, epoxy, recovery gear), so this is a reading, not a measurement.
- **Inference: growth may fall with size.** The 5 rockets of 10 kg or more grew a median +5.4%; the 5 of 2 to 10 kg
  a median +22.4%. With 5 a side and the file basis mixed in, this is a lead, not a finding.

## 3. Makers' files: overrides, part sums and skeletal lists

- **Two ways to read a RockSim part list.** A `.rkt` file stores, per part, a computed mass and a known mass, with
  no per-part override flag in the files read. Summing the computed masses (plus mass objects) is one reading;
  taking the known mass wherever it is set is the other. On one library file whose page states RockSim's
  calculated weight, 161.4 oz, the known-mass reading gives 161.4 oz exactly and the computed sum 164.4 oz, so the
  known-mass reading is likelier; one file doesn't settle it. In one maker's file the known masses of the main and
  drogue are those of the named parachutes, while the computed ones are 65% and 231% higher [S16].
- **Overrides hide the gap.** Madcow's files set a whole-rocket override to the catalog weight for the Frenzy XL and
  Arcas [S12], [S13]; builders confirm the override replaces the part sum in flight ([S2] p. 63; [S9] #114). Against
  the override the three builds grew 0.0% to +13.4%; against the files' own part lists the figure depends on the
  reading above (Arcas +20.4% or −22.6%).
- **Skeletal files.** Two library files model only the airframe: no chute, cord or hardware. A Performance Rocketry
  Gizmo file credited to the maker reads +112.5% or +69.7% against another builder's 2,892 g without motor [S17]; a
  user's Competitor 4 file +156.7% or +80.0% against 6,804 g ("I tend to overbuild") [S18]. These measure what the
  file leaves out, not how a build grows.

## 4. Paint and epoxy on high-power rockets

| what | finish | mass | per area | share of the rocket | source |
|---|---|---|---|---|---|
| a 10.5 in BT-55 tube, weighed | 3 primer, 3 color | 4.9 g | 174 g/m² | n/a | [S20] |
| a 185 in² coupon, weighed | 1 primer, 2 color | 0.074 g/in² | 115 g/m² | 3.4%, projected, 6.4 lb | [S21] #61, #69 |
| a 6 in fiberglass L3, estimate | paint | about 0.5 lb | about 110–150 g/m² | about 1.2% of 41 lb | [S22] #4 |
| a 6 in fiberglass L3, expectation | clear coat | about 80 g | n/a | about 0.6% of 12.6 kg | [S6] #39 |

- **The bound holds, at its low end.** At the two weighed rates, 115–174 g/m², the earlier note's 6 kg, 0.84 m²
  rocket gains 96–146 g: +1.6% to +2.4%, against its +1.7% to +4.2% (from an assumed 120–300 g/m²). Nothing here
  reaches 300 g/m². The L3 rows' area is assumed (body and fins), so their g/m² is an inference. No builder found
  weighed a fiberglass airframe before and after paint.
- **Falling share, by estimate only.** The two L3 rows' estimates, 0.6% and 1.2% on 12–19 kg rockets, against the
  coupon's 3.4% projected on a 2.9 kg L2, fit the earlier note's rule (paint grows with area, structure with wall
  mass); no weighing tests it.
- **Epoxy and hardware weigh more.** One 8 in L3 budgets 16.5 oz of epoxy and 4 oz of clear coat (581 g, 2.5% of its
  budget) [S2]; on the Pike, epoxy and hardware added 1,461 g to the booster alone [S3] #40; on a 4 in fiberglass L2,
  120–140 g of fillet epoxy [S24] #27.

## 5. Flights under the sim

Six of the 16 pairs report a flight below the sim. With the pre-build file: the Tembo flew 8,563 ft against 9,673 ft
(−11.5%) [S1], the modified Drago about 1,000 ft low [S11] #126, the Pike 2,310 ft against 2,370 ft (−2.5%) [S3]
#102; the Madcow Level-2 also flew low [S9] #229, #233. With the weighed mass already in the sim: the Formula 200
flew 7,890 ft against 8,396 ft (−6.0%) [S2] p. 77, 88, the Prometheus 6 6,686 ft against 7,150 ft (−6.5%) [S5] #54,
#57. Only the Tembo's builder names a cause (the 550 g and the bare finish). **Inference:** at the earlier note's
elasticities (−0.32% to −0.90% of apogee per 1% of dry mass), the Tembo's +5.4% explains 1.7 to 4.9 points of its
11.5, and the two weighed sims still read 6–7% high: as in that note's section 4, mass is one cause among several.

## What this suggests

1. **Keep +7.4% as the median of the as-built mass term, and give it a longer upper side.** The pooled 36 pairs give
   IQR −0.4% to +13.6%; these 12 fiberglass L2/L3 pairs alone +0.2% to +18.4%, with a mean twice the median. A term
   that samples growth (M7.5's mass term, the Monte Carlo default) should be skewed heavy, not symmetric.
2. **A file's completeness may predict its growth.** The pairs above +20% come from files with parts missing or
   catalog masses; complete files from kit weights sat within ±6%, though one from weighed parts grew +17%. A design
   check that counts a file's recovery and hardware parts (the "weighed against estimated mass" idea in
   [ideas](ideas-design-and-physics.md)) could test this on the collection.
3. **Count the collection's whole-rocket overrides**, as the earlier note's item 1 asks, and tell a maker's override
   set to the catalog weight apart from a weighed one.

## Open questions

- **How RockSim reads per-part masses.** One file's stated total favors the known-mass reading; it moves two pairs
  by 21 to 43 points and the 16-pair median from +7.6% to +11.4%. No RockSim documentation was found.
- **Size dependence.** The 10 kg split (+5.4% against +22.4%) rests on 5 pairs a side with the file basis mixed in.
- **Paint on fiberglass.** No before-and-after weighing of a fiberglass airframe was found; the L3 rows are estimates.
- **Forum attachments.** The threads' .ork and .rkt attachments need a login and weren't opened.
- **Selection.** Heavy builds get threads, and L3 packages are written by flyers who flew; neither is a random sample.

## Sources (read 2026-10-18)

Forum posts, reviews, L3 packages and makers' files carry no license: cited, cached under `refs/` only.

1. **S1** A flyer's NAR L3 package, Madcow Tembo (mass section, flight report).
   <https://docs.google.com/document/d/1yp7NMjZYsvLtPwiq1k4fLaJ5yl-_YKBymiJtlD1TECs>
2. **S2** A flyer's TRA L3 package, Formula 200 (Table 2, p. 26; "Assembled weight", p. 63).
   <https://drive.google.com/file/d/1KqshUDP_3Tv3A_ByFJSFlLqICa46pJiO>
3. **S3** <https://www.rocketryforum.com/threads/l3-project-8-agm-33-pike.167236/>
4. **S4** <https://www.rocketryforum.com/threads/performance-hobbies-performer-150-my-l3-cert-build.121049/>
5. **S5** <https://www.rocketryforum.com/threads/garydean-l3-build-thread-prometheus-6.182049/>
6. **S6** <https://www.rocketryforum.com/threads/revised-l3-attempt-build-6-n2000-airfest.187616/> #22, #70
7. **S7** <https://www.rocketryforum.com/threads/quentins-scratch-built-l3-build-thread.183640/>
8. **S8** <https://www.rocketryforum.com/threads/canadian-level-4-certification-build-ultimate-wildman.171927/>
9. **S9** <https://www.rocketryforum.com/threads/level-2-build-madcow-level-2-via-apogee-components.132913/>
10. **S10** <https://www.rocketryforum.com/threads/l3-project-wildman-intimidator-4.169218/>
11. **S11** <https://www.rocketryforum.com/threads/l3-its-a-southern-thing-a-level-3-build-thread.193049/> #31
12. **S12** Madcow 4 in fiberglass Frenzy XL page and review (2017-10-27), and its RockSim file.
    <https://www.madcowrocketry.com/4-fiberglass-frenzy-xl/>,
    <https://www.madcowrocketry.com/content/rkt/frenzyxl.rkt.zip>
13. **S13** Lam, Madcow Arcas (fiberglass) review, 2009-11-21, and Madcow's file.
    <https://www.rocketreviews.com/madcow-rocketry-arcas-fiberglass-by-peter-lam.html>,
    <https://www.madcowrocketry.com/content/rkt/arcas.rkt.zip>
14. **S14** Doll, Madcow Honest John (fiberglass) review, and Madcow's file.
    <https://www.rocketreviews.com/madcow-rocketry-honest-john-fiberglass-by-tim-doll.html>,
    <https://www.madcowrocketry.com/content/rkt/hojo3126.rkt.zip>
15. **S15** Composite Warehouse ARCAS design file and comment.
    <https://www.rocketreviews.com/rocksim-composite-warehouse-arcas.html>
16. **S16** Caplinger, Extreme Wildman review, 2011-06-02, and the file linked from the maker's page.
    <https://www.rocketreviews.com/wildman-rocketry-extreme-wildman-by-mike-caplinger.html>,
    <https://wildmanrocketry.com/products/wildman-extreme>
17. **S17** Gizmo file and review. <https://www.rocketreviews.com/performance-rocketry-gizmo-6166.html>,
    <https://www.rocketreviews.com/performance-rocketry-gizmo--by-howard-druckerman.html>
18. **S18** Competitor 4 file and review (2007-06-26).
    <https://www.rocketreviews.com/unknown-competitor-4performance.html>,
    <https://www.rocketreviews.com/performance-rocketry-competitor-4--by-kyle-cornelius.html>
19. **S19** <https://www.rocketryforum.com/threads/level-3-build-wildman-jumbo-jart-8.198394/>
20. **S20** DeAngelis, "Painted Body Tubes of the DEFCON 1" and "Building the DEFCON 1" (the tube size).
    <https://www.rocketreviews.com/painted-body-tubes-of-the-defcon-1.html>,
    <https://www.rocketreviews.com/building-the-defcon-1-180703114933.html>
21. **S21** <https://www.rocketryforum.com/threads/i-roc-l2-build.190360/page-3>
22. **S22** <https://www.rocketryforum.com/threads/how-much-weight-does-a-typical-paint-job-add.124765/>
23. **S23** ThrustCurve motor search API (total masses). <https://www.thrustcurve.org/api/v1/search.json>
24. **S24** <https://www.rocketryforum.com/threads/133697/>
