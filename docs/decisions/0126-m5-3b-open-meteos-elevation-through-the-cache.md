# ADR-126: M5.3b, Open-Meteo's elevation through the cache (2026-09-30)

- **Status:** accepted
- **Summary:** M5.3b: Open-Meteo's elevation in `hpr_net::elevation`, up to 100 places a request, coordinates to 5 decimals in the URL, a year's TTL, heights refused outside −1,000 to 9,000 m, a surface model above the EGM2008 geoid with `N` left to the caller; no command yet

**Context.** M5.3b is done when a recorded elevation lookup's height is the answer's and a second
lookup works offline from the cache (ADR-125 §1). Open-Meteo's elevation API
(`api.open-meteo.com/v1/elevation`) takes comma-separated latitudes and longitudes, up to 100
places, and answers `{"elevation":[...]}`, one height per place in order, or HTTP 400 with
`{"error":true,"reason":...}`. Its data is the Copernicus DEM GLO-90 (2021 release, 3″ in
latitude), a digital *surface* model (buildings and vegetation included; the dataset's readme),
whose heights are above the EGM2008 geoid (product handbook issue 5.0, §1.2.1, p. 13), stated
to under 4 m absolute, 90% linear error, a global mean outside Antarctica and Greenland (Table 1,
p. 10); 184 of the 16,363 geotiles there (each about a degree across) are over 10 m (Table 12,
p. 31). Its licence asks for
the credit "© DLR e.V. 2010-2014 and © Airbus Defence and Space GmbH 2014-2018 provided under
COPERNICUS by the European Union and ESA; all rights reserved"; Open-Meteo's API data is CC BY 4.0
and it asks for a credit to Copernicus and to itself. The recorded heights are whole metres, and
Spaceport America's 1,400 m equals the `elevation` of the two weather recordings there.

**Decision.**

1. **A module beside the weather.** `hpr_net::elevation` mirrors `open_meteo`: an
   `ElevationRequest` builds the URL (1 to 100 places, latitude and longitude ranges, an optional
   self-hosted endpoint with no `?` or `#`), `parse` is pure and returns an `Elevation` (the place
   and `height_msl_m`) per place, and `fetch` goes through `Client::fetch_checked`, so an answer
   `parse` refuses is never cached. The URL writes each coordinate to 5 decimals (about 1 m),
   halves away from zero, trailing zeros dropped and zero unsigned: the cache key is the URL, and
   a place kept in radians comes back in degrees with other last digits (−106.91 as
   −106.91000000000001), which would miss its cached answer offline. Rounding the binary value
   straight to 5 decimals still flips about 1 in 17 six-decimal values on a half step (32.990415
   comes back as 32.990415000000006), so the value is first written exactly to 9 decimals and
   that decimal is rounded: a value given to 8 decimals or fewer keeps its key. The other sources
   still write `f64`'s shortest digits (#272).
2. **A year's TTL.** The DEM changes with a new release, years apart; a year keeps an old cache
   from going stale on the field while still refreshing.
3. **What `parse` refuses.** Another count of heights than places, a height that is not a number,
   and a height outside −1,000 to 9,000 m: the lowest land (by the Dead Sea, about −440 m) and the
   highest (8,849 m) with margin, so a 16-bit no-data value (−32,768) can't pass as a height. The
   ocean, which has no tiles, reads 0 m and passes.
4. **Heights above sea level, `N` left to the caller.** hpr has no geoid model, so the height is
   returned as the API gives it. The guide says to give a site `H + N` and the environment `N`
   when `N` is known, and otherwise `H` with `N = 0`, which keeps the atmosphere's height right and
   moves the site's place in space by `N` (gravity by about 3×10⁻⁴ m/s² per 100 m).
5. **Two recordings, no command.** `open-meteo-elevation.json` (Spaceport America) and
   `open-meteo-elevation-three.json` (with the Dead Sea's surface as the radar saw it, −427 m, and
   the open Atlantic, 0 m), recorded unchanged; the example `site_elevation` reads the second. A command-line lookup
   is left for later: the done-when is the library's.

**Consequences.** M5.3b is met. Nothing measures the DEM's accuracy against surveyed heights, and
nothing in a flight looks the site up by itself: a program passes the height to its `Geodetic`
site. Over trees or buildings the height is the surface's, not the pad's. The cache key is the
whole request, so a place looked up in a list is found offline only by the same list.
