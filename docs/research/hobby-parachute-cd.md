# Hobby parachute drag coefficients: what makers state and what flights measure

What this covers: the drag coefficient (C_D) each common hobby parachute maker states, the area it is taken on,
what the maker's own descent table implies, and what independent tests measure. It is for M5.6b's parachute
catalog, M6.6's landing-energy and descent-time numbers and M8.1's auto-sizing. How far to trust it: most numbers
here are back-solved from the makers' own tables, which test a table against its own claim, not the canopy in the
air. **No wind-tunnel or controlled drop test of any Fruity Chutes, Rocketman, Top Flight, SkyAngle, Spherachutes
or b2 canopy was found.** The independent data are one 15-flight series on a small flat chute, two rooftop drops,
29 balloon descents with a poorly known mass, and forum reports. Builds on [suite-parachutes.md](suite-parachutes.md)
(vendor C_D list, which way each output must err) and [parts-catalog.md](parts-catalog.md) (store C_D with its area).
Researched 2026-10-17 and 2026-10-18.

## Method

- Back-solving a table row: C_D·S = 2W/(ρv²), W the stated load, v the stated speed, ρ = 1.225 kg/m³ (sea level)
  unless the maker names another. A row then gives a **drag area** C_D·S, which needs no reference area at all.
- "Flat circle of the size" is π·(size)²/4: the area a simulator uses when the catalog size is typed in as a
  diameter. Comparing every maker on this one basis shows what a user gets from the size alone.
- Descent speed at one mass goes as 1/√(C_D·S) and landing energy as 1/(C_D·S): a drag area that is really 0.8 of
  the modeled one means a 12% faster descent and 25% more landing energy. Every back-solved number is computed here.
- Reference bases (Knacke, NWC TP 6575, p. 5-2): D_0, the diameter of a circle of the whole cloth area (the
  "nominal" area S_0); D_c, the constructed diameter; D_p, the inflated projected diameter.

## 1. Each maker's figure and its area

| maker, family [1 to 8] | stated C_D | stated area | table implies, on its own area | on flat circle of size |
|---|---|---|---|---|
| Fruity Chutes Iris Ultra | 2.2 | opening (projected) diameter | 2.10 to 2.21, net of the spill hole | 2.04 to 2.14 |
| Fruity Chutes Classic Elliptical | 1.5 to 1.6 | opening diameter | 1.40 to 1.53, net of spill | 1.34 to 1.47 |
| Rocketman Standard (parabolic) | 0.97 | arc over the top | as the next column | 0.74 to 0.90, median 0.84, 72 rows |
| Rocketman Rotating Star | 1.65 | not stated | none | 42 of 50 rows 1.64 to 1.69 |
| Rocketman HP "CD 2.2" (toroidal) | 2.2 | not stated | most 2.26 to 2.31 on listed projected | most 2.20 to 2.23 |
| Rocketman Elliptical | 1.6 | not stated | 1.31 to 1.50 on listed projected (3 to 10 ft) | 1.26 to 1.44 |
| Top Flight standard, 30 in and up | none | not stated | none | 1.04 to 1.49, most 1.1 to 1.4 |
| Top Flight X-Type | none | not stated | none | 0.41 to 0.82 |
| SkyAngle Classic (b2 Rocketry) | 0.80 to 1.89 | "circular cap", unpublished | can't check | 0.94 (20 in) to 1.52 |
| Spherachutes | 0.75 | arc over the top (gore length) | 0.80 to 0.93 | 0.80 to 0.93 |

- **Fruity Chutes.** The Iris ratings at 20 ft/s imply 2.10 to 2.21 on the opening circle minus the spill hole
  (spill 17.6% of D, 3.1% less area than πD²/4), loads being whole pounds; so 2.2 is on the netted area, not on
  πD²/4 as the calculator page writes [3]. The Classic
  Elliptical's ratings sit at or below the bottom of its stated band; the 72 and 84 in "custom" chutes imply 1.31.
  A simulator that uses 1.6 where the rating implies 1.31 to 1.50 descends 3 to 10% slower than the maker's own
  rating. On whole cloth area, the basis Knacke's C_D0 uses, the calculator's database gives the Iris 1.24 and the
  elliptical 0.84 [3].
- **Rocketman.** Its "size" is the length over the top, line loop to line loop [4]. The Standard's 0.97 against
  its own table's 0.84 means a simulator using 0.97 on the flat circle of the size predicts a 7% slower descent
  than Rocketman's table (√(0.837/0.97) = 0.93). The polyconical and disk-gap-band tables are one table (1.27);
  the TARC table's load grows with v, not v², so it is not a drag table. Rotating Star and HP CD 2.2 match.
- **Top Flight** states no C_D and no size definition [5]. Its guide's 17 to 22 ft/s rows imply about 1.2 on the
  flat circle for standard chutes of 30 in and up, well above the 0.75 usually assumed for a flat sheet (and used
  for Top Flight by the Valkyrie list [7] and Fruity Chutes' calculator [3]). If 0.75 is right, the guide's loads
  fall at 21 to 28 ft/s, about 26% faster than charted (√(1.2/0.75) = 1.26). On the true hexagon area, C_D is 9%
  lower if the size is across flats and 21% higher if across corners; the definition is not published.
- **SkyAngle is b2 Rocketry** (one maker). Its Cd is on "the reference surface area of circular cap only" [6],
  and the cap sizes it implies are not in step with size (the 24 in below the 20 in; the 44 a 35 in, the 52 a 50 in;
  CERT-3 Large and XLarge imply the same cap with drag areas 2× apart). The drag areas follow v² to ±3% and are
  the usable number. Pairing b2's Cd with the size as a diameter predicts the Classic 44 descending 20% slower
  than b2's own table (√(1.21/1.87) = 0.80).
- **Spherachutes.** The chart's "size" equals π·D/2 to 0.1 in on every row: the arc over the crown (a 48 in chute
  is 30.6 in across) [8]. Its rows imply 0.80 to 0.93 on that size's flat circle, so using the stated 0.75 there
  descends 4 to 11% faster than the chart. Its 5,000 and 10,000 ft columns scale with pressure, not density.
- **Fruity Chutes' calculator** [3] holds 121 models from six makers, at one sea-level density. For other makers
  it fits an "equivalent reference diameter" and a C_D to their published rates, so its numbers for them are
  their tables, not tests. Its Spherachute entries carry 30% less drag area than the current Spherachutes chart.
- **Valkyrie's master list** [7] has 50 rows and no reference area for any of them. Of its six "calculated" rows,
  the Rocketman Standard (0.83) matches the table and CERT-3's 1.98 is the mean of b2's four Cds; its disk-gap-band
  0.55 is far below Rocketman's 1.27 table, source not given, and its SkyAngle Classic "1.54 avg" doesn't reproduce
  (b2's eight Classic Cds average 1.32).

## 2. Independent measurements

- **Firth, Apogee 24 in flat hexagon** [9]: 15 logged flights of one rocket (396 g, BME280 barometer at 4 Hz,
  density 1.02 kg/m³ at a 2,160 m site). Most landed at 5 to 7 m/s, which maps to C_D 1.04 to 0.53 on the circle
  of the flat-to-flat width; the center of the band, 6 m/s, gives 0.72 against Apogee's suggested 0.75. The
  0.53 to 1.04 are the edges of a shaded speed band, not a per-flight minimum and maximum. Descent against the
  0.75 prediction: −15% to +19%. On the true hexagon area (1.103× that circle) each value is 9% lower.
  (This is Firth's series; Flanagan's later review [10] restates it.)
- **Catholic University rooftop drops** [11]: two small hexagonal kit chutes, hand-timed from 20 m, C_D fitted
  with the equations of motion on the hexagon's sheet area: 0.99 and 1.28 at loadings of about 0.68 kg/m²,
  descending 13% and 23% slower than a 0.75 rating predicts. Hand timing is worth about ±6% in C_D per trial
  (estimated here, not by the authors).
- **Balloon descents on a 7 ft Rocketman** [12]: 29 flights, mass "not accurately recorded", GNSS every 5 min.
  Speeds predicted from the maker's area and C_D were "10–20% too low" near the ground (evidence the authors call
  marginal): measured 1.10 to 1.25 times predicted, a drag area 0.64 to 0.83 of the maker's. Weak, but the sign
  agrees with the table back-solve above.
- **Builder reports** (forum; leads only) [13]: a 7 ft Rocketman Standard at 6.2 lb logged 18 to 20 ft/s at a
  610 ft field, where the table gives 11 lb at 17 ft/s. That is C_D 0.34 to 0.43 on the 7 ft circle, half the
  table. A 54 in Spherachute at about 7 lb averaged 22 to 26 ft/s: 0.55 to 0.77 on the 54 in circle, against
  the chart's 0.80 to 0.93.
- **Student Launch.** One report read flew a 72 in Iris main and found its descent "slightly higher than the predicted
  model" [14]; its numbers are in plots. The team's next report modeled the airframe under the main as a 10.5 in chute
  of C_D 0.5 [15]. NASA's own slides tell teams not to trust vendor C_D [16]. Mining these reports needs a
  figure-reading pass per report.
- **Iris Ultra log, re-counted (maker-run, not independent)** [17]: 16 rows, 2011 to 2016. Porous fabric (n = 14,
  seven the lighter SUL build) 2.083 to 3.224, median 2.53, s.d. 0.37; two zero-porosity rows 2.40 and 2.99; 13 of 16
  at or above 2.2. Every row reproduces with one fixed density: sea-level ρ on the spill-netted area, or equally 1.188
  kg/m³ on the full circle (0.00238 × 0.969 = 0.002306 slug/ft³). So the page's "up to 20% from the atmosphere" is a
  density error that makes high or hot sites read low (at a 5,000 ft field the true value is 16% above the logged
  one); site elevations aren't given. Seven rows from one tester, one day and one 13.78 lb load still spread about
  ±15%. Measured descent against the rating's: median −7%, range +3% to −17%.

## 3. Knacke: why one C_D0 doesn't fit hobby descents

Knacke [18] is cited, not reproduced; values as read from its tables and figures.

- **Type table** (Table 5-1, p. 5-3): flat circular C_D0 0.75 to 0.80, D_p/D_0 0.67 to 0.70; hemispherical
  0.62 to 0.77, D_c/D_0 0.71; cross 0.60 to 0.85, D_c/D_0 1.15 to 1.19; annular 0.85 to 0.95, D_p/D_0 0.94.
  Converted (computed here): a hemisphere on its skirt diameter is 1.23 to 1.53, so the hobby "1.5 for a
  hemisphere" is the top of the range, and a cross on its span is 0.42 to 0.64, which matches the 0.4 to 0.62
  vendors quote for X-form chutes. A hemisphere sold by its arc L has D_0 = 2√2·L/π = 0.900 L and a skirt (D_c) of
  2L/π = 0.637 L, so D_c/D_0 = 0.71 as Knacke has it, and D_p is about 0.59 L.
- **Speed** (Figures 5-18 and 5-24): solid flat and conical canopies read about 0.75 to 0.90 at 30 ft/s and above,
  about 0.85 to 1.05 at 15 to 20 ft/s and 1.1 to 1.2 at 10 ft/s, as oscillation gives way to glide (text, p. 5-2).
  Hobby mains fly at 15 to 22 ft/s, in the rising part. Against the simulator's flat-circular default of 0.775
  ([recovery.md](../physics/recovery.md)), a 15 to 20 ft/s descent at 0.85 to 1.05 is 5 to 14% slower than predicted.
- **Lines** (Figure 5-20): against lines of one D_0, lines of 0.75 D_0 cost about 7 to 9% of C_D0 and lines of 0.6
  D_0 up to 20% (short lines are common on kit chutes [10]); 1.5 D_0 adds 6 to 9% on flat canopies.
- **Wake** (Figure 5-21): at D_p/D_B = 3 the forebody costs about 13% of drag at 1 body diameter of trailing
  distance and under 5% beyond about 12. Hobby mains have larger D_p/D_B, so this is an upper bound by
  extrapolation, not a measurement.
- **A tunnel check** (NASA TN D-6423 [19]): 3.5 ft flat circular canopies, fully inflated and free-floating with no
  frame in the Langley spin tunnel, averaged C_D0 0.81 and 0.78 (low and high porosity), inside Knacke's range.

## 4. Spread against the maker, in descent speed

| canopy | basis | measured C_D | n | maker | descent against the maker's |
|---|---|---|---|---|---|
| Apogee 24 in hexagon [9] | flat-to-flat circle | 0.53 to 1.04, center 0.72 | 15 flights | 0.75 | −15% to +19% |
| kit hexagons [11] | hexagon sheet | 0.99, 1.28 | 2 chutes | 0.75 typical | −13%, −23% |
| Rocketman 7 ft [12] | maker's spec | C_D·S 0.64 to 0.83 of spec | 29 balloon flights | 0.97 | +10% to +25% low down |
| Rocketman 7 ft [13] | 7 ft circle | 0.34 to 0.43 | 1 flight | 0.97 | +50% to +69% |
| Spherachute 54 in [13] | 54 in circle | 0.55 to 0.77 | 1 rocket, average | chart, 0.80 to 0.93 | +2% to +30% |
| Iris Ultra [17], maker-run | projected | 2.08 to 3.22, median 2.53 | 16 flights | 2.2 | +3% to −17%, median −7% |

## What this suggests

- **Store the maker's drag area, not its C_D.** For most families above, the table's C_D·S per size is self-consistent
  where the stated C_D and size are not (Rocketman's TARC table and a few typo rows aside; SkyAngle's cap area is
  unpublished; Rocketman's and Spherachutes' sizes are arcs; Fruity Chutes nets the spill hole). M5.6b's "each Cd
  stored with its reference area" can be met by storing C_D·S, with the stated C_D and area kept as text.
- **The maker's table is not the low end.** Rocketman's canopy flies below its table's drag area, by 17 to 36%
  (balloons) and by half (one flight). Firth's center sits within 4% of Apogee's 0.75, but his band's low edge is 0.71
  of it; the rooftop drops and the Iris log fly above. M6.6's landing energy and M8.1's sizing need a low-end factor
  on the table drag area; no source found sizes it for HPR canopies, so the factor needs a measurement.
- **The simulator's 0.775 default errs on the fast side at hobby descent speeds** (Knacke: 0.85 to 1.05 at 15 to
  20 ft/s). Fast is the safe side for landing energy, but it shortens descent time and drift by 5 to 14%, the
  flattering side for both. A speed-dependent C_D0 from Figures 5-18 and 5-24 would carry that, if the canopy's
  lines and wake are known.
- **A flyer's own log is the best source of a canopy's C_D.** The analyzer already reads barometric logs; a
  descent C_D·S fit, with the canopy, descent mass and site density stated, would let flyers build the dataset
  this topic couldn't find.

## Open questions

- Top Flight's size definition (across corners or flats) sets a −9% to +21% band on its implied C_D.
- SkyAngle's cap areas are unpublished, so b2's Cd column can't be checked; only its drag areas.
- Whether Rocketman's and Top Flight's tables are measured or computed isn't stated.
- Why Fruity Chutes' calculator models Spherachutes with 30% less drag area than the current chart.
- Van Milligan's 0.64 to 1.05 (*Peak of Flight* 449, cited by Flanagan [10]) is in an image and wasn't verified.
- Not reached: Malewicki's 1970 indoor drops, a 2000 *Extreme Rocketry* Cert-3 article, a 2021 Hanoi tunnel
  study of small flat and extended-skirt models, and Apogee's and Aerotech's parachute pages.

## Sources

All read 2026-10-17 or 2026-10-18; maker pages and forum posts are copyrighted: cite only, cached privately.

1. Fruity Chutes product pages (Iris Ultra Standard and Compact, Classic Elliptical): https://shop.fruitychutes.com/
2. Fruity Chutes, NARCON 2013 handout, pp. 1 and 2:
   https://fruitychutes.com/files/help/NARCON-2013-Rocketry-Recovery-Technology.pdf
3. Fruity Chutes descent-rate calculator and its notes on reference diameters:
   https://fruitychutes.com/help_for_parachutes/parachute-descent-rate-calculator
4. The Rocketman product pages and descent tables: https://www.the-rocketman.com/
5. Top Flight Recovery, parachutes and descent guide: https://topflightrecoveryllc.homestead.com/descent_rates.html
6. b2 Rocketry (SkyAngle), descent chart and FAQ: https://www.b2rocketry.com/ (parachut.htm, faqs.htm)
7. Valkyrie Recovery Systems, hobby parachute C_D master list (2024):
   https://www.valkyrierecoverysystems.com/uploads/1/1/9/5/119545746/hobby_parachute_cd_master_list.pdf
8. Spherachutes descent-rate chart: https://spherachutes.com/pages/decent-rate-chart
9. J. Firth, "Drag Coefficient of a Model Rocket Recovery Parachute", *Peak of Flight* 662 (2025), pp. 3 to 9:
   https://www.apogeerockets.com/education/downloads/Newsletter662.pdf
10. D. Flanagan, "Drag Coefficients of Model Rocket Parachutes", *Peak of Flight* 668 (2025), pp. 3 to 11:
    https://www.apogeerockets.com/education/downloads/Newsletter668.pdf
11. T. A. Campbell Jr. and M. Okutsu, "Model Rocket Project for Aerospace Engineering Course" (2017), §4.2,
    Table 1: https://arxiv.org/abs/1708.01970
12. E. L. Sirks et al., "Download by Parachute: Retrieval of Assets from High Altitude Balloons" (2020), §4.1.3
    to §4.3.1: https://arxiv.org/abs/2004.10764
13. The Rocketry Forum, "Rocketman parachutes posted decent rates not matching reality" (2023):
    https://www.rocketryforum.com/threads/rocketman-parachutes-posted-decent-rates-not-matching-reality.179543/
14. Penn State LionTech, 2017 PLAR, §5: https://sites.psu.edu/psurocket/files/2018/04/ltrl_2017_plar-ruubzr.pdf
15. Penn State, 2018 FRR slides:
    https://sites.psu.edu/psurocket/files/2017/08/Penn-State-2018-FRR-Presentation-1ugzagv.pdf
16. NASA Student Launch, "Rocketry by the Numbers" (2024), pp. 8 and 13:
    https://www.nasa.gov/wp-content/uploads/2023/09/nasa-sl-2024-arw-rocketry-by-the-numbers-508.pdf
17. Fruity Chutes, Iris Ultra Cd performance log:
    https://fruitychutes.com/help_for_parachutes/iris-parachute-cd-performance-log
18. T. W. Knacke, *Parachute Recovery Systems Design Manual*, NWC TP 6575 (1991), Tables 5-1 and 5-2,
    Figures 5-18 to 5-24, pp. 5-2 to 5-25.
19. S. H. Scher and J. W. Young, NASA TN D-6423 (1971), pp. 3, 5 and 6:
    https://ntrs.nasa.gov/api/citations/19710023919/downloads/19710023919.pdf
