# ADR-339: M0.17, the docs site at the tool level, queued second (2026-11-11)

- **Status:** accepted; adds [M0.17][m0-17] to the roadmap from issue [#1364][i1364], in two increments, [M0.17a][m0-17a] and [M0.17b][m0-17b], and puts it second in the [queue][queue], after [M2.3d][m2-3d], twelve public flights, and before [M7.5][m7-5], the bias every simulator shares, which amends [ADR-322][adr-322]'s order for this one milestone.
- **Summary:** The product system the docs site is pinned to now has three levels: front pages, tools and records. A docs site is a tool: it keeps what the drawing means (line types, trust notes, signal colors, units) and leaves off the drawing's frame. The site still wears the whole record: a designation tag over each title, sheet numbers and a title block at the foot of every page. Issue #1364 asks for the move, and it is a milestone's work, not an issue's. This record adds it as M0.17 in two increments, the frame off and then a fourth color theme, Field, and queues it second, where the maintainer asked for it.

[m0-17]: ../decisions-and-roadmap.md#m0-17
[m0-17a]: ../decisions-and-roadmap.md#m0-17a
[m0-17b]: ../decisions-and-roadmap.md#m0-17b
[m2-3d]: ../decisions-and-roadmap.md#m2-3d
[m7-5]: ../decisions-and-roadmap.md#m7-5
[m0-13]: ../decisions-and-roadmap.md#m0-13
[m0-9g]: ../decisions-and-roadmap.md#m0-9g
[queue]: ../ROADMAP.md#queue
[i1364]: https://github.com/nrdptel/fusionspace-eridanus/issues/1364
[design12]: https://github.com/nrdptel/fusionspace-design/pull/12
[adr-164]: 0164-the-fusionspace-product-system.md
[adr-223]: 0223-the-pages-frame-first.md
[adr-231]: 0231-the-page-anatomy-the-drawing-first.md
[adr-235]: 0235-every-page-opens-with-trust.md
[adr-282]: 0282-the-project-tests-each-ui-milestone-itself.md
[adr-284]: 0284-the-maintainers-own-voice.md
[adr-322]: 0322-the-2026-11-05-progress-review-accuracy-before-reader-facing-work.md

**Context.** The docs site follows the FusionSpace product system ([ADR-164][adr-164]) at a
pinned revision. [A change to that system][design12], merged on 2026-11-10, gives it three
levels. A record is a drawing and keeps the whole sheet. A front page and a tool keep the
drawing's meaning and leave off its frame. The docs site is a tool, and today it is drawn as a
record ([ADR-223][adr-223], [ADR-231][adr-231], [ADR-235][adr-235]).

[#1364][i1364] lists what moving it takes: the pin (the revision of the system that `theme/`
copies), the fonts and a new wide cut of the typeface, the frame taken out of what `cargo xtask
site` draws, a one-line footer, labels in sentence case, a fourth color theme, and a decision
record for the parts of four earlier records it changes. That is more than an
issue's fix: it restyles a whole surface and needs its own acceptance pass
([ADR-282][adr-282]).

[ADR-322][adr-322] put accuracy against real flights ahead of reader-facing work while the
project has no outside users, and held [M0.13][m0-13] and [M0.9g][m0-9g] behind it. The
maintainer asked for this milestone second all the same.

**Decision.**

1. **M0.17, the docs site at the tool level, in two increments.** M0.17 is met when both are.
   - **M0.17a, the frame off.** *Done when:* `theme/` is re-pinned to the merge commit of
     [the three-levels change][design12]: the stylesheet, the fonts with the new wide cut and
     its `@font-face` rule, the icons and the check script, with `theme/README.md`'s table and
     `THIRD-PARTY-NOTICES.md` naming the new revision. `cargo xtask site` and `theme/hpr.css` no
     longer draw the designation tag or the sheet numbers, and a page's parts keep their 2 px
     rule and their name. The foot of each page is a one-line footer: the mark and "FusionSpace
     HPR", the version, the page's date and a link to the page about the project. The print
     cover keeps the title block. Navigation labels, note heads, status chips and the theme
     switch's buttons are in sentence case at the sizes the system's level table gives, and
     CAUTION, WARNING and DANGER keep their capitals, in the wide cut. A decision record
     supersedes the parts of [ADR-164][adr-164], [ADR-223][adr-223], [ADR-231][adr-231] and
     [ADR-235][adr-235] it changes. `cargo xtask site` passes its page check at every width in
     light and dark, and `cargo xtask voice` and `cargo xtask figures` pass.
   - **M0.17b, Field.** *Done when:* the theme switch offers Auto, Light, Dark and Field, where
     Field is mdBook's `rust` theme as the re-pinned stylesheet defines it. `cargo xtask site`
     passes its page check at every width in light, dark and field. The acceptance pass
     ([ADR-282][adr-282]) is in the pull request and covers the whole restyle, M0.17a's with
     it.
2. **Out of scope,** as [#1364][i1364] has it: the figures `hpr sim --plot` draws and anything
   exported as a report stay records and keep the title block, and the command line doesn't
   change.
3. **Second in the queue,** after [M2.3d][m2-3d] and before [M7.5][m7-5]. The rest of
   [ADR-322][adr-322]'s order stands, and so does its hold on M0.13, awareness and care on
   the sites, and M0.9g, the figures' digits and the command line's words.
4. **[#1364][i1364] closes with M0.17b.**

**Consequences.**

- One milestone of reader-facing work runs before the next accuracy item. M7.5 and the items
  after it each start one place later.
- M0.17 restyles a surface that exists. It adds no new surface, so the rule that a known wrong
  number in shipped physics blocks new user-facing surfaces doesn't hold it.
- Moving the pin reopens the design audit ([ADR-284][adr-284]). M0.17a's own record says which
  of the audit's rows it reads again.

**How it was checked.** Nothing is built by this record. The site draws the designation tag,
the sheet numbers and the title block today, the theme switch has no Field, and `theme/` is at
the earlier pin, so both *done when*s fail today. `cargo test -p xtask` checks that the queue's
ids are open in the roadmap.
