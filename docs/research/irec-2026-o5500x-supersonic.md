# IREC 2026's three highest O5500X flights: apogee-only candidates past Mach 1

**In short: the three highest-scoring 2026 30k COTS flights on the AeroTech O5500X-PS passed Mach 1 by the bound of
[IREC 2025's COTS reports][i25], and two of them could be flown apogee-only from their reports alone.** Teams 216
(Mississippi State, Bandit), 202 (UTEP, Odyssey) and 213 (Chulalongkorn, CURSR-V) scored 29,518, 29,714 and 30,683
ft [E1]. Their top speed was at least 382, 384 and 390 m/s with no drag at all: Mach 1.07 or more on a 35 °C day,
even with a 4.5 s burn (**Inference**, §2). Bandit (no air brakes; full shape, CG, CP and mass; predicted Mach 2.0
to 2.1) is the cleanest. Odyssey has no brakes and most of its shape, but no CG or CP. CURSR-V's report is the most
complete, but its active air brakes mean its apogee tests drag only if they stayed shut. The two brake-free flights
would join the candidates [#724][i724] already holds for [M2.3c7][m2-3c7], both in the core band (Mach 0 to 2.5).
**Confidence:** high for the apogees and the bound; medium for the terms (216 is marked proprietary, §7) and the
design numbers, the teams' pre-flight values with conflicts (§4); low for every top Mach (one claimed after the
flight, method unstated). **What it moves:** M2.3c7's floor of 5 scored COTS flights past Mach 1, and
`accuracy.supersonic`, through [issue #724][i724]'s apogee-only route.

**What this covers.** The three reports in full [T202, T213, T216], the scoresheet, rules and ESRA's page [E1 to E3]
and three news items [N1 to N3], read 2026-11-05; no GPL or AGPL source opened, nothing flown. Pages are the PDFs'.

[i25]: irec-2025-cots-reports.md
[vv]: vayuvega-supersonic-cots.md
[pf26]: public-flights-2026.md
[rcf]: rasaero-comparison-flights.md
[m2-3c7]: ../decisions-and-roadmap.md#m2-3c7
[i724]: https://github.com/nrdptel/fusionspace-eridanus/issues/724

## 1. The three flights and the programs behind them

| Team | School, rocket | Scored ft | Damage | Report's prediction (tool) | Scored vs predicted |
|---|---|---|---|---|---|
| 216 | Mississippi State, Bandit | 29,518 | Minor | 29,872 ft (RASAero II) | −1.19% |
| 202 | UTEP, Odyssey | 29,714 | None | 30,757 ft (OpenRocket) | −3.39% |
| 213 | Chulalongkorn, CURSR-V | 30,683 | None | 28,813 ft (not named) | +6.49% |

- **Scores.** Rows 1, 3 and 14 of the scoresheet, PDF p. 1 [E1]. Each row's apogee points match the rules'
  formula, 350 − 350·|30,000 − h| / 9,000 (§2.8.1.4, p. 30 [E2]): 331.26, 338.88 and 323.44 (**Inference**,
  recomputed). So the scored number is the height scored against the 30,000 ft AGL target.
- **No prediction column** among the sheet's 16 (counted): the table uses the reports' (**Inference**).
- **Same programs, new rockets, new team numbers.** 2026's team 216 is not 2025's team 216 (Melbourne). It is
  Mississippi State, 2025's team 208, whose Helios won the class; Bandit descends from it ([T216] pp. 6, 28).
  UTEP flew as 2025's team 218 (Sandstorm), and Chulalongkorn as 2025's team 204 (CURSR-IV) ([i25] §2).
- **All three designs changed.** Bandit: 139 in, 64.83 lbf; Helios: 140 in, about 69 lb ([T216] p. 7; [i25] §5).
  Odyssey: 131 mm, not 157 mm ([T202] p. 4). CURSR-V: one 131 mm body, not 157 to 127 mm ([T213] pp. 12, 33).

## 2. All three passed Mach 1, whatever the drag

The bound is [the 2025 note's][i25] §3, unchanged. While the motor burns, the rocket climbs at most v_max·t_b.
After burnout, it slows at least as fast as gravity alone would slow it, so the coast adds at most
v_max²/(2g). Hence h ≤ v_max·t_b + v_max²/(2g), and v_max is at least that equation's positive root. Drag, brakes
and tilt only lower h for a given v_max. Inputs: t_b = 4.0 s, the burn every report gives ([T216] p. 29;
[T202] p. 26; [T213] p. 33), g = 9.807 m/s², and the scored apogee as height above the pad. Sound runs 340.3 m/s
at 15 °C and 351.9 m/s at 35 °C; Mach values are rounded down, since they are lower bounds (**Inference**).

| Team | Apogee (m) | v_max ≥ (m/s), 4.0 s | Mach ≥, 15 °C | Mach ≥, 35 °C | Mach ≥, 35 °C and 4.5 s |
|---|---|---|---|---|---|
| 216 | 8,997 | 382.7 | 1.12 | 1.08 | 1.07 |
| 202 | 9,057 | 384.1 | 1.12 | 1.09 | 1.07 |
| 213 | 9,352 | 390.9 | 1.14 | 1.11 | 1.09 |

Worked example, 216: 29,518 ft is 8,997 m; v²/19.61 + 4.0 v = 8,997 gives v = 382.7 m/s, or Mach 1.087 at
351.9 m/s. With a 4.5 s burn the root is 378.3 m/s, which only air near 83 °C would hold below Mach 1. The bound
fails only if the scored figure is not height above the pad, or the altimeter over-read by about 14% or more
(216's case; the others need more) (**Inference**).

## 3. What could stop a scored apogee from testing drag

- **Air brakes: only 213.** CURSR-V carries a second-generation brake that holds the coast to a prescribed
  apogee ([T213] p. 17). It may open only after 20 s, below Mach 0.75, above 6,000 m AGL, with pitch under 30°
  and supply above 4.8 V (Table 6, p. 18). Searches of the other two reports for air brake, airbrake, drag flap
  or apogee control found no hit (counted: 0 each).
- **213's apogee sits high.** It is 6.49% above the report's 28,813 ft and 2.1 standard deviations above its
  10,000-run mean of 28,470 ± 1,034 ft ([T213] pp. 13, 17). A brake can only lower apogee, so if it opened, the
  passive flight would have gone higher still (**Inference**). Which happened is unpublished.
- **Staging: none.** 216 and 213 state one stage ([T216] p. 29; [T213] pp. 9, 33); 202 shows one motor (p. 5).
- **216's rebuild.** Bandit I broke up on its only test flight, on 28 March 2026, blamed on wind shear; the team
  rebuilt it with a commercial motor tube in place of its own composite one ([T216] pp. 28, 32). The report,
  written after the rebuild, gives the rebuilt rocket's numbers; the as-flown mass is unconfirmed.
- **202's conflicts.** Its conclusion predicts 5,700 m "(25,708 ft)", two numbers that disagree, and neither is
  its 30,757 ft elsewhere ([T202] pp. 5, 24). 25,708 ft is also in UTEP's 2025 report ([i25] Open questions),
  so it looks carried over (**Inference**). UTEP says Odyssey flew twice before IREC [N2]; the report shows one
  test, on an M4500ST, to 7,839 ft baro ([T202] p. 55).
- **The scoring altimeter.** The rules make the first COTS barometric altimeter the team connects to or listens
  to the official one (§2.5, pp. 13 to 14 [E2]). 216 flew two Blue Ravens ([T216] pp. 19, 30); 202 a TeleMega
  and a Blue Raven (p. 19); 213 a TeleMega V7 beside its own board (pp. 22, 34). No source says which unit was
  read or a second unit's apogee; the news repeats the scoresheet [N1 to N3].

## 4. Design inputs, team by team

Pre-flight values as stated; SI conversions and sums are **Inference**.

| Input | 216 Bandit | 202 Odyssey | 213 CURSR-V |
|---|---|---|---|
| Diameter | 6.052 in (p. 29) | 5.15 in (pp. 4, 31) | 131 mm (p. 33) |
| Length | 139 in (p. 29) | 3.14 m (p. 4) | 3,097 mm (p. 33) |
| Nose | 5:1 von Kármán, 35.52 in (pp. 72, 73) | 4:1 tangent ogive, 20.7 in (p. 30) | von Kármán, 695.4 mm (p. 9) |
| Fins | 4, diamond wedge (p. 14) | 3, double wedge (p. 9) | 4, biconvex (p. 9) |
| Root, tip, span | 12, 3, 6.3 in (p. 29) | 10, 4, 5 in (p. 32) | 260, 65, 108 mm (p. 33) |
| Fin thickness | 0.3 in (p. 29); 0.28 in drawn (p. 108) | 0.23 in (p. 32) | 5 mm (p. 33) |
| Fin station | root ends 1.00 in from tube end (p. 104) | not given | not given |
| Tail cone | 6.088 to 4.090 in over 5 in (p. 109) | 5.15 to 4.0 in, 6.8 in (p. 34) | 131 to 89.74 mm (p. 12) |
| Liftoff mass | 29.41 kg, 64.83 lbf (p. 29) | 36.21 kg, a sum (p. 5) | 36.858 kg (p. 33) |
| CG, CP | 89.697, 104.72 in (p. 29) | not given | 2,155, 1,826 mm (p. 33) |
| Minimum margin | 2.06 calibers (p. 29) | "10%" target (p. 8) | 2.36 calibers (p. 34) |
| Rail | "17 in" (p. 29), likely 17 ft | 14 ft in the sims (p. 6) | 5.18 m (p. 33) |
| Parts | a drawing per part (pp. 71 to 110) | section lengths (p. 32) | lengths, masses (p. 35) |

- **216's conflicts.** Liftoff weight 64.83 lbf, weight on the rail 66 lbf (p. 29). Diameters of 6.052, 6.10,
  6.088 and 6.123 in on pp. 29, 104, 109 and 73. The nose is a 33.10 in body and a 2.42 in aluminum tip. The CG
  and CP give 2.48 calibers at 6.052 in; the datum, probably the nose tip, is unstated. How the drawn parts add
  up to 139 in (tip, the tail's 1 in shoulder) is left to the design's assumption list.
- **202's conflicts.** Its sections and nose sum to 121.7 in, or 128.5 in with the 6.8 in tail cone, against
  123.6 in. Its nose base is 5.17 in (p. 30). The 36.21 kg adds the 16.78 kg motor (p. 26) to the stated
  motorless 19.43 kg; the recovery table's 26.42 kg dry mass agrees within 0.04 kg (p. 43) (**Inference**).
  Without a CG, a design must place mass by assumption, flagged as in [the Vayuvega note][vv] §7.
- **213's datum.** A CG behind the CP would be unstable, so the 2,155 and 1,826 mm are probably measured from the
  aft end (**Inference**): their 329 mm gap is 2.51 calibers, the margin its chosen N1 fin gives in Table 3
  (p. 11). Its nose includes a 70.8 mm aluminum tip (p. 9). Its listed outer parts sum to about 2,954 mm,
  143 mm short of 3,097 mm; Table 20 (p. 35) gives the airframe as 12.7 cm, not 131 mm (**Inference**).
- **Missing for all three:** burnout CG, inertia, launch angle and azimuth, and launch-day weather; the sims used
  Midland climate (ERA5 for 213), not the day's ([T216] pp. 7, 8; [T213] p. 12).

## 5. Predicted top Mach, against 1.0 and 2.5

| Team | Predicted top speed | Mach | Tool | Above 1.0 by | Below 2.5 by | Source |
|---|---|---|---|---|---|---|
| 216 | 2,342.5 ft/s (714.0 m/s) | 2.03 to 2.10 | RASAero II | 1.03 to 1.10 | 0.40 to 0.47 | [T216] p. 29 |
| 202 | 594 m/s at 294.41 K | 1.73 | CFD case's free stream | 0.73 | 0.77 | [T202] p. 42 |
| 202 | 588 m/s | 1.67 to 1.73 | OpenRocket | 0.67 to 0.73 | 0.77 to 0.83 | [T202] p. 5 |
| 213 | 571.63 m/s | 1.62 to 1.68; 1.7 plotted | RASAero II plot | 0.62 to 0.7 | 0.8 to 0.88 | [T213] pp. 11, 34 |

- **The conversion.** Where no Mach is given, speed is divided by 340.3 to 351.9 m/s (**Inference**). For 202,
  594 m/s over the stated 294.41 K's 344.0 m/s is Mach 1.73. 213's Fig. 6 peaks near Mach 1.7 by eye.
- **216 may run faster than shown.** Its flutter analysis puts Mach 1.05 at 1,144.8 ft/s ([T216] p. 16), so its
  run used about 332 m/s sound: 714.0 m/s is then Mach 2.15 (**Inference**). It is the lightest, by about 7 kg.
- **One claimed flight Mach.** A news report says CURSR-V reached Mach 1.93 [N3], without saying how. The
  TeleMega logs acceleration, so the figure may be integrated from it (**Inference**, unconfirmed). It is 0.25
  (15%) above the report's 1.68, more than the apogee's 6.49% excess.

## 6. Which could fly apogee-only from the report

- **216 Bandit: yes, the strongest.** No brakes, single stage, a complete outer shape from drawings, a fin
  station, CG, CP and liftoff mass. Its assumptions to record: one diameter of four, the 64.83 or 66 lbf mass,
  the CG datum, the length gap and inertia. Predicted top Mach 2.03 to 2.15.
- **202 Odyssey: yes, with a weaker design.** No brakes and most of the shape, but no CG, CP or fin station; the
  liftoff mass is a sum. Each gap becomes a stated assumption with its apogee sensitivity. Predicted Mach 1.73.
- **213 CURSR-V: only flagged.** The best-documented design, but its brake use is unpublished, so by [the 2025
  note's][i25] §7 it is "air brakes, use unknown", out of drag statistics. Predicted Mach 1.62 to 1.7; claimed 1.93.

## 7. Licenses and terms

| Source | Terms | What the project may do |
|---|---|---|
| Reports [T202, T213, T216] | No license; 216 marked proprietary (5 pages) | Cite facts with pages; cache in `refs/` |
| 2026 rules [E2] | CC 4.0 for the rules (p. 1); all reports published, no confidentiality (p. 16) | Cite; may cache |
| Scoresheet, page [E1, E3] | None stated | Cite the rows; cache under `refs/` |
| News [N1 to N3] | Publishers' own terms | Cite single facts only |

216's checklist pages (pp. 43, 52, 58, 65, 68) say its contents are proprietary to the university and not to be
copied without consent, as 2025's team 208 marked "Do Not Share"; the rules publish every report anyway (p. 16).
**Inference:** treat the reports as all rights reserved; a design built from them stays in `refs/` and only error
statistics are published, as in [the 2025 note][i25] §1.

## What this suggests

1. **Add Bandit and Odyssey to #724's record.** The rule [the 2025 note][i25] §7 proposed (public report, scored
   apogee above 27,000 ft, the §2 bound past Mach 1, a stated-assumption list) admits both. Beside RASAero II's 4
   [rcf] and 2025's five, they add margin to the floor of 5 and two 2026 flights on a second year's rules.
2. **Fly Bandit first.** Of the three, it alone has a drawn fin station and a prediction past Mach 2,
   in the upper core band that 2025's flights (predicted Mach 1.4 to 1.9) leave empty.
3. **Score CURSR-V's apogee as a one-sided check.** If its brakes opened, 30,683 ft is a lower bound on the passive
   apogee: a simulator that predicts less is wrong in a known direction (**Inference**). The record would have to
   allow a bound, not a value.
4. **Weather.** Midland, 15 to 20 June 2026 [N3], launch times unpublished: weather needs a day range per flight.

## Open questions

- Did CURSR-V's brakes deploy at IREC? How was its Mach 1.93 measured, and by which unit?
- Which altimeter gave each official reading, and what did the second unit read?
- What did Bandit weigh on the pad after its rebuild, which of its four diameters is the airframe's, and how do
  its drawn parts add up to 139 in (not reproduced in the check)?
- What did Odyssey's second test flight show, and where are its CG and fin station?

## Sources

All read 2026-11-05. sha256 prefixes are of the copies under `refs/research/irec-2026-o5500x-supersonic/`.

1. **E1.** ESRA, IREC 2026 final scores, <https://esrarocket.org/s/final_scores_ext_7-13.pdf>, 5 pp., created
   2026-07-13, `6f9984b7812bc1d2`.
2. **E2.** ESRA, IREC Rules and Requirements 2026 v1.0 (effective 2025-10-05),
   <https://www.esrarocket.org/s/IREC-Rules-and-Requirements-Document-2026-v10-f8wf.pdf>, 43 pp., `eab4e31ef47e75d6`.
3. **E3.** ESRA, Past IREC events, <https://www.esrarocket.org/past-irec-events>, and its "2026 Technical Reports"
   Dropbox folder, served as one zip of 2,445,666,982 bytes ([the 2026 survey][pf26] §4).
4. **T202.** University of Texas at El Paso, Team 202 report (Odyssey), 110 pp., from E3, `5f799cc16167b469`.
5. **T213.** Chulalongkorn University, Team 213 report (CURSR-V), 90 pp., from E3, `94ed1680ce04a60c`.
6. **T216.** Mississippi State University, Team 216 report (Bandit), 114 pp., from E3, `bd19a1b85606b6f0`.
7. **N1.** Mississippi State University news, 2026-07-10, <https://www.msstate.edu/newsroom/article/2026/07/>
   `msu-space-cowboys-win-international-rocket-engineering-competition`.
8. **N2.** UTEP College of Engineering news, 2026-07-22, <https://www.utep.edu/engineering/news/2026/july/>
   `utep-rocket-team-proves-its-place-among-the-worlds-best-with-second-consecutive-international-podium-finish.html`.
9. **N3.** Khaosod English, 2026-07-16, read in a browser (it refuses scripted fetches),
   <https://www.khaosodenglish.com/life/2026/07/16/chula-rocket-team-claims-third-place-at-irec-2026/>.
