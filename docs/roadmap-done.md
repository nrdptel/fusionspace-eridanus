# Roadmap: done

The milestones and increments checked off in [the roadmap](ROADMAP.md), which holds open work
only. `cargo xtask records` moves each one here once its box is checked, its lines unchanged, so
every *done when* reads as it was written. Under each phase they keep the roadmap's nesting: a
done increment of a milestone that is still open sits under a one-line stub naming it. Never
edit an entry here; a change to a met milestone is a new ADR.

## Index

One line per milestone or increment, in the order it was archived within its phase.

- M0.1 Workspace, CI, licenses ([Phase 0](#phase-0-foundations))
- M0.2 Reference library ([Phase 0](#phase-0-foundations))
- M0.3 Lessons from Loft ([Phase 0](#phase-0-foundations))
- M0.4 A documentation site people can read ([Phase 0](#phase-0-foundations))
- M0.4a The site and its link checks ([Phase 0](#phase-0-foundations))
- M0.4b Model pages, Accuracy, Glossary, Checking a claim ([Phase 0](#phase-0-foundations))
- M0.4c Getting started, and how a flight is simulated ([Phase 0](#phase-0-foundations))
- M0.4d Publish ([Phase 0](#phase-0-foundations))
- M0.4e The reader test ([Phase 0](#phase-0-foundations))
- M0.5 Leaner bookkeeping ([Phase 0](#phase-0-foundations))
- M0.5a One file per decision record ([Phase 0](#phase-0-foundations))
- M0.5b A roadmap of open work, in queue order ([Phase 0](#phase-0-foundations))
- M0.5d Generated mirrors ([Phase 0](#phase-0-foundations))
- M0.5c Short planning notes ([Phase 0](#phase-0-foundations))
- M0.5e Reviews while CI runs ([Phase 0](#phase-0-foundations))
- M0.5f Shorter planning files ([Phase 0](#phase-0-foundations))
- M0.5g The gate runs the `refs/` checks ([Phase 0](#phase-0-foundations))
- M0.5h The checks enforce the new rules ([Phase 0](#phase-0-foundations))
- M0.5i The first 20 pull requests against the baseline ([Phase 0](#phase-0-foundations))
- M0.6 The FusionSpace product system ([Phase 0](#phase-0-foundations))
- M0.6a The docs site ([Phase 0](#phase-0-foundations))
- M0.6b The CLI and exports ([Phase 0](#phase-0-foundations))
- M0.6c The plot ([Phase 0](#phase-0-foundations))
- M0.6d US spelling, no em dashes ([Phase 0](#phase-0-foundations))
- M0.6d1 The check, and the pages ([Phase 0](#phase-0-foundations))
- M0.6d2 The crates' words ([Phase 0](#phase-0-foundations))
- M0.6d3 The crates' em dashes ([Phase 0](#phase-0-foundations))
- M0.6e US names ([Phase 0](#phase-0-foundations))
- M0.7a1 The no-clipping checks ([Phase 0](#phase-0-foundations))
- M0.9a The name everywhere ([Phase 0](#phase-0-foundations))
- M0.9a1 The packages' text ([Phase 0](#phase-0-foundations))
- M0.9a2 The site's pages ([Phase 0](#phase-0-foundations))
- M0.9b The design audit ([Phase 0](#phase-0-foundations))
- M0.9c The CLI, exports and plot to the design ([Phase 0](#phase-0-foundations))
- M0.9c1 Diagnostics on stderr ([Phase 0](#phase-0-foundations))
- M0.9c2 US units in brackets ([Phase 0](#phase-0-foundations))
- M0.9c3 How far to trust a result ([Phase 0](#phase-0-foundations))
- M0.9c4 Units and trust notes ([Phase 0](#phase-0-foundations))
- M0.9c5 Provenance ([Phase 0](#phase-0-foundations))
- M0.9c6 Errors and typed numbers ([Phase 0](#phase-0-foundations))
- M0.9c7 The plot's type and spacing ([Phase 0](#phase-0-foundations))
- M0.9c8 Provenance on every export ([Phase 0](#phase-0-foundations))
- M0.9c9 Help, roles and progress ([Phase 0](#phase-0-foundations))
- M0.9c12 The plot's balloons, banking and title block ([Phase 0](#phase-0-foundations))
- M0.9c13 The site's figures and units ([Phase 0](#phase-0-foundations))
- M0.9c14 Provenance on the other files ([Phase 0](#phase-0-foundations))
- M0.9c15 Next steps and waits ([Phase 0](#phase-0-foundations))
- M0.9c16 US units on the model pages ([Phase 0](#phase-0-foundations))
- M0.9d1 The figures in tokens ([Phase 0](#phase-0-foundations))
- M0.9d2 The site's colors, corners and motion ([Phase 0](#phase-0-foundations))
- M0.9d3 The page's frame ([Phase 0](#phase-0-foundations))
- M0.9d4 The head, the keyboard and forced colors ([Phase 0](#phase-0-foundations))
- M0.9d5 The notes ([Phase 0](#phase-0-foundations))
- M0.9d6 The site's chrome ([Phase 0](#phase-0-foundations))
- M0.9d7 The page's frame ([Phase 0](#phase-0-foundations))
- M0.9d8 Print and the text marks ([Phase 0](#phase-0-foundations))
- M1.1 Core math, frames, Earth ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.2 Atmosphere and wind ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.3 Solid motors ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.4 Design model and mass properties ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.4a Shapes, materials and component mass properties ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.4b Design tree, configurations and checks ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.5 Aerodynamics I (subsonic) ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.5a Normal force and centre of pressure ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.5b Drag and override tables ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.6 6-DOF flight engine ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.6a Integrator and events ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.6b Rigid-body flight ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.7 Recovery ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.7a Parachutes and descent ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.7b Streamers and tumble ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.7c Separated bodies ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M2.1 Validation harness plus the RocketPy code-to-code suite ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M2.1a The harness ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M2.1b Whole flights against RocketPy, same-drag ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M2.1b1 The whole-flight oracle ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M2.1b2 The whole-flight cases ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M2.1c Predicted mode, CI and regeneration ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M2.1c1 The CI job and the regeneration workflow ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M2.1c2 Predicted mode ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M2.1d The time-series RMS and the path in wind ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M2.1d1 The time-series RMS ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M2.1d2 The calm-air cases (issue #50) ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M2.1d3 The path in wind (issue #50) ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.8 Aerodynamics II (transonic and supersonic, damping, overrides) ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.8a Normal force and centre of pressure through Mach 1 ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.8b Transonic and supersonic drag ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.8b1 The drag buildup through Mach 1 ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.8b2 Drag against RASAero through Mach 2 ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.8b3 The boattail and base faster than sound ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.8c Roll and damping ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.8d Normal-force overrides ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.8e The body's supersonic normal force ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.8e1 The second-order shock-expansion method ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.8e2 The body's supersonic normal force in flight ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.8e3 The supersonic join's start without grid steps ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.8e4 The boattail's share faster than sound ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.8e5 The remaining gap, source by source ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.8e6 Crossflow and the boattail faster than sound ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.8e7 Blunt tips faster than sound ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.8e8 The lip faster than sound ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.8e9 #90's boattail cap, and M1.8e's 15% bullet judged ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.8e10 The lip's shelter, weighed not switched ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.8e11 Cone slopes past Fig. 2's edge, from Sims ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.8e12 What the handover's cap is worth, and what stops it ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.8e13 What the answer follows when it follows the mesh ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.8e14 Where the flare's march stops ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.8e17 The flare through the method ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.8e18 What a marched flare is worth ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.8e15 The step in radius ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.8e19 The near-flat flare the march refuses ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M3.1 OpenRocket `.ork` import ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M3.1a The container and the design document ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M3.1b The component tree ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M3.1b1 The values inside the tags ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M3.1b2 The spine ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M3.1b3 The parts on and inside the body ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M3.1b4 The designs that still do not lay out ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M3.1c Motors, recovery, stages and what OpenRocket last did ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M3.1c1 Motors and their configurations ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M3.1c2 Recovery and separation ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M3.1c3 What OpenRocket last did ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M3.1c4 Pods, parallel stages and the rest ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M3.1d The corpus and the cross-check ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M3.1d1 Snapshots of public designs ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M3.1d2 The cross-check ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M2.2 OpenRocket oracle and corpus ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M2.2a Structure mass, CG and inertia ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M2.2b OpenRocket's mass conventions ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M2.2b1 What a `.ork` leaves unsaid, and overrides ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M2.2b2 Fins, rail buttons and roll inertia ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M2.2b3 Packed parts ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M2.2b4 Clusters, fillets and unread parts ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M2.2b5 Stored results as found ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M2.2c The motors OpenRocket flies ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M2.2c1 Every curve as OpenRocket integrates it ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M2.2c2 The configurations held back for want of a curve ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M2.2d Flights to apogee on the public designs ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M2.2d1 OpenRocket's flights, and what its words mean ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M2.2d2 hpr's flights against the record ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M2.2e The corpus ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M2.2e1 Mass and CG in the flight report ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M2.2e2 OR's flights of the corpus ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M2.2e3 hpr's flights of the corpus ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M2.2e4 The causes ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M2.2e5 A tilted rod ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M2.2e6 The old override flag ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M2.2e7 Fin fillets and an inner tube's radius ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M2.2e8 Tube fins OpenRocket sizes ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M2.2e9 Tube fin aerodynamics ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M2.2e10 Twenty designs ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M2.2f Lessons L19 and L82 ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.9 Staging, clusters, airstarts (COTS) ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.9a Ignition times and powered staging ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.9b Clusters ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.9c Against OpenRocket ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.10 Outputs and derived metrics ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.10a Flight metrics ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.10b Fin flutter ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.10c Exports ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.10c1 Text exports ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.10c2 Parquet ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M2.3a ERA5 weather ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M2.3b RocketPy's logged flights ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M2.3c1 Logged apogees ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M2.4 Accuracy census gate ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.11 Ejected sections and payloads ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.11a Pieces at any joint ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.11b Ejection impulse and tumbling pieces ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.12 Mass that moves or leaves in flight ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.12a A mass that moves along the airframe ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.12b Mass released in flight ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.13 Pods ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.13a Pod mass ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.13b `.ork` pods ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.13b1 Pods of body components ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.13b2 Pods of no length ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.13c Pod aerodynamics ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.13c1 Pods fly ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.13c2 A pod design against OpenRocket ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.14a Warnings ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.14a1 Envelope flags ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.14a2 Drag issue warnings ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M1.14a3 Stability issue warnings ([Phase 1](#phase-1-physics-core-the-heart-with-validation-interleaved))
- M4.1 Facade API ([Phase 2](#phase-2-library-surfaces-and-interop))
- M4.1a The builder ([Phase 2](#phase-2-library-surfaces-and-interop))
- M4.1b Custom models and the rustdoc guide ([Phase 2](#phase-2-library-surfaces-and-interop))
- M4.2 CLI ([Phase 2](#phase-2-library-surfaces-and-interop))
- M4.2a The command surface and `hpr motors` ([Phase 2](#phase-2-library-surfaces-and-interop))
- M4.2b `hpr sim` ([Phase 2](#phase-2-library-surfaces-and-interop))
- M4.2c `hpr validate` and `hpr convert` ([Phase 2](#phase-2-library-surfaces-and-interop))
- M4.2d `hpr analyze` ([Phase 2](#phase-2-library-surfaces-and-interop))
- M3.2 OpenRocket `.ork` export ([Phase 2](#phase-2-library-surfaces-and-interop))
- M3.2a The writer ([Phase 2](#phase-2-library-surfaces-and-interop))
- M3.2b OpenRocket flies the export ([Phase 2](#phase-2-library-surfaces-and-interop))
- M3.3 The hpr open design format v0.1 ([Phase 2](#phase-2-library-surfaces-and-interop))
- M3.3a The document ([Phase 2](#phase-2-library-surfaces-and-interop))
- M3.3b The container, migrations and the comparison ([Phase 2](#phase-2-library-surfaces-and-interop))
- M3.3c Generated types ([Phase 2](#phase-2-library-surfaces-and-interop))
- M4.3 Python bindings ([Phase 2](#phase-2-library-surfaces-and-interop))
- M4.3a The package ([Phase 2](#phase-2-library-surfaces-and-interop))
- M4.3b RocketPy's example ([Phase 2](#phase-2-library-surfaces-and-interop))
- M4.3c Python models ([Phase 2](#phase-2-library-surfaces-and-interop))
- M5.1 Online layer and cache ([Phase 2](#phase-2-library-surfaces-and-interop))
- M5.1a Cache and offline mode ([Phase 2](#phase-2-library-surfaces-and-interop))
- M5.1b HTTP ([Phase 2](#phase-2-library-surfaces-and-interop))
- M5.2 Weather ([Phase 2](#phase-2-library-surfaces-and-interop))
- M5.2a Open-Meteo ([Phase 2](#phase-2-library-surfaces-and-interop))
- M5.2b U. Wyoming soundings ([Phase 2](#phase-2-library-surfaces-and-interop))
- M5.2c GFS/RAP GRIB2, pure Rust ([Phase 2](#phase-2-library-surfaces-and-interop))
- M5.2d Files the user provides, and `hpr weather` ([Phase 2](#phase-2-library-surfaces-and-interop))
- M5.2d1 `hpr weather` ([Phase 2](#phase-2-library-surfaces-and-interop))
- M5.2d2 Complex packing ([Phase 2](#phase-2-library-surfaces-and-interop))
- M5.2d3 JPEG 2000 ([Phase 2](#phase-2-library-surfaces-and-interop))
- M5.3 Site data ([Phase 2](#phase-2-library-surfaces-and-interop))
- M5.3a WMM2025 ([Phase 2](#phase-2-library-surfaces-and-interop))
- M5.3b Elevation ([Phase 2](#phase-2-library-surfaces-and-interop))
- M5.3c Geodetic helpers and a user's elevation file ([Phase 2](#phase-2-library-surfaces-and-interop))
- M5.3c1 Geodesics ([Phase 2](#phase-2-library-surfaces-and-interop))
- M5.3c2 A user's GeoTIFF ([Phase 2](#phase-2-library-surfaces-and-interop))
- M5.4 Motor stock and prices ([Phase 2](#phase-2-library-surfaces-and-interop))
- M5.4a The motor finder's API ([Phase 2](#phase-2-library-surfaces-and-interop))
- M5.4b ThrustCurve ([Phase 2](#phase-2-library-surfaces-and-interop))
- M5.4c `hpr motors search` ([Phase 2](#phase-2-library-surfaces-and-interop))
- M5.5 Parts catalog ([Phase 2](#phase-2-library-surfaces-and-interop))
- M5.5a The `.orc` reader ([Phase 2](#phase-2-library-surfaces-and-interop))
- M5.5b Catalog parts in the builder ([Phase 2](#phase-2-library-surfaces-and-interop))
- M4.5 Fly my .ork ([Phase 2](#phase-2-library-surfaces-and-interop))
- M4.5a Recovery from a `.ork` ([Phase 2](#phase-2-library-surfaces-and-interop))
- M4.5b Motors on demand ([Phase 2](#phase-2-library-surfaces-and-interop))
- M4.5c Design checks that match reality ([Phase 2](#phase-2-library-surfaces-and-interop))
- M4.5d Readable output ([Phase 2](#phase-2-library-surfaces-and-interop))
- M4.5e Plots ([Phase 2](#phase-2-library-surfaces-and-interop))
- M4.5f How-to guides ([Phase 2](#phase-2-library-surfaces-and-interop))
- M4.5g Then by blocked count ([Phase 2](#phase-2-library-surfaces-and-interop))
- M4.5g1 A powered separation in `hpr sim` ([Phase 2](#phase-2-library-surfaces-and-interop))
- M4.5g2 Several separations ([Phase 2](#phase-2-library-surfaces-and-interop))
- M4.5g3 An unpowered separation before apogee ([Phase 2](#phase-2-library-surfaces-and-interop))
- M4.5g4 Freeform fins ([Phase 2](#phase-2-library-surfaces-and-interop))
- M4.5h A part's drag override ([Phase 2](#phase-2-library-surfaces-and-interop))
- M4.5i A powered pods example ([Phase 2](#phase-2-library-surfaces-and-interop))
- M4.5j The CLI's corpus count ([Phase 2](#phase-2-library-surfaces-and-interop))
- M4.5k A ring around its tube ([Phase 2](#phase-2-library-surfaces-and-interop))
- M4.5l A packed part wider than its bore ([Phase 2](#phase-2-library-surfaces-and-interop))
- M4.5m Parallel booster staging ([Phase 2](#phase-2-library-surfaces-and-interop))
- M4.5n Pods--powered's first configuration ([Phase 2](#phase-2-library-surfaces-and-interop))
- M4.5o Base drag hack's E12-4 ([Phase 2](#phase-2-library-surfaces-and-interop))
- M4.6 Monte Carlo from the CLI and Python ([Phase 2](#phase-2-library-surfaces-and-interop))
- M4.6a `hpr mc` ([Phase 2](#phase-2-library-surfaces-and-interop))
- M4.6b Monte Carlo in Python ([Phase 2](#phase-2-library-surfaces-and-interop))
- M6.1 Monte Carlo and sensitivity ([Phase 3](#phase-3-uncertainty-optimization-challenges))
- M6.1a Seeded dispersion ([Phase 3](#phase-3-uncertainty-optimization-challenges))
- M6.1b Landing ellipses ([Phase 3](#phase-3-uncertainty-optimization-challenges))
- M6.1c Sensitivity ([Phase 3](#phase-3-uncertainty-optimization-challenges))
- M6.1d 10,000 flights ([Phase 3](#phase-3-uncertainty-optimization-challenges))
- M6.2a CMA-ES, continuous variables ([Phase 3](#phase-3-uncertainty-optimization-challenges))
- M6.2b Discrete variables and constraints ([Phase 3](#phase-3-uncertainty-optimization-challenges))
- M6.2b1 Constraints ([Phase 3](#phase-3-uncertainty-optimization-challenges))
- M6.2b2 Discrete choices ([Phase 3](#phase-3-uncertainty-optimization-challenges))
- M6.2c NSGA-II ([Phase 3](#phase-3-uncertainty-optimization-challenges))
- M6.2d EGO ([Phase 3](#phase-3-uncertainty-optimization-challenges))
- M6.2d1 Branin and Hartmann 3 ([Phase 3](#phase-3-uncertainty-optimization-challenges))
- M6.2d2 Hartmann 6 ([Phase 3](#phase-3-uncertainty-optimization-challenges))
- M10.1 Release 0.1: the simulator ([Phase 8](#phase-8-releases-adr-162))
- M10.1a The flight and design fixes ([Phase 8](#phase-8-releases-adr-162))
- M10.1b The data, network and Python fixes ([Phase 8](#phase-8-releases-adr-162))
- M10.1c The release workflow ([Phase 8](#phase-8-releases-adr-162))
- M10.1d1 The release notes and the install pages ([Phase 8](#phase-8-releases-adr-162))
- M10.1d2 The safety warning and the shape warnings ([Phase 8](#phase-8-releases-adr-162))
- M10.1d3 The friction and freeform fin warnings ([Phase 8](#phase-8-releases-adr-162))
- M10.1d5 The separated parts' warnings ([Phase 8](#phase-8-releases-adr-162))
- M10.1d6 The unknown signs ([Phase 8](#phase-8-releases-adr-162))
- M10.1d7 The designation and the design pin ([Phase 8](#phase-8-releases-adr-162))
- M10.1d8 The new name's URLs ([Phase 8](#phase-8-releases-adr-162))
- M10.1d9 FusionSpace HPR ([Phase 8](#phase-8-releases-adr-162))
- M10.1d4 The checklist ([Phase 8](#phase-8-releases-adr-162))

## Phase 0: Foundations

- [x] **M0.1 Workspace, CI, licenses:** the workspace from `ARCHITECTURE.md` (edition 2024, pinned
  stable, shared lints and dependencies); dual licenses and notices; `deny.toml` denying copyleft;
  `xtask wasm-check`; CI (fmt, clippy, test on three OSes, doc, wasm-check, deny) on **every** PR,
  no `paths-ignore`; README "pre-alpha"; `.gitignore` covers `refs/`, `corpus/` and local state.
  *Done when:* the local gate passes, CI is green on all three operating systems, `cargo deny
  check` passes, and ADR-001 records the license choice and workspace layout. *Met.*
- [x] **M0.2 Reference library:** `cargo xtask refs fetch|verify|doctor`, driven by
  `validation/refs.lock.toml`, populates `refs/` with RocketPy (pinned tag), the OpenRocket 24.12
  jar (sha256 pinned), the public PDFs listed in `VALIDATION.md`, `openrocket-database` (pinned
  commit), ThrustCurve and motor.fusionspace.co snapshots, and `fusionspace-loft` plus the private
  `loft-fixtures` repo (skipped with a note when unavailable, as in CI); a `uv`-managed Python venv
  in `refs/venv` with `rocketpy==1.13.0` and JPype; a Java 17 check.
  *Done when:* `fetch` is idempotent, `verify` checks every hash, `doctor` prints which oracles are
  runnable, `git status` shows nothing from `refs/`, and `THIRD-PARTY-NOTICES.md` lists every
  source with its license and usage mode (bundled, fetched or run-only). *Met.*
- [x] **M0.3 Lessons from Loft.** Read `refs/fusionspace-loft` and write
  `docs/research/loft-lessons.md` (at most 200 lines): the models used, known weaknesses, importer
  quirks, test cases worth porting, and process mistakes to avoid. *Done when:* the file exists, and
  every quirk it lists maps to a roadmap milestone or a named test to write.
- [x] **M0.4 A documentation site people can read.** Added by Neer on 2026-09-17 (VISION V15,
  the rule that docs are a deliverable). Retrofit everything shipped so far; later milestones
  keep the site current. Tool and layout by ADR (mdBook first); each page has one source, equations
  render on the site and on GitHub, and workspace rustdoc is published next to the guide, each
  linking the other. Pages: *Start here*; *Getting started* (a runnable first flight); *How a
  flight is simulated*; one page per model, each opening with *In short*; *Accuracy* (every result,
  gaps included); *Glossary*; *Checking a claim*; the decisions; the roadmap.
  *Done when:* split on 2026-09-17 into M0.4a to M0.4e, which carry its four done-when bullets
  unchanged. Done 2026-09-18, with M0.4d's first deploy.
  - [x] **M0.4a The site and its link checks** (ADR-016 to ADR-018): CI builds the site on every PR,
    and a broken link or a bare internal label fails it. *Met.*
  - [x] **M0.4b Model pages, Accuracy, Glossary, Checking a claim**: a model page without *In short*
    fails CI, and *Accuracy* gives every result so far from the committed report. *Met.*
  - [x] **M0.4c Getting started, and how a flight is simulated**: the first-flight example runs in
    CI, and the page walks pad to landing with a diagram. *Met.*
  - [x] **M0.4d Publish** (ADR-019): a workflow deploys the site and the rustdoc to GitHub Pages
    from `main`. *Met* 2026-09-18; Neer turned Pages on, CI run 35396233336 deployed it.
  - [x] **M0.4e The reader test**: a reader new to the project answers ten questions from the site
    alone, citing a page each. *Met.*
- [x] **M0.5 Leaner bookkeeping** (ADR-144 §1). Added on 2026-10-03; runs first. It
  adds an explicit queue order that the checks follow: after M0.5, M6.2d2 (one attempt), M4.5,
  M1.14, M6.2e, then M6.3 onward in file order. *Done when:* ADR-144 §1's a to i all hold.
  Split on 2026-10-04 into M0.5a to M0.5i, one per item of ADR-144 §1, each carrying its item
  unchanged as its *done when* (ADR-146).
  - [x] **M0.5a One file per decision record.** *Done when:* ADR-144 §1a holds.
  - [x] **M0.5b A roadmap of open work, in queue order.** *Done when:* ADR-144 §1b holds.
  - [x] **M0.5d Generated mirrors.** *Done when:* ADR-144 §1d holds.
  - [x] **M0.5c Short planning notes.** *Done when:* ADR-144 §1c holds.
  - [x] **M0.5e Reviews while CI runs.** *Done when:* ADR-144 §1e holds.
  - [x] **M0.5f Shorter planning files.** *Done when:* ADR-144 §1f holds.
  - [x] **M0.5g The gate runs the `refs/` checks.** *Done when:* ADR-144 §1g holds.
  - [x] **M0.5h The checks enforce the new rules.** *Done when:* ADR-144 §1h holds.
  - [x] **M0.5i The first 20 pull requests against the baseline.** *Done when:* ADR-144 §1i holds.
    *Met* 2026-10-05: a report compared the 20 pull requests after M0.5 with the 21 before it,
    by a comparison script and a repeated review sample.
- [x] **M0.6 The FusionSpace product system** (ADR-164; VISION V40). The CLI, docs site, plot and
  exports to fusionspace-design's `product/` at the pinned Rev A. *Done when:* a to e hold.
  - [x] **M0.6a The docs site.** *Done when:* `cargo xtask site` builds with Rev A's mdBook theme,
    its fonts served by the site under hpr's own `@font-face` rules (no request leaves it), the
    strip on every page, and a title block (designation, version, date, status, units, data
    sources) ending the landing page; copied files keep their SPDX lines.
  - [x] **M0.6b The CLI and exports.** *Done when:* `--color` and `NO_COLOR`, `FORCE_COLOR`,
    `CLICOLOR_FORCE` follow `cli.md`'s order per stream, each case tested; piped and `--json`
    output hold no escape code; every hint hpr writes is a `help:` line; `hpr --version`, every
    file `hpr` writes and every `--json` document name the tool, version and `FS · SW · TOOL 005`.
  - [x] **M0.6c The plot.** *Done when:* `hpr sim --plot` draws in token colors only (a test lists
    them), every simulated series dashed in the predicted color, events dotted with numbered
    balloons and a table, ground a chain line, unit-bearing axis titles, no legend box, and an SVG
    `<desc>` giving apogee, its time and the top speed.
  - [x] **M0.6d US spelling, no em dashes.** *Done when:* a CI check over `docs README.md crates
    xtask schema python scripts` finds no em dash and no form of centre, metre, catalogue,
    licence, colour, analyse, behaviour or modelling outside code identifiers, frozen records
    (`docs/decisions/`, `docs/roadmap-done.md`), quotations and source titles; it prints how many
    exceptions it allows.
    - [x] **M0.6d1 The check, and the pages.** *Done when:* `cargo xtask spelling`, run by CI's
      `site` step and `cargo test -p xtask`, finds no em dash and no form of d's words in `docs
      README.md python scripts` outside code identifiers, fenced code blocks, frozen records,
      quotations and source titles, and prints how many exceptions it allows (ADR-176).
    - [x] **M0.6d2 The crates' words** (ADR-177). *Done when:* the check's scope is d's whole
      list, adding `crates xtask schema` and the pages' fenced code blocks, with no UK spelling
      found, and it prints how many em dashes it leaves there for d3.
    - [x] **M0.6d3 The crates' em dashes** (ADR-177). *Done when:* the same check's scope is d's
      whole list, adding `crates xtask schema` and the pages' fenced code blocks, with no findings
      and no em dash left for later.
  - [x] **M0.6e US names.** *Done when:* no public Rust, Python, JSON or schema name holds a word on
    d's list, and design files written with the old keys still read (a test).
- **M0.7** is open; its entry: [roadmap](ROADMAP.md#phase-0-foundations).
  - **M0.7a** is open; its entry: [roadmap](ROADMAP.md#phase-0-foundations).
    - [x] **M0.7a1 The no-clipping checks** (ADR-204). *Done when:* M0.7a's three checks fail as
      stated, each with its test; CI runs them on every SVG under `docs/`, each `--plot` that
      `xtask cli` writes and every page of the site; all pass, fixed by layout, not cropping.
- **M0.9** is open; its entry: [roadmap](ROADMAP.md#phase-0-foundations).
  - [x] **M0.9a The name everywhere,** before 0.1 publishes. *Done when:* `cargo xtask names` fails
    on `hpr-sim` (any case, U+2011 too), or a lowercase `hpr` outside code formatting, on every
    surface ADR-205 §1 lists, the published package text included, outside an allowlist of
    narrow, reasoned entries that must each match; tests plant a mention per surface kind, drop
    an entry and widen one to a page, each failing; a test fails unless the landing page's
    first paragraph names FusionSpace HPR and each shipped product.
    - [x] **M0.9a1 The packages' text** (ADR-206). *Done when:* M0.9a's check, allowlist rules
      and three tests hold on every ADR-205 §1 surface but the site's pages, the landing page
      included, with its first-paragraph test.
    - [x] **M0.9a2 The site's pages** (ADR-206, ADR-207). *Done when:* the check covers every page
      `SUMMARY.md` renders, the records page's rows inheriting only their linked ADR's
      allowance (a test each way), so M0.9a is met.
  - [x] **M0.9b The design audit.** *Done when:* `docs/research/design-conformance.md` names the
    pinned commit and rows every `##` section of the 14 `product/` files as ADR-205 §2 says,
    each applying row met (a named test, or reviewed at the commit) or in an issue with a
    milestone; a test fails on a missing section or a commit differing from `refs.lock.toml`'s or
    `DESIGN_REV`.
  - [x] **M0.9c The CLI, exports and plot to the design** (ADR-208): #377–#382, #392, #395, #400,
    #403, #405, #409–#411, each fix held by a named test failing on its issue's defect, the
    audit's rows naming them; a rule declined has an ADR.
    - [x] **M0.9c1 Diagnostics on stderr** (#377). *Done when:* every `warning:`, `note:` and
      `help:` line a command prints beside a text result goes to stderr, and the result alone to
      stdout, held by a test per command that prints them; `--json` output is unchanged; the
      docs say so; the audit's *principles.md · 8* row is met by those tests.
    - [x] **M0.9c2 US units in brackets** (#378, ADR-210). *Done when:* #378 is closed, with
      every item it lists: feet and feet per second in brackets wherever `hpr sim` and `hpr mc`
      print a height or speed, and on the plot's axes; `hpr motors show` gives the class's
      impulse range and the propellant's name; the summary gives the CG and CP stations.
    - [x] **M0.9c3 How far to trust a result** (#379, ADR-211). *Done when:* #379 is closed, with every
      item it lists: a trust note with the report's spread on `hpr sim`'s and `hpr mc`'s text
      and on the plot; headline values labelled simulated; one trust-note shape on the site.
    - [x] **M0.9c4 Units and trust notes** (#392, #395). *Done when:* both closed: US units in
      brackets in `weather`, `motors list`, `search`, `fetch`, `analyze` (g); a trust note (in
      `--json` too) on `weather`, `analyze`, `motors show`.
    - [x] **M0.9c5 Provenance** (#380; ADR-214). *Done when:* it is closed, with every item it lists, the
      exports' trust note included.
    - [x] **M0.9c6 Errors and typed numbers** (#381's *Errors* and *Typed numbers*; ADR-215).
      *Done when:* each of those items is met with a test: `hpr sim … --latitude 95` names
      `--latitude`, the degrees typed and the range, with a `help:`; a missing file says what to
      do, naming the closest file in its folder; `hpr motors show J350` names what answers J350;
      every numeric option reads ` 2.0 ` and `1,280`, the second shown as read with its unit, and
      refuses `3,9` with a question; the audit's rows name the tests.
    - [x] **M0.9c7 The plot's type and spacing** (#382's type, spacing and units; ADR-216). *Done when:* the
      plot's balloon numerals are 12 px, its title is the `subtitle` token (20 px, weight 600), its
      caption, line and marker spacing sit on the 4 px token scale, its time axis title sits at the
      axis's end, and its caption and notes join each number to its unit with a non-breaking
      space; each held by a plot test that fails on the build before it.
    - [x] **M0.9c8 Provenance on every export** (#403; ADR-214). *Done when:* it is closed, with
      every item it lists, each held by a test that fails on the build before it.
    - [x] **M0.9c9 Help, roles and progress** (#381, #405). *Done when:* both are closed, with
      all their items.
    - [x] **M0.9c12 The plot's balloons, banking and title block** (#382; ADR-218). *Done when:*
      colliding balloons step right on a leader, panels bank toward 45°, a title block and a CSV
      end the figure; each held by a test failing before it.
    - [x] **M0.9c13 The site's figures and units** (#382, #400; ADR-216, ADR-219). *Done when:*
      #382 is closed, and #400 on the guides: a site check fails a height, distance or speed in
      SI alone on every page but those ADR-219 defers.
    - [x] **M0.9c14 Provenance on the other files** (#409; ADR-214). *Done when:* it is closed,
      each item held by a test failing on the build before it.
    - [x] **M0.9c15 Next steps and waits** (#410, #411). *Done when:* both are closed, each item
      held by a check failing before it.
    - [x] **M0.9c16 US units on the model pages** (#400; ADR-219). *Done when:* #400 is closed,
      with every item it lists: the units check defers no page and reads millimeters, a motor's
      or mount's size excepted.
  - **M0.9d** is open; its entry: [roadmap](ROADMAP.md#phase-0-foundations).
    - [x] **M0.9d1 The figures in tokens** (#385). *Done when:* #385 is closed: `flight-phases.svg`
      in the system's line types and fonts, the badges in token fills and Cascadia Mono, and
      `xtask figures` failing on a color that isn't a token, a blend, the default black or another
      font, in every committed SVG and each `--plot` figure.
    - [x] **M0.9d2 The site's colors, corners and motion** (#383, ADR-221). *Done when:* the
      colors, radii, shadows and motion #383 lists that mdBook's stylesheets give the screen are
      gone, in light and navy and with reduced motion set, each failing `xtask site`'s page check
      when it comes back.
    - [x] **M0.9d3 The page's frame** (#384's type, measure and gutters; ADR-223). *Done when:*
      `xtask site`'s page check fails, on every page at every width, text off the type scale
      (size, line height, family, capitals), Archivo without tabular figures, navigation not in
      Cascadia Mono 14 px or a current page not underlined 2 px, a prose line past 68 characters,
      and gutters other than 16 px below 720 px and 32 px from it; a canary each; the site passes.
    - [x] **M0.9d4 The head, the keyboard and forced colors** (#383, ADR-224). *Done when:* the
      head's metas, the focus ring, forced colors, contrast, the sticky bar's scroll padding,
      the system's `fonts.css` and the script's smooth scroll each fail `xtask site` or a site
      test when mdBook's default comes back.
    - [x] **M0.9d5 The notes** (#384, ADR-225). *Done when:* every quote block a page writes is
      drawn as the system's note, its signal word in a strip, and the page check fails a quote
      block or a note off the system's shape, each with a canary.
    - [x] **M0.9d6 The site's chrome** (#383, ADR-228). *Done when:* `xtask site` fails a Font
      Awesome glyph, an icon not the system's, a second `h1`, a favicon not the system's, an
      icon off 16, 20 or 24 px and a selection off the action role, each with a test or canary.
    - [x] **M0.9d7 The page's frame** (#384, ADR-229). *Done when:* every built page has a
      `header` with the one-color lockup, no menu button from 720 px with the sidebar shown and
      reachable there, and ends with the title block in a `footer`, each failing `xtask site`'s
      file check or page check (with canaries) when broken.
    - [x] **M0.9d8 Print and the text marks** (#383, ADR-230). *Done when:* #383 is closed, each
      class of mdBook default it lists failing `xtask site` or a site test when it comes back.
## Phase 1: Physics core (the heart), with validation interleaved

- [x] **M1.1 Core math, frames, Earth.** `hpr-core`: vectors and quaternions (glam f64) and
  interpolation tables (linear/cubic, clamped, with extrapolation flags); the frames spec in
  `docs/physics/frames.md` (ENU launch frame, body frame, Euler conventions, geodetic/ECEF); WGS84
  Somigliana gravity with altitude, optional Earth-rotation terms.
  - Loft lessons: L1.
  *Done when:* gravity matches the published formula values at at least 6 latitude/altitude points
  to 1e-6 relative; frame round-trip property tests pass; quaternion integration keeps the norm
  within 1e-12 over 1e6 steps; everything compiles for wasm32.
- [x] **M1.2 Atmosphere and wind.** USSA76 from 0 to 86 km (temperature, pressure, density, speed
  of sound, dynamic viscosity by Sutherland); ISA temperature offset; custom profiles from
  soundings (p, T, RH, wind vs height); wind models constant, power/log law, tabulated layers and
  seeded Dryden turbulence.
  - Loft lessons: L2, L3, L4, L5, L6.
  *Done when:* USSA76 matches the tables at at least 25 altitudes to at most 0.1% (the small table
  fixture is committed with its citation); a Dryden spectrum test passes (PSD within tolerance of
  theory); and `docs/physics/atmosphere.md` cites every equation.
- [x] **M1.3 Solid motors:** `.eng` and `.rse` readers and writers (clean room, from the public
  specs); thrust(t), propellant mass by impulse fraction (default) with an optional grain-geometry
  model, CG and inertia over time, nozzle exit area; delays (including plugged) and case/retainer
  mass; an offline catalog type with per-curve provenance and license, bundling only curves with
  clear terms, the rest cached later (M5).
  - Loft lessons: L36, L37, L38, L39, L40, L41, L42, L43.
  *Done when:* for every bundled curve, total impulse, average thrust and burn time match the
  ThrustCurve metadata within 1%; parse-write-parse round trips are identical; mass and inertia
  evolution matches RocketPy's SolidMotor for 3 motors within 1% (reference JSON generated by a
  script in `validation/oracles/rocketpy/`).
- [x] **M1.4 Design model and mass properties:** `hpr-design` components — nose cones (conical,
  tangent/secant ogive, elliptical, power series, parabolic series, Haack/LV-Haack, von Kármán),
  body tubes, transitions, couplers, fin sets (trapezoidal, elliptical, freeform, tube fins), launch
  lugs, rail buttons, inner tubes, motor mounts, centering rings, bulkheads, mass components,
  parachutes, streamers, shock cords; cited clean-room materials; stages and configurations; mass,
  CG and full inertia tensor from geometry, with overrides; structural checks with typed warnings; a
  small public test-design set, `validation/designs/`, so tests never snapshot the private corpus.
  *Done when:* analytic volume, area and CG tests pass for every shape; the inertia tensor of
  composite test bodies matches hand calculations; mass, CG and inertia match RocketPy's example
  rockets where RocketPy exposes them; the OpenRocket stored-value comparison is deferred to M2.2
  and noted there.
  - [x] **M1.4a Shapes, materials and component mass properties:** every nose and transition shape
    above (clipped or not), solids of revolution filled or with a wall, fin planforms,
    cross-sections and tabs, and every other component listed, each with mass, CG and full inertia
    tensor from geometry in its own frame; cited materials; `MassProperties` with the parallel-axis
    theorem and rotations.
    - Loft lessons: L44, L45, L46, L48, L49, L91.
    *Done when:* analytic volume, area and CG tests pass for every shape, and the inertia tensor
    of composite test bodies matches hand calculations.
  - [x] **M1.4b Design tree, configurations and checks:** stages and component placement,
    configurations with motors, overrides, reference diameter, structural checks with typed
    warnings, and `validation/designs/`.
    - Loft lessons: L47, L50.
    *Done when:* mass, CG and inertia match RocketPy's example rockets where RocketPy exposes
    them, and the OpenRocket stored-value comparison is deferred to M2.2 and noted there.
- [x] **M1.5 Aerodynamics I (subsonic):** Barrowman CNα and CP for every component with
  Prandtl–Glauert, body lift at angle of attack and fin–body interference; the drag buildup — skin
  friction (laminar/turbulent with roughness), nose/transition pressure drag, base drag power-on
  and power-off, fin profile and thickness, protuberances — and Cd at angle of attack; Cd-vs-Mach
  override tables (power-on/off) from CSV, including RocketPy/RASAero exports, so the dynamics are
  validated on the oracle's drag first. `docs/physics/aero.md` cites each term.
  *Done when:* split below into M1.5a and M1.5b, which carry its three bullets unchanged.
  *Result:* see M1.5a (the Recruiter's six-fin slopes, ADR-008) and M1.5b (Valetudo, ADR-009).
  Skin friction is fully turbulent with roughness, as in Niskanen; laminar and transitional
  friction were not built (ADR-009).
  - [x] **M1.5a Normal force and centre of pressure:** Barrowman CNα and CP for every component
    with Prandtl–Glauert, body lift at angle of attack, fin–body interference, and the `aero.md`
    sections for them.
    - Loft lessons: L8, L9, L10, L89.
    *Done when:* CNα and CP reproduce Barrowman's worked example(s) within 1%.
  - [x] **M1.5b Drag and override tables:** the drag buildup, Cd at angle of attack, Cd-vs-Mach
    override tables from CSV, and the `aero.md` sections for them.
    - Loft lessons: L11, L12, L13, L14, L15, L16, L90.
    *Done when:* subsonic Cd for the RocketPy example rockets is within 10% of their RASAero CSVs
    at Mach 0.3 (tighten this later), and unit tests cover every drag term's limits.
    *Result (ADR-009):* not met for Valetudo (−47% power-off, −50% power-on; its table is 1.44 times
    its own OpenRocket export, which hpr matches to 2%) or Cavour power-on (−18.3%, cause open).
    Calisto, Juno III and Cavour power-off are within 10% (+4.4%, −6.0%, −8.3%) under a declared
    input rule that can't pin the unrecorded inputs.
- [x] **M1.6 6-DOF flight engine:** state — position, velocity, attitude quaternion, angular
  velocity, time-varying mass properties; a guided rail/tower phase with friction and rail-button
  geometry; powered and coast phases with jet damping; adaptive Dormand–Prince 5(4) with dense
  output and event root-finding (liftoff, rail exit, burnout, apogee, ground hit, user events), a
  fixed-step RK4 option; a recorder with a configurable channel set, an observer trait and a
  `criterion` benchmark. *Done when:* its four bullets, which M1.6a and M1.6b carry word for word.
  - [x] **M1.6a Integrator and events:** adaptive Dormand–Prince 5(4) with dense output, event
    root-finding and stop times that put discontinuities on step boundaries; the fixed-step RK4
    option; `docs/physics/integration.md`. *Done when:* step-halving convergence shows the
    expected order, and events are located to ≤1e-6 s.
    - Loft lessons: L21, L22, L23.
  - [x] **M1.6b Rigid-body flight:** the state, the rail phase, powered and coast phases with jet
    damping, the flight events, the recorder and observer, and the `criterion` benchmark. *Done
    when:* analytic tests pass (vacuum ballistic, terminal velocity, torque-free precession, and
    pitch oscillation frequency vs linear theory), and a single typical L2 flight simulates in
    ≤5 ms release-mode (number recorded in `docs/perf.md`).
    - Loft lessons: L20, L24, L25, L26.
- [x] **M1.7 Recovery.** Parachutes (Cd·S, inflation time or area growth), streamers and tumble;
  drogue and main with their triggers; descent with wind drift, separated bodies tracked
  independently, landing detection. *Done when:* M1.7a's, word for word. *Result:* met by M1.7a;
  M1.7b and M1.7c add the streamers, tumble and separation (ADR-012, ADR-013, ADR-014).
  - [x] **M1.7a Parachutes and descent:** parachutes (Cd·S, inflation time or area-growth model),
    drogue and main with deployment triggers (apogee, altitude, timer, motor delay), drogue
    release, descent with wind drift and landing detection.
    - Loft lessons: L27, L28, L29, L92.
    *Done when:* analytic tests for terminal velocity, descent time and drift pass, and descent
    rate and drift match RocketPy's for 3 example rockets within 3%. *Result (ADR-012):* met.
    Analytic descents to 2.1e-8; five RocketPy examples within 0.71% (time), 0.03% (rate), 0.27%
    (drift); worst drift component 2.87% (NDRT's added mass).
  - [x] **M1.7b Streamers and tumble,** each with a cited drag model. *Done when:* a streamer's and
    a tumbling body's descent rates match the terminal velocity of their cited drag models (analytic
    tests). *Result (ADR-013):* met. Streamers: Carruthers and Filippone (within 9% of Kidwell's
    flat streamer; appendix C 88% fast). Tumble: OpenRocket §3.5, −10 to +19% on its own drop tests.
  - [x] **M1.7c Separated bodies:** separation, with every body flown to its own landing and its
    own mass properties and drag. *Done when:* a separation gives every body a landing, and the
    bodies' masses sum to the rocket's. *Result (ADR-014):* met. A `Separation` splits the stack at
    a stage boundary, each body a point mass under its devices; on the two-stage test design both
    land (2.11 m/s under a canopy, 16.74 m/s tumbling), masses to 1e-12, momenta to 1e-9. Every
    body carries a device.
- [x] **M2.1 Validation harness plus the RocketPy code-to-code suite.** The first end-to-end
  milestone: `hpr-validate` and `cargo xtask validate [--fast]`, TOML cases and reference JSON with
  provenance, Markdown and JSON reports, oracle scripts in `validation/oracles/rocketpy/`. At least
  five RocketPy example rockets fly in two modes — **same-drag**, which isolates dynamics,
  environment and motor, and **predicted**, hpr's own aero, whose supersonic gaps are reported
  rather than hidden — and CI compares against the stored references.
  *Done when:* at least 5 cases pass their same-drag tolerances; predicted-mode results are
  reported, with explained gaps; `validation/reports/latest.md` is generated; the CI job is green;
  and a separate, manually triggered workflow regenerates the references.
  - [x] **M2.1a The harness.**
    - Loft lessons: L76, L77, L78, L79.
    *Done when:* `cargo xtask validate` runs every locked case against its reference and writes
    `validation/reports/latest.md`; a metric with no tolerance, a value with no provenance, and
    fewer cases than the lock expects each fail. *Result (ADR-015):* met. Five descent cases, 30
    metrics within 3%; twelve tests refuse each malformed case (L76–L79). Gravity's bug: #27.
  - [x] **M2.1b Whole flights against RocketPy, same-drag.** Met in two parts.
    - Loft lessons: L75.
    - [x] **M2.1b1 The whole-flight oracle.** Met: `validation/oracles/rocketpy/flight.py` writes
      all five cases byte for byte, no RocketPy data committed; declared `C_D0` 0.5; #33 fixed.
    - [x] **M2.1b2 The whole-flight cases.** Met (ADR-021): six cases, five pass every scored
      metric within 3% (largest +1.783%, Bella Lui); drifts unscored until M2.1d3.
      - Loft lessons: L75.
  - [x] **M2.1c Predicted mode, CI and regeneration:** the cases with hpr's own aero beside the
    same-drag ones; a CI job checking the stored references; a manual workflow regenerating them.
    *Done when:* its three bullets, which M2.1c2 (the first) and M2.1c1 (the other two) carry.
    - [x] **M2.1c1 The CI job and the regeneration workflow.** *Done when:* CI is green on three
      OSes; regeneration runs only when a human triggers it, a diff to review, not a commit.
      *Result (ADR-022):* met. `validate --check` writes nothing, fails on a miss or an unreproduced
      report; *Regenerate references* is dispatch only; run locally, it reproduced every byte.
    - [x] **M2.1c2 Predicted mode.** *Done when:* every case's predicted results are reported, each
      gap explained, `M ≥ 1` cases as gaps until M1.8. *Result (ADR-023):* met. Six `predicted-*`
      cases, 3% targets: 56 of 75 within; Valetudo and NDRT 2020 +10% in apogee; misses pinned.
  - [x] **M2.1d The time-series RMS and the path in wind.** Met in three parts.
    - [x] **M2.1d1 The time-series RMS.** Met (ADR-024): RMS at RocketPy's 120 series times, held
      to 3% of apogee and max speed; same-drag all pass; predicted, three outside (the drag).
    - [x] **M2.1d2 The calm-air cases (issue #50).** Met (ADR-025): Calisto and Bella Lui pass;
      Juno III's drifts miss (−3.7%), reported not scored, 1.6 points being the rail release.
    - [x] **M2.1d3 The path in wind (issue #50).** Met (ADR-026): RocketPy's moment point and
      nozzle tensor corrected, then within 1.4% in wind; six drifts gated, five not.
- [x] **M1.8 Aerodynamics II (transonic and supersonic, damping, overrides).**
  - Transonic drag rise and supersonic wave drag.
  - Supersonic fin normal force (Ackeret/Busemann or a documented alternative).
  - CP shift with Mach.
  - Pitch, yaw and roll damping; roll forcing from cant.
  - Extend the M1.5 override tables to CNα and CP vs Mach and AoA, importable from RASAero CSV.
  - Loft lessons: L7, L17, L18.
  *Done when:* Cd vs Mach is within 10% of RocketPy's RASAero CSVs across Mach 0.1–2.0 for the
  available rockets, per-band errors in the report; the supersonic cases in the M2.1 suite are
  within their tolerances; roll-rate steady state matches the analytic cant/damping balance.

  Split into M1.8a to M1.8e. The measured reference throughout is NASA's Arcas Robin wind-tunnel
  model: TN D-4013 (Mach 0.6–1.2) and TN D-4014 (Mach 1.5–4.63).
  *Result (ADR-143):* closed with its misses: Cd bullet not met (ADR-029, ADR-030), now an M1.14
  input (met or re-measured); same-drag supersonic case passes (ADR-027); the predicted one (Mach
  1.06) misses its 3% target, a target not a gate (ADR-023), explained in its case; roll met.
  - [x] **M1.8a Normal force and centre of pressure through Mach 1:** fin slope through the
    transonic region to supersonic linear theory, and the fin CP shift with Mach. The normal force
    accepts Mach numbers past 1, so a flight on a drag table flies through Mach 1. Loft lesson L7.
    *Done when:* L7's test passes (the fin slope and CP are Barrowman's at Mach 0 and change with
    Mach); a committed fixture, pinned by a test, holds hpr's `C_Nα` and CP against the Arcas Robin
    measurements at every Mach they give and RASAero II's Calisto export from Mach 0.1 to 2.0,
    against targets set before measuring — CP within 0.5 calibers, `C_Nα` within 15% — with every
    miss explained; and the same-drag Prometheus 2022 case flies through Mach 1 and passes.
    *Result (ADR-027):* met. Linear theory from `M_s`, a join from Mach 0.8. L7 passes; 37 rows
    pinned, 16 outside the targets, explained. Mach 1.5–2.96: `C_Nα` −13.4% to +3.3%, CP within
    0.42 calibers; past Mach 3, 17–25% low (M1.8e). Prometheus flies through Mach 1.010. Since
    M1.8e6's body lift (ADR-037): 17 outside; Mach 1.5–2.96 −16.3% to +1.4%, CP within 0.47.
  - [x] **M1.8b Transonic and supersonic drag.** Every drag term's transonic and supersonic branch,
    and nose wave drag. Loft lessons L17 and L18. *Done when:* M1.8's Cd bullet is met or an ADR
    records why not, with the gap in the report; the Arcas Robin's measured axial force is compared;
    the predicted Prometheus case flies. Split below into M1.8b1 to M1.8b3, which carry these
    bullets between them.
    - [x] **M1.8b1 The drag buildup through Mach 1.**
      - Nose, shoulder and step pressure drag through Mach 1 (Niskanen eq. 3.87 and appendix B,
        Stoney's fineness-3 curves); the buildup accepts Mach 0 to 5. Loft lesson L17.
      *Done when:* L17's test passes; the predicted Prometheus case flies; the Arcas Robin's
      measured axial force is compared: a committed fixture, pinned by a test, holds hpr's
      forebody drag against TN D-4013's and TN D-4014's at every Mach they give, fins on and off.
      *Result (ADR-028):* met. L17's test passes; `drag_against_mach` pins 44 rows, 8 within 10%
      (misses: blunt fin edges, the base lip, the boattail rule). Predicted Prometheus flies.
    - [x] **M1.8b2 Drag against RASAero through Mach 2.** Cd against Mach from RocketPy's RASAero
      CSVs, per band (L18). *Done when:* M1.8's Cd bullet is met or an ADR records why not. *Result
      (ADR-029):* not met, recorded. Calisto's export: 15/15 subsonic, 2/7 transonic, 0/17
      supersonic within 10%; no fin input is within 10% subsonic and supersonic both. MIL-HDBK-762's
      worked example: 6/12, the body 6–10% low past Mach 1.6. Boattail a candidate.
    - [x] **M1.8b3 The boattail and base faster than sound.**
      - A conical boattail's supersonic wave drag (MIL-HDBK-762 Fig. 5-122), the base behind it,
        and a lip in its wake.
      *Done when* (targets set before measuring; or an ADR records why not): the Arcas Robin's 11
      fins-off rows from Mach 1.5 within 10%, the 2 now within staying; Calisto's 17 supersonic
      rows against RASAero II within 10%; each row's change reported (ADR-029).
      *Result (ADR-030):* not met, recorded. Boattails of 3° to 10° from Mach 1.2: −21.9% to
      +28.3%; Arcas Robin fins off from Mach 1.5: 0 of 11, +13.5% to +24.1% (#72); Calisto: 8 of
      17, −14.9% to −5.1%.
  - [x] **M1.8c Roll and damping.** Roll forcing from fin cant and roll damping; pitch and yaw keep
    hpr's local-flow damping (ADR-011). *Done when:* M1.8's roll bullet is met, and hpr's roll
    forcing is compared with the Arcas Robin's measured roll effectiveness (TN D-4014). *Result
    (ADR-031):* met. Barrowman's strip theory with his body factors. Valetudo canted 1° at 100 m/s
    settles on the closed-form balance, −16.948 rad/s, within 1e-11. Against TN D-4014: from Mach
    2.3, 8 of 8 within 5.3%; at Mach 1.5 and 1.8, +14.3% to +47.8%. Damping against the Basic
    Finner: −5.9% to −16.2%.
  - [x] **M1.8d Normal-force overrides.** `C_Nα` and CP tables against Mach and angle of attack from
    a RASAero II export. *Done when:* its `C_Nα` and CP columns replace hpr's in a flight, and the
    reading is tested on the Calisto export. *Result (ADR-032):* met. The table's force at the
    centre of mass's flow, hpr's damping kept. Calisto's export (0°, 2°, 4°): 4,999 rows re-read
    with `refs/`, M1.8a's 30 values in CI; Calisto flies on it; a table's pitch period within 4e-6
    of linear theory, pitch and yaw.
  - [x] **M1.8e The body's supersonic normal force.** M1.8a measured the gap: past Mach 3 the
    Arcas Robin's body alone lifts 3.9 to 4.6 per rad, where slender-body theory gives 2.3 to 2.8
    with body lift. A cited supersonic method for noses, boattails and crossflow. *Done when:*
    the Arcas Robin's body-alone `C_Nα` (fins off, TN D-4014) is within 15% at every Mach number
    from 1.5, and both configurations' `C_Nα` within 15% at Mach 3.96 and 4.63, or an ADR records
    why not with the gap in the report. Split below into M1.8e1 to e12; e9 judged this bullet.
    *Result (ADR-040, ADR-143):* closed, unmet for the body alone: M1.14d (Mach 2.96 row: M1.14h).
    - [x] **M1.8e1 The second-order shock-expansion method** (NACA TN 3527, a pointed body at
      `α → 0`). *Done when* (targets set before): a pinned fixture holds every row of its Tables I
      and II within 0.05/rad and 0.1 cal of its values and ±0.2 of its measurements, misses
      explained, and the Arcas Robin beside TN D-4014. *Result (ADR-033):* not met, recorded: 102
      and 125 of 144 within its values (#81 at its limit), 117 and 109 of 120 of its measurements;
      Arcas Robin −18.7% to +16.4%.
    - [x] **M1.8e2 The body's supersonic normal force in flight** (M1.8e1 joined to slender-body
      theory). *Done when:* a flight takes the body's `C_Nα` and CP at its Mach, no jump at ±1e-9
      in Mach at the join, and the Arcas Robin's body alone from Mach 1.5 is in the report.
    - [x] **M1.8e3 The supersonic join's start without grid steps** (#87's grid half). *Done when*
      (set after building): a test finds a 20° cone's start off the grid, moved under 1e-7 in Mach
      by 1e-6° and strictly by each 0.1° to 20.5°, its cylinder share under 1e-5, no jump at ±1e-9
      at the join's ends or first even row. *Result:* met: Mach 1.341910 (was 1.35); report same.
    - [x] **M1.8e4 The boattail's share faster than sound** (footnote 8; a station rule for shares
      that cross zero). *Done when:* a boattailed body flies with no jump at ±1e-9 in Mach; the long
      Arcas Robin through a flight's path in the report. *Result:* met, long to −27.0%.
    - [x] **M1.8e5 The remaining gap, source by source.** *Done when:* a `docs/research/` page sizes
      each candidate from cited sources against the Arcas Robin's gaps, ranked. *Result:* met; like
      for like hpr reads 15–73% high: crossflow's size, then the boattail.
    - [x] **M1.8e6 Crossflow and the boattail faster than sound** (ADR-036, ADR-037). *Done when:*
      flown with no jump at ±1e-9 in Mach; the Arcas Robin through a flight's path in the report.
      *Result:* met; like for like +3.4% to +41.0% (was +14.9% to +73.2%); M1.8a gains a miss.
    - [x] **M1.8e7 Blunt tips faster than sound** (from e6; split, ADR-038). A vertical or blunt
      nose tip flies a Newtonian cap ahead of TN 3527's method (NASA TN D-4865). *Done when:* flown
      with no jump at ±1e-9 in Mach; the Arcas Robin's committed nose (lip left off) through a
      flight's path in the report; TN D-4865's sphere-cone against its measured normal force.
      *Result:* met; like for like, sphere-cone −1.2% to +32.1%; the nose lip off −4.8% to +37.2%.
    - [x] **M1.8e8 The lip faster than sound** (from e7; ADR-039). *Done when:* flown with no jump
      at ±1e-9 in Mach; the committed Arcas Robin designs through a flight's path in the report.
      *Result:* met; a lip in a boattail's wake carries nothing, so both designs fly the method to
      their base: M1.8a's rows from Mach 1.5 are all within the slope's 15% (+9.4% to −3.3%).
    - [x] **M1.8e9 #90's boattail cap, and M1.8e's 15% bullet judged** (split from e9's pair;
      ADR-040). *Done when:* #90 closed (a bound on the boattail angle, and footnote 8's size pinned
      by a hand calculation); M1.8e's 15% bullet met for the Arcas Robin's body alone, or an ADR
      records why not with the gap in the report. *Result:* met; the correlation is read no steeper
      than 16° and the boattail with its tube integrated by hand; the bullet met at Mach 3.96 and
      4.63, the body alone outside on six rows.
    - [x] **M1.8e10 The lip's shelter, weighed not switched.** #87's five switches all flip one gate
      — whether the method covers the body at all — so each is worth the whole body. The largest is
      the lip's, and the drag buildup already grades its shelter continuously (`share_by_rise`, a
      quarter to a half of the boattail's drop) where the normal force reads a threshold. *Done
      when:* every switch's size is measured by a test; the lip's is gone, its two sides agreeing
      across the old threshold in proportion to the change in shape; no committed fixture moves; an
      ADR records the weight. *Result:* met (ADR-041); the lip's **rise** is a weight, not a switch,
      and five keep measured sizes — a step −8.7%/1.03 cal, a flare −27.5%/0.29 cal, a pointed tip
      −10.4%/1.14 cal, a vertical tip −7.0%/0.64 cal, and the lip's own length −33.0%/1.77 cal,
      which #87 didn't list.
    - [x] **M1.8e11 Cone slopes past Fig. 2's edge, from Sims.** NASA SP-3007 (pinned) tabulates the
      same theory to 30°, where TN 3527's Fig. 2 stops at 24°. *Done when:* the method flies a
      tangent cone of 24° to 30°, a fineness-1 cone among them, from Sims's slopes checked against
      Fig. 2 where they overlap; no committed fixture moves. *Result:* met (ADR-042); SP-3007 Table
      2 at 25°, 27.5° and 30°, agreeing with the chart to 0.0021 per rad at 22.5°; the pointed tip's
      switch moves to 30° and falls to −7.7%/0.81 cal.
    - [x] **M1.8e12 What the handover's cap is worth, and what stops it** (split from the old e12,
      whose aim is now M1.8e13). With slopes to 30° a cap can hand over at the detachment angle, not
      Fig. 2's edge — but moving it there takes the march out of its range on a nose that flattens
      fast, so this increment measures the move and the next one makes it. *Done when:* the cap is a
      parameter, its flown default unchanged so that no committed fixture moves; a sweep of it is
      measured into a fixture (each cap's sphere-cone error, and the nose's readings over 10 to 160
      elements with their reduced counts); tests pin both ends; an ADR records why the default
      stays; and the blocker is a GitHub issue. *Result:* met (ADR-043); 30° follows TN D-4865's
      rule to Mach 2.52 and reads nearer its sphere-cone wherever a cap binds, but puts 109 of 160
      elements into `η < 0` at Mach 4.63, where the answer follows the element count (3.047 to
      3.260). 24° stands.
    - [x] **M1.8e13 What the answer follows when it follows the mesh** (split from the old e13,
      whose aim is now M1.8e16). Issue #108 asked for a reading of `η < 0` that settles as the nose
      is cut finer; before writing one, find out what the answer actually follows. *Done when:* the
      count that separates a settled reading from a moving one over the cap sweep's meshes is
      measured and stored beside every reading; a pointed body of TN 3527's own is shown reducing
      without moving; tests pin both and the mechanism; an ADR and the guide say what it means and
      does not; and #108 is re-scoped to it. *Result:* met (ADR-044); it is the surface pressure
      **crossing** its tangent cone's, not `η < 0`. Across 10, 40 and 160 elements the 27 readings
      without a crossing hold to 0.012 per radian and the 5 with one move 0.035 or more — a flag,
      not a verdict.
    - [x] **M1.8e14 Where the flare's march stops** (the first of three the old e14 splits into;
      M1.8e17 and M1.8e18 carry its other clauses word for word and go next, ADR-045). The method
      already marches a flare — a cone, a tube and a flare return a finite `C_Nα` at Mach 3 — and
      the model around it refuses one, stopping at the first widening body. *Done when:* the
      steepest flare it marches is bisected to f64 resolution over Mach, tests pin it and what stops
      it, and an ADR records what happens where the flare's shock is detached. *Result:* met
      (ADR-045). The edge is the corner's **isentropic** turn running out, not the shock detaching,
      and lands either side of a wedge's limit: 11.9312175° at Mach 1.5 against 12.1126689°,
      26.4714031° at Mach 2 against 22.9735318°, then the tables' 30° from Mach 2.129702032593.
      Which side is the body's doing (no tube: 14.194333° at Mach 1.5), so a march that answers is
      no evidence of attachment.
    - [x] **M1.8e17 The flare through the method** (the second of the old e14's three, ADR-045).
      *Done when:* a flared body flies the method where the flare's shock is attached, with no jump
      at ±1e-9 in Mach or in the flare's angle across that boundary. *Result:* met (ADR-047). The
      test is NACA 1135's wedge limit read at the flow the march delivers to the corner — TN D-4865
      p. 5's own — under the cone tables' 30°, which binds from Mach 2.5192034260. A steeper flare
      reads as the same radii drawn out to that turn, so at the limit the branches are one body: at
      Mach 2 (22.969761173077°) a ±1e-9° probe moves the slope 4.527e-11 and a ±1e-5° probe
      4.527e-7; in Mach at 18.5°, 3.622e-10 and 3.622e-6. The value is continuous; its slope is not
      (−31.4%). The near-flat region it refuses is M1.8e19's (#117).
    - [x] **M1.8e18 What a marched flare is worth** (the third, ADR-045). TN D-4865's model 2 is a
      2.75° blunted cone with an 18.5° flare; its fig. 8 carries normal force and pitching moment
      from Mach 1.50 to 4.63, integrated from the pressures its tables VII to XII print, and from
      Mach 2.96 up its boundary layer separates ahead of the juncture. *Done when:* those readings
      are committed with provenance, and the guide says what it is worth and leaves out. *Result:*
      met (ADR-048). Fig. 8(b) read by M1.8e7's pipeline into `tn-d-4865-flared-cone.json`. hpr
      reads −1.9%, +7.0% and +13.4% at Mach 1.90, 2.30 and 2.96, then +51.5% and +50.4% at 3.95 and
      4.63, where the shadowgraphs show that flare separated (unflared, model 1 reads +29.7% and
      +32.1%). No reading below about Mach 1.5289.
    - [x] **M1.8e15 The step in radius.** Needs a model of its own, as the march refuses a step.
      *Done when:* #87 closed or narrowed to the step alone, its measured size in an ADR and the
      guide. *Result:* met (ADR-049): no source gives a step's supersonic normal force, so the size
      is published: bisected thresholds 2.7e-11 m and 1.3e-13 m, −8.65% and 1.03 cal there, −12.55%
      and 1.36 at 2 mm down, −4.75% and 0.71 at 2 mm up, −11.34% and 1.10 on a boattail; stopping
      the march at the step rejected (ADR-034's mixture), and it does not close the boattail's
      band. #87 is narrowed to the step; #120 and #121 split off it.
    - [x] **M1.8e19 The near-flat flare the march refuses** (#81, #117). Met (ADR-050), bars kept:
      the region's edges derived, not bisected (the corner's crossing and balance); the reduction
      read there, so both switches (−4.6%, −8.3%) go; a test pins the sizes on both sides.
- [x] **M3.1 OpenRocket `.ork` import.** Handles zip, gz and raw XML, schema 1.0 to 1.10, plus the
  documented 1.11 additions; reads components, materials, finishes, motor configurations, recovery,
  stages, and stored simulation results; unknown content is kept in `extensions.x-openrocket` for a
  lossless round trip, and warnings are graceful, never failures.
  - Loft lessons: L49, L56, L57, L58, L59, L60, L61, L62, L63, L64, L65, L66.
  *Done when:* every `.ork` in `refs/loft-fixtures` and the OR example set imports with zero
  errors; committed `insta` snapshots use only public files (the Loft demo fixtures and synthetic
  designs), and private-corpus results go to a gitignored `corpus-out/`, as counts only; the
  RocketSerializer cross-check agrees on the key geometry. *Result:* met by M3.1a to M3.1d. Split
  a to d (ADR-051): the container and the document first, as everything after it walks that tree.
  - [x] **M3.1a The container and the design document.** Sniff zip, gzip and raw XML; keep every
    archive entry and all the XML said; warn, never crash. Loft lesson L56. *Done when:* every
    `.ork` in the reference library and in the OpenRocket jar's example set either reads or is
    shown by a second XML parser not to be well-formed; each one written back out and read again
    gives the same document; `cargo xtask ork` prints those counts and writes the per-file detail
    to a gitignored `corpus-out/`; and `hpr_io::ork::tests::malformed_inputs_error_not_panic` is
    live. *Result:* met (ADR-051): 76 of 78 read and round-trip; 2 Loft fixtures are not
    well-formed XML, as expat agrees.
  - [x] **M3.1b The component tree.** Components, shapes, materials, finishes and overrides into
    `hpr-design` types, automatic dimensions resolved (L49, L58 to L63). *Done when:* every design
    in the reference library gives a `hpr_design::Rocket` whose `layout()` succeeds, each of those
    lessons' named tests is live, and the counts go to `corpus-out/` as above. *Result:* met by
    M3.1b1 to M3.1b4 (ADR-052 to ADR-054): 75 of 75 designs lay out; 1 document holds none.
    - [x] **M3.1b1 The values inside the tags.** What every later step asks the tree for: numbers,
      counts, flags, and the dimensions OpenRocket works out for itself; the tags it writes under
      two names; the overrides. Loft lessons L58, L62, L63. *Done when:* those three lessons' named
      tests are live, each resting on what the corpus shows rather than on an assumption, and `cargo
      xtask ork` prints the counts they rest on. *Result:* met (ADR-052): `auto <number>` keeps both;
      a stated `0` is a value.
    - [x] **M3.1b2 The spine.** The stages and the body components stacked in them, with their
      shapes, lengths, radii, walls, materials and overrides, every automatic radius marked for
      `layout()` to resolve rather than filled in, across a stage boundary too (L59). *Done when:*
      L59's named test is live and `cargo xtask ork` says how many spines lay out and what was left
      off them. *Result:* met: 73 of 76 spines lay out; a wall-less shoulder reads solid, for M2.2.
    - [x] **M3.1b3 The parts on and inside the body** (L49, L60, L61; M3.1b4 carried the rest).
      *Done when:* every part on or in a body is read or left out with its reason, the lessons'
      tests live, and `cargo xtask ork` counts both. *Result:* met (ADR-053): 765 parts, 5 left
      out with a reason; angles are degrees, their sense unsettled; 67 of 71 cached answers match.
    - [x] **M3.1b4 The designs that still do not lay out**, carrying M3.1b's bullet. Three of the
      76: one holds no `<rocket>` with components in it at all, and two have a chain of automatic
      radii with no fixed radius anywhere to resolve against, one caching a number and one not.
      *Done when:* each either lays out or is shown to hold no design, with its reason on the
      `.ork` page; a rule for an unresolvable chain, if there is to be one, rests on something
      written down rather than on a cached number; and `cargo xtask ork` says so. *Result:* met
      (ADR-054): 75 of 75 lay out, 1 document holds none; 7 radii take OpenRocket's 25 mm default;
      67 of 67 body radii agree with OpenRocket.
  - [x] **M3.1c Motors, recovery, stages and what OpenRocket last did** (L57, L64, L65, L66). Met
    (ADR-055 to ADR-058) bar the text of a tag's unread second copy, its done-when bars kept; split
    c1 to c4 (ADR-055).
    - [x] **M3.1c1 Motors and their configurations** (L57, L65). Met (ADR-055), bars kept: 206
      motors in 174 configurations, 6 left out in pods and parallel stages; 6 curves found from the
      archive or the bundled catalog; 1 configuration flies; L57's and L65's tests live.
    - [x] **M3.1c2 Recovery and separation.** Met (ADR-056), bars kept: 137 devices read, 2 left
      out in pods; 18 of 93 stages separate, 2 parallel stages' left out; counts by `xtask ork`.
    - [x] **M3.1c3 What OpenRocket last did** (L64). Met (ADR-057), its done-when bars kept: stored
      conditions, summaries, time series and events read back, 174 simulations and 142 series
      counted by `cargo xtask ork`, units measured by a probe.
    - [x] **M3.1c4 Pods, parallel stages and the rest** (L66). Met (ADR-058), its done-when bars
      kept: 17 parts, 87 sections, 1,888 tags and 3,128 attributes round-trip, all 5,120 found again.
  - [x] **M3.1d The corpus and the cross-check.** `insta` snapshots on public files only, and the
    RocketSerializer cross-check. *Done when:* the parent's four bullets above are met. Split: d1, d2.
    - [x] **M3.1d1 Snapshots of public designs.** Met, bars kept: `insta` snapshots of public files
      only (Loft's 7 demos and a synthetic one); private results to `corpus-out/`, as counts.
    - [x] **M3.1d2 The cross-check.** Met (ADR-059), bars kept: every `.ork` (27, 17 OR examples)
      imports with 0 errors; of the first record's 1,212 numbers none is apart from both oracles.
- [x] **M2.2 OpenRocket oracle and corpus.** `validation/oracles/openrocket/` (JPype, OR 24.12)
  flies the OR examples and the corpus; the stored results inside the `.ork` files are used as a
  second reference; the deferred M1.4 mass/CG checks run against OR values. *Done when:* at least
  20 designs are in the report with an error distribution (apogee, max velocity, stability margin,
  mass, CG); every design with apogee error above 5% has a written hypothesis; private designs
  appear only as anonymised ids. Split a to f (ADR-060, 101); met (ADR-100), L19 missed (ADR-102).
  - Loft lessons: L19, L51, L80, L81, L82, L87.
  - [x] **M2.2a Structure mass, CG and inertia** (the M1.4 deferral). Met (ADR-060), its done-when
    bars kept: `cargo xtask ork` holds every design OpenRocket opens to its structure's mass, CG and
    inertias, the Loft demo record is checked in CI, and 57 and 58 of 74 were within 1% in the
    original snapshot, the 17 outside five causes hpr warns of.
  - [x] **M2.2b OpenRocket's mass conventions** (L51, L87). *Done when:* L51, L87 are live and each
    convention ADR-060 lists, and roll inertia, is hpr's rule or a written departure, M2.2a rerun.
    Split into b1 to b5 (ADR-061 to ADR-065). *Result:* met (ADR-101): each is OR's rule or a
    pinned departure; the rerun has mass and CG within 1% on 68 of 71, each file outside with a cause.
    - Loft lessons: L51, L87.
    - [x] **M2.2b1 What a `.ork` leaves unsaid, and overrides** (L51). Met (ADR-061), bars kept:
      walls, shoulders, materials as OR's on probes; two override departures pinned; L51 live.
    - [x] **M2.2b2 Fins, rail buttons and roll inertia.** Met (ADR-062), bars kept: the roll gap is
      OR's fin rule (hpr's exact integral kept); median 0.001%, each file outside 1% has a cause.
    - [x] **M2.2b3 Packed parts.** Met (ADR-063), bars kept: a packed part with no size and an
      override on a weightless one are OR's on probes, to 1e-12; roll within 1% on 57 of 74.
    - [x] **M2.2b4 Clusters, fillets and unread parts.** Met (ADR-064), bars kept: cluster, fillets
      pinned, unread parts kept; 58/71 mass, 59/71 centre, 50/71 pitch, 56/71 roll within 1%.
    - [x] **M2.2b5 Stored results as found** (L87). Met (ADR-065), bars kept (L87 live, two screens
      with stable reasons): 91 of 174 pass, 83 excluded; 1 of the 91 reproducible by hpr.
  - [x] **M2.2c The motors OpenRocket flies.** Met (ADR-066, ADR-067), its done-when bars kept:
    every configuration held back only for want of a curve flies or is named with its reason, each
    curve's impulse within 0.1% of OR's. Split into c1 and c2.
    - [x] **M2.2c1 Every curve as OpenRocket integrates it.** Met (ADR-066), bars kept: all 32
      bundled curves bit for bit OR's impulse, peak, 5% window and duration; average thrust departs.
    - [x] **M2.2c2 The configurations held back for want of a curve.** Met (ADR-067), bars kept: OR's
      database supplied by digest; of the 162 held back, 66 fly, 72 wait on another reason, 24 named.
  - [x] **M2.2d Flights to apogee on the public designs.** Met (ADR-068, ADR-069), its done-when
    bars kept: those that fly are in a report against OR, and L80, L81 are live. Split in two.
    - [x] **M2.2d1 OpenRocket's flights, and what its words mean** (L80, L81). Met (ADR-068), bars
      kept: 56 calm flights, 1 aborted, in a pinned record; 9 of 10 words held, one withheld.
      - Loft lessons: L80, L81.
    - [x] **M2.2d2 hpr's flights against the record.** Met (ADR-069), bars kept: 21 flown; margin
      within 0.016 cal; 5 apogees over 5%, each with a named cause (early chute, #165).
  - [x] **M2.2e The corpus** (L19, L82: moved to M2.2f, ADR-101). *Done when:* the parent's *done
    when* is met, unchanged. Split into e1 to e10 (ADR-070, 072, 094, 095, 098 to 100). Met in e10.
    - [x] **M2.2e1 Mass and CG in the flight report.** Met (ADR-070), bars kept: launch and
      rod-clearance mass and CG against OR's, tested, on all 21; within 0.22% and 0.016 cal.
    - [x] **M2.2e2 OR's flights of the corpus.** Met (ADR-071), bars kept: 88 of the 27 `.ork`
      designs' 89 configurations flown into `corpus-out/` (OR loads no motor for the 89th).
    - [x] **M2.2e3 hpr's flights of the corpus.** Met (ADR-072), bars kept (anonymised ids,
      differences only, curves checked as OR's, a test on its words and sums), the 20 designs
      moved to e6 (ADR-094): 17 flights, 4 of 12 private designs, 9 in all.
    - [x] **M2.2e4 The causes.** *Done when:* every apogee over 5% has a written hypothesis.
      *Result:* met (ADR-073): all 5 sized by OR flying without the cause; 4 within 5%, 1 at +7.80%.
    - [x] **M2.2e5 A tilted rod** (#173). Met (ADR-094), bars kept (a rod as OR records it, a test
      on OR's landings, probes within 5%, bearing 0.1°, the five spreads, e4's bar): bearings within
      0.03°, loss within 0.27 points; `C12` 0.53%.
    - [x] **M2.2e6 The old override flag** (#174). Met (ADR-095), bars kept (probes, a test, the five
      spreads, e4's bar): 11 probes; `C05` flies, 0.26%. **Not met for `C10`**: #184, in e10's pool.
    - [x] **M2.2e7 Fin fillets and an inner tube's radius** (#174). Met (ADR-096), bars kept: 21
      probes to 1e-15; `C01`, `C06` fly: 18 designs. `C06/1` +13.60%, +1.11% on OR's drag (ADR-097);
      supersonic pressure drag twice OR's (#222).
    - [x] **M2.2e8 Tube fins OpenRocket sizes** (#133; split from the old e8, whose flight is now
      e9 and whose bar is e10; ADR-098). *Done when:* an `auto` tube-fin radius resolves as OR 24.12
      does, on probes, with a test. *Result:* 19 probes, radius within 1e-15; the example's mass
      within 5e-6; roll (past any ring's bound) and pitch (no spread term) OR departures.
    - [x] **M2.2e9 Tube fin aerodynamics.** *Done when:* tube fins have a cited normal-force and drag
      method, pinned by tests; the tube fin example flies, e4's bar on it. *Result:* met (ADR-099):
      ring wings (Weissinger, Fletcher), 9 tests; the example +6.95%, +0.03% on OR's drag (#228).
    - [x] **M2.2e10 Twenty designs.** *Done when:* anonymised ids beside the public report make at
      least 20 designs with the five spreads; e4's bar on the flights added. *Result:* met
      (ADR-100): a motor whose ignition never comes flies unlit, as 5 OR probes show; `C04` +0.88%.
  - [x] **M2.2f Lessons L19 and L82.** *Done when:* both lessons' tests are live, L19's asserting
    tube fins' CP within 0.25 cal of OR's; if that cannot hold, an ADR measures it and L19's row
    names a test pinning the gap and says so, as L18's row does. *Result:* L82 live; L19 not met,
    pinned (ADR-102): CP 0.42-3.0 cal ahead of OR's to Mach 0.5 on 14 probes, 1.07 on its example.
    - Loft lessons: L19, L82.
- [x] **M1.9 Staging, clusters, airstarts (COTS).** Separation triggers (burnout plus delay,
  altitude, time) and sustainer ignition; the booster tracked through recovery; clustered mounts
  with mass and thrust summed and the thrust offset handled. *Done when:* a two-stage design and a
  cluster design each match OpenRocket within the per-case tolerance; event ordering tests pass.
  Met in a to c (ADR-074 to ADR-076).
  - Loft lessons: L30, L31, L93.
  - [x] **M1.9a Ignition times and powered staging.** Met (ADR-074), bars kept: motors lit at
    launch, a time, a burnout or a separation plus a delay; the sustainer flies on, the booster lands.
    - Loft lessons: L30, L93.
  - [x] **M1.9b Clusters.** Met (ADR-075), bars kept: a motor out's turn is the hand calculation's
    to 3.7e-7; every tube of 25 OpenRocket probes within 1e-15 m; `.ork` clusters read.
    - Loft lessons: L31.
  - [x] **M1.9c Against OpenRocket.** Met (ADR-076), bars kept: a `.ork` two-stage and a cluster
    design within 5% of OpenRocket in apogee (3 vs OR without its early chute) and largest speed.
- [x] **M1.10 Outputs and derived metrics.** Met in a to c (ADR-077 to ADR-080): stability
  margins over the flight, optimum ejection delay, max q, flutter velocity and margin, landing
  lat/lon; CSV, JSON, Parquet, KML, GeoJSON. *Done when* (met): metrics are unit-tested; exports
  validated (GeoJSON by schema, KML by parsing); flutter matches the cited source's worked example.
  - Loft lessons: L32, L33, L34, L35, L94.
  - [x] **M1.10a Flight metrics.** Met (ADR-077): hand-tested to 1e-6, margins to 1e-9.
    - Loft lessons: L33, L34, L35, L94.
  - [x] **M1.10b Fin flutter.** Met (ADR-078): TN 4197 eq. 18 at Martin's printed resolution.
    - Loft lessons: L32.
  - [x] **M1.10c Exports.** Met (ADR-079, ADR-080), the parent's second bullet.
    - [x] **M1.10c1 Text exports.** Met (ADR-079): CSV, JSON, GeoJSON by schema, KML parsed.
    - [x] **M1.10c2 Parquet.** Met (ADR-080): Apache's `parquet` reads every value back.
- **M2.3** is open; its entry: [roadmap](ROADMAP.md#phase-1-physics-core-the-heart-with-validation-interleaved).
  - [x] **M2.3a ERA5 weather.** Met: netCDF classic and 64-bit offset read as Unidata's library
    reads them, netCDF-4 refused with the conversion; ERA5 levels match RocketPy 1.13's to 1e-12;
    a guide page and an example CI runs.
  - [x] **M2.3b RocketPy's logged flights.** Met (ADR-082): seven flights in their ERA5 weather,
    apogee and ascent RMS each, hpr read as the logs' barometers; mean absolute apogee error 6.04%
    against 5%, outside it; the five outliers explained (drag, impulse) by claims the report checks.
    Astra and Andromeda (EuRoC 2022, netCDF-4) wait.
  - **M2.3c** is open; its entry: [roadmap](ROADMAP.md#phase-1-physics-core-the-heart-with-validation-interleaved).
    - [x] **M2.3c1 Logged apogees** (ADR-184). *Done when:* each tier-A flight from a `.ork` is flown by hpr
      and by OpenRocket in its day's weather, or refused with its cause in an open issue, the
      counts published as aggregates with ADR-151's attribution; each
      logged apogee is converted to height above the pad with the day's temperature profile,
      early deployments flagged or left out; each simulator's error is in the report as
      anonymised statistics.
      *Result:* of 83, both fly 61 (hpr 61, OpenRocket 79), the rest under 9 causes, each with an
      issue; over 55 compared, hpr +9.83% (mean absolute 13.96%), OpenRocket +9.00% (13.08%), hpr
      within 5% of OpenRocket on 53.
- [x] **M2.4 Accuracy census gate.** Generate a summary census (a README table and badge) from the
  report. CI fails on any per-case regression beyond tolerance.
  - Loft lessons: L84, L85, L86, L88.
  *Done when:* a deliberately perturbed drag coefficient on a throwaway draft PR makes CI fail.
  The failing run is linked from the real PR's description, and the throwaway PR is closed with
  `gh pr close --delete-branch`.
  *Result (ADR-084):* 648 rows; 2% more skin friction failed validate on all three OSes
  (74 rows, 17 worse; run 36267483063); that change was dropped. Linux's reproduction miss there is #200.
- [x] **M1.11 Ejected sections and payloads.** Added by Neer on 2026-09-18 (VISION V17). A
  separation at any joint, not only a stage boundary (ADR-014): an ejected nose cone, a body
  section, or a payload carried inside, each flown to its own landing under its own recovery
  device, or tumbling; pieces joined by a shock cord fly as one. Ejection triggers as for recovery
  devices (apogee, altitude, timer, motor delay), and an optional ejection impulse.
  *Done when (met by M1.11a):* a design that ejects its nose cone and a payload, each under its own
  parachute, lands every piece and reports each landing point; the pieces' masses sum to the
  rocket's and momentum is conserved at each split, to M1.7c's tolerances; each descent rate
  matches the analytic terminal velocity for its device and mass.
  - [x] **M1.11a Pieces at any joint** (ADR-085), done when the milestone's bullets. *Result:* met:
    nose cone at apogee, 250 g payload at 300 m, each lands within 0.1% of `v_e`; 1e-12, 1e-9.
  - [x] **M1.11b Ejection impulse and tumbling pieces** (ADR-086). An optional impulse, equal and
    opposite on the two pieces, and a tumble model over a piece's own components. *Done when:* an
    impulse gives each piece the hand-computed change of velocity and conserves momentum to 1e-9,
    and a tumbling nose cone lands at its tumble model's terminal speed. *Result:* met. 1 N·s:
    `J/m` to 1e-9 at both partings (4 m/s on the 250 g payload), momenta to 1e-9; the nose cone
    tumbles down at 13.849 m/s, its `v_e` to 1e-6. Tumble side areas now integrate curved noses.
- [x] **M1.12 Mass that moves or leaves in flight.** Added by Neer on 2026-09-18 (VISION V18).
  Payload mass that moves along the airframe, or leaves it (released ballast or payload), on an
  event or a schedule, with the mass, centre of gravity and inertia updated through the flight;
  the equations of motion carry the moving mass's relative-motion terms, or an ADR shows, with
  numbers, that they are negligible. *Done when (met by M1.12a and b):* mass properties before,
  during and after a change match hand-computed values, and a release conserves mass and momentum;
  a test shows a moving mass shifting the stability margin as the hand calculation predicts.
  - [x] **M1.12a A mass that moves along the airframe.** Met (ADR-087), bars kept: a shift's mass
    properties to 1e-15, the margin to 1e-12 cal (4.30 to 3.00), momenta off the axis to 6.9e-12.
  - [x] **M1.12b Mass released in flight.** Met (ADR-088), bars kept: after a release, mass
    properties to 1e-15, momentum to 1.5e-13 and angular momentum to 7.3e-12, the rest in 6-DOF.
- [x] **M1.13 Pods.** Added by Neer on 2026-09-18 (VISION V19). Split a to c (ADR-089). Side pods
  and outboard motor pods (M1.9's clusters): mass off the axis, each pod's normal force, drag and
  body interference from a cited source; the `.ork` importer reads pods. *Done when* a pod's mass
  properties match the hand-computed parallel-axis values, and a pod design matches OpenRocket
  within the per-case tolerance, both codes' pod limits stated in the docs. Met:
  - [x] **M1.13a Pod mass** (ADR-089): two pods, one off the axis (`I_yz = −m y z`), to 1e-15; a
    motor in a 3-pod set is three motors.
  - [x] **M1.13b `.ork` pods** (ADR-090): 9 of 9 read (12 pods), cached numbers 71 of 75 agree.
    - [x] **M1.13b1 Pods of body components.** 5 of 9, placed as OR does to 1e-15 m, 9 probes.
    - [x] **M1.13b2 Pods of no length**, and an empty pod set (ADR-091): the other 4.
  - [x] **M1.13c Pod aerodynamics** (ADR-092), met by c2.
    - [x] **M1.13c1 Pods fly**, Barrowman's rules once per pod: hand values to 1e-11; a private
      lug-pod design within 2.2% of OR.
    - [x] **M1.13c2 A pod design against OpenRocket** (ADR-093): six probes, apogee +0.41% to
      +0.81%, speed to +1.24%, the pods' change within 0.32 points.
- **M1.14** is open; its entry: [roadmap](ROADMAP.md#phase-1-physics-core-the-heart-with-validation-interleaved).
  - [x] **M1.14a Warnings,** four flags (ADR-143 §1). *Done when:* beyond the validated range, high
    angle of attack, outside the core band, beyond the envelope: each warns, tested at its edge;
    and each issue on ADR-163 §4's list warns, by number, when a flight or design meets its Mach
    or geometry condition, tested at its edge.
    - [x] **M1.14a1 Envelope flags** (ADR-179). *Done when:* beyond the validated range, high
      angle of attack, outside the core band and beyond the envelope each warn from the library,
      `hpr sim` (text and JSON) and Python, each tested at its edge; a test reads the validated
      range's Mach from the committed public reports.
    - [x] **M1.14a2 Drag issue warnings** (ADR-180). *Done when:* #67, #68, #70, #72 and #222
      each warn, by number, when a flight or design meets its Mach or geometry condition, tested
      at its edge, in the library, `hpr sim` and Python.
    - [x] **M1.14a3 Stability issue warnings** (ADR-180, ADR-181). *Done when:* #87, #120, #121 and #172
      each warn, by number, when a flight or design meets its Mach or geometry condition (#172's
      chosen in an ADR, as it states none), tested at its edge.
## Phase 2: Library surfaces and interop

- [x] **M4.1 Facade API.** The `hpr` crate offers a RocketPy-like builder (`Environment`, `Motor`,
  `Rocket`, `Flight`) plus trait-based custom models. Add `examples/` (at least 4) and a rustdoc
  guide.
  - Loft lessons: L95.
  *Done when:* the examples run in CI; rustdoc has zero warnings; a "custom aero model" example
  overrides a built-in model through the trait. Split into a and b (ADR-103). Met, bars kept: M4.1a
  (L95) flies the builder's rocket as `own_rocket.rs`'s bit for bit, each refusal pinned, two
  examples and a guide page (ADR-103); M4.1b adds `custom_drag`, 5 examples (ADR-104).
  - [x] **M4.1a The builder.**
    - Loft lessons: L95.
  - [x] **M4.1b Custom models and the rustdoc guide.**
- [x] **M4.2 CLI.** `hpr sim|validate|convert|motors|mc|optimize|compare|analyze|diagnose` (stubs
  are fine for commands whose milestone hasn't come yet), `--json` everywhere, and shell
  completions. The README's command and format table is generated from the registered commands
  (Loft lesson P10). `hpr analyze <log>` reads a flight log and prints its readings; it takes no
  design and runs no simulation (ADR-046), which is how the analyzer reaches a user who only ever
  wants that.
  *Done when:* `assert_cmd` tests cover every implemented command and the JSON output validates
  against the published schemas, and `hpr analyze` is tested on a log with no design file present.
  Split into a to d (ADR-105). Met, bars kept (ADR-105 to 108): M4.2a `motors`, `completions` and
  every refusal, schemas and a stale-table check; M4.2b a public `.ork` flown bit for bit (the pod
  probe, `--motor H54`); M4.2c a spoiled copy fails three ways, 32 curves round-trip; M4.2d the
  public Pnut log, 1,010 ft against 1,009 ft.
  - [x] **M4.2a The command surface and `hpr motors`.**
  - [x] **M4.2b `hpr sim`.**
  - [x] **M4.2c `hpr validate` and `hpr convert`.**
  - [x] **M4.2d `hpr analyze`.**
- [x] **M3.2 OpenRocket `.ork` export** (schema 1.10).
  - Loft lessons: L67, L68.
  *Done when:* `.ork` → hpr → `.ork` → OR 24.12 (oracle) loads every corpus design; OR's flight of
  the export is within 0.5% of the original's apogee. Split into a and b (ADR-109). Met, bars kept:
  M3.2a writes from the design and `x-openrocket`, 73 of 73 read back the same and byte for byte,
  L67's and L68's tests, a hand-derived document (ADR-109); M3.2b, OR opens 71 of 75 as written,
  all 71 exports, 151 of 151 within 0.5% (ADR-110).
  - [x] **M3.2a The writer.**
    - Loft lessons: L67, L68.
  - [x] **M3.2b OpenRocket flies the export.**
- [x] **M3.3 The hpr open design format v0.1.** See the brief in `ARCHITECTURE.md`. Spec in
  `docs/format/`; JSON Schema generated by `schemars` under `schema/`. Migrations framework; zip
  container. TypeScript and Python type generation.
  *Done when* (1) every corpus design round-trips `.ork` → hpr format → `.ork`, and hpr's own
  simulated apogee for the result matches the original import's to within 1e-9 relative; (2) schema
  validation runs in tests; (3) the spec includes a comparison table against `.ork`, `.rkt`, `.CDX1`
  and `.rpy`; (4) an ADR records the file extensions and versioning policy.
  Split into a to c (ADR-111). Met, bars kept: M3.3a `.hpr`, 73 of 73 round-trip, 109
  configurations to one apogee, 17 public designs schema-checked in CI (ADR-111); M3.3b `.hprz`
  byte for byte, a 0.1 document and 73 corpus ones migrate, the table in `hpr.md` (ADR-112);
  M3.3c TypeScript and Python readers agree with the schema on 4,892 mutations (ADR-113).
  - [x] **M3.3a The document.**
  - [x] **M3.3b The container, migrations and the comparison.**
  - [x] **M3.3c Generated types.**
- [x] **M4.3 Python bindings.** `hpr-py` (PyO3 abi3 + maturin) with numpy outputs, a RocketPy-like
  API, and Python callbacks for custom models. pytest suite; CI builds wheels on 3 operating
  systems (no publishing). *Done when:* pytest passes in CI; a notebook-style example reproduces
  a RocketPy example flight via hpr within the M2.1 tolerance. Split into a to c (ADR-114). Met:
  M4.3a pytest on three OSes against a wheel built there; M4.3b `DragTable`, `calisto.py` within
  3% on all 14 metrics scored in % (largest: landing drift, +1.257%) (ADR-115); M4.3c drag and
  wind as Python functions, their exceptions raised in Python (ADR-116).
  - [x] **M4.3a The package.**
  - [x] **M4.3b RocketPy's example.**
  - [x] **M4.3c Python models.**
- [x] **M5.1 Online layer and cache.** `hpr-net`: HTTP client (rustls), on-disk cache (platform
  dirs), TTLs, an explicit offline mode, attribution strings. *Done when:* tests run against
  recorded fixtures (no live network in CI); offline mode never touches the network (asserted by
  test). Split into a and b. Met: M5.1a an offline fetch never calls the transport (one that fails
  if called) and serves a stale entry marked stale, in recorded-fixture tests, `tests/offline.rs`
  (ADR-117); M5.1b a loopback server replaying a recorded fixture fills the cache, `cargo deny`
  passes, `tests/http.rs` (ADR-118).
  - [x] **M5.1a Cache and offline mode** (TTLs, `Mode::Offline`, attribution).
  - [x] **M5.1b HTTP.** `ureq` with rustls behind feature `http`, the platform cache directory.
- [x] **M5.2 Weather.** Split a to d (ADR-119): Open-Meteo forecast and historical-forecast with
  pressure-level winds, turned into an atmosphere/wind profile; GFS/RAP GRIB2 (pure Rust); U.
  Wyoming soundings; offline import of ERA5/GFS files the user provides. *Done when:*
  recorded-fixture tests pass; a profile built from a recorded Open-Meteo response reproduces the
  pressure, temperature and wind values at the pressure levels. Met: M5.2a Open-Meteo, 2
  recordings, ground and 14 levels to rounding (ADR-119); M5.2b Wyoming, 3 recordings, every kept
  row to rounding (ADR-120); M5.2c GFS/RAP cuts, 6,123 values to 2.2e-16 of ecCodes (ADR-121);
  M5.2d `hpr weather` offline from each source (d1, ADR-122), a whole GFS file's 746,770,303
  values to 4.4e-16 (d2, ADR-123), 4 RAP JPEG 2000 messages (d3, ADR-124).
  - [x] **M5.2a Open-Meteo.**
  - [x] **M5.2b U. Wyoming soundings.**
  - [x] **M5.2c GFS/RAP GRIB2, pure Rust.**
  - [x] **M5.2d Files the user provides, and `hpr weather`.**
    - [x] **M5.2d1 `hpr weather`.**
    - [x] **M5.2d2 Complex packing** (5.2, 5.3).
    - [x] **M5.2d3 JPEG 2000** (5.40).
- [x] **M5.3 Site data.** Elevation (Open-Meteo API with cache; optional user GeoTIFF/DEM file),
  geodetic helpers, magnetic declination (WMM2025). Split a to c (ADR-125). *Done when:* WMM
  matches NOAA test values; elevation lookups are cached and work offline after the first fetch.
  Met: M5.3a WMM2025, Table 6 to its printing, NCEI's 100 points bar `X`'s 7.18e-4 nT residue
  (ADR-125); M5.3b Open-Meteo elevation, 4 heights exact, offline from the cache (ADR-126);
  M5.3c1 Karney's 500,000 geodesics within 15 nm (ADR-127); M5.3c2 a GeoTIFF read as rasterio
  reads it, 7 fixtures and a USGS tile (ADR-128).
  - [x] **M5.3a WMM2025.**
  - [x] **M5.3b Elevation.**
  - [x] **M5.3c Geodetic helpers and a user's elevation file.** Split c1, c2 (ADR-127).
    - [x] **M5.3c1 Geodesics** on WGS 84.
    - [x] **M5.3c2 A user's GeoTIFF.**
- [x] **M5.4 Motor stock and prices.** motor.fusionspace.co client (`meta`, `motors`, `in-stock`,
  `vendors`, and per-motor endpoints), joined with ThrustCurve curves; offline snapshot; `hpr
  motors search --in-stock --class L --max-price 150`. *Done when:* recorded-fixture tests pass;
  the designation to ThrustCurve id mapping covers at least 95% of in-stock motors, with a report
  of the misses; attribution is displayed as the API asks. Split a to c (ADR-129). Met: M5.4a 8 answers read back and offline, the credit on each (ADR-129); M5.4b 282 of 282
  mapped, J450DM's file read (ADR-130; stand-in searches since, ADR-145); M5.4c none at $150 on the recording, the one on an edited
  copy, offline too (ADR-131).
  - [x] **M5.4a The motor finder's API** (`hpr_net::motor_finder`).
  - [x] **M5.4b ThrustCurve.** `hpr_net::thrustcurve`.
  - [x] **M5.4c `hpr motors search`.**
- [x] **M5.5 Parts catalog.** Import the OpenRocket `.orc` component database (Apache-2.0, with
  notices). Lookup by vendor and part number; parts can be used from the design API. *Done when:*
  all `.orc` files parse, and a design built from catalog parts simulates. Split a and b (ADR-132).
  - [x] **M5.5a The `.orc` reader** (`hpr_io::orc`), the 16 files OpenRocket 24.12 ships bundled.
    *Done when:* every bundled file reads, every part OpenRocket's preset loader returns (the
    oracle) with each value equal to its reading bar named, counted departures; parts are found
    by maker and part number. *Result:* met (ADR-132): 3,449 parts; 17,911 of 18,306 numbers to
    the bit, the rest 185 ounces, 207 stated-mass densities, 3 undefined; 252 makers' names.
  - [x] **M5.5b Catalog parts in the builder.** *Done when:* a rocket built from catalog parts
    flies through the builder; each part's mass as built is held to OpenRocket's for its preset.
    *Result:* met (ADR-133): 3,445 built, 4 refused; each held to OpenRocket or its gap counted
    with its cause: hollow shoulders (OR's weigh nothing), two wall definitions, ounces, a streamer.
- [x] **M4.5 Fly my .ork** (ADR-144 §8). On 2026-10-03 none of OpenRocket's 17 example files flew
  as saved in `hpr sim`, and the CLI flew 4 of 170 configurations. *Done when:* every in-scope OR
  example flies as saved with recovery and its parts' drag overrides (#165; *Base drag hack* within
  5% of OpenRocket's apogee), offline after one fetch;
  the CLI's corpus count is published.
  *Result (ADR-173):* 54 of OpenRocket's 56 example configurations fly as saved, offline after one
  fetch; the other two are hybrid motors, out of scope until 1.0. The CLI's count is published
  (M4.5j). *Base drag hack* with recovery in both: C11-5 +1.95%, D12-3 +4.52%, met. **Not met,
  recorded:** E12-4 +7.80%, OpenRocket's unmeasured subsonic nose drag (M4.5o).
  - [x] **M4.5a Recovery from a `.ork`** (#240). *Done when:* held to OR's descents, against a
    target set before measuring. Shipped 2026-10-04 (ADR-153): `hpr::ork::recovery` and `hpr sim`
    fly a `.ork`'s parachutes and streamers; of 28 public descents in scope, landing speeds within
    0.02% and flight times within 3.2% of OpenRocket's, 21 meeting every target as written and all
    28 once the apogee's difference and OpenRocket's step are taken out of the deployment times.
  - [x] **M4.5b Motors on demand.** *Done when:* `hpr sim` fetches and caches a missing motor.
    Met (ADR-154): `--motor NAME` and a `.ork`'s curve-less motor are fetched from ThrustCurve.org
    by name, cached and flown offline after; `hpr motors fetch` pre-loads; offline it names the
    command. Live on 2026-10-04 (not committed), 5 OR example files that lacked a curve fly.
  - [x] **M4.5c Design checks that match reality.** *Done when:* everyday fits warn, with a cited
    tolerance; impossible fits stay errors.
    Met (ADR-155): a part wider than its parent's bore warns within ISO 2768-1's coarse tolerance
    for the bore's size, or as a ring capping its parent's end, and is an error past that; a
    nominal 18, 24 or 29 mm motor warns while AeroTech's drawn case fits the bore. No OR example
    raises a design error; the private library's fall from 13 flights to 8.
  - [x] **M4.5d Readable output** (ADR-156). *Done when:* configurations by name, not UUID;
    sentences, not JSON (JSON with `--json`).
  - [x] **M4.5e Plots** (ADR-157). *Done when:* `hpr sim --plot` writes the fixed SVG figure set.
  - [x] **M4.5f How-to guides.** *Done when:* the three guides and the README section are published.
    Done 2026-10-04 (ADR-158): "Fly your .ork", "Pick a motor" and "Check stability for a
    certification flight" on the site, the README's "Install and first flight"; their outputs are
    run and checked by `cargo xtask cli`; `hpr sim --delay` added for `--motor`.
  - [x] **M4.5g Then by blocked count:** unpowered separation (#184), freeform fins, multiple
    separations (#183), a powered separation and a separated stage's devices in `hpr sim` (#240's
    second item). *Done when:* each flies, or is refused by name. Split by OpenRocket example, and
    met when its four increments are:
    - [x] **M4.5g1 A powered separation in `hpr sim`** (#240's second item). *Done when:* the two
      configurations of OpenRocket's *Two stage high power rocket* fly as saved in `hpr sim`, each
      part under its own parachutes, a part with none tumbling, and each part's landing printed;
      the flight is the library's bit for bit (an `assert_cmd` test); `cargo xtask ork-flights`
      compares the sustainer's descent with OpenRocket's against ADR-153's targets.
      Met (ADR-159): both fly; the sustainer lands within 0.004% of OpenRocket's landing speed,
      flight time +0.32% and +0.73%, one target met as written and the other once sized.
    - [x] **M4.5g2 Several separations** (#183). *Done when:* each configuration of OpenRocket's
      *Three stage low power rocket* flies in `hpr sim` within ADR-076's 5% of OpenRocket's apogee
      and top speed; one that misses stays refused, its measured error in the report and an ADR.
      Met (ADR-160): all three fly in `hpr sim`, the library's flight bit for bit; on OpenRocket's
      own curves apogees are within 1.48% and top speeds within 1.64%, both separations at
      OpenRocket's times. `hpr sim`, fetching the ThrustCurve files that hold OpenRocket's curves
      (ADR-161), reads apogees within 2.48% and top speeds within 1.67%.
    - [x] **M4.5g3 An unpowered separation before apogee** (#184). *Done when:* *Deployable
      payload* and *ARC payload rocket* fly in `hpr sim`, the part keeping the nose held to
      OpenRocket's record (apogee within 5%, its descent to ADR-153's targets); a part hpr can't fly
      honestly stays refused, with an ADR that measures what flying it would get wrong.
    - [x] **M4.5g4 Freeform fins.** *Done when:* *Pods--airframes and winglets*, whose freeform
      outlines don't run from the root's leading edge to its trailing edge, flies in `hpr sim` with
      them read as written, each configuration's apogee within 5% of OpenRocket's record; or an ADR
      measures why such an outline can't be read as written, and the refusal names it.
  - [x] **M4.5h A part's drag override** (#165). *Done when:* *Base drag hack (short-wide)* flies in
    `hpr sim` with its part's drag-coefficient override applied, each configuration's apogee within
    5% of OpenRocket's record (today −15.5% to −19.1%).
    *Result (ADR-167):* the override flies as OpenRocket's, measured on 25 probes. With recovery in
    both, as `hpr sim` flies it: C11-5 +2.06% and D12-3 +4.65%, met. **Not met, recorded:** E12-4
    +7.97%, the drag coefficient (the nose by OpenRocket's breakdown, #177): on OpenRocket's drag
    hpr comes within 0.03% of OpenRocket's flight with nothing deployed.
  - [x] **M4.5i A powered pods example** (#329). *Done when:* each configuration of OpenRocket's *Pods--powered
    with recovery deployment* that OpenRocket completes flies in `hpr sim`, its screw heads weighed
    as OpenRocket's probes measure them, each pod set's motors burning into its own pods' bases,
    and a device whose stage has no lit motor opening as OpenRocket's does; each apogee within 5% of
    OpenRocket's record; the margin it prints its weakest plane's (#329).
    *Result (ADR-168):* all three fly; `[C6-5; None]` −0.36% and `[None; 2× A10-3, B6-4]` −1.86%,
    descents met. **Not met, recorded:** `[C6-7; B6-0]` −37.9%: its sustainer, −4.21 cal in its
    weakest plane, turns over before apogee in both tools (OpenRocket's record tumbles at 2.84 s).
  - [x] **M4.5j The CLI's corpus count.** *Done when:* `cargo xtask ork-cli` flies every motor
    configuration of `cargo xtask ork`'s survey (170) through `hpr sim --offline` after one fetch,
    counting those that fly as saved, with `--accept-design-errors`, and refused by reason, into a
    committed report; the `.ork` format page quotes the report's sentences, a test holding them
    equal; the roadmap's and known risks' "4 of 170" replaced by the measurement.
    *Result:* met: 108 of 170 fly as saved, 32 more with the flag, 30 refused (21 a motor with no
    curve to find, 3 hybrids, 2 a motor in a part not read, 2 an airframe not read as written, 1
    an ignition, 1 a separation); of OpenRocket's 56, 38, 13 and 5.
  - [x] **M4.5k A ring around its tube.** *Done when:* a centering ring written as a child of the
    motor tube it wraps (its bore at least the tube's outside) is checked for room in what holds the
    tube, so *Pods--airframes and winglets*' five configurations and *Pods--powered*'s three that
    OpenRocket completes fly as saved in `cargo xtask ork-cli`'s report; a ring inside its tube, or
    too wide for what holds it, stays an error (a test, with mutation probes).
    *Result (ADR-169):* met: all eight fly as saved; OpenRocket's examples 46 of 56 as saved (38
    before), the survey 121 of 170 (108); *Deployable payload*'s five still need the flag. A ring
    is measured in the part around it at its own station that holds it whole: what holds the tube
    for all but one of *Pods--powered*'s rings, which sits in the airframe ahead of the coupler.
  - [x] **M4.5l A packed part wider than its bore.** *Done when:* a mass component, parachute,
    streamer or shock cord drawn wider than its parent's bore, its centre inside the bore, warns
    instead of stopping the flight, so *Deployable payload*'s five configurations fly as saved in
    `cargo xtask ork-cli`'s report, with the width's effect on their flights measured; a ring as
    wide, or a packed part centred past the bore, stays an error (tests, with a mutation probe).
    *Result (ADR-170):* met: all five fly as saved, each apogee within 12 µm, 7.5e-8 of the
    apogee, of the same flight with both parts narrowed to their bore; OpenRocket's examples 51 of
    56 as saved (46 before), none needing the flag; the survey 131 of 170 (121), 9 with the flag
    (19).
  - [x] **M4.5m Parallel booster staging.** *Done when:* *Parallel booster staging*'s two
    configurations fly as saved in `cargo xtask ork-cli`'s report, the boosters beside the core
    burning and dropping at their own separation, each apogee within 5% of OpenRocket's record
    with recovery as saved, under an ADR on how a parallel stage is read and flown.
  - [x] **M4.5n Pods--powered's first configuration.** *Done when:* `hpr sim` flies *Pods--powered
    with recovery deployment*'s first configuration as saved, its motors in three mounts burning
    out in the order their curves give, under an ADR naming what it is checked against, since
    OpenRocket aborts its own run of it.
  - [x] **M4.5o Base drag hack's E12-4.** *Done when:* *Base drag hack*'s E12-4 apogee is within
    5% of OpenRocket's with recovery as saved (8.0% above on 2026-10-05,
    [#177](https://github.com/nrdptel/fusionspace-eridanus/issues/177)), from a blunt nose's subsonic pressure
    drag taken from a cited measurement and pinned by a test against it.
    *Result (ADR-173):* an ellipsoid takes Hoerner's measured forebody pressure drag below Mach 0.8
    (1965 p. 3-12, Fig. 20), pinned by
    `nose_drag::tests::a_blunt_ellipsoid_takes_hoerners_measured_forebody_drag`. This nose gets
    0.0008, so the E12-4 reads +7.80% with recovery in both (340.99 m against 316.32 m; `hpr sim`
    341.4 m, +7.9%). **Not met, recorded:** within 5% needs about 0.013 to 0.015 (a one-off probe
    and the report's own shift), more than the measured hemisphere's 0.01 for a longer head.
- [x] **M4.6 Monte Carlo from the CLI and Python** (ADR-163). *Done when:* `hpr mc` flies any
  design `hpr sim` flies with M6.1's scatter, prints apogee and landing spreads with a landing
  ellipse and writes the runs as CSV in round-trip form; Python returns the same runs as arrays;
  both equal the Rust API's runs for the same seed, build and platform, bit for bit; failed runs
  are counted and printed.
  - [x] **M4.6a `hpr mc`** (ADR-182). *Done when:* `hpr mc` flies any design `hpr sim` flies,
    staged `.ork` flights included, with M6.1's eleven dispersions as flags; prints apogee and
    landing spreads with a landing ellipse and the failed runs' count; writes the runs as CSV that
    reads back to the same bits; and a test shows its runs equal `MonteCarlo::run`'s for the same
    seed, bit for bit.
  - [x] **M4.6b Monte Carlo in Python** (ADR-183). *Done when:* Python returns M4.6a's runs as arrays, its
    failed runs counted, equal bit for bit to `hpr mc`'s CSV from the same build and seed.
## Phase 3: Uncertainty, optimization, challenges

- [x] **M6.1 Monte Carlo and sensitivity.**
  - Seeded, parallel dispersion over: mass/CG, Cd scale, motor impulse and timing (certification
    tolerances), wind, launch angle, deployment delays.
  - Landing ellipses at confidence levels; apogee distribution.
  - Sensitivity analysis (Morris screening and Sobol indices).
  - Loft lessons: L52, L53, L54, L55, L96.
  *Done when:* results are bit-reproducible for the same seed; the ellipse math is tested against
  analytic Gaussians; 10,000 flights of an L2 design finish in ≤10 s on the dev machine (recorded).
  Split a to d (ADR-134).
  - [x] **M6.1a Seeded dispersion.** *Done when:* the same seed gives bit-identical samples
    whatever the run's length or thread count; zero dispersion flies the nominal bit for bit;
    failures counted; the apogee's distribution reported; the lessons' tests pass. Met (ADR-134).
  - [x] **M6.1b Landing ellipses** at confidence levels. *Done when:* the ellipse math (covariance,
    axes, the χ² scale of a confidence level) is tested against analytic Gaussians. *Result:* met
    (ADR-135): axes to 1e-14, a density's mass inside to 1e-12, sampled shares within 5σ.
  - [x] **M6.1c Sensitivity:** Morris and Sobol. *Done when:* both match the known indices of test
    functions with closed forms (Ishigami, Sobol's g) within sampling error. Met (ADR-136).
  - [x] **M6.1d 10,000 flights.** *Done when:* 10,000 flights of an L2 design finish in ≤10 s,
    recorded in `docs/perf.md`. Met (ADR-137): Valetudo 3.00 s, a supersonic K940 rocket 9.43 s.
- **M6.2** is open; its entry: [roadmap](ROADMAP.md#phase-3-uncertainty-optimization-challenges).
  - [x] **M6.2a CMA-ES, continuous variables**: four test functions' minima from 20 seeds (Rosenbrock 17), within
    5% of pycma's evaluations; 3,048 m and 2.2 cal in 300 flights, re-flown. Met (ADR-138).
  - [x] **M6.2b Discrete variables and constraints**, split b1, b2 (ADR-139). Met.
    - [x] **M6.2b1 Constraints** (Deb's rules): three problems to 1e-10 from 20 seeds. Met.
    - [x] **M6.2b2 Discrete choices**: a motor and part chosen for 3,048 m, re-checked. Met (ADR-140).
  - [x] **M6.2c NSGA-II.** *Done when:* ZDT1 to ZDT3 fronts within a stated generational distance;
    a two-goal rocket problem gives a front. Met (ADR-141).
  - [x] **M6.2d EGO.** *Done when:* Branin and Hartmann reach their minima in a stated budget. Split d1, d2 (ADR-142).
    Met by d1 and d2 (ADR-142, ADR-152).
    - [x] **M6.2d1 Branin and Hartmann 3.** *Done when:* each within 1% of its minimum in 50 evaluations from 20 seeds. Met (ADR-142): worst 0.12%.
    - [x] **M6.2d2 Hartmann 6.** *Done when:* within 1% of its minimum in a stated budget from 20
      seeds (d1's EGO ends 7 runs of 10 at its −3.20 local minimum after 100).
      *Plan (ADR-144 §3):* one attempt, the `−ln(−y)` transform; a miss gets an ADR, then move on.
      Met (ADR-152): with `−ln(−y)`, 20 of 20 within 1% in 250 evaluations, worst 0.29%.
## Phase 4: More formats and embeddings

## Phase 5: Flight data and forensics

## Phase 6: Design experience (library level)

## Phase 7: UI, 3D, web, mobile (after release 0.4, ADR-162)

## Phase 8: Releases (ADR-162)

- [x] **M10.1 Release 0.1: the simulator.** The library crates, the CLI and the Python package,
  after M4.5, M1.14a, M4.6 and M2.3c1 (ADR-163). *Done when:* the whole checklist holds; #252,
  #272, #295, #298, #308, #231, #242, #46 and #313 are fixed and #79 decided; and: a CI dispatch builds and
  smoke-tests CLI archives for the three operating systems, abi3 wheels and the source package,
  each carrying its dependencies' license texts (ADR-114); publishing waits on a `release`
  environment; `cargo publish --workspace --dry-run` passes with only `hpr-py`, `hpr-ffi`,
  `hpr-wasm`, `hpr-validate` and `xtask` left `publish = false`; `CHANGELOG.md` and the install
  pages are written.
  Split by ADR-185 and ADR-188 to ADR-190; d7 to d9 added by ADR-198 and ADR-199:
  - [x] **M10.1a The flight and design fixes.** *Done when:* #46, #242, #231 and #313 are fixed,
    each with a test that fails without its fix, and #79 is decided.
  - [x] **M10.1b The data, network and Python fixes.** *Done when:* #252, #272, #295, #298 and
    #308 are fixed, each as its issue's acceptance says.
  - [x] **M10.1c The release workflow.** *Done when:* a CI dispatch builds and smoke-tests CLI
    archives for the three operating systems, abi3 wheels and the source package, each carrying
    its dependencies' license texts (ADR-114); publishing waits on a `release` environment; and
    `cargo publish --workspace --dry-run` passes with only `hpr-py`, `hpr-ffi`, `hpr-wasm`,
    `hpr-validate` and `xtask` left `publish = false`.
  - [x] **M10.1d1 The release notes and the install pages** (ADR-188). *Done when:*
    `CHANGELOG.md` holds 0.1.0's entry (what it holds, what works, how far to trust it, the known
    gaps), whose gaps list by number every open `env-core` issue on the flattering side or of
    unknown sign, warned or not; `cargo test -p xtask` checks its links, its labels and its
    version's entry; and the README, *Getting started*, *Python* and *The API reference* say how
    to install the release.
  - [x] **M10.1d2 The safety warning and the shape warnings** (ADR-188, ADR-189). *Done when:*
    #335 is fixed: a flight whose static margin is negative while a motor burns, before apogee,
    says in `hpr sim`'s text and JSON and in the library's summary that it is unstable under
    power and its apogee is not a prediction, tested on both sides of a margin of zero; #64,
    #73, #325 and #367 each print a warning naming them on a flight that meets their
    condition, pinned by tests on both sides of it; and #172's warning quotes the largest
    measured gap, 0.1108 calibres.
  - [x] **M10.1d3 The friction and freeform fin warnings** (ADR-190). *Done when:* #18 and #326
    each print a warning naming them on a flight that meets their condition, pinned by tests on
    both sides of it.
  - [x] **M10.1d5 The separated parts' warnings** (ADR-190, ADR-200). *Done when:* #179 and
    #354 each print a warning naming them on a flight that meets their condition, tested on both
    sides.
  - [x] **M10.1d6 The unknown signs** (ADR-190, ADR-201). *Done when:* #8, #104, #106, #213, #219, #360
    and #361 each have a sign recorded from evidence or a warning.
  - [x] **M10.1d7 The designation and the design pin** (ADR-198). *Done when:* stamps read
    `FS-ACHERNAR · SW · TOOL 001`; an `.eng` stamped `FS · SW · TOOL 005` reads without keeping
    that stamp and writes back with one, its tests keeping the old literal; the design pin at its
    head with ADR-164's change list; `docs/writing.md` has the unit's non-breaking space; the
    README's design section and banners are current.
  - [x] **M10.1d8 The new name's URLs** (ADR-198), after Neer renames the repository. *Done when:*
    outside `docs/decisions/`, no tracked file names `nrdptel/hpr-sim` (but `-fixtures`) or
    `nrdptel.github.io/hpr-sim`, nor `book.toml`'s `site-url` or xtask's `SITE_PATH` `/hpr-sim/`;
    `cargo xtask site` passes; the maintainer's PyPI step names the new repository.
  - [x] **M10.1d9 FusionSpace HPR** (ADR-199, ADR-203). *Done when:* the dry runs build PyPI's
    `fusionspace-hpr` (`import fusionspace.hpr`) and every published crate as `fusionspace-hpr` or
    `fusionspace-hpr-*`; release title and archives, `hpr --version` and `--help`, stamps, site and
    README titles, User-Agent and cache folders follow ADR-199 §4; no doc or script outside
    `docs/decisions/` says `pip install hpr-sim` or `import hpr`; an `.eng` stamped `hpr-sim` with
    either designation reads without keeping it, its tests keeping the old literals.
  - [x] **M10.1d4 The checklist** (ADR-189), after d5 to d9. *Done when:* ADR-162 §2's whole
    checklist holds at a commit on `main`, shown by a `Release` dispatch from that commit, and
    `CHANGELOG.md`'s known gaps list no flattering-side issue as unwarned.
## Phase 9: The suite's new lines, one at a time after 1.0 (ADR-162 §4, ADR-163 §5)


