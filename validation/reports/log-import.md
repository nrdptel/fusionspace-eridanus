# Flight-log samples, read

Written by `cargo xtask logs` from `log-import.json`; don't edit. Every flight-log sample under `refs/` read by HPR Sim's importer (M7.1, ADR-251, ADR-253, ADR-254, ADR-255): the private collection's catalog, Debrief's fixtures and samples, and AerospaceNU's logs, each file once. Left out: `refs/research/public-flights`, a download cache whose flights' logs the collection catalogs, and RocketPy's repository under `refs/rocketpy`, whose data folder holds tables RocketPy processed; the teams' logs from it that the collection catalogs are read like any other. A family is the catalog's name for a layout; *unrecognized* means no reader knows the layout yet, and *damaged* a file of a known layout no reader can make sense of.

| Source | Files | Not met before |
|---|---:|---:|
| collection | 1056 | 1039 |
| debrief | 23 | 14 |
| aerospacenu | 66 | 50 |
| archives | 29 | 19 |

| Family | Increment | Files | Read | Refused |
|---|---|---:|---:|---|
| aerospacenu-fcb-csv | M7.1c4 | 20 | 20 | none |
| aerospacenu-gs-txt | M7.1c4 | 4 | 4 | none |
| aerospacenu-sillygoose-txt | M7.1c3 | 14 | 14 | none |
| altimax-xls | M7.1c20 | 1 | 1 | none |
| altimetercloud-csv | M7.1c2 | 11 | 11 | none |
| altos-csv | M7.1a | 48 | 48 | none |
| altos-csv-damaged | damaged | 1 | 0 | 1 syntax |
| altos-eeprom | M7.1c10 | 14 | 14 | none |
| altos-resaved-csv | M7.1c21 | 1 | 1 | none |
| altos-table-csv | M7.1c21 | 1 | 1 | none |
| altos-telem | M7.1c9 | 2 | 2 | none |
| altos-telem-csv | M7.1c21 | 1 | 1 | none |
| arts2-csv | M7.1c3 | 4 | 4 | none |
| arts2-raw-csv | M7.1c11 | 3 | 3 | none |
| binary | M7.1c23 | 2 | 0 | 2 unrecognized |
| blueraven-csv | M7.1b | 89 | 89 | none |
| blueraven-events-txt | not a log | 17 | 0 | 17 unrecognized |
| blueraven-settings-csv | not a log | 1 | 0 | 1 unrecognized |
| blueraven-settings-xlsx | not a log | 10 | 0 | 10 archive |
| blueraven-summary-copy-txt | not a log | 1 | 0 | 1 unrecognized |
| blueraven-summary-csv | M7.1c6 | 26 | 26 | none |
| blueraven-usb-txt | M7.1c17 | 2 | 2 | none |
| checklist | not a log | 4 | 0 | 4 archive |
| console-capture | M7.1c16 | 3 | 3 | none |
| csv | M7.1c23 | 2 | 2 | none |
| debrief-synthetic-csv | not a log | 6 | 0 | 6 unrecognized |
| e80-imu-txt | M7.1c8 | 54 | 54 | none |
| eggtimer-csv | M7.1b | 44 | 44 | none |
| eggtimer-csv-damaged | damaged | 1 | 0 | 1 unit |
| eggtimer-rtf | M7.1c4 | 1 | 1 | none |
| eggtimer-txt | M7.1c2 | 2 | 2 | none |
| empty | not a log | 1 | 0 | 1 unrecognized |
| entacore-aim-csv | M7.1c2 | 16 | 16 | none |
| entacore-xtra | M7.1c23 | 2 | 0 | 2 unrecognized |
| featherweight-summary-csv | M7.1c6 | 25 | 25 | none |
| flight-computer-params-csv | not a log | 3 | 0 | 3 unit |
| flight-summary-xlsx | not a log | 3 | 0 | 3 archive |
| fluctus-csv | M7.1c3 | 6 | 6 | none |
| fwgps-csv | M7.1c1 | 40 | 40 | none |
| held-table | M7.1c22 | 20 | 20 | none |
| housekeeping-csv | not a log | 2 | 0 | 2 unrecognized |
| jpeg | not a log | 1 | 0 | 1 unrecognized |
| kml | M7.1c1 | 43 | 43 | none |
| kml-map | not a log | 1 | 1 | none |
| mapped-table-csv | M7.1c15 | 6 | 6 | none |
| marsa-dat | M7.1c21 | 2 | 2 | none |
| missileworks-rtx | M7.1c12 | 5 | 5 | none |
| nmea | M7.1c1 | 12 | 12 | none |
| odf | M7.1c11 | 3 | 3 | none |
| openrocket-simulation-csv | not a log | 1 | 0 | 1 unrecognized |
| pdf | not a log | 1 | 0 | 1 unrecognized |
| perfectflite-csv | M7.1a | 15 | 15 | none |
| perfectflite-txt | M7.1a | 6 | 6 | none |
| pf2 | M7.1a | 175 | 175 | none |
| pfa | M7.1c2 | 25 | 25 | none |
| phone-launch-csv | M7.1c4 | 27 | 27 | none |
| png | not a log | 6 | 0 | 6 unrecognized |
| radio-packets-rtf | M7.1c19 | 2 | 2 | none |
| radio-packets-txt | M7.1c19 | 2 | 2 | none |
| raven-csv | M7.1a | 80 | 80 | none |
| raven-fip-sheet-xlsx | M7.1c20 | 2 | 2 | none |
| raven-fipa | M7.1c5 | 57 | 57 | none |
| record-dump | M7.1c16 | 2 | 2 | none |
| rocket-parameters | not a log | 4 | 0 | 4 unrecognized |
| rrc3-csv | M7.1b | 51 | 51 | none |
| rrc3-csv-damaged | damaged | 1 | 0 | 1 syntax |
| rrc3-rff | M7.1c5 | 6 | 6 | none |
| rtx-csv | M7.1c12 | 5 | 5 | none |
| rtx-txt | M7.1c12 | 4 | 4 | none |
| table-csv | M7.1c13 | 31 | 31 | none |
| teensy-named-txt | M7.1c18 | 1 | 1 | none |
| teensy-tabs-txt | M7.1c18 | 7 | 7 | none |
| track-copy-csv | not a log | 2 | 0 | 2 unrecognized |
| wind-profile-csv | not a log | 1 | 0 | 1 unrecognized |
| workbook-xlsx | M7.1c14 | 16 | 16 | none |
| worked-out-table | not a log | 3 | 0 | 1 archive, 2 unrecognized |
| zip | M7.1c7 | 8 | 8 | none |
| **All** | | 1122 | 1048 | |

Files whose catalog family isn't their layout, moved to their layout's:

- `5f2b0f963a9e52e9 csv -> held-table`: a table whose header states too little or that has none, read by the column mapping stated for it (`validation/log-held-mappings.json`), each unit it states held to a physical check; filed as generic CSV by its extension
- `47f7e2fe2d0414a3 csv -> held-table`: a table whose header states too little or that has none, read by the column mapping stated for it (`validation/log-held-mappings.json`), each unit it states held to a physical check; filed as generic CSV by its extension
- `6416a31040c017d7 csv -> held-table`: a table whose header states too little or that has none, read by the column mapping stated for it (`validation/log-held-mappings.json`), each unit it states held to a physical check; filed as generic CSV by its extension
- `dc85b22c94b96291 csv -> held-table`: a table whose header states too little or that has none, read by the column mapping stated for it (`validation/log-held-mappings.json`), each unit it states held to a physical check; filed as generic CSV by its extension
- `0b81bee53c3096cd csv -> held-table`: a table whose header states too little or that has none, read by the column mapping stated for it (`validation/log-held-mappings.json`), each unit it states held to a physical check; filed as generic CSV by its extension
- `5cf819cc85147315 csv -> held-table`: a table whose header states too little or that has none, read by the column mapping stated for it (`validation/log-held-mappings.json`), each unit it states held to a physical check; filed as generic CSV by its extension
- `419f0ec872abda2d txt -> held-table`: a table with no header whose rows are written as lists, `[…, True, True]`, read by the column mapping stated for it (`validation/log-held-mappings.json`), each unit it states held to a physical check; filed as text by its extension
- `4ea3b7a8a4d4119c csv -> held-table`: a table whose header states too little or that has none, read by the column mapping stated for it (`validation/log-held-mappings.json`), each unit it states held to a physical check; filed as generic CSV by its extension
- `f77449557bb81617 csv -> held-table`: a table whose header states too little or that has none, read by the column mapping stated for it (`validation/log-held-mappings.json`), each unit it states held to a physical check; filed as generic CSV by its extension
- `238756c53758ae7c txt -> held-table`: a table whose header states too little or that has none, read by the column mapping stated for it (`validation/log-held-mappings.json`), each unit it states held to a physical check; filed as text by its extension
- `055206ce7e9a2fae txt -> held-table`: a table whose header states too little or that has none, read by the column mapping stated for it (`validation/log-held-mappings.json`), each unit it states held to a physical check; filed as text by its extension
- `f1788c6722a77dd6 txt -> held-table`: a table whose header states too little or that has none, read by the column mapping stated for it (`validation/log-held-mappings.json`), each unit it states held to a physical check; filed as text by its extension
- `5ab193c0ed84f6b4 txt -> held-table`: a table with no header whose rows each open with a date and time, `YYYY-MM-DD HH:MM:SS.ffffff: …`, read by the column mapping stated for it (`validation/log-held-mappings.json`), each unit it states held to a physical check; filed as text by its extension
- `a601fc98238e2546 txt -> held-table`: a table with no header whose rows each open with a date and time, `YYYY-MM-DD HH:MM:SS.ffffff: …`, read by the column mapping stated for it (`validation/log-held-mappings.json`), each unit it states held to a physical check; filed as text by its extension
- `4bdad9db3e40a5b3 csv -> held-table`: a table whose header states too little or that has none, read by the column mapping stated for it (`validation/log-held-mappings.json`), each unit it states held to a physical check; filed as generic CSV by its extension
- `5843d20becfb8cda csv -> held-table`: a table whose header states too little or that has none, read by the column mapping stated for it (`validation/log-held-mappings.json`), each unit it states held to a physical check; filed as generic CSV by its extension
- `73304debd898b712 csv -> held-table`: a table whose header states too little or that has none, read by the column mapping stated for it (`validation/log-held-mappings.json`), each unit it states held to a physical check; filed as generic CSV by its extension
- `9b45103dd53075fa csv -> held-table`: a table whose header states too little or that has none, read by the column mapping stated for it (`validation/log-held-mappings.json`), each unit it states held to a physical check; filed as generic CSV by its extension
- `268517db228becd0 xlsx -> held-table`: an Excel workbook, `.xlsx`, whose first sheet is an altimeter's table pasted in with cells about the flight beside it, read by the column mapping stated for it (`validation/log-held-mappings.json`), each unit it states held to a physical check; filed as a spreadsheet
- `ff3c6deef6ab4527 csv -> housekeeping-csv`: a flight computer's housekeeping: its processor's load, its memory, disk and network counters and its core's temperature twice a second, and no measurement of the flight; not a log
- `a9c693adac127305 csv -> housekeeping-csv`: a battery pack's housekeeping: its cells' voltages, its temperatures and its current once a second, and no measurement of the flight; not a log
- `b4235128d1b200ce csv -> track-copy-csv`: coordinates with no clock: a GPS track's latitude, longitude and altitude as its map file holds them, the times left out; the tracker's own file beside it holds the same fixes with their times; not a log
- `2d2692ead2e72287 csv -> track-copy-csv`: coordinates with no clock: a GPS track's latitude, longitude and altitude as its map file holds them, the times left out; the tracker's own file beside it holds the same fixes with their times; not a log
- `3a002cd6365ebabc xlsx -> worked-out-table`: a filter's output in a workbook: a Kalman filter's state and its standard deviations by time, worked out from a log; its altitude runs to 169 km and its speed to 87 km/s, so the filter diverged; not a log
- `8539ed7d61a84b2d csv -> worked-out-table`: a filter's output: an inertial unit's recording turned into the rocket's frame with gravity taken out and cut to 30 s, worked out from a recording the collection doesn't hold; not a log
- `dc65924fec7f15b8 csv -> worked-out-table`: a flyer's rebuild of the altimeter's log beside it: rows the flyer marks as made up, an altitude worked out again from the pressure and a speed from the altitude; not what the logger wrote, which the survey reads; not a log
- `6f5e20612831ac57 xlsx -> raven-fip-sheet-xlsx`: a Featherweight Raven's FIP export pasted into an Excel workbook, `.xlsx`, below no header line: each channel's clock, value and `b` status, a row shifting left where a channel stops, read under the header stated for it (`validation/log-headers.json`); filed as a spreadsheet
- `b2a86ded765053b4 xlsx -> raven-fip-sheet-xlsx`: a Featherweight Raven's FIP export pasted into an Excel workbook, `.xlsx`, below no header line: each channel's clock, value and `b` status, a row shifting left where a channel stops, read under the header stated for it (`validation/log-headers.json`); filed as a spreadsheet
- `444687655195f5c8 xls -> altimax-xls`: a Rocketronics AltiMAX flight as its PC software exports it to Excel: a bare `.xls` worksheet stream, its table headed `TIME,PRESS FILTER [hpa],HEIGHT FILTER [m],…`; filed as a spreadsheet
- `4be82002a95c68db xml -> marsa-dat`: a MARSA altimeter's flight download: its settings as `<NAME> value` lines, such as `<BARO_ZERO 0> 790`, then a row a sample of the time, the barometric altitude in feet and the acceleration in g, as its software saves it to a `.dat` file or a spreadsheet re-saves it; filed as XML for its `<…>` lines
- `a80063e417b87028 xml -> marsa-dat`: a MARSA altimeter's flight download: its settings as `<NAME> value` lines, such as `<BARO_ZERO 0> 790`, then a row a sample of the time, the barometric altitude in feet and the acceleration in g, as its software saves it to a `.dat` file or a spreadsheet re-saves it; filed as XML for its `<…>` lines
- `850771db6621eaf6 csv -> altos-telem-csv`: AltOS's radio telemetry decoded to a table, a TeleMega's `0x09` packets one a row under a header of the packet's fields, `serial,tick,ptype,state,…,height,crc`; filed as generic CSV by its extension
- `03adc04683e2884f csv -> altos-table-csv`: a table cut from an AltOS export, `time,state,state_name,acceleration,…`: AltOS's own column names with no `version,serial,flight` before them, read by its states; filed as generic CSV by its extension
- `cd6f44ad29c505e6 txt -> radio-packets-txt`: a team flight computer's own log of the radio telemetry packets it sends, `255:T:…:254` lines after a whole number and its filter's `Gain[…]` lines; filed as text by its extension
- `a10560cbf44516e5 txt -> radio-packets-txt`: the text a ground receiver wrote of one team's radio telemetry packets, `255:T:…:254` from the flight computer and `255:N:…:254` relayed, with GPS fixes, by a second transmitter, many cut or run together; filed as text by its extension
- `12f752fa1d21c27f txt -> teensy-tabs-txt`: a course's Teensy data logger, one sensor's reading a line: a tag, the clock in microseconds and the values, by tabs, `Alt` lines giving a pressure and an altitude; the course's *Flight Data* page lists it as "Teensy Data Logger"
- `2b8cca725a7b5fd6 txt -> teensy-tabs-txt`: a course's Teensy data logger, one sensor's reading a line: a tag, the clock in microseconds and the values, by tabs, `Alt` lines giving a pressure and an altitude; the course's *Flight Data* page lists it as "Teensy Data Logger"
- `34e2963eda0e95bc txt -> teensy-tabs-txt`: a course's Teensy data logger, one sensor's reading a line: a tag, the clock in microseconds and the values, by tabs, `Alt` lines giving a pressure and an altitude; the course's *Flight Data* page lists it as "Teensy Data Logger"
- `3f3c6284f174edfc txt -> teensy-tabs-txt`: a course's Teensy data logger, one sensor's reading a line: a tag, the clock in microseconds and the values, by tabs, `Alt` lines giving a pressure and an altitude; the course's *Flight Data* page lists it as "Teensy Data Logger"
- `425b1ac3fbe4f794 txt -> teensy-tabs-txt`: a course's Teensy data logger, one sensor's reading a line: a tag, the clock in microseconds and the values, by tabs, `Alt` lines giving a pressure and an altitude; the course's *Flight Data* page lists it as "Teensy Data Logger"
- `bfea33e67007ae0e txt -> teensy-tabs-txt`: a course's Teensy data logger, one sensor's reading a line: a tag, the clock in microseconds and the values, by tabs, `Alt` lines giving a pressure and an altitude; the course's *Flight Data* page lists it as "Teensy Data Logger"
- `d17b644dac464173 txt -> teensy-tabs-txt`: a course's Teensy data logger, one sensor's reading a line: a tag, the clock in microseconds and the values, by tabs, `Alt` lines giving a pressure and an altitude; the course's *Flight Data* page lists it as "Teensy Data Logger"
- `0bc1429eeabbbda7 txt -> teensy-named-txt`: a course's Teensy data logger, newer layout: lines stating each sensor's units, then one sensor's reading a line, a tag, `t`, the clock in microseconds and name–value pairs, by commas; the course's *Flight Data* page lists it as "New Teensy Data Logger"
- `455ecee4a48bd98e txt -> blueraven-usb-txt`: a Blue Raven's flight data as its USB port downloads it to a serial terminal, `@ LOG_HIR` or `@ LOG_LOW` lines each closed by a CRC, in the layout its manual describes
- `a106c197fc5f6dec txt -> blueraven-usb-txt`: a Blue Raven's flight data as its USB port downloads it to a serial terminal, `@ LOG_HIR` or `@ LOG_LOW` lines each closed by a CRC, in the layout its manual describes
- `4ee1c37b025e6a9f txt -> blueraven-summary-copy-txt`: a flyer's own program's copy of the Blue Raven summary CSV beside it, its rows written as a table of cells between `|`: a copy, as the program's events file beside it is a reading; the CSV reads
- `5c15ad3db182065c txt -> console-capture`: a serial console's capture of a team flight computer's shell dumping its memory, base64 blocks between `----++++//[[( name start )]]\\++++----` marker lines, read by the record layout stated for it (`validation/log-layouts.json`); filed as text by its extension
- `a964e00316c378d9 txt -> console-capture`: a serial console's capture of a team flight computer's shell dumping its memory, base64 blocks between `----++++//[[( name start )]]\\++++----` marker lines, read by the record layout stated for it (`validation/log-layouts.json`); filed as text by its extension
- `d304c5e63044e3e3 txt -> console-capture`: a serial console's capture of a team flight computer's shell dumping its memory, base64 blocks between `----++++//[[( name start )]]\\++++----` marker lines, read by the record layout stated for it (`validation/log-layouts.json`); filed as text by its extension
- `67f97f55050a505f binary -> record-dump`: a team flight computer's raw file of fixed-size binary records end to end, with no header or marks, read by the record layout stated for it (`validation/log-layouts.json`); filed as binary
- `8f291cf155079b3d binary -> record-dump`: a team flight computer's raw file of fixed-size binary records end to end, with no header or marks, read by the record layout stated for it (`validation/log-layouts.json`); filed as binary
- `045f5b9ed473b800 csv -> mapped-table-csv`: a table whose header names its clock with no unit, `t`, `ts` or `time`, read by the column mapping its publisher states (`validation/log-mappings.json`); filed as generic CSV by its extension
- `0c7808a84fb39c83 csv -> mapped-table-csv`: a table whose header names its clock with no unit, `t`, `ts` or `time`, read by the column mapping its publisher states (`validation/log-mappings.json`); filed as generic CSV by its extension
- `a9cf47fe2b2e5be0 csv -> mapped-table-csv`: a table whose header names its clock with no unit, `t`, `ts` or `time`, read by the column mapping its publisher states (`validation/log-mappings.json`); filed as generic CSV by its extension
- `d88a7fc53cbd55d1 csv -> mapped-table-csv`: a table whose header names its clock with no unit, `t`, `ts` or `time`, read by the column mapping its publisher states (`validation/log-mappings.json`); filed as generic CSV by its extension
- `dd5287b375e84be8 csv -> mapped-table-csv`: a table whose header names its clock with no unit, `t`, `ts` or `time`, read by the column mapping its publisher states (`validation/log-mappings.json`); filed as generic CSV by its extension
- `f912724676eea0ca csv -> mapped-table-csv`: a table whose header names its clock with no unit, `t`, `ts` or `time`, read by the column mapping its publisher states (`validation/log-mappings.json`); filed as generic CSV by its extension
- `7a71bfeb7d7b1ee4 csv -> eggtimer-csv-damaged`: an Eggtimer's CSV download saved with semicolons by a spreadsheet that took out its decimal points, `12.34` written `1234`: its filtered altitude is about 100 times `Alt`, so the reader refuses it, its numbers not to be trusted
- `cda3fcdb9d634cd0 csv -> eggtimer-csv`: an Eggtimer's CSV download, `T;Alt;Veloc;…`, saved with semicolons between its cells, as a European locale's spreadsheet writes it; filed as generic CSV by its extension
- `bc062b34affee518 csv -> rrc3-csv-damaged`: an RRC3's mDACS export in a European locale's form, semicolons and decimal commas, whose rows a spreadsheet sorted by their time's text, `10,05` before `2`; put in order by value, they step 0.05 s from 0 with none missing, but the reader keeps the file's order and refuses a time that goes back, naming the line
- `02cf95e874f824f7 xlsx -> workbook-xlsx`: an Excel workbook, `.xlsx`, of a flight's log: a sheet or several, each a table a reader takes once the cells are written out as text, such as a Blue Raven's high-rate and low-rate files side by side or a table naming its units; filed as a spreadsheet
- `084653fe77f2ca50 xlsx -> workbook-xlsx`: an Excel workbook, `.xlsx`, of a flight's log: a sheet or several, each a table a reader takes once the cells are written out as text, such as a Blue Raven's high-rate and low-rate files side by side or a table naming its units; filed as a spreadsheet
- `229efb1695f3277f xlsx -> workbook-xlsx`: an Excel workbook, `.xlsx`, of a flight's log: a sheet or several, each a table a reader takes once the cells are written out as text, such as a Blue Raven's high-rate and low-rate files side by side or a table naming its units; filed as a spreadsheet
- `2d2455b75cfe7672 xlsx -> workbook-xlsx`: an Excel workbook, `.xlsx`, of a flight's log: a sheet or several, each a table a reader takes once the cells are written out as text, such as a Blue Raven's high-rate and low-rate files side by side or a table naming its units; filed as a spreadsheet
- `3e56a501d1b71a8a xlsx -> workbook-xlsx`: an Excel workbook, `.xlsx`, of a flight's log: a sheet or several, each a table a reader takes once the cells are written out as text, such as a Blue Raven's high-rate and low-rate files side by side or a table naming its units; filed as a spreadsheet
- `44a479aaaf2812ce xlsx -> workbook-xlsx`: an Excel workbook, `.xlsx`, of a flight's log: a sheet or several, each a table a reader takes once the cells are written out as text, such as a Blue Raven's high-rate and low-rate files side by side or a table naming its units; filed as a spreadsheet
- `8855863e76ea3fbc xlsx -> workbook-xlsx`: an Excel workbook, `.xlsx`, of a flight's log: a sheet or several, each a table a reader takes once the cells are written out as text, such as a Blue Raven's high-rate and low-rate files side by side or a table naming its units; filed as a spreadsheet
- `9ee24565e8232902 xlsx -> workbook-xlsx`: an Excel workbook, `.xlsx`, of a flight's log: a sheet or several, each a table a reader takes once the cells are written out as text, such as a Blue Raven's high-rate and low-rate files side by side or a table naming its units; filed as a spreadsheet
- `acc19f3b1096cb74 xlsx -> workbook-xlsx`: an Excel workbook, `.xlsx`, of a flight's log: a sheet or several, each a table a reader takes once the cells are written out as text, such as a Blue Raven's high-rate and low-rate files side by side or a table naming its units; filed as a spreadsheet
- `b27cc498f8d15caa xlsx -> workbook-xlsx`: an Excel workbook, `.xlsx`, of a flight's log: a sheet or several, each a table a reader takes once the cells are written out as text, such as a Blue Raven's high-rate and low-rate files side by side or a table naming its units; filed as a spreadsheet
- `c34428a0bff32b60 xlsx -> workbook-xlsx`: an Excel workbook, `.xlsx`, of a flight's log: a sheet or several, each a table a reader takes once the cells are written out as text, such as a Blue Raven's high-rate and low-rate files side by side or a table naming its units; filed as a spreadsheet
- `c3e5272b33d0ecdb xlsx -> workbook-xlsx`: an Excel workbook, `.xlsx`, of a flight's log: a sheet or several, each a table a reader takes once the cells are written out as text, such as a Blue Raven's high-rate and low-rate files side by side or a table naming its units; filed as a spreadsheet
- `d21437764a14c64b xlsx -> workbook-xlsx`: an Excel workbook, `.xlsx`, of a flight's log: a sheet or several, each a table a reader takes once the cells are written out as text, such as a Blue Raven's high-rate and low-rate files side by side or a table naming its units; filed as a spreadsheet
- `e3ff82bb5d72ff52 xlsx -> workbook-xlsx`: an Excel workbook, `.xlsx`, of a flight's log: a sheet or several, each a table a reader takes once the cells are written out as text, such as a Blue Raven's high-rate and low-rate files side by side or a table naming its units; filed as a spreadsheet
- `ef7b7dbbf32a8e67 xlsx -> workbook-xlsx`: an Excel workbook, `.xlsx`, of a flight's log: a sheet or several, each a table a reader takes once the cells are written out as text, such as a Blue Raven's high-rate and low-rate files side by side or a table naming its units; filed as a spreadsheet
- `f7b6e0a3273d6b79 xlsx -> workbook-xlsx`: an Excel workbook, `.xlsx`, of a flight's log: a sheet or several, each a table a reader takes once the cells are written out as text, such as a Blue Raven's high-rate and low-rate files side by side or a table naming its units; filed as a spreadsheet
- `331cacde5f5bbc07 xlsx -> flight-summary-xlsx`: a flyer's summary of flights in a spreadsheet, a few rows of names, dates and figures by flight or by launch, and no samples; not a log
- `9e277f9ff863b66e xlsx -> flight-summary-xlsx`: a flyer's summary of flights in a spreadsheet, a few rows of names, dates and figures by flight or by launch, and no samples; not a log
- `e972c010b8861aca xlsx -> flight-summary-xlsx`: a flyer's summary of flights in a spreadsheet, a few rows of names, dates and figures by flight or by launch, and no samples; not a log
- `ccf8bcecd91d5b91 csv -> table-csv`: a table no other reader takes, a header row naming its clock with a unit, such as `Time (s)`, then one row a sample, filed as generic CSV by its extension
- `061c084cf2d6155c csv -> table-csv`: a table no other reader takes, a header row naming its clock with a unit, such as `Time (s)`, then one row a sample, filed as generic CSV by its extension
- `065306f8cf7fc60c csv -> table-csv`: a table no other reader takes, a header row naming its clock with a unit, such as `Time (s)`, then one row a sample, filed as generic CSV by its extension
- `0e2537156abd776a csv -> table-csv`: a table no other reader takes, a header row naming its clock with a unit, such as `Time (s)`, then one row a sample, filed as generic CSV by its extension
- `111209c05b9be47d csv -> table-csv`: a table no other reader takes, a header row naming its clock with a unit, such as `Time (s)`, then one row a sample, filed as generic CSV by its extension
- `32deb5cb173b5983 csv -> table-csv`: a table no other reader takes, a header row naming its clock with a unit, such as `Time (s)`, then one row a sample, filed as generic CSV by its extension
- `4243bc06d5f49f2a csv -> table-csv`: a table no other reader takes, a header row naming its clock with a unit, such as `Time (s)`, then one row a sample, filed as generic CSV by its extension
- `469c003f029e47b4 csv -> table-csv`: a table no other reader takes, a header row naming its clock with a unit, such as `Time (s)`, then one row a sample, filed as generic CSV by its extension
- `4eef930c2b534a42 csv -> table-csv`: a table no other reader takes, a header row naming its clock with a unit, such as `Time (s)`, then one row a sample, filed as generic CSV by its extension
- `5428e790bc55cec5 csv -> table-csv`: a table no other reader takes, a header row naming its clock with a unit, such as `Time (s)`, then one row a sample, filed as generic CSV by its extension
- `5cb271496c8e5623 csv -> table-csv`: a table no other reader takes, a header row naming its clock with a unit, such as `Time (s)`, then one row a sample, filed as generic CSV by its extension
- `67fe4200832a39ad csv -> table-csv`: a table no other reader takes, a header row naming its clock with a unit, such as `Time (s)`, then one row a sample, filed as generic CSV by its extension
- `6821b3c5c90eeaf3 csv -> table-csv`: a table no other reader takes, a header row naming its clock with a unit, such as `Time (s)`, then one row a sample, filed as generic CSV by its extension
- `6bc2c4836a5a3504 csv -> table-csv`: a table no other reader takes, a header row naming its clock with a unit, such as `Time (s)`, then one row a sample, filed as generic CSV by its extension
- `76dad6a7c38d259b csv -> table-csv`: a table no other reader takes, a header row naming its clock with a unit, such as `Time (s)`, then one row a sample, filed as generic CSV by its extension
- `7e7df671ac0ab9ad csv -> table-csv`: a table no other reader takes, a header row naming its clock with a unit, such as `Time (s)`, then one row a sample, filed as generic CSV by its extension
- `82ebfd0a31998e39 csv -> table-csv`: a table no other reader takes, a header row naming its clock with a unit, such as `Time (s)`, then one row a sample, filed as generic CSV by its extension
- `89e3cfd45c66aa71 csv -> table-csv`: a table no other reader takes, a header row naming its clock with a unit, such as `Time (s)`, then one row a sample, filed as generic CSV by its extension
- `956ff350f72097d1 csv -> table-csv`: a table no other reader takes, a header row naming its clock with a unit, such as `Time (s)`, then one row a sample, filed as generic CSV by its extension
- `9c6ffdeffee7488a csv -> table-csv`: a table no other reader takes, a header row naming its clock with a unit, such as `Time (s)`, then one row a sample, filed as generic CSV by its extension
- `b348b373dbe7073f csv -> table-csv`: a table no other reader takes, a header row naming its clock with a unit, such as `Time (s)`, then one row a sample, filed as generic CSV by its extension
- `bdd32fb1d19bf31e csv -> table-csv`: a table no other reader takes, a header row naming its clock with a unit, such as `Time (s)`, then one row a sample, filed as generic CSV by its extension
- `cf949df321940506 csv -> table-csv`: a table no other reader takes, a header row naming its clock with a unit, such as `Time (s)`, then one row a sample, filed as generic CSV by its extension
- `cface22527bb390b csv -> table-csv`: a table no other reader takes, a header row naming its clock with a unit, such as `Time (s)`, then one row a sample, filed as generic CSV by its extension
- `dab6728a9ab22228 csv -> table-csv`: a table no other reader takes, a header row naming its clock with a unit, such as `Time (s)`, then one row a sample, filed as generic CSV by its extension
- `e79347ccfd0bd43b csv -> table-csv`: a table no other reader takes, a header row naming its clock with a unit, such as `Time (s)`, then one row a sample, filed as generic CSV by its extension
- `eafaeb9b290d7529 csv -> table-csv`: a table no other reader takes, a header row naming its clock with a unit, such as `Time (s)`, then one row a sample, filed as generic CSV by its extension
- `ebd3b757555b564c csv -> table-csv`: a table no other reader takes, a header row naming its clock with a unit, such as `Time (s)`, then one row a sample, filed as generic CSV by its extension
- `ec3a7fcc83121635 csv -> table-csv`: a table no other reader takes, a header row naming its clock with a unit, such as `Time (s)`, then one row a sample, filed as generic CSV by its extension
- `fd61caa7235b81b6 csv -> table-csv`: a table no other reader takes, a header row naming its clock with a unit, such as `Time (s)`, then one row a sample, filed as generic CSV by its extension
- `096375243584d110 csv -> rtx-csv`: Missile Works mDACS's CSV export of an RTx GPS tracker's fixes, a `UTC Time,Latitude,Longitude,Altitude,…` header then one row a fix, filed as generic CSV by its extension
- `1d34c9a2ea3264f0 csv -> rtx-csv`: Missile Works mDACS's CSV export of an RTx GPS tracker's fixes, a `UTC Time,Latitude,Longitude,Altitude,…` header then one row a fix, filed as generic CSV by its extension
- `2920bddb383455de csv -> rtx-csv`: Missile Works mDACS's CSV export of an RTx GPS tracker's fixes, a `UTC Time,Latitude,Longitude,Altitude,…` header then one row a fix, filed as generic CSV by its extension
- `3d1d6c4ef87ce0da csv -> rtx-csv`: Missile Works mDACS's CSV export of an RTx GPS tracker's fixes, a `UTC Time,Latitude,Longitude,Altitude,…` header then one row a fix, filed as generic CSV by its extension
- `fc937944649779db csv -> rtx-csv`: Missile Works mDACS's CSV export of an RTx GPS tracker's fixes, a `UTC Time,Latitude,Longitude,Altitude,…` header then one row a fix, filed as generic CSV by its extension
- `0cfb74a4c988654b txt -> rtx-txt`: Missile Works mDACS's text export of an RTx GPS tracker's fixes, a tab-separated `UTC Lat Lon Alt …` header then one row a fix, filed as text by its extension
- `74201deabc8b669a txt -> rtx-txt`: Missile Works mDACS's text export of an RTx GPS tracker's fixes, a tab-separated `UTC Lat Lon Alt …` header then one row a fix, filed as text by its extension
- `8b482eb6ae9af583 txt -> rtx-txt`: Missile Works mDACS's text export of an RTx GPS tracker's fixes, a tab-separated `UTC Lat Lon Alt …` header then one row a fix, filed as text by its extension
- `a65103199f08052b txt -> rtx-txt`: Missile Works mDACS's text export of an RTx GPS tracker's fixes, a tab-separated `UTC Lat Lon Alt …` header then one row a fix, filed as text by its extension
- `d306250ba73ea7f4 txt -> rrc3-csv`: an mDACS export of an RRC3's flight, tab-separated and in UTF-16, opening after its UTF-16 byte-order mark with a UTF-8 mark written as three UTF-16 characters; filed as text by its extension
- `9c5017ab68168088 altos-csv -> altos-resaved-csv`: a spreadsheet's re-save of an AltOS export, no longer AltosUI's layout: its header edited to carry units, its `height` column deleted, and three columns holding formulas; read by its columns' names, the formulas kept as text
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
- `b4f7a3949e33ca0d csv -> arts2-raw-csv`: Ozark Aerospace ARTS2's raw data, a `MET,Acc(HI),Acc/Baro(LO),Baro(HI),State` header then the sensors' counts with no calibration in the file, filed as generic CSV by its extension; the flight's `.odf` holds the same records with the calibration
- `c392977fab8558c2 csv -> arts2-raw-csv`: Ozark Aerospace ARTS2's raw data, a `MET,Acc(HI),Acc/Baro(LO),Baro(HI),State` header then the sensors' counts with no calibration in the file, filed as generic CSV by its extension; the flight's `.odf` holds the same records with the calibration
- `eda453d88e88e9c5 csv -> arts2-raw-csv`: Ozark Aerospace ARTS2's raw data, a `MET,Acc(HI),Acc/Baro(LO),Baro(HI),State` header then the sensors' counts with no calibration in the file, filed as generic CSV by its extension; the flight's `.odf` holds the same records with the calibration
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
- `3a36f081e5be9e9d rtf -> radio-packets-rtf`: one team's radio telemetry packets cut to the flight, saved as an RTF document and then by a spreadsheet as CSV, `255,T,…,254`, the document's markup in the first cells; filed as RTF by the catalog
- `70415b30a59d4411 rtf -> eggtimer-rtf`: an Eggtimer TRS's terminal download, its settings, summary and `T,Alt,…` table, saved inside an RTF document; filed as RTF, not a log, by its extension
- `8720c50cdeaf8ad3 rtf -> radio-packets-rtf`: one team's radio telemetry packets cut to the flight, saved as an RTF document and then by a spreadsheet as CSV, `255,T,…,254`, the document's markup in the first cells; filed as RTF by the catalog
- `88f33383ccf621a0 aerospacenu-sillygoose-txt -> held-table`: AerospaceNU's ground station's conversion of a SillyGoose's downlink, a `CONFIG` line then 28 comma-separated columns with no header in the order of the board's log with its settings, read by the column mapping stated for it (`validation/log-held-mappings.json`), each unit it states held to a physical check
- `0ad26ad4cfc8539b ods -> checklist`: a team's checklist for preparing a flight, its steps in a spreadsheet's cells and no measurement; not a log
- `34affe179ab66e72 ods -> checklist`: a team's checklist for preparing a flight, its steps in a spreadsheet's cells and no measurement; not a log
- `8e23a8dc0c55248b ods -> checklist`: a team's checklist for preparing a flight, its steps in a spreadsheet's cells and no measurement; not a log
- `c560a04cf2a5dccf ods -> checklist`: a team's checklist for preparing a flight, its steps in a spreadsheet's cells and no measurement; not a log
- `0f9a81a6d3406353 xlsx -> blueraven-settings-xlsx`: a Blue Raven's deployment settings saved as a spreadsheet, a `Hardware_Channel:` row naming its outputs (`Apo_Pri`, `Apo_Sec`, …) and their conditions; an altimeter's settings, not a flight's data
- `1887d4ff0c2ac130 xlsx -> blueraven-settings-xlsx`: a Blue Raven's deployment settings saved as a spreadsheet, a `Hardware_Channel:` row naming its outputs (`Apo_Pri`, `Apo_Sec`, …) and their conditions; an altimeter's settings, not a flight's data
- `40d01ca400d6085a xlsx -> blueraven-settings-xlsx`: a Blue Raven's deployment settings saved as a spreadsheet, a `Hardware_Channel:` row naming its outputs (`Apo_Pri`, `Apo_Sec`, …) and their conditions; an altimeter's settings, not a flight's data
- `526a2e2c94f25728 xlsx -> blueraven-settings-xlsx`: a Blue Raven's deployment settings saved as a spreadsheet, a `Hardware_Channel:` row naming its outputs (`Apo_Pri`, `Apo_Sec`, …) and their conditions; an altimeter's settings, not a flight's data
- `63543ab201ee2fbd xlsx -> blueraven-settings-xlsx`: a Blue Raven's deployment settings saved as a spreadsheet, a `Hardware_Channel:` row naming its outputs (`Apo_Pri`, `Apo_Sec`, …) and their conditions; an altimeter's settings, not a flight's data
- `936ac194735a318a xlsx -> blueraven-settings-xlsx`: a Blue Raven's deployment settings saved as a spreadsheet, a `Hardware_Channel:` row naming its outputs (`Apo_Pri`, `Apo_Sec`, …) and their conditions; an altimeter's settings, not a flight's data
- `afcf758d866f6e78 xlsx -> blueraven-settings-xlsx`: a Blue Raven's deployment settings saved as a spreadsheet, a `Hardware_Channel:` row naming its outputs (`Apo_Pri`, `Apo_Sec`, …) and their conditions; an altimeter's settings, not a flight's data
- `d2a7db8b9da843ef xlsx -> blueraven-settings-xlsx`: a Blue Raven's deployment settings saved as a spreadsheet, a `Hardware_Channel:` row naming its outputs (`Apo_Pri`, `Apo_Sec`, …) and their conditions; an altimeter's settings, not a flight's data
- `d3354f2054b6b18c xlsx -> blueraven-settings-xlsx`: a Blue Raven's deployment settings saved as a spreadsheet, a `Hardware_Channel:` row naming its outputs (`Apo_Pri`, `Apo_Sec`, …) and their conditions; an altimeter's settings, not a flight's data
- `77e28512645441b1 csv -> perfectflite-txt`: a PerfectFlite text export with no header row, tab-separated, each temperature written with its unit's letter; filed as CSV by its extension
- `0ed0a85f84b31849 txt -> perfectflite-txt`: a PerfectFlite text export with no header row, its temperatures in °F
- `c9bcbd7bdcf795bd txt -> perfectflite-txt`: a PerfectFlite text export with no header row, its temperatures in °F
- `06ba9b8df63e90ca txt -> perfectflite-txt`: a PerfectFlite text export with no header row, its temperatures in °C
- `a0f814e69afe42b6 csv -> perfectflite-csv`: a PerfectFlite CSV export whose header names its temperature `Temp`, without the text export's dot, each temperature written with its unit's letter
- `6ad288f13a3ec6d8 csv -> blueraven-csv`: a Blue Raven high-rate file whose header opens `Flight_Time_(s),Sync`, with no date or time of day
- `ff3ad892e63f060d csv -> table-csv`: a Blue Raven low-rate file a person edited in a spreadsheet: no date or time of day, a column with no name, and two columns named in words the app doesn't write; its names state their units, so it reads as a table by its header
- `12fd94ba690475b8 txt -> blueraven-events-txt`: a flyer's own program's events read from a Blue Raven's low-rate file, titled with the file it read
- `4aae566440b3f002 txt -> blueraven-events-txt`: a flyer's own program's events read from a Blue Raven's low-rate file, titled with the file it read
- `b26cc0ad79d74a7b txt -> rocket-parameters`: a flyer's own program's description of a rocket and its motor, `# Name = value` lines
- `b2f63e295244a1c5 txt -> rocket-parameters`: a flyer's own program's description of a rocket and its motor, `# Name = value` lines
- `386d1424975a627f txt -> rocket-parameters`: a team's description of a rocket for its own program, `name: value` lines
- `45df38a00638c4ae csv -> flight-computer-params-csv`: a team flight computer's saved state, one row: a boot count, gyro biases and the pressure before boost
- `ca5fce4c1fca4c14 csv -> flight-computer-params-csv`: a team flight computer's saved state, one row: a boot count, gyro biases and the pressure before boost
- `62b4c3516138c59a csv -> flight-computer-params-csv`: a team flight computer's saved state, one row: a boot count, gyro biases and the pressure before boost
- `35d8b51bbba48517 csv -> openrocket-simulation-csv`: an OpenRocket simulation's export, `# Simulation` comment lines then simulated values: a simulation, not a flight

PerfectFlite's exports state no unit for their altitude. Of the 21 read, 16 peak at exactly the height of a `.pf2` beside them, which is in feet; read in feet, their climbs take 0.94 to 1.05 times as long as a fall in vacuum from the peak. The other 5 take 0.63 to 1.07 times, read in feet, and would take 0.35 to 0.59 times read in meters.
By id, those 5, in feet and in meters: `0ed0a85f84b31849` 0.63 and 0.35; `1341d296b3f6b676` 1.07 and 0.59; `376b9bbc13b11ab7` 0.93 and 0.51; `6b9406226516e391` 0.93 and 0.51; `77e28512645441b1` 0.99 and 0.55.

A `.pfa` states neither its rate nor its unit; its reader takes 10 samples a second, in feet, from PerfectFlite's documents. Of the 25 read, 25 hold as many heights as `NumSamps:` states and 25 state an `Apogee:` equal to their highest; 17 first reach 80 ft, the height that sets off launch detection, at their eighth sample, 0.7 s, and 8 first reach 40 ft, the least it can be set to, there.

The raw downloads against their makers' exports of the same flight, an export being one beside the raw file, in its folder, that its reader reads. A match agrees in every sample of every channel the two share: the value within half a step of the raw format's resolution, the time as the export prints it (the FIP's to six significant figures; mDACS's from its own clock, 0.05 s summed in single precision, which runs a hundredth of a second ahead of the RRC3's from 156.65 s). Each raw sample is counted once; only the accelerometer's velocity may stop early, as the FIP stops it at apogee, and its samples after the stop are counted apart:

| Family | Files | With an export | Matching one | Raw samples compared | Velocity after the stop | Worst time, s | Worst value, steps | Export only | Differing |
|---|---:|---:|---:|---:|---:|---:|---:|---|---|
| raven-fipa | 57 | 55 | 55 | 6016104 | 140865 | 5.0e-4 | 0.000 | `LiftOff` (1), `[Altitude (Accel-Ft)]` (38), `[Altitude (Baro-Ft-AGL)]` (55), `[Altitude (Baro-Ft-ASL)]` (55), `[Velocity (Accel-Ft/Sec)]` (38), `[Velocity (Accel-MPH)]` (38), `bILBA` (37) | none |
| rrc3-rff | 6 | 1 | 1 | 3541 | 0 | 1.0e-2 | 0.000 | `Altitude` (1), `Temperature` (1), `Velocity` (1), `Voltages` (1) | none |

The Featherweight summaries against the files of samples exported beside each, in its folder, that their reader reads: a Blue Raven's low-rate CSV, a GPS tracker's CSV, each holding an altitude. The stated maximum altitude is compared with a file's highest altitude above the pad (for a tracker, its highest GNSS altitude less the pad altitude the summary states), and the stated pad altitude with the median of a file's first 20 altitudes above sea level, each in SI; a figure agrees within 10%, and its ratio, summary over file, is the one nearest 1. A folder may hold other flights' files, so a summary may agree with none; one that would agree only were its feet meters contradicts its unit and fails the check:

| Logger | Summaries | With a file beside | Stating an apogee | Apogee agrees | Its ratios | Pad agrees | Its ratios | Agreeing in neither | Contradicted |
|---|---:|---:|---:|---:|---|---:|---|---|---|
| Featherweight Blue Raven | 39 | 36 | 36 | 35 | 0.9698 to 1.0091 | 35 | 0.9770 to 1.0032 | cd1bfe19ff70e17d | none |
| Featherweight GPS tracker | 12 | 11 | 5 | 5 | 0.9995 to 1.0000 | 10 | 0.9766 to 0.9999 | 413de085a44f4872 | none |

AltOS's radio telemetry, each file's packets as its record counts them:

| File | Packets heard | Failing the radio's CRC | Share | Power-ons | Pressure in pascals |
|---|---:|---:|---:|---:|---|
| 73115a1066a7294d | 18870 | 2561 | 13.6% | 1 | yes |
| a238a3c22470953a | 12131 | 498 | 4.1% | 3 | yes |

Against AltosUI's exports of the same flight, an export being an AltOS CSV beside the telemetry, in its folder, that its reader reads. The two clocks are the board's tick, which turns over every 655.36 s; their `boost` states say by how many turns they differ. At each tick both hold, the telemetry's pressure must equal the export's, but where the export's rows there repeat the pressure of the row before them, which are counted apart; at the pressures equal, the temperatures must agree within 0.05 K, half the export's step, and at every tick both hold a GPS fix, the positions within 0.01° and the GPS altitudes within 100 m, the export's row carrying its last fix; fixes on both sides that never meet fail. Of 2 telemetry files, 1 have an export beside them and 1 agree with one: of their 3927 sensor packets, 138 fall at a tick the export holds, 123 pressures equal and 15 at repeated rows; worst temperature 0.050 K; 39 GPS values compared, worst position 0.00006°, GPS altitude 3 m; differing: none.

The board's own estimates, sent in its telemetry, against each other: the median ratio of `height`'s climb to `speed` after burnout, from 0.9659 to 1.0006, and of `speed`'s change to `acceleration` in the climb, from 0.9393 to 1.0213; and the telemetry's highest `height` over the export's, from 0.9962 to 0.9962. Each is 1 if the three are in the units read, and must lie from 0.8 to 1.25; outside: none; too few fast packets for a ratio: none.

AltOS's raw downloads, `.eeprom`, each file's records as its record counts them, and against AltosUI's export and the board's radio telemetry of the same flight where they lie beside it:

| File | Form | Log format | Records | Checksums | Erased | Repeats dropped | After a silence | Last line cut | Pressure step | Climb ratio | Pressures vs export | Temperatures | Accelerations | Mega voltages | Export rows, without a sample | Pressures vs telemetry |
|---|---|---|---:|---|---:|---:|---:|---|---:|---:|---|---|---|---|---|---|
| 01f1e77df7e18bc1 | JSON | 1 | 4088 | 4088, 0 failing | 8 | 0 | 0 | no | 13.57 Pa | 0.859 | 2206, 0.0035 Pa | – | 2206, 0.0050 m/s² | – | 4118, 0 without | – |
| 0bc0d96357b9eb10 | text | 1 (from the product) | 5750 | none printed | 0 | 4 | 0 | yes | 13.57 Pa | 0.902 | 5029, 0.0050 Pa | – | 5029, 0.0050 m/s² | – | 5045, 0 without | – |
| 0ef41688774d1d2a | text | 7 | 5238 | none printed | 0 | 0 | 0 | no | 1.00 Pa | 1.221 | – | – | – | – | – | – |
| 2084c9945938a5e7 | text | 7 | 2182 | none printed | 0 | 0 | 0 | no | 1.00 Pa | 1.211 | – | – | – | – | – | – |
| 219767cba01f7d9d | text | 7 | 7231 | none printed | 0 | 0 | 0 | no | 1.00 Pa | 1.400 | 5799, 0.0000 Pa | 5799, 0.050 K | 5799, 0.0049 m/s² (zeroed at +1 g) | – | 7230, 0 without | – |
| 56da61ddf4935294 | JSON | 6 | 2389 | 2389, 0 failing | 11 | 0 | 0 | no | 1.00 Pa | – | 2384, 0.0000 Pa | 2384, 0.050 K | – | – | 2403, 0 without | – |
| 67417925a3f2a144 | JSON | 22 | 6877 | 6877, 0 failing | 3 | 0 | 0 | no | 1.00 Pa | 1.125 | 5228, 0.0000 Pa | 5228, 0.050 K | 5228, 0.0050 m/s² | 7000, 0.0050 V | 6907, 0 without | – |
| 9611014e6dae139c | JSON | 7 | 7483 | 7483, 0 failing | 5 | 0 | 0 | no | 1.00 Pa | 0.884 | – | – | – | – | – | 108 of 108 |
| 9e001801c8f2b8e1 | text | 7 | 13123 | none printed | 0 | 0 | 0 | no | 1.00 Pa | 1.340 | 10262, 0.0000 Pa | 10262, 0.050 K | 10262, 0.0050 m/s² | – | 13122, 0 without | 123 of 123 |
| a24b048cc7f450cf | JSON | 16 | 1777 | 1777, 0 failing | 7 | 0 | 0 | no | 1.00 Pa | 0.971 | 1592, 0.0000 Pa | 1592, 0.050 K | 1592, 0.0050 m/s² | 1253, 0.0049 V | 1807, 0 without | – |
| ae3260cae4174bc2 | JSON | 15 | 3388 | 3388, 0 failing | 4 | 0 | 0 | no | 1.00 Pa | 0.907 | 2979, 0.0000 Pa | 2979, 0.050 K | 2979, 0.0050 m/s² | 1715, 0.0050 V | 3418, 0 without | – |
| b122d4bddf7a8a89 | text | 1 (from the product) | 9848 | none printed | 0 | 15 | 14 | no | – | 1.988 | – | – | – | – | – | – |
| e5b35cdd0836c39e | text | 7 | 4027 | none printed | 0 | 0 | 0 | no | 1.00 Pa | 1.068 | – | – | – | – | – | – |
| f44c03b81cb9cf81 | JSON | 14 | 3435 | 3435, 0 failing | 5 | 0 | 0 | no | 1.00 Pa | – | 3429, 0.0000 Pa | 3429, 0.050 K | – | – | 3497, 0 without | – |

An export of the same flight is an AltosUI CSV in the file's folder whose `serial` and `flight` are the file's, read by its header's names apart from HPR Sim's AltOS CSV reader. The two line up by tick, the export's `time` counting from the flight record's tick, or, where it equals its `clock` at every row, from the tick itself. At each of the file's samples the export's last row at that hundredth must agree within half the export's print step (each cell above: the values compared and the worst difference): the pressure, the temperature where an MS5607 gives it, the acceleration, and a Mega's pyro, igniter, apogee and main voltages. An export whose acceleration differs from the file's by one constant, the zero AltosUI may set at the +1 g calibration instead of the flight record's ground reading, within half a step, is counted apart. The other way, every row of the export must fall at a tick where the file holds a sample of some channel. The telemetry is the board's `.telem` of the same serial number, its clock lined up by whole turns from the two `boost` events; every pressure at a tick a sensor packet holds must equal it. The pressure step is the smallest difference between two of the file's pressures. The climb ratio is the reader's climb check on a board with an accelerometer: the climb over the first second after boost by the pressure in the standard atmosphere over the climb by the acceleration integrated twice from rest, which the reader holds from 0.6 to 5/3, keeping the acceleration's counts as written outside it, and a TeleMetrum v1's pressure's too; inside it, over 11 files, it lies from 0.859 to 1.400; outside: b122d4bddf7a8a89 (1.988). A log format from the product is format 1 for a TeleMetrum v1 whose configuration states none. Of 14 files, 9 have an export beside them and 9 agree with it; zeroed at the +1 g calibration: 219767cba01f7d9d; 2 have telemetry beside them and 2 agree with it; differing: none.

ARTS2's `.odf` files, each against the files of its flight beside it: its raw CSV, word for word; DataAnalyzer's interpreted export, `Acc` in the accelerometer's counts on every record after the first, and `pAlt` against the pressure one record earlier, as DataAnalyzer takes it, by order only (each pressure meeting one `pAlt`, zero at the pad's pressure and above and rising as the pressure falls below it, and no `pAlt` a gap), its difference from the standard atmosphere's reported only, as one export takes its altitude otherwise; and the pad's and the least pressure less an AltOS board's flown with it, kPa, reported only:

| File | Records | Climb ratio | Raw CSV | Rows compared | Worst `Acc`, counts | First row zero | Rows after | `pAlt` gaps | Clashes | Inversions | `pAlt` from the standard, m | Pad less AltOS, kPa | Least less AltOS, kPa |
|---|---:|---:|---|---:|---:|---|---|---:|---:|---:|---:|---:|---:|
| 206193a3811c8bc0 | 16384 | 0.754 | same | 16383 | 0.076 | yes | 1, zero | 0 | 0 | 0 | 151.2 | -1.08 | -0.48 |
| 2074af0d10ad080f | 16115 | 1.032 | same | 16114 | 0.080 | yes | 0, zero | 0 | 0 | 0 | 3.1 | -0.42 | +0.02 |
| a34c382592dfd50d | 16115 | 1.145 | same | 16114 | 0.076 | yes | 0, zero | 0 | 0 | 0 | 3.3 | -0.44 | +0.11 |

Differing: none.

Missile Works RTx's `.rtx` files, each decoded here from its bytes apart from the reader and held to the reader's record, field for field, and to the exports of its flight beside it: the CSV and text exports' rows and the KML's points every record but the last, their times and numbers equal, their positions equal or, written in degrees, cut toward zero at the sixth decimal (the largest difference given), their altitude the record's meters × 3.280 84 rounded down to a foot (the KML's, its meters as written). Reported only, the medians over the fixes where `SOG` is at least 15 of `SOG` over the speed between the fixes either side, m/s, and of how far `COG` lies from the bearing between them, degrees, either way; and the climb from the lowest of the 120 fixes before the highest to the highest, over the highest altitude of a PerfectFlite log flown beside the tracker:

| File | Records | Reader's record | Exports beside it | `SOG` over speed | `COG` off bearing, ° | Climb over barometric |
|---|---:|---|---|---:|---:|---:|
| 0ef081627f173419 | 2254 | yes | RTx CSV: 2253 rows, every record but the last, 0.00 × 10⁻⁶°, altitude as written; RTx text: 2253 rows, every record but the last, 0.00 × 10⁻⁶°, altitude as written; KML: 2253 rows, every record but the last, 0.83 × 10⁻⁶°, altitude as written | 4.00 | 3.8 | 0.979 |
| 338d96e93ea3dc64 | 4096 | yes | RTx CSV: 4095 rows, every record but the last, 0.00 × 10⁻⁶°, altitude as written; RTx text: 4095 rows, every record but the last, 0.00 × 10⁻⁶°, altitude as written; KML: 4095 rows, every record but the last, 0.83 × 10⁻⁶°, altitude as written | 3.54 | 4.3 | 1.071 |
| 4228b05ebc899f6d | 3126 | yes | RTx CSV: 3125 rows, every record but the last, 0.83 × 10⁻⁶°, altitude as written; RTx text: 3125 rows, every record but the last, 0.83 × 10⁻⁶°, altitude as written; KML: 3125 rows, every record but the last, 0.83 × 10⁻⁶°, altitude as written | 3.61 | 6.9 | 1.150 |
| 7636132a8ee49d52 | 4096 | yes | KML: 4095 rows, every record but the last, 0.83 × 10⁻⁶°, altitude as written | 3.48 | 1.7 | 0.003 |
| b1a16428685d2881 | 2949 | yes | RTx CSV: 2948 rows, every record but the last, 0.83 × 10⁻⁶°, altitude as written; RTx text: 2948 rows, every record but the last, 0.83 × 10⁻⁶°, altitude as written; KML: 2948 rows, every record but the last, 0.83 × 10⁻⁶°, altitude as written | 3.66 | 4.7 | 0.995 |

Differing: none.

The 31 tables read by their headers (M7.1c13): each column but the clock read in the unit its name states, kept as written by its header (no unit the reader knows, a name that says another quantity, an error or a difference, or a length, a time or a unit that may stand for another under a name that doesn't say what it measures), kept as written because a value contradicts its unit, or text; *Two units* is the largest difference, in SI, between two channels of one quantity under one name in two units. In all, 224 columns read in a stated unit, 364 kept by their headers, 5 by their values, 11 of text.

| File | Rows | Columns | Clock | Read | Kept by header | Contradicted | Text | Events | Two units |
|---|---:|---:|---|---:|---:|---:|---:|---:|---:|
| `061c084cf2d6155c` | 26336 | 26 | `s` | 12 | 13 | 0 | 0 | 0 | – |
| `065306f8cf7fc60c` | 13644 | 25 | `ms` | 11 | 13 | 0 | 0 | 0 | – |
| `0e2537156abd776a` | 1047 | 5 | `ms` | 1 | 2 | 1 | 0 | 0 | – |
| `111209c05b9be47d` | 12925 | 39 | `seconds` | 18 | 19 | 0 | 1 | 5 | – |
| `32deb5cb173b5983` | 370 | 38 | `s` | 14 | 22 | 1 | 0 | 0 | – |
| `4243bc06d5f49f2a` | 9000 | 3 | `ms` | 0 | 2 | 0 | 0 | 0 | – |
| `469c003f029e47b4` | 1195 | 7 | `s` | 6 | 0 | 0 | 0 | 0 | – |
| `4eef930c2b534a42` | 12838 | 25 | `ms` | 11 | 13 | 0 | 0 | 0 | – |
| `5428e790bc55cec5` | 634 | 4 | `s` | 1 | 2 | 0 | 0 | 0 | – |
| `5cb271496c8e5623` | 18904 | 17 | `s` | 5 | 10 | 0 | 1 | 6 | – |
| `67fe4200832a39ad` | 7646 | 34 | `seconds` | 14 | 18 | 0 | 1 | 5 | – |
| `6821b3c5c90eeaf3` | 9750 | 50 | `ms` | 23 | 26 | 0 | 0 | 0 | – |
| `6bc2c4836a5a3504` | 9000 | 23 | `ms` | 0 | 22 | 0 | 0 | 0 | – |
| `76dad6a7c38d259b` | 2401 | 4 | `s` | 3 | 0 | 0 | 0 | 0 | – |
| `7e7df671ac0ab9ad` | 530 | 38 | `s` | 13 | 23 | 1 | 0 | 0 | – |
| `82ebfd0a31998e39` | 2932 | 7 | `s` | 4 | 0 | 0 | 2 | 8 | – |
| `89e3cfd45c66aa71` | 3928 | 4 | `s` | 1 | 1 | 0 | 1 | 0 | – |
| `956ff350f72097d1` | 10965 | 34 | `seconds` | 14 | 18 | 0 | 1 | 5 | – |
| `9c6ffdeffee7488a` | 401 | 4 | `ms` | 3 | 0 | 0 | 0 | 0 | – |
| `b348b373dbe7073f` | 366 | 19 | `s` | 0 | 15 | 0 | 3 | 0 | – |
| `bdd32fb1d19bf31e` | 191 | 38 | `s` | 13 | 23 | 1 | 0 | 0 | – |
| `ccf8bcecd91d5b91` | 768 | 5 | `s` | 1 | 3 | 0 | 0 | 0 | – |
| `cf949df321940506` | 6616 | 37 | `seconds` | 17 | 18 | 0 | 1 | 5 | – |
| `cface22527bb390b` | 2601 | 4 | `s` | 3 | 0 | 0 | 0 | 0 | – |
| `dab6728a9ab22228` | 2201 | 4 | `s` | 3 | 0 | 0 | 0 | 0 | – |
| `e79347ccfd0bd43b` | 160 | 5 | `ms` | 1 | 2 | 1 | 0 | 0 | – |
| `eafaeb9b290d7529` | 815 | 3 | `s` | 1 | 1 | 0 | 0 | 0 | – |
| `ebd3b757555b564c` | 1139 | 5 | `s` | 4 | 0 | 0 | 0 | 0 | 4.55e-13 |
| `ec3a7fcc83121635` | 2401 | 4 | `s` | 3 | 0 | 0 | 0 | 0 | – |
| `fd61caa7235b81b6` | 13655 | 25 | `ms` | 11 | 13 | 0 | 0 | 0 | – |
| `ff3ad892e63f060d` | 36700 | 99 | `s` | 13 | 85 | 0 | 0 | 0 | – |

Eggtimer filtered altitudes against `Alt` (M7.1c15): over the 44 Eggtimer CSV files read with a filtered altitude, the least-squares factor through zero from `Alt` to it lies from 0.9000 (`e186d325c61f5968`) to 0.9999 (`2add1c81ab430ce7`); the reader refuses a file outside 0.1 to 2. Refused for it: `7a71bfeb7d7b1ee4`, Eggtimer CSV line 1: `Filt_Alt` isn't a filter of `Alt`: it is 99.934 times `Alt` by a least-squares fit, outside the 0.1 to 2 a filter of the altitude can be, so the file's numbers can't be trusted; a file whose decimal points were taken out writes `12.34` as `1234`, 100 times over.

The 6 tables read by the column mapping their publisher states (M7.1c15, `validation/log-mappings.json`, which says where each is stated): each column but the clock read in the unit the mapping states, kept as written where it states none, or kept as written because a value contradicts the unit stated.

| File | Rows | Columns | Clock | Read | Kept | Contradicted | Stated by |
|---|---:|---:|---|---:|---:|---:|---|
| `045f5b9ed473b800` | 746 | 2 | `s` | 1 | 0 | 0 | RocketPy 9bd6ad3, its example notebook's FlightDataImporter: columns_map {"t": "time", "alt": "altitude"}, units=None |
| `0c7808a84fb39c83` | 12578 | 2 | `s` | 1 | 0 | 0 | RocketPy 9bd6ad3, its example notebook's FlightDataImporter: columns_map {"time": "time", "altitude": "altitude"}, units=None |
| `a9cf47fe2b2e5be0` | 44454 | 3 | `s` | 2 | 0 | 0 | RocketPy 9bd6ad3, its example notebook's FlightDataImporter: columns_map {"ts": "time", "filtered_altitude_AGL": "altitude", "filtered_acceleration": "az"}, units=None |
| `d88a7fc53cbd55d1` | 17933 | 3 | `s` | 2 | 0 | 0 | RocketPy 9bd6ad3, its example notebook's FlightDataImporter: columns_map {"ts": "time", "filtered_altitude_AGL": "altitude", "filtered_acceleration": "az"}, units=None |
| `dd5287b375e84be8` | 64 | 4 | `s` | 1 | 2 | 0 | RocketPy 9bd6ad3, its example notebook's FlightDataImporter: columns_map {"t": "time", "alt": "altitude"}, units=None |
| `f912724676eea0ca` | 404 | 5 | `s` | 2 | 2 | 0 | RocketPy 9bd6ad3, its example notebook's FlightDataImporter: columns_map {"ts": "time", "altitude[m]": "altitude", "velocity[m/s]": "vz"}, units=None |

The 5 files of binary records (M7.1c16), each read by the record layout stated for it (`validation/log-layouts.json`), the fields' names from the table its team decoded from the same records, beside it: each record's fields, decoded here from the file's bytes (a capture's block as the reader decodes its base64) apart from the reader's records, against that table's row, field for field (both NaN counting as equal), a row a record; and the record read, against the team's table read by its header, cut to the records not erased, channel for channel. *Erased* counts the records at the end that are erased memory, every byte 0xFF, which the reader drops; *Fields written* counts the fields of the others, the samples the record read holds (a float that isn't finite a gap among them); *Misshapen* counts the table's rows of a cell count other than the layout's fields.

| File | Family | What it is | Record, bytes | Records | Erased | Fields written | Table rows | Misshapen | Fields compared | Differing | Same channels |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---|
| `5c15ad3db182065c` | console-capture | a team's flight computer, its shell's dump of flash; the team's table beside it names 25 columns, the first a clock in ms | 100 | 13655 | 12222 | 35825 | 13655 | 0 | 341375 | 0 | yes |
| `67f97f55050a505f` | record-dump | a team's flight computer's raw file, a ground recording; the team's table beside it names 3 columns, the first a clock in ms | 12 | 9000 | 0 | 27000 | 9000 | 0 | 27000 | 0 | yes |
| `8f291cf155079b3d` | record-dump | a team's flight computer's raw file; the team's table beside it names 23 columns, the first a clock in ms | 92 | 9000 | 0 | 207000 | 9000 | 0 | 207000 | 0 | yes |
| `a964e00316c378d9` | console-capture | a team's flight computer, its shell's dump of flash; the team's table beside it names 25 columns, the first a clock in ms | 100 | 13644 | 0 | 341100 | 13644 | 0 | 341100 | 0 | yes |
| `d304c5e63044e3e3` | console-capture | a team's flight computer, its shell's dump of flash; the team's table beside it names 25 columns, the first a clock in ms | 100 | 12838 | 0 | 320950 | 12838 | 0 | 320950 | 0 | yes |

The 2 Blue Raven USB downloads (M7.1c17), each checked here apart from the reader: *Pieces* counts the parts of its lines that open `@ LOG_`, each up to the next or the line's end; *CRC differs* those that close with `CRC:` and four hexadecimal digits whose CRC-16/BUYPASS, worked out here, differs from their text, and *CRC checks* those whose CRC checks; *Length agrees* those of these whose stated length is their own, with the line end the logger sends; *Read* the records the reader keeps, which must be the ones whose CRC checks. Each checked record's values, decoded here by the manual's table, are compared with the reader's channels. *Quaternion norm* is the least and greatest norm of a high-rate record's 7th to 10th values, which a unit quaternion × 30,000 cut to whole numbers keeps within 2 of 30,000; *Pad over AltOS* the low-rate pressure on the pad, the median of the first 20, over the AltOS board's in the same rocket; *Liftoff lag* the time from the high-rate file's first acceleration, the three axes' magnitude, above 3 g to the first low-rate record whose first register has its lowest bit set, the bit the app's files call `Liftoff`. `455ecee4a48bd98e`'s acceleration, the three axes' magnitude, stays above 3 g for at most 0.056 s at a time, and its greatest mean over 1 s is 1.68 g. `a106c197fc5f6dec`'s pressure moves by 0.16 % of its pad's over the whole file, where the AltOS board's falls to 0.773 of its pad's.

| File | Kind | Pieces | CRC differs | CRC checks | Length agrees | Read | Values compared | Differing | Quaternion norm | Pad over AltOS | Liftoff lag, s |
|---|---|---:|---:|---:|---:|---:|---:|---:|---|---:|---:|
| `455ecee4a48bd98e` | high-rate | 17930 | 0 | 17930 | 17930 | 17930 | 304810 | 0 | 29998.25 to 30000.00 | – | – |
| `a106c197fc5f6dec` | low-rate | 1861 | 0 | 1706 | 1706 | 1706 | 49474 | 0 | – | 0.9977 | 0.024 |

The 8 files of a course's Teensy data logger (M7.1c18), each decoded here apart from the reader: *Lines* counts the lines of readings by tag, and each value, in the unit read, is compared with the reader's channels. *MSL off* is the most an `Alt` line's `MSL` differs from 44,330 × (1 − (P / 101,325)^(1/5.255)), the Bosch BMP180's formula worked out in 64-bit floats, m, which its half step, 0.005 m, and an allowance of 44,330 × 2⁻²³ = 0.0053 m for the logger's own rounding of the formula bound: 0.0103 m; *Past half step* the `Alt` lines it differs on by more than 0.005 m; *Pad* and *Landed* the accelerometer's magnitude over 1 g, the median on the pad, the 5 s ending 1 s before liftoff, and over the last 2 s, `A` taken in m/s² and `ICM` in thousandths of a g, each read by the reader as an acceleration, as `P` must be read as a pressure and `MSL` as an altitude; *First 2 s* the same over the file's first 2 s, minutes before launch, a measure, not a check; *Apogee time* the time from liftoff, the first magnitude above 3 g in the 30 s before apogee, to apogee over √(2h/g), h the apogee's height above the pad's, the clock read in microseconds; *Pad pressure* whether the `MS5` pressure on the pad is between 50 and 110 kPa, the value itself not written, as it gives the site's elevation. *High-g over low* is a measure, not a check: the high-g accelerometer's magnitude over the other's during the boost, from 3 g to 90 % of its greatest, `HG` over `A` (both read as m/s²) or `LIS` over `ICM` (both as stated): `HG` is kept as written for it.

| File | Layout | Lines | Values compared | Differing | `Alt` lines | MSL off, m | Past half step | Pad | Landed | First 2 s | Apogee time | Pad pressure | High-g over low |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|---:|
| `2b8cca725a7b5fd6` | tab-separated | A 42155, ADC Pin-35 42155, ADC Pin-36 42155, Alt 42155, G 42155, HG 42155, M 42155 | 674480 | 0 | 42155 | 0.0054 | 1545 | 0.9889 | 0.9988 | 0.9946 | 0.9999 | – | 1.910 |
| `12f752fa1d21c27f` | tab-separated | A 40028, ADC Pin-35 40028, ADC Pin-36 40028, Alt 40028, G 40028, HG 40028, M 40028 | 640448 | 0 | 40028 | 0.0054 | 3044 | 0.9874 | 1.0042 | 1.0476 | 1.0684 | – | 1.433 |
| `d17b644dac464173` | tab-separated | A 49728, ADC Pin-35 49728, ADC Pin-36 49728, Alt 49728, G 49728, HG 49728, M 49728 | 795648 | 0 | 49728 | 0.0054 | 9912 | 0.9869 | 0.9975 | 1.0012 | 0.9079 | – | 1.375 |
| `3f3c6284f174edfc` | tab-separated | A 46510, ADC Pin-35 46510, ADC Pin-36 46510, Alt 46510, G 46510, HG 46510, M 46510 | 744160 | 0 | 46510 | 0.0054 | 6692 | 0.9864 | 1.0073 | 1.0335 | 1.0053 | – | 2.389 |
| `425b1ac3fbe4f794` | tab-separated | A 39934, ADC Pin-35 39934, ADC Pin-36 39934, Alt 39934, G 39934, HG 39934, M 39934 | 638944 | 0 | 39934 | 0.0054 | 144 | 0.9859 | 1.0036 | 1.0207 | 0.9686 | – | 2.003 |
| `bfea33e67007ae0e` | tab-separated | A 43489, ADC Pin-35 43489, ADC Pin-36 43489, Alt 43489, G 43489, HG 43489, M 43489 | 695824 | 0 | 43489 | 0.0054 | 103 | 0.9838 | 1.0007 | 1.0396 | 0.9542 | – | 1.972 |
| `34e2963eda0e95bc` | tab-separated | A 43496, ADC Pin-35 43496, ADC Pin-36 43496, Alt 43496, G 43496, HG 43496, M 43496 | 695936 | 0 | 43496 | 0.0054 | 100 | 0.9840 | 1.0068 | 1.0163 | 1.0989 | – | 1.855 |
| `0bc1429eeabbbda7` | named fields | ADC 5070, ICM 98138, LIS 100615, MS5 4817, VOL 200 | 1303199 | 0 | – | – | – | 0.9889 | 1.0012 | 0.9882 | 1.1351 | inside | 0.996 |

The 4 files of one team's radio telemetry packets, `255:…:254` (M7.1c19), each decoded here apart from the reader: *Packets* counts the whole packets by kind and by path (`T` from the flight computer, `N` relayed by a second transmitter), repeats included; *Repeated* those with the same stage, kind, clock and values as one before them, which the reader drops; *Broken* the lines that are no packet, mark, filter gain or opening number, or a packet with another count of values than its kind's first; *Marks* the `Stage` lines, `Stage` and a number's word from `One` to `Nine`, with a packet after them, each the reader's event, and *Marks repeated* those repeating one before them, which the reader drops. Each value and its time, in the units read (the clock in s from ms, `PP 1` in Pa from hPa), is compared with the reader's channels, as are its counts of repeats and broken lines, its events and its channels' names.

| File | Form | Packets by kind | By path | Repeated | Broken | Marks | Marks repeated | Values compared | Differing |
|---|---|---|---|---:|---:|---:|---:|---:|---:|
| `3a36f081e5be9e9d` | RTF export | AA 1233, CC 1237, PP 2469 | T 4939 | 2 | 0 | 4 | 0 | 20984 | 0 |
| `8720c50cdeaf8ad3` | RTF export | CC 1, PP 2468 | T 2469 | 1 | 1 | 0 | 0 | 9873 | 0 |
| `cd6f44ad29c505e6` | onboard log | AA 66113, CC 66118, PP 132231 | T 264462 | 2 | 0 | 4 | 0 | 1123957 | 0 |
| `a10560cbf44516e5` | received | AA 10578, CC 10634, GG 4321, PP 17263 | N 34235, T 8561 | 8368 | 10495 | 1 | 1 | 160974 | 0 |

Each received file and export against the flight computer's own log beside it: *Held* counts, by path, its whole packets, repeats included, for which the log holds a packet of the same stage, kind and clock, and *Differing* those whose values differ from it as numbers; the rest the log doesn't hold, the GPS fixes (`GG`), which only the second transmitter sends, and others.

| File | Form | Onboard log | Held | Differing | Not held, GPS | Not held, other |
|---|---|---|---|---:|---:|---:|
| `3a36f081e5be9e9d` | RTF export | `cd6f44ad29c505e6` | T 4939 | 0 | 0 | 0 |
| `8720c50cdeaf8ad3` | RTF export | `cd6f44ad29c505e6` | T 2469 | 0 | 0 | 0 |
| `a10560cbf44516e5` | received | `cd6f44ad29c505e6` | N 29914, T 8561 | 0 | 4321 | 0 |

The onboard log `cd6f44ad29c505e6` held to its flight, in the units read: the pad's pressure, the median of `PP 1` from 6 s to 1 s before liftoff, inside 500–1,100 hPa (the value not written, as it gives the site's elevation); liftoff the first packet after the last of stage `00` whose pressure is more than 1 hPa below the median of the 6 s to 1 s before it, where before liftoff no packet falls more than 0.61 hPa below its median, 0.894 s before the flight computer's next stage; apogee the least pressure. The time from liftoff to apogee over √(2h/g), h the standard atmosphere's height of the apogee's pressure above the pad's (44,330.77 × (1 − (P / 101,325)^0.190263) m, U.S. Standard Atmosphere, 1976), the clock read in ms: 1.0548, inside 0.5–2. Over the 2113 packets whose `PP 3` is above 200, `PP 3` is 1.116 to 1.231 times the standard atmosphere's height of `PP 1` above the pad's in meters, a measure, not a check: no unit is settled for it, so the reader keeps it as written.

The 2 Featherweight Raven FIP exports pasted into a workbook below no header (M7.1c20), each read under the header stated for it (`validation/log-headers.json`) and compared with the `.FIPa` of the same flight beside it. *Channels* counts those read, status columns apart; *Left out* the channels a pasted table covers, with the rows. Every channel the `.FIPa` holds must match the sheet's sample by sample, each at the `.FIPa`'s time as the FIP prints it and within half a step of its resolution: *Samples* counts them, *After the stop* the `.FIPa`'s velocity samples after the export stops that channel, and *Time off*, s, and *Value off*, in steps, are the worst. Four of the five derived channels the `.FIPa` doesn't hold, where a sheet keeps them, back the stated header's names for them (`[Altitude (Accel-Ft)]` is held to nothing): *ASL* is whether `[Altitude (Baro-Ft-ASL)]` is the standard atmosphere's height of the sheet's own `Baro (Atm)`, 44,330.77 × (1 − (P / 101,325)^0.190263) m with P in pascals, less one offset, within the pressure's printed half step, 0.000005 atm, times the formula's slope, and the altitude's in its sixth significant figure, with half the spread, m; *AGL* whether `[Altitude (Baro-Ft-AGL)]` is that less one constant, the pad's, within both printed half steps, and *Pad*, m, its first sample in magnitude, within 3 m of 0, as the FIP sets it to 0 on the pad, where the pad's height in the standard atmosphere would stand were the two altitudes' names swapped; *mph* whether `[Velocity (Accel-MPH)]` is `[Velocity (Accel-Ft/Sec)]`, both in m/s, within both printed half steps.

| File | Channels | Left out | `.FIPa` | Matching | Samples | After the stop | Time off, s | Value off, steps | ASL | Spread, m | AGL | Spread, m | Pad, m | mph |
|---|---:|---|---|---|---:|---:|---:|---:|---|---:|---|---:|---:|---|
| `6f5e20612831ac57` | 16 | none | `66aadbe6b4daa392` | every sample | 40841 | 850 | 5.00e-5 | 0.000 | held | 0.0040 | held | 0.0015 | 0.20 | held |
| `b2a86ded765053b4` | 13 | "[Altitude (Accel-Ft)]" left out: the pasted columns cover its cells in 1259 rows; "[Velocity (Accel-Ft/Sec)]" left out: the pasted columns cover its cells in 11331 rows; "[Velocity (Accel-MPH)]" left out: the pasted columns cover its cells in 23921 rows | `856e61d186324d0a` | every sample | 48290 | 811 | 5.00e-5 | 0.000 | held | 0.0196 | held | 0.0015 | 0.72 | – |

The AltiMAX export `444687655195f5c8` (M7.1c20), its `.xls` stream decoded here apart from the reader. Its records, by id: `0x0002` 50668, `0x0003` 202672, `0x0004` 25345, `0x000a` 1, `0x0024` 11, `0x0200` 1, `0x041e` 36, `0x0809` 1; the first, a BOF, of 0x6 bytes, version 0x0000, type 0x0010. Its 278685 cells of BIFF2's cell records (`INTEGER` `0x0002`, `NUMBER` `0x0003`, `LABEL` `0x0004`) against the reader's sheet text: 0 differ. Its 253340 values, the text read here by the stated units but the pressures in pascals, against the reader's: 0 differ; the pressures read as pressures, the heights as heights above the pad, `ACCEL` as an acceleration and `SPEED` as a speed: yes. Each height of both columns, filtered and raw, 50668 pairs, against the standard atmosphere's height of its pressure in pascals, 44,330.77 × (1 − (P / 101,325)^0.190263) m (U.S. Standard Atmosphere, 1976), less one offset for the file, the offset not written, as it gives the site's elevation: half the spread is 0.0044 m, inside the bound of 44,330.77 × 2⁻²³ = 0.0053 m for the software's own working of the formula; read in hectopascals, as the columns state, it would be more than a kilometer. The first raw pressure, on the pad, is inside 50–110 kPa. `ACCEL` before liftoff, its first magnitude above 3 g, reads 0.940 g in magnitude, its sign negative: the axis it measures along points down the rocket, so it reads −1 g at rest, not +1 g; the band is 0.85–1.15. `SPEED`'s change over 0.08 s against `ACCEL`, the median over the boost: 0.9988, as `SPEED` sums `ACCEL` over the clock, gravity left in, so it isn't the rocket's speed; the band is 0.9–1.1. The time from liftoff to apogee, the highest raw height, over √(2h/g), h the apogee's height above the first sample's: 0.945, inside 0.5–2. The reader states 21 events and splits `VOLTAGE [V]` into 5 channels.

The 2 MARSA downloads (M7.1c21), each decoded here apart from the reader: *Settings* counts its `<NAME> value` lines, *Samples* its rows, and each sample's time in s, altitude in ft and acceleration in g is compared with the reader's record, six comparisons a sample: the time and altitude on the primary track and the times and values of the altitude and acceleration channels; *Figures* is whether the record's figures are the settings, by name and value, and *Quantities* whether it read the altitude as a height above its zero and the acceleration as an acceleration. *Beside* is the other logger in the same flight, the AltOS board in the flight's folder, whose other log is the other stage's. *Apogee ratio*, the check of the feet, is the highest altitude over that board's highest height above its pad, both in m, which must be within 0.95–1.05; *In meters* the same were the altitude read in meters. *Pad, ft* is the altitude before liftoff, the first acceleration above 3 g, the median: the MARSA's zero isn't its reading on the pad. *Pad, g* is the acceleration's median there, which must be within 0.5 g of 0, nearer 0 than 1 g, as gravity is taken out. *Time off, s* is the time from liftoff to the highest altitude less the board's, a measure. *Climb*, the check of the g, is the barometric climb over the first 2 s after liftoff over the acceleration integrated twice from rest over them, which must be within 0.6–1.667, the band AltOS's `.eeprom` reader holds its own climb to; *In m/s²* the same were the acceleration read in m/s². *Count step, ft* is the least and greatest step between the altitudes written within 300 ft above the pad's reading that are one count, under 1.5 times the least: a measure, from which the format's step, 29.3 ft, is taken, so it holds near the pad only, a count spanning more feet where the air is thinner.

| File | Settings | Samples | Values compared | Differing | Figures | Quantities | Beside | Apogee ratio | In meters | Pad, ft | Pad, g | Time off, s | Climb | In m/s² | Count step, ft |
|---|---:|---:|---:|---:|---|---|---|---:|---:|---:|---:|---:|---:|---:|---|
| `4be82002a95c68db` | 53 | 398 | 2388 | 0 | yes | yes | `2a665d94d9ad35d1` | 0.981 | 3.22 | -87.0 | -0.12 | -2.6 | 0.915 | 8.972 | 29.0–29.3 |
| `a80063e417b87028` | 53 | 554 | 3324 | 0 | yes | yes | `2f3fbd62dbc4f064` | 0.986 | 3.23 | -87.1 | -0.12 | -1.2 | 1.035 | 10.149 | 29.1–29.3 |

AltOS's radio telemetry decoded to a table (M7.1c21), each read by its header's names, the fields of the TeleMega's `0x09` packet in *AltOS Telemetry* (§2.3, Table 6), and decoded here apart from the reader: *Rows* counts them, *CRC false* those left out as packets that failed their check, and *Packet types* the types they name; each value of the rest and its time is compared with the reader's channel, and each time with its primary track, *Values compared* counting the comparisons; *Unknown* lists the columns it kept as names it doesn't know. The resolution the packet carries, which no wrong unit need fail: *Sixteenths* is whether every acceleration and speed is a whole number of sixteenths, *Meters* every height a whole meter, *Hundredths* every tick a whole hundredth of a second. *Battery, V* is `v_batt`'s least and greatest, which must be within 2.5–5.0 V, a single LiPo cell's, 3.7 V nominal (*The Altus Metrum System*, §3.2): read as the document's 12-bit counts behind its 100k/27k divider, 4 V would be about 1,055. *Events* are the reader's, each state named as AltOS's exports name its number, none `state N`, and the first `ground_pres` must be inside 50–110 kPa, the value not written. *Pad accel* is the acceleration's median in the pad state, m/s², about 0 with gravity taken out, a measure. The checks of the units: *Climb over speed* is the median from boost to coast of the height's change between two rows over their time, over their mean speed, where it is over 10 m/s; *Speed over accel* the same of the speed's change over the mean acceleration, where it is over 5 m/s². Each must be within 0.6–1.667, the band of the `.eeprom` reader's climb check: a speed or an acceleration in the packet's sixteenths, a height in feet or a tick in its 10 ms counts would miss it.

| File | Rows | CRC false | Packet types | Values compared | Differing | Unknown | Sixteenths | Meters | Hundredths | Battery, V | Events | Pad accel | Climb over speed | Speed over accel |
|---|---:|---:|---|---:|---:|---|---|---|---|---|---|---:|---:|---:|
| `850771db6621eaf6` | 325 | 0 | 9 ×325 | 12025 | 0 | none | yes | yes | yes | 4.09–4.10 | pad, boost, fast, coast, drogue | -0.438 | 1.019 | 0.929 |

Tables cut from an AltOS export (M7.1c21), AltOS's own column names with no `version,serial,flight` before them, each read by its states and decoded here apart from the reader: *States* counts each row's `state` and `state_name`, and *AltOS's* is whether every pair is one AltOS's exports write, 2 `pad` to 8 `landed`; each value and its time is compared with the reader's channel, and the primary track's times and heights with the first row at each time, as AltOS repeats one, *Values compared* counting the comparisons; the channels must be the table's columns but the clock and `state_name`, in order, and *Events* the reader's, each state named as AltOS's exports name it. *Height offset, m* is half the spread of `altitude` less `height`, which AltOS writes as its barometric altitude less the pad's, one offset, not written as it gives the site's elevation: each to the centimeter, it must be within 0.01 m. *Rise ratio*, the check of the units, is `altitude`'s rise from the first row to its highest over the standard atmosphere's height's rise of `pressure` over the same rows, 44,330.77 × (1 − (P / 101,325)^0.190263) m, which must be within 0.6–1.667, the band of the `.eeprom` reader's climb check: an altitude in feet, or a pressure in hPa, would miss it. *From pressure, m* is the most `altitude` differs from that height: a measure of AltOS's own conversion.

| File | Rows | States | AltOS's | Values compared | Differing | Same columns | Events | Height offset, m | Rise ratio | From pressure, m |
|---|---:|---|---|---:|---:|---|---|---:|---:|---:|
| `03adc04683e2884f` | 1005 | 3 boost ×347, 5 coast ×155, 7 main ×502, 8 landed ×1 | yes | 15948 | 0 | yes | boost, coast, main, landed | 0.0050 | 1.0000 | 0.0056 |

AltOS exports a spreadsheet re-saved (M7.1c21), decoded here apart from the reader: *Formulas* counts the cells of each column that hold a formula, `=…`; each is rebuilt from the reader's notes, which give a column's formulas run by run, `{row}` standing for each cell's own row, and compared with the file's cell (*Compared*, *Differing*); *Not channels* is whether every column of formulas alone is left out of the channels, as text. *Units* lists the columns whose name carries a unit after it and the quantity the reader read each as: AltOS's where the unit is the one it reads the column in, kept as written (`Unrecognized`) where it isn't. Each value of the channels, a formula or a cell that isn't a number a gap, and its time is compared with the reader's channel, *Values compared* counting the comparisons; *Not numbers* counts the cells, formulas apart, read as gaps. *Primary track* is none, as the header names no `height`.

| File | Rows | Formulas | Compared | Differing | Not channels | Units | Values compared | Differing | Not numbers | Primary track |
|---|---:|---|---:|---:|---|---|---:|---:|---:|---|
| `9c5017ab68168088` | 26916 | `(ft/s)` 26916, `Altitude (ft)` 26916, `altitude (ft)` 26916 | 80748 | 0 | yes | `acceleration (m/s^2)` Acceleration, `altitude (m)` AltitudeMsl, `speed (m/s)` Speed, `accel_x (m/s)` Unrecognized, `accel_y (m/s)` Unrecognized, `accel_z (m/s)` Unrecognized, `tilt (degrees)` Angle, `altitude (m)` GnssAltitude | 4198896 | 0 | 1 | none |

The 20 tables read by a column mapping held to physics (M7.1c22, ADR-274): tables whose headers state too little or that have none, each read by the mapping stated for it in `validation/log-held-mappings.json`, which names a private table's columns by its own labels or by the header at each run, but for the last name of a header that runs onto its first row. *Read* counts the columns read in the unit the mapping states, *Kept* those kept as written, with no unit, and *Left out* those the mapping leaves out. Each check holds a unit, its columns counted from 0: *apogee time*, the clock's, is the time from liftoff to apogee of a height over √(2h/g), within 0.5–2.0; *clock beside*, the clock's, is the overlap of this file's span of times and another's, of the same logger, over the longer of the two, at least 0.5; *clock rate*, for a recording with no flight, is its median step over the same logger's in a file held by its flight, within 0.9–1.1; *rows of* is the share of its rows that are rows of another file, value for value, which must be 1; *pad pressure*, a pressure's, is its reading at the pad within 50–110 kPa, its value not shown, as it would give the site's elevation; *standard atmosphere*, an altitude's, is its rise over the standard-atmosphere height's rise of the file's pressure; *other logger* its rise over another logger's in the same flight; *stated apogee* its rise over the apogee the flyer states; *speed over height*, a vertical speed's, is an altitude's change over it from liftoff to apogee; *track speed*, a track's GPS altitude's, is how fast the track's own vertical speed falls over the 5 s before its highest fix, in g, from a parabola fitted to those fixes, as a rocket coasting to apogee slows at g and a little more from drag: each within 0.6–5/3, which no unit off by 3.28 or more reaches; *1 g at rest* is the accelerometer's median magnitude over the recording, in g, within 0.85–1.15; and *place*, a track's, is whether the median size of each column's readings other than 0 lies past ±π/2 for a latitude and ±π for a longitude, which no place has in radians, its values not shown. A column with no unit is kept as written: a temperature, as no check here tells °C from °F at a launch's temperatures, a rate of turn, which none holds, and every other column no check holds.

| File | What | Rows | Read | Kept | Left out | Events | Checks |
|---|---|---:|---:|---:|---:|---:|---|
| `055206ce7e9a2fae` | a university club's GPS fixes, `timestamp,latitude,longitude,altitude`: Unix seconds, degrees and meters, as the club's own notes on the flight state, the same rows with no header | 434 | 3 | 0 | 0 | 0 | apogee time (3) 0.9315 ✓; place (1, 2) ✓; other logger (3) 0.9552 ✓; track speed (3) 0.9925 ✓ |
| `0b81bee53c3096cd` | a university team's flight computer's file of one sensor's readings, no header: a 32-bit count of microseconds, which it prints 9 × 2³² high in the upper half of each turn, then the values: a GPS receiver's fixes, of which only the longitude is a number the survey can read | 17222 | 1 | 3 | 0 | 0 | clock beside 0.9999 ✓; place (3) ✓ |
| `238756c53758ae7c` | a university club's GPS fixes, `timestamp,latitude,longitude,altitude`: Unix seconds, degrees and meters, as the club's own notes on the flight state, under that header | 434 | 3 | 0 | 0 | 0 | apogee time (3) 0.9315 ✓; place (1, 2) ✓; other logger (3) 0.9552 ✓; track speed (3) 0.9925 ✓ |
| `268517db228becd0` | a flyer's workbook of an altimeter's download pasted into its first sheet, `Time,Altitude,Temp,Voltage`: seconds and feet above the pad, a temperature written with its unit, and the battery; the cells beside the table, about the flight, left out | 1184 | 1 | 2 | 7 | 0 | apogee time (1) 0.9784 ✓; stated apogee (1) 1.0025 ✓ |
| `419f0ec872abda2d` | the same team's radio telemetry as its ground station saved it, each packet a list, `[…, True, True]`, on the board's microsecond count: an altitude, two pressures, two accelerometer axes, a GPS fix and its time | 3870 | 6 | 5 | 0 | 0 | apogee time (2) 0.9468 ✓; pad pressure (2) ✓; pad pressure (3) ✓; standard atmosphere (1, 2) 0.9992 ✓; place (7, 6) ✓; other logger (8) 0.9008 ✓ |
| `47f7e2fe2d0414a3` | a university team's flight computer's file of one sensor's readings, no header: a 32-bit count of microseconds, which it prints 9 × 2³² high in the upper half of each turn, then the values: two barometers' pressures | 953298 | 2 | 0 | 0 | 0 | apogee time (1) 0.8627 ✓; pad pressure (1) ✓; pad pressure (2) ✓ |
| `4bdad9db3e40a5b3` | a hobbyist's small flight computer's table, its header naming 17 columns with no unit: seconds, a state, a temperature, humidity, a pressure in hectopascals, a gas sensor, an altitude above the pad in feet, a gyroscope and an accelerometer in g with their magnitudes | 125 | 6 | 10 | 0 | 5 | apogee time (5) 1.7105 ✓; pad pressure (5) ✓; standard atmosphere (7, 5) 1.0005 ✓; 1 g at rest (13, 14, 15) 0.9983 ✓; 1 g at rest (16) 1.0000 ✓ |
| `4ea3b7a8a4d4119c` | a student team's avionics table, no header: milliseconds, a pressure, a temperature, three accelerometer axes and three gyroscope axes, each row ending in a comma; a recording on the pad that ends before liftoff | 2480 | 4 | 5 | 0 | 0 | clock rate 1.0000 ✓; pad pressure (1) ✓; 1 g at rest (3, 4, 5) 0.9814 ✓ |
| `5843d20becfb8cda` | a flight computer's GNSS fixes, `ts,latitude,longitude,satellites`, on the clock of the flight table beside it, which its publisher maps in seconds | 246 | 2 | 1 | 0 | 0 | clock beside 0.9993 ✓; place (1, 2) ✓ |
| `5ab193c0ed84f6b4` | a university team's payload's text, each line a date and time, `:` and a reading: a pressure in hectopascals | 125920 | 1 | 0 | 0 | 0 | apogee time (1) 0.8640 ✓; pad pressure (1) ✓ |
| `5cf819cc85147315` | a university team's flight computer's file of one sensor's readings, no header: a 32-bit count of microseconds, which it prints 9 × 2³² high in the upper half of each turn, then the values: a GPS receiver's fix flag, longitude and latitude | 19034 | 2 | 3 | 0 | 0 | clock beside 0.9999 ✓; place (4, 3) ✓ |
| `5f2b0f963a9e52e9` | a university team's flight computer's file of one sensor's readings, no header: a 32-bit count of microseconds, which it prints 9 × 2³² high in the upper half of each turn, then the values: two barometers' pressures | 862537 | 2 | 0 | 0 | 0 | apogee time (1) 0.9912 ✓; pad pressure (1) ✓; pad pressure (2) ✓ |
| `6416a31040c017d7` | a university team's flight computer's file of one sensor's readings, no header: a 32-bit count of microseconds, which it prints 9 × 2³² high in the upper half of each turn, then the values: a two-axis accelerometer's axes, 0 at rest (gravity taken out), kept as written | 862536 | 0 | 2 | 0 | 0 | clock beside 1.0000 ✓ |
| `73304debd898b712` | a university team's flight computer's table, its header naming 25 columns with their units but its clock, `log_us`, with no time word: two accelerometers in m/s², their gyroscopes, a barometer's pressure and each sensor's state | 14837 | 7 | 17 | 0 | 67 | apogee time (23) 1.0208 ✓; pad pressure (23) ✓; 1 g at rest (5, 6, 7) 1.0356 ✓; 1 g at rest (14, 15, 16) 1.0531 ✓ |
| `88f33383ccf621a0` | a university club's ground station's conversion of a flight computer's downlink to that board's log: a `CONFIG` line, then 28 comma-separated columns with no header, in the order of the board's log with its settings (`hpr_flightdata::sillygoose`) | 11206 | 8 | 19 | 0 | 0 | apogee time (1) 1.1103 ✓; pad pressure (1) ✓; standard atmosphere (11, 1) 0.9559 ✓; standard atmosphere (14, 1) 0.9559 ✓; 1 g at rest (3, 4, 5) 0.9854 ✓; 1 g at rest (13) 0.9854 ✓; speed over height (12, 11) 0.9709 ✓ |
| `9b45103dd53075fa` | a phone-sized flight computer's table, its header naming 34 columns with no unit and the first row run onto its last name: seconds, a pressure in hectopascals, temperatures, a pressure altitude in meters and in feet, an accelerometer in m/s², and its gyroscope, magnetometer, filters' outputs and rotations | 9388 | 6 | 28 | 0 | 0 | apogee time (1) 1.3211 ✓; pad pressure (1) ✓; standard atmosphere (5, 1) 1.0002 ✓; standard atmosphere (6, 1) 1.0003 ✓; 1 g at rest (7, 8, 9) 0.9822 ✓ |
| `a601fc98238e2546` | a university team's payload's text, each line a date and time, `:` and a reading: an accelerometer's three axes in m/s² as a tuple, `(x, y, z)`, and a line marking motion detected | 126269 | 3 | 0 | 0 | 1 | clock beside 0.9984 ✓; 1 g at rest (1, 2, 3) 1.0278 ✓ |
| `dc85b22c94b96291` | a university team's flight computer's file of one sensor's readings, no header: a 32-bit count of microseconds, which it prints 9 × 2³² high in the upper half of each turn, then the values: a two-axis accelerometer's axes, 0 at rest (gravity taken out), kept as written | 953297 | 0 | 2 | 0 | 0 | clock beside 1.0000 ✓ |
| `f1788c6722a77dd6` | a university club's GPS fixes, `timestamp,latitude,longitude,altitude`: Unix seconds, degrees and meters, as the club's own notes on the flight state, 37 rows cut from the file with no header | 37 | 3 | 0 | 0 | 0 | rows of 1.0000 ✓ |
| `f77449557bb81617` | a student team's avionics table, no header: milliseconds, a pressure, a temperature, three accelerometer axes and three gyroscope axes, each row ending in a comma | 12000 | 4 | 5 | 0 | 0 | apogee time (1) 1.0367 ✓; pad pressure (1) ✓; 1 g at rest (3, 4, 5) 0.9600 ✓ |

A track's own speed, which ADR-274's plan named among the checks, holds the track's GPS altitude by its vertical speed: the *track speed* check above. It takes the highest fix as apogee and needs 4 fixes or more in the 5 s before it, the highest included, that set the parabola's curvature about as well as 5 fixes 1 s apart; it holds a unit within a wide band, and doesn't measure drag. It passes in 2 files, slowing at 0.9925 g. The survey fails a GPS altitude of a track a place check holds that could take the check and takes none. The other track with a GPS altitude has 1 fix there, too few or too close together to take it, and its altitude is held by its other checks. It doesn't hold the clock, which the clock checks hold, as every held file's: read 10 times too short, the window would take 50 s of fixes.

A track's ground speed holds no unit. Over the 4 tracks held by a place check of a latitude and a longitude (5 files, a file whose fixes are another's counted once), the median ground speed from each fix to the next the receiver reports, the great-circle distance over the step of the track's own clock, taken over the whole recording, the pad's hours included, so that a median can be a receiver's jitter as much as the wind's drift, lies 0.3460–7.3836 m/s, a spread of 21.34 times, wider than the 10 times a clock misread by a factor of 10 moves it, so no band on these tracks refuses that misread on every one of them. Nor could a band hold a clock on every flight: a track drifts with the wind, and a launch may go on a calm day, so its speed has no floor (NFPA 1127 limits only the surface wind, to 20 mph, 8.9 m/s). Each track's fastest step lies 24.8509–400.0430 m/s, a spread of 16.10 times, so a track's top speed holds no clock either. Radians stated as degrees would read 57 times slower, which a band could part on these tracks alone, but a band fitted to them is a sample, not a bound; the place check holds a track's degrees with no band.

The 16 Excel workbooks (M7.1c14): each sheet's cells written out as text, as the workbook stores them, and read as a file of its own; a sheet's outcome is the layout it reads as, or why it doesn't (*no-data* for a sheet with no value, such as one holding only a chart). In all, 28 sheets: 6 Blue Raven CSV, 1 Eggtimer CSV, 2 Featherweight GPS CSV, 2 Featherweight summary CSV, 10 a table read by its header, 3 no-data, 1 syntax, 3 unrecognized.

| File | Sheets | Outcomes |
|---|---:|---|
| `02cf95e874f824f7` | 1 | 1 a table read by its header |
| `084653fe77f2ca50` | 1 | 1 a table read by its header |
| `229efb1695f3277f` | 3 | 1 Eggtimer CSV, 1 no-data, 1 syntax |
| `2d2455b75cfe7672` | 1 | 1 a table read by its header |
| `3e56a501d1b71a8a` | 3 | 1 a table read by its header, 1 no-data, 1 unrecognized |
| `44a479aaaf2812ce` | 3 | 1 a table read by its header, 2 unrecognized |
| `8855863e76ea3fbc` | 1 | 1 a table read by its header |
| `9ee24565e8232902` | 1 | 1 Featherweight GPS CSV |
| `acc19f3b1096cb74` | 1 | 1 Featherweight GPS CSV |
| `b27cc498f8d15caa` | 3 | 2 Blue Raven CSV, 1 Featherweight summary CSV |
| `c34428a0bff32b60` | 1 | 1 a table read by its header |
| `c3e5272b33d0ecdb` | 1 | 1 a table read by its header |
| `d21437764a14c64b` | 1 | 1 a table read by its header |
| `e3ff82bb5d72ff52` | 3 | 2 Blue Raven CSV, 1 Featherweight summary CSV |
| `ef7b7dbbf32a8e67` | 3 | 2 Blue Raven CSV, 1 no-data |
| `f7b6e0a3273d6b79` | 1 | 1 a table read by its header |

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
| altos-eeprom | `accel` kept as written | 1 |
| altos-telem | `accel`, `v_batt`, `sense_d` and `sense_m` kept as written | 2 |
| altos-telem | `ground_accel`, `accel_plus_g` and `accel_minus_g` kept as written | 2 |
| altos-telem | `pres` read in pascals, not the document's tenths of a pascal | 2 |
| arts2-csv | `Acc` read in meters per second squared, from `Vel` | 1 |
| arts2-csv | `Acc` read in standard gravities, 9.806 65 m/s², from `Vel` | 3 |
| arts2-csv | `MET` read in seconds, from the apogee | 4 |
| arts2-csv | `Vel` read in feet per second, from the apogee and the barometer's step | 3 |
| arts2-csv | `Vel` read in meters per second, from the apogee and the barometer's step | 1 |
| arts2-csv | `iAlt` read in feet, from `Vel` | 3 |
| arts2-csv | `iAlt` read in meters, from `Vel` | 1 |
| arts2-csv | `pAlt` read in feet, from its step and `Vel` | 3 |
| arts2-csv | `pAlt` read in meters, from its step and `Vel` | 1 |
| arts2-raw-csv | `Acc(HI)` and `Baro(HI)` kept as written | 3 |
| arts2-raw-csv | `Acc/Baro(LO)` and `State` kept as written | 3 |
| blueraven-csv | `Accel_X`, `Accel_Y` and `Accel_Z` read in standard gravities, from the manual and the pad | 40 |
| blueraven-csv | `Current` kept as written, in a unit no source states | 40 |
| blueraven-csv | `Gyro_X`, `Gyro_Y` and `Gyro_Z` read in degrees per second, from the manual and the summary | 40 |
| blueraven-csv | `Inertial_Altitude`, `Inertial_DR_Position` and `Inertial_CR_position` read in feet, from the manual and the summary | 49 |
| blueraven-csv | `Velocity_Up`, `Velocity_DR` and `Velocity_CR` read in feet per second, from the manual and the summary | 49 |
| blueraven-summary-csv | `Hi-G accel pad offset` kept as written, in a unit no source states | 26 |
| blueraven-summary-csv | `Low-G accel pad offset` kept as written, in a unit no source states | 26 |
| blueraven-summary-csv | `Main chute descent rate` kept as written, in a unit that isn't a speed | 26 |
| blueraven-summary-csv | `Pad Hi-G accel` kept as written, in a unit no source states | 26 |
| blueraven-summary-csv | `Pad Low-G Accel` kept as written, in a unit no source states | 26 |
| e80-imu-txt | `Acc`, `Prs`, `ADC0`, `ADC1`, `ADC2`, `ADC3`, `ADC4`, `ADC5` kept as written | 54 |
| e80-imu-txt | `Dig` read in hexadecimal and kept as its number | 54 |
| e80-imu-txt | `Time` read in seconds | 54 |
| eggtimer-csv | `AltAccel` read in feet, from the manual | 4 |
| eggtimer-csv | `Alt` read in feet, from the manual | 44 |
| eggtimer-csv | `FAccelAlt` read in feet, from the manual | 4 |
| eggtimer-csv | `FAccelVel` read in feet per second, from the manual | 4 |
| eggtimer-csv | `FAlt` read in feet, from the manual | 38 |
| eggtimer-csv | `FVeloc` read in feet per second, from the manual | 44 |
| eggtimer-csv | `Filt_Alt` read in feet, from the manual | 6 |
| eggtimer-csv | `T` read in milliseconds, from the step | 4 |
| eggtimer-csv | `T` read in seconds, from the step | 40 |
| eggtimer-csv | `VAccel` read in feet per second, from the manual | 4 |
| eggtimer-csv | `Veloc` read in feet per second, from the manual | 41 |
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
| featherweight-summary-csv | `Hi-G accel pad offset` kept as written, in a unit no source states | 13 |
| featherweight-summary-csv | `Low-G accel pad offset` kept as written, in a unit no source states | 13 |
| featherweight-summary-csv | `Main chute descent rate` kept as written, in a unit that isn't a speed | 13 |
| featherweight-summary-csv | `Pad Hi-G Accel` kept as written, in a unit no source states | 1 |
| featherweight-summary-csv | `Pad Hi-G accel` kept as written, in a unit no source states | 12 |
| featherweight-summary-csv | `Pad Low-G Accel` kept as written, in a unit no source states | 12 |
| featherweight-summary-csv | `Pad low-G Accel` kept as written, in a unit no source states | 1 |
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
| kml | `coordinates` read in meters, from the KML standard | 42 |
| kml | `time` read in seconds, from the steps between points | 3 |
| kml | `when` read in UTC, from the KML standard | 2 |
| kml-map | `coordinates` kept as written, in a track whose altitude mode tells a viewer to ignore its altitude | 1 |
| missileworks-rtx | `Altitude` read in meters, from the manual and the exports | 5 |
| missileworks-rtx | `COG` read in degrees, from the positions | 5 |
| missileworks-rtx | `Velocity` and `SOG` kept as written, in units no document states | 5 |
| missileworks-rtx | `word 15` kept as written, a word the exports leave out | 5 |
| nmea | `GSA` and `GSV` read in the time of the `GGA` or `RMC` before them, from the sentence order | 12 |
| nmea | `RMC date` read in UTC days, from status-`A` sentences only | 12 |
| odf | `Acc(HI)` read in m/s², along the board's axis with the pad's reading taken out | 3 |
| odf | `Acc/Baro(LO)` and `State` kept as written | 3 |
| odf | `Baro(HI)` read in pascals, by the MPX4100A's nominal transfer function | 3 |
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
| rrc3-csv | `Altitude` read in feet, from the pressure | 47 |
| rrc3-csv | `Pressure` read in hectopascals, from the values | 47 |
| rrc3-csv | `Temperature` read in degrees Fahrenheit, assumed | 41 |
| rrc3-csv | `Time` read in seconds, from the step | 47 |
| rrc3-csv | `Velocity` kept as written, its unit unknown | 2 |
| rrc3-csv | `Velocity` read in feet per second, from the altitude | 36 |
| rrc3-csv | `Velocity` read in miles per hour, from the altitude | 9 |
| rrc3-csv | `Voltages` read in volts, assumed | 43 |
| rtx-csv | `Altitude` read in feet, from the `.rtx` files beside the sample exports | 5 |
| rtx-csv | `COG` read in degrees, from the positions | 5 |
| rtx-csv | `Velocity` and `SOG` kept as written, in units no document states | 5 |
| rtx-txt | `Alt` read in feet, from the `.rtx` files beside the sample exports | 4 |
| rtx-txt | `COG` read in degrees, from the positions | 4 |
| rtx-txt | `Vel` and `SOG` kept as written, in units no document states | 4 |
| teensy-tabs-txt | `A` read in m/s² and `Alt`'s `P` in Pa and `MSL` in m, as written | 7 |

The files read, by the logger each names:

| Logger | Files |
|---|---:|
| AED Electronics R-DAS | 54 |
| AerospaceNU SillyGoose | 13 |
| AerospaceNU flight computer board | 20 |
| AerospaceNU flight computer board, by its ground station | 4 |
| AltOS | 55 |
| AltOS EasyMega-v2.0 | 1 |
| AltOS EasyMini-v1.0 | 1 |
| AltOS EasyMini-v2.0 | 1 |
| AltOS TeleGPS-v1.0 | 3 |
| AltOS TeleGPS-v2.0 | 4 |
| AltOS TeleMega-v4.0 | 1 |
| AltOS TeleMega-v6.0 | 1 |
| AltOS TeleMetrum | 1 |
| AltOS TeleMetrum v2 or newer | 2 |
| AltOS TeleMetrum-v1.0 | 2 |
| AltOS TeleMetrum-v1.2 | 1 |
| AltOS TeleMetrum-v2.0 | 6 |
| AltOS TeleMetrum-v4.0 | 1 |
| Big Red Bee BeeLine | 2 |
| Eggtimer | 45 |
| Eggtimer Build_1.52 | 1 |
| Eggtimer TRS | 2 |
| Entacore AIM | 16 |
| Featherweight Blue Raven | 138 |
| Featherweight GPS ground station | 14 |
| Featherweight GPS tracker | 40 |
| Featherweight Raven | 139 |
| GPS Visualizer | 1 |
| KML | 3 |
| MARSA | 2 |
| Missile Works RRC3 | 57 |
| Missile Works RTx | 26 |
| NMEA 0183 receiver (talker GP) | 12 |
| Ozark Aerospace ARTS2 | 10 |
| PerfectFlite | 21 |
| PerfectFlite Alt15K/WD | 25 |
| PerfectFlite Pnut | 39 |
| PerfectFlite SL100 | 35 |
| PerfectFlite SLCF | 101 |
| Rocketronics AltiMAX | 1 |
| Rocketry Ltd Mercury | 4 |
| Rocketry Ltd Nano V1 | 7 |
| Silicdyne Fluctus | 6 |
| SillyGooseV2 | 1 |
| Teensy data logger | 8 |
| fw2kml | 15 |
| phone app | 27 |
| radio telemetry packets | 4 |
| unknown | 72 |
