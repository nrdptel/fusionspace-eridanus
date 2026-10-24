# ADR-291: The bench's chamber band from Fetter's seven printed peaks (2026-10-24)

- **Status:** accepted; corrects [M13.7][m13-7]'s chamber pass band ([issue #693][i693]); M13.7 hasn't started, so nothing was measured against the old band
- **Summary:** The ground-test bench's seven 0.5 g black-powder shots in an empty chamber must give a mean peak from 23.68 to 33.85 psi (28.76 ± 5.08), the band from the seven peaks in Fetter's 2025 ejection-testing paper (Fig. 7-5) and their sample deviation; a correct logger lands inside about 93% of the time. The figure's printed mean and deviation, 28.49 and 4.04 psi, don't match those peaks. The band is fixed, so a logger whose shots scatter more doesn't get an easier test.

[m13-7]: ../decisions-and-roadmap.md#m13-7
[note]: ../research/suite-field-tools.md#ejection-testing
[i693]: https://github.com/nrdptel/fusionspace-eridanus/issues/693

**Context.** M13.7 builds a pressure logger for a sealed chamber. Its done-when checks the logger
against a published test: seven shots of 0.5 g of FFFFg (a very fine grade of black powder) in an
empty 150.8 in³ (2.47 L) chamber, from Fetter (2025, Rev 1.2), Fig. 7-5, p. 24 ([field tools
note][note]). The done-when gave a band of 28.5 ± 4.7 psi from the figure's printed mean, 28.49
psi, and standard deviation, 4.04 psi. A re-check ([#693][i693]) found that those don't match the
seven peaks the same figure lists: 36.64, 26.78, 23.14, 31.15, 27.05, 26.34 and 30.24 psi.

- Their mean is 28.76 psi.
- 4.04 psi is their population deviation: it divides by 7 and describes only these seven shots.
  Their sample deviation divides by 6 and estimates the spread of all such shots: 4.36 psi.
- A new set of seven shots, read by a correct logger, has a mean that differs from Fetter's by
  chance. That difference has a deviation of s × √(2/7), since each mean carries s × √(1/7). The
  old band was t × s × √(2/7), with t = 2.179, Student's t at 95% for 12 degrees of freedom
  (7 + 7 − 2), as a two-sample t-test takes it. With the printed 4.04 that gives the old ±4.7;
  with 4.36, ±5.08.

So the correct band is 8% wider than the old one, and its center is 0.27 psi above the printed
28.49. The old band erred on the strict side: a correct logger could fail it.

**Decision.** M13.7 passes a mean peak from 23.68 to 33.85 psi: 28.76 ± 5.08, computed from the
seven listed peaks and their sample deviation, rounded to 0.01 psi. The edges decide, not the
rounded "about 28.8 ± 5.1", whose top edge would be 33.9. The band is fixed before the shots.
The logger's seven shots have their sample deviation recorded beside the mean.

The band is not quite a 95% band. Its s comes from Fetter's seven shots alone, which carry 6
degrees of freedom, so a correct logger with Fetter's scatter lands inside it with probability
P(|t₆| < 2.179) = 0.928, about 93%. That is the chance before either set of shots is drawn,
counting the chance in Fetter's own seven. The band keeps that 7% chance of a false fail on
purpose: a true 95% band is wider and passes a logger that reads high more easily. A correct
logger that fails has the done-when's own way out, the gap recorded in an ADR with the
measurement.

Two other rules were weighed and refused:

- **A pooled two-sample t-test**, whose band uses the new shots' spread too. It widens as the
  new logger scatters more: seven shots with a sample deviation of 10 psi would pass a mean of
  37 psi, 29% high. A logger that reads high leads to charges sized too small, the worst class
  of error here.
- **A true 95% band from Fetter's deviation alone**, t = 2.447 for 6 degrees of freedom, ±5.7
  psi. It is wider, so it passes a logger that reads high more easily.

**Consequences.** M13.7's other done-whens don't change. A logger whose shots scatter far more
than Fetter's 4.36 psi still has to meet the band, and its recorded deviation shows the scatter.
The figure's printed 28.49 and 4.04 are named in the done-when as not matching its peaks, so a
later reader doesn't restore them.
