# NOAA forecasts: GFS and RAP

Fetch NOAA's GFS and RAP forecasts over a launch site and fly a rocket through the weather they
predict.

These are the United States weather service's own forecasts. NOAA's National Centers for
Environmental Prediction (NCEP) run two weather models whose output HPR Sim reads:

- The **Global Forecast System (GFS)** covers the whole Earth on a grid a quarter of a degree
  apart, about 28 km (17 mi).
- The **Rapid Refresh (RAP)** covers the contiguous United States and nearby parts of Canada and
  Mexico, on a grid 13 km (8.1 mi) apart. It's started again every hour.

The simulator asks NOAA's download server for a small piece of one forecast around the site. The
server is NOMADS (NOAA Operational Model Archive and Distribution System). The piece arrives as a
[GRIB2](glossary.md#grib2) file, the World Meteorological Organization's binary format for weather
on a grid. The simulator reads it with its own decoder.

The result is a [sounding](glossary.md#sounding): temperature, pressure, humidity and wind at a
column of heights. The column runs from the ground up to about 31 km (102,000 ft) in GFS or 16 km
(52,000 ft) in RAP. You can also download a whole GFS file and read it offline
([A whole GFS file](#a-whole-gfs-file)). A whole file's sounding goes up to about 79 km
(259,000 ft).

This page is for anyone who wants a flight in a named NOAA model's forecast. Open-Meteo's choice
of model is on [Launch-day weather](weather.md), and a weather balloon's measurement on
[Weather-balloon soundings](soundings.md).

> **How far to trust it.** A weather model's forecast, read as the model wrote it.
>
> - HPR Sim's decoder gives every value in the two recorded files that ecCodes, the European weather
>   center's reference decoder, gives. All 6,123 values are within 2.2e-16 of each other, relatively
>   (one rounding in the last binary digit). The profile gives back those values, interpolated to
>   the site, at every level it keeps. That's checked below.
> - A whole GFS file you download yourself reads too ([A whole GFS file](#a-whole-gfs-file)). Every
>   one of the 746,770,303 values in one such file is within 4.4e-16 of ecCodes', relatively. That
>   check was a script run once outside CI, and CI checks eight of the file's messages. The file's
>   profile at Spaceport America is the recorded cut's to 1.04e-7.
> - How good a forecast is depends on the model, and nothing here measures that. No forecast has
>   been compared with a weather balloon or a flight log.
> - The pad sits on the model's ground, which is smoothed. At Spaceport America it's
>   1,476 m (4,843 ft) in GFS and 1,429 m (4,688 ft) in RAP. Open-Meteo gives 1,400 m (4,593 ft),
>   as in [Launch-day weather's example](weather.md#an-example).
> - Fields packed as JPEG 2000 images, as in RAP's whole files, read too. Four public RAP fields,
>   182,443 points, match ecCodes in CI ([Files in JPEG 2000](#files-in-jpeg-2000)). No whole RAP
>   file has been tried.
> - The tests replay two recorded answers. The live connection to NOMADS isn't tested in CI.

Code: `hpr_net::nomads` ([API reference](api/hpr_net/nomads/index.html)), with the decoder in
`hpr_io::grib2` ([API reference](api/hpr_io/grib2/index.html)). It was written for
[the third weather increment](decisions-and-roadmap.md#m5-2c). Later increments extended it
[to whole GFS files](decisions-and-roadmap.md#m5-2d2) and
[to JPEG 2000](decisions-and-roadmap.md#m5-2d3). It needs the `net` feature of the `hpr` crate.
The choices are in three decision records:

- [GFS and RAP from NOMADS' grib filter, read by an in-house GRIB2 decoder](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0121-gfs-and-rap-from-nomads-grib-filter-read-by-an.md)
- [Complex packing, and a whole GFS file](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0123-complex-packing-and-a-whole-gfs-file.md)
- [JPEG 2000 packing](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0124-jpeg-2000-packing-through-hayro-jpeg2000.md)

## What the simulator asks for

A forecast comes from a *run*: the model starts from the weather observed at one hour, its
[cycle](glossary.md#forecast-run-cycle), and steps forward. Each *forecast hour* is the forecast
for that many hours after the cycle. Times are in UTC (universal time, the time at Greenwich) and
given in seconds since 1 January 1970 UTC (Unix time). Pressures are in hectopascals (hPa, 100 Pa;
sea-level pressure is about 1013 hPa). A *pressure level* is a height named by the pressure there.

| model | grid | runs start | forecast hours | pressure levels asked for |
|---|---|---|---|---|
| GFS | 0.25° of latitude and longitude, the whole Earth | every 6 hours (00, 06, 12, 18 UTC) | every hour to 120, then every third hour to 384 | 28, from 1000 to 10 hPa, about 31 km (102,000 ft) |
| RAP | 13 km (8.1 mi), on a [Lambert conformal](glossary.md#lambert-conformal-projection) map of the contiguous United States and nearby parts of Canada and Mexico (not Alaska or Hawaii) | every hour | every hour to 21; to 51 only from the 03, 09, 15 and 21 UTC runs | 37, from 1000 to 100 hPa every 25, about 16 km (52,000 ft) |

`NomadsRequest::new(latitude, longitude, model, cycle, forecast_hour)` says what to ask for, and
it can't fail. The request's `url()`, and so `nomads::fetch`, refuse a cycle the model doesn't run
or a forecast hour its run doesn't have. `NomadsModel::has_forecast_hour(cycle_hour, hour)` says
which hours a run has.

The simulator picks neither for you. To fly at 18 UTC, ask for GFS's 00 UTC run at hour 18, say,
or RAP's 12 UTC run at hour 6. A site outside the model's grid is refused once the answer arrives.

The request's address is NOMADS' *grib filter*. It's a web form that cuts chosen variables, levels
and a box of latitude and longitude out of a run's file. The simulator asks for a box 0.3° each
way around the site. The box holds 9 GFS or about 25 RAP [grid points](glossary.md#grid-point),
among them the four around the site. That's about 28 KB for GFS and 40 KB for RAP.

The box's edges are written to hundredths of a degree, worked out from the site rounded to
5 decimals, about 1 m (3.3 ft). Your program keeps the site in radians and turns it back into
degrees, which can change its last digits. Even so, a site given to 8 decimals or fewer asks for
the same box and finds its saved answer. The simulator asks for:

| variable | where |
|---|---|
| [geopotential height](glossary.md#geopotential-height) | the ground (the model's terrain) and each pressure level |
| pressure | the ground |
| temperature and relative humidity | 2 m (6.6 ft) above the ground and each pressure level |
| wind, as two components | 10 m (33 ft) above the ground and each pressure level |

GRIB2 fixes each variable's unit, so there are no units to check. The filter also sends the
ground's own (skin) temperature, since the request asks for temperature at the surface. The
simulator decodes it but doesn't use it. With the skin temperature, a cut holds 147 GFS or 192 RAP
[messages](glossary.md#grib2), one variable on one level each. A cut of more than 1,000 is refused
unread.

The answer goes through `hpr-net`'s cache ([Online data and the cache](online-data.md)).
`nomads::fetch(&client, &request, now)` takes the time now, in Unix seconds, to judge how fresh a
saved copy is. A run's files don't change once NOAA writes them, so a saved copy stays fresh for
30 days.

NOMADS keeps only recent runs. On 2026-09-30 its grib filters listed 10 days of GFS runs and 2 of
RAP. So a past launch needs a copy saved while NOMADS still had it. A run appears on NOMADS some
hours after its start time. NOMADS limits how often you can ask, so keep to that.

Offline mode answers from the disk only. Only an answer that decodes, makes a sounding, and is for
the run and hour asked for is saved. So a bad answer can't replace a good copy.

NCEP's forecasts are U.S. government works, free of copyright. Every answer carries the credit
"Forecast data from NOAA/NCEP (GFS, RAP), via NOMADS". Show it wherever you show the forecast.

## How the answer becomes a sounding

**Between grid points.** A model gives each value only at its grid points. The simulator takes the
four around the site and blends them *bilinearly*. Each point's weight grows as the site nears it,
in the grid's own rows and columns. The four weights add up to 1.

A cut is refused and not saved when its grid steps and projection aren't the model's. GFS's grid
has 0.25° steps, and RAP's has 13,545 m (44,439 ft) cells about 265° E.

**The ground** is at the model's terrain height at the site. It takes the surface pressure, the
2 m (6.6 ft) temperature and humidity, and the 10 m (33 ft) wind. Putting the 10 m (33 ft) wind at
the ground makes it the wind on the launch rail. The rail's wind doesn't jump to the lowest level's
wind above it ([a Loft lesson](decisions-and-roadmap.md#l6): a forecast profile that stepped at its
lowest level). This is what Open-Meteo's page does too.

**Levels below the ground are left out.** The models give every level everywhere, inventing values
beneath high ground. A level is kept only when its pressure is below the ground's and its height is
above the ground. A level with a value missing at any of the four grid points is left out too.

At Spaceport America, about 1,400 m (4,593 ft) up, six levels are underground in each model.
They're 1000 to 850 hPa in GFS and 1000 to 875 hPa in RAP. GFS keeps 22 of its 28 levels and RAP
31 of its 37. The profile lists every level it left out, and why.

**Heights.** GRIB2 gives heights in geopotential meters. The simulator converts them to heights
above sea level at the site's latitude. It uses the World Meteorological Organization's formula
(WMO-No. 8 eq. 12.16, as the [atmosphere page](physics/atmosphere.md) explains).

The model's terrain height is also given in geopotential meters and converted the same way. At
this latitude that adds about 2 m (6.6 ft): GFS's 1,474.4 gpm becomes 1,476.4 m (4,844 ft).
[Open-Meteo's elevation](weather.md#how-the-answer-becomes-a-sounding) is a height above sea level.
If the model's terrain were really one too, the ground would sit about 2 m (6.6 ft) too high.

**RAP's winds are turned to east and north.** RAP's map is a cone unrolled flat (a
[Lambert conformal](glossary.md#lambert-conformal-projection) projection). RAP gives each wind as
two components along its map's rows and columns, not east and north.

"Up" on that map points true north only along 95° W, one line of longitude. Elsewhere the lines of
longitude lean towards the map's center. So the map's "up" is turned from true north by an angle
that grows with the distance from 95° W:

$$
\begin{aligned}
\theta &= n\,(\lambda - \lambda_0), \\
n &= \sin 25^\circ, \\
\lambda_0 &= 265^\circ\ (95^\circ\,\mathrm{W})
\end{aligned}
$$

Here $\lambda$ is the site's longitude, and 25° N is where RAP's cone touches the Earth. At Spaceport
America, 106.97° W (253.03° E),

$$
\begin{aligned}
\theta &= 0.4226 \times (253.03^\circ - 265^\circ) \\
&= -5.06^\circ,
\end{aligned}
$$

so the map's "up" points 5.06° west of true north. The simulator turns the wind by that angle, with
$u$ and $v$ the components along the map's rows and columns:

$$
\begin{aligned}
\mathrm{east} &= u \cos\theta + v \sin\theta, \\
\mathrm{north} &= -u \sin\theta + v \cos\theta
\end{aligned}
$$

Left unturned, a 20 m/s (45 mph) wind would be about 1.8 m/s (4.0 mph) off sideways (20 m/s × sin
5.06°). The components are blended between grid points first and turned once, at the site. Across
one 13 km (8.1 mi) cell, $\theta$ changes by about 0.06°. GFS gives its winds east and north
already.

**Humidity** is taken as relative to liquid water, as for Open-Meteo. Whether NCEP's models give
it relative to ice at cold levels isn't settled here. If they do, the density there moves by under
0.1% ([Launch-day weather](weather.md#how-the-answer-becomes-a-sounding) explains the bound). A
humidity above 100% is kept in the level as given and taken as 100% in the sounding.

**Between levels** the sounding works as for any other: the temperature and humidity are linear,
and the pressure is hydrostatic. The wind is interpolated by its speed and direction. Pass
`WindInterpolation::Components` to interpolate its east and north parts instead, as
[RocketPy](glossary.md#rocketpy) does.

## An example

[`crates/hpr/examples/nomads_forecast.rs`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/crates/hpr/examples/nomads_forecast.rs)
fetches both models' forecasts for Spaceport America at 18 UTC (noon local) on 30 September 2026.
They're GFS's 00 UTC run at hour 18 and RAP's 12 UTC run at hour 6. It flies Calisto (one of the
[example rockets](glossary.md#example-rockets)) through each, without its parachutes. It makes
these calls:

1. `Client::new(transport, Cache::new(folder), Mode::Online)` sets up the fetching. A real
   program passes `hpr_net::Http::new()` as the *transport* (what fetches) and keeps its cache in
   `Cache::platform_dir()`. This one never uses the network: a stand-in transport answers with
   the two files recorded for the tests.
2. `NomadsRequest::new(latitude, longitude, NomadsModel::Gfs, cycle, 18)` says what to ask for
   (and `NomadsModel::Rap` for RAP), and `nomads::fetch(&client, &request, now)` fetches it and
   reads it at the site. Here it reads the two recorded forecasts, as a real program would fetch
   them.
3. `profile.sounding(WindInterpolation::SpeedDirection)` makes the sounding, and
   `sounding.wind()` its wind.
4. `hpr_sim::Environment::new(earth, sounding, wind)` puts both in a flight's environment.

It prints each model's ground and the standard levels 850, 700, 500, 250 and 100 hPa that lie
above it. Heights are in meters above sea level. The wind's direction is where it blows from,
clockwise from north. Then it prints where Calisto is at apogee: east and north of the pad,
negative for west and south.

Each flight starts on its own model's ground, and the standard-atmosphere flight starts on GFS's.
Run it from a copy of the repository with
`cargo run --example nomads_forecast -p fusionspace-hpr --features net`. It prints:

<!-- quote: crates/hpr/examples/nomads_forecast.output.txt -->
```text
Spaceport America, 2026-09-30 18 UTC
Forecast data from NOAA/NCEP (GFS, RAP), via NOMADS
freshness: Fetched
GFS: run of 00 UTC, hour 18: 22 levels kept, 6 left out
RAP: run of 12 UTC, hour 6: 31 levels kept, 6 left out
RAP's winds turned by -5.06° to east and north

GFS   level (hPa)   height (m)   temperature (°C)   humidity (%)   wind (m/s)   from (°)
              848         1476               19.3             42          5.5        242   the ground
              700         3077                2.8             90          7.3        262
              500         5720              -12.9             55         17.0        246
              250        10822              -32.7              1         45.8        230
              100        16741              -70.6             17         17.7        209

RAP   level (hPa)   height (m)   temperature (°C)   humidity (%)   wind (m/s)   from (°)
              854         1429               16.9             70          3.7        246   the ground
              850         1463               16.1             67          4.4        249
              700         3076                2.6             77          8.1        272
              500         5722              -12.0             39         20.5        242
              250        10822              -32.6              1         42.9        231
              100        16739              -70.8             10         13.0        211

Calisto to apogee      apogee (m above the pad)   east of the pad (m)   north (m)
GFS                                        2842.8                -253.0      -131.7
RAP                                        2841.3                -204.3       -83.9
standard, calm                             2826.9                  -5.1         0.0
```

`freshness: Fetched` means the answer came from the transport, not the cache. A second call within
30 days would say `Cached`. GFS has no 850 hPa row because its ground, at 848 hPa, is above that
level.

The two models agree closely aloft: at 500 hPa their heights are 2 m (6.6 ft) apart and their
temperatures 0.9 °C. Near the ground they differ more. GFS's ground is 47 m (154 ft) higher and
2.4 °C warmer. Its 10 m (33 ft) wind is 1.8 m/s (4.0 mph) stronger.

Calisto climbs 14–16 m (46–52 ft) higher in either forecast than in the standard atmosphere with no
wind. At apogee it's west-southwest of the pad, upwind. The wind at the ground blows from the
west-southwest (242° and 246°). A rocket just off the rail, still slow, turns into the wind
([weathercocking](glossary.md#weathercocking)) and flies that way.

GFS's stronger wind near the ground turns Calisto further into the wind. It ends 49 m (161 ft)
further west and 48 m (157 ft) further south than in RAP's forecast. How much of that comes from
the ground's wind and how much from the winds aloft isn't separated.

The calm flight's 5.1 m (17 ft) west is Earth's rotation, as on the
[Launch-day weather](weather.md#an-example) page. A climbing rocket is pushed west (the Coriolis
effect). Which model is nearer the air Calisto would have met isn't measured.

## Whole files offline

A whole downloaded GFS file reads with no network. Fields packed as JPEG 2000 images, as RAP's
whole files are, read too, though no whole RAP file has been tried.

### A whole GFS file

Instead of asking NOMADS for a cut, you can download a whole GFS file and read it offline. Do it,
for example, before driving out to a launch with no signal. NOMADS keeps the most recent runs, and
NOAA's open data bucket on Amazon's cloud older ones. A file holds one run's forecast for one hour
over the whole Earth, about 550 MB:

```text
https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/gfs.20260930/00/atmos/gfs.t00z.pgrb2.0p25.f018
https://noaa-gfs-bdp-pds.s3.amazonaws.com/gfs.20260930/00/atmos/gfs.t00z.pgrb2.0p25.f018
```

Here `20260930/00` and `t00z` are the run (00 UTC on 30 September 2026) and `f018` the forecast
hour. Only the 0.25° files (`pgrb2.0p25`) read. The smaller 0.5° and 1° files are refused, since
the simulator checks the file is on GFS's 0.25° grid. The [command line](cli.md#hpr-weather) reads
the file as it reads a saved cut, with the site you want:
`hpr weather gfs --latitude 32.99 --longitude -106.97 --cycle 2026-09-30T00Z --hour 18 --from
gfs.t00z.pgrb2.0p25.f018 --output profile.json`. It took about a third of a second on a Mac.

NCEP packs whole files more tightly than the grib filter's cuts, with GRIB2's *complex packing*.
The values are split into groups, each with its own smallest value and number of bits. Most fields
store the differences between neighboring values rather than the values (*spatial differencing*).

GRIB2 numbers these packings as templates 5.2 and 5.3. The simulator's decoder reads both. A whole
file also holds 743 [messages](glossary.md#grib2) where a cut holds 147:

- Totals and averages over a time span, such as the rain that fell in the last six hours. The
  simulator reads these, but the profile doesn't use them.
- Values for a layer between two heights, such as the humidity from the ground to mid-air. The
  simulator skips these, because they don't belong to one level.
- Every pressure level up to 0.01 hPa, so the profile goes on above 10 hPa, where a cut stops. In
  the profile the simulator writes from this file, the 0.01 hPa level is 79.2 km (259,800 ft) above
  sea level. The highest levels are near the top of the model, and nothing here checks how good its
  forecast is there.

The grid runs all the way round the Earth, so the simulator joins its last column to its first. A
site between 359.75° and 0° of longitude still has four grid points around it. That's checked by
`a_grid_round_the_earth_wraps_its_columns` in `crates/hpr-net/tests/nomads.rs`.

For that run and hour at Spaceport America, the whole file's profile has the cut's 22 levels. Each
of their values is within 1.04e-7 of the cut's, relatively, and 13 more levels sit above them. Why
they differ at all: NOMADS packs its cuts again in fewer bits. Read with ecCodes, without HPR Sim,
the two files already differ by up to 2.4e-6, relatively, at the cut's grid points.

### Files in JPEG 2000

Some of NCEP's files compress each field as a picture. The packed whole numbers become a grayscale
image, stored in the JPEG 2000 image format (GRIB2's template 5.40). RAP's whole pressure-level
files are packed this way. The simulator decodes them with
[`hayro-jpeg2000`](https://crates.io/crates/hayro-jpeg2000), a JPEG 2000 decoder written in Rust.
Then it turns each whole number into a value, as it does for the other packings.

A *lossless* image gives back exactly the numbers packed into it. A *lossy* one gives back
approximations. The simulator reads lossless images of up to 21 bits a value, coded the way NCEP
codes them. The four RAP fields below have 6 to 16 bits. Anything else is refused, naming what it
is: a lossy image, one with more bits, or one coded some other way.

The limit of 21 bits comes from the decoder's arithmetic. It works in 32-bit floats, which keep
whole numbers exact only up to 16,777,216. Its intermediate sums run about 8 times larger than the
values. A field with a bitmap is stored as one row of its values. The decoder takes at most 60,000
in a row, so such a field of more values is refused too.

A picture has to be decoded whole, so reading one point decodes the whole field. Read all the
values you need in one go
([`Field::values_at`](api/hpr_io/grib2/struct.Field.html#method.values_at)), as `hpr weather` does.
A whole RAP file through `hpr weather rap --from` hasn't been tried. No whole RAP file was
downloaded, only four of its fields ([How it is checked](#how-it-is-checked)).

## How it is checked

The tests in [`crates/hpr-net/tests/nomads.rs`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/crates/hpr-net/tests/nomads.rs)
use the two files the example reads. They were recorded unchanged from NOMADS' grib filter on
30 September 2026 for Spaceport America (32.99° N, 106.97° W). One is GFS's 00 UTC run at hour 18
(27,586 bytes). The other is RAP's 12 UTC run at hour 6 (40,026 bytes), and both are for 18 UTC.
No test uses the network.

The expected values come from [ecCodes](https://confluence.ecmwf.int/display/ECC) 2.49.0, the European Centre
for Medium-Range Weather Forecasts' decoder.
[`validation/oracles/grib2/eccodes_dump.py`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/oracles/grib2/eccodes_dump.py)
ran it on the same files, and its output is committed as
`crates/hpr-net/tests/fixtures/nomads-eccodes.json`. The tests read that file directly, not
through the decoder under test.

Each line gives what was measured, then the bound the test holds it to. All the measurements below
were made on macOS. CI runs the same tests with the same bounds on Linux and Windows.

- The cuts hold 147 GFS messages (9 grid points each) and 192 RAP messages (25 grid points each).
  Each names the same variable, level and times as ecCodes. All 6,123 values are within 2.2e-16 of
  ecCodes', relatively (bound 2.5e-16). That's one rounding: ecCodes multiplies by an inexact
  $10^{-D}$ where HPR Sim divides by an exact $10^{D}$. Decoding a value uses only basic
  arithmetic, which is the same on every platform.
- Every grid point's position is within 5.7e-14° of ecCodes' (bound 1e-12°, since other
  platforms' maths libraries can differ in the last digits). The four points' weights add up to 1,
  and the blend of their positions is the site, to 1e-5°.
- RAP's wind turn is $\sin 25^\circ \times (\lambda - 265^\circ)$, −5.06° at the site. The
  positions ecCodes gives two neighboring grid points show the map's rows bearing
  $90^\circ + \theta$ from north, to 2.2e-6° (bound 3e-6°). That pins the sign. GFS's winds aren't
  turned.
- The sounding is sampled at the ground and every kept level's height. It gives back ecCodes'
  values interpolated to the site. The pressure is within a relative 1e-13, the temperature within
  1e-11 K and the wind within 1e-11 m/s. Each level holds the humidity within 1e-14. The six
  underground levels of each model are the ones left out.
- The heights are geopotential meters. From 500 hPa up, each layer's thickness matches what the
  [hypsometric equation](glossary.md#hypsometric-equation) gives it, with the file's heights read
  as geopotential meters. On average the match is to −0.002% (GFS, 15 layers to 10 hPa) and −0.08%
  (RAP, 16 layers to 100 hPa). Read as meters above sea level, the layers would be 0.63% and 0.51%
  too thin. The test holds each average to those figures as rounded.
- The two models read alike for the same hour. Their 2 m (6.6 ft) temperatures are within 3 K of
  each other. Their 500 hPa heights are within 30 m (98 ft), and their 500 hPa winds within 5 m/s
  (11 mph). That's a guard against a misread file, not a check of either forecast.
- The request's address is the one recorded, so a replayed answer fills the cache. A second
  request is answered from the cache. Offline mode answers without the network 40 days later,
  [marked stale](online-data.md#what-it-promises).
- Wrong answers aren't saved. A RAP request answered with a GFS cut is refused and not saved. So is
  a cut of another run or hour than the one asked for. A GFS cut whose steps are changed to 0.5°
  still reads, but isn't GFS's grid, so it would be refused too.
- A humidity over 100% is kept in the level and taken as 100% in the sounding (a unit test).
- A level whose field has no value at one of the four points around the site is left out as
  missing data. A hostile cut of more than 1,000 fields is refused before it's read (a real one has
  147 or 192).
- A whole GFS file was checked too (the 00 UTC run of 30 September 2026 at hour 18, 550 MB, not
  committed):

  | check | where it runs | measured | bound |
  |---|---|---|---|
  | every one of its 746,770,303 values against ecCodes', relatively | a script, run once outside CI | 4.4e-16 | none |
  | its 24,642,017 grid points without a value are ecCodes' | the same script | all the same | all |
  | eight whole messages cut from it, one of each kind: which field each is, its points without a value, and every 997th value | CI | 4.4e-16 or less | 4.5e-16 |
  | the same eight messages: two sums over all their values | CI | within the bound | 1.35e-15 of the terms' sizes |
  | `hpr weather` writes the cut's profile from it, relatively | a test skipped in CI, which doesn't have the file | 1.04e-7 | 1.1e-7 |

  The script is
  [`validation/oracles/grib2/whole_file.py`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/oracles/grib2/whole_file.py).
  Its record, with ecCodes' survey of the file, is
  [`gfs-whole-file.json`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/oracles/grib2/gfs-whole-file.json).
  The sums catch a value decoded wrong anywhere in a message. A value off by a single packing step
  would move a sum at least 23,000 times more than the bound allows.
- Complex packing is checked in CI on the recorded GFS cut, packed again by ecCodes with complex
  packing and both orders of differencing
  ([`repack.py`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/oracles/grib2/repack.py)).
  The repacked cut goes through every check above. Its 1,323 values match ecCodes' to one rounding
  (bound 2.5e-16), and its profile matches at every level. From it, `hpr weather` writes a profile
  within 6.9e-8 of the cut's, relatively (bound 7e-8).
- JPEG 2000 is checked in CI on four fields cut unchanged from RAP's run of 00 UTC,
  30 September 2026. Two are the 500 hPa temperature on RAP's 13 km (8.1 mi) grid
  (151,987 points) and on a coarser one (10,152). The other two are the heights of cloud base and
  cloud top. They have a value only where there's cloud (6,824 and 434 points of 10,152).

  Their 182,443 points match ecCodes' reading: which points have no value, and the plain sum of
  every value. So do a sum weighted by each point's position, which catches a value in the wrong
  place, and every 13th value alone. The test is
  `rap_messages_in_jpeg2000_decode_to_eccodes_values` in `crates/hpr-io/tests/grib2_gfs.rs`.
  `validation/oracles/grib2/whole_file.py cut-rap` writes ecCodes' reading
  ([the decision record on JPEG 2000 packing](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0124-jpeg-2000-packing-through-hayro-jpeg2000.md)).
- A damaged JPEG 2000 image is refused, never a crash. `damaged_jpeg2000_never_panics` in
  `crates/hpr-io/src/grib2/tests.rs` changes bytes of one at random, 256 times in CI. Another test
  cuts one short at every length.
- The simulator refuses a site outside the file's grid and text that isn't GRIB2. It also refuses
  a cycle the model doesn't run and a forecast hour its run doesn't have. The decoder's own tests
  build small files by hand. They pin the unpacking formula, bitmaps (masks of grid points with no
  value), both kinds of grid and complex packing's groups. They also pin missing values, both
  orders of differencing, and refusals of every other kind of file by name.

## What it leaves out

- A lossy JPEG 2000 image, one of more than 21 bits a value, or one coded otherwise than NCEP codes
  is refused by name. So is a bitmap field of more than 60,000 values
  ([Files in JPEG 2000](#files-in-jpeg-2000)).
- Whole RAP files' packing is read, but a whole file hasn't been run through `hpr weather`.
- GFS's coarser 0.5° and 1° files are refused: the simulator checks for the 0.25° grid.
- Nothing here checks a forecast's accuracy against the weather that came.
- Each request is for one forecast hour, with no interpolation between hours. You pick the run and
  the hour closest to your launch.
- RAP stops at 100 hPa, about 16 km (52,000 ft), and GFS as asked for here at 10 hPa, about 31 km
  (102,000 ft). A whole GFS file goes on to 0.01 hPa, about 79 km (259,000 ft). The layer thickness
  check above stops at 10 hPa.
- Above the top level, the sounding continues as the
  [standard atmosphere](glossary.md#standard-atmosphere), shifted to pass through the top level's
  temperature and pressure. It keeps the top level's share of water vapor, capped where the air
  saturates. The wind holds the top level's wind. Both are marked as extrapolated. A flight above
  16 km (52,000 ft) in RAP's forecast is flying on that guess.
- A kept level is blended from the four grid points even where one of them has that level
  underground. In the RAP cut, 850 hPa takes 13% of its weight from a point whose ground is at
  845.8 hPa. The effect here is about 0.02 K, and larger in steep terrain.
- The pad is placed on the model's smoothed terrain, not the site's real height. The simulator
  doesn't shift the profile to the real ground.
- Whether NCEP gives humidity over ice at cold levels is unsettled here. The difference moves the
  density by under 0.1%.
- The live connection to NOMADS isn't tested in CI. Nothing tells you which runs NOMADS still holds
  before you ask.
- The command line fetches them with [`hpr weather gfs` and `rap`](cli.md#hpr-weather), but
  `hpr sim` doesn't fly them yet (issue [#265](https://github.com/nrdptel/fusionspace-eridanus/issues/265)).
  The Python package doesn't fetch NOAA's forecasts.
