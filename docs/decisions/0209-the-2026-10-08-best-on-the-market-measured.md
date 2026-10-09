# ADR-209: Neer's 2026-10-08 standard: the best on the market, measured, for every product (2026-10-08)

- **Status:** accepted; tightens ADR-162 §2's release checklist, ADR-196's product guides and M1.14's accuracy target; amends ADR-147 §6 (the roadmap's budget)
- **Summary:** every product aims to be the best available: never worse than the best comparable tool on any axis, and decisively better on the two or three axes its users care most about, plus at least one useful capability no other tool has. Claims stand only on committed comparisons. A generated scoreboard ratchets: a release can't make a number worse without an ADR (M0.10). Accuracy is judged on held-out logged flights with predictions recorded before the comparison (M2.7), and uncertainty on whether its ranges hold real flights as often as they say (M2.8). Release 0.3 needs held-out mean absolute apogee error below OpenRocket's on the same flights, bias within 3% and honest 90% ranges, or an ADR with the miss. An open benchmark (M2.6), a correction for the bias every simulator shares (M7.5), steps-to-first-answer measurement now (M0.11) and tests by flyers outside the project at 0.5 complete it. Each guide has a *Compared with* section; each product line starts with a survey.

[adr-143]: 0143-the-operating-envelope-and-a-stop-rule-for.md
[adr-162]: 0162-the-suite-and-its-releases.md
[adr-196]: 0196-the-2026-10-07-product-guides.md

**Context.** On 2026-10-08, after release 0.1, Neer set the standard for every product: the best on the market in
every way, with higher fidelity, accuracy and resolution than the tools people use now, easier to use, and doing new,
useful things they don't. A product that performs the same as the tools it replaces has no reason to exist. He also
asked for the strongest form of that standard, not just its statement.

Where the simulator stands, from the committed reports:

- **Against real flights it is level with OpenRocket, not ahead.** Over the 55 logged flights of the private collection,
  HPR Sim's apogees average +9.83% above the logs (mean absolute 13.96%); OpenRocket 24.12's +9.00% (13.08%). The two
  agree within 5% on 53 of 55, so they likely share a cause that neither models: drag a real airframe adds, or motors
  delivering less than their files. On the 7 public real flights HPR Sim's mean absolute error is 6.04%.
- **Its flight mechanics match RocketPy's:** on the same drag, all 142 gated metrics pass, apogee within +0.04% to
  +1.21%.
- **It already does things neither does,** and is behind on a design app, years of users, hybrid and liquid motors and
  transonic nose drag (#67).

What the field publishes, read on 2026-10-08:

- **No tool's accuracy is measured on held-out flights.** RocketPy's "about 1%" comes from three university flights
  flown on the teams' own drag curves, which tests the trajectory more than the drag; its later examples are 7 to 9%
  off ([Ceotto et al., 2021](https://ascelibrary.com/doi/10.1061/%28ASCE%29AS.1943-5525.0001331);
  [RocketPy's flight examples](https://docs.rocketpy.org/en/latest/examples/index.html)). RASAero II's 36 flights
  average 3.47%, with several fitted after the flight and no account of how they were chosen
  ([RASAero's comparisons](https://www.rasaero.com/comparisons-alt.htm)). OpenRocket's published validation is three
  flights from 2009, one after its thrust curve was replaced
  ([Niskanen, 2009](https://github.com/openrocket/openrocket/releases/download/Development_of_an_Open_Source_model_rocket_simulation-thesis-v20090520/Development_of_an_Open_Source_model_rocket_simulation-thesis-v20090520.pdf)).
  None reports whether its uncertainty ranges hold real flights.
- **Single flights have a floor.** Certified motors' total impulse varies about 0.5 to 1.4% in certification reports
  ([ThrustCurve](https://www.thrustcurve.org/info/certification.html)); a barometric altimeter's height above the pad
  moves about 0.35% per °C that the air differs from the standard atmosphere; wind adds more. A few percent on one
  flight is near the limit, so beyond it "better" means smaller bias and honest uncertainty.
- **Users' most repeated complaint is that the tools disagree and nothing says which is right** (for example OpenRocket
  against RASAero II on the same rocket,
  [The Rocketry Forum](https://www.rocketryforum.com/threads/openrocket-and-rasaero-ii-predicted-altitude-difference.186458/)),
  followed by supersonic drag, weathercocking, fin flutter, landing spread, logs, interoperability and scripting. No
  public benchmark of designs with their logs exists.

**Decision.**

1. **The standard.** Each product is never worse than the best comparable tool on any axis it measures, and is
   decisively better on the two or three axes its users care most about, named in its survey (§11). It does at least
   one useful thing no comparable tool does. Over time every axis is won; matching the field is never the finish.
2. **Measured, never claimed.** "Better" appears only beside a committed comparison: the same inputs, the tools named
   with their versions and set up as their docs advise, numbers read from a report by a test. Where a product is behind,
   its docs say so, with the number.
3. **The scoreboard (M0.10).** `cargo xtask scoreboard` writes `validation/reports/scoreboard.md`: for each product,
   each axis (accuracy, bias, honest uncertainty, speed, interop round trips, features, steps to a first answer) against
   each named tool. A check fails when a release makes a number worse than the committed one, unless an ADR records why.
4. **Held-out flights and predictions recorded first (M2.7).** A seeded third of the logged flights is held out: never
   used to fit, tune or choose a model. Reports give both halves. A new flight gets HPR Sim's prediction recorded, with
   its commit and date, before its log is compared.
5. **Honest uncertainty (M2.8).** Each logged flight is also flown as a dispersion, and the report gives how often its
   50% and 90% apogee ranges hold the logged apogee, with Clopper–Pearson bounds. A range that holds real flights as
   often as it says is a claim no comparable tool makes today.
6. **The shared bias (M7.5).** The analyzer fits drag from logs (M7.3); a correction learned across many flights, for
   the drag real airframes carry and the impulse motors deliver, ships only if it lowers the held-out error.
7. **Release 0.3's target.** On the held-out logged flights that both fly, HPR Sim's mean absolute apogee error is below
   OpenRocket's, its mean bias is within ±3%, and its 90% ranges hold the logged apogee as often as their bounds allow.
   If the accuracy campaign ends by [ADR-143][adr-143]'s stop rule short of this, an ADR records the measurement and the
   gap stays in the report; the target doesn't move.
8. **Each release's guide compares (ADR-196 tightened).** From 0.2, and in the simulator's guide (M0.8a), a *Compared
   with* section names the comparable tools, tabulates what each does, gives the measured comparisons and the gaps, and
   names at least one capability none of them has, each absence checked against that tool's current docs with the date.
9. **An open benchmark (M2.6).** Public designs with their logs and weather (the project's public flights; RocketPy's
   MIT-licensed examples after a per-file license check), a runner that scores any simulator's predictions the same way,
   and a page with the scores. It answers the users' first complaint and lets anyone check the claims in §2.
10. **Ease of use is measured.** M0.11 counts the steps to a first answer (open a design, fly it, get the landing
    spread) in HPR Sim's command line and Python against OpenRocket and RocketPy, into the scoreboard. Release 0.5
    adds a test by at least five flyers outside the project, their times and problems recorded as aggregates.
11. **Each product line starts with a survey.** Before a line's first milestone, a note under `docs/research/` lists the
    tools it competes with, what each does best, the two or three axes the line will win on, and where it won't yet.
12. **The app wins on what is new.** M9.0's ADR names the app's winning axes from its survey: 3D replay against the real
    flight, analysis, honest uncertainty, the web and the Mac. Until its editor is better, `.ork` files round-trip
    without loss, so a flyer can keep designing in OpenRocket and fly, compare and analyze here.
13. **Every milestone moves a number.** From now, a new milestone's *done when* names the scoreboard number it moves, or
    says it moves none and why it is still worth doing.
14. **Releases still ship.** The standard governs what each release must show; 0.1 shipped on 2026-10-08 as planned.
15. **The roadmap's budget rises** from 43,500 to 46,500 bytes, for the milestones above.
