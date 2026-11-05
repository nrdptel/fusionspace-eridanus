# How a body's normal force and center of pressure move from Mach 0.6 to 1.2, without fins

Below Mach 1.2, HPR Sim gives each body part slender-body theory's normal-force slope (C_Nα) plus body lift, with no
Mach term ([aero](../physics/aero.md#normal-force-through-mach-1)). NASA TN D-4013's finned Arcas Robin moves its center
of pressure (CP) forward from Mach 0.6 to 0.9, and HPR Sim's sits +1.34 (short model) and +0.68 caliber (long) aft of it
at Mach 0.9 ([#1198](https://github.com/nrdptel/fusionspace-eridanus/issues/1198)). This note asks which public tests
give C_Nα and CP of an ogive- or cone-cylinder *without fins* through Mach 0.6–1.2, how far that CP moves, and how
much of the Arcas's miss a body term could explain. **Inference** marks this note's own fits, readings and arithmetic.
It builds on [transonic-normal-force](transonic-normal-force.md) (finned bodies) and
[transonic-fin-lift-methods](transonic-fin-lift-methods.md) (the fins' share), and bears on
[M1.14c](../decisions-and-roadmap.md#m1-14c) and the transonic CP rows of [accuracy](../accuracy.md).

**In short: the body explains none of the Arcas's Mach 0.9 miss; a better body term would widen it.**

- **On the Arcas itself.** TN D-4013's fins-off pitching moment, read here for the first time, is 11% to 31% (short)
  and 15% to 19% (long) below HPR Sim's body. Put in place of HPR Sim's body, with HPR Sim's fins kept, it moves the
  whole rocket's CP **aft** by +0.29 to +0.36 caliber at Mach 0.9 (short) and +0.16 (long): it widens the miss by
  about a quarter. The body's own change with Mach moved the tunnel's CP *aft* (+0.32 caliber at 0.9, short), while
  the whole rocket moved 0.56 caliber forward. The forward move and the miss are in the fins' share.
- **On public bodies alone.** Four tabulated tests and one range table cover 2- to 3-caliber ogives (one drawn, not
  named) on 9- to 13.5-caliber bodies. From Mach 0.6–0.8 to 0.9, C_Nα rises only 0.6% to 7.3% and the CP moves
  **aft** 0.12 to 0.28 caliber on three of four bodies (0.13 forward on the fourth). From Mach 1.0 to 1.2 the three
  plain ogive-cylinders that resolve it move their CP **forward** 0.33 to 0.48 caliber. No cone-cylinder was found.
- **The body's error is a level error, not a transonic one.** HPR Sim's body moment is already 14% (short) and 22%
  (long) too high at Mach 0.6. The public bodies' slopes sit 16% to 39% above slender-body theory's 2 per rad, their
  CP 0.8 to 2.6 calibers aft of its nose centroid, at every Mach in the band.

Confidence: high on every sign; ±0.05 caliber on each Arcas size; ±0.1 caliber on a public body's CP from one run.
The body's own CP on the Arcas can't be resolved: its fins-off C_N is 0.16 to 0.27 at 4°, against the report's ±0.03.

## 1. The Arcas Robin's body, fins off

Both models are on a 2.25 in body ([1] Fig. 1), Langley 8-foot transonic pressure tunnel, Mach 0.60 to 1.20 (the
short model also 0.95), moment center at 70% of length ([1] pp. 2–4); 18.2 and 23.8 calibers long [1] p. 3. The
4.17-caliber tabulated ogive nose and 15° boattail are the fixture's, from TN D-4014 Fig. 1(a) [9].

- **Normal force** (committed in `arcas-robin-wind-tunnel.json`, `cn_alpha_fins_off`; Figs. 7(b), 9(b)): short 1.53,
  1.41, 2.44, 2.88, 1.58, 2.43 per rad at Mach 0.6, 0.8, 0.9, 0.95, 1.0, 1.2; long 2.19, 1.90, 2.30, 2.30, 2.47
  (`normal-force-vs-mach.json`, `reference_body_cn_alpha_per_rad`). A ±0.02 point gives ±0.20 per rad on a slope
  (Inference), so the +60% rise from 0.6 to 0.9 on the short model is mostly scatter: no public body rises over 8%.
- **The short model's Mach 0.9 row** stands out at 2.44 per rad. A re-read of its −2.07°
  point gives −0.033, not the fixture's −0.004; the circle sits on the zero line under a fins-on symbol. Dropping it
  gives 2.61 per rad, the re-read 2.55 (Inference). It moves the swap below by −0.02 caliber at most.
- **Pitching moment, new here** (Inference). Read off the native 300-ppi scans of Fig. 7(c) (printed pp. 23–25) and
  Fig. 9(c) (pp. 42–44), circles located by ring fits against local grid lines; each point about ±0.03 in C_m. Their
  angles match the same runs on the C_N pages within 0.06°. Slopes about the 70% center, per rad (±0.3):

| Mach | 0.6 | 0.8 | 0.9 | 0.95 | 1.0 | 1.2 |
|---|--:|--:|--:|--:|--:|--:|
| short, body C_mα | 25.9 | 25.9 | 22.7 | 20.6 | 26.3 | 23.7 |
| long, body C_mα | 35.6 | 36.7 | 36.1 | | 35.2 | 36.5 |
| HPR Sim, short / long | 29.6 / 43.3 | 29.6 / 43.4 | 29.6 / 43.4 | 29.6 | 29.6 / 43.4 | 29.6 / 43.4 |

- **The long model's body moment is flat** within 2.2% of its mean; the short model's dips 12% at 0.9 and 21% at 0.95.
  TN D-4014's [9] fins-off moments at Mach 1.5 and 1.8, moved to the 70% center, continue them: 25.7 and 38.0 per rad.

## 2. HPR Sim's body on the Arcas

Recomputed by hand from `body_potential`, `body_term` and `body_terms` in `crates/hpr-aero/src/model.rs` and the
crossflow tables (Inference): nose +2.000 per rad at 2.33 calibers, boattail −1.324 at 17.75, lip +0.178 at 18.19;
body lift on 20.89 (short) and 27.99 (long) reference areas of planform at 9.8 and 12.6 calibers. On the fins-off angles
this gives 1.904 to 1.917 per rad (short) and 2.606 to 2.637 (long), the fixture's `hpr_body_cn_alpha_per_rad` to
10⁻⁴ on all 11 rows; with it the fins' CP backs out at the join's own 0.375 to 0.495 root chords. Its moment about
the 70% center is 29.6 (short) and 43.3 to 43.4 per rad (long), flat with Mach to 0.3%.

- **Against the tunnel:** 13% to 44% high (short), 18% to 23% (long), already at Mach 0.6. This is the hidden half of
  the errors that cancel at Mach 0.6 ([aero](../physics/aero.md#normal-force-through-mach-1)).
- **A correction to a sibling note.** [transonic-fin-lift-methods](transonic-fin-lift-methods.md) §6 puts HPR Sim's
  body CP "about 0.1 m" ahead of the nose tip: it is 0.16 m on the fins-off angles and 0.39 m on the CP angles
  (−2° to 2°, `CP_ANGLES_DEG` in `xtask/src/aero_mach.rs`; Inference).

## 3. The swap: the tunnel's body with HPR Sim's fins (Inference)

Whole-rocket CP = (N_b x_b + N_f x_f)/(N_b + N_f), worked as moments about the 70% center so the poorly read body
force enters only the denominator. HPR Sim's fin term is its total less its body (`normal-force-vs-mach.json`).
"Miss" is HPR Sim's CP less the tunnel's; "swap" is the change in that miss over the fixture's own CP angles.

| calibers, + aft | 0.6 | 0.8 | 0.9 | 0.95 | 1.0 | 1.2 |
|---|--:|--:|--:|--:|--:|--:|
| short, miss | +0.12 | +0.91 | +1.34 | +0.68 | −0.41 | −0.11 |
| short, swap | +0.14 to +0.22 | +0.22 | +0.29 to +0.36 | +0.34 to +0.36 | +0.11 | +0.08 to +0.10 |
| long, miss | −0.36 | +0.07 | +0.68 | | +2.36 | +0.67 |
| long, swap | +0.30 | +0.23 | +0.16 | | +0.17 | +0.01 |

The swap widens every aft miss from Mach 0.8 to 1.0: by 22% to 27% at 0.9 (short), 49% to 52% at 0.95, 23% at 0.9
(long). It closes only the forward misses, 27% at Mach 1.0 and 73% to 91% at 1.2 (short).

- **Uncertainty.** ±0.20 per rad on the body's force moves a swap ±0.02 to 0.03 caliber (short) and ±0.03 to
  0.04 (long); ±0.3 per rad on its moment, ±0.02 to ±0.03. A scale error in the balance's C_m would have to read the
  body's moments 11% to 31% low to undo the sign; the report's ±0.05 in C_m [1] p. 4 is 1.8% to 3.4% of them at 4°.
- **Not the fin anchors:** the short model kept them with fins off, the long one was faired flush [1] pp. 3–4.
- **What a body term would need.** To close +1.34 at Mach 0.9 (short), the body's C_mα would have to be about 17 per
  rad *above* HPR Sim's 29.6 (+58%); the tunnel's is 23% below. For the long model's +0.68, +21% against −17%.
- **Where the miss sits.** The tunnel's fins-on less fins-off share is centered 0.5 to 0.6 caliber *ahead* of the
  fins' root leading edge at Mach 0.8 and 0.9 (short) and 2.7 calibers ahead at 1.0 (long). Part may be the fitting
  basis: slopes over −3° to 4°, CP over −2° to 2°, where the short model is "almost neutral" [1] p. 6.

## 4. Public tests of finless ogive-cylinders, Mach 0.6 to 1.2

Each item gives Mach: C_Nα per rad on the body's cross-section, then CP in calibers from the tip, fitted over about
−2° to +4° (a body's slope grows with angle). All fits are Inference, from transcriptions of the tables.

- **Allen 2001** [2] Table 4(kk), 3-caliber tangent ogive, 12 calibers, Langley 8-ft: 0.60: 2.46, 2.88; 0.90: 2.59,
  3.05; 1.20: 2.72, unresolved (2.1 to 3.6 over windows; its C_m jumps 0.243 between 2.01° and 3.01°).
- **Allen 2005** [3] configuration 6, a drawn nose, a flare and a boattail, 13.5 calibers: 0.60: 2.29, 3.57; 0.90:
  2.30, 3.44; 1.19: 2.58, 3.74. Nine roll angles per Mach on a body of revolution measure the repeat scatter
  directly: ±0.02 to 0.03 per rad, ±0.07 to 0.09 caliber.
- **Martin B4** [4], 2.5-caliber tangent ogive, 9.5 calibers, AEDC 4T, the body of
  [dense-transonic-fin-body](dense-transonic-fin-body.md)'s set: 0.80: 2.57, 2.78; 0.92: 2.65, 2.89; 0.98: 2.72, 2.90;
  1.20: 2.71, 2.45 (end-of-run repeat points left out; with them, 2.49 at 1.20). Its zero-angle C_m jumps to +0.08 to
  +0.10 at Mach 1.2 and 1.3, a hint of a wall wave.
- **Calspan AA-4017-W-11** [5] configuration B, 3-caliber tangent ogive, 10.4 calibers, a 52 in model in an 8-ft
  tunnel: 0.70: 2.31, 2.42; 0.90: 2.48, 2.70; 1.00: 2.45, 2.63; 1.20: 2.47, 2.22.
- **BRL TN 902** [6], the Army-Navy Spinner Rocket in a spark range, 9 calibers, 2-caliber head (four of six rounds
  cut flat): single rounds at Mach 0.92 to 1.18, 2.55 to 2.85 and CP 2.13 to 2.80. Its C_Nα assumes BRL's 8/π
  convention, which the report doesn't state; the CP doesn't depend on it.
- **AEDC-TR-73-44** [7], 2-, 3- and 4-caliber ogives on 8- to 14-caliber bodies, 1-ft AEDC 1T: plotted only, read to
  ±0.15 to 0.25 caliber. Its Mach 1.0 points jump 0.44 to 1.0 caliber aft and return by 1.2; Calspan's larger tunnel
  shows no jump at 1.00, so it is likely a wall effect and not used for sizes. Four more calibers of cylinder move
  the CP 0.86 to 1.19 caliber aft at Mach 0.6.
- **Set aside:** NASA TM X-3128 (first points near 6°), NACA RM L53L28a (pressure-integrated; CP change with Mach
  "irregular and small", printed p. 8), RM L52J01 (too coarse), RM L52D21a, TN 4362, TN D-3203 (not ogive-cylinders),
  NAVORD 2934 (two Mach points, illegible).

**How the CP moves (Inference;** first figure in the −2° to +4° window, range over other windows). From Mach
0.6–0.8 to 0.9–0.98: Allen 2001 +0.17 caliber (0.0 to +0.28), slope +5.4%; Calspan +0.28 (+0.24 to +0.31), +7.3%;
Martin B4 +0.12 (+0.09 to +0.15), +5.8%; Allen 2005 −0.13 (−0.01 to −0.19), +0.6%. From Mach 0.9–1.0 to 1.2:
Calspan −0.48 (−0.40 to −0.48), slope −0.4%; Martin B4 −0.45 (−0.33 to −0.46), −0.3%; BRL's cut noses
−0.43 (1.043 to 1.184), −10%. All three plain ogive-cylinders with a square base move forward there (Allen 2001's
Mach 1.2 CP is unresolved); the only body with a flare and a boattail (Allen 2005) moves +0.30 aft (+0.12 to +0.63),
slope +12.2%.

## 5. Theory against these bodies

- **Slender-body theory** (Barrowman's body terms) gives 2 per rad and a CP at the nose's volume centroid, 0.462 of a
  tangent ogive's length (Inference, by integration). The public bodies sit 16% to 39% above in slope (43% on BRL's
  rounds) and 0.8 to 2.6 calibers aft in CP, at every Mach in the band; the CP gap grows with cylinder length. HPR Sim
  adds body lift, so its slope over a −2° to 4° window comes nearer; its subsonic level is not checked here on these
  bodies.
- **USAF DATCOM** §4.2.1.1-B and §4.2.2.1-B [8]: "Slender-body theory states that body force characteristics are not
  functions of Mach number", subsonic and supersonic values faired, with test data as a guide. That holds for C_Nα to
  about 8% below Mach 0.9 on these bodies; it misses the 0.33 to 0.48 caliber forward CP move from Mach 1.0 to 1.2.

## 6. What a body term could explain at Mach 0.9 (Inference)

HPR Sim passes at Mach 0.6 (+0.12 caliber, short) and misses at 0.8 to 0.95. A body error fixed with Mach is also
present at 0.6, so only the body's *change* from 0.6 to 0.9 could make the miss.

- **Bounded by the public bodies:** a slope rise of at most 8% (+0.19 per rad), acting near 3 calibers against the
  Arcas's whole-rocket CP near 13.0 calibers and C_Nα 10.22, moves the CP 0.15 to 0.20 caliber forward; the body CP's
  own move adds −0.05 to +0.07. Together −0.08 to −0.25 caliber: at most 6% to 37% of the miss, and only if the
  slope rise wins over the aft CP move that three of four bodies show.
- **Measured on the Arcas:** the opposite sign. Its body's change moves the CP aft at 0.9, and its level, put in
  HPR Sim, widens the miss (§3). A Mach term on the body would not fix #1198; the fins' share must.
- **From 0.9 to 1.2,** the public bodies' forward move is worth −0.05 to −0.08 caliber of whole-rocket CP on the
  Arcas at Mach 1.2, small against its −0.11 miss.

## What this suggests

1. **Commit the fins-off C_m readings** of TN D-4013 Figs. 7(c) and 9(c) as `cm_fins_off` in the wind-tunnel fixture,
   and judge HPR Sim's body by its moment about the report's center, not by its C_N slope. Expected: a body row that
   misses by +13% to +44% (short) and +18% to +23% (long), resolving what the fins-off slopes can't.
2. **Record in #1198 that a body fix alone makes it worse.** Bringing HPR Sim's body to the tunnel's moves the CP aft
   0.16 to 0.36 caliber from Mach 0.8 to 0.95: a higher margin, the flattering side. A body change should land with or
   after the fin-side work (#619, #841, #860).
3. **A finless-body fixture** from the four tabulated public bodies, flown through `AeroModel` on each source's angle
   window: the first measured check of HPR Sim's body below Mach 1.2, at Mach 0.6 to 1.2.
4. **Find which body term carries HPR Sim's excess moment** (3 to 9 per rad about the 70% center) before changing the
   body model. Candidates, none sized: the boattail's slender-body −1.32 per rad under a thick boundary layer (+6.6
   per rad of nose-up moment about that center), the body-lift station, the nose's slender-body CP.

## Open questions

- **Is the fins' share really centered ahead of the fins** at Mach 0.8 to 1.0, or is it fitting slopes and CP on
  different angles? Reading the fins-on C_m of Figs. 7(c) and 9(c) on the same angles would settle it.
- **Cone-cylinders:** none found. NASA TN D-3088 (blunted cones alone, Mach 0.5 to 5.0) is a lead, not read.
- **Gwin and Spring (ABMA, 1961), tangent ogive-cylinders at Mach 0.2 to 1.5,** cited by NWL TR-2796: no copy found
  (DTIC's public search offline in a browser, 2026-11-05; no hit on Google or the Internet Archive).
- **Wall effects** at Mach 1.0 (AEDC 1T) and 1.1 to 1.3 (AEDC 4T, Martin's moment offset) are unbounded.

## Sources (all read 2026-11-05)

**Terms.** All sources are public: NTRS marks TN D-4013 and both Allen sets "GOV_PUBLIC_USE_PERMITTED"; the others
carry public-release statements. Readings with figure and page may be committed, as `arcas-robin-wind-tunnel.json`
does. Copies and transcriptions are cached under `refs/research/`.

1. J. C. Ferris, NASA TN D-4013, 1967: pp. 2–4, 6; Figs. 1, 7, 9. <https://ntrs.nasa.gov/citations/19670020050>
2. J. M. Allen, NASA/TM-2001-210652, 2001, Table 4(kk), PDF pp. 164–165. <https://ntrs.nasa.gov/citations/20010020654>
3. J. M. Allen, NASA/TM-2005-213541, 2005, and its data supplement, configuration 6, runs 1430–1456.
   <https://ntrs.nasa.gov/citations/20050160470>, <https://ntrs.nasa.gov/citations/20050165156>
4. G. R. Gomillion, AEDC-TR-75-125 Vol. II, 1976, parts 196–202, PDF pp. 123–129.
   <https://archive.org/details/DTIC_ADB010054>
5. R. E. de Kuyper, Calspan AA-4017-W-11, 1974, runs 10, 12, 16, 28, 27, PDF pp. 103–108.
   <https://archive.org/details/DTIC_AD0782208>
6. E. J. Roschke, BRL Technical Note 902, revised 1955, Table 2, PDF p. 10. <https://archive.org/details/DTIC_AD0061551>
7. E. G. Allee Jr., AEDC-TR-73-44, 1973, Figs. 11a, 12a. <https://archive.org/details/DTIC_AD0758817>
8. *USAF Stability and Control DATCOM*, 1978, §4.2.1.1 and §4.2.2.1. <https://archive.org/details/DTIC_ADB072483>
9. C. D. Babb, D. E. Fuller, NASA TN D-4014, 1967: Fig. 1(a), Fig. 5(a). <https://ntrs.nasa.gov/citations/19670020031>
10. Set aside (NTRS ids): TM X-3128 19750004825, RM L53L28a 19930088063, RM L52J01 19930087341, RM L52D21a
    19930087194, TN 4362 19930085285, TN D-3203 19670003839; NAVORD 2934 <https://archive.org/details/DTIC_AD0775892>
