# ADR-319: The motor stock API is open again, under CC BY 4.0 (2026-11-03)

- **Status:** accepted; reverses [ADR-315][adr-315] §2 for the motor stock API at motor.fusionspace.co only; the rest of ADR-315 stands; no done-when changes
- **Summary:** From 2026-11-03 the motor stock API's data is licensed under CC BY 4.0 again, and any website may call it from a browser, as before 2026-11-01. The maintainer chose this because the scraper that builds the data is MIT-licensed and will stay so, so anyone can build the same data anyway. The narrower terms of 2026-11-01 are withdrawn, and copies from those days are CC BY 4.0 too. hpr's credit line names CC BY 4.0 again, as release 0.1.0's does.

[adr-315]: 0315-the-2026-11-01-licensing-layers.md
[adr-129]: 0129-m5-4-split-and-m5-4a-the-motor-finders-api.md
[i1097]: https://github.com/nrdptel/fusionspace-eridanus/issues/1097
[mf-api]: https://motor.fusionspace.co/api

**Context.** On 2026-11-01 [ADR-315][adr-315] §2 put [motor.fusionspace.co's stock API][mf-api]
under the site's own terms. The data was free for personal use and for FusionSpace's own tools,
and any other site, app or service needed the maintainer's written permission. Browsers on other
sites got a refusal. [#1097][i1097] then changed hpr's credit line to name those terms.

Two days later a paid simulator was still showing the data. It fetched the API through its own
server, which the origin check doesn't stop, since programs such as `hpr` have to keep working.
Closing that route as well would not have stopped anyone either:

- The data is rebuilt every hour by a scraper whose source is public under MIT. Anyone can run it
  and get the same data without the service.
- The service's repository publishes the hourly snapshot as a file.
- What the data says, which vendor lists which motor at what price, is a set of facts.

**Decision.**

1. **The data is CC BY 4.0 again.** It is credited "Motor stock data from motor.fusionspace.co",
   as before 2026-11-01. The narrower terms are withdrawn, and data published under them, from
   2026-11-01 to 2026-11-03, is CC BY 4.0 too.
2. **Any site may call the API from a browser.** Every API file answers with
   `Access-Control-Allow-Origin: *`, and the allowlist of sites is gone.
3. **The scraper stays MIT.** The service is as open as the code that builds it.
4. **hpr's credit line names CC BY 4.0 again.** `motor_finder::ATTRIBUTION`, `hpr motors search`,
   the command reference and the stock guide go back to their wording before #1097. Release 0.1.0
   prints that wording, so it is right again.
5. **The rest of ADR-315 stands.** The engine stays `MIT OR Apache-2.0`, M9.0 chooses the app's
   license, and the trademark waits. Services still to come, such as hpr's own stock service,
   accounts and cloud saves, set their terms when they are built. This record covers the motor
   stock API alone.

**What changes for a user.**

- **The `hpr` command and the Python package:** the credit line names CC BY 4.0, as release 0.1.0's
  does. Nothing else changes.
- **A website, app or service that shows the stock data:** it may, under CC BY 4.0 with the
  credit, and a browser on any site can read the API again.

**Alternatives considered.**

- **Keep ADR-315's terms and block the server route too.** Rejected. Any block has a free way
  round it (the published snapshot or the MIT scraper), so it would cost work and goodwill and
  protect nothing.
- **Keep the terms and grant permission site by site.** Rejected. It keeps a restriction that
  can't hold, and asks every user to write in first.
- **Make the scraper's source closed.** Rejected. The maintainer wants it to stay open.

**Consequences.** ADR-315's status names this record. THIRD-PARTY-NOTICES.md, VISION V14 and the
stock guide say the API is CC BY 4.0 again. The service's own pages carry the same change, and
say when the narrower terms ran.
