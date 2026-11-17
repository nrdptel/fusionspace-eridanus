# What TMT's 2018 delay study measured

Tripoli Motor Testing's (TMT) chair, Alan Whitmore, timed the ejection delays of 40 certified commercial motors and
ran three paired comparisons of delay time between low and high sites, published in the *Tripoli Report* of Summer
2018 [1]. This note records its numbers by page for [motor delay spread](motor-delay-spread.md) and [delay
altitude](delay-altitude-tra-letters.md), which had leaned on forum summaries of it. Read 2026-11-16 from a member's
copy (members-only: numbers cited, no table or passage copied; page numbers are the magazine's printed ones, which
match the PDF's). **Inference** marks this note's arithmetic. Trust the counts as printed; the altitude ratios are
whole percents from small sets, with no temperatures, per-firing sites or (outside the whole-motor sets) lots.

**In short.**
- **12 of 40 firings missed NFPA 1125's delay band, 11 early and 1 late,** from all three makers [1, p. 16].
- **Whole AeroTech motors ran 7.5% to 22% longer at about 5,500 ft** than at 11 or 427 ft [1, pp. 16–17].
- **Delay grains burned alone ran 6.4% and 7.5% longer high** (**Inference**, from 94% and 93%) [1, p. 17].

## 1. The main series

- **How delays were timed.** On TMT's nearly vertical load-cell stand, from thrust falling to 5% of its peak to the
  ejection charge's peak, to 1 ms [1, pp. 15–16].
- **What was fired.** Previously certified motors only, none in certification, bought or given by the makers
  [1, p. 16]. The core comparison was 38 mm H and I white-smoke motors: AeroTech I161W, Cesaroni I175W and Loki
  H144W [1, p. 16]. Table 1 adds other AeroTech H and I types; 30 of its 41 rows are white propellant, fired
  2017-04-02 to 2018-03-10 [1, p. 17].
- **The band.** NFPA 1125 (the US code for making model and high-power motors) §8.2.7(1)(b), as the study quotes it:
  within 1.5 s or 20% of the label, whichever is greater, at most 3 s [1, p. 16].
- **The result.** 12 of 40 outside the band (30%), 11 early and 1 late, "in approximately the same proportion"
  from AeroTech, Cesaroni and Loki [1, p. 16]. The study's Table 1 marks 9 of 25 AeroTech, 2 of 9 Cesaroni and 1 of 7
  Loki (**Inference**, counted [1, p. 17]). It has 41 rows against its printed total of 40, so the share is 29% or
  30%.
- **Spread within a type.** In Table 1 the 12 I161W (14 s) have an SD of 6.3% of their 11.72 s mean, the 5 H219T
  (14 s) 3.4% of 12.24 s (**Inference**, computed [1, p. 17]).
- **Orientation.** Twelve I161W split 4 up, 4 sideways and 4 down missed the band 1, 0 and 3 times; the study calls
  the numbers small [1, pp. 16–17].

## 2. The altitude comparisons

| Comparison | Low site | High site | Low time as a share of high | High longer by (**Inference**) |
|---|---|---|---:|---:|
| Whole motors, H219T | NC, 11 or 427 ft | Cedar City, UT, about 5,500 ft | 88% | 13.6% |
| Whole motors, H283ST | NC, 11 or 427 ft | Cedar City, UT, about 5,500 ft | 82% | 22.0% |
| Whole motors, I161W | NC, 11 or 427 ft | Cedar City, UT, about 5,500 ft | 93% | 7.5% |
| Isolated grains, Oregon | Beaverton, 197 ft | near Brothers, 5,680 ft | 94% | 6.4% |
| Split grains | North Carolina, not given | Montana, not given ("over a mile", [4]) | 93% | 7.5% |

- **Whole motors** [1, pp. 16–17]. Sets of 5 or 6 AeroTech H and I motors from one production run, full-length
  delays, fired at AeroTech's factory in Cedar City and at TMT's Chapel Hill (427 ft) or Bayboro, Pamlico County
  (11 ft) sites. The study doesn't say which NC site or delay label each set had. AeroTech's own stand is horizontal,
  TMT's points up [1, p. 16]; if the Utah sets used AeroTech's, the pairs also differ in orientation.
- **Oregon grains** [1, p. 17]. John Lyngdal of NAR's Standards and Testing committee burned isolated delay grains at
  his home and at the Oregon club's launch site. No count is given.
- **Split grains** [1, p. 17]. In winter 2017–18 Tripoli's president in Montana and Whitmore in North Carolina burned
  sixteen 1.75 in sections of AeroTech delay, cased in motor-delay tubing, divided between the two, timed by
  stopwatch. Neither site's height is given.
- **The study's reading.** A delay in a motor burns first at chamber pressure, then at or below outside pressure, so
  delay times can't be compared across motor types; site altitude "may have a significant effect" [1, p. 17].

## 3. What the forum summaries had

- **A 2021 summary** [2] matched the study on the count (40), the share (30%) and two of the three motor sites
  (5,500 and 11 ft), and like it gave no site per result or temperature. A 2023 post's hedged spring-2020 date [3]
  doesn't match: the study is signed 2018-06-04 [1, p. 17].
- **The Montana burner's own account.** Tripoli's president, whom the study names as the Montana burner, posted from
  Butte that his site was "over a mile MSL" and Whitmore's "about 400' MSL", and that the results are in Whitmore's
  article [4, 5]. He recalled three halved grains, 8%, timed by video; the study has 16 stopwatch-timed sections and
  93%, a time ratio of 1.075, which rounds to 8%. This note takes his test as the study's split-grain one.

## Open questions

- Why whole motors stretched more than the grains (up to three times), and each set's temperatures, label and NC site.
- Per-firing sites for Table 1; whether its 41st row or its total of 40 is the slip.

## Sources

Read 2026-11-16; cached under `refs/papers/` (gitignored), cited, not copied. `trf:` is
`https://www.rocketryforum.com/threads/`.

1. A. Whitmore, "Tripoli Motor Testing investigation: pyrotechnic delay element timing in commercial high power
   rocket motors", *Tripoli Report* 29(3), Summer 2018, pp. 15–17. Page 17's text sits under Table 1 in the PDF;
   text extraction shows it.
2. "BP motor consistency", post #31 (2021-05-02, id `2134103`): `trf:bp-motor-consistency.166006/page-2`
3. "Dual deployment first event: Ejection delay or altitude?", post #9 (2023-11-26, id `2517393`):
   `trf:dual-deployment-first-event-ejection-delay-or-altitude.183395/`
4. "Accuracy Of Aerotech RMS Drillable Motor Delay Times", post #10 (2022-11-08, id `2342354`):
   `trf:accuracy-of-aerotech-rms-drillable-motor-delay-times.175762/`
5. "Understanding effect of burn time on Aerotech motor delays?", post #11 (2021-07-16, id `2158424`):
   `trf:understanding-effect-of-burn-time-on-aerotech-motor-delays.167321/`
