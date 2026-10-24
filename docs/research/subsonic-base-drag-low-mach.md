# Base drag of a flat-based body below Mach 0.6, without a rear sting

**In short: below Mach 0.5, four tests that hold a flat-based body without a rear sting put its base drag at 0.14–0.16
on the base area; the shared rule, 0.12 + 0.13 M², gives 0.120–0.132 at Mach 0.06–0.3, low by 0.02–0.04.** Two tests
hold the body by magnetic suspension (0.142 and 0.147 at Mach 0.08–0.19). One holds a full-scale, turbulent, l/d 10.9
body on a strut 4.1 diameters ahead of a closed base, and Pisa's strut-held body reads 0.146–0.160. The full-scale one
reads 0.153, 0.158, 0.162 and 0.172 at Mach 0.30, 0.50, 0.71 and 0.82, tabulated, not read off a plot. So the measured
level rises only 12% from Mach 0.3 to 0.82 where the rule rises 57%: the rule is right near Mach 0.5 and turns high
above it, as the parent note [cylinder-base-drag](cylinder-base-drag.md) found from Mach 0.7 up. A thicker boundary
layer lowers it, by 0.014 as δ/d goes from 0.107 to 0.171 at Mach 0.06. A rear sting lowers it by about 10–25% at low
speed, so sting-held data read low. Two small transonic-tunnel models held from the side or upstream read 0.117 at
Mach 0.6, 0.04 below the strut set; nothing here explains that gap. Every value is on the base's own area, positive as
drag; "Inference" marks this note's own arithmetic.

## 1. The sets

The rule is Niskanen's eq. 3.94 after Fleeman, used by HPR Sim and OpenRocket (parent note §1). Below, C_DB = −C_pb,
the base drag on the base area. "Read" values come off a plot, with the read-off error given.

| Set | Source | Body, diameter | Support | Mach | Boundary layer |
|:--|:--|:--|:--|:--|:--|
| LM | [S1] | ogive-cylinder, l/d 9.2, 30.8 mm | magnetic | 0.11–0.19 | mostly laminar; tripped |
| KM | [S2] | hemisphere-cylinder, l/d 6.5, 42 mm | magnetic | 0.08 | not stated |
| TP | [S3] | ogive-cylinder, l/d 10.9, 20.3 cm | strut 4.1 d ahead of base | 0.30–0.82 | turbulent from the nose |
| PI | [S4] | elliptic nose, l/d 5.7, 70 mm | faired strut over a plate | 0.06 | turbulent, δ/d 0.107–0.171 |
| IC | [S5] | blunt-ended body, l/d 6.5, 196.5 mm | thin airfoil-shaped sting | 0.044 | turbulent, θ/d 0.011 |
| LS | [S6] | Kármán ogive, l/d 10, 2 in | side support | 0.6–0.8 | tripped at 20% length |
| SO | [S7] | ogive-cylinder, l/d 7, 22 mm | magnetic; stings brought close | ≤ 0.19 | smooth; tripped |

- **LM, Langley 13-inch magnetic suspension (Britcher, Alcorn and Kilgore 1990).** "No mechanical support of any kind"
  [S1] p. 7; closed base, four taps from center to 87.5% radius; buoyancy and blockage corrected [S1] pp. 16–18, 21.
  Re_D 79,000–133,000. **Inference:** Mach 0.11–0.19, Re_L 0.7–1.2 × 10⁶. Taps read 0.121–0.171 over 20 points, ±0.004,
  with "no distinct trend" in Reynolds number [S1] p. 23. **Inference:** the area-weighted mean of the radial profile at
  Re_D 94,000 (0.155 center, 0.136 at 87.5%) is 0.142. Tripped and free layers made "no significant" difference
  [S1] p. 23; the grit was too fine to trip fully at the lowest Reynolds numbers [S1] p. 55.
- **KM, Kyushu University with a loaned 300 mm magnetic suspension (Tadakuma et al. 2005).** 27 m/s, Re_D 7.6 × 10⁴,
  turbulence within 0.7%, blockage corrected; telemetry ±1.5 Pa [S2] pp. 563–565. Six radii read 0.140–0.157, ±0.003
  [S2] Fig. 10. **Inference:** area mean 0.147; Mach 0.08; the layer is probably laminar.
- **TP, NASA TP-2638 (Powers, Huffman and Fox 1986).** A full-scale F-111 fin-cap body of revolution in the Langley
  7- by 10-foot high-speed tunnel, joints sealed against base bleed [S3] pp. 6–8. A NACA 64-010 strut of 17.8 cm chord
  ends 82.8 cm (4.1 d) ahead of the base; no sting [S3] p. 8. Grit trip 3.6 cm behind the nose tip, Re_L 2.0–4.0 × 10⁷
  [S3] pp. 9–10, Table 4(a). C_Db integrates the radial base taps [S3] p. 10. **Inference:** a flat-plate turbulent
  layer, δ = 0.37 x Re_x^(−0.2), gives δ/d 0.12–0.14 at the base, at the thin end of the HPR range (§4).
- **PI, Pisa (Mariotti and Buresti 2013).** The journal paper [S8] is paywalled; the same experiment is in full in
  Mariotti's PhD thesis [S4] and a master's thesis [S9]. Open test section 1.1 m across, 0.9% free-stream turbulence,
  Re_L 5.5 × 10⁵ [S4] pp. 108–110. Emery-cloth strips thickened the layer, measured 0.1 d ahead of the base
  [S4] pp. 111–112. C_pb is averaged out to r = 0.4 d; to the rim it changes "only slightly" [S4] p. 123. **Inference:**
  Mach 0.06.
- **IC, Imperial College (Zhu 2023 thesis).** U∞ 15 m/s, Re_D 1.88 × 10⁵, turbulence under 1% [S5] p. 54; θ 2.14 mm,
  quoted from Oxlade et al. (2015) [S5] p. 72; the averaged base pressure oscillates about −0.11 [S5] p. 104. Where the
  sting meets the body is not stated in the thesis; if it enters the base, IC reads low (§4).
- **LS, Lee and Summers 1957.** Ames 2- by 2-foot transonic tunnel, Re_L 8 × 10⁶, one orifice in the base [S6] pp. 3–4.
  The side support alone read 0.117 at Mach 0.60 and 0.129 at 0.80, ±0.005 [S6] Fig. 19, p. 28; probable error ±0.004
  [S6] p. 4. The side support's own interference is not given.
- **SO, Southampton magnetic suspension (Henderson 1976).** Base pressure was measured only through a dummy sting's
  tip. **Inference:** from the sting-free total drag (0.26 smooth, 0.29 tripped [S7] p. 81) less the nearly constant
  forebody part [S7] p. 83, the sting-free base drag lies at 0.12–0.16 smooth and 0.10–0.12 tripped. A bound only.

## 2. The rule against the sets

| Set | Mach | Measured | Rule | Rule − measured |
|:--|--:|--:|--:|--:|
| IC | 0.044 | 0.11 | 0.120 | +0.01 |
| PI | 0.06 | 0.146–0.160 | 0.120 | −0.026 to −0.040 |
| KM | 0.08 | 0.147 | 0.121 | −0.026 |
| LM | 0.11–0.19 | 0.142 | 0.122–0.125 | −0.02 |
| SO | ≤ 0.19 | 0.10–0.16 (bound) | 0.125 | inside |
| TP | 0.30 | 0.1531 | 0.132 | −0.021 |
| TP | 0.50 | 0.1576 | 0.153 | −0.005 |
| LS | 0.60 | 0.117 | 0.167 | +0.050 |
| TP | 0.71 | 0.1615 | 0.186 | +0.024 |
| TP | 0.82 | 0.1720 | 0.207 | +0.035 |

- **Mach 0.06–0.3: the rule reads low by 0.017–0.040 (12–25%)** against the two magnetic sets, the turbulent strut set
  and Pisa. **Inference:** four of the six sets at or below Mach 0.3 (PI, KM, LM, TP) sit at 0.14–0.16; IC reads
  0.11 and SO's bound spans 0.10–0.16.
- **Mach 0.5: within 0.005** of TP. **Above Mach 0.55 it reads high**: +0.024 and +0.035 against TP at 0.71 and 0.82,
  joining the parent note's +0.028 to +0.071 against Hart's free flight at Mach 0.7–0.9. At Mach 0.71 TP (0.162)
  agrees with Hart (0.156) within Hart's ±0.030.
- **Two families at Mach 0.6.** LS and Cubbage's upstream-held cylinder (parent note) both read 0.117 at Mach 0.6;
  TP and Hart read 0.156–0.162 at 0.7. Both small-model sets are in transonic tunnels (LS at Re_L 8 × 10⁶). The side
  support, the tunnels and Cubbage's nozzle cavity are candidates; none is sized. Nose length doesn't separate LS
  from TP: LS's nose is half the body [S6] p. 3, and TP's reaches full radius at 53% of its length [S3] Table 1(a).

## 3. How it changes with Mach number below 0.6

- **Measured (TP).** Over Mach 0.30: +2.9% at 0.50, +5.5% at 0.71, +12.3% at 0.82 [S3] Table 4(a). Reynolds number
  rises with Mach in that tunnel (2.0 to 4.0 × 10⁷). **Inference:** with PI's slope (§4), the thinner layer accounts
  for about 0.004 of the rise to 0.82, leaving about +0.015 for Mach alone.
- **Hoerner** expects a "comparatively modest" subsonic rise, possibly "a certain power of the Prandtl factor",
  with no exponent given [S10] p. 16-4.
- **Inference (fit).** C_DB ∝ (1 − M²)^(−n/2) through TP gives n of 0.18–0.30: about a quarter power of the
  Prandtl–Glauert factor. From rest to Mach 0.6 that is a 4–7% rise; the rule's M² term gives +39% and the full
  factor +25%.
- **DATCOM 4.2.3.1** has no subsonic Mach term in its base drag (eq. 4.2.3.1-b) and works its sample problem at
  Mach 0.8 [S11] pp. 4.2.3.1-2 to -3.

## 4. Boundary layer, forebody length and supports

- **Thicker layer, less base drag (PI).** C_DB 0.160, 0.153, 0.146 at δ/d 0.107, 0.143, 0.171 (θ/d 0.0086, 0.0121,
  0.0144) [S4] Table 5.2, p. 198: −0.014 (−8.7%). **Inference:** −0.22 per unit δ/d. Carried past the tested range,
  it gives 0.140 at δ/d 0.20 and 0.118 at 0.30. Southampton's shoulder trip lowered base drag by about 0.02 [S7] p. 83,
  the same sign; Langley's showed no significant change (§1).
- **No low-speed data above δ/d 0.17.** **Inference:** flat-plate estimates for HPR bodies (l/d 8–20, Re_L 2 × 10⁶ to
  3 × 10⁷) give δ/d 0.10–0.41, so the thick-layer half of the HPR range is untested here.
- **Forebody length.** Mair (1965), as cited by both Pisa theses, gives −C_pb 0.15 at l/d about 6 and 0.14 at about 10
  on a round-nosed flat-based body [S4] p. 107; not read at source, support not stated. **Inference:** about −0.01 for
  4 more calibers, the same sign as a thicker layer.
- **Hoerner's eq. 34 reads low on cylinders.** **Inference:** 0.029/√C_fB, with C_fB = 8θ/d at the base edge, gives
  0.085–0.111 on PI (low by 0.049–0.061, 31–42%) and 0.098 on IC (low by 0.012); with skin friction on the wetted
  area or TP's own forebody drag, 0.083–0.100 on TP at Mach 0.30 (low by 0.053–0.070). The parent note found the
  same on Hart's body. In Fig. 37 every point above C_DB 0.14 is cone-cylinder data "at M > 1" or projectile data;
  below it sit 5 AVA and 3 DVL fuselages with cut-off streamline tails and two projectile points of unstated Mach,
  none a flat-based cylinder (counted from a 900 dpi render; ±2 triangles overlap) [S10] p. 3-19. DATCOM's eq.
  4.2.3.1-b is the same 0.029/√ form with a base-to-body area factor [S11] p. 4.2.3.1-2.
- **A rear sting at low speed lowers base drag by about 10–25%.** Measured against the same model with no sting:
  - LM: a 0.41 d dummy sting lowered C_D by 0.027–0.034 with free transition and 0.018–0.019 tripped, read ±0.001
    [S1] Fig. 12, p. 26. **Inference:** 17–24% and 11–13% of the sting-free base drag.
  - KM: 0.24 d and 0.60 d stings each cut suction by 0.014–0.026 (10–17%) at the base's outer radii [S2] Fig. 10.
  - SO: stings of 0.14–0.5 d lowered C_D by 0.011–0.026 [S7] Fig. B4; **Inference:** 7–25% of the bound.
  - Hart's 0.6 d sting at Mach 0.8 took 40% (parent note). **Inference:** the effect grows toward transonic speed.
- **A fin-like mount cut it by 0.10–0.14.** TP's body flown as an aircraft fin cap read 0.028–0.065 at local Mach
  0.69–0.94 and 4–7° aircraft angle of attack, 18 points counted [S3] Table 3(a), pp. 28–31; the authors blame the fin's
  recompression [S3] p. 13. The parent note found fins near the base lower it by 0.030–0.039 (Hart's flights).
- **Strut and turbulence effects are unbounded.** TP's strut makes the pressures on the body ahead of the base more
  negative [S3] pp. 20–21; its effect on the base is not sized. Pisa's authors say a support effect "can not be
  excluded" and name the free-stream turbulence [S4] p. 218 (0.9%, p. 108). IC reads 0.036–0.050 below PI at nearly the
  same θ/d and l/d; support, turbulence, Re_D and averaging radius differ, and no source separates them.
- **Incidence raises it.** TP at 3.2–3.4°: 0.179, 0.185, 0.199 at Mach 0.30, 0.50, 0.71 [S3] Table 4(a).
  **Inference:** +0.026 to +0.037 (+17–23%), larger than the rule's error below Mach 0.5.

## What the project can use

- **Terms.** TP and LS are NASA and NACA reports; LM (NASA CR-4299, by Old Dominion University under a NASA grant)
  and DATCOM (a contractor report cleared for public release) carry no copyright notice. All four are public on NTRS
  or DTIC and cached under `refs/` (gitignored); TP's tabulated values may go into a fixture with table and page. KM
  (© JSASS), the Pisa theses (no license stated), Henderson's thesis (University of Southampton thesis license),
  Zhu's thesis (CC BY-NC 4.0) and Hoerner (©) are cited, not copied.
- **Reference to add.** TP's four α ≈ 0.2° points, with body, strut position, Re_L and tabulated source, beside the
  parent note's transonic fixture, and a test printing the rule's signed error. It is the only closed flat base found
  spanning Mach 0.3–0.8 without a rear support.
- **Sign of the change on #68's apogee bias. Inference:** below Mach 0.5 the rule under-counts base drag by about 0.02.
  On a minimum-diameter rocket coasting with the whole base exposed (base about the reference area, C_D near 0.45),
  that is about 4% of C_D, making simulated apogee too high: the same sign as the collection's +8.73% bias
  ([accuracy](../accuracy.md)). Above Mach 0.55 the rule over-counts (parent note), with the opposite sign. The net
  depends on each flight's time in each band and is not sized here.
- **Candidate rule (suggestion). Inference:** C_DB = 0.15 (1 − M²)^(−1/8) is within 0.0022 of TP at all four Mach
  numbers, within 0.003–0.008 of KM and LM, and within 0.007 of Hart at Mach 0.7. It fits one set's shape and misses LS
  (+0.042 at 0.6) and Hart at 0.9 (+0.031), where the parent note's transonic rise takes over. A δ/d term of about
  −0.22 per unit δ/d (PI) would need data above δ/d 0.17 first. Ship any change with the bias re-measured.
- **Bounded task for #68 (core band).** Build TP's fixture. *Done when* the base-drag rule (old or replaced) is within
  ±0.01 of TP at Mach 0.30 and 0.50 and within ±0.02 at 0.71 and 0.82, or an ADR records the measured gap. Today it
  fails: −0.021 at Mach 0.30, +0.024 at 0.71 and +0.035 at 0.82.

## Open questions

- Why LS and Cubbage read 0.117 at Mach 0.6 and TP and Hart 0.156–0.162 at 0.7: support, tunnel or cavity.
- How much TP's strut moves its base pressure; a panel or CFD check, strut on against off, would bound it.
- No flat-base data below Mach 0.6 with δ/d above 0.17, fins at the base, or a motor-tube cavity; no free flight below
  Mach 0.69. Paywalled leads: Gai and Patil (J. Spacecraft 17, 1980; l/d 6, strut), Porteiro, Przirembel and Page
  (AIAA J. 21, 1983; upstream-held, θ varied by blowing and suction), Morel (Aero. Quart. 30, 1979), Mair (1965).
- Kai, Sugiura and Tezuka (AIAA J. 57, 2019) suspended a 10 mm ogive-cylinder magnetically up to Mach 0.6; the
  abstract gives no drag. Higuchi et al. (JFM 596, 2008) and Tohoku's 1-m system test flat-faced short cylinders, a
  different flow. Goodyer's Southampton report ISAV-150 (1966) was not found online.
- ESDU 76033, *Subsonic base drag of cylindrical bodies with conical boat-tails*: title only [S12].
- Where Imperial's sting attaches (Oxlade et al., JFM 770, 2015, paywalled).
- The magnetic sets are at Re_D near 10⁵; HPR coasts at Re_D 10⁵–10⁶. None at HPR Reynolds numbers was found.

## Sources

All read 2026-10-23. Printed page numbers.

1. **S1.** C. P. Britcher, C. W. Alcorn, W. A. Kilgore, *Subsonic Sting Interference on the Aerodynamic
   Characteristics of a Family of Slanted-Base Ogive-Cylinders*, NASA CR-4299, 1990.
   <https://ntrs.nasa.gov/citations/19900014926>
2. **S2.** K. Tadakuma, S. Aso, Y. Tani, Y. Mizoguchi, T. Okada, base-pressure correction for bluff-base bodies with a
   magnetic suspension and balance system, J. JSASS 53(623), 562–568, 2005 (in Japanese).
   <https://www.jstage.jst.go.jp/article/jjsass/53/623/53_623_562/_article/-char/ja>
3. **S3.** S. G. Powers, J. K. Huffman, C. H. Fox Jr., *Flight and Wind-Tunnel Measurements Showing Base Drag Reduction
   Provided by a Trailing Disk for High Reynolds Number Turbulent Flow for Subsonic and Transonic Mach Numbers*, NASA
   TP-2638, 1986. <https://ntrs.nasa.gov/citations/19880004917>
4. **S4.** A. Mariotti, *Experimental and numerical investigations for the characterization and control of separated
   flows*, PhD thesis, University of Pisa, 2014. <https://etd.adm.unipi.it/t/etd-04092014-012557>
5. **S5.** T. Zhu, *The turbulent axisymmetric bluff body wake: analysis and control through experiment and
   simulation*, PhD thesis, Imperial College London, 2023, CC BY-NC 4.0. <https://doi.org/10.25560/105834>
6. **S6.** G. Lee, J. L. Summers, *Effects of Sting-Support Interference on the Drag of an Ogive-Cylinder Body With and
   Without a Boattail at 0.6 to 1.4 Mach Number*, NACA RM A57I09, 1957. <https://ntrs.nasa.gov/citations/19930089881>
7. **S7.** R. I. Henderson, *The Use of a Magnetic Suspension Balance for the Measurement of Magnus Forces on a Body of
   Revolution*, MPhil thesis, University of Southampton, 1976. <https://eprints.soton.ac.uk/462474/>
8. **S8.** A. Mariotti, G. Buresti, Exp. Fluids 54, 1612, 2013 (abstract only).
   <https://doi.org/10.1007/s00348-013-1612-5>
9. **S9.** M. Simonelli, master's thesis on the same experiment, University of Pisa, 2014.
   <https://etd.adm.unipi.it/t/etd-02132014-002304>
10. **S10.** S. F. Hoerner, *Fluid-Dynamic Drag*, 1965. <https://archive.org/details/FluidDynamicDragHoerner1965>
11. **S11.** USAF Stability and Control DATCOM, AFWAL-TR-83-3048, 1978 revision, §4.2.3.1 (DTIC ADB072483).
12. **S12.** Aeronautical Journal 82(806), 1978, front matter (ESDU item list).
    <https://doi.org/10.1017/S0001924000096111>
