# US export control and hpr: the rules, the record, and what hpr publishes

**Bottom line:** the closest precedent puts a hobby simulator like hpr outside the Munitions List.
In 2020 the State Department ruled RockSim, a hobby rocket design and flight simulator, EAR99, the
lowest category. Published software is outside the Commerce rules, apart from encryption code and
firearm print files. Whether that settles hpr's own case, with its controller models, is still
open (question 4 below). The exposed point is hardware: a rocket that carries "active controls
(e.g., RF, GPS)" loses the Munitions List's exclusion for hobby rockets. So firmware and
hardware that move a control surface wait for a ruling, and nothing that might be controlled is
put on GitHub or given to a third-party cloud service. Researched October 6, 2026, from the
regulations, the Federal Register, court records, DDTC's published rulings and what other
projects do.
[ADR-192, the 2026-10-06 follow-up][adr-192] records what hpr does about it. **Not legal
advice:** the open questions at the end are for a lawyer or a ruling.

This research was added the same day: the rules, how they have been applied in the past, and
how other people go about it.

## The two sets of rules

The US has two export regimes, and they treat publishing differently.

| | ITAR (State Department) | EAR (Commerce Department) |
|---|---|---|
| covers | the US Munitions List: defense articles and their technical data | dual-use items on the Commerce Control List, and almost everything else as EAR99 |
| rockets | Category IV(a)(5): any rocket not listed elsewhere, no size floor | sounding-rocket and GPS items (7A105, 9A604.x and others) |
| publishing on the internet | not named in its public-domain definition (22 CFR 120.34); State proposed in 2015 that posting technical data without approval is a violation, and never settled it | named: "posting on the Internet on sites available to the public" makes software and technology published and not subject to the EAR (15 CFR 734.7(a)(4)) |
| research | fundamental research only at accredited US universities | any research meant to be published (734.8) |

**The hobby-rocket exclusion.** Category IV(a)'s Note 3 excludes model and high-power rockets
"made of paper, wood, fiberglass, or plastic containing no substantial metal parts and designed to
be flown with hobby rocket motors that are certified for consumer use. Such rockets must not
contain active controls (e.g., RF, GPS)" ([22 CFR 121.1](https://www.ecfr.gov/current/title-22/chapter-I/subchapter-M/part-121/section-121.1),
text of 2026-10-01). It was proposed in January 2013 (78 FR 6765) and took effect on July 1, 2014
(79 FR 34). Before 2014 the category had no hobby exclusion at all. The final rule's preamble
doesn't discuss hobby rockets, but answers a question on sounding and research rockets: "all such
rockets are controlled under USML Category IV".

**What else in Category IV touches control.**

- IV(h)(1): flight control and guidance systems "specially designed" for a Category IV(a) rocket.
  A "specially designed" item can be released from the list by the tests in 22 CFR 120.41(b),
  most surely by a commodity jurisdiction ruling.
- IV(h)(4): rocket thrust vector control, listed by name. No release test applies to it.
- XII(d): inertial units only above set accuracy thresholds or rated above 25 g.

**The pending rewrite.** State proposed a new Category IV on October 23, 2024
([89 FR 84482](https://www.federalregister.gov/documents/2024/10/23/2024-24091)). It defines an
"amateur rocket" by six conditions:

- no substantial metal parts;
- no harmful payload;
- 40,960 N·s total impulse or less;
- room for 5 lb of propellant or less;
- "Has no active controls";
- unable to pass 150 km.

The "(e.g., RF, GPS)" examples are gone; "active controls" is still undefined. The docket
(DOS-2024-0035) drew 848 comments. Among them, a Tripoli member warned that the wording "could also
unintentionally cover devices such as drag brakes". AIAA's April 2026 paper asks to define active
control as "targeting functions against a designated target"
([AIAA](https://aiaa.org/wp-content/uploads/2026/04/ITAR-Reforms-Paper.pdf)). The regulatory
agenda lists an interim final rule for September 2026. As of October 6, 2026 nothing has been
published, and the eCFR still shows Note 3.

**Storage can be an export too.** Storage in the US that only US persons can reach is not an
export. Beyond that, 22 CFR 120.54(a)(5) (84 FR 70887, effective March 25, 2020) gives a safe
harbor for sending or storing technical data that is all of: unclassified; end-to-end encrypted;
encrypted with FIPS 140-2 modules under NIST guidance, or at least AES-128 strength; not stored in
or sent to a country on the §126.1 list; and not sent from one. Home-made encryption that isn't
FIPS-validated misses the harbor. GitHub says GitHub.com "has not been designed to host data
subject to the ITAR" and can't restrict access by country
([GitHub](https://docs.github.com/en/site-policy/other-site-policies/github-and-trade-controls)).
It doesn't separate private repositories from public ones; reading it to cover both is this
note's inference. A cloud service that processes text reads it in plaintext, and its staffing
and storage can't be checked from outside, so the same caution applies. Releasing technical data to a foreign
person inside the US is an export to their country (22 CFR 120.50, 120.56).

## How the rules have been applied

**No prosecution or penalty found involved a hobbyist or a hobby rocketry product.** DOJ's case
lists and BIS's enforcement booklet show three patterns: data from a military contract shared,
military-grade hardware smuggled, or a sale to a listed buyer.

| case | what happened | the lesson |
|---|---|---|
| RockSim v11.0, DDTC ruling, 2020-06-19 | a hobby rocket design and flight simulator ruled EAR99 | the closest precedent to hpr's simulator |
| UT Austin rocket-avionics thesis, DDTC ruling, 2020-12-17 | ruled subject to neither regime | a top-level avionics description can be free of both |
| AeroTech (RCS) hobby motors, 2016-11-23 | Munitions List above 5 lb of propellant; at or below, "seek a CCATS" (a Commerce classification) | the hobby line is drawn by size, in writing |
| PRODAS v3.5, DDTC ruling, 2014-09-03 | "Projectile Rocket Ordnance" design and analysis software ("all Types of Projectiles", ballistics), ruled Munitions List IV(i) | an ordnance simulator was ruled a defense article; RockSim's hobby purpose is the clearest difference |
| Cloud Cap Piccolo UAV autopilot, 2016 | Commerce (7A994, 7D994) | a commercial autopilot and its software can be dual-use, not munitions |
| Roth, University of Tennessee, 2009 | 48 months: Air Force drone data shown to two foreign graduate students | access by a foreign person is the export; the court required only knowledge that the act was unlawful |
| Soong, 2023 | 20 months: EAR99 aircraft software sold through a front to a listed university | even the lowest category needs the buyer screened |
| Hanson, 2010 | 24 months after a plea to false statements: miniature UAV autopilots shipped to China under a "model airplane civilian flying club" story | a hobby label protects nothing |
| Defense Distributed, 2013 to 2021 | State ordered CAD files offline; courts weighed security over speech; firearm files later moved to the EAR with an explicit carve-out | posting online is not a safe harbor under ITAR; "code is speech" is an argument, not a defense |
| FLIR, Honeywell, 3D Systems, Boeing, 2018 to 2024 | civil settlements of $13M to $51M for data reached by foreign persons | companies are fined for access, not just shipments |

Sources: [DDTC's ruling list](https://www.pmddtc.state.gov/ddtc_public?id=ddtc_kb_article_page&kb_number=KB0011272)
(dated 2026-08-01), [Roth's appeal](https://www.govinfo.gov/content/pkg/USCOURTS-ca6-09-05805/pdf/USCOURTS-ca6-09-05805-0.pdf),
[Soong](https://www.justice.gov/usao-ndca/pr/castro-valley-resident-pleads-guilty-illegally-exporting-american-aviation-technology),
[Defense Distributed, 5th Cir.](https://www.ca5.uscourts.gov/opinions/pub/15/15-50759-CV0.pdf),
[DOJ's 2015 case list](https://www.justice.gov/sites/default/files/pages/attachments/2015/07/22/export-case-list-201505-final.pdf).

## How others go about it

- **US student teams publish airbrake code openly:** NC State's
  [AirbrakesV2](https://github.com/NCSU-High-Powered-Rocketry-Club/AirbrakesV2), MIT licensed, has
  no export notice. Other teams' repositories turned up in searches but weren't opened. RocketPy's
  airbrake guide ships a full controller ([docs](https://docs.rocketpy.org/en/latest/user/airbrakes.html)).
- **Non-US teams publish more,** canards included: Waterloo's roll-control canard firmware, EPFL,
  ARIS, CATS Vega. One US group did too: Portland State's rocket society flew and published canard
  roll control ([roll and drag control](roll-and-drag-control.md)).
- **Thrust vector control is what gets held back.** BPS.space keeps its flight software private
  and sells its thrust-vectoring computer only to "US citizens and residents due to US export
  restrictions" ([Signal](https://joe-barnard-oh4g.squarespace.com/signal)).
- **Universities split.** MIT's rocket team runs a technology control plan with locked storage and
  an access log. Stanford's stays inside fundamental research "by frequently publicizing
  information about group activities".
- **IREC's line** is the clearest in rocketry. Control only "for pitch and/or roll stability
  augmentation, or for aerodynamic braking", never "actively guided towards a designated spatial
  target or GPS location", and the rocket stable with the controls inert
  ([DTEG 2026 §7](https://www.esrarocket.org/s/2026-IREC-DTEG-V11.pdf)). The competition calls
  itself "ITAR-free" and makes US authors responsible for their own submissions.
- **Tracker sellers** (Featherweight, Eggtimer, Missile Works, Altus Metrum) publish no export
  statement. They ship receivers capped at 500 m/s and 50 to 80 km by the GPS chip's own firmware.
- **Drone autopilots** (PX4, ArduPilot) publish everything, on the EAR's published-software rule.
  ArduPilot adds a use policy against weaponization. Neither has a formal export statement.
- **No rocketry project found** splits public algorithms from private tuned parameters, or checks
  citizenship before giving repository access.

## What hpr does

ADR-192 sorts the work into three tiers.

| tier | what | where it lives | why |
|---|---|---|---|
| **Public** | the simulator, its airbrake, canard and controller *models*, design checks, the recovery and tracker logic within commercial GPS limits, the ground station, every format | this public repository | RockSim's EAR99 ruling; the EAR's published-software rule; what US teams and RocketPy already publish |
| **Held** | firmware or hardware that moves a control surface on a real rocket | not written for the project; if Neer writes any, on his own machine or in storage that meets the 120.54(a)(5) harbor, never on GitHub or in a third-party cloud service | Note 3 and IV(h)(1); GitHub's own policy; the 120.54 carve-out |
| **Never** | guidance toward a target or a GPS point, thrust vector control, active pitch or yaw control (ADR-193), code that lifts a GPS receiver's speed or altitude caps | nowhere | IREC's line, narrowed; IV(h)(4) has no release; 7A105 |

Three rules come with the tiers:

- **The design check behind the public tier:** a design with active control must be stable with
  the controls inert (IREC §7.2.1). [M6.4, airbrakes][m6-4] checks it.
- **Contributions:** hpr accepts no file that came from defense or contract work, or that carries
  an export marking.
- **Selling:** hardware or paid downloads are screened against the government's
  [consolidated screening list](https://www.trade.gov/consolidated-screening-list), and sell in the
  US first. That is Neer's step when he sells.

**The way to move the held tier into the public one** is a commodity jurisdiction ruling: DDTC's
formal answer on whether an item is on the Munitions List. It is requested on form DS-4076
through DDTC's online system. RockSim's ruling shows it works for hobby software. A
ruling that a drag-only airbrake computer and its firmware are under the EAR would let them be
published like the rest. Before filing, wait for the interim final rule: if it defines "active
controls" as targeting, as AIAA and several commenters asked, the question may settle itself.

## Open questions for a lawyer or a ruling

1. Does Note 3's "active controls (e.g., RF, GPS)" reach a passive GPS tracker or telemetry radio,
   or only steering? Is a drag-only airbrake or a roll-only tab or canard a "flight control system"
   under IV(h)(1)? The 2024 docket asks State for exactly this line, unanswered so far
   ([roll and drag control](roll-and-drag-control.md)).
2. Would a drag-only airbrake computer, documented as general-purpose (for drones, robots and
   rockets alike), qualify for release under 120.41(b)(4) or (b)(5)? Or is a ruling the only sure
   route? State's proposed rule of October 1, 2026 (91 FR 62361, comments due November 30, 2026)
   amends the "specially designed" test, so re-read 120.41 when it is final.
3. Under ITAR, does publishing on GitHub put material in the public domain, given State's
   unresolved 2015 position?
4. Is hpr's published simulator, with its controller models, technical data "directly related" to
   a Category IV(a)(5) rocket, or general engineering principles? RockSim's ruling suggests the
   latter; the controller models make hpr's case a little different.
5. What will the interim final rule say? Re-read Category IV when it is published (RIN 1400-AE73).

[adr-192]: ../decisions/0192-the-2026-10-06-follow-up-best-over-first-and-export-control.md
[m6-4]: ../decisions-and-roadmap.md#m6-4
