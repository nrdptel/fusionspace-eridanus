# ADR-317: M7.3c in two: the posterior's sampler, then the test (2026-11-02)

- **Status:** accepted; splits [M7.3c][m7-3c] in two under [ADR-314][adr-314], whose *done when* is unchanged and met when M7.3c1 and M7.3c2 are
- **Summary:** the Bayesian option draws each flight's posterior by importance sampling around the
  least-squares fit, so by estimate it costs a few hundred flights a fit, where a Markov chain
  would need thousands. The sampler comes first, tested on posteriors known in closed form; then its 90%
  credible intervals face ADR-257 §1's test on sets A and B, raw, with no fitted shares.

[m7-3c]: ../decisions-and-roadmap.md#m7-3c
[adr-257]: 0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
[adr-314]: 0314-m7-3-in-five-residuals-first.md
[adr-316]: 0316-m7-3b-the-least-squares-fit.md

## Context

M7.3c asks for the same parameters as [the least-squares fit][adr-316], by a Bayesian fit with
uncertainty, under the same test, with truths drawn from the stated ranges and not from its prior.
The least-squares fit already has a Gaussian prior on each stated range, and its covariance at the
answer is the Laplace approximation to that posterior. Its intervals needed two widenings and
fitted shares to pass ([ADR-316][adr-316] §11–12): the posterior isn't Gaussian where a wind is told
by heights alone, and the residuals run on from bin to bin.

A least-squares fit flies the simulation about 70 times on set A, about 0.8 CPU-seconds (ADR-316
§7, measured in development). A random-walk Markov chain on the fit's eleven unknowns would need,
by estimate, some thousands of steps a flight to settle and mix, each a flight flown: at about
11 ms a flight, 3,000 steps for each of a report's 3,500 fits is over a CPU-day.

## Decisions

1. **Importance sampling from the Laplace approximation.** Draws from a Student-t of 4 degrees of
   freedom centered on the least-squares answer, its scale that fit's covariance, are weighed by the
   posterior (J. Geweke, *Econometrica* 57(6), 1989). Each draw flies once. A posterior close to its
   Laplace approximation needs, by estimate, a few hundred draws; one far from it shows as few effective draws
   (L. Kish, *Survey Sampling*, 1965): its quantiles still come back, flagged as too few to read,
   and M7.3c2 counts how many are.
2. **The likelihood is the least-squares fit's own.** The same residuals, noise and priors, so the
   two options differ only in how they read the posterior. The residuals' run from bin to bin enters
   as ADR-316 §11's widened noise; modelling it in the likelihood instead is left to M7.3c2's
   measurement.
3. **The intervals are tested raw.** A credible interval that needed fitted shares would be the
   least-squares fit's again. M7.3c2 counts the posterior's 5% and 95% points as they come. If they
   fail, the report shows by how much, M7.3c stays open, and an ADR says what's done about it; the
   shares aren't brought in to make them pass.
4. **Truth is drawn as M7.3b's is.** Set A's truths are drawn uniform on the stated ranges and the
   prior is Gaussian, so truth isn't drawn from the prior, as the done-when asks. Set B's truth is
   M7.3b's, the least-squares fit on noise-free samples.

## Increments

- **M7.3c1 The posterior's sampler.** *Done when:* `hpr_forensics::posterior::sample` draws a
  posterior by importance sampling from a Student-t around a given peak and covariance, and gives
  each unknown's 5%, 50% and 95% points, its effective draws and its failed draws; on a Gaussian
  posterior its points are within 0.06 standard deviations of the closed form, and on a curved one
  within 0.08 of a quadrature, each test failing when the proposal's density is left out of the
  weights; a draw that can't be had has no weight and is counted; too few effective draws says so;
  the same seed gives the same answer, bit for bit.
- **M7.3c2 The Bayesian fit under the test.** *Done when:* `hpr_forensics::fit` has a Bayesian
  option, the posterior of the least-squares fit's unknowns sampled by M7.3c1 around its answer,
  flying each draw; `cargo xtask fit` writes its counts beside the least-squares fit's, on the same
  1,000 fitted flights of each of sets A and B and the same 1,500 flights for the wind (1,497 with a
  track read), truths drawn from the stated ranges and not its prior; its raw 90% credible
  intervals pass ADR-257 §1's three counts on sets A and B, its three probes failing; the drag and
  impulse scales are identified on set A, and every parameter comes back not identified on pure
  noise; its effective and failed draws are counted; the fit's page says what it adds and how far
  to trust it.

## What this leaves

- **The cost is a measurement to come.** A few hundred draws a fit is the plan; M7.3c2 measures it
  and the report's time.
- **A posterior far from its peak isn't reached.** Two modes, or a ridge the Laplace approximation
  crosses, give few effective draws, which is reported, not mended.
