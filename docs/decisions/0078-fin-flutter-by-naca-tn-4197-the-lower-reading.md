# ADR-078: Fin flutter by NACA TN 4197: the lower reading wherever the source leaves room (2026-09-26)

- **Status:** accepted
- **Summary:** Fin flutter by NACA TN 4197: the lower reading wherever the source leaves room

**Context.** M1.10b asks for the flutter speed and margin from a cited primary source, matching its
worked example, with each shear modulus cited (L32: Loft's constant was half the source's, and 7
of its moduli had no source). The source is D. J. Martin, NACA TN 4197 (1958), eq. 18, which the
hobby community's flutter formulas descend from. Martin's appendix derives it from
Theodorsen and Garrick's flutter speed; his worked examples (pp. 6–7) read his figure 4.

**Decision.**

1. **Martin's eq. 18 as printed, constant from eq. 16.** `(V_f/a)² = G_E / D` with
   `D = (24εγ/π) p · A³/((t/c)³(A + 2)) · (λ + 1)/2`, `ε = 0.25`, `γ = 1.4`. The constant is
   computed (`24 · 0.25 · 1.4/π · 14.696 psi = 39.29 psi`), not his rounded 39.3.
   `hpr_sim::flutter::FlutterPanel` holds `A`, `λ` and `t/c`; `λ` must lie in `[0, 1]`, where
   his taper factors are defined.
2. **A flutter dynamic pressure.** Eq. 16 depends on the air only through `ρa²`, so it fixes
   `q_f = π G_E / (24 ε X (λ + 1))` at every height, and `V_f/V = √(q_f/q)`. The margin of a
   flight is that ratio at its max q, which M1.10a's watcher already finds on the dense output; no
   new observer is needed. A booster's fins get the whole flight's max q, which can only
   understate their margin.
3. **The worked example is matched at Martin's printed resolution.** His examples are chart
   readings: `X` "about 1.25 × 10⁶" psi (eq. 19: 1.228, which rounds to 1.25 at his 0.05 steps),
   and titanium thicknesses 2.5, 4.5 and "about 6.5" percent (eq. 19: 2.54, 4.61, 6.43, each
   those at his half-percent steps). The test asserts that rounding, not a percentage tolerance.
   His verdicts on the first wing (magnesium in the flutter region, aluminium marginal, steel
   probably safe) and titanium's margin are the example's margin half: with the moduli he marks
   on figure 3's axis, each box measured on the scan, magnesium's figure 3 ratio lies wholly above
   his band, aluminium's overlaps it, steel's and titanium's lie below.
4. **Martin's line is a measured band, and `V_f/V = 1` is not it.** Figure 3's shaded band,
   measured on a 250 dpi scan (both log axes calibrated on their ticks, 69 columns traced from
   `G_E` = 0.05 to 10 × 10⁶ psi; the axis runs to 20), runs at `D/G_E` = 0.25 to 0.31 all along
   that range: `V_f` of 1.8 to 2.0 times the speed of sound,
   for wings that flew to at least Mach 1.3. `FIGURE_3_BAND` holds it and
   `FlutterPanel::figure_3_ratio` gives `D/G_E`, the reading that decides. The hobby convention of
   `V_f/V = 1` as the limit isn't calibrated by the source, and no fixed `V_f/V` is: at max q,
   `D/G_E = 1/(M · V_f/V)²`, so the band is at `1.8/M` to `2.0/M`, and a fin is below it only
   above `2.0/M`. (A first draft treated 1 as the line, and a second 1.5; review caught both.)
5. **The lower flutter speed wherever the source leaves room.** The thickness ratio is taken at
   the root, the smallest on a constant-thickness fin. A solid fin's `G_E` is its material's `G`,
   as Martin's text says (p. 6), though his eq. 12 with a flat plate's `J = ct³/3` would give
   twice that (a `√2` higher speed). His `(λ + 1)/2` replaces `1/(f₁² f₂²)`, which it exceeds by
   up to 47% at `λ = 0.31` (17.5% lower speed); it is kept, since his figure 3 was drawn with it.
   One reading goes the other way and is stated: an airfoiled fin's `G_E` by eq. 12 is `0.946 G`,
   so its `V_f` is up to 2.7% high.
6. **Shear moduli with their own sources, beside the densities.** `Material` is unchanged (a
   design stores a density only); `hpr_design::materials::SHEAR_MODULI` gives 14 built-in
   materials an in-plane shear modulus with source, page, URL and basis: metals from MIL-HDBK-5J,
   carbon from NCAMP's AS4/8552 `G₁₂`, plywood from Riga Wood's panel shear, woods from the Wood
   Handbook's `G_LT/E_L` times 1.10 × the bending modulus (its footnote), and nylon and acetal as
   `E/(2(1 + ν))` from their data sheets. Where a source gives a range, the lower is kept. No
   source found gives G10/FR-4's, or PLA's, ABS's, PETG's, polycarbonate's or acrylic's; they have
   none, and the caller passes one.
7. **Refused, not guessed.** Elliptical, freeform and reverse-tapered fins return
   `SimError::Unsupported` (a new variant): Martin's taper factors are for trapezoids tapering
   outward. A panel's numbers are checked when it is made and when it is read from JSON.

**Consequences.** hpr's flutter numbers are a screening check, not a flutter analysis, and are
not shown to be conservative as a whole: the fin's mounting, sweep and Mach effects are left out,
and nothing is checked against a hobby rocket. On the synthetic 54 mm rocket on an I175, 3.2 mm
birch plywood fins reach 1.75 times `V_f` at max q; carbon fibre has `V_f/V` of 1.46 but a
figure 3 ratio of 0.45, above the band; aluminium passes both (3.40 and 0.083). A later milestone could add a plate-theory or measured-stiffness
option; this one doesn't.
