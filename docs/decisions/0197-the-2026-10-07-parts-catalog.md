# ADR-197: Neer's 2026-10-07 parts catalog: commercial parts, and a catalog of hpr's own (2026-10-07)

- **Status:** accepted; amends [ADR-147, a roadmap of open work][adr-147] §6 (its budget)
- **Summary:** OpenRocket's catalog, bundled since M5.5, stays as the base. M5.6 adds a catalog of hpr's own in four increments. M5.6a adds the format, where every value carries its source and date, plus `hpr parts search` and a parts list to buy from. M5.6b adds parachutes and recovery hardware: each Cd is stored with its reference area, and a range's two ends are compared on one area. Descent, drift and opening load each take the end that errs safe for that output. M5.6c adds motor hardware and rail buttons; a catalog case beside a motor file is refused unless the motor's mass is marked as excluding it. M5.6d adds more airframe makers and electronics; a mass range records what it spans, and safety outputs report its worst end. Only facts go in, each cited; no vendor text or photos are copied. M9.1's design editor gets a part picker and the parts list. M5.6a comes after release 0.5, ahead of M8.2. The rest comes before 1.0, with M5.6b ahead of M8.1's parachute sizing. The roadmap's budget rises to 43,000 bytes.

**Context.** Later on October 7, 2026, after [ADR-196, the product guides][adr-196], Neer set the
goal of matching what OpenRocket offers with its materials and commercial parts (body tubes, nose
cones, parachutes, motor hardware), so a flier can design from commercial parts in about five
minutes and then buy them. Taking OpenRocket's catalog is fine where allowed, and in time hpr should
research and build a higher-fidelity, more informative and larger catalog of its own
([VISION](../VISION.md), that date).

hpr has carried OpenRocket's catalog since M5.5 ([ADR-132][adr-132], [ADR-133][adr-133]): its 16
files are bundled unchanged under Apache-2.0, 3,449 parts and 402 materials. Each part is held
to OpenRocket's own reading, and the builder makes a rocket from catalog parts in Rust and
Python. The optimizer takes a catalog part as a choice. What is missing is everything around
that, as surveyed in [the parts-catalog note](../research/parts-catalog.md):

- **No way in:** no search command, no parts list for a design, no picker in an app.
- **Thin where high-power flyers shop:** SEMROC's model-rocket parts are 1,676 of the 3,449.
  None of the 151 parachutes states a drag coefficient.
- **Whole kinds missing:** motor hardware, rail buttons, shock cord and electronics.
- **Upstream dormant:** no commit upstream since July 2025.
- **Vendors publish more:** parachute drag coefficients, packed sizes, and hardware masses and
  lengths.

**Decision.**

1. **OpenRocket's catalog stays the base.** It is bundled as it is, held to OpenRocket's reading,
   and refreshed when upstream changes.
2. **A catalog of hpr's own sits beside it** (M5.6a), in hpr's own files (TOML, with a JSON
   schema). Lookup searches both. Every value carries:
   - its source: a vendor page or datasheet by URL, a paper, or a named measurement;
   - the date it was read;
   - its kind: stated by the vendor, measured, or derived (with the derivation).

   The reader refuses a value with no source or date. A vendor's mass given as a range is kept
   as a range. A product URL is kept. A price is optional, dated, with a stale-after date, and
   shown as "as of".
3. **Facts only.** Sizes, masses, materials and drag coefficients are entered by hand or from a
   vendor's own data, each cited. No vendor text, photos or drawings are copied. A whole
   European vendor catalog is never copied wholesale, because of the EU database right. No site
   is scraped against its terms. A vendor's entries are removed when it asks. Asking vendors for
   data is outreach, so it is the maintainer's call. This is the project's reading, not legal
   advice.
4. **The five-minute design.**
   - `hpr parts search` and `hpr parts show` cover both catalogs, in the CLI and in Python.
   - A parts list for a design names each catalog part by maker, number and quantity, with a
     link where the catalog has one, and lists the rest as custom parts with their sizes. It is
     a list to buy from: no verdicts, no buy buttons.
   - A `.ork` file names a catalog part in a `<preset>` tag, which hpr already keeps
     ([the `.ork` format page](../format/ork.md)), so an imported design's list names those parts.
   - M9.1's design editor gets a part picker over the catalog, and the parts list. This is the
     app workflow Neer described, and he uses it before M9.1 counts as done
     ([ADR-144][adr-144]).
5. **Parachutes (M5.6b).**
   - A Cd is stored with the area it was taken on: Fruity Chutes states its figure on projected
     area, on the opening diameter, while Knacke's Table 5-1 gives nominal area.
   - A Cd whose area is unknown doesn't size a parachute.
   - A range's two ends are compared on one area. The sourced end (Knacke, for the canopy's own
     type) is converted with Knacke's ratio of projected to nominal diameter. A type Knacke
     doesn't list maps to a named nearest type, or is refused. Knacke's figures are full-scale;
     at hobby scale they are unvalidated, and the docs say so.
   - The ends are ordered by value, not by source. A descent speed is √(2mg / (ρ·C_D·S)), so the
     descent rate and landing energy take the low C_D·S end, and drift and opening load take the
     high one. Each output's direction gets a test and a mutation probe ([ADR-144][adr-144]).
   - M8.1's parachute sizing reads the descent rate's end, so M5.6b comes before it, and M8.1's
     entry says so.
6. **Motor hardware and rail buttons (M5.6c).**
   - Cases, closures, spacers, adapters, retainers and buttons enter the rocket's mass and CG at
     their stations.
   - A motor's loaded mass includes its case and closures: ThrustCurve defines it as "propellant
     and case", and neither `.eng` nor `.rse` says otherwise. So a catalog case or closure beside
     a motor from a file is refused by name, unless the user marks that motor's mass as
     excluding its hardware. Then it flies, with a test for each path.
   - Subtracting is rejected: the file gives no breakdown, so a catalog case subtracted from it
     would give a wrong inert mass and CG. Spacers, adapters and retainers are never in a
     motor's mass and need no check.
7. **More makers and electronics (M5.6d).**
   - Wildman and the other airframe makers the note names.
   - A mass range records what it spans: one unit's tolerance, or a spread across a product
     family. A family's spread is not flown. The part's mass comes from its own geometry and
     material instead.
   - A unit's tolerance is drawn uniformly in Monte Carlo, the maximum-entropy choice when only
     bounds are known (Jaynes, 1957). The CG and inertia move with the drawn mass, and repeated
     parts are drawn together.
   - Outside Monte Carlo, the stability margin and the descent rate report the range's worst
     end, with a test pinning the direction.
   - Altimeters, trackers and batteries enter as masses. M9.7's equipment register (after 1.0,
     [ADR-194][adr-194]) will read these records, not copy them.
8. **Placement.** No release before 0.5 is served by this, so none is held. The queue:
   - M5.6a after M0.7c, ahead of M8.2 and M9.1, so the editor has a catalog to pick from;
   - M5.6b after M9.1, ahead of M8.1;
   - M5.6c and M5.6d after M8.1, before M3.4 and release 1.0.
9. **The roadmap's budget rises** from 40,500 to 43,000 bytes, for M5.6's entry.

**Alternatives considered.**

- **Extend OpenRocket's `.orc` files and send the additions upstream.** Rejected as the main
  path, because the format has no field for a source, a date, a range or a Cd's area. It is kept
  as an option: parts that fit the format could be offered upstream later.
- **Scrape vendors' sites in bulk.** Rejected: terms of use, the EU database right, and unchecked
  numbers. A catalog value is a claim, like any number hpr prints.
- **The catalog in release 0.4 or 0.5.** Rejected: neither release's workflow needs it, and the
  picker belongs to the 1.0 editor. Release dates aren't the reason; what each release serves is.
- **Prices bundled as current.** Rejected: they go stale within weeks. They are dated, or fetched
  live through `hpr-net` later (an idea in the backlog).

**Consequences.** The library and the Python package keep their catalog as it is. M5.6 is four
increments, in [the roadmap](../ROADMAP.md). The ideas backlog gains flyer-measured masses, a
"vendor confirmed" flag, live prices and regional vendors. Open for the maintainer:
whether to ask vendors for their data.

[adr-132]: 0132-m5-5a-openrockets-orc-parts-catalogues-held-to.md
[adr-133]: 0133-m5-5b-catalogue-parts-in-the-builder-weighed-as.md
[adr-144]: 0144-the-2026-10-03-planning-review-leaner-bookkeeping.md
[adr-147]: 0147-m0-5b-a-roadmap-of-open-work-an-archive-and-a.md
[adr-194]: 0194-the-2026-10-07-field-tools-checklists-equipment-ground-tests.md
[adr-196]: 0196-the-2026-10-07-product-guides.md
