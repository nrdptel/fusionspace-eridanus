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
| aerospacenu-sillygoose-gs-csv | M7.1c19 | 1 | 0 | 1 unrecognized |
| aerospacenu-sillygoose-txt | M7.1c3 | 14 | 14 | none |
| altimetercloud-csv | M7.1c2 | 11 | 11 | none |
| altos-csv | M7.1a | 48 | 48 | none |
| altos-csv-damaged | damaged | 1 | 0 | 1 syntax |
| altos-eeprom | M7.1c10 | 14 | 14 | none |
| altos-telem | M7.1c9 | 2 | 2 | none |
| arts2-csv | M7.1c3 | 4 | 4 | none |
| arts2-raw-csv | M7.1c11 | 3 | 3 | none |
| binary | M7.1c19 | 2 | 0 | 2 unrecognized |
| blueraven-csv | M7.1b | 89 | 89 | none |
| blueraven-events-txt | not a log | 17 | 0 | 17 unrecognized |
| blueraven-settings-csv | not a log | 1 | 0 | 1 unrecognized |
| blueraven-settings-xlsx | not a log | 10 | 0 | 10 archive |
| blueraven-summary-copy-txt | not a log | 1 | 0 | 1 unrecognized |
| blueraven-summary-csv | M7.1c6 | 26 | 26 | none |
| blueraven-usb-txt | M7.1c17 | 2 | 2 | none |
| checklist | not a log | 4 | 0 | 4 archive |
| console-capture | M7.1c16 | 3 | 3 | none |
| csv | M7.1c19 | 23 | 2 | 1 syntax, 20 unrecognized |
| debrief-synthetic-csv | not a log | 6 | 0 | 6 unrecognized |
| e80-imu-txt | M7.1c8 | 54 | 54 | none |
| eggtimer-csv | M7.1b | 44 | 44 | none |
| eggtimer-csv-damaged | damaged | 1 | 0 | 1 unit |
| eggtimer-rtf | M7.1c4 | 1 | 1 | none |
| eggtimer-txt | M7.1c2 | 2 | 2 | none |
| empty | not a log | 1 | 0 | 1 unrecognized |
| entacore-aim-csv | M7.1c2 | 16 | 16 | none |
| entacore-xtra | M7.1c19 | 2 | 0 | 2 unrecognized |
| featherweight-summary-csv | M7.1c6 | 25 | 25 | none |
| flight-computer-params-csv | not a log | 3 | 0 | 3 unit |
| flight-summary-xlsx | not a log | 3 | 0 | 3 archive |
| fluctus-csv | M7.1c3 | 6 | 6 | none |
| fwgps-csv | M7.1c1 | 40 | 40 | none |
| jpeg | not a log | 1 | 0 | 1 unrecognized |
| kml | M7.1c1 | 43 | 43 | none |
| kml-map | not a log | 1 | 1 | none |
| mapped-table-csv | M7.1c15 | 6 | 6 | none |
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
| raven-csv | M7.1a | 80 | 80 | none |
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
| telemetry-rtf | M7.1c19 | 2 | 0 | 2 not-this-format |
| txt | M7.1c19 | 8 | 0 | 8 unrecognized |
| wind-profile-csv | not a log | 1 | 0 | 1 unrecognized |
| workbook-xlsx | M7.1c14 | 16 | 16 | none |
| xls | M7.1c19 | 1 | 0 | 1 unrecognized |
| xlsx | M7.1c19 | 4 | 0 | 4 archive |
| xml | M7.1c19 | 2 | 0 | 2 unrecognized |
| zip | M7.1c7 | 8 | 8 | none |
| **All** | | 1122 | 1016 | |

Files whose catalog family isn't their layout, moved to their layout's:

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
- `3a36f081e5be9e9d rtf -> telemetry-rtf`: a radio telemetry stream, lines such as `255,T,…` in a layout no maker's document names yet, saved inside an RTF document; filed as RTF, not a log, by its extension
- `70415b30a59d4411 rtf -> eggtimer-rtf`: an Eggtimer TRS's terminal download, its settings, summary and `T,Alt,…` table, saved inside an RTF document; filed as RTF, not a log, by its extension
- `8720c50cdeaf8ad3 rtf -> telemetry-rtf`: a radio telemetry stream, lines such as `255,T,…` in a layout no maker's document names yet, saved inside an RTF document; filed as RTF, not a log, by its extension
- `88f33383ccf621a0 aerospacenu-sillygoose-txt -> aerospacenu-sillygoose-gs-csv`: a SillyGoose flight as a ground station's conversion wrote it: a `CONFIG` line, then 28 comma-separated columns with no header, in another order than the board's own log, so nothing in the file says which column is which; for the column mapper
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
| AltOS | 52 |
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
| Featherweight Raven | 137 |
| GPS Visualizer | 1 |
| KML | 3 |
| Missile Works RRC3 | 57 |
| Missile Works RTx | 26 |
| NMEA 0183 receiver (talker GP) | 12 |
| Ozark Aerospace ARTS2 | 10 |
| PerfectFlite | 21 |
| PerfectFlite Alt15K/WD | 25 |
| PerfectFlite Pnut | 39 |
| PerfectFlite SL100 | 35 |
| PerfectFlite SLCF | 101 |
| Rocketry Ltd Mercury | 4 |
| Rocketry Ltd Nano V1 | 7 |
| Silicdyne Fluctus | 6 |
| SillyGooseV2 | 1 |
| Teensy data logger | 8 |
| fw2kml | 15 |
| phone app | 27 |
| unknown | 52 |
