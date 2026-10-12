# Flight-log samples, read

Written by `cargo xtask logs` from `log-import.json`; don't edit. Every flight-log sample under `refs/` read by HPR Sim's importer (M7.1a, ADR-251): the private collection's catalog, Debrief's fixtures and samples, and AerospaceNU's logs, each file once. A family is the catalog's name for a layout; *unrecognized* means no reader knows the layout yet.

| Source | Files | Not met before |
|---|---:|---:|
| collection | 1056 | 1039 |
| debrief | 23 | 14 |
| aerospacenu | 66 | 50 |

| Family | Increment | Files | Read | Refused |
|---|---|---:|---:|---|
| aerospacenu-fcb-csv | M7.1b | 20 | 0 | 20 unrecognized |
| aerospacenu-sillygoose-txt | M7.1b | 15 | 0 | 15 unrecognized |
| altimetercloud-csv | M7.1b | 1 | 0 | 1 unrecognized |
| altos-csv | M7.1a | 48 | 48 | none |
| altos-eeprom | M7.1b | 14 | 0 | 14 unrecognized |
| altos-telem | M7.1b | 2 | 0 | 2 unrecognized |
| binary | M7.1b | 4 | 0 | 4 unrecognized |
| blueraven-csv | M7.1b | 80 | 0 | 4 not-this-format, 76 unrecognized |
| blueraven-events-txt | M7.1b | 12 | 0 | 12 unrecognized |
| blueraven-summary-csv | M7.1b | 23 | 0 | 23 unrecognized |
| csv | M7.1b | 141 | 2 | 2 syntax, 137 unrecognized |
| debrief-synthetic-csv | M7.1b | 6 | 0 | 6 unrecognized |
| e80-imu-txt | M7.1b | 54 | 0 | 54 unrecognized |
| eggtimer-csv | M7.1b | 43 | 0 | 43 unrecognized |
| eggtimer-txt | M7.1b | 2 | 0 | 2 unrecognized |
| empty | not a log | 1 | 0 | 1 unrecognized |
| entacore-aim-csv | M7.1b | 1 | 0 | 1 unrecognized |
| entacore-xtra | M7.1b | 2 | 0 | 2 unrecognized |
| featherweight-summary-csv | M7.1b | 25 | 0 | 25 unrecognized |
| fwgps-csv | M7.1b | 40 | 0 | 40 unrecognized |
| jpeg | not a log | 1 | 0 | 1 unrecognized |
| kml | M7.1b | 43 | 0 | 43 unrecognized |
| missileworks-rtx | M7.1b | 5 | 0 | 5 unrecognized |
| nmea | M7.1b | 12 | 0 | 12 unrecognized |
| odf | M7.1b | 3 | 0 | 3 unrecognized |
| ods | M7.1b | 4 | 0 | 4 unrecognized |
| pdf | not a log | 1 | 0 | 1 unrecognized |
| perfectflite-csv | M7.1a | 14 | 14 | none |
| perfectflite-txt | M7.1a | 2 | 2 | none |
| pf2 | M7.1a | 175 | 175 | none |
| pfa | M7.1b | 25 | 0 | 25 no-data |
| png | not a log | 6 | 0 | 6 unrecognized |
| raven-csv | M7.1a | 80 | 80 | none |
| raven-fipa | M7.1b | 57 | 0 | 57 unrecognized |
| rrc3-csv | M7.1b | 50 | 0 | 50 unrecognized |
| rrc3-rff | M7.1b | 6 | 0 | 6 unrecognized |
| rtf | not a log | 3 | 0 | 3 unrecognized |
| txt | M7.1b | 39 | 0 | 39 unrecognized |
| xls | M7.1b | 1 | 0 | 1 unrecognized |
| xlsx | M7.1b | 32 | 0 | 32 unrecognized |
| xml | M7.1b | 2 | 0 | 2 unrecognized |
| zip | M7.1b | 8 | 0 | 8 unrecognized |
| **All** | | 1103 | 321 | |

Files whose catalog family isn't their layout, moved to their layout's:

- `9c5017ab68168088 altos-csv -> csv`: a spreadsheet's re-save of an AltOS export, no longer AltosUI's layout: its header edited to carry units, its `height` column deleted, and three columns holding formulas

The files read, by the logger each names:

| Logger | Files |
|---|---:|
| AltOS | 49 |
| AltOS TeleMetrum | 1 |
| Featherweight Raven | 80 |
| PerfectFlite | 16 |
| PerfectFlite Pnut | 39 |
| PerfectFlite SL100 | 35 |
| PerfectFlite SLCF | 101 |
