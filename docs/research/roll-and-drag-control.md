# Drag-only brakes and roll-only surfaces: the rules, the record and the aerodynamics

**Bottom line:** hpr limits active control to airbrakes that only add drag, and to canards and
fin tabs that only roll the rocket, with the rocket stable whenever the surfaces are inert. Every
competition rule found allows this class (some allow more), and NASA itself set
US students a roll-control task in 2016. No regulation or ruling yet separates this narrower class
from the Munitions List's "active controls", though. So the simulator side goes ahead in public,
and the hardware stays in [ADR-192][adr-192]'s held tier until a ruling. Of the two roll devices,
tail-fin tabs come first. In NASA's tests at Mach 1.6 to 3.5, a tail spanning as much as the
canards removed about 70% to 130% of their roll; no subsonic data was found.
[ADR-193][adr-193] records the decision. Researched October 7, 2026, from the regulations, the 2024 rule's public docket,
competition rules and NASA and NACA reports. **Not legal advice.** Figure values marked "read by
eye" are approximate (about ±0.01) and must be digitized before any test uses them.

On October 7, 2026, Neer decided on airbrakes designed strictly to add drag, with no active pitch
or yaw control; for canards that give roll control only, with no pitch or yaw control; and for
the servo-driven tabs some rockets carry on their fins for roll control to be considered.

## The design rule

| device | what it may do | what it may not do |
|---|---|---|
| airbrakes | add drag, all flaps moving together | make a pitch or yaw moment, by command or by design |
| roll canards | roll the rocket, opposite surfaces deflecting equally and oppositely | pitch, yaw or steer |
| roll tabs (hinged trailing-edge tabs on the tail fins) | roll the rocket, the same way | pitch, yaw or steer |

Three conditions hold for all three devices:

- **Stable when inert.** The rocket is stable with every surface retracted or neutral (IREC
  §7.2.1). It also has to stay stable with any one surface stuck anywhere in its travel.
- **Built so it can't steer.** The controller has only two commands, brake deployment and roll.
  A fixed mixer turns the roll command into equal and opposite deflections. Without a pitch or yaw
  channel, a stuck or broken surface is the only way to make a side force.
- **Quiet through boost.** The surfaces stay neutral until burnout, and go neutral again on an
  abort, a power loss, or a tilt past 30° (IREC §7.3 and §7.4).

## What the rules say about this class

| source | what it says |
|---|---|
| USML Cat. IV(a), Note 3 (22 CFR 121.1) | hobby rockets "must not contain active controls (e.g., RF, GPS)"; no exception for drag or roll |
| 2024 proposal (89 FR 84482) | "(5) Has no active controls", with the examples dropped; still not final |
| IV(h)(1) and its note | "flight control and guidance systems … specially designed"; a guidance set corrects "the trajectory" |
| EAR 7A116, 7A103; MTCR 10.A | flight and attitude control (7A116, MTCR 10.A) only for systems that carry 500 kg to 300 km; flight instruments with autopilots (7A103.b, .c) for a 300 km range at any payload; inertial units (7A103.a) by sensor grade, with no range threshold |
| DDTC's ruling list (2026-08-01) | nothing on hobby airbrakes or roll control. Closest: RockSim EAR99 (2020); a Base 11 avionics thesis "Not subject to ITAR or EAR" (2020); a military rocket's control actuator and a guided bomb's canards both under the EAR's 600 series |
| Tripoli Unified Safety Code v2 (2026) | no rule on guidance; thrust to weight below 3:1 "only … if an active stability system is included" |
| NAR high-power code | "I will not launch my rocket at targets" |
| FAA, 14 CFR 101.29(b)(4) | a Class 3 waiver application describes the rocket's "flight control" |
| IREC DTEG 2026 §7 | control "strictly for pitch and/or roll stability augmentation, or for aerodynamic braking"; never guided "towards a designated spatial target or GPS location"; stable when "inert and mechanically neutral" |
| EuRoC 2026, Appendix F1 | a system that can only roll may spin the rocket from the rail; stopping the spin waits for 1,500 m |
| Launch Canada 2026 | "strictly for roll stability augmentation or … aerodynamic 'braking'"; a new roll or airbrake system first flies on 5 kN·s or less |
| NASA Student Launch 2016-17, Option 2 | "Teams shall design a system capable of controlling launch vehicle roll post motor burnout": two turns induced, then stopped |
| Wisconsin Space Grant, 2023 | a required system that "actively minimizes the roll-rate" in coast, its surfaces at most 15% of the fin area |

**The 2024 docket asks for this line, and State hasn't answered.** The proposal's docket
(DOS-2024-0035) returned 660 comments for the phrase "targeting functions" on October 7, 2026.
That phrase comes from an IREC student form letter, which asks State to "clarify that simple
altitude control mechanisms, such as airbrakes, are not targeting functions".
ESRA, which runs IREC, wrote that it "has interpreted this clause to catch targeting functions, but
not altitude control schemes or stability augmentation systems". It describes airbrakes as
"symmetrical, drag-causing aerodynamic surfaces".
A NAR trustee, writing for himself, asked State to exclude airbrakes, which "do not have the ability to
control the future position of the rocket". No final rule or response to comments has been
published.

**How others go about it.**

- Portland State's rocket society, a US group, flew and published canard roll control. It
  says that nothing it works on "falls under any really strict regulatory compliance (like
  ITAR)".
- The READMEs of 35 student airbrake and roll-control repositories, US and foreign, carry no
  export statement (11 more had no README).
- A Portuguese team flew its own airbrake to a 3,000 m target.
- BPS.space restricts sales of its thrust-vectoring computer to US persons. It dropped its fin
  outputs because "commercial active canards" were more complex than thrust vectoring.
- No commercial airbrake or roll kit was found.
- **No enforcement case** involving hobby roll or drag control was found. The search covered
  DOJ's case lists of 2015, 2016 and 2019, BIS's 2024 case book, five DDTC consent agreements and
  the first page of DDTC's penalty list (2017 on). DOJ lists after 2019 and the BIS database were not searched.

**What this changes.**

- *For a distinction:*
  - Note 3's own examples are RF and GPS.
  - The guidance-set note is about correcting a trajectory.
  - Every Commerce and MTCR flight-control entry is tied to a 300 km range.
  - Every competition, and NASA, treats this class as allowed.
- *Against one:*
  - The text has no exception, and the proposal drops the examples.
  - IV(h)(1) has no range threshold.
  - The comments asked for the narrow reading precisely because the text doesn't give it.
  - A competition's rule is not a ruling.
- *For hpr:* the class doesn't move hardware out of the held tier. It does turn the commodity
  jurisdiction request into one well-described item: drag-only and roll-only by construction,
  stable when inert.

## The aerodynamics

**A roll tab acts like fin cant, scaled by the tab's effectiveness.** hpr already computes roll
from fin cant by Barrowman's strip theory ([ADR-031][adr-031]; the OpenRocket technical
documentation's eq. 3.66 has the same form). Deflecting a trailing-edge tab by δ turns the fin's
section lift by about τ·δ, where τ is the flap effectiveness. Glauert's thin-airfoil result is
τ = 1 − (θ − sin θ)/π with cos θ = 2c_f/c − 1, for a tab of chord c_f on a fin of chord c.
That gives τ = 0.40, 0.55, 0.61 and 0.66 at c_f/c = 0.10, 0.20, 0.25 and 0.30. These values match
NACA TN-2486's subsonic curve and Hoerner and Borst's approximation (4/π)√(c_f/c). The original
paper, R&M 1095, was not fetched. *Worked example:* a tab of a quarter chord, deflected 10°,
turns the fin like a cant of about 0.61 × 10° = 6.1° across the span it covers.

The estimate is optimistic, and it has corrections:

- **Real tabs do worse than theory.** Hoerner and Borst say measured values "are always less
  than indicated by theory", from hinge gaps and thick trailing edges.
- **A part-span tab** turns only the strips it covers. Their share of the fin's roll moment is
  what counts.
- **Transonic.** On a rocket-launched NACA test model, 5° ailerons lost effectiveness at Mach
  0.90 and "reversed between Mach numbers 0.94 and 1.0" (RM L8E25). 10° ones did not.
- **Supersonic.** Linear theory gives τ = c_f/c, so a 20% tab gives 0.20 against 0.55 subsonic
  (TN-2486). Four tail-fin ailerons on 11% of the fin area, at 10°, gave a measured roll
  coefficient of about 0.45, 0.30, 0.19 and 0.10 at Mach 1.6, 2.16, 2.86 and 4.63 (TP-1353
  Fig. 18, read by eye). That falls roughly as 1/β. A thick fin does worse: a 10%-thick wedge
  gives about 0.11 for a 20% tab, not 0.20 (TN-2486 Fig. 4(b), read by eye).

**A canard's roll is undone by the tail behind it.** Differentially deflected canards shed
vortices that hit the tail fins, which "induce rolling moments of opposite direction" (NASA
TP-2157). Its Figure 13, read by eye, gives the canards' roll effectiveness per degree against
the ratio of exposed tail span to canard span:

| Mach | canards alone | ratio 0.47 | 0.75 | 1.07 | 1.25 |
|---|---|---|---|---|---|
| 1.75 | 0.16 | 0.135 | 0.075 | 0.045 | 0.04 |
| 2.50 | 0.12 | 0.11 | 0.05 | about 0 | −0.02 |

The report finds canard roll control feasible only at a ratio of 0.75 or less. An earlier NASA
test from Mach 0.2 to 4.63 concluded "the canards are not suitable devices for roll control"
(TM X-3070). A free-spinning tail removes the reversal (TP-1316), but bearing friction brings
part of it back (TP-2401). These data are supersonic; no subsonic measurement of the cancellation
was found. Forward canards also move the center of pressure forward even when undeflected, which
cuts the static margin. Student Launch banned forward canards in its 2015-16, 2016-17 and
2019-20 handbooks; its 2021-22, 2022-23, 2026 and 2027 handbooks don't.

**Airbrakes.** A flat plate square to the flow has a drag coefficient of about 1.17. Flaps on a
fuselage measured 0.9 to 1.0, nearly flat up to Mach 0.96, the highest tested (Hoerner, *Fluid-Dynamic Drag*, from
NACA RM L8B06). Solid flaps can make the tail buffet; perforated flaps avoid it. RocketPy models
airbrakes as a drag coefficient against deployment and Mach, added along the axis, with no lift
and no failure modes (MIT, read at commit 9bd6ad3). Its Camões example carries a real flight's
altimeter log and airbrake history from EuRoC 2023, but the flight data's own license is unclear.

**Failure modes the checks must cover:**

- **A stuck surface.** One canard or tab stuck hard over, or one brake flap left out, makes a
  side force and a moment. The rocket trims at an angle of attack of about that moment over its
  pitch stiffness.
- **Roll lock-in.** If the roll rate matches the pitch natural frequency
  ω_n ≈ √(C_Nα·q·A·(x_cp − x_cg)/I), the rocket can lock into resonance (MIL-HDBK-762, p. 5-42;
  the Aerobee 350's flights, NASA TM X-55236). A tab stuck hard over acts as a cant, and its
  steady roll rate must stay clear of ω_n along the flight.
- **Buzz and flutter of a hinged tab.**
  - Trim tab flutter comes from "low rotational stiffness and/or free play".
  - Transonic buzz is seen "between 0.85 and 0.90" Mach on aircraft. NACA's rocket-launched models
    buzzed between Mach 0.70 and 1.24; one also vibrated with its wing's bending at 0.54 to 0.64
    (RM L53I29).
  - The Shuttle met a rotation stiffness of c·ω/V = 2.0, where c is the tab chord, ω its rotation
    frequency and V the airspeed (NASA/TP-2006-212490 vol. 2).

## What hpr has and needs

**What hpr has.** Roll forcing from cant and roll damping for every fin set (`hpr_aero::Roll`),
the steady roll rate, and roll in the 6-degree-of-freedom state. Cant is capped at 15°.

**What it lacks:** hinged surfaces of any kind, a roll-resonance warning, and a canard-to-tail roll
interaction. Each surface will need its hinge line and chord fraction, travel and rate limits,
actuator lag, free play, and a fault state (stuck, or one flap out). The controller will need a
sample rate, sensor noise, bias and delay, and the neutral-until-burnout gate.

## Sources for validation

All are US government works on NASA's report server (NTRS id in brackets):

- **Tabs and ailerons:** TP-1353 [19790009638], RM L8E25 [19930085384], RM L54H17
  [19930088315], RM A7A17 [20050080743], TN-2486 [19930083051].
- **Canards:** TP-2157 [19830019688], TP-1316 [19780024124], TP-2401 [19870008235], TM X-3070
  [19740025319].
- **Brakes:** RM L8B06 [19930085315].
- **Lock-in:** TM X-55236 [19650021461], CR-76800 [19660023265].
- **Flutter and buzz:** TP-2006-212490 [20070008370], RM L53I29 [19930089146].

No open log of a rocket's roll rate under active roll control was found.

**Not verified:** NFPA 1122 and 1127 (paywalled), Glauert's R&M 1095, Nicolaides on roll lock-in,
the Student Launch handbooks for 2017-18, 2018-19 and 2020-21, and whether five AIAA student
airbrake papers carry flight data.

[adr-031]: ../decisions/0031-roll-from-canted-fins-and-roll-damping-by.md
[adr-192]: ../decisions/0192-the-2026-10-06-follow-up-best-over-first-and-export-control.md
[adr-193]: ../decisions/0193-the-2026-10-07-control-scope-drag-only-brakes-roll-only-surfaces.md
