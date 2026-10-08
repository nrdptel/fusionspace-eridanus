# ADR-059: The RocketSerializer cross-check: three readers, with OpenRocket settling a difference (2026-09-21)

- **Status:** accepted
- **Summary:** The RocketSerializer cross-check: three readers, with OpenRocket settling a difference

**Context.** M3.1d2's *done when* has two parts. Every `.ork` in `refs/loft-fixtures` and the
OpenRocket example set must import with zero errors, and "the RocketSerializer cross-check agrees
on the key geometry". RocketSerializer (MIT, RocketPy-Team) turns a `.ork` design into RocketPy's
inputs. Research for this increment found three things:
- Release 0.2.0 fails on OpenRocket 24.12.
- Its `ork2json` command needs a stored simulation, which Loft's demo designs lack, and writes
  files beside its input.
- It depends on `orhelper` (GPL-2.0), which pins JPype below 1.5.

The first run of its extractors also showed that its stations are not always OpenRocket's. Its
walk of the tree adds the lengths of the parts before each one in its parent. And it matches parts
by name, so a two-way comparison could never agree everywhere without hpr copying its mistakes.

**Decision.**

1. **Three readers, not two.** `validation/oracles/rocketserializer/geometry.py` calls
   RocketSerializer's extractors one by one, as its `ork_extractor` does:
   `process_elements_position`, `get_rocket_radius`, `search_nosecone`,
   `search_trapezoidal_fins`, `search_elliptical_fins` and `search_transitions`. For each
   component it reports, the script also asks OpenRocket 24.12, loaded on the same file and
   settled by a throwaway save (ADR-054), for the same numbers. `cargo xtask ork` holds each of
   hpr's numbers to RocketSerializer's, to 1 part in 10⁹. Where they differ, OpenRocket settles it:
   - When OpenRocket's number is hpr's, RocketSerializer is apart.
   - Otherwise hpr is apart, and the survey fails.

   And whatever RocketSerializer says, the survey fails when one of hpr's numbers is not
   OpenRocket's, or when OpenRocket draws a nose otherwise than hpr. Without that, a mistake hpr
   and RocketSerializer share (a cant OpenRocket clamps at 15°, #148) would pass as agreement.

   **This is what "agrees on the key geometry" is taken to mean.** Every number RocketSerializer
   reads as OpenRocket does, hpr reads the same; no number of hpr's is apart from both; and every
   number of hpr's is OpenRocket's.
2. **The key geometry** is what RocketSerializer reports about the airframe's shape:
   - the nose cone's shape, length and base radius, and a Haack series's parameter;
   - each transition's length and end radii;
   - each trapezoidal and elliptical fin set's count, chords, span, sweep, cant and cross-section;
   - the station of each of those parts;
   - the body radius (the largest radius written as a number).
3. **A cause is named only where the record proves it.** Two causes can be checked:
   - A station further aft by exactly the lengths of the parts before it in its parent. The
     script records that sum from OpenRocket for every part.
   - A transition given the radii OpenRocket gives the first transition of its name.

   Every other difference is counted as "no cause shown", with all three numbers kept.
4. **Two of OpenRocket's own conventions are allowed for, each measured.**
   - OpenRocket turns a canted fin about the middle of its root chord, which moves the root's front
     aft by (c/2)(1 − cos δ). For all 10 canted fin sets in the library and the jar's examples, its
     turned station is aft of its unturned one by that amount, to 1 part in 10¹¹. The script
     therefore reads a fin set's station with the cant set to zero, restores the cant, and keeps
     the turned station too. `cargo xtask ork` checks the amount for every canted fin set and fails
     if one is off.
   - An ogive of parameter 0 is a cone. OpenRocket still calls it an ogive, so a shape is settled
     by OpenRocket's profile radius a quarter, a half and three quarters of the way along.
5. **Pinned as a tool, not as a reference checkout.** RocketSerializer is pinned at `66d8ca8`, after
   release 0.2.0, with the environment it runs in:
   `validation/oracles/rocketserializer/requirements.txt`, installed with `--no-deps`. `orhelper` is
   never installed, and JPype is the oracle environment's 1.7.1. (A first run, with `orhelper`
   and JPype 1.4.1 installed, gave the same records when compared locally; that run is not kept.) During the research, about 15 lines of
   `orhelper`'s source (its function signatures) were read before its licence was checked. Nothing
   here comes from them: the script imports none of it, and it starts the JVM the way
   `automatic_radius.py` (ADR-054) already did.
6. **Public record committed, library record not.** The record for Loft's seven public demo designs
   is `validation/fixtures/ork/rocketserializer-loft-demo.json`. An `xtask` test holds hpr to it in
   CI. The library's record goes to the gitignored `corpus-out/` and is summarised as counts.
   RocketSerializer's own 23 example designs are not added to the reference library here. Adding
   them moves every count the `.ork` page quotes, and brings findings of their own, so that is
   #147.
7. **Import errors are counted per source, and held.** An import error is a file that does not
   read, or a design that reads but does not lay out. Warnings do not count, since they never stop
   an import. `cargo xtask ork` prints the count for each source, and fails if `loft-fixtures` or
   the jar's examples has any, or if a design OpenRocket opens does not lay out in hpr. It also
   fails when a RocketSerializer extractor raises, since its parts would go uncompared.

**Consequences.** On 2026-09-21:
- **Imports.** `loft-fixtures` imports 27 of 27 files and the jar's examples 17 of 17, each with 0
  errors. The only errors under `refs/` are the two Loft test fixtures that are not XML (ADR-051).
- **The cross-check.** The record covers all 78 files the survey reads, and the survey fails if a
  design it lays out is missing from it. It compares 1,212 numbers over the 74 designs OpenRocket
  opens (54 distinct files; 896 numbers counting each once):
  - 1,102 agree (813 of the 896);
  - in 110 (83), RocketSerializer is apart and hpr's number is OpenRocket's: 75 stations from its
    walk, 15 transition radii from a same-named transition, 20 with no cause shown;
  - in 0, hpr is apart from both.
  - hpr's number is OpenRocket's in all 1,212, the 1,102 agreements included, so no agreement is a
    mistake hpr and RocketSerializer share. All 72 noses match OpenRocket's profile at three points.
  - RocketSerializer's stations come from OpenRocket's tree, and it agrees with hpr on 8 of 96 fin
    stations, so stations rest on OpenRocket; what it reads from the file agrees every time.
- **Not compared:** 6 parts inside pods or parallel stages, 3 values RocketSerializer gives none
  for, and 4 files OpenRocket does not open.
- **The public record.** Six of the seven Loft designs open in OpenRocket. Of their 80 numbers, 74
  agree, and 6 fin stations are RocketSerializer's walk.

M3.1's *done when* is met.

---
