# How strong nylon shear pins are: material data and measured breaks

What force breaks a 2-56, 4-40 or 6-32 nylon 6/6 shear pin in single shear, how much does it spread, and how do
moisture, temperature, hole fit and loading rate move it? It serves the ejection-charge sizing (M6.7 in the
[roadmap](../ROADMAP.md)), which sizes a charge against "the shear pins' load at the high end of their strength", the
ground-test log (M6.8), and the shear-pin idea in [pad and recovery ideas](ideas-pad-and-recovery.md). It builds on
[suite web tools](suite-web-tools.md), which found that the Charge port sizes pins at nylon's minimum strength, and on
[suite field tools](suite-field-tools.md), which covers Fetter's 2025 ground tests.
Read 2026-10-17: resin makers' data sheets, thread-standard tables, one instrumented pin test series, three builders'
pull tests, one screw maker's catalog and two calculators. **Inference** marks this note's own reading. Trust the
areas (exact) and the printed values; the high end below is an inference from few tests, ±20% at best.

**In short.**
- **Fetter's pin force is a low-to-middle value, not a high end.** His Table 6-1 multiplies nylon 6/6's lowest
  data-sheet shear strength, 9,600 psi, by the thread's minor-diameter area: 31.0 / 49.8 / 75.0 lbf for 2-56 /
  4-40 / 6-32 (138 / 222 / 333 N). His "max" column keeps the minor area at 10,500 psi (33.9 / 54.5 / 82.0 lbf).
- **The only published upper bound is Feretich's:** 10,500 psi on the pitch-diameter area, 46 / 76 / 114 lbf
  (205 / 338 / 507 N), about 1.5 times Fetter's. No measured mean found exceeds it by more than 2% (one 6-32 report).
- **Measured breaks scatter widely around the data-sheet value.** The one instrumented series (Damerau, 2004, 36
  pulls) averages 21.4 lbf (#2) and 38.2 lbf (#4) per pin with three or four pins, 1σ 4% to 6%: 23% to 31% *below*
  9,600 psi × minor area. Builders' pulls (forum reports, weak evidence) read 28.5 to 35 lbf for 2-56 in a snug
  tapped joint, 56 lbf for 4-40 and 116 lbf for 6-32. The measured high end: about 35 / 56 / 116 lbf.
- **No source measures moisture, temperature or loading rate on a pin.** Resin data says a cold pin is stronger
  (×1.16 at 0 °C, dry), a fast break is stronger (+0% to +20%), and a damp pin is weaker (5% to 33%). The two that
  raise the force stack to about 64 / 106 / 160 lbf on the pitch area; that is a stack of separate figures.

## 1. What Fetter 2025 uses (Rev 1.2, 23 Feb 2025; printed page numbers, at the head of each page)

- "The shear strength of Nylon is between 9600 and 10500 psi" (§6, p. 18). Table 6-1 (p. 18), credited to
  Feretich, has columns Min 9,600 psi and Max 10,500 psi, both on the minor area: 2-56 0.0641 in (0.00323 in²),
  4-40 0.0813 in (0.00519 in²), 6-32 0.0997 in (0.00781 in²). Check: 0.003227 in² × 9,600 psi = 30.98 lbf.
- Every later sizing uses Min: Table 9-1 (p. 39), 61.9 lbf for two 2-56; Table 17-2 (p. 78), 149.5 lbf for three
  4-40 and 224.8 lbf for three 6-32. Eq. 16-1 (p. 69) multiplies the pin force by (1 + safety factor), "around 40%".
- **Two slips, one inherited misprint.** Eq. 6-1 (p. 18) names 2-56's minor diameter, 0.0641 in, but computes with
  M2's 0.05942 in and prints 26.62 lbf; the table gives 30.98. Fetter calls Damerau's #2 result "very close" to the M2
  row (p. 19), but Damerau's columns are #2 and #4: against 2-56 and 4-40 his two-pin means are 80% and 81% of Table
  6-1. Table 6-2's five #2 values average 24.96 lbf, not 24.64, a misprint copied from Damerau's page.
- Fetter [13] measures no pin force; his Table 6-2 reprints Damerau's. His ground tests bracket a charge, not a pin.

## 2. Material data (ASTM D732 punch shear, unfilled nylon 6/6; DuPont and Micro Plastics at 23 °C)

| Source | Grade | Dry as molded | 50% RH |
|---|---|---|---|
| DuPont design guide [3], Tables 7, 8 | Zytel 101 / 101L | 9,600 psi (66.2 MPa) | not given |
| DuPont design guide [3], Tables 7, 8 | Zytel 105 | 10,500 psi (72.4 MPa) | 10,000 psi (69.0 MPa) |
| Mitsubishi Nylon 101, extruded rod [6] | PA66 | 10,000 psi (condition unstated) | not given |
| Proto Labs machining sheet [5] | nylon 6/6 | 10,000 to 10,600 psi (condition unstated) | not given |
| Micro Plastics screw catalog [9], p. 167 | molded nylon 6/6 | 9,600 psi | not given |

- **Inference:** Feretich's 9,600 to 10,500 psi band is exactly DuPont's dry values for Zytel 101 and 105. The
  highest value found is 10,600 psi [5]. Current resin sheets (Zytel 101L [4], Vydyne 21SPC [8]) use ISO tests and
  give no shear strength.
- D732 shears sheets, plates or molded shapes with a punch and divides by punch circumference × thickness [16]. A
  threaded pin adds a notched root, bending in any hole clearance and a molded skin; ASTM's scope warns geometry and
  rate can move the result strongly. **Inference:** data-sheet values bound the resin, not the pin.

## 3. The area that shears (ASME B1.1, inches)

| Size | Major | Pitch | Minor 3A | Minor 2A | Stress area | Minor 3A / pitch area (in²) |
|---|---|---|---|---|---|---|
| 2-56 UNC | 0.0860 | 0.0744 | 0.0641 | 0.0635 | 0.00370 in² | 0.00323 / 0.00435 |
| 4-40 UNC | 0.1120 | 0.0958 | 0.0813 | 0.0805 | 0.00604 in² | 0.00519 / 0.00721 |
| 6-32 UNC | 0.1380 | 0.1177 | 0.0997 | 0.0989 | 0.00909 in² | 0.00781 / 0.01088 |

Major and minors from an ASME B1.1-1989 chart [10]; pitch diameters are the 2B basic minimums [11]; stress areas
from [12]. Fetter's minors are class 3A; a common class 2A screw has 1.6% to 2.0% less root area.

- **Feretich's rule** [2]: the minor area gives the minimum force; the pitch area the maximum, because a close fit
  forces the cut through part of the thread. He also warns against tapping both tube and coupler, which preloads
  the screw.
- **Inference:** any plane square to a single-start thread's axis cuts the same area, the mean of πr² over one
  pitch: 0.00447 / 0.00744 / 0.01124 in² for the basic UN profile, 3% above the pitch area. So the pitch area is a
  fair upper area for a square cut; a loose fit lets the pin bend and break along the root (the minor area).

## 4. Measured breaks

**Damerau, RocketMaterials.org, 2004** [1], the only instrumented series found: a carbon-fiber coupler in a 38 mm tube
(so, by this note's reading, one shear plane per screw), a 500 lb load cell at 20 in/min. Screws are labeled "#2" and
"#4" nylon; pitch, grade and maker are unstated. Per pin (total ÷ pin count), from the page's rows:

| Size, pins | Pulls | Mean per pin (lbf) | 1σ (lbf) | Range (lbf) | On the 3A minor area |
|---|---|---|---|---|---|
| #2, 2 pins | 5 | 25.0 | 1.5 | 23.0 to 26.6 | 7,740 psi |
| #2, 3 or 4 pins | 12 | 21.4 | 0.9 | 19.7 to 22.9 | 6,630 psi |
| #4, 2 pins | 5 | 40.4 | 1.6 | 38.0 to 42.6 | 7,780 psi |
| #4, 3 or 4 pins | 14 | 38.2 | 2.2 | 33.3 to 41.2 | 7,360 psi |

- Two pins let the joint "cock to one side causing a binding effect": +17% (#2) and +6% (#4) per pin over three or
  more. In a cardboard tube one pull each gave 11.4 (#2) and 20.2 lbf (#4) per pin; the #4 screws "began to tear
  up the tubing", which held up to the #2. A 3 × #6 pull, fixture unstated, gave 60.5 lbf per pin. Damerau:
  "There is not enough data to draw any conclusions."
- **Inference:** either the screws were not full-strength nylon 6/6, or the fit lets them bend; the page can't say.

**Builders' pull tests** (forum reports: weak evidence).
- A 2017 builder [14]: 2-56 in snug filament-wound fiberglass, outer tube tapped, a luggage scale (about ±2 lbf), one
  or two pulls per count: 28.5 to 35.0 lbf per pin over 1 to 4 pins (the 4-pin load estimated past the scale's top),
  33% to 64% above Damerau's 21.4.
- A 2022 builder [15]: a 6 in joint, pins drilled as flown, a crane scale and a 240 fps camera, breaking "within
  500ms": 2-56 18 lbf (range 7.5 to 31), 4-40 56 lbf, 6-32 116 lbf; counts and screw source unstated. He suspects
  his 6-32 screws were not nylon 6/6.
- **The folklore** "#2 ≈ 35 lbf, #4 ≈ 70 lbf" traces to a 2004 newsgroup reply [7]: dead weight on a phenolic tube,
  McMaster screws, no counts or spread.
- **Calculators:** HARA's sizing page [17] assumes 25 lbf (#2) and 40 lbf (#4), unsourced (**Inference:** Damerau's
  two-pin means). The Rocketry Forum's calculator [18] uses 9,500 psi × minor area (30 to 35, 45 to 50 lbf) and
  credits *Peak of Flight* #338, an issue about Kevlar shock cords that never mentions shear.

**Screw maker's data.** Micro Plastics [9], p. 166 (dry as molded, 73 °F, fixture and counts unstated), lists
**double-shear** breaks of 50 lbf (4-40) and 97 lbf (6-32), none for 2-56; per plane that is 25 and 48.5 lbf. Its
19 / 41 / 69 lbf column, which one builder took as a shear rating, is the *tensile* break. Black screws are dyed in
hot water and carry about 3% to 4% moisture against about 0.5% as molded (p. 168).

## 5. What moves the force: sign and size

| Cause | Sign | Size | Basis |
|---|---|---|---|
| Two pins against three or more | higher | +6% to +17% per pin | Damerau [1], measured |
| Snug tapped fiberglass against Damerau's fit | higher | +33% to +64% per pin, 2-56 | one builder [14] |
| Pitch-area cut against minor-area cut | higher | +35% to +39% by area | thread geometry [2], [10] |
| Cold, dry: 0 °C / −10 °C against 23 °C | higher | ×1.16 / ×1.22 (yield) | Zytel 101 Fig. 4 [3] |
| Fast loading: 1,000 to 3,000 vs 2 to 10 in/min | higher | +13% to +21% (yield); carried as +0% to +20% | Fig. 5 [3] |
| Moisture: 50% RH against dry as molded | lower | −5% (shear, Zytel 105) to −33% (yield) | [3], [4], [8] |
| Hot pad: 50 °C against 23 °C | lower | ×0.75 to ×0.79 (yield) | Fig. 4 [3] |
| Soft (cardboard) tube | lower joint force | −47% per pin, one pull each; the #4 tore it | Damerau [1] |

- **Inference:** which test rate matches a pin broken by a charge in milliseconds is unknown; Fig. 5 gives no
  specimen length to turn crosshead speed into strain rate, hence the +0% floor.
- The temperature, rate and moisture rows are resin yield in tension, read from DuPont's plots by pixel scan
  (±1 to ±3 MPa). **Inference:** applying them to a pin's shear assumes shear scales with yield; no source shows it,
  and the one conditioned shear pair (Zytel 105, −5%) falls far less than yield does (−23% to −33%).
- A pin at apogee is cold only if it has cooled: **Inference:** in a closed airframe a pin may sit near pad
  temperature through a short flight. No source measures a pin's temperature in flight.
- PA66 takes about 2.5% water at 50% RH and about 8.5% saturated, slowly (125 days for a 1.5 mm plate at 50% RH,
  [3] p. 2). **Inference:** a pin sits between dry and conditioned depending on storage, and a dyed black pin
  starts conditioned.

## 6. The force per pin (single shear, F = τ × A; 1 lbf = 4.448 N)

| Case | 2-56 | 4-40 | 6-32 |
|---|---|---|---|
| Damerau's measured mean, 3 or 4 pins | 21.4 lbf / 95 N | 38.2 lbf / 170 N | 60.5 lbf, one pull |
| Fetter (Min): 9,600 psi × 3A minor | 31.0 lbf / 138 N | 49.8 lbf / 222 N | 75.0 lbf / 333 N |
| Measured high (one builder each) | 35 lbf / 156 N | 56 lbf / 249 N | 116 lbf / 516 N |
| Feretich's max: 10,500 psi × pitch | 46 lbf / 205 N | 76 lbf / 338 N | 114 lbf / 507 N |
| Dry, 0 °C, fast: 14,750 psi × pitch | 64 lbf / 285 N | 106 lbf / 473 N | 160 lbf / 714 N |

Check for 2-56: π/4 × 0.0744² = 0.004347 in²; × 10,500 psi = 45.6 lbf, which Feretich rounds to 46. The last row is
10,600 psi × 1.16 (0 °C) × 1.2 (rate) = 14,750 psi; it stacks factors read from separate figures.

## What this suggests

- **M6.7's high end could be Feretich's pitch-area bound,** 46 / 76 / 114 lbf per pin, cited as [2] and reported
  with the measured breaks of §4. It sits 1.5 times above Fetter's 9,600 psi row, which errs toward too little
  powder for a strong pin. The one 6-32 report (116 lbf, count unstated) sits on it, so it leaves no margin for 6-32.
- **Say the spread, not one number.** From Damerau's lowest to the builders' highest, per-pin breaks span 0.6 to
  1.1 times Fetter's row for 2-56 and 4-40 (one builder's 2-56 low, 7.5 lbf, is 0.24 times); the model can't
  predict a given builder's pins within that.
- **M6.8's ground-test log could record** pin count, size, maker, color (black is pre-moistened), tapped or drilled,
  and tube material: each moves the force by the sizes in §5.
- **Fetter's slips** (§1) matter if M6.7 cites Table 6-1 or Eq. 6-1; cite the table's 30.98 lbf, not 26.62.
- **A measured series would settle the high end:** 2-56, 4-40 and 6-32 from two makers, dry and conditioned, at
  23 °C and cold, in a fiberglass joint with three pins, at a slow and a fast rate.

## Open questions

- No paired pin test of loading rate, cold soak or humidity was found; all three are unbounded on pins.
- Damerau's fit, screw maker and thread pitch are unstated, so his 23% to 31% shortfall is unexplained.
- Whether the Micro Plastics double-shear rating is per joint or per plane: its fixture isn't described.
- Six searches of Student Launch and Spaceport America Cup reports found ejection ground tests but no pin force.
- No published shear strength at 50% RH for Zytel 101, and no shear against temperature for unfilled nylon 6/6.

## Sources

Read 2026-10-17; all cite only (copyrighted, no license). RF is The Rocketry Forum, rocketryforum.com/threads/.

1. D. Damerau, "Shear Pins", RocketMaterials.org, 2004, archive copy: https://web.archive.org/web/20150218234304/
   http://www.rocketmaterials.org/datastore/cord/Shear_Pins/index.php
2. B. Feretich, "Shear Pin/Screw Calculations" (live page gone; archive of 2026-04-17): https://web.archive.org/web/
   20260417090259/http://feretich.com/Rocketry/Resources/shearPins.html
3. DuPont, Zytel and Minlon Design Guide, Module II (1996-97), Table 7 (p. 9), Table 8 (p. 19), Figs. 4, 5 (p. 31):
   http://foremostplastic.com/wp-content/uploads/2015/04/DuPont-Module-II-Nylon.pdf
4. Celanese, Zytel 101L NC010 data sheet, 2023:
   https://quickparts.com/wp-content/uploads/2023/04/zytel%C2%AE-101l-nc010-gb.pdf
5. Proto Labs, nylon 6/6 CNC material sheet, 2026:
   https://www.protolabs.com/media/slyigq1s/2026-026-cnc-material-data-sheet_nylon-6-6.pdf
6. Mitsubishi Chemical Advanced Materials, Nylon 101 data sheet: https://polymershapes.com/wp-content/uploads/2020/04/
   Polymershapes_MitsubishiChemicalAdvancedMaterials_DataSheet-Nylon-101.pdf
7. "Rocket Materials.com Test Help - Shear Pin Test", newsgroup thread, May 2004:
   https://www.polytechforum.com/rockets/rocket-materials-com-test-help-shear-pin-test-42840-.htm
8. Ascend, Vydyne 21SPC data sheet: https://essentracomponents.bynder.com/m/58e0a44e0f6e117b/original/1220192-pdf.pdf
9. Micro Plastics catalog, pp. 166 to 168: https://www.farnell.com/datasheets/3153494.pdf
10. Engineers Edge, screw thread chart per ASME B1.1-1989: https://www.engineersedge.com/screw_threads_chart.htm
11. American Fastener, internal threads class 2B: https://www.americanfastener.com/wp-content/uploads/2020/06/
    NUTS-INTERNAL-THREADS-%E2%80%93-CLASS-2B-AMERICAN-FASTENER.pdf
12. Allied Titanium, UNC thread table: https://ns2.alliedtitanium.com/technical/threads/unc/UNC_Threads.pdf
13. T. Fetter, *Using Black Powder for Parachute Deployment*, Rev 1.2, 2025, pp. 18, 19, 39, 69, 78.
14. RF, Apr 2017: https://www.rocketryforum.com/threads/shear-pin-shear-force-higher-than-expected.140272/
15. RF, Nov 2022, posts 1 and 7: https://www.rocketryforum.com/threads/nylon-shear-pin-testing.176101/
16. ASTM D732-24, scope page: https://store.astm.org/d0732-24.html
17. HARA, "How to size an ejection charge": http://www.hararocketry.org/hara/resources/how-to-size-ejection-charge/
18. RF ejection charge calculator: https://www.rocketryforum.com/rocket-calculators/ejection-charge/
