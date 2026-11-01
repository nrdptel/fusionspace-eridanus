# Ghost files from RocketPy's flight logs

Written by `cargo xtask ghost`. Each of the 22 real flight logs in RocketPy's repository with an altitude or a GNSS track, read and laid out as a ghost file; each file read back as JSON by Ajv, as CZML by CesiumJS and as glTF by Khronos's glTF-Validator and glTF-Transform, and each format held to the others at every moment (539356 moments compared). "Highest", "lowest" and "farthest" are what the file holds, in meters from the pad. Rows left out are counted by why: a height too fast to reach, a height too far under the pad, or a clock or a height that isn't usable.

| Log | Read by | Clock | Moments | Height from | Highest | Lowest | Farthest | Fixes kept | Too fast | Under the pad | Not usable | Pad |
|---|---|---|---:|---|---:|---:|---:|---:|---:|---:|---:|---|
| `NDRT_2020/ndrt_2020_flight_data.csv` | own reader | liftoff | 1819 | the log's altitude | 1320 m | -2 m | none | 0 of 0 | 0 | 0 | 0 | not known |
| `andromeda/flight_data.csv` | own reader | own clock | 1139 | the log's altitude less its pad | 3417 m | -21 m | none | 0 of 0 | 0 | 0 | 0 | not known |
| `juno3/cots_altimeter.csv` | own reader | liftoff | 609 | the log's altitude | 3213 m | -9 m | none | 0 of 0 | 1 | 1 | 0 | not known |
| `prometheus/2022-06-24-serial-5115-flight-0001-TeleMetrum.csv` | own reader | liftoff | 5710 | the log's altitude | 3904 m | -4 m | 1170 m | 5712 of 7081 | 2 | 0 | 0 | fixes before liftoff |
| `prometheus/2022-06-24-serial-6583-flight-0003-TeleMega.csv` | own reader | liftoff | 5722 | the log's altitude | 3898 m | -14 m | 1171 m | 5723 of 6234 | 1 | 0 | 0 | fixes before liftoff |
| `EPFL_Bella_Lui/bella_lui_flight_data_filtered.csv` | mapping | liftoff | 768 | the log's altitude | 459 m | -17 m | none | 0 of 0 | 0 | 0 | 0 | not known |
| `astg/altimeter_halcyon.csv` | mapping | liftoff | 65882 | the log's altitude | 3471 m | -1 m | none | 0 of 0 | 0 | 0 | 0 | not known |
| `astg/gnss_halcyon.csv` | mapping | own clock | 2056 | none | none | none | 7685 m | 2056 of 2056 | 0 | 0 | 0 | first fix |
| `astra/flight_data.csv` | mapping | own clock | 62 | the log's altitude | 3249 m | 0 m | 1231 m | 59 of 64 | 0 | 0 | 2 | first fix |
| `camoes/flight_data.csv` | mapping | liftoff | 14346 | the log's altitude | 3016 m | -20 m | none | 0 of 0 | 0 | 0 | 0 | not known |
| `camoes/gnss_flight.csv` | mapping | own clock | 500 | none | none | none | 1205 m | 500 of 500 | 0 | 0 | 0 | first fix |
| `erebus11/flight_data_filtered.csv` | mapping | own clock | 746 | the log's altitude | 3002 m | 0 m | none | 0 of 0 | 0 | 0 | 0 | not known |
| `genesis/flight_data_faraday.csv` | mapping | liftoff | 17933 | the log's altitude | 2917 m | -2 m | none | 0 of 0 | 0 | 0 | 0 | not known |
| `genesis/gnss_faraday.csv` | mapping | own clock | 246 | none | none | none | 600 m | 246 of 246 | 0 | 0 | 0 | first fix |
| `hedy/cats_tust/filteredDataInfo.csv` | mapping | liftoff | 24563 | the log's altitude | 5232 m | -1 m | none | 0 of 0 | 0 | 0 | 0 | not known |
| `hedy/cats_tust/flightInfo.csv` | mapping | liftoff | 24563 | the log's altitude | 5230 m | 0 m | none | 0 of 0 | 0 | 0 | 0 | not known |
| `hedy/cats_tust/gnssInfo.csv` | mapping | own clock | 386 | none | none | none | 1022 m | 386 of 388 | 0 | 0 | 0 | first fix |
| `juno3/cots_GNSS.csv` | mapping | own clock | 390 | GNSS altitude | 3369 m | -26 m | 1481 m | 390 of 444 | 0 | 0 | 0 | first fix |
| `juno3/srad_telemetry.csv` | mapping | own clock | 44859 | GNSS altitude | 3401 m | -28 m | 2660 m | 51224 of 72709 | 0 | 3400 | 2965 | first fix |
| `lince/main_data.csv` | mapping | liftoff | 44397 | the log's altitude | 3629 m | -8 m | none | 0 of 0 | 57 | 0 | 0 | not known |
| `polito/altimeter_cavour.csv` | mapping | liftoff | 404 | the log's altitude | 2789 m | 0 m | 815 m | 404 of 404 | 0 | 0 | 0 | fixes before liftoff |
| `valkyrie/flightInfo_merged.csv` | mapping | liftoff | 12578 | the log's altitude | 2059 m | 0 m | none | 0 of 0 | 0 | 0 | 0 | not known |
