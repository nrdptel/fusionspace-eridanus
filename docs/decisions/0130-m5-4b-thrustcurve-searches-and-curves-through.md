# ADR-130: M5.4b, ThrustCurve searches and curves through the cache, and the in-stock join (2026-10-01)

- **Status:** accepted; the three searches (§7) superseded by ADR-145
- **Summary:** M5.4b: ThrustCurve.org's search and download in `hpr_net::thrustcurve` through the cache, a day's TTL; the join by exact maker and designation, misses reported not guessed; three makers' searches and two public-domain files committed as fixtures; the join's report in `validation/reports/thrustcurve-join.md`

**Context.** M5.4b (ADR-129 §1) asks for ThrustCurve.org's search and a motor's curve through the
cache; done when the designation to ThrustCurve id mapping covers at least 95% of the motors in
stock, with a report of the misses, and a mapped motor's recorded curve reads with `hpr_motor`.
ThrustCurve's API (`/api/v1`, an OpenAPI 2 spec under the ISC licence; its page says the JSON
endpoints take a query string by GET or a JSON body by POST, and no key or header) has a search,
whose records carry ThrustCurve's 24-hex-digit `motorId`, and a download, whose files come
base64-encoded with their format (`RASP` or `RockSim`), source (`cert`, `mfr`, `user`) and licence
(`PD`, `free`, `other`, or absent). Its spec says "only fields with values will be returned". The
API states no terms for its data. Errors come back as HTTP 200 with an `error` field, on the
answer or on a search criterion; an unknown motor id downloads as an empty list. On 2026-10-01 a
search by maker with `maxResults=5000` returned AeroTech's 307 records, Cesaroni Technology's 296
and Loki Research's 60, each whole (`matches` equal to the records returned), and the same records
whether the maker was named in full or by its abbreviation.

**Decision.**

1. **`hpr_net::thrustcurve`,** shaped as the other sources: `Search` and `Download` build the URLs
   (GET, query values percent-encoded, fields in a fixed order so a URL is one cache key);
   `parse_search` and `parse_download` are pure; `fetch_search` and `fetch_download` go through
   `Client::fetch_checked`, so an answer that doesn't parse is never cached. The types mirror the
   JSON field for field and write back to it; every record field but `motorId`, `manufacturer` and
   `designation` is optional, as the spec allows. The motor type, availability, source and licence
   stay the API's words (strings: the response schema types them as open strings); the format is
   an enum of the two the request takes, and the answer must hold the one asked. A request's motor
   id is taken in either case and kept in lower case, as the API writes ids, so one motor has one
   cache key and its answer matches the request.
2. **Refused:** an answer or criterion carrying `error`; a search returning more records than
   `matches`; a `motorId` not of 24 hex digits; a size, thrust, impulse, burn time or weight below
   zero; a file's data that isn't base64; a download holding another motor's file or format. A
   request's motor id is checked before the client is asked. A file that decodes but isn't UTF-8
   (`hpr motors` reads motor files as UTF-8 too), or that `hpr_motor` refuses, is an error of
   `DataFile::text` or `DataFile::read` for that file alone, not of the parser: the answer is
   still ThrustCurve's, and the motor's other files stay readable.
3. **The join by name only.** `join` maps a finder motor to the one record whose `manufacturer`
   (full name) and `designation` equal the motor's, byte for byte; none or several is a `Miss` with
   its reason, and `Join::report` lists counts by maker and every miss. A record given twice (the
   same `motorId`, as from overlapping searches) counts once. A `Mapped` carries the finder
   motor's index and the whole record, for M5.4c to show price beside curve. No fallback on case,
   punctuation, impulse or diameter: the finder copies ThrustCurve's spellings (ADR-129), and a
   guessed match could hand a flier the wrong curve, where a miss only costs a lookup. As the
   finder copies ThrustCurve's figures too, a full match was expected on the recording (ADR-129
   found 598 of 598 exact); the join is there to catch drift.
4. **Whole searches for the join.** `fetch_finder_records` runs one search per maker the finder
   reads (`motor_finder::MANUFACTURERS`, by full name), each asking `maxResults=5000`, and refuses
   (and doesn't cache) one that matches more records than it returns, or holds another maker's
   record, so a join never runs on part of a maker.
5. **A day's TTL.** Records and curves change seldom; offline or with the site down, the cached
   copy is served stale, as every source does (ADR-117).
6. **The credit.** The API asks for none. `ATTRIBUTION`, "Motor data and thrust curves courtesy of
   ThrustCurve.org", matches the bundled curves' notice (ADR-005) and is on every `Fetched`. A
   file's licence is passed through, not filtered: the caller decides what to keep or pass on.
7. **Fixtures.** Five answers recorded 2026-10-01 08:22 UTC are committed under
   `crates/hpr-net/tests/fixtures/replay/`: the three makers' searches (250, 246 and 50 kB) and
   two downloads, AeroTech J450DM's RASP file (certification data, `PD`) and AeroTech F27R/L's
   RockSim file (`user`, `PD`), requested by format so no file without a public-domain licence is
   recorded. The records are published motor figures, as the bundled catalogue already copies
   (ADR-005); ThrustCurve's whole `search.json` (1,156 records) stays under `refs/`, the three
   makers' searches being enough for the join. *Superseded by ADR-145 (2026-10-03):* the three
   searches are no longer ThrustCurve's answers but stand-ins in their shape; the two downloads
   stay. The measurements below were made on ThrustCurve's answers.
8. **The report.** `validation/reports/thrustcurve-join.md` holds `Join::report` on the recordings,
   compared as text on every OS by `tests/thrustcurve.rs` (counts only, no float);
   `HPR_WRITE_THRUSTCURVE_JOIN=1` rewrites it.

**Consequences.** M5.4b is met: on the recorded in-stock list (282 motors, the finder's build of
07:07:29 UTC) and the three searches (663 records), 282 of 282 motors map to exactly one record,
each listing a data file, against the 95% asked. The test builds its own name-to-id table from the
recordings' JSON, not through `join` (the same rule, so it shows the code keeps the rule), and,
independently of the rule, checks that every match's diameter, total impulse, average thrust and
burn time equal the finder motor's: they do for all 282. J450DM, mapped and in stock, has its recorded RASP file read
by `hpr_motor::eng` to the 36 points in its lines, and the file is byte for byte the public-domain
one `hpr_motor` bundles (`5f4294d20002e9000000086b.eng`, downloaded 2026-09-17); F27R/L's RockSim
file reads by `hpr_motor::rse` to the points in its XML. A renamed motor, a doubled record and a
designation in another case are misses with their reasons; a record given twice counts once, and
another maker's record of the same designation leaves the match alone. Each refusal is tested by a
one-field change to a recording. Review found an upper-case id accepted and then its answer
refused, a record given twice making every motor a miss, a search of another maker accepted, one
non-UTF-8 file refusing a motor's whole answer, and gaps in the refusal tests; all are fixed. The example `motor_stock` prints the join and J450DM's curve: 1,061.6 N·s,
2.28 s and 541.4 N from the file, beside its record's published figures of 1,055 N·s, 2.27 s
and 558 N. M5.4c (`hpr motors search`) can now show a motor in stock with its curve.
