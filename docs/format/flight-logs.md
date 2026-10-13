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
> with [M7.1c5](../decisions-and-roadmap.md#m7-1c5), the other loggers' formats.

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
decided, and [M7.1c3](../decisions-and-roadmap.md#m7-1c3) (the SillyGoose, Fluctus and ARTS2
readers), as
[ADR-256](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0256-three-club-and-team-layouts-before-the-rest.md)
decided, and [M7.1c4](../decisions-and-roadmap.md#m7-1c4) (AerospaceNU's flight computer board
and ground station, a phone app's `launch` CSV and an Eggtimer's RTF), as
[ADR-258](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0258-aerospacenus-board-and-a-phone-apps-csv-before-the-rest.md)
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
| Eggtimer RTF | the same terminal download, saved by a text editor as an RTF document | opens with `{\rtf`, and its text holds the terminal download's title and header row | `hpr_flightdata::eggtimer` ([below](#units-the-file-decides)) |
| AltimeterCloud CSV | Rocketry Ltd Nano V1 and Mercury, as a flight page on altimetercloud.com hands the log back | a header within the first 65 lines whose first name is `Time(ms)` (in any case) and which also names `acceleration_total(mG)` and `tilt` | `hpr_flightdata::altimetercloud` ([below](#units-the-file-decides)) |
| Entacore AIM CSV | Entacore AIM XTRA (the flight computer's own log) and AIM BASE (the telemetry its ground station received), as the AIM software's *Export to Excel…* writes them | the first line starts with a `time` column, names at least two, and names one of the AIM's own channels, such as `Pressure AGL`, `lat. X accel.` or `eject. bat. voltage` | `hpr_flightdata::entacore` ([below](#units-the-file-decides)) |
| SillyGoose text | AerospaceNU's SillyGoose flight computer: its own log, tab- or space-separated, with or without a `CONFIG` settings line | a first line, or the one after `CONFIG`, starting `timestampMs` and naming `pressurePa` and `altitudeM` | `hpr_flightdata::sillygoose` ([below](#units-the-file-decides)) |
| Silicdyne Fluctus CSV | Silicdyne Fluctus, as its Fluctus Control Center exports a flight | a `sep=` line (such as `sep=;`) within the first 16 lines, or none, then a header starting `time` that names `deltaTime` and `baro-altitude` | `hpr_flightdata::fluctus` ([below](#units-the-file-decides)) |
| ARTS2 CSV | Ozark Aerospace ARTS2, its interpreted data as its software exports a flight | a first line of `#` and the names `MET Acc Vel iAlt pAlt`, by spaces, tabs or commas, in any case | `hpr_flightdata::arts2` ([below](#units-the-file-decides)) |
| AerospaceNU board CSV | AerospaceNU's flight computer board (FCB), its binary log as the club's tools turn it into a CSV | a first line whose first name is empty (the club's row number) and which names `packetType`, `timestamp_ms` or `timestamp_s` | `hpr_flightdata::aerospacenu_fcb` ([below](#units-the-file-decides)) |
| AerospaceNU ground station text | the same board's radio downlink, one packet a line, as the club's ground station logged it | a first line that isn't blank starting `RUN START` | `hpr_flightdata::aerospacenu_gs` ([below](#units-the-file-decides)) |
| Phone app `launch` CSV | an app, named in none of its files, that logs a phone's own sensors in flight | a first line that isn't blank starting `Seconds,Timestamp,AltitudeMSLGPS` | `hpr_flightdata::phone_launch` ([below](#units-the-file-decides)) |
| Featherweight GPS CSV | Featherweight's GPS tracker: its own log, and its ground station's log of every tracker it hears | a first line opening `UTCTIME,UNIXTIME,` or `TRACKER,DATE,TIME,` that names the latitude and longitude (`LAT` and `LON`, or `TRACKER Lat` and `TRACKER Lon`) | [below](#gps-logs) |
| KML | GPS tracks as AltOS, Missile Works' RTx, Big Red Bee's BeeLine, fw2kml (a converter of Featherweight's logs), GPS Visualizer and others write them for Google Earth | the document's root element is `kml` | [below](#gps-logs) |
| NMEA 0183 | a GPS receiver's own sentences, written to a file as it sends them | at least 3 of the first 50 lines open with a sentence whose checksum checks out, such as `$GPGGA,…*hh`, or, in a shorter file, every line does | [below](#gps-logs) |

The Blue Raven app writes each flight as a low-rate file, logged 50 times a second, and a
high-rate file, 500 times a second (*Blue Raven User's Guide*, 2026-05-12, p18); each is read on its own, as a record of its own, not joined
to the other. The high-rate file holds no altitude, so its record's
[primary track](#the-record) is empty and its channels hold everything; the low-rate file of the
same flight holds the altitude. The app's summary and events files beside them aren't read yet.
The other layouts in the sample logs, among them AltOS's `.eeprom`, the RRC3's binary `.rff`, the
Raven's `.fipa`, spreadsheets and a radio telemetry stream saved as RTF, are refused today, most as
*unrecognized*; they come with [M7.1c5](../decisions-and-roadmap.md#m7-1c5), and the
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

A cell that is a number but that its unit's conversion to SI makes infinite, such as `1.7e308`
in g, which times 9.806 65 passes the largest number a computer's `f64` holds, refuses the file,
naming its line. Every reader applies this rule, so a channel holds SI values or gaps and nothing
else ([ADR-258, the rule for infinite values](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0258-aerospacenus-board-and-a-phone-apps-csv-before-the-rest.md),
§3).

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
  manual names tesla, but the sample files' values are 17 to 34 times the Earth's field read so
  ([below](#units-the-file-decides)).

### Units the file decides

These layouts leave some units out of their headers: the RRC3's, Eggtimer's (CSV and terminal
text, and that text saved as RTF), the Blue Raven's, AltimeterCloud's, Entacore AIM's, a
Fluctus's, an ARTS2's, AerospaceNU's flight computer board's and its ground station's, and a phone
app's; a `.pfa` states none ([below](#perfectflites-older-pfa)), and a SillyGoose's header names
units that one file's values contradict. For some, the reader works the unit out from the file as it
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
  - *The summary's times:* the block at the top of a Nano log gives the apogee's and the landing's
    times with no unit. The apogee time is read in seconds when the log's highest altitude falls
    within half a second of it (in all 7 sample logs with a summary, within 0.01 s). The landing
    time is read in seconds when, read so, it comes after the highest altitude and the log's
    altitude there is the summary's landing height within 0.01 m (0.033 ft), and read in
    milliseconds it isn't; the manual's *Flight info page* gives the landing as "when it landed, and
    the height it came to rest at". In all 7, the two agree within 0.001 m (0.0033 ft), where read
    in milliseconds the time falls on the climb, more than a meter from the landing height. A time
    the altitude doesn't bear out is left out, with a note saying why.
  - *Kept as written:* the Mercury's air density, predicted apogee, target altitude and
    `mt1_temp`, which have no published unit.
  - *Gaps:* two kinds of value aren't readings and become gaps, each counted in a note: the zeros
    the accelerometer writes on the pad before its first reading, and the exact 0 °C
    temperatures that end every sample log after readings of 27 to 44 °C.
- **Entacore AIM's pressure, heights and clock, worked out from the file.** The AIM's export puts
  a `time` column before each group of channels, each group on its own clock (*AIM XTRA/BASE
  v4.x.x Manual*, Entacore, 2 February 2025, p40), and every column is kept on its own clock,
  following Debrief's reader. The record's altitude is the AIM's own `Pressure AGL`, its
  barometric height above the ground it was set to. Any unit its evidence doesn't settle is kept
  as written, with a note saying why.
  - *From the manual:* g for the accelerations (p23–24), degrees per second for the gyroscope
    (p24), and dBm for the ground station's `RSSI` (p29), kept as written.
  - *Pressure:* pascals, as its first value that is a launch site's pressure in any unit is one in
    pascals (the manual's sample export writes pascals too, Table 2, p40).
  - *Heights:* `Pressure MSL` and `Pressure AGL` are in meters or feet, whichever matches how far
    the pressure climbs in the standard atmosphere, as for the RRC3, from a climb of 2 m (6.6 ft); a
    file that fits neither, or whose pressure can't settle it, is refused.
  - *Clock:* `time` is in seconds where `GPS Time of Week`, in milliseconds, rises about 1,000 for
    each unit of it, and in milliseconds where it rises about 1; without it, the step between
    samples decides, against the AIM's 10 to 500 ms sampling periods (p2, p21), and a file it
    can't settle is refused.
  - *Temperature:* it follows the heights' unit system, °C beside meters and °F beside feet, as the
    software uses one unit system for all it shows (p33), and its first value must be one an
    electronics bay can have.
  - *Voltages:* volts, as every value lies within 0 to 20, where the batteries the manual names
    are 3.5 to 10 V (p10) and millivolts would read in thousands.
  - *`GPS MSL`:* in the heights' unit where it reads within 300 m (984 ft) of `Pressure MSL` in that
    unit and not in the other; at a site too low for the 3.28 factor to show, kept as written.
  - *Magnetometer:* kept as written: the manual says tesla, but the sample files' field would then
    be 17 to 34 times the Earth's at its strongest, about 65 µT.
  - *A run-out GPS group:* in 11 of the 13 sample files with GPS columns, the export writes the GPS
    group one empty cell too wide once the GPS stops logging, pushing every later column one place
    right; the reader drops that cell only where every group is then whole, and counts each drop
    in a note. A header naming a column twice is refused.
  - *Events:* the rises of `launch detect`, `apogee detect` and the outputs' `A on.` … `D on`. The
    AIM detects launch only after the rocket has kept above 30 m/s (98 ft/s) for half a second
    (p36), so that event comes after liftoff.
- **A SillyGoose's units, its header checked against its values.** AerospaceNU's SillyGoose board
  names most units in its columns, such as `pressurePa`, `tempK` and `altitudeM`, but two sample
  files' values contradict their names. One has temperatures below zero under `tempK` and an
  accelerometer that reads 1 at rest. The other, which looks replayed rather than recorded (after
  a few samples on the pad it jumps into the boost, it ends still climbing, and its temperature
  never changes), has
  `angularVelRadS_*` rates beyond what a gyroscope (a sensor of the rate of turn) can read. So
  the reader checks each named unit against the file, and keeps a column as written, with a note,
  where the values refute it.
  - *Accelerometer:* m/s² or g. A board at rest on the pad or hanging under a parachute reads
    1 g. The reader takes the unit in which the board reads 1 g, within 10%, for at least 1 s and
    for at least twice as long as in the other.
  - *Gyroscope:* rad/s or °/s. While the board reads about 1 g, the reader compares how far its
    rates say it turned with how far gravity's direction turned in the accelerometer. A log too
    short to show a turn keeps the gyroscope as written.
  - *Temperatures:* kelvin where every value lies within 233.15 to 333.15 K (−40 to 60 °C), the
    range the project takes for an electronics bay; otherwise kept as written.
  - *Heights:* `altitudeM` must climb within 25% of what the pressure gives in the standard
    atmosphere, or the file is refused, as the record's altitude would have no unit. `unfiltAlt`
    is read in meters on the same check, and otherwise kept as written.
  - *Speed:* `velocityMS` must match the altitude's rate of climb, within 25%.
  - *Events:* each rise of `drogueFired` and `mainFired`; the flight-state codes, which no source
    publishes, are kept as numbers.
- **A Fluctus's GPS altitude, worked out from the file.** Silicdyne's Fluctus writes each unit in
  brackets in its header, such as `baro-altitude (m)`, with a decimal comma (*Fluctus – Full
  Documentation* v1.7b, p27, p58), but `gpsAltMSL` has none, in the header or the manual. It is
  read in meters only where the barometric altitude bears it out at the GPS's fresh fixes, the
  rows where its reading changes: at least 10 fixes, spanning at least 120 m (394 ft) of the
  barometric altitude, lying along a straight line (scatter within 10% of that span), whose slope
  is within 10% of 1; in feet it would be 3.28. The 120 m (394 ft) lets two readings each 6 m
  (20 ft) off move the slope by no more than 10%: twice the manual's ±3 m (9.8 ft) horizontal
  figure (p18), as a GPS's vertical error runs about 1.6 times its horizontal. Otherwise, as
  where the GPS lost its fix in the climb or lags the flight, it is kept as written; in all 6
  sample files it is kept. The
  latitude and longitude are degrees, from the manual (p38) and a range check, and each change of
  `status` is an event named as the manual names it (p23).
- **An ARTS2's unit system, worked out from the flight.** The ARTS2's interpreted data comes in
  English or metric units, and neither the file nor the manual (*ARTS2 User Manual* V1.0.0, Ozark
  Aerospace, 2007) says which. So the reader works it out from the flight:
  - *Velocity, and the length unit:* at apogee the drag vanishes, and the logged velocity falls at
    g: 9.81 a second in m/s, 32.17 in ft/s. The reader fits the last second of coast before the
    velocity crosses zero, or, if the acceleration jumps in the 2 s before (a parachute out
    early), the second before the jump. Separately, the barometric altitude moves in steps of one
    sensor count, about 92 Pa (the manual's MPXA4100A sensor and 10-bit converter, p16): 5.5 to
    14.8 m (18 to 49 ft) near any ground, so 17.9 to 48.6 written in feet. The file is read only
    when both give the same unit, which also pins the clock in seconds.
  - *Acceleration:* g, or the velocity's unit per second, whichever makes its integral over the
    boost the velocity's rise, within 10%.
  - *Altitudes:* the inertial altitude `iAlt` must be the velocity's integral, and the barometric
    altitude `pAlt`'s peak within 25% of the climb the velocity gives, from a climb of 176 m
    (577 ft).

  A file whose velocity or barometric altitude no check settles is refused; an acceleration or
  inertial altitude none settles is kept as written. The barometric altitude is the record's
  altitude, above the pad reading the logger took on entering flight mode (p17); the file starts
  at launch. The velocity and inertial altitude are the logger's integrals of its accelerometer,
  and drift by the end of the descent, so the record keeps them as channels, not as its vertical
  speed.
- **AerospaceNU's flight computer board, each unit by two checks.** The flight computer board
  (FCB) of AerospaceNU, Northeastern University's rocketry club, logs a binary file, which the
  club's tools turn into a CSV; the club publishes both (AerospaceNU, *rocket-flight-data*,
  `README.md`, read October 12, 2026). The 20 sample files come under eight headers of 73 to 87
  columns. No document describes the columns, so a unit a column's name doesn't state is read
  only where two checks on the file's values agree. Neither check compares the column with
  another of the same unknown unit, and neither can be met by the weather or the flight in the
  wrong unit. Where they don't agree, the column is kept as written, with a note:
  - *Pressure, in atmospheres:* a pad's pressure lies near 1 in atmospheres or bars, and far
    from it in pascals, hectopascals, psi or inches of mercury. And every value is a whole number
    of pascals divided by 101,325, the club's conversion, a step no flight or weather can mimic;
    in bars the values would miss a step of 1/100,000 by up to half a step. Every barometer that
    reads in the 20 files is in atmospheres; one that reads 0 throughout is kept as written.
  - *Clock, in milliseconds or seconds,* whatever the column's name says: the rows' median step
    must lie within 5 to 500 ms, and the climb the pressure gives must match, within a factor of
    10, the one the accelerometer integrates to on that clock. A file neither unit fits is
    refused, as nothing in it would have a time. Of the 20, `timestamp_ms` holds milliseconds in
    12, and `timestamp_s` holds milliseconds in 6 and seconds in 2.
  - *Accelerations, in m/s² or g:* the board reads 1 g at rest, and each scaled column is its raw
    counts times one of the inertial unit's published sensitivities (*LSM9DS1* data sheet,
    STMicroelectronics, DocID025715 Rev 3, Table 3). The board's own estimate, `acc_*`, is
    checked by the pad's 1 g and the climb instead. One board, the FCBV2 in both its files,
    reads 1.5 g at rest, its counts scaled for another part, and is kept as written, as is the
    high-g accelerometer in every file, whose scale is none of the LSM9DS1's.
  - *Rates of turn, in rad/s:* the gyroscope's turn must match how far gravity's direction turned
    in the accelerometer while the board read about 1 g, and its scale must be one of the
    LSM9DS1's gyroscope sensitivities.
  - *Heights and speed, in meters and m/s:* `pos_z`, the board's filtered altitude above the pad
    and the record's altitude, and the other heights must climb within 25% of the pressure's
    climb and within a factor of 1.8 of the accelerometer's (1.8² is 3.24, under the 3.28 between
    meters and feet). `vel_z` must match `pos_z`'s rate and the accelerometer's speed. `pos_z`
    reads in meters in all 20 files, `gps_alt` in 12 (kept as written in 8) and `vel_z` in 18.
  - *GPS position:* `gps_lat` and `gps_long` are read as NMEA 0183's degrees and minutes run
    together (42.5° written `4230.0`) where some value passes 90 or 180, which no degrees do, and
    their companions `gps_lat_mod` and `gps_long_mod` hold every value converted to degrees. In
    15 files both checks pass. In three later files the "minutes" pass 60, so the values aren't
    degrees and minutes, yet the companions hold them converted as if they were; all four
    columns are kept as written there, as in 2 more.
  - *Kept as written:* the temperatures, which read as a barometer's in degrees Celsius or
    Fahrenheit alike; the battery voltages; the magnetometers; and the raw counts the scaled
    columns are made from. The state codes, in three numberings no source publishes, are kept as
    numbers, so the record takes no events from them.
- **AerospaceNU's ground station text, each unit by two checks.** The same board sends its
  readings by radio, and the club's ground station logs each packet as a line: the time of day it
  arrived, the packet's name and its fields as a Python dictionary, under a `RUN START` line. An
  invented line, shortened:

  ```text
  10:00:01.512345: Position Data {'crc': 'Good', 'time_stamp_ms': 61000, 'fcb_altitude': 0.25, …}
  ```

  No document describes the packets' units, so each unit a name doesn't state is read only where
  two checks on the file's own values agree, neither resting on a column whose unit is as
  unknown; otherwise it is kept as written, with a note. The club's per-flight summaries aren't
  a check: they exist for few flights and needn't match the packets received. In the 4 sample
  files:
  - *Altitude, `fcb_altitude`, the record's altitude:* its climb within 25% of the barometer's
    climb in the standard atmosphere, where feet would be 3.28 times off (a climb further off
    refuses the file); and its coast, from the end of the boost to its highest value, slowing
    within a factor of 1.8 of 1 g, where feet would read 3.28 g or more. Read in meters in all 4:
    climbs 1.006 to 1.090 times the barometer's, coasts slowing 1.08 to 1.28 g.
  - *Vertical speed, `v_speed`:* its integral over the barometer's ascent within 25% of the
    barometer's climb, the speed taken as a straight line across gaps in the reception; and its
    coast, from its highest value to its first at or below 0, slowing within a factor of 1.8 of
    1 g. Neither check uses `fcb_altitude`. In m/s in all 4: the climb 0.909 to 1.004 times the
    integral, coasts slowing 1.00 to 1.47 g.
  - *Accelerometer:* 1 g at rest on the pad, in m/s² rather than g; and, from the launch, its
    force along the pad's direction less 1 g, integrated to a climb, within a factor of 1.8 of the
    barometer's climb (the median 0.997 to 1.090). Read so in 3; kept as written in 1, which
    reads 0 throughout.
  - *Pressures, in atmospheres:* by their size and their whole-pascal step, as in the board's CSV.
  - *Rates of turn, in rad/s:* by the board's turning against the accelerometer's, and by its
    tilt from vertical, which can change no faster than the board turns. Read so in 3; kept as
    written in 1, whose accelerometer isn't read.
  - *Positions, in degrees:* most fixes pass ±π/2 or ±π, which radians can't, and most have a
    fraction of a degree of 0.6 or more, which degrees and minutes run together can't; one stray
    fix decides nothing.
  - *Clock:* the board's `time_stamp_ms`, in milliseconds by its name, must advance 0.8 to
    1.25 s for each second of the ground station's own clock, over at least 20 s. Packets after
    the board restarts are placed by the ground station's clock.
  - *Left out or kept as written:* a packet whose radio checksum failed (its `crc` not `'Good'`)
    is left out and counted, as its bytes can't be trusted. The temperatures, the GPS altitudes
    (only the barometer could check them, one check), the battery voltage and the roll, pitch
    and yaw are kept as written.
- **A phone app's `launch` CSV, each unit by two checks.** An app that no file names logs a
  phone's GPS, accelerometer, gyroscope, magnetometer and, in newer files, barometer, about 20
  rows a second. No description of the format was found. Of the 27 sample files, 19 are an
  older layout of 35 columns with no barometer, and 8 a newer one of 67 or 69 columns that adds
  one. `Seconds` is the clock, by its name. Every other column is read in a unit only where two
  checks that look at different evidence agree; otherwise it is kept as written, with a note. An
  invented start of a file, shortened:

  ```text
  Seconds,Timestamp,AltitudeMSLGPS,Record,Static,Latitude,Longitude,GTotal,Gx,Gy,Gz,…
  0.000,1941868800000,100,1,12,40.123456,-100.234567,1.000000,0.000000,0.932039,0.362358,…
  ```

  - *Pressure, `hPa`:* it names its unit, and its first reading must be a launch site's, 500 to
    1,100 hPa.
  - *Barometric altitude, the record's altitude, in meters:* it must be the pressure altitude of
    `hPa` in the standard atmosphere, which in feet would be 3.28 times as large. And in the
    coast before apogee it must slow at 0.7 g to 0.7 × 3.28 g, as gravity and drag slow a coasting
    rocket by at least 1 g, which an altitude written in feet can't. Read so in 7 of the 8 newer
    files. The barometric speeds are read in m/s by the altitude's rate and the same coast.
  - *Accelerations, in g:* the pad reads 1, and the boost's integral matches the barometric climb
    rate. The older files have no barometer for the second check, so their accelerations are
    kept as written: read in g in 6 files, kept in 21.
  - *Attitude and rates of turn, in radians and rad/s:* `Yaw`, `Roll` and `Pitch` by their range
    and by wrapping at ±π (for the pitch, by gravity's direction in the accelerometer); the
    gyroscope by the attitude's turn and by a full scale a phone's gyroscope offers. The
    gyroscope reads in rad/s in 7 files and is kept as written in 20.
  - *Position, heading and time stamp:* degrees, as some value passes what radians reach; the
    heading also wraps at 360; `Timestamp` is a Unix time in milliseconds, its steps the clock's
    and its date in this century.
  - *GPS altitude, kept as written in all 27:* only one check reaches it, its gain against the
    pressure altitude. Fixes once a second can't show the coast, and on the pad the weather moves
    a pressure altitude further than meters and feet are apart below 4,000 m (13,123 ft).
- **An Eggtimer's download saved as RTF, unwrapped.** One sample file is an Eggtimer TRS's
  terminal download that a text editor saved as an RTF document (Rich Text Format, a word
  processor's markup). The reader takes the document's plain text, following the parts of the RTF
  specification (Microsoft, version 1.9.1, 2008) a saved capture needs: groups, control words and
  escaped characters, with fonts, colors and pictures skipped. It reads that text as it reads a
  terminal download (above). A document holding several downloads gives the first; the rest are
  counted in a note. In the sample file, the table's clock is in milliseconds by its step and its
  heights in feet by the manuals; its speeds, which don't follow the altitude's change, are kept
  as written, as are the summary's apogee and site elevation, which carry no unit mark the reader
  knows. An RTF document holding anything else is refused as not an Eggtimer's, among them the
  radio telemetry stream in two other sample files, which comes with
  [M7.1c5](../decisions-and-roadmap.md#m7-1c5).

The record's readings withhold on all these layouts, as on every logger's but PerfectFlite's
`.pf2`, until [M7.2](../decisions-and-roadmap.md#m7-2).

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
pinned, read but counted apart as not a log. And 62 files the catalog files as generic CSV, by
their extension, are in AltimeterCloud's layout (10), Entacore AIM's (15), a Fluctus's (6), an
ARTS2's interpreted data (4) or the phone app's `launch` CSV (27), and are held to those readers.
Of the 3 RTF files the catalog files as not a log, by their extension, one is an Eggtimer's
download, held to its reader, and two a radio telemetry stream, refused until
[M7.1c5](../decisions-and-roadmap.md#m7-1c5). Four more are moved to layouts not read yet: three
of an ARTS2's raw sensor counts, which carry no calibration, and a ground station's conversion
of a SillyGoose flight, its columns in another order under no header.

The collection is other people's data, Debrief's fixtures come from flights shared on terms nobody
recorded, and AerospaceNU's are published under the GPL, so no file and no value read from one is
committed. The digest pins each reading instead, as a snapshot (a stored copy of a test's output)
would: the check fails if a reader's output changes or a file of these layouts stops reading.
Invented files in the crate's tests are read in CI and pinned whole by snapshots.
