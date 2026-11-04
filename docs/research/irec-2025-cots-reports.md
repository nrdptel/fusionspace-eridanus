# IREC 2025's 30k COTS reports: five public flights past Mach 1, no public log

**In short: every 2025 30k COTS team's technical report is public, and the five flights that topped 27,000 ft
all passed Mach 1 by a bound that needs no drag model. Searches found no public log, trace or design file, so
each could join [M2.3c7, faster than sound][m2-3c7] only as an apogee-only reconstruction, by a decision record.** ESRA
(the Experimental Sounding Rocket Association, which runs the International Rocket Engineering Competition, IREC)
publishes all 149 of its 2025 reports in one folder; its rules say every report is published after the event
[E2, E4]. The five are teams 204, 207, 208, 216 and 218, all on an AeroTech O5500X-PS. Their scored apogees, 27,541
to 29,541 ft, need a top speed of at least 364 m/s even with no drag at all: Mach 1.03 or more on a 35 °C day
(**Inference**, §3). Their own predicted top Mach runs 1.4 to about 1.9, 0.6 to 1.1 below Mach 2.5 (§4). Two of the
five carry active air brakes whose flight use is unpublished (§2), so their apogees test drag only if the brakes
stayed shut. With [RASAero II's four][rcf], the floor of 5 past Mach 1 is within reach on one rule.
**Confidence:** high for where the reports are, the terms, the scored apogees and the Mach 1 bound. Medium for the
design numbers: two reports contradict themselves (§5). Low for the top Mach: no flight's speed was measured in
public. **What it moves:** M2.3c7's floor of 5 scored COTS flights past Mach 1, and `accuracy.supersonic`.

**What this covers.** ESRA's 2025 IREC page, its documents folder, rules, test guide and copyright form [E1 to E6];
the final scoresheet [E3]; the 2025 reports of teams 204, 208, 216 and 218 in full, and of seven lower 30k COTS
rows in brief [T200 to T224]; team and university pages [N1 to N4]; all read 2026-11-03, no GPL or AGPL source
opened, nothing flown. It builds on [Vayuvega][vv] (team 207, not repeated here), [RASAero II's comparison
flights][rcf] and [ADR-236][adr-236]. **Stated** marks what a source says; **Inference** what this note works out.

[m2-3c7]: ../decisions-and-roadmap.md#m2-3c7
[vv]: vayuvega-supersonic-cots.md
[rcf]: rasaero-comparison-flights.md
[adr-236]: ../decisions/0236-the-2026-10-11-review-supersonic-flights-from-published-comparisons.md

## 1. Where the reports are, and under what terms

- **The folder.** The 2025 IREC page's "2025 Technical Documents" link is a Dropbox folder [E1]. Dropbox serves it
  as one 2.68 GB zip of 149 PDFs named `Team_<id>_Technical_Report_2025_IREC.pdf`, all dated 2025-06-18 [E2].
  It holds no posters, design files (`.ork`, `.rkt`, `.CDX1`), logs or spreadsheets (counted with `unzip -l`).
- **Coverage.** The scoresheet has 21 rows in class "30k - COTS" (counted), and every one has a file [E2, E3].
  Team 216's file is a one-page letter linking a 200-page report on Google Drive, still downloadable [T216].
- **The rules' terms.** Rules v1.6 §2.6.2.1 (PDF p. 19): "All reports will be publicly published after the event
  concludes. Petitions for confidentiality will not be accepted." [E4] The rules themselves carry a Creative Commons
  4.0 license (p. 1); that covers the rules, not the teams' reports. The test guide has no publication clause [E6].
- **The copyright form** on the page is AIAA's assignment form for conference papers, not for these reports [E5].
- **The reports' own terms.** None of the four read in full has a license or copyright line. **Team 208's report is
  marked "Do Not Share"** on p. 1 and asks readers not to publish its contents without permission, though ESRA
  published it under the rule above [T208]. **Inference:** treat all as all rights reserved. Cite single facts with
  pages; keep any design built from them under `refs/`; publish only error statistics. This note gives 208 only
  the figures its university's news release also states, plus its motor, predicted speed and one plotted Mach
  (§4). The scoresheet and the news and team pages state no terms either: cite them, cache under `refs/` only.

## 2. The five scored flights

Scoresheet values [E3]; the error is (scored − team's scoresheet prediction) / prediction (**Inference**).

| Team | School | Scored ft | Predicted ft | Error | Damage | Air brakes | Public log |
|---|---|---|---|---|---|---|---|
| 204 | Chulalongkorn University | 27,541 | 29,469 | −6.5% | None | yes, after burnout | none |
| 207 | Manipal Institute of Technology | 29,432 | 30,198 | −2.5% | None | no | none |
| 208 | Mississippi State University | 28,639 | 30,083.9 | −4.8% | Minor | no | none |
| 216 | The University of Melbourne | 28,460 | 30,000 | −5.1% | Excessive | yes, after burnout | none |
| 218 | University of Texas at El Paso | 29,541 | 27,817 | +6.2% | None | no | none |

- **Air brakes.** 204's brake opens only after burnout and below Mach 0.8, trimming a planned 1,000 to 1,500 ft
  overshoot ([T204] pp. 9, 45). 216's report predicts 32,207 ft, a deliberate overshoot for its brakes, also barred
  above Mach 0.8 ([T216] pp. 40, 82). Neither says whether they deployed at IREC. Both climbs past Mach 1 are passive.
- **"Excessive Damage"** grades the recovered airframe, not the altitude. Of the scoresheet's 39 rows with that
  status, 15 still score altitude, 216 among them (counted) [E3]. **Inference:** 216's apogee stood as read.
- **Other sources agree.** MSU's news release gives 28,639 ft and says weather moved 208's launch angle from 4° to
  6° [N1]. UTEP's pages say 218 reached Mach 1.5 and 20 g, without saying how that was measured [N2, N3].
- **Logs.** GitHub and team-site searches found no published log, trace or design file. 204, 216 and 218 flew COTS
  recorders: a TeleMega, two Blue Ravens and two TeleMegas ([T204] p. 50; [T216] p. 65; [T218] p. 16).
  The rules make each team's COTS barometric altimeter log the official apogee ([E4] §2.5, p. 16).

## 3. All five passed Mach 1, whatever the drag

While the motor burns, height gained is at most v_max·t_b. After burnout the vertical speed falls at least as fast
as gravity alone would slow it, so the coast adds at most v_max²/(2g). So the apogee h obeys

  h ≤ v_max·t_b + v_max²/(2g),

and v_max is at least the positive root. This needs no drag model; drag, air brakes and launch angle all lower the
apogee for a given v_max, so they only raise the bound's true value. Inputs: t_b = 4.0 s (the O5500X-PS burn,
[T216] p. 5 and [T218] p. 4), g = 9.807 m/s², and the scored apogee as height above the pad. The speed of sound
is 340.3 m/s at 15 °C and 351.9 m/s at 35 °C; warmer air only lowers the Mach, and 35 °C is a hot-day
ceiling: pulling 204 below Mach 1 would take air near 56 °C at top speed, even with a 4.5 s burn
(**Inference**, worked here).

| Team | Apogee (m) | v_max ≥ (m/s), 4.0 s | Mach ≥, 15 °C | Mach ≥, 35 °C | Mach ≥, 35 °C and 4.5 s |
|---|---|---|---|---|---|
| 204 | 8,394 | 368.4 | 1.08 | 1.05 | 1.03 |
| 207 | 8,971 | 382.1 | 1.12 | 1.09 | 1.07 |
| 208 | 8,729 | 376.4 | 1.11 | 1.07 | 1.06 |
| 216 | 8,675 | 375.1 | 1.10 | 1.07 | 1.05 |
| 218 | 9,004 | 382.8 | 1.12 | 1.09 | 1.08 |
| 222 | 6,516 | 320.4 | 0.94 | 0.91 | 0.90 |

Worked example, 216: 28,460 ft is 8,675 m; v²/19.61 + 4.0 v = 8,675 gives v = 375.1 m/s, which is 375.1 / 351.9 =
Mach 1.07. The bound holds if the burn runs to 4.5 s (last column). It fails only if the scored altitude is not
height above the pad, or the altimeter over-read by more than about 3%; in air warmer than standard, as at
Midland in June, a barometric altimeter tends to under-read (**Inference**). The bound is weak by design: each
team's own prediction is Mach 1.4 to about 1.9 (§4). For team 222 (University of North Dakota, 21,377 ft), it
can't decide.

## 4. Predicted top Mach, against 1.0 and 2.5

The top speed is each report's prediction; the Mach range converts it at 340.3 to 351.9 m/s where the report gives
no Mach (**Inference**). No report states a measured top speed for its IREC flight.

| Team | Predicted top speed | Mach | Tool | Above 1.0 by | Below 2.5 by | Source |
|---|---|---|---|---|---|---|
| 204 | 588 m/s | 1.67 to 1.73 | not named for the table | 0.67 to 0.73 | 0.77 to 0.83 | [T204] p. 75 |
| 207 | see the Vayuvega note | 1.64 to 1.71 | as there | 0.64 to 0.71 | 0.79 to 0.86 | [vv] §5 |
| 208 | 644.8 m/s | about 1.9 (plotted) | RASAero II (p. 34) | about 0.9 | about 0.6 | [T208] pp. 40, 64 |
| 216 | 562.05 m/s | 1.7 stated; 1.60 to 1.65 | RASAero | 0.6 to 0.7 | 0.8 to 0.9 | [T216] pp. 1, 82 |
| 218 | 1,924 ft/s (586 m/s) | 1.4; 1.67 to 1.72 | OpenRocket (1.4) | 0.4 to 0.72 | 0.78 to 1.1 | [T218] pp. 10, 26 |

- **One plotted flight Mach.** 208's Fig. 54 plots "real flight data as well as simulated flights" ([T208] p. 40).
  Read by pixel, the trace labeled TL1, probably its test flight, peaks near Mach 1.89 and the simulations near
  1.91 (**Inference**). No page names the flight or recorder, and the axis is not height above the pad.
- **218's 1.4** is stated "at its maximum acceleration" ([T218] p. 10), not at top speed; its own 1,924 ft/s gives
  1.67 to 1.72. The 1.5 claimed after the flight [N2] sits between.
- All five sit well inside the core band (Mach 0 to 2.5): at least 0.4 above Mach 1 and 0.6 below 2.5.

## 5. Design inputs: given and missing

- **204** gives 2,971 mm overall, a body stepping from 157 to 127 mm, a 70 cm von Kármán nose, four clipped-delta
  fins 6 mm thick, 35.83 kg at liftoff, CG, CP and a 5.18 m rail ([T204] pp. 74 to 77). Missing: the tail cone's
  aft diameter, the fin station, the CG and CP datum, the launch angle, inertia and the brake log.
- **208** is 140 in long and about 69 lb by its university's release [N1]; the report gives the full geometry,
  masses, and a CG and CP from RASAero II. Missing: the nose length, fin station, the day's mass trim, the 6°
  angle's azimuth and inertia. Its terms are the strictest (§1).
- **216** gives 3,076 mm by 145.2 mm, a 726 mm LD-Haack nose, four clipped deltas, a 250 mm conical boat tail,
  38.8 kg at liftoff, CG and CP ([T216] p. 82). Conflicts: fin thickness 5.65 or 8.00 mm (pp. 9, 82), semispan
  129.8 or 119.7 mm, aft diameter 108 or 112 mm (pp. 10, 192). Missing: burnout CG, inertia and the brake log.
- **218** gives a 157 mm diameter, a 5:1 tangent-ogive nose, three aluminum trapezoid fins, CG, CP and a
  17 ft rail ([T218] pp. 3 to 26). Conflicts: "single diameter" (p. 3) against a 4.44 in lower airframe (p. 26),
  two fin sizes (pp. 11, 26), a length of 345.4 cm or 126 in, and a mass of 80.6 lb (p. 26) or 90 lb (team page
  [N2]). Missing: burnout CG, launch angle and inertia.

**Inference:** for a reconstruction, 207 and 208 are the cleanest (no brakes, one geometry each), 216 next (one
value per conflict to choose; its brakes may have flown), then 204 (brakes designed to deploy) and 218 (geometry
too conflicted to fly without guessing). Every one needs launch-day weather, which the project's weather sources
could supply for Midland, Texas, 9 to 14 June 2025 [E1].

## 6. The lower 30k COTS rows

Seven more rows reached above 1,000 ft [E3]; all their reports are in the folder, none with a log [T200 to T224].
Their predicted top Mach: 222 1.62, 220 1.9, 201 about 1.8, 203 1.80, 224 1.71 to 1.77, 214 1.97, 200 about 1.75.
No source says any of them passed Mach 1 in flight. 222's own Mach 1.62 suggests it did; §3's bound can't decide.

- **Texas A&M (214)** logs a pre-IREC test flight in its report: 1,426 ft/s measured by its TeleGPS against
  RASAero II's 1,414.5 ft/s, and 14,062 ft against 15,069 ft ([T214] p. 19 Table 15). That is a measured top
  speed of Mach 1.24 to 1.28 (**Inference**, at 351.9 to 340.3 m/s), with RASAero II's prediction beside it,
  though no log is public.
- **Ateneo de Davao (200)** broke up shortly after top speed when the motor's forward closure came loose [N4].

## 7. What this suggests

1. **One decision record for IREC flights.** Issue #724 asks for a record admitting Vayuvega as an apogee-only
   flight. The same record could admit the IREC 2025 30k COTS class on one rule: a public report, a scored apogee
   above 27,000 ft, the §3 bound past Mach 1, and the stated-assumption list of [the Vayuvega note][vv] §7. That
   yields five flights past Mach 1, enough for the floor alone, all in the Mach 1 to 2.5 band.
2. **Fly 207 and 208 first**, then 216 with its conflicts recorded as assumptions; flag 204 and 216's apogees
   "air brakes, use unknown" and hold them out of drag statistics unless a source shows the brakes stayed shut.
3. **214's test flight** is the class's only measured top speed with a RASAero II prediction beside it (0.8%
   under the measurement); a design rebuilt from its report could score top speed too, if the record allows it.
4. **IREC's 2024 and 2026 folders** may hold more such flights under the same rule.

## Open questions

- Did 204's and 216's air brakes deploy at IREC? Neither report (both written before the flight) nor any page says.
- Which scoresheet prediction did each team submit, and when? 204's 29,469 ft differs from its report's 29,948 ft,
  and 218's 27,817 ft from its report's 30,000 and 25,708 ft.
- How were 218's Mach 1.5 and 20 g measured, and which recorder fed 208's plotted test-flight Mach?
- How far did each official altimeter read from true height at 9 km? A 3% over-read would still leave all five
  above Mach 1 by §3; Melbourne's report shows a Blue Raven reading about 600 m below GPS in 2024 ([T216] p. 13).

## Sources

ESRA files E4 to E6 are under `https://www.soundingrocket.org/uploads/9/0/6/4/9064598/`. All read 2026-11-03;
sha256 prefixes from the copies cached under `refs/research/irec-2025-cots-reports/`.

1. **E1.** ESRA, 2025 IREC page, <https://www.soundingrocket.org/2025-irec.html>, `969cad1f9265c143`.
2. **E2.** ESRA, "2025 Technical Documents" Dropbox folder (E1's link), served as `2025 IREC Technical
   Reports.zip`, 2,684,255,717 bytes, `e3732281b42bd8dc`.
3. **E3.** ESRA, final scoresheet `irec2025_final_scoresheet_8-2-25.xlsx`, posted 2025-08-02, `b3bf5fd20ec0b043`.
4. **E4.** ESRA, IREC Rules & Requirements v1.6 (2025-03-04), `irec_rules_and_requirements_document_v_1.6_.pdf`,
   `fc0b5faef49bff9e`.
5. **E5.** AIAA, IREC 2025 copyright form,
   `esra_competition_student_meeting_paper_copyright_form_2025.docx`, `b31047a9f57f1cdd`.
6. **E6.** ESRA, Design, Test & Evaluation Guide v1.1.6 (2025-02-14), `2025-irec_dteg_v1.1.6_02-14-25.pdf`,
   `5dfff3a5b68d2e8a`.
7. **T204.** Chulalongkorn University, Team 204 report (CURSR-IV), from E2, 129 pp., `711130195eae0a99`.
8. **T208.** Mississippi State University, Team 208 report (Helios), from E2, 152 pp., `1d0ae588de5f64e6`.
9. **T216.** The University of Melbourne, Team 216 report (Lemaire), 200 pp., Google Drive file
   `18GaLbGtIHv3Wl2qTPgC4HkdHBf9NtaMu` linked from E2's one-page letter, `58af6474ce98`.
10. **T218.** University of Texas at El Paso, Team 218 report (Sandstorm), from E2, 156 pp., `dd0b51123153`.
11. **T200 to T224.** Teams 200, 201, 203, 214, 220, 222, 224, reports from E2. 214 (Texas A&M): `a1e5245d3579`.
12. **N1.** Mississippi State University news, 2025-07-03, <https://www.msstate.edu/newsroom/article/2025/07/>
    `msu-space-cowboys-rocketry-team-wins-challenge-2025-international-rocket`.
13. **N2.** UTEP Mechanical Engineering teams page, <https://www.utep.edu/engineering/mechanical/teams/index1.html>.
14. **N3.** UTEP news, 2025-07-07, <https://www.utep.edu/newsfeed/2025/july/>
    `utep-rocket-team-places-second-at-international-rocket-competition.html`.
15. **N4.** Atenews, 2025-07-02, <https://atenews.ph/>
    `addu-rocketry-team-remains-grounded-despite-mid-flight-rocket-failure-in-intl-tilt`.
