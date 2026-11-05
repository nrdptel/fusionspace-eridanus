# How many digits each kind of figure HPR Sim quotes is known to

How many digits may a page quote for each kind of figure, so that M0.9g's check ([ADR-296][adr], [#966][i966]) can
fail "a figure quoted to more digits than its kind is known to"? This note reads the published rules (the GUM, NIST,
ISO 80000-1, ASTM E29), takes each kind's uncertainty from the project's notes, and counts what the site quotes.
**Inference** marks this note's arithmetic. Read 2026-11-05.

**In short.** The rules agree: give the uncertainty to one or two significant digits, round the value to the same
place, round once. Applied to the uncertainties the notes already hold, most kinds earn 2 to 4 significant digits.
- **Two digits of uncertainty at most, the value to match.** The GUM §7.2.6 says two "usually suffices"; the Particle
  Data Group keeps one when the uncertainty starts 355 to 949; ASTM E29 rounds to between 0.05σ and 0.5σ (§1).
- **A bare figure claims half a unit in its last digit** (ISO 80000-1 §7.2.4). "3,892.1 m" from a barometer claims
  ±0.05 m; two altimeters on one rocket differ by a median 0.37% (14 m at that height). So "3,892 m" (§2).
- **Simulated values are known to their model, not to f64.** HPR Sim's apogee errors over 60 flights have an SD of
  about 13% (**Inference**): a prediction gets 2 or 3 significant digits ("3,400 m"), a mean error over 60 one decimal
  ("+7.8%", standard error 1.7 points), not "+7.76%". A code's C_D, about 16%, gets 2 decimals (§4).
- **A code-to-code agreement is a bound on the code, not digits of the rocket.** Quote "agrees within 1e-9" as a
  tolerance to one significant digit; the value it qualifies keeps its kind's digits (§3).
- **The five-decimal figures #966 counts are mostly not the problem:** of 236 read one by one, 3 are a physical value
  past its uncertainty. Over-quoted values sit among shorter figures, and a digit count misses some ("+7.76%") (§5).
- **For the check:** fail a figure with more decimals than 1 − k, k the place of its kind's standard uncertainty's
  first digit (the GUM's ceiling); exempt five classes; report, not fail, a figure of unknown kind (§6).

## 1. What the rules say about the uncertainty's digits

- **GUM, JCGM 100:2008 §7.2.6 (p. 26)** [1]: a value and its uncertainty "should not be given with an excessive number
  of digits"; it "usually suffices" to give the uncertainty to at most two significant digits, keeping more only to
  avoid round-off in later sums. Its worked case: y = 10.057 62 Ω with u = 27 mΩ is written 10.058 Ω. In a final
  result it "may sometimes be appropriate" to round an uncertainty up (10.47 mΩ to 11 mΩ), with common sense.
  §7.2.2 discourages ± for a standard uncertainty. The EURACHEM/CITAC guide §9.5.1 (p. 31) says the same [6].
- **When one digit, when two.** The GUM gives a ceiling, not a switch. The Particle Data Group gives one [7]
  (Introduction §5.3): an error whose first three digits are 100 to 354 keeps two; 355 to 949 keeps one; 950 to 999
  rounds up to 1000 and keeps two. So 0.827 ± 0.119 is 0.83 ± 0.12, and 0.827 ± 0.367 is 0.8 ± 0.4.
- **ASTM E29-90 §7.4** [5]: round a test result to a unit between 0.05σ and 0.5σ (1.457 29 with s = 0.0052 is
  1.457). Its Note 4: a rounding unit w adds w²/12 to the variance, so w ≤ 0.5σ raises σ by at most 1%
  (√(1 + 0.25/12) = 1.010). A u from n readings is itself uncertain by [2(n − 1)]^(−1/2), 24% at n = 10 [1] §4.3.2.
- **Round once, from the full figure.** ISO 80000-1:2022 B.4 [4]: rounding in stages "may lead to errors" (12.254 is
  12.3, not 12.25 then 12.2); E29-90 §6.5 says the same. **Ties:** ISO B.3 names two rules in use, the even multiple
  (Rule A, "generally preferable") and the one greater in magnitude (Rule B, half away from zero); E29-90 §6.4
  prescribes the even one, and NIST SP 811 §B.7.1 [3] gives it among the rules "one may follow". ISO B.5: where
  safety limits apply, round only one way. E29-22 is current; this note read the 1990 text.

## 2. A figure with no uncertainty beside it

- **ISO 80000-1 §7.2.4 (p. 12)** [4]: a bare number is read as rounded to 1 in its last digit, so 401 008 stands for
  401 007.5 to 401 008.5; SP 811 §7.9 [3] and E29-90 §3.1 [5] agree (1200 m against 1.200 km). NIST TN 1297 [2]
  sets no digit rule; its Appendix D.1.1.1 pairs a number with "uncertainty", never the qualitative "accuracy".
- **So every bare figure on the site claims half a unit in its last digit** (**Inference**), and `docs/writing.md`
  already says so by example: "`5,104 ft` from a barometer, not `5,103.87 ft`".

## 3. A code-to-code agreement is not a measurement

- **Verification against validation.** Oberkampf and Trucano [8] (p. 14) give the AIAA definitions: verification shows
  a code solves its model correctly, validation how well the model represents the world. Comparing codes is a
  verification test (Fig. 6, p. 45); its errors (p. 19) are discretization, iteration, round-off and programming, none a
  physical uncertainty. Agreement says little about physics: in a study they summarize (p. 43), nine implementations
  of the same published algorithms agreed to one or two significant figures.
- **Numerical uncertainty goes with its scope:** the ASME fluids journal [9] reports it as a percentage, without
  modeling error.
- **f64 bounds what a code can promise.** IEEE binary64 holds log₁₀(2⁵³) = 15.95 decimal digits; Rust's
  `f64::EPSILON` is 2.22 × 10⁻¹⁶ and its docs call `sin`, `exp`, `powf` and `ln` non-deterministic in precision
  across platforms [10]. #966 notes that macOS and Linux reports can differ in their last digits.
- **How to quote it** (**Inference**): "agrees with OpenRocket within 1e-9 relative" is a bound, one significant
  digit, rounded outward (ISO B.5). Agreeing values, if shown, are code output in code formatting. Nine agreeing
  digits never license nine quoted ones: "within 0.016 calibers of OpenRocket" is not "known to 0.016 calibers".

## 4. Each kind's uncertainty and the digits it allows

1σ unless stated. "Quote to" applies the rule (two digits of u if it starts with 1 or 2, else one; the value to that
place) at a typical size. Project notes are linked; their own primary sources are cited there.

| Kind | Typical uncertainty | Source | Quote to |
|---|---|---|---|
| Apogee, logged by a barometric altimeter | ≈0.4% per unit, either sign | [two altimeters][n1] §5 | whole m to ~7 km |
| Apogee, simulated, as a prediction | SD of errors 13.4% (60 flights), 7.3% (7) | [accuracy][acc] | 2 sig. digits |
| Mean apogee error over n flights | SD/√n: 1.7 points (60), 2.8 points (7) | [accuracy][acc] | 0.1 point |
| One flight's apogee error | ≈0.4 point, from the log alone | [n1] | 0.1 point |
| Top speed, accelerometer log | ≈1% after a two-point calibration | [accel scale][n2] | 0.1 m/s below 300 m/s |
| Top speed or Mach, simulated | no flight check; OpenRocket −0.7% to +6.1% | [accuracy][acc] | unsourced |
| Mach, from a logged speed | as the speed; the air adds ≈0.2% | [n2], [era5-surface-temperature][n3] | 0.001 |
| Dry mass, weighed | 0.33% assumed (pro); ≈0.5% "fair" | [dispersion-sigmas][n4] §3 | 1 g at 5 kg |
| Dry mass, from a design file | median 7.4% light, σ ≈ 11% | [as-built][n5], [flyer][n6] | 2 sig. digits |
| CG position, measured or from a file | none found | none | unsourced |
| CP and margin, calibers | 0.15 to 0.9 cal on two finned shapes | [dtic finners][n7], [tunnel data][n8] | 0.1 cal |
| C_D, whole rocket from a code | σ ≈ 16% (coast fits) | [flight-derived-cd][n9] | 2 decimals |
| Total impulse, one certification test | 3.1% to 3.8% per test | [lot impulse][n10], [n11] | 3 sig. digits |
| Burn time | 2.0% (one session) to 9.2% (years apart) | [n11] | 0.01 s |
| Air temperature, reanalysis layer mean | 0.9 to 1.35 K per flight; bias ±0.5 K | [n3] | 0.1 K |
| Temperature, altimeter's own sensor | ±0.8 to ±2.0 °C limits; reads the bay | [11], [bay][n12] | 1 °C as air |
| Wind speed, reanalysis | 1.4 to 1.9 m/s RMS | [era5-wind-direction-us][n13] | 0.1 m/s |
| Descent rate under a rated canopy | ≈7% | [hobby-parachute-cd][n14] §2 | 0.1 m/s |
| Logged trace RMS, % of apogee | none found | none | unsourced |

- **Logged apogee.** Two units on one rocket differ by a median 0.37% over 59 flights, one pair in five over 2%;
  as two independent units, about 0.4% each, with a heavy tail ([n1] §5). At 3,892 m, 0.4% is 16 m: "3,892 m".
- **Simulated apogee.** The committed 60-flight report's mean error +7.76% and RMS 15.43% give a sample SD of 13.4%;
  the 7 public errors, −8.90% to +10.40%, 7.3% (**Inference**, n − 1). As a prediction "3,400 m", not "3,412 m"
  (Rule G in §6, one place looser, would pass "3,410 m"); a per-flight error "+10.4%", not "+10.40%".
- **CP and margin** is a model error, not a σ: HPR Sim's linear fin terms put two finners' CP 0.15 to 0.22 and 0.7
  to 0.9 caliber ahead of the measured (a third report's own theory, 0.34 to 0.70), so the margin reads low. Taken as
  u, 0.15 to 0.29 allows hundredths and 0.36 to 0.9 tenths; a margin also carries the CG, which no note sizes, so
  tenths is the defensible place (**Inference**).
- **C_D.** Fourteen coast fits put flight C_D at 0.89 to 1.11 of the code's (quartiles), σ ≈ IQR/1.349 ≈ 16%
  (**Inference**): 0.45 ± 0.07 is "0.45".
- **Impulse and burn time.** Repeat tests of 22 motors moved impulse a median −2.7%, SD 4.4 points of a difference:
  3.1% per test (**Inference**, ÷√2); re-read curves give 3.8%. A 2.5 s burn is "2.53 s" at most. **Air:** ERA5's
  per-flight 1σ of 0.9 to 1.35 K moves sound speed (as √T) by about 0.2%. **Sensor temperature:** the datasheet's
  limits would allow 0.1 °C, but as the air's temperature a bay sensor is off by the climb's cooling, so 1 °C.

## 5. What the site quotes today

A script read the 78 pages `docs/SUMMARY.md` links and `README.md` at main f8a00664 for figures with 4 or more
decimals or 5 or more significant digits. **2,045 figures on 67 of 79 pages;** 513 in fenced code. Most:
`physics/aero.md` 437, `format/flight-logs.md` 276, `cli.md` 105 (85 in code), `format/ork.md` 101, `accuracy.md` 89.
- **5 or more decimals, #966's measure: 316,** 80 in code. The other 236, each read and classed:

| Class | Figures | Example |
|---|--:|---|
| Model internals: f64 crossings, validity edges | 69 | Mach 1.5288696, a bisected edge |
| Worked examples that re-derive a printed value | 58 | 0.61575 kg summed by hand |
| Constants and defined values | 39 | 9.7803253359 m/s², WGS 84's equatorial gravity |
| Literals: float output, file contents | 35 | 0.30000000000000004 |
| Code-to-code agreement, convergence | 28 | 0.00005% |
| Identifiers read as figures | 4 | an arXiv number |
| Physical value past its uncertainty | 3 | −0.01263 per degree, read off a chart |

- **Over-quoted values sit among the shorter figures.** The script left 1,235 plain values: 562 with four decimals,
  491 with fewer but five or more significant digits ("3,892.1"), 182 with five or more. In a 1-in-31 sample of 40
  read by hand, 9 were a physical value past its uncertainty: 22%, 95% Clopper–Pearson 11% to 39%, some 130 to 480
  figures (**Inference**; the 40 judgments weren't kept). Examples: a logged "3,892.1 m", a simulated C_D "0.5566".
  A mean error "+7.76%" has too few digits to flag, yet its kind allows one decimal: only a check by kind finds it.
- **Traced figures.** 82 sit beside a link to a committed report or in `accuracy.md`'s tables (§6).

## 6. A rule for the check

Let a figure have d decimals in its unit, and its kind a standard uncertainty u. Let k = floor(log₁₀ u).
- **Rule G (the GUM's ceiling, suggested):** fail if d > 1 − k. It allows two digits of u everywhere, so it never
  fails a figure the GUM accepts. **Rule P** (Particle Data Group) is one decimal stricter when u starts 355 to 999;
  **Rule E** (E29) fails if 10^(−d) < 0.05u. Worked (**Inference**): u = 0.36 allows 2, 1, 1 decimals under G, P, E;
  u = 2.5 allows 1, 1, 0; u = 36 allows 0, −1, −1 (round to tens).
- **Relative form:** with r = u/|y|, Rule G allows 2 − log₁₀ r significant digits, ±1: 10% gives 3, 1% gives 4.
- **Exempt:** code spans and fences; exponent-form tolerances and rows marked code-to-code; a sourced list of
  constants; worked examples, allowed one guard digit past the result they derive; identifiers. **Report, not fail,**
  a figure whose kind the check can't tell from its unit (m, m/s, kg, N·s, s, K, °C, cal) or a nearby word.
- **Beside `rounds_to`.** It accepts a results-table cell that is the report's figure rounded once, half away from
  zero, keeping at least three significant digits (`KEPT_DIGITS` = 3). Rounding once meets ISO B.4; half away from
  zero is ISO's Rule B, allowed but not preferred, and differs from E29 and SP 811 only on an exact tie. It reads
  only `accuracy.md`'s difference cells, so its floor of three digits clashes with §4 only where an error under 10%
  earns two ("+3.4%") (**Inference**). `untraced_numbers` matches exact digits: a figure rounded outside a results
  table fails its trace until `matches` accepts it.

## What this suggests

- Build M0.9g's digits check on Rule G with the §4 table as its data, each row carrying its source, so the table
  and the check can't drift apart; report unknown kinds until they are classified.
- Let `rounds_to` allow what §3 and §4 allow (one digit for a code-to-code bound, 0.1 point for an error) rather
  than a fixed three digits, and let `untraced_numbers` accept a figure that `rounds_to` its report.
- Write code-to-code results as bounds ("within 1e-9", "at most 0.00005%"), one digit, rounded outward.
- Start the clean-up with the shorter figures and their 130 to 480 values, not the five-decimal list #966 counts.

## Open questions

- No note sizes a measured or file CG, a scale's error, a tunnel balance's C_D error, a simulated speed against a
  real flight, or a logged trace RMS's own uncertainty; their rows stay unsourced until one does.
- How many of a model page's internal digits a reader needs is a writing call. ASTM E29-22 and ASME V&V 20-2009
  are paywalled: this note reads E29's 1990 text and V&V 20 not at all.

## Sources

All read 2026-11-05. Standards are cited, not quoted at length.
1. JCGM 100:2008 (GUM), §4.3.2, §7.2, Table E.1. <https://www.bipm.org/documents/20126/2071204/JCGM_100_2008_E.pdf>
2. Taylor and Kuyatt, NIST TN 1297 (1994). <https://nvlpubs.nist.gov/nistpubs/Legacy/TN/nbstechnicalnote1297.pdf>
3. NIST SP 811 (2008). <https://nvlpubs.nist.gov/nistpubs/Legacy/SP/nistspecialpublication811e2008.pdf>
4. ISO 80000-1:2022, Quantities and units, Part 1, §7.2.4 and Annex B (normative). ISO copyright.
5. ASTM E29-90, incorporated by reference in 40 CFR 86.000-28. <https://archive.org/details/gov.law.astm.e29.1990>
6. EURACHEM/CITAC Guide CG 4, QUAM:2012.P1, §9.5.1. <https://www.eurachem.org/images/stories/Guides/pdf/QUAM2012_P1.pdf>
7. Navas et al. (PDG), PRD 110, 030001 (2024). <https://pdg.lbl.gov/2024/reviews/rpp2024-rev-rpp-intro.pdf>
8. Oberkampf and Trucano, SAND2002-0529 (2002), pp. 14, 19, 43, 45. <https://www.osti.gov/servlets/purl/793406>
9. I. Celik, U. Ghia, P. Roache and C. Freitas, J. Fluids Eng. 130, 078001 (2008), with the journal's policy.
   <https://www.asme.org/wwwasmeorg/media/ResourceFiles/Shop/Journals/JFENumAccuracy.pdf>
10. Rust standard library, `f64`. <https://doc.rust-lang.org/std/primitive.f64.html>
11. MS5607-02BA03 datasheet (2024), pp. 1, 4, via [altimeter-sensor-noise](altimeter-sensor-noise.md).

[acc]: ../accuracy.md
[adr]: ../decisions/0296-what-the-words-leave-digits-and-the-command-lines-words.md
[i966]: https://github.com/nrdptel/fusionspace-eridanus/issues/966
[n1]: dual-altimeter-spread.md
[n2]: accel-scale-at-boost.md
[n3]: era5-surface-temperature.md
[n4]: dispersion-sigmas.md
[n5]: as-built-mass-growth.md
[n6]: hpr-flyer-mass-growth.md
[n7]: dtic-subsonic-finner-data.md
[n8]: hobby-rocket-tunnel-data.md
[n9]: flight-derived-cd.md
[n10]: hpr-lot-impulse.md
[n11]: hpr-burn-time-scatter.md
[n12]: altimeter-bay-temperature.md
[n13]: era5-wind-direction-us.md
[n14]: hobby-parachute-cd.md
