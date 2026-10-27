# ADR-296: What the words leave: the figures' digits and the command line's words, M0.9g (2026-10-27)

- **Status:** accepted; adds [M0.9g][m0-9g] under [M0.9][m0-9], after [M0.9f][m0-9f] ([ADR-295][adr-295]) is done
- **Summary:** M0.9f holds the site's pages and both READMEs to #476's writing rules, and `hpr sim`'s warnings to 40 words a sentence. Ten rows of the design audit named M0.9f for gaps its *done when* doesn't cover: figures quoted to more digits than are known (#966), and the command line's and the plot's words, which no check reads for dates, joined names, colons or table heads (#973). M0.9g holds both, queued after M0.13.

[m0-9]: ../decisions-and-roadmap.md#m0-9
[m0-9f]: ../decisions-and-roadmap.md#m0-9f
[m0-9g]: ../decisions-and-roadmap.md#m0-9g
[adr-284]: 0284-the-maintainers-own-voice.md
[adr-295]: 0295-the-rest-of-the-words-in-four.md

**Context.** [ADR-295][adr-295] split M0.9f in four. Its last increment, M0.9f4, joins each
figure to its unit on the site and in the READMEs, closes #476 and carries M0.9f's own *done
when*. The design audit (`docs/research/design-conformance.md`) needs every row that is not met to
name its issue and an open milestone that will meet it, and `cargo test -p xtask` checks both.
Ten rows named #476 and M0.9f. Once M0.9f is done, what each row leaves is:

- **The figures' digits:** *README · Status*, *principles.md · 3*, *Mechanics*, *Numbers*, *Before
  release* and *web.md · Documentation sites*. M0.9f cut the figure #476 names (35.350931 m is
  35.4 m), but many pages still quote five or more decimals, and no check reads precision. #966
  holds it, with a *done when* of its own.
- **The command line's and the plot's words:** *On screens* (ISO dates), *Names* (joined names),
  *Voice* (stacked colons) and *Tables* (column heads). M0.9f's checks read the site and the
  READMEs; only the 40-word limit reads `hpr`'s output. #973 holds these.

**Decision.**

1. **M0.9g, the figures' digits and the command line's words,** is done when #966 and #973 are
   closed: a check of the site and both READMEs fails a figure quoted to more digits than its
   kind is known to, and they pass it; a test fails an ISO date in prose, an en dash between two
   words, and a sentence with two or more colons in `hpr`'s messages and the plot's words; a test
   fails a column head without a label and a unit in a table `hpr` prints; the command line and
   the plot pass them; and the audit's rows name the checks.
2. **Its place** is after M0.13 in the queue and before the accuracy milestones M2.3c2 and
   M2.3c3. The maintainer asked on October 18, 2026 that the words and the sites' awareness stay
   ahead of those two. It doesn't go first: M0.9f, the words [ADR-284][adr-284] queued, is done,
   and what is left is smaller. It can split, as M0.9f did, if a survey finds it bigger than a
   session.
3. **The ten rows** name #966, #973 or both, with M0.9g, in place of #476 and M0.9f. Their
   status stays "not met".

**Consequences.** M0.9f is done with its *done when* met. The rules it leaves keep an issue and a
queued milestone, as the audit requires, rather than a parent no queue line reaches.
