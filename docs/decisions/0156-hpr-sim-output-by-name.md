# ADR-156: `hpr sim` reads by name: configurations by their motors, a lead summary (2026-10-04)

- **Status:** accepted
- **Summary:** M4.5d: `hpr sim`'s text names configurations, mounts and the design checks' parts by name, not by UUID; an unnamed configuration is called by its motors and delays in brackets, such as `[C6-5]`; `--config` takes a configuration's number, name or id, and `--mount` a mount's name or id; the checks print as sentences; the summary leads with margin, apogee, rail exit speed, delay and descent. `--json` keeps the ids

**Context.** [ADR-144](0144-the-2026-10-03-planning-review-leaner-bookkeeping.md) §8d asks for
"configuration names, not UUIDs; sentences, not JSON (JSON with `--json`); the summary leads with
margin, apogee, rail exit speed, delay and descent rate". Before this decision, `hpr sim` printed
each configuration and motor mount by the id the file gives it, a UUID in an OpenRocket file, and
each design-check finding as its JSON. OpenRocket files seldom name a configuration: of the 56
configurations in OpenRocket 24.12's 17 example files, 6 have a name.

**Decision.**

1. **A configuration's name** is the file's, or where it is blank, its motors in brackets,
   separated by `; `, each as a flyer writes it: the designation and the delay, such as `[C6-5]`,
   `[H999N-P]` for a plugged motor, or `[D12-0]` for a `0` that may mean either. Two
   configurations of *Two stage high power rocket* store names in this form,
   `[H148R-0; H148R-0]` and `[I59WN-P; I357T-14]`, which suggests it is OpenRocket's own. A designation that already carries a dash,
   such as `168H54-10A`, is left as it is. The delay is part of it because a file's
   configurations often differ by delay alone: *A simple model rocket* has `[C6-3]`, `[C6-5]` and
   `[C6-7]`.
2. **Configurations are numbered** in the file's order, and the output lists them that way, so two
   of one name can still be told apart. `--config` reads a word as an id (exact, else its case
   ignored), a number, and a name (exact, else its case and brackets ignored), and every reading
   that matches must mean one configuration: a name several share is refused with their numbers,
   and a word that is one's number and another's id or name, such as `2` where a program numbered
   its configurations' ids from 0, with their names and ids.
3. **Parts by name.** Motor mounts, recovery devices and every finding's parts are named in
   backticks; a part with no name is called by its id. `--mount` takes a mount's name or id, and
   a name two mounts share is refused with their ids. A design error names the part at fault by
   name, at every depth.
4. **Findings as sentences.** `hpr_design::checks::Finding::describe` gives each finding as a
   sentence, sizes in millimetres, with the parts named by a function the caller passes, so the
   library has the words and the CLI the names. Repeated identical warnings, such as a coupler in
   each of two tubes of one name, print once with a count.
5. **The lead summary,** after the configuration: the static margin off the rail and its least
   before apogee; the apogee; the rail exit speed; the delay, as the coast from burnout to apogee,
   the motor's set delay, and for one motor lit at launch, how far before or after apogee its
   charge fires; and the descent, the vertical speed under each set of open devices as the next
   opens and at landing. With no device open, the descent says the fall is not a prediction and
   gives no speed, as the landing's caveat already says
   ([ADR-153](0153-a-orks-recovery-flown-as-openrocket-flies-it.md)).
6. **`--json` keeps the ids** and gains `label` for each configuration, `configuration_label`,
   `not_flown` (why a configuration doesn't fly as read, in a few words), each motor's `delay`, and
   each event's `vertical_velocity_m_s`.

**Consequences.** Over the 17 examples, the text holds no UUID and no JSON in 56 runs: each file
as saved, then by `--config` number every configuration of the files that fly and the first of
the rest. Over 701 `.ork` files, the private corpus and the repository's own fixtures, run
offline, it holds for every file: 45 fly, so their summaries and check sentences were seen, and
656 are refused, so their refusals and configuration lists were.
A configuration whose motor this run didn't fetch says so (`its motor's curve to fetch`) rather
than that it doesn't fly. The bracket form isn't checked against OpenRocket's own window. A
configuration name with OpenRocket's placeholders, such as `{motors}`, prints as typed
([#315](https://github.com/nrdptel/hpr-sim/issues/315)). Two compared sizes in a finding print to as many decimals as tell them apart, so a
29 mm motor in a 28.956 mm bore reads `29.00 mm` in a `28.96 mm`.
