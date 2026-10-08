# The four FusionSpace web tools: research for rebuilding them in hpr

**Bottom line:** three of the four tools (Charge, Window, Muster) can move into hpr entirely: their
logic becomes library code, and their pages become screens of the app. Motor Finder can't. Its
scraper, stock history and restock emails need a server that runs every hour. It stays a hosted
service, and hpr keeps reading its API. Each old site stays up until its replacement ships
([M9.6, the four web tools rebuilt][m9-6]; [ADR-191, the 2026-10-06 ideas][adr-191]). Researched
October 6, 2026 from the four public repositories and their live sites. Nothing here is
implemented yet.

The same day, Neer decided to sunset all four and rebuild them from the ground up in this
project, more platform agnostic and more connected to the other hpr tools.

## The tools

The register numbers are from the FusionSpace product system
([ADR-164, the product system decision][adr-164]). hpr itself was `FS · SW · TOOL 005` and is now
`FS-ACHERNAR · SW · TOOL 001`, after its star in project Eridanus
([ADR-198, project Eridanus](../decisions/0198-the-2026-10-07-project-eridanus.md)).

| tool | what it does | how it runs | lines of code |
|---|---|---|---|
| Motor Finder (001), [motor.fusionspace.co](https://motor.fusionspace.co) | stock and price for AeroTech, Cesaroni and Loki across 12 vendors; substitutes; order plans; restock emails | Python scraper every hour, a Cloudflare Worker relay, SQLite snapshots, a static Next.js site, email alerts on Upstash | about 35,800 |
| Charge (002), [charge.fusionspace.co](https://charge.fusionspace.co) | black powder ejection charge sizing; ground-test log; bench mode; printable cards | static Next.js; data only in the browser and the URL | about 11,500 |
| Window (003), [window.fusionspace.co](https://window.fusionspace.co) | launch weather for about 100 US fields: wind, winds aloft, density altitude, ceiling, alerts, calm windows | static Next.js calling Open-Meteo, NWS and METAR from the browser; an hourly rebuild of a conditions feed | about 14,000 |
| Muster (004), [muster.fusionspace.co](https://muster.fusionspace.co) | which reloads fit which motor cases, with spacers; shopping list; kit planner | static Next.js; a sourced compatibility graph checked at build time | about 9,100 |

All four are MIT. None shows public traffic figures.

## Motor Finder: the API hpr already reads

hpr's motor search ([M5.4, live motor stock][m5-4]) reads Motor Finder's public API through
`hpr_net::motor_finder`. The base address is fixed in the code, so any released hpr breaks if the
API moves. The API:

- base `https://motor.fusionspace.co/api/v1/`;
- files `meta.json`, `motors.json`, `in-stock.json`, `vendors.json`, `openapi.json`, and one file
  per motor at `motors/{maker}/{designation}.json` (a `/` in a designation is written `~`);
- every file carries `schema_version: 1` and `generated_at`;
- no sign-in, no rate limit, open CORS, `cache-control: max-age=600`, an ETag on every file;
- data under CC BY 4.0.

On October 6 its `meta.json` listed 598 motors, 256 in stock, from 12 vendors. hpr caches it for
3,600 s.

**This API is not a contract.** Neer decided later on October 6 that breaking 0.1 is fine: the
aim is the best product, and none of his old products may hold it back. hpr's stock
service is designed for hpr: its own data shape, its own address, its own cache rules. Release
0.1's reader may stop working when it lands, and the release notes say so
([ADR-192, the 2026-10-06 follow-up][adr-192]).

## The workflows to rebuild

[M9.6][m9-6]'s first check is that each workflow below runs in the app. Those marked *service*
stay on the hosted Motor Finder and are reached through it, with a cached fallback offline.

| tool | workflows |
|---|---|
| Motor Finder | search and filter motors; a motor's page with its stock history (*service* for the history); in-stock substitutes; compare up to four motors; my rockets; an order plan for the cheapest total; restock emails (*service*) |
| Charge | size by pressure; size by shear pins and friction; size by Fetter's model; a backup charge; vent holes; the ground-test log; bench mode; a printable card and report |
| Window | a field's conditions now (surface wind, winds aloft, density altitude, ceiling, alerts, air quality); calm windows; the 7-day outlook; drift; a club briefing; the field list |
| Muster | the reloads that fit a case; the cases that take a reload; a shopping list; a kit planner |

## What each tool brings, and what doesn't come across

hpr's rules ([CONTRIBUTING.md](https://github.com/nrdptel/fusionspace-eridanus/blob/main/CONTRIBUTING.md)) are stricter than the tools' in four places.
Each conflict below is what the port must change.

**Charge.** Its two sizings are the ideal gas law (the HARA sizing page's constants) and Tom
Fetter's *Using Black Powder for Parachute Deployment*, Rev 1.2, tested against the paper's
Tables 16-1 and 17-2. Fetter's tables are a good candidate for the measured basis that
[M6.7, ejection charges][m6-7] asks for. Four changes:

- **The pins' strength is taken from the low end.** Charge follows Fetter's paper, which sizes
  shear pins at nylon 6/6's *minimum* strength, 9,600 psi (its §6, Table 6-1), and then adds the
  paper's 40% safety factor. A weaker pin needs less force, so the minimum gives less powder than
  a stronger pin would need. The comment at `lib/fetter.ts:54` states the reverse. Whether the
  40% margin covers the real spread of pin strength hasn't been measured. hpr doesn't port the
  minimum: M6.7 already asks for the high end of the pins' strength.
- The ideal-gas mode has no efficiency factor, which M6.7 requires.
- The backup charge's default margin, 20% over the primary, cites no source.
- Fetter's model is stated for sea level up to about 20,000 ft; the port refuses outside that.

**Window.** It reads ISA pressure altitude and Magnus–Tetens vapor pressure for density altitude,
and drift as mean wind divided by descent rate. Three changes:

- Drift comes from hpr's simulated landing ([M9.4, pad day][m9-4]). The mean-wind formula ignores
  weathercocking and the climb.
- The ceiling panel's "No-go", "Tight" and "Clear" labels (`lib/weather/ceiling.ts`) go; hpr
  shows the margin below the cloud base and gives no go/no-go verdicts.
- The 20 mph wind line gets its source, a source date and a stale-after date, like every
  regulatory number in hpr.

hpr already fetches Open-Meteo ([M5.2, weather][m5-2]). It doesn't yet have density altitude, METAR
ceilings, NWS alerts or a calm-window search.

**Muster.** Its graph of cases, closures and spacers, each edge with its source, and the validator
that fails a build on a bad edge, become a data module beside the motor catalog. Then a design
check can say whether a reload fits the case a user owns. One change: its `lib/data/reloads.json`
copies ThrustCurve statistics, which hpr refused to bundle (issue #295). hpr rebuilds those from
the public-domain curve files, as `cargo xtask motor-catalog` already does.

**Motor Finder.** Its substitute rule (±15% total impulse, ±35% average thrust) becomes substitutes
ranked by simulated apogee for the user's own rocket ([M6.3, challenge specs][m6-3]). The
order-plan search is a pure function and can move into the library. Its committed
`data/curves.json` and `data/thrustcurve_*.json` files copy ThrustCurve data and stay out of hpr
for the same reason as Muster's.

## A sunset, tool by tool

| tool | the old site | users' data | the maintainer's call |
|---|---|---|---|
| Motor Finder | stays; its pages may later point to the app | restock-alert subscribers (emails) stay with the service | the relay's fit with the site's "no evasion" line |
| Charge | permanent redirects that map its URL parameters into the app | ground-test logs live only in users' browsers: the app imports its versioned backup file | the cut-over date |
| Window | redirects from each field's page to pad day | saved fields are exported by the user | whether the hourly conditions feed stays |
| Muster | one redirect for each case and reload page | none | closing its open reconcile issue (#54 in its repository) |

Domains, the fusionspace.co project cards and donation links are Neer's.

[adr-164]: ../decisions/0164-the-fusionspace-product-system.md
[adr-192]: ../decisions/0192-the-2026-10-06-follow-up-best-over-first-and-export-control.md
[adr-191]: ../decisions/0191-the-2026-10-06-ideas-web-tools-watches-repo-layout-hardware.md
[m5-2]: ../decisions-and-roadmap.md#m5-2
[m5-4]: ../decisions-and-roadmap.md#m5-4
[m6-3]: ../decisions-and-roadmap.md#m6-3
[m6-7]: ../decisions-and-roadmap.md#m6-7
[m9-4]: ../decisions-and-roadmap.md#m9-4
[m9-6]: ../decisions-and-roadmap.md#m9-6
