# ADR-119: M5.2 split; Open-Meteo's pressure levels as a sounding (2026-09-30)

- **Status:** accepted
- **Summary:** M5.2 split a to d; M5.2a: Open-Meteo's pressure levels as a sounding, the ground as its lowest level, levels below it dropped, heights geopotential

**Context.** M5.2 names four sources: Open-Meteo's forecast and historical-forecast APIs with
pressure-level winds, NOAA's GFS and RAP as GRIB2 read in pure Rust, University of Wyoming
soundings, and ERA5 or GFS files the user provides. Its *done when*: recorded-fixture tests pass,
and a profile built from a recorded Open-Meteo response reproduces the pressure, temperature and
wind at the pressure levels. A GRIB2 decoder alone is a pull request's work.

**Decision.**

1. **Split a to d**, each with its own *done when*: a, Open-Meteo; b, Wyoming soundings; c, GRIB2;
   d, the user's files and the `hpr weather` command that the CLI already lists as planned for
   M5.2 (ADR-105). The parent's two bullets are M5.2a's.
2. **`hpr_net::open_meteo`**: an `OpenMeteoRequest` (place, time, API, optional model, optional
   endpoint for a self-hosted server) builds the URL, `OpenMeteoProfile::parse` reads the JSON, and
   `fetch` goes through a `Client`, so the cache and the offline mode apply. The request asks for
   the two whole hours around the launch (`start_hour`, `end_hour`, UTC, `timeformat=unixtime`),
   so one launch time is one cached answer of about 7 KB; the 19 levels Open-Meteo serves, each
   with temperature, relative humidity, wind speed and direction, and geopotential height; and the
   surface pressure, 2 m temperature and humidity, and 10 m wind. Units are named in the request
   (`wind_speed_unit=ms`) and checked in `hourly_units`: any other unit is refused, not converted.
   A forecast stays fresh for an hour, an archived forecast for 30 days.
3. **The ground is a level**, at the answer's `elevation`, with the surface pressure, the 2 m
   temperature and humidity, and the 10 m wind: the wind on the rail comes from the 10 m wind, not
   a step to the lowest level above (Loft lesson L6). Placing 2 m and 10 m values at 0 m is a
   stated approximation.
4. **Levels below the ground are dropped**: a level is kept only if its pressure is below the
   surface pressure and its height above the elevation. The models extrapolate beneath high
   ground; at Spaceport America's 1,400 m, 1000 to 900 hPa. A level with a `null` at either hour is
   dropped too, and one whose relative humidity is outside 0 to 100% (which `SoundingProfile`
   refuses), rather than refusing a whole forecast over one level; a `null` or an out-of-range
   humidity at the surface refuses the answer. Each is listed in `dropped` with its reason.
5. **Heights are geopotential metres**, converted with WMO-No. 8 eq. 12.16 at the answer's
   latitude, as `docs/physics/atmosphere.md` already said. Open-Meteo's documentation calls the
   variable an altitude above sea level, so the recordings are checked: from 500 to 30 hPa, the
   recorded layers match the hypsometric thickness from the levels' virtual temperatures to within
   0.03% to 0.13% on average; read as geometric heights they would be 0.58% to 0.68% too thin
   (`recorded_heights_are_geopotential` holds each mean to those ranges).
6. **Between the hours**, values are linear in time, as `a + w (b − a)`, which is exact when the
   hours agree (so 100% humidity stays 100%; `(1 − w) a + w b` rounded it above 1 at some
   seconds), and the wind is interpolated by components, as `hpr_io::era5` does. On the hour, the
   recorded speed and direction pass through unchanged. Directions are folded into `[0, 2π)`.
   Hours outside 1970 to 9999 are refused, which keeps the time arithmetic from overflowing.
7. **Relative humidity is read as over liquid water**, which `SoundingLevel` means. Whether each
   model reports it over ice at cold levels is not known; the density effect, `0.378 (e_w − e_i)/p`
   with `e_w − e_i` at most 27 Pa near −12 °C, is under 0.03% at 400 hPa; higher up the air is
   colder and the gap smaller (about 6 Pa at −40 °C, 0.08% even at 30 hPa).
8. **Fixtures** are two answers recorded unchanged on 2026-09-30 (historical forecast for 21 June
   2025, forecast for 2 October 2026, both at 32.99° N, 106.97° W), CC BY 4.0 with attribution in
   `THIRD-PARTY-NOTICES.md`. `Replay` serves them keyed by the exact URL `OpenMeteoRequest` builds,
   which pins the URL; the loopback server serves them over HTTP.
9. **Only an answer that parses is cached.** `Client::fetch_checked` takes the source's check (here
   `OpenMeteoProfile::parse` at the launch time, then `sounding`): a refused body is not stored, and online a stale
   good copy comes back in its place with the reason, else `NetError::Refused`; a cached copy the
   check refuses is a miss online and the error offline. Without it, a 200 with empty hours (the
   historical API before its data lands) would be kept for 30 days and could overwrite a good
   copy. `Client::fetch` is the check that accepts everything.
10. The request and the answer's types are `#[non_exhaustive]`, so an API key (Open-Meteo's
    commercial servers) or another field can come without a breaking change.

**Consequences.** M5.2a meets the parent's two bullets for Open-Meteo. The forecast's own accuracy
is unmeasured: no flight in Open-Meteo weather is compared with a log, and no forecast with a
balloon. A refused request (HTTP 400) reports its status but not Open-Meteo's reason, since `Http`
drops a failed answer's body. The example `open_meteo_weather` flies Calisto in the recorded
weather.
