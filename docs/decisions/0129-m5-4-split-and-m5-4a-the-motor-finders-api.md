# ADR-129: M5.4 split, and M5.4a, the motor finder's API through the cache (2026-10-01)

- **Status:** accepted; the credit's wording and the fixtures' licence (§6, §7) superseded by ADR-145
- **Summary:** M5.4 split a to c; M5.4a: motor.fusionspace.co's five files in `hpr_net::motor_finder` through the cache, an hour's TTL, its structural rules refused and its derived ones pinned on the recording; eight answers of one build committed as fixtures; the credit with the site's caution on every answer

**Context.** M5.4 asks for a motor.fusionspace.co client (`meta`, `motors`, `in-stock`,
`vendors` and per-motor endpoints) joined with ThrustCurve's curves, an offline snapshot, and
`hpr motors search --in-stock --class L --max-price 150`; done when recorded-fixture tests pass,
the designation to ThrustCurve id mapping covers 95% of in-stock motors with a report of the
misses, and attribution is displayed as the API asks. That is three pieces of work: a client, a
second client and a join, and a command. The site's API (documented in the public MIT repository
`nrdptel/Hobby-Rocket-Motor-Finder`, `docs/api.md`) is static JSON on a CDN, rebuilt about
hourly, with no key, rate limit or query parameters; `cache-control: public, max-age=600`. Every
file carries `schema_version` 1 and `generated_at`; a breaking change ships under `/api/v2/`.
One motor's file is `motors/{aerotech|cesaroni|loki}/{designation}.json`, a `/` in the
designation written `~`; an unknown one is a 404 HTML page. Its terms: "Free to use; attribution
to motor.fusionspace.co is appreciated. The data is aggregated from public vendor listings and
ThrustCurve; it's provided as-is, with no warranty — verify stock and price on the vendor's own
page before relying on it." Motors carry the finder's own `id`, not ThrustCurve's, but its
designations are "verbatim as ThrustCurve spells them": on 2026-10-01 all 598 listed motors
match exactly one ThrustCurve record on (manufacturer, designation), as all 598 did on
2026-09-17's snapshot.

**Decision.**

1. **Split a to c.** M5.4a, the finder's five files through the cache; M5.4b, ThrustCurve's
   search and a motor's curve through the cache, and the join with its report of misses; M5.4c,
   `hpr motors search`, from the network, a recorded snapshot or the cache, showing the credit.
   The parent's done-when is unchanged and met by the three.
2. **`hpr_net::motor_finder`,** shaped as the other sources: `Endpoint` builds each URL; pure
   parsers `parse_meta`, `parse_motors`, `parse_in_stock`, `parse_vendors` and `parse_motor`; and
   `fetch_*` through `Client::fetch_checked`, so an answer that doesn't parse is never cached. The
   types mirror the API's JSON field for field (prices `u64` cents, impulse and thrust `f64`, the
   listing status, motor type and hazmat as enums), so a value read is the answer's and writes
   back to it, bar a listing status added later (below). Unknown fields are ignored (the API may
   add some under v1). The cheapest offer's prices are optional, as the API's OpenAPI schema has
   them, though no recorded offer lacks one; a listing status the API adds later reads as
   `Unknown` (its word is lost), so one new word doesn't refuse the list; a stock count is read as
   the vendor shows it, signed. An unknown motor type or hazmat label is refused (ThrustCurve's
   metadata lists the three types). `fetch_motor` returns the page, with its build time.
3. **Refused: the structural rules; pinned: the derived ones.** The parser refuses another schema
   version, a build time that isn't UTC ISO 8601, a list whose `count` disagrees, an impulse
   class that isn't one capital letter, a diameter not above zero, a negative impulse, thrust or
   burn time, a `listing_count` other than the listings', a cheapest offer on a motor out of stock
   or none on one in stock, a pack of zero, a unit price over its sticker price, a motor out of
   stock in `in-stock.json`, and a page holding another motor than asked. The rules the site
   derives (the unit price is the sticker over the pack, rounded half up; the cheapest offer is
   the lowest-priced in-stock listing; in stock exactly when a listing is; distinct-vendor counts;
   each motor's `path`) are tested on the recording, not enforced: a change in how the site
   computes them would otherwise refuse the whole catalogue. A value that breaks a checked rule
   refuses its whole file: the last good copy is kept, served stale online with the reason.
4. **A manufacturer and designation.** `Endpoint::motor` takes the API's three manufacturers by
   name or slug, any case, and a designation of ASCII letters, digits, `-`, `_`, `.` and `/` (the
   characters ThrustCurve's designations use), not empty and not all dots, so no request leaves
   the API's `motors/` folder; anything else is refused before the client is asked. The name is a
   `MotorName` with private fields, so no caller builds or changes one past the checks.
5. **An hour's TTL,** the site's rebuild interval; offline, or with the site down, the cached copy
   is served stale, as every source does (ADR-117).
6. **The credit.** `ATTRIBUTION` names motor.fusionspace.co, its two sources and its caution to
   check the vendor's page; it is on every `Fetched`, and the guide page and the example print
   it first. The API asks for credit but says nothing on where; M5.4c will print it with every
   listing, in text and JSON. *Superseded by ADR-145 (2026-10-03):* `ATTRIBUTION` now opens with
   the credit the site's CC BY 4.0 licence asks for, "Motor stock data from motor.fusionspace.co".
7. **Fixtures.** Eight answers of one build (2026-10-01 07:07:29 UTC) are committed under
   `crates/hpr-net/tests/fixtures/replay/`: the four lists (`motors.json` 1.6 MB, `in-stock.json`
   0.96 MB) and four motors' pages, one per manufacturer and one with a `/` (`F27R~L.json`,
   stored as `F27R_L` for file systems). The site is Neer's own, its terms say free to use, and
   its listings are public vendor pages; the motor figures in them are ThrustCurve's published
   values, which the bundled catalogue already copies with attribution (ADR-005). Recorded for the
   maintainer as no action if fine. *Superseded by ADR-145 (2026-10-03):* the site now licenses
   its answers under CC BY 4.0 and permits recorded answers as test fixtures.

**Consequences.** M5.4a is met: `tests/motor_finder.rs` reads each of the eight answers, writes it
back and finds exactly the recording's keys and values; reads it again from the cache without a
fetch, and offline, through a transport that fails if called, fresh for the hour and stale after;
and finds `ATTRIBUTION` on every answer. A typed test pins the enums' meanings (H128W a reload
shipped as hazardous material; 827 listings in stock, 2,363 out, 495 special order), since a
round trip through the same names can't. Review found the first draft refusing the whole list
for an offer with no price, which the API allows; an enum whose names could be swapped unseen;
several refusals untested; and a page fetch that dropped its build time. All are fixed. The files agree with each other (`in-stock.json` is
`motors.json`'s 282 motors in stock, value for value; `meta.json` counts 598, 282 and 12), each
refusal above is tested by a one-field change to a recording, and the derived rules hold on all
598 motors and 3,685 listings. On that build, 20 L motors were in stock and none sold for $150
or less a motor (the cheapest, $260.99), so M5.4c's example command lists nothing on the
recording; its test will need a price that does. The join (M5.4b) starts from the exact match
above, with ThrustCurve's whole `search.json` (1,156 motors, 0.93 MB, its terms unstated) kept
under `refs/` unless a smaller recorded answer can carry the test.
