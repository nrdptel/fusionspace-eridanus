# Tumbling versus fixed: measured drag of rotating bodies against broadside ones

This note asks whether a tumbling or autorotating body has more or less drag than the same body held broadside, and by
how much. It serves the tumble model (`TUMBLE_BODY_DRAG_COEFFICIENT` 0.56 and `TUMBLE_FIN_DRAG_COEFFICIENT` 1.42 in
`crates/hpr-sim/src/recovery.rs`), its −10 to +19% replay (`docs/physics/recovery.md`, `docs/accuracy.md`) and
issue #179's booster descent band. It builds on [broadside-body-drag.md](broadside-body-drag.md) (quasi-static
averages, strip theory) and [streamer-and-tumble-drag.md](streamer-and-tumble-drag.md), repeating neither. **Trust:**
every rotation number is for flat plates; no free source measures a tumbling cylinder or finned stage against the
same body held fixed at Re 1e4 to 1e6. Read 2026-10-29. **Inference** marks the note's own arithmetic.

## In short

- **Rotation raises drag above the quasi-static average; it stays near the fixed broadside value.** Three
  autorotating-plate sources (two measured, one a model fitted to measured flights) average 1.05 to 1.6 on plate area:
  +35 to +59% over the quasi-static mean (two have one), 0.74 to 1.31 of the same plate held broadside (0.91 to 1.13
  where the plate's shape is known) (§1). Strip theory's +4 to +15% at the reported spin rates is far too small, so it
  is no bound.
- **An autorotating part also glides.** One measured plate fell 37% slower vertically than its drag alone would
  allow (§2). For a rocket this side force is unmeasured: it would slow the descent and add drift, by an unbounded
  amount.
- **Quasi-static averaging is itself a choice worth 2×.** Crossflow weighting gives 0.42 to 0.59 of broadside drag;
  fragment practice (fixed coefficient times mean projected area) gives 0.66 to 0.94 (§3).
- **The drop tests behind the model agree with the plates.** Under hpr's area reading, a refit of the same five
  drops gives a body constant of 0.76 and a fin constant of 1.07, every model within −2.8 to +3.6% (today −10 to +19%).
  The tumbling tube then sits at about 1.07 of its broadside value, inside the plates' range (§5). This is a fit,
  not a validation: the same five drops, two parameters.
- **Coffee's NAR report measures fixed orientations, not tumbling** (§4): a small rocket hung broadside under an
  8 in chute fell 8.6% slower than hung end-on, a 1.4 standard-error difference; under a 16 in chute, none.

## 1. Autorotating plates: the only measured rotation data

Each row is a plate spinning about its span while it falls or sits in a wind tunnel. "Spin" is the tip-speed ratio
S = ωc/(2V), with c the chord (Smith's 0.35 is his own rate, not confirmed in this convention). Coefficients are
mean drag over a cycle on the plate's planform area.

| source | plate, Re | spin S | mean C_D | fixed broadside | quasi-static | ratio |
| --- | --- | --- | --- | --- | --- | --- |
| Smith [1] | tunnel and free fall, 2.4e5 | 0.35 | 1.6 | 2.0 to 2.15 (2-D); 1.22 (l/w 5) | 1.19 (2-D) | 0.74 to 1.31 |
| Dupleich [2] | card 37.5 × 15 cm, free fall, 3.3e4 | 0.67 | 1.05 to 1.07 | 1.16 (l/w 2.5) | not given | 0.91 to 0.92 |
| Kordi [3] | thin square, Tachikawa's flights | stable | 1.30 | 1.15 (l/w 1) | 0.82 | 1.13 |

"Ratio" is rotating over fixed broadside.

- **Smith** [1] (abstract): at Re ≈ 240,000 the mean lift was about 2.2 and the mean drag 1.6, with peaks of 4.5
  and 4.3. His nondimensional spin rate (defined in the full text) leveled near 0.35, and free-falling plates above
  Re ≈ 4,000 matched. Stall was delayed until the plate was nearly broadside. The abstract states neither the
  reference area nor the aspect ratio, so the static pair is a bracket: a 2-D plate at 2.0 (normal force; 2.15 with
  Kordi's skin friction, p. 32) or a finite one at 1.22 (TN D-540 Fig. 5, p. 13, l/w 5) [5].
- **Dupleich** [2], NACA TM 1201: a 37.5 × 15 cm card of 9.8 g/dm² glided at 47°15′ below horizontal at 331 cm/s,
  spinning at 4.7 rev/s (pp. 21 to 22). **Inference:** drag along the path is W sin 47.25°, so C_D = 1.05 to 1.07
  on plate area (ρ 1.20 to 1.225 kg/m³), C_L 0.97 to 0.99, S = 0.67, Re ≈ 33,000. His fitted law for heavy plates
  (p. 15), decoded the same way, gives 0.98 for this card, within 9%. TN D-540 Fig. 5 (p. 13, by eye) [5] puts a
  fixed plate of l/w 2.5 at about 1.16. Light plates of aspect ratio near 1 spin irregularly or not at all (p. 10).
- **Kordi** [3] (PhD thesis, Western Ontario): a quasi-steady model fitted to Tachikawa's 1983 wind-driven plates
  splits a thin square plate's drag at stable autorotation into a static mean over a half cycle of 0.82 and a
  rotational part of about 0.48 (p. 18; on plate area l·B, p. 10). The rotational part grows with spin, from 0 at
  rest to its full value at the stable rate (eq. 2.3, p. 16). For 2-D plates it is 0.35 (eq. 2.16, p. 33), at
  Skews' S₀ = 0.47 (p. 14; Lugt computed 0.45, p. 13). **Inference:** Kordi's 2-D static model (eqs. 2.11 to
  2.12, p. 32) averages 1.19 over a half turn; adding 0.35 gives 1.54, within 4% of Smith's 1.6.
- **Sign, independently:** Martinez-Vazquez et al. [4] (abstract) measured pressures on fixed and autorotating
  plates; normal forces in autorotation exceeded the static values at most pitch angles.

**Inference (size).** Over the quasi-static mean, rotation adds +35% (Smith, read as 2-D) to +59% (Kordi, square).
Against the same plate fixed broadside, the mean is −8 to −9% (Dupleich) to +13% (Kordi), or −26 to +31% for Smith's
plate read either way. Kordi's eq. 2.3 grows with S/S₀ (piecewise: 62% of the full part at half the stable rate), so a
slower tumble gets less. Strip theory's 1 + S²/3 (broadside note §2) gives only +4% at S = 0.35 (if Smith's rate is
ωc/(2V)), +7% at 0.47 and +15% at 0.67: the gain is a flow effect (Smith's delayed stall), not the q²X² term.

**Why plates are only an analog.** A tube tumbling end over end shows the flow a rectangle of chord L and span d,
turning about its short axis: aspect ratio d/L of 0.05 to 0.4, below every plate above, and with a round edge that
gives no autorotating torque of its own. A fin swung around the center of gravity sees the body's rotation, not
its own. Whether either keeps the plates' gain is unmeasured.

## 2. Lift: an autorotating body glides

Smith's mean lift (2.2) exceeds his mean drag (1.6) [1]; Kordi's square plate keeps a rotational lift of 0.42 [3] (p.
18). **Inference:** Dupleich's card [2] fell 2.43 m/s vertically; at its own C_D with no lift it would fall at 3.86
m/s, so the glide cut its sink rate by 37%. A tumbling rocket's rotation could turn a similar side force into drift
and a slower descent. No source measures it for a cylinder or a finned stage, and Holland et al.'s falling cylinders,
falling in water (abstract) [10], show only that a shifted center of mass makes the motion more complex. The sign is
known (slower descent, more drift than a drag-only model); the size is unbounded.

## 3. Two quasi-static conventions for a tube

The broadside note's 0.42 to 0.59 weights a tube's crossflow drag by sin³α: normal force C_D90 sin²α, projected
on the flow (TR R-474's crossflow form [14]). Fragment ballistics instead keeps the coefficient fixed and averages the
projected area; Moxnes et al. [6] (abstract) state that tumbling "only affects the expected projected area".
**Inference:** a tube's projected area along the flow is L·d|sin α| + (πd²/4)|cos α|. Averaged over a turn in one
plane that is (2/π)(1 + πd/(4L)) of L·d; over random attitudes (Cauchy's mean, surface area over 4) it is
(π/4)(1 + d/(2L)). For l/d 20 to 2.5 that is 0.66 to 0.84 in one plane and 0.81 to 0.94 at random, against 0.42 to
0.59 for crossflow weighting: the conventions differ by 1.4 to 2.0×, before any rotation gain. Static normal
force on inclined bodies is usually built the crossflow way (TR R-474, broadside note), so the projected-area form
overstates a slow, quasi-static tumble. The plates (§1) suggest real rotation lands at or above it anyway.

## 4. Coffee's report: fixed orientations under a canopy

Coffee [7] (NAR R&D, NARAM-57, 2015, B division; a scan with no text layer, read from page images) compares a
5.21 g rocket hung broadside and end-on under a parachute. It has no tumbling case.

- **Drops** (Table 3, PDF p. 9): five timed drops each from a five-story garage, outdoors with wind and thermals.
  With an 8 in chute: broadside 5.32 s (SD 0.63), 2.86 m/s; end-on 4.88 s (SD 0.30), 3.13 m/s. With a 16 in chute:
  1.84 against 1.82 m/s. **Inference:** about 15.2 m of fall; broadside was 8.6% slower under the small chute, 1.4
  standard errors, short of 95% significance; no difference under the large one. The sign matches the broadside
  note's §6 (a broadside body adds drag area under a small canopy).
- **Areas:** the table's 0.0219 m² side and 0.0001 m² end would need an 11 mm tube 1.94 m long (**inference**),
  so Table 4's coefficients (0.48 broadside, 86.9 end-on, 8 in chute; PDF p. 10) are not usable.
- **Wind tunnel** (Table 2, PDF p. 7): broadside force 11 to 13× end-on at 4.6 to 6.1 m/s. The report says its
  force readings lagged its speed readings (PDF p. 5). **Inference:** at 4.57 m/s the broadside force implies
  C_D·A = 0.062 m², or 2.8 on its own side area, above any static cylinder; the tunnel numbers can't be used.
- The report's conclusion: orientation matters less as the canopy grows.

## 5. What the model's own drops want

The OpenRocket technical documentation's Table 3.3 (p. 54) [8], as `the_tumble_model_against_its_own_drop_tests`
replays it (one fin's trapezoid times Table 3.4's factor; body d·l; ρ 1.31 kg/m³; speeds read to ±0.3 m/s, ±4.5 to
6%). **Inference**, a least-squares fit of relative speed error over both constants (grid 0.005 and 0.002; best 1.07,
0.758, printed rounded):

| constants (fin, body) | #1 | #2 | #3 | #4 tube | #5 fins | spread |
| --- | --- | --- | --- | --- | --- | --- |
| 1.42, 0.56 (shipped) | −5.8% | −5.4% | −7.2% | +19.0% | −10.0% | −10 to +19% |
| 1.07, 0.76 (refit) | −1.8% | −1.7% | −2.8% | +2.2% | +3.6% | −2.8 to +3.6% |
| 1.06, 0.79 (body from #4) | −2.3% | −2.3% | −3.2% | +0.2% | +4.1% | −3.2 to +4.1% |

- **Body.** TN D-540 Fig. 5 [5] gives about 0.71 broadside on side area at l/d 2.5 (Re 88,000). The shipped 0.56 is
  0.79 of it; the refit's 0.76 is 1.07 of it, inside the plates' 0.91 to 1.13. Read as quasi-static 0.56 plus §1's
  +35 to +59%, the body term would be 0.76 to 0.89.
- **Fins.** 1.07 sits below the static fins' 1.19 to 1.33 (Koenig, broadside note §3) and at Dupleich's rotating
  1.05 to 1.07. The fit doesn't need a coefficient above every static value.
- **Size of the change.** Descent speed goes as one over the square root of drag area. A body-dominated stage
  would fall up to 14% slower; a fin-dominated piece up to 15% faster.
- **Limits.** Five drops of 44 to 103 mm models at Re about 2e4 to 5e4, l/d 2.3 to 2.8; two parameters
  leave three degrees of freedom. The fit holds only for hpr's reading of the areas, which the documentation does
  not pin; its residuals are below the timing error, so it shows the data allow these values, not that they hold.

## 6. Other leads read

- Nau and de la Iglesia [9] (NARAM-38, 1996): long gap-staged boosters are aerodynamically stable and streamline
  down nose-first, so they need a device; twelve flights, no descent rates. This supports the broadside note's §7.
- Reynolds [11]: photogate drops of tube fins against flat fins, no tumbling. Randall and Klett [12]: tumbling
  cylinders at hypersonic speeds and Mach 10 to 30, nothing at descent speeds.

## What the project can use

- **Terms.** TN D-540 is a US Government work; NACA TM 1201 translates a 1941 French paper (rights unstated); the
  Sandia memos are AEC-contractor reports on OSTI. Kordi's thesis (© the author, 2009) and the NAR reports (member
  content) are cite only. All are cached in the gitignored `refs/`, never committed. The OpenRocket documentation is
  CC BY-SA; its Table 3.3 numbers are already in hpr's tests.
- **Tumble constants (suggestion, needs an ADR).** Keep 0.56 and 1.42 for comparing with OpenRocket, and document
  that the same five drops, read hpr's way, prefer 0.76 and 1.07 (−2.8 to +3.6%), as plate data would predict.
  Adopting them would shrink the in-sample spread from −10 to +19% to about ±4%, but it's the same data. The
  accuracy row should still say "in-sample" until an independent tumbling drop exists.
- **The 1.42 constant.** It now has a free comparison: rotating plates average 1.05 to 1.6 on their own area
  [1, 2, 3], so 1.42 is plausible for a rotating fin, though a refit doesn't need it.
- **#179's booster band.** Replace the quasi-static floor with the plates' range. **Inference:** a tumbling mean of
  0.74 to 1.31 × broadside puts the tumbling booster at 0.87 to 1.16 × its flat-fall speed, not the 1.3 to 1.5 ×
  that 0.42 to 0.59 gives: 13% slower to 16% faster than flat, with glide (§2) an unbounded term slowing it.
- **A cheap measurement would settle it:** timed drops of one finless tube and one finned model, free to tumble,
  then the same bodies held broadside in a tunnel or on a fixed mount, with video for the spin rate.

## Open questions

- Tumbling cylinders and finned stages: the five drops (§5) tumble, but no free source pairs a tumbling body with
  the same body held fixed at Re 1e4 to 1e6. Unread lead: Richards et al. 2008 rod flights (via Saini, Shafei 2022).
- Smith's reference area and aspect ratio (full text paywalled), which set whether his ratio is 0.74 or 1.31.
- Whether a free tumbling tube settles at a stable spin rate, and its S. The plates sit at 0.35 to 0.67.
- Not reached on 2026-10-29: Bustamante, Sandia SC-RR-68-132 (DTIC ADA395124) and APEX-299 (ADA395503). DTIC
  showed a maintenance page to plain requests and in a browser, twice each. Also unread: the full texts of Iversen
  1979 [13], Tachikawa 1983, Lin et al. 2006 and Holland 2004 (paywalled), and Moxnes 2017 (open access; not opened).
- Above the drag crisis (Re > 2e5 for tubes): no rotating data for a round section; sharp-edged plates have none.

## Sources

All read 2026-10-29.

1. Smith, "Autorotating wings: an experimental investigation", J. Fluid Mech. 50(3), 513 to 534 (1971), abstract.
   <https://doi.org/10.1017/S0022112071002738>
2. Dupleich, "Rotation in free fall of rectangular wings of elongated shape", NACA TM 1201 (1949; French original
   1941). <https://ntrs.nasa.gov/api/citations/20050029433/downloads/20050029433.pdf>
3. Kordi, "Aerodynamics of wind borne plate debris", PhD thesis, University of Western Ontario (2009).
   <https://hdl.handle.net/20.500.14721/18851>
4. Martinez-Vazquez, Baker, Sterling, Quinn, Richards, "Aerodynamic forces on fixed and rotating plates", Wind and
   Structures 13(2), 127 to 144 (2010), abstract. <https://koreascience.kr/article/JAKO201015541094848.do>
5. McKinney, NASA TN D-540 (1960). <https://ntrs.nasa.gov/api/citations/20040047039/downloads/20040047039.pdf>
6. Moxnes et al., "Projected area and drag coefficient of high velocity irregular fragments that rotate or tumble",
   Def. Technol. 13(4), 269 to 280 (2017), abstract on FFI's archive page (ffi.no, Publikasjoner, Arkiv).
   <https://doi.org/10.1016/j.dt.2017.03.008>
7. Coffee, "The Effects of Rocket Orientation on Drag During Descent", NAR R&D, NARAM-57 (2015), members only.
   <https://www.nar.org/docs.ashx?id=1358299>
8. Niskanen, OpenRocket Technical Documentation v13.05 (CC BY-SA), §3.5, Tables 3.3 and 3.4 (pp. 54 to 55); a
   release asset of <https://github.com/openrocket/openrocket>, as replayed in `crates/hpr-sim/src/recovery.rs`.
9. Nau, de la Iglesia, "Investigation Into Gap-Stage Booster Recovery", NAR R&D, NARAM-38 (1996), members only.
   <https://www.nar.org/docs.ashx?id=1357944>
10. Holland et al., Exp. Fluids 37(5), 690 to 700 (2004), abstract. <https://aquila.usm.edu/fac_pubs/2983>
11. Reynolds, "Drag Analyses Tubular Fin Drop Test", NAR R&D, NARAM-41 (1999), members only.
    <https://www.nar.org/docs.ashx?id=1357981>
12. Randall, SC-TM-64-528 (1964), <https://www.osti.gov/servlets/purl/4385919>; Klett, SC-RR-64-2141,
    <https://www.osti.gov/servlets/purl/4630398>.
13. Iversen, J. Fluid Mech. 92(2), 327 to 348 (1979), abstract. <https://doi.org/10.1017/S0022112079000641>.
14. Jorgensen, NASA TR R-474 (1977). <https://ntrs.nasa.gov/api/citations/19770026166/downloads/19770026166.pdf>
