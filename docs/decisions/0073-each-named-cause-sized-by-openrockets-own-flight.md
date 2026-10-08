# ADR-073: Each named cause sized by OpenRocket's own flight without it (2026-09-25)

- **Status:** accepted; a cause in the drag sized by ADR-097
- **Summary:** Each named cause sized by OpenRocket's own flight without it

**Context.** M2.2e4's bar is a written hypothesis for every apogee more than 5% from
OpenRocket's. The public report has five; the private report none. ADR-069 named two causes, an
early parachute (both *C6-3* flights, +12.19% and +13.80%) and a part stated to have no drag that
hpr does not apply (#165; the three *Base drag hack* flights, −15.47% to −19.13%). Loft excused its
largest misses with known issues (L82), so a name alone is not a hypothesis. The evidence then was
indirect: another configuration's flight for the parachute (C6-7), and hpr flown with the part
removed, which also takes its lift and shape, for the override. On the two flights with both
causes that probe overshot, and the split was not measured.

**Decision.**

1. **Size a cause by OpenRocket flying without it.** `flights.py` flies every configuration again
   with nothing deployed (as it did for the optimum delay) and now records that flight's apogee.
   Where a part states a drag coefficient, it flies a third time with nothing deployed and every
   stated coefficient cleared, which is OpenRocket flying what hpr flies (hpr reads the setting and
   applies none); and where every such part states zero and is neither the rocket nor a stage, a
   fourth time with those parts removed, the rocket hpr's removal probe flies. Each takes the
   summary's apogee, as the flight's own is taken, and records the ids of the parts it changed.
   `cargo xtask ork-flights` compares hpr's apogee with each and prints what the causes leave. The
   record must name the same parts hpr reads, or the report stops; a flight OpenRocket refused or
   aborted, or the oracle failed on, is shown with its reason and sizes nothing.
2. **What counts.** An apogee more than 5% off is brought within the bar by its named causes when
   every flight of OpenRocket's with all of them taken out, the cleared setting and the removed
   part alike, is within the same 5% of hpr's. The 5% is M2.2's own threshold, not a new one. The
   strictest comparison decides, because two differences can cancel in one of them. It is a
   code-to-code statement about OpenRocket's flight, not a claim that hpr would match with the
   cause modelled: hpr still flies no `.ork` parachute and applies no drag override.
3. **Both ways.** A test holds that over the record's 56 finished flights, the undeployed apogee
   equals the flown one to the bit on the 41 whose parachute opens at or after apogee, is higher
   on 14 of the 15 where it opens before, and on the 15th (0.10 s early) differs by 1 mm, within
   the 3 mm the deployment event alone can move the peak of rows 0.05 s apart. It also holds each
   flight's summary apogee equal to its altitude column's peak. Another test recomputes every
   sized number and count from the record.
4. **The library.** Its record is written by the same script, so it was flown again; its
   rows are unchanged, none of its 17 flights is more than 5% off, and M2.2e5 adds these columns to
   it when it has one.

**Consequences.**

- The bar is met: each of the five has a written hypothesis with its size. Four are brought within
  5%. With nothing deployed the *C6-3* flights read −0.16% and −1.07%. On the *Base drag hack*,
  holding the parachute alone gives −16.77%, −20.80% and −20.50%, since the early parachute and
  the ignored setting pull opposite ways. With the setting cleared too they read +1.34%, +3.51%
  and +4.79%, and with the cone removed from both programs +1.15%, +4.76% and +7.80%.
- The E12-4 is not brought within 5%. Its +4.79% rests on a cancellation: the cone costs hpr 84.0 m
  of apogee and OpenRocket 71.1 m. Like for like it is +7.80%.
- What is left on the *Base drag hack* is not explained, and it has the other sign from the 12
  flights with no named cause (−0.06% to −4.34%). It is not the cone's own drag, since it stays
  with the cone removed from both. A same-state comparison of the two programs' drag by part, a
  scratch probe that is not committed, suggests two candidates, neither sized in apogee. One is the
  nose: OpenRocket charges this blunt ellipsoid nose (length 0.58 diameters) subsonic pressure
  drag, 0.0117 at Mach 0.1 to 0.0482 at Mach 0.25 on the 4.87e-3 m² cross-section. hpr charges it
  none below Mach 0.8, where hpr joins Stoney's ellipsoid curve (measured from Mach 1.2) to 0, and
  eq. B.9 keeps that 0 at any fineness. The other is the base while the motor burns: hpr takes the
  motor's area off it (0.1100 against 0.1213 at Mach 0.1), OpenRocket 24.12 does not. Which
  program is right is not measured; #177 holds it.
- Flying the corpus again moved no row of the private report. From one run to the next of the same
  jar and Java, OpenRocket's figures for six library designs moved in the 13th to 16th digit, and
  one apogee by 5e-7 of itself; the public record came back bit for bit.
