# ADR-183: Monte Carlo in Python (2026-10-06)

- **Status:** accepted
- **Summary:** `hpr.MonteCarlo` flies the library's run on every core and returns its `RunTable` as NumPy arrays: numbers as `float64` with NaN for a missing value, `index` as `uint64`, words as string arrays. Its eleven arguments are `hpr mc`'s options under the names its JSON uses. `Rocket.from_file(..., recovery=True)` flies a file's recovery devices as `hpr sim` does, so a Python run and `hpr mc` can fly the same design, and a test holds them equal bit for bit with both release builds.

**Context.** [ADR-182](0182-hpr-mc-monte-carlo-from-the-command-line.md) split M4.6: `hpr mc`
(M4.6a, shipped) and Python (M4.6b), whose *done when* is "Python returns M4.6a's runs as arrays,
its failed runs counted, equal bit for bit to `hpr mc`'s CSV from the same build and seed". Two
gaps stood in the way. The Python package had no Monte Carlo. And a design it read from a file
flew without the parachutes the file stores, so no `hpr mc` run of a file with recovery could be
flown from Python at all.

**Decision.**

1. **`hpr.MonteCarlo(rocket, environment, rail_length_m, *, runs=100, seed=0, ...)`** takes the
   rocket, the environment and the rail as `Flight` does (its `drag_table` and `drag` too) and
   flies as soon as it is made, as `Flight` does. It flies the nominal flight first, so a rocket
   that can't fly at all raises the library's error and isn't counted as `runs` failed flights,
   as `hpr mc` refuses it. Then it flies `run_parallel` with the GIL released: `hpr-py` turns on
   `hpr`'s `parallel` feature.
2. **The arguments are `hpr mc`'s options under the names its JSON output and `.meta.json`
   already use** (`mass_sd_fraction`, `cg_sd_m`, `wind_from_sd_deg` and the rest), in the same
   units: degrees for angles, converted with the same `to_radians`, so the same numbers give the
   same dispersion to the bit. Each is 0 by default; there is no preset scatter (ADR-182 §4).
   `runs` is 1 to 100,000, as `--runs`. A negative or non-finite standard deviation raises
   `HprError` naming its argument.
3. **The run is the table `hpr mc --export` writes** (`hpr_analysis::table::RunTable`), read as
   a dictionary of read-only NumPy arrays in the CSV's column order, as `Flight` reads as one of
   its recording. A number column is `float64`, NaN where the CSV's cell is empty, since the CSV
   refuses a number that isn't finite and NaN can't be a value. `index` is `uint64`. A word column
   is a NumPy string array, empty where the CSV's is. `to_csv()` returns the CSV's bytes.
   `failed` counts the failed flights and `failures` groups them as `hpr mc`'s JSON does (`at`,
   `reason`, `count`, `first_index`). `nominal` is the nominal flight's summary, and `dispersion`
   is the arguments as given. The spreads and landing ellipses that `hpr mc` prints are left to
   NumPy for now; the page lists that as a gap.
4. **A Python drag or wind function works in a run.** Each flight calls it, taking the GIL for
   the call, from whatever thread flies it. The run shares one exception slot: the first
   exception makes every later call refuse at once, so the remaining flights fail fast, and
   `MonteCarlo` then raises that exception. The run is still deterministic when the function is
   (the samples' draws depend only on the seed, the index and the inputs).
5. **`Rocket.from_file(path, configuration=None, *, recovery=False)`.** With `recovery=True`,
   the file's devices are mapped by `hpr::ork::recovery`, the function `hpr sim` uses, and come
   before any `add_parachute` adds, so the file's first device is parachute 0. Left out, nothing
   changes, so no existing script changes its flight. A device that never opens in the
   configuration is named in `notes`, as `hpr sim` notes it. A mapping hpr refuses raises
   `HprError`, where `hpr sim` flies no device and notes why: here the caller asked for the
   file's devices. A rocket's JSON stores none, and says so in `notes`. Separating
   configurations stay refused, as before.
6. **The check.** `test_a_run_is_hpr_mcs_bit_for_bit` runs the release `hpr mc --export` on the
   public `level-1.ork` (one parachute at the ejection charge) for 40 flights with all eleven
   inputs scattered, a wind and a leaning rail, and the 50% mass scatter leaves some flights a
   negative mass. It then holds every cell of every column equal to Python's arrays (numbers by
   their bits), and `to_csv()` equal to the file byte for byte. `scripts/python-tests.sh` builds
   the command line in release, as the wheel is (ADR-182 §1). Three mutations fail the test: a
   seed one higher, one standard deviation one ulp higher, and a hand-added parachute of the
   file's printed drag area in place of `recovery=True`.

**Alternatives considered.**

- *Defaulting `recovery` to true*, as `hpr sim` flies a file's devices: rejected for now. It
  would silently change every existing script's landing. A later release can flip the default
  with a note.
- *Refusing Python functions in a run*: rejected, as the slot already serializes their
  exceptions and a wind profile written in Python is the main way to fly one from Python (#265).
- *Spreads and ellipses as methods*: left for later. The done-when asks for the runs as arrays,
  and NumPy's percentiles cover the spreads.
- *Running the command line from the test as a debug build*: rejected, as its last digits may
  differ from the wheel's release build (ADR-182 §1).

**Consequences.** A Python program flies `hpr mc`'s runs and gets their numbers to the bit, failed
flights included. The comparison depends on the command line and the wheel being built with the
same profile on the same platform; the test builds both. A Python wind function in a run is called
from several threads one at a time, so a run with one is slower than one without.
