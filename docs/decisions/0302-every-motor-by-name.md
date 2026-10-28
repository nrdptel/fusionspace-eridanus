# ADR-302: Every motor by name, offline (2026-10-28)

- **Status:** accepted; carries out [ADR-301][adr-301] §2's switch for [M5.7b2][m5-7], and [ADR-288][adr-288] §3's digest-first default
- **Summary:** Every lookup now reads the whole bundled catalog: 1,004 motors, 924 of them with a curve file. A name two makers share takes its maker (`Estes C6`). A designation outranks a common name. Each motor gets a selector, a name that finds it alone. A motor with no curve is listed and refused, not hidden. A bundled default file more than 5% light on propellant warns, as a fetched one does. `cargo xtask motor-catalog --check` lists and flies every motor offline, with counts from the raw snapshot.

[m5-7]: ../decisions-and-roadmap.md#m5-7
[adr-005]: 0005-solid-motors-statistics-consumption-grains-file.md
[adr-288]: 0288-every-solid-motor-ships-in-the-package.md
[adr-298]: 0298-every-solid-motor-in-four-increments.md
[adr-301]: 0301-the-whole-catalog-in-two-increments.md
[i578]: https://github.com/nrdptel/fusionspace-eridanus/issues/578
[i1003]: https://github.com/nrdptel/fusionspace-eridanus/issues/1003
[survey]: ../research/fetched-mass-survey.md

**Context.** M5.7b2 is done when "offline, `hpr motors list` lists each motor and `hpr sim
--motor` flies each with a curve, the counts taken from the raw snapshot; M5.7a's default, listing
and warning hold on every file it adds, failing files warned of." M5.7b1 compiled in ThrustCurve.org's
whole solid catalog from the 2026-10-28 snapshot, while the lookups kept M5.7a's 32 motors.
Pointing them at all 1,004 motors raises questions the 32 never did:

- 32 designations are shared by two or more makers, and 166 names, once case, spaces and hyphens
  are dropped, find more than one motor. A bare `C6` found one motor among the 32; it finds Estes'
  and Quest's now.
- 80 motors have no curve file that reads. The 32 all had one.
- 48 motors' default files give a propellant mass more than 5% below their record's
  ([issue #1003][i1003]). A light propellant mass with a right loaded mass flies too heavy a
  burnout mass and too low an apogee: the flattering side for a waiver.
- A `.ork` motor's file names its OpenRocket curve by digest; with several files a motor, the
  digest says which one the file was saved with.

**Decision.**

1. **Names.** `Catalog::find` reads a leading maker name, full (`Cesaroni Technology`) or short
   (`Cesaroni`, `Estes`), followed by a space, and keeps only that maker's motors. The rest is
   matched as before, ignoring case, spaces and hyphens, against designations and common names.
   A motor whose designation matches outranks one whose common name does, so `I175WS` is
   AeroTech's single-use motor, not every I175. A `.ork` motor is looked up by its maker and
   designation (`Catalog::find_by_maker`).
2. **Selectors.** Each motor's selector is its designation where that finds it alone, else its
   maker's short name and designation (`Estes C6`). `hpr motors list` prints it. Every message that
   lists candidates for a name that finds several lists their selectors, so each one it suggests
   works when pasted back.
3. **Motors with no curve.** `hpr motors list` lists all 1,004, marks each with or without a
   curve, and ends with a note naming the 80 without one. `hpr motors show` on such a motor
   refuses with that reason, and skips it with a warning in a listing of several. `hpr sim --motor`
   refuses it. A motor's figures can't be worked out without a curve, so none are shown.
4. **The default file.** A `.ork` motor whose curve OpenRocket names by digest flies the bundled
   file with that digest, if it is one of that motor's files. Otherwise it flies ADR-298 §5's
   default: a file that passes [ADR-005][adr-005]'s 1% check first. The table from OpenRocket's
   digests to ThrustCurve.org's files moves from `hpr-cli` to `hpr-motor`
   (`hpr_motor::openrocket_curves`), so the `.ork` reader can use it.
5. **Warnings.** A flight warns for each file that fails the 1% check, as M5.7a's 32 did. It also
   warns when the file's propellant mass is more than 5% below its record's: the rule a fetched
   file has had since [issue #578][i578], with the same bound (`PROPELLANT_SHORT_BOUND`, now
   `hpr_motor`'s, which `hpr_net` reuses). A heavy file isn't warned of, for the fetched rule's
   reason: about half of those give the loaded mass less a measured burnout mass, which flies the
   right burnout mass ([fetched-mass survey][survey], §5). A warning for a motor that `--motor`
   replaces, or for a configuration not flown, is dropped.
6. **Every motor listed and flown.** `cargo xtask motor-catalog --check`, which the gate's `refs`
   step runs, runs the command line in-process and offline. `hpr motors list --json` must list
   every solid motor of the snapshot, each by a selector that finds it alone. Then each motor with
   a curve flies in a probe rocket built for its case size, with `hpr sim --motor <selector>`. The
   probe's bore is the case plus 0.2 mm, its airframe a fifth wider, with three fins. A flight
   counts when it leaves the rail and reaches an apogee on that motor's bundled curve, and when it
   warns of a failing default file, and only then. The expected motors come from the raw snapshot
   under `refs/` (every solid motor; every one with a RASP or RockSim file no named refusal leaves
   out), or from the committed records and file list without it. On this snapshot: 1,004 listed,
   924 flown, in 24 probe rockets.
7. **A left-out `.ork` configuration's reason** is one warning: the first part left out, else the
   first value left out, else the first. It names no count of the others. The count differed
   between a `.ork` and the copy `hpr convert` writes of it, which states values the first left to
   be assumed. Every warning is still listed beside the reason.

**Consequences.**

- Reports move only where a motor that had no curve now has one. In
  `validation/reports/ork-cli-flights.md`, 25 configurations fly with `--accept-design-errors`
  where 9 did, and refusals fall from 25 to 9; the 136 that fly as saved are unchanged. In
  `openrocket-flights.md`, one design not flown gives a new first reason, as the motor it lacked
  is now found; in `fixture-flights.md`, one flight not flown moves to its next cause the same way.
- A script that relied on a bare shared designation finding one motor now gets a list of
  selectors to choose from.
- The 80 motors with no file that reads stay unflyable until ThrustCurve.org has one for them.
- M5.7c's oracle tests and the converter's round trip still cover M5.7a's 32 files, as ADR-301 §6
  set.
