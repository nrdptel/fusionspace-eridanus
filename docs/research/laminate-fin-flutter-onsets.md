# Measured flutter onsets of bending-twist coupled laminate plates

**In short: two more MIT test series give low-angle onsets for eight more graphite/epoxy designs, seven of them coupled
(three of the eight start at divergence), and second onsets for the three designs already scored. With Martin's eq. 18
read as before, the largest flattering-side error on a plate that fluttered rises from 1.42 to 1.45 (1.455, on a
rounding edge).** Landsberger's 1983 thesis tested 13 plates, 0.804 mm thick (taken as 305 by 76 mm, section 1),
including the three designs already scored. Dunn and Dugundji's 1991 report tested three plates, about 559 by 140 mm
clamped, inside foam airfoil fairings. Where the plies twist a flat plate nose down as it bends up (washout), eq. 18
reads 0.79 to 1.45 times the onset. Where they twist it nose up (wash-in), the plate flutters early or diverges, and
eq. 18 reads 1.05 to 3.14 times the measured speed. Eq. 18 has no term for either. One stiffness correction (section 5)
brings every washout plate below 1 (at most 0.97) but leaves wash-in plates at 0.95 to 2.1. HPR Sim's built-in carbon
modulus reads lower still: 0.52 to 0.72 on every flat-plate flutter onset here, but 1.00 to 1.29 on four of the five
divergences. All cases are at 11 to 29 m/s, from one lab, roots clamped. The onsets and stiffnesses are as the sources
print them; the ratios are **Inference**. This moves M2.11's flattering-side count (one new design above 1 in classical
flutter, two whose oscillation starts at divergence, three designs' divergence speeds) and the flutter page's
coupled-laminate bound.

All sources read 2026-11-03. Ratios are Martin's speed over the lowest measured onset; above 1 is the flattering side.

## 1. Method, and where it had to assume

The ratios use the method of [measured flutter onsets](flutter-measured-onsets.md), section 6: eq. 18 as
[the flutter page](../physics/flutter.md) states it, $t/c$ on the root chord, $G_E = 12 D_{66}/t^3$ and air at 1 atm,
15 °C (no source here tabulates it; the tunnel is MIT's, in Cambridge, Massachusetts). Chords are the nominal 3 in
(76.2 mm) and 5.5 in (139.7 mm); at a round 76 mm the headline 1.455 becomes 1.456. Two points make it simpler:

- With $G_E = 12 D_{66}/t^3$ the thickness cancels: $(V_f/a)^2 \propto G_E (t/c)^3 = 12 D_{66}/c^3$. Only the plate's
  twisting stiffness $D_{66}$, chord and aspect ratio matter (**Inference**, algebra on eq. 18).
- $D_{66}$ is computed by classical lamination theory. The script reproduces Hollowell's Table A-2 (S4 p. 111) within
  0.0007 N·m (0.04%) on all four of his layups, and Dunn's Appendix B (S2 p. 175) exactly with 0.135 mm plies.

Ply constants: AS/3501-6 at $E_L$ 98 GPa, $E_T$ 7.9 GPa, $\nu$ 0.28, $G_{LT}$ 5.6 GPa (S1 p. 21, Hollowell's "Turner"
set) or 130, 10.5, 0.28, 6.0 GPa ("TELAC" set, S4 Table A-1 p. 110); ratio ranges span the two sets. Dunn's plates use
his out-of-plane set, 97.3, 6.3, 0.28, 5.3 GPa (S2 p. 173). Assumed: Landsberger gives no planform in text, so his
plates take Hollowell's 305 mm cantilever by 76 mm (S4 p. 51, Table D-2 p. 124). Support: he says his work used "some of
the same wings" (S1 p. 11), and a 305 mm cantilever with the Turner $D_{11}$ gives a first bending frequency of 11.05 Hz
for [0₂/90]s against his computed 10.8 Hz (S1 Table 4.2 p. 78; **Inference**). Air at 25 °C instead of 15 °C raises
every ratio by 1.7%.

## 2. Landsberger, MIT, 1983: thirteen 305 by 76 mm plates

Six-ply AS/3501-6 rectangles, 0.804 mm thick, clamped in the MIT acoustic tunnel (1.5 by 2.3 m free jet, about 30 m/s
top speed, S1 p. 63). The 13 layups (S1 Table 3.1 p. 60, counted) are [0₂/90]s and, at 15°, 30° and 45°, four each:
[+θ₂/0]s, [±θ/0]s, [∓θ/0]s, [−θ₂/0]s. His sign convention: +θ plies outside give negative coupling,
bending up twists the tip nose down (washout); −θ outside gives wash-in (S1 p. 61). Flutter boundaries were found by
sweeping the root angle of attack at each speed, then raising the speed 1 m/s (S1 p. 67). Table D.1 (pp. 120–121)
lists, per plate, the speed, root angle, tip motion and frequency of a low-angle and a 10° case, taken at the boundary
speed or up to 1 m/s above it for visibility (p. 67). The boundary curves (Figs. D.1–D.4, pp. 124–127, read by
pixel) agree with Table D.1 within 0.8 m/s at zero angle. Table 4.1 (p. 77) gives measured divergence speeds.

| Plate | Onset, root angle, frequency | $G_E$ (GPa) | Martin (m/s) | Ratio | Ratio, section 5 |
|---|---|---|---|---|---|
| [+15₂/0]s, washout | 25 m/s, 1°, 32 Hz | 10.4–12.7 | 22.1–24.4 | 0.88–0.98 | 0.68–0.72 |
| [±15/0]s, light washout | 28 m/s, 3°, 37 Hz | 10.4–12.7 | 22.1–24.4 | 0.79–0.87 | 0.75–0.83 |
| [∓15/0]s, light wash-in | 21 m/s, 1°, 43 Hz | 10.4–12.7 | 22.1–24.4 | 1.05–1.16 | 1.01–1.11 |
| [∓30/0]s, wash-in | 24 m/s, 1°, 40 Hz | 20.0–26.1 | 30.7–35.0 | 1.28–1.46 | 1.19–1.36 |
| [∓45/0]s, wash-in | 22 m/s, 1°, 40 Hz | 24.8–32.8 | 34.1–39.3 | 1.55–1.79 | 1.46–1.68 |
| [0₂/90]s, uncoupled (M1) | 26 m/s, 1°, 33 Hz | 5.6–6.0 | 16.2–16.8 | 0.62–0.65 | 0.62–0.65 |
| [+30₂/0]s, washout (M3) | 29 m/s, 0°, 60 Hz | 20.0–26.1 | 30.7–35.0 | 1.06–1.21 | 0.67–0.73 |
| [+45₂/0]s, washout (M2) | 27 m/s, 1°, 25 Hz | 24.8–32.8 | 34.1–39.3 | 1.26–1.45 | 0.86–0.97 |

Source: S1 Table D.1 pp. 120–121; $G_E$, Martin and ratios **Inference**. The last three are the designs Hollowell
already measured (25.9, 27.1 and 27.7 m/s, S4 Table 6a p. 103); a repeat counts once, so the M2 design's range becomes
1.23–1.45 over both onsets on the nominal planform (the earlier note's 1.24–1.42 used Hollowell's measured planform
and thicknesses, which also moved its $G_E$; the thickness cancels in the speed). The [∓30/0]s and [∓45/0]s onsets
equal their measured divergence speeds (24 and 22 m/s, Table 4.1 p. 77): the tip sat 13.5 and 16.0 cm off center while
it oscillated (Table D.1), so we read these as divergence turning into stall flutter, not a classical onset
(**Inference**). [±30/0]s and [±45/0]s didn't flutter at low angle up to the tunnel's top speed (p. 80), where Martin
gives 30.7–35.0 and 34.1–39.3 m/s: ratio below 1.02–1.17 and 1.14–1.31, side unknown. Plates swept forward 30°
(Table D.2 pp. 122–123) are left out: eq. 18 has no sweep term.

## 3. Dunn and Dugundji, MIT, 1991: three faired 559 by 140 mm wings

Eight-ply by their layups, AS4/3501-6, 584 by 140 mm cut, with a 25 mm aluminum root tab (S2 p. 78, Fig. 8 p. 80), so
about 559 mm clamped span (half-span aspect ratio 4, p. 76), under NACA 0012 Styrofoam fairings over the outer 508 mm;
same tunnel, 1 m/s steps (S2 pp. 84–86). Plate thickness is not printed; 1.08 mm is what reproduces Appendix B's
$D_{66}$ (p. 175; Fig. 8 labels the plate "6 ply"). Onsets are read off Figs. 24–26 (pp. 134–136) at the lowest root
angle plotted, to about 0.5 m/s. The table's ratios are **Inference**.

| Wing | Onset | $D_{66}$ (N·m) | Martin, plate only | With foam's 1.2 N·m |
|---|---|---|---|---|
| [0₃/90]s, uncoupled | 26 m/s, 1°, 14 Hz | 0.556 | 9.9 m/s, 0.38 | 17.6 m/s, 0.68 |
| [+15₂/0₂]s, washout | 28 m/s, 1°, 14.7 Hz | 1.012 | 13.4 m/s, 0.48 | 19.8 m/s, 0.71 |
| [−15₂/0₂]s, wash-in | 18 m/s, 0.5°, 2.3 Hz | 1.012 | 13.4 m/s, 0.74 | 19.8 m/s, 1.10 |

The report itself says the foam's torsional stiffness, smeared at 80% of its 17 mm depth, is about 1.2 N·m of
equivalent $D_{66}$, more than the plate's own (S2 pp. 97–98); the last column adds it, the middle one is HPR Sim's
flat-fin reading. The [−15₂/0₂]s "onset" is a slow bending oscillation below the 3.6 Hz first bending mode (Table
2 p. 99), at the computed divergence speed of 18.0 m/s (Table 4, p. 122). These wings are airfoils, not flat fins; they
suggest how much a fairing changes the answer (**Inference**), not the flat-plate class. Dunn's 1989 thesis (S3) tested
330 by 76 mm faired wings; no measured point sits below 3° root angle (Figs. 20a–20d pp. 117–120, p. 116), so it
adds no low-angle onset.

## 4. Divergence: the wash-in plates

Eq. 18 is a flutter criterion, but a fin that diverges fails statically, so its speed against measured divergence is the
number a margin rests on. Measured divergence near zero root angle: Landsberger [−15₂/0]s 18, [−30₂/0]s 15,
[−45₂/0]s 14 m/s (S1 Table 4.1 p. 77); Hollowell [−30₂/0]s 11.7, [−45₂/0]s 12.5 m/s (S4 Table 5 p. 95).
Ratios (**Inference**): 1.23–1.36, 2.04–2.34, 2.44–2.81, 2.62–3.00 and 2.73–3.14. Hollowell's [−θ₂/0]s
specimens were his [+θ₂/0]s laminates rotated 180° about the span axis (S4 p. 51), so one laminate is a washout fin
or a wash-in fin depending only on how it is mounted. The two theses' divergence speeds differ by up to 3.3 m/s on the
same design; Landsberger called a plate diverged when it took a large bend near zero angle (S1 p. 77).

## 5. Why eq. 18 reads high on coupled plates (Inference)

$12 D_{66}/t^3$ is the plate's twisting stiffness with bending held. A cantilever fin is free to bend, and the
coupling term $D_{16}$ then lowers the stiffness it shows to a twisting load to about $D_{66} - D_{16}^2/D_{11}$
(beam reduction of the plate matrix; this note's reading, not a source's). On [+45₂/0]s that is 0.63 of 1.42 N·m
(TELAC set): the plate twists 2.2 times as easily as $D_{66}$ says. Used as $G_E$, it gives section 2's last column:
every washout plate drops below 1 (0.67–0.97 on section 2's flat plates, and M2 and M3's own onsets 0.84–0.95 and
0.72–0.79), while wash-in plates stay at 1.01–1.68 in flutter and 0.95–2.10 in divergence. The correction removes
the washout plates' excess; it doesn't model divergence, which needs its own check.

HPR Sim's built-in carbon fiber modulus, 4.83 GPa (the flutter page's material table), is below every $G_E$ here. With
it, Martin reads 15.1 m/s on every 305 by 76 mm plate: 0.52–0.72 on the flutter onsets, 0.84–1.29 on the divergences
(1.00, 1.08, 1.21 and 1.29 on the four slowest), and 0.34–0.53 on Dunn's faired wings with the plate alone.

## 6. Sources not reached or not counted

- **Abang Haji Abdul Majid, UPM PhD (2008)**, six-ply woven glass/epoxy plates from −45° to +45° tested for flutter:
  the repository host timed out; an archived copy holds only the 25 front pages, which report a 29% measured gain in
  flutter speed from tailoring and no plate data (S6 pp. iii–iv). Its journal paper (Acta Mech. Sin., 2008) is
  paywalled.
- **Kuttenkeuler and Ringertz**, J. Aircraft 35 (1998) 505–507, a tunnel-tested optimized orthotropic wing: paywalled;
  the KTH thesis record has no full text (S8).
- **Tang, Henry and Dowell**, AIAA J. 37(11) 1999 1355–1362, the Duke cropped delta: paywalled; the abstract (S7)
  names a constant-thickness plate but not its material, so whether it is a laminate is not known here.
- **Chen and Dugundji (1986)**, DTIC ADA172922: the same wings on a free-flying forward-swept model, not cantilevers
  (S5 abstract and refs. 1–2); no new cantilever onset.
- The journal versions of Hollowell (J. Aircraft 21(1), 1984) and Landsberger (22(8), 1985) were not read; the theses
  carry their data.

## What the project can use

- **Cases for M2.11, cite only.** S1, S3 and S4 are MIT theses: free to view, reproduction or distribution only with
  written permission (DSpace terms of use). S2 is marked approved for public release, distribution unlimited (its DD
  Form 1473); as a contractor report its copyright is not settled here, so cite numbers only. Reading copies are cached
  under `refs/research/laminate-fin-flutter-onsets/` (gitignored), never committed.
- **New distinct designs with a low-angle flutter onset:** [+15₂/0]s, [±15/0]s and [∓15/0]s (S1); two faired wings
  (S2), which are airfoils rather than fins. Second onsets for the M1, M2 and M3 designs (S1). All from one group (MIT
  TELAC), so they add cases, not a group.
- **The sign test:** on flat plates that fluttered, leaving out the two divergence-onset cases, ratios run 0.62 to 1.45
  (section 2). The class's flattering bound becomes 1.45 (M2's design at Landsberger's 27 m/s, TELAC ply set), or
  1.79 if a divergence-onset oscillation counts.
- **Air state:** assumed 1 atm, 15 °C for every case here; no thesis tabulates it.

## What this suggests

- The flutter page's coupled-laminate line could read "1.06 to 1.45" for the 30° and 45° washout designs (both
  theses' onsets) and name wash-in separately: plies that twist a fin nose up as it bends diverged at 0.32 to 0.81
  times eq. 18's speed in these tests (the inverse of 1.23–3.14), the largest flattering factor the project's notes
  record so far.
- A possible change (expected effect, not a plan): for a user-entered laminate, take $G_E$ from
  $12(D_{66} - D_{16}^2/D_{11})/t^3$ and warn on wash-in layups. On these plates that moves every washout case below 1
  (largest 0.97) and leaves wash-in cases flagged rather than scored. It would need a source for the reduction.
- The built-in carbon modulus is on the safe side on every flutter onset in sections 2 and 3, not for divergence:
  M2.11 could count divergence cases in their own row.
- A check that a fin's mounting is known: Hollowell's 30° and 45° laminates read 1.06 to 1.45 one way out and 2.6 to
  3.1 times their divergence speed the other (sections 2 and 4).

## Open questions

- Is $D_{66} - D_{16}^2/D_{11}$ the right effective twisting stiffness for a short cantilever plate with root warping
  restrained? A plate-theory source would settle it; here it is only a beam-style reading.
- Did Landsberger's flutter runs use the airspeed gauge he found reading low in his steady tests (true 11.5 and 16 m/s
  for read 10 and 15, S1 p. 66)? If so his onsets are low and his ratios high. Bounded only where his and Hollowell's
  onsets overlap: 26 against 25.9 m/s and 27 against 27.7 m/s agree within 3%, but 29 against 27.1 m/s differ by 7%.
- None of these is a hobby fin: no tube, no fillet, a thickness ratio of 0.008 to 0.011 (the thin end of the 0.01 to
  0.03 the earlier note gives for hobby fins), and onsets far below rocket speeds, where compressibility and the air's
  density move eq. 18 further.
- Fig. D.4 labels the [+45₂/0]s boundary at 50 to 52 Hz; Table D.1 gives 25 Hz at the low-angle onset. The speed
  agrees; which frequency is right is not settled.
- The woven glass plates (S6) and the KTH wing (S8) would add a second group; neither was readable.

## Sources, all read 2026-11-03

- **S1.** Landsberger, "Aeroelastic properties of straight and forward swept graphite/epoxy wings", S.M. thesis, MIT
  (1983), pp. 11, 21, 59–67, 79–80, Tables 3.1 p. 60, 4.1 p. 77, 4.2 p. 78, D.1 pp. 120–121, D.2 pp. 122–123,
  Figs. D.1–D.4 pp. 124–127. <https://hdl.handle.net/1721.1/51439> (read from the Internet Archive copy of the
  DSpace file, as DSpace returned 429).
- **S2.** Dunn and Dugundji, "Nonlinear stall flutter of wings with bending-torsion coupling", TELAC Report 91-16A,
  AFOSR (December 1991), DTIC ADA254323, pp. 76, 78, 80, 84–86, 97–99, 118–119, Table 4 p. 122, Figs. 24–26 pp.
  134–136, Appendices A–B pp. 173–175. Read from the Internet Archive's 2016 copy of
  <https://www.dtic.mil/dtic/tr/fulltext/u2/a254323.pdf> (DTIC returned 403). The same text is Dunn's PhD thesis, MIT
  (1992), <https://hdl.handle.net/1721.1/31028>.
- **S3.** Dunn, "Stall flutter of graphite/epoxy wings with bending-torsion coupling", S.M. thesis, MIT (1989), p. 64,
  Table 2 p. 109, p. 116, Figs. 20a–20d pp. 117–120. <https://hdl.handle.net/1721.1/41240>
- **S4.** Hollowell, "Aeroelastic flutter and divergence of graphite/epoxy cantilevered plates with bending-torsion
  stiffness coupling", S.M. thesis, MIT (1981), pp. 51–53, Table 5 p. 95, Table 6a p. 103, Tables A-1, A-2 pp.
  110–111, Table D-2 p. 124. <https://hdl.handle.net/1721.1/65005>
- **S5.** Dugundji and Chen, AFOSR final report (March 1986), DTIC ADA172922, abstract and references.
  <https://apps.dtic.mil/sti/pdfs/ADA172922.pdf> (read from the Internet Archive's copy; DTIC returned 403).
- **S6.** Abang Haji Abdul Majid, "Aeroelastic tailoring of woven cantilevered glass-epoxy plate-like aircraft wing",
  PhD thesis, Universiti Putra Malaysia (2008), FK 2008 56, pp. iii–iv. <https://core.ac.uk/download/42992647.pdf>
  (read from the Internet Archive's 2019 copy, 25 pages; CORE returned 403).
- **S7.** Tang, Henry, Dowell, AIAA J. 37(11) (1999) 1355–1362, abstract only. <https://doi.org/10.2514/2.627>
- **S8.** Kuttenkeuler, "Aircraft composites and aeroelastic tailoring", doctoral thesis, KTH (1998), record only.
  <http://urn.kb.se/resolve?urn=urn:nbn:se:kth:diva-2641>
- **S9.** Martin, NACA TN 4197 (1958), eq. 18, as [the flutter page](../physics/flutter.md) states it.
