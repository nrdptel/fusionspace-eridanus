# ADR-200: The separated parts' warnings (2026-10-07)

- **Status:** accepted
- **Summary:** M10.1d5: #179 warns on every flight with a body aft of a stage boundary flying on its own (a booster, at any speed); #354 on every flight with a body that starts its own flight with nothing open, a device opening at that same instant counting as open. Both read the flight's bodies, so a drag of the flight's own keeps them.

**Context.** [ADR-190](0190-the-friction-and-freeform-fin-warnings.md) left M10.1d5 two
warnings that need a condition read from a flight's separations. After a split each part flies
on as a point mass with only its open recovery devices' drag ([ADR-014][adr-014]). Two open
issues follow from that, and `CHANGELOG.md`'s known gaps listed both as flattering the drift with
no warning:

- [#179](https://github.com/nrdptel/hpr-sim/issues/179): a booster tumbles side-on from the
  split, where a real one coasts nose-first on its airframe's drag first. When the issue was
  filed, the library's two-stage example's booster left at 341.2 m/s and climbed only to
  745.0 m. Its peak and its landing point are likely too low and too close. The size is not
  measured.
- [#354](https://github.com/nrdptel/hpr-sim/issues/354): a part whose device has not opened by
  the split has no drag at all until it opens. M4.6a refuses the climbing case (a fired device's
  lag on a part parted before apogee), and the flight refuses a part that lands with nothing open,
  so what is left is mostly a part already falling. It lands early and fast, and the wind carries
  it less far. The size is not measured.

**Decision.**

1. **#179 warns on every flight with a separated body whose first stage is past the nose's**
   (stage 0): a booster dropped at a separation, or a parallel stage. A body all in the nose's
   stage, as an ejection's pieces are, doesn't meet it. The error is unsized, so no speed at the
   split is excluded, the way ADR-190 took in #18's unsized range. The message says the booster's
   peak and drift *likely* read short, the issue's own word: the sign holds where it tumbles from
   the split, as every `.ork` booster does. A booster with nothing open at the split flies with no
   drag, so its peak reads high; it also meets #354, and #179's message points there.
2. **#354 warns on every flight with a body that starts its own flight with no drag area open**,
   unless a device opens at that same instant. The start sample is taken just before the devices
   act at the split's instant, so a device fired by the split with no lag reads a drag area of
   zero there; it shows as a `Deployment` event at the start's time, and that counts as open. A
   canopy with a fill time, opened on the stack at the split, also starts at zero and warns: an
   over-warning, on the safe side, left for library users (a `.ork` device opens at once). A NaN
   area or event time counts as closed, so a broken number can't hide the warning. The message
   gives both sides: falling, the part likely lands early and drifts short (a part moving faster
   than the wind across it keeps that speed with no drag, so its drift can read long); climbing
   (which the library still flies when the device's trigger, not its lag, comes after the split),
   its peak reads high.
3. **Both read the flight's bodies, not its drag,** in `separated_part_issue_warnings`, so a
   flight on a drag table or model of its own meets them too, as a part's descent never takes the
   rocket's drag. They print as `warning: drag:` lines (`kind` `"drag"`): the error is a missing
   drag. They name no parts and no Mach number. Each warns once per flight, however many bodies
   meet it.

**Consequences.** Every staged flight `hpr sim` flies from a `.ork` file now prints
`warning: drag: issue #179`. `hpr sim` can't meet #354 today: a `.ork` payload's delayed device is
refused before the flight. A Monte Carlo run's drawn lag can make one, but `hpr mc` reports only
the nominal flight's warnings, so a sample that meets #354 isn't warned; the library's `Flight`
warns. Python flies no separations yet (M4.6b). `CHANGELOG.md` marks both warned. A model of a
headless body's drag, the fix #179 asks for, would retire both warnings for the booster.

[adr-014]: https://github.com/nrdptel/hpr-sim/blob/main/docs/DECISIONS.md#adr-014-separation-bodies-their-masses-and-their-descents-2026-09-17
