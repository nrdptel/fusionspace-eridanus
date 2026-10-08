# ADR-101: OpenRocket's mass conventions rolled up; M2.2 left open for two lessons (2026-09-28)

- **Status:** accepted
- **Summary:** OpenRocket's mass conventions rolled up; M2.2 left open for two lessons

**Context.** M2.2b had three conditions. The tests of Loft lessons L51 (which mass or centre
override wins) and L87 (stored results used as references) must be live. Every convention
ADR-060 lists, and roll inertia, must be hpr's rule or a written departure. And M2.2a's survey
must be run again. Its five parts (ADR-061 to ADR-065) are met. Later decisions changed some
answers: pods (ADR-089 to ADR-091), clusters (ADR-075), fillets (ADR-096) and tube fins
(ADR-098). This roll-up says where each convention stands now, and why M2.2 cannot close with it.

**Decision.**

1. **Each convention ADR-060 lists, where it stands.** Every departure below is held by a test.
   Other departures the probes found along the way are in the guide: the elliptical fin's outline
   on the mass page, a mass component's packed length in the `.ork` format guide.

   | convention | now | settled by |
   |---|---|---|
   | a shoulder written with no wall | OpenRocket's rule: it weighs nothing | ADR-061 |
   | a part weighed by volume, with no material | OpenRocket's rule: 680 kg/m³ | ADR-061 |
   | which override wins (L51) | OpenRocket's mass on every probe; the centre under a part's covering mass override that states no centre is a departure (3.7 mm; 19.7 mm when the part inside has its own override); flags that disagree are a limitation (4.7 mm) | ADR-061 |
   | inertia under a mass override | departure: hpr scales all it covers | ADR-061 |
   | rounded and airfoil fin sections | departure: hpr's cited sections | ADR-062 |
   | roll inertia | explained by OpenRocket's fin shortcut; hpr keeps its exact integral | ADR-062 |
   | packed parts with no size, overrides on weightless parts | OpenRocket's rule | ADR-063 |
   | clusters | every tube weighed where it is; OpenRocket stacks them on the cluster's axis, so roll and pitch depart | ADR-075 |
   | fin fillets | OpenRocket's mass and centre; pitch apart by −0.64% to +0.43%, pinned: hpr's is the exact prism, OpenRocket's rule for fins unmeasured | ADR-096 |
   | pods | OpenRocket's mass and centre, and roll with its fin shortcut; pitch with one or two pods that have a length departs (0.3% to 1.1%); an override on an empty pod set is dropped | ADR-090, ADR-091 |
   | tube fins | OpenRocket's radius and mass; roll and pitch depart | ADR-098 |
   | parts hpr still keeps unread (parallel stages, some fin sets) | departure: kept whole, design marked reduced | ADR-058, ADR-064 |
   | stored results (L87) | references only when current and plausible | ADR-065 |

2. **L51 and L87 are owned by M2.2b.** Their lesson rows named only M2.2, so the check that a
   lesson's tests are live would wait for M2.2. Each row now names M2.2b first. Checking M2.2b
   off makes `cargo test -p xtask` hold both tests live. A row pointed at a test that does not
   exist fails it.
3. **M2.2 stays open for M2.2f, which takes L19 and L82 from M2.2e.** ADR-060 §1 gave both
   lessons to increment e, and M2.2e was checked off on the parent's bar without them. This
   supersedes that assignment. Their tests do not exist: `tube_fin_cp_within_0_25_cal_of_oracle`
   and `excused_cases_stay_in_the_census_statistics_against_both_references`. L19's bar is not
   met today. On the *Tube fin rocket* at rod clearance, hpr's margin is 0.79 calibres against
   OpenRocket's 1.87 (ADR-099). The two centres of mass there are 0.002 calibres apart
   (`validation/reports/openrocket-flights.json`), so the gap is the centre of pressure's:
   hpr's is 1.07 calibres forward of OpenRocket's, further than Loft's 0.9. M2.2f is done when the
   test asserts L19's 0.25 calibres and passes. If that cannot hold, an ADR gives the measurement,
   and the lesson's row names a test that pins the gap and says so, as L18's row does.

**Evidence.** `cargo xtask ork`, run on 2026-09-28 with OpenRocket's record unchanged, passes on
71 files, 51 distinct by content:

| quantity | within 0.1% | within 1% | median |
|---|---|---|---|
| mass | 60 of 71 | 68 of 71 | 0.001% |
| centre of mass (share of length) | 65 of 71 | 68 of 71 | 0.000% |
| pitch inertia | 41 of 71 | 56 of 71 | 0.065% |
| roll inertia | 10 of 71 | 29 of 71 | 1.686% |
| roll inertia, with hpr using OpenRocket's fin shortcut | 57 of 71 | 57 of 71 | 0.001% |

- Three files, two by content, are outside the mass or centre-of-mass threshold, each with a
  cause: a parallel stage kept unread, and fins weighed by hpr's section. ADR-060 had 17 files,
  12 by content.
- Roll inertia is outside 1% on 14 files even with OpenRocket's shortcut. By content they are 9,
  each with a cause: a mass override (5), a cluster's tubes (2), a part kept unread (1) and tube
  fins (1).
- Pitch inertia is held to no threshold (ADR-060 §4). The mass page says which of the 15 files
  outside 1% have a cause and which do not.
- The mass page's table is this run's, number for number.

**Consequences.** M2.2b is checked off. The next increment is M2.2f, then M4.1. Replacing any
departure above takes new probes, not a quiet change.
