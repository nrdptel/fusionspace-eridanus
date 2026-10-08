# ADR-113: M3.3c: TypeScript and Python types generated from the schema (2026-09-29)

- **Status:** accepted
- **Summary:** M3.3c: TypeScript and Python types, and a reader for each, generated from the schema by xtask

**Context.** M3.3c asks for TypeScript and Python types generated from the design format's schema
(ADR-111), a check that fails when they are stale, and each reading a public document. Types alone
check nothing at run time: `JSON.parse(text) as DesignFile` believes any JSON. Ready-made
generators exist under permissive licences (quicktype, json-schema-to-typescript,
datamodel-code-generator), but each covers one language or needs a Node.js or Python toolchain
inside `cargo test`, their output moves with their versions, and none writes a reader that checks
a document the way hpr's does.

**Decision.**

1. **xtask generates both**, from the schema `hpr_format::schema()` builds, which a test holds equal
   to the committed one (`xtask/src/format_types.rs`, about 1,700 lines with its two reader
   templates, tests aside). `cargo xtask format` writes the schema and both files, `--check` fails when any is
   stale, and so does a test. The generator refuses a schema keyword or `format` it doesn't
   check, so a new kind of constraint can't pass the readers silently.
2. **What they hold.** A type per schema definition, under the schema's names, with its
   descriptions as comments. TypeScript: interfaces, unions and string literals. Python:
   `TypedDict`s (the functional form where a key isn't a Python name, `from` and `x-openrocket`),
   `Literal`s and `Union`s; an object written inline in the schema takes a name from where it sits
   (`PartNoseCone`, `PositionTop`). A document stays plain JSON data; written back by
   `JSON.stringify` or `json.dumps`, it reads in hpr as the same design, tested on the 18 public
   documents, though numbers may be spelled differently.
   Only the current version gets types; an older document goes through `hpr convert`.
3. **A reader in each**, `readDesign` and `read_design`, checks the format and version first, then
   the document against a copy of the schema embedded without its prose. It checks what the schema
   says, no more: the rules only hpr's reader checks (ADR-112's source-file names, base64) stay
   unchecked (one gap runs the other way: #253). Before the schema, both refuse what hpr's JSON
   reader refuses: a number too large for a 64-bit float, a lone UTF-16 surrogate, nesting 128
   levels deep, `NaN`. Python also refuses a repeated key and `2.0` for a whole number, as hpr
   does; JavaScript's `JSON.parse` can't see either, so TypeScript takes them, and refuses a
   `uint` of 2^53 or more, which it would round. The generator also refuses a schema the readers
   would read differently: a boolean schema, a constraint beside a `$ref`, union or `const`, and a
   `pattern` with a class such as `\d` (Python's `$` is translated to `\Z`).
4. **Where.** `schema/format/typescript/hpr-design.ts` and `schema/format/python/hpr_design.py`,
   unversioned, beside an example that reads documents (`read-design.ts`, `read_design.py`). They
   are not published to npm or PyPI: that is Neer's call
   ([rule 7](../../CONTRIBUTING.md#7-releases-are-the-maintainers)).
5. **How they are checked.** xtask's tests run both examples, under Node.js (22.18 or later runs
   TypeScript as it is) and Python 3.11 or later, on the 17 public designs' documents and the
   migrated 0.1 fixture, and on 4,892 mutations of two of them (the schema refuses 4,114), where
   each reader takes a document exactly when the `jsonschema` crate does; edge cases are held to
   `hpr_format::read_json`. A missing interpreter fails the tests rather than
   skipping them; CI's test job installs Node.js 24 and Python 3.12 on all three systems.
   `cargo xtask format --typecheck` writes the 18 documents as literals of type `DesignFile` and
   checks them, and both examples, with TypeScript 7.0.2's `tsc --strict --target es2020` and mypy
   2.3.1's `--strict`, fetched by `npx`
   and `uvx`, and that each refuses a misspelt tag; CI runs it in its own `types` job, on Linux,
   and the gate as an extra step, `types`, since it needs the network the first time.

**Consequences.** `cargo test` now needs Node.js and Python on the path. A schema change regenerates
both files with `cargo xtask format`; one that adds a keyword needs the generator and both reader
templates taught it first.
