# ADR-031: Roll from canted fins and roll damping by Barrowman's strip theory (2026-09-19)

- **Status:** accepted
- **Summary:** Roll from canted fins and roll damping by Barrowman's strip theory

**Context.** M1.8c's done-when (`ROADMAP.md`): M1.8's roll bullet, "roll-rate steady state
matches the analytic cant/damping balance", and hpr's roll forcing compared with the Arcas Robin's
measured roll effectiveness (TN D-4014 Fig. 14). No target was set for the comparison. Until now
the design carried a cant (`FinSet::cant_rad`, used by mass properties only) and the flight
integrated the roll rate with no aerodynamic moment about the axis. Sources, all U.S. Government
works except Niskanen's thesis: Barrowman 1967 (NASA/TM-2001-209983) §3.13–3.14, §3.33, §3.42,
appendix A, Figs. 5-6 and 5-7; Niskanen 2009 §3.3; TN D-4014 Fig. 14.

**Decision.**

- **Strip theory, Barrowman's.** One fin's forcing is its normal force at its mean aerodynamic
  chord, `C_lδ = (C_Nα)₁ (r_t + y_MAC)/d` (eq. 3-35); its damping sums strips at the incidence
  `−pξ/V` (eq. 3-40–3-49), `C_lp = −2a ∫ξ² dA/(A_ref d²)`. Faster than sound both take M1.8a's
  load, `4α/β` halved in the tip's Mach cone (appendix A, first order), and between Mach 0.8 and
  `M_s` each is a straight line in `M`, as the fin's slope is (ADR-027). The integrals come from the
  fin's polygon, so any outline works.
- **The fin's own slope in the damping,** `a = (C_Nα)₁ A_ref/A_fin`. Barrowman's text (eq. 3-40)
  and Niskanen's (eq. 3.69) write the airfoil's `C_Nα0 = 2π/β`, but Barrowman's computed curve for
  the Basic Finner, −34.21 at Mach 0.07 read from Fig. 5-7, is the fin's slope spread over the
  strips (hpr: −33.53, −2.0%); the airfoil's gives about −81, and would climb without bound toward
  Mach 1 where his curve rises about 20%. That point chose the method, so it is not a validation. Stubby fins lift far less than an airfoil section.
- **The body's interference,** Barrowman's `k_T(B)` (eq. 3-95, 3-105) on the forcing and `k_R(B)`
  (eq. 3-122, 3-123) on the damping, from slender-body theory; Niskanen leaves both out. `k_R(B)`
  is for a chord falling linearly from root to tip; another outline takes it at its tip-to-root
  chord ratio (an elliptical fin as a triangle: 5.5% too much damping at `τ` of 2 and 2.9, by
  integrating eq. 3-121 over the ellipse). Both are held constant below `τ = 1.001`, where their
  terms cancel to rounding, and past `τ = 10⁶`, where they overflow. `k_T(B)` is checked only
  against its limits and its own transcription; no independent tabulation was at hand.
- **Fin–fin interference is not applied to roll** (Niskanen eq. 3.66 uses `N`).
- **Physics review's caveats, recorded, not changed:** `k_R(B)` is a force ratio applied to a
  moment (weighted by the moment it is 3.4% to 4.2% smaller here); `k_T(B)` is reference 23's
  factor for fins turned together, not derived for cant's antisymmetric load; appendix A takes the
  supersonic strip moment about the root, where hpr takes it about the axis as eq. 3-35 does
  (about half the forcing otherwise); uniform strips likely overstate a short fin's subsonic
  damping. hpr refuses a cant beyond 15° (stall) and cant on a single fin (its side force isn't
  carried).
- **Signs.** `C_l` is about `+z_B`; a positive cant turns fin 0's leading edge toward `−y_B`
  (`hpr_design`'s convention, `docs/physics/mass.md`), so `C_l0 = −N C_lδ k_T(B) δ`.
- **In flight** the moment is `q A d (C_l0 cos α + C_lp p d/2V)` at the centre of mass's Mach
  number, the damping written `ρ V A d² C_lp p/4`. The `cos α` is a judgement (code review): the
  cant meets the air as it runs along the axis, so a rocket falling tail first spins the other
  way and one broadside isn't driven, as ADR-011 has the fins' normal force follow `sin α`. The
  terms that don't change with Mach are built once per fin set (`FinRollTerms`). Pitch and yaw damping stay the local-flow damping
  (ADR-011); `ROADMAP.md` and the planning notes had cited ADR-026 for it.
- **The references committed:** TN D-4014 Fig. 14's readings (made in M1.8a at `α` = −4°, 0°,
  +4°) into `arcas-robin-wind-tunnel.json`, and the Basic Finner's geometry and Fig. 5-7 read on a
  300-dpi render into `basic-finner-roll-damping.json` (five wind-tunnel points, ±0.3; the
  computed curve at Mach 0.07). `cargo xtask aero` writes `roll-vs-mach.json`, and
  `hpr_aero::tests::roll_against_mach` recomputes every row.

**Result.**

- *The balance* (`hpr_sim::tests::canted_fins_spin_to_the_analytic_balance`): Valetudo with 1° of
  cant at 100 m/s, with no drag and no gravity, settles on `p = −δ V A_fin (r_t + y_MAC) k_T(B) /
  (k_R(B) ∫ξ² dA)` = −16.948 rad/s within 1e-6, the test's bound (1e-11 measured), and is within
  1e-5 of the exponential approach one time constant (0.48 s) in (2e-10 measured). The closed
  form uses the trapezoid's
  integrals (Niskanen eq. 3.70), independent of the polygon code. M1.8's roll bullet is met.
- *The forcing against TN D-4014* at `α = 0`: from Mach 2.3, all 8 readings within 5.3%
  (+3.2%, +2.4%, −0.1%, +1.0%, −1.4%, −1.0%, +2.5%, −5.3%); at Mach 1.5, +47.8%; at 1.8, +14.3%
  and +17.8%. *The short model's readings were corrected* (validation audit): M1.8a had placed
  each of its six panels' zeros 14.5 to 18.5 px (0.005 to 0.007) above the grid line the zero
  lies on (the grid's bottom edge at Mach 1.50, where the "0" label sits, then every 0.2); each
  reading is now M1.8a's curve position measured from the grid line, and agrees within 0.0016
  with the audit's own re-read. Before, the comparison read 10.3% worst from Mach 2.3 and +53.6%
  at 1.5; the long model's zeros were on their lines. The correction was found by the audit, not
  sought to improve agreement, and it moved every short-model reading the same way, by 0.005 to
  0.007. Linear theory's load climbs as `1/β` toward Mach 1 and the fins' doesn't; Barrowman
  found the same on the Tomahawk ("the theoretical value at M = 1.5 is no good", p. 66).
- *The damping against the Basic Finner:* −5.9%, −7.8%, −14.3%, −15.8%, −16.2% from Mach 1.51 to
  3.00. Barrowman's curve, with Busemann's third-order terms, averages 5.68% from the same
  points (p. 66); the fins are wedges 8% thick, which first order doesn't see, part of the gap.

**Alternatives considered.**

- *The airfoil's slope in the damping,* as the texts write it: about 2.4 times the damping for the
  Basic Finner, and unbounded toward Mach 1.
- *No body interference,* as Niskanen: 7% more forcing and about 17% less damping on the Arcas
  Robin's fins; Barrowman has the factors and the forcing already reads high.
- *Busemann's higher-order terms faster than sound:* M1.8a left them out of the normal force
  (ADR-027); adding them for roll alone would make the fin's roll and its normal force disagree.
- *A target set now:* the roadmap set none, and one chosen after measuring would be no target.

**Consequences.**

- Canted fins now spin a flight; the roll rate follows the forcing and damping. Designs with zero
  cant fly as before, but for the damping of a roll rate from inertia coupling or the jet.
- Open: roll near Mach 1.5 reads high; nothing measured checks roll below Mach 1.5 (TN D-4013's
  rolling-moment plots at Mach 0.6 to 1.2, with the fins canted 2°, are the next reference); the
  roll forcing doesn't change with the angle of attack, where TN D-4014 measures up to 13%.
