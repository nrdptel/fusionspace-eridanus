# ADR-029: Drag against RASAero II through Mach 2: the gap by band, MIL-HDBK-762's sample calculation, and the boattail's wave drag (2026-09-18)

- **Status:** accepted
- **Summary:** Drag against RASAero II through Mach 2: the gap by band, MIL-HDBK-762's sample calculation, and the boattail's wave drag

**Context.** M1.8's drag bullet asks for `C_D` within 10% of RocketPy's RASAero curves from Mach
0.1 to 2.0 for the available rockets, with the errors by band in the report; M1.8b2 carries it
(ADR-028). ADR-009 compared the same curves at Mach 0.3 only, with one declared rule for the
inputs the curves don't record (fin section, thickness, finish). What the curves are:

- **Calisto**: a real RASAero II export (RocketPy's first commit, 2018, so version 1.0.1.0 or
  earlier), to Mach 2. The only curve traceable to a RASAero II run.
- **Juno III**: 3 decimals, hand-edited: from Mach 0.93 to 1.0 it climbs a constant 0.072 per
  0.01, then drops to 0.001.
- **Cavour**: stops at Mach 0.895 (power-off) and 0.923 (power-on).
- **Valetudo**: to Mach 1.53, 1.44 times its own rocket's OpenRocket export at Mach 0.3 (ADR-009).

RASAero II's Users Manual (1.0.2.0, 2019) cites no drag method. Its drag breakdown lists terms
Niskanen's buildup doesn't have: fin–body interference, fin base drag, and "other body wave" drag
(pp. 90, 92). Its exports take the Reynolds number at sea level (p. 84), a smooth finish by default
(p. 53), and laminar flow up to a transition at `5 × 10⁵` unless "All Turbulent" is set (p. 55).
It names eight fin sections (p. 14) and no default among them. No RASAero input file (`.CDX1`) for
any of the four rockets is public: RocketPy's history on every branch, its companion repositories,
the two teams' repositories and a GitHub search found none. So the inputs stay unknown, and the
comparison is between two codes, one of whose inputs are guessed.

**Decision.**

- **The comparison.** `cargo xtask aero` compares hpr's `C_D0` with each curve every 0.05 from Mach
  0.1 to 2.0, wherever the curve reaches without extrapolation. Each point uses USSA76 sea level's
  Reynolds number for its Mach number, as RASAero II computes its exports. ADR-009's rule for the
  inputs is unchanged, and Juno III's curve is used to Mach 0.92. Bands are Niskanen's Table 3.1:
  subsonic to 0.8, transonic below 1.2, supersonic from 1.2.
  `validation/fixtures/aero/rocketpy-drag-curves.json` records hpr's value and the error at each
  Mach number, and each band's count within 10%, least and greatest error, and RMS. It doesn't
  record the curves, which carry their own terms (ADR-009), but hpr's value and the error at full
  precision give the curve back exactly at each sampled Mach number: 147 values of five curves. That
  is more than ADR-009's one value per curve, so the maintainer's open question on RASAero values
  now covers it. The report for this comparison, as for M1.8a's and M1.8b1's, is the fixtures and
  the *Aerodynamics* and *Accuracy* pages; `validation/reports/latest.md` holds whole flights only.
- **Loft lesson L18's test measures and pins** instead of asserting agreement. It is renamed
  `hpr_aero::drag::tests::supersonic_cd_against_rasaero_tables`. It recomputes every row from the
  committed designs, checks every verdict and band summary, and pins each band's count within
  10%. The lesson named it `supersonic_cd_within_tolerance_of_rasaero_tables`; that assertion
  doesn't hold, and a changed assertion needs a decision record (`docs/research/loft-lessons.md`).
- **A second reference, with every input known: MIL-HDBK-762's sample drag calculation.** This is
  Table 5-4 (printed pp. 5-58 to 5-66) for the rocket of Fig. 5-155 (pp. 5-223 to 5-224): a
  3-calibre tangent ogive on a 21-calibre cylinder 0.16 m across, and four fins flush with the
  base. The table gives each term from Mach 0.5 to 3.2 at a flight's Reynolds numbers: friction,
  the nose's wave drag, the fins' wave and trailing-edge base drag, and the body's jet-off base
  drag. It is a calculation by the handbook's methods, not a measurement; its base drag comes
  from measured bases. The handbook is a U.S. Government work. Its values are transcribed with
  their pages into `validation/fixtures/aero/mil-hdbk-762-sample-drag.json`, checked against the
  rendered pages, with each row summing to its printed total. `cargo xtask designs` builds the
  rocket with a smooth finish (the handbook's flat-plate friction).
  - **The fins are left out.** Fig. 5-155 draws each fin as a single wedge, sharp at the leading
    edge and blunt at the trailing edge, and the calculation gives them a double wedge's wave drag
    and base drag on the trailing edge. hpr has no such section (#70). The design takes square
    edges, and the fins' pressure drag is recorded on both sides but compared on neither: a first
    draft compared the totals and read hpr high faster than sound, which was the square leading
    edge's 0.06 to 0.10 (review).
  - `cargo xtask aero` compares it term by term in `validation/fixtures/aero/drag-vs-mach.json`,
    and `hpr_aero::tests::drag_against_mil_hdbk_762_sample` pins it. The target is M1.8's 10%,
    not set blind: hpr's numbers were first computed in a scratch run while choosing references.
- **M1.8's drag bullet is not met, and the gap stays in the report.** Closing it by tuning would
  move hpr away from the references whose inputs are known. The candidates for its cause become a
  new increment, M1.8b3, for the afterbody (a boattail's supersonic wave drag and the base), with
  targets on the Arcas Robin's measured forebody and on Calisto's supersonic band, and issues for
  the rest.

**Result.**

- **By band, rows within 10%** (fixture):

  | case | subsonic (to 0.8) | transonic | supersonic (from 1.2) |
  |---|---|---|---|
  | Calisto, 2018 fins | 15 of 15, +3.9% to +8.9% | 2 of 7, −31.4% to +3.8% | 0 of 17, −29.8% to −24.4% |
  | Calisto, getting-started fins (variant) | 12 of 15, −8.2% to +30.7% | 4 of 7, −7.8% to +48.5% | 6 of 17, −1.8% to +21.6% |
  | Juno III (to 0.9) | 15 of 15, −6.2% to +9.1% | 0 of 2, +14.1% to +21.9% | — |
  | Cavour, power-off (to 0.85) | 6 of 15, −12.6% to −2.2% | 0 of 1, −12.6% | — |
  | Cavour, power-on (to 0.9) | 1 of 15, −26.2% to −9.2% | 0 of 2, −27.0% to −26.7% | — |
  | Valetudo, power-off (to 1.5) | 0 of 15, −51.4% to −43.5% | 0 of 7, −53.3% to −50.9% | 0 of 7, −54.6% to −52.9% |
  | Valetudo, power-on (to 1.5) | 0 of 15, −55.6% to −46.6% | 0 of 7, −57.6% to −54.7% | 0 of 7, −57.8% to −56.2% |

- **No plausible input closes Calisto's gap**
  (`hpr_aero::tests::calistos_supersonic_gap_survives_every_plausible_fin_and_finish`). Over
  square, rounded and airfoil fins, 2 to 6.35 mm thick, smooth or painted (20 µm), no combination
  has rows within 10% in both the subsonic and the supersonic band, and none passes 3 of 7
  transonic. Supersonic rows come within 10% only with square fins 4.76 mm or thicker (all 17 at
  6.35 mm painted), and those put every subsonic row 14.6% to 44.9% high. Every other combination
  stays 10.1% to 37.6% low supersonic. So the supersonic gap is a difference between the two
  codes' models, not the inputs.
- **Against MIL-HDBK-762's sample calculation, fins left out, hpr's body reads high through
  Mach 1 and low faster than sound.** 6 of 12 within 10%: −10.3% at Mach 0.5 and −1.1% at 0.7;
  +20.1% to +31.9% from 0.9 to 1.1 and +12.3% at 1.2; −6.0% to −9.6% from 1.6 to 3.2. By term
  (handbook against hpr):
  - Nose, Niskanen's ogive: 0.052 against 0.164 at Mach 1.0, and 0.109 against 0.234 at 1.1.
    The handbook's transonic ogive curve (Fig. 5-113) and Stoney's measured 3:1 cone (ADR-028)
    both sit well under it (#67). From Mach 2 the two agree within 10%.
  - Base, Fleeman's formula (eq. 3.94): 0.183 against 0.250 at Mach 1.0, but 0.147 against 0.125
    at Mach 2. Above Mach 1.2 the handbook's base pressure follows Love's correlation of measured
    bases (NACA TN 3819, Fig. 5-139) (#68).
  - Friction, 10.4% to 15.9% lower in hpr: its body form factor is 1.02, the handbook's 1.15,
    which accounts for about 11 points; the rest, unexplained, may be the two methods'
    compressibility corrections.
  So faster than sound hpr's body reads 6% to 10% low against a method with every input known:
  the same sign as Calisto's gap, a third of its size. NASA's Arcas Robin wind tunnel, with the
  fins off and the base left out, reads hpr high, most of it a lip at the model's base (ADR-028).
- **A candidate for the rest: the boattail's supersonic wave drag.** Calisto's boattail is short
  and steep, 0.472 calibres long, down to `(d_b/d)² = 0.469` (18.4°). hpr's boattail rule (eq.
  3.88) only scales base drag, and gives it 0.083 at Mach 1.2, 0.066 at 1.5 and 0.050 at 2.0.
  MIL-HDBK-762's chart for conical boattails at supersonic speeds (Fig. 5-122, p. 5-187, read for
  this decision) gives 0.338 ± 0.008 at Mach 1.2 and 0.215 ± 0.007 at 1.5; at Mach 2 its
  parameter `√(M² − 1)/(2 l/d)` is 1.83, past the chart's end at 1.4 (Mach 1.66 here), and
  extrapolating gives about 0.13. Against hpr's whole gap of 0.20, 0.156 and 0.128, the chart's
  extra 0.255, 0.149 and about 0.08 would overshoot at Mach 1.2, match at 1.5 and fall short at 2.
  RASAero II lists such drag as its own term, "other body wave" drag (p. 92). Against it:
  - The Mach trends differ: the gap falls by 1.6 times from Mach 1.2 to 2, the chart's extra
    over hpr's rule by 3.2 times.
  - The handbook advises boattails under 8° to avoid flow separation (p. 5-12); a separated
    18.4° boattail sees about the base's pressure, which is roughly what hpr's rule gives.
  - A boattail raises its base pressure (Fig. 5-141, only at Mach 2.5 to 3.5), which would offset
    part of the term; that is not subtracted here.
  - The one measured boattail argues against the whole term. The Arcas Robin's 15° boattail is in
    the tunnel's forebody. With the fins off the short model reads +8.1% at Mach 1.5, where hpr's
    boattail is 0.069; with the chart's 0.198 in its place it would read about +46%, or +21% with
    the whole lip removed. At Mach 2.96, with the lip removed, hpr's rule already agrees (−1.2%),
    where the chart's 0.072 would put it about +17% high.
  So the chart's term is a candidate, not a finding: the measured boattail wants more pressure
  drag than hpr's rule at Mach 1.5, less than the chart's, and about the rule's at Mach 2.96.
  M1.8b3 measures it before hpr changes.

**Consequences.**

- **M1.8b3, the supersonic afterbody**, is next:
  - The conical boattail's wave drag (Fig. 5-122 or the linear theory behind it), weighed against
    the Arcas Robin's measured boattail.
  - The base pressure behind a boattail, and the base drag faster than sound (#68).
  - The Arcas Robin's lip, which hpr takes as a shoulder in the free stream though it sits in the
    boattail's wake (ADR-028).
  - Its targets, set before measuring, are on the Arcas Robin's fins-off forebody and Calisto's
    supersonic band (`ROADMAP.md`).
- **Issues, outside M1.8b3:**
  - #67: Niskanen's transonic cone and ogive drag, against Stoney's measured cone and the
    handbook's ogive.
  - #68: Fleeman's base drag against Love's correlation.
  - #69: fin–body interference drag, which RASAero II has and Niskanen's buildup neglects (p. 41).
  - #70: a sharp double-wedge fin section, for the Arcas Robin's fins supersonic (ADR-028).
- **The maintainer's question on RASAero values is widened** to the drag sweep's 147 recoverable
  values. If Neer objects, the sweep keeps its band summaries and drops its rows' errors.
