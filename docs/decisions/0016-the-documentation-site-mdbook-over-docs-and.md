# ADR-016: The documentation site: mdBook over `docs/`, and checks for links, labels and equations (2026-09-18)

- **Status:** accepted
- **Summary:** The documentation site: mdBook over `docs/`, and checks for links, labels and equations

**Context.** M0.4 asks for one searchable site that a hobby rocketeer can read (VISION V15, and the
project's rule that documentation is a deliverable). Its first increment, M0.4a, asks for three
things: an ADR that picks the tool and the layout; one source per page, with equations that render
both on the site and on GitHub; and a CI build that fails on a broken link or a bare internal label.
What was in hand on 2026-09-18:

- **The pages.** There are 18 pages under `docs/physics/` and `docs/format/`. They write
  equations in Unicode inside ```` ```text ```` blocks and inline code (`γ_e = GM/(ab)`), with no
  LaTeX anywhere, and they had no Markdown links at all. Code comments, docs and oracles cite
  their paths 135 times. They carried 146 bare labels: `L12`, `ADR-008`, `M1.5b` and the like.
- **The tools.** Versions and licences were read from crates.io on 2026-09-18:
  - mdBook 0.5.4 (MPL-2.0).
  - mdbook-linkcheck 0.7.7 (MIT), which is built against mdBook 0.4 and pulldown-cmark 0.8 and
    does not load in 0.5.
  - lychee 0.24.2 (MIT OR Apache-2.0).
  - pulldown-cmark 0.13.4 (MIT), the Markdown parser mdBook 0.5.4 itself uses.

**Decision.**

- **mdBook 0.5.4 builds the site.** It is a single binary with built-in search. It takes its
  sidebar from a plain `SUMMARY.md`, and its pages are ordinary Markdown with no front matter, so
  GitHub renders the same file. The alternatives each break one of those:
  - Zola needs TOML front matter on every page, which GitHub would show as text.
  - MkDocs needs a Python toolchain and a YAML nav.
  - Docusaurus needs a Node toolchain.

  mdBook is MPL-2.0, and we only run it: it is never linked or ported, and `cargo deny` never
  sees it. Its theme files (MPL-2.0) and the libraries they bundle are copied into the built site
  unchanged, with their licence headers and texts. `THIRD-PARTY-NOTICES.md` lists them. CI pins
  0.5.4, and `cargo xtask site` refuses any other release series. A local build with another 0.5
  release (Homebrew installs the latest) may differ slightly from CI's; `cargo install mdbook
  --version 0.5.4 --locked` matches it exactly.
- **The site's source is `docs/` itself.**
  - `book.toml` sits at the root with `src = "docs"`, and the site builds into the gitignored
    `target/site`.
  - `docs/SUMMARY.md` lists the pages. *Start here* is `docs/start-here.md`, not `README.md`,
    because mdBook turns a link to `README.md` into `README.html`, a file it never writes.
  - `docs/physics/` and `docs/format/` are therefore in the site's source without moving. Each
    page has one source, and the 135 citations of their paths keep working.
  - The working files are not pages: the planning notes, `ROADMAP.md`, `DECISIONS.md` and the
    research notes. The site links to them on GitHub. Whether the decisions and the roadmap become
    pages is M0.4b's call.
- **Equations stay in Unicode**, in ```` ```text ```` blocks and inline code.
  - They render the same on GitHub, on the site and in rustdoc, which renders no LaTeX either.
    Every existing page already writes them this way.
  - mdBook's MathJax doesn't accept the `$` delimiters GitHub uses. `mdbook-katex` would add a
    preprocessor to build and pin, for pages that don't need one.
  - `$x$`, `` $`x`$ ``, `$$` and ```` ```math ```` fail the check, because only GitHub would
    render them. Two prices in a sentence ("$5 and $10") are not math.
    Revisit this if a page needs typeset math.
- **One link rule serves both renderers.** A relative link goes only to another page of the site,
  or to a file under `docs/`, which mdBook copies. Anything else in the repository is linked by
  its `https://github.com/nrdptel/hpr-sim/blob/main/...` URL. A relative link to anything else
  works on GitHub but breaks on the site.
- **The checks are our own**, in `xtask/src/site.rs`, and read pages with `pulldown-cmark`, so
  they see the same links the site renders.
  - **On the sources**, which `cargo test` checks too:
    - every summary entry exists, and every page under `docs/physics/` and `docs/format/` is
      listed;
    - relative links and their `#fragments` resolve, with anchors derived as GitHub derives them;
    - a GitHub URL to `main` names a file in the working tree, and for Markdown, a heading in it;
    - an undefined `[text][ref]` fails, as do bare labels, math only GitHub renders, and a link
      inside a heading.
  - **On the built HTML**, which `cargo xtask site` checks after `mdbook build`: every relative
    `href` and `src` must reach a file, and every fragment an `id`. This catches what mdBook
    rewrites, and any id it derives differently from GitHub. A warning from mdBook fails the build
    too.
  - **Why not lychee.** The link rule needs a check of our own anyway: lychee accepts a relative
    link to `../ROADMAP.md`, because the file exists on disk. Labels need a Markdown parser
    anyway. A Rust check runs in `cargo test` with unit tests that show each failure, offline,
    with nothing to install but mdBook. lychee remains the candidate for fetching external links.
- **External links are counted, not fetched.** A PR's checks must not depend on the network
  ([rule 5](../../CONTRIBUTING.md#5-offline-first), and flaky CI). Today the site has eight:
  RocketPy's and OpenRocket's sites, the three format specs, and three links to this repository's
  issues. Links to files in this repository on GitHub (`blob/`, `tree/` or `raw/main/`) are checked
  offline against the working tree. A scheduled check of external links is tracked in
  [issue #38](https://github.com/nrdptel/hpr-sim/issues/38).
- **Bare labels fail.** `L` or `ADR-` followed by digits, and milestone ids (`M1.5b`, `M2.1b1`),
  fail as whole words outside a link's text: in prose, tables, headings, inline code, raw HTML
  and image alt text. Fenced code blocks are exempt, since they quote files verbatim. A label that
  emphasis splits in two (`**ADR**-008`) is not caught.
  - The fix is a link with a few words of meaning: `[Loft lesson L10][lessons]`,
    `[ADR-009][adr-009] (drag)`, or `[M1.8][roadmap], the supersonic aerodynamics milestone`.
    Reference definitions at the end of the page point into `DECISIONS.md` (by heading anchor),
    `ROADMAP.md` and `research/loft-lessons.md` on GitHub.
  - Some ordinary words look like labels, and are written so they don't: a motor designation in
    full (`L1150R`, not `L1150`, which the lesson pattern would catch); a certification level as
    "Level 2", not "L2"; an appendix equation as "eq. A1.3" is fine, since only `M` starts a
    milestone id.
  - Labels stay out of headings: mdBook wraps each heading in a link, so a link inside one is
    invalid HTML, and the check fails it.
- **CI.** A `site` job on Linux installs mdBook 0.5.4 through `taiki-e/install-action` and runs
  `cargo xtask site` on every PR.

**Consequences.**

- A PR that moves or renames a file a page links to fails until the page follows.
- Milestone and lesson links open the whole roadmap or lessons page, because their entries have no
  heading anchors. The link text names the label, so the browser's search finds it.
- Anchors are GitHub's slugs. mdBook's ids agree with them for every heading on the site today, and
  the built check confirms that on every build. A heading whose two anchors differ fails that
  check, and is reworded.
- The local gate gains `cargo xtask site`, which needs mdBook installed (`brew install mdbook`, or
  `cargo install mdbook --version 0.5.4 --locked`).
- The site is not published until M0.4d, which waits for Neer to turn on GitHub Pages.

---
