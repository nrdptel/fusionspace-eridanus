# ADR-217: Neer's 2026-10-09 plan: fusionspace.co rebuilt from the ground up (2026-10-09)

- **Status:** accepted; amends ADR-147 §6 (the roadmap's budget)
- **Summary:** fusionspace.co, FusionSpace's front page, is rebuilt from the ground up as `FS · SITE 001`, like the old web tools (M9.6) but sooner: M9.8, queued after the simulator's guide (M0.8a) so the page can send people to it. It presents FusionSpace and its projects, FusionSpace HPR's products first, each with its status, its guide, docs and latest release, and the old tools until M9.6 replaces them. It follows fusionspace-design's web rules, passes the no-clipping checks at every width in both themes, makes no claim of "better" without a committed comparison (ADR-209), and lives in its own public repository, clean of tooling traces from its first commit. Pointing the domain at it is the maintainer's step.

[adr-191]: 0191-the-2026-10-06-ideas-web-tools-watches-repo-layout-hardware.md
[adr-209]: 0209-the-2026-10-08-best-on-the-market-measured.md

**Context.** On 2026-10-09 Neer decided that fusionspace.co gets the same treatment as the old web tools: rebuilt
from the ground up rather than patched.

- **Today's site** is a Next.js app in the public `nrdptel/fusionspace-landing` repository, served through Cloudflare.
  It was last changed on 2026-10-03, before FusionSpace HPR, project Eridanus and the design system's current revision
  existed.
- **The design system already sketches it.** fusionspace-design's product kit has an example home page, `FS · SITE
  001`: a drawing register of every tool, its status and where it lives, with the principles beside it
  (`tools/build/kit_product.py`), and `product/web.md` sets the rules every FusionSpace page follows.
- **The page is the front door.** Release 0.1 is published; people arriving from PyPI, crates.io or a forum land on
  fusionspace.co first, and today it doesn't say what FusionSpace HPR is or where to start.

**Decision.**

1. **Rebuilt, not patched.** M9.8 builds fusionspace.co anew as `FS · SITE 001`, on fusionspace-design's web rules and
   tokens, with the stack chosen at the milestone for speed, offline reading and the fewest moving parts.
2. **What it says.** What FusionSpace is; each project and its products, FusionSpace HPR's first, each with its status
   (released, in preparation), its guide, docs and latest release; the old tools (Motor Finder, Charge, Window,
   Muster) until M9.6 replaces each, then a redirect to its replacement.
3. **The standards it meets.** No text, control or figure clipped at any width from 320 to 1,920 px in both themes,
   checked by a tool; [ADR-209][adr-209]'s rule that "better" appears only beside a committed comparison; the design
   system's accessibility and performance rules; the same docs and writing rules as the rest of the project.
4. **Its own repository, clean from the start.** The site lives in a public repository of its own, with no tooling
   traces in files, commits or pull requests, and the same private overlay for its working notes. The old
   `fusionspace-landing` is archived once the new site serves the domain.
5. **When.** M9.8 is queued after M0.8a, the simulator's guide, so the page has a tour to point to. The rebuilt tools
   ([ADR-191][adr-191], M9.6) keep their place after mobile.
6. **The maintainer's steps.** Pointing fusionspace.co at the new site and archiving the old repository are Neer's,
   written down at the milestone with the exact clicks.
7. **The roadmap's budget rises** from 46,500 to 47,500 bytes, for M9.8.
