# ADR-092: A pod's parts are Barrowman's, once per pod, on the axis (2026-09-27)

- **Status:** accepted
- **Summary:** A pod's parts are Barrowman's, once per pod, on the axis

**Context.** M1.13c asks for each pod's normal force and drag, and its interference with the body,
from a cited source, and a pod design matching OpenRocket. No published source models a pod whole.
OpenRocket documents nothing on its pod aerodynamics; its developers' posts say a pod's forces are
counted per pod and interference is not. The aerodynamic model was built for an airframe on one
axis: forces as a normal-force slope and a moment along it, drag as one axial force, and the
flight engine applying each component's force on the axis. The corpus's pods are OpenRocket's two
examples (one with a single pod) and one private design with a lug on each of its pods.

**Decision.**

1. **Split c1, c2.** M1.13c1 flies pods on the cited model below, checked by hand. M1.13c2 is the
   milestone's second bullet: a pod design with bodies and fins against OpenRocket, with both
   codes' limits stated, which needs probe designs flown in OpenRocket: neither of its own pod
   examples flies in hpr for reasons outside the pods (a freeform fin hpr leaves out; rail buttons
   read without their screw heads).
2. **A pod's body components are Barrowman's** (Barrowman 1967 eq. 3-65, Niskanen 2009 eq. 3.19):
   `C_Nα = 2 ΔA/A_ref` at the part's own centre of pressure, the step from the pod's previous part
   included and its first part stepping up from nothing, as the airframe's first does. Slender-body
   theory's slope is kept at every Mach number: the shock-expansion method covers the airframe
   alone, and a pod's slope kept out of the airframe's list leaves the airframe's march unchanged.
   Body lift is Jorgensen's crossflow at the pod's own fineness. A pod adds all of it once per pod.
3. **A pod's fins are the pod's, turned with it.** Copy `k`'s fin `j` stands at `θ_j + φ_k` in the
   flow; the fin–fin factor counts one pod's fins; the body interference is the pod tube's (1 on a
   tube of no radius). Canted fins on a pod are refused: their roll forcing about the rocket's
   axis, from a root off it, is not modelled.
4. **Drag once per pod.** Each pod gets its own buildup: friction at its own fineness's form
   factor, its nose, steps, boattails and base, coupled among its own parts only; its fins, lugs
   and buttons once per pod. The Reynolds number and the roughness scale stay the rocket's length,
   as for every part. A burning motor's area comes off the base of the body it sits in: the
   airframe's motors off the airframe's base, a pod's off its own pod's (each pod its share of the
   pods' total). Motor mounts in two different pod sets are refused. A pod's tube of no length with
   a radius (a flat disc) is refused; one with no radius, the phantom body, adds nothing.
5. **Forces act on the axis; roll damping is added.** For two pods or more, spaced evenly, the
   pods' offsets add to zero, so their forces summed on the axis have the moments they would have
   at the pods, to first order. What an offset adds at first order is roll damping: a body part
   `ρ` from the axis gives `C_lp = −2 C_Nα ρ²/d²`, and a pod's fin damps by Barrowman's strips taken
   about the rocket's axis, each strip `ρ_0 + y` out, `ρ_0` the root's distance along the span.
   Summed over the fins, that is quadratic in `ρ_0`, so two strip evaluations at the offsets' mean
   plus and minus their standard deviation give the sum exactly. The whole sum takes the pod
   tube's roll-damping factor `k_R(B)`: exact on a tube of no radius; on the offset part, whose
   factor is nearer `K_T(B)`, some 10% small at `τ = 2` (1.33 against 1.5), not sized by a test.
   The pods' drag also damps pitch, `C_mq ≈ −2 C_D,pod Σρ²/d²`, about −0.15 on the worked example,
   against the fins' order of −10³: left
   out.
6. **Left out, sized in the docs.** The body's and pods' effect on each other's flow: in potential
   flow past the body the pods meet the air at `α (1 − (a²/r²) cos 2θ)` (NACA Report 1307 eq. 15
   for `θ = 90°`), with a side component `−α (a²/r²) sin 2θ`; both average out for three pods or
   more and not for one or two (up to ±46% on the worked example's pair). A pod sunk into the
   airframe is not refused (#206). Interference drag, as Barrowman and Niskanen leave it out. A single
   pod's off-axis drag and normal-force moments (issue #213; about 0.15° of trim on the worked
   example with a one-calibre margin).
7. **The tumble model** counts each pod's tubes and fins as the airframe's, once per pod: the
   documentation's fit has no term for pods, which the recovery page says.

**Consequences.**

- `hpr_aero::AeroModel` gains `pod_sets()` (`PodSetAero`), `fin_set_start()` and
  `FinSetAero::pods` (`PodFins`); components run airframe bodies, pod bodies, fin sets.
  `ComponentDragTerms` gains `copies`, `in_pod` and `pod_holds_motors`, and `DragConditions` gains
  `thrusting_pod_motor_area_m2` (`with_pod_motors`). `hpr_design::Layout` gains `pod_set_of`. The
  flight engine's first fin index is `fin_set_start()`, and it splits the burning motors' area
  between the airframe and the pods.
- Tests hold the rules to hand-worked values: slopes and centres to 1e-11, body lift and drag
  shares to 1e-12, roll damping to 1e-12 and to Barrowman's strips summed by hand over pods with
  fins pointing each way to 1e-7. Valetudo with three pods and canted fins spins to the balance
  the pods' damping predicts, to 1e-6.
- The private design whose pods hold a lug now flies: over its 5 configurations, apogee −0.94% to
  +2.15% from OpenRocket's, margin +0.0041 to +0.0048 calibres (census accepted); 3 of the 5 are
  compared with an OpenRocket flight whose parachute opened early. Its pods add only the lug's
  drag, so it checks the pods' placement and weight, not these rules: M1.13c2 does.
