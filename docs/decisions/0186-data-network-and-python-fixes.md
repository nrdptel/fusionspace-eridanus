# ADR-186: The data, network and Python fixes of release 0.1 (2026-10-06)

- **Status:** accepted
- **Summary:** M10.1b fixes five issues. The bundled motor catalog is written from its 32 public-domain curve files alone by `cargo xtask motor-catalog`, every figure the file's own (#295). `.hprz` attachment names are compared after canonical decomposition as well as case folding (#252). Open-Meteo and NOMADS write a site's coordinates to 5 decimals so a radians round trip keeps the cache key (#272). Three pages' accuracy figures are the report's, and the site's number trace now checks them (#298). A pytest pins Python's `recovery=True` flight to `hpr sim`'s bit for bit (#308).

**Context.** ADR-185 split release 0.1 (M10.1) in four; M10.1b holds the five issues that touch
the motor data, the network cache, the docs' accuracy figures, the Python binding and the `.hprz`
container. The largest is #295: `crates/hpr-motor/data/thrustcurve/catalog.json` held each bundled
motor's statistics as ThrustCurve.org's API states them, and that API publishes no terms. The 32
curve files are marked public domain, and their headers carry most of the same facts.

**Decision.**

1. **#295: the catalog from the files alone.** `cargo xtask motor-catalog` writes `catalog.json`
   and `src/bundled.rs`; `--check` fails if either is stale, and an xtask test runs it.
   - From each file's header: diameter, length, propellant and loaded masses, delays as written.
   - From its curve, with the methods the simulator flies: total impulse (trapezoid integral),
     peak thrust, burn time (NFPA 1125, between the 5%-of-peak crossings), average thrust (impulse
     over that burn time), and the impulse class.
   - Kept by hand: names that identify the motor (ThrustCurve.org's motor id, which `hpr-io`
     reads; designation; maker; single-use or reload; case; propellant) and the file's provenance
     (data source, license, download date, address). Names, not measurements.
   - Dropped: certifying body, update date, availability, whether the delay is adjustable, and the
     API snapshot's address and digest.
   - `validation/oracles/thrustcurve/bundle.py` no longer writes the index. With the API capture
     under gitignored `refs/`, the command prints how far the files are from the API's figures.
2. **#252: names compared as macOS compares them.** The key is NFD, then the crate's case fold,
   then NFD again: the Unicode Standard's canonical caseless match (§3.13, D145) with the crate's
   own fold. The first NFD matters: U+0345 folds into a letter, so marks in either order would
   otherwise pass as two names. `unicode-normalization` (MIT OR Apache-2.0, no I/O, builds for
   wasm32) does the decomposition.
3. **#272: coordinates to 5 decimals.** Elevation's formatter (ADR-126) moves to a shared module;
   Open-Meteo writes the site with it. Rounding moves the point at most 0.6 m, against grids no
   finer than 1 km. NOMADS' URL never held the site, only its subregion's edges, and those were
   rounded from the binary value: 2,103 of 360,001 sites given to 3 decimals flipped an edge after
   a round trip. Each edge now comes from the 5-decimal site in integer steps, rounded half away
   from zero. Wyoming's key is the station id the caller names, so it is unchanged. A saved entry
   whose URL changes is fetched once more when online.
4. **#298: the report's figures, and the trace extended.** On `VALIDATION.md`,
   *How a flight is simulated* and *Start here*, a paragraph, list item or table row that links
   the report must quote its numbers as the report writes them. Applying the rule to every page
   would flag about eight rounded quotes elsewhere: #369.
5. **#308: closed by a pin.** M4.6b's `Rocket.from_file(recovery=True)` (ADR-183) already flies a
   `.ork`'s devices. The new pytest flies three variants of public files both ways, `hpr sim` and
   Python, and compares every event, apogee, landing and recorded cell exactly. The default stays
   off, as ADR-183 decided.

**Value changes from #295, measured.** Against the API capture: no class letter changes; total
impulse, average thrust and burn time within 1% (largest +0.99%, B4's burn time); peak thrust
−16.7% (26E31-15A) to +2.1%; 26E31-15A's propellant 11.1 g to 16.9 g, as its header writes
`0.0169`. The header is the likelier value: 11.1 g would need an effective exhaust velocity of
2,350 m/s, above every other bundled motor but one. Loaded masses move by at most 4.0% (C5),
N3300R's length 1046 to 1060 mm. In the examples, the two-stage sustainer lights at 212.0 m, not
213.5 m.

**Consequences.**

- The catalog no longer depends on terms nobody stated, and every figure in it can be rebuilt from
  committed files. The check against ThrustCurve.org's stated figures (Loft lesson L42) now runs only
  where the gitignored capture is present.
- `hpr motors` labels its figures as worked out from the file, not as ThrustCurve.org's.
- `.hprz` files with two spellings of one name, which unpacked as one on macOS, are now refused.
- Breaking, as 0.x allows: `hpr motors show --json` loses its `stated` block, and the index's
  `snapshot` gives `files_url` in place of the API snapshot's address and digest.
- The delays are the header's: 68F240-15A lists 3 to 10 s where the record listed 8 to 15, and
  O6000W's header writes `0`, shown as "at burnout, or plugged" with a warning, where the record
  said plugged. The simulator flies no catalog delay; a flight's delay is the user's.
- The other propellant masses that move (3300L3200-P +6.65%, 21062O3400-P +3.1%, E26W −1.6%, C5
  −2.7%, and two more within that range) are the headers' as written, by rule; none was checked
  against its maker's figure.
