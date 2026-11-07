# Brake wake behind a finite-width plate: how far downstream it reaches, in plate heights

What this covers: how far behind a plate of finite width standing out of a wall the wake keeps a dynamic pressure
deficit of more than 10%, in plate heights, and how that distance moves with the plate's width, its height against the
boundary layer, and Mach number to 0.9. It is for issue #867, M6.4's deployed-brake stability margin: the distance
ahead of the fins beyond which a deployed-brake warning may go silent. Silencing it too close is the flattering side.
How far to trust it: **no one has measured the wake of a thin finite plate beyond 5 plate heights at the brake's
height-to-boundary-layer ratio, nor at any Mach above low speed.** The distances below come from blocks in thick,
turbulent wind-tunnel layers (lower bounds) and from extrapolating free-plate wakes (*inference*). Builds on
[airbrake-wake-fin-loss.md](airbrake-wake-fin-loss.md) §3, which bounded the wake only by a free plate and by
full-width fences. Researched 2026-11-07.

**In short.** 10 plate heights is not enough. Behind every measured body 2.4 to 4 times as wide as tall, the wake still
lacks more than 10% of local dynamic pressure at 10 heights, even in thick, very turbulent layers that refill a wake
fast: it reaches 90% at **14 to 23 heights** behind blocks of width 3 to 4 heights, and at 7 to 10 behind a cube. Each
way a brake differs from those tests (a thin plate instead of a block, a boundary layer thinner than the plate, the
quiet air a rocket flies in) makes the wake longer. Extrapolated, the 90% point lies at about **20 to 80 heights for a
square brake and 40 to 250 for one 3 times as wide as tall** (*inference*; the low ends take the fastest decay measured
near a wall, the high ends the slowest form for a free wake). Mach makes it longer again: on one step, reattachment
lengthened 11% from Mach 0.48 to 0.79 and the vortex center moved 47% farther back by Mach 0.9; slower shear-layer
growth allows up to about 1.9 times the low-speed length at Mach 0.9; near Mach 1 the length is unbounded. For a brake
reaching about 0.46 body diameters, as in the rocket study the parent note leans on, 20 heights is about 9 diameters
and 40 about 18 (*inference*): no distance on a rocket of ordinary length is shown to be clear.

## Conventions

- **h**: the plate's (brake's) height out of the wall; **w**: its width across the flow; **D** (blocks): depth along
  the flow; **δ**: the boundary layer's thickness; **x**: distance downstream of the plate, in h.
- **q_w/q**: dynamic pressure in the wake over the undisturbed value at the same height, taken as
  ((U_local − Δu)/U_local)². It ignores static pressure, which behind a free plate is back to free stream just past
  the bubble (R&M 3120 p. 5). Converting a source's Δu/U∞ to local q by its stated power-law profile is
  *inference*.
- **"Thick layer"**: blocks at h/δ ≤ 0.25 in 15% to 26% turbulence, as in building tests. **"Brake regime"**: h/δ 1
  to 5 in a quiet outer stream, as on a rocket.

## 1. Reattachment behind a finite plate

- **Square plate, h/δ = 5.7 (Yamada and others 1987),** the one thin plate found in the brake regime: 56 mm square in
  a 9.8 mm turbulent layer at 15 m/s (p. 1697). Mean flow reattaches about **3.4 h** behind it (fig. 3a, p. 1698). The
  centerline wall pressure coefficient is lowest near 1.6 h (−0.58), zero near 3.4 h and +0.05 at 7.5 h (fig. 7, p.
  1699).
- **Free plates as mirror images (ARC R&M 3120, Fail, Lawford and Eyre 1957).** Behind a free rectangular plate the
  bubble ends at 2.96 √S for aspect 1, 2.86 √S for 2, 2.46 √S for 5 and 2.26 √S for 10 (table 1, p. 9; S is its
  area). A wall plate and its image make a free plate w × 2h, so (*inference*): w/h 0.5, about 2.6 h; w/h 1, 4.0 h;
  w/h 2, 5.9 h; w/h 3, about 7.1 h. Yamada's 3.4 h is 16% under the mirror's 4.0 h
  (the two define reattachment differently; *inference*).
- **Toward two dimensions.** At h/δ 0.42 and 0.81 the shedding loses its dominant frequency above w/h = 3
  (Sakamoto, Moriya, Taniguchi and Arie 1982, part 2, p. 1684, fig. 3, p. 1685; Strouhal data span h/δ 0.25 to 0.98),
  and smoke in a laminar layer shows its form changing near w/h = 0.8 (p. 1690). The wall-pressure low
  sits 1.1 h behind a w/h = 1 plate and 2 h behind a w/h = 4 plate at h/δ = 0.58 (fig. 11, p. 1688): the bubble grows
  with width. So w/h = 3 falls between the mirror's 7 h and a full fence's 10 to 12.5 h: **7 to 12.5 h**
  (*inference*).
- **Blocks are a poor stand-in.** Behind blocks as deep as tall, the decay curves' flat recirculation region ends at
  1.1 to 1.5 H for W/H 0.5 to 2 and 2.5 H for W/H 3 and 4 (Woo, Peterka and Cermak 1976, table 9, p. 106).

## 2. Centerline deficit against distance, measured in thick turbulent layers

- **Woo, Peterka and Cermak (1976).** Wide, low blocks 2 in tall and as deep, in a 28 in layer (H/δ = 0.07, 18%
  turbulence at H; pp. 98, 100), hot wire on the axis at the block's height (fig. 112, p. 225). The deficit Δu/U∞ is
  about 0.30 (W/H 3) and 0.33 (W/H 4) at 1 H, easing to about 3 H, then falls as x to the power −1.37 and −1.48;
  −1.18 for a cube; −1.06 and −0.97 for tall narrow blocks (W/H 0.33, 0.25). With local speed 0.52 U∞ at that
  height, **q_w/q reaches 0.9 at 7 to 10 H for the cube, 14 to 18 H for W/H 3 and 19 to 23 H for W/H 4**; at 10 H it
  is 0.78 (W/H 3) and 0.72 (W/H 4) (*inference*, read from the figure).
- **Hansen, Peterka and Cermak (NASA CR-2540, 1975).** A block 2.56 in tall, 6.24 in wide, 1.92 in deep (W/H 2.44) at
  H/δ = 1/9.4 (p. 11); the report says the wake "decayed within 16-18 building heights" (p. 14). On the axis at about
  H (fig. 5, p. 30), Δu/U∞ is 0.33, 0.16 and 0.06 at 2.55, 7.17 and 12.86 H (fig. 5's stations), so q_w/q ≈ 0.17,
  0.51 and 0.80 with local speed 0.56 U∞ (*inference*).
- **Cubes in very turbulent flow** refill fastest: a few percent deficit at 4.5 H and none at 8.5 H at 26%
  turbulence (Castro and Robins 1977, as Woo pp. 23 to 24 report it).
- **Decay law.** Hunt's theory gives x^−3/2 behind a three-dimensional obstacle in a boundary layer and x^−1
  behind a two-dimensional one (as Woo p. 28 quotes it; not read). Free wakes in a quiet stream decay more slowly,
  as x^−2/3 in the classical form and about x^−1 in the non-equilibrium form, seen out to about 100 plate lengths
  (Obligado, Klein and Vassilicos 2021, section 1, citing earlier work).

## 3. Off the centerline, at about one width to the side

- Only CR-2540 traverses the wake (W/H 2.44, at 0.94 H, fig. 5, p. 30). At y = ±W the deficit is near zero at 0.56 and
  2.55 H, with up to 0.1 U∞ of speed-up farther out. As the wake spreads it reaches 0.05 to 0.07 U∞ at 4.9 to 7.2 H
  (**q_w/q ≈ 0.77 to 0.83**), and falls to 0.02 to 0.03 U∞ by 12.9 H (0.90 to 0.93) (q *inference*). The wake spans
  about five widths at 12.8 H (p. 14). So a fin one width to the side is clear close behind the body, and in the wake
  from about 5 heights back (*inference* for a brake as wide as tall). No off-axis data were found for a thin plate or
  a quiet stream.

## 4. What lengthens the wake, with sign and size

- **Larger h/δ: longer.** On the axis q_w/q = 0.9 comes at about 9 H for H/δ = 0.05 and 16 to 17 H for 0.107
  (CR-2540 fig. 9, p. 34, *inference*). Square-plate drag at 1 < h/δ ≤ 2 grows as 0.98 (h/δ)^0.28 (Sakamoto and
  others 1982, part 1, eq. 7, p. 1679), and the wake's momentum deficit with it. At h/δ 1 to 5: not measured.
- **Quieter outer flow: longer.** The wake's extent behind a cube grows from 13 H at 18% turbulence to 20 to 30 H at
  8% to 9%, and behind the W/H 2.44 block from 23 H to 32 to 42 H at 7.8% to 9% (Woo table 9, pp. 106 to 107; mixing
  by the approaching turbulence speeds recovery, p. 84). One of those runs also changed H/δ and the profile, so not
  all of the change is turbulence (p. 87). A rocket's outer flow is quieter still (*inference*: longer again).
- **Thin plate instead of a block: longer.** A 3.4 h bubble against a cube's 1.5 H; at 5 h, q_w/q ≈ 0.4 to 0.65
  behind the plate (Yamada fig. 3a, vector lengths, ±10%) against about 0.81 behind the cube (*inference*).
- **Quiet-stream extrapolation (*inference*, no upper bound).** Free plates of aspect 1 to 5 keep about 0.72 to 0.78
  U near the axis two bubble lengths back (R&M 3120 fig. 11c, p. 19), q_w/q ≈ 0.55. Cutting that 0.23 velocity
  deficit to 0.05 takes 2.8 times the distance at x^−1.5 (the wall-layer law), 4.6 at x^−1 and 9.9 at x^−2/3 (the
  free-wake forms): q_w/q = 0.9 at 5.5, 9.2 or 19.7 bubble lengths. With bubbles of 4.0 h (w/h = 1) and 7.1 to
  12.5 h (w/h = 3): **22, 37 or 79 h for w/h = 1; 39 to 69, 65 to 115 or 140 to 250 h for w/h = 3.** Virtual origins
  are ignored; this sizes the answer, it does not measure it.

## 5. Mach number to 0.9: longer, and a little deeper

- **The one open Mach sweep (Saile, Kühl and Gülhan 2019),** a separated bubble behind an axisymmetric step (step
  about 20 mm on a 66.7 mm body, PIV at Mach 0.48 to 0.90; §2, table 3). Reattachment length grows from 1.06 to 1.18
  diameters between Mach 0.48 and 0.79 (+11%; about 3.5 to 4.0 step heights, *inference*). Saile reports it close to
  the model's end at Mach 0.7 to 0.8 and past it at 0.9, where an estimated 1.36 diameters (+28%) is given; the end
  may cap the measured values (*inference*). The recirculating vortex's center moves from 0.51 to 0.75 diameters
  between Mach 0.48 and 0.90 (+47%). Scaled by its own length, the bubble keeps one shape at every Mach (figs. 6, 7,
  for r/D ≥ 0.2). Peak turbulence intensity rises from 0.14 to 0.18 (§3.2.2). An earlier study it quotes gives +7%
  from Mach 0.5 to 0.77. Every ratio here starts at Mach 0.48, not low speed, so it understates the stretch.
- **A two-dimensional step (Bolgar, Scharnowski and Kähler 2018, abstract only):** reattachment moves downstream from
  subsonic to transonic flow, then shifts suddenly once the flow behind the step is locally supersonic.
- **The mechanism.** A separated shear layer spreads more slowly as its convective Mach number rises. Growth fell to
  about 0.70 of its low-speed value at convective Mach 0.33 and 0.39 and 0.50 at 0.55 in Papamoschou's thesis (1987,
  fig. 5.3, p. 158), while Birch and Eggers (NASA SP-321, 1973, fig. 6) find no change below Mach 1 for a stream
  mixing with still air. A brake at Mach 0.9 sits near convective Mach 0.43 to 0.52 (*inference*), where
  Papamoschou's points give 0.66 to 0.54: its shear layer spreads 0% to about 45% more slowly and needs up to about
  1.85 times the distance to close (*inference*).
- **A tail behind a fuselage brake (NACA RM A7K28, Morrill and Boddy 1948),** new to this topic. A side brake about
  5 in wide, hinged 16.0 in (about 3.2 brake widths) ahead of the tail's quarter chord (pp. 4 to 5, fig. 3; distance
  *inference*), cut the elevator's pitching moment per 5° from 0.0372 to 0.0246 at Mach 0.3 (34%) and from 0.0292 to
  0.0190 at Mach 0.9 (35%) (fig. 25, read against the grid, ±0.001). The report calls the loss roughly constant with
  Mach (p. 10). The tail kept about 0.65 of its effectiveness and did not recover as Mach rose (*inference*).
- **Stretch on a low-speed distance (*inference*):** about 1.0 to 1.1 at Mach 0.5, 1.1 to 1.4 at Mach 0.8, up to
  about 1.9 at Mach 0.9; from Mach 0.95 to 1.05, unbounded. These come from steps and shear layers, not from plates on
  a wall.

## 6. What it means for a rocket brake (*inference* throughout)

| Brake | Reattaches | q_w/q = 0.9 on axis: measured floor | brake-regime estimate, low speed |
|---|---|---|---|
| w/h ≈ 1 | 3.4 to 4 h | 7 to 10 h (cube, thick layer) | 22 to 79 h |
| w/h ≈ 3 | 7 to 12.5 h | 14 to 18 h (block, thick layer) | 39 to 250 h |

- Multiply by up to 1.4 at Mach 0.8 and 1.9 at Mach 0.9. The floors are measured in flows that refill a wake faster
  than a rocket's; the estimates are extrapolations with no ceiling.
- A rocket is a body, not a flat wall: the wake can spread round the body and, at incidence, crossflow carries it
  toward the leeward fins. Neither is in these data. A fin at 45° from the brake is about 0.39 D round the skin; for
  a brake 0.46 D tall and as wide, that is about one width off axis, inside the wake from about 5 h back (section 3).
- At 9 D for 20 h, even the lowest estimate for a square brake spans much of an ordinary rocket's length. That is
  consistent with the WVU study's brakes 6 D (about 13 h) ahead of the fins still costing up to 1.14 calibers, though
  part of that loss is the brakes' own normal force (parent note §1, §6).

## What this suggests

- **#867:** a distance beyond which the deployed-brake warning goes silent has no cited support. The measured floor
  is 14 to 23 h behind wide blocks in the most favorable flows found; the brake-regime estimates run from 22 h
  (square brake, fastest decay) to 250 h (w/h = 3, slowest), stretched by up to 1.9 at Mach 0.9. The rule these data
  support is to warn for any deployed brake ahead of the fins, giving the measured figures as the reason. If a
  distance is wanted anyway, nothing under about 80 h for square brakes is defensible on the free-wake form.
- The warning text can say what is measured: even in the most favorable tests found, the wake still lacks over 20%
  of dynamic pressure 10 heights behind a block 3 times as wide as tall.
- A measurement that would settle it: a hot-wire or PIV traverse behind a thin plate (w/h 1 and 3) at h/δ 1 to 5 out
  to 100 h, on axis and at ±w, at low speed and Mach 0.7 to 0.9.

## Open questions

- No wake data past 5 h for a thin finite plate at h/δ ≥ 1; none off axis for a plate in a quiet stream. Sakamoto
  and Arie (1983), Castro and Robins (1977), Martinuzzi and Tropea (1993) and Fang and Tachie (2019) are paywalled
  and were read in abstract only; they may hold more.
- Not read: Hunt (1971) and Counihan, Hunt and Jackson (1974), seen only through Woo p. 28; Peterka, Meroney and
  Kothari (1985); Hosker (1984)'s cavity-length fit; ESDU fence items; Hoerner's *Fluid-Dynamic Lift*.
- No wake survey behind any finite-width plate, spoiler or fuselage brake at Mach 0.5 to 0.9 was found; the Mach
  stretch rests on a step, a base and shear layers. Bolgar (2018)'s lengths are in its paywalled full text.
- How far edge speed-up raises a brake's convective Mach, and how a body's curvature and incidence move the wake.

## Sources (read 2026-11-07)

NASA and NACA reports are US government works (cached). ARC reports are Crown copyright; JSME papers are © JSME (free
on J-STAGE); Saile is CC BY 4.0; all of these are cached. The CSU report and the thesis are cached for reading and
cited only; the arXiv preprint and the paywalled papers are cited only. No table or figure is copied.

1. H. Yamada, H. Osaka, Y. Kageyama, M. Furuya, Trans. JSME B 53 (490), pp. 1696 to 1703 (1987), in Japanese with an
   English abstract; pp. 1697 to 1699, figs. 3a, 7: https://doi.org/10.1299/kikaib.53.1696
2. H. Sakamoto, M. Moriya, S. Taniguchi, M. Arie, Trans. JSME B 48 (433), pp. 1674 to 1682 (part 1, drag), p. 1679;
   https://doi.org/10.1299/kikaib.48.1674; and pp. 1683 to 1691 (part 2, shedding), pp. 1684, 1688, 1690:
   https://doi.org/10.1299/kikaib.48.1683
3. R. Fail, J. A. Lawford, R. C. W. Eyre, ARC R&M 3120 (1957), pp. 5, 9, 19:
   https://reports.aerade.cranfield.ac.uk/handle/1826.2/3689
4. A. C. Hansen, J. A. Peterka, J. E. Cermak, NASA CR-2540 (1975), pp. 11, 14, 16, 30, 34:
   https://ntrs.nasa.gov/citations/19750012586
5. H. G. C. Woo, J. A. Peterka, J. E. Cermak, Colorado State University CER75-76HGCW-JAP-JEC40 (1976), pp. 23 to 24,
   28, 84, 87, 98 to 107, 225: http://hdl.handle.net/10217/48067
6. M. Obligado, S. Klein, J. C. Vassilicos, arXiv:2103.14377 (2021), section 1: https://arxiv.org/abs/2103.14377
7. D. Saile, V. Kühl, A. Gülhan, Exp. Fluids 60, 50 (2019), CC BY 4.0, §§2, 3.2, table 3, figs. 6, 7:
   https://doi.org/10.1007/s00348-019-2690-9 (open copy: https://elib.dlr.de/126714/)
8. I. Bolgar, S. Scharnowski, C. J. Kähler, Flow Turbul. Combust. 101, pp. 653 to 680 (2018), abstract only:
   https://doi.org/10.1007/s10494-018-9921-7
9. D. Papamoschou, PhD thesis, Caltech (1987), p. 80, fig. 5.3 (p. 158): https://doi.org/10.7907/M7PQ-Y494
10. S. F. Birch, J. M. Eggers, NASA SP-321 (1973), pp. 19 to 20, fig. 6: https://ntrs.nasa.gov/citations/19730019424
11. C. P. Morrill, L. E. Boddy, NACA RM A7K28 (1948), pp. 4 to 5, 10, figs. 3, 25:
    https://ntrs.nasa.gov/citations/19930085821
12. Abstracts only, cited: I. P. Castro, A. G. Robins, J. Fluid Mech. 79, pp. 307 to 335 (1977),
    https://doi.org/10.1017/S0022112077000172; R. Martinuzzi, C. Tropea, J. Fluids Eng. 115, pp. 85 to 92 (1993),
    https://doi.org/10.1115/1.2910118; H. Sakamoto, M. Arie, J. Fluids Eng. 105, pp. 98 to 104 (1983),
    https://doi.org/10.1115/1.3240950; X. Fang, M. F. Tachie, J. Fluid Mech. 877, pp. 717 to 758 (2019),
    https://doi.org/10.1017/jfm.2019.617
