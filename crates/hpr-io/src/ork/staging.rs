//! When a `.ork` design's motors light and where its stack comes apart, in the terms HPR Sim's flight
//! takes ([M1.9c][m1-9c], decision [ADR-076][adr-076]).
//!
//! **Ignition.** Each motor's `<ignitionevent>` and `<ignitiondelay>` ([`Ignition`]) become an
//! [`hpr_design::Ignition`]. OpenRocket's words are measured by a committed probe (the
//! [`.ork` format page][format]):
//!
//! | the file says | HPR Sim lights the motor |
//! |---|---|
//! | `launch` plus `d` | at `t = d` |
//! | `automatic` plus `d`, in the bottom stage | at `t = d` |
//! | `automatic` plus `d`, in a stage above | as `ejectioncharge` |
//! | `ejectioncharge` plus `d` | `x + d` after the burnout of the stage below's motor, whose ejection delay is `x` |
//! | `burnout` plus `d` | `d` after the stage below's first burnout |
//! | `never` | never ([`hpr_design::Ignition::Never`]) |
//! | `burnout` or `ejectioncharge`, in the bottom stage | never: it has no stage below |
//! | `ejectioncharge` or `automatic`, the stage below's motor plugged | never: that fires no charge |
//!
//! "The stage below" is the next stage aft. Its first burnout is its first motor's to burn out, by
//! each motor's ignition and curve, the first in the file on a tie, as OpenRocket 24.12 flies it
//! ([ADR-172][adr-172], `first_burnout.py`'s probe). For a charge its motors must sit in one mount (one tube
//! or one cluster): which of several fires first is not measured. A motor that never lights is carried
//! loaded, with no thrust, as OpenRocket 24.12 flies it ([M2.2e10][m2-2e10]), and so is one that
//! waits on it, which has no burnout or charge to wait on (inferred: no probe chains two). `unlit_motors.py` sets the booster of OpenRocket's two-stage example so
//! that it, or the sustainer waiting on its plugged charge, never lights: OpenRocket lights only
//! the other motor and loses only that one's propellant. A configuration is not flown when a
//! motor waits on a stage below that holds no motor, or on the charge of a stage below with
//! motors in more than one mount, or of a motor below that states no ejection delay, or has a
//! negative delay, or when no motor of it lights at all.
//!
//! **Separation.** A stage's `<separationevent>` says when it drops away from the stage ahead of
//! it. HPR Sim flies a separation, as a [`Staging`], when its time is known before the flight (a time
//! after launch, or a motor's burnout or ejection charge). When the part ahead of it still has a
//! motor to burn then, burning or due to light, that part is a sustainer, and HPR Sim flies it on
//! ([Staging][staging]). The motors of the part that drops away must have burned out by then, but
//! at a stage's own `burnout` (its first) under power: there the stage's other motors drop still
//! burning, as in OpenRocket, and the flight leaves their thrust out of the part's flight as a
//! point ([ADR-172][adr-172]). A motor behind still to light is refused either way.
//!
//! A separation at its own motor's burnout or ejection charge, when that motor never lights, never
//! comes, as in the probes, and is left out. One at the ignition of a motor that never lights, or
//! at launch (HPR Sim's flight fires a separation only once the rocket is off the rod), is not flown.
//!
//! A separation at `apogee` or at a height on the way down can only come at or after apogee, so the
//! climb is the whole stack's in both programs, as long as every motor is spent by then, and the
//! separation belongs to the descent, which HPR Sim's flights of a `.ork` do not fly yet (the decision
//! record on reading recovery, [ADR-056][adr-056]): it is left out, and the configuration flies
//! whole. Whether every motor is spent by apogee is known only once flown; `cargo xtask
//! ork-flights` checks it of every flight it reports. Any other separation with nothing ahead of
//! it left to burn, such as a payload's at the booster's ejection charge, is read when it is the
//! configuration's only one: HPR Sim's flight flies each part from it as a point with its devices'
//! drag alone (the decision record on separation, [ADR-014][adr-014]), so `hpr::ork::tumbling`
//! flies it only when the part keeping the nose has a device of its own open from the split
//! ([ADR-165][adr-165]). Beside another separation it is not flown, as each of several must hand
//! the flight on to a sustainer.
//!
//! **Flying it.** A configuration with a [`Staging`] is among the design's rocket configurations,
//! with each motor's ignition, but the separation is not part of it: flown without one, the stack
//! would carry its booster to the ground with the sustainer lit on it. `hpr::ork::separation` turns
//! a [`Staging`] into the flight's separation, and the example `ork_two_stage` in the `hpr` crate
//! flies one.
//!
//! [m1-9c]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m1-9c
//! [m2-2e10]: https://hpr.fusionspace.co/decisions-and-roadmap.html#m2-2e10
//! [adr-076]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/DECISIONS.md#adr-076-a-ork-files-ignitions-and-one-powered-separation-flown-against-openrocket-2026-09-25
//! [adr-056]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/DECISIONS.md#adr-056-a-ork-designs-recovery-and-separation-read-as-written-with-openrockets-words-measured-2026-09-21
//! [adr-014]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/DECISIONS.md#adr-014-separation-bodies-their-masses-and-their-descents-2026-09-17
//! [adr-165]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0165-an-unpowered-separation-in-hpr-sim.md
//! [adr-172]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0172-a-stage-s-first-burnout.md
//! [format]: https://hpr.fusionspace.co/format/ork.html#delays-and-ignition
//! [staging]: https://hpr.fusionspace.co/physics/staging.html

use hpr_motor::Delay;
use serde::{Deserialize, Serialize};

use super::motors::{Ignition, IgnitionEvent, OrkMotor};
use super::recovery::{SeparationEvent, StageSeparation};

/// When a separation fires, in the terms of HPR Sim's flight triggers
/// (`hpr_sim::recovery::Trigger`, which this crate does not depend on).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
#[non_exhaustive]
pub enum StagingTrigger {
    /// At a time after launch, s.
    Time {
        /// The time after launch, s.
        time_s: f64,
    },
    /// A delay after the burnout of the motor in a mount (the first tube's, for a cluster, whose
    /// tubes light together).
    Burnout {
        /// The mount's component id.
        mount: String,
        /// The delay after its burnout, s.
        delay_s: f64,
    },
}

/// A separation a configuration flies: where the stack comes apart and when. It is powered when
/// the part ahead of it has a motor burning or still to light then (the flight's test,
/// `hpr_sim::Separation::powered_at`), and otherwise the configuration's only separation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct Staging {
    /// The last stage that stays with the nose; the stages after it drop away.
    pub after_stage: usize,
    /// When.
    pub trigger: StagingTrigger,
    /// When that is, s after launch: known before the flight.
    pub time_s: f64,
    /// Whether it drops motors of its own stage still burning, as OpenRocket's first burnout of a
    /// stage drops the stage's other motors (the decision record on a stage's first burnout,
    /// [ADR-172][adr-172]): only a split at the stage's own `burnout` under power does. The flight
    /// leaves their thrust out of the dropped part's flight as a point.
    ///
    /// [adr-172]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0172-a-stage-s-first-burnout.md
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub drops_burning: bool,
}

/// The motors of `motors` in stage `stage`, with the one mount they sit in, or why there is no
/// single such mount.
fn one_mount(motors: &[OrkMotor], stage: usize) -> Result<Option<&OrkMotor>, String> {
    let mut in_stage = motors.iter().filter(|m| m.stage == stage);
    let Some(first) = in_stage.next() else {
        return Ok(None);
    };
    // A mount holds one motor per configuration, so each other motor is another mount.
    let others = in_stage.filter(|m| m.mount != first.mount).count();
    if others > 0 {
        return Err(format!(
            "stage {stage} holds motors in {} mounts, and which fires its charge first is not \
             measured",
            others + 1
        ));
    }
    Ok(Some(first))
}

/// Each of `motors`' ignition in HPR Sim's terms, in their order, given the stage each stage hangs on
/// (`None` for a stage on the axis, [`hpr_design::ParallelStage`]); or, for each, why HPR Sim can't
/// light it as the file says ([`ignition`]).
///
/// A motor lit at the stage below's `burnout` waits on that stage's first burnout, as OpenRocket
/// 24.12 flies it ([ADR-172][adr-172], `first_burnout.py`'s probe): the first of its motors to
/// burn out, each at its ignition plus its curve's burn time, the first in the file on a tie (no
/// probe sets one). A stage in one mount needs no curve for it: its motor is its own first. With
/// none of a stage's motors lit, the first in the file stands for them: its burnout never comes
/// ([`never_lit`]). A motor waits only on stages further aft, so one pass from the tail forward
/// settles every stage's first burnout before any motor waits on it, each motor once.
///
/// [adr-172]: https://github.com/nrdptel/fusionspace-eridanus/blob/main/docs/decisions/0172-a-stage-s-first-burnout.md
pub(super) fn ignitions(
    motors: &[OrkMotor],
    hung_on: &[Option<usize>],
) -> Vec<Result<hpr_design::Ignition, String>> {
    // Every motor is in one stage, so each entry is written below.
    let mut lit: Vec<Result<hpr_design::Ignition, String>> =
        vec![Ok(hpr_design::Ignition::Never); motors.len()];
    // Each motor's burnout, s after launch: `None` when it never lights.
    let mut out_s: Vec<Result<Option<f64>, String>> = vec![Ok(None); motors.len()];
    // Each stage's first burnout, by the motor's index: `None` when it holds no motor.
    let mut firsts: Vec<Result<Option<usize>, String>> = vec![Ok(None); hung_on.len()];
    for stage in (0..hung_on.len()).rev() {
        let in_stage: Vec<usize> = (0..motors.len())
            .filter(|&i| motors[i].stage == stage)
            .collect();
        for &i in &in_stage {
            lit[i] = ignition(&motors[i], motors, hung_on, &firsts);
            let lit_s = match &lit[i] {
                Ok(hpr_design::Ignition::Launch) => Ok(Some(0.0)),
                Ok(hpr_design::Ignition::Time { time_s }) => Ok(Some(*time_s)),
                Ok(hpr_design::Ignition::Burnout { mount, delay_s }) => motors
                    .iter()
                    .position(|m| m.mount == *mount)
                    .ok_or_else(|| format!("no motor sits in mount `{mount}`"))
                    .and_then(|waited| out_s[waited].clone())
                    .map(|waited_s| waited_s.map(|waited_s| waited_s + delay_s)),
                Ok(hpr_design::Ignition::Never) => Ok(None),
                Ok(_) => Err(format!(
                    "{} lights at an event HPR Sim can't time",
                    motors[i].designation
                )),
                Err(why) => Err(why.clone()),
            };
            out_s[i] = lit_s.and_then(|lit_s| {
                lit_s
                    .map(|lit_s| burn_s(&motors[i]).map(|burn_s| lit_s + burn_s))
                    .transpose()
            });
        }
        let Some(&first) = in_stage.first() else {
            continue;
        };
        firsts[stage] = if in_stage
            .iter()
            .all(|&i| motors[i].mount == motors[first].mount)
        {
            Ok(Some(first))
        } else {
            in_stage
                .iter()
                .try_fold(None, |earliest: Option<(f64, usize)>, &i| {
                    Ok(match out_s[i].clone()? {
                        Some(at_s) if earliest.is_none_or(|(least_s, _)| at_s < least_s) => {
                            Some((at_s, i))
                        }
                        _ => earliest,
                    })
                })
                .map(|earliest| Some(earliest.map_or(first, |(_, i)| i)))
        };
    }
    // A motor in no stage of the rocket's waits on none of its stages.
    for i in (0..motors.len()).filter(|&i| motors[i].stage >= hung_on.len()) {
        lit[i] = ignition(&motors[i], motors, hung_on, &firsts);
    }
    lit
}

/// `delay_s`, or why it can't be flown: a delay is a time after its event.
fn delay(delay_s: f64, what: &str) -> Result<f64, String> {
    if delay_s.is_finite() && delay_s >= 0.0 {
        Ok(delay_s)
    } else {
        Err(format!(
            "its {what} delay, {delay_s} s, is not a time after its event"
        ))
    }
}

/// When `motor` lights in HPR Sim's terms, given every motor of its configuration, the stage each
/// stage hangs on (`None` for a stage on the axis, [`hpr_design::ParallelStage`]) and the first
/// burnout of each stage aft of `motor`'s ([`ignitions`]); or why HPR Sim can't light it as the file
/// says.
///
/// The stage below is the next stage on the axis: a parallel stage is beside its stage, not below
/// it. A motor in a parallel stage lights `automatic`ally as one in the stage it hangs on does, at
/// launch when that is the bottom stage, as OpenRocket 24.12 lights its parallel-booster
/// example's E12s with the core's I115W (`validation/fixtures/ork/openrocket-flights.json`); at
/// the stage below's events it is not lit (ADR-171).
fn ignition(
    motor: &OrkMotor,
    motors: &[OrkMotor],
    hung_on: &[Option<usize>],
    firsts: &[Result<Option<usize>, String>],
) -> Result<hpr_design::Ignition, String> {
    let Ignition { event, delay_s } = &motor.ignition;
    let delay_s = delay(*delay_s, "ignition")?;
    let at = |time_s: f64| {
        if time_s == 0.0 {
            hpr_design::Ignition::Launch
        } else {
            hpr_design::Ignition::Time { time_s }
        }
    };
    let axial = |stage: usize| hung_on.get(stage).is_some_and(Option::is_none);
    let last_stage = (0..hung_on.len()).rev().find(|&stage| axial(stage));
    let carrier = hung_on.get(motor.stage).copied().flatten();
    if let Some(carrier) = carrier {
        return match event {
            IgnitionEvent::Launch => Ok(at(delay_s)),
            IgnitionEvent::Automatic if Some(carrier) == last_stage => Ok(at(delay_s)),
            IgnitionEvent::Never => Ok(hpr_design::Ignition::Never),
            IgnitionEvent::Other(word) => Err(format!("its ignition event `{word}` is not known")),
            _ => Err(format!(
                "it lights at `{}` in a parallel stage, which HPR Sim has no reading for",
                event.as_str()
            )),
        };
    }
    // The stage below's motor, or `None` in the bottom stage, which has no stage below: its event
    // never comes, and OpenRocket flies the motor unlit. Its first burnout's for a `burnout`,
    // its one mount's for a charge.
    let below = |at_burnout: bool| -> Result<Option<&OrkMotor>, String> {
        let Some(below) = (motor.stage + 1..hung_on.len()).find(|&stage| axial(stage)) else {
            return Ok(None);
        };
        if at_burnout {
            firsts
                .get(below)
                .cloned()
                .unwrap_or(Ok(None))?
                .and_then(|first| motors.get(first))
        } else {
            one_mount(motors, below)?
        }
        .ok_or_else(|| {
            format!(
                "it lights at `{}` of the stage below, and stage {below} holds no motor",
                event.as_str(),
            )
        })
        .map(Some)
    };
    let ejection = || -> Result<hpr_design::Ignition, String> {
        let Some(below) = below(false)? else {
            return Ok(hpr_design::Ignition::Never);
        };
        match below.delay {
            Some(Delay::Seconds(charge_s)) => Ok(hpr_design::Ignition::Burnout {
                mount: below.mount.clone(),
                delay_s: delay(charge_s, "ejection")? + delay_s,
            }),
            // Plugged: no charge fires, and OpenRocket flies the motor unlit.
            Some(Delay::Plugged) => Ok(hpr_design::Ignition::Never),
            _ => Err(format!(
                "it lights at the ejection charge of {} in the stage below, which states no \
                 delay",
                below.designation
            )),
        }
    };
    match event {
        IgnitionEvent::Launch => Ok(at(delay_s)),
        IgnitionEvent::Automatic if Some(motor.stage) == last_stage => Ok(at(delay_s)),
        IgnitionEvent::Automatic | IgnitionEvent::EjectionCharge => ejection(),
        IgnitionEvent::Burnout => Ok(below(true)?.map_or(hpr_design::Ignition::Never, |below| {
            hpr_design::Ignition::Burnout {
                mount: below.mount.clone(),
                delay_s,
            }
        })),
        IgnitionEvent::Never => Ok(hpr_design::Ignition::Never),
        IgnitionEvent::Other(word) => Err(format!("its ignition event `{word}` is not known")),
    }
}

/// How long `motor` burns, s.
fn burn_s(motor: &OrkMotor) -> Result<f64, String> {
    motor
        .curve
        .motor()
        .map(hpr_motor::SolidMotor::burnout_time_s)
        .ok_or_else(|| format!("{} has no thrust curve", motor.designation))
}

/// When each of `motors` lights, s after launch, given their ignitions `lit` in the same order:
/// `None` for one that never lights ([`never_lit`]). Every other one is known before the flight: a
/// `.ork` lights a motor at a time, or at the burnout of a motor in the stage below, whose time is
/// known in turn.
fn ignition_times_s(
    motors: &[OrkMotor],
    lit: &[hpr_design::Ignition],
    burns_s: &[f64],
) -> Result<Vec<Option<f64>>, String> {
    let never = never_lit(motors, lit);
    let mut times: Vec<Option<f64>> = vec![None; motors.len()];
    // Each pass settles at least one motor while any can be, so this many passes settle them all.
    for _ in 0..motors.len() {
        let mut settled = false;
        for index in 0..motors.len() {
            if times[index].is_some() || never[index] {
                continue;
            }
            let time = match &lit[index] {
                hpr_design::Ignition::Launch => Some(0.0),
                hpr_design::Ignition::Time { time_s } => Some(*time_s),
                hpr_design::Ignition::Burnout { mount, delay_s } => motors
                    .iter()
                    .position(|m| m.mount == *mount)
                    .and_then(|at| times[at].map(|time| time + burns_s[at] + delay_s)),
                _ => None,
            };
            if time.is_some() {
                times[index] = time;
                settled = true;
            }
        }
        if !settled {
            break;
        }
    }
    if (0..motors.len()).any(|index| times[index].is_none() && !never[index]) {
        return Err("a motor's ignition waits on one whose time is not known".to_owned());
    }
    Ok(times)
}

/// `lit`, the ignitions of `motors` in the same order, with each motor that never lights
/// ([`never_lit`]) written as [`hpr_design::Ignition::Never`]; or why the configuration can't be
/// flown: no motor of it lights.
pub(super) fn never_when_waiting_on_never(
    motors: &[OrkMotor],
    lit: Vec<hpr_design::Ignition>,
) -> Result<Vec<hpr_design::Ignition>, String> {
    let never = never_lit(motors, &lit);
    if never.iter().all(|&never| never) {
        return Err("no motor of it ever lights, so the rocket would not leave the pad".to_owned());
    }
    Ok(lit
        .into_iter()
        .zip(never)
        .map(|(ignition, never)| {
            if never {
                hpr_design::Ignition::Never
            } else {
                ignition
            }
        })
        .collect())
}

/// Whether each of `motors`, lit by `lit` in the same order, never lights: it is set so
/// ([`hpr_design::Ignition::Never`]), or lit by the burnout of a mount whose motors never light,
/// as HPR Sim's flight lights it.
fn never_lit(motors: &[OrkMotor], lit: &[hpr_design::Ignition]) -> Vec<bool> {
    let mut never: Vec<bool> = lit
        .iter()
        .map(|ignition| *ignition == hpr_design::Ignition::Never)
        .collect();
    // Each pass settles at least one more link of a chain, so this many passes settle them all.
    for _ in 0..motors.len() {
        let mut changed = false;
        for index in 0..motors.len() {
            if never[index] {
                continue;
            }
            if let hpr_design::Ignition::Burnout { mount, .. } = &lit[index] {
                let mut in_mount = (0..motors.len()).filter(|&i| motors[i].mount == *mount);
                if in_mount.all(|i| never[i]) {
                    never[index] = true;
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
    }
    never
}

/// The ignition of the motors in `stage` as a separation's trigger `delay_s` after it, when they
/// light together at a time or a burnout.
fn lit_at(
    motors: &[OrkMotor],
    lit: &[hpr_design::Ignition],
    stage: usize,
    delay_s: f64,
    what: &str,
) -> Result<StagingTrigger, String> {
    let mut ignitions = motors.iter().zip(lit).filter(|(m, _)| m.stage == stage);
    let Some((_, first)) = ignitions.next() else {
        return Err(format!(
            "it separates at {what}'s ignition, and stage {stage} holds no motor"
        ));
    };
    if ignitions.any(|(_, other)| other != first) {
        return Err(format!(
            "it separates at {what}'s ignition, and stage {stage}'s motors light at different times"
        ));
    }
    match first {
        hpr_design::Ignition::Launch => Ok(StagingTrigger::Time { time_s: delay_s }),
        hpr_design::Ignition::Time { time_s } => Ok(StagingTrigger::Time {
            time_s: time_s + delay_s,
        }),
        hpr_design::Ignition::Burnout {
            mount,
            delay_s: after_s,
        } => Ok(StagingTrigger::Burnout {
            mount: mount.clone(),
            delay_s: after_s + delay_s,
        }),
        hpr_design::Ignition::Never => Err(format!(
            "it separates at {what}'s ignition, which never comes"
        )),
        // The `.ork` reading never gives one, and a separation timed from the ignition that a
        // separation causes could never fire.
        _ => Err(format!(
            "it separates at {what}'s ignition, which waits on a separation"
        )),
    }
}

/// The separations configuration `id` flies, in the order they fire (from the tail
/// forward), none when it flies none, or why HPR Sim can't fly its stages as the file says. `motors`
/// are its motors and `lit` their ignitions, in order; `stages` is the count of the rocket's
/// stages.
pub(super) fn staging(
    id: &str,
    motors: &[OrkMotor],
    lit: &[hpr_design::Ignition],
    separations: &[StageSeparation],
    hung_on: &[Option<usize>],
) -> Result<Vec<Staging>, String> {
    let stages = hung_on.len();
    let never = never_lit(motors, lit);
    // A stage's own motors, when it holds some and none of them ever lights: a separation at
    // their burnout or ejection charge never comes, and OpenRocket keeps the stage on. A
    // parallel stage holding none keeps on too: OpenRocket 24.12 flies its parallel-booster
    // example's `[I115W-10; None]` with the boosters on to the ground (ADR-171).
    let unlit = |stage: usize| {
        let mut own = (0..motors.len()).filter(|&i| motors[i].stage == stage);
        let parallel = hung_on.get(stage).is_some_and(Option::is_some);
        (parallel || own.clone().next().is_some()) && own.all(|i| never[i])
    };
    let mut active = Vec::new();
    for stage in 1..stages {
        let setting = separations
            .iter()
            .find(|s| s.stage == stage)
            .map(|s| s.separation_in(id));
        match setting.and_then(|s| s.event.as_ref().map(|event| (event, s))) {
            Some((SeparationEvent::Never, _)) => {}
            Some((SeparationEvent::Burnout | SeparationEvent::Ejection, _)) if unlit(stage) => {}
            Some((event, setting)) => active.push((stage, event, setting)),
            None => return Err(format!("stage {stage} states no separation event")),
        }
    }
    let late = |event: &SeparationEvent| {
        matches!(
            event,
            SeparationEvent::Apogee | SeparationEvent::AltitudeDescending
        )
    };
    if let [(_, event, _)] = active.as_slice()
        && late(event)
    {
        // At or after apogee: the climb is the whole stack's, and the descent is not flown. That
        // assumes every motor is spent by apogee, which `cargo xtask ork-flights` checks of each
        // flight.
        return Ok(Vec::new());
    }
    if active.len() > 1
        && let Some((stage, ..)) = active.iter().find(|(_, event, _)| late(event))
    {
        return Err(format!(
            "{} stages separate, stage {stage} at or after apogee, and HPR Sim flies more than one \
             separation only under power",
            active.len()
        ));
    }
    // From the tail forward, as the stack drops them; each must come no earlier than the one
    // behind it, which holds its stages on until then.
    let mut flown: Vec<Staging> = Vec::with_capacity(active.len());
    for &(stage, event, setting) in active.iter().rev() {
        // The stage above a parallel stage is no stage: it hangs beside one.
        if hung_on.get(stage).is_some_and(Option::is_some)
            && matches!(event, SeparationEvent::UpperIgnition)
        {
            return Err(format!(
                "stage {stage}, a parallel stage, separates at `{}`, which HPR Sim has no reading for",
                event.as_str()
            ));
        }
        let staging = separation(motors, lit, (stage, active.len() == 1), event, setting)?;
        if let Some(behind) = flown.last()
            && staging.time_s < behind.time_s
        {
            return Err(format!(
                "stage {stage} separates at {} s, before stage {} behind it does at {} s, while \
                 that stage still holds it",
                staging.time_s,
                behind.after_stage + 1,
                behind.time_s
            ));
        }
        flown.push(staging);
    }
    Ok(flown)
}

/// The separation at the boundary ahead of stage `stage` on `event`, or why HPR Sim can't fly it as
/// the file says. `alone` says it is the configuration's only one: only then may it come with
/// nothing ahead of it left to burn.
fn separation(
    motors: &[OrkMotor],
    lit: &[hpr_design::Ignition],
    (stage, alone): (usize, bool),
    event: &SeparationEvent,
    setting: &super::recovery::Separation,
) -> Result<Staging, String> {
    let delay_s = delay(setting.delay_s.unwrap_or(0.0), "separation")?;
    let own = || {
        one_mount(motors, stage)?
            .ok_or_else(|| format!("stage {stage} separates at its own motor, and holds none"))
    };
    let burns_s = motors.iter().map(burn_s).collect::<Result<Vec<_>, _>>()?;
    let times_s = ignition_times_s(motors, lit, &burns_s)?;
    // The stage's first burnout (ADR-172): its lit motors' earliest, the first in the file on a
    // tie; `staging` lets through only a stage with a motor that lights.
    let first_out = || {
        if !motors.iter().any(|m| m.stage == stage) {
            return Err(format!(
                "stage {stage} separates at its own motor, and holds none"
            ));
        }
        (0..motors.len())
            .filter(|&i| motors[i].stage == stage)
            .filter_map(|i| times_s[i].map(|lit_s| (lit_s + burns_s[i], i)))
            .fold(
                None,
                |earliest: Option<(f64, usize)>, (out_s, i)| match earliest {
                    Some((least_s, _)) if least_s <= out_s => earliest,
                    _ => Some((out_s, i)),
                },
            )
            .map(|(_, i)| &motors[i])
            .ok_or_else(|| format!("stage {stage} separates at its own motor, and none lights"))
    };
    let trigger = match event {
        SeparationEvent::Apogee | SeparationEvent::AltitudeDescending => {
            return Err(format!(
                "stage {stage} separates at or after apogee, beside another separation"
            ));
        }
        SeparationEvent::Launch => StagingTrigger::Time { time_s: delay_s },
        SeparationEvent::Ignition => lit_at(motors, lit, stage, delay_s, "its own motor")?,
        SeparationEvent::UpperIgnition => {
            lit_at(motors, lit, stage - 1, delay_s, "the stage above")?
        }
        SeparationEvent::Burnout => StagingTrigger::Burnout {
            mount: first_out()?.mount.clone(),
            delay_s,
        },
        SeparationEvent::Ejection => {
            let own = own()?;
            match own.delay {
                Some(Delay::Seconds(charge_s)) => StagingTrigger::Burnout {
                    mount: own.mount.clone(),
                    delay_s: delay(charge_s, "ejection")? + delay_s,
                },
                _ => {
                    return Err(format!(
                        "stage {stage} separates at the ejection charge of {}, which has none \
                         (plugged, or no delay given)",
                        own.designation
                    ));
                }
            }
        }
        SeparationEvent::AltitudeAscending => {
            return Err(format!(
                "stage {stage} separates at a height on the way up, which HPR Sim has no trigger for"
            ));
        }
        _ => {
            return Err(format!(
                "stage {stage} separates at `{}`, which HPR Sim has no trigger for",
                event.as_str()
            ));
        }
    };
    let time_s = match &trigger {
        StagingTrigger::Time { time_s } => *time_s,
        StagingTrigger::Burnout { mount, delay_s } => {
            let at = motors
                .iter()
                .position(|m| m.mount == *mount)
                .ok_or_else(|| format!("no motor sits in mount `{mount}`"))?;
            // The stage's motors light (`unlit` let through no other), so the time is known.
            let lit_s = times_s[at]
                .ok_or_else(|| format!("stage {stage} separates at a motor that never lights"))?;
            lit_s + burns_s[at] + delay_s
        }
    };
    // hpr's test, when the separation fires: the part ahead has a motor burning or still to
    // light, and the part behind has none (flight.rs). A motor that never lights does neither.
    let burning =
        |index: usize| times_s[index].is_some_and(|lit_s| lit_s + burns_s[index] > time_s);
    // With nothing ahead of it left to burn, the flight flies every part as a point with its
    // devices' drag alone from the split (ADR-014), which hands no sustainer on to another.
    let powered = (0..motors.len()).any(|i| motors[i].stage < stage && burning(i));
    if !alone && !powered {
        return Err(format!(
            "stage {stage} separates at {time_s} s with no motor ahead of it left to burn, beside \
             another separation, and HPR Sim flies more than one separation only under power"
        ));
    }
    // At its stage's first burnout under power, the stage's other motors may still burn, as
    // OpenRocket's stage drops them there: the flight drops them burning, their thrust left out
    // of the part's descent (ADR-172). Each must have lit before the split; none still to light,
    // none in another stage, and none at any other split.
    let at_first_burnout = powered && matches!(event, SeparationEvent::Burnout);
    let dropped_burning =
        |i: usize, lit_s: f64| at_first_burnout && motors[i].stage == stage && lit_s < time_s;
    if let Some((behind, lit_s)) = (0..motors.len())
        .filter(|&i| motors[i].stage >= stage && burning(i))
        .filter_map(|i| times_s[i].map(|lit_s| (i, lit_s)))
        .find(|&(i, lit_s)| !dropped_burning(i, lit_s))
    {
        return Err(format!(
            "stage {stage} separates at {time_s} s, while {} behind it burns until {} s",
            motors[behind].designation,
            lit_s + burns_s[behind]
        ));
    }
    // hpr's flight fires a separation only once the rocket is off the rod (flight.rs), so one at
    // launch would come late.
    if time_s <= 0.0 {
        return Err(format!(
            "stage {stage} separates at launch, on the pad, where HPR Sim's flight fires no separation"
        ));
    }
    let drops_burning = (0..motors.len()).any(|i| motors[i].stage >= stage && burning(i));
    Ok(Staging {
        after_stage: stage - 1,
        trigger,
        time_s,
        drops_burning,
    })
}
