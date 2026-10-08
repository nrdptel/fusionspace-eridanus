# Custom parachutes: research for the recovery designer

**Bottom line:** a hobbyist's canopy can be designed from public sources alone. The gore geometry
is closed-form, the drag and opening-load methods are in public US government reports, and
vendors publish enough descent data to check a range against. This note backs the recovery
designer's milestones ([M12.1 and M12.2](../ROADMAP.md), [ADR-162, the suite and its releases][adr-162]). It extends the
recovery ideas already in [Pad, range and recovery hardware](ideas-pad-and-recovery.md): gore
patterns, parachute sizing, a vendor Cd library and deployment shock loads. Researched 2026-10-04;
nothing here is implemented yet.

## What hobbyists sew

- Hemispherical and semi-ellipsoidal canopies with 8 to 18 gores and a spill hole of about 20% of
  the diameter (about 4% of the flat circle's area; 2% of a hemisphere's cloth). Fruity Chutes' tutorial uses 1.1 oz ripstop, flat-felled
  seams and lines 115% of the diameter
  ([tutorial](https://fruitychutes.com/help_for_parachutes/drone-parachute-tutorials/how_to_make_a_parachute)).
- Nakka's 12-gore semi-ellipsoid: height over radius 0.707, a 2 cm seam allowance, lines
  2.25 (D + S) long ([Nakka](https://www.nakka-rocketry.net/paracon.html)).
- Annular ([Apogee newsletter 497](https://www.apogeerockets.com/education/downloads/Newsletter497.pdf)),
  cruciform (X-form) and flat parasheets.

## The geometry

For a canopy whose meridian has radius r(s) at arc length s, a gore's width at s is the chord
between neighbouring meridians, 2 r(s) sin(π/N), for N gores, plus a correction for the cloth's
bulge between seams ([Laboratori d'envol](https://www.laboratoridenvol.com/info/parachute/parachute-gore.en.html);
a hemisphere worked by [Thyssen](https://www.kiteplans.org/planos/hemisphere/hemisphere.html)).
Two constructions differ. A gore of chord width laid along the meridian, so its seam equals
the meridian arc, sews a faceted surface whose area falls short of the surface of revolution by
1 − sin(π/N)/(π/N): 2.55% at 8 gores, 0.51% at 18. The MIT generator below lays the arc width
2πr/N along the arc instead; its area matches the surface of revolution, but its seam runs 3.75%
longer than the meridian on an 8-gore hemisphere. Both lengths depend on the coordinate the gore
is laid out along (the centerline or the edge), so a tool names that coordinate too. A tool names its construction and checks its
area against the surface it actually builds. Drozd relates the constructed shape to the inflated one: gore count, bulge and the ratio of
projected to nominal diameter
([AIAA 2009-2944](https://airborne-sys.com/wp-content/uploads/2016/10/aiaa-2009-2944_axisymmetric_parachute_sh.pdf)).

## Drag and opening loads (public reports)

- Knacke, *Parachute Recovery Systems Design Manual*, NWC TP 6575 (1991),
  [DTIC ADA247666](https://apps.dtic.mil/sti/pdfs/ADA247666.pdf). Cite the government report, not
  the 1992 commercial edition. Its distribution statement is still to be read from the PDF
  (DTIC refused a scripted fetch).
- Ewing, Bixby and Knacke, *Recovery Systems Design Guide*, AFFDL-TR-78-151 (1978),
  [DTIC ADA070251](https://apps.dtic.mil/sti/tr/pdf/ADA070251.pdf).
- Knacke's opening force: F = q (C_D S) C_x X_1, with q the dynamic pressure at line stretch,
  C_x the opening-load factor (about 1.6 to 1.8 for solid canopies) and X_1 the force-reduction
  factor, from 1 for an infinite mass down to about 0.02 for a lightly loaded canopy
  ([a CSU thesis that applies it](https://scholarworks.calstate.edu/downloads/c821gr23j)).
- Pflanz: drag area grows as (t / t_f)^η, with η = 2 for solid canopies, 1 for ribbon and ½ for
  reefed or extended-skirt ones; it needs the fill time
  ([USU MAE 6530 §3.5](http://mae-nas.eng.usu.edu/MAE_6530_Web/New_Course/launch_design/Section3.5.pdf)).
  Ludtke: [NSWC TR 86-142](https://apps.dtic.mil/sti/tr/pdf/ADA170962.pdf).
- A flat circular canopy's C_D0 is 0.75 to 0.80 (Knacke, through secondary sources). The other
  types' ranges must be read from Knacke's tables before any code. Porosity: the NASA Ames
  wind-tunnel series ([NTRS 19670015726](https://ntrs.nasa.gov/api/citations/19670015726/downloads/19670015726.pdf)).

## Existing tools

| tool | what | license | use here |
|---|---|---|---|
| [Parachute-DXF-Generator](https://github.com/martinscaune/Parachute-DXF-Generator) | hemispherical gores to DXF | MIT | portable with attribution; an oracle for outlines |
| [openchute](https://github.com/ElvinC/openchute) | many canopy types, PDF/DXF | none stated | don't read; may be run as an oracle |
| [Scott Bryce's generator](https://scottbryce.com/parachute/spherical_parachute.html) | spherical gores | none stated | run only |
| [Nakka's `parapat`](https://www.nakka-rocketry.net/soft/parapat_v1.1.xls) | spreadsheet | none stated | run only |
| [Fruity Chutes calculator](https://fruitychutes.com/help_for_parachutes/parachute-descent-rate-calculator) | descent of about 100 canopies | proprietary | published numbers only |

## Validation data

- Fruity Chutes' Iris Ultra log: about 16 flights measured by altimeter, C_D 2.08 to 3.22, the
  vendor's 2.2 near the worst case
  ([log](https://fruitychutes.com/help_for_parachutes/iris-parachute-cd-performance-log)).
- Spherachutes rate C_D 0.75 on a size that is the gore length, not the diameter (a 48 in chute
  is 30.56 in across) ([chart](https://spherachutes.com/pages/decent-rate-chart)).
- Rocketman claims 0.97; its own tables imply about 0.84
  ([forum thread](https://www.rocketryforum.com/threads/rocketman-parachutes-posted-decent-rates-not-matching-reality.179543/)).
- Valkyrie's master list of vendor C_D with back-computed corrections
  ([PDF](https://www.valkyrierecoverysystems.com/uploads/1/1/9/5/119545746/hobby_parachute_cd_master_list.pdf));
  Flanagan's measurements, 0.53 to 1.04 on model-rocket chutes, varying with loading and rigging
  ([Apogee 668](https://www.apogeerockets.com/education/downloads/Newsletter668.pdf)).
- **The trap:** vendors quote C_D on nominal, projected or gore-length areas. Pin the reference
  area before comparing a coefficient.

## Which way each number must err

- Descent rate: from the low end of the C_D range; too slow a descent is flattering.
- Drift: from the high end; the designer carries both bounds.
- Opening load: high C_D S, C_x and X_1 at the worst deployment speed; too low a load is
  flattering. Knacke's opening force leaves out the snatch force at line stretch, which can
  dominate on a lightly loaded hobby canopy.
- Line and seam margin: load over derated strength (seam efficiency 0.75 to 0.90,
  [NASA 20150003409](https://ntrs.nasa.gov/citations/20150003409)); mass and packed volume err high.
- Every load estimate says "drop test first", as charge sizing says "ground test first".

## Outputs worth making

A 1:1 gore template (SVG, PDF and DXF) with a printed 100 mm scale bar, tiled across paper sizes;
spill-hole and seam outlines; a bill of materials (fabric with nesting waste, lines, tape,
thread); canopy mass and packed volume; descent rate as a range with its reference area; opening
load as a range against line strength times the design factor.

[adr-162]: ../decisions/0162-the-suite-and-its-releases.md
