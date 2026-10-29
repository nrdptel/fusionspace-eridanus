# ADR-305: Neer's 2026-10-28 licensing calls: vendors' published sizes and masses as facts, and which holds stay (2026-10-28)

- **Status:** accepted; amends [ADR-197][adr-197] (its open question on asking vendors) and [ADR-287][adr-287] (the motor finder's test records come from the shipped snapshot, not the 2016 sample dump); no done-when changes
- **Summary:** Dimensions and masses a parts vendor publishes are facts, used with credit, as ADR-287 reads ThrustCurve.org's records; no photo, drawing or descriptive text is copied and no vendor is contacted, so M5.6b needs no question to vendors. The motor finder's real test records come from the 2026-10-28 snapshot `hpr-motor` ships, and the held catalog file is cleared. CATS's log layout, RASAero II's design files and the guide's real-flight trace each get a route for when their work comes up. Featherweight's Bluetooth packets, makers' data sheets, Wyoming soundings and the refused references stay as they are. The orhelper migration idea is dropped.

[adr-120]: 0120-university-of-wyoming-soundings.md
[adr-194]: 0194-the-2026-10-07-field-tools-checklists-equipment-ground-tests.md
[adr-197]: 0197-the-2026-10-07-parts-catalog.md
[adr-236]: 0236-the-2026-10-11-review-supersonic-flights-from-published-comparisons.md
[adr-247]: 0247-the-simulators-guide-in-four-increments.md
[adr-274]: 0274-the-rest-of-the-importers-planned-teensy-logger-first.md
[adr-287]: 0287-thrustcurve-orgs-motor-records-used-as-published-facts.md
[adr-301]: 0301-the-whole-catalog-in-two-increments.md
[m5-6b]: ../decisions-and-roadmap.md#m5-6b
[m5-7b1]: ../decisions-and-roadmap.md#m5-7b1
[parts]: ../research/parts-catalog.md
[ideas]: ../research/ideas.md
[i715]: https://github.com/nrdptel/fusionspace-eridanus/issues/715
[i764]: https://github.com/nrdptel/fusionspace-eridanus/issues/764
[i885]: https://github.com/nrdptel/fusionspace-eridanus/issues/885
[i1006]: https://github.com/nrdptel/fusionspace-eridanus/issues/1006
[i1016]: https://github.com/nrdptel/fusionspace-eridanus/issues/1016

**Context.** Several pieces of work were held back over licensing, each waiting on a call only the
maintainer can make, as the project's rules reserve legal and licensing calls to him. On
2026-10-28 the maintainer made these calls ([#1016][i1016]). This record keeps them in one place,
so the work they free can start and the holds that stay keep their reasons.

Two of the calls concern data a company publishes about products anyone can buy, the same shape
as ThrustCurve.org's records in [ADR-287][adr-287]:

- **Parts vendors.** [ADR-197][adr-197] builds a parts catalog of hpr's own from vendors' own data
  (parachute drag coefficients and sizes, hardware masses and lengths), facts only and each cited.
  It left one question open for the maintainer: whether to ask vendors for their data.
- **The motor finder's test records.** [#885][i885] asks for real ThrustCurve.org records in the
  motor finder's tests. ADR-287 named the site's 2016 sample dump as one source of them. Since
  then `hpr-motor` ships a snapshot of the whole catalog, taken on 2026-10-28.

**Decision.**

1. **Vendors' published dimensions and masses are facts,** used with credit, by ADR-287's reasoning:
   a number a maker publishes about a product it sells is a fact about that product. Each value
   keeps ADR-197's citation (the page or datasheet, the date read, and whether it was stated,
   measured or derived). No photos, drawings or descriptive text are copied, ADR-197's rules on
   scraping and the EU database right still hold, and a vendor's entries are still removed when
   it asks. No vendor is contacted, which settles ADR-197's open question: [M5.6b][m5-6b] needs no
   question to vendors. Materials and drag coefficients were already allowed as cited facts by
   ADR-197 §3 and stay so; what this call adds is that no vendor is asked.
2. **The motor finder's real test records come from the 2026-10-28 snapshot** compiled into
   `hpr-motor` ([#885][i885]), not from the 2016 sample dump; the dump's licensing question is
   dropped.
3. **The held catalog file is cleared** ([#1006][i1006]). [ADR-301][adr-301] §5 held it back until
   the maintainer reviewed it, for a reason that is not the file's own; that review is this call.
4. **Routes, each for when its work comes up:**
   - **CATS's `.cfl` logs**, its flight computer's log format ([ADR-274][adr-274] §5): low priority. If a user asks for them, the
     layout is inferred from cached sample files' bytes, as for the Teensy logger, never from
     CATS's GPL source, and only invented test files are committed.
   - **RASAero II's design files** ([ADR-236][adr-236]): when a validation case needs one, its
     geometry is re-entered from the stated dimensions as the project's own file.
   - **The guide's real-flight trace** ([ADR-247][adr-247]): drawn from an MIT-licensed flight,
     RIT's public designs and logs ([#715][i715]).
5. **Kept as they are:**
   - Featherweight's Bluetooth packets stay out; the vendor asks users not to use them
     ([ADR-194][adr-194] §6).
   - Motor makers' data sheets stay as ADR-287 treats such data: their numbers are used as
     facts, and the PDFs are not committed.
   - Wyoming soundings: only U.S. stations' are committed fixtures ([ADR-120][adr-120] §8, which
     left that open for the maintainer to reverse; he keeps it); every station stays available
     live.
   - The refused or limited references stay so: ARL-TN-1128 and the Mach 2.45 set ([#764][i764]),
     Knacke, the streamer papers, OpenRocket's motor database, AerospaceNU's GPL-3.0 data, DRDC's
     tables (statistics only) and the STEP plans.
6. **The orhelper migration layer is dropped** (orhelper is a GPL Python bridge to OpenRocket) from [the ideas backlog][ideas]: the `.ork`
   importer already serves the users it was for.

**Alternatives considered.**

- **Ask each vendor before using its figures.** Rejected: published dimensions and masses need no
  permission as facts, and asking would hold M5.6b on answers that may never come. A vendor that
  objects still has its entries removed.
- **Keep the 2016 dump as the test source.** Rejected: the shipped snapshot is newer, already in
  the package, and its records are checked against their files.
- **Port CATS's reader from its source.** Rejected: GPL code, which the project's clean-room rule
  keeps out.

**Consequences.** ADR-197 points here for its open question. M5.6's roadmap entry states that
vendors' published dimensions and masses are used with credit and that no vendor is asked. [The
parts-catalog note][parts] and the ideas backlog record the call; the orhelper idea is marked
dropped. Carried out by their own work, not by this record: taking in #1006's file, which
unblocks [M5.7b1, the whole catalog compiled in][m5-7b1]; #885; #715; and the routes above.
