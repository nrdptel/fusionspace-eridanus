# ADR-110: M3.2b: OpenRocket flies the export; only counts are published (2026-09-29)

- **Status:** accepted
- **Summary:** M3.2b: OpenRocket flies the export; only counts are published

**Context.** M3.2's bullets: every corpus design, read by hpr and written back out, loads in
OpenRocket 24.12, and OpenRocket's flight of the written file is within 0.5% of its flight of the
original. M3.2a (ADR-109) wrote files that read back in hpr as the same design, but OpenRocket
reads more than hpr does. Flown before a kept `<preset>` was moved first (ADR-109, point 6), a jar
example flew 0.6% to 0.9% low and a private design 10% to 11%: the check must fly files, not only
read them. The designs are other people's, so neither the files nor per-design results can be
committed.

**Decision.**

1. **Two `flights.py` records, compared.** Both are gitignored: OpenRocket's calm-air flights
   (seed 1, conditions from each file's first stored simulation, or OpenRocket's defaults) of the originals under `refs/` and
   in the jar (`corpus-out/openrocket-flights.json`), and of the files
   `cargo xtask ork --export corpus-out/ork-export` writes (`corpus-out/openrocket-export-flights.json`).
   `cargo xtask ork-export-flights` matches them configuration by configuration and commits counts
   by source (`validation/reports/openrocket-export-flights.{json,md}`), never a file name.
2. **The records must be current and comparable.** It refuses two records made with different
   jars, scripts or seeds, or with other scripts or another jar than the committed ones; an
   original whose SHA-256 is not the one flown; an export record whose files are not the ones hpr
   writes now; and a design found in one record only, unless hpr cannot read it.
3. **"Every corpus design" means every design OpenRocket 24.12 opens as written.** An export can't
   be expected to open where its original doesn't. Of the others, a design hpr can't read has no
   export, and one hpr reads must be refused as its original is, with the same error. A
   configuration left unflown (no motor, or aborted) must be left unflown alike both ways. The
   bar is met when every export of an opened original opens, no design is refused for a new
   reason or opened only as exported, no configuration is unmatched, and every configuration
   flown both ways is within 0.5%.
4. **CI holds the committed report to itself**: its page is its JSON, its total the sum of its
   sources, it meets the bar, and it names the committed scripts. CI has neither OpenRocket nor the corpus, so `--check`, which
   remakes the report and compares, runs only where both are.

**Measured** (2026-09-29): 75 designs; OpenRocket opens 71 as written and all 71 exports. The other
four: hpr reads two of them, and OpenRocket refuses those two exports with the originals' errors.
151 configurations of 48 designs fly both ways, all within 0.5%, 142 to the same apogee bit for
bit; the largest difference is 0.000162%, cause untraced. 23 opened designs have nothing to fly
(no configuration, or no motor); eighteen configurations fly neither way (17 with no motor, one
aborted both times for the same cause).

**Not chosen: a tolerance tighter than 0.5%.** The flights agree far more closely, but the
milestone's bar is 0.5%, and the report publishes the largest difference beside it.
