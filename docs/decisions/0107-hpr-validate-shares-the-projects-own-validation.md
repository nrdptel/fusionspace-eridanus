# ADR-107: `hpr validate` shares the project's own validation check; `hpr convert` translates `.eng` and `.rse` by the format notes (2026-09-29)

- **Status:** accepted
- **Summary:** `hpr validate` shares the project's own validation check; `hpr convert` translates `.eng` and `.rse` by the format notes

**Context.** M4.2c is done when `hpr validate` fails where `cargo xtask validate --check` does,
and `hpr convert` round-trips `.eng` and `.rse` motor files. The check (the report reproduced,
every scored metric in tolerance, the census held, ADR-084) lived in `xtask`, the repository's
own tool, which a user's `hpr` can't call. `hpr_motor` wrote each format back to itself, but
converting between them was only sketched in `docs/format/rse.md` ("not implemented yet"). The
formats weigh in different units: `.eng` in kg, `.rse` in g, and multiplying by 1000 rounds in
binary for about a quarter of four-decimal masses (`0.0041 × 1000 = 4.1000000000000005`).

**Decision.**

1. **One check, in `hpr-validate`.** `committed::check` holds a run to the committed reports and
   the census, and `Checked::lines` and `summary` give what is printed. `cargo xtask validate
   --check` and `hpr validate` both call them, so they pass and fail together by construction.
   On every path that reaches the reports, xtask prints byte for byte what it printed before the
   move; the reviewer checked nine spoiled copies. Two error paths changed: an unreadable
   `latest.md` or `.json` is reported with the census's result rather than alone, and a report
   that isn't JSON says so rather than "reading". `hpr-cli` depends on `hpr-validate` (the crate
   map's row changes); `census --accept` stays in xtask.
2. **`hpr validate` needs a copy of the repository** (`--root`, the current folder by default),
   runs every case in the lock (no `--fast`: a partial run can't reproduce the whole report) and
   writes nothing. The lock holds the 20 RocketPy cases; the OpenRocket and real-flight reports
   are checked only against the census, as their oracles need OpenRocket and private files. A
   failing check prints its own document, `passed: false` with the reasons, and exits 1, even
   when standard output is closed; it is a result, not an unreadable input, so it isn't an error
   document. `hpr` must be built from the copy's commit. A release build reproduces the report
   too (the check compares the digits the platforms share).
3. **`hpr convert` converts motor files**, by `hpr_motor::convert`, in the pure core. To `.rse`
   it fills what ThrustCurve.org's `.rse` files give, as the format notes counted them (the
   origin, `Itot`, `peakThrust`, `burn-time`, `avgThrust`, `m` by the impulse fraction, `cg` at
   half the length, the flags, `massFrac`, `Isp`), and `Type="unspecified"`, which RockSim's
   guide requires. To `.eng` it drops those, named in a warning; hpr uses none of them for a
   solid motor. Delays trade `-` for `,` and `P` for `1000`. A name or maker of several words is
   joined by `_`, also when rewriting `.eng`: OpenRocket 24.12 refuses a `.eng` header of eight
   fields (`motor_files.py` checks it), and no real file has one. A hybrid and an engine without delays it can
   read are refused, the latter until `--delays` gives them, which fills only those. A catalog
   motor converts from its bundled curve with the catalog's size and masses, the ones hpr flies,
   warned where the curve's header differs (10 of the 32 differ by over 0.1%); a header that is
   already the mass hpr flies (`g × 1e-3`) keeps its digits, and in a `.rse` file `massFrac`,
   `Isp`, `m` and `cg` are rescaled to the replaced figures. The delays stay the curve's. The output is replaced if it
   exists, never when it is the input. Design files are not converted (M3.2).
4. **Masses move by the decimal point.** A mass is printed in its shortest digits, its exponent
   shifted by three and read back, so a mass of up to 15 significant digits in a double's normal
   range comes back bit for bit (such decimals never share a double; a proptest pins it). One of
   16 or 17 digits may fall between the other unit's doubles, and a warning says so; one with no
   finite, nonzero value in the other unit is refused.
5. **What "round-trips" means, measured.** The thrust curve, the size and the masses come back
   bit for bit both ways. Other text may come back respelled (delays, names and makers of
   several words, the origin point, comments), each read the same or warned. After one
   conversion a file is a fixed point, byte for byte, but for `.eng` delays of `-`, which name
   none: `.rse` leaves them out, and converting back needs `--delays`. Over the 32 bundled curves: all 29 `.eng`
   files come back whole except two whose delays are respelled (`p`, `1000` to `P`) and two with
   a 17-digit mass, each warned; the 3 `.rse` files keep every shared value but one maker,
   joined by `_`.
6. **Checked by another reader.** `validation/oracles/openrocket/motor_files.py` converts the 32
   bundled curves with `hpr convert` and loads both files in OpenRocket 24.12: it opens all 32
   and reads 29 as it reads the originals, comparing name, maker, type, size, masses, curve and
   delays. Two `.rse` files' type (reloadable, single-use) reads as unknown from `.eng`, which
   can't say it; one `.eng` delay `1000` reads as no delay there and as plugged from the `.rse`,
   as hpr reads both. RockSim is not checked.
7. **The guide's examples may write files.** An argument with no `/` ending in an extension of
   letters (`F15.eng`) goes to a scratch folder of the call's own, so `cargo xtask cli` never
   writes into the repository, and the page shows that folder as `<the scratch folder>/`.

**Consequences.** M4.2c's two commands leave `registry::PLANNED`; the README's and the guide's
tables list them as available; `schema/cli/` gains `convert.schema.json` and
`validate.schema.json`. The facade's `hpr::Motor::from_rse` and the catalog's curve reader still
turn grams into kilograms in binary, so a motor read from a converted `.rse` may differ from its
`.eng` in a mass's last bit; nothing claims them equal. `hpr validate` output lines change
whenever the report does, so the guide describes them in outline instead of quoting a run.
