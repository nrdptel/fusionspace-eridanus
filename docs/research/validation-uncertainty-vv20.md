# Validation uncertainty after ASME V&V 20

## In short

ASME V&V 20-2009 is paywalled, and this note did not read it. Its method is read from open texts: mainly a 2020
paper by Eça, Dowding and Roache that clarifies the standard's validation procedure [S1], with the standard's
contents pages [S2], two 2008 workshop documents [S3, S4], and three summaries of its parts [S5 to S7].

The combination that [M1.14b7, a validation uncertainty per real flight](../decisions-and-roadmap.md#m1-14b7)
plans is the standard's own: `E = S − D`, and `u_val` is the root sum of squares of `u_num`, `u_input`
and `u_D` when the three errors are independent [S1 p. 4, eq. 10]. Two of the plan's choices need care, and
both err on the flattering side:

- **A bare halving difference is not V&V 20's `u_num`.** The standard works through Roache's grid
  convergence index (GCI). On two runs, the GCI scales the difference by `3/(2^p − 1)` [S7; S4 pp. 3, 5]. For
  HPR Sim's fourth- and fifth-order integrators, the bare difference is 5 to 10 times that GCI, the finer run's.
- **A count of flights with `|E| ≤ 2 u_val` is not V&V 20's outcome.** The paper says that meeting this
  inequality is not the goal, since larger uncertainties can always meet it [S1 p. 5, p. 11].

How sure: the definitions and the reading of the interval come from [S1] and [S3] and agree. The conversion of
a GCI to a standard uncertainty is known only through the summaries [S5, S6]: 2, or 1.1 to 1.15 [S5 p. 17].

## What was read

- [S1] was read whole, all 11 pages. The standard's sections it cites (1-5, 3, 6-3, C-6) are known only
  through it.
- [S2] is ASME's four-page preview: cover and contents, pp. iii to v. It names §3-2's local (sensitivity) and
  §3-3's global (sampling) input methods, §5's four cases of how `D` is found, and §6's two readings of `E`.
- Not read: the standard's text, including §2-4's GCI procedure, its conversion to a standard uncertainty, and
  §5-3 to §5-4's equations for shared error sources.

## The definitions

The paper's notation follows the standard's [S1 pp. 2 to 3]. `S` is the simulated value, `D` the
experimental value and `T` the unknown true value of one scalar validation variable at one set point.

```text
δ_S = S − T          δ_D = D − T                         (S1 eqs. 1, 2)
E   = S − D = δ_S − δ_D                                  (eqs. 3, 4)
δ_S = δ_model + δ_num + δ_input                          (eq. 5)
δ_model = E − (δ_num + δ_input − δ_D)                    (eq. 6)
u_val² = V[δ_num + δ_input − δ_D] = u_num² + u_input² + u_D²   (eq. 10, independent errors)
E − u_val ≤ δ_model ≤ E + u_val                          (eq. 11)
```

- `δ_model` is the error of the continuum model's assumptions; `δ_num` comes from solving it numerically;
  `δ_input` is the error in `S` from inexact inputs [S1 p. 3].
- Known signed biases are taken out of both `S` and `D` first. The errors left are fixed but unknown for the
  run made; each `u` is a standard uncertainty, an estimate of one standard deviation [S1 p. 3; S3 p. 3].
- So the estimate of `δ_model` is `E` itself, and `u_val` is that estimate's standard uncertainty
  [S1 p. 3, eq. 8; S3 p. 3].
- The paper notes a misprint in the standard's eq. (1-5-4): its third `=` should be a `−` [S1 p. 3, note 2].

**Worked example (invented numbers).** A flight is simulated to 1,000 m and its log reads 950 m, so
`E` = +50 m. Take `u_num` = 1 m, `u_input` = 20 m and `u_D` = 10 m. Then `u_val` = √(1 + 400 + 100) =
22.4 m, and `2 u_val` = 44.8 m. `|E|` is above `2 u_val`, so the model error is read as positive.

## When the parts add in quadrature

Eq. 10 holds only when the numerical, input and experimental errors are independent [S1 p. 3]. The standard
treats dependent errors in detail, and the plain sum is its simplest case [S1 p. 3; S3 p. 2, note 1].

- **Shared sources.** When the same uncertain quantity enters both `S` and `D`, eq. 10 should not be used
  [S1 p. 7]. The standard's propagation then has five kinds of terms: parameters in both `S` and `D`,
  parameters in `S` only, parameters in `D` only, the numerical part, and correlated bias errors in both.
- **Which way it errs.** In the case the workshop names, the effects nearly cancel in `E`, yet eq. 10 counts
  them twice [S3 p. 4]. The paper calls the plain sum the more conservative choice there [S1 p. 7].
- **How much, in their example.** In the standard's fin-tube example, the full and plain-sum values differ by
  0.05% of `D`. The input terms that touch only the simulation dominate there [S1 p. 7].
- **The input part is never dropped.** An uncertain input held fixed moves its error into `δ_model`; one
  propagated moves it into `δ_input` [S1 p. 4]. With every input fixed, `u_input` = 0: the "strong model".

**Inference.** HPR Sim's input and data parts share sources. A barometric altimeter turns pressure into height
through an atmosphere model. That matches the standard's Case 4, whose data-reduction equation is itself a
model [S2 p. iii]. The site's pressure and temperature then enter both `S` and `D`, so eq. 10 does not hold as
it stands. If an error moves `S` and `D` the same way, eq. 10 overstates `u_val`; if opposite ways, it
understates it. Its size isn't known until both are computed from one draw.

## The input part, `u_input`

The standard gives two ways [S2 p. iii; S3 p. 4]:

- **Local:** sensitivity coefficients. For uncorrelated inputs `X_i`, `u_input² = Σ (∂S/∂X_i · u_Xi)²`, with
  the derivatives found by finite differences [S5 p. 18, eq. 19, citing Coleman and Steele 2009]. It fails
  where `S` is strongly nonlinear over the inputs' spread [S5 p. 19].
- **Global:** sampling the inputs' distributions, by Monte Carlo or Latin hypercube sampling, and taking the
  spread of `S` [S5 p. 19]. The standard's worked example does both and tabulates the comparison
  (its Table 7-3-16) [S2 p. v].

A flight's dispersion over its stated input uncertainties is the global method. Its sample standard deviation
of apogee is `u_input` (inference).

## The numerical part, `u_num`

The standard's solution verification (§2-4) implements Roache's GCI in five steps [S6 pp. 3 to 4]. Three
meshes, each refined by a factor above 1.3, give the observed order `p`; then the extrapolated value and the GCI
are reported. §2-5 suggests three to six levels when convergence is noisy [S6 p. 4].

```text
fine-run error estimate   E_1 = ε/(r^p − 1),   ε = f_2 − f_1     (S4 p. 3)
GCI                       = F_s · |ε|/(r^p − 1)                  (S6 p. 7, eq. 11)
F_s = 1.25 over three or more runs,  3.0 for two runs            (S7; S6 p. 7)
```

- The GCI is a 95% uncertainty, not a standard one. The standard converts it to one standard deviation
  [S6 p. 7]. NIST's summary puts the divisor at 2 for a Gaussian about the fine solution, or 1.1 to 1.15 for
  a shifted one [S5 p. 17]. Neither number was checked against the standard. Refinement may be in time [S7].
- A bare difference is an error estimate, not an uncertainty. The 2008 workshop asks any error estimate to be
  multiplied by a factor of safety, as the GCI does [S3 p. 1]. Roache argues against `ε` alone because it is
  not an ordered estimator [S4 p. 5].
- Roache's `F_s` = 3 came from experts who trusted `ε` from a grid doubling with a second-order method
  [S4 p. 5]. There `3/(2^2 − 1)` = 1, so the bare difference is the two-run GCI at `r` = 2, `p` = 2.

**What halving gives at other orders.** With `r` = 2 and `F_s` = 3, the two-run GCI is `3|ε|/(2^p − 1)`
(inference from [S4, S7]):

| Order `p` | GCI over the bare difference `|ε|` | Bare difference over the GCI |
| --------: | ---------------------------------: | ---------------------------: |
|         1 |                               3.00 |                         0.33 |
|         2 |                               1.00 |                         1.00 |
|         4 |                               0.20 |                         5.00 |
|         5 |                              0.097 |                        10.33 |

HPR Sim's fixed-step RK4 is fourth order. Its Dormand-Prince stepping is fifth order, and its dense output,
which places apogee, is fourth order ([Time integration](../physics/integration.md)). So `|ε|` used as `u_num`
overstates the finer run's GCI 5 to 10 times; divided by 1.15 to 2, 5.75 to 21 times. For the coarser run, the
bare difference is the smaller, the flattering side. The size matters only when `u_num` is
not small beside the other two parts.

**Inference.** The adaptive Dormand-Prince stepper has no fixed `h` to halve. A tighter tolerance is not a
refinement with a known `r` and `p`. A third run at `h/4` (or a third tolerance) gives an observed order, and
with it the three-run `F_s` = 1.25.

## The interval and how to read it

- **The coverage factor.** `U = k·u` gives an expanded uncertainty [S1 p. 4, eq. 12; standard §6-3]. `k` needs
  an assumed error distribution. For the uniform, triangular and Gaussian distributions the standard
  considers, `k` is typically 2 to 3. The method is not limited to Gaussian errors [S1 p. 4].
- **`k` = 2 is named.** The standard's example plots `E ± 2 u_val` for Gaussian errors and 95% probability
  (its Figures 7-3-7 and 7-3-8) [S2 p. iv]. The paper uses `U_95 = 2 u_val` on that example [S1 p. 7].
- **`|E| ≫ U_95`:** the model error is about `E`, sign and size [S1 p. 5].
- **`|E| ≪ U_95`:** only `|δ_model| ≤ U_95` is known, and its sign is not [S1 p. 5]. The paper calls this the
  outcome of a poor-quality exercise that is easily misread [S1 p. 5].
- **In between:** `|δ_model| < |E| + U_95`; when `|E| ≥ U_95`, `δ_model` has the sign of `E` [S1 p. 5].
- **A computable rule.** For `|E| > C·U`, `δ_model = E`; for `C·U ≥ |E| ≥ U`, the sign is known and the size
  bounded; below `U`, the sign is unknown. `C` is typically 10, and 7 is proposed elsewhere [S1 pp. 5 to 6].
- **`u_val` measures the exercise, not the model.** It depends only on the numerical, input and data
  uncertainties [S1 p. 5]. Validation is an assessment of `δ_model`, not a pass or fail [S1 p. 1].

The earlier ITTC (2002) rule called a model validated when `|E| < U_val`. The paper says this cannot be the
goal of the exercise [S1 p. 5]. NIST's summary carries the same two-case reading [S5 p. 15].

## What the project can use

- **Cite [S1] for the combination.** It states the equations, the independence assumption and the reading of
  the interval. It is a conference paper (ASME copyright), so it is cited, not copied. [S2] confirms the
  standard's structure and its `k` = 2 example.
- **Count by the sign rule, not by inclusion alone.** Beside the flights inside `2 u_val`, report those
  outside it and how many of them have `E` > 0. A count inside the band alone rewards a large `u_val`.
  Expected effect: the report says how many flights show a model error of known sign, which V&V 20 is for.
- **Take `u_num` as a GCI.** Use `F_s · |ε|/(2^p − 1)` with `p` = 4 and `F_s` = 3 on two runs, or the observed
  `p` and `F_s` = 1.25 on three, for the finer run; for the coarser, multiply by `2^p`. Then divide by a stated
  `k` (2, and say so). With `p` = 4, the coarser run's `u_num` is 1.6 times the bare difference.
- **Pin a hand-worked case.** The worked example above (`u_val` = 22.4 m) and its GCI version are cheap tests.
  The paper's fin-tube numbers are a second: `u_D` = 2.17 W, `u_num` = 0.07 W and `u_input` = 6.37 W give
  `2 u_val` = 13.5 W, 18.0% of `q_D` = 74.9 W [S1 p. 7, Tables 1 and 2].
- **Flag the shared atmosphere.** Where site pressure or temperature feeds both the simulation and the
  altimeter reduction, say the plain sum double counts it, or sample it once through both.

## Open questions

- The standard's own divisor from a GCI to `u_num` (§2-4 or §2-5) was not read. [S5] gives 2 or 1.1 to 1.15
  [S5 p. 17]; [S6] confirms a conversion exists [S6 p. 7].
- §5-4's propagation for a data-reduction equation that is itself a model was not read. That case is the
  barometric altimeter's.
- [S1] did not confirm that its authors sat on the committee that wrote the standard. The preview has no
  roster page.
- Whether `u_D` from a log's own noise is random or systematic changes how it averages over samples. Random
  parts shrink as √N and systematic ones do not [S1 p. 10]. This note does not settle it for apogee.

## Sources

All read 2026-11-23; PDFs cached under `refs/papers/vv20/` (gitignored), none committed.

1. **S1.** L. Eça, K. Dowding, P. J. Roache, "On the Interpretation and Scope of the V&V 20 Standard for
   Verification and Validation in Computational Fluid Dynamics and Heat Transfer", VVS2020-13077, ASME V&V
   Symposium 2020, SAND2020-3788C, <https://www.osti.gov/servlets/purl/1774746>. Copyright ASME: cite only.
2. **S2.** ASME V&V 20-2009, cover and contents (pp. iii to v),
   <https://www.asme.org/getmedia/692ee706-d396-49fd-b62c-d8e3de655a27/21356.pdf>. ASME's: cite only.
3. **S3.** *Validation Procedure for the 3rd Workshop on CFD Uncertainty Analysis*, Lisbon, October 2008, 5 pp.,
   <https://maretec.ist.utl.pt/html_files/VV2010_files/ValidationProcedure.pdf>. No license stated: cite only.
4. **S4.** P. J. Roache, *Comments on the Third Lisbon V&V Workshop*, 2008, 9 pp., no license stated: cite only,
   <https://maretec.ist.utl.pt/html_files/CFD_workshops/html_files_2008/Discussion/RoacheCommentsLisbonIII.pdf>.
5. **S5.** D. Yeo, *A Summary of Industrial Verification, Validation, and Uncertainty Quantification Procedures
   in Computational Fluid Dynamics*, NISTIR 8298, March 2020, §4, pp. 12 to 19,
   <https://doi.org/10.6028/NIST.IR.8298>. Free of charge (title page): cite and cache.
6. **S6.** J. Weinmeister, D. P. Sanjaya, "Successive Procedure for Solution Verification Based on User Needs",
   VVUQ2024-127747, ASME VVUQ Symposium 2024, <https://www.osti.gov/servlets/purl/2406787>. Cite only.
7. **S7.** NPARC Alliance, "Examining Spatial (Grid) Convergence", NASA Glenn, updated 2021-02-10,
   <https://www.grc.nasa.gov/www/wind/valid/tutorial/spatconv.html>. Not cached.
