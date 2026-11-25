# ADR-378: MAPLEAF as a second open oracle, flown on three public designs (2026-11-24)

- **Status:** accepted; carries out [M1.14b8][m1-14b8] by the first branch of its *done when* ([ADR-351][adr-351] decision 2): MAPLEAF flies, and a committed report gives the differences; changes no number HPR Sim prints
- **Summary:** MAPLEAF 0.8.16 (MIT) at commit `af970d3` runs as an outside program in a pinned Python 3.10 environment and flies three public designs from inputs HPR Sim writes. Each difference is HPR Sim's value less MAPLEAF's. With one drag table in both codes, the apogees differ by +0.13% to +0.35% and the rail-exit speeds by +0.007% to +0.034%. With MAPLEAF's gravity also matched to HPR Sim's at the pad, the apogees are within 0.019 m (0.063 ft). With each code's own aerodynamics the apogees differ by −0.22%, +1.00% and +7.63%. Nothing is gated. MAPLEAF does not fly a leaning rail as given, so every flight leaves a vertical one.

[m1-14b8]: ../decisions-and-roadmap.md#m1-14b8
[adr-351]: 0351-m1-14b-in-eight-the-basic-finners-normal-force-first.md
[adr-023]: 0023-predicted-mode-each-codes-own-drag-reported.md
[report]: ../../validation/reports/mapleaf-flights.md
[report-matched]: ../../validation/reports/mapleaf-flights.md#with-mapleafs-gravity-matched
[report-not-shared]: ../../validation/reports/mapleaf-flights.md#what-the-two-could-not-share
[report-rail]: ../../validation/reports/mapleaf-flights.md#mapleaf-on-a-leaning-rail
[report-momentum]: ../../validation/reports/mapleaf-flights.md#with-the-internal-momentum-force-added
[flight]: ../physics/flight.md
[issue-1626]: https://github.com/nrdptel/fusionspace-eridanus/issues/1626
[mapleaf]: https://github.com/henrystoldt/MAPLEAF

**Context.** HPR Sim's whole flights are compared with two other simulators, RocketPy and
OpenRocket. [ADR-351][adr-351] decision 2 asked for a third, [MAPLEAF][mapleaf]: an open-source
(MIT) six-degree-of-freedom simulator with its own aerodynamics, written apart from both. Its
*done when* has two branches. Either MAPLEAF, at a pinned commit and run as an outside program,
flies at least three of the public designs on the inputs it can share with HPR Sim, and a
committed report gives the differences in apogee and in rail-exit speed with each input the two
could not share beside them. Or a decision record refuses it with what was measured. Before this
record MAPLEAF had not been installed or run.

This record takes the first branch. `cargo xtask mapleaf-flights` writes [the report][report],
and every number below is the report's.

**Decisions.**

1. **MAPLEAF runs in an environment of its own, pinned by a lock file.** The route MAPLEAF
   documents, `pip install MAPLEAF`, fails on Python 3.9 to 3.13 before anything is built, with
   `ModuleNotFoundError: No module named 'pkg_resources'`, and on Python 3.8 while building the
   `matplotlib==3.2.2` its requirements pin. An editable install of the source fails the same
   two ways. (Those are from a pip run on 2026-11-24 whose logs are not committed;
   `pip install MAPLEAF` in a fresh environment of each Python shows them again.) What works is
   Python 3.10 with every dependency resolved as it stood on 2022-01-15,
   five weeks after the pinned commit (2021-12-11), and MAPLEAF's `matplotlib` pin overridden
   with `>=3.5`; these flights never plot. `validation/oracles/mapleaf/pyproject.toml` states
   that, its `uv.lock` pins MAPLEAF by its commit and the 21 packages it needs by hash, and the
   environment is installed into the
   gitignored `refs/venv-mapleaf`. MAPLEAF's source is MIT; it was read to write its input files
   and to learn what its log's columns mean, and none of it is ported.

2. **HPR Sim writes the inputs.** `cargo xtask mapleaf-flights --inputs` writes
   `validation/fixtures/flight/mapleaf-inputs.json` from HPR Sim's own reading of each design:
   the outside shape and its roughness, the mass, center of mass and inertia without
   propellant, the motor's thrust, propellant flow, propellant center and propellant inertia
   through the burn, the guided travel, the site, and the height the center of mass stands at
   on the pad. MAPLEAF's rail holds the center of mass at the rail's foot until thrust lifts it,
   where HPR Sim stands the tail on the ground, so without that height MAPLEAF starts 0.9 to
   1.8 m (3.0 to 5.9 ft) lower and in denser air. A second reading of the design file, by
   hand or by script, would add its own differences to the comparison. MAPLEAF's flights carry
   the SHA-256 of the inputs they were flown from, and the report refuses flights whose hash is
   not the committed file's. After each run the oracle script reads MAPLEAF's own log back and
   refuses a case whose mass, center of mass or inertia at ignition or at burnout is not the
   inputs' to one part in a million.

3. **Three designs: the RocketPy examples with a tangent-ogive nose.** MAPLEAF's nose cone is
   a tangent ogive or a cone. The other examples' noses are shapes it could only fly as
   something else, so they are left out, not approximated.

4. **Each design flies with the same drag and with each code's own.** MAPLEAF has a part that
   flies a table of coefficients (`TabulatedAeroForce`), so drag is an input the two can share.
   In the same-drag flights both codes fly one table of drag coefficient against Mach number:
   HPR Sim's own coasting drag at no angle of attack, in the air at the site. What is left
   between them is everything but the drag. In the own-drag flights each code works its
   aerodynamics out from the same outside shape, as [predicted mode][adr-023] does against
   RocketPy.

5. **Two more flights are measurements, not MAPLEAF as it ships.** MAPLEAF's flat Earth has one
   inverse-square gravity at every latitude; HPR Sim's is WGS 84 normal gravity at the site.
   At the three pads HPR Sim's is 0.12% to 0.32% lower. To measure what that moves, each
   same-drag flight is flown once more with MAPLEAF's gravity constant scaled so that its
   gravity at the pad is HPR Sim's ([the table][report-matched]). The second measurement is
   of a term. HPR Sim's translational equation carries three internal-momentum terms beside the
   thrust, as RocketPy's does ([flight mechanics][flight]), and MAPLEAF's carries none. With the
   propellant's center fixed in the airframe they sum to one force along the axis, and each
   gravity-matched flight is flown again with that force added to MAPLEAF's thrust
   ([the table][report-momentum]). Both are labeled as measurements, not as MAPLEAF: the
   same-drag and own-drag tables fly MAPLEAF as it ships.
   MAPLEAF also has a round and a WGS 84 Earth model. Both turn with the Earth and fly in an
   inertial frame, whose log this comparison does not read; they were not flown, and the list
   of what the two could not share says so.

6. **Every flight leaves a vertical rail.** Flown on the one example rail that leans most,
   5.3° from the vertical, MAPLEAF at this commit moves the rocket up a line 10.60° from the
   vertical with its body 5.30° from it, an angle of attack of 5.30° while still guided
   ([the measurement][report-rail]). That is not the launch the input asks for, so the lean is
   an input the two cannot share, and both codes fly a rail that stands straight up. In calm
   air a vertical rail also makes MAPLEAF's lack of a tip-off phase and of rail friction cost
   nothing; HPR Sim flies with both turned off.

7. **Both numbers are read the same way in both codes.** Apogee is the rise of the center of
   mass above where it stood at ignition. Rail-exit speed is the airframe's speed where the
   airframe has traveled the guided length. The second needed care. MAPLEAF's position is its
   center of mass's, its velocity is the airframe's, and its rail lets go when the center of
   mass has traveled the rail's length. The center of mass moves forward in the airframe as
   propellant burns, so MAPLEAF frees the airframe 6.5 to 16.3 mm (0.25 to 0.65 in) short of the
   guided length.
   The oracle script therefore reads MAPLEAF's speed where its airframe has traveled all of
   it, and the report reads HPR Sim's rail-exit event at the airframe's speed, not at the
   center of mass's speed its flight summary gives.

8. **Nothing is gated.** Neither code is the truth. The report gives each difference and each
   input the two could not share ([the list][report-not-shared]) and holds neither to a
   tolerance. What is held is the record: `cargo xtask mapleaf-flights --check`, and a test
   that runs on every platform, fly HPR Sim again and fail unless the committed inputs, report
   and page reproduce. MAPLEAF itself does not run in CI. The gate's reference step, which
   runs on a machine that holds the references and MAPLEAF's environment, flies MAPLEAF again
   and fails unless its committed flights reproduce.

**What was measured.**

| flights | apogee, HPR Sim less MAPLEAF | rail-exit speed, HPR Sim less MAPLEAF |
|---|---|---|
| same drag | +0.35%, +0.13%, +0.13% | +0.034%, +0.012%, +0.007% |
| same drag, MAPLEAF's gravity matched | −0.00004%, −0.00005%, −0.00132% | −0.0017%, −0.0001%, −0.0009% |
| own drag | −0.22%, +1.00%, +7.63% | +0.028%, +0.013%, +0.008% |

The designs are in the report's order: Valetudo, Bella Lui, NDRT 2020.

- **Flight mechanics.** With the drag and the gravity both shared, the apogees are within
  0.019 m (0.063 ft) of each other and the rail-exit speeds within 0.00040 m/s (0.0013 ft/s).
  Between its 1 ms and 2 ms steps MAPLEAF's apogee in those flights moves by up to 0.030 m
  (0.099 ft), so a difference smaller than that is not resolved. Still unshared in those flights
  are HPR Sim's internal-momentum terms, the Coriolis force, the two atmosphere codes, mass
  between the motor file's rows, and the integrators. Their effects are inside that difference
  together, and it bounds none of them alone.
- **Internal-momentum terms.** Added to MAPLEAF's thrust, the force moves MAPLEAF's apogee at
  its 1 ms step by −0.014 m, −0.007 m and −0.104 m. Between MAPLEAF's two steps the flights that carry it move
  by up to 0.065 m (0.214 ft), so the measurement sizes the terms and does not settle them
  ([issue 1626][issue-1626]). At the 1 ms step the apogee differences with the force added are
  +0.014 m, +0.007 m and +0.085 m, larger than without it on all three designs. At the 2 ms step
  they are nearer than without it on Valetudo and Bella Lui and farther on NDRT 2020, so the
  direction is not resolved either. NDRT 2020's rail-exit difference at 1 ms grows from
  −0.00018 m/s to −0.00121 m/s, a change inside that flight's own step change. The one sharp
  reading is Valetudo's
  rail-exit speed, where the two codes differed most: the force moves MAPLEAF's by −0.00039 m/s,
  and the difference left is under 0.00001 m/s.
- **Gravity.** Matching it takes the same-drag apogee differences from +0.13% to +0.35% down to
  at most 0.0014% in size.
- **The air.** With the start height shared, the two codes' air density at ignition agrees to
  2.6e-9 of itself: the two atmosphere codes give the same density at that height.
- **Drag.** Just after burnout HPR Sim's axial coefficient is 0.9%, 11.0% and 25.2% below
  MAPLEAF's. The report gives it by part. MAPLEAF is higher on every part the two share, and
  it was flown with its defaults, a boundary layer turbulent from the nose among them. Which
  code's drag is nearer a measurement, this comparison cannot say.

**Consequences.**

- [The validation plan's reference list](../VALIDATION.md) marks MAPLEAF reported, with these
  numbers.
- The comparison covers three subsonic flights straight up in calm air, to apogee. It says
  nothing about wind, a leaning rail, the descent or supersonic flight, and the report's first
  paragraph says so.
- A fourth design needs a nose MAPLEAF has. A flight in wind needs MAPLEAF's wind models
  compared with HPR Sim's first. Neither is planned here.
- The own-drag gap is a lead for the drag work, not a finding against either code.
