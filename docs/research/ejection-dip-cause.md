# What makes a StratoLogger's deep one-sample dip at deployment

An earlier survey counted 7 of 17 public PerfectFlite logs dipping below −16 m for one or two samples at a
deployment, down to −1,503 m, while the same flights' backups, firing later, dip only a few meters
([public-logs-evidence-gaps](public-logs-evidence-gaps.md) §5; §5 below counts per firing instead). Is that charge
gas reaching the sensor, or the firing current pulling down the altimeter's supply? The answer sets how issue #725
compares ejection pulses and how the synthetic flights' ejection pulse (`noise::PULSE_M`) should be drawn.
Sources: the 17 StratoLogger logs and four other pairs already cached for that note, re-read sample by sample;
makers' manuals and datasheets; Fetter's black-powder chamber tests, as in [suite-field-tools](suite-field-tools.md).
Read 2026-10-28; no GPL code opened. **Inference** marks this note's own reasoning.

**In short.**

- **The deep dips at the first charge are most likely pressure in the bay (Inference).** In the four UND
  dual-StratoLogger flights whose apogee charge dipped below −16 m, the unit that was *not* firing, with no change
  in its battery reading, saw the same dip 0.09 to 0.16 s after the firing, as deep or deeper: −86, −184, −227 and
  −325 m against the firing unit's −51, −106, −114 and −317 m (0.5 to 3.4 kPa).
- **The firing current doesn't predict a dip.** The seven firings whose battery reading sagged most (2.4 to 9.2 V)
  dipped below −16 m twice; 12 of the 24 that sagged 1.9 V or less did. A supply that merely scales the sensor's
  reading can move it about ±21 to 24 m at most (the MS5611's ±2.5 mbar supply error), far short of −300 m.
- **What dips is the first charge into a closed airframe.** Undelayed drogue charges: 7 of 10 below −16 m (−22 to
  −1,503 m, 95% interval 35% to 93%). Backups 1 to 2 s later, into an airframe already open (**Inference**): 0 of 6
  (0 to −4.3 m, at most 46%). Mains: 7 of 15 (−23 to −100 m), and in the two flights whose mains fire 0.15 s apart,
  each dip shows only in the unit that fired. The mains stay unexplained.
- **The synthetic pulse is too shallow, and belongs to the bay.** `PULSE_M` dips −8 m then rises +16 m, scaled
  by a per-logger draw of 0 to 2 at every charge: at most −16 m (about 0.2 kPa), the bottom of the real
  first-charge range. Real first-charge dips are deeper, one or two samples, and shared by the bay.

## 1. The test: what the other altimeter reads

UND-ARC's MIT full-scale flights carry two StratoLogger SL100s: a primary with no apogee delay and a secondary with
a 1 s delay. Their flight counters advance in step and their temperatures agree within 1.3 °F at the drogue (1.7 °F
over a flight), so they share one bay (**Inference**). Pairs were aligned by each log's 50%-of-apogee crossing on
the way up (offsets −0.013 to +0.021 s), not by the dips. A dip is a reading's deviation from its ±0.5 s running
median, below −16 m; the step in pressure uses the US Standard Atmosphere at the header's ground elevation.
"Witness" is the unit not firing.

| Flight (file stem) | Firing unit: dip, samples, ΔV | Witness: dip, samples, delay | Witness ΔV | Step, kPa |
|---|---|---|---|---|
| `und23_..._2023_3_5_FRR` | −51 m, 2, −0.2 V | −86 m, 1, +0.16 s | 0.0 V | 0.48 / 0.82 |
| `und23_..._2__4-2-2023` | −106 m, 2, −0.6 V | −184 m, 2, +0.13 s | 0.0 V | 1.05 / 1.82 |
| `und23_..._NSLI_Launch_NASAFLIGHT` | −114 m, 2, −0.2 V | −227 m, 2, +0.09 s | 0.0 V | 1.16 / 2.32 |
| `und24_Full_Scale_Launch_1` | −317 m, 2, −0.4 V | −325 m, 2, +0.11 s | 0.0 V | 3.31 / 3.38 |
| `und24_..._PDF` | none (V to 0.00) | none | 0.0 V | none |
| same five, secondary's drogue +1 s | 0 to −4.3 m; ΔV to −2.6 V (`PDF` −7.7 V) | −0.3 to −4.3 m | ≤ 0.1 V | ≤ 0.04 |

- In `und24_Full_Scale_Launch_1` the three samples after firing read −44, −1,041 and −127 ft (primary) and −40,
  −1,066 and −134 ft (secondary), each against its own baseline; both recover within 0.20 s. Whatever the firing
  unit's own electronics add is under about 8 m there (**Inference**).
- In the other three flights the witness reads 1.7 to 2.0 times deeper than the firing unit, so no local term
  shows. After separation both units ring together, about 70 m peak to peak near 2 Hz for a second in
  `4-2-2023`: the bay's air settling as the airframe opens (**Inference**).
- In `und24_..._PDF` the primary's battery reading fell to 0.00 V at its drogue firing and neither unit dipped,
  then or at the backup. **Inference:** no gas reached the bay; the logs can't say whether the charges burned.
- Battery sag alone makes no dip: the `FRR` secondary's drogue firing sagged 2.6 V (9.4 to 6.8 V) and read −0.6 m.

**Other makers.** In `nate_SpacePort_24` (no license, cited only) the RRC3 logged its drogue event first, and
its altitude stayed flat through it; 0.95 s later, at the EasyMini's drogue firing, the EasyMini read −72 m and the
RRC3 −54 m on its next samples (logged steps 0.56 and 0.42 kPa). Bay-wide, but this descent holds other shared dips
of that size (−61 and −55 m at 89.3 s), and the first firing made none, against the UND pattern. Two pairs show a
dip in one unit only: a Blue Raven −88 m (two 50 Hz rows) beside a TeleMega reading −4.0 m
(`werocketry/Hyperion-II`, MIT), and one SG1.2 StratoLogger −224 m beside its partner's −0.6 m (ISS, no license).
The witness samples fall 0 to 36 ms from the dip as aligned, but the Hyperion pair's alignment is uncertain
(+0.05 s by the ascent crossing, +1.43 s by descent wiggles), so a missed short pulse there rests on that
alignment (**Inference**); neither bay layout is published.

## 2. What the makers say

- **PerfectFlite.** One battery runs the logic and fires the match; outputs are rated 10 A (SL100) or 5 A (CF),
  on for 1.0 s [1, pp. 10, 48], [2, pp. 10, 47]. The SL100 rides out a power loss of about 3 s [3]. Both bulkheads
  need "a good airtight seal" against ejection gas, and wires through them are to be sealed [1, pp. 7 to 8, 14]. The
  warranty
  excludes diaphragms broken by "ejection charge pressure/hot gasses" [1, p. 49]. Each log holds altitude,
  temperature and battery voltage at 20 samples/s [3]; no manual describes rejecting spikes in the logged data.
  PerfectFlite's 2001 FAQ, from the microAlt era, warns that a shared supply's "voltage glitches" while firing
  "could cause corrupted data" [4, p. 2].
- **Featherweight** says gas entering the bay leaves "misleading" recorded data, which its deployment logic
  filters [5, p. 8]. **Entacore** supports and recommends a separate pyro battery, and still warns that an unsealed
  bay gives "large spikes in the readings", adding that a fired charge usually shows a spike [6, pp. 45 to 46].
- **Missile Works** names both routes: over-pressure in an unsealed bay makes the RRC3 "malfunction" (p. 4), and
  firing demand can cause "power and processor brown-out" (p. 6), cured by a second battery [7]. **Eggtimer**
  discusses only brownouts [8]. **Altus Metrum** covers brownouts, says the altimeters must be protected from
  "ejection charge gasses", and its release notes filter "speed and acceleration spikes caused by ejection charge
  firing" from its maxima [9]. No maker gives a leak size, a logic-rail sag or a pressure error measured in firing.
- **Builders' reports (anecdote).** A Pnut and a MicroPeak in one model-rocket payload bay both dipped when the
  motor's charge fired, neither altimeter firing [10]; a TeleMetrum's "big negative spike" at apogee was put down
  by its owner, as an assumption, to a poorly sealed aft bulkhead [11]. Two Rocketry Forum threads on such spikes
  returned 403 and were not read.

## 3. Sizes: what a dip means, and what each cause can make

Overpressure that reads a height h lower than the true one (US Standard Atmosphere, sea-level site):

| Dip | at 300 m above ground | at 1,500 m above ground |
|---|---|---|
| −16 m | 0.19 kPa | 0.17 kPa |
| −300 m | 3.55 kPa | 3.16 kPa |
| −1,503 m | 18.9 kPa (2.74 psi) | 16.8 kPa (2.44 psi) |

- **Gas.** Fetter measured 3.58 psi gauge for one packed shot of 0.5 g in a 150.8 in³ chamber, 28.5 psi empty
  (mean of 7) [12, pp. 24, 48]. The deepest dip, −1,503 m in a subscale (an 18.5 kPa step at its height), needs the
  bay at about 75% of that packed peak (**Inference**). Choked flow from 297 kPa absolute (Fetter's empty-chamber
  mean) at 1,500 K for 2 to 5 ms into a 1 to 2.4 L bay needs about 0.5 to 3 mm² of effective leak for −16 m, 9 to
  50 mm² for −300 m and 46 to 280 mm² for −1,503 m, five times more if the gas cools in the bay (**Inference**, an
  orifice model with Cd 0.6; the area scales inversely with that pressure, and a packed bay's 3.58 psi gauge would
  not choke the flow). Either way it is a gap, not a weeping seal.
- **Supply.** The MS5611, which a NARCON 2017 paper found in a StratoLogger [13, Table 1], allows at most
  ±2.5 mbar over its 1.8 to 3.6 V supply range [14, p. 4]: about 21 m at 300 m and 24 m at 1,500 m. Only a corrupted
  conversion makes more: reading before a conversion ends returns 0 [14, p. 11], which with the datasheet's example
  coefficients is a spike *up*; an all-ones word gives a dip of thousands of meters (**Inference**). No ejection
  dip here lands on a repeated or rail value, on ground level or on −apogee.
- **Sampling.** An MS5611 conversion lasts 7.4 to 9.0 ms at its finest setting [14, p. 3], and averages over that
  window. At 20 Hz, a pulse τ ms long lands in a window with chance about (8 + τ)/50, and a short pulse reads below
  its peak (**Inference**). That fits one-sample dips, the witness reading deeper than the firing unit or the
  reverse, and the "local" pairs above. Whether the StratoLogger averages several conversions per sample is unknown.

## 4. Dips to exclude, and dips with no firing

- **Power-loss rows.** Five logs whose header reads `Apogee: PWRLOSS` end on one row of −1 ft, 31.98 °F and
  25.50 V (both `FRR` units, both `PDF` units, `iss_20230305_AL1`). A deviation scan reads it as a dip of −1,170 m
  or −293 m. It is a reset marker, not a pressure. Rows before it also carry 25.50 V (some with real altitudes),
  and 0.00 V rows precede them, so a comparison should drop everything from the first 25.50 V row.
- **Boost and landing.** Both units of a pair dip at landing (−26 and −111 m in `und24_Full_Scale_Launch_1`) and
  in boost (−19 to −63 m, `iss_20230305`). Two ISS booster logs dip −63 to −197 m at 2.5 to 3.4 s; in SG1.1 the
  temperature rises 2.7 °F within 0.7 s of the dip, and in SG1.2 both units see it: staging gas (**Inference**).
  But two of these three dips land near pad level (32 and 40 ft above ground), so a corrupted reading isn't ruled
  out there.
- **A dip before any firing.** `iss_20221029_stratologger` reads −462 m 1.35 s before its own drogue; its battery
  column reads 9.10 V on every row, so it says nothing. Its own charges then read about −4 m. Some other charge
  (**Inference**; no record says what).

## 5. A distribution for the synthetic pulse

From the 17 StratoLogger logs that carry an event header, 31 firings (one never-fired main and one unit with no
load on either output left out). Intervals are exact binomial 95% bounds.

| Firing | Count | Below −16 m | Depths below −16 m, m | Samples | Step, kPa |
|---|---|---|---|---|---|
| Drogue, no delay | 10 | 7 (35% to 93%) | −22, −51, −106, −114, −294, −317, −1,503 | 1, 2 | 0.2 to 18.5 |
| Drogue, 1 to 2 s delay | 6 | 0 (0% to 46%) | none (0 to −4.3 m) | none | ≤ 0.04 |
| Main, either unit | 15 | 7 (21% to 73%) | −23, −38, −39, −59, −63, −82, −100 | 1, 2 | 0.3 to 1.2 |

The witnesses of the four first charges read −86 to −325 m (0.8 to 3.4 kPa). The two subscale dips (−294 and
−1,503 m, 3.4 and 18.5 kPa) are deeper than the subscale was high, a reading below the pad.

## What this suggests

1. **Draw the pulse once per charge for the bay, not per logger.** Apply one overpressure to every barometer in
   the bay for one or two samples at 20 Hz: a chance near 0.7 at the first charge into a closed airframe, a step
   across 0.2 to 3.4 kPa (a log-uniform draw is a choice, not a measurement), 18.5 kPa seen in one subscale. Give
   delayed backups a dip rarely (0 of 6, at most 46%). Widening a range takes a new decision record citing these
   counts and a census change, as #725 states.
2. **Keep the mains' dips on the firing unit only, at about one in two (0.3 to 1.2 kPa),** until a dual flight with
   a known bay layout explains them.
3. **In #725's comparison, drop the power-loss tail** (from the first 25.50 V row) and compare first-charge dips
   separately from backups and mains, so a quiet backup doesn't count against a deep primary.
4. **Analyzer (idea):** a one- or two-sample dip at a firing that both altimeters in a bay see is most likely gas
   from an imperfect seal, a build note for the flyer; it should never move apogee, and a filter should reject it.

## Open questions

- Why does each main dip show only in the unit that fired, when the drogue dips reach both (two flights)? A bay
  drawing or photo from the UND teams would settle wiring against vents; none was found.
- Does a StratoLogger sample hold one conversion or an average? That sets how often a short pulse is caught.
- Which sensor the SL100 carries: the NARCON paper doesn't say whether it opened an SL100 or a CF.
- More dual flights whose second unit fires much later, to sort the mains by charge order. The two forum threads
  (Rocketry Forum 22040 and 46744) were unreadable here.

## Sources

Logs: the cache of [public-logs-evidence-gaps](public-logs-evidence-gaps.md) §5, UND-ARC/RocketModels-2023
@`922c084f` and UND-ARC/Nasa-SLI-2024 @`97bee76e` (MIT), werocketry/Hyperion-II @`efda132f` (MIT); the ISS and
`nate_` files carry no license and AerospaceNU's `anu_mbta_strat.txt` is GPL-3.0 data: all cited, not copied.
Documents below have no stated license unless noted.

- [1] PerfectFlite, *StratoLogger* (SL100) manual, 2011; [2] *StratoLoggerCF* manual, 2014; [3] *SL100 info*,
  2011; [4] *PF FAQs*, 2001. All under <http://www.perfectflite.com/Downloads/>.
- [5] Featherweight, *Blue Raven* manual, 2026-05-12. [6] Entacore, *AIM XTRA/BASE v4.x.x Manual*, 2025-02-02.
- [7] Missile Works, *RRC3+/RRC3 User Manual*, rev. 1.71. [8] Eggtimer, *Quantum* guide 1.09G.
- [9] Altus Metrum, *The Altus Metrum System* (CC BY-SA 3.0), §3.1 to 3.2, Installation, release notes.
- [10] B. Cooke, "Altimeter funny business", 2015-09-07,
  <https://billsrockets.blogspot.com/2015/09/altimeter-funny-business.html>.
- [11] B. Garbee, "TeleMetrum First Flight" (last edited 2013-07-03),
  <https://gag.com/bdale/blog/posts/TeleMetrum_First_Flight.html>.
- [12] Fetter, "Using Black Powder for Parachute Deployment", vNARCON 2025, Rev 1.2.
- [13] D. Wolf and M. Kuzel, "Development of a Digital Pressure Sensor Emulator for Altimeter Testing", NARCON
  2017, 2017-02-24, nar.org (Wayback 2024-05-11).
- [14] TE Connectivity, *MS5611-01BA03* datasheet, 06/2017.
