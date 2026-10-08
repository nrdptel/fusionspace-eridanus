# ADR-170: A packed part wider than its bore warns (2026-10-05)

- **Status:** accepted
- **Summary:** M4.5l: a packed part (a mass component, parachute, streamer or shock cord) drawn wider than the room in its parent warns, by any amount, while its centre is in that room, and flies as drawn; its width sets only its own inertia. A ring, tube or packed part centred past the room stays an error. OpenRocket's *Deployable payload* flies its five configurations as saved

**Context.** M4.5 asks that every in-scope OpenRocket example fly as saved. After
[M4.5k](0169-a-ring-around-its-tube.md), the five configurations of OpenRocket 24.12's
*Deployable payload* were the only ones of the examples' 56 that flew only with
`--accept-design-errors`. The file packs its payload, a mass component of 14.2 g, and its 10 in
parachute each 25 mm across (`packedradius` 0.0125 m, not `auto`) into a body tube and a coupler
with a 21 mm bore. The fit check ([ADR-155](0155-design-checks-that-match-reality.md)) measures
them like any internal part: 2 mm past the bore on each side, past the 0.5 mm fit tolerance, so
two errors. [ADR-165](0165-an-unpowered-separation-in-hpr-sim.md) kept them; ADR-169 left M4.5 a
decision on them.

ADR-155 says what an error is for: "designs that can't exist as described, whose simulation would
be wrong." A ring or a tube drawn too wide fails both halves. Its mass comes from its volume and
material, so the part of it inside the parent's wall counts twice, and a part that is really there
must be narrower. A packed part fails only the first. hpr takes its mass as stated, not from its
size (`hpr_design::parts`, `Packing::place`), and its packed length and station set where that
mass sits. Its packed radius enters the flight in one place: the part's own moment of inertia, a
solid cylinder's, `I_axial = m r²/2` and `I_transverse = m (3r² + L²)/12` about its centre.

Packing it in the room it has, at the same mass, length and station, changes the rocket's moments
of inertia by `m (r_drawn² − r_room²)/2` about its axis and `m (r_drawn² − r_room²)/4` across it,
and nothing else. For the *Deployable payload*'s payload that is 14.2 g × (12.5² − 10.5²) mm² / 4,
1.6e-7 kg m² across the rocket.

**Measured.** Each configuration flown by `hpr sim` with the file as saved and with a scratch copy
whose two `packedradius` values are set to the 10.5 mm room (both copies outside `refs/`):

| Configuration | Apogee as saved (m) | Packed in its room (m) | Δ, relative to the apogee | Landing, both (s) |
|---|---:|---:|---:|---:|
| 1, [A8-3] | 32.555426 | 32.555426 | −2.5e-8 | 10.913 |
| 2, [B4-4] | 96.940366 | 96.940359 | +7.5e-8 | 29.537 |
| 3, [C6-3] | 233.726484 | 233.726484 | +1.2e-10 | 64.660 |
| 4, [C6-5] | 265.041529 | 265.041538 | −3.4e-8 | 74.339 |
| 5, [C6-7] | 265.961521 | 265.961533 | −4.6e-8 | 73.427 |

The largest change is 12 µm of apogee (configuration 5), and 7.5e-8 of the apogee at most
(configuration 2). The static margins agree to the printed 0.01 calibre.

**Decision.**

1. **A packed part drawn wider than its room warns.** A mass component, parachute, streamer or
   shock cord that reaches past the room in its parent (a tube's bore, or a nose cone's or
   transition's largest outer radius, as before) is `packed_part_wider_than_parent`, a warning,
   by any amount, while its centre (its radial offset from the parent's axis) is in that room. It
   flies as drawn, as OpenRocket flies it. The warning says it can't go in as drawn, that its
   mass, length and station fly as drawn, and that its width sets only its own inertia.
2. **No fit tolerance for packed parts.** A packed part's mass doesn't count twice where it
   crosses a wall, so `internal_part_tight_in_parent`'s "fit to sand" doesn't describe it. A
   packed part past its room by less than the fit tolerance gets the new warning, or the error
   below, instead.
3. **What stays an error.** A packed part centred past its room is
   `internal_part_wider_than_parent`, an error, however little it reaches past: its mass sits
   off the axis where no part can be, moving the rocket's centre of gravity sideways. Before this
   decision a thin one reaching past by less than the fit tolerance warned as a fit to sand.
   Rings, tubes and every other internal part keep ADR-155's tolerance and caps.
4. **Room is as before.** In a nose cone or a transition the room is still its largest outer
   radius, not its radius where the part sits ([#313](https://github.com/nrdptel/hpr-sim/issues/313)),
   so a packed part near a nose's tip is tested against the cone's base: its centre can be
   outside the skin at its own station and still warn.

**Why not narrow it.** hpr could fly a packed part at its room's radius, as a builder would pack
it. That keeps the inertia bounded whatever the file says, but it changes a stated value without
the user asking, and departs from OpenRocket's flight of the same file. The warning tells the user
instead, and the effect is the closed form above: on the *Deployable payload* 12 µm of apogee at
most. The warning's band has no upper limit, and this decision cites no bound for one: a packed
part typed much too wide, ballast 400 mm across in a 54 mm nose cone, adds inertia that narrowing
would remove. It warns, so the reader sees it, and a test pins that case.

**The private library.** `cargo xtask ork-cli` flies the survey's 170 configurations through
`hpr sim`. In the private design library, 5 configurations flew only with the flag before this
decision and fly as saved now. They carry the same two packed parts, at the same sizes, as
OpenRocket's example, and the same apogee changes when narrowed, so they are no independent
evidence. One other configuration family, 5 configurations with a packed
part 0.2 mm past its bore, warned as a fit to sand before and warns as packed now; narrowing it
moves no apogee. No other configuration's outcome changed, and no new error appeared.

**Consequences.**

- Of OpenRocket's 56 example configurations, `hpr sim` flies 51 as saved (46 before), none with
  the flag, and refuses 5. Across the survey's 170, 131 fly as saved (121 before), 9 with the flag
  (19) and 30 are refused.
- M4.5's other blockers are increments of their own: *Parallel booster staging* (M4.5m),
  *Pods--powered with recovery deployment*'s first configuration (M4.5n), and *Base drag hack*'s
  E12-4, 8.0% above OpenRocket's apogee (M4.5o, issue #177).
- Tests in `hpr_design::checks` pin the warning for a payload 5 mm and 0.5 mm too wide and a
  parachute 4.5 mm too wide, the error for a ring as wide, the centre's edge on both sides, and
  the error for thin parts centred in the wall within the fit tolerance. A mutation dropping the
  centre's condition fails two tests; one giving packed parts the fit tolerance fails one.
