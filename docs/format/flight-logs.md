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
> Other layouts are refused, not guessed at; they come with
> [M7.1c12](../decisions-and-roadmap.md#m7-1c12), the other loggers' formats.

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
| RRC3 CSV | Missile Works RRC3, through mDACS, Missile Works' program that downloads its flights | `Time`, `Altitude`, `Pressure`, `Velocity`, then any of `Temperature`, `Events`, `Voltages`, perhaps with units in brackets | `hpr_flightdata::rrc3` |
| RRC3 `.rff` | Missile Works RRC3, its memory as mDACS downloads it, before any export | a .NET serialization header (MS-NRBF, version 1.0) and, within the first 4,096 bytes, mDACS's class name `mDACS.AnalysisSupport+FlightRecord` | `hpr_flightdata::rrc3_rff` ([below](#raw-downloads)) |
| Eggtimer CSV | Eggtimer Classic, TRS, Quantum and Proton, through Eggtimer's download | `T,Alt,` and more of Eggtimer's column names | `hpr_flightdata::eggtimer` |
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
| Zip archive | not a log: a flight's files, zipped to share them | its bytes open `PK` then 3, 4 | `hpr_flightdata::archive` ([below](#zip-archives)) |

The Blue Raven app writes each flight as a low-rate file, logged 50 times a second, and a
high-rate file, 500 times a second (*Blue Raven User's Guide*, 2026-05-12, p18); each is read on its own, as a record of its own, not joined
to the other. The high-rate file holds no altitude, so its record's
[primary track](#the-record) is empty and its channels hold everything; the low-rate file of the
same flight holds the altitude. The app's summary of the flight is read too, as a record of
figures with no samples ([below](#featherweights-summaries)).

Some files among the sample logs aren't flight logs, and are counted apart as such: Debrief's
made-up sample flights, which say so in their first row; a team's checklists for preparing a
flight; a Blue Raven's deployment settings saved as spreadsheets or CSV; the events files a
flyer's own program wrote from a Blue Raven's low-rate file, and the descriptions of a rocket
such programs read; a team flight computer's saved state, one row; an OpenRocket simulation's
export; and a wind profile. The other layouts in the sample logs, among
them spreadsheets and a radio telemetry stream
saved as RTF, are refused today, most as *unrecognized*; they come with
[M7.1c12](../decisions-and-roadmap.md#m7-1c12), and the [survey's report](#how-it-was-checked)
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
isn't known, so they aren't read as a Blue Raven's; they wait for the column mapping of
[M7.1c12](../decisions-and-roadmap.md#m7-1c12).

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
- **An Eggtimer's clock, worked out from the file.** The Classic and TRS write their time in
  milliseconds, the Quantum and Proton in seconds, under the same `T`. The reader takes
  milliseconds when the median step is above 5: the manuals' rates, 1 to 33 samples a second, put
  a step between 0.03 and 1 in seconds and between 30 and 1,000 in milliseconds. Of the 43
  Eggtimer files, 3 are read in milliseconds and 40 in seconds.
- **An Eggtimer's heights and speeds, from the manuals.** The Classic and TRS manuals state feet
  and feet per second. The Quantum and Proton manuals state no unit for the download, but give
  every height in feet, and no Eggtimer manual offers meters; the heights are read in feet for
  every model, and in a terminal download, plain or saved as RTF, only where the coast before
  apogee also slows at about g in feet (below). A speed is read in feet per second only where it matches the altitude's change
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
  knows. An RTF document holding anything else is refused as not an Eggtimer's, among them the
  radio telemetry stream in two other sample files, which comes with
  [M7.1c12](../decisions-and-roadmap.md#m7-1c12).

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
against the files of samples in the same archive ([zip archives](#zip-archives)); those inside
spreadsheets wait for [M7.1c12](../decisions-and-roadmap.md#m7-1c12).

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

The latitude `4807.0380,N` is 48 degrees and 7.038 minutes north, $48 + 7.038/60 = 48.1173$°; the
longitude `01131.0000,E` is 11 degrees and 31 minutes east, 11.5167°. The altitude is 545.4 m
(1,789 ft) above mean sea level, in the unit the sentence names (`M`). The speed of 22.4 knots
reads as $22.4 \times 1852/3600 = 11.52$ m/s (37.8 ft/s), and the course is 84.4° from true north. The
record's zero is the first sentence's time, 12:35:19 UTC on 23 March 1994.

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
>   follows the pressure in order. The climb check's ratios are 0.754, 1.032 and 1.145, where the
>   reader accepts 0.6 to 5/3.
> - **Assumed:** the rate, 50 records a second, which DataAnalyzer gives all three files; where the
>   file keeps it isn't known, so it rests on the climb check. The pressure is the sensor's
>   nominal conversion, rated to ±1.56 kPa; an AltOS board flown with each flight reads a pad
>   pressure 0.42 to 1.08 kPa above the ARTS2's.
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

Of the settings block, the reader takes three things. At its offset 10 is the barometer's count
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
  gives every sample file in both its exports. Where the file keeps the rate isn't known, so the
  climb check below bears it out.
- **Acceleration**, along the board's axis, toward its forward end, from the file's own
  calibration. The two end sums are 2 g apart, so a g is a 128th of their difference, in counts:
  64 readings in each sum, over 2 g. The pad's reading is taken out, as DataAnalyzer's export
  takes it out, so the rocket at rest on the pad reads zero, gravity with it. With $c$ the count,
  $c_{\mathrm{pad}}$ the pad's baseline and $k$ the counts a g,

  $$
  a = \pm\frac{c - c_{\mathrm{pad}}}{k}\, g_0,
  $$

  the sign set by which end sum the pad's reading lies near, as the rocket stands nose up. The
  ADXL78's data sheet must bear the calibration out (*ADXL78*, Rev. B, Analog Devices, 2010, p. 3,
  for ±50 g): a g from 36.1 to 39.9 mV, 7.39 to 8.17 counts on the board's 5 V supply; the side's
  reading within 150 mV, 30.72 counts, of half the supply and between the two ends; and the pad's
  reading nearer an end than the side's. Otherwise the file is refused.
- **Pressure**, by the MPX4100A's nominal transfer function, the first line below, with $P$ in
  kPa and $V_S$ the supply (*MPX4100A*, Rev 9, Freescale, 2009, p. 5). The converter reads the
  output against the supply as its full scale, so $V_{\mathrm{out}}/V_S$ is $c/1024$, which gives
  the second:

  $$
  \begin{aligned}
  V_{\mathrm{out}} &= V_S\,(0.010\,59\,P - 0.1518), \\
  P &= \frac{c/1024 + 0.1518}{0.010\,59}\ \mathrm{kPa}.
  \end{aligned}
  $$

  One count is 92.2 Pa, about 7.7 m (25 ft) of height near sea level. The pad's pressure must lie
  in the sensor's rated 20 to 105 kPa (p. 3), or the file is refused. The sensor is rated to
  ±1.8% of its 4.59 V span, ±1.56 kPa (p. 3).
- **The climb check** holds the clock and both units to each other, as for
  [AltOS's `.eeprom`](#altoss-eeprom). Over the first 2 s from the first record, the climb by the
  pressure, its pressure altitude in the U.S. Standard Atmosphere, 1976, must agree with the climb
  by the acceleration, integrated twice from rest: their ratio must lie from 0.6 to 5/3. The
  window is twice AltOS's 1 s because this barometer is coarse: over 1 s, one sample flight
  climbs less than two counts. A clock off by a factor of 1.3 or more fails the check, as the
  climb by the acceleration goes as the square of the time step.
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
it must rise as the pressure one record earlier falls, each pressure meeting one value.
`cargo xtask logs --check` fails on a file that differs. Of the 3 `.odf` files:

| Compared | Files | Values | Result |
|---|---:|---:|---|
| Records, against the raw CSV | 3 | 48,614 | every word equal |
| `Acc`, against the interpreted export | 3 | 48,611 | within 0.080 of a count |
| `pAlt`'s order, against the pressure | 3 | 419, 406 and 443 pressures | no pressure meeting two values; none out of order |
| The climb check | 3 | | ratios 0.754, 1.032 and 1.145 |

Reported, not checked: two exports' `pAlt` lies within 3.3 m (11 ft) of the standard
atmosphere's pressure altitude from the `.odf`'s pressure, and the third's departs by up to
151 m (496 ft), worked by some other formula. An AltOS board flew beside the ARTS2 on each
flight: the ARTS2's pad pressure reads 0.42 to 1.08 kPa below it, and its least pressure from
0.48 kPa below to 0.11 kPa above, inside the MPX4100A's rated ±1.56 kPa. The
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
of downloads whose flights' logs the collection already catalogs, and RocketPy's processed flight
tables, which no logger wrote. It writes what came of each to
[`validation/reports/log-import.md`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/reports/log-import.md):
per layout, how many files were read and how many refused and why; how many files' readers
decided each unit their headers leave out, and how; how each raw download compares with its
maker's export ([above](#checked-against-the-makers-exports)); how each Featherweight summary
compares with the files of samples beside it ([above](#checked-against-the-files-beside)); how
AltOS's telemetry compares with AltosUI's export beside it, and its board's estimates with each
other ([above](#altos-telemetry)); how each AltOS `.eeprom` file compares with the export and the
telemetry beside it ([above](#altoss-eeprom)); how each ARTS2 `.odf` compares with the raw CSV
and the interpreted export beside it ([above](#the-arts2s-raw-data)); and
per file an anonymized id (the start of a hash of the file, which says nothing of its contents)
with a digest, a hash, of the record read from it. The files inside the zip archives among them
are unpacked and counted too, each once: of the 29 in the 8 archives, 19 are in no other source
([zip archives](#zip-archives)). Of the 1,122 files it holds today, 933 are read, the 8 archives
counted as read when they unpack. The report labels 131 files, of layouts no reader knows yet,
for [M7.1c12](../decisions-and-roadmap.md#m7-1c12), the rest of the importers.

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
[M7.1c12](../decisions-and-roadmap.md#m7-1c12). Five files filed as generic CSV or text are
PerfectFlite's exports: four with no header row and one that names its temperature `Temp`. A
Blue Raven high-rate file with no date is held to the Blue Raven's reader; its low-rate partner,
which a person edited in a spreadsheet (a column with no name, two columns named in words the
app doesn't write), is refused, counted as damaged. Three more are an ARTS2's raw CSV, held to
its reader. One more is moved to a layout not read yet: a ground station's conversion of a
SillyGoose flight, its columns in another order under no header.

Some files aren't flight logs, and are counted apart as not logs, as pictures and maps are. By
id: four spreadsheets of a team's checklist for preparing a flight, its steps and no measurement,
and nine spreadsheets of a Blue Raven's deployment settings, a `Hardware_Channel:` row naming its
outputs. By family: Debrief's six made-up sample flights, whose first row says each isn't a
recording of anything, and twelve text files a flyer's own program wrote from a Blue Raven's
low-rate file, titled as that file's events. By id, since
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
