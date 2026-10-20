# Low-subsonic static tests of finned bodies on DTIC

[hobby-rocket-tunnel-data](hobby-rocket-tunnel-data.md) found no tunnel normal-force slope (C_Nα) or center of
pressure (CP) for a hobby-shaped rocket. Of its five sources usable below Mach 0.6, only Cranfield's (Mach 0.12,
issue #611) is tabulated; NAL TR-814 (Mach 0.50) and RM L53J06 (Mach 0.50–0.88) are fully dimensioned but plotted.
This note checks the DTIC leads it couldn't reach: RAE TN Aero 2811, the Basic Finner's low-speed static tests, and
others found on the way. DTIC was "Under Maintenance" on 2026-10-19 (HTTP 403 to scripts), so every report here is
the Internet Archive's copy of DTIC's scan. **Inference** marks this note's own readings and arithmetic.

**In short: RAE TN Aero 2811 has no fins. The Basic Finner has three independent low-speed measurements that put its
CP at 8.0–8.1 calibers from the nose. A 1976 AEDC compilation holds a second finned rocket at Mach 0.59, tabulated
and fully dimensioned, with the fins on their own balances.**

- **Kettle's rocket is finless,** a spiked body plotted only, with one point in the band, at Mach 0.4.
- **The Basic Finner at low speed.** A 1957 Caltech water-tunnel test (Mach about 0) gives C_N about 0.97 at 4°
  (13.9 per rad, a secant) and CP 8.04 d, 7.90–8.18 over the line width (Inference, its Fig. 4). A 1968 Notre Dame
  free-oscillation test tabulates C_Nα 12.9 and C_mα −40.1 about mid-length: CP 8.11 d (Inference). ARL's replot of
  DRDC's tunnel set gives 15.0 per rad and 8.14 d at Mach 0.5
  ([transonic-normal-force](transonic-normal-force.md#the-basic-finner)); DRDC's tables weren't reached.
- **AEDC-TR-76-58** tested the Basic Finner at Mach 0.22, but its small-angle error (C_N ±0.64) is about half the
  signal at 5°; it brackets the CP at 7.5–8.1 d and no more.
- **The Martin Generalized Research Model** (AEDC-TR-75-125) at Mach 0.59, body B3 with fins F15, measures C_Nα
  6.0–6.4 per rad and CP 6.4–6.6 d ([its own part](dtic-subsonic-finner-data-grm.md)).
- **The linear fin equations HPR Sim uses** give the Basic Finner 10.84 per rad and CP 7.89 d: C_Nα 16–22% below the
  low-speed measurements, the CP 0.15–0.22 caliber ahead. On the Martin model they give 4.52 per rad and 5.68 d:
  24–29% low and 0.7–0.9 caliber ahead. Its fin panels match the model within −3% to +6%; the gap is the body's.
  Both read the margin *low*, Cranfield's sign (Inference).

Confidence: high that the Basic Finner's CP is near 8.1 d at low speed (three methods' central values agree within
0.10 caliber); medium on its C_Nα (12.9 to 13.9) and on the Martin model's readings, from a worn scan. Neither is a
hobby shape: they add two measured references to #611's one but can't size #172's 0.02–0.06 caliber CP gap.

## 1. RAE TN Aero 2811 (Kettle 1962): a finless rocket

*8 ft × 6 ft transonic wind tunnel tests on a 0.7 scale model of a 6½ in unguided rocket*, D. J. Kettle, RAE
Farnborough, February 1962, 27 PDF pages [K]. DTIC's cover: "Approved for public release; distribution is
unlimited", downgraded from Confidential on DSTL's authority of 9 Dec 2008 (PDF p. 1). Crown copyright, with an
unclear term for an unpublished 1962 report: cache, cite, commit only readings.

- **No fins.** A nose spike and a base plate (bourrelet) replace the "more usual" tail fins (PDF p. 6). Fig. 1 (PDF
  p. 10) dimensions all but the spike's tip rod and the base step's position.
- **Conditions.** Mach 0.4 to 1.24, −2½° to 10°, 2 × 10⁶ per foot, no tunnel-constraint corrections (PDF p. 6): Re
  about 4.7 × 10⁶ on the 28 in length (Inference). Mach 0.4 is the one plotted point in the band (Fig. 6, PDF p. 15).
- **References.** Area 0.1129 ft² and length 0.379 ft (text, printed p. 4, PDF p. 6). Table 1 (PDF p. 9) puts the
  moment axis 14.38 in (3.16 calibers) from the spike tip, on a 4.55 in diameter with a 5.01 in bourrelet.
- **Values (Inference, by eye).** At Mach 0.4 the four slopes are 0.046–0.052 per degree (2.6–3.0 per rad; Fig. 6)
  and the CP 3.2–4.0 calibers from the tip (Fig. 15): a finless body-lift check at most.

## 2. The Basic Finner at low speed

A 10° cone 2.836 d long on a cylinder, 10 d overall, four 1 d × 1 d fins flush with the base, wedge-section to 0.08 d
([finner note](finner-reference-data.md#1-the-basic-finner)); HPR Sim can't draw the wedge. All on πd²/4 and d.

### Kermeen 1957: Caltech water tunnel, fully wetted

*Static force coefficients of the Basic Finner missile in fully wetted flow*, R. F. Kermeen, Caltech Hydrodynamics
Laboratory Report E-73.2, September 1957, Navy contract NOrd-16200 [W].

- **Terms.** DTIC's cover: approved for public release (USNOSC letter, 5 Sep 1969; PDF p. 1); the title page permits
  US Government reproduction (PDF p. 3). A university contract report: cache, cite, commit only readings.
- **Test.** 1 in and 2 in models (the 2 in one 20 in long) on a three-component balance in a 14 in water tunnel,
  strut interference subtracted by image runs (PDF pp. 5–7). No cavitation: the incompressible limit, Mach about 0.
- **Coefficients.** Lift, drag and pitching moment about the midpoint, 5.00 d from the tip (PDF p. 7), plotted in
  Fig. 4 (PDF p. 15) at Re 1.7, 2.5, 3.4 and 5.1 × 10⁶ to 14°. Fig. 3 (PDF p. 14) spans Re 0.7–8 × 10⁶ for 4–65 ft/s,
  so its length is the 20 in body (Inference). Lift and moment hardly vary with it (PDF p. 8).
- **Geometry.** Fig. 1 (PDF p. 11) dimensions the cone (2.836 d), span (3.000 d), trailing edge (0.080 d) and moment
  center; the chord's label is illegible in the scan, drawn about 1 d.

Read by pixel calibration of Fig. 4 (Inference). The C_L and C_m columns are the outer edges of the inked lines at
that angle, all Reynolds numbers together. C_N takes the C_L band's center times cos α plus C_D sin α. The CP is at
the C_m band's center, with the band's edges in brackets.

| α, deg | C_L | C_m about 5.00 d | C_N | C_N/α, per rad | CP from nose, d |
|---:|---:|---:|---:|---:|---:|
| 2 | 0.41–0.51 | 1.28–1.77 | 0.48 | 13.6 | 8.20 (7.69–8.72) |
| 4 | 0.89–0.98 | 2.81–3.08 | 0.97 | 13.9 | 8.04 (7.90–8.18) |
| 6 | 1.42–1.49 | 4.61–4.88 | 1.51 | 14.4 | 8.15 (8.06–8.24) |

C_D is 0.46 at 0° and about 0.5–0.58 from 4° to 6°. At 2° the moment lines haven't merged, so that row is the least
certain. A restoring moment is printed positive. A dynamic-balance test in the same tunnel [W2] reports very good
agreement with static balance tests (printed p. 10, PDF p. 14); plotted only, not read here.

### Nicolaides et al. 1968: Notre Dame, free oscillation

*Flight dynamics of the Basic Finner in various degrees of freedom*, J. D. Nicolaides, R. S. Eikenberry, C. W. Ingram
and T. A. Clare, University of Notre Dame, AFATL-TR-68-82, July 1968 [N].

- **Terms.** Public release (AFATL letter, 8 Mar 1977, PDF p. 1), superseding the export notice; cite, commit readings.
- **Test.** A model free to pitch on a side support (ball bearings, then a jewel pivot) in a horizontal tunnel "at
  low subsonic speeds" (printed p. 5, PDF p. 14). Speed, model size and Reynolds number are not stated.
- **Data, tabulated.** Table I (printed p. 9, PDF p. 18) fits six pivot positions: C_mα −40.1 (probable error
  ±0.168) and C_zα −12.9 (±0.126) at 5.0 calibers from the base. It prints no unit; per rad is inferred. So C_Nα is
  12.9 per rad and the CP 5.0 + 40.1/12.9 = 8.11 d from the nose, ±0.03 caliber (Inference).
- **Caveat.** The values fit oscillations that start at about 24–25° (Fig. 3, PDF p. 16), and the report calls C_mα
  "quite non-linear". Fig. 6 (PDF p. 18) shows it falling from about 10.0 at 4° RMS to 9.2 at 7°, about a pivot it
  doesn't state, so only the relative drop (about 8%) carries over. Table I is a mean over amplitude, not a slope at 0°.

### Jenke 1976: AEDC-TR-76-58, Mach 0.22

*Experimental roll-damping, Magnus, and static-stability characteristics of two slender missile configurations at high
angles of attack (0 to 90 deg) and Mach numbers 0.2 through 2.5*, L. M. Jenke, ARO Inc., AEDC, July 1976 [A].

- **Terms.** Approved for public release (form DD 1473, PDF p. 3); Air Force work by AEDC's contract operator.
- **Test.** The Basic Finner (d = 1.8 in, fins canted 5°, spinning about 33 rad/s) in VKF Tunnel A run subsonic, at
  Mach 0.22, Re_d 1.05 × 10⁵ (about 1.05 × 10⁶ on length), −5° to 83° (PDF p. 12). Moment reference 6.1 d; the drawn
  chord is 0.94 d (Fig. 2, PDF p. 25). The Modified Basic Finner starts at Mach 0.6.
- **Precision.** Near minimum loads at Mach 0.22: C_N ±0.64, C_m ±0.70 (PDF p. 17). From Mach 0.6, C_N is ±0.17 or
  better, but C_m is ±0.29 at Mach 0.6 and Re_d 2.6 × 10⁵ (±0.18 at 4.17 × 10⁵) and ±0.22 at Mach 0.9.
- **Values (Inference, Fig. 18a, PDF p. 91).** C_m about −1.8 at 5.0° and −3.7 at 10.0°; the Mach 0.22 C_N symbols
  hide under the Mach 2.49 ones below about 35°. With Kermeen's C_N (1.22 at 5°; 2.64 at 10°, from C_L 2.47–2.59 and
  C_D 0.85, his Fig. 4) the CP is 7.5–7.6 d; with the Mach 2.49 C_N (1.9 at 10°), 8.05–8.07 d. Plotted only.

### DRDC TM 2002-136 (Dupuis 2002): not reached

ARL-TR-6725 [R] (printed p. 10, PDF p. 20) gives its tunnel range as Mach 0.5–4.5 at Re 1.4–2.9 × 10⁶, length basis
not stated, with coefficients from fitted polynomials. Only the replot was read: 15.0 per rad and CP 8.14 d at Mach
0.5. DTIC was down, and search found no Canadian government copy. Canadian Crown copyright: cite, commit only readings.

### The three low-speed sets side by side (Inference)

| Source | Mach | Re on length | Method | C_Nα, per rad | CP from nose, d |
|---|---:|---|---|---:|---:|
| Kermeen 1957 | about 0 | 1.7–5.1 × 10⁶ | static balance, water | 13.6–14.4 (secant, 2–6°) | 8.04 (7.90–8.18) |
| Nicolaides 1968 | low, not stated | not stated | free oscillation | 12.9 ± 0.13 | 8.11 ± 0.03 |
| Jenke 1976 | 0.22 | 1.05 × 10⁶ | static balance | not readable | 7.5–8.1 |
| Dupuis 2002 (replot) | 0.5 | 1.4–2.9 × 10⁶, basis not stated | tunnel | 15.0 ± 0.3 | 8.14 |

The rise from 13.9 to 15.0 between Mach 0 and 0.5 is +8%. Plain 1/β gives +15%. The fin formula in
[aero](../physics/aero.md#fins) has β inside its square root: one fin goes from 3.314 to 3.444 per rad (+3.9%), the
linear terms +3.2% (Inference).

## 3. The Martin Generalized Research Model at Mach 0.59

AEDC-TR-75-125 (1976, five volumes, public release since 1979 [G], [G2]) says Mach 0.20 to 4.63. Test 1's points
below Mach 0.8 are body alone (Table 7, PDF p. 528), but Test 2's Reynolds-number study (Table 8, printed p. 526, PDF
p. 529) adds body B3 with fins F15 at Mach 0.59: parts 24–50, 0.3 to 3.4 × 10⁶ per foot (1.0 to 11 × 10⁶ on the
40 in length), fixed and free transition, roll 0° and 180°, tabulated in Volume II, fully dimensioned, each fin on
its own balance. At 3.4 × 10⁶ per foot, C_Nα is 6.0–6.4 per rad and the CP 6.40–6.57 d, against the linear terms'
4.52 and 5.68 d. The fin panels match; the missing 1.5–1.7 per rad sits near the fins (Inference). Details and pages
are in [the part note](dtic-subsonic-finner-data-grm.md).

## 4. Other DTIC tests screened

- **Regan, NAVORD Report 6652:** roll damping only. The finner note quotes it at Mach 1.53–2.76; AEDC-TR-76-58
  reports agreeing with it at Mach 0.22 (PDF p. 18).
- **Not found:** the National Bureau of Standards tunnel test that Kermeen's introduction cites (PDF p. 5), and Notre
  Dame's ref. 17 (PDF p. 44), a subsonic tunnel test of the Apache sounding rocket "pending publication" in 1968.

## 5. What the linear fin model gives the Basic Finner (Inference)

HPR Sim's fin equations ([aero](../physics/aero.md#fins)) at Mach 0, for four 1 d × 1 d fins on a 1 d body: one fin
2π(1)²/(π/4) / (1 + √2) = 3.31 per rad; four fins, sum of sin² = 2, so 6.63; the interference factor 1 + 0.5/1.5 =
1.333 gives 8.84. A cone nose adds 2.0 at 1.89 d (two-thirds of its length). The fins' CP is a quarter chord aft of
the root, 9.25 d. Total: C_Nα 10.84 per rad, CP (2.0 × 1.89 + 8.84 × 9.25)/10.84 = 7.89 d. At Mach 0.5 the same
terms give 11.18 per rad and 7.93 d. HPR Sim's full model adds body lift and its own nose treatment; not bounded here.

- Against Nicolaides' 12.9 that is −16%, and −22% against Kermeen's 13.9 at 4°. The linear sum leaves out body lift,
  which raises the measured value; the wedge section's effect has no sign here. The split isn't bounded.
- The CP is 0.22 caliber ahead of Nicolaides' 8.11 d, 0.15 of Kermeen's (0.01–0.29 over his band) and 0.21 of DRDC's
  at Mach 0.5: the margin reads low, the safe side. This repeats Cranfield's sign, where theory put the CP 0.34–0.70
  caliber ahead ([hobby-rocket-tunnel-data](hobby-rocket-tunnel-data.md#2-government-and-academic-tests-below-mach-06)).

## What this suggests

1. **A subsonic Basic Finner case.** Fly it (`Square` fins) at Mach 0.1 and 0.5 and α 2°–6° against Nicolaides'
   Table I, Kermeen's Fig. 4 readings and the DRDC replot, as the Arcas fixture does: readings with page and figure,
   the gap with its sign. Expected effect: measured subsonic C_Nα and CP of a finned body beside Cranfield's point
   (#611), at Re 10⁶ to 5 × 10⁶ on length. Trust: CP ±0.1 caliber; C_Nα ±5%, from the 12.9 to 13.9 spread and the
   amplitude-mean caveat. The wedge fins can't be drawn, so a thickness effect stays in the gap; state it.
2. **A Martin model case.** Fly B3W0F15 at Mach 0.59 against parts 33 and 50 up to α 5°. Its fin balances split the
   fin term from the body term, which no other source here does. Expected effect: C_Nα 24–29% low and the CP 0.7–0.9
   caliber ahead in the linear terms, most of it in the body's share. Trust: CP ±0.1 caliber, C_Nα ±4%.
3. **On #172.** The references resolve about 0.1 caliber, more than #172's 0.02–0.06 caliber, so they can't decide
   it. They do say the linear CP sits 0.15–0.9 caliber ahead of measured ones, most on the small-finned body.
4. **Drop RAE TN Aero 2811** from the finned-rocket search; keep it as a finless body-lift point at Mach 0.4.

## Open questions

- **DRDC TM 2002-136's tables** and moment reference: DTIC was down on 2026-10-16 and 10-19; a retry would replace
  the replot reading at Mach 0.5.
- **Notre Dame's speed and Reynolds number** are not in the report; Nelson's 1966 thesis (its ref. 5) may give them.
- **Kermeen's Reynolds-number length** is inferred, his wall correction isn't described, and his fin chord is
  illegible (DREV and NOL give 1.0 d, AEDC's drawing 0.94 d).
- **How much of the 16–22% C_Nα shortfall is body lift** and how much is the wedge section is not bounded.
- **The Martin model's body share:** whether the 1.5–1.7 per rad near its fins is carryover lift is not sized here.

## Sources

All read 2026-10-19 from the Internet Archive's DTIC mirror; cached under `refs/research/dtic-subsonic-finner-data/`,
not committed. DTIC's own pages (<https://apps.dtic.mil/sti/citations/AD0329780>) were "Under Maintenance" that day.

- **[K]** D. J. Kettle, RAE Technical Note Aero 2811, 1962. <https://archive.org/details/DTIC_AD0329780>
- **[A]** L. M. Jenke, AEDC-TR-76-58, 1976. <https://archive.org/details/DTIC_ADA027027>
- **[W]** R. F. Kermeen, Caltech Hydrodynamics Lab. Report E-73.2, 1957. <https://archive.org/details/DTIC_AD0146516>
- **[W2]** T. Kiceniuk, Caltech Hydrodynamics Lab. Report E-73.3, 1957. <https://archive.org/details/DTIC_AD0142815>
- **[N]** J. D. Nicolaides et al., AFATL-TR-68-82, 1968. <https://archive.org/details/DTIC_AD0852114>
- **[G]** G. R. Gomillion, AEDC-TR-75-125, Vol. I, 1976. <https://archive.org/details/DTIC_ADB010053>
- **[G2]** G. R. Gomillion, AEDC-TR-75-125, Vol. II, 1976. <https://archive.org/details/DTIC_ADB010054>
- **[R]** V. A. Bhagwandin, J. Sahu, ARL-TR-6725, 2013. <https://archive.org/details/DTIC_ADA592550>
- A. Dupuis, DRDC Valcartier TM 2002-136, 2002 (not reached; cited by [R] as ref. 27, printed p. 35, PDF p. 45).
