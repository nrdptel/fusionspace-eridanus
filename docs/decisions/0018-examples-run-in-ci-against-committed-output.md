# ADR-018: Examples run in CI against committed output; pages quote files, checked line for line (2026-09-18)

- **Status:** accepted
- **Summary:** Examples run in CI against committed output; pages quote files, checked line for line

**Context.** M0.4c asks for a *Getting started* page whose example program runs in CI. The project's
rules ask that code in the docs compile and run in CI, and that the numbers a page quotes can't go
stale. mdBook's `{{#include}}` would put a file into a page, but only on the site: GitHub shows the
directive as text, and every page must read the same on both (ADR-016). And a page that shows what
a program prints quotes numbers that move whenever a model does.

**Decision.**

- **The first flight is Valetudo, recovered.** `crates/hpr-sim/examples/first_flight.rs` flies
  RocketPy's Valetudo design (K400C curve) from a 3 m rail in a 5 m/s wind, with a drogue at
  apogee and a main at 150 m. The design is committed, the flight tests and benchmarks already fly
  it, and it stays subsonic (Mach 0.36). The synthetic 54 mm design on its I175 reaches Mach 1,
  which hpr refuses until M1.8. The design is built in with `include_str!`, so the program runs
  from any directory.
- **What an example prints is committed beside it,** as `<name>.output.txt`. `cargo xtask
  examples` runs every example target that `cargo metadata` lists and writes those files;
  `--check` writes nothing, and fails if an example fails, has no file, or prints anything else.
  CI's test job runs `--check` on macOS, Windows and Linux, which shows the output is the same on
  each. Examples print rounded values (0.1 m, 0.01 s), where the platforms agree; they agree to six
  decimals on descents (M2.1a).
- **A page quotes a file under a marker.** A `<!-- quote: <path> -->` comment right above a fenced
  code block makes the block a quote of that repository file, and `cargo xtask site` fails if the
  two differ by a line (a `\r\n` reads as `\n`). The comment shows on neither GitHub nor the site.
  So the page, the file and what the program prints on each OS are one text.
- **`Environment::with_wind`** sets a wind without `Arc` and struct-update syntax, which a first
  program shouldn't need.

**Consequences.**

- A model change that moves a printed digit fails CI until `cargo xtask examples` rewrites the
  output and the page's quote is copied again: two steps, on purpose, so the page can't go stale.
- An example that prints more digits than the platforms agree on fails CI; it must print fewer.
- The test job builds and runs every example, which adds seconds to it.
- A block without a marker is not checked, and code in prose (such as the page's suggested edits)
  is not compiled. Reviews cover those.
