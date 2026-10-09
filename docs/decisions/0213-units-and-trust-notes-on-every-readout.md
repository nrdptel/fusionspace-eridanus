# ADR-213: US units and a trust note on every readout; M0.9c4 split in four (2026-10-09)

- **Status:** accepted; carries out [ADR-210][adr-210] and [ADR-211][adr-211] for the readouts they left (#392, #395), and splits M0.9c4
- **Summary:** `hpr weather`, `hpr motors list`, `search` and `fetch`, and `hpr analyze` give US units in brackets after the SI, as `hpr sim` does: a table's heading names both units (`m MSL (ft)`), and each cell gives the US figure in brackets after the SI one, both parts right-aligned. `hpr weather` adds °F to the temperature and mph to the wind, and keeps the pressure in hPa; `hpr analyze` gives an acceleration in g too. `hpr weather`, `hpr analyze` and `hpr motors show` end with the three-part note `hpr sim` has, and `--json` carries it as `trust`, beside a `kind` with four new values: `measured`, `forecast`, `reanalysis` and `copied`. Each note says "not yet checked" where nothing has checked its figures against what they stand for. M0.9c4 is split into c4 (this), c5 provenance (#380), c6 errors and help (#381) and c7 the plot and the site's prose (#382, #400).

[adr-164]: 0164-the-fusionspace-product-system.md
[adr-208]: 0208-the-design-audit.md
[adr-210]: 0210-us-units-in-brackets.md
[adr-211]: 0211-how-far-to-trust-a-result.md

**Context.** [ADR-210][adr-210] brought `hpr sim`, `hpr mc`, `hpr motors show` and the plot to
the product system's units rule ([ADR-164][adr-164] §6: SI first, US in brackets), and left the
other readouts to #392. [ADR-211][adr-211] gave `hpr sim`, `hpr mc` and the plot a *How far to
trust it* note, and left `hpr weather`, `hpr analyze` and `hpr motors show` to #395. M0.9c4 held
those two issues with #380 to #382: five issues, too many to ship and review in one change.

**Decision.**

1. **The split.** M0.9c4 takes #392's command-line readouts and #395. #380 (provenance) is
   M0.9c5, #381 (errors, help and typed numbers) M0.9c6, and #382 (the plot) M0.9c7, which also
   takes #392's last item, the site's prose, filed as #400 so #392 can close with its command
   line. Each keeps its issue's full list as its *done when*.
2. **Units in tables.** A table can't repeat the unit in every cell, so its heading names both,
   `dia mm (in)`, and each cell gives the bare figures, `38 (1.50)`. The SI figures align on
   their right edge, and so do the brackets, so both columns read down. The places follow
   `hpr motors show` for a motor's size (inches to a hundredth across, a tenth along) and
   ADR-210 for the rest: whole feet, whole mph. Two quantities are new:
   - the air's temperature in whole °F, from kelvin by the definitions (a Fahrenheit degree is
     five ninths of a kelvin; 273.15 K is 32 °F);
   - an acceleration in g to a tenth, standard gravity (9.80665 m/s²) as the plot's scale
     uses. No log format read so far records an acceleration, so a unit test sets one by hand.

   Weather pressure stays in hPa alone, as `data.md` gives it for weather. `--json` stays SI, as
   ADR-210 §4 keeps it.
3. **The notes.** Each has [ADR-211][adr-211]'s three parts and shape, on standard output after
   the result, wrapped at 100 columns, its link on a line of its own:
   - **`hpr weather`**, one note per kind of source, linking that source's page: a forecast
     (Open-Meteo, GFS, RAP), a balloon's measurement at its station and time (Wyoming), a
     reanalysis (ERA5). What is checked is HPR Sim's reading of the source, on recorded answers
     and files; the forecast or sounding itself is "not yet checked here against measured winds
     or a flight's log". What to go by: the latest forecast and the wind measured at the field,
     and the RSO.
   - **`hpr analyze`**: measured, the altimeter's own log, its heights from its barometer;
     HPR Sim's readings checked on an invented log, not yet on real logs in CI, with no check
     yet of a barometer's errors near Mach 0.9 (the site's page says why). What to log: the
     altimeter's own reading.
   - **`hpr motors show`**: copied from ThrustCurve.org's curve file, with the catalog's download
     date, or read from the file named; the total impulse and peak thrust held to OpenRocket's
     on every bundled curve (`hpr_motor::catalog`'s
     `openrocket_s_total_impulse_matches_every_bundled_curve`), no figure to a maker's or a
     certifying body's data. What comes first: the motor's printed data and the maker's
     instructions, and the RSO.

   The notes carry no number of their own: the only comparisons committed for these commands
   check HPR Sim's code, not the forecast, the log or the motor.
4. **JSON.** `Weather`, `Analyze` and `MotorShow` gain `kind` and `trust`, as `SimFlight` and
   `McRun` have. `kind` gains `measured` (a log, a sounding), `forecast`, `reanalysis` and
   `copied`. New fields and values only; nothing is renamed.

**Alternatives considered.**

- **Separate US columns** (`dia mm  dia in`). Rejected: a second column per quantity reads as a
  second quantity, and the bracket form is the one every other readout uses.
- **One note for every weather source.** Rejected: a balloon's measurement and a model's forecast
  are different kinds of figure, and the note's first part is to say which.
- **Notes on `hpr motors list`, `search` and `fetch` too.** Not now: they are lookups that lead
  to `hpr motors show` and `hpr sim`, whose notes cover the figures flown. `search`'s stock and
  prices have their note on the site ([motor stock](../motor-stock.md)).

**Consequences.** The audit's *data.md · Units for rocketry* row waits only on the site's prose
(#400). The trust-note item of `review.md`'s *Before release* passes, leaving 6 of its 11 items
failing. A spread for one flight, which `principles.md` §3 and the FusionSpace test also ask, is
#401, for M2.8.
