# ADR-099: Tube fins flown as ring wings (2026-09-28)

- **Status:** accepted
- **Summary:** Tube fins flown as ring wings

**Context.** Since ADR-098 hpr reads and weighs tube fins, but `hpr-aero` refused them for want
of a cited method, and `hpr-io` screened every configuration of a design holding them out of
flight (`NotFlown::NoAerodynamicModel`). M2.2e9 asks for a cited normal-force and drag method,
pinned by tests, and for OpenRocket's *Tube fin rocket* to fly with M2.2e4's bar on it: an apogee
more than 5% off needs a written, sized cause. OpenRocket's own source is GPL and was not read;
its method is known only from its pull requests' prose.

**Decision.**

1. **Each tube is a thin ring wing.** Its normal-force slope on the area `d L` (`d` the mean
   diameter, `λ = L/d`) is Weissinger's `π²/(1 + πλ/2 + λ arctan 1.2λ)` (1955, as quoted by Wagner
   2021, arXiv:2102.02647, eq. 15). It runs from Ribner's lifting-line `π²` for a short ring to
   slender-body theory's `π/λ` for a long one, which is Hoerner's `L = q d² π α` (*Fluid-Dynamic
   Drag*, 1965, p. 7-13): a ring turns the air inside it too. On Fletcher's five measured rings
   (NACA TN 4117, 1957, Fig. 11, Mach 0.13, `A = d/c` from 1/3 to 3), it is within 3% (2.3% at
   worst, read from the chart). Faster than Mach 0 the slope takes Göthert's rule,
   `C_Lα(λ/β)/β`, the idea of Barrowman's fin slope. It leaves the long-ring limit unchanged.
2. **The set is `N` times one tube, with no interference factor.** None is measured or cited. The
   body's crossflow at a tube, `R²/s² e^{−2iφ}`, sums to zero over three or more tubes evenly
   spaced, so the model refuses fewer than three. That is a derivation, stated as one.
3. **The centre is Fletcher's measured aerodynamic centre, from `A = 2/3`.** Fletcher's Fig. 8
   (p. 16) reads 0.143, 0.203, 0.253 and 0.355 of the chord at `A = 2/3`, 1, 1.5 and 3. The
   table ends there: a shorter ring is refused (point 5). Below `A = 2/3` the centre runs in a straight line to the leading edge at
   `A = 0`, where hpr's own slender-body derivation puts a thin ring's lift. His `A = 1/3`
   ring, whose centre sits 0.11 of its chord ahead of its leading edge, is left out. Fletcher
   puts that point down to its low aspect ratio: that ring behaves more like a slender body of
   revolution than the others (p. 4). That it would not carry over to a paper tube is hpr's
   inference, untested: his rings had a Clark Y section 11.7% of the chord thick, all outside a
   straight bore, so at a chord of three bores the wall is 0.35 of the bore thick and about two
   thirds of the frontal disc, against a few per cent for a paper tube. His thinner rings may
   carry some of the same forward shift. The choice is a judgement that moves the *Tube fin
   rocket*'s margin a long way: 0.29 calibres with his −0.11 held below `A = 1/3` (a one-off
   run, not in the report), 0.79 with the line, against OpenRocket's 1.87. The rule is taken at the
   stretched ring's `β A`.
4. **The drag is friction inside and out, and a square edge on the wall.** Friction is on
   `2π L (r_o + r_i)` per tube at the rocket's skin-friction coefficient. The wall's annulus
   takes a square fin edge's pressure drag: stagnation at the front and base drag behind
   (Niskanen 2009, eqs. 3.90 and 3.92).
5. **Mach 0.8 and above are refused**, by the normal force, the component stations, the roll and
   the drag buildup. No source covers tube fins near the speed of sound, where a tube's flow may
   choke. A flight that reaches Mach 0.8 stops with the tube-fin model's error. Also refused:
   solid tubes, tube fins on a pod, a ring shorter than a third of its diameter (`A > 3`, past
   Fletcher's rings), and tubes that overlap each other. Roll damping follows the pods' rule,
   `−2 C_Nα ρ²/d²` per tube at its axis's distance `ρ`, a derivation. Pitch damping comes from the
   local flow at the set's centre, like any other component's.
6. **The importer's screen is gone.** `NotFlown::NoAerodynamicModel` had no other use and is
   removed. A tube fin set the model still refuses fails the flight, as any refused part does.
7. **The public report sizes a cause in the drag as the library's does** (ADR-097).
   `drag_curves.py`, unchanged, flew the jar's 17 examples, unpacked outside `refs/`. Its record
   is committed as `validation/fixtures/ork/openrocket-drag-curves.json`, since the examples are
   public. `cargo xtask ork-flights` reads it for the public designs. A test holds it to the
   flight record's examples, by digest, and to the current scripts. The public report's tests
   hold their own `DRAG_CAUSES_WRITTEN`, which names this record for the *Tube fin rocket*.

**Evidence.**

- `hpr_aero::tube_fins` tests:
  - the slope against Fletcher's five rings within 3%, and both of its limits;
  - the centre against the table, the line to the leading edge, and the hold past `A = 3`.
- `hpr_aero::model` tests:
  - a set's slope, moment, station, listed component and roll damping against the formulas by
    hand, at Mach 0, 0.3 and 0.7, with no side force at any roll;
  - its friction and wall drag by hand;
  - the refusal at Mach 0.8 in all seven calls, in one shape, pinned to the tube-fin model's
    error, and in the tubes' own drag terms called alone;
  - a short ring's slope by hand at Mach 0 and 0.6, and its centre from Fletcher's table;
  - the guide's worked example, slope, centre and areas;
  - the refusal of two tubes, solid tubes, overlapping tubes and a ring past `A = 3`.
- `hpr_sim` flies a rocket with six tube fins below Mach 0.8 and stops its flight past it with
  the tube-fin model's error.
- `cargo xtask ork-flights` flies the *Tube fin rocket* `[D12-7]`:
  - its apogee is 302.5 m against OpenRocket's 282.8 m, +6.95%, and its largest speed +6.40%;
  - on OpenRocket's drag curve they are +0.025% and +0.033%, so the cause is named "hpr's own
    drag coefficient", and M2.2e4's bar holds;
  - with only OpenRocket's whole base under power, the apogee is +4.61%.
- A look at OpenRocket's per-component output at Mach 0.2, not kept as a record, as ADR-097's
  was. Its numbers are leads for #228, not measurements the repository reproduces:
  - the nose, body and lug agree with hpr's within 0.005;
  - the tube fins take 1.18 of OpenRocket's 1.777 and 1.05 of hpr's 1.654;
  - OpenRocket's tube fins take a slope of 37.8 per radian, 1.62 times the 23.4 that six isolated
    thin rings reach at their long-ring limit; hpr's is 22.9;
  - OpenRocket's centre sits a quarter of the tube's length aft of the leading edge.
- OpenRocket's pull requests #1333 and #2066 refined its tube-fin drag against measured flights
  of two tube-fin rockets. The table, in a comment on #2066, has seven flights in five motor
  cases; OpenRocket's apogee is within −2% to +5.2% of the measured one and high in four of the
  five.

**Consequences.**

- **`Tube fin rocket [D12-7]`'s breakdown** (this record):
  - On OpenRocket's drag: apogee +0.025%, largest speed +0.033%, within the bar. The net gap is
    the drag's; parts of it could cancel.
  - With only OpenRocket's base rule, the whole base under power: +4.61% and +3.01%. That switch
    alone lowers the apogee by 2.34 percentage points; it is measured by its own flight.
  - Switching the rest of the drag to OpenRocket's then lowers it 4.59 percentage points. That
    number is by subtraction, and the split depends on which change is made first.
  - The look above points the rest to the tube fins, 1.18 of OpenRocket's drag coefficient
    against 1.05 of hpr's. It is a lead for #228, not a measurement the repository reproduces.
- hpr's tube-fin drag probably reads low. OpenRocket's was fitted to measured flights, and hpr's
  leaves out the gaps between the tubes and the body and how the flow develops inside a tube. No
  measurement checks either code here. #228 holds the search for a measured source, for the drag
  and for a thin tube's centre.
- The example's margin at rod clearance is 0.79 calibres, 1.08 below OpenRocket's 1.87. The quarter
  length alone would add about 0.51, by the look above. The report shows the gap. It is past the
  census's 0.5-calibre margin bar (ADR-084), accepted in writing with this record; M2.2e4's bar
  covers only the apogee. Nothing measured separates the two codes.
- The configurations the reader flies go from 106 to 108 of 170, in 29 designs. The public report
  flies 34 configurations and names a cause for each of its seven apogees over 5%. With the
  library, 19 designs carry all five spreads, one short of M2.2e10's 20.
- A tumbling airframe with tube fins is still refused: the tumble model has no factor for them.
- A flight on a drag or normal-force override table (ADR-009, ADR-032) still stops at Mach 0.8
  when it holds tube fins: it takes the components' stations and the roll from the model, which
  refuse past it.
