# How a packed bay's least charge scales from Fetter's fixtures to bays of 4 in and up

Which published ground tests bracket the least charge (a failed shot, then the next that worked) in packed bays of 4
in and larger? Does that least charge follow the pins' load, the bulkhead area or the volume, and does "the rule"
still clear every failure? The answer feeds [issue #857](https://github.com/nrdptel/fusionspace-eridanus/issues/857)
(which way [M6.7](../ROADMAP.md)'s charge errs) and M6.7's done-when that a packed bay is sized to separate. It
extends [published ground-test outcomes](ejection-ground-test-outcomes.md), which defines the rule (4.67 to 4.86
times the gas-law charge at the pins' balance pressure), the ld/pin ratio, m = 0.000516 × P × V and the pin forces.
Its rows are cited, not repeated. Read 2026-11-02: 31 student documents, 4 forum threads in full and a text screen of
129 more (counts below). Outcomes are as reported; **Inference** marks this note's own reading.

**In short.**
- **One new complete bracket at 4 in and up, and the rule clears it by 2.0.** A builder's 5.38 in bay, three 2-56
  pins, packed full: 1.62 g didn't move the nose cone, 2.0 g separated it [1]. That is ld/pin 2.33 failed and 2.87
  separated. The rule gives 3.25 to 3.38 g, 2.01 to 2.09 times the failure. The closest failure is still the parent's
  Buffalo row, 1.32 to 1.37 under the rule, on its ambiguous reading.
- **Four more brackets at 4 in and up lack a bay length or a pin size**, so they can't be tested outright. A 4 in bay
  failed at 1.5 g and separated at 2.0 g on 84 lbf of pins [2]; a 6.25 in bay's main failed at 5 g and worked at 6.5
  g, its drogue 3 g then 4.5 g [3]. The rule clears them if the bays are longer than 7.4 in, 14.4 in and 6.9 in
  (Oregon State's measured pins). Notre Dame 2018's lacks pin sizes; its 2019 pair is a leak [4, 5].
- **The least charge follows the pins' load times the bay length, not the volume or the area.** Over the 5 complete
  brackets (Fetter's two, Rice, Buffalo and [1]), the least charge over the gas-law charge at the pins' load spreads
  1.56 times (coefficient of variation 0.17). Per in³ it spreads 4.69 times, per in² of bulkhead 4.18. Without
  Buffalo's row the force basis spreads 1.12 times. **Inference**, on 5 brackets, 3 of them 4 in and up.
- **A student bay broke below the rule's size for it.** NC State's 7.34 in bay lost a bulkhead at 6 to 6.55 g, where
  the rule gives 9.7 to 10.1 g [6]. Texas Tech's 3.5 g split a 6 in tube, about 48 psi on its free volume, with its
  pins unstated [7]. A rule that clears every failure can still over-size a bay; no source bounds how much.
- **Brackets stay rare.** Of the 31 documents, 3 hold a failed-then-worked pair at 4 in and up, none with length, pin
  size and packing all stated. Of the 133 forum threads read or screened, 2 do, one of them complete.

## How each bracket is worked out

The method is the parent note's. A bay of inner diameter D and length L has area A = π/4 × D² and volume V = A × L.
Its n pins of force F (Fetter's nominal 31.0 lbf for 2-56, 49.8 lbf for 4-40) balance a pressure P_pin = n × F / A.
The gas-law charge at that pressure is m_pin = 0.000516 × P_pin × V, which is 0.000516 × n × F × L: it grows
with the pins' load and the length, and the diameter cancels. A shot's ld/pin is its charge over m_pin. The rule's
charge is 4.67 to 4.86 times m_pin. **Inference:** the least charge is taken as the geometric mean of the failed and
the clean charge (VTS-1's lower shot is a partial); every bracket here spans a step of 1.23 to 1.50 times.

Worked example, the new bay [1]: A = π/4 × 5.38² = 22.73 in², V = 22.73 × 14.5 = 329.6 in³, P_pin = 3 × 31.0 /
22.73 = 4.09 psi. Then m_pin = 0.000516 × 4.09 × 329.6 = 0.696 g, so 1.62 g is ld/pin 2.33 and 2.0 g is 2.87. The
rule asks 4.67 × 0.696 = 3.25 g. The builder's "about 9.5 psi" for 1.62 g matches the gas law on this volume
(1.62 / (0.000516 × 329.6) = 9.5 psi), so 5.38 in is the bore used (**Inference**).

## The new complete bracket: a 5.38 in bay with three 2-56 pins

A builder's 2020 thread [1]: one person's word; its linked video was not watched.

- **Setup** (#1, #5): 5.38 in tube, main bay 14.5 in of usable length, nose cone held by three nylon 2-56 pins.
  Packed with a deployment bag, a 9 ft parachute and shock cord: "NO extra space", a little compressed.
- **Shots:** 1.62 g, the pop is heard and the nose cone stays on (#1, #7). The next shot, 2.0 g, separated, though a
  reply judged it a little short (#8 to #10). Later the nose cone was found loose (#18); after masking tape and
  bronze shear plates on its shoulder (#27, #39), 2.0 g threw it 74 in with the bag not fully out, and 2.2 g was
  chosen (#42). The builder's own cap was 300 lbf, 2.24 g in this tube (#13).
- **Against the rule:** failure ld/pin 2.33, first separation 2.87, chosen charge 3.16; the rule sits at 4.67 to
  4.86. Gas-law pressure: 9.5 psi failed, 11.8 psi separated. Per in³: 0.0049 g failed, 0.0061 g separated.
- **The per-in³ basis misses it the other way:** 1.4 times VTS-1's 0.0151 g/in³ asks 6.97 g here, 3.5 times the
  charge that separated it. Its drogue bay (28.25 in) opened at 3.3 g (#1); no failed shot is given there.

## Brackets that lack a length or a pin size

| source | bay | pins | failed g | worked g | step | rule clears the failure if |
|---|---|---|---|---|---|---|
| 4 in build thread [2, #93] | 4 in main, length not stated | 4, "84 lbs" total | 1.0, 1.5 | 2.0 | 1.33 | L > 7.4 in |
| Oregon State 2020 [3, slide 10] | 6.25 in ID, main | 4, about 36 lbf each | 5 | 6.5 | 1.30 | L > 14.4 in |
| Oregon State 2020 [3, slide 10] | 6.25 in ID, drogue | 5, about 36 lbf each | 3 | 4.5 | 1.50 | L > 6.9 in |
| Notre Dame 2018 [4, pp. 184-185] | 5.38 in aft tube, main | 4, size unstated | 4, weak | 5 | 1.25 | no pin size |
| Notre Dame 2019 [5, pp. 37, 108] | recovery tube | not stated | 4, leak | 5, sealed | 1.25 | not a pin test |

- **The length test:** the rule's charge is 0.000516 × 4.67 × n × F per inch of bay, so a failure m is cleared
  when L > m / (0.000516 × 4.67 × n × F). On Fetter's 4-40 to 2-56 values, Oregon State's would be 10.4 to 16.7 in
  and 5.0 to 8.0 in. **Inference:** a bay holding a 60 in canopy in a bag [2, #1] or an 11.7 ft toroid [3, slide
  5] is likely longer than these, but no source states it.
- **The 4 in bay** [2]: the 84 lbf is the builder's own figure, 21 lbf a pin, below Fetter's 31.0 lbf for 2-56. On
  Fetter's 2-56 value the bound falls to 5.0 in. Its drogue was friction fit and took 1.5 g.
- **Oregon State** [3]: the backups were 9.75 g and 6.75 g, 1.5 times the working charges, both successful
  (2020-02-29). The tube is 6.25 in inside; 6 pins held about 1 kN in the team's test; the 34 in upper airframe holds
  the main and the avionics bay [9, p. 15], so the main bay is under 34 in.
- **Notre Dame 2018** [4]: 4 g separated the main section "not with significant force", too weak to deploy the
  parachute (**Inference** from the wording); 5 g was satisfactory. Each separation had four pins (p. 62); the main
  sits in the 5.38 in aft tube (**Inference** from the layout, pp. 13, 61).
- **Notre Dame 2019** [5]: 4 g failed, gas escaping through unsealed wire holes; sealed with duct seal, 5 g worked.

## Does the rule clear every failure at 4 in and up?

Yes, wherever it can be computed: 3 failures with known pins and length, all below the rule.

- **Buffalo** (parent, 6.148 in): 3.33 g at ld/pin 3.54, cleared 1.32 to 1.37 times if one charge fired. Its log says
  main and backup fired; then it is no bracket and sits above the rule.
- **Rice** (parent, 4.0 in outside diameter): 0.6 g at ld/pin 2.34, cleared 1.99 to 2.07 times.
- **The 5.38 in bay** [1]: 1.62 g at ld/pin 2.33, cleared 2.01 to 2.09 times.
- **Without lengths:** the table's three failures with a pin count pass in any bay over 14.4 in (unconfirmed).
- **A mutation dropping the 1.4 margin** (rule 3.33 to 3.47) still clears Rice and the 5.38 in bay (1.42 to 1.49
  times) and misses Buffalo (0.94 to 0.98). So Buffalo is still the only row that pins the margin.

## What the least charge tracks

Over the 5 complete brackets, each least charge (geometric mean) is divided by five candidate scales.

| scale | TR-1 | VTS-1 | Rice | Buffalo | 5.38 in [1] | max/min | CV (population) | max/min without Buffalo |
|---|---|---|---|---|---|---|---|---|
| over m_pin (pins' load × length) | 2.91 | 2.89 | 2.71 | 4.04 | 2.59 | 1.56 | 0.17 | 1.12 |
| g per lbf of pins | 0.0135 | 0.0223 | 0.0112 | 0.0306 | 0.0194 | 2.74 | 0.36 | 2.00 |
| g per in of length | 0.093 | 0.092 | 0.087 | 0.259 | 0.124 | 2.98 | 0.50 | 1.43 |
| g per in² of bulkhead | 0.231 | 0.196 | 0.055 | 0.128 | 0.079 | 4.18 | 0.49 | 4.18 |
| g per in³ | 0.0256 | 0.0131 | 0.0069 | 0.0087 | 0.0055 | 4.69 | 0.61 | 4.69 |

- **Pins' load times length wins on these 5.** Its spread, 1.56 times, is the smallest; the next, per lbf of pins, is
  2.74. It is also the basis the rule already uses. **Inference.**
- **At 4 in and up alone** (Rice, Buffalo, the 5.38 in bay), volume ties it: 1.59 times against 1.56. Three rows
  can't separate the two. **Inference:** Fetter's small fixtures are what make volume fail.
- **Pin forces matter:** these use Fetter's nominal values. Rice's checklist names 4-40 pins, which cut its value 38%
  (parent note). **Inference:** the force basis's spread would then widen to 2.40 times (1.72 without Buffalo).
- **What it says about the factor:** the least charge sat at 2.6 to 4.0 times m_pin. The rule's 4.67 to 4.86 is 1.16
  to 1.88 times those least charges; the 1.16 is Buffalo's.

## Charges that broke a bay

- **NC State 2018** [6, p. 57]: the first full-scale test tore the nose cone's bulkhead out and pushed the payload
  bulkhead free, with structural damage. The charge was sized for the empty bay, "6(+)" g; the team's formula gives
  6.55 g (**Inference**: 6 to 6.55 g). The bay was 7.34 in by 20.25 in, 857 in³ gross, with four 4-40 pins (p. 46).
  That is 13.6 to 14.8 psi on the gross volume, about 570 to 630 lbf on the bulkhead, ld/pin 2.88 to 3.15. The rule
  gives 9.7 to 10.1 g. The team then sized 4 g on the free length.
- **Texas Tech 2018** [7]: a 3.5 g main charge ruptured the 6 in Blue Tube airframe at its seams (pp. 51, 54). The
  team's own estimate is 48.5 psi on its 140 in³ effective volume (pp. 52-53), about 1,360 lbf on a 5.973 in bore.
  Pins for that bay are not stated, so the rule can't be sized there. It then chose 1.5 g, "about 20 psi" (p. 55).
- **Inference:** both breaks are joints and tubes, which vary build to build. They show the rule's charge can exceed
  a breaking one; they don't give a breaking ld/pin that would hold for another bay.

## What was read, and what held nothing

- **Student documents (31):** 22 new downloads, 4 cached ones re-read and 5 team-wiki pages. With a bracket: Oregon
  State's deck [3] and Notre Dame's 2018 and 2019 FRRs [4, 5].
- **No bracket:** NC State's 2015 to 2019 FRRs, 2015 to 2017 and 2019 PLARs, 2019 and 2021 addenda, 2025 and 2026
  PLARs (13, not read by the parent) [8]: first-try or chosen charges. Purdue 2019 FRR and PLAR, Maryland's 2023
  Spaceport America Cup report (first test clean), Penn State 2017 (charges cut, not raised; a printed coupler broke
  at an unstated charge, pp. 16-17), Windward CC's 2013 FRR and PLAR, Oregon State's report excerpt and drawing [9],
  and Texas Tech 2018 [7].
- **Not black powder or not a bay:** FAMU-FSU 2024 (CO₂), MIT's Hermes piston tests (wiki), Stanford's wiki.
- **Forum:** 129 threads titled with "ground test", "ground testing" or "ejection test" were listed on 2026-11-02. A
  text screen of up to 3 pages each (144 pages) kept posts with a gram charge, a failure word and a size of 4 in or
  more: 15 threads, all read, none a bracket at 4 in and up. Eight full-text searches gave 90 posts, 6 at 4 in and
  up, none a bracket. Four threads outside the screen were read whole: the 2 brackets [1, 2], a 3 in bay's and the
  parent's failed 3.9 in bay (id 51470), whose #15 reports a later success after the pins and vent changed, so no
  bracket. A text screen can miss.

## What this suggests

- **#857's table:** add the 5.38 in bay [1] as a failure the rule must exceed (1.62 g against 3.25 to 3.38 g).
  Expected effect: a second complete failure above 5 in; Buffalo's 1.32 still catches a dropped margin.
- **M6.7's scaling:** carry Fetter's factor by m_pin (pins' load times length), as the rule does. Expected effect:
  least charges span 1.56 times in m_pin against 4.69 per in³, and the rule sizes 1.16 to 1.88 times them.
- **M6.7's other side:** show the bulkhead load (gas-law pressure × area) beside the charge, since one bay broke
  below the rule's size. Expected effect: a reader sees 570 lbf before firing, not after; in [1] the rule's 434 to
  452 lbf is above the builder's own 300 lbf cap. No cap: no source gives a breaking load that carries between
  builds.
- **M6.8's log:** add the nose cone's fit (loose, taped) and each changed item between shots; [1] and [5] each
  changed a leak path mid-series, which a charge-only log would read as a pin bracket.

## Open questions

- **Buffalo decides the margin.** If both its charges fired, the closest complete failure sits 1.99 times under the
  rule (Rice), and nothing at 4 in and up tests the 1.4 margin.
- **Lengths:** the 4 in thread [2] and Oregon State [3] would make 3 more complete brackets with a bay length; Oregon
  State's main bay is only known to be under 34 in.
- **Not reached or searched:** Sport Rocketry (members' issue archive, no full-text index found); rec.models.rockets
  (one web search, no row); Spaceport America Cup reports after 2018 beyond Maryland's and Oregon State's; the
  University of Hawaii's current pages. All brackets are at sea level or near it.

## Sources

All read 2026-11-02. None states a license: cite only. Student documents are cached under the gitignored research
cache; forum posts are cited, not copied. Forum threads are at `https://www.rocketryforum.com/threads/<id>/`.

1. Rocketry Forum, "Ground test" (2020), id 162965, posts #1, #5, #7 to #10, #13, #18, #27, #39, #42.
2. Rocketry Forum, a 4 in fiberglass build thread (2017), id 138256, posts #1, #93 (2017-04-02).
3. Oregon State ESRA 30K team, "Aerodynamics and Recovery" final presentation (2020), slides 4, 5, 10. The file
   `esra_aerodynamics_and_recovery_final_presentation_0_0.pdf` under
   `https://events.engineering.oregonstate.edu/sites/expo.engr.oregonstate.edu/files/` is gone (404); read in the
   Wayback snapshot `https://web.archive.org/web/20240715220138/<that URL>`.
4. Notre Dame Rocket Team, FRR 2018, pp. 13, 61, 62, 184-185.
   https://seniordesign.ee.nd.edu/2018/Design%20Teams/rover/documents/Univ-of-Notre-Dame-2018-FRR-Report.pdf
5. Notre Dame Rocket Team, FRR 2019, Table 7 p. 37, p. 108.
   https://seniordesign.ee.nd.edu/2019/Design%20Teams/uav/NDRT_FRR_2019.pdf
6. NC State (Tacho Lycos), 2018 FRR, §4.3.6 p. 57, pins p. 46.
   https://ncsurocketry.org/wp-content/uploads/2019/09/nc-state-university-2018-frr-report.pdf
7. Texas Tech University, 2018 FRR, pp. 6, 12, 51 to 55.
   https://raideraerospace.github.io/assets/texas-tech-university---2018---frr---report.pdf
8. NC State, media library listed 2026-11-02 (151 PDFs) at `https://ncsurocketry.org/wp-json/wp/v2/media`; files
   under `https://ncsurocketry.org/wp-content/uploads/2019/08/`, `2019/09/`, `2019/10/`, `2021/04/`, `2025/06/`,
   `2026/05/`.
9. Purdue SEDS, `https://purdueseds.space/student-launch/project-walker/`; Terrapin Rocket Team 2023,
   https://drum.lib.umd.edu/items/5555aa80-3f2d-480d-b7f9-b18af4293571 ; Penn State 2017 FRR,
   https://sites.psu.edu/psurocket/files/2018/04/ltrl_2017_frr_report-2hg35yg.pdf ; Windward CC 2013 FRR and PLAR
   under `https://laulima.hawaii.edu/access/content/group/7f13b953-c1b5-4532-00fe-072e1311e8a4/`
   (`USLI%20Web%20Pub%202013/FRR/FRR2013.pdf`, `.../PLAR/PLAR2013.pdf`); Oregon State's 2019-20 technical report
   excerpt (22 pages, p. 15) and drawing under the same expo path as [3]. FAMU-FSU 2024 FRR,
   `https://eng-web1.eng.famu.fsu.edu/me/` `senior_design/2024/team502/documents/NASA/FRR.docx`; MIT Rocket Team
   wiki, Hermes ground tests (2017),
   `https://wikis.mit.edu/confluence/display/RocketTeam/November+10%2C+2017+Hermes+Ground+Test`.
