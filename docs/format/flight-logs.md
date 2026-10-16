# Flight logs: the record and the layouts read

See which altimeters' flight logs HPR Sim reads, what it keeps from each, and how that was checked.

An altimeter or flight computer saves its flight as a file: a clock and the readings of its
sensors, such as a barometer's altitude, an accelerometer, a battery's voltage, and, on some, a
GNSS receiver's position. Each maker writes its own layout. HPR Sim reads each layout into one
record, the same for every logger, so that what comes after (the readings, and later the
comparison with a simulation) doesn't depend on which altimeter flew.

> **How far to trust it.** Every sample log the project holds of the layouts below is read,
> except one AltOS export whose header line is damaged, an Eggtimer download whose decimal points
> were taken out, an RRC3 export whose rows were sorted out of order, and 4 sheets inside the
> sample Excel workbooks ([Excel workbooks](#excel-workbooks))
> ([how it was checked](#how-it-was-checked)). No maker publishes a specification of these
> layouts, so each reader follows what the files show, the makers' manuals and the notes of
> Debrief, the project owner's earlier flight-log analyzer, with each unit checked where the
> files allow. Where a header names no unit, the reader decides it from the file and says how in
> a note ([units the file decides](#units-the-file-decides)); the few units no source states are
> named [below](#units-no-source-states). In the four layouts read since
> [M7.1c4](../decisions-and-roadmap.md#m7-1c4), a unit two checks don't settle is kept as written
> (the file's number stored unchanged, with no SI value made from it), so many of their columns keep the file's own numbers, and no reading is taken from them yet.
> The two raw downloads read since [M7.1c5](../decisions-and-roadmap.md#m7-1c5), the Raven's
> `.FIPa` and the RRC3's `.rff`, match their makers' exports of the same flights in every sample,
> wherever an export sits beside them; 7 of their 63 sample files have none, and are unchecked
> ([raw downloads](#raw-downloads)). The Featherweight app's flight summaries, read since
> [M7.1c6](../decisions-and-roadmap.md#m7-1c6), hold the logger's own figures and no samples;
> only their maximum and pad altitudes are checked, against the files of samples beside them
> ([Featherweight's summaries](#featherweights-summaries)). A zip archive of a flight's files,
> unpacked since [M7.1c7](../decisions-and-roadmap.md#m7-1c7), is read file by file, each as if
> it lay on its own ([zip archives](#zip-archives)). The R-DAS's raw text, read since
> [M7.1c8](../decisions-and-roadmap.md#m7-1c8), holds the counts of its converters and no
> calibration, so every count is kept as written and the record holds no altitude
> ([R-DAS raw text](#r-das-raw-text)). AltOS's radio telemetry, read since
> [M7.1c9](../decisions-and-roadmap.md#m7-1c9), follows Altus Metrum's published document,
> but for its pressure unit, which the files contradict; every unit converted is checked a second
> way, and the one sample file with an AltosUI export beside it matches it
> ([AltOS telemetry](#altos-telemetry)). AltOS's raw downloads, the `.eeprom` files read since
> [M7.1c10](../decisions-and-roadmap.md#m7-1c10), follow a layout worked out from the files; each
> unit converted rests on a data sheet and a second check, and all 9 sample files with an AltosUI
> export beside them match it on every value converted to SI, one export's acceleration, zeroed
> at +1 g, counted apart ([AltOS's `.eeprom`](#altoss-eeprom)). An ARTS2's raw data, the `.odf`
> file and its raw CSV export read since [M7.1c11](../decisions-and-roadmap.md#m7-1c11), also
> follows a layout worked out from the files; the `.odf`'s units rest on its own calibration and
> the sensors' data sheets, held to a climb check, and all 3 sample `.odf` files agree with the
> raw CSV and the interpreted export beside them ([the ARTS2's raw data](#the-arts2s-raw-data)).
> Missile Works' RTx `.rtx`, read since [M7.1c12](../decisions-and-roadmap.md#m7-1c12), follows a
> layout worked out from the files, and all 5 sample files agree with mDACS's exports beside them
> on every fix the exports hold; its velocity and speed over ground are kept as written
> ([Missile Works' RTx](#missile-works-rtx)).
> A table from a logger no other reader knows, read since
> [M7.1c13](../decisions-and-roadmap.md#m7-1c13), is read only as far as its header states its
> units, each unit held to the column's name and values; no maker stands behind those units, and
> a header that states a wrong one is read in it
> ([tables read by their headers](#tables-read-by-their-headers)).
> An Excel workbook, read since [M7.1c14](../decisions-and-roadmap.md#m7-1c14), is read sheet by
> sheet, each cell as the workbook stores it; every cell of the 16 sample workbooks matches an
> independent reader's, and 4 of their 28 sheets are refused
> ([Excel workbooks](#excel-workbooks)).
> A table whose header names its clock with no unit is read only by a column mapping someone
> states: since [M7.1c15](../decisions-and-roadmap.md#m7-1c15) (European-locale exports and
> stated mappings), the survey reads six team tables by the mapping RocketPy's example notebooks
> read them with; `import::read` alone still refuses them
> ([mappings a publisher states](#mappings-a-publisher-states)).
> A team flight computer's binary records, from a raw file or a console capture's base64, are read
> by a layout the caller states, since [M7.1c16](../decisions-and-roadmap.md#m7-1c16); in the 5
> sample files every field matches the team's own decoded table
> ([binary records by a stated layout](#binary-records-by-a-stated-layout)).
> A Blue Raven's USB download, read since [M7.1c17](../decisions-and-roadmap.md#m7-1c17) (its
> reader), keeps only the records whose CRC checks, each held to its stated length, and reads them
> by the maker's manual; every
> value in the 2 sample files equals the survey's own decoding, and values the manual doesn't
> list are kept as written ([the Blue Raven's USB download](#the-blue-ravens-usb-download)).
> A course's Teensy data logger, read since [M7.1c18](../decisions-and-roadmap.md#m7-1c18), writes
> one sensor's reading a line; where its file states no unit, the unit read is held to the
> physics of the flight, and the rest is kept as written
> ([a course's Teensy data logger](#a-courses-teensy-data-logger)).
> One team's radio telemetry packets, read since [M7.1c19](../decisions-and-roadmap.md#m7-1c19),
> come in three forms: the flight computer's own log, the text a receiver wrote of what it heard,
> and exports inside an RTF document. Every packet the receiver or an export holds that the
> onboard log also holds, by its stage, kind and clock, matches the log's field for field; the
> pressure and the clock are read in units held to the flight's physics, and the rest is kept as
> written
> ([one team's radio telemetry packets](#one-teams-radio-telemetry-packets)).
> Other layouts are refused, not guessed at; they come with
> [M7.1c20](../decisions-and-roadmap.md#m7-1c20) to
> [M7.1c23](../decisions-and-roadmap.md#m7-1c23), the other loggers' formats.

**To read a flight** from these layouts today, use the Rust library: give the file's bytes to
`hpr_flightdata::import::read`
([API reference](../api/hpr_flightdata/import/index.html)). It recognizes the layout from the
file's content, never its name, and reads it or refuses it, naming the layouts it knows. A file
of binary records has no layout to recognize: read it with `records::read`, or a console capture
of them with `capture::read`, by the layout you state
([below](#binary-records-by-a-stated-layout)):

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
decided, and [M7.1c5](../decisions-and-roadmap.md#m7-1c5) (the Raven's `.FIPa` and the RRC3's
`.rff` raw downloads), as
[ADR-259](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0259-raw-downloads-against-their-makers-exports-first.md)
decided, and [M7.1c6](../decisions-and-roadmap.md#m7-1c6) (Featherweight's summaries, and the
files filed as not logs), as
[ADR-260](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0260-featherweight-summaries-before-the-rest.md)
decided, and [M7.1c7](../decisions-and-roadmap.md#m7-1c7) (zip archives, PerfectFlite's exports
with no header row and Blue Raven files with no date), as
[ADR-261](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0261-archives-and-two-readers-variants-before-the-rest.md)
decided, and [M7.1c8](../decisions-and-roadmap.md#m7-1c8) (the R-DAS's raw text), as
[ADR-262](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0262-the-r-das-raw-text-before-the-rest.md)
decided, and [M7.1c9](../decisions-and-roadmap.md#m7-1c9) (AltOS's radio telemetry), as
[ADR-263](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0263-altos-telemetry-before-the-rest.md)
decided, and [M7.1c10](../decisions-and-roadmap.md#m7-1c10) (AltOS's `.eeprom` downloads), as
[ADR-265](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0265-altos-raw-downloads-before-the-rest.md)
decided, and [M7.1c11](../decisions-and-roadmap.md#m7-1c11) (the ARTS2's `.odf` and raw CSV), as
[ADR-266](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0266-arts2-raw-data-before-the-rest.md)
decided, and [M7.1c12](../decisions-and-roadmap.md#m7-1c12) (Missile Works' RTx), as
[ADR-267](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0267-missile-works-rtx-before-the-rest.md)
decided, and [M7.1c13](../decisions-and-roadmap.md#m7-1c13) (tables read by their headers), as
[ADR-268](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0268-tables-read-by-their-headers-before-the-rest.md)
decided, and [M7.1c14](../decisions-and-roadmap.md#m7-1c14) (Excel workbooks), as
[ADR-269](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0269-excel-workbooks-before-the-rest.md)
decided, and [M7.1c15](../decisions-and-roadmap.md#m7-1c15) (European-locale exports and
mappings a publisher states), as
[ADR-270](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0270-euroc-exports-and-stated-mappings-before-the-rest.md)
decided, and [M7.1c16](../decisions-and-roadmap.md#m7-1c16) (binary records by a stated
layout), as
[ADR-271](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0271-binary-records-by-a-stated-layout-before-the-rest.md)
decided, and [M7.1c17](../decisions-and-roadmap.md#m7-1c17) (the Blue Raven's USB download), as
[ADR-272](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0272-the-blue-ravens-usb-download-before-the-rest.md)
decided.

## The layouts read

| Layout | Loggers | Recognized by | Reader |
|---|---|---|---|
| PerfectFlite `.pf2` | Pnut, StratoLogger, StratoLoggerCF | names PerfectFlite | [`.pf2` files](pf2.md) |
| PerfectFlite CSV export | the same, through PerfectFlite's software | `Time,Altitude,` and more of the export's own column names | [below](#perfectflites-exports) |
| PerfectFlite text export | the same | the same names, separated by tabs; or, with no header row, rows of four tab-separated numbers, the third a temperature followed by its unit's letter, `F` or `C`, such as `71.5F` | [below](#perfectflites-exports) |
| PerfectFlite `.pfa` | Alt15K/WD, an older PerfectFlite altimeter, through PerfectFlite's DataCap program (AltCapWD on the Mac) | names PerfectFlite and has a `**!!Data:` line | [below](#perfectflites-older-pfa) |
| AltOS CSV | Altus Metrum's TeleMetrum, TeleMega, EasyMega, EasyMini, through AltosUI (Altus Metrum's ground software) | `#version,serial,flight,`, or `#` comment lines and then that header or `version,serial,flight,` | `hpr_flightdata::altos` |
| Raven FIP CSV | Featherweight Raven, through the Featherweight Interface Program | opens with a time column and holds the Raven's `Axial Accel (Gs)` channel | `hpr_flightdata::raven` |
| Raven `.FIPa` | Featherweight Raven, its memory as the Featherweight Interface Program downloads it, before any export | opens `#Component=Raven`, and its text lines end, within the first 4,096 bytes, in a `#den` line followed by a frame's `:` | `hpr_flightdata::raven_fipa` ([below](#raw-downloads)) |
| Blue Raven CSV | Featherweight Blue Raven, through its phone app: a low-rate file (barometer, inertial figures, flags) and a high-rate file (gyroscopes, accelerometers, attitude) | `Year,Month,Day,Time,Flight_Time_(s),Sync,`, or `Flight_Time_(s),Sync,` in a file with no date | `hpr_flightdata::blueraven` |
| Blue Raven USB download | Featherweight Blue Raven, its low-rate or high-rate set as its USB port sends it to a serial terminal, one line a sample | a line among the first 1,000 that opens `@ LOG_HIR` or `@ LOG_LOW` and closes with a CRC that checks | `hpr_flightdata::blueraven_usb` ([below](#the-blue-ravens-usb-download)) |
| Teensy data logger, tab-separated | a university course's own logger, built around a Teensy board, its older layout: one sensor's reading a line | its first line is a tag, a whole-number clock and values, by tabs, and one of its first 100 is an `Alt` line with `P` and `MSL` | `hpr_flightdata::teensy` ([below](#a-courses-teensy-data-logger)) |
| Radio packets | one university team's flight computer's radio packets, `255:…:254`: its own log, or the text a receiver wrote of what it heard | its first line is a packet, a whole number or a `Gain[…]` line, and one of its first 100 is a packet | `hpr_flightdata::packets` ([below](#one-teams-radio-telemetry-packets)) |
| Radio packets in RTF | the same packets in an RTF document a spreadsheet saved as CSV, `255,…,254` | an RTF document, one of whose first 100 lines, its markup taken off, is a packet | `hpr_flightdata::packets` ([below](#one-teams-radio-telemetry-packets)) |
| Teensy data logger, named fields | the same course's logger, its newer layout | its first line states a sensor's units, such as `ICM data in us, milli-g, …`, and one of its first 100 is a tag, `t`, a clock and name–value pairs, by commas | `hpr_flightdata::teensy` ([below](#a-courses-teensy-data-logger)) |
| RRC3 CSV | Missile Works RRC3, through mDACS, Missile Works' program that downloads its flights | `Time`, `Altitude`, `Pressure`, `Velocity`, then any of `Temperature`, `Events`, `Voltages`, perhaps with units in brackets; separated by commas, or by semicolons, a European locale's, perhaps ending each line | `hpr_flightdata::rrc3` |
| RRC3 `.rff` | Missile Works RRC3, its memory as mDACS downloads it, before any export | a .NET serialization header (MS-NRBF, version 1.0) and, within the first 4,096 bytes, mDACS's class name `mDACS.AnalysisSupport+FlightRecord` | `hpr_flightdata::rrc3_rff` ([below](#raw-downloads)) |
| Eggtimer CSV | Eggtimer Classic, TRS, Quantum and Proton, through Eggtimer's download | `T,Alt,` and more of Eggtimer's column names, or the same separated by semicolons, as a European locale's spreadsheet saves it | `hpr_flightdata::eggtimer` ([below](#units-the-file-decides)) |
| Eggtimer text | Eggtimer TRS, its download captured by a terminal program | a title `Eggtimer <model> <firmware> … Flight Settings`, then Eggtimer's header row `T,Alt,…` | `hpr_flightdata::eggtimer` ([below](#units-the-file-decides)) |
| Eggtimer RTF | the same terminal download, saved by a text editor as an RTF document | opens with `{\rtf`, and its text holds the terminal download's title and header row | `hpr_flightdata::eggtimer` ([below](#units-the-file-decides)) |
| AltimeterCloud CSV | Rocketry Ltd Nano V1 and Mercury, as a flight page on altimetercloud.com hands the log back | a header within the first 65 lines whose first name is `Time(ms)` (in any case) and which also names `acceleration_total(mG)` and `tilt` | `hpr_flightdata::altimetercloud` ([below](#units-the-file-decides)) |
| Entacore AIM CSV | Entacore AIM XTRA (the flight computer's own log) and AIM BASE (the telemetry its ground station received), as the AIM software's *Export to Excel…* writes them | the first line starts with a `time` column, names at least two, and names one of the AIM's own channels, such as `Pressure AGL`, `lat. X accel.` or `eject. bat. voltage` | `hpr_flightdata::entacore` ([below](#units-the-file-decides)) |
| SillyGoose text | AerospaceNU's SillyGoose flight computer: its own log, tab- or space-separated, with or without a `CONFIG` settings line | a first line, or the one after `CONFIG`, starting `timestampMs` and naming `pressurePa` and `altitudeM` | `hpr_flightdata::sillygoose` ([below](#units-the-file-decides)) |
| Silicdyne Fluctus CSV | Silicdyne Fluctus, as its Fluctus Control Center exports a flight | a `sep=` line (such as `sep=;`) within the first 16 lines, or none, then a header starting `time` that names `deltaTime` and `baro-altitude` | `hpr_flightdata::fluctus` ([below](#units-the-file-decides)) |
| ARTS2 CSV | Ozark Aerospace ARTS2, its interpreted data as its software exports a flight | a first line of `#` and the names `MET Acc Vel iAlt pAlt`, by spaces, tabs or commas, in any case | `hpr_flightdata::arts2` ([below](#units-the-file-decides)) |
| ARTS2 `.odf` | Ozark Aerospace ARTS2, a flight as its software, DataAnalyzer, saves it: the board's settings and calibration, then the sensors' counts | its bytes open with the length 9, then `Ozark.odf` | `hpr_flightdata::arts2_raw` ([below](#the-arts2s-raw-data)) |
| ARTS2 raw CSV | the same, the sensors' counts as DataAnalyzer exports them, with no calibration | a first line that isn't blank reading `MET,Acc(HI),Acc/Baro(LO),Baro(HI),State`, the names as written | `hpr_flightdata::arts2_raw` ([below](#the-arts2s-raw-data)) |
| AerospaceNU board CSV | AerospaceNU's flight computer board (FCB), its binary log as the club's tools turn it into a CSV | a first line whose first name is empty (the club's row number) and which names `packetType`, `timestamp_ms` or `timestamp_s` | `hpr_flightdata::aerospacenu_fcb` ([below](#units-the-file-decides)) |
| AerospaceNU ground station text | the same board's radio downlink, one packet a line, as the club's ground station logged it | a first line that isn't blank starting `RUN START` | `hpr_flightdata::aerospacenu_gs` ([below](#units-the-file-decides)) |
| Phone app `launch` CSV | an app, named in none of its files, that logs a phone's own sensors in flight | a first line that isn't blank starting `Seconds,Timestamp,AltitudeMSLGPS`, the only mark the files carry: any file whose header opens so is read as this app's, as the app's name is unknown | `hpr_flightdata::phone_launch` ([below](#units-the-file-decides)) |
| Featherweight summary CSV | Featherweight Blue Raven and GPS tracker: the flight's summary, as the Featherweight app exports it beside the samples | a first line that isn't blank naming `Rocket Name`, and `Max Altitude` or `Launch time UTC` among the first 200 lines | `hpr_flightdata::fwsummary` ([below](#featherweights-summaries)) |
| Featherweight GPS CSV | Featherweight's GPS tracker: its own log, and its ground station's log of every tracker it hears | a first line opening `UTCTIME,UNIXTIME,` or `TRACKER,DATE,TIME,` that names the latitude and longitude (`LAT` and `LON`, or `TRACKER Lat` and `TRACKER Lon`) | [below](#gps-logs) |
| KML | GPS tracks as AltOS, Missile Works' RTx, Big Red Bee's BeeLine, fw2kml (a converter of Featherweight's logs), GPS Visualizer and others write them for Google Earth | the document's root element is `kml` | [below](#gps-logs) |
| NMEA 0183 | a GPS receiver's own sentences, written to a file as it sends them | at least 3 of the first 50 lines open with a sentence whose checksum checks out, such as `$GPGGA,…*hh`, or, in a shorter file, every line does | [below](#gps-logs) |
| R-DAS raw text | AED Electronics R-DAS, its raw data as its Windows software exports a flight | a first line that isn't blank reading `*** Configuration ***`, and a `*** Raw data ***` line within the first 200 | `hpr_flightdata::rdas` ([below](#r-das-raw-text)) |
| AltOS telemetry | Altus Metrum's TeleMetrum v2 and newer, the packets a TeleDongle receiver hears, as AltosUI saves them to a `.telem` file | a first line that isn't blank reading `TELEM`, a space and an even number of hexadecimal digits | `hpr_flightdata::altos_telem` ([below](#altos-telemetry)) |
| AltOS `.eeprom` | Altus Metrum's TeleMetrum, TeleMega, EasyMega and EasyMini, the board's own log as AltosUI downloads it | a JSON object naming `log_format` and `product`; or, within the first 1,000 lines, a `Config version:` line and a `manufacturer altusmetrum.org` line | `hpr_flightdata::altos_eeprom` ([below](#altoss-eeprom)) |
| RTx `.rtx` | Missile Works' RTx GPS tracker, its fixes as mDACS (Missile Works' program) saves a download | a .NET serialization header (MS-NRBF, version 1.0), then the class record of a list of 32-bit integers | `hpr_flightdata::rtx` ([below](#missile-works-rtx)) |
| RTx CSV and text | the same fixes, as mDACS exports them | a first line that isn't blank reading `UTC Time,Latitude,Longitude,Altitude,Velocity,VDOP,NavStatus,Sats,SOG,COG`, or the text file's tab-separated `UTC`, `Lat`, an empty name, `Lon`, an empty name, `Alt`, `Vel`, `VDOP`, `Nav`, `Sats`, `SOG`, `COG` | `hpr_flightdata::rtx` ([below](#missile-works-rtx)) |
| Table read by its header | a logger no other reader knows: a team's flight computer, a logging script, a spreadsheet saved as CSV | tried after every other layout: a header row naming a clock with a unit of time, such as `Time (s)` or `time__us`, then a row with a number there | `hpr_flightdata::generic` ([below](#tables-read-by-their-headers)) |
| Excel workbook, `.xlsx` | a flyer saving a log, or several, as a spreadsheet | not by `import::read`, which refuses an archive: `workbook::read` opens it, then each sheet is recognized as a file of its own | `hpr_flightdata::workbook` ([below](#excel-workbooks)) |
| Zip archive | not a log: a flight's files, zipped to share them | its bytes open `PK` then 3, 4 | `hpr_flightdata::archive` ([below](#zip-archives)) |
| Binary records | a team's flight computer saving each sample as a fixed run of bytes, with no header | not recognized: read by the record layout you state | `hpr_flightdata::records` ([below](#binary-records-by-a-stated-layout)) |
| Console capture | the same records, dumped as base64 by the flight computer's shell and saved by a terminal program | a start marker line, `----++++//[[( name start )]]\\++++----`, in its first 1,000 lines; `import::read` refuses it as stating no layout | `hpr_flightdata::capture` ([below](#binary-records-by-a-stated-layout)) |

The Blue Raven app writes each flight as a low-rate file, logged 50 times a second, and a
high-rate file, 500 times a second (*Blue Raven User's Guide*, 2026-05-12, p18); each is read on its own, as a record of its own, not joined
to the other. The high-rate file holds no altitude, so its record's
[primary track](#the-record) is empty and its channels hold everything; the low-rate file of the
same flight holds the altitude. The app's summary of the flight is read too, as a record of
figures with no samples ([below](#featherweights-summaries)).

Some files among the sample logs aren't flight logs, and are counted apart as such: Debrief's
made-up sample flights, which say so in their first row; a team's checklists for preparing a
flight; a Blue Raven's deployment settings saved as spreadsheets or CSV; a flyer's summaries of
flights in spreadsheets; the events files a
flyer's own program wrote from a Blue Raven's low-rate file, the same program's copy of a Blue
Raven's summary, and the descriptions of a rocket such programs read; a team flight computer's saved state, one row; an OpenRocket simulation's
export; and a wind profile. The other layouts in the sample logs, among
them an older `.xls` spreadsheet, are refused today, most as *unrecognized*; they come with
[M7.1c20](../decisions-and-roadmap.md#m7-1c20) to
[M7.1c23](../decisions-and-roadmap.md#m7-1c23), and the [survey's report](#how-it-was-checked)
counts them by layout.

### Zip archives

Flyers often share a flight as one zip archive: a Blue Raven's high-rate and low-rate files with
the app's summary, or a folder of everything the flight left behind. An archive isn't a log, so
`hpr_flightdata::import::read` refuses one with a reason that says so. Instead
`hpr_flightdata::archive::unpack` takes its files out, in memory, each a name and its bytes, and
each is read as it would be on its own. Nothing is written to disk, so a file's name, folders
and all, is only a label. An archive inside an archive is one of its files, and isn't opened.

```rust,ignore
let bytes = std::fs::read("flight.zip")?;
for member in hpr_flightdata::archive::unpack(&bytes)? {
    match hpr_flightdata::import::read(&member.bytes) {
        Ok(log) => println!("{}: {} ({} channels)", member.name, log.format.name(), log.channels.len()),
        Err(error) => println!("{}: not read, {error}", member.name),
    }
}
```

An archive comes from someone else, and a small one can be built to unpack to far more than
memory holds (a *zip bomb*). So at most 256 MiB (268 MB) come out of one archive, about eight
times the largest sample archive, which unpacks to 32 MB, counted as the bytes decompress, never
taken from the sizes the archive states. An archive is refused whole, with its reason, and none of
its files is read, when:

- two of its files share a name (the `zip` library HPR Sim uses would silently keep only one);
- a file is a symbolic link, or a folder's entry holds bytes;
- a file is encrypted;
- a file is compressed by a method other than the two that archives of logs use, *stored* (not
  compressed) and *deflated* (zip's usual compression);
- a file fails its CRC-32, the checksum every zip file carries to catch damage.

The crate's tests build an archive of each kind and check its refusal, and two property tests
feed random and damaged archives to the reader to show it refuses them or stays within the
bound, and never crashes. Only the Rust library opens archives so far: `hpr analyze` reads a
`.pf2` file, not a zip.

The 8 archives among the sample logs hold 29 files. 10 are byte-for-byte copies of files already
among the samples. The other 19 are 8 Blue Raven high-rate and low-rate files, 3 of the app's
summaries, a GPS tracker's KML track, and 7 files that aren't logs; every one of the 12 logs
reads. The 3 summaries are checked against the Blue Raven files in their archive, as the others
are ([checked against the files beside](#checked-against-the-files-beside)), and all 3 agree.
Two CSV files in one archive name other altimeters' formats in their file names and use the Blue
Raven's column names, but their values aren't the Blue Raven's: at the same moments, their
altitudes differ from its low-rate file's by up to several hundred feet. Which logger wrote them
isn't known, so they aren't read as a Blue Raven's; since
[M7.1c13](../decisions-and-roadmap.md#m7-1c13) they are read as
[tables by their headers](#tables-read-by-their-headers), their altitude in the feet the header
states and the rest of their columns as written.

### Excel workbooks

Some flyers keep a flight's log in an Excel workbook (`.xlsx`): the Blue Raven app's files pasted
side by side as sheets, or a table whose header names its units. Only the Rust library reads
workbooks so far; `hpr analyze` and the Python package don't open them. A workbook is a zip
archive of XML parts (ECMA-376, *Office Open XML*, Part 1, §18), so
`hpr_flightdata::import::read` refuses one, as it does any archive. Instead,
`hpr_flightdata::workbook::read` writes each worksheet's cells out as comma-separated text, one
line a row, and reads each sheet as the file it came from: a Blue Raven's CSV or its app's
summary, an Eggtimer's CSV, a Featherweight tracker's, or a
[table read by its header](#tables-read-by-their-headers). One workbook can give several logs.
A sheet that doesn't read is reported with its reason, and the other sheets are still read.

```rust,ignore
let bytes = std::fs::read("flight.xlsx")?;
for sheet in hpr_flightdata::workbook::read(&bytes)? {
    match &sheet.log {
        Ok(log) => println!("{}: {}", sheet.name, log.format.name()),
        Err(error) => println!("{}: not read, {error}", sheet.name),
    }
}
```

**What a cell becomes.** A cell is written as the workbook stores it, never as a spreadsheet
displays it, so no digit is lost or invented: a number as its stored digits (`0.10000000000000001`
stays so, though Excel shows `0.1`), a string as its text, a formula as the value saved with it,
`TRUE` or `FALSE` for a truth value. An error such as `#DIV/0!`, or a formula with no value saved,
is left empty, and the log's notes count them. A line break inside a cell, as in a header wrapped
onto two lines, becomes a space. Every row is written as wide as the sheet's widest, so short rows
get empty cells; a row with no value is left out.

**Dates.** A spreadsheet stores a date as a count of days, a *serial number*, and shows it as a
date only through the cell's *number format*, the display pattern attached to it, such as
`m/d/yy h:mm`. A number under a format that shows a date is written as one: serial 45,000.75
under `m/d/yy h:mm` is `2023-03-15 18:00:00`, 45,000 days after 1899-12-30 and then three
quarters of a day. Under a format that shows only a time, a number under 1 is written as a time of
day, `18:00:00`; one that also holds a day is written as a date and a time. An elapsed time, such
as `[h]:mm:ss`, counts its hours past 24 (`36:00:00`). Seconds are rounded to the millisecond. A
workbook that counts from 1904, as old Mac workbooks may, is read from 1904; no sample workbook
does, so that path is checked by the crate's tests alone. A date before 1900-03-01, where Excel's
calendar holds a 29 February 1900 that never was, or after 9999-12-31, is kept as the number
stored, and the notes count it. Every other number is kept as stored, so a clock a logger wrote in
seconds stays a number.

**Bounds.** A workbook comes from someone else. Its parts come out within the
[archive bound](#zip-archives), and at most 256 MiB of text is kept from one workbook, its list of
shared strings (the text cells share, stored once) and the text written, each byte counted before
it is kept. So a small file of one long string repeated in every cell is refused, not written
out. The sheets are read as a stream: the largest sample sheet is 60 MB of XML. A workbook is
refused, with its reason, when:

- a part it names is missing or isn't well-formed XML;
- a part declares a document type or names an entity of its own (a way to make a small file
  expand);
- cells or rows come out of order, past Excel's last cell, `XFD1048576`, or a cell holds two
  values;
- a cell holds what its type says it can't, such as a number that isn't one;
- two sheets share one part.

The crate's tests check each refusal and the bound, and a property test writes random cells and
reads them back.

**How it was checked.** The 16 sample workbooks of flights' logs hold 28 sheets:

| Outcome | Sheets |
|---|---:|
| read as a Blue Raven's CSV | 6 |
| read as the Featherweight app's summary of a Blue Raven's flight | 2 |
| read as a Featherweight tracker's CSV | 2 |
| read as an Eggtimer's CSV | 1 |
| read as a table by its header | 10 |
| no value, only a chart | 3 |
| refused: an Eggtimer table a person added a column with no name to, and three layouts no reader knows yet (a sheet of the flight's peak figures, an OpenRocket simulation's export, a simulation set beside the flight) | 4 |

The workbook reader's text was checked against openpyxl 3.1.5, an independent Python reader of
workbooks (`validation/oracles/xlsx/cells.py`): every one of the 5,743,747 cells of the 16 that
hold a value agrees, 147,958 of them dates and times. That checks the copying, not the layouts:
each sheet's columns and units are only as right as the reader it goes to. A probe run when the
check was built, not committed, moved the reader's calendar by one day, and 27,103 cells
disagreed. Not read yet, with [M7.1c20](../decisions-and-roadmap.md#m7-1c20) and
[M7.1c22](../decisions-and-roadmap.md#m7-1c22): four workbooks (a
PerfectFlite export with a person's notes beside it, a state estimator's output whose clock names
no unit, and two tables with no header row) and an older `.xls` file, a different binary format.
Three more workbooks are flyers' summaries of flights, not logs.

### R-DAS raw text

HPR Sim reads the R-DAS's raw export but converts nothing in it: the record holds the file's
counts, no altitude and no events, and only the Rust library reads it so far. The R-DAS, a
flight computer AED Electronics sold in the early 2000s, reads its accelerometer, its barometer
and, as its owner chooses, four digital and six analog inputs, up to 200 times a second, keeping
the 2 s before liftoff (*R-DAS User Manual*, V1.5, 2001, p. 6, p. 7, p. 14). Its Windows software
exports a flight as text, raw or converted with calibration values held in the software. The
sample logs hold the raw export only: 54 files from a college course's flights in 2008, 28
recorded on board at 200 Hz and 26 at 10 Hz, by their names the course's telemetry records.

The raw file opens with a summary of the configuration, then a table of counts: each a 10-bit
converter's whole-number reading of 0 to 5 V, 0 to 1,023, a step 4.883 mV, to be converted "with
the calibration values" (p. 14). The file holds none:

- **`Time`** counts seconds from liftoff (the manual's figure 2.2, p. 10); every sample file
  starts at −2.000. The seconds rest on the configuration's `Data is sampled at N Hz.`: the
  clock's average step must be that interval within 1%, or the file is refused. In every sample
  file it is exact.
- **`Acc`, `Prs` and `ADC0` to `ADC5`** are kept as written, in no unit HPR Sim knows; a value
  that isn't a whole count refuses the file, as a converted export would. The manual's R-DAS has an
  ADXL150 accelerometer (±50 g) and an MPXS4100A barometer, resolved to 0.09 kPa (p. 7), but the
  file doesn't say which board or sensors wrote it, and others have been fitted (a college
  course's [pressure lab](https://pages.hmc.edu/spjut/NewE80/PresTempLab.html) names an
  MPXH6115A), so no data sheet is applied.
- **`Dig`** is read as a hexadecimal word, such as `00B0` (176), and kept as its number. The
  manual says it holds the digital inputs and the flight's phase, and that "bit 3 is set when
  timed recovery was used" (p. 14, p. 22), but the files write a 16-bit word that never sets bit
  3 of either byte. In 27 of the 28 on-board sample files it reads `0000`, then `0010` within
  0.25 s after the zero, `0020`, `0030` and, near the end, `8030`; the 28th has `00B0` and `80B0`
  for the last two. Unconfirmed as the phases, no event is taken from it.

The configuration must state the rate and list the inputs once each, and the table's header must
name them in order (`Acceleration` as `Acc`, `Pressure` as `Prs`, `the digital inputs` as `Dig`),
or the file is refused. Its lines are kept in a note, and its firmware version is the record's.
The record's [primary track](#the-record) is empty, as for the Raven's `.FIPa`, and every column
is a channel. For example, the file the crate's tests read, invented, eleven rows at 10 Hz from
−0.3 s, one of them shown:

```text
Data is sampled at 10 Hz.
Inputs that were sampled: Acceleration, Pressure, the digital inputs, ADC0, ADC1.
*** Raw data ***
   Time  Acc  Prs  Dig ADC0 ADC1
  0.100  700  799 0010  505  252
```

That row is `Acc` 700 and `Prs` 799 counts and `Dig` 16, at 0.1 s; the note says `Time` is in
seconds because its average step, from −0.3 s to 0.7 s over ten steps, is 0.1, the 10 Hz interval.

Checked: all 54 sample files read, pinned by the [survey](#how-it-was-checked). Left out:
converting the counts, which needs the software's calibration values, and events from `Dig`; the
readings of [M7.2](../decisions-and-roadmap.md#m7-2) need an altitude this record doesn't hold.

### AltOS telemetry

HPR Sim reads the telemetry an Altus Metrum flight computer sends by radio, as AltosUI, Altus
Metrum's ground software, saves it; only the Rust library reads it so far. It is checked on two
sample files, two flights of one team's TeleMetrum, one of them with AltosUI's export (its CSV)
of the board's own log of the same flight beside it, and it follows the maker's published
description of the packets (*AltOS Telemetry*, Bdale Garbee and Keith Packard, v1.9.22.2, 2025,
§2–4) but for one unit, the pressure's, which the files contradict.

The board sends a 32-byte packet several times a second, each of a type: sensors, GPS fix,
satellites, configuration, calibration. A TeleDongle,
the ground station's receiver, writes each packet it hears as a text line, `TELEM` and the bytes
in hexadecimal, and AltosUI keeps them all in a `.telem` file (*The Altus Metrum System*, §12.1).
Each line holds a length byte, the packet, two bytes the radio adds, the signal strength (RSSI,
in dBm as $\mathrm{RSSI}/2 - 74$) and a link-quality byte whose top bit says whether the packet passed
the radio's CRC, its error check, and last a checksum: 0x5A plus every byte after the length byte
up to, not including, the checksum, modulo 256. The document's table places the bytes one
position early, and its formula as written fails its own example, while this reading passes it
(a test reads the document's example line) and every line of both files. A line that isn't such a
packet, or fails its checksum, refuses the file, as damage to it. A packet that failed the radio's
CRC is noise and is left out, counted: 13.6% and 4.1% of the two files' packets
([the survey's report](#how-it-was-checked)).

- **The clock** is the board's tick, a hundredth of a second, which turns over every 655.36 s; each
  packet's tick goes on the turn nearest the latest so far, so a GPS packet, which carries the
  earlier tick of its fix, falls behind instead of a turn ahead. It counts from the first packet
  heard, not from liftoff.
- **A board switched off and on** starts its tick again: each switching on is a power-on, and one
  sample file holds three. The record keeps the one that leaves the pad, a state from `boost` to
  `main` (or the last, if none does), counts the others' packets in a note, and refuses a file in
  which two leave it. The tick alone can't tell every case: a radio silence of 327.68 s to
  655.36 s reads as a power-on, a longer one as a turn shorter than it was, and a power-on whose
  new tick lands ahead of the old one as a silence.
- **Pressure is in pascals,** not the document's tenths: a launch site's pressure in pascals is
  at most 12 kPa in tenths, more than 15 km (9.3 mi) up. Of the 138 sensor packets at a tick the
  export also holds, 123 equal its pressure; at the other 15 the export's row repeats the row
  before it, counted apart. Pascals are taken only where the pad's pressure lies from 10 to
  120 kPa so read; otherwise the field is kept as written. One file, from one firmware, has an
  export to hold it to.
- **The board's own estimates**, acceleration and speed in sixteenths of a m/s² and of a m/s,
  height in meters above the pad, are checked against each other: the median ratio of the
  height's climb to the speed after burnout is 0.966 to 1.001, of the speed's change to the
  acceleration in the climb 0.939 to 1.021 (a wrong unit would be 3.28 or 16 times off; the survey
  fails a file outside 0.8 to 1.25). The highest height is 0.996 of the export's.
- **The temperature**, in hundredths of a °C, agrees with the export within its print step,
  0.05 K. **GPS** positions, in ten-millionths of a degree, agree within 0.000 06° at the 39
  values compared, and altitudes within 3 m (9.8 ft); the survey fails a file past 0.01° or
  100 m (328 ft). A GPS packet without its valid bit holds no fix, and its position is a gap.
- **Kept as written:** the converters' counts (raw accelerometer, battery, continuity,
  calibration), which the document gives no conversion for, and the GPS fields nothing checks.
- **States** become events, named as AltosUI's exports name the numbers (2 is `pad`, 3 `boost`, 4
  `fast`, 5 `coast`, 6 `drogue`, 7 `main`, 8 `landed`), which the document doesn't give.

Only the five packet types the sample files hold are decoded: the sensors and calibration of the
TeleMetrum v2 and newer, the configuration, and the two GPS packets. The document lays out eleven
more, for the TeleMega, the TeleMini and older boards; with its pressure unit wrong where the files
can check it, those aren't decoded until a sample holds one, and a packet of one is counted in a
note. The record's [primary track](#the-record) is the board's height, its own estimate, which
may lean on its accelerometer, not a conversion of the pressure alone; every other field is a
channel on its own packets' clock. The crate's tests read an invented file in which a board is
switched on twice, a packet fails the radio's CRC, and a GPS packet's tick falls behind the
sensor packet sent before it. One of its lines, a configuration packet:

```text
TELEM 22d2040001040b0300011b0000fa0000084e3043414c4c0000312e392e32320000f4a8c1
```

| Bytes | Field | Value |
|---|---|---|
| `22` | length | 34: the packet and the radio's two bytes |
| `d204` | serial number, low byte first | 1234 |
| `0001` | tick | 256, so 2.56 s |
| `04` | packet type | configuration |
| `0b` | device type | 11 |
| `0300` | flight | 3 |
| `01` `1b` | configuration version | 1.27 |
| `0000` | apogee delay | 0 s |
| `fa00` | main deployment altitude | 250 m (820 ft) |
| `0008` | flight log's size | 2,048 kB |
| the next 8 bytes | call sign, in ASCII | `N0CALL` |
| the 8 after them | firmware, in ASCII | `1.9.22` |
| `f4` | RSSI | −12, so −12 / 2 − 74 = −80 dBm |
| `a8` | link quality | top bit set, the packet passed the CRC; 40 |
| `c1` | checksum | 0x5A plus the 34 bytes from `d2` to `a8`, modulo 256 |

Left out: the eleven packet types no sample holds; and the GPS fields nothing checks. The
board's own log, which the document doesn't describe, is read from AltosUI's `.eeprom` files
([AltOS's `.eeprom`](#altoss-eeprom)).

### Tables read by their headers

Teams' own flight computers, logging scripts and spreadsheets saved as CSV write tables no maker
documents: a header row naming the columns, then one row a sample. `hpr_flightdata::generic`
reads such a table by what its header states and no more, after every other layout has passed
on it
([ADR-268, tables read by their headers](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0268-tables-read-by-their-headers-before-the-rest.md)).

> **How far to trust it.** All 31 such sample tables are read: besides the clocks, 224 columns
> in a unit the header states, 364 kept as written by their headers, 5 kept as written by their
> values and 11 of text. No maker stands behind these units: a wrong unit whose values stay
> inside the bounds below is read in it, and milliseconds headed as seconds pass the clock's
> check above about 17 rows a second. The table that writes its altitude and speed in both feet
> and meters agrees with itself to 4.5 × 10⁻¹³ m. No readings are taken from these records until
> [M7.2](../decisions-and-roadmap.md#m7-2).

**The header** is the first line within 64 that isn't blank, a `#` comment or `sep=`, split at
`,`, `;`, a tab or `|`, whichever gives the most cells. One column must be the clock: a time word
such as `time` or `t` with a unit of time. A unit is read from brackets at a name's end,
`Alt AGL (ft)`, or from an underscored ending, `acc_y__m_s2`; the
[API reference](../api/hpr_flightdata/generic/index.html) lists every spelling. A decimal comma
is read only where the separator isn't a comma, no number has a point, and some comma stands
before other than three digits (`0,05`); a cell in the other mark, as `1,250` among points, is a
gap, never a number a thousand times too small.

**What a column is.** A column is kept as written, with a note saying why, when its name says
another quantity than its unit does (`vz_filtered_ms`: a speed's name, a time's unit), an error or
a difference (`Alt error (m)`, `Diff press (Pa)`), or nothing at all over a length, a time or a
unit that may stand for another (`g` is also grams and gauss; single letters): `z (m)` may be a
height or a distance. A length is a GNSS, barometric
([AGL](../glossary.md#agl-above-ground-level) or [MSL](../glossary.md#height-above-sea-level-msl))
or inertial altitude where the name says which, and otherwise an *altitude* of unstated sensor and
zero. A column of words whose name says a state, phase, status, event or mode gives an
[event](#the-record) at each change.

**The values' check.** A column is also kept as written if any value, converted, lies outside
what a flight logger can read:

| Quantity | Allowed | Why |
|---|---|---|
| pressure | 0 to 110 kPa | a little over the highest sea-level pressure on record, 108.38 kPa (Agata, 1968, WMO's archive of weather extremes) |
| temperature | −100 °C to 150 °C | past −40 °C to 125 °C, the automotive grade 1 range (AEC-Q100) |
| length | within 150 km (93 mi) | above any hobby rocket's flight |
| speed | within 3,000 m/s (9,843 ft/s) | more than twice Mach 3.5, the simulator's limit |
| acceleration | within 1,000 g | a generous bound, no source |
| rate of turn | within 50,000 °/s | a generous bound, no source |
| voltage, current | within 100 V, 100 A | a generous bound, no source |
| latitude, longitude | within 90°, 180° | the globe's |

Five sample columns failed it: two headed in pascals read above 110 kPa on the pad, and three
headed in amperes above 100 A, milliamperes by their size. The clock's median step must lie from
0.1 ms to 60 s, and it must step forward, or the file is refused.

**A worked example.** An invented table:

```text
Time (s),Alt AGL (ft),Pressure (hPa),Velocity,Phase
0.00,0,1001.2,0,PAD
0.05,12,1000.8,45,BOOST
```

`Time (s)` is the clock. `Alt AGL (ft)` is an *altitude* (above ground, but no sensor named):
12 ft is $12 \times 0.3048 = 3.6576$ m. 1000.8 hPa is 100,080 Pa, inside the bounds. `Velocity`
states no unit, so its 45 stays as written. `Phase` gives two events, `PAD` at 0 s and `BOOST` at
0.05 s.

**A column mapping.** `generic::mapping` returns the header's reading as a `Mapping`, which
serializes to JSON; `generic::read_with` reads a file by one, with the same checks, so a person
can state a unit the header leaves out or name the columns of a file with no header row. A unit
that doesn't measure its column's quantity is refused. Only the Rust library reads these tables
so far.

**What is left out.** No [primary track](#the-record): which column is a barometric altitude
above the pad isn't stated. Most of the 21 files filed as generic CSV that are still refused have
a clock with no unit or no header row, and no publisher states their mapping; they wait, with
[M7.1c21](../decisions-and-roadmap.md#m7-1c21) and [M7.1c22](../decisions-and-roadmap.md#m7-1c22). A table saved in an Excel workbook is read the
same way, once its cells are written out ([Excel workbooks](#excel-workbooks)).

### Mappings a publisher states

Some tables name their clock `t`, `ts` or `time` with no unit, and their columns with no unit
either, so the header reader can't take them, and `import::read` refuses them. When whoever
publishes such a file states what each column holds and in what unit, the survey of the sample
logs reads the file by that statement: `validation/log-mappings.json` holds, for each file by its
id, the `Mapping` as `generic::read_with` takes it and where it is stated. To read such a file
yourself, write a `Mapping` the same way (`generic::mapping` gives one to start from, for a file
whose header names its clock's unit) and pass it to `generic::read_with`
([ADR-270](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0270-euroc-exports-and-stated-mappings-before-the-rest.md), which decided it).

Six of the sample logs are teams' flights that RocketPy, the open-source simulator, keeps in its
repository. RocketPy's example notebook for each flight reads the file with its
`FlightDataImporter`, naming the columns (`"ts": "time"`, `"filtered_altitude_AGL":
"altitude"`, `"filtered_acceleration": "az"`) and passing `units=None`, which the importer's own
documentation defines as every column in SI. That is RocketPy's use of the file, the only
statement of its units there is, not the logging team's own word. So each file's clock is read in
seconds and its altitude above the ground in meters, as an `altitude` whose sensor isn't stated
(barometer or GPS), a vertical speed in m/s, and a vertical acceleration in m/s²: the
acceleration of the rocket's motion, about 0 at rest on the pad and −9.8 m/s² (−32.2 ft/s²) in
free fall, not an accelerometer's reading. Two of the files write that acceleration as 0 from
about apogee on, and it is kept as written. A column the notebook doesn't map, such as a
latitude, is kept as written. Each unit still goes through the values' check above, and none of
the six files' columns fails it.

The survey refuses a mapping on a file whose header doesn't name the mapping's columns in order,
on a file a reader reads on its own (a mapping can't stand in for a reader that takes the file),
and for a file of this family with none stated. A mapping is only as right as its publisher's statement: the
values' check catches a unit far off, a pressure over 110 kPa or a length beyond 150 km (93 mi), not feet
read as meters. A seventh RocketPy table, a GPS log, is loaded by no notebook and is left unread.

### Binary records by a stated layout

Some teams' flight computers save each sample as a fixed run of bytes, such as a 4-byte clock in
milliseconds then 24 four-byte floats, and nothing in the file says so: no header, no names, no
marks. Such a file can't be recognized, only read by a layout someone states. The library reads it
by the layout you give, each field's name and type in the record's order
([`records::RecordLayout`](../api/hpr_flightdata/records/struct.RecordLayout.html)), and never
guesses one. A type is an integer of 8, 16, 32 or 64 bits, signed or not, or a 32-bit or 64-bit
IEEE 754 float (binary32 or binary64, the standard's names for single and double precision),
little-endian (the least significant byte first, as most microcontrollers store numbers) unless
the layout's `byte_order` says
[`ByteOrder::Big`](../api/hpr_flightdata/records/enum.ByteOrder.html). A layout has 2 to 512
fields, a clock and at least one more, each name of 1 to 256 characters, no two alike, none a
number, holding a comma, a semicolon, `|`, a quotation mark or a control character (a line break
or a tab among them), or with spaces at its ends, and the first not opening with `#`, `sep=` or a
byte order mark; a layout outside these is refused (`LogError::Layout`), naming the field and
why. The
records are written out as a table, the names its header and one row a record, and read by the
[header reader](#tables-read-by-their-headers): name a field with its unit, such as
`pressure__kpa` or `Time (ms)`, and it is read in that unit if its values bear it out; a field
whose name states none is kept as written. A layout whose names state no clock with a unit of
time is refused; `records::table` gives the table for reading it by a
[column mapping](#mappings-a-publisher-states) instead
([ADR-271](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0271-binary-records-by-a-stated-layout-before-the-rest.md),
which decided it).

A 32-bit float is widened to 64 bits exactly and written as the shortest decimal that reads back
to it, so a pressure stored as the float nearest 101.3 kPa is read as 101.30000305175781 kPa, the
value the bytes hold, not as 101.3. A float that isn't a number or is infinite becomes a gap, and a
note (one of the record's [notes](#the-record) on what the reader worked around) counts them.

```rust,ignore
use hpr_flightdata::records::{Field, FieldKind, RecordLayout};
let layout = RecordLayout::new(vec![
    Field::new("Time (ms)", FieldKind::U32),
    Field::new("pressure__kpa", FieldKind::F32),
]);
let log = hpr_flightdata::records::read(&std::fs::read("flight.bin")?, &layout)?;
// The same records, dumped as base64 in a console capture's block named `data`:
let text = std::fs::read_to_string("capture.txt")?;
let log = hpr_flightdata::capture::read(&text, "data", &layout)?;
```

The [module's own example](../api/hpr_flightdata/records/index.html) reads two invented records
and is compiled and run with the library's tests.

A logger saving to flash leaves the memory it never wrote erased, every byte `0xFF`, and a dump of
its memory may run on into it. Records that are wholly `0xFF` at the end of the file are dropped as
that erased memory, as are erased bytes past the last whole record, and a note counts both. Any
other bytes past the last whole record are refused, as a sign the layout doesn't fit. An erased
record with written ones after it isn't dropped, so a logger's fault stays visible: its integers
read at their largest (−1 if signed) and its floats as gaps. With a float clock that record has
no time, and the reading is refused, naming the record by its number and where it starts; with
an integer clock it is read: a `u32` clock in milliseconds reads 4,294,967.295 s at that record,
its floats are gaps, and the records after it read at their own times, so the jump shows where
the logger skipped.

The same records may come as a **console capture**: what a terminal program saves when a flight
computer's USB shell is asked to dump its memory, the commands typed at it and then each
dump as base64 lines (bytes written as text, 3 bytes to 4 characters) between two marker lines the firmware prints,
`----++++//[[( data start )]]\\++++----` and `----++++//[[( data end )]]\\++++----`, `data` being
the block's name (the sample captures' blocks are `data` and `params`).
`import::read` recognizes a capture by a start marker in its first 1,000 lines, and refuses it,
naming its first blocks, as the capture doesn't state its records' layout.
[`capture::read`](../api/hpr_flightdata/capture/fn.read.html) reads a named block by the layout
you give, as `records::read` reads a file. The base64 is decoded strictly (RFC 4648's standard alphabet); a
character outside it, padding out of place or over bits that aren't zero, a block that never ends,
a marker inside a block or two blocks of one name are refused, naming the line.

> **How far to trust it.** The reading is exact for the bytes and types you state; whether they
> are the logger's is your statement, so rely on the logger's own code or decoded table for the
> layout. The sample logs hold 5 such files, 3 console captures and 2 raw files, each beside the
> table its team decoded from the same records. The survey reads each by the types
> `validation/log-layouts.json` states for it, its names from that table, then decodes every
> record's fields itself from the file's bytes, apart from the reader's records, and compares them
> with the table field by field (a wrong base64 decoding would fail it too): all 931,875 fields of
> the 45,915 records written agree, and each record read holds the same channels as the team's
> table read by its header. The types aren't the teams' own statement: no team documents them, so
> each was found from the bytes and from that same table. The match shows the layout decodes every
> field exactly as the team did; it doesn't show the layout is the logger's own. The files are
> private, so the layouts file holds only their types, by file id. One capture's last 12,222 records are erased memory, which the team's
> table writes as rows of `4294967295` and `nan` and the reader drops; their fields agree with
> the table too, 1,237,425 fields in all.

**What is left out.** No [primary track](#the-record), as for any table read by its header.
`hpr analyze` and the Python package don't read these files yet: the Rust library does. A
record's layout can't be found from its bytes alone: a file whose layout no one states stays
unread. A capture's other blocks, such as the saved state a `params` block holds, are decoded but
not read.

### The Blue Raven's USB download

A Blue Raven can also send its flight data out of its USB port as text, one line a sample, which
a terminal program saves. HPR Sim keeps only the lines whose own check value shows them whole, and
reads their fields by the maker's manual wherever a line holds what the manual lists.

The manual describes the download (*Blue Raven User's Guide*, Featherweight, updated May 12, 2026,
Appendix A, p22–24). Typed at the logger, `<download l>` sends the low-rate set, recorded 50 times
a second, and `<download h>` the high-rate set, 500 times a second. Each line opens with its
mnemonic, `@ LOG_LOW` or `@ LOG_HIR`, then its length, the logger's date and time of day, a sync
code and the data, and closes with `CRC:` and four hexadecimal digits. The sync code is "a
millisecond counter recorded in both the low-rate and high-rate data so that the two data sets can
be aligned", rolling over every 250 ms (p23). The CRC, a cyclic redundancy check, is a check value
worked out from the line's text, so a line that lost or changed a character no longer matches it:
the manual names it for the low-rate line (p23), and the sample files' high-rate lines carry one
of the same kind. `import::read` recognizes a download by a line among its first
1,000 that opens with either mnemonic, holds no other `@ LOG_`, and whose CRC checks, and reads it with `hpr_flightdata::blueraven_usb`
([ADR-272, reading the Blue Raven's USB download](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0272-the-blue-ravens-usb-download-before-the-rest.md),
which decided it). Each set is a record of its own, as the app's two files are.

Two invented records, one of each kind, as the crate's tests read them, each from a file of its
own:

```text
@ LOG_HIR  139 2026  5 23 10:15:00.120   20    -15     42     -7     -1      3     98  -3000   1500    -40  29812      0      0 CRC: 7730
```

```text
@ LOG_LOW  211 2026  5 23 10:15:00.140  40 Bo:  7212 49655 V:  4105    22    21    20    23 Vel:     0     0     0 Pos:     0     0     0 ang:   12    5    0 FER:  400  1c1   c3   c1   c3   0   0   0 CRC: F7EB
```

> **How far to trust it.** The layout and the units are the maker's, from its manual, and every
> value in the 2 sample files equals the survey's own decoding of them; three physical checks hold
> parts of the reading to the files ([below](#how-the-usb-download-was-checked)), and the other
> fields rest on the manual's scales alone. The sample files' firmware writes more or fewer values
> than the manual lists in some places;
> those values are kept as written, not guessed at. The 2 sample files record no flight: neither
> their pressure nor their acceleration shows a launch, though the TeleMega in the same rocket
> records one, and the files don't say why. So they check how the download is read, not how
> a Blue Raven's channels follow a flight.

**Each line's two checks.** A terminal saves what the port sends, and a capture can lose part of
it: in one sample file, a record whose `CRC:` and line end were lost runs on into the next record,
on one line. So the reader cuts every line before each `@ LOG_`, and keeps a piece only if two
things the logger wrote for it agree with it:

- *Its CRC*: CRC-16/BUYPASS, as the manual names it (polynomial `0x8005`, starting from 0, with no
  reflection and no final XOR; its value for the text `123456789` is `0xFEE8`). It covers the text
  from `@` to the last character before `CRC:`, the spaces before `CRC:` left out. The manual
  says the CRC is "for line of text" but not which characters it starts and ends at; this is the
  span under which every CRC in the sample files checks.
- *Its length*: the number after the mnemonic is the piece's length in bytes, from `@` through the
  CRC's last digit, plus 2 for the CR and LF the logger sends. The high-rate line above is 137
  characters long and states `139`.

A piece with no CRC, or whose CRC differs from its text, is dropped, as it can't be told from a
record the capture damaged; a note counts each kind. A line with no record is skipped and counted
too, such as the status line, `@ BLR_STAT`, which the port sends five times a second (p22). A
piece whose CRC checks is the logger's own, so anything else wrong with it refuses the whole file,
naming its line: a length that isn't its own, another mnemonic, a date or time that can't be one, a
value that isn't a whole number (or, in `FER:`, hexadecimal digits), more than 256 values, or a
layout unlike the first record's. So a file holding both kinds of record is refused, and so is one
in which no record's CRC checks.

**The fields.** After the sync code, each value is a whole number, scaled as the manual says:

| Record or group | The manual's fields, in its order | Read as |
|---|---|---|
| high-rate record | gyro X, Y and Z in deg/s × 100; acceleration X, Y and Z in g × 100; the quaternion's X, Y and Z terms and its magnitude term, each × 30,000 | `Gyro X`, `Gyro Y`, `Gyro Z` in degrees per second; `Accel X`, `Accel Y`, `Accel Z` in m/s², 1 g being 9.806 65 m/s²; `Quaternion X`, `Quaternion Y`, `Quaternion Z`, `Quaternion magnitude` as numbers with no unit |
| `Bo:` | the baro sensor's temperature; the pressure in atm × 50,000 | `Bo: temperature` kept as written; `Bo: pressure` in Pa, 1 atm being 101 325 Pa |
| `V:` | the battery's and the Apo, Main, 3rd and 4th outputs' voltages, mV; the outputs' current, mA | `V: battery`, `V: Apo`, `V: Main`, `V: 3rd`, `V: 4th` in V; `V: current` in A |
| `Vel:` | upward, down-range and cross-range velocity, ft/s | `Vel: up`, `Vel: down-range`, `Vel: cross-range` in m/s |
| `Pos:` | inertial altitude, down-range and cross-range position, ft | `Pos: altitude`, an inertial altitude, and `Pos: down-range`, `Pos: cross-range`, distances, in m |
| `ang:` | tilt angle, deg × 10; roll angle, deg; future tilt angle, deg × 10 | `ang: tilt`, `ang: roll`, `ang: future tilt` in degrees |
| `FER:` | the flight event register of the rocket, then of the Apo, Main, 3rd and 4th outputs, in hexadecimal | `FER: rocket`, `FER: Apo`, `FER: Main`, `FER: 3rd`, `FER: 4th`, each register's value as a number |

The gyro rates are "raw data in the sensor axis", and the velocities and positions are in
"ground-relative inertial coordinates (rather than rocket axes)" (p23–24). The accelerations come
from the ±32 g accelerometers below 32 g and from the high-range one above it. The manual says the
download writes the quaternion, four numbers the logger keeps its attitude in (a unit quaternion,
whose four terms' squares sum to 1), "with the rotation axis first and the magnitude term fourth"
(p24). The app's high-rate CSV doesn't: in each such file among the sample logs whose first row
holds numbers, `Quat_1` is the largest term there, about 1 on the pad, so the CSV writes the
magnitude term first (checked by hand on 2026-10-15). Both keep the quaternion's terms as numbers,
so neither order changes a value read. A flight event register is a set of bits, "one bit for each
event" (p24), such as apogee detected; the rocket's records the events that affect every output,
and each output's its own.

**Counts unlike the manual's.** A logger's firmware need not write what the manual of the day
lists. The sample files' firmware writes other counts: a high-rate record holds 12
values where the manual lists 10, `V:` holds 5 where it lists 6, and `FER:` holds 8 where it lists
5. One rule takes all three:

- A group holding at least the manual's count is read by the manual, in its order, and the values
  past it are kept as written: the high-rate record's 11th and 12th values, and `FER:`'s 6th to
  8th, registers of unknown meaning kept as numbers.
- A group holding fewer than the manual's count is kept as written whole, as nothing says which
  value is missing: `V:`'s 5 values aren't read as voltages.
- A group the manual doesn't list is kept as written.

A value kept as written is named by its group and its place there, counted from 1, such as `V: 3`
or `LOG_HIR 11`, and a note says so for each group. The maker's forum describes two 500 Hz fields
the manual doesn't, the "deployment output current" and the "aux analog voltage" (Featherweight,
*Blue Raven development thread*, rocketryforum.com, March 31, 2023), but not their order, so the
high-rate record's 11th and 12th values stay as written.

**The clock.** Each record states the logger's date and time of day to the millisecond, such as
`2026  5 23 10:15:00.120`. The record's clock counts from the first record's, in seconds, and a
time that doesn't come after the one before refuses the file, naming its line. The date need not
be the flight's: the manual says the date stamp "starts with the build time/date and increments
from there", the firmware's build date, unless the phone app has set it (p22). So a note gives the
first record's date and time as written, and the record claims no date for the flight. `Year`,
`Month`, `Day` and `Sync` are kept as numbers and `Time` as a time of day, as the app's CSV keeps
them. The sync code is "a millisecond counter recorded in both the low-rate and high-rate data so
that the two data sets can be aligned", which "rolls over every 250 msec" (p23); the reader keeps
it but doesn't join the two sets.

**What is kept as written, and why.**

- *The baro sensor's temperature*, `Bo: temperature`: the manual states no unit for it in the
  download, and a note says so.
- *The registers' bits*: the manual says the registers are "hex-encoded binary values that use one
  bit for each event" (p24), but not which bit is which event, so no event is taken from them.
- *No [primary track](#the-record)*: the download writes the pressure, not an altitude, so the
  record's time and altitude are empty, as for the [Raven's `.FIPa`](#the-ravens-fipa), and every
  value is a channel.

**A worked example**, the two invented records above. In the high-rate record, `20` is the sync
code and `-15` the first gyro rate, so `Gyro X` is −0.15 degrees per second. The sixth value, `98`,
makes `Accel Z` 0.98 g, 9.611 m/s². The quaternion's four terms, `-3000`, `1500`, `-40` and
`29812`, read as −0.1, 0.05, −0.001 33 and 0.993 73; the four whole numbers' norm is 30,000.12,
within 2 of 30,000, as a unit quaternion's must be after the scaling cuts it to whole numbers. The
last two values, `0` and `0`, are `LOG_HIR 11` and `LOG_HIR 12`, kept as written. In the low-rate
record, `Bo:  7212 49655` gives `Bo: temperature` 7,212, as written, and `Bo: pressure` 49,655 /
50,000 atm, 100,626 Pa. `V:` holds 5 values, so they are `V: 1` to `V: 5`, kept as written.
`ang:   12    5    0` is a tilt of 1.2°, a roll of 5° and a future tilt of 0°. `FER:  400  1c1`
makes `FER: rocket` 1,024 and `FER: Apo` 449, and the last three registers are `FER: 6`, `FER: 7`
and `FER: 8`.

<a id="how-the-usb-download-was-checked"></a>**How it was checked.** The sample logs hold 2
downloads, a team's high-rate and low-rate sets from one rocket's logs. The survey,
[`cargo xtask logs`](#how-it-was-checked), works out each piece's CRC and length itself, apart from
the reader, and decodes each record that checks by its own copy of the manual's table and rule.
The reader must keep exactly the records that check, and each of their values, the clock's
included, must equal the survey's:

| File | Pieces | CRC checks | Records read | Values compared | Differing |
|---|---:|---:|---:|---:|---:|
| high-rate | 17,930 | 17,930 | 17,930 | 304,810 | 0 |
| low-rate | 1,861 | 1,706 | 1,706 | 49,474 | 0 |

Every record whose CRC checks states its own length too. The low-rate file's other 155 pieces
close with no CRC, records the capture ran together, and are dropped. Three physical checks hold
parts of the reading, and of the rule for counts unlike the manual's, to the files; a file that
failed one would be counted as unread:

- Each high-rate quaternion, read from the 7th to the 10th value, has a norm from 29,998.25 to
  30,000.00, within 2 of 30,000 as it must be.
- The low-rate pressure on the pad, the median of the first 20 records, is 0.9977 of the pressure
  a TeleMega (an Altus Metrum board) in the same rocket logged; the check allows 2% either way.
- The first register's lowest bit, the one the app's files call `Liftoff`, first sets 0.024 s
  after the three axes' acceleration in the high-rate file first passes 3 g; the check allows
  0.1 s. This holds the two files' clocks and the register's first value to each other; it
  doesn't show a launch, as the next paragraph says.

The download records no flight. The high-rate file's acceleration stays above 3 g for at most
0.056 s at a time, and its greatest mean over any 1 s is 1.68 g: the bit was set by a short spike,
not a boost. Over the whole low-rate file the Blue Raven's pressure moves by only 0.16% of its
pressure on the pad, while the TeleMega's falls to 0.773 of its own. So none of the download's
channels, the gyroscopes' and accelerometers' included, describes the TeleMega's flight. Nothing
in the files says why, so the reading keeps the values as written, and the
[survey's report](#how-it-was-checked) prints these measures.

**What is left out.** No altitude is made from the pressure, and the two sets aren't joined by
their sync code. `hpr analyze` and the Python package don't read these files yet: the Rust library
does.

### A course's Teensy data logger

Harvey Mudd College's advanced rocketry course flies a data logger the course builds around a
Teensy, a small microcontroller board. The course publishes its flights' files on its
[*Flight Data* page](https://pages.hmc.edu/spjut/AdvRoc/FlightData.md.html), which lists two
kinds, "Teensy Data Logger" and "New Teensy Data Logger". Both write one sensor's reading a line:
the sensor's tag (a short label such as `A` for the accelerometer or `Alt` for the barometer), the
logger's clock in microseconds since it started, and the values. No document describes either layout. So HPR Sim reads what each file states, and where a
file states no unit, it reads a unit only if the flight's own physics confirms it
([ADR-274, the Teensy logger and the rest of the importers](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0274-the-rest-of-the-importers-planned-teensy-logger-first.md),
which decided it). `import::read` reads both with `hpr_flightdata::teensy`.

> **How far to trust it.** The layouts are taken from the files, as no document describes them.
> In the 8 sample files, every value equals the survey's own decoding. Each unit read without
> the file stating it passes a physical check on every file ([below](#how-the-teensy-files-were-checked)).
> The gyroscope, the magnetometer, the high-g accelerometer of the older layout and the analog
> inputs are kept as written, because no check settles their units. Those values aren't
> converted, so they can't be wrong in SI; they simply aren't in SI. No readings, such as the
> apogee, are taken from these files yet: they come with
> [M7.2](../decisions-and-roadmap.md#m7-2).

**The older layout** separates its fields by tabs. Seven tags come round in a cycle, each line
with its own time. These invented lines are from the crate's test file, the tabs shown as
two spaces:

```text
A  2000  0.25  -0.50  9.75
G  2000  1.50  0.50  -2.00
M  2000  20.10  -3.40  17.80
Alt  3100  P  95000  MSL  540.42
HG  3600  0.4788  -0.9577  10.5337
ADC Pin-35  3700  0.0120000000
ADC Pin-36  3800  0.0150000000
```

A value is named by its tag and its place, `A 1` to `A 3`, or by its tag and the name the line
gives it, `Alt P` and `Alt MSL`. No line states a unit, so three of the values are read in SI only
because physical checks confirm them:

| Values | Read as | Why |
|---|---|---|
| `A 1` to `A 3` | acceleration, m/s², as written | their magnitude is within 3% of 1 g, 9.806 65 m/s², on the pad and after landing |
| `Alt P` | pressure, Pa, as written | `Alt MSL` follows from it by the BMP180's formula to within 0.0103 m (0.41 in), below |
| `Alt MSL` | a pressure altitude, m, as written | it is 44,330 × (1 − (P / 101,325)^(1/5.255)), the altitude formula in the data sheet of the Bosch BMP180 barometer, on every line |

`Alt MSL` is a pressure altitude: the height at which the standard atmosphere's pressure equals
the one measured, counted from where it would be 101,325 Pa. It isn't a height above the sea. In
the line above, 95,000 Pa gives 540.4176 m (1,773.02 ft) by the formula, written as 540.42.

The rest is kept as written, as a number with no unit:

- the gyroscope `G`;
- the magnetometer `M`, a quantity HPR Sim doesn't hold;
- the high-g accelerometer `HG`;
- the analog inputs.

`HG` moves in steps of 0.4788 if read as m/s², but it doesn't agree with `A`. During the boost,
while `A` reads from 3 g to 90% of its greatest, `HG` reads 1.37 to 2.39 times `A`'s magnitude,
depending on the file. So its scale isn't settled.

**The newer layout** opens with a line for each sensor that states its units, the clock's first.
Each line after them gives a tag, `t`, the clock, and name–value pairs, separated by commas:

```text
ICM data in us, milli-g, deg/s, milli-T, deg C
MS5 data in us, deg C, Pa
ICM, t, 500000500, xa, 12.20, ya, -20.50, za, 998.40, xg, 0.18, yg, -0.12, zg, 0.06, xm, 30.15, ym, 4.20, zm, -18.75, T, 24.50
MS5, t, 500001000, T, 24.10, P, 95000
```

The clock's unit must be `us`, microseconds. The other units go to the fields by their names. A
field's group is its name without a leading `x`, `y` or `z`, so `xa`, `ya` and `za` form one
group. The units go to the groups in the order the tag's first line names them, or, when only one
unit is stated, to every field. In the example:

- `xa`, `ya` and `za` are in thousandths of a g, read as m/s².
- `xg` to `zg` are in degrees per second.
- `xm` to `zm` are stated in milli-T and kept as written: HPR Sim holds no magnetic field, and
  the stated unit looks wrong besides, as Earth's field, 0.025 to 0.065 milli-T, is about a
  thousandth of the values the sample file holds, which fit microtesla.
- `T` is in degrees Celsius, read as kelvins.

A value is named by its tag and its name, such as `ICM za`. A tag with no units line, such as the
sample file's `VOL`, is kept as written, and so is one whose units don't fit its groups; a note
says which.

**The clock.** Each tag is its own set of channels, on its own clock, as each sensor is read at its
own rate: in the sample files, about 43 lines a second for each of the older layout's tags, and
about 1,000 for the newer layout's main accelerometer. Times are seconds from the logger's start.
The file is refused at a line that isn't a tag, a clock and values; at a tag's line laid out
unlike its first; or at a clock that doesn't increase from the tag's line before. The count of
microseconds runs over after 2³² µs, 71.6 minutes, so a longer recording is refused there. A
file's last line, if it ends with no line end and can't be read, is a line the logger was writing
when it stopped: it is dropped, with a note.

<a id="how-the-teensy-files-were-checked"></a>**How it was checked.** The sample logs hold 7 files
in the older layout and 1 in the newer, a 100 s cut from a longer recording. The survey,
[`cargo xtask logs`](#how-it-was-checked), decodes each file itself and compares each of the
6,188,639 values with the reader's: none differs. Four physical checks hold the units read; a file
that failed one would be counted as unread:

- **`Alt MSL` against the BMP180's formula,** worked out from `Alt P` in 64-bit floats. On all
  305,340 `Alt` lines it differs by at most 0.0054 m (0.21 in). The check allows 0.0103 m (0.41 in): half the
  hundredth of a meter `MSL` is written to, plus an allowance of 44,330 × 2⁻²³ m, 5.3 mm (0.21
  in), for the logger's own rounding of the formula. On 21,540 lines the gap is more than the half
  step alone, and it is greatest at the same pressure in every file, so the logger rounds the
  formula its own way, which no document gives.
- **The accelerometer at rest.** Its magnitude on the pad, the median over the 5 s ending 1 s
  before liftoff, is 0.983 to 0.989 g. Over the file's last 2 s, after landing, it is 0.997 to
  1.008 g.
  The check allows 3% either way; read in another unit, such as g or ft/s², it would be 3 or more
  times off. Over the first 2 s of the older layout's files, minutes before launch while the
  logger was handled, three files read 1.034 to 1.048 g, so the pad is taken just before liftoff.
- **The time to apogee.** From liftoff (the accelerometer's first reading above 3 g in the 30 s
  before apogee) to apogee (the highest pressure altitude), the time is 0.90 to 1.14 times
  √(2h/g), where h is the apogee's height above the pad: the time a coast from the pad would take
  with no drag. It is a coarse check, as the burn adds time and drag takes some away, so it
  allows a factor of 2, where a clock misread by a power of ten would be 10 or more times off.
- **The pad pressure** of the newer layout, the median on the pad, is between 50 and 110 kPa, as
  any launch site's is. The report says so without the value, which would give the site's
  elevation.

Each check also needs the reader to have read the values it holds as the quantities it takes them
in, so a unit the reader got wrong fails it.

The newer layout's other stated units rest on the file's statement. Its high-g accelerometer, read
in its stated g, is 0.996 times its main one during the boost, which agrees with the unit stated;
the report prints this as a measure, not a check.

**What is left out.** There is no [primary track](#the-record), as neither layout writes an
altitude above the pad. `hpr analyze` and the Python package don't read these files yet: the Rust
library does.

### One team's radio telemetry packets

A university team's flight computer writes each reading as a *packet*, a line framed by `255` and
`254`, and sends the packets down by radio. For one flight the team published four files of them:
the flight computer's own log, the text a ground receiver wrote of what it heard, and two exports
of the log, cut to the flight, each saved as an RTF document (a word processor's format) and then
as CSV by a spreadsheet. No document describes the packets, so HPR Sim reads what the files show,
and holds the reading to the flight computer's own log
([ADR-274, the Teensy logger and the rest of the importers](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0274-the-rest-of-the-importers-planned-teensy-logger-first.md),
which decided it). `import::read` reads all three forms with `hpr_flightdata::packets`.

> **How far to trust it.** The layout is taken from the files. Every packet the receiver's text
> or an export holds that the onboard log also holds, by its stage, kind and clock, matches the
> log's field for field ([below](#how-the-radio-packets-were-checked)); the receiver's GPS packets
> are the only ones the log doesn't hold. That match shows the copies are decoded alike, not that
> a value is right: the two physical checks below, a pressure band and a time to apogee within a
> factor of 2, are what hold the units. The clock is read in milliseconds and one value
> in hectopascals, each held to the flight's physics; the other values are kept as written,
> because nothing settles their units. No readings, such as the apogee, are taken from these
> files yet: they come with [M7.2](../decisions-and-roadmap.md#m7-2).

**A packet** separates its fields by colons, or by commas in the exports. These invented lines
are from the crate's test files:

```text
255:T:11:PP:100150:950.12:21.50:3.25:254
255:T:11:AA:100152:9.70:0.40:-0.80:254
255:N:88:GG:100200:1.00:2.00:1234.56:9876.54:8.00:250.00:90.00:0.50:254
```

| Field | What it is |
|---|---|
| `255` | the packet's start |
| `T` | the path it came by: `T` from the flight computer, `N` relayed by a second transmitter, which also sends the GPS packets |
| `11` | the flight computer's stage of the flight, `00` to `55` in the samples, or `88` on a GPS packet |
| `PP` | the packet's kind, two capital letters: the samples have `PP`, `AA`, `CC` and `GG` (GPS) |
| `100150` | the clock, in milliseconds: the flight computer's, or on stage `88` the relaying transmitter's own |
| the values | numbers, as many as the kind's first packet holds |
| `254` | the packet's end |

The path, the stages and the GPS packets are read from the files, not from a document: no
document confirms what they mean, and what `AA` and `CC` measure isn't known.

Each kind becomes a set of channels on its own clock, named by the kind and the value's place,
such as `PP 1`, with the stage as a channel of its own, `PP stage`. The GPS packets (stage `88`)
carry the relaying transmitter's own clock, not the flight computer's, ahead of it by an offset
nothing states. So their times are left empty (gaps, with no place on the record's time axis), and
their clock is kept as written in a channel of its own, `GG clock`. A `Stage Two` line, as the
onboard log writes when the stage changes, becomes an event at the next packet's time. The word
must be a number's, `One` to `Nine`: a mark the radio garbled is a broken line, and one heard
again is dropped.

| Values | Read as | Held to |
|---|---|---|
| the clock | seconds, from milliseconds | the time from liftoff to apogee, against √(2h/g) ([below](#how-the-radio-packets-were-checked)) |
| `PP 1` | a pressure, Pa, from hectopascals | the pad's pressure, between 500 and 1,100 hPa, and the time to apogee, which takes the height from it |
| the rest | kept as written | nothing settles a unit |

`PP 3` climbs with the flight, but where it reads above 200 it is 1.11 to 1.24 times the
height the standard atmosphere gives the pressure, in meters, so neither meters nor feet fits it,
and it is kept as written.

**Lines that aren't packets.** The radio loses bytes, so the receiver's text holds many *broken*
lines: a packet cut short, or run into the next. A broken line is dropped, and a note counts
them. So is a packet heard again: the same stage, kind, clock and values as one already read, by
either path, as the receiver heard many packets twice. The onboard log
opens with a whole number and its filter's gains, `Gain[0]: …`, which a note counts.

**The exports** are RTF documents whose lines a spreadsheet split at the colons and saved as CSV.
So each line has commas after it, the document's markup sits in the first cells, and a `\` ends a
line where the document broke it. The reader takes each line on its own: it drops the commas, the
markup before a packet, a `\` at the end and the `}` that closes the document. A line of markup
alone is dropped.

A file with no whole packet is refused, and so is one with more than 32 kinds, a packet with more
than 32 values, or a clock of 2³² ms, 49.7 days, or more.

<a id="how-the-radio-packets-were-checked"></a>**How it was checked.** The sample logs hold the
four files of one flight. The survey, [`cargo xtask logs`](#how-it-was-checked), decodes each file
itself and compares every value with the reader's: none differs. It then looks up each packet of
the receiver's text and of the exports in the onboard log, by its stage, kind and clock:

| File | Form | Whole packets | Held by the onboard log | Differing | Not held: GPS | Not held: other | Repeated | Broken lines |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| the receiver's text | received | 42,796 | 38,475 (29,914 relayed, 8,561 direct) | 0 | 4,321 | 0 | 8,368 | 10,495 |
| the export of all three kinds | RTF | 4,939 | 4,939 | 0 | 0 | 0 | 2 | 0 |
| the export of mostly pressures | RTF | 2,469 | 2,469 | 0 | 0 | 0 | 1 | 1 |

The onboard log itself holds 264,462 whole packets and no broken line; 2 are repeated. Counts
include repeats; *repeated* counts the packets a reading drops as heard before, by either path. The survey compares 1,315,788 values
with the reader's in all four files, and none differs. The GPS packets are the second
transmitter's fixes, which the onboard log doesn't record.

Two physical checks hold the units read, on the onboard log:

- **The pad pressure,** the median of `PP 1` over the 5 s ending 1 s before liftoff, is between 500 and
  1,100 hPa, as any launch site's is. The report says so without the value, which would give the
  site's elevation.
- **The time to apogee.** From liftoff (the first packet, after the pad's stage `00`, whose
  pressure is more than 1 hPa below the median of the 5 s ending 1 s before it) to apogee (the
  lowest pressure), the time is 1.055 times √(2h/g), where h is the height the standard
  atmosphere gives the two pressures. The check allows a factor of 2; a clock misread by a
  power of ten would be 10 or more times off.

**What is left out.** There is no [primary track](#the-record), as no packet writes an altitude
above the pad in a unit that is settled. The GPS packets' values are kept as written. `hpr
analyze` and the Python package don't read these files yet: the Rust library does.

## The record

Every reader fills the same record, `FlightLog`, in SI units:

- **What the file says about itself:** the logger's name, serial number, firmware and flight
  number where the file states them, and figures the logger worked out itself, such as its apogee,
  kept beside HPR Sim's readings and never in place of them. A file that states many such
  figures, as a [Featherweight summary](#featherweights-summaries) does, keeps each one in the
  record's `stated.figures`, its name and value as written.
- **The primary track:** the barometric altitude above the pad (AGL, above ground level) on the
  logger's clock, with the barometer's vertical speed, the temperature and the battery voltage
  where they share that clock. The [readings](../physics/log-readings.md) are taken from it. A
  log with no barometer, such as a [GPS log](#gps-logs), has an empty primary track, and so does
  a [raw download](#raw-downloads), whose barometer's readings are pressures, not altitudes. A
  [summary](#featherweights-summaries) has no samples at all, so its clock and tracks are empty.
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

A cell can be a valid number and still overflow when it is converted to SI: `1.7e308` read as g
becomes 1.7e308 × 9.806 65, more than the largest value a 64-bit floating-point number holds,
about 1.8e308. The reader then refuses the whole file and names the line, so a bad cell never
reaches a channel as an infinite value. Every reader applies this rule, so a channel holds SI
values or gaps and nothing else ([ADR-258, the rule for infinite values](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0258-aerospacenus-board-and-a-phone-apps-csv-before-the-rest.md),
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
- A Featherweight summary's four pad accelerations and offsets (`Pad Low-G Accel`, `Pad Hi-G
  accel`, `Low-G accel pad offset`, `Hi-G accel pad offset`) are kept as written: three numbers
  with no unit, which no source states ([below](#featherweights-summaries)).

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
- **An RRC3 export saved by a European locale.** mDACS's export in a European locale's form is
  semicolon-separated with decimal commas, and may end every line, the header's too, with a
  semicolon; the reader drops the empty cell after it, on full rows and short ones. The one such
  sample had its rows sorted by a spreadsheet by their time's text, `10,05` before `2`. Put in
  order by value, they step 0.05 s with none missing, but the reader keeps a file's order and
  refuses a time that goes back, naming the line, so it is refused and counted as damaged: to
  read such a file, sort its rows by time first.
- **An Eggtimer's clock, worked out from the file.** The Classic and TRS write their time in
  milliseconds, the Quantum and Proton in seconds, under the same `T`. The reader takes
  milliseconds when the median step is above 5: the manuals' rates, 1 to 33 samples a second, put
  a step between 0.03 and 1 in seconds and between 30 and 1,000 in milliseconds. In the CSV
  download, a step above 1 and below 30 is refused (below). Of the 44
  Eggtimer files read, 4 are read in milliseconds and 40 in seconds.
- **An Eggtimer's heights and speeds, from the manuals.** The Classic and TRS manuals state feet
  and feet per second. The Quantum and Proton manuals state no unit for the download, but give
  every height in feet, and no Eggtimer manual offers meters; the heights are read in feet for
  every model, and in a terminal download, plain or saved as RTF, only where the coast before
  apogee also slows at about g in feet (below). A speed is read in feet per second only where it matches the altitude's change
  over the step on at least 90% of rows (on the sample logs, 95.9% or more); otherwise it is kept
  as written.
- **An Eggtimer's filtered altitude, held to `Alt`.** The filtered altitude must scale to `Alt`
  as a filter of it can: the one multiplier that best takes `Alt` to it, with no offset (a
  least-squares fit through zero), must lie from 0.1 to 2, or the file is refused. A filter lags
  the altitude, under it going up and over it coming down. On a steady climb at $v$ ft/s to $H$
  ft, a lag of $\tau$ s lowers the multiplier by about $1.5\,v\tau/H$: 100 ft/s, a lag of 1 s
  and a climb to 600 ft give 0.75. A decimal place lost writes a value 10 times over, two places
  100 times. A refused file's numbers can't be trusted, its clock's among them: a seconds clock
  written without its decimal point reads as milliseconds. The check doesn't catch a column off
  by less than a factor of 2, such as a filtered altitude in meters (0.3048 times). On the 44
  sample files read, the multiplier is 0.900 to 1.000 (the
  [survey's report](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/log-import.md)
  prints the range). A 45th, a download a spreadsheet saved with its decimal points taken out
  (`12.34` written `1234`), gives 99.9 and is refused, and counted as damaged. In the CSV
  download, a median step between rows above 1 and below 30 is neither seconds nor milliseconds at
  the manuals' rates, and the file is refused too: `0.05` s written without its point is `5`.
- **An Eggtimer download saved by a European locale.** A spreadsheet set to a European locale
  saves the CSV download with semicolons between the cells, and may write its numbers with a
  decimal comma, the mark between a number's whole part and its fraction (`0,05` for 0.05). A
  header that reads as an Eggtimer's only when split on semicolons is split on them. The reader
  then uses a decimal comma only if no number has a point and some number has a comma before
  anything but three digits (`0,05`). A cell in the other mark, such as `1,250` in a file of
  points, is refused on its line, where the table reader takes it as a gap: here a cell the
  manuals write as a number can't be left out, so a thousands separator is never read as a
  decimal mark. Two of the sample downloads, from the European Rocketry Challenge (EuRoC), are
  semicolon-separated: one with decimal points, read; the other with none at all, as a
  spreadsheet took them out, refused (above). Decimal commas are tested on invented files only.
- **A Blue Raven's speeds, inertial positions, rates of turn and accelerations, from the
  manual.** The manual gives their units for the logger's USB download, which HPR Sim reads too
  ([below](#the-blue-ravens-usb-download)): feet per second, feet, degrees per second and g
  (*Blue Raven User's Guide*, 2026-05-12, p23). The app's own files
  agree: its summary of a flight states the top speed in feet per second and the inertial
  altitude in feet, and its accelerometers read 1 g at rest on the pad. A file that disagreed
  would still be read in these units; its note would show the disagreement.
- **An Eggtimer TRS's terminal download, its preamble by its own unit marks.** Captured in a
  terminal program, a TRS's download prints its flight settings and a flight summary before the
  table its CSV download writes (*Eggtimer TRS Flight Computer User's Manual*, firmware 2.07,
  p35–37). The table is read as the CSV download is, but its clock and its heights each rest on
  two checks that agree: the clock on its step and the coast, the heights on the manuals' feet
  and the coast. Just before the top nothing pushes the rocket and its drag fades as it slows,
  so its altitude curves down at gravity: 32.17 ft/s² in feet and seconds, 9.81 m/s² in meters.
  The reader fits a parabola to each height over the 3 s up to the top, or up to the row before a
  charge's marker (an Eggtimer can fire one on the way up) or a jump in the altitude, cut short
  16 times at most before the file is refused as having no clean coast, and takes a slowing of
  0.7 to 1.3 g; a clock 1,000 times off would put it a million times off. A file whose
  clock this doesn't bear out, or with no such coast, is refused; a height that slows at g in
  meters rather than feet is kept as written, with the speeds that are its change. In the 3 sample
  files, two text and one RTF, the raw heights slow at 0.96 to 1.02 g in feet. The summary's
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
  speed. The same flight's `.odf` file holds the sensors' counts with the calibration
  ([the ARTS2's raw data](#the-arts2s-raw-data)).
- **AerospaceNU's flight computer board, each unit by two checks.** The flight computer board
  (FCB) of AerospaceNU, Northeastern University's rocketry club, logs a binary file, which the
  club's tools turn into a CSV; the club publishes both (AerospaceNU, *rocket-flight-data*,
  `README.md`, read October 12, 2026). The 20 sample files come under eight headers of 73 to 87
  columns. No document describes the columns, so a unit a column's name doesn't state is read
  only where two checks on the file's values agree, neither of which the weather or the flight
  can meet in the wrong unit. A check sets the column against a physical mark (1 g at rest, a
  barometer's whole pascals, a sensor's published sensitivity) or against another column; where
  that is a sibling, such as the inertial units' average against the units, it counts only once
  the sibling's own unit is decided, so no counted check relates two columns that share one
  unknown unit. Where the checks don't agree, the column is kept as written, with a note:
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
    climb and within a factor of 1.8 of the accelerometer's ($1.8^2$ is 3.24, under the 3.28 between
    meters and feet). `vel_z` must match `pos_z`'s rate and the accelerometer's speed. A GPS
    may write 0, or hold an old value, until its first fix, so `gps_alt` is checked only at its
    fresh fixes (a new value other than 0), each check a line fitted with its own offset, so that
    no pad reading enters either; a pad reading far from where its fixes put the pad is a held
    value, and the column is kept as written. Where `gps_alt` is read, a 0 is a gap. `pos_z`
    reads in meters in all 20 files, `gps_alt` in 12 (kept as written in 8) and `vel_z` in 18.
  - *GPS position:* NMEA 0183's degrees and minutes run together (42.5° written `4230.0`) put
    every latitude more than 1° from the equator past 90 and every longitude more than 2° from
    the prime meridian past 180, which no degrees reach. `gps_lat` and `gps_long` are read in
    degrees and minutes where most latitudes pass 90 and most longitudes 180, every value's
    minutes are under 60 and every converted value is a latitude or longitude; in degrees where
    every latitude lies within ±90 and every longitude within ±180. A position (0, 0) is no fix:
    it takes no part, and where the position is read it is a gap. The companions `gps_lat_mod`
    and `gps_long_mod` count in neither decision; they are read in degrees only where they hold
    the decided position in degrees. Of the 20 files, 15 read in degrees and minutes, 4 in
    degrees (their companions kept as written in 3, which hold the values converted as if they
    were degrees and minutes), and 1, with no fix, is kept as written. NMEA 0183 writes the
    hemisphere (N or S, E or W) in a field of its own, which the board's columns drop, so only
    a sign can carry it: where no value in `gps_lat` or `gps_long` is negative, a place west of
    the prime meridian can't be told from its mirror east, and the position and its companions
    are kept as written, with a note saying how the checks would read them so that someone who
    knows the site can sign them. Each of the 19 files with a fix writes its longitudes
    negative, west, and is read.
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
  - *Positions, in degrees:* most fixes pass $\pm\pi/2$ or $\pm\pi$, which radians can't, and most have a
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
  rows a second (from the 27 sample files' timestamps). No description of the format was found.
  Of the 27 sample files, 19 are an older layout of 35 columns with no barometer, and 8 a newer
  one of 67 or 69 columns that adds one. `Seconds` is the clock, by its name. Every other column
  is read in a unit only where two checks agree that read different numbers (two columns, a
  column against another of settled unit or direction, or a column's extent against its steps);
  otherwise it is kept as written, with a note. An invented start of a file, shortened:

  ```text
  Seconds,Timestamp,AltitudeMSLGPS,Record,Static,Latitude,Longitude,GTotal,Gx,Gy,Gz,…
  0.000,1941868800000,100,1,12,40.123456,-100.234567,1.000000,0.000000,0.932039,0.362358,…
  ```

  - *Pressure, `hPa`:* it names its unit, and its first reading must be a launch site's, 500 to
    1,100 hPa.
  - *Barometric altitude, the record's altitude, in meters:* it must be the pressure altitude of
    `hPa` in the standard atmosphere, which in feet would be 3.28 times as large. And in the
    coast before apogee it must slow at 0.7 g to $0.7 \times 3.28$ g, as gravity and drag slow a coasting
    rocket by at least 1 g, which an altitude written in feet can't. Read so in 7 of the 8 newer
    files. The barometric speeds are read in m/s by the altitude's rate and the same coast.
  - *Accelerations, in g:* the pad reads 1, and the boost's integral matches the barometric climb
    rate. The older files have no barometer for the second check, so their accelerations are
    kept as written: read in g in 6 files, kept in 21.
  - *Attitude, in radians:* each angle by its range (within $\pm\pi$, the pitch $\pm\pi/2$), and by its
    direction against a sensor that needs no unit, on rows where the phone is still: the pitch
    and roll against gravity's direction in the accelerometer, the yaw against magnetic north in
    the magnetometer, as Android's `SensorManager.getOrientation` defines the three. Read so: the
    pitch in 25 files, the roll in 24, the yaw in 10 (the 8 newer; in most older files the yaw
    doesn't follow the magnetometer, so it is kept as written).
  - *Rates of turn, in rad/s:* the gyroscope by the attitude's turn, where all three angles are
    read, and by a full scale a phone's gyroscope offers. Read so in 7 files, kept as written in 20.
  - *Heading, in degrees:* by its range (0 to 360, some reading past $2\pi$) and by its period:
    crossing north it jumps by 360 less its turn, where gradians would jump by 400 less it.
  - *Position, in degrees:* the latitude and longitude together, both or neither, by the
    latitude's largest reading (past $\pi/2$, within 90) and the longitude's (past $\pi$, within 180).
    That rules out radians, scaled integers and NMEA's degrees and minutes run together, the last
    for any site on land. No range can tell degrees from gradians (0.9°); degrees are taken as
    phones' location services give them.
  - *Time stamp:* a Unix time in milliseconds, its steps the clock's and its date in this century.
  - *GPS altitude, kept as written in all 27:* only one check reaches it, its gain against the
    pressure altitude. Fixes once a second can't show the coast. On the pad, the weather on record
    moves a pressure altitude across a band 1,840 m (6,037 ft) wide, and a GPS reading $N$ read in meters and
    in feet differs by $0.695\,N$ m, so the pad separates the two only where $N$ tops about 2,650:
    a pad above about 2,650 m (8,694 ft) if the column is in meters, 810 m (2,657 ft) if in feet. No sample
    file's pad is that high.
- **An Eggtimer's download saved as RTF, unwrapped.** One sample file is an Eggtimer TRS's
  terminal download that a text editor saved as an RTF document (Rich Text Format, a word
  processor's markup). The reader takes the document's plain text, following the parts of the RTF
  specification (Microsoft, version 1.9.1, 2008) a saved capture needs: groups, control words and
  escaped characters, with fonts, colors and pictures skipped. It reads that text as it reads a
  terminal download (above). A document holding several downloads gives the first; the rest are
  counted in a note. In the sample file, the table's clock is in milliseconds and its heights in
  feet, each by two checks, the coast among them (above); its speeds, which don't follow the
  altitude's change, are kept as written, as are the summary's apogee and site elevation, which carry no unit mark the reader
  knows. An RTF document holding anything else is refused as not an Eggtimer's. The radio packets
  in two other sample files are read by
  [the packets' reader](#one-teams-radio-telemetry-packets), not this one.

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

### Featherweight's summaries

A Featherweight summary is the logger's own account of a flight, a figure a row, with no samples.
The Featherweight app exports one beside a Blue Raven's or a GPS tracker's samples. The Blue Raven
"records three sets of data for each flight", the first "flight summary data such as maximum
altitude and speed" (*Blue Raven User's Guide*, updated May 12, 2026, p18). Each row is a
figure's name, then its value, with the unit written after the number:

```text
Rocket Name,Club demo
Max Altitude,2500.25 feet
Max velocity,500.50 feet/sec
Thrust tilt efficiency,98.50%
Direction of 1st motion,-1.0000,0.0000,0.0000
```

> **How far to trust it.** The logger's own figures, copied as the file states them, not HPR
> Sim's readings. Two are checked: the maximum altitude and the pad's altitude, against the file
> of samples beside each summary, and 32 of the 33 Blue Raven summaries with such a file agree in
> both ([checked against the files beside](#checked-against-the-files-beside)). The speeds,
> accelerations, times and the rest are converted from the unit the file writes and checked
> against nothing. Where a number matters, set it beside the samples the app exported with it.

**How it is recognized.** A file is a summary when its first line that isn't blank names
`Rocket Name`, and a line among its first 200 names `Max Altitude` or `Launch time UTC`, figures
only a summary states. A tracker's summary is told from a Blue Raven's by its `Launch time UTC`.
A name is matched without regard to case or repeated spaces, and kept as written.

**What the record holds.** A summary fills the part of the [record](#the-record) that says what
the file states about the flight, and nothing else:

- **Every row is a figure** in the record's `stated.figures`, in the file's order: its name and
  value as written, its quantity, and its value in SI where the reader knows the figure.
- **The stated apogee** is `Max Altitude` in SI: the barometer's altitude above the pad for a Blue
  Raven, and for a tracker its receiver's altitude above the pad altitude it states.
- **The pad's elevation** is a Blue Raven's `Pad altitude ASL`, its barometric altitude of the pad,
  or a tracker's `Pad altitude`, its receiver's.
- **The serial number and firmware** are the rows' text as written, and the flight number is
  `Flight number` where it is a whole number.
- **The clock, the tracks and the channels are empty**, with a note saying the samples are in the
  file exported beside it: the Blue Raven's low-rate CSV or the tracker's CSV. So every reading
  withholds on a summary.

**A unit is read only where the figure's name allows it**, as a header's unit is read for a
column. Each figure the reader knows names its quantity and the words the files write for its
unit. The main ones are these; the [reader's reference](../api/hpr_flightdata/fwsummary/index.html)
lists every figure:

| Figures | Read as | Unit the file writes |
|---|---|---|
| `Max Altitude`, `Main chute deploy altitude` | altitude above the pad, m | `feet` or `ft`, $\times 0.3048$ |
| `Pad altitude ASL` (a Blue Raven's), `Pad altitude` (a tracker's) | altitude above sea level, m | `feet` or `ft`, $\times 0.3048$ |
| `Max velocity`, `Drogue descent rate`, a tracker's velocities | speed, m/s | `feet/sec` or `ft/sec`, $\times 0.3048$ |
| `Max motor burn acceleration`, `Peak drag deceleration`, each deployment output's maximum (`Apo channel max accel` and the like), `Max landing accel` | acceleration, m/s² | `Gs`, $\times 9.806\,65$ |
| `Temperature on pad`, `Max Temperature`, `Min Temperature` | temperature, K | `F`, degrees Fahrenheit |
| `Time to first burnout`, each deployment output's firing time | time, s | `sec` |
| `Thrust tilt efficiency` | a fraction | `%`, ÷ 100 |
| `Direction of 1st motion`, `Gravity direction on pad` | a direction of unit length, a number an axis | none |

The Blue Raven's manual describes the same summary as the logger's USB port prints it, with each
field's unit (p31–33). The manual's unit words match the app's except for the two figures
below. The USB form also scales some figures (accelerations in g × 10); that scaling isn't
compared. These two are kept as written in every file, never guessed:

- **`Main chute descent rate`**: the app writes its value in `feet`, a length, where the manual
  gives the USB summary's average descent under the main in "feet/second x 10". A figure whose
  unit isn't one its name allows is kept as written, with a note, and isn't made into a speed.
- **The four pad accelerations and offsets**: each three numbers with no unit. The manual gives the
  USB summary's in "units of Gs x 10,000"; the app's are not scaled so, and no source states
  what they are in.

A name the reader doesn't know is kept as written too, and so is a known figure whose cells
aren't the numbers it needs, such as a direction with two components; each has a note. A zero is
kept as the file states it: what a zero means is a reading, which comes with
[M7.2](../decisions-and-roadmap.md#m7-2). The [survey's report](#how-it-was-checked) counts
each figure kept as written by the catalog's family. The main's descent rate, for one, is kept
in 23 files filed as `blueraven-summary-csv` and 13 filed as `featherweight-summary-csv`.

**Cells and rows.** Cells are separated by commas and trimmed of spaces. Empty cells at a row's
end are dropped, as some files pad every row to four cells. Blank lines are skipped, lines may
end in CR, LF or both, and a byte-order mark is dropped. A row whose name is empty or a number is
refused, as is a name stated twice, and a value too large for its unit's conversion.

**A worked example**, from the invented Blue Raven summary that the crate's test
`tests/featherweight_summary.rs` reads and checks:

- `Max Altitude,2500.25 feet` gives the stated apogee, $2500.25 \times 0.3048 = 762.08$ m.
- `Max velocity,500.50 feet/sec` is a speed of 152.55 m/s (500.50 ft/s).
- `Max motor burn acceleration,12.0 Gs` is $12.0 \times 9.806\,65 = 117.68$ m/s².
- `Temperature on pad,77.0 F` is 298.15 K (77.0 °F).
- `Thrust tilt efficiency,98.50%` is the fraction 0.985.
- `Main chute descent rate,-20.0 feet` stays −20.0 as written, with the note
  `` `Main chute descent rate` kept as written, in a unit that isn't a speed: the file writes `-20.0 feet` ``.

### Checked against the files beside

A summary and the samples exported beside it describe the same flight, so the survey checks the
two lengths a summary states against them. `cargo xtask logs` takes each file of samples in the
summary's folder that its reader reads and that holds an altitude: a Blue Raven's low-rate CSV,
not its high-rate one, or a tracker's CSV. It
compares the summary's maximum altitude with the file's highest altitude above the pad (for a
tracker, its highest GNSS altitude less the pad altitude the summary states). It compares the pad
altitude with the median of the file's first 20 altitudes above sea level. A figure agrees within
10%. When several files sit beside a summary, the ratio, summary over file, nearest 1 is the one
counted.

Feet and meters are 3.28 apart, so a band of 10% tells them apart. A summary that would agree
only were its feet meters contradicts its unit, and `cargo xtask logs --check` fails on it. A
folder may hold other flights' files, so agreeing is counted, not required. The
[survey's report](#how-it-was-checked) prints the counts, naming a file by the survey's id, the
first 16 hex digits of its SHA-256:

| Logger | Summaries | With a file beside | Stating an apogee | Apogee agrees | Its ratios | Pad agrees | Its ratios | Agreeing in neither | Contradicted |
|---|---:|---:|---:|---:|---:|---:|---:|---|---|
| Featherweight Blue Raven | 36 | 33 | 33 | 32 | 0.9698 to 1.0091 | 32 | 0.9770 to 1.0032 | `cd1bfe19ff70e17d` | none |
| Featherweight GPS tracker | 12 | 11 | 5 | 5 | 0.9995 to 1.0000 | 10 | 0.9766 to 0.9999 | `413de085a44f4872` | none |

No summary agrees only as meters. Of the tracker summaries, 5 of the 5 that state an apogee
agree in it. The one of each logger that agrees in neither figure agrees with no file beside it,
not even as meters; the survey doesn't show why. Nor is it measured why a Blue Raven's stated
apogee runs up to 3% below its low-rate file's highest altitude
([ADR-260, the summaries' check](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0260-featherweight-summaries-before-the-rest.md)
§4). The summaries with no file of samples beside them are read by the same rules, unchecked. No
other figure is checked. Summaries inside zip archives are read and checked as the others are,
against the files of samples in the same archive ([zip archives](#zip-archives)); the two inside
Excel workbooks are read by the same rules, unchecked ([Excel workbooks](#excel-workbooks)).

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
> ([units no source states](#units-no-source-states)). The RTx's altitudes can't go below 0, so
> near sea level many read exactly 0 ([Missile Works' RTx](#missile-works-rtx)).

Four layouts are read. Each point's time is the one the file gives it; where it gives none, the
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
- **Missile Works' RTx**, a GPS tracker: the `.rtx` file its maker's program saves, and the CSV
  and text files it exports, read since [M7.1c12](../decisions-and-roadmap.md#m7-1c12)
  ([below](#missile-works-rtx)).

For example, take a `GGA` sentence's position and an `RMC` sentence's speed, invented after the
textbook example public descriptions of NMEA 0183 give; a test in the crate reads these two lines
and checks every number below:

```text
$GPGGA,123519.000,4807.0380,N,01131.0000,E,1,08,0.9,545.4,M,46.9,M,,0000*59
$GPRMC,123519.000,A,4807.0380,N,01131.0000,E,022.4,084.4,230394,,,A*62
```

The latitude `4807.0380,N` is 48 degrees and 7.038 minutes north, $48 + 7.038/60 = 48.1173$°; the
longitude `01131.0000,E` is 11 degrees and 31 minutes east, 11.5167°. The altitude is 545.4 m
(1,789 ft) above mean sea level, in the unit the sentence names (`M`). The speed of 22.4 knots
reads as $22.4 \times 1852/3600 = 11.52$ m/s (37.8 ft/s), and the course is 84.4° from true north. The
record's zero is the first sentence's time, 12:35:19 UTC on 23 March 1994.

### Missile Works' RTx

Missile Works' RTx/GPS is a GPS tracker built around a u-blox 7 receiver. It keeps its fixes once
a second, up to "~68 minutes per flight", and radios them to a base station (*RTx/GPS User
Manual*, firmware 1.31, Missile Works, 2016-11-26, pp2, 14). Missile Works' mDACS program saves a
download as an `.rtx` file and exports it to CSV, text and KML (*mDACS User Manual*, revision
1.71, May 2024, p16). The KML is read as [above](#gps-logs); the `.rtx` and the CSV and text
exports by `hpr_flightdata::rtx`. Neither manual describes the files: what follows was worked out
from the 5 `.rtx` files among the sample logs and the 13 exports beside them (8 CSV and text, 5
KML), with no other program's source read
([ADR-267, reading the RTx before the other layouts](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0267-missile-works-rtx-before-the-rest.md)).
Like every GPS log here, an RTx file feeds no reading yet: the readings wait for a barometer's
track (see [GPS logs](#gps-logs)).

> **How far to trust it.** In 4 of the 5 sample `.rtx` files every field the reader converts
> agrees with mDACS's own exports on every fix they hold (all but the last), positions written
> in degrees to their sixth decimal; the fifth has only a KML beside it, which holds no times,
> so there only the positions and altitudes are compared. In 4 flights the tracker's climb is
> 0.98 to 1.15 times the highest altitude of the barometric altimeter flown beside it, a figure
> [the survey](#how-it-was-checked) reports but doesn't check. The velocity and the speed over
> ground stay in the file's own numbers, since no document gives their units. The fixes are the
> receiver's; HPR Sim claims no accuracy for them.

- *The `.rtx`.* A stream written by .NET's built-in object serializer, `BinaryFormatter`,
  holding one list of 65,536 32-bit integers, in the layout Microsoft publishes as MS-NRBF: 4,096 records of 16 *slots*, 68.3 minutes at one a
  second, the manual's flight. The list's size counts the slots in use; the rest are 0.
- *The slots.* Each holds a 16-bit number, a *word*, but not as written: the word times
  387,420,489 ($3^{18}$), keeping only the remainder after dividing by $10^9$. Multiplying a slot
  by 513,180,409 and keeping the same remainder gives the word back, because
  $387\,420\,489 \times 513\,180\,409$ leaves a remainder of 1. The files show it three ways: the
  slot holding the time steps by exactly 387,420,489, less any whole $10^9$, from one fix to the
  next a second later; every one of the 264,336 slots in use in the sample files reads back as a
  word below 65,536, which a slot drawn at random from 0 to $10^9 - 1$ would do about once in
  15,000; and the words equal the exports' figures, field by field.
- *The words.* Each record is one fix:

  | Words | Field | Read as |
  |---:|---|---|
  | 0, 1 | the time, `HHMMSS` | word 1 × 65,536 + word 0 |
  | 2 | the latitude's degrees and whole minutes, `DDMM` | |
  | 3, 4 | the minutes' five decimals, after a leading 1 | word 4 × 65,536 + word 3 − 100,000 |
  | 5, 6, 7 | the longitude, as the latitude | |
  | 8 | the altitude, m | |
  | 9 | `Velocity` | as written |
  | 10 | `VDOP`, the vertical dilution of precision | as written |
  | 11 | the hemispheres: `N` or `S` in its low byte, `E` or `W` in its high | |
  | 12 | `NavStatus`, the receiver's fix status, in its low byte; `Sats` (satellites used) in its high | as written |
  | 13 | `SOG`, the speed over ground | as written |
  | 14 | `COG`, the course over ground, degrees | |
  | 15 | a word the exports leave out, 814 to 2,272 in the sample files | as written |

- *The exports.* The CSV and the tab-separated text file hold every fix of their `.rtx` but the
  last, in the same order: the time as `HHMMSS`; a position as `DDMM.mmmmm` and its hemisphere's
  letter, or, in 5 of the 9 sample exports, in signed degrees cut toward zero at the sixth
  decimal and a degree sign; the altitude in whole feet, the meters × 3.280 84 rounded down, which
  the reader converts back to meters; and the rest as the `.rtx` holds them. The text file's header names two
  empty columns its rows don't have.
- *Units.*
  - *Altitude, meters.* The manual says the GPS altitude "is always expressed as Mean Sea Level"
    (p13); the KML export beside each sample file writes the word as meters, and the CSV and text
    exports convert it to feet. All three come from mDACS reading the same word, so the survey
    adds one more check, reported but not enforced: the climb from the pad to the highest fix
    agrees with the barometric altimeter flown beside the tracker, as above, where feet would be
    3.3 times off. The fifth file's list is full and holds a climb of 0.003 times the
    altimeter's: its memory filled before the flight.
  - *Altitude at 0.* The file can't hold an altitude below 0, so the receiver's altitude looks
    clamped there: in 4 of the 5 sample files, from pads near sea level, most fixes read exactly
    0. A note counts them; each means sea level or below.
  - *Course, degrees.* It lies 1.7° to 6.9° from the bearing between the fixes either side of
    it, the median difference either way in each file, over the fixes whose speed over ground
    reads at least 15 in the file's own numbers.
  - *Speed over ground and velocity, as written.* The speed over ground runs 3.5 to 4.0 times
    the speed between fixes in m/s, by file, as km/h would, one of the two speed units the
    tracker's display offers (p12), but nothing says which unit the file holds, and feet a
    second would give 3.3. The velocity reads far more than the climb rate in m/s while the
    rocket climbs, and about the fall rate itself while it falls, a pattern no document
    explains. Plotted, both show when the tracker moved, but not how fast in any known unit.
- *The clock.* The GPS's time of day, UTC, in whole seconds, with no date; the log's zero is the
  first fix's. Each time takes the day that puts it nearest the one before it, so the clock runs
  on past midnight UTC, and a repeat written across midnight goes back to the day before. In 3 of the 5
  sample files a pair of fixes repeats the pair before it a second back; every fix is kept in the
  file's order, with a note. In one file, one fix's time reads 65,282 × 65,536 more than it
  should, as a garbled radio packet's might: a time that isn't a time of day is a gap, counted in
  a note.
- *Gaps and refusals.* A coordinate out of range (minutes past 59, the decimals without their
  leading 1, a hemisphere byte other than its two letters, beyond 90° or 180°) is a gap, as is a
  position of exactly (0, 0); no sample file has one. An `.rtx` that isn't the list's shape, or
  whose size or slots break the rules above, is refused, naming the byte; an export's row of
  other than ten cells, or a cell that isn't its field's, is refused, naming the line.

For example, the crate's tests read an invented `.rtx` and its exports. The first fix is at
17:59:50 UTC, `HHMMSS` 175950, more than 16 bits hold, so the record splits it: word 1 is
$\lfloor 175\,950 / 65\,536 \rfloor = 2$ and word 0 is $175\,950 - 2 \times 65\,536 = 44\,878$.
The file holds word 0 as the slot 656,705,342, the remainder of $44\,878 \times 387\,420\,489$
after dividing by $10^9$; the reader multiplies it by 513,180,409 and keeps the last nine digits,
44,878 again. The latitude's words are 3259, 35,698 and 1: $1 \times 65\,536 + 35\,698 =
101\,234$, a leading 1 and the decimals `01234`, so the fix is 32° and 59.01234′ north,
$32 + 59.01234/60 = 32.983\,539$°. Its altitude word is 1,400 m (4,593 ft). The CSV export
writes the fix as

```text
175950,3259.01234N,10658.54321W,4593,0,2,122,9,12,40
```

its altitude $\lfloor 1400 \times 3.280\,84 \rfloor = 4593$ ft, read back as
$4593 \times 0.3048 = 1399.95$ m. The text export writes the longitude in degrees, `-106.975720°`,
where the `.rtx` holds $-(106 + 58.54321/60)$, which is $-106.975\,720\,17$°: cut, not
rounded, at the sixth decimal.

## PerfectFlite's other files

Besides the `.pf2`, which has [a page of its own](pf2.md), PerfectFlite's software exports a
flight as CSV or text, and an older PerfectFlite altimeter saves its flights as a `.pfa`.

### PerfectFlite's exports

PerfectFlite's software exports a flight as CSV, `Time,Altitude,Velocity,Temperature (F),Voltage`,
sometimes with `Acceleration (calculated)` after them, or as tab-separated text, `Time`,
`Altitude`, `Temp.`, `Voltage`, whose temperatures carry their unit letter, such as `58.7F`. One
sample CSV export names its temperature `Temp`, without the dot, and is read alike. Some text
exports have no header row at all: rows of four tab-separated numbers, the third a temperature
with its unit letter, `F` or `C` (`0.05`, `3`, `71.5F`, `9.2`). No other layout HPR Sim reads
writes a unit's letter after a number in such a row, and among the sample logs only PerfectFlite's
text exports do, so such a row is read as the text export's four columns, and the record's notes
say that its columns were taken, not read. That is a guess at which column is which, made from
the layout alone.

The units are a `.pf2`'s: seconds, feet and feet per second. The header doesn't state the
altitude's unit, and the sample logs bear out feet. Of the 21 exports among them, 16 peak at
exactly the height of the `.pf2` saved beside them, which states feet. Read in feet, those 16
climb in 0.94 to 1.05 times as long as a fall in vacuum from their peak would take. The other 5
climb in 0.63 to 1.07 times, read in feet, and would climb in 0.35 to 0.59 times read in meters
([the survey's report](#how-it-was-checked) prints these, and each of the 5 by id). Of the 4
headerless exports, 2 are one flight, exported once with its temperatures in °F and once in °C,
and both peak at exactly the height of the `.pf2` beside them: the headerless layout writes feet,
whatever its temperature unit. A third climbs in 0.99 times a fall in vacuum read in feet, among
the matched exports' 0.94 to 1.05, and in 0.55 times read in meters; the `.pf2` files beside it
are other flights'. The fourth has no `.pf2` beside it, and its climb settles nothing: 0.63 read
in feet and 0.35 in meters both fall outside that band, and its altitude is ragged near its peak.
Its feet rest on the layout's convention alone; check its altitude against another source before
trusting it. An export doesn't name the
altimeter's model, so its logger reads `PerfectFlite`.

A header with any other name, or these names in another case, isn't read as PerfectFlite's: the
RRC3's `Time,Altitude,Pressure,…` is the RRC3's, and a bare `time,altitude` says nothing of its
units.

### PerfectFlite's older `.pfa`

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
80 ft or more, and reads as $88 \times 0.3048 = 26.82$ m above the pad.

DataCap's help also says a `NumSamps:` of 2,200 (a revision 2 altimeter) or 2,400 (the original)
likely means the altimeter lost power in flight, and that older flights then follow this one in
the data. Such a file reads, with a note saying so; none of the sample files is one. A line among
the heights that isn't a whole number, or an empty line between them, is refused, naming the line:
every line is a step of 0.1 s, so one left out would move every time after it.

## Raw downloads

A raw download is an altimeter's memory as its maker's program copies it to a computer, before
that program turns it into an export with columns and units. HPR Sim reads four: the
Featherweight Raven's `.FIPa` and Missile Works' RRC3 `.rff`, described first, then AltOS's
`.eeprom` and the ARTS2's `.odf`, each in its own section below
([AltOS's `.eeprom`](#altoss-eeprom), [the ARTS2's raw data](#the-arts2s-raw-data)). Each holds
the sensors' readings as whole numbers, *counts*. None of the four holds an altitude. The Raven's
and the RRC3's counts are turned into SI by the maker's own arithmetic; AltOS's and the ARTS2's
by the sensors' data sheets and the calibration the file carries, and a count with no source
stays as written.

> **How far to trust it.** The logger's own readings, decoded from layouts no maker publishes, and
> checked sample by sample against the makers' exports of the same flights.
>
> - **Raven `.FIPa`:** 55 of the 57 sample files have the FIP's export beside them, and in all 55
>   every sample of every channel the two share agrees, 6,016,104 raw samples, with no value off
>   by a step. The other 2 are decoded by the same rules, unchecked by an export.
> - **RRC3 `.rff`:** 1 of the 6 has mDACS's export beside it, and its 3,541 pressures agree, each
>   to the step. The pairs and single words have no calibration to check; the other 5 files are
>   decoded by the same rules, unchecked by an export.
> - **AltOS `.eeprom`:** 9 of the 14 sample files have AltosUI's export beside them, and all 9
>   agree within half its print step on every value converted to SI; 1 more has only the board's
>   radio telemetry, which agrees; the other 4 have nothing beside them. The TeleMetrum v1's
>   pressure assumes its converter's reference is the sensor's supply, which no document states;
>   the TeleMega v4's IMU isn't read. [Its section](#altoss-eeprom) has the details.
> - **ARTS2 `.odf`:** all 3 sample files have the raw CSV and DataAnalyzer's interpreted export of
>   the same flight beside them. The raw CSVs hold the same 48,614 records word for word, and the
>   exports' acceleration agrees within 0.080 of a count on every record after the first. The
>   rate rests on DataAnalyzer's exports and a climb check, not on the file.
>   [Its section](#the-arts2s-raw-data) has the details.
>
> Not compared: the exports' altitudes (but the order of the ARTS2's), flags and the velocities
> they work out, which are their programs' conversions, not the altimeter's words. Where a number matters, set it beside the
> maker's program's export of the same flight.

### The Raven's `.FIPa`

The Featherweight Interface Program (FIP) saves a `.FIPa` as a few text lines starting `#`, then
the memory. The text lines give the firmware, the flight's number, the settings and the length of
a tick of the Raven's clock. The memory comes in frames of a `:`, 256 bytes and a checksum byte.
A frame holds four blocks of 32 words of 16 bits, and each block covers 0.05 s, its words taking
turns among the channels:

- the axial accelerometer, 11 bits, 400 times a second;
- the lateral accelerometer, 10 bits, 200 times a second;
- ten slow channels, a word each, 20 times a second: the current draw, the four pyro channels'
  voltages, the battery's voltage, the temperature, the pressure, the speed the Raven integrates
  from its accelerometer, and the sixth, which the FIP doesn't export (the reader names it
  `slow channel 5`, counting from zero).

The rates follow from the blocks' layout, and the FIP's export times its rows by them;
Featherweight's manual says the Raven records the data its deployment logic uses at 20 Hz
(*Raven User's Manual, updated for the Raven4*, 2020-08-15). The first five blocks are the quarter second before the Raven detected the launch,
and the sixth is the settings block, which holds the accelerometers' calibration and the ground's
elevation. Time zero is the first sample before the launch, as in the FIP's export.

Each word becomes the value the FIP's export prints, by the export's own arithmetic, and then SI
as the [Raven FIP CSV](#the-layouts-read) reader reads that column. The pressure word counts
1/25,000 of an atmosphere, the voltages thousandths of a volt, the temperature hundredths of a
degree Fahrenheit, and the speed tenths of a foot per second. Kept as written, as *unrecognized*:
the sixth slow channel, the two words after each block's samples, and, on a Raven whose header says its
accelerometer has one axis (one sample file), the lateral bits, as `lateral counts`, since that
Raven calibrates no lateral axis. A frame whose checksum fails is refused, naming its byte.

**A worked example**, with invented numbers, which the crate's test `tests/raven_fipa.rs` reads
and checks. The axial accelerometer reads in g as

$$
\frac{1}{100}\left[\operatorname{trunc}\left(\frac{\mathrm{X1}\,(a - \mathrm{Z1})}{1000}\right) + \mathrm{F3}\right],
$$

where $a$ is the count; the settings block gives the gain X1, the zero Z1, and the offset F3 in
hundredths of a g; trunc drops the fraction. Take a gain of 8,200, a zero of 1,000 counts and an
offset of 5 hundredths. A count of 1,025 is 25 above the zero. The FIP works $\mathrm{X1}/1000$ in
double-precision floating point, where 8.2 is stored a hair below 8.2, so $8.2 \times 25$ comes to
204.999… and truncates to 204. Adding the offset gives 209 hundredths: 2.09 g, or 20.50 m/s².
Exact whole-number arithmetic would give 205 and 2.10 g. The reader works as the FIP does.

### The RRC3's `.rff`

mDACS, Missile Works' program, saves an RRC3's download as a .NET serialized object, the layout
Microsoft publishes as MS-NRBF, holding one list of 16-bit words: the altimeter's memory as it
came off the board. The list has no columns and no times, so the reader tells the words apart by
their size, as Debrief, the project owner's earlier analyzer, does:

- **A word below `0x4000` (16,384) is a pressure**, in tenths of a millibar. `0x2710`, 10,000, is
  1,000.0 mbar, or 100,000 Pa. `0x4000` would be 1,638.4 mbar, above the 1,200 mbar at which the
  RRC3 itself reports a pressure sensor fault (*RRC3+ / RRC3 User Manual* v1.71, p. 9), so no
  reading reaches it.
- **A word at or above `0x4000` is an auxiliary word.** Most come in pairs once a second,
  presumably the temperature and battery voltage the RRC3 records once a second, in counts. A few
  stand alone (three in each sample file: one near apogee, one later, and a last `0xFFFF`); the
  first two fall where the manual's low-voltage samples, taken as the drogue and the main fire,
  would, but that is a guess. The file holds no calibration for either, so both are kept as
  written, as *unrecognized*, in channels of their own.
- **A zero word ends the log.** The words before the first one that can be a launch pad's
  pressure, at least 500 mbar, among the first eight, are kept in a note.

The pressures are on the RRC3's 20 Hz clock, from its manual (*RRC3+ / RRC3 User Manual* v1.71,
p. 23): the *n*th at *n*/20 s. The file has no times to confirm it, so the reader checks it
another way: the once-a-second pairs must count the same seconds as the pressures do at 20 a
second, within 10% and two pairs, or the file is refused.

### What the record keeps

Neither file holds an altitude: the altitudes in the FIP's and mDACS's exports are those programs'
conversions of the pressure. So each record has an empty [primary track](#the-record), with a
note, as a [GPS log](#gps-logs) and the Blue Raven's high-rate file have, and its channels hold
everything, the pressure among them. Working an altitude out of the pressure is a reading, which
comes with [M7.2](../decisions-and-roadmap.md#m7-2). From the Raven's settings block, the ground's
elevation becomes the launch site's, and its flight number and firmware are kept as stated.

### Checked against the makers' exports

A raw download and its maker's export of the same flight are two readings of the same bytes, so
one checks the other sample by sample. `cargo xtask logs` pairs each raw file with every export
in its folder that the export's own reader reads. It counts a match when every sample of every
channel the two share agrees: the value within half a step of the raw format's resolution, and
the time as the export prints it. Every raw sample counts once. The FIP's export prints a row
every tick of the Raven's clock and holds each channel's latest sample in it, so every raw sample
must show at a row of its own time, none skipped. Only the velocity may stop before the raw
file's last sample, as the FIP stops it at apogee; its samples after the stop are counted apart.
`cargo xtask logs --check` fails on a raw file with an export beside it that matches none. The
[survey's report](#how-it-was-checked) prints the counts, in its section on the raw downloads:

| Layout | Files | With an export beside | Matching | Raw samples compared | Velocity after the stop | Worst value, steps | Worst time, s |
|---|---:|---:|---:|---:|---:|---:|---:|
| Raven `.FIPa` | 57 | 55 | 55 | 6,016,104 | 140,865 | 0 | 0.0005 |
| RRC3 `.rff` | 6 | 1 | 1 | 3,541 | 0 | 0 | 0.01 |

The worst times are the exports' own rounding. The FIP's newer exports print a time to six
significant figures, 0.0005 s at most here. mDACS keeps its clock as 0.05 s added up row by row in
single precision, which runs a hundredth of a second ahead of the RRC3's 20 Hz from 156.65 s;
worked that way, the survey reproduces all 3,541 of its export's times exactly. Every value
matches only with the FIP's own arithmetic: with exact whole-number arithmetic for the
accelerometer's gain, 11 files differ from their exports by a hundredth of a g, as in the worked
example above, and 44 of the 55 would match ([ADR-259](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0259-raw-downloads-against-their-makers-exports-first.md),
a decision record, gives the probes). The one RRC3 export is mDACS's, in UTF-16
with a UTF-8 byte-order mark written a second time inside it; the importer's text decoding now
drops that mark, so the export reads as [RRC3 CSV](#the-layouts-read).

Two `.FIPa` files and five `.rff` files have no export beside them. They are decoded by the same
rules, and pass the checks each file carries (every frame's checksum, the RRC3's clock check),
but no export checks their values.

### AltOS's `.eeprom`

HPR Sim reads the log an Altus Metrum flight computer keeps in its own memory, as AltosUI, Altus
Metrum's ground software, downloads it and saves it to an `.eeprom` file; only the Rust library
reads it so far. Altus Metrum publishes no layout for these records, so the reader's layout was
worked out from the 14 sample files and the AltosUI exports (CSV) of the same flights beside 9 of
them, without AltOS's or AltosUI's source code. Each unit it converts rests on a sensor's data
sheet and a second check, and a count with no source is kept as written.

> **How far to trust it.** The board's own readings, decoded from a layout the maker doesn't
> publish, and checked against what lies beside each file.
>
> - **Checked:** 9 of the 14 files against AltosUI's export of the same flight, every value
>   converted to SI within half the export's print step and every row of the export at a tick the
>   file holds; 2 against the board's radio telemetry, every pressure at a shared tick equal (1 of
>   the 2 also has an export); and the 12 files from boards with an accelerometer by the climb
>   check below: 11 pass, their ratios from 0.859 to 1.400 where the reader accepts 0.6 to 5/3,
>   and the twelfth, at 1.988, keeps its acceleration and pressure as counts.
> - **Assumed:** that the TeleMetrum v1's converter measures against the pressure sensor's own
>   supply, which no document states; the exports of its flights agree.
> - **Not read:** the TeleMega v4's IMU, which its one sample file never ran.
> - **Unchecked:** 4 files have neither an export nor telemetry beside them (14 = 9 with an export,
>   1 with telemetry alone, and these 4). They're decoded by the same rules and pass the reader's
>   own checks.

The board logs its sensors 100 times a second in the climb and 10 times a second in the descent
(*The Altus Metrum System*, Bdale Garbee and Keith Packard, v1.9.22.2, 2025, Appendix F, Table 11,
p. 77). Each record is a type letter, a checksum byte, the board's 16-bit tick and the type's
fields: `F` the flight (its number, the accelerometer's and the barometer's readings on the pad),
`A` the sensors, `S` a change of state, and on a board with GPS, records of its fix and its
satellites. The record size follows the log format the configuration states:

| Log format | Record size | Boards |
|---:|---:|---|
| 1 | 8 bytes | TeleMetrum v1 |
| 6 | 16 bytes | EasyMini v1 |
| 7 | 16 bytes | TeleMetrum v2 and newer |
| 14 | 16 bytes | EasyMini v2 |
| 15 | 32 bytes | TeleMega v4 |
| 16 | 32 bytes | EasyMega v2 |
| 22 | 32 bytes | TeleMega v6 |

Any other number refuses the file, as misaligned bytes would read as a plausible flight. Firmware
before 1.0 states no log format; a TeleMetrum v1's file is then format 1, as the manual's release
notes for version 1.4 say AltosUI does (§H.43.2, p. 128), and any other product's is refused. The
survey's report marks the files whose format came from the product. A file comes in one of two
forms:

- **JSON**, as AltosUI writes today: the board's configuration as a JSON object naming
  `log_format` and `product`, then the memory's bytes in hexadecimal. Every record's bytes sum,
  with 0x5A, to 0 modulo 256, the seed of the line checksum of the TeleDongle, the radio receiver
  AltosUI listens through (*AltOS Telemetry*, v1.9.22.2, §4, p. 10); a record that fails refuses
  the file. All 29,437 records of the 7 JSON sample files pass. Erased memory, every byte 0xFF,
  ends the log and is counted in a note; memory that ends partway through a record is refused.
- **Text**, as older AltosUI and a serial console wrote: the configuration as the board prints it
  (`Config version:`, `manufacturer altusmetrum.org` and the rest), then one record a line, its
  checksum left out. A last line with no line break, as a download cut short leaves, is left out
  with a note.

What the reader does with the records:

- **The clock** is the board's tick, a hundredth of a second (*AltOS Companion Port*, v1.9.22.2,
  Table 1), which turns over every 655.36 s; each record goes on the turn nearest the latest tick
  so far. The records aren't written in time order: the pad's samples, held in a ring buffer, come
  after the flight record, and a GPS record carries its fix's earlier tick. So they are sorted by
  tick, the file's order kept among equal ticks. Time zero is the flight record's tick, the
  board's detection of boost, as in AltosUI's exports, so the pad's samples have negative times. A
  record more than 10 s from the latest tick ends the flight, and it and those after it are
  counted in a note: one sample file ends with 14 records of another session, more than 10 s
  later. Records repeated exactly, the same bytes at the same tick, as old firmware wrote some,
  are dropped and counted. A file needs exactly one `F` record.
- **Pressure and temperature, MS5607** (the barometer chip on every board but the TeleMetrum v1):
  the data sheet's compensation of its two counts, D1 and D2, by the six constants the
  configuration lists, with its second order below 20 °C (TE Connectivity, *MS5607-02BA03*,
  Figures 2 and 3, pp. 8–9). Each division by a power of 2 is a floor, as an arithmetic shift is:
  truncating toward zero instead gives different pressures below 20 °C, and a unit test pins the
  floor. Second checks: the pad's pressure must lie from 10 to 120 kPa and within 1% of the flight
  record's, and the pad's temperature within the chip's operating range, −40 to 85 °C (p. 8), or
  the file is refused.
- **Pressure, TeleMetrum v1:** its MP3H6115A's output is `Vout = Vs (0.009 P − 0.095)`, P in kPa
  (Freescale, *MP3H6115A*, rev. 5.1, p. 5), read by the converter of the CC1111, the board's radio
  microcontroller. The converter's single-ended result reaches 2,047 at its reference and sits in
  the top bits of its 16-bit result, 2,047 × 16 = 32,752 at full scale (TI, *CC1110Fx/CC1111Fx*,
  SWRS033H, §12.10.2.6, p. 143, and p. 144). That the converter's reference is the sensor's own
  supply is the board's design, which no document states. The second checks are the pad's
  pressure, from 10 to 120 kPa, or the file is refused, and the climb check below, or the counts
  are kept as written, since that band alone rejects almost no count; the exports agree too. The
  sample
  files' pressures step by 13.57 Pa (the survey's report prints each file's smallest step), about
  1.13 m (3.7 ft) of height at sea level.
- **Acceleration**, along the board's axis: AltosUI calibrates the accelerometer antenna up, then
  down (the manual, §12.6.20, p. 42), and the configuration keeps its counts at +1 g and −1 g.
  The record's value is `(ground − count) / ((minus − plus) / 2) × 9.806 65` m/s², `ground` the
  flight record's count on the pad: 0 at rest on the pad, positive forward along the flight path.
  A board mounted antenna down logs flipped counts and keeps a flipped calibration with the same
  span, so either reads alike, as long as both counts come from one pair; the configuration's
  pad orientation is kept as a stated figure. Without a calibration, with one outside a 16-bit
  count's range, without a pressure in pascals to check it against, or failing the climb check
  below, the counts are kept as written.
- **The climb check** holds the pressure and the acceleration to each other on every board with
  an accelerometer. Over the first second after boost, the climb by the pressure, its pressure
  altitude in the U.S. Standard Atmosphere, 1976, which is ISO 2533's below 32 km (20 mi), must
  agree with the climb by the acceleration, integrated twice from rest on the pad: their ratio
  must lie from 0.6 to 5/3. Outside it, or where the samples don't cover that second or leave a
  gap in it of more than 0.1 s, neither unit is borne out, so the acceleration stays in counts,
  and so does a TeleMetrum v1's pressure, which has no other check; an MS5607 board's pressure
  keeps its flight record's check. On 11 of the 12 sample files with an accelerometer the ratio
  runs from 0.859 to 1.400: the barometer reads the pressure at its ports, which the airflow past
  them can lower in the climb. The twelfth, an early TeleMetrum v1 flight with nothing beside it,
  reads 1.988: its two sensors disagree by a factor of 2, and nothing in the file says which is
  wrong, so both stay in counts. The band catches swapped or misplaced fields, a wrong sign, and a
  calibration off by 2 or more on a flight whose ratio is near 1. Without a calibration, an MS5607
  board's pressure's second checks are the pad's alone, and a TeleMetrum v1's pressure stays in
  counts.
- **A TeleMega v4.0 or v6.0's or an EasyMega v2.0's pyro battery, igniter, apogee and main
  senses**, the voltages across the pyro channels that show continuity, become volts by the
  divider *AltOS Telemetry* gives a 15 V board (p. 3); the exports agree. The same page gives
  another divider for 30 V boards, and a later TeleMega takes 30 V (the manual's release notes
  for 1.9.20), so any other Mega's senses are kept as written.
- **GPS** latitude and longitude are in ten-millionths of a degree and altitude in meters, as
  in the telemetry's GPS packet (*AltOS Telemetry*, Table 15, pp. 6–7); a record without a valid
  fix is a gap.
- **States** become events, named as AltosUI's exports name the numbers: 2 `pad`, 3 `boost`,
  4 `fast`, 5 `coast`, 6 `drogue`, 7 `main`, 8 `landed`.
- **Kept as written:** the battery and other senses and the nine words of the IMU, the inertial
  unit's accelerometer, gyroscope and magnetometer, whose factors fit the exports but have no
  source; the TeleMetrum v1's chip temperature, because no document gives the conversion AltosUI
  uses and the CC1111 data sheet's constants don't reproduce its export; the GPS's other fields.
  Not read: the TeleMega v4's IMU, and a Mega's IMU when the flight record's IMU fields are all
  zero.

The file holds no altitude: the record's [primary track](#the-record) is empty, as for the other
[raw downloads](#raw-downloads), and every field is a channel. The configuration gives the serial
number and firmware, the `F` record the flight's number, and the record's stated figures hold the
pad's readings, the calibration, the pad orientation, the main deployment height and the apogee
delay.

**A worked example**, invented. A TeleMetrum v2's flight record, flight 3, in the JSON form:

```text
46 90 64 01 03 00 19 00 d0 7e 01 00 00 00 00 00
```

| Bytes | Field | Value |
|---|---|---|
| `46` | type | `F`, the flight record |
| `90` | checksum | 0x5A and all 16 bytes sum to 0x300, 0 modulo 256 |
| `6401` | tick, low byte first | 356; this record's tick is time zero |
| `0300` | flight | 3 |
| `1900` | accelerometer on the pad | 25 counts |
| `d07e0100` | pressure on the pad | 98,000 Pa |
| `00000000` | unused | |

The text form prints the same record as `F 164 03 00 19 00 d0 7e 01 00 00 00 00 00`: the tick as
one hexadecimal number, `164`, while the other bytes keep their order. Take a calibration of
16,000 counts at +1 g and 16,800 at −1 g: 400 counts a g. With 16,010 on the pad, a count of
12,010 in the climb is (16,010 − 12,010) / 400 = 10 g, 98.07 m/s² above rest. The MS5607 data
sheet's own example counts, D1 6,465,444 and D2 8,077,636, read 110,002 Pa at 20.00 °C, which a
test pins.

**Checked.** `cargo xtask logs` pairs each file with an AltosUI export in its folder of the same
serial number and flight, read by the export's column names, and lines the two up by tick. At each
of the file's samples, the export's row at that hundredth must agree within half the export's
print step, and each of the export's rows must fall at a tick the file holds a sample at; with the
board's `.telem` file of the same serial number, each pressure at a tick both hold must be equal.
`cargo xtask logs --check` fails on any file that differs. Of the 14 files:

| Compared | Files | Values | Worst difference | Half the print step |
|---|---:|---:|---:|---:|
| Pressure, against the export | 9 | 38,908 | 0.005 Pa | 0.005 Pa |
| Temperature, against the export | 7 | 31,673 | 0.05 K | 0.05 K |
| Acceleration, against the export | 7 | 33,095 | 0.005 m/s² | 0.005 m/s² |
| A Mega's voltages, against the export | 3 | 9,968 | 0.005 V | 0.005 V |
| The export's rows, at a tick the file holds | 9 | 47,547 | 0 without | |
| Pressure, against the telemetry | 2 | 231 | 0 Pa | |

The MS5607 boards' pressures match their exports exactly; the TeleMetrum v1's, a conversion,
within half the export's 0.01 Pa print step. One export, the damaged one the survey's report
files as `altos-csv-damaged`, zeroes the acceleration at the +1 g calibration instead of the
pad's reading, so it differs from the file by one constant; the survey finds that constant
within half a step and counts the file apart. The same export counts its time from the tick
itself. Per file, by an anonymized id, the [survey's report](#how-it-was-checked) prints each
count and each climb ratio.

Left out: the TeleMega v4's IMU and the fields kept as written, until a source or a second check
bears their units out; and any log format not listed above, which no sample holds.

### The ARTS2's raw data

HPR Sim reads an Ozark Aerospace ARTS2's flight in two more files: the `.odf` file DataAnalyzer,
the board's PC software, saves a downloaded flight to, and the sensors' counts as DataAnalyzer
exports them to CSV. Only the Rust library reads them so far. The *ARTS2 User Manual* (V1.0.0,
Ozark Aerospace, 2007) describes neither file, so the layout was worked out from the 3 sample
flights that hold both, each with DataAnalyzer's interpreted export of the same flight beside it
([an ARTS2's unit system](#units-the-file-decides)), without any other program's source code. The
`.odf` carries the board's calibration, so its acceleration and pressure are read in SI; the raw
CSV carries none, so its counts are kept as written.

> **How far to trust it.** The board's own readings, decoded from a layout the maker doesn't
> publish, and checked against the files of the same flight beside each.
>
> - **Checked:** all 3 sample `.odf` files. Their raw CSVs hold the same 48,614 records word for
>   word, 0.02 s apart. Against DataAnalyzer's interpreted exports, the acceleration agrees within
>   0.080 of a count on all 48,611 records after the first, and each export's barometric altitude
>   follows the pressure in order. The climb check, the climb over the first 2 s by the pressure
>   over the climb by the acceleration, gives ratios of 0.754, 1.032 and 1.145, where the reader
>   accepts 0.6 to 5/3.
> - **Assumed:** the rate, 50 records a second, which DataAnalyzer gives all three files. Where the
>   file keeps it isn't known: the reader refuses a file whose likeliest word for it isn't the
>   sample files' 2, and on the sample files the climb check is sure to catch only a time step
>   below 0.67 or above 1.38 times the true one. The pressure is the sensor's nominal conversion, with an error band of
>   ±1.5 kPa; an AltOS board flown with each flight reads a pad pressure 0.42 to 1.08 kPa above
>   the ARTS2's.
> - **Not read:** the meaning of most of the flags byte, the board's state codes and the
>   deployment settings.

The ARTS2 reads an Analog Devices ADXL78 accelerometer, set for ±50 g, and a Motorola MPXA4100A
pressure sensor with its processor's 10-bit converter, so each count runs from 0 to 1,023 (the
manual, Appendix A, p. 16). The `.odf` is little-endian throughout, and its length must be
exactly its parts':

| Offset | Bytes | What |
|---:|---:|---|
| 0 | 4 + 9 | the length 9, then `Ozark.odf` |
| 13 | 20 | five 32-bit words, the same in every sample file; not read |
| 33 | 256 | the board's settings and calibration (below) |
| 289 | 4 | the number of records, `n` |
| 293 | 12 | three 32-bit words, each the memory's size in records; equal, and at least `n` |
| 305 | 8 `n` | the records |
| 305 + 8 `n` | 124 | DataAnalyzer's own settings; not read |

Of the settings block, the reader takes four things, each a 16-bit little-endian word. At its
offset 8 is a word that is 2 in every sample file, the likeliest place for the recording rate,
which the owner sets (pp. 6–7); a file where it is anything else is refused, as its clock would
be unknown. At 10 is the barometer's count
on the pad, the "Zero Altitude baseline" the board averages on entering flight mode (p. 17). At 12
is eight times the accelerometer's count on the pad, its "Zero G baseline" (p. 16), so it keeps an
eighth of a count. At 20, 22 and 24 are three sums of 64 accelerometer readings, taken when the
board was calibrated "at +1g, −1g, and 0" (p. 16): in every sample file the middle one is the
reading on its side, midway between the two ends.

Each record is four 16-bit words, in the order of the raw CSV's columns: the accelerometer's count
(`Acc(HI)`), a byte of flags (`Acc/Baro(LO)`), the barometer's count (`Baro(HI)`) and the board's
state (`State`). The first record is the two baselines, not a sample: the pad's barometer count,
and the accelerometer's cut to a whole count. A file whose first record isn't so is refused, as
its bytes would be laid out otherwise than the sample files'. DataAnalyzer exports that record as
the flight's first row, at time 0, and so does the reader.

What the reader does with the records:

- **The clock.** The records are 0.02 s apart, 50 a second, from the first: the rate DataAnalyzer
  gives every sample file in both its exports. Where the file keeps the rate isn't known: the
  sample files are all at one rate, so no byte can be shown to change with it. The reader refuses
  a file whose settings word at 8 isn't 2, and the climb check below bounds the clock otherwise.
- **Acceleration**, along the board's axis, toward its forward end, from the file's own
  calibration. The two end sums are 2 g apart, so a g is a 128th of their difference, in counts:
  64 readings in each sum, over 2 g. The pad's reading is taken out, as DataAnalyzer's export
  takes it out, so the rocket at rest on the pad reads zero, gravity with it. With $c$ the count,
  $c_{\mathrm{pad}}$ the pad's baseline and $k$ the counts a g,

  $$
  a = \pm\frac{c - c_{\mathrm{pad}}}{k}\, g_0,
  $$

  the sign set by the pad: standing nose up, the board reads +1 g along its axis, so the pad's
  reading lies near one end sum, and a count moving further toward that end is an acceleration
  forward, positive. The
  ADXL78's data sheet must bear the calibration out (*ADXL78*, Rev. B, Analog Devices, 2010, p. 3,
  for ±50 g): a g from 36.1 to 39.9 mV, 7.39 to 8.17 counts on the board's 5 V supply; the side's
  reading within 150 mV, 30.72 counts, of half the supply and between the two ends; and the pad's
  reading nearer an end than the side's. Otherwise the file is refused.
- **Pressure**, by the MPX4100A's nominal transfer function, the first line below, with $V$ the
  sensor's output, $V_S$ its supply and $P$ in kPa (*MPX4100A*, Rev 9, Freescale, 2009, p. 5).
  The converter reads the output against the supply as its full scale, so $V/V_S$ is $c/1024$,
  which gives the second:

  $$
  \begin{aligned}
  V &= V_S\,(0.010\,59\,P - 0.1518) \\
  P &= \frac{c/1024 + 0.1518}{0.010\,59}
  \end{aligned}
  $$

  One count is 92.2 Pa, about 7.7 m (25 ft) of height near sea level. The pad's pressure must lie
  in the sensor's rated 20 to 105 kPa (p. 3), or the file is refused. The sensor's error band is
  ±1.5 kPa over that range from 0 to 85 °C (p. 6).
- **The climb check** holds the clock and both units to each other, as for
  [AltOS's `.eeprom`](#altoss-eeprom). Over the first 2 s from the first record, the climb by the
  pressure, its pressure altitude in the U.S. Standard Atmosphere, 1976, must agree with the climb
  by the acceleration, integrated twice from rest: their ratio must lie from 0.6 to 5/3. The
  window is twice AltOS's 1 s because this barometer is coarse: over 1 s, one sample flight
  climbs less than two counts. The check bounds the clock only loosely: the climb by the
  acceleration goes as the square of the time step, so with a flight's ratio $r$ on its true
  clock, a step $k$ times the true one passes for $k$ from $\sqrt{r/(5/3)}$ to $\sqrt{r/0.6}$,
  0.67 to 1.12 for the sample file of the least ratio and 0.83 to 1.38 for the greatest's.
- **A file that fails the climb check is refused.** AltOS's reader keeps such a file's counts as
  written; this one can't, because a count kept as written still needs a clock, and nothing else
  in the file bears one out.
- **Kept as written:** the flags byte and the state. Bits 3 and 2 of the flags byte are the
  barometer count's low two bits in every record of every sample file, and a record whose aren't
  is refused, a check that the words are where the reader takes them. The other bits follow the
  flight, but no document names them. The state steps through 0, 1, 2, 3, 5, 6 and 7 in each
  flight, in order; no document names those either, so no event is taken from it.

The file holds no altitude: the record's [primary track](#the-record) is empty, as for the other
[raw downloads](#raw-downloads), and every word is a channel. The record's stated figures hold the
two baselines and the three calibration sums.

**A worked example**, from the invented file the crate's test `tests/arts2_raw.rs` reads. Its
settings block holds a pad barometer count of 800, an accelerometer word of 4,100 (512.5 counts),
and calibration sums of 31,744 (64 × 496), 32,250 and 32,768 (64 × 512). The ends differ by 1,024,
so a g is 1,024 / 128 = 8 counts, inside the data sheet's 7.39 to 8.17; the side's reading,
32,250 / 64 = 503.9, lies between the ends and within 30.72 of 512; and the pad's 512.5 lies near
the 512 end, so the acceleration is positive as the count rises. A count of 576 in the boost is
(576 − 512.5) / 8 = 7.94 g, 77.84 m/s² above rest. The pad's barometer count reads
(800 / 1,024 + 0.1518) / 0.010 59 = 88.107 kPa, inside the sensor's 20 to 105 kPa.

**The raw CSV** is DataAnalyzer's raw export of the same records: a
`MET,Acc(HI),Acc/Baro(LO),Baro(HI),State` header, then a row for each record, its time and its four
words as whole numbers:

```text
MET,Acc(HI),Acc/Baro(LO),Baro(HI),State
0.000,512,0,800,64
0.020,576,0,800,0
```

It holds no calibration, only the first record's whole-count baselines, so no column is
converted: the two counts and the flags byte are kept as written, the state as a number, and
`MET`, DataAnalyzer's clock, as the time. Every row must hold five numbers, its four words whole
and in range, a time after the row before's, and a flags byte holding the barometer's low bits,
or the file is refused, naming the line. For values in units, read the flight's `.odf`.

**Checked.** `cargo xtask logs` pairs each `.odf` with the files of its flight in its folder. A
raw CSV must hold its records word for word, 0.02 s apart. DataAnalyzer's interpreted export must
agree in `Acc` within half a count on every record after the first, and read zero in `Acc` and
`pAlt` at its first row and at any row after the records. Its `pAlt` is compared by order only:
zero at the pad's pressure and above, it must rise as the pressure one record earlier falls below
it, each pressure meeting one value, and no row's `pAlt` may be missing.
`cargo xtask logs --check` fails on a file that differs. Of the 3 `.odf` files:

| Compared | Files | Values | Result |
|---|---:|---:|---|
| Records, against the raw CSV | 3 | 48,614 | every word equal |
| `Acc`, against the interpreted export | 3 | 48,611 | within 0.080 of a count |
| `pAlt`'s order, against the pressure | 3 | 48,611 | no pressure meeting two values; none out of order; no gap |
| The climb check | 3 | | ratios 0.754, 1.032 and 1.145 |

Reported, not checked: two exports' `pAlt` lies within 3.3 m (11 ft) of the standard
atmosphere's pressure altitude from the `.odf`'s pressure, and the third's departs by up to
151 m (496 ft), worked by some other formula. An AltOS board flew beside the ARTS2 on each
flight: the ARTS2's pad pressure reads 0.42 to 1.08 kPa below it, and its least pressure from
0.48 kPa below to 0.11 kPa above, inside the MPX4100A's ±1.5 kPa error band. The
[survey's report](#how-it-was-checked) prints each file's counts and climb ratio under an
anonymized id, and
[ADR-266, the ARTS2's raw data](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0266-arts2-raw-data-before-the-rest.md)
gives the evidence for the layout.

Left out: the flags byte's bits beyond the barometer's two, the state codes' meanings, the
deployment settings and the rest of the settings block; and where the file keeps its rate, which
the climb check stands in for.

## How it was checked

`cargo xtask logs`, a command of the project's own build tool, reads every sample log the project
holds: the private collection's catalog of flight logs, the public fixtures and samples of
Debrief, and AerospaceNU's published flight logs, each distinct file once. It leaves out a cache
of downloads whose flights' logs the collection already catalogs, and the data folder of
RocketPy's repository, tables RocketPy processed; the teams' logs from it that the collection
catalogs are read like any other. It writes what came of each to
[`validation/reports/log-import.md`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/log-import.md):
per layout, how many files were read and how many refused and why; how many files' readers
decided each unit their headers leave out, and how; how each raw download compares with its
maker's export ([above](#checked-against-the-makers-exports)); how each Featherweight summary
compares with the files of samples beside it ([above](#checked-against-the-files-beside)); how
AltOS's telemetry compares with AltosUI's export beside it, and its board's estimates with each
other ([above](#altos-telemetry)); how each AltOS `.eeprom` file compares with the export and the
telemetry beside it ([above](#altoss-eeprom)); how each ARTS2 `.odf` compares with the raw CSV
and the interpreted export beside it ([above](#the-arts2s-raw-data)); how each RTx `.rtx`
compares with the CSV, text and KML exports beside it ([above](#missile-works-rtx)); how each
table read by its header had its columns decided, as counts ([above](#tables-read-by-their-headers)); and
per file an anonymized id (the start of a hash of the file, which says nothing of its contents)
with a digest, a hash, of the record read from it. The files inside the zip archives among them
are unpacked and counted too, each once: of the 29 in the 8 archives, 19 are in no other source
([zip archives](#zip-archives)), and each Excel workbook's sheets by the layout each reads as
([Excel workbooks](#excel-workbooks)), each table read by a mapping its publisher states
([above](#mappings-a-publisher-states)), and the range of the Eggtimer files' filtered-altitude
multiplier ([above](#units-the-file-decides)), and each file of binary records against its
team's table ([above](#binary-records-by-a-stated-layout)), and each Blue Raven USB download
against the survey's own decoding of it ([above](#the-blue-ravens-usb-download)), and each Teensy
logger's file against the survey's decoding and its physical checks
([above](#how-the-teensy-files-were-checked)), and each file of radio packets against the
survey's decoding, and each received or exported packet against the onboard log's
([above](#how-the-radio-packets-were-checked)). Of the 1,122
files it holds today, 1,020 are read, the 8 archives counted as read when they unpack and the 16
workbooks when a sheet of each reads. The report labels 41 files for
[M7.1c20](../decisions-and-roadmap.md#m7-1c20), the next increment.
[ADR-274](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0274-the-rest-of-the-importers-planned-teensy-logger-first.md)
plans all five increments left, through the last file; 39 of the 41, of layouts no reader
knows yet, are unread.

A file the collection's catalog files under a layout it doesn't have is moved, by id, with the
reason printed: one is a spreadsheet's re-save of an AltOS export, its header edited, which is
generic CSV; another is an AltOS export whose header line is damaged, two column names run
together, which no reader can line up with its rows and which is refused; a third, a KML file, is
a map of a flight made in a map program, its predicted tracks with the pad and landing spot
pinned, read but counted apart as not a log. And 62 files the catalog files as generic CSV, by
their extension, are in AltimeterCloud's layout (10), Entacore AIM's (15), a Fluctus's (6), an
ARTS2's interpreted data (4) or the phone app's `launch` CSV (27), and are held to those readers;
2 more are Eggtimer downloads saved with semicolons, one held to the Eggtimer's reader and one,
its decimal points taken out, counted as damaged ([above](#units-the-file-decides)); 6 are
tables whose publisher states their mapping ([above](#mappings-a-publisher-states)); and one is
an RRC3 export counted as damaged ([above](#units-the-file-decides)).
Of the 3 RTF files the catalog files as not a log, by their extension, one is an Eggtimer's
download, held to its reader, and two exports of radio packets, held to the packets' reader
([above](#one-teams-radio-telemetry-packets)). Sixteen spreadsheets are workbooks of flights'
logs, held to the workbook reader. Three files filed as text are console captures and two filed
as binary are files of records, each read by the layout stated for it
([above](#binary-records-by-a-stated-layout)). Two more filed as text are a Blue Raven's USB
downloads, held to their reader ([above](#the-blue-ravens-usb-download)), and eight more are a
course's Teensy logger's, held to theirs ([above](#a-courses-teensy-data-logger)), and two
are radio packets, the onboard log and the receiver's text, held to the packets' reader
([above](#one-teams-radio-telemetry-packets)). Five files filed as
generic CSV or text are
PerfectFlite's exports: four with no header row and one that names its temperature `Temp`. A
Blue Raven high-rate file with no date is held to the Blue Raven's reader; its low-rate partner,
which a person edited in a spreadsheet (a column with no name, two columns named in words the
app doesn't write), was counted as damaged until
[M7.1c13](../decisions-and-roadmap.md#m7-1c13), and is now read as a
[table by its header](#tables-read-by-their-headers), and 30 other files filed as generic CSV. Three more are an ARTS2's raw CSV, held to
its reader. Nine are mDACS's exports of an RTx's fixes, five CSV and four text, held to their
reader. One more is moved to a layout not read yet: a ground station's conversion of a
SillyGoose flight, its columns in another order under no header.

Some files aren't flight logs, and are counted apart as not logs, as pictures and maps are. By
id: four spreadsheets of a team's checklist for preparing a flight, its steps and no measurement,
nine spreadsheets of a Blue Raven's deployment settings, a `Hardware_Channel:` row naming its
outputs, and three spreadsheets of a flyer's summary of flights, a few rows of figures by flight
or by launch and no samples. By family: Debrief's six made-up sample flights, whose first row says each isn't a
recording of anything, and twelve text files a flyer's own program wrote from a Blue Raven's
low-rate file, titled as that file's events. By family too, since
[M7.1c17](../decisions-and-roadmap.md#m7-1c17): one text file the same flyer's program wrote
from a Blue Raven's summary CSV beside it, the summary's rows as cells between `|`: a copy, every
cell equal to the CSV's, and the CSV is read. By id, since
[M7.1c7](../decisions-and-roadmap.md#m7-1c7): two more of those events files, and three
descriptions of a rocket written for a flyer's or a team's own program; three one-row files of a
team flight computer's saved state (a boot count, gyro biases, the pressure before boost); and an
OpenRocket simulation's export, a simulation and not a flight. Inside the archives, by id: three
more events files, one more description of a rocket, a Blue Raven's settings as a spreadsheet
and as CSV, and a wind profile, altitude against wind speed and direction.

The collection is other people's data, Debrief's fixtures come from flights shared on terms nobody
recorded, and AerospaceNU's are published under the GPL, so no file and no value read from one is
committed. The digest pins each reading instead, as a snapshot (a stored copy of a test's output)
would: the check fails if a reader's output changes or a file of these layouts stops reading.
Invented files in the crate's tests are read in CI and pinned whole by snapshots.
