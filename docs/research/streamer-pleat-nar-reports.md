# What the NAR R&D streamer reports measured: flat against pleated

HPR Sim's default streamer model (Carruthers and Filippone's curves, ADR-013) predicts Kidwell's one flat streamer
+9% fast, but his pleated Micafilm streamer +58% fast, because HPR Sim models no pleats
([known risks](known-risks.md), [streamer and tumble drag](streamer-and-tumble-drag.md)). [NAR R&D
reports](nar-rd-reports.md) named three members-only reports as the leads for that gap. This note reads them: what
each measured, what drag coefficient (C_D, on the one-side planform `S = w·l`) the numbers imply, and whether they
explain the +58%. "Inference" marks this note's own arithmetic; the scripts are kept privately with the PDFs.

**In short: pleats multiply a streamer's drag by about 2 to 4, so +58% fast is the normal size of the miss.**

- **One direct flat-against-pleated test.** Stenberg (NARAM-52) dropped a rolled and a pleated 5 × 50 in streamer
  together from an RC glider: 17.06 against 7.47 ft/s, 2.28 times in speed and **5.2 times in drag** at equal mass.
  Kidwell's pleated Micafilm against his flat crepe is 2.18 times in drag, with the material changed too.
- **Pleated streamers carry 1.9 to 4.4 times the default's drag.** Stenberg's 14 pleated rates (2 mil left out) imply
  C_D 0.25 to 0.57 (**inference**), median 0.33, against the default's 0.12 to 0.15. Kidwell's pleated 0.338 sits 3%
  above that median. Flanigan's tunnel (NARAM-58) bounds 1 mil Mylar with 1 in accordion folds at C_D above about 0.38
  (**inference**), about 3 times the default. Unpleated streamers stay near the curve: Stenberg's rolled one reads
  −17%, Kidwell's crepe +19%.
- **No report sizes the pleat itself.** Flanigan measured no force and flew no flat streamer; Estrada (NARAM-58) has
  six flight durations with no altitude. Stenberg's flat case is one rolled streamer that may not have opened fully.

Confidence: high on what each report printed (Stenberg's figures read off the scanned pages); medium on the pleated
C_D band (no scatter, drop counts or air motion reported); low on any one pleat factor.

## 1. Stenberg, "Updated Streamer Testing Using High Altitude Drop Method" (NARAM-52)

**Title.** The cover (PDF p. 1) gives this title. NAR's index repeats another report's title for id `1358230`,
and its 2015 bibliography gives "Streamer Optimization Testing with High Altitude Drop"
([NAR R&D reports](nar-rd-reports.md)); neither is on the file. No page prints a year: 2010 is NARAM-52's year.
Division A.

**Method** (pp. 2 to 8). An RC glider carried two or three streamers to about 500 ft and released them together;
the pilot dove at once, so the onboard altimeter's peak marks the release height. Each test summed drops to at
least 1,000 ft, and each table prints a "total amount of drop" of 1,305 to 3,668 ft. Rates are printed as "feet
dropped per second", one per streamer (**inference:** total height over total time). There are no per-drop rows,
drop counts, scatter or timing uncertainty. A streamer that failed to open was refolded and dropped again.

**Streamers** (pp. 2, 3, 6, 8). All 250 in² (0.161 m²); 5 in wide at 10:1. Each hung from motor casings and clay
trimmed to 21.5 g. Materials: 1 and 2 mil Mylar and Fibafilm. Every streamer but the rolled one was accordion-folded
in ½ in pleats over 90% of its length. Tests 1 and 3 don't name the material; "many" tests used Fibafilm, so the
C_D below take them as Fibafilm (1 mil Mylar instead lowers them about 4%).

| Test (page) | Streamers and rates, ft/s | Total drop, ft |
| --- | --- | ---: |
| 1, rolled against folded, 5×50 (p. 9) | rolled 17.06, folded 7.47 | 3,668 |
| 2a, thickness (p. 10) | 1 mil Mylar 10.27, 2 mil Mylar 14.13 | 1,305 |
| 2b, material (p. 10) | 1 mil Mylar 11.14, Fibafilm 10.27 | 2,502 |
| 3a, length to width (p. 11) | 10:1 9.72, 11:1 9.83, 12:1 9.89 | 1,585 |
| 3b, length to width (p. 11) | 10:1 8.36, 9:1 8.83, 8:1 8.61 | 1,897 |
| 3c, length to width (p. 11) | 12:1 9.30, 8:1 9.99 | 2,194 |
| 4, fatigue (p. 13) | fresh 10.22, well used 10.59 | 2,923 |

**Implied C_D (inference).** `C_D = 2mg/(ρ v_t² S)`, with m the 21.5 g weight plus the streamer: 6.8 g of Fibafilm at
42 g/m² (a builder's covering table; Kidwell's Micafilm is 42.1 g/m², 4.34 g on 4 × 40 in in his Table 1), or 5.7 g of
1 mil Mylar at 35.4 g/m² (DuPont Teijin's 1.395 g/cm³). ρ = 1.15 kg/m³, as the site and weather are unstated; 1.10 to
1.225 moves C_D by about −6% to +5%. Over 152 m the mean speed is 98.8% (rolled) to 99.8% of terminal, and the script
corrects for the rest. The default ("CF") is evaluated as shipped: at 0.161 m² it is clamped to its 0.075 m² end
curve, so every CF figure here is an extrapolation.

| Case | C_D | CF C_D | CF predicts | C.6 predicts |
| --- | --- | ---: | ---: | ---: |
| Rolled, 10:1 (T1) | 0.108 | 0.130 | −9% | +63% |
| Folded 10:1, five tests (T1, T2b, T3a, T3b, T4) | 0.302 to 0.574 | 0.130 | +52% to +109% | +171% to +272% |
| Folded 1 mil Mylar (T2a, T2b) | 0.291, 0.247 | 0.130 | +49%, +37% | +179%, +157% |
| Folded 8:1 to 12:1 (T3) | 0.320 to 0.431 | 0.119 to 0.145 | +48% to +75% | +172% to +215% |
| Folded 2 mil Mylar (T2a) | 0.184 | 0.130 | +19% | +79% |

"Predicts" is the predicted descent speed against the measured one, positive meaning too fast. C.6 is OpenRocket's
streamer fit, kept in HPR Sim as the `OpenRocket` option. Leaving the streamer's own mass out lowers each C_D by 20 to
24% (35% for the 2 mil streamer). The 2 mil streamer "did not seem to unfold that much" (p. 9), which fits its low
C_D.

**What the numbers don't support.**

- **The rolled streamer is a weak flat control.** It "just fluttered" and fell fast (p. 9); a roll that never fully
  opens has less drag than an open flat strip, so 5.2 may overstate the pleat effect. The report can't bound this.
- **Test-to-test scatter is about 2 times in drag.** The 10:1 folded streamer reads 7.47 to 10.27 ft/s across tests,
  1.38 times in speed, 1.9 in drag, larger than any difference inside Test 3. Her "10:1 is best" rests on single
  numbers. Her direct 12:1 against 8:1 drop (9.30 against 9.99 ft/s) contradicts her "wider is better" (p. 14).
- **No length-to-width trend.** CF says 12:1 should fall 10.5% faster than 8:1 at equal area; Test 3c has it 7%
  slower. **Inference:** the flat-strip exponent shouldn't be carried over to pleated streamers.
- **One repeated figure.** 10.27 ft/s is printed for 1 mil Mylar in 2a and for Fibafilm in 2b (p. 10).
- **Air motion and the altimeter are unbounded.** Sink or lift of ±0.25 m/s moves Test 1's folded C_D by −19% to
  +26%; a release-height error moves C_D by twice its fraction.

## 2. Flanigan, "Flight and Wind Tunnel Testing of Streamers" (NARAM-58, 2016)

Page numbers are the printed ones (PDF page minus 4). The cover's date reads "July 23-39, 2016".

**Flights** (pp. 14 to 17). A D12 carrier ejected a 3 in BT-20 capsule holding a Pnut barometric altimeter under
the streamer. Three flights flew one 4.5 × ~34 in 1 mil aluminized Mylar streamer with ½ in Z folds (p. 14); two
flew three 1 mil Mylar strips, 3, 4.5 and 6 × 50 in, whose folds aren't stated (p. 15; the photo on p. 16 appears to
show pleats). Apogees read off the plots were 463 to 628 ft (the text prints 580 to 620 ft for the single
streamer). Mean rates, read off the vector plots on pp. 15 and 17 (landing times
match Table 5.2-1 within 0.3 to 1.1 s), are 1.67 to 4.09 m/s, so every fall reaches terminal speed.

**The flights can't size the gap: the descent mass isn't given.** The Pnut alone is 7.4 g with battery (its maker's
manual), a floor. **Inference:** at ρ 1.25 kg/m³ and the tunnel strips' 39.2 g/m², the median of nine implied C_D over
CF is 1.52 (0.72 to 3.07) at 7.4 g, 1.99 at an assumed 12 g and 2.47 at 16 g (about 3% higher at 1.21). Even at the
floor, 7 of 9 descents show more drag than CF. One streamer's implied C_D varies about 2 times between flights; the
author blames wind and thermals (pp. 17, 25).

**Tunnel** (pp. 18 to 24). Vertical, 24 × 24 in section, a room fan pulling air from above; a Kestrel 4000 read
1.3 m/s with the tunnel vertical, 2.3 m/s horizontal, for reasons the author doesn't know (p. 21). Four streamers
were tested at 1.3 m/s only, with no force gauge; he expects 2 to 10 g of force (p. 24). Table 7.1-1 (p. 23)
records each strip's areal density (its "gr" read as grams) and whether the strip held up its own weight:

| Streamer | Areal density, g/in² | Held its weight at 1.3 m/s | C_D bound (inference) |
| --- | ---: | --- | --- |
| Micafilm, 1 in accordion | 0.0316 | no | below 0.47 |
| 1 mil Mylar, 1 in accordion | 0.0256 | yes, with margin | above 0.38 |
| 1 mil Mylar, "scorpion" (rolled, baked) | 0.0250 | yes, with margin | above 0.37 |
| Party banner, ~4 mm accordion | 0.0128 | "mixed" | none |

**Inference:** a tethered strip that holds itself up has drag at least its weight, so `C_D ≥ 2 ρ_m g/(ρ v²)`, whatever
its size. At ρ 1.20 kg/m³ the accordion Mylar bound is 0.38 (0.35 to 0.43 over ρ 1.15 to 1.25 and the Kestrel's ±3% of
reading, read off a garbled mirror of its manual and not confirmed; a resolution floor would widen it): 2.96 times
CF's 0.130 at its shape, 2.48 times Kidwell's flat crepe. The text calls the Micafilm strip 4 × 40 in; the table's
7.9 g and 0.0316 g/in² only agree at 5 × 50. Caveats: one speed, below Kidwell's 2.0 to 2.9 m/s; strips drifted into the
wall boundary layer until a centering device held their base (p. 24).

**Worked examples.** On p. 10 the C.6 example prints C_D 0.037 beside 0.042 N, which implies 0.025; the p. 9 skin
friction of 0.01 N computes to 0.017 N with both sides wetted. Neither is used above.

## 3. Estrada, "Streamer Pleats" (NARAM-58, Division A)

Three 1 mil Mylar streamers, 1¾ × 25 in, pleated every ½, 1 and 1½ in, each with factory ripples of about ⅛ in
(pp. 2 to 5). Flown on small kit rockets with A8-3 motors in 10 to 12 mph wind, two flights each; rocket mass,
apogee and deployment height aren't given, and there is no flat streamer (pp. 1 to 5). Flight times
(p. 5): ½ in 16.8 and 17.2 s; 1 in 18.6 and 19.4 s; 1½ in 19.3 and 20.8 s. **The printed 1½ in average, 19.7 s,
doesn't match its rows (20.05 s)**, so the text's margins are 1.05 and 3.05 s, not 0.7 and "almost 3".
**Inference:** 1½ in against ½ in gives Welch t 3.9 on about one degree of freedom (two-sided p near 0.15). If both
deployed at one height, C_D rises 1.4 to 1.5 times from ½ to 1½ in spacing. It gives no C_D and no flat case.

## 4. Does this explain the +58% on Kidwell's drops?

**Yes in sign and size, no as a measured factor (inference).**

- **The sign is settled.** Two reports beside Kidwell's, by different authors and three methods (glider drops,
  altimeter flights, a tunnel weight balance), find pleated streamers well above CF's drag; Estrada's gives no drag
  figure. Only Flanigan's flights at the impossible 7.4 g floor fall below CF (2 of 9).
- **The size fits.** +58% in speed is a C_D 2.5 times CF's. Stenberg's 14 pleated rates span 1.9 to 4.4 times CF's
  C_D (median 2.6) and Flanigan's tunnel bound is about 3 (2.7 to 3.3); Kidwell's Micafilm (2.6) is in the middle.
- **The flat side looks close, on thin evidence.** Stenberg's rolled streamer at −9% and Kidwell's crepe at +9%
  bracket CF, even clamped outside its fitted areas; that is two cases, one perhaps never fully open.
- **What stays unknown.** The pleat factor itself: 2.2 (Kidwell, material confounded) or 5.2 (Stenberg, one rolled
  streamer) as the only ratios, a 2 times test-to-test scatter, and pleat spacing (½ to 1½ in) moving C_D perhaps 1.4
  times. A single multiplier would carry a range of about 2 to 4.

**Which way the miss errs (inference).** A model that ignores pleats predicts a pleated streamer landing too fast,
+37% to +109% on Stenberg's drops, so its descent time and drift in a steady wind read 27% to 52% short. The landing
speed reads high; the drift reads short, which is the flattering side.

## What this suggests

- Replace known-risks' "+58% on Kidwell's" with the three-report band: unmodeled pleats read +37% to +109% fast on
  Stenberg's pleated drops, +58% on Kidwell's, and the default C_D is about 3 times low in Flanigan's tunnel.
- Add Stenberg's rolled and folded Test 1 and the 10:1 folded series beside the Kidwell replay
  (`streamer_models_against_kidwells_drop_tests`), so the pleat gap is pinned on more than one streamer.
- A "pleated" streamer option would need a cited multiplier with its range, about 2 to 4 on C_D, and should say
  that drift is flattered without it; the length-to-width exponent shouldn't apply to pleated strips.
- C.6 (`OpenRocket`) misses widely here, +63% fast flat and +157% to +272% fast pleated; its option text could say so.

## Open questions

- A force measurement on one streamer, flat and pleated, in one material at 2 to 4 m/s. None of the three reports
  has it; Flanigan's planned force gauge (p. 24) is the nearest lead.
- Stenberg's drop counts and per-drop scatter, the release-height accuracy, and air motion on the drop days.
- Flanigan's capsule mass, without which his nine descents give no C_D better than about 2 times.
- Whether pleat spacing matters at the 1.4 times Estrada's two flights per spacing hint at.

## Sources

1. A. Stenberg, "Updated Streamer Testing Using High Altitude Drop Method", NAR R&D, NARAM-52, docs.ashx id
   `1358230`, NAR members' download, read 2026-11-03 from the scanned PDF, pp. 1 to 15. Cite only.
2. C. Flanigan, "Flight and Wind Tunnel Testing of Streamers", NAR R&D, NARAM-58 (2016), docs.ashx id `1358319`,
   NAR members' download, read 2026-11-03, whole report. Cite only.
3. R. Estrada, "Streamer Pleats", NAR R&D, NARAM-58, docs.ashx id `1358330`, NAR members' download, read
   2026-11-03, pp. 1 to 6. Cite only.
4. C. Kidwell, "Streamer Duration Optimization: Material and Length-to-Width Ratio", NAR R&D, NARAM-43 (2001),
   Table 1 (Micafilm masses). His implied C_D (0.155 flat, 0.338 pleated) and the CF and C.6 forms are from
   [streamer and tumble drag](streamer-and-tumble-drag.md).
5. PerfectFlite Pnut altimeter manual, French edition by Planète Sciences, p. 11,
   <https://planete-sciences.org/espace/IMG/pdf/notice_altimetre_pnut_fr_v1.pdf>, read 2026-11-03 (7.4 g with
   battery). Cite only.
6. DuPont Teijin Films, Mylar ADS datasheet,
   <https://polymerfilms.com/wp-content/uploads/2023/09/MylarADS-Datasheet.pdf>, p. 1, read 2026-11-03
   (density 1.395 g/cm³). Cite only.
7. pldaniels.com, "RC model aircraft covering materials", Wayback copy of 2026-01-14 of
   <https://pldaniels.com/flying/balsa/rc-model-aircraft-covering-materials.html>, read 2026-11-03 (Fibafilm
   42 g/m²). A builder's page, cite only.
8. Kestrel 4000 manual, specification page, ManualsLib mirror,
   <https://www.manualslib.com/manual/1048402/Kestrel-4000.html?page=4>, read 2026-11-03 (wind speed ±3% of
   reading; the table text is garbled, so unconfirmed). Secondary, cite only.
