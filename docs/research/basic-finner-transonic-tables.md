# The Basic Finner's transonic tables and its fin chord

[transonic-normal-force.md](transonic-normal-force.md#the-basic-finner) read the Basic Finner's normal-force slope
(C_Nα) and center of pressure (CP) from Mach 0.5 to 1.3 off ARL-TR-6725 Fig. 5 by pixel calibration, because the
report behind the figure, DRDC TM 2002-136, was not reached. Issue #619 would turn those readings into a fixture.
This note reports what that report and its Air Force Finner companion (DRDC TM 2002-008) tabulate, gives their values
at Mach 0.78, 0.92, 1.07 and 1.20, and settles the fin chord from the shape's first drawing.

**In short: both reports were reached, and they tabulate every point; the chord is 1.0 caliber.**

- **The tables exist.** DRDC TM 2002-136 (85 pages) lists the tunnel's C_Nα, C_mα and CP for all 53 runs, at 0° and
  45° roll, Mach 0.496 to 4.101 (Tables 12 and 13), the range's fits (Tables 9 and 10, standard deviations in Tables
  6 and 7) and each Mach number's Reynolds number (Table 11). It describes no wall correction.
- **The pixel readings hold.** Every reading in the earlier note lies within 0.2 per rad of a tabulated point, inside
  its stated ±0.3, and every CP within 0.04 caliber. ARL plotted the 0°-roll tunnel runs (**Inference**).
- **The peak's height depends on roll and is loose in the range.** From Mach 0.79 to 1.07 the tunnel's C_Nα rises
  +21.6% at 0° roll and +12.6% at 45°. The range's 21.2 per rad at Mach 1.105 carries ±16% (±3.4 per rad).
- **The chord is 1.0 caliber.** BRL Report 711 (1949), older than every source the DRDC report cites, draws a 0.786 in
  chord on a 0.786 in body. NOL, DREV and DRDC state 1 caliber in text. AEDC alone draws 0.94 caliber, on its own 1.8
  in model, while its text says "approximately one caliber". A 0.94 chord would lower C_Nα by about 2% to 5%.
- **Terms.** Both DRDC reports are Crown copyright and bar commercial use and release to third parties without
  DRDC's consent: whether a fixture may hold their numbers is the maintainer's call.

Confidence: high on the values (read from the scanned tables, checked for internal consistency); medium on what
causes the roll and tunnel-to-range spreads, which the report measures but explains only in part (PDF pp. 27, 33).

## 1. How the report was reached

DTIC answered "Under Maintenance" on 2026-10-19, and DRDC's own hosts (`cradpdf`, `pubs.drdc-rddc.gc.ca`) resolved
but did not answer. The Internet Archive holds 554 captured records of DRDC's catalog with system numbers 517000 to
520999 (counted from its capture index on 2026-10-20). Their titles name two finner reports: system number 518050
is TM 2002-136 and 517505 is TM 2002-008 [D1, D2]. The archive also holds both PDFs from DRDC's PDF server [D1, D2].

Both are image scans with no text layer. This note read them by optical character recognition, then checked every
number it uses against the page image. Two internal checks pass on all 53 tunnel rows: C_mα about the base less
4.5 × C_Nα gives C_mα about the 5.5-caliber center within 0.35, and C_mα about the base over C_Nα gives the CP from
the base within 0.03 caliber (**Inference**, this note's check).

## 2. What TM 2002-136 measured

**Models** [D1 §2, PDF p. 18, printed p. 3]: 30 mm diameter, 10 calibers long, a 20° cone, four "1 cal x 1 cal"
wedge fins 0.08 caliber thick at the trailing edge, leading-edge and nose radius 0.004 caliber, moment center 5.5
calibers from the nose. **All fins were canted 2.0°, in the range and in the tunnel.** ARL's CFD found no
significant difference between canted and uncanted static coefficients [D4 §4.1, PDF p. 25].

**Tunnel** [D1 §4, PDF pp. 27–30]: DRDC Valcartier's indraft trisonic tunnel, a 60 × 60 cm test section, transonic
flow from a perforated chamber with suction, a 19.05 mm six-component balance. Each run swept −10° to +10°; C_Nα
and C_mα are the slopes of a straight line from −5° to +5°. Mach comes from one side-wall tap. The base pressure was
not measured. No wall, blockage or sting correction is described (searched: "wall", "correct", "interfer",
"blockage"). **Inference:** the model and fins block about 0.28% of the test section (body alone 0.20%).

**Range** [D1 §3, PDF pp. 20–24]: eight models, Mach 0.6 to 1.18, angles of attack 0.8° to 3.2° (PDF p. 20). Poor film
cut the usable stations; the report says C_Nα came from multiple fits, "with higher than normal errors" (PDF p. 34),
though Table 6 solves it in single fits for B03, B04, B07 and B10 (14% to 24%). Table 10 adds DREV's rows [D3].

**Reynolds number on length** [D1 Table 11, PDF p. 45]: tunnel 3.9, 4.2, 4.3, 4.2 and 4.4 × 10⁶ at Mach 0.8, 0.92,
1.0, 1.15 and 1.28; free flight 5.5, 6.3, 6.9, 7.9 and 8.8 × 10⁶. ARL's "1.4 to 2.9 × 10⁶" [D4 PDF p. 20] is only the
table's Mach 4.0 and 0.5 ends: the transonic runs sit at 4.2 to 4.4 × 10⁶, unstated by ARL.

## 3. The values at the four Mach points

C_Nα per rad, with each point's own Mach number. Tunnel: Tables 12 (0° roll, PDF p. 46) and 13 (45°, PDF p. 47).
Range multiple fits: Table 10 (PDF p. 44); standard deviations from Table 7 (PDF p. 41).

| Near Mach | Tunnel, 0° roll | Tunnel, 45° roll | Range, multiple fit |
|---|---|---|---|
| 0.78 | 16.2 (0.789) | 15.9 (0.790) | 16.7 ± 17% (0.766) |
| 0.92 | 18.0, 17.9 (0.916, 0.917) | 17.3 (0.916) | 18.3 ± 14% (0.934) |
| 1.07 | 19.5, 19.7 (1.054, 1.076) | 17.9, 17.9 (1.071, two runs) | 18.5 (1.077, DREV); 21.2 ± 16% (1.105) |
| 1.20 | 18.2, 18.3 (1.176, 1.177) | 17.4 (1.157), 17.0 (1.177) | none from 1.105 to 1.293; 18.3 (1.293) |

C_mα per rad about 5.5 calibers, then the CP in calibers from the nose (10 less the tables' CP from the base):

| Near Mach | Tunnel, 0° roll | Tunnel, 45° roll | Range, multiple fit |
|---|---|---|---|
| 0.78 | −44.8; 8.27 | −43.5; 8.24 | −40.8 ± 1%; 7.94 |
| 0.92 | −51.6, −51.4; 8.38, 8.37 | −49.1; 8.34 | −47.2 ± 0%; 8.08 |
| 1.07 | −58.4, −58.7; 8.49, 8.48 | −52.5, −52.3; 8.42 | −52.6 (1.077); 8.35. −52.6 ± 0% (1.105); 7.98 |
| 1.20 | −53.8, −54.2; 8.46 | −50.1 (1.157), −48.9 (1.177); 8.37, 8.38 | −51.0 (1.293); 8.29 |

- **DREV rows.** Table 10's Mach 1.077 and 1.293 rows are DREV-TM-9703's Table IX (18.47, 18.27) [D3]; 0.766,
  0.934 and 1.105 are this program's shots B04 to B10 (Table 7).
- **Table 13's last columns are mislabeled.** Its headers read CP, roll moment, C_lδ; the values sit as roll moment,
  C_lδ, CP. C_mα about the base over C_Nα matches the last column (25.9/13.7 = 1.89 at Mach 0.496): **Inference**.
- **Single fits** [D1 Table 9, PDF p. 43] scatter more: 16.5 to 18.8 per rad at Mach 0.64 to 0.97; 18.7, 18.0 and
  18.5 at 1.056, 1.058 and 1.116; 21.2 at 1.077 (B09) and 1.134 (B10). B09's 21.2 and B05, B06 and B08 are held at
  the multiple-fit value, not fitted, when mean squared yaw was under 1.0 (Table 6, PDF pp. 39–40; p. 24). The text
  puts the subsonic slope near 17 and a maximum of 21 just past Mach 1 (PDF p. 25).

**Against the earlier pixel readings** [transonic-normal-force.md, B2]: 15.0 at Mach 0.5 (table 14.9), 16.1 and 16.7
at 0.78 (16.2, 16.7), 17.9–18.3 at 0.92 (17.9–18.3), 19.7 at 1.07 (19.7), 21.1 at 1.10 (21.2), 18.0–18.2 at
1.18–1.20 (18.2–18.3), 18.3 at 1.29 (18.3). CP 8.14, 8.31, 8.36, 8.47, 8.49 against 8.14, 8.27, 8.38, 8.48, 8.46.
The 45° runs read 17.0 to 17.9 near Mach 1.07 to 1.18, below every reading there, so ARL's tunnel points are the
0° runs (**Inference**).

## 4. What moves the peak, with sign and size

- **Roll orientation: −2% to −9%.** The 45° runs' C_Nα sits below the 0° runs' by 7.4% at Mach 0.5, 1.9% at 0.79,
  3.6% at 0.92, 9.1% at 1.07 and 7.1% at 1.18 (**Inference** from Tables 12–13). The report notes C_mα at 45° about
10% below 0° from Mach 0.5 to 1.05 (PDF p. 33), though its own tables give 2.9% at 0.79 and 4.7% at 0.92. So the rise
from Mach 0.79 to the peak is +21.6% at 0° and +12.6% at 45°; to Mach 1.18 it is +12.7% and +6.9%. HPR Sim's slope
does not depend on roll, so this spread is a measured limit on how closely it can match one orientation.
- **Tunnel against range: about +6% in C_Nα, +12% in C_mα, near Mach 1.08.** At Mach 1.076 the 0° runs read 19.7 and
  −58.7; DREV's range fit at 1.077 reads 18.5 (±2%) and −52.6 [D3] (**Inference**, this note's ratios). The report
  calls the Mach 1.06 point 10% above the others in C_mα (PDF p. 33) and notes the tunnel's Reynolds number is half
  the range's transonically (PDF p. 27). Wall interference, Reynolds number and the sting could each contribute; the
  report bounds none alone, so each is **unbounded** on its own; together they read the tunnel 6% to 12% high.
- **The range's own scatter: ±14% to ±17% in C_Nα.** The low yaw (mean squared yaw 1.1 to 2.3 deg², Table 7) leaves
  C_Nα loose while C_mα is held to 1%; the report blames the range's low CP on this (PDF p. 33). **Inference:** its CP
  inherits C_Nα's error: at Mach 1.105 the arm C_mα/C_Nα = 2.48 calibers carries about ±0.4 caliber, most of its
  0.5-caliber gap to the tunnel's 8.48.
- **Where the peak sits.** Both roll orientations peak at the tabulated Mach 1.054–1.076 and fall by Mach 1.157–1.177;
  the tunnel has no point between 1.076 and 1.157. The peak lies between Mach 0.92 and 1.16, most likely near 1.07,
  with no finer placement from these tables (**Inference**).

## 5. The fin chord

What each primary source states in its text and what it draws (PDF pages):

| Source | Text | Drawing |
|---|---|---|
| BRL 711, 1949 [D5] | wedge fins, 16% thick (p. 29) | 0.786 in chord on a 0.786 in body: 1.00 (Fig. 8, p. 30) |
| NAVORD 6652, 1964 [D6] | "one caliber in chord" (pp. 7–8) | 1.0 (Fig. 1, p. 19, per finner-reference-data.md) |
| AEDC-TR-76-58, 1976 [D7] | "approximately one caliber" (p. 9) | 0.94 d, with d = 1.8 in (Fig. 2, p. 25) |
| DREV-TM-9703, 1997 [D3] | "1 cal × 1 cal" (§3.1, p. 16) | not re-read here |
| DRDC TM 2002-136, 2002 [D1] | "1 cal x 1 cal" (p. 18) | chord 1.0, span 1.0, cone 2.84 (Fig. 1, p. 48) |

- BRL 711's drawing is the oldest found: TM 2002-136 says its model follows BRL 934, BRL 799 and a 1954 Navy
  review (PDF p. 18), all later than BRL 711. Its 16% wedge (a 0.1245 in trailing edge) is twice DREV's 0.08 caliber.
- AEDC's 0.94 d is a model dimension, not the definition: its own text and the four other sources give one caliber.
  Its fins also differ in tip radius (0.002 d against DREV's 0.004 d).
- **Effect of 0.94 instead of 1.0 (Inference).** Fin area falls 6.0%. At Mach 0, a low-aspect fin's lift per area
  rises with aspect ratio as 2πA/(2 + √(4 + A²)); A goes from 2.00 to 2.13, so fin lift falls 1.9%. Supersonic, lift
  per area is nearly independent of A, so fin lift falls close to 6%. Taking the fins' share of the slope as roughly
  85% (an assumption, not sourced), C_Nα falls about 2% to 5% (0.3 to 0.9 per rad near 17). The fins' center moves
  0.045 caliber aft; the whole CP moves about 0.02 caliber.

## 6. The Air Force Finner companion

DRDC TM 2002-008 (Dupuis and Hathaway, May 2002, 79 pages) [D2]: a 2.5-caliber tangent-ogive nose, a 7.5-caliber
cylinder and four clipped-delta fins, cants 0°, 2° and 4°, moment center 4.8 calibers (PDF p. 16); Mach 0.5 to 2.5
in the abstract (p. 5), 0.6 to 2.5 in the introduction (p. 15).
It tabulates range fits only, no tunnel. Multiple fits [D2 Table 8, PDF pp. 37–38]:

| Mach | 0.652 | 0.782 | 0.910 | 0.992 | 1.142 | 1.494 | 2.209 |
|---|--:|--:|--:|--:|--:|--:|--:|
| C_Nα, per rad | 7.15 | 6.46 | 6.52 | 7.16 | 7.71 | 8.86 | 9.19 |
| its standard deviation | 3% | 8% | 13% | 26% | 15% | 11% | 6% |
| C_mα about 4.8 d | −12.613 | −11.417 | −10.932 | −16.968 | −16.563 | −13.595 | −6.544 |
| CP from nose, d (Inference) | 6.56 | 6.57 | 6.48 | 7.17 | 6.95 | 6.33 | 5.51 |

- C_mα's standard deviations are 0% to 1%. **Inference:** C_mα jumps 55% from Mach 0.910 to 0.992, and the CP moves
  about 0.7 caliber aft, then forward again by Mach 1.5. The C_Nα errors at 0.992 (±26%) make that CP ±0.6 caliber.
- The authors find their C_Nα "a bit below" Eglin's from Mach 0.6 to 1.2, blaming their low yaw (PDF p. 24).
  Eglin's report, AFATL-TR-81-87, was not reached (DTIC down; not in the Internet Archive's DTIC collection).

## What the project can use

- **Terms.** Both reports are Crown copyright and marked "unlimited distribution" (box 11; D1 PDF p. 84, D2 p. 78).
  Their terms of release allow information and evaluation use only, bar commercial use, and bar release of the
  report or its information to third parties without DRDC's written consent (D1 PDF p. 5; D2 PDF p. 4). The project
  may cache the PDFs under `refs/papers/` and cite values with table and page; a committed fixture of their numbers
  in an MIT/Apache repository is a licensing call for the maintainer, as with DREV-TM-9703.
- **For #619's fixture** (suggestion): replace the pixel readings with Table 12's 0° rows at Mach 0.789, 0.916, 0.917,
1.054, 1.076, 1.176, 1.177, 1.246, 1.247, 1.280 and 1.282, add Table 13's 45° rows as a second band, and Table 10's
range rows with Table 7's standard deviations. Expected effect: the ±0.3 per rad reading error goes; the comparison
gains a measured spread (−2% to −9% by roll, tunnel 6% to 12% above range) that a tolerance can cite.
- **Score the shape, not one roll.** HPR Sim has no roll-dependent C_Nα, so a test can ask that its slope lie in the
  band between the 0° and 45° runs, and its peak's Mach between 0.92 and 1.16. Expected effect: a pass or fail on
  the peak's position that the pixel set could not give, without overclaiming its height.
- **Chord decision:** fly 1.0 caliber, citing BRL 711 Fig. 8; use 0.94 only against AEDC-TR-76-58's data.
- **The Air Force Finner's C_mα** (±1%) is a tight transonic check of a pitching-moment slope on an ogive with
  delta fins, a shape nearer a hobby rocket's. Expected effect: a second measured transonic case besides the Arcas,
  with its CP jump near Mach 0.95 as a sign check.

## Open questions

- **Wall and sting effects** are not bounded apart; the tunnel-to-range gap (+6% C_Nα, +12% C_mα) bounds them together.
- **Why 45° roll reads lower** by up to 9% near Mach 1.07 is not explained in the report.
- **Not reached:** AFATL-TR-81-87 (Eglin's Air Force Finner range and tunnel data), and the full run-by-run tunnel
  database the report says is "available from the author" (PDF p. 30); the project does no outreach.
- **No point between Mach 1.076 and 1.157** in the tunnel, nor between 1.105 and 1.293 in the range multiple fits.

## Sources (read 2026-10-20 unless noted)

1. **D1.** A. Dupuis, *Aeroballistic Range and Wind Tunnel Tests of the Basic Finner Reference Projectile from
   Subsonic to High Supersonic Velocities*, DRDC Valcartier TM 2002-136, October 2002, 85 pp. Crown copyright.
   <https://web.archive.org/web/20220914071220/https://cradpdf.drdc-rddc.gc.ca/PDFS/zbd97/p518050.pdf>. Record:
   <https://web.archive.org/web/20190917064149/pubs.drdc-rddc.gc.ca/BASIS/pcandid/www/engpub/DDW?W%3DSYSNUM=518050>
2. **D2.** A. D. Dupuis, W. Hathaway, *Aeroballistic Range Tests of the Air Force Finner Reference Projectile*, DRDC
   Valcartier TM 2002-008, May 2002, 79 pp. Crown copyright.
   <https://web.archive.org/web/20220913233622/https://cradpdf.drdc-rddc.gc.ca/PDFS/zbd95/p517505.pdf>. Record:
   <https://web.archive.org/web/20191015140711/pubs.drdc-rddc.gc.ca/BASIS/pcandid/www/engpub/DDW?W%3DSYSNUM=517505>
3. **D3.** A. D. Dupuis, W. Hathaway, DREV-TM-9703, 1997. <https://archive.org/details/DTIC_ADA636861>
4. **D4.** V. Bhagwandin, J. Sahu, ARL-TR-6725, 2013. <https://archive.org/details/DTIC_ADA592550>
5. **D5.** R. E. Bolz, J. D. Nicolaides, BRL Report 711, 1949. <https://archive.org/details/DTIC_ADB202855>
6. **D6.** Regan, NAVORD Report 6652, 1964. <https://archive.org/details/DTIC_AD0600975>
7. **D7.** Jenke, AEDC-TR-76-58, 1976. <https://archive.org/details/DTIC_ADA027027>
8. DTIC search, <https://apps.dtic.mil/sti/search>, "Under Maintenance" (read 2026-10-19). D3 to D7 were read from
   copies cached for earlier notes; the US reports (D4 to D7) are approved for public release.
