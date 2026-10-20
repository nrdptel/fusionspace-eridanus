# What makes technical docs read as one person wrote them

**In short: a page reads as a person wrote it through what it claims, not its pronouns.** The well-regarded solo and
small-team docs measured here state limits plainly, own mistakes in a sentence and date their numbers; SQLite's pages
do it with no genuine "I" in 22,675 words (§2). First person follows the kind of page: one author writes about 3 "I"
per 100 words on a blog and 0 in the project's reference pages. Contractions mostly separate the registers (1.0–3.5
per 100 words on most personal pages, 0–0.25 on reference pages); sentence length does not (means of 10.8–22.2
words on reference pages, 14.2–35.2 on personal ones). The phrase lists that style guides share (puffery, inflated
vocabulary, buzz verbs, "please note", recaps, "in order to") get 1 hit ("magic bytes") on this repo's 81 doc pages and
none in `docs/physics/`, so a check can fail on them today, with technical words such as "deploy" (210 hits,
parachutes) kept off the list (§1). Structural patterns (bold-label items, long sentences, triplets) are common here
and the sources disagree on them: count, don't fail. Readers (§3) favor plain wording (judges and lawyers chose it
80–86% of the time) and second person in short lessons (transfer d = 0.54 pooled, fading past 35 minutes); they can't
tell generated from human prose (50–52%) and wrongly take "I" as a human sign. **How far to trust it:** repo counts
are a rough script over prose at 8097f08e, exemplar counts a script over every sentence of 28 pages from nine
self-selected sources; no study found measures voice in engineering or software reference docs, so §3's transfer here
is **Inference**.

This builds on [design-conformance](design-conformance.md) (the voice rule, not met) and [the writing
guide](../writing.md).

## 1. The published catalogs of stock prose

Ten outside sources were read: Wikipedia's "Words to watch" [1] and its page on signs of generated writing [2],
Google's [3] and Microsoft's [4] style guides, GOV.UK's [5], the Federal Plain Language Guidelines [10], Orwell [11]
and three studies of excess vocabulary [6, 7, 8]. 11 of the 17 rows below rest on two or more of them. Counts are hits
over the 81 pages of `docs/**/*.md` outside `decisions/` and `research/`, then the 24 `docs/physics/` pages; prose
only.

| Pattern, example phrases | Sources | All | Physics | Class |
|---|---|---:|---:|---|
| Puffery: seamless, powerful, revolutionary, cutting-edge, world-class | 1, 2, 9 | 1 | 0 | fail |
| Inflation: testament, pivotal, tapestry, "plays a key role" | 2, 6, 7 | 0 | 0 | fail |
| Excess vocabulary: delve, showcase, garner, boast, intricate, realm | 2, 6, 7, 8 | 0 | 0 | fail |
| Buzz verbs: leverage, utilize, facilitate, empower, streamline | 2, 3, 4, 5 | 0 | 0 | fail |
| "it's important to note", "it should be noted", "please note" | 1, 2, 3 | 0 | 0 | fail |
| Vague attribution: "research has shown", "experts declare", "experts argue" | 1, 2 | 0 | 0 | fail |
| Wordy: in order to, a number of, is able to, at this point in time | 3, 4, 5, 10 | 0 | 0 | fail |
| Recaps: In summary, In conclusion | 2 | 0 | 0 | fail |
| Chat residue: "I hope this helps", "let me know", "Certainly!" | 2 | 0 | 0 | fail |
| Same lists, technical senses: robust, key, critical, deploy, impact | 2, 5 | 297 | 77 | reader |
| Minimizers: simply, just, easy, easily, quickly | 3, 4 | 107 | 42 | count |
| Signposts: "This section covers", "In this section", "Let's" | 3 | 25 | 4 | count |
| Bold-label list items | 2 (3, 4 allow) | 1,860 | 897 | count |
| Sentences over 25 words | 5, 10 | ~35% | ~34% | count |
| Hedges: may, might, could; stacked (may ... possibly) | 2 | 321; 0 | 84; 0 | count |
| Negative parallelism ("not just X but Y"), rule of three, "serves as" | 2 | 914 | 238 | reader |
| Hidden verbs: "-tion of", "-ment of" | 5, 10, 11 | 319 | 112 | reader |

- **What fails cleanly.** The phrase rows marked *fail* hit nothing here but "magic bytes". Hold exceptions for "magic
  bytes", "magic number" and "seamless tube". GOV.UK's 37-entry "Words to avoid" list itself allows the technical
  senses: robust "(unless talking about a sturdy object)", deploy "(unless it's military or software)", key "(unless
  it unlocks something)" [5]. So "robust", "key", "critical", "deploy" and "impact" stay off the fail list. "In order
  to" is a plain-words rule, not a tell: Wikipedia lists it among signs of human writing [2].
- **What only counts.** Bold-label items are 39% of the 4,828 list items here and 52% on physics pages. Wikipedia
  calls the bold-header-and-colon item a sign of generated text [2], but Microsoft's term lists "use sentence case and
  bold" [4] and Google documents run-in headings [3]. Signposts conflict with the guidelines' own advice to tell
  readers "what they're going to read about" (p. 63) [10]; 47 of the 67 "In short" hits are the house's trust box and
  must be exempt. The 321 single hedges carry the honesty rule's limits; never fail them.
- **The word lists drift.** Wikipedia dates its sets: the 2023 one (delve, tapestry, pivotal) gives way to a mid-2026
  one with ordinary words ("practical", "steady", "clearer", "matters") that would fail plain technical prose [2]. It
  now files the em dash under historical signs [2]; the design system bans it regardless [9].
- **The vocabulary shift is measured.** Over 15 million PubMed abstracts, 2024 against a trend from 2021–2022,
  "delves" ran r = 28.0 times its expected frequency, "underscores" 13.8, "showcasing" 10.7; 379 style words were in
  excess, 66% of them verbs; at least 13.5% of 2024 abstracts were processed this way [6, pp. 1–3]. Juzek and Ward
  find "delves" rising from 0.21 to 14.38 per million words, 2020 to 2024 [8, Table 2].
- **Where sources disagree.** Google recommends "don't" because readers can "miss the word not" [3]; GOV.UK says
  avoid it [5]. Wikipedia lists "Additionally" [2]; the guidelines say connectives are more often missing (p. 64) [10].

## 2. What personal-voice technical docs do

Twenty-eight pages from SQLite, curl, Daniel Stenberg's blog, Julia Evans, Dan Luu, Simon Willison (blog and Datasette
docs), Beej's guide, the Lua manual and the Rust Book were counted by a script, every sentence, prose only (code
blocks dropped, inline code kept) [12–21]. Rates are per 100 prose words.

| Page | Kind | Words | Words a sentence, mean | I | you | Contractions |
|---|---|---:|---:|---:|---:|---:|
| SQLite, appropriate uses [12] | guidance | 2,407 | 21.8 | 0 | 0.25 | 0 |
| SQLite, how it is tested [12] | claims | 7,003 | 19.0 | 0 | 0.01 | 0 |
| curl man page [13] | reference | 40,725 | 13.1 | ≈0 | 0.72 | 0 |
| Stenberg, the 2026 curl team [14] | blog | 656 | 14.2 | 1.37 | 0.46 | 0.30 |
| Evans, git config options [15] | blog | 2,178 | 17.0 | 3.03 | 1.70 | 2.11 |
| Luu, thoughts on writing [16] | essay | 9,854 | 25.4 | 4.01 | 0.99 | 2.91 |
| Willison, sqlite-utils 4.0 [17] | release | 1,825 | 21.0 | 2.74 | 0.49 | 1.10 |
| Datasette, SQL queries [18] | docs | 1,669 | 17.5 | 0 | 2.70 | 0.24 |
| Beej's guide, intro [19] | guide | 2,002 | 16.0 | 2.60 | 3.65 | 2.35 |
| Lua 5.4 manual [20] | reference | 40,855 | 22.2 | ≈0 | 0.36 | 0 |
| Rust Book, ownership [21] | tutorial | 3,674 | 17.3 | 0 | 1.31 | 1.77 |

The ≈0 rows hold only code ("-I", "%I"), checked hit by hit. The 17 other pages reach wider: 5.64 "I" and 3.46
contractions (an Evans essay), 4.64 "you" (an Evans how-to), 35.2 words a sentence (Luu), 10.8 (curl's README).

- **Limits stated plainly, about the product.** "SQLite does not compete with client/server databases. SQLite competes
  with fopen()." [12]. The testing page says "the developers" 11 times and "I" never.
- **Mistakes owned in one sentence, then the consequence.** "In hindsight, we should not have tried to make SQLite
  accept MySQL 3.x syntax" and "Due to an historical oversight, the column values of a PRIMARY KEY are allowed to be
  NULL." [12, quirks]. Four sentences on that page admit a past error.
- **Numbers dated and qualified.** "As of version 3.42.0 (2023-05-16), the SQLite library consists of approximately
  155.8 KSLOC of C code" [12, testing]; "The 100K hits/day figure is a conservative estimate, not a hard upper bound."
  [12, appropriate uses]. Who wrote a page and when is said once, at the top [16].
- **Doubt said where it is real.** Hedges ("I think", "probably", "not sure") run 2.2–11.9 per 1,000 words on Evans's
  pages and 0–0.14 on SQLite's; reference pages state rules [12, 15].
- **Lists are headed paragraphs or one-line facts.** SQLite's uses page puts 96% of its words in 21 items averaging
  110 words; Evans's git page 31% in 64 items averaging 10.6 words; Luu's essays keep 82–91% in paragraphs [12, 15,
  16]. None restate a paragraph as bullets.
- **Reference pages open with the definition.** All 7 reference or docs pages open with a definition or limit in
  sentence 1 ("Datasette treats SQLite database files as read-only and immutable." [18]); 3 of 10 posts do.
- **Where "I" goes (Inference from the table).** None on syntax, option, CLI or API pages, where the program is the
  subject and "you" gives instructions. Tutorials use "you" (1.3–4.6 per 100) and a classroom "we". Rationale pages
  use a rare "we" or "the developers" (at most 0.35). Prefaces, essays, release notes and reports use "I" for who
  measured or decided, when, what is unfinished and how to report a mistake: "This book is never finished. I intend to
  keep working on it." [13]. The exceptions: Stenberg's posts are personal at 0.15–0.30 contractions, and the Rust
  Book reaches 1.77 with no "I".

## 3. What the evidence says about readers

- **Second person helps short lessons, for transfer, not recall.** Mayer et al. changed "the" to "your" in 12 places
  of a narrated lesson of about 60 s; transfer rose d = 0.65, 1.07 and 0.72 across three experiments (n = 62, 27, 32;
  p = .034, .019, .037), recall did not [22, pp. 392–393]. The meta-analysis of Ginns, Martin and Marsh pools transfer
  d = 0.54 and retention d = 0.30, with "small, non-significant effects" in lessons over 35 min [23] (abstract only);
  their summary gives 25 and 30 studies and 3,312 students [24]. It can reverse: in a diabetes lesson for 139 students
  the formal version won on transfer, F(1, 134) = 4.90, p = .029, ηp² = .035, and the authors offer "your type 1
  diabetes", addressed to readers without it, as an explanation still to test [25].
- **Plain language has the broadest support.** Judges in three states chose plain paragraphs in 85%, 86% and 82% of
  answers, lawyers in 80%, over 1,176 replies [26]. In a memo study 262 naval officers answered more questions right
  and spent 17% less time reading (23% for one group) with the bottom line first, headings and first- and
  second-person pronouns [27, pp. 27–28].
- **Promotional wording costs usability.** On one site with 51 users, usability rose 58% for concise text, 47% for
  scannable, 27% for objective (no marketing claims) and 124% for all three; for objective alone only satisfaction was
  significant [28].
- **Easy wording doesn't lower trust; visible sources raise it a little.** With 1,144 lay readers analyzed, easiness
  had no effect on trust (all p > .343); a "scientific" register with references, method and a neutral tone raised
  perceived expertise (β = .512, R² = .037) [29].
- **First person is a weak, fakeable signal.** In experiments with 4,600 people, readers told generated from human
  self-descriptions apart 50–52% of the time in the three main ones, and took first-person pronouns and contractions
  as human signs, wrongly; repetition, nonsense and grammar errors pointed the right way [30]. A "generated" label cut
  a combined rating of perceived accuracy and sharing by 0.17 on a 6-point scale (95% CI −0.29 to −0.04), about a
  third of a "false" label [31].
- **Developer surveys don't ask about voice.** 93% of 5,500 respondents had met incomplete or outdated docs, a problem
  the survey calls pervasive, and nearly a quarter read English less than "very well" [32].

## What this suggests

- **The fail list for `xtask voice`:** the nine *fail* rows of §1, each with a test, pinned to the stable lists, not
  the newest dated set; exceptions "magic bytes", "magic number", "seamless tube"; expected false alarms today: 0.
- **Count, don't fail:** minimizers, signposts (with the "In short" box exempt), bold-label items, sentences over 25
  words, single and stacked hedges; report contractions and first person per page kind.
- **Flag "I" on reference pages** (CLI, format and model reference); expect almost no false alarms: 90,933 reference
  words in §2 hold no genuine "I". Don't require "I" anywhere, and don't fail long sentences.
- **Voice rules for the writing guide, with their sources:** state a limit about the program plainly; own a past
  mistake in a sentence and give its consequence; date or version a number that can go stale and say how conservative
  it is; hedge only at the claim; "you" in tutorials and worked examples [23], with no role the reader lacks [25];
  reference pages neutral and sourced [29]; no promotional words [28].

## Open questions

- No study found varies voice in engineering or software reference docs and measures trust or task success. This is
  the gap that matters most, and it is unbounded.
- Unread: Ginns, Martin and Marsh's moderators, and Mayer's later tally (only in snippets). The 35% over 25 words is
  from a rough splitter; the nine exemplars are self-selected, with no stock-prose set measured the same way.

## Sources (all read 2026-10-18)

1. Wikipedia, "Words to watch", https://en.wikipedia.org/wiki/Wikipedia:Manual_of_Style/Words_to_watch
2. Wikipedia, signs of generated writing, https://en.wikipedia.org/wiki/Wikipedia:Signs_of_AI_writing
3. Google developer documentation style guide (tone, contractions, word list), https://developers.google.com/style/
4. Microsoft Writing Style Guide (voice, simple words, lists), https://learn.microsoft.com/en-us/style-guide/
5. GOV.UK, A to Z style guide ("Words to avoid") and "Use clear language".
   https://guidance.publishing.service.gov.uk/writing-to-gov-uk-standards/style-guides/a-to-z-style-guide/
6. Kobak, González-Márquez, Horvát, Lause, Science Advances 11(27), 2025. https://arxiv.org/abs/2406.07016
7. Liang et al., the increasing share of processed text in scientific papers, 2024. https://arxiv.org/abs/2404.01268
8. Juzek and Ward, why "delve" spread, COLING 2025. https://arxiv.org/abs/2412.11385
9. FusionSpace design system, `product/writing.md` (the project's own design repository).
10. PLAIN, Federal Plain Language Guidelines, Rev. 1, May 2011.
    https://web.archive.org/web/2024/https://www.plainlanguage.gov/media/FederalPLGuidelines.pdf
11. G. Orwell, "Politics and the English Language", 1946.
    https://orwellfoundation.com/the-orwell-foundation/orwell/essays-and-other-works/politics-and-the-english-language
12. SQLite: whentouse, testing, quirks, different, lang_select and lang_createtable (.html), at https://sqlite.org/
13. curl man page and README, https://curl.se/docs/manpage.html; Everything curl, https://everything.curl.dev/
14. D. Stenberg, two posts of 2026-10-12 and 10-13, https://daniel.haxx.se/blog/2026/10/
15. J. Evans, three posts, https://jvns.ca/blog/ (2024/02/16, 2020/11/15, 2022/01/11)
16. D. Luu, https://danluu.com/writing-non-advice/, /cgroup-throttling/ and /simple-architectures/
17. S. Willison, sqlite-utils 4.0 and Python 3.15 posts, https://simonwillison.net/2026/; sqlite-utils README
18. Datasette docs, getting_started, sql_queries, cli-reference, https://docs.datasette.io/en/stable/
19. B. Hall, Beej's Guide to Network Programming, intro, https://beej.us/guide/bgnet/html/split/intro.html
20. Lua 5.4 Reference Manual, https://www.lua.org/manual/5.4/manual.html
21. The Rust Programming Language, 4.1, https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
22. Mayer, Fennell, Farmer, Campbell, J. Educ. Psych. 96:389–395, 2004. doi:10.1037/0022-0663.96.2.389
23. Ginns, Martin, Marsh, Educ. Psych. Rev. 25:445–472, 2013. doi:10.1007/s10648-013-9228-0 (abstract)
24. Ginns, Marsh, Martin, Teacher magazine, 2015.
    https://www.teachermagazine.com/au_en/articles/engaging-students-through-a-conversational-writing-style
25. Almeida, Münzer, Kühl, J. Comput. Assist. Learn. 40:2248–2260, 2024. doi:10.1111/jcal.13026
26. Kimble, Prokop, Mich. Bar J., May 1990. https://www.michbar.org/file/generalinfo/plainenglish/pdfs/90_may.pdf
27. Kimble, 6 Scribes J. Legal Writing 1, 1996–97.
    https://scribes.org/wp-content/uploads/2022/12/Scribes_vol6_04_Writing_for_Dollars.pdf
28. Morkes, Nielsen, 1997. https://www.nngroup.com/articles/concise-scannable-and-objective-how-to-write-for-the-web/
29. Jonas, Kerwer, Chasiotis, Rosman, Public Underst. Sci. 33:37–57, 2024. doi:10.1177/09636625231176377
30. Jakesch, Hancock, Naaman, PNAS 120:e2208839120, 2023. doi:10.1073/pnas.2208839120
31. Altay, Gilardi, PNAS Nexus 3(10):pgae403, 2024. doi:10.1093/pnasnexus/pgae403
32. GitHub Open Source Survey 2017, https://opensourcesurvey.org/2017/
