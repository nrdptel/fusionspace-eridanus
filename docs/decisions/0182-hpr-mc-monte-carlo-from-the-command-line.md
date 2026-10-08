# ADR-182: `hpr mc`, Monte Carlo from the command line (2026-10-06)

- **Status:** accepted
- **Summary:** M4.6 is split: M4.6a `hpr mc`, M4.6b Python. A Monte Carlo run's `FlightInputs` now carry the flight's separations, so a staged `.ork` is scattered as `hpr sim` flies it, and a tumble hpr adds to a separated part is not given a dispersed lag. `hpr mc` takes one option a dispersion, in the command line's units, flies its samples on every core, prints the nominal flight beside the apogee's and landing distance's spreads and three landing ellipses, counts its failed flights by reason, and exports every flight as a CSV table (`hpr_analysis::table::RunTable`) whose numbers read back to the run's bits.

**Context.** [ADR-163](0163-the-second-pass-products-and-priorities.md) put Monte Carlo from the
command line and Python in release 0.1 (M4.6). M6.1's run ([ADR-134](0134-monte-carlo-dispersion-independent-normals-one.md))
was reachable only from Rust, and `FlightBuilder::inputs` refused a flight with separations, since
`FlightInputs` had nowhere to keep them: so a staged `.ork`, which `hpr sim` flies, couldn't be
scattered. M4.6's *done when* asks for any design `hpr sim` flies, the runs as CSV in round-trip
form, Python's runs as arrays, and both equal to the Rust API's runs bit for bit.

**Decision.**

1. **Two increments.** M4.6a is `hpr mc` with staged flights; M4.6b is Python, its arrays checked
   against `hpr mc`'s CSV from the same build and seed. The wheel is a release build and the CLI's
   tests a debug build, whose last digits may differ, so Python's check needs the CLI built the
   same way; that is M4.6b's.
2. **`FlightInputs` carry the separations** (`separations: Vec<Separation>`), applied after the
   recovery devices, as `FlightBuilder::simulation` applied them, which now builds its simulation
   from `inputs()`. A run with no dispersion flies the builder's staged flight, bit for bit
   (`a_staged_flight_is_dispersed_with_its_separation`). A separation's trigger isn't dispersed: a
   split at a burnout follows the dispersed burn, and one timed in seconds stays, so a flight whose
   booster still burns then fails and is counted, unless the split may drop a burning motor
   ([ADR-172](0172-a-stage-s-first-burnout.md)); a test pins it. After a split with nothing left
   to burn, each part flies as a point with only its open devices' drag, so a part parted before
   apogee by a separation or a charge, still rising, whose device fired by the split but waits out a drawn lag would climb
   through the lag with no drag (found in review). The flight now refuses that by name, in
   `hpr-sim`, and the sample counts as failed; no `.ork` flight changes, as the reader already
   refuses such a device's delay ([ADR-165](0165-an-unpowered-separation-in-hpr-sim.md)).
   The flight's landing is then the part's that keeps the nose (`FlightSummary::nose_landing`),
   as its apogee is, in the spreads, the ellipses and the CSV.
3. **A tumble's lag is not dispersed.** A device whose drag is `DeviceDrag::Tumble` is hpr's
   stand-in for a part with nothing open ([ADR-159](0159-a-powered-separation-in-hpr-sim.md)),
   triggered at the split. Nothing deploys it, so it has no deployment lag to scatter, and a
   positive draw would leave the dropped part coasting with no drag. Its entry in the draw is 0 and
   its stream is never drawn. This changes no run that has no tumble.
4. **One option a dispersion, in the command line's units:** `--mass-sd`, `--cg-sd`, `--drag-sd`,
   `--impulse-sd`, `--burn-time-sd`, `--delay-sd`, `--wind-sd`, `--wind-from-sd`,
   `--inclination-sd`, `--heading-sd` and `--deployment-lag-sd`, angles in degrees as `hpr sim`'s
   are, fractions as fractions. Each is 0 by default, as the library's are; a run with none says
   every flight is the nominal one. No preset scatter: M6.1 has none, and a default would be a
   number the user didn't choose. `--runs` is 1 to 100,000 (100 by default), `--seed` 0 by
   default. A negative or non-finite standard deviation is refused by its option's name.
5. **The design, motor and launch are `hpr sim`'s,** read by the same code (`sim::Setup`), so
   every refusal and note of `hpr sim` holds. The nominal flight is flown once for the table's
   *nominal* column and its issue warnings (ADR-180, ADR-181); the envelope flags are counted over
   the flights. Issue warnings per flight are left out: they are the design's and the speed's, and
   the nominal flight names them. `hpr sim`'s notes on the flight's own numbers (a fall from
   apogee before a device opened, a device set above the apogee, a dropped part that climbs
   higher) are said of the nominal flight, from the same code.
6. **On every core.** `hpr-cli` turns on `hpr`'s `parallel` feature and flies `run_parallel`.
   M6.1 made a sample's draws depend only on the seed, its index and the input, so the run is the
   same on any number of threads; a test compares the CLI's parallel run with the library's serial
   `MonteCarlo::run`, text for text.
7. **The CSV is a library table,** `hpr_analysis::table::RunTable`, so Python can return the same
   columns: an integer `index`; `outcome`, `failed_at`, `reason`; the draw, its lists by stage,
   motor and device counted from 1; the flight's peaks and landing; each dropped part's landing;
   the flags. A number is Rust's shortest form that reads back to the same bits (`{:?}`), a
   missing one an empty cell, words quoted per RFC 4180, and a number that isn't finite refused,
   as `hpr sim`'s export does. A `.meta.json` sidecar names the tool, the design, the seed and the
   dispersion ([ADR-174](0174-the-command-line-s-colors-hints-and-title-block.md)).
8. **Failed flights** are grouped by where they failed and their reason, each with its count and
   first index; the spreads count them as tried with no value, and an ellipse's share counts them
   outside.

**Alternatives considered.**

- *A preset scatter* (the guide's example values) as the default: rejected, as above; the guide's
  [Choosing the numbers](https://nrdptel.github.io/hpr-sim/monte-carlo.html#choosing-the-numbers)
  gives sources instead.
- *A dispersion file* (`--dispersion file.toml`): left for later; eleven options cover M6.1's
  scatter, and a file adds a format to maintain.
- *Refusing staged designs in `hpr mc`*: rejected, as the *done when* asks for every design
  `hpr sim` flies, and the library change is small and pinned by a bit-for-bit test.

**Consequences.** `hpr mc` runs on any design `hpr sim` flies, its runs the library's. Python
still can't run one (M4.6b). `hpr mc` flies one wind at every height in the standard atmosphere,
as `hpr sim` does (#265). A library caller who built `FlightInputs` with a struct literal needs
the new field.
