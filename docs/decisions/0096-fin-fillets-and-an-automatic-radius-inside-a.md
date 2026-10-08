# ADR-096: Fin fillets and an automatic radius inside a nose cone read as OpenRocket reads them (2026-09-28)

- **Status:** accepted
- **Summary:** Fin fillets and an automatic radius inside a nose cone read as OpenRocket reads them

**Context.** Issue #174 holds back two private designs from M2.2's count of 20. `C01` has fin
fillets, the rounded glue joint along a fin root. hpr left them out with a warning, a measured
departure since ADR-064 (−0.808% and −2.79% of the probe's mass at 5 and 10 mm). A warning makes a
design "an airframe not read exactly as written", which is not flown (ADR-055). `C06` has fillets
too, and a coupler inside a nose cone whose outer radius is `auto`. hpr's neighbour rule
(ADR-007, ADR-054) looks only at body tubes, so inside a nose it found nothing and took
OpenRocket's default, which is wrong there.

**Decision.**

1. **Measured, not assumed.** `conventions.py` gained 21 probes. The 86 earlier probes' answers are
   unchanged.
   - Seven fillet probes: a 30 mm radius, their own material, no material named, a single fin,
     four rounded fins, a freeform fin set and a wider tube.
   - Fourteen bore probes: a coupler with an automatic radius at a nose's bottom, past its base,
     with a shoulder, long from its middle, at the tip, and with a wall thicker than its bore; an
     ogive nose; a transition; an engine block; a centering ring and a bulkhead; a mass component
     inside; an inner tube written `auto`, in a nose and in a tube.
2. **A fillet is a prism of its section along the root.** The section is the region between the
   tube, the fin's mid-plane and the fillet's circle: OpenRocket's reading, which leaves the fin's
   thickness out. With body radius `R` and fillet radius `r`,
   the circle's centre is `c = √(R² + 2Rr)` along the fin and `r` off it, and the region is the
   triangle `(0, 0)`, `(c, 0)`, `(c, r)` less the body's sector up to `θ = atan2(r, c)` and the
   fillet's quarter circle up to the tangent points. There is one fillet each side of every fin,
   of the root chord's length, in `filletmaterial` (cardboard's density when none is named, as
   OpenRocket does). hpr computes the section's area, first moment and second moments in closed
   form, the sectors' second moments in a stable form, and tests it by quadrature to 1e-11. The
   final subtraction still loses about `log10(R/r)` digits, at most 2 for a real fillet
   (`r/R ≥ 0.01`). Past a thousand times the body radius it cancels away, so such a fillet is
   refused; under a millionth of it, or on a body of no radius, a fillet weighs nothing.
   OpenRocket's mass and centre of mass agree to 1e-15 on every probe (the test holds them to
   1e-12). The pitch inertia is apart by −0.0077% to −0.638%, largest with the 30 mm fillets, and
   +0.425% on a single fin: hpr's is the exact prism, and OpenRocket's pitch rule for fins is not
   measured (ADR-062).
3. **An automatic outer radius inside a nose or transition is its bore at the narrow end.** It is
   the parent's outer radius at whichever end of the part is narrower, less the parent's wall,
   the shoulder left out; zero if the wall is thicker. A filled nose has no bore and is refused.
   A packed radius inside a nose is still not taken. A coupler at the tip, where the wall meets
   the axis, has no radius: OpenRocket weighs it as nothing, and hpr refuses the design. An inner
   tube whose own radius is thinner than its wall is solid, in a body tube too.
4. **An `innertube` written `auto` keeps 9.5 mm**, as OpenRocket does (a motor mount's radius is
   not something OpenRocket works out). It raises no warning: it is OpenRocket's reading, and a
   warning would keep the design from flying (ADR-055). A number written after `auto` is not an
   input (ADR-052), so it is not kept either.
5. **Held to the probes.** `hpr_validate::openrocket` holds the fillets' mass, centre, roll and
   pitch inertia (pinned rows), and every bore probe's parts to OpenRocket's mass (to 1e-14) and
   station (to 1e-15). The noses carry the gaps their walls already had: up to 3.9e-5 of the mass and 2.5e-6 m.
   One mass component is pinned apart: OpenRocket shortens its packed length to keep its volume,
   and hpr keeps the written length (#186).

**Consequences.**

- `C01` and `C06` fly. The two reports hold 18 designs.
- In the mass survey the fillet cause is gone. One private design stays outside 1% in mass: its
  airfoil fins, which OpenRocket weighs as the outline times the thickness times 0.85 (ADR-062),
  where hpr integrates the NACA section. That cause is now sized: `cargo xtask ork` names it only
  when the design comes within both thresholds with its fin sections weighed OpenRocket's way.
- The library report's largest speed is now taken over every sample up to the apogee. The old
  rule stopped at the first sample whose vertical speed stopped being positive. A rocket held on
  the pad can read a few µm/s either way (#223), so on `C06/1` the old rule stopped before
  liftoff.
- `C06/1`'s apogee is 13.60% above OpenRocket's; ADR-097 sizes it.
