# Base suction behind a jet near Mach 1
Re-checked 2026-11-02: 18 claims corrected.

**In short: a jet does deepen the base suction most between Mach 1.0 and 1.2; the deepest readings are cold tunnel jets
at Mach 1.0, while supersonic jets near ambient exit pressure through HPR-like cones deepen it less, and a hot jet less
again.** Sonic cold jets at total-pressure ratio 2 lower the annulus C_pb by 0.31 to 0.37 below jet-off at Mach 1.0 to
1.2 (exit 0.65 to 0.75 of the base); a second tunnel repeats it within 0.05 to 0.08. Near-matched supersonic cold jets
(exit 0.47 to 0.75) lower it by 0.00 to 0.28, except a 5° cone at Mach 1.0, 0.48 (the author suspects the tunnel there);
the cone angle alone spreads it by 0.22 at Mach 1.1. Five rocket-model flights with the motor burning (exit 0.55 to
0.64) give 0.00 to 0.20 at Mach 1.0 to 1.4, their drop near Mach 1.02 to 1.05, about 0.05 to 0.1 later than the
tunnel's. On one tunnel body a hot aluminized jet reads 0.06 to 0.09 higher than a cold jet at Mach 1.2, against −0.02
to +0.06 at Mach 0.9. For HPR exits this note's band is ΔC_pb 0 to −0.30 at Mach 1.0 to 1.2, most likely −0.10 to
−0.20. On the most-likely band the whole-base rule errs by −0.12 to +0.08 on the base area and the exit-annulus rule
reads low by up to 0.16 (full band: −0.19 to +0.12, and 0.24); HPR Sim's case rule reads lower still. Plot readings
carry their read-off error; "Inference" marks this note's arithmetic. Sources read 2026-10-29 and 2026-11-02.

It follows [power-on base drag](power-on-base-drag.md) ("parent"), which found both rules low near Mach 1.1 by up to
0.11 and 0.19, and serves #222's supersonic case C06/1 and M1.14f's power-on item. It builds on [hot jets below Mach
1](hot-jet-subsonic-base.md) (FA 22 at Mach 0.9) and [cylinder base drag](cylinder-base-drag.md) (jet off). C_pb = (p_b
− p∞)/q∞ on the annulus; r = d_e/d_b, exit over base diameter; ΔC_pb = jet-on minus jet-off C_pb (negative: the jet
deepens suction). Base drag on the base area is −C_pb(1 − r²) (the jet's own thrust apart) [4] p. 2. The whole-base rule
keeps −C_pb,off on the full base; the exit-annulus rule −C_pb,off(1 − r²).

## 1. Why the jet's suction peaks near Mach 1.0 to 1.2

- **Two opposing effects.** The jet's mixing layer pumps air out of the dead space behind the base (more suction,
  growing with jet periphery and mass flow); the plume displaces the outer stream, and the compression where they meet
  raises base pressure [3] pp. 5, 9; [1] pp. 8–9; [5] p. 6. Small exits and low pressure ratios sit on the pumping side,
  large exits and high ratios on the other [3] p. 9; [1] p. 11. At Mach 0.9 the plume's compression reaches the base
  only above total-pressure ratio about 4 [1] p. 8, so near-matched jets sit at the pumping end.
- **The drop sits between Mach 0.9 and 1.0.** A sonic jet at total-pressure ratio 2 takes the annulus from −0.27 (r
  0.65) and −0.31 (0.75) at Mach 0.9 to −0.54 and −0.55 at Mach 1.0, and stays at −0.52 to −0.55 to Mach 1.2; jet-off
  goes −0.14, −0.18, −0.21 [1] Figs. 11(a), 11(b), pp. 36–37 (pixel read ±0.01). ΔC_pb: −0.13 and −0.17 at 0.9, −0.37 at
  1.0, −0.31 to −0.33 at 1.1 to 1.2 (Inference).
- **Why there (Inference, built on cited accounts).** Hoerner's jet-off transonic wake neck is "open": fluid flows back
  from behind the wake shock and holds base pressure up; rim expansion and pumping take over once it is "sealed" [13]
  pp. 16-5, 16-6. A jet seals the neck, and the outer stream, sonic at the rim, expands into the low pressure the jet
  pumps. The trailing shock off the plume strengthens with Mach, so base pressure recovers above the peak [3], [1] p. 8.
- **Nozzle cone angle.** A wider cone turns the outer flow nearer the base and raises base pressure [1] p. 10. A Mach 2
  jet (r 0.75) at H_j/p∞ 8 exits near ambient [1] p. 11 (isentropic p_e/p∞ 1.02, Inference); at Mach 1.1 it reads −0.48,
  −0.40, −0.26 for 5°, 12°, 25° cones, jet-off −0.22 [1] Fig. 11(c), p. 38 (±0.02), a 0.22 spread. At Mach 1.0 the 5°
  reads −0.66 against −0.17 to −0.18 (ΔC_pb −0.48), the 12° −0.45 (tunnel suspected); a 30° flight nozzle flips it (§4).

## 2. How much of the dip could be the tunnel

- **The author's warning.** At Mach 1.0 the base expansion may reflect from the slotted wall, so base pressure "may be
  excessively low", size undetermined [1] pp. 6–7, 10; the later report holds magnitude and trend valid [2] p. 7.
- **Blockage (Inference from stated sizes):** Cubbage 3.1% (his 2 in models, same tunnel: 3.08% [2] pp. 4–5, 11); Nelson
  and Scott, Scott and Slocumb 3.9% [4] p. 3, [3] p. 2; MA 10F 2.8% with strut [6] p. 12; Henry, Cahn 0.3% [5] p. 9.
- **Large exits, two rigs.** Nelson and Scott's sting-held model, r 0.625, sonic, ratio 2: C_D,b 0.183, 0.282, 0.284 on
  the base at Mach 0.88, 0.98, 1.19 [4] Fig. 4(a), p. 17 (±0.01), so C_pb −0.30, −0.46, −0.47 (Inference); Cubbage at r
  0.65: −0.27, −0.54, −0.52. The drop repeats, 0.08 and 0.05 shallower at Mach 1.0 and 1.2; the authors call their
  effect "generally smaller" than others' and name blockage among the causes [4] p. 5.
- **Small exits, two rigs.** At Mach 1.0 the 0.3% tunnel reads −0.23 at r 0.248 and 0.351 [5] Figs. 4(c), 4(g), pp. 16,
  20 (±0.01), against −0.20 and −0.22 at r 0.225 and 0.32 in the 3.9% rig [3] Figs. 10(b), 3(b), pp. 35, 16 (±0.02). The
  low-blockage tunnel is not shallower: no sign of wall-made suction at small exits.
- **Not bounded.** Solid walls changed MA 10F's pressures at Mach 1.2 and 1.46, not 0.9 [6] pp. 13–14; high blockage
  raises the local Mach number near the lip [14] §1.4.5. No large-exit (r ≥ 0.6) test in a low-blockage tunnel was
  found, so the wall bias there near Mach 1.0 is unbounded.

## 3. Henry and Cahn's small exits

Sonic cold jets at total-pressure ratio 2; annulus C_pb, jet-off in brackets; read ±0.01 (Henry and Cahn, pixel grids)
and ±0.02 (Scott and Slocumb, by eye); Cubbage as in the parent's §2.1:

| Mach | Henry, Cahn r 0.248 / 0.351 [5] | Cubbage r 0.25 / 0.45 [1] Fig. 13 | Scott, Slocumb r 0.225/0.32/0.45 [3] |
|--:|--:|--:|--:|
| 0.9 | −0.20 / −0.23 (−0.15) | −0.16 / −0.20 (−0.14) | −0.18 / −0.19 / −0.21 |
| 1.0 | −0.23 / −0.23 (−0.17 / −0.14) | not plotted | −0.20 / −0.22 / −0.28 |
| 1.1 | −0.30 / −0.30 (−0.23 / −0.20) | −0.25 / −0.41 (−0.22) | r 0.225: −0.23 |

- **The gap is rig scatter.** At Mach 0.9 Henry and Cahn read 0.02 to 0.05 deeper than both small rigs; at 1.0 they
  agree with Scott and Slocumb within 0.03; at 1.1 they sit 0.05 deeper than Cubbage at r 0.25 (Inference).
- **Candidate causes.** (a) Cubbage's small exits were 25° convergent nozzles, whose lip boundary layer makes the exit
  act smaller than drawn [1] pp. 5, 10: right sign, unsized. (b) The small rigs ran a turbulent layer about 0.20 base
  diameters thick [1] p. 7; [4] p. 4; a 5 in body likely carries a thinner one, and base drag falls as the layer
  thickens [13] p. 16-6: right sign, unsized. (c) Scott and Slocumb's two taps sit at one radius, 90° apart [3] Table I,
  p. 12; they part by up to 0.09 at r 0.32, Mach 0.6, 0.01 to 0.04 from Mach 0.9 [3] Fig. 3(b), p. 16: no sign. (d)
  Blockage: wrong sign.

## 4. Flights with the motor burning

Langley models flown at Wallops [8] p. 3; [10] p. 2. Purser's jet-off values are glides at the same Mach number, his
exit pressures computed [8] pp. 4–5; Hermes' were measured [10] p. 7.

| Flight, figure | r, nozzle | p_e/p∞ | Mach | C_pb on | C_pb off | ΔC_pb |
|:--|:--|--:|--:|--:|--:|--:|
| Purser C [8] Fig. 6 | 0.55, 10° | above 1.7 | 1.11 | −0.23 | −0.08 | −0.15 |
| | | 1.3 / 0.8 | 1.30 / 1.36 | −0.28 / −0.30 | −0.08 / −0.10 | −0.20 / −0.20 |
| Purser D [8] Fig. 7 | 0.64, 9° | 2.0 | 1.0 | +0.01 | +0.01 | 0.00 |
| | | 1.9 / 1.7 | 1.10 / 1.23 | −0.08 / −0.15 | −0.05 / −0.09 | −0.03 / −0.05 |
| Hermes A(2) [10] Fig. 22 | 0.63, jet vanes | about 1.4 | 1.06 to 1.29 | −0.20 | −0.11 to −0.12 | −0.08 to −0.09 |
| Hermes A(3) [10] | same, fins aft | 1.35 to 1.55 | 1.06 / 1.29 | −0.26 / −0.21 | −0.11 / −0.12 | −0.16 / −0.09 |
| Gillespie [9] Fig. 5(b) | 0.62, 15° | not given | 1.0 / 1.4 | −0.20 / −0.18 | −0.10 / −0.12 | −0.10 / −0.06 |
| Purser B [8] Fig. 5 | 0.86, 30° | 1.1 | 1.05 / 1.24 | +0.35 / +0.27 | −0.10 / −0.05 | +0.45 / +0.32 |

- **Bodies.** All boattailed, base 0.60 to 0.70 of the largest diameter [8] Table I, p. 12; [10] p. 4 (Inference), fins
  ending at the base [8] Fig. 2, p. 15; [10] pp. 5–6. Jet-off +0.01 to −0.12, shallower than a flat base's −0.22: levels
  don't transfer, ΔC_pb may. Purser D's exit is 0.25 in inside a skirt [8] Fig. 3(d); Gillespie's may be a Hermes [9].
- **How read.** Purser C by pixel on its 1 psi grid (±0.015); D and B by eye (±0.02 to ±0.03; B at Mach 1.05 ±0.05);
  Hermes, Gillespie ±0.01. They give back the report's +50% base drag for A(3) at Mach 1.1 and "no change" for A(2) at
  1.30 [10] pp. 1, 15 (Inference: 0.267 × 0.61/0.110 = 1.48; 0.198 × 0.61/0.118 = 1.02).
- **Onset.** Hermes A(2) falls to −0.20 at Mach 1.02 to 1.05 [10] Fig. 22 (PDF p. 43), p. 15; Cubbage's jet-on
  curves start down near Mach 0.92 and are down by 1.0 [1] Figs. 11(a), 11(b): the flight drop comes 0.05 to 0.1 later.
- **Fins.** Jet off, thin fins ending 0.05 base diameters ahead of the base cut base drag 0.03 to 0.04 below Mach 1,
  about 0.02 at 1.05 to 1.1 ([cylinder base drag](cylinder-base-drag.md) §2; [11]). Jet on, no test varied fins; A(3)'s
  deeper dip comes with fins 0.16 base diameters past the base and a higher p_e/p∞ (1.55 p. 15, 1.35 summary) [10].
- **Against Cubbage.** Same sign, later onset; no flight annulus falls below −0.30, against his −0.41 to −0.57 at r 0.45
  to 0.75 at Mach 1.1 (parent §2.1), mostly the boattails' jet-off level; the ΔC_pb overlap (flight 0.00 to −0.20,
  his sonic and 12° jets at r 0.65 to 0.75 −0.17 to −0.37). Flight supports his sign and half to all his size.
- **The other way.** The X-15's eight liquid chambers raised its fuselage-base C_pb from −0.31 to −0.23 at Mach 1.2 [12]
  Fig. 8(c), p. 22 (±0.01): a clustered, underexpanded jet at altitude, far from an HPR boost.

## 5. A hot rocket jet against a cold one

- **The pair.** On one 1.5 in strut-held body, MA 10F's nozzle 1 blew cold CF4 (15.1° cone, area ratio 7.97), FA 22's
  nozzles 5, 6 fired 2% and 16% aluminum (15.0°, 8.0 to 8.1), exits within 2% in radius [7] Table 2-1, p. 2-14. Mach
  1.2: three hot runs, exit Mach 2.92 to 3.19, p_e/p∞ 1.10 to 1.47 [7] Table 2-6; seven cold, p_e/p∞ 1.29 to 4.90 [7].
- **Result (Inference).** At equal thrust coefficient on the base area (2.6 to 3.6), hot C_pb is −0.32 to −0.37 and
  cold, interpolated, −0.40 to −0.45: the hot jet reads 0.063 to 0.088 higher over CF4's tabled γ, 1.155 to 1.229 [7]
  p. 2-22 (MJ over MJ/GAMMA); one hot run is a short extrapolation. FA 22 flew no jet-off run; against the same body's
  cold-test jet-off, −0.20 at Mach 1.2 [6] Fig. 19, p. 58, the hot runs (r 0.47) give −0.12 to −0.17.
- **Same pair at Mach 0.9 (Inference, [7] Tables 2-3, p. 2-16, and 2-6):** two hot runs in the cold thrust range read
  −0.02 to +0.05 from cold over its γ (1.147 to 1.212), two outside +0.04 and +0.06 (run 14/7, +0.14, is nozzle 7,
  area ratio 4.0). The shift grows from about 0 to +0.06 at Mach 0.9 to +0.08 at 1.2, as at Mach 1.91 (parent §2.2).
- **Heated sonic jets.** Henry and Cahn's 800 and 1,200 °F jets read above cold at equal pressure ratio, Mach 1.0 / 1.1:
  +0.01 to +0.05 / +0.02 to +0.06 at r 0.248 (ratio 2 to 7), +0.04 to +0.06 / +0.10 to +0.14 at r 0.351 (ratio 3 to
  3.8; the 1.1 cold line rests on two points) [5] Figs. 4(c), 4(g), pp. 16, 20 (±0.01): heat alone gives the sign.

## 6. A band for HPR exits, and the rules on it (Inference)

HPR exits (parent §1): r about 0.2 to 0.7 at minimum diameter, under 0.5 in larger airframes; exit Mach 2.2 to 3.2;
p_e/p∞ 0.5 to 1.5 at sea level; hot, often aluminized; cones about 12° to 15°.

- **Sign.** Every 9° to 15° nozzle with p_e/p∞ 0.8 to 2 gives ΔC_pb ≤ 0 above Mach 1.0 (§1, §4, §5).
- **r 0.45 to 0.65:** flights −0.03 to −0.20; FA 22 hot −0.12 to −0.17; Cubbage's cold r 0.45 and 0.65 jets −0.19 and
  −0.32 at Mach 1.1, or −0.11 to −0.24 with the hot shift. Best estimate −0.10 to −0.20.
- **r 0.75 to 0.8:** cold tunnel data only, −0.17 (matched, 12°) to −0.37 (sonic); with the hot shift about −0.08 to
  −0.30 (5° cone at Mach 1.0: −0.48). **r 0.4 and under:** between Cubbage's r 0.25 and 0.45 jets, about 0 to −0.11 hot.
- **Band:** 0 to −0.30 at Mach 1.0 to 1.2, narrowing to 0 to −0.20 by Mach 1.3, where Hermes, Purser D and Gillespie
  shrink to −0.05 to −0.09 (not Purser C). Wall bias and the 5° cone near Mach 1.0, and fins under power, lie outside.

**The rules.** Rule minus measured, on the base area, is ΔC_pb(1 − r²) for the exit annulus, always low when the jet
deepens suction, and −C_pb,off r² + ΔC_pb(1 − r²) for the whole base, exact when ΔC_pb = C_pb,off r²/(1 − r²). With
C_pb,off −0.22, the whole base is exact at ΔC_pb −0.06, −0.12 and −0.28 for r 0.45, 0.6 and 0.75: it holds best for
large exits, which pump most. On the best-estimate band its error is −0.12 to −0.04 at r 0.45, −0.05 to +0.02 at r 0.6,
and +0.04 to +0.08 at r 0.75 (−0.01 to +0.09 on that exit's hot band); the exit annulus reads low by up to 0.16, 0.13
and 0.09 (0.13). Across the cold supersonic tunnel cases (§1, MA 10F) the whole base is within −0.03 to +0.05 in seven
of nine (5° to 35° cones); Cubbage's 25° cone gives +0.11 at Mach 1.1, his 5° cone −0.11 at 1.0; the sonic jets −0.14
to +0.01. The parent's "low by up to 0.11 and 0.19" come from these sonic jets: a worst case, not an HPR boost.

## 7. A worked example (Inference)

A flat-based rocket at Mach 1.1, r 0.6, C_pb,off −0.22, ΔC_pb −0.15 (mid-band). Measured-like base drag on the base
area: 0.37 × 0.64 = 0.237. Whole base: 0.220 (0.017 low). Exit annulus: 0.141 (0.096 low). HPR Sim's case rule at
minimum diameter, case 0.95 of the base: 0.22 × (1 − 0.95²) = 0.021 (0.215 low). At minimum diameter these move the
rocket's C_D almost one for one. Too little base drag under power raises the simulated apogee: flattering for an
altitude goal, the safe side against a waiver ceiling.

## What this suggests

1. Through Mach 1.0 to 1.3 under power, the whole-base rule is the closest simple rule for HPR exits (§6); the
   exit-annulus and case rules read low.
2. A better rule adds ΔC_pb −0.10 to −0.20 on the annulus (r 0.45 to 0.65, Mach 1.0 to 1.2, fading by 1.3 to 1.5), with
   0 to −0.30 as its band and the cold sonic −0.37 and 5° cone −0.48 (Mach 1.0) as worst cases.
3. A test row for #222 and M1.14f: Mach 1.1 to 1.2, hot, r 0.47 to 0.63, annulus C_pb −0.20 to −0.37 (Hermes A(2), A(3),
   FA 22); any power-on rule should land inside it.
4. Re-run C06/1 under the whole-base rule and the band: its 24 points (parent §6) lean to the whole-base end.

## Open questions

- No flight of a flat-based body with a burning motor and annulus pressures near Mach 1 was found (Gillespie's cylinder
  gives only the drag share [9] p. 1; Peck's flights [15] have no jet).
- MA 10F's exits sit 0.21 base diameters aft of the base [6] Table 1, p. 18, which likely shrinks the effect (a
  third-diameter extension cut sonic-jet base drag about 40% [4] p. 5).
- Fins with the jet on (§4); Henry and Cahn's deeper small exits at Mach 0.9 (§3): unsized.
- Not reached: NASA TN D-754 and D-978 (no NTRS file); NACA RM L56D30 (twin-jet rocket flights, Mach 0.8 to 1.15);
  AEDC-TR-76-109 (hot and cold afterbody jets in 16T).
- Dropped 2026-11-02: §3's cause (c) "wrong sign" (two taps at one radius read as inner and outer).

## Sources

All read 2026-10-29 ([1] to [3], [5], [7], [10] re-read 2026-11-02); cached in `refs/papers/` and `refs/research/`.

1. J. M. Cubbage, Jr., NACA RM L56C21, 1956. <https://ntrs.nasa.gov/citations/19930089150>
2. J. M. Cubbage, Jr., NACA RM L57B21, 1957. <https://ntrs.nasa.gov/citations/19930089627>
3. W. R. Scott, T. H. Slocumb, Jr., NASA Memo 3-10-59L, 1959. <https://ntrs.nasa.gov/citations/19980230684>
4. W. J. Nelson, W. R. Scott, NACA RM L58A27, 1958. <https://ntrs.nasa.gov/citations/19930090053>
5. B. Z. Henry, Jr., M. S. Cahn, NACA RM L56G12, 1956. <https://ntrs.nasa.gov/citations/19930089293>
6. C. D. Andrews, C. E. Cooper, Jr., LMSC-HREC TM D306909 (MA 10F), 1974. <https://ntrs.nasa.gov/citations/19760024194>
7. P. R. Sulyma, M. M. Penny, NASA CR-150826, 1978. <https://ntrs.nasa.gov/citations/19780025191>
8. P. E. Purser, J. G. Thibodaux, H. H. Jackson, NACA RM L50I18, 1950. <https://ntrs.nasa.gov/citations/19930086428>
9. W. Gillespie, Jr., NACA RM L51J29, 1951. <https://ntrs.nasa.gov/citations/19930086791>
10. H. H. Jackson, NACA RM SL55F15, 1955. <https://ntrs.nasa.gov/citations/20010057623>
11. R. G. Hart, NACA RM L52E06, 1952. <https://ntrs.nasa.gov/citations/19930087048>
12. E. J. Saltzman, NASA TN D-1056, 1961. <https://ntrs.nasa.gov/citations/19980227195>
13. S. F. Hoerner, *Fluid-Dynamic Drag*, 1965. <https://archive.org/details/FluidDynamicDragHoerner1965>
14. AGARD-AR-226, *Aerodynamics of Aircraft Afterbody*, 1986, PDF p. 24, archived DTIC copy:
    <http://web.archive.org/web/20240703082013/https://apps.dtic.mil/sti/pdfs/ADA172341.pdf>
15. R. F. Peck, NACA TN 3372, 1955. <https://ntrs.nasa.gov/citations/19930084618>
