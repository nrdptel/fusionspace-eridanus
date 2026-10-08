# ADR-061: What a `.ork` leaves unsaid, read as OpenRocket reads it; overrides measured, two departures kept (2026-09-21)

- **Status:** accepted
- **Summary:** What a `.ork` leaves unsaid, read as OpenRocket reads it; overrides measured, two departures kept

**Context.** M2.2a (ADR-060) traced the designs outside a threshold to five causes. Two were
readings, not physics. hpr read a shoulder written with no wall as solid, where OpenRocket gives it
no mass. hpr gave a part written with no material no mass, where OpenRocket gives it a default. And
[Loft lesson L51](https://github.com/nrdptel/hpr-sim/blob/main/docs/research/loft-lessons.md) asks which
centre-of-gravity override wins: Loft's rule came from OpenRocket's source, which this project does
not read. M2.2b's *done when* asks for each convention to be hpr's rule or a written departure.
M2.2b is more than one pull request, so it splits into b1 (these readings and the overrides), b2
(clusters, fillets, airfoil fins, the parts kept unread, and roll inertia) and b3 (Loft lesson L87,
stored results).

**Decision.**

1. **Measured, not assumed.** `validation/oracles/openrocket/conventions.py` writes 32 small probe
   designs, each asking one question, and records what OpenRocket 24.12 makes of each: its
   structure and its own per-part breakdown, and the material it gives each part through the
   public `getMaterial` and `getLineMaterial`. The record is
   `validation/fixtures/ork/openrocket-conventions.json`. `hpr_validate::openrocket::tests` reads
   the same documents and holds hpr to it, and fails if a probe is added that no test reads.
2. **Where the file leaves something unsaid, hpr reads it as OpenRocket does.** OpenRocket's
   reading is what the file means to the people who wrote it.
   - A wall of no thickness on a nose cone, transition or body tube is a surface with no wall: the
     part keeps its shape and weighs nothing. So does an inner tube, coupler or lug of no wall,
     which hpr already read so but warned of. `hpr_design::solids::Wall::Shell` now takes a
     thickness of zero. A solid part is written `filled`; OpenRocket writes it so.
   - A shoulder of no wall, or none written, weighs nothing, capped or not, and whether or not
     its nose is filled. A filled nose keeps its shoulder's own wall (the probe: 0.84446 kg, the
     solid cone and the walled shoulder).
   - A nose cone, transition or body tube that writes no thickness has a 2 mm wall, whatever its
     radius: a nose and a tube at 50 mm and at 30 mm, and a transition, each weigh what a 2 mm
     wall gives.
   - A part that names no material is made of OpenRocket's default for its kind: cardboard,
     680 kg/m³, for a part weighed by its volume (probed on a nose, transition, body tube, inner
     tube, coupler, engine block, three kinds of fin set, ring, bulkhead and lug); ripstop nylon, 0.067 kg/m², for a canopy or streamer; an elastic
     cord, 0.0018 kg/m, for shroud lines and a shock cord; and Delrin, 1,420 kg/m³, for a rail
     button.

   - A mass override on a part that weighs nothing is a point mass at the middle of its length,
     in both programs.

   None of these is warned of any more: nothing is assumed. So a configuration held back only for
   them now flies. One reading is not taken: an inner tube, coupler or lug that writes no
   thickness gets a wall of OpenRocket's own (0.5 mm on the probe's 20 mm inner tube, 1 mm on its
   5 mm lug, none on the coupler). One size each does not say whether it follows the radius and
   no file in the library has one, so hpr keeps reading it as no wall, with a warning, and the
   test pins the difference. OpenRocket's default materials were read with none saved in its
   preferences; the script records that and refuses to run otherwise.
3. **Which override wins is OpenRocket's.** On every override probe hpr's mass is OpenRocket's.
   - A parent's override that covers its children wins over a child's own, and a stage's wins over
     everything in it.
   - A centre-of-gravity override is measured from the part's front, not its shoulder's, and moves
     the shoulder with the part.
   - A centre-of-gravity override alone, written to cover the parts inside, sets the whole
     assembly's centre. hpr had taken the scope from the absent mass flag; it now takes the flag
     of the one quantity overridden. (How the parts inside are placed differs, which shows only in
     the inertia: §4.)
4. **Two departures are kept, and one limitation, each pinned by a test.**
   - **The centre under a covering mass override.** When a mass override covers the parts inside
     and states no centre, OpenRocket puts the centre at the overriding part's own and leaves the
     parts inside out of it. hpr keeps the centre its parts lay out. A builder who weighs a tube
     with its fins and mount inside has not moved their centre, and hpr's is the better estimate.
     On the probe the two are 3.7 mm apart, or 19.7 mm when the part inside has an override of its
     own.
   - **Inertia under an override.** hpr scales the inertia of everything a mass override covers by
     the override's ratio, so the extra mass sits where the parts' mass does and the radius of
     gyration is kept. OpenRocket scales only the overriding part's own inertia, keeps the parts
     inside at theirs, and a stage, having none of its own, scales nothing. On a lone part the two
     agree. On the probes hpr's roll inertia is 6.9% to 37% below OpenRocket's under a tube's
     covering override, and 2.5 to 5.0 times OpenRocket's under a stage's. Neither rule is right
     for every rocket; hpr's is consistent with its own mass. Likewise under a centre override
     that covers the parts inside: hpr moves the assembly whole, OpenRocket the part alone, and
     OpenRocket adds the parts inside where they were (pitch inertia 2.65% apart on the probe).
   - **Flags that disagree** (a limitation). A part overriding both quantities with flags that
     disagree cannot be said in `hpr-design`, which scopes a part's overrides once: the mass flag
     decides, with a warning, as before. On the probe that is 4.7 mm.

**Consequences.** On 2026-09-21, rerunning M2.2a's survey (`cargo xtask ork`, OpenRocket's record
unchanged):
- **Mass:** 61 of 74 within 1% (was 57), median 0.002% (was 0.020%).
- **Centre of mass:** 62 of 74 within 1% of length (was 58).
- **Pitch inertia:** 53 of 74 within 1% (was 46).
- **Outside a threshold:** 13 files (8 by content), down from 17 (12). Each is a cluster, fillets
  left out, or parts kept unread, b2's and M1.9's.
- **Flying:** a second configuration flies (2 of 174).
- **Warnings:** those reading designs fall from 96 to 57, and parts that weigh nothing from 21 to 14.
- **Two gaps the probes found**, both for b2 and both pinned: OpenRocket puts a rail button's
  centre at its position, where hpr places it by its forward edge, one radius (5 mm) further aft
  (#151); and OpenRocket's elliptical fin weighs 0.18% less than hpr's exact ellipse, which
  matches a 30-sided polygon inscribed at equal angles to 13 digits (inferred from its output).
- **L51 is live:** `hpr_validate::openrocket::tests::override_precedence_matches_oracle`.

---
