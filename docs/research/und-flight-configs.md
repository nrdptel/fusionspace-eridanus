# Which design, mass and motor flew on each UND Student Launch flight

**In short: six of the eight flights pair with a named design file; the rest need records that could not be found.**
The University of North Dakota's rocket club keeps its 2023 and 2024 NASA Student Launch designs and altimeter logs in
two MIT repositories ([public-flights](public-flights.md) found them). This note pairs each flight with the `.ork`
version, configuration, motor and liftoff mass that fit it, from the repositories' git history, their stored
simulations, and OpenRocket 24.12 run as an outside oracle. No team report (PDR, CDR, FRR or PLAR) for either season
could be found, so nothing on record states a flown mass or ballast, and no file names a part as ballast. Three
findings: the 2023-03-05 flight's gap is 24%, not 12.5% (the 6,386 ft file was reworked four days after it flew), and
a lighter rocket can't explain it; the 2024 full-scale file gained 1.72 kg (+8.3%) four weeks after its flight, which
moves that flight's error between −12.3% and −4.1%; the 2024 subscale flew 46% below its H268R simulation and 1.2%
below the team's later HP-H182R one. How sure: high for the file history and the stored numbers; low for which motor
flew on 2023-03-05 and on the 2024 subscale, and for the 2024 full-scale masses.

## How this was read

- Both repositories are MIT (GitHub's license API, read 2026-10-15), so their files may be cached and quoted. The
  local cache (`refs/research/public-flights/dl/`) has no git history; the history came from a blobless clone and the
  GitHub API. The `.ork` files are Git LFS objects; each revision was fetched by its id and unzipped outside `refs/`.
- Liftoff masses and refly apogees come from the OpenRocket 24.12 jar with its own motor database, run as an external
  oracle. The runs used here reproduce their stored apogees within 0.2% (files saved by 22.02) or 1% (by 15.03);
  older stored runs that were never re-run in their file can be off by more than 10%.
- Logged apogees are each StratoLogger's header value (primary / secondary). Where a header reads `PWRLOSS`, the peak
  is a 5-sample median; raw maxima there are ejection-charge spikes.
- `h(+n)` is the height n seconds after a log or simulation first passes 10 ft. It checks a boost's shape, which
  separates a mass change from a motor change: a lighter rocket climbs faster from the first second.

## The eight flights

| flight | logged apogee | file and run that fit | motor | liftoff | file vs flight |
|---|---|---|---|---|---|
| 2023 subscale, 2022-12-04 | 854 ft | `Scale_model_Nasa_Updated` (Simulation 6) | G80T-4 | 1.58 kg | 841 ft (−1.5%) |
| 2023 full scale, 2023-03-05 | 7,186 / 7,196 ft | none; day-of `V2_L1420` | L1420R? | 19.26 kg | 5,791 ft (−19%) |
| 2023 full scale, 2023-04-02 | 5,776 / 5,785 ft | final `V3_L1420` (`L1420`) | L1420R | 19.21 kg | 5,786 ft (+0.2%) |
| 2023 launch week | 5,146 / 5,157 ft | final `V3_L1420` (competition week) | L1420R | 19.21 kg | 5,787 ft (+12%) |
| 2024 subscale | 743 ft | `Scale_Post_Analysis_New_Motor` (H182R) | H182R? | 2.806 kg | 752 ft (+1.3%) |
| 2024 VDF, 2024-03-02 | 4,377 / 4,391 ft | `L1420_Emergency` (3/2/24) | L1420R | 20.67 / 22.39 kg | +14% / +4.3% |
| 2024 payload demo | 3,893 / 3,898 ft | none with its site or wind | L1420R | 22.39 kg | +13% to +17% |
| 2024 launch day, 2024-04-13 | 4,035 ft | `L1420_Emergency` (20 mph) | L1420R | 22.39 kg | 4,393 ft (+8.9%) |

The last column is the file's apogee over the flight's, minus one. The sections below say how far the flight reads
above or below the file, the flight's apogee over the file's. A `?` marks a motor no record states.

## 2023 season ([RocketModels-2023](https://github.com/UND-ARC/RocketModels-2023))

- **Subscale, 2022-12-04.** Flight 22 on altimeter 4047, 854 ft. `Scale_model_Nasa.ork` was saved on the flight
  day; its G80T-4 run at 15 mph stores 947 ft at 1.47 kg (the flight reads 9.8% below).
  `Scale_model_Nasa_Updated.ork`, committed 2023-02-02, adds 0.11 kg to one body tube and stores 841 ft
  (**Inference**: a fit made after the flight). The G80T matches the boost (h(+1, +2, +3) = 160, 368, 529 ft
  against the log's 146, 355, 536); the file's other motor, an H55W, reaches 1,033–1,221 ft (**Inference**: ruled
  out). Grand Forks airport reported 26–30 °F and 16–17 kt gusting 25–30 kt from 11:53 to 13:53 CST [4] (the
  whole day: 15–34 °F, 8–17 kt; the flight time is unknown), in line with the team's 15 mph run if the site was
  near Grand Forks (**Inference**).
- **Full scale, 2023-03-05 (the FRR flight).** Flights 29 and 23 on the two altimeters, both `PWRLOSS`; the peak comes
  20.8–20.9 s after the 10 ft mark. The file as it stood that day, `NASA_Rocket_V2_L1420.ork` (last changed
  2023-02-18, "Corrected for incorrect fins"), reflies its calm `L1420` run to 5,791 ft at 19.26 kg; the team's
  pre-flight export `Data_L1420.csv` says 5,788 ft. The flight reads 24% higher.
  - The 6,386 ft quoted in [public-flights](public-flights.md) is `NASA_Rocket_FRR_L1420_Nose_Shift.ork`, committed
    2023-03-09 with `Corrected_FRR_Simulation.csv` and `Simulation_VS_Demonstration_Altitude.xlsx` (Sheet1 plots the
    run, peak 6,385.7 ft, against `Primary.pf2`). Its final version lengthens the nose from 0.381 m to 0.914 m and
    swaps mass components for overrides: 16.93 kg at liftoff (−2.34 kg, −12%). No file says why.
  - **Less mass on the L1420R doesn't fit.** Reaching 7,185 ft on the flight-day file takes 14.45 kg (−4.81 kg,
    −25%). That rocket would reach h(+2) = 851 ft and peak 19.55 s after the 10 ft mark; the log shows 539 ft and
    20.8 s. The 2023-04-02 flight shows these altimeters read 4–10% low in boost while matching apogee; even so, the
    lighter rocket would read about 770 ft at +2 s (**Inference**).
  - **A bigger, slower motor fits (Inference).** Of the 266 thrust curves of 75 mm in OpenRocket's database flown on
    the flight-day file, the best fit to the four boost heights and apogee is about 5,500–5,900 N·s with a
    4.6–5.1 s burn; for example Cesaroni 5506M1230 gives 7,149 ft at 20.73 s. This is a profile fit, not a
    record: the runs saved for this flight are named L1420. The flight-day file's other runs name other motors
    (among them an L2200G, which reaches 6,583 ft but climbs to 1,029 ft by +2 s, too fast).
  - **Cold air narrows the gap but can't close it.** A StratoLogger converts pressure to height "according to
    the US Standard Atmosphere model" [3, p. 2], so it reads high in cold air: 2.3–7.3% with ground air at −6 to
    0 °C and 6–9% at −9 °C, from the hypsometric ratio between an isothermal column and a 6.5 K/km lapse
    (**Inference**). Grand Forks reported 15–20 °F (−9 to −7 °C) [4] before the logs were committed; the bay
    fell to 32 °F. Dense air also lowers the true apogee by 1.5%. After both, the flight still sits about 16–23%
    above the file (**Inference**). Wind and a 5° rod lower the file's apogee by 1.0–4.5%, the wrong sign.
- **Full scale, 2023-04-02.** Flights 30 and 24, 5,776 / 5,785 ft. The file on the day (V3 as of 2023-03-22, new surface
  finishes) predicted 6,094 ft (the flight reads 5.2% below); `V4_L1420.ork` the next day 5,985 ft at a 13.99 kg
  structure override. The final V3 (committed 2023-04-24: avionics 2.0 lb, new tube masses, larger main) stores 5,786 ft
  at 19.21 kg, within 0.2%. It is the cleanest pairing in either season, though it was saved after the flight
  (**Inference**: fitted to it).
- **Launch week.** Flights 31 and 25, 5,146 / 5,157 ft. The flight counters follow 04-02's on both units, so nothing
  flew between. No date or site is stated; NASA's event ran 2023-04-13 to 04-15 near Huntsville, and teams could fly
  from home [6]. The team's final-V3 exports give 5,787 ft calm, 5,716 ft at 10 mph, 5,634 ft at 15 mph; the flight is
  11.1% below calm. Adding 2.09 kg (+10.9%) matches both apogee and boost (h(+2, +3, +4, +6) = 576, 1,140, 1,715,
  2,695 ft against the log's 567, 1,122, 1,680, 2,629); a 20 mph wind gives 5,529 ft with a boost above the log
  (**Inference**: more mass or less thrust, not wind). The bay read 90–91 °F. Hot air would raise the true apogee and
  make the altimeter read low; without the air temperature, neither is bounded.

## 2024 season ([Nasa-SLI-2024](https://github.com/UND-ARC/Nasa-SLI-2024))

- **Dates.** NASA's 2024 handbook [5, p. 8] sets the subscale deadline 2024-01-08, the vehicle demonstration flight
  (VDF) 2024-03-04, the payload demonstration flight (PDF) 2024-04-01 and launch day 2024-04-13. The VDF flies the
  competition motor fully ballasted, and ballast may be at most 10% of unballasted pad weight [5, pp. 13–14]. The
  altimeter counters run without a gap: unit 4048 flew #35 subscale, #36 full scale 1, #37 `*_PDF`; unit 4047 flew #29
  and #30. The `*_PDF` logs were committed on 2024-03-30 (local time), two days before the PDF deadline. **Inference:**
  they are the payload demonstration flight, as [public-flights](public-flights.md) guessed; no report confirms it.
- **Subscale.** Flight 35, 743 ft, flown by 2023-12-22. `2024_NASA_L1420_Scale.ork` holds an H268R (default) and an
  H219T. Before the flight it stored 1,964 ft for the H268R; after it, the unnamed mass component rose 0.703 kg and
  the stored apogee fell to 1,383 ft; the flight reads 46% below it. `2024_NASA_Scale_Post_Analysis_New_Motor.ork`
  (2024-01-07) adds an HP-H182R run, 752.4 ft at 2.806 kg, plotted against the team's fit to the flight as "Updated
  OpenRocket (Motor, Drag Coefficients)". Yet the team's workbook (sheet Config, C11–C13) lists the H268R's masses and
  burn. A point-mass check (**Inference**) needs +1.22 to +1.27 kg (+42%) for an H268R to fall to 743 ft, against the
  team's 5.5 lb airframe (C3, "Need to double check"), which is lighter than the file; drag alone would need C_D ≈
  3.7. The data favor about 216 N·s, H182R class, but no record says which motor flew.
- **Trap:** the workbook's "OpenRocket Sim Data" sheet holds the 2023 subscale log and its 946.6 ft run, a template
  leftover. Don't pair it with the 2024 flight.
- **Vehicle demonstration flight, 2024-03-02.** Flights 36 and 29, 4,377 / 4,391 ft. `2024_NASA_L1420_Emergency.ork`
  (added 2024-03-03 with a new fin design for the materials at hand) has a run named "Full-Scale Launch Conditions
  (3/2/24)": 46.9° N, 96.8° W, 902 ft, 17.0 mph wind, 18 ft rail, L1420R (**Inference**: the date and site of the
  flight). The run's stored apogee by file revision:

| revision | date | liftoff mass | stored apogee | flight vs file |
|---|---|---|---|---|
| `c37eac0` | 2024-03-04 | 20.669 kg | 4,992.6 ft | −12.3% |
| `3283b34` | 2024-03-30 | 22.39 kg | 4,564.5 ft | −4.1% |

  The second revision sets mass overrides on the nose, both body tubes and avionics, and raises the unnamed mass
  component from 4.309 to 4.536 kg (+1.72 kg in all, +8.3%; apogee −8.6%). It came 28 days after the flight, the day
  the PDF data landed (**Inference:** weighed for the PDF). The team compared the flight with a Huntsville 20 mph run
  (4,761 ft, so −8.1%) and updated only the parachute drag, which changes the descent. Nothing in the repository
  explains the ascent gap.
- **Payload demonstration flight.** 3,893 / 3,898 ft, both `PWRLOSS`. No run carries its site, date or wind. Against
  the current file's 3/2/24 run (4,565 ft) it is −14.7%, against its Huntsville 20 mph run (4,393 ft) −11.4%. Its
  ground readings sit 211–258 ft below the VDF's: another site, or another day's pressure.
- **Launch day, 2024-04-13** (NASA's date; the CSV has none). A StratoLogger CSV with no header values. Its apogee
  is **4,035 ft** at 16.85 s (4,026 ft above the pad's 9 ft reading); the
  4,053 ft in [public-flights](public-flights.md) is a three-sample ejection spike (3,299 → 3,686 → 4,053 →
  4,033 ft). Of the file's Huntsville runs (34.7° N, 580 ft, 12 ft rail at 5°), only the 20 mph one is at the new
  mass: 4,392.6 ft, so −8.1%. The 0–15 mph runs are still at 20.669 kg. Scaling the file's own wind sweep, a calm
  day would put the gap near −13% (**Inference**). The wind that day is in no source reached.
- **Other files.** `2024_NASA_L1420.ork` (CDR era, 19.42 kg) and `Stripe.ork` (a larger N-motor vehicle) match no
  flight.

## Reports: none reached

Tried on 2026-10-15 for both seasons: web searches; the UND rocket team's site (2018 documents only); the club's
GitHub pages and all 25 of its repositories; North Dakota Space Grant's team and past-meeting pages (its 2022 club
deck [7] predates both seasons); NASA's 2023 and 2024 team, results and winners pages. NASA doesn't host past teams'
reports there. The handbook requires an FRR and a PLAR from every team, but none was found anywhere searched.

## What this suggests

1. **Fly five flights now, each against a named file:** 2023-04-02 (final V3, the cleanest), 2023 launch week (final
   V3; carry its −11% as unexplained), the 2023 subscale (Updated file, 15 mph), and the 2024 VDF and launch day on
   the Emergency file's L1420R. Record the 2024 flights at both candidate masses: the 8.2-point spread between them
   is larger than the effects the accuracy work is chasing. All five are fitted or post-flight files, so they
   measure the simulator against a team's tuned model, not a blind prediction; say so beside each.
2. **Hold out 2023-03-05.** Flown as an L1420R it adds a +24% "error" that is most likely an input
   (**Inference**). Hold out the
   2024 subscale from apogee statistics too, or mark its motor "inferred".
3. **Correct two figures in [public-flights](public-flights.md):** the 2023-03-05 file comparison (5,791 ft on the
   flight-day file, not 6,386 ft) and the 2024 launch-day apogee (4,035 ft, not 4,053 ft).
4. **State air temperature with every real flight.** A standard-atmosphere altimeter reads 2–9% high with ground air
   at −9 to 0 °C; that is as large as the errors being measured. The comparison should carry each flight's
   temperature or this band.

## Open questions

- Which motor flew on 2023-03-05, and what did the rocket weigh? Only the team's FRR or flight card would say.
- Which mass flew on the 2024 VDF and payload flight (20.67 or 22.39 kg), and which motor on the 2024 subscale?
- Launch week 2023's site, date and weather; launch-day wind and air temperature for every flight. Grand Forks
  airport records [4] would serve only if the sites are confirmed.
- Why does the 2023-03-09 rework lengthen the nose to 0.914 m, and does it describe the rocket that flew?

## Sources (read 2026-10-15)

1. UND-ARC/RocketModels-2023, MIT: https://github.com/UND-ARC/RocketModels-2023; files as named, history and LFS
   revisions through the GitHub API.
2. UND-ARC/Nasa-SLI-2024, MIT: https://github.com/UND-ARC/Nasa-SLI-2024; revisions `c37eac0`, `3283b34`, `49e16b0`,
   `b306bda`, `0bb7459` and others as named.
3. PerfectFlite, StratoLogger SL100 user manual (2011), printed p. 2; vendor copyright, cited only:
   http://www.perfectflite.com/Downloads/StratoLogger%20manual.pdf
4. Iowa Environmental Mesonet, Grand Forks (KGFK) ASOS hourly observations (public US data), 2022-12-04 to 2023-04-02:
   https://mesonet.agron.iastate.edu/request/download.phtml
5. NASA, 2024 Student Launch Handbook (a NASA public document), PDF pp. 4, 8, 12–14:
   https://www.nasa.gov/wp-content/uploads/2023/08/2024-sli-handbook-final-copy1.pdf
6. NASA, "NASA Announces Teams for the 2023 Student Launch Challenge":
   https://www.nasa.gov/centers-and-facilities/marshall/nasa-announces-teams-for-the-2023-student-launch-challenge
7. UND Advanced Rocketry Club, North Dakota Space Grant symposium deck (2022), cited only:
   https://ndspacegrant.und.edu/_files/docs/meetings/2022/university-of-north-dakota-advanced-rocketry-club.pdf
8. OpenRocket 24.12 (GPL), run only as an external oracle; its motor data come from ThrustCurve.org.
