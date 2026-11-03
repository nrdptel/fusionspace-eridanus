# How much a glued or through-the-wall fin root lowers flutter speed

**In short: no measurement was found of a fin glued to the outside of a tube beside the same fin clamped, in frequency
or in flutter onset, so the glued root's effect stays unbounded (evidence row G22, issue #675).** What exists:
- **One through-the-wall root measured against a clamp** rang higher, not lower: a carbon fin epoxied through a carbon
  airframe to its motor tube rang 1.17 times the clamped fin's first bending frequency (1.12 modeled; S1). Bending
  only, one build; torsion, which drives flutter, was not measured.
- **Roots held over part of the chord** lost up to 41 to 46% of their torsion frequency (S4, S6). The one such plate
  tested in a tunnel lost 23% of its onset speed, about as the square root of its torsion loss (9% if the report's
  unstated reference frequency was the calculated one; S6).
- **A root pitch spring** (all-movable wings, S7): flutter dynamic pressure rose almost linearly with stiffness; near
  the top of the tested range, halving it cut the equal-air onset speed to 0.63 to 0.65 on one wing (0.44 to 0.51 on
  another). Softening a root lowers the onset while it moves torsion toward bending, and raised it 1.57 times once past
  coincidence (S8). A root stiffened in bending only lowered the onset about 4 to 9% (S9).

A frequency ratio is no safe stand-in for the onset ratio: it overstated the loss on one plate (S6), understated it on
the ASROC fin (S5). Martin's speed (NACA TN 4197, eq. 18) and a plate's torsion frequency both scale with $\sqrt{G}$,
so a root lowering torsion by $r$ would scale his speed by $r$ (**Inference**). Builds on
[fin-flutter-tests](fin-flutter-tests.md) §4 and [flutter-measured-onsets](flutter-measured-onsets.md) (all clamped).
Sources read 2026-11-03; ratios are our arithmetic (**Inference** where a step is ours); "equal-air speed" is
$V \propto \sqrt{q}$ at one Mach number.

## 1. Real roots measured against a clamp or a rigid model

**Through-the-wall carbon fin (Qaumi and Hashemi 2023, S1).** A flown student booster, all carbon. Each fin is 12 plies
at alternating angles (§3.1.2, p. 5); its 1 in tab passes through the booster tube, sits on the motor tube and meets
two centering rings, every contact epoxied with an epoxy and carbon-fiber mix (§3.1.4, p. 8, Fig. 6). Span, chords and
thickness are not in the text. A separate fin of the same design was clamped between aluminum extrusions on a table
(§3.2.2, p. 10). Hammer taps, laser vibrometers, five trials each; first mode only, bending (pp. 15–17).

| Setup | Measured, Hz (sd) | Ansys model, Hz | Source |
|---|---|---|---|
| Fin clamped on the bench | 80.13 (0.07) | 84.05 | Table 10, pp. 15–16; Table 13, p. 18 |
| Fin in the airframe | 93.34 (0.41) | 94.01 | Table 12, p. 17; Table 13, p. 18 |
| Ratio, mounted over clamped | 1.165 | 1.118 | computed |

- The four fins in the airframe model ring at 88.5 to 104.2 Hz by orientation (Table 6, p. 13). The tested fins are
  different parts; the clamped one measured 4.7% below its model, which the authors put on too little fiber near its
  edges and too little epoxy elsewhere (p. 19). Since the model with a rigid clamp also rings 11% below the bonded
  model, the rise comes from the bonded tab and root, not only a soft bench clamp (**Inference**).

**Bolted fin on a bracket, FalconLAUNCH V (Simmons 2009, S2).** Four bolts to a bracket fixed to an optical table
(pp. 33–35). Measured (Table 1, p. 33, quoting AIAA-2009-515): first bending 62 Hz, first torsion 212 Hz. A solid
model with the bolts merged and the bracket base fixed: 63.3 and 215.4 Hz (Table 2, p. 40). Measured over model: 0.98
for both, the bolted joint and other model differences together (**Inference**). Not a ratio to a clamp.

**Bolted cast wing on a bench (Vinacco 2010, S3).** A357-T6 aluminum, main section 9.2 in root, 3.5 in tip, 7.0 in
semi-span, 0.75 in thick, plus a winglet (Table 6, p. 54), bolted with its bracket to a bench. Measured 96.9, 205.6,
265.6 Hz; fixed-bracket model 107.6, 201.6, 374.3 Hz (Table 8, p. 60): ratios 0.90 bending, 1.02 torsion, 0.71 third
mode. Far from hobby size.

**ASROC fin, root filler butted on rather than integral (Levey and Hanson 1958, S5).** 0.4-scale 2024 aluminum fins,
0.050 in (1.27 mm) thick, bolted between steel plates (pp. 3–4). Table I(b) (p. 7): f1 128 and 128 Hz, f2 184 and 197,
f4 336 and 363 (butted BF-14, integral BF-15): ratios 1.00, 0.93, 0.93. Its onset fell about 12% in equal-air speed
([fin-flutter-tests](fin-flutter-tests.md) §4), more than the 7% frequency loss: the flattering side for a frequency
proxy (**Inference**). Cutting away the forward 2.15 in of a full clamp (BF-13 to BF-13A) moved f1, f2, f4 by +2%, 0%,
−3%, yet BF-13A fluttered and BF-13 did not (Table I(b)): frequencies missed it.

**Partial root support on a steel plate (Sepaha and Durvasula 1972, S4).** Mild steel, 1/16 in (1.6 mm) thick, 12 in
root, 6 in tip, 12 in span, leading edge swept 30°, root strip bolted and clamped (Fig. 2, p. 46; p. 44); partial
support made by cutting along the root (p. 44). Measured by resonance (Figs. 3–9, pp. 47–53); mode 2 has a spanwise
node line, so it is roughly first torsion. Ratios computed.

| Root held | f1, Hz | f2, Hz | f1 over clamped | f2 over clamped |
|---|---|---|---|---|
| all | 18 | 59 | 1.00 | 1.00 |
| 75%, leading edge 25% free | 18 | 59 | 1.00 | 1.00 |
| 50%, leading edge 50% free | 17 | 42 | 0.94 | 0.71 |
| 75%, trailing edge 25% free | 16 | 50 | 0.89 | 0.85 |
| 50%, trailing edge 50% free | 16 | 35 | 0.89 | 0.59 |
| middle 50% | 14.5 | 50 | 0.81 | 0.85 |
| 25% | 12 | 36 | 0.67 | 0.61 |

Holding part of the chord (tabs, a short rail, two bolts) cost up to 41% of torsion; a free trailing edge mattered far
more than a free leading edge (pp. 46–47). No onset was measured.

## 2. Flutter onsets with a root that is not a rigid clamp

**Partial clamp, NASA TM 101547 (Doggett and Soistmann 1989, S6), re-read for its frequencies.** One 45° aluminum
delta, 0.064 in (1.63 mm) thick, 22.8 in root chord, its clamp sawed to 74, 47, 34 and 21% of the chord (p. 2,
Fig. 3), Mach 0.10 to 0.35 (p. 6). Fig. 10 (p. 5) and Fig. 15(a) (p. 8) read by pixels on their axis ticks (about
±0.3 Hz, ±0.01); onset ratios from [fin-flutter-tests](fin-flutter-tests.md) §4.

| Clamped | $f_{b1}$, Hz | $f_t$, Hz | $f_f/f_t$ | Onset over clamped | $f_t$ over clamped | Exponent $n$ |
|---|---|---|---|---|---|---|
| 100% | 7.0 | 44.7 | 0.612 | 1 | 1 | |
| 74% | 6.5 | 38.9 | 0.742 | 0.92 | 0.870 | 0.58 |
| 47% | 6.3 | 30.6 | 0.793 | 0.83 | 0.685 | 0.48 |
| 34% | 5.4 | 27.3 | 0.691 | 0.83 | 0.611 | 0.38 |
| 21% | 5.0 | 24.2 | 0.809 | 0.77 | 0.541 | 0.43 |

- $n$ = ln(onset ratio)/ln($f_t$ ratio), computed: the onset fell about as the square root of the torsion frequency
  (mean 0.47). First bending fell to 0.71 at 21%, close to the onset's 0.77. Scaling Martin's speed by $f_t$ would
  predict 0.54 at 21%, 0.70 of the measured onset: the safe side on this plate (**Inference**).
- Whether the report's reference torsion frequency is measured or calculated is unstated (p. 6). With Fig. 10's
  calculated torsion frequencies in place of the measured ones, the measured index gives 0.91 at 21%, not 0.77
  (**Inference**). The report's lifting-surface theory, with finite-element modes of each root, gives 0.86 to 0.95 at
  21% if its reference is the calculated frequency (**Inference**).

**All-movable wing on a root pitch spring (Hanson 1959, S7).** Fighter wing panels at 0.0333 scale, aspect ratio 1.71,
taper 0.246, an aluminum box spar with balsa and silk (pp. 5–6), each on a mount pitching on a leaf spring plus an
adjustable spring, stiffness $K$ measured by optical lever (p. 7). Langley 9- by 18-inch supersonic flutter tunnel,
density raised at fixed Mach number until flutter (p. 4). Table VII (p. 27): 25 flutter points on wings W1 and W2.

| Mach | $K$, in-lb/rad | $q_f$, lb/ft² | Points | Slope of log $q_f$ on log $K$ | Equal-air speed at $K/2$ |
|---|---|---|---|---|---|
| 1.30 | 2,535–11,100 | 463–3,485 | 8 | 1.34 | 0.63–0.65 |
| 1.64 | 2,535–9,770 | 555–3,458 | 8 | 1.34 | 0.63–0.65 |
| 2.00 (W1) | 3,955–7,550 | 1,370–3,300 | 3 | 1.34 | 0.62–0.63 |
| 2.00 (W2) | 3,955–6,525 | 1,475–3,780 | 4 | 1.95 | 0.44–0.51 |

- Slopes and the $K/2$ column are fits (power law and linear), computed near the top of the tested range; lower in it
  the linear fit drops further. Measured pairs at Mach 1.30: $K$ 11,100 to 6,010 gave equal-air speed 0.63 times;
  9,710 to 4,085 gave 0.57. The report: $q_f$ "varied almost linearly with the pitch stiffness" except at very low $K$
  (p. 1).
- The panel's listed modes barely moved while $K$ changed 4.4 times; the flutter frequency fell from 340 to 176 Hz,
  close to $\sqrt{K}$. A vibration test that misses the root mode would miss what set the onset (**Inference**).
  Raising the mount's pitching inertia 2.3 and 2.9 times at fixed $K$ lowered $q_f$ about 43% and 50% (pp. 11–12).
- The flutter coupled the panel's second bending with root pitch (p. 8); a glued plate would also twist along span.

**Elevators on an actuator spring: coincidence sets the sign (Morgan, Figge and Presnell 1958, S8).** Half-scale
missile elevators on spindles, pitch springs changed, bending nearly fixed at 70 Hz (pp. 4–5; Table IV, p. 15).
Table V (pp. 16–17), plan-form 2: with pitch at 68.5 Hz (bending over pitch 1.02) flutter stopped at $q$ 364 to 372
lb/ft² at Mach 1.60 and 608 at Mach 2.21. A spring about 45% softer (pitch 50.9 Hz, ratio 1.41; **Inference**, same
pitch inertia) raised it to 1,492 at Mach 2.21: 1.57 times in equal-air speed. A stiffer spring on another model
(ratio 0.91) reached 925 at Mach 1.60, 1.59 times; at ratio 0.82 nothing fluttered up to 2,115. The report: flutter $q$
at least doubled at every Mach number when the ratio changed from 1.02 to 0.82, 0.91 or 1.41 (p. 10).

**Root stiffened in bending only (Cole 1989, S9).** A rigid wing (30 in semispan, 20 in chord) on a flexible shaft in
the Langley Transonic Dynamics Tunnel; a beam pressed on the shaft raised first bending from 3.0 to 3.3 or 3.5 Hz, with
torsion at 13.8 to 14.1 Hz (Table 4, p. 8). Flutter $q$ fell to about 0.84 to 0.92 of the basic wing at Mach 0.74 to
0.85 (Fig. 13, p. 15, read by eye): equal-air speed 0.91 to 0.96. A root soft in bending but stiff in torsion is not
the flattering case (**Inference**).

## 3. A root spring on a uniform bar and beam (a model, **Inference**)

A uniform torsion bar with a free tip and a root rotational spring $k$ has $\mu \tan\mu = K$, $K = kL/GJ$, and
$f/f_{\text{clamped}} = \mu/(\pi/2)$; a uniform beam with a root spring ($K = kL/EI$) has first-mode ratio
$(\lambda/1.8751)^2$. Computed (script cached under `refs/`):

| $K$ | 1 | 3 | 10 | 30 | 100 | 1,000 |
|---|---|---|---|---|---|---|
| Torsion $f/f_c$ | 0.548 | 0.759 | 0.910 | 0.968 | 0.990 | 0.999 |
| Bending $f/f_c$ | 0.443 | 0.651 | 0.844 | 0.939 | 0.981 | 0.998 |

For a fin glued to a thin tube, $k$ comes from the wall's local bending and the fillets; no measurement was found.

## 4. Hobby evidence and vendor numbers

- **Forum threads** (search summaries only; the threads returned 403) call through-the-wall roots stronger; none gives
  speed, geometry, material and root for one failure, nor does any IREC or university report found. FalconLAUNCH V's
  bolted carbon fins sheared near Mach 4 (S2, pp. 2, 34, 44), blamed on a flutter-formula error, not the root.
- **Apex Rocket Simulator's "Fin Flutter" page** lists a root fixity factor from 0.70 (no fillet) to 0.99 (typical
  fillet) with no source or test: a vendor estimate, not evidence.

## What this suggests

- **Mounting:** S1 shows no bending penalty for one carbon through-the-wall build; torsion is unmeasured, so no factor
  either way is supported, and a surface-glued root has none at all. Keep G22's gap open on the flutter page.
- **No measured factor is a bound:** partial clamps lowered the onset to 0.77 at most (S6), a root spring at half
  stiffness to 0.63 (S7), S8's softening onto coincidence below 0.42 (none fluttered at 2.4 times that speed). One
  soft-root factor taken from the partial clamps alone would be on the flattering side next to S7 and S8.
- **A sign test:** a root-softening factor should never raise a predicted flutter speed (the flattering
  side), even though S8 shows a real onset rising past coincidence.
- **A frequency input errs safe on one plate only:** if a user ever enters a measured torsion frequency of a mounted
  fin, scaling Martin's speed by its ratio to the clamped value erred safe on S6 and on the flattering side on S5.
- **A tap test closes the gap cheaply:** a hobby fin tapped clamped, then glued to cardboard, phenolic and fiberglass
  tubes with and without fillets (bending and torsion), would give the missing ratio.

## Open questions

- A surface-glued fin on cardboard, phenolic or fiberglass tube against the same fin clamped: none found, frequency or
  onset. The tube wall's rotational stiffness under a glued root, what fillets add, and torsion on any mounted fin
  (S1 measured bending only) are unmeasured.
- TM 101547's reference torsion frequency, measured or calculated, moves its 21% onset ratio from 0.77 to 0.91.
- Unread leads: Hanson NASA TN D-984 (1961, panels on leaf springs); Boswinkle and Morgan NACA RM L57D23c; Gaukroger's
  1953 delta with an all-moving tip (ARC); Ruhlin and Tuovila (NTRS 19980227183, canards at reduced pitch stiffness,
  no flutter); AIAA 2025-99192 (a student rocket with strain-gauged fluttering fins; PDF 403).

## Sources, all read 2026-11-03

S1 is CC BY 4.0, S2 and S3 theses approved for public release, S5 to S9 US government works: all may be cached and
cited. S4 is "All rights reserved": cite its numbers only.

- S1. Qaumi, T.; Hashemi, S. M. "Experimental and Numerical Modal Analysis of a Composite Rocket Structure."
  *Aerospace* 10, 867 (2023). <https://doi.org/10.3390/aerospace10100867>. CC BY 4.0.
- S2. Simmons, J. R., III. *Aeroelastic Optimization of Sounding Rocket Fins.* AFIT thesis (2009), DTIC ADA502110,
  <https://scholar.afit.edu/etd/2426>; read in an Internet Archive copy (DTIC returned 403).
- S3. Vinacco, M. J. *ExFiT Flight Design and Structural Modeling for FalconLAUNCH VIII Sounding Rocket.* AFIT thesis
  (2010), DTIC ADA517570, <https://scholar.afit.edu/etd/2058>; Internet Archive copy.
- S4. Sepaha, S. P.; Durvasula, S. "Vibration of Trapezoidal Cantilever Plates with Partial Root Chord Support."
  *J. Indian Inst. Sci.* 54(1), 43–54 (1972). <https://journal.iisc.ac.in/index.php/iisc/article/view/3020>.
- S5. Levey, G. M.; Hanson, P. W. *Experimental Investigation of Flutter and Divergence Characteristics of the
  Rocket-Motor Fin of the ASROC Missile.* NACA RM SL58H08a (1958). <https://ntrs.nasa.gov/citations/19710075830>.
- S6. Doggett, R. V., Jr.; Soistmann, D. L. NASA TM 101547 (1989). <https://ntrs.nasa.gov/citations/19890010723>.
- S7. Hanson, P. W. NASA Memo 10-16-58L (1959), Table VII p. 27. <https://ntrs.nasa.gov/citations/19660024794>.
- S8. Morgan, H. G.; Figge, I. E.; Presnell, J. G., Jr. NACA RM L58B20 (1958), Tables IV–V.
  <https://digital.library.unt.edu/ark:/67531/metadc53088/>.
- S9. Cole, S. R. NASA TM 4116 (1989), Table 4 p. 8, Fig. 13 p. 15. <https://ntrs.nasa.gov/citations/19890014076>.
- Apex Rocket Simulator, "Fin Flutter," <https://apexrocketsim.com/fin-flutter/>: cited only, no test behind it.
