# A round-nosed plate's leading-edge drag at Mach 0.8, from Tain and Cumpsty's own test

**In short: reduced from Tain and Cumpsty's test (Inference), an unswept semicircular leading edge on a 2% fin
comes to about 0.18–0.38 at Mach 0.8 when the chord Reynolds number is 1 × 10⁶, and 0.09–0.27 at 1 × 10⁷.** That
widens [thin-edge-mach-dependence](thin-edge-mach-dependence.md)'s 0.23–0.36 and, for larger or faster fins, moves
it down; the rule's 0.53 stays high by 0.15–0.44. The test was one 6 mm plate with a 3 mm nose radius in a free jet,
at a nose Reynolds number of 12,000: a 2% fin at 1 × 10⁶ on chord matches it, one at 1 × 10⁷ is 8 times past it.
The paper has the nose flow supersonic from Mach 0.7; its Fig. 7 interpolates to sonic near 0.64 (Inference). A
laminar bubble sits on the nose up to Mach 0.90. The spread comes from repeat runs that differ by up to 0.07 in
θ/r, from laminar or turbulent friction, and from scaling past the tested Reynolds numbers. "Inference" marks this
note's arithmetic; figures are read by pixel.

## 1. What was tested

- **Rig.** A closed-circuit transonic facility whose density sets Reynolds number apart from Mach number. One
  airfoil sat between side walls in a free jet 102 mm wide and 140 mm high, open at top and bottom [1] p. 224,
  Fig. 3: an isolated plate, not a cascade.
- **Plate.** 6 mm thick, with a semicircular nose of radius r = 3 mm, a flat forward part 103 mm long, then a 100 mm
  hinged flap drawn tapering to a sharp edge [1] p. 224, Fig. 3. The nose meets the flat 90° round, at surface
  distance s/r = 1.57; taps sit every 5° round the nose and along the flat to s/r 20 [1] p. 224.
- **Shapes and Mach.** A circular nose only; elliptic ones appear only in the low-speed work it summarizes [1]
  p. 223. Inlet Mach 0.5, 0.6, 0.65, 0.7, 0.75, 0.8, 0.85, 0.88, 0.9 and 0.95 [1] p. 226, Table 1.
- **Reynolds number.** R_r = U r/ν at inlet static conditions: about 12,000 for most runs, 8,000–20,000 in the
  Reynolds-number series at Mach 0.6, 0.75, 0.85 and 0.90 [1] pp. 224, 234.
- **Incidence.** "Zero" meant an effective incidence under 1.5° [1] p. 225. Table 1 lists −1.4° to −0.8°, −0.9°
  at Mach 0.8 and −0.0° at 0.85 [1] p. 226.
- **Turbulence.** The tunnel's own level is not stated; a grid gave 3.5–4% r.m.s. in earlier use [1] pp. 224–225.
- **Loss measured.** A boundary-layer traverse on the upper face at s/r = 17.9, with a flattened pitot 0.36 mm deep;
  the flow there is subsonic [1] p. 224. It gives the compressible displacement thickness δ* and momentum thickness
  θ, each plotted over the nose radius r [1] p. 230, Fig. 9.
- **Computed.** MSES, an Euler solver with a coupled integral boundary layer, with transition placed where the
  measured recovery starts and turbulent stress tuned with Mach number. The authors call the results "not
  predictions" but a way to read the flow [1] pp. 225–226, 230. The drag numbers below rest on measurements.
- **What θ holds (Inference).** The station is 17.9 r from the nose, about half the flat forward part. θ there holds
  the nose's pressure loss, the bubble's mixing loss and friction over the first 17.9 r; no trailing-edge loss, no
  friction past the station, and no loss in a shock outside the layer, where the traverse stops.
- **The plate's shape against a hobby fin (Inference).** The plate is 3.0% of its whole 203 mm chord and 5.8% of
  the forward part; on chord its Reynolds number is 12,000 × 203/3 ≈ 8 × 10⁵. The nose flow scales with r, and the
  paper's own premise is that the nose geometry, incidence and R_r fix it [1] p. 223. A 2% fin is 100 r long, so
  R_r = Re_c/100: 1 × 10⁴ to 1 × 10⁵ for Re_c 1 × 10⁶ to 1 × 10⁷.

## 2. When the nose goes sonic, and what forms

- **Onset near Mach 0.64.** The paper says the flow goes supersonic at inlet Mach 0.7 [1] pp. 221, 240; the spike
  is "close to sonic" at 0.6 [1] p. 227. Fig. 7's peak isentropic Mach number reads 0.94 at inlet 0.6, 1.10 at 0.7,
  1.15 at 0.75 and 1.22 at 0.8 [1] p. 228 (pixel reading, ±0.01). Straight interpolation puts sonic at inlet Mach
  0.64 (Inference); the taps limit how well the peak is resolved [1] p. 227, so onset may be at or below that.
- **It agrees with the low-speed peak.** The summarized low-speed work found the bubble-laden circular nose peaking
  at 1.3 times the free-stream speed at zero incidence, 1.4 with raised turbulence [1] p. 223. That is C_p −0.69
  and −0.96, which Kármán–Tsien carries to sonic at Mach 0.65 and 0.59 (Inference, method as [3] §4). A
  bubble-free nose would peak at C_p −1.54 and go sonic at 0.51 [3] §4; the bubble lowers the peak [1] p. 222.
- **Where.** Above inlet Mach 0.7 sonic speed is reached about 53–60° round the nose, before the 90° blend
  [1] pp. 227, 242. The flow then expands; waves reflected off the sonic line keep the peak below a simple
  expansion's, about 1.7 at inlet 0.95 rather than 2.0 [1] p. 227.
- **Bubble and shock.** A laminar separation near the blend persists up to inlet Mach 0.90 and is gone at 0.95
  [1] p. 241. At 0.85 the schlieren shows a shock closing the pocket and meeting the surface near s/r 7, in the
  bubble's recovery [1] pp. 227–228, Fig. 8(a). At Mach 0.8 the flow is the same kind: a supersonic plateau over a
  laminar bubble, closed by a shock (Inference: no schlieren at 0.8).
- **More Reynolds number or turbulence shortens the bubble** and raises the peak Mach number [1] p. 239. So in
  flight, onset may come earlier than 0.64, toward 0.51 (Inference).

## 3. The momentum thickness, read from the paper

Fig. 9 [1] p. 230 holds θ/r at R_r 12,000, zero incidence and no grid; read here by pixel (axis ticks calibrated,
±0.003). Goodhand's replot, read by eye in [3] §5, agrees within 0.003 at every point.

| Inlet Mach | 0.5 | 0.6 | 0.65 | 0.7 | 0.75 | 0.8 | 0.85 | 0.88 | 0.9 | 0.95 |
|------------|----:|----:|-----:|----:|-----:|----:|-----:|-----:|----:|-----:|
| θ/r, Fig. 9 | 0.097 | 0.120 | 0.139 | 0.156 | 0.194 | 0.214 | 0.240 | 0.201 | 0.166 | 0.164 |

- **The trend.** From Mach 0.5 to 0.85 θ grows about 2.4 times and δ* more than 3 times; the shape factor
  δ*/θ goes from about 1.35 to 1.76 [1] pp. 228, 230. From 0.85 to 0.90 compression waves ease the recovery and
  both fall; above 0.90 a strong shock on a turbulent layer separates it [1] p. 230.
- **Repeat runs disagree.** The same nominal condition appears in other figures. At Mach 0.6: 0.118 (Fig. 18),
  0.120 (Fig. 9), 0.139 (Fig. 22). At 0.85: 0.190 (Fig. 18, at −0.4°), 0.240 (Fig. 9), 0.264 (Figs. 22 and 25).
  At 0.75: 0.194 and 0.204 [1] pp. 230, 236, 239, 241 (pixel readings). The paper doesn't comment on it.
  Scaled by the 0.85 spread, Mach 0.8's value is 0.17–0.235 (Inference).
- **Incidence.** Near zero, θ/r rises about 0.02 per degree at Mach 0.6 and falls about 0.02 per degree at 0.85
  [1] p. 236, Fig. 18. Only the upper face was traversed. Doubling it at Mach 0.8's −0.9° misreads both faces' sum
  by up to ±0.04 in the drag coefficient below, sign unknown at 0.8 (Inference).
- **Reynolds number.** θ falls with R_r at every Mach number, steeply only at 0.90 [1] p. 235. From 8,000 to
  20,000, θ/r goes 0.153 → 0.121 at Mach 0.6, 0.228 → 0.197 at 0.75 and 0.283 → 0.222 at 0.85 [1] p. 239, Fig. 22
  (pixel).
- **Turbulence.** The grid halved the thicknesses at Mach 0.6 and 0.75 [1] p. 235 and, the paper says, cut them to
  under a third at 0.85 except at the lowest R_r [1] p. 238. That fits δ*; read from Fig. 25, θ/r falls only to
  0.36–0.42 of its no-grid value at R_r 10,000–20,000 (0.106 against 0.264 at 12,000) [1] p. 241.

## 4. From θ/r to an edge drag coefficient, redone

The momentum lost from the stream equals the drag on the surface ahead of the station [2] p. 2-4, eq. 19, stated
for constant static pressure; applying it to nose and bubble loss is Inference. Per unit
span, each face carries ρU²θ. On the edge's frontal area t = 2r, both faces give c_d = 2θ/r, taking the speed at the
station as free stream (Inference, as in [3] §5). Friction over the first 17.9 r is subtracted, since the
simulator charges skin friction apart.

- **Friction to subtract (Inference).** Over x = 17.9 r at R_r 12,000, Re_x = 214,800. Turbulent from the nose,
  C_f = 0.074/Re_x^0.2 [2] p. 2-5, gives θ_f/r = 0.037 × 17.9^0.8 × 12,000^−0.2 = 0.057. Laminar,
  C_f = 1.328/√Re_x [2] p. 2-4, gives 0.026. Compressibility, left out, cuts turbulent friction about 6% at Mach
  0.8 (θ_f 0.054), which would raise the low ends below by under 0.01.
- **Mach 0.8 in the test (Inference).** With Fig. 9's 0.214: 2 × (0.214 − 0.057) = 0.31 to 2 × (0.214 − 0.026) =
  0.38. With the repeat spread, 0.17–0.235: 2 × (0.17 − 0.057) = 0.23 to 2 × (0.235 − 0.026) = 0.42.
- **Mach 0.5 in the test (Inference).** 2 × (0.097 − 0.057) = 0.08 to 2 × (0.097 − 0.026) = 0.14, perhaps up to
  0.06 higher, since the run sat at −1.4° (Table 1) and Fig. 18's slope is 0.02 per degree. [3] §5 bracketed this
  nose term from 0 to the rule's 0.127; the paper puts it at 0.08–0.14 or more, present below sonic onset.

## 5. From the test to a hobby fin

Each factor scales the test's 0.23–0.42 (Inference unless cited).

| Cause | Sign | Factor or size | Basis |
|-------|------|----------------|-------|
| Station to trailing edge | lowers | ×0.76–0.86 for a 100 r chord | below |
| Reynolds number, R_r 1 × 10⁴ (Re_c 1 × 10⁶) | raises slightly | ×1.03–1.06 | Fig. 22 fit, below |
| Reynolds number, R_r 3 × 10⁴ | lowers | ×0.76–0.88 | extrapolated 1.5 times past the data |
| Reynolds number, R_r 1 × 10⁵ (Re_c 1 × 10⁷) | lowers | ×0.53–0.74 | extrapolated 5 times past the data |
| Incidence of the test points | either | ±0.04 | §3 |
| Free-stream turbulence | lowers | to about 0.10–0.16 at Mach 0.85 with the grid | §3, below |
| Shock loss outside the layer | raises | ≤ 0.013–0.032 × h/2r, height h unknown | normal shock, Fig. 7's peak |
| Free-jet speed at the station | lowers slightly | about −2% on ρU² at Mach 0.8 | Fig. 6: station Mach 0.79 |

- **Station to trailing edge.** A thickened turbulent layer behind the nose rubs less downstream, so part of the
  nose's excess θ is repaid by the trailing edge. Treating the excess as a shift of the layer's virtual origin
  under [2]'s power law, the excess left at a 100 r trailing edge is 0.76–0.86 of the excess at the station, for
  excesses of 0.05–0.19 θ/r and R_r 1 × 10⁴ to 1 × 10⁵. The wake keeps it, as a flat plate ends at free-stream speed.
  That law is past its range (Re_x below 10⁶ [2] p. 2-5) at a 100 r chord; Hoerner's log law gives 0.78–0.89.
- **Reynolds number.** Less the turbulent friction above at each R_r, Fig. 22's end points (8,000 and 20,000)
  fall as R_r^−0.15 at Mach 0.75 and R_r^−0.28 to R_r^−0.30 at 0.6 and 0.85. Applied from 12,000, that gives the
  table's factors. At Mach 0.90 a higher R_r switched the flow pattern [1] p. 235, so past 20,000 a step is possible.
- **Turbulence.** Free flight has little free-stream turbulence, so the no-grid data apply. A fin root in the
  body's boundary layer, or a rough nose, may sit nearer the grid case: θ/r 0.106 at Mach 0.85 gives
  2 × (0.106 − 0.057) = 0.10 to 2 × (0.106 − 0.026) = 0.16.

**The result (Inference),** an envelope over both friction laws and how the bounds pair. At Re_c 1 × 10⁶:
0.23 × 1.03 × 0.76 = 0.18 to 0.42 × 1.06 × 0.86 = 0.38. At Re_c 1 × 10⁷: 0.23 × 0.53 × 0.76 = 0.09 to
0.42 × 0.74 × 0.86 = 0.27. For [4]'s 3.2 mm fin at sea level (R_r ≈ 3.0 × 10⁴, chord about 97 r): 0.13–0.32.
Each is unswept, with ±0.04 for incidence on top; sweep is as [3] §6 has it.

- **Narrow, move or leave?** It widens 0.23–0.36 to 0.18–0.38 where the test's Reynolds number matches, and moves
  it to 0.09–0.27 at the top of the hobby range: the low end from Reynolds extrapolation, the high from scatter.
- **Why not narrower.** The paper's own repeat scatter at one condition (θ/r 0.19–0.26 at Mach 0.85) is wider than
  [3]'s whole band once doubled. Only one nose shape and one plate were tested, and no point above R_r 20,000.

## 6. Corrections to the earlier notes

- [3] §4 took the paper's "supersonic from Mach 0.7", which is right. Fig. 7 has the spike near sonic at 0.6,
  which interpolates to about 0.64, or earlier given the tap spacing (§2, Inference). It also took the low-speed
  peak as C_p −1.0; the paper's summary gives 1.3 U, C_p −0.69, at zero incidence and low turbulence; −1.0 matches
  its raised-turbulence 1.4 U. Onset 0.5–0.7 stands.
- [3] §5's readings hold (within 0.003). Its rise to Mach 0.8, 2 × (0.214 − 0.097) = 0.234, plus a Mach 0.5 term of
  0.08–0.14 instead of 0–0.127, gives §4's 0.31–0.38. It left out the trailing-edge factor and the repeat spread.
- [3]'s open questions: the Reynolds number (R_r about 12,000; 8,000–20,000), the station (s/r 17.9) and the plate
  (6 mm, 3 mm radius, 103 + 100 mm) are now known. [3]'s 3.2 mm fin, 6 × 10⁴ on thickness, is R_r ≈ 3.0 × 10⁴ on
  radius: 1.5 times the tested top, and 2.5 times the usual 12,000.
- [4] §4 found no test of a round-nosed plate at Mach 0.6–0.9 and took the term as 0 at Mach 0.8. This paper is
  such a test, on a 3% plate; reduced to a 2% fin, unswept, the term is 0.09–0.38 (Inference).

## What this suggests

- **Mach 0.8 round-nose term.** For [#614](https://github.com/nrdptel/fusionspace-eridanus/issues/614), an added
  sensitivity run with round leading edges at Mach 0.8 could take 0.09–0.38, with 0.24 as a middle, instead
  of 0 or 0.53. On [4] §6's invented rocket (fins swept 45°, fin frontal over reference area 0.127, the rule's
  cos² Γ_L at free-stream Mach), each 0.1 in the unswept term is 0.0064 in C_D, 1.3% of its 0.482 (Inference).
  At [apogee-bias](apogee-bias.md)'s −0.31% to −0.57% of apogee per 1% of C_D, that is 0.4–0.8% of apogee.
- **Below sonic onset, not zero.** The test's Mach 0.5 term of 0.08–0.14 is mostly bubble loss (Inference).
  A model that holds the round nose at 0 below its critical Mach number may read low there at R_r near 1 × 10⁴.
  For that critical Mach number, about 0.64 fits a bubble-laden nose at low turbulence, 0.51 an attached one.
- **The Arcas Mach 0.8 fin increment** gains nothing here: its fins are sharp double wedges ([3] §5).

## What the project can use

- **The paper [1]** (© IMechE 2000), **Hoerner [2]** and **Goodhand [5]** stay cached for reading and citing only,
  as [3] records. No figure or table is reproduced; pixel readings are restated with their figure and page.

## Open questions

- No data above R_r 20,000. A 2% fin at Re_c above about 2 × 10⁶ is past the test, and §5's low end is an
  extrapolation of end-point fits (a fit to all five points per Mach number moves the factors by 0.01 or less).
  A test or a calibrated MSES-style run at R_r 1 × 10⁵ would settle it.
- Tain's 1999 dissertation [1] ref. 8 holds the complete incidence and turbulence series and the tuned MSES
  parameters; it was not sought here. It may explain the repeat spread at Mach 0.85.
- Elliptic noses at high speed were not tested; [3] §4's critical Mach numbers for 2:1 to 4:1 noses stay
  untested, as does any square or rounded nose with a flat face.
- Whether a flight fin's nose flow behaves like the low- or the high-turbulence case is open. Walraevens and
  Cumpsty (1995) was read only through this paper's summary [1] p. 223.

## Sources

All read 2026-10-29. Page numbers are the journal's printed pages.

1. L. Tain and N. A. Cumpsty, *Compressor blade leading edges in subsonic compressible flow*, Proc. IMechE Part C
   214(1), 2000, pp. 221–242. <https://doi.org/10.1243/0954406001522921> (the publisher's page refused a plain
   fetch; read from the cached copy).
2. S. F. Hoerner, *Fluid-Dynamic Drag*, 1965, pp. 2-4, 2-5. <https://archive.org/details/FluidDynamicDragHoerner1965>
3. [thin-edge-mach-dependence](thin-edge-mach-dependence.md), this project's note, §§4–7 and Open questions.
4. [fin-edge-drag](fin-edge-drag.md), this project's note, §§4 and 6.
5. M. N. Goodhand, *Compressor Leading Edges*, PhD dissertation, University of Cambridge, 2010.
   <https://www.repository.cam.ac.uk/handle/1810/247860>
6. This note's pixel-reading and arithmetic scripts (Inference), not committed.
7. R. E. Walraevens and N. A. Cumpsty, *Leading edge separation bubbles on turbomachine blades*, ASME J.
   Turbomachinery 117, 1995, pp. 115–125; read only as [1] p. 223 summarizes it.
