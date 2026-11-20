# Base drag behind a boattail below Mach 0.8: Hoerner's rule on the Hermes A-3A

**In short: on the Hermes A-3A's measured power-off base drag below Mach 0.8, Hoerner's subsonic rule, which scales
with the base diameter cubed and with the forebody's friction, reads high by 0.0120 to 0.0163 on the largest
cross-section, where HPR Sim's rule reads high by 0.0503 to 0.0638. Both are past the report's stated error and the
reading together, ±0.006.** The numbers are the `hoerner_power_off` rows of
`validation/fixtures/aero/base-drag-under-power.json` (`cargo xtask aero`). This note is for
[M1.14i3](../decisions-and-roadmap.md#m1-14i3) (ADR-359 decision 3): which cited rule the base term below Mach 0.8
changes to, if any. Source read 2026-11-20.

## 1. The rule

Hoerner, *Fluid-Dynamic Drag* (1965), p. 3-19, gives the base drag of a body of revolution with a turbulent boundary
layer as C_DB = 0.029 / √C_DfB on the base area (eq. 34), with C_DfB the forebody's drag over the base area. Referred
to the largest cross-section, with C_Df the forebody's drag on that area, it becomes ΔC_D = 0.029 (d_B/d)³ / √C_Df
(p. 3-20, eq. 35). There Hoerner adds that a moderate boattail (d_B/d about 0.9, about 5°) lowers a projectile's drag
somewhat, and that for larger taper angles or thick boundary layers "the maximum cross-section area of the body must
be considered as base". The rule has no Mach number in it: it is a low-speed correlation.

HPR Sim's rule below Mach 1 is (0.12 + 0.13 M²) on the base area (Niskanen 2009, eq. 3.94), so on the largest
cross-section it scales with (d_B/d)², and it takes no account of the forebody. Below Mach 0.8 HPR Sim gives a
boattail no relief on its base.

## 2. On the Hermes A-3A

The model is 8 in across, with a 4.5° cone to a 4.8 in base: d_B/d = 0.6. C_Df is HPR Sim's own skin friction on the
model, coasting, 0.0632 to 0.0646 on the largest cross-section (`hpr_friction`), so the rule gives 0.0247 to 0.0249.

| Mach | measured, power off | HPR Sim | Hoerner | HPR Sim less measured | Hoerner less measured |
|--:|--:|--:|--:|--:|--:|
| 0.65 | 0.0127 | 0.0630 | 0.0247 | +0.0503 | +0.0120 |
| 0.70 | 0.0105 | 0.0661 | 0.0247 | +0.0556 | +0.0142 |
| 0.75 | 0.0096 | 0.0695 | 0.0248 | +0.0599 | +0.0152 |
| 0.79 | 0.0086 | 0.0724 | 0.0249 | +0.0638 | +0.0163 |

Its mean absolute error over the four rows is 0.0144, against HPR Sim's 0.0574 (Inference: the means of the table's
columns). A test holds that Hoerner's rule reads high past ±0.006 on every row, by less than a third of HPR Sim's
(`the_flown_rule_reads_low_the_others_high_and_the_jet_raises_the_base_drag`).

## 3. What is open for M1.14i3

- **Straight bases.** With d_B/d = 1 the rule gives 0.029 / √C_Df: 0.065 at C_Df = 0.2 and 0.046 at 0.4, below HPR
  Sim's 0.12 to 0.20 from Mach 0 to 0.8 (Inference: the two rules' arithmetic).
  Changing every base on one boattailed vehicle's flight would move every flight's apogee up, and the collection
  already climbs above its logs on average. A change limited to the boattail's relief, (d_B/d)³ against (d_B/d)²,
  leaves straight bases as they are; on this model it gives 0.6 of HPR Sim's value, 0.0378 to 0.0434, still high.
- **No Mach number.** HPR Sim's 0.13 M² rise has no counterpart in eq. 35; the Hermes curve falls with Mach number
  below 0.8 where both rules rise or stay level.
- **Under power** the measured base drag is 2.1 to 2.4 times its power-off value; neither rule here covers the jet.
- Each choice rests on this one vehicle; no number may be fitted to the collection's flights (ADR-346, decision 6).
