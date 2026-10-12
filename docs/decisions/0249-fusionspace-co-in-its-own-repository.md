# ADR-249: fusionspace.co in two increments, a static page in its own repository, checked in Chrome at every width (2026-10-12)

- **Status:** accepted; carries out [ADR-217](0217-the-2026-10-09-fusionspace-co-rebuilt.md) §1 to §4 and §6; splits M9.8 into M9.8a and M9.8b and closes M9.8a
- **Summary:** M9.8, fusionspace.co rebuilt, splits in two, its own *done when* unchanged: the page, its checks and the steps that move the domain (M9.8a), and the maintainer's use (M9.8b, blocked on the maintainer). The page lives in the new public repository `nrdptel/fusionspace-site`, whose first commit is GitHub's own. It is one static page in plain HTML and CSS with one script, served by Cloudflare Pages as the previous site is, with no build step. A check written for it drives Chrome over the DevTools protocol from Node, with no packages. It lays each page out at the 41 widths from 320 to 1,920 px, and both sides of each breakpoint, in the light, dark and field themes, under the site's real security policy. It fails on anything cut off, scrolled, past an edge or its parent's content, overlapping or covered, on text drawn by CSS, on a font or browser error, on a comparative word with no committed comparison linked beside it, and on a file from another site. Pages built to fail prove each detector can fire.

[adr-198]: 0198-the-2026-10-07-project-eridanus.md
[adr-209]: 0209-the-2026-10-08-best-on-the-market-measured.md

**Context.** M9.8's *done when*: "the page in its own repository lists each project and product
with status, guide, docs and release, and the old tools until M9.6 replaces each; a check fails on
clipped text or controls from 320 to 1,920 px in both themes, and on a "better" with no committed
comparison; it passes the trace check from its first commit; Neer has used it; the domain's switch
is written for him."

The maintainer's use can't happen inside a session, and it needs the page first. The rest is one
piece of work. No repository for the site existed. The previous site is a Next.js export in
`nrdptel/fusionspace-landing`, deployed to Cloudflare Pages by a workflow. It registers a service
worker at `/sw.js` and has a page per project at `/projects/<id>`. hpr's own page check
(`xtask site`) is written for mdBook pages and lets tables scroll sideways (#498).

**Decision.**

1. **Two increments.** M9.8 keeps its *done when* word for word, and closes when both have:
   - **M9.8a, the page and its checks:** every part of M9.8's *done when* but the maintainer's
     use.
   - **M9.8b, the maintainer's use,** blocked on the maintainer, as M0.7a6 and M0.8a4 are.
2. **The repository.** The site lives in `nrdptel/fusionspace-site`, public, created with GitHub's
   own first commit (a one-line README), so the trace check passes from that commit. Everything
   after it arrives through pull requests. The trace check's term list stays private: the private
   overlay's `pre-push` hook for the site runs it on every pushed tree, message and branch name,
   and no copy of it is committed to the public repository.
3. **The stack.** Static HTML and CSS, with one script for the theme switch, and no build step:
   `site/` is what the host serves. The host is Cloudflare Pages, which already serves the domain,
   with a project connected to the repository's `main`. The page sets no cookies, loads nothing
   from another site, and sends a content security policy that allows only its own files. It
   installs no service worker. Its job is to send readers to other sites, which need the network
   anyway, and a cache could keep showing a release list that's out of date. It ships a `sw.js`
   that removes the previous site's worker and caches, and redirects the previous site's project
   pages to each tool.
4. **The design.** The design system's stylesheet is copied unchanged (Apache-2.0), with its fonts
   (SIL OFL) and a `fonts.css` of the site's own. The lockup, mark and icons are bundled,
   unchanged, under the brand's terms and listed in the site's notices, as hpr's are. The page is
   `FS · SITE 001` as the system's example sketches it: an intro, sheets and a title block. Its
   registers are tables of three columns, each designation set above its product's name so no
   column is held wide. Below 720 px they reflow into labelled blocks instead of scrolling
   sideways, because a table that scrolls sideways hides its right-hand columns.
5. **What it lists.** FusionSpace HPR's seven products, with the public names [ADR-199](0199-the-2026-10-07-fusionspace-hpr.md) gives
   them: HPR Sim is released at 0.1.0, with its guide, tour, docs, release, PyPI and crates.io.
   The other six are in preparation, each with its planned release, and have no designation
   yet, since each product numbers its own when its first milestone ships ([ADR-198][adr-198]
   §4). Next come the brand system (`FS-ARA`) and its two tools; Loft and Debrief, withdrawn and
   replaced by FusionSpace HPR; and the four web tools at their own addresses. The status words
   are the system's: Released, In preparation, Withdrawn.
6. **The check.** `checks/check.mjs` drives headless Chrome over the DevTools protocol, using
   Node's built-in WebSocket. It sets each width as the device's metrics, which lays the page out
   at 320 px, as a measurement in the page confirmed: the 41 widths 40 px apart, and both sides
   of each breakpoint the stylesheets set. It sets light and dark as the system
   asking for them, and picks field on the page's own switch. It serves the site with the headers
   in `site/_headers`. At each width and theme it fails when:
   - the page is wider than the window;
   - a box that hides or scrolls its overflow has more content than room;
   - text is cut by a clipping ancestor (overflow, a clip-path, paint containment), or SVG text
     runs past its drawing;
   - text or a box (after its clipping ancestors) reaches past the left, right or top edge (or the
     bottom, inside something fixed to the window), or a box reaches past its parent's content
     (into the page's margin, out of a cell);
   - two pieces of text overlap, or something, or nothing, is drawn at a control's centre;
   - CSS draws text (`::before`, `::after`), or a clip-path takes a shape other than `inset()` or
     a corner cut off the top right (the designation tag's, measured against each line of text),
     neither of which can be measured;
   - a font fails to load, or the browser logs an error (a missing file, a request the security
     policy refused, an exception).

   It finds the pages itself (every `.html` under `site/`). Of the 41 pages under
   `checks/canaries/`, 39 are built to fail one way and name it, and a test requires the check to
   find exactly that kind there; the other two (a clean page, and a claim beside its comparison)
   must pass, as the site must.
7. **The "better" rule** ([ADR-209][adr-209] §2), one rule after three rounds of review found
   phrasings a word list missed: a claim is anything that compares. That is any "than" ("smaller
   error than", "more reliable than"), any multiple ("twice as accurate as"), and the words that
   compare without either ("better", "fastest", "most accurate", "outperforms", "lower error").
   A claim passes only in the same block of text as a link to an entry of `comparisons.json`.
   Each entry is a published comparison committed with its inputs and the tools' versions; the
   first is *Compared with other simulators*. In text that can't carry a link (the title, the
   description and social tags, `alt`, `aria-label` and the like) a claim always fails. The rule
   errs strict: a plain comparison ("wider than the window") fails too and is worded another
   way; only "rather than" and "other than" pass. A claim worded with none of these (say "the
   one to trust") is left to review.
8. **Staying current.** `checks/links.mjs` fails on a link out of the site that doesn't answer,
   and on a stated release that isn't the latest on PyPI and crates.io. CI runs it on every pull
   request and weekly, so a new release fails it until the page lists it. The page check and the
   tests run on Linux, macOS and Windows.
9. **The maintainer's steps.** The site's README lists the Cloudflare clicks that move the domain:
   create the project, free the domain from the old project, add it to the new one, check it,
   archive `fusionspace-landing`. The private status file points to them.

**Alternatives considered.**

- **Port hpr's `xtask site` page check.** Rejected: it is Rust built around mdBook's layout, and a
  sibling repository would need the whole workspace to run it. The new check keeps its ideas
  (text rectangles cut by clipping ancestors, the window's edges, overlapping text, canaries) and
  is stricter on sideways scrolling.
- **A static-site generator or a framework.** Rejected: one page needs neither, and each adds a
  build and dependencies to keep current.
- **A service worker for offline reading.** Rejected for now (§3): the page's links need the
  network, and the previous site's worker is the kind of cache that outlives a release list.
- **GitHub Pages, as hpr's docs use.** Rejected: the domain's DNS and the previous site are on
  Cloudflare, so Cloudflare Pages moves it in a few clicks and can switch back the same way.

**Consequences.** The page states facts that change: releases, statuses, links. The link check
catches a release the page misses and a dead link, but not a status that changed without a release.
Each milestone that ships a product, or replaces a web tool (M9.6), updates the site in its own
repository.
