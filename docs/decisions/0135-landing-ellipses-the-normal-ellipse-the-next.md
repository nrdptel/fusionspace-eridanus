# ADR-135: Landing ellipses: the normal ellipse, the next flight's, and a count of what each holds (2026-10-01)

- **Status:** accepted
- **Summary:** M6.1b: landing ellipses in `hpr_analysis::ellipse`; the normal ellipse of the sample's mean and covariance, scaled by `k² = −2 ln(1 − p)`; a prediction ellipse for the next flight by Hotelling's `T²`; the landings each ellipse really holds counted, failures as bounds; tested against normal spreads with known answers

**Context.** M6.1b asks for landing ellipses at confidence levels, tested against analytic
Gaussians. A run gives each sample's landing east and north of the pad (`FlightSummary::landing`),
or none for a failed flight or one that never landed. Three choices are open: which ellipse "95%"
means, what to do with the samples that gave no landing, and what to say when the landings aren't
normal.

**Decision.**

1. **The default ellipse is the normal one** (`Scatter::ellipse`): centred on the sample's mean,
   its axes along the sample covariance's eigenvectors, `k √λᵢ` long with `k² = −2 ln(1 − p)`, the
   chi-square quantile with two degrees of freedom in closed form (Abramowitz and Stegun, eq.
   26.3.21 and 26.4.5; Wang, Shi and Miao, *PLoS ONE* 10(3), e0118537, 2015). It is what a
   reader expects from a "95% ellipse", and what other tools draw.
2. **A prediction ellipse sits beside it** (`Scatter::prediction_ellipse`): the region the next
   flight lands in with probability `p` exactly, for normal landings whose mean and covariance were
   estimated from `n`, by Hotelling's `T²`: `k² = ((n² − 1)/n)((1 − p)^(−2/(n − 2)) − 1)`
   (NIST/SEMATECH e-Handbook §6.5.4.3.4, after Ryan, 2000; the `F(2, m)` function from A&S eq.
   26.6.4). At 200 flights its axes are 1.3% longer; at 10, 36%. Tolerance regions (an ellipse
   holding `p` of all flights with a stated confidence) are left out: they have no closed form.
3. **Samples without a landing are counted, not dropped**, as in a `Distribution` (L54): a
   `Scatter` keeps the samples tried, and the share an ellipse holds is a `Share`, the missing
   samples counted as outside and as inside. The ellipse itself is drawn from the landings there
   are.
4. **Whether the landings are normal is measured, not assumed**: `Scatter::share_inside` counts the
   landings inside each ellipse, and the guide says what a share far from the level means (an arc
   from an uncertain wind heading). No test of normality, no other shape (kernel density, convex
   hull): the count is the check a reader can act on.
5. **The heading is reported clockwise from north in `[0, π)`**, as the rail's and the wind's are;
   a circle's is east's, `π/2`. Sums run over the points sorted by east then north, shifted by the
   first, as `Distribution`'s are, so an ellipse is the same however the samples were flown.
6. **A scatter's axes are measured from its points** (`Scatter::principal_axes`): the heading from
   the covariance, each variance as the mean square along or across it (a Rayleigh quotient,
   wrong only by `δ²(λ₁ − λ₂)` for a heading error `δ`). A covariance's entries are each rounded
   to about `ε λ₁`, so no formula on them keeps a spread narrower than that; a given
   `Covariance`'s minor variance is `(ac − b²)/λ₁`, which keeps it along the axes. The covariance
   of points on a line is cut to the Cauchy–Schwarz bound, so it passes its own checks, and
   `Covariance::new` allows four units of rounding past it. A point on a flat ellipse's axis is
   kept inside by a floor of `10⁻¹²` of its size and distance on each semi-axis. A point more
   than 10⁹ m from the pad is refused, so no square or sum can overflow. An `Ellipse`'s fields
   stay public, as `Summary`'s do, but it is `#[non_exhaustive]` and checked when read back.
7. **The tests use normal spreads whose answers are known**: the scale against NIST's chi-square
   table and the closed forms; the axes of rotated covariances to 1e-14; the normal density
   integrated over its ellipse (along rays, in closed form per ray) to its level within 1e-12;
   100,000 seeded points giving back their covariance and their share inside within five standard
   errors; the prediction ellipse of 3, 5 and 20 points holding a new point at its level over
   20,000 trials, and the normal ellipse visibly less; a spread 10⁸ times longer than wide; any
   2 to 20 points inside their own 99.99% ellipse (proptest, from the leverage bound
   `(n − 1)²/n`).

**Consequences.** M6.1b is met. The guide's example draws the 50% and 95% ellipses of its 200
landings (48.0% and 94.5% inside). Hotelling (1931) and Chew (1966) were not read; the prediction
formula rests on NIST's handbook and on the Monte Carlo test. NIST's two pages are cited by URL,
not pinned in `refs.lock.toml`: each fetch differs (a per-request script). Landings over uneven ground, and
a landing ellipse in latitude and longitude, are not handled: the ellipse is in the pad's local
east-north plane.
