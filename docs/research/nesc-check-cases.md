# NASA's six-degree-of-freedom check cases as references for HPR Sim

**In short: NASA's check cases are references for frames, gravity, Earth rotation and rigid-body dynamics, not for
Mach 1.5 to 2.5.** Of the 17 atmospheric cases, 10 fly a sphere or a brick on constant coefficients, 6 an F-16 and
1 a two-stage rocket to orbit [S1 p. 15–16]. None is a finned rocket or a spin-stabilized body, and no case HPR Sim
could use flies a Mach-dependent coefficient. The cases set no tolerance: six codes' trajectories form a "family
of possible solutions" [S1 p. 10]. On the 10 simple cases the worst code is 0.133% to 0.281% off the ensemble,
except one code in case 3 (0.972%) and one in cases 4 and 5 (10.0%) [S2 Table 77]. In feet, altitudes after 30 s
spread from 0.0008 ft to 18.2 ft (**Inference**, §3). The TM and the 2015 paper are US-government works, public use
permitted [S6]; the data files state no terms. Cases 1 and 2 fit HPR Sim with a test-only free body, 6 to 10 also
need a sphere's drag; 3, 4 and 5 need a model it lacks; 11 to 17 do not fit. They would move the Frames, Gravity,
Rigid-body flight and Time integration rows of the [accuracy table](../accuracy.md), not apogee.

**What this covers.** NASA/TM-2015-218675 Volumes I and II [S1, S2], the AIAA papers [S3, S4], the NESC Academy page
[S5] and its 81 time-history files, read or counted 2026-10-18. **How far to trust it.** Case data come from the TM.
Spreads in feet and degrees per second are this note's reading of the cached CSV files, marked **Inference**.
Page numbers are the reports' own headers ("N of 26", "N of 609"). HPR Sim flew no case. For [ADR-143][adr-143] §6b,
[M1.14b, references from Mach 1.5 to 2.5](../decisions-and-roadmap.md#m1-14b) ("each … reported or refused").

[adr-143]: ../decisions/0143-the-operating-envelope-and-a-stop-rule-for.md
[adr-011]: ../decisions/0011-rigid-body-flight-equations-of-motion.md
[adr-003]: ../decisions/0003-frames-attitude-geodesy-and-the-gravity-model.md

## 1. What NESC published

- **Seven tools, anonymized:** Core, JEOD, JSBSim, LaSRS++, MAVERIC, POST II and VMSRTE [S1 p. 13]. Atmospheric
  results are labeled SIM 1 to SIM 6 [S2 p. 92]. JSBSim is the one non-NASA tool, a volunteer open-source project
  [S2 p. 53].
- **No answer key.** The team named no "known good" tool and no pass tolerance; agreeing trajectories serve as a
  guide [S1 p. 10]. A case needed three tools to be published [S1 p. 13].
- **Scope.** 17 atmospheric cases, 16 run by three or more tools; case 14 (supersonic F-16) was left out, for lack of
  agreement on its inputs [S1 p. 13] or of resources [S2 p. 53]: the TM gives both. The orbital cases (26 by
  p. 13, 27 by p. 18) are out of scope.
- **Models fixed for all.** The 1976 US Standard Atmosphere, as tables or equations [S1 p. 12]; J2 gravitation on
  WGS-84, or inverse-square gravitation on a sphere, rotating or not [S1 p. 11–12]. Constants include
  J2 = 0.00108262982, μ = 3.986004418e14 m³/s², ω = 7.292115e-5 rad/s and a 6,371,007.1809 m sphere [S2 p. 93,
  Table 73]. Vehicles come in DAVE-ML, an XML model format under ANSI/AIAA S-119 [S1 p. 12–13].
- **Errata of 2025-09-08** open both NTRS PDFs: cases 2 and 3 start at 30,000 ft, not 0 ft [S2 errata, p. 58,
  Tables 26–27]; initial rates are relative to the local north-east-down frame [S2 errata, p. 56]; case 17 has no
  wind [S1 errata].

## 2. The atmospheric cases

Cases 1 to 10 and 17 start at 0° N, 0° E, the F-16's elsewhere; English units [S2 p. 57–68, Tables 25–44].

- **Case 1, dragless sphere** (gravitation, translation). A 1-slug (14.59 kg) sphere, 3.6 slug·ft² on each axis
  [S2 p. 12, Table 1], falls 30 s from 30,000 ft, at rest relative to the Earth, J2 and WGS-84 [S2 p. 57, Table 25].
  Its zero inertial rate reads −0.004178073 deg/s in roll relative to the Earth: the Earth's rate.
- **Case 2, tumbling brick, no aerodynamics** (rotation). An 8 × 4 × 2.25 in brick, three unequal moments:
  0.001894220, 0.006211019 and 0.007194665 slug·ft² [S2 p. 13, Table 4]; inertial rates 10, 20 and 30 deg/s, 30 s
  [S2 p. 57–58, Table 26].
- **Case 3, the brick with damping** (inertial coupling): Clp = Cmq = Cnr = −1.0 on S = 0.22222 ft², b = 0.33333 ft,
  c̄ = 0.66667 ft, no drag [S2 p. 13, Table 5; p. 58, Table 27].
- **Cases 4, 5 and 6, the sphere with CD = 0.1** on 0.1963495 ft² [S2 p. 12, Table 2], 30,000 ft, 30 s: a round
  fixed Earth with inverse-square gravitation (4), the same Earth rotating (5), WGS-84 with J2 (6) [S2 p. 58–60].
- **Cases 7 and 8, wind:** case 6 with a steady 20 ft/s wind from the west, or from the west at (0.003 h − 20) ft/s,
  h in feet (70 ft/s at 30,000 ft) [S2 p. 60–61, Tables 31–32].
- **Cases 9 and 10, a sea-level shot,** 1,000 ft/s up and 1,000 ft/s east (9) or north (10), 30 s. The body's x
  axis starts level [S2 p. 61–62, Tables 33–34], so the sphere starts at 45° angle of attack (**Inference**).
- **Cases 11 to 16, the F-16:** trim, control-law steps, circles round the pole and the date line [S2 p. 62–68].
- **Case 17, two-stage rocket to orbit.** Two 3 m stages, 314,000 kg at launch and 99,000 kg after staging, 17 MN and
  5 MN of thrust, a 96.79 s coast, jet damping at half the mass-tensor change [S2 p. 30–35, Table 16]. Aerodynamics
  vary with angle of attack and sideslip only, noted as "V2 at Mach 3" [S2 p. 30–31]. Launch at 55.22° pitch, 200 s,
  a 0.001 s step advised [S2 p. 68, Table 44]; air density zero above 86 km [S2 p. 33].

## 3. How closely the codes agree

**The TM's measure is a percent of the largest reference value:** the worst difference over the run divided by the
largest magnitude of the ensemble average [S2 p. 588, eq. 38]. Its worst values [S2 p. 591, 595, Tables 77, 81]:

| Case | Worst code | Next worst | Cause the reports give |
|---|---:|---:|---|
| 1 and 2, sphere and brick | 0.139%, 0.133% | 0.030%, 0.036% | |
| 3, brick with damping | 0.972% | 0.264% | |
| 4 and 5, round Earth | 9.998%, 10.011% | 3.333%, 3.337% | SIM 2's truncated gravitational constant [S2 p. 589] |
| 6 to 10, the sphere | 0.142% to 0.281% | 0.079% to 0.142% | atmosphere tables [S4 p. 10]; geodesy in 10 [S2 p. 589] |
| 17, two-stage rocket | 16.790% | 8.398% | SIM 6's mass properties [S2 p. 589]; staging times [S2 p. 590] |

**An outlier shifts the average and so marks the good codes too.** In cases 4 and 5, SIM 2 is 10.0% off and the
other three read 3.33%: 10.0/3, what one outlier does to a four-code mean (**Inference**). In case 9 the 2015 paper
compared to the agreeing SIM 4, 5 and 6 instead [S4 p. 9]. HPR Sim should compare to each code, not to the mean.

**In absolute terms (Inference).** Each file interpolated linearly onto a 0.1 s grid; largest minus smallest:

- **Case 1:** altitude within 0.0021 ft at 30 s, all six codes, after a 14,401 ft fall to 15,599 ft.
- **Case 2:** body rates within 0.005 deg/s, five codes. **Case 3:** 0.074 deg/s, from SIM 1; the others 0.004 deg/s.
- **Case 4:** 18.1 ft at 30 s, SIM 2 alone; the other three within 0.011 ft. **Case 6:** 0.90 ft, six codes.
- **Cases 9 and 10:** 4.3 ft at 30 s, SIM 4 against SIM 3; the paper puts case 9's at "nearly 5 feet" [S4 p. 9],
  explains part by density tables, not SIM 3's [S4 p. 10]. **Case 17:** 55,688 ft at 200 s, SIM 6 apart.

**The effects the cases isolate (Inference, SIM 5's files).** The Earth's rotation (case 4 to 5) leaves the sphere
45 ft higher at 30 s; WGS-84 in place of the round Earth (5 to 6) adds 8.1 ft. Rotation pushes case 1's falling
sphere east at 2.10 ft/s by 30 s, and case 10's northbound shot west at 1.06 ft/s.

## 4. The data files

- **What ships.** `Atmospheric_checkcases.zip` (24.9 MB): 81 CSV files in 19 case folders (counted 2026-10-18):
  6, 5, 5, 4, 4, 6, 6, 6, 6 and 6 for cases 1 to 10, 3 for each F-16 case and for case 17. The TM's 84 trajectories
  [S1 p. 19] are likely these 81 and case 14's three [S1 p. 13] (**Inference**). Also models and a workbook [S5].
- **Columns.** 22 to 38 per file, S-119 names, English units: time, Earth-centered position and velocity,
  north-east-down velocity, latitude, longitude, altitude, local gravity, Euler angles, inertial body rates, air
  density, pressure, temperature, speed of sound, aerodynamic forces and moments, Mach, dynamic pressure.
- **Rates.** Most files are at 0.1 s; SIM 5's at 0.01 s in cases 1 to 10 and 17, and SIM 4's in case 17 (counted).
  The codes stepped at 10 to 1,000 Hz mostly with RK4 or Adams-Bashforth methods [S2 p. 50–53, Tables 20–22].
- **Two traps.** `localGravity` is gravitation without the centrifugal term: 32.10654 ft/s² at case 1's start, as
  J2 gives (**Inference**). Body rates are inertial (`bodyAngularRateWrtEi`).

## 5. Terms

- **The TM (both volumes) and the 2015 paper:** NTRS lists each as "Work of the US Gov. Public Use Permitted",
  public distribution [S6]. Cache under `refs/` and cite; their text and figures are not copied.
- **The 2013 interim paper:** "Copyright © 2013 by the American Institute of Aeronautics and Astronautics, Inc.",
  with a government-use license [S3 p. 1]. Cite only.
- **The data files and models:** the NESC page states no license or terms [S5]. NASA's media guidelines say its
  material is generally not under US copyright, and that third-party material is marked [S7]. But the CSVs are
  anonymous outputs of six tools, one non-NASA (JSBSim; JEOD ran orbital cases only) [S1 p. 13; S2 p. 50, 53], so
  no file can be called a government work.
  **Cite and cache under `refs/`, pinned by sha256; never commit files or excerpts.** Derived statistics may be
  published; committing an extract would be a licensing call for the maintainer.

## 6. What each case would test in HPR Sim

**What HPR Sim checks today** ([accuracy](../accuracy.md)): a vacuum parabola and a terminal velocity in uniform air;
torque-free precession of an axisymmetric body; coning attitude over 1e6 steps; WGS-84 gravity against 40-digit
formulas; the 1976 atmosphere's tables at 32 heights; Coriolis direction and size in unit tests; RocketPy and
OpenRocket flights. The Frames, Rigid-body flight and Time integration rows have no published-source check.

**How HPR Sim differs from the cases.** It integrates in an Earth-fixed east-north-up frame at the pad, with WGS-84
normal gravity (centrifugal included) plus Coriolis ([Gravity](../physics/gravity.md), [ADR-003][adr-003]). Its
rotational equations use rates relative to that frame ([ADR-011][adr-011]). Its axial force acts along the body
axis, and `hpr-aero` has no pitch or yaw damping coefficients ([flight](../physics/flight.md#aerodynamics-in-flight)).

- **Case 1: fits.** New: a 30 s, 14,400 ft fall in a rotating Earth's field against six codes. HPR Sim's normal
  gravity is 4.4e-6 stronger than J2 less the centrifugal term at 30,000 ft (**Inference**), which should leave it
  about 0.06 ft low, 30 times the 0.0021 ft spread: a known model gap, to state, not fix.
- **Case 2: fits.** New: torque-free tumbling with three unequal moments (today's test needs two equal). HPR Sim
  starts from the Earth-relative rates and must report inertial ones. Leaving the Earth's rate out of the rotational
  equations drops terms of order ω·Ω ≈ 4.8e-5 rad/s² (**Inference**); against the 0.005 deg/s spread, unbounded.
- **Case 3: needs** constant roll, pitch and yaw damping on a finless body; HPR Sim's roll damping comes from fins.
  New: damping on three axes. Until then, refused.
- **Cases 4 and 5: need** inverse-square gravitation on a sphere. HPR Sim's four gravity models are a constant and
  three WGS-84 forms; on its default it would sit about case 6's 8.1 ft from case 5, model not code (**Inference**).
- **Case 6: fits** once drag acts along the air-relative velocity, as a sphere's does: body x points north and the
  sphere falls along body z, at 90° angle of attack, where axial-only drag gives no force (SIM 5's drag is all on
  body z [S5 data]). New: a drag fall through the layered 1976 atmosphere against six codes, within 0.90 ft.
- **Cases 7 and 8: fit** with the same drag. New: steady and sheared wind on a free body.
- **Cases 9 and 10: fit,** with the same drag (45°). New: a 6.9 km shot, where the pad's tangent plane sits 3.7 m
  (12 ft) above the ellipsoid (**Inference**): a test, inside a 4.3 ft spread, of the conversion HPR Sim uses for
  apogee and landing ([Frames](../physics/frames.md#launch-frame-l-east-north-up)); 10 adds Coriolis on a path.
- **Cases 11 to 17: refused.** 11 to 16 fly an aircraft under flight control. Case 17 is orbital, past Mach 9 in the
  files, 314 tonnes, aerodynamics without Mach, its codes 16.790% apart; staging is checked against OpenRocket.

**What all need.** A test-only free body of given mass and inertia, no motor or design tree, started at a set state as
`vacuum_ballistic_flight_follows_the_parabola` does; no geoid offset; the TM's exact unit factors [S2 Table 73].

## 7. Mach band

**None is a Mach 1.5 to 2.5 reference.** In SIM 5's files, cases 1 to 8 peak at Mach 0.82 to 0.91, and cases 9 and
10 at Mach 1.27, at launch, on a constant CD (**Inference**). The F-16's case 12 reaches Mach 2.01, as an aircraft.
Case 14, on "Mach effects in tables", was never finished [S1 p. 16]; case 17's Mach was left out of the comparison
[S2 p. 590]. They are frame, gravity, rotation and integrator references, at any speed.

## What the project can use

- **For M1.14b's done-when.** Report the item as not a Mach 1.5–2.5 reference: cases 1, 2 and 6 to 10 adopted as
  checks (6 to 10 once a sphere's drag exists); 3, 4 and 5 deferred for a model; 11 to 17 refused (§6). The band
  gains nothing.
- **Pin and cache.** Add `Atmospheric_checkcases.zip` (sha256 `1a83df6d…`), `Atmospheric_models.zip` (`10e2dc26…`)
  and both TM volumes to `validation/refs.lock.toml` as `fetched`, with a `THIRD-PARTY-NOTICES.md` row: reports a
  US-government work; data of unstated terms, cached, not committed.
- **A check-case runner (suggestion).** A test that flies cases 1 and 2 against each code's file and prints the
  difference over time would give Frames, Gravity, Rigid-body flight and Time integration a row against six
  independent codes. Expected: within each spread, except case 1's 0.06 ft, which the gravity model explains
  (**Inference**). Cases 6 to 10 follow once a sphere's drag can be flown. Apogee does not move.

## Open questions

- Does leaving the Earth's rate out of the rotational equations ([ADR-011][adr-011]) show in case 2's rates beyond
  the 0.005 deg/s spread? Only flying it will say.
- Case 17's text gives a 0.1 ft/s start [S2 p. 33]; its table gives 0.32808399 ft/s, which is 0.1 m/s [S2 p. 68].

## Sources

1. **S1.** D. G. Murri, E. B. Jackson, R. O. Shelton, *Check-Cases for Verification of 6-Degree-of-Freedom Flight
   Vehicle Simulations*, NASA/TM-2015-218675/Vol. I, NESC-RP-12-00770, 2015, errata of 2025-09-08,
   <https://ntrs.nasa.gov/citations/20150001263>. Read 2026-10-18.
2. **S2.** The same, Volume II: Appendices, errata of 2025-09-08, <https://ntrs.nasa.gov/citations/20150001264>.
   Read 2026-10-18: B.1, B.4–B.6, C.1, C.3, D intro, E.1.
3. **S3.** E. B. Jackson et al., "Development of Verification Check-Cases for Six Degree-of-Freedom Flight Vehicle
   Simulations", AIAA 2013-5071, 2013,
   <https://nescacademy.nasa.gov/src/flightsim/Reports/aiaa-13-5071-EOM_chkcases.pdf>. Read 2026-10-18, p. 1.
4. **S4.** E. B. Jackson, M. M. Madden, R. Shelton et al., "Further Development of Verification Check-cases for
   Six-Degree-of-Freedom Flight Vehicle Simulations", AIAA 2015-1810, SciTech 2015,
   <https://nescacademy.nasa.gov/src/flightsim/Reports/aiaa-15-1810-EOM_chkcases-II.pdf>. Read 2026-10-18, pp. 1–10.
5. **S5.** NESC Academy, "Six degree-of-freedom (6-DOF) Flight Simulation Check-cases", updated 2015-02-20,
   <https://nescacademy.nasa.gov/flightsim/2015>, with `src/flightsim/Datasets/Atmospheric_checkcases.zip`,
   `src/flightsim/Models/Atmospheric_models.zip` and `src/flightsim/Scenarios/Initial_Conditions.xlsx`. Read and
   fetched 2026-10-18. The 2023 set at <https://nescacademy.nasa.gov/flightsim> is lunar orbits only.
6. **S6.** NTRS records 20150001263, 20150001264 and 20150006038 (the 2015 paper), copyright field. Read 2026-10-18.
7. **S7.** NASA media guidelines, <https://www.nasa.gov/nasa-brand-center/images-and-media/>. Read 2026-10-18.
