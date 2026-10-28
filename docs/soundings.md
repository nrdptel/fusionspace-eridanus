# Weather-balloon soundings

Fetch a weather balloon's measurements from the University of Wyoming's archive and fly a rocket
through the air they recorded.

The [University of Wyoming's archive](https://weather.uwyo.edu/upperair/sounding.shtml) serves
weather balloons' measurements. A balloon carries a [radiosonde](glossary.md#radiosonde), which
measures the air on the way up, from the ground to 30 km (98,000 ft) or more. It records a
[sounding](glossary.md#sounding): temperature, pressure, humidity and wind at a column of heights.
Hundreds of stations release one at 00 and 12 UTC (universal time, the time at Greenwich) every
day.

This page is for anyone who wants a flight in air measured near a launch. The alternatives are a
forecast ([Launch-day weather](weather.md), [NOAA forecasts: GFS and RAP](nomads.md)) or the
[standard atmosphere](glossary.md#standard-atmosphere).

> **How far to trust it.** A weather balloon's measurement, passed on as the archive gives it.
>
> - HPR Sim turns the archive's answer into a profile. The profile gives back the pressure,
>   temperature, humidity and wind of every level it keeps, to rounding error. That's checked on
>   three recorded soundings, below.
> - The measurement holds only at its station and time. The nearest station can be
>   100 km (62 mi) or more from a launch site. Santa Teresa, below, is about 130 km (81 mi) from
>   Spaceport America. The balloon goes up hours before or after the flight; nothing here measures that effect.
> - The tests replay recorded answers. The live connection to the archive isn't tested in CI.

What the checks catch and miss:

- Each row's height is checked against the row before it. That catches a gross error in a
  pressure or height (57 hPa recorded for 557). More than 10 bad rows in a row refuse the
  answer, and so does a ground that the rows after it agree is wrong. That can refuse a good
  ground too, when the first few rows after it are a little off.
- A wrong wind, humidity or temperature isn't caught. On layers under about 100 m (328 ft)
  thick, the check allows any temperature from −150 to 80 °C.
- The archive serves two versions of most soundings, and they can disagree near the ground. In
  the example below, Calisto is 402 m (1,319 ft) from the pad at apogee in one and
  563 m (1,847 ft) in the other.

Code: `hpr_net::wyoming` ([API reference](api/hpr_net/wyoming/index.html)), written for
[the second weather increment](decisions-and-roadmap.md#m5-2b). It needs the `net` feature of the
`hpr` crate. The choices are in
[the decision record](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0120-university-of-wyoming-soundings.md).

## What the simulator asks for

A request names a station and the sounding's hour. The station is its World Meteorological
Organization (WMO) number, such as station 72364 for Santa Teresa, New Mexico; the archive's page has a
map of them. `WyomingRequest::latest_before(station, launch_time)` picks the last 00 or 12 UTC
sounding at or before a launch. Times are in seconds since 1 January 1970 UTC (Unix time). The
balloon goes up about an hour before that hour.

The simulator asks for the archive's comma-separated text, one row per level, and reads these
columns. Pressures are in hectopascals (hPa, 100 Pa; sea-level pressure is about 1,013 hPa).

| column | read as |
|---|---|
| time | when the balloon was released, from the first row |
| latitude, longitude | where it was released, from the first row |
| pressure, in hPa | the level's pressure |
| geopotential height, in m | its [geopotential height](glossary.md#geopotential-height) above sea level |
| temperature, in °C | its temperature |
| relative humidity, in % | its humidity, relative to liquid water (the file gives it relative to ice too) |
| wind direction, in degrees, and wind speed, in m/s | the wind, and where it blows from, clockwise from north |

The units are in the column names, and the simulator refuses a file with any other. The archive
serves two versions of most soundings:

| version | what it is | rows |
|---|---|---|
| Coded message, the default | the message stations have sent for decades (WMO's TEMP code, FM 35) | about 200 |
| BUFR file | the station's newer digital file (WMO's Binary Universal Form for the Representation of meteorological data), a row every second of the climb | about 6,000 |

The coded message holds the standard pressure levels (850, 700, 500 hPa and so on). It also holds
the *significant* levels between them, where the temperature or wind changes its trend.

The answer goes through `hpr-net`'s cache ([Online data and the cache](online-data.md)).
`wyoming::fetch(&client, &request, now)` takes the time now, in Unix seconds, to judge how fresh a
saved copy is. For a day after its hour the archive's copy of a sounding can still fill in, as a
station's later messages arrive:

| the sounding's age | a saved copy stays fresh for |
|---|---|
| under a day | an hour |
| over a day | 30 days, if it was saved after the sounding's first day; an earlier copy is fetched again |

Offline mode answers from the disk only, however old the copy. An answer the simulator can't read
is never saved, so it can't replace a good copy. A sounding the archive doesn't have comes back as
an HTTP error: 404, or 400 for a BUFR file the station doesn't send. The request reports that
error. No test covers a missing sounding.

The archive states no terms of use. Show the credit "Sounding from the University of Wyoming's
radiosonde archive" wherever you show the sounding. Every answer carries it.

## How the answer becomes a sounding

**Heights.** The file gives geopotential meters. The simulator converts them to heights above sea
level at the first row's latitude. It uses the WMO's formula (WMO-No. 8, its *Guide to Instruments
and Methods of Observation*, eqs. 12.15 and 12.16, as the [atmosphere page](physics/atmosphere.md)
explains).

The balloon drifts as it climbs. Converting at the latitude it reached instead would move a
height. At 10 km (33,000 ft) the move is about 0.8 m (2.6 ft) per degree of drift. At
30 km (98,000 ft) it's about 2.5 m (8.2 ft) per degree. The example's balloon drifted 0.06°.

**Why geopotential.** Air pressure falls with height at a rate set by the air's temperature and
humidity. The [hypsometric equation](glossary.md#hypsometric-equation) turns that into how thick
each layer between two pressures must be, in geopotential meters:

$$
\text{thickness} = \frac{R_d \bar{T}_v}{g_0} \ln\frac{p_{\mathrm{bottom}}}{p_{\mathrm{top}}}
$$

Here $p_{\mathrm{bottom}}$ and $p_{\mathrm{top}}$ are the pressures at the layer's bottom and top.
$R_d$ is dry air's gas constant (287.05 J/(kg·K)), and $g_0$ is standard gravity
(9.80665 m/s²). $\bar{T}_v$ is the mean of the two rows'
[virtual temperatures](glossary.md#virtual-temperature). A virtual temperature is the
temperature, raised a little for the humidity (WMO-No. 8, eqs. 12.17 and 12.18).

In the three recorded soundings, take the 13 layers between the standard levels from 850 to
10 hPa. The recorded thicknesses match the equation to 0.01% to 0.03% on average. Single layers
are off by 0.05% to 0.13% on average, either way. Read as meters above sea level instead, the
layers would be 0.51% to 0.60% too thin on average.

So they're geopotential meters, as the column says.

**Each row is checked against the row before it.** Take a row with a pressure, height and
temperature. It *fits* the row before it when its height above that row is the layer's
thickness, give or take an *allowance*. The allowance is 5% of the thickness, plus what rounding
the two pressures can move it, plus 30 m (98 ft). The pressures are rounded to 1 hPa in the coded
message at 100 hPa or more, and to 0.1 hPa elsewhere. For example, from the coded message's row
at 557 hPa to the next at 549 hPa:

| | value |
|---|---|
| virtual temperatures | 272.24 K and 271.42 K (−1.7 °C and −2.5 °C, 79% and 81% humidity) |
| $R_d \bar{T}_v / g_0$ | 7,956.8 m (26,105 ft) |
| thickness | 7,956.8 m × ln(557/549) = 115.1 m |
| recorded | 5,151 m − 5,035 m = 116 m, a miss of 0.9 m (3.0 ft) |
| allowance | 5% of 115.1 m + 14.4 m (rounding each pressure by 0.5 hPa) + 30 m = 50.1 m |

**Rows are kept from a chain.** The simulator looks for the longest *chain* of rows from the ground
in which each row fits the one before it in the chain. The chain may pass by up to 10 rows at a
time, and the rows it passes by are left out. Of two chains equally long, the simulator takes the
one whose layers fit more closely. Closer means a smaller total of each layer's miss as a share of
its allowance. A miss of 25 m (82 ft) with an allowance of 50 m (164 ft) is a share of 0.5.

Why a chain: every row kept must also lie above the last row kept, so one bad row kept can hide
the good ones. On that rule alone, a 557 hPa row with a digit lost, 57 hPa at 5,035 m (16,519 ft),
would be kept. Every good row up to 57 hPa, about 20 km (66,000 ft), would then lie below it and
be dropped.

To fit, the bad row would have to be 18.6 km (61,000 ft) above the row before it, at 570 hPa. It's
183 m (600 ft) above that row. So the chain passes it by, from 570 hPa to 549 hPa, and it's left
out. A chain through two or three bad rows that fit each other would have to pass by more good
rows. So they're passed by the same way.

A row with no wind or humidity is left out of the profile but can still be in the chain. So the
next row is checked across a thin layer, not a thick one. A row with no temperature can't be
checked. The chain steps over it without counting it, and it's left out as missing a value.

In the three recordings every row fits the one before it, missing by at most 1 m (3.3 ft) beyond
rounding. So the 30 m (98 ft) and the 5% are margin, and the check is for gross errors, not small
ones:

- The 30 m (98 ft) leaves room for the coded message's heights from 500 hPa up, which are rounded
  to 10 m (33 ft). A height 30 m (98 ft) off misplaces its level by about as much as a 0.33% to
  0.55% pressure error. The range comes from the air's *scale height*, 5.5 km (18,000 ft) to
  9 km (30,000 ft). That's the climb over which pressure falls by a factor of e.
- The 5% is a judgment, not a measurement: no row in the recordings needs any of it. It leaves
  room for a layer whose inner rows have no temperature, where the mean of its two ends'
  temperatures gives its thickness less well.

Then the simulator keeps:

- **The ground, the first row**: the pressure, temperature, humidity and wind at the station
  when the balloon was released. A first row with a value missing or impossible refuses the
  answer.
- **One row of each run of rows in the chain with the same pressure, the middle one.** The BUFR file
  gives pressures to 0.1 hPa. High up, the balloon climbs tens of meters while the pressure falls
  that much, so runs of rows share one pressure. The rounded value is the pressure at about the
  middle of its run. In the example, 1,931 of the BUFR file's 5,851 rows are the other rows of
  such runs; the coded message has none.
- **Each such row that lies above the last row kept**, higher and at a lower pressure. Rows that
  fall or stay at one height, a balloon coming down, fit but don't lie above, and are left out
  however many.

It leaves out:

- **A row the chain passes by**.
- **A row with a value missing or impossible**: the last row often has no wind. An impossible
  value is any of these:
  - a pressure below 0.1 hPa or above 1,200 hPa;
  - a temperature outside −150 to 80 °C;
  - a height outside −1 km (−3,281 ft) to 60 km (196,850 ft);
  - a wind speed below zero or above 300 m/s (671 mph);
  - a humidity below zero, or a direction beyond 360°.

The profile lists every row it left out, and why. Two things refuse the answer:

- **A ground the rows after it disagree with.** No row before the ground checks it. So HPR Sim also
  looks for chains each starting at one of the next 11 rows with a pressure, height and
  temperature within the bounds. If one of them beats every chain from the ground, the ground is
  taken as wrong.

  Bad rows right after a good ground can do the same. They miss the ground but fit the rows above
  them, within the allowance. They beat the ground when the chain through them is longer, or as
  long and closer. The chain through them is longer when it passes by fewer good rows than it
  holds bad ones, less one.

  That happens in narrow bands of error, just past the ground's allowance. In the BUFR file, whose
  first layers are about 8 m (26 ft) thick, two rows 31 m (102 ft) high refuse the answer. Two
  rows 20 m (66 ft) high don't, since they fit the ground and are kept. Nor do two
  35 m (115 ft) high, which fit nothing but each other and are passed by.

  In the winter coded message, three rows 45 m (148 ft) high refuse the answer. So do 11 bad rows
  that fit each other, however far off. HPR Sim can't tell these from a bad ground. Rows grossly
  off, 850 m (2,789 ft) say, fit nothing above them and are passed by, up to 10.
- **More than 10 rows after the chain's end.** The chain can pass by only 10 rows at a time, so
  11 bad rows in a row end it. HPR Sim takes that to mean the chain's end is wrong, or all 11 are.
  A block of heights 1 km (3,281 ft) off is one such case.

  A long run of rows with no temperature can also refuse the answer. The layer across the run is
  then too thick for its two ends' temperatures to give. In a BUFR file 10 rows are about 10 s of
  the climb, some 50 m (164 ft). In a coded message they can span kilometers.

A refused answer names the lines and isn't cached. The other version, or the sounding 12 hours
earlier, may be usable instead.

In the profile:

- **Humidity above 100%**, which radiosondes can report in cloud, is kept in the level as
  recorded and taken as 100%.
- **Between levels** the sounding works as for any other. The temperature and humidity are
  linear, and the pressure is hydrostatic (as the [atmosphere page](physics/atmosphere.md)
  explains). The wind is interpolated by its speed and direction. Pass
  `WindInterpolation::Components` to interpolate its east and north parts instead, as
  [RocketPy](glossary.md#rocketpy) does. Above the top level the standard atmosphere continues,
  and the air is marked extrapolated.

## An example

[`crates/hpr/examples/wyoming_sounding.rs`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/crates/hpr/examples/wyoming_sounding.rs)
fetches the Santa Teresa sounding a launch at 15:30 UTC on 21 June 2025 would have had, in both
versions. It flies Calisto (one of the [example rockets](glossary.md#example-rockets)) from the
station without its parachutes. It makes these calls:

1. `Client::new(transport, Cache::new(folder), Mode::Online)` sets up the fetching. A real
   program passes `hpr_net::Http::new()` as the *transport* (what fetches) and keeps its cache in
   `Cache::platform_dir()`. This one never uses the network: a stand-in transport answers with
   the two versions recorded for the tests.
2. `WyomingRequest::latest_before("72364", launch_time)` picks the 12 UTC sounding, and
   `wyoming::fetch(&client, &request, now)` fetches and reads it. Setting the request's `version`
   to `WyomingVersion::Bufr` asks for the BUFR file instead.
3. `sounding.sounding(WindInterpolation::SpeedDirection)` makes the profile, and
   `profile.wind()` its wind.
4. `hpr_sim::Environment::new(earth, profile, wind)` puts both in a flight's environment.

It prints the ground and five standard levels, with heights in meters above sea level, and each
version's second row. Then it prints the air at the pad and above it, beside the standard
atmosphere. It also prints where Calisto is at apogee: east and north of the pad, negative for
west and south.

Each flight starts on its own sounding's ground. The third flight drops the BUFR file's rows
between its ground and 1,438 geopotential meters, the height of the coded message's second row.
Run it from a copy of the repository with
`cargo run --example wyoming_sounding -p fusionspace-hpr --features net`. It prints:

<!-- quote: crates/hpr/examples/wyoming_sounding.output.txt -->
```text
Santa Teresa, New Mexico (72364), 2025-06-21 12 UTC
Sounding from the University of Wyoming's radiosonde archive
freshness: Fetched
released 58 minutes before 12 UTC
coded message: 227 levels kept, 1 left out
BUFR file: 3920 levels kept, 1931 left out

level (hPa)   height (m)   temperature (°C)   humidity (%)   wind (m/s)   from (°)
        872         1254               28.4             31          5.7        265   the ground
        850         1482               26.6             32         11.8        270
        700         3165               14.6             36          2.6        215
        500         5903               -5.3             63          8.2        180
        250        11002              -40.1              7         13.9        260
        100        16724              -73.1             13         13.9        255

second row       above the ground (m)   wind (m/s)   from (°)
coded message                     186         10.7        269
BUFR file                           8         11.1        264

height above the pad (m)   pressure (hPa)   temperature (°C)   density (kg/m³)
     0 sounding                     872.0               28.4            1.0022
     0 standard                     871.4                6.9            1.0842
  1000 sounding                     778.5               22.3            0.9151
  1000 standard                     770.3                0.4            0.9811
  3000 sounding                     613.5                4.9            0.7664
  3000 standard                     596.5              -12.6            0.7977

Calisto to apogee      apogee (m above the pad)   east of the pad (m)   north (m)
coded message                              2848.1                -401.4       -23.4
BUFR file                                  2822.5                -561.3       -48.0
BUFR above 1,438 m                         2845.9                -419.1       -19.1
standard, calm                             2811.0                  -5.1         0.0
```

`freshness: Fetched` means the answer came from the transport, not the cache. That early
morning (6 a.m. local) was 21.5 °C warmer at the station than the standard atmosphere. The air
was 7.6% less dense at the ground and 3.9% less dense 3 km (9,843 ft) up. Calisto climbs 1.3%
higher in it than in the standard atmosphere with no wind. The thinner air and the wind both play
a part.

At apogee Calisto is about 400 m (1,312 ft) **west** of the pad, upwind. The wind blows from the
west. A rocket just off the rail, still slow, turns into the wind
([weathercocking](glossary.md#weathercocking)) and flies that way. The calm flight's
5.1 m (17 ft) west is Earth's rotation. A climbing rocket is pushed west (the Coriolis effect), as
on the [Launch-day weather](weather.md#an-example) page.

The two versions put Calisto's apogee 26 m (85 ft) apart, and the BUFR flight 160 m (525 ft)
further west. Most of that is the first 186 m (610 ft) of the climb. Both start from the station's
wind at the ground, 5.7 m/s (13 mph). The coded message's second row, 186 m (610 ft) up, is
10.7 m/s (24 mph) from the west. So its wind grows steadily over that climb.

In the BUFR file the wind is already 11.1 m/s (25 mph) 8 m (26 ft) above the ground. That's where
the rocket is slowest and turns into the wind the most. Without its rows below 1,438 geopotential
meters, the BUFR flight's apogee is 419 m (1,375 ft) west. That's within 18 m (59 ft) of the coded
message's. This flight peaks at 2,846 m (9,337 ft), within 2.2 m (7.2 ft) of the coded message's.

Which version is nearer the truth isn't known.

## How it is checked

The tests in [`crates/hpr-net/tests/wyoming.rs`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/crates/hpr-net/tests/wyoming.rs)
use three answers recorded from the archive. Two are Santa Teresa on 21 June 2025 at 12 UTC, in
both versions. The third is Salt Lake City on 15 January 2025 at 12 UTC, a winter sounding. No
test uses the network.

- The sounding is sampled at every kept row's height. It gives back the recorded pressure to a
  relative 1e-12, the temperature to 1e-9 K, the humidity to 1e-15 and the wind to
  1e-9 m/s. The test reads the expected values from the recording itself, not through the code
  under test, and applies the rules above on its own.
- The rows left out are exactly those the rules leave out. They're the last row of each coded
  message (no wind), and 1,931 rows of the BUFR file (the other rows of a run). The coded messages
  keep 227 and 241 rows, the BUFR file 3,920.
- Every BUFR row, kept or not, lies within 0.08 hPa of the profile's pressure at its height. The
  worst is 0.071 hPa. Keeping the first row of each run instead would miss by 0.093 hPa.
- The ground's wind is 5.7 m/s (13 mph) from 265°. It's checked against its east and north parts
  worked out by hand: 5.678 m/s (12.70 mph) east and 0.497 m/s (1.11 mph) north.
- An edited file drops each of these rows:
  - a row with a missing temperature;
  - one with a humidity below zero, a temperature of −999 °C, a negative wind speed or pressure,
    or a direction of 361°;
  - one that fits but lies below the last row kept (and not merely the row before);
  - and a row at the ground's pressure.

  The rows left out are listed in line order. A humidity of 103% is kept and taken as 100%, and a
  wind from 360° reads as from north.
- In the three recordings, every row with a pressure, height and temperature fits the one before
  it. Each misses by at most 1 m (3.3 ft) beyond rounding. The most is 0 m (0 ft) in the coded
  messages and 0.97 m (3.2 ft) in the BUFR file.

  The test works out the thickness itself, with the file's mixing ratio (grams of water vapor per
  kilogram of dry air) for the humidity. Unit tests pin the check, and the worked example above,
  to thicknesses and allowances worked out by hand. They test 0.01 m (0.03 ft) either side of the
  edge.
- Rows the chain passes by, with the rest of the answer kept:
  - A pressure missing a digit, 57 hPa at 5 km (16,000 ft); the profile is the recording's without
    the row.
  - A height raised 850 m (2,789 ft), and a height lowered to 4,000 m (13,123 ft).
  - Two bad rows in a row, and two blocks of six raised rows with a good row between them.
  - A bad row after which the balloon bursts and falls back.
  - A row whose vapor pressure would exceed the air's pressure (16 hPa at 30 °C, saturated).
  - A row that barely fits the row before it, alone: 854 hPa 50 m (164 ft) low and 101 hPa
    95 m (312 ft) high. Also a BUFR row at 112.8 hPa 35 m (115 ft) high. The next row misses it,
    and a chain through it isn't longer and fits less closely. The rows before and after it are
    kept.
  - A row that misses the good row before it but barely fits the one below that, alone:
    808 hPa raised 50 m (164 ft). Also a BUFR row at 244.2 hPa raised 35 m (115 ft). A chain could
    skip the good row to reach it. But the chain through the good row is longer, or as long and
    closer.
  - A BUFR row at 150.6 hPa raised 50 m (164 ft), whose pressure is rounded to 0.1 hPa. In the
    coded message, 549 hPa raised 45 m (148 ft) fits within its pressures' 1 hPa rounding and is
    kept.
  - Blocks of bad rows that fit each other but not the good rows beside them, which are kept. The
    blocks are 820 and 808 hPa raised 50 m (164 ft), and 101, 100 and 99 hPa raised
    100 m (328 ft). In the BUFR file they're rows at 112.8 and 112.7 hPa raised 50 m (164 ft).
    They're also two BUFR rows at 13.9 hPa lowered 50 m (164 ft), and two at 10.1 hPa lowered
    60 m (197 ft). Each of those two pairs is inside a run sharing one pressure.
  - The two or three rows after the ground raised 850 m (2,789 ft); the ground and the rest are
    kept.
  - A row inserted at 870 hPa, recorded 65 m (213 ft) above the ground. The air's thickness puts
    it about 20 m (66 ft) up. It misses the ground, but the next row fits the ground. A chain
    starting at the inserted row fits less closely. So the row is left out and the answer isn't
    refused.
- Rows kept or left out as they fit:
  - With no wind, or no humidity, from 250 to 55 hPa (73 rows), every row after the gap fits and
    is kept. Checked across the gap instead, the next row would not fit.
  - Rows falling or staying at one height at the end are left out without refusing the answer.
    When a row that fits and lies above follows them, the rows are left out and the row is kept.
    Of rows crossing back and forth over the last row kept, the first above it is kept.
  - A BUFR run of three rows at one pressure whose middle is raised 100 m (328 ft) keeps its first
    row.
  - A row at 0.1 hPa and −150 °C, 55.6 km (182,400 ft) up, fits the row below it and is kept.
- Answers refused:
  - A block of 10 rows raised 1 km (3,281 ft) is left out; 11 refuse the answer.
  - In the coded message, a ground at 87.2 hPa (872 with its decimal point misplaced). Also one at
    125 m (410 ft), where 1,252 m (4,108 ft) lost a digit, and one 8 hPa off. Also one
    52 m (171 ft) or 80 m (262 ft) high. And a BUFR ground 34 m (112 ft) high, whose first layer
    is 8 m (26 ft) thick.
  - A ground at 1,200 hPa, at −1,000 m (−3,281 ft) or at −150 °C. These are within the bounds, but
    no row above fits them.
  - In the coded message, the first layer is 186 m (610 ft) thick. A ground 40 m (131 ft) high
    there, or at 80 °C, fits it and is kept with every row.
  - With only one row after a bad ground, the ground is kept and the row left out. A row with no
    temperature right after a bad ground isn't counted: the refusal names the row after it.
  - Rows after a good ground that beat it. In the BUFR file they're two rows raised
    31 m (102 ft), and four lowered 35 m (115 ft). In the winter coded message they're three
    raised 45 m (148 ft), and in the other, four raised 50 m (164 ft). Eleven rows raised
    1 km (3,281 ft) beat it too.
  - Two BUFR rows raised 20 m (66 ft) fit the ground and are kept, leaving out the three good rows
    no higher than them. Two BUFR rows raised 35 m (115 ft), and 10 coded-message rows raised
    1 km (3,281 ft), are passed by. A chain without the ground may start only at the 11 rows after
    it. After 11 rows that fit nothing, the answer is refused for the gap.
- A row raised within its allowance is kept, and leaves out the good rows just above it, which
  now lie below it. The tests raise one row, or a block of two or three, by 20 m (66 ft) or
  35 m (115 ft). They raise or lower it by 50 m (164 ft) or 100 m (328 ft). None of these refuses
  an answer, and a coded message loses at most 2 other levels.

  The BUFR file is sampled at every 150th row, to keep the test under a minute. It's also sampled
  where a sweep of every row, run once and not in the tests, found the worst. In the BUFR file a
  change loses at most 8 other rows for one row. A block loses at most 10, about 50 m (164 ft) of
  the climb. Lowering one row, or changing its pressure by 3%, loses no other.
- Values just past the bounds above are left out. Unit tests keep values at each bound. At the top
  row, temperatures of −150 and 80 °C and a wind of 300 m/s (671 mph) are kept and make a
  profile.
- The request's address is the one recorded, so a replayed answer fills the cache. A second
  request is answered from the cache, and offline mode answers without the network. A copy saved
  while the sounding was young is fetched again once it has settled.
- An answer that isn't the archive's text (such as an error page) isn't saved. With no earlier
  copy the request fails. With one, the earlier copy comes back, marked stale.
- Text with no header, a missing or doubled column, or a unit other than the one required is
  refused. So is a row with too few or too many fields, or a field that isn't a number. So is a
  first row with no date, or with a value missing. A header or a row of a million commas, and more
  than 100,000 rows, are refused without being read further.
- The heights are geopotential, by the hypsometric check above. The test holds each average under
  0.1% as geopotential meters, and more than 0.4% too thin as meters above sea level.

## What it leaves out

- How far a station's sounding is from the air over a launch site isn't measured, in distance or
  in time.
- Which version is nearer the truth near the ground, where they differ most, isn't known. A
  balloon's first seconds of drift aren't a steady wind either.
- A level with any value missing is left out whole, even when its other values are good.
- The check catches a gross error in a row's pressure or height. It keeps:
  - A wrong wind or humidity. The check doesn't use the wind, and humidity moves the
    thickness by only a few percent. For saturated air at 30 °C and sea-level pressure, it's 1.6%.
    Only a wind below zero or above 300 m/s (671 mph) is caught.
  - A wrong temperature that keeps the thickness within the allowance, and a height error within
    it. On the example's layer that's 50 m (164 ft), more on thicker ones.
  - A row whose pressure and height are both wrong yet fit each other.
  - A wrong ground that fits the row after it, or has only one row after it. In the coded message
    a ground 40 m (131 ft) high, or at 80 °C instead of 28.4 °C, fits.
  - Bad rows that fit their neighbors: a block of them can be kept, and good rows beside them
    passed by instead. No more good rows are passed by than the block holds, and fewer unless the
    block fits more closely. In the coded message, 683 and 673 hPa both raised 50 m (164 ft) are
    kept, and 664 hPa is left out.

    High in a BUFR file, rounding the pressure to 0.1 hPa widens the allowance. There four rows at
    13.9 hPa lowered 50 m (164 ft) are kept, and the two good rows at 14.0 hPa before them left
    out.
- A good ground followed by bad rows that fit the rows above them is refused, as if the ground
  were wrong. That happens when the chain through them wins.
- A row, or a short block, kept a little too high leaves out the good rows just above it. In the
  tests that's at most 2 other levels of a coded message and 10 other rows of a BUFR file (8 for
  one row).
- Only the archive's comma-separated text is read, not its other formats, and there's no list of
  stations to search by place.
- Only soundings from U.S. stations, which are U.S. government works, are committed as test data
  ([the decision record](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0120-university-of-wyoming-soundings.md)).
- The Python package doesn't fetch soundings. The command line fetches them with
  [`hpr weather wyoming`](cli.md#hpr-weather).
