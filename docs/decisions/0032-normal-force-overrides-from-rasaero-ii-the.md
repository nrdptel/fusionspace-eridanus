# ADR-032: Normal-force overrides from RASAero II: the static force replaced, hpr's damping kept (2026-09-19)

- **Status:** accepted
- **Summary:** Normal-force overrides from RASAero II: the static force replaced, hpr's damping kept

**Context.** M1.8d's done-when (`ROADMAP.md`): "a RASAero II export's `C_Nα` and CP columns
replace hpr's in a flight, and the reading is tested on the Calisto export"; M1.8's bullet asks for
the M1.5 override tables extended to `C_Nα` and CP against Mach and angle of attack, importable
from RASAero CSV. A flight takes its pitch and yaw damping from each component in its own local
flow (ADR-011), so a whole-rocket normal force can't simply stand in for the components. RASAero
II's aerodynamic export (its Aero Plots screen, File, Export, To CSV File: RASAero II Users Manual,
2019, p. 76) gives, per Mach number and angle of attack, `CN`, its `CN Potential` and `CN Viscous`
parts, `CP` and two columns that repeat the 4° values, and no damping. The manual takes every
dimension in inches (p. 13), measures the CP from the nose ("distance measured from the nose",
p. 114), puts the coefficients on the largest cross-section of the body (p. 72), and takes the
viscous part from Jorgensen's crossflow method (p. 55).

**Decision.**

- **The table** (`hpr_aero::NormalForceTable`): one column per angle of attack in `[0°, 90°)`, each
  `C_N/α` and CP against Mach number. A lookup reads each column at the Mach number (linear,
  holding its ends and saying so) and interpolates linearly in `α` between columns, so `C_N` comes
  back exactly at the columns' angles and is a part linear in `α` plus one in `α²` between them.
  In the Calisto export the viscous part starts at Mach 0.91, and `CN Viscous(4°)/CN Viscous(2°)`
  is (sin 4°/sin 2°)² = 3.995 from there through Mach 1.3 (exactly `sin² α`), then 3.90 at 1.5,
  3.16 at 2, 1.73 at 3 and 1.05 at 4: the quadratic is RASAero II's shape through Mach 1.3 and an
  assumption faster. Below the first column the table holds that column.
- **Past the last column** `α_n`, the force at `α_n` splits: the 0° column's slope times `α_n`
  is the linear share, at the 0° CP, growing as `sin α / sin α_n` (hpr's fins, ADR-011); the rest
  of the force and of the moment grows as `(sin α / sin α_n)²`, the crossflow form hpr's body
  lift takes (Galejs; Niskanen eq. 3.26) and RASAero II's viscous part takes from Jorgensen.
  Written as `C_N(α_n) s + R (s² − s)` with `s = sin α / sin α_n` and `R` the rest, the extra
  term vanishes at `α_n` whatever the rest's station, so the station is held within the rocket
  (`lookup_within`, which `AeroModel` calls with nose tip to aft end) and the linear share within
  the force (a falling `C_N/α` leaves no rest), with no jump anywhere: force and CP are continuous
  at `α_n` and in the table's values, the force is never negative, and tail first it is zero. A
  first draft scaled the whole force by `sin α` with the CP held; physics review showed that
  loses the viscous part's faster growth and its forward CP (on the guide's invented example, 18%
  less force at 10° with the CP 5.3 cm aft). The second review found the unguarded split turned
  the force round for a falling `C_N/α`; the third, that a guard switching the split on and off
  jumped (36% in `C_N`) as the Mach number moved the rest through zero, and left the rest's CP
  unbounded aft. Proptests now hold the sign, the CP within the stations while `s ≥ 1`, and the
  continuity in Mach.
  From Mach 3 RASAero II's viscous part hardly grows between 2° and 4°, so there the `sin² α`
  share probably overstates the force. All of this is an assumption past the data; the lookup
  reports it (`beyond_alpha`), and `NormalForce::table` carries the lookup out of the model.
- **Reading RASAero II** (`from_rasaero_csv`): the columns `Mach`, `Alpha`, `CN`, `CN Potential`,
  `CP`, the only ones that must be numbers. At `α > 0` the slope is `CN/α`. At 0° `CN` is zero,
  so the slope is `CN Potential(α₁)/α₁` at the smallest positive angle and the same Mach number:
  the potential part is linear in `α` (the Calisto export's spread between 2° and 4° is 2.3e-15),
  and through Mach 1.3 the viscous part is `sin² α`, which has no slope at zero; faster, how it
  starts from 0° isn't in the export, and leaving it out is an assumption. Not
  `CNalpha (0 to 4 deg)`, the 4° secant with the viscous part in it (ADR-027). `CP` is converted at 0.0254 m to
  the inch from the nose tip, the datum of hpr's stations. The table's reference is
  `TableReference::LargestBody`, which `AeroModel` resolves from the largest body radius, so a
  design whose reference is its nose base still gets RASAero II's area. Angles need not share
  their Mach numbers (the Calisto export's 4° rows end at Mach 24.99, the others at 25); rows out
  of order within an angle, or an angle with one row, are refused with their line.
- **A units guard**: `AeroModel::with_normal_force_table` and `Simulation::with_normal_force_table`
  refuse a table with a CP value outside the rocket, nose tip to aft end, at its Mach numbers a
  flight can reach (to Mach 5 and the first knot past it), as `SolidMotor` refuses an impossible
  exhaust speed (#11). It checks the table's values, not what a user-built table's own
  interpolation might give between or past them. Both return `Result` where `with_drag_table` doesn't: a check at
  attachment names the problem before a flight starts. `with_reference_diameter_m` checks its
  diameter when given, for the same reason.
- **In flight** (`Simulation::with_normal_force_table`): the table's normal force at the centre
  of mass's airflow acts at its CP; each component adds its force in its own local flow less its
  force in the centre of mass's. In linear theory that difference is `q̄A C_Nα,i θ̇ ℓᵢ/V`, so the
  damping moment stays hpr's `q̄A Σ C_Nα,i ℓᵢ² θ̇/V` and the path term `q̄A Σ C_Nα,i ℓᵢ`. With no
  rotation the two component terms are the same number and cancel exactly. The table has no side
  force (an axisymmetric code). Mach 5 and faster is still refused, since the damping needs hpr's
  components; `AeroModel::normal_force` alone takes any Mach number with a table, as a drag table
  does. A flight on a table evaluates each component twice; flights without one are unchanged,
  bit for bit (the validation report doesn't move).
- **Tested on the Calisto export without committing more of it.** `cargo xtask aero` reads the
  pinned export with the library's reader and writes `normal-force-override.json`: each column's
  count and Mach range; every row at a positive angle read again with a plain split and compared
  with the table; a hash of the table; the viscous part's growth; the 0° column at the 15 Mach
  numbers M1.8a compared (the 30 values ADR-027 already commits); and Calisto flown four ways.
  CI has no `refs/`: there a test checks that the fixture's 30 values agree with those M1.8a's
  own parser committed (the same 0° rule, so it checks the reading, not the rule) and flies the
  15-point table again. Reading the export itself needs it: `cargo xtask aero --check`, which
  `aero::tests` runs where `refs/` is present.

**Result.**

- *The reading:* 0°, 2° and 4° columns of 2,500, 2,500 and 2,499 Mach numbers, Mach 0.01 to 25;
  the 4,999 rows at a positive angle come back with `CN` within 2.2e-16 relative and `CP` exact;
  the 30 values equal M1.8a's to 1e-12.
- *Linear theory* (`hpr_sim::tests::pitch_oscillation_follows_a_normal_force_table`): Valetudo at
  100 m/s with a table of 1.5 times hpr's slope and its CP 5 cm aft oscillates in pitch and in yaw
  with a period of 1.1040774 s against the theory's 1.1040734 s (1.44965 s on hpr's own; bound
  3e-5), and decays within 0.03% of the rate hpr's damping gives (bound 1%). The table's `K₁` in
  the path would predict 1.1044079 s and lumped damping a decay of −0.138 against −0.244: the
  test tells them apart.
- *Tables of hpr's own normal force* fly Valetudo in a crosswind to within 7.8 mm of hpr's own
  apogee (every 0.5° and Mach 0.01; bound 5 cm) and 5.5 cm (0°, 2° and 4°, past which the
  continuation flies; bound 10 cm); at 0°, 1°, 2°, 4°, 8°, 16°, 30°, 60° and 89°, every Mach
  0.05, 1.18 m, the interpolation's. A RASAero-shaped table continues its `sin² α` part to 1e-12.
  Calisto's export has no viscous part below Mach 0.91, so its flights (to Mach 0.75) grow no rest;
  the tables of hpr's own normal force, whose body lift is a rest, are the flights that do.
  A table on RASAero II's reference is rescaled by 4 on a rocket whose reference is half its
  largest body.
- *Calisto* from a 5.2 m rail at 85° in 5 m/s of crosswind, peaking at Mach 0.746: apogee
  2,793.11 m on the export's normal force against 2,794.39 m on hpr's own, 14.2 m further upwind.
  hpr's own as a table at the export's angles and Mach numbers moves it 0.03 m (the method
  alone). Each flight spends about 2.2 s past 4°: 0.3 s after the rail (up to 7.9°) and the last
  1.9 s before apogee. The 15-point table's apogee is within 0.006 m of the whole export's, and
  its position within 0.07 m.

**Alternatives considered.**

- *The table's force alone, at its CP:* the damping becomes `C_Nα (X_cp − x_cg)²`, smaller than
  the components' `Σ C_Nα,i ℓᵢ²` by their spread about the CP (the parallel-axis theorem), so the
  rocket would oscillate longer.
- *hpr's components scaled to the table:* matching both the slope and the CP takes two scale
  factors, and no split between body and fins is unique.
- *The 4° columns `CNalpha (0 to 4 deg)` and `CP (0 to 4 deg)` as a table in Mach only:*
  simpler, but every angle would carry the 4° viscous part.
- *The viscous share growing as `sin α` past the last angle from Mach 3,* where the export's own
  growth slows: a Mach-dependent rule fitted to one code's trend; recorded as a limit instead.
- *A warning when the table's CP at low Mach differs from hpr's by more than a caliber* (physics
  review): hpr has no warning channel on a flight yet; the guard refuses only a CP outside the
  rocket.

**Consequences.**

- A flight can fly RASAero II's normal force; with a drag table too, both of its main forces. The
  2018 Calisto export flies subsonic only (plain Barrowman there: no viscous part), so no real
  export has been flown through Mach 1; the transonic and supersonic columns are tested by the
  reader's unit tests and the re-read of every row.
- Open: the continuation past the last column is unmeasured; a design whose nose tip isn't
  RASAero II's reads a shifted CP with no warning unless it leaves the rocket. The
  maintainer's question about RASAero values in fixtures is unchanged: no new values are committed
  (the fixture adds counts, differences, ratios, a hash and flights).
