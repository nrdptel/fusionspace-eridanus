# ADR-188: Release 0.1's notes, and the flattering-side issues without a warning (2026-10-06)

- **Status:** accepted
- **Summary:** Release 0.1's last increment splits in two. M10.1d1 writes `CHANGELOG.md` and the install pages, and lists by number every open `env-core` issue on the flattering side or of unknown sign; M10.1d2 adds a warning for each flattering-side issue that has none (#18, #64, #73, #179, #325, #326, #354, #367), settles or warns each unknown sign, fixes #335, and then shows the whole checklist at a commit on `main`. The changelog links pages by the site's address and is held to the site's link and label checks.

**Context.** [ADR-162](0162-the-suite-and-its-releases.md) §2's checklist asks, among its items,
that every open `P-critical` or `env-core` issue whose error lies on a flattering side (a stability
margin or flutter speed too high, an apogee too low against a waiver ceiling, a charge too small;
[ADR-144](0144-the-2026-10-03-planning-review-leaner-bookkeeping.md) §6) be listed in the release's
PR and in the changelog's known gaps, and that a flight reaching its regime print a warning naming
it. It names five issues for 0.1 and says the rule, not that list, is the condition.

M10.1d's survey read every open `env-core` issue on 2026-10-06 (no `P-critical` issue was open):
37 issues, each sorted by its own evidence. The warnings in `crates/hpr-sim/src/issues.rs` cover
the issues of [ADR-163](0163-the-second-pass-products-and-priorities.md) §4's list; #67 of those is
now closed.

| sort | issues | warned |
|---|---|---|
| flattering (13) | #70, #72, #87, #120, #121, #172 | yes |
| | #18, #64, #179, #325, #326, #354, #367 | no |
| mixed, flattering in some regime (2) | #68 | yes, in that regime |
| | #73 | no |
| unknown sign (8) | #222 | yes, as if flattering |
| | #8, #104, #106, #213, #219, #360, #361 | no |
| safe side (3) | #69, #228, #234 | not needed |
| not an accuracy error (11) | #312, #335, #337, #357, #358, #359, #362, #363, #364, #369, #372 | not needed |

The eight flattering or mixed issues with no warning:

- #325 and #326: fin sets at one station counted apart, and a kinked freeform fin's center of
  pressure; together they put OpenRocket's *Pods--airframes and winglets* margin 0.071 to 0.076
  calibers above OpenRocket's.
- #64: a forward-swept fin's supersonic normal-force slope rises up to 7.7% past linear theory's
  start, moving the center of pressure aft.
- #367: a packed mass drawn at a nose cone's tip, where its center of gravity can't sit, warns as
  a fit but doesn't name the margin it inflates.
- #179 and #354: a separated part falls with no airframe drag (a booster as it tumbles; a part
  through its device's lag), so its drift reads short.
- #18: fully turbulent skin friction; laminar flow would lower the friction by `1700/R`, about 2%
  of the zero-lift drag on Calisto at Mach 0.3, so the drag reads high on a smooth surface.
- #73: the subsonic boattail rule gives long boattails no drag (the safe side) but over-predicts
  a 16° one by up to 41.7% and the Arcas Robin's 15° one below Mach 1 (the flattering side).

#335, a flight that turns unstable under power printing its apogee with no warning, is not an
accuracy error but is safety-relevant. #172's warning quotes the issue's 0.073 calibers, below the
0.1108 that the stability page now gives for the private designs.

Each missing warning needs its own condition read from the design or the flight (two fin sets
sharing a station, a kinked freeform outline, a forward-swept leading edge past Mach 1, a separated
part with a lag or no airframe), and its own test that flies a design across the condition's
edge. Eight of them, with their review, don't fit beside the release notes in one pull request.

**Decision.**

1. **M10.1d splits in two** (one increment level; M10.1d is retired as an id):
   - **M10.1d1, the release notes and the install pages.** `CHANGELOG.md` holds 0.1.0's entry:
     what the release holds, what works, how far to trust it with the accuracy page's figures,
     and the known gaps. The gaps list every flattering-side issue by number, those warned apart
     from those not yet warned, and every unknown-sign issue, #335, and #372's release-file
     limits. The README, *Getting started*, *Python* and *The API reference* say how to install
     the published release and, until it is published, how to build it.
   - **M10.1d2, the missing warnings and the checklist.** Each of the eight issues above prints a
     warning naming it on a flight that meets its condition, pinned by a test on each side of the
     condition; #172's warning quotes the largest gap measured on the private designs (0.1108
     calibers on `docs/stability-for-certification.md`, above the 0.073 it quotes now); each
     unknown-sign issue without a warning gets its sign recorded from evidence or a warning;
     #335 is fixed; then the whole checklist holds at a commit on `main`, shown by a `Release`
     dispatch from that commit.
2. **The rule's reading.** An issue counts as flattering when its own evidence puts the error on
   ADR-144 §6's side for some flight. A drift that reads short counts too: `issues.rs` already
   names a short drift among the numbers a high drag flatters. Where the sign depends on the
   regime, as for #68, the warning covers the flattering regime. An issue whose sign is unknown is
   listed, as the cautious reading; M10.1d2 settles each.
3. **The changelog is checked as a page is.** It is read on GitHub and copied into a release's
   notes, where a relative link breaks, so it links a page by the site's address and a file by its
   GitHub URL. `cargo xtask site` (and `cargo test -p xtask`) checks each such link against the
   page and its anchor. It refuses a relative link, a link to one of its own headings, a bare
   label, and a link to the site or the repository written any other way (`http://`, capitals),
   which would otherwise pass unchecked as a web link. A test asks for an entry headed by the
   workspace's version. `cargo xtask spelling` reads it.

**Consequences.** The release's PR and the changelog name every flattering-side issue now, so
nobody installs 0.1.0 without the list; the eight that don't warn yet say so in the changelog.
Release 0.1 is not met until M10.1d2. #372 (the glibc floor, license files inside the crates,
crates.io's limit on new crate names) stays a separate `P-high` issue to settle before the first
publish, and the changelog states its first two limits.
