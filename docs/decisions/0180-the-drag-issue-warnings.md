# ADR-180: The drag issue warnings, and M1.14a2's split (2026-10-06)

- **Status:** accepted
- **Summary:** M1.14a2 splits in two: a2 the warnings for the five drag issues on ADR-163 §4's list (#67, #68, #70, #72, #222), a3 the four stability ones (#87, #120, #121, #172). A drag warning comes from the flight's top Mach number and the design's parts: a nose or shoulder whose shape takes the closed-form cone past Mach 0.8 (#67); every rocket past Mach 0.8, adding the low side from 1.5 once past 1.2 (#68); airfoil fins past Mach 1 (#70); a boattail steeper than 10° past Mach 1 (#72); an ogive nose or airfoil fins past Mach 1 (#222). Each Mach edge is exclusive; a NaN top Mach number raises every warning the design's shape allows; a flight on a drag of its own raises none. The library, `hpr sim` (text and JSON) and Python report them.

**Context.** [ADR-163](0163-the-second-pass-products-and-priorities.md) §4 lists nine issues that
must warn, by number, before release 0.1, when a flight or design meets the issue's Mach or
geometry condition. [ADR-179](0179-the-envelope-flags.md) split M1.14a and left the nine to
M1.14a2. They fall in two kinds:

- **Five are drag that reads high** (#67, #68, #70, #72, #222). Their conditions are a Mach band
  and a shape the design states outright: a nose's shape, a fin set's section, a transition's
  radii.
- **Four are stability that reads high** (#87, #120, #121, #172). Three are switches inside the
  supersonic body method, whose condition is whether `hpr_aero`'s shock-expansion run covers the
  whole body and which rule stopped it. The fourth, #172, measured on two private designs, states
  no condition at all, so one has to be chosen.

The two kinds share no code, and the second needs the aerodynamics to say which switch fired.

**Decision.**

1. **M1.14a2 splits** into M1.14a2, the five drag warnings, and M1.14a3, the four stability
   warnings, each with its own *done when*. The parent keeps its own.
2. **The conditions,** each Mach edge exclusive, as the envelope's are:

   | issue | condition | why this edge |
   |---|---|---|
   | #67 | a nose, or a shoulder (a transition widening aft), whose shape takes Niskanen's closed-form cone (eq. B.3–B.6, times eq. B.8 for an ogive): a cone, an ogive, a power series of exponent above ¾ or a parabolic series of parameter below ½, which blend toward it (`hpr_aero::nose_drag::takes_cone_formula`); past Mach 0.8 | the closed form reads +87% at Mach 0.8 and +105% at 0.85 against Stoney's measured 3:1 cone (NASA TR R-100), 2 to 9 times MIL-HDBK-762's 3:1 ogive from Mach 0.9 to 1.2 |
   | #68 | every rocket, past Mach 0.8; past Mach 1.2 the message adds that from Mach 1.5 base drag reads low | Fleeman's base drag reads high from Mach 0.8 to 1.2 against MIL-HDBK-762's data (0.225 against 0.156 at 0.9, 0.208 against 0.194 at 1.2), and low from 1.5 against Love's correlation (5% at 1.5, 17% at 2.0); between 1.2 and 1.5 the sign is unmeasured |
   | #70 | an `Airfoil` fin set, past Mach 1 | hpr has no sharp section, so a sharp fin is drawn as an airfoil; the gap was measured from Mach 1.5, and the warning starts at Mach 1, the lowest of the supersonic issues' edges |
   | #72 | a transition of some length narrowing aft at a mean angle `atan((r_fore − r_aft)/l)` above 10°, past Mach 1 (one of no length is a step, not a boattail, to the drag buildup) | measured −6.3% to +17.4% at 10° and below, +13.5% to +50.8% at 15° and +26.4% to +54.1% at 16°; between 10° and 15° is unmeasured, so it warns |
   | #222 | an `Ogive` nose or an `Airfoil` fin set, past Mach 1 | the shapes where hpr's supersonic pressure drag was measured at about twice OpenRocket's |

   Where a source leaves a range unmeasured, the warning takes it in. A drag that reads high
   flatters the apogee against a waiver's ceiling, the top speed and the flutter margin, and an
   unwarned high drag is the worst class of error (ADR-162 §2).
3. **A NaN top Mach number** raises every warning whose shape the design has, and a NaN radius
   counts as a steep boattail, so a broken number can't hide one. A flight with no top Mach number
   raises none.
4. **A flight on a drag of its own** (`FlightBuilder::drag_table` or `drag_model`, Python's
   `drag_table=` and `drag=`) raises none: its drag isn't hpr's buildup, so the buildup's errors
   aren't its numbers'. A drag model that hands back hpr's own buildup loses its warnings too; the
   builder can't tell it from any other.
5. **Each warning carries** its issue number and address, the flight's top Mach number, the ids of
   the parts that meet its shape (none for #68) and a message saying which way its numbers lean:
   the apogee, the top speed and the drift low, the flutter margin and the largest dynamic pressure
   flattered. #68's message says, past Mach 1.2, that from Mach 1.5 the apogee reads high. #222's
   says which tool is right is unresolved.
6. **Where they show:** `hpr_sim::issues::issue_warnings`, from any parts, and
   `layout_issue_warnings`, from a design's layout; `hpr::Flight::issue_warnings`, computed when it
   flies (a record saved before has none); `hpr sim`'s `issues` in JSON and a `warning: drag:` line
   each in its text; Python's `Flight.issue_warnings`. A warning is not a verdict: every flight
   still flies, and the numbers print.

**Alternatives considered.**

- *All nine in one increment.* The three switch warnings need `hpr_aero` to report which of its
  rules stopped the supersonic run, a change inside the aerodynamics that deserves its own review.
- *Mach edges at each issue's first measured gap* (Mach 1.5 for #70 and #222, 1.0 for #72). These
  would leave Mach 1.0 to 1.5 unwarned where the error is unmeasured, not absent.
- *#72 from 15°.* The issue measured 10°, 15° and 16°; between 10° and 15° is unknown, and an
  unwarned high drag is the flattering side.
- *#67 below Mach 0.8.* Stoney's measurement starts at Mach 0.8, and eq. 3.87 below it rises
  toward the closed form's high value, so the error below Mach 0.8 is unmeasured rather than shown
  to be absent. The issue states its range from Mach 0.8, and a warning on every flight past
  Mach 0.5 or so would bury the rest; the edge stays at 0.8, a known limit of this warning.
- *Only noses for #67.* A shoulder drags with the same curve (`hpr_aero::drag`), so it carries
  the same error.

**Consequences.**

- Of the public designs `hpr sim` flies offline in their default configuration, 25 `.ork` files
  (OpenRocket 24.12's examples and the repository's fixtures) and 12 validation designs, flown off
  an 85° rail in calm air (`hpr sim <design> --offline --inclination 85 --wind 0 --json`, on
  2026-10-06), 8 raise at least one warning. #68 is raised on all 8, #67 on 4 and #222 on 2.
  #70 and #72 are raised on none: no public design that flies past Mach 1 has airfoil fins or a
  steep boattail. Their edges are pinned by unit tests only.
- The examples on the README and three guide pages, regenerated, now show the warnings of the
  Mach 0.8 and Mach 1.1 flights they print.
- The CLI's JSON schema gains `issues`.
- M1.14a3 remains before M1.14a is done; the queue keeps M1.14a first.
