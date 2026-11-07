# NASA SP-8039: thrust, impulse, burn-time and mass definitions

Re-checked 2026-11-07: 5 claims corrected.

In short: SP-8039 defines total impulse, effective exhaust velocity (instantaneous c = F/ṁ_p) and
thrust with its pressure term as the motor page uses them. It never defines action time, average
thrust or a %-of-maximum burn-time cut; it gives web-burnout criteria (pp. 77-78) and cites CPIA
refs. 117 and 118 for them.

NASA SP-8039, *Solid Rocket Motor Performance Analysis and Prediction*, May 1971
(https://ntrs.nasa.gov/api/citations/19720011135/downloads/19720011135.pdf; cached in gitignored
`refs/papers/`). Read from page images. PDF page = printed + 8 up to p. 93; p. 94 is absent from
the scan, so glossary pp. 95-101 = PDF 102-108. US units: g_c = 32.17 lbm·ft/(lbf·s²).

## 1. Thrust and the pressure term

- Eq. (2), p. 3 (PDF 11), "stationary motor with one-dimensional steady flow":
  `F = ṁ_e u_e/g_c + (P_e − P_a) A_e` (P_e exit static, P_a ambient, A_e exit flow area).
- Eq. (3), p. 4 (PDF 12): `F = P_c A_t C_F = (ṁ_t I_spd)_motor = (ṁ_t I_spd)_propellant +
  (ṁ_t I_spd)_inerts + (ṁ_t I_spd)_igniter`. `C_F = F/(P_c A_t)`: p. 14 (PDF 22), p. 95 (PDF 102).
- Eq. (22), p. 29 (PDF 37): `C_F,ideal = √{ [2γ²/(γ−1)] [2/(γ+1)]^((γ+1)/(γ−1))
  [1 − (P_e/P_c)^((γ−1)/γ)] } + ((P_e − P_a)/P_c)(A_e/A_t)`. For a fixed pressure ratio C_F peaks
  at P_e = P_a; its largest value is in vacuum, P_a = 0 (paraphrase).
- Eq. (23): `λ = (1 + cos α)/2`. Eq. (28), p. 31 (PDF 39): `F_vac = λ ṁ_e′ u_e′/g_c + P_e′ A_e`;
  eq. (29): `F_vac = λ(ṁ_e′ u_e′/g_c + P_e′ A_e′)`.
- Eq. (65), p. 69 (PDF 77): `C_F,delivered = η_F { √{ [2γ²/(γ−1)] [2/(γ+1)]^((γ+1)/(γ−1))
  [1 − (P_e′/P_c)^((γ−1)/γ)] } + ((P_e′ − P_a)/P_c) ε_e′ }`, the bracket inside the root as in (22).
  η_F = C_F/C_F,ideal (p. 100, PDF 107): usually 0.92-0.98, 0.96 or more for a well-designed
  nozzle (p. 32, PDF 40).
- Eq. (34), p. 37 (PDF 45): `I°_sp = u_e/g_c + ((P_e − P_a)/ṁ) A_e`; eq. (35):
  `I°_sp,vac = u_e/g_c + P_e A_e/ṁ`.
- No printed sea-level-to-altitude conversion. Derived here from eq. (2), with ṁ_e, u_e, P_e fixed:
  `F(P_a2) = F(P_a1) + (P_a1 − P_a2) A_e`, so `F_vac = F_SL + P_a,SL A_e`.
- Caveat, pp. 32-34 (PDF 40-42): sea-level tests of altitude nozzles run over-expanded. Flow stays
  attached when P_a is "less than or only slightly greater than" P_e. Eq. (32):
  `P_i/P_a = (2/3)(P_a/P_c)^(1/5)`. The test prints "when P_i/P_a < P_e/P_a" (confirmed at 300 dpi);
  separation starts where the expanding flow falls to P_i, so the physical condition is P_e < P_i
  (Inference).
  Eqs. (66)-(67), p. 70 (PDF 78), give separated-flow C_F.

## 2. Impulse, specific impulse, propellant consumed

- p. 96 (PDF 103): `I = ∫_{t1}^{t2} F dt` (lbf·s). I_sp values must state P_c, P_a, ε and α.
  p. 97 (PDF 104): I_spd (measured) also needs "(5) time interval used for impulse determination
  (6) propellant mass assumption". I_sps: P_c = 1000 psia, P_a = 14.7 psia, ε optimum, α = 0°.
- p. 35 (PDF 43): I_spd is the sum of forces on the thrust chamber divided by the flowrate of the
  mass discharged (paraphrase). Eq. (33): `I_sp = c* C_F/g_c`.
  Eqs. (68)-(69), p. 72 (PDF 80): `I_spd = η_μ I°_spd`, `I_spd = η_θ c* η_F C_F/g_c`.
- p. 95 (PDF 102): c = effective exhaust velocity, "in instantaneous form, c = F/ṁ_p";
  c* "in instantaneous form, c* = (P_c A_t)/ṁ_p".
- Eq. (4), p. 5 (PDF 13): `ṁ_p = A_b r_b ρ_p`. Eq. (46), p. 57 (PDF 65):
  `P_c = c* ṁ_t/(A_t g_c) = (c*/(A_t g_c)) [ρ_p Σ_{i=1}^{N} (Δvol)_i/Δt]`.
- Fig. 25, p. 74 (PDF 82): BEM `I_spd = ∫F dt / W_p` (operator faint, reads "="); full scale
  `F = ṁ I_spd`. Same page: "Specific impulse should be based on propellant charge weight.
  Consumed weight should be determined by pre- and post-firing motor weight measurements."
- I_sp is not constant over a burn: nozzle erosion gave "a loss of 6 to 9 seconds in delivered
  specific impulse" by burnout of a 55-second-duration motor (p. 14, PDF 22; one instance, ref. 26),
  losses including lower expansion ratio and rougher surfaces. I_spd efficiency (Fig. 16, against ṁ)
  and vacuum-I_sp efficiency (Figs. 18, 19, against P_c and throat size) compare
  different motors; reading them as drift within one burn is an inference, not SP-8039's claim.
- Inerts: (I_sp)_inert 120-200 lbf·s/lbm, or half the propellant I_sp for a same-binder liner
  (p. 43, PDF 51). Eq. (71), p. 76 (PDF 84): `(I_spd)_g = (I_spd)_p (W_p/(W_p + W_L))^(−1/2)`.
- Not stated: mass consumed ∝ impulse, or ∝ ∫P_c dt. Both follow only if c (or c* and A_t) is held
  constant, via c = F/ṁ_p and eq. (46).

## 3. Burn time, action time, web burn time

- p. 77 (PDF 85): burning rate is the propellant web divided by the burning time. Web burning
  starts at the inflection point in the ignition transient of the pressure-time trace; web burnout
  is the point of maximum rate of change of curvature in the tailoff (ref. 117; paraphrase).
  p. 78 (PDF 86): if the trace is not reduced by computer or that point is not clearly
  distinguishable, burnout may be found by "the widely used tangent-bisector method" (ref. 118).
- p. 45 (PDF 53): main cause of BEM/full-scale rate disagreement is "improper definition of the
  burning time of the small motors". Web-burnout points there came from the tangent-bisector method.
- Not in SP-8039: the tangent-bisector construction, and definitions of action time, average
  thrust or average pressure. No %-of-max threshold (such as 10%) appears.
- Used without definition: p. 69 (PDF 77), heat flux "for the action time"; p. 74 (PDF 82), BEM
  neutral trace "(within ±10%)", burning time at least 0.87 of action time, and tailoff pressure
  integral under 5% of the action-time integral. Table III, p. 53 (PDF 61), coefficients of
  variation: action time 2.0%, total impulse 0.3%, avg thrust 2.1%.
- Defers to ref. 118 (CPIA Publ. 80, *Solid Propellant Nomenclature Guide*, May 1965, AD 465058)
  and ref. 117 (Jessup & VanWie, CPIA Publ. 24, Sept. 1963, AD 345567). Ref. 27 (ICRPG, CPIA
  Publ. 174, Aug. 1968) is cited for nozzle losses (pp. 17, 21, 32, 39, 42), BEM design and Figs. 4,
  5, 16, not for time definitions. p. 45 (PDF 53) says refs. 117 and 118 both recommend the
  curvature point, so SP-8039 is not fully consistent about ref. 118.
- Absence claims rest on an OCR search of all 112 pages ("action time": Table III, p. 69 and p. 74,
  each a use, not a definition; "average thrust": Table III only; "average pressure": none); OCR
  can miss words.

## 4. Mass, CG and inertia during burn (qualitative, no equations)

- pp. 34-35 (PDF 42-43): some programs output propellant mass, CG and principal MOI histories
  (ref. 3); inert discharge is predicted separately.
- §3.2.1, p. 71 (PDF 79): account for "the discharge of both propellant and inert products":
  propellant from eq. (4), inerts from heat-transfer analyses, "combined with hardware mass".
- §3.2.2, p. 71 (PDF 79): CG and MOI "shall be based on the motor station distribution of the
  discharged propellant and inert material". "Use standard mathematical techniques in calculating
  centers of gravity and in calculating and shifting principal moments of inertia."

## 5. Temperature (p. 61, PDF 69)

- Eqs. (53)-(54): `a₀ = a_ref exp(σ_p ΔT)`, `P₀ = P_ref exp(π_K ΔT)`, ΔT = firing minus reference
  temperature in °F (stated under eq. 54; eq. 53 uses the same ΔT), so a per-K coefficient is 1.8
  times SP-8039's per-°F value.
- Firing temperature is recommended as a tabular input against web burned for symmetric web
  gradients; uneven gradients are evaluated graphically and averaged over the grain length.

## What this suggests

- [physics/motor.md](../physics/motor.md) line ~456 says SP-8039 defines c "only" instantaneously
  and cites p. 14 for drift with chamber pressure; p. 14 supports nozzle erosion only. Line ~641's
  "the conversion doesn't hold" under separation is an inference from pp. 32-34.
- [propellant-temperature.md](propellant-temperature.md) could note SP-8039's ΔT is in °F and link
  the full NTRS PDF; its %/K values come from other sources, so no number moves.

## Open questions

- None of the corrections moves a number the code or a page uses. The eq. (32) sign remains a
  likely misprint in the source; nothing in the project uses eq. (32).

## Sources

- NASA SP-8039 (1971), NTRS 19720011135, read from 200-300 dpi page images 2026-11-07. A US
  government work: citable, cached under gitignored `refs/papers/`.
