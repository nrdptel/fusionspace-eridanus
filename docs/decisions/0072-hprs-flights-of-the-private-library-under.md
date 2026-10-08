# ADR-072: hpr's flights of the private library, under anonymised ids (2026-09-25)

- **Status:** accepted
- **Summary:** hpr's flights of the private library, under anonymised ids

**Context.** ADR-070's M2.2e3 has hpr fly the private library OpenRocket flew in M2.2e2, in a
report of anonymised ids that, with the public report, holds at least 20 designs with the five
spreads: apogee, largest speed, stability margin, mass and centre of mass. Measured before
deciding:

- Of the library's 27 `.ork` files, 13 are byte for byte OpenRocket examples in the public record
  (the public report compares 4 of them and lists the rest with a reason), and two more are edited
  copies of examples. One keeps the example's rocket name, stored conditions and configuration
  ids, with a changed comment, one moved freeform-fin point and new stored flight data; the other
  keeps its configuration and component ids, with the rocket renamed and all four motor
  digests changed. That leaves 12 private designs.
- hpr flies 17 configurations of 4 of them. The public report flies 5 designs. That is 9, not 20.
- The 8 private designs hpr flies no configuration of wait on: a motor igniting in flight, which
  staging brings (M1.9), 2 designs; an airframe read simpler than written, 5 (pods 1, the single
  `overridesubcomponents` flag OpenRocket later replaced 2, fin fillets 1, an inner tube taking an
  automatic radius from a nose cone 1); a tilted launch rod, 1.
- The public record has 19 designs hpr does not fly: staging 4, clusters 2, pods, parallel stages
  and tube fins 4, hybrid motors 2 (out of scope), a motor with no curve 1; and 6 of Loft's demos,
  5 declaring a motor with no digest that neither program finds a curve for, 1 OpenRocket refuses.
  M1.9 (staging, clusters, air starts) is worth up to 6 public and 2 private designs, so 17 in all.
  The last three can come from the tilted rod (#173, 1 design), the airframe readings (#174, 5
  designs, the old override flag alone 2), or the 4 public designs held by pods and parallel
  stages (M1.13) or tube fins (#133).

**Decision.**

1. **Split M2.2e3.** Milestone ids take one increment level, so the siblings are renumbered.
   M2.2e3 is now the report and its screens, and prints the count against the bar. A new M2.2e5
   takes M2.2e3's bar word for word, at least 20 designs with the five spreads, is blocked on
   M1.9 and then on three designs from #173, #174, M1.13 or #133, and also meets M2.2e4's bar on the flights
   it adds. M2.2e4, the causes, is unchanged and next, on the flights so far; then M1.9.
2. **Anonymised ids.** A private design is `C01` to `C12` in the order of its file's SHA-256, and a
   flight `C09/2`, the design's second configuration. The report's `ids_sha256` changes when the
   ids would move, so `--check` fails and prose citing an id is looked at again.
3. **Copies.** A file that is a public design, by its bytes or its rocket, is left to the public
   report, and a private design found twice counts once: two readings of one rocket are not two
   designs. A rocket is its name with its configurations' ids, or those ids alone when OpenRocket
   made them all (random UUIDs, which a renamed rocket keeps). The name alone is never enough,
   since OpenRocket gives every new rocket one default name, which 4 of the 12 keep.
4. **Differences, never values.** The committed report holds each flight's differences (apogee,
   largest speed and mass in per cent of OpenRocket's; margin, centre of mass and centre of
   pressure in calibres), the class of OpenRocket's largest Mach number and of the launch site,
   how early OpenRocket's parachute opened, how many parts carry a drag override, what hpr's
   design checks found by kind, and each unflown configuration's reason. A value and its
   difference in per cent give back the other value, so no value is published. Unrounded, a
   difference of one floating-point step gave back a launch mass's digits, so differences are
   rounded to 6 decimals, masses to 3 (across a design's configurations they give the ratios of
   its launch masses), and the parachute time to 0.01 s. A row with a key off a fixed list, or a
   number not so rounded, stops the report; tests hold every string to a closed list per key and
   the summary to the rows it is computed from.
5. **The curve OpenRocket flew.** A configuration is compared only when every motor's curve is
   the design's own or one supplied for its digest (ADR-067), and the motor record finds
   OpenRocket placing exactly those digests there. A curve from hpr's bundled catalog is found by
   name and could be another curve, so it is named, not flown (1 configuration). A design the
   motor record does not read back is an error, not a reason. The public report keeps all 21 of
   its flights under the same test.
6. **Only a vertical rod in calm standard air.** A tilted rod is listed with its reason (3
   configurations of one design) until OpenRocket's rod direction is pinned (#173). A drag
   override that is not zero, or one on the rocket or a stage, is named as a cause with no removal
   probe, which only stands in for a zero on a part. A record lacking what a comparison needs is
   an error; a flight hpr fails is listed with that reason, the detail printed locally only.

**Consequences.** The library's 17 flights: apogee −4.84% to +1.17%, none more than 5% off;
largest speed −0.65% to +2.28%; margin −0.0008 to +0.0730 calibres; launch mass within 0.004%;
centre of mass −0.0273 to +0.0042 calibres. Two designs carry the margin gap on every flight, hpr
reading the rocket more stable: `C03` by +0.056 to +0.073 calibres, +0.061 of it the centre of
pressure, and `C09` by about +0.04, half centre of pressure and half centre of mass (#172; M2.2e4
covers apogees only). The 14 flights launched above sea level all read low in apogee and the 3 at
sea level high, but they are two designs each, and the public report's flights, all at sea level,
read low: a pattern for M2.2e4 to test, not yet a lead. With 9 designs of 20 the parent's bar is
visibly not met, in the report and the guide. `cargo xtask ork-flights --library`
needs the private library, so CI checks only the committed report's arithmetic and privacy.
