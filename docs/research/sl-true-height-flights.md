# Student Launch true height: each flight

This part holds the per-flight table of [Student Launch apogee errors against true height](sl-true-height.md):
each of the 55 flights' date, site, temperatures, height-over-reading factor and error, raw and against the
height climbed in the day's air. The method, the statistics and the sources are in the main note; the table was
computed from the stated inputs by the script the main note names. Read 2026-10-18.

Sites: BB Bayboro NC, CE Cedarville OH, CU Culpeper VA, DZ Dalzell SC, EL Elsberry MO, HV Bragg Farms AL,
MI Milford IN, PB Palm Bay FL, PC Plant City FL, PE Pence IN, SA Samson AL, SC South Charleston OH, TO Three
Oaks MI. Stated: the report's launch temperature. f A, B, C: the three methods; error, true uses A, else C.

| Flight | Date | Site | ERA5 2 m, °C | Stated, °C | f A | f B | f C | Sim | Error, raw | Error, true |
|---|---|---|--:|--:|--:|--:|--:|---|--:|--:|
| N15s | 2014-12-20 | BB | 5.9 | 3.3 | 0.965 | 0.966 | 0.957 | S | −4.9% | −1.5% |
| N15v | 2015-02-28 | BB | 3.8 | 5.8 | 0.952 | 0.956 | 0.963 | U | −3.1% | +1.8% |
| N16s | 2015-11-28 | BB | 18.7 | 21.1 | 1.012 | 1.011 | 1.019 | S | +74.1% | +72.1% |
| F16s |  | PC |  |  |  |  |  | U | +1.1% |  |
| N17s1 | 2016-12-17 | BB | 11.5 | 15.6 | 1.006 | 0.987 | 1.001 | S | +1.7% | +1.1% |
| N17s2 | 2017-01-21 | BB | 13.0 |  | 1.009 | 0.993 |  | U | +2.3% | +1.4% |
| N17v | 2017-02-25 | BB | 21.6 | 23.3 | 1.027 | 1.024 | 1.030 | S | +3.4% | +0.7% |
| F17s | 2016-12-17 | PC | 26.9 | 25.6 | 1.039 | 1.041 | 1.036 | U | +14.8% | +10.5% |
| F17w | 2017-04-08 | HV | 17.8 |  | 1.002 | 1.014 |  | U | +13.6% | +13.3% |
| N18s | 2017-11-18 | BB | 18.2 |  | 1.016 | 1.012 |  | U | −4.2% | −5.7% |
| N18v | 2018-02-24 | BB | 23.8 |  | 1.027 | 1.029 |  | U | −4.2% | −6.7% |
| G18v | 2018-02-24 | SA | 25.2 | 16.7 | 1.035 | 1.036 | 1.006 | U | +14.8% | +10.9% |
| D18v | 2018-03-03 | TO | 5.2 | 5.0 | 0.962 | 0.967 | 0.966 | S | −0.2% | +3.7% |
| F18v2 | 2018-02-17 | PC | 28.5 |  | 1.036 | 1.047 |  | D | −0.2% | −3.7% |
| F18f3 | 2018-03-17 | PC | 23.8 |  | 1.026 | 1.030 |  | U | +0.5% | −2.0% |
| N19s | 2018-11-17 | BB | 12.7 |  | 0.988 | 0.990 |  | U | +21.0% | +22.5% |
| N19v | 2019-02-09 | BB | 7.1 |  | 0.963 | 0.968 |  | U | +17.6% | +22.1% |
| D19v | 2019-03-02 | PE | 2.0 | −0.6 | 0.953 | 0.958 | 0.949 | U | −14.3% | −10.1% |
| F19w | 2019-04-06 | HV | 22.2 | 21.1 | 1.023 | 1.029 | 1.025 | D | +4.5% | +2.1% |
| N20s | 2019-11-23 | BB | 12.9 |  | 1.004 | 0.992 |  | U | +19.3% | +18.8% |
| N20v | 2020-02-23 | BB | 12.0 | 14.4 | 0.987 | 0.987 | 0.996 | U | +38.7% | +40.5% |
| D20s | 2019-12-07 | TO | 1.3 | 1.1 | 0.951 | 0.954 | 0.953 | U | −8.1% | −3.4% |
| D20v | 2020-02-16 | TO | 1.8 |  | 0.960 | 0.957 |  | U | −2.0% | +2.1% |
| U20s | 2019-11-24 | PE | 10.0 |  | 0.991 | 0.989 |  | U | +9.7% | +10.7% |
| U20v | 2020-02-15 | PE | −0.6 | 2.2 | 0.964 | 0.949 | 0.959 | U | +8.1% | +12.1% |
| N21S | 2020-11-21 | BB | 19.0 | 15.6 | 1.013 | 1.011 | 0.999 | U | +63.0% | +60.9% |
| N21V | 2021-02-21 | BB | 5.4 | 6.1 | 0.957 | 0.963 | 0.965 | U | +43.3% | +49.8% |
| D21S | 2020-11-13 | TO | 2.5 | 2.2 | 0.957 | 0.959 | 0.958 | U | +3.0% | +7.6% |
| D21V | 2021-02-27 | TO | 5.1 | 6.1 | 0.981 | 0.969 | 0.973 | U | +1.1% | +3.0% |
| W21S |  | EL |  | 13.3 |  |  | 0.997 | D | −7.5% | −7.3% |
| W21V | 2021-02-20 | EL | 1.2 | 3.3 | 0.960 | 0.953 | 0.960 | U | −7.0% | −3.1% |
| N22S | 2021-11-20 | BB | 12.9 | 12.8 | 0.988 | 0.989 | 0.989 | U | +17.8% | +19.3% |
| N22V | 2022-02-19 | BB | 13.6 | 8.3 | 0.987 | 0.994 | 0.976 | U | +10.5% | +12.0% |
| D22S | 2021-11-20 | TO | 5.1 | 2.8 | 0.966 | 0.969 | 0.961 | D | +0.4% | +4.0% |
| D22V | 2022-02-24 | TO | −2.4 | −2.8 | 0.942 | 0.941 | 0.940 | U | −10.6% | −5.1% |
| X22V | 2022-02-12 | CU | 15.4 | 12.8 | 0.997 | 1.004 | 0.994 | U | +5.6% | +6.0% |
| N23S | 2022-11-19 | BB | 12.6 | 11.7 | 0.986 | 0.990 | 0.986 | D | +7.2% | +8.8% |
| N23V | 2023-02-26 | BB | 10.1 | 17.8 | 0.989 | 0.982 | 1.009 | U | +4.3% | +5.5% |
| D23S | 2022-12-04 | TO | 0.7 | −2.8 | 0.950 | 0.953 | 0.941 | D | +2.6% | +8.0% |
| D23V | 2023-02-18 | TO | 4.1 | 4.0 | 0.968 | 0.965 | 0.964 | S | +3.0% | +6.4% |
| N24S | 2023-11-18 | BB | 21.6 |  | 1.025 | 1.024 |  | D | +0.3% | −2.2% |
| N24V | 2024-02-24 | BB | 15.2 | 15.6 | 0.995 | 1.002 | 1.003 | U | −3.7% | −3.2% |
| T24S | 2023-12-16 | SC | 11.4 | 7.2 | 0.994 | 0.993 | 0.979 | D | +1.3% | +1.9% |
| T24V | 2024-02-17 | MI | −5.7 | −7.2 | 0.923 | 0.932 | 0.927 | U | +26.2% | +36.8% |
| T24P | 2024-03-24 | CE | 2.8 | 2.8 | 0.961 | 0.963 | 0.963 | U | +9.3% | +13.7% |
| M24S |  | PB |  | 18.9 |  |  | 1.014 | U | +4.2% | +2.8% |
| M24V | 2024-02-10 | PB | 24.1 | 25.0 | 1.026 | 1.030 | 1.033 | U | +13.8% | +10.9% |
| N25S | 2024-11-16 | BB | 17.1 | 15.6 | 1.006 | 1.007 | 1.002 | D | +3.7% | +3.1% |
| N25V | 2025-03-08 | DZ | 26.2 | 21.1 | 1.030 | 1.041 | 1.024 | U | +14.5% | +11.1% |
| T25S |  | MI |  | 10.0 |  |  | 0.987 | D | +13.9% | +15.4% |
| T26S | 2025-10-25 | CE | 7.9 | 16.7 | 0.987 | 0.980 | 1.010 | U | +6.5% | +7.9% |
| P26S1 | 2025-12-07 | SA | 11.1 | 14.1 | 1.012 | 0.987 | 0.998 | D | +5.0% | +3.8% |
| P26S2 | 2026-01-11 | SA | 14.2 | 24.5 | 0.989 | 0.996 | 1.032 | D | −3.0% | −1.9% |
| P26V | 2026-02-22 | SA | 12.9 | 20.4 | 0.978 | 0.992 | 1.018 | U | −2.1% | +0.1% |
| P26P | 2026-03-29 | SA | 17.7 | 23.4 | 1.010 | 1.008 | 1.027 | U | −7.7% | −8.6% |

Pages read for date, place, time and temperature, in the team reports keyed and linked in
[Student Launch apogees](student-launch-apogees.md): N15c p29–32, N15f p18–19, N16c p34–35, N17c p28–31, p39,
N17f p45, p160, N18c p58, N18f p89–90, N19c p36, N19f p61, N20c p21, N20f p84, N21C p28, N21F p100, N22C p33,
N22F p83, N23C p34, N23F p153, N24C p41, p160, N24F p151, N25C p39, N25F p94; D18f p43, p102, D19f p62, D20c p37,
D20f p95, D21C p35, D21F p83, D22C p30, D22F p69, D23C p34–35, p48, D23F p94; U20c p48, U20f p45; G18f p81; F17c p38,
p109, F17p p3, F18f p67, F19p p9; W21C p25, W21F p108; X22F p19; T24C p27, T24F p73, T24Ad p4, T25C p53, T26C p46–49;
M24C §3.4, M24F §5.1; P26C p45, P26F p96, P26Ad p6.

- **Sim:** S, the report says standard conditions; D, the day's temperature or conditions entered; U, not
  stated in the passages read. **f B, f C** need a date and a stated temperature respectively.
- **Times** for NC State's 2024–25 subscale and VDF come from the team's launch log (AirbrakesV2
  `metadata.json`, MIT); ERA5 2 m is Open-Meteo's, downscaled to the site's elevation. Sources: the main note.
