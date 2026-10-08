# ADR-181: The stability issue warnings (2026-10-06)

- **Status:** accepted
- **Summary:** M1.14a3's four stability warnings. #87, #120 and #121 warn past Mach 1.2 when `hpr_aero`'s supersonic body run, which is otherwise silent about why it stops, names that switch as the one that stopped it: a new public `SupersonicFallback` on `AeroModel`. #172 states no condition, and its cause is unmeasured, so it warns on every flight on hpr's own aerodynamics that reports a smallest static margin, quoting the largest measured gap (0.073 calibres high) and no threshold.

**Context.** [ADR-180](0180-the-drag-issue-warnings.md) split M1.14a2 and left the four stability
issues on [ADR-163](0163-the-second-pass-products-and-priorities.md) §4's list to M1.14a3. A
margin or centre of pressure that reads high is the flattering side ([ADR-162](0162-the-suite-and-its-releases.md)
§2), so each must warn, by number, when a flight or design meets its condition.

Three of them are switches inside the supersonic body method. Each makes the whole body fall back
from the second-order shock-expansion method to slender-body theory, which overstates the normal
force's lever arm (CP aft):

| issue | where the run stops (`crates/hpr-aero/src`) | the predicate |
|---|---|---|
| [#87](https://github.com/nrdptel/hpr-sim/issues/87) | `model.rs`, the run's segment arms (tube, boattail, conical flare), and `ShockExpansionBody::new` as a backstop | a step in area larger than a millionth of the component's fore area |
| [#120](https://github.com/nrdptel/hpr-sim/issues/120) | `model.rs`, the sheltered lips' `take_while` | a lip longer than its boattail's drop in diameter |
| [#121](https://github.com/nrdptel/hpr-sim/issues/121) | `shock_expansion.rs`, `cone_normal_force_slope`, called at the vertex | a tip cone steeper than the cone tables' 30° (NASA SP-3007, `blunt_tip::CONE_TABLE_CAP_RAD`) |

Each turns the method's failure into `None` and drops the reason. The only public query is
`AeroModel::supersonic_body()`, which says whether the run covers the body, not why it doesn't.
The table is built only past `SUPERSONIC_JOIN_START_MACH` (Mach 1.2); below it no switch fires.

The fourth, [#172](https://github.com/nrdptel/hpr-sim/issues/172), is a measurement on two
private designs: the margin at rod clearance reads 0.035 to 0.073 calibres higher than
OpenRocket 24.12's on every flight of both, from the centre of pressure on one and from both the
centre of pressure and the centre of mass on the other. Its cause is not known, so it states no
shape or Mach condition.

**Decision.**

1. **`hpr_aero` says why the run stopped.** A public, `#[non_exhaustive]`
   `SupersonicFallback` enum, with a variant for each of the three switches (`RadiusStep`,
   `LongLip`, `SteepTip`), each naming the component's id, and `Other` for any rule not on the
   list. `AeroModel::supersonic_fallback()` returns it when `supersonic_body()` is `None` and the
   body has a normal force to lose. The first switch along the body is reported; the run stops
   there. The predicates stay as they are: this is a report, not a model change, and a test pins
   that the model's numbers don't move.
2. **#87, #120, #121** warn past Mach 1.2, exclusive, as every Mach edge in
   `hpr_sim::issues` is, when the flight's design reports that switch. The warning names the
   component and says the margin and centre of pressure read high (CP aft) from Mach 1.2; a NaN
   top Mach number raises it whenever the switch is reported.
3. **#172 warns on every flight** flown on hpr's own aerodynamics that reports a smallest static
   margin. The message gives the bound, "may read up to 0.073 calibres high, cause unknown", and
   the issue's address. There is no margin threshold: a threshold would be a verdict, which the
   RSO and the safety code own, and the measured gap says nothing about which margins it applies
   to. #68 is the precedent for a warning every rocket raises. When the part-by-part comparison
   finds a cause, this condition narrows to it in a new ADR.
4. **Where they show:** as ADR-180 §6, the same `IssueWarning` list, `hpr sim`'s `issues` and a
   `warning: stability:` line each, Python's `Flight.issue_warnings`. A flight on a drag of its
   own keeps its stability warnings: those come from the normal force, which it still takes from
   hpr. A model given a normal-force table of its own (`AeroModel::with_normal_force_table`)
   raises none of the four, as its normal force isn't hpr's.
5. **The examples change.** #172 shows on every flight, so the README's and the guide pages'
   regenerated `hpr sim` outputs (`docs/cli.md`, `docs/fly-your-ork.md`, `docs/pick-a-motor.md`,
   `docs/stability-for-certification.md`) gain its line, in the same PR.

**Alternatives considered.**

- *Read the reason from `AeroError::Unsupported`'s text.* Free text is not a contract; a reworded
  message would silently stop a safety warning.
- *Re-test the three predicates in `hpr_sim`.* Two copies of a rule drift; the report has to come
  from the code that switches.
- *#172 only below a margin, such as one calibre plus the gap.* A rule of thumb is not a measured
  limit, and choosing one would be a verdict; the gap's cause and reach are unmeasured.
- *#172 only on designs like the two private ones.* What they share that others lack is not known,
  and naming their features in public would describe private designs.

**Consequences.**

- A step the run's gate (a millionth of the area) lets through but the march refuses (a billionth
  of the radius) fails the table, not the run; `supersonic_fallback` names it `RadiusStep` too, so
  #87's edge is the march's, 2.7e-11 m on the tests' rocket.
- A staged flight asks the whole stack's model, its upper stage at the front, with the whole
  flight's top Mach number: a switch in the booster can warn though the booster never passed
  Mach 1.2. Whether it can also miss an upper stage's switch is not yet checked; asking each
  stage's own model is left for later (a recorded gap).

- Every flight on hpr's aerodynamics raises #172 until its cause is found; the text and JSON
  outputs gain a line each. This is the cost of a flattering error with no known condition.
- `hpr_aero` gains a public enum and accessor; the switch sites record their reason instead of
  dropping it.
