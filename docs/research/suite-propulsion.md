# Hybrids, liquids and experimental solids: research for the propulsion line

**Bottom line:** design experimental solids first. The physics is textbook, demand is proven by
openMotor, a runnable oracle exists, and the result is a thrust curve hpr already flies. Hybrids
come next, then liquids, which have the weakest public validation data and the most
export-control doubt. Flying a user's own thrust curve, labelled uncertified, costs almost
nothing and comes before all of it. This note backs the propulsion milestones ([M11.1 to M11.5](../ROADMAP.md),
[ADR-162, the suite and its releases][adr-162]). Researched 2026-10-04; nothing here is
implemented. Items marked *unverified* come from search summaries only. Not legal advice.

## Experimental solids

- **The model.** Burn rate r = a Pⁿ, valid for n < 1; a in SI units (m/s per Paⁿ), where
  Nakka quotes mm/s with pressure in MPa. Steady chamber pressure follows from Kn, the burning
  area over the throat area, the propellant's density ρ and the characteristic velocity c*:
  P = (Kn a ρ c*)^(1/(1−n)). That is the steady value, not the maximum expected operating
  pressure, which adds a high burn rate, a hot grain and the start-up peak. Thrust is
  F = C_F P A_t, corrected for nozzle divergence and throat losses. Each grain regresses with its
  geometry; a fast marching method handles any core shape. Erosive burning: Lenoir–Robillard
  ([MATEC 2018](https://www.matec-conferences.org/articles/matecconf/pdf/2018/12/matecconf_icmme2018_03001.pdf)).
- **Sources.** Sutton and Biblarz, *Rocket Propulsion Elements*, 9th ed. (Wiley, 2017); NASA
  SP-8039 ([NTRS 19720011135](https://ntrs.nasa.gov/citations/19720011135)); Nakka's pages
  ([burn rate](https://www.nakka-rocketry.net/ptburn.html)).
- **Tools.**

  | tool | license | use here |
  |---|---|---|
  | [openMotor](https://github.com/reilleya/openMotor) 0.6.2 | GPL-3.0 | never read; run headless as an oracle (`openmotor -h`, [manpage](https://manpages.ubuntu.com/manpages/jammy/man1/openmotor.1.html)); its `.ric` files are YAML |
  | [BurnSim](https://www.burnsim.com/) | commercial | none |
  | [Nakka's SRM and CASING spreadsheets](https://www.nakka-rocketry.net/softw.html) | freeware, no license stated | published numbers only |
  | [jsrm](https://github.com/jbgust/jsrm) | GPL-3.0 | never read; run as an oracle |

- **Validation data is thin.** Nakka's KSB-002 delivered 1821 N·s against a 2000 N·s design, 9%
  short ([KSB-002](https://www.nakka-rocketry.net/ksb002.html)); KSB-001 fell 11% short
  ([KSB-001](https://www.nakka-rocketry.net/ksb001.html)). A KNSB study reports 7.7% in pressure
  and 3.2% in thrust at steady state ([Energies 19:2260](https://doi.org/10.3390/en19102260),
  *unverified*). A 1% change in burn rate moves thrust 1.5 to 2%, so the propellant's burn rate
  sets the error more than the model does. Nakka's data carries no license: cache it under
  `refs/`, publish only error statistics.

## Hybrids

- **What flies today.** ThrustCurve's API, queried 2026-10-04, lists 152 hybrids, 90 available.
  Contrail is current (67 motors). Hypertek's production status is unclear, and RATTworks is out
  of production.
- **The files lose the tank.** A RASP file's propellant mass lumps fuel and nitrous oxide
  together ([format](https://www.thrustcurve.org/info/raspformat.html)), and neither RASP nor
  RockSim separates tank from grain. The oxidizer's center-of-gravity drift is lost. hpr already
  refuses to convert a hybrid `.rse` to `.eng` (`crates/hpr-motor/src/convert.rs`).
- **Tank models.** Zimmerman and others compare an equilibrium model with two non-equilibrium
  ones on six tests ([AIAA 2013-4045](https://www.ibb.ch/publication/N2O/AIAA%202013-4045%20.pdf)).
  Equilibrium: pressure within 15%. Zilliac and Karabeyoglu: within 5%, with a factor fitted to
  each system. Injector flow: Dyer and others
  ([AIAA 2007-5702](https://www.semanticscholar.org/paper/Modeling-Feed-System-Flow-Physics-for-Propellants-Dyer-Doran/e871c217a03b0510084407292a6fcfff558ac17e)).
  Fuel regression: Marxman and Gilbert, rate proportional to G^0.8
  ([doi](https://doi.org/10.1016/S0082-0784(63)80046-6)).
- **An oracle.** RocketPy's hybrid motor is MIT and takes the mass flow as an input
  ([docs](https://docs.rocketpy.org/en/latest/user/motors/hybridmotor.html)).

## Liquids

- **Thermochemistry.** NASA's CEA is published under Apache-2.0 with the RP-1311 examples
  ([nasa/cea](https://github.com/nasa/cea); [RP-1311](https://ntrs.nasa.gov/api/citations/19950013764/downloads/19950013764.pdf)),
  so it can be ported with attribution. Cantera is BSD-3. RocketCEA is GPL-3: run it, never read
  it. Stanford's PropSim states no license: don't read it.
- **Validation data is scarce.** One calibrated model against McGill's hot fires
  ([arXiv 2302.06725](https://arxiv.org/abs/2302.06725)).
- **Demand is new.** Tripoli allows nitrous and alcohol liquids from 2026-01-01, pressurised by
  the nitrous alone, with no throttling in flight
  ([Tripoli](https://tripoli.org/content.aspx?page_id=5&club_id=795696&item_id=126571)).

## What the simulator needs

A motor's mass becomes parts at their own positions: tank liquid, tank vapour, fuel grain and
dry hardware. The tank's center of gravity moves as it drains, the vapour keeps burning after the
liquid runs out, and oxidizer can be left at burnout. Neither motor file format carries this, so
the hpr format needs a motor document of its own.

## Safety and the line not crossed

- **Case burst margin is a safety number.** IREC's 2026 guide asks for a burst pressure of twice
  the largest operating pressure for metal cases and three times for composite, a proof test at
  1.5 times, and a static fire of every student-built motor
  ([DTEG §6.18](https://static1.squarespace.com/static/687d3841f7d71450d1ba824d/t/69a0d10868129127b9febed4/1772146953831/2026+IREC+DTEG+V1.1+-+REDLINES.pdf)).
  Too low a maximum pressure, or too high a burst margin, is the flattering side; the flown
  thrust curve stays nominal. Every design says "static test first".
- **No formulations.** hpr takes a propellant as properties (a, n, density, c*, ratio of specific
  heats), as openMotor does. It never gives a recipe, a mixing method or a manufacturing step.
- **Rules.** Tripoli's Unified Safety Code applies from January 2026
  ([tripoli.org](https://www.tripoli.org/safetycode)); NFPA 1127 is the high-power code.
- **Export control (Neer's legal call before M11.5, hybrid and liquid design):** the EAR treats software
  posted publicly as published and outside its scope
  ([15 CFR 734.7](https://www.law.cornell.edu/cfr/text/15/734.7)). ITAR's public-domain rule does
  not name the internet ([22 CFR 120.34](https://www.law.cornell.edu/cfr/text/22/120.34)), and its
  exclusion for principles taught in schools ([120.33](https://www.law.cornell.edu/cfr/text/22/120.33))
  plausibly covers textbook solid ballistics. Liquid engine design is the less certain part.

[adr-162]: ../decisions/0162-the-suite-and-its-releases.md
