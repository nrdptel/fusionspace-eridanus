# ADR-048: What a marched flare is worth, measured against TN D-4865's model 2 (2026-09-20)

- **Status:** accepted
- **Summary:** What a marched flare is worth, measured against TN D-4865's model 2

**Status:** accepted. **Milestone:** M1.8e18, the last of the three the old M1.8e14 split into.

**Context.** ADR-047 gave a flared body the second-order shock-expansion method and compared it
with nothing measured: the guide had to say, in its own opening paragraph, that no measured flare
force had been compared with any of it. M1.8e18 asks for TN D-4865 model 2's readings, committed
with their provenance, and for the guide to say what a marched flare is worth and what it leaves
out.

Model 2 is the only flared body in this project's sources whose **normal force and pitching
moment** are printed rather than only its pressures: a blunt 2.75° cone with an 18.5° flare, in the
Langley Unitary Plan tunnel at Mach 1.50, 1.90, 2.30, 2.96, 3.95 and 4.63, with fig. 8(b) plotting
`C_N`, `C_m` and `C_A` against `α` from 0° to 12°. It is the same report, the same figure and the
same tunnel as model 1, whose readings M1.8e7 already committed for the blunt tip's cap — so the
two can be read, fitted and compared the same way, and the difference between them is as close as
this source gets to "with a flare and without".

**Decision.**

- **The readings are committed as `tn-d-4865-flared-cone.json`**, read from fig. 8(b) by the pixel
  pipeline M1.8e7 wrote for fig. 8(a), with the grid fits' residuals, the per-circle uncertainty
  and the report's own statements about the flow stored beside them. `C_N` and `C_m` are read;
  `C_A` is not, because hpr's supersonic drag is a separate model this milestone does not touch.
  The `α` = 0 circles read −0.0039 to +0.0043 where they should read 0, which is the plotting's own
  accuracy.

- **The drawing is closed on the base, not on the printed lengths.** Fig. 3(b)'s printed
  dimensions are mutually inconsistent at the 1% level: the nose derived from its three radii is
  0.3429960 diameters long, 0.3429960 + 0.743 + 0.523 = 1.6090 is the printed length exactly, but
  the two printed half-angles then carry the base to 1.0084 diameters rather than 1.000. hpr keeps
  the nose, both half-angles, the length and the base — every quantity the measured coefficients
  are divided by, and every angle the flow turns through — and solves for the split of the rest.
  The alternative, keeping the printed lengths and scaling the body to a 1.000 base, is computed
  too and published beside it, and so is a third that keeps the printed lengths **and** the base
  and gives up the flare's stated half-angle instead (18.5° → 18.0864°) — the one closure that
  moves the quantity the drawing actually contradicts, since the cone closes on its own printed
  numbers and the flare does not. Over all three the spread is 0.64 points in the slope and 0.0043
  calibres in the centre of pressure, and Mach 1.50 is refused in every one, so the comparison
  does not rest on the choice. The angles are kept because they are the report's *text* (printed
  p. 8), stated to three decimals, while the lengths are only its drawing; the drawing's fourth
  longitudinal dimension, 0.722 from the blend arc's centre to the juncture, is recorded as an
  independent check on the derived nose and agrees to 0.0004 diameters.

- **A blunt nose may take more than one segment, and the cap may hand over on a later one — but
  never past the nose.** Model 2's nose is a 0.257 sphere blended into its cone by a 0.429 arc
  whose centre sits 0.135 below the axis, and the sphere is still at 38.3° where the arc takes
  over, steeper than the handover's 24° cap at any Mach number. `ShockExpansionBody::handover_m`
  therefore searches the nose's segments instead of the first segment alone, and `lay_out` skips
  the segments a cap covers wholly. The same run says which elements the rule on a reduced element
  treats as the nose's.

  What counts as the nose is deliberately narrow: normally the first segment and nothing else,
  and behind a **spherical cap** — the one shape that is a piece of a nose rather than a whole one
  — the curved segments that follow it, stopping at the first that is straight *or narrows*. So a
  pointed nose followed by a curved widening transition reads exactly as it did before, and a cap
  that reached a cylinder or a boattail is still refused: it would hand the flow over at no angle
  at all, with none of the total pressure the tip took out of it. No committed number moved, and
  `a_blunt_nose_hands_over_on_a_later_segment_but_never_past_the_nose` pins all four cases.

- **What it is worth is published as three tables, with no target.** M1.8e18 set none, and none is
  invented here. hpr reads model 2's normal-force slope −1.9% at Mach 1.90, +7.0% at 2.30, +13.4%
  at 2.96, +51.5% at 3.95 and +50.4% at 4.63, with the centre of pressure within 0.05 calibres
  through Mach 2.96 and 0.088 at 3.95. Model 1, unflared, reads −1.2%, +0.0%, +7.5%, +12.5%,
  +29.7% and +32.1% over the same six rows, so **the flare adds between −1.9 and +0.9 points
  through Mach 2.96 and 21.7 and 18.3 points at 3.95 and 4.63**. That is where the report's own
  shadowgraphs show the laminar boundary layer separating ahead of the juncture (printed p. 10),
  and where the measured `C_Nα` itself falls from 1.594 to 1.270 while both attached-flow methods
  on the figure stay between 1.57 and 1.96. Read as **consistent with** separation, not measured
  by it: the report blames the separated flow for its own method's disagreement with the measured
  *pressures* and for its over-prediction of *axial* force (printed p. 10 and p. 12), and says
  nothing about the normal force; and its own method is uniformly high on model 2 (+24.8% and
  +31.6% at the attached Mach 1.90 and 2.30 against +28.8% and +20.3% at the separated rows), so
  it carries no separation signature to corroborate with.

- **Below about Mach 1.5289 hpr has no reading for model 2, and that is left standing.** At Mach
  1.50 the 18.5° flare is steeper than the 15.4885° its corner's shock holds, so ADR-047 draws it
  out to 15.4885° — and the march then refuses, because the corner's *isentropic* turn runs out at
  15.3647°. ADR-045 and ADR-047 bound different quantities and cross near Mach 1.55; below the
  crossing the march's bound is the tighter one, so drawing a flare out to the shock's bound can
  land past what the march can do. Bisected to `f64` resolution the first reading is at Mach
  1.5288696 (the fixture keeps the whole f64; seven figures is what the three platforms agree
  on), where the two bounds agree to five parts in 1e14, both 16.2844275°:
  the reading begins exactly where they meet. Swept every 0.005 Mach from 1.05 to 4.63, the
  reading turns on exactly once, so "the first" is a measured claim and not the artefact of a
  bisection over a predicate that can refuse in bands. A flared body below that takes slender-body
  theory, carried up by the join.

- **A refusal is committed with its numbers rounded.** Where the method declines a row, the
  fixture keeps `hpr-aero`'s own words for why — but that text prints `f64`s in full, and the same
  refusal reads `Mach 1.5250956752494207` on macOS and `Mach 1.525095675249415` on Linux. The
  fixture's *numbers* are compared to 1e-12 relative; its *strings* are compared as text, so the
  message is rounded to six decimals before it is stored. Six decimals is far more than the
  reading is worth and far less than the platforms disagree at. For the same reason the guide
  quotes seven figures of the bisected Mach number, not the sixteen the fixture keeps.

**Consequences.**

- The guide's flare section no longer says that nothing has been compared with a measurement; it
  says what the comparison found, on one body and one flare angle, three of whose six rows are a
  separated flare.
- A separated flare is now a named, sized gap rather than an unexamined one: +51.5% and +50.4% on
  this body, with nothing in the sources to say how that scales with a flare's angle, its length or
  the boundary layer's thickness.
- The drawn-out reading past the limit is still measured against nothing. The one row where it
  would have applied is the row the march then refused, so ADR-047's rule above the limit remains a
  construction chosen for continuity, not a checked one.
- `xtask/src/aero_flare.rs` repeats, on a hand-built body, the six lines `SupersonicRun::shares`
  runs for a flare, because `hpr-design` has no spherical-cap nose and model 2 cannot be flown
  through a `Rocket`. `the_flare_is_read_as_the_model_reads_it` pins the two share by share to a
  part in 1e12, on a flared body the design route can express, above the corner's limit and below
  it, so the fixture cannot quietly drift into being a second model.

**Not chosen: read model 2 through a `Rocket` by adding a spherical-cap nose to `hpr-design`.** A
design-level blunt nose is a real gap, but it is a design feature with file formats, mass
properties and a UI behind it, not a step in this milestone. The pinning test buys the same
guarantee for the price of one test.

**Not chosen: draw a refused flare out to the march's own edge instead of the shock's.** It would
have given a number at Mach 1.50 rather than a refusal, and the number would have been a third rule
invented to avoid an empty cell — with the report itself saying the flare's shock is detached
there, and its own method +28.6% high. A refusal that falls back to slender-body theory is the
honest reading, and where it starts is now measured to `f64` resolution.

**Not chosen: call the 3.95 and 4.63 rows an error in hpr.** They are a flow hpr does not model,
named by the report that measured them. Counting them as a model error would hide that the same
rows read +29.7% and +32.1% on the unflared model 1, and that the report's own attached-flow method
goes the same way.
