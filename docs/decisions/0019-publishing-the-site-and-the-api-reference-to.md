# ADR-019: Publishing the site and the API reference to GitHub Pages (2026-09-18)

- **Status:** accepted
- **Summary:** Publishing the site and the API reference to GitHub Pages

**Context.** M0.4d asks for the site to be published from `main`, with the workspace's rustdoc
beside it and each linking the other. GitHub Pages hosts a public repository's site for free, but
turning it on is a repository setting, which only the owner may change
([rule 7](../../CONTRIBUTING.md#7-releases-are-the-maintainers)); until then, a deploy fails.
Rustdoc merges its search index and list of crates with whatever an earlier run left in its
output. With `--no-deps`, cargo documents a workspace's crates in no set
order, and rustdoc links another crate's item only if that crate's pages already exist; otherwise
it leaves the link as text, without a warning. And the crates' documentation is also read outside
the site, in `cargo doc --open` and later perhaps on docs.rs, so it can only link the guide by its
address.

**Decision.**

- **The API reference is part of the site, under `api/`.** `cargo xtask site` builds the rustdoc
  of every library crate (not `xtask`, not the command-line binary), with all features and
  `-D warnings`, and copies it to `target/site/api`. The site is one artifact, and a local build
  has both halves.
- **Built on its own, in order.** The rustdoc build has its own target directory,
  `target/site-rustdoc`, whose documentation is cleared before each build (not its compiled
  dependencies, so a rebuild takes seconds). Crates are documented one at a time, each after the
  workspace crates it depends on, so every link between crates resolves. `api/index.html` sends a
  reader to the guide's page, *The API reference*.
- **They link each other, and the check holds both ends.** *The API reference* links each crate's
  front page by a relative link. Each crate's `//!` documentation links the guide's pages for its
  models by address, `https://nrdptel.github.io/hpr-sim/...`, which the built-site check reads as
  the local build. So a crate the site doesn't link fails, as does a crate that links no page of
  the guide, or a link to a page the guide doesn't have.
- **Every link in the reference is checked**, as the guide's are, and a link to another crate that
  rustdoc left as text fails. Three kinds of link are exempt, each named in the code: the
  implementor scripts rustdoc loads only when they exist, line ranges on its source pages, and
  links inside documentation it copies from a dependency (glam's, for the vector types), which it
  passes on as written when it can't place them.
- **The site knows its path.** Pages serves a project's site under `/hpr-sim/`. `book.toml` sets
  `site-url` to it, so mdBook's 404 page works at any address, and the check resolves
  root-absolute links, links by the site's address and a `<base href>` against it, as a browser
  would.
- **CI deploys from `main`, after every other check.** The `site` job uploads `target/site` as
  the Pages artifact on every run (`actions/upload-pages-artifact@v5`). A `deploy` job publishes
  it (`actions/deploy-pages@v5`) on `main` only, once fmt, clippy, the three test jobs, doc,
  wasm-check, deny and site have passed on that commit. It alone may write to Pages.
- **Until Pages is on, the deploy is skipped, and says so.** On `main`, the `site` job asks
  GitHub's API whether Pages is on and deploys from GitHub Actions. If not (a 404, or another
  source), it prints a warning and the deploy job is skipped, so `main` stays green. Any other
  answer from the API fails the job, so a broken check can't pass for "off". Pull requests don't
  ask, since they never deploy.
- **The README's first lines link the address,** and say the site goes live once Pages is on.

**Alternatives.**

- A workflow of its own for Pages: it would publish a commit whose other checks failed.
- Letting the deploy fail until Pages is on: `main` would be red for a reason outside the code,
  which hides real failures.
- `actions/configure-pages` with `enablement: true`, which turns Pages on from CI: a change to a
  repository setting, which is the owner's call.
- docs.rs for the rustdoc: it needs the crates published on crates.io, also the owner's call.
- Linking the guide relatively from the crates (`../../physics/aero.html`): it works inside the
  site only, not in `cargo doc --open` or on docs.rs.
- One `cargo doc` for all the crates: faster by a few seconds, but its cross-crate links depend on
  which crate cargo happens to document first.
- Not inlining glam's types (`#[doc(no_inline)]`): no copied links to exempt, but `DVec3` in every
  signature would lead nowhere, since glam's own pages aren't built.

**Consequences.**

- The deploy bullet of M0.4d waits for the owner to turn Pages on (the maintainer's call).
  Then the next push to `main` deploys, or `gh workflow run CI --ref main` without one, and the
  README's "goes live once" sentence goes.
- `cargo xtask site` also runs `cargo doc` once per library crate, about 5 s with a warm build and
  10 s from cold.
- A library crate added to the workspace fails the site until *The API reference* links it and its
  documentation links the guide.
