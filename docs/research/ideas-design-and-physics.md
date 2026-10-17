# Ideas: design, build, physics and trust

Part of the [ideas backlog](ideas.md) (2026-10-03). One line each, with its tier.

## Design and build

- SOON: a kit library, built from makers' published specs.
- SOON: reverse design from requirements.
- SOON: design-diff apogee attribution (which change moved the apogee, and by how much).
- LATER: a quality-diversity design atlas, a map of good designs that differ from each other, by
  MAP-Elites or CMA-ME (quality-diversity search algorithms; [arXiv 2504.02177](https://arxiv.org/abs/2504.02177)).
- LATER: evolutionary fin shapes.
- LATER: freeform fins traced from an image.
- SOON: a fabrication pack: printable templates and marking guides, fin jigs. Its DXF and SVG fins
  and rings and its STL nose cones are PROMOTED to [M8.3a and b, CAD interop][m8-3].
- PROMOTED to [M8.3d][m8-3]: STEP export.
- LATER: live Onshape and Fusion parametric add-ins.
- PROMOTED to [M8.3e and f][m8-3]: import a shape from CAD or STL as a custom part.
- SOON: per-part drag coefficient against Mach sweeps.
- SOON: custom expressions.
- SOON: external aerodynamic tables per component, against angle of attack and Reynolds number
  (extends the existing overrides).
- LATER: per-part forces exported for finite-element analysis.
- SOON: as-built mode: measured mass, center of gravity (CG) and a swing test update the model.
- SOON: material mass (fiberglass, epoxy, paint, 3D-print infill).
- SOON: composite layup stiffness (classical laminate theory) feeding flutter.
- SOON: measured fin torsional stiffness as an input, with a DIY twist test.
- SOON: a fin resonance warning.
- SOON: flutter as a spread across methods.
- SOON: blunt trailing-edge fins.
- SOON: motor hardware fit lookup (cases, spacers, retainers).
- SOON: launcher selection (rod whip, rail size, button loads).
- SOON: drag plates sized to cap the altitude.
- LATER: boosted darts and drag separation.
- SOON: scaling a design.
- LATER: spin cans.
- SOON: fins that differ by axis ([OpenRocket #1548](https://github.com/openrocket/openrocket/issues/1548)).
- SOON: streamer and tumble descent improvements.
- LATER: oddrocs and short, fat rockets.
- SOON: camera shrouds and other protrusions.
- LATER: a photo studio and decals.
- LATER: a phone scan compared with the as-built model.
- SOON: one-line parameter sweeps (`hpr sweep`).

## Physics and trust

NEXT, already in [M1.14f, the audit's physics gaps][m1-14f]: asymmetries (thrust and fin
misalignment, a lateral CG offset), airframe drag in recovery, power-on base drag, gusts in
flight, parachute opening loads and swing, laminar friction, interference and fillet drag,
default nozzle exits, separation-charge push, grain CG shift. The rest:

- SOON: propellant temperature correction.
- SOON: effective rail length from the rail buttons ([OpenRocket #2507](https://github.com/openrocket/openrocket/issues/2507));
  the Spaceport America Cup's IREC (Intercollegiate Rocket Engineering Competition) and the
  European Rocketry Challenge (EuRoC) each define it.
- SOON: stability judged by shape, by dynamic stability and by competition rules together.
- SOON: structural loads, fin divergence and canted-fin roll resonance.
- SOON: aerodynamic heating past Mach 2, labeled lightly validated.
- SOON: a tuned model per rocket (a digital twin; Kennedy and O'Hagan's calibration).
- SOON: two-knob truing: impulse against drag per Mach band, from ballistics.
- SOON: normalize a flight to standard conditions.
- SOON: why-simulators-disagree attribution.
- SOON: "explain this number".
- SOON: a credibility card (NASA-STD-7009, NASA's standard for models and simulations).
- SOON: an accuracy band on every result.
- SOON: exact sensitivities (automatic differentiation).
- SOON: subset simulation for rare events.
- SOON: Wilks 95/95 bounds (order-statistic tolerance bounds: how many runs cover 95% of outcomes
  with 95% confidence) and a sample-size advisor.
- SOON: a polynomial-chaos surrogate.
- LATER: a learned fast surrogate for the web.
- SOON: a mid-fidelity panel-method aerodynamics tier: see Ironbark, a published panel-method
  code ([doi 10.1016/j.ast.2026.111758](https://doi.org/10.1016/j.ast.2026.111758)), and PANAIR,
  NASA's supersonic panel code (its license to check).
- LATER: flexible-airframe dynamics.

From the 2026-10-09 planning review:

- SOON: the apogee across surface finishes. User: an L2 flyer who can't name a paint's roughness.
  Workflow: `hpr sim` prints the apogee at the finishes either side of the chosen one. Why: a 2026
  study measured 17–75% more drag from manufacturing roughness (Seniwan and Mohd Saiah). Moves: the
  share of the collection's logged apogees inside that band, not measured today. *Done when:* the
  fixture report gives it with two-sided 95% Clopper–Pearson bounds, failed runs counted as misses.

From the 2026-10-08 planning review:

- SOON: weighed against estimated mass at import. User: a flyer whose apogee is 20% off. Workflow:
  reads which masses are weighed (overrides, and what each covers) and which estimated, with each
  one's effect on the apogee. Moves: the collection's apogee error split by the share of liftoff
  mass weighed. *Done when:* `hpr sim` prints that share and each override's scope; the fixture
  report gives the error above and below the median share, fixed before the errors are read, as
  aggregates.

From the 2026-10-10 planning review:

- SOON: every flight's energy closes. User: a flyer showing a waiver flight to a safety reviewer.
  Workflow: `hpr sim` prints the work done by thrust, drag and gravity beside the change in kinetic
  and potential energy. Why: in October 2026 RocketPy 1.13 still printed its potential energy with
  the wrong sign (its issue #1202). Moves: the share of the report's flights whose energy residual
  is below 1e-6 of the impulse's work, not measured today. *Done when:* the fixture and public
  reports give that share, and a mutation of drag's sign fails a test.
- SOON: oracle aborts named, not counted as misses. User: a reader of the OpenRocket comparisons.
  Workflow: the census lists each flight OpenRocket ends early by its event (a 2026-10-07 fix in its
  development line removed a false tumble abort on finless boosters; 24.12 still has it). Moves: the
  OpenRocket examples within 5% (45 of 53 today), with aborts kept apart. *Done when:* the
  OpenRocket reports count aborts by event, and a probe design with a finless booster shows one.
- SOON: thrust at altitude, bounded. User: a flyer at a field 1,400 m up writing a waiver. Workflow:
  `hpr sim` gives the apogee with and without the ambient-pressure thrust term for a catalog motor,
  whose file has no nozzle; the exit is the drawn one where published, else bounded by the
  nozzle cap's bore (AeroTech RMS) or the aft boss (Cesaroni), an inferred bound 2 to 6 times below
  the case's area ([motor data](motor-data.md) §2), else the case's area, a sea-level test assumed (a test site above the field would make the term negative). Why: OpenRocket's development line added the
  term on 2026-09-23 (its PR #3327). Leaving it out errs low, the flattering side for a waiver; the
  term raises the apogee, so it grows the collection's +8.73% mean bias. Moves: that bias, and
  hpr-less-OpenRocket once M2.3c3 flies its newest release. *Done when:* `hpr sim` prints both; the
  fixture report gives the bound's median and largest as aggregates; a test pins it by hand and
  that the bounded apogee is never below the other.
- SOON: rail tip-off. User: a flyer of a heavy rocket leaving a short rail slowly. Workflow:
  `hpr sim` lets the rocket pitch about the aft guide, which slides along the rail, between the
  forward guide's release and the aft one's (not modeled today), and prints the pitch rate and angle
  of attack at rail exit. Why: RocketPy's development line added that phase in September 2026 (its
  PR #920). Moves: angle of attack at rail exit against RocketPy, not measured today. *Done when:*
  a body on a frictionless line under gravity matches the energy integral
  ω² = 2mgd(sin θ₀ − sin θ)/(I_cg + md² cos² θ), and RocketPy's example's pitch rate is within 5%.
- SOON: motor curves checked against the certifier. User: a flyer simulating offline on a cached
  curve. Why: in September 2026 a contributor replaced 63 ThrustCurve curves and had 70 deleted
  for rows doubled at time zero, times out of order and impulse off the certification
  ([its thread][tc-thread]); NAR's retests of 17 motors, F and smaller, ran a median 9.4% below
  their older test mean, which the catalog keeps on 15 ([motor data](motor-data.md)). Workflow:
  `hpr motors check` flags each defect class and an impulse off the certifier's latest mean beyond
  its tolerance; `hpr sim` also flies the certifier's impulse, thrust scaled at a fixed burn time
  and propellant with it, each check taking its worse side. It also flags a curve whose samples
  duplicate another motor's: one catalog H255 curve was another motor's file, reported
  2026-10-11 on [OpenRocket #2348][or-2348]. Moves: `bias.public` and `bias.collection`, flown
  both ways. *Done when:* a fixture per defect class is flagged and a
  clean curve isn't; both reports give the bias with flagged flights at the certifier's mean, and
  their count (groups under 5 flights as a count only).
- LATER: aero tables OpenRocket can read. User: a flyer checking a design in both tools. Why:
  OpenRocket's development line reads drag and stability lookup tables, documented in its user
  guide ([a forum thread][or-tables], 2026-10-03). Workflow: `hpr aero` writes HPR Sim's drag and
  center of pressure against Mach in that format for OpenRocket to fly. Moves: [M2.3c6][m2-3c6]'s
  split of the 0.93-point gap in `accuracy.collection`, isolating aerodynamics. *Done when:* on 10
  public designs and the collection (as aggregates), both tools' apogees with and without the
  tables are published; a test pins the file to bytes derived by hand from the format's docs.

From the 2026-10-15 planning review:

- SOON: a measured reference for a tangent ogive's wave drag. User: a flyer of a minimum-diameter
  rocket past Mach 1, where the nose is most of the pressure drag. Workflow: the aerodynamics
  validation report scores HPR Sim's nose pressure drag against a published wind-tunnel measurement
  of ogive-cylinders, the half of [#222][i-222] that the OpenRocket comparison can't settle. Moves:
  a new supersonic nose-drag row in `validation/reports/latest.md` (none today);
  `accuracy.supersonic` only if a model change follows. Tracked in [#616][i-616], from [the note on
  ogive wave drag](ogive-wave-drag.md). *Done when:* the report gives the error and its sign at
  three or more Mach numbers from 1.3 to 3 (to 2.5 the core band, past it the extended) on at least
  two fineness ratios, at zero angle of attack on the base area, each against a pressure-only
  reference (integrated surface pressures, or forebody drag with its friction correction stated)
  cited to its table or figure, and a test pins each case.

From the 2026-10-17 planning review:

- LATER: roughness from a measurement. A part's finish takes a measured Ra or Rz, or an FDM layer
  height, mapped to a sand-grain band by [hobby finish roughness](hobby-finish-roughness.md), and
  the apogee is flown at both ends of the band. User: a flyer with printed fins. Moves: the apogee
  band from finish, per design. *Done when:* each mapping is cited and pinned, both ends print, and
  a value with no unit is refused.
- LATER: a boattail's center of pressure against tunnel data. User: a builder adding a boattail,
  where forum threads report simulators moving CP forward too far. Moves: a new CP row in
  `validation/reports/latest.md`. *Done when:* three or more boattail angles at two or more Mach
  numbers from a published tunnel test give HPR Sim's CP error and its sign, each pinned by a test.

[m1-14f]: ../decisions-and-roadmap.md#m1-14f
[m8-3]: ../decisions-and-roadmap.md#m8-3
[tc-thread]: https://www.rocketryforum.com/threads/thrustcurve-open-thread.164391/page-30
[or-tables]: https://www.rocketryforum.com/threads/omg-openrocket-26xx-sapshot-drag-csv-and-stability-csv.199161/
[or-2348]: https://github.com/openrocket/openrocket/issues/2348
[m2-3c6]: ../decisions-and-roadmap.md#m2-3c6
[i-222]: https://github.com/nrdptel/fusionspace-eridanus/issues/222
[i-616]: https://github.com/nrdptel/fusionspace-eridanus/issues/616
