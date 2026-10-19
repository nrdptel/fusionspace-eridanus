# Line length on screen: does 60–75 characters read better than 95–110?

This note asks what the on-screen studies say about a measure of 60–75 characters a line against 95–110, the range
the docs site's prose runs at (89–106 characters, [wide-window margins](wide-window-margins.md) §1). It checks how
[ADR-250](../decisions/0250-the-column-and-the-reading-scale.md) (the column and the reading scale) states the two
studies it leans on, then reads every study found that varies line length on a screen, from 1998 on. It builds on
wide-window margins §3 and doesn't repeat its print guidance. **Inference** marks this note's reasoning. All sources
were read on 2026-10-19.

**In short.** No study measures a comprehension difference between 60–75 and 95–110 characters.
- **Three tests at or around those lengths found none, and none could rule out a small one.** Dyson & Kipping: 70
  against 100 on a weak check. Shaikh: 75 against 95, 20 readers, paged news. McMullin et al.: 55 against 115, 58
  readers, 61% against 60% correct (§3).
- **The one comprehension test that found an effect favored the shorter line, at 55 against 100.** Dyson &
  Haselgrove: 36 readers, scrolled magazine text; about 0.16 on a scale 1.57 wide from chance to perfect (§3).
  Nothing between 55 and 100 was tested.
- **The long line's speed edge is small and comes and goes:** 5.5% at 95 against 75 characters; about 6% faster
  word search at 100 against 70; no gain in three other comparisons (§4).
- **Readers preferred a medium or shorter line over the longest in four of the five studies that asked**, most
  strongly after looking things up on a site (η² = 0.224). In the fifth they split between the extremes (§5).
- **For looking things up, the task closest to reading docs, 55 to 100 characters made no difference** to time or
  pages visited (99 readers, §6).
- **ADR-250 misstates one finding and leans on a weak test.** Shaikh's readers had no significant preference;
  Dyson & Kipping's comprehension test was mainly a check that people had read. It omits Dyson & Haselgrove (§1).

## 1. What ADR-250 cites, checked

- **"100 characters against 25" [DK]: holds, narrowly.** In Experiment 1 it was the only significant pair of six
  lengths (PDF pp. 9–10). In Experiment 2 it held only when scrolled (pp. 20–21, 25).
- **"Comprehension unchanged" [DK]: holds, on a weak test.** Readers ticked whether each of five questions could be
  answered from the text, a check that they had read it (PDF pp. 4, 8); line length had no effect (p. 14). Dyson's
  review calls it a check on comprehension, not a task built to detect differences ([DY] p. 383).
- **"95 the fastest of 35 to 95" [SH]: holds.** 95 read faster than every other length, F(3, 57) = 3.45, p = .02,
  η² = .15, with no effect on comprehension.
- **"Readers in both studies said they found shorter lines easier": wrong for [SH].** Its readers had no
  significant preference: 30% liked 35 best and 30% liked 95. In [DK], 55 was rated easiest and 100 hardest, but
  25 came fifth of six (PDF p. 14), so "medium" fits better than "shorter".

ADR-250 omits [DH], the same lab's follow-up with a fuller test: 55 cpl was understood better than 100 (p. 598).
Dyson & Kipping call their own speed effect relatively small and not entirely reliable across designs (PDF p. 26).

## 2. The studies

"cpl" is characters per line. Comp. is comprehension; n.s. is not significant at p < .05.

| Study | cpl | Text and task | Moving | n | Comp. | Speed |
|---|---|---|---|---:|---|---|
| [DK] 1, 1998 | 25–100, six | 800-word MSN articles | scroll or page | 48 | check, n.s. | 100 > 25 |
| [DK] 2, 1998 | 25, 55, 100 | the same, gray surround | both | 24 | check, n.s. | 100 > 25 scrolled |
| [MV] 2002 | 55, 115 | 200-word facts, 2 min | not stated | 58 | 61% vs 60%, n.s. | not measured |
| [DH] 2001 | 25, 55, 100 | ≤1,000-word magazine | arrow-key scroll | 36 | 55 > 100 | normal: 55 > 25 |
| [BF] 2002 | 45, 76, 132 | 1,028-word encyclopedia | scroll | 20 | not measured | n.s. |
| [SH] 2005 | 35, 55, 75, 95 | 375-word news | page | 20 | n.s. | 95 > others |
| [BR] 2005 | wide, narrow | instructional pages | unread | unread | narrow better | narrow faster |
| [LV] 1, 2006 | 55, 70, 85, 100 | find a word on a page | none | 72 | not measured | 85, 100 faster |
| [LV] 2, 2006 | 55, 70, 85, 100 | answer questions on a site | none | 99 | not measured | n.s. |
| [RB] 2017 | 22, 44, 66, 88 | Spanish, eye tracking | slides | 92 | filter only | n.s. (fixations) |

[DK], [MV], [DH] and [BF] predate 2004 but test the same question. [BR] was read in abstract only. [RB] (46 readers with
dyslexia, 46 without) kept only sessions with every question right, so comprehension was a filter, not an outcome
(manuscript pp. 1, 7). [DK], [DH], [SH] and [LV] set 10 pt Arial or 12 pt Times; [DK] and [LV] on 800×600 screens.

## 3. Comprehension

- **[DH], 55 > 100.** A three-level effect, F(2, 68) = 3.17, p < .05; only 55 against 100 is significant
  (Newman–Keuls, p. 598). Partial η² = 0.085, computed here from F as 6.34 / (6.34 + 68). Measured by pixel from
  Fig. 2 (±0.01), on their transformed scale (chance 1.57, perfect 3.14): 2.16, 2.29 and 2.14 for 25, 55 and 100
  cpl at normal speed, 1.81, 1.80 and 1.71 at fast speed. So 100 cpl lost 0.16 at normal speed and 0.09 fast
  (unrounded readings): 10% and 5–6% of the chance-to-perfect span. Speed and line length didn't interact; a combined
  test rules out a speed-for-accuracy trade (p. 606). The authors blame the sweep back to a long line's start (p. 605).
- **[SH], 75 against 95: no effect** on a six-type, 19-question test like [DH]'s. No means or spread are given, so
  the size it could have missed is unbounded.
- **[MV], 55 against 115: 61% (SE 1.8) against 60% (SE 1.9) correct**, five multiple-choice questions a passage.
  **Inference:** taking the means as independent, the 95% interval on the difference is about −4 to +6 points;
  each reader did both, so correlated scores would narrow it. A narrow-line gain above 6 points is unlikely there.
- **[DK], 25 to 100 in six steps: no effect** on a check built to confirm reading (§1).
- **[BR], wide against narrow: narrow retained more** in a post-test, but narrow-format readers abandoned the ends
  of long paragraphs more. Widths and sizes are behind a paywall.

So the comprehension evidence is two positive results (55 against 100, and [BR]'s of unknown widths) and three nulls
of low power. **Inference:** the site's 89–106 sits in [DH]'s "100" condition; 70–75 lies between their 55 and 100
and was never compared in [DH]'s design. [SH] found 75 and 95 alike, but gives no means, and [DK]'s 70 against 100
used a weak check. Whether 70 keeps 55's advantage is unknown.

## 4. Speed

Long lines read faster in [SH] and [LV] 1, and in [DK] against 25 cpl only. They were no faster in [DH] at normal
speed, [BF] or [LV] 2; [RB] measured fixation length only, not speed:
- [SH]: 178.8 words a minute at 95 against 169.4 at 75, 5.5% faster; 55 and 35 read at 167.
- [LV] 1, finding a word: 85 and 100 beat 55 and 70, F(3, 210) = 13.10, η² = 0.038 (p. 398). From 70 to 100 the
  mean fell from 1,805 to 1,699 ms (Arial) and 1,718 to 1,609 ms (Times), about 6% (Table 1, p. 399). Accuracy at
  rejecting an absent word was best at 70, significant only against 85, η² = 0.004.
- [DK] 1: 100 beat 25, no other pair. [DK] 2: 100 beat 25 only when scrolled, and the gap went once scrolling time
  was taken out (PDF pp. 20–21, 25).
- [DH]: at normal speed 55 beat 25 and 100 didn't beat 55 (p. 599). Read by eye from Fig. 4, 55 and 100 differ by
  about 2% in words a second. At fast speed 55 and 100 both beat 25.
- [BF]: adults took 366, 363 and 370 s at 45, 76 and 132 cpl, p = .88. [LV] 2: no difference in time or pages
  visited (p. 401). [RB]: no difference in fixation length, F(3, 372) = 0.155, p = .927 (p. 13).

**Inference:** where long lines won, scrolling explains part of it. [DK] and [DH] scrolled one line per arrow-key
press, so fewer, longer lines meant less key work. A wheel or trackpad moves many lines at once, so that part of
the edge is probably smaller on today's sites.

## 5. Preference

- [LV]: shorter preferred in both experiments, η² = 0.044 after search and 0.224 after retrieval (pp. 399, 401).
  After retrieval, 70 cpl won 31% and 29% of paired choices, 100 cpl 17% and 11% (Arial, Times; Table 1).
- [DK]: 55 easiest in both experiments, 100 hardest; ratings didn't track speed or comprehension (PDF pp. 14–15,
  24).
- [BF]: adults picked 76 first most often, none 132, and rated 45 easiest to concentrate on. They also rated 132's
  amount of scrolling the best, F(2, 38) = 6.70.
- [RB]: readers without dyslexia preferred 44 of 22–88, χ²(3) = 14.75, p = .002; readers with dyslexia had no
  preference (p. 13).
- [SH]: no significant difference (§1); every reader named 35 or 95 as least liked.

## 6. Looking things up, skimming and scrolling

No study found measures documentation readers. The nearest is [LV] 2: 99 readers answered 40 questions by browsing
five 30-page sites, at 96–98% accuracy, with no effect of line length on time or pages visited. Its authors still
advise 55–70 over 85–100: readers preferred it, and most web text is read rather than skimmed (p. 403). [LV] 1, a
pure word search, favored long lines by about 6%.

[DH]'s fast group was trained to cut its reading time to 70% of normal or less, close to skimming (p. 596). Long
lines kept pace with 55 there, but comprehension at 100 was still lowest (Fig. 2). The authors suggest long lines
help skimming at the expense of comprehension (p. 606).

Scrolling doesn't settle it either way. [DK] found paging and scrolling read at the same rate in Experiment 1. In
[DK] 2 the long-line gain lived only in the scrolled condition (§4). [SH], which found the clearest speed gain,
paged. The scrolled studies moved by arrow key ([DK], [DH]) or don't say how ([BF]).

## 7. How far these carry to this site

- **Characters a line, not width, is the variable to compare**, the review concludes ([DY] p. 377). Every study
  above but [BR] states its lengths that way.
- **The screens were small.** In [DK], 100 cpl of 10 pt type filled an 800×600 screen (PDF p. 3). The site's
  106 cpl line is 904 px of 18 px type. **Inference:** its visual angle and type size differ from every study's,
  and how that moves comprehension is unmeasured.
- **The readers were young.** Mostly students or young adults; [DK]'s ran from under 18 to 64, [SH]'s were
  college students and [LV] 2's averaged 24.
- **Other surveys.** Nanavati & Bias's 2005 review adds no new data [NB]. Wikimedia's width page reports no line
  length test of its own [WM]. NN/g advises 50–75 characters without citing a study [NNG].
- **[SN]** asks a different question: 12.7 against 67.2 characters on two devices, for students with dyslexia.
  The small device's short lines read 27% faster with no loss of comprehension.

## What this suggests

- **Correct ADR-250's rule 1** where it cites [DK] and [SH]: [SH]'s readers had no significant preference, [DK]'s
  comprehension measure was a check, and [DH] found 55 understood better than 100. A wording fix; no number moves.
- **Inference: the evidence doesn't force a narrower measure, and doesn't defend the wide one either.** Going from
  about 100 to about 70 characters would cost at most the 6% speed some studies found, and it matches what most
  readers preferred. Any comprehension gain at 70 is unmeasured. At 55, [DH] found one of 10% of the span from
  chance to perfect, at normal speed.
- **A narrower measure, wide-window margins' open question.** Its rank 2 (notes in the sheet rail) leaves the
  measure as it is. A plain max-width of ~70–80 characters would narrow it without notes, but reopens ADR-250's
  rule 1. The studies don't say whether that is worth the empty space.
- **What the project can use.** Cached papers carry publishers' or authors' copyright: cite only. [SN] is CC0;
  [MV]'s page names both CC BY 4.0 and CC BY-NC 4.0, so cite only. No dataset was found.

## Open questions

- No study tests 60–75 against 95–110 characters with a comprehension measure able to detect a few points.
- [BR]'s widths, sample and effect sizes, and Shaikh & Chaparro's HFES paper itself, are paywalled; only abstracts
  and the Usability News report were read.
- Not read: a 2024 eye-tracking comparison of one, two and three columns (Teixeira et al.,
  doi:10.1007/978-3-031-60107-1_5), and Orton's IBM report that Wikimedia cites.
- Not found: an eye-tracking study of documentation reading that varies line length; a Dyson experiment after 2001.

## Sources

Read 2026-10-19. Cached, cite only, under the gitignored `refs/research/prose-measure-on-screen/`: [DK], [DH], [LV],
[SH], [BF], [MV], [RB], [SN], [NB]. [DY] is cached under `refs/research/wide-window-margins/`.

1. [DK] Dyson & Kipping, "The effects of line length and method of movement on patterns of reading from screen",
   Visible Language 32(2), 150–181, 1998; journals.uc.edu/index.php/vl/article/view/5671 (PDF pages).
2. [DH] Dyson & Haselgrove, "The influence of reading speed and line length on the effectiveness of reading from
   screen", Int. J. Human-Computer Studies 54, 585–612, 2001, doi:10.1006/ijhc.2001.0458; authors' copy at
   psychology.nottingham.ac.uk/staff/mxh/Papers/, via web.archive.org (capture of 2023-03-30).
3. [DY] Dyson, "How physical text layout affects reading from screen", Behav. Inf. Technol. 23(6), 377–393, 2004.
4. [SH] Shaikh, "The effects of line length on reading online news", Usability News 7(2), 2005;
   psychology.wichita.edu/surl/usabilitynews/72/LineLength.asp via web.archive.org (2017-09-18). Its HFES 49th
   Annual Meeting paper, Shaikh & Chaparro, 2005, doi:10.1177/154193120504900514: abstract only (portfolio.erau.edu).
5. [LV] Ling & van Schaik, "The influence of font type and line length on visual search and information retrieval
   in web pages", Int. J. Human-Computer Studies 64(5), 395–404, 2006, doi:10.1016/j.ijhcs.2005.08.015; copy at
   makinggood.ac.nz/media/1266/ling_2006_fonts.pdf.
6. [MV] McMullin, Varnhagen, Heng & Apedoe, "Effects of surrounding information and line length on text
   comprehension from the Web", Canadian J. Learning and Technology 28(1), 2002, doi:10.21432/T27607;
   cjlt.ca/index.php/cjlt/article/view/26584 (HTML, Method and Results).
7. [BF] Bernard, Fernandez & Hull, "The effects of line length on children and adults' online reading
   performance", Usability News 4(2), 2002; psychology.wichita.edu/surl/usabilitynews/42/text_length.htm, via
   web.archive.org (capture of 2008-04-13).
8. [BR] Beymer, Russell & Orton, "Wide vs. narrow paragraphs: an eye tracking analysis", INTERACT 2005, LNCS 3585,
   741–752, doi:10.1007/11555261_59 (abstract, link.springer.com).
9. [RB] Rello & Baeza-Yates, "How to present more readable text for people with dyslexia", Universal Access in the
   Information Society 16(1), 29–49, 2017, doi:10.1007/s10209-015-0438-8; authors' manuscript at
   repositori.upf.edu/bitstream/10230/47817/1/Rello_UAIS_pres.pdf (manuscript pages cited).
10. [SN] Schneps et al., "Shorter lines facilitate reading in those who struggle", PLoS ONE 8(8), e71161, 2013, CC0.
11. [NB] Nanavati & Bias, "Optimal line length in reading: a literature review", Visible Language 39(2), 2005;
    journals.uc.edu/index.php/vl/article/view/5765.
12. [WM] mediawiki.org/wiki/Reading/Web/Desktop_Improvements/Features/Limiting_content_width.
13. [NNG] Moran, "How chunking helps content processing", NN/g, 2016, nngroup.com/articles/chunking/.
