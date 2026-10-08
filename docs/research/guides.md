# Guides: high-power rocketry, illustrated and live

**Bottom line:** high-power rocketry has deep text sources and plenty of one-formula calculators,
but no modern guide that draws the physics and lets a reader change it. The nearest are NASA
Glenn's beginner's pages, written for school students, subsonic, with some of their simulators
still in Java. The largest gaps are the ones hpr already models: stability as Mach and angle of
attack move the center of pressure, fin flutter and how uncertain its estimate is, drag through
Mach 1, and how far to trust a simulation at all. Certification steps, construction and avionics
wiring are covered well elsewhere, so a guide there should link out more than it writes. Nothing
here is built yet; where each piece goes is in [ADR-195, the guides][adr-195]. Researched October
7, 2026, by a survey of the sources below; the repository survey is in the ADR.

## What exists

| Source | Depth | Visual or interactive | What an MIT project may do |
|---|---|---|---|
| [NAR L2 exam pool][nar-l2] (HPRL2-2020-V2, 100 questions) | Exam level | Text, online practice test | Link only (copyrighted) |
| [Tripoli L2 study guide][tra-l2] (2022) | Answers cite Tripoli's safety code | Text | Link only |
| [Apogee *Peak of Flight*][pof] (600+ issues) | Deep, active | Static figures, videos | Link only |
| [OpenRocket user guide][or-docs] | How to use the tool | Screenshots | Link only: CC BY-SA's share-alike clashes with MIT |
| [ThrustCurve motor guide][tc-guide] | Strong on choosing a motor | Search tool, simple simulator | Link |
| [Rocketry Forum calculators][trf-calc] (2026) | Nine tools with worked examples and stated limits | Input boxes, no drawings | Link |
| [NASA Glenn, guide to rockets][nasa-bga] | School level, subsonic: stability, drag, weathercocking, staging | Simulators, [some in Java][nasa-sims] | Public-domain figures reusable with credit ([NASA's terms][nasa-media]); contractor material and logos excepted, no endorsement implied |
| Stine, *Handbook of Model Rocketry*; Canepa, *Modern High-Power Rocketry 2* (2005) | The standard books | Photos | Cite only |
| BPS.space (Joe Barnard), video | Thrust vectoring, avionics, simulation | Video | Link only |
| Altimeter manuals (Eggtimer, Missile Works) | Wiring and setup | PDF diagrams | Link only |

[NFPA 1127][nfpa-free] is copyrighted; NFPA lets anyone read it free online but not copy it, so a
guide cites its sections and puts them in its own words.

## The gaps, largest first

1. **Stability past the subsonic picture.** Readers get a rule of thumb (one to two calibers) and
   nose-weight calculators; nobody shows the center of pressure moving with Mach and angle of
   attack, where Barrowman's method stops holding, or what too much margin does.
2. **Fin flutter.** Calculators exist, but none shows what flutter is or how wide its estimate's
   spread is. The Rocketry Forum's [flutter calculator][trf-flutter] notes that the forms printed
   in *Peak of Flight* #291 (2011) and #416 (2016) overstate the flutter speed by √2, about 41%,
   corrected in #615 (2023): the flattering side, and still widely copied. Loft's lesson
   [L32](../decisions-and-roadmap.md#l32) records the same √2 error in Loft's constant. The issue
   numbers come from the forum's page; read the issues before a guide repeats them.
3. **How far to trust a simulation.** No source draws error bars on a predicted apogee. hpr's
   committed validation reports have the numbers to do it.
4. **Drag through Mach 1:** the transonic rise, base drag, surface finish. Static articles only.
5. **Wind, weathercocking and rail exit in one picture.** NASA's page is at school level;
   elsewhere it is a single number.
6. **Recovery as a timeline:** dual deploy's events, descent rates, drift and opening shock in one
   drawing. Charge calculators exist; the whole sequence is drawn nowhere.
7. **Thrust curves:** a smaller gap, since ThrustCurve covers choosing a motor, though not average
   against peak thrust or choosing the delay.

Covered well already, so linked more than written: the certification steps, construction (Canepa
and the forum's build threads) and avionics wiring (the makers' manuals). Staging and air starts
are covered thinly but weigh heavily on safety; they come last, citing the safety codes.

## What the best explainers do

- **One idea per figure, and the model does the explaining.** Bartosz Ciechanowski's essays (for
  example [cameras and lenses][ciechanowski]) build each topic from many small figures the
  reader can move, each showing one effect.
- **Live numbers in the prose.** Bret Victor's [explorable explanations][victor] make an
  author's assumptions into numbers the reader can drag, the result updating in the sentence.
- **A tight, repeated format.** [Seeing Theory][seeing-theory] uses six chapters with three
  figures each, on plain web standards.
- **What to avoid:** NASA's Java simulators show how plugin-era figures die. Plain HTML, SVG and
  WebAssembly last.

Two lessons are hpr's own:

- **The figures run hpr.** A figure computed by the library itself (compiled to WebAssembly, as
  [M4.4][m4-4] plans) can never disagree with the simulator, and a CI test can hold the two
  equal.
- **Draw the uncertainty and its side.** A band, not a line, with the flattering side marked: the
  project's rule for safety outputs, applied to its figures.

## What a guide must not do

- Copy text or figures from the copyrighted and share-alike sources above; cite and link instead.
- Reproduce or answer certification exam questions; link the official pools.
- Print a single charge size; give M6.7's range with "ground test first" ahead of it.
- Say a rocket is safe or unsafe, or will or won't pass; the RSO and the safety code decide.
- Quote a rule without its source date, a stale-after date and "not legal advice".

[adr-195]: ../decisions/0195-the-2026-10-07-guides-rocketry-explained.md
[m4-4]: ../decisions-and-roadmap.md#m4-4
[nar-l2]: https://narocket.clubexpress.com/HPRCERTIFICATIONEXAMCURRENTVERSIONS
[tra-l2]: https://tripoli.org/Level2
[pof]: https://www.apogeerockets.com/PeakOfFlight/Archive
[or-docs]: https://openrocket.readthedocs.io/en/latest/
[tc-guide]: https://www.thrustcurve.org/motors/guidehelp.html
[trf-calc]: https://www.rocketryforum.com/rocket-calculators/
[trf-flutter]: https://www.rocketryforum.com/rocket-calculators/fin-flutter/
[nasa-bga]: https://www1.grc.nasa.gov/beginners-guide-to-aeronautics/guide-to-rockets/
[nasa-sims]: https://www1.grc.nasa.gov/beginners-guide-to-aeronautics/bga-simulations/
[nasa-media]: https://www.nasa.gov/nasa-brand-center/images-and-media/
[nfpa-free]: https://www.iafc.org/topics-and-tools/resources/resource/free-access-to-nfpa-codes-and-standards
[ciechanowski]: https://ciechanow.ski/cameras-and-lenses/
[victor]: https://en.wikipedia.org/wiki/Explorable_explanation
[seeing-theory]: https://blog.cs.brown.edu/2018/01/22/seeing-theory-teaching-statistics-through-interactive-web-based-visualizations/
