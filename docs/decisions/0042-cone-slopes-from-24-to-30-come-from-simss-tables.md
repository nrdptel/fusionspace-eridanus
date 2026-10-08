# ADR-042: Cone slopes from 24° to 30° come from Sims's tables, where TN 3527's chart stops (2026-09-20)

- **Status:** accepted
- **Summary:** Cone slopes from 24° to 30° come from Sims's tables, where TN 3527's chart stops

**Context.** The second-order shock-expansion method needs each tangent cone's normal-force slope
at `α → 0`. hpr read those from TN 3527's Fig. 2, a chart that stops at a half-angle of 24°, so
any body with a steeper tangent cone was refused outright and kept slender-body theory — one of
the model switches issue #87 tracks, worth −10.4% of the normal force and 1.14 calibres of centre
of pressure on the tests' rocket. A fineness-1 cone is 26.57°, so hpr could not fly one.

**Decision.** Past 24° the slopes come from J. L. Sims, *Tables for Supersonic Flow Around Right
Circular Cones at Small Angle of Attack*, NASA SP-3007 (1964), Table 2 (printed p. 20), at his own
grid of 25°, 27.5° and 30°.

- **It is the same theory, tabulated rather than drawn.** Sims solves Stone's problem with Ferri's
  correction and says his expressions are "identical to those found by Kopal" (p. 7); Fig. 2 plots
  Kopal's tables. Same reference area (the cone's base), same `γ` = 1.4, and his Mach grid
  contains all six of hpr's rows — 3, 4, 5, 6, 8 and 10 — exactly, so nothing is interpolated
  between the two sources.
- **The overlap is the check.** At 22.5°, the steepest angle both cover, this reading of Fig. 2
  and Sims's value differ by 0.0005 to 0.0021 per radian, the largest at Mach 6. The chart is read
  to about ±0.001, so where they disagree most it is the hand reading that is loose, not the
  theory; `sims_and_fig_2_agree_where_they_overlap` pins it.
- **Nothing below 24° moves.** The chart's columns stay as they are, so every number already
  validated against TN 3527's own tables is untouched and no committed fixture changes. Replacing
  them with Sims's throughout is a separate question, and a tempting one now that the overlap is
  measured; it belongs with M1.8e12, which revisits the tables.
- **The edge moves rather than disappearing.** A tangent cone past 30° is still refused, and that
  is still a switch: −7.7% and 0.81 calibres on the tests' rocket, against −10.4% and 1.14 at the
  old 24°. Sims's tables stop there.

**Consequences.**

- A fineness-1 cone flies the method, as does any nose whose tangent cones stay under 30°.
- One of issue #87's switches moves from 24° to 30° and shrinks; the guide's table records both
  the new size and the old.
- A blunt tip's handover is still capped at 24° ([`blunt_tip::MAX_HANDOVER_RAD`]) even though the
  slopes now reach 30°. Raising it changes what every committed blunt nose flies, so it is
  M1.8e12's decision, not a side effect of this one.
