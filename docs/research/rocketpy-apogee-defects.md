# RocketPy defects that move an apogee by more than 0.5%

This note lists the defects open in RocketPy's tracker that its own reports say move an apogee, with
the size each report gives. It feeds [M2.9's RocketPy cells](../ROADMAP.md), whose rule is set by
[ADR-246, the oracle-defects decision][adr-246]: name every open defect sized above 0.5%, and mark
the unsized ones. Read on 2026-10-18. The latest release is **1.13.0** (GitHub and PyPI, 2026-07-22);
the `develop` branch's head is `a50d07d` (2026-09-14). Every size below is a report's own number, on
RocketPy's own test rockets, unless it is marked **Inference** or credited to this project.

**In short.**
- **One open defect is sized above 0.5% by its own report, and it is off the path M2.9 flies.**
  PR #1195 moves the windy Calisto's apogee from 4,729 to 4,643 m (−1.8%), but only for flights that
  ask for `equations_of_motion="solid_propulsion"`. RocketPy's default, `"standard"`, never runs the
  faulty line, in 1.13.0 or on `develop` (code read; **Inference**: the seven public flights and the
  project's oracle use the default, so the fix changes none of their numbers).
- **The default path's open defect is sized below 0.5% by its own report; its fix with another is
  above it by this project's measurement.** Issue #1186 (fix: PR #1196) moves Calisto in wind by +0.13% and Defiance by
  +0.045%. On the project's flights, #1196 with the merged #1188 moves RocketPy's apogees by up to 1.6%
  ([ADR-026][adr-026]); the oracle already flies both.
- **No fix since 1.13.0 is released.** The open release PR for 1.14.0 carries `develop` as it stands,
  so #1188 should ship in it; its text adds #1190 and says every other PR waits for 1.15 or later.
- **Three open defects are unsized:** a wind and heading convention (#717), a rotation in the generic
  aerodynamic surface (#932), and a dropped center-of-mass acceleration term (#1185, sized only on a
  hybrid). None has a fix in a release.

## How the list was built

- **Open items:** 61 on 2026-10-18 (45 issues, 16 pull requests), counted from the REST listing with
  `state=open` [1]. Each title was read, and the bodies of the 11 touching flight physics.
- **Closed since the release:** 131 items closed after 2026-07-22 00:24 UTC (41 issues, 90 pull
  requests, 78 of them merged), counted from the listing sorted by update time [2]. Titles were
  screened for drag, thrust, mass, wind, energy, rail, equations of motion, atmosphere and recovery.
  Of those that matched, the 8 that could change a trajectory were read, with the unmerged #1192 and #1193.
- **The changelog was not trusted alone.** Issue #1173 reports it 37 merged PRs behind, and #1188 is
  missing from its Unreleased section [3]. Merge state comes from each PR's record.

## (a) Open, sized above 0.5% by its own report

**PR #1195, angular rates zeroed in the solid-propulsion equations** [4]. Open, approved by a
maintainer on 2026-09-19, not merged; head `f3c0c86`, against `develop`.
- **Cause.** `Flight.u_dot` sets the three body rates to zero right after reading the state
  (`flight.py:1931` in 1.13.0), so no aerodynamic damping acts. The PR deletes that one line.
- **Size and sign, from the PR's table** (Calisto, 5 to 15 m/s wind, flown to apogee): apogee
  4,729 m before, 4,643 m after (−1.8%), and 4,632 m on the default equations. Peak pitch rate falls
  from 12.0 to 0.13 rad/s. The PR says the two equation sets then agree to about 0.25% on apogee.
- **Which flights it touches.** `Flight.__init_equations_of_motion` binds `u_dot` only when
  `equations_of_motion == "solid_propulsion"`; the default is `"standard"` (`flight.py:506`,
  `:1655-1660` in 1.13.0; the same test on `develop`) [5]. None of RocketPy's example notebooks set
  it; two test files and the user guide do (`docs/user/flight.rst:829`, which calls it faster "with
  no accuracy loss"). The PR's sentence that every `SolidMotor` rocket is mapped to these equations
  automatically is contradicted by that code (**Inference**).
- **Flying with the fix:** possible from the public branch `bug/solid-propulsion-angular-rates`.

## (a2) Open, sized below 0.5% by its own report

- **#1186, the center-of-mass lever arm's sign in `u_dot_generalized`** [6]. Open; its fix, PR #1196,
  is open and unreviewed, head `927e771` [7]. Calisto in custom wind: 3,423.27 to 3,427.72 m (+0.13%).
  Defiance, on top of #1188: 9,243.84 to 9,247.97 m (+0.045%).
- **#1185, the CM offset's second derivative at ignition** [8]. Open, no PR. On calisto_hybrid_modded,
  removing the term moves apogee 0.90 m of 4,405 m (0.02%).
- **#1202, the sign of `Flight.potential_energy`** [9]. Open; its fix, PR #1204, is open, head
  `cb85f0a`. No apogee change: the report says only the energy outputs move, not the trajectory.

- **#1186's size depends on the rocket.** Its own report calls the apogee change small and the
  lateral change large: on the windy Calisto, the x velocity at apogee goes from −14.83 to −11.63 m/s.
  The moment lever from the center of mass to a center of pressure is `cp + r_CM` instead of
  `cp − r_CM`, which on Calisto at ignition is 0.132 m against a 0.261 m arm.
- **This project's measurement is larger.** With #1188 and #1196 applied, RocketPy's apogees on the
  project's flights move by at most 1.6% (Prometheus 2022; Juno III +1.0%), and Juno III's apogee gap
  to HPR Sim goes from +1.71% to +0.70% ([ADR-026][adr-026]). So by ADR-246's own-report rule #1186
  falls under the bar, while the two fixes together move the flights M2.9 scores by more; no source
  separates #1196's share.
- **#1185 has a second half without a size.** For an array-sourced, linearly interpolated CM offset,
  which is the solid Calisto's case, the second derivative is identically zero, so the report says
  the `−m r_CM''` term is dropped everywhere. No flight is flown with it restored.
- **Flying with the fix:** #1196 from its public head (the oracle's `corrections.py` already applies
  its three edits to 1.13.0); #1204 from its head, though it moves no trajectory.

## (b) Open and unsized, plausibly touching apogee

- **#717, launch heading against the wind's components** [10]. Open since 2024-10-24. A user's
  crosswind flight with heading 11.6° flew west instead of north. A maintainer replied the same day
  that something is wrong with the `wind_u` definition, present since v1.0 or earlier, and later
  pointed at the `Flight.u_dot` methods. No size, no sign, no fix. A crosswind's apogee loss depends
  on its direction relative to the heading, so a swapped convention could move a windy apogee either
  way (**Inference**; unbounded).
- **#932, the wrong rotation in `GenericSurface.compute_forces_and_moments`** [11]. Open since
  2026-03-05. A comment's check at α = 0.2 rad and β = 0.1 rad, against the direction-cosine mapping:
  drag keeps its axial part but its lateral parts flip sign, and lift and side force flip their axial
  parts. The fix sits inside PR #1064 (open; no longer a draft); a maintainer asked to wait for it. In 1.13.0 the nose,
  fin and tail classes derive from `AeroSurface`, not `GenericSurface` (`rocketpy/rocket/aero_surface/`),
  so only rockets built from generic surfaces are touched (**Inference**: none of the seven public
  flights). Unsized.
- **Model gaps, not defects:** no body lift on cylinders and linear nose lift (draft PR #1184, the
  Galejs term; its text says high-angle flight changes and low-angle flight barely does), and no
  lift slope varying with Mach for the nose and tail (#133). Neither report sizes an apogee.

## (c) Fixed since 1.13.0, on `develop` only

- **#1188, the nozzle gyration tensor's parallel-axis term,** a quarter of its value [12]. Merged
  2026-09-09 (`799761b`). Calisto in custom wind: 3,423.27 to 3,423.88 m (+0.018%). Defiance:
  9,242.37 to 9,243.84 m (+0.016%).
- **#1085, a thrust curve starting after t = 0 never left the rail** (#411) [13]. Merged 2026-08-08.
  The delayed case went from 0 m to the on-time case's 10,329 m; ignition at t = 0 is byte-for-byte
  unchanged.
- **#1056, `pressure_ISA` sampled from −2,000 m, not 0 m** [14]. Merged 2026-07-26. About 1e-5
  relative on trajectory quantities; 2.3% in pressure at −1,500 m.
- **#920, an opt-in tip-off phase, `use_udot_rail2=True`** [15]. Merged 2026-09-14 (`ebe1b77`).
  About −2 m on Calisto when turned on; off, bit-for-bit identical.
- **#1121, parachute triggers evaluated once per time node** (#1086). Merged 2026-08-15. No size
  given (**Inference**: none for a trigger that fires at or after apogee).
- **#1138, opt-in unbonded grains.** Merged 2026-08-15. No size given; the default keeps the grain
  center fixed.

What these mean for M2.9:

- **Release state.** None of these is in a release. PR #1194, opened by a maintainer on 2026-09-14 to
  merge `develop` into `master` for 1.14.0 (its head is now `a50d07d`, #1187's merge), says #1187
  and #1190 join it and every other PR waits for 1.15 or later [16]. So 1.14.0 would carry #1188 but
  not #1195, #1196 or #1204.
- **#920's 2 m** is under 0.1% of any Calisto apogee quoted above (**Inference**). It is a new phase,
  off by default, so a cell that keeps the default sees no change.
- **#1085 matters only for a late-igniting motor,** such as an air-started or delayed stage, which
  1.13.0 leaves on the pad. Of the seven public flights' notebooks, four give `burn_time` as one
  number, two read it from a parameter file and Juno III's sets none; only the liquid-motor example
  passes a start time. So the seven ignite at t = 0 (**Inference**).

## (d) Examined and judged not to move an apogee

- **#1193, Earth radius ten times too large** in three geopotential helpers (closed 2026-09-05
  without merging). The PR says the `Environment` paths pass `self.earth_radius` explicitly; 1.13.0
  confirms it at each call (`environment.py:1714`, `:1802`, `:2060`, `:2389`, `:2575`), and
  `pressure_ISA` converts with `earth_radius` in place [5]. The PR names `EnvironmentAnalysis` as a
  caller of the default.
- **Outputs and tools, not the trajectory:** the units table read by `FlightDataImporter` (#1199),
  the fin flutter speed √2 too high (#1200, #1201), the accelerometer's gravity sign (#1175), and the
  potential-energy sign (#1202).
- **Monte Carlo only:** wind factors on an ensemble member (#1160), seeding and sampling fixes.
- **Liquid motors, outside the project's scope:** `LiquidMotor.propellant_I_33` returned 0 (#1191,
  closed as completed; its PR #1192 closed unmerged; not checked further).

## What this suggests

- **M2.9's RocketPy cells, read on 2026-10-18 from 1.13.0,** would name #1195 with its −1.8%, marked
  "only with the `solid_propulsion` equations; the cell's flights use the default". They would list
  #1186 at its own +0.13%, with this project's 1.6% for #1188 and #1196 together beside it, and
  #717, #932 and #1185 as unsized. Expected effect: the cells stop implying that #1195's −1.8% sits in their numbers.
- **ADR-246's example of #1195** reads as if it moved every RocketPy cell. A one-line note in the
  cell, not a new decision, would carry the path condition.
- **Flown with the fixes:** the oracle already flies #1188 and #1196 ([ADR-026][adr-026]). When 1.14.0
  ships, `corrections.py` can drop #1188 and keep #1196, as ADR-026 plans.
- **Terms.** RocketPy is MIT (its `LICENSE`; the PyPI classifier says the same), so its code and
  diffs may be read, run and cached under `refs/` with attribution. Issue and PR text is its authors'
  and is cited here, numbers restated, not copied. The listings and diffs read for this note are
  cached under `refs/research/rocketpy-apogee-defects/`, never committed.

## Open questions

- #1195's size on the seven public flights under `solid_propulsion` is unknown; nothing flies them
  that way, so it matters only if a cell ever does.
- #717 and #932 have no size or sign. #1185's dropped term on solid flights has no size either.
- Whether 1.14.0 ships as PR #1194 describes, and when, is not stated anywhere read.
- HPR Sim's comparisons find a rail-phase difference: RocketPy's rail equation leaves out the
  center-of-mass motion terms ([accuracy](../accuracy.md)). No RocketPy issue reports it, so
  ADR-246's rule would not list it; it moves peak acceleration, and its apogee effect is unsized.

## Sources

All read on 2026-10-18.

1. Open issues and PRs, REST listing: https://api.github.com/repos/RocketPy-Team/RocketPy/issues?state=open
2. Closed since the release, sorted by update: same endpoint with `state=closed&since=2026-07-22`.
   Releases: https://github.com/RocketPy-Team/RocketPy/releases (v1.13.0, 2026-07-22; 43 releases).
3. Changelog on `develop`: https://github.com/RocketPy-Team/RocketPy/blob/develop/CHANGELOG.md; issue
   #1173: https://github.com/RocketPy-Team/RocketPy/issues/1173
4. PR #1195 body, review and diff: https://github.com/RocketPy-Team/RocketPy/pull/1195
5. RocketPy 1.13.0 source, commit `9bd6ad3`, `rocketpy/simulation/flight.py` and
   `rocketpy/environment/environment.py`: https://github.com/RocketPy-Team/RocketPy/tree/v1.13.0;
   `develop`'s `flight.py` at `a50d07d`: https://github.com/RocketPy-Team/RocketPy/tree/develop
6. Issue #1186 and its comment on Defiance: https://github.com/RocketPy-Team/RocketPy/issues/1186
7. PR #1196: https://github.com/RocketPy-Team/RocketPy/pull/1196
8. Issue #1185: https://github.com/RocketPy-Team/RocketPy/issues/1185
9. Issue #1202: https://github.com/RocketPy-Team/RocketPy/issues/1202; PR #1204:
   https://github.com/RocketPy-Team/RocketPy/pull/1204
10. Issue #717 and its comments: https://github.com/RocketPy-Team/RocketPy/issues/717
11. Issue #932 and its comments: https://github.com/RocketPy-Team/RocketPy/issues/932
12. PR #1188: https://github.com/RocketPy-Team/RocketPy/pull/1188
13. PR #1085: https://github.com/RocketPy-Team/RocketPy/pull/1085
14. PR #1056: https://github.com/RocketPy-Team/RocketPy/pull/1056
15. PR #920: https://github.com/RocketPy-Team/RocketPy/pull/920
16. PR #1194: https://github.com/RocketPy-Team/RocketPy/pull/1194

[adr-246]: ../decisions/0246-the-2026-10-12-review-student-launch-by-requirement-and-oracle-defects.md
[adr-026]: ../decisions/0026-the-path-in-wind-rocketpys-corrected-equations.md
