# What NAR's four R&D reports on motor delays measured

[Motor delay spread](motor-delay-spread.md) could not read four NAR R&D reports on ejection delays. This note reads
them: Staab's field flights of composite F motors (NARAM-55, 2013), Team Pod Bay Doors on igniter placement in
composite motors (NARAM-46, 2004), Weaver on delay change with age (NARAM-31, 1989) and Gaff on black-powder motors
after extreme cold (NARAM-26, 1984). Do they move that note's composite scatter, its unsized composite temperature
term or its age line? It serves the settings check (M6.8 in the [roadmap](../ROADMAP.md)), the "delay mismatch"
fault (M7.4) and the optimum-delay output. Read 2026-11-04 from NAR members' copies (cited, not copied; page
numbers are PDF pages), every table row transcribed and its statistics recomputed; no source code read.
**Inference** marks this note's own arithmetic. The comparison is with motor delay spread's §3 table (composite
values mean −0.12 s, SD 0.64 s, n 195; black powder −0.34 s, SD 0.47 s, n 54) and its §7 distribution: center
L(1 − 0.03), σ = √(0.5² + (0.065 L)²) s for a model motor of label L.

**In short.**
- **Composite scatter depends on the motor type.** Staab's 39 field flights of five F-class composite (APCP) types
  average 0.01 s early, SD 1.10 s (**Inference**). Two types of the same impulse and the same 6 s label differ
  by 3.4 times: SD 1.14 s on 16 single-use motors, 0.33 s on 18 reloads. Motor delay spread assumes 0.50 s for
  every type, which is between the two.
- **Field firings break the standard's band more often.** 5 of Staab's 39 flights (12.8%, 95% 4.3% to 27.4%) missed
  NFPA 1125's 1.5 s-or-20% band, the worst 3.15 s early on a 5 s label. Motor delay spread found no clear miss in
  301 NAR certification firings. Its §7 distribution holds 26 of the 39 (67%) in its 90% interval.
- **Older black-powder delays run shorter.** Weaver's Estes C6-7s made in April 1988 averaged 5.50 s, 0.55 s
  shorter than ones made in June 1989 (6.05 s): about −0.47 s per year, early side, with age and lot confounded.
- **Temperature stays unsized for composite delays.** Staab logged air, not motor, temperature, and his steadier
  type bounds the slope only to −0.42% to +0.20% per K (**Inference**). Gaff timed no delays. His black-powder
  motors fired at about −40 °C mostly failed (10 of 12). The ones warmed first all fired normally (12 of 12).

**How far to trust it.** High on what each report printed: every Staab offset equals its row's difference, and
Weaver's 29 per-motor averages all reproduce from their four readings. Moderate on Staab's spread: one flyer and
one set of equipment, timed from liftoff, makers withheld. Low on any size from Pod Bay Doors (one bench pair) or
Gaff (no timer).

## 1. Staab, delay variability of APCP F motors (NARAM-55, 2013)

**What was flown.** About 40 F-class APCP motors (the text says 44, then "over 40"), all near 52 N·s. The report
names each type only by a code: F, the first digit of average thrust, and a letter. A and B are single-use, X, Y
and Z reloads, and the maker is withheld on purpose (pp. 3–4). F3B and F3Z were bought at random from four lots
each and flown in random order (pp. 4, 8). One flyer flew them over several days in Indiana, from a 6 ft rail with
the maker's igniters, in TARC-style rockets about 3 ft long and 2.75 in across (pp. 3–5). Of the seven types
named, five appear in the table. Air temperature was logged for each flight, −2 to 24 °C (p. 9); motor
temperature was not.

**How it was timed.** Two PerfectFlite pNut barometric altimeters recorded ejection (p. 5). The offset is the
altimeter's ejection time less the author's theoretical ejection time (p. 9). That time is near nominal burn plus
label but not equal: 7.8 s for F3B against 1.66 + 6 s, unexplained. Read against burn plus label, F3B's mean
would be +0.04 s and 6 flights, not 5, would miss the band (**Inference**). The clock starts at liftoff, not
burnout, so each motor's burn-time error rides in the offset; certification sheets time from burnout. Ejection
times sit on 0.05 s steps, consistent with a 20 Hz sample [5, p. 11], though that manual is a later edition. The
p. 9 table lists 39 flights. Two runs were dropped, an altimeter failure and a recalled lot that never ejected
(pp. 5–6), which leaves 3 of the stated 44 unaccounted for.

**What it found (Inference, from the p. 9 rows; labels inferred from theoretical ejection less burn).**

| Type | Label | n | Mean offset | SD | Range | Early | Out of NFPA band |
|---|---:|---:|---:|---:|---:|---:|---:|
| F3B, single-use | 6 s | 16 | −0.10 s (−1.7%) | 1.14 s | −2.00 to +1.65 s | 10 | 3 |
| F3Z, reload | 6 s | 18 | +0.54 s (+9.0%) | 0.33 s | −0.10 to +1.05 s | 1 | 0 |
| F4A, single-use | 4 s | 2 | −1.38 s (−34%) | 0.11 s | −1.45 to −1.30 s | 2 | 0 |
| F3X, reload | 5 s | 2 | −2.75 s (−55%) | 0.57 s | −3.15 to −2.35 s | 2 | 2 |
| F2X, reload | 7 s | 1 | −0.20 s (−2.9%) | none | none | 1 | 0 |
| All | | 39 | −0.01 s (−1.2%) | 1.10 s | −3.15 to +1.65 s | 16 | 4 early, 1 late |

- **The spread differs by type.** 95% chi-square bounds on the SD are 0.84 to 1.77 s for F3B and 0.25 to 0.50 s for
  F3Z, which don't overlap. Pooled within-type SD is 0.80 s on 34 degrees of freedom. Type means scatter by 1.30 s.
- **The author's own table is of absolute values.** His p. 7 "Average 15.3 / 8.7, Std. Deviation 10.7 / 5.0" are
  the mean and SD of the absolute percent offset; F3B's reproduces exactly. His F3Z n of 17 leaves out one of two
  1.05 s rows (**Inference**). They are not a bias and a signed spread.
- **Out of band.** The author counts 10 flights outside 20% of the label, 5 of them also outside 1.5 s (p. 5). The
  worst, more than 60% early, cut the shock cord (p. 5). The rows give the same 5 of 39.
- **Against motor delay spread's §7 (Inference).** Its 90% interval holds 26 of 39 (67%), its 95% 29 of 39 (74%).
  It holds 10 of 16 F3B flights at either level, and 15 and 18 of 18 F3Z flights. The note claims 91% and 96% on
  NAR's sheets.
- **Liftoff timing is unlikely to explain the tail.** No source bounds an F motor's burn-time error, but a 3.15 s
  miss on a 5 s delay is longer than any burn time in the table, 1.23 to 1.9 s (**Inference**).
- **The author** found this reload steadier, warned against a blanket statement and found another reload poor (p. 8).

**Temperature (Inference).** The author plotted offset against air temperature and saw "no correlation" (pp. 7–8).
On F3Z's 18 flights (−1 to 24 °C) the ejection time's slope is −0.11% of the label per K, 95% −0.42% to +0.20%.
Its 8 coldest flights (mean 40 °F) ejected 0.13 s later than its 10 warmest (71 °F), 95% −0.22 to +0.48 s. The
cold and warm sets were also flown on different days and likely from different lots, since the nominal time
changes between them. F3B flew only at or below 11 °C and says nothing. The interval contains the black-powder
−0.30% per K that motor delay spread uses, so it neither confirms nor rules that out for composite grains.

## 2. Pod Bay Doors, igniter placement and composite delays (NARAM-46, 2004)

**The claim.** An igniter that lights the delay grain off its center lengthens the delay; centering it shortens and
steadies it (pp. 2, 18–20). The authors advise pushing the igniter fully in, then backing it out slightly (p. 19).
The sign is later ejection only, so it can't explain the early bias motor delay spread measures.

**The model.** The delay grain burns at a boost rate while the motor thrusts and a slower coast rate after. Rim
ignition adds path length (pp. 9–13). With assumed rates of 0.25 in/s and 0.063 in/s, "fairly representative" of
AeroTech (p. 14), Table 6-1 (p. 15) gives the added delay for an F40 lit at the rim:

| Label | 1 s | 4 s | 6 s | 10 s | 15 s |
|---|---:|---:|---:|---:|---:|
| Added delay | +0.85 s | +0.65 s | +0.56 s | +0.44 s | +0.35 s |

- **The table reproduces (Inference).** Every row matches within about 0.003 s for a 0.25 in offset (half of AeroTech's
  0.5 in grain, p. 19) and about 1.97 s of boost. The table divides the path by cos θ; Eq. 6-1 prints a product,
  which would shorten the path, so the equation is a typo.
- **The rim is the worst case (Inference).** Ignition spread evenly over the face gives half the rim value on
  average (+0.28 s at 6 s, SD 0.16 s), since the added path goes as r².
- **The evidence is one bench pair.** Data from over 1,000 motors "is no longer available" (p. 15). The only data:
  two reload bulkheads lit at ambient pressure and stopwatch-timed from ignition to ejection, one through a center
  hole and one at the edge (pp. 15–17). Center 18.24 s, edge 21.26 s: +3.02 s, +16.6% (Table 7-1, p. 17). The
  authors say the low pressure exaggerates the effect (p. 18). One firing each, so there's no scatter and placement
  can't be told from grain-to-grain variation.

## 3. Weaver, age-related changes in delay time (NARAM-31, 1989)

**What was tested.** 60 Estes C6-7 black-powder motors donated with known manufacture dates (p. 1): 30 from three
April 1988 lots and 30 from two June 1989 lots (pp. 5, 9). The stand's burnout-to-ejection timer failed, so four
people timed each delay from videotape by stopwatch, burnout to ejection, and averaged their readings (pp. 1, 4).
One old motor blew its nozzle out (p. 5), leaving 29 timed. Test date, temperature and storage aren't given.

**What it found.** The old motors averaged 5.50 s and the new ones 6.05 s (lot means 6.01 and 6.08 s; p. 5). Both
ran short of the 7 s label and the catalog's ±15% floor of 5.95 s: the new "at the very low part", the old "far
below" (pp. 2, 6). Weaver suggests the delay is effectively timed from ignition (p. 6).

**Arithmetic (Inference).** Only the old group's per-motor table survives in the scan (p. 9); a faint show-through
of the new group's sheet is unreadable.
- Old group: offset −1.50 s (−21.4%), SD 0.41 s, n 29, range 4.57 to 6.78 s. 17 of 29 sit below NFPA 1125's
  5.5 s floor for a 7 s label. One reading scatters 0.10 s about its motor's mean, so timing adds little.
- New group: offset −0.95 s (−13.6%); no SD. The 1988 lot means span 0.20 s (5.38 to 5.58 s).
- Age: −0.55 s over the 1.16 years between mean manufacture dates, −0.47 s (6.8% of the label) per year. With the
  new group's SD taken equal to the old one's, the gap is about 5 standard errors; the slope's 95% range is −0.29
  to −0.66 s per year. Production year, storage and lot can't be told from age; one interval gives no shape.
- Cross-check: NAR's 1995 Estes C6 sheet gives 6.24 s for the 7 s delay [7], 0.19 s above Weaver's new group.

## 4. Gaff, extreme cold and black-powder motors (NARAM-26, 1984)

**What was tested.** 24 Estes B6-4 from one hobby store, two manufacturing runs, sealed and held between layers of
dry ice at −45 °C for 10 hours (pp. 2–3). Twelve were static-fired next morning still frozen, at about −40 °C;
the other twelve warmed for about an hour to the field's temperature, which isn't stated, then fired (p. 2). There
was no thrust stand and no timer: outcomes are categories (p. 4).

**What it found.** Fired cold: 2 split casings, 8 ejection blow-throughs, 2 normal (p. 3). Warmed first: all 12
normal (p. 3). Gaff concludes that a motor fired cold is likely to fail and a thawed one shows little risk (p. 4).

**Arithmetic (Inference).**
- Fired at −40 °C, 10 of 12 failed (Clopper–Pearson 95% 52% to 98%); warmed, 0 of 12 (one-sided 95% bound 22%).
- One run failed 9 of 9 cold, the other 1 of 3 (Fisher p = 0.045): weak evidence that the lot mattered.
- The report doesn't define a blow-through or say when it fired. Read as gas reaching the ejection charge past the
  delay, it is an ejection at or near burnout: an early-side delay failure, not a long delay.
- Motor delay spread's black-powder term, delay × (1 − 0.0030 ΔT), fitted over about 2 to 36 °C [6], predicts an
  18% longer delay at −40 °C. Gaff instead shows a failure regime there. Nothing between 0 and −40 °C is measured.

## 5. Against motor delay spread

| Note's figure | What these reports do |
|---|---|
| Model bias −3% | Not moved: Staab's 39 average −1.2%. Weaver's C6-7s run −14% and −21%, one type far off |
| Composite SD 0.64 s, within-type 0.50 s | Challenged: Staab's types run 0.33 s and 1.14 s, pooled 0.80 s, one flyer |
| No clear out-of-band NAR firing | Field rate 5 of 39 (95% 4.3% to 27.4%) on retail motors |
| Composite temperature unsized | Still unsized; one reload type bounds it to −0.42% to +0.20% per K |
| Black-powder −0.30% per K | Not contradicted (none timed it); Gaff shows failures, not long delays, at −40 °C |
| Age left out | Black powder: −0.47 s per year on one type, lot-confounded; composite unknown |
| Altitude sign only | Not measured by any of the four |

None of the four touches the 249-value recount that #735 carries into early deployment bias: they add no NAR sheet.

## What this suggests

- **Draw a scatter per motor type.** A Monte Carlo could draw each type's within-type SD once, spread about a factor
  of 3 (0.33 to 1.14 s on Staab's two types), instead of a fixed 0.50 s. That would widen M6.8's interval on a
  motor with no NAR sheet. With a sheet, the sheet's own spread is better evidence.
- **Give M7.4's delay mismatch a field rate.** About 13% of Staab's retail flights missed the band, most early;
  the sample allows 4% to 27%, and one flyer's sample can't fix the rate.
- **Bound the black-powder temperature factor** to about 0 to 36 °C, where it was fitted. Below that, say a cold
  motor may fail rather than run long; Gaff's 10 of 12 at −40 °C is the only figure.
- **Say the age effect's sign:** older black-powder delays fire earlier (−0.47 s per year on one type), with no
  composite figure. Igniter placement adds a one-sided late term of at most about +0.6 s, unsized in flight.

## Open questions

- **Cold.** No report fired composite motors at stated grain temperatures; Staab's air temperatures are confounded
  with lot and day. Black powder between 0 and −40 °C is unmeasured too.
- **Which motors Staab flew.** Makers and types are withheld, so no type can be checked against its NAR sheet.
- **Weaver's new group** has no per-motor table, so the age gap's error assumes equal SDs; one lot at two ages
  would separate age from lot.
- **Igniter placement in flight.** The 1,000-motor data are lost; chamber gas may light the whole face at once.

## Sources

Read 2026-11-04; NAR's reports are members' copyrighted work, cached under `refs/`, cited, not copied. Index:
<https://www.nar.org/content.aspx?page_id=22&club_id=114127&module_id=674799>.

1. J. Staab, "Delay Variability of APCP Mid-Power Model Rocket Motors (and its Effects on Precision Altitude/Set
   Duration Performance)", NAR R&D report, NARAM-55, 2013, 10 pp.
2. Pod Bay Doors (Team T-201), "Igniter Placement and the Effect on Delay Times in Composite Motors", NAR R&D
   report, NARAM-46, 2004, 23 pp.
3. C. Weaver, "Age-Related Changes in Ejection Delay Time of Model Rocket Engines", NAR R&D report, NARAM-31,
   1989, 10 pp.
4. R. Gaff, "Effect of Extreme Cold on Model Rocket Motors", NAR R&D report, NARAM-26, 1984; NAR *Technical Review
   No. 7*, ch. 5, pp. 45–48.
5. PerfectFlite, pNut user manual (French edition, Planète Sciences), p. 11:
   <https://planete-sciences.org/espace/IMG/pdf/notice_altimetre_pnut_fr_v1.pdf>
6. M. Nakano, H. Mashidori, Y. Takahashi, "Effects of Temperature and Aging on Performance of A8-3 Model Rocket
   Engines", TMCIT bulletin 7 (2013): <https://metro-cit.repo.nii.ac.jp/record/153/files/KJ00008928239.pdf>
7. NAR S&T, Estes C6 motor data sheet, tested 1995-03-25 ([motor delay spread](motor-delay-spread.md) source 8).
