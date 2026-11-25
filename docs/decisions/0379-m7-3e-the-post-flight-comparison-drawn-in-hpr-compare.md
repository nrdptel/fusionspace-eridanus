# ADR-379: M7.3e, the post-flight comparison drawn by `hpr compare --plot` (2026-11-24)

- **Status:** accepted; carries out [M7.3e][m7-3e] ([ADR-257][adr-257] §4, [ADR-314][adr-314]); changes no number the residuals print for a log with no GNSS fix
- **Summary:** `hpr compare --plot FILE.svg` draws the log's altitude and vertical speed from liftoff to landing beside the simulation's: as designed (the 1976 standard atmosphere, no wind), and under the launch day's conditions where they are given. Those come from Open-Meteo's archive of past forecasts at the site and launch time (`--launch-time`, or `--launch-time log` for an AltosUI CSV's GPS time), a weather profile (`--weather`), or values entered (`--wind`, `--temperature`, `--pressure`), so it works offline from values entered or a cached answer. The site is the log's GNSS fixes before liftoff unless one is given. The figure's table gives each reading's provenance.

[m7-3e]: ../decisions-and-roadmap.md#m7-3e
[adr-257]: 0257-the-2026-10-13-review-calibrated-intervals-and-the-post-flight-comparison.md
[adr-314]: 0314-m7-3-in-five-residuals-first.md
[adr-157]: 0157-hpr-sim-plot.md
[adr-175]: 0175-the-plot-in-the-product-system.md
[page]: ../comparing-a-flight.md#draw-it

**Context.** NASA Student Launch asks a team to compare its predicted flight with its flight data
after each launch, and ADR-257 §4 makes that one `hpr` command: predicted and logged altitude and
velocity, liftoff to landing, as designed and under launch-day conditions, the log's site and date
with the weather source's wind and air or values entered, with each reading's provenance.
`hpr compare` (M7.3a) already lines a log up with a simulation and reads both the same way, and
`hpr weather` already fetches Open-Meteo's archive. Neither drew anything, and the comparison flew
one simulation in whatever air its options gave.

**Decisions.**

1. **One command, an option.** The figure is `hpr compare --plot`, not a new command. The
   residuals and the figure come from one library call (`hpr_forensics::residuals::compare`), as
   ADR-314 asks, so they share the clocks and the heights. Two library functions give the lines:
   `logged_lines`, every logged sample from liftoff to landing (the residuals drop those past the
   simulated flight's end), and `Simulated::read_lines`, the simulated flight read as the log's
   sensor reads it, from its liftoff to its end.
2. **"As designed" is the design in the 1976 standard atmosphere with no wind,** at the site and on
   the rail given. It is what the design predicts before anyone looks at the day. "On the launch
   day" is the same flight in the day's air and wind. The text's residuals are against the launch
   day's flight where any condition is given, and against the flight as designed otherwise, as
   before; only `--plot` flies both.
3. **The launch day comes from one source at a time.** `--launch-time TIME` asks Open-Meteo's
   archive of past forecasts at the site, the profile checked as `--weather`'s is (it gives winds,
   and the site's elevation lies within its levels). `--weather` flies a profile from any of
   `hpr weather`'s sources that gives winds and reaches the site's height. Values entered are `--wind` and `--wind-from`, as `hpr sim` takes them,
   and the air at the pad: `--temperature` (°C) and `--pressure` (hPa), the 1976 standard shifted
   to pass through them (`Ussa76::anchored`), either one the standard's at `--elevation` when left
   out. `--launch-time` conflicts with the others.
4. **Entered air past the records is refused.** A temperature outside −90 to 60 °C, past the World
   Meteorological Organization's ground records, is refused. So is a pressure more than 15% from
   the standard's at the pad's height, past the sea-level records (870 and 1,083.8 hPa, the second the highest below 750 m, 14% under
   and 7% over 1,013.25 hPa). A weather report's pressure is reduced to sea level, and at a 1,400 m
   field it lies about 18% over the standard's 856 hPa, so that mistake is caught there.
   Near sea level the two pressures differ less than 15% and pass, and so does the air flown.
5. **The log's site is its GNSS fixes before liftoff.** Where neither `--latitude` nor `--longitude`
   is given (both 0, the defaults), the site is the median of the log's last fixes before liftoff,
   or its first fix, as `hpr analyze --ghost` places its pad. That holds with or without a launch
   day, so a log with fixes now flies from its own site, where before it flew from 0° N, 0° E. A log with no fix keeps 0° N, 0° E, and `--launch-time` is then
   refused. The height is always `--elevation`'s: a receiver's height may be above the ellipsoid or
   above sea level, and the profile's check needs the site's height above sea level.
6. **The log's date is read only where the reader trusts it.** `--launch-time log` takes the time an
   AltosUI CSV's GPS columns (`year` to `second`, a four-digit year) give at the last locked fix at
   or before liftoff, or else the first after it. AltOS's telemetry keeps two digits of the year,
   and its flash's date "is not reliably the flight's" (the reader's own note), so neither is read;
   the Featherweight summary states a launch time but holds no track to compare. Every other log
   needs the time typed. The figure says which: "(the log's GPS time at liftoff)" or "(the time
   given)".
7. **The figure follows `hpr sim --plot`'s** ([ADR-157][adr-157], [ADR-175][adr-175]): two panels,
   altitude above the pad and vertical speed, over the time since each one's liftoff, SI on the
   left and US on the right, a caption in place of a legend. The log is measured, so ink, solid,
   2 px. Each simulation is predicted, so dashed in the predicted color: 1 px as designed, 2 px on
   the launch day, which the caption says. The product system allows dash or width to tell series
   apart; both simulations are predictions, so both keep its predicted color.
8. **Provenance on the figure.** A table under the panels gives each reading three ways: the log's
   value with its bound and 90% interval, where it came from and whether it was measured or derived
   (M7.2's provenance), then each simulation's. A simulated reading withheld where the log's is
   says "withheld too"; one withheld for a reason of its own gives the reason's first sentence (the
   text gives it whole). Under the table, the site and where it came from, each simulation's
   conditions with the weather source's grid point, time, cache state and credit, and the clocks.
9. **An axis holds the simulations and the log near them.** A garbled sample or a charge's pressure
   spike would stretch an axis until the flight is a line at its foot (Juno III's log holds a 7.8 km
   residual, ADR-314). So each axis holds every simulated value and each logged value within one
   span of the simulated values either side; a logged sample past that runs off the panel's edge,
   and the figure counts them and gives the furthest. A value under the ground by at most 1/360 of
   the panel's largest, a logger's noise on the pad, is taken as on it for the axis, so it doesn't
   add a whole step underneath. On the tallest panel, 360 px, its line's middle draws at most 1 px
   under the foot, where the panel's clip ends, so at most the lower half of the log's 2 px line is
   cut there.
10. **Its data go beside it,** as `hpr sim --plot`'s do: a CSV with each line's points and a sidecar
    naming the program and carrying the trust note. The figure carries no date of issue, but a
    weather answer from the cache names when it was fetched, so two runs a fetch apart differ in
    that line.

**How it is checked.** On a public flight, Prometheus 2022's AltosUI log from RocketPy's data, a
test runs the command a person would type with `--launch-time log`: the site comes from the log's
fixes (32.93950° N, 106.91145° W), the time from its GPS (2022-06-24T15:17:16Z), and Open-Meteo's
answer for them, recorded on 2026-11-25 (UTC), comes from the cache. It needs the gitignored checkout,
as M7.3a's real-flight test does. Tests that CI runs draw a constructed log against values entered
and against Open-Meteo's earlier recorded answer, and check the lines' styles, the provenance, the
conditions' words and every text's box on the figure. Others pin the refusals and the log's GPS
time.

**Consequences.** [Comparing a flight][page] explains it with the example. The text output gains a
line naming the conditions flown, and the JSON document gains `site`, `flown_in` and `plot`. Unsure:
how far back Open-Meteo's archive of past forecasts reaches is Open-Meteo's to say; a flight before
it needs a profile from another source through `--weather`. The weather source's air and wind are a
model's, hourly, on a grid kilometers wide, so the gap the figure shows between the log and the
launch day's flight isn't all the simulator's.
