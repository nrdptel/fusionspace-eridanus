# Measured coefficients of standard finned test bodies, Mach 1.5–2.5

[ADR-143](../decisions/0143-the-operating-envelope-and-a-stop-rule-for.md) §6b names three sources for
[M1.14b, references from Mach 1.5 to 2.5](../decisions-and-roadmap.md#m1-14b): the Basic Finner's free-flight data
(DREV-TM-9703), "ARL-TN-1128's measured supersonic coefficients (HSARV)" and a Mach 2.45 tunnel drag point
(doi 10.2478/tar-2024-0018). For each, this note gives the geometry, Mach and angle range, coefficients measured,
terms of use and whether HPR Sim's design tree can draw the body; it adds older Basic Finner reports and the AGARD
calibration models, and checks the reference behind HPR Sim's roll-damping row, −5.9% to −16.2% against the Basic
Finner's tunnel points ([aero](../physics/aero.md#roll-against-the-arcas-robin-and-the-basic-finner)).

**In short: one of the three is a reference, and it questions the roll-damping row.**

- **The Basic Finner is the reference.** DREV-TM-9703 tabulates C_X0, C_Nα, C_mα, C_mq and C_lp from Mach 1.08 to
  4.3. Four measured sets of its roll damping, DREV's range, BRL's range, NOL's tunnel and AEDC's tunnel, each lie
  within about 11% of DREV (up to 19% apart from each other). The Nicolaides points that HPR Sim compares against,
  read from Barrowman's Fig. 5-7, sit **above** DREV's at matched Mach. Against DREV, HPR Sim's damping
  reads **+2% to +13% high** at Mach 1.82–3.00, not 5.9% to 16.2% low. Near Mach 1.5 it reads +26% to +30% high
  against BRL and NOL. The cause of the gap is not found (**Inference**, §2).
- **ARL-TN-1128 holds no coefficients.** It defines a shape, the HARV (High-Speed *Army* Reference Vehicle, not
  "HSARV"): no Mach number, no test, no prediction, no mass. ADR-143 §6b describes it wrongly.
- **The Mach 2.45 point is not usable.** Its measured C_D, 0.138 to 0.189 for three noses, is a quarter to a third
  of the authors' CFD for one of them (0.568); the authors blame their balance. CC BY-NC-ND 4.0; noses confidential.

Confidence: high that the four sets share one convention and sit well below Nicolaides'; medium on figure
readings (**Inference**). Why Nicolaides' values are larger is unbounded: BuWeps TN-100A wasn't reached.

## 1. The Basic Finner

**Geometry.** A 20° cone (10° half-angle) on a cylinder, 10 d long overall. Four fins, each 1 d × 1 d, sit flush
with the base, 3 d tip to tip. Each fin is a wedge from a sharp leading edge (radius 0.004 d) to 0.08 d thick at the
trailing edge. The moment reference is 5.5 d from the nose [F1 §3.1, PDF p. 16, printed p. 3].
"Army-Navy Finner" and "Tri-Service Basic Finner" name the same shape [A1 p. 24; S1 p. 1; F7 PDF p. 7].

- **The chord is 1.0 d.** That is DREV's value and NOL's drawing [F5 PDF p. 19]. AEDC's Fig. 2 draws 0.94 d
  [F6 PDF p. 25], but its text says "approximately one caliber" (PDF p. 9). **Inference:** 0.94 d belongs to AEDC's
  own 1.8 in model only. This settles the open question in
  [transonic-normal-force.md](transonic-normal-force.md#open-questions).
- **BRL 711's fins** are thicker: its Fig. 8 draws a 0.125 in trailing edge on a 0.786 in body, 0.16 d [F2 PDF p. 30].

**DREV-TM-9703** (Dupuis and Hathaway, August 1997) [F1]: a spark-range free-flight test of 30 mm models, Mach 1.06
to 4.47, small yaw. Table IX (PDF pp. 48–49) gives multiple fits of C_X0, C_Nα, C_mα about 5.5 d, C_mq and C_lp
from Mach 1.077 to 4.300; this note uses its rows at Mach 1.832, 2.375, 2.718 and 3.147 (C_Nα's probable errors 2%
to 4% where printed). The readings are cached locally under `refs/`: the terms of release bar redistribution (#619).

- The CP is 5.5 d − C_mα/C_Nα. C_lp's probable errors print as 0%; C_X0 has none, fitted shot by shot (PDF p. 23).
- **Gap:** no round flew between Mach 1.377 and 1.799 (Table VIII, PDF pp. 44–47): Mach 1.5 has no DREV point.
- **Convention (Inference).** DREV doesn't state the rate's denominator (PDF pp. 12–13); its C_lp interpolated to
  Mach 2.5 lies close to AEDC's, defined per p·d/(2V). Re 7 to 30 × 10⁶ on the length (PDF p. 19).

**Other measured sets, Mach 1.5–3,** converted to C_lp per p·d/(2V) on area πd²/4 and length d:

- **BRL Report 711** (Bolz and Nicolaides 1949, range) [F2 Table V, PDF p. 36] gives 0.546 to 0.564 at Mach 1.59 to
  1.75. Its coefficient is L/(q·A_p·b·n) per p·b/(2V), with A_p one panel, n fins and b the 3 d span (PDF p. 18).
  **Inference:** × A_p·n·b²/(πd⁴/4) = 144/π = 45.84 gives −25.0 to −25.9. ARL-TR-846 [F7 Table 7,
  PDF p. 21] lists the same within 0.1, and adds −19.54, −19.44 and −18.39 at Mach 2.25, 2.42 and 2.68 from
  the journal version of BRL 711 (J. Aero. Sci., 1950; PDF p. 20).
- **NAVORD Report 6652** (Regan 1964, NOL tunnel, spin decay, α = 0) [F5 Table 2, PDF p. 17] uses the same
  definition as BRL 711 (PDF p. 6). The averages are 0.532, 0.422, 0.371, 0.352 and 0.315 at Mach 1.53, 2.03, 2.27,
  2.54 and 2.76 (Table 2 prints 2.70; Table 1, 2.76). × 45.84 gives −24.4, −19.3, −17.0, −16.1 and
  −14.4. NOL calls its values "slightly below" BRL 799's (PDF p. 13).
- **AEDC-TR-76-58** (Jenke 1976, tunnel, Mach 2.50) [F6 Figs. 9b–c, PDF pp. 43–44] is defined per p·d/(2V) on
  πd²/4. **Inference**, read by eye: −17.5 ± 0.5 at α = 0, the same at two Reynolds numbers, growing
  to about −30 near α = 35°. Its Fig. 9b plots NOL's Mach 2.5 point near −16, which confirms the factor of 45.84.
  A 2024 paper cites AEDC at Mach 2.49 as −16.8 ± 0.8, with C_mq −180 ± 45 [A1 p. 27] (secondary).

## 2. HPR Sim's roll damping against the measured sets

HPR Sim's values are Barrowman's Fig. 5-7 points times one minus the shortfalls in
[ADR-031](../decisions/0031-roll-from-canted-fins-and-roll-damping-by.md), not flown here; measured values are
interpolated linearly in Mach (**Inference**).

| Mach | Fig. 5-7 (Nicolaides) | HPR Sim | BRL | NOL | HPR Sim vs NOL |
|--:|--:|--:|--:|--:|---|
| 1.51 | −33.60 | −31.6 | −25.0 (1.59) | −24.4 (1.53) | +30% (+26% vs BRL) |
| 1.82 | −27.45 | −25.3 | −24.3 | −21.5 | +18% |
| 2.27 | −23.48 | −20.1 | −19.5 | −17.0 | +18% |
| 2.60 | −20.92 | −17.6 | −18.7 | −15.7 | +12% (+5% vs AEDC, at 2.5) |
| 3.00 | −18.32 | −15.4 | none | −13.8 | +11% |

- **Against DREV** (readings cached under `refs/`, §1), at Mach 1.82 to 3.00 (its 1.832 row, then interpolated), HPR
  Sim reads +2% to +13% high, and Fig. 5-7's points sit above DREV's.
- **Fig. 5-7's points sit above DREV's** (in magnitude) at every Mach number. **Inference:** a reference-area slip
  between TN-100A and Fig. 5-7 could explain it, unproven while TN-100A is unread; Barrowman doesn't state their accuracy.
- **What changes.** On the measured consensus the row's sign flips: HPR Sim overstates the Basic Finner's
  supersonic roll damping, and the aero page's account (first-order theory reads low for want of a thickness
  term) no longer fits.
- **Below Mach 1.5** tunnel C_lp swings with spin rate (NOL −9 to −17 at Mach 0.77 [F5 Fig. 4]; AEDC about −18
  at 0.22 [F6 Fig. 9a]), far from Barrowman's computed −34.21: no clean reference.

**Other coefficients.** BRL 934 [F3 Table III, PDF p. 41] gives range C_D 0.710 at Mach 1.506, 0.604 to 0.662 at
1.78 to 1.83 and 0.449 at 2.468; its C_Nα is too scattered to use (low-yaw rounds' probable errors 14% to 93%).
NOL's static tunnel [F4 Figs. 29–30], by eye, gives C_Nα about 13.2 at Mach 1.58 and 9.7 at 2.48, CP 8.2 d to
7.2 d: DREV's within reading error.

**Design tree.** HPR Sim can draw the body: a `Conical` nose, a body tube, and a four-fin `FinSet` with root = tip =
1 d, span 1 d and no sweep, flush with the base. What it can't draw is the single wedge: `FinCrossSection` offers only
`Square`, `Rounded` and `Airfoil`. **Inference:** C_lp, C_Nα and CP still compare fairly to first order, since
thin-airfoil lift ignores thickness, though BRL 711 measured its thicker wedges' C_lp 21% above its linear
theory (PDF p. 33). C_X0 does not compare. The trailing edges' base area is 0.32 d², 41% of the reference
area, so a base pressure coefficient of −0.1 to −0.2 adds 0.04 to 0.08, a rough bound.

## 3. ARL-TN-1128: a shape, no data

*High-Speed Army Reference Vehicle* (Vasile et al., DEVCOM ARL, other Army centers and Texas A&M, August 2022,
21 pages, approved for public release; every page read) [S1].

- **Geometry** (calibers; printed pp. 1–4, PDF pp. 8–11). L/D 10: a 5-cal nose, a 5-cal cylinder, no boattail.
  The nose is a cone (marked about 5°; a pointed one would be 5.71°, **Inference**) or a von Kármán, each with a
  0.05-cal spherical tip. Three or four clipped-delta fins: root 4.8, tip 2.125, 0.95 from the axis to the tip
  (0.45 exposed span); flat plates 0.07 thick with rounded leading edges, the trailing edge not stated.
- **What it lacks.** No Mach or angle range, no method, no C_D, C_Nα, CP, C_mq or C_lp, no prediction code, no
  mass properties and no scale; "mach", "coefficient" and "mass" hit only its references and boilerplate. Its
  purpose is hypersonic transition studies (p. 1).
- **Companions,** not read (DTIC gave 403 or "Under Maintenance" on 2026-10-18), both predictions: ARL-TR-8936,
  the Missile DATCOM study behind the shape, and ARL-TR-9019, a projectile dataset whose link to the HARV is
  unconfirmed. An arXiv paper (2512.13530) runs NASA's CBAERO on the HARV, with no measurements [S2].
- **Terms.** Six of the eight authors are Army staff, two university staff: **Inference,** not clearly public
  domain; cite it and restate its dimensions as facts. **Design tree.** Nose, body and fin planform fit; the
  spherical tip doesn't (`NoseCone` has no tip radius), nor do the fin edges. The fins are strakes (aspect ratio
  0.26 for two joined), far from hobby fins: at most a code-to-code case.

## 4. The Mach 2.45 drag point

Wasilczuk, Kurowski and Flaszyński, "Test Section Design for Measuring the Drag Coefficient of a Suborbital Rocket
Model at Ma 2.45", *Transactions on Aerospace Research* 276(3), 2024, pp. 86–100 [T1].

- **Model** (p. 89): a 1:50 sounding rocket, 10 mm body cut to 180 mm, four fins (19 mm root, 8 mm tip, 6.5 mm
  span), sweep not given; the three noses (von Kármán and ogive types, l/d 3.1) are "confidential". **Test**
  (pp. 88, 92, 96): IMP PAN blowdown tunnel, static 6 kPa and 130 K, α = 0, axial force only on a 0–10 N cell
  (±0.05 N); reference area the frontal area with fins.
- **Values** (Fig. 8, p. 97): C_D 0.189, 0.138 and 0.160 for the three noses; the authors' RANS gives 0.568 for
  nose 2. The model bent in the flow, and the setup "should be reconsidered" (p. 98).
- **Inference:** 0.138 is about 0.29 N, so the cell's ±0.05 N alone is ±0.024 in C_D; friction on an 18-caliber
  body plus base drag is about 0.35, so the CFD is the plausible value. **License:** CC BY-NC-ND 4.0 (landing page
  and Crossref): cite a value as a fact, never as a fixture.

## 5. AGARD calibration models

- **AGARD Model B** is an 8.5 d body, a 3 d nose, and a 60° equilateral delta wing, 4 d span and 4% biconvex, ending
  1.40 d ahead of the base [B1 p. 4; B2 p. 99]. NACA TN 3300 [B1] (Langley, public use permitted) tested it at Mach
  1.62, 1.94 and 2.41, Re about 3 × 10⁶. Read by eye from Figs. 4–5:

  | Mach | C_Lα /deg | CP from nose, d | on body area, C_Nα /rad (Inference) |
  |---|---|---|---|
  | 1.62 | 0.044 | 5.7 | about 22 |
  | 1.94 | 0.038 | 5.6 | about 19 |
  | 2.41 | 0.032 | 5.4 | about 16 |

  C_Lα is on wing area; × 8.82 puts it on body area. AGARDograph 64's review [B2 pp. 109–111, Figs. 8, 10] and
  JAXA's tunnel [B3] agree within a few percent; forebody drag scatters between tunnels, so B checks C_Nα and CP,
  not drag. The wing's leading edge turns sonic at Mach 2.0, inside the core band: a test of a fin model's switch
  between subsonic and supersonic edges. HPR Sim draws the wing exactly as a two-fin pointed trapezoid mid-body;
  it has no biconvex section; a tangent ogive misses the nose's profile by up to 5.8% of the radius, a bulged
  secant ogive (`radius_ratio` near 0.86) by about 1.4% (**Inference**).
- **AGARD Model C** adds a T-tail, which the tree can't draw. **AGARD Model A with fins** (NACA RM-10), a
  parabolic body with four swept tail fins tested to Mach 3, one point at 6.9 [B2 Part I], is drawable; its lift
  figure states no units or reference area, so its numbers are **unverified**.

## What this suggests

1. **Re-point the roll-damping row** to the measured sets: BRL 711 and ARL-TR-846 for 1.59–2.68 and NOL for
   1.53–2.76 as committed points, and DREV Table IX (Mach 1.83–3.15) scored locally from the cached readings, its
   error statistics published, never its values (#619). Expected: the row turns from −5.9…−16.2% to about +2…+13%
   at Mach 1.82–3.00 and +26 to +30% near Mach 1.5. Keep Nicolaides' points as a disputed set until TN-100A is found.
2. **Fly the Basic Finner against DREV Table IX** for M1.14b, locally, publishing only the errors: C_Nα and CP at
   Mach 1.83, 2.38 and 2.72, and C_X0 with Table VIII's per-shot errors. Expect C_X0 to read low by up to 0.04–0.08,
   from the missing wedge base.
3. **Record ARL-TN-1128 as refused** under M1.14b, giving the reason: no coefficients. Correct "HSARV" to HARV.
4. **Record the Mach 2.45 point as refused:** the authors doubt it, and its license bars a fixture.
5. **AGARD Model B** as a C_Nα and CP check across the Mach 2.0 leading-edge switch (NACA TN 3300).

## Open questions

- **Why Nicolaides' points sit above DREV's** (they are 1.12–1.20 times BRL's, 1.28–1.38 NOL's). BuWeps TN-100A
  wasn't reached, nor was BRL 799, seen only as replotted by DREV and NOL. Whether BRL 711's thicker fins explain its
  higher values is open.
- DREV's rate convention is inferred from its agreement with AEDC. DRDC TM 2002-136, DREV Table X (C_lδ),
  BRL 934's centers of gravity and AGARD Model A's drag (NACA Report 1160) were not read.

## Sources (all read 2026-10-18)

- **F1.** Dupuis, Hathaway, DREV-TM-9703, 1997. <https://archive.org/details/DTIC_ADA636861>
- **F2.** Bolz, Nicolaides, BRL Report 711, 1949. <https://archive.org/details/DTIC_ADB202855>
- **F3.** MacAllister, BRL Report 934, 1955. <https://archive.org/details/DTIC_AD0068959>
- **F4.** Shantz, Groves, NAVORD Report 4516, 1960. <https://archive.org/details/DTIC_AD0245560>
- **F5.** Regan, NAVORD Report 6652, 1964. <https://archive.org/details/DTIC_AD0600975>
- **F6.** Jenke, AEDC-TR-76-58, 1976. <https://archive.org/details/DTIC_ADA027027>
- **F7.** Mikhail, ARL-TR-846, 1995. <https://archive.org/details/DTIC_ADA298617>
- **F8.** Barrowman, NASA/TM-2001-209983 (MS thesis, 1967). <https://ntrs.nasa.gov/citations/20010047838>
- **S1.** Vasile et al., ARL-TN-1128, 2022. <https://arl.devcom.army.mil/wp-content/uploads/sites/3/ARL-TN-1128.pdf>
- **S2.** Prim et al., arXiv 2512.13530. <https://arxiv.org/abs/2512.13530>
- **T1.** Wasilczuk et al., 2024. <https://doi.org/10.2478/tar-2024-0018>
- **B1.** Bromm, NACA TN 3300, 1954. <https://ntrs.nasa.gov/citations/19930084038>
- **B2.** Hills (ed.), AGARDograph 64, 1961 (NATO STO archive; read from a Wayback copy).
- **B3.** Iijima et al., JAXA-RR-06-046, 2007. <https://jaxa.repo.nii.ac.jp/record/2185/files/63877000.pdf>
- **A1.** Svorcan et al., OTEH 2024, pp. 24–27. <https://doi.org/10.5937/OTEH24005S>

US Government reports (F2–F7, B1) are approved for public release. DREV-TM-9703 (F1) is Crown copyright, and its terms
of release bar release to third parties without DRDC's consent: its readings stay cached under the gitignored `refs/`,
and only statistics computed from them are published (issue #619).
