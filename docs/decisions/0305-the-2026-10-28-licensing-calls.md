# ADR-305: Neer's 2026-10-28 licensing calls: vendors' published data as facts, and which holds stay (2026-10-28)

- **Status:** accepted; amends [ADR-197][adr-197] (its open question on asking vendors) and [ADR-287][adr-287] (the 2016 sample dump as a source of the motor finder's test records); no done-when changes
- **Summary:** Dimensions and masses a parts vendor publishes are facts, used with credit, as ADR-287 reads ThrustCurve.org's records; no photo, drawing or descriptive text is copied and no vendor is contacted, so M5.6b no longer waits on a question to vendors. The motor finder's real test records come from the 2026-10-28 snapshot `hpr-motor` ships, and the held catalog file is cleared. CATS's log layout, RASAero II's design files and the guide's real-flight trace each get a route for when their work comes up. Featherweight's Bluetooth packets, makers' data sheets, Wyoming soundings and the refused references stay as they are. The orhelper migration idea is dropped.

[adr-120]: 0120-university-of-wyoming-soundings.md
[adr-194]: 0194-the-2026-10-07-field-tools-checklists-equipment-ground-tests.md
[adr-197]: 0197-the-2026-10-07-parts-catalog.md
[adr-236]: 0236-the-2026-10-11-review-supersonic-flights-from-published-comparisons.md
[adr-247]: 0247-the-simulators-guide-in-four-increments.md
[adr-274]: 0274-the-rest-of-the-importers-planned-teensy-logger-first.md
[adr-287]: 0287-thrustcurve-orgs-motor-records-used-as-published-facts.md
[m5-6b]: ../decisions-and-roadmap.md#m5-6b
[parts]: ../research/parts-catalog.md
[ideas]: ../research/ideas.md
[i715]: https://github.com/nrdptel/fusionspace-eridanus/issues/715
[i764]: https://github.com/nrdptel/fusionspace-eridanus/issues/764
[i885]: https://github.com/nrdptel/fusionspace-eridanus/issues/885
[i1006]: https://github.com/nrdptel/fusionspace-eridanus/issues/1006
[i1016]: https://github.com/nrdptel/fusionspace-eridanus/issues/1016

**Context.** Several pieces of work were held back over licensing, each waiting on a call only the
maintainer can make (CLAUDE.md's rule on legal and licensing calls). On 2026-10-28 Neer went
through the list of holds and the project's recommendation for each, and approved all of them
([#1016][i1016]). This record keeps those calls in one place, so the work they free can start and
the holds that stay keep their reasons.

Two of the holds had the same shape as ThrustCurve.org's records before [ADR-287][adr-287]: data a
company publishes about products anyone can buy, with no license granted for it.

- **Parts vendors.** [ADR-197][adr-197] builds a parts catalog from vendors' published figures
  (parachute drag coefficients and sizes, hardware masses and lengths), facts only and each cited.
  It left open whether to ask vendors for their data, and the ideas backlog held "asking parts
  vendors" as Neer's call. [M5.6b][m5-6b], the parachutes and recovery hardware, was planned
  behind that question.
- **The motor finder's test records.** [#885][i885] asked for real ThrustCurve.org records in the
  motor finder's tests. ADR-287 named the site's 2016 sample dump as one source, and whether that
  dump could be committed was still a question. Since then `hpr-motor` ships a snapshot of the whole
  catalog, taken on 2026-10-28.

**Decision.**

1. **Vendors' published dimensions and masses are facts,** used with credit, by ADR-287's reasoning:
   a number a maker publishes about a product it sells is a fact about that product, not an
   expression of it. Each value keeps ADR-197's citation (the page or datasheet, the date read,
   and whether it was stated, measured or derived). No photos, drawings or descriptive text are
   copied, ADR-197's rules on scraping and the EU database right still hold, and a vendor's entries
   are still removed when it asks. No vendor is contacted, so [M5.6b][m5-6b] no longer waits on
   asking them.
2. **The motor finder's real test records come from the 2026-10-28 snapshot** compiled into
   `hpr-motor` ([#885][i885]). The question of committing the 2016 sample dump is dropped, because
   the tests no longer need it.
3. **The held catalog file is cleared** ([#1006][i1006]). It was held only because the project's
   private-data guard matched its bytes; they are ThrustCurve.org's own published answer.
4. **Routes, each for when its work comes up:**
   - **CATS's `.cfl` logs** ([ADR-274][adr-274] §5): low priority. If a user asks for them, the
     layout is inferred from cached sample files' bytes, as for the Teensy logger, never from
     CATS's GPL source, and only invented test files are committed.
   - **RASAero II's design files** ([ADR-236][adr-236]): when a validation case needs one, its
     geometry is re-entered from the stated dimensions as the project's own file.
   - **The guide's real-flight trace** ([ADR-247][adr-247]): drawn from an MIT-licensed flight,
     RIT's public designs and logs ([#715][i715]).
5. **Kept as they are:**
   - Featherweight's Bluetooth packets stay out; the vendor asks users not to use them
     ([ADR-194][adr-194] §6).
   - Motor makers' data sheets: their numbers are used as facts; the PDFs stay out of the
     repository.
   - Wyoming soundings: only U.S. stations' are committed fixtures ([ADR-120][adr-120] §8); every
     station stays available live.
   - The refused or limited references stay so: [#764][i764]'s, Knacke's, the streamer papers,
     OpenRocket's motor database, AerospaceNU's GPL-3.0 data, DRDC's tables (statistics only) and
     the STEP plans.
6. **The orhelper migration layer is dropped** from [the ideas backlog][ideas]: the `.ork`
   importer already serves the users it was for, and the layer would bind the project to a GPL
   tool's interface.

**Alternatives considered.**

- **Ask each vendor before using its figures.** Rejected: published figures need no permission as
  facts, the request is outreach the maintainer has declined, and it would hold M5.6b on answers
  that may never come. A vendor that objects still has its entries removed.
- **Keep the 2016 dump as the test source.** Rejected: the shipped snapshot is newer, already in
  the package, and its records are checked against their files.
- **Port CATS's reader from its source.** Rejected: GPL code (CLAUDE.md's clean-room rule).

**Consequences.** ADR-197 points here for its open question. M5.6's roadmap entry states that
vendors' published figures are used with credit and that no vendor is asked. [The parts-catalog
note][parts] and the ideas backlog record the call; the orhelper idea is marked dropped. #885,
#715 and the routes above are carried out by their own work, not by this record.
