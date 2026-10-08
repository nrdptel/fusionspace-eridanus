# ADR-131: M5.4c, `hpr motors search`, stock and prices at the command line (2026-10-01)

- **Status:** accepted
- **Summary:** M5.4c: `hpr motors search`, the finder's list from the network, the cache or a saved file; five filters, `--max-price` in exact cents on the cheapest in-stock offer; cheapest first; both credits on every list; the example tested on an edited copy, as the recording lists nothing at $150

**Context.** M5.4c (ADR-129 §1) asks for `hpr motors search --in-stock --class L --max-price 150`,
listing from a recorded snapshot and offline from the cache, with the credit shown as the API
asks. The library reads motor.fusionspace.co's lists (`hpr_net::motor_finder`, ADR-129) and joins
them to ThrustCurve.org's records (`hpr_net::thrustcurve`, ADR-130). On the recorded build
(2026-10-01 07:07:29 UTC), 20 L motors were in stock and the cheapest sold for $260.99 a motor, so
the example lists nothing on the recording.

**Decision.**

1. **A third subcommand of `hpr motors`,** in `crates/hpr-cli/src/motor_search.rs`. The list comes
   from the network through the platform's cache (`Http`, an hour's TTL, as ADR-129 §5), from the
   cache alone with `--offline`, or from a saved `motors.json` or `in-stock.json` with `--from`,
   which touches neither; `hpr weather`'s client and its read-from line are shared, not copied.
   `--in-stock` or `--max-price` (which keeps only motors in stock) fetches `in-stock.json`
   (0.96 MB), and otherwise `motors.json` (1.6 MB). Offline, a search of motors in stock with no
   copy of `in-stock.json` reads `motors.json`'s copy, which answers it too; with neither, the
   refusal names `in-stock.json`. A saved file is read with `parse_motors`, whose checks a fetched
   answer passes too; `--in-stock` then filters it, so either file serves any search.
2. **Five filters, each refused before anything is read when it can't match anything real:**
   `--in-stock`; `--class` as `hpr motors list` reads it; `--diameter` within 0.5 mm, as `list`;
   `--manufacturer` by the API's three names or slugs, in any case (`Cesaroni` is a slug); and
   `--max-price` in U.S. dollars, read as exact cents (`149.99` is 14,999 cents, never a float just
   under it; at most two decimals; no sign, `$` or exponent). The price compared is the site's
   `cheapest_in_stock.unit_price_cents`, one motor's price at the cheapest vendor with it in
   stock: a motor out of stock has none, so `--max-price` keeps only motors in stock, and an offer
   in another currency, or with no price, never passes. The site derives `cheapest_in_stock`
   itself (ADR-129 §3, tested on the recording); hpr doesn't recompute it from the listings, so a
   cheapest offer in another currency would hide a dearer one in dollars. Every recorded offer is
   in dollars.
3. **Cheapest first,** by that price, then by maker and designation; motors with no price in
   dollars last, by maker and designation. A search for the cheapest motor is the command's use.
4. **Both credits on every list,** empty or not: the finder's `ATTRIBUTION`, with its caution,
   then ThrustCurve.org's, whose published figures the finder repeats. Text prints them on lines 3
   and 4, under the count and where the list was read from; JSON carries them in `attribution`.
   ADR-129 §6 promised the finder's credit with every listing, in text and JSON.
5. **An empty `--max-price` search says why:** a last line names the cheapest motor in stock the
   other filters keep, and its price, so the example's empty answer still answers "what does an L
   cost?"; counted in stock, it reads the same from either file. Text only; the JSON's motors are
   the answer. Text cells from the site (designations, makers, vendors, currencies) print with
   control characters as `?`, so a scraped escape sequence can't reach the terminal.
6. **No ThrustCurve.org lookup in the command.** The figures shown are the finder's (ThrustCurve's
   published values); matching to ThrustCurve.org records and downloading curves stays in the
   library (ADR-130), which a program calls, until a command to fetch a curve is asked for.
7. **The example's test.** On the recording, the example lists nothing, and the hint names AeroTech
   L1520T at $260.99. The test then edits a copy of the recording in a temporary folder, L1520T's
   cheapest offer and its matching listing at $149.99, and the same command lists exactly that
   motor; at `--max-price 149.98` it lists none. The committed recording is not edited.

**Consequences.** M5.4c is met, and with it M5.4: `tests/motors_search.rs` runs the command as a
user does. From the recorded in-stock list, `--in-stock --class L --max-price 300` lists 4 motors
and the example none, each output checked against `schema/cli/motors-search.schema.json`. Every
listed motor's values equal the recording's, and the motors and their order equal what the test
finds by filtering the recording's JSON itself, on seven searches over both lists (282 motors in
stock, 598 in all, 55 of class L, 20 of them in stock; 81 of 75 mm, and 94 at 75.5 mm, the
tolerance's edge). An edited copy with one L offer in Canadian dollars and one with no price shows
both left out of `--max-price` and listed last. Offline, from a cache filled through the
recorded answers, three searches list what the saved file does, read from the cache; with an empty
cache both lists are refused, naming their URLs; with one list cached, the searches it can answer
read it. Each refusal is tested, before and after a file is read. Review found the
non-dollar and unpriced paths and the diameter's tolerance untested, an offline search refused
with the other list cached, the hint counting motors out of stock from the whole list,
`--max-price` taking the next option as its value, and site text printed raw; all are fixed. The guide's [CLI page](../cli.md#motors-you-can-buy) runs both examples through
`cargo xtask cli`. Fetching online was not run; it is the same `Client` and `Http` as `hpr weather`,
which were run by hand on 2026-09-30.
