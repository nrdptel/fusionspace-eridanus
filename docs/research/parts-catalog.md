# Parts catalog: commercial parts, and a catalog of hpr's own

**Bottom line:** hpr already carries OpenRocket's parts catalog, all 16 files (3,449 parts and
402 materials), and a rocket can be built from it in Rust or Python
([ADR-132][adr-132], [ADR-133][adr-133]). Today a person can't search it from the command line,
get a list of parts to buy, or pick parts in an app. The catalog is thin where high-power flyers
shop. About half its parts are one maker of vintage model-rocket parts, none of its 151
parachutes states a drag coefficient, and it has no motor hardware, rail buttons, shock cord or
electronics. Vendors publish much of what is missing, such as drag coefficients, packed sizes and
hardware masses. A catalog of hpr's own can carry those values, each with its source and date,
and OpenRocket's stays underneath. Where each piece goes is in [ADR-197, the parts
catalog][adr-197]. Researched October 7, 2026. Vendor figures are as their pages stated them
that day, read once and not yet entered or checked.

## What hpr has

- **The files:** the 16 `.orc` files OpenRocket 24.12 ships, bundled unchanged under Apache-2.0
  in `crates/hpr-io/data/openrocket-database/`. Every part is held to OpenRocket's own reading
  of it, value by value ([`.orc` format page](../format/orc.md)).
- **The builder:** `from_catalog` on noses, tubes, transitions and motor tubes, and `Fitting` for
  couplers, rings, bulkheads, lugs, parachutes and streamers. Each built mass is held to
  OpenRocket's ([the builder](../the-builder.md#parts-from-a-catalog)). Python has the same.
- **The optimizer** already takes a catalog part as a choice
  ([optimization](../optimization.md#choices-a-motor-a-catalog-part)).
- **Missing:** a search command, a parts list for a design, a picker in an app, and any part
  kind OpenRocket's format lacks.

## What the catalog covers

Counted from the bundled files' `<Manufacturer>` fields:

- SEMROC has 1,676 parts and Estes 452. Both are model-rocket makers.
- High-power makers have far fewer: Madcow 240, Public Missiles 139, LOC Precision 113, Giant
  Leap 92, Always Ready (BlueTube) 53 and Top Flight 47 (34 parachutes, 13 streamers). Balsa
  Machining Service has 298 (model-rocket balsa), Rocketarium 161 and Apogee 6.
- **Parachutes:** 151, and none states a drag coefficient, so the flyer types one in.
- **Upstream is dormant.** `openrocket/openrocket-database` has had no commit since 2025-07-26.
  Its one open pull request (#9, a draft from 2025-10-30) adds Klima, a German vendor.

## What is missing, by vendor

- **Airframe:** Wildman Rocketry (filament-wound noses, tubes, couplers, G10 rings). Its pages
  give a cone's weight only as a range per shape. Not yet checked: Binder Design, Mach1,
  Performance Rocketry, Composite Warehouse, Polecat and Proline.
- **Recovery:**
  - Fruity Chutes (Iris Ultra parachutes, drogues, bags, Nomex). Its 72-inch Iris Ultra page
    states a Cd of 2.2, 13.4 oz, a pack of 3.9 by 6.2 in (74.1 in³), 12 gores and 400 lb flat
    nylon lines, but not how many lines or how long.
  - Rocketman (Cd 2.2, rotating-star and elliptical families), with descent rate against weight
    for each family.
  - Sky Angle, Onebadhawk (shock cord), not yet checked.
- **Motor hardware:**
  - AeroTech RMS cases (Apogee lists the RMS-75/2560 at 15.077 in and 956 g).
  - Cesaroni Pro cases and closures (Pro54 6-grain: 22.14 in, 405.1 g).
  - Loki cases.
  - Aero Pack retainers, 38 to 98 mm, with diameter, length and mass.
  - Spacers and floating closures, sold separately.
- **Rail buttons:** Rail-Buttons.com (1010, 1515, mini, micro).
- **Electronics:** Featherweight, Eggtimer, Missile Works and PerfectFlite publish board sizes and
  masses (for example, about 14 g for an RRC3+). This is the same equipment as the field register
  (M9.7, after 1.0; [ADR-194][adr-194], V47).
- **Every product page** has a price, a SKU and a URL. OpenRocket's format carries none of them.

## Three traps

- **A drag coefficient means nothing without its area.**
  - Flat circular canopies run 0.75 to 0.80 on their nominal (flat) area, and conical ones 0.75
    to 0.90. An annular canopy runs 0.85 to 0.95 and opens to a projected diameter 0.94 times its
    nominal one (Knacke, *Parachute Recovery Systems Design Manual*, Table 5-1).
  - Fruity Chutes' calculator calls its figure "Cd (Projected)", taken on the opening diameter.
    On projected area, Knacke's annular canopy is about 0.96 to 1.07, so a stated 2.2 isn't
    explained by the area alone.
  - So a Cd is stored with the area it was taken on. One whose area is unknown can't size a
    parachute, and the two ends of a range are compared on one area, converted with Knacke's
    ratio of projected to nominal diameter for the canopy's own type.
  - Knacke's figures come from full-scale parachutes. At hobby scale, with ripstop's porosity,
    they are unvalidated.
- **Which end is flattering depends on the output.** A descent speed is √(2mg / (ρ·C_D·S)).
  - For the descent rate and the landing energy, a high C_D·S flatters: the descent reads slow,
    and the flyer picks too small a parachute.
  - For drift, and for the opening load (proportional to C_D·S), a low C_D·S flatters: drift
    reads short and the load reads small.
  - So each output takes its own end of the range, ordered by value, not by source. A vendor's
    figure is the high end only when it exceeds the sourced one. Each output's direction gets a
    test and a mutation probe ([ADR-144][adr-144]).
- **A motor's mass already holds its case.** ThrustCurve defines a motor's loaded weight as
  "propellant and case", and hpr's [motor page](../physics/motor.md) says so for a reload. Neither
  `.eng` nor `.rse` says otherwise. A catalog case added to a motor from a file counts the case
  twice: 956 g for the RMS-75/2560 case above. Spacers, adapters and retainers are never in the motor's
  mass.

## Sourcing and licensing (not legal advice)

- **Open data:** the only permissive parts database found is `openrocket-database`. RocketPy (MIT)
  has no parts catalog, RockSim's is proprietary, and community spreadsheets have no clear
  license. ThrustCurve's motor data is already handled per file ([#295][issue-295]).
- **Facts:**
  - In the US, facts such as a size, a mass or a Cd can't be copyrighted (*Feist v. Rural*,
    1991). A vendor's photos, text, drawings and the way it arranges its catalog can be.
  - In the EU and UK, a database right covers taking a substantial part of a database, so
    copying one European vendor's whole catalog is the exposure. Single facts are not.
  - A site's terms of use can forbid scraping by contract.
- **So:**
  - Enter facts only, by hand or from a vendor's own data, each with its URL and date.
  - Copy no photos or text.
  - Remove a vendor's entries when it asks.
  - Don't scrape a site whose terms forbid it.
  - Asking vendors for data, which also yields figures they don't publish, is outreach, so it is
    Neer's call.

## Ideas for later

- Masses that flyers measure, sent in with their scale's resolution, beside the vendor's figure.
- A "vendor confirmed" flag for data a maker sends.
- Prices fetched live through `hpr-net`, cached, dated and shown with their date; never bundled
  as current.
- Regional vendors (Klima, Wizard Rockets).

[adr-132]: ../decisions/0132-m5-5a-openrockets-orc-parts-catalogues-held-to.md
[adr-133]: ../decisions/0133-m5-5b-catalogue-parts-in-the-builder-weighed-as.md
[adr-144]: ../decisions/0144-the-2026-10-03-planning-review-leaner-bookkeeping.md
[adr-194]: ../decisions/0194-the-2026-10-07-field-tools-checklists-equipment-ground-tests.md
[adr-197]: ../decisions/0197-the-2026-10-07-parts-catalog.md
[issue-295]: https://github.com/nrdptel/fusionspace-eridanus/issues/295
