# How much longer composite delays run at high sites

Tripoli Motor Testing (TMT) has fired near Denver since 2025. Can its letters, paired with the same motors' low-site
ones, size how much longer a composite ejection delay runs in thinner air, and bound a burn-rate pressure exponent?
It serves the settings check's late side (M6.8 in the [roadmap](../ROADMAP.md)), the "delay mismatch" fault (M7.4)
and [motor delay spread](motor-delay-spread.md) §7, which leaves altitude out. It builds on that note's §3, §4 and §6,
[TMT test sites](tmt-test-sites.md) §1 and §3, and [NAR delay reports](nar-delay-reports.md). Read 2026-11-04:
ThrustCurve's TRA letters and API, Tripoli's TMT page, five forum posts, two abstracts; no source code. **Inference**
marks this note's arithmetic. Trust the letter count fully; the size only to about a factor of two, since it rests on
one remembered bench test of three split AeroTech delay grains.

**In short.**
- **The letters can't answer it.** ThrustCurve holds 4 TRA letters dated 2025 or later, all dated 2025-10-16; Tripoli
  lists none for 2026. They print delay labels only: 28 motors fired, no measured delay. The pairing fails, and so
  does any indirect comparison with the low-site distribution (§1).
- **One paired test sizes it: +8% over a mile up (Butte, Montana).** TMT's late chair and a Butte flyer split three
  full-length AeroTech delay grains and burned the halves at their two sites. The high halves ran 8% longer, "very
  consistent", per the flyer's account [5, 6]. An AeroTech forum account gives the sign: delays run longer up high [5].
- **That bounds the exponent to about 0.27 to 0.51.** With burn rate ∝ pⁿ, 8% at the two sites' pressures gives
  n = 0.41; rounding and site spread give 0.31 to 0.46, and an unknown temperature difference widens it to 0.27 to
  0.51 (**Inference**, §3). Motor delay spread §6 assumed 0.3 to 0.8.
- **Size at a Windsor or Denver pad: +5% to +10% before the flight climbs.** Over a 1,500 m coast from there the
  delay lasts 9% to 18% longer than on a sea-level stand (**Inference**, drag-free, §4). The high-power early bias of
  −10% nearly cancels on the pad and turns late in flight.
- **Certification letters could resolve n only with many lots.** One lot per site leaves n ±0.28 to ±1.26 at 90%
  (§5). TMT's 2018 study (*Tripoli Report* 29(3)) is behind Tripoli's member login and was not read (§6).

## 1. The post-2025 TRA letters print no delay times

**How they were counted.** ThrustCurve's API returned 607 TRA-certified motors on 2026-11-04; 10 were updated since
2025-10-01: the four below and six others, whose certification pages were read with those of six Quest and AeroTech
motors [2]. Four letters date from 2025 on, all of 2025-10-16 for firings of 2025-10-06, on a Windsor, CO letterhead
[1]. The one other new upload is a 2020 North Carolina I175WS letter whose four firings motor delay spread §4 already
holds. Tripoli's TMT table, read the same day, has 9 entries for 2025 and none for 2026 [3].

| Letter (2025-10-16) | Motors fired | Delay line as printed | Measured delays |
|---|---:|---|---:|
| AeroTech F115SN-12A DMS | 9 | 12, 10, 6 s, adjustable with the drilling tool | 0 |
| Enerjet by AeroTech G73C | 11 | 7, 11, 15 s | 0 |
| AeroTech J550ST-14 RMS | 6 | 14 s only, or plugged | 0 |
| AeroTech N3120X-PS | 2 | none (plugged bulkhead) | 0 |

- **The format dropped the timings.** The North Carolina letters printed nominal and measured delays (16 of 50 listed
  each firing, motor delay spread §4); the I175WS letter of 2020 is one [2]. The 2025 letters print labels only [1].
- **The Quest nozzle changes** (four motors, 2025-08-14 and 2025-09-08 [3]): no 2025 letter on ThrustCurve [2].
- **Siblings exist but don't help.** North Carolina letters time 9 Super Thunder firings (J615ST-20A, J1265ST-14A) and
  20 Classic ones (E24C, F67C), cousins of the J550ST and G73C, but there is nothing at altitude to pair them with.
- **"Complies with NFPA 1125" bounds nothing.** TMT certified the J520W-13A with two firings outside the band
  (motor delay spread §4), so a pass does not prove every firing was inside it.

## 2. A paired bench test: delay halves burned near 1,690 m ran 8% long

- **The test.** A Rocketry Forum member in Butte, Montana wrote in 2021 and 2022 that they and TMT's chair, Alan
  Whitmore (died 2024, TMT test sites T3), cut full-length AeroTech delay grains in half, three per the 2022 post
  [5, 6]. One set of halves burned at "about 400' MSL", the other at "over a mile MSL", timed from one end lighting
  to sparks leaving the other (the member by frame-marking video) [5, post #10]. One grain has no lot or drilling
  scatter. **Inference:** burning outside a motor matches a delay after burnout, when the case has vented.
- **The result.** The high halves burned 8% longer, a difference the member calls very consistent; in 2021 it was
  the low-site rate being 8% faster [5, 6]. The member says the numbers are in Whitmore's *Tripoli Report* study.
- **The sites (Inference).** Butte's town-center point is 1,687 m (5,536 ft); Carrboro and Chapel Hill, NC, where
  Whitmore lived, are 141 and 149 m [9; TMT test sites §2]. The Tripoli study's two sites, 5,500 ft and 11 ft per a
  forum summary [7], fit Butte; the 11 ft site is not the 400 ft one, so may be another stand.
- **Its limits.** Three pairs, recalled years later, rounded to a whole percent; no temperatures or day pressures.
  A 10 K gap moves the 8% by about 1.5 points ([motor temperature firings](hobby-motor-temperature-firings.md)).
- **The sign has two more sources.** The forum account named AeroTech: delays are certified at sea level and burn
  longer higher up, "sometimes significantly longer" [5, post #8]. NAR's combined list agrees, unsized (motor delay
  spread §6). The member adds that a grain burns about five times faster while the motor runs [5].

## 3. What exponent that implies

With burn rate r ∝ pⁿ, a delay of fixed length lasts t ∝ p⁻ⁿ, so n = ln(t_high/t_low) / ln(p_low/p_high). Pressures
are ISA at USGS heights [9]: 101.28 kPa at 3 m, 99.55 kPa at 149 m, 83.43 kPa at 1,609 m, 82.63 kPa at 1,687 m.
The 1,900 m edge below is this note's allowance above Butte's center, not a sourced height.

| Case (Inference) | Time ratio | Low site | High site | n |
|---|---:|---:|---:|---:|
| As reported, named sites | 1.080 | 149 m | 1,687 m | 0.41 |
| Rounding of "8%", site spread | 1.075 to 1.085 | 3 to 149 m | 1,609 to 1,900 m | 0.31 to 0.46 |
| Plus a 10 K temperature gap, either way | 1.065 to 1.095 | 3 to 149 m | 1,609 to 1,900 m | 0.27 to 0.51 |

- **A rough cross-check (Inference).** If the "five times faster" rate is chamber against outside pressure, and an
  AeroTech chamber runs 2.5 to 7 MPa (assumed, not sourced), then n = ln 5 / ln(p_c/p_a) is 0.38 to 0.50.
- **The literature gives only the trend.** No measured n for a delay grain or slow AP propellant near 1 atm was found.
  A simulation gives AP/HTPB a smaller index at 0.02 to 0.08 MPa than at 5 MPa [10]; tests saw it burn irregularly
  at 13 to 100 kPa [11]. The member says AeroTech lists one of its two delay compositions as a smoky propellant [6];
  this note found no such AeroTech listing.

## 4. What it means for a delay at a site and in flight (Inference)

Stretch against a sea-level stand, (101.325/p)ⁿ − 1, for the three exponents above:

| Where the delay burns | ISA p, kPa | n = 0.27 | n = 0.41 | n = 0.51 |
|---|---:|---:|---:|---:|
| 1,000 m | 89.87 | +3.3% | +5.0% | +6.3% |
| Windsor, CO (1,462 m) | 84.95 | +4.9% | +7.5% | +9.4% |
| Denver (1,596 m) | 83.56 | +5.3% | +8.2% | +10.3% |
| Denver-area bound (85.3 to 80.5 kPa) | 80.5–85.3 | +4.8% to +6.4% | +7.3% to +9.9% | +9.2% to +12.5% |
| 3,000 m | 70.11 | +10.5% | +16.3% | +20.7% |
| 5,000 m | 54.02 | +18.5% | +29.4% | +37.8% |

**In flight the delay burns at the rocket's altitude, not the pad's.** A drag-free coast, apogee timed at the delay's
sea-level length, burning by the law above (script cached, `flight_stretch.py`):

| Pad | Coast height gain | Coast time | n = 0.27 | n = 0.41 | n = 0.51 |
|---|---:|---:|---:|---:|---:|
| Sea level | 1,500 m | 17.5 s | +3.3% | +5.2% | +6.5% |
| Sea level | 3,000 m | 24.7 s | +7.0% | +11.0% | +13.9% |
| Denver (1,596 m) | 300 m | 7.8 s | +6.1% | +9.4% | +11.8% |
| Denver (1,596 m) | 1,500 m | 17.5 s | +9.1% | +14.2% | +18.1% |
| Denver (1,596 m) | 3,000 m | 24.7 s | +13.2% | +21.0% | +27.0% |

Drag makes a real coast shorter and front-loads the climb, so the rocket spends more of the delay high: these rows
likely understate. The grain burned under chamber pressure barely changes, so delays timed from burnout carry the
whole stretch. At 5,000 ft (84.3 kPa) the ground figure is +5.1% to +9.8%; motor delay spread §6 assumed +6% to +16%.

- **Against motor delay spread §7.** An undrilled high-power 14 s delay centers near 12.6 s at sea level (−10%). At a
  Denver pad it centers near 13.6 s on the ground (×1.082), and near 14.4 s over a 1,500 m coast (×1.142): the early
  bias turns late. Drilled delays, already +0.8 s late, run later still.
- **Against NFPA 1125's band.** If TMT times delays at Windsor–Denver without correction, a 14 s delay reads +0.7 to
  +1.75 s long (+4.8% to +12.5%), 24% to 63% of its ±2.8 s band.

## 5. Why letters can't size it, and how many firings would

Two-site firings carry 0.50 s per firing at every label (motor delay spread §3) and lot scatter, which letters years
apart keep; no source measures it for one composite type. Proxies: §3's type means, 6.5% of the label (an upper
proxy); Weaver's black-powder C6-7 runs of 1988 (three lots) and 1989 (two), 0.55 s (9%) apart on about 6 s ([NAR
delay reports](nar-delay-reports.md), age and lot confounded); and an assumed optimistic 2%.

Between North Carolina towns (99.6 kPa) and the Windsor–Denver middle (84.3 kPa), ln(p ratio) = 0.167. For a 90%
interval of ±0.1 on n (**Inference**, `firings_needed.py`):

| Requirement | 6 s label | 10 s label | 14 s label |
|---|---:|---:|---:|
| Firings per site, firing scatter only | 136 | 49 | 25 |
| Lots per site, lot scatter 2% | 8 | 8 | 8 |
| Lots per site, lot scatter 6.5% | 83 | 83 | 83 |

With one lot at each site, the 90% half-width on n is ±0.28, ±0.91 or ±1.26 for lot scatter of 2%, 6.5% or 9%,
whatever the firing count. TMT fired 2 to 11 motors per type in 2025 (§1): a letter pair, even with timings, bounds
n only if its lots match. Split grains avoid both noises; a jar pumped to a set pressure would also remove weather.

## 6. The Whitmore study: not reached

*Tripoli Report* Vol. 29 No. 3 (Summer 2018) carries Whitmore's study of delay timing in high-power motors, per a
forum poster who read it on Tripoli's downloads page in 2021: about 40 certified motors, about 30% outside NFPA 1125,
tests at 5,500 ft and 11 ft with no site per result, no temperatures [7]. A 2023 post dates it to spring 2020, hedged
[8]. Tripoli's reports page needed a member login in 2020 [12] and is gone now; the Internet Archive holds one 2016
issue, not this one. Without a Tripoli login, its split-grain numbers stay unread.

## What the project can use

- **Data terms.** TRA letters on ThrustCurve and Tripoli's pages: cite, cache under `refs/` only (Tripoli marks its
  site "all rights reserved", [TMT test sites](tmt-test-sites.md)). Forum posts carry no license: cite, quote under
  25 words. USGS heights are public domain. The two abstracts: cite; Kumar et al. is CC BY-NC-ND 4.0 [11].
- **A pressure term for §7 (suggestion).** Multiply the post-burnout delay by (p₀/p)ⁿ, burned along the trajectory,
  with n = 0.41 and 0.27 to 0.51 as its range, p₀ the stand's pressure (near 100 kPa for NAR and North Carolina
  TRA data). Expected effect: +5% to +10% at a Denver-area pad, +9% to +18% over a 1,500 m coast from there.
- **M6.8's late-side check and M7.4's prior (suggestion).** Compare the optimum delay with the stretched interval,
  and let the "delay mismatch" side depend on pad height and apogee: undrilled high-power delays err early at sea
  level; above about 1,400 m the pad cancels most of that, and a long coast turns it late.
- **Flight logs could test it.** A logged flight times burnout to the charge's pressure spike; the motor delay spread
  §7 interval, stretched by the coast-average pressure, is the prediction. No source tallies such flights.

## Open questions

- The Whitmore study's numbers: the 8% split-grain pairs, and the 40 or so motors by site; a member can read it.
- Whether TMT's Denver-area firings time delays at all now, and whether they correct to sea level.
- A measured n for an AeroTech or Cesaroni delay composition between 50 and 100 kPa, and at chamber pressure.
- Between-lot scatter for one composite type, which decides whether letters could ever size n; and whether the
  temperature and pressure terms simply multiply (cold high sites push both late).

## Sources

Read 2026-11-04 unless marked; cached under `refs/research/delay-altitude-tra-letters/` (gitignored), cited, not
copied. `tc:` is `https://www.thrustcurve.org/`; `trf:` is `https://www.rocketryforum.com/threads/`.

1. TMT certification letters, 2025-10-16, on ThrustCurve, `tc:motors/cert/<id>/`: F115SN 6919f0102d19df0002c39edc,
   G73C 6919f1702d19df0002c39fe2, J550ST 6919f2e42d19df0002c3a10f, N3120X 6919f5ee2d19df0002c3a316 (cached by
   [motor data](motor-data.md), re-read).
2. ThrustCurve API `tc:api/v1/search.json` (certOrg TRA, 607 motors) and the `cert.html` pages of 12 motors; I175WS
   letter of 2020-04-29, `tc:motors/cert/692089650db8660002205561/`.
3. Tripoli Rocketry Association, Tripoli Motor Testing page, certification table 2018–2025:
   <https://www.tripoli.org/TMT>; news of 2025-11-08 ([TMT test sites](tmt-test-sites.md) T5).
4. [Motor delay spread](motor-delay-spread.md) §3, §4, §6, §7 and its sources 8, 9 and 14.
5. "Accuracy Of Aerotech RMS Drillable Motor Delay Times", posts #8 (AeroTech account, 2022-11-07, id `2342265`) and #10
   (2022-11-08, id `2342354`): `trf:accuracy-of-aerotech-rms-drillable-motor-delay-times.175762/`
6. "Understanding effect of burn time on Aerotech motor delays?", post #11 (2021-07-16, id `2158424`):
   `trf:understanding-effect-of-burn-time-on-aerotech-motor-delays.167321/`
7. "BP motor consistency", post #31 (2021-05-02, id `2134103`): `trf:bp-motor-consistency.166006/page-2`
8. "Dual deployment first event: Ejection delay or altitude?", post #9 (2023-11-26, id `2517393`):
   `trf:dual-deployment-first-event-ejection-delay-or-altitude.183395/`
9. USGS Elevation Point Query Service, town centers (Butte, MT; Carrboro and Chapel Hill, NC):
   <https://epqs.nationalmap.gov/v1/json>; ISA pressure.
10. Y. C. Jin, Z. J. Wei, X. N. Wang, "Numerical study on combustion characteristics of AP/HTPB composite propellant at
    low pressure", *J. Propulsion Technology* 43(8), `210319` (2022), abstract: `doi:10.13675/j.cnki.tjjs.210319`
11. P. Kumar, M. Varshney, A. Manash, "Combustion performance studies of aluminum and boron based composite solid
    propellants in sub-atmospheric pressure regimes", *Propulsion and Power Research* 8(4), pp. 329–338 (2019),
    abstract: `doi:10.1016/j.jppr.2019.09.001`
12. Tripoli Rocketry Association, "Tripoli Reports" page, Internet Archive 2020-06-02 (member login):
    <https://web.archive.org/web/20200601113708/http://www.tripoli.org/TripoliReports>
