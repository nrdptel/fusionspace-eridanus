# ADR-191: Neer's 2026-10-06 ideas: the four web tools, watches, the repo layout and the hardware line (2026-10-06)

- **Status:** accepted; amends [ADR-162, the suite and its releases][adr-162] §1 and §6; §1's API contract and §5 amended by [ADR-192, the follow-up](0192-the-2026-10-06-follow-up-best-over-first-and-export-control.md)
- **Summary:** the four FusionSpace web tools (Motor Finder, Charge, Window, Muster) are rebuilt as library code and app screens in M9.6, after mobile; Motor Finder's scraper stays a hosted service whose API hpr keeps reading. hpr stays one repository with several workspaces, split only by named triggers. Watches wait until after mobile, with the phone as hub. The hardware line is ordered by regulation, and active-control firmware or hardware waits for Neer's export-control answer.

**Context.** On October 6, 2026, Neer added four ideas ([VISION](../VISION.md), that date):

- smartwatch apps after the mobile apps, researched when the time comes;
- sunsetting his four web tools, Motor Finder, Charge, Window and Muster
  (`FS · SW · TOOL 001` to `004`, [ADR-164][adr-164]), and rebuilding them in hpr, less tied to
  one platform and better connected;
- splitting into smaller workspaces, whether separate directories, separate repositories or
  sub-repositories, as the products grow;
- a hardware line: printable designs, several flight computers and trackers, camera, drone and
  plane payloads, canards and airbrakes.

He asked for research, care and the right priorities. Four research notes back this record:
[the web tools](../research/suite-web-tools.md), [the repo layout](../research/repo-layout.md),
[watches](../research/suite-watches.md) and [the hardware line](../research/suite-hardware-line.md).
Three findings shape it:

- **Motor Finder can't move into hpr whole.** Its hourly scraper, stock history and restock emails
  need a server, and every released hpr reads its API at a fixed address.
- **Charge sizes shear pins at nylon's minimum strength,** as Fetter's paper does, with the
  paper's 40% safety factor on top. The minimum gives less powder than stronger pins need, and
  whether the margin covers the spread hasn't been measured
  ([the web tools note](../research/suite-web-tools.md)). hpr's rule is the high end.
- **The US Munitions List's exclusion for hobby rockets has conditions, one of them no active
  controls.** Its Note 3 to Category IV(a) says such rockets "must not contain active controls
  (e.g., RF, GPS)", besides conditions on materials and motors
  ([the hardware line note](../research/suite-hardware-line.md)). What that means for hpr's
  hardware is a lawyer's question. [ADR-162][adr-162]'s avionics
  guardrails knew the Commerce rules but not this one.

**Decision.**

1. **The four web tools are rebuilt in hpr in a new milestone, M9.6, queued right after mobile
   (M9.4).**
   - Each tool's logic becomes library code, and its pages become screens of the app on every
     platform.
   - Charge's sizing arrives earlier, as [M6.7, ejection charges][m6-7] in release 0.4. It is
     rebuilt, not ported as written: pins at the high end of their strength, an efficiency factor
     from published ground tests, and Fetter's tables as one candidate basis.
   - Window's drift becomes pad day's simulated landing ([M9.4][m9-4]). Its "No-go" label isn't
     carried over.
   - Muster's compatibility graph becomes a sourced data module beside the motor catalog.
   - Motor Finder's scraper, history and emails stay a hosted service in their own repository.
     Its pages become the app's motor picker, and its substitutes become substitutes ranked by
     simulated apogee ([M6.3][m6-3]).
   - Neither Muster's nor Motor Finder's copies of ThrustCurve data come in (issue #295).
   - No site is turned off before its replacement ships and Neer has used it. Turning one off,
     redirects and domains are Neer's.
   - **Motor Finder's API is a contract.** The paths under `/api/v1/`, schema version 1, open
     CORS and the CC BY license stay for as long as any released hpr reads them. A new schema
     ships beside version 1, not in place of it.
2. **One repository, several workspaces** (amends [ADR-162][adr-162] §1, "one workspace").
   - The root Cargo workspace keeps the library crates.
   - Firmware, the desktop shell, the mobile apps and the web app each become their own top-level
     workspace or build directory, when trigger T2 fires (another target or toolchain). For
     firmware that is [M13.2][m13-2]; for the shells, [M9.0][m9-0].
   - A part gets its own repository only for a service that runs on a server (T5) or a license
     other than MIT, Apache-2.0, CERN-OHL-P or CC BY (T4). No git submodules.
   - The triggers T1 to T8 are in [the repo layout note](../research/repo-layout.md), and each
     move names its trigger in an ADR.
3. **Watches come after mobile, with no milestone yet.**
   - A watch is a companion: the phone is the hub and runs every simulation.
   - Two things carry the idea until then. M9.0's decision names which part of the mobile app
     owns the watch link, and M13.1's codec includes a compact event and position message a watch
     can decode.
   - The idea waits in [the ideas backlog](../research/ideas.md) as LATER.
4. **The hardware line is ordered by regulation.**
   - Simulation first: airbrakes ([M6.4][m6-4]) with IREC's control checks, then printed-part
     warnings (M8.3g, new), a camera shroud, released payloads, and canards ([M6.5][m6-5]).
   - Physical products, least regulated first: printable files, camera shroud kits, the ground
     station, a family of trackers, a family of flight computers (M13.5, new, after the first
     board), active-control hardware, then drone and plane payloads.
   - The camera, drone and plane payloads wait in the ideas backlog.
5. **A new guardrail** (amends [ADR-162][adr-162] §6). hpr's simulated airbrakes and canards are
   models in a published simulator, as RocketPy's airbrakes are, and they go ahead. *Firmware*
   or *hardware* that moves a control surface waits for Neer's answer to the export-control
   questions in [the hardware line note](../research/suite-hardware-line.md). Those questions join
   his existing FCC and export call before M13.4.

**Alternatives considered.**

- *Rebuild the web tools before 1.0.* Rejected. It would put four more surfaces in progress
  against the cap of three ([ADR-144][adr-144] §6) and delay the releases planned to come early.
  Charge's library half already sits in 0.4.
- *Port Charge as written.* Rejected: hpr sizes pins at the high end of their strength, and
  the minimum-strength reading gives less powder than that.
- *Move Motor Finder's scraper into hpr.* Rejected: core crates do no I/O, and an hourly scraper
  with secrets doesn't fit an offline-first repository (T5).
- *A repository per product, or submodules.* Rejected: the products share crates (the flight
  record, the `no_std` estimator), and automated work in one checkout trips on submodule pins.
- *Put M9.6 last in the queue, after the avionics.* Rejected: the four tools have users today,
  and the avionics wait on hardware and Neer's legal calls.

**Consequences.**

- The queue gains M9.6 at position 33. The three avionics milestones (M13.3, M13.1, M13.2) each
  move one place later.
- The roadmap gains M9.6, M8.3g and M13.5. [ARCHITECTURE](../ARCHITECTURE.md)'s suite table
  shows the planned layout.
- Open for the maintainer:
  - Charge's live site sizes pins at the minimum strength, as its source does;
  - the export-control questions;
  - Motor Finder's API contract.

[adr-144]: 0144-the-2026-10-03-planning-review-leaner-bookkeeping.md
[adr-162]: 0162-the-suite-and-its-releases.md
[adr-164]: 0164-the-fusionspace-product-system.md
[m6-3]: ../decisions-and-roadmap.md#m6-3
[m6-4]: ../decisions-and-roadmap.md#m6-4
[m6-5]: ../decisions-and-roadmap.md#m6-5
[m6-7]: ../decisions-and-roadmap.md#m6-7
[m9-0]: ../decisions-and-roadmap.md#m9-0
[m9-4]: ../decisions-and-roadmap.md#m9-4
[m13-2]: ../decisions-and-roadmap.md#m13-2
