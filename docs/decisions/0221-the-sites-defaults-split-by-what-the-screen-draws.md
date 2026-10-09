# ADR-221: The site's defaults split: what the screen's stylesheets draw first (2026-10-09)

- **Status:** accepted; splits M0.9d2 ([ADR-208][adr-208]'s M0.9d) in two
- **Summary:** #383 lists seven classes of mdBook default the product system's theme doesn't reach: colors, shape, motion, icons, the head's metas and accessibility, fonts, and print. M0.9d2 now covers the first three as mdBook's stylesheets draw them on screen, each held by `xtask site`'s page check reading every element's computed style in light and navy and with reduced motion set. The new M0.9d4 keeps the old done-when whole (#383 closed, every class held), after M0.9d3, and takes the rest: icons, metas and accessibility, fonts, print (its link color included), and the smooth scroll mdBook's script asks for, which no stylesheet can turn off.

[adr-208]: 0208-the-design-audit.md

**Context.** M0.9d2 was one increment for all of #383. The first three classes share one check, a
computed-style probe in the page check (`xtask/src/site/pages.rs`), and one stylesheet of overrides
(`theme/hpr.css`). The other four each need different work: replacing mdBook's Font Awesome icons
with the system's means templates of its own; the head's metas and the system's `fonts.css` are
file checks; print needs a print stylesheet and a check that reads it. One cycle holds the first
three with their checks and canaries; it doesn't hold all seven.

**Decision.**

1. **M0.9d2: colors, corners and motion on screen.** Every color the site computes is one of the
   system's roles for its theme (`foundations.md`, *Semantic roles*, with the signal fills);
   transparent passes, and a background may be a role seen through, as a dialog's dimmed canvas
   is. No element has a rounded corner or a shadow. Every transition and animation runs at 0, 100,
   160 or 240 ms with the system's `standard` or `exit` easing, none forever, no smooth scroll in
   the stylesheets, and with the stylesheets' reduced-motion rules applied, nothing moves. Each
   page is read in the theme it loads in and in the other, as mdBook's theme menu switches it.
2. **M0.9d4: the rest of #383**, with M0.9d2's old done-when unchanged: icons, the head's metas
   (`theme-color` by scheme, `color-scheme`), forced colors, the focus ring, one `h1`,
   `scroll-padding-top`, contrast, the system's `fonts.css`, print, and the script's smooth
   scroll. It goes after M0.9d3, already queued.
3. **Code is ink on the surface.** mdBook's highlighters color each kind of token; the system has
   no syntax roles, and its docs pattern sets code blocks on the surface (`web.md`). Code is ink,
   comments secondary ink, keywords and titles semibold, a diff's lines danger and normal ink.
4. **Search hits take the action fill**, as a selection does, not the caution fill, which means a
   state. **The current chapter is underlined 2 px**, as `web.md`'s site nav marks the current
   page, not shaded; the page's own headings under it hang from a 1 px rule.

**Consequences.** The page check reads hover and focus styles only as far as the element's own
transition declares them, and a script's styles not at all; M0.9d4's forced-colors and focus work
adds those states. The page check, reading both themes and reduced motion on every page at every
width, ran in 73 s for 66 pages at 41 widths on the maintainer's machine.
