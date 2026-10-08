# ADR-027: The normal force through Mach 1: supersonic linear theory, a transonic join, and the measured references (2026-09-18)

- **Status:** accepted
- **Summary:** The normal force through Mach 1: supersonic linear theory, a transonic join, and the measured references

**Context.** M1.8 is too big for one pull request and is split into M1.8a to M1.8d (`ROADMAP.md`).
M1.8a carries the normal force and centre of pressure through Mach 1, so that a flight on a drag
table can pass it; the drag buildup's transonic terms are M1.8b. Until now `hpr-aero` refused
`M ≥ 1` (ADR-008) and a flight stopped there (ADR-011), which left Prometheus 2022, the suite's
one supersonic rocket, a known gap in both modes. Loft lesson L7: Loft's fin slope had no
compressibility factor, so its slope and CP never moved with Mach.

Three findings shaped the method:

- *Niskanen's supersonic fin slope counts one surface.* [N09] eq. 3.48–3.49 write the fin's
  normal force as its area times one strip's pressure coefficient, `K₁α + K₂α² + K₃α³` with
  `K₁ = 2/β`. That is the pressure on one face. A flat plate carries the difference between its
  faces, `2(K₁α + K₃α³)` (the `K₂` terms cancel), whose first term is Ackeret's `4α/β`.
  Barrowman's own method sums every surface region ([B67] appendix A, pp. 82–86). Niskanen's
  thesis finds its simulated `C_Nα` for the Arcas Robin "notably lower than the experimental
  values", with the reason unknown (p. 91, over a comparison that runs to Mach 4). hpr uses both
  faces.
- *Barrowman's half-load in the tip's Mach cone is exact linear theory for a rectangle*, in lift
  and in CP: the exact cone load `(2/π) asin √t` averages one half, and [N09] eq. 3.35 (from
  Fleeman, *Tactical Missile Design*, p. 33) is the rectangle's exact CP. For tapered fins the
  strip method runs above exact linear theory: against [TN2114] eq. A7 (printed p. 18), a fin
  with a taper ratio of 0.5, an unswept trailing edge and `βA = 3` gets 4.5% more slope (worked
  by hand here; the research found +2.1% and +0.8% at `βA = 4` and 6).
- *No source gives the transonic region in closed form.* MIL-HDBK-762 reads it from McDevitt's
  transonic-similarity charts for rectangular fins (pp. 5-104–5-105), and [B67] §1.00 makes "no
  attempt" at it. RocketPy 1.13.0 flies Diederich's subsonic slope at every Mach number, with
  `β` held at 0.6 from Mach 0.8 to 1.1 (`aero_surface.py:51-56`); supersonic, that tends to
  `2π cos Γ_c/β`, about π/2 times linear theory's `4/β`. Its fin CP doesn't move with Mach.

**Decision.**

- **Supersonic fins: linear theory on the fin's outline** (`FinOutline::supersonic`). The load is
  `4α/β` per unit area, halved inside the Mach cone from the tip's leading edge, with the root a
  reflection plane (a cone that crosses the root counts its part over the mirror image). So
  `(C_Nα)₁ = (4/β)(A_fin − A_cone/2)/A_ref`, and the CP is the load's centroid. The outline is the
  planform's polygon (an ellipse as a 256-gon, 2.5e-5 short in area), clipped against the Mach
  line by Sutherland–Hodgman and summed by the shoelace formula without allocating. Only the
  first-order term: `K₃α²` adds 1% at 0.1 rad at Mach 2, but 35% at Mach 1.2, where the expansion
  itself fails.
- **Where linear theory starts:** `M_s = max(1.2, 1/cos Γ_L, 1/cos Γ_T, √(1 + 1/A²),
  √(1 + (c_t/2s)²))`: the bottom of [N09]'s supersonic region (Table 3.1), supersonic leading and
  trailing edges ([TN2114]'s case), `βA ≥ 1` with `A = 2s²/A_fin`, and the mirror fin's tip cone
  off this fin's tip, `β ≥ c_t/(2s)`. The tip-chord term came from review: `βA ≥ 1` alone let an
  inverse-tapered fin's two tip cones overlap its tip, and its slope rose past `M_s`. The
  trailing-edge term is TN 2114's stated case, supersonic trailing edges. Neither binds for any
  validated fin, so no reported number moved. Leading edges swept forward stay outside the
  half-load's domain: the tip sits ahead of the root, and the slope rises past `M_s` by up to
  +7.7% at 40° to 50° forward (issue #64).
- **The transonic join:** from Mach 0.8, the top of [N09]'s subsonic region and of the drag
  buildup's documented range, to `M_s`, slope and CP each linear in `M` between the subsonic
  method's values at 0.8 and linear theory's at `M_s`. Continuous, with the slope's peak at `M_s`.
  Below 0.8 nothing changes: Diederich's slope with Prandtl–Glauert at the quarter chord.
- **Bodies don't change with Mach** (slender-body theory, [B67] p. 18), and `K_T(B)` stays
  Barrowman's. `K_B(T)`, the fins' lift carried onto the body, stays out, as below Mach 1.
- **Ranges.** `AeroError::Mach` gains the refusing model's limit. The normal force covers
  `0 ≤ M < 5` (`NORMAL_FORCE_MACH_LIMIT`; [N09]'s hypersonic region starts at about 5); the drag
  buildup `0 ≤ M < 1` (`BUILDUP_MACH_LIMIT`) until M1.8b; a drag table any Mach number.
- **In flight.** A fin set's CP moves with Mach, so `AeroModel::component_station_m` takes the
  Mach number, and the flight takes each component's local airflow at its CP for the centre of
  mass's Mach number, each step.

**The references.**

- *RASAero II's Calisto export* (`refs/rocketpy-history/calisto-cd-test-2018.csv`, RocketPy's first
  commit, now pinned in the lock). Its `CNalpha (0 to 4 deg)` is the secant slope to 4° and, from
  Mach 0.95, includes a viscous cross-flow term that is zero below. So the comparison takes its
  `CN Potential` at 2° over the angle and its CP at 0°, against hpr's small-angle slope and CP. A
  code, not a measurement. The choice decides much of the result: against its 4° columns, 1 of the
  11 rows from Mach 0.8 is within the targets, not 6, and Mach 2 is −30.6%. Below Mach 0.8 the
  agreement is partly by construction, since ADR-009 gave the Calisto design the 2018 fins because
  they reproduce this export at low speed. The fixture commits the export's values at the 15 Mach
  numbers compared (30 values), more than ADR-009's one per curve; whether that is fine is the
  maintainer's call.
- *NASA's half-scale Arcas wind-tunnel models* (TN D-4013, Mach 0.6–1.2; TN D-4014, Mach
  1.5–4.63): the Arcas Robin (short, 18.2 calibers) and the long bioscience version (23.8). The
  reports print no tables, so their plots were read on 600-dpi renders, each plot's grid
  calibrated locally, into `validation/fixtures/aero/arcas-robin-wind-tunnel.json` with every
  point's figure and page. The CP read back from C_m and C_N reproduces TN D-4014 Fig. 7 within
  1.2% of body length (0.0% at Mach 2.96). The reports plot `C_N` against `α`, so hpr is fitted
  the same way: its `C_N` at the plotted angles, a least-squares slope; its CP from its moment and
  normal force over −2° to 2°. `cargo xtask designs` builds both models from the recorded
  geometry. The nose is a coordinate table, not a tangent ogive; the design's power-series nose
  (`n = 0.6369`) has its volume, which sets the slender-body CP, and its planform within 2.4%.

**Result** (`validation/fixtures/aero/normal-force-vs-mach.json`, pinned by
`hpr_aero::tests::normal_force_against_mach`; 37 rows, 16 outside the targets). The targets were
written into `ROADMAP.md` before the comparison first ran, and landed in the same commit as it;
they have not moved since.

| reference, band | `C_Nα`, hpr against the reference | CP, calibers | within both targets |
|---|---|---|---|
| Arcas Robin, short, Mach 1.5–2.96 | −13.4% to +3.3% | −0.02 to +0.42 | 4 of 4 |
| Arcas, long, Mach 1.8–2.96 | −8.2% to +0.5% | −0.12 to −0.04 | 3 of 3 |
| Arcas Robin, short, Mach 3.96–4.63 | −19.6%, −25.0% | −0.17, −0.16 | 0 of 2 |
| Arcas, long, Mach 3.96–4.63 | −17.2%, −22.8% | −0.16, −0.19 | 0 of 2 |
| Arcas, both, Mach 0.6 | −2.4%, +3.6% | +0.06, −0.44 | 2 of 2 |
| Arcas, both, Mach 0.8–1.2 | −6.9% to +29.3% | −0.46 to +2.29 | 2 of 9 |
| Calisto RASAero II, Mach 0.1–0.7 | +0.1% to +10.1% | −0.08 to +0.43 | 4 of 4 |
| Calisto RASAero II, Mach 0.8–2.0 | −16.8% to +21.9% | −0.56 to +0.95 | 6 of 11 |

Every miss, measured:

- *Past Mach 3: the body.* Fins on less fins off, the fins' share agrees with hpr's fins within
  −1.4% to +7.0% at Mach 3.96 and 4.63. The body alone lifts 3.9 to 4.6 per radian there, fitted
  the same way, against hpr's 2.3 to 2.8 with its body lift. Slender-body theory's nose (2 per
  radian) and boattail (−1.15 here) don't change with Mach, and the real body's lift grows. The
  CP stays within 0.19 calibers, so the stability margin holds; the slope, and so the weathercock
  rate, is low. M1.8e takes this on. The fins' agreement is uncertain by about its own size: the
  design leaves out the strip where each fin's root follows the boattail below the cylinder, about
  0.32 in² of 5.8 in² (5.5%).
- *Mach 0.6 passes by cancelling errors:* hpr's body is 40% (short) and 34% (long) above the
  fins-off readings and its fins' share 9.3% and 3.9% below the measured one.
- *Transonic, Mach 0.8 to 1.2.* Fins on less fins off, the measured fin lift falls from Mach 0.6
  to 0.9 (9.5, 8.5, 7.8 per radian on the short model) and jumps at 1.0 (14.2); hpr's rises by
  Prandtl–Glauert and then along the join to linear theory's peak at `M_s = 1.2` (16.6 against
  13.0 measured there). The long model's CP jumps forward at Mach 1.0 (71.5% of its length
  against 75.6% at 0.8), 2.29 calibers from hpr's. These are the flows no closed-form method
  covers. The transonic body readings are poorly determined (±0.02 per point on a coarse grid).
- *RASAero II.* Its subsonic slope and CP stay constant to Mach 0.9; hpr's rise by Prandtl–Glauert,
  14.4% by Mach 0.8, and its fins' CP starts moving aft at 0.8, so the CP misses from Mach 0.8 to
  0.95 by 0.52 to 0.95 calibers. The wind tunnel sides with neither: at Mach 0.8 the short model's
  `C_Nα` is +11.9% in hpr against the measurement, and the long model's +12.0%. At Mach 1.3 hpr is
  just past its join's peak (+12.7%, CP +0.65); at Mach 2, −16.8% with the CP 0.56 calibers forward.
  Calisto has no fins-off data, so that miss can't be split into fins and body; the wind
  tunnel's gap at the same speeds is the body's.

Prometheus 2022, the suite's supersonic rocket, flies through Mach 1.010 on its declared drag:
14 metrics scored and passing, the largest its apogee at +1.525%. Its drifts are −9.273% (apogee)
and +6.283% (landing). `wind_response.py`, which now flies it, puts RocketPy with hpr's rail
release and body lift at 1484.3 m and 1468.5 m, within 0.1% of hpr's 1483.1 m and 1469.4 m. So
they are ADR-026's body-lift difference and are reported, not scored, as ADR-026's are. Near
Mach 1 it flies almost straight into the airflow (its angle of attack, sampled at hpr's steps by a
local probe of the harness, stays below 0.11° from Mach 0.8 to 1.2), so the transonic normal force
has little to act on. Its apogee, +1.525%, is body lift too: RocketPy with hpr's body lift and rail
release reaches 3735.4 m, against hpr's 3735.3. On its own drag it stays a known gap until M1.8b.
No other case passes Mach 0.8, so no other number in the report moved.

**Rejected.**

- *Tuning the join or `M_s` to the wind tunnel.* The misses would shrink by fitting the model to
  its own check. `M_s` and the join's start come from the sources' regions, set before measuring.
- *Niskanen's one-face slope and his quintic CP fit from Mach 0.5.* The first is half of linear
  theory; the second would move every subsonic flight's CP from Mach 0.5 with no measurement
  asking for it (the Arcas Robin's CP moves forward, not aft, from Mach 0.6 to 0.8).
- *RocketPy's Diederich slope past Mach 1.* About π/2 times linear theory.
- *The third-order Busemann terms.* See Decision.

**Consequences.**

- Flights on a drag table fly to Mach 5. On hpr's own drag they still stop at Mach 1 until M1.8b.
- M1.8e: the body's supersonic normal force, against the Arcas Robin's fins-off measurements.
- M1.8c's roll forcing and damping will use the same outline and its Mach cone; TN D-4014 Fig. 14
  (roll effectiveness per degree of cant, recorded in the digitization but not yet committed)
  and TN 2114's roll-damping formulas are its references.
- `AeroModel::component_station_m` and `FinSetAero` changed shape (`fin: FinAero`, `fore_station_m`,
  `cp_station_m(mach)`); nothing outside the workspace uses them yet.
