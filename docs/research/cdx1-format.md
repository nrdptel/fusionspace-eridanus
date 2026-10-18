# What is publicly known of RASAero II's `.CDX1` design file

**In short: a `.CDX1` is plain XML in fixed US customary units, and every file found says `FileVersion` 2.** It has no
published schema, but the RASAero II manual, its screens and 159 public sample files settle the parts that matter for
[M3.5, `.CDX1` import and export](../decisions-and-roadmap.md#m3-5). Lengths are inches, weights pounds, altitudes and
rail feet [S1, S2]. A part's `Location` is its station from the nose tip. A fin's `Location` runs from its root leading
edge **to the aft end** of its tube, as the manual says (p. 13). All 20 installer designs that also ship as RASAero (I)
files agree to 0.01 in. That settles Loft lesson L73 ([Loft lessons](loft-lessons.md)), but the opposite way from
Loft's code. Two programs write most files: RASAero II and, since 2023, OpenRocket's exporter, and they differ in ways
a reader must handle. The file holds no part masses: one loaded weight and CG per stage per simulation, motor included.
No file states an overall length, so M3.5's *done when* ("within 0.5% of the stated values") needs a definition.

**What this covers.** The format's elements, units, defaults and versions, where parts sit, what maps to HPR Sim's
design tree each way, and the samples with their rights. Sources: the RASAero II 1.0.2.0 manual and installer [S1, S2],
135 distinct files from 51 GitHub repositories [S6], Loft's reader [S11] and OpenRocket's issue texts [S7–S9]. **How
far to trust it.** Counts come from scripts run over cached files on 2026-10-17; units and placement rest on manual
screens whose values match sample files. No `.CDX1` was imported into HPR Sim, and RASAero II was not run. It builds on
[RASAero II's comparison flights](rasaero-comparison-flights.md) §1.

## 1. Structure and units

The root `<RASAeroDocument>` holds, always in this order, `FileVersion`, `RocketDesign`, `LaunchSite`, `Recovery`,
`MachAlt` and `SimulationList`, with no XML declaration in files RASAero II writes [S2, S6]. `RocketDesign` lists the
parts nose to tail, then the rocket-wide settings. There is no units setting: the Options menu lists launch site,
Mach-Alt, recovery, turbulence, Barrowman, surface finish and comments only (p. 78), so the file's units never change
(**Inference**). The decimal mark does: RASAero II writes the Windows locale's comma (`13,05`) in 6 public files from 2
repositories [S5, S6].

- **Inches:** every length, diameter and station (`Location`, `Chord`, `BluntRadius`, `LERadius`), and the CGs, from
  the nose tip. "All dimensions are in inches" (p. 13); Fig. 117's CG of 86 in and nozzle of 2.5 in equal a sample's
  stored values (p. 100).
- **Pounds, loaded:** `SustainerLaunchWt`, `Booster1LaunchWt`. Fig. 117's "Loaded Wt (lb)" of 70 equals the file (p.
  100); §4 checks that the motor is included.
- **Parachutes:** `Size1/2` is the flat diameter in inches; Fig. 98's 96 and 160 equal a sample (p. 82). `Altitude1/2`
  is feet above ground (p. 81–82).
- **Site** (p. 78–80): altitude in ft above sea level, °F, pressure in inHg reduced to sea level (0 = unset), wind in
  mph (positive = headwind), rail in ft, rail angle in degrees from vertical.
- **Areas** in in²: `LaunchShoeArea` (p. 21; Fig. 50's 0.03310 equals a sample, p. 42) and protuberance areas.
- **Stored results:** `TimetoApogee` s, `MaxAltitude` ft, `MaxVelocity` ft/s. A sample's comment quotes 3,063 ft/s and
  81,499 ft for its stored 3,062.629 and 81,498.75 [S2].

**The parts** (counts are files holding the part: installer of 24 / public of 135) [S2, S6]:

- Every part has `PartType`, `Length`, `Diameter`, `Location` and `Color`. The one `NoseCone` sits at 0 with `Shape`,
  `BluntRadius`, and `PowerLaw` (the exponent) only with the power-law shape (0 / 5 files).
- `BodyTube` (1 to 7 a file): launch lug, rail guide and launch shoe sizes, an inline boattail (`BoattailLength`,
  `BoattailRearDiameter`, `BoattailOffset`), `Overhang`, then an optional `Fin` and `Protuberance`.
- `FinCan` (9 / 10), `BoatTail` (4 / 51), `Transition` (0 / 24) and `Booster` (1 / 35), each with its own fields.
- `Fin`: `Count`, `Chord` (root), `Span`, `SweepDistance`, `TipChord`, `Thickness`, `LERadius`, `Location`,
  `AirfoilSection`, and `FX1`/`FX3`, the leading- and trailing-edge diamond lengths. Fig. 12 (p. 13) matches a sample
  field for field. Every fin in the files is a trapezoid (**Inference** from the census).
- `Protuberance` (0 / 19): summed frontal areas of streamlined bumps with and without base drag, and two inclined
  plates by angle and area (p. 24–25).
- Rocket-wide: `Surface`, `CP` (0 in every file), `ModifiedBarrowman`, `Turbulence`, the stage nozzles, `Comments`.
- `Recovery`: two events, each with `Event` (the check box), `DeviceType`, `EventType` (Apogee, Altitude, None, empty),
  `Altitude`, `Size` and `CD`. An event fires only when its box is checked (p. 81). `MachAlt` affects Run Test only,
  never the flight simulation (p. 84).
- `Simulation` (none in 16 public files, one in 57, more in 62): per stage the motor (`designation  (maker code)`),
  launch weight, CG, nozzle diameter and delays; `IncludeBooster1/2`; and the stored results.

**Enumerations and defaults.** `Shape` takes Tangent Ogive, Conical, Von Karman Ogive, LV-Haack, Elliptical, Power Law
and Parabolic (p. 10). `AirfoilSection` takes Hexagonal, Rounded, Square, Double Wedge, Single Wedge, Hexagonal Blunt
Base and "Subsonic NACA"; the manual's Biconvex (p. 14) appears in no file. `Surface` takes the 8 names of the manual's
Table 1, each an equivalent sand roughness from 0 (Smooth) to 0.01 in (Cast Iron) (p. 53). The defaults are a smooth
finish, plain Barrowman, laminar flow to Re 500,000 then turbulent, a parachute drag coefficient of 1.33 and a fin
leading-edge radius of 0 (p. 16, 53–56, 82). A new rocket takes the last launch site used (p. 79); the installer's
`LSD.xml` holds 0 ft, 74 °F, a 4 ft rail and no wind [S2].

## 2. Where the parts sit

- **Parts sit at absolute stations.** Each part's `Location` equals the previous part's `Location` + `Length` in 36 of
  38 installer links and 422 of 436 public ones (comma-decimal files aside) [S2, S6]; the rules below explain each
  break.
- **A fin can is a sleeve.** Its `Location` is the aft face of the tube it covers, its `Offset` is −`Length`, and its
  `InsideDiameter` is that tube's diameter, in 9 of 9 installer files and 10 of 10 public ones. The next part chains
  from the tube (p. 33).
- **A nested booster starts at the top of the sustainer's short boattail** (p. 39–43), so it overlaps the part before
  it. `ShoulderLength` is the transition length between stages: nonzero in 26 of 26 public RASAero II cases with
  unequal diameters and in the installer's one, and 0 in 7 of 7 with equal ones [S2, S6].
- **A fin's `Location` runs from its root leading edge to its parent's aft end.** The manual: the distance "from the
  start of the fin to the bottom of the body tube" (p. 13). RASAero (I) stores the leading edge from the nose instead.
  In all 20 installer pairs, the tube's aft end minus the fin `Location` equals it to 0.01 in [S2]. Raven: 73.5 −
  9.370003 = 64.13 in, the `.ALX1`'s value. On public tubes and boosters at least three chords long, 122 of 122 fins
  sit in the aft half under this reading [S6]. Measured from the tube's front, as Loft does, 15 of 24 installer designs
  would put the fins' root midpoint ahead of the stated CG.
- **RASAero II writes the boattail twice.** A tube's inline boattail repeats the next `BoatTail` part in 3 of 3
  installer and 41 of 41 public RASAero II files, and in none of 12 OpenRocket-written ones (9 public, 3 private). A
  reader builds the cone once (Loft lesson L74).
- **No file states an overall length.** No element holds one, and none of the 24 installer comments gives one (every
  number with a unit is an altitude, speed or time). The extent of the `Location` chain is the only in-file value. It
  agrees with the 20 `.ALX1` twins and, within 0.20%, with the installer's RockSim file of one design [S2]. Adding part
  lengths end to end, Loft's L72 bug, is wrong in 10 of 24 files, by +4.4% to +14.4%.

## 3. Versions and writers

- **One version.** `FileVersion` is 2 in all 24 installer files, all 135 public ones and the private collection. 21
  public files, in repositories last pushed in 2016–2018, share the layout: an earlier release, such as the 1.0.1.0 of
  2016-08-05, wrote them (**Inference**). Only 1.0.2.0 (2019-05-22) is downloadable [S3]; its "new protuberance drag
  models" fit `Protuberance` appearing only in later files (**Inference**).
- **OpenRocket's exporter** (merged 2023-04-20 [S9]) writes `CD` for `CP`, leaves out a motor it can't match in
  `rasp.eng`, doesn't repeat the inline boattail, and writes values such as 29.9999. It wrote the rail in inches until
  a fix merged 2023-11-22 and first released in 24.12 [S7]: 12 of 13 public OpenRocket-written files hold a rail that
  reads as inches, 4 of them 39.37 (1 m) [S4, S6]. A RASAero II re-save turns `CD` into `CP` [S5]. One repository's
  script writes a third kind, with a quoted XML declaration (5 files).
- **`.ALX1` (RASAero I) is another format,** a .NET object dump; RASAero II converts it on open (p. 57).

## 4. What maps to HPR Sim's design tree

**RASAero II to HPR Sim.** The geometry maps well [S10]. Each nose shape has a match: LV-Haack is Haack C = 1/3, power
law needs 0.05 ≤ n ≤ 1. So do tubes, conical boattails and transitions, boosters (as stages) and trapezoidal fins. A
fin can becomes a shortened tube, a conical step and a tube at the can's diameter (**Inference**). The 8 finishes map
exactly as custom roughness heights. HPR Sim's default is 20 µm, RASAero II's 0, so every part needs the height set.
Recovery maps to two devices on apogee or altitude triggers. What HPR Sim can't hold changes RASAero II's drag, so the
importer should warn about each:

- **Fin airfoils** other than square, rounded and NACA (hexagonal, wedges, biconvex), `LERadius` and `FX1/FX3`: 15 of
  the 25 installer fin sets are hexagonal, 3 with a leading-edge radius. HPR Sim has 3 kinds. The apogee moves either
  way, by the mapping chosen; not bounded.
- **A blunt nose tip** (`BluntRadius`, 8 of 24 files): HPR Sim has no tip radius. Dropping it moves the drag by ΔC_D of
  at most 0.013 either way (**Inference**: stagnation pressure on the tip's share of the area); the sign isn't shown.
- **The laminar run to Re 500,000**, RASAero II's default (20 of 24 files): HPR Sim is always fully turbulent, so its
  apogee comes out lower; not bounded. The run is about 7 cm at 100 m/s and 24 cm at 30 m/s, longer aloft, and each fin
  starts its own boundary layer (**Inference**).
- **Launch shoes and protuberances** (3 parts with shoes): no part. Dropping them raises the apogee; not bounded.
- **Rail guides** (17 parts) store a diameter and height only; HPR Sim's rail button needs 5 sizes.

**HPR Sim to RASAero II (export).** A `.CDX1` can't hold secant or bulged ogives, other Haack values, curved
transitions, more than one fin set on a tube, elliptical, freeform or tube fins, fin cant, pods, clusters, more than 3
stages, part masses, per-part finish, drag overrides, or more than 2 recovery devices (p. 10–103). It holds one finish
for the whole rocket from 8 bins, and HPR Sim's default 20 µm falls between two of them.

**Mass.** The file stores a lift-off weight and CG per stage per simulation, and no part masses, materials or inertia.
The weight includes the loaded motor: in 20 of 20 `.ALX1` twins, total minus empty weight equals the motor's loaded
mass in `rasp.eng` to 0.01 lb [S2] (Raven: 5.44 − 3.49 = 1.95 lb; 0.886 kg = 1.954 lb). A booster's weight and CG are
the whole stack, "Total Loaded Wt (lb)" and "Combined CG (in)" (p. 100). HPR Sim can fly this as a stage mass and CG
override net of the catalog motor [S10], exact at lift-off. Pitch inertia then comes from assumed materials, an error
not bounded here.

## 5. Samples and their rights

- **RASAero II 1.0.2.0 installer** `Examples/` [S2]: 24 `.CDX1`, 26 `.ALX1` and a RockSim `.rkt`. No rights granted:
  cite, cache in `refs/`.
- **Public GitHub repositories** [S6]: 135 distinct files from 51 repositories, both writers. 41 repositories have no
  license; 5 are MIT, 1 Apache-2.0 (GitHub reads it as unstated), 1 Unlicense, 2 GPL-3.0 and 1 LGPL-3.0. Their noses
  include LV-Haack (14 files), elliptical (8) and parabolic (4), each just a `Shape` string.
- **A team's paired files** [S5]: an OpenRocket export, its RASAero II re-save and a comma copy. Unlicensed.
- **Loft's demo** [S11]: one hand-written design in 3 copies, MIT. It omits `MachAlt` and places fins from the front.
- **The private collection** (aggregates only): 47 distinct files, 24 identical to installer examples and 6 to public
  ones, so 17 are private-only. One element, `Booster2Engine`, appears only there.

**Readers.** No complete permissive reader exists: pyRATT's (MIT) is a stub [S12], RocketPy (MIT) reads none [S13], and
OpenRocket's (GPL) and FreeCAD's (LGPL) were not read. Loft's [S11] may be ported. Besides L72 to L74 it has two
defects (**Inference**, its code against §1): it reads `Size` as feet, 12 times too wide, and ignores the event check
boxes, so in 6 installer files it would deploy a 36 in chute and a 100 in one that RASAero II leaves off. It skips the
power-law exponent, `BluntRadius`, `LERadius`, nozzles, `Turbulence`, the second booster and decimal commas.

## 6. What this suggests

- M3.5 should define "the stated values" as the `Location` extent, and also check each part's station and each fin's
  leading edge against the file. A 0.5% length bar alone passes a wrong fin datum.
- M3.5's tests: read decimal commas; build the boattail once; treat `CD` as `CP`; read the rail in feet, but flag it in
  OpenRocket-written files (values can't tell: RASAero II files hold 39.37 and 157.48 too); skip unchecked events; read
  `Size` in inches; take `Booster1LaunchWt` as the stack.
- Loft lesson L73's row can now cite the manual (p. 13) and the 20 paired files for the aft-end datum.
- Warn on fin airfoils, blunt tips and the laminar default; the flights of [M2.3c7](../decisions-and-roadmap.md#m2-3c7)
  carry all three.

## Open questions

- What do `RocketDesign/CP` and `BoattailOffset` mean (0 in every file)? How do `UseBooster1` and `IncludeBooster1`
  differ? Only one of 7 private staged files sets `UseBooster1`.
- The design's nozzle diameter differs from the first simulation's in 73 of 119 public files. Which should a reader
  trust? Run Test copies it from the flight screen (p. 87).
- Which K′ is RASAero II's parabolic nose? Where does it put the motor, and what pitch inertia do its wind runs use?
- How much drag does each fin airfoil add in RASAero II against HPR Sim's nearest kind, from Mach 0.8 to 3?
- Which release first wrote `FileVersion` 2? No installer older than 1.0.2.0 is downloadable.

## Sources

Page numbers (p.) are S1's printed page markers. Copies sit in the gitignored `refs/`; all read 2026-10-17.

1. **S1.** C. E. Rogers and D. Cooper, *RASAero II Users Manual*, version 1.0.2.0, 2019.
   <https://www.rasaero.com/dl_manual_ii.htm>. Copyright: cited, not redistributed.
2. **S2.** RASAero II 1.0.2.0 installer, <https://www.rasaero.com/dl_software_ii.htm>: `Examples/`, `LSD.xml`,
   `rasp.eng`, and the installer root's RockSim file. No license: cached in `refs/` only.
3. **S3.** RASAero home and download pages, <https://www.rasaero.com/>.
4. **S4.** UND-ARC, `IREC-ARC-2024`, `OpenRocket/2024_IREC_30k.CDX1`, MIT. <https://github.com/UND-ARC/IREC-ARC-2024>.
5. **S5.** MDNich, `MIT-RT-26_27-Work`, `support/iteration3_rasaero_comparison/` at `15dcf88d1426`, no license.
   <https://github.com/MDNich/MIT-RT-26_27-Work>.
6. **S6.** GitHub code search for `RASAeroDocument`: 137 files saved, 135 distinct, at most 6 a repository, with each
   repository's license. Repositories holding OpenRocket's source were left out unread.
7. **S7.** OpenRocket PR #2410 (2023-11-22; texts only): <https://github.com/openrocket/openrocket/pull/2410>.
8. **S8.** OpenRocket issues #2137 and #2470. <https://github.com/openrocket/openrocket/issues/2137>.
9. **S9.** OpenRocket pull request #2189 (the exporter) and #3243 (its motor matching, 2026-08-14);
   <https://wiki.openrocket.info/Importing_and_Exporting_to_Other_Applications>.
10. **S10.** HPR Sim at `7f7e2846`: `crates/hpr-design/src/` (`shapes.rs`, `parts.rs`, `fins.rs`, `finish.rs`,
    `tree.rs`), `crates/hpr-aero/src/drag.rs`. No `.CDX1` reader exists yet.
11. **S11.** Loft, the maintainer's sunset MIT project: `lib/rasaero/adapt.ts`, `public/samples/demo-rasaero.CDX1`.
12. **S12.** elliottmckee, pyRATT, `pyRATT/src/obj_rocket.py`, MIT. <https://github.com/elliottmckee/pyRATT>.
13. **S13.** RocketPy documentation, `rocket_usage.rst`, MIT. <https://github.com/RocketPy-Team/RocketPy>.
