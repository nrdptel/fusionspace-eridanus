# Plan

**This page says what each FusionSpace HPR release holds, in what order, and how likely each goal
is to work out.** It was made on October 6, 2026, from the roadmap as it stood that day. It goes
out of date as work ships or the roadmap changes. It gives no dates: releases come at product
boundaries.

> **How far to trust it.**
> - **What kind of figure:** a judgment of each goal's outlook, from the work done so far.
> - **Based on:** the roadmap and the validation results as of October 6, 2026.
> - **What to rely on instead:** [the roadmap](decisions-and-roadmap.md) for what comes next and
>   in what order; a published release for what is done.

## In short

Releases come in this order. A release is out once Neer, the project's owner, publishes it.

- **Release 0.1, the simulator.**
- **Release 0.2, the flight analyzer.**
- **Release 0.3, accuracy and fault diagnosis.**
- **Release 0.4, the competition kit.**
- **Release 0.5, the app preview.** It is out once Neer has used it and any changes that turns up
  are made.
- **Release 1.0, the app.** It is out, again, once Neer has used it.
- **Everything else on the roadmap that is software** comes after 1.0.
- **The one goal that is not only software,** an avionics board built and flown, follows Neer's
  build and the launch calendar, so spring 2027 at the earliest.

## The releases

| release | what it adds | also waits on |
|---|---|---|
| 0.1 the simulator | Monte Carlo from the command line and Python ([M4.6](decisions-and-roadmap.md#m4-6); [Monte Carlo](glossary.md#monte-carlo) flies many copies of a flight with its inputs scattered); logged apogees from real flights ([M2.3c1](decisions-and-roadmap.md#m2-3c1)); release builds and named fixes ([M10.1](decisions-and-roadmap.md#m10-1)) | Neer publishing it |
| 0.2 the flight analyzer | flight-log importers ([M7.1](decisions-and-roadmap.md#m7-1)); readings and smoothing ([M7.2](decisions-and-roadmap.md#m7-2)); a flight against its simulation ([M7.3](decisions-and-roadmap.md#m7-3)) | Neer publishing it |
| 0.3 accuracy and diagnosis | the accuracy campaign from [Mach](glossary.md#mach-number) 0 to 2.5 ([M1.14b to M1.14g](decisions-and-roadmap.md#m1-14)); logged traces ([M2.3c2](decisions-and-roadmap.md#m2-3c2)); fault diagnosis ([M7.4](decisions-and-roadmap.md#m7-4)) | Neer publishing it |
| 0.4 the competition kit | optimizing a Monte Carlo result ([M6.2e](decisions-and-roadmap.md#m6-2e)); challenge presets ([M6.3](decisions-and-roadmap.md#m6-3)); airbrakes ([M6.4](decisions-and-roadmap.md#m6-4)); submission packs ([M6.6](decisions-and-roadmap.md#m6-6)); ejection charges ([M6.7](decisions-and-roadmap.md#m6-7)); RASAero and RocketPy files ([M3.5](decisions-and-roadmap.md#m3-5), [M3.6](decisions-and-roadmap.md#m3-6)) | Neer publishing it |
| 0.5 the app preview | C and WebAssembly bindings ([M4.4](decisions-and-roadmap.md#m4-4)); the user-interface choice ([M9.0](decisions-and-roadmap.md#m9-0)); 3D replay of a real flight beside its simulated ghost ([M9.2](decisions-and-roadmap.md#m9-2)) | Neer using it, then publishing it |
| 1.0 the app | the desktop editor ([M9.1](decisions-and-roadmap.md#m9-1)); the web app that works offline ([M9.3](decisions-and-roadmap.md#m9-3)); the design assistant ([M8.1](decisions-and-roadmap.md#m8-1)); undo and edits ([M8.2](decisions-and-roadmap.md#m8-2)); RockSim files ([M3.4](decisions-and-roadmap.md#m3-4)) | Neer using it, then publishing it |
| after 1.0 | canards, CAD files, phone apps, accounts, the extended Mach band, motor, parachute and avionics design | see below |

A release is out when Neer publishes it. For 0.5 and 1.0 it also takes his own use of the app, and
any changes that use turns up.

## How feasible each goal is

| goal | outlook | why |
|---|---|---|
| A simulation library on par with RocketPy | high; mostly done | The physics, staging, pods, Monte Carlo, optimization and Python bindings have shipped. |
| Accuracy: 5% mean apogee error on real flights, at least as good as OpenRocket | medium | Against seven real flights, HPR Sim's apogees miss by 6.04% on average ([Accuracy](accuracy.md#real-flights)). Against 55 more from a private collection, HPR Sim's apogees are +9.83% above the logs on average and OpenRocket's +9.00%: neither meets 5%, and HPR Sim is close to OpenRocket ([Accuracy](accuracy.md#real-flights-of-the-private-collection)). On real flights, the inputs limit accuracy too: motors vary in impulse, logged masses are off, the weather is uncertain. Matching OpenRocket is likely; the 5% mean may not hold everywhere. If the campaign stalls, its gaps are written down rather than hidden ([the stopping rule](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0143-the-operating-envelope-and-a-stop-rule-for.md)). |
| Accuracy above Mach 1 | medium | Few real flights go supersonic. None of the seven public flights does. In the project's private collection of flight logs (not published), 5 of the 42 flights that state a top speed reach about Mach 1 or more, and more may show up when the rest of its logs are read. Flights near Mach 2 are rare. |
| File interop: OpenRocket, RockSim, RASAero, RocketPy | high | OpenRocket's files fly today. The others follow known formats and earlier work. |
| Competition design and submission packs | high | Mostly well-defined calculations from cited models. Testing that each safety number errs the safe way adds work, not risk. |
| Flight forensics: logs, fits, diagnosis | high; medium for diagnosis | Importers and fits are well defined. Diagnosis must name the right fault first for at least 80% of 200 simulated faulty flights; real anomalous flights depend on finding logs of them. |
| The app: editor, 3D replay, web app | medium | The code can be built steadily. Whether it is good to use is judged by people, starting with Neer, and that review sets the pace. |
| Phone apps and accounts | medium | They need an app-store account and a hosting provider, both Neer's choices. Neither is part of 1.0. |
| CAD files | high for meshes and drawings; medium-low for STEP | Few permissively licensed libraries read or write STEP's exact solids. STEP import may need a licensing decision. |
| Motor design | high for solid motors; medium for hybrids and liquids | Hybrid and liquid design waits first on a decision about US export rules for rocket propulsion. |
| Parachute design and avionics software | high | Each is checked against closed forms, published tables or exact round trips. |
| An avionics board | depends on Neer | It needs a board built, bench-tested and flown beside a commercial flight computer. |

## What only Neer can do

- Publish each release.
- Use the 0.5 and 1.0 apps before they count as done.
- Choose an app-store account and a hosting provider, when phone apps and accounts come up.
- Decide how US export rules apply, before hybrid and liquid design.
- Build and fly the avionics board.

None of these blocks the work before release 0.5.

## What could change the plan

1. **New goals.** The plan covers the roadmap as it stood on October 6, 2026.
2. **Bug reports.** 96 issues were open that day, 29 of them high priority. Critical ones come
   before milestone work.
3. **Hard milestones growing more than expected.** Accuracy, diagnosis and the app are the most
   open-ended.

## What 1.0 includes, and what comes after

Release 1.0 is the simulator, the flight analyzer, the competition kit and the desktop and web app.
Phone apps, accounts, canards, CAD files and the Mach 2.5–3.5 band come after 1.0. Motor, parachute
and avionics design come after that, one at a time ([the suite decision
record](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0162-the-suite-and-its-releases.md)).
Until 1.0, HPR Sim flies only commercial solid motors.
