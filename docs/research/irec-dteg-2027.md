# What IREC's 2027 design guide changes for control, flutter and stability

Which limits in §5 to §11 of the IREC (Intercollegiate Rocket Engineering Competition) Design, Test & Evaluation
Guide (DTEG) change from 2026 V1.1 to 2027, by section and page? It serves [M6.4](../ROADMAP.md)'s done-when
(airbrakes), [#417](https://github.com/nrdptel/fusionspace-eridanus/issues/417) (airbrake presets),
[#881](https://github.com/nrdptel/fusionspace-eridanus/issues/881) (citations of the removed rules) and M6.3's IREC
preset and its stale-after date. It follows [rules-2027](rules-2027.md) §1, which gives the 2026 limits and ESRA's
2026-09-05 announcement. Read 2026-11-04 from ESRA's and Tripoli's own pages and PDFs; **Inference** marks this
note's own reading. Pages are printed footers: "Page N of 65" in 2027, "Page N of 58" in 2026.

**In short.**
- **The 2027 DTEG is out:** revision 2027 REV P1, effective 2026-10-15 (source 1), with a redline of its draft,
  2027 V1.0 of 2026-10-14 (source 2). ESRA's news page has posted nothing since 2026-09-05.
- **Control loosens, fail-neutral stays.** The 30° tilt shutoff and the whole boost-dormancy section (burnout,
  max Q, 2,000 or 6,000 m) are gone. Control must still go neutral on an abort or a power loss, and neutral still
  reads "aerodynamic surfaces trimmed or retracted" (D&T-7.3.1 to 7.3.2, p. 35). So brakes still retract on a fault.
- **Flutter has no number:** teams calculate and report it (D&T-8.2.2, p. 37); the 1.5 times margin is gone and
  nothing replaces it.
- **Stability keeps its numbers but not its window.** The 7.5% and 10% minimums now hold from launch through motor
  burnout, not to the first deployment (D&T-10.2.1, p. 43). ESRA's post had promised no stability change, and the
  revision log omits it.
- **Rail exit and descent numbers hold** (25 m/s; 457 m; 20 to 40 m/s; under 11 m/s), but the 890 m site elevation
  and the weather appendix are gone: the guide says the 2027 site isn't chosen. Each 100 m of site elevation moves a
  computed descent rate by about 0.49% (**Inference**, standard atmosphere).
- **Tripoli's code is Version 2, effective January 2026;** no newer version or notice was on tripoli.org on
  2026-11-04. ESRA expects a Tripoli update in early 2027 that may change the DTEG again.

## 1. What is posted, and when

- **Documents page** (source 3), read 2026-11-04: "2027 IREC Design, Test and Evaluation Guide REV P1, Last
  Updated: October 15, 2026", and a redline link labeled "2026 To 2027 IREC DTEG V1.0". The DTEG no longer sits
  under "Work-In-Progress For 2027"; that label now marks other files, such as the band plan and shipping guides.
- **The clean copy** (source 1): title page "Revision: 2027 REV P1, Effective Date: October 15, 2026"; 65 pages;
  PDF made 2026-10-19. Section numbers §5 to §11 match 2026's; each paragraph now carries a "D&T-" prefix.
- **The redline** (source 2): "Revision: 2027 V1.0, October 14, 2026", 72 pages, cut 2026 words beside the new. Every
  limit change below appears in both copies, the stability window included (redline p. 37).
- **The revision log** (App. F, pp. 64 to 65) lists 19 changes. It names 7.3.1, the removed 7.4, 8.2.2 and the
  removed Appendix G. It does not list 7.2.2 or 10.2.1. It dates itself "10/15/2027" and calls the tilt limit
  "30%"; the 2026 text said 30° (§7.3.1, p. 24).
- **The R&R** (source 4) moved too: REV P1, effective 2026-10-15, replaces v1.0 of 2026-10-12. The payload minimum
  is now 4.4 lb in all three places (R&R-3.1.1, p. 7; 3.4.1.1 and 3.4.1.5, p. 9), settling v1.0's 4.4 or 5.0 lb.
  Table 3-1 (p. 7) splits hybrid from liquid: 10 scored categories, a new SRAD Liquid 5K aimed at 5,000 ft AGL and
  no liquid 30K. The score formula (R&R-6.6.1, p. 28) and the predicted-flight list (Table 4-1, R&R-4.4.10.3, p. 20)
  are unchanged. v1.0 had 8 scored categories plus demonstration flights.
- **News** (source 5): 34 posts listed on 2026-11-04 (two pages of the JSON feed); the newest is still "DTEG
  Updates", 2026-09-05. No FAQ page was found (esrarocket.org/faq returns 404); ESRA points teams to its Discord.
- **IMS** (source 6): REV P1 of 2026-09-21 is unchanged (same file as cached on 2026-10-18). It lists no DTEG date;
  the PDR is due 2026-12-14, and its event schedule runs June 12 to 19, 2027.

## 2. §5 to §11 limit by limit

| Limit | 2026 V1.1 | 2027 REV P1 | Change |
| --- | --- | --- | --- |
| Rail exit at least 25 m/s at effective length | 5.3.1 to 5.3.2, p. 12 | 5.3.1 to 5.3.2, p. 19 | none |
| Team pad: use its own rail length | none | 5.3.3, p. 19 | new |
| Rails 17 ft single stage, 21 ft two-stage | 11.1.2 to 11.1.3, p. 31 | 11.1.2 to 11.1.3, p. 45 | none |
| Air start past 70% of sim altitude, within 20° | 5.13.1, p. 14 | 5.13.1, p. 22 | none |
| Main opens no higher than 457 m AGL | 6.4.1, p. 18 | 6.4.1, p. 27 | none |
| Drogue 20 to 40 m/s at main deployment | 6.3.3, p. 18 | 6.3.3, p. 27 | none |
| Landing under 11 m/s | 6.4.3, p. 18 | 6.4.3, p. 27 | none |
| Descent computed at 890 m MSL | 6.3.2, 6.4.2, p. 18 | "the launch site elevation", fn. 9, p. 27 | removed |
| Site weather (wind 11.4 to 11.8 mph mean) | App. G, p. 49 | none (App. F, p. 65) | removed |
| Control only for stability or braking | 7.1.1, p. 24 | 7.1.1, p. 35 | none |
| Attitude control may hold a vertical path | none | 7.2.2, p. 35 | new |
| Stable with control inert | 7.2.1, p. 24 | 7.2.1, p. 35 | none |
| Neutral on abort or power loss | 7.3.1, p. 24 | 7.3.1, p. 35 | none |
| Neutral past 30° from launch elevation | 7.3.1, p. 24 | none | removed |
| Neutral means "trimmed or retracted" | 7.3.2, p. 24 | 7.3.2, p. 35 | none |
| Dormant to burnout, max Q, 2,000 or 6,000 m | 7.4, p. 24 | none | removed |
| Flutter speed at least 1.5 × peak speed | 8.2.2, p. 26 | calculate and report, 8.2.2, p. 37 | removed |
| Head-end coupler overlap at least 0.5 caliber | fn. 9, p. 27 (2026 only) | 8.5.1, p. 38 | standing |
| Least margin 7.5% sub, 10% supersonic, of length | 10.2.1, p. 30 | 10.2.1, p. 43 | window |
| "Supersonic": at any point in the flight | none | 10.2.1, p. 43 | defined |
| Fin span at least 0.8 calibers | 10.2.4, p. 30 | 10.2.4, p. 43 | none |
| Margin at most 18% at launch, 25% in flight | 10.3.1, p. 30 | 10.3.1, p. 44 | none |
| Elevation 84° ± 2° single, 87° ± 2° multistage | 10.1.1, p. 30 | 10.1.1, p. 43 | none |

Section numbers in the 2027 column carry the "D&T-" prefix in the source. Every other §5 to §11 edit found by a
paragraph-by-paragraph comparison of the two texts is wording, bullets, propellant, radio, wiring or ground
equipment, none a flight number. One example: ammonium nitrate is now banned in SRAD solids (5.5.4, p. 20). One
logic change: an air-start inhibit now also checks "flight angle criteria", not only altitude (5.13.2, p. 22).

## 3. Active control (§7)

- **Boost dormancy is gone.** 2026 §7.4 kept control neutral until the first of burnout, max Q, or 2,000 m AGL
  (10K) or 6,000 m (30K, 45K). The 2027 §7.4 is now the old §7.5, electronics (p. 35). Sign: a brake may now act
  in the burn. Brakes only add drag, so the reachable apogee can only fall or stay the same. Size: unbounded here;
  it depends on the design and the controller.
- **The 30° shutoff is gone.** 2027 D&T-7.3.1 (p. 35) names only an abort signal or primary power loss. Sign: a
  brake may stay out past 30° of tilt. Size: unbounded; it matters only in a flight that tilts that far.
- **Brakes still retract on a fault.** D&T-7.3.2 (p. 35) keeps "neutral" as a state that applies no moments, with
  "aerodynamic surfaces trimmed or retracted" as the example. **Inference:** retracting is the only reading that
  matches the example. Symmetric brakes held out apply no moment only at zero angle of attack, so the text doesn't
  settle whether extended counts. EuRoC's 2026 rule is the opposite: fail fully extended ([rules-2027](rules-2027.md)).
- **A new allowance:** D&T-7.2.2 (p. 35) adds that an attitude control system may be used to achieve a vertical
  flight path. It still allows no guidance to a target (7.1.2) and no control a rocket needs to be stable (7.2.1).
  This matters to M6.5's roll and canard work more than to drag-only brakes.

## 4. Flutter (§8.2)

- **The number is gone.** 2026 §8.2.2 (p. 26) required a flutter speed at least 50% above the peak expected speed.
  2027 D&T-8.2.2 (p. 37) asks teams to calculate it and report it in progress reports. The shear-modulus note
  (8.2.3) stays. ESRA's reason (source 5): the methods vary widely. Safety reviewers may still require fin changes.
- **The log misstates the old rule.** App. F (p. 65) says it removes a need for flutter speed "equal or greater
  than the expected maximum flight velocity"; 2026's actual bar was 1.5 times that speed.
- **Sign:** looser for teams. No printed pass mark remains at any ratio, below 1.0 times peak speed included. The
  R&R still asks for the fin-flutter velocity in the predicted flight data (Table 4-1, R&R-4.4.10.3, p. 20).

## 5. Stability (§10)

- **The window ends at burnout.** 2026 §10.2.1 (p. 30) held the minimum from launch "through the first recovery
  system deployment event"; 2027 D&T-10.2.1 (p. 43) holds it "from launch through motor burnout". The redline
  strikes the old words (p. 37). Sign: looser. A margin dip in the coast, from burnout to apogee, no longer fails
  the 7.5% or 10% minimum. Size: the coast-phase least margin of the design; unbounded here.
- **Where it bites (Inference).** A supersonic rocket's center of pressure usually moves aft as it slows, so its
  least margin usually falls in the burn and the change rarely binds. It binds where the margin falls in the coast,
  such as brakes forward of the center of gravity. The 25% ceiling still applies "through the flight" (p. 44).
  The stable-when-inert rule (7.2.1) still applies during ascent.
- **"Supersonic" is now defined:** a flight supersonic at any point uses the 10% minimum (D&T-10.2.1, p. 43).
  Sign: stricter for a flight that is briefly supersonic; it settles which threshold a preset applies.
- ESRA's post (source 5) said "No changes are proposed for ... stability rules". The guide changed the window
  anyway, and App. F doesn't list the change. Whether it was meant is an open question.

## 6. Site elevation and descent (§6)

- **The 890 m figure is gone.** 2026 §6.3.2 and §6.4.2 (p. 18) fixed descent calculations at 890 m (2,920 ft) MSL.
  2027 says "the launch site elevation" (p. 27). Footnote 9 (p. 27) says the 2027 location was not yet known, and
  App. F (p. 65) removes the weather appendix for that reason. The limits themselves (20 to 40 m/s, under 11 m/s,
  457 m) are unchanged.
- **The city is named elsewhere.** ESRA's 2027 page (source 7) says IREC runs in Midland, Texas, June 14 to 19,
  2027. It lists "2027 Spaceport Midland Launch Site Directions" without a link. R&R-4.7.2 (p. 24) times deadlines
  in Midland's Central Time. No document gives the launch site's elevation.
- **Size (Inference).** Descent rate scales as one over the square root of air density. In the 1976 standard
  troposphere (288.15 K at sea level, 6.5 K/km lapse; source 10), each 100 m above 890 m raises it 0.49%. A site at
  sea level would compute 4.2% lower than at 890 m; one 500 m higher, 2.5% higher. Real-day temperature moves it
  too.

## 7. Tripoli's safety code

- **Current:** Version 2, "Effective January 2026" (source 8). Tripoli posted it for early member review on
  2025-11-05 (source 9). The newest item on Tripoli's news page on 2026-11-04 is dated 2026-08-11, and none after
  2025-11-05 concerns the code.
- **What it limits:** thrust to weight typically at least 5:1, as low as 3:1 with the RSO's approval, below 3:1
  only with an active stability system (7-1.4). High-power launches are no more than 20° from vertical (13-6). Of the 4
  places "active" appears in the code's text, none sets airbrake dormancy, a tilt cutoff or a fault state.
- **Expected:** ESRA's post (source 5) says Tripoli expects to update the code in early 2027, and the DTEG may
  follow. DTEG footnote 13 (p. 37) says Tripoli is weighing a metals update; neither it nor the post dates that.

## What this suggests

- **M6.4's done-when keeps its bars as the project's own choices.** "Retracted through boost" and "retract past
  30°" are no longer IREC rules. Per #881, they stay as labeled project defaults; no done-when is loosened. A preset
  switch could turn them off for an IREC 2027 run. The abort and power-loss retract tests still cite IREC:
  D&T-7.3.1 to 7.3.2 (p. 35). The margin check from rail exit to apogee covers burnout to apogee beyond IREC's
  window, though not launch to rail exit; it stays valid as the project's own check.
- **#417 gains page citations:** its 2026-10-20 comment already read the post this way; REV P1 confirms that
  brakes go neutral on an abort or a power loss, "retracted" the example (p. 35). EuRoC's fail-extended state
  stays a separate preset; the M6.4 preset decision stays open there.
- **M6.3's IREC preset** can cite the R&R and DTEG REV P1 (both 2026-10-15): payload 4.4 lb; minimum margin to
  burnout, 10% if supersonic anywhere; rail exit 25 m/s on 17 or 21 ft. The site elevation should be a required
  input, with 890 m only as a labeled 2026 value; App. G's wind is no longer a 2027 default. The project's
  scope is COTS solids, so the new liquid 5K target need not enter yet.
- **Stale-after:** 2027-03-31 is suggested (**Inference**). That forces a recheck after Tripoli's early-2027 update
  and before the event (June 14 to 19 on ESRA's page). A newer DTEG revision on ESRA's page stales it sooner.
- **Flutter:** the IREC preset can report flutter speed against peak speed with no IREC pass mark. The project's
  own flutter check is a safety number whose flattering side is the worst error class, so it should not follow
  ESRA's margin down.
- **Effect on printed numbers:** none today. No shipped output uses these limits; they feed M6.3 and M6.4 checks.

## Open questions

- Whether a symmetric brake held extended is "neutral" under D&T-7.3.2. Only "trimmed or retracted" is named.
- Whether the burnout window in D&T-10.2.1 is intended, given the post and App. F's silence.
- The 2027 launch site's elevation and weather. Neither the DTEG nor the R&R gives them.
- When Tripoli's next code version lands, and whether it sets rules for active controls that the DTEG then adopts.
- The license of the REV P1 files: neither carries a license line (searched for "Creative", "Attribut", "Copyright").
  The 2026 V1.1 and the 2027 V1.0 redline say "Creative Commons 4.0 International", attributed to ESRA, no variant.

## Sources

All read 2026-11-04. ESRA files are under https://www.esrarocket.org/s/; file names are ESRA's. Cached under
`refs/` (gitignored) for reading only. Terms: ESRA's REV P1 files carry no license (cite
only); the redline says CC 4.0 with no variant (cite only). Tripoli's code is "Copyright ©2026", all rights reserved,
no reproduction without permission (cite only). The 1976 standard atmosphere is a US government work.

1. ESRA, IREC 2027 Design, Test, Evaluation Guide, REV P1, effective 2026-10-15, 65 pp.: `2027-IREC-DTEG-REV-P1.pdf`
2. ESRA, IREC DTEG 2027 V1.0 redline against 2026, 2026-10-14, 72 pp.: `2027-IREC-DTEG-REV-P1-REDLINES.pdf`
3. ESRA, Documents and Forms. https://www.esrarocket.org/documents-and-forms
4. ESRA, IREC 2027 Rules & Requirements, 2027 REV P1, effective 2026-10-15, 44 pp.: `2027-IREC-RR-REV-P1-tcda.pdf`;
   compared with v1.0, 2026-10-12 (`IREC-2027-Rules-and-Requirments-v10.pdf`, cached 2026-10-18 by rules-2027)
5. J. McCarty, "DTEG Updates", ESRA news, 2026-09-05. https://www.esrarocket.org/news/dteg-updates; news list
   https://www.esrarocket.org/news (JSON feed `?format=json`)
6. ESRA, 2027 IREC Integrated Master Schedule, REV P1, effective 2026-09-21: `2027-IREC-IMS-REV-P1.pdf`
7. ESRA, 2027 IREC page. https://www.esrarocket.org/2027irec
8. Tripoli Rocketry Association, Unified Safety Code, Version 2, effective January 2026. https://www.tripoli.org/safety
9. Tripoli Rocketry Association, news, item of 2025-11-05 and list through 2026-08-11. https://www.tripoli.org/News
10. NOAA, NASA, USAF, U.S. Standard Atmosphere, 1976, NOAA-S/T 76-1562. https://ntrs.nasa.gov/citations/19770009539
11. ESRA, IREC DTEG 2026 V1.1, effective 2026-02-23, 58 pp.: `2026-IREC-DTEG-V11.pdf` (cached 2026-10-18 by rules-2027)
