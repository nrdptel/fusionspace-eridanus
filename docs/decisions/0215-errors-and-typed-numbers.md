# ADR-215: Errors that name what was typed, and numbers read as typed; M0.9c6 split in two (2026-10-09)

- **Status:** accepted; carries out the product system's `cli.md` and `writing.md` (*Errors*) and `data.md` (*Copying and typing numbers*) for #381's errors and typed numbers (M0.9c6), and splits #381's help, color roles and progress, with #405, into M0.9c9
- **Summary:** every numeric option of `hpr` reads a number as a person types it: spaces around it trimmed, a comma read only as a thousands separator between groups of three digits (`1,280` is 1280, and a `note:` line says `--elevation 1,280 read as 1280 m` before anything else), any other comma refused with a question (`is 3,9 meant as 3.9?`), and a value that isn't a number refused with what a number looks like. A test walks the command tree and holds every numeric option to it. `hpr sim` and `hpr mc` refuse a launch option the library wouldn't fly by its name, the value as given and its unit, with an example, before the library refuses it in its own terms (a latitude in radians). A file that isn't there is named with the closest name in its folder, or the missing folder, or a reminder that a relative path starts from the current folder; a folder given for a file is called one. `hpr motors show` with a name the bundled catalog lacks names the catalog's motors that the name begins, and what HPR Sim's cache of ThrustCurve.org says: the motors that answer it, or the command that fetches it. A refusal from inside a flight still has the library's words and no next step (#405, M0.9c9).

[adr-164]: 0164-the-fusionspace-product-system.md
[adr-208]: 0208-the-design-audit.md

**Context.** The design audit ([ADR-208][adr-208]) found six gaps in how `hpr` refuses and reads
what it is given (#381): an out-of-range latitude refused in radians without naming
`--latitude`, a missing file reported as the operating system's words alone, a motor the catalog
lacks refused without the names `hpr sim --motor` lists, help without examples and its link in
the wrong place, two color roles unused, no progress on a long run, and numbers refused when
typed with spaces or a thousands separator. The product system ([ADR-164][adr-164]) asks an error
to say what happened, then what to do, naming the thing as the user typed it and suggesting the
closest valid value; and a typed number to be trimmed, read with a thousands comma, asked about
for any other comma, and shown as read.

**Decision.**

1. **Split.** M0.9c6 takes the errors and the typed numbers, the items about what the user gave;
   M0.9c9 takes help, the Heading and Literal roles and progress, with #405. The new increment
   is numbered after c8, not by renumbering c7 and c8, so the records that name those ids stay
   right; the queue keeps c7 and c8 before it.
2. **One parser for every number.** `crates/hpr-cli/src/typed.rs`'s `Number<T>` is each numeric
   option's `value_parser`, for `f64`, `u64` and `u32` alike, and `--max-price` and `--delay`,
   which read their own text, share its trimming and comma rule. A comma is a separator only
   in the whole-number part, after one to three digits and before each group of exactly three.
   A comma read is shown back as a `note:` on standard error, and only then: a `--json` run's
   document carries the value read, and standard error stays empty. Values that are not finite
   (`inf`, `nan`) still parse, as before, and the checks after parsing refuse them.
3. **The launch's ranges are the library's.** The checks before a flight
   (`crates/hpr-cli/src/sim.rs`, `launch_checks`) copy the ranges the library already refuses
   outside: a latitude within ±90°, a finite longitude, height, heading and wind direction, a
   rail longer than 0, an inclination in (0°, 90°] and a wind of at least 0 m/s. The edges the
   library flies are flown, which a test holds.
4. **A missing file's closest name** is the name in its folder within two edits and under half
   the name's length, case ignored, after reading at most 10,000 entries, so a large folder
   can't stall a refusal. A folder is recognized by asking the path, since Windows refuses to
   read one as "access denied".
5. **`hpr motors show` reads the cache, never the network,** for its suggestion: the search
   answer `hpr sim --motor` or `hpr motors fetch` left there. With no answer cached it names the
   fetch command instead, so the same command line suggests the same thing offline and online.

**Held by.** `crates/hpr-cli/tests/cli.rs`: `every_numeric_option_reads_typed_numbers` (42
options, which a probe that gave `--interval` clap's own parser failed),
`typed_numbers_reach_the_flight`, `launch_options_are_refused_by_name_and_unit`,
`a_missing_file_says_what_to_do` and `motors_show_suggests_what_answers_the_name`; and
`crates/hpr-cli/src/typed.rs`'s unit tests of the comma rule.

**Consequences.** The audit's *data.md · Copying and typing numbers* row is met; the two
*Errors* rows stay open for #405, naming the tests that hold what is fixed. `hpr --help`,
`-h`, the color roles and progress are M0.9c9's.
