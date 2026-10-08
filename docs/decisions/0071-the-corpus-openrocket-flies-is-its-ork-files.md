# ADR-071: The corpus OpenRocket flies is its `.ork` files (2026-09-25)

- **Status:** accepted
- **Summary:** The corpus OpenRocket flies is its `.ork` files

**Context.** ADR-070's M2.2e2 has `flights.py` fly every configuration of `refs/loft-fixtures`
OpenRocket can. That library holds 27 `.ork` files, 4 RASAero `.CDX1` files and 4 RockSim `.rkt`
files. `flights.py` finds `.ork` files only. Named directly, the other eight give 3 driver errors
(`'NoneType' object is not iterable`, all `.CDX1`) and 5 files that open with no configuration
(`cargo xtask ork-flights --corpus corpus-out/openrocket-flights-other-formats.json`). OpenRocket
itself opens them (#168). hpr reads neither format until M3.4 (RockSim) and M3.5 (RASAero).

**Decision.** M2.2e's corpus is the library's 27 `.ork` files. The eight others are counted and
named here, not flown: M2.2e3 could not compare them, since hpr can't read them, and fixing the
driver belongs with the milestones that bring their readers (#168). The `.ork` flights are
counted by `cargo xtask ork-flights --corpus`, which prints no file name.

**Consequences.** OpenRocket flies 88 of the 89 configurations of the 27 `.ork` files to the end,
none refused or aborted. The 89th names a motor, with its digest, that OpenRocket loads no motor
for, so it has none to fly. The record keeps neither OpenRocket's loader warnings nor the digest
of the curve each flight used, so M2.2e3 must check that its curve is the file's before comparing.
