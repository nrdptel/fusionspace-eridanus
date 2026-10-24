# ADR-291: The bench's chamber test from Fetter's seven printed peaks, as a two-sample t-test (2026-10-24)

- **Status:** accepted; corrects [M13.7][m13-7]'s chamber pass band (issue #693), which hasn't been measured against, since M13.7 hasn't started
- **Summary:** M13.7's seven 0.5 g shots in an empty chamber are compared with the seven peaks Fetter's Fig. 7-5 lists by a two-sample t-test at 95% (pooled deviation, 12 degrees of freedom). With shots as spread as Fetter's, a mean within 28.8 ± 5.1 psi passes. The figure's printed mean and deviation, 28.49 and 4.04 psi, don't match its own peaks, so they are not used.

[m13-7]: ../decisions-and-roadmap.md#m13-7
[note]: ../research/suite-field-tools.md#ejection-testing
[i693]: https://github.com/nrdptel/fusionspace-eridanus/issues/693

**Context.** M13.7 builds a pressure logger for a sealed chamber. Its done-when checks the logger
against a published test: seven 0.5 g shots of black powder in an empty 150.8 in³ (2.47 L)
chamber, from Fetter (2025, Rev 1.2), Fig. 7-5, p. 24 ([field tools note][note]). The done-when
gave a band of 28.5 ± 4.7 psi from the figure's printed mean, 28.49 psi, and standard deviation,
4.04 psi. A re-check ([#693][i693]) found that those don't match the seven peaks the same figure
lists: 36.64, 26.78, 23.14, 31.15, 27.05, 26.34 and 30.24 psi.

- Their mean is 28.76 psi.
- 4.04 psi is their population deviation, dividing by 7. Their sample deviation, dividing by 6,
  is 4.36 psi.
- The old ±4.7 is t × 4.04 × √(2/7), with t = 2.179 at 12 degrees of freedom. With 4.36 it is
  ±5.08.

So the band was about 8% too narrow and slightly off center. That is the strict side: a correct
logger could fail it.

**Decision.** The done-when names the test the band stood for. The new shots pass when a
two-sample t-test at 95% finds no difference from the seven listed peaks. Pooling the two sets'
deviations gives 12 degrees of freedom and t = 2.179. With shots as spread as Fetter's (sample
deviation 4.36 psi), that is a mean peak within 28.8 ± 5.1 psi. Unrounded, it runs from 23.68 to
33.85 psi; rounded, from 23.7 to 33.9.

A band fixed before the shots, from Fetter's deviation alone, would have 6 degrees of freedom,
t = 2.447, and ±5.7 psi. The test is chosen over that band because it uses all 14 shots'
spread, so it is the stricter check when the new logger's shots are as steady as Fetter's.

**Consequences.** M13.7's other done-whens don't change. A logger whose seven shots scatter more
than Fetter's gets a wider pooled band, as the test intends. The figure's printed 28.49 and 4.04
are named in the done-when as not matching its peaks, so a later reader doesn't restore them.
