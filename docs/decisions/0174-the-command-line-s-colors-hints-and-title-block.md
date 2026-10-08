# ADR-174: The command line's colors, `help:` lines and title block (2026-10-05)

- **Status:** accepted
- **Summary:** M0.6b: `hpr` colors its streams by the product system's `cli.md` (the `--color` flag, then `NO_COLOR`, then `FORCE_COLOR` or `CLICOLOR_FORCE`, then auto, each stream on its own), in the system's `fs_style.rs` roles, copied in unchanged; never in `--json` output or a file. Every hint is a `help:` line after the fact it is about, and an error document's `help` array; clap's `tip:` and "For more information" lines are rewritten to match. `hpr --version`, every `--json` document (a `tool` object first), and every file `hpr` writes name `hpr-sim`, its version and `FS · SW · TOOL 005`, each in the place its format keeps for it; a CSV's goes in a `.meta.json` sidecar. A converted design file names this program, its `source` kept as read, amending ADR-112 §6.

**Context.** M0.6b's *done when*: "`--color` and `NO_COLOR`, `FORCE_COLOR`, `CLICOLOR_FORCE`
follow `cli.md`'s order per stream, each case tested; piped and `--json` output hold no escape
code; every hint hpr writes is a `help:` line; `hpr --version`, every file `hpr` writes and every
`--json` document name the tool, version and `FS · SW · TOOL 005`"
([ADR-164](0164-the-fusionspace-product-system.md)). On `main` before it, `hpr` printed no color,
wrote hints into the error sentence after a colon or semicolon (about 25 places in the CLI), and
printed `hpr 0.1.0`. Of the eleven formats it writes, the `.hpr` design (`provenance.tool` and
`tool_version`), the `.ork` (`creator`) and the Parquet file (`created_by`) named the program and
its version; none named the designation.

**Decision.**

1. **Colors.** `fs_style.rs` from the product system (Rev A, `32770a4`) is copied in unchanged,
   Apache-2.0, as `crates/hpr-cli/src/fs_style.rs`; a test holds it to the pinned copy when
   `refs/fusionspace-design` is checked out. Its `color_choice` reads the variables; `hpr`'s
   `console` module applies the flag first and, under auto, colors a stream that is a terminal
   with `TERM` other than `dumb`, standard output and standard error apart. `hpr_cli::run`, which
   the tests and the guide's examples call, takes both streams for pipes and reads no variable,
   so their text doesn't depend on the shell; the binary calls `run_with` with the process's own.
   The binary wraps both streams in `anstream`'s `AutoStream::always`, which passes escape codes
   through and, on a Windows console that can't take them, writes them as the console's colors.
   `--json` turns standard output's color off whatever the flag says (`cli.md`: machine output
   "never contains color"); files are written without it.
2. **What gets color.** The prefixes `error:` (bold red), `warning:` (bold yellow), `help:` (bold
   blue) and `note:` (bold) on either stream, and clap's help and usage text through
   `fs_style::CLAP_STYLES`. The rest of a result stays plain until a later increment needs more:
   M0.6b's *done when* asks for the decision and the prefixes, not a restyle of every table.
3. **Hints.** A refusal with something to do is `Failure::Helped { message, help }`: the `error:`
   line holds the fact, each hint follows on its own `help:` line, and the JSON error document
   gains `help`, an array (empty when there is none). A hint inside the text of a result (the
   configurations `--config` can fly, the Accuracy page, what flies a fetched motor) is a `help:`
   line too. Clap writes its own hints as an indented `tip:` and a closing "For more information,
   try '--help'."; `hpr` rewrites both as `help:` lines, matching clap's words, and a test fails
   if a clap release words them otherwise; its `[possible values: …]` line becomes one too.
   `--delay`'s parser hands its hint to clap as a suggestion, so it follows the same path. A
   library's message keeps its hint at its end, after `"; "`, so other programs read it whole,
   and the library exports the hint as a constant (`hpr_motor::motor::UNITS_HINT`,
   `hpr_atmos::error::INCOMPLETE_COLUMN_HINT`, `hpr_validate::run::HINTS`); `hpr` splits a
   refusal that ends with one into its `error:` and `help:` lines. `hpr validate`'s failed check
   gives its problems and their hints apart (`Checked::help`), and its document gains `help`.
4. **The title block.** `hpr_core::tool` holds the name `hpr-sim` (what the design files already
   wrote), the version and `FS · SW · TOOL 005`, and `stamp()`, the three on one line. Each format
   carries them where it keeps such a thing: a `tool` object first in every `--json` document, the
   JSON recording, the GeoJSON (a foreign member, RFC 7946 §6.1) and the weather profile; a
   comment in KML and `.rse`; a first `;` comment line in `.eng`, dropped when read and not carried
   into the next write, so stamps don't pile up; `<metadata>` in the SVG plot; the `.ork`'s
   `creator`; key-value metadata `tool`, `tool_version` and `designation` in Parquet beside its
   `created_by`; and `provenance.designation` in the `.hpr` format, an optional key added in place
   to version 0.2, which `docs/format/hpr.md` allows while the major number is 0. `hpr --version`
   prints `hpr 0.1.0 · FS · SW · TOOL 005`, with the binary's name as clap writes it.
5. **The CSV's sidecar.** `data.md` asks that a CSV open cleanly in a spreadsheet, with the tool
   in a sidecar or in `#` lines only where the reader expects them. `hpr sim --export flight.csv`
   writes `flight.meta.json` beside it: the `tool` object, the CSV's file name, the design and
   configuration flown and the rows. It leaves out the date `data.md` lists, so a run writes the
   same bytes each time; the file's own time says when. A run refuses before flying when the
   sidecar would be a file it reads or writes.
6. **A conversion names this program.** ADR-112 §6 kept a converted design's provenance as read.
   A written file's provenance says which program wrote it (`docs/format/hpr.md`), so a
   conversion now writes this program, its version and its designation
   (`DesignFile::stamped`), and keeps `source`, where the design came from, as read.

**Alternatives considered.**

- **The system's `color_choice` alone, with anstream deciding per stream.** It reads the process's
  variables, so every in-process test and every generated guide example would change with the
  shell that ran it; the decision is split so the tests can give it the variables.
- **`#` comment lines in the CSV.** Rejected by `data.md`'s own rule: a spreadsheet shows them as
  data.
- **The designation inside Parquet's `created_by`.** Readers parse that string for the writer's
  version (`<app> version <x> (build <y>)`), and extra words can break the parse.
- **A `tool` name of `hpr`, the binary's.** The design files already wrote `hpr-sim`, and the
  designation is the project's.

**Consequences.**

- Every published CLI schema gains a required `tool`; `error.schema.json` gains `help`; a new
  `sim-export-meta.schema.json`. The `.hpr` schema and its Python and TypeScript types gain
  `provenance.designation`.
- A `.hpr` written now has a key an earlier build of 0.2 refuses as unknown; files already written
  read as before.
- The guide's examples show `help:` lines; [The command line](../cli.md#colors-and-messages)
  describes the order and the prefixes.
- Not a file for the user: the network cache (`hpr_net`'s answers kept as fetched, with their own
  metadata), which holds a server's bytes exactly and so names no tool.
- Not checked: whether RockSim and OpenRocket open a `.eng` with the middle dot in its first
  comment, a `.rse` with a comment before its root, and a `.ork` with the new `creator`; each
  format's page says so.
- Left for later: the rest of `cli.md`'s rules that M0.6b's *done when* doesn't name, among them
  warnings and notes on standard error rather than with the result (they go to standard output
  today, with the result they qualify), and colors beyond the prefixes.
