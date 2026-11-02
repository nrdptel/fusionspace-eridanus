# Can the Violent Agreement flights be rebuilt from what is published

**In short: yes for the geometry, no for the flight.** Every geometry input of both rockets is published as text, on
two input-screen images that rasaero.com holds but its pages don't link. A rebuild from them leaves under 0.1% of
apogee to rounding. What limits a rebuild is everything else: the mass split, the launch site's height and air, the
launch angle and the motor as flown, none of them published. Worse, the J530 flight's published numbers don't close.
With the stated 3.9 lb lift-off weight and the certified J530, the motor can add at most 742 m/s even with no drag,
but the claimed Mach 2.44 needs 802–830 m/s. So that flight's supersonic drag curve rests on a speed or a mass that
is wrong. The I216 flight's numbers are consistent, and it would be the fifth flight past Mach 1 for
[M2.3c7, faster than sound](../decisions-and-roadmap.md#m2-3c7). But its design doesn't ship with RASAero II, which
M2.3c7's rule requires, so counting it needs a decision.

It builds on [rasaero-comparison-flights](rasaero-comparison-flights.md), [flight-derived-cd](flight-derived-cd.md)
§2 and [supersonic-coast-cd](supersonic-coast-cd.md). Read 2026-11-02. Tags: **measured** (off an image, in code),
**Inference** (this note's reasoning or arithmetic).

## 1. The flights

Both rockets were built and flown by Adrian Adamson, founder of Featherweight Altimeters, who supplied the data
[S1, S2, W1]. He is not one of RASAero II's authors (Charles E. Rogers and David Cooper [S3]). Both are scratch-built,
38 mm minimum-diameter carbon airframes. The two flights used different aft airframes with the same nose cone and
avionics bay [F4].

| | Violent Agreement (I216) | Violent Agreement SS sustainer (J530) |
|---|---|---|
| Flown | Mile High Mayhem 2010 [F9, F12], on 2010-05-28 [F4] | same weekend, two days later [F4, F12] |
| Staging | single stage | the top stage of his 3-stage stack, flown alone [F5, P1] |
| Builder's raw apogee | 17,408 ft [F3, F4] | 22,328 ft [F3, F4] (22,530 in a 2011 list [F12]) |
| Page's measured apogee | 18,495 ft, baro adjusted to a balloon's pressure data [S1] | 22,874 ft, same method [S2] |
| RASAero II prediction | 18,784 ft, +1.56% [S1] | 24,784 ft, +8.35% [S2] |
| Comparison table's value | 19,256 ft, +4.11% [S4] | 25,602 ft, +11.93% [S4] |
| Peak Mach | 1.49, where the measured C_D trace ends (**measured**) | 2.44 (**measured**); "about Mach 2.5" [F6, P1] |
| Boost, builder's words | straight, about 30 g of acceleration [F4] | about 60 g, "significant wobble early on" [F4] |

- **How the numbers were made.** Speed comes from integrating the Raven's axial accelerometer, and C_D from the
  coast deceleration and the burnout mass [S1, S2, F1]. The pages don't say how Mach was computed. The J530 page
  blames its over-prediction on angle-of-attack oscillations early in the flight [S2].
- **The balloon correction** adds 1,087 ft (+6.2%) to one flight and 546 ft (+2.4%) to the other, two days apart
  (**Inference**, the table's arithmetic). Neither source says why they differ or what ground the raw values use.
- **The date of the model.** The input screens' file names read `I216Apr42011` and `VASustainerApr42011`, so the
  page's model was most likely made about ten months after the flights (**Inference**).
- **Site, wind, launcher and angle:** not stated. His 2024 log from the same club's field starts near 5,470 ft [D1].
- **No license.** Neither the pages nor the forum posts state reuse terms. Only facts are cited here.

## 2. What the pages publish

Each per-flight page links three images: RASAero II's main screen with a scale drawing on an inch grid, its C_D
against Mach, and a chart of measured against predicted C_D [S1, S2]. The server holds three more per rocket that no
page links (`_1.jpg`, `_2.jpg`, `_3.jpg`). The `_2.jpg` image is the older RASAero (I) input screen and shows every
geometry input as text, to 0.0001 in [S5, S6]:

| Input (in, unless stated) | I216 rocket | J530 sustainer |
|---|---|---|
| Nose | Von Kármán ogive, 9.5 long, tip radius 0 | same |
| Body diameter, length | 1.6, 17 | 1.6, 23 |
| Boattail length, base diameter | 0.4, 1.4 | 0.4, 1.4 |
| Fins | 3; root 3.7, tip 0.8, sweep 2.7, span 1.53 | 3; root 5.8, tip 0.8, sweep 4.3, span 1.7 |
| Fin leading edge from nose tip | 22.8 | 23.5 |
| Fin section | biconvex, 0.07 thick, leading-edge radius 0.02 | same |
| Lugs, rail guides, fin can | none | none |
| Surface | Smooth (zero roughness) | same |
| CG from nose tip | 16.40 | 17.90 |
| Lift-off weight (RASAero II screen) | 2.83 lb, CTI I216 | 3.9 lb, CTI J530 IMAX |
| CP at Mach 0 (RASAero II screen) | 20.68 | 22.41 |

- **The linked drawings agree with that text.** Calibrating inches per pixel from every grid line (least squares,
  residual under 0.75 px) puts lengths, diameters, CG and CP within 0.03 in of the text, and fin tips within
  0.09 in (**measured**). They can't show the fin section or thickness. A stated 2.011 in² reference area fixes the
  diameter to ±0.0002 in.
- **The red "RASAero Prediction" line** on the measured-versus-predicted chart follows the older RASAero (I) curve
  through and past Mach 1, not the RASAero II curve shown beside it: their transonic peaks agree to 0.01
  (**measured**). RASAero II's peak sits about 0.04 higher, and its C_D at Mach 1.5 is 6–8% higher.
- **What is not published:** burnout mass and CG, inertia, the site's height and the balloon data, launch angle and
  rail, wind, and the motor lot. The model's "Smooth" is the program's default [S7, p. 53]. RASAero II's screen
  lists no booster, so the page's model is single-stage, matching the builder's account [S7, pp. 41–43].

## 3. The J530 flight doesn't close

RASAero II's lift-off weight is the loaded weight with the motor [S7, pp. 100, 123]. ThrustCurve's certified data
[S8] then give:

| | I216 rocket | J530 sustainer |
|---|---|---|
| Total impulse, propellant, listed peak thrust | 636 N s, 312.5 g, 380 N | 1,115 N s, 576.5 g, 829 N |
| Lift-off, burnout mass | 1.284 kg, 0.971 kg | 1.769 kg, 1.193 kg |
| Drag-free vertical speed gain | 537 m/s | 742 m/s |
| Speed the measured Mach needs (sound at 0–3 km, standard air) | 490–507 m/s | 802–830 m/s |
| Peak thrust over lift-off mass | 30 g (builder: about 30) | 48 g (builder: about 60) |
| Point-mass simulation, peak Mach, site 0–1,667 m | 1.37–1.43 | 1.93–2.01 |
| Point-mass simulation, apogee at 1,667 m | 17,130–18,200 ft | 23,400–24,820 ft |

The speed gain is the rocket equation with the motor's effective exhaust speed (impulse over propellant mass), less
gravity over the burn. The simulation is vertical, with the certified curve and either the flight's C_D fit or
RASAero II's; 1,667 m is about the 2024 log's field height (**Inference**, this note's scripts).

- **The J530's Mach 2.44 is out of reach** of the published mass and motor by at least 60 m/s, even with no drag.
  Reaching it needs a lift-off mass of at most 1.62–1.66 kg (3.6–3.7 lb) with no drag, and about 1.4–1.5 kg with
  drag. The builder's 60 g points the same way: at the certified peak thrust it means about 1.4 kg (**Inference**).
- **The I216 numbers agree.** Its 30 g fits the published mass; its Mach 1.49 sits 4–7% above the simulation, inside
  the drag-free bound (**Inference**).
- **Staging doesn't rescue the J530.** A stage boosted to Mach 2.44 would climb to about 30,000–34,000 ft if
  vertical. Matching 22,874 ft would need a 30–35° tilt at staging (**Inference**, model only). The builder's
  accounts imply the flight was not staged [F5, P1].
- **So one J530 input is wrong** (**Inference**): the published weight, the motor as flown, or the accelerometer.
  The 60 g and the Mach come from the same accelerometer, so they can't separate a lighter rocket from a reading
  that runs high. The same avionics bay read 30 g two days before, consistent with the I216's mass. The baro apogee
  can't decide either: in the simulation, 1.41 kg instead of 1.769 kg raises it only about 4%.
- **The drag curve rests on the answer.** If the speed runs high, then dynamic pressure, which goes as speed squared,
  does too, so each C_D comes out low and sits at too high a Mach. If the mass is lower than the one used for C_D,
  the C_D comes out high (**Inference**). Without the raw log, which isn't posted (§5), neither can be checked.

## 4. How far a rebuild would move a predicted apogee

From the same point-mass simulation, with either C_D; the two agree to ±0.05 points (**Inference**):

| Change | I216 rocket | J530 sustainer |
|---|---|---|
| C_D +1% | −0.54% | −0.68% |
| Diameter +0.01 in (reference area +1.25%) | −0.68% | −0.85% |
| Non-propellant mass +1% | −0.2 to −0.3% | −0.1 to −0.2% |
| Total impulse +1% | +0.8 to +0.9% | +0.9 to +1.0% |
| Site +1,000 ft | +1.6% | +2.2% |
| Fin thickness 0.07 → 0.06 in | +0.4 to +0.8% | +0.6 to +0.8% |

The fin row uses linear supersonic theory for a biconvex section [S9]. Its range comes from where the theory's
1/√(M² − 1) is cut off near Mach 1 (Mach 1.05 to 1.2).

- **Geometry from the input screens:** under 0.1% of apogee from rounding; "3.9 lb", if rounded, adds ±0.27%.
- **Two geometry conflicts.** The model's fins are 0.07 in thick. The builder wrote "about 0.050 in" for the I216
  rocket's [F4], and 0.045 in (2011) and "about 0.060 in" (2019, after a re-layup) for the sustainer's [F10, F13].
  Thinning 0.07 to 0.05 in would add 0.8–1.6% (I216) or 1.1–1.5% (J530) to apogee. And the builder calls the nose
  "a variation of the Von Karman shape" [F4], which the model's plain Von Kármán only approximates.
- **The unpublished inputs dominate.** The site's height alone is worth 1.6–2.2% per 1,000 ft. The motor as flown
  is unbounded: the builder found an earlier CTI load well below its rated impulse [F1]. So are the launch angle
  and the J530's early wobble.
- **A point-mass rebuild doesn't reproduce the page's prediction.** With RASAero II's own C_D at 1,667 m, it gives
  17,130 ft (I216) and 23,400 ft (J530), 9% and 6% below the page's 18,784 and 24,784 ft (**Inference**).

Rebuilt to the published values, the I216 flight carries about ±2% of apogee from fin thickness and rounding, before
the motor, site and air, which need another source such as reanalysis weather for 2010-05-28 (**Inference**).

## 5. What else is published

- **No design file and no log** for either 2010 flight is posted on the builder's site or The Rocketry Forum. The
  data threads were on Rocketry Planet (6636, 6650), now gone and not found in the Wayback Machine.
- **Same flyer, files posted** (none on RASAero's comparison page, so none counts toward M2.3c7):
  - A Loki J1026 single-stage flight, 2024-06-02, with Blue Raven CSV logs [F16]. Peak upward speed in the file is
    2,930 ft/s, about Mach 2.5 at the log's ground temperature, and the baro apogee is 23,830 ft above the ground
    (**Inference**, read from the file). No design file was found.
  - A RASAero `.CDX1` with flight CSVs for a 2023 G/F record rocket [F15], and `.CDX1` files for a two-stage
    rocket's 2025 J and K flights. Their peak Machs were not checked.
  - The 2023 J1026 flight is already in [supersonic-coast-cd](supersonic-coast-cd.md).

## 6. What this suggests

1. **For M2.3c7's count, the I216 flight is a candidate, not a given** (**Inference**). Its geometry is fully
   published, its numbers agree, and it passes Mach 1, so it would be the fifth past Mach 1. But its design is
   transcribed from an image, not shipped with RASAero II, and the page's model was most likely built after the
   flight. Counting it needs M2.3c7's ADR to admit page-published inputs, with the flight flagged as a reconstruction.
2. **Hold the J530 flight out as a Mach 2.4 reference** until its mass or speed is reconciled (**Inference**). Its
   Mach, its drag curve's Mach axis and its supersonic C_D all rest on a speed the published mass and motor can't
   give. The earlier notes' "measured Mach of at least 2.4" for it is the page's claim, not a checked value.
3. **The posted J1026 logs** are a measured supersonic drag source with raw data, but have no design.

## Open questions

- Which J530 input is wrong: the weight, the motor as flown or the accelerometer? The raw Raven log would settle it.
- Why do the balloon corrections differ (+6.2%, +2.4%)? Why does a point-mass rebuild fall 6–9% below the page's?
- The sustainer's fin thickness in May 2010 (0.045, 0.06 or 0.07 in), and the launcher, angle and pad height.

## Sources

- **S1, S2.** RASAero II comparisons with flight data, Violent Agreement and Violent Agreement SS, and their linked
  `img/fc-ViolentAgreement*_{1.PNG,3.PNG,4.jpg}`. <https://www.rasaero.com/comp-flight-ViolentAgreement.htm>,
  <https://www.rasaero.com/comp-flight-ViolentAgreementSS.htm>
- **S3.** RASAero home page, "By Charles E. Rogers and David Cooper". <https://www.rasaero.com/>
- **S4.** RASAero II's apogee comparison table, both rows. <https://www.rasaero.com/comparisons-alt.htm>
- **S5, S6.** Unlinked images `fc-ViolentAgreement_2.jpg` and `fc-ViolentAgreementSS_2.jpg` (also `_1.jpg`, `_3.jpg`)
  under <https://www.rasaero.com/img/>.
- **S7.** RASAero II Users Manual (2019), pp. 41–43, 53, 100, 123.
  <https://www.rasaero.com/dloads/RASAero%20II%20Users%20Manual.pdf>
- **S8.** ThrustCurve, CTI 636I216-14A and 1115J530-15A, certification curves. <https://www.thrustcurve.org/>
- **S9.** Ackeret's linear theory for a biconvex section, c_d = (16/3)(t/c)²/√(M² − 1), as in Anderson, *Modern
  Compressible Flow*, ch. 4.
- **W1.** Featherweight Altimeters, About us. <https://www.featherweightaltimeters.com/about-us.html>
- **F1–F16.** The Rocketry Forum, under <https://www.rocketryforum.com/threads/>, by thread slug:
  - **F1.** `i600-flight-to-15965-feet.3499`, post 30248 (2009-05-31).
  - **F3.** `how-high-have-you-gone.5342`, post 105777 (2010-06-05).
  - **F4.** `june-best-rocket-competition-mpr-hpr.12220`, post 106342 (2010-06-07).
  - **F5.** `airfest-16-around-the-corner.13878`, post 121686 (2010-08-05).
  - **F6.** `hypersonic-and-fiberglass-how.16430`, post 145696 (2010-11-15).
  - **F9.** `giant-leap-rocketrys-new-mariah-38-kit-review.17971`, post 163178 (2011-01-20).
  - **F10.** `violent-agreement.25834` (2011-08 to 09).
  - **F12.** `all-time-altitude-list-is-back.28300`, post 252476 (2011-10-31).
  - **F13.** `violent-agreement-two-stage-38mm-to-33-290-feet.154768` (2019-09).
  - **F15.** `taking-a-shot-at-the-g-record.176171`, post 2503576 (2023-10-25).
  - **F16.** `j-record-attempt-using-loki-j1026.181367` and `9-days-of-38mm-mach-busting.186649` (2024-06);
    D1 is his 2024-06-02 GPS and baro log posted there.
- **P1.** "Violent Agreement 2: Adrian Adamson's BALLS 2010 project", Rocketry Planet (2011-01-13), as re-posted on
  The Rocketry Forum. <https://www.rocketryforum.com/attachments/violent-agreement-2-balls-2010-project-pdf.393039/>
