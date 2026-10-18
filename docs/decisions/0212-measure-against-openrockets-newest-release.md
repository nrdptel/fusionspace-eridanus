# ADR-212: Measure against OpenRocket's newest release, not only 24.12 (2026-10-08)

- **Status:** accepted; tightens [ADR-209, the best on the market, measured][adr-209] §7 and M10.3; adds M2.3c3; context corrected 2026-10-18: the powered base-drag and thrust-at-altitude corrections are off by default ([#624](https://github.com/nrdptel/fusionspace-eridanus/issues/624))
- **Summary:** OpenRocket's development branch has merged changes since 24.12 that can move its apogees, body-in-fin interference among them. Two others, a powered base-drag correction and thrust corrected for air pressure, apply only when a nozzle exit diameter is entered, and the default, 0, applies neither. Release 0.3's target, a held-out mean absolute apogee error below OpenRocket's, would be judged against a version flyers are about to leave. So the comparison follows OpenRocket's releases. A new milestone, M2.3c3, flies the private collection and the public comparison sets on OpenRocket's newest release after 24.12 beside 24.12, on the flights both fly, by the same scripts, each version named. Release 0.3 (M10.3) then needs HPR Sim's held-out error below 24.12's and below that of any OpenRocket release published before M2.7 is met, on the held-out flights all fly; with none, 24.12's alone, so no release waits for OpenRocket's. Nothing is built from OpenRocket's source.

[adr-143]: 0143-the-operating-envelope-and-a-stop-rule-for.md
[adr-209]: 0209-the-2026-10-08-best-on-the-market-measured.md

**Context.** ADR-209 §1 sets the standard: never worse than the best comparable tool on any axis. §7
makes it Release 0.3's accuracy target: on the held-out logged flights both fly, HPR Sim's mean
absolute apogee error below OpenRocket's. Every comparison with OpenRocket today runs its 24.12 jar,
released on 2025-07-27. A planning review on 2026-10-08 read OpenRocket's public release notes and
pull-request titles and descriptions (never its source, which is GPL): since 24.12 its development
branch (`unstable`) has merged body-in-fin interference in the normal-force slope, then the full
NACA 1307 model (pull requests #3220, 2026-08-10, and #3263, 2026-09-09); a powered base-drag
correction citing NACA RM L50I18 (#3244, 2026-08-25); thrust corrected for the air's pressure
(#3327, 2026-09-23). The interference changes can move its apogees: with them a test's time to
apogee fell from 3.62 s to 3.35 s (#3233, which restored #3220), and their size on a collection is
not yet flown. The other two apply only when a nozzle exit diameter is entered in the motor chooser,
and the default, 0, applies neither: "If this equals 0 (default value), powered base drag correction
is not applied" (a comment by #3244's author, 2026-08-25); "if the nozzle exit area is 0 there is no
correction" (#3327's description). Both are absent from 24.12, the release the comparisons run, so
they move nothing there; on a newer release they move an apogee only where a design file or a
comparison script sets an exit diameter (corrected 2026-10-18,
[#624](https://github.com/nrdptel/fusionspace-eridanus/issues/624)). [What OpenRocket changed after
24.12](../research/openrocket-after-24-12.md), read on 2026-10-16, says so in its §2, and its §8
lists 12 more merged changes that this list left out and that may move an apogee, among them its
pull requests for the tangent-ogive zero (#2999), body friction (#3237) and the tumble rules (#3190,
#3382). A supersonic center-of-pressure fix for low-aspect-ratio fins (#3279) was closed without
merging. Its forum calls the coming release "26.xx"; no snapshot jar is published (its downloads
page offers 24.12 only, read on 2026-10-08).

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
   report (`fixture-flights`) and the public OpenRocket reports fly OpenRocket's newest release
   after 24.12 beside 24.12, by the same scripts, on the flights both versions fly, with each
   version named. Moves: accuracy (the scoreboard's OpenRocket column names its version).
3. **Release 0.3 is judged against every version flown.** M10.3's done-when, ADR-209 §7's
   target, now reads: on the held-out flights that HPR Sim and every OpenRocket version flown all
   fly, HPR Sim's mean absolute apogee error is below 24.12's and below that of any OpenRocket
   release published before M2.7 is met. On one common set that is the same as beating the lowest
   of the errors, and no version can be picked for flying a kinder set; a release out by then must
   be flown (M2.3c3), not skipped. The cutoff is M2.7 because that is when the held-out
   predictions are fixed; a release published later sets the next release's bar, so Release 0.3
   never waits for OpenRocket's. With no release after 24.12, M2.3c3 waits in the queue after
   M2.3c2, and a session that reaches it first skips to the next milestone and says so in the
   status.
4. **The target only tightens.** ADR-209 §7's other terms (the bias within 3%, the ranges'
   coverage, the paired bootstrap interval) are unchanged, and so is [ADR-143][adr-143]'s stop
   rule. Nothing is built from OpenRocket's source to get ahead of its release.

**Consequences.** Release 0.3 can't be met by outrunning an old version. The comparison scripts
must take the jar by path and hash, not assume 24.12 (the second jar may need a different Java;
`refs doctor` reports which). The reports grow one row per OpenRocket version. M2.3c3 adds a
reference without changing HPR Sim's physics, so by ADR-143's stop rule it counts as progress
only once it is measured.
