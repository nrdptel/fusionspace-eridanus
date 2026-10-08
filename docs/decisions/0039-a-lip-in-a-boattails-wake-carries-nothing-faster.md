# ADR-039: A lip in a boattail's wake carries nothing faster than sound (2026-09-19)

- **Status:** accepted
- **Summary:** A lip in a boattail's wake carries nothing faster than sound

**Context.** M1.8e7 gave the committed Arcas Robin designs' vertical tip a Newtonian cap, but their
*lip* — a reflexed flare 0.053 in long at the very base, rising from 1.308 in to 1.47 in behind a
15° boattail, about 57° to the axis — still kept the shock-expansion method off the whole body:
the method covers a run only while nothing behind it carries a potential-flow slope of its own
(ADR-034), and slender-body theory gives that lip `2 ΔA/A_ref` = 0.178 per radian at the base.
So both designs flew slender-body theory past Mach 1, 15% to 50% below the tunnel's body alone, and
M1.8a's short model read −16.3% at Mach 2.96 and −28.0% at 4.63.

**Decision.**

- **A lip wholly in a boattail's wake carries no potential-flow slope faster than sound**, so the
  method covers the body to its base. Below the join it keeps slender-body theory's share, and the
  join blends them, so nothing jumps. The shelter is the drag buildup's own measure
  (`crate::drag::WakeTerm`, ADR-030): wholly in the wake to a rise of a quarter of the boattail's
  drop in diameter, not at all from half of it, fading over any tube between. Anything else — a
  flare further out, or further back — still keeps the whole body on slender-body theory.
- **Why nothing, from three readings.**
  - TN D-4014 (p. 6) traces an odd chamber axial force at Mach 1.50 and 1.80, fins off, to the
    reflex lip, and says separation over the boattail at higher Mach numbers, or the thicker
    boundary layer with fins on or on the longer model, "masks" it.
  - Seiff's embedded Newtonian flare method (NASA TN D-1304), which TN D-4865 uses for a flare
    whose shock has detached, holds for "thin shock layers when the flow is not extensively
    separated" (p. 13), and he notes "a 90° ramp will invariably separate the flow" (p. 4). This
    lip's face stands about 57° to the axis behind a 15° expansion.
  - Taken anyway as an upper bound (eq. 9, p. 12; for a conical flare at one dynamic pressure
    `2 (q₁/q∞) cos²θ ΔA/A_ref`, with `q₁` the flow expanded through the boattail's turn), it gives
    0.044 per radian at Mach 1.5 falling to 0.014 at 4.63 — a quarter to a twelfth of slender-body
    theory's 0.178.
- **The moment, checked, and found unable to settle it** (`arcas-robin-lip.json`). For each
  fins-off row, the share at the lip's station that would put hpr's centre of pressure on the
  measured one runs from −0.256 ± 0.068 per radian (short, Mach 1.5) to +0.229 ± 0.084 (long,
  Mach 3.96), changing sign with Mach number and with the model's length. Fitting one share gives
  +0.021 ± 0.019 over all eleven rows (χ² per degree of freedom 4.5), −0.016 ± 0.022 over the short
  model's six (6.4, so its rows disagree among themselves) and +0.108 ± 0.034 over the long
  model's five (0.9, so those five agree — on a share three standard errors above zero and two
  below slender-body theory's). The number blames the lip for every miss in the centre of pressure,
  and the long model's body alone reads 15% to 19% high at those speeds, which moves its centre of
  pressure by more than any lip could; the short model's fit comes out negative, which no flare can
  give. So the moment rejects slender-body theory's 0.178 (8.7 standard errors on the short model,
  2.1 on the long) and cannot separate zero from Seiff's bound. The physics above, not this,
  decides the rule (the physics review asked for the split).
- **"The committed Arcas Robin designs through a flight's path in the report"** means, as for
  M1.8e2, e4, e6 and e7, the committed fixture `arcas-robin-lip.json` and the guide's table pinned
  to it; no validation flight passes Mach 1.2, so the whole-flight report is unchanged.

**Consequences.**

- M1.8a (`normal-force-vs-mach.json`): every row from Mach 1.5 is now within the 15% slope target,
  +8.8% to −3.3% on the short model and +9.4% to −2.3% on the long, where the short read −16.3% at
  Mach 2.96 and −28.0% at 4.63. The short model's body alone, fins off, went from 2.0–2.1 per
  radian to 3.0–4.0 and the long model's from 2.5–2.6 to 3.8–4.4, against the tunnel's 2.2–4.6.
- Five rows left the miss list (the short model at Mach 2.96, 3.96 and 4.63, the long at 3.96 and
  4.63) and two joined it: the long model's centre of pressure at Mach 1.8 and 2.3 now sits 0.53
  and 0.52 calibres forward of the measured, just outside the half-calibre target, because its body
  alone reads 15% to 19% high there — the excess M1.8e6 sized and left (ADR-037), which M1.8e9
  carries.
- With the lip left off entirely the same bodies read within 0.05 percentage points at ten of the
  eleven rows, and 0.5 at Mach 1.5 on the short model, so the rule changes which parts the method
  may cover far more than it changes the lip's own lift.
- Unvalidated, and said so: nothing here measures a lip's lift directly.
- The shelter's threshold is a switch in shape, of issue #87's family, and a large one, since it
  decides whether the whole body flies the method: on the test rocket at Mach 3 and 4°, a lip
  rising 0.2499 of the boattail's drop gives `C_N` 0.2976 and one rising 0.2501 gives 0.1995, a
  third less, with the centre of pressure 1.8 calibres further **forward** — the direction was
  printed backwards here and corrected in ADR-041, which also replaced the rise's threshold with
  a weight, so the two sides now agree to a ten-thousandth. Pinned by
  `a_lip_in_a_boattails_wake_carries_nothing` and noted on the issue.
- A narrowing part behind the run is a boattail the method hasn't covered, not a lip, whatever the
  wake does to its drag: it keeps slender-body theory's share (the code review found this; a second
  boattail drawn with a small step up would otherwise have been zeroed).
