# Flight-log samples, read

Written by `cargo xtask logs` from `log-import.json`; don't edit. Every flight-log sample under `refs/` read by HPR Sim's importer (M7.1, ADR-251, ADR-253, ADR-254, ADR-255): the private collection's catalog, Debrief's fixtures and samples, and AerospaceNU's logs, each file once. Left out: `refs/research/public-flights`, a download cache whose flights' logs the collection catalogs, and RocketPy's processed flight tables, which no logger wrote. A family is the catalog's name for a layout; *unrecognized* means no reader knows the layout yet, and *damaged* a file of a known layout no reader can make sense of.

| Source | Files | Not met before |
|---|---:|---:|
| collection | 1056 | 1039 |
| debrief | 23 | 14 |
| aerospacenu | 66 | 50 |

| Family | Increment | Files | Read | Refused |
|---|---|---:|---:|---|
| aerospacenu-fcb-csv | M7.1c3 | 20 | 0 | 20 unrecognized |
| aerospacenu-sillygoose-txt | M7.1c3 | 15 | 0 | 15 unrecognized |
| altimetercloud-csv | M7.1c2 | 11 | 11 | none |
| altos-csv | M7.1a | 48 | 48 | none |
| altos-csv-damaged | damaged | 1 | 0 | 1 syntax |
| altos-eeprom | M7.1c3 | 14 | 0 | 14 unrecognized |
| altos-telem | M7.1c3 | 2 | 0 | 2 unrecognized |
| binary | M7.1c3 | 4 | 0 | 4 unrecognized |
| blueraven-csv | M7.1b | 80 | 80 | none |
| blueraven-events-txt | M7.1c3 | 12 | 0 | 12 unrecognized |
| blueraven-summary-csv | M7.1c3 | 23 | 0 | 23 unrecognized |
| csv | M7.1c3 | 115 | 2 | 1 syntax, 112 unrecognized |
| debrief-synthetic-csv | M7.1c3 | 6 | 0 | 6 unrecognized |
| e80-imu-txt | M7.1c3 | 54 | 0 | 54 unrecognized |
| eggtimer-csv | M7.1b | 43 | 43 | none |
| eggtimer-txt | M7.1c2 | 2 | 2 | none |
| empty | not a log | 1 | 0 | 1 unrecognized |
| entacore-aim-csv | M7.1c2 | 16 | 16 | none |
| entacore-xtra | M7.1c3 | 2 | 0 | 2 unrecognized |
| featherweight-summary-csv | M7.1c3 | 25 | 0 | 25 unrecognized |
| fwgps-csv | M7.1c1 | 40 | 40 | none |
| jpeg | not a log | 1 | 0 | 1 unrecognized |
| kml | M7.1c1 | 42 | 42 | none |
| kml-map | not a log | 1 | 1 | none |
| missileworks-rtx | M7.1c3 | 5 | 0 | 5 unrecognized |
| nmea | M7.1c1 | 12 | 12 | none |
| odf | M7.1c3 | 3 | 0 | 3 unrecognized |
| ods | M7.1c3 | 4 | 0 | 4 unrecognized |
| pdf | not a log | 1 | 0 | 1 unrecognized |
| perfectflite-csv | M7.1a | 14 | 14 | none |
| perfectflite-txt | M7.1a | 2 | 2 | none |
| pf2 | M7.1a | 175 | 175 | none |
| pfa | M7.1c2 | 25 | 25 | none |
| png | not a log | 6 | 0 | 6 unrecognized |
| raven-csv | M7.1a | 80 | 80 | none |
| raven-fipa | M7.1c3 | 57 | 0 | 57 unrecognized |
| rrc3-csv | M7.1b | 50 | 50 | none |
| rrc3-rff | M7.1c3 | 6 | 0 | 6 unrecognized |
| rtf | not a log | 3 | 0 | 3 unrecognized |
| txt | M7.1c3 | 39 | 0 | 39 unrecognized |
| xls | M7.1c3 | 1 | 0 | 1 unrecognized |
| xlsx | M7.1c3 | 32 | 0 | 32 unrecognized |
| xml | M7.1c3 | 2 | 0 | 2 unrecognized |
| zip | M7.1c3 | 8 | 0 | 8 unrecognized |
| **All** | | 1103 | 643 | |

Files whose catalog family isn't their layout, moved to their layout's:

- `9c5017ab68168088 altos-csv -> csv`: a spreadsheet's re-save of an AltOS export, no longer AltosUI's layout: its header edited to carry units, its `height` column deleted, and three columns holding formulas
- `76a7d8937258e946 csv -> altos-csv-damaged`: an AltosUI export whose header line is damaged: it names 25 columns, two of them run together, where its rows hold other counts, so no reader can tell which cell is which; refused
- `c7936ac4d86f259f kml -> kml-map`: a map of a flight made in a map program, not a logger's file: the tracks a simulation predicted for several launch times, with pins for the pad, the predicted events and where the rocket landed, and a line measuring the distance to it
- `1eac3d58c44943fa csv -> altimetercloud-csv`: AltimeterCloud's CSV of a Rocketry Ltd altimeter's flight, a Nano V1's or a Mercury's, filed as generic CSV by its extension
- `2d8063ec6a98cac0 csv -> altimetercloud-csv`: AltimeterCloud's CSV of a Rocketry Ltd altimeter's flight, a Nano V1's or a Mercury's, filed as generic CSV by its extension
- `3c6a8343d5570aa9 csv -> altimetercloud-csv`: AltimeterCloud's CSV of a Rocketry Ltd altimeter's flight, a Nano V1's or a Mercury's, filed as generic CSV by its extension
- `54f08de4175c5e72 csv -> altimetercloud-csv`: AltimeterCloud's CSV of a Rocketry Ltd altimeter's flight, a Nano V1's or a Mercury's, filed as generic CSV by its extension
- `6e78a74048da02e5 csv -> altimetercloud-csv`: AltimeterCloud's CSV of a Rocketry Ltd altimeter's flight, a Nano V1's or a Mercury's, filed as generic CSV by its extension
- `88fcb6bb17c5d9c6 csv -> altimetercloud-csv`: AltimeterCloud's CSV of a Rocketry Ltd altimeter's flight, a Nano V1's or a Mercury's, filed as generic CSV by its extension
- `8e234dc4a78fcd61 csv -> altimetercloud-csv`: AltimeterCloud's CSV of a Rocketry Ltd altimeter's flight, a Nano V1's or a Mercury's, filed as generic CSV by its extension
- `d3ac56fff9b7d7c0 csv -> altimetercloud-csv`: AltimeterCloud's CSV of a Rocketry Ltd altimeter's flight, a Nano V1's or a Mercury's, filed as generic CSV by its extension
- `f021107424bfa9ee csv -> altimetercloud-csv`: AltimeterCloud's CSV of a Rocketry Ltd altimeter's flight, a Nano V1's or a Mercury's, filed as generic CSV by its extension
- `f2233f659bd4b671 csv -> altimetercloud-csv`: AltimeterCloud's CSV of a Rocketry Ltd altimeter's flight, a Nano V1's or a Mercury's, filed as generic CSV by its extension
- `28a11da1eaf93e69 csv -> entacore-aim-csv`: Entacore AIM's CSV, each channel group beside its own time column, filed as generic CSV by its extension
- `46d4a9ba5d9b0a58 csv -> entacore-aim-csv`: Entacore AIM's CSV, each channel group beside its own time column, filed as generic CSV by its extension
- `4d8cf9e8c6f3aa1f csv -> entacore-aim-csv`: Entacore AIM's CSV, each channel group beside its own time column, filed as generic CSV by its extension
- `5dc17a47fd185243 csv -> entacore-aim-csv`: Entacore AIM's CSV, each channel group beside its own time column, filed as generic CSV by its extension
- `5f48e98c9d3b14ee csv -> entacore-aim-csv`: Entacore AIM's CSV, each channel group beside its own time column, filed as generic CSV by its extension
- `7890f092f27f7d6f csv -> entacore-aim-csv`: Entacore AIM's CSV, each channel group beside its own time column, filed as generic CSV by its extension
- `9b3616ebe985dd0b csv -> entacore-aim-csv`: Entacore AIM's CSV, each channel group beside its own time column, filed as generic CSV by its extension
- `9e4dc73ea980b34a csv -> entacore-aim-csv`: Entacore AIM's CSV, each channel group beside its own time column, filed as generic CSV by its extension
- `a39eb686e97b5997 csv -> entacore-aim-csv`: Entacore AIM's CSV, each channel group beside its own time column, filed as generic CSV by its extension
- `a67bdeaf23283bd7 csv -> entacore-aim-csv`: Entacore AIM's CSV, each channel group beside its own time column, filed as generic CSV by its extension
- `bd3c3e390d38cf68 csv -> entacore-aim-csv`: Entacore AIM's CSV, each channel group beside its own time column, filed as generic CSV by its extension
- `df76575057cffc1e csv -> entacore-aim-csv`: Entacore AIM's CSV, each channel group beside its own time column, filed as generic CSV by its extension
- `f5df1597457a39ea csv -> entacore-aim-csv`: Entacore AIM's CSV, each channel group beside its own time column, filed as generic CSV by its extension
- `fe5e01edf5f1202c csv -> entacore-aim-csv`: Entacore AIM's CSV, each channel group beside its own time column, filed as generic CSV by its extension
- `ff6326c2236a0723 csv -> entacore-aim-csv`: Entacore AIM's CSV, each channel group beside its own time column, filed as generic CSV by its extension

PerfectFlite's exports state no unit for their altitude. Of the 16 read, 13 peak at exactly the height of a `.pf2` beside them, which is in feet; read in feet, their climbs take 0.94 to 1.05 times as long as a fall in vacuum from the peak. The other 3 take 0.93 to 1.07 times, read in feet, and would take 0.51 to 0.59 times read in meters.

A `.pfa` states neither its rate nor its unit; its reader takes 10 samples a second, in feet, from PerfectFlite's documents. Of the 25 read, 25 hold as many heights as `NumSamps:` states and 25 state an `Apogee:` equal to their highest; 17 first reach 80 ft, the height that sets off launch detection, at their eighth sample, 0.7 s, and 8 first reach 40 ft, the least it can be set to, there.

Units no header states, as each file's reader decided them, or kept the column as written, and noted:

| Family | Decision | Files |
|---|---|---:|
| altimetercloud-csv | `Apogee time` read in seconds, from the altitude | 7 |
| altimetercloud-csv | `Landing time` read in seconds, from the altitude | 7 |
| altimetercloud-csv | `acceleration_total(mG)` read in thousandths of standard gravity, from its name and the Nano V1 manual | 7 |
| altimetercloud-csv | `acceleration_total(mG)` read in thousandths of standard gravity, from its name and the Nano V1 manual, whose layout has the same columns | 4 |
| altimetercloud-csv | `air_brake_percent` read in percent, from its name | 4 |
| altimetercloud-csv | `air_density` kept as written | 4 |
| altimetercloud-csv | `apogee_prediction` kept as written | 4 |
| altimetercloud-csv | `bmp_temp(x100)` read in hundredths of a degree Celsius, from its name and the values | 4 |
| altimetercloud-csv | `gyro_x`, `gyro_y` and `gyro_z` read in degrees per second, from the Nano V1 manual | 7 |
| altimetercloud-csv | `gyro_x`, `gyro_y` and `gyro_z` read in degrees per second, from the Nano V1 manual, whose layout has the same columns | 4 |
| altimetercloud-csv | `mt1_temp` kept as written | 4 |
| altimetercloud-csv | `pitch`, `roll`, `yaw` and `tilt` read in degrees, from the Nano V1 manual | 7 |
| altimetercloud-csv | `pitch`, `roll`, `yaw` and `tilt` read in degrees, from the Nano V1 manual, whose layout has the same columns | 4 |
| altimetercloud-csv | `target_alt` kept as written | 4 |
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
| eggtimer-txt | `ASL Alt.` read in feet, from its unit mark | 2 |
| eggtimer-txt | `Alt` read in feet, from the manual | 2 |
| eggtimer-txt | `Apogee` read in feet, from its unit mark | 2 |
| eggtimer-txt | `FVeloc` read in feet per second, from the manual | 2 |
| eggtimer-txt | `Filt_Alt` read in feet, from the manual | 2 |
| eggtimer-txt | `T @` and `Time @` read in seconds, from the manual | 2 |
| eggtimer-txt | `T` read in milliseconds, from the step | 2 |
| eggtimer-txt | `Veloc` read in feet per second, from the manual | 2 |
| entacore-aim-csv | `A Voltage` read in volts, assumed | 9 |
| entacore-aim-csv | `B Voltage` read in volts, assumed | 9 |
| entacore-aim-csv | `C Voltage` read in volts, assumed | 16 |
| entacore-aim-csv | `D Voltage` read in volts, assumed | 15 |
| entacore-aim-csv | `GPS MSL` read in meters, assumed | 13 |
| entacore-aim-csv | `GPS Time of Week` kept as written, in milliseconds | 16 |
| entacore-aim-csv | `Gyro X`, `Gyro Y` and `Gyro Z` read in degrees per second, from the manual | 9 |
| entacore-aim-csv | `Mag X`, `Mag Y` and `Mag Z` kept as written, in a unit no source bears out | 9 |
| entacore-aim-csv | `Pressure MSL` and `Pressure AGL` read in meters, from the pressure | 16 |
| entacore-aim-csv | `acceleration` read in g, from the manual | 7 |
| entacore-aim-csv | `acceleration`, `lat. X accel.` and `lat. Y accel.` read in g, from the manual | 9 |
| entacore-aim-csv | `comp. bat. voltage` read in volts, assumed | 16 |
| entacore-aim-csv | `eject. bat. voltage` read in volts, assumed | 16 |
| entacore-aim-csv | `lat` and `long` read in degrees, from the values | 13 |
| entacore-aim-csv | `pressure` read in pascals, from the values | 16 |
| entacore-aim-csv | `temperature` read in degrees Celsius, assumed | 16 |
| entacore-aim-csv | `time` read in seconds, from `GPS Time of Week` | 16 |
| fwgps-csv | `ALT` and `Altitude AGL` read in feet, from the manual alone | 1 |
| fwgps-csv | `ALT` and `Altitude AGL` read in feet, from the manual and the horizontal speed | 20 |
| fwgps-csv | `ALT` read in feet, from the manual alone | 2 |
| fwgps-csv | `ALT` read in feet, from the manual and the horizontal speed | 13 |
| fwgps-csv | `BATT` read in volts, from its range | 40 |
| fwgps-csv | `DATE` and `TIME` kept as written, in a time zone the file doesn't state | 12 |
| fwgps-csv | `GS Alt asl`, `TRACKER Alt asl`, `Found rocket altitude` and `GS_ASL_offset` read in feet, from the manual, the horizontal speed and `Alt AGL (ft)` | 4 |
| fwgps-csv | `HEAD` read in degrees, from the manual and its range | 40 |
| fwgps-csv | `HORZV` read in feet per second, from the manual alone | 3 |
| fwgps-csv | `HORZV` read in feet per second, from the manual and the distance between fixes | 37 |
| fwgps-csv | `UNIXTIME` kept as written, off `UTCTIME` by whole hours | 28 |
| fwgps-csv | `VERTV` read in feet per second, in the altitude's unit, from the change of altitude | 38 |
| fwgps-csv | `VERTV` read in feet per second, in the altitude's unit, from the manual alone | 2 |
| kml | `Seconds` read in seconds, from its name | 2 |
| kml | `UTC` read in hours, minutes and seconds of the UTC day, from its name and layout | 2 |
| kml | `alt` kept as written, as it equals the tuple's altitude at no point, so nothing in the file shows its unit | 3 |
| kml | `alt` read in meters, from the coordinates | 1 |
| kml | `coordinates` kept as written, as GPS Visualizer takes a source's altitude as meters unless the source's header names feet | 1 |
| kml | `coordinates` read in meters as the barometer's altitude above sea level, from the GPS altitude AltOS writes beside each point | 4 |
| kml | `coordinates` read in meters, from the KML standard | 41 |
| kml | `time` read in seconds, from the steps between points | 3 |
| kml | `when` read in UTC, from the KML standard | 2 |
| kml-map | `coordinates` kept as written, in a track whose altitude mode tells a viewer to ignore its altitude | 1 |
| nmea | `GSA` and `GSV` read in the time of the `GGA` or `RMC` before them, from the sentence order | 12 |
| nmea | `RMC date` read in UTC days, from status-`A` sentences only | 12 |
| pfa | `Data` read in feet, 0.1 s apart, from the manual | 25 |
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
| Eggtimer TRS | 2 |
| Entacore AIM | 16 |
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
| PerfectFlite Alt15K/WD | 25 |
| PerfectFlite Pnut | 39 |
| PerfectFlite SL100 | 35 |
| PerfectFlite SLCF | 101 |
| Rocketry Ltd Mercury | 4 |
| Rocketry Ltd Nano V1 | 7 |
| fw2kml | 14 |
