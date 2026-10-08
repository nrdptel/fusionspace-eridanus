# ADR-102: Tube fins' centre of pressure measured against OpenRocket; L19's bar not met, the gap pinned (2026-09-28)

- **Status:** accepted
- **Summary:** Tube fins' centre of pressure measured against OpenRocket; L19's bar not met, the gap pinned

**Context.** Loft lesson L19 asks that hpr put a tube fin set's centre of pressure within a quarter
calibre of OpenRocket's; Loft's was about 0.9 calibres forward of it. M2.2f is done when a test
asserts that bar and passes, or, if it cannot hold, when an ADR measures the gap and L19's row
names a test that pins it, as L18's row does (ADR-101 §3). On OpenRocket's *Tube fin rocket* hpr's
centre is 1.07 calibres forward of OpenRocket's (ADR-099). ADR-099's account of where that comes
from, OpenRocket's slope and centre for the tubes, was a look at its per-component output that
was never kept. A search for a measured tube-fin centre of pressure found none: not in NACA or NASA
reports, Apogee's newsletters (issues 27, 119 and 335) or OpenRocket's own pull requests, whose
author wrote that "we won't really know until we can find a wind tunnel" (openrocket#1413).

**Decision.**

1. **OpenRocket's answers are kept as a record.** `validation/oracles/openrocket/tube_fin_aero.py`
   runs OpenRocket 24.12 as an external oracle. It writes 14 probe designs: one nose and body, with
   a tube fin set at the foot that varies one thing at a time. The variations are the tubes'
   length (25 to 300 mm), their count at a stated radius (3 to 8), their radius and their wall.
   The script also reads the *Tube fin rocket* from the jar. It records the slope `C_Nα` and the
   centre of pressure its Barrowman calculator gives each component at Mach 0.05, 0.3, 0.5, 0.6
   and 0.75. Each answer comes from a new load and a new calculator, because a calculator asked
   again after a part changes gives its cached answer. The record is
   `validation/fixtures/ork/openrocket-tube-fin-aero.json`.
2. **What OpenRocket does, from its output alone** (its source is GPL and was not read):
   - The tubes' slope is the same at every Mach number and grows with the count, the length and
     the bore.
   - On every probe it is 1.26 to 1.86 times hpr's, and 1.20 to 1.86 times `N π d²/A_ref`. That is
     the long-ring limit of `N` isolated thin rings of mean diameter `d` (Hoerner 1965, p. 7-13),
     which Weissinger's
     formula approaches from below as a ring lengthens.
   - Its slope per tube is the same, to 1e-15, for 3, 4, 5, 6 and 8 tubes of 6 mm. So it
     models no interference that changes with the count. Slender-body theory with the body
     included does predict one, and it falls as the count rises. A 2D apparent-mass solve run
     during review, not kept here and not checked (#234), puts three such tubes at 1.96 times as
     many isolated rings, four at 1.86, five at 1.72, six at 1.57, eight at 1.30, and six touching
     each other at 1.13, each at a gap of 0.005 radii and still rising as the gap closes.
     OpenRocket's slope is 1.73 times the long-ring limit on the 6 mm probes, and 1.62 on the
     touching one. So it is below that estimate for three to five tubes, and above it for six,
     eight and the touching tubes. At five the two are close: a finer solve, at 0.0005 radii, gives
     1.79, about 3.5% above OpenRocket. Nothing measured supports either code.
   - Its centre is a quarter of a tube's length aft of the leading edge up to Mach 0.5, and at the
     leading edge from Mach 0.6. So the *Tube fin rocket*'s own centre of pressure moves 0.73
     calibres forward between those two speeds in OpenRocket. OpenRocket's pull request #3235,
     merged on 2026-08-11 after 24.12, calls that jump a bug and fixes it.
   - Its pull request #3262 describes the quarter chord as the subsonic rule that conventional fins
     and tube fins share: a flat fin's rule, not a ring's.
3. **hpr keeps ADR-099's ring wing.** OpenRocket's centre runs against the evidence there is, and
   nothing measured supports its slope:
   - Fletcher's measured centre moves forward, as a share of the length, as a ring gets longer:
     0.355 of the chord at `A = 3`, 0.143 at `A = 2/3`.
   - Hoerner and Borst (*Fluid-Dynamic Lift*, 1985, p. 19-16) take the lift of the air turned
     inside an open tube as `2α` on its frontal area, "assuming that the turning takes place at or
     near the rim of the inlet". That assumption is the leading edge hpr's line runs to, which
     ADR-099 derived. They write that they "do not have suitable experimental results at hand"
     on the influence of an axial duct on a slender body's lift and moment. On the same page they
     treat Fletcher's thick `A = 1/3` ring as a ducted body, and find its measured slope
     "practically the same" as that treatment gives. Fletcher measured that ring's centre ahead of its leading
     edge.
   - hpr leaves out the body's interference that slender-body theory predicts (#234). With it,
     hpr's slope would rise. On the three-tube probe the gap would all but close, but only by
     cancellation: a larger slope placed at hpr's centre, 0.03 of the length, not OpenRocket's
     0.25. On the touching tubes it would fall from 1.04 to about 0.89 calibres. That is not
     measured either, and it is a separate change.
   - OpenRocket's pull request #1413 says its tube-fin method comes from "a paper cited in the
     code". hpr cannot see which without reading GPL source, and no measurement supports its
     numbers. Taking them to pass a code-to-code bar is what the first hard rule forbids.
4. **L19's row names the test that pins the gap:**
   `hpr_validate::openrocket::tests::tube_fin_cp_against_the_oracle_measured_and_pinned`. The test
   builds every probe with `hpr_io::ork` and `hpr_aero`, and holds three things to the record:
   - the geometry, and the nose and body, which agree;
   - OpenRocket's rule as listed above;
   - each gap, to 5e-4 calibres.

   Each of the 70 gaps, 14 probes at five Mach numbers, is pinned in a table.

   It also shows that the whole gap is the tubes'. Given OpenRocket's slope and centre for the
   tubes, hpr's rocket has OpenRocket's centre of pressure within 0.01 calibres, whatever the probe
   or Mach number. A change in either code fails it. Meeting L19 would too, and the row would then
   have to change.
5. **L82's test is live:**
   `hpr_validate::tests::excused_cases_stay_in_the_census_statistics_against_both_references`.
   - Every number the reports compare is a census row, with the report's own difference: the
     harness's metrics, the six OpenRocket metrics of every example and library flight, and each
     log's apogee and climb. A scored one is never withheld, the flights with written causes
     (ADR-073) among them, and a real flight with a written explanation stays a miss.
   - Every group's apogee spread and bar counts are over all its compared rows.
   - A harness rocket that RocketPy flew, and whose team logged it, counts against both. On a
     flight whose miss is explained, the two references are more than 5% apart.
   - Mutations of the census each fail the test: withholding the misses of any one scored
     metric (apogee, max speed, margin, climb), withholding any one mass or centre-of-mass metric,
     dropping misses from the spread, and dropping them from the bar counts.
   - The results stored inside `.ork` files, which M2.2 also names as a reference, never enter the
     census (ADR-065), so this test does not cover them.
6. **M2.2 closes.** Its *done when* was met in M2.2e10 (ADR-100), and its last two lessons are now
   live.

**Evidence.** `cargo test -p hpr-validate --lib tube_fin_cp_against_the_oracle` on the record.
hpr's rocket centre less OpenRocket's, in calibres:

| probe | Mach 0.05 | 0.3 | 0.5 | 0.6 | 0.75 |
|---|---|---|---|---|---|
| six tubes touching the body and each other, 75 mm long (the *Tube fin rocket*'s build) | −1.043 | −1.050 | −1.065 | −0.363 | −0.387 |

- On that probe at Mach 0.05, OpenRocket's centre alone moves hpr's 0.495 calibres aft, and its
  slope alone 0.532.
- Up to Mach 0.5, no probe is within the bar: the gaps run from 0.420 to 2.998 calibres. From Mach
  0.6, 5 of the 28 are within it: the 25 mm and 300 mm tubes at both speeds, and the 3 mm wall at
  Mach 0.6. Those five probably meet the bar only because of the jump that #3235 fixes; no
  OpenRocket with that change was run.
- On the *Tube fin rocket* itself at rod clearance (Mach 0.056), hpr's centre is 1.075 calibres
  forward of OpenRocket's (`openrocket-flights.json`). The record's own answer for the example is
  1.4e-5 m from the flight record's.

**Consequences.**

- The flight report's 1.08-calibre margin gap on the *Tube fin rocket*, accepted in writing with
  ADR-099, stands; the guide's tube-fin section now quotes the record.
- [#234](https://github.com/nrdptel/hpr-sim/issues/234) holds the body's interference, and the
  first-order wording of ADR-099's "sums to zero", which the guide and the module doc now qualify.
- [#228](https://github.com/nrdptel/hpr-sim/issues/228) stays open for a measured source.
  Weissinger's lifting-surface coefficients for thin rings (Bagley, Kirby and Marcer, ARC R&M 3146,
  1961) are a lead for a thin ring's centre down to `A = 1/2`; they are not checked here.
- The next milestone is M4.1.
