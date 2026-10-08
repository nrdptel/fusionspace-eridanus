# ADR-145: Recorded answers and their licences: stand-in ThrustCurve searches, CC BY 4.0 for the motor finder (2026-10-03)

- **Status:** accepted
- **Summary:** Recorded answers and their licences: ThrustCurve.org's three searches replaced by stand-ins (five invented motors per maker, and the in-stock motors' records carrying the motor finder's CC BY 4.0 values); motor.fusionspace.co's answers under CC BY 4.0, credited "Motor stock data from motor.fusionspace.co"

**Context.** ADR-130 §7 committed five ThrustCurve.org answers as test fixtures: three makers'
searches (250, 246 and 50 kB of ThrustCurve's motor records) and two downloads, each one data file
ThrustCurve marks public domain (`PD`). ThrustCurve.org grants no licence for its motor records;
its site reads "All rights under copyright reserved" (checked 2026-10-03). Separately,
motor.fusionspace.co's API page (<https://motor.fusionspace.co/api>, "Data licence", checked
2026-10-03) now licenses its responses, and the compilation in them, under CC BY 4.0, asks for the
credit "Motor stock data from motor.fusionspace.co", and permits storing recorded responses as
software test fixtures (ADR-129 §6, §7 predate it). The finder's answers relay ThrustCurve's
published figures for each motor they list.
On 2026-10-03 Neer decided to replace the three manufacturer records with invented motors in the
same format, keeping the two public-domain curves. Wholly invented records can't exercise the join
against the finder's 282 in-stock motors, whose names they would not carry, so the build kept 15
wholly invented motors plus 282 records carrying only the motor finder's CC BY 4.0 values
(ThrustCurve.org's published figures as the finder relays them). On 2026-10-03 Neer also decided
that the 282 records rest on his site's CC BY 4.0 license and on their being published factual
figures.

**Decision.**

1. **Stand-in searches.** `thrustcurve-search-{aerotech,cesaroni,loki}.json` keep the API's shape
   (criteria, `matches`, `results`, `source_url`; each record's fields in the API's order). They
   hold 297 records (158, 104 and 35, 0.17 MB in all, against 0.55 MB), of two kinds:
   - **15 wholly invented records,** five per maker: designations ending `-INVENTED`, common
     names and lengths no record of ThrustCurve's 2026-10-01 searches has, ids `e000…`,
     `Example-` cases, `example.test` links and a made-up certifying body. Between them they cover
     every record field, a hybrid, out-of-production and zero-file records, and missing optional
     fields.
   - **282 records, one per motor in the committed `motor-finder-in-stock.json`** (153, 99 and
     30). They carry only the values that CC BY 4.0 answer states (designation, common name,
     class, diameter, type, total impulse, average thrust, burn time, delays, `delayAdjustable`,
     case, propellant, sparky; availability from its `discontinued`), which are ThrustCurve's
     published figures as the finder relays them, already committed under ADR-129. Each adds an
     invented id and an invented count of data files (1; 3 for J450DM, so a test reads that
     record's own). J450DM and F27R/L keep the ids of the two downloads, so the join still leads
     to a committed file. The fields only ThrustCurve states (maker abbreviation, length, weights,
     peak thrust, certifying body, update date, `infoUrl`, `source_url`) are left out, as the API
     leaves out fields with no value.

   So no ThrustCurve.org search answer is committed as such any more. The licensing basis of the
   282 records, as Neer decided on 2026-10-03 (Context), is the finder's CC BY 4.0 grant, and
   that they are published factual figures.
   `each_stand_in_record_carries_only_the_finders_values` checks both kinds, record by record.
2. **The tests keep their intent.** Every test of ADR-130 still runs, with its counts moved to the
   stand-ins (158, 104 and 35 records; 297 in all). The join still maps 282 of 282 in-stock
   motors, and its report's table is unchanged; the miss test's edits and counts are unchanged.
   What changes is the evidence: the committed join now shows that the code keeps the rule on
   records in ThrustCurve's shape. Its "no data file listed" column, and the checks that each
   match lists a file and has the finder's figures, hold on the stand-ins by construction. The
   facts on ThrustCurve's own records (282 of 282 mapped, each listing a file, with the finder's
   figures) are ADR-130's measurement of 2026-10-01, last run on 2026-10-01 (the test added
   in M5.4b), kept in its text, `docs/motor-stock.md` and the report's preamble, not re-run.
   The example `motor_stock` says which of its records are invented, and prints the record's
   average thrust beside the file's (465 N against 465.6 N) instead of the peak thrust, which the
   stand-in record leaves out.
3. **The motor finder's credit.** `motor_finder::ATTRIBUTION` opens with the licence's words,
   "Motor stock data from motor.fusionspace.co", names CC BY 4.0 with its link, and keeps the
   site's caution to check stock and price on the vendor's page. `hpr motors search`, the example
   and the guide print it. `THIRD-PARTY-NOTICES.md` and `validation/refs.lock.toml` give the
   finder's answers as CC-BY-4.0.

**Consequences.** No ThrustCurve.org search answer is committed as a fixture from here on; its two
public-domain curve files are. The searches recorded on 2026-10-01 remain in `main`'s history,
which is not rewritten (rewriting it would need a force-push, which the project's rules forbid).
The bundled catalogue (`crates/hpr-motor/data/thrustcurve/`, ADR-005) is unchanged: its 32 curve
files are all marked `PD`, but `catalog.json` also holds ThrustCurve-only fields (lengths,
weights, peak thrusts, certifying bodies, dates) that no CC BY source relays; that is
[#295](https://github.com/nrdptel/hpr-sim/issues/295).
