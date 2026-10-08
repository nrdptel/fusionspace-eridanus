# ADR-083: M2.3c blocked: no private design is the rocket of a logged flight (2026-09-26)

- **Status:** accepted; the block lifted by ADR-151 (2026-10-04), whose private flight collection pairs designs with their flights' logs
- **Summary:** M2.3c blocked: no private design is the rocket of a logged flight

**Context.** M2.3c asks for the private designs that have a flight log, flown in their day's
weather and published as anonymised statistics beside M2.3b's report. The private data is in two
collections, both read only from `refs/` (rule 4): the design library (`loft-fixtures`, the corpus
of ADR-071 and ADR-072) and the flight logs (`debrief-fixtures`). Neither manifest links one to the
other: the logs have a flight group and no design column, and the library's groups only tie together
copies of one design saved in different tools. So on 2026-09-26 each log was matched against each design by
hand, on the rocket's name, its motor, the flight's date and the source each file came from, in
both manifests and in the files themselves.

**Findings.**

1. **No private design is the rocket of a logged flight.** No log in `debrief-fixtures` matches a
   design in the library. The only designs there whose rockets have logs at all are copies of
   RocketPy's examples, which are public; their logs are RocketPy's (`refs/rocketpy`), and they
   were already flown in M2.3b (ADR-082 item 2) or left out there for the reason given (an apogee
   and no climb).
2. **One design shares a source with some logs, and nothing more.** Its team published both, but
   the design was saved before any of those flights, names none of the logged rockets, and holds
   several motor configurations, and none of those logs names its motor. Choosing a pairing
   and a configuration would be a guess, and an apogee error against a guessed pair measures
   nothing. It is not flown.
3. **One design has a flown apogee stated by its source, but no log,** and no date or site, and
   hpr can't fly it yet ([#174](https://github.com/nrdptel/hpr-sim/issues/174)).
4. **No design carries a recorded flight.** Every data point in the library's files belongs to a
   saved simulation. One file among the logs is a simulation too, of a public example, not a flight.
5. **Weather.** No ERA5 file under `refs/` covers the date and site of the flight in item 2. The
   Copernicus Climate Data Store, ERA5's source, needs an account to download one.

**Decision.**

1. **M2.3c is `[blocked]` on data only Neer can add** (the maintainer's to supply): a pair, that
   is a design file of a logged rocket as flown, or a log of a library design's flight, with its
   date, site and motor, in the private collections. A day not covered by a cached file also needs
   an ERA5 file, and that needs a Data Store account. Its *done when* is unchanged.
2. **Nothing is flown against a guessed pair.** The survey's result is recorded here, on the
   roadmap and on the accuracy page, without naming a private design.
3. **M2.3's own bullets are met by M2.3b's seven flights.** There are at least six; the mean
   absolute apogee error, 6.04%, is reported against the 5% target and is outside it; each outlier
   has an explanation the report checks. The milestone stays open only for M2.3c.
4. **Work moves on to M2.4,** the accuracy census gate, which does not depend on M2.3c.

**Consequences.** The real-flight evidence stays at M2.3b's seven public flights; no private design
adds to it. When a pair is added, M2.3c reuses M2.3b's reading of a log and of a barometer
(`hpr_validate::real_flight`) and M2.2e3's anonymised ids (`cargo xtask ork-flights --library`).
It also needs a way to fly a `.ork` design from the library, where M2.3b flies committed designs.
