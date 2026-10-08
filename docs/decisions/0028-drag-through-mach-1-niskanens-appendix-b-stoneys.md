# ADR-028: Drag through Mach 1: Niskanen's appendix B, Stoney's curves, and the Arcas Robin's axial force (2026-09-18)

- **Status:** accepted; the ellipsoid's curve below Mach 1.2 replaced by ADR-173 (a measured value below Mach 0.8, the line to Mach 1.2 drawn after scaling)
- **Summary:** Drag through Mach 1: Niskanen's appendix B, Stoney's curves, and the Arcas Robin's axial force

**Context.** M1.8b is too big for one pull request and is split into M1.8b1 (the buildup through
Mach 1, against the Arcas Robin wind tunnel, predicted Prometheus flying) and M1.8b2 (drag against
RocketPy's RASAero curves through Mach 2) (`ROADMAP.md`). Until now the buildup held nose and
shoulder pressure drag at eq. 3.86's value at rest, which ADR-009 expected to read low from about
Mach 0.6, and refused `M ≥ 1` (ADR-009); predicted Prometheus 2022 was the harness's one known
gap (ADR-021, ADR-023).
Every other term already had its supersonic branch: friction (eq. 3.83–3.84), base drag
(`0.25/M`, eq. 3.94), the fins' edges (eq. 3.89–3.93) and the stagnation pressure (eq. B.1).
Loft lesson L17: Loft froze the fin leading-edge drag at its Mach 1 value and gave nose drag no
Mach term.

What the sources give ([N09] §3.4.3, pp. 47–48, and appendix B, pp. 106–110; the OpenRocket
technical documentation 13.05 reprints them word for word):

- Eq. 3.87 carries a nose's pressure drag from eq. 3.86 at rest to "the lower bound of the
  transonic method" as `a Mᵇ + C₀`, "non-decreasing" with zero slope at rest; `a` and `b` fit the
  value and slope there. The thesis gives no closed form for them and doesn't say what to do
  where they can't fit.
- Cones have closed forms (eq. B.3–B.6, after Hoerner), from Mach 1: `sin ε` and slope
  `4/(γ + 1)(1 − sin ε/2)` at Mach 1, eq. B.4 from "M ≳ 1.3", and "polynomial interpolation"
  between. Ogives are the cone times `0.72(κ − ½)² + 0.82` (eq. B.8, after NAVWEPS 1488 p. 239).
- Elliptical, power, parabolic and Haack noses take Stoney's measured curves at fineness 3 (NASA
  TR R-100, 1961, Figure 12), "written into the software as data curve points", scaled to other
  fineness ratios by eq. B.9 through a flat face at fineness 0. The thesis prints no values; Stoney
  prints only the plots.
- Shoulders are treated "similar to nose cones", "somewhat dubious at supersonic velocities"
  (p. 48), without saying what their fineness is.

The measured reference is NASA's half-scale Arcas Robin (TN D-4013, Mach 0.6–1.2; TN D-4014,
Mach 1.5–4.63), already the normal force's (ADR-027). Both reports take the base apart, because
the models sat on a sting: TN D-4013 plots `C_A,corr`, "corrected for base axial force" to the
free stream's pressure over the full base (its Fig. 3 base pressure over Fig. 11's `C_A,b` gives
0.43 of the reference area, the 1.470-in base's 0.427); TN D-4014 plots `C_A` with the balance
chamber's force in it and `C_A,c` apart, uncorrected (printed p. 4), and states no chamber area.
[N09] Fig. 6.6 compares OpenRocket with the same data and finds its drag about 80% high by Mach
3.96, blaming the boattail, the airfoil fins and "less reliable results" at higher speeds.

**Decision.**

- **Every nose, shoulder and step: eq. 3.86 at rest, eq. 3.87 to `M_L`, appendix B from `M_L`**
  (`hpr_aero::nose_drag::PressureDragCurve`, precomputed per component when the model is built).
  `b = C_T′(M_L) M_L/Δ` and `a = Δ/M_Lᵇ` with `Δ = C_T(M_L) − C₀`, evaluated as `Δ (M/M_L)ᵇ`,
  which stays within `[0, Δ]`: in review, `a` overflowed where `Δ` is tiny and `b` huge (an x^0.868
  nose at 3:1 gave NaN below Mach 0.8), and a regression test holds it. The power is used only
  where it meets Niskanen's conditions, a rise with `b > 1` (flat at rest); see the fallback below.
- **Cones and ogives** as printed from fineness 1, with a cubic Hermite between Mach 1 and 1.3 and
  `M_L` = 1 (the thesis's "interpolated using equation (3.86)" between 0 and 1 in B.2 can only mean
  eq. 3.87). The ogive's `κ` is the reciprocal of hpr's radius ratio. **A bulged secant ogive**
  (radius ratio below 1, `κ > 1`) is outside eq. B.8 and **refused by the buildup** when it is
  evaluated: the model still builds (after review), so the normal force and a drag table work.
- **Below fineness 1, cones and ogives scale by eq. B.9's form** between a flat face at fineness
  0 and their own whole curve at fineness 1, `C₀ (C₁/C₀)^(ln(f + 1)/ln 2)`, at every Mach number.
  Eq.
  B.4 runs past a flat face's drag as a cone flattens (2.39 at Mach 2 as `f → 0`, against 1.41),
  and with it a shrinking shoulder no longer tended to a step (Loft lesson L15's test failed:
  0.801 against 0.842 at Mach 0.3). Continuous at fineness 1 and at 0 (the step), so L15 holds
  exactly.
- **Stoney's curves, digitized and committed as data** (`StoneyNose`, in the source: core crates
  do no I/O). Read from a 600-dpi render of Figure 12 (printed p. 16), each panel's grid fitted
  line by line (skew up to 6 px in panel (b)), at the line's centre, checked on overlays: about
  ±0.0015, up to ±0.005 on the steepest rises. Panel (a), the flight models, for the seven shapes
  it has (x^½, x^¾, the ½, ¾ and full parabolas, L-V Haack, von Kármán), from Mach 0.8 to each
  line's end at 1.94–1.99; panel (b), the wind tunnel of Stoney's ref. 30, for the x^¼ and the
  ellipsoid, which begin at Mach 1.2, to 3.59. Past the last point the value is held; panel (b)
  puts the hold within 8% for von Kármán (to Mach 3.59) and x^¾ (to 3.2), and panel (a)'s x^½ is
  still rising at its end. After review, the x^¼ and the ellipsoid are joined by a straight line
  to 0 at Mach 0.8, where every smooth 3:1 nose of panel (a) reads 0: before, their `M_L` of 1.2
  stretched eq. 3.87 to 1.2, which made a power series' drag jump at `n = ½` (0.0511 against 0 at
  Mach 0.9) and gave elliptical noses subsonic drag the data doesn't show. Stoney's
  configuration key (Fig. 9) numbers the parabolas 59 full, 62 three-quarter, 57 half. Where (a)
  and (b) overlap, (b)'s von Kármán reads 0.004–0.011 higher from Mach 1.2; (a) is used because it
  is Stoney's own data and covers the shapes together. A NASA report is a U.S. Government work.
- **Between measured shapes, linear in the parameter at fineness 3, then eq. B.9**: a power series
  through the flat face (`n = 0`), x^¼, x^½, x^¾ and the 3:1 cone (`n = 1`); a parabolic series
  through the 3:1 cone (`K′ = 0`) and the three parabolas; a Haack series between von Kármán and
  L-V Haack. **A Haack series past `C = ⅓` is refused** by the buildup in the same way, as [N09]
  limits it (p. 103). `M_L` is 0.8, [N09]'s start of the semi-empirical method (p. 47), for every
  measured shape. The ends of the power and parabolic series are B.9's 3:1 cone, not the cone of
  the nose's own fineness, so a power series of exponent 1 and a cone of the same fineness agree
  only at fineness 3 (review, advisory; the structure is [N09]'s).
- **Where eq. 3.87 can't fit** (`Δ ≤ 0`, or `b ≤ 1`), `C₀ + Δ (M/M_L)²`: continuous, flat at
  rest, a kink at `M_L`. Falling, where a joint that isn't smooth meets a measured curve still at 0
  at Mach 0.8, it follows Stoney's measurement rather than [N09]'s "non-decreasing" (review,
  advisory; coefficients below 0.01). Rising with `b ≤ 1`, it serves near-flat noses, which eq.
  3.86 gives almost nothing at rest: x^0.05 at 3:1 rises to 0.80 at Mach 0.8, flat at rest where
  the power rose to 0.29 by Mach 0.1 (review).
- **A shoulder's fineness is `l/(d_aft − d_fore)`**, a nose's `l/d` when `d_fore = 0`: the cone of
  the same surface angle. A clipped transition takes its shape as if unclipped. A widening too
  small to change the diameter as computed is no shoulder (review). **A step** (and a body's bare
  front face) is a flat face, the blunt cylinder's `0.85 q_stag/q` at every Mach number (eq. B.2),
  0.85 at rest: eq. 3.86 "does not take into account the effect of extremely blunt nose cones
  (length less than half of the diameter)" (p. 47), and a step has no length (review). Before, it
  was 0.8 at every speed.
- **The buildup covers `0 ≤ M < 5`** (`BUILDUP_MACH_LIMIT`), like the normal force.
  `Drag::beyond_subsonic_methods` is removed: nothing read it, and the flag no longer marked an
  error of hpr's own.
- **The known gap** (ADR-021) is now a refusal of a Mach number at or past the normal force's or
  the drag buildup's top, both Mach 5, which the reference must reach too; no case declares one.
  The logic moved into `run::settle`, which checks the reference against the refusing model's own
  limit, so its paths stay tested without a flight that reaches Mach 5.
- **The Arcas Robin comparison: forebody drag, hpr's `C_D0` less its base drag, against `C_A,corr`
  (TN D-4013) and `C_A − 1.383 C_A,c` (TN D-4014)**, fins at 0° and off, at every Mach number the
  reports give, at their 3.0 × 10⁶ per foot. 1.383 = (1.470/1.250)², the base over the chamber's
  1.250-in cavity in TN D-4014 Fig. 1(a), so the chamber's pressure acts over the whole base as TN
  D-4013's correction assumes; `C_A − C_A,c` is committed beside it, 0.002 to 0.015 higher, and
  changes no row's verdict; a test checks both against `C_A` and `C_A,c`.
  Digitized as the normal force was (ADR-027), each plot's grid mapped for skew and shear; TN
  D-4014's `C_A` at zero angle is a quadratic through the symbols within 2.6°. Checking TN
  D-4013's earlier readings found Fig. 11's scan sheared (up to −0.0045 at Mach 1.2) and its
  base-force labels swapped at Mach 0.6; both are corrected in the committed readings. For drag,
  the committed designs take hpr's airfoil section for the double-wedge fins (Niskanen's choice,
  p. 90; square edges give about 3.7 times the fin drag at Mach 0.6 and 1.45 times at Mach 1.5)
  and a polished finish, 0.5 µm, for machined steel (the reports state none; at 20 µm the friction
  would be about 26% higher). Both choices moved hpr toward the tunnel, and they were made in the
  same commit as the measurement, so the audit recomputed the count for each: 3 of 44 within 10%
  with square edges and 20 µm, 5 with the airfoil section alone, 6 with the finish alone, 8 with
  both (`drag_against_mach_depends_on_the_fins_and_finish`). The airfoil section follows the
  drawings; the finish is a guess. Target, set before measuring: M1.8's 10%. `cargo xtask aero` writes `validation/fixtures/aero/drag-vs-mach.json`
  with the drag by part; `hpr_aero::tests::drag_against_mach` recomputes it and pins the rows
  within target.

**Result.**

- L17's test and the model's own tests pass: a 3:1 cone by hand (0.0216 at rest, 0.1644 at
  Mach 1, 0.1042 at Mach 2), the joins smooth, eq. B.9 through its anchors, a shoulder tending to
  the step, Stoney's curves reproduced, and a property test over every shape to Mach 5.
- **Against the Arcas Robin, 8 of 44 rows within 10%** (fixture), 3 of them within their reading
  uncertainty and the reports' ±0.004 of the 10% edge, so 5 to 8. Fins off: +28% to +49% from Mach
  0.6 to 0.9, −9.2% to +18.4% from 0.95 to 1.8, +20.5% to +71.1% from 2.3. Fins on: +31% to +47%
  subsonic, −10.9% to +11.3% from 0.95 to 1.2, +30% to +191% from 1.5. The drag by part explains
  it:
  - *The fins past Mach 1.2*: the rounded leading-edge formula (eq. 3.89) holds hpr's fin drag near
    0.30 from Mach 1.5, where the measured fins-on less fins-off falls from 0.153 to 0.046 (+78% to
    +551%). A thin, sharp fin's wave drag is far smaller and falls with Mach; nothing in hpr models
    it. This is Niskanen's own Fig. 6.6 miss.
  - *The model's 1.3-mm reflexed lip*, taken as a shoulder in the free stream (fineness 0.33),
    worth 0.065 to 0.086. It sits in the boattail's wake. Without it, fins off, the short model's
    forebody is within −28% to +14% at every Mach number (−1.2% at 2.96) and the long model's
    within −15% to +23%. The short model also keeps raised fin-root fairings with its fins off,
    which hpr leaves out and TN D-4013 (pp. 4–5) blames for its higher drag from Mach 0.975 to 1.2.
  - *The boattail rule* (eq. 3.88) gives the 15° boattail 0.063 at Mach 0.6, where the measured
    fins-off forebody, 0.22, leaves little above hpr's friction of 0.19: the rule over-predicts
    this boattail, as [N09] found against the same tunnel (p. 90). It is most of the subsonic
    excess, with the lip. (A first draft called it bookkeeping; the audit and the physics review
    showed the tunnel's forebody holds exactly the pressure the rule models.)
- Niskanen's 3:1 cone against Stoney's measured one, high through the whole rise: +87% at Mach
  0.8, +105% at 0.85, +49% at 1.0, +48% at 1.1 (0.234 against 0.158), +15% at 1.5 and +4% at 1.94.
  Ogives inherit it. So stubby cones and ogives gain the most at high subsonic speeds. The whole
  rocket's `C_D0` at sea level against the buildup before M1.8b1 (measured in review; "rest" held
  the nose at eq. 3.86):

  | rocket (nose) | Mach 0.6 | 0.8 | 0.9 | 0.95 |
  |---|---|---|---|---|
  | Bella Lui (1.55:1 tangent ogive) | +6.5% | +22.7% | +37.6% | +47% |
  | NDRT 2020 | +0.4% | +5.6% | +16.0% | +26% |
  | Valetudo | +0.1% | +2.3% | +7.7% | +13% |
  | Calisto, Prometheus (von Kármán) | 0 | 0 | 0 | +0.3% to +0.6% |

  An earlier draft said the held value "read low" by what eq. 3.87 now adds; that measured the old
  model against the new one, and Stoney's data say the opposite for cones (review).
- **Predicted Prometheus 2022 flies**, to Mach 1.059 against RocketPy's 1.048: 17 metrics, 9
  within target; apogee −6.985%. hpr's coasting `C_D0` rises to about 0.49 at Mach 0.8 and 0.54 at
  Mach 1, mostly base drag, where the example's falls to 0.30; under power hpr relieves the base by
  the motor's area. The misses are explained in the case file and pinned.
- The predicted flights barely move, as none spends long above Mach 0.6 on a stubby nose: Bella Lui's
  `C_D0` at Mach 0.3 from 0.423 to 0.424 and its predicted apogee from +1.018% to +1.004%; NDRT
  2020's from +10.322% to +10.306%; of the Mach 0.3 comparison with RocketPy's curves (ADR-009),
  only Valetudo's `C_D0`, by 4e-7.

**Consequences.**

- Supersonic drag with fins reads high, the more so the thinner and sharper the fins, until a fin
  wave-drag model replaces the blunt leading edge for sharp sections. M1.8b2 measures it against
  RASAero II through Mach 2 and decides. It should also weigh Stoney's measured 3:1 cone (digitized
  with the other curves) against Niskanen's closed form for cones and ogives through Mach 1.2.
- The buildup has no transonic or supersonic check of its base drag (the tunnel's base is the
  sting's), and shoulders past Mach 1 rest on [N09]'s own doubt.
- ADR-009's held nose drag and `M ≥ 1` refusal, and ADR-021's `M ≥ 1` gap, are superseded here.
