# ADR-212: Measure against OpenRocket's newest release, not only 24.12 (2026-10-08)

- **Status:** accepted; tightens [ADR-209, the best on the market, measured][adr-209] §1 and §7; adds M2.3c3
- **Summary:** OpenRocket's main branch has merged changes since 24.12 that move its apogees: body-in-fin interference, a powered base-drag correction and thrust corrected for air pressure. Release 0.3's target, a held-out mean absolute apogee error below OpenRocket's, would be judged against a version flyers are about to leave. So the comparison follows OpenRocket's releases. A new milestone, M2.3c3, flies the private collection and the public comparison sets on OpenRocket's next published release beside 24.12, by the same scripts, each version named in the reports; ADR-209 §7 is then judged against the lower of the two errors. Until that release is published the milestone waits; nothing is built from OpenRocket's source.

[adr-143]: 0143-the-operating-envelope-and-a-stop-rule-for.md
[adr-209]: 0209-the-2026-10-08-best-on-the-market-measured.md

**Context.** ADR-209 §1 sets the standard: never worse than the best comparable tool on any
axis. §7 makes it Release 0.3's accuracy target: on the held-out logged flights both fly, HPR
Sim's mean absolute apogee error below OpenRocket's. Every comparison with OpenRocket today runs
its 24.12 jar, released on 2025-07-27. A planning review on 2026-10-08 read OpenRocket's public
release notes and pull-request titles and descriptions (never its source, which is GPL): since
24.12 its main branch has merged body-in-fin interference in the normal-force slope, then the full
NACA 1307 model (pull requests #3220, 2026-08-10, and #3263, 2026-09-09); a powered base-drag
correction citing NACA RM L50I18 (#3244, 2026-08-25); thrust corrected for the air's pressure
(#3327, 2026-09-23); and a supersonic center-of-pressure fix for low-aspect-ratio fins (#3279,
2026-08-27). Each moves its apogees. Its forum calls the coming release "26.xx"; no snapshot jar
is published (its downloads page offers 24.12 only, read on 2026-10-08).

Today's gap on the private collection is HPR Sim's 13.96% mean absolute error against
OpenRocket 24.12's 13.08%, 0.88 points worse, on 55 flights (M2.3c1, ADR-184). A target met
against 24.12 after flyers move to 26.xx would claim "better than OpenRocket" against a version
nobody runs, which ADR-209 §2 forbids in spirit: "better" stands only beside a committed
comparison, and the comparison has to be with the tool as flyers use it.

**Decision.**

1. **The comparison follows OpenRocket's releases.** Each OpenRocket release published after
   24.12 joins the comparisons as a second oracle, beside 24.12, run as a jar (never built from
   source, never read). Its jar is pinned by its published address and SHA-256, as 24.12's is.
2. **M2.3c3 OpenRocket's newest release** (core band, Mach 0 to 2.5). *Done when:* the fixture
   report (`fixture-flights`) and the public OpenRocket reports fly OpenRocket's newest published
   release beside 24.12, by the same scripts, with each version named in the reports; and ADR-209
   §7 is judged against the lower of the two versions' held-out errors. Moves: accuracy (the
   scoreboard's OpenRocket column names its version).
3. **While no newer release exists, M2.3c3 waits** in the queue after M2.3c2; a session that
   reaches it skips to the next milestone and says so in the status. Nothing is built from
   OpenRocket's source to get ahead of its release.
4. **The target only tightens.** If the newer release's error is higher than 24.12's on the same
   flights, 24.12 stays the bar; the lower error always sets it. ADR-209 §7's other terms (the
   bias within 3%, the ranges' coverage, the paired bootstrap) are unchanged, and so is
   [ADR-143][adr-143]'s stop rule.

**Consequences.** Release 0.3 can't be met by outrunning an old version. The comparison scripts
must take the jar by path and hash, not assume 24.12 (the second jar may need a different Java;
`refs doctor` reports which). The reports grow one row per OpenRocket version. M2.3c3 adds a
reference without changing HPR Sim's physics, so by ADR-143's stop rule it counts as progress
only once it is measured.
