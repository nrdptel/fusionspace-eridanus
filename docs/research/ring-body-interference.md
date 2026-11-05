# Does a body raise or cut the lift of a tube beside it?

HPR Sim flies each tube of a [tube fin](../physics/aero.md#tube-fins) set as an isolated ring wing, with no interference
from the body ([ADR-099](../decisions/0099-tube-fins-flown-as-ring-wings.md), the tube-fin model's record).
[#234](https://github.com/nrdptel/fusionspace-eridanus/issues/234) estimates, by slender-body theory, that a set of
N tubes beside a body lifts 1.13 to 1.96 times N isolated rings. The only measurements found so far are rings *around*
a body, and those lose lift ([tube-fin data](tube-fin-data.md) section 2). This note reads the three sources that note
could not reach on 2026-10-18: Weissinger's "Ring plus body" (AFOSR TN 60-343), the Army ringtail report AD0467829 and
the 1964 ring-wing bibliography AD0452725. It asks whether any of them gives the lift of a ring beside a body, and
whether #234's sign holds. "Inference" marks this note's own arithmetic; the scripts are kept under the gitignored
`refs/research/ring-body-interference/scripts/`.

**In short: none of the three treats a ring beside a body, and nothing measured does; the sign of #234's rise rests on
theory alone: two agreeing computations, and a published theory for closed bodies with the same sign.**

- **Weissinger's TN 60-343 is a ring around a central body**, not beside one, by the 1964 bibliography's summary of
  the same report. None of its 25 entries (of 69) that pair a ring with a body puts the body beside the ring.
- **AD0467829 measured zero-lift drag only.** Its companion data report shows the 2.50-caliber rings were run only at
  Mach 1.75 to 4.5. The Mach 0.8 gap trend stays as before: a concentric ring loses less lift as the gap grows.
- **Slender-body theory for bodies beside a body gives a rise.** Ishikawa and Makino's (1978) results for two closed
  boosters touching a core imply it on average over bank angle. This note's panel solve reproduces #234's factors.
  The concentric loss is the same theory on a different shape, not a contradiction.
- **The size is not bounded.** Standing three tubes off the body by a quarter of their radius removes 63% of the
  theory's rise: it lives where the body's boundary layer sits. Avecilla's tube-fin CP data can't tell the factor from
  none. Applying it would move the CP aft, the side that flatters the margin.

Confidence: high on what each source covers and that the inviscid factor is computed right; low on its size in a real
flow. This moves #234 (sign: theory-supported, unmeasured) and the wording of the tube-fin CP record,
[ADR-102](../decisions/0102-tube-fins-centre-of-pressure-measured-against.md).

## 1. Weissinger's "Ring plus body" is a ring around its axis

Hoerner and Borst list "Weissinger, Ring Plus Body, AFOSR TN 60-343 (1960)" as reference (14,e) of their chapter on
small-aspect-ratio wings (p. 17-11). HathiTrust's catalog record for the report gives its real title, *Theoretical
Investigations on Ring Airfoils*, 1960, "AFOSR TN 60-343" on the cover, contract AF 61(514)-1207, 68 leaves of text,
23 of tables and 22 of diagrams [S1]. The scan is search-only and its in-text search did not load: not read.

The 1964 bibliography summarizes a report of the same title, year and contract as entry 65 (AD-234 517, p. 26) [S2]. It
studies the interference between a ring airfoil and a *slender central body*, with simple formulas for lift and moment
and numbers for ellipsoids (Inference: the same report, by title, year and contract). Weissinger's earlier notes in the
same series treat the same layout:

- **AFOSR TN 58-224 and 59-226 (entries 63 and 64, p. 26):** a central body's effect on a ring's lift and moment,
  then slender bodies of finite length, narrow against the ring, and a ring with a strut.
- **DVL Report 140 (entry 21, pp. 9 to 10):** a ring wing with a fuselage on the ring's axis, four body shapes.

So Weissinger's body is always coaxial. Hoerner and Borst also cite (14,e) for rings of d/c from 1/3 to 3 (p. 17-10;
Fig. 16, p. 17-11), the range of Fletcher's five, their (14,c): the report may compare theory with Fletcher (Inference).

## 2. The 1964 bibliography: no ring beside a body

AD0452725 (Terry, Redstone Scientific Information Center report RSIC-285, September 1964) lists 69 entries from 1950 to
June 1964 (Foreword, p. iii) [S2]. This note split its text into entries and flagged each whose summary names a body,
fuselage or central body: 27 hits, two of them not ring-body work (entries 35 and 66). Of the 25 left (Inference,
counted 2026-11-04):

- 19 put a complete ring around a body on its axis: theory by Weissinger (4), van der Walle (4), Beane, Maeder,
  Zandbergen, Ehlers, Moiseyenko, Pivko and Portnoy; tests by Brown University (2), NASA TN D-1272 and Nordström.
- 5 study half or partial rings around a body, for supersonic drag (Douglas, MIT, Rand twice, RAE). One more Rand memo
  names "ring-body configurations" without saying which.
- None places a ring or tube beside a body. The summary (p. 1) says the ring-body work aimed at cutting wave drag. Its
  tests are transonic or supersonic: Brown to Mach 1.10 (entry 6, p. 3), NASA at Mach 2.2 (entry 44, p. 19) and
  Nordström at Mach 2 and 3 (entry 45, p. 20).

## 3. AD0467829 and the Army ringtail series

**AD0467829 is Connolly, RD-TR-65-8 (28 May 1965), a drag report** [S3]. It reports zero-lift foredrag and body base
drag for rings 1.25 to 2.50 calibers across and 0.60 to 1.50 calibers long, at Mach 0.8 to 4.5 (abstract, p. ii).
It gives no normal force; the stability data are its references 5 and 6 (pp. 1 to 2, 29). The body is 1.15 in across,
10 calibers long, at Reynolds numbers of 3 to 5 million (p. 3). On the smallest unflared ring (T14, 1.316 calibers)
the gap is "almost two-thirds filled with boundary layer" (p. 9).

**The data report is Rubin, RD-TR-65-6, DTIC AD0464027 (15 April 1965)** [S4]. It tabulates normal force and moment for
every configuration in 52 tables (Tables 3 to 54, p. 3). Its Table 1 (p. 8) shows where each ring ran:

- The 2.50-caliber rings T10 and T11 ran only in the Ballistic Research Laboratories tunnel, Mach 1.75 to 4.5.
- Rings of 1.25 to 2.0 calibers (T1 to T5, T7, T12 to T14) ran at the Arnold transonic tunnel from Mach 0.8,
  with a stated normal-force accuracy of ±0.01 there (p. 2). No 2.50-caliber ring was measured below Mach 1.75.

**The gap trend at Mach 0.8 shows the concentric sign.** Brazzel's measured increments from the same tests [S5], set
against an isolated ring's slope in [tube-fin data](tube-fin-data.md) section 2, put rings with a 0.125 or 0.25
caliber gap at 0.24 to 0.39 of it, and rings with a 0.5 caliber gap at 0.48 to 0.69. Brazzel writes that where the
boundary layer is thick against the gap, more chord need not add normal force (p. 4). The loss shrinks as the ring
moves away from the body. At supersonic speeds his rings gave 1.3 to 2.0 times the normal-force slope of flat fins of
equal chord and total span, body interference included (p. 5, Fig. 15). Neither result places a ring beside the body.

## 4. Bodies beside a body: slender-body theory gives a rise

**Ishikawa and Makino (1978)** solved a core with two boosters touching it on opposite sides, all closed bodies of
revolution, by slender-body theory and Milne-Thomson's circle theorem [S6]. The core's radius is 4% of its length and
the boosters' 3% (Fig. 3, p. 178); the boosters touch the core (p. 180). At 5° angle of attack, on the core's base area,
the vehicle's normal-force coefficient is 0.601 − 0.405 cos 2φ, φ the bank angle (p. 179): 0.196 with the bodies in
line along the crossflow (φ = 0), 1.006 side by side across it (φ = 90°), against 0.174 for the core alone. The paper
compares only φ = 0 with the core alone, calling them nearly equal; the comparisons below are this note's.

- **Against isolated bodies (Inference).** Slender-body theory gives each closed body alone a normal force in
  proportion to its base area, so the three alone sum to 0.174 × (16 + 9 + 9)/16 = 0.370. The combination gives 0.53
  times that in line, 2.72 times across and 1.63 times on the bank-angle average. Shielding cuts lift in line; a wider
  section raises it across, and the rise wins on average.
- **An independent check (Inference).** This note's source-panel solve of the same cross-section gives 1.07 and 6.6 to
  6.9 times the core alone in line and across (0.50 and 3.1 to 3.2 times the isolated sum). That is 5% lower in line and
  14% to 19% higher across than the paper, whose potential stops at seventh order (p. 178) and which set the axes
  about 1.2% closer than the bodies' radii would (p. 180).

**#234's factors, re-derived.** A tube's lift in slender-body theory is the jump in the section's apparent mass at its
leading edge, plus the air its bore carries. The panel solve (validated below) gives the set's lift over N isolated
rings for #234's probes: a body of 12.5 mm radius, tubes with a gap of 0.005 tube radii (Inference):

| probe | tubes | #234 | this note | at 0.0005 r: #234 | this note |
|---|--:|--:|--:|--:|--:|
| 6 mm tubes, 5.7 mm bore | 3 | 1.96 | 1.971 | none given | 2.065 |
| 6 mm tubes | 4 | 1.86 | 1.875 | none given | none run |
| 6 mm tubes | 5 | 1.72 | 1.730 | 1.79 | none run |
| 6 mm tubes | 6 | 1.57 | 1.576 | 1.62 | 1.633 |
| 6 mm tubes | 8 | 1.30 | 1.303 | none given | none run |
| six 12.5 mm tubes, touching | 6 | 1.13 | 1.135 | 1.15 | 1.153 |

- **Validation (Inference).** One circle comes out 0.34% high at 400 panels, 0.09% at 1,600. Two touching circles
  tend to the classical π²/3 − 1 per circle across their line (2.24 at a gap of 0.001 radii) and π²/6 − 1 along it
  (0.646). The gap to #234, under 1%, is the panel error's size.
- **Why the concentric ring loses (Inference).** Put a body inside a ring and the body's own crossflow disappears into
  the ring's: the ring adds 2π(R² − a²), a factor 1 − (a/R)² below an isolated ring. Put a tube beside the body and
  the section grows sideways, which adds apparent mass. The same theory gives both signs; the data on rings around a
  body test the first and say nothing on the second.

**The rise lives near contact (Inference).** The factor against the tube's gap from the body, 6 mm tubes:

| gap, tube radii | contact (extrapolated) | 0.005 | 0.05 | 0.25 | 0.5 | 1 | 3 |
|---|--:|--:|--:|--:|--:|--:|--:|
| three tubes | about 2.1 | 1.97 | 1.72 | 1.40 | 1.26 | 1.13 | 1.02 |
| six tubes | about 1.66 | 1.58 | 1.42 | 1.23 | 1.15 | 1.07 | 1.01 |
| six 12.5 mm tubes, touching | about 1.16 | 1.14 | 1.09 | 1.04 | 1.02 | 1.01 | 1.00 |

The factor never falls below 1, so the inviscid sign is robust. Its size is not: it falls short of its contact value
by an amount that grows like the square root of the gap, and standing three tubes off by 0.25 tube radii removes 63% of
the contact rise. The near-contact sliver that carries it sits in the body's boundary layer. The Army's concentric rings
measured 0.47 to 0.90 of the theory's 1 − (a/R)², a further loss that source leaves without a named cause
([tube-fin data](tube-fin-data.md) section 2). How much of the rise survives is unbounded.

## 5. The one hobby measurement can't separate the factor from none

Avecilla's blower-jet CP brackets on BT-20 models ([NAR tube-fin reports](tube-fin-nar-reports.md) section 1) are the
only measured tube-fin CPs found. On his geometry (tubes the body's size) the panel solve gives 1.72 for three tubes and
1.16 for six at contact (Inference). Scaling HPR Sim's tube slope by those, with its CP kept, on that note's rebuild:

- Base geometry: the shifts from flat fins become −0.34, −0.51 and +0.32 in for models B, C and D, against
  −0.27, −0.53 and 0.00 measured. The RMS miss falls from 0.28 to 0.19 in, and C lands inside its interval.
- Over five readings of the unstated geometry (as built; tubes 0.15 in fore or aft; fin sweep halved or zero), the
  RMS miss is 0.17 to 0.43 in with the factor and 0.26 to 0.35 in without. The two ranges overlap, so the one
  measurement leans neither way. Its Reynolds number, near 10⁴, would also thicken the layer in the corners.

## What this suggests

- **Record #234's sign as theory-supported and unmeasured.** Three results agree: #234's multipole solve, this
  note's panel solve and, for closed bodies, Ishikawa and Makino. No source gives a loss for open tubes beside a body
  on average over bank angle (closed bodies in line lose). ADR-102 can cite this note where it says "not checked".
- **Keep HPR Sim's isolated-ring set for now.** Applying the factor moves the CP aft and raises the margin, the side
  that flatters a rocket, by a size no measurement bounds. Leaving it out keeps the margin cautious.
- **If the factor is ever applied, cap it and pin its direction with a test.** The factor at a gap of 0.25 to 0.5
  tube radii (1.26 to 1.40 for three 6 mm tubes, 1.15 to 1.23 for six) keeps the theory's shape and drops the
  near-contact part the boundary layer likely eats. That cap is a judgment, not a measured bound.
- **Stop chasing Weissinger and AD0467829 for this question.** Neither holds a ring beside a body.
- **The measurement that would settle it** is a tunnel or blower test of one tube set on a body, with the set's force
  taken against the same tubes held away from the body. Avecilla's rig with a balance would do it.

## Open questions

- Does any measurement exist of a tube, duct or nacelle beside a body with the body-off force for comparison? A
  Cranfield report of three closely spaced bodies at subsonic speed (College of Aeronautics, PDF file 9114) measured
  pressures on closed bodies [S7]; its PDF was blocked on 2026-11-04 and has no archived copy reached.
- How much of the near-contact rise survives a real boundary layer, and how does it scale with Reynolds number?
- Does the rise carry over to short tubes (L/d near 3), whose isolated slope sits well below the long-ring limit?
  #234's factor is a slender-body ratio; its value for a short tube is unbounded.
- F. M. Martin's AFATL-TR-71-69 (1971), cited by [S6], may hold measured cases; DTIC was down on 2026-11-04.

## Sources

DTIC reports are US government works (the three Army test reports approved for public release): cached under the
gitignored `refs/research/ring-body-interference/dl/`. The journal paper and the books are copyrighted: cited, numbers
restated, the paper cached privately. All read 2026-11-04.

1. **S1.** HathiTrust catalog record, J. Weissinger, *Theoretical Investigations on Ring Airfoils*, AFOSR TN 60-343,
   1960. <https://catalog.hathitrust.org/Record/001526478>. Search-only scan, not read.
2. **S2.** J. E. Terry, *Aerodynamic Characteristics of Ring Wings: A Bibliography*, RSIC-285, US Army Missile
   Command, 1964, DTIC AD0452725, <https://archive.org/details/DTIC_AD0452725>. pp. iii, 1, 3, 9 to 10, 19 to 20, 26.
3. **S3.** P. R. Connolly, *The Zero-Lift Foredrag and Body Base Drag Coefficients of a Series of Ring Tail-Strut-Body
   Configurations at Mach Numbers from 0.80 to 4.50*, RD-TR-65-8, 1965, DTIC AD0467829,
   <https://archive.org/details/DTIC_AD0467829>. pp. ii, 2, 3, 9; Fig. 2.
4. **S4.** D. V. Rubin, *Tabulated Stability Data for a Series of Ring Tail Body Configurations at Mach Numbers from 0.8
   to 4.5*, RD-TR-65-6, 1965, DTIC AD0464027, <https://archive.org/details/DTIC_AD0464027>. pp. 2, 3, 8, 9.
5. **S5.** C. E. Brazzel, J. H. Henderson, J. C. Craft, *Longitudinal Stability Characteristics of a Series of
   Ringtail-Body Combinations at Mach Numbers of 0.8–4.5*, RD-TR-65-7, 1970, DTIC AD0871756,
   <https://archive.org/details/DTIC_AD0871756>. pp. 4, 5; Figs. 14, 15.
6. **S6.** Y. Ishikawa, M. Makino, "A Theoretical Study of Multiple Body Interference of Rocket", *Journal of the Japan
   Society for Aeronautical and Space Sciences* 26(290), 1978, pp. 172 to 180, doi:10.2322/jjsass1969.26.172,
   <https://www.jstage.jst.go.jp/article/jjsass1969/26/290/26_290_172/_article/-char/en>. Figs. 3, 8; Table 3.
7. **S7.** D. I. T. P. Llewelyn-Davies, *An Investigation into the Interface between Three Closely Spaced Axi-Symmetric
   Bodies at Subsonic Speed*, Cranfield College of Aeronautics (PDF file coareport9114; undated on the record),
   <https://reports.aerade.cranfield.ac.uk/handle/1826/223>. Abstract only.
8. **S8.** S. F. Hoerner, H. V. Borst, *Fluid-Dynamic Lift*, 1985, pp. 17-10, 17-11, ref. (14).
