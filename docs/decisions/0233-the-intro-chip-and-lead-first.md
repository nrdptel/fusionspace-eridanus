# ADR-233: The pages' own words split again: the intro's chip and lead first (2026-10-11)

- **Status:** accepted; splits M0.9d11 ([ADR-231][adr-231]'s rest of #384) in two; adds M0.9d12
- **Summary:** M0.9d11 took the rest of #384: the intro's status chip and one-sentence lead, two to six sheets a page, and every page opening with how far to trust it. M0.9d11 now covers the intro: after mdBook builds the site, `cargo xtask site` sets the title block's status (IN PREPARATION or RELEASED, the product system's drawing words) in the system's chip beside the designation tag, and makes the paragraph under every page's title its lead, which a file check holds to one sentence of at most 25 words; the page check fails a chip or a lead drawn off the system's, with canaries. The title block's version and status, which still read "not yet released" after 0.1.0 shipped on 2026-10-08, now say what the site describes. The new M0.9d12 keeps the old done-when whole (#384 closed, every rule held) and takes the sheets and the trust openings.

[adr-231]: 0231-the-page-anatomy-the-drawing-first.md

**Context.** `web.md`, *Page anatomy*, opens a page with "the designation tag (chamfered), a
status chip, the title (Cascadia Mono 28) and one sentence in `lead`"; `principles.md` gives
status in drawing words, IN PREPARATION, RELEASED and WITHDRAWN, and `web.md` names the first two
for a tool. ADR-231 left M0.9d11 three parts. Measured on `main` at 8969df4c, they differ in size:
the chip and the lead are one line of text and one rule for each of 62 pages, while 39 pages
have more than six `h2` sections (the glossary 187) and need restructuring, and the trust
openings rewrite each page's first lines. One increment holds the intro; the sheets and the
openings are a second.

**Decision.**

1. **One status for the site.** The chip shows the title block's *Status* entry, the word before
   its colon, on every page, as each sheet of a drawing set carries the set's status. A
   per-page status would need a judgment on each page of what the last release holds, and the
   site is built from `main`. RELEASED is the system's `ok` chip, filled, with the circle icon
   for normal; IN PREPARATION is `info`, outlined in the action color, with the square for
   information (`foundations.md`, *Icons*: status icons differ in shape). Work in preparation
   is no hazard, so it takes no caution color. A status with no chip fails the build.
2. **The title block says what the site is.** Its version reads 0.1.0, released 2026-10-08, the
   site built from `main`, which may be ahead of it; its status reads IN PREPARATION: the
   release after 0.1.0. Every page's chip says so.
3. **The lead is the first paragraph.** The build makes the paragraph right under each title the
   lead; a title followed by anything else fails the build. The file check fails a lead that
   doesn't end in a period, holds a period, question mark or exclamation mark followed by a
   space (a second sentence, or an abbreviation such as "U.S."), or runs past 25 words, where
   `docs/writing.md`'s rule 2 flags a sentence for a second look: the lead, read first on every
   page, is held to it. The *In short* check on model pages accepts that one paragraph between
   the title and *In short*. The print page's cover and mdBook's 404 page get leads too; the
   404 page's source is now `docs/404.md`.
4. **The page check** fails, at every width, a chip not in Cascadia Mono 12 at weight 600 in
   capitals, under 24 px tall, with an icon off 14 px, off its state's colors, before its tag or
   not above the title; and a lead not in Archivo 20/28 at weight 400, not in the muted ink of
   the page's theme, or not right under its title. Twelve canaries fail one each; a RELEASED
   chip and an IN PREPARATION chip drawn as the system's pass.
5. **M0.9d12, the pages' sheets and trust,** keeps M0.9d11's old done-when unchanged: #384
   closed, each rule it lists held by `xtask site`'s page check or a site test. It takes two to
   six sheets a page and every page opening with what works and how far to trust it.

**Consequences.** Every page shows IN PREPARATION until a release builds the site with the
title block's status set to RELEASED. A lead can't name an abbreviation with a period in it.
The leads say what each page covers and claim no accuracy; how far to trust a page stays in its
*In short* and trust notes until M0.9d12 opens every page with it.
