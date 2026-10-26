# ADR-295: The rest of the words in four increments, joined names and long warnings first (2026-10-26)

- **Status:** accepted; splits [M0.9f][m0-9f] into four increments, M0.9f1 to M0.9f4, and plans every remaining increment, the last with M0.9f's own *done when*
- **Summary:** M0.9f holds the site, the README and `hpr sim`'s warnings to seven writing rules that #476 lists. Fixing them all is several sessions' work, and two of the rules add bytes to a search index with 71 bytes to spare. So it goes in four increments: joined names and long warnings first, then long notes and table heads, which make room in the index, then grouped figures and prose dates, and last the no-break space between a figure and its unit, joined in the built pages after mdBook writes its index.

[m0-9f]: ../decisions-and-roadmap.md#m0-9f
[adr-239]: 0239-the-words-to-the-design.md
[adr-292]: 0292-the-programs-words-in-four.md

**Context.** M0.9f is done when "#476 is closed: a check of the site and the README fails on an
ungrouped figure of four or more digits in prose (the identifiers it passes listed), an ISO date
in running prose, an en dash between names, a number split from its unit by a breaking space, and
a lead note or table cell over 120 words; number columns' heads give a label and a unit; a test
fails on an `hpr sim` warning sentence over 40 words; each over-precise figure #476 names is cut
to what is known; the audit's rows name the checks." [ADR-239][adr-239] §6 queued it.

A survey of main (c82d8e4a) on 2026-10-26 read the 73 pages the site's contents list and the
README, with code, link addresses and HTML tags left out. It counted:

- **En dashes between words:** 58, all joined names or joined words (`Dormand–Prince` 15 times,
  `Prandtl–Meyer` 12, `fin–fin` 5), none a range.
- **Notes and table cells over 120 words:** 56 notes, each a whole quote block, which the site
  draws as a note, and 71 table cells. The longest note runs about 1,200 words, the longest cell
  481.
- **Number columns' heads:** 45 of the 511 right-aligned columns are headed by a code identifier
  (`apogee_agl_m`), 40 of them on the accuracy page.
- **Figures of four or more digits, ungrouped:** 757 that look like years, 162 of them the
  year of an ISO date, and 245 others, 58 of them in table cells. Among the others are
  identifiers a check must pass: NTRS report numbers, NFPA standards, NACA and NASA report
  numbers.
- **ISO dates outside tables:** 117. Some are in link addresses the survey's rough cut kept.
- **A figure and its unit joined by a plain space:** up to 6,344. The count is high: 737 of
  them are a figure before the word "in", which is not always inches.

These are rough counts from a script, not the checks; each increment's check gives the real one.
Of `hpr sim`'s warnings, the issue warnings are composed at run time, so the message census
([ADR-292][adr-292]) reads only their pieces. Two can pass 40 words in one sentence: the near-calm
wind's first sentence (about 43 words with four spans) and the body-damping station's when a
flight has more than one join.

The search index, the file behind the site's search box, is 71 bytes under mdBook's 10 MB warning.
The site check fails a build past that warning; #414 is the issue for a search that grows with the
site.
A grouping comma adds a byte, and "October 4, 2026" five more than `2026-10-04`. A no-break space
(U+00A0) is two bytes in UTF-8 against a space's one, so 6,344 of them in the pages' text would
add up to 6.3 KB.

**Decision.**

1. **The increments:**

   | Increment | What | Size on main |
   |---|---|---|
   | M0.9f1 Joined names and long warnings | an en dash between words fails on the site and in the README; a test holds `hpr sim`'s warning sentences to 40 words | 58 en dashes; two issue warnings |
   | M0.9f2 Short notes and cells, labeled heads | a note or table cell over 120 words fails; a number column's head gives a label and a unit | 56 notes, 71 cells; 45 heads |
   | M0.9f3 Grouped figures, prose dates and known digits | an ungrouped figure of four or more digits in prose fails, the identifiers it passes listed; so does an ISO date in running prose; the over-precise figures #476 names are cut | 245 figures besides years; 117 dates |
   | M0.9f4 Units held to their figures | a figure split from its unit by a breaking space fails on the built site and in the README; #476 closed; the audit's rows | 6,344 pairs |

2. **The order follows the index.** M0.9f1 takes bytes out: a hyphen is one byte, an en dash
   three. M0.9f2 cuts long notes and cells, which makes room for M0.9f3's commas and dates. M0.9f4
   costs the index nothing, as §3 says. An increment that still crosses the warning trims its own
   words until #414 lands, as M0.14c5 did (#902): it stated one fact once instead of three times.
3. **The no-break space is joined in the built pages.** After mdBook builds the site, `cargo
   xtask site` already rewrites its pages (the notes, `notes::draw`). M0.9f4 adds a pass there that
   joins each figure to its unit with U+00A0 in the pages' text, never in code. The search index,
   which mdBook has written by then, keeps the plain space, so search reads the same. The check
   reads the built pages, so it also fails a pair the pass missed, such as a figure and a unit in
   different tags. The README is shown by GitHub and crates.io, which run no such pass, so it gets
   the no-break space in its source.
4. **What each increment's check reads.** Each check reads every page the site's contents list
   and the two READMEs, the project's and the Python package's. Like the minus-sign check
   ([ADR-239][adr-239] §3), it reads the text as it renders, never code. The API reference, which
   rustdoc builds from the crates' doc comments, is left out, as it is from the minus-sign check;
   its comments still hold joined names (`Dormand–Prince`, `Gauss–Kronrod`).
   M0.9f1's en-dash rule fails a dash with a word of two or more letters on each side, since
   `writing.md` keeps the en dash for ranges. A range of figures (`0.3–0.5`) or of single letters
   (`H–O`, motor classes) passes. The joined names take a hyphen (`Dormand-Prince`). The site's
   pages are read as mdBook renders them, with smart punctuation, which shows a `--` as an en
   dash: OpenRocket's examples *Pods-\-powered with recovery deployment* and *Pods-\-airframes
   and winglets*, 60 times on the site, reached readers as `Pods–powered`. Their names escape one
   hyphen (`Pods-\-powered`), which the site and GitHub show as the file names' two. The READMEs
   are read as GitHub shows them, `--` as it stands. An equation ends a run of text, so an en
   dash between two equations has no word beside it for the check to see; the one on the frames
   page (`$x_B$–$y_B$`) takes a hyphen by hand.
5. **M0.9f1's tests.** The 40-word limit holds each sentence of every issue warning `hpr sim` can
   print, built at its longest: every known issue, each with the detail that makes it longest
   (four spans of near-calm wind; joins on four sustainers; an offset that isn't a number). It
   holds every envelope flag's message too. It also holds every note, reader warning, flag and
   issue warning that `hpr sim --json` prints for 16 of the 18 committed designs under
   `validation/fixtures/ork/`. Each flies on the first of its own motor, an I377-CT and a
   131G84-10A that takes it off the rail at 15 m/s or more before apogee, and the test fails if
   one doesn't. `level-1.ork` also flies on the I377-CT past Mach 1, where the supersonic drag
   issues are raised, and a probe flies a motor fetched from ThrustCurve.org. The other two
   designs refuse a motor of one's own (one has two stages, the other an airframe not read as
   written), and offline their own motors have no curve. The tests split sentences at a period,
   question mark or exclamation mark followed by a space. A decimal point has no space after it,
   so `Mach 0.85` stays whole. M0.9f1's *done when* names "the committed designs it flies
   offline" for this reason: `hpr sim` refuses those two with any motor it can fly offline, so it
   prints no warning for them to hold.

**Consequences.** The audit's rows that cite #476 stay "not met" until M0.9f4, and each increment
names its check in its rows. M0.9f4 closes #476 and carries M0.9f's own *done when*.
