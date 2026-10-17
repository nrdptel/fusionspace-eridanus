# A published supersonic COTS flight: thrustMIT's Vayuvega at IREC 2025

**In short: Vayuvega is a well-documented supersonic COTS flight, but only its apogee was measured in public. It
can join [M2.3c7, faster than sound][m2-3c7] as an apogee-only flight past Mach 1, through its own decision
record.** The rocket flew an AeroTech O5500X-PS at the 2025 International Rocket Engineering Competition (IREC). It
reached 29,432 ft by the official barometric altimeter [S2, S3, S7]. The team's 349-page technical report gives
nearly all of the outer shape, the wet mass, the CG and the motor [S2]. It gives no part-by-part mass list, and the
parts it lists sum to 210 mm less than its overall length. No logged altitude, speed or Mach trace is public. The
best estimate of the top Mach is 1.64 to 1.71, from predicted speed only (**Inference**, §5). The report's
predictions ran 2.25% (the team's own 6-DoF code), 4.08% (OpenRocket) and 5.22% (RASAero II) above the measured
apogee. The paper's 2.55% matches none of them exactly; the competition's own figure is 2.54%, against the 30,198 ft
prediction on IREC's scoresheet [S1, S3].
**Confidence:** high for the apogee, the design numbers and the predictions, each read in its source. It is low
for the top Mach, and the paper's full text could not be read at all (§1). **What it moves:** M2.3c7's floor of 5
scored flights past Mach 1, where [RASAero II's comparison flights][rcf] found 4, and `accuracy.supersonic`.

**What this covers.** The paper's metadata and abstract [S1], the team's report [S2], IREC's final scoresheet [S3],
the team's website source and flight firmware [S4, S5], ThrustCurve [S6], the EasyMini manual [S7] and a university
news item [S8], all read 2026-10-17. The report is a scanned print with no text layer. It was read through OCR
(`tesseract`), which can misread digits. Values with a second source agree with it: the motor's impulse and masses
with ThrustCurve, the CG and CP with the stated 1.65 calibers, the Mach with the stated speed. HPR Sim flew
nothing for this note. It builds on [RASAero II's comparison flights][rcf], [the Rocket Flight Database][duke] and
[ADR-236: supersonic flights from published comparisons][adr-236], and does not repeat them.

[rcf]: rasaero-comparison-flights.md
[duke]: duke-flight-database.md
[adr-236]: ../decisions/0236-the-2026-10-11-review-supersonic-flights-from-published-comparisons.md
[m2-3c7]: ../decisions-and-roadmap.md#m2-3c7

## 1. The paper: its license is clear, its full text out of reach

- **What it is.** Sathe, Bhartia, Sridhar and Radhakrishnan, Aerospace Science and Technology 178, article 113242
  (November 2026). Crossref and Elsevier's API both record the published version under CC BY 4.0. The open-access fee
  is recorded as waived, with Manipal Academy of Higher Education as sponsor [S1]. Crossref lists 47 references,
  among them the team report and works on RASAero II and OpenRocket.
- **What it claims.** The abstract gives the 29,432 ft apogee and a 2.55% deviation "from the predicted trajectory".
  It also calls this the highest apogee in the 30k COTS category [S1]. The scoresheet disagrees. Team 218 reached
  29,541 ft, so Vayuvega is second of the 21 30k COTS rows [S3, counted from the sheet].
- **Routes tried.** These are the doi.org redirect, ScienceDirect's article and `pdfft` links (HTTP 403 to scripts),
  and Elsevier's article API (metadata only without a key). The open-access indexes were OpenAlex, Unpaywall and
  Semantic Scholar: each lists only the publisher's copy, and OpenAlex reports no repository copy. A search for a
  ResearchGate or team copy found none. In a browser the page stopped at a human-verification check, so the
  attempt ended there [S1].
- **What stays unknown.** The paper's own prediction and which tool made it, any flight trace it plots, its Mach
  figures, and whether any figure carries a credit line that takes it out of CC BY.

## 2. The measured flight: one number, from a barometric altimeter

- **The apogee.** 29,432 ft on 2025-06-12 at Midland, Texas [S3, S8]. The report names the COTS Altus Metrum
  EasyMini as the altimeter that logs the official altitude [S2 p. 55]. Its manual lists an MS5607 barometer and no
  accelerometer or GPS [S7 Table 5]. So the official number is a barometric apogee, read near zero speed. The Mach
  effects on baro readings in [altimeter Mach error](altimeter-mach-error.md) act in boost, not at apogee
  (**Inference**).
- **The scoresheet row.** Team 207 lists a target of 30,000 ft, an actual 29,432 ft and an apogee prediction of
  30,198 ft. The delta is 766 ft, and the % error is 0.02537, which is the delta over the prediction. The rocket was
  recovered with no damage, 4th in its category and 23rd overall [S3]; the team's site says 22nd overall [S4].
- **No trace is public.** The flight firmware's last commit, 2025-06-14, is titled as the code that ran on launch
  day. Per sample it writes to SD the time, pressure, baro altitude, three accelerations, GNSS position and state
  [S5, `LTS-iris.ino`]. The repository holds 35 files, all code plus a license and README: no log (counted 2026-10-17).
  The team's site, the scoresheet and the report hold none either [S2, S3, S4].

## 3. The design inputs: given and missing

All values below are the report's pre-flight design numbers, written before the flight. The table states facts in this
note's words; pages are the PDF's, which match the printed page numbers in the report's body.

| Input | Given? | What the source gives |
|---|---|---|
| Nose | yes | Tangent ogive, fineness 5.6, 700 mm, CFRP with a metal tip [S2 pp. 11, 67, 146] |
| Body | yes | 126 mm outer, 121 mm inner diameter; 3,500 mm overall [S2 pp. 10, 14, 67] |
| Body sections | partly | Bays of 630, 390 and 1,490 mm [S2 Table 3 p. 15]; with nose and tail they sum to 3,290 mm |
| Fins | yes | 4 trapezoids, mm: root 250 (254 on drawing), tip 40, semi-span 98.5, sweep 171.16, 4.5 thick |
| Fin section | yes | Hexagonal, 5° half-wedge angle, aluminum 6061 [S2 Table 6 p. 22, drawing p. 153] |
| Fin station | no | In words only: stringers to the motor bay's aft ring, slots in the boattail [S2 pp. 33, 311] |
| Boattail | yes | Conical, 80 mm long, 126 to 106 mm [S2 Table 8 p. 33] |
| Surface | yes | Painted tube Sa 0.3408 µm, profilometer per ISO 25178 [S2 p. 17] |
| Rail buttons | partly | Two airfoiled 1515 buttons; no drag figure [S2 p. 34] |
| Mass and CG | partly | Wet 36.8 kg; CG 2,062 mm and CP 2,270 mm from the nose at rail exit [S2 Table 16 p. 67] |
| Part masses | partly | No part list; payload 4.55 kg [S2 p. 44]; Feb 2025 update: 37.05 kg total [S2 pp. 313-314] |
| Motor | yes | AeroTech O5500X-PS, 98 mm, 21,384 N·s, 4.0 s burn [S2 Table 17 p. 68]; ThrustCurve 4.03 s [S6] |
| Rail | partly | ESRA rail 5,816 mm [S2 Table 18 p. 69]; the sims used 4.84 m of travel and 6° tilt [S2 p. 35] |
| Launch-day weather | no | The sims assumed 2,683 ft ASL, 35 °C, 0.903 atm and 5 m/s wind [S2 p. 35] |
| Logged trace | no | Firmware logs baro, ±16 g accelerations and GNSS [S5]; no file published |

- **The length gap.** The listed parts leave 210 mm of the 3,500 mm unexplained (**Inference**). Exposed coupler
  rings are a likely home: the aluminum motor-bay coupler has a 126 mm outer ring [S2 p. 21]. Their widths aren't
  given in the text read, so an importer must not guess them.
- **Mass without a breakdown.** Wet mass less the motor's 16.78 kg leaves 20.02 kg for the rocket (**Inference**).
  The CG margin checks out: (2,270 − 2,062) / 126 = 1.65 calibers, the report's own rail-exit stability [S2 p. 35].
  A lumped airframe with the ThrustCurve motor placed aft can match both numbers. The moment of inertia stays a guess.
- **One typo.** The propulsion text gives 21,348.0 N·s [S2 p. 37]; the table and ThrustCurve give 21,384 N·s
  [S2 p. 68, S6].

## 4. Which simulators predicted it, and how far off they were

The report used three tools: OpenRocket v23.09, RASAero II v1.0.2.0, and the team's own MATLAB 6-DoF code. The 6-DoF
took drag from ANSYS Fluent CFD [S2 pp. 71, 296, 322]. Its predicted-data table cites the 6-DoF code for the predicted
apogee [S2 Table 18 p. 69]. The errors below are the predictions' distance above the measured 29,432 ft
(**Inference**: this note's arithmetic).

| Prediction | Tool | Apogee (ft) | Above measured |
|---|---|---|---|
| Report, flight analysis | RASAero II | 30,967 | +5.22% |
| Report, flight analysis | OpenRocket | 30,632 | +4.08% |
| Report, predicted-data table | In-house 6-DoF | 30,093 | +2.25% |
| IREC scoresheet | Not stated | 30,198 | +2.60% (IREC: 2.54% of prediction) |
| Paper abstract | Not readable | Not given | 2.55% |

- **The paper's 2.55% fits no row exactly.** It would need a prediction of about 30,183 ft against the measured
  apogee, or 30,202 ft against the predicted one (**Inference**). The scoresheet's 30,198 ft is the closest.
- **The predictions moved.** Two progress updates in the report's appendix gave OpenRocket 31,037 ft and RASAero II
  31,965 ft in December 2024, then 30,451 ft and 30,851 ft in February 2025 [S2 pp. 296, 322]. Each update says
  it chose OpenRocket's lower value over RASAero II's. So the inputs were still changing up to the flight.
- **Sign.** Every published prediction sits above the flight: the report's final three by 2.25% to 5.22%, the
  earlier updates by 3.46% to 8.61% (**Inference**). Their launch-day assumptions, a 6° tilt and 5 m/s wind, can't
  be checked, so how much of the gap they explain is unbounded.

## 5. The top Mach number: predicted 1.71, never measured

- **Predicted.** By RASAero II, maximum dynamic pressure comes at 6,816 ft ASL and Mach 1.71, in air at 274.18 K
  [S2 p. 14]. The predicted-data table gives a peak speed of 568.43 m/s [S2 Table 18 p. 69]. The CFD drag table
  runs to Mach 1.71 [S2 Table 42 p. 173].
- **How the 1.71 is derived.** 274.18 K is close to the standard atmosphere's 274.65 K at that height
  (**Inference**), not the 35 °C day the same report assumes. At 274.18 K the speed of sound is 331.9 m/s, so
  Mach 1.71 is 567.6 m/s, matching the table. Starting from 35 °C at the pad with a 6.5 K/km lapse, the air there
  is about 300 K. There the speed of sound is 347.2 m/s, and 568.43 m/s is Mach 1.64 (**Inference**). So for that
  speed the top Mach is 1.64 to 1.71 by temperature; the speed itself is "Calculated" by an unstated tool.
- **Measured.** Nothing public. The EasyMini has a barometer but no accelerometer or GPS [S7]. The flight
  computer sets its ADXL345 to its top range, ±16 g [S5], but the predicted peak is 22.75 g [S2 Table 18 p. 69].
  If an axis along the rocket's length is logged, a speed integrated through boost would clip and read low
  (**Inference**; size unbounded without the log).
- **Band.** Either value puts the flight in the 1 to 2.5 band, at least 0.6 from any cut. The speed would need to
  be about 40% lower to fall below Mach 1 (**Inference**). [ADR-236][adr-236] lets HPR Sim's own Mach decide the
  band when none is measured.

## 6. Licenses and terms

| Source | Terms | What the project may do |
|---|---|---|
| Paper [S1] | CC BY 4.0 (Crossref, Elsevier API); site reserves TDM | Cite; cache if reached; check figure credits |
| Team report [S2] | None stated; public link on the team's site | Cite and cache under `refs/` only; no tables copied |
| IREC scoresheet [S3] | None stated on the page | Cite the row; cache under `refs/` only |
| Website source [S4] | No license file in the repository | Cite only |
| Flight firmware [S5] | MIT | Cite; reading is fine; nothing ported |
| ThrustCurve curve [S6] | As in [ThrustCurve data](thrustcurve-data.md) | As the catalog already does |

## 7. What the project can use

1. **A fifth candidate past Mach 1, on its own ADR route.** [M2.3c7][m2-3c7]'s done-when takes only flights on
   RASAero II's list whose design ships with RASAero II. Vayuvega is on neither, so admitting it needs a decision
   record. It would add a source kind, "a design built by hand from a published report", beside the shipped files
   of [ADR-236][adr-236]. With the 4 from [the comparison list][rcf], it makes 5. That meets the floor only while
   Raven stays above Mach 1. **Expected effect:** the first scored COTS flight between Mach 1.5 and 2.5.
2. **Terms such an ADR would need to set.** The design file is built from §3's values and kept in `refs/`, with only
   errors published. The 210 mm gap, fin station and inertia are recorded as stated assumptions, each with its
   apogee sensitivity. The flight is flagged "pre-flight design, apogee only, launch-day weather reconstructed". The
   comparison column shows the report's three predictions, not the paper's, until the paper is read.
3. **Launch-day weather.** The date and place are known [S8]; the launch time is not. The project's weather
   sources could bound the day's temperature and wind. Until then, how much of the 2.25% to 5.22% prediction gaps
   the day's wind and tilt explain stays unbounded.
4. **A wider lead.** The same scoresheet lists 21 30k COTS rows; 12 have altitudes above 1,000 ft and 5 above
   27,000 ft [S3, counted]. COTS rockets that reach 27,000 ft or more are likely supersonic (**Inference**).
   ESRA's 2025 page links a folder of technical documents that wasn't opened for this note [S3]. If it holds the
   other teams' reports, the IREC 30k COTS class could supply several flights past Mach 1 on one rule.

## 8. Open questions

- What prediction and which tool does the paper's 2.55% use? Does it plot the flight computer's trace, and does it
  state a measured top speed or Mach?
- What fills the 210 mm between the listed parts and the 3,500 mm length, and where do the fins sit?
- Did the as-flown mass match the 36.8 kg design value? The scoresheet's prediction (30,198 ft) differs from the
  report's (30,093 ft); when it was made isn't stated.
- What was the launch time, and the wind and temperature at the pad? Was the rail tilted 6° as simulated?
- Does ESRA's 2025 documents folder hold other 30k COTS teams' reports, and under what terms?

## Sources

1. **S1.** A. S. Sathe, A. N. Bhartia, S. Sridhar, J. Radhakrishnan, "Subsystem-level aerodynamic optimization with
   max-Q-based system integration and flight testing of an experimental sounding rocket", Aerospace Science and
   Technology 178, 113242, <https://doi.org/10.1016/j.ast.2026.113242>. Read: Crossref record
   <https://api.crossref.org/works/10.1016/j.ast.2026.113242> (license, references), Elsevier API PII
   S1270963826016202 (open-access fields), OpenAlex abstract, Unpaywall and Semantic Scholar records; 2026-10-17.
   Full text not reached.
2. **S2.** thrustMIT, "Project Vayuvega, Team 207 Project Technical Report to the 2025 IREC", 349 pp.,
   <https://pub-5e90a2f5e8c44905a47c1b15177024fe.r2.dev/public/technical-report/Vayuvega.pdf>, sha256 prefix
   `eb38d3dcb0d8e4dd`, fetched 2026-10-17 and cached in `refs/` with its OCR text.
3. **S3.** ESRA, IREC 2025 page <https://www.soundingrocket.org/2025-irec.html> and its final scoresheet
   `irec2025_final_scoresheet_8-2-25.xlsx` (posted 2025-08-02, sha256 prefix `b3bf5fd20ec0b043`), row for team 207;
   read 2026-10-17.
4. **S4.** thrustMIT website source, <https://github.com/thrustMIT/thrustMIT-new>,
   `src/components/ProjectDetail.jsx` at commit `ec5966f688`, read 2026-10-17.
5. **S5.** TMIT-Avionics, FC-Code-2025 (MIT), <https://github.com/TMIT-Avionics/FC-Code-2025>, `LTS-iris.ino` at
   the 2025-06-14 commit, read 2026-10-17.
6. **S6.** ThrustCurve.org search API, AeroTech O5500X-PS (3 data files),
   <https://www.thrustcurve.org/motors/AeroTech/O5500X-PS/>, read 2026-10-17.
7. **S7.** Altus Metrum, EasyMini manual, Table 5, <https://altusmetrum.org/AltOS/doc/easymini.html>, read
   2026-10-17.
8. **S8.** Manipal Academy of Higher Education news, 2025-06-27,
   <https://www.manipal.edu/mu/news-events/mit-manipal-s-thrustmit-team-achieves-outstanding-success-at-int.html>
   (redirects to the news page), read 2026-10-17.
