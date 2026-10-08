# ADR-026: The path in wind: RocketPy's corrected equations, and hpr's body lift (2026-09-18)

- **Status:** accepted; Prometheus's drifts superseded by ADR-027
- **Summary:** The path in wind: RocketPy's corrected equations, and hpr's body lift

**Context.** In wind, hpr's whole flights turned into the wind less than RocketPy 1.13.0's
(issue #50). Juno III's apogee drift was −60.8% of RocketPy's (its landing drift +151%), NDRT
2020's −18.6%, Bella Lui's −15.6% and Calisto's −5.6%. In still air, Valetudo's landing drift was −3.4%, and in calm air
Juno III's drifts were −3.7% (ADR-025). Same-drag mode shares only `C_D0`, so M2.1d3 looked at
everything else that turns a rocket in wind: the rail release, the drag's growth with angle of
attack, the normal force and the damping.

**How it was found.**

- *One candidate at a time.* In a local build with switches (not committed), hpr was flown with
  each candidate matched to RocketPy. Juno III's apogee drift went from −60.8% to −44.1% with body
  lift off, −58.3% with the rail release at the first button, −60.5% with a linear normal force and
  −60.9% with no angle-of-attack drag factor. With all four matched, and Juno III's fin slope
  raised 7.6% to RocketPy's, it was still −31.9%. Something else was at work.
- *The same state in both codes.* RocketPy's states along Juno III's windy flight were fed to
  hpr's equations of motion (a local probe of `Simulation`'s derivative). With hpr's normal force
  made linear like RocketPy's, the two agreed on the lateral acceleration to 1%. But at the rail
  exit, where the rotation rate is zero and no damping of any kind acts, RocketPy's angular
  acceleration was 1.75 times hpr's (0.3608 against 0.2066 rad/s²). After burnout the two agreed.
  With no rotation, both codes' rotational equations reduce to the moment about the centre of mass
  over the inertia there. So they disagreed about where the centre of mass was.

**The cause, in RocketPy.** `Flight.u_dot_generalized`, RocketPy's default 6-DOF equations, is
written about the centre of dry mass (CDM) and needs the vectors *from* it to the centre of mass
(`r_CM`) and to the nozzle exit (`r_NOZ`). It reads `Rocket.com_to_cdm_function` and
`Rocket.nozzle_to_cdm`, which point *to* the CDM: the note in `rocket.py:996-1025` says
`com_to_cdm_function + center_of_mass == center_of_dry_mass_position`. So while the motor burns,
RocketPy takes its moments about a point as far forward of the CDM as the true centre of mass is
behind it. For Juno III at the rail exit that point is 1.315 m from the nose tip, where
RocketPy's own `center_of_mass` puts the centre of mass at 1.639 m. The margin RocketPy flies there
is 1.75 times the one its own `static_margin` reports. After burnout `r_CM` is zero and the
error ends. A rocket that is too stable during the burn turns into the wind too much.

Upstream, an outside contributor reported the sign in issue #1186 (2026-08-25) and proposed the
fix in PR #1196 (open, not yet reviewed, head `927e771e`): three edits that negate `r_CM` with its
derivatives, negate `r_NOZ`, and flip the `r_CM ^ w_dot` term in `v_dot`, which was written for
the reversed vector. PR #1196 builds on PR #1188, merged into `develop` 2026-09-09 and not yet
released, which corrects the nozzle gyration tensor's parallel-axis term from
`0.25 * nozzle_to_cdm**2` to `nozzle_to_cdm**2`, so that the jet damping uses the whole lever.
RocketPy's own `develop` branch agrees on the convention: the tip-off phase merged there in PR #920
(2026-09-14) says, at `flight.py:2086-2087`, "The generalized EOM store r_CM / r_NOZ as (point ->
CDM) vectors, i.e. the negative of the true-frame position; hence the sign flips below", and
negates `com_to_cdm_function` for its own use.

Two checks:

- *RocketPy against itself* (`validation/oracles/rocketpy/wind_response.py` prints it for every
  case). At the rail exit of each windy case, RocketPy's angular acceleration as released equals
  the moment about the mirrored point over the inertia, to four digits. For Juno III it is 0.3608
  rad/s²; about RocketPy's own centre of mass the same moment gives 0.2067.
- *Corrected RocketPy against hpr* (the local probe). With both corrections, at RocketPy's states
  through Juno III's burn with the rocket rotating, its angular acceleration agrees with hpr's
  (normal force linear) to 0.1 to 5%, the larger where the value is small after burnout, and its
  CDM acceleration to 0.01 m/s². So hpr's variable-mass terms and jet damping match RocketPy's
  corrected ones.

**What remains is the two codes' models.** With the corrections, RocketPy's drifts in wind move
by up to 78% (Juno III's landing; 87% on its own drag). hpr against them: Juno III −42.5% (apogee drift) and +40.9%
(landing drift), Bella Lui −11.3% and −23.8%, NDRT 2020 −4.65% and +1.95%, Calisto −0.99% and
+1.43%, Valetudo −0.93% and −1.95%. In calm air every drift agrees within 2.2%. `wind_response.py`
then adds hpr's choices to the corrected RocketPy one at a time. The drifts, in metres:

| case, drift | RocketPy as released | corrected | + release at the last button | + body lift | + thin fins | hpr |
|---|---|---|---|---|---|---|
| Juno III, apogee | 582.4 | 396.6 | 360.7 | 270.6 | 231.1 | 228.0 |
| Juno III, landing | 268.3 | 478.5 | 519.6 | 624.0 | 670.2 | 674.4 |
| Bella Lui, apogee | 123.9 | 117.9 | 110.9 | 104.7 | | 104.6 |
| Bella Lui, landing | 74.6 | 67.2 | 58.9 | 51.7 | | 51.2 |
| NDRT 2020, apogee | 69.6 | 59.4 | 58.1 | 57.1 | | 56.6 |
| NDRT 2020, landing | 341.0 | 347.5 | 348.3 | 354.4 | | 354.2 |
| Calisto, apogee | 447.1 | 426.4 | 424.3 | 422.2 | | 422.2 |
| Calisto, landing | 1195.5 | 1261.5 | 1265.7 | 1278.3 | | 1279.6 |
| Valetudo, apogee | 167.7 | 165.2 | 163.6 | 163.6 | | 163.7 |
| Valetudo, landing | 206.4 | 203.3 | 201.4 | 199.5 | | 199.4 |

With hpr's three choices, RocketPy lands within 1.4% of hpr in every windy case. The local build
agrees from the other side: hpr with its normal force linear, no body lift, the first-button
release, no drag factor and Juno III's fin slope matched is within 0.7% of the corrected RocketPy
in every windy case (Juno III 396.4 against 396.6 m). The three choices:

- **Body lift** (ADR-008): `C_N = K (A_plan/A_ref) sin² α`, `K = 1.1` (Galejs), acting at each
  body component's planform centroid. It is zero at small angles, but a slow rocket leaves the rail
  at a large one: Juno III at 18 m/s in an 8.5 m/s wind, its rail leaning 5° downwind, 26° off the
  airflow, where body lift is about half its normal force. Much of it acts ahead of the loaded
  rocket's centre of mass, the nose's above all (its planform centroid is 0.35 m from the tip, the
  centre of mass 1.64 m), so at a steep angle it moves the centre of pressure forward by about
  0.3 m and weakens the moment that turns the rocket into the wind. It turns the rocket more than
  it pushes it: in RocketPy with hpr's release and fins, Juno III's apogee drift is 328.0 m with no
  body lift and 231.1 m with it, 232.6 m with the nose's alone, and 311.2 m with all of it placed
  at the centre of mass, where it can only push (Bella Lui: 110.9, 104.7, 108.2 and 110.0 m).
  RocketPy's normal force is linear in `α` and has no body term.
- **The rail release** (ADR-025): hpr guides the rocket until its last button leaves the rail;
  RocketPy frees it at the first. Neither models tip-off, the pivot about the last button between
  the two, so the real release lies between them: each is a modelling choice.
- **Juno III's fins:** the example gives them an airfoil lift curve, which RocketPy uses in place
  of the thin-plate `2π`, and which hpr does not model; RocketPy's fin slope is 7.6% steeper.

The drag's growth with the angle of attack (hpr's, up to 1.3 times at 17°) moves no drift by more
than 0.1%, and hpr's `sin α` in place of `α` moves none in wind by more than 0.9% (local
build).

**Decision.**

- **The oracle flies RocketPy 1.13.0 with PRs #1188 and #1196 applied** (`corrections.py`). The
  corrected `u_dot_generalized` is RocketPy's own source recompiled with #1196's three edits, each
  required to match exactly once, so a RocketPy that has moved stops the script instead of flying
  something else. The fixtures record both corrections (`corrections`), and their `model` names
  them. Both whole-flight references were regenerated; the mass and descent fixtures are
  bit-identical. RocketPy's apogees move by at most 1.6% (Prometheus 2022; Juno III +1.0%).
- **A drift is gated at 3% unless a measurement shows why it misses.** Six drifts reported before
  are now gated, and pass: Calisto's two in wind, Valetudo's and NDRT 2020's landing drifts, and
  Juno III's two in calm air (superseding ADR-025's decision on them). Five stay reported but not
  scored, each as a measured model difference: Juno III's and Bella Lui's two, and NDRT 2020's
  apogee drift. Prometheus 2022's drifts, excused before, are held to 3% for when hpr flies it.
- **hpr keeps its body lift and its rail release.** Body lift is a real force at these angles, and
  OpenRocket carries it too. The release at the last button is hpr's modelling choice (ADR-025);
  with no tip-off in either code, neither end is the real one.
- **Drop the corrections when RocketPy releases them.** Then re-pin the oracle and delete
  `corrections.py`, or keep only what the release still lacks.

**Alternatives rejected.**

- *Keeping RocketPy as released and the drifts unscored.* The reference would carry an error in
  its equations of motion that its own `static_margin` contradicts, that RocketPy's `develop`
  branch describes in a comment (PR #920), and that PR #1188 (merged) and PR #1196 (open)
  correct. hpr's correct answer would read as a miss:
  Juno III's apogee, +1.71% as released, is +0.70% corrected.
- *Only #1196.* It is written on top of #1188, which is merged upstream.
- *RocketPy's `develop` branch with #1196.* Unreleased, and it would move every reference for
  reasons unrelated to this one.
- *Gating the five drifts now against RocketPy flying hpr's body lift, release and fin slope.* It
  would check hpr's equations at steep angles as same-drag mode checks them with the drag shared,
  but not hpr's normal force against another's, and it needs a reference whose rail differs from
  the one hpr flies, which the harness does not read yet. Until then the five are pinned as the
  committed report pins every number (`validate --check` fails if one moves) and the regeneration
  script prints `wind_response.py` beside them. Filed as issue #61.
- *Dropping or shrinking hpr's body lift to agree.* That fits hpr to a code that leaves a real
  force out. `K` is uncertain (Galejs gives 1.0 to 1.5) but it is not zero.
- *Loosening the drifts' gate.* That would widen a gate to fit a result, which the hard rules
  forbid.

**Consequences.**

- Issue #50 closes. M2.1's landing offset is met in calm air, in still air (Valetudo), for
  Calisto in wind and for NDRT 2020's landing. It is not met for Juno III and Bella Lui in wind,
  where the two codes' models differ: body lift and the rail release by design, Juno III's airfoil
  fins a feature hpr lacks. The report shows both numbers.
- Whether `K` should be lower at these rockets' crossflow Reynolds numbers is open: crossflow
  methods other than Galejs's (Allen and Perkins; Jorgensen, NASA TR R-474) are not yet read.
- In wind, a slow rocket's drift in hpr depends on body lift's uncertain `K`. Flown in RocketPy
  with hpr's body lift, rail release and fin slope (`wind_response.py`), Juno III's apogee drift is
  240.2 m at `K = 1.0`, 231.1 m at 1.1 and 194.1 m at 1.5, and 328.0 m with no body lift; its
  landing drift is 659.5, 670.2 and 714.2 m. hpr itself gives 237, 228, 191 and 326 m (local
  build). Calisto, off the rail at 28 m/s and 11°, changes its drifts by under 0.5% across
  `K = 1.0` to 1.5. Which code is nearer a real flight is for M2.3.
- The corrections do not only move RocketPy toward hpr: Valetudo's apogee difference grew from
  +0.003% to +0.116%, and predicted NDRT 2020's apogee drift from −4.44% to +11.906% (outside its
  target before and after).
- The time-series RMS: Juno III's height RMS is 15.9 m, from 39.2, and Bella Lui's 1.9 m, from
  2.3; four others grow by under a metre (Valetudo 1.4 to 2.4 m, NDRT 2020 2.0 to 2.8 m, and
  Juno III's and Calisto's calm cases 1.3 and 1.1 to 2.1 and 2.0 m). Burnout speed now reads
  +0.018% to +0.063% in every case, where it read −0.066% to +0.040%: a small rest of one sign,
  far inside the gate.
- Predicted mode: 66 of 85 metrics within target, from 63; Calisto's drifts and Juno III's apogee
  are now within, and nothing new misses.
- `rail_release.py` now flies the corrected equations. The numbers ADR-025 quotes from it were
  measured before this correction; `wind_response.py` gives the current ones.
