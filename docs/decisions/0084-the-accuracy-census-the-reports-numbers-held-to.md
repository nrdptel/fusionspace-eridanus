# ADR-084: The accuracy census: the reports' numbers held to the ones accepted (2026-09-26)

- **Status:** accepted
- **Summary:** The accuracy census: the reports' numbers held to the ones accepted

**Context.** M2.4 asks for a census of the validation results, in the README with a badge, and for
CI to fail on any per-case regression beyond tolerance; *done when* a perturbed drag coefficient on a
throwaway draft PR turns CI red. CI already failed when the committed report was stale
(`validate --check`, ADR-022), and a same-drag miss or a flipped predicted-mode verdict failed the
run or a pinned test. What nothing caught was a report regenerated with a worse number that kept
its verdict: a predicted-mode apogee going from 1% to 2.9% of RocketPy's, inside its 3% target
(ADR-023), or an OpenRocket or real-flight difference, which have no gate at all. Loft's lessons
L84 (hand-written counts), L85 (a "now passes" check at half the tolerance), L86 (a headline with
no oracle, population or regime) and L88 (its own aerodynamics never gated) apply.

**Decision.**

1. **One row per number a report holds hpr to** (`hpr_validate::census`), from the four committed
   reports: every comparison and known gap of the harness's (`latest.json`); each real flight's
   apogee error and climb RMS; and each OpenRocket flight's apogee, largest speed, margin, mass at
   launch and at rod clearance and centre of mass there, with each configuration the report lists
   as not flown (L86's population: 24 of the examples' and 19 of the private designs'). Left out:
   figures a report gives to explain a difference, not to measure one (a real flight on its team's
   drag, its satellite heights). A row is keyed by its report's group, its case and its metric; a
   key seen twice is refused, and every count is the census's own (L84). Six groups, each naming its
   reference and version, its kind (code-to-code on the same inputs, code-to-code on each code's own
   model, or measured), and what its report holds it to (a gate, a target, or neither).
2. **Speed classes** by the largest Mach number: subsonic below 0.8, transonic to 1.2, supersonic
   above, the OpenRocket library report's classes, and "unknown" where a report has none. The
   harness's flights by RocketPy's number (its `max_mach` reference), OpenRocket's by
   OpenRocket's, and the logged flights by hpr's, which the real-flight report now records
   (`hpr_max_mach`), since a log has none. Each headline names reference, kind, population, the
   not-flown count and the classes (L86).
3. **A ratchet, both ways.** The census accepted last is committed (`census.json`). It is a change
   when a row's difference moves by more than its accepted slack, when its standing changes (a miss
   that starts passing too, at any margin: L85), when its unit, scale, slack or class changes, when
   it comes or goes (a gap or not-flown case that is flown now is named so), when a group's
   reference changes, or when the slack rule does. An improvement is a change too, or it could slip
   back unseen. The slack is 0.1% of the row's scale, and for a harness row never less than the
   harness's reproduction bound (2e-6 or 1e-7 of the value). The scale is the row's tolerance, or
   where it has none its kind's bar: 3% of the reference for a harness metric not scored (M2.1's
   bound on each metric, which `no_committed_gate_is_looser_than_the_milestone_says` holds every
   gate and target to); 5% for OpenRocket's apogee (ADR-070's line for a written cause) and largest
   speed (M1.9c's bar on both); 1% for its masses (M2.2a's); 0.5 calibres for a margin or centre of
   mass (M1.8a's centre-of-pressure target; either moves the margin as much); 5% for a logged apogee
   (the real-flight target, which is on the mean, used per flight as a bar); 3% for a logged climb
   (ADR-024's bound on a series RMS). The committed reports are regenerated on one machine and
   compared as files, so the noise the slack must clear is a regeneration's, not a platform's.
4. **Checked in `validate --check`,** whether or not the run reproduces the report, so CI's
   `validate` job holds it on three platforms and the gate keeps its nine steps. A change passes
   only once accepted: `cargo xtask census --accept --reason "<why>"` refuses a changed census
   without a reason, and an unchanged one with one, and writes the reason and the list of changes
   into `census.json` and its page. A regression can merge, in writing, in the diff that brings it;
   not by regenerating a report.
5. **Its outputs are written from the accepted rows, its summaries computed again each time** and
   checked as text: `census.md`, the table between markers in `README.md` and `docs/accuracy.md`
   (the same table, linking the census page at its GitHub address), and two static badges in
   `docs/images/` (no network): the gated code-to-code metrics that pass ("vs RocketPy, same
   inputs") and, beside it, the real flights' mean apogee error against its target, so the one
   measurement is as visible as the agreement with another program.

**Consequences.** The first census holds 648 rows. Predicted mode's rows, which failed a run before
only when a verdict flipped, and the OpenRocket and real-flight differences, which no run could
fail, now fail CI when they move. On the
throwaway PR, 2% more subsonic skin friction with the report regenerated failed the check on 74
rows, 17 for the worse. Two limits stay. The census holds each number to where it was, not to the
truth, and adds no evidence of its own. The OpenRocket and real-flight reports need files CI lacks,
so CI holds their committed numbers, not a fresh run: a code change that would move them is caught
only when someone runs them again and commits the result. Every physics change that moves any
number now needs `census --accept` with a reason, one more command than before, which is the point.
