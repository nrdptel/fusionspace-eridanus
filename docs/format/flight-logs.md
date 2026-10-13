# Flight logs: the record and the layouts read

See which altimeters' flight logs HPR Sim reads, what it keeps from each, and how that was checked.

An altimeter or flight computer saves its flight as a file: a clock and the readings of its
sensors, such as a barometer's altitude, an accelerometer, a battery's voltage, and, on some, a
GNSS receiver's position. Each maker writes its own layout. HPR Sim reads each layout into one
record, the same for every logger, so that what comes after (the readings, and later the
comparison with a simulation) doesn't depend on which altimeter flew.

> **How far to trust it.** Every sample log the project holds of the layouts below is read but
> one AltOS export whose header line is damaged, which is refused
> ([how it was checked](#how-it-was-checked)). No maker publishes a specification of these
> layouts, so each reader follows what the files show, the makers' manuals and the notes of
> Debrief, the project owner's earlier flight-log analyzer, with each unit checked where the
> files allow. Where a header names no unit, the reader decides it from the file and says how in
> a note ([units the file decides](#units-the-file-decides)); the few units no source states are
> named [below](#units-no-source-states). Other layouts are refused, not guessed at; they come
> with [M7.1c3](../decisions-and-roadmap.md#m7-1c3), the other loggers' formats.

**To read a flight** from these layouts today, use the Rust library: give the file's bytes to
`hpr_flightdata::import::read`
([API reference](../api/hpr_flightdata/import/index.html)). It recognizes the layout from the
file's content, never its name, and reads it or refuses it, naming the layouts it knows:

```rust,ignore
let bytes = std::fs::read("flight.csv")?;
let log = hpr_flightdata::import::read(&bytes)?;
for channel in &log.channels {
    println!("{} ({:?}): {} samples", channel.column, channel.quantity, channel.values.len());
}
```

[`hpr analyze`](../cli.md#hpr-analyze), the command line, and Python read PerfectFlite's `.pf2`
only so far. The readings `hpr analyze` prints (liftoff, apogee and the rest) were set and tested
on PerfectFlite logs, so the library withholds them on another logger's record, with that reason,
until [M7.2](../decisions-and-roadmap.md#m7-2) validates them there.

Written for [M7.1a](../decisions-and-roadmap.md#m7-1a) (the record, the survey of the sample
logs, and the PerfectFlite, AltOS and Raven readers), as
[ADR-251](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0251-flight-log-importers-in-two-increments.md)
decided, and [M7.1b](../decisions-and-roadmap.md#m7-1b) (the Blue Raven, RRC3 and Eggtimer
readers), as
[ADR-253](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0253-rrc3-eggtimer-and-blue-raven-csv-first.md)
decided, [M7.1c1](../decisions-and-roadmap.md#m7-1c1) (the GPS logs), as
[ADR-254](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0254-gps-tracks-first-among-the-other-importers.md)
decided, and [M7.1c2](../decisions-and-roadmap.md#m7-1c2) (AltimeterCloud, Entacore AIM,
PerfectFlite's `.pfa` and an Eggtimer TRS's terminal text), as
[ADR-255](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0255-text-loggers-before-the-rest-of-the-importers.md)
decided.

## The layouts read

| Layout | Loggers | Recognized by | Reader |
|---|---|---|---|
| PerfectFlite `.pf2` | Pnut, StratoLogger, StratoLoggerCF | names PerfectFlite | [`.pf2` files](pf2.md) |
| PerfectFlite CSV export | the same, through PerfectFlite's software | `Time,Altitude,` and more of the export's own column names | [below](#perfectflites-exports) |
| PerfectFlite text export | the same | the same names, separated by tabs | [below](#perfectflites-exports) |
| PerfectFlite `.pfa` | Alt15K/WD, an older PerfectFlite altimeter, through PerfectFlite's DataCap program (AltCapWD on the Mac) | names PerfectFlite and has a `**!!Data:` line | [below](#perfectflites-older-pfa) |
| AltOS CSV | Altus Metrum's TeleMetrum, TeleMega, EasyMega, EasyMini, through AltosUI (Altus Metrum's ground software) | `#version,serial,flight,`, or `#` comment lines and then that header or `version,serial,flight,` | `hpr_flightdata::altos` |
| Raven FIP CSV | Featherweight Raven, through the Featherweight Interface Program | opens with a time column and holds the Raven's `Axial Accel (Gs)` channel | `hpr_flightdata::raven` |
| Blue Raven CSV | Featherweight Blue Raven, through its phone app: a low-rate file (barometer, inertial figures, flags) and a high-rate file (gyroscopes, accelerometers, attitude) | `Year,Month,Day,Time,Flight_Time_(s),Sync,` | `hpr_flightdata::blueraven` |
| RRC3 CSV | Missile Works RRC3, through mDACS, Missile Works' program that downloads its flights | `Time`, `Altitude`, `Pressure`, `Velocity`, then any of `Temperature`, `Events`, `Voltages`, perhaps with units in brackets | `hpr_flightdata::rrc3` |
| Eggtimer CSV | Eggtimer Classic, TRS, Quantum and Proton, through Eggtimer's download | `T,Alt,` and more of Eggtimer's column names | `hpr_flightdata::eggtimer` |
| Eggtimer text | Eggtimer TRS, its download captured by a terminal program | a title `Eggtimer <model> <firmware> … Flight Settings`, then Eggtimer's header row `T,Alt,…` | `hpr_flightdata::eggtimer` ([below](#units-the-file-decides)) |
| AltimeterCloud CSV | Rocketry Ltd Nano V1 and Mercury, as a flight page on altimetercloud.com hands the log back | a header within the first 65 lines whose first name is `Time(ms)` (in any case) and which also names `acceleration_total(mG)` and `tilt` | `hpr_flightdata::altimetercloud` ([below](#units-the-file-decides)) |
| Entacore AIM CSV | Entacore AIM XTRA (the flight computer's own log) and AIM BASE (the telemetry its ground station received), as the AIM software's *Export to Excel…* writes them | the first line starts with a `time` column, names at least two, and names one of the AIM's own channels, such as `Pressure AGL`, `lat. X accel.` or `eject. bat. voltage` | `hpr_flightdata::entacore` ([below](#units-the-file-decides)) |
| Featherweight GPS CSV | Featherweight's GPS tracker: its own log, and its ground station's log of every tracker it hears | a first line opening `UTCTIME,UNIXTIME,` or `TRACKER,DATE,TIME,` that names the latitude and longitude (`LAT` and `LON`, or `TRACKER Lat` and `TRACKER Lon`) | [below](#gps-logs) |
| KML | GPS tracks as AltOS, Missile Works' RTx, Big Red Bee's BeeLine, fw2kml (a converter of Featherweight's logs), GPS Visualizer and others write them for Google Earth | the document's root element is `kml` | [below](#gps-logs) |
| NMEA 0183 | a GPS receiver's own sentences, written to a file as it sends them | at least 3 of the first 50 lines open with a sentence whose checksum checks out, such as `$GPGGA,…*hh`, or, in a shorter file, every line does | [below](#gps-logs) |

The Blue Raven app writes each flight as a low-rate file, logged 50 times a second, and a
high-rate file, 500 times a second (*Blue Raven User's Guide*, 2026-05-12, p18); each is read on its own, as a record of its own, not joined
to the other. The high-rate file holds no altitude, so its record's
[primary track](#the-record) is empty and its channels hold everything; the low-rate file of the
same flight holds the altitude. The app's summary and events files beside them aren't read yet.
The other layouts in the sample logs, among them AltOS's `.eeprom`, the RRC3's binary `.rff`, the
Raven's `.fipa` and spreadsheets, are refused today, most as *unrecognized*; the
[survey's report](#how-it-was-checked) counts them by layout.

## The record

Every reader fills the same record, `FlightLog`, in SI units:

- **What the file says about itself:** the logger's name, serial number, firmware and flight
  number where the file states them, and figures the logger worked out itself, such as its apogee,
  kept beside HPR Sim's readings and never in place of them.
- **The primary track:** the barometric altitude above the pad (AGL, above ground level) on the
  logger's clock, with the barometer's vertical speed, the temperature and the battery voltage
  where they share that clock. The [readings](../physics/log-readings.md) are taken from it. A
  log with no barometer, such as a [GPS log](#gps-logs), has an empty primary track.
- **Every column, as a channel:** each data column of the file, in the file's order, with the
  name the file gives it, what it measures, and its own clock. Nothing the logger wrote is dropped:
  a column HPR Sim can't name a quantity for is kept as written and marked *unrecognized*.
- **Events:** what the logger says happened and when, such as AltOS's change from `boost` to
  `coast`.
- **Notes:** what the reader noticed and worked around.

A channel's unit follows from what it measures:

| Quantity | Unit | Examples |
|---|---|---|
| altitude above the pad (AGL), above sea level (ASL or MSL), from GNSS (satellite navigation, such as GPS), or integrated from the accelerometer | m | `height`, `[Altitude (Baro-Ft-AGL)]` |
| pressure | Pa | `pressure`, `Baro (Atm)` |
| speed | m/s | `speed`, `Velocity (Accel-Ft/Sec)` |
| acceleration | m/s² | `acceleration`, `Axial Accel (Gs)` |
| temperature | K | `temperature`, `Temperature (F)` |
| voltage, current | V, A | `battery_voltage`, `Current Draw (A)` |
| latitude, longitude, an angle | degrees | `latitude`, `pad_az` (the bearing from the pad) |
| a rate of turn | degrees per second | `Gyro_X` |
| a time of day on the logger's clock | s after its midnight, in no stated time zone | a Blue Raven's `Time` |
| distance | m | `pad_dist` (the distance from the pad) |
| a number with no unit | none | a count of satellites, a signal strength, a state code |

A foot is 0.3048 m (1.000 ft), a g 9.806 65 m/s² and an atmosphere 101 325 Pa, each exactly. A
cell the file leaves empty, or fills with something that isn't a number, is a gap: `NaN` ("not a
number") in the record, and `null` in its JSON, which reads back as a gap. A channel's time is a
gap in the same way where the file gives a sample no time, as a KML path without times does.

### Units no source states

Most units come from the file's own column names or from the maker's manual. A few don't:

- AltOS's `pad_dist` and `pad_range` are read as meters, as its manual says its exports are
  metric, without naming these columns.
- AltOS's gyroscope and magnetometer columns (`gyro_*`, `mag_*`) are kept as written, unrecognized,
  as no document states their units.
- The Raven's status columns, `bILBA`, are kept as numbers, the bits read as binary: no published
  source says what each bit means, so the record takes no events from them.
- The Blue Raven's `Current` is kept as written, unrecognized: the app's file states no unit, and
  its manual gives milliamperes only for another download; the values fit amperes better, but
  that is a size, not a source.
- An RRC3's `Temperature` and `Voltages`, without units in brackets, are read in degrees
  Fahrenheit, or Celsius when the altitude is in meters, and in volts, assumed: the RRC3's own
  unit settings pair feet with degrees Fahrenheit and meters with degrees Celsius (*RRC3+/RRC3
  User Manual*, rev. 1.71, p12), but mDACS writes each in its own preferred unit (*mDACS User
  Manual*, rev. 1.71, p5), which the file doesn't name. Each record says so in a note.
- GPS Visualizer's KML altitude is kept as written, unrecognized: the program takes its source's
  altitude as meters unless the source's column names feet, so its KML holds meters only if the
  source said which, and the file can't show whether it did. The one such sample, set beside a
  barometric logger on the same flight, holds feet written as meters.
- Two KML files styled after AltOS's, which name no writer, give each point a `Seconds` and an
  `Alt` in a comment; the seconds are read as seconds, from the name, and the altitude in meters,
  from the KML standard, as a height above the ground. Their tracks rise less than half as high
  as their flights' barometric peaks, which neither meters nor feet explains, so trust their
  heights least of any GPS log.
- AltOS's KML writes a `time` beside every point; where it is 0 at every point, as in most of its
  sample files, the reader takes it as unset and the points have no time. No document says so.
- AltOS's KML also writes the GPS receiver's `alt` beside every point. Where it never equals the
  path's altitude, as in 3 of its older sample files, nothing in the file shows its unit, and it is
  kept as written.
- AltimeterCloud's Mercury writes an air density, a predicted apogee, a target altitude and a
  second temperature (`mt1_temp`) that no published page gives a unit for; each is kept as
  written, unrecognized.
- Entacore AIM's magnetometer (`Mag X`, `Mag Y`, `Mag Z`) is kept as written, unrecognized: its
  manual names tesla, but the sample files' values are many times the Earth's field read so
  ([below](#units-the-file-decides)).

### Units the file decides

These layouts leave some units out of their headers: the RRC3's, Eggtimer's (CSV and terminal
text), the Blue Raven's, AltimeterCloud's and Entacore AIM's; a `.pfa` states none
([below](#perfectflites-older-pfa)). For some, the reader works the unit out from the file as it
reads it; for the rest, it takes the unit the maker's manual gives, and checks it against the file
where the file can show a disagreement. Either way it writes a note giving the decision and this
file's evidence, and the [survey's report](#how-it-was-checked) counts the decisions over the
sample logs:

- **An RRC3's altitude, speed and clock, worked out from the file.** Missile Works' mDACS (the
  program that downloads an RRC3's flights to a computer) writes the altitude, speed and
  temperature each in the unit chosen in its preferences, and the header names none of them
  unless the units are in brackets. The pressure settles the altitude: the climb the pressure
  implies in the standard atmosphere is set against the altitude column's climb read in feet and
  in meters, 3.28 times apart, and the unit within 25% of it is taken. The speed is set against
  the altitude's rate of climb, allowing for the speed to lag behind it: of feet per second,
  miles per hour, meters per second and kilometers per hour, the one two measures agree on within
  3% is taken. Where they don't, or the record stops before the rocket is halfway down, the column
  is kept as written, with a note. The time is in seconds where its step is the RRC3's 20 samples
  a second. Of the 50 RRC3 exports among the sample logs, 46 are read in feet from the pressure
  and 4 state feet; 36 have speeds in feet per second and 8 in miles per hour, 4 state feet per
  second, and 2, whose records stop early, keep their speeds as written.
- **An Eggtimer's clock, worked out from the file.** The Classic and TRS write their time in
  milliseconds, the Quantum and Proton in seconds, under the same `T`. The reader takes
  milliseconds when the median step is above 5: the manuals' rates, 1 to 33 samples a second, put
  a step between 0.03 and 1 in seconds and between 30 and 1,000 in milliseconds. Of the 43
  Eggtimer files, 3 are read in milliseconds and 40 in seconds.
- **An Eggtimer's heights and speeds, from the manuals.** The Classic and TRS manuals state feet
  and feet per second. The Quantum and Proton manuals state no unit for the download, but give
  every height in feet, and no Eggtimer manual offers meters; the heights are read in feet for
  every model. A speed is read in feet per second only where it matches the altitude's change
  over the step on at least 90% of rows (on the sample logs, 95.9% or more); otherwise it is kept
  as written.
- **A Blue Raven's speeds, inertial positions, rates of turn and accelerations, from the
  manual.** The manual gives their units for the logger's USB download: feet per second, feet,
  degrees per second and g (*Blue Raven User's Guide*, 2026-05-12, p23). The app's own files
  agree: its summary of a flight states the top speed in feet per second and the inertial
  altitude in feet, and its accelerometers read 1 g at rest on the pad. A file that disagreed
  would still be read in these units; its note would show the disagreement.
- **An Eggtimer TRS's terminal download, its preamble by its own unit marks.** Captured in a
  terminal program, a TRS's download prints its flight settings and a flight summary before the
  table its CSV download writes (*Eggtimer TRS Flight Computer User's Manual*, firmware 2.07,
  p35–37). The table is read as the CSV download is, its clock decided by its step. The summary's
  apogee becomes the record's stated apogee, and its `ASL Alt.` the launch site's elevation, each
  read by its own unit mark, `ft`. The summary's event times, written without a unit in both
  sample files, are read in seconds, as the manual's download writes them (`secs.`, p36). They are
  on the table's clock: in both sample files every one matches its marker's row to the
  millisecond. So a summary time with a matching marker is kept once, as the marker's event; the
  top speed's time, which no marker marks, becomes an event of its own; and a time of 0 means a
  channel that didn't fire (p25). The settings and the other summary lines, such as the base
  temperature with its own `F` or `C`, are kept as written in the record's notes. The name the
  owner gave the flight computer is left out, since it may name a person.
- **AltimeterCloud's temperature and summary times, worked out from the file.** The Nano V1's
  manual has a page on its CSV layout (*CSV Flight log format*,
  <https://www.altimetercloud.com/support/nano-v1/flight-log-format/>, read 2026-10-13). The
  reader follows Debrief's in recognizing the layout and in the centi-degree temperature.
  - *From the manual:* time in milliseconds from liftoff, altitude above the pad in meters,
    acceleration in thousandths of a g, rates of turn and attitude angles in degrees. The
    Mercury's CSV uses the same names for the same columns, so its rates and angles are read in
    the Nano's units too, with a note saying so.
  - *Total acceleration:* `acceleration_total(mG)` is the acceleration's magnitude with 1 g taken
    out, so it reads about zero at rest and −1 g in free fall; it is kept so, as an acceleration
    along no axis, and its note says what it is.
  - *The Mercury's temperature:* `bmp_temp(x100)` holds hundredths of a degree Celsius. It is read
    so only when every value is a whole number, every value divided by 100 falls within the
    barometer's measurement range, −40 to 85 °C (Bosch Sensortec, *BMP581 data sheet*,
    BST-BMP581-DS004-13, table 2, p. 10), and some values as written fall outside it, so the
    column can't already be in degrees; otherwise it is kept as written.
  - *The summary's times:* the block at the top of a Nano log gives the apogee's and the
    landing's times with no unit. The apogee time is read in seconds when the log's highest
    altitude falls within half a second of it (in all 7 sample logs with a summary, within
    0.01 s). The landing time is read in seconds when, read so, it comes after the highest
    altitude and the log's altitude there is the summary's landing height within 0.01 m, and read
    in milliseconds it isn't; the manual's *Flight info page* gives the landing as "when it
    landed, and the height it came to rest at". In all 7, the two agree within 0.001 m, where read
    in milliseconds the time falls on the climb, more than a meter from the landing height. A time
    the altitude doesn't bear out is left out, with a note saying why.
  - *Kept as written:* the Mercury's air density, predicted apogee, target altitude and
    `mt1_temp`, which have no published unit.
  - *Gaps:* two kinds of value aren't readings and become gaps, each counted in a note: the zeros
    the accelerometer writes on the pad before its first reading, and the exact 0 °C
    temperatures that end every sample log after readings of 27 to 44 °C.
- **Entacore AIM's pressure, heights and clock, worked out from the file.** The AIM's export puts a
  `time` column before each group of channels, each group on its own clock (*AIM XTRA/BASE v4.x.x
  Manual*, Entacore, 2 February 2025, p40), and every column is kept on its own clock, following
  Debrief's reader. The record's altitude is the AIM's own `Pressure AGL`, its barometric height
  above the ground it was set to. The manual gives g for the accelerations (p23–24) and degrees
  per second for the gyroscope (p24); the rest the reader decides and notes. The pressure is in
  pascals, as the first sample, on the pad, is a launch site's pressure in pascals (the manual's
  own sample export writes pascals too, Table 2, p40). `Pressure MSL` and `Pressure AGL` are in
  meters or feet, whichever matches how far the pressure climbs in the standard atmosphere, as
  for the RRC3; a file that fits neither is refused. `time` is in seconds, as `GPS Time of Week`,
  in milliseconds, rises about 1,000 for each unit of it. The temperature and `GPS MSL` follow the
  heights' unit system, an assumption: the software uses one unit system for all it shows (p33).
  The voltages are taken as volts, which the manual never names. The magnetometer is kept as
  written: the manual says tesla, but the sample files read 20 to 40 times the Earth's field in
  tesla. In 11 of the 13 sample files with GPS columns, the export writes the GPS group one empty
  cell too wide once the GPS stops logging, pushing every later column one place right; the
  reader drops that cell and counts each drop in a note. The rises of `launch detect`,
  `apogee detect` and the outputs' `A on.` … `D on` become events; the AIM detects launch only
  after the rocket has kept above 30 m/s (98 ft/s) for half a second (p36), so that event comes after
  liftoff.

A unit read in the wrong one of feet and meters would put every height 3.28 times too high or too
low, so the RRC3's is never assumed where its pressure can settle it, and the Eggtimer's rests on
the manuals.

### A worked example

PerfectFlite's CSV export of a flight starts like this: the first rows of an invented flight,
`perfectflite-export.csv`, which the crate's tests read and check these numbers against:

```text
Time,Altitude,Velocity,Temperature (F),Voltage,Acceleration (calculated)
0.00,0,0,71.5,9.1,-3
0.50,0,0,71.5,9.1,-3
1.00,0,88,71.4,9.1,1
```

It reads into a record whose primary track has the times 0, 0.5 and 1 s and the altitudes 0, 0 and
0 m (0 ft), and five channels on that clock: `Altitude`; `Velocity`, whose 88 at 1 s reads as
26.82 m/s (88.0 ft/s); `Temperature (F)`, whose 71.5 reads as 295.1 K (71.5 °F); `Voltage`; and
`Acceleration (calculated)`, which the export doesn't explain and is kept as written,
unrecognized.

## GPS logs

A GPS tracker or a flight computer's GPS receiver logs where the rocket was: a latitude and
longitude in degrees, an altitude above sea level, and figures of the *fix* (the receiver's
solution of its position), such as how many satellites it used. These logs have no barometer, so
a GPS log's record has an empty [primary track](#the-record) and keeps everything as channels,
with a note; the [readings](../physics/log-readings.md), set on a barometer's altitude, withhold
on it until [M7.2](../decisions-and-roadmap.md#m7-2), the milestone that reads other loggers.

> **How far to trust it.** Positions, heights and times are read as the file writes them; HPR
> Sim claims no accuracy for a GPS fix, and a GPS altitude is above sea level, not above the
> pad. The clocks are the files' own, and some don't state their time zone. Heights from KML
> files are the least sure: one writer's are feet written as meters, kept as written, and two
> files rise less than half as high as the barometer on the same flight did
> ([units no source states](#units-no-source-states)).

Three layouts are read. Each point's time is the one the file gives it; where it gives none, the
point keeps its place with its time a gap.

- **Featherweight's GPS CSV**, from Featherweight's GPS tracker.
  - *Clocks.* The tracker's own log times each fix by `UTCTIME`, such as
    `Jul 14 2031 17:04:31.250 UTC` (UTC is Coordinated Universal Time, the time zone GPS
    receivers keep). Its `UNIXTIME` runs a whole number of hours, 4 to 8, ahead of `UTCTIME` on
    every row of all 28 tracker logs among the sample logs, as if the site's offset from UTC had
    been added where it should have been taken away, so it is kept as written with a note of its
    offset. In the older layout rows step back in time and repeat; they are kept in the file's
    order and counted in a note. The ground station's log times each row by `DATE` and `TIME`,
    in a zone the file doesn't state (in most sample files, the launch site's), and may hold
    several trackers: each gets its own channels, such as `LAT [Rocket-1]`.
  - *Units.* No header names one. The manual gives the altitude in feet, the speeds in feet per
    second and the heading in degrees (*Featherweight GPS Tracker User's Manual*, 2025-02-20,
    p17), and the reader checks each file. The horizontal speed against the distance between
    fixes is the check that tells feet from meters, since the positions are in degrees whatever
    the unit: of the 40 sample files, 37 move enough for it and in all 37 it agrees with the
    manual; in the other 3 the manual alone decides, and the note says so. Where it disagreed,
    every length and speed would be kept as written. The vertical speed against the change of
    altitude shows only that the two share a unit.
  - *No fix.* A position of exactly (0, 0), which the tracker writes before it has a fix, is
    read as a gap.
- **KML** (Keyhole Markup Language), the XML file Google Earth opens, standardized as OGC
  07-147r2.
  - *Tracks.* A track is a path of longitude, latitude and altitude, in meters by the standard.
    The track's `altitudeMode` says what its altitude is: above sea level for `absolute`, a
    height above the ground for `relativeToGround`, and nothing a viewer uses for
    `clampToGround`, whose altitudes are kept as written. An AltOS file can hold the barometer's
    altitude in the path and the GPS receiver's beside each point; the reader keeps each as what
    it is.
  - *Times.* Only Google's `gx:Track` pairs each point with a time (`when`). AltOS, Big Red
    Bee's BeeLine and two unnamed files styled after AltOS's write each point's time in an XML
    comment beside it instead, which the reader takes. The rest (fw2kml's paths, from a
    converter of Featherweight's logs; Missile Works' RTx; GPS Visualizer) give no time: 9 of
    the 42 sample KML logs time their points, and 33 read as paths without times.
- **NMEA 0183**, the sentences a GPS receiver sends, in the format the National Marine
  Electronics Association set: one a line, each ending in a *checksum*, two hexadecimal digits
  that check the characters before them.
  - *Sentences.* The reader takes four: `GGA` (the position, altitude and fix), `RMC` (the
    position, speed, course and date), `GSA` (the satellites used and the precision) and `GSV`
    (each satellite in view). A sentence counts only if its checksum checks out; noise before
    it, as a receiver writes on power up, is left out and counted.
  - *Clock and units.* The time of day is UTC; the date comes only from an `RMC` with a valid
    fix, because before its fix the receiver writes a placeholder date. A speed in knots is read
    as 1852/3600 m/s a knot.

For example, take a `GGA` sentence's position and an `RMC` sentence's speed, invented after the
textbook example public descriptions of NMEA 0183 give; a test in the crate reads these two lines
and checks every number below:

```text
$GPGGA,123519.000,4807.0380,N,01131.0000,E,1,08,0.9,545.4,M,46.9,M,,0000*59
$GPRMC,123519.000,A,4807.0380,N,01131.0000,E,022.4,084.4,230394,,,A*62
```

The latitude `4807.0380,N` is 48 degrees and 7.038 minutes north, 48 + 7.038/60 = 48.1173°; the
longitude `01131.0000,E` is 11 degrees and 31 minutes east, 11.5167°. The altitude is 545.4 m
(1,789 ft) above mean sea level, in the unit the sentence names (`M`). The speed of 22.4 knots
reads as 22.4 × 1852/3600 = 11.52 m/s (37.8 ft/s), and the course is 84.4° from true north. The
record's zero is the first sentence's time, 12:35:19 UTC on 23 March 1994.

## PerfectFlite's exports

PerfectFlite's software exports a flight as CSV, `Time,Altitude,Velocity,Temperature (F),Voltage`,
sometimes with `Acceleration (calculated)` after them, or as tab-separated text, `Time`,
`Altitude`, `Temp.`, `Voltage`, whose temperatures carry their unit letter, such as `58.7F`. The
units are a `.pf2`'s: seconds, feet and feet per second. The header doesn't state the altitude's
unit, and the sample logs bear out feet. Of the 16 exports among them, 13 peak at exactly the
height of the `.pf2` saved beside them, which states feet. Read in feet, those 13 climb in 0.94 to
1.05 times as long as a fall in vacuum from their peak would take, and the other 3 in 0.93 to
1.07 times, alike; read in meters, the 3 would climb in 0.51 to 0.59 times
([the survey's report](#how-it-was-checked) prints these). An export doesn't name the
altimeter's model, so its logger reads `PerfectFlite`.

A header with any other name, or these names in another case, isn't read as PerfectFlite's: the
RRC3's `Time,Altitude,Pressure,…` is the RRC3's, and a bare `time,altitude` says nothing of its
units.

## PerfectFlite's older `.pfa`

The Alt15K/WD, an older PerfectFlite altimeter, saves its flights through PerfectFlite's DataCap
program as a `.pfa`: a line naming the altimeter, then `Ground:`, `Apogee:`, `NumSamps:` and
`Comments:` lines, a blank line, a `**!!Data:` line, and one height a line, such as `00274`. Most
end their lines in a carriage return alone, as old Macs did; some in CR LF. Both read.

> **How far to trust it.** The file states neither its rate nor its unit; both come from
> PerfectFlite's documents, not the file, and the 25 sample `.pfa`s agree with them (the survey's
> `pfa` row of unit decisions counts the note on each).

The file has no time column and names no unit. The altimeter's manual says it takes a sample
every 100 milliseconds (*Alt15K/WD User's Manual*, p. 2) and reports heights in feet (p. 6), and
the help of DataCap 1.10 for the Mac (AltCapWD) says 10 samples a second, so each height is read
in feet above the pad, 0.1 s after the one before. The 25 sample `.pfa`s bear it out: in every
one, `NumSamps:` is the number of heights and `Apogee:` the highest of them
([the survey's report](#how-it-was-checked) prints these checks). Each record says so in a note,
which the survey counts.

Two things in the file aren't what they look like:

- **Time zero isn't liftoff.** The altimeter notices a launch once the rocket is 80 ft up
  (manual, p. 2; a later revision lets it be set from 40 to 250 ft, PerfectFlite's Alt15K
  specifications page), and saves its last 8 samples, the one that noticed among them (p. 2):
  DataCap's help says sampling "begins at a point 0.7 seconds before the launch is detected",
  seven steps of 0.1 s. So time zero is 0.7 s before the
  altimeter noticed the launch. The rocket left the pad before it noticed; a slow rocket may
  have left before time zero, so the file can miss the first moments of the climb. Every sample
  `.pfa` first reaches 40 or 80 ft at 0.7 s (17 at 80 ft, 8 at 40 ft, as the report prints).
- **`Ground:` isn't the pad's elevation.** It is the pad's pressure altitude: the height the day's
  air pressure would mean if sea-level pressure were the standard 29.92 inches of mercury (inHg).
  DataCap's help says it is about 100 ft off for every 0.1 inHg the day differs, and gives it
  "for interest only". The record keeps it in a note, not as the ground elevation.

A worked example: in the invented sample `perfectflite-alt15k.pfa` (read by the test
`tests/perfectflite_pfa.rs`), the eighth height, `00088`, is the sample at 0.7 s, the first at
80 ft or more, and reads as 88 × 0.3048 = 26.82 m above the pad.

DataCap's help also says a `NumSamps:` of 2,200 (a revision 2 altimeter) or 2,400 (the original)
likely means the altimeter lost power in flight, and that older flights then follow this one in
the data. Such a file reads, with a note saying so; none of the sample files is one. A line among
the heights that isn't a whole number, or an empty line between them, is refused, naming the line:
every line is a step of 0.1 s, so one left out would move every time after it.

## How it was checked

`cargo xtask logs`, a command of the project's own build tool, reads every sample log the project
holds: the private collection's catalog of flight logs, the public fixtures and samples of
Debrief, and AerospaceNU's published flight logs, each distinct file once. It leaves out a cache
of downloads whose flights' logs the collection already catalogs, and RocketPy's processed flight
tables, which no logger wrote. It writes what came of each to
[`validation/reports/log-import.md`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/log-import.md):
per layout, how many files were read and how many refused and why; how many files' readers
decided each unit their headers leave out, and how; and per file an anonymized id (the start of a
hash of the file, which says nothing of its contents) with a digest, a hash, of the record read
from it.

A file the collection's catalog files under a layout it doesn't have is moved, by id, with the
reason printed: one is a spreadsheet's re-save of an AltOS export, its header edited, which is
generic CSV; another is an AltOS export whose header line is damaged, two column names run
together, which no reader can line up with its rows and which is refused; a third, a KML file, is
a map of a flight made in a map program, its predicted tracks with the pad and landing spot
pinned, read but counted apart as not a log. And 25 files the catalog files as generic CSV, by
their extension, are in AltimeterCloud's layout (10) or Entacore AIM's (15), and are held to
those readers.

The collection is other people's data, Debrief's fixtures come from flights shared on terms nobody
recorded, and AerospaceNU's are published under the GPL, so no file and no value read from one is
committed. The digest pins each reading instead, as a snapshot (a stored copy of a test's output)
would: the check fails if a reader's output changes or a file of these layouts stops reading.
Invented files in the crate's tests are read in CI and pinned whole by snapshots.
