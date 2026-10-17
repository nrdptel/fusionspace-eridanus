# US export control and hpr: the rules, the record, and what hpr publishes

Re-checked 2026-10-17: 20 claims corrected.

**Bottom line:** the closest precedent puts a hobby simulator like hpr outside the Munitions List.
In 2020 the State Department ruled RockSim, a hobby rocket design and flight simulator, EAR99, the
lowest category. Published software is outside the Commerce rules, apart from encryption code and
firearm print files. Whether that covers hpr's controller models is open (question 4). The exposed
point is hardware: a rocket that carries "active controls (e.g., RF, GPS)" loses the Munitions
List's exclusion for hobby rockets. So firmware and hardware that move a control surface wait for a
ruling, and nothing that might be controlled is put on GitHub or given to a third-party cloud
service. Researched October 6, 2026, from the regulations, court records, DDTC's rulings and other
projects; sources re-read October 17, 2026. [ADR-192][adr-192] records hpr's response. **Not legal advice.**

## The two sets of rules, which treat publishing differently

- **ITAR (State Department)** covers the US Munitions List: defense articles and their
  technical data. *Rockets:* Category IV(a)(5), any rocket not listed elsewhere, no size floor.
  *Internet:* not named in its public-domain definition (22 CFR 120.34); State proposed in 2015
  that posting technical data without approval is a violation ([80 FR 31525][fr-2015]), and
  never settled it. *Research:* fundamental research only at accredited US universities.
- **EAR (Commerce Department)** covers dual-use items on the Commerce Control List, and almost
  everything else as EAR99. *Rockets:* rocket parts and GPS receivers (9A604.x, 7A105); sounding
  rockets with a 300 km range (9A104) are "subject to the ITAR". *Internet:* named. "Posting on
  the Internet on sites available to the public" makes software and technology published and
  not subject to the EAR (15 CFR 734.7(a)(4)). *Research:* any meant to be published (734.8).

**The hobby-rocket exclusion.** Category IV(a)'s Note 3 excludes model and high-power rockets
"made of paper, wood, fiberglass, or plastic containing no substantial metal parts and designed to
be flown with hobby rocket motors that are certified for consumer use. Such rockets must not
contain active controls (e.g., RF, GPS)" ([22 CFR 121.1][usml], text of 2026-10-13). It was
proposed in January 2013 (78 FR 6765) and took effect on July 1, 2014 (79 FR 34). Before 2014 the
category had no hobby exclusion at all. The final rule's preamble doesn't discuss hobby rockets,
but answers a question on sounding and research rockets: "all such rockets are controlled under
USML Category IV".

**What else in Category IV touches control.**

- IV(h)(1): flight control and guidance systems "specially designed" for a Category IV(a) rocket. 22 CFR
  120.41(b) can release a part, component, accessory, attachment or software from the list, not
  a system or end item; a commodity jurisdiction ruling (120.41(b)(1)) is the surest route.
- IV(h)(4): rocket thrust vector control, listed by name. No release test applies to it.
- XII(d)(1): navigation systems (INS, IRU, AHRS) only above set accuracy thresholds or rated
  beyond 25 g; inertial measurement units are in XII(e), rocket flight control in IV(h).

**The pending rewrite.** State proposed a new Category IV on October 23, 2024 ([89 FR
84482][fr-2024]). It defines an "amateur rocket" by six conditions: no substantial metal parts; no
harmful payload; 40,960 N·s total impulse or less; room for 5 lb of propellant or less; "Has no
active controls"; and unable to pass 150 km. The "(e.g., RF, GPS)" examples are gone; "active
controls" is still undefined. The docket (DOS-2024-0035) drew 848 comments. Among them, a Tripoli
member warned that the wording "could also unintentionally cover devices such as drag brakes"
(comment DOS-2024-0035-0060). AIAA's April 2026 paper asks to define active control as "targeting
functions against a designated target" ([AIAA][aiaa]). The regulatory agenda lists an interim final
rule, "USML Categories IV and XV", for September 2026; it has been under White House (OIRA) review
since June 24, 2026 ([reginfo.gov][oira], read 2026-10-17). As of October 17, 2026 nothing has been
published, and the eCFR (current to October 15) still shows Note 3.

**Storage can be an export too.** Storage in the US that only US persons can reach is not an
export. Beyond that, 22 CFR 120.54(a)(5) (84 FR 70887, effective March 25, 2020) gives a safe
harbor for sending or storing technical data that is all of: unclassified; end-to-end encrypted;
secured with FIPS 140-2 modules under NIST guidance, or by other means at least as strong as
AES-128; not intentionally sent to or stored in a country on the §126.1 list; and not sent from
one. Outside a FIPS 140-2 (or later) module, encryption weaker than AES-128 misses the harbor.
GitHub says GitHub.com "has not been designed to host data subject to the ITAR" and can't restrict
access by country ([GitHub][github]). It doesn't separate private repositories from public ones;
reading it to cover both is this note's inference, and a third-party cloud service gets the same
caution. Releasing technical data to a foreign person inside the US is an export to every country
where they hold or have held citizenship, or hold permanent residency (22 CFR 120.50, 120.56).

## How the rules have been applied

**No prosecution or penalty found involved a hobbyist or a hobby rocketry product.** DOJ's case
lists and BIS's enforcement booklet show three patterns: data from a military contract shared,
military-grade hardware smuggled, or a sale to a listed buyer.

DDTC's rulings, from its [ruling list][ddtc-list] (dated 2026-10-01):

- **RockSim v11.0, 2020-06-19:** a hobby rocket simulator, EAR99. The closest precedent.
- **UT Austin rocket-avionics thesis, 2020-12-17:** neither regime; a top-level description.
- **AeroTech (RCS) hobby motors, 2016-11-23:** Munitions List above 5 lb of propellant; at or
  below, "seek a CCATS" (a Commerce classification). The hobby line is drawn by size, in writing.
- **PRODAS v3.5, 2014-09-03:** ordnance design software ("all Types of Projectiles", ballistics),
  Munitions List IV(i). RockSim's hobby purpose is the clearest difference.
- **Cloud Cap Piccolo UAV flight controller and software, 2016-02-17:** Commerce (7A994, 7D994).
- **Embention's Veronte, 2025-02-10:** a "user-programable controller" for drones, ground
  vehicles and robots, "Seek a CCATS". **QuickShot v8.0.2** (trajectory optimization), EAR99.

Cases and settlements:

- **Roth, University of Tennessee, 2009:** 48 months ([DOJ][doj-list]) for Air Force drone data
  shown to two foreign graduate students. Access is the export; the court required only knowledge
  that the act was unlawful ([Sixth Circuit][roth]).
- **Soong, 2023:** 20 months for EAR99 aircraft software exported to Beihang University, on the
  Entity List ([88 FR 88565][soong]). Even the lowest category needs the buyer screened.
- **Hanson, 2010:** 24 months after a plea: UAV autopilots shipped to a Chinese aviation group under a "model
  airplane civilian flying club" story ([DOJ][doj-list]); a hobby label protects nothing.
- **Defense Distributed, 2013 to 2016:** State ordered CAD files offline; courts weighed security
  over speech ([Fifth Circuit][dd]). Posting online is no ITAR safe harbor.
- **FLIR, Honeywell, 3D Systems, Boeing, 2018 to 2024:** $30M, $13M, $20M and $51M settlements
  (State's media notes; [DDTC's list][ddtc-pen]) for data reached by foreign persons. Companies are
  fined for access, not just shipments.

## How others go about it

- **Student teams publish airbrake code openly:** the READMEs of 35 US and foreign student
  airbrake and roll-control repositories, NC State's MIT-licensed AirbrakesV2 among them, carry no export
  notice ([roll and drag control](roll-and-drag-control.md)). RocketPy's airbrake guide ships a
  controller ([docs](https://docs.rocketpy.org/en/latest/user/airbrakes.html)).
- **Non-US teams publish more,** canards included: Waterloo's roll-control canard firmware, EPFL,
  ARIS, CATS Vega. One US group did too: Portland State's rocket society flew and published canard
  roll control ([roll and drag control](roll-and-drag-control.md)).
- **Thrust vector control is what gets held back.** BPS.space sells its thrust-vectoring computer
  only to "US citizens and residents due to US export restrictions" ([Signal][bps]).
- **Universities split.** MIT's rocket team ran a technology control plan with locked storage and
  an access log ([wiki][mit]). Stanford's student space group stays inside fundamental research
  "by frequently publicizing information about group activities" ([SSI wiki][ssi]).
- **IREC's line** is the clearest in rocketry. Control only "for pitch and/or roll stability
  augmentation, or for aerodynamic braking", never "actively guided towards a designated spatial
  target or GPS location", and the rocket stable with the controls inert ([DTEG 2026 §7][dteg]).
  It calls itself "an ITAR-free event", US authors answering for their own work ([§3.0][irec-rr]).
- **Tracker sellers** (Featherweight, Eggtimer, Missile Works, Altus Metrum) publish no export
  statement. u-blox receivers of their kind stop at 500 m/s and 50 km ([NEO-M8][m8]) or 80 km
  ([MAX-M10S][m10]), in the chip's firmware; which chip each seller uses wasn't checked.
- **Drone autopilots** (PX4, ArduPilot) publish everything, with no formal export statement.
  ArduPilot adds a developer code of conduct against weaponization.
- **No rocketry project found** splits public algorithms from private tuned parameters, or checks
  citizenship before giving repository access.

## What hpr does

ADR-192 sorts the work into three tiers.

- **Public, in this repository:** the simulator, its airbrake, canard and controller *models*,
  design checks, the recovery and tracker logic within commercial GPS limits, the ground station,
  every format. *Why:* RockSim's EAR99 ruling; the EAR's published-software rule; what US teams
  and RocketPy already publish.
- **Held:** firmware or hardware that moves a control surface on a real rocket (drag-only brakes
  and roll-only surfaces since ADR-193). Not written for the project; if
  Neer writes any, on his own machine or in storage that meets the 120.54(a)(5) harbor, never on
  GitHub or in a third-party cloud service. *Why:* Note 3 and IV(h)(1); GitHub's own policy.
- **Never:** guidance toward a target or a GPS point, thrust vector control, active pitch or yaw
  control (ADR-193), code that lifts a GPS receiver's speed or altitude caps. *Why:* IREC's line,
  narrowed; IV(h)(4) has no release; 7A105 controls airborne GPS receivers that work above
  600 m/s (it has no altitude term; the altitude cap is kept as caution).

Three rules come with the tiers. A design with active control must be stable with the controls
inert (IREC §7.2.1), which [M6.4, airbrakes][m6-4] checks. hpr accepts no file from defense or
contract work, or with an export marking. Buyers of hardware or paid downloads are screened
against the [consolidated screening list](https://www.trade.gov/consolidated-screening-list).

**The way to move the held tier into the public one** is a commodity jurisdiction ruling: DDTC's
formal answer on whether an item is on the Munitions List, requested on form DS-4076 online.
RockSim's ruling shows it works for hobby software. A ruling that a drag-only airbrake computer and
its firmware are under the EAR would let them be published like the rest. Before filing, wait for
the interim final rule: if it defines "active controls" as targeting, as AIAA and several
commenters asked, the question may settle itself.

## Open questions for a lawyer or a ruling

1. Does Note 3's "active controls (e.g., RF, GPS)" reach a passive GPS tracker or telemetry radio,
   or only steering? Is a drag-only airbrake or a roll-only tab or canard a "flight control system"
   under IV(h)(1)? The 2024 docket asks State for exactly this line, unanswered so far
   ([roll and drag control](roll-and-drag-control.md)).
2. Would a drag-only airbrake computer, documented as general-purpose (for drones, robots and rockets alike),
   qualify for release under 120.41(b)(4) or (b)(5), or only by a ruling? Veronte's 2025 ruling sent a
   general-purpose controller to Commerce. State's proposed rule of October 1, 2026 (91 FR 62361, RIN
   1400-AF40, comments due November 30, 2026) amends the "specially designed" test, and State "is considering
   removing references to 'software'" from 120.41(a)(2), (b), (b)(3) and (b)(5) ([proposed rule][fr-2026]).
   This note reads that as leaving stand-alone software outside the (b) releases; firmware counts as part of
   its end item or component (120.40(f)), and software controlled as technical data is untouched.
3. Under ITAR, does publishing on GitHub put material in the public domain, given State's
   unresolved 2015 position?
4. Is hpr's published simulator, with its controller models, technical data "directly related" to
   a Category IV(a)(5) rocket, or general engineering principles? RockSim's ruling suggests the
   latter; the controller models make hpr's case a little different.
5. What will the interim final rule (RIN 1400-AE73), at OIRA since June 24, 2026, say? Re-read
   Category IV when it is published.

[adr-192]: ../decisions/0192-the-2026-10-06-follow-up-best-over-first-and-export-control.md
[m6-4]: ../decisions-and-roadmap.md#m6-4
[usml]: https://www.ecfr.gov/current/title-22/chapter-I/subchapter-M/part-121/section-121.1
[fr-2015]: https://www.federalregister.gov/d/2015-12844
[fr-2024]: https://www.federalregister.gov/d/2024-24091
[fr-2026]: https://www.federalregister.gov/d/2026-20079
[aiaa]: https://aiaa.org/wp-content/uploads/2026/04/ITAR-Reforms-Paper.pdf
[oira]: https://www.reginfo.gov/public/do/eoReviewSearch
[github]: https://docs.github.com/en/site-policy/other-site-policies/github-and-trade-controls
[ddtc-list]: https://www.pmddtc.state.gov/ddtc_public?id=ddtc_kb_article_page&kb_number=KB0011272
[ddtc-pen]: https://www.pmddtc.state.gov/ddtc_public?id=ddtc_kb_article_page&sys_id=384b968adb3cd30044f9ff621f961941
[roth]: https://www.govinfo.gov/content/pkg/USCOURTS-ca6-09-05805/pdf/USCOURTS-ca6-09-05805-0.pdf
[soong]: https://www.federalregister.gov/d/2023-28268
[doj-list]: https://www.justice.gov/sites/default/files/pages/attachments/2015/07/22/export-case-list-201505-final.pdf
[dd]: https://www.ca5.uscourts.gov/opinions/pub/15/15-50759-CV0.pdf
[bps]: https://joe-barnard-oh4g.squarespace.com/signal
[mit]: https://wikis.mit.edu/confluence/display/RocketTeam/Export+Controls+Compliance
[ssi]: https://ssi-wiki.stanford.edu/ITAR
[dteg]: https://www.esrarocket.org/s/2026-IREC-DTEG-V11.pdf
[irec-rr]: https://www.soundingrocket.org/uploads/9/0/6/4/9064598/irec_rules_and_requirements_document_v_1.6_.pdf
[m8]: https://content.u-blox.com/sites/default/files/NEO-M8-FW3_DataSheet_UBX-15031086.pdf
[m10]: https://content.u-blox.com/sites/default/files/MAX-M10S_DataSheet_UBX-20035208.pdf
