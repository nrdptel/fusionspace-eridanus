# ADR-093: Pod probes flown as the public designs are, and listed apart (2026-09-27)

- **Status:** accepted
- **Summary:** Pod probes flown as the public designs are, and listed apart

**Context.** M1.13's second bullet asks for a pod design within the per-case tolerance of
OpenRocket, with the limits of both codes' pod models stated. ADR-092 left it to M1.13c2 because
no design that flies has pods with bodies or fins: OpenRocket's two pod examples fail in hpr for
other reasons, and the one private design with pods holds only lugs. OpenRocket documents no pod
model, and its source is not to be read.

**Decision.**

1. **Probe designs, written by a script.** `validation/oracles/openrocket/pod_probes.py` writes
   six plain-XML `.ork` files to `validation/fixtures/ork/pod-flights/`: one airframe on an
   AeroTech H128W (named by OpenRocket's database digest, so both codes fly one curve, ADR-067)
   carrying three body pods, two finned pods, four finned pods with tail cones, two pods of no
   length with winglets turned to lift in the plane the margin is taken in, two finned pods with a
   motor each and none in the airframe, or no pods (the control). Every outside
   part states `<finish>normal</finish>`, since the two codes read a missing finish differently
   (issue #216).
2. **Flown by the existing pipeline.** `flights.py` flies them with the public designs, and
   `cargo xtask ork-flights` flies hpr on them, so they take the same conditions, curve checks and
   metric definitions as every other comparison.
3. **Listed apart.** A probe is not a design: the report holds them in `probes`, out of the
   designs' statistics and out of M2.2's count toward twenty designs, and a probe hpr cannot fly
   is an error rather than a row in `not_flown`. The census (ADR-084) leaves them out too: its
   OpenRocket group is the examples' headline in the README and the accuracy page, and probes
   would pad it. In CI they are held instead by the bounds in item 4 and by the test that ties
   every report row to the record and to `compare`; `ork-flights --check` holds them locally.
4. **The bar.** Each probe within 5% of OpenRocket's apogee and largest speed (ADR-076's per-case
   tolerance). Because a calm vertical flight tests drag and mass far more than normal force, each
   margin is held within 0.005 calibres, and what the pods change against the control, in apogee
   and in margin at rod clearance, within 1 point and 0.005 calibres: bounds set after measuring
   0.32 points and 0.0004 calibres, tight enough that a pod fin's interference factor taken on the
   airframe (some 0.04 calibres) fails.

**Consequences.**

- Met: apogee +0.41% to +0.81%, largest speed +0.59% to +1.24%, margin +0.0011 to +0.0014
  calibres; the pods change the apogee from −31.3% to +13.0% in OpenRocket and within 0.32 points
  of that in hpr, and the margin within 0.0004 calibres.
- Agreement this close, with interference left out by both, means OpenRocket's pod model is, in
  effect, hpr's on these probes' drag and centre of pressure: the comparison shows the rules are
  applied alike, and cannot size what both leave out. A single pod, a pod past Mach 0.81, flight at
  an angle of attack (and so a pod's body lift), airframe and pod motors burning together, and roll
  are not tested.
- With no `<finish>` stated, the probes flew 6.0% to 7.3% high in hpr: its reader gives such a part
  hpr's 20 µm default where OpenRocket reads 60 µm (issue #216).
