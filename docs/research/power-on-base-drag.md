# Base drag while the motor burns

**In short: below Mach 1 the measured base drag with a jet stays near its power-off value on the whole base, and HPR
Sim's rule, which takes the motor case's area off the base, reads it low by about the whole power-off value on a
minimum-diameter rocket.** In every cold-jet tunnel test of a flat-based cylinder read here, a jet near ambient exit
pressure *lowers* the pressure on the ring of base around it (the annulus), and the deeper suction roughly makes up for
the area the jet takes. From Mach 0.6 to 0.9, for exit-to-base diameter ratios of 0.25 to 0.75, keeping the power-off
value on the whole base is within −0.06 to +0.02 (coefficient on the base area); Niskanen's rule, the power-off value
on the annulus only, reads low by 0.02 to 0.09. Near Mach 1.1 both mostly read low, by up to 0.11 and 0.19. From Mach
1.6 up the data move toward the annulus rule; a hot rocket jet raises annulus pressure by about 0.08 at Mach 1.9. The
tools split three ways: the whole base (OpenRocket 24.12, and every tool with no nozzle exit entered), the base less
the exit (OpenRocket's development line, RASAero II and Missile DATCOM, with an exit entered), and the base less the
motor case (HPR Sim alone). No test matches HPR boost (a hot jet, exit Mach near 3, exit pressure near ambient,
subsonic), so the subsonic hot-jet effect is unbounded. Plot readings carry their read-off error; "Inference" marks
this note's arithmetic. Sources read 2026-10-17.

It serves M1.14f's "power-on base drag validated" item, Cavour's open power-on miss ([aero](../physics/aero.md),
"Power-on") and #222, building on [cylinder base drag](cylinder-base-drag.md) and [nozzle exits](nozzle-exits.md).

## 1. What an HPR jet looks like (Inference)

- **Diameter ratio.** AeroTech's drawn exits are 0.21 to 0.71 of the case diameter ([nozzle exits](nozzle-exits.md)
  §3). In an airframe a few mm over the case (minimum diameter) that is about 0.19 to 0.68 of the base diameter, an
  annulus of 54% to 96% of the base. In 4 in and 6 in airframes it is about 0.07 to 0.47 (annulus 78% or more).
- **Exit pressure at sea level,** from each reload's average thrust, exit and area ratio (γ 1.2 to 1.25, thrust
  efficiency 0.92 to 1.0): exit Mach 2.2 to 3.2; exit-to-ambient 0.54 to 1.53 at area ratios 4 to 7.5 (seven reloads,
  H to M), 2.5 to 3.9 at 2.3 to 3.1 (two). The ratio grows as the air thins with height.
- **Thrust coefficient** (thrust over q∞ times base area): 13 to 50 at Mach 0.3, 1.5 to 5.5 at 0.9, 0.5 to 2.0 at 1.5.

## 2. What was measured

C_pb = (p_b − p∞)/q∞ throughout; negative is suction. Base drag on an area is −C_pb times that area over the reference.

### 2.1 Tunnel tests on cylinders, cold air

- **Cubbage 1956, Mach 0.6 to 1.25:** held from upstream (no sting), turbulent layer about 0.20 base diameters,
  jets near 70 °F, one rim orifice [1] pp. 2–7; Mach 1.0 values may read too low (no wall correction) [1] pp. 6–7.
- **The jet lowers base pressure at practical ratios.** It was "in general, detrimental" below a total pressure ratio
  of about 5 [1] p. 1; at d_j/d_b 0.65 it beat no-jet only from about 6 to 7 at Mach 1.0 to 1.25, and not within the
  tested range (to about 6) at Mach 0.6 and 0.9 [1] Fig. 7(a), p. 27. The crossover rises as the exit shrinks [1] p. 13.
- **By diameter ratio** (Fig. 13, pp. 43–44, read ±0.01), total pressure ratio 2 (sonic exit about matched): at Mach
  0.9, d_j/d_b 0.25, 0.45, 0.65 and 0.75 give −0.16, −0.20, −0.28 and −0.30, against −0.14 with no jet; at Mach 1.1,
  −0.25, −0.41, −0.54 and −0.57, against −0.22.
- **A matched Mach 2 jet,** d_j/d_b 0.75 [1] Fig. 10(b), p. 35, read ±0.01: −0.32 at Mach 0.6, −0.285 at 0.9, −0.52
  at 1.0 and −0.43 at 1.1, against −0.12, −0.145, −0.18 and −0.22 with no jet [1] Fig. 11(c), p. 38.
- **Bromm and O'Donnell,** d_j/d_b 0.75, exit Mach 2.5 to 3.5, tripped layer [3] Fig. 4, pp. 15–17, read ±0.02. At
  exit-to-ambient 1.0 with 10° nozzles: −0.31 at Mach 1.62, −0.23 at 1.94 and −0.14 at 2.41, against about −0.15,
  −0.15 and −0.13 jet off (Fig. 3). A 20° nozzle reads about 0.07 higher at Mach 1.94 and 2.41.
- **Reid and Hastings at Mach 2,** exit Mach 2, d_j/d_b 0.2 to 0.8, redrawn in MIL-HDBK-762 Fig. 5-146 [4] p. 5-214,
  read ±0.01 in p_b/p∞. Jet off, p_b/p∞ ≈ 0.60 (C_pb −0.14). The minimum, about 0.18 to 0.20 (C_pb −0.29), is nearly
  the same for every diameter ratio. At exit-to-ambient 1.0: C_pb −0.23, −0.26, −0.28 and −0.20 for d_j/d_b 0.8,
  0.6, 0.4 and 0.2. No curve climbs back to the jet-off value by exit-to-ambient 2.2.

### 2.2 Jet Mach number and temperature

- **Jet Mach number drops out at a fixed static pressure ratio.** Exit Mach 1.0 to 3.2 plotted against exit-to-ambient
  static pressure spread by at most 0.030, at Mach 1.91 [2] p. 19, Figs. 32–33.
- **Hot rocket jets raise base pressure, supersonic.** A propane-oxygen rocket, core near 4,200 °R [2] p. 5, gave
  curves parallel to cold air's, about 0.08 higher [2] p. 21; Fig. 40(a), Mach 1.91, d_j/d_b 0.5 to 0.71, 5.63°
  boattail: +0.07 to +0.09. The ratio of specific heats alone does not explain it [2] Fig. 41.
- **Henry and Cahn's cylinders, Mach 0.8,** sonic jets at total pressure ratio about 2 [5] Fig. 4(c), (g), pp. 16, 20,
  read ±0.01: cold −0.19 at d_j/d_b 0.248 and −0.22 at 0.351, against −0.13 and −0.135 with no jet: deeper than
  Cubbage's at the same size. Jets at 800 and 1,200 °F read +0.01 to +0.02 and +0.04 higher. None is supersonic or as
  hot as a solid motor's jet. Salmi's subsonic cylinder agrees on the sign [9] p. 1.

### 2.3 Flights with rocket motors burning

- **Hermes A-3A model, cordite sustainer,** d_j/d_b 0.63 (Inference), jet vanes in the jet [7] pp. 4–7. An
  overexpanded jet (exit 0.8 of ambient) raised subsonic base drag by about 100%, 15% of total drag [7] pp. 1, 14.
  Fig. 22 (±0.01): annulus C_pb −0.13 on against −0.045 off at Mach 0.7. Underexpanded: +50% at Mach 1.1 [7] p. 15.
- **Purser, Thibodaux and Jackson** [6], boattailed finned bodies, sustainer burning. Model C at Mach 1.23 and 1.36:
  −0.26 and −0.31 on against −0.09 and −0.10 off (Fig. 6, p. 23; Inference from psia curves, ±0.02). Model D near
  Mach 0.8, exit about 2.0 times ambient: +0.12 on against +0.02 off (Fig. 7, p. 24; ±0.03).
- **Gillespie's rocket models:** annulus ratios 0.61 and 0.70 gave negative jet increments, 0.24 and 0.37 positive,
  and 30° nozzles sat above 9° ones [8] p. 3.
- **Hoerner** reads full-scale rockets near Mach 2 as about 0.1 power-off, −0.2 on the annulus power-on [10] p. 16-10.

## 3. The rules in use

- **HPR Sim** applies Niskanen's eq. 3.94 (0.12 + 0.13M² below Mach 1, 0.25/M above) to the aft base less the burning
  motors' area, floored at 0 (`base_drag_coefficient`, `DragConditions::thrusting` in `crates/hpr-aero/src/drag.rs`).
  The area is the case's: π(d/2)², d the mounted motor's `diameter_m`, its case outer diameter
  (`crates/hpr-sim/src/dynamics.rs`, line 208). It holds the nozzle exit radius but doesn't use it for base drag.
- **Niskanen's thesis** subtracts "the area of the thrusting motors", so a base the motor's size has no base drag
  [12] pp. 50–51. It cites Fleeman's *Tactical Missile Design* and doesn't say case or exit.
- **OpenRocket 24.12 keeps the whole base.** The project's probe found the power-on base drag equal to the whole
  base's on all 42 flights of one data branch ([aero](../physics/aero.md), "OpenRocket's rule, for comparisons").
- **OpenRocket #3244** (merged 2026-08-25, after 24.12) subtracts the nozzle exit area while a motor fires, only when
  an exit diameter is entered; the default, 0, gives no correction (comment of 2026-08-25; the description still
  says the case) [13]. On 2026-08-14 a reviewer cited an M1350 (75 mm case, 47 mm exit), and the author wrote that
  Fleeman uses the exit area and that the case takes about 2.54 times too much [13].
- **RASAero II** takes the exit diameter "for calculating the change in base drag" under power; at its default of 0,
  power-on drag equals power-off; the exhaust "helps to pressurize the base area" [14] p. 14. Neither manual gives an
  equation; a small exit gives "little difference" subsonic [15] p. 121.
- **Missile DATCOM** [16]: `DEXIT` 0 gives the full base drag, the exit diameter "base drag of annulus" (Table 5,
  p. 29). Its optional base-jet model runs above Mach 1.2 (p. 11); its input ranges are Mach 2 to 5 and exit-to-base
  0.80 to 0.95, extrapolated outside (Fig. 2, p. 32). HPR exits and speeds lie outside them (Inference).
- **Brazzel and Henderson** (1966), read only as a US patent transcribes them [17] eqs. 101–105, change the base
  pressure itself, from the jet momentum flux ratio and jet temperature ratio. Their data start at Mach 1.5.

## 4. The rules against the measurements (Inference)

Whole-base drag on the base area is −C_pb,on × (1 − (d_j/d_b)²). The "annulus rule" keeps the same test's power-off
C_pb on the annulus (Niskanen with the exit); "whole base" keeps it on the full base (OpenRocket 24.12). Using each
test's own power-off value isolates the jet's effect from eq. 3.94's own error. Signed error is rule minus measured.

| Source | Mach | d_j/d_b | Measured | Annulus rule | Whole base |
|:--|--:|--:|--:|--:|--:|
| [1] sonic, ratio 2 | 0.9 | 0.25–0.75 | 0.131–0.162 | −0.081 to −0.019 | −0.022 to +0.009 |
| [1] sonic, ratio 2 | 1.1 | 0.25–0.75 | 0.230–0.327 | −0.185 to −0.023 | −0.107 to −0.010 |
| [1] Mach 2 jet, matched | 0.6–1.2 | 0.75 | 0.123–0.228 | −0.149 to −0.059 | −0.047 to +0.035 |
| [5] sonic, ratio 2 | 0.8 | 0.25–0.35 | 0.178–0.193 | −0.075 to −0.056 | −0.058 to −0.048 |
| [3] exit Mach 2.5–3.5 | 1.62–2.41 | 0.75 | 0.061–0.136 | −0.070 to −0.004 | +0.014 to +0.069 |
| [4] Reid and Hastings | 2.0 | 0.2–0.8 | 0.082–0.237 | −0.117 to −0.031 | −0.094 to +0.061 |
| [2] hot sonic jet, ratio 2.5 | 1.91 | 0.5 | 0.098 (cold 0.150) | −0.004 | +0.027 |

- **Through Mach 0.9 the whole base fits better.** It is within −0.06 to +0.02 across d_j/d_b 0.25 to 0.75 (low by
  0.05 to 0.06 on Henry and Cahn's small exits); the annulus rule reads low by 0.02 to 0.09, most for larger exits.
  The Hermes flight (overexpanded) agrees on which is closer (whole base low by 0.034); Purser's model D
  (underexpanded, 2.0 times ambient) goes the other way: rules high by about 0.05 on the base area.
- **Mach 1.0 to 1.2: both mostly read low,** the annulus rule by up to 0.185, the whole base by up to 0.107; the
  matched Mach 2 jet at Mach 1.2 has the whole base 0.035 high. Cubbage warns Mach 1.0 may read low [1] pp. 6–7.
- **Mach 1.6 to 2.4: the annulus rule closes in** for d_j/d_b 0.75 (−0.004 at Mach 2.41). With the hot-jet shift of
  about +0.08 it fits within about 0.03 at Mach 1.9 to 2.0 for d_j/d_b 0.5 to 0.8.
- **HPR Sim's case rule** reads lower than the annulus rule in every row: on a minimum-diameter rocket it leaves
  almost no base, so it reads low by nearly the whole power-off value, 0.12 to 0.18 at Mach 0.6 to 1.0.

## 5. A worked example (Inference)

A flat-based cylinder; C_D on the base area; the power-off value from eq. 3.94; annulus rules are C_b(1 − r²), r the
exit or case diameter over the base's. Script: `refs/research/power-on-base-drag/b-scripts/compare.py`.

| Rule (diameter ratio) | Mach 0.5 | Mach 0.9 | Mach 1.5 | Mach 2.0 |
|---|---:|---:|---:|---:|
| Whole base (OpenRocket 24.12; any tool at exit 0) | 0.153 | 0.225 | 0.167 | 0.125 |
| Exit annulus, exit 0.4 | 0.128 | 0.189 | 0.140 | 0.105 |
| Exit annulus, exit 0.6 | 0.098 | 0.144 | 0.107 | 0.080 |
| HPR Sim, case 0.75 | 0.067 | 0.099 | 0.073 | 0.055 |
| HPR Sim, case 0.95 (minimum diameter) | 0.015 | 0.022 | 0.016 | 0.012 |

At minimum diameter the base is nearly the reference area, so these move the rocket's C_D almost one for one. At Mach
0.9 HPR Sim's case rule takes 0.203 off the whole-base value; a 75 mm reload with a 0.61 exit (one of AeroTech's
drawn nozzles) would keep 63% of the base drag, against HPR Sim's 10%.

Brazzel and Henderson's correlation, at an exit 0.4 of the base, exit area ratios 3.1 to 7.5, an assumed exit
pressure 1.0 to 1.5 times ambient and jet γ 1.2, gives 0.295 to 0.338 at Mach 1.5: 1.8 to 2.0 times the whole base.
Four of its six cases exceed the momentum flux ratio of 0.5 the patent holds it to. Its p_b/p∞ of 0.37 to 0.45 sits
below the 0.74 that eq. 3.94 implies power-off, so part of that gap is eq. 3.94's.

## 6. What it means for the project's numbers

- **Cavour's power-on miss.** Its RASAero power-on table is within 0.001 of power-off from Mach 0.16 up; HPR Sim's
  miss is −8.3% with no relief, −18.3% at the 67 mm diameter used, −20.8% at 75 mm ([aero](../physics/aero.md),
  "Power-on"). The relief is 10 of the 18.3 points; §4's subsonic data side with no relief (Inference).
- **The supersonic private flight `C06/1`** moves about 24 points between OpenRocket's whole-base rule and HPR Sim's
  (#222). §4 says neither end is right at low supersonic speed: near Mach 1.1 both read low, the case rule most.
- **Sign.** Too little base drag under power raises the simulated apogee: safe against a waiver ceiling, flattering
  for an altitude goal or a motor choice. RocketPy computes no base term; it flies given curves [18].

## What this suggests

1. Take the nozzle exit, not the case, off the base under power when the exit is known (motor files, [nozzle
   exits](nozzle-exits.md)); with no exit, keep the whole base, as every other tool does.
2. Better, below Mach 1 keep the power-off value on the whole base under power, which §4 finds within −0.06 to
   +0.02 of the data; use the exit annulus only from about Mach 1.6, where the data and the hot-jet shift support it.
3. Record the jet effect near Mach 1.0 to 1.2 as a known gap: both rules read low there, by up to 0.11 to 0.19.
4. Re-run Cavour's power-on comparison and `C06/1` under the chosen rule, and pin the rule with tests at a
   minimum-diameter and a large-airframe base.

## Open questions

- No test of a hot, Mach 3, near-matched jet in a subsonic stream at HPR thrust coefficients was read. MIL-HDBK-762
  Fig. 5-150 covers one, but the cached scan's thrust axis has no numbers; Craven's [11] was read in abstract only.
- Why Henry and Cahn's small exits dip deeper than Cubbage's of the same size at Mach 0.8 to 0.9 is not known.
- Brazzel's reports and paper and DATCOM's plume report are on DTIC, down on 2026-10-17. Fleeman wasn't read; that he
  uses the exit rests on #3244's author. RASAero II's equation is unpublished. Fins at the base, motor-tube cavities
  and afterburning were not separated from the jet effect.

## Sources

NACA reports: US government works; MIL-HDBK-762: Distribution A; DATCOM manual: public release; the rest cited only.

1. J. M. Cubbage, Jr., NACA RM L56C21, 1956. <https://ntrs.nasa.gov/citations/19930089150>
2. L. E. Baughman, F. D. Kochendorfer, NACA RM E57E06, 1957. <https://ntrs.nasa.gov/citations/19930089763>
3. A. F. Bromm, Jr., R. M. O'Donnell, NACA RM L54I16, 1954. <https://ntrs.nasa.gov/citations/19930088494>
4. MIL-HDBK-762(MI), 1990. <https://archive.org/details/MILHDBK762DesignOfAerodynamicallyStabilizedFreeRockets>
5. B. Z. Henry, Jr., M. S. Cahn, NACA RM L56G12, 1956. <https://ntrs.nasa.gov/citations/19930089293>
6. P. E. Purser, J. G. Thibodaux, H. H. Jackson, NACA RM L50I18, 1950. <https://ntrs.nasa.gov/citations/19930086428>
7. H. H. Jackson, NACA RM SL55F15, 1955. <https://ntrs.nasa.gov/citations/20010057623>
8. W. Gillespie, Jr., NACA RM L51J29, 1951. <https://ntrs.nasa.gov/citations/19930086791>
9. R. J. Salmi, NACA RM E54I13, 1954. <https://ntrs.nasa.gov/citations/19930088483>
10. S. F. Hoerner, *Fluid-Dynamic Drag*, 1965, p. 16-10. <https://archive.org/details/FluidDynamicDragHoerner1965>
11. A. H. Craven, College of Aeronautics Rep. 129, 1960. <https://reports.aerade.cranfield.ac.uk/handle/1826.2/4803>
12. S. Niskanen, OpenRocket thesis, 2009, pp. 50–51. <https://openrocket.sourceforge.net/thesis.pdf>
13. OpenRocket pull request #3244, description and comments. <https://github.com/openrocket/openrocket/pull/3244>
14. Rogers, Cooper, *RASAero Users Manual*, 2011, p. 14. <http://rasaero.com/dloads/RASAero%20Users%20Manual.pdf>
15. Rogers, Cooper, *RASAero II Users Manual*, 2019. <https://rasaero.com/dloads/RASAero%20II%20Users%20Manual.pdf>
16. W. B. Blake, *Missile DATCOM User's Manual, 1997 Fortran 90 Revision*, AFRL-VA-WP-TR-1998-3009, pp. 11, 29, 32.
17. F. G. Moore, T. C. Hymer, US 6,721,682 B1, 2004, eqs. 101–105. <https://patents.google.com/patent/US6721682B1/en>
18. RocketPy `Rocket` reference (MIT). <https://docs.rocketpy.org/en/latest/reference/classes/Rocket.html>
