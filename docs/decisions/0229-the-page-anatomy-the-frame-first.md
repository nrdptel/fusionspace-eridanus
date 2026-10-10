# ADR-229: The page anatomy split again: the frame first (2026-10-10)

- **Status:** accepted; splits M0.9d7 ([ADR-225][adr-225]'s rest of #384) in two; adds M0.9d9
- **Summary:** M0.9d7 took the rest of #384: title blocks, sheets, the intro, the lockup header with no menu button from 720 px, the `header` and `footer` landmarks, and every page opening with how far to trust it. M0.9d7 now covers the page's frame, what stands above and below every page's text: after mdBook builds the site, `cargo xtask site` makes the menu bar a `header` that draws the system's one-color horizontal lockup, keeps the sidebar open with no menu button from 720 px, and ends every page with the system's title block in a `footer`, its entries written once at the end of the landing page's source. A file check fails a page without them, and the page check fails a lockup off 24 px or the ink role or without its clear space, a menu button from 720 px, a sidebar a keyboard or screen reader reaches while it is put away (or can't while it is shown), and a title block off the system's shape or not last, each with canaries. The new M0.9d9 keeps the old done-when whole (#384 closed, every rule held) and takes the intro, the sheets and the trust openings.

[adr-225]: 0225-the-page-anatomy-the-notes-first.md

**Context.** ADR-225 left M0.9d7 five parts. Two are the frame mdBook draws around every page's
text, the same on every page: the menu bar, which `web.md` (*Page anatomy*) asks to be the
system's header (the horizontal lockup at 24 px in one color, Void on light and Paper on dark;
navigation with the current page underlined 2 px, which the sidebar has drawn since M0.9d3; no
menu button above 720 px), and the title block, which ends every page in the system and only the
landing page here. Both are landmarks `web.md` (*Accessibility*) asks for, `header` and `footer`.
mdBook shows its sidebar only from 1,080 px and hides it behind the menu button below, so dropping
the button from 720 px means keeping the sidebar open by script there, with its `aria-hidden` and
its links' focus following what is drawn. The other three parts change each page's own text: the
intro wants a designation, a status and a lead for every page, the sheets want the long model
pages cut to six `h2` sections or fewer and a rail beside each, and the trust openings want every
page's first lines rewritten. One increment can hold the frame with its checks and canaries; the
three parts that rewrite the pages are a second.

**Decision.**

1. **M0.9d7: the frame.**
   - The build rewrites the frame, as it rewrites the notes and the chrome
     ([ADR-228][adr-228]): no copy of mdBook's template. It makes the menu bar a `header` and
     draws in place of its title the system's one-color horizontal lockup, from the brand kit's
     `kit/logo/svg/fusion-space-horizontal-void.svg`, copied unchanged into `theme/` and inlined
     with its paths in the text's color, so it is Void on light and Paper on dark as `web.md`
     asks; the product's name, HPR, follows it, and both link to the landing page. mdBook's
     click on its title scrolled to the top; a link to the landing page replaces it.
   - mdBook's script that shows the sidebar from 1,080 px shows it from 720 px, whatever an
     earlier visit left, and `theme/hpr.js` opens it when a window widens past 720 px; the
     stylesheet hides the menu button from 720 px.
   - Every page ends with the system's title block, a `footer` after the page's arrows. Its
     entries are written once, at the end of the landing page's source; the build takes them
     from there and draws the block on every page, the landing page and the print page too,
     adding after the site's title the page's own (its `h1`; the print page's, the book's), as
     ISO 7200 adds a supplementary title to a sheet's. The owner's entry carries the mark, one color, at 24 px, the
     brand's least cluster height.
   - A file check fails a built page without one `header` holding the lockup, one `footer`
     title block after the page's `main` with its fields in order and the page's own title, or
     with mdBook's 1,080 px sidebar script left. The page check fails, at every width:
     a lockup not 24 px tall, not in the ink role, cut by its box or with something inside its
     clear space (0.25 of its height, the brand's rule); a menu button drawn from 720 px; a
     sidebar shown that is `aria-hidden` or whose links the keyboard skips, or put away and not;
     and a title block whose border isn't 2 px of ink, whose entries aren't in Cascadia Mono, or
     with anything below it. Each has a canary.
2. **M0.9d9: the intro, the sheets and the trust openings,** with M0.9d7's old done-when
   unchanged: #384 closed, each rule it lists held by `xtask site`'s page check or a site test.
   It goes after the queued M0.9d8.

**Consequences.** `mdbook serve` still shows mdBook's menu bar and the landing page's title
block as a table; the published site is what `xtask site` builds. From 720 to 1,080 px the
sidebar now takes 300 px beside the page, so the prose column there is narrower than the
68-character measure, as it is on a phone; the reader who wants the room can drag the sidebar's
edge, as mdBook allows. The rewrite is tested on mdBook 0.5.4's output; a newer mdBook that
changes the menu bar, the sidebar script or the page arrows fails the build until the rewrite
learns it. The lockup and the mark are the project owner's brand files, all rights reserved, as
the favicon and the icons are (`THIRD-PARTY-NOTICES.md`).

[adr-228]: 0228-the-sites-defaults-the-chrome-first.md
