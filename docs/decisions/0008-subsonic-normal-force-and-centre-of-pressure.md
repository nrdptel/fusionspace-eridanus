# ADR-008: Subsonic normal force and centre of pressure (2026-09-17)

- **Status:** accepted
- **Summary:** Subsonic normal force and centre of pressure

**Context.** M1.5a adds Barrowman's normal-force slope and centre of pressure. The sources
disagree in places. Barrowman's 1966 report fits ogive noses with 0.466 L, and his 1967 thesis
adds slender-body interference terms that the report leaves out. Niskanen (2009) keeps
`sin α/α` and adds body lift. The OpenRocket technical documentation (13.05) replaces the thesis's
roll-dependent three- and four-fin terms with plain `N/2`. Barrowman's own TIR-33 (1970) treats
six fins with its own interference factor. Loft reused the conical CP for every transition (L9),
had no fin-count correction (L8), and swapped elliptical fins for an equal-area trapezoid (L10).

**Decision.**

- **Bodies.** `(C_Nα)_B = (2/A_ref)ΔA` and `X_B = [l A(l) − V]/ΔA`, with `V` integrated from the
  real profile (no 0.466 L fit). Moments are summed as `(2/A_ref)[l A(l) − V]`, which stays well
  conditioned when `ΔA → 0`. The potential term carries `sin α/α` (Niskanen eq. 3.19). No Mach
  term.
- **Body lift.** Galejs's `K (A_plan/A_ref) sin² α` with `K = 1.1` at the planform centroid
  (Niskanen eq. 3.26–3.27), on every body component.
- **Fins.** Diederich's slope with `β = √(1 − M²)` (Barrowman 1967 eq. 3-6), the mean aerodynamic
  chord integrals with the CP at its quarter chord for all subsonic Mach (Barrowman 1967 p. 6).
  Niskanen's aft CP shift above Mach 0.5 (eq. 3.35–3.36) belongs with the supersonic fit it
  interpolates to and moves to M1.8. Freeform fins follow Niskanen pp. 27–29: the filled chord
  for the CP, the true area for the slope, the span-averaged mid-chord angle.
- **Fin count and roll.** `Σ sin² Λ_k` (exactly `N/2` for three or more fins) times 1, 0.948,
  0.913, 0.854 or 0.810 for up to 4, 5, 6, 7 or 8 fins (technical documentation eq. 3.54, from
  MIL-HDBK-762(MI) p. 5-24). More than eight fins are refused: the documentation's 0.750 has no
  source. One- and two-fin sets also report a side force across the flow's plane,
  `Σ sin Λ_k cos Λ_k`, derived from Niskanen's per-fin angle `α sin Λ_k` (eq. 3.50). Niskanen drops
  it, arguing that it cancels for two or more fins, but for two fins it adds. Fin sets at the same
  station are not combined.
- **Interference.** `K_T(B) = 1 + r_t/(s + r_t)` (Barrowman 1966 eq. 77, a fit for
  `r_t/(s + r_t) < 0.4`), not the thesis's full slender-body terms; the fin-induced body lift
  `K_B(T)` is neglected, as in the report.
- **Scope.** `0 ≤ M < 1`; `M ≥ 1` is an error until M1.8. The sources document the subsonic
  models only to Mach 0.8, and Niskanen's fin CP shift would already be 0.05 MAC aft there, so
  0.8–1 is an unvalidated extrapolation, accepted so M1.6 can fly through it until M1.8. Tube fins
  and unknown part kinds are refused. Lugs and rail buttons add no normal force. Cant is ignored
  until roll (M1.8). The model is a small-angle model; `α` is accepted over `[0, π]` so the flight
  engine can decide what to do near apogee. A slope that cancels to below 1e-12 of its terms has no
  CP; `moment_m` is always defined.
- **Six fins: MIL-HDBK-762 over TIR-33.** TIR-33 handles six fins with `K = 1 + 0.5 R/(S + R)` and
  no fin-count factor, without a derivation and for six fins only. MIL-HDBK-762 gives six and eight
  fins from slender-body theory, the technical documentation interpolates five and seven, and
  OpenRocket (the M2.2 oracle) uses the same factors, so a comparison there isolates other
  differences. With `x = r/(s + r)`, hpr's six-fin slope over TIR-33's is
  `0.913 (1 + x)/(1 + 0.5 x)`: −8.7% as `x → 0`, −4.3% at 0.1, +3.2% at 0.3 (the Recruiter) and
  +6.5% at 0.4, the edge of the interference fit's range.
- **Radius steps (an extrapolation).** Where one body component's aft radius differs from the next
  one's fore radius, the step adds `(2/A_ref)ΔA` at the joint (a zero-length transition), so the
  body's total slope is Barrowman 1966 eq. 10 over the whole body. Barrowman 1967 p. 18 assumes no
  discontinuities, so this goes beyond the source; dropping the step would lose its slope silently.
  It is reported as part of the aft component. A body that starts blunt (no nose cone) gets no term
  for its front face, as eq. 10 gives.
- **Validation, and the gap it leaves.** Barrowman's five published worked examples (NARAM-8's
  Testbed II and Aerobee 350; TIR-33's Javelin, Recruiter and Arcon-Hi), every printed component
  and total. With hpr's own model, four examples agree within 1% (worst −0.77%), and the
  Recruiter's six-fin slopes do not: +3.42% (fins) and +2.87% (total). That gap is this ADR's
  six-fin choice, measured: the rules differ by +3.22% and +2.83%, and with TIR-33's rule
  substituted in the slope and the CP weighting the Recruiter agrees within 1% (worst +0.19%). The
  test pins that exactly those two values fall outside 1% with hpr's model, and prints both.

**Consequences.**

- M1.6 builds an `AeroModel` once per design and calls `normal_force` (about 10 ns) per
  derivative evaluation, using `moment_m` for the moment (defined even with no net force). It
  decides how to treat large angles of attack.
- Unknown part kinds are refused, so a new `Part` variant needs an aerodynamic decision.
- M1.8 adds the transonic and supersonic slopes, the fin CP shift, roll forcing and damping.
- M2.2 compares CNα and CP against OpenRocket, which uses the same fin-count factors.
