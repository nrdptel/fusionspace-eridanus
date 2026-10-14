# ADR-263: Typeset equations, from the product system's Rev E (2026-10-13)

- **Status:** accepted; supersedes [ADR-016](0016-the-documentation-site-mdbook-over-docs-and.md)'s rule that equations are written in Unicode; moves the product system's pin from Rev D (`d372afa`) to Rev E, as [ADR-164](0164-the-fusionspace-product-system.md) lets a deliberate update do; reopens the design audit at the new pin ([ADR-208](0208-the-design-audit.md)); carries out M0.12
- **Summary:** The site's equations were plain Unicode text, in code blocks and code spans, which reads like a formula typed into a chat. The maintainer asked for equations set as a textbook sets them. Pages now write LaTeX between dollar signs; an mdBook preprocessor in `xtask` turns each formula into MathML when the site builds, with the math-core converter (MIT, pure Rust); the browser draws it in New Computer Modern Math, the face of LaTeX's textbooks, served from the site. No script runs and nothing loads from another server. A formula the converter can't read fails the build, and the page check fails one wider than its box.

**Context.** [ADR-016](0016-the-documentation-site-mdbook-over-docs-and.md) chose Unicode for
equations because it reads the same on GitHub, on the site and in rustdoc, and the site's check
failed any `$x$` as math only GitHub renders. The maintainer, after reading the four guides and the
simulator's guide, asked for traditional typeset math: plain text was readable but not the easiest
to interpret, and he had the textbooks and assignments of a LaTeX-trained engineering student in
mind. He left the call to research.

The research (2026-10-13) found:

- **No controlled study** compares comprehension of typeset and plain-text math. Readers do read
  symbols worse than the same content in prose (Österholm 2006, *Educational Studies in
  Mathematics* 63), which argues for the words-before-equations rule this site already has, not
  for either rendering. Typeset math is the convention of the textbooks and reports the site
  cites, and MathML carries structure a screen reader can walk, which code spans don't.
- **Where readers see source, plain text wins:** Unicode's plain-text math note (UTN #28) and
  rustdoc's own practice. Doc comments, terminal output and code stay Unicode.
- **Options for mdBook 0.5:** its built-in MathJax loads MathJax 2.7 from a CDN, which breaks the
  offline rule; `mdbook-katex` runs KaTeX in a JavaScript engine whose builds its README limits
  to some platforms, and writes HTML without MathML by default; client-side KaTeX, Temml or
  MathJax 4 need a script on every page. math-core 0.8 converts LaTeX to MathML Core in Rust, is
  MIT, and is the converter rustdoc's accepted math RFC builds on
  ([RFC 3958](https://rust-lang.github.io/rfcs/3958-rustdoc-texmath.html)). MathML Core is drawn by
  every current browser (Chrome and Edge since 109).
- **MathML doesn't break a line.** A long equation is written on several lines with `aligned`, as
  print does when a display is wider than the measure.

**Decision.**

1. **Pages write LaTeX:** `$…$` in a sentence, `$$…$$` as its own paragraph for a display
   equation. A symbol a sentence names is math too. `docs/writing.md`, *Equations*, has the
   rules: short inline formulas, slashes for inline fractions, displays that fit a 320 px phone,
   ISO 80000-2 notation, vectors in bold.
2. **The build converts them** (`xtask/src/site/math.rs`): mdBook runs `cargo xtask site-math`
   as a preprocessor, which `cargo xtask site` points at its own running binary. It reads each
   page with pulldown-cmark's math extension, so a dollar sign in code is never math, and replaces
   each formula with math-core's MathML, a display equation in `<div class="fs-equation">`. It
   fails the build, naming the page and line, on a formula math-core can't read or warns about,
   math in a heading, a display equation sharing its paragraph, an arrow accent (`\vec`, which
   Chromium and Safari draw off center), GitHub's code-span math, and a formula that runs into a
   digit (two prices read as math; a price's dollar sign is written `\$`). `--check` does the same
   for named pages, and notes formulas long enough to risk a phone's column.
3. **The product system's Rev E** (`nrdptel/fusionspace-design`): a third face, New Computer
   Modern Math (Book, 8.1.1 from CTAN, under the GUST Font License, the LPPL 1.3c), for equations
   only; the web uses a subset (Latin, Greek, operators, arrows, delimiters, math italic and bold,
   with the math table's size variants), 324 KB, renamed *NewCM Math Subset* as the license asks
   of a changed font, with each prime mapped to its script-size glyph, which Chromium otherwise
   draws raised twice. The subset draws pixel for pixel as the whole font does on a page of
   stretched brackets, radicals, integrals, sums and cases. Math is 1.1 em of the text around it,
   in the text's color; a display is centered with a line's space around it. The face loads only
   on a page with math, and waits for its file (`font-display: block`) rather than drawing a
   formula in a stand-in face. `fusionspace-check.js` gains `equations`, which finds math wider
   than its box.
4. **The page check** knows math: its text must be in the math face, sized as TeX sizes it and
   never under 12 px; a formula's text is measured for overlaps inside the formula's own box,
   and not against itself, since a script sits over its base by design; and a new kind, *a
   formula wider than its box*, runs the system's `equations`, with a canary in a box that scrolls
   sideways, where the window's check can't see it. The ordinary canary page now holds a formula,
   which must pass.
5. **The site's math rule changes:** `$…$` and `$$…$$` pass the source check, which now parses
   them as math; GitHub's two forms the site doesn't read, `` $`x`$ `` and a ```` ```math ````
   block, still fail.
6. **Rustdoc stays Unicode** until rustdoc's own math lands; its pages are read in editors and
   terminals as often as in a browser.

**How far to trust it.**

- **Checked:** every page's formulas convert (`cargo xtask site`); the page check passes at 41
  widths from 320 to 1,920 px with math held to its column, its face and 12 px, in both themes;
  the vitals stay within their bounds on a throttled phone. Each page's converted equations were
  compared with the plain text they replaced, term by term, by a review that read the diff of
  every page (no equation's meaning changed, no number moved); the subset was compared pixel for
  pixel with the whole font on a page of stretched brackets, radicals, integrals, sums and cases.
- **Not checked:** a screen reader reading the MathML aloud; Firefox and Safari (the page check
  runs Chrome; both draw MathML Core, Safari with known small misplacements of some accents);
  the print layout of long displays.

**Consequences.** Every page's equations are drawn as a textbook draws them, on GitHub as on the
site. A page with math downloads the 324 KB face once. Writing a long equation takes more care: it
must fit a phone, on several lines if need be. math-core is pinned exactly (`=0.8.2`); a newer
version is a deliberate update, its output checked by the site's checks. The face's license is
not the SIL Open Font License of the other two faces, a choice made without the maintainer that
the design repository's *Status* lists for his review. The design audit reopens at Rev E: it
changed `foundations.md`'s *Type*, `web.md`'s *Documentation sites* and `README.md`'s *Status*,
which are read again; every other row reviewed at `d372afa` carries over.
