# ADR-243: The stability guide: the center of pressure against a wind tunnel, and the guide rocket's own report (2026-10-11)

- **Status:** accepted; carries out [ADR-195](0195-the-2026-10-07-guides-rocketry-explained.md) §6 and §7 for its stability guide, on [ADR-242](0242-the-guides-format-and-the-trust-guide.md)'s format; closes M0.7a3
- **Summary:** The second guide, *Stability in flight*, explains how Mach number and angle of attack move the center of pressure. Its numbers come from two committed files: the existing comparison of HPR Sim's center of pressure with NASA's Arcas Robin wind-tunnel tests, and a new report of the guide rocket's center of pressure through Mach number and angle of attack, which `cargo xtask guides` writes from the design and fails when stale. The figure draws the tunnel's readings, each with its uncertainty as a band, against HPR Sim's line, and marks in the caution color each speed where HPR Sim's center of pressure sits aft of the tunnel's: a margin that reads high, tied to a new test of that sign. The angle section marks no side, since nothing measured checks the whole rocket at an angle.

**Context.** M0.7a3's *done when* is M0.7a2's: the guide meets the format, ADR-195 §7 and M0.7a's
terms on numbers and marks, is read cold in a docs review, and passes M0.7a1's checks. ADR-242
§3 lets a guide read only committed reports, and its consequences say a guide whose numbers come
from another source adds that source to the reader first. No committed file held a hobby rocket's
center of pressure through Mach number or at an angle, and no test pinned which way an error in
the center of pressure errs.

**Decision.**

1. **Two sources join the reader.** `validation/fixtures/aero/normal-force-vs-mach.json`, which
   `cargo xtask aero` writes and `hpr-aero`'s `normal_force_against_mach` pins, and a new
   `validation/reports/guide-rocket-cp.json`: the guide rocket's (`level-1.ork`) center of
   pressure straight into the wind at 11 Mach numbers from 0 to 3, and at 5 angles of attack from
   0° to 15° at Mach 0.3, from `hpr_aero::AeroModel::normal_force`, with the body's lift at each
   angle. `cargo xtask guides` writes it; `--check` and the guides' test fail a committed copy
   whose numbers sit more than 1e-9 from what the design gives now, so the last bits that move
   between platforms' maths libraries don't fail CI and a real change does.
2. **The figure is the measurement, not the model alone.** It draws the short Arcas Robin
   model's 12 tunnel readings (Mach 0.6 to 4.63), each with the reading's stated uncertainty as a
   band, against HPR Sim's line, and shades the transonic band where HPR Sim joins its fin
   methods with a straight line. A model's curve with nothing to check it against would show
   what HPR Sim computes, not how far to trust it.
3. **The flattering mark.** A center of pressure aft of the true one makes the margin read high.
   The figure marks each speed with a positive `cp_error_calibers` in the caution color; the new
   `aero_mach` test `a_cp_aft_of_the_reference_reads_a_higher_margin` pins that a CP aft of the
   reference's is a positive error and a margin higher by that much at any CG, and the guide's
   `the_flattering_marks_are_the_speeds_aft_of_the_tunnel` that the marks are exactly those
   speeds. The guide names the first.
4. **The worst case is quoted from both models.** The trust note quotes the largest error aft over
   both of the tunnel's models (2.36 calibers on the longer one), not the drawn model's alone, so
   the page doesn't state a smaller error than the comparison holds.
5. **The angle section marks no side.** HPR Sim's body lift is checked against bodies without
   fins, but nothing measured checks the whole rocket at a larger angle, so the guide says so and
   draws no mark. Its numbers show the model's behavior on the guide rocket.
6. **The mutation probe also shortens lists.** A count of rows is an array's length, which moving
   a value can't change; the probe now also drops a list's last item, so such a count is probed
   too.

**Alternatives considered.**

- **Draw the guide rocket's center of pressure by Mach number.** Rejected as the figure: it is
  HPR Sim's model alone. The guide quotes it in words instead.
- **Quote the short model's worst error (1.34 calibers) in the trust note.** Rejected: the
  comparison holds a larger one on the longer model, and the smaller would flatter.
- **Compare the report's text exactly.** Rejected: aerodynamics through `sin`, `atan` and `sqrt`
  can differ in the last bit between platforms, which would fail CI with no change.

**Consequences.**

- A change to `hpr-aero` that moves the guide rocket's center of pressure fails
  `cargo test -p xtask` until `cargo xtask guides` rewrites the report and the guide follows.
- A regenerated wind-tunnel comparison that moves a number fails the guide the same way.
