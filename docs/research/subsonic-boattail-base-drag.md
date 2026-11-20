# Base drag behind a boattail below Mach 0.8: Hoerner's rule on the Hermes A-3A

**In short: on the Hermes A-3A's measured power-off base drag below Mach 0.8, Hoerner's subsonic rule, which scales
with the base diameter cubed and with the forebody's friction, reads high by 0.0120 to 0.0163 on the largest
cross-section, where HPR Sim's rule before M1.14i3 read high by 0.0503 to 0.0638. Both are past the report's stated
error and the reading together, ±0.006. M1.14i3 took Stoney's relief behind a boattail (§3; ADR-362): HPR Sim now reads
high by 0.0251 to 0.0348.** The numbers are the `hoerner_power_off` rows of
`validation/fixtures/aero/base-drag-under-power.json` (`cargo xtask aero`). This note is for
[M1.14i3](../decisions-and-roadmap.md#m1-14i3) (ADR-359 decision 3): which cited rule the base term below Mach 0.8
changes to, if any. Source read 2026-11-20.

## 1. The rule

Hoerner, *Fluid-Dynamic Drag* (1965), p. 3-19, gives the base drag of a body of revolution with a turbulent boundary
layer as C_DB = 0.029 / √C_DfB on the base area (eq. 34), with C_DfB the forebody's drag over the base area. Referred
to the largest cross-section, with C_Df the forebody's drag on that area, it becomes ΔC_D = 0.029 (d_B/d)³ / √C_Df
(p. 3-20, eq. 35). There Hoerner adds that a moderate boattail (d_B/d about 0.9, about 5°) lowers a projectile's drag
somewhat, and that for larger taper angles or thick boundary layers "the maximum cross-section area of the body must
be considered as base". The rule has no Mach number in it: it is a low-speed correlation. USAF DATCOM (1978,
§4.2.3.1, eq. 4.2.3.1-b) gives the same rule for a body's subsonic base drag, citing Hoerner, with C_Df the body's
zero-lift drag less its base (its eq. 4.2.3.1-a's first term).

HPR Sim's rule below Mach 1 is (0.12 + 0.13 M²) on the base area (Niskanen 2009, eq. 3.94), so on the largest
cross-section it scales with (d_B/d)², and it takes no account of the forebody. Before M1.14i3 (ADR-362), below
Mach 0.8 HPR Sim gave a boattail no relief on its base.

## 2. On the Hermes A-3A

The model is 8 in across, with a 4.5° cone to a 4.8 in base: d_B/d = 0.6. C_Df is HPR Sim's own drag of the body less
its base, coasting, 0.0632 to 0.0646 on the largest cross-section (`hpr_forebody_drag`; its pressure part is zero
here), so the rule gives 0.0247 to 0.0249.

| Mach | measured, power off | HPR Sim before M1.14i3 | Hoerner | HPR Sim before less measured | Hoerner less measured |
|--:|--:|--:|--:|--:|--:|
| 0.65 | 0.0127 | 0.0630 | 0.0247 | +0.0503 | +0.0120 |
| 0.70 | 0.0105 | 0.0661 | 0.0247 | +0.0556 | +0.0142 |
| 0.75 | 0.0096 | 0.0695 | 0.0248 | +0.0599 | +0.0152 |
| 0.79 | 0.0086 | 0.0724 | 0.0249 | +0.0638 | +0.0163 |

Its mean absolute error over the four rows is 0.0144, against HPR Sim's 0.0574 (Inference: the means of the table's
columns). A test holds that Hoerner's rule reads high past ±0.006 on every row, by less than a third of HPR Sim's
(`the_power_off_rule_reads_high_the_flown_power_on_rule_low`; the old rule's column is `flat_base_rule_power_off`).

## 3. What is open for M1.14i3

- **Straight bases.** With d_B/d = 1 the rule gives 0.029 / √C_Df: 0.065 at C_Df = 0.2 and 0.046 at 0.4, below HPR
  Sim's 0.12 to 0.20 from Mach 0 to 0.8 (Inference: the two rules' arithmetic). Flat bases measured with no sting
  read 0.12 to 0.16 on the base area from Mach 0.6 to 0.9 ([cylinder base drag](cylinder-base-drag.md)); for eq. 34 to
  give that, C_DfB would be 0.033 to 0.058 (Inference: (0.029/0.16)² to (0.029/0.12)²), too low for a forebody
  several diameters long. So on flat bases the rule reads low there. Changing every base on one boattailed
  vehicle's flight would move every flight's apogee up, on a rule that reads low on the flat bases measured.
- **The relief alone.** HPR Sim's flat-base rule with Hoerner's dependence on the base's diameter, (d_B/d)³ on the
  largest cross-section where HPR Sim's is (d_B/d)², leaves flat bases as they are and multiplies a boattailed base's
  value by d_B/d (`hpr_power_off` since M1.14i3). On this model that is 0.0378 to 0.0434, high by 0.0251 to 0.0348: a mean
  absolute error of 0.0303 against HPR Sim's 0.0574 and Hoerner's 0.0144 (Inference: the fixture's rows).
- **A source for the relief.** Stoney's NASA TR R-100 (1961, printed p. 11) gives, from wind-tunnel data on conical
  afterbodies, C_D,b = C_D,cylinder base (r_b/R)³ on the largest cross-section: the relief above, exactly. He warns
  that it "does not account for the possibility of negative base drags" at subsonic speeds. Missile DATCOM's manual
  names "NASA method, NASA-TR-R-100" for a boattail's base drag below Mach 1.2 (1998, Table 27, printed p. 97).
- **Measured relief may be larger.** Read by eye in a research pass and not yet reduced: behind 5.6° cones of d_B/d
  0.42 to 0.75 held from upstream, Salmi's jet-off base pressures at Mach 0.9 are near zero (NACA RM E54I13, Fig. 8);
  Stoney's free-flight body of d_B/d 0.44 shows a base pressure above ambient from Mach 0.6 to 0.85 without its sting
  (NACA RM L51L03, Fig. 4). Each would read the relief above as high, as the Hermes flight does.
- **The relief in the code,** since M1.14i3 (ADR-362; the factor behind a boattail below Mach 0.8 set to d_B/d₁, the
  transonic ramp starting there). Of `hpr-aero`'s tests, one hand pin of the old base drag fails, one fixture's
  informational base column goes stale, and the code-to-code checks against RASAero II curves (ADR-009, ADR-029)
  move:
  - At Mach 0.3, two more of RocketPy's example rockets fall outside 10% of their curves: Calisto's getting-started
    variant and Juno III, both power off, beside the three ADR-009 explains.
  - Rows within 10%: Calisto's own export, the one real RASAero II file, goes from 3 to 5 of 7 transonic and keeps
    15 of 15 subsonic; the getting-started variant falls from 12 to 8 of 15 subsonic, and Juno III from 15 to 12.
  - The Arcas Robin tunnel comparison holds the forebody, the drag less its base, and does not move.
  - Whether RASAero II relieves a boattailed base below Mach 0.8 is not known here: these checks size a difference
    between codes, not which is right.
- **No Mach number.** HPR Sim's 0.13 M² rise has no counterpart in eq. 35; the Hermes curve falls with Mach number
  below 0.8 where both rules rise or stay level.
- **Under power** the measured base drag is 2.1 to 2.4 times its power-off value; neither rule here covers the jet.
- Each choice rests on this one vehicle; no number may be fitted to the collection's flights (ADR-346, decision 6).
