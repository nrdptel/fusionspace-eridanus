# Drag of a partly extended airbrake blade: fences and ribs below twice the boundary layer

What this covers: the drag of a flat blade standing square to the flow on a wall, at heights h from about 0.2 to 2
times the boundary-layer thickness δ, from open fence and rib measurements in turbulent boundary layers. It is for
M6.4's deployment-to-drag curve for blade brakes and its controller's gain. It builds on
[airbrake-drag.md](airbrake-drag.md) (Vick's wall charts, which start near H/δ₉₅ ≈ 2, Hoerner's (h/δ)^(1/3) rule and
the rocket CFD) and [airbrake-installed-drag.md](airbrake-installed-drag.md) (hobby brakes at H/δ₉₉ ≈ 1 to 3). How far
to trust it: **every measurement here is on a flat tunnel wall at 8 to 25 m/s, with no slot, body curvature or fins.**
One open data set (Arie and others, 1975) carries the curve; a 1968 square-plate curve agrees within 2%, and
Plate's fit reads 3% to 21% above it. Chart values are read by pixel from scans, about ±0.02 in C_D. Good and
Joubert (1968) is paywalled and cited from its abstract and from open papers that quote it; Sakamoto and Arie's
finite-width plate (1983) was not reached. Read 2026-11-04.

**In short.** A blade half out carries **about 0.42 to 0.47 of its full-out drag force** (0.38 at the lowest, for a
narrow brake 2δ tall), not "almost nothing". Its drag coefficient on free-stream dynamic pressure falls only slowly
as it sinks into the boundary layer: on a wall, a thin rib reads about 0.60 to 0.68 at h/δ₉₉ = 0.25, 0.74 to 0.79 at
0.5, 0.87 to 0.90 at 1.0 and 0.95 at 1.35, linear in log(h/δ), and Vick's 90° brakes continue it at 1.0 to 1.1 above
about 2δ₉₉. So drag force goes as about h^1.2 (1.1 to 1.4) at fixed width, against h^(4/3) for Hoerner's rule and a
step for the Mississippi State CFD. The fraction is nearly the same for a full-out height of 1 to 3 δ, so the
curve's shape doesn't need δ known well; the absolute level does. Width over height from 1 to 10 changes C_D by under
about 10%, though the same 1968 curves put a very wide fence about 25% above a square plate at h = δ. The pieces
still unmeasured are a blade fed through a slot in a curved body, and any Mach effect below about 2δ.

## Conventions

- **C_D** is the drag over free-stream q times the blade's exposed frontal area h × w, unless a source's other
  reference is named. **Fraction** is a half-out blade's force over the full-out force at the same width and speed:
  0.5 × C_D(H/2) / C_D(H).
- **δ.** δ₉₉ is where the speed reaches 0.99 of the stream; Vick's δ₉₅ is 0.95 of it. For a 1/7-power profile
  δ₉₉ ≈ 1.34 δ₉₅ (*inference*; airbrake-drag.md rounds it to 1.4). Each source's own δ is named where it is used.
- **Aspect ratio A** = width across the flow over height, as Vick defines it. Arie's b/h is a different quantity: a
  rib's streamwise thickness over its height. A blade of fixed width that is half out has twice the A it has full out.

## 1. Two-dimensional ribs: Arie, Kiya, Tamura and Kanayama, Bulletin of the JSME (1975)

- **Setup.** A 1.2 m by 1.5 m closed-circuit tunnel; ribs spanning the 1.5 m width, 10 to 70 mm tall, at most 1.4 δ,
  b/h 0.125 to 6.0, pressure-tapped (p. 1262). Zero pressure gradient within ±0.5% of q; free stream 8.0, 15.5 and
  25.4 m/s, u_τ/U 0.0392, 0.0380, 0.0366, momentum-thickness Reynolds number 3,970 to 9,380 (pp. 1263 to 1264,
  table 2). δ is δ₉₉, measured: 51 to 62 mm at the ribs (p. 1261; fig. 5 legend, table 2). C_D is pressure drag
  over free-stream q times h.
- **The curve** (fig. 8, p. 1265, read by pixel along the fitted lines, b/h = 1):

| h/δ₉₉ | 0.20 | 0.25 | 0.50 | 0.75 | 1.00 | 1.35 |
|---|---|---|---|---|---|---|
| C_D, 8.0 m/s | 0.56 | 0.60 | 0.74 | 0.81 | 0.87 | (0.90 at 1.13) |
| C_D, 25.4 m/s | 0.63 | 0.68 | 0.79 | 0.85 | 0.90 | 0.95 |

- **A log law.** The authors find C_D linear in log₁₀(h/δ) up to h/δ = 1.37 (p. 1265); the slope read here is 0.38
  to 0.45 per decade (*inference*). In wall units it is D/(½ρu_τ²h) = A₁ log₁₀(hu_τ/ν) + B₁, with A₁ and B₁
  functions of b/h only (eq. 6, p. 1266). Fig. 10 gives A₁ ≈ 278 and B₁ ≈ −308 at b/h = 1, read by eye; with these
  the law returns the 8.0 and 25.4 m/s lines within about 2% (*inference*).
- **A thin plate reads like the b/h = 1 rib.** At h/δ = 0.55, C_D is about 0.76 at b/h = 0.125 and 0.77 at 1.0, and
  falls to 0.51 by b/h = 6 as the flow reattaches on top (fig. 13, p. 1267; reattachment at b/h = 4, p. 1266). So
  the b/h = 1 line stands for a blade within about 0.02 (*inference*).
- **Reynolds number.** At fixed h/δ, C_D rose about 3% (h/δ = 1), 6% (0.5) and 13% (0.2) from 8.0 to 25.4 m/s
  (fig. 8). With the 1/7-power δ and Coles's u_τ at 100 to 250 m/s, 1 to 2 m from a nose (δ₉₉ ≈ 13 to 28 mm,
  δu_τ/ν ≈ 3,800 to 12,500, up to 4 times the tunnel's), the law gives 0.61 to 0.64 at 0.2, 0.75 at 0.5 and 0.84
  to 0.85 at 1.0 (*inference*, an extrapolation): in the tunnel's spread at 0.2 and 0.5, 3% to 5% under it at 1.0.
- **Against Good and Joubert.** Arie and others say Good and Joubert's log law held only to h/δ ≈ 0.6, and put the
  difference down to blockage in that test, an expected 10% to 15% error in C_D (p. 1265). Good and Joubert's
  wall-law line has the same slope as the b/h = 1 rib and a slightly higher level (p. 1266, fig. 9).
- **Trust.** One tunnel, one wall, low speed, centerline taps; no blockage correction described (ribs ≤ 6% of height).

## 2. Two-dimensional fences: Plate (1963) and Good and Joubert (1968)

- **Plate, Colorado State report CER63EJP66** (for the 1964 ASME separated-flow symposium). Sharp-edged steel fences
  1, 1.5 and 2 in tall on the floor of a 6 ft by 6 ft tunnel, 11 to 72 ft/s (p. 6). Its fit, for all its data and
  valid below h/δ = 0.1 too, is C_D = 1.05 (h/δ)^(2/7) on free-stream q (eq. 16, p. 13). δ is not measured: it comes
  from the 1/7-power smooth-plate law (p. 13). That gives 0.66 at h/δ = 0.2, 0.86 at 0.5 and 1.05 at 1.0, 3% to
  21% above Arie's lines; the report leaves h/δ > 1 unestablished (p. 17). The scanned copy lacks its figures.
- **Good and Joubert, J. Fluid Mech. 31 (1968), paywalled, read from the abstract.** Upstream-face pressures follow a
  wall-similarity law even for plates taller than δ; drag correlates with height in forms like the law of the wall
  and the velocity-defect law; the boundary layer's history acts only through the rear bubble, and only above
  h/δ = 0.5. A later summary by Joubert, Perry and Stevens (1971) writes the scaling as
  D/(½ρu_τ²h) = f(hu_τ/ν, h/δ), with a wall law in hu_τ/ν for small h and a drag-defect law in h/δ (p. 181).
  Morton (arXiv physics/0605209) quotes C_D = 1.45 at h/δ = 2.34 from their fig. 6, at 11% blockage (table 2 and
  note f, p. 7); whether blockage was corrected is not stated there.
- **Ranga Raju, Loeser and Plate, J. Fluid Mech. 76 (1976), paywalled, abstract only.** After blockage corrections,
  earlier fence data collapsed onto one curve of C_D on u_τ against height over the approach flow's roughness
  parameter. Its numbers were not reached.

## 3. Finite width: plates of breadth b and height h on a smooth wall

- **Joubert and others (1968), as redrawn by Joubert, Perry and Stevens (1971), fig. 9, p. 188.** C_D on free-stream
  q against A = b/h from 0.01 to 100, one curve per h/δ. The summary says A from 1 to ∞ has a small effect while h/δ
  has a large one (p. 181). Read by pixel: at A ≈ 1.2 the curves sit at 1.08 (h/δ → ∞), 0.89 (marked h/δ = 1),
  0.76 and 0.62; at A ≈ 11 at 1.17, 0.95, 0.72 and 0.60; a fifth sits near 0.50. So from A ≈ 1 to 11, +8% at the
  top, +6% at h/δ = 1 and −3% to −5% lower down (*inference*). The lower curves' h/δ labels can't be read in the
  scan, and δ's definition isn't given.
- **The square plate at h/δ = 1 (0.89) matches Arie's two-dimensional rib (0.87 to 0.90)** in a different tunnel
  (*inference*). But the same h/δ = 1 curve rises to about 1.12 at the figure's two-dimensional end: within one
  source a square plate reads about 20% under a fence. The two tunnels disagree on the span cost (*inference*).
- **Vick's 90° brakes** (fig. 12a, p. 30, Mach 0.2) at the shortest chord, R/δ₉₅ = 2.67: about 1.03 at A = 1 and 1.13
  at A = 0.5; A = 2 reads near 1.0. The faired A = 1 curve runs on to 1.00 at H/δ₉₅ = 2.0 (*inference*: the charts are
  faired; the shortest chord's line meets the 90° curve at 2.67).
- **Sakamoto and Arie, J. Fluids Eng. 105 (1983), on a normal plate of finite width:** not reached (the publisher's
  page refused the fetch). Its numbers would size the span effect directly.

## 4. Joining the pieces, and the fraction at half deployment

- **They join.** Arie's 0.95 at h/δ₉₉ = 1.35 continued by his log law gives about 1.02 at 2.0 δ₉₉; Vick's A = 1
  brake reads 1.03 at H/δ₉₅ = 2.67, about 2.0 δ₉₉ (*inference*). So one curve runs from 0.2 δ₉₉ to Vick's peak:
  about 0.64, 0.77, 0.88, 1.03, 1.07 and 1.10 at h/δ₉₉ = 0.25, 0.5, 1, 2, 3 and 4.5 (A ≈ 1, Mach about 0.2).
- **The fraction** (*inference*, 0.5 × C_D(H/2) / C_D(H), from the ranges above):

| Full-out height H | Half-out C_D | Full-out C_D | Fraction |
|---|---|---|---|
| 1.0 δ₉₉ | 0.74 to 0.79 (Arie) | 0.87 to 0.90 (Arie) | 0.42 to 0.44 |
| 1.37 δ₉₉ | 0.84 (Arie, 25.4 m/s) | 0.95 (Arie) | 0.44 |
| 2 δ₉₉ (2.7 δ₉₅) | 0.87 to 0.90 (Arie) | 1.03 (Vick A = 1); 1.14 (A = 0.5) | 0.42 to 0.44; 0.38 to 0.39 |
| 3 δ₉₉ (4.0 δ₉₅) | 0.97 to 1.00; 1.00 at A = 1 | 1.07 (A = 1); 1.24 (A = 0.5) | 0.45 to 0.47; 0.40 |

- **As a power law** F/F_full = (h/H)^p, the fraction 0.38 to 0.47 gives p = 1.09 to 1.40, about 1.2 (*inference*).
  Locally, C_D grows as h^n with n = slope / (2.30 C_D): about 0.30 at h/δ = 0.2 and 0.21 at 1.0. One fixed exponent
  fits only over a narrow range.

## 5. The rules and the CFD against the curve

| Source | C_D at h/δ = 0.2, 0.5, 1.0 | Fraction at half (H ≤ δ) | Against the fence data |
|---|---|---|---|
| Arie, 2D rib, δ₉₉ | 0.56 to 0.63, 0.74 to 0.79, 0.87 to 0.90 | 0.42 to 0.44 | the reference |
| Hoerner, 0.9 (h/δ)^(1/3) | 0.53, 0.71, 0.90 | 0.40 | with 1.2: 4% to 17% low at 0.2 and 0.5, 0% to 3% high at 1 |
| Plate, 1.05 (h/δ)^(2/7), δ from formula | 0.66, 0.86, 1.05 | 0.41 | 3% to 21% high |
| WVU CFD, extruded blade | 0.75 half, 1.0 full (on impact pressure) | 0.375 (rocket 0.35, 0.36) | 0.05 to 0.07 under |
| Mississippi State CFD, square blades | 0.47 full, on blade area | ≤ 0.016 | 25 times under |

- Hoerner's 0.9 is 1.2 × 0.75, the 1.2 being airbrake-drag.md's reading of fig. 13; his p. 5-8 text gives square plates
  1.0 to 1.3, which moves the row from 30% under to 13% over Arie. The 0.40 fraction doesn't depend on it. His δ is the
  total thickness, about 1.6% of the run (p. 5-6, eqs. 9 to 11); his two-dimensional strips were tested at h/δ = 0.075
  (p. 5-7, fig. 9). The Hoerner row's 4% to 17% follows from those numbers (*inference*).
- The WVU and Mississippi State values are airbrake-drag.md's readings of their tables (Farha table 5.2; Sanipara
  and others table 1), not reread here. Mississippi State's 75% setting gave 0.53 to 0.56 of full, against 0.70 to
  0.71 from the fence curve (*inference*). Its mesh is unreported; nothing found here explains a factor of 25.
- **Hoerner's 1/3 power is a fair rule for H ≤ δ** (fraction 0.40 against 0.42 to 0.44); it says nothing for H > δ.
  The "half out adds almost nothing" reading has no support in any wall measurement found (both *inference*).

## 6. What the fence data leave out (sign and size)

- **Slot and body.** Every case is a plate on a flat, sealed wall. A blade through a slot in a body tube may leak,
  and transverse curvature thins the boundary layer (Hoerner p. 18-11, per airbrake-drag.md). Sign of the slot
  effect unknown; unbounded.
- **Mach.** All fence data are below 25 m/s. Vick's walls rise 16% to 44% from Mach 0.2 to 0.9 above 2δ₉₅; below,
  unmeasured. An equal rise at every height leaves the fraction unchanged (*inference*).
- **Pressure gradient and history.** On a rocket δ grows from a nose with its own gradient (*inference*); Good
  and Joubert find history matters only above h/δ = 0.5 and only through the bubble behind. Size not found.
- **δ on a rocket** is itself an estimate (flat-plate law from the nose tip). The fraction moves by only about ±0.03
  across H = 1 to 3 δ, so the curve's shape is robust to it; the full-out level (0.87 to 1.10) is not (*inference*).
- **Fins and the body behind.** As in airbrake-installed-drag.md: unmeasured for blades; not changed here.

## What the project can use

- **Data.** Arie and others (1975) is free to read on J-STAGE (© JSME): cite the figures, and keep digitized points
  in refs/ for tests; Plate's report is marked in copyright by Colorado State (cite only); Vick is a US government
  work (may be digitized and committed); the 1971 conference paper and Morton's arXiv paper carry no open license
  (cite only). Hoerner is copyrighted (cite only).
- **M6.4's deployment-to-drag curve** (a suggestion). For a radial blade of fixed width, model force as
  F/F_full = (h/H)^p with p = 1.2, and carry p = 1.1 to 1.4 as the range. Its expected effect against a
  linear-in-area model (p = 1): 13% less drag at half out (0.44 against 0.50) and about 20% more force per unit of
  travel near full out. Against the Mississippi State table, about 25 times more drag at half out.
- **The controller's gain.** dF/dh = p (F_full/H) (h/H)^(p−1): about 1.2 F_full/H near full out, 1.04 at half and
  0.87 at a fifth out (p = 1.2, *inference*). A gain tuned on a linear model is about 17% low near full out.
- **Which way it errs.** A model reading partial-blade drag high (p = 1, or C_D held at the full-out value)
  predicts a braked apogee too low: the flattering side for a ceiling. A model with half-out drag near zero reads
  apogee too high and makes a controller overdrive the blades. A test could pin that the half-out fraction stays in
  0.38 to 0.47.
- **If C_D itself is wanted** at a known δ: the table in section 4, or Arie's wall law with A₁ ≈ 278, B₁ ≈ −308 for
  h ≤ 1.37 δ₉₉, joined to Vick's 90° curves above about 2 δ₉₅.

## Open questions

- No measurement of a blade fed through a slot in a body of revolution, partly out, was found; a whole-rocket tunnel
  test at three or four extensions would settle the fraction and the slot's sign.
- Sakamoto and Arie's (1983) finite-width constants, and the Kitami Institute bulletins of Sakamoto and Moriya on
  normal plates (repository returned errors on 2026-11-04), would size the span effect at h/δ < 1.
- Good and Joubert's own constants, Ranga Raju's collapsed curve, Wieghardt's 1943 and 1953 balance data, ESDU's
  excrescence items and the NACA spoiler reports (Wenzinger and Rogallo; Fischel) were not read.
- Mach dependence below 2δ, and why the Mississippi State CFD reads 25 times under every wall measurement.

## Sources (all read 2026-11-04)

1. M. Arie, M. Kiya, H. Tamura, Y. Kanayama, "Flow over rectangular cylinders immersed in a turbulent boundary layer,
   part 1", Bulletin of the JSME 18 (125), pp. 1260 to 1268 (1975), table 2, figs. 5, 8, 9, 10, 13; free access,
   © JSME, cached: https://www.jstage.jst.go.jp/article/jsme1958/18/125/18_125_1260/_article/-char/en
2. E. J. Plate, "The drag on a smooth flat plate with a fence", Colorado State University CER63EJP66 (December 1963),
   pp. 6 to 17, eqs. 3, 16, 22; "In Copyright", cached: https://hdl.handle.net/10217/198002
3. M. C. Good, P. N. Joubert, J. Fluid Mech. 31 (3), pp. 547 to 582 (1968), abstract only:
   https://doi.org/10.1017/S0022112068000327
4. P. N. Joubert, A. E. Perry, L. K. Stevens, "Drag of a bluff body immersed in a rough-wall boundary layer", Wind
   Effects on Buildings and Structures (1971), pp. 179 to 188, fig. 9; no license stated, cached:
   https://www.aivc.org/sites/default/files/members_area/medias/pdf/Airbase/airbase_00472.pdf
5. T. S. Morton, "A correlation between drag and an integral property of the wake", arXiv physics/0605209, table 2,
   p. 7; no open license listed on arXiv, cached: https://arxiv.org/abs/physics/0605209
6. K. G. Ranga Raju, J. Loeser, E. J. Plate, J. Fluid Mech. 76 (2), pp. 383 to 399 (1976), abstract only:
   https://doi.org/10.1017/S0022112076000682
7. A. R. Vick, NACA RM L57J23a (1958), fig. 12a, p. 30; US government work:
   https://ntrs.nasa.gov/citations/19930090297
8. S. F. Hoerner, *Fluid-Dynamic Drag* (1965), pp. 5-6 to 5-8, eqs. 9 to 11, figs. 9, 13; p. 18-11; cited only.
9. H. Sakamoto, M. Arie, J. Fluids Eng. 105 (1), p. 98 (1983): https://doi.org/10.1115/1.3240950 (not reached).
10. Farha (WVU, 2025), table 5.2, and Sanipara, Belew and Sprague (AIAA 2024-85628), table 1, as read in
    [airbrake-drag.md](airbrake-drag.md); cited only.
