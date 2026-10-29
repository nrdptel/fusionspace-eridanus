# A fin set flying in another's downwash: cited methods and measured shares

This note asks how much normal force a rear fin set gives when it flies behind a forward one, as a booster's fins do
behind a sustainer's on an in-line two-stage stack. It follows
[hagerty-tunnel-lift](hagerty-tunnel-lift.md#5-two-fin-sets-in-line), whose booster fins added only 4–19% of their
own Barrowman slope, and serves
[#730](https://github.com/nrdptel/fusionspace-eridanus/issues/730). The share η used throughout is the rear set's lift
beside the body and the forward set, over its lift on the body alone (on Hagerty's model, over Barrowman's value for
it, as no booster-fins-only run exists). **Inference** marks this note's own arithmetic. Pages are printed pages
unless marked PDF.

**In short: every method and every dataset says the rear set lifts less than alone, so summing the sets, as Barrowman's
method and HPR Sim do, always reads a two-stage margin high, the flattering side.** Measured in-line shares near 0° are
0.06–0.28 for rectangular fins of equal span (Grigsby, Mach 1.62) and 0.04–0.19 on Hagerty's model rocket (Mach
0.03–0.09); delta fins keep 0.43–0.59, fins rotated 45° against the forward set (interdigitated) 0.62–0.84. The one
cited method with tandem validation, NACA Report 1307's rolled-up vortex (USAF DATCOM's Method 3, Missile DATCOM's
model), errs both ways: at Mach 1.62 it reads the rear set's share 0.04–0.35 *low* (its whole-model slope and CP fit by
compensating errors), while on Hagerty's close-coupled subsonic model it keeps 0.78 of Barrowman's value, three to
twenty times the measured increment. Jones's slender-body rule (0.29) and Hagerty's own outboard-span estimate (0.16)
come closer there. On an invented 56 mm stack the range of shares moves the center of pressure (CP) forward 0.7–4.5
calibers (1.0–4.5 in line). Confidence: high in the sign and the readings, medium in the hand calculations, low in
which method holds below Mach 1 at hobby spacing.

## 1. What was measured

| Dataset | Mach | Layout | Span ratio rear/forward | Gap (calibers) | η near 0° |
|---|---|---|--:|--:|---|
| Hagerty, configuration 1 [H] | 0.03–0.09 | cruciform, in line | 1.16 | 0–1 (assumed) | 0.04–0.19; a third at 23° |
| Grigsby, rectangular, in line [G51] | 1.62 | cruciform | 1.00 | 2.8–5.3 | 0.06–0.28 |
| Grigsby, delta and reversed delta, in line [G51] | 1.62 | cruciform | 1.00 | 3.2–5.3 | 0.43–0.59 |
| Grigsby, interdigitated (45°) [G51] | 1.62 | cruciform | 1.00 (1.42 for T6) | 2.8–5.3 | 0.62–0.84 |
| Spahr [S54] | 1.99 | cruciform, in line | 0.64 | about 2.8 | 0.74 (0.88 interdigitated) |
| Kassner, Wettlaufer [K77] | 0.80 | nose canards, tail | 0.96–1.2 | about 5 | 0.62 at 4°, 0.80 at 8°, 1.0 from 16° |
| Pierpont [P62] | 0.60–1.20 | planar stack | 1.20 | about 4–5 | stability share cut "more than one-half" |

Spans are total; gaps run trailing edge to leading edge. Hagerty's, Kassner's and Pierpont's gaps and Kassner's span
ratio are this note's estimates from drawings or assumed positions (**Inference**).

- **Grigsby, NACA RM L51J15 and RM L52H05 [G51, G52].** The one clean match to a two-stage stack: an ogive-cylinder
  body 0.700 in across and 9.00 in long, cruciform wing and tail sets of equal 2.00 in span (one 2.83 in tail, T6),
  forward set at two stations, in line (θ = 0°) or interdigitated (θ = 45°), Mach 1.62, Reynolds number 3.1 × 10⁶ on
  length, α −5° to 15° (L51J15 pp. 1, 4, Table I). Table III gives slopes for the body, body-wing, body-tail and the
  whole model, so η follows from them (**Inference**). Summing body-wing and body-tail, as a component-sum method
  does, exceeds the measured in-line slope 1.2–1.6 times (1.5–1.6 rectangular) and puts the CP 1.4–2.6 calibers too
  far aft for rectangular sets, 0.8–1.3 for deltas and 0.3–0.8 interdigitated. Three of the six rectangular in-line
  models measure their CP 0.18–1.28 calibers *ahead* of the moment point; the sum puts it 1.3–1.7 aft (**Inference**).
- **The loss fades with angle.** Grigsby's rectangular in-line η rises from 0.28 at 0° to about 0.47 at 4°, 0.72 at
  8° and about 0.85 at 12°, where the measured curve ends (L52H05 Fig. 4, p. 23, read by hand ±0.03), as the
  forward set's vortices rise off the rear fins' plane. Kassner's Mach 0.8 panel balances show the same rise.
  Hagerty's model rises more slowly, to a third at 23°.
- **A smaller rear set loses less.** Spahr's rear set sits well inside the forward span: η 0.74 in line, CP shift
  from summing only 0.22 caliber (Table II; **Inference**). Hagerty's and the invented stack's booster fins span as
  much or more than the forward set, the Grigsby case.
- **Subsonic.** No subsonic dataset was found with a full body, body-wing, body-tail, whole-model build-up for two
  equal fin sets; Hagerty stays the only low-speed one. Kassner's canards are small nose deltas carrying about 54% of
  the isolated tail's load at 8° (**Inference**), so they shed weaker vortices than sustainer fins would. Pierpont's
  second-stage fins lost more than half their stability share behind a winged spacecraft, which the author puts down
  to the spacecraft's interference, adding that the wing wake's lower dynamic pressure would also account for it
  (pp. 5–6); no fins-off run separates η.

## 2. NACA Report 1307: one rolled-up vortex per forward panel [P57]

- **The model.** Each forward panel sheds one line vortex of strength
  Γ = V∞ K_W(B) α (C_Lα)_W S_W / [4(f_W − r_W)] (eq. 38, p. 12), where K_W(B) is the forward panels' lift factor on
  the body, (C_Lα)_W and S_W the slope and area of the forward panels joined, r the body radius and f the vortex's
  lateral position, within about 3% of π/4 of the exposed span from slender-body theory (eq. 39) or from charts 5
  and 6 for real planforms. It trails along the free stream, so its height at the rear set is
  h = (distance from the forward trailing edge) × sin α (eq. 41, p. 13): at α → 0 it lies in the plane of in-line rear
  fins. Image vortices inside the body and strip theory give a tail interference factor i (eqs. 42, B4, B13), and the
  rear set's lift loss is (C_Lα)_W (C_Lα)_T K_W(B) α i (s_T − r_T) / [2π A_T (f_W − r_W)] (eq. 55, p. 15).
- **Inputs.** Only geometry and the two sets' slopes: i depends on geometry alone, so the method's Mach dependence
  comes through the forward set's slope and vortex strength.
- **Limits it states.** Small α; no swept-back trailing edges or swept-forward leading edges; body vortices or more
  than one vortex per panel at high α invalidate it (p. 30). Strip theory errs up to about 35% when the vortex passes
  just inboard of a rectangular tail's tip, under 5% outboard (Fig. 12, p. 14). In the transonic range nonlinearities
  "may cause the method to fail" (p. 30).
- **What it was checked against.** Table III: 24 wing-body-tail models, Mach 0.20–1.99, overall lift within about
  ±10% and CP within ±0.02 body length (summary, p. 1). Its equal-span tandem rows (115–124) are ten of Grigsby's
  Mach 1.62 models: the theory's rear-set share is 0.02–0.43 against Grigsby's measured 0.06–0.59 (L51J15 Table III),
  low by 0.04–0.35 on all ten, while the whole-model CP lands within −0.51 to +0.26 calibers where summing reads
  1.07–2.73 aft (**Inference** from both tables). Grigsby's own version of the method (Lagerstrom–Graham vortex
  positions in line, measured ones interdigitated; reversed-flow tail loads) brought all 13 configurations' slopes
  within −0.8% to +7.3% and the CP within 0.2 caliber on average, 0.4 at worst (L52H05 pp. 12–13), an agreement he
  calls "somewhat fortuitous" because the body-wing and body-tail errors cancel.
- **DATCOM's use [D78].** USAF DATCOM §4.4.1 gives three downwash methods. Method 1 needs a forward span at least 1.5
  times the rear one's (p. 4.4.1-3); Method 2's empirical gradient was fitted over aspect ratio 2–8 and span ratio
  0.28–0.80 (p. 4.4.1-7). A two-stage stack fits neither: Hagerty's forward set has aspect ratio 1.8 and span ratio 1.16
  (**Inference**). Method 3, "for configurations in which the span of the forward surface is approximately equal to or
  less than that of the aft surface," is NACA 1307 (p. 4.4.1-8), with a warning that a vortex trailing very close to
  the aft panel gives "erroneous answers" (p. 4.4.1-2).
- **Missile DATCOM [MD98]** uses the same line-vortex model for any fin set behind another (Table 33, p. 102), but
  notes that a sheet "does not fully roll up until it is at least four semispans downstream" and that closer sets may
  be in error with "too much vorticity" (p. 14). **Inference:** four semispans of Hagerty's upper fins is about 12 in,
  7.5 calibers; the gaps in sections 4 and 5, 0–5.9 calibers, fall short of that.

## 3. Flat-sheet limits

- **Jones's slender-wing rule [J46].** A slender wing's lift depends only on how its span grows downstream: "sections
  behind the section of maximum width develop no lift" (NACA Report 835, summary). **Inference:** on a body with two
  fin sets the rear set adds only the growth of 2πs²(1 − r²/s²)² from the forward span s_W to its own s_T, a share of
  1 − [s_W(1 − τ_W²)]² / [s_T(1 − τ_T²)]² with τ = r/s; zero for equal spans.
- **DATCOM's flat wake.** Behind a lifting surface the downwash gradient dε/dα runs from 1 at the trailing edge to
  2C_Lα/(πA) far downstream (eq. 4.4.1-a). **Inference:** for Hagerty's upper fins (aspect ratio 1.8, C_Lα 2.41 per
  rad) that is 0.85, or 1.04 with K_W(B) = 1.22, so rear-fin span inside the forward span keeps 0–15% of its angle.
- **Hagerty's reasoning [H].** The upper fins' downwash nearly cancels α behind them, so only booster-fin span beyond
  them lifts (printed p. 7). **Inference:** that strip is 15.7% of a booster fin's chord-weighted area.

## 4. What each method predicts for Hagerty's configuration 1 (Inference)

Inputs from the [hagerty-tunnel-lift](hagerty-tunnel-lift.md) note: body 1.625 in; upper fins root 3, tip 2, span
2.25 in; booster fins root 3.5, tip 2.5, span 2.75 in, in line; Barrowman gives the booster fins 20.6 per rad on the
body. NACA 1307 puts the vortex 2.52 in from the axis, 71% of the booster fins' semispan, and gives i = −1.98: a
share of 0.63 of its own slender-body slope for the set (25.3 per rad), or 0.78 of Barrowman's. Shares below are of
Barrowman's 20.6, the base of the measured values.

| Method | Share η | Increment, per rad | Against the measured 0.8–3.9 |
|---|--:|--:|---|
| Summed (Barrowman, HPR Sim) | 1.00 | 20.6 | 5–26 times high |
| NACA 1307, DATCOM Method 3 | 0.78 (0.62 with a 35% strip error) | 16.0 (12.7) | 3–20 times high |
| Jones's flat-sheet limit | 0.29 | 6.0 | 1.5–7.5 times high |
| Hagerty's outboard span | 0.16 | 3.2 | inside |

At 23° the vortex rises 0.27–0.97 in off the plane and NACA 1307 keeps 0.80–0.89 of Barrowman's slope against a
measured 0.33: right in the direction of change, wrong in size. At Mach 1.62 the same method reads the share low, so
the miss here is most likely the close coupling (a sheet not yet rolled up, Missile DATCOM's warning) and the low
speed. That reading is not proven: Hagerty's fin positions are assumed (his Fig. 1 is missing from the scan), the
chord Reynolds number is about 5 × 10⁴, and this is one test.

## 5. How far each moves a two-stage stack's margin (an invented stack; Inference)

A 56 mm body, 4:1 ogive nose (C_Nα 2, CP 104 mm), four fins per set with root 120, tip 60 and span 90 mm, the tip's
leading edge 60 mm aft of the root's, trailing edges at each stage's end; sustainer 1,100 mm. Barrowman gives each
set 20.85 per rad. A share below 1 moves the CP forward, so the margin falls by the same calibers whatever the CG.

| Case | Summed CP from nose | Share and its source | CP moves forward |
|---|--:|---|--:|
| A: 450 mm booster, same fins, gap 5.9 cal | 21.47 cal | 0.42, NACA 1307 at Mach 0 | 1.88 cal |
| | | 0.66, NACA 1307 at Mach 2.5 | 0.98 cal |
| | | 0.59 to 0.06, Grigsby's measured in-line range | 1.2 to 4.0 cal |
| | | 0.19, Hagerty's top value; 0, Jones | 3.12; 4.52 cal |
| | | 0.75, interdigitated (Grigsby) | 0.67 cal |
| B: 250 mm booster, same fins, gap 2.3 cal | 19.77 cal | 0.42, NACA 1307; 0, Jones | 1.17; 2.82 cal |
| C: 450 mm booster, fins 140/70/110 mm | 22.20 cal | 0.57, NACA 1307; 0.29, Jones | 1.30; 2.69 cal |

NACA 1307's share on case A by Mach: 0.42 (0), 0.36 (0.8), 0.38 (1.5), 0.66 (2.5), scaled by the forward set's slope
(Diederich's formula below Mach 1, 4/β × [1 − 1/(2βA)] above, an approximation). At α → 0 it doesn't depend on the gap.
Against a 1–2 caliber margin, a forward shift of 1–4.5 calibers can leave a stack that the sum calls stable neutral
or unstable, as Grigsby measured on three in-line models (section 1).

## 6. What other tools document

The OpenRocket technical documentation (v13.05) sums components for C_Nα and the CP (eq. 3.29, PDF p. 34), treats
fin-fin interference only within one set (PDF p. 42), and has no term for a set behind another (searched for
"downwash", "wake", "tandem", "interference") [OR13]. No RASAero II or RockSim text on tandem sets was reached. An
Apogee newsletter letter repeats Stine's advice to interdigitate the two stages' fins, with no number [A97].

## What this suggests

- **State the limit now.** A margin with one fin set behind another reads high by a cited amount: on the invented
  stack, 1.0–4.5 calibers for in-line sets and 0.7 interdigitated; on Grigsby's models, 0.8–2.6 in line and 0.3–0.8
  interdigitated, measured [H, G51]. That serves the first item of #730's done-when.
- **No cited method is ready to model it below Mach 1.** NACA 1307 is the only method validated on equal-span
  tandem sets, and only at Mach 1.62, where it reads the share low; at Hagerty's spacing and speed it reads 3–20 times
  high in increment, so a model built on it alone would stay on the flattering side there. For equal spans Jones's
  share of 0 is a bound on the safe side for the margin; for a larger rear set it read high on Hagerty (0.29 against
  0.04–0.19), so a bound needs the measured lower edge too; either may suit #730's decision record.
- **A pin for #730's test.** Grigsby's Table III gives three measured in-line cases where the sum puts the CP aft of
  the moment point and the model has it ahead; with Hagerty's configuration 1, four cases to pin the error's sign on.

## Open questions

- **Subsonic tandem data at 2–10 calibers** with a full build-up: none found besides Hagerty's one close-coupled,
  low-Reynolds-number test. Rainey's Langley series (RM L9L30, L50G07, L50I29a, Mach 1.62–2.40) has build-ups with
  span ratios of 1 and below, as graphs only; not digitized here.
- **Partial rollup between close sets:** neither method bounds it, and Missile DATCOM gives no sign.
- **Interdigitated subsonic shares and three-fin sets:** no data read. Nielsen's *Missile Aerodynamics* (cite only)
  was not read.
- **Hagerty's fin positions:** his Fig. 1 is missing, so the gap and the CP shift on his model rest on assumptions.

## Sources

Read 2026-10-29. NACA and NASA reports and both DATCOM manuals are US government works approved for public release;
Hagerty's report (a NAR member benefit), the OpenRocket documentation and the Apogee newsletter are cite only.

1. **[H]** J. J. Hagerty, "Low Speed Wind Tunnel Tests of a Small Rocket", 1975; NAR R&D report, NARAM-52, 2010, as
   read in the [hagerty-tunnel-lift](hagerty-tunnel-lift.md) note.
2. **[G51]** C. E. Grigsby, NACA RM L51J15, 1952. <https://ntrs.nasa.gov/citations/19930087120>
3. **[G52]** C. E. Grigsby, NACA RM L52H05, 1952. <https://ntrs.nasa.gov/citations/19930087562>
4. **[P57]** W. C. Pitts, J. N. Nielsen, G. E. Kaattari, NACA Report 1307, 1957.
   <https://ntrs.nasa.gov/citations/19930091008>
5. **[J46]** R. T. Jones, NACA Report 835, 1946. <https://ntrs.nasa.gov/citations/19930091913>
6. **[D78]** R. D. Finck, *USAF Stability and Control DATCOM*, 1978 revision, DTIC ADB072483, §4.4.1.
7. **[MD98]** W. B. Blake, *Missile DATCOM User's Manual, 1997 Fortran 90 Revision*, AFRL-VA-WP-TR-1998-3009.
8. **[S54]** J. R. Spahr, NACA RM A54H27, 1954. <https://ntrs.nasa.gov/citations/19930088413>
9. **[K77]** D. L. Kassner, B. Wettlaufer, NASA TM X-73218, 1977. <https://ntrs.nasa.gov/citations/19770021127>
10. **[P62]** P. K. Pierpont, NASA TM X-677, 1962. <https://ntrs.nasa.gov/citations/19660024055>
11. **[OR13]** S. Niskanen, OpenRocket technical documentation, v13.05, 2013 (documentation only).
    <https://openrocket.sourceforge.net/techdoc.pdf>
12. **[A97]** Apogee Components, *Peak of Flight* newsletter 97, 2003, p. 2.
    <https://www.apogeerockets.com/education/downloads/Newsletter97.pdf>
