# Airbrake drag: flaps and blades against deployment and Mach

What this covers: the drag a deployed airbrake adds to a hobby rocket, against how far it is out and against Mach
up to about 0.9. Sources are tunnel tests, rocket computational fluid dynamics (CFD) and flight fits. The note also
compares the simple rules in use with those data and gives what is known of the brakes' effect on the fins behind.
It is for M6.4's drag model and its citation. How far to trust it: **no hobby-rocket wind-tunnel test was found.**
The one systematic data set is a 1958 NACA test of small brakes on a flat tunnel wall. Every rocket number is
CFD, often on unreported meshes, or one flight's fit. Chart values are read by eye, about ±0.03 in C_D. Builds on
[roll-and-drag-control.md](roll-and-drag-control.md) (flat plate's 1.17, D-558, RocketPy). Researched 2026-10-18.

## Conventions

- **On brake area**: the added drag over q times the brakes' total projected frontal area. **On body area**: over
  q times the body's cross-section, the usual rocket reference.
- **Flap**: a plate hinged at the body, opened through α from the skin (90° is square to the flow); projected
  height H = R sin α, R its hinge-to-tip length. **Blade**: a plate pushed out radially at 90°.
- δ is the boundary-layer thickness. Vick defines it where the speed reaches 0.95 of the stream (δ₉₅); the
  textbook δ₉₉ is about 1.4 times larger for a 1/7-power profile (*inference*: 0.95⁷ ≈ 0.70).

## 1. Brakes on a wall: NACA RM L57J23a (Vick 1958)

Flat plates 1/16 in thick, hinged on a tunnel wall in its turbulent boundary layer, α from 15° to 90°, aspect ratio
A = width/chord from 0.25 to 4, Mach 0.2 to 1.3, with sting-mounted plates in the free stream for comparison. C_D is
on projected area. δ was about 0.12 in up to Mach 0.5 (extrapolated below about Mach 0.45) and 0.10 in at Mach 0.8;
Reynolds number 0.1 to 0.6 million per inch (pp. 2 to 4, fig. 3). The design charts (fig. 12) give C_D against H/δ.
Peak values:

| α | A = 0.5, M 0.2 | A = 0.5, M 0.5 | A = 0.5, M 0.9 | A = 1, M 0.2 | A = 1, M 0.9 |
|---|---|---|---|---|---|
| 90° | 1.27 (H/δ ≈ 5) | 1.31 | 1.52 | 1.10 | 1.38 |
| 60° | 1.12 | 1.18 | 1.40 | 0.97 | 1.27 |
| 45° | 0.95 (H/δ ≈ 4) | 1.00 | 1.22 | 0.83 | 1.11 |
| 30° | 0.75 | 0.78 | 0.91 | 0.64 | 0.91 |
| 15° | 0.44 (H/δ ≈ 2) | 0.47 | 0.51 | 0.43 | 0.62 |

- **The boundary layer raises the drag of a brake that clears it** (aspect ratio up to about 1). In the summary's
  words, the drag "in the presence of a small amount of boundary layer was generally greater than that obtained
  with a comparable flat plate in a uniform stream" (p. 1). A 90° wall brake's base drag is "on the order of 40
  percent greater" than the sting plate's below Mach 1 (p. 6, fig. 10). Each angle's curve rises with H/δ to a peak
  at H/δ ≈ 2 (15°) to 5 (90°), then falls slowly toward the free plate (pp. 7 to 8); below H/δ ≈ 2 it falls
  steeply. The 90° curves start near H/δ ≈ 2, so a 90° brake shorter than about twice δ is not covered. At A = 2,
  Mach 0.2 to 0.5, the 90° wall brake reads at or below the sting plate (fig. 12).
- **Deflection.** Each chart draws one flap opened through α (constant R/δ) as a straight line through the origin
  (fig. 12). So C_D on projected area goes as sin α, and drag on the flap's own area as sin² α (*inference* from
  the charts' construction). At A = 0.5, Mach 0.2 the 45° peak is 0.75 of the 90° peak, against sin 45° = 0.71.
  It holds for flaps up to about R/δ = 5; the longest-chord line (R/δ = 8.34) curves, and along it 15° and 30°
  read about 1.4 and 1.2 times the sin² α value.
- **Aspect ratio.** C_D falls from A = 0.5 to 2 and rises again at 4 (p. 8): at 90°, Mach 0.2, A = 1 reads 13%
  under A = 0.5. Side plates mostly add drag (p. 9). The sting plate read 1.04 at Mach 0.25 and 1.23 at 0.9
  (fig. 10), below the textbook 1.17; Vick puts the older values at Mach under 0.1 and low Reynolds number (p. 6).
- **Caveats.** Vick's overlay of fuselage-brake tests shows "no areas of major disagreement", each with an assumed
  δ (p. 8). Tunnel-wall reflections near Mach 1 were faired out (p. 5); the Mach 0.9 charts may keep some
  (unbounded). The text read does not say which edge was hinged upstream.

**Where hobby brakes sit (*inference*).** The textbook turbulent flat-plate δ₉₉ ≈ 0.37 x Re_x^(−0.2) (the
1/7-power law) gives about 20 mm at x = 1.5 m and 150 m/s, so δ₉₅ ≈ 14 mm. Published brakes stick out 2 to 4 cm
(ARIS 2.1, WVU 3.9, Mississippi State about 3.2), so H/δ₉₅ ≈ 1.5 to 3: at or below the start of Vick's 90°
curves, where C_D depends most on δ. ARIS's and WVU's blades (A ≈ 2 to 2.6, H/δ₉₅ ≈ 1.5 to 1.7) sit below it.
Vick's Reynolds number per brake height (up to about 4 × 10⁵) is close to a 4 cm brake's at 150 m/s (4 × 10⁵).

## 2. Aircraft fuselage brakes

- **D-558-1, NACA RM L8B06 (1948).** A 1/16-scale model with two side brakes at 60°, tested to Mach 0.96. The
  increment is about 0.030 to 0.025 on wing area, nearly flat with Mach (p. 4, fig. 8). Each brake is 1.47 in² on a
  0.587 ft² wing (table I, fig. 2). *Inference*: 0.72 to 0.86 on flap area, 0.83 to 1.0 on projected area. At
  0° angle of attack fig. 8 reads 0.034, 0.98 on flap area. No serious stability change with brakes out, at one
  tail setting (pp. 4 to 5).
- **Hoerner, *Fluid-Dynamic Drag* (1965).** Reduces the same brake to 0.97 on flap area at Mach 0.5 (p. 13-28,
  fig. 63a), and gives fuselage brakes "between 0.9 and 1.0" across Mach, a wing flap near 1.0 to Mach 0.8
  (pp. 15-32 and 15-33).

## 3. Rocket CFD with tabulated numbers

- **West Virginia University MS thesis (Farha 2025), Ansys Fluent RANS, generic 157 mm rocket.** Four brakes per
  set, 45° off the fins; three designs 6 calibers ahead of the fins, one between them (pp. 11 to 14). Whole-rocket
  C_D (tables 3.2, 5.1) gives, at full deployment and Mach 0.2 to 0.8, 0.86 to 0.96 on brake area for radial
  blades, 0.83 to 0.98 for rotating plates, 0.92 to 1.18 for folding flaps (*inference*, four brakes per set). Half
  deployment gives 0.61 to 1.07. The brake-only force over impact pressure is about 1.0 at full and 0.75 at half
  (table 5.2); for forward sets the whole-rocket increment is 0.85 to 0.92 of the brakes' own drag (*inference*;
  the half-out folding set reads 1.05): the body behind loses about a tenth of it in the wake.
- **Mississippi State (AIAA 2024-85628), SolidWorks Flow Simulation, 4.12 in rocket, four square blades.** Full
  deployment adds 0.21 to 0.22 on body area, about 0.47 on blade area, flat from Mach 0.07 to 0.67 (table 1,
  *inference* from forces). 75% gives 0.53 to 0.56 of full; 50% and less add nearly nothing. The mesh is not
  reported. Their hand estimate with a flat-plate C_D of 1.28 gave 17 lbf at about 575 ft/s, 2.4 times the CFD's
  7.1 lbf increment (p. 4, table 1).
- **Georgia (AIAA 2025-99705), Ansys k-ω SST, three petals.** Whole-rocket C_D 0.275 to 0.395 by CFD against 0.513
  to 0.770 by OpenRocket: an increment of 0.12 against 0.26 (table 1, p. 7). No brake area is stated.

## 4. Flight fits

- **ETH Zürich ARIS (EUCASS 2019-388), 0.1 m rocket.** Three plates, 11.5 cm² each at 2.1 cm out (p. 10), on
  81.7 cm² of body (p. 12). The flight's drag coefficient (fig. 6, read by eye) is about 0.40 to 0.47 before the
  brakes move and 0.73 to 0.79 with them out: 0.26 to 0.39 on body area, 0.62 to 0.92 on brake area
  (*inference*). Their model, below, read 8% to about 18% high. No other flight with brake area and drag together
  was found.

## 5. The simple rules against the data

- **Free flat plate.** C_D = 1.17 on a square or round plate's area; a rectangle stays near 1.18 down to a
  height-to-span ratio of 0.2 (Hoerner p. 3-16, fig. 28). Inclined, the normal force stays about 1.17 from 45° to
  90° (fig. 29a), so drag on plate area goes as 1.17 sin α. Against Vick it reads about 8% low for a 90° brake at
  its A = 0.5 peak (Mach 0.2 to 0.5) and 23% high for a 45° flap: sin α against the walls' sin² α (41% apart).
- **Hoerner's protuberance rule.** Below the edge of the boundary layer (h < δ) a small body sees q_eff/q ≈
  0.75 (h/δ)^(1/3) (p. 5-6, eq. 9), with an independent C_D of about 1.2 for a small plate (p. 5-8, fig. 13).
  Hoerner states it for h < δ only.
- **The ARIS discount.** C_D = 1.17 (1 − 0.25 δ/h) (EUCASS 2019-388, p. 14, eq. 30) is that rule's mean over a
  plate taller than δ with free stream above δ (*inference*: the two join at h = δ). The paper cites Hoerner
  p. 3-17, which in the cached copy does not hold it. It lowers C_D at every H/δ, while Vick's walls raise it above
  H/δ ≈ 2 at aspect ratios up to 1: the sign is wrong there. It discounts only the incoming flow and keeps the
  free plate's base pressure.
- **Mach, low speed to 0.9.** The stagnation-pressure factor 1 + M²/4 + M⁴/40 (Hoerner p. 15-2, eq. 4) gives +21%
  from Mach 0.2; Hoerner's disk, 0.45 + 0.70(1 + 0.25 M²) (p. 15-4), +12%; Vick's sting plate +18%; Vick's wall
  brakes +16% to +44%, most at small angles and A = 1; the installed D-558 brake about −10%; WVU's CFD +11% to
  +28% (Mach 0.2 to 0.8; +32% between the fins); Mississippi State flat. 1 + M²/4 sits inside that spread.

**Worked example (invented round numbers).** A 98 mm body (75.4 cm²) with three flaps 30 mm wide and 60 mm long
(A = 0.5, 18 cm² each), 1.2 m from the nose at sea level: δ₉₉ ≈ 18 mm at Mach 0.3 and 15 mm at 0.8,
δ₉₅ ≈ 12.8 and 10.5 mm. The added C_D on body area:

| α, Mach | Free plate × disk factor | ARIS discount (δ₉₉) | Vick, A = 0.5 |
|---|---|---|---|
| 45°, 0.3 | 0.60 | 0.53 | 0.48 (C_D 0.94, H/δ 3.3) |
| 45°, 0.8 | 0.65 | 0.54 | 0.55 (C_D 1.09, H/δ 4.0) |
| 90°, 0.3 | 0.85 | 0.77 | 0.92 (C_D 1.28, H/δ 4.7) |
| 90°, 0.8 | 0.92 | 0.79 | 1.03 (C_D 1.44, H/δ 5.7) |

The rules spread by up to 30%, in different orders at 45° and 90°. The Vick column interpolates between charts in
M² (*inference*). Whether a wall-tunnel value is the installed one is open: ARIS's flight (0.62 to 0.92 on brake
area) and WVU's blades (0.86 to 0.96) sit below where Vick's 90° curves start, so they can't be checked against
him directly, and nothing here separates the body's curvature, the wake on the body behind and the fit's error.

## 6. Interference with the fins behind

- **Brakes between the fins (WVU CFD, 10 cm aft of the fin root, 45° off the fins).** Static margin fell from 3.37 to
  0.25 calibers at Mach 0.2 at full deployment, 1.16 at 0.4, 2.34 at 0.6 and 3.12 at 0.8; at half deployment 2.46
  at Mach 0.2 (table 3.3, p. 26). A low-pressure wake covers the fins and moves the center of pressure forward
  (p. 39). The same brake area there gave 15% to 35% more drag than the forward set (*inference*, section 3).
- **Brakes 6 calibers ahead (same study).** Margin changed by +0.2 to −1.14 calibers by design and Mach.
- **Buffet** of fins behind solid brakes: no measurement found; unbounded (perforated flaps avoid it on aircraft).

## 7. RocketPy's air-brakes data (MIT, read at commit 9bd6ad3)

- `data/rockets/calisto/air_brakes_cd.csv`: 133 rows of (deployment level, Mach, C_D), both axes 0 to 1.1 by 0.1.
  With `reference_area=None` the C_D is on the body's 0.01267 m² and added to the rocket's drag. Full deployment
  gives 0.932 at Mach 0.1, 0.798 at 0.5, 0.916 at 0.8 and 0.810 at 0.9: not monotonic in Mach. Half deployment is
  0.217 to 0.360. No source and no brake geometry are stated; the file came in with pull request 426.
- The docs page `docs/user/airbrakes.rst` prints an excerpt "from the CSV" that does not match it: 0.051 against
  0.217 at level 0.5, Mach 0.1.
- `data/rockets/camoes/drag_airbrakes.csv`: 121 rows of whole-rocket C_D from a EuRoC 2023 team. Full deployment
  adds 0.20 to 0.33 over retracted; level 0.9 adds 0.125 to 0.174, so the last tenth doubles it at Mach 0.2 and
  below and multiplies it by 1.2 to 1.5 above (*inference*). The flight data's license is unclear: cite, don't copy.

## What this suggests

- **A table, not a formula, for M6.4's default.** Digitize Vick's fig. 12 into a (α, A, H/δ, Mach) table, a US
  government work, with a test pinning sin² α along a flap's opening and the rise with Mach. Blades follow the 90°
  curve as H/δ grows; flaps follow a line of constant R/δ.
- **Carry a range.** Installed full-deployment values on brake area run 0.62 to 1.18 (ARIS flight, WVU CFD,
  D-558), Mississippi State's CFD 0.47, Vick's walls 1.0 to 1.5 to Mach 0.9. A model should say which it follows
  and carry the spread; a blade below H/δ ≈ 2 has no measured value at all.
- **Which way it errs.** A brake model that reads high predicts a lower braked apogee, the flattering side for a
  waiver or range limit. The brakes-retracted apogee doesn't depend on the brake model at all.
- **Don't copy the 1 − 0.25 δ/h discount.** Its sign disagrees with Vick above H/δ ≈ 2 at aspect ratios up to 1.
- **RocketPy comparison (M6.4's done-when).** Its Calisto table has no stated source; the 3% match measures
  agreement with RocketPy's integration of the same table, not drag accuracy. Its non-monotonic Mach rows and the
  docs excerpt's mismatch are worth reporting upstream.
- **Fins.** Brakes between the fins cost most of the static margin at low speed in one CFD study; sets 6 calibers
  ahead lost up to about a third. A warning for brakes near the fins could stand until a measurement exists.
- **#417.** EuRoC's fail-extended rule makes full deployment a state the stability check sees for the whole
  descent from the fault on, which brings the fin interference above into that check.

## Open questions

- No measurement of airbrakes on a body of revolution below Mach 1 was found; Hoerner (p. 18-11) says transverse
  curvature thins the boundary layer, by an amount not sized here.
- 90° brakes shorter than about 2δ (blades partly out) are below Vick's data; Mississippi State's CFD says they add
  almost nothing; Hoerner's (h/δ)^(1/3) gives a half-height blade about 0.8 of a full-height (h = δ) blade's C_D,
  about 0.4 of its force. Which holds is open.
- Not read: Good and Joubert (1968, paywalled) on fence drag; ESDU fence items; DATCOM's spoiler section; NACA
  TN 1939, ARC C.P. 251 and R&M 2614; IREC and EuRoC team reports.
- Only one systematic Mach data set (Vick); his Mach 0.9 charts may carry tunnel-wall interference. Fin buffet
  behind solid brakes: no data.

## Sources (all read 2026-10-18)

1. A. R. Vick, NACA RM L57J23a (January 1958), pp. 1 to 9, figs. 3, 10, 12. US government work:
   https://ntrs.nasa.gov/citations/19930090297
2. J. B. Wright, NACA RM L8B06 (1948), pp. 2 to 5, table I, figs. 2, 8. US government work:
   https://ntrs.nasa.gov/citations/19930085315
3. S. F. Hoerner, *Fluid-Dynamic Drag* (1965), pp. 3-15 to 3-17, 5-6 to 5-8, 13-28, 15-2 to 15-4, 15-32 and 15-33,
   18-11. Copyrighted: cited only.
4. M. L. Farha, MS thesis, West Virginia University (2025), tables 3.2, 3.3, 5.1, 5.2, pp. 10 to 14, 26, 39;
   cited only: https://researchrepository.wvu.edu/etd/12808/
5. Sanipara, Belew and Sprague, AIAA 2024-85628, table 1; cited only: https://arc.aiaa.org/doi/10.2514/6.2024-85628
6. Blenk and Rupram, AIAA 2025-99705, table 1; cited only: https://arc.aiaa.org/doi/10.2514/6.2025-99705
7. T. Lew, F. Lyck and G. Müller, EUCASS 2019-388, pp. 10 to 14, fig. 6, eq. 30; authors' copyright, cited only:
   https://www.eucass.eu/doi/EUCASS2019-0388.pdf
8. RocketPy (MIT), `data/rockets/calisto/air_brakes_cd.csv`, `data/rockets/camoes/drag_airbrakes.csv`,
   `docs/user/airbrakes.rst`, commit 9bd6ad3: https://github.com/RocketPy-Team/RocketPy
9. Good and Joubert, J. Fluid Mech. 31(3), 547 to 582 (1968), abstract: https://doi.org/10.1017/S0022112068000327
