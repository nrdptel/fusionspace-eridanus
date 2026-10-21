# The magnetic field and declination

Use the World Magnetic Model's declination to turn a compass reading at your launch site into the
true heading you give HPR Sim.

## In short

- **What it models:** the Earth's main magnetic field at any place and time from 2025.0 to
  2030.0, from the World Magnetic Model (WMM2025). Its most useful output is the
  **declination**: the angle between true north and the north a compass shows. Nothing in HPR Sim
  applies it for you. A rail's heading (`hpr sim --heading`, the builder's `heading_deg`) is a
  true bearing. Add the declination to a compass reading first ([A worked
  example](#a-worked-example) shows how).
- **Sources:** A. Chulliat, W. Brown, M. Nair and others, *The US/UK World Magnetic Model for
  2025–2030: Technical Report*, NOAA NCEI (2025). With it, NCEI's coefficient file
  `WMM2025.COF`, which NCEI places in the public domain.
- **How well it is validated:** the simulator reproduces all 12 of the report's test points (its
  Table 6) to their last printed digit. Declination is held to 0.005°. NCEI's file of 100 points is
  printed to 1e-6 nT and angles to 0.01°. It matches to its last digit in declination,
  inclination, the east component and four rates. Its north component differs at 97 points, by
  up to 7.18e-4 nT (2.11e-8 of the total field). The horizontal and total intensities differ with
  it. An independent check in the tests does not use HPR Sim's derivative formula. It places that
  difference in NCEI's file, not in HPR Sim ([The test values](#the-test-values)).
  That shows the model is computed correctly. The model itself is only as good as the Earth
  allows. Its own error estimate for declination is 0.29° at best and 0.35° to 0.56° at the
  example's sites, more near the magnetic poles. Each result carries that estimate.
- **What it leaves out:** local magnetic rocks, magnetic storms and the steel near a compass.
  Dates outside 2025.0 to 2030.0 are refused, and so are heights below −1 km (−0.62 mi) or above 850 km (528 mi)
  ([Limits](#limits)).

## Sources

Code: [`hpr_core::magnetic`](../api/hpr_core/magnetic/index.html). Conventions:
[Frames](frames.md) and [Geodesy](geodesy.md). Each source below is pinned by its checksum in the
reference library ([`validation/refs.lock.toml`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/validation/refs.lock.toml)),
under the name given. DLMF is a web reference.

- **[WMM]** A. Chulliat, W. Brown, M. Nair, N. Gomez Perez, L.-Y. Young, C. Watson, N. Boneh,
  C. Beggan, B. Meyer and M. Paniccia. *The US/UK World Magnetic Model for 2025–2030: Technical
  Report*. National Centers for Environmental Information, NOAA, 2025,
  <https://doi.org/10.25923/prbc-s316>. Section 1.2 gives the equations, 1.4 the poles, 1.8 the
  blackout zones, 3.4 the error model. Pinned as `wmm2025-report`.
- **[COF]** NCEI, `WMM2025COF.zip` (2024-12-17): the coefficient file `WMM2025.COF` and the test
  values `WMM2025_TestValues.txt`, committed unchanged in
  [`crates/hpr-core/data/wmm2025/`](https://github.com/nrdptel/fusionspace-eridanus/tree/main/crates/hpr-core/data/wmm2025),
  with the report's Table 6 as `WMM2025_TEST_VALUES.txt`. Pinned as `wmm2025-coefficients`, and
  Table 6 as `wmm2025-test-values`.
- **[DLMF]** NIST Digital Library of Mathematical Functions, equation
  [14.10.3](https://dlmf.nist.gov/14.10.E3): the recurrence used for the Legendre functions.

## Declination

What declination is, how the World Magnetic Model computes it, and a worked example.

### What declination is

A compass needle lines up with the horizontal part of the Earth's magnetic field. That points to
magnetic north, which is not true north (the direction of the North Pole along the ground). The
angle from true north to magnetic north is the **declination** $D$. It is positive when magnetic
north is east of true north.

So a bearing read off a compass becomes a true bearing by adding the declination:

$$
\begin{aligned}
&\text{true bearing} \\
&\quad = \text{magnetic bearing} + D
\end{aligned}
$$

At Spaceport America in mid-2026, $D$ is +7.75°: a rail aimed at a compass's north points 7.75°
east of true north. The simulator's flight engine takes headings as true bearings, clockwise from
true north ([Frames](frames.md)). So a heading measured with a compass needs this correction
first.

The field also dips into the ground. The field's **inclination** $I$ (not the rail's inclination)
is that dip, positive downward. It is about 60° across the southern United States, and 90° at a
magnetic pole, where a compass has nothing horizontal to follow.

Field strengths are in nanoteslas (nT). The Earth's field is 20,000 to 70,000 nT at the surface.

### The model

The WMM writes the field as a sum of **spherical harmonics**: patterns over the globe that get
finer as their **degree** $n$ rises. The degree runs from 1 (one north and one south pole, like a
bar magnet) to 12. Each degree is split into **orders** $m$ from 0 to $n$. That makes 90 terms.

Each term has two **Gauss coefficients**, $g$ and $h$ in nT ($h$ is zero where $m = 0$), which set
how strong the pattern is. Each coefficient changes linearly in time from its 2025.0 value, at the rate $\dot{g}$ or $\dot{h}$
([WMM] eq. 9):

$$
g(t) = g(2025.0) + (t - 2025.0)\, \dot{g}
$$

where $t$ is the decimal year (2026.5 is the middle of 2026). The simulator computes the field the
way [WMM] section 1.2 sets out:

1. The site's latitude $\varphi$, longitude $\lambda$ and height above the WGS 84 ellipsoid become
   geocentric coordinates (eqs. 7, 8). They are a radius $r$ and a geocentric latitude $\varphi'$, the
   angle seen from the Earth's center.
2. There the field has three parts (eqs. 10 to 12): north $X'$, east $Y'$ and down $Z'$. Each is a
   sum over degree $n$ and order $m$, with $a = 6{,}371{,}200\,\mathrm{m}$:

   $$
   \begin{aligned}
   X' &= -\sum_n \left(\frac{a}{r}\right)^{n+2} \\
   &\qquad \times \sum_m \frac{\mathrm{d}\breve{P}(\sin\varphi')}{\mathrm{d}\varphi'} \\
   &\qquad \times (g \cos m\lambda \\
   &\qquad\quad + h \sin m\lambda), \\
   Y' &= \sum_n \left(\frac{a}{r}\right)^{n+2} \\
   &\qquad \times \sum_m \frac{m\, \breve{P}(\sin\varphi')}{\cos\varphi'} \\
   &\qquad \times (g \sin m\lambda \\
   &\qquad\quad - h \cos m\lambda), \\
   Z' &= -\sum_n (n+1) \left(\frac{a}{r}\right)^{n+2} \\
   &\qquad \times \sum_m \breve{P}(\sin\varphi') \\
   &\qquad \times (g \cos m\lambda \\
   &\qquad\quad + h \sin m\lambda)
   \end{aligned}
   $$

   $\breve{P}$ are the Schmidt semi-normalized associated Legendre functions ([WMM] eq. 5).
3. The parts turn into the local frame (eq. 17) by the angle $\varphi' - \varphi$ between the two latitudes.
   That gives $X$, $Y$, $Z$ in the site's north, east and down directions.
4. The elements follow (eq. 19): horizontal intensity $H = \sqrt{X^2 + Y^2}$, total intensity
   $F = \sqrt{H^2 + Z^2}$, inclination $I = \operatorname{atan2}(Z, H)$ and declination $D = \operatorname{atan2}(Y, X)$.

The same sums over the coefficients' rates $\dot{g}$ and $\dot{h}$ give how fast each part changes per year
(eqs. 13 to 15, 18 and 20). Rates are written with a dot: $\dot{X}$, $\dot{D}$ and so on (`dD` in the
example).

Two details differ from the report's printed text, and NOAA's test values settle both:

- Equation 11 and the derivative in equation 16 divide by $\cos\varphi'$, which is zero at a pole.
  The simulator writes each function as $\cos^m\varphi'$ times a polynomial in $\sin\varphi'$, so the division
  cancels exactly. The field at a pole is the limit along the meridian given ([WMM]
  section 1.4). A test checks the report's printed field over each pole.
- Equation 15 prints the rate of $Z'$ with $\dot{g} \cos m\lambda - \dot{h} \sin m\lambda$. The potential
  gives $+$, as in equation 12, and only $+$ reproduces NOAA's rates.

#### Grid variation

Near the geographic poles, declination swings with every step east or west. So polar navigators
use **grid variation** instead ([WMM] eq. 1). It is the angle from a map grid's north, that of the
polar stereographic grid (Table 6's note), to magnetic north. It is $D - \lambda$ north of 55° N, and $D + \lambda$
south of 55° S. Elsewhere the simulator gives none (`None`), as the test values print `NaN`.

#### How far a compass can be trusted

The report marks a **blackout zone** around each magnetic pole, where the horizontal intensity $H$
is under 2,000 nT. There declination can be wrong by up to 180°. Around it is a **caution zone**,
under 6,000 nT, where declination errors exceed 1° ([WMM] section 1.8).

`MagneticField::compass_zone` says which applies: `Reliable`, `Caution` or `Blackout`. The report
draws the zones on the ground. The simulator uses the field at the height asked, which is a little
weaker higher up, so its zones there are a little wider.

The report's error model ([WMM] eq. 43) gives the declination's expected error, one standard
deviation, in degrees, with $H$ in nT:

$$
\delta D = \sqrt{0.26^2 + (5417/H)^2}
$$

That is 0.29° where the horizontal field is strongest and grows without bound toward a magnetic
pole. `MagneticField::declination_uncertainty_rad` returns it. It covers the coefficients'
errors, the drift of the forecast to 2030, local rocks the model leaves out, and magnetic storms.
It does not cover a steel rail, a car or a motor case beside the compass.

### A worked example

The example program
[`crates/hpr/examples/declination.rs`](https://github.com/nrdptel/fusionspace-eridanus/blob/main/crates/hpr/examples/declination.rs)
asks for the field at four launch sites on 20 June 2026. It turns the date into a decimal year
with `decimal_year` (the start of that day, 2026 + 170/365). Then it calls `WMM2025.field` with
each site's latitude, longitude and height above the ellipsoid. Run it from a copy of the repository
with `cargo run --example declination -p fusionspace-hpr`. It prints:

<!-- quote: crates/hpr/examples/declination.output.txt -->
```text
WMM-2025 on 2026-06-20 (decimal year 2026.4658)

site                             D (°)  ± (°)  dD (°/yr)   I (°)  F (nT)  GV (°)  zone
Spaceport America, New Mexico     7.75   0.35     -0.076   59.89   46981       -  Reliable
Black Rock Desert, Nevada        12.85   0.36     -0.096   64.24   49646       -  Reliable
Lucerne Dry Lake, California     11.16   0.35     -0.079   59.38   46407       -  Reliable
Andøya, Norway                    9.41   0.56      0.268   78.16   53674   -6.61  Reliable

At Spaceport America, New Mexico, a rail aimed at magnetic north points 7.75° east of true north.
```

`±` is the model's own error estimate, `dD` the declination's drift per year, `GV` the grid
variation (only at Andøya, north of 55°). The drift is small: Spaceport America's declination
falls by about 0.08° a year, so it moves well under a degree over the model's five years.

## Tests and limits

The model's checks, and where it stops applying.

### The test values

| check | points | what is compared | worst difference | held to |
|---|---|---|---|---|
| the coefficient table | 90 rows | every coefficient against `WMM2025.COF` | none | equal |
| the report's Table 6 | 12 | $X$, $Y$, $Z$, $H$, $F$, their rates; $I$, $D$, grid variation, their rates | half the last digit | 0.05 nT, 0.005° |
| the report's Table 3b | 1 | $\varphi'$, $r$, the coefficients, $X'$, $Y'$, $Z'$, $X$ … $\dot{D}$, printed to 10 decimals | 5.0e-11 nT on $X'$ | half the last digit, plus 8 units in the last place |
| the report's poles (section 1.4) | 2 | $X$, $Y$, $Z$ over each pole at $r = a$ | inside 0.05 nT | 0.05 nT |
| NCEI's high-precision file, printed to 1e-6 nT and angles to 0.01° | 100 | $Y$, $D$, $I$, and the rates of $Y$, $Z$, $D$, $I$ | half the last digit | 5e-7 nT, 0.005° |
| the same file | 100 | $X$, $H$, $F$ | 7.18e-4 nT | 7.2e-4 nT, the measured worst |
| the same file | 100 | $Z$; the rates of $X$, $H$, $F$ | 2.2e-6 nT; 1.5e-6 nT/yr | the measured worst |
| the potential, by differences | 100 | HPR Sim's $X'$ and $\dot{X}'$ | 1.3e-7 nT; $\dot{X}'$ within the rounding allowance (under 1e-9 nT/yr) | 1e-6 nT; 1e-6 nT/yr |

Rows are counted from 0. The north component $X$ in NCEI's file differs from HPR Sim's at 97 of its
100 points. The largest is 7.18e-4 nT (row 35: 2026.5, 12 km (7.5 mi), 33° N, 145° W).
That is at most 2.11e-8 of the total field. It is 140 times smaller than the 0.1 nT the report
allows for single precision (the note under its Table 6).

The tests show where the difference comes from:

- It lies in one quantity, the geocentric $X'$. If the file's $X'$ is off by an amount $e$,
  its $X$ moves by $e \cos(\varphi' - \varphi)$ and its $Z$ by $e \sin(\varphi' - \varphi)$. Taking $e$ from each point's $X$
  and applying it to $Z$ brings the file's $Z$ to within 4.9e-7 nT of HPR Sim's, inside its
  printing.
- HPR Sim's $X'$ is right. The tests take the potential's derivative in latitude by differences,
  independently of HPR Sim's formula for it. HPR Sim's $X'$ matches that to 1.3e-7 nT at all 100
  points. The report's own ten-decimal $X'$ (Table 3b) matches to 5e-11 nT.
- A second program agrees with HPR Sim: pygeomag 1.1.0 (MIT), a port of NOAA's own `geomag`
  program. Run once by hand on two of the points (rows 3 and 35, not in CI), it gives HPR Sim's $X$
  to 7e-7 nT. So it differs from the file by the same amount.

The rate of $X$ differs too, by up to 9.5e-7 nT a year at 24 points: a second, smaller residue
whose cause is not known. HPR Sim's $\dot{X}'$ passes the same derivative check. The rates of $H$ and $F$
follow from $X$ and its rate.

So the file's $X$, $H$, $F$ and $Z$ are held to the measured differences, not to the file's
printing. So are the rates of $X$, $H$ and $F$. [The WMM2025 decision][adr-125] records the choice. For a flight
the difference is immaterial: 7.18e-4 nT turns the declination by about a millionth of a degree.

Every comparison with Table 6 and NCEI's file first allows 64 units in the last place of the
point's total field, about 7e-10 nT. The allowance covers the rounding in both programs' sums and is
about 700 times below half the finest printed digit.

A property test also checks, at random places and times, that $H$, $F$, $I$ and $D$ agree with
$X$, $Y$, $Z$ by their definitions. At the same places and times it checks that $F$ lies between 20,000 and 70,000 nT. The
report's Table 1 rounds the surface range to 23,000 to 67,000 nT. But the model itself falls to
about 21,900 nT over South America by 2030, which a test pins at 26° S, 61° W.

### Limits

- WMM2025 covers only five years, 2025.0 to 2030.0. A flight log from 2024 needs WMM2020, which
  the simulator does not bundle. Such a date is refused, not extrapolated.
- The model is specified from −1 km (−0.62 mi) to 850 km (528 mi) above the WGS 84 ellipsoid
  ([WMM] section 3). Outside that, a height is refused.
- Heights are above the WGS 84 ellipsoid, as `Geodetic` holds them. A site's height above sea
  level (from a map, a GPS or an altimeter) differs by the geoid, up to about 100 m (328 ft). The
  report puts that effect at about 1 nT or less, far below the model's error. So a height above
  sea level can be used as it is.
- The WMM models the main field only. It leaves out the crust's local fields, which can move a
  compass by degrees near iron ore or volcanic rock. It also leaves out the fields of magnetic
  storms. The error estimate allows for them as a worldwide average, but at any one site they can
  be larger.
- Nothing in a flight uses it yet. Converting a compass heading is the caller's step, with
  `MagneticField::true_from_magnetic_rad`.

[adr-125]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0125-site-data-split-and-wmm2025-in-hpr-core.md
