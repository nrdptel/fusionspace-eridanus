# The effective exhaust velocity guard: why 200 to 5,000 m/s

Re-checked 2026-11-07: 13 claims corrected.

This note holds the reasoning and the measurements behind the units check in `hpr_motor`
(`EXHAUST_VELOCITY_RANGE_M_S` in `crates/hpr-motor/src/motor.rs`). The Solid motors page
(`docs/physics/motor.md`, "The effective exhaust velocity is a units check") gives the short
version. It was moved here from that page in
[M0.4e](../decisions-and-roadmap.md#m0-4e), the documentation reader test, and re-checked against
its data on 2026-11-07.

## What the check is

Both motor constructors (`SolidMotor::new` and `SolidMotor::from_envelope`) refuse a motor whose
curve and propellant mass imply an effective exhaust velocity `c = I/m_p` (total impulse over
propellant mass) outside **200 to 5,000 m/s**.

It catches a propellant mass in the wrong unit. Nothing else in the API notices one: the
411I175's envelope in millimeters and grams read as meters and kilograms
(`from_envelope(curve, 38.0, 245.0, 228.9, 437.5)`) has positive, finite dimensions, a propellant
mass below the loaded mass, and an exhaust velocity of 1.8 m/s (its file's 411.41 N·s over
228.9 "kg"). The grams are what the check sees.

It can't catch a size in the wrong unit, because `c` doesn't depend on the motor's size and
`from_envelope` accepts any finite, positive diameter and length. The I377 of the motors example
with its size left in millimeters and its masses in kilograms,
`from_envelope(curve, 38.0, 292.0, 0.250, 0.560)`, builds at 2,103 m/s (525.78 N·s over
0.250 kg), the same as with its size in meters. A size slip is caught only by the design checks
(`hpr_design::checks`), and only when the same size is given to the `MountedMotor`: a diameter
wider than the mount's bore is the error `motor_wider_than_mount`, which stops the flight unless
design errors are accepted, while a case that runs past the mount's forward end is only the
warning `motor_past_mount_top`. Nor does `c` involve the loaded mass, so a loaded mass in grams
with the propellant mass in kilograms is accepted too.

## Which impulse

`I` is the curve's own impulse, uncorrected for ambient pressure, because that is the impulse the
model actually delivers: `state()` derives the mass flow as `F(t)/c` from the same uncorrected
curve, so `∫ṁ dt = I/c = m_p` closes exactly. The bound is therefore on `c` at the curve's
reference pressure. For a sea-level-tested motor that is below its vacuum value by the ambient
pressure times the nozzle exit area, `p_a A_e`, over the thrust: the thrust equation
`F = ṁ u_e + (p_e − p_a) A_e` (NASA SP-8039, eq. 2, p. 3) with the flow held fixed. Ideal-nozzle
arithmetic (ratio of specific heats 1.2 to 1.25) puts the sea-level value 3 to 14% below the
vacuum one for chamber pressures of 3.5 to 7 MPa and expansion ratios of 3 to 8, and more at
lower chamber pressure. Either way it is far inside the factor of 1,000 the check is built to
catch.

## Where the range comes from

Chemistry sets the scale. Rather than quote a specific-impulse table hpr has not read, the range
came from the catalog itself. It was measured over the 1,708 ThrustCurve.org simulator files in
the 2026-09-17 download that have both a parsed impulse and a propellant mass in their motor's
ThrustCurve.org record (`propWeightG`). Of the 1,712 files then listed, 2 had no statistics and
2 belonged to a motor with no recorded propellant mass.

That is not the mass the bundled catalog flies. Since the catalog was rebuilt from the curve
files ([issue #295](https://github.com/nrdptel/fusionspace-eridanus/issues/295), closed
2026-10-08), each motor's index masses are its default file's header, and
[M5.7b1](https://github.com/nrdptel/fusionspace-eridanus/pull/1005) (2026-10-28) kept that when
it compiled the whole catalog; the record's mass is kept only for comparison (`stated`).
`CatalogMotor::motor` prefers the index's mass over the file's header, so in the bundle a file
other than the default flies the default's mass, and only an index from outside the bundle can
correct a header.

A percentile says how much of the list lies below a value: `p5` is the value 5% of the files fall
below, `p50` the median (half below, half above), and so on. The figures are nearest-rank.

| percentile | min | p1 | p5 | p50 | p95 | p99 | max |
|---|---|---|---|---|---|---|---|
| `c`, m/s | 236 | 597 | 928 | **1,866** | 2,210 | 2,463 | 3,031 |

So 90% of the files lie between 928 and 2,210 m/s. The bulk of that distribution is APCP
(ammonium perchlorate composite propellant). Both ends are data errors, not chemistry:

- The highest, 3,031 m/s, is a misfiled curve: a file listed under the Cesaroni 229H255-14A
  whose header names a different H255 load, with 316 N·s of impulse against the record's 104.3 g.
  Its own header mass would give 1,951 m/s.
- The lowest, 235.5 m/s, is a Cesaroni E75 whose record gives 104 g of propellant for a 52 g
  motor, which is impossible; its ThrustCurve.org page, read 2026-11-07, still shows it, with a
  user note that it should be 10.4 g.

Of the 67 files whose record names black powder, 64 lie below 900 m/s (the 67's median is 733);
the other three are an Estes C6 at 928 and two Estes E30 files at 1,887. But black powder is not
the whole low tail: 19 of the 83 files below 900 m/s are not recorded as black powder (9 of them
record no propellant type at all). The next-lowest entry, the Quest A8 at 296.8 m/s, is suspect
too: its record gives 6.3 g of propellant where NAR's combined list gives 3.6 g, which would be
519 m/s. The lowest entry whose record mass NAR's list matches is the Quest B4 at 369.2 m/s
(10.4 g in both).

The same survey over today's bundled files (the 2026-10-28 download, 1,699 files with a record
mass) gives a minimum of 235.5 (the same E75), a median of 1,865.5 and a maximum of 2,829.3 m/s
(a Cesaroni M1670): the H255 file at 3,031 is no longer listed.

## What that means for the check

**Under that convention the bound rejects none of them.** That is the honest statement of what
this check is: a guard against a slip in the units of the propellant mass, which moves `c` by a
factor of 1,000, not a filter on propellant.

It has real headroom, but not a great deal at either end. Over the record-mass survey the lowest
entry is 1.18x above the floor, but that entry is the impossible E75 record; the Quest A8 is
1.48x above it and the Quest B4, the lowest NAR backs, 1.85x. The highest, itself a misfiled
curve, is 1.65x below the ceiling. The catalog hpr
actually flies uses header masses, and there the highest is the Ellis J228 at 4,128.18 m/s, only
1.21x below the ceiling (next section).

A small motor whose recorded propellant mass is too large for its impulse could fall under the
floor. If one ever does, the fallback is [issue #11](https://github.com/nrdptel/fusionspace-eridanus/issues/11)'s
second option: apply the bound only above a couple of grams of propellant.

## Two numbers worth keeping straight

Both have been got wrong here before.

- Reading the **curve file header** mass instead gave a different distribution in the 09-17
  download (max 10,111 m/s, from the Cesaroni 836J210-16A file whose header claims 82.7 g where
  its record says 396 g; with 396 g it is 2,112 m/s). That file is no longer listed. In today's
  bundle the header-mass maximum is the Ellis J228 at 4,128.18 m/s: its header gives 270 g where
  its record gives 569 g, which would be 1,958.9 m/s. The bundled catalog flies header masses
  (above), so the J228 flies at 270 g; since issue
  [#1003](https://github.com/nrdptel/fusionspace-eridanus/issues/1003) closed (2026-10-28) a file
  more than 5% short of its record draws a warning (`CatalogMotor::propellant_warning`).
- The bundled catalog's range, 924 motors' default files with each header's propellant mass, runs
  **296.83 m/s** (the Quest A8, a black-powder A) to **4,128.18 m/s** (the Ellis J228), computed
  from each curve's own impulse. `motor::tests::every_bundled_motor_is_inside_the_range` builds
  every bundled motor and asserts both ends, so the figures can't drift. (An earlier version of
  this note quoted 689.78 to 2,651.64 m/s for the 32-motor bundle that preceded the whole
  catalog. The test then pinned 708.59 at the low end, the Estes C5 at its 11.0 g header mass;
  689.78 is the same curve over the record's 11.3 g, which a bundle from before issue #295
  appears to have used.)

## Open questions

- **Black powder's specific impulse.** No primary source was found. Issue #11's "about 80 s"
  is uncited, so the note says only what the catalog shows.
- **Dropped: "Estes and Quest count the delay grain and the ejection charge as propellant."**
  No source was found, and what was found points the other way. ThrustCurve.org's glossary
  defines the field only as "the weight of the propellant used by this motor". NAR's combined
  certified-motor list (prepared 2024-07-31) gives the Estes C6-0, -3, -5 and -7 one row at
  10.8 g, so it can't separate a booster with no delay or ejection charge from the rest; Purdue's
  Estes page gives the C6-0 booster alone 10.8 g. Neither supports the claim, so the earlier
  explanation of the low tail is withdrawn; record errors such as the E75's are documented.
- **"The stored impulse differs from the curve's by up to 0.3%".** The earlier note said this of
  the 32-motor bundle; across that bundle, the record's stated impulse and the curve's differ by
  up to 0.83% (the E26W, of the stated value). The 0.3% may describe a bundle older than the
  public history (an inference).
- **The 411I175's 9.5 mm nozzle exit** on the Solid motors page has no source here: the `.rse`
  file gives no exit diameter, and Cesaroni does not publish nozzle drawings. This note no longer
  uses it.

## Sources

- ThrustCurve.org data: the 2026-09-17 simulator-file download (survey) and the 2026-10-28
  bundle in `crates/hpr-motor/data/thrustcurve/`; motor pages for the
  [Cesaroni 25E75-17A](https://www.thrustcurve.org/motors/Cesaroni/25E75-17A/) and
  [836J210-16A](https://www.thrustcurve.org/motors/Cesaroni/836J210-16A/), read 2026-11-07.
- ThrustCurve.org glossary, "propellant weight", read 2026-11-07. Curve files are public
  domain; the motor records' terms are unstated, so only aggregates and designations are quoted.
- NASA SP-8039, *Solid Rocket Motor Performance Analysis and Prediction* (May 1971), eq. 2, p. 3;
  the vacuum-to-ambient difference `p_a A_e` is eq. 35, p. 37.
- NAR, Combined Motors by Impulse (prepared 2024-07-31; the file sits in NAR's 2024-04 folder),
  read from a cached copy.
- Purdue University, Black Powder Model Rocket Engine (Estes), read 2026-11-07:
  <https://engineering.purdue.edu/~propulsi/propulsion/rockets/solids/estes.html>.
- Issues [#11](https://github.com/nrdptel/fusionspace-eridanus/issues/11) and
  [#1003](https://github.com/nrdptel/fusionspace-eridanus/issues/1003); the code in
  `crates/hpr-motor/src/motor.rs`, `catalog.rs` and `crates/hpr-design/src/checks.rs`.
