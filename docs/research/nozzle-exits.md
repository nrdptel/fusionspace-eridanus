# Nozzle exits of the common high-power reloads

This note asks what the nozzle exit diameter is for today's most-flown 38–98 mm AeroTech RMS and Cesaroni reloads,
and how close the geometric bounds of [motor data](motor-data.md) §2 come to it. The exit sets the thrust a motor
gains at altitude, $(p_0 - p) A_e$, which HPR Sim leaves out ([motor model](../physics/motor.md)). It is for whoever
builds the thrust-at-altitude idea ([ideas](ideas-design-and-physics.md)) and for [apogee bias](apogee-bias.md) §1.
AeroTech's exits are the maker's published numbers; Cesaroni's are bounds only. Pressure-thrust shares are
**Inference**: this note's arithmetic at one field height. Sources read 2026-10-15 unless marked.

**In short.** AeroTech publishes the exit; Cesaroni doesn't; nobody publishes a measurement.
- **RCS gives an exit per nozzle part.** RCS, AeroTech's parent, lists throat, exit and OD for 22 single-throat and
  3 Medusa (multi-port) nozzles in its store. Matching the parts named on the 376 reload drawings gives an exit for
  **213 of the 215 RMS reload drawings at 38–98 mm**, up from 15 drawings that state one (§1).
- **The L1500T's exit is 1.75 in.** Its nozzle was redesigned in 2003 "for net molded exit/throat"; the 1998
  letter's 1.4 in is, by inference, the older part. Its pressure thrust goes from +0.99% to about +1.5% (§2).
- **At a 1,300 m field the drawn exits add 0.63–2.92% of average thrust** on 16 AeroTech reloads (median 1.37%),
  about +0.4% to +3.8% of apogee left out (§3). The cap-bore bound overstates the exit's area 1.4–2× on most
  single-throat nozzles of 29–75 mm, and 3–9× on the 01780, the two smallest molds and the Medusas.
- **Cesaroni publishes no exit or throat.** Its aft-boss bounds stand; a scaled outline puts the standard Pro75
  nozzle near 1.75 in against the 2.200 in XL bound (§4).
- **Flyers publish simulator inputs, not measurements.** Of 41 Cesaroni exits in public simulator inputs (one of
  them a forum post), 25 exceed the aft-boss bound and 6 are wider than the case; none says it was measured (§5).

## 1. RCS gives an exit for each nozzle part

RCS's store lists each molded nozzle with its throat, exit, OD and mass [R1, R2], and most pages link a part
drawing [R3]. On the drawings in the table below, the dash numbers mark in-house drilling of the throat, so all
reloads using one of these families share its exit [R3, R4]. (The 01600, used only by DMS motors below 38 mm, is the
exception: its dash numbers open the exit too.) A text rule read every nozzle line in the bills of material of the
325 RMS and 51 DMS drawings [R5]; the case comes from each drawing's title. Of 215 RMS drawings at 38–98 mm, 213 map
to a part with a stated exit, and 44 of 51 DMS motors do. The two left out: the J615ST's 01680 aerospike, and the
L400W-PS, whose plain 01800-3 is drilled to 0.500 in, below the part's 0.734 in molded throat (**Inference**).

| Part | Case, mm | Throat molded, in | Exit, in | Used by (examples) [R5] |
|---|---|---|---|---|
| 01400 | 29/38 | 0.104 | 0.250 | G61W, G67R |
| 01450 | 29/38 | 0.141 | 0.313 [R4] | H73J, H112J, I59WN |
| 01500 | 29/38 | 0.180 | 0.438 | H123W, H148R, I161W (78 drawings) |
| 01550 | 29/38 | 0.291 | 0.688 | J350W, J420R, J570W (48 drawings) |
| 01670 | 54 | 0.455 | 1.250 | K550W-L, K700W, K1275R, L1090W |
| 01700 Medusa | 54 | 0.192 center, 6 × 0.125 | 0.500 center, 6 × 0.375 | J90W, J135W, J415W, K1100T-M |
| 01750 Medusa | 75 | 0.344 center, 6 × 0.188 | 0.750 center, 6 × 0.562 | K560W, K1000T, L850W |
| 01770 | 75 | 0.685 | 1.875 | L1256WS, L1520T, L2200G, M1850W |
| 01780 | 75 | 0.485 | 1.250 | K750ST, K1800ST, M650W (rev C) |
| 01800 | 98 | 0.344 | 0.900 | K458W, K680R, K650T, L952W, L1300R, L339N-P |
| 01800-3M, -xM | 98 | 0.734–1.000 | 1.750 | L1500T, K1999N, M1600R, M2400T, N1000W, N3300R |
| 01880 | 98 | 1.000 | 2.737 [R1, R4] | M2500T, N2220DM; DMS N1600W–O5500X |
| 01850 Medusa | 98 | 0.413 center, 6 × 0.250 | 1.000 center, 6 × 0.750 | M750W |

A Medusa flows through its center port and the outer ports its dash number drills (01700-10: center plus three).
The 01450, 01770 and 01880 drawings were found on RCS's file host, not linked from a page read [R4].

## 2. The L1500T: two exits, two nozzles

The 01800-3 drawing's revision B of 2003-03-11 reads "redesigned for net molded exit/throat": 1.750 in exit,
0.734 in throat [R3, revision and title blocks]. The store sells it as the 01800-3M with the same numbers [R2]. The
older 01800 has a 0.900 in molded exit, its -1 and -2 drilled to 0.413 and 0.594 in [R3]; six 98 mm reloads still
list it on their drawings [R5]. **Inference:** the 1998 TMT letter's 1.4 in (motor-data S10) describes the
pre-2003 nozzle, and today's L1500T flies the 1.75 in of its 2007 drawing (motor-data M5). The drawn exit has 1.56
times the letter's area, so motor-data §2's +0.99% of average thrust at 1,300 m becomes about +1.5%.

## 3. AeroTech exits against the bounds, and what they add

**The cap bound is the nozzle's aft boss.** On every part drawing the boss diameter equals the bore of the plastic
cap the reload drawing lists: 0.625 in on 01400–01500, 0.813 in on 01550, 1.500 in on 01670 and 01700, 2.250 in on
01750–01780 [R3, R5]. The added thrust is $\Delta F = \Delta p \, A_e$, with $\Delta p$ = 14.7 kPa between sea level
and 1,300 m (ISA, as motor-data §2); it doesn't depend on the throat or chamber pressure. A Medusa counts as one
hole of the same total area. The share is of ThrustCurve's average thrust [T2], so it is also the share of impulse.

| Reload (case) | Part | Exit, in (mm) | Area ratio | Exit/cap, d · A | Exit/case, d · A | ΔF, N | Share |
|---|---|---|---|---|---|---:|---:|
| H123W (38) | 01500 | 0.438 (11.1) | 5.9 | 0.70 · 0.49 | 0.29 · 0.086 | 1.4 | 1.14% |
| I59WN (38) | 01450 | 0.313 (8.0) | 4.0 | 0.50 · 0.25 | 0.21 · 0.044 | 0.7 | 1.15% |
| J350W (38) | 01550 | 0.688 (17.5) | 5.6 | 0.85 · 0.72 | 0.46 · 0.21 | 3.5 | 0.79% |
| J90W (54) | 01700-1 | 0.500 (12.7) | 3.5 | 0.33 · 0.11 | 0.24 · 0.055 | 1.9 | 2.11% |
| K550W-L (54) | 01670 | 1.250 (31.8) | 7.5 | 0.83 · 0.69 | 0.59 · 0.35 | 11.6 | 2.92% |
| K550W-M, -S (54) | 01700-10 | 0.820 eq. (20.8) | n/a | 0.55 · 0.30 | 0.39 · 0.15 | 5.0 | 1.26% |
| K700W (54) | 01670-5 | 1.250 (31.8) | 5.1 | 0.83 · 0.69 | 0.59 · 0.35 | 11.6 | 1.66% |
| K1000T (75) | 01750-2 | 1.353 eq. (34.4) | n/a | 0.60 · 0.36 | 0.46 · 0.21 | 13.6 | 1.28% |
| L1256WS (75) | 01770 | 1.875 (47.6) | 7.5 | 0.83 · 0.69 | 0.64 · 0.40 | 26.2 | 2.08% |
| L1520T (75) | 01770-5 | 1.875 (47.6) | 6.8 | 0.83 · 0.69 | 0.64 · 0.40 | 26.2 | 1.67% |
| L2200G (75) | 01770-1 | 1.875 (47.6) | 5.9 | 0.83 · 0.69 | 0.64 · 0.40 | 26.2 | 1.19% |
| M1850W (75) | 01770-4 | 1.875 (47.6) | 4.9 | 0.83 · 0.69 | 0.64 · 0.40 | 26.2 | 1.37% |
| L1500T (98) | 01800-3M | 1.750 (44.5) | 5.7 | no cap | 0.45 · 0.21 | 22.8 | 1.52% |
| M2500T (98) | 01880 | 2.737 (69.5) | 7.5 | no cap | 0.71 · 0.50 | 55.8 | 2.23% |
| N1000W (98) | 01800-6M | 1.750 (44.5) | 5.0 | no cap | 0.45 · 0.21 | 22.8 | 2.11% |
| N3300R (98) | 01800-5M | 1.750 (44.5) | 3.1 | no cap | 0.45 · 0.21 | 22.8 | 0.72% |
| L952W (98) | 01800-2 | 0.900 (22.9) | 2.3 | no cap | 0.23 · 0.054 | 6.0 | 0.63% |

Throats from each reload's drawing [R5]; area ratio is exit over throat area; "eq." is one hole of the ports'
total area. 12 of these 16 reloads are ThrustCurve's most viewed (§5); the L1500T, L1520T, K1000T and L952W are
added for their history or their part. Script: `scripts/ratios.py` in the private cache.

- **Single throat, 01500–01770:** the exit is 0.70–0.85 of the cap bore across, 0.49–0.72 in area, so the cap
  bound overstates $\Delta F$ 1.4–2.0×. The 01780 sits at 0.56 · 0.31 (3.2×), the 01450 at 0.25 in area (4×),
  the 01400 at 0.16 (6×). **Medusa:** 0.11–0.36 of the cap's area, 2.8–9×. **98 mm:** 0.05–0.50 of the case.
- **Apogee (Inference).** At 0.6–1.3% of apogee per 1% of impulse ([apogee bias](apogee-bias.md) §1), 0.63–2.92%
  of impulse is about +0.4% to +3.8% of apogee, which the simulator leaves out at a 1,300 m field (it reads low).
  The rocket's climb during the burn lowers the outside pressure further and adds more; that is not bounded here.

## 4. Cesaroni: bounds only

- **No nozzle numbers.** The Pro75 instructions [C4, pp. 11–15], Pro98 notes [C3, pp. 1–2], FAQ [C5] and 284
  product pages (motor-data §1) give none. In 2014 a flyer helping a dealer reported that Cesaroni declined to
  release a nozzle chart; a Cesaroni staff member replied that dealers should ask, reload by reload [F1, posts 1–2].
- **The aft-boss bounds stand:** Pro54 1.25 in, Pro75 2.200 in (XL nozzle), Pro98 2.05 in, or 2.70 in for the 13
  reloads the Pro98 notes first list as XL-style (a second list swaps the O3400 for the N5600) [C1–C3]. Pro38 has
  no boss; only the 1.500 in case caps its exit [C2].
- **The standard Pro75 nozzle is about 1.75 in (Inference).** The 2011 Gen1 sheet draws its aft end proud of the
  ring, undimensioned [C1, p. 1]. At 400 dpi it spans 350 px; the 2.965 in case spans 593 px and the XL nozzle
  440 px, which reads its stated 2.200 in, so 200 px per inch gives 1.75 in (±0.02 in). No Pro75 reload is listed
  by nozzle style, so 2.200 in stays the safe bound.
- **What the bounds allow at 1,300 m (Inference):** 0.8–2.2% of average thrust for the K–O reloads checked
  (K1440 0.81%, N5800 0.94%, O3400 1.59%, M1520 2.05%, M1670 2.16% at 2.200 in or 1.37% at 1.75 in), and 8.5% for
  the low-thrust 396I140 under the Pro54 boss [T2]. Real exits sit below these by an unknown factor.

## 5. What flyers publish: simulator inputs, not measurements

No public source found reports a measured exit for a 38–98 mm COTS nozzle. Four forum threads and a chart were
read; web searches for Spaceport America Cup and Student Launch reports found none that states a COTS exit, and
the one report read gives none [P2]. The searches found no photo of a nozzle with a scale.
- **Forum numbers trace back to RCS.** The N4000W thread settled on the 01880's 2.737 in [F3]; the M685W thread on
  the 01780-2's 1.25 in exit and 0.627 in throat [F2]. The Info Central chart's "exit diameter" column holds throats
  (0.180 in for the H123W) and still shows the K700W with a Medusa nozzle, replaced by the 01670 in 2004 [F5, R5].
- **Cesaroni inputs are mostly too wide.** A GitHub code search for `nozzle_radius` with "Cesaroni" or
  "AeroTech" (139 files cached) and for `NozzleExitDiameter` (82 RASAero II files), RocketPy's example notebooks
  [P1] and one forum post [F4] gave 41 distinct Cesaroni (reload, exit) pairs [G1, G3]. Against the aft-boss bound
  (±1% counted as "at"): 12 within, 4 at, 25 over (61%), 6 wider than the case. None says it was measured; one
  says "estimated", one is marked "find". RocketPy's Erebus example (a Pro54 K, 1.59 in), not among the 41, is over.
- **RocketPy's own examples are guesses.** Its M1670 (Pro75-5G) exit, 66 mm, is 1.18 times the 2.200 in bound
  (1.40 in area), reused for seven other Cesaroni reloads in four repositories [P1, G1]; its L1395 (49.5 mm, 0.89)
  is the one example inside, its K650 (90 mm) wider than the 54 mm case. **Inference:** the inputs drift because
  RocketPy adds $(p_{ref} - p) A_e$ only when `reference_pressure` is set, which no example does [P1, `motor.py`];
  there the exit feeds only jet-damping terms.
- **AeroTech inputs match RCS only where they copy RCS.** A J250W (0.688 in) and one RASAero II file's O5500X
  (2.737 in) match; RocketPy O5500X inputs use 2.89–3.86 in. An N3300R input uses the 01880's 2.737 in against its
  drawn 1.75 in. An example K828FJ's radii, 44.45 and 21.44 mm, equal the 98 mm nozzle's 1.75 in exit and 0.844 in
  throat as diameters (**Inference**: diameters entered as radii) [G2, P1].

No count of flights by reload is published. The proxy for "most flown" is ThrustCurve's "Most Popular" page [T1],
which ranks page views "over the last couple of months": interest, not flights. On 2026-10-15 its 38–98 mm
high-power reloads were the 12 AeroTech reloads of §3 not marked as added, and the Cesaroni 396I140, N5800, O3400.

## What this suggests

- **Thrust at altitude can use drawn exits for AeroTech:** a per-part table (§1, 13 families) covers 213 of 215
  RMS reload drawings at 38–98 mm. Dimensions are facts; no RCS document is copied (motor-data M17, ADR-197).
- **Cesaroni keeps the aft-boss bound,** per case size, with 2.70 in for the named Pro98 XL reloads. The scaled
  1.75 in for the standard Pro75 is an inference and should not replace the 2.200 in bound.
- **Apogee-bias §1's L1500T line** should read 1.75 in and about +1.5% of thrust, and its 0.3–1.5% range of apogee
  widens to about 0.4–3.8% at 1,300 m across the AeroTech reloads here.
- **An import check:** an exit wider than the boss or the case flags 25 of the 41 Cesaroni inputs found here. A
  RocketPy-file importer should warn rather than trust the exit.

## Open questions

- Do the K550W-M and -S still ship the Medusa 01700-10? Their drawings date from 2002; the -L moved to the 01670 in
  2004. The answer changes the K550W's $\Delta F$ by 2.3×.
- Which nozzle does the L400W-PS ship with, and have the six 0.900 in 98 mm reloads moved to the -xM?
- Which Pro75 reloads use the XL nozzle? Cesaroni names them only for Pro98.
- A real Cesaroni exit per case size, from a spent nozzle and calipers; and how much the climb during the burn adds.

## Sources

`rmp:` is `https://www.rocketmotorparts.com/`; `cf:` is `https://d3l66gvjdr7rqw.cloudfront.net/Templates/170652/
myimages/`; `trf:` is `https://www.rocketryforum.com/threads/`. Cached under `refs/research/nozzle-exits/web/`,
never committed. RCS and Cesaroni grant no license (motor-data M17, C9); team files, forums: cite only.

1. **R1** RCS, Single-Throat Nozzles, pages 1–2 (22 products), "Dimensions" of each:
   `rmp:SingleThroat_Nozzles/cat1577810_1455491.aspx`, `rmp:index.aspx?pageid=1577810&category=1455491&page=2`.
2. **R2** RCS product pages: 26 sitemap URLs matching nozzle or throat, e.g. `rmp:product/54mm-medusa-nozzle`.
3. **R3** RCS part drawings linked from R2 (18), e.g. `cf:rcs_01670_nozzle_dwg.pdf`, `cf:rcs_018003_nozzle_dwg.pdf`,
   `cf:75mm_hp_small_nozzle__01780_.pdf`: dimension callouts, notes, revision and title blocks.
4. **R4** RCS part drawings on the same host, not linked from a page read: `cf:rcs_01770_nozzle_dwg.pdf`,
   `cf:rcs_01880_nozzle_dwg.pdf` (linked in F3), `cf:rcs_01450_nozzle_dwg.pdf`, `cf:rcs_01600_nozzle_dwg.pdf`.
5. **R5** RCS RMS and DMS assembly drawings, 325 + 51 ([motor data](motor-data.md) M1, M2); read 2026-10-11.
6. **C1–C3** Cesaroni dimension sheets, `http://pro38.com/wp-content/uploads/2024/11/`: `Pro75_dimensions.pdf`
   (4 sheets), `Pro38_dimensions.pdf`, `Pro54_dimensions.pdf`, `Pro98_dimensions.pdf`, `Pro98_notes_V1.6.pdf`.
7. **C4, C5** Cesaroni, Pro75 instructions v5.00 (2026-08-17), `http://pro38.com/wp-content/uploads/2026/08/
   CTI-AERO-MAN-109-v5.00-Pro75-Instructions_EN.pdf`; FAQ, <https://cesaroni-aerospace.com/frequently-asked-questions/>.
8. **F1–F4** TRF threads: `trf:cesaroni-nozzle-throat-diameters-and-compatibility.69243/` (2014);
   `trf:whats-the-nozzle-exit-diameter-for-a-medusa-nozzle.194450/` (2025-10, posts 13, 15);
   `trf:aerotech-n4000-nozzle-exit-diameter.196496/` (2026-03, post 5);
   `trf:how-do-you-find-grain-parameters-for-l645-cesaroni-pro75-simulation.188336/` (2024).
9. **F5** Info Central, AeroTech Nozzles (2020), <https://info-central.rocketlabdelta.com/propulsion/aerotech-nozzles/>.
10. **P1** RocketPy, commit 9bd6ad3 (1.13.0, MIT), <https://github.com/RocketPy-Team/RocketPy>: `docs/examples/`,
    `docs/notebooks/getting_started.ipynb`, `data/rockets/valkyrie/VLK.json`, `rocketpy/motors/motor.py`.
11. **P2** Spaceport America Cup 2018, team 73 report,
    <https://soundingrocket.org/uploads/9/0/6/4/9064598/73_project_report.pdf>.
12. **G1** Public RocketPy scripts with Cesaroni inputs, on github.com: AuroraRocketryTeam/RocketPySimulator,
    HPMR-24-25/HPMR-FlightSimulation, M2I-ARES/Flight-Simulations, thrust-team/SFR0, StrathSEDS-on-GitHub/Mach24,
    Mines-Rocketry-Club/NearHigher; reusing the M1670 input: AnTARES-UGent/simulation, tudsat-rocket/sim_scripts,
    aesirkth/euroc23_electronics, bears-space/rocketpal.
13. **G2** Public scripts with AeroTech inputs: Ryerson-Rocketry/RocketPy, MASA-Dearborn/HADES-Airbrakes,
    legomyleg/byu-hpt-27-telemetry, EricCadar/LRA-OpenRocket, dssong2/fin-iterations (RASAero II).
14. **G3** Public RASAero II files: AerospaceNU/19SU_Karman_Simulations, AIAANortheastern/KarmanSims,
    icl-rocketry/art-designs, stanford-ssi/rockets-irec.
15. **T1, T2** ThrustCurve, Popular Motors, <https://www.thrustcurve.org/motors/popular.html>; API search results,
    average thrust per motor, cached by motor-data (read 2026-10-11).
