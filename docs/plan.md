# Plan

Read the order of FusionSpace HPR's releases, what each will include, roughly when each may come,
and how likely each goal is to work out.

**The plan was made on October 6, 2026, from the roadmap as it stood that day.** It goes out of
date as work ships or the roadmap changes. Releases come at product boundaries, not on dates.
The release rows and the paragraph under them were brought up to date on November 24, 2026, when
releases stopped waiting on outside users ([the decision record](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0377-no-release-waits-on-users-and-release-0-2-comes-next.md)).
[Estimates of when each release may come](#when-each-release-may-come) were added the same day
and made again on November 26, 2026. That moved 1.0's estimate from the end of March to the end
of April 2028. The padded date itself moved one day, to April 1, and rounding carried it into a
new month. A new milestone, time to apogee against the logs, joined the queue, and finished work
took off about as much time as it added
([the decision record](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0380-the-2026-11-26-review-time-to-apogee-held-to-the-logs-predictions-first.md)).
The outlooks further down are still October 6's.

> **How far to trust it.** Each goal's outlook here is a judgment, from the work done so far. It's
> based on the roadmap and the validation results as of October 6, 2026. The release estimates are
> guesses from November 26, 2026, padded because work has run long, and not promises. For what
> comes next and in what order, rely on [the roadmap](decisions-and-roadmap.md) instead, and for
> what's done, on a published release.

## In short

Releases come in this order. A release is out once Neer, the project's owner, publishes it. By
[the padded estimates](#when-each-release-may-come), 0.2 may come by the end of December 2026 and
1.0 by the end of April 2028.

1. Release 0.1, the simulator.
2. Release 0.2, the flight analyzer.
3. Release 0.3, accuracy and fault diagnosis.
4. Release 0.4, the competition kit.
5. Release 0.5, the app preview. It's out once
   [the project's own acceptance pass](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0282-the-project-tests-each-ui-milestone-itself.md)
   is done and any changes it turns up are made.
6. Release 1.0, the app. It's out, again, once its acceptance pass is done.
7. Everything else on the roadmap that's software comes after 1.0.

The one goal that isn't only software, an avionics board built and flown, follows Neer's build
and the launch calendar. So it comes in spring 2027 at the earliest.

## The releases

| release | what it adds | also waits on |
|---|---|---|
| 0.1 the simulator | [Monte Carlo from the command line and Python](decisions-and-roadmap.md#m4-6) ([Monte Carlo](glossary.md#monte-carlo) flies many copies of a flight with its inputs scattered); [logged apogees from real flights](decisions-and-roadmap.md#m2-3c1); [release builds and named fixes](decisions-and-roadmap.md#m10-1) | nothing: it came out on 2026-10-08 ([How a release is built](releasing.md)) |
| 0.2 the flight analyzer | [flight-log importers](decisions-and-roadmap.md#m7-1); [readings and smoothing](decisions-and-roadmap.md#m7-2); [a flight against its simulation](decisions-and-roadmap.md#m7-3): residuals, the log drawn beside its simulation, and drag fitted from the log on the command line and in Python, its intervals passing their test again; [a spread beside each flight's apogee](decisions-and-roadmap.md#m2-8) | its emulated flyers' pass, then Neer publishing it |
| 0.3 accuracy and diagnosis | the accuracy campaign from [Mach](glossary.md#mach-number) 0 to 2.5 ([M1.14b to M1.14g and M1.14i](decisions-and-roadmap.md#m1-14)); [logged traces](decisions-and-roadmap.md#m2-3c2); [the fit's second, Bayesian interval](physics/flight-fit.md#the-bayesian-option); [fault diagnosis](decisions-and-roadmap.md#m7-4) | its emulated flyers' pass, then Neer publishing it |
| 0.4 the competition kit | [optimizing a Monte Carlo result](decisions-and-roadmap.md#m6-2e); [challenge presets](decisions-and-roadmap.md#m6-3); [airbrakes](decisions-and-roadmap.md#m6-4); [submission packs](decisions-and-roadmap.md#m6-6); [ejection charges](decisions-and-roadmap.md#m6-7); [RASAero](decisions-and-roadmap.md#m3-5) and [RocketPy](decisions-and-roadmap.md#m3-6) files | its emulated flyers' pass, then Neer publishing it |
| 0.5 the app preview | [C and WebAssembly bindings](decisions-and-roadmap.md#m4-4); [the user-interface choice](decisions-and-roadmap.md#m9-0); [3D replay of a real flight beside its simulated ghost](decisions-and-roadmap.md#m9-2) | its acceptance pass and its emulated flyers' study, then Neer publishing it |
| 1.0 the app | [the desktop editor](decisions-and-roadmap.md#m9-1); [the web app that works offline](decisions-and-roadmap.md#m9-3); [the design assistant](decisions-and-roadmap.md#m8-1); [undo and edits](decisions-and-roadmap.md#m8-2); [RockSim files](decisions-and-roadmap.md#m3-4) | its acceptance pass and its emulated flyers' pass, then Neer publishing it |
| after 1.0 | canards, CAD files, phone apps, accounts, the extended Mach band, motor, parachute and avionics design | see below |

A release is out when Neer publishes it. For 0.5 and 1.0 it also takes the project's own acceptance
pass, and any changes that pass turns up. No release waits on outside users. From 0.2 on, each has
an emulated flyers' pass instead: three written personas from public sources, each played by a
reviewer who didn't build the release, try its tasks from its files and guide alone. Two or three
defects planted in a copy measure how much they catch, and a pass that catches none doesn't count.
Release 0.5 has a larger study: at least twelve emulated flyers and eight to ten planted
defects. No pass has run yet. Emulated flyers are assumed to be more patient than people and to
agree with each other more, so a pass shows whether a careful newcomer can finish the tasks, not
whether flyers want the tool ([the decision record](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0377-no-release-waits-on-users-and-release-0-2-comes-next.md)).

### When each release may come

| release | estimate |
|---|---|
| 0.2 the flight analyzer | by the end of December 2026 |
| 0.3 accuracy and diagnosis | by the end of May 2027 |
| 0.4 the competition kit | by the end of August 2027 |
| 0.5 the app preview | by the end of November 2027 |
| 1.0 the app | by the end of April 2028 |

These are [estimates](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/research/release-estimates.md), made on November 26, 2026, and not promises. A release still comes out only
when its checklist passes, and none is cut short to meet a date. Each estimate counts the
milestones ahead of its release in [the roadmap's queue](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/ROADMAP.md#queue).
It sizes each by the pieces finished milestones have taken and goes at the pace of the 29 days
before. That time is then padded by judgment, one stretch between releases at a time. The stretch
to 0.2 is doubled, the next two tripled, and the two that hold the app, the newest kind of work,
quadrupled.

The padding is there because work has run long so far. [The log importers](decisions-and-roadmap.md#m7-1)
were planned on October 12 as two pieces of work, each shipped on its own, and took 27 such
pieces. In November, planning reviews added at least six milestones to the queue ahead of 0.3.
A release that slips moves every later one with it, and the later a release, the more its
estimate may move.

## How feasible each goal is

| goal | outlook | why |
|---|---|---|
| A simulation library on par with RocketPy | high; mostly done | The physics, staging, pods, Monte Carlo, optimization and Python bindings have shipped. |
| Accuracy: 5% mean apogee error on real flights, at least as good as OpenRocket | medium | Against 12 public real flights, HPR Sim's apogees miss by 8.83% on average ([Accuracy](accuracy.md#real-flights)). Against 60 more from a private collection, rechecked against its catalog, HPR Sim's apogees miss by 12.10% on average (mean absolute) and OpenRocket's by 11.26%. Signed, HPR Sim's are +8.01% above the logs and OpenRocket's +6.72%. Neither meets 5%, and HPR Sim is close to OpenRocket ([Accuracy](accuracy.md#real-flights-of-the-private-collection)). On real flights, the inputs limit accuracy too: motors vary in impulse, logged masses are off, the weather is uncertain. Matching OpenRocket is likely; the 5% mean may not hold everywhere. If the campaign stalls, its gaps are written down rather than hidden ([the stopping rule](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0143-the-operating-envelope-and-a-stop-rule-for.md)). |
| Accuracy above Mach 1 | medium | Few real flights go supersonic. Of the 12 public flights, only Andromeda passes Mach 1, at Mach 1.019 in the simulation. In the project's private collection of flight logs (not published), 5 of the 42 flights that state a top speed reach about Mach 1 or more, and more may show up when the rest of its logs are read. Flights near Mach 2 are rare. |
| File interop: OpenRocket, RockSim, RASAero, RocketPy | high | OpenRocket's files fly today. The others follow known formats and earlier work. |
| Competition design and submission packs | high | Mostly well-defined calculations from cited models. Testing that each safety number errs the safe way adds work, not risk. |
| Flight forensics: logs, fits, diagnosis | high; medium for diagnosis | Importers and fits are well defined. Diagnosis must name the right fault first for at least 80% of 200 simulated faulty flights; real anomalous flights depend on finding logs of them. |
| The app: editor, 3D replay, web app | medium | The code can be built steadily. Whether it is good to use is judged by the project's own acceptance pass on each milestone. |
| Phone apps and accounts | medium | They need an app-store account and a hosting provider, both Neer's choices. Neither is part of 1.0. |
| CAD files | high for meshes and drawings; medium-low for STEP | Few permissively licensed libraries read or write STEP's exact solids. STEP import may need a licensing decision. |
| Motor design | high for solid motors; medium for hybrids and liquids | Hybrid and liquid design waits first on a decision about US export rules for rocket propulsion. |
| Parachute design and avionics software | high | Each is checked against closed forms, published tables or exact round trips. |
| An avionics board | depends on Neer | It needs a board built, bench-tested and flown beside a commercial flight computer. |

## What only Neer can do

- Publish each release.
- See the 0.5 and 1.0 apps before they go out. His look never holds up their milestones.
- Choose an app-store account and a hosting provider, when phone apps and accounts come up.
- Decide how US export rules apply, before hybrid and liquid design.
- Build and fly the avionics board.

None of these blocks the work before release 0.5.

## What could change the plan

1. New goals, since the plan covers the roadmap as it stood on October 6, 2026.
2. Bug reports. On that day 96 issues were open, 29 of them high priority. Critical ones come
   before milestone work.
3. Hard milestones growing more than expected. Accuracy, diagnosis and the app are the most
   open-ended.

## What 1.0 includes, and what comes after

Release 1.0 is the simulator, the flight analyzer, the competition kit and the desktop and web app.
Phone apps, accounts, canards, CAD files and the Mach 2.5–3.5 band come after 1.0. Motor, parachute
and avionics design come after that, one at a time ([the suite decision
record](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0162-the-suite-and-its-releases.md)).
Until 1.0, HPR Sim flies only commercial solid motors.
