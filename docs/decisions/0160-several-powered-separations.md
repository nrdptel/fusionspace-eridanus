# ADR-160: Several powered separations, each handing the flight on to a sustainer cut at its boundary (2026-10-04)

- **Status:** accepted
- **Summary:** M4.5g2: the flight takes a list of separations, in the order they fire, each at a stage boundary further forward than the one before (`Simulation::with_separations`). With more than one, each must leave the nose's body a motor to burn: at each, the stages behind drop away as the next body and the flight flies on as a sustainer cut from the whole design at that boundary. The `.ork` reader returns every powered separation, from the tail forward. OpenRocket's *Three stage low power rocket* flies in `hpr sim` and in `cargo xtask ork-flights`. In the report, on OpenRocket's own curves, its three configurations are within 1.48% of OpenRocket's apogee and 1.64% of its largest speed, both separations at OpenRocket's times; in `hpr sim`, on the curves it fetches (ADR-161), within 2.48% and 1.67%

**Context.** [ADR-074](0074-ignition-times-and-powered-staging-the-sustainer.md) flies one
separation, and [ADR-076](0076-a-ork-files-ignitions-and-one-powered-separation.md) reads one
from a `.ork` file; a configuration with more was left out with *stages hpr can't separate as
written* (#183). M4.5g2 asks that each configuration of OpenRocket's *Three stage low power
rocket* fly in `hpr sim` within ADR-076's 5% of OpenRocket's apogee and largest speed. The example
drops its booster at its burnout as the middle stage lights, and the middle stage at its own
burnout as the sustainer lights.

**Decision.**

1. **A list, in firing order.** `Simulation::with_separations` takes the separations in the
   order they fire. Each boundary is forward of the one before, so separation `k` makes body
   `k + 1`: the stages from its boundary back to the one before it, or the tail. Body 0 keeps the
   nose. `Separation::body_of` and `Separation::stages_of_body` give the numbering.
   `with_separation` is the list of one, and every flight with one separation is the same to the
   bit (the regenerated OpenRocket report moves no number of any other flight).
2. **Each split hands the flight on.** At separation `k` the part behind drops and flies on its
   own to its landing, as a point under its own devices (ADR-014). The flight carries on as a
   sustainer, cut again from the whole design at the new boundary (`Sustainer::of`), from the
   nose tip's state. The motor indices of the whole stack still hold, and so do the cut layouts'
   nose-tip origins. A motor lit by an earlier separation counts from that one at the next. That
   fixes a defect a second split would have had: rebuilding the ignitions from the current
   separation alone dropped a motor lit by the first, so it would have flown as unlit.
3. **What is refused.**
   - The builder refuses:
     - boundaries that don't move forward;
     - times known before the flight that come in the other order;
     - more than one separation together with ejections, in either builder order
       (`SimError::Unsupported`): a sustainer's cut design doesn't track the ejections' pieces.
   - In flight, these are refused by name (`SimError::Domain`):
     - a separation that leaves nothing ahead of it to burn while there is more than one, since
       once every body is a point mass there is no sustainer left to part;
     - a separation that fires before the one ahead of it in the list.
   - Each dropped part must have a device open at its split, as the booster must (ADR-159).
4. **The `.ork` reader.** It returns every powered separation, from the tail forward. Each is
   checked as ADR-076's one was. Two more are left out by name: a separation at or after apogee
   beside another, and one that comes before the separation behind it.
   - `MotorConfiguration::staging` stays the first separation. The design format's version 0.2
     gains an optional `later_stagings` for the rest, so every document already written reads
     the same.
   - `MotorConfiguration::stagings`, `hpr::ork::separations` and
     `FlightBuilder::separations` carry the list.
   - `hpr::ork::separated_recovery` and `tumbling` take a slice. Each dropped part tumbles from
     its own split until its first own device opens.
5. **The command.** `hpr sim` flies every separation the file has. Its note lists each part that
   drops and when. Each part's landing is printed, marked rough as ADR-159's booster is.
   `cargo xtask ork-flights` reports each separation's time for both tools, a list in place of
   ADR-076's single entry.

**Result.** `cargo xtask ork-flights`, on OpenRocket's own curves by digest (ADR-067). The apogee
of a flight whose parachute opened before apogee is compared with OpenRocket's flight with nothing
deployed, as ADR-076 does.

| motors | apogee | largest speed | separations, OpenRocket / hpr (s) | descent targets |
|---|---:|---:|---|---|
| A8-5, B6-0, B6-0 | −1.37% (held) | +0.25% | 0.857 / 0.857; 1.714 / 1.714 | met |
| C6-5, B6-0, B6-0 | −0.95% (held) | +1.51% | 0.857 / 0.857; 1.714 / 1.714 | met |
| C6-7, C6-0, C6-0 | −1.48% | +1.64% | 1.860 / 1.860; 3.720 / 3.720 | met |

The sustainer's landing speed is within 0.009% of OpenRocket's on each, and its flight time
−1.13%, −0.56% and −1.41%. The report flies 37 configurations, up from 34. Of the 33 in-scope
public descents, 25 meet every target as written (was 22 of 30). The private library's report
changes only in its count of public designs toward M2.2's 20: 10, up from 9, so 21 in all.

**What `hpr sim` reads, with the curves it fetches.** With no curve in the file, `hpr sim` fetches
ThrustCurve.org's (ADR-154).

- Its B6 and C6 were OpenRocket's from the start. On them the second and third configurations
  read 121.5 m/s and 132.0 m/s, the report's speeds to 0.01 m/s.
- Its A8 was at first another file: it burns out 0.536 s after lighting, where OpenRocket's A8
  burns 0.73 s. So the first configuration's largest speed read 82.8 m/s, 5.7% above
  OpenRocket's 78.4, a miss of the *done when* in `hpr sim`. Flown at OpenRocket's latitude and
  1 m rod, it read the same 82.8 m/s, so the site and rail don't account for it.
- [ADR-161](0161-openrocket-curves-by-digest.md) fixes the pick: `hpr sim` takes the ThrustCurve
  file holding OpenRocket's curve for the motor's digest. With it the three configurations read
  largest speeds +0.31%, +1.55% and +1.67% against OpenRocket's, and apogees −1.15%, −0.58% and
  −1.37% against OpenRocket's own record, which opens the parachutes as `hpr sim` does (−1.28%
  and −2.48% on the first two against its flight with nothing deployed).

**The rod-clearance mass.** The census flags each new flight's mass at rod clearance as over its
bar, +1.80% to +3.26%. OpenRocket's recorded mass falls as if every motor like the booster's
burned from launch (#185). Its drop from launch to rod clearance is 2.0000, 1.9998 and 2.9944
times hpr's: the count of motors of the booster's type in each configuration. On the two-stage
example's two H148Rs the ratio is 1.9992. The launch masses agree within 0.054%, and the rows are
accepted with that reason.

**Consequences.**

- OpenRocket's three-stage example flies as saved in `hpr sim`, offline once its curves are
  fetched, and the report's test of staged designs holds all three flights to 5%.
- A dropped middle stage's own flight, like the booster's, is compared with nothing (#179).
- An unpowered separation after a powered one is refused. So is an unpowered one before apogee
  in a single-separation `.ork` file, which stays M4.5g3's (#184).
- A motor lit by a separation can't time a later separation, since its burnout has no time before
  the flight. The `.ork` reader never lights a motor that way.
