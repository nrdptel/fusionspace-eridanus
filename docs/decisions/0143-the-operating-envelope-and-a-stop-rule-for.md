# ADR-143: The operating envelope and a stop rule for accuracy work; M1.8 closed with its misses (2026-10-03)

- **Status:** accepted
- **Summary:** The operating envelope orders accuracy work: bands by Mach only (core 0–2.5, extended to 3.5, beyond the envelope deferred), angle of attack a separate condition (accuracy assumes ≤ 15°), four warning flags; the stop rule (two increments in a row that neither shrink a measured error nor add a reference end a milestone); M1.8 closed with its misses, its Cd bullet an M1.14 input; four slender-body switches (#87, #120, #121, the vertical tip) first in M1.14d, M1.14h above Mach 2.5; only #108's Mach 4 part and M1.8e16 deferred

**Context.** On 2026-10-03 Neer set out what the accuracy work should serve and when it should
stop: slower development is worth it when it improves accuracy, but only to a point.
Nothing ordered the work by flight
regime. M1.8e has 18 increments done and e16 open, most of them on body lift past Mach 3. Yet
code-to-code whole flights mostly stay below Mach 1.15 and 4 km (the fastest public one is
OpenRocket's example at Mach 1.147; one private flight is supersonic), and real flights reach
about Mach 1.0 and 3.9 km. Where flights on commercial (COTS) motors go:

- The fastest reach about Mach 3.5: record builds, such as minimum-diameter CTI O3400 and N5800
  flights. Tripoli's single-stage commercial altitude records were 13,885 m (45,554 ft) on an M,
  15,614 m (51,228 ft) on an N and 20,040 m (65,748 ft) on an O, in a 2016 snapshot.
- Spaceport America Cup teams in the 9,144 m (30,000 ft) COTS category fly Mach 1.6 to 2.1
  (Concordia's 2018 report, Mach 1.64; UC Aerospace in 2024, "just over Mach 2").
- NASA Student Launch (1,220 to 1,830 m, 4,000 to 6,000 ft) and the American Rocketry Challenge
  (229 m, 750 ft, its 2026-season target) are subsonic.
- The legal wind limit (NFPA 1127: 20 mph, 29.3 ft/s, 8.9 m/s) against a rail exit speed of 50 to
  100 ft/s (15 to 30 m/s) gives an angle of attack of about 30° (at 50 ft/s) to 16° (at 100 ft/s)
  just off the rail. So 15° covers the climb, not the first instant off the rail.

**Decision.**

1. **The operating envelope** sets the order of work, not a ceiling. Mach number and angle of
   attack are two separate axes.
   - **Bands, by Mach number only.** The *core band* is Mach 0 to 2.5; accuracy work goes here
     first. The *extended band* is Mach 2.5 to 3.5, record-class COTS flights: hpr flies it, and
     its accuracy is checked less than the core band's; fidelity work there is welcome once the
     core band has validated whole-flight references. The *envelope* is Mach 0 to 3.5, at any
     angle of attack. *Beyond the envelope* is past Mach 3.5: deferred, not dropped.
   - **Angle of attack, a separate condition.** Accuracy work assumes an angle of attack of at
     most 15°. A flight above 15° more than 1 s after rail clearance is *at high angle of attack*,
     at any Mach number. The first second off the rail is the expected transient (16° to 30° at
     the legal wind limit, above); this is what "for more than a moment" means, and it makes the
     edge testable. The 1 s was chosen, not measured: M1.14e measures the transient and studies
     high angles.

   Every flight still flies. M1.14a will raise four flags, each tested at its edge:
   - (i) *beyond the validated range*: faster than the fastest public whole-flight reference, that
     is, any committed comparison of a public flight against an independent reference, gated or
     not. Today that is OpenRocket's example at Mach 1.147. The threshold is read from the
     committed reports, not hard-coded, and rises as M1.14b adds references. Private flights never
     set it (hard rule 4).
   - (ii) *at high angle of attack*: above 15° more than 1 s after rail clearance.
   - (iii) *outside the core band*: past Mach 2.5.
   - (iv) *beyond the envelope*: past Mach 3.5.
2. **Each accuracy increment names the Mach band it serves.** Work outside the core band needs a
   stated reason. An accuracy issue carries the label of its Mach band, `env-core`,
   `env-extended` or `env-deferred`; a high-angle issue says so in its text and belongs to M1.14e.
3. **The stop rule** replaces any cap on increments. An accuracy increment makes progress when it
   shrinks a measured error against an independent reference, or adds a reference that will.
   Either counts as progress; two increments in a row that do neither end the milestone, gaps
   written down, and the queue moves on.
4. **M1.8 closes with its recorded misses.** Its done-when is not rewritten (hard rule 2); this
   record is the reason. Its Cd bullet (within 10% of RocketPy's RASAero II exports, Mach 0.1 to
   2.0) is **not met**. Calisto's is the only real RASAero II export among them (ADR-029): within
   10% at 15 of 15 subsonic rows, 3 of 7 transonic (2 when first measured) and 8 of 17 supersonic,
   −14.9% to −5.1% (ADR-029, ADR-030). That bullet moves into M1.14 as an input, to be met or
   re-measured (M1.14d). The same-drag supersonic case, Prometheus through Mach 1.010, passes
   (ADR-027); the predicted one (Mach 1.06) is outside its 3% target on apogee (−7.280%) and several
   other metrics, but predicted mode is a target, not a gate (ADR-023), and the case explains its
   miss. The roll balance is met to 1e-11 (ADR-031). M1.8e closes too, its 15% bullet not met for
   the body alone: six of eleven rows outside, Mach 1.5 to 2.96, five of them in the core band
   (ADR-040). The core-band misses become M1.14d inputs, the Mach 2.96 one M1.14h's.
5. **Only the part above Mach 4 is deferred.** Issue #108 (the loading through a tangent-cone
   crossing, which moves with the mesh) bites at Mach 4.63 to 5 on the blunt tip's handover, and
   M1.8e16 would move the 24° cap past it: both are deferred as beyond the envelope, not closed.
   M1.8e16 stays `[blocked]` with this record as its reason, and #108 stays open, `env-deferred`
   for that part. The rest moves into M1.14d, and above Mach 2.5 into M1.14h:
   - **The vertical-tip switch.** The 24° cap binds from Mach 2.06 (ADR-043). A vertical-tip nose
     steeper than the cap's handover all the way to its base keeps slender-body theory past Mach
     1.2: on the tests' straight rocket (four fins) at Mach 3 and 4°, −7.0% in normal force with the
     centre of pressure 0.64 calibres aft, so the rocket reads more stable than it is. Under
     ADR-144's guardrail that is the worst class of error. With #87 (a step in radius: −8.65%, 1.03
     calibres aft) and #121 (a pointed tip past the cone tables' 30°: −7.7%, 0.81 calibres aft),
     both on the same rocket, and #120 (a flare behind a boattail, too long for its wake: −27.5%,
     0.29 calibres aft, measured on a finless body, so a share of a body's normal force alone and
     not comparable with the other three), it is one of four *switches that fall back to
     slender-body theory and overstate stability*, ranked first among M1.14d's flattering-side
     items. All four are sized at Mach 3.
   - **#108's extended-band part**, now M1.14h's: the near-flat flare's crossing, which steps
     −2.77% and 0.14 calibres between Mach 2.90 and 2.95 on a short-shouldered body.
6. **M1.14 *Accuracy inside the envelope*** joins Phase 1. Done when: real flights meet the 5%
   mean apogee target, hpr is at least as accurate as OpenRocket on the same real flights, and
   M1.8's Cd bullet is met or its gap re-measured in an ADR. Inputs: the switches above; M1.8's
   Cd bullet and M1.8e's body-alone misses; the core-band issues #67, #68, #70, #73, #172, #219
   and #222. Increments, in order:
   - a. Warnings: the four flags of §1 (*beyond the validated range*, *at high angle of attack*,
     *outside the core band*, *beyond the envelope*), each tested at its edge; the aero page split
     by Mach band, each band page short with detail beneath.
   - b. Validation from Mach 1.5 to 2.5 and new references: the Duke Rocket Flight Database
     (CC BY 4.0; 28 flights, Mach 0.54 to 7.22, apogees with RASAero II and OpenRocket-Plus
     predictions; https://github.com/AidanSYu/rocket-flight-database,
     https://zenodo.org/records/20531977); the Basic Finner's free-flight data (DREV-TM-9703) and
     ARL-TN-1128's measured supersonic coefficients (HSARV); NASA NESC's 6-DOF check cases
     (https://ntrs.nasa.gov/citations/20150001263); ASME V&V 20 validation uncertainty per real
     flight; tip-off against RocketPy PR #920 (MIT); wind reconstructed from the GPS descent;
     MAPLEAF (MIT) as another oracle; a Mach 2.45 tunnel drag point (doi 10.2478/tar-2024-0018,
     licence to check).
   - c. Transonic normal force and drag, Mach 0.8 to 1.2.
   - d. Supersonic, Mach 1.2 to 2.5: the four switches first, then drag (Calisto −14.9% to −5.1%
     against RASAero II; #222) and M1.8e's body-alone misses in the core band (Mach 1.5 to 2.3).
   - e. Large angles of attack: measure the cost at the 20 mph legal limit; past 1% on apogee or
     drift, a high-angle model (Jorgensen's non-linear body lift; the report cached). It also
     measures how long the post-rail transient lasts, revisits §1's chosen 1 s, and takes the
     high-angle issues.
   - f. The audit's physics gaps, ranked: asymmetries (thrust and fin misalignment, a lateral CG
     offset; as inputs and in Monte Carlo, RocketPy the oracle); airframe drag in recovery;
     power-on base drag validated (the rule moves one supersonic flight about 24 points); gusts
     in flight (Dryden, opt-in, in Monte Carlo); parachute opening loads (Knacke, about 1.7 to
     1.8) and swing; laminar friction; interference and fillet drag; default nozzle exit sizes;
     the separation charge's push; the grain's CG shift. Magnus checked: negligible, recorded.
   - g. Figures: every accuracy and aero page gets generated plots, the reference overlaid and
     the error shown, regenerated and checked in CI.
   - h. The extended band, Mach 2.5 to 3.5, after the core-band increments. Its stated reason is the
     flattering-side stability errors measured there. Done when each of these is fixed or
     re-measured in an ADR: the four switches' errors as measured at Mach 3 (flattering: the centre
     of pressure moves aft); #108's step at Mach 2.90 to 2.95 (−2.77% and 0.14 calibres; which way
     it errs is not yet measured); and M1.8e's Mach 2.96 body-alone row, +16.9% in the body-alone
     `C_Nα`, 77% of that gap hpr's own zero-α slope (ADR-040; `arcas-robin-body-gap.json`,
     `zero_alpha_share_of_gap` 0.7736). On the finned rocket that errs conservative: at Mach 2.96
     the finned Arcas Robin's centre of pressure reads 0.19 (short) and 0.43 (long) calibres forward
     of the tunnel's (`normal-force-vs-mach.json`, `cp_error_calibers`).

**Consequences.** The validation plan gains an *Operating envelope* section, and the aero page's
switch table points the four switches at M1.14d, and at M1.14h above Mach 2.5. Nothing in the
physics changes here; the next increments are ordered by band, flattering-side errors first, instead
of by the largest error anywhere.
