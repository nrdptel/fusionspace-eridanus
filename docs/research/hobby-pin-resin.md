# What hobby shear pins are made of, and how damp they arrive

This note asks what the shear pins sold to hobby rocketeers are made of: resin, molding, thread class, color and
moisture, and whether a black pin arrives wetter than a natural one. It serves the pin load spread in M6.7's charge
sizing ([roadmap](../ROADMAP.md); issue [#857](https://github.com/nrdptel/fusionspace-eridanus/issues/857), pin
strength) and the columns of M6.8's ground-test log. It builds on [shear-pin-strength](shear-pin-strength.md) (§2
resin data, §3 thread areas, §4 Damerau's breaks, §5 causes) and [pa66-rate-cold-shear](pa66-rate-cold-shear.md)
(§3, moisture and cold), and does not repeat them. Read 2026-11-03: five hobby vendors' pages, McMaster-Carr, two
editions of the Micro Plastics catalog, Essentra drawings and listings, and DuPont's design guide. **Inference** marks
this note's own reading. Trust the vendors' words as printed (most state little); the moisture times and strength
factors are resin-data inferences, good to their sign and roughly a factor of two.

**In short.**
- **The vendors state almost nothing.** Of five hobby vendors, none names a resin grade, molding process, thread
  class or moisture state. Apogee gives color (black 2-56, white 4-40); its strength figures are Damerau's 2004
  breaks, not a test of its own pins. Missile Works prints "about 25 lbf" (2-56) and "about 35 lbf" (4-40), unsourced.
- **The industrial maker states a lot.** Micro Plastics (Essentra since December 2017) molds its screws in natural
  nylon 6/6, with threads that "approximate" class 2A, and colors black ones by dyeing in water at about 93 °C
  (200 °F), which takes them from about 0.5% to 3% to 4% moisture. Shipments are "moisture packed".
- **So a dyed black pin from this maker arrives wetter than "conditioned."** Nylon 6/6 holds about 2.5% water at
  50% RH. **Inference:** a pin-sized part takes up that much in about 0.5 to 1.3 h at 77 to 100 °C (DuPont Fig. 145),
  so the dye bath soaks it through; a dry pin left at 50% RH closes half its gap in 5 to 20 days (2-56).
- **Apogee's two pins match two Micro Plastics catalog parts exactly** (2-56 × 1/4 in round slotted; 4-40 × 5/16 in
  fillister). Apogee names no maker, so a rebadge is likely but not shown.
- **Sign and size:** a pin at 3% to 4% water is weaker than a dry one, by −5% (if it follows punch shear) to −41% (if
  it follows tensile yield). That range covers Damerau's 23% to 31% shortfall at its top and a sixth to a fifth of it
  at its bottom; his pins' color and maker are unstated, so moisture is a sufficient cause, not a shown one.

## 1. What each source states ("not stated": the page says nothing on it)

| Source, product | Size, head | Resin | Molded or machined | Thread class | Color |
|---|---|---|---|---|---|
| Apogee 29615 [3] | 2-56 × 1/4 in, round slotted | "Nylon" | not stated | not stated | black |
| Apogee 29632 [4] | 4-40 × 5/16 in, fillister slotted | "Nylon" | not stated | not stated | white |
| Missile Works NF-256 [5] | 2-56, 0.5 in; Phillips or slotted | "nylon" | not stated | not stated | not stated |
| Missile Works 4-40 [6] | 4-40, 0.5 in; head "supply dependent" | "nylon" | not stated | not stated | not stated |
| Wildman, SKU 1938 [7] | "2/56", 10 pack | not stated | not stated | not stated | not stated |
| LOC Precision [8] | M2 × 3/8 in; M3, M4 × 1/2 in | "nylon" | not stated | not stated | not stated |
| McMaster 93135A272 [10] | 4-40 × 1/4 in, Phillips pan | "Nylon" ("Nylon 6/6" [11]) | not stated | 2A | off-white |
| Micro Plastics [12] to [14] | e.g. 010256R025 | nylon 6/6 | injection molded | about 2A | natural; dyed black |

| Source | Moisture or packaging | What it says about strength |
|---|---|---|
| Apogee [3], [4] | not stated | three pins: 64.24 lbf (2-56), 119.2 lbf (4-40), credited to RocketMaterials.org |
| Missile Works [5], [6] | not stated | each shears at "about 25 lbf" (2-56), "about 35 lbf" (4-40); no basis given |
| Wildman, LOC [7], [8] | not stated | nothing |
| McMaster 93135A272 [10] | nylon "will swell and weaken" in moisture; best for dry use | tensile strength "Not Rated" |
| Micro Plastics [12], [13] | 0.5% molded, 3% to 4% dyed; "moisture packed" | dry, double shear: 50 lbf (4-40) |

- **Rocketman sells no pins online:** its store's product feed listed 68 products on 2026-11-03, and none mentions
  "shear" in its title or text [9] (rocketmanparachutes.com redirects there).
- **Apogee's figures are Damerau's.** 64.24 lbf is his three-pin #2 average to the digit; 119.2 lbf matches his first
  three-pin #4 pull (119.273 lbf), not his three-pin #4 average, 114.97 lbf (archive copy of [1]'s source 1).
  **Inference:** Missile Works' "about 25 lbf" is close to Damerau's two-pin #2 average (24.64 lbf per pin); its
  "about 35 lbf" for 4-40 sits below every three- and four-pin #4 average he measured (38.2 and 38.3 lbf).
- **McMaster:** made in the United States; nylon may "expand" in moisture [11]; its 2-56 and black parts need a browser.
- **Micro Plastics' grade:** its nylon 6/6 meets MIL-M-20693B "A" type 1, LP-410A and ASTM D789 type 1 grade 2 ([12]
  p. 166; [13] p. 242), and its sheet lists D732 shear as 9,600 psi ([12] p. 167), Fetter's Min value.

## 2. Whose screws the hobby pins are

- **Micro Plastics is now Essentra.** Essentra bought the Flippin, Arkansas maker, about 400 employees, in December
  2017; the sale "closed Wednesday", 13 December, per [16] of the 14th. Distributors list today's Essentra screws
  under the series "MicroPlastics" [15], and Essentra's part drawings still read "Property of Micro Plastics Inc."
  with material "6/6 Nylon" (date box blank; the PDFs were made in 2006) [14].
- **Apogee's two pins match two catalog parts.** Micro Plastics' 1999 catalog lists 010256R025, a 2-56 × 1/4 in round
  slotted machine screw ([12] p. 21), and 010440F031, a 4-40 × 5/16 in fillister slotted one (p. 17): the same size,
  head and drive as Apogee's two [3], [4]. Both parts are active, and 010256R025B is listed [15] (black by the
  catalog's "B" suffix, **Inference**). **Inference:** Apogee's pins are likely Micro Plastics (Essentra) screws;
  its page says "Manufactured by: Apogee", so this rests on the match alone. Other molders make the same sizes; a
  part marking or Apogee's word would settle it.
- **Which resin Essentra molds today** is unstated; its asset library hosts Ascend's Vydyne 21SPC sheet ([1] source
  8), a nylon 6/6 that loses 23% of its yield when conditioned: a hint, not a statement of use.

## 3. Do black pins arrive conditioned?

**The maker's words.** Micro Plastics molds every screw in natural resin. Black or other colors are made by dyeing
the molded screw with RIT dye in water at about 200 °F, which "adds color but also moisture". Screws leave the press at
about 0.5% moisture and the dye bath at about 3% to 4% ([12] p. 168; same text in the 2005 edition, [13] p. 248). Its
terms add that all shipments are "moisture packed" and the sealed package should stay closed until use ([12] p. 169;
[13] p. 249). The 2005 edition calls nylon hygroscopic: moisture plasticizes it and lowers tensile strength ([13]
p. 244). **Inference:** sealing keeps a part at its shipped moisture, dry for natural, wet for dyed.

**Against the resin's own equilibrium.** Nylon 6/6 holds about 2.5% water at 50% RH and 8.5% at 100% RH ([17] p. 2;
Vydyne 21SPC 2.4%, [1] source 8). A dyed pin at 3% to 4% is wetter than the 50% RH "conditioned" state that data sheets
test. **Inference:** [1] §5's "a dyed black pin starts conditioned" is right in sign and understates the amount.

**How fast the bath soaks a pin.** DuPont's Fig. 145 ([17] p. 108) gives hot-water immersion time to add 2.5% to
Zytel 101 against part thickness. This note's pixel scan (400 dpi, ±0.1 h): 0.5 h at 100 °C, 0.8 h at 77 °C and 2.0 h
at 60 °C for a 1.7 mm part; 0.9, 1.3 and 3.0 h for 2.2 mm. A 2-56 pin is 1.6 to 2.2 mm across its root and crest
([1] §3), and a cylinder fills faster than a plate of the same thickness. **Inference:** an hour in a 93 °C bath
moistens a pin through, which agrees with the maker's 3% to 4%.

**How fast a stored pin moves toward room humidity** (**Inference**, Fickian diffusion, constant D).
- DuPont: a dry 1.5 mm plate needs about 125 days to reach equilibrium at 50% RH ([17] p. 2). Reading "equilibrium"
  as 90% to 99% of the uptake, the plate solution gives D = 0.0038 to 0.0080 mm²/day.
- In water at 23 °C (Fig. 147, [17] p. 108; pixel scan, ±1 day), a 1.52 mm Zytel 101 plate holds half of 8.5% at
  4.3 days and 90% at 18.9 days: D = 0.026 mm²/day, 3 to 7 times faster than at 50% RH.
- A long cylinder of radius a takes up the fraction 1 − Σ 4/(αₙa)² exp(−Dαₙ²t), with J₀(αₙa) = 0: half at
  Dt/a² = 0.063, 90% at 0.334.

| Pin, radius (root to crest) | Half the gap closed, 50% RH | 90% closed, 50% RH | 90% closed, in water at 23 °C |
|---|---|---|---|
| 2-56, 0.81 to 1.09 mm | 5 to 20 days | 27 to 104 days | 8 to 15 days |
| 4-40, 1.03 to 1.42 mm | 8 to 33 days | 44 to 177 days | 14 to 26 days |

- So a natural pin from a fresh bag is near dry, one kept a season at 50% RH near conditioned, and a dyed one
  dries toward 2.5% as slowly, likely slower at the end, as D falls with moisture (water D is 3 to 7 times larger).

## 4. What it does to the pin force: sign and size

Against Fetter's dry row (9,600 psi on the minor area, [1] §1), Damerau's three- and four-pin means are 0.69 (#2) and
0.77 (#4): 23% to 31% low ([1] §4).

| Cause | Sign | Size on the force | Basis |
|---|---|---|---|
| Dyed: 3% to 4% water against dry as molded | lower | ×0.59 to ×0.70 (yield); about ×0.95 (punch) | below; [1] §2 |
| Natural pin kept a season at 50% RH | lower | ×0.72 (yield); ×0.95 (punch), in §3's times | Fig. 4 [17]; [1] §2 |
| Molded class 2A against Fetter's 3A root | lower | −1.6% to −2.0% by area | [12] p. 168; [1] §3 |
| Parting-line mismatch, flash, gate (molded) | not known | unbounded | [12] p. 168 |

- **The yield reading (Inference).** DuPont Fig. 4 ([17] p. 31; this note's 400 dpi scan, ±1 MPa) gives Zytel 101's
  yield at 23 °C: 81.2 MPa dry (0.2%), 58.3 MPa at 2.5% and 36.3 MPa at 8.5%. Interpolating straight in moisture, 3%
  and 4% give 56.5 and 52.8 MPa (×0.70, ×0.65). Sandia's rule, 1% moisture acting like 14 °C ([2] §3), puts 3% and
  4% on the 2.5% curve at 30 °C and 44 °C: 54.3 and 48.2 MPa (×0.67, ×0.59). Hence ×0.59 to ×0.70.
- **The punch-shear reading.** The one conditioned shear pair, Zytel 105, falls only 5% at 2.5% ([1] §2); no shear
  value at 3% to 4% was found, so "about ×0.95" for a dyed pin is a floor on the loss, not a measurement.
- **Against the 23% to 31% shortfall.** **Inference:** if a pin breaks like tensile yield, a dyed pin alone explains
  all of it (×0.59 to ×0.70 against 0.69 and 0.77 measured); if like punch shear, about a sixth to a fifth. Damerau
  names no color or maker, so the shortfall stays unexplained; moisture is the largest cause that this note can bound.
- **Which way M6.7 errs.** Moisture only lowers the force, so a high end built on dry resin stays on the safe
  side (a larger charge); the cold and fast-loading factors in [2] §6 still raise it.

## What this suggests

- **M6.8's ground-test log could add four columns:** pin source and part number (vendor SKU, or maker's number),
  color (natural or black), whether the bag was sealed when bought and the date it was opened, and where the pins were
  kept (indoors, garage, launch box). Expected effect: a logged spread can then be split into a moisture part (§4,
  −5% to −41%) and a fit part ([1] §5), instead of one unexplained scatter.
- **M6.7's low end should not stop at 0.6:** a dyed pin is ×0.59 to ×0.70 of dry on the yield reading, but that is
  no floor: a pin soaked to 8.5% reads ×0.45 (Fig. 4), and a hot pad (×0.75 to ×0.79, [1] §5) lowers either. The
  high end stays dry.
- **A cheap check settles the moisture of a batch** (**Inference**): Apogee lists "Weight: 0.02 g" on its 20-pack
  [3]; read as per pin, 100 pins weigh about 2 g and each 1% of water is 0.02 g, within a 0.001 g jeweler's scale.
  Weigh as bought, dry at 80 °C to constant mass, and the difference is the water.
- **A paired break test** would turn §4's range into a number: black and natural 2-56 of one part number, dry and
  as-bought, in one tapped fiberglass joint with three pins, beside [1]'s suggested series.

## Open questions

- Whether Essentra still dyes black screws, or now molds them in black resin: its product pages refused plain
  fetches (HTTP 403) on 2026-11-03, and the 2005 catalog [13] is the newest maker's text found.
- Who makes Apogee's and McMaster's pins; whether McMaster's black nylon is dyed. McMaster's 2-56 and black pages
  need a browser. A Rocketry Forum thread comparing Apogee's pins with McMaster screws ("shear pins vs. nylon screws",
  thread 108620) refused fetches, the Internet Archive was offline, and the browser route was not connected; it
  stays a lead.
- What "moisture packed" is meant to keep: the shipped state or a set humidity.
- No shear strength of nylon 6/6 at 3% to 4% water, and no break test of dyed against natural pins, was found.
- DigiKey's head-type field calls 010440F031 "Pan Head"; the catalog's "F" code is fillister [12], [15].
- Equilibrium moisture below 50% RH, for pins kept in dry climates, is not in the sources read.

## Sources

All read 2026-11-03; vendor and maker pages are copyrighted: cite only, cached privately under
`refs/research/hobby-pin-resin/` (gitignored) where noted.

1. [shear-pin-strength](shear-pin-strength.md), this repository, §1 to §5 and What this suggests; its source 1 is
   Damerau, "Shear Pins", https://web.archive.org/web/20150218234304/http://www.rocketmaterials.org/datastore/cord/
   Shear_Pins/index.php (archive copy cached by that note), and its source 8 the Vydyne 21SPC sheet.
2. [pa66-rate-cold-shear](pa66-rate-cold-shear.md), this repository, §3 (Sandia's moisture rule) and §6.
3. Apogee Components, "Small Nylon Shear Pins - 20 pack", P/N 29615: https://www.apogeerockets.com/Building-Supplies/
   Misc-Hardware/Nylon-Shear-Pins-20-pack. Cached.
4. Apogee Components, "Large 4-40 Nylon Shear Pins - 20 pack", P/N 29632: https://www.apogeerockets.com/
   Building-Supplies/Misc-Hardware/Large-4-40-Nylon-Shear-Pins-20-pack. Cached.
5. Missile Works, "2-56 Nylon Shear-Pin (10 pack)", store linked from missileworks.com/store:
   https://store2263081.ecwid.com/2-56-Nylon-Shear-Pin-10-pack-p24591273. Cached.
6. Missile Works, "4-40 Nylon Shear-Pin (10 pack)":
   https://store2263081.ecwid.com/4-40-Nylon-Shear-Pin-10-pack-p24591284
7. Wildman Rocketry, "SHEER PINS", SKU 1938: https://wildmanrocketry.com/products/sheer-pins (read through the store's
   product feed, https://wildmanrocketry.com/products.json, page 3). Cached.
8. LOC Precision, "M2 Shear Pins" and "M3 Shear Pins": https://locprecision.com/products/m2-shear-pins,
   https://locprecision.com/products/m3-shear-pin. Cached.
9. Rocketman (the-rocketman.com), product feed: https://www.the-rocketman.com/products.json; 68 products counted.
10. McMaster-Carr, 93135A272, nylon pan head Phillips screw, 4-40 × 1/4 in: https://www.mcmaster.com/93135A272.
11. McMaster-Carr, "Nylon 6/6 Screws" category page: https://www.mcmaster.com/nylon-6/6-screws.
12. Micro Plastics catalog (PDF of June 1999), pp. 17, 21, 166 to 169: https://www.farnell.com/datasheets/3153494.pdf.
    Cached by [1].
13. Micro Plastics catalog (PDF of June 2005), pp. 242, 244, 248, 249, served as the datasheet of Essentra 010440F031:
    https://mm.digikey.com/Volume0/opasdata/d220001/medias/docus/6166/5020.PDF. Cached.
14. Micro Plastics drawings for Essentra 010256R025 and 010256F037 (2006): https://mm.digikey.com/Volume0/opasdata/
    d220001/medias/docus/871/010256R025.PDF; https://essentracomponents.bynder.com/m/6d7da4390922193f/original/
    1501695-pdf.pdf. Cached.
15. DigiKey listings of Essentra 010256R025, 010440F031 and 010256F037 (series "MicroPlastics", active), and a search
    showing 010256R025B: https://www.digikey.com/en/products/detail/essentra-components/010256R025/9677278,
    https://www.digikey.com/en/products/detail/essentra-components/010440F031/10242246,
    https://www.digikey.com/en/products/result?keywords=010256R025B.
16. "U.K. maker of parts buys Micro Plastics operation in Flippin", Arkansas Democrat-Gazette, 14 December 2017:
    https://www.arkansasonline.com/news/2017/dec/14/u-k-maker-of-parts-buys-micro-plastics-/.
17. DuPont, Zytel and Minlon Design Guide, Module II: p. 2 (moisture), Fig. 4 (p. 31), Figs. 145 and 147 (p. 108):
    http://foremostplastic.com/wp-content/uploads/2015/04/DuPont-Module-II-Nylon.pdf. Cached by [1].
