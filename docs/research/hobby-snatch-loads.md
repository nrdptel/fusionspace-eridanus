# Snatch and shock-cord recoil loads at hobby scale

**In short: a published closed form exists, and the few hobby flights that resolve the cord phase say it matters.** The
Air Force's 1978 recovery design guide gives the peak cord load when two masses meet a cord at relative speed Δv: F =
Δv·√(κ·μ) plus drag, with κ the cord's spring constant and μ = m₁m₂/(m₁ + m₂) the reduced mass [1, pp.
237, 337]. Static stiffness flatters it [1, p. 338]. On invented round numbers (§4), a 3 kg booster and a 1 kg nose
meeting at 10 m/s on 6 m of cord peak near 0.5 kN on 1,000 lbf tubular nylon and 1.4 kN (2.6 times) on 1,000 lbf Kevlar,
at or above a hobby main's 0.3 to 0.5 kN opening. On three public 500 Hz flights of two small Kevlar-cord rockets (§6),
the cord phase read 2.3 to 11 times the simulator's `Instant` load at every drogue event and 1.9 to 4.3 times at two of
three mains, on a 50 ms mean. **How far to trust it:** the model and the cord data are primary; hobby Δv is bounded,
not measured; the flights are six events with one accelerometer each and no section masses, so no cord tension is
measured. Every number marked **Inference** is computed here.

This builds on [canopy-opening-loads](canopy-opening-loads.md) (§3: Knacke p. 5-52's 3 to 7% band, the −93 g builder
trace) and [hobby-deployment-fast-logs](hobby-deployment-fast-logs.md) (two humps in four of six 500 Hz openings).
`Instant` is (v₁/w)² g, v₁ the speed before the firing and w the steady descent after, as in that note.

## 1. The published model

- **AFFDL-TR-78-151** [1], the third edition of the USAF parachute handbook: snatch F_s = F_i + D − W·sinθ, an
  inertial part plus drag less the weight component, with F_i = Δv_max·√(m·κ), m the canopy plus half the lines
  and any attached pilot chute with its air [1, p. 237, eqs. 6-1, 6-2].
- **Two bodies** [1, p. 337, eqs. 7-4 to 7-7]: momentum and energy (kinetic plus κΔl²/2) are conserved from slack to
  full stretch; with Hooke's law the peak is Δv_max·√(κ·m_b·m_c/(m_b + m_c)). This is the form for a hobby
  rocket, whose charge throws a nose section, not a light canopy.
- **The spring constant** [1, p. 237, eq. 6-3]: κ = Z·F_T/(l·(ε − ε₀)), Z cords in parallel, F_T the load at
  strain ε (taken at 0.4 to 0.6 of the rating), l the unstretched length, ε₀ a creep offset that fast loading lacks.
  Members in series add as 1/κ = Σ 1/κᵢ [1, p. 240, eq. 6-4].
- **The worked example** [1, pp. 429 to 431] (a 1 in nylon webbing bridle): a tangent at half load gave 1,504 lb/ft; a
  higher Δv then justified the steepest slope, 2,099 lb/ft, and a snatch of 6,572 lbf, "close enough to the ultimate to
  risk failure". It adds the pilot chute's air as ρ(C_D S)^{3/2}·0.66 [1, p. 430].
- **Knacke** gives no formula; caught air raises snatch "considerably" [2, p. 7-23]. Cockrell defines it, with no
  formula [3, p. 44]; NASA's analysis treats lines as inelastic [4, p. 5].
- **Hobby texts** give no load formula: Nakka judges the opening more critical [10]; one maker rates a cord at 100 times
  the rocket's weight [11]; Apogee advises 40 calibers of cord [22, PDF p. 8]. **Inference:** a hobby charge throws
  about a quarter of the mass (§4), far past Knacke's 3 to 7%, so snatch should often rival the opening.

## 2. Cord stiffness

| cord | rated strength | elongation at break | source |
|---|---:|---:|---|
| 1/2, 9/16, 1 in tubular nylon, MIL-W-5625K | 1,000 / 1,500 / 4,000 lbf min | not specified | [5, Table I, p. 4] |
| nylon cord, MIL-C-5040H types I to IV (III is 550) | 95 to 750 lbf min | 30% min | [6, Table II, p. 6] |
| nylon fiber | 2.5 to 9.5 g/denier | 29 to 40% | [2, p. 6-75] |
| 1 in nylon webbing, static curve, read by eye | 7,000 to 7,500 lbf | about 21% | [1, Fig. 7.6, p. 337] |
| Kevlar 29 / 49 yarn | 23.0 / 23.6 g/denier | 3.6% / 2.4% | [8, Table II-1] |
| 500 lbf braided Kevlar cord, first pull, read by eye | 545 lbf | about 3% | [9, slide 9] |
| 1/4 in bungee, MIL-C-5651D type I | 120 lbf min | 140% min | [7, Table III, p. 16] |

- **Nylon's curve is an S** [1, Fig. 7.6], soft to 10% strain and steep to 15%. **Inference:** its steepest slope is
  about 2.5 times the secant to break, moving the peak up to 1.58 times. Tubular nylon's spec sets no elongation [5].
- **Fast loading is stiffer:** 400 lbf nylon cord at 120 to 690% per second carries 1.2 to 1.3 times the static load,
  near 1.5 at low strain [1, Fig. 7.7a, p. 338, by eye]; 10 m/s on 6 m is 170% per second.
- **Kevlar stiffens with use** [9, slides 9, 11]: on 500 lbf cords the mean slope rose from about 220 to 329 lbf per
  percent (Kevlar 29) over five pulls, with about 1% permanent stretch; 2,000 lbf cords from 675 and 750 to 1,006 and
  1,056. **Inference:** a used cord's slope is about 1.75 times the first-pull secant (329 against 545/2.9), so a
  first-pull κ flatters snatch by up to √1.75 = 1.32 times.
- **Bungee** [7, Table III]: 1/4 in carries 16 to 26 lbf at 100% stretch (**Inference:** 130 to 300 times softer).

## 3. The relative speed Δv

- **From the charge (Inference, energy bound).** Gas at gauge pressure p over a bore of area A for the shoulder's travel
  s does at most p·A·s of work on the relative motion, ½μΔv², so Δv ≤ √(2pAs/μ): a 98 mm bore at 15 psi over
  0.05 to 0.15 m gives 10 to 18 m/s for μ = 0.75 kg. It errs high unless the pressure passes the design value; a 6 in
  student bay estimated 48.5 psi ([ejection-charge-bay-scaling](ejection-charge-bay-scaling.md)).
- **From drag, when the chute catches air early** [1, Fig. 6.5, p. 240]: Wolf's chart gives Δv_max/v₀ against ρ(C_D
  S)·l/(2m); read by eye with the body near terminal, 0.31 at 0.1, 0.50 at 0.36, 0.74 at 1.8. **Inference:** a 1 kg
  nose whose chute shows 0.1 to 0.5 m² on 6 m of cord sits at 0.36 to 1.8; at v₀ = 50 m/s, well off apogee, Δv is 25
  to 37 m/s. Doubling the cord raises Δv, so the peak falls only to 0.78 to 0.88 times, not 0.71.
- **Slack.** The one direct comparison found: a 53 lb Student Launch rocket flown once on 52 ft of main cord instead of
  its usual 12 ft read about 4 times the usual shock after 1.5 s of free fall [14, post 53].

## 4. Worked example (Inference; invented round numbers, not any real design)

3 kg booster, 1 kg nose (μ = 0.75 kg), 6 m of cord; κ the first-pull secant P/(ε_u·l); g per section weight.

| cord | κ | Δv | peak | of rating | nose | booster | Δv at rating |
|---|---:|---:|---:|---:|---:|---:|---:|
| 1/2 in nylon, 1,000 lbf, ε_u 21% | 3,530 N/m | 10 m/s | 515 N | 12% | 52 g | 18 g | 86 m/s |
| Kevlar, 1,000 lbf, ε_u 3% | 24,700 N/m | 10 m/s | 1,361 N | 31% | 139 g | 46 g | 33 m/s |
| 1/2 in nylon, ε_u 21% | 3,530 N/m | 25 m/s | 1,286 N | 29% | 131 g | 44 g | 86 m/s |
| Kevlar, 1,000 lbf, ε_u 3% | 24,700 N/m | 25 m/s | 3,404 N | 77% | 347 g | 116 g | 33 m/s |
| 1/4 in bungee, 16 to 26 lbf at 100% | 12 to 19 N/m | 10 m/s | 30 to 38 N | 6 to 7% | 3 to 4 g | 1 g | not linear |

- **Kevlar against nylon** at equal rating: κ is 7 times higher, the peak 2.6 times. To its rating the nylon stores
  2,800 J and the Kevlar 400 J (½P·ε_u·l), against 37.5 J at 10 m/s and 234 J at 25 m/s. With §2's corrections at
  10 m/s, nylon reaches about 1.0 kN and a used Kevlar cord 1.8 kN.
- **Against an opening.** A main of C_D S = 2.7 m² at 25 m/s has q·C_D S ≈ 1.0 kN; measured hobby factors of 0.29 to
  0.52 ([canopy-opening-loads §3](canopy-opening-loads.md)) put its peak near 0.3 to 0.5 kN, at or below the snatch.

## 5. Which way each assumption errs

| assumption | direction | size | source |
|---|---|---|---|
| static κ, not dynamic | flattering | 1.1 to 1.2 times on the peak | [1, p. 338] |
| secant to break, not steepest slope | flattering | up to 1.58 times (nylon) | [1, pp. 337, 430] |
| first-pull κ on a used cord | flattering | up to 1.32 times (Kevlar) | [9] |
| drag D left out | flattering | D at line stretch; small near apogee | [1, p. 237] |
| canopy air left out of the moving mass | flattering | up to 1.10 times for 0.1 to 0.5 m² | [1, p. 430; 2, p. 7-23] |
| a lumped mass for a bagged line bundle | conservative | "excessively high"; nil for a nose cone | [1, p. 339] |
| linear spring near the rating | flattering near rating | an S-curve stores less than ½P·ε_u·l | [1, Fig. 7.6] |

- **Recoil (Inference).** Textiles are soft springs with slight rebound on a first load [1, p. 240], so later stretches
  peak lower and the first bounds the cord load; the recoil's harm is contact (a nose striking its booster, a cord
  dragged over the tube edge). An opening chute's drag adds energy, so a later peak is not bounded by this. A stress
  wave is √(cord mass/μ) of the lumped peak, 0.35 times for 6 m of 1/2 in nylon (0.09 kg [5]).

## 6. Three public 500 Hz flights (Inference, computed 2026-11-04)

One builder posted Blue Raven 500 and 50 Hz files for three flights of two small dual-deploy rockets, A and B on one
rocket and C on the other, each with Kevlar cord in taped z-folds [12]. Masses, sizes and motors are left out here.
Peaks are the three axes' magnitude on a 10 or 50 ms centered mean. "Cord phase" is 0.03 to 0.4 s after the charge
spike; "later" is 0.4 to 4 s. Each `Instant` range takes w from both the log's baro slope and the logger summary's
descent rate.

| event | charge, raw / 50 ms | cord phase, 10 / 50 ms | later, 10 / 50 ms | `Instant` | cord phase / `Instant` |
|---|---|---|---|---|---|
| A, drogue | 68 / 14 g | 22 / 15 g at +0.20 s | 14 / 11 g | 2.7 to 3.8 g | 4.0 to 5.5 |
| B, drogue | 52 / 10 g | 9 / 4 g at +0.29 s | 17 / 7 g | 1.4 to 1.8 g | 2.3 to 2.9 (later 4.0 to 5.1) |
| C, drogue | 73 / 12 g | 23 / 10 g at +0.31 s | 19 / 7 g | 0.9 to 1.1 g | 9.6 to 11 |
| A, main | 170 / 19 g | 30 / 22 g at +0.20 s | 24 / 12 g at +0.69 s | 8.4 to 11.3 g | 1.9 to 2.6 |
| B, main | 68 / 9 g | none above the charge tail | 8 / 7 g at +0.75 s | 4.0 to 6.2 g | none |
| C, main | 235 / 24 g | 38 / 18 g at +0.11 s | 13 / 8 g at +0.42 s | 4.1 to 4.5 g | 3.9 to 4.3 |

- **Which is snatch?** One accelerometer can't tell line stretch from a tape break. The pulses come 0.11 to 0.31 s after
  the charge, as the z-folds break and the cord runs out [12]. **Inference:** at the posted main's size and Knacke's
  fill constants for unreefed canopies, 7 to 14 diameters [2, Table 5-6, p. 5-44], it fills in 0.3 s or more at 17 m/s,
  so the pulses most likely precede inflation.
- **Tally.** Cord phase above every later pulse (10 and 50 ms): drogue 2 of 3, main 2 of 3; above `Instant`: drogue 3 of
  3, main 2 of 3. The builder suspects A's later drogue spikes were a small zipper; another member doubts it [12].
  Charge kicks are larger on single samples (52 to 235 g).
- **Tension, bounded above.** With the whole coast mass on the sensor's side, the largest pulses are at most 5 to 7% of
  the cord's stated rating.
- **High or low.** High: A's main pulse came at 2,800 to 3,100 deg/s or more (a gyro axis at its rail), 25 to 29 g or
  more of ω²r per 0.1 m of offset (others 0.1 to 4.4 g). Low: 500 Hz misses spikes under 2 ms.

## 7. Other reports, student rigs and failures

- **The −93 g trace** [13, post 28] was the main's shear pins breaking; the apogee cord running out read −8 g.
- **Off-nominal.** A two-stage rocket of 8 lb stages put its main out at apogee at 220 mph horizontal on 15 ft of 3/16
  in tubular Kevlar: 120 g at the charge, harness spikes past 50 g, one of 167 g (**Inference:** up to about 1,340 lbf
  if a whole stage moved), two shroud lines broken [13, posts 38, 42, 45].
- **Failures, no load given.** A 550 lb Kevlar line broke at main deployment after a Mach 3.1 flight (a reply guesses a
  tube-edge cut) [16, posts 11, 26]; 200 lb Kevlar snapped on a 2 lb rocket [15, post 6]. 32 and 50 g accelerometers
  rail at deployment [13, post 16; 16, post 6], reading low by an unknown amount.
- **Student load cells resolve no snatch.** Portland State's car-mounted drogue test [17] logged at 10.6 Hz: run peaks
  of 16 to 114 lbf (counted here from the raw file [18]), drag at the car's speed, not snatch. NUSTARS logged at 90.5 Hz
  ([hobby-deployment-fast-logs §5](hobby-deployment-fast-logs.md)); MIT planned 4 Hz [20]; WashU's snatch tester
  printed no values [19]; Georgia Tech judged snatch negligible and proof-tested parts [21, p. 8].
- **Knots** kept 40% (aramid) to 92% of strength; knotted cords broke near static strength when dropped [23].

## What this suggests

- **A snatch output for M12.2, beside M6.6 and M6.7's deployment numbers.** Print F = Δv·√(κμ) + D as a range,
  apart from the opening load: κ from P/(ε_u·l) up to 2.5 times it (nylon) or 1.75 (Kevlar), times 1.2 to 1.5 for
  rate; Δv from √(2pAs/μ) up to that plus Fig. 6.5's drag term; μ with the chute, its air and half the cord on the
  nose side; beside it the Δv at the cord's rating, P/√(κμ), and "ground test first". Inputs: cord rating,
  material, free length, section masses, bore, shoulder travel. M12.2's *done when* names snatch.
- **`Instant` is no harness bound at the drogue:** on §6's flights the cord phase read 2.3 to 11 times it.
- **Terms.** The specs, [1] and NASA's works are public and cached under `refs/research/hobby-snatch-loads/`; forum
  files, [2], [8], [10], [22] and [23] are cited only; PSAS's repository [18] is GPL-3.0: its data file only.
- **A check:** fit §6's drogue rows (the canopy loads least) with the closed form, Δv from the charge pulse's area.
  Settling the main needs a swivel load cell at 1 kHz or more, a logger per section, masses and offsets, and video.

## Open questions

- No measured hobby Δv at line stretch; no load-elongation curve for hobby cord sizes; no rebound energy for any cord.
- Section masses and sensor offsets for §6's flights (to get tension and remove spin); B's missing main pulse.
- Not read: the 1963 handbook (ASD-TR-61-579), Purvis's papers, a second 1970 Huckins paper (DTIC down, 2026-11-04); the
  two-stage flight's files; UAH's early phase; Notre Dame's FRR. Wolf's chart is for a pack behind a body.

## Sources (read 2026-11-04)

1. Ewing, Bixby, Knacke, AFFDL-TR-78-151 (1978). https://everyspec.com/USAF/USAF-General/AFFDL-TR-78-151_39040/
2. Knacke, *Parachute Recovery Systems Design Manual*, NWC TP 6575 (1991), DTIC ADA247666. Cited, not reproduced.
3. Cockrell, *The Aerodynamics of Parachutes*, AGARDograph 295 (1987).
4. Huckins, NASA TN D-5619 (1970). https://ntrs.nasa.gov/citations/19700005898
5. MIL-W-5625K (1997). https://everyspec.com/MIL-SPECS/MIL-SPECS-MIL-W/download.php?spec=MIL-W-5625K.014723.pdf
6. MIL-C-5040H (1994). https://everyspec.com/MIL-SPECS/MIL-SPECS-MIL-C/download.php?spec=MIL-C-5040H.031090.pdf
7. MIL-C-5651D (1985). https://everyspec.com/MIL-SPECS/MIL-SPECS-MIL-C/download.php?spec=MIL-C-5651D.013946.pdf
8. DuPont, *Kevlar Aramid Fiber Technical Guide* (2017). https://www.r-g.de/wiki/images/e/ec/Td_en_Kevlar_guide.pdf
9. Nichols, Daum and Hemler, braided Kevlar cords, AIAA (2026). https://ntrs.nasa.gov/citations/20260004667
10. Nakka, Appendix K, parachute design. https://nakka-rocketry.net/Appendix-K.htm
11. Rocketman Parachutes, braided Kevlar cord. https://www.the-rocketman.com/products/braided-kevlar-shock-cord
12. Rocketry Forum, one builder's Blue Raven files for three flights (2024 to 2026); cited collectively, not linked.
13. Rocketry Forum thread 196302, posts 16, 28, 38, 42, 45 (2026). https://www.rocketryforum.com/threads/196302/
14. Rocketry Forum thread 17705, post 53 (2011). https://www.rocketryforum.com/threads/17705/
15. Rocketry Forum thread 161101, posts 5, 6 (2020). https://www.rocketryforum.com/threads/161101/
16. Rocketry Forum thread 198456, posts 11, 13, 26 (2026). https://www.rocketryforum.com/threads/198456/
17. House, Portland State honors thesis (2019). https://pdxscholar.library.pdx.edu/honorstheses/753/
18. PSAS, parachute data zip (2019), data file only. https://github.com/psas/lv3.0-recovery
19. Mallalieu et al., *Parachute Snatch Force Tester* (2025). https://repository.library.washu.edu/mems411/270
20. MIT Rocket Team wiki (2018). https://wikis.mit.edu/confluence/display/RocketTeam/Load+Cell+Apparatus
21. Garud, Johnson and Lagares de Toledo, *Material Girl Launch Report*, arXiv:2411.00807 (2024).
22. Van Milligan, *Peak of Flight* #591 (2023). https://www.apogeerockets.com/education/downloads/Newsletter591.pdf
23. Moyer, Tusting and Harmston (2000). https://user.xmission.com/~tmoyer/testing/High_Strength_Cord.pdf
