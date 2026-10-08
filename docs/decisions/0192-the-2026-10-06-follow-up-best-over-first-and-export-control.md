# ADR-192: Neer's 2026-10-06 follow-up: the best product over the first, no legacy contracts, and three export tiers (2026-10-06)

- **Status:** accepted; amends [ADR-191, the 2026-10-06 ideas][adr-191] §1 and §5
- **Summary:** Neer's decisions on ADR-191's three open questions. The old web tools get no more work and constrain nothing: Motor Finder's API is not a contract, and hpr's stock service is designed fresh even if release 0.1's reader breaks. The best product outranks the first release. Export control sorts the work into three tiers (public, held, never): the simulator and its control models stay public; firmware or hardware that moves a control surface is held off GitHub and out of cloud services until a ruling; target guidance, thrust vector control and lifted GPS caps are never built.

**Context.** [ADR-191][adr-191] left three questions open: Charge's minimum-strength pins, Motor
Finder's API kept as a contract, and four export-control questions. On 2026-10-06 Neer decided
them ([VISION](../VISION.md), that date):

- On Charge: drop the question, since the tool has no users and will be sunset.
- On the API: breaking release 0.1's reader is fine; the best product matters more, and none of
  his old products should hold it back.
- On export control: research the rules, how they have been applied and how others handle them;
  keeping some material in a separate private repository is acceptable if that is best.
- On releases: the best product comes before the first one out the door.

The export research is in [the export control note](../research/export-control.md). It found:

- **The precedent:** DDTC ruled RockSim, a hobby rocket simulator, EAR99 in 2020, and EAR
  15 CFR 734.7 puts published software outside the Commerce rules.
- **The exposure:** a rocket with "active controls (e.g., RF, GPS)" loses Category IV(a)'s hobby
  exclusion, and a rewrite of Category IV is pending.
- **No hobbyist case:** none of the prosecutions or penalties found involved a hobbyist. Every one
  involved contract data, military hardware or a listed buyer.
- **Storage:** GitHub says GitHub.com isn't built for ITAR data (reading that to cover private
  repositories is this record's inference), and a cloud service holds plaintext.

**Decision.**

1. **The old tools get no more work.** Charge's minimum-strength question is dropped. hpr's own
   charge sizing ([M6.7][m6-7]) still sizes pins at the high end, by its own rule.
2. **No legacy contracts** (amends [ADR-191][adr-191] §1).
   - Motor Finder's `/api/v1/` is not a contract. hpr's stock service is designed for hpr, with
     its own data shape, address and cache rules.
   - Release 0.1's reader may stop working when that service lands. The release notes say so.
   - None of the four old tools sets a constraint on hpr's design.
3. **The best product over the first release.**
   - The release train stays ([ADR-162][adr-162]), but no *done when* is trimmed, and no review
     or check is skipped, to make a release sooner.
   - A release goes out when its milestones are met, not on a date.
4. **Three export tiers** (amends [ADR-191][adr-191] §5).
   - **Public:** the simulator, its airbrake, canard and controller *models*, design checks,
     recovery and tracker logic within commercial GPS limits, the ground station, and every
     format. They live in this repository.
   - **Held:** firmware or hardware that moves a control surface on a real rocket.
     - It isn't written as part of this repository's work.
     - If Neer writes any, it stays on his own machine, or in storage that meets 22 CFR
       120.54(a)(5)'s safe harbor, whose five conditions are listed in
       [the export control note](../research/export-control.md). It never goes on GitHub, in a private repository there, or in a
       cloud service.
     - It moves to the public tier only after a commodity jurisdiction ruling, or a lawyer's
       answer, puts it outside the Munitions List. A Category IV rule that defines "active
       controls" as targeting would also count.
   - **Never:** guidance toward a target or a GPS point, thrust vector control, and code that lifts
     a GPS receiver's speed or altitude caps.
   - **Two rules come with the tiers.** No contributed file from defense or contract work, or one
     carrying an export marking, is accepted. [M6.4, airbrakes][m6-4] gains a check that a design
     is stable with its brakes retracted and fully deployed (IREC's stable-when-inert rule).
5. **A private repository is for privacy, not export control.** Neer's private repositories stay
   right for his own data (the fixture collections) and for anything he wants to keep to himself.
   They are not a place for material that might be ITAR technical data, because GitHub.com isn't
   built to hold it.

**Alternatives considered.**

- *Keep held material in a private GitHub repository.* Rejected for anything that
  might be controlled: GitHub's own trade-controls page says GitHub.com isn't designed for ITAR
  data, private or public. Still fine for privacy.
- *Publish airbrake firmware now,* as NC State's team does. Rejected for now. No enforcement
  against hobbyists was found, but the Munitions List note is explicit, and its rewrite is due.
  Waiting for that rule costs little, because the hardware is after release 1.0 anyway.
- *Hold back the simulated controllers too.* Rejected: RockSim's EAR99 ruling and RocketPy's
  published airbrake controller put simulator models on the published side.
- *Keep Motor Finder's API frozen for 0.1's users.* Rejected.

**Consequences.**

- Open for the maintainer: the export item, which names the ruling as the step that would
  publish held work, with the interim final rule to watch first. Charge and the API are settled.
- The roadmap's M6.4 gains the stable-when-inert check.
- [The web tools note](../research/suite-web-tools.md) drops the API contract.
- [VISION](../VISION.md) records the answers. Its *Not now* line on control firmware points here.

[adr-162]: 0162-the-suite-and-its-releases.md
[adr-191]: 0191-the-2026-10-06-ideas-web-tools-watches-repo-layout-hardware.md
[m6-4]: ../decisions-and-roadmap.md#m6-4
[m6-7]: ../decisions-and-roadmap.md#m6-7
