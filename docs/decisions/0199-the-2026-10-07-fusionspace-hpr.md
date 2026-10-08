# ADR-199: Neer's 2026-10-07 public name: FusionSpace HPR, on every product and package (2026-10-07)

- **Status:** accepted; amends [ADR-147, a roadmap of open work](0147-m0-5b-a-roadmap-of-open-work-an-archive-and-a.md) §6 (its budget), [ADR-118, HTTP over the cache](0118-m5-1b-http-over-the-cache.md) (the cache folders), [ADR-198, project Eridanus][adr-198] §3 (external names) and §4 (the stamp's name), and [ADR-162, the suite and its releases][adr-162] §1 (the names)
- **Summary:** the suite's public name is FusionSpace HPR, and each product is "FusionSpace HPR ·" and its role: Sim, Analyzer, Competition Kit, Motor Designer, Recovery Designer, Avionics, with the app and its store listing called FusionSpace HPR. Packages carry the brand: PyPI `fusionspace-hpr` imported as `fusionspace.hpr`, crates `fusionspace-hpr` and `fusionspace-hpr-*`, npm `@fusionspace/hpr`. The command stays `hpr`, and its help, version and banner open with FusionSpace HPR. Stamps read `FusionSpace HPR 0.1.0 · FS-ACHERNAR · SW · TOOL 001`, and files stamped by `hpr-sim` still read. The docs move to `hpr.fusionspace.co`. M10.1d9 renames everything before release 0.1 publishes, because a published name is permanent. Star names stay internal codes.

[adr-162]: 0162-the-suite-and-its-releases.md
[adr-198]: 0198-the-2026-10-07-project-eridanus.md

**Context.** [ADR-198][adr-198] kept the external names `hpr-sim` and `hpr`; this record names the release. On
October 7, 2026 Neer decided FusionSpace should appear in the product names, in more places than not and across every
product under the project ([VISION](../VISION.md), that date). Neer named the public brand FusionSpace HPR.

The research, checked on 2026-10-07 (not legal advice):

- **Registries.** `fusionspace`, `fusionspace-hpr` and the other candidates were free on PyPI, crates.io, npm (the
  `@fusionspace` scope), Homebrew, conda-forge, Scoop, winget and Docker Hub. Taken: the GitHub login `fusionspace`
  (an empty user account from 2018), so `brew tap fusionspace/…` and `ghcr.io/fusionspace` are out of reach; and `hpr`
  on npm, a utility library.
- **PyPI prefixes.** [PEP 752](https://peps.python.org/pep-0752/), accepted on 2026-06-29, lets an organization
  reserve a prefix: only its holder can upload `fusionspace` or `fusionspace-*`. [PEP 755](https://peps.python.org/pep-0755/),
  PyPI's policy for granting one, is still a draft. A package that only holds a name is squatting on both PyPI
  ([PEP 541](https://peps.python.org/pep-0541/)) and crates.io, so nothing is published early to hold a name.
- **Cargo namespaces.** [RFC 3243](https://github.com/rust-lang/rust/issues/122349)'s `fusionspace::hpr` crates aren't
  stabilized, and the RFC expects them for one project split in crates, not a brand's products, so the prefix is
  spelled out.
- **Trademarks.** No FUSIONSPACE mark was found registered or pending in classes 9 or 42; the US and EU registers
  need a check by hand. A Malaysian company publishes ride-hailing apps under the developer name "Fusionspace" in both
  app stores. Autodesk's pending FUSION (serial 99642989) crowds the word "Fusion" in class 9. "hpr-sim" and "HPR"
  are descriptive of high-power rocketry, so the protection comes from the house mark; the star names are arbitrary,
  and two (Cursa, already a running app and read as "curse"; Rana, common) would be weak public names.
- **How others do it.** Brands that want their name at install time prefix every package (`google-cloud-*` imported
  as `google.cloud`, `azure-*`, `adafruit-circuitpython-*`, npm `@sentry/*`). In hobby rocketry, RocketPy and
  OpenRocket are brand and product at once; Featherweight's app is "Featherweight UI".

**Decision.**

1. **The suite is FusionSpace HPR.** Each product's external name is "FusionSpace HPR ·" and its role:

   | Product | External name | Internal (ADR-198) |
   |---|---|---|
   | The simulator: library, CLI, Python | FusionSpace HPR · Sim | Achernar |
   | The app: desktop, web, phones, watch | FusionSpace HPR | Zaurak |
   | The flight analyzer | FusionSpace HPR · Analyzer | Angetenar |
   | The competition kit | FusionSpace HPR · Competition Kit | Acamar |
   | The motor designer | FusionSpace HPR · Motor Designer | Rana |
   | The recovery designer | FusionSpace HPR · Recovery Designer | Cursa |
   | Avionics | FusionSpace HPR · Avionics | Sceptrum |

   The rebuilt web tools (ADR-191, M9.6) become "FusionSpace HPR · Motor Finder", "· Charge", "· Window" and
   "· Muster". In prose after the first mention, "HPR Sim" or "the analyzer" is enough. "FusionSpace" stays one word.
2. **Every package carries the brand.**
   - **PyPI:** the distribution is `fusionspace-hpr`, imported as `fusionspace.hpr`, with `fusionspace` an implicit
     namespace package so later FusionSpace packages can share it, as `google.cloud` does. If maturin can't build that
     wheel, the import is `fusionspace_hpr`, recorded in an ADR with the failure.
   - **crates.io:** the facade crate `hpr` becomes `fusionspace-hpr`, and every published `hpr-*` crate becomes
     `fusionspace-hpr-*`. The folders under `crates/` may keep their names.
   - **npm:** `@fusionspace/hpr` for the WASM build (M4.4) and `@fusionspace/hpr-design-types` for the schema's types.
3. **The command stays `hpr`.** It's short, free on every platform checked, and what the hobby calls itself. Its
   `--help`, `--version` and banner open with FusionSpace HPR. The environment variables stay `HPR_*`.
4. **Everything else a user meets says FusionSpace HPR:**
   - the stamp, `FusionSpace HPR 0.1.0 · FS-ACHERNAR · SW · TOOL 001`, with readers still recognising files stamped
     `hpr-sim …` with either designation, since builds between M10.1d7 and M10.1d9 write the new designation under
     the old name;
   - the docs site's title and the README's heading;
   - a media type for `.hpr` files, defined for the first time as `application/vnd.fusionspace.hpr+json`, with the
     extension and format id unchanged;
   - [ADR-118](0118-m5-1b-http-over-the-cache.md)'s cache folders, renamed: `co.fusionspace.hpr` on macOS,
     `FusionSpace\hpr\cache` on Windows and `fusionspace-hpr` under Linux's XDG folders. The old folders are not read:
     nothing has been published, and a developer's cache refills from the network;
   - the User-Agent, `fusionspace-hpr/<version> (+https://hpr.fusionspace.co)`;
   - the release title, `FusionSpace HPR <version>`, and the release archives' names.
5. **The docs live at `hpr.fusionspace.co`** once Neer points the name at GitHub Pages, so a later repository rename
   breaks nothing. Until then they follow the repository.
6. **The apps and hardware follow the same rule when they come.** App and bundle identifiers are `co.fusionspace.hpr`
   (the watch's adds `.watchkitapp`); the store title is "FusionSpace HPR", with a subtitle saying what it does;
   winget's identifier is `FusionSpace.HPR`. Boards carry the FusionSpace mark and an `FS-SCEPTRUM-00n` part number,
   and USB devices report "FusionSpace" as their maker.
7. **M10.1d9 does the rename before release 0.1's checklist (M10.1d4),** in the same release as M10.1d7 and M10.1d8.
8. **The roadmap's budget rises** from 43,000 to 43,500 bytes, for M10.1d7 to M10.1d9.

**Consequences.**

- The rename touches every crate's manifest and most `use` lines, the Python tests and examples, the docs' code,
  `xtask`'s generated pages and the release workflow. After 0.1 publishes, a rename would mean new packages and dead
  old ones, so it can't wait.
- Neer's optional steps, all his to take: a PyPI organization and an npm organization named FusionSpace (free;
  PyPI's is where a prefix grant would go). A FUSIONSPACE filing in classes 9 and 42 (about $700 at the USPTO; ™ until
  registered). For the App Store to show FusionSpace as the seller, an organization account, which needs a company and
  a D-U-N-S number, before M9.4. A GitHub organization would need another name, as `fusionspace` is taken.
- The old tools' domains redirecting into `hpr.fusionspace.co` stays Neer's call (ADR-191 §1).
