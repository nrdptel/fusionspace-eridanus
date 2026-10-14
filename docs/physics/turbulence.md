# Turbulence

This page describes the random gusts HPR Sim generates from the Dryden model, repeatable from a
seed, and why no flight uses them.

## In short

- **What it models:** random gusts from the Dryden model: a gust pattern with a published
  spectrum (how gust strength spreads over wavelength), sized by height and the wind at 20 ft, and
  repeatable from a seed.
- **Sources:** MIL-F-8785C, the US military flying-qualities specification (1980), noting where
  MIL-HDBK-1797 (1997) differs; xoshiro256++ (Blackman and Vigna, 2021) and Marsaglia and Bray's
  polar method (1964) for random numbers.
- **How well it is validated:** analytic and unit tests only: over 2²⁰ samples, each component's
  spectrum is within 4 standard errors of theory in every octave band (±1–3% in the wide bands).
  Not compared with another simulator or a real flight.
- **What it leaves out:** Dryden is an aircraft model, unvalidated for rockets. Its frozen gust
  pattern needs airspeed well above the gusts, which fails on the rail and near apogee. No flight
  uses it, and none is planned ([issue #39](https://github.com/nrdptel/fusionspace-eridanus/issues/39)).
  Above 2000 ft the caller supplies the intensity.

## Code and sources

Code: `hpr_atmos::dryden`, using the seeded generator in `hpr_core::random`.

Sources:

- **[8785C]** MIL-F-8785C, *Military Specification: Flying Qualities of Piloted Airplanes*
  (5 November 1980), §3.7 and the definitions in §6.2.7, pinned as `mil-f-8785c`.
- **[1797]** MIL-HDBK-1797 (1997), Appendix A §4.9, for its differences only. It is not pinned:
  no stable public copy was found.
- **[BV]** D. Blackman and S. Vigna, "Scrambled linear pseudorandom number generators", *ACM
  TOMS* 47(4) (2021): xoshiro256++ with SplitMix64 seeding.
- **[MB]** G. Marsaglia and T. A. Bray, "A convenient method for generating normal variables",
  *SIAM Review* 6(3) (1964): the polar method.

## The Dryden model

The Dryden spectra, the parameters that size them, and the generator that turns them into gusts.

### Dryden spectra

Turbulence is a frozen random field that the vehicle moves through, so its spectra are functions
of spatial frequency $\Omega$ (rad/m, [8785C] §6.2.7). They are one-sided, with
$\int_0^\infty \Phi\,\mathrm{d}\Omega = \sigma^2$ ([8785C] §3.7.1.2), and $\Phi_w$ is like $\Phi_v$:

$$
\begin{aligned}
\Phi_u(\Omega) &= \frac{2L_u}{\pi}\, \frac{\sigma_u^2}{1 + (L_u \Omega)^2}, \\
\Phi_v(\Omega) &= \frac{L_v}{\pi}\, \frac{\sigma_v^2 \left(1 + 3(L_v \Omega)^2\right)}{\left(1 + (L_v \Omega)^2\right)^2}, \\
R_u(\xi) &= \sigma_u^2 \exp(-\lvert\xi\rvert/L_u), \\
R_v(\xi) &= \sigma_v^2 \exp(-\lvert\xi\rvert/L_v) \left(1 - \frac{\lvert\xi\rvert}{2L_v}\right)
\end{aligned}
$$

Each spectrum and autocorrelation is a Fourier pair,

$$
\Phi(\Omega) = \frac{2}{\pi} \int_0^\infty R(\xi) \cos(\Omega\xi)\,\mathrm{d}\xi,
$$

and a test checks this numerically.

**The factor-of-two trap.** [1797] writes the transverse spectra with $2L_v$ and $12(L_v\Omega)^2$ over
$(1 + 4(L_v\Omega)^2)^2$, and halves the lengths ($L_u = 2L_v$). The spectra are identical, but mixing
one document's lengths with the other's formula puts the transverse scales off by two. This code
uses [8785C]'s form and lengths throughout.

### Parameters

- **Low altitude** ([8785C] §3.7.3.4, Figs. 10–11). With $h$ the height above terrain in ft,
  the lengths in ft, $u_{20}$ the mean wind at 20 ft, and $k = 0.177 + 0.000823\,h$:

  $$
  \begin{aligned}
  L_u = L_v &= \frac{h}{k^{1.2}}, \\
  L_w &= h, \\
  \sigma_w &= 0.1\,u_{20}, \\
  \sigma_u = \sigma_v &= \frac{\sigma_w}{k^{0.4}}
  \end{aligned}
  $$

  - The formulas hold from 10 to 1000 ft. Below 10 ft this code uses the 10 ft values. From 1000
    to 2000 ft the figures give $L = 1000\,\mathrm{ft}$ and equal intensities.
  - The model covers only up to about 2000 ft (§3.8.1), so above that the constructor returns an
    error instead of quietly holding values.
  - Fig. 9 marks $u_{20}$ = 15, 30 and 45 kt for light, moderate and severe turbulence.
  - For aircraft at low altitude, $u$ lies along the horizontal *relative* mean wind and $w$ is
    vertical ([8785C] p. 60).
- **Medium/high altitude** ([8785C] §3.7.2, above about 2000 ft): isotropic, with
  $L = 1750\,\mathrm{ft}$.
  - The intensity against altitude and exceedance probability is only a graph (Fig. 7), so the
    caller supplies it.
  - Neither military document says how to blend 1000–2000 ft. MATLAB's documentation
    interpolates linearly, but that is its own choice.
- **Axes of the gust field:** $u$ is the longitudinal component and $v$, $w$ the transverse ones.
  - The longitudinal spectrum belongs to the component along the path through the frozen
    field. For a climbing rocket that is nearly vertical. The flight engine ([M1.6](../decisions-and-roadmap.md#m1-6))
    doesn't use turbulence, and no milestone plans it yet
    ([issue #39](https://github.com/nrdptel/fusionspace-eridanus/issues/39)); whoever adds it must align $u$
    with the path.
  - Getting that wrong changes the statistics: a horizontal gust given the longitudinal spectrum
    has twice the transverse power at low frequency and 2/3 of it at high frequency.
  - The axes' signs don't matter.

### Generator

Exact discretization, so step length never changes the statistics.

- **$u$:** a first-order Gauss–Markov state. Over a step $\Delta s$, with $\rho = e^{-\Delta s/L}$:
  $x \leftarrow \rho x + \sqrt{1 - \rho^2}\, n$.

- **$v$ and $w$:** two normalized states, driven as $\mathrm{d}x_1/\mathrm{d}s = (-x_1 + \eta)/L$ and
  $\mathrm{d}x_2/\mathrm{d}s = (x_1 - x_2)/L$, with output

  $$
  \begin{aligned}
  y &= \frac{\sigma}{\sqrt{2}} \bigl(\sqrt{3}\, x_1 \\
  &\quad + (1 - \sqrt{3})\, x_2\bigr).
  \end{aligned}
  $$

  - The stationary covariance is the same for every $L$:

    $$
    P = \begin{pmatrix} 1 & \tfrac12 \\ \tfrac12 & \tfrac12 \end{pmatrix}
    $$

  - The output's autocorrelation is exactly $R_v$. Algebra: $c\,\Phi(r)\,P\,c^{\mathsf{T}} = 2e^{-r}(1 - r/2)$
    with $c = (\sqrt{3}, 1 - \sqrt{3})$.
  - Over $r = \Delta s/L$, the transition is $\Phi(r)$, plus Gaussian noise of
    covariance $Q = P - \Phi P \Phi^{\mathsf{T}}$:

    $$
    \begin{aligned}
    \Phi(r) &= e^{-r} \begin{pmatrix} 1 & 0 \\ r & 1 \end{pmatrix}, \\
    Q_{11} &= P(1,2r), \\
    Q_{12} = Q_{21} &= \tfrac12 P(2,2r), \\
    Q_{22} &= \tfrac12 P(3,2r), \\
    P(n,x) &= \frac{\gamma(n,x)}{\Gamma(n)}
    \end{aligned}
    $$

  - Written with the regularized incomplete gamma function, $Q$ keeps full relative precision
    for tiny steps, where the closed form $\tfrac12 - e^{-2r}(r^2 + r + \tfrac12)$ cancels to nothing. The
    noise is drawn through $Q$'s Cholesky factor.

- **State is normalized,** so intensities and lengths may change from one step to the next
  (with altitude, say) without breaking stationarity.

- **Determinism:** draws come in a fixed order ($u$, two for $v$, two for $w$) from a seeded
  xoshiro256++, so the same seed and steps give bit-identical gusts on one platform.
  - The integer stream is identical everywhere.
  - Normals and gusts go through the math library's `ln` and `exp`, which may differ in the last
    bit between platforms.
- **Checkpointing:** the generator serializes, so a run can be checkpointed and resumed.

`GustField` precomputes a realization at a fixed spacing and interpolates it linearly. That makes
the gust a pure function of the path coordinate, which an adaptive integrator can evaluate
repeatedly and on rejected steps.

- **Spacing:** linear interpolation smooths wavelengths near the spacing, so keep the spacing at
  or below a tenth of the smallest scale length.
- **Size limit:** a field holds at most $10^7$ samples.

## Limits for rockets

- **Scaled for aircraft.** Dryden's lengths and intensities describe aircraft flying roughly
  level. A rocket climbs through the low-altitude model's height dependence in seconds. The
  frozen-field assumption holds when airspeed is well above the gust velocities. That is false on
  the rail and near apogee, where the gust field barely moves past the vehicle.
- **Path coordinate:** whoever adds turbulence to a flight decides what to key the field on:
  distance flown through the air, or altitude, and how to fade gusts in on the rail. This module
  does neither.

## Tests that pin this

- **`dryden::tests::dryden_spectrum_matches_theory`** (the *done when* of [M1.2](../decisions-and-roadmap.md#m1-2), the
  atmosphere and wind milestone):
  - Setup: 2²⁰ samples at 1 m (3.3 ft), with $\sigma = (1.5, 1.2, 0.9)$ m/s and $L = (40, 40, 20)$ m.
  - Estimate: 256 Hann-windowed segments of 4096 samples, averaged (Bartlett's method).
  - In every octave band from bin 1 to Nyquist, each component's mean ratio to theory is within
    4 standard errors.
  - The variance of a band's mean ratio is

    $$
    \begin{aligned}
    &\frac{1}{nK} \Bigl[1 + 2\rho_1^2\, \frac{n-1}{n} \\
    &\quad + 2\rho_2^2\, \frac{n-2}{n}\Bigr]
    \end{aligned}
    $$

    for $n$ bins
    and $K$ segments, with the Hann window's neighboring-bin correlations $\rho_1 = 2/3$ and
    $\rho_2 = 1/6$. The bracket tends to 1.94.
  - The one- and two-bin bands at the bottom are loose (±25%, ±21%); the wide bands (±1–3%) pin
    the spectrum.
  - Theory is the continuous spectrum sampled at 1 m (3.3 ft), in closed form. Below a tenth of Nyquist it
    is checked against [8785C]'s formula to 1%.
  - The variance of the record is within 5% of $\sigma^2$.
- **Exactness:**
  - `transition_preserves_the_stationary_covariance`: $\Phi P \Phi^{\mathsf{T}} + Q = P$ to $4\varepsilon$ for steps from $10^{-12}$
    to $10^3$ scale lengths.
  - `two_steps_compose_into_one`, to 1e-12 relative, down to $r = 10^{-9}$:

    $$
    \begin{aligned}
    Q(a+b) &= \Phi\, Q(a)\, \Phi^{\mathsf{T}} \\
    &\quad + Q(b)
    \end{aligned}
    $$

    with $\Phi = \Phi(b)$.

  - `transverse_output_has_the_dryden_autocorrelation`.
  - `regularized_gamma_matches_independent_references`.
- **Stepping loop:** `quarter_meter_steps_give_the_one_meter_correlation`, to 2e-3.
  - The sampling error is about 4e-4, so this catches a scale length 10% off.
  - It can't tell an exact step from an Euler step (8e-5 apart), which is why the exactness
    tests above exist.
- **Spectra:**
  - `spectra_integrate_to_the_variance`
  - `spectra_are_the_cosine_transforms_of_the_autocorrelations`
- **Parameters:** `low_altitude_parameters_follow_the_specification`, at 100 ft moderate:
  $L_u = 505.169\,\mathrm{ft}$ and $\sigma_u/\sigma_w = 1.715849$. It also checks the 10 ft floor, the 1000–2000 ft
  hold, and the refusal above 2000 ft.
- **Determinism:** the same seed gives bit-identical fields, and a serialized generator resumes
  the stream.
- **`hpr_core::random::tests`:**
  - Bit-identical to `rand_xoshiro` over $10^4$ draws for 5 seeds.
  - The normal sampler's moments and CDF at $\pm 2\sigma$, within 5 standard errors.

