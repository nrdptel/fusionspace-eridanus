# ADR-154: Motors fetched from ThrustCurve.org by name, cached, and named when offline (2026-10-04)

- **Status:** accepted; the file order (§2) and a `.ork` motor's fetch (§4) amended by ADR-161, which takes first the file holding OpenRocket's curve for the digest
- **Summary:** M4.5b: `hpr sim` fetches a motor the bundled catalog lacks from ThrustCurve.org, by name for `--motor` and by manufacturer and designation for a `.ork` motor with no curve, through `hpr_net`'s cache; `hpr motors fetch` pre-loads; offline and uncached, the refusal names the exact command. One rule picks the file; nothing is bundled

**Context.** [ADR-144](0144-the-2026-10-03-planning-review-leaner-bookkeeping.md) §8b asks that
`hpr sim` fetch a missing motor from ThrustCurve.org and cache it, that `hpr motors fetch <name>`
pre-load it, and that offline and uncached it print the exact command, with no bulk bundling of
ThrustCurve's data, which carries no data licence
([ADR-145](0145-recorded-answers-and-their-licences-stand-in.md)). `hpr_net::thrustcurve`
already searches and downloads through the cache with a day's freshness
([ADR-130](0130-m5-4b-thrustcurve-searches-and-curves-through.md)). Probed on 2026-10-04,
ThrustCurve's search matches `designation` exactly but for case (`F27R` finds nothing, `F27R/L`
finds AeroTech's), and `commonName` can match several makers (`J450` finds four). In the same
live probe (not committed), each of the 26 solid motors OpenRocket 24.12's example files name was
found, alone, by its manufacturer and designation as the file writes them.

**Decision.**

1. **Finding a motor** (`hpr_net::on_demand::find`): search the name as a designation, then, if no
   motor has it, as a common name, with the manufacturer when one is given. Exactly one solid
   motor must answer; several are refused with each listed by maker and designation; a hybrid is
   refused ([rule 6](../../CONTRIBUTING.md#6-scope-until-10)).
2. **Picking the file:** download the motor's RASP (`.eng`) files and take the first that makes a
   motor the caller flies (`on_demand::find_with`; `hpr sim` builds the motor from the file's
   masses), ranked by who measured it: a certification test, the manufacturer, a user,
   then none stated, in the answer's order within a rank; with none, the RockSim (`.rse`) files
   the same way. The rule is mechanical, not a judgement of which curve is right; the output
   names the file, its source and its licence.
3. **`hpr sim --motor NAME`:** the bundled catalog first, as before; a name it doesn't hold is
   fetched as in (1).
4. **A `.ork` motor with no curve** (no embedded curve, not in the bundled catalog) in the
   configuration flown, when `--motor` doesn't replace it: fetched by the file's manufacturer and
   designation, and supplied to the reader for the digest the file records
   (`ork::SuppliedCurves`). The reader's rule, a supplied curve only for its own digest, stands:
   the name lookup is the CLI's, and the flight's notes say so and that it may not be the curve
   OpenRocket flies. A motor with no digest can't be supplied and says so.
5. **Cache and offline:** every answer goes through `hpr_net`'s cache, so a motor found once
   flies with no network after. `--offline` on `hpr sim` and `hpr motors fetch`, or the
   environment variable `HPR_OFFLINE`, reads the cache alone. Offline with no copy, the refusal
   names the command that fetches it, `hpr motors fetch [--manufacturer M] NAME`, whose requests
   are the flight's own, so it fills the cache the flight reads.
6. **Tests and pages stay off the network:** the CLI tests run the binary with `HPR_OFFLINE` and
   an empty cache, or a cache filled from the replay fixtures; `cargo xtask cli` runs a page's
   examples through `hpr_cli::run_offline` over an empty cache. The fixtures added are stand-ins
   (ADR-145): invented records and curves, bar F27R/L's name, common name, class, type and id,
   and empty download answers.
8. **What reaches the terminal:** ThrustCurve's words pass through the CLI's `printable` (a
   control character shown as `?`), and the command a refusal prints is offered only when every
   word is one POSIX shells, PowerShell and `cmd` all take as it is; otherwise the refusal says to
   run `hpr motors fetch` with the motor's manufacturer and designation.
7. **Shown with the motor:** ThrustCurve's credit (`thrustcurve::ATTRIBUTION`) in `hpr sim`'s
   notes and `hpr motors fetch`'s output, and each file's licence as ThrustCurve states it. A
   fetched file stays in the user's cache; none is committed or bundled.

**Consequences.** On 2026-10-04, with the network, five of OpenRocket's 17 example files, which
failed for a missing curve before, fly as saved: *A simple model rocket*, *Airstart timing*, *Clustered
motors*, *Dual parachute deployment* and *Tube fin rocket*; the rest stop on design checks,
separations, parallel stages, freeform fins or a hybrid, M4.5's later increments. That count is a
live observation, not a committed survey: the corpus count M4.5 publishes needs one. A fetched
curve is compared with OpenRocket's own curve for the same motor nowhere yet, so a flight on it is
held to ThrustCurve's file alone. The Python binding and `.hpr` designs fetch nothing yet.
