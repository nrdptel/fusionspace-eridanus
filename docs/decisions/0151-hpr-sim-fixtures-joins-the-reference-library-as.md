# ADR-151: hpr-sim-fixtures joins the reference library as private data (2026-10-04)

- **Status:** accepted
- **Summary:** Neer's collection of designs paired with the logs of the same flights is pinned in the reference library and kept private under `refs/`; only aggregate statistics are published; it stays out of the default `.ork` survey until an increment brings it in

**Context.** M2.3c has been blocked since
[ADR-083](0083-m2-3c-blocked-no-private-design-is-the-rocket-of.md): no private design is the
rocket of a logged flight. On 2026-10-04 Neer added `nrdptel/hpr-sim-fixtures`, a private collection
gathered for this project, to be made ready for use without starting work on it. It holds 468
flights in three tiers. In the 89 of tier A, the source itself ties the design as flown to a numeric
barometric log, with the exact commercial motor, the date and the site. The 289 of tier B have one
stated gap, and the 90 of tier C are looser. There are also 98 partial leads. Its 9,556 files are
listed with their SHA-256 in a manifest, and 298 flights carry ERA5 weather for the day.

Three things stood in the way of simply cloning it into `refs/`:

- **Licence.** Its `LICENSING.md` says the files are not for redistribution. Results may be
  published only as aggregate statistics, crediting the sources collectively, and never as
  per-flight data that would let someone rebuild a design. The ERA5 files are Copernicus data used
  under the CDS licence, which requires an attribution on anything derived from them.
- **The `.ork` survey.** `cargo xtask ork` reads every `.ork` under `refs/`. The collection's 396
  designs would join its corpus and move every count `docs/format/ork.md` quotes. Its gitignored
  working area holds 260 more.
- **The local pre-commit check.** It screened commits against `loft-fixtures` and
  `debrief-fixtures` only.

**Decision.**

1. **Pinned.** `validation/refs.lock.toml` holds it as a private git item at commit `31771d8`, with
   its `CHECKSUMS.sha256` manifest. On Neer's Mac, `refs/hpr-sim-fixtures` is a symlink to the
   sibling checkout `../hpr-sim-fixtures`; a local setup step makes it.
2. **Private, on its own terms.** [Rule 4](../../CONTRIBUTING.md#4-private-data-stays-private)
   covers it as it covers the other two private repositories, with this collection's stricter terms.
   Only aggregate statistics are published, with the sources credited collectively. Its flight ids
   carry the rocket's name, so they are not anonymised case ids: a report numbers its own cases.
   Anything derived from its ERA5 files carries the Copernicus attribution and the ERA5 citation
   from its `LICENSING.md`.
3. **Outside the default survey.** The default `cargo xtask ork` corpus leaves out
   `refs/hpr-sim-fixtures`, as it leaves out `refs/scratch`; `--dir refs/hpr-sim-fixtures` reads
   it. A milestone that wants it in a survey brings it in with an increment of its own, together
   with the count refresh.
4. **Screened.** A local pre-commit check refuses the collection's files and names: its files by
   the hashes in its `files.csv`. The exceptions are files saved unchanged from public data services (ThrustCurve's
   curves, Wyoming soundings, ERA5 from the CDS or Earth Data Hub, Open-Meteo), as `files.csv`
   names each file's source. Six of those curves are bundled byte for byte in
   `crates/hpr-motor/data/thrustcurve/curves`. Fliers' own weather logs and teams' motor files
   are screened like the rest. A commit message naming one of its rockets is refused. The
   names come from its tables' rocket names, flight ids and their slugs (148 ids carry a suffix
   after the date), lead ids, and its design files' stems and `.ork` rocket names. The check for
   names already public now runs in two passes: `git grep -l` lists the files that hold any name,
   then Python finds which names those files hold. Both lists are cached for the command.
5. **Not scheduled here.** The queue is unchanged. The collection may unblock M2.3c's design and
   log pairs, give M1.14 real flights to measure against (some of them supersonic), and widen
   M4.5's corpus of designs. Each milestone decides when it uses the collection.

**Evidence.**

- `cargo xtask refs verify hpr-sim-fixtures`: at `31771d8`, tracked files match, 9,556 manifest
  hashes match (about 20 s). `refs fetch` leaves the checkout alone at the pin.
- `cargo xtask ork` with the link in place reads the same sources as before and exits 0.
- `ork::tests::only_the_default_excluded_directories_are_skipped` fails when the excluded name is
  changed (a mutation probe).
- The check refuses a commit message that names one of the collection's rockets or quotes a
  flight id, and a commit of a flier's weather log. All 468 flight ids stay screened after the public-name filter.
  The 51 motor and weather files from fliers and teams are screened, and the six bundled curves
  are not. On the first version's name list the two-pass filter kept the same 846 names as the
  old filter.

**Left open.** The Python oracles' `SKIPPED` set, in
`validation/oracles/rocketserializer/geometry.py`, does not name the collection. With the symlink
they skip it anyway, because `Path.rglob` in their Python 3.11 and 3.12 does not enter a linked
directory (checked: no designs found). A real clone would join their corpus but not the Rust
one. Editing the script changes the hash its records carry and forces a rerun of the OpenRocket
records, so the fix waits for that rerun:
[issue #306](https://github.com/nrdptel/hpr-sim/issues/306).
