# How much heavier finished high-power rockets fly than their design files

**In short: a median +7.4% of dry mass, from −20% to +40%, and a weighed mass doesn't remove the high bias.** This
note collects 24 public pairs in which one rocket has both a design-file mass and a scale mass. Seventeen come from
NASA Student Launch reports: the Critical Design Review (CDR) estimate against the Flight Readiness Review (FRR)
as-built mass. Seven come from builders' reviews, forum threads and one Spaceport America Cup report. Sixteen of
the 24 rockets came out heavier than their file. The median growth is +7.4%, the interquartile range −1.4% to +13.1%,
the range −20.2% to +39.6%. Through [apogee-bias](apogee-bias.md)'s elasticities (−0.32% to −0.90% of apogee per 1%
of dry mass), the median makes a simulation built from the file read 2.4% to 6.7% high: 27% to 76% of the private
collection's +8.73% mean bias ([accuracy](../accuracy.md#real-flights)), but only if its files carry no weighed
override, which nobody has counted yet. Five of six Student Launch simulations that already used the weighed mass
still read 3.6% to 18.4% high, so mass is one cause among several. How sure: high on each pair's
numbers (re-read against the reports); low on the median as a population figure, because one team supplies 9 of the 17
Student Launch pairs and the forum pairs pick themselves.

## How the pairs were read

- **File mass:** the CDR report's stated estimate (OpenRocket, RockSim or CAD sums of parts), or the poster's file
  mass. **Scale mass:** the FRR's stated as-built or measured mass, or the poster's scale reading.
- Both sides are reduced to the rocket with no motor and no ballast. A loaded motor is the team's figure, otherwise
  ThrustCurve's total mass [S12]. Student Launch teams add ballast on purpose to hit their target apogee; it is capped
  at 10% of the unballasted pad weight [S11]. Ballast is a deliberate change, not growth, so it is taken out of both
  sides. Growth is scale ÷ file − 1.
- A pair is like-for-like only if no report states a material swap, a part swap or a length change of over 2 in.
  Three more pairs with a stated change are listed apart. Ten more team-seasons were dropped: inconsistent totals
  (3), no total on a common basis (3), dead links or no FRR posted (4); six more teams' sites gave no reports.
- Quartiles are inclusive (Python `statistics.quantiles`). Page numbers are PDF page indices, not printed numbers.

## 1. Student Launch: CDR estimate against FRR as-built (17 pairs)

| team, season | CDR file | FRR as-built | growth | airframe | source, pages |
|---|---|---|---|---|---|
| NC State 2015 | 18.0 lb loaded | 19.9 lb, unpainted | +13.0% | Blue Tube | [S1] CDR p. 6, 19; FRR p. 8 |
| NC State 2016 | 29.3 lb loaded | 33.7 lb loaded | +20.8% | fiberglass | [S1] CDR p. 10; FRR p. 9, 31 |
| NC State 2017 | 49.6 lb loaded | 52.6 lb loaded | +7.7% | fiberglass | [S1] CDR p. 12; FRR p. 12–13 |
| NC State 2020 | 43.5 lb with motor | 47.5 lb liftoff | +11.3% | fiberglass | [S1] CDR p. 11; FRR flysheet |
| NC State 2022 | 37.41 lb, 3.0 ballast | 45.48 lb, 0.7 ballast | +39.6% | fiberglass | [S1] CDR p. 30–31; FRR 48–49 |
| NC State 2023 | 35.87 lb dry | 34.8 lb dry, 3.7 ballast | −3.4% | fiberglass | [S1] CDR p. 12; FRR p. 14, 18 |
| NC State 2024 | 37.89 lb dry | 51.60 lb total | +11.3% | fiberglass | [S1] CDR p. 12; FRR p. 33–34 |
| NC State 2025 | 30.41 lb dry | 30.8 lb dry | +1.3% | fiberglass | [S1] CDR p. 17; FRR p. 18 |
| NC State 2026 | 31.15 lb dry | 30.75 lb dry | −1.3% | fiberglass | [S1] CDR p. 12; FRR p. 12 |
| Georgia Tech 2015 | 16.88 lb, weighed parts | 16.9 lb liftoff | +10.3% | fiberglass | [S2] CDR p. 34–35; FRR sheet |
| Georgia Tech 2018 | 37.38 lb liftoff | 39.734 lb with motor | +8.2% | fiberglass | [S2] CDR sheet; FRR p. 9 |
| Purdue 2020 | 53.4 lb liftoff | 38.0 lb weighed | −20.2% | fiberglass | [S3] CDR p. 16; FRR p. 35–36 |
| Notre Dame 2021 | 771.2 oz with motor | 793.7 oz, 36.7 ballast | −2.3% | Kevlar, glass | [S4] CDR p. 13; FRR p. 11 |
| Notre Dame 2022 | 805.1 oz | 823.0 oz, 36.4 ballast | −2.9% | carbon | [S4] CDR p. 26; FRR p. 26 |
| Toledo 2024 | 17.92 lb dry | 20.48 lb dry | +14.3% | cardboard | [S5] CDR p. 10; FRR p. 13 |
| Toledo 2025 | 666.25 oz dry | 664.34 oz dry | −0.3% | fiberglass | [S5] CDR p. 13; FRR p. 13 |
| Toledo 2026 | 35.16 lb dry | 32.6 lb dry | −7.3% | fiberglass | [S5] CDR p. 13; FRR p. 13 |

Growth is of the dry rocket after the motor and ballast come off both sides; the masses shown are the reports' own
totals. Georgia Tech 2015's CDR total holds a 605 g margin, taken out here; against the total it grew +0.1%. NC State
2023's sides both hold the empty motor case, also taken out (raw −3.0%). Readings that need judgment: NC State 2016's
FRR flysheet says 32.1 lb (+13.2%); NC State 2020's text says "nearly 3 pounds" (+8.5%); NC State 2024's own table
gives 48.38 against 51.61 lb loaded (+6.7%); Purdue compares 48 with 55 lb (−15.6%). With those readings the median is
+6.7%. NC State 2026's FRR table sums to 39.05 lb against a stated 39.80 lb. Weighing is stated for 7 of the 17
as-built masses; 5 more come from itemized as-built tables, and 5 give a total without saying how it was measured.

**Summary:** median **+7.7%**, IQR −2.3% to +11.3%, range −20.2% to +39.6%, mean +5.9%; 10 heavier, 7 lighter. The
three pairs set aside for a design change (NC State 2021 +10.4%, 4 in. longer; Notre Dame 2023 −3.2%, a lighter nose
from an earlier year; Notre Dame 2018 −8.5%, carbon replaced by phenolic) bring n = 20 to a median of +4.5%.

**One team weighs heavily.** NC State is 9 of the 17. The five teams' own medians are +11.3% (NC State), +9.2%
(Georgia Tech), −0.3% (Toledo), −2.6% (Notre Dame) and −20.2% (Purdue, one pair); their median is −0.3%. Seasons
2015–2020 grew a median +10.2% (n = 7); seasons 2021–2026 a median −0.8% (n = 10). **Inference:** later teams budget
more carefully (Notre Dame applies the aerospace growth-allowance standard, below), so the pooled median says more
about one team's habits than about student rockets in general.

**With ballast left in,** five of the six pairs where both sides state it grew +2.8% to +5.8%; NC State 2022 +27.7%.

## 2. Builders' files against the scale (7 pairs)

| rocket (source) | file | scale | growth | airframe | file built from |
|---|---|---|---|---|---|
| Madcow DX3 [S6] | RockSim about 17 oz | 18.2 oz | +7.1% | cardboard | unstated |
| Apogee Saturn 1B [S7] | maker's RockSim 13 oz | 17.1 oz | +20.7% | cardboard, styrene | the kit's file |
| LOC Minnie-Magg [S8] post 6–8 | LOC's RockSim 68.1 oz | 76.1 oz | +1.5% | cardboard | the kit's file |
| AGM-88 kit [S9] post 1 | OpenRocket 9.6 oz | 13.4 oz | +39.6% | cardboard | unstated |
| scratch low-power [S10] post 1 | RockSim 4.26 oz | 4.55 oz | +6.8% | unstated | weighed parts |
| Estes ESAM [S14] post 1 | OpenRocket about 3 oz | 3.4 oz | +13.3% | cardboard | unstated |
| 2018 IREC hybrid [S15] p. 21 | OpenRocket 176 lb | 173 lb | −1.7% | carbon | weighed parts |

Two rows are corrected for nose weight the builder added beyond the plan: the Saturn 1B for 40 g of clay over the 20 g
the plan calls for (+15% to +21% by how much is removed), the Minnie-Magg for about 7 oz the builder names (raw
+11.7%). A Quest Starhawk at 43 g with its motor against a 40 g OpenRocket file [S13] post 15 is set aside: the file's
motor isn't stated (+13.9% of dry mass if it held the motor too, −38% if not). **Summary:** median +7.1%, IQR +4.2% to
+17.0%, range −1.7% to +39.6%; the five files from catalog or unstated parts ran a median +13.3%, the two from weighed
parts −1.7% and +6.8%. Nearly all are cardboard rockets under 3 kg; most threads began because a rocket came out
heavy.

**Kit makers' published weights are not a file.** Seventeen scale weights against a maker's printed weight run
−25.6% to +73.9% (median +19.2%), as the figure may be padded ([S16] post 5), bare parts or a shipping weight.

## 3. Where the growth comes from, with sign

- **Heavier (sim high).** About 145 g of adhesive, mostly fin fillets, on a mid-power kit: 14.4% of its parts
  mass [S17] post 1. Hardware a file leaves out: eye bolts, U-bolts, quick links, t-nuts, GPS (Georgia
  Tech 2018 [S2] FRR p. 8, 43; Toledo 2024 [S5] FRR p. 30–31). A material density below the product's: RockSim's
  G12 fiberglass in NC State 2022 [S1] FRR p. 47. Paint: 18.4 g, 32% of an unpainted low-power rocket [S18].
- **Lighter (sim low).** Parts lighter than their online listing (Notre Dame 2021 [S4] FRR p. 10), payloads lighter
  than budgeted (NC State 2023, Purdue 2020), and catalog defaults that run heavy: one RockSim paper-tube default is
  about three times the vendor's weight for the same tube [S19] post 4. **Inference:** catalog masses set the
  scatter's width, either sign; glue, paint and hardware set its offset, always heavier.

**A first-principles bound for paint.** A gloss spray paint's data sheet gives 25–50 µm of dry film per coat [S20]
p. 2; it gives no film density. **Inference:** at an assumed 1.2–1.5 g/cm³, four coats weigh 120–300 g/m². On a
102 mm × 1.5 m cardboard rocket of 2.5 kg (0.68 m², body 0.48 m² plus assumed fins) that is +3.3% to +8.2% of dry
mass; on a 98 mm × 2.0 m fiberglass rocket of 6 kg (0.84 m², body 0.62 m²), +1.7% to +4.2%. Paint grows with area and
structure with wall mass, so its share falls as a rocket grows. Epoxy has no such bound: 6 mm outside fillets on three
25 cm roots weigh about 13 g (cured density 1.11 [S21] p. 1), 0.2% of 6 kg, but one builder measured 0.3 oz per
fillet pair on a 6 in root, a 1 cm equivalent radius [S26] post 18, so technique sets it.

## 4. A weighed mass doesn't close the gap

| flight | simulation with the as-built mass | flown | sim error | source |
|---|---|---|---|---|
| NC State 2020 | 3,775 ft, RockSim, mass and CG overridden | 3,187 ft | +18.4% | [S1] FRR p. 37, 84 |
| NC State 2021 | weights overridden | 87.3% of sim | +14.5% | [S1] FRR p. 104–105 |
| NC State 2022 | 3,676 ft, weights from the test flight | 3,294 ft | +11.6% | [S1] FRR p. 47, 83 |
| NC State 2024 re-flight | 3,950 ft, 52.5 lb, drag cut 15% | 3,812 ft | +3.6% | [S1] addendum p. 6, 12 |
| Georgia Tech 2018 | 5,060 ft, actual masses | 4,734 ft | +6.9% | [S2] FRR p. 84 |
| NC State 2018 | file 44.5 lb against a 44.3 lb scale | 4.4% above sim | −4.2% | [S1] FRR p. 94 |

Five of six read high, by 3.6% to 18.4% (6.9% up without the 2024 run, whose drag was fitted to an earlier flight
of the same rocket); two reports blame an unpainted or rough finish [S1] 2020 FRR p. 87, 2021 FRR p. 104. This
matches [apogee-bias](apogee-bias.md) §2, where Niskanen's weighed rockets still ran 7% and 16% high; his thesis
prints no computed mass beside a measured one, so it adds no pair [S22] pp. 83–87.

## 5. What the tools and the rules say

- OpenRocket's wiki and user guide tell users to match the simulated mass to the finished rocket, glue, paint and
  decals included, with a stage-level override, and give no number [S23], [S24].
- No Student Launch handbook read (2020, 2022, 2024, 2026) sets a numeric growth margin. They ask for a "mass
  statement and mass margin" at CDR and require FRR predictions to use the as-built vehicle [S11].
- Teams set their own margins: Notre Dame follows the aerospace mass-growth-allowance standard, AIAA S-120A, at
  1.97% (2022) [S4] CDR 2022 p. 26, about 2.6% from its 2023 table (p. 29–30); Toledo used 7.14% (2024); Georgia
  Tech 10% (2015) and 30% (2017). A 2018 Spaceport America Cup team required a margin of at least 10% [S25] p. 3.

## What this suggests

1. **Count the private collection's whole-rocket overrides** (aggregates only) and split its +8.73% bias by them, as
   [apogee-bias](apogee-bias.md)'s *What this suggests* item 3 asks. At this note's +7.4% median, files with no
   override should read about 2.4 to 6.7 points higher than overridden ones; if they don't, mass is not the cause.
2. **Use +7.4% (IQR −1.4% to +13.1%) as the as-built mass term** for a file with no weighed override, wherever a mass
   uncertainty is set (M7.5's mass term, the Monte Carlo default). It is a two-sided spread, not a correction.
3. **The "weighed against estimated mass" idea** ([ideas](ideas-design-and-physics.md)) gains a number to print:
   the scale reads a median 7.4% above an un-weighed file, one in four by more than 13%.
4. **Teams' FRRs and addenda are an oracle source.** NC State's pair a weighed mass with a flown apogee in four
   seasons (section 4); a flight with its weighed mass and its logged apogee is the clean case the bias split needs.

## Open questions

- **How many of the collection's files carry a weighed override?** It decides whether the 2.4–6.7% applies to them.
- **Growth in hobby high-power files.** The Student Launch pairs come from budgeted teams and the forum pairs from
  small cardboard rockets; no public set covers a single flyer's fiberglass L2 or L3 rocket. Paint's bound says
  growth falls with size, and heavy builds get forum threads, so the forum medians likely overstate it, by how much
  is unknown.
- **Elasticity.** The −0.32% to −0.90% comes from two other rockets; these rockets' own were not measured.

## Sources (read 2026-10-16)

Team reports, forum posts, reviews and data sheets carry no license: cited, cached under `refs/` only.

1. **S1** NC State Rocketry report archive, CDR, FRR, flysheets and addenda, 2015–2026.
   <https://ncsurocketry.org/report-archive/>
2. **S2** Georgia Tech Team ARES, Student Launch documentation, 2015 and 2018. <http://usli.gatech.edu/documentation>
3. **S3** Purdue SEDS, 2020 CDR and FRR.
   <https://purdueseds.space/wp-content/uploads/psp-sl/casper/frr/Purdue%20University%20-%202020%20FRR%20Report.pdf>
4. **S4** Notre Dame Rocketry Team, 2018–2023 reports. <https://seniordesign.ee.nd.edu/>
5. **S5** University of Toledo Rocketry, 2024–2026 reports. <https://www.utrocketry.club/projects/nasa-usli-2024>
6. **S6** Caplinger, Madcow DX3 review. <https://www.rocketreviews.com/madcow-rocketry-dx3-by-mike-caplinger.html>
7. **S7** Stevens, Apogee Saturn 1B review.
   <https://www.rocketreviews.com/apogee-components-saturn-1b-170th--by-chan-stevens.html>
8. **S8** <https://www.rocketryforum.com/threads/minnie-magg-build-for-l1-attempt.172197/>
9. **S9** A thread on overriding CG in OpenRocket.
   <https://www.rocketryforum.com/threads/over-ride-cg-or-enter-mass-component-to-get-correct-cg-openrocket.35852/>
10. **S10** <https://www.rocketryforum.com/posts/1926820/>
11. **S11** NASA Student Launch handbooks: 2026, p. 16, 31, 33.
    <https://www.nasa.gov/wp-content/uploads/2025/08/g-677852-2026-sli-handbook-508-aug-7-optimized.pdf>; 2024, p. 14.
    <https://www.nasa.gov/wp-content/uploads/2023/08/2024-sli-handbook-final-copy1.pdf>
12. **S12** ThrustCurve motor search API. <https://www.thrustcurve.org/api/v1/search.json>
13. **S13** <https://www.rocketryforum.com/goto/post?id=64600>
14. **S14** <https://www.rocketryforum.com/threads/my-rockets-are-too-heavy.172760/>
15. **S15** UW SARP, Team 09 project report, 2018 Spaceport America Cup.
    <https://www.soundingrocket.org/uploads/9/0/6/4/9064598/09_project_report.pdf>
16. **S16** <https://www.rocketryforum.com/threads/dragon-rocketry-fat-boy-xl-build.179180/>
17. **S17** <https://www.rocketryforum.com/threads/aerotech-g-force-redux.10816/>
18. **S18** The Rocket N00b, "The Weight of Paint", 2016-11-06.
    <http://rocketn00b.blogspot.com/2016/11/the-weight-of-paint.html>
19. **S19** <https://www.oldrocketforum.com/archive/index.php/t-1027.html>
20. **S20** Rust-Oleum Painter's Touch 2X data sheet, rev. 111020.
    <https://www.rustoleum.com/-/media/387FD0183D54415A93C7E98516156EC5.ashx>
21. **S21** West System 105/205 data sheet, rev. 2.
    <https://eu.westsystem.com/app/uploads/2023/07/WEST-SYSTEM-105-Epoxy-Resin-with-205-Fast-Hardener-Rev-2.pdf>
22. **S22** Niskanen, OpenRocket MSc thesis, 2009. <https://openrocket.sourceforge.net/thesis.pdf>
23. **S23** OpenRocket wiki, Overrides and Surface Finish. <http://wiki.openrocket.info/Overrides_and_Surface_Finish>
24. **S24** <https://openrocket.readthedocs.io/en/latest/user_guide/overrides_and_surface_finish.html>
25. **S25** Stanford SSI, Team 111 project report, 2018.
    <https://www.soundingrocket.org/uploads/9/0/6/4/9064598/111_projectreport.pdf>
26. **S26** <https://www.rocketryforum.com/threads/estimating-the-weight-of-glue-and-paint.188221/>
