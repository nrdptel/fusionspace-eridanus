# How large real joint steps, band steps and proud heads are on HPR airframes

[Excrescence drag](excrescence-drag.md) §4 sized the drag of joint mismatches, switch bands and screw heads on two
invented rockets, with every feature size invented. This note replaces those sizes with makers' published dimensions and
fastener standards. **In short: real parts are smaller than §4 assumed, about half its clean-airframe total.** Tubes of
one nominal size from fiberglass, phenolic and Blue Tube makers share an outer diameter within 0.004–0.006 in, a radial
step of 0.05–0.08 mm; LOC's kraft tubes run 0.14–0.33 mm smaller in radius. A switch band cut from the airframe's own
material stands about 0.1 mm proud, not §4's 0.5 mm. Heads stand 0–2.4 mm proud, not 1–3 mm: no #2–#8 or M3–M4 head
reaches 3 mm, trimmed shear pins and fully sunk flat heads stand at 0, and several makers' rail buttons hide their
screw. Redone with these sizes (Inference), the clean airframe adds about 0.8–1.2% of C_D at 98 mm (0.3–3.9%) and
1.3–2.6% at 54 mm (0.5–7.5%), against §4's 2.0% (0.6–6.0%) and 4.4% (1.4–11.2%): about −0.25% to −0.7% of apogee at 98
mm, 3–8% of the +8.73% apogee bias. How far to trust it: nominal sizes and fastener tables are firm; no hobby maker
publishes an outer-diameter tolerance, so the tolerance is an industrial tube maker's; no builder's measured joint step
was read; humidity can swell paper-based tubes by tenths of a millimeter. "Inference" marks this note's own arithmetic.

## 1. Tube-to-tube steps at a joint

- **One size class, several makers.** HPR tubes are sold by inner diameter; PML says the outer diameter is the inner
  plus twice the wall [T1] p. 1, which gives the PML and Blue Tube rows below (Inference). Inches; LOC states its sizes
  to 0.01 in (54 mm) or 0.1 in (75, 98 mm), so its row is nominal.

| maker and material | 54 mm (2.1 in) | 75 mm (3.0 in) | 98 mm (3.9 in) |
|---|---|---|---|
| LOC kraft [T2]: inner 2.14, 3.0, 3.9 | 2.26 | 3.1 | 4.0 |
| PML phenolic [T3]: inner 2.152, 3.002, 3.900, wall 0.062 | 2.276 | 3.126 | 4.024 |
| Wildman G12 fiberglass [T4]: inner 2.152, 3.000, 3.9 | 2.276 | 3.120 | 4.020 |
| Blue Tube 2.0 [T5]: inner 2.147, 3.000, 3.900, wall 0.062 | 2.271 | 3.124 | 4.024 |

- **Spread (Inference).** Without LOC, the nominal outer diameters differ by 0.005, 0.006 and 0.004 in: radial steps of
  0.06, 0.08 and 0.05 mm. LOC sits 0.011–0.016, 0.020–0.026 and 0.020–0.024 in smaller: 0.14–0.20, 0.25–0.33 and
  0.25–0.30 mm radial. A LOC 3.9 in nose cone [T2], if its base matches LOC's 4.0 in tube, ahead of a 4.024 in phenolic
  or Blue Tube body leaves a 0.30 mm forward-facing step; a LOC body behind another maker's cone faces back. PML warns
  that one maker's 4 in cones and couplers may not fit another's tubes [T1] p. 2.
- **Tolerance.** No hobby maker read states an outer-diameter tolerance [T2–T5]; PML gives only length, −0/+0.25 in [T1]
  p. 1. An industrial filament-wound maker's sheet gives ±0.015 in sanded and ±0.050 in unsanded for 2.000–4.249 in [T6]
  p. 1. Two tubes each within ±t differ in radius by up to t; spread evenly, the mean is t/3 (Inference): sanded, 0.13
  mm mean and 0.38 mm worst; unsanded, 0.42 and 1.27 mm. Whether hobby G12 is sanded to size is not stated. Half of such
  random joints face forward, half back. A buyer's review on Wildman's 3.0 in page reports the tube's sizes off by about
  0.25 mm, without saying which [T4].
- **For §4 (Inference):** one maker's tubes, about 0.1 mm typical (0.05–0.4 mm); a LOC part on another maker's tube,
  0.15–0.3 mm. §4's 0.2 mm (0.1–0.5), all facing forward, spans this, its top 0.1 mm above.

## 2. Switch bands

- **A band shares its bay's material.** LOC sells every bay with a band, phenolic bays with a phenolic band and Quantum
  Tube bays with a Quantum Tube one [T8]. That the band comes from the same tube lot is not stated.
- **Its step (Inference).** A band of the airframe's own material steps by §1's tube-to-tube spread: about 0.1 mm
  typical, 0.05–0.4 mm on sanded-class stock, and it may sit shy rather than proud. A band of another maker's tube adds
  §1's maker spread, about 0.3 mm at 98 mm. No published or measured band-to-tube difference was found.
- **For §4:** 0.1 mm typical (0.05–0.4 mm) before paint, against §4's 0.5 mm (0.1–1.0). §4's typical needs unsanded
  stock, mixed makers, or paint and primer built up on the band; finish thickness is the subject of [hobby finish
  roughness](hobby-finish-roughness.md).

## 3. Humidity and temperature

- **Blue Tube is vulcanized fiber;** its maker's page links a vulcanized-fiber safety data sheet and advises sealing the
  tube, with no swelling figure [T5]. Vulcanized fiber grows 0.10% along and 0.25% across the sheet per 1% change in
  moisture content [T9] p. 2, [T10] p. 1.
- **Paper's moisture** (bleached kraft linerboard) is 6.2% at 30% relative humidity, 9.6–9.7% at 60% and 12.7–13.0% at
  80% [T11] p. 735, Table 1; using it for vulcanized fiber, whose curve was not found, is an assumption.
- **Its step (Inference).** A wound tube's hoop direction mixes both sheet directions, so 0.10–0.25% brackets it. From
  30% to 60% humidity a 98 mm tube's radius grows 0.17–0.43 mm, a 54 mm one's 0.10–0.25 mm; to 80%, 0.33–0.83 mm and
  0.19–0.47 mm. A step forms only where paper meets fiberglass (a fiberglass cone or band on a Blue Tube or kraft body),
  paper proud when humid. How slowly a sealed tube follows the air is not bounded. Temperature is small: a 50 °F swing
  moves the radius 0.02–0.04 mm, from the sheets' thermal expansion of 1.1–1.7 × 10⁻⁵ per °F [T9] p. 2.

## 4. Shear pins, rivets and screw heads

- **Nylon shear pins** keep the steel pan-head shape: a #2-56 head is 3.94–4.24 mm across and 1.14–1.35 mm tall, a #4-40
  one 5.21–5.56 mm and 1.47–1.73 mm [H1], equal to ASME B18.6.3's slotted pan-head limits [H2] Table 1. Pins are sold 1
  in long, to be cut to length [H3]. One kit maker's manuals use nylon screws or 0.060 in (1.5 mm) styrene rod, trimmed
  with end-cutting pliers after fitting [H4] (Vertical Assault p. 20, Talon 4 p. 23): a trimmed pin leaves a stub near 0
  mm. How often builders leave the head on is not known.
- **Push rivets** are low domes. Giant Leap's small rivet head is 8.05 mm across and 1.98 mm tall [H5]; its large one
  reads 11.1 mm and 1.8 mm, and LOC's 8 mm and 1.8 mm [H6], both from unlabeled dimension letters (Inference).
- **Bay, fin-can and nose screws** (Inference from the tables below: h 1.3–2.4 mm, 5–8 mm across for #4–#8 and M3–M4 pan
  or button heads; about 0 for a flat head in a full countersink). Only a #10 pan head (2.79 mm) or larger nears 3 mm. A
  flat head sits flush only in a countersink as deep as the head, 1.70 mm (#4) to 2.54 mm (#8) [H8] Table 1; a 1/16 in
  (1.6 mm) wall is shallower than any of them, so a flat head on a thin wall stands proud by the shortfall.

| head, mm | #4 | #6 | #8 | #10 | M3 | M4 |
|---|---|---|---|---|---|---|
| pan: across | 5.2–5.6 | up to 6.9 | 7.8–8.2 | 9.1–9.5 | 5.3–5.6 | 7.64–8.0 |
| pan: height | 1.47–1.73 | 1.83–2.08 | 2.16–2.44 | 2.51–2.79 | 1.66–1.80 | 2.26–2.40 |
| socket button: across | 5.1–5.4 | 6.4–6.7 | 7.6–7.9 | 8.8–9.2 | 5.40–5.70 | 7.24–7.60 |
| socket button: height | 1.30–1.50 | 1.60–1.85 | 1.96–2.21 | 2.31–2.57 | 1.40–1.65 | 1.95–2.20 |

  Inch pan heads per B18.6.3 [H2] Table 1, inch button heads per B18.3 [H7], metric pan per DIN 85 and button per
  ISO 7380-1 [H9] Table 1. The standards themselves are paid; these are vendor reproductions of their tables.
- **Against §4 (Inference):** §4's "1–3 mm" heads become 0–2.4 mm, typically 1.3–2.0 mm. Its 7 mm dome 2 mm proud is a
  real head: a #6 pan, an M4 button or a #8 button head is within 1 mm of it. §4's count of 6–8 heads is unchecked.

## 5. Rail buttons and their screws

- **Several makers' buttons hide the screw.** Apogee's 1010 button (made by AeroPack) takes a 6-32 flat screw and says
  no round head sticks out [H10]; Fruity Chutes lists countersunk 8-32 screws [H11]; Rail-Buttons.com's mini button
  takes a countersunk head, while its one-piece 1010 and 1515 pages don't state the head type [H3]. Its three-piece kit
  (button, spacer, washers) ships a #8 × 1/2 in screw whose photos show a pan-style head (Inference from photos):
  2.16–2.44 mm proud and 7.8–8.2 mm across [H2]. Some kits glue their rail guides with no screw at all [H4] (Vertical
  Assault p. 18).
- **So [excrescence drag](excrescence-drag.md) §5's 8 mm head 2–2.5 mm proud is the three-piece case only;** on a
  countersunk button it is 0 mm, and §4's rail-button row (0.4%, 0.15–0.7% at 98 mm) is an upper bound.
- **Button size.** Apogee's 1010 button is 9.7 mm (0.381 in) across and 9.7 mm long [H10], against §5's invented 11 mm;
  its 1515 button is 16.0 mm across and 11.2 mm long [H12]. Apogee's FAQ gives the 1010's length as 0.435 in. A 1010
  rail's slot opening is 6.50 mm (0.256 in) [H13] p. 2-16, so a 1010 button's waist is under 6.5 mm, not §5's 7
  (Inference). Standoffs lift a button 1.6 mm [H3] or 3.3 mm [H14].

## 6. Excrescence drag §4 redone with these sizes (Inference)

Same rockets, coefficients and method as §4, at 300 m/s. A step's drag scales as h × (h/δ)^(1/3), so each row moves as
(h_new/h_§4)^(4/3). Mismatches take 0.1 mm typical with half facing back (0.05 mm, all back, to 0.4 mm, all forward);
the band 0.1 mm (0.05–0.4); the head row keeps its typical with its top scaled by (2.44/3)^(9/7) = 0.77 from Hood's rule
at fixed width, and its low end stays §4's (with every pin trimmed and every head flush it falls toward 0); gaps and
ports as §4; rail-button heads 0 for countersunk buttons, §4's values for three-piece ones.

| feature | 98 mm: % of C_D, typical (range) | 54 mm: % of C_D, typical (range) |
|---|---|---|
| 3 tube mismatches | 0.12 (0.04–0.98) | 0.25 (0.07–2.0) |
| switch band | 0.08 (0.03–0.53) | 0.16 (0.07–1.0) |
| heads | 0.5 (0.2–1.5) | 0.8 (0.35–1.8) |
| 2 rail-button screw heads | 0 or 0.4 (0–0.7) | 0 or 1.3 (0–2.3) |
| 3 joint gaps and 4 ports, as §4 | 0.06 (0.01–0.17) | 0.13 (0.03–0.40) |
| **clean airframe, total** | **0.77–1.17 (0.28–3.91)** | **1.34–2.64 (0.52–7.47)** |

- **Against §4,** the typical total falls from 2.0% to 0.8–1.2% (98 mm) and from 4.4% to 1.3–2.6% (54 mm). In apogee, at
  −0.31% to −0.57% per 1% of drag ([apogee bias](apogee-bias.md)), that is −0.24% to −0.67% and −0.42% to −1.5%: 3–8%
  and 5–17% of the +8.73% bias.
- **Outside the sourced range:** unsanded tubes at their 1.27 mm worst would give 4.6% (98 mm) and 9.2% (54 mm) if all
  three joints faced forward; one paper-to-fiberglass joint at 80% humidity adds about 0.9% (98 mm, 0.83 mm) and 0.8%
  (54 mm, 0.47 mm). Paint built up on a band, and an external camera shroud or antenna (§4's 1–47%), are not changed
  here.

## What the project can use

- **Defaults for an optional joints term.** If [excrescence drag](excrescence-drag.md)'s suggested per-joint term is
  built, 0.1 mm per joint and per band is a sourced default; 0.3 mm for a joint across LOC and another maker. Expected
  effect, with §4's gaps: +0.25% of C_D on 98 mm and +0.5% on 54 mm, against §4's 1.15% and 2.3% for those rows.
- **Head heights by type:** trimmed pin 0, pan-head pin 1.1–1.7 mm, rivet 1.8–2.0 mm, pan or button screw 1.3–2.4 mm,
  flat head 0 in a full countersink; rail-button screws 0 unless the button is a three-piece kit.

## What this suggests

- **Lower the excrescence share of the bias** to about 3–8% at 98 mm (from 7–13%), leaving more to other causes.
- **A hobby measurement:** calipers on the outer diameter of a few tubes per maker, and at each joint of a built rocket,
  would replace §1's industrial tolerance with a hobby one.

## Open questions

- **Measured steps on built rockets.** Rocketry Forum threads on cone-to-tube steps (143684, 80690, 4047, 135324)
  returned HTTP 403 on 2026-10-28; none was read.
- **Hobby tolerances:** whether G12 tubes are sanded to size, lot-to-lot outer diameters, and nose-cone base diameters
  (Wildman lists its cones by nominal size only [T7], LOC by the tube they fit [T2]).
- **How many heads** a typical L2 dual-deploy rocket carries, and how often shear pins are trimmed flush.
- **Vulcanized fiber's moisture curve,** how fast a sealed tube follows the air, and button waist and flange sizes.

## Sources

All read 2026-10-28. No source carries an open license: cited only, with copies kept under `refs/`, never committed.

- **T1.** PML, *PML Airframes FAQ*, 2003, pp. 1, 2.
  <https://cdn.shopify.com/s/files/1/0588/8370/2933/files/PMLAirframesFAQ_22d4efad-3f6d-4b5b-835f-24a6f35b4b5d.pdf>
- **T2.** LOC Precision, cardboard airframes. <https://www.locprecision.com/products/cardboard-airframes/>
- **T3.** LOC / Public Missiles, phenolic tubing. <https://locprecision.com/products/phenolic-airframe-tubing>
- **T4.** Wildman Rocketry, G12 tubes. <https://wildmanrocketry.com/products/g12-2-1>,
  <https://wildmanrocketry.com/products/g12-3-0>, <https://wildmanrocketry.com/products/g12-4-0>
- **T5.** Always Ready Rocketry, Blue Tube 2.0 <https://alwaysreadyrocketry.com/blue-tube-2-0/> and its airframes, for
  example <https://alwaysreadyrocketry.com/product/1-15-29mm-x-062-wall-x-48-airframe-mmt/>
- **T6.** Rock West Composites, stock tube tolerance sheet RW-PS-071 Rev F, 2025, p. 1.
  <https://www.rockwestcomposites.com/media/downloads/RW-PS-071-Stock-Tube-Tolerance-Sheet.pdf>
- **T7.** Wildman Rocketry, filament-wound nose cones. <https://wildmanrocketry.com/products/fnc4-0-5-1-vk-fw-mt-copy>
- **T8.** LOC Precision, electronics bays. <https://locprecision.com/product/electronics-bays>
- **T9, T10.** ESP, vulcanized fiber specifications, p. 2,
  <https://electrical-insulation.espemfg.com/Asset/vulcfibreprop.pdf>; Oliner, Vulcanex engineering data, p. 1,
  <https://www.redseal.com/s/vulcanex-datasheet.pdf>
- **T11.** Ramarao, B. V., and Chatterjee, S. G., "Moisture sorption by paper materials under varying humidity
  conditions," FRC 11th Fundamental Research Symposium, 1997, pp. 703–749, Table 1. DOI 10.15376/frc.1997.2.703.
  <https://bioresources.cnr.ncsu.edu/wp-content/uploads/2020/03/1997.2.703.pdf>
- **H1.** Micro Plastics (Essentra), nylon 6/6 slotted pan screw drawings 010256P018 (#2-56) and 010440P025 (#4-40).
  <https://mm.digikey.com/Volume0/opasdata/d220001/medias/docus/951/010256P018.PDF>,
  <https://mm.digikey.com/Volume0/opasdata/d220001/medias/docus/850/010440P025.PDF>
- **H2, H8, H9.** Westfield Fasteners, Table 1 of ASME B18.6.3 slotted pan (H2) and countersunk (H8) heads, UNC, and of
  DIN 85 and ISO 7380-1 (H9), under <https://westfieldfasteners.co.uk/Standards/>: `ScrewBolt-SlotPan-UNCoarse.pdf`,
  `ScrewBolt-SlotCsk-UNCoarse.pdf`, `ScrewBolt-SlotPan-M.pdf`, `ScrewBolt-SHBtn-M.pdf`
- **H3.** Rail-Buttons.com, product pages 50–52, 54, 55, 60–62, 66–68, 71, 96.
  <https://rail-buttons.com/ocart3/index.php?route=product/product&product_id=52> (change the id)
- **H4.** Giant Leap Rocketry, Vertical Assault manual r5 (pp. 18, 20) and Talon 4 manual r3 (p. 23).
  <https://cdn.shopify.com/s/files/1/0518/3809/1436/files/Vertical_Assault_Instruction_Manual_r5.pdf>,
  <https://cdn.shopify.com/s/files/1/0518/3809/1436/files/Talon_4_Instruction_Manual_r3.pdf>
- **H5.** Giant Leap Rocketry, removable rivets. <https://giantleaprocketry.com/products/large-removable-rivets-10-pack>
- **H6.** LOC Precision, nylon rivet. <https://locprecision.com/products/nylon-rivet>
- **H7.** fasten.it, ASME B18.3 Table 3 <https://www.fasten.it/en/norms/norm/asme_b18.3_table_3>; Industrial Threaded
  Products, <https://www.buildsite.com/pdf/industrialthreadedproducts/Socket-Screws-Product-Data-2101152.pdf>
- **H10.** Apogee Components, 13060 1010 rail button.
  <https://apogeerockets.com/building_supplies/rail_buttons/1in_1010_rail_button_standard>
- **H11.** Fruity Chutes, rail buttons. <https://fruitychutes.com/blog/rocketry-rail-buttons-are-back>
- **H12, H14.** Apogee, 13063 1515 button and 13071 standoff, under
  <https://www.apogeerockets.com/Building_Supplies/Launch_Lugs_Rail_Buttons/Rail_Buttons/>: `1-5_Rail_Button_Large`,
  `1_8_Rail_Button_Standoff`
- **H13.** 80/20 Inc., catalog p. 2-16, 1010 profile.
  <https://assets.rs-online.com/v1691207229/Datasheets/4f46794a2dd82157e98593622819e88e.pdf>
