# Where the Arcas Robin's fin share sits, Mach 0.6 to 1.2, with force and moment on one window

NASA TN D-4013's Arcas Robin models put HPR Sim's center of pressure (CP) up to 2.36 calibers aft of the tunnel's from
Mach 0.8 to 1.0 ([#1198](https://github.com/nrdptel/fusionspace-eridanus/issues/1198)).
[transonic-body-normal-force](transonic-body-normal-force.md) §3 found the fin share (fins on less fins off) centered
0.5 to 0.6 caliber *ahead* of the fins' root leading edge at Mach 0.8 and 0.9 (short model) and 2.7 calibers ahead at
1.0 (long). It took slopes over all plotted angles (−3° or −4° to 4°) but the CP from the faired curves of Figs. 11
and 12, labeled −2° to 2°. This note reads the fins-on pitching moment (C_m) of both models and fits force and moment
on one set of angles. **Inference** marks this note's readings, fits and arithmetic. It builds on
[arcas-short-model-mach-0-9](arcas-short-model-mach-0-9.md), and it moves #1198's CP-error table and the transonic CP
rows of [accuracy](../accuracy.md).

**In short: on one window the fin share sits aft of the fins' root leading edge, and the long model's worst row goes.**

- **The forward center was mostly the mixed basis.** On one window the share sits 0.20 to 0.53 caliber aft of the root
  leading edge (short, −3° to 4°) and 0.20 to 0.63 aft (long, −4° to 4°). HPR Sim puts its fin term 0.56 to 0.74 aft.
- **The long model's 2.7 calibers ahead at Mach 1.0 rests on Fig. 12's curve, not its points.** Its CP there, 71.5% of
  length, disagrees with the plotted C_N and C_m at ±4° by 10.4% of length (2.5 calibers). On those points the share
  is 0.20 aft, as at Mach 0.9. The committed +2.36-caliber miss becomes −0.54 on the same angles.
- **Within ±2° the short model's share moves forward,** to 0.30 (Mach 0.8) and 0.12 caliber (0.9) ahead of the edge,
  as the fins-on moment flattens near zero ("almost neutral" [1] p. 6); the fins-off body doesn't change. It sits 0.2 to
  1.1 standard deviations from the edge, so "ahead" isn't established; its cause isn't bounded.
- **The short model's miss depends on the window.** HPR Sim less tunnel, both fitted on the same angles: +0.87, +1.27,
  +0.65 calibers at Mach 0.8, 0.9, 0.95 over −2° to 2°, but +0.08, +0.58, +0.37 over −3° to 4°.
- **A correction.** At Mach 0.9 the short model's fins-on square at −2.1° sits at C_m +0.18, above the zero line, not
  at −0.03 to −0.05. Its points over ±2° then give 71.8% of length, beside Fig. 11's 71.6%, not 69.9%.

Confidence: high on the long model's Mach 1.0 conflict and on every center's sign over the wide windows; low on the
sign over ±2°. Reading spreads: ±0.09 to 0.33 caliber on a share's center, ±0.03 to 0.07 on a whole-rocket CP.

## 1. The readings

- **Figures** [1]: fins-on and fins-off symbols at roll 0, fins at 0° cant. C_N in Figs. 7(b) and 9(b) (printed pp. 22,
  41), C_m in Figs. 7(c) and 9(c) (pp. 23–25, 42–44); the PDF page is the printed one plus 2. Moments are about 70% of
  length [1] p. 2: 12.74 calibers (short) and 16.64 (long) from the nose tip.
- **Counted angles.** The short model was run at about −3°, −2°, 0°, 2° and 4° near zero; the long one only at −4°,
  0° and 4° (Figs. 7(b), 9(b)). So a −2° to 2° window exists only for the short model.
- **New here (Inference):** 90 C_m points, 60 short (Mach 0.6, 0.8, 0.9, 0.95, 1.0, 1.2) and 30 long (0.6, 0.8, 0.9,
  1.0, 1.2), from the native 300-ppi scans. Each symbol is matched to its own page's legend symbol, the angle is read
  off the thin vertical grid lines, and C_m off the horizontal lines near the panel's zero, tilted as the zero line
  runs.
- **Cross-checks (Inference).** The fins-off points match the ring fits of
  [transonic-body-normal-force](transonic-body-normal-force.md) §1: 35 points, rms 0.016, at most 0.050. The short
  fins-on points match the earlier reading of [arcas-short-model-mach-0-9](arcas-short-model-mach-0-9.md): 24 of 25
  points, rms 0.035, at most 0.062, mostly a near-constant offset on the Mach 1.0 panel, which a slope ignores. The
  25th, Mach 0.9 at −2.1°, differs by 0.22: both first readings took the flat-bottomed symbol under the zero line, but
  the roll-0 square is the open one above it, on the faired curve from −3° (checked by eye on the scan). The two
  plots' angles for one run agree within 0.09° (mean 0.03°); each run takes their mean.
- **C_N** is the committed reading (`arcas-robin-wind-tunnel.json`). Alternates: the earlier re-read of Fig. 7(b),
  and a re-read of Fig. 9(b) here. On Fig. 9(b) the roll-0 square overlaps the 22.5° and 45° symbols at ±4°, about
  ±0.03 in C_N; the two readings differ by up to 0.05 there, and up to 0.11 in the clusters at 0°.
- **Spreads (Inference).** Each point is drawn 4,000 times at ±0.02 in C_N (±0.03 long) and ±0.03 in C_m; a spread is
  one standard deviation. The report's own accuracy, ±0.03 in C_N and ±0.05 in C_m [1] p. 4, is the wider "balance"
  spread.

## 2. The fin share on one window (Inference)

Fin share = fins-on slope less fins-off slope, each a least-squares line with an intercept over the window's points.
Its center is the moment center less C_mα/C_Nα calibers (C_m is per caliber, nose-up positive). Root leading edges, from
the committed designs (TN D-4014 Fig. 1(a) [2]): 16.58 calibers (short) and 22.16 (long) from the nose tip.

| per rad, C_Nα / C_mα | 0.6 | 0.8 | 0.9 | 0.95 | 1.0 | 1.2 |
|---|--:|--:|--:|--:|--:|--:|
| short, −2° to 2° | 8.40 / −36.4 | 8.00 / −28.3 | 6.90 / −25.6 | 9.07 / −34.4 | 14.61 / −62.5 | 12.77 / −58.0 |
| short, −3° to 4° | 9.49 / −40.3 | 8.49 / −34.3 | 7.74 / −31.8 | 8.97 / −36.2 | 14.23 / −59.4 | 13.00 / −56.8 |
| long, −4° to 4° | 8.95 / −55.0 | 8.73 / −52.1 | 9.52 / −54.4 | none | 14.61 / −83.5 | 12.67 / −76.4 |

Reading spreads: ±0.56 / ±0.8 per rad (short, −2° to 2°), ±0.28 / ±0.4 (short, −3° to 4°), ±0.44 / ±0.4 (long).

| center, calibers from the root leading edge, + aft | 0.6 | 0.8 | 0.9 | 0.95 | 1.0 | 1.2 |
|---|--:|--:|--:|--:|--:|--:|
| short, −2° to 2° | +0.50 ± 0.31 | −0.30 ± 0.28 | −0.12 ± 0.33 | −0.05 ± 0.25 | +0.44 ± 0.17 | +0.70 ± 0.21 |
| short, −3° to 4° | +0.41 ± 0.13 | +0.20 ± 0.14 | +0.27 ± 0.16 | +0.20 ± 0.13 | +0.33 ± 0.09 | +0.53 ± 0.10 |
| long, −4° to 4° | +0.63 ± 0.31 | +0.46 ± 0.31 | +0.20 ± 0.27 | none | +0.20 ± 0.17 | +0.52 ± 0.21 |
| body note §3's mixed basis, short | −0.12 | −0.60 | −0.54 | 0.00 | +0.75 | +1.27 |
| body note §3's mixed basis, long | +0.46 | +0.31 | +0.12 | none | −2.69 | +0.78 |
| HPR Sim's fin term, both models | +0.56 | +0.56 | +0.61 | +0.63 | +0.65 | +0.74 |

- **Balance spreads** are about 1.6 times the reading's on the short model and about equal on the long: ±0.27 to
  0.53 caliber (short, −2° to 2°), ±0.13 to 0.24 (short, −3° to 4°), ±0.18 to 0.32 (long).
- **The C_N reading matters most on the long model:** its Fig. 9(b) re-read moves the centers to +1.05, +0.66, +0.12,
  +0.30 and +0.55. On the short model the earlier re-read moves them by 0.17 at most. The two readings of Fig. 7(c)
  agree within 0.04 on every center.
- **Two-angle half-windows** (0° to 2°, −2° to 0°, and the long model's 0° to 4°, −4° to 0°) carry 16th-to-84th
  percentile ranges up to 2.4 calibers wide and move up to 1.3 between C_N readings: too few points to place a center.

## 3. Flow or fitting? (Inference)

- **Long model, Mach 1.0: Fig. 12.** On the plotted points the whole rocket's CP is 81.9% of length (82.1% on the
  re-read C_N); Fig. 12's curve gives 71.5% [1] p. 61. A 71.5% CP needs C_mα near −6 per rad; the ±4° symbols give
  −48 (+3.05 at −3.8°, −3.53 at 4.05°, each clear of the other roll angles). At Mach 0.6 to 0.9 and 1.2 the curve and
  the points agree within 0.9% of length. At Mach 1.0 the short model's two windows differ by only 0.9% of length
  (0.17 caliber), so a ±2° nonlinearity eleven times that size is unlikely. The mixed basis inherited the 10.4%.
- **Short model, Mach 0.8 and 0.9: partly the window, partly the data.** Over −3° to 4° the share sits +0.20 and +0.27
  aft. Over −2° to 2° the fins-off body moment slope barely changes (26.7 against 26.2 per rad at Mach 0.8, 22.7
  against 22.8 at 0.9). The fins-on slope goes from −8.1 (Mach 0.8) and −9.0 (0.9) per rad over −3° to 4° to −1.6
  and −3.0 over ±2°: the curve flattens near zero, as the report says of Mach 0.60 to 0.90 [1] p. 6, though the Mach
  0.6 points don't flatten. The share's moment falls 17% to 19% there, its force 6% to 11%, so its center moves 0.4
  to 0.5 caliber forward.
- **How far ahead, and how sure.** Over ±2°: 0.30 (Mach 0.8, 0.44 on the re-read C_N) and 0.12 caliber (0.9; 0.08
  *aft* on the re-read C_N). That is 1.1 and 0.4 reading spreads from the edge, and 0.7 and 0.2 balance spreads.
- **Causes, none sized.** With 2° of fin cant the report finds the small-angle stability "considerably improved" [1]
  p. 6, which points at the fins' own flow, not the body. The short model's fin anchors, in its slots with the fins off
  and not flush with the skin ([1] pp. 3–4, 6), would bias the share forward if they carry lift. But they are in every
  fins-off run, and the fins-off slope barely changes between windows, so they can't make the ±2° move. Taking the
  extra Mach 0.9 fins-off lift that [arcas-short-model-mach-0-9](arcas-short-model-mach-0-9.md) §4 put near 88% of
  length out of the body moves the center under 0.01 caliber (±2°) and about 0.08 forward (−3° to 4°).

## 4. The committed CP error on one window (Inference)

The committed tunnel CP is read off the faired curves of Figs. 11 and 12 (pp. 60–61); HPR Sim's is fitted over −2° to
2° (`CP_ANGLES_DEG` in `xtask/src/aero_mach.rs`). Here the tunnel's CP comes from the plotted points, force and moment
on the same angles. HPR Sim's comes from the same angles too: its body by hand from its slender-body and crossflow terms
(the [sibling note](transonic-body-normal-force.md) §2's model, which matches the fixture to 10⁻⁴), plus its fin term,
which is linear in α (`fin_term` in `crates/hpr-aero/src/model.rs`).

| tunnel CP, % of length | 0.6 | 0.8 | 0.9 | 0.95 | 1.0 | 1.2 |
|---|--:|--:|--:|--:|--:|--:|
| short, Fig. 11 (committed) | 74.7 | 70.9 | 71.6 | 76.45 | 83.55 | 85.2 |
| short, points −2° to 2° | 75.4 | 71.0 | 71.8 | 76.5 | 82.4 | 82.2 |
| short, points −3° to 4° | 77.1 | 74.5 | 74.9 | 77.3 | 81.5 | 81.8 |
| long, Fig. 12 (committed) | 76.8 | 75.6 | 76.2 | none | 71.5 | 82.0 |
| long, points −4° to 4° | 77.4 | 76.1 | 76.5 | none | 81.9 | 81.1 |

| calibers, HPR Sim's CP less the tunnel's | 0.6 | 0.8 | 0.9 | 0.95 | 1.0 | 1.2 |
|---|--:|--:|--:|--:|--:|--:|
| short, committed | +0.12 | +0.91 | +1.34 | +0.68 | −0.41 | −0.11 |
| short, both −2° to 2° | −0.03 | +0.87 | +1.27 | +0.65 | −0.22 | +0.41 |
| short, both −3° to 4° | −0.49 | +0.08 | +0.58 | +0.37 | −0.19 | +0.37 |
| long, committed | −0.36 | +0.07 | +0.68 | none | +2.36 | +0.67 |
| long, both −4° to 4° | −0.94 | −0.50 | +0.17 | none | −0.54 | +0.51 |
| long, points −4° to 4°, HPR Sim −2° to 2° | −0.49 | −0.05 | +0.61 | none | −0.12 | +0.90 |

- **Spreads:** ±0.06 to 0.07 caliber on a −2° to 2° row, ±0.03 to 0.04 on −3° to 4°, ±0.05 to 0.06 on the long model.
- **HPR Sim's CP moves forward with the window,** by 0.12 to 0.15 caliber from −2° to 2° to −3° to 4° (short) and
  0.38 to 0.45 caliber at ±4° (long): its body lift grows as sin²α. The tunnel's short-model CP moves 0.31 to 0.64
  caliber aft between the same windows at Mach 0.6 to 0.9.
- **What changes for #1198.** The short model's Mach 0.9 row stands: +1.27 on the report's own −2° to 2° label, within
  0.07 of the committed +1.34. The long model's Mach 1.0 row, the table's largest, rests on Fig. 12 alone: on its
  plotted points it is −0.54 like for like. At Mach 1.2 both models' points sit 0.2 to 0.63 caliber forward of the
  curves, unexplored here.
- **What carries the one-window miss.** Short model, Mach 0.9, −3° to 4°: putting the tunnel's fin share in place of
  HPR Sim's fin term turns +0.58 into −0.66. Its force alone (7.74 per rad against HPR Sim's 10.87, 40% high) moves the
  CP 0.97 caliber; its center alone (0.27 against 0.61 caliber aft of the edge) moves it 0.29. Putting in the tunnel's
  body instead widens the miss to +1.04, as [transonic-body-normal-force](transonic-body-normal-force.md) §3 found.

## What this suggests

1. **Flag Fig. 12's Mach 1.0 CP in #1198 and the fixture,** without dropping it, and add a points-based CP row for
   each model and Mach number, scored like for like on the plotted angles. Expected effect: the long Mach 1.0 miss
   reads −0.54, not +2.36, and the transonic table's worst row is the short model's Mach 0.9 (+1.27 to +1.34). Which
   row scores is a decision for an ADR, with the 10.4% conflict stated.
2. **Correct [transonic-body-normal-force](transonic-body-normal-force.md) §3's "Where the miss sits".** On one window
   the share sits 0.20 to 0.63 caliber aft of the root leading edge, ahead of it only over ±2° on the short model at
   Mach 0.8 (and barely at 0.9), and not established there.
3. **Aim the fin-side work at the share's force first:** at Mach 0.9 HPR Sim's fin term is 40% (short) and 14% (long)
   above the tunnel's share and sits 0.34 to 0.41 caliber further aft. The force is worth about 1.0 caliber of the
   short model's one-window miss, the center about 0.3.
4. **Commit the 90 C_m points** with figure, page and Mach, so a CP row can be refitted on any window.
5. **Treat the ±2° flattening as its own effect.** The tunnel's CP moves 0.64 and 0.56 caliber between the windows at
   Mach 0.8 and 0.9; HPR Sim's moves 0.14. A fix to the fins' force alone won't close the short model's −2° to 2°
   rows there.

## Open questions

- **Where Fig. 12's Mach 1.0 dip comes from.** Unplotted runs near ±2°, or a fairing through other cant or roll data?
  The canted long model (Fig. 10(c), printed pp. 52–54) and its ±2° symbols, if any, are the next read.
- **Is the ±2° flattening flow?** Fig. 8(c) (2° cant, short) and the 22.5° and 45° roll symbols at ±2° would show
  whether it follows the fins. Its cause and size in flight are unbounded.
- **Why Fig. 11 at Mach 1.2 (85.2%) sits 3.0% to 3.4% of length aft of the points** (82.2% and 81.8%).

## Sources

All read 2026-11-05. US government works on NASA's Technical Reports Server (NTRS), public use; cached under `refs/`
(gitignored). This note's readings and scripts are in `refs/research/arcas-fin-share-cp-station/` (gitignored);
readings with figure and page may be committed, as `arcas-robin-wind-tunnel.json` does.

1. J. C. Ferris, *Static Stability Investigation of a Single-Stage Sounding Rocket at Mach Numbers From 0.60 to 1.20*,
   NASA TN D-4013, 1967. <https://ntrs.nasa.gov/citations/19670020050>. Printed pp. 2–4, 6; Figs. 7(b), 7(c), 9(b),
   9(c), 11, 12 (printed pp. 22–25, 41–44, 60, 61; PDF page = printed + 2). Cached as `refs/papers/nasa-tn-d-4013.pdf`.
2. C. D. Babb, D. E. Fuller, NASA TN D-4014, 1967, Fig. 1(a): the fin root's leading edge 3.65 in ahead of the base,
   through the committed geometry; not re-read here. <https://ntrs.nasa.gov/citations/19670020031>
