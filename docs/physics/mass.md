# Mass properties of components

See how HPR Sim finds each part's mass, center of mass and inertia from its shape and material, and
sums them for the rocket.

## In short

- **What it models:** the mass, center of mass and inertia of each part, and their sum. The parts
  are tubes, rings, shoulders, fins and their fillets, rail buttons, lugs, mass components and
  recovery gear, with every tube of a cluster. There are 49 built-in material densities.
- **Sources:** Meriam and Kraige's *Engineering Mechanics: Dynamics*, the *OpenRocket technical
  documentation* v13.05 and Abbott and von Doenhoff's *Theory of Wing Sections*. Golub and Van
  Loan's *Matrix Computations* gives the method for the principal moments. Densities come from
  data sheets, specifications and handbooks.
- **How well it is validated:** by analytic tests, the first of four [kinds of evidence][levels].
  A cone, a tube, four fins and an off-axis payload agree with hand calculation to 1e-11. Fin
  cross-sections agree with exact numerical integration to 1e-13. Density unit conversions
  reproduce their sources, such as the *Wood Handbook*'s white ash at 678 kg/m³. Against
  [OpenRocket](../glossary.md#openrocket) 24.12, on the structure (the rocket without motors) of
  71 compared designs, the mass is within 1% on 70. The center of mass is within 1% of the
  rocket's length on 70. The one file outside either has a named cause: airfoil fins, which
  OpenRocket weighs by a factor. Fin fillets agree with OpenRocket's to 1e-15 in mass and center of
  mass on nine probe designs. The probes' pitch inertia is up to 0.638% apart
  ([below](#fin-fillets)). The roll inertia is a median 1.619% apart, and that is explained.
  OpenRocket takes a shortcut for fins that HPR Sim does not. HPR Sim's figure is the exact one for
  the fin as drawn ([below](#fins-rail-buttons-and-roll-inertia)). On the four files with a
  cluster (two designs by content), OpenRocket also stacks the tubes on the cluster's axis
  ([below](#clusters-and-fillets)). A ring of tube fins departs twice. For it, OpenRocket's roll
  inertia is more than any mass inside the ring could have. OpenRocket's pitch inertia leaves out
  how far the tubes sit from the axis. HPR Sim keeps its own for both ([below](#tube-fins)). The
  pitch inertia is within 1% on 58. Of the 13 outside, the two copies of OpenRocket's tube fin
  example are the tube fin departure, and the rest have no named cause yet. Not compared with
  weighed parts or a real flight ([checked against OpenRocket](#checked-against-openrocket)).
- **What it leaves out:** the sliver between a flat fin root and the round tube, and the step ring
  at a nose shoulder. Fillets are weighed, but the aerodynamics leaves them out
  ([Drag limits](aero.md#drag-limits)). Parachutes weigh as flat circular canopies. Where a `.ork`
  file leaves something unsaid (a wall of no thickness, no material), HPR Sim reads it as
  OpenRocket does. Two rules for overrides stay HPR Sim's own, each measured
  ([below](#what-a-ork-leaves-unsaid-and-overrides)). Fin sections are the simulator's own too. An
  airfoil fin weighs 0.6851 of a square slab of its outline, where OpenRocket's weighs 0.85. So
  HPR Sim's airfoil fins are 19.4% lighter, with no warning
  ([below](#fins-rail-buttons-and-roll-inertia)). Packed recovery gear and mass components are
  OpenRocket's too, down to the size one takes when its file writes none ([below](#packed-parts)).
  A cluster's inertia differs from OpenRocket's on purpose. OpenRocket stacks the tubes on the
  axis, and HPR Sim weighs each where it sits ([below](#clusters-and-fillets)). Some designs have
  parts the simulator does not read, such as a parallel stage on a rocket of several stages.
  They're retained as reduced designs
  ([the format guide](../format/ork.md#what-the-simulator-keeps-for-writing-the-file-back)), and
  none of the 71 compared is one. A parallel stage HPR Sim reads is weighed as its own stage
  ([Parallel stages](design.md#parallel-stages)).

## Code and sources

Code: `hpr_design::mass` (`MassProperties`), `hpr_design::parts`, `hpr_design::fins`,
`hpr_design::material`, `hpr_design::materials`. The nose and transition solids are in
[Shapes](shapes.md).

Sources:

- **[MK]** J. L. Meriam and L. G. Kraige, *Engineering Mechanics: Dynamics*, appendix B (standard
  solids' moments of inertia, the parallel-axis theorem and the inertia tensor).
- **[GVL]** G. H. Golub and C. F. Van Loan, *Matrix Computations*, 4th ed. (2013), §8.5 (the
  Jacobi eigenvalue method).
- **[TD]** S. Niskanen, *OpenRocket technical documentation* v13.05 (2013), §3.2.2, pp. 25–29 (fin
  planforms), §3.4.4, pp. 49–50 (cross-sections), §4.2.3, p. 66 and Table 5.1, p. 75 (component
  masses), pinned as `openrocket-techdoc-13.05`.
- **[AvD]** I. H. Abbott and A. E. von Doenhoff, *Theory of Wing Sections*, Dover (1959), eq. 6.2
  (NACA four-digit thickness distribution).

## The model

The conventions, the solids and fins each part is weighed as, and the materials.

### Frames and conventions

These conventions were set in [the decision on component geometry and mass properties][adr-006].

- Body axes follow [Frames](frames.md): $z$ along the axis toward the nose, $x$ the zero radial
  direction, $y = z \times x$. Roll angles run from $x$ toward $y$.
- A component's frame has body axes, with its origin on the axis at the component's forward end,
  or at a nose cone's tip. So the component lies at $z \le 0$. The design tree places it by
  translating it to its station ([Design tree](design.md)).
- `MassProperties` holds the mass, the center of mass in body axes, and the **full** inertia
  tensor about the center of mass. The tensor is taken with the positive products-of-inertia
  convention: $I = \int (\lvert\mathbf{r}\rvert^2 E - \mathbf{r}\mathbf{r}^{\mathrm{T}})\,\mathrm{d}m$, so $I_{xy} = -\int x y\,\mathrm{d}m$.
- The operations follow [MK]:
  - Parallel axis:

    $$
    \begin{aligned}
    I_p &= I_{\mathrm{cg}} \\
    &\quad + m(\lvert\mathbf{d}\rvert^2 E \\
    &\qquad - \mathbf{d}\mathbf{d}^{\mathrm{T}}),
    \end{aligned}
    $$

    with $\mathbf{d} = \mathbf{cg} - \mathbf{p}$.
  - Rotation: $\mathbf{cg}' = R\,\mathbf{cg}$, $I' = R I R^{\mathrm{T}}$.
  - Combination: sum the masses, mass-weight the centers, and sum each tensor moved to the common
    center.
  - Zero total mass gives the plain average of the centers, so placeholders stay finite.
- `validate` requires a finite, non-negative mass, and a tensor that is symmetric (to 1e-9 of its
  largest entry) with non-negative principal moments obeying $I_1 + I_2 \ge I_3$. That
  condition is the same as $J = \tfrac12\operatorname{tr}(I)\,E - I = \int \mathbf{r}\mathbf{r}^{\mathrm{T}}\,\mathrm{d}m$ being positive semidefinite. Principal
  moments come from cyclic Jacobi ([GVL] algorithm 8.5.1). It's accurate for repeated eigenvalues,
  where the closed-form trigonometric method loses $\sqrt{\varepsilon}$.

### Standard solids ([MK])

- **Hollow cylinder**, radii $R \ge r$, length $L$: $I_{\mathrm{axis}} = m(R^2 + r^2)/2$ and

  $$
  \begin{aligned}
  I_{\mathrm{across}} &= m \frac{R^2 + r^2}{4} \\
  &\quad + m \frac{L^2}{12}.
  \end{aligned}
  $$

  This covers body tubes, inner tubes and couplers, centering
  rings, bulkheads ($r = 0$), launch lugs, tube fins, and shoulders.
  - $r = R$ is allowed. It's a tube of **no wall**: mass $\pi(R^2 - r^2)L\rho$ is exactly zero, and so
    is the tensor. That is a real thing for a design to say. An imported `.ork` says it of twelve
    parts ([`.ork` design files](../format/ork.md#what-is-left-out-and-why)). Refusing it would
    force a reader to invent a wall instead. A **centering ring** is the exception. A bore that
    reaches the rim leaves no ring at all, so `CenteringRing` refuses $r \ge R$ rather than
    weighing nothing in silence. `Wall::Shell` also still refuses a zero thickness. A solid of
    revolution says "filled" with `Wall::Filled`, so a zero there is a mistake, not a statement.
  - Loft used $mL^2/12$ with no radial term, and no roll inertia at all ([a Loft lesson](../decisions-and-roadmap.md#l44)).
- **Solid cylinder**, radius $a$, height $h$: $I_{\mathrm{axis}} = m a^2/2$ and
  $I_{\mathrm{across}} = m(3a^2 + h^2)/12$.
  This covers mass components, packed parachutes, streamers and shock cords, and each disc of a
  rail button. [TD] Table 5.1 treats recovery parts as cylinders too.
- **Rail button:**
  - Three coaxial discs stacked outward on a radial line: base (outer diameter), waist (inner
    diameter), flange (outer diameter).
  - The waist height is the total height less the base and flange.
  - The screw's head, when there is one (`screw_height_m`, 0 for none), is half a solid ellipsoid
    of revolution on top of the flange. It's as wide as the button ($a$, half the outer diameter)
    and the screw's height $h$ tall. Its volume is $\tfrac23\pi a^2 h$, and its center of mass is
    $3h/8$ above the flange. Its inertia is $\tfrac25 m a^2$ about its own axis and
    $m(a^2/5 + 19h^2/320)$ across it through its center. The last is half the ellipsoid's
    $m(a^2 + h^2)/5$ about its base's diameter, moved by the parallel-axis theorem. With $a = h$
    these are a solid hemisphere's textbook $3R/8$ and $\tfrac25 mR^2$. The head carries no drag, and
    OpenRocket's comparison is [below](#fins-rail-buttons-and-roll-inertia).
  - Buttons and lugs may repeat along the axis (`count`, `spacing_m`).
- **Shoulder:** a hollow cylinder beyond the profile's end. A capped shoulder adds a disc of its
  inner radius and wall thickness, flush with its far end. The step ring between a nose's base
  radius and its shoulder is not modeled.
- **Parachute:** $m = \rho_s\pi D^2/4 + n\ell\rho_l$, the nominal area of a flat circular canopy plus its
  shroud lines. A conical or hemispherical canopy has more cloth than $\pi D^2/4$. Give its mass
  through an override ([Design tree](design.md)) or a matching nominal diameter.
- **Streamer:** $\rho_s \times \text{length} \times \text{width}$. **Shock cord:** $\rho_l \times \text{length}$.

### Fins

- **Planforms** ([TD] §3.2.2):
  - Trapezoidal: root chord $c_r$, tip chord $c_t$ parallel to the body, span $s$, and sweep $x_t$
    from the root's leading edge to the tip's.
  - Elliptical: $c(h) = c_r\sqrt{1 - (h/s)^2}$, centered on the root chord (implied by [TD] eq. 3.71).
  - Freeform: a simple polygon from the root leading edge $[0, 0]$ to the root trailing edge
    $[c_r, h_r]$, closed along the root through its root points. On a body tube the root is level,
    $h_r = 0$ with no root points. On a nose cone or a transition it follows the surface
    ([the decision on a fin root along the surface](https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0166-a-fin-root-that-follows-the-body.md)).
    The heights are measured from the radius $R_b$ at the root leading edge. Each root point lies
    within a micron of the surface. Crossing edges and points below the root leading edge are
    errors. So are outlines that turn below their root or don't run from the origin aft. The test
    draws the cockpit of OpenRocket's *Pods-\-airframes and winglets* through OpenRocket's own root
    points. Its area, mass and center of mass are OpenRocket's to the six digits printed
    (`fins::tests::a_root_along_a_nose_cone_is_openrockets`).
- **Cross-sections.** Each chord from $a$ to $b$ has a thickness distribution $t(x)$. [TD] uses
  the cross-section for drag only. HPR Sim also counts the volume it removes.
  - **Square:** $t(x) = t$.
  - **Rounded:** semicircular edges of radius $a_r = \min(t, c)/2$, so a chord shorter than $t$
    near a pointed tip is a disc of diameter $c$. Its moments are closed forms in $a_r$, for the
    removed edge material

    $$
    \begin{aligned}
    D_0 &= a_r^2(2 - \pi/2), \\
    D_1 &= a_r D_0 - a_r^3/3, \\
    D_2 &= a_r^2 D_0 - \pi a_r^4/8,
    \end{aligned}
    $$

    and $E_0 = 8a_r^4 - 3\pi a_r^4/2$ for $\int t^3$. A wide rounded chord loses
    $(1 - \pi/4)\,t^2$ of section area.
  - **Airfoil:** $t(x) = 10\,t\,P(\xi)$ with the NACA four-digit polynomial ([AvD])

    $$
    \begin{aligned}
    P &= 0.2969\sqrt{\xi} \\
    &\quad - 0.1260\xi \\
    &\quad - 0.3516\xi^2 \\
    &\quad + 0.2843\xi^3 \\
    &\quad - 0.1015\xi^4.
    \end{aligned}
    $$

    - Its maximum is $1.0003\,t$ at $\xi = 0.2998$.
    - Its moments $10\int \xi^k P$ = 0.685083, 0.288033, 0.158919 hold term by term, and
      mpmath gives $0.4728895$ for $1000\int P^3$.
    - An airfoiled fin weighs 68.5% of the square slab, with its centroid at 42% chord.
    - A hand-sanded "airfoil" is between the two. [TD] doesn't define the section.
- **Integrals.** Per fin, over the span $h$ with $r = R_b + h$, and chordwise moments
  $M_k = \int x^k t\,\mathrm{d}x$ and $T = \int t^3/12\,\mathrm{d}x$, taken per unit density:

  $$
  \begin{aligned}
  V &= \int M_0, \\
  \int r &= \int r M_0, \\
  \int r^2 &= \int r^2 M_0, \\
  \int x &= \int M_1, \\
  \int x^2 &= \int M_2, \\
  \int r x &= \int r M_1, \\
  \int \tau^2 &= \int T.
  \end{aligned}
  $$

  The span integration is split at every vertex height and runs adaptively.
  - With the fin at roll 0 (points at $(r, \tau, -x)$):
    $I_{xx} = \int(\tau^2 + x^2)$, $I_{yy} = \int(r^2 + x^2)$, $I_{zz} = \int(r^2 + \tau^2)$ and $I_{xz} = \int r x$, all $\mathrm{d}m$.
  - Loft ignored the span ([a Loft lesson](../decisions-and-roadmap.md#l44)) and fixed a freeform
    fin's CG at $0.42\,c_r$ ([another](../decisions-and-roadmap.md#l45)).
- Tabs are square slabs below the root, $-h_{\mathrm{tab}} \le h \le 0$, with closed-form integrals. A tab
  is weighed over the length its design states, even where it starts ahead of the root or ends
  past it, as OpenRocket weighs one. OpenRocket trims two tabs to the root on loading, where the
  file's rounded numbers pass it by under 0.03 mm (0.0012 in); HPR Sim keeps them as stated, a
  slab too small to show in a mass. A tab must meet the root over some length and reach no deeper
  than the body radius ([a tab weighed as stated][adr-309]). Loft never
  read them ([a Loft lesson](../decisions-and-roadmap.md#l46)).
- The flat root is placed at radius $R_b$. The sliver between it and the curved tube,
  $t^2/(8R_b)$ deep, is ignored. Fillets are solids of their own
  ([Clusters and fillets](#clusters-and-fillets)).
- Cant $\delta$ turns each fin and its tab about the fin's outward span axis through the root
  mid-chord, right-handed. So a positive cant turns fin 0's leading edge toward $-y_B$
  (`positive_cant_turns_the_leading_edge_toward_negative_y`). [TD] doesn't state the pivot. Mass
  and trace are unchanged. The products of inertia in the fin's own frame grow as $\sin 2\delta$.
- Sets roll the fin to $\varphi_k = \varphi_0 + 2\pi k/N$ and combine. Three or more fins are isotropic across
  the axis. One or two are not, and the full tensor keeps the difference.

### Materials

`Density` is `bulk` (kg/m³), `surface` (kg/m²) or `line` (kg/m). A part asking for the wrong kind
is an error. A design stores the values, not a library key. The built-in values and their sources
are in `hpr_design::materials` and summarized below.

`hpr_design::materials::BUILTIN` holds 49 materials. Each carries its source (with table or page),
the URL it was read from, and a basis:

- **published:** the source states the value.
- **derived:** computed from the source's numbers.
- **maximum:** a specification's upper weight limit.
- **vendor:** a seller's figure, used where no specification exists.

| group | values | sources | basis |
|---|---|---|---|
| hobby tubes | cardboard 790, kraft phenolic 950, Blue Tube 1,250, Quantum 1,090 kg/m³ | LOC Precision and Public Missiles weight tables (mass over wall volume); Always Ready Rocketry's own material file | derived; published |
| composites | G10/FR-4 1,800, filament-wound E-glass 1,990, carbon/epoxy 1,580 kg/m³ | Norplex-Micarta NP130, Comptec, Hexcel HexPly 8552 data sheets | published |
| metals | Al 6061 2,700, Al 7075 2,800, steel 7,850, Ti-6Al-4V 4,430, brass 8,500 kg/m³ | Kaiser Aluminum, MIL-HDBK-5J, TIMET, Copper Development Association | published |
| woods | balsa 180; basswood, yellow birch, Sitka spruce, eastern white pine, sugar maple, northern red oak from $G_{12}$; birch plywood 680 kg/m³ | Wood Handbook FPL-GTR-190 (pinned as `fpl-gtr-190-wood-handbook`), p. 2-21 and Table 5-3a; Riga Wood Plywood Handbook | published; derived |
| plastics | PLA 1,240, ABS 1,040, PETG 1,270, nylon 6/6 1,140, PC 1,200, PMMA 1,190, acetal 1,420, PS 1,040, PVC 1,400, HDPE 955, epoxy 1,180, Depron 40, paper 755 kg/m³ | manufacturers' data sheets (NatureWorks, SABIC, Eastman, Celanese, Covestro, Röhm, AmSty, Charlotte Pipe, Chevron Phillips, West System, Depron, HP) | published (paper derived) |
| fabrics | ripstop 1.1 and 1.6 oz/yd², Mylar and LDPE film at 1 mil, paper 80 g/m², Nomex cloth, silnylon | MIL-C-7020H, DuPont Teijin, Dow, HP, MIL-C-83429B; a seller for silnylon | maximum, derived, published, vendor |
| cords | nylon cord types I and III, tubular nylon ½", 9/16", 1", ⅛" and ¼" Kevlar, ¼" bungee, Tex 80 Kevlar thread | MIL-C-5040H, MIL-W-5625K, MIL-C-5651D, A-A-55220; Giant Leap Rocketry's measurements for Kevlar | maximum, vendor, published |

- Wood at 12% moisture is $\rho = 1000\,G_{12}\,(1.12)$ (Wood Handbook eq. 4-12). The handbook's own
  example, white ash at $G_{12} = 0.605$, gives 678 kg/m³.
- Specification maxima overstate typical cloth and webbing. Giant Leap's measured 9/16"
  tubular nylon is 12% under MIL-W-5625K's limit.
- The openrocket-database (Apache-2.0) was used only as a cross-check. Two problems turned up in it:
  - Its ripstop weights use 31 g/m² per oz/yd² (the factor is 33.906), so they are 8.6% low.
  - Its "Plywood, aircraft" at 337–361 kg/m³ is lite-ply, not birch.

## Compared with OpenRocket

How the masses compare with OpenRocket's, and each rule where the two differ or a file is silent.

### Checked against OpenRocket

**In short.** HPR Sim adds a design's parts into its *structure*: every stage together, with no
motor. OpenRocket 24.12 computes the same thing. On September 21, 2026 the two were
[compared](../decisions-and-roadmap.md#m2-2a) on every file OpenRocket opens among the project's `.ork` test files. One set is the *reference library*: designs
gathered under `refs/`, many of them private files other people shared. The other is the 17
example designs that ship inside OpenRocket's program file (its Java *jar*). The current default
survey compares 71 designs. Some hold the same design found in two places (several private files
are copies of OpenRocket's examples), so there are 51 different files by content. Mass and center
of mass agree closely on most, and every file outside 1% has a named cause. The roll inertia is a
median 1.619% apart. That's OpenRocket's shortcut for fins
([below](#fins-rail-buttons-and-roll-inertia)) and, on the cluster designs, its stacking of their
tubes on the axis ([below](#clusters-and-fillets)). [The decision on this comparison][adr-060] records how it was
decided. The numbers below are from the current scratch-excluding rerun after HPR Sim
[weighed every tube of a cluster](../decisions-and-roadmap.md#m1-9b)
([below](#clusters-and-fillets)). Before that, [the step on clusters and fillets](../decisions-and-roadmap.md#m2-2b4)
settled two more of the survey's causes ([next section](#what-a-ork-leaves-unsaid-and-overrides)).

**What you can check yourself.** The private files are not public, so only counts come from them.
A fresh clone can't reproduce the 71-file table. It can check the probe tube and Loft's public
demo designs (`cargo test -p xtask ork_mass`). It can run the script on `.ork` files of its
own to see OpenRocket's numbers (*Run it yourself*, at the end of this section). Comparing
HPR Sim's with them is not automated yet.

**How.** `validation/oracles/openrocket/mass.py` runs OpenRocket and asks it for each design's
structure. It saves the design once first, so that every automatic dimension is the one OpenRocket
settles on. `cargo xtask ork` compares HPR Sim's with it:

- the mass, relative to OpenRocket's;
- the center of mass's [station](../glossary.md#station), as a share of the rocket's length;
- the roll inertia (about the rocket's axis) and the pitch inertia (about an axis across it). Each
  is about the program's own center of mass and relative to OpenRocket's. Pitch is taken as the
  mean of the two inertias across the axis. That mean doesn't depend on how either program turns
  its axes about the rocket's length.

Which of OpenRocket's numbers is roll was measured, not assumed. The script first reads a probe,
one tube of a material at 1,000 kg/m³. It's 1 m (3.3 ft) long and 50 mm (2.0 in) in outer radius, with a 2 mm (0.079 in) wall.
Worked by hand, it weighs 0.61575 kg. Its roll inertia is $m(r_o^2 + r_i^2)/2$ = 0.0014790 kg·m²
and its pitch inertia about its middle is $m((r_o^2 + r_i^2)/4 + L^2/12)$ = 0.052052 kg·m².
OpenRocket's numbers match to 15 digits, and so does HPR Sim's layout of the same file (the test
`the_probe_tube_is_the_one_worked_by_hand`).

The thresholds, 1% of the mass and 1% of the length, were set before any design was measured. A
design outside either needs a written reason, not a pass.

**The results**, as `cargo xtask ork` printed them on October 5, 2026, after HPR Sim
[read parallel stages](../decisions-and-roadmap.md#m4-5m).

| | within 0.1% | within 1% | median |
|---|---|---|--:|
| mass | 64 of 71 | 70 of 71 | 0.001% |
| center of mass (share of length) | 67 of 71 | 70 of 71 | 0.000% |
| pitch inertia | 41 of 71 | 58 of 71 | 0.065% |
| roll inertia | 10 of 71 | 31 of 71 | 1.619% |
| roll inertia, OpenRocket's fin shortcut in HPR Sim's place | 59 of 71 | 59 of 71 | 0.001% |

Counting each file's content once, 50 of 51 are within 1% in mass and 50 of 51 in center of mass.
On September 28, 2026, after HPR Sim [read tube fins sized from the body](../decisions-and-roadmap.md#m2-2e8),
60 and 68 of 71 were within 0.1% and 1% in mass. In center of mass, 65 and 68 were. Within 1%, 56
were in pitch inertia and 57 in roll inertia with the shortcut, at a median 1.686% without it.
Before tube fins were read, 58 and 66 of 71 were within 0.1% and 1% in mass, and 63 and 66 in
center of mass. Before fillets were weighed, 65 of 71 were within 1% in mass, 55 in pitch inertia, and 56 in
roll inertia with the shortcut.
Before HPR Sim [read every tube](../decisions-and-roadmap.md#m1-9b) of a
[cluster](../glossary.md#cluster), 58 and 59 of 71 were.
Before it [read what a `.ork` leaves unsaid](../decisions-and-roadmap.md#m2-2b1), 57 of 74 were
within 1% in mass and 58 in center of mass (median mass 0.020%). Those are the earlier
74-file measurement. The current default survey is the 71-file table above.

**The 1 file outside a threshold** is outside both, and has one cause. `cargo xtask ork` works the
causes out and counts them by content, as below. It fails if a file outside has none:

| cause | what HPR Sim does | what OpenRocket does | files by content |
|---|---|---|--:|
| airfoil fin sections | integrates the airfoil's section, 0.6851 of a square slab ([below](#fins-rail-buttons-and-roll-inertia)) | weighs the outline times the thickness times 0.85 | 1 |

The fin-section cause is sized, not only present. HPR Sim gives no warning for it, since the section
is HPR Sim's own choice. So `cargo xtask ork` weighs the design again with its rounded and airfoil fins
weighed OpenRocket's way: square, at 0.99 or 0.85 of their density. It names the cause only if the
design then comes within both thresholds. The one design it names is a private one: `C06` in
HPR Sim's flights of the private designs. There HPR Sim's center of mass sits forward of
OpenRocket's, and the same fins are a likely cause. But the survey's thresholds are coarser than
that flight's gap. So the fins are a lead for it, not its size
([the format guide](../format/ork.md#the-simulators-flights-of-the-private-designs)).

Five causes are gone. A cluster read as one tube went when HPR Sim
[read every tube of a cluster](../decisions-and-roadmap.md#m1-9b). The cause's two files by content
are now within both thresholds. Pods went when HPR Sim [read them](../decisions-and-roadmap.md#m1-13b).
Fin fillets, which HPR Sim left out, went when it [weighed them](../decisions-and-roadmap.md#m2-2e7)
([below](#clusters-and-fillets)). Tube fins, which the simulator left out when OpenRocket sizes
them from the body, went when it [read them](../decisions-and-roadmap.md#m2-2e8)
([below](#tube-fins)). OpenRocket's *Tube fin rocket*, the one design with them, is now within
5.0e-6 of OpenRocket's mass. Its center of mass is within 2.4e-6 of its length. Parallel stages,
which the simulator kept unread, went when it [read them](../decisions-and-roadmap.md#m4-5m)
([Parallel stages](design.md#parallel-stages)). OpenRocket's *Parallel booster staging*, in the
jar and in the library's copy, is now within 3.3e-6 of OpenRocket's mass. Its center of mass is
within 1.3e-6 of its length. `cargo xtask ork` prints the file outside with the parts that differ
most, by id, or by name in an older file that writes no ids. A private design is named only by the
start of its file's hash.

**A worked example, now settled.** In [the first comparison](../decisions-and-roadmap.md#m2-2a)
the OpenRocket jar's *Two stage high power rocket* was 18.74% heavier in HPR Sim. It was 1.956 kg
in OpenRocket, and 0.3666 kg more in HPR Sim. All of it was the nose cone. Its shoulder is written
with a radius of 49.28 mm (1.9 in), a length of 50.8 mm (2.0 in) and a wall thickness of 0. HPR Sim read that as solid: a
cylinder of $\pi \times 0.04928^2 \times 0.0508$ = 3.875e-4 m³ of the file's own material, polypropylene at 946 kg/m³,
weighs 0.3666 kg. OpenRocket gives the same shoulder no mass, and so has HPR Sim since it
[read what a `.ork` leaves unsaid](../decisions-and-roadmap.md#m2-2b1). So the file is within 1% in
both mass and center of mass. Two causes the first comparison counted are gone the same way. One is
this shoulder of no wall (6 files by content), the other a part written with no material (1).

**Two more conventions.**

- Under a mass override, the inertia is a departure kept on purpose
  ([below](#what-a-ork-leaves-unsaid-and-overrides)). Loft's public `stage-weighed.ork` overrides
  its stage to 1.234 kg on 0.614 kg of parts, a ratio of 2.009. HPR Sim scales the stage's inertia
  by it and OpenRocket does not. So HPR Sim's pitch inertia is +100.9% apart and its roll +108.6%,
  the largest inertia differences measured.
- HPR Sim's airfoil fin section weighs less than OpenRocket's. The CONTROL fins of the jar's
  *Simulation scripting* example are 0.0378 kg in HPR Sim and 0.0469 kg in OpenRocket, 19.4%
  lighter, with no warning. It is a departure kept on purpose
  ([below](#fins-rail-buttons-and-roll-inertia)), and the cause of the one private design above.

**Roll and pitch inertia.** On the six Loft demo designs OpenRocket opens, the roll inertia is 1.2%
to 3.8% apart. Yet their mass, center of mass and pitch inertia agree within 0.1%, and every part
of each is within 0.3 g of OpenRocket's. Across all 71 compared designs the median is 1.619%. It
is the fins. OpenRocket takes a shortcut for a fin set's roll inertia, and HPR Sim integrates the
fin exactly ([below](#fins-rail-buttons-and-roll-inertia)). With OpenRocket's shortcut in HPR Sim's
place, the median is 0.001% and 59 files are within 1%. The shortcut takes OpenRocket's own mass
for each fin set, paired by id, or in an older file by name. So the way OpenRocket weighs a section
(below) is set aside too. Five of the six Loft demos come within 0.0005%. The sixth, whose fins are
elliptical, is 0.093% apart in that row. It's within 0.0002% once OpenRocket's ellipse is drawn as
OpenRocket draws it, a 30-sided polygon (a test). For each of the 12 files still outside 1% (8 by
content), `cargo xtask ork` names a cause, and it fails if it can't. Each has exactly one:

| cause | files by content |
|---|--:|
| a mass override covering the parts inside (a departure, [below](#what-a-ork-leaves-unsaid-and-overrides)) | 5 |
| tube fins, whose roll inertia is a departure ([below](#tube-fins)) | 1 |
| a cluster's tubes, which OpenRocket weighs stacked on the cluster's axis ([below](#clusters-and-fillets)) | 2 |

The cluster cause is sized, not only present. The survey names it only when HPR Sim's roll inertia,
less the spread of the clusters' own tubes, is within 1% of OpenRocket's. The two are +1.01% and
+2.08% apart, and +0.04% and +0.00% without the spread.

The pitch inertia is within 1% on 58 of 71. Two of the 13 outside are the two copies of OpenRocket's
*Tube fin rocket*, 1.98% below. OpenRocket's pitch rule for tube fins accounts for that gap
([below](#tube-fins)). The other 11 have no named cause yet, and no bound is known. On the fin
probes below, pitch differs by up to 0.41% where the fins weigh the same.

**What it leaves out.** Motors: this is the structure alone, and a motor's mass is left to
[the milestone on the motors OpenRocket flies](../decisions-and-roadmap.md#m2-2c). Only the design's selected
[configuration](../glossary.md#configuration) (the one OpenRocket opens it with) is weighed. The 4
files OpenRocket 24.12 does not open are not compared.

**Run it yourself.** CI does not run OpenRocket. It holds HPR Sim to OpenRocket's saved answers for
Loft's seven public demo designs, `validation/fixtures/ork/openrocket-mass-loft-demo.json`, with
`cargo test -p xtask ork_mass`. OpenRocket opens six of the seven. On those six HPR Sim is within
0.1% in mass, center of mass and pitch inertia, and every part of each within 0.3 g. With
Java 17 and the OpenRocket jar (`cargo xtask refs fetch`), from the repository root:

```sh
refs/venv/bin/python validation/oracles/openrocket/mass.py \
    validation/fixtures/ork/openrocket-mass-loft-demo.json validation/fixtures/ork/loft-demo
refs/venv/bin/python validation/oracles/openrocket/mass.py corpus-out/openrocket-mass.json \
    refs refs/hpr-sim-fixtures/imported/loft-fixtures refs/hpr-sim-fixtures/imported/debrief-fixtures --jar
cargo xtask ork
```

The script takes any directory of `.ork` files in place of `refs`. `cargo xtask ork` compares
HPR Sim with the record of the reference library only. The two `imported/` directories hold the
private design library and Debrief's private files, copied into the private flight collection.
They are named because the script doesn't follow the symbolic link the local setup makes to that
collection. With the collection as a real directory under `refs/`, the script would read all of
it, and `cargo xtask ork` refuses the record.

[adr-309]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0309-a-fin-tab-weighed-as-stated.md
[adr-060]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0060-m2-2-split-and-the-structures-mass-held-to.md

### What a `.ork` leaves unsaid, and overrides

**In short.** A design file does not say everything. What does a nose cone's shoulder written with
a wall thickness of 0 weigh? What is a part that names no material made of? When a part and the
parts inside it both have an [override](../glossary.md#override) (a mass or center of mass the
designer typed in), which wins? OpenRocket, which writes these files, has an answer to each. Its
answer is what the file means to the person who wrote it. So this project
[asks it](../decisions-and-roadmap.md#m2-2b1), with 32 small *probe designs*, each a rocket of a few
parts built to ask one question. `validation/oracles/openrocket/conventions.py` writes them. It runs OpenRocket 24.12 on all of them and records its answers. The test module
`hpr_validate::openrocket::tests` reads the same designs with HPR Sim and holds it to them. Where
HPR Sim keeps a rule of its own, the test pins how far apart the two are. The decisions are in
[the decision on what a `.ork` leaves unsaid][adr-061].

> **How far to trust it.** Each probe asks about one kind of part at one size, so each reading is
> measured, not proven for every case. OpenRocket's defaults were read with its preferences as a
> fresh install sets them. An OpenRocket whose preferences were changed may give others.

**Read as OpenRocket reads it.** On every probe of the readings in this table, HPR Sim's mass is
OpenRocket's within 0.001%, part by part as well as whole. The worst, a transition, is 0.0003%
apart. HPR Sim's center of mass is OpenRocket's within 0.001 mm (0.000039 in). Where no fin, rail button or
recovery part is in the probe, the inertias agree within 0.001% too. One gap is pinned rather than
hidden, and described below: an elliptical fin set weighs 0.18% more. None of these readings
raises a warning. Where the file writes no value, or a wall of 0, HPR Sim takes what
OpenRocket 24.12 takes, as measured on its probes. The table below lists each. One case no probe
reaches: a part whose radius is no more than the 2 mm (0.079 in) default wall, with no thickness
written. It's taken as filled, as a stated wall that thick fills it.

| the file says | what it weighs (OpenRocket 24.12, and now HPR Sim) |
|---|---|
| a nose cone, transition or body tube with a wall of 0 | nothing: the part keeps its shape (HPR Sim's drag uses the shape, not the wall) but has no wall; a part meant to be solid is written `filled` |
| an inner tube, coupler or launch lug with a wall of 0 | nothing |
| a shoulder with a wall of 0, or none written | nothing, whether or not the file closes its end with a cap, on a hollow nose or a filled one |
| a filled nose cone with a walled shoulder | the solid cone plus the shoulder's own wall |
| a nose cone, transition or body tube with no thickness written | a 2 mm (0.079 in) wall, whatever its radius past that (measured on a nose cone and a tube at 50 mm (2.0 in) and at 30 mm (1.2 in), and on a transition) |
| a part weighed by its volume (a nose, transition, tube, coupler, engine block, fin set, ring or lug) with no material | cardboard, 680 kg/m³ |
| a canopy or streamer with no material | ripstop nylon, 0.067 kg/m² |
| shroud lines or a shock cord with no material | a 2 mm (0.079 in) elastic cord, 0.0018 kg/m |
| a rail button with no material | Delrin, 1,420 kg/m³ |

A worked example with the probe's numbers: a conical nose cone 0.3 m (0.98 ft) long on a 50 mm (2.0 in) base. It has a
2 mm (0.079 in) wall of a material at 1,000 kg/m³, and weighs 0.091726 kg. With a shoulder 100 mm (3.9 in) long, 48 mm (1.9 in) in
radius and a 2 mm (0.079 in) wall, it weighs 0.150787 kg. With the same shoulder written with a wall of 0, it
weighs 0.091726 kg again in both programs. That's the probe *a nose whose shoulder has no wall*.
Before this step HPR Sim read that shoulder as solid, and would have added $\pi \times 0.048^2 \times 0.1 \times 1000$ =
0.724 kg.

**Which override wins.** An override is a number the designer typed in place of what the parts add
up to, usually after weighing the real thing. The file can say that an override *covers the parts
inside*: that the number is for the part together with everything attached to it. The probes find
HPR Sim and OpenRocket agree on which override wins, and on where a center is measured from:

- An override on a part that covers the parts inside it wins over any of theirs, and a stage's wins
  over everything in the stage.
- A center-of-gravity override is measured from the part's front, not from its shoulder's, and the
  shoulder moves with the part.
- A center-of-gravity override alone, written to cover the parts inside, sets the whole assembly's
  center. (The two place the parts inside differently, which shows only in the inertia: below.)

This settles [a Loft lesson](../decisions-and-roadmap.md#l51). Loft's rule for this came from
OpenRocket's source and was unsettled by up to 133 mm (5.2 in). The test is
`override_precedence_matches_oracle`.

**Where the simulator keeps its own rule.** Each of these is a *departure*: HPR Sim knowingly
differs from OpenRocket, and a test pins by how much.

| when | HPR Sim | OpenRocket | apart on the probe |
|---|---|---|---|
| a mass override covers the parts inside and states no center | keeps the center the parts lay out | puts it at the overriding part's own, ignoring where the parts inside sit | HPR Sim's center 3.7 mm (0.15 in) forward of OpenRocket's, or 19.7 mm (0.78 in) if the part inside has an override of its own |
| a mass override covers more than one part | scales the inertia of everything it covers by the override's ratio | scales only the overriding part's own inertia, and keeps the parts inside at theirs; a stage, having none of its own, scales nothing | roll inertia 6.9% to 37% lower in HPR Sim under a tube's; 2.5 to 5.0 times OpenRocket's under a stage's |
| a center override covers the parts inside | moves the whole assembly, so the inertia about the new center is the assembly's own | moves the overriding part alone, and adds the parts inside where they were | the center agrees; pitch inertia 2.65% lower in HPR Sim |

Under a tube's covering override HPR Sim's roll inertia is the *lower* one, though it scales more of
the parts. OpenRocket keeps the inertia of the parts inside while leaving their mass out of the
total. So its assembly carries inertia for mass it does not count.

Why keep them? A builder who weighs a tube with its fins and motor mount inside has not moved their
center. So the center the parts lay out is the better estimate. And scaling the inertia with the
mass keeps it consistent with the mass: the extra weight sits where the parts' weight does.
Neither rule is right for every rocket (a heavy avionics bay at the center of mass adds little
inertia). On a single part, with nothing inside, the two programs agree.

One more difference can't be said in the HPR design format. It's a part that overrides both its
mass and its center, with one covering the parts inside and the other not. HPR Sim scopes a part's
overrides once, takes the mass's, and warns. On the probes the center is 4.7 mm (0.19 in) apart when the
center's override is the covering one. The center agrees when the mass's is, with pitch inertia
8.2% lower in HPR Sim.

**Two gaps the probes found** were both [settled later](../decisions-and-roadmap.md#m2-2b2)
([next section](#fins-rail-buttons-and-roll-inertia)). A rail button sat 5 mm (0.20 in) further aft in HPR Sim
than in OpenRocket, and is now where OpenRocket puts it. OpenRocket's elliptical fin weighs 0.18%
less than HPR Sim's exact ellipse. OpenRocket's matches, to 13 digits, a 30-sided polygon drawn
inside the ellipse at equal angles. The simulator keeps the ellipse.

**What it leaves out.** An inner tube, coupler or lug that writes no thickness at all is read as
no wall, with a warning. OpenRocket gives it a wall of its own. On the probe a 20 mm (0.79 in) inner tube gets
0.5 mm (0.020 in). A 5 mm (0.20 in) lug gets 1 mm (0.039 in), and a coupler none. One size each doesn't say whether that
wall follows the radius, and no file in the reference library has one. So HPR Sim doesn't follow it
yet. The test pins the difference, −2.4% in the mass of that probe's structure.

**Run it yourself.** With Java 17 and the OpenRocket jar (`cargo xtask refs fetch`), from the
repository root:

```sh
refs/venv/bin/python validation/oracles/openrocket/conventions.py \
    validation/fixtures/ork/openrocket-conventions.json
cargo test -p hpr-validate openrocket
```

[adr-061]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0061-what-a-ork-leaves-unsaid-read-as-openrocket.md

### Fins, rail buttons and roll inertia

**In short.** A rocket's roll inertia is its resistance to spinning about its long axis. HPR Sim's
was a median 2.351% from OpenRocket's on the 71 compared designs, and nothing explained it. It is
the fins. OpenRocket works out a fin set's roll inertia with a shortcut, and HPR Sim integrates over
the fin exactly. The shortcut was [measured](../decisions-and-roadmap.md#m2-2b2) on 33 more probe
designs, each a tube and one part. HPR Sim keeps its own: a *departure*, a rule it keeps on purpose,
measured and pinned by a test. The same probes settle how each fin section is weighed and where a
rail button sits. The decisions are in [the decision on fins and rail buttons][adr-062].

> **How far to trust it.** The shortcut is inferred from OpenRocket's output. Its source is GPL, so
> the project does not read it. It matches every fin probe but two to 1e-12, and those two are
> explained below. Every tapered probe has a span half its root chord. Loft's demos, whose spans
> are 0.39 to 0.50 of the root, hold to 0.0005% as well.

**The other parts agree, bar a rail button.** A bulkhead, centering ring, inner tube, mass
component, parachute, shock cord and streamer each have OpenRocket's mass, center of mass and both
inertias. They agree to 1e-15 on their probes. A rail button's inertias are apart by up to 0.05% of
its probe's, and a launch lug's pitch inertia by 0.03% (below).

**OpenRocket's shortcut.** For a set of two or more fins, OpenRocket spreads the set's mass $m$
evenly along a thin rod. The rod runs straight out from the body, at radius $R$, to $R + h_e$, and
OpenRocket takes the rod's roll inertia. $h_e$ is an *effective span*:

$$
\begin{aligned}
I_{\mathrm{roll}} &= m\left(R^2 + R h_e + \frac{h_e^2}{3}\right), \\
h_e^2 &= \frac{A h}{c_r}
\end{aligned}
$$

Here $A$ is one fin's area, $h$ its span and $c_r$ its root chord. For a rectangular fin, $h_e$ is
the span, and the shortcut is exact except that it leaves out the fin's thickness. $h_e$ is shorter
than the span when the fin narrows outward, and longer when it widens. The rule is inferred from
OpenRocket's output, and every tapered probe narrows outward. So a fin that widens is the rule
carried past what was measured. A tab's mass goes where the
fin's does, and neither the section nor the thickness enters. A single fin gets the same rod about
its own middle, $m h_e^2/12$.

**A worked example: the probe's trapezoid.** Three fins sit on a tube 50 mm (2.0 in) in radius. Each has a
100 mm (3.9 in) root chord and a 50 mm (2.0 in) tip chord. Its span and sweep are 50 mm (2.0 in) each, and it's 3 mm (0.12 in) thick,
of 1,000 kg/m³. Each fin has an area of 0.00375 m², so the set weighs 33.75 g. Then

$$
\begin{aligned}
h_e^2 &= 0.00375 \times 0.05 / 0.1 \\
&= 0.001875\,\mathrm{m^2},
\end{aligned}
$$

so $h_e$ = 43.3 mm, and

$$
\begin{aligned}
I_{\mathrm{roll}} &= 0.03375 \times (0.0025 \\
&\quad + 0.05 \times 0.0433 + 0.000625) \\
&= 1.7854 \times 10^{-4}\,\mathrm{kg\cdot m^2},
\end{aligned}
$$

OpenRocket's figure. HPR Sim's exact integral is 1.8284e-4 kg·m², 2.4% more: the
fin's outer part weighs more than the rod puts there.

The rod spreads the mass evenly, but a real fin's mass follows its chord. So the sign depends on
the outline: a triangle's mass sits nearer the body than the rod's. A tab lies inside the body
tube, 40 to 50 mm (1.6 to 2.0 in) from the axis on the probe. But the rod puts its mass out with the fin's, so
OpenRocket's figure is the larger there.

| probe fin set | HPR Sim's roll inertia, kg·m² | OpenRocket's, kg·m² | HPR Sim vs OpenRocket |
|---|--:|--:|---|
| rectangular, 100 mm (3.9 in) by 50 mm (2.0 in) | 2.6253e-4 | 2.6250e-4 | +0.013% (the thickness) |
| the trapezoid above | 1.8284e-4 | 1.7854e-4 | +2.41% |
| triangular, 100 mm (3.9 in) root, 50 mm (2.0 in) span | 1.0314e-4 | 1.0540e-4 | −2.14% |
| the trapezoid with a tab 50 mm (2.0 in) by 10 mm (0.39 in) | 1.9199e-4 | 2.0234e-4 | −5.12% |

The shortcut doesn't match two fin probes to 1e-12. One is an elliptical fin set (its polygon,
below). The other is a canted one, by 2.66e-5 of the probe's roll inertia, not traced.
`hpr_validate::openrocket::openrocket_fin_set_roll_kg_m2` states the shortcut. The survey uses it
for the second roll row of the table [above](#checked-against-openrocket). The test `each_part_alone_is_openrocket_s_or_pinned` holds every probe of this section to OpenRocket's,
or pins how far apart they are.

**How each fin section is weighed.** OpenRocket weighs a fin set as its outline times its thickness
times a factor for its section. The factor is 1 for square, 0.99 for rounded and 0.85 for an
airfoil. It doesn't change with the thickness, checked at 3 mm (0.12 in) and 6 mm (0.24 in). HPR Sim works the section
out. A rounded edge is a semicircle, 0.9914 of the square section on the probe. An airfoil is
NACA's four-digit section, 0.6851 (Abbott and von Doenhoff). A file that says `airfoil` doesn't
say which airfoil, and a builder who weighed the fins can give their mass. So HPR Sim keeps its
sections. The elliptical fin stays the exact ellipse too. OpenRocket's 30-sided polygon weighs
0.18% less.

**Where a rail button sits.** OpenRocket gives a rail button no length. It puts the button's center
where a part of no length would sit, whichever end of the tube the file measures from. A row's
first button goes there, and the rest follow aft. HPR Sim now reads a `.ork` button so (issue
[#151](https://github.com/nrdptel/fusionspace-eridanus/issues/151)). Before, the simulator put the row's forward edge, middle
or aft edge on the position. The move depends on which end the file measures from:

| measured from | how the row moves (r the button's outer radius, s the spacing center to center, n buttons) | two buttons of 10 mm (0.39 in) outer diameter, 100 mm (3.9 in) center to center |
|---|---|---|
| the top, after a part, or absolute | forward r | 5 mm (0.20 in) forward |
| the middle | aft (n − 1)s/2; one button does not move | 50 mm (2.0 in) aft |
| the bottom | aft r + (n − 1)s | 105 mm (4.1 in) aft |

A flight leaves the rail when its aft-most guide does. So a row placed from the top, after a part
or absolutely now leaves it a radius earlier. One placed from the middle or the bottom leaves it
later: more rail to travel, so a little faster off the rail. The probes check the top, the middle and the bottom, with one button
and with a row of two.

**A rail button's screw head is weighed.** HPR Sim [reads](../decisions-and-roadmap.md#m4-5i) a
`.ork` button's `<screwheight>` and weighs the head as half a solid ellipsoid on the flange. The
model is above, under *Standard solids*. Before, it left the head off with a warning, and a rocket
with one did not fly ([the powered pods decision][adr-168]). A 10 mm (0.39 in) button with a 2 mm (0.079 in) screw, in Delrin at
1,420 kg/m³, gains a head of

$$
\begin{aligned}
&\frac23 \times \pi \times (5\,\mathrm{mm})^2 \times 2\,\mathrm{mm} \\
&\quad = 104.7\,\mathrm{mm^3},
\end{aligned}
$$

or 0.149 g, its center
0.75 mm (0.030 in) above the flange. OpenRocket 24.12 was probed with screws of 0, 1, 2 and 5 mm (0.20 in). It gives the
same volume to 2e-16 once its single-precision $2/3$ is used. HPR Sim's double-precision $2/3$ gives
1.1e-8 of the volume more at a 5 mm (0.20 in) screw. Two things differ, both pinned by the test
`a_rail_buttons_screw_head_is_half_an_ellipsoid`:

- OpenRocket puts the head's center $4h/(3\pi)$ above the flange, a flat half-disc's centroid.
  That's 0.049 $h$ further out than the solid's $3h/8$. HPR Sim keeps the solid's.
- With a mass override on the button, OpenRocket still moves the button's center by the screw, and
  so does HPR Sim.

The screw head changes neither the drag, the center of pressure nor the normal force in
OpenRocket, and HPR Sim gives it no drag either.

**What is left, each pinned by a test.**

- Fin fillets' pitch inertia: −0.008% to −0.638% on the eight probes with more than one fin,
  and +0.425% on a single fin ([below](#clusters-and-fillets)). Their mass and center agree.
- Small and not traced:
  - a canted fin set's mass, −0.004%;
  - fins' pitch inertia, up to 0.41% where the fins weigh the same, on a single fin, and up to
    0.11% on the other probes;
  - a launch lug's pitch inertia, 0.03%;
  - a rail button's inertias, up to 0.05%.
- A rail button's screw head: its center, 0.049 of the screw's height nearer the body in HPR Sim
  than in OpenRocket (above).

Fin fillets were left out, a measured departure ([the decision to keep it visible][adr-064]),
until HPR Sim [weighed them](../decisions-and-roadmap.md#m2-2e7) ([below](#clusters-and-fillets)).
A cluster's departure is measured below. Two more gaps these probes found, both in packed parts,
are settled [below](#packed-parts).

**Run it yourself.** As for [the probes above](#what-a-ork-leaves-unsaid-and-overrides): the same
script writes these, and `cargo test -p hpr-validate openrocket` checks them.

[adr-062]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0062-fins-and-rail-buttons-against-openrocket-roll.md
[adr-168]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0168-pods-powered-flies.md

#### Tube fins

A *tube fin set* is a ring of short open tubes along the airframe, in place of flat fins. HPR Sim
weighs each tube as a hollow cylinder with its axis at $R + r$ from the airframe's axis. $R$ is the
body tube's outer radius and $r$ the tube's own. HPR Sim also [reads](../decisions-and-roadmap.md#m2-2e8)
a `.ork` tube fin set whose radius OpenRocket works out from the body
([the format guide](../format/ork.md#tube-fins-sized-from-the-body) has the rule).
[The decision on that radius][adr-098] records what follows.

**Mass and center agree.** On 19 OpenRocket [probe designs](../glossary.md#probe-design), each tube
fin set's mass is within 1e-14 of OpenRocket's and its center within 1e-15 m. OpenRocket's *Tube
fin rocket* example is within 5.0e-6 of OpenRocket's mass and 2.4e-6 of its length in center of
mass. Its two inertias are not: each is a *departure*, a difference HPR Sim keeps on purpose,
measured and pinned by a test.

**Roll inertia, the first departure.** Divide a part's roll inertia by its mass and you get its
*unit inertia*, in m². It's the mean of the squared distance of its mass from the axis. No mass inside
a ring of tubes is farther from the axis than $R + 2r$, the outer edge of the tubes. So no ring's
unit inertia can exceed $(R + 2r)^2$. For two tubes or more, OpenRocket's does:

| six tubes of 20 mm (0.79 in) radius on a 50 mm (2.0 in) body | unit roll inertia |
| --- | --- |
| the most any ring can have, $(0.05 + 2 \times 0.02)^2$ | 0.0081 m² |
| OpenRocket 24.12's | 0.3047 m², about 38 times that |

HPR Sim keeps its own figure, which is below that bound, as it must be. For a single tube the two
agree. OpenRocket puts its center at $R + r$, where HPR Sim does. OpenRocket's unit inertia about the tube's
own axis is $(r^2 + r_i^2)/2$, with $r_i$ the tube's inner radius, as HPR Sim's is.

**Pitch inertia, the second departure.** Pitch is turning end over end, about an axis across the
airframe through the set's center. One tube's own unit pitch inertia, about its own middle, is
$(r^2 + r_i^2)/4 + L^2/12$ for a tube of length $L$.

- For three tubes or more, HPR Sim adds the ring's spread, $(R + r)^2/2$. That's the mean squared
  distance of the tube axes from the pitch axis, since they sit at $R + r$ all around.
- On all 19 probes, OpenRocket's is $N$ times one tube's own, for $N$ tubes, with no term for the
  tubes' distance from the airframe's axis.

OpenRocket's figure is 1.3 to 2.6 times HPR Sim's on the probes. On six tubes of 20 mm (0.79 in) radius on a
50 mm (2.0 in) body it's 1.77 times. On the probes, whose tubes are 0.1 m (0.33 ft) long, it stays under a bound. That's the
most any mass inside the ring could have about that axis, $(R + 2r)^2 + (L/2)^2$. On the
*Tube fin rocket*'s longer tubes it is 18% over that bound, 3.352e-3 m² against 2.834e-3 m² a unit
of mass. So no mass could have it. Either way it is not what tubes at $R + r$ weigh, so HPR Sim keeps its own. For
a single tube the two agree.

**On the example.** On OpenRocket's *Tube fin rocket*, HPR Sim's roll inertia is 98.7% below
OpenRocket's, and its pitch inertia 1.98% below. The pitch gap is sized. `cargo xtask ork` swaps
OpenRocket's pitch rule into HPR Sim's structure for each tube fin set and prints the result. The
gap goes from −1.9800% to +0.0023%, well inside the survey's 0.1%. The survey
[above](#checked-against-openrocket) names tube fins as the cause of the roll gap. OpenRocket's fin
shortcut, which the survey swaps in for flat fins, is not given OpenRocket's mass for tube fins,
so it cannot close this one.

The test `a_tube_fin_sets_automatic_radius_reads_as_openrocket_does` in `hpr-validate` holds both
departures both ways. OpenRocket's roll is above the bound and HPR Sim's below it, on every probe
with two tubes or more. OpenRocket's pitch is $N$ times one tube's own, to 1e-14. HPR Sim's roll and
pitch match their closed forms to 1e-13.

[adr-098]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0098-a-tube-fin-sets-automatic-radius-read-as.md

### Clusters and fillets

A [cluster](../glossary.md#cluster) is a motor mount with more than one motor tube. HPR Sim
[weighs every tube](../decisions-and-roadmap.md#m1-9b) where it sits, each with its own
[parallel-axis](../glossary.md#parallel-axis-theorem) term. It repeats what a tube holds in every
tube ([Clusters](design.md#clusters)). OpenRocket 24.12 was asked on 25 probes
([the decision on clusters][adr-075]). It agrees on the mass and the center of mass to 1e-12, but not on the inertia.
It weighs a cluster's tubes as if stacked on the cluster's axis. A 3-ring at scale 1 and at scale
1.5 have the same inertias in OpenRocket. It does weigh an engine block inside each tube where it
sits. HPR Sim does not copy the stacking, since the tubes are not on the axis.

The fixed 3-ring probe is a tube and a 200 mm (7.9 in) inner tube, three of them 23.09 mm (0.91 in) from the axis. Each
inner tube has a 20 mm (0.79 in) outer radius and 1 mm (0.039 in) wall. OpenRocket's saved structure is 0.3813893481458014 kg, with
center 0.24036243822075784 m (0.788590676577 ft), roll 0.0007674901427941916 kg m² and pitch 0.007191233036548777
kg m². HPR Sim's mass and center are the same. Its roll inertia is +5.11% apart and its pitch
+0.273%: the spread of the tubes, and half of it. The spread is $3\,m d^2$, 3.92e-5 kg m², for each
tube's mass $m$ at $d$ = 23.09 mm from the axis. On every cluster probe on the body's axis the difference is exactly
that, to 1e-12, whatever the pattern, scale, contents or overrides. Off the axis, three differences
are left and pinned as measured, with the spread taken out. A lone tube 10 mm (0.39 in) off the axis is
+0.303% in roll and +0.016% in pitch. That's $m d^2 (1 - m/M)$ in roll and half of it in pitch, as
if OpenRocket left the offset out. Here $m$ is the tube's mass, $d$ its offset and $M$ the
structure's mass. The pitch of two clusters off the axis is +0.015% and +0.021%, which the project
has not traced. Before HPR Sim [weighed every tube](../decisions-and-roadmap.md#m1-9b), it read
one. That left it 12.85% light, 5.95 mm (0.23 in) forward, 2.43% low in roll and 3.68% low in pitch. `each_part_alone_is_openrocket_s_or_pinned` and
`a_cluster_weighs_as_openrocket_s_but_for_its_tubes_spread` check these.

#### Fin fillets

A [fin fillet](../glossary.md#fillet) is the rounded glue joint along a fin's root, where the fin
meets the tube. HPR Sim [weighs fillets](../decisions-and-roadmap.md#m2-2e7) as OpenRocket 24.12
does ([the decision on fillets][adr-096]). Before, it left them out with a warning. The 5 mm (0.20 in) and
10 mm (0.39 in) probes were then 0.808% and 2.79% light ([the decision on clusters, fillets and unread parts][adr-064]).

**The shape.** A fillet fills the corner between the tube and the fin's side. Its concave face is a
circle of the fillet's radius that touches both. Its section is bounded by three edges: the tube's
circle, the fin's plane and that circle. As in OpenRocket, the fin is taken as having no thickness
there, so the section starts at the fin's middle plane, not its face. Two fillets run along each
fin, one each side, as long as the root chord. So each is a prism of that section. It is made of the
file's `filletmaterial`. A file that names none gets OpenRocket's cardboard, 680 kg/m³. Most
fillets are epoxy, which is heavier (HPR Sim's `epoxy` is 1,180 kg/m³). So set `filletmaterial` in
OpenRocket, or the fillet's material in HPR Sim, to what you used.

**The equation.** Take the body radius $R$ and the fillet radius $r$. Take $x$ outward from the
body's axis along the fin's mid-plane, and $y$ square to it. The fillet circle's center is at
$(c, r)$, with $c = \sqrt{R^2 + 2Rr}$. That puts it $\sqrt{c^2 + r^2} = R + r$ from the axis, so the circle
touches the tube. The section is the triangle $(0, 0)$, $(c, 0)$, $(c, r)$ less two circular
sectors. One is the tube's, up to the angle $\theta = \arctan(r/c)$, and the other the fillet
circle's, whose angle is $\pi/2 - \theta$:

$$
\begin{aligned}
A &= \frac{c r}{2} - \frac{R^2\theta}{2} \\
&\quad - \frac{r^2(\pi/2 - \theta)}{2}
\end{aligned}
$$

On a flat body ($R$ very large) this tends to $r^2(1 - \pi/4)$: a square less a quarter circle.
HPR Sim works out the section's first and second moments the same way, in closed form. The sectors'
second moments are computed in a stable form. But the final subtraction still loses about
$\log_{10}(R/r)$ digits: at most 2 for a fillet 1% of the body radius or larger. A test checks all four
against numerical integration to 1e-11 (`fillet_section_is_its_region_by_quadrature` in
`hpr_design::fins`). Far from real fillets the subtraction cancels away. So HPR Sim weighs a
fillet under a millionth of the body radius as nothing, and refuses one over a thousand times it
(`FILLET_RATIO_MAX`). It gives a body of no radius no fillet. Against 60-digit arithmetic, the
area keeps 10 significant digits at a millionth of the body radius and 12 at a thousand times it.
That was checked once while writing the code. The thousand is a cautious limit, not where the
digits run out: at ten thousand times it still keeps 10.

**A worked example.** An invented 5 mm (0.20 in) fillet on a tube 30 mm (1.2 in) in radius. Then
$c = \sqrt{30^2 + 2 \times 30 \times 5}$ = 34.64 mm and $\theta = \arctan(5/34.64)$ = 0.1433 rad (8.21°).

| piece | formula | area |
|---|---|--:|
| the triangle | $c r/2$ | 86.60 mm² |
| less the tube's sector | $R^2\theta/2$ | 64.51 mm² |
| less the fillet circle's sector | $r^2(\pi/2 - \theta)/2$ | 17.84 mm² |
| the section | $A$ | 4.253 mm² |

That is 79% of the 5.365 mm² a flat body would give: the tube curves away from the fin, so the
corner holds less. Three fins with a 100 mm (3.9 in) root chord have six fillets. Their volume is
$6 \times 4.253\,\mathrm{mm^2} \times 100\,\mathrm{mm}$ = 2,552 mm³ = 2.55 cm³. In cardboard they weigh
$2.552\,\mathrm{cm^3} \times 0.680\,\mathrm{g/cm^3}$ = 1.735 g. A unit test pins these numbers
(`the_worked_fillet_example_is_the_docs` in `hpr_design::fins`).

**How well.** Nine probe designs measure it: the 5 mm (0.20 in) and 10 mm (0.39 in) probes of
[the earlier fillets step](../decisions-and-roadmap.md#m2-2b4), and seven
[new ones](../decisions-and-roadmap.md#m2-2e7):

- fillets of 30 mm (1.2 in);
- fillets in their own material;
- fillets naming no material;
- a single fin;
- four fins of rounded section;
- a freeform fin set;
- a wider tube.

On every one the fillets' mass and center of mass are OpenRocket's to 1e-15, and the test holds
them to 1e-12. On the rounded-section probe the whole fin set's mass is 1.79e-4 apart. That's from
the fin section's factor ([above](#fins-rail-buttons-and-roll-inertia)), not from the fillets. The whole
probe's pitch inertia is apart by −0.0077% to −0.638% on the eight with more than one fin, growing
with the fillets' mass. On the single fin it is +0.425%, near the +0.406% a single fin reads
without fillets. HPR Sim's figure is the exact prism's. How OpenRocket works out a fin's pitch
inertia has not been measured ([the decision on fins and rail buttons][adr-062]). The roll rows use OpenRocket's fin shortcut, as every fin probe
does. `each_part_alone_is_openrocket_s_or_pinned` pins every row.

**What it leaves out.** The fillets' drag and lift: the aerodynamics ignores them
([Drag limits](aero.md#drag-limits)). A fillet that is not a circular arc, such as a hand-shaped
bead of glue, is weighed as one.

[adr-096]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0096-fin-fillets-and-an-automatic-radius-inside-a.md
[adr-064]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0064-clusters-fillets-and-unread-parts-remain-visible.md
[adr-075]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0075-a-cluster-is-one-tube-repeated-and-a-motor-in-it.md

### Packed parts

A parachute, streamer, shock cord or mass component (an altimeter, a battery, ballast) is weighed
as a solid cylinder ([above](#standard-solids-mk)). The cylinder is its packed length and packed
radius, where it sits in the tube. Two cases need a rule of their own. HPR Sim now
[takes OpenRocket's](../decisions-and-roadmap.md#m2-2b3) for both, measured on probe designs
OpenRocket 24.12 reads ([the decision on packed parts][adr-063]). Like every OpenRocket rule on this page, they are
inferred from what OpenRocket prints, not read from its source.

**A file that writes no packed size.** OpenRocket packs the part 25 mm (0.98 in) long and 12.5 mm (0.49 in) in radius.
The radius is a fixed number, not the tube's bore. It's the same in bores 48 and 98 mm (3.9 in) in radius.
OpenRocket does not shrink it to fit a bore of 8 mm (0.31 in), and neither does HPR Sim. Each number
stands alone. So a file that writes only a length gets the 12.5 mm (0.49 in) radius. One that writes only a
radius gets the 25 mm (0.98 in) length. HPR Sim reads a `.ork` the same way, with no warning.
Before, it read a length of zero with no warning, and a radius of zero with one.

**A mass override on a part that weighs nothing.** A parachute with no canopy, or a mass component
of 0 g, given a 30 g override: OpenRocket spreads the 30 g over the packing. So does HPR Sim now. In
a packing 50 mm (2.0 in) long and 20 mm (0.79 in) in radius that is:

| | formula | 30 g in the probes' packing |
|---|---|---|
| roll inertia | $m r^2/2$ | 6.0 × 10⁻⁶ kg·m² |
| pitch inertia, about its own center | $m(3r^2 + l^2)/12$ | 9.25 × 10⁻⁶ kg·m² |
| center | halfway along the packing | 25 mm (0.98 in) aft of its forward end |

Before, HPR Sim put the 30 g at a point, which left the probe's roll inertia 0.805% low. Any other
part that weighs nothing still becomes a point mass under an override. Both programs do that
([above](#what-a-ork-leaves-unsaid-and-overrides)).

**How well.** Eleven probes ask these questions, a tube and one packed part each. HPR Sim's
structure is OpenRocket's to 1e-12 on every one, in mass, center of mass, roll and pitch. The
earlier 74-file before-and-after measurement took the roll inertia from 49 to 53 files within
0.1%, and from 55 to 57 within 1%. That's with OpenRocket's fin shortcut in HPR Sim's place. The current
survey, on 71 designs, has 59 within 0.1% and within 1% with that shortcut
([above](#checked-against-openrocket)):

| (earlier 74-file measurement) | before | after |
|---|---|---|
| center of mass within 0.1% of length | 54 of 74 | 55 of 74 |
| pitch inertia within 0.1% (median) | 34 of 74 (0.110%) | 37 of 74 (0.086%) |
| roll inertia, fin shortcut in HPR Sim's place, within 1% | 55 of 74 | 57 of 74 |
| roll inertia, HPR Sim's own fins, within 1% (median) | 29 of 74 (2.112%) | 28 of 74 (2.354%) |

The last row moves the wrong way. HPR Sim's own fin roll departs from OpenRocket's
([above](#fins-rail-buttons-and-roll-inertia)). In one file the point mass had left HPR Sim's roll
inertia low, which happened to cancel part of that departure. The packing now adds it back.

**What it leaves out.** A packed size the file writes but HPR Sim can't read as a number is read as
zero, with a warning, like any unreadable number. Every probe places its part from the tube's top.
How the 25 mm (0.98 in) length moves a part placed from the middle, the bottom or after another part is
worked out, not measured. The override rule was probed on a parachute, a mass component and a
shock cord. A streamer takes it too, by the same packing, without a probe of its own. Mass is
unchanged: the rules move mass, they add none.

**Run it yourself.** `refs/venv/bin/python validation/oracles/openrocket/conventions.py
validation/fixtures/ork/openrocket-conventions.json` writes the probes (it needs OpenRocket 24.12
and Java 17), and `cargo test -p hpr-validate openrocket` checks them.

[adr-063]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0063-packed-parts-read-and-weighed-as-openrocket.md

## Verification

- By hand:
  - `mass::tests`:
    - two boxes make one box;
    - point masses give the products of inertia;
    - rolling swaps and mixes axes, so $I'_{xy}$ is $(I_{xx} - I_{yy})\sin\theta\cos\theta$;
    - rotation keeps the principal moments;
    - motor elements land on the axis.
  - `tests::composite_rocket_inertia_matches_hand_calculation` (crate root) combines a filled cone, a tube,
    four fins and an off-axis payload. Each part's moments come from its own formula and the six
    tensor terms are written out; the result agrees to 1e-11.
  - `parts::tests::a_nose_cone_with_a_capped_shoulder_adds_up_by_hand` checks the cone, tube and
    cap to 1e-11.
  - `fins::tests`:
    - A rectangular fin is a box, and four fins are the sum of rotated boxes.
    - A swept fin's $I_{xz}$ matches quadrature of the planform.
    - A fin canted 90° is the box turned.
    - Tube fins match the parallel-axis theorem.
- Closed forms against quadrature:
  - Each cross-section's $M_0$, $M_1$, $M_2$, $T$ matches exact quadrature of $t(x)$ to 1e-13, on a wide
    chord and one narrower than $t$.
  - The airfoil constants match to 1e-14.
  - Trapezoidal, elliptical and freeform areas and centroids match to 1e-12.
- Properties (`mass::tests`, proptest): combining is associative and order-free, and turning a
  body keeps its principal moments. The inertia about any point exceeds that about the center.
- Materials: ids are unique, sources present, and the unit conversions reproduce the sources
  (1.1 oz/yd² = 37.3 g/m², 225 ft/lb = 6.61 g/m, white ash 678 kg/m³).
- Loft lessons:
  - [L44](../decisions-and-roadmap.md#l44) `thin_tube_inertia_includes_radial_term`.
  - [L45](../decisions-and-roadmap.md#l45) `hollow_transition_and_freeform_fin_cg_are_exact_centroids`: a conical wall's
    exact centroid, and an M-shaped fin against the shoelace centroid.
  - [L46](../decisions-and-roadmap.md#l46) `fin_tab_and_rail_button_mass_counted`.

[adr-006]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0006-component-geometry-and-mass-properties-frames.md
[levels]: ../accuracy.md#four-kinds-of-evidence
