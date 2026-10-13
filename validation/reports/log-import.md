# Flight-log samples, read

Written by `cargo xtask logs` from `log-import.json`; don't edit. Every flight-log sample under `refs/` read by HPR Sim's importer (M7.1, ADR-251, ADR-253, ADR-254, ADR-255): the private collection's catalog, Debrief's fixtures and samples, and AerospaceNU's logs, each file once. Left out: `refs/research/public-flights`, a download cache whose flights' logs the collection catalogs, and RocketPy's processed flight tables, which no logger wrote. A family is the catalog's name for a layout; *unrecognized* means no reader knows the layout yet, and *damaged* a file of a known layout no reader can make sense of.

| Source | Files | Not met before |
|---|---:|---:|
| collection | 1056 | 1039 |
| debrief | 23 | 14 |
| aerospacenu | 66 | 50 |

| Family | Increment | Files | Read | Refused |
|---|---|---:|---:|---|
| aerospacenu-fcb-csv | M7.1c4 | 20 | 20 | none |
| aerospacenu-gs-txt | M7.1c4 | 4 | 4 | none |
| aerospacenu-sillygoose-gs-csv | M7.1c5 | 1 | 0 | 1 unrecognized |
| aerospacenu-sillygoose-txt | M7.1c3 | 14 | 14 | none |
| altimetercloud-csv | M7.1c2 | 11 | 11 | none |
| altos-csv | M7.1a | 48 | 48 | none |
| altos-csv-damaged | damaged | 1 | 0 | 1 syntax |
| altos-eeprom | M7.1c5 | 14 | 0 | 14 unrecognized |
| altos-telem | M7.1c5 | 2 | 0 | 2 unrecognized |
| arts2-csv | M7.1c3 | 4 | 4 | none |
| arts2-raw-csv | M7.1c5 | 3 | 0 | 3 unrecognized |
| binary | M7.1c5 | 4 | 0 | 4 unrecognized |
| blueraven-csv | M7.1b | 80 | 80 | none |
| blueraven-events-txt | M7.1c5 | 12 | 0 | 12 unrecognized |
| blueraven-summary-csv | M7.1c5 | 23 | 0 | 23 unrecognized |
| csv | M7.1c5 | 75 | 2 | 1 syntax, 72 unrecognized |
| debrief-synthetic-csv | M7.1c5 | 6 | 0 | 6 unrecognized |
| e80-imu-txt | M7.1c5 | 54 | 0 | 54 unrecognized |
| eggtimer-csv | M7.1b | 43 | 43 | none |
| eggtimer-rtf | M7.1c4 | 1 | 1 | none |
| eggtimer-txt | M7.1c2 | 2 | 2 | none |
| empty | not a log | 1 | 0 | 1 unrecognized |
| entacore-aim-csv | M7.1c2 | 16 | 16 | none |
| entacore-xtra | M7.1c5 | 2 | 0 | 2 unrecognized |
| featherweight-summary-csv | M7.1c5 | 25 | 0 | 25 unrecognized |
| fluctus-csv | M7.1c3 | 6 | 6 | none |
| fwgps-csv | M7.1c1 | 40 | 40 | none |
| jpeg | not a log | 1 | 0 | 1 unrecognized |
| kml | M7.1c1 | 42 | 42 | none |
| kml-map | not a log | 1 | 1 | none |
| missileworks-rtx | M7.1c5 | 5 | 0 | 5 unrecognized |
| nmea | M7.1c1 | 12 | 12 | none |
| odf | M7.1c5 | 3 | 0 | 3 unrecognized |
| ods | M7.1c5 | 4 | 0 | 4 unrecognized |
| pdf | not a log | 1 | 0 | 1 unrecognized |
| perfectflite-csv | M7.1a | 14 | 14 | none |
| perfectflite-txt | M7.1a | 2 | 2 | none |
| pf2 | M7.1a | 175 | 175 | none |
| pfa | M7.1c2 | 25 | 25 | none |
| phone-launch-csv | M7.1c4 | 27 | 27 | none |
| png | not a log | 6 | 0 | 6 unrecognized |
| raven-csv | M7.1a | 80 | 80 | none |
| raven-fipa | M7.1c5 | 57 | 0 | 57 unrecognized |
| rrc3-csv | M7.1b | 50 | 50 | none |
| rrc3-rff | M7.1c5 | 6 | 0 | 6 unrecognized |
| telemetry-rtf | M7.1c5 | 2 | 0 | 2 not-this-format |
| txt | M7.1c5 | 35 | 0 | 35 unrecognized |
| xls | M7.1c5 | 1 | 0 | 1 unrecognized |
| xlsx | M7.1c5 | 32 | 0 | 32 unrecognized |
| xml | M7.1c5 | 2 | 0 | 2 unrecognized |
| zip | M7.1c5 | 8 | 0 | 8 unrecognized |
| **All** | | 1103 | 719 | |

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
- `0bc847e6d3d9a9ee csv -> fluctus-csv`: Silicdyne Fluctus's CSV, a `sep=;` line then a semicolon-separated table whose names carry their units, filed as generic CSV by its extension
- `5f6d8d9b2a364465 csv -> fluctus-csv`: Silicdyne Fluctus's CSV, a `sep=;` line then a semicolon-separated table whose names carry their units, filed as generic CSV by its extension
- `aad887197af5df0e csv -> fluctus-csv`: Silicdyne Fluctus's CSV, a `sep=;` line then a semicolon-separated table whose names carry their units, filed as generic CSV by its extension
- `b8bd3d94f2a563d0 csv -> fluctus-csv`: Silicdyne Fluctus's CSV, a `sep=;` line then a semicolon-separated table whose names carry their units, filed as generic CSV by its extension
- `f544929da1867b18 csv -> fluctus-csv`: Silicdyne Fluctus's CSV, a `sep=;` line then a semicolon-separated table whose names carry their units, filed as generic CSV by its extension
- `f5ffeb304d9213e8 csv -> fluctus-csv`: Silicdyne Fluctus's CSV, a `sep=;` line then a semicolon-separated table whose names carry their units, filed as generic CSV by its extension
- `88b16ea55c9a6415 csv -> arts2-csv`: Ozark Aerospace ARTS2's interpreted data, a `# MET Acc Vel iAlt pAlt` header then comma-separated rows, filed as generic CSV by its extension
- `9559781b880bbec8 csv -> arts2-csv`: Ozark Aerospace ARTS2's interpreted data, a `# MET Acc Vel iAlt pAlt` header then comma-separated rows, filed as generic CSV by its extension
- `e5f252c299d9dbe5 csv -> arts2-csv`: Ozark Aerospace ARTS2's interpreted data, a `# MET Acc Vel iAlt pAlt` header then comma-separated rows, filed as generic CSV by its extension
- `f3e56b3adb3d1581 csv -> arts2-csv`: Ozark Aerospace ARTS2's interpreted data, a `# MET Acc Vel iAlt pAlt` header then comma-separated rows, filed as generic CSV by its extension
- `b4f7a3949e33ca0d csv -> arts2-raw-csv`: Ozark Aerospace ARTS2's raw data, a `MET,Acc(HI),Acc/Baro(LO),Baro(HI),State` header then the sensors' counts with no calibration in the file, filed as generic CSV by its extension; read with the ARTS2's own `.odf` files
- `c392977fab8558c2 csv -> arts2-raw-csv`: Ozark Aerospace ARTS2's raw data, a `MET,Acc(HI),Acc/Baro(LO),Baro(HI),State` header then the sensors' counts with no calibration in the file, filed as generic CSV by its extension; read with the ARTS2's own `.odf` files
- `eda453d88e88e9c5 csv -> arts2-raw-csv`: Ozark Aerospace ARTS2's raw data, a `MET,Acc(HI),Acc/Baro(LO),Baro(HI),State` header then the sensors' counts with no calibration in the file, filed as generic CSV by its extension; read with the ARTS2's own `.odf` files
- `1a5db9dc7064aff9 csv -> phone-launch-csv`: a phone app's `launch….csv`, a header opening `Seconds,Timestamp,AltitudeMSLGPS` then one row a sample, filed as generic CSV by its extension
- `1b9914f2b095a2ca csv -> phone-launch-csv`: a phone app's `launch….csv`, a header opening `Seconds,Timestamp,AltitudeMSLGPS` then one row a sample, filed as generic CSV by its extension
- `292cd2cbeeb151d1 csv -> phone-launch-csv`: a phone app's `launch….csv`, a header opening `Seconds,Timestamp,AltitudeMSLGPS` then one row a sample, filed as generic CSV by its extension
- `2eb51f234ada08a3 csv -> phone-launch-csv`: a phone app's `launch….csv`, a header opening `Seconds,Timestamp,AltitudeMSLGPS` then one row a sample, filed as generic CSV by its extension
- `34ffb9c40a52f8a5 csv -> phone-launch-csv`: a phone app's `launch….csv`, a header opening `Seconds,Timestamp,AltitudeMSLGPS` then one row a sample, filed as generic CSV by its extension
- `3682e0a8fb151b8f csv -> phone-launch-csv`: a phone app's `launch….csv`, a header opening `Seconds,Timestamp,AltitudeMSLGPS` then one row a sample, filed as generic CSV by its extension
- `385526be706eaacb csv -> phone-launch-csv`: a phone app's `launch….csv`, a header opening `Seconds,Timestamp,AltitudeMSLGPS` then one row a sample, filed as generic CSV by its extension
- `53116705370ee99f csv -> phone-launch-csv`: a phone app's `launch….csv`, a header opening `Seconds,Timestamp,AltitudeMSLGPS` then one row a sample, filed as generic CSV by its extension
- `53d675d508e843fa csv -> phone-launch-csv`: a phone app's `launch….csv`, a header opening `Seconds,Timestamp,AltitudeMSLGPS` then one row a sample, filed as generic CSV by its extension
- `5488847e996f7ed6 csv -> phone-launch-csv`: a phone app's `launch….csv`, a header opening `Seconds,Timestamp,AltitudeMSLGPS` then one row a sample, filed as generic CSV by its extension
- `5e38d29923392275 csv -> phone-launch-csv`: a phone app's `launch….csv`, a header opening `Seconds,Timestamp,AltitudeMSLGPS` then one row a sample, filed as generic CSV by its extension
- `6d3ba122c9852293 csv -> phone-launch-csv`: a phone app's `launch….csv`, a header opening `Seconds,Timestamp,AltitudeMSLGPS` then one row a sample, filed as generic CSV by its extension
- `80167395bac22d32 csv -> phone-launch-csv`: a phone app's `launch….csv`, a header opening `Seconds,Timestamp,AltitudeMSLGPS` then one row a sample, filed as generic CSV by its extension
- `810aa6083d876fff csv -> phone-launch-csv`: a phone app's `launch….csv`, a header opening `Seconds,Timestamp,AltitudeMSLGPS` then one row a sample, filed as generic CSV by its extension
- `81693d9be93bd905 csv -> phone-launch-csv`: a phone app's `launch….csv`, a header opening `Seconds,Timestamp,AltitudeMSLGPS` then one row a sample, filed as generic CSV by its extension
- `836ec61f238d97b3 csv -> phone-launch-csv`: a phone app's `launch….csv`, a header opening `Seconds,Timestamp,AltitudeMSLGPS` then one row a sample, filed as generic CSV by its extension
- `af9709847c264b99 csv -> phone-launch-csv`: a phone app's `launch….csv`, a header opening `Seconds,Timestamp,AltitudeMSLGPS` then one row a sample, filed as generic CSV by its extension
- `b1bc4b3a856d71f1 csv -> phone-launch-csv`: a phone app's `launch….csv`, a header opening `Seconds,Timestamp,AltitudeMSLGPS` then one row a sample, filed as generic CSV by its extension
- `c8405deebb6c61de csv -> phone-launch-csv`: a phone app's `launch….csv`, a header opening `Seconds,Timestamp,AltitudeMSLGPS` then one row a sample, filed as generic CSV by its extension
- `cb3c53e2a90c8811 csv -> phone-launch-csv`: a phone app's `launch….csv`, a header opening `Seconds,Timestamp,AltitudeMSLGPS` then one row a sample, filed as generic CSV by its extension
- `cba087e4c9ac6ef3 csv -> phone-launch-csv`: a phone app's `launch….csv`, a header opening `Seconds,Timestamp,AltitudeMSLGPS` then one row a sample, filed as generic CSV by its extension
- `cc9cb757ee5ff666 csv -> phone-launch-csv`: a phone app's `launch….csv`, a header opening `Seconds,Timestamp,AltitudeMSLGPS` then one row a sample, filed as generic CSV by its extension
- `d1da0707d6023367 csv -> phone-launch-csv`: a phone app's `launch….csv`, a header opening `Seconds,Timestamp,AltitudeMSLGPS` then one row a sample, filed as generic CSV by its extension
- `d516f89cea3600b4 csv -> phone-launch-csv`: a phone app's `launch….csv`, a header opening `Seconds,Timestamp,AltitudeMSLGPS` then one row a sample, filed as generic CSV by its extension
- `db13cb37b77732af csv -> phone-launch-csv`: a phone app's `launch….csv`, a header opening `Seconds,Timestamp,AltitudeMSLGPS` then one row a sample, filed as generic CSV by its extension
- `e556289cc743ecd9 csv -> phone-launch-csv`: a phone app's `launch….csv`, a header opening `Seconds,Timestamp,AltitudeMSLGPS` then one row a sample, filed as generic CSV by its extension
- `ec774588ec195de6 csv -> phone-launch-csv`: a phone app's `launch….csv`, a header opening `Seconds,Timestamp,AltitudeMSLGPS` then one row a sample, filed as generic CSV by its extension
- `3a36f081e5be9e9d rtf -> telemetry-rtf`: a radio telemetry stream, lines such as `255,T,…` in a layout no maker's document names yet, saved inside an RTF document; filed as RTF, not a log, by its extension
- `70415b30a59d4411 rtf -> eggtimer-rtf`: an Eggtimer TRS's terminal download, its settings, summary and `T,Alt,…` table, saved inside an RTF document; filed as RTF, not a log, by its extension
- `8720c50cdeaf8ad3 rtf -> telemetry-rtf`: a radio telemetry stream, lines such as `255,T,…` in a layout no maker's document names yet, saved inside an RTF document; filed as RTF, not a log, by its extension
- `88f33383ccf621a0 aerospacenu-sillygoose-txt -> aerospacenu-sillygoose-gs-csv`: a SillyGoose flight as a ground station's conversion wrote it: a `CONFIG` line, then 28 comma-separated columns with no header, in another order than the board's own log, so nothing in the file says which column is which; for the column mapper

PerfectFlite's exports state no unit for their altitude. Of the 16 read, 13 peak at exactly the height of a `.pf2` beside them, which is in feet; read in feet, their climbs take 0.94 to 1.05 times as long as a fall in vacuum from the peak. The other 3 take 0.93 to 1.07 times, read in feet, and would take 0.51 to 0.59 times read in meters.

A `.pfa` states neither its rate nor its unit; its reader takes 10 samples a second, in feet, from PerfectFlite's documents. Of the 25 read, 25 hold as many heights as `NumSamps:` states and 25 state an `Apogee:` equal to their highest; 17 first reach 80 ft, the height that sets off launch detection, at their eighth sample, 0.7 s, and 8 first reach 40 ft, the least it can be set to, there.

Units no header states, as each file's reader decided them, or kept the column as written, and noted:

| Family | Decision | Files |
|---|---|---:|
| aerospacenu-fcb-csv | `acc_x`, `acc_y` and `acc_z` read in meters per second squared, from the pad's gravity and the climb | 19 |
| aerospacenu-fcb-csv | `baro1_alt` read in meters, from the pressure's climb and the accelerometer's | 1 |
| aerospacenu-fcb-csv | `baro1_pres` kept as written | 1 |
| aerospacenu-fcb-csv | `baro1_pres` read in atmospheres, from its size and its step | 19 |
| aerospacenu-fcb-csv | `baro1_temp` kept as written | 20 |
| aerospacenu-fcb-csv | `baro2_pres` read in atmospheres, from its size and its step | 18 |
| aerospacenu-fcb-csv | `baro2_temp` kept as written | 18 |
| aerospacenu-fcb-csv | `baro_height` read in meters, from the pressure's climb and the accelerometer's | 1 |
| aerospacenu-fcb-csv | `baro_pres_avg` read in atmospheres, from its size and its step | 20 |
| aerospacenu-fcb-csv | `baro_temp_avg` kept as written | 20 |
| aerospacenu-fcb-csv | `baroalt_agl` read in meters, from the pressure's climb and the accelerometer's | 1 |
| aerospacenu-fcb-csv | `battery_voltage_1` kept as written | 2 |
| aerospacenu-fcb-csv | `battery_voltage_2` kept as written | 2 |
| aerospacenu-fcb-csv | `battery_voltage` kept as written | 18 |
| aerospacenu-fcb-csv | `dynamixel_length_1` kept as written | 2 |
| aerospacenu-fcb-csv | `dynamixel_length` kept as written | 2 |
| aerospacenu-fcb-csv | `gps_alt` kept as written | 8 |
| aerospacenu-fcb-csv | `gps_alt` read in meters, from the pressure's climb and the accelerometer's | 12 |
| aerospacenu-fcb-csv | `gps_lat_mod` and `gps_long_mod` kept as written | 4 |
| aerospacenu-fcb-csv | `gps_lat_mod` and `gps_long_mod` read in degrees, from `gps_lat` and `gps_long` | 16 |
| aerospacenu-fcb-csv | `gps_lat` and `gps_long` kept as written | 1 |
| aerospacenu-fcb-csv | `gps_lat` and `gps_long` read in NMEA 0183's degrees and minutes, ddmm.mmmm, from their ranges and their minutes | 15 |
| aerospacenu-fcb-csv | `gps_lat` and `gps_long` read in degrees, from the latitudes' range and the longitudes' | 4 |
| aerospacenu-fcb-csv | `heading` kept as written | 20 |
| aerospacenu-fcb-csv | `high_g_accel_x_real`, `high_g_accel_y_real` and `high_g_accel_z_real` kept as written | 20 |
| aerospacenu-fcb-csv | `high_g_accel_x`, `high_g_accel_y` and `high_g_accel_z` kept as written | 20 |
| aerospacenu-fcb-csv | `imu1_accel_x_real`, `imu1_accel_y_real` and `imu1_accel_z_real` kept as written | 2 |
| aerospacenu-fcb-csv | `imu1_accel_x_real`, `imu1_accel_y_real` and `imu1_accel_z_real` read in meters per second squared, from the pad's gravity and the sensor's scale | 18 |
| aerospacenu-fcb-csv | `imu1_accel_x`, `imu1_accel_y` and `imu1_accel_z` kept as written | 20 |
| aerospacenu-fcb-csv | `imu1_gyro_x_real`, `imu1_gyro_y_real` and `imu1_gyro_z_real` kept as written | 3 |
| aerospacenu-fcb-csv | `imu1_gyro_x_real`, `imu1_gyro_y_real` and `imu1_gyro_z_real` read in radians per second, from the board's turning and the sensor's scale | 17 |
| aerospacenu-fcb-csv | `imu1_gyro_x`, `imu1_gyro_y` and `imu1_gyro_z` kept as written | 20 |
| aerospacenu-fcb-csv | `imu1_mag_x_real`, `imu1_mag_y_real` and `imu1_mag_z_real` kept as written | 20 |
| aerospacenu-fcb-csv | `imu1_mag_x`, `imu1_mag_y` and `imu1_mag_z` kept as written | 20 |
| aerospacenu-fcb-csv | `imu2_accel_x_real`, `imu2_accel_y_real` and `imu2_accel_z_real` kept as written | 5 |
| aerospacenu-fcb-csv | `imu2_accel_x_real`, `imu2_accel_y_real` and `imu2_accel_z_real` read in meters per second squared, from the pad's gravity and the sensor's scale | 13 |
| aerospacenu-fcb-csv | `imu2_accel_x`, `imu2_accel_y` and `imu2_accel_z` kept as written | 18 |
| aerospacenu-fcb-csv | `imu2_gyro_x_real`, `imu2_gyro_y_real` and `imu2_gyro_z_real` kept as written | 4 |
| aerospacenu-fcb-csv | `imu2_gyro_x_real`, `imu2_gyro_y_real` and `imu2_gyro_z_real` read in radians per second, from the board's turning and the sensor's scale | 14 |
| aerospacenu-fcb-csv | `imu2_gyro_x`, `imu2_gyro_y` and `imu2_gyro_z` kept as written | 18 |
| aerospacenu-fcb-csv | `imu2_mag_x_real`, `imu2_mag_y_real` and `imu2_mag_z_real` kept as written | 18 |
| aerospacenu-fcb-csv | `imu2_mag_x`, `imu2_mag_y` and `imu2_mag_z` kept as written | 18 |
| aerospacenu-fcb-csv | `imu_accel_x_avg`, `imu_accel_y_avg` and `imu_accel_z_avg` kept as written | 7 |
| aerospacenu-fcb-csv | `imu_accel_x_avg`, `imu_accel_y_avg` and `imu_accel_z_avg` read in meters per second squared, from the pad's gravity and the IMUs | 13 |
| aerospacenu-fcb-csv | `imu_gyro_x_avg`, `imu_gyro_y_avg` and `imu_gyro_z_avg` kept as written | 4 |
| aerospacenu-fcb-csv | `imu_gyro_x_avg`, `imu_gyro_y_avg` and `imu_gyro_z_avg` read in radians per second, from the board's turning and the IMUs | 16 |
| aerospacenu-fcb-csv | `imu_mag_x_avg`, `imu_mag_y_avg` and `imu_mag_z_avg` kept as written | 20 |
| aerospacenu-fcb-csv | `pos_x` kept as written | 20 |
| aerospacenu-fcb-csv | `pos_y` kept as written | 20 |
| aerospacenu-fcb-csv | `pos_z` read in meters, from the pressure's climb and the accelerometer's | 20 |
| aerospacenu-fcb-csv | `timestamp_ms` kept as written | 7 |
| aerospacenu-fcb-csv | `timestamp_ms` read in milliseconds, from its spacing and the climb | 12 |
| aerospacenu-fcb-csv | `timestamp_s` read in milliseconds, from its spacing and the climb | 6 |
| aerospacenu-fcb-csv | `timestamp_s` read in seconds, from its spacing and the climb | 2 |
| aerospacenu-fcb-csv | `vel_x` kept as written | 20 |
| aerospacenu-fcb-csv | `vel_y` kept as written | 20 |
| aerospacenu-fcb-csv | `vel_z` kept as written | 2 |
| aerospacenu-fcb-csv | `vel_z` read in meters per second, from `pos_z` and the accelerometer | 18 |
| aerospacenu-fcb-csv | `vtg` kept as written | 20 |
| aerospacenu-gs-txt | `Alt Info & Cfg.barometer_2_pressure` read in atmospheres, from its size and its step | 4 |
| aerospacenu-gs-txt | `Alt Info & Cfg.barometer_pressure` read in atmospheres, from its size and its step | 4 |
| aerospacenu-gs-txt | `Alt Info & Cfg.ground_elev` kept as written, in a unit nothing in the file checks | 4 |
| aerospacenu-gs-txt | `Alt Info & Cfg.ground_temp` kept as written, in degrees Celsius or Fahrenheit alike | 4 |
| aerospacenu-gs-txt | `Alt Info & Cfg.pitot_ducer_press` kept as written, in a unit nothing in the file checks | 4 |
| aerospacenu-gs-txt | `Alt Info & Cfg.press_ref` read in atmospheres, from its size and the barometer's | 4 |
| aerospacenu-gs-txt | `GPS Passthrough.GPS Alt` kept as written, with no check of its unit but the barometer's | 1 |
| aerospacenu-gs-txt | `GPS Passthrough.Latitude` and `GPS Passthrough.Longitude` kept as written, holding no fix | 1 |
| aerospacenu-gs-txt | `Ground Station GPS.gs_alt` kept as written, with no check of its unit but the barometer's | 3 |
| aerospacenu-gs-txt | `Ground Station GPS.gs_baro` read in atmospheres, from its size and its step | 3 |
| aerospacenu-gs-txt | `Ground Station GPS.gs_lat` and `Ground Station GPS.gs_lon` kept as written, holding no fix | 1 |
| aerospacenu-gs-txt | `Ground Station GPS.gs_lat` and `Ground Station GPS.gs_lon` read in degrees, from their range and their fractions | 2 |
| aerospacenu-gs-txt | `Ground Station GPS.gs_temp` kept as written, in degrees Celsius or Fahrenheit alike | 3 |
| aerospacenu-gs-txt | `Groundstation Status.gs_alt` kept as written, with no check of its unit but the barometer's | 1 |
| aerospacenu-gs-txt | `Groundstation Status.gs_baro` read in atmospheres, from its size and its step | 1 |
| aerospacenu-gs-txt | `Groundstation Status.gs_lat` and `Groundstation Status.gs_lon` kept as written, holding no fix | 1 |
| aerospacenu-gs-txt | `Groundstation Status.gs_temp` kept as written, in degrees Celsius or Fahrenheit alike | 1 |
| aerospacenu-gs-txt | `Orientation.accel_x`, `Orientation.accel_y` and `Orientation.accel_z` kept as written, the pad's gravity and the barometer not both bearing meters per second squared out | 1 |
| aerospacenu-gs-txt | `Orientation.accel_x`, `Orientation.accel_y` and `Orientation.accel_z` read in meters per second squared, from the pad's gravity and the barometer | 3 |
| aerospacenu-gs-txt | `Orientation.angle_vertical` kept as written, with no accelerometer read in m/s² to check it by | 1 |
| aerospacenu-gs-txt | `Orientation.angle_vertical` read in degrees, from its range and the accelerometer | 3 |
| aerospacenu-gs-txt | `Orientation.magnetic_field_x`, `Orientation.magnetic_field_y` and `Orientation.magnetic_field_z` kept as written, with no quantity for a magnetic field | 4 |
| aerospacenu-gs-txt | `Orientation.roll`, `Orientation.pitch`, `Orientation.yaw`, `Orientation.orientation_rpy[0]`, `Orientation.orientation_rpy[1]` and `Orientation.orientation_rpy[2]` kept as written, with no check of their unit but their range | 4 |
| aerospacenu-gs-txt | `Orientation.rot_vel_x`, `Orientation.rot_vel_y` and `Orientation.rot_vel_z` kept as written, with no accelerometer read in m/s² | 1 |
| aerospacenu-gs-txt | `Orientation.rot_vel_x`, `Orientation.rot_vel_y` and `Orientation.rot_vel_z` read in radians per second, from the board's turning and its tilt | 3 |
| aerospacenu-gs-txt | `Position Data.course_over_ground` kept as written, in a unit nothing in the file checks | 4 |
| aerospacenu-gs-txt | `Position Data.fcb_altitude` read in meters, from the barometer and the coast | 4 |
| aerospacenu-gs-txt | `Position Data.fcb_battery_voltage` kept as written, in a unit nothing in the file checks | 4 |
| aerospacenu-gs-txt | `Position Data.fcb_latitude` and `Position Data.fcb_longitude` read in degrees, from their range and their fractions | 4 |
| aerospacenu-gs-txt | `Position Data.gps_alt` kept as written, with no check of its unit but the barometer's | 4 |
| aerospacenu-gs-txt | `Position Data.ground_speed` kept as written, in a unit nothing in the file checks | 4 |
| aerospacenu-gs-txt | `Position Data.temperature` kept as written, in degrees Celsius or Fahrenheit alike | 4 |
| aerospacenu-gs-txt | `Position Data.v_speed` read in meters per second, from the barometer and the coast | 4 |
| aerospacenu-gs-txt | `time_stamp_ms` read in milliseconds, from its name and the ground station's clock | 4 |
| aerospacenu-sillygoose-txt | `accelX`, `accelY` and `accelZ` read in meters per second squared, from the pad's gravity | 13 |
| aerospacenu-sillygoose-txt | `accelX`, `accelY` and `accelZ` read in standard gravities, × 9.806 65 m/s², from the pad's gravity | 1 |
| aerospacenu-sillygoose-txt | `altitudeM` read in meters, from its name | 2 |
| aerospacenu-sillygoose-txt | `angularVelRadS_x`, `angularVelRadS_y` and `angularVelRadS_z` kept as written | 1 |
| aerospacenu-sillygoose-txt | `gyroX`, `gyroY` and `gyroZ` kept as written | 1 |
| aerospacenu-sillygoose-txt | `gyroX`, `gyroY` and `gyroZ` read in degrees per second, from the board's turning | 2 |
| aerospacenu-sillygoose-txt | `gyroX`, `gyroY` and `gyroZ` read in radians per second, from the board's turning | 11 |
| aerospacenu-sillygoose-txt | `imuTemp` kept as written | 2 |
| aerospacenu-sillygoose-txt | `imuTemp` read in kelvin, from the values | 12 |
| aerospacenu-sillygoose-txt | `tempK` kept as written | 1 |
| aerospacenu-sillygoose-txt | `unfiltAlt` kept as written | 2 |
| aerospacenu-sillygoose-txt | `unfiltAlt` read in meters, from the pressure | 12 |
| aerospacenu-sillygoose-txt | `velocityMS` read in meters per second, from its name | 1 |
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
| arts2-csv | `Acc` read in meters per second squared, from `Vel` | 1 |
| arts2-csv | `Acc` read in standard gravities, 9.806 65 m/s², from `Vel` | 3 |
| arts2-csv | `MET` read in seconds, from the apogee | 4 |
| arts2-csv | `Vel` read in feet per second, from the apogee and the barometer's step | 3 |
| arts2-csv | `Vel` read in meters per second, from the apogee and the barometer's step | 1 |
| arts2-csv | `iAlt` read in feet, from `Vel` | 3 |
| arts2-csv | `iAlt` read in meters, from `Vel` | 1 |
| arts2-csv | `pAlt` read in feet, from its step and `Vel` | 3 |
| arts2-csv | `pAlt` read in meters, from its step and `Vel` | 1 |
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
| eggtimer-rtf | `ASL Alt.` kept as written, with no unit mark the reader knows | 1 |
| eggtimer-rtf | `Alt` read in feet, from the manual and the coast | 1 |
| eggtimer-rtf | `Apogee` kept as written, with no unit mark the reader knows | 1 |
| eggtimer-rtf | `FAlt` read in feet, from the manual and the coast | 1 |
| eggtimer-rtf | `FVeloc` kept as written, not read as a speed | 1 |
| eggtimer-rtf | `T` read in milliseconds, from the step and the coast | 1 |
| eggtimer-rtf | `Veloc` kept as written, not read as a speed | 1 |
| eggtimer-txt | `ASL Alt.` read in feet, from its unit mark | 2 |
| eggtimer-txt | `Alt` read in feet, from the manual and the coast | 2 |
| eggtimer-txt | `Apogee` read in feet, from its unit mark | 2 |
| eggtimer-txt | `FVeloc` read in feet per second, from the manual | 2 |
| eggtimer-txt | `Filt_Alt` read in feet, from the manual and the coast | 2 |
| eggtimer-txt | `T @` and `Time @` read in seconds, from the manual | 2 |
| eggtimer-txt | `T` read in milliseconds, from the step and the coast | 2 |
| eggtimer-txt | `Veloc` read in feet per second, from the manual | 2 |
| entacore-aim-csv | `A Voltage` read in volts, from the values | 9 |
| entacore-aim-csv | `B Voltage` read in volts, from the values | 9 |
| entacore-aim-csv | `C Voltage` read in volts, from the values | 16 |
| entacore-aim-csv | `D Voltage` read in volts, from the values | 15 |
| entacore-aim-csv | `GPS MSL` read in meters, from `Pressure MSL` | 13 |
| entacore-aim-csv | `GPS Time of Week` kept as written, in milliseconds, from the values | 16 |
| entacore-aim-csv | `Gyro X`, `Gyro Y` and `Gyro Z` read in degrees per second, from the manual | 9 |
| entacore-aim-csv | `Mag X`, `Mag Y` and `Mag Z` kept as written, in a unit no source bears out | 9 |
| entacore-aim-csv | `Pressure MSL` and `Pressure AGL` read in meters, from the pressure | 16 |
| entacore-aim-csv | `RSSI` kept as written, in dBm, from the manual | 7 |
| entacore-aim-csv | `SVs` kept as written, a count, from the manual | 13 |
| entacore-aim-csv | `acceleration` read in g, from the manual | 7 |
| entacore-aim-csv | `acceleration`, `lat. X accel.` and `lat. Y accel.` read in g, from the manual | 9 |
| entacore-aim-csv | `comp. bat. voltage` read in volts, from the values | 16 |
| entacore-aim-csv | `eject. bat. voltage` read in volts, from the values | 16 |
| entacore-aim-csv | `lat` and `long` read in degrees, from the values | 13 |
| entacore-aim-csv | `launch detect`, `apogee detect`, `A cont.`, `B cont.`, `C cont.`, `D cont.`, `A on.`, `B on`, `C on` and `D on` kept as written, flags, from the values | 16 |
| entacore-aim-csv | `pressure` read in pascals, from the values | 16 |
| entacore-aim-csv | `temperature` read in degrees Celsius, from the heights' unit and the manual, p33 | 16 |
| entacore-aim-csv | `time` read in seconds, from `GPS Time of Week` | 16 |
| fluctus-csv | `gpsAltMSL` kept as written, in a unit no source states | 6 |
| fluctus-csv | `gpsLat` read in degrees, from the manual and the values | 6 |
| fluctus-csv | `gpsLng` read in degrees, from the manual and the values | 6 |
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
| phone-launch-csv | `AltitudeMSLGPS` and `AltitudeMSLMaxGPS` kept as written, in a unit no second check settles | 27 |
| phone-launch-csv | `AltitudeRelativeBarometer` kept as written, holding no reading | 19 |
| phone-launch-csv | `AltitudeRelativeBarometer` kept as written, in a unit two checks don't settle | 1 |
| phone-launch-csv | `AltitudeRelativeBarometer` read in meters, from the pressure and the coast | 7 |
| phone-launch-csv | `AltitudeRelativeSerial` and `AltitudeRelativeMaxSerial` kept as written | 19 |
| phone-launch-csv | `Battery` kept as written | 27 |
| phone-launch-csv | `GMax` and `AltitudeRelativeMaxBarometer` kept as written, as running extremes | 19 |
| phone-launch-csv | `GMax`, `AltitudeRelativeMaxBarometer`, `NoGravGMax`, `VelocityBarometerMax` and `VelocityBarometerMin` kept as written, as running extremes | 8 |
| phone-launch-csv | `Gx`, `Gy`, `Gz` and `GTotal` kept as written, in a unit two checks don't settle | 21 |
| phone-launch-csv | `Gx`, `Gy`, `Gz` and `GTotal` read in standard gravity, from the pad and the boost | 6 |
| phone-launch-csv | `GyroX`, `GyroY` and `GyroZ` kept as written, in a unit two checks don't settle | 20 |
| phone-launch-csv | `GyroX`, `GyroY` and `GyroZ` read in radians per second, from the attitude and the full scale | 7 |
| phone-launch-csv | `Heading` read in degrees, from its range and its wrap's period | 27 |
| phone-launch-csv | `HorizontalResGPS` and `VerticalResGPS` kept as written | 27 |
| phone-launch-csv | `Latitude` and `Longitude` read in degrees, from the latitude's range and the longitude's | 27 |
| phone-launch-csv | `MagnetometerX`, `MagnetometerY`, `MagnetometerZ`, `HeadingX`, `HeadingY` and `HeadingZ` kept as written | 27 |
| phone-launch-csv | `NoGravGx`, `NoGravGy`, `NoGravGz` and `NoGravGTotal` kept as written, in a unit two checks don't settle | 2 |
| phone-launch-csv | `NoGravGx`, `NoGravGy`, `NoGravGz` and `NoGravGTotal` read in standard gravity, with the accelerometer's axes | 6 |
| phone-launch-csv | `Pitch` kept as written, in a unit two checks don't settle | 2 |
| phone-launch-csv | `Pitch` read in radians, from its range and gravity's direction | 25 |
| phone-launch-csv | `Record` and `Static` kept as written, as numbers with no unit | 19 |
| phone-launch-csv | `Record`, `Static`, `UsbState` and `Satellites` kept as written, as numbers with no unit | 8 |
| phone-launch-csv | `Reserved1` and `Reserved2` kept as written | 8 |
| phone-launch-csv | `Reserved1`, `Reserved2`, `Reserved3` and `Reserved4` kept as written | 8 |
| phone-launch-csv | `Revolutions` and `RevApogee` kept as written | 11 |
| phone-launch-csv | `Revolutions`, `RevApogee` and `RevolutionsClock` kept as written | 8 |
| phone-launch-csv | `Roll` kept as written, in a unit two checks don't settle | 3 |
| phone-launch-csv | `Roll` read in radians, from its range and gravity's direction | 24 |
| phone-launch-csv | `Temperature` kept as written, in a unit no two checks settle | 8 |
| phone-launch-csv | `Timestamp` read in milliseconds since 1970 UTC, from the clock and the date | 27 |
| phone-launch-csv | `UsbV1`, `UsbV2`, `UsbV3`, `UsbV4`, `UsbT1`, `UsbT2`, `UsbT3`, `UsbT4`, `UsbT5`, `UsbT6`, `UsbUR1`, `UsbUR2`, `UsbUR3`, `UsbUR4`, `UsbUR5`, `UsbUR6`, `UsbUR7` and `UsbUR8` kept as written | 7 |
| phone-launch-csv | `UsbV1`, `UsbV2`, `UsbV3`, `UsbV4`, `UsbT1`, `UsbT2`, `UsbT3`, `UsbT4`, `UsbUR1`, `UsbUR2`, `UsbUR3`, `UsbUR4`, `UsbUR5`, `UsbUR6`, `UsbUR7` and `UsbUR8` kept as written | 1 |
| phone-launch-csv | `VelocityBarometerRaw` read in meters per second, from the barometric altitude and the coast | 7 |
| phone-launch-csv | `VelocityBarometer` and `VelocityBarometerRaw` kept as written, in a unit two checks don't settle | 1 |
| phone-launch-csv | `VelocityBarometer` read in meters per second, from the barometric altitude and the coast | 7 |
| phone-launch-csv | `Yaw` kept as written, in a unit two checks don't settle | 17 |
| phone-launch-csv | `Yaw` read in radians, from its range and the magnetic field's direction | 10 |
| phone-launch-csv | `hPa` read in hectopascals, from its name and the pad | 8 |
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
| AerospaceNU SillyGoose | 13 |
| AerospaceNU flight computer board | 20 |
| AerospaceNU flight computer board, by its ground station | 4 |
| AltOS | 52 |
| AltOS TeleGPS-v1.0 | 3 |
| AltOS TeleGPS-v2.0 | 4 |
| AltOS TeleMetrum | 1 |
| AltOS TeleMetrum-v2.0 | 1 |
| Big Red Bee BeeLine | 2 |
| Eggtimer | 43 |
| Eggtimer Build_1.52 | 1 |
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
| Ozark Aerospace ARTS2 | 4 |
| PerfectFlite | 16 |
| PerfectFlite Alt15K/WD | 25 |
| PerfectFlite Pnut | 39 |
| PerfectFlite SL100 | 35 |
| PerfectFlite SLCF | 101 |
| Rocketry Ltd Mercury | 4 |
| Rocketry Ltd Nano V1 | 7 |
| Silicdyne Fluctus | 6 |
| SillyGooseV2 | 1 |
| fw2kml | 14 |
| phone app | 27 |
