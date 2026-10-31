# Flight logs: the record and the layouts read

See which altimeters' flight logs HPR Sim reads, what it keeps from each, and how that was checked.

An altimeter or flight computer saves its flight as a file. The file holds a clock and the readings
of its sensors. Examples are a barometer's altitude, an accelerometer, a battery's voltage and, on
some, a GNSS receiver's position. Each maker writes its own layout. HPR Sim reads each layout into
one record, the same for every logger. That way, what comes after (the readings, and later the
comparison with a simulation) doesn't depend on which altimeter flew.

> **How far to trust it.** Every sample log the project holds of these layouts is read, except
> 3 damaged files and 4 workbook sheets ([Excel workbooks](#excel-workbooks))
> ([how it was checked](#how-it-was-checked)). No maker publishes a
> specification of these layouts. So each reader follows what the files show, the makers' manuals
> and the notes of Debrief, the project owner's earlier flight-log analyzer. Each unit is checked
> where the files allow. The few units no source states are named
> [below](#units-no-source-states). Many of the sections below open with their own note on how far to
> trust their layout's reading. Other layouts are refused, not guessed at.

**What is checked, and what is not.** One damaged file is an AltOS export whose header
line is damaged. Another is an Eggtimer download whose decimal points were taken out. The third
is an RRC3 export whose rows were sorted out of order. Where a header names no unit, the reader
decides it from the file and says how in a note ([units the file decides](#units-the-file-decides)).
In the four layouts read since
[M7.1c4](../decisions-and-roadmap.md#m7-1c4), a unit two checks don't settle is kept as written.
The file's number is stored unchanged, with no SI value made from it. So many of their columns
keep the file's own numbers, and their readings come only from a height or pressure whose unit
was read. The Raven's `.FIPa` and the RRC3's `.rff` raw downloads match their makers' exports of the same flights wherever an export sits
beside them. Of their 63 sample files, 7 have no export beside them, and those 7 are unchecked
([raw downloads](#raw-downloads)) ([evidence row G6](../evidence.md#g6)). The Featherweight
app's flight summaries hold the logger's own figures and no samples. Only their maximum and pad
altitudes are checked, against the files of samples beside them
([Featherweight's summaries](#featherweights-summaries)) ([evidence row G7](../evidence.md#g7)).
No document describes the bytes of the AltiMAX's `.ALS` or the AIM's `.xtra`, so each of those
layouts rests on one sample flight. The layouts the roadmap names with no sample
are [listed](#formats-that-need-samples), with what a reader would need
([evidence row G9](../evidence.md#g9)).

**To read a flight** from these layouts today, use the Rust library: give the file's bytes to
`hpr_flightdata::import::read` ([API reference](../api/hpr_flightdata/import/index.html)). It
recognizes the layout from the file's content, never its name, and reads it or refuses it, naming
the layouts it knows. A file of binary records has no layout to recognize. Read it with
`records::read`, or a console capture of them with `capture::read`, by the layout you state
([below](#binary-records-by-a-stated-layout)):

```rust,ignore
let bytes = std::fs::read("flight.csv")?;
let log = hpr_flightdata::import::read(&bytes)?;
for channel in &log.channels {
    println!("{} ({:?}): {} samples", channel.column, channel.quantity, channel.values.len());
}
```

[`hpr analyze`](../cli.md#hpr-analyze), the command line, and Python read each layout `import::read`
recognizes by its own reader. The same tools read a table or binary records by a column mapping or a
record layout ([Reading your own logger](../your-own-logger.md)). Since [M7.2b, readings on every
logger](../decisions-and-roadmap.md#m7-2b), the readings (liftoff, apogee, the top speed and
landing) come from any record's height track. A height track is the logger's own barometric
altitude, a pressure turned into a height by the 1976 standard atmosphere, or an altitude channel.
Each reading is within a stated bound or withheld with its reason. A GNSS height gives none
([Flight-log readings](../physics/log-readings.md#the-height-track)). Since [M7.2c, the rest of the
readings](../decisions-and-roadmap.md#m7-2c), the same track gives the flight time and each canopy's
descent. Since the same milestone, an accelerometer column the reader names as the rocket's axis gives the top acceleration
and burnout. These columns are AltOS's `acceleration`, the Raven's and Blue Raven's
`Axial Accel (Gs)`, the Eggtimer's `Accel`, the ARTS2's `Acc`, among others. AltOS's `acceleration`
doesn't count when it comes from an EasyMini or a TeleMini, which have no accelerometer. On those
boards the column is then the barometer's estimate. The file shows this by having neither an
inertial unit's columns nor a GPS receiver's. A board's three axes don't say which lies along the
rocket, so they give neither. Nor does an axial column whose pad, coast and thrust don't settle
which way up it reads. Nor does one that shows no thrust (`direction_unstated`). Six records of the
sample logs have such an axial column: three AltOS CSV, two Entacore AIM CSV and a Raven CSV ([the
accelerometer](../physics/log-readings.md#the-accelerometer)). The survey's report counts the
readings each family gives and withholds ([how it was checked](#how-it-was-checked)).

The readers were written for these milestones, each as the decision record beside it decided:

- [M7.1a](../decisions-and-roadmap.md#m7-1a) (the record, the survey of the sample logs, and the
  PerfectFlite, AltOS and Raven readers), as
  [ADR-251](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0251-flight-log-importers-in-two-increments.md) decided.
- [M7.1b](../decisions-and-roadmap.md#m7-1b) (the Blue Raven, RRC3 and Eggtimer readers), as
  [ADR-253](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0253-rrc3-eggtimer-and-blue-raven-csv-first.md) decided.
- [M7.1c1](../decisions-and-roadmap.md#m7-1c1) (the GPS logs), as
  [ADR-254](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0254-gps-tracks-first-among-the-other-importers.md) decided.
- [M7.1c2](../decisions-and-roadmap.md#m7-1c2) (AltimeterCloud, Entacore AIM, PerfectFlite's `.pfa`
  and an Eggtimer TRS's terminal text), as
  [ADR-255](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0255-text-loggers-before-the-rest-of-the-importers.md) decided.
- [M7.1c3](../decisions-and-roadmap.md#m7-1c3) (the SillyGoose, Fluctus and ARTS2 readers), as
  [ADR-256](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0256-three-club-and-team-layouts-before-the-rest.md) decided.
- [M7.1c4](../decisions-and-roadmap.md#m7-1c4) (AerospaceNU's flight computer board and ground
  station, a phone app's `launch` CSV and an Eggtimer's RTF), as
  [ADR-258](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0258-aerospacenus-board-and-a-phone-apps-csv-before-the-rest.md) decided.
- [M7.1c5](../decisions-and-roadmap.md#m7-1c5) (the Raven's `.FIPa` and the RRC3's `.rff` raw
  downloads), as
  [ADR-259](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0259-raw-downloads-against-their-makers-exports-first.md) decided.
- [M7.1c6](../decisions-and-roadmap.md#m7-1c6) (Featherweight's summaries, and the files filed as
  not logs), as
  [ADR-260](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0260-featherweight-summaries-before-the-rest.md) decided.
- [M7.1c7](../decisions-and-roadmap.md#m7-1c7) (zip archives, PerfectFlite's exports with no header
  row and Blue Raven files with no date), as
  [ADR-261](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0261-archives-and-two-readers-variants-before-the-rest.md) decided.
- [M7.1c8](../decisions-and-roadmap.md#m7-1c8) (the R-DAS's raw text), as
  [ADR-262](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0262-the-r-das-raw-text-before-the-rest.md) decided.
- [M7.1c9](../decisions-and-roadmap.md#m7-1c9) (AltOS's radio telemetry), as
  [ADR-263](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0263-altos-telemetry-before-the-rest.md) decided.
- [M7.1c10](../decisions-and-roadmap.md#m7-1c10) (AltOS's `.eeprom` downloads), as
  [ADR-265](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0265-altos-raw-downloads-before-the-rest.md) decided.
- [M7.1c11](../decisions-and-roadmap.md#m7-1c11) (the ARTS2's `.odf` and raw CSV), as
  [ADR-266](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0266-arts2-raw-data-before-the-rest.md) decided.
- [M7.1c12](../decisions-and-roadmap.md#m7-1c12) (Missile Works' RTx), as
  [ADR-267](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0267-missile-works-rtx-before-the-rest.md) decided.
- [M7.1c13](../decisions-and-roadmap.md#m7-1c13) (tables read by their headers), as
  [ADR-268](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0268-tables-read-by-their-headers-before-the-rest.md) decided.
- [M7.1c14](../decisions-and-roadmap.md#m7-1c14) (Excel workbooks), as
  [ADR-269](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0269-excel-workbooks-before-the-rest.md) decided.
- [M7.1c15](../decisions-and-roadmap.md#m7-1c15) (European-locale exports and mappings a publisher
  states), as
  [ADR-270](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0270-euroc-exports-and-stated-mappings-before-the-rest.md) decided.
- [M7.1c16](../decisions-and-roadmap.md#m7-1c16) (binary records by a stated layout), as
  [ADR-271](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0271-binary-records-by-a-stated-layout-before-the-rest.md) decided.
- [M7.1c17](../decisions-and-roadmap.md#m7-1c17) (the Blue Raven's USB download), as
  [ADR-272](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0272-the-blue-ravens-usb-download-before-the-rest.md) decided.
- [M7.1c18](../decisions-and-roadmap.md#m7-1c18) (a course's Teensy data logger), as
  [ADR-274](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0274-the-rest-of-the-importers-planned-teensy-logger-first.md) decided.
- [M7.1c19](../decisions-and-roadmap.md#m7-1c19) (one team's radio telemetry packets), as
  [ADR-274](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0274-the-rest-of-the-importers-planned-teensy-logger-first.md) decided.
- [M7.1c20](../decisions-and-roadmap.md#m7-1c20) (the FIP export pasted below no header, and the AltiMAX's Excel
  export), as
  [ADR-274](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0274-the-rest-of-the-importers-planned-teensy-logger-first.md) decided.
- [M7.1c21](../decisions-and-roadmap.md#m7-1c21) (a MARSA download, AltOS's telemetry as a table, and its CSV after
  a spreadsheet), as
  [ADR-274](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0274-the-rest-of-the-importers-planned-teensy-logger-first.md) decided.
- [M7.1c22](../decisions-and-roadmap.md#m7-1c22) (tables a mapping holds to physics), as
  [ADR-274](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0274-the-rest-of-the-importers-planned-teensy-logger-first.md) decided.
- [M7.1c23](../decisions-and-roadmap.md#m7-1c23) (the AltiMAX's `.ALS` and the AIM's `.xtra` and raw
  dump), as
  [ADR-274](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0274-the-rest-of-the-importers-planned-teensy-logger-first.md) decided.

## The layouts read

| Layout | Loggers | Recognized by | Reader |
|---|---|---|---|
| PerfectFlite `.pf2` | Pnut, StratoLogger, StratoLoggerCF | names PerfectFlite | [`.pf2` files](pf2.md) |
| PerfectFlite CSV export | the same, through PerfectFlite's software | `Time,Altitude,` and more of the export's own column names | [below](#perfectflites-exports) |
| PerfectFlite text export | the same | the same names, separated by tabs; or, with no header row, rows of four tab-separated numbers, the third a temperature followed by its unit's letter, `F` or `C`, such as `71.5F` | [below](#perfectflites-exports) |
| PerfectFlite `.pfa` | Alt15K/WD, an older PerfectFlite altimeter, through PerfectFlite's DataCap program (AltCapWD on the Mac) | names PerfectFlite and has a `**!!Data:` line | [below](#perfectflites-older-pfa) |
| AltOS CSV | Altus Metrum's TeleMetrum, TeleMega, EasyMega, EasyMini, through AltosUI (Altus Metrum's ground software) | a full export: `#version,serial,flight,`, or `#` comment lines and then that header or `version,serial,flight,`. A table cut from an export: a header of AltOS's own names that includes `time`, `state` and `state_name`, where at least one of the first 100 rows pairs a state number with its AltOS name | `hpr_flightdata::altos` ([below](#altoss-csv-after-a-spreadsheet)) |
| Raven FIP CSV | Featherweight Raven, through the Featherweight Interface Program | opens with a time column and holds the Raven's `Axial Accel (Gs)` channel | `hpr_flightdata::raven` |
| Raven `.FIPa` | Featherweight Raven, its memory as the Featherweight Interface Program downloads it, before any export | opens `#Component=Raven`, and its text lines end, within the first 4,096 bytes, in a `#den` line followed by a frame's `:` | `hpr_flightdata::raven_fipa` ([below](#raw-downloads)) |
| Blue Raven CSV | Featherweight Blue Raven, through its phone app: a low-rate file (barometer, inertial figures, flags) and a high-rate file (gyroscopes, accelerometers, attitude) | `Year,Month,Day,Time,Flight_Time_(s),Sync,`, or `Flight_Time_(s),Sync,` in a file with no date | `hpr_flightdata::blueraven` |
| Blue Raven USB download | Featherweight Blue Raven, its low-rate or high-rate set as its USB port sends it to a serial terminal, one line a sample | a line among the first 1,000 that opens `@ LOG_HIR` or `@ LOG_LOW` and closes with a CRC that checks | `hpr_flightdata::blueraven_usb` ([below](#the-blue-ravens-usb-download)) |
| Teensy data logger, tab-separated | a university course's own logger, built around a Teensy board, its older layout: one sensor's reading a line | its first line is a tag, a whole-number clock and values, by tabs, and one of its first 100 is an `Alt` line with `P` and `MSL` | `hpr_flightdata::teensy` ([below](#a-courses-teensy-data-logger)) |
| Radio packets | one university team's flight computer's radio packets, `255:…:254`: its own log, or the text a receiver wrote of what it heard | its first line is a packet, a whole number or a `Gain[…]` line, and one of its first 100 is a packet | `hpr_flightdata::packets` ([below](#one-teams-radio-telemetry-packets)) |
| Radio packets in RTF | the same packets in an RTF document a spreadsheet saved as CSV, `255,…,254` | an RTF document, one of whose first 100 lines, its markup taken off, is a packet | `hpr_flightdata::packets` ([below](#one-teams-radio-telemetry-packets)) |
| Teensy data logger, named fields | the same course's logger, its newer layout | its first line states a sensor's units, such as `ICM data in us, milli-g, …`, and one of its first 100 is a tag, `t`, a clock and name-value pairs, by commas | `hpr_flightdata::teensy` ([below](#a-courses-teensy-data-logger)) |
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
| GPX | GPS tracks in the GPS Exchange Format, version 1.0 or 1.1; no sample log is one, so the reader is checked on invented files only ([evidence row G1](../evidence.md#g1)) | the document's root element is `gpx` | [below](#gpx) |
| NMEA 0183 | a GPS receiver's own sentences, written to a file as it sends them | at least 3 of the first 50 lines open with a sentence whose checksum checks out, such as `$GPGGA,…*hh`, or, in a shorter file, every line does | [below](#gps-logs) |
| R-DAS raw text | AED Electronics R-DAS, its raw data as its Windows software exports a flight | a first line that isn't blank reading `*** Configuration ***`, and a `*** Raw data ***` line within the first 200 | `hpr_flightdata::rdas` ([below](#r-das-raw-text)) |
| MARSA download | MARSA Systems' MARSA4 and MARSA54, a flight as their software, MARSA_Connect, saves it to a `.dat` file, or a spreadsheet re-saves it | a first line that isn't blank that is a setting or a heading, `<NAME> value` or `<NAME>`, and a `<MARSA VERSION>` setting with a value within the first 200 lines that aren't blank | `hpr_flightdata::marsa` ([below](#a-marsa-download)) |
| AltOS telemetry | Altus Metrum's TeleMetrum v2 and newer, the packets a TeleDongle receiver hears, as AltosUI saves them to a `.telem` file | a first line that isn't blank reading `TELEM`, a space and an even number of hexadecimal digits | `hpr_flightdata::altos_telem` ([below](#altos-telemetry)) |
| AltOS telemetry CSV | the same packets decoded to a table, a TeleMega's `0x09` packets one a row | a first line that isn't blank opening `serial,tick,ptype,` | `hpr_flightdata::altos_telem_csv` ([below](#altoss-telemetry-as-a-table)) |
| AltOS `.eeprom` | Altus Metrum's TeleMetrum, TeleMega, EasyMega and EasyMini, the board's own log as AltosUI downloads it | a JSON object naming `log_format` and `product`; or, within the first 1,000 lines, a `Config version:` line and a `manufacturer altusmetrum.org` line | `hpr_flightdata::altos_eeprom` ([below](#altoss-eeprom)) |
| RTx `.rtx` | Missile Works' RTx GPS tracker, its fixes as mDACS (Missile Works' program) saves a download | a .NET serialization header (MS-NRBF, version 1.0), then the class record of a list of 32-bit integers | `hpr_flightdata::rtx` ([below](#missile-works-rtx)) |
| RTx CSV and text | the same fixes, as mDACS exports them | a first line that isn't blank reading `UTC Time,Latitude,Longitude,Altitude,Velocity,VDOP,NavStatus,Sats,SOG,COG`, or the text file's tab-separated `UTC`, `Lat`, an empty name, `Lon`, an empty name, `Alt`, `Vel`, `VDOP`, `Nav`, `Sats`, `SOG`, `COG` | `hpr_flightdata::rtx` ([below](#missile-works-rtx)) |
| AltiMAX `.ALS` | Rocketronics AltiMAX, a flight as the altimeter writes it to its memory card | a header stating at least one record, then records of 22 bytes, the first up to 1,000 each holding a first byte with bit 0 set and bit 1 clear and its voltage channel the one after the record before's | `hpr_flightdata::altimax_als` ([below](#the-altimaxs-als)) |
| Entacore AIM `.xtra` | Entacore AIM XTRA, a flight as the AIM software saves it | a Boost serialization archive's signature, `serialization::archive`, the top object's header the sample has, and a first record whose class is an AIM kind's | `hpr_flightdata::entacore_raw` ([below](#the-aims-xtra-and-raw-dump)) |
| Entacore AIM raw dump | the same flight computer's memory, as the AIM software downloads it before any conversion | a first byte `0x13`, a recording's opening, then the AIM's records, up to 1,000, or erased memory | `hpr_flightdata::entacore_raw` ([below](#the-aims-xtra-and-raw-dump)); `import::read` takes a dump of one recording, and `entacore_raw::read_dump` each recording of several; `import::read_one`, as `hpr analyze --recording` does, reads the one chosen |
| Table read by its header | a logger no other reader knows: a team's flight computer, a logging script, a spreadsheet saved as CSV | tried after every other layout: a header row naming a clock with a unit of time, such as `Time (s)` or `time__us`, then a row with a number there | `hpr_flightdata::generic` ([below](#tables-read-by-their-headers)) |
| Excel workbook, `.xlsx` | a flyer saving a log, or several, as a spreadsheet | not by `import::read`, which refuses an archive: `workbook::read` opens it, then each sheet is recognized as a file of its own; `import::read_one`, as `hpr analyze --sheet` does, reads the sheet chosen, or with none the one sheet that reads | `hpr_flightdata::workbook` ([below](#excel-workbooks)) |
| Raven FIP rows below no header | a Raven FIP CSV pasted into a workbook with its header line missing | not recognized: read under the header line you state | `hpr_flightdata::raven::read_rows`, and `custom::read_by_fip_header` from a workbook's sheet, as `hpr analyze --fip-header` does ([below](#a-fip-export-pasted-below-no-header)) |
| AltiMAX Excel export, `.xls` | Rocketronics AltiMAX, a flight as its PC software exports it to Excel | not by `import::read`, which refuses the `.xls` bytes: `xls::read` writes the worksheet out as text, whose header's first five names are `TIME,PRESS FILTER [hpa],HEIGHT FILTER [m],PRESS RAW [hpa],HEIGHT RAW [m]`; `import::read_one`, as `hpr analyze` does, reads it | `hpr_flightdata::xls` and `hpr_flightdata::altimax` ([below](#the-altimaxs-excel-export)) |
| Zip archive | not a log: a flight's files, zipped to share them | its bytes open `PK` then 3, 4 | `hpr_flightdata::archive` ([below](#zip-archives)) |
| Binary records | a team's flight computer saving each sample as a fixed run of bytes, with no header | not recognized: read by the record layout you state | `hpr_flightdata::records` ([below](#binary-records-by-a-stated-layout)) |
| Console capture | the same records, dumped as base64 by the flight computer's shell and saved by a terminal program | a start marker line, `----++++//[[( name start )]]\\++++----`, in its first 1,000 lines; `import::read` refuses it as stating no layout | `hpr_flightdata::capture` ([below](#binary-records-by-a-stated-layout)) |

The Blue Raven app writes each flight as two files. The low-rate file is logged 50 times a second,
and the high-rate file 500 times a second (*Blue Raven User's Guide*, May 12, 2026, p18). Each is
read on its own, as a record of its own, not joined to the other. The high-rate file holds no
altitude. So its record's [primary track](#the-record) is empty and its channels hold everything.
The low-rate file of the same flight holds the altitude. The app's summary of the flight is read
too, as a record of figures with no samples ([below](#featherweights-summaries)).

Some files among the sample logs aren't flight logs, and are counted apart as such. They are:

- Debrief's made-up sample flights, which say so in their first row;
- a team's checklists for preparing a flight;
- a Blue Raven's deployment settings saved as spreadsheets or CSV;
- a flyer's summaries of flights in spreadsheets;
- the events files a flyer's own program wrote from a Blue Raven's low-rate file;
- that program's copy of a Blue Raven's summary;
- the descriptions of a rocket such programs read;
- a team flight computer's saved state, one row;
- an OpenRocket simulation's export;
- a wind profile;
- a flight computer's and a battery's housekeeping;
- two GPS tracks' coordinates copied from their maps, with no clock;
- three tables a program worked out from a log.

Every other file in the sample logs is in a layout read here, except 3 damaged files. The
[survey's report](#how-it-was-checked) counts each kind.

### Zip archives

Flyers often share a flight as one zip archive. It may hold a Blue Raven's high-rate and low-rate
files with the app's summary, or a folder of everything the flight left behind. An archive isn't a
log, so `hpr_flightdata::import::read` refuses one with a reason that says so. Instead,
`hpr_flightdata::archive::unpack` takes its files out, in memory, each a name and its bytes. Each
file is read as it would be on its own. Nothing is written to disk, so a file's name, folders and
all, is only a label. An archive inside an archive is one of its files, and isn't opened.

```rust,ignore
let bytes = std::fs::read("flight.zip")?;
for member in hpr_flightdata::archive::unpack(&bytes)? {
    match hpr_flightdata::import::read(&member.bytes) {
        Ok(log) => println!("{}: {} ({} channels)", member.name, log.format.name(), log.channels.len()),
        Err(error) => println!("{}: not read, {error}", member.name),
    }
}
```

An archive comes from someone else. A small one can be built to unpack to far more than memory
holds (a *zip bomb*). So at most 256 MiB (268 MB) come out of one archive. That is about eight
times the largest sample archive, which unpacks to 32 MB. The limit counts the bytes as they
decompress, never from the sizes the archive states. An archive is refused whole, with its reason,
and none of its files is read, when:

- two of its files share a name (the `zip` library HPR Sim uses would silently keep only one);
- a file is a symbolic link, or a folder's entry holds bytes;
- a file is encrypted;
- a file is compressed by a method other than the two that archives of logs use, *stored* (not
  compressed) and *deflated* (zip's usual compression);
- a file fails its CRC-32, the checksum every zip file carries to catch damage.

The crate's tests build an archive of each kind and check its refusal. Two property tests feed
random and damaged archives to the reader. They show the reader refuses them or stays within the
bound, and never crashes. Only the Rust library opens archives so far: `hpr analyze` and the Python
package refuse one, and the command line asks for the log inside it. A workbook, though built on
a zip archive, they read one sheet at a time ([Excel workbooks](#excel-workbooks)).

The 8 archives among the sample logs hold 29 files. 10 are byte-for-byte copies of files already
among the samples. The other 19 files are 8 Blue Raven high-rate and low-rate files, 3 of the
app's summaries and a GPS tracker's KML track. The remaining 7 aren't logs. Every one of the 12
logs reads. The 3 summaries are checked against the Blue Raven files in their archive, as the
others are ([checked against the files beside](#checked-against-the-files-beside)). All 3 agree.
Two CSV files in one archive name other altimeters' formats in their file names. They use the Blue
Raven's column names, but their values aren't the Blue Raven's. At the same moments, their
altitudes differ from its low-rate file's by up to several hundred feet. Which logger wrote them
isn't known, so they aren't read as a Blue Raven's. They are read as
[tables by their headers](#tables-read-by-their-headers) since
[M7.1c13](../decisions-and-roadmap.md#m7-1c13). Their altitude is in the feet the header states,
and the rest of their columns are as written.

### Excel workbooks

Some flyers keep a flight's log in an Excel workbook (`.xlsx`). It may be the Blue Raven app's
files pasted side by side as sheets, or a table whose header names its units. `hpr analyze` and
the Python package read one sheet at a time, in one of two ways. The first is by the sheet's own
reader. The sheet is chosen with `--sheet N` (`sheet=` in Python), counting from 0. With none
chosen, it is the workbook's one sheet that reads as a log. The second is by a column mapping and
the sheet's number
([A file of several records](../cli.md#a-file-of-several-records)). In Rust,
`hpr_flightdata::import::read_one` reads one sheet as they do. A workbook is a zip
archive of XML parts (ECMA-376, *Office Open XML*, Part 1, §18), so
`hpr_flightdata::import::read` refuses one, as it does any archive. Instead,
`hpr_flightdata::workbook::read` writes each worksheet's cells out as comma-separated text, one
line a row. It then reads each sheet as the file it came from. The file is a Blue Raven's CSV or
its app's summary, an Eggtimer's CSV, a Featherweight tracker's, or a
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
displays it, so no digit is lost or invented. A number is written as its stored digits
(`0.10000000000000001` stays so, though Excel shows `0.1`). A string is written as its text, a
formula as the value saved with it, and a truth value as `TRUE` or `FALSE`. An error such as
`#DIV/0!`, or a formula with no value saved, is left empty, and the log's notes count them. A line
break inside a cell, as in a header wrapped onto two lines, becomes a space. Every row is written
as wide as the sheet's widest, so short rows get empty cells. A row with no value is left out.

**Dates.** A spreadsheet stores a date as a count of days, a *serial number*. It shows the number
as a date only through the cell's *number format*, the display pattern attached to it, such as
`m/d/yy h:mm`. A number under a format that shows a date is written as one. Serial 45,000.75
under `m/d/yy h:mm` is `2023-03-15 18:00:00`, 45,000 days after December 30, 1899 and then three
quarters of a day. Under a format that shows only a time, a number under 1 is written as a time of
day, `18:00:00`. Under that format, a number that also holds a day is written as a date and a time. An elapsed time,
such as `[h]:mm:ss`, counts its hours past 24 (`36:00:00`). Seconds are rounded to the
millisecond. A workbook that counts from 1904, as old Mac workbooks may, is read from 1904. No
sample workbook does, so that path is checked by the crate's tests alone
([evidence row G2](../evidence.md#g2)). A date before March 1, 1900 is kept as the number stored, and
the notes count it. That is the range where Excel's calendar holds a 29 February 1900 that never
was. A date after December 31, 9999 is kept and counted the same way. Every other number is kept as
stored, so a clock a logger wrote in seconds stays a number.

**Bounds.** A workbook comes from someone else. Its parts come out within the
[archive bound](#zip-archives). At most 256 MiB of text is kept from one workbook. That text is
its list of shared strings (the text cells share, stored once) and the text written. Each byte is
counted before it is kept. So a small file of one long string repeated in every cell is refused,
not written out. The sheets are read as a stream: the largest sample sheet is 60 MB of XML. A
workbook is refused, with its reason, when:

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
workbooks (`validation/oracles/xlsx/cells.py`). Every one of the 5,743,747 cells of the 16 that
hold a value agrees, 147,958 of them dates and times. That checks the copying, not the layouts:
each sheet's columns and units are only as right as the reader it goes to. A probe run when the
check was built, not committed, moved the reader's calendar by one day, and 27,103 cells
disagreed. Two more workbooks, Raven FIP exports pasted below no header, are read by the header
stated for each ([below](#a-fip-export-pasted-below-no-header)). The older `.xls` file, a
different binary format, is read by its own reader ([below](#the-altimaxs-excel-export)). Each of
these is checked the same way. Since [M7.1c22](../decisions-and-roadmap.md#m7-1c22), a PerfectFlite
export pasted into a workbook with a person's notes beside it is read by a column mapping. The
notes are left out
([tables a mapping holds to physics](#tables-a-mapping-holds-to-physics)). A state estimator's
output in a workbook is a filter's, not a log. Three more workbooks are flyers' summaries of
flights, not logs.

### R-DAS raw text

HPR Sim reads the R-DAS's raw export but converts nothing in it. The record holds the file's
counts, no altitude and no events. Only the Rust library reads it so far. The R-DAS is a flight
computer AED Electronics sold in the early 2000s. It reads its accelerometer, its barometer and,
as its owner chooses, four digital and six analog inputs, up to 200 times a second. It keeps the
2 s before liftoff (*R-DAS User Manual*, V1.5, 2001, p. 6, p. 7, p. 14). Its Windows software
exports a flight as text, raw or converted with calibration values held in the software. The
sample logs hold the raw export only: 54 files from a college course's flights in 2008. Of these,
28 were recorded on board at 200 Hz and 26 at 10 Hz. By their names, the 26 are the course's
telemetry records.

The raw file opens with a summary of the configuration, then a table of counts. Each count is a
10-bit converter's whole-number reading of 0 to 5 V, 0 to 1,023, a step 4.883 mV. The counts are
to be converted "with the calibration values" (p. 14). The file holds none:

- **`Time`** counts seconds from liftoff (the manual's figure 2.2, p. 10); every sample file
  starts at −2.000. The seconds rest on the configuration's `Data is sampled at N Hz.`: the
  clock's average step must be that interval within 1%, or the file is refused. In every sample
  file it is exact.
- **`Acc`, `Prs` and `ADC0` to `ADC5`** are kept as written, in no unit HPR Sim knows. A value
  that isn't a whole count refuses the file, as a converted export would. The manual's R-DAS has an
  ADXL150 accelerometer (±50 g) and an MPXS4100A barometer, resolved to 0.09 kPa (p. 7). But the
  file doesn't say which board or sensors wrote it, and others have been fitted (a college
  course's [pressure lab](https://pages.hmc.edu/spjut/NewE80/PresTempLab.html) names an
  MPXH6115A). So no data sheet is applied.
- **`Dig`** is read as a hexadecimal word, such as `00B0` (176), and kept as its number. The
  manual says it holds the digital inputs and the flight's phase. The manual also says that "bit 3
  is set when timed recovery was used" (p. 14, p. 22). But the files write a 16-bit word that
  never sets bit 3 of either byte. In 27 of the 28 on-board sample files the word reads `0000`,
  then `0010` within 0.25 s after the zero. After that it reads `0020`, `0030` and, near the end,
  `8030`. The 28th has
  `00B0` and `80B0` for the last two. Those values are unconfirmed as the phases, so no event is
  taken from the word.

The configuration must state the rate and list the inputs once each. The table's header must
name them in order (`Acceleration` as `Acc`, `Pressure` as `Prs`, `the digital inputs` as `Dig`).
A file that fails either is refused. Its lines are kept in a note, and its firmware version is the
record's. The record's [primary track](#the-record) is empty, as for the Raven's `.FIPa`, and every
column is a channel. For example, the file the crate's tests read, invented, eleven rows at 10 Hz
from −0.3 s, one of them shown:

```text
Data is sampled at 10 Hz.
Inputs that were sampled: Acceleration, Pressure, the digital inputs, ADC0, ADC1.
*** Raw data ***
   Time  Acc  Prs  Dig ADC0 ADC1
  0.100  700  799 0010  505  252
```

That row is `Acc` 700 and `Prs` 799 counts and `Dig` 16, at 0.1 s. The note says `Time` is in
seconds. That is because its average step, from −0.3 s to 0.7 s over ten steps, is 0.1, the 10 Hz
interval.

Checked: all 54 sample files read, pinned by the [survey](#how-it-was-checked). Left out:
converting the counts, which needs the software's calibration values, and events from `Dig`. With
no altitude or pressure in a unit, the record gives no reading (`no_height_track`,
[Flight-log readings](../physics/log-readings.md#when-a-reading-is-withheld)).

### AltOS telemetry

HPR Sim reads the telemetry an Altus Metrum flight computer sends by radio, as AltosUI, Altus
Metrum's ground software, saves it. Only the Rust library reads it so far. It is checked on two
sample files, two flights of one team's TeleMetrum. One of them has AltosUI's export (its CSV) of
the board's own log of the same flight beside it. The reader follows the maker's published
description of the packets (*AltOS Telemetry*, Bdale Garbee and Keith Packard, v1.9.22.2, 2025,
§2–4). But one unit, the pressure's, is the exception: the files contradict it. The same packets
decoded to a table are read too ([AltOS's telemetry as a table](#altoss-telemetry-as-a-table)).

The board sends a 32-byte packet several times a second, each of a type: sensors, GPS fix,
satellites, configuration, calibration. A TeleDongle,
the ground station's receiver, writes each packet it hears as a text line, `TELEM` and the bytes
in hexadecimal. AltosUI keeps all such lines in a `.telem` file (*The Altus Metrum System*,
§12.1). Each line holds a length byte, the packet, two bytes the radio adds, and last a checksum.
The radio's two bytes are the signal strength (RSSI, in dBm as $\mathrm{RSSI}/2 - 74$) and a
link-quality byte. The link-quality byte's top bit says whether the packet passed the radio's CRC,
its error check. The checksum is 0x5A plus every byte after the length byte up to, not including,
the checksum, modulo 256. The document's table places the bytes one position early. Its formula as
written fails its own example. This reading, by contrast, passes that example (a test reads the
document's example line) and every line of both files. A line that isn't such a
packet, or fails its checksum, refuses the file, as damage to it. A packet that failed the radio's
CRC is noise and is left out, counted: 13.6% and 4.1% of the two files' packets
([the survey's report](#how-it-was-checked)).

- **The clock** is the board's tick, a hundredth of a second, which turns over every 655.36 s.
  Each packet's tick goes on the turn nearest the latest so far. So a GPS packet, which carries the
  earlier tick of its fix, falls behind instead of a turn ahead. The clock counts from the first
  packet heard, not from liftoff.
- **A board switched off and on** starts its tick again. Each switching on is a power-on, and one
  sample file holds three. The record keeps the power-on that leaves the pad, a state from `boost`
  to `main` (or the last, if none does). It counts the others' packets in a note, and refuses a
  file in which two leave it. The tick alone can't tell every case. A radio silence of 327.68 s to
  655.36 s reads as a power-on. A longer silence reads as a turn shorter than it was. A power-on
  whose new tick lands ahead of the old one reads as a silence.
- **Pressure is in pascals,** not the document's tenths. That is because a launch site's pressure
  in pascals is at most 12 kPa in tenths, more than 15 km (9.3 mi) up. Of the 138 sensor packets at
  a tick the export also holds, 123 equal its pressure. At the other 15 the export's row repeats
  the row before it, counted apart. Pascals are taken only where the pad's pressure lies from 10 to
  120 kPa so read. Otherwise the field is kept as written. One file, from one firmware, has an
  export to hold it to.
- **The board's own estimates** are acceleration, speed and height. Acceleration and speed are in
  sixteenths of a m/s² and of a m/s, and height is in meters above the pad. The estimates are checked against each other. The median ratio
  of the height's climb to the speed after burnout is 0.966 to 1.001. The median ratio of the
  speed's change to the acceleration in the climb is 0.939 to 1.021. A wrong unit would be 3.28 or
  16 times off. The survey fails a file outside 0.8 to 1.25. The highest height is 0.996 of the
  export's.
- **The temperature**, in hundredths of a °C, agrees with the export within 0.05 K, its print
  step. **GPS** positions, in ten-millionths of a degree, agree within 0.000 06° at the 39
  values compared, and altitudes within 3 m (9.8 ft). The survey fails a file past 0.01° or
  100 m (328 ft). A GPS packet without its valid bit holds no fix, and its position is a gap.
- **Kept as written:** the converters' counts (raw accelerometer, battery, continuity,
  calibration), which the document gives no conversion for, and the GPS fields nothing checks.
- **States** become events. They are named as AltosUI's exports name the numbers (2 is `pad`, 3
  `boost`, 4 `fast`, 5 `coast`, 6 `drogue`, 7 `main`, 8 `landed`). The document doesn't give these
  names.

Only the five packet types the sample files hold are decoded. They are the sensors and
calibration of the TeleMetrum v2 and newer, the configuration, and the two GPS packets. The
document lays out eleven more, for the TeleMega, the TeleMini and older boards. Its pressure unit
is wrong where the files can check it. So those eleven aren't decoded until a sample holds one. A
packet of one of those types is counted in a note. The record's [primary track](#the-record) is
the board's height, its own estimate, which may lean on its accelerometer. It is not a conversion
of the pressure alone. Every other field is a channel on its own packets' clock. The crate's tests
read an invented file in which:

- a board is switched on twice;
- a packet fails the radio's CRC;
- a GPS packet's tick falls behind the sensor packet sent before it.

One of that file's lines is a configuration packet:

```text
TELEM 22d2040001040b0300011b0000fa0000084e3043414c4c0000312e392e32320000f4a8c1
```

| Bytes | Field | Value |
|---|---|---|
| `22` | length | 34: the packet and the radio's two bytes |
| `d204` | serial number, low byte first | `1234` |
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

Left out: the eleven packet types no sample holds
([evidence row G3](../evidence.md#g3)); and the GPS fields nothing checks. The
board's own log, which the document doesn't describe, is read from AltosUI's `.eeprom` files
([AltOS's `.eeprom`](#altoss-eeprom)).

### Tables read by their headers

Teams' own flight computers, logging scripts and spreadsheets saved as CSV write tables no maker
documents. Such a table is a header row naming the columns, then one row a sample.
`hpr_flightdata::generic` reads such a table by what its header states and no more. It does so
after every other layout has passed on it
([ADR-268, tables read by their headers](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0268-tables-read-by-their-headers-before-the-rest.md)).

> **How far to trust it.** All 31 such sample tables are read. Besides the clocks, 224 columns
> are in a unit the header states. Another 364 are kept as written by their headers, 5 are kept
> as written by their values, and 11 are of text. No maker stands behind these units. A wrong
> unit whose values stay inside the bounds below is read in it. Milliseconds headed as seconds
> pass the clock's check above about 17 rows a second.

The table that writes its altitude and
speed in both feet and meters agrees with itself to 4.5 × 10⁻¹³ m. The readings come from a
pressure, or from an altitude whose header says it is above the ground or above sea level.
An altitude headed only `Altitude` says nothing of its sensor. So it gives none
(`unstated_sensor`), unless a column mapping marks it as the barometer's
([Flight-log readings](../physics/log-readings.md#the-height-track)).

**The header** is the first line within 64 that isn't blank, a `#` comment or `sep=`. It is split
at `,`, `;`, a tab or `|`, whichever gives the most cells. One column must be the clock: a time
word such as `time` or `t` with a unit of time. A unit is read from brackets at a name's end,
`Alt AGL (ft)`, or from an underscored ending, `acc_y__m_s2`. The
[API reference](../api/hpr_flightdata/generic/index.html) lists every spelling. A decimal comma
is read only where the separator isn't a comma and no number has a point. It is read only where
some comma also stands before other than three digits (`0,05`). A cell in the other mark, as
`1,250` among points, is a gap, never a number a thousand times too small.

**What a column is.** A column is kept as written, with a note saying why, when its name says
one of three things:

- another quantity than its unit does (`vz_filtered_ms`: a speed's name, a time's unit);
- an error or a difference (`Alt error (m)`, `Diff press (Pa)`);
- nothing at all over a length, a time or a unit that may stand for another (`g` is also grams
  and gauss; single letters). `z (m)` may be a height or a distance.

A length is a GNSS, barometric
([AGL](../glossary.md#agl-above-ground-level) or [MSL](../glossary.md#height-above-sea-level-msl))
or inertial altitude where the name says which. Otherwise it is an *altitude* of unstated sensor
and zero. A column of words whose name says a state, phase, status, event or mode gives an
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

Five sample columns failed it. Two headed in pascals read above 110 kPa on the pad. Three headed
in amperes read above 100 A, milliamperes by their size. The clock's median step must lie from
0.1 ms to 60 s, and it must step forward, or the file is refused.

**A worked example.** An invented table:

```text
Time (s),Alt AGL (ft),Pressure (hPa),Velocity,Phase
0.00,0,1001.2,0,PAD
0.05,12,1000.8,45,BOOST
```

`Time (s)` is the clock. `Alt AGL (ft)` is an *altitude* (above ground, but no sensor named):
12 ft is $12 \times 0.3048 = 3.6576\,\mathrm{m}$. 1,000.8 hPa is 100,080 Pa, inside the bounds. `Velocity`
states no unit, so its 45 stays as written. `Phase` gives two events, `PAD` at 0 s and `BOOST` at
0.05 s.

**A column mapping.** `generic::mapping` returns the header's reading as a `Mapping`, which
serializes to JSON. `generic::read_with` reads a file by a mapping, with the same checks. So a
person can state a unit the header leaves out or name the columns of a file with no header row. A
unit that doesn't measure its column's quantity is refused. The command line and Python read a
table by a mapping too, from a JSON file (`hpr_flightdata::custom`). `hpr analyze --print-mapping`
prints the header's reading, and `--mapping` reads the table by an edited mapping
([Reading your own logger](../your-own-logger.md)).

**What is left out.** These tables have no [primary track](#the-record): which column is a
barometric altitude above the pad isn't stated, unless a mapping marks it. The mark is its
`primary`, a column mapped to `altitude_above_ground`, whose clock must go forward. Tables whose
clock has no unit, or that have no header row, are read by a mapping. Either their publisher
states it ([below](#mappings-a-publisher-states)) or it is held to physics
([below](#tables-a-mapping-holds-to-physics)). A table saved in an Excel workbook is read
the same way, once its cells are written out ([Excel workbooks](#excel-workbooks)).

### Mappings a publisher states

Some tables name their clock `t`, `ts` or `time` with no unit, and their columns with no unit
either. So the header reader can't take them, and `import::read` refuses them. When whoever
publishes such a file states what each column holds and its unit, the survey of the sample
logs reads it by that statement. `validation/log-mappings.json` holds, for each file by its
id, the `Mapping` as `generic::read_with` takes it and where it is stated. To read such a file
yourself, write a `Mapping` the same way and pass it to `generic::read_with`. `generic::mapping`
gives one to start from, for a file whose header names its clock's unit. Or save the mapping as a
JSON file for `hpr analyze --mapping` and Python ([Reading your own logger](../your-own-logger.md))
([ADR-270](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0270-euroc-exports-and-stated-mappings-before-the-rest.md), which decided it).

Six of the sample logs are teams' flights that RocketPy, the open-source simulator, keeps in its
repository. RocketPy's example notebook for each flight reads the file with its
`FlightDataImporter`. It names the columns (`"ts": "time"`, `"filtered_altitude_AGL":
"altitude"`, `"filtered_acceleration": "az"`) and passes `units=None`, which the importer's own
documentation defines as every column in SI. That is RocketPy's use of the file, the only
statement of its units there is, not the logging team's own word. So each file's clock is read in
seconds. Its altitude above the ground is read in meters, as an `altitude` whose sensor isn't
stated (barometer or GPS). A vertical speed is read in m/s, and a vertical acceleration in m/s².
That acceleration is the acceleration of the rocket's motion, not an accelerometer's reading. It
is about 0 at rest on the pad and −9.8 m/s² (−32.2 ft/s²) in free fall. Two of the files write
that acceleration as 0 from about apogee on, and it is kept as written. A column the notebook
doesn't map, such as a latitude, is kept as written. Each unit still goes through the values'
check above, and none of the six files' columns fails it.

The survey refuses a mapping on a file whose header doesn't name the mapping's columns in order.
It refuses one on a file a reader reads on its own. A mapping can't stand in for a reader that
takes the file. The survey also refuses a file of this family with none stated. A mapping is only
as right as its publisher's statement. The values' check catches a unit far off, a pressure over
110 kPa or a length beyond 150 km (93 mi). It doesn't catch feet read as meters. A seventh
RocketPy table, a GPS log, is loaded by no notebook. Since
[M7.1c22](../decisions-and-roadmap.md#m7-1c22) that table is read by a mapping held to physics
([below](#tables-a-mapping-holds-to-physics)). Its clock is held to the table it was logged
beside.

### Tables a mapping holds to physics

This section covers the 20 sample tables the survey reads by a mapping it states itself. It also
covers the options a mapping now has for files the header reader can't take. The physical checks
below run in the survey only, on its own mappings. A mapping you write and pass to `read_with` is
held only by [the values' check](#tables-read-by-their-headers). Each check tells a unit from the
ones it could be mistaken for. It says nothing of the logger's accuracy.

The 20 are tables whose headers state too little, or that have none, and whose publishers state
no mapping. They are:

- a university team's flight computer and its radio telemetry;
- student avionics;
- a club's GPS fixes;
- a payload's text;
- small flight computers of hobbyists and teams;
- a flyer's workbook.

The survey reads each by a column mapping in `validation/log-held-mappings.json`. Since no one's
word backs those mappings, every unit a mapping states is held to the flight's physics. A wrong
unit fails a check instead of reading as a number
([ADR-274](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0274-the-rest-of-the-importers-planned-teensy-logger-first.md)
decided it, in its amendment for [M7.1c22](../decisions-and-roadmap.md#m7-1c22)).

**What a check tells:**

- a clock's unit from one a power of ten away;
- a height's, a speed's and an accelerometer's from one a factor of 3.28 (feet for meters) or
  more away;
- a pressure's in pascals from hectopascals;
- and degrees from radians.

A check doesn't tell torr or mmHg stated as hectopascals. Read as hPa, 760 torr is 76 kPa, inside
the pad's band.

Sixteen of the tables are in the private collection. So the mapping file names their columns by
its own labels, such as `pressure 1`, or reads each name from the file's header at each run. The
mapping file gives one name from a private header, a sensor's. That header's last name runs onto
its first row, so the file can't give it.

**What else a mapping can say.** Besides each column's quantity and unit, a
[`Mapping`](../api/hpr_flightdata/generic/struct.Mapping.html) can say how the rows are written,
for files the header reader can't take:

- *A clock that turns over*, `"clock_wrap"`: the count the clock turns over at, in the clock's
  own unit. A 32-bit count of microseconds uses `4294967296` (2³²); it turns over every 71.6
  minutes. Each value is taken modulo the turn, and each step back of more than half a turn adds
  one turn, so the clock runs on. One team's board writes values 9 × 2³² too large in the upper
  half of each turn. Modulo the turn, the four turns in each of its files read as one clock.
- *Rows written as lists*, `"rows": "list"`: `[1000, 101325, True]`, as a program printing its
  lists writes them. A row without its brackets is refused on its line. A word such as `True` is
  text: a column of words is left out, with a note.
- *Stamped rows*, `"rows": "stamped"`: `2024-06-01 14:05:09.125: 1005.71`, or a tuple,
  `2024-06-01 14:05:09.125: (0.1, -0.2, 9.8)`. The stamp is the clock, in seconds from
  January 1, 1970 in the stamp's own zone. The file doesn't state that zone, so only differences of
  times mean anything. As a 64-bit float, such a time resolves to about 0.25 µs. A line with no
  stamp, such as `=== MOTION DETECTED ===`, is an event at the next row's time. A line shaped like
  a stamp with no valid date and time in it, a leap second's `23:59:60` among them, is refused on
  its line.
- *A column left out*, `"skip": true`: cells beside a table that aren't part of it. One case is a
  flyer's notes typed beside an altimeter's table in a workbook. The column still has its entry
  in `columns`, as every column does, and counts in a row's width.

A mapping is JSON. This one reads a payload's stamped text, its second cell a note left out:

```json
{"header_line": null, "separator": ",", "decimal_comma": false, "clock": 0,
 "rows": "stamped",
 "columns": [
   {"name": "stamp", "quantity": null, "unit": "s"},
   {"name": "pressure", "quantity": "pressure", "unit": "hPa"},
   {"name": "note", "quantity": null, "unit": null, "skip": true}]}
```

Stamped rows need the stamp's entry first, as the clock, in `s` and with no turn. The clock can't
be left out. A turn must be a number above 0. A mapping that breaks one of these is refused,
naming the rule. The [API documentation](../api/hpr_flightdata/generic/index.html) runs this
example and a turning clock in rows written as lists.

**The pad, apogee and liftoff,** as the checks read them from the data, not the
[glossary's liftoff](../glossary.md#liftoff), the first motion on the rail. The file's height is its
altitude, or the standard atmosphere's height of its pressure, smoothed by a running median of 5
readings. Apogee is the last of the highest smoothed heights. The pad height is the lowest
smoothed height in the 300 s before apogee. A rocket at Mach 3.5, about 1.2 km/s (3,900 ft/s),
takes 122 s to coast to apogee against gravity alone. The 300 s window is more than twice that.
The window keeps out the hours before launch. Over a whole recording, one board's barometer read
lower heights in those hours than on the pad. That put liftoff hours early (found while the
checks were built). Liftoff is the last reading before apogee whose height above the pad is
within 5% of the climb, apogee less the pad height.

**The checks.** Each row below is one kind of check. A file that fails any of them fails the
survey, listed among the files left unread. The bands are the earlier increments':

- the clock's from [M7.1c18](../decisions-and-roadmap.md#m7-1c18) (a course's Teensy logger);
- the altitude's from [M7.1c21](../decisions-and-roadmap.md#m7-1c21) (altimeters' other text);
- the accelerometer's from [M7.1c20](../decisions-and-roadmap.md#m7-1c20) (FIP sheets).

The overlap, the clock rate and the track's own speed are new checks here. The pad's 50 kPa is
the standard atmosphere's pressure 5.6 km (18,300 ft) up, above any launch site. Here a *band* is
the range of values a check accepts, and a *fix* is one position a GPS receiver reports.

| Reading | Check | Band | A unit it refuses |
|---|---|---|---|
| The clock | the time from liftoff to apogee of the file's height, over √(2h/g) | 0.5 to 2 | a power of ten: 10 times off or more |
| The clock, with no height in the file | its span of times overlapping another file's of the same logger, over the longer span | 0.5 or more | the same |
| The clock, on the pad with no flight | its median step over the same logger's, in a file with a flight | 0.9 to 1.1 | the same |
| A pressure | its reading on the pad | 50 to 110 kPa | hectopascals for pascals, 100 times off |
| An altitude | its rise over the standard atmosphere's height of the file's pressure, another logger's rise, or the apogee the flyer states | 0.6 to 5/3 | feet for meters, 3.28 times off |
| A vertical speed | the altitude's change between two readings over the time between them, against the logged speed, from liftoff to apogee where the speed tops 10 m/s (33 ft/s); their median | 0.6 to 5/3 | ft/s for m/s |
| A GPS track's altitude | how fast the track's own vertical speed falls over the 5 s before its highest fix, in g, from a parabola fitted to those fixes ([below](#a-tracks-own-speed)) | 0.6 to 5/3 | feet for meters, 3.28 times off |
| An accelerometer | its median magnitude over the recording, mostly at rest on the pad or under a parachute | 0.85 to 1.15 g | g for m/s², 9.8 times off; ft/s², 3.28 |
| A latitude and a longitude | each on its own: the median size of its readings other than 0, past ±π/2 for a latitude and ±π for a longitude | | radians stated as degrees, which can't lie past them |

A worked example, with invented numbers: a table with no header logs a pressure in pascals and an
altitude above the pad in feet. On the pad the pressure reads 97,000 Pa, inside 50 to 110 kPa.
Had the file written 970 hPa and the mapping stated Pa, the pad would read 970 Pa, below 50 kPa,
and fail. At apogee the altitude reads 3,281 ft and the pressure 85,951 Pa. The standard
atmosphere puts 97,000 Pa at 366.4 m (1,202 ft) and 85,951 Pa at 1,366.4 m (4,483 ft). That is a
rise of 1,000.0 m (3,281 ft), so the altitude's 3,281 ft over that rise is 1.000. Mapped in
meters, the altitude would rise 3,281 m (10,764 ft), a ratio of 3.28, and fail. The climb takes
15 s, and √(2 × 1,000 m / 9.807 m/s²) is 14.28 s, a ratio of 1.05. A clock in seconds mapped as
milliseconds would give 0.001.

**What is kept as written.** No check here tells °C from °F at a launch's temperatures. That is
because 24 °C read as °F would be −4 °C, a launch's temperature too. So every temperature is kept
as written. So is every rate of turn. So is every other column no check holds, such as a battery's
voltage, humidity, a magnetometer and an orientation's quaternion. One board's accelerometer reads
0 at rest, with gravity taken out, and is kept too, as are words, packed fields and counts.

**What the 20 show.** Their mappings read 64 columns in a unit, keep 102 as written and leave 7
out, and all 62 checks pass. The 12 times to apogee lie 0.8627 to 1.7105 times √(2h/g). The 6
altitudes held to their pressure rise 0.9559 to 1.0005 times the rise of its standard-atmosphere
height. The 3 held to another logger read 0.9008 to 0.9552 times that logger's rise. All three
are a GPS receiver's altitude, two of them the same fixes with and without a header. The 1 held
to the flyer's stated apogee reads 1.0025 times that apogee. The 1 vertical speed gives 0.9709,
and the 2 checks of a track's own speed 0.9925 g, one track in two files. The 10 accelerometer
checks read 0.9600 to 1.0531 g. The other 27 are:

- 13 pad pressures and 6 places, whose values would give a site away;
- 6 clocks beside another file's, overlapping 0.998 to 1.000 of the longer span;
- 1 clock rate of 1.000 times its logger's;
- and 1 cut of a file, its rows all the whole file's.

The survey's report,
[`validation/reports/log-import.md`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/log-import.md),
lists each file's checks, each to 4 decimals.

#### A track's own speed

A GPS receiver gives an altitude and a time but no speed this check could trust, and the check
needs none: gravity is its ruler. In the last seconds before apogee a coasting rocket slows at
g, 9.807 m/s² (32.2 ft/s²), plus a little drag. So a track's altitudes there, fitted with a
parabola, must slow at about 1 g. In a wrong unit they slow 3.28 times too fast (feet stated as
meters) or too slowly (meters stated as feet). The check holds a unit within a wide band. It
doesn't measure drag, which a receiver's noise can hide.
([ADR-277, a track's own speed](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0277-a-tracks-own-speed-held-by-its-vertical-speed.md)
decided it.)

- *The fit.* The check takes the track's highest fix as apogee. It fits h = a + bτ + cτ² to the
  fixes in the 5 s before apogee, by least squares. The highest fix is included. τ is the time from
  the fixes' mean. Then −2c is how fast the vertical speed falls, and the check divides it by g. An
  altitude repeated over consecutive rows, as a logger writes rows faster than its receiver
  updates, counts once, at its first row. So do two fixes at one altitude, which near apogee moves
  the top a step early. Of several equal highest altitudes that aren't consecutive, the last is the
  highest.
- *Enough fixes, spread enough.* The check needs at least 4 fixes, one more than the parabola's
  three terms. They must set its curvature c about as well as 5 fixes 1 s apart. The measure is
  [(XᵀX)⁻¹]_cc, the variance of c for each square meter of independent noise on a fix. It must be
  at most 1/12 s⁻⁴. A set of 5 fixes 1 s apart gives 1/14, and 1/12 lets them come 3.8% closer
  together. The reading's standard error is then at most 0.0589 g per meter of noise. With 3 m (9.8
  ft) of noise on each fix, a right unit reading 1.03 g lies 2.4 standard errors above the band's
  foot. The same fixes in feet stated as meters read 3.0 standard errors past its top. Four fixes
  over the last 3 s, as a receiver that lost its fix on the way up leaves, give 1/4 s⁻⁴. There
  noise would decide, so such a track doesn't take the check.
- *Why 5 s, and the band.* Drag's vertical share is ½ρC_DA/m · v · v_z. Here ρ is the air's
  density, C_D the drag coefficient and A the frontal area. The mass is m, the speed v and its
  vertical part v_z. That share falls to 0 at apogee as v_z does. Take a 54 mm (2.1 in) rocket of
  1.5 kg (3.3 lb), C_D 0.5, in air of 1.1 kg/m³. Coasting straight up, it climbs at 50.8 m/s
  (167 ft/s) 5 s before apogee, a little more than g × 5 s. Its drag adds 0.11 g there. Carrying
  100 m/s (328 ft/s) across at apogee, its drag adds 0.33 g. Fitted to its fixes once a second,
  the parabola reads 1.031 g straight up and 1.135 g carrying that speed across. The test
  `a_coast_with_drag_reads_inside_the_band` pins both. Leaving the band's top takes 1.667 g. A
  longer window would let drag grow; a shorter one holds too few fixes once a second. Drag only
  adds. So a right unit reads low only by noise or a receiver's lagging filter, which fail the
  check rather than pass a wrong unit.
- *What it assumes.* A coast of at least 5 s, which a rocket has if it climbs 123 m (402 ft) or
  more after burnout, ½g × (5 s)². A motor still burning in the window would make the fit read
  anything. The club's track climbs far past that.
- *It doesn't hold the clock.* The window is read on the file's own clock, so a misread clock
  changes which fixes it takes. Read 10 times too short, it would take 50 s of fixes, the pad
  and the boost among them. Then the fit could land anywhere. The clock checks hold every held
  file's clock, so the window is the right one.
- *A worked example*, with invented numbers. A receiver reports once a second. Its fixes 5, 4, 3, 2
  and 1 s before its highest lie ½gτ² below it. That is from 122.58 m (402.2 ft) at 5 s to 4.90 m
  (16.1 ft) at 1 s. The least-squares parabola through the six fixes, the highest and the five
  before it, has c = −4.903 m/s², so −2c is 9.807 m/s², 1.000 g. Written in feet and mapped in
  meters, the same fixes read 3.281 g; written in meters and mapped in feet, 0.305 g. Both fail.
- *Where it runs.* The check runs on the club's GPS track (2 files, the same fixes with and
  without a header), which slows at 0.9925 g. That is a little under the 1 g a coast can't go
  below, within the fit's noise. Another logger's rise holds the same altitude too. A track that
  the latitude-and-longitude check holds, and whose GPS altitude could take this check, must take
  it, or the survey fails. Of the other 3 of the 4 tracks, the radio telemetry's receiver lost
  its fix on the way up. That left 1 fix in the window, the highest itself. So another logger's
  rise alone holds its GPS altitude. The other 2 have no altitude.

#### A track's ground speed holds no unit

The survey also measures each track's ground speed from each fix to the next. That is the
distance along the Earth's surface between them (the great-circle distance) over the step of the
track's own clock. A fix the logger repeats over several rows counts once.

- *What was measured.* The 4 tracks held by a latitude-and-longitude check are 5 files, two of
  them the same fixes. Over them, each track's median lies 0.3460 to 7.3836 m/s (1.1352 to
  24.2244 ft/s), a spread of 21.34 times. Each track's fastest step lies 24.8509 to 400.0430 m/s
  (81.5318 to 1,312.4770 ft/s), a spread of 16.10 times. The medians span the whole recording,
  the pad's hours included, so a median can be a receiver's jitter as much as the wind's drift.
- *Why no band holds a unit.* A clock misread by a factor of 10 (the checks table's power of ten)
  moves every speed 10 times, less than either spread. So no band on these tracks refuses that
  misread on every one of them. Taking the flight alone wouldn't help. A track drifts with the
  wind, and a launch may go on a calm day. So its speed has no floor (NFPA 1127 limits only the
  surface wind, to 20 mph (8.9 m/s)). Radians stated as degrees would read 57 times slower, which
  a band could tell apart on these 4 tracks. But a band fitted to 4 tracks is a sample, not a
  bound. So a track's degrees are held by the latitude-and-longitude row of the table, which
  needs no band.

### Binary records by a stated layout

Some teams' flight computers save each sample as a fixed run of bytes. One example is a 4-byte
clock in milliseconds, then 24 four-byte floats. Nothing in the file says so: no header, no names,
no marks. Such a file can't be recognized, only read by a layout someone states. The library reads
it by the layout you give, each field's name and type in the record's order
([`records::RecordLayout`](../api/hpr_flightdata/records/struct.RecordLayout.html)). The library
never guesses a layout. The command line and Python take the layout as a JSON file
([Reading your own logger](../your-own-logger.md#binary-records-by-a-record-layout)).

A type is an integer of 8, 16, 32 or 64 bits, signed or not, or a 32-bit or 64-bit IEEE 754 float.
Binary32 and binary64 are the standard's names for single and double precision. A type is
little-endian unless the layout's `byte_order` says
[`ByteOrder::Big`](../api/hpr_flightdata/records/enum.ByteOrder.html). Little-endian means the least
significant byte first, as most microcontrollers store numbers.

A layout has 2 to 512 fields: a clock and at least one more. Each name is 1 to 256 characters. No
two names are alike. No name is a number. No name holds a comma, a semicolon, `|`, a quotation mark
or a control character (a line break or a tab among them). No name has spaces at its ends. The
first name doesn't open with `#`, `sep=` or a byte order mark. A layout outside these rules is
refused (`LogError::Layout`), naming the field and why.

The records are written out as a table: the names are its header, and each record is one row. The
[header reader](#tables-read-by-their-headers) then reads that table. Name a field with its unit,
such as `pressure__kpa` or `Time (ms)`, and it is read in that unit if its values bear it out. A
field whose name states none is kept as written. A layout whose names state no clock with a unit of
time is refused. For such a layout, `records::table` gives the table for reading it by a
[column mapping](#mappings-a-publisher-states) instead
([ADR-271](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0271-binary-records-by-a-stated-layout-before-the-rest.md),
which decided it).

A 32-bit float is widened to 64 bits exactly. It is written as the shortest decimal that reads back
to it. So a pressure stored as the float nearest 101.3 kPa is read as 101.30000305175781 kPa, the
value the bytes hold, not as 101.3. A float that isn't a number or is infinite becomes a gap, and
a note counts them. That note is one of the record's [notes](#the-record) on what the reader worked
around.

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

A logger saving to flash leaves the memory it never wrote erased, every byte `0xFF`. A dump of its
memory may run on into it. Records that are wholly `0xFF` at the end of the file are dropped as
that erased memory. So are erased bytes past the last whole record, and a note counts both. Any
other bytes past the last whole record are refused, as a sign the layout doesn't fit. An erased
record with written ones after it isn't dropped, so a logger's fault stays visible. Its integers
read at their largest (−1 if signed), and its floats read as gaps. With a float clock, that record
has no time. The reading is refused, naming the record by its number and where it starts. With an
integer clock it is read. A `u32` clock in milliseconds reads 4,294,967.295 s at that record, and
its floats are gaps. The records after it read at their own times, so the jump shows where the
logger skipped.

The same records may come as a **console capture**. That is what a terminal program saves when a
flight computer's USB shell is asked to dump its memory. It holds the commands typed at the shell,
and then each dump as base64 lines (bytes written as text, 3 bytes to 4 characters). Each dump sits
between two marker lines the firmware prints,
`----++++//[[( data start )]]\\++++----` and `----++++//[[( data end )]]\\++++----`. Here `data` is
the block's name (the sample captures' blocks are `data` and `params`).
`import::read` recognizes a capture by a start marker in its first 1,000 lines. It refuses the
capture, naming its first blocks, as the capture doesn't state its records' layout.
[`capture::read`](../api/hpr_flightdata/capture/fn.read.html) reads a named block by the layout
you give, as `records::read` reads a file. The base64 is decoded strictly (RFC 4648's standard
alphabet). The reader refuses each of these, naming the line:

- a character outside that alphabet;
- padding out of place or over bits that aren't zero;
- a block that never ends;
- a marker inside a block;
- two blocks of one name.

> **How far to trust it.** The reading is exact for the bytes and types you state. Whether they
> are the logger's is your statement, so rely on the logger's own code or decoded table for the
> layout. The sample logs hold 5 such files, 3 console captures and 2 raw files, each beside the
> table its team decoded from the same records. All 931,875 fields of the 45,915 records written
> agree with that table. The match shows the layout decodes every field exactly as the team did.
> It doesn't show the layout is the logger's own. The types aren't the teams' own statement: no
> team documents them.

**How the survey checked it.** The survey reads each file by the types
`validation/log-layouts.json` states for it, with its names from that table. Each type was found
from the bytes and from that same table. Then the survey
decodes every record's fields itself from the file's bytes, apart from the reader's records. It
compares them with the table field by field. A wrong base64 decoding would fail that comparison
too. Each record read holds the same
channels as the team's table read by its header. The files are private, so the layouts file holds only their types, by file id. One
capture's last 12,222 records are erased memory, which the team's table writes as rows of
`4294967295` and `nan` and the reader drops. The fields of those records agree with the table
too, 1,237,425 fields in all.

**What is left out.** No [primary track](#the-record), as for any table read by its header.
`hpr analyze` and the Python package read these files only by the layout you state
([Reading your own logger](../your-own-logger.md)). A record's layout can't be found from its
bytes alone. So a file whose layout no one states stays unread. A capture's other blocks, such as
the saved state a `params` block holds, are decoded but not read.

### The Blue Raven's USB download

A Blue Raven can also send its flight data out of its USB port as text, one line a sample, which
a terminal program saves. HPR Sim keeps only the lines whose own check value shows them whole. It
reads their fields by the maker's manual wherever a line holds what the manual lists.

The manual describes the download (*Blue Raven User's Guide*, Featherweight, updated May 12, 2026,
Appendix A, p22–24). Typed at the logger, `<download l>` sends the low-rate set, recorded 50 times
a second. `<download h>` sends the high-rate set, 500 times a second. Each line opens with its
mnemonic, `@ LOG_LOW` or `@ LOG_HIR`, then its length, the logger's date and time of day, a sync
code and the data. It closes with `CRC:` and four hexadecimal digits. The sync code is "a
millisecond counter recorded in both the low-rate and high-rate data so that the two data sets can
be aligned" (p23). It rolls over every 250 ms (p23). The CRC, a cyclic redundancy check, is a check
value worked out from the line's text. So a line that lost or changed a character no longer
matches it. The manual names the CRC for the low-rate line (p23). The sample files' high-rate lines
carry one of the same kind. `import::read` recognizes a download by a line among its first 1,000.
That line opens with either mnemonic, holds no other `@ LOG_`, and has a CRC that checks.
`import::read` reads the download with `hpr_flightdata::blueraven_usb`
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

> **How far to trust it.** The layout and the units are the maker's, from its manual. Every
> value in the 2 sample files equals the survey's own decoding of them. Three physical checks hold
> parts of the reading to the files ([below](#how-the-usb-download-was-checked)). The other
> fields rest on the manual's scales alone. In some places the sample files' firmware writes more or
> fewer values than the manual lists; those are kept as written. The 2
> sample files record no flight. Neither their pressure nor their acceleration shows a launch,
> though the TeleMega in the same rocket records one. The files don't say why. So they check how
> the download is read, not how a Blue Raven's channels follow a flight.

**Each line's two checks.** A terminal saves what the port sends, and a capture can lose part of
it. In one sample file, a record whose `CRC:` and line end were lost runs on into the next record,
on one line. So the reader cuts every line before each `@ LOG_`. It keeps a piece only if two
things the logger wrote for it agree with it:

- *Its CRC*: CRC-16/BUYPASS, as the manual names it. The polynomial is `0x8005`. The CRC starts
  from 0, with no reflection and no final XOR. Its value for the text `123456789` is `0xFEE8`. The
  CRC covers the text from `@` to the last character before `CRC:`, with the spaces before `CRC:`
  left out. The manual says the CRC is "for line of text" but not which characters it starts and
  ends at. This is the span under which every CRC in the sample files checks.
- *Its length*: the number after the mnemonic is the piece's length in bytes. The count runs from `@`
  through the CRC's last digit, plus 2 for the CR and LF the logger sends. The high-rate line above
  is 137 characters long and states `139`.

A piece with no CRC, or whose CRC differs from its text, is dropped. That is because it can't be told from a
record the capture damaged. A note counts each kind. A line with no record is skipped and counted
too, such as the status line, `@ BLR_STAT`. The port sends that line five times a second (p22). A
piece whose CRC checks is the logger's own. So anything else wrong with it refuses the whole file,
naming its line:

- a length that isn't its own;
- another mnemonic;
- a date or time that can't be one;
- a value that isn't a whole number (or, in `FER:`, hexadecimal digits);
- more than 256 values;
- a layout unlike the first record's.

So a file holding both kinds of record is refused, and so is one in which no record's CRC checks.

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
from the ±32 g accelerometers below 32 g and from the high-range one above it. The quaternion is
four numbers the logger keeps its attitude in. It is a unit quaternion, whose four terms' squares
sum to 1. The manual says the download writes it "with the rotation axis first and the magnitude
term fourth" (p24). The app's high-rate CSV doesn't. In each such file among the sample logs whose
first row holds numbers, `Quat_1` is the largest term there, about 1 on the pad. So the CSV writes
the magnitude term first (checked by hand on October 15, 2026). Both keep the quaternion's terms as
numbers, so neither order changes a value read. A flight event register is a set of bits, "one bit
for each event" (p24), such as apogee detected. The rocket's register records the events that
affect every output. Each output's register records its own.

**Counts unlike the manual's.** A logger's firmware need not write what the manual of the day
lists. The sample files' firmware writes other counts. A high-rate record holds 12 values where
the manual lists 10. `V:` holds 5 where it lists 6. `FER:` holds 8 where it lists 5. One rule
takes all three:

- A group holding at least the manual's count is read by the manual, in its order. The values past
  that count are kept as written. These are the high-rate record's 11th and 12th values, and
  `FER:`'s 6th to 8th. The `FER:` ones are registers of unknown meaning, kept as numbers.
- A group holding fewer than the manual's count is kept as written whole, as nothing says which
  value is missing. So `V:`'s 5 values aren't read as voltages.
- A group the manual doesn't list is kept as written.

A value kept as written is named by its group and its place there, counted from 1, such as `V: 3`
or `LOG_HIR 11`. A note says so for each group. The maker's forum describes two 500 Hz fields the
manual doesn't. They are the "deployment output current" and the "aux analog voltage" (Featherweight,
*Blue Raven development thread*, rocketryforum.com, March 31, 2023). The forum doesn't give their
order. So the high-rate record's 11th and 12th values stay as written.

**The clock.** Each record states the logger's date and time of day to the millisecond, such as
`2026  5 23 10:15:00.120`. The record's clock counts from the first record's, in seconds. A time
that doesn't come after the one before refuses the file, naming its line. The date need not be the
flight's. The manual says the date stamp "starts with the build time/date and increments from
there" (p22). That is the firmware's build date, unless the phone app has set it (p22). So a note gives
the first record's date and time as written, and the record claims no date for the flight. `Year`,
`Month`, `Day` and `Sync` are kept as numbers and `Time` as a time of day, as the app's CSV keeps
them. The sync code is "a millisecond counter recorded in both the low-rate and high-rate data so
that the two data sets can be aligned" (p23). It also "rolls over every 250 msec" (p23). The reader
keeps the code but doesn't join the two sets.

**What is kept as written, and why.**

- *The baro sensor's temperature*, `Bo: temperature`: the manual states no unit for it in the
  download, and a note says so.
- *The registers' bits*: the manual says the registers are "hex-encoded binary values that use one
  bit for each event" (p24). But it doesn't say which bit is which event, so no event is taken
  from them.
- *No [primary track](#the-record)*: the download writes the pressure, not an altitude. So the
  record's time and altitude are empty, as for the [Raven's `.FIPa`](#the-ravens-fipa). Every
  value is a channel.

**A worked example**, the two invented records above. In the high-rate record, `20` is the sync
code and `-15` the first gyro rate, so `Gyro X` is −0.15 degrees per second. The sixth value, `98`,
makes `Accel Z` 0.98 g, 9.611 m/s². The quaternion's four terms, `-3000`, `1500`, `-40` and
`29812`, read as −0.1, 0.05, −0.001 33 and 0.993 73. The four whole numbers' norm is 30,000.12,
within 2 of 30,000. A unit quaternion's norm must be within 2 of 30,000 too, after the scaling cuts
it to whole numbers. The last two values, `0` and `0`, are `LOG_HIR 11` and `LOG_HIR 12`, kept as
written. In the low-rate record, `Bo:  7212 49655` gives `Bo: temperature` 7,212, as written, and
`Bo: pressure` 49,655 / 50,000 atm, 100,626 Pa. `V:` holds 5 values, so they are `V: 1` to
`V: 5`, kept as written. `ang:   12    5    0` is a tilt of 1.2°, a roll of 5° and a future tilt
of 0°. `FER:  400  1c1` makes `FER: rocket` 1,024 and `FER: Apo` 449. The last three registers are
`FER: 6`, `FER: 7` and `FER: 8`.

<a id="how-the-usb-download-was-checked"></a>**How it was checked.** The sample logs hold 2
downloads, a team's high-rate and low-rate sets from one rocket's logs. The survey,
[`cargo xtask logs`](#how-it-was-checked), works out each piece's CRC and length itself, apart from
the reader. It decodes each record that checks by its own copy of the manual's table and rule. The
reader must keep exactly the records that check. Each of their values, the clock's included, must
equal the survey's:

| File | Pieces | CRC checks | Records read | Values compared | Values differing |
|---|---:|---:|---:|---:|---:|
| high-rate | 17,930 | 17,930 | 17,930 | 304,810 | 0 |
| low-rate | 1,861 | 1,706 | 1,706 | 49,474 | 0 |

Every record whose CRC checks states its own length too. The low-rate file's other 155 pieces
close with no CRC, records the capture ran together, and are dropped. Three physical checks hold
parts of the reading, and of the rule for counts unlike the manual's, to the files. A file that
failed one would be counted as unread. The three checks are:

- Each high-rate quaternion is read from the 7th to the 10th value. Its norm is from 29,998.25 to
  30,000.00, within 2 of 30,000 as it must be.
- The low-rate pressure on the pad is the median of the first 20 records. That median is 0.9977 of
  the pressure a TeleMega (an Altus Metrum board) in the same rocket logged. The check allows 2%
  either way.
- The first register's lowest bit is the one the app's files call `Liftoff`. It first sets 0.024 s
  after the three axes' acceleration in the high-rate file first passes 3 g. The check allows
  0.1 s. This holds the two files' clocks and the register's first value to each other. It
  doesn't show a launch, as the next paragraph says.

The download records no flight. The high-rate file's acceleration stays above 3 g for at most
0.056 s at a time. Its greatest mean over any 1 s is 1.68 g. So the bit was set by a short spike,
not a boost. Over the whole low-rate file, the Blue Raven's pressure moves by only 0.16% of its
pressure on the pad. By contrast, the TeleMega's falls to 0.773 of its own. So none of the
download's channels, the gyroscopes' and accelerometers' included, describes the TeleMega's
flight. Nothing in the files says why, so the reading keeps the values as written. The
[survey's report](#how-it-was-checked) prints these measures.

**What is left out.** The record makes no altitude from the pressure, though the
[readings](../physics/log-readings.md#the-height-track) turn it into a height by the 1976
standard atmosphere. The two sets aren't joined by their sync code.

### A course's Teensy data logger

Harvey Mudd College's advanced rocketry course flies a data logger the course builds around a
Teensy, a small microcontroller board. The course publishes its flights' files on its
[*Flight Data* page](https://pages.hmc.edu/spjut/AdvRoc/FlightData.md.html), which lists two
kinds, "Teensy Data Logger" and "New Teensy Data Logger". Both write one sensor's reading a line.
A line holds three things:

- the sensor's tag, a short label such as `A` for the accelerometer or `Alt` for the barometer;
- the logger's clock in microseconds since it started;
- the values.

No document describes either layout. So HPR Sim reads what each file states. Where a file states
no unit, HPR Sim reads a unit only if the flight's own physics confirms it.
[ADR-274, the Teensy logger and the rest of the importers](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0274-the-rest-of-the-importers-planned-teensy-logger-first.md),
decided this. `import::read` reads both with `hpr_flightdata::teensy`.

> **How far to trust it.** The layouts are taken from the files, as no document describes them.
> In the 8 sample files, every value equals the survey's own decoding. Each unit read without
> the file stating it passes a physical check on every file ([below](#how-the-teensy-files-were-checked)).
> The gyroscope, the magnetometer, the high-g accelerometer of the older layout and the analog
> inputs are kept as written, because no check settles their units. Those values aren't
> converted, so they can't be wrong in SI; they simply aren't in SI. The readings, such as the
> apogee, come from the barometer's pressure, turned into a height by the 1976 standard atmosphere
> ([Flight-log readings](../physics/log-readings.md#the-height-track)).

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

A value is named by its tag and its place, `A 1` to `A 3`. Or it is named by its tag and the name
the line gives it, `Alt P` and `Alt MSL`. No line states a unit, so three of the values are read
in SI only because physical checks confirm them:

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

`HG` moves in steps of 0.4788 if read as m/s², but it doesn't agree with `A`. The comparison is
made during the boost, while `A` reads from 3 g to 90% of its greatest. `HG` reads 1.37 to 2.39
times `A`'s magnitude then, depending on the file. So its scale isn't settled.

**The newer layout** opens with a line for each sensor that states its units, the clock's first.
Each line after them gives a tag, `t`, the clock, and name-value pairs, separated by commas:

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
- `xm` to `zm` are stated in milli-T and kept as written. HPR Sim holds no magnetic field. The
  stated unit also looks wrong, as Earth's field, 0.025 to 0.065 milli-T, is about a thousandth of
  the values the sample file holds. Those values fit microtesla.
- `T` is in degrees Celsius, read as kelvins.

A value is named by its tag and its name, such as `ICM za`. A tag with no units line, such as the
sample file's `VOL`, is kept as written. So is a tag whose units don't fit its groups. A note says
which.

**The clock.** Each tag is its own set of channels, on its own clock, as each sensor is read at its
own rate. In the sample files, each of the older layout's tags has about 43 lines a second. The
newer layout's main accelerometer has about 1,000. Times are seconds from the logger's start.
The file is refused at any of these:

- a line that isn't a tag, a clock and values;
- a tag's line laid out unlike its first;
- a clock that doesn't increase from the tag's line before.

The count of microseconds runs over after 2³² µs, 71.6 minutes, so a longer recording is refused
there. A file's last line, if it ends with no line end and can't be read, is a line the logger was
writing when it stopped. That line is dropped, with a note.

<a id="how-the-teensy-files-were-checked"></a>**How it was checked.** The sample logs hold 7 files
in the older layout and 1 in the newer, a 100 s cut from a longer recording. The survey,
[`cargo xtask logs`](#how-it-was-checked), decodes each file itself and compares each of the
6,188,639 values with the reader's. None differs. Four physical checks hold the units read. A file
that failed one would be counted as unread. The four checks are:

- **`Alt MSL` against the BMP180's formula,** worked out from `Alt P` in 64-bit floats. On all
  305,340 `Alt` lines it differs by at most 0.0054 m (0.21 in). The check allows 0.0103 m
  (0.41 in). That is half the hundredth of a meter `MSL` is written to, plus an allowance.
  The allowance is 44,330 × 2⁻²³ m, 5.3 mm (0.21 in), for the logger's own rounding of the formula. On
  21,540 lines the gap is more than the half step alone. It is greatest at the same pressure in
  every file. So the logger rounds the formula its own way, which no document gives.
- **The accelerometer at rest.** Its magnitude on the pad, the median over the 5 s ending 1 s
  before liftoff, is 0.983 to 0.989 g. Over the file's last 2 s, after landing, it is 0.997 to
  1.008 g.
  The check allows 3% either way. Read in another unit, such as g or ft/s², it would be 3 or more
  times off. Over the first 2 s of the older layout's files, minutes before launch while the
  logger was handled, three files read 1.034 to 1.048 g. So the pad is taken just before liftoff.
- **The time to apogee.** From liftoff to apogee, the time is 0.90 to 1.14 times √(2h/g). Liftoff
  is the accelerometer's first reading above 3 g in the 30 s before apogee. Apogee is the highest
  pressure altitude. In the formula, h is the apogee's height above the pad. The formula gives the
  time a coast from the pad would take with no drag. This is a coarse check, as the burn adds time
  and drag takes some away. So the check allows a factor of 2. A clock misread by a power of ten
  would still be 10 or more times off.
- **The pad pressure** of the newer layout, the median on the pad, is between 50 and 110 kPa, as
  any launch site's is. The report says so without the value, which would give the site's
  elevation.

Each check also needs the reader to have read the values it holds as the quantities it takes them
in. So a unit the reader got wrong fails the check.

The newer layout's other stated units rest on the file's statement. Its high-g accelerometer, read
in its stated g, is 0.996 times its main one during the boost. That agrees with the unit stated.
The report prints this as a measure, not a check.

**What is left out.** There is no [primary track](#the-record), as neither layout writes an
altitude above the pad; the readings take the pressure instead.

### One team's radio telemetry packets

A university team's flight computer writes each reading as a *packet*, a line framed by `255` and
`254`. It sends the packets down by radio. For one flight the team published four files of them. Two
of them are the flight computer's own log and the text a ground receiver wrote of what it heard. The
other two are exports of the log, cut to the flight. Each export was saved as an RTF document (a
word processor's format) and then as CSV by a spreadsheet. No document describes the packets, so HPR
Sim reads what the files show. It holds the reading to the flight computer's own log
([ADR-274, the Teensy logger and the rest of the importers](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0274-the-rest-of-the-importers-planned-teensy-logger-first.md),
which decided it). `import::read` reads all three forms with `hpr_flightdata::packets`.

> **How far to trust it.** The layout is taken from the files. The receiver's text and the exports
> hold packets that the onboard log also holds, found by their stage, kind and clock. Every such
> packet matches the log's field for field ([below](#how-the-radio-packets-were-checked)). The
> receiver's GPS packets are the only ones the log doesn't hold. That shows the copies are decoded
> alike, not that a value is right. The units rest on two physical checks below: a
> pressure band and a time to apogee within a factor of 2. The clock is read in milliseconds and one
> value in hectopascals. Each is held to the flight's physics. The other values are kept as written,
> because nothing settles their units.

The readings, such as the apogee, come from the pressure. It is turned into a height by the 1976
standard atmosphere ([Flight-log readings](../physics/log-readings.md#the-height-track)).

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

The path, the stages and the GPS packets are read from the files, not from a document. No document
confirms what they mean. What `AA` and `CC` measure isn't known.

Each kind becomes a set of channels on its own clock. They are named by the kind and the value's
place, such as `PP 1`. The stage is a channel of its own, `PP stage`. The GPS packets (stage `88`)
carry the relaying transmitter's own clock, not the flight computer's, ahead of it by an offset
nothing states. So their times are left empty (gaps, with no place on the record's time axis). Their
clock is kept as written in a channel of its own, `GG clock`. A `Stage Two` line, as the onboard log
writes when the stage changes, becomes an event at the next packet's time. The word must be a
number's, `One` to `Nine`. A mark the radio garbled is a broken line. One heard again is dropped.

| Values | Read as | Held to |
|---|---|---|
| the clock | seconds, from milliseconds | the time from liftoff to apogee, against √(2h/g) ([below](#how-the-radio-packets-were-checked)) |
| `PP 1` | a pressure, Pa, from hectopascals | the pad's pressure, between 500 and 1,100 hPa, and the time to apogee, which takes the height from it |
| the rest | kept as written | nothing settles a unit |

`PP 3` climbs with the flight. But where it reads above 200, it is 1.11 to 1.24 times the height the
standard atmosphere gives the pressure, in meters. So neither meters nor feet fits it, and it is
kept as written.

**Lines that aren't packets.** The radio loses bytes, so the receiver's text holds many *broken*
lines. A broken line is a packet cut short, or run into the next. A broken line is dropped, and a
note counts them. A packet heard again is dropped too: it has the same stage, kind, clock
and values as one already read, by either path. The rule is there because the receiver heard many
packets twice. The onboard log opens with a whole number and its filter's gains, `Gain[0]: …`, which
a note counts.

**The exports** are RTF documents whose lines a spreadsheet split at the colons and saved as CSV. So
each line has commas after it. For the same reason, the document's markup sits in the first cells,
and a `\` ends a line where the document broke it. The reader takes each line on its own. It drops the commas, the markup
before a packet, a `\` at the end and the `}` that closes the document. A line of markup alone is
dropped.

A file is refused if it has any of these:

- no whole packet;
- more than 32 kinds;
- a packet with more than 32 values;
- a clock of 2³² ms, 49.7 days, or more.

<a id="how-the-radio-packets-were-checked"></a>**How it was checked.** The sample logs hold the four
files of one flight. The survey, [`cargo xtask logs`](#how-it-was-checked), decodes each file itself
and compares every value with the reader's. None differs. It then looks up each packet of the
receiver's text and of the exports in the onboard log, by its stage, kind and clock:

| File | Form | Whole packets | Packets held by the onboard log | Packets differing | Not held: GPS | Not held: other packets | Repeated packets | Broken lines |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| the receiver's text | received | 42,796 | 38,475 | 0 | 4,321 | 0 | 8,368 | 10,495 |
| the export of all three kinds | RTF | 4,939 | 4,939 | 0 | 0 | 0 | 2 | 0 |
| the export of mostly pressures | RTF | 2,469 | 2,469 | 0 | 0 | 0 | 1 | 1 |

Of the receiver's 38,475 packets the onboard log holds, 29,914 were relayed and 8,561 direct.

The onboard log itself holds 264,462 whole packets and no broken line. Of these, 2 are repeated.
Counts include repeats. *Repeated* counts the packets a reading drops as heard before, by either
path. The survey compares 1,315,788 values with the reader's in all four files, and none differs.
The GPS packets are the second transmitter's fixes. The onboard log doesn't record them.

Two physical checks hold the units read, on the onboard log:

- **The pad pressure** is the median of `PP 1` over the 5 s ending 1 s before liftoff. It is between
  500 and 1,100 hPa, as any launch site's is. The report says so without the value, which would give
  the site's elevation.
- **The time to apogee.** Liftoff is the first packet, after the pad's stage `00`, whose pressure is
  more than 1 hPa below the median. The median is of the 5 s ending 1 s before that packet. Apogee
  is the lowest pressure. From liftoff to apogee, the time is 1.055 times √(2h/g), where h is the
  height the standard atmosphere gives the two pressures. The check allows a factor of 2. A clock
  misread by a power of ten would be 10 or more times off.

**What is left out.** There is no [primary track](#the-record), as no packet writes an altitude
above the pad in a unit that is settled. The GPS packets' values are kept as written. `hpr analyze`
and the Python package read these files as the library does. Each reading is given within its bound
or withheld for the reason the command prints ([Flight-log readings](../physics/log-readings.md)).

### A FIP export pasted below no header

Two sample workbooks hold a Featherweight Raven's FIP export. That is the CSV the Featherweight
Interface Program (FIP) writes. It is pasted into a sheet with its header line missing. Beside each
export is the [`.FIPa`](#the-ravens-fipa) of the same flight, the Raven's raw download. Each row is
the FIP CSV's. For each of 16 channels it has a clock, a value and a `b` status cell (the `bILBA`
column, [kept as a number](#units-no-source-states)). Where a channel stops, its two cells are empty
and every channel after it sits one cell to the left. Without the header nothing names the columns,
so the caller states it. `hpr_flightdata::raven::read_rows` reads the rows under the header line it
is given. `read_rows_around` reads a sheet another table was pasted over. The decision record,
[ADR-274, the Teensy logger and the rest of the importers](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0274-the-rest-of-the-importers-planned-teensy-logger-first.md),
decided both readers. It did so in its amendment for [M7.1c20, workbooks with a reading
beside](../decisions-and-roadmap.md#m7-1c20). `import::read` doesn't read such a workbook on its
own. `custom::read_by_fip_header` reads such a workbook under a header stated in a JSON file. The
file holds the header line and the pasted columns. The function reads from a workbook's sheet or
from the rows as text. `hpr analyze --fip-header FILE` and Python's
`FlightLog.read(path, fip_header=...)` take that file ([Reading your own
logger](../your-own-logger.md#a-fip-export-below-no-header)).

> **How far to trust it.** The header stated for both sample sheets is the one Debrief's public
> sample of a Raven flight carries. It has the same 16 channels in the same order. Both sheets' rows
> hold 48 cells in the header's pattern. The `.FIPa` beside each sheet is the raw file of the same
> flight. Every channel it holds matches the sheet's sample by sample
> ([below](#how-the-fip-sheets-were-checked)). The five channels the FIP works out from the others,
> the bracketed ones, aren't in the `.FIPa`. Two altitudes and two velocities are held to other
> channels, and `[Altitude (Accel-Ft)]` isn't held to anything.

**The bracketed channels.** The two barometric altitudes are held to the sheet's
own pressure and to each other. The two velocities are held to each other where a sheet keeps
them. These checks back the names the stated header gives them.

**The stated header** is in `validation/log-headers.json`, by the file's id:

```text
Time@Axial Accel (Gs),Axial Accel (Gs),bILBA,Time@Baro (Atm),Baro (Atm),bILBA,…,
Time@[Velocity (Accel-MPH)],[Velocity (Accel-MPH)],bILBA
```

The rows are read as the FIP CSV's are, but for one thing a sheet does. It writes every row as wide
as its widest, so a row may end in empty cells. A first row that reads as a header, its first cell
beginning with `Time`, is refused. The file has its own header, and `raven::read` reads it whole.

**A table pasted over.** In one sample sheet, columns 34 and 35 hold a clock and a value pasted over
the export's cells. The header puts `[Altitude (Accel-Ft)]` in those columns. As the export's rows
shift left where channels stop, the two columns cover different channels' cells in different rows.
Given the pasted columns, a channel whose sample, in any row, has a cell in them is left out whole.
A note names it and counts the rows. Every other channel is read. To walk a row, each channel must
still show whether it has a sample there. One sign is a cell of its time or value outside the pasted
columns that isn't empty. Failing that, the sign is the cell after the pasted columns. That cell
must be a sample's status cell, `b` and digits, or, for a stopped channel, the next channel's time.
A row is refused if its pasted cells hide that: the cell after the pasted columns is pasted over or
empty too.

In that sheet the rule leaves out `[Altitude (Accel-Ft)]` (covered in 1,259 rows),
`[Velocity (Accel-Ft/Sec)]` (11,331) and `[Velocity (Accel-MPH)]` (23,921). It reads the other 13
channels. The 11 the `.FIPa` also holds always sit in a row's first 33 cells, so no pasted cell
touches them.

<a id="how-the-fip-sheets-were-checked"></a>**How it was checked.** The survey,
[`cargo xtask logs`](#how-it-was-checked), compares each sheet's reading with the `.FIPa` beside it.
It compares the FIP's own exports with theirs the same way ([checked against the makers'
exports](#checked-against-the-makers-exports)). Each sample is compared at the `.FIPa`'s time as the
FIP prints it. Each value is compared within half a step of the `.FIPa`'s resolution.

| Sheet | Channels read | Samples compared | Samples differing | Worst time, s | Worst value, steps |
|---|---:|---:|---:|---:|---:|
| the first | 16 | 40,841 | 0 | 0.00005 | 0 |
| the second | 13 | 48,290 | 0 | 0.00005 | 0 |

Every channel of the `.FIPa` whose quantity is known has a channel of the sheet to compare with. An
export stops the velocity from the accelerometer early, so the `.FIPa`'s 850 and 811 samples after
each sheet stops it aren't compared. An independent reader, openpyxl 3.1.5
(`validation/oracles/xlsx/cells.py`), reads every cell of both sheets as the workbook reader writes
it.

The derived channels, in each sheet:

- `[Altitude (Baro-Ft-ASL)]` is the standard atmosphere's height of the sheet's own `Baro (Atm)`,
  less one offset. The height is 44,330.77 × (1 − (P / 101,325)^0.190263) m, with P in pascals, the
  atmospheres times 101,325. The match is within the pressure's printed half step, 0.000005 atm,
  times the formula's slope, and the altitude's own. Half the spread is under 0.004 m (0.013 ft) and
  0.020 m (0.066 ft).
- `[Altitude (Baro-Ft-AGL)]` is that less one constant, the pad's. It is within both printed half
  steps (half the spread under 0.0016 m (0.0053 ft) in each). Its first sample, on the pad, is
  within 3 m (9.8 ft) of 0: 0.20 m (0.65 ft) and 0.72 m (2.4 ft). Were the two altitudes' names
  swapped, the pad would read its height in the standard atmosphere. That is more than 3 m (9.8 ft)
  except where the pad's pressure is within about 36 Pa (0.005 psi) of 101,325 Pa.
- `[Velocity (Accel-MPH)]` is `[Velocity (Accel-Ft/Sec)]`, both in m/s, within both printed half
  steps, in the first sheet. The second sheet has neither.

A test on invented rows (`a_sheet_with_its_altitudes_swapped_fails_its_pad_check`, in
`xtask/src/logs.rs`) reads a sheet both ways round. Swapped, the two altitudes still pass the ASL
and AGL checks, each one offset from the other. The pad check fails, with the pad 540 m (1,770 ft)
up in the standard atmosphere.

### The AltiMAX's Excel export

Rocketronics' AltiMAX altimeters are downloaded with the maker's PC software. It can export a flight
"to Excel as altitude values" (*AltiMAX G4 Manual*, Rocketronics, 2021, p. 52). The sample export is
an `.xls` file, the binary format of Excel before 2007. It is an unusual one: a bare *worksheet
stream*, its records end to end. It is not inside the container file (an OLE2 compound document)
that Excel wraps its own `.xls` files in. `hpr_flightdata::xls::read` writes the worksheet out as
text. The text is then read, here with `hpr_flightdata::altimax`. The decision record,
[ADR-274, the Teensy logger and the rest of the importers](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0274-the-rest-of-the-importers-planned-teensy-logger-first.md),
decided this. It did so in its amendment for [M7.1c20](../decisions-and-roadmap.md#m7-1c20).
`import::read` refuses the `.xls` bytes and names `xls::read`. `import::read_one` reads its one
sheet. So do `hpr analyze` and Python's `FlightLog.read` with no option. `--sheet 0` (`sheet=0`)
names it.

> **How far to trust it.** The one sample file is read cell for cell. The survey's own decoding and
> xlrd 2.0.2, an independent Python reader, agree with the text on all 278,685 cells. No manual
> describes the columns, so the reading is held to physics. Each height follows its pressure by the
> standard atmosphere's formula to within 4.5 mm (0.18 in). The clock, the accelerometer and its sum
> are held to the flight. The pressure columns say `[hpa]` but hold pascals, and are read so. The
> voltages are kept as stated, unchecked ([evidence row G30](../evidence.md#g30)).

**The stream.** An `.xls` file is written in BIFF, Excel's Binary Interchange File Format. Its
versions run from BIFF2, Excel 2.x's, to BIFF8, Excel 97's and later. A record is a 2-byte id, a
2-byte length and its data. *OpenOffice.org's Documentation of the Microsoft Excel File Format*
(Daniel Rentz, revision 1.42, 2008, §2.1) describes it. The sample opens with a BOF (beginning of
file) record, the stream's start, with the id `0x0809` that BIFF5 to BIFF8 share. Excel writes that
BOF's body as 8 bytes, version `0x0500`, or 16, version `0x0600` (§5.8.1). This one is 6 bytes with
a version field of 0, the kind other tools write. Rentz reads that kind as BIFF5 (§5.8.2). Its cells
settle it. They are BIFF2's records, `INTEGER` (`0x0002`), `NUMBER` (`0x0003`) and `LABEL`
(`0x0004`), the text in single bytes. And xlrd, below, reads the file as BIFF2 too. The plan called
the stream BIFF8's, from its BOF's id. The plan for the rest of the importers, in
[ADR-274's amendment](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0274-the-rest-of-the-importers-planned-teensy-logger-first.md),
records what the stream holds. The stream ends with an `EOF` (end of file) record. The reader writes these
cells out under whichever BOF opens the stream. The reader refuses these:

- a later version's cell records, by name: a shared string or a formula, as it doesn't read them;
- a record longer than 8,224 bytes;
- a cell past column `IV`, the 256th, or out of order;
- text beyond ASCII;
- a stream with a second BOF or with bytes after its `EOF`.

A number is written as its shortest form that reads back to the same double, so no digit is lost or
invented.

**The table** has one header row and a row for each sample:

```text
TIME,PRESS FILTER [hpa],HEIGHT FILTER [m],PRESS RAW [hpa],HEIGHT RAW [m],ACCEL [m/s2],
SPEED [m/s],EVENT,EVENT,VOLT. CHANNEL,VOLTAGE [V]
```

| Column | Read as | Held to |
|---|---|---|
| `TIME` | s, though it names no unit | its shortest step, 5 ms within 10 %, the 200 samples a second the manual gives (pp. 8 and 10); and the time to apogee |
| `PRESS FILTER [hpa]`, `PRESS RAW [hpa]` | Pa: the values are pascals, not the hectopascals stated | the pad's pressure, 50 to 110 kPa; and the heights |
| `HEIGHT FILTER [m]`, `HEIGHT RAW [m]` | m above the pad, the raw one the record's [primary track](#the-record) | the formula below |
| `ACCEL [m/s2]` | m/s², its sign as written | 1 g before liftoff, within 15 % |
| `SPEED [m/s]` | m/s, its sign as written | its change against `ACCEL` |
| `EVENT`, a code and its name | the record's events, where the code isn't 0 | nothing |
| `VOLT. CHANNEL`, `VOLTAGE [V]` | a voltage channel for each channel number, 1 to 5 | nothing: kept as stated |

The reader decides the pressure's unit by its first value, on the pad. Read in hectopascals as
stated, it would put the pad above 50,000 hPa. Read in pascals, it falls between 500 and 1,100 hPa,
as any launch site's does. The two ranges don't overlap, so only one reading fits. A note says so
when it is pascals. A file whose pad fits neither is refused. The reader also refuses:

- a header that names a column twice (but `EVENT`);
- a file of one row, whose clock has no step to check;
- a value that isn't a finite number or is too large for pascals;
- more than 16 voltage channel numbers.

`ACCEL` is measured along an axis pointing down the rocket. It reads −0.94 g at rest before liftoff,
where an axis pointing up would read +1 g. `SPEED` is `ACCEL` summed over the clock, gravity left
in. So `SPEED` isn't the rocket's speed, and the record's vertical speed is left empty.

<a id="how-the-altimax-export-was-checked"></a>**How it was checked.** The survey decodes the stream
itself, apart from the reader. The stream's records are 50,668 `INTEGER`, 202,672 `NUMBER` and 25,345 `LABEL`
cells, a BOF, an `EOF`, a `DIMENSIONS`, 11 column widths and 36 number formats. All 278,685 cells
equal the reader's text. And xlrd 2.0.2 (`validation/oracles/xls/cells.py`), which reads the file as
BIFF2, agrees on every one. The 253,340 values of the reader's channels equal the text read by the
stated units, but the pressures in pascals. Then, on the flight:

- **The heights.** Each of the 50,668 heights, filtered and raw, matches the standard atmosphere's
  height of its pressure in pascals, less one offset for the file. That height is 44,330.77 × (1 −
  (P / 101,325)^0.190263) m (U.S. Standard Atmosphere, 1976). The match is to within 0.0045 m (0.015
  ft). The check allows 44,330.77 × 2⁻²³ = 0.0053 m (0.017 ft) for the software's own working of the
  formula. No document gives the form of that working. The offset is the pad's height in that
  atmosphere. It isn't written, as it gives the site's elevation. Read in hectopascals, the heights
  would miss by more than a kilometer (0.6 mi).
- **The pad pressure,** the first raw pressure, is between 50 and 110 kPa (7.3 and 16 psi).
- **The accelerometer.** Liftoff is `ACCEL`'s first magnitude above 3 g. Before it, `ACCEL` reads
  0.940 g. The check allows 0.85 to 1.15. A column in g or ft/s², read as m/s², would be 9.8 or 3.3
  times off. Over the boost, `SPEED`'s change is 0.999 times `ACCEL`, inside 0.9 to 1.1.
- **The time to apogee.** From liftoff to the highest raw height, the time is 0.945 times √(2h/g), h
  the apogee's height above the first sample's. The check allows a factor of 2.

**What is left out.** The voltages aren't held to anything. The software lists five (the battery's,
the igniter's and three pyro channels', *AltiMAX G4 Manual*, p. 42) but not which number is which.
So none is named. The event codes aren't documented, so they are kept as written beside their names.
The AltiMAX's own `.ALS` file of the same flight is read too. It is held to this export ([the
AltiMAX's `.ALS`](#the-altimaxs-als)).

### A MARSA download

MARSA Systems' MARSA4 and MARSA54 flight computers share their firmware and their PC software,
MARSA_Connect. It downloads a flight and saves it to a `.dat` file (*MARSA54L Flight Computer User
Manual*, V1.4, 2014, p. 18). No document describes the file, so the reader takes its layout from the
two sample files. The survey holds each unit to a check against another logger in the same rocket.
The file is the altimeter's settings, one a line, then a row a sample. An invented download, cut
short:

```text
<BARO_ZERO 0> 790
<ACCEL_ZERO 0> 498
<FIRMWARE VER> 26
<MARSA VERSION> 2
<CHAN 1>
<CHAN APOGEE OPTION> 1
-0.1,-58.1,-0.08
0.0,-58.1,8.20
0.1,-29.0,8.65
```

**The settings** are the record's figures. Each is named as the file names it, after its section's
heading where it has one. For example, `<CHAN APOGEE OPTION> 1` under `<CHAN 1>` is the figure
`CHAN 1, CHAN APOGEE OPTION`, 1. No document gives their units, so each is kept as written, and as a
number where it is one. The MARSA4 manual names two. `BARO_ZERO` and `ACCEL_ZERO` are "the A/D
values of the ground barometric and sensor readings" (*MARSA4 User Manual*, Version 1.0, 2009, p.
17). `FIRMWARE VER` is the record's firmware.

**The samples** are three numbers that no line names. They are read as:

- the time in seconds from liftoff, negative before it;
- the barometric altitude in feet, as the manuals give it (the MARSA54L's display reads "898 ft" in
  its example, p. 12);
- the acceleration in g, its reading at rest taken out, so about 0 on the pad.

The example's last row reads 0.1 s, −29.0 ft (−29.0 × 0.3048 = −8.84 m) and 8.65 g (8.65 × 9.806 65
= 84.8 m/s²). The altitude is the record's [primary track](#the-record). It is a height above the
logger's own zero, which isn't its reading on the pad. In both sample files the pad reads three
counts below that zero, −87 ft (−27 m). The altitude moves by its barometer's count, about 29 ft (9
m) near the pad. It does not move by the tenth of a foot it is written in. The acceleration is along
the altimeter's axis, so it is the rocket's own only while the rocket points straight up. AltOS's
acceleration has the same limit.

**A spreadsheet's re-save** of the file, each setting's line ending in two empty cells and the
numbers rewritten (`-87` for `-87.0`), reads the same.

**How the units were checked.** Each sample file sits in its flight's folder in the private
collection beside an AltOS board's export. The folder's other log is the other stage's, so the board
flew in the MARSA's stage. The [survey](#how-it-was-checked) reads both and holds the MARSA's units
to them:

- **The feet.** The highest altitude, read in feet, is 0.981 and 0.986 times the AltOS board's
  highest height above its pad. The check allows 0.95 to 1.05, the plan's 5 %
  ([ADR-274, the rest of the importers' plan](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0274-the-rest-of-the-importers-planned-teensy-logger-first.md)).
  Read in meters, the altitude would be 3.2 times the board's.
  That checks the unit, not the MARSA's accuracy.
- **The g.** Liftoff is the first acceleration above 3 g. Over the first 2 s after it, the
  barometric climb is 0.915 and 1.035 times the acceleration integrated twice from rest. The check
  allows 0.6 to 5/3, the band of the [`.eeprom` reader's climb check](#altoss-eeprom). The band is
  wide so that only a wrong unit fails it. Read in m/s², the climb would be 9.0 and 10.1 times the
  integrated acceleration.
- **The g at rest.** Before liftoff the acceleration's median is −0.12 g in both files. The check
  holds it within 0.5 g of 0, nearer 0 than the 1 g an accelerometer with gravity left in would
  read.
- **The decoding.** All 5,712 comparisons of the reader's record with the survey's own decoding of
  the files agree. Each sample gives six: its time and altitude on the primary track, and the times
  and values of its altitude and acceleration channels. The figures are the settings, by name and
  value.

**What is left out.** The settings other than the two zeros aren't described anywhere, so none
is read as a quantity. No line of the file is an event, so the record has none.

### AltOS's telemetry as a table

This is a table of AltOS's radio telemetry, one packet a row under a header of the packet's fields.
The packets are the ones an [AltOS `.telem` file](#altos-telemetry) logs, decoded. The table is read
from one sample file and the maker's description of the packets. Only the TeleMega's `0x09` packet
is read, one board and one packet type to a table. Only the Rust library reads such a table. An
Altus Metrum board's radio packet opens with its serial number, its clock in ticks of 10 ms and the
packet's type. The type lays out the rest (*AltOS Telemetry*, Bdale Garbee and Keith Packard,
v1.9.22.2, 2025, §2.1, p. 2). The sample file holds a TeleMega's type `0x09` packets, "TeleMega
Kalman and Voltage Data for 15V boards" (§2.3, Table 6). An invented row:

```text
serial,tick,ptype,state,v_batt,v_pyro,sense,v_apogee,v_main,ground_pres,ground_accel,accel_plus_g,accel_minus_g,acceleration,speed,height,crc
1234,402.94,9,3,4.1,4.19,0.00 0.00 0.00 4.20,4.2,4.14,98500,-28,-29,10,62.25,27.375,6,TRUE
```

The reader recognizes the header's opening, `serial,tick,ptype,`. It reads each column by its name:

- `tick` is the clock in seconds;
- `height` is the board's estimate of its height above the pad, m, and the primary track;
- `acceleration` and `speed` are its estimates along its axis, m/s² and m/s (the packet carries
  sixteenths, and the table writes them divided out);
- `ground_pres` is the barometer on the ground, Pa;
- `v_batt`, `v_pyro`, `v_apogee`, `v_main` and each number of `sense`, one a pyro channel, are in
  volts.

The row reads 402.94 s, 6 m (20 ft), 62.25 m/s² and 27.375 m/s (89.81 ft/s). The voltages are volts,
not the packet's counts. The example's 4.1 V is a single lithium-polymer cell's, the battery a
TeleMega takes (*The Altus Metrum System*, v1.9.22.2, §3.2, p. 6). Each change of `state` is an
event, named as AltosUI's exports name the number, `3` `boost` here.

A row whose `crc` is `FALSE`, a packet that failed its check, is left out and counted. A row of
another board or packet type, or a tick no later than the one before, refuses the file at its line.
That is because the tick turns over every 655.36 s. A table of seconds can't tell a turn from a
board switched off and on.

**How it was checked.** In the sample file, all 325 rows are type 9 and pass their CRC.

- **The decoding.** All 12,025 comparisons of the reader's record with the
  [survey's](#how-it-was-checked) own decoding agree. They cover each value and its time in the
  channels, and each time on the primary track.
- **The units.** From boost to coast, the median over pairs of rows of the height's change over
  their time is 1.019 times their mean speed. For the speed's change over their time, the median is
  0.929 times their mean acceleration. Each check allows 0.6 to 5/3. A speed or an acceleration left
  in the packet's sixteenths would miss it. So would a height in feet or a tick in its 10 ms counts.
  The battery reads 4.09 to 4.10 V. That is within the 2.5 to 5 V that the check allows. The other
  voltages are read as volts and held to nothing. The first ground pressure lies between 50 and 110
  kPa (7.3 and 16 psi).
- **The resolution.** Every acceleration and speed is a whole number of sixteenths. Every height is
  a whole meter and every tick a whole hundredth of a second. The packet carries them that way. This
  is a measure, as a wrong unit could keep them too. On the pad the acceleration's median is −0.44
  m/s², about 0.

### AltOS's CSV after a spreadsheet

Two sample files read as AltOS exports that went through a spreadsheet, and
[AltOS's CSV reader](../api/hpr_flightdata/altos/index.html) reads both. Each case below rests on
one sample file and on invented ones in the library's tests ([evidence row G6](../evidence.md#g6)).

**A table cut from an export** holds AltOS's columns alone. Its header has AltOS's own names with no
`#version,serial,flight` before them, such as
`time,state,state_name,acceleration,pressure,altitude,height,speed`. Names alone don't show the file
is AltOS's, so its states must. AltosUI's exports write each row's state as a number and a name. The
pairs are 2 `pad`, 3 `boost`, 4 `fast`, 5 `coast`, 6 `drogue`, 7 `main` and 8 `landed`. Of the
file's first 100 rows, at least one must give one of these pairs. None of those rows may contradict them, such as
3 named `pad`. A row further on that contradicts them refuses the file at its line. A full export,
whose header shows it is AltOS's, is held less strictly. A contradicting pair is counted in a note
and its name kept as written, as Debrief's sample export numbers `pad` 1. In the sample file, all
1,005 rows give AltOS's pairs. All 15,948 comparisons of the reader's record with the survey's own
decoding agree. They cover each value and its time in the channels, and the primary track's times
and heights. Those keep the first row at each time AltOS repeats. The record's events are also
AltOS's names. The file's `altitude` less its `height` lies within 0.005 m (0.016 ft) of one offset, the
pad's altitude. The check allows 0.01 m (0.033 ft), the two columns' centimeter. Its `altitude`'s
rise from the first row to its highest is 1.000 times the standard atmosphere's height's rise of its
`pressure`. The check allows 0.6 to 5/3. An altitude in feet would give 3.3. And, a measure, its
`altitude` is that height of its `pressure` to within 0.006 m (0.020 ft).

**An export a spreadsheet re-saved** can carry three changes:

- *A unit after a name,* such as `height (m)`. The column is read by its name. The unit may be the
  one the reader reads that column in, such as `m` for an altitude or `m/s^2` for the acceleration.
  Then the column reads as AltOS's. Any other unit, such as `altitude (ft)`, keeps the column as
  written, as the reader can't tell how its values were made. So does `pressure (Pa)` over values
  that read as hectopascals. A unit after `time` other than `s` refuses the file, as the clock can't
  be placed.
- *Formulas,* such as `=3.28084*J2`, in place of values. The reader doesn't work them out. A column
  of formulas alone isn't a channel. A column with some has those cells as gaps. Either way the
  formulas are kept as text in a note, run by run. Each reference to the cell's own row is written
  `{row}`. For example, `=3.28084*J2` on line 2 and `=3.28084*J3` on line 3 are one run,
  `` `=3.28084*J{row}` on lines 2–3 ``. Each cell the note gives can be rebuilt from it, as a
  spreadsheet's CSV writes its row *n* on line *n*. A note gives up to 8 runs a column and 200
  characters a formula. Up to 64 columns are noted. Past those limits the notes count the cells they
  leave out.
- *A deleted `height`.* With no `height`, the record has no primary track, only its channels.

The sample file has all three: 26,916 rows, three columns of formulas only and `height` deleted.
Each of its 80,748 formulas, rebuilt from the reader's notes, equals the file's cell, so none was
cut. All 4,198,896 comparisons of the reader's channels with the survey's own decoding agree, each
value and its time.

### Formats that need samples

Three of the formats the roadmap names have no sample among the logs the survey reads ([evidence row
G9](../evidence.md#g9)). GPX's layout is published and another program reads it. So its reader was
built from the schemas and checked on invented files against that program ([GPX](#gpx)) ([evidence
row G1](../evidence.md#g1)). For the other two, CATS's and "Juno", no reader can be built and
checked yet. That is because a reader is built from samples, the bytes a logger wrote. It is best
built with the maker's own export of the same flight beside them. The table says what was found for
each of the two, and what a reader would need. The search, of makers' pages and public repositories,
is in
[a research note](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/sample-logs-cats-juno.md),
read on October 16, 2026. A sample of a format in the table below can be offered in
[its issue, #662](https://github.com/nrdptel/fusionspace-eridanus/issues/662), with the terms it may be tested on.

| Format | Samples found | What a reader needs |
|---|---|---|
| CATS Vega `.cfl`, the flight computer's own log | none among the survey's; 11 public logs from the EuRoC 2023 competition in CATS's own repository, under the GPL-3.0, and 13 in its gallery, licensed to CATS alone | samples on terms that let their bytes be tested, and a layout: only CATS's GPL source code publishes one, and this project doesn't read GPL code, so a reader would be checked against the CSV the CATS Configurator exports, the program run as an outside tool; offer one in [#662](https://github.com/nrdptel/fusionspace-eridanus/issues/662) |
| CATS ground station's CSV | none among the survey's; 22 public files beside those logs, under the GPL-3.0 | samples on terms that let them be tested; its header states each unit, such as `altitude[m]` and `ts[deciseconds]`, so the [table reader](#tables-read-by-their-headers) may take it, untried; offer one in [#662](https://github.com/nrdptel/fusionspace-eridanus/issues/662) |
| "Juno" | no logger of that name found: an earlier roadmap's "EuRoC/Juno CSV layouts" meant a rocket, Projeto Jupiter's Juno III, whose files RocketPy's repository holds | a sample from a logger named Juno, if one exists; none was found. The rocket's own files would need no new reader: its altimeter's table has an RRC3 export's columns, which [the RRC3's reader](#the-layouts-read) reads, though it isn't run on them here, as RocketPy's data isn't among the survey's samples; its GNSS receiver's table and its team's telemetry state no unit, so they read only by a [column mapping](#mappings-a-publisher-states) someone states, and none is stated; offer one in [#662](https://github.com/nrdptel/fusionspace-eridanus/issues/662) |

The roadmap names no others, but makers HPR Sim reads also write formats no sample holds. AltOS's
`.eeprom` log formats other than 1, 6, 7, 14, 15, 16 and 22 are refused, naming the format ([AltOS's
`.eeprom`](#altoss-eeprom)). Public files in formats 5, 9 and 19 exist, but none with terms that let
it be tested ([evidence row G4](../evidence.md#g4); offer one in
[#657](https://github.com/nrdptel/fusionspace-eridanus/issues/657)). The others with no sample HPR
Sim can test are these ([evidence row G9](../evidence.md#g9); offer one in
[#662](https://github.com/nrdptel/fusionspace-eridanus/issues/662)):

- Entacore AIM USB's `.aim` files;
- the Eggtimer Quasar's and ION's CSV;
- the Featherweight Blue Jay's;
- a MicroPeak's memory;
- Jolly Logic's AltimeterThree export;
- the EasyTimer V2's, TeleMini's and EasyMotor's logs.

## The record

Every reader fills the same record, `FlightLog`, in SI units:

- **What the file says about itself:** the logger's name, serial number, firmware and flight
  number where the file states them. The record also holds the figures the logger worked out
  itself, such as its apogee. These are kept beside HPR Sim's readings and never in place of them.
  A file that states many such figures, as a [Featherweight summary](#featherweights-summaries)
  does, keeps each one in the record's `stated.figures`. Each keeps its name and value as written.
- **The primary track:** the barometric altitude above the pad (AGL, above ground level) on the
  logger's clock. The track also holds the barometer's vertical speed, the temperature and the
  battery voltage where they share that clock. The
  [readings](../physics/log-readings.md#the-height-track) take the track first. Where it is empty,
  they take a pressure or an altitude channel. A log with no barometer, such as a
  [GPS log](#gps-logs), has an empty primary track. So does a [raw download](#raw-downloads),
  whose barometer's readings are pressures, not altitudes. So does an
  [AltOS export a spreadsheet re-saved](#altoss-csv-after-a-spreadsheet) with its `height` deleted.
  A [summary](#featherweights-summaries) has no samples at all, so its clock and tracks are empty.
- **Every column, as a channel:** each data column of the file, in the file's order. Each channel
  carries the name the file gives it, what it measures, and its own clock. Nothing the logger wrote
  is dropped. A column HPR Sim can't name a quantity for is kept as written and marked
  *unrecognized*.
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
cell the file leaves empty, or fills with something that isn't a number, is a gap. A gap is `NaN`
("not a number") in the record, and `null` in its JSON, which reads back as a gap. A channel's time
is a gap in the same way where the file gives a sample no time, as a KML path without times does.

A cell can be a valid number and still overflow when it is converted to SI. `1.7e308` read as g
becomes 1.7e308 × 9.806 65. That is more than the largest value a 64-bit floating-point number
holds, about 1.8e308. The reader then refuses the whole file and names the line. So a bad cell
never reaches a channel as an infinite value. Every reader applies this rule, so a channel holds SI
values or gaps and nothing else ([ADR-258, the rule for infinite values](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0258-aerospacenus-board-and-a-phone-apps-csv-before-the-rest.md),
§3).

### Units no source states

Most units come from the file's own column names or from the maker's manual. A few don't:

- AltOS's `pad_dist` and `pad_range` are read as meters, as its manual says its exports are
  metric, without naming these columns.
- AltOS's gyroscope and magnetometer columns (`gyro_*`, `mag_*`) are kept as written, unrecognized,
  as no document states their units.
- The Raven's status columns, `bILBA`, are kept as numbers, the bits read as binary. No published
  source says what each bit means, so the record takes no events from them.
- The Blue Raven's `Current` is kept as written, unrecognized. The app's file states no unit. The
  Blue Raven's manual gives milliamperes only for another download. The values fit amperes better,
  but that is a size, not a source.
- An RRC3's `Temperature` with no unit in brackets is kept as written, unrecognized. mDACS's
  preferences set the units it applies (*mDACS User Manual*, rev. 1.71, p5), and two public files
  in feet hold one in °C and one in °F ([evidence row G31](../evidence.md#g31)). To have it read,
  put its unit in the header: `Temperature [C]`. Its `Voltages` with no unit are read in volts,
  assumed. Each record says so in a note.
- GPS Visualizer's KML altitude is kept as written, unrecognized. The program takes its source's
  altitude as meters unless the source's column names feet. So its KML holds meters only if the
  source said which, and the file can't show whether it did. The one such sample, set beside a
  barometric logger on the same flight, holds feet written as meters.
- Two KML files styled after AltOS's, which name no writer, give each point a `Seconds` and an
  `Alt` in a comment. The seconds are read as seconds, from the name. The altitude is read in
  meters, from the KML standard, as a height above the ground. Their tracks rise less than half as
  high as their flights' barometric peaks. Neither meters nor feet explains that, so trust their
  heights least of any GPS log.
- AltOS's KML writes a `time` beside every point. Where it is 0 at every point, as in most of its
  sample files, the reader takes it as unset. Then the points have no time. No document says so.
- AltOS's KML also writes the GPS receiver's `alt` beside every point. Where it never equals the
  path's altitude, as in 3 of its older sample files, nothing in the file shows its unit. In that
  case, the GPS receiver's altitude is kept as written.
- AltimeterCloud's Mercury writes an air density, a predicted apogee, a target altitude and a
  second temperature (`mt1_temp`). No published page gives a unit for any of these. Each is kept
  as written, unrecognized.
- Entacore AIM's magnetometer (`Mag X`, `Mag Y`, `Mag Z`) is kept as written, unrecognized. Its
  manual names tesla, but the sample files' values are 17 to 34 times the Earth's field read so
  ([below](#units-the-file-decides)).
- A Featherweight summary's four pad accelerations and offsets (`Pad Low-G Accel`, `Pad Hi-G
  accel`, `Low-G accel pad offset`, `Hi-G accel pad offset`) are kept as written. They are three
  numbers with no unit, which no source states ([below](#featherweights-summaries)).

### Units the file decides

These layouts leave some units out of their headers:

- the RRC3's;
- the Eggtimer's: its CSV and terminal text, and that text saved as RTF;
- the Blue Raven's and AltimeterCloud's;
- Entacore AIM's, a Fluctus's and an ARTS2's;
- AerospaceNU's flight computer board's and its ground station's, and a phone app's.

A `.pfa` states none ([below](#perfectflites-older-pfa)). A SillyGoose's
header names units that one file's values contradict. For some, the reader works the unit out from
the file as it reads it. For the rest, it takes the unit the maker's manual gives. It checks that
unit against the file where the file can show a disagreement. Either way the reader writes a note
giving the decision and this file's evidence. The [survey's report](#how-it-was-checked) counts the
decisions over the sample logs:

- **An RRC3's altitude, speed and clock, worked out from the file.** Missile Works' mDACS is the
  program that downloads an RRC3's flights to a computer. It writes the altitude, speed and
  temperature each in the unit chosen in its preferences. The header names none of them unless the
  units are in brackets. The pressure settles the altitude. The climb the pressure implies in the
  standard atmosphere is set against the altitude column's climb. That climb is read in feet and in
  meters, 3.28 times apart. The unit within 25% of the pressure's climb is taken. The speed is set
  against the altitude's rate of climb, allowing for the speed to lag behind it. Of feet per
  second, miles per hour, meters per second and kilometers per hour, the one two measures agree on
  within 3% is taken. Where they don't, or the record stops before the rocket is halfway down, the
  column is kept as written, with a note. The time is in seconds where its step is the RRC3's 20
  samples a second. Of the 51 RRC3 exports among the sample logs, 47 are read in feet from the
  pressure and 4 state feet. Of the speeds, 36 are in feet per second and 9 in miles per hour.
  Another 4 state feet per second. The remaining 2, whose records stop early, keep their speeds as
  written.
- **An RRC3 export saved by a European locale.** mDACS's export in a European locale's form is
  semicolon-separated with decimal commas. It may end every line, the header's too, with a
  semicolon. The reader drops the empty cell after that semicolon, on full rows and short ones. The
  one such sample had its rows sorted by a spreadsheet by their time's text, `10,05` before `2`.
  Put in order by value, the rows step 0.05 s with none missing. But the reader keeps a file's
  order and refuses a time that goes back, naming the line. So that file is refused and counted as
  damaged. To read such a file, sort its rows by time first.
- **An Eggtimer's clock, worked out from the file.** The Classic and TRS write their time in
  milliseconds, the Quantum and Proton in seconds, under the same `T`. The reader takes
  milliseconds when the median step is above 5. The manuals' rates, 1 to 33 samples a second, put
  a step between 0.03 and 1 in seconds and between 30 and 1,000 in milliseconds. In the CSV
  download, a step above 1 and below 30 is refused (below). Of the 44 Eggtimer files read, 4 are
  read in milliseconds and 40 in seconds.
- **An Eggtimer's heights and speeds, from the manuals.** The Classic and TRS manuals state feet
  and feet per second. The Quantum and Proton manuals state no unit for the download, but give
  every height in feet. No Eggtimer manual offers meters. The heights are read in feet for every
  model. In a terminal download, plain or saved as RTF, they are read in feet only where a condition
  holds. The condition is that the coast before apogee also slows at about g in feet (below). A speed
  is read in feet per second only where it matches the altitude's change over the step on at least
  90% of rows. On the sample logs, the match is on 95.9% or more of rows. Otherwise the speed is
  kept as written.
- **An Eggtimer's filtered altitude, held to `Alt`.** The filtered altitude must scale to `Alt`
  as a filter of it can. The one multiplier that best takes `Alt` to it, with no offset (a
  least-squares fit through zero), must lie from 0.1 to 2. Otherwise the file is refused. A filter
  lags the altitude, under it going up and over it coming down. On a steady climb at $v$ ft/s to
  $H$ ft, a lag of $\tau$ s lowers the multiplier by about $1.5\,v\tau/H$. For 100 ft/s, a lag of
  1 s and a climb to 600 ft, the multiplier is 0.75. A decimal place lost writes a value 10 times
  over, two places 100 times. A refused file's numbers can't be trusted, its clock's among them. A
  seconds clock written without its decimal point reads as milliseconds. The check doesn't catch a
  column off by less than a factor of 2, such as a filtered altitude in meters (0.3048 times). On
  the 44 sample files read, the multiplier is 0.900 to 1.000 (the
  [survey's report](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/log-import.md)
  prints the range). A 45th, a download a spreadsheet saved with its decimal points taken out
  (`12.34` written `1234`), gives 99.9. It is refused, and counted as damaged. In the CSV
  download, a median step between rows above 1 and below 30 is neither seconds nor milliseconds at
  the manuals' rates. The file is refused too. A `0.05`&nbsp;s step written without its point is `5`.
- **An Eggtimer download saved by a European locale.** A spreadsheet set to a European locale
  saves the CSV download with semicolons between the cells. It may write its numbers with a
  decimal comma, the mark between a number's whole part and its fraction (`0,05` for 0.05). A
  header that reads as an Eggtimer's only when split on semicolons is split on them. The reader
  then uses a decimal comma only if two conditions hold. No number has a point. And some number has
  a comma before anything but three digits (`0,05`). A cell in the other mark, such as `1,250` in a
  file of points, is refused on its line. The table reader takes the same cell as a gap. Here a
  cell the manuals write as a number can't be left out, so a thousands separator is never read as a
  decimal mark. Two of the sample downloads, from the European Rocketry Challenge (EuRoC), are
  semicolon-separated. One has decimal points, and is read. The other has none at all, as a
  spreadsheet took them out, and is refused (above). Decimal commas are tested on invented files
  only ([evidence row G2](../evidence.md#g2)).
- **A Blue Raven's speeds, inertial positions, rates of turn and accelerations, from the
  manual.** The manual gives their units for the logger's USB download, which HPR Sim reads too
  ([below](#the-blue-ravens-usb-download)). The units are feet per second, feet, degrees per second
  and g (*Blue Raven User's Guide*, May 12, 2026, p23). The app's own files
  agree. Its summary of a flight states the top speed in feet per second and the inertial
  altitude in feet. Its accelerometers read 1 g at rest on the pad. A file that disagreed
  would still be read in these units. The note on that file would show the disagreement.
- **An Eggtimer TRS's terminal download, its preamble by its own unit marks.** Captured in a
  terminal program, a TRS's download prints its flight settings and a flight summary. Both come
  before the table its CSV download writes (*Eggtimer TRS Flight Computer User's Manual*, firmware
  2.07, p35–37). The table is read as the CSV download is. But its clock and its heights each rest
  on two checks that agree. The clock rests on its step and the coast. The heights rest on the
  manuals' feet and the coast. Just before the top nothing pushes the rocket, and its drag fades as
  it slows. So its altitude curves down at gravity: 32.17 ft/s² in feet and seconds, 9.81 m/s² in
  meters. The reader fits a parabola to each height over the 3 s up to the top. The fit stops early
  at the row before a charge's marker or a jump in the altitude. An Eggtimer can fire a charge on
  the way up. The window is cut short 16 times at most before the file is refused as having no clean
  coast. The reader takes a slowing of 0.7 to 1.3 g. A clock 1,000 times off would put the slowing
  a million times off. A file whose clock this doesn't bear out, or with no such coast, is refused.
  A height that slows at g in meters rather than feet is kept as written, with the speeds that are
  its change. In the 3 sample files, two text and one RTF, the raw heights slow at 0.96 to 1.02 g
  in feet. The summary's apogee becomes the record's stated apogee, and its `ASL Alt.` the launch
  site's elevation, each read by its own unit mark, `ft`. The summary's event times, written
  without a unit in both sample files, are read in seconds, as the manual's download writes them
  (`secs.`, p36). They are on the table's clock: in both sample files every one matches its
  marker's row to the millisecond. So a summary time with a matching marker is kept once, as the
  marker's event. The top speed's time, which no marker marks, becomes an event of its own. A time
  of 0 means a channel that didn't fire (p25). The settings are kept as written in the record's
  notes. So are the other summary lines, such as the base temperature with its own `F` or `C`. The
  name the owner gave the flight computer is left out, since it may name a person.
- **AltimeterCloud's temperature and summary times, worked out from the file.** The Nano V1's
  manual has a page on its CSV layout (*CSV Flight log format*,
  <https://www.altimetercloud.com/support/nano-v1/flight-log-format/>, read October 13, 2026). The
  reader follows Debrief's in recognizing the layout and in the centi-degree temperature.
  - *From the manual:* time in milliseconds from liftoff, altitude above the pad in meters,
    acceleration in thousandths of a g. Rates of turn and attitude angles are in degrees. The
    Mercury's CSV uses the same names for the same columns. So its rates and angles are read in the
    Nano's units too, with a note saying so.
  - *Total acceleration:* `acceleration_total(mG)` is the acceleration's magnitude with 1 g taken
    out. So it reads about zero at rest and −1 g in free fall. It is kept so, as an acceleration
    along no axis, and its note says what it is.
  - *The Mercury's temperature:* `bmp_temp(x100)` holds hundredths of a degree Celsius. It is read
    so only when three things hold. Every value is a whole number. Every value divided by 100 falls
    within the barometer's measurement range. That range is −40 to 85 °C (Bosch Sensortec,
    *BMP581 data sheet*, BST-BMP581-DS004-13, table 2, p. 10). And some values as written fall
    outside that range, so the column can't already be in degrees. Otherwise it is kept as written.
  - *The summary's times:* the block at the top of a Nano log gives the apogee's and the landing's
    times with no unit. The apogee time is read in seconds when the log's highest altitude falls
    within half a second of it. In all 7 sample logs with a summary, the two agree within 0.01 s.
    The landing time is read in seconds when it passes two tests read so, but not read in
    milliseconds. First, the time comes after the highest altitude. Second, the log's altitude there
    is the summary's landing height within 0.01 m (0.033 ft). Read in milliseconds, the time must
    fail at least one of them. The manual's *Flight info page* gives the landing as "when it landed, and the height it came to rest at". In all 7, the two agree
    within 0.001 m (0.0033 ft). Read in milliseconds, the time falls on the climb, more than a
    meter from the landing height. A time the altitude doesn't bear out is left out, with a note
    saying why.
  - *Kept as written:* the Mercury's air density, predicted apogee, target altitude and
    `mt1_temp`, which have no published unit.
  - *Gaps:* two kinds of value aren't readings and become gaps, each counted in a note. One is the
    zeros the accelerometer writes on the pad before its first reading. The other is the exact 0 °C
    temperatures that end every sample log after readings of 27 to 44 °C.
- **Entacore AIM's pressure, heights and clock, worked out from the file.** The AIM's export puts
  a `time` column before each group of channels. Each group is on its own clock (*AIM XTRA/BASE
  v4.x.x Manual*, Entacore, 2 February 2025, p40). Every column is kept on its own clock,
  following Debrief's reader. The record's altitude is the AIM's own `Pressure AGL`, its
  barometric height above the ground it was set to. Any unit its evidence doesn't settle is kept
  as written, with a note saying why.
  - *From the manual:* g for the accelerations (p23–24) and degrees per second for the gyroscope
    (p24). The ground station's `RSSI` is in dBm (p29), kept as written.
  - *Pressure:* pascals, as its first value that is a launch site's pressure in any unit is one in
    pascals. The manual's sample export writes pascals too (Table 2, p40).
  - *Heights:* `Pressure MSL` and `Pressure AGL` are in meters or feet, whichever matches how far
    the pressure climbs in the standard atmosphere. That works as for the RRC3, from a climb of 2 m
    (6.6 ft). A file that fits neither, or whose pressure can't settle it, is refused.
  - *Clock:* `time` is in seconds where `GPS Time of Week`, in milliseconds, rises about 1,000 for
    each unit of it, and in milliseconds where it rises about 1. Without that column, the step
    between samples decides, against the AIM's 10 to 500 ms sampling periods (p2, p21). A file the
    step can't settle is refused.
  - *Temperature:* it follows the heights' unit system, °C beside meters and °F beside feet. That
    is because the software uses one unit system for all it shows (p33). Its first value must be
    one an electronics bay can have.
  - *Voltages:* volts, as every value lies within 0 to 20. The batteries the manual names are 3.5
    to 10 V (p10). Millivolts would read in thousands.
  - *`GPS MSL`:* in the heights' unit where it reads within 300 m (984 ft) of `Pressure MSL` in
    that unit and not in the other. At a site too low for the 3.28 factor to show, it is kept as
    written.
  - *Magnetometer:* kept as written. The manual says tesla, but the sample files' field would then
    be 17 to 34 times the Earth's at its strongest, about 65 µT.
  - *A run-out GPS group:* the export writes the GPS group one empty cell too wide once the GPS
    stops logging. This happens in 11 of the 13 sample files with GPS columns. The extra cell pushes every
    later column one place right. The reader drops that cell only where every group is then whole,
    and counts each drop in a note. A header naming a column twice is refused.
  - *Events:* the rises of `launch detect`, `apogee detect` and the outputs' `A on.` … `D on`. The
    AIM detects launch only after the rocket has kept above 30 m/s (98 ft/s) for half a second
    (p36). So that event comes after liftoff.
- **A SillyGoose's units, its header checked against its values.** AerospaceNU's SillyGoose board
  names most units in its columns, such as `pressurePa`, `tempK` and `altitudeM`, but two sample
  files' values contradict their names. One has temperatures below zero under `tempK` and an
  accelerometer that reads 1 at rest. The other looks replayed rather than recorded. After a few
  samples on the pad it jumps into the boost, it ends still climbing, and its temperature never
  changes. It has `angularVelRadS_*` rates beyond what a gyroscope (a sensor of the rate of turn)
  can read. So the reader checks each named unit against the file, and keeps a column as written,
  with a note, where the values refute it.
  - *Accelerometer:* m/s² or g. A board at rest on the pad or hanging under a parachute reads
    1 g. The reader takes the unit in which the board reads 1 g, within 10%, for at least 1 s. That
    reading must also last at least twice as long as in the other unit.
  - *Gyroscope:* rad/s or °/s. While the board reads about 1 g, the reader compares how far its
    rates say it turned. It compares that with how far gravity's direction turned in the
    accelerometer. A log too short to show a turn keeps the gyroscope as written.
  - *Temperatures:* kelvin where every value lies within 233.15 to 333.15 K (−40 to 60 °C). That is
    the range the project takes for an electronics bay. Otherwise the column is kept as written.
  - *Heights:* `altitudeM` must climb within 25% of what the pressure gives in the standard
    atmosphere. Otherwise the file is refused, as the record's altitude would have no unit.
    `unfiltAlt` is read in meters on the same check, and otherwise kept as written.
  - *Speed:* `velocityMS` must match the altitude's rate of climb, within 25%.
  - *Events:* each rise of `drogueFired` and `mainFired`; the flight-state codes, which no source
    publishes, are kept as numbers.
- **A Fluctus's GPS altitude, worked out from the file.** Silicdyne's Fluctus writes each unit in
  brackets in its header, such as `baro-altitude (m)`, with a decimal comma (*Fluctus – Full
  Documentation* v1.7b, p27, p58). But `gpsAltMSL` has none, in the header or the manual. It is
  read in meters only where the barometric altitude bears it out at the GPS's fresh fixes, the
  rows where its reading changes. That takes at least 10 fixes, spanning at least 120 m (394 ft) of
  the barometric altitude. The fixes must lie along a straight line, with scatter within 10% of
  that span. The line's slope must be within 10% of 1. In feet the slope would be 3.28. The 120 m
  (394 ft) lets two readings each 6 m (20 ft) off move the slope by no more than 10%. That error is
  twice the manual's ±3 m (9.8 ft) horizontal figure (p18). That is because a GPS's vertical error
  runs about 1.6 times its horizontal. Otherwise, as where the GPS lost its fix in the climb or
  lags the flight, it is kept as written. In all 6 sample files it is kept. The latitude and
  longitude are degrees, from the manual (p38) and a range check. Each change of `status` is an
  event named as the manual names it (p23).
- **An ARTS2's unit system, worked out from the flight.** The ARTS2's interpreted data comes in
  English or metric units. Neither the file nor the manual (*ARTS2 User Manual* V1.0.0, Ozark
  Aerospace, 2007) says which. So the reader works it out from the flight:
  - *Velocity, and the length unit:* at apogee the drag vanishes, and the logged velocity falls at
    g: 9.81 a second in m/s, 32.17 in ft/s. The reader fits the last second of coast before the
    velocity crosses zero. If the acceleration jumps in the 2 s before (a parachute out early), the
    reader fits the second before the jump instead. Separately, the barometric altitude moves in
    steps of one sensor count, about 92 Pa (the manual's MPXA4100A sensor and 10-bit converter,
    p16). That step is 5.5 to 14.8 m (18 to 49 ft) near any ground, so 17.9 to 48.6 written in
    feet. The file is read only when both give the same unit, which also pins the clock in seconds.
  - *Acceleration:* g, or the velocity's unit per second, whichever makes its integral over the
    boost the velocity's rise, within 10%.
  - *Altitudes:* the inertial altitude `iAlt` must be the velocity's integral. The barometric
    altitude `pAlt`'s peak must be within 25% of the climb the velocity gives, from a climb of 176
    m (577 ft).

  A file whose velocity or barometric altitude no check settles is refused. An acceleration or
  inertial altitude none settles is kept as written. The barometric altitude is the record's
  altitude, above the pad reading the logger took on entering flight mode (p17). The file starts
  at launch. The velocity and inertial altitude are the logger's integrals of its accelerometer,
  and drift by the end of the descent. So the record keeps them as channels, not as its vertical
  speed. The same flight's `.odf` file holds the sensors' counts with the calibration
  ([the ARTS2's raw data](#the-arts2s-raw-data)).
- **AerospaceNU's flight computer board, each unit by two checks.** The flight computer board
  (FCB) of AerospaceNU, Northeastern University's rocketry club, logs a binary file. The club's
  tools turn it into a CSV. The club publishes both (AerospaceNU, *rocket-flight-data*,
  `README.md`, read October 12, 2026). The 20 sample files come under eight headers of 73 to 87
  columns. No document describes the columns. So a unit a column's name doesn't state is read
  only where two checks on the file's values agree. Neither check is one that the weather or the
  flight can meet in the wrong unit. A check sets the column against a physical mark (1 g at rest,
  a barometer's whole pascals, a sensor's published sensitivity) or against another column. Where
  that is a sibling, such as the inertial units' average against the units, it counts only once the
  sibling's own unit is decided. So no counted check relates two columns that share one unknown
  unit. Where the checks don't agree, the column is kept as written, with a note:
  - *Pressure, in atmospheres:* a pad's pressure lies near 1 in atmospheres or bars. It is far from
    1 in pascals, hectopascals, psi or inches of mercury. And every value is a whole number of
    pascals divided by 101,325, the club's conversion, a step no flight or weather can mimic. In
    bars the values would miss a step of 1/100,000 by up to half a step. Every barometer that
    reads in the 20 files is in atmospheres. One that reads 0 throughout is kept as written.
  - *Clock, in milliseconds or seconds,* whatever the column's name says. The climb the pressure
    gives must match, within a factor of 10, the one the accelerometer integrates to on that clock.
    The rows' median step must also lie within 5 to 500 ms on it. A file neither unit fits is
    refused, as nothing in it would have a time. Of the 20, `timestamp_ms` holds milliseconds in
    12, and `timestamp_s` holds milliseconds in 6 and seconds in 2.
  - *Accelerations, in m/s² or g:* the board reads 1 g at rest. Each scaled column is its raw
    counts times one of the inertial unit's published sensitivities (*LSM9DS1* data sheet,
    STMicroelectronics, DocID025715 Rev 3, Table 3). The board's own estimate, `acc_*`, is
    checked by the pad's 1 g and the climb instead. One board, the FCBV2 in both its files,
    reads 1.5 g at rest, its counts scaled for another part. That board is kept as written. So is
    the high-g accelerometer in every file, whose scale is none of the LSM9DS1's.
  - *Rates of turn, in rad/s:* the gyroscope's turn must match how far gravity's direction turned
    in the accelerometer while the board read about 1 g. The gyroscope's scale must also be one of
    the LSM9DS1's gyroscope sensitivities.
  - *Heights and speed, in meters and m/s:* `pos_z` is the board's filtered altitude above the pad
    and the record's altitude. It and the other heights must climb within 25% of the pressure's
    climb. They must also climb within a factor of 1.8 of the accelerometer's ($1.8^2$ is 3.24,
    under the 3.28 between meters and feet). `vel_z` must match `pos_z`'s rate and the
    accelerometer's speed. A GPS may write 0, or hold an old value, until its first fix. So
    `gps_alt` is checked only at its fresh fixes (a new value other than 0). Each check is a line
    fitted with its own offset, so that no pad reading enters either. A pad reading far from where
    its fixes put the pad is a held value, and the column is kept as written. Where `gps_alt` is
    read, a 0 is a gap. `pos_z` reads in meters in all 20 files, `gps_alt` in 12 (kept as written
    in 8) and `vel_z` in 18.
  - *GPS position:* NMEA 0183's degrees and minutes run together (42.5° written `4230.0`) put
    every latitude more than 1° from the equator past 90. They put every longitude more than 2°
    from the prime meridian past 180. No degrees reach those values. `gps_lat` and `gps_long` are
    read in degrees and minutes where three things hold. Most latitudes pass 90 and most
    longitudes 180. Every value's minutes are under 60. And every converted value is a latitude or
    longitude. `gps_lat` and `gps_long` are read in degrees where every latitude lies within ±90 and every longitude
    within ±180. A position (0, 0) is no fix: it takes no part, and where the position is read it
    is a gap. The companions `gps_lat_mod` and `gps_long_mod` count in neither decision. They are
    read in degrees only where they hold the decided position in degrees. Of the 20 files, 15 read
    in degrees and minutes and 4 in degrees. In 3 of the files read in degrees, the companions are
    kept as written. These companions hold the values converted as if they were degrees and
    minutes. The remaining 1, with no fix, is kept as written. NMEA 0183 writes the hemisphere (N
    or S, E or W) in a field of its own, which the board's columns drop. So only a sign can carry
    the hemisphere. Where no value in `gps_lat` or `gps_long` is negative, a place west of the
    prime meridian can't be told from its mirror east. Then the position and its companions are
    kept as written, with a note saying how the checks would read them. That lets someone who knows
    the site sign them. Each of the 19 files with a fix writes its longitudes negative, west, and
    is read.
  - *Kept as written:* the temperatures, which read as a barometer's in degrees Celsius or
    Fahrenheit alike. So are the battery voltages, the magnetometers, and the raw counts the scaled
    columns are made from. The state codes, in three numberings no source publishes, are kept as
    numbers, so the record takes no events from them.
- **AerospaceNU's ground station text, each unit by two checks.** The same board sends its
  readings by radio. The club's ground station logs each packet as a line, under a `RUN START`
  line. The line holds the time of day it arrived, the packet's name and its fields as a Python
  dictionary. An invented line, shortened:

  ```text
  10:00:01.512345: Position Data {'crc': 'Good', 'time_stamp_ms': 61000, 'fcb_altitude': 0.25, …}
  ```

  No document describes the packets' units. So each unit a name doesn't state is read only where
  two checks on the file's own values agree. Neither check rests on a column whose unit is as
  unknown. Otherwise the unit is kept as written, with a note. The club's per-flight summaries
  aren't a check: they exist for few flights and needn't match the packets received. In the 4
  sample files:
  - *Altitude, `fcb_altitude`, the record's altitude:* its climb within 25% of the barometer's
    climb in the standard atmosphere, where feet would be 3.28 times off. A climb further off
    refuses the file. The second check is its coast, from the end of the boost to its highest
    value. The coast must slow within a factor of 1.8 of 1 g, where feet would read 3.28 g or more.
    Read in meters in all 4: climbs 1.006 to 1.090 times the barometer's, coasts slowing 1.08 to
    1.28 g.
  - *Vertical speed, `v_speed`:* its integral over the barometer's ascent within 25% of the
    barometer's climb. The speed is taken as a straight line across gaps in the reception. The
    second check is its coast, from its highest value to its first at or below 0. The coast must
    slow within a factor of 1.8 of 1 g. Neither check uses `fcb_altitude`. In m/s in all 4: the
    climb 0.909 to 1.004 times the integral, coasts slowing 1.00 to 1.47 g.
  - *Accelerometer:* 1 g at rest on the pad, in m/s² rather than g. The second check starts from
    the launch. Its force along the pad's direction less 1 g, integrated to a climb, is compared
    with the barometer's climb. It must be within a factor of 1.8 (the median 0.997 to 1.090). Read
    so in 3; kept as written in 1, which reads 0 throughout.
  - *Pressures, in atmospheres:* by their size and their whole-pascal step, as in the board's CSV.
  - *Rates of turn, in rad/s:* by the board's turning against the accelerometer's, and by its
    tilt from vertical. The tilt can change no faster than the board turns. Read so in 3; kept as
    written in 1, whose accelerometer isn't read.
  - *Positions, in degrees:* most fixes pass $\pm\pi/2$ or $\pm\pi$, which radians can't. Most have
    a fraction of a degree of 0.6 or more, which degrees and minutes run together can't. One stray
    fix decides nothing.
  - *Clock:* the board's `time_stamp_ms`, in milliseconds by its name, must advance 0.8 to
    1.25 s for each second of the ground station's own clock. This must hold over at least 20 s.
    Packets after the board restarts are placed by the ground station's clock.
  - *Left out or kept as written:* a packet whose radio checksum failed (its `crc` not `'Good'`)
    is left out and counted. That is because its bytes can't be trusted. The temperatures, the GPS
    altitudes, the battery voltage and the roll, pitch and yaw are kept as written. Only the
    barometer could check the GPS altitudes, and that is one check.
- **A phone app's `launch` CSV, each unit by two checks.** An app that no file names logs a
  phone's GPS, accelerometer, gyroscope, magnetometer and, in newer files, barometer. It logs about
  20 rows a second (from the 27 sample files' timestamps). No description of the format was found.
  Of the 27 sample files, 19 are an older layout of 35 columns with no barometer. The other 8 are a
  newer layout of 67 or 69 columns that adds a barometer. `Seconds` is the clock, by its name.
  Every other column is read in a unit only where two checks agree. The checks must read different
  numbers. The checks can compare two columns, a column against another of settled unit or
  direction, or a column's extent against its steps. Otherwise the column is kept as written, with
  a note. An invented start of a file, shortened:

  ```text
  Seconds,Timestamp,AltitudeMSLGPS,Record,Static,Latitude,Longitude,GTotal,Gx,Gy,Gz,…
  0.000,1941868800000,100,1,12,40.123456,-100.234567,1.000000,0.000000,0.932039,0.362358,…
  ```

  - *Pressure, `hPa`:* it names its unit, and its first reading must be a launch site's, 500 to
    1,100 hPa.
  - *Barometric altitude, the record's altitude, in meters:* it must be the pressure altitude of
    `hPa` in the standard atmosphere. In feet it would be 3.28 times as large. And in the
    coast before apogee it must slow at 0.7 g to $0.7 \times 3.28\,\mathrm{g}$. That is because gravity and
    drag slow a coasting rocket by at least 1 g, which an altitude written in feet can't. Read so
    in 7 of the 8 newer files. The barometric speeds are read in m/s by the altitude's rate and the
    same coast.
  - *Accelerations, in g:* the pad reads 1, and the boost's integral matches the barometric climb
    rate. The older files have no barometer for the second check, so their accelerations are
    kept as written. Over all the files, the unit is read in g in 6 files and kept as written in 21.
  - *Attitude, in radians:* each angle by its range (within $\pm\pi$, the pitch $\pm\pi/2$). The
    second check is its direction against a sensor that needs no unit, on rows where the phone is
    still. That compares the pitch and roll against gravity's direction in the accelerometer, and
    the yaw against magnetic north in the magnetometer. Android's
    `SensorManager.getOrientation` defines the three that way. Read so: the pitch in 25 files, the
    roll in 24, the yaw in 10 (the 8 newer). In most older files the yaw doesn't follow the
    magnetometer, so it is kept as written.
  - *Rates of turn, in rad/s:* the gyroscope by the attitude's turn, where all three angles are
    read. A second check is a full scale a phone's gyroscope offers. Read so in 7 files, kept as
    written in 20.
  - *Heading, in degrees:* by its range (0 to 360, some reading past $2\pi$) and by its period.
    Crossing north it jumps by 360 less its turn, where gradians would jump by 400 less it.
  - *Position, in degrees:* the latitude and longitude together, both or neither. They are decided
    by the latitude's largest reading (past $\pi/2$, within 90) and the longitude's (past $\pi$,
    within 180). That rules out radians, scaled integers and NMEA's degrees and minutes run
    together, the last for any site on land. No range can tell degrees from gradians (0.9°).
    Degrees are taken as phones' location services give them.
  - *Time stamp:* a Unix time in milliseconds, its steps the clock's and its date in this century.
  - *GPS altitude, kept as written in all 27:* only one check reaches it, its gain against the
    pressure altitude. Fixes once a second can't show the coast. On the pad, the weather on record
    moves a pressure altitude across a band 1,840 m (6,037 ft) wide. A GPS reading $N$ read in
    meters and in feet differs by $0.695\,N$ m. So the pad separates the two readings only where
    $N$ tops about 2,650. That means a pad above about 2,650 m (8,694 ft) if the column is in
    meters, or 810 m (2,657 ft) if in feet. No sample file's pad is that high
    ([evidence row G8](../evidence.md#g8)).
- **An Eggtimer's download saved as RTF, unwrapped.** One sample file is an Eggtimer TRS's
  terminal download that a text editor saved as an RTF document (Rich Text Format, a word
  processor's markup). The reader takes the document's plain text. It follows the parts of the RTF
  specification (Microsoft, version 1.9.1, 2008) a saved capture needs: groups, control words and
  escaped characters. Fonts, colors and pictures are skipped. The reader reads that text as it
  reads a terminal download (above). A document holding several downloads gives the first; the rest
  are counted in a note. In the sample file, the table's clock is in milliseconds and its heights
  in feet, each by two checks, the coast among them (above). Its speeds, which don't follow the
  altitude's change, are kept as written. So are the summary's apogee and site elevation, which
  carry no unit mark the reader knows. An RTF document holding anything else is refused as not an
  Eggtimer's. The radio packets in two other sample files are read by
  [the packets' reader](#one-teams-radio-telemetry-packets), not this one.

Where one of these layouts holds a barometric altitude or a pressure, the readings come from it,
as from every logger's. A file with neither gives none
([Flight-log readings](../physics/log-readings.md)).

A unit read in the wrong one of feet and meters would put every height 3.28 times too high or too
low. So the RRC3's is never assumed where its pressure can settle it. And the Eggtimer's rests on
the manuals.

### A worked example

PerfectFlite's CSV export of a flight starts like this. These are the first rows of an invented
flight, `perfectflite-export.csv`, which the crate's tests read and check these numbers against:

```text
Time,Altitude,Velocity,Temperature (F),Voltage,Acceleration (calculated)
0.00,0,0,71.5,9.1,-3
0.50,0,0,71.5,9.1,-3
1.00,0,88,71.4,9.1,1
```

It reads into a record. The record's primary track has the times 0, 0.5 and 1 s and the altitudes
0, 0 and 0 m (0 ft). The record also has five channels on that clock:

- `Altitude`
- `Velocity`, whose 88 at 1 s reads as 26.82 m/s (88.0 ft/s)
- `Temperature (F)`, whose 71.5 reads as 295.1 K (71.5 °F)
- `Voltage`
- `Acceleration (calculated)`, which the export doesn't explain and is kept as written,
  unrecognized

### Featherweight's summaries

A Featherweight summary is the logger's own account of a flight, a figure a row, with no samples.
The Featherweight app exports one beside a Blue Raven's or a GPS tracker's samples. The Blue Raven
"records three sets of data for each flight". The first is "flight summary data such as maximum
altitude and speed" (*Blue Raven User's Guide*, updated May 12, 2026, p18). Each row is a
figure's name, then its value, with the unit written after the number:

```text
Rocket Name,Club demo
Max Altitude,2500.25 feet
Max velocity,500.50 feet/sec
Thrust tilt efficiency,98.50%
Direction of 1st motion,-1.0000,0.0000,0.0000
```

> **How far to trust it.** These are the logger's own figures, copied as the file states them, not
> HPR Sim's readings. Two are checked: the maximum altitude and the pad's altitude. Each is
> checked against the file of samples beside the summary. Of the 33 Blue Raven summaries with such
> a file, 32 agree in both ([checked against the files beside](#checked-against-the-files-beside)).
> The speeds, accelerations, times and the rest are converted from the unit the file writes. They
> are checked against nothing ([evidence row G7](../evidence.md#g7)). Where a number matters, set
> it beside the samples the app exported with it.

**How it is recognized.** A file is a summary when two things hold. Its first line that isn't
blank names `Rocket Name`. And a line among its first 200 names `Max Altitude` or
`Launch time UTC`, figures only a summary states. A tracker's summary is told from a Blue Raven's
by its `Launch time UTC`. A name is matched without regard to case or repeated spaces, and kept
as written.

**What the record holds.** A summary fills the part of the [record](#the-record) that says what
the file states about the flight, and nothing else:

- **Every row is a figure** in the record's `stated.figures`, in the file's order. Each holds its
  name and value as written, its quantity, and its value in SI where the reader knows the figure.
- **The stated apogee** is `Max Altitude` in SI. For a Blue Raven that is the barometer's altitude
  above the pad. For a tracker it is its receiver's altitude above the pad altitude it states.
- **The pad's elevation** is a Blue Raven's `Pad altitude ASL`, its barometric altitude of the pad,
  or a tracker's `Pad altitude`, its receiver's.
- **The serial number and firmware** are the rows' text as written, and the flight number is
  `Flight number` where it is a whole number.
- **The clock, the tracks and the channels are empty.** A note says the samples are in the file
  exported beside it. That file is the Blue Raven's low-rate CSV or the tracker's CSV. So every
  reading withholds on a summary.

**A unit is read only where the figure's name allows it**, as a header's unit is read for a
column. Each figure the reader knows names its quantity and the words the files write for its
unit. The main ones are these. The [reader's reference](../api/hpr_flightdata/fwsummary/index.html)
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
below. The USB form also scales some figures (accelerations in g × 10). That scaling isn't
compared. These two are kept as written in every file, never guessed:

- **`Main chute descent rate`**: the app writes its value in `feet`, a length. The manual, though,
  gives the USB summary's average descent under the main in "feet/second x 10". A figure whose
  unit isn't one its name allows is kept as written, with a note, and isn't made into a speed.
- **The four pad accelerations and offsets**: each three numbers with no unit. The manual gives the
  USB summary's in "units of Gs x 10,000". The app's are not scaled so, and no source states
  what they are in.

A name the reader doesn't know is kept as written too. So is a known figure whose cells aren't
the numbers it needs, such as a direction with two components. Each of these has a note. A zero
is kept as the file states it. What a zero means is a reading, which HPR Sim doesn't take from a
summary. The [survey's report](#how-it-was-checked) counts each figure kept as written by the
catalog's family. The main's descent rate, for one, is kept in 23 files filed as
`blueraven-summary-csv` and 13 filed as `featherweight-summary-csv`.

**Cells and rows.** Cells are separated by commas and trimmed of spaces. Empty cells at a row's
end are dropped, as some files pad every row to four cells. Blank lines are skipped. Lines may
end in CR, LF or both. A byte-order mark is dropped. A row whose name is empty or a number is
refused. So is a name stated twice, and so is a value too large for its unit's conversion.

**A worked example**, from the invented Blue Raven summary that the crate's test
`tests/featherweight_summary.rs` reads and checks:

- `Max Altitude,2500.25 feet` gives the stated apogee, $2500.25 \times 0.3048 = 762.08\,\mathrm{m}$.
- `Max velocity,500.50 feet/sec` is a speed of 152.55 m/s (500.50 ft/s).
- `Max motor burn acceleration,12.0 Gs` is $12.0 \times 9.806\,65 = 117.68\,\mathrm{m/s^2}$.
- `Temperature on pad,77.0 F` is 298.15 K (77.0 °F).
- `Thrust tilt efficiency,98.50%` is the fraction 0.985.
- `Main chute descent rate,-20.0 feet` stays −20.0 as written, with the note
  `` `Main chute descent rate` kept as written, in a unit that isn't a speed: the file writes `-20.0 feet` ``.

### Checked against the files beside

A summary and the samples exported beside it describe the same flight. So the survey checks the two
lengths a summary states against them. `cargo xtask logs` takes each file of samples in the
summary's folder that its reader reads and that holds an altitude. That means a Blue Raven's
low-rate CSV, not its high-rate one, or a tracker's CSV. The command compares the summary's maximum
altitude with the file's highest altitude above the pad. For a tracker, that is its highest GNSS
altitude less the pad altitude the summary states. It compares the pad altitude with the median of
the file's first 20 altitudes above sea level. A figure agrees within 10%. When several files sit
beside a summary, the ratio, summary over file, nearest 1 is the one counted.

Feet and meters are 3.28 apart, so a band of 10% tells them apart. A summary that would agree
only were its feet meters contradicts its unit. `cargo xtask logs --check` fails on it. A
folder may hold other flights' files, so agreeing is counted, not required. The
[survey's report](#how-it-was-checked) prints the counts. It names a file by the survey's id, the
first 16 hex digits of its SHA-256:

| Logger | Summaries | Summaries with a file beside | Summaries with a file beside and an apogee stated | Apogee agrees | Apogee ratio, range | Pad agrees | Pad ratio, range | Agreeing in neither | Contradicted |
|---|---:|---:|---:|---:|---:|---:|---:|---|---|
| Featherweight Blue Raven | 36 | 33 | 33 | 32 | 0.9698 to 1.0091 | 32 | 0.9770 to 1.0032 | `cd1bfe19ff70e17d` | none |
| Featherweight GPS tracker | 12 | 11 | 5 | 5 | 0.9995 to 1.0000 | 10 | 0.9766 to 0.9999 | `413de085a44f4872` | none |

No summary agrees only as meters. Of the tracker summaries, 5 of the 5 that state an apogee
agree in it. The one of each logger that agrees in neither figure agrees with no file beside it,
not even as meters. The survey doesn't show why. Nor is it measured why a Blue Raven's stated
apogee runs up to 3% below its low-rate file's highest altitude
([ADR-260, the summaries' check](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0260-featherweight-summaries-before-the-rest.md)
§4). The summaries with no file of samples beside them are read by the same rules, unchecked
([evidence row G7](../evidence.md#g7)). No
other figure is checked. Summaries inside zip archives are read and checked as the others are,
against the files of samples in the same archive ([zip archives](#zip-archives)). The two inside
Excel workbooks are read by the same rules, unchecked ([Excel workbooks](#excel-workbooks)).

## GPS logs

A GPS tracker or a flight computer's GPS receiver logs where the rocket was. A log holds a
latitude and longitude in degrees and an altitude above sea level. It also holds figures of the
*fix* (the receiver's solution of its position), such as how many satellites it used. These logs
have no barometer. So a GPS log's record has an empty [primary track](#the-record) and keeps
everything as channels, with a note. The [readings](../physics/log-readings.md#the-height-track)
withhold on it, each by name. A receiver's altitude lags a climb and overshoots its top, and its
fix drops out at liftoff. So no bound can be stated for it.

> **How far to trust it.** Positions, heights and times are read as the file writes them. HPR
> Sim claims no accuracy for a GPS fix. A GPS altitude is above sea level, not above the
> pad. The clocks are the files' own, and some don't state their time zone. Heights from KML files
> are the least sure ([evidence row G8](../evidence.md#g8)). One writer's are feet written as
> meters, kept as written. Two files rise less than half as high as the barometer on the same
> flight did ([units no source states](#units-no-source-states)). The RTx's altitudes can't go below
> 0, so near sea level many read exactly 0 ([Missile Works' RTx](#missile-works-rtx)).

GPX is read from invented files only, as no sample log is one. Its elevations state no datum
([GPX](#gpx)) ([evidence row G1](../evidence.md#g1)).

Five layouts are read. Each point's time is the one the file gives it. Where the file gives none,
the point keeps its place with its time a gap.

- **Featherweight's GPS CSV**, from Featherweight's GPS tracker.
  - *Clocks.* The tracker's own log times each fix by `UTCTIME`, such as
    `Jul 14 2031 17:04:31.250 UTC` (UTC is Coordinated Universal Time, the time zone GPS receivers
    keep). Its `UNIXTIME` runs a whole number of hours, 4 to 8, ahead of `UTCTIME`. That holds on
    every row of all 28 tracker logs among the sample logs. It is as if the site's offset from UTC
    had been added where it should have been taken away. So that time is kept as written with a note
    of its offset. In the older layout rows step back in time and repeat. They are kept in the
    file's order and counted in a note. The ground station's log times each row by `DATE` and
    `TIME`, in a zone the file doesn't state (in most sample files, the launch site's). That log may
    hold several trackers. Each gets its own channels, such as `LAT [Rocket-1]`.
  - *Units.* No header names one. The manual gives the altitude in feet, the speeds in feet per
    second and the heading in degrees (*Featherweight GPS Tracker User's Manual*, February 20, 2025,
    p17). The reader checks each file. The horizontal speed against the distance between
    fixes is the check that tells feet from meters. That works since the positions are in degrees
    whatever the unit. Of the 40 sample files, 37 move enough for it, and in all 37 it agrees with
    the manual. In the other 3 the manual alone decides, and the note says so. Where it disagreed,
    every length and speed would be kept as written. The vertical speed against the change of
    altitude shows only that the two share a unit.
  - *No fix.* A position of exactly (0, 0), which the tracker writes before it has a fix, is
    read as a gap.
- **KML** (Keyhole Markup Language), the XML file Google Earth opens, standardized as OGC
  07-147r2.
  - *Tracks.* A track is a path of longitude, latitude and altitude, in meters by the standard.
    The track's `altitudeMode` says what its altitude is. It is above sea level for `absolute`.
    It is a height above the ground for `relativeToGround`. It is nothing a viewer uses for
    `clampToGround`, whose altitudes are kept as written. An AltOS file can hold the barometer's
    altitude in the path and the GPS receiver's beside each point. The reader keeps each as what
    it is.
  - *Times.* Only Google's `gx:Track` pairs each point with a time (`when`). AltOS, Big Red
    Bee's BeeLine and two unnamed files styled after AltOS's write each point's time in an XML
    comment beside it instead. The reader takes that time. The rest give no time. They are
    fw2kml's paths (from a converter of Featherweight's logs), Missile Works' RTx and GPS
    Visualizer. The survey reads 43 sample KML logs. When 42 were read, 9 timed their points and
    33 read as paths without times. The survey doesn't count them yet
    ([#929](https://github.com/nrdptel/fusionspace-eridanus/issues/929)).
- **NMEA 0183**, the sentences a GPS receiver sends, in the format the National Marine
  Electronics Association set. Each is one line and ends in a *checksum*, two hexadecimal digits
  that check the characters before them.
  - *Sentences.* The reader takes four kinds. `GGA` holds the position, altitude and fix. `RMC`
    holds the position, speed, course and date. `GSA` holds the satellites used and the
    precision. `GSV` holds each satellite in view. A sentence counts only if its checksum checks
    out. Noise before it, as a receiver writes on power up, is left out and counted.
  - *Clock and units.* The time of day is UTC. The date comes only from an `RMC` with a valid
    fix, because before its fix the receiver writes a placeholder date. A speed in knots is read
    as 1,852/3,600 m/s a knot.
- **Missile Works' RTx**, a GPS tracker: the `.rtx` file its maker's program saves, and the CSV
  and text files it exports. They have been read since
  [M7.1c12](../decisions-and-roadmap.md#m7-1c12) ([below](#missile-works-rtx)).
- **GPX**, the GPS Exchange Format. It has been read since
  [M7.1d1](../decisions-and-roadmap.md#m7-1d1), from invented files only ([below](#gpx))
  ([evidence row G1](../evidence.md#g1)).

For example, take a `GGA` sentence's position and an `RMC` sentence's speed, invented after the
textbook example public descriptions of NMEA 0183 give. A test in the crate reads these two lines
and checks every number below:

```text
$GPGGA,123519.000,4807.0380,N,01131.0000,E,1,08,0.9,545.4,M,46.9,M,,0000*59
$GPRMC,123519.000,A,4807.0380,N,01131.0000,E,022.4,084.4,230394,,,A*62
```

The latitude `4807.0380,N` is 48 degrees and 7.038 minutes north, $48 + 7.038/60 = 48.1173$°. The
longitude `01131.0000,E` is 11 degrees and 31 minutes east, 11.5167°. The altitude is 545.4 m
(1,789 ft) above mean sea level, in the unit the sentence names (`M`). The speed of 22.4 knots
reads as $22.4 \times 1852/3600 = 11.52\,\mathrm{m/s}$ (37.8 ft/s). The course is 84.4° from true north. The
record's zero is the first sentence's time, 12:35:19 UTC on 23 March 1994.

### Missile Works' RTx

Missile Works' RTx/GPS is a GPS tracker built around a u-blox 7 receiver. It keeps its fixes once
a second, up to "~68 minutes per flight". It radios them to a base station (*RTx/GPS User
Manual*, firmware 1.31, Missile Works; November 26, 2016; pp2, 14). Missile Works' mDACS program saves a
download as an `.rtx` file. The program exports that file to CSV, text and KML (*mDACS User
Manual*, revision 1.71, May 2024, p16). The KML is read as [above](#gps-logs). The `.rtx` and the
CSV and text exports are read by `hpr_flightdata::rtx`. Neither manual describes the files. What
follows was worked out from two sets of files. They are the 5 `.rtx` files among the sample logs
and the 13 exports beside them (8 CSV and text, 5 KML). The sample logs hold one more CSV
export, beside no `.rtx`. No other program's source was read
([ADR-267, reading the RTx before the other layouts](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0267-missile-works-rtx-before-the-rest.md)).
Like every GPS log here, an RTx file feeds no reading. That is because the readings need a
barometer's track (see [GPS logs](#gps-logs)).

> **How far to trust it.** In 4 of the 5 sample `.rtx` files, every field the reader converts
> agrees with mDACS's own exports. The agreement is on every fix they hold (all but the last),
> with positions written in degrees to their sixth decimal. The fifth file has only a KML beside
> it, which holds no times. So there only the positions and altitudes are compared, and they agree.
> The velocity and the speed over ground stay in the file's own numbers, since no document gives
> their units. The fixes are the receiver's. HPR Sim claims no accuracy for them.

In 4 flights the tracker's climb is 0.98 to 1.15 times the highest altitude of the barometric altimeter
flown beside it. [The survey](#how-it-was-checked) reports that figure but doesn't check it.

- *The `.rtx`.* A stream written by .NET's built-in object serializer, `BinaryFormatter`. It holds
  one list of 65,536 32-bit integers, in the layout Microsoft publishes as MS-NRBF. That is 4,096
  records of 16 *slots*, 68.3 minutes at one a second, the manual's flight. The list's size counts
  the slots in use. The rest are 0.
- *The slots.* Each holds a 16-bit number, a *word*, but not as written. The slot holds the word
  times 387,420,489 ($3^{18}$), keeping only the remainder after dividing by $10^9$. Multiplying a
  slot by 513,180,409 and keeping the same remainder gives the word back. That is because
  $387\,420\,489 \times 513\,180\,409$ leaves a remainder of 1. The files show it three ways.
  First, the slot holding the time steps by exactly 387,420,489, less any whole $10^9$, from one
  fix to the next a second later. Second, every one of the 264,336 slots in use in the sample
  files reads back as a word below 65,536. A slot drawn at random from 0 to $10^9 - 1$ would do
  that about once in 15,000. Third, the words equal the exports' figures, field by field.
- *The words.* Each record is one fix:

  | Word number (from 0) | Field | Read as |
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
  last, in the same order. The time is `HHMMSS`. A position is `DDMM.mmmmm` and its hemisphere's
  letter. In 5 of the 9 sample exports it is instead in signed degrees cut toward zero at the
  sixth decimal, with a degree sign. The altitude is in whole feet, the meters × 3.280 84 rounded
  down, which the reader converts back to meters. The rest is as the `.rtx` holds it. The text
  file's header names two empty columns its rows don't have.
- *Units.*
  - *Altitude, meters.* The manual says the GPS altitude "is always expressed as Mean Sea Level"
    (p13). The KML export beside each sample file writes the word as meters. The CSV and text
    exports convert it to feet. All three come from mDACS reading the same word. So the survey
    adds one more check, reported but not enforced. The check is that the climb from the pad to
    the highest fix agrees with the barometric altimeter flown beside the tracker, as above.
    Feet would be 3.3 times off. The fifth file's list is full and holds a climb of 0.003 times
    the altimeter's. Its memory filled before the flight.
  - *Altitude at 0.* The file can't hold an altitude below 0. So the receiver's altitude looks
    clamped there. In 4 of the 5 sample files, from pads near sea level, most fixes read exactly
    0. A note counts them. Each means sea level or below.
  - *Course, degrees.* The course lies 1.7° to 6.9° from the bearing between the fixes either
    side of it. That is the median difference either way in each file. It is taken over the fixes
    whose speed over ground reads at least 15 in the file's own numbers.
  - *Speed over ground and velocity, as written.* The speed over ground runs 3.5 to 4.0 times
    the speed between fixes in m/s, by file. That is what km/h would give, and km/h is one of the
    two speed units the tracker's display offers (p12). But nothing says which unit the file
    holds, and feet a second would give 3.3. The velocity reads far more than the climb rate in
    m/s while the rocket climbs. It reads about the fall rate itself while the rocket falls. No
    document explains that pattern. Plotted, both show when the tracker moved, but not how fast
    in any known unit.
- *The clock.* The GPS's time of day, UTC, in whole seconds, with no date. The log's zero is the
  first fix's. Each time takes the day that puts it nearest the one before it. So the clock runs
  on past midnight UTC, and a repeat written across midnight goes back to the day before. In 3 of
  the 5 sample files a pair of fixes repeats the pair before it a second back. Every fix is kept
  in the file's order, with a note. In one file, one fix's time reads 65,282 × 65,536 more than
  it should, as a garbled radio packet's might. A time that isn't a time of day is a gap, counted
  in a note.
- *Gaps and refusals.* A coordinate out of range is a gap, as is a position of exactly (0, 0). Out
  of range means minutes past 59 or the decimals without their leading 1. A hemisphere byte other
  than its two letters counts too, as does a value beyond 90° or 180°. No sample file has an
  out-of-range coordinate or a (0, 0) position ([evidence row G2](../evidence.md#g2)). An `.rtx` that isn't the list's shape, or whose size or
  slots break the rules above, is refused, naming the byte. An export's row of other than ten cells,
  or a cell that isn't its field's, is refused, naming the line.

For example, the crate's tests read an invented `.rtx` and its exports. The first fix is at
17:59:50 UTC, `HHMMSS` `175950`. That is more than 16 bits hold, so the record splits it. Word 1 is
$\lfloor 175\,950 / 65\,536 \rfloor = 2$ and word 0 is $175\,950 - 2 \times 65\,536 = 44\,878$.
The file holds word 0 as the slot 656,705,342, the remainder of $44\,878 \times 387\,420\,489$
after dividing by $10^9$. The reader multiplies it by 513,180,409 and keeps the last nine digits,
`44878` again. The latitude's words are `3259`, `35698` and `1`. Then $1 \times 65\,536 + 35\,698 =
101\,234$, a leading 1 and the decimals `01234`. So the fix is 32° and 59.01234′ north,
$32 + 59.01234/60 = 32.983\,539$°. Its altitude word is 1,400 m (4,593 ft). The CSV export
writes the fix as one line

```text,wrap
175950,3259.01234N,10658.54321W,4593,0,2,122,9,12,40
```

its altitude $\lfloor 1400 \times 3.280\,84 \rfloor = 4593\,\mathrm{ft}$, read back as
$4593 \times 0.3048 = 1399.95\,\mathrm{m}$. The text export writes the longitude in degrees, `-106.975720°`.
The `.rtx` holds $-(106 + 58.54321/60)$, which is $-106.975\,720\,17$°. So the sixth decimal is
cut, not rounded.

### GPX

GPX, the GPS Exchange Format, is the XML file GPS receivers, phone apps and mapping tools save
tracks in. Topografix publishes its two versions as schemas,
[GPX 1.0](https://www.topografix.com/GPX/1/0/gpx.xsd) and
[GPX 1.1](https://www.topografix.com/GPX/1/1/gpx.xsd), with
[a manual for 1.0](https://www.topografix.com/gpx_manual.asp). HPR Sim reads both since
[M7.1d1](../decisions-and-roadmap.md#m7-1d1), by `hpr_flightdata::gpx`. The reader was written
from those documents.

> **How far to trust it.** No sample log in the collection is a GPX file. So the reader is checked
> only on three invented files, against gpxpy 1.6.2, an outside GPX reader ([evidence row
> G1](../evidence.md#g1)). All 212 values of their tracks' channels and all 12 figures of their
> waypoints and route points match gpxpy's as the same 64-bit numbers. A rocket flight's GPX with
> its barometric log would show what `ele` is measured from. Offer one in [issue
> #654](https://github.com/nrdptel/fusionspace-eridanus/issues/654).

**The check.** The test is `every_value_of_the_samples_equals_gpxpys` in
`crates/hpr-flightdata/tests/gpx.rs`. The gpxpy reading is written by
`validation/oracles/gpx/gpxpy_read.py`. The samples hold only what both readers read alike. gpxpy
reads a number with an exponent, a point off the globe and a time that goes back, which this
reader refuses. gpxpy cuts a time to the microsecond and refuses an `ele` of spaces only, which
this reader reads as a gap.

- *Tracks.* A file holds tracks (`trk`), each of segments (`trkseg`), each of points (`trkpt`).
  A receiver starts a new segment where it lost reception or was turned off. Each segment is
  read as a GPS track. Its channels are named after its track, such as `Flight 7 latitude`. They
  are numbered where the track has several segments, such as `Rocket segment 2 speed`. A point
  gives its latitude and longitude in degrees, its elevation in meters (`ele`) and its time.
  Where it states them, it also gives its satellite count and its dilutions of precision. These
  are horizontal, vertical and position (3-D) (`hdop`, `vdop`, `pdop`: how much the satellites'
  geometry spreads the fix). In GPX 1.0 it can also give its course in degrees from true north
  and its speed in meters per second. A value a point leaves out, or leaves empty, such as
  `<ele/>`, is a gap. The rest, such as `fix` and an app's own `extensions`, is counted in a
  note and not read.
- *Elevation.* GPX gives `ele` in meters but no [vertical datum](../glossary.md#vertical-datum).
  It is read as an altitude of unstated zero: only its rise between points means what it
  says. A receiver's [NMEA GGA](https://gpsd.gitlab.io/gpsd/NMEA.html) altitude is above mean
  sea level. [Android's](https://developer.android.com/reference/android/location/Location#getAltitude())
  location API gives the [ellipsoidal height](../glossary.md#ellipsoidal-height),
  [Apple's](https://developer.apple.com/documentation/corelocation/cllocation/altitude) the height
  above sea level. That offset barely changes over a flight: it moves the pad's elevation, not
  the height above the pad. A handheld may write its barometer's, whose error grows with
  height. A track point's `geoidheight`, the geoid's height above the ellipsoid, is
  read as a channel, `<name> geoid height`. It doesn't say which `ele` is:
  [GPSLogger](https://gpslogger.app/#whyisthealtitudevaluemissing), which wrote 7 of 10 such files
  opened on GitHub, writes the ellipsoidal height unless set to sea level
  ([#645](https://github.com/nrdptel/fusionspace-eridanus/issues/645)). One Eggfinder track saved
  as GPX most likely holds sea level, with moderate confidence ([a research
  note](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/forum-eggfinder-gpx-vs-baro.md);
  [evidence row G1](../evidence.md#g1)).
- *Times.* GPX times are in UTC, as GPX 1.1's schema and GPX 1.0's manual require. A time with
  an offset, such as `08:30:00-04:00`, is moved to UTC. One with no zone is read as UTC, with a
  note. A second written 60, a leap second's, is read as the next minute's first. The record's
  zero is the first time the file gives a track point (or, with none, a waypoint or route
  point). A point with no time keeps its place with its time a gap. A time that goes back
  within a track is refused, as such a track isn't one recording. Two tracks may run in any
  order, as a file holding two flights does.
- *Waypoints and routes.* A waypoint (`wpt`) marks a place, such as the pad, and a route (`rte`)
  plans a path. Neither was flown. They are kept apart from the tracks, as figures the file
  states, such as `waypoint Pad latitude`. So no reading takes them for the flight.
- *Refused.* A document type declaration is refused, and so is a reference to an entity XML
  doesn't define, such as `&rocket;`, so no entity, XML's named text, can expand. A point off
  the globe is refused, with a latitude outside −90° to 90° or a longitude outside −180° to
  180°. So is a value that isn't a plain decimal number, such as `1e3`, and a time that goes
  back. Each refusal names what it found and its line.
- *Limits.* At most 256 segments, 1,000,000 track points and 10,000 waypoints and route points
  are read. The rest are counted in a note and left out.

For example, take the first and the tenth point of the track in the invented sample
`gpx-1.1-phone-app.gpx`, shortened. A test in the crate reads the file and checks every number
below:

```text
<trkpt lat="35.2000120" lon="-97.1000340">
  <ele>402.0</ele><time>2031-07-14T17:04:04.000Z</time>
<trkpt lat="35.2001659" lon="-97.0999611">
  <ele>868.4</ele><time>2031-07-14T17:04:13.000Z</time>
```

The record's zero is the first point's time, 17:04:04 UTC on 14 July 2031. The tenth point is $13 -
4 = 9\,\mathrm{s}$ later and $868.4 - 402.0 = 466.4\,\mathrm{m}$ (1,530 ft) higher. It is the track's highest point. The
same file marks the pad as a waypoint at 17:03:40 UTC. That is read as the figure `waypoint Pad
time`, 24 s before the zero, −24 s.

## Other PerfectFlite files

Besides the `.pf2`, which has [a page of its own](pf2.md), PerfectFlite's software exports a
flight as CSV or text. An older PerfectFlite altimeter saves its flights as a `.pfa`.

### PerfectFlite's exports

PerfectFlite's software exports a flight as CSV, `Time,Altitude,Velocity,Temperature (F),Voltage`,
sometimes with `Acceleration (calculated)` after them. The software can also export tab-separated
text, `Time`, `Altitude`, `Temp.`, `Voltage`. The temperatures in the text carry their unit
letter, such as `58.7F`. One sample CSV export names its temperature `Temp`, without the dot, and
is read alike. Some text exports have no header row at all. Their rows are four tab-separated
numbers, the third a temperature with its unit letter, `F` or `C` (`0.05`, `3`, `71.5F`, `9.2`).
No other layout HPR Sim reads writes a unit's letter after a number in such a row. Among the
sample logs only PerfectFlite's text exports do. So such a row is read as the text export's four
columns. The record's notes say that its columns were taken, not read. That is a guess at which
column is which, made from the layout alone.

The units are a `.pf2`'s: seconds, feet and feet per second. The header doesn't state the
altitude's unit, and the sample logs bear out feet. Of the 21 exports among them, 16 peak at
exactly the height of the `.pf2` saved beside them, which states feet. Read in feet, those 16
climb in 0.94 to 1.05 times as long as a fall in vacuum from their peak would take. The other 5
climb in 0.63 to 1.07 times, read in feet. They would climb in 0.35 to 0.59 times read in meters
([the survey's report](#how-it-was-checked) prints these, and each of the 5 by id). Of the 4
headerless exports, 2 are one flight, exported once with its temperatures in °F and once in °C.
Both peak at exactly the height of the `.pf2` beside them. So the headerless layout writes feet,
whatever its temperature unit. A third climbs in 0.99 times a fall in vacuum read in feet, among
the matched exports' 0.94 to 1.05. Read in meters it climbs in 0.55 times. The `.pf2` files beside
it are other flights'. The fourth has no `.pf2` beside it, and its climb settles nothing. Its 0.63
read in feet and 0.35 in meters both fall outside the band of the matched exports. Its altitude is
ragged near its peak. Its feet rest on the layout's convention alone. Check its altitude against
another source before trusting it. An export doesn't name the
altimeter's model, so its logger reads `PerfectFlite`.

A header with any other name, or these names in another case, isn't read as PerfectFlite's. The
RRC3's `Time,Altitude,Pressure,…` is the RRC3's, and a bare `time,altitude` says nothing of its
units.

### PerfectFlite's older `.pfa`

The Alt15K/WD, an older PerfectFlite altimeter, saves its flights through PerfectFlite's DataCap
program as a `.pfa`. The file has a line naming the altimeter, then `Ground:`, `Apogee:`,
`NumSamps:` and `Comments:` lines. Then comes a blank line, a `**!!Data:` line, and one height a
line, such as `00274`. Most files end their lines in a carriage return alone, as old Macs did.
Some end them in CR LF. Both read.

> **How far to trust it.** The file states neither its rate nor its unit. Both come from
> PerfectFlite's documents, not the file. The 25 sample `.pfa`s agree with them (the survey's
> `pfa` row of unit decisions counts the note on each).

The file has no time column and names no unit. The altimeter's manual says it takes a sample
every 100 milliseconds (*Alt15K/WD User's Manual*, p. 2). The manual also says it reports heights
in feet (p. 6). The help of DataCap 1.10 for the Mac (AltCapWD) says 10 samples a second. So each
height is read in feet above the pad, 0.1 s after the one before. The 25 sample `.pfa`s bear it
out. In every one, `NumSamps:` is the number of heights and `Apogee:` the highest of them
([the survey's report](#how-it-was-checked) prints these checks). Each record says so in a note,
which the survey counts.

Two things in the file aren't what they look like:

- **Time zero isn't liftoff.** The altimeter notices a launch once the rocket is 80 ft up
  (manual, p. 2). A later revision lets the threshold be set from 40 to 250 ft (PerfectFlite's
  Alt15K specifications page). The altimeter saves its last 8 samples, the one that noticed among
  them (p. 2). DataCap's help says sampling "begins at a point 0.7 seconds before the launch is
  detected", seven steps of 0.1 s. So time zero is 0.7 s before the
  altimeter noticed the launch. The rocket left the pad before it noticed. A slow rocket may
  have left before time zero, so the file can miss the first moments of the climb. Every sample
  `.pfa` first reaches 40 or 80 ft at 0.7 s (17 at 80 ft, 8 at 40 ft, as the report prints).
- **`Ground:` isn't the pad's elevation.** It is the pad's pressure altitude. That is the height
  the day's air pressure would mean if sea-level pressure were the standard 29.92 inches of
  mercury (inHg). DataCap's help says it is about 100 ft off for every 0.1 inHg the day differs.
  The help gives it "for interest only". The record keeps it in a note, not as the ground elevation.

A worked example: take the invented sample `perfectflite-alt15k.pfa` (read by the test
`tests/perfectflite_pfa.rs`). Its eighth height, `00088`, is the sample at 0.7 s, the first at
80 ft or more. It reads as $88 \times 0.3048 = 26.82\,\mathrm{m}$ above the pad.

DataCap's help also says a `NumSamps:` of 2,200 (a revision 2 altimeter) or 2,400 (the original)
likely means the altimeter lost power in flight. It says older flights then follow this one in
the data. Such a file reads, with a note saying so. None of the sample files is one. A line among
the heights that isn't a whole number, or an empty line between them, is refused, naming the line.
That is because every line is a step of 0.1 s, so one left out would move every time after it.

## Raw downloads

A raw download is an altimeter's memory as its maker's program copies it to a computer. It is the
memory before that program turns it into an export with columns and units. HPR Sim reads six. The
Featherweight Raven's `.FIPa` and Missile Works' RRC3 `.rff` are described first. The other four
are AltOS's `.eeprom`, the ARTS2's `.odf`, the AltiMAX's `.ALS` and the Entacore AIM's raw dump.
The AIM's section covers the dump beside the `.xtra` file its software makes of it. Each of the
four has its own section below
([AltOS's `.eeprom`](#altoss-eeprom), [the ARTS2's raw data](#the-arts2s-raw-data),
[the AltiMAX's `.ALS`](#the-altimaxs-als),
[the AIM's `.xtra` and raw dump](#the-aims-xtra-and-raw-dump)).
Each holds the sensors' readings as whole numbers, *counts*. The AltiMAX's are in pascals instead.
None holds an altitude. The maker's own arithmetic turns the Raven's, the RRC3's, the AltiMAX's and
the AIM's counts into SI. The sensors' data sheets and the calibration the file carries do that for
AltOS's and the ARTS2's. A count with no source stays as written.

> **How far to trust it.** These are the logger's own readings, decoded from layouts no maker
> publishes. Each is checked against the maker's export, where one sits beside it.
>
> - **Raven `.FIPa`:** 55 of the 57 sample files; every sample of every shared channel agrees.
> - **RRC3 `.rff`:** 1 of the 6, and its 3,541 pressures agree to the step.
> - **AltOS `.eeprom`:** 9 of the 14 agree within half its print step; 1 more, with telemetry; 4 have neither.
> - **ARTS2 `.odf`:** all 3 agree; the rate rests on the export and a climb check.
> - **AltiMAX `.ALS`:** the one file agrees, some values once two rules fitted to it are applied.
> - **AIM raw dump and `.xtra`:** 435,075 values are equal bit for bit.

Not compared: the exports' altitudes (but the order of the ARTS2's), flags and the velocities they
work out. Those are the programs' conversions, not the altimeter's words. Where a number matters, set it
beside the maker's program's export of the same flight. The other 2 `.FIPa` files and 5 `.rff` files
are decoded by the same rules, unchecked by an export ([evidence row G6](../evidence.md#g6)). The
RRC3's pairs and single words have no calibration to check. The sections have the rest:

- [AltOS's `.eeprom`](#altoss-eeprom): the TeleMetrum v1's pressure assumes its converter's
  reference is the sensor's supply, which no document states. The TeleMega v4's IMU isn't read.
- [The ARTS2's raw data](#the-arts2s-raw-data): the exports' acceleration agrees within 0.080 of a
  count on every record after the first. The raw CSVs hold the same 48,614 records word for word.
- [The AltiMAX's `.ALS`](#the-altimaxs-als): pressures, event codes and voltage numbers equal the
  export's bit for bit; the rest match to 15 digits once the two rules are applied. The rules are shown consistent with the one file,
  not to hold for every file. The voltages' factors and the speed after apogee are the software's,
  not the file's.
- [The AIM's `.xtra` and raw dump](#the-aims-xtra-and-raw-dump): the dump's times run slow against
  the GPS's clock by 0.9 % in flight and 2.1 % on the pad. The `.xtra`'s times, which the software
  sums in 32-bit floats, run slow by about 1.8 % over the climb.

### The Raven's `.FIPa`

The Featherweight Interface Program (FIP) saves a `.FIPa` as a few text lines starting `#`, then
the memory. The text lines give the firmware, the flight's number, the settings and the length of
a tick of the Raven's clock. The memory comes in frames of a `:`, 256 bytes and a checksum byte.
A frame holds four blocks of 32 words of 16 bits. Each block covers 0.05 s, its words taking turns
among the channels:

- the axial accelerometer, 11 bits, 400 times a second;
- the lateral accelerometer, 10 bits, 200 times a second;
- ten slow channels, a word each, 20 times a second. They are the current draw, the four pyro
  channels' voltages, the battery's voltage, the temperature and the pressure. They also include
  the speed the Raven integrates from its accelerometer, and the sixth. The FIP doesn't export the
  sixth (the reader names it `slow channel 5`, counting from zero).

The rates follow from the blocks' layout. The FIP's export times its rows by them. Featherweight's
manual says the Raven records the data its deployment logic uses at 20 Hz
(*Raven User's Manual, updated for the Raven4*, August 15, 2020). The first five blocks are the quarter
second before the Raven detected the launch. The sixth is the settings block, which holds the
accelerometers' calibration and the ground's elevation. Time zero is the first sample before the
launch, as in the FIP's export.

Each word becomes the value the FIP's export prints, by the export's own arithmetic. Each value then
becomes SI, as the [Raven FIP CSV](#the-layouts-read) reader reads that column. The pressure word
counts 1/25,000 of an atmosphere. The voltages count thousandths of a volt, and the temperature
hundredths of a degree Fahrenheit. The speed counts tenths of a foot per second. The reader keeps
as written, as *unrecognized*, the sixth slow channel and the two words after each block's
samples. On a Raven whose header says its accelerometer has one axis (one sample file), the lateral
bits are kept the same way, as `lateral counts`. That is because that Raven calibrates no lateral
axis. A frame whose checksum fails is refused, naming its byte.

**A worked example**, with invented numbers. The crate's test `tests/raven_fipa.rs` reads and
checks it. The axial accelerometer reads in g as

$$
\frac{1}{100}\left[\operatorname{trunc}\left(\frac{\mathrm{X1}\,(a - \mathrm{Z1})}{1000}\right) + \mathrm{F3}\right],
$$

where $a$ is the count. The settings block gives the gain X1, the zero Z1, and the offset F3 in
hundredths of a g. The function trunc drops the fraction. Take a gain of 8,200, a zero of 1,000
counts and an offset of 5 hundredths. A count of 1,025 is 25 above the zero. The FIP works
$\mathrm{X1}/1000$ in double-precision floating point. There 8.2 is stored a hair below 8.2, so
$8.2 \times 25$ comes to 204.999… and truncates to 204. Adding the offset gives 209 hundredths:
2.09 g, or 20.50 m/s². Exact whole-number arithmetic would give 205 and 2.10 g. The reader works as
the FIP does.

### The RRC3's `.rff`

mDACS, Missile Works' program, saves an RRC3's download as a .NET serialized object. Microsoft
publishes that layout as MS-NRBF. The object holds one list of 16-bit words: the altimeter's memory
as it came off the board. The list has no columns and no times, so the reader tells the words apart
by their size. Debrief, the project owner's earlier analyzer, does the same:

- A word below `0x4000` (16,384) is a pressure, in tenths of a millibar. `0x2710`, 10,000, is
  1,000.0 mbar, or 100,000 Pa. `0x4000` would be 1,638.4 mbar. That is above the 1,200 mbar at
  which the RRC3 itself reports a pressure sensor fault (*RRC3+ / RRC3 User Manual* v1.71, p. 9).
  So no reading reaches it.
- A word at or above `0x4000` is an auxiliary word. Most come in pairs once a second. The pairs
  are presumably the temperature and battery voltage the RRC3 records once a second, in counts. A
  few stand alone (three in each sample file: one near apogee, one later, and a last `0xFFFF`). The
  first two fall where the manual's low-voltage samples, taken as the drogue and the main fire,
  would, but that is a guess. The file holds no calibration for either, so both are kept as
  written, as *unrecognized*, in channels of their own.
- A zero word ends the log. The reader looks among the first eight words for the first that can
  be a launch pad's pressure, at least 500 mbar. The words before it are kept in a note.

The pressures are on the RRC3's 20 Hz clock, from its manual (*RRC3+ / RRC3 User Manual* v1.71,
p. 23). The *n*th is at *n*/20 s. The file has no times to confirm that clock, so the reader checks
it another way. The once-a-second pairs must count the same seconds as the pressures do at 20 a
second, within 10% and two pairs. Otherwise the file is refused.

### What the record keeps

Neither file holds an altitude. The altitudes in the FIP's and mDACS's exports are those programs'
conversions of the pressure. So each record has an empty [primary track](#the-record), with a note.
A [GPS log](#gps-logs) and the Blue Raven's high-rate file have the same. The record's channels hold
everything, the pressure among them. The [readings](../physics/log-readings.md#the-height-track)
turn that pressure into a height by the 1976 standard atmosphere and zero it on the pad. The
Raven's settings block gives the ground's elevation, which becomes the launch site's. The Raven's
flight number and firmware are kept as stated.

### Checked against the makers' exports

A raw download and its maker's export of the same flight are two readings of the same bytes.
So one checks the other sample by sample. `cargo xtask logs` pairs each raw file with every export
in its folder that the export's own reader reads. It counts a match when every sample of every
channel the two share agrees. The value must agree within half a step of the raw format's
resolution, and the time as the export prints it. Every raw sample counts once. The FIP's export
prints a row every tick of the Raven's clock and holds each channel's latest sample in it. So every
raw sample must show at a row of its own time, none skipped. Only the velocity may stop before the
raw file's last sample, as the FIP stops it at apogee. The velocity's samples after the stop are
counted apart. `cargo xtask logs --check` fails on a raw file with an export beside it that matches
none. The [survey's report](#how-it-was-checked) prints the counts, in its paragraph that opens
"The raw downloads against their makers' exports":

| Layout | Files | Files with an export beside | Files matching | Raw samples compared | Velocity samples after the stop | Worst value, steps | Worst time, s |
|---|---:|---:|---:|---:|---:|---:|---:|
| Raven `.FIPa` | 57 | 55 | 55 | 6,016,104 | 140,865 | 0 | 0.0005 |
| RRC3 `.rff` | 6 | 1 | 1 | 3,541 | 0 | 0 | 0.01 |

The worst times are the exports' own rounding. The FIP's newer exports print a time to six
significant figures, 0.0005 s at most here. The mDACS program keeps its clock as 0.05 s added up
row by row in single precision. That clock runs a hundredth of a second ahead of the RRC3's 20 Hz
from 156.65 s. Worked that way, the survey reproduces all 3,541 of its export's times exactly.
Every value matches only with the FIP's own arithmetic. Suppose the accelerometer's gain used exact
whole-number arithmetic. Then 11 files would differ from their exports by a hundredth of a g, as in
the worked example above. The other 44 of the 55 would match
([ADR-259](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0259-raw-downloads-against-their-makers-exports-first.md),
a decision record, gives the probes). The one RRC3 export is mDACS's, in UTF-16
with a UTF-8 byte-order mark written a second time inside it. The importer's text decoding now
drops that mark, so the export reads as [RRC3 CSV](#the-layouts-read).

Two `.FIPa` files and five `.rff` files have no export beside them. They are decoded by the same
rules and pass the checks each file carries (every frame's checksum, the RRC3's clock check). But no
export checks their values.

### AltOS's `.eeprom`

HPR Sim reads the log an Altus Metrum flight computer keeps in its own memory. It reads that log as
AltosUI, Altus Metrum's ground software, downloads it and saves it to an `.eeprom` file. So far
only the Rust library reads it. Altus Metrum publishes no layout for these records. So the reader's
layout was worked out from the 14 sample files and the AltosUI exports (CSV) of the same flights
beside 9 of them. That work was done without AltOS's or AltosUI's source code. Each unit the reader
converts rests on a sensor's data sheet and a second check. A count with no source is kept as
written.

> **How far to trust it.** These are the board's own readings, decoded from a layout the maker
> doesn't publish, checked against what lies beside each file.
>
> - **Checked:** 9 of the 14 files against AltosUI's export, within half the
>   export's print step on every value converted to SI. 2 against the board's radio telemetry, every
>   pressure equal at a shared tick (1 also has an export). 11 of 12 accelerometer boards' files
>   pass the climb check.
> - **Assumed:** that the TeleMetrum v1's converter measures against the pressure sensor's own
>   supply, which no document states ([evidence row G4](../evidence.md#g4)).
> - **Not read:** the TeleMega v4's IMU, unused in its one sample file.
> - **Unchecked:** 4 files, with no export or telemetry beside them
>   ([evidence row G4](../evidence.md#g4)).

More on the note's items:

- **Checked.** The telemetry pressures are compared at a shared tick. Every row of the export is at
  a tick the file holds. The climb check's 11 passes have ratios from 0.859 to 1.400, where the
  reader accepts 0.6 to 5/3. The twelfth, at 1.988, keeps its acceleration and pressure as counts.
- **Assumed.** The exports of the TeleMetrum v1's flights agree.
- **Unchecked.** Those 4 files are decoded by the same rules and pass the reader's own checks. The
  14 files are 9 with an export, 1 with telemetry alone, and those 4
  ([evidence row G4](../evidence.md#g4)).

The board logs its sensors 100 times a second in the climb and 10 times a second in the descent.
Those rates are in *The Altus Metrum System*, Bdale Garbee and Keith Packard, v1.9.22.2, 2025,
Appendix F, Table 11, p. 77. Each record is a type letter, a checksum byte, the board's 16-bit tick
and the type's fields. `F` is the flight (its number, the accelerometer's and the barometer's
readings on the pad). `A` holds the sensors. `S` is a change of state. A board with GPS also has
records of its fix and its satellites. The record size follows the log format the configuration
states:

| Log format number | Record size | Boards |
|---:|---:|---|
| 1 | 8 bytes | TeleMetrum v1 |
| 6 | 16 bytes | EasyMini v1 |
| 7 | 16 bytes | TeleMetrum v2 and newer |
| 14 | 16 bytes | EasyMini v2 |
| 15 | 32 bytes | TeleMega v4 |
| 16 | 32 bytes | EasyMega v2 |
| 22 | 32 bytes | TeleMega v6 |

Any other number refuses the file, as misaligned bytes would read as a plausible flight. Firmware
before 1.0 states no log format. A TeleMetrum v1's file is then format 1, as the manual's release
notes for version 1.4 say AltosUI does (§H.43.2, p. 128). Any other product's file is then refused.
The survey's report marks the files whose format came from the product. A file comes in one of two
forms:

- **JSON**, as AltosUI writes today: the board's configuration as a JSON object naming
  `log_format` and `product`, then the memory's bytes in hexadecimal. Every record's bytes sum,
  with 0x5A, to 0 modulo 256. That 0x5A is the seed of the line checksum of the TeleDongle, the
  radio receiver AltosUI listens through (*AltOS Telemetry*, v1.9.22.2, §4, p. 10). A record that
  fails refuses the file. All 29,437 records of the 7 JSON sample files pass. Erased memory, every
  byte 0xFF, ends the log and is counted in a note. Memory that ends partway through a record is
  refused.
- **Text**, as older AltosUI and a serial console wrote. It has the configuration as the board
  prints it (`Config version:`, `manufacturer altusmetrum.org` and the rest), then one record a
  line. Each record's checksum is left out. A last line with no line break, as a download cut short
  leaves, is left out with a note.

What the reader does with the records:

- **The clock** is the board's tick, a hundredth of a second (*AltOS Companion Port*, v1.9.22.2,
  Table 1). The tick turns over every 655.36 s, and each record goes on the turn nearest the latest
  tick so far. The records aren't written in time order. The pad's samples, held in a ring buffer,
  come after the flight record, and a GPS record carries its fix's earlier tick. So the records are
  sorted by tick, the file's order kept among equal ticks. Time zero is the flight record's tick,
  the board's detection of boost, as in AltosUI's exports. So the pad's samples have negative
  times. A record more than 10 s from the latest tick ends the flight. That record and those after
  it are counted in a note. One sample file ends with 14 records of another session, more than 10 s
  later. Records repeated exactly, the same bytes at the same tick, are dropped and counted. Old
  firmware wrote some. A file needs exactly one `F` record.
- **Pressure and temperature, MS5607** (the barometer chip on every board but the TeleMetrum v1).
  The data sheet compensates the chip's two counts, D1 and D2, by the six constants the
  configuration lists. It adds a second order below 20 °C (TE Connectivity, *MS5607-02BA03*,
  Figures 2 and 3, pp. 8–9). Each division by a power of 2 is a floor, as an arithmetic shift is.
  Truncating toward zero instead gives different pressures below 20 °C, and a unit test pins the
  floor. Second checks: the pad's pressure must lie from 10 to 120 kPa and within 1% of the flight
  record's. The pad's temperature must lie within the chip's operating range, −40 to 85 °C (p. 8).
  If any second check fails, the file is refused.
- **Pressure, TeleMetrum v1:** its MP3H6115A's output is `Vout = Vs (0.009 P − 0.095)`, P in kPa
  (Freescale, *MP3H6115A*, rev. 5.1, p. 5). The converter of the CC1111, the board's radio
  microcontroller, reads that output. The converter's single-ended result reaches 2,047 at its
  reference and sits in the top bits of its 16-bit result. That makes 2,047 × 16 = 32,752 at full
  scale (TI, *CC1110Fx/CC1111Fx*, SWRS033H, §12.10.2.6, p. 143, and p. 144). That the converter's
  reference is the sensor's own supply is the board's design, which no document states. The second
  checks: the pad's pressure must lie from 10 to 120 kPa, or the file is refused. The climb check
  below must pass, or the counts are kept as written, since that pressure band alone rejects almost
  no count. The exports agree too. The sample files' pressures step by 13.57 Pa (the survey's
  report prints each file's smallest step). That is about 1.13 m (3.7 ft) of height at sea level.
- **Acceleration**, along the board's axis. AltosUI calibrates the accelerometer antenna up, then
  down (the manual, §12.6.20, p. 42). The configuration keeps its counts at +1 g and −1 g. The
  record's value is `(ground − count) / ((minus − plus) / 2) × 9.806 65`&nbsp;m/s², with `ground` the
  flight record's count on the pad. It is 0 at rest on the pad, positive forward along the flight
  path. A board mounted antenna down logs flipped counts and keeps a flipped calibration with the
  same span. So either reads alike, as long as both counts come from one pair. The configuration's
  pad orientation is kept as a stated figure. The counts are kept as written when there is no
  calibration or the calibration is outside a 16-bit count's range. They are also kept as written
  when there is no pressure in pascals to check against, or when the climb check below fails.
- **The climb check** holds the pressure and the acceleration to each other on every board with
  an accelerometer. It looks at the first second after boost. The climb by the pressure is its
  pressure altitude in the U.S. Standard Atmosphere, 1976, which is ISO 2533's below 32 km (20 mi).
  That climb must agree with the climb by the acceleration, integrated twice from rest on the pad.
  Their ratio must lie from 0.6 to 5/3. A ratio outside that band means neither unit is borne out.
  The same holds where the samples don't cover that second or leave a gap in it of more than 0.1 s.
  So the acceleration stays in counts, and so does a TeleMetrum v1's pressure, which has no other
  check. An MS5607 board's pressure keeps its flight record's check. On 11 of the 12 sample files
  with an accelerometer the ratio runs from 0.859 to 1.400. That is because the barometer reads
  the pressure at its ports, which the airflow past them can lower in the climb. The twelfth, an
  early TeleMetrum v1 flight with nothing beside it, reads 1.988. Its two sensors disagree by a
  factor of 2, and nothing in the file says which is wrong. So both stay in counts. The band
  catches swapped or misplaced fields and a wrong sign. It also catches a calibration off by 2 or
  more on a flight whose ratio is near 1. Without a calibration, an MS5607 board's pressure gets
  the pad's second checks alone. Then a TeleMetrum v1's pressure stays in counts.
- **A TeleMega v4.0 or v6.0's or an EasyMega v2.0's pyro battery, igniter, apogee and main
  senses** are voltages across the pyro channels that show continuity. They become volts by the
  divider *AltOS Telemetry* gives a 15 V board (p. 3). The exports agree. The same page gives
  another divider for 30 V boards. A later TeleMega takes 30 V (the manual's release notes for
  1.9.20), so any other Mega's senses are kept as written.
- **GPS** latitude and longitude are in ten-millionths of a degree and altitude in meters. That
  follows the telemetry's GPS packet (*AltOS Telemetry*, Table 15, pp. 6–7). A record without a
  valid fix is a gap.
- States become events, named as AltosUI's exports name the numbers: 2 `pad`, 3 `boost`,
  4 `fast`, 5 `coast`, 6 `drogue`, 7 `main`, 8 `landed`.
- **Kept as written:** the battery and other senses and the nine words of the IMU, the inertial
  unit's accelerometer, gyroscope and magnetometer. Their factors fit the exports but have no
  source. The TeleMetrum v1's chip temperature is also kept as written. That is because no document
  gives the conversion AltosUI uses, and the CC1111 data sheet's constants don't reproduce its
  export. The GPS's other fields are kept as written too. Not read: the TeleMega v4's IMU, and a
  Mega's IMU when the flight record's IMU fields are all zero.

The file holds no altitude. The record's [primary track](#the-record) is empty, as for the other
[raw downloads](#raw-downloads), and every field is a channel. The configuration gives the serial
number and firmware. The `F` record gives the flight's number. The record's stated figures hold the
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

The text form prints the same record as `F 164 03 00 19 00 d0 7e 01 00 00 00 00 00`. The tick is
one hexadecimal number, `164`, while the other bytes keep their order. Take a calibration of
16,000 counts at +1 g and 16,800 at −1 g: 400 counts a g. With 16,010 on the pad, a count of
12,010 in the climb is (16,010 − 12,010) / 400 = 10 g, 98.07 m/s² above rest. The MS5607 data
sheet's own example counts, D1 6,465,444 and D2 8,077,636, read 110,002 Pa at 20.00 °C, which a
test pins.

**Checked.** `cargo xtask logs` pairs each file with an AltosUI export in its folder of the same
serial number and flight. It reads the export by its column names and lines the two up by tick. At
each of the file's samples, the export's row at that hundredth must agree within half the export's
print step. Each of the export's rows must fall at a tick the file holds a sample at. With the
board's `.telem` file of the same serial number, each pressure at a tick both hold must be equal.
`cargo xtask logs --check` fails on any file that differs. Of the 14 files:

| Compared | Files | Values | Worst difference (Pa, K, m/s², V) | Half the print step (Pa, K, m/s², V) |
|---|---:|---:|---:|---:|
| Pressure, against the export | 9 | 38,908 | 0.005 Pa | 0.005 Pa |
| Temperature, against the export | 7 | 31,673 | 0.05 K | 0.05 K |
| Acceleration, against the export | 7 | 33,095 | 0.005 m/s² | 0.005 m/s² |
| A Mega's voltages, against the export | 3 | 9,968 | 0.005 V | 0.005 V |
| The export's rows, at a tick the file holds | 9 | 47,547 | 0 without | |
| Pressure, against the telemetry | 2 | 231 | 0 Pa | |

The MS5607 boards' pressures match their exports exactly. The TeleMetrum v1's pressures, a
conversion, match within half the export's 0.01 Pa print step. One export is the damaged one the
survey's report files as `altos-csv-damaged`. It zeroes the acceleration at the +1 g calibration
instead of the pad's reading, so it differs from the file by one constant. The survey finds that
constant within half a step and counts the file apart. The same export counts its time from the
tick itself. Per file, by an anonymized id, the [survey's report](#how-it-was-checked) prints each
count and each climb ratio.

Left out: the TeleMega v4's IMU and the fields kept as written, until a source or a second check
bears their units out. Also left out is any log format not listed above, which no sample holds
([evidence row G4](../evidence.md#g4)).

### The ARTS2's raw data

HPR Sim reads an Ozark Aerospace ARTS2's flight from two more files. One is the `.odf` file.
DataAnalyzer, the board's PC software, saves a downloaded flight to it. The other is the sensors'
counts as DataAnalyzer exports them to CSV. Only the Rust library reads them so far. The *ARTS2 User
Manual* (V1.0.0, Ozark Aerospace, 2007) describes neither file. So the layout was worked out from
the 3 sample flights that hold both. Each has DataAnalyzer's interpreted export of the same flight
beside it ([an ARTS2's unit system](#units-the-file-decides)). The work used no other program's
source code. The `.odf` carries the board's calibration, so its acceleration and pressure are read
in SI. The raw CSV carries none, so its counts are kept as written.

> **How far to trust it.** These are the board's own readings. They are decoded from a layout the
> maker doesn't publish. They are checked against the files beside each.
>
> - **Checked:** all 3 sample `.odf` files. Their raw CSVs hold the same 48,614 records word for
>   word, 0.02 s apart. Against DataAnalyzer's interpreted exports, the acceleration agrees within
>   0.080 of a count on all 48,611 records after the first.
> - **Assumed:** the rate, 50 records a second, which DataAnalyzer gives all three files ([evidence
>   row G5](../evidence.md#g5)). The pressure is the sensor's nominal conversion, with an error
>   band of ±1.5 kPa.
> - **Not read:** the meaning of most of the flags byte, the board's state codes and the
>   deployment settings.

More on the note's items:

- **Checked.** Each export's barometric altitude follows the pressure in order. The climb check is
  the climb over the first 2 s by the pressure, over the climb by the acceleration. It gives ratios
  of 0.754, 1.032 and 1.145, and the reader accepts 0.6 to 5/3.
- **Assumed: the rate** ([evidence row G5](../evidence.md#g5)). Where the file keeps the rate isn't known. The reader refuses a file whose
  likeliest word for it isn't the sample files' 2. On the sample files, the climb check is sure to
  catch only a time step below 0.67 or above 1.38 times the true one.
- **Assumed: the pressure** ([evidence row G5](../evidence.md#g5)). Against its ±1.5 kPa error band, an AltOS board flown with each flight
  reads a pad pressure 0.42 to 1.08 kPa above the ARTS2's.

The ARTS2 reads an Analog Devices ADXL78 accelerometer, set for ±50 g, and a Motorola MPXA4100A
pressure sensor. It reads them with its processor's 10-bit converter, so each count runs from 0 to
1,023 (the manual, Appendix A, p. 16). The `.odf` is little-endian throughout. Its length must be
exactly its parts':

| Offset (bytes) | Length (bytes) | What |
|---:|---:|---|
| 0 | 4 + 9 | the length 9, then `Ozark.odf` |
| 13 | 20 | five 32-bit words, the same in every sample file; not read |
| 33 | 256 | the board's settings and calibration (below) |
| 289 | 4 | the number of records, `n` |
| 293 | 12 | three 32-bit words, each the memory's size in records; equal, and at least `n` |
| 305 | 8 `n` | the records |
| 305 + 8 `n` | 124 | DataAnalyzer's own settings; not read |

Of the settings block, the reader takes four things, each a 16-bit little-endian word. At its offset
8 is a word that is 2 in every sample file. That is the likeliest place for the recording rate,
which the owner sets (pp. 6–7). A file where that word is anything else is refused, as its clock
would be unknown. At 10 is the barometer's count on the pad. This is the "Zero Altitude baseline"
the board averages on entering flight mode (p. 17). At 12 is eight times the accelerometer's count
on the pad, its "Zero G baseline" (p. 16). So it keeps an eighth of a count. At 20, 22 and 24 are
three sums of 64 accelerometer readings, taken when the board was calibrated "at +1g, −1g, and 0"
(p. 16). In every sample file the middle one is the reading on its side, midway between the two
ends.

Each record is four 16-bit words, in the order of the raw CSV's columns. They are the
accelerometer's count (`Acc(HI)`), a byte of flags (`Acc/Baro(LO)`), the barometer's count
(`Baro(HI)`) and the board's state (`State`). The first record is the two baselines, not a sample:
the pad's barometer count, and the accelerometer's cut to a whole count. A file whose first record
isn't so is refused, as its bytes would be laid out otherwise than the sample files'. DataAnalyzer
exports that record as the flight's first row, at time 0, and so does the reader.

What the reader does with the records:

- **The clock.** The records are 0.02 s apart, 50 a second, from the first. That is the rate
  DataAnalyzer gives every sample file in both its exports. Where the file keeps the rate isn't
  known. The sample files are all at one rate, so no byte can be shown to change with it. The reader
  refuses a file whose settings word at 8 isn't 2. Otherwise, the climb check below bounds the
  clock.
- **Acceleration**, along the board's axis, toward its forward end, from the file's own calibration.
  The two end sums are 2 g apart. So a g is a 128th of their difference, in counts: 64 readings in
  each sum, over 2 g. The pad's reading is taken out, as DataAnalyzer's export takes it out. So the
  rocket at rest on the pad reads zero, gravity with it. With $c$ the count, $c_{\mathrm{pad}}$ the
  pad's baseline and $k$ the counts a g, the acceleration is

  $$
  a = \pm\frac{c - c_{\mathrm{pad}}}{k}\, g_0,
  $$

  The pad sets the sign. Standing nose up, the board reads +1 g along its axis. So the pad's reading
  lies near one end sum. A count moving further toward that end is an acceleration forward,
  positive. The ADXL78's data sheet must bear the calibration out (*ADXL78*, Rev. B, Analog Devices,
  2010, p. 3, for ±50 g). A g must be from 36.1 to 39.9 mV, 7.39 to 8.17 counts on the board's 5 V
  supply. The side's reading must lie within 150 mV, 30.72 counts, of half the supply and between
  the two ends. The pad's reading must be nearer an end than the side's. Otherwise the file is
  refused.
- **Pressure**, by the MPX4100A's nominal transfer function, the first line below (*MPX4100A*, Rev
  9, Freescale, 2009, p. 5). Here $V$ is the sensor's output, $V_S$ its supply and $P$ in kPa. The
  converter reads the output against the supply as its full scale, so $V/V_S$ is $c/1024$. That
  gives the second line:

  $$
  \begin{aligned}
  V &= V_S\,(0.010\,59\,P - 0.1518) \\
  P &= \frac{c/1024 + 0.1518}{0.010\,59}
  \end{aligned}
  $$

  One count is 92.2 Pa, about 7.7 m (25 ft) of height near sea level. The pad's pressure must lie
  in the sensor's rated 20 to 105 kPa (p. 3), or the file is refused. The sensor's error band is
  ±1.5 kPa over that range from 0 to 85 °C (p. 6).
- **The climb check** holds the clock and both units to each other, as for [AltOS's
  `.eeprom`](#altoss-eeprom). Over the first 2 s from the first record, the climb by the pressure
  must agree with the climb by the acceleration. The acceleration is integrated twice from rest. The
  pressure climb is taken as its pressure altitude in the U.S. Standard Atmosphere, 1976. Their
  ratio must lie from 0.6 to 5/3. The window is twice AltOS's 1 s because this barometer is coarse.
  Over 1 s, one sample flight climbs less than two counts. The check bounds the clock only loosely.
  The climb by the acceleration goes as the square of the time step. So with a flight's ratio $r$ on
  its true clock, a step $k$ times the true one passes for $k$ from $\sqrt{r/(5/3)}$ to
  $\sqrt{r/0.6}$. That is 0.67 to 1.12 for the sample file of the least ratio and 0.83 to 1.38 for
  the greatest's.
- **A file that fails the climb check is refused.** AltOS's reader keeps such a file's counts as
  written. This reader can't, because a count kept as written still needs a clock. Nothing else in
  the file bears a clock out.
- **Kept as written:** the flags byte and the state. Bits 3 and 2 of the flags byte are the
  barometer count's low two bits in every record of every sample file. A record where they aren't is
  refused. That is a check that the words are where the reader takes them. The other bits follow the
  flight, but no document names them. The state steps through 0, 1, 2, 3, 5, 6 and 7 in each flight,
  in order. No document names those either, so no event is taken from it.

The file holds no altitude. The record's [primary track](#the-record) is empty, as for the other
[raw downloads](#raw-downloads), and every word is a channel. The record's stated figures hold the
two baselines and the three calibration sums.

**A worked example**, from the invented file the crate's test `tests/arts2_raw.rs` reads. Its
settings block holds a pad barometer count of 800 and an accelerometer word of 4,100 (512.5 counts).
The calibration sums are 31,744 (64 × 496), 32,250 and 32,768 (64 × 512). The ends differ by 1,024,
so a g is 1,024 / 128 = 8 counts, inside the data sheet's 7.39 to 8.17. The side's reading, 32,250 /
64 = 503.9, lies between the ends and within 30.72 of 512. The pad's 512.5 lies near the 512 end, so
the acceleration is positive as the count rises. A count of 576 in the boost is (576 − 512.5) / 8 =
7.94 g, 77.84 m/s² above rest. The pad's barometer count reads (800 / 1,024 + 0.1518) / 0.010 59 =
88.107 kPa, inside the sensor's 20 to 105 kPa.

**The raw CSV** is DataAnalyzer's raw export of the same records. It has a
`MET,Acc(HI),Acc/Baro(LO),Baro(HI),State` header, then a row for each record. Each row holds its
time and its four words as whole numbers:

```text
MET,Acc(HI),Acc/Baro(LO),Baro(HI),State
0.000,512,0,800,64
0.020,576,0,800,0
```

It holds no calibration, only the first record's whole-count baselines. So no column is converted.
The two counts and the flags byte are kept as written, the state as a number, and `MET`,
DataAnalyzer's clock, as the time. Every row must hold five numbers. Its four words must be whole
and in range. Its time must be after the row before's. Its flags byte must hold the barometer's low
bits. A file where a row fails any of these is refused, naming the line. For values in units, read
the flight's `.odf`.

**Checked.** `cargo xtask logs` pairs each `.odf` with the files of its flight in its folder. A raw
CSV must hold its records word for word, 0.02 s apart. DataAnalyzer's interpreted export must agree
in `Acc` within half a count on every record after the first. It must read zero in `Acc` and `pAlt`
at its first row and at any row after the records. Its `pAlt` is compared by order only. It must be
zero at the pad's pressure and above. It must rise as the pressure one record earlier falls below
that. Each pressure must meet one value, and no row's `pAlt` may be missing. `cargo xtask logs
--check` fails on a file that differs. For the 3 `.odf` files, the results are:

| Compared | Files | Values | Result |
|---|---:|---:|---|
| Records, against the raw CSV | 3 | 48,614 | every word equal |
| `Acc`, against the interpreted export | 3 | 48,611 | within 0.080 of a count |
| `pAlt`'s order, against the pressure | 3 | 48,611 | no pressure meeting two values; none out of order; no gap |
| The climb check | 3 | | ratios 0.754, 1.032 and 1.145 |

Reported, not checked: in two exports, `pAlt` lies within 3.3 m (11 ft) of the standard atmosphere's
pressure altitude from the `.odf`'s pressure. In the third, `pAlt` departs by up to 151 m (496 ft),
worked by some other formula. An AltOS board flew beside the ARTS2 on each flight. The ARTS2's pad
pressure reads 0.42 to 1.08 kPa below the AltOS board's. The ARTS2's least pressure reads from 0.48 kPa
below to 0.11 kPa above the AltOS board's. Both fall inside the MPX4100A's ±1.5 kPa error band. The
[survey's report](#how-it-was-checked) prints each file's counts and climb ratio under an anonymized
id.
[ADR-266, the ARTS2's raw data](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0266-arts2-raw-data-before-the-rest.md)
gives the evidence for the layout.

Left out: the flags byte's bits beyond the barometer's two, the state codes' meanings, the
deployment settings and the rest of the settings block. Also left out is where the file keeps its
rate. The climb check stands in for that.

### The AltiMAX's `.ALS`

An AltiMAX altimeter with a memory card writes each flight to it as an `.als` file (*AltiMAX G2
Handbuch*, Louis Schreyer, 2010, p. 12). The maker's PC software opens and exports that file ([the
AltiMAX's Excel export](#the-altimaxs-excel-export)). `hpr_flightdata::altimax_als` reads the file,
and `import::read` recognizes it like any log. No document describes the bytes. The layout was
worked out from the one sample file against the Excel export of the same flight. Every value the
file stores is the export's.

The file is a header of 32 bytes. Then comes one record of 22 bytes for each row of the export. Then
comes a trailer that isn't read. The header holds the *zero pressure*, the pressure the export
measures every height from, the apogee's pressure, and the count of records. A record holds the
filtered and the raw pressure in pascals. It holds the acceleration in counts of 0.01 g along an
axis that points down the rocket. It also holds an event code and the count of one of five voltage
channels, the channels taking turns. A bit of a record's first byte says whether it was stored 5 ms
or 25 ms after the record before. Those are the 200 and 40 samples a second of the maker's table
(*AltiMAX G4 Manual*, Rocketronics, 2021, p. 10). The clock is the running sum, as the export's
`TIME` is. The [API reference](../api/hpr_flightdata/altimax_als/index.html) gives every byte.

**What the record holds.** Each value is a channel, named as the export names its column. `PRESS
FILTER` and `PRESS RAW` are in pascals. `ACCEL` is in m/s², each count 0.0981 m/s², the export's own
factor, so at rest it reads about −1 g. `EVENT` and `VOLT. CHANNEL` are numbers. Each voltage
channel's counts are kept as written, `VOLTAGE count (channel 1)` to `(channel 5)`. A record's event
code is an event, named as the export names it (`BARO-LIFTOFF`, `APOGEE PRESSURE`, `LAND` and 13
more). For a code the sample doesn't show, it is `event N`. The zero and apogee pressures are
figures. The file holds no height, so the record's [primary track](#the-record) is empty and its
channels hold everything. A height is the [standard
atmosphere](../glossary.md#standard-atmosphere)'s height of a pressure, less that of the zero
pressure. For example, take a zero pressure of 98,000 Pa and a filtered pressure of 97,000 Pa. The
height is 44,330.77 × (1 − (97,000 / 101,325)^0.190263) − 280.5 = 366.4 − 280.5 = 85.9 m (282 ft)
above the pad.

**Checked.** `cargo xtask logs` reads the `.ALS` and the Excel export of the same flight beside it.
`cargo xtask logs --check` fails if any of these comparisons does. The acceleration, voltages and
clock are written the export's way. That uses the two rules after the table, which were fitted to
this file:

| Compared, against the export | Values | Result |
|---|---:|---|
| Records, against its rows | 25,334 | one a row |
| Both pressures, the event code and the voltage channel's number, as stored | 101,336 | every one equal |
| The acceleration | 25,334 | every one equal |
| The voltages, by the export's own factors | 25,334 | every one equal |
| The clock, 5 ms or 25 ms a record summed in 64-bit floats | 25,334 | every one equal |
| The speed before apogee, the counts summed from liftoff | 8,373 | every one equal |
| The events | 21 | the same names, in the same order |
| Each height, filtered (25,334) and raw (25,334), against the standard atmosphere from the zero pressure | 50,668 | within 8.7 mm (0.34 in); the check allows 41.9 mm (1.65 in), the height of half a pascal at the zero pressure |

The [survey's report](#how-it-was-checked) prints these under the files' anonymized ids.

**How the export writes a number.** Two rules, fitted to this one file, reproduce the export's
numbers. *Converting* is a model of how the export turns a number into 15 digits. *Multiplying* is
how it turns a voltage channel's count into volts. Neither changes what the reader returns, the
values the file stores. The rules only write those values as the export does, so the check can
compare them. The software holds each value as a 64-bit float, a *double*. Its significant part (the
*significand*) holds 53 bits, good to about 16 significant digits. The export keeps 15, as Excel
does.

*Converting.* The export doesn't round a value to 15 digits once. Its numbers match arithmetic in
80-bit floating point. That is the format of an x86 processor's older floating-point unit. Its
significant part holds 64 bits. The model is fitted to this file, not taken from the software. One
row of the 25,334 is what tells it from plainer rules (below). In three steps, the model:

1. guesses the value's power of ten from its power of two: the power of two's exponent times log₁₀
   2, rounded down. The guess is one too low from each power of ten up to the next power of two.
   That holds from 10 to under 16, 100 to under 128 and 1,000 to under 1,024, for example. It also
   holds from 0.001 to under 2⁻⁹, 0.001 953 125;
2. scales the value by that power of ten to 18 digits. It rounds that to 64 bits, the nearest number
   whose significant part fits 64 bits. Where the guess was low, that makes 19 digits. The model
   then divides them by 10, rounded to 64 bits again;
3. rounds the result to a whole number, *half to even* (an exact half goes to the even neighbor:
   …498.5 to …498, …499.5 to …500). It then rounds that to 15 digits, upward when exactly halfway.

A clock summed from 5 ms steps in doubles can land just under halfway between two 15-digit values.
It can land close enough that the 18 digits land on halfway, which then rounds up. For example, 347
steps make 1.734 999 999 999 984 998 7 s. The model writes that 1.734 999 999 999 99. Rounding once
gives 1.734 999 999 999 98.

The file's clock at 122.005 s is where the guess shows. Its value is 122.005 000 000 015 499 495 s.
Its power of two is 2⁶, so the guess is 10¹, where 10² is right. Each line of this table is one
step, as a whole number at the scale it works at:

| Step | With the power of ten exact, 10² | With the guess, 10¹ |
|---|---|---|
| Scaled | 122 005 000 000 015 499.495, 18 digits | 1 220 050 000 000 154 994.95, 19 digits |
| Rounded to 64 bits | …015 499.492 | …154 995 |
| Divided by 10 | not needed | …015 499.5 |
| Rounded to a whole number, half to even | …015 499 | …015 500 |
| To 15 digits, half up | 122.005 000 000 015 | 122.005 000 000 016, as the export writes it |

The other choices in the model can't change a value, so no file can test them. Half up instead of
half to even in step 3 gives the same 15 digits, as …499.5 goes to …500 either way. The second
rounding to 64 bits moves a value too little to cross a half.

Take the power of ten as known exactly. Values are rounded first to each number of digits, then
half up to 15. The table below counts how many of the 25,334 clock values then differ from the
export's. Rounded once, 11 differ, the 122.005 s row among them. Rounded first to 18 digits, only
that row differs. Under the model, that row alone needs the guess and the first rounding to 64 bits.
A running sum one double higher at that row in the software would explain it under the plainer
18-digit rule. The file can't rule that out.

| Rounded first to (digits) | Clock values that differ |
|---|---:|
| 15 (once) | 11 |
| 16 | 1,245 |
| 17 | 109 |
| 18 | 1 |
| 19 | 11 |
| 20 | 11 |

The model divides by 10 for 2,327 clock values, 3,265 accelerations and 5,066 voltages. Every one of
the 25,334 clock values, accelerations and voltages matches the export.

*Multiplying.* The volts are the double nearest the count times the factor, computed exactly and
then rounded once. Multiplying the count by the factor already rounded to a double puts 31
voltages a unit off in the 15th digit.

The [survey's report](#how-it-was-checked) prints each count. *By doubles* counts the voltages
multiplied by the rounded factor. *Divided by 10* counts the clock values, accelerations and
voltages the guess put at 19 digits. *First rounded to 15–20 digits* is the table above.

**Left out.** The voltages stay counts. The export turns them into volts by one factor for channels
1 and 5 and another for 2 to 4. Those are 0.00314278884 and 0.00266754240 V a count, rounded here.
These were found by dividing the export's volts by its counts. They equal 3.218 215 77 V ÷ 1,024 and
5.463 126 83 V ÷ 2,048 exactly. The multiplying rule uses that reading, which is this project's own.
No document states either factor, nor which channel is which. The export's speed after apogee,
16,961 rows, is worked out by the software from the pressure, not stored. It isn't compared. The
header's other bytes and the trailer aren't read. A record's last 8 bytes are zero in every sample
record. They are held to zero, and a file where they aren't is refused, naming the byte. So is one
with a record past the header's count. A flight of more than 65,536 records, more than its 16 bits
count, would have one. One sample, from one altimeter, is all the layout rests on.

### The AIM's `.xtra` and raw dump

The Entacore AIM software downloads an AIM XTRA flight computer's memory and saves each flight as an
`.xtra` file. The software exports from that file the CSV HPR Sim also reads. The AIM XTRA's manual
names both files but describes neither (*AIM XTRA/BASE v4.x.x Manual*, Entacore, 2 February 2025, p.
38). The sample logs hold one flight's three files. They are the raw dump of the memory, the `.xtra`
the software made of it, and an earlier copy of that `.xtra`, cut short.
`hpr_flightdata::entacore_raw` reads both layouts. `import::read` takes an `.xtra` file, and a dump
that holds one recording. The sample's holds ten: nine whose pressure shows no flight, and then the
flight. `import::read` refuses a dump of several, naming the byte where the second starts.
`entacore_raw::read_dump` reads each as a record of its own. `import::read_one` reads the one
chosen, counted from 0, as `hpr analyze --recording N` and Python's `FlightLog.read(path,
recording=N)` do. With none chosen, they read a dump of one recording and refuse one of several,
saying how many it holds ([A file of several records](../cli.md#a-file-of-several-records)).

A dump is the memory's recordings end to end, then erased memory, bytes of `0xFF`. A recording opens
with the byte `0x13` and 101 bytes that aren't read. After that, a byte from `0x80` to `0xFE` adds
its low 7 bits to a clock counted in the computer's *ticks*. Any other byte is a record's type, its
fields following, big-endian (the most significant byte first). Each of the 16 types is one group of
readings, such as the barometer's or the status's, at its own rate. The manual gives these rates (p.
2):

- the barometer every 20 ms;
- the accelerometers and gyroscope every 10 ms;
- the magnetometer every 100 ms;
- the temperature every 500 ms;
- the GPS's position every 200 ms and its time every 500 ms.

The `.xtra` is an archive in the binary format of Boost, a C++ library. It holds the same samples
converted, one record each with its time in seconds. The [API
reference](../api/hpr_flightdata/entacore_raw/index.html) gives every type and field.

**What the record holds.** Each group's fields are channels on the group's own clock, named as the
CSV export names the same columns:

- `acceleration`, `lat. X accel.` and `lat. Y accel.` in m/s², from the g the computer records;
- `Gyro X` to `Gyro Z` in degrees a second;
- `pressure` in pascals;
- `temperature` in kelvin;
- `battery 1 voltage`, `battery 2 voltage` and `A Voltage` to `D Voltage` in volts;
- `lat`, `long`, `GPS MSL`, `SVs` and the GPS's time;
- the status, `state` and the four outputs' `on` and `cont` flags.

The magnetometer and the attitude quaternion are kept as written. Each change of `state` is an
event, `state 1` to `state 3`. So is each output's firing, such as `C on`. The values are converted
as the software converts them, in 32-bit floats (single precision, about 7 significant digits), so
the two readers' channels are equal. A raw battery record `04 80 00`, for example, holds the count
2,048 in its top 12 bits. The count is then multiplied by 3.3 V × 133/33, a factor of 13.299 999 2 V
taken in 32-bit floats, and divided by 4,096. The result reads 6.649 999 6 V, one step of a 32-bit
float below 6.65 V, as the software writes it to the `.xtra`. Neither file holds an altitude, so the
record's primary track is empty, as for the `.ALS`.

**The clock.** The dump's ticks are read as milliseconds, as the software reads them. The `.xtra`'s
times are the software's own sum of them in 32-bit floats. That sum falls behind as it grows. In the
sample, it is behind by 2.40 s by the end of the recording, and by 0.9 % of the climb's time.
Neither set of times is GPS time. Against the GPS's time of week, recorded beside them, a tick is
1.009 ms of the GPS's clock over the flight. On the pad it is 1.021 ms. So the dump's times run 0.9
% short in flight and 2.1 % on the pad. The `.xtra`'s times are behind the ticks as well. They are
about 1.8 % short over the climb (0.9910 of the ticks' time, each tick 1.0087 ms: 0.982 of the
GPS's). The reader doesn't correct either set of times. One flight can't tell whether that is this
computer's clock or every AIM's.

**Checked.** `cargo xtask logs` reads the three files of the flight. `cargo xtask logs --check`
fails if any of these comparisons does:

| Compared | Values | Result |
|---|---:|---|
| The `.xtra`'s channels, against the dump's flight recording | 435,075 | every one equal, bit for bit |
| The `.xtra`'s times, against the dump's ticks summed in 32-bit floats a tick byte at a time | 224,643 | every one equal |
| The dump reader's times, against the ticks walked apart from it | 224,644 | every one equal |
| The short copy, against the whole `.xtra`'s first samples | 814,206 | every value and time equal; its bytes are the whole one's first |
| The pad: the three accelerometers' medians over the last 10 s before liftoff, as one magnitude | | 0.965 g, inside 0.85 to 1.15 |
| The time from liftoff to the least pressure, over √(2h/g), h the standard atmosphere's climb | | 1.064, inside 0.5 to 2 |

The dump holds one sample past the `.xtra`'s, which the software leaves out. On the pad the
accelerometer reads 0.965 g, 0.961 along the rocket alone. The maker's `.xtra` reads the same. On
this flight the sensor reads about 3.5 % low. The check allows 15 %, the band for an accelerometer
at rest that [the AltiMAX export](#the-altimaxs-excel-export) set. Against that band, the axial
reading in another unit or at another of the sensor's ranges is 2 or more times off. The sideways
readings are held to the `.xtra`'s values, bit for bit. Liftoff, for these checks, is the first
reading above 1.5 g in the run that holds `state 1`. That state is the computer's launch detect,
which came 1.14 s after liftoff on the sample flight.

**Left out.** A recording's 101 opening bytes; what `state`'s numbers mean; the magnetometer's unit;
and which battery is the computer's and which the charges'. Each byte whose meaning the sample
doesn't show is held to the one value it has there. A file where it differs is refused, naming the
byte. A type or class the sample doesn't hold is refused too, as its length is unknown. But among a
dump's first 1,000 records, or as an `.xtra`'s first, such a type or class leaves the file
unrecognized instead. One flight, from one computer, is all the layouts rest on.

## How it was checked

Each layout's reader is also fed cut, changed and arbitrary bytes, failing on a panic or a hang:
[Damaged files](damaged-files.md).

`cargo xtask logs`, a command of the project's own build tool, reads every sample log the project
holds, each distinct file once. The sample logs come from:

- the private collection's catalog of flight logs;
- the public fixtures and samples of Debrief;
- AerospaceNU's published flight logs.

The command leaves out a cache of downloads whose flights' logs the collection already catalogs. It
also leaves out the data folder of RocketPy's repository, tables RocketPy processed. The teams' logs
from that folder that the collection catalogs are read like any other. It writes what came of each
to
[`validation/reports/log-import.md`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/log-import.md).
That report holds:

- per layout, how many files were read and how many refused and why;
- how many files' readers decided each unit their headers leave out, and how;
- how each raw download compares with its maker's export
  ([above](#checked-against-the-makers-exports));
- how each Featherweight summary compares with the files of samples beside it
  ([above](#checked-against-the-files-beside));
- how AltOS's telemetry compares with AltosUI's export beside it, and its board's estimates with
  each other ([above](#altos-telemetry));
- how each AltOS `.eeprom` file compares with the export and the telemetry beside it
  ([above](#altoss-eeprom));
- how each ARTS2 `.odf` compares with the raw CSV and the interpreted export beside it
  ([above](#the-arts2s-raw-data));
- how each RTx `.rtx` compares with the CSV, text and KML exports beside it
  ([above](#missile-works-rtx));
- how each table read by its header had its columns decided, as counts
  ([above](#tables-read-by-their-headers));
- per file, an anonymized id with a digest, a hash, of the record read from it. The id is the start
  of a hash of the file, which says nothing of its contents.

The files inside the zip archives among them are unpacked and counted too, each once. Of the 29 in
the 8 archives, 19 are in no other source ([zip archives](#zip-archives)). The report also covers:

- each Excel workbook's sheets, by the layout each reads as ([Excel workbooks](#excel-workbooks));
- each table read by a mapping its publisher states ([above](#mappings-a-publisher-states));
- the range of the Eggtimer files' filtered-altitude multiplier ([above](#units-the-file-decides));
- each file of binary records, against its team's table
  ([above](#binary-records-by-a-stated-layout));
- each Blue Raven USB download, against the survey's own decoding of it
  ([above](#the-blue-ravens-usb-download));
- each Teensy logger's file, against the survey's decoding and its physical checks
  ([above](#how-the-teensy-files-were-checked));
- each file of radio packets, against the survey's decoding, and each received or exported packet,
  against the onboard log's ([above](#how-the-radio-packets-were-checked));
- each FIP sheet, against the `.FIPa` beside it ([above](#how-the-fip-sheets-were-checked));
- the AltiMAX export, against the survey's own decoding of its stream and its physical checks
  ([above](#how-the-altimax-export-was-checked));
- each MARSA download, AltOS telemetry table, table cut from an AltOS export and re-saved AltOS
  export. Each is held against the survey's own decoding and the checks of its units
  ([above](#a-marsa-download));
- each table a mapping holds to physics, against its checks
  ([above](#tables-a-mapping-holds-to-physics));
- the AltiMAX `.ALS`, against the export beside it ([above](#the-altimaxs-als));
- the AIM's `.xtra` files and raw dump, against each other ([above](#the-aims-xtra-and-raw-dump)).

Of the 1,122 files it holds today, 1,052 are read. The counting rules are:

- the 8 archives count as read when they unpack;
- the 16 workbooks of their own layout count as read when a sheet of each reads;
- the 2 FIP sheets count as read when they read under their stated headers and match their `.FIPa`;
- the AltiMAX `.xls` counts as read when it reads and holds to its checks;
- the `.ALS` and the AIM's files count as read when they hold to theirs.

Of the other 70, 67 aren't logs and 3 are damaged (below). No file of a layout HPR Sim reads is left
unread. Since [M7.1c23](../decisions-and-roadmap.md#m7-1c23), the last of the importers that
[ADR-274](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0274-the-rest-of-the-importers-planned-teensy-logger-first.md)
plans, `cargo xtask logs --check` fails on one. Since [M7.2b, readings on every
logger](../decisions-and-roadmap.md#m7-2b), it also takes the readings from every record it reads.
It counts each reading given and withheld, by family and by reason, with a case for each reason
([Flight-log readings](../physics/log-readings.md#checked-against)).

A file the collection's catalog files under a layout it doesn't have is moved, by id, with the
reason printed. One is a spreadsheet's re-save of an AltOS export, its header edited, held to
AltOS's reader ([above](#altoss-csv-after-a-spreadsheet)). Another is an AltOS export whose header
line is damaged, two column names run together. No reader can line up that line with its rows, and
the file is refused. A third, a KML file, is a map of a flight made in a map program. Its predicted
tracks have the pad and landing spot pinned. The file is read but counted apart as not a log.

And 62 files the catalog files as generic CSV, by their extension, are in these layouts and held to
those readers:

- AltimeterCloud's layout (10);
- Entacore AIM's (15);
- a Fluctus's (6);
- an ARTS2's interpreted data (4);
- the phone app's `launch` CSV (27).

The same filing holds more:

- 2 more are Eggtimer downloads saved with semicolons, one held to the Eggtimer's reader and one,
  its decimal points taken out, counted as damaged ([above](#units-the-file-decides));
- 6 are tables whose publisher states their mapping ([above](#mappings-a-publisher-states));
- one is an RRC3 export counted as damaged ([above](#units-the-file-decides));
- 4 are AltOS's: its telemetry decoded to a table, a table cut from an export and 2 whole exports,
  each held to its reader ([above](#altoss-telemetry-as-a-table));
- 12 are tables a mapping holds to physics ([above](#tables-a-mapping-holds-to-physics));
- 6 aren't logs (below).

The 2 files the catalog files as XML, for their `<…>` lines, are MARSA downloads, held to their
reader ([above](#a-marsa-download)). Of the 4 files it files as binary, 2 are a team's binary
records, read by a stated layout ([above](#binary-records-by-a-stated-layout)). The other 2 are the
AltiMAX's `.ALS` and the AIM's raw dump, held to their readers ([above](#the-altimaxs-als),
[above](#the-aims-xtra-and-raw-dump)). Of the 3 RTF files the catalog files as not a log, by their
extension, one is an Eggtimer's download, held to its reader. Two are exports of radio packets, held
to the packets' reader ([above](#one-teams-radio-telemetry-packets)). Sixteen spreadsheets are
workbooks of flights' logs, held to the workbook reader. Three files filed as text are console
captures. Two filed as binary are files of records. Each of these is read by the layout stated for
it ([above](#binary-records-by-a-stated-layout)). Two more filed as text are a Blue Raven's USB
downloads, held to their reader ([above](#the-blue-ravens-usb-download)). Eight more are a course's
Teensy logger's, held to theirs ([above](#a-courses-teensy-data-logger)). Two are radio packets, the
onboard log and the receiver's text, held to the packets' reader
([above](#one-teams-radio-telemetry-packets)). Five files filed as generic CSV or text are
PerfectFlite's exports: four with no header row and one that names its temperature `Temp`.

A Blue Raven high-rate file with no date is held to the Blue Raven's reader. Its low-rate partner
was edited by a person in a spreadsheet (a column with no name, two columns named in words the app
doesn't write). It was counted as damaged until [M7.1c13](../decisions-and-roadmap.md#m7-1c13). It
is now read as a [table by its header](#tables-read-by-their-headers), as are 30 other files filed
as generic CSV. Three more are an ARTS2's raw CSV, held to its reader. Nine are mDACS's exports of an
RTx's fixes, five CSV and four text, held to their reader. Six more filed as text, and a ground
station's conversion of a SillyGoose flight, are tables a mapping holds to physics
([above](#tables-a-mapping-holds-to-physics)). So is one workbook. The conversion has its columns in
the order of the board's log, with its settings under no header.

Some files aren't flight logs, and are counted apart as not logs, as pictures and maps are. By id,
four are spreadsheets of a team's checklist for preparing a flight, its steps and no measurement.
Nine are spreadsheets of a Blue Raven's deployment settings, with a `Hardware_Channel:` row naming
its outputs. Three are spreadsheets of a flyer's summary of flights, a few rows of figures by flight
or by launch and no samples. By family, six are Debrief's made-up sample flights. Their first row
says each isn't a recording of anything. Twelve are text files a flyer's own program wrote from a
Blue Raven's low-rate file, titled as that file's events. By family too, since
[M7.1c17](../decisions-and-roadmap.md#m7-1c17), one is a text file the same flyer's program wrote
from a Blue Raven's summary CSV beside it. It has the summary's rows as cells between `|`. It is a
copy, every cell equal to the CSV's, and the CSV is read.

By id, since [M7.1c7](../decisions-and-roadmap.md#m7-1c7), two more are those events files. Three
are descriptions of a rocket written for a flyer's or a team's own program. Three are one-row files
of a team flight computer's saved state (a boot count, gyro biases, the pressure before boost). One
is an OpenRocket simulation's export, a simulation and not a flight. Inside the archives, by id,
there are three more events files and one more description of a rocket. There are also a Blue
Raven's settings as a spreadsheet and as CSV, and a wind profile, altitude against wind speed and
direction.

By id, since [M7.1c22](../decisions-and-roadmap.md#m7-1c22), these are not logs either:

- a flight computer's housekeeping (its processor, memory, disk and network counters);
- a battery pack's housekeeping (its cells' voltages, temperatures and current);
- two GPS tracks' coordinates copied from their maps with no clock, the tracker's own file of the
  same fixes beside each;
- three tables a program worked out from a log:
  - a Kalman filter's state in a workbook, whose altitude reaches 169 km (105 mi);
  - an inertial unit's recording turned into the rocket's frame with gravity taken out;
  - a flyer's rebuild of an altimeter's log, rows marked as made up, the altimeter's own file beside
    it read.

The collection is other people's data. Debrief's fixtures come from flights shared on terms nobody
recorded. AerospaceNU's are published under the GPL. So no file and no value read from any of them
is committed. The digest pins each reading instead, as a snapshot (a stored copy of a test's output)
would. The check fails if a reader's output changes or a file of these layouts stops reading.
Invented files in the crate's tests are read in CI and pinned whole by snapshots.
