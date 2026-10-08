# ADR-012: Recovery: drag areas, triggers, inflation and the descent phase (2026-09-17)

- **Status:** accepted
- **Summary:** Recovery: drag areas, triggers, inflation and the descent phase

**Context.** M1.7a adds parachutes to the M1.6 flight engine. What is in hand:

- Knacke's *Parachute Recovery Systems Design Manual* (NWC TP 6575, 1991), which prints canopy
  drag coefficients on the nominal area, canopy fill constants, drag-area growth laws, opening-load
  methods and the equilibrium descent speed. Its title page limits distribution, so it is cited,
  never redistributed, and no text is copied.
- RocketPy 1.13.0 (MIT), whose parachute phase is a point mass and which M1.7a is compared against
  within 3% (the milestone's *done when*).
- Loft's lessons L27 (instant inflation, no opening load, an unsourced body term, no drogue
  release), L28 (a step floor and a time cap that left descents unlanded), L29 (a parachute `C_D`
  copied from OpenRocket's GPL source) and L92 (the terminal-descent case).
- `hpr-design` already has parachute and streamer **parts**, which carry mass and packing but no
  drag or deployment data.

**Decision.**

- **The milestone is split.** M1.7a is parachutes, triggers, inflation, release, drift and landing,
  with both of M1.7's *done when* bullets. M1.7b is streamers, tumble and separated bodies, with
  its own bullets. Streamers and tumble need drag sources Knacke does not have (he has no streamer
  data at all), and separation needs a decision about how a body's mass and drag are defined; that
  is a milestone's worth of work on its own.
- **Recovery devices live in `hpr-sim`, not in the design tree.** A `Device` is a drag area, a
  trigger, a lag and an inflation law, given to a `Simulation` through `with_recovery`. The design's
  `Parachute` part stays what it is: mass and packing. A design file that carries deployment data is
  M3.1's problem (`.ork` has it) and can map onto these types.
- **Drag areas are `C_D S`, either given directly (RocketPy's `cd_s`) or from a canopy's nominal
  diameter with `C_D0` on the nominal area `S₀ = π D₀²/4`** (Knacke's convention, printed page 5-2).
- **Canopy data comes from Knacke's tables, with the printed page at each accessor**, for thirteen
  types: the `C_D0` range (Tables 5-1 and 5-2), the fill constant (Table 5-6, unreefed), Pflanz's
  drag-area growth exponent (Figure 5-51) and the infinite-mass opening-force coefficient `C_x`.
  - **The default `C_D0` is the middle of the printed range** (flat circular: 0.775 of 0.75 to
    0.80). Knacke prints no single value, and hpr will not copy OpenRocket's 0.8 (L29). Where his
    tables say "insufficient data" the accessor returns `None` and the caller must supply a number.
  - RocketPy's default `C_D` of 1.4 is a hemispherical canopy's coefficient on the **projected**
    area, used only to turn `cd_s` into a radius for its added mass. It is not a `C_D0`.
- **Inflation: `(C_D S)(t) = (C_D S)₀ min(1, (t − t_d)/t_f)^j`**, with `t_f` either zero (instant,
  RocketPy's model), fixed, or Knacke's `t_f = n D₀/v` from the airspeed at line stretch.
  - Knacke's measured **overshoot** (10 to 80% above the steady drag area, Figure 5-40) and his
    `C_x`/`X1` opening-load methods are **not** modelled. hpr's peak load is therefore a lower
    bound on the real opening shock, and instant inflation is hpr's own upper bound. Both are
    stated in `docs/physics/recovery.md`; Ludtke's and Pflanz's laws are later work.
    - *Corrected 2026-09-18 (M0.4b's physics review):* neither bound holds in general. Opening at
      once is Knacke's infinite-mass case, and his force-reduction factor `X1` (printed page 5-50)
      falls as low as 0.02 for a canopy large for its load, so the real peak can be far below
      hpr's. Near the canopy's terminal speed, as at apogee, a filling time gives hpr a higher
      peak than an instant opening. `docs/physics/recovery.md` states the corrected limits.
  - A deployment at zero airspeed has no filling time in `n D₀/v`, so the canopy opens at once.
- **Triggers: apogee, a height above the site while descending, a time after ignition, and a
  motor's ejection delay after its burnout.** Both the apogee and the height trigger are numeric as
  well as event-driven — "descending", and "descending at or below the height" — which is
  RocketPy's own form (`y[5] < 0`) and an altimeter's behaviour, and which means a flight that
  starts past its apogee still deploys (found in review: an event-only apogee trigger fell
  ballistically to the ground with no deployment and no error). A motor with no delay in seconds
  (plugged, or unset) is refused rather than assumed.
  There is no sampling rate and no barometric noise: hpr locates the crossing with its event
  finder. Monte Carlo can perturb the setting instead (M1.10).
- **A device can be released by another's opening** (`released_by`), which cuts a drogue away under
  a main (L27). The release waits until the releasing device is **fully open**: releasing at its
  line stretch collapsed the drag area to almost nothing while the main filled, and the descent
  sped up (found in review, now a test). A device released before its own charge fires never
  deploys. Open devices otherwise **add** their drag areas, where RocketPy keeps one `cd_s` and
  replaces it, which is why the comparison gives its drogue `released_by` the main.
- **The descent is a point mass in a new `Phase::Descent`**, entered at the first deployment:
  `m a_cg = −½ ρ (C_D S) |v_cg − w| (v_cg − w) + m (g + a_Coriolis) + T`.
  - The attitude freezes and the body rates are set to zero. A tethered rocket's attitude under a
    canopy is not modelled by anything hpr can cite.
  - **The airframe's own drag is left out**, as RocketPy leaves it out, rather than carrying Loft's
    unsourced `0.5 A_ref` term (L27). For a drogue whose drag area is near the body's broadside
    area this is a real omission, and it is where M1.7b's tumble drag belongs.
  - The thrust is kept along the frozen axis, so an off-nominal deployment under thrust is not
    silently thrust-free.
  - **Added mass is not modelled.** Knacke gives no closed form (printed page 5-40 is qualitative),
    and RocketPy's `m_a = k_a ρ (2/3) π R² H` carries no citation in its code and no weight in its
    equations, so it changes no equilibrium rate, only the transient. The comparison shows the
    cost: NDRT 2020, whose main's added mass is 15.9 kg against a 20.8 kg rocket, is hpr's worst
    case at +0.71% in descent time and +2.86% in the smaller drift component.
- **Every deployment, every end of filling (which is also a release) and every known trigger time
  is a stop time**, so no step straddles a change in the drag area, and events are located as in
  M1.6a. The numeric triggers are checked once per interval, on one evaluation shared by every
  pending device, which `Stats` does not count. The event list is
  now built per interval as a `Watch` list instead of numbered by hand, because the height triggers
  come and go.
- **The comparison with RocketPy starts where both models agree.** Both simulators start from the
  same declared post-burnout state near apogee with the first device's lag overridden to zero, so
  almost no segment under either model's aerodynamics separates them (RocketPy's trigger sampling
  leaves 2.5 to 13 ms of its own 6-DOF flight, `recovery.md`), with RocketPy's noise zeroed (it
  draws on the global `np.random`) and a declared wind (the examples' own winds need the network or
  Copernicus files). Five example rockets. The oracle runs at `rtol = atol = 1e-8` and records its
  own solver's contribution per metric: at most 3.5e-6 on every compared metric (its one larger
  entry, 2.1e-3, is on a 20 µm drift component this comparison did not compare; M2.1a measures it,
  and reading 28x high there is what found the gravity-model difference of ADR-015, issue #27). The
  differences that remain
  are listed in `recovery.md`, with the trigger-sampling one measured rather than assumed away. Whole-flight comparisons, ascent included, are M2.1's.

**Consequences.**

- M1.7b adds streamers, tumble and separation. It needs a streamer drag source (Knacke has none),
  a tumble model, and a decision about how a separated body's mass and drag are defined.
- A cited apparent-mass model would close the gap against RocketPy's transient; it is the only
  known model difference left in the descent.
- `Sample` gained `recovery_drag_area_m2` and `Channel` gained `RecoveryDragArea`, so reports can
  show the opening.
- M1.10's Monte Carlo can vary deployment heights, lags and drag areas; nothing here samples.
- M3.1 maps `.ork` recovery devices onto these types.
