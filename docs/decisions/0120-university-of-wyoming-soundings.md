# ADR-120: University of Wyoming soundings (2026-09-30)

- **Status:** accepted
- **Summary:** M5.2b: University of Wyoming soundings from its CSV, FM 35 by default or BUFR, the first row as the ground, the middle of each same-pressure run, rows off the longest chain fitting the hypsometric thickness from the row before, or not above the last kept, dropped; freshness by age; U.S. stations' soundings committed as fixtures

**Context.** M5.2b's *done when*: a recorded sounding's profile reproduces its pressure, temperature
and wind at every level, from recorded-fixture tests. The archive's current interface
(`/wsgi/sounding`) serves each sounding as an HTML page (`TEXT:LIST`) or as comma-separated text
(`TEXT:CSV`), and in two versions (`src=FM35`, the coded TEMP message; `src=BUFR`, a row a second);
with no `src` it picks one. The site states no terms of use (its pages were read on 2026-09-30), and
[rule 4](../../CONTRIBUTING.md#4-private-data-stays-private) keeps third-party data of unclear
licence out of the repository.

**Decision.**

1. **`hpr_net::wyoming`**, mirroring `open_meteo`: a `WyomingRequest` (station, nominal hour,
   `version`, optional endpoint) builds the URL, always naming `src` so the cache key is exact;
   `WyomingSounding::parse` reads the answer; `fetch` goes through `Client::fetch_checked` with
   parse-then-sounding as the check. The station must be 1 to 16 ASCII letters and digits (no
   escaping needed), the time a whole hour from 1970 to 9999, and an endpoint non-empty with no
   `?` or `#`. `latest_before` picks the last 00 or 12 UTC, saturating at the ends of `i64`.
2. **Freshness follows the sounding's age.** The archive's copy fills in for some hours after the
   flight, as later messages arrive. Until a day after its hour an answer stays fresh an hour;
   after, the TTL is `min(30 days, now − (hour + 1 day))`, so only a copy fetched after the day
   is fresh, and a copy fetched young is fetched again.
3. **Read the CSV, not the HTML.** Its header names each column's unit (`pressure_hPa`,
   `geopotential height_m`, `wind speed_m/s`), which the parser checks, refusing any other unit,
   as `open_meteo` does with `hourly_units`. Columns are found by name, once each; a header of
   more than 64 names and a row of more than the header's fields are refused before they are
   split further, so a hostile answer costs no more memory than a good one.
4. **FM 35 by default, BUFR as an option.** The coded message is about 200 rows and 21 KB, the
   BUFR file about 6,000 rows and 540 KB. They differ most near the ground: in the recorded
   Santa Teresa pair the BUFR wind is 11.1 m/s 8 m up, where the coded message ramps from 5.7 to
   10.7 m/s over 186 m. Calisto is 402 m from the pad at apogee in one and 563 m in the other,
   upwind (the example `wyoming_sounding`, which also flies the BUFR file without those rows:
   419 m west). Which is nearer a rocket's wind is not known; the guide says so.
5. **The first row is the ground**; a first row missing a value or with one out of range refuses the
   answer. A later row with a value missing is dropped (`NoData`), and so is one out of range
   (`OutOfRange`: pressure outside 0.1 to 1,200 hPa, temperature outside −150 to 80 °C, geopotential
   height outside −1 to 60 km, wind speed outside 0 to 300 m/s, humidity below zero, direction
   outside 0° to 360°), rather than refusing the whole sounding; the bounds keep every level `parse`
   returns within what `SoundingProfile` accepts. A row with a pressure, height and temperature fits
   the row before it when its geopotential height above (or below) it is the layer's hypsometric
   thickness, `(R_d T̄_v/g₀) ln(p_bottom/p_top)` (WMO-No. 8 (2023) Vol. I eqs. 12.17 and 12.18;
   `T_v` from the humidity, dry without one, the vapour's share of the pressure capped at 1 so
   hostile input can't make it infinite), within 5% of it plus what rounding the two pressures can
   move it (half of 1 hPa for a pressure of 100 hPa or more in an answer whose pressures there are
   all whole, a coded message's; else half of 0.1 hPa) plus 30 m. Rows are kept from the longest
   chain from the ground in which each row fits the one before it, skipping at most 10 rows with a
   temperature at a time; of chains equally long, the one whose misses, as shares of their
   allowances, sum least. It is found by dynamic programming over the 11 rows before each, so it is
   linear in the rows. A row missing only its wind or humidity can be in the chain, so a gap in the
   wind keeps the checked layers thin. A row not in the chain is dropped (`Thickness`). Nothing
   before the ground checks it, so a chain starting at one of the 11 rows after it with a pressure,
   height and temperature within the bounds that beats every chain from it (longer, or as long with
   a smaller sum) refuses the answer (`GroundMisfit`). Bad rows right after a good ground that miss
   it but fit the rows above can win the same way, in narrow bands of error just past the ground's
   allowance, and refuse the answer: two BUFR rows 31 m high (not 20 m, which fit the ground, nor
   35 m, which fit nothing but each other), or three rows of the winter coded message 45 m high.
   So do 11 bad rows that fit each other, however far off. The data can't tell these from a
   bad ground, and a refusal is the safe failure.
   Rows grossly off fit nothing above and are passed by, up to 10. A ground with one row after it
   goes unchecked. Of each run of complete rows in the chain with the same pressure the middle one
   is a candidate (`SamePressure` for the rest; none at the ground's pressure): BUFR's pressures, to
   0.1 hPa, repeat on 1,931 of 5,851 rows, and the rounded value is the pressure at about the middle
   of its run. Keeping the first row of each run put 10 hPa 26 m low (the physics review) and misses
   a row by up to 0.093 hPa; with the middle, every BUFR row lies within 0.071 hPa of the profile. A
   candidate higher than, and at a lower pressure than, the last row kept is kept (`NotAbove`
   otherwise). In the three recordings every row fits the one before it within 1 m beyond rounding
   (0 in the coded messages, 0.97 m in BUFR; the test derives the thickness from the file's mixing
   ratio), so the 30 m and 5% are margin: 30 m for heights rounded to 10 m (the coded message's,
   from 500 hPa up), and because a height 30 m off misplaces a level by as much as a 0.33% to 0.55%
   pressure error; 5%, a judgment rather than a measurement, for layers whose inner rows lack a
   temperature. A bad row (57 hPa for 557, which would have to be 18.6 km above the row before it,
   or a height 850 m off) is dropped instead of kept to hide the good rows after it, and rows that
   fall or stay at one height fit and are dropped as `NotAbove`, however many. More than 10 rows
   after the chain's end refuse the answer (`Misfit`): the chain's end or all of them are wrong, or
   a long run of rows with no temperature leaves a layer too thick for its ends' mean to give. Ten
   rows are about 50 m of a BUFR climb and can be kilometres of a coded message. A ground at 1,200
   hPa, −1,000 m or −150 °C, which the bounds allow, now refuses. Counting rows hidden below a kept
   row, tried first, failed each way the code review probed it: a tail's first and last rows let a
   burst then a fall through and refused a float ending higher; a climbing chain stopped at a second
   bad row; rows above the row kept before failed on two bad rows in a row. The first thickness
   check, against the last row kept with 10 m of slack and 0.5 hPa for every whole pressure, refused
   a BUFR answer for a 20 m glitch and, across a long gap in the wind, for its ends' mean
   temperature; the chain without the look ahead let a row, or a ground, that only just fit drop the
   good rows after it; refusing when one row missed the ground and the next fitted that row refused
   a good ground before two bad rows; and local rules on top (the next row deciding an odd one out,
   a backtrack to an earlier chain row, three rows refusing the ground) kept a ground just outside
   its allowance and could drop a BUFR run of ten good rows for two bad ones (the physics and code
   reviews). The longest chain replaces them all. Raising one row, or a block of two or three, by
   20, 35, 50 or 100 m, or lowering it by 50 or 100 m, refuses nothing and loses at most 2 other
   levels of the coded messages; in the BUFR file, sampled every 150th row (for the test's run time)
   plus the worst rows a sweep of every row found, at most 8 other rows for one row and 10 for a
   block. Not caught: a wrong wind, humidity or temperature (the check doesn't use the wind,
   humidity moves it a few percent, and on layers under about 100 m any temperature within the
   bounds fits), a height error within the allowance (50 m on the coded message's 557 to 549 hPa
   layer), a pressure and height both wrong yet fitting, a ground within its first layer's
   allowance, and bad rows within the allowance of their neighbours, which can be kept while good
   rows are passed by, no more than the bad and fewer unless the bad fit more closely (683 and 673
   hPa raised 50 m leave out 664 hPa). `hpr-net` now depends on `hpr-core` for standard gravity: it
   is pure, `hpr-atmos` already uses it, and no third-party crate is added. More than 100,000 rows
   are refused as they are read, which bounds the memory a hostile answer costs. A relative humidity
   above 100% is kept as recorded and clamped to 100% in `sounding()`, as ADR-004 asked of
   radiosonde imports.
6. **Heights are geopotential**, converted with WMO-No. 8 (2023) eqs. 12.15 and 12.16 at the first
   row's latitude, the latitude `SoundingProfile` then uses; the balloon's drift would move a height
   about 0.8 m per degree at 10 km, 2.5 m at 30 km. Checked as in ADR-119: across the 13 layers
   between the standard levels from 850 to 10 hPa the recorded thicknesses match the hypsometric
   ones (the file's mixing ratios for the virtual temperature) to 0.01% to 0.03% on average, single
   layers 0.05% to 0.13% off on average either way; read as geometric heights they would be 0.51% to
   0.60% too thin on average. The test holds the mean under 0.1% and beyond −0.4%.
7. **Relative humidity** is the file's `relative humidity_%`, over liquid water; the file gives
   `humidity wrt ice_%` separately.
8. **Fixtures**: three answers recorded unchanged on 2026-09-30, Santa Teresa (72364) on 21 June
   2025 at 12 UTC in both versions and Salt Lake City (72572) on 15 January 2025 at 12 UTC in the
   coded version, served by `Replay` keyed by the exact URL. Committing them is decided by the
   project, left open for the maintainer to reverse: both are U.S. National Weather Service
   stations, whose observations are U.S. government works (17 U.S.C. § 105) exchanged freely
   under WMO Resolution 40; the archive's table of them adds only derived values; the archive
   states no terms; and Unidata's `siphon` (BSD-3-Clause) commits recorded answers from the same
   archive as test fixtures (`tests/fixtures/wyoming_sounding` and others). Only U.S. stations'
   soundings are committed; any other stays under `refs/`.
9. The dates in URLs and in the release time use Hinnant's algorithms, now in the private
   `civil` module shared with `open_meteo`.

**Consequences.** M5.2b is met by `tests/wyoming.rs`. How far a station's sounding is from the air
over a launch site, in distance and time, is unmeasured. The HTML form, the archive's other
formats and a station list are not read. The examples `wyoming_sounding` and `open_meteo_weather`
now print where Calisto is at apogee, east and north, since a drift distance alone read as
downwind when the rocket had weathercocked upwind.
