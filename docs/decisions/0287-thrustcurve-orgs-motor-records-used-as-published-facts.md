# ADR-287: ThrustCurve.org's motor records used as published facts (2026-10-20)

- **Status:** accepted; replaces the reading, in [ADR-145][adr-145] §1, [ADR-161][adr-161] §3 and [ADR-186][adr-186], that ThrustCurve.org's lack of a license keeps its motor records out of the repository; returns to [ADR-005][adr-005]'s first reading; no done-when changes
- **Summary:** ThrustCurve.org's motor records are published facts about motors anyone can buy, so the project uses them, from the site, its API or the site's 2016 sample dump, with ThrustCurve.org's credit; curve files marked public domain or with no license may be committed too, files marked "free usage" or "other (restricted)" are fetched only when a user asks, and every accuracy check stays.

[adr-005]: 0005-solid-motors-statistics-consumption-grains-file.md
[adr-067]: 0067-curves-come-from-openrockets-own-database-by.md
[adr-145]: 0145-recorded-answers-and-their-licences-stand-in.md
[adr-161]: 0161-openrocket-curves-by-digest.md
[adr-186]: 0186-data-network-and-python-fixes.md
[g23]: ../evidence.md#g23
[i295]: https://github.com/nrdptel/fusionspace-eridanus/issues/295
[i885]: https://github.com/nrdptel/fusionspace-eridanus/issues/885
[tc-note]: ../research/thrustcurve-data.md
[ideas]: ../research/ideas.md

**Context.** [ThrustCurve.org](../glossary.md#thrustcurveorg) is the hobby's public database of
motor data. For each motor it publishes a record: designation, maker, diameter, length, masses,
total impulse, average and peak thrust, burn time, delays and certifying body. It also lists the
thrust-curve files that contributors upload. Each file carries the license its uploader picked:
public domain, "free usage" (free software licenses, the GPL among them), "other (restricted)", or
none. The site grants no license for the records. Its home page reads "All rights under
copyright reserved" ([the ThrustCurve note][tc-note]).

The project has read that line two ways.

- [ADR-005][adr-005] (2026-09-17) copied the 32 bundled motors' figures from the API, with credit,
  as facts about commercial products. The maintainer confirmed that reading on 2026-09-18, and
  ADR-130 (2026-10-01) committed three makers' searches on it.
- [ADR-145][adr-145] (2026-10-03) replaced the three recorded searches with stand-ins, keeping
  only the figures the motor finder relays. Then [ADR-186][adr-186] (2026-10-06, [#295][i295])
  rebuilt the bundled catalog from the curve files alone. Both took the stricter reading for what
  only ThrustCurve.org states.

So the motor finder's tests run on stand-in records ([evidence row G23][g23]). G23's search on
2026-10-20 found a 2016 sample dump of the site's database in the site's own source repository,
`JohnCoker/thrustcurve3` (ISC license, commit 3325878d). It holds 280 motor records and 411 curve
files: 104 public domain, 16 free usage and 291 with no license. ISC grants rights in "this
software", so whether it covers that data was put to the maintainer.

**Decision.** On 2026-10-20 the maintainer decided that ThrustCurve.org's data may be used, with
credit. Its figures are facts about products sold to the public. OpenRocket's own motor database
comes from ThrustCurve.org too ([ADR-067][adr-067]).

1. **Records.** ThrustCurve.org's motor records may be committed, bundled and published, as
   recorded and unchanged. That holds whether they come from the site, its API or the 2016 dump.
   The ISC question is moot: the records don't rest on that license.
2. **Curve files.** Files marked public domain or with no license are treated the same way. Two
   kinds stay out of the repository. `hpr` still fetches them when a user asks, as now:
   - "other (restricted)": the uploader marked the file restricted, for example with an explicit
     copyright.
   - "free usage": the license may be copyleft, which this `MIT OR Apache-2.0` project can't take
     in. `cargo deny` refuses copyleft code for the same reason.

   The maintainer's decision covered all the site's data; these two lines are the project's own.
   They cost little: on 2026-10-17 the live site's 1,705 solid-motor files were 1,084 with no
   license, 588 public domain and 33 free usage, so none marked restricted
   ([the ThrustCurve note][tc-note]).
3. **Credit.** Every committed ThrustCurve.org file, and every output that shows its figures,
   carries ThrustCurve.org's credit and link, as `hpr_net::thrustcurve::ATTRIBUTION` and
   `THIRD-PARTY-NOTICES.md` give it now. `THIRD-PARTY-NOTICES.md` lists each such file, gives
   this ADR as its basis and says the site reserves its rights.
4. **Every check stays.** Real data comes in through the same checks as anything else. A record is
   replayed as recorded, never edited to agree with its curve. A curve is bundled only if it passes
   the bundle's accuracy rule. That matters: on 2026-10-17 only 198 of the site's 588 public-domain
   solid-motor files came within 1% of their motor's published total impulse, average thrust and
   burn time. In the first survey, 41 of 554 public-domain files missed their motor's total
   impulse by more than 5% ([the ThrustCurve note][tc-note]). More data makes the simulator safer
   only where the checks catch the wrong records.
5. **If ThrustCurve.org's maintainer objects,** the data comes out of `main` and the notices say
   so. Git history keeps it: removing it there would need a force-push, which the project's rules
   forbid.

This is the maintainer's call on a licensing question. It isn't legal advice.

**Consequences.**

- [#885][i885] does the work. The motor finder's tests replay real records, and G23's row says
  so. The "grants no license, so ..." reasoning in pages, code comments and notices is reworded
  as a statement about the site.
- Until then the stand-ins stay, and the pages stay true: the site does grant no license.
- `catalog.json` may carry ThrustCurve.org's own figures again, beside those worked out from the
  curves. Nothing requires it.
- One idea placed in [the ideas list][ideas]: ThrustCurve.org's whole catalog of records bundled,
  so motor search works with no network.
