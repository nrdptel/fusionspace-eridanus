# ADR-225: The page anatomy split again: the notes first (2026-10-10)

- **Status:** accepted; splits M0.9d5 ([ADR-223][adr-223]'s rest of #384) in two; adds M0.9d7
- **Summary:** M0.9d5 took the six parts of #384 that change what each page is made of. It now covers one: every quote block a page writes is drawn as the product system's note (`.fs-note`), its signal word in a strip above the message. Pages keep writing Markdown that GitHub renders: a trust note opening with `**How far to trust it.**`, or a GitHub alert, `[!NOTE]`, `[!CAUTION]` or `[!WARNING]`. After mdBook builds the site, `cargo xtask site` rewrites each as the system's specimen writes a note and fails any other quote block; its page check fails a quote block, or a note whose signal word, strip, fill or border is off the system's, each with a canary. The new M0.9d7 keeps the old done-when whole (#384 closed, every rule held) and takes the rest: title blocks, sheets, the intro, the lockup header with no menu button from 720 px, and every page opening with how far to trust it.

[adr-223]: 0223-the-pages-frame-first.md

**Context.** ADR-223 left M0.9d5 six parts: title blocks on every page, sheets with their rail,
the intro (designation tag, status chip, lead), the lockup header with no menu button from
720 px (which needs the sidebar kept open by script, its `aria-hidden` and link focus too),
the system's notes, and every page opening with how far to trust it. Each needs a way to change
the markup mdBook writes, and sheets need the long model pages cut to six `h2` sections or fewer.
One cycle holds one of them with its checks, not six.

The notes go first: they are self-contained, and every part after them reuses the way they are
drawn. Every quote block on the site's 62 pages is a note already: 50 trust notes and one
caution-like paragraph on the landing page, which becomes a `[!NOTE]` alert.

**Decision.**

1. **The source stays Markdown.** A note is written as a quote block that GitHub renders on its
   own: a trust note, its shape held by the source check `trust_notes` (#379), or a GitHub
   alert. mdBook 0.5 renders alerts with GitHub's title and icon; the system's signal words
   (ANSI Z535: DANGER, WARNING, CAUTION, NOTICE, NOTE) and GitHub's alerts share three:
   NOTE, CAUTION and WARNING. TIP and IMPORTANT fail the build; DANGER and NOTICE wait for a page
   that needs them.
2. **The build draws them.** `xtask site` already writes into the built pages (the landing
   page's date of issue). After mdBook it rewrites each note in every page of the guide, the
   print page included, as `<aside class="fs-note" data-kind="…">` with the specimen's head (its
   icon and the signal word, "How far to trust it" for a trust note, whose bold label leaves the
   message) and body, and fails any quote block left over, at any depth: a page quotes a source
   in running text. `mdbook serve` shows the old quote blocks; the published site is what
   `xtask site` builds. A preprocessor would rewrite the Markdown instead, at the cost of a
   second program mdBook runs on every build; the rewrite is one pass over the HTML, tested on
   mdBook 0.5.4's output.
3. **The stylesheet and the page check.** `theme/hpr.css` draws the system's `.fs-note` in the
   theme's roles: a 1 px rule-strong border on the surface, the head in the `label` token (Cascadia
   Mono 12/16 semibold, capitals) over a 1 px rule, a caution's head in the caution fill and a
   warning's in the danger fill with a 2 px border; the box stops at the prose measure. The page
   check fails, at every width, a `blockquote` in `main` and a note that isn't a head then a
   body, whose head says another kind's word, isn't set in the `label` token, isn't set apart
   by a rule or a fill, isn't across the note's top above its message, misses its kind's fill,
   or whose border is missing or thinner than its kind's. Each of those rules has a canary that
   breaks it and nothing else (nine in all, a caution's fill and a warning's apart), and one
   with a note of each kind passes. A note is an `aside` with `role="note"`, so a page with
   several doesn't gain an unnamed landmark for each.
4. **M0.9d7: the rest of #384,** with the old done-when unchanged, after M0.9d6.

**Consequences.** 51 notes on the site's pages are drawn as the system's: `xtask site` counts
103 drawn, adding the print page's 51 copies and the landing page's note a second time in
`index.html`, mdBook's copy of the landing page. The trust note's source shape is unchanged, so writers keep `writing.md`'s rule; the
landing page's note now reads NOTE on GitHub too. The page check reads every note at 41 widths;
its run took 193 s against about 191 s before.
