# Flight-log samples, read

Written by `cargo xtask logs` from `log-import.json`; don't edit. Every flight-log sample under `refs/` read by HPR Sim's importer (M7.1, ADR-251, ADR-253): the private collection's catalog, Debrief's fixtures and samples, and AerospaceNU's logs, each file once. Left out: `refs/research/public-flights`, a download cache whose flights' logs the collection catalogs, and RocketPy's processed flight tables, which no logger wrote. A family is the catalog's name for a layout; *unrecognized* means no reader knows the layout yet, and *damaged* a file of a known layout no reader can make sense of.

| Source | Files | Not met before |
|---|---:|---:|
| collection | 1056 | 1039 |
| debrief | 23 | 14 |
| aerospacenu | 66 | 50 |

| Family | Increment | Files | Read | Refused |
|---|---|---:|---:|---|
| aerospacenu-fcb-csv | M7.1c2 | 20 | 0 | 20 unrecognized |
| aerospacenu-sillygoose-txt | M7.1c2 | 15 | 0 | 15 unrecognized |
| altimetercloud-csv | M7.1c2 | 1 | 0 | 1 unrecognized |
| altos-csv | M7.1a | 48 | 48 | none |
| altos-csv-damaged | damaged | 1 | 0 | 1 syntax |
| altos-eeprom | M7.1c2 | 14 | 0 | 14 unrecognized |
| altos-telem | M7.1c2 | 2 | 0 | 2 unrecognized |
| binary | M7.1c2 | 4 | 0 | 4 unrecognized |
| blueraven-csv | M7.1b | 80 | 80 | none |
| blueraven-events-txt | M7.1c2 | 12 | 0 | 12 unrecognized |
| blueraven-summary-csv | M7.1c2 | 23 | 0 | 23 unrecognized |
| csv | M7.1c2 | 140 | 2 | 1 syntax, 137 unrecognized |
| debrief-synthetic-csv | M7.1c2 | 6 | 0 | 6 unrecognized |
| e80-imu-txt | M7.1c2 | 54 | 0 | 54 unrecognized |
| eggtimer-csv | M7.1b | 43 | 43 | none |
| eggtimer-txt | M7.1c2 | 2 | 0 | 2 unrecognized |
| empty | not a log | 1 | 0 | 1 unrecognized |
| entacore-aim-csv | M7.1c2 | 1 | 0 | 1 unrecognized |
| entacore-xtra | M7.1c2 | 2 | 0 | 2 unrecognized |
| featherweight-summary-csv | M7.1c2 | 25 | 0 | 25 unrecognized |
| fwgps-csv | M7.1c1 | 40 | 40 | none |
| jpeg | not a log | 1 | 0 | 1 unrecognized |
| kml | M7.1c1 | 42 | 42 | none |
| kml-map | not a log | 1 | 1 | none |
| missileworks-rtx | M7.1c2 | 5 | 0 | 5 unrecognized |
| nmea | M7.1c1 | 12 | 12 | none |
| odf | M7.1c2 | 3 | 0 | 3 unrecognized |
| ods | M7.1c2 | 4 | 0 | 4 unrecognized |
| pdf | not a log | 1 | 0 | 1 unrecognized |
| perfectflite-csv | M7.1a | 14 | 14 | none |
| perfectflite-txt | M7.1a | 2 | 2 | none |
| pf2 | M7.1a | 175 | 175 | none |
| pfa | M7.1c2 | 25 | 0 | 25 unrecognized |
| png | not a log | 6 | 0 | 6 unrecognized |
| raven-csv | M7.1a | 80 | 80 | none |
| raven-fipa | M7.1c2 | 57 | 0 | 57 unrecognized |
| rrc3-csv | M7.1b | 50 | 50 | none |
| rrc3-rff | M7.1c2 | 6 | 0 | 6 unrecognized |
| rtf | not a log | 3 | 0 | 3 unrecognized |
| txt | M7.1c2 | 39 | 0 | 39 unrecognized |
| xls | M7.1c2 | 1 | 0 | 1 unrecognized |
| xlsx | M7.1c2 | 32 | 0 | 32 unrecognized |
| xml | M7.1c2 | 2 | 0 | 2 unrecognized |
| zip | M7.1c2 | 8 | 0 | 8 unrecognized |
| **All** | | 1103 | 589 | |

Files whose catalog family isn't their layout, moved to their layout's:

- `9c5017ab68168088 altos-csv -> csv`: a spreadsheet's re-save of an AltOS export, no longer AltosUI's layout: its header edited to carry units, its `height` column deleted, and three columns holding formulas
- `76a7d8937258e946 csv -> altos-csv-damaged`: an AltosUI export whose header line is damaged: it names 25 columns, two of them run together, where its rows hold other counts, so no reader can tell which cell is which; refused
- `c7936ac4d86f259f kml -> kml-map`: a map made before a flight: the tracks a simulation predicted for several launch times, with the pad, apogee, main and landing marked, and no measurement

PerfectFlite's exports state no unit for their altitude. Of the 16 read, 13 peak at exactly the height of a `.pf2` beside them, which is in feet; read in feet, their climbs take 0.94 to 1.05 times as long as a fall in vacuum from the peak. The other 3 take 0.93 to 1.07 times, read in feet, and would take 0.51 to 0.59 times read in meters.

Units no header states, as each file's reader decided them, or kept the column as written, and noted:

| Family | Decision | Files |
|---|---|---:|
| blueraven-csv | `Accel_X`, `Accel_Y` and `Accel_Z` read in standard gravities, from the manual and the pad | 35 |
| blueraven-csv | `Current` kept as written, in a unit no source states | 35 |
| blueraven-csv | `Gyro_X`, `Gyro_Y` and `Gyro_Z` read in degrees per second, from the manual and the summary | 35 |
| blueraven-csv | `Inertial_Altitude`, `Inertial_DR_Position` and `Inertial_CR_position` read in feet, from the manual and the summary | 45 |
| blueraven-csv | `Velocity_Up`, `Velocity_DR` and `Velocity_CR` read in feet per second, from the manual and the summary | 45 |
| eggtimer-csv | `AltAccel` read in feet, from the manual | 4 |
| eggtimer-csv | `Alt` read in feet, from the manual | 43 |
| eggtimer-csv | `FAccelAlt` read in feet, from the manual | 4 |
| eggtimer-csv | `FAccelVel` read in feet per second, from the manual | 4 |
| eggtimer-csv | `FAlt` read in feet, from the manual | 38 |
| eggtimer-csv | `FVeloc` read in feet per second, from the manual | 43 |
| eggtimer-csv | `Filt_Alt` read in feet, from the manual | 5 |
| eggtimer-csv | `T` read in milliseconds, from the step | 3 |
| eggtimer-csv | `T` read in seconds, from the step | 40 |
| eggtimer-csv | `VAccel` read in feet per second, from the manual | 4 |
| eggtimer-csv | `Veloc` read in feet per second, from the manual | 40 |
| fwgps-csv | `ALT` and `Altitude AGL` read in feet, from the manual and the vertical speed | 21 |
| fwgps-csv | `ALT` read in feet, from the manual alone | 2 |
| fwgps-csv | `ALT` read in feet, from the manual and the vertical speed | 13 |
| fwgps-csv | `BATT` read in volts, from its range | 40 |
| fwgps-csv | `GS Alt asl`, `TRACKER Alt asl`, `Found rocket altitude` and `GS_ASL_offset` read in feet, from the manual and `Alt AGL (ft)` | 4 |
| fwgps-csv | `HEAD` read in degrees, from the manual and its range | 40 |
| fwgps-csv | `HORZV` read in feet per second, from the manual alone | 3 |
| fwgps-csv | `HORZV` read in feet per second, from the manual and the change of position | 37 |
| fwgps-csv | `UNIXTIME` kept as written, off `UTCTIME` by whole hours | 28 |
| fwgps-csv | `VERTV` read in feet per second, from the manual alone | 2 |
| fwgps-csv | `VERTV` read in feet per second, from the manual and the change of altitude | 38 |
| kml | `Seconds` read in seconds, from its name | 2 |
| kml | `UTC` read in hours, minutes and seconds of the UTC day, from its name and layout | 2 |
| kml | `alt` read in meters, from the coordinates | 4 |
| kml | `coordinates` kept as written, as GPS Visualizer takes a source's altitude as meters unless the source's header names feet | 1 |
| kml | `coordinates` read in meters, from the KML standard | 41 |
| kml | `time` read in seconds, from the steps between points | 3 |
| kml | `when` read in UTC, from the KML standard | 2 |
| kml-map | `coordinates` kept as written, in a track whose altitude mode tells a viewer to ignore its altitude | 1 |
| nmea | `GSA` and `GSV` read in the time of the `GGA` or `RMC` before them, from the sentence order | 12 |
| nmea | `RMC date` read in UTC days, from status-`A` sentences only | 12 |
| rrc3-csv | `Altitude` read in feet, from the pressure | 46 |
| rrc3-csv | `Pressure` read in hectopascals, from the values | 46 |
| rrc3-csv | `Temperature` read in degrees Fahrenheit, assumed | 40 |
| rrc3-csv | `Time` read in seconds, from the step | 46 |
| rrc3-csv | `Velocity` kept as written, its unit unknown | 2 |
| rrc3-csv | `Velocity` read in feet per second, from the altitude | 36 |
| rrc3-csv | `Velocity` read in miles per hour, from the altitude | 8 |
| rrc3-csv | `Voltages` read in volts, assumed | 42 |

The files read, by the logger each names:

| Logger | Files |
|---|---:|
| AltOS | 52 |
| AltOS TeleGPS-v1.0 | 3 |
| AltOS TeleGPS-v2.0 | 4 |
| AltOS TeleMetrum | 1 |
| AltOS TeleMetrum-v2.0 | 1 |
| Big Red Bee BeeLine | 2 |
| Eggtimer | 43 |
| Featherweight Blue Raven | 80 |
| Featherweight GPS ground station | 12 |
| Featherweight GPS tracker | 28 |
| Featherweight Raven | 80 |
| GPS Visualizer | 1 |
| KML | 3 |
| Missile Works RRC3 | 50 |
| Missile Works RTx | 12 |
| NMEA 0183 receiver (talker GP) | 12 |
| PerfectFlite | 16 |
| PerfectFlite Pnut | 39 |
| PerfectFlite SL100 | 35 |
| PerfectFlite SLCF | 101 |
| fw2kml | 14 |
