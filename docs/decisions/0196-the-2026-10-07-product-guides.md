# ADR-196: Neer's 2026-10-07 product guides: each product, illustrated (2026-10-07)

- **Status:** accepted; amends [ADR-162, the suite and its releases][adr-162] §2 (the release checklist) and §3 (the queue), and [ADR-147, a roadmap of open work][adr-147] §6 (its budget)
- **Summary:** Each of the suite's products gets a guide on the docs site: a landing page and an illustrated tour that ends on a public real flight, in M0.7's format and checks. Every picture is made by a command CI reruns, never taken by hand: CLI output as terminal figures, plots from hpr, app screens from end-to-end tests, with phone and watch captures checked against the device's safe area. M0.8a writes the simulator's guide after M0.7a; from release 0.2 on, the release checklist requires that release's guide, read by Neer; each line after 1.0 ships its guide with its first milestone. The roadmap's budget rises to 40,500 bytes.

**Context.** Later on October 7, 2026, after [ADR-195, the guides][adr-195], Neer added
in-depth, well-illustrated guides to the project's products as well ([VISION](../VISION.md), that
date).

The simulator's pages are a tutorial, task pages and the API reference, listed flat with no page
per product. The site has two figures in its pages. CLI output appears as text that
`xtask cli --check` regenerates, and nothing makes screenshots. ADR-162 §2's checklist asks only
for a changelog entry and the install pages. Outside ([the product-guides
note](../research/product-guides.md)), rocketry tools rely on screenshots refreshed in batches,
or on one long manual; RocketPy's worked examples, ending on a real flight, read best.

**Decision.**

1. **Each product gets a guide:** a landing page (what it is, who it is for, how far to trust it,
   how to install it), then a tour that ends on a real flight, then task pages, all in M0.7's
   format and [ADR-195][adr-195] §7: words before equations, numbers from CI runs or committed
   reports, no verdicts, nothing copied. The tour ends on a real flight from the committed
   real-flights report, named by its case id, never a private design or log. Its measured
   numbers come from that report, shown beside the spread over every flight in it, so a
   flattering pick cannot stand alone; a measured trace is drawn only where `THIRD-PARTY-NOTICES.md`
   licenses it, and anything drawn from ERA5 weather carries Copernicus's credit.
2. **No picture is taken by hand.** A command makes each, and CI reruns it, failing on a stale
   one:
   - CLI output is drawn as terminal figures from the runs `xtask cli --check` already makes;
   - plots come from hpr's plot writer;
   - app screens come from end-to-end tests at fixed device sizes, in the default appearance,
     with labels in captions and callouts rather than baked into the image;
   - callouts use the product system's drawing language (balloons and leaders);
   - hardware manuals use the system's signal-word panels.
3. **Clipping is checked, not eyeballed.** M0.7a's checks run on every product picture, and a
   phone or watch capture also fails when a label, control or number crosses the device's safe
   area: rounded corners, the notch or island, the home indicator, a round face's inscribed area.
   The shape comes from each platform's published insets and corner radii, not from the test
   runner, and a test moves one label into a corner to show the check fails. M9.4's done-when
   carries this, and a later watch milestone takes the same check.
4. **Placement:** M0.8a, the simulator's guide, follows M0.7a, after release 0.1, so 0.1 is not
   held. From 0.2 on, ADR-162 §2's checklist gains an item: "the release's product guide,
   written by the release milestone in M0.8a's form, and Neer has read it". Release 0.2 writes
   the analyzer's; 0.3 updates the simulator's and the analyzer's; 0.4 writes the competition
   and field kits'; 0.5 and 1.0 the app's. Each line after 1.0 ships its guide with its first
   milestone.
5. **The roadmap's budget rises** from 38,500 to 40,500 bytes, for M0.8's entry and the rules above.

**Alternatives considered.**

- **One manual for the whole suite.** Rejected: readers come for one product, and RASAero's
  single manual shows how long ones age.
- **Hand-taken screenshots, refreshed before each release.** Rejected: nothing then fails when
  one goes stale, and Neer's no-clipping rule needs an automated check on every picture.
- **The simulator's guide inside release 0.1.** Rejected: 0.1 already has its tutorial and task
  pages, and the guide's figure tooling is M0.7a's; it follows right after.
- **Animated recordings first.** Deferred: still figures can be tested for clipping; recordings
  wait in the backlog.

**Consequences.**

- Releases 0.2 to 1.0 each take a guide's work; the analyzer's release grows by one page set.
- The app's end-to-end tests double as its screenshot source, so M9.0's spike should pick a test
  runner that can capture.
- A versioned copy of the site per release, animated CLI recordings and runnable notebooks wait
  in the ideas backlog.

[adr-147]: 0147-m0-5b-a-roadmap-of-open-work-an-archive-and-a.md
[adr-162]: 0162-the-suite-and-its-releases.md
[adr-195]: 0195-the-2026-10-07-guides-rocketry-explained.md
