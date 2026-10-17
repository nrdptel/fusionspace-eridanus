# ADR-279: The 2026-10-17 review: fin flutter held to measurement, and intervals by design (2026-10-17)

- **Status:** accepted; adds milestone [M2.11][m2-11]; tightens [ADR-209][adr-209] §7 and [M10.3][m10-3]; no release moves
- **Summary:** HPR Sim's fin flutter check, a safety output, is tested only against Martin's own worked examples, and nothing shows which way it errs on a fin someone else measured (the evidence register's row G22). A published wind-tunnel test found five identical composite fins fluttering at 19.6–21.5 m/s (64–71 ft/s); on that fin HPR Sim's flutter speed reads about 10.7 m/s (35 ft/s), on the safe side. M2.11 scores HPR Sim's flutter readings against at least three distinct measured fins from at least two research groups, each case's inputs and test conditions stated, and against an independent BSD-licensed calculator; it states the largest error on the flattering side and the range the cases cover, and adds a flutter row to the scoreboard. It is queued after release 0.3 and before the IREC prediction table that prints a flutter speed (M6.6). Separately, release 0.3's paired bootstrap interval is also reported resampled by design, with the number of designs, since resampling single flights treats repeat flights of one design as independent.

[m2-11]: ../decisions-and-roadmap.md#m2-11
[m10-3]: ../decisions-and-roadmap.md#m10-3
[m6-6]: ../decisions-and-roadmap.md#m6-6
[adr-209]: 0209-the-2026-10-08-best-on-the-market-measured.md
[adr-144]: 0144-the-2026-10-03-planning-review-leaner-bookkeeping.md
[adr-151]: 0151-hpr-sim-fixtures-joins-the-reference-library-as.md
[adr-236]: 0236-the-2026-10-11-review-supersonic-flights-from-published-comparisons.md
[g22]: ../evidence.md#g22
[i-675]: https://github.com/nrdptel/fusionspace-eridanus/issues/675
[flutter]: ../physics/flutter.md
[readings]: ../physics/flutter.md#readings-where-the-source-leaves-room
[hardware]: ../research/suite-hardware-line.md
[dinulovic]: https://doi.org/10.5937/fme2203576D
[bennett]: https://github.com/jkb-git/Fin-Flutter-Velocity-Calculator
[rp-1200]: https://github.com/RocketPy-Team/RocketPy/issues/1200
[irec27]: https://www.esrarocket.org/s/IREC-2027-Rules-and-Requirments-v10.pdf
[sl-note]: ../research/student-launch-apogees.md

**Context.** The guardrails of [ADR-144][adr-144] rank a flattering error in a safety output as the
worst class: a flutter speed that reads too high tells a flyer a fin is safe when it is not. HPR
Sim checks fin flutter by D. J. Martin's criterion in NACA TN 4197 (1958) and gives two readings
([the flutter page][flutter]): the figure 3 check, which places the fin against the line Martin's
missile and wind-tunnel wings drew and decides, and the ratio of the criterion's flutter speed
$V_f$ (his eq. 18) to the rocket's airspeed at peak dynamic pressure. Both reproduce Martin's worked
examples. That checks the arithmetic, not the answer: nothing here holds either reading against a
fin that someone else measured fluttering, and the evidence register says so in
[row G22][g22], open as [#675][i-675].

A search on 2026-10-17 found a measurement of the kind the row lacks, though not on a fin glued to
a tube. Dinulović and others ("Flutter analysis of tapered composite fins: analysis and
experiment", [FME Transactions 50, 2022][dinulovic], pp. 576–585, Tables 3 and 4 on p. 583)
clamped five identical quasi-isotropic e-glass fins in a wind tunnel and raised the speed in
0.5 m/s steps until each fluttered: 19.6–21.5 m/s (64–71 ft/s) over ten runs. The fin:

| Root chord | Tip chord | Span | Thickness |
|---|---|---|---|
| 180 mm (7.1 in) | 90 mm (3.5 in) | 262 mm (10.3 in) | 0.65 mm (0.026 in) |

The paper evaluates the NACA formula at 15.3 m/s (50 ft/s) for it, 21.9–28.8% below the onsets,
taking the thickness over the mean chord and an effective shear modulus of 7.96 GPa. HPR Sim takes
the thickness over the root chord, its deliberate lower reading ([readings][readings]), and with
the same modulus in sea-level standard air gives about 10.7 m/s (35 ft/s), 45–50% below. With the
ply's own shear modulus, 4.3 GPa, it gives 7.9 m/s. So on this one fin the speed errs on the safe
side, and the inputs move it a lot: the chord choice by 46%, the modulus by 26%. One fin is not a
bound. It is a clamped plate at Mach 0.06 with a thickness ratio of 0.36%, outside the 1–10% Martin's
data spans; a hobby fin glued to a tube is less stiff at its root, near Mach 1 where the criterion
is most likely to read high. The same group's test of 3D-printed plates (Materials, 2025, cited in
[the hardware line's note][hardware]) is not independent of this one.

The same search found an independent flutter calculator with its own examples, published under the
BSD 2-Clause license ([Bennett's][bennett]), and a defect in another tool: RocketPy's flutter speed
reads √2 high ([RocketPy #1200][rp-1200], open), the flattering side. IREC's 2027 prediction table
asks for a flutter speed ([Table 4-1, p. 21][irec27]), which [M6.6][m6-6] will print.

Release 0.3's target ([ADR-209][adr-209] §7) reports a paired bootstrap interval, a resampling
estimate of how far the difference between HPR Sim's and OpenRocket's mean absolute error on
held-out flights could move. Resampling flights one at a time treats them as independent; repeat
flights of one design are not. The public Student Launch predictions show the size this can take
for a different statistic: the 95% interval on the median apogee error is 6.1 points wide over 55
flights, and 8.95 points over the 30 team-seasons they come from ([the note][sl-note]).

**Decision.**

1. **M2.11, fin flutter held to measurement,** joins the roadmap in Phase 1 and the queue after
   M10.3, release 0.3, so no release waits for it, and before M6.6 prints IREC's flutter speed. Its
   *done when* (the roadmap entry states it in full):
   - **The cases.** At least three distinct fins measured fluttering (wind tunnel or flight, none
     of the wings Martin's line was drawn through), from published tests by at least two research
     groups. Each fin is committed with its table and page: planform, thickness, mounting (clamped
     root or glued to a tube), material and the shear modulus used (the effective value, and a
     ply's where the source gives one), and the test's air pressure, temperature and Mach number,
     each marked measured or assumed. Repeat runs or samples of one design count as one fin.
   - **The scores.** For each fin, `validation/reports/latest.md` gives the flutter speed's error
     against its lowest measured onset (the test's speed step stated), with its sign under each
     modulus reading, and the side of Martin's line the figure 3 check places it on; a fin that
     fluttered but is placed on the safe side is a flattering error.
   - **The page.** The flutter page states the largest flattering-side error, or that the cases
     show none, with the range of Mach number and thickness ratio they cover and that they are not
     a bound. Either one case lies inside Martin's thickness ratios (1–10%) or that gap stays open
     in row G22.
   - **The tests.** A test pins each case's error and side, and a mutation probe that multiplies
     the speed by √2 fails one.
   - **The calculator.** The same fins and the calculator's own examples are compared with
     Bennett's calculator, each within 1% or the difference put down to a named modeling choice
     that a test pins.
   - **The register and the scoreboard.** Row G22's kind and what it leaves unchecked are updated
     from these. The scoreboard gains a flutter row: HPR Sim's largest flattering-side error over
     the cases, with their count, and each other tool's cell read from its current docs with the
     date, or named as a capability it lacks.
   - **Fewer cases** (fewer than three fins, or one group) only by an ADR that gives the count
     found and why no more are published, with the gap left in the report, as [ADR-236][adr-236]
     does for supersonic flights.
2. **The interval by design.** Where ADR-209 §7 and M10.3 report a paired bootstrap interval, it is
   reported twice: resampled by flight, and resampled by design, each design's held-out flights
   drawn together with both tools' errors kept paired; each interval names its number of flights
   or designs, and the collection's designs appear as aggregates only ([ADR-151][adr-151]). The
   pass rule doesn't change: the target is still HPR Sim's mean absolute error below OpenRocket's,
   with both intervals reported beside it.

The flutter idea is recorded as promoted in `docs/research/ideas.md`. It adds no user-facing
surface: the flutter output exists; this checks it.

**Consequences.** A safety number that today rests on its source's arithmetic gets a measured
error, a stated side and a stated range. A case where the criterion reads high would be found
before M6.6 prints a flutter speed for IREC teams; M6.6's *done when* already takes the speed from
its unflattering side, and would take that error with it. The scoreboard shows flutter beside the
other tools, where RocketPy's open defect is on the flattering side. Release 0.3's comparison
states how much its interval depends on a few designs, whichever way that moves it. No *done when*
is loosened, no release waits for new work, and the queue's other items keep their order.
