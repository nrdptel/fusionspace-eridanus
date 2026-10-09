# ADR-220: The margin bar tightened to 0.2 calibres, M1.14 held to Release 0.3's target (2026-10-09)

- **Status:** accepted; tightens the census's margin and center-of-mass bar ([ADR-084][adr-084]) and M1.14's done-when ([ADR-143][adr-143] §6), toward [ADR-209][adr-209] §7 and [ADR-212][adr-212]
- **Summary:** the bar a stability margin or a center of mass is judged by, against OpenRocket, drops from 0.5 to 0.2 calibres. Every difference measured on both comparison sets but one is within 0.111 calibres, so the old bar let a change make the worst agreement four times worse without any row changing its standing; the new one catches a doubling. No row changes its standing: margins within it on 52 of 53 public flights and 35 of 35 private, centers of mass on 54 of 54 and 35 of 35. M1.14, the accuracy milestone, now also needs its error on the private flight collection below OpenRocket's, 24.12's and its newest release's, with its bias within 3%: what Release 0.3 needs, asked of the milestone meant to deliver it.

[adr-084]: 0084-the-accuracy-census-the-reports-numbers-held-to.md
[adr-143]: 0143-the-operating-envelope-and-a-stop-rule-for.md
[adr-209]: 0209-the-2026-10-08-best-on-the-market-measured.md
[adr-212]: 0212-measure-against-openrockets-newest-release.md

**Context.** A planning review on 2026-10-09 read every bar the committed reports are held to and
looked for ones met with so much room that they no longer guard anything.

The census ([ADR-084][adr-084]) holds each committed report's rows to their standing: a row that
moves from within its bar to over it, or back, fails `cargo xtask validate --check` until a reason
is written. For the two OpenRocket comparisons, the stability margin at rod clearance and the
center of mass (CG) there were judged by 0.5 calibres, the center-of-pressure target M1.8a set
before the transonic normal force was measured. The reports now measure, hpr less OpenRocket, in
OpenRocket's calibres:

| set | quantity | rows judged | range | largest size |
|---|---|---|---|---|
| OpenRocket's examples | margin | 53 | −1.0768 to +0.0764 | 0.0764 without the one at −1.0768 |
| OpenRocket's examples | CG at rod clearance | 54 | −0.0028 to +0.0624 | 0.0624 |
| the private designs | margin | 35 | −0.0166 to +0.1108 | 0.1108 |
| the private designs | CG at rod clearance | 35 | −0.1102 to +0.0198 | 0.1102 |

(The rows `validation/reports/census.json` judges against the bar, and the ranges the summary
lines of `openrocket-flights.md` and `openrocket-library-flights.md` print. One more example
flight's margin is withheld, not judged; the examples report's summary line compares 53 centers of
mass where the census judges 54, with the same range.) The one
public margin at −1.08 is over either bar and stays a counted miss. Every other difference is
within 0.111 calibres, a fifth of the old bar. A change could have made the worst agreement four
times worse and kept every row within it.

A margin is a safety number: a margin that reads larger than OpenRocket's is the flattering side
(the private set's median is +0.031 calibres). A bar that notices a doubling of the worst
difference guards it; one that waits for a fourfold change doesn't.

M1.14, the accuracy milestone, asked that "hpr is at least as accurate as OpenRocket on the same
real flights". Release 0.3 (M10.3) and ADR-209 §7 ask for more: a mean absolute apogee error
strictly below OpenRocket's, 24.12's and its newest release's (ADR-212), and a bias within 3%. On
the private collection today HPR Sim's error is 13.96% against OpenRocket 24.12's 13.08%, and its
mean bias +9.83% (M2.3c1, `validation/reports/fixture-flights.md`). M1.14 is the milestone meant
to close that gap, so it should be held to the release's target, not a weaker one that a tie
meets.

The review also checked the fin-flutter speed against an error reported in RocketPy (its issue
#1200, open on 2026-10-09: the flutter speed √2 too high, from a factor of two applied twice).
HPR Sim's `crates/hpr-sim/src/flutter.rs` follows NACA TN 4197's eq. 18 term by term; for a
fibreglass fin (root chord 0.15 m, tip 0.075 m, span 0.1 m, 3.2 mm thick, G = 4.0 GPa) at 1,000 m
it gives 316.73 m/s against 316.71 m/s worked by hand from the paper, where the doubled factor would
give about 448 m/s. The test `flutter_denominator_matches_tn_4197_eq_18` fails on that doubling.
Nothing changes.

**Decision.**

1. **The margin and CG bar is 0.2 calibres** (`MARGIN_BAR_CAL` in
   `crates/hpr-validate/src/census.rs`). It is the smallest round bar that keeps every row's
   standing with room: the largest difference within it, 0.111 calibres, is 55% of it. A row's
   slack, 0.1% of its scale, goes from 0.0005 to 0.0002 calibres with it. The census was accepted
   again with that reason; every margin and CG row keeps its standing.
2. **M1.14's done-when adds the collection.** It now reads: real flights meet the 5% mean apogee
   target, hpr is at least as accurate as OpenRocket on the same real flights, and on the private
   collection's flights that both fly its mean absolute apogee error is below OpenRocket 24.12's
   and its newest release's (ADR-212), its mean bias within 3%; and M1.8's Cd bullet is met or its
   gap re-measured in an ADR. Nothing is taken out. ADR-143's stop rule still ends M1.14 short of
   this, its gaps in an ADR, as M10.3 already allows.
3. **Left as they are,** each met with room, for a later review: the whole-flight comparison with
   RocketPy on the same drag (at most 3% a metric; flights measure at most 1.81%, but descents
   reach 2.865% of their 3%, and the bars are set metric by metric); the `.ork` export round trip
   (0.5%; the largest measured is 0.000162%); the 1% mass bar (54 of 54 public launch masses
   within it, but 4 of 54 masses at rod clearance over it).

**Consequences.** A change that moves any margin or CG difference past 0.2 calibres now fails the
census check until its reason is written. M1.14 can't be met by tying OpenRocket. No release
waits on this: M10.3 already asked for the same, and nothing is added before any release in the
queue.
