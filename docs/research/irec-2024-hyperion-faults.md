# What failed on UND's IREC 2024 flight and Hyperion-II's descent

Two of the four MIT-licensed faulty flights in [public logs of faulty flights](anomalous-public-flights.md) have their
fault only from the trace: UND's IREC 2024 flight (loss of control under thrust) and Hyperion-II's flight of
2025-08-21 (no drogue deceleration). Fault diagnosis (M7.4 in the [roadmap](../ROADMAP.md)) needs each real case's
fault label marked as stated by its source or inferred from the trace (issue #638). This note asks whether the teams'
own words state the causes, why ESRA's score sheet gave UND an apogee 30% above its coast peak, and whether
Hyperion-II's air brakes opened. Read 2026-10-29: score sheets, rules, team reports, repositories, wikis, public
social posts and the flights' own logs; no GPL or AGPL source code was opened. **Stated** marks what a source says;
**Inference** marks what this note reads from a trace, with the method.

**In short.** Neither team states a cause, so both of issue #638's labels stay inferred. The traces now say more.
- **UND IREC 2024: no stated cause.** UND is team 129; its only public report [A2] is dated 2024-05-10, six weeks
  before the 2024-06-20 flight, with no post-flight section. ESRA scored recovery 150 of 150, "No Damage" [A1], which
  weighs against a shed fin or broken airframe (**Inference**, one word in a judge's column).
- **ESRA's 11,753 ft is the backup board's stored peak** (**Inference**, strong): 3,582.41 m at 26.44 s in its drogue
  state is 11,753.3 ft, under 1 ft from the score sheet. The rules make a COTS altimeter's log official [A3]. It reads
  2,694.5 ft (+29.7%) above the coast peak of 9,058.8 ft.
- **Hyperion-II: no apogee separation, not a tangled drogue** (**Inference**). Both apogee outputs fired, each with a
  6.7 to 10.0 g jolt lasting under 35 ms, yet the rocket fell nose-down as one body with the drag per unit mass it had
  on the way up. The whole rocket's deceleration at 95 m/s was about 1.5% of what an open drogue would add. The team
  calls the flight a success [S3].
- **Hyperion-II's air brakes: no sign they opened in the coast** (**Inference**). The flight's drag shows no step
  from 3.5 to 17 s, and with the sensor's offset removed its drag coefficient (about 0.40 to 0.43) stays below the
  team's clean-airframe run. No source says whether they deployed.

## UND IREC 2024 (team 129, AeroTech N3300R)

**What the sources say about the vehicle (Stated).**
- Single stage, 4 in fiberglass tubes, an N3300R-PS, about 30,000 ft and Mach 1.8 estimated; a custom aluminum boat
  tail threads into the motor case in place of the aft closure ([A2] p. 2).
- Fins of 1/8 in fiberglass sheet, "epoxied directly to the surface of the body tube" because the rocket is minimum
  diameter, with 1 in fillets and a seven-layer tip-to-tip layup ([A2] p. 3).
- "This was the teams first minimum diameter rocket", so much of the process "was experimental" ([A2] p. 11).
- The report plans two StratoLogger CF altimeters and an Eggtimer Quasar ([A2] pp. 5, 13). The flown logs are AltOS
  files from a TeleMega and a baro-only board [A5], so the avionics changed after the report (**Inference**).

**What failed at about 1.2 s.** No source read gives a post-flight statement on a fin, coupler, airframe or motor.
The parent note's trace stands: the primary's tilt goes 6.3° at launch, 23.5° at 1.09 s and 55.8° at 1.88 s, with
accelerations of 18 to 21 g on each axis and roll to about 910 °/s, and its log stops under thrust.
- **Recovery score (Stated):** 150 of 150, status "No Damage"; the "Judges Notes" column is empty [A1]. ESRA's rules
  call a recovery successful without "excessive damage", meaning the vehicle could fly again once its consumables
  are replaced ([A3] p. 27).
- **Weight (Inference):** a rocket that shed a surface-mounted fin or cracked its fin can would be hard to score as
  undamaged. That favors a loss of control without structural failure (weathercocking, a misaligned fin, the boat
  tail or the motor) over a fin failure, but only weakly.
- **The 2025 report [A4] (team 222)** moves to 6 in tubes, 5/16 in aluminum fins bolted through steel tabs to the
  motor mount, and an O5500X; its flutter paragraph calls a full-length fin's figure of "79" not safe enough (p. 4).
  It never mentions the 2024 flight, so this is not a stated lesson from that flight.
- **Size:** the coast peak of 9,058.8 ft is 69.7% below the 29,856 ft prediction [A1]. The cause's share of that is
  unbounded: no source names a mechanism.

**Why ESRA recorded 11,753 ft.**
1. **The rule (Stated).** Each vehicle carries a COTS barometric altimeter that gives "an official log of apogee for
   scoring"; postflight officials listen to its beeps, then check its readout ([A3] §2.5, pp. 13–14).
2. **The number (Stated).** "Actual Alt" 11,753 for team 129, launched 2024-06-20, against an "Apogee Predict" of
   29,856 [A1]. It is typed in on all four sheets, not computed. The altitude score is 0. Computed here: the formula
   ([A3] p. 26) gives −359.6, and the coast peak would give −464.4, so the transient changed nothing.
3. **The match (Inference, computed 2026-10-29).** In `IREC_Secondary_Flight_Data.csv` [A5] the `height` column peaks
   at 3,582.41 m at 26.44 s, state `drogue` (entered 24.53 s); 3,582.41 / 0.3048 = 11,753.3 ft. No other column fits:
   the backup's `altitude` above sea level peaks at 4,870.13 m (15,978 ft) and the primary's `height` at 1,229.82 m
   (4,034.8 ft). The backup's coast peak is 2,761.11 m (9,058.8 ft) at 24.24 s.
4. **Reading.** The official number is the backup board's stored maximum, which includes the pressure swing after
   the drogue charge. Which board the team named official is not stated anywhere read. Sign: high, by 29.7%, on this
   one case; it is a sample, not a bound.

## Hyperion-II, 2025-08-21 (Launch Canada 2025, Cesaroni M2505)

**What the team states.**
- "Successful **Hyperion II** launch, with first re-use of major airframe components which performed nominally"
  ([S3] `wiki/history.md` line 12, 2025-09-03).
- "Launched to 9364 feet at Launch Canada 2025" and "Reached speeds of 273 m/s" ([S4] p. 7). Hyperion I's entry on
  that page says "Successfully recovered"; Hyperion II's has no recovery line.
- An Instagram post of 2025-08-26 announces the launch; neither it nor its six comments mention recovery [S5].
- Design ([S1] `DSS.md`): the 24 in (61 cm) drogue shares one 56 cm carbon tube with the air brakes, three flaps cut
  from the body tube and driven out by a lead screw on the team's own flight computer; Kevlar shock cords.
- Prediction ([S1] `ork/outputs-v3/`): 27.5 m/s between drogue and main, apogee 3,001.84 m, top speed 306.65 m/s.
- Settings [S2]: the Blue Raven's apogee output fires 1.0 s after apogee, its main below 1,500 ft AGL; the TeleMega's
  `apogee_delay` is 3 s and `main_deploy` 425 m.
- Not found: a cause, a post-flight report, any mention of the drogue, shock cord or bag, or of air-brake deployment.

**What the trace shows (all Inference).** Logs from the team's MIT repository [S1] and its newer
`post-flight-analysis` repository [S2] (data CC BY 4.0), which adds the 500 Hz Blue Raven file and the TeleMega
settings; the two repositories' shared CSVs are byte-identical.
1. **Both apogee outputs fired.** The Blue Raven's apogee output reads 0.01 V from 25.40 to 26.40 s and its fired
   flag turns true at 26.92 s, the 1.0 s on-time and 1.5 s flag delay the maker's manual gives ([S6] p. 15). The
   TeleMega's drogue state is at 24.10 s; with its 3 s delay it fires at about 27.1 s.
2. **Neither apogee circuit opened cleanly.** The Blue Raven's apogee channel reads 3.96 to 4.01 V again until
   47.78 s, above its 3.8 V continuity threshold ([S6] p. 8). The TeleMega's drogue voltage decays from 4.12 V at
   27.21 s to 1.09 V at 33.7 s and 0.61 V at 47.9 s, while its main steps from 4.10 to 0.03 V at 48.02 s and stays.
   A failed match and a fired match with shorted leads both read this way; why is unknown.
3. **Each firing jolted the airframe.** The 500 Hz file needs a clock fix first: it repeats its first 0.7 s of
   thrust after 2.0 s of rest rows, so true time is file time − 2.82 s (sync codes fit 2.81 to 2.83 s). At true
   25.39 s, the instant the Blue Raven's output turned on: 6.7 g in one 2 ms sample, under 1 g within 25 ms. At true
   26.99 s: 10.0 g then 5.7 g, gone within 35 ms. That is within 0.1 s of the TeleMega's nominal firing; matched by
   barometric altitude, the two boards' clocks agree to 35 to 75 ms. Around both, |a| is about 0.02 g, free fall
   plus the sensor's offset.
4. **The rocket fell as one body.** k = a/(ρv²), with a the 500 Hz |a| averaged over 0.5 s, v the Blue Raven's
   inertial speed and ρ from its logged pressure and temperature. Coast, 150 to 250 m/s (4 to 10 s): k = 1.70 to
   1.88 × 10⁻⁴ m⁻¹. Descent, 100 to 183 m/s (35 to 47 s): 1.60 to 1.87 × 10⁻⁴ m⁻¹. The tilt holds at 159 to 166°
   (nose 14 to 21° off straight down). Equal k at equal speeds means equal drag per unit mass; two sections on a
   shock cord, or a drogue trailing partly open, would raise k and break the steady attitude.
   - **Size:** an open 24 in drogue (C_D 1.6 in the team's `.ork`, 18.61 kg) holds about 29.5 m/s at ρ = 0.9 kg/m³.
     At about 95 m/s (34.0 to 34.5 s) it would add about 10 g of deceleration; the whole rocket's measured total is
     0.146 g, about 1.5% of that, and most of it is the body's own drag.
   - **Sign:** the descent ran about 6 times the predicted 27.5 m/s, to 173.7 m/s (TeleMega) at 46.1 s.
5. **Air brakes: no step in the coast.** C_D = 2mk/A, with m = 18.61 kg and A = 0.0150 m² from the team's
   OpenRocket file: 0.427 at 5 s (Mach 0.66 to 0.69), 0.444 at 8 s, 0.481 at 12 s. The team's stored OpenRocket
   run, where the brakes are a mass only, gives 0.551, 0.500 and 0.470. No 0.5 s bin from 3.5 to 17 s steps more
   than about 6% from its neighbors. Over that window the raw bins run 0.42 to 0.56, above the stored run from
   13.5 s on; with the sensor's 0.017 g offset removed they lie flat at about 0.40 to 0.43, below it throughout.
   An open brake raises C_D, so no step and a level at or below the clean run argue against deployment. Both are
   compared at equal time, not equal Mach.
   - **Limits:** below about 60 m/s (after 18 s) the sensor offset swamps drag, so a late deployment can't be seen.
     The flap area is unpublished, so a deployment's size is unbounded. C_D scales with the mass assumed.
   - **The apogee gap came at burnout:** 9,364 ft (2,854 m) stated against 3,002 m predicted, −4.9%; the burnout
     speed was already 273 m/s stated against 306.65 m/s predicted, before any brake could act.
6. **The Blue Raven browned out when the main fired** at about 47.78 s: its battery reading fell from 4.09 to
   0.02 V, and its altitude rows after 47.80 s jump between 1,075 and 4,134 ft. They are not usable for a fit.

## Searched, not found

- **UND:** `UND-ARC/IREC-ARC-2024` (MIT): no post-flight text, no issues; its last two commits (2024-07-06) add the
  flight CSVs and a nose-cone log that is a Git LFS pointer whose object GitHub reports missing. `UND-ARC/IREC_2025`
  and `IREC_2026`: template READMEs. ESRA's 2024 report folder (126 PDFs) holds only team 129's pre-flight report;
  UND isn't in the 2024 podium schedule. Grand Forks Herald (2025-11-17), UND engineering news (2025-06-20), the
  North Dakota Space Grant 2025 slides, the club's sites and LinkedIn: nothing on the 2024 flight. Instagram shows
  12 posts before a login wall; Facebook and ESRA's 2024 launch-day video were not reviewed.
- **Hyperion-II:** all 29 commits of `werocketry/Hyperion-II`, no issues, pull requests or wiki pages; the
  organization's other repositories (an AGPL air-brake tool not opened, clean room); the team's site and wiki;
  three Instagram posts (highlights need a login); web and Rocketry Forum searches. Data was sent to Launch Canada
  (commit message "as submitted to LC"); any report sent with it is not public.

## What this suggests

1. **Keep both of issue #638's labels inferred, and sharpen Hyperion-II's.** "No apogee separation: both apogee
   outputs fired, drag per unit mass unchanged" says more than "no drogue deceleration", and M7.4 can test it: a
   diagnosis that names a tangled or torn drogue for this flight is wrong by the trace.
2. **Mark a 2 ms charge jolt as an event, not a fault.** Hyperion-II's 6.7 and 10.0 g spikes come with no change in
   drag. M7.4's evidence should test the drag after a charge, not the spike, to call a separation.
3. **Read a high-rate log's clock before trusting it.** The Blue Raven's 500 Hz file repeats its first 0.7 s of
   thrust, a 2.82 s offset. An importer that reads `Flight_Time_(s)` as is misplaces every event in it.
4. **ESRA's "Actual Alt" is an altimeter's stored peak, transients included.** Any comparison against ESRA score
   sheets should read the team's log where one exists, as the parent note suggested.

## Open questions

- UND IREC 2024: what failed at about 1.2 s? Only the team's own account would say; the nose-cone log is lost.
- Which UND board did ESRA's officials read as official?
- Hyperion-II: why did neither apogee charge separate the airframe (charge size, shear pins, a fouled bay shared
  with the air brakes)? Why did neither apogee circuit read open after firing?
- Did Hyperion-II's air brakes deploy after 18 s, where the trace can't see them?

## Licenses

- `UND-ARC/IREC-ARC-2024`, `werocketry/Hyperion-II`: MIT. `werocketry/post-flight-analysis`: code Apache-2.0, data
  CC BY 4.0. ESRA's 2024 rules: CC BY 4.0 (cover). ESRA's score sheet and team reports, the team wiki, site,
  sponsorship package, Instagram and the Blue Raven manual state no license: cited, quoted under 25 words,
  cached under `refs/` only.

## Sources

- [A1] ESRA, `2024_master_scores_final_7_05_24_rev4.xlsx`, team 129 rows, columns "Actual Alt", "Apogee Predict",
  "Recovery Score", "Status": <https://www.soundingrocket.org/2024-sa-cup.html>. Team number from
  `published_team_list_061124` in the same folder.
- [A2] UND ARC, "Team 129 Project Technical Report for the 2024 IREC", 28 pp., 2024-05-10, ESRA 2024 technical
  documents folder linked from <https://www.soundingrocket.org/2024-sa-cup.html>; pp. 2, 3, 5, 11, 13.
- [A3] ESRA, "Spaceport America Cup IREC Rules & Requirements", 2024 v1.4, 2024-03-04, §2.5 pp. 13–14, §2.7.1.4
  pp. 26–27: <https://www.soundingrocket.org/uploads/9/0/6/4/9064598/>
  `sa_cup_irec_rules_and_requirements_document_-2024_v1.4_20240304.pdf`.
- [A4] UND ARC, "Team 222 Project Technical Report" (2025 IREC), 71 pp., linked from
  <https://www.esrarocket.org/past-irec-events>; pp. 3–5, 19.
- [A5] `UND-ARC/IREC-ARC-2024@94a991b`, `Analysis/IREC_Primary_Flight_Data.csv`, `IREC_Secondary_Flight_Data.csv`,
  MIT: <https://github.com/UND-ARC/IREC-ARC-2024>.
- [S1] `werocketry/Hyperion-II@efda132`, `DSS.md`, `ork/outputs-v3/`, `flight-data/`, MIT:
  <https://github.com/werocketry/Hyperion-II>.
- [S2] `werocketry/post-flight-analysis@7ba6ee2`, `data/2025-08-21_hyperion2_lc25/`, code Apache-2.0, data
  CC BY 4.0: <https://github.com/werocketry/post-flight-analysis>.
- [S3] `werocketry/wiki@ecc8339`, `wiki/history.md` line 12: <https://github.com/werocketry/wiki>.
- [S4] `werocketry/werocketry.github.io@60544bf`, `assets/files/werocketry_sponsorship_package_2025-2026.pdf` p. 7.
- [S5] Instagram @werocketry, 2025-08-26: <https://www.instagram.com/werocketry/p/DN010cPYnTa/>.
- [S6] Featherweight, Blue Raven user's manual, 2026-05-12, pp. 8, 15:
  <https://www.featherweightaltimeters.com/uploads/1/0/9/5/109510427/blue_raven_users_manual_2026may12.pdf>.
