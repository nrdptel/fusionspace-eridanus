# ADR-105: The command line's surface: every command registered, JSON by schema, a generated table (2026-09-29)

- **Status:** accepted
- **Summary:** The command line's surface: every command registered, JSON by schema, a generated table

**Context.** M4.2 asks for `hpr sim|validate|convert|motors|mc|optimize|compare|analyze|diagnose`,
stubs being fine for a command whose milestone hasn't come, `--json` everywhere, shell
completions, and the README's command table generated from the registered commands (Loft lesson
P10: Loft's README claimed importers that didn't exist). `hpr-cli` was a stub. Most commands have
little under them yet: flying a `.ork` from the command line has to join `hpr-io`'s reader to the
facade (its parachutes are not flown yet), `hpr-flightdata` is empty (its readers are M7.1), and
`convert` has only the motor formats to convert. That is more than one pull request.

**Decision.**

1. **Split into a to d.** M4.2a: the command surface, `hpr motors` and `hpr completions`. M4.2b:
   `hpr sim`. M4.2c: `hpr validate` and `hpr convert`. M4.2d: `hpr analyze`, with
   `hpr-flightdata`'s first log reader. M4.2's own *done when* is unchanged and closes with d.
2. **Every command is registered from the start.** A command whose milestone hasn't come accepts
   any arguments and refuses with exit status 3 and that milestone, so `hpr --help`, the
   completions and the README list the whole tool without claiming it works. `weather` (M5.2),
   which `ARCHITECTURE.md` lists, joins the roadmap's nine; `completions` is the tenth. `mc` is
   M6.1, `optimize` M6.2, `compare` (a log against its simulation) M7.3, `diagnose` M7.4.
3. **Exit codes:** 0 done; 1 an input missing, unreadable or refused; 2 a wrong command line
   (clap's own); 3 not available yet. `--help` and `--version` are answers, status 0.
4. **`--json` prints exactly one document on standard output**, success or failure, and nothing
   on standard error; a failure is an `ErrorDocument` whose `kind` (`input`, `usage`,
   `not_available`) matches the status. `--json` is looked for before parsing, up to a `--`, so a
   usage error is JSON too. clap's built-in `help` command takes no `--json`. The registry, not
   the dispatch, decides which commands refuse.
5. **The output types are the CLI's own** (`hpr_cli::output`), not the libraries', so a published
   schema changes only when the command's output does. Each derives `schemars::JsonSchema`
   (draft 2020-12); `cargo xtask cli` writes them to `schema/cli/`. Units are SI and named in the
   field, except `motors list`, which keeps the catalog's millimetres and says so in its names.
6. **One registry, generated tables.** Names and summaries come from clap's command tree;
   `registry::availability` adds what a command reads and writes, or its milestone. A format
   listed is taken from the enum the command dispatches on (`MotorFile::ALL`,
   `clap_complete::Shell`), so the table can't list one the code doesn't read. `cargo xtask cli`
   writes the table into `README.md` and `docs/cli.md`, and re-runs the page's examples in-process;
   an example must exit as its marker says (`exits 3`, else 0), takes no quotes or paths, and a
   block missing its end is refused rather than spliced up to the next block's.
7. **`hpr motors show` works a motor's figures out from its curve** with `hpr_motor` (the curve
   the simulator flies), and for a catalog motor sets ThrustCurve.org's stated figures beside
   them; `list` repeats the catalog's stated figures. A name several motors share shows each. A
   `--manufacturer` the catalog doesn't know is refused, listing its makers: `CTI`, the files'
   abbreviation, would otherwise list nothing, as if Cesaroni had no motors.
8. **A closed pipe** (`hpr motors list | head`) ends with status 0 and says nothing: the reader
   chose to stop, and whether the write fails depends on the pipe's buffer.

**Evidence.** `cargo test -p hpr-cli`: 15 tests run the built binary with `assert_cmd` and check
every `--json` document against the committed schema. Every planned command refuses with 3 and
its milestone, as text and as JSON, whatever its arguments; `motors list` is the catalog, and its
three filters narrow it; a filter that can't mean anything, `--manufacturer CTI` among them, is
exit 1; `--json` after `--` is an argument; `motors show` gives the
library's own figures for a name, a shared name, each file format `MotorFile::ALL` lists, and a
two-motor `.eng` with a `0` delay and its warning; missing, broken and non-UTF-8 files are exit 1;
every shell's completions; six wrong command lines are exit 2. Over the whole catalog, `motors
show`'s total impulse, average thrust and burn time are within 1% of the stated ones, and its
peak thrust from 16.7% below to 2.1% above, the range the guide quotes. Unit tests hold the registry to
clap both ways and pin the exit codes and the closed pipe. `cargo test -p xtask cli` fails when
a schema, a table or an example is stale.

**Consequences.** New dependencies: `clap` (derive), `clap_complete`, `schemars`; `assert_cmd`
for tests. A command's milestone makes it available by moving its name out of
`registry::PLANNED`, adding its output type to `output::schemas`, and running `cargo xtask cli`.
`ARCHITECTURE.md`'s `hpr-cli` row lists `analyze` and `completions`.
