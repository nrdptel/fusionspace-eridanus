# ADR-009: Subsonic drag buildup, surface finishes and drag override tables (2026-09-17)

- **Status:** accepted; the held nose drag and the `M ≥ 1` refusal superseded by ADR-028
- **Summary:** Subsonic drag buildup, surface finishes and drag override tables

**Context.** M1.5b adds drag. Niskanen's thesis (2009, §3.4 and appendix B) is the primary
source; the OpenRocket technical documentation 13.05 reprints the same drag equations unchanged,
and Barrowman's 1967 thesis (ch. 4) is the source of its friction, roughness and leading-edge
formulas. The sources leave gaps: no coefficients for drag at angle of attack, undefined areas in
the boattail rule, an unstated diameter in the lug rule, no rail buttons, and friction that jumps
where eq. 3.81 switches branches. Loft merged fin sets (L11), used uncited constants (L12), had no
power-on base relief (L13), uncited lug drag (L14), no drag at bare steps (L15) and a silent cap of
10 (L16). The done-when compares with RocketPy's RASAero curves, whose inputs aren't recorded.

**Decision.**

- **Buildup.** `C_D0` = friction + pressure + base + parasitic on the reference area (eq. 3.75,
  3.97); `C_A = C_D0 f(α)`, positive toward the tail. Each component keeps its own terms (fin sets
  are never merged), and `AeroModel::buildup_components` reports them.
- **Friction.** Fully turbulent (Niskanen p. 43, who measured laminar runs changing apogee by under
  5%), `R` on the rocket's length (nose tip to
  the aft end of the last body component), eq. 3.78–3.84 as printed, the `R < 1e4` branch first,
  keeping the jumps at `R_crit` and at Mach 1. Roughness is per component (`hpr_design::Finish`),
  always over the rocket's length. The body form factor `1 + 1/(2 f_B)` uses body length over
  maximum body diameter; the fin factor `1 + 2t/c̄` is per fin set.
- **No laminar or transitional friction (M1.5's scope bullet names it).** Barrowman 1967's
  transitional form (eq. 4-2, 4-6: laminar below `R = 5e5`, `− 1700/R` above, turbulent throughout
  on any rough surface) and RASAero II's default laminar trip at `5e5` would lower smooth-surface
  friction by `1700/R`, about 4% of it and 2% of `C_D0` on Calisto (`R` 1.8e7). Following Niskanen,
  hpr stays fully turbulent; an option is issue #18.
- **Friction area (a departure).** A body's friction area is its axial projection
  `2π ∫ r dx = π A_plan`, not the slant surface of eq. 3.85: only the axial share of the wall shear
  makes drag. It changes slender noses by about 1% of their own friction (2.4% for a tangent ogive
  of fineness 2). With the slant surface, a shoulder's drag would stay about `C_fc ΔA/A_ref`
  (0.0015 in the L15 test) above a bare step's as its length goes to zero. M2.2 will see the
  difference against OpenRocket.
- **Pressure drag.** Noses and shoulders `0.8 sin² φ`, `φ = atan(dr/dx)` at the aft joint, on the
  increase in area (eq. 3.86), held through subsonic flow. Boattails follow eq. 3.88 on their
  decrease in area. Niskanen writes `A_base/A_boattail` without defining them and says a
  zero-length boattail drags like "the total base drag" (p. 48). Read with `A_base` as the aft
  base, a zero-length boattail would count that base twice and leave the uncovered annulus out, so
  hpr reads both as the decrease in area (Calisto's boattail: 0.052, against 0.046 the other way).
  Steps in radius are zero-length shoulders (`0.8 ΔA`) or boattails (base drag on `ΔA`); a body
  without a nose cone gets `0.8 A` on its face.
- **High subsonic.** Eq. 3.87 interpolates nose and shoulder pressure drag from eq. 3.86 at
  Mach 0 to appendix B's value and slope at Mach 1 (closed forms for cones and ogives, Stoney's
  data for other shapes); all of it arrives with M1.8. Until then the drag reads low from about
  Mach 0.6: a 3:1 tangent ogive misses 0.006 at Mach 0.7 and 0.021 (4–5% of `C_D0`) at 0.8, a 2:1
  cone 0.037 at 0.8. The buildup accepts Mach numbers up to 1 so M1.6 can fly, and sets
  `Drag::beyond_subsonic_methods` above Mach 0.8, the top of Niskanen's subsonic region (Table 3.1);
  the flag marks that edge, not the start of the error.
- **Base drag.** Eq. 3.94 on the last body component's aft area less the thrusting motors'
  cross-section (`DragConditions::thrusting` with the burning motors' area; Niskanen p. 50), down
  to zero.
- **Fins.** Eq. 3.89–3.93 by cross-section (square, rounded, airfoil) on `N t s`; `Γ_L` is
  `atan(x_t/s)` for trapezoids, the span average for freeform outlines, and a closed-form average for
  ellipses. Averaging the angle follows eq. 3.91; the drag goes as `cos² Γ`, whose span average is
  6% lower for an ellipse of `k = 1` (for M2.2).
- **Parasitic.** Launch lugs follow eq. 3.95–3.96 with `d` the outer diameter (the rail-pin passage
  on p. 52 only reads that way). Rail buttons follow the rail-pin rule, the stagnation coefficient
  on their side profile. Neither adds friction.
- **Angle of attack (derived).** Niskanen gives only the shape: 1 at 0°, 1.3 at 17°, 0 at 90°, zero
  slope at each. hpr uses the unique cubic per part that meets those conditions. Past 90° the flow
  meets the tail and hpr mirrors with the sign reversed, `f(α) = −f(180° − α)` (an assumption), so
  drag still opposes the motion. M2.2 compares with OpenRocket.
- **Refusals.** `M ≥ 1` for the buildup (the term functions are defined and finite to Mach 5),
  geometry the terms can't use, negative roughness (already at `Rocket::layout`), a coasting
  condition with a motor area, non-finite conditions and non-finite results are errors. Nothing is
  clamped.
- **Finishes.** `hpr_design::Finish` names the fifteen rows of Barrowman 1967 Table 4-1 (after
  Hoerner p. 5-3; Niskanen Table 3.2 reprints ten) plus a custom height, on `Component::finish`.
  The default is "paint in aircraft mass production", 20 µm: no source gives a hobby-rocket
  default, RASAero II defaults to smooth, and OpenRocket's "regular paint" is 60 µm (Niskanen
  p. 83).
- **Override tables.** `DragTable` holds `C_D0(M)` power-off and optionally power-on, read from CSV
  text: two columns (RocketPy's curves), or a named column of a headed file with rows at non-zero
  `Alpha` skipped (RASAero II's export). An identical repeated row is skipped; a Mach number
  repeated with another value, or out of order, is refused with its line. Linear interpolation,
  end values held, extrapolation reported. `DragConditions::thrusting` selects power-on, and an
  optional reference diameter rescales a table made on another reference area. The angle-of-attack
  factor still applies, and a model with a table accepts any Mach number for drag.
- **The comparison with RocketPy's curves.** Every RocketPy example whose drag curve is labelled
  RASAero: Calisto, Juno III, Cavour and Valetudo. At Mach 0.3 and USSA76 sea level (RASAero II
  computes its exports' Reynolds numbers at sea level, Users Manual p. 84). The curves stay in
  `refs/`; `cargo xtask aero` writes only derived numbers (each curve's value at Mach 0.3, hpr's
  `C_D0`, the error, the file's sha256) to `validation/fixtures/aero/rocketpy-drag-curves.json`,
  `hpr_aero`'s test recomputes hpr's values and the errors, and an xtask test reruns the comparison
  when `refs/rocketpy` is present. Provenance, traced through RocketPy's history:
  - Calisto's curve is a RASAero II export: the alpha-0 `CD Power-Off` column of the `CD Test.CSV`
    in RocketPy's first commit (2018). Its power-on column equals power-off (RASAero's v1 manual:
    power-on equals power-off without a nozzle exit diameter), so only power-off is compared. The
    design with the 2018 notebook's fins is the case: the export's `C_Nα` (6.09 per rad) and CP
    (66.2 in) are within 1.5% of hpr's for those fins (6.18, 66.2 in), not for the getting-started
    fins (7.32, 70.3 in), which are reported as a variant, not as more evidence.
  - Juno III's, Cavour's and Valetudo's are labelled RASAero but are 3-decimal tables with no input
    file. Juno III's serves power-off and power-on alike; Cavour's and Valetudo's have separate
    power-on tables, which are compared too (Calisto's power-on file is its power-off file; hpr's
    power-on result would be −5.0%). Cavour's power-off table has 22 extra rows repeating 13 Mach
    numbers up to 0.107, 7 with values 0.001 apart; the comparison keeps the first of each, and
    `parse_mach_csv` itself refuses that curve.
- **Inputs for the comparison: one declared rule, placeholders where nothing is known.** The
  exports record no inputs. Finish: RASAero II's documented default, smooth (p. 53,
  `Finish::Mirror`). Fin cross-section: a NACA 00xx airfoil file in the example gives an airfoil
  section that thick at the mean aerodynamic chord (Calisto's getting-started fins, NACA 0012); a
  published section is used (Juno III's team was cited for "análise de aletas com perfil de
  aerofólio truncado", an analysis of truncated-airfoil fins: a rounded leading edge and a blunt
  trailing edge, taken as rounded, with no thickness given); otherwise the M1.4b placeholder,
  square 3 mm (Calisto's 2018 fins, Cavour, Valetudo). A lift-curve airfoil says
  nothing about the edges and doesn't count. Rail buttons stay as the RocketPy examples define
  them; Calisto's come from RocketPy's test fixture, not the 2018 notebook.
- **Result, and the gaps it leaves.**

  | case | error | range over the inputs below |
  |---|---|---|
  | Calisto, 2018 fins | +4.4% (+1.8% without the buttons) | −14.0% to +12.8% |
  | Calisto, getting-started fins (variant) | −7.3% | −12.9% to +19.3% |
  | Juno III | −6.0% | −10.5% to +24.1% |
  | Cavour, power-off | −8.3% | −22.3% to −0.4% |
  | **Cavour, power-on** | **−18.3%** | −32.2% to −10.3% |
  | **Valetudo, power-off** | **−47.0%** | −59.4% to −42.5% |
  | **Valetudo, power-on** | **−50.4%** | −62.8% to −45.9% |

  The range is over square, rounded and airfoil fins (3 mm, or 12% of the chord for the airfoil),
  0 or 20 µm, with and without rail buttons. Before the published-section rule, square 3 mm fins
  gave Juno III +14.6% and the getting-started Calisto +10.7%. The test pins exactly the three
  cases outside 10%:
  - **Cavour under power, cause open.** At Mach 0.3, subtracting the motor's area removes 42% of
    Cavour's base drag (0.055) and 29% of Valetudo's (0.038). Cavour's power-on table is within its
    0.001 rounding of power-off from Mach 0.16 up (0.0001 at 0.3; 0.001 to 0.013 lower below),
    Valetudo's 0.004 lower, about a ninth of hpr's relief. The RocketPy designs have no motor case,
    so their motor diameter is the larger of the grain and nozzle exit diameters, and Cavour's
    result runs from −8.3% with no relief to −20.8% with its 75 mm motor (−18.3% at the 67 mm
    nozzle exit). The miss may be Niskanen's relief, a RASAero run with little or no nozzle exit
    diameter, or tables sampled along a flight (their uneven Mach spacing suggests it;
    unconfirmed). Flights with known motors (M2.1, M2.3) will test the relief.
  - **Valetudo's table** gives 1.05 at Mach 0.3, 1.44 times the OpenRocket export for the same
    rocket in RocketPy's RocketPaper repository (0.728). With that `.ork`'s inputs (regular paint,
    60 µm; two 14 mm × 30 mm lugs instead of rail buttons; its 3 mm square fins) hpr gives 0.714,
    1.9% under the OpenRocket export and 32% under the table. RocketPy's own notebook rescales the
    table by 0.9081/1.05.
  - **What the passing cases show.** Unrecorded inputs move each by 20% or more, and each passing
    rocket falls outside 10% under some plausible inputs. The check places hpr near RASAero's
    subsonic drag with a declared rule; it can't show agreement to 10% without the inputs. M2.2
    (OpenRocket, with known inputs) is the sharper test.

**Consequences.**

- M1.6 supplies `DragConditions` (Reynolds number per metre from the airspeed and the atmosphere's
  kinematic viscosity; whether a motor is thrusting and the burning motors' case area), uses
  `axial_coefficient` along `−z_B`, and decides what to do with `beyond_subsonic_methods`.
- M1.8 adds eq. 3.87's nose and shoulder interpolation, the transonic and supersonic branches of
  every term, and extends override tables to `C_Nα` and CP.
- M2.1's same-drag mode uses `DragTable`. It must decide whether RocketPy's drag scales with angle
  of attack as hpr's does, and how to read curves with repeated Mach numbers such as Cavour's, which
  `parse_mach_csv` refuses; its flights test the power-on base relief.
- M2.2 checks the angle-of-attack polynomial, the lug diameter, the boattail reading, the friction
  area and the leading-edge sweep average against OpenRocket.
- `AeroModel::drag` takes about 50 to 110 ns (`docs/perf.md`).
