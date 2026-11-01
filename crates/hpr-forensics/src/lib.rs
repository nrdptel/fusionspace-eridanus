//! Flight against simulation: residuals, parameter identification and fault diagnosis.
//!
//! [Start here][guide-start] says what works today and what is planned.
//!
//! [guide-start]: https://hpr.fusionspace.co/start-here.html
//! [roadmap]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/ROADMAP.md
//! [adr-046]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/DECISIONS.md#adr-046-debrief-folded-in-and-flight-log-analysis-that-stands-without-the-simulator-2026-09-20
//! [adr-280]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0280-m7-2-in-seven-the-synthetic-flights-first.md
//!
//! This is the only crate where a reading taken from a flight log meets a number the simulator
//! produced. Reading a log and working out what it says is [`hpr_flightdata`]. That crate doesn't
//! depend on the simulator, so that [analyzing a flight stands on its own][adr-046].
//!
//! [`synthetic`] makes the flights whose truth is known, on which a log's readings and fits are
//! tested ([the synthetic flights first][adr-280]). [`residuals`] sets a log against a simulation
//! of it. Parameter identification is planned, and so is fault diagnosis ([the roadmap][roadmap]).

pub mod residuals;
pub mod synthetic;
