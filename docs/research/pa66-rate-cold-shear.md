# How fast loading and cold change nylon 6/6's strength, for shear pins

This note sizes how much a fast break and a cold pin raise unfilled nylon 6/6's (PA66's) strength. It serves the pin
load "at the high end of their strength" in M6.7's charge sizing ([roadmap](../ROADMAP.md); issue
[#857](https://github.com/nrdptel/fusionspace-eridanus/issues/857), pin strength), and which side a sized charge errs
on. It builds on [shear-pin-strength](shear-pin-strength.md) §2, §5 and §6, whose rate factor (+0% to +20%) and cold
factor (×1.16 at 0 °C) were tensile-yield readings with no strain rate (it now cites this note), and doesn't repeat it.
Read 2026-11-03: two split Hopkinson bar studies, one torsion study, DuPont's design-guide plots and table, a BASF
data sheet, a Sandia abstract and Fetter's ground tests. **Inference** marks this note's own reading. No source breaks
a pin, shears nylon faster than 0.2 /s, or loads it fast and cold together. Trust the measured ratios to about ±5%,
the combined factors to their sign.

**In short.**
- **A charge strains a pin 2,000 to 100,000 times faster than a data-sheet test.** **Inference:** a yield strain
  of 4.5% (dry) to 20% or 25% (conditioned) reached over a 0.2 to 2 ms pressure rise gives 22 to 1,250 /s; ISO 527
  at 50 mm/min runs near 0.01 /s.
- **Near those rates PA66 yields at ×1.27 to about ×1.95 its slow value where measured, not ×1.0 to ×1.2.** DuPont's
  fastest tensile test gives ×1.27 dry and ×1.35 at 50% RH (about 27 to 54 /s, **Inference**); Hopkinson bar
  compression gives ×1.29 at 1,100 /s (Xu 2020) and about ×1.95 near 2,000 /s (Gómez-del Río 2024).
- **Cold goes further than the parent note shows:** ×1.86 at −40 °C for conditioned Zytel 101 (DuPont Fig. 3), and
  tensile strength ×1.37 at −40 °C dry (DuPont Table 7); dry −30 °C yield is not plotted (×1.34, **Inference**).
- **Shear yield tracks tensile yield across rate** (ratio 0.52, flat over 0.003 to 0.03 /s), so tensile factors
  carry to shear at slow rates; no shear test above 0.2 /s was found.
- **The parent's 14,750 psi "dry, 0 °C, fast" stack errs low** (too small a charge) whether rate and cold multiply or
  not: 14,900 to 15,600 psi on DuPont's measured top rate, up to 20,900 psi on the compression fit at 100 /s. A
  single Eyring process shrinks the rate gain when cold (multiplying overstates by 3% to 9%); nothing shows which
  holds below 23 °C. Heating in a fast break and notch brittleness pull the force down, unbounded.

## 1. How fast a charge strains a pin (Inference)

- Fetter's chamber pressure rises as fast as his sensor's 2 ms response, and the true rise could be much faster
  ([1] p. 52); his fixture's plug leaves in under 2 ms (p. 39); a pressure wave crosses his 15 in bay in 1 ms (p. 67).
- PA66 yields in tension at 4.5% strain dry and 20% conditioned (BASF Ultramid A3K, ISO 527 at 50 mm/min, [6]
  p. 2). DuPont gives Zytel 101's elongation at yield as 5% dry as molded (DAM) and 25% at 50% RH ([3] Table 7, p. 9).
- **Inference:** if the load climbs to the break over 0.2 to 2 ms, the mean equivalent strain rate up to yield is
  yield strain ÷ rise time: 22 to 250 /s dry and 100 to 1,250 /s conditioned. A shear strain rate is √3 times the
  equivalent rate ([7] eq. 3). A slow-burning charge (Fetter's vial tests in partial vacuum, p. 52) lowers this; a
  rise faster than 0.2 ms raises it. Neither end is measured on a pin. ISO 527's 50 mm/min is near 0.01 /s over a
  gauge of several centimeters; Damerau's pulls (parent §4, 20 in/min) and ASTM D732 punch tests are slow too.

## 2. Loading rate at room temperature: 1 to 2,000 /s

| Source, test | Material | Slow value | Near 1 /s | Near 50 to 100 /s | Near 1,000 to 2,000 /s |
|---|---|---|---|---|---|
| DuPont Fig. 5 [3], tension | Zytel 101, DAM | 83.6 MPa | +3% | +27% (plot's top) | not tested |
| DuPont Fig. 5 [3], tension | Zytel 101, 50% RH | 55.9 MPa | +7% | +35% (plot's top) | not tested |
| Xu [4] Fig. 5a, compression | PA66 bar | 86 MPa at 0.01 /s | +5% | no data | +29% (1,100 /s) |
| Gómez-del Río [5] Fig. 3, compression | PA66 rod | 83 MPa, fit | +35%, fit | +70% (100 /s), fit | +95% (2,000 /s) |

- **DuPont Fig. 5** ([3] p. 31) plots yield against crosshead speed, labeled "strain rate". This note's pixel scan
  (400 dpi, ±1 MPa): DAM 83.6 MPa at the slow end, 86.0 MPa at 2,540 mm/min and 106.2 MPa near 183,000 mm/min; 50% RH
  55.9, 59.8 and 75.4 MPa (fast end near 188,000 mm/min). The guide files these curves under ASTM D638 (p. 30).
  **Inference:** on D638's Type I bar (57 mm narrow section, 115 mm between grips [9]) the plot spans about 0.008 to
  0.016 /s at its slow end and 27 to 54 /s at its fast end, and 2,540 mm/min is 0.4 to 0.7 /s.
- **Xu et al. 2020** ([4] §3.3, Fig. 5a): PA66 cylinders cut from one 100 mm bar; maker and moisture unstated.
  Yield reads 86, 88 and 90 MPa at 0.01, 0.1 and 1 /s, then 111, 117, 125, 131 and 135 MPa from 1,100 to 3,500 /s
  (read by eye, ±1 MPa). The text gives +4.0 MPa from 0.01 to 1 /s and +20 MPa from 1,000 to 4,000 /s. Their two
  fits disagree: Fig. 5b's line gives ×1.08 at 100 /s against 1 /s, Table 2's parameters ×1.05, and Fig. 5b's points
  sit about 9% above Fig. 5a's ratios. Times Fig. 5a's 90/86: ×1.10 to ×1.13 against 0.01 /s (**Inference**).
- **Gómez-del Río and Ruiz 2024** ([5], accepted manuscript): unfilled PA66 rod from Ensinger, moisture unstated;
  5 mm tall cylinders at 1.3 and 90 mm/min, and Hopkinson bar tests at 1,000 to 2,000 /s at room temperature (p. 3).
  Fig. 3 (PDF p. 9) fits compressive yield as 112.67 + 14.67 log₁₀(rate) MPa: 83 MPa at 0.01 /s, 142 at 100 /s,
  157 at 1,000 /s. Its one measured Hopkinson point reads about 163 MPa (error bar 155 to 171) near 2,000 /s, ×1.95
  over 83.8 MPa (pixel scan); they report yield rising about 100% (p. 5). With no data from 0.3 to about 2,000 /s,
  100 /s is interpolated.
- **The two Hopkinson studies differ by about ×1.37 near 2,000 /s** (measured: 1.95 against Xu's 1.40 to 1.45).
  Xu's data are bilinear: almost flat below 1 /s, steep above 1,000 /s. **Inference:** the usual reading is a
  secondary relaxation that a fast test brings into play; no source read here shows that for PA66.
- **A softer PA66 can be far more rate-sensitive.** In Ghorbel et al.'s tensile tests ([7] Fig. 10, by eye) yield ÷
  temperature rises about ×1.7 (0.11 to 0.18 MPa/K) from 2.5 × 10⁻⁴ to 0.14 /s, from a slow 32 MPa: **Inference:**
  a moist or low-crystallinity bar (25% crystallinity, [7] Table 2), unlike DuPont's 83.6 MPa DAM.

**Inference:** at a pin's 20 to 1,250 /s, the measured gain over the slow test is at least +27% (DuPont, 27 to
54 /s, dry) and +29% (Xu, 1,100 /s). Every measured fast value sits above the parent's +20% top.

## 3. Cold: −10 to −40 °C, dry and conditioned

- **Conditioned, new here:** DuPont Fig. 3 ([3] p. 30), Zytel 101 at 50% RH, gives a yield point of 108.9 MPa at
  −40 °C and 58.7 MPa at 23 °C (this note's 400 dpi pixel scan, ±1 MPa): ×1.86. With Fig. 4's 93 MPa at −20 °C
  (this note's scan of [3] Fig. 4) that is ×1.58. **Inference:** −30 °C conditioned is about 101 MPa, ×1.72, by
  straight interpolation; the 50% RH curve falls about 0.8 MPa per °C all the way from 23 °C to −40 °C.
- **Dry, below −10 °C:** Fig. 4's DAM curve leaves the plot's 100 MPa top at about −12 °C. DuPont's Table 7 ([3]
  p. 9) gives Zytel 101's tensile *strength* (not yield) at −40 °C: 114 MPa DAM and 110 MPa at 50% RH, against 83
  and 77 MPa at 23 °C (×1.37 dry). **Inference:** Fig. 4's DAM slope of 0.50 to 0.55 MPa per °C (23 to −10 °C)
  reaches 109 to 110 MPa at −30 °C (×1.34 to ×1.35 over 81.3 MPa) and 114 to 116 MPa at −40 °C, as in Table 7.
- **Moisture acts like heat.** Sandia's compression tests (dry, 2% and 6% water; 20 to 200 °C; 10⁻⁴ to 10² /s)
  give an empirical rule: 1% moisture is equivalent to 14 °C ([8], abstract). **Inference:** the 2.3-point gap
  from DAM (0.2%) to 50% RH (2.5%) is then about 32 °C. DuPont's 50% RH yield at 23 °C (about 59 MPa) matches its
  DAM yield near 55 °C (57 MPa at 55 °C, 58.5 MPa near 53 °C, Fig. 4). Below 0 °C the match loosens: dry 0 °C reads
  94 MPa against about 103 MPa for 50% RH at −32 °C (interpolated between Figs. 4 and 3), 9% apart.
- **Notch sensitivity in the cold is mixed dry, strong conditioned.** Dry A3K's notched Charpy falls from 5 to
  4 kJ/m² from 23 °C to −30 °C, but its notched Izod rises from 5.5 to 6 kJ/m² ([6] p. 2). Conditioned Zytel 101L
  drops from 15 to 3 kJ/m² (Charpy) and 12 to 3 kJ/m² (Izod) at −30 °C ([11] p. 2). **Inference:** a thread root
  is a notch, so a cold conditioned pin may crack before it fully yields: sign lower, size unbounded.

## 4. Does shear yield follow tensile yield at speed?

- Ghorbel et al. twisted PA66 bars at 23 °C ([7] §3.1 to §3.3). Shear yield at 0.3% offset rose from 17.5 to
  22.8 MPa (+30%) as shear strain rate went from 0.0039 to 0.196 /s (Fig. 9; this note's pixel scan, ±0.3 MPa).
- The ratio of shear to tensile yield stayed near 0.523 at equivalent rates of 0.0026 to 0.034 /s (Fig. 13); von
  Mises gives 0.577. The compression-to-tension ratio stayed at 1.067 (Fig. 12, §3.3 text).
- **Inference:** at slow rates a tensile rate factor carries over to shear. Above 0.2 /s shear rate no PA66 shear test
  was found, so the carry-over at a pin's rate is assumed, not shown. D732's punch value is a strength, not a yield
  (66.2 ÷ 82.8 = 0.80 of tensile yield for Zytel 101, [3] Table 7, p. 9); whether that ratio holds at speed is unknown.

## 5. Do rate and cold multiply?

- **DuPont measured rate at three temperatures** (Fig. 5, [3] p. 31). Over the same crosshead speeds (about 5,900 to
  183,000 mm/min; this note's scan at 300 to 400 dpi, ±1 to ±2 MPa):

  | Curve | Low end | High end | Gain |
  |---|---|---|---|
  | DAM, 23 °C | 88.4 MPa | 106 MPa | +20% |
  | DAM, 130 °C | 30.8 MPa | 32.0 MPa | +4% to +5% |
  | 50% RH, 130 °C | 20.7 MPa | 23.2 MPa | +12% |
  | DAM, 200 °C | 12.7 MPa | 14.8 MPa | +16% |
  | 50% RH, 200 °C | 9.8 MPa | 11.9 MPa | +21% |

  The relative gain is not monotone in temperature, and every plotted temperature but 23 °C lies above the glass
  transition (70 °C dry, 40 °C conditioned, for Zytel 101L [11] p. 2). Fig. 5 says nothing below 23 °C.
- **A single Eyring process** has a yield-versus-log-rate slope proportional to absolute temperature, so its gain in
  MPa shrinks when cold. **Inference:** scaling DuPont's 23 °C gain by T/296, multiplying overstates by 3.5% (×1.20)
  to 4.5% (×1.27) at 0 °C, and by about 9% at −30 °C dry (109 + 22 × 243/296 = 127 MPa, not 109 × 1.27 = 138).
- Ghorbel et al. fit PA66's tensile yield with two processes (Ree–Eyring); a single Eyring line misses its upturn
  ([7] §3.2, Fig. 10). **Inference:** in a two-process material the second, steeper branch starts at a lower rate
  when the resin is cold, so multiplying would understate. Which form holds below 23 °C has no source here; Walley et
  al. compared polymers at high rate at 300 K and 100 K [10], but this note could not reach the paper.

## 6. What it does to the parent's high end

Single shear on the pitch-diameter area (parent §3: 0.00435, 0.00721 and 0.01088 in²), 10,600 psi at 23 °C. "Top"
is DuPont's measured top rate (×1.27); "Eyring" rows scale the rate gain by T/296 (§5), the low reading.

| Case, factors on 10,600 psi | Stress | 2-56 | 4-40 | 6-32 |
|---|---|---|---|---|
| Parent: ×1.16 (0 °C) × 1.20 (rate) | 14,750 psi | 64.2 lbf / 285 N | 106.4 lbf / 473 N | 160.5 lbf / 714 N |
| Dry 0 °C, top, Eyring: 1.16+0.27×0.92 | 14,940 psi | 65.0 lbf / 289 N | 107.7 lbf / 479 N | 162.5 lbf / 723 N |
| Dry 0 °C, top: ×1.16 × 1.27 | 15,620 psi | 67.9 lbf / 302 N | 112.6 lbf / 501 N | 169.9 lbf / 756 N |
| Dry −30 °C, top, Eyring: 1.34+0.27×0.82 | 16,550 psi | 72.0 lbf / 320 N | 119.4 lbf / 531 N | 180.1 lbf / 801 N |
| Dry −30 °C, top: ×1.34 × 1.27 | 18,040 psi | 78.5 lbf / 349 N | 130.1 lbf / 579 N | 196.3 lbf / 873 N |
| Dry 0 °C, fit at 100 /s: ×1.16 × 1.70 | 20,900 psi | 90.9 lbf / 404 N | 150.7 lbf / 670 N | 227.4 lbf / 1,012 N |

- **Which way the parent errs:** low in every row, by 1% to 6% (DuPont's measured top rate) and up to 42% (the
  compression fit) at 0 °C dry, and by 12% to 22% at −30 °C on DuPont's top rate. Low means too small a charge: the
  flattering side.
- **What pulls it back down, unbounded:** heat from a fast break softens nylon after yield (a Hopkinson test lasts
  about 200 µs, [5] p. 5). **Inference:** that heat barely escapes in a millisecond, so a pin that must strain well
  past yield to part may break below the yield-based force; so may cold notch brittleness in a conditioned pin (§3).
  Moisture lowers it 5% to 33% (parent §5); a conditioned pin's ×1.35 and ×1.86 apply to that lower base.

## What this suggests

- **M6.7 could carry the rate factor as a sourced range,** ×1.27 (DuPont's fastest measured tensile yield, dry,
  [3] Fig. 5) to ×1.7 (compression fit at 100 /s, [5] Fig. 3), and say which it prints. The parent's +0% to +20% sits
  below every measured value at a pin's rate. Expected effect: the high-end pin force rises 1% to 42% at 0 °C.
- **Extend the cold table** to −40 °C (×1.86 conditioned yield, [3] Fig. 3; ×1.37 dry strength, [3] Table 7) and
  flag dry yield below −10 °C as extrapolated.
- **Report both combinations** (multiplied and Eyring-scaled); both sit above the parent's stack, and the high end
  should take the multiplied one until a fast, cold test shows otherwise.
- **Name the counter-causes beside the number:** adiabatic softening and notch brittleness, both unbounded.
- **A measurement would settle it:** 2-56 and 4-40 pins in single shear at 0.01, 1 and about 100 /s (a drop weight),
  at 23 °C and −20 °C, dry and conditioned; and a pin-side pressure trace sampled faster than 0.1 ms.

## Open questions

- No test of nylon 6/6 loaded fast and cold together, nor a measured pin strain rate, was found. Walley et al. [10]
  (via HAL) and Kawahara et al.'s full report [8] (via DTIC) refused automated fetches on 2026-11-03.
- No PA66 shear yield above 0.2 /s; the shear-to-tension ratio at speed is assumed. Lead not followed: Jin et al.
  2018 (*Polym. Test.* 69:563–582), cited by Xu [4] §3.3, ran compression-shear specimens on PA66.
- The Hopkinson studies disagree by about ×1.37 near 2,000 /s; neither states grade, moisture or crystallinity fully.
- Dry yield below −10 °C: only Table 7's −40 °C tensile strength was found; Zytel's CAMPUS curves may hold more.

## Sources

Read 2026-11-03; cached under `refs/research/pa66-rate-cold-shear/` (gitignored), [1], [3], [11] by the parent note.

1. T. Fetter, *Using Black Powder for Parachute Deployment*, Rev 1.2, 2025, printed pp. 39, 52, 67. Cite only.
2. [shear-pin-strength](shear-pin-strength.md), this repository, §2 to §6.
3. DuPont, Zytel and Minlon Design Guide, Module II, Table 7 (p. 9), pp. 30 to 31, Figs. 2 to 5:
   http://foremostplastic.com/wp-content/uploads/2015/04/DuPont-Module-II-Nylon.pdf. Copyrighted; cite only.
4. L. Xu, C. Cheng, C. Du, Z. Jiang, Z. Du, G. Gao, "Semi-Crystalline Polymers Applied to Taylor Impact Test:
   Constitutive, Experimental and FEM Analysis", *Polymers* 12(7):1615, 2020, §3.1 to §3.3, Figs. 3 and 5, Table 2.
   https://doi.org/10.3390/polym12071615 (Europe PMC PMC7408129). CC BY 4.0; cached (`xu-pkg/`, `polymers-*.xml`).
5. T. Gómez-del Río, A. Ruiz, "High Strain Rate Mechanical Behavior of Polyamide 66 and Polyamide 66-Glass Fiber
   Reinforced", *Mech. Compos. Mater.* 59:1217–1222, 2024, accepted manuscript PDF pp. 3, 5, 9 (Fig. 3):
   https://hdl.handle.net/10115/107199. Springer accepted-manuscript terms; cite only, cached privately.
6. BASF, Ultramid A3K product information, 09/2026, p. 2: https://download.basf.com/p1/8a8082587fd4b608017fd64108ab6d3b/
   en/ULTRAMID_A3K_Product_Data_Sheet_Asia_PacificEurope_English.pdf. Copyrighted; cite only, cached.
7. E. Ghorbel, I. Hadriche, G. Casalino, N. Masmoudi, "Characterization of Thermo-Mechanical and Fracture Behaviors
   of Thermoplastic Polymers", *Materials* 7(1):375–398, 2014, §3.1 to §3.3, Figs. 9, 10, 12, 13, Tables 2 to 4.
   https://doi.org/10.3390/ma7010375 (PMC5453126). CC BY 3.0; cached (`ghorbel-pkg/`, `ghorbel-2014-*.xml`).
8. W. A. Kawahara, S. L. Brandon, J. S. Korellis, *Temperature, moisture, and strain rate effects on the compressive
   mechanical behavior of Nylon 6/6*, SAND88-8219, Sandia, 1988; abstract only: https://www.osti.gov/biblio/7006226.
9. Shimadzu, ASTM D638 specimen table (Type I: 57 mm parallel length, 115 mm between grips):
   https://www.shimadzu.com/an/products/materials-testing/uni-ttm-system/tensile-test-methods-for-plastics-astm-d638/
   spec.html. A vendor's restatement of the standard.
10. S. M. Walley, J. E. Field, N. A. Safford, paper comparing polymers' high strain rate compression at 300 K and
    100 K, *J. Phys. IV* 1(C3):C3-185–190, 1991, https://doi.org/10.1051/jp4:1991324. Not read.
11. Celanese, Zytel 101L NC010 data sheet, 2023, p. 2 (glass transition, notched impact):
    https://quickparts.com/wp-content/uploads/2023/04/zytel%C2%AE-101l-nc010-gb.pdf. Cite only; cached by the parent.
