# ADR-064: Clusters, fillets and unread parts remain visible departures (2026-09-22)

- **Status:** accepted; fillets superseded by ADR-096
- **Summary:** Clusters, fillets and unread parts remain visible departures

**Context.** M2.2b4 asks whether motor clusters, fin fillets and parts hpr does not read are hpr's
rule or a written departure, then asks for the M2.2a corpus comparison again. The reader currently
reads a clustered inner tube as one `InnerTube`, omits the mass of a fin's fillet, and keeps parts it
cannot model in `x-openrocket` while marking the design reduced. The first two are deliberate
simplifications; the third is the lossless boundary set in [ADR-058][adr-058]. Full clustered-motor
mount and flight behavior belongs to M1.9, not this mass-convention increment.

**Decision.**

1. **Clusters stay one tube for now.** A non-`single` `clusterconfiguration` remains one hpr inner
tube and raises the existing warning. A new fixed probe, `a tube and a clustered inner tube`,
adds a 3-ring inner tube to the conventions oracle. OpenRocket 24.12, after saving the probe, gives
0.3813893481458014 kg, a centre station of 0.24036243822075784 m, roll inertia
0.0007674901427941916 kg m² and pitch inertia 0.007191233036548777 kg m². hpr's one-tube reading
is pinned as a departure: relative mass −12.85%, centre +5.95 mm, relative roll inertia −2.43%
and relative pitch inertia −3.68%. The values are measured by the external oracle; hpr does not
read OpenRocket's source. N-tube geometry, motor summation and motor-out moments remain M1.9's
work.
2. **Fillets remain omitted, explicitly.** A positive `filletradius` raises the existing warning;
`filletmaterial` is not used to invent a shape or mass. The 5 mm probe pins hpr's omission at
−0.808% mass, +0.737 mm centre and −0.439% pitch inertia; the 10 mm probe pins −2.79% mass,
+2.54 mm centre and −1.52% pitch inertia. Roll is compared with OpenRocket's fin shortcut, so the
pinned roll departure is zero for these probes. A future fillet model must replace this departure
with a measured shape, not silently change it.
3. **Unread parts remain losslessly visible.** Pods, parallel stages and unsupported fin shapes, plus five skipped parts across three kinds, remain
in `x-openrocket`, with their paths and a reduced-design flag;
hpr does not guess their geometry or add their mass. The existing warning taxonomy and survey
matching remain the evidence that the gap is visible. This confirms [ADR-058][adr-058] as the b4
rule rather than treating unread content as an unexplained comparison failure.
4. **The corpus rerun is the check.** On 2026-09-22, `cargo xtask ork` found 78 files, read 76,
laid out 75 and compared 74 designs (54 distinct contents). Structure mass was within 1% on 61
of 74, centre of mass within 1% of rocket length on 62, pitch inertia within 1% on 53 and roll
inertia within 1% on 57 when OpenRocket's fin rule was used. The eight distinct contents outside
one or more thresholds have nine cause assignments: cluster read as one tube (2), fillets left
out (2), unread/reduced parts (5), and switched-off stage (0). The one overlap is why assignments
sum to nine. The probe suite and `cargo test -p xtask ork_mass` both pass.

**Consequences.** M2.2b4 is complete without pretending that one tube is a cluster or that an
unread part has a mass model. The import warnings, reduced flag and `x-openrocket` preservation
make each departure inspectable. M1.9 may replace the cluster departure when its mount and flight
work is ready; a fillet model may replace its two measured departures only with new probes. M2.2b5
is next.

The numbers above are the 2026-09-22 as-of snapshot. The reproducible rerun on 2026-09-23 excludes
generated `refs/scratch/` files from the default scan: it found 75 files, read 73, laid out 72 and
compared 71 designs (51 distinct contents). Mass was within 1% on 58/71, centre of mass on 59/71,
pitch inertia on 50/71 and roll inertia on 56/71 with OpenRocket's fin rule. The current survey's
cause summary is 2 cluster cases, 2 fin-fillet cases and 6 reduced designs among the 11 distinct
contents whose roll remains outside 1% after that rule; the mass-or-CG comparison has 2 cluster,
2 fillet and 5 reduced cases. `cargo xtask ork` prints these counts and keeps its detailed record
private in `corpus-out/ork-survey.json`.

[adr-058]: https://github.com/nrdptel/hpr-sim/blob/main/docs/decisions/0058-what-a-ork-holds-that-hpr-does-not-model-kept.md
